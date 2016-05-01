
/// A VR compositor
pub struct Compositor(*const ());

impl Compositor {
    /// Check to see if the compositor is fullscreen
    pub fn is_fullscreen(&self) -> bool {
        unsafe { openvr_sys::VR_IVRCompositor_IsFullscreen(self.0) }
    }

    /// Check if vsync in enabled
    pub fn get_vsync(&self) -> Option<u64> {
        unsafe { openvr_sys::VR_IVRCompositor_GetVSync(self.0) }
    }

    /// Set the vsync value
    pub fn set_vsync(&self, v: bool) {
        unsafe { openvr_sys::VR_IVRCompositor_SetVSync(self.0, v) }
    }

    /// Check if vsync in enabled
    pub fn can_render_scene(&self) -> bool {
        unsafe { openvr_sys::VR_IVRCompositor_CanRenderScene(self.0) }
    }

    /// Get the gamma value
    pub fn get_gamma(&self) -> f32 {
        unsafe { openvr_sys::VR_IVRCompositor_GetGamma(self.0) }
    }

    /// Get the gamma value
    pub fn set_gamma(&self, v: f32) {
        unsafe { openvr_sys::VR_IVRCompositor_SetGamma(self.0, v) }
    }

    /// Submit an eye to the render
    pub fn submit(&self, eye: Eye, texture: usize, bounds: TextureBounds) {
        let mut b = bounds.to_raw();
        let e = eye.to_raw();
        unsafe {
            use std::mem;
            let t = mem::transmute(texture);

            openvr_sys::VR_IVRCompositor_Submit(
                self.0,
                e,
                openvr_sys::GraphicsAPIConvention::OpenGL,
                t,
                &mut b as *mut openvr_sys::VRTextureBounds_t,
                openvr_sys::VRSubmitFlags_t::Default
            );
        }
    }

    /// Get the poses
    pub fn wait_get_poses(&self) -> TrackedDevicePoses {
        unsafe {
            let mut data: [openvr_sys::TrackedDevicePose_t; 16] = std::mem::zeroed();
            openvr_sys::VR_IVRCompositor_WaitGetPoses(
                self.0,
                &mut data[0],
                16,
                std::ptr::null_mut(),
                0
            );
            to_tracked(data)
        }
    }
}
