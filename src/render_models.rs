use std::{fmt, ptr, slice};
use std::ffi::CStr;

use openvr_sys as sys;

use {RenderModels};

impl<'a> RenderModels<'a> {
    /// Loads and returns a render model for use in the application. `name` should be a render model name from the
    /// `RenderModelName_String` property or an absolute path name to a render model on disk.
    ///
    /// The method returns `Ok(None)` while the render model is still being loaded. Call it at regular intervals until
    /// it returns `Ok(Some(model))`.
    pub fn load_render_model(&self, name: &CStr) -> Result<Option<Model>> {
        let mut ptr = ptr::null_mut();
        let r = unsafe {
            self.0.LoadRenderModel_Async.unwrap()(name as *const _ as *mut _, &mut ptr)
        };
        match Error(r) {
            error::NONE => Ok(Some(Model { ptr: ptr, sys: self.0 })),
            error::LOADING => Ok(None),
            x => Err(x),
        }
    }

    /// Loads and returns a texture for use in the application. Texture IDs can be obtained from
    /// `Model::diffuse_texture_id()`.
    ///
    /// The method returns `Ok(None)` while the texture is still being loaded. Call it at regular intervals until it
    /// returns `Ok(Some(texture))`.
    pub fn load_texture(&self, id: TextureId) -> Result<Option<Texture>> {
        let mut ptr = ptr::null_mut();
        let r = unsafe {
            self.0.LoadTexture_Async.unwrap()(id, &mut ptr)
        };
        match Error(r) {
            error::NONE => Ok(Some(Texture { ptr: ptr, sys: self.0 })),
            error::LOADING => Ok(None),
            x => Err(x),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Error(sys::EVRRenderModelError);

pub mod error {
    use super::{sys, Error};

    pub const NONE: Error = Error(sys::EVRRenderModelError_EVRRenderModelError_VRRenderModelError_None);
    pub const LOADING: Error = Error(sys::EVRRenderModelError_EVRRenderModelError_VRRenderModelError_Loading);
    pub const NOT_SUPPORTED: Error = Error(sys::EVRRenderModelError_EVRRenderModelError_VRRenderModelError_NotSupported);
    pub const INVALID_ARG: Error = Error(sys::EVRRenderModelError_EVRRenderModelError_VRRenderModelError_InvalidArg);
    pub const INVALID_MODEL: Error = Error(sys::EVRRenderModelError_EVRRenderModelError_VRRenderModelError_InvalidModel);
    pub const NO_SHAPES: Error = Error(sys::EVRRenderModelError_EVRRenderModelError_VRRenderModelError_NoShapes);
    pub const MULTIPLE_SHAPES: Error = Error(sys::EVRRenderModelError_EVRRenderModelError_VRRenderModelError_MultipleShapes);
    pub const TOO_MANY_VERTICES: Error = Error(sys::EVRRenderModelError_EVRRenderModelError_VRRenderModelError_TooManyVertices);
    pub const MULTIPLE_TEXTURES: Error = Error(sys::EVRRenderModelError_EVRRenderModelError_VRRenderModelError_MultipleTextures);
    pub const BUFFER_TOO_SMALL: Error = Error(sys::EVRRenderModelError_EVRRenderModelError_VRRenderModelError_BufferTooSmall);
    pub const NOT_ENOUGH_NORMALS: Error = Error(sys::EVRRenderModelError_EVRRenderModelError_VRRenderModelError_NotEnoughNormals);
    pub const NOT_ENOUGH_TEX_COORDS: Error = Error(sys::EVRRenderModelError_EVRRenderModelError_VRRenderModelError_NotEnoughTexCoords);
    pub const INVALID_TEXTURE: Error = Error(sys::EVRRenderModelError_EVRRenderModelError_VRRenderModelError_InvalidTexture);
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(::std::error::Error::description(self))
    }
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        use self::error::*;
        match *self {
            NONE => "NONE",
            LOADING => "LOADING",
            NOT_SUPPORTED => "NOT_SUPPORTED",
            INVALID_ARG => "INVALID_ARG",
            INVALID_MODEL => "INVALID_MODEL",
            NO_SHAPES => "NO_SHAPES",
            MULTIPLE_SHAPES => "MULTIPLE_SHAPES",
            TOO_MANY_VERTICES => "TOO_MANY_VERTICES",
            MULTIPLE_TEXTURES => "MULTIPLE_TEXTURES",
            BUFFER_TOO_SMALL => "BUFFER_TOO_SMALL",
            NOT_ENOUGH_NORMALS => "NOT_ENOUGH_NORMALS",
            NOT_ENOUGH_TEX_COORDS => "NOT_ENOUGH_TEX_COORDS",
            INVALID_TEXTURE => "INVALID_TEXTURE",
            _ => "UNKNOWN",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(::std::error::Error::description(self))
    }
}

pub type Result<T> = ::std::result::Result<T, Error>;

/// 3D geometry for rendering as an indexed triangle list
pub struct Model<'a> {
    ptr: *mut sys::RenderModel_t,
    sys: &'a sys::VR_IVRRenderModels_FnTable,
}

impl<'a> Model<'a> {
    pub fn vertices(&self) -> &[Vertex] {
        unsafe {
            let model = &*self.ptr;
            slice::from_raw_parts(model.rVertexData as *mut Vertex, model.unVertexCount as usize)
        }
    }

    pub fn indices(&self) -> &[u16] {
        unsafe {
            let model = &*self.ptr;
            slice::from_raw_parts(model.rIndexData, 3 * model.unTriangleCount as usize)
        }
    }

    pub fn diffuse_texture_id(&self) -> Option<TextureId> {
        let id = unsafe { (&*self.ptr).diffuseTextureId };
        if id < 0 { None } else { Some(id) }
    }
}

impl<'a> Drop for Model<'a> {
    fn drop(&mut self) { unsafe { self.sys.FreeRenderModel.unwrap()(self.ptr) } }
}

pub struct Texture<'a> {
    ptr: *mut sys::RenderModel_TextureMap_t,
    sys: &'a sys::VR_IVRRenderModels_FnTable,
}

impl<'a> Texture<'a> {
    pub fn dimensions(&self) -> (u16, u16) {
        let tex = unsafe { &*self.ptr };
        (tex.unWidth, tex.unHeight)
    }

    /// R8G8B8A8
    pub fn data(&self) -> &[u8] {
        unsafe {
            let tex = &*self.ptr;
            slice::from_raw_parts(tex.rubTextureMapData, tex.unWidth as usize * tex.unHeight as usize * 4)
        }
    }
}

impl<'a> Drop for Texture<'a> {
    fn drop(&mut self) { unsafe { self.sys.FreeTexture.unwrap()(self.ptr) } }
}

pub type TextureId = sys::TextureID_t;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub texture_coord: [f32; 2],
}
