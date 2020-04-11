#[derive(Clone, Copy, Debug)]
pub struct TileConfig {
    // The size of the texture images
    texture_size: i32,
    // The size of the tile in the texture
    tile_size: i32,
    // Delta depth i.e. log2(texture_size / tile_size)
    delta_depth: u8,

    // Num tiles per texture
    num_tiles_per_texture: usize,

    // Max depth of the current HiPS tiles
    max_depth: u8,

    // Format of the texture images e.g.:
    // * WebGl2RenderingContext::RGB for jpg images
    // * WebGl2RenderingContext::RGBA for png images
    // * WebGl2RenderingContext::R for one channel images such as FITS images
    format: TileImageFormat,

    // The number of bytes in a tile
    num_bytes: usize,
}

pub const NUM_TEXTURES_BY_SIDE_SLICE: i32 = 8;
pub const NUM_TEXTURES_BY_SLICE: i32 = NUM_TEXTURES_BY_SIDE_SLICE * NUM_TEXTURES_BY_SIDE_SLICE;
pub const NUM_SLICES: i32 = 4;
pub const NUM_TEXTURES: usize = (NUM_TEXTURES_BY_SLICE * NUM_SLICES) as usize;

use crate::WebGl2Context;
use crate::core::Texture2DArray;
impl TileConfig {
    fn new(tile_size: i32, max_depth_tile: u8, format: ImageFormat) -> TileConfig {
        // Assert size is a power of two
        assert!(is_power_of_two(tile_size as usize));
        // Determine the size of the texture to copy
        // it cannot be > to 512x512px
        let texture_size = std::cmp::min(512, tile_size << max_depth_tile);
        let num_tile_per_side_texture = texture_size / tile_size;

        let delta_depth = math::log_2(num_tile_per_side_texture as i32) as u8;

        let num_tiles_per_texture_side = 1 << delta_depth;
        let num_tiles_per_texture = num_tiles_per_texture_side * num_tiles_per_texture_side;

        let format = TileImageFormat::new(format);

        let num_tile_pixels = (tile_size as usize) * (tile_size as usize);
        let num_channels = format.num_channels();
        let num_bytes = num_channels * num_tile_pixels;

        let max_depth = max_depth_tile - delta_depth;
        TileConfig {
            texture_size,
            tile_size,
            delta_depth,
            max_depth,
            num_tiles_per_texture,
            format,
            num_bytes
        }
    }

    // Define a set of textures compatible with the HEALPix tile format and size
    pub fn create_texture_array(&self, gl: &WebGl2Context) -> Texture2DArray {
        Texture2DArray::create_empty(
            gl,
            self.texture_size * NUM_TEXTURES_BY_SIDE_SLICE as i32,
            self.texture_size * NUM_TEXTURES_BY_SIDE_SLICE as i32,
            NUM_SLICES as i32,
            &[
                // The HiPS tiles sampling is NEAREST
                (WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::NEAREST),
                (WebGl2RenderingContext::TEXTURE_MAG_FILTER, WebGl2RenderingContext::NEAREST),
                
                // Prevents s-coordinate wrapping (repeating)
                (WebGl2RenderingContext::TEXTURE_WRAP_S, WebGl2RenderingContext::CLAMP_TO_EDGE),
                // Prevents t-coordinate wrapping (repeating)
                (WebGl2RenderingContext::TEXTURE_WRAP_T, WebGl2RenderingContext::CLAMP_TO_EDGE),
            ],
            self.format.flag() as u32,
        )
    }

    #[inline]
    pub fn delta_depth(&self) -> u8 {
        self.delta_depth
    }

    #[inline]
    pub fn num_tiles_per_texture(&self) -> usize {
        self.num_tiles_per_texture
    }

    #[inline]
    pub fn get_texture_size(&self) -> i32 {
        self.texture_size
    }

    #[inline]
    pub fn get_tile_size(&self) -> i32 {
        self.tile_size
    }

    #[inline]
    pub fn get_num_channels(&self) -> usize {
        self.format.num_channels()
    }

    #[inline]
    pub fn get_flag(&self) -> i32 {
        self.format.flag()
    }

    #[inline]
    pub fn get_ext(&self) -> &'static str {
        self.format.ext()
    }

    #[inline]
    pub fn get_num_bytes(&self) -> usize {
        self.num_bytes
    }
}

#[derive(Clone, Copy, Debug)]
struct TileImageFormat {
    format: ImageFormat,

    num_channels: usize, // number of format channels
    flag: i32, // Storage format e.g. WebGl2RenderingContext::RGB
    ext: &'static str, // Used for composing the URI to retrieve the tile
}

use web_sys::WebGl2RenderingContext;
impl TileImageFormat {
    fn new(format: ImageFormat) -> TileImageFormat {
        match format {
            ImageFormat::JPG => TileImageFormat {
                num_channels: 3,
                // 3 bytes, one for the red, one for the green the last for the blue
                flag: WebGl2RenderingContext::RGB as i32,
                ext: "jpg",
                format,
            },
            ImageFormat::PNG => TileImageFormat {
                num_channels: 4,
                // 4 bytes: red, green, blue and an alpha channel
                flag: WebGl2RenderingContext::RGBA as i32,
                ext: "png",
                format
            },
            ImageFormat::FITS => TileImageFormat {
                num_channels: 1,
                // Values are 32 bit floats 
                flag: WebGl2RenderingContext::DEPTH_COMPONENT32F as i32,
                ext: "fits",
                format
            }
        }
    }

    #[inline]
    fn num_channels(&self) -> usize {
        self.num_channels
    }

    #[inline]
    fn flag(&self) -> i32 {
        self.flag
    }

    #[inline]
    fn ext(&self) -> &'static str {
        self.ext
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ImageFormat {
    JPG, // RGB
    PNG, // RGBA
    FITS, // One channel (grayscale)
}

#[derive(Clone, Debug)]
pub struct HiPSConfig {
    pub name: String,

    tile_config: TileConfig,
}

#[inline]
fn is_power_of_two(x: usize) -> bool {
    x & (x - 1) == 0
}

use crate::math;
impl HiPSConfig {
    pub fn new(name: String, max_depth: u8, tile_size: usize, fmt: ImageFormat) -> HiPSConfig {
        let tile_config = TileConfig::new(tile_size as i32, max_depth, fmt);

        HiPSConfig {
            name,

            tile_config
        }
    }

    #[inline]
    pub fn max_depth(&self) -> u8 {
        self.tile_config.max_depth
    }

    #[inline]
    pub fn tile_config(&self) -> &TileConfig {
        &self.tile_config
    }
}

use crate::shader::HasUniforms;
use crate::shader::ShaderBound;

impl HasUniforms for HiPSConfig {
    fn attach_uniforms<'a>(&self, shader: &'a ShaderBound<'a>) -> &'a ShaderBound<'a> {
        // Send max depth
        shader.attach_uniform("max_depth", &(self.max_depth() as i32));

        shader
    }
}
