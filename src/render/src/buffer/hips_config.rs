use crate::image_fmt::FormatImageType;

#[derive(Clone, Debug)]
struct TileConfig {
    // The size of the tile in the texture
    width: i32,
    blank_tile: Rc<TileArrayBufferImage>,
}

#[derive(Debug)]
pub enum TileArrayBufferImage {
    F32(TileArrayBuffer<ArrayF32>),
    U8(TileArrayBuffer<ArrayU8>),
    I16(TileArrayBuffer<ArrayI16>),
    I32(TileArrayBuffer<ArrayI32>),
}

use super::TileArrayBuffer;
use std::rc::Rc;

use crate::WebGl2Context;
use super::{ArrayU8, ArrayF32, ArrayI32, ArrayI16};

fn create_empty_tile(format: &FormatImageType, width: i32, value: f32) -> TileArrayBufferImage {
    let num_channels = format.get_num_channels() as i32;
    match format {
        FormatImageType::JPG => TileArrayBufferImage::U8(TileArrayBuffer::<ArrayU8>::blank(width, num_channels, value as u8)),
        FormatImageType::PNG => TileArrayBufferImage::U8(TileArrayBuffer::<ArrayU8>::blank(width, num_channels, value as u8)),
        FormatImageType::FITS(fits) => {
            match format.get_type() {
                WebGl2RenderingContext::FLOAT => {
                    TileArrayBufferImage::F32(TileArrayBuffer::<ArrayF32>::blank(width, num_channels, value as f32))
                },
                WebGl2RenderingContext::INT => {
                    TileArrayBufferImage::I32(TileArrayBuffer::<ArrayI32>::blank(width, num_channels, value as i32))
                },
                WebGl2RenderingContext::SHORT => {
                    TileArrayBufferImage::I16(TileArrayBuffer::<ArrayI16>::blank(width, num_channels, value as i16))
                },
                WebGl2RenderingContext::UNSIGNED_BYTE => {
                    TileArrayBufferImage::U8(TileArrayBuffer::<ArrayU8>::blank(width, num_channels, value as u8))
                },
                _ => unimplemented!()
            }
        }
    }
}

impl TileConfig {
    fn new(width: i32, format: &FormatImageType, blank_value: f32) -> TileConfig {
        assert!(is_power_of_two(width as usize));
        let blank_tile = Rc::new(create_empty_tile(format, width, blank_value));
        TileConfig {
            width,
            blank_tile,
        }
    }

    #[inline]
    pub fn get_tile_size(&self) -> i32 {
        self.width
    }

    #[inline]
    pub fn get_blank_tile(&self) -> Rc<TileArrayBufferImage> {
        self.blank_tile.clone()
    }

    pub fn set_blank_value(&mut self, format: &FormatImageType, blank_value: f32) {
        self.blank_tile = Rc::new(create_empty_tile(format, self.width, blank_value));
    }
}

use crate::transfert_function::TransferFunction;
use crate::shaders::Colormap;
#[derive(Debug)]
pub struct HiPSConfig {
    pub root_url: String,
    // HiPS image format
    format: FormatImageType,

    tile_config: TileConfig,

    // The size of the texture images
    pub texture_size: i32,
    // Delta depth i.e. log2(texture_size / tile_size)
    delta_depth: u8,
    // Num tiles per texture
    num_tiles_per_texture: usize,
    // Max depth of the current HiPS tiles
    max_depth_texture: u8,
    num_textures_by_side_slice: i32,
    num_textures_by_slice: i32,
    num_slices: i32,
    num_textures: usize,

    min_cutout: f32,
    max_cutout: f32,
    transfer_f: TransferFunction,
    blank_value: Option<f32>,
    colormap: Colormap,
}

#[inline]
fn is_power_of_two(x: usize) -> bool {
    x & (x - 1) == 0
}

use crate::image_fmt::FITS;

