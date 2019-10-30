use std::convert::TryFrom;
pub struct BufferData<T>(pub Vec<T>);

use js_sys::WebAssembly;
use wasm_bindgen::JsCast;
impl TryFrom<BufferData<f32>> for js_sys::Float32Array {
    type Error = &'static str;

    fn try_from(data: BufferData<f32>) -> Result<Self, Self::Error> {
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .map_err(|_| "Unable to get the WASM memory buffer for storing the vertices data!")?
            .buffer();
        let location = data.0.as_ptr() as u32 / 4;
        let data = js_sys::Float32Array::new(&memory_buffer)
            .subarray(location, location + data.0.len() as u32);
        Ok(data)
    }
}
impl TryFrom<BufferData<u32>> for js_sys::Uint32Array {
    type Error = &'static str;

    fn try_from(data: BufferData<u32>) -> Result<Self, Self::Error> {
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .map_err(|_| "Unable to get the WASM memory buffer for storing the vertices data!")?
            .buffer();
        let location = data.0.as_ptr() as u32 / 4;
        let data = js_sys::Uint32Array::new(&memory_buffer)
            .subarray(location, location + data.0.len() as u32);
        Ok(data)
    }
}

impl<T> From<Vec<T>> for BufferData<T> {
    fn from(data: Vec<T>) -> Self {
        BufferData(data)
    }
}