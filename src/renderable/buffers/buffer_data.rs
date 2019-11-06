use std::convert::TryFrom;
pub struct BufferData<'a, T>(pub &'a Vec<T>);

use js_sys::WebAssembly;
use wasm_bindgen::JsCast;
impl<'a> TryFrom<BufferData<'a, f32>> for js_sys::Float32Array {
    type Error = &'static str;

    fn try_from(data: BufferData<'a, f32>) -> Result<Self, Self::Error> {
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

impl<'a> TryFrom<BufferData<'a, u32>> for js_sys::Uint32Array {
    type Error = &'static str;

    fn try_from(data: BufferData<'a, u32>) -> Result<Self, Self::Error> {
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
impl<'a> TryFrom<BufferData<'a, u16>> for js_sys::Uint16Array {
    type Error = &'static str;

    fn try_from(data: BufferData<'a, u16>) -> Result<Self, Self::Error> {
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .map_err(|_| "Unable to get the WASM memory buffer for storing the vertices data!")?
            .buffer();
        let location = data.0.as_ptr() as u32 / 2;
        let data = js_sys::Uint16Array::new(&memory_buffer)
            .subarray(location, location + data.0.len() as u32);
        Ok(data)
    }
}