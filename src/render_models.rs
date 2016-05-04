use openvr_sys;
use openvr_sys::Enum_EVRRenderModelError::*;

use std::string::String;
use std::ptr::null_mut;
use std::slice;
use subsystems::render_models;

pub struct IVRRenderModels(*const ());

pub struct RenderModel(*mut openvr_sys::RenderModel_t);

impl Drop for RenderModel {
    /// will inform openvr that the memory for the render model is no longer required
    fn drop (&mut self) {
        unsafe {
            let models = * { render_models().unwrap().0 as *mut openvr_sys::Struct_VR_IVRRenderModels_FnTable};
            models.FreeRenderModel.unwrap()(
                self.0
            );
        }
    }
}

impl RenderModel {
    /// Returns an iterator that iterates over vertices
    pub fn vertex_iter(&self) -> slice::Iter<openvr_sys::RenderModel_Vertex_t> {
        unsafe {
            let slice = slice::from_raw_parts((*self.0).rVertexData, (*self.0).unVertexCount as usize);
            slice.iter()
        }
    }

    /// Returns an iterator that iterates over indices
    pub fn index_iter(&self) -> slice::Iter<u16> {
        unsafe {
            let slice = slice::from_raw_parts((*self.0).rIndexData, (*self.0).unTriangleCount as usize);
            slice.iter()
        }
    }

    /// returns the unique identifier for the texture that the models uses
    pub fn texture_identifier(&self) -> usize {
        unsafe {
            (*self.0).diffuseTextureId as usize
        }
    }
}

impl IVRRenderModels {
    pub unsafe fn from_raw(ptr: *const ()) -> Self {
        IVRRenderModels(ptr as *mut ())
    }

    /// Returns the amount of render models available
    pub fn get_count(&self) -> u32 {
        unsafe {
            let models = * { self.0 as *mut openvr_sys::Struct_VR_IVRRenderModels_FnTable};

            models.GetRenderModelCount.unwrap()()
        }
    }

    /// Returns the name of an available render model
    pub fn get_name(&self, index: u32) -> String {
        unsafe {
            let models = * { self.0 as *mut openvr_sys::Struct_VR_IVRRenderModels_FnTable};
            let name_out = String::with_capacity(256);

            let size = models.GetRenderModelName.unwrap()(
                index,
                name_out.as_ptr() as *mut i8,
                256
            );

            if size > 0 {
                return String::from_raw_parts(name_out.as_ptr() as *mut _, (size - 1) as usize, (size - 1) as usize);
            } else {
                return String::from("");
            }
        };
    }

    /// Loads an render model into local memory
    pub fn load(&self, name: String) -> Result<RenderModel, openvr_sys::EVRRenderModelError> {
        unsafe {
            let models = * { self.0 as *mut openvr_sys::Struct_VR_IVRRenderModels_FnTable};
            let mut resp: *mut openvr_sys::RenderModel_t = null_mut();

            let err = models.LoadRenderModel_Async.unwrap()(
                name.as_ptr() as *mut i8,
                &mut resp
            );

            match err {
                EVRRenderModelError_VRRenderModelError_None => {
                    Ok(RenderModel ( resp ))
                },
                _ => {
                    Err(err)
                }
            }

        }
    }
}