use crate::math;
use web_sys::WebGl2RenderingContext;
use wasm_bindgen::JsValue;
use crate::HiPSDefinition;
impl HiPSConfig {
    pub fn new(gl: &WebGl2Context, hips_definition: HiPSDefinition) -> Result<HiPSConfig, JsValue> {

        let fmt = hips_definition.format;
        let format: Result<_, JsValue> = if fmt.contains("fits") {
            // Check the bitpix to determine the internal format of the tiles
            let fits = match hips_definition.bitpix {
                8 => FITS::new(WebGl2RenderingContext::R8UI as i32),
                16 => FITS::new(WebGl2RenderingContext::R16I as i32),
                32 => FITS::new(WebGl2RenderingContext::R32I as i32),
                -32 => FITS::new(WebGl2RenderingContext::R32F as i32),
                _ => unimplemented!()
            };

            Ok(FormatImageType::FITS(fits))
        } else if fmt.contains("png") {
            Ok(FormatImageType::PNG)
        } else if fmt.contains("jpg") || fmt.contains("jpeg") {
            Ok(FormatImageType::JPG)
        } else {
            Err(format!("{:?} tile format unknown!", fmt).into())
        };
        let format = format?;

        let max_depth_tile = hips_definition.maxOrder;
        let tile_size = hips_definition.tileSize;
        let tile_config = TileConfig::new(tile_size, &format, 0.0);

        // Define the size of the 2d texture array depending on the
        // characterics of the client
        let num_textures_by_side_slice = 8;
        let num_textures_by_slice = num_textures_by_side_slice * num_textures_by_side_slice;
        let num_slices = 3;
        let num_textures = (num_textures_by_slice * num_slices) as usize;

        // Assert size is a power of two
        // Determine the size of the texture to copy
        // it cannot be > to 512x512px

        let texture_size = std::cmp::min(512, tile_size << max_depth_tile);
        let num_tile_per_side_texture = texture_size / tile_size;

        let delta_depth = math::log_2(num_tile_per_side_texture as i32) as u8;

        let num_tiles_per_texture_side = 1 << delta_depth;
        let num_tiles_per_texture = num_tiles_per_texture_side * num_tiles_per_texture_side;

        let max_depth_texture = max_depth_tile - delta_depth;
        let blank_value = None;
        let colormap = Colormap::RedTemperature;
        let transfer_f = TransferFunction::Linear;
        Ok(HiPSConfig {
            // HiPS name
            root_url: hips_definition.url,
            format,
            // Tile size & blank tile data
            tile_config,
            // Texture config
            // The size of the texture images
            texture_size,
            // Delta depth i.e. log2(texture_size / tile_size)
            delta_depth,
            // Num tiles per texture
            num_tiles_per_texture,
            // Max depth of the current HiPS tiles
            max_depth_texture,
            num_textures_by_side_slice,
            num_textures_by_slice,
            num_slices,
            num_textures,
            min_cutout: hips_definition.minCutout,
            max_cutout: hips_definition.maxCutout,
            transfer_f,
            blank_value,
            colormap
        })
    }

    pub fn set_HiPS_definition(&mut self, hips_def: HiPSDefinition) {
        let max_depth_tile = hips_def.maxOrder;
        let tile_size = hips_def.tileSize;

        let texture_size = std::cmp::min(512, tile_size << max_depth_tile);
        let num_tile_per_side_texture = texture_size / tile_size;

        self.delta_depth = math::log_2(num_tile_per_side_texture as i32) as u8;

        let num_tiles_per_texture_side = 1 << self.delta_depth;
        self.num_tiles_per_texture = num_tiles_per_texture_side * num_tiles_per_texture_side;

        self.max_depth_texture = max_depth_tile - self.delta_depth;

        self.root_url = hips_def.url;
        self.min_cutout = hips_def.minCutout;
        self.max_cutout = hips_def.maxCutout;
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
        self.tile_config.width
    }

    #[inline]
    pub fn get_num_channels(&self) -> usize {
        self.format.get_num_channels()
    }

    #[inline]
    pub fn get_internal_format(&self) -> i32 {
        self.format.get_internal_format()
    }

    #[inline]
    pub fn get_ext_file(&self) -> &'static str {
        self.format.get_ext_file()
    }

    #[inline]
    pub fn get_blank_tile(&self) -> Rc<TileArrayBufferImage> {
        self.tile_config.get_blank_tile()
    }

    #[inline]
    pub fn max_depth(&self) -> u8 {
        self.max_depth_texture
    }

    #[inline]
    pub fn num_textures(&self) -> usize {
        self.num_textures
    }

    #[inline]
    pub fn num_textures_by_side_slice(&self) -> i32 {
        self.num_textures_by_side_slice
    }

    #[inline]
    pub fn num_textures_by_slice(&self) -> i32 {
        self.num_textures_by_slice
    }

    #[inline]
    pub fn num_slices(&self) -> i32 {
        self.num_slices
    }

    #[inline]
    pub fn format(&self) -> FormatImageType {
        self.format
    }

    #[inline]
    pub fn get_min_cutout(&self) -> f32 {
        self.min_cutout
    }

    #[inline]
    pub fn get_max_cutout(&self) -> f32 {
        self.max_cutout
    }

    pub fn set_cutouts(&mut self, min_cutout: f32, max_cutout: f32) {
        self.min_cutout = min_cutout;
        self.max_cutout = max_cutout;
    }

    pub fn set_transfer_function(&mut self, transfer_func: TransferFunction) {
        self.transfer_f = transfer_func;
    }

    #[inline]
    pub fn set_fits_colormap(&mut self, colormap: Colormap) {
        self.colormap = colormap;
    }

    #[inline]
    pub fn is_blank_value(&self) -> bool {
        self.blank_value.is_some()
    }

    pub fn set_blank_value(&mut self, blank_value: f32) {
        self.blank_value = Some(blank_value);
        self.tile_config.set_blank_value(&self.format, blank_value);
    }
}

use crate::shader::HasUniforms;
use crate::shader::ShaderBound;

impl HasUniforms for HiPSConfig {
    fn attach_uniforms<'a>(&self, shader: &'a ShaderBound<'a>) -> &'a ShaderBound<'a> {
        // Send max depth
        shader.attach_uniform("max_depth", &(self.max_depth_texture as i32))
            .attach_uniform("blank_value", &self.blank_value.unwrap_or(std::f32::MIN))
            .attach_uniform("colormap", &self.colormap)
            .attach_uniforms_from(&self.transfer_f);

        shader
    }
}