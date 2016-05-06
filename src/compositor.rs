use openvr_sys;
use openvr_sys::Enum_EGraphicsAPIConvention::*;
use openvr_sys::Enum_EVRSubmitFlags::*;
use openvr_sys::Enum_EColorSpace::*;
use common::*;
use tracking::*;

/// A VR compositor
pub struct IVRCompositor(*const ());

impl IVRCompositor {
    pub unsafe fn from_raw(ptr: *const ()) -> Self {
        IVRCompositor(ptr as *mut ())
    }

    /// Check to see if the compositor is fullscreen
    pub fn is_fullscreen(&self) -> bool {
        unsafe {
            let comp = * { self.0 as *mut openvr_sys::Struct_VR_IVRCompositor_FnTable };
            comp.IsFullscreen.unwrap()() > 0
        }
    }

    /// Check if compositor can render a scene
    pub fn can_render_scene(&self) -> bool {
        unsafe {
            let comp = * { self.0 as *mut openvr_sys::Struct_VR_IVRCompositor_FnTable };
            comp.CanRenderScene.unwrap()() > 0
        }
    }

    /// Submits an opengl framebuffer as an eye to the render
    pub fn submit(&self, eye: Eye, texture: usize, bounds: TextureBounds) {
        let mut b = bounds.to_raw();
        let e = eye.to_raw();

        unsafe {
            use std;

            let comp = * { self.0 as *mut openvr_sys::Struct_VR_IVRCompositor_FnTable };
            let mut t = openvr_sys::Texture_t {
                eType: EGraphicsAPIConvention_API_OpenGL,
                eColorSpace: EColorSpace_ColorSpace_Auto,
                handle: texture as *mut std::os::raw::c_void,
            };

            comp.Submit.unwrap()(
                e,
                &mut t,
                &mut b as *mut openvr_sys::VRTextureBounds_t,
                EVRSubmitFlags_Submit_GlRenderBuffer
            );
        }
    }

    /// Get the poses
    pub fn wait_get_poses(&self) -> TrackedDevicePoses {
        use std;

        unsafe {
            let comp = * { self.0 as *mut openvr_sys::Struct_VR_IVRCompositor_FnTable };
            let mut data: [openvr_sys::TrackedDevicePose_t; 16] = std::mem::zeroed();

            comp.WaitGetPoses.unwrap()(
                &mut data[0],
                16,
                std::ptr::null_mut(),
                0
            );
            to_tracked(data)
        }
    }
}
