use std::convert::TryFrom;

use crate::core::VertexAttribPointerType;

pub enum BufferData<'a, T>
where T: VertexAttribPointerType {
    VecData(&'a Vec<T>),
    SliceData(&'a [T]),
}

use js_sys::WebAssembly;
use wasm_bindgen::JsCast;

use js_sys::Float32Array;
impl<'a> TryFrom<BufferData<'a, f32>> for Float32Array {
    type Error = &'static str;

    fn try_from(buffer: BufferData<'a, f32>) -> Result<Self, Self::Error> {
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .map_err(|_| "Unable to get the WASM memory buffer for storing the vertices data!")?
            .buffer();
        
        let (ptr, len) = match buffer {
            BufferData::VecData(data) => (data.as_ptr(), data.len()),
            BufferData::SliceData(data) => (data.as_ptr(), data.len())
        };
        let location = ptr as u32 / 4;
        let data = Float32Array::new(&memory_buffer)
            .subarray(location, location + len as u32);
        Ok(data)
    }
}

use js_sys::Int32Array;
impl<'a> TryFrom<BufferData<'a, i32>> for Int32Array {
    type Error = &'static str;

    fn try_from(buffer: BufferData<'a, i32>) -> Result<Self, Self::Error> {
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .map_err(|_| "Unable to get the WASM memory buffer for storing the vertices data!")?
            .buffer();
        
        let (ptr, len) = match buffer {
            BufferData::VecData(data) => (data.as_ptr(), data.len()),
            BufferData::SliceData(data) => (data.as_ptr(), data.len())
        };
        let location = ptr as u32 / 4;
        let data = Int32Array::new(&memory_buffer)
            .subarray(location, location + len as u32);
        Ok(data)
    }
}

use js_sys::Uint32Array;
impl<'a> TryFrom<BufferData<'a, u32>> for Uint32Array {
    type Error = &'static str;

    fn try_from(buffer: BufferData<'a, u32>) -> Result<Self, Self::Error> {
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .map_err(|_| "Unable to get the WASM memory buffer for storing the vertices data!")?
            .buffer();
        
        let (ptr, len) = match buffer {
            BufferData::VecData(data) => (data.as_ptr(), data.len()),
            BufferData::SliceData(data) => (data.as_ptr(), data.len())
        };
        let location = ptr as u32 / 4;
        let data = Uint32Array::new(&memory_buffer)
            .subarray(location, location + len as u32);
        Ok(data)
    }
}

use js_sys::Uint16Array;
impl<'a> TryFrom<BufferData<'a, u16>> for Uint16Array {
    type Error = &'static str;

    fn try_from(buffer: BufferData<'a, u16>) -> Result<Self, Self::Error> {
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .map_err(|_| "Unable to get the WASM memory buffer for storing the vertices data!")?
            .buffer();
        
        let (ptr, len) = match buffer {
            BufferData::VecData(data) => (data.as_ptr(), data.len()),
            BufferData::SliceData(data) => (data.as_ptr(), data.len())
        };
        let location = ptr as u32 / 2;
        let data = Uint16Array::new(&memory_buffer)
            .subarray(location, location + len as u32);
        Ok(data)
    }
}

use js_sys::Uint8Array;
impl<'a> TryFrom<BufferData<'a, u8>> for Uint8Array {
    type Error = &'static str;

    fn try_from(buffer: BufferData<'a, u8>) -> Result<Self, Self::Error> {
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .map_err(|_| "Unable to get the WASM memory buffer for storing the vertices data!")?
            .buffer();
        
        let (ptr, len) = match buffer {
            BufferData::VecData(data) => (data.as_ptr(), data.len()),
            BufferData::SliceData(data) => (data.as_ptr(), data.len())
        };
        let location = ptr as u32;
        let data = Uint8Array::new(&memory_buffer)
            .subarray(location, location + len as u32);
        Ok(data)
    }
}