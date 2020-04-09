use web_sys::HtmlImageElement;
#[derive(Debug)]
pub enum Image {
    HtmlImageElement(HtmlImageElement),
    Bytes(Box<[u8]>),
}

use cgmath::{Vector3, Vector2};
use crate::core::Texture2DArray;
impl Image {
    pub fn write(&self, offset: &Vector3<i32>, size: &Vector2<i32>, textures_array: &Texture2DArray) {
        match self {
            Image::HtmlImageElement(image) => {
                textures_array.bind()
                    .tex_sub_image_3d_with_html_image_element(
                        offset.x,
                        offset.y,
                        offset.z,
                        size.x,
                        size.y,
                        &image,
                    );
            }
            Image::Bytes(bytes) => {
                textures_array.bind()
                    .tex_sub_image_3d_with_opt_u8_array(
                        offset.x,
                        offset.y,
                        offset.z,
                        size.x,
                        size.y,
                        Some(bytes.as_ref()),
                    );
            }
        }
    }
}