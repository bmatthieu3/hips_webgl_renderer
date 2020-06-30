pub enum Colormap {
    BlackWhiteLinear,
    RedTemperature,
    IDLCBGnBu,
    IDLCBYIGnBu,
    BluePastelRed,
    IDLCBBrBG,
}

use crate::ShaderManager;
use crate::Shader;
impl Colormap {
    pub fn get_shader<'a>(&self, shaders: &'a ShaderManager) -> &'a Shader {
        let shader = match self {
            Colormap::BlackWhiteLinear => shaders.get("black_white_linear"),
            Colormap::RedTemperature => shaders.get("red_temperature"),
            Colormap::IDLCBGnBu => shaders.get("IDL_CB_GnBu"),
            Colormap::IDLCBYIGnBu => shaders.get("IDL_CB_YIGnBu"),
            Colormap::BluePastelRed => shaders.get("BluePastelRed"),
            Colormap::IDLCBBrBG => shaders.get("IDL_CB_BrBG"),
        };

        shader.unwrap()
    }
}