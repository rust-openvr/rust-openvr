use std::ffi::{CStr, CString};
use std::{fmt, mem, ptr, slice};

use openvr_sys as sys;

use {get_string, ControllerState, RenderModels};

impl RenderModels {
    /// Loads and returns a render model for use in the application. `name` should be a render model name from the
    /// `RenderModelName_String` property or an absolute path name to a render model on disk.
    ///
    /// The method returns `Ok(None)` while the render model is still being loaded. Call it at regular intervals until
    /// it returns `Ok(Some(model))`.
    pub fn load_render_model(&self, name: &CStr) -> Result<Option<Model>> {
        let mut ptr = ptr::null_mut();
        let r = unsafe { self.0.LoadRenderModel_Async.unwrap()(name.as_ptr() as *mut _, &mut ptr) };
        match Error(r) {
            error::NONE => Ok(Some(Model {
                ptr: ptr,
                sys: self.0,
            })),
            error::LOADING => Ok(None),
            x => Err(x),
        }
    }

    /// Returns the number of components of the specified render model.
    ///
    /// Components are useful when client application wish to draw, label, or otherwise interact with components of tracked objects.
    /// Examples controller components:
    ///  renderable things such as triggers, buttons
    ///  non-renderable things which include coordinate systems such as 'tip', 'base', a neutral controller agnostic hand-pose
    ///  If all controller components are enumerated and rendered, it will be equivalent to drawing the traditional render model
    ///  Returns 0 if components not supported, >0 otherwise
    pub fn component_count(&self, model: &CStr) -> u32 {
        unsafe { self.0.GetComponentCount.unwrap()(model.as_ptr() as *mut _) }
    }

    /// Get the names of available components.
    ///
    /// `component` does not correlate to a tracked device index, but is only used for iterating over all available
    /// components.  If it's out of range, this function will return None.
    pub fn component_name(&self, model: &CStr, component: u32) -> Option<CString> {
        unsafe {
            get_string(|ptr, n| {
                self.0.GetComponentName.unwrap()(model.as_ptr() as *mut _, component, ptr, n)
            })
        }
    }

    /// Gets all component names of a given model
    pub fn component_names(&self, model: &CStr) -> ::std::vec::IntoIter<CString> {
        // FIXME: impl Iterator rather than allocating
        let n = self.component_count(model);
        (0..n)
            .map(|i| {
                self.component_name(model, i)
                    .expect("inconsistent component presence reported by OpenVR")
            })
            .collect::<Vec<_>>()
            .into_iter()
    }

    /// Use this to get the render model name for the specified rendermodel/component combination, to be passed to
    /// `load_render_model`.
    ///
    /// If the component name is out of range, this function will return None.
    /// Otherwise, it will return the size of the buffer required for the name.
    pub fn component_render_model_name(&self, model: &CStr, component: &CStr) -> Option<CString> {
        unsafe {
            get_string(|ptr, n| {
                self.0.GetComponentRenderModelName.unwrap()(
                    model.as_ptr() as *mut _,
                    component.as_ptr() as *mut _,
                    ptr,
                    n,
                )
            })
        }
    }

    /// Use this to query information about the component, as a function of the controller state.
    ///
    /// Returns None if the component is invalid.
    ///
    /// Check `ComponentState::is_visible()` to determine whether the returned component should be rendered.
    ///
    /// For dynamic controller components (ex: trigger) values will reflect component motions.
    /// For static components this will return a consistent value independent of the `ControllerState`.
    pub fn component_state(
        &self,
        model: &CStr,
        component: &CStr,
        state: &ControllerState,
        mode: &ControllerMode,
    ) -> Option<ComponentState> {
        unsafe {
            let mut out = mem::uninitialized();
            if self.0.GetComponentState.unwrap()(
                model.as_ptr() as *mut _,
                component.as_ptr() as *mut _,
                state as *const _ as *mut _,
                mode as *const _ as *mut _,
                &mut out as *mut _ as *mut _,
            ) {
                Some(out)
            } else {
                None
            }
        }
    }

    /// Loads and returns a texture for use in the application. Texture IDs can be obtained from
    /// `Model::diffuse_texture_id()`.
    ///
    /// The method returns `Ok(None)` while the texture is still being loaded. Call it at regular intervals until it
    /// returns `Ok(Some(texture))`.
    pub fn load_texture(&self, id: TextureId) -> Result<Option<Texture>> {
        let mut ptr = ptr::null_mut();
        let r = unsafe { self.0.LoadTexture_Async.unwrap()(id, &mut ptr) };
        match Error(r) {
            error::NONE => Ok(Some(Texture {
                ptr: ptr,
                sys: self.0,
            })),
            error::LOADING => Ok(None),
            x => Err(x),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Error(sys::EVRRenderModelError);

pub mod error {
    use super::{sys, Error};

    pub const NONE: Error = Error(sys::EVRRenderModelError_VRRenderModelError_None);
    pub const LOADING: Error = Error(sys::EVRRenderModelError_VRRenderModelError_Loading);
    pub const NOT_SUPPORTED: Error =
        Error(sys::EVRRenderModelError_VRRenderModelError_NotSupported);
    pub const INVALID_ARG: Error = Error(sys::EVRRenderModelError_VRRenderModelError_InvalidArg);
    pub const INVALID_MODEL: Error =
        Error(sys::EVRRenderModelError_VRRenderModelError_InvalidModel);
    pub const NO_SHAPES: Error = Error(sys::EVRRenderModelError_VRRenderModelError_NoShapes);
    pub const MULTIPLE_SHAPES: Error =
        Error(sys::EVRRenderModelError_VRRenderModelError_MultipleShapes);
    pub const TOO_MANY_VERTICES: Error =
        Error(sys::EVRRenderModelError_VRRenderModelError_TooManyVertices);
    pub const MULTIPLE_TEXTURES: Error =
        Error(sys::EVRRenderModelError_VRRenderModelError_MultipleTextures);
    pub const BUFFER_TOO_SMALL: Error =
        Error(sys::EVRRenderModelError_VRRenderModelError_BufferTooSmall);
    pub const NOT_ENOUGH_NORMALS: Error =
        Error(sys::EVRRenderModelError_VRRenderModelError_NotEnoughNormals);
    pub const NOT_ENOUGH_TEX_COORDS: Error =
        Error(sys::EVRRenderModelError_VRRenderModelError_NotEnoughTexCoords);
    pub const INVALID_TEXTURE: Error =
        Error(sys::EVRRenderModelError_VRRenderModelError_InvalidTexture);
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
            slice::from_raw_parts(
                model.rVertexData as *mut Vertex,
                model.unVertexCount as usize,
            )
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
        if id < 0 {
            None
        } else {
            Some(id)
        }
    }
}

impl<'a> Drop for Model<'a> {
    fn drop(&mut self) {
        unsafe { self.sys.FreeRenderModel.unwrap()(self.ptr) }
    }
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
            slice::from_raw_parts(
                tex.rubTextureMapData,
                tex.unWidth as usize * tex.unHeight as usize * 4,
            )
        }
    }
}

impl<'a> Drop for Texture<'a> {
    fn drop(&mut self) {
        unsafe { self.sys.FreeTexture.unwrap()(self.ptr) }
    }
}

pub type TextureId = sys::TextureID_t;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub texture_coord: [f32; 2],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ControllerMode {
    pub scroll_wheel_visible: bool,
}

impl Default for ControllerMode {
    fn default() -> Self {
        ControllerMode {
            scroll_wheel_visible: false,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ComponentState {
    pub tracking_to_component_render_model: [[f32; 4]; 3],
    pub tracking_to_component_local: [[f32; 4]; 3],
    pub properties: ComponentProperties,
}

impl ComponentState {
    pub fn is_static(&self) -> bool {
        self.properties & component_properties::IS_STATIC != 0
    }
    pub fn is_visible(&self) -> bool {
        self.properties & component_properties::IS_VISIBLE != 0
    }
    pub fn is_touched(&self) -> bool {
        self.properties & component_properties::IS_TOUCHED != 0
    }
    pub fn is_pressed(&self) -> bool {
        self.properties & component_properties::IS_PRESSED != 0
    }
    pub fn is_scrolled(&self) -> bool {
        self.properties & component_properties::IS_SCROLLED != 0
    }
}

type ComponentProperties = sys::VRComponentProperties;

pub mod component_properties {
    use super::{sys, ComponentProperties};

    pub const IS_STATIC: ComponentProperties =
        sys::EVRComponentProperty_VRComponentProperty_IsStatic as ComponentProperties;
    pub const IS_VISIBLE: ComponentProperties =
        sys::EVRComponentProperty_VRComponentProperty_IsVisible as ComponentProperties;
    pub const IS_TOUCHED: ComponentProperties =
        sys::EVRComponentProperty_VRComponentProperty_IsTouched as ComponentProperties;
    pub const IS_PRESSED: ComponentProperties =
        sys::EVRComponentProperty_VRComponentProperty_IsPressed as ComponentProperties;
    pub const IS_SCROLLED: ComponentProperties =
        sys::EVRComponentProperty_VRComponentProperty_IsScrolled as ComponentProperties;
}

pub mod component {
    pub mod controller {
        use openvr_sys as sys;
        use std::ffi::CStr;

        // TODO: Real constants
        lazy_static! {
            pub static ref GDC2015: &'static CStr = unsafe {
                CStr::from_bytes_with_nul_unchecked(sys::k_pch_Controller_Component_GDC2015)
            };
            pub static ref BASE: &'static CStr = unsafe {
                CStr::from_bytes_with_nul_unchecked(sys::k_pch_Controller_Component_Base)
            };
            pub static ref TIP: &'static CStr =
                unsafe { CStr::from_bytes_with_nul_unchecked(sys::k_pch_Controller_Component_Tip) };
            pub static ref HAND_GRIP: &'static CStr = unsafe {
                CStr::from_bytes_with_nul_unchecked(sys::k_pch_Controller_Component_HandGrip)
            };
            pub static ref STATUS: &'static CStr = unsafe {
                CStr::from_bytes_with_nul_unchecked(sys::k_pch_Controller_Component_Status)
            };
        }
    }
}
