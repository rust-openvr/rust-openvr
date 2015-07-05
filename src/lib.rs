

extern crate openvr_sys;

pub struct IVRSystem(*const ());

#[derive(Debug, Copy, Clone)]
pub struct Size {
    pub width: u32,
    pub height: u32
}

#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

#[derive(Debug, Copy, Clone)]
pub struct Rectangle {
    pub position: Position,
    pub size: Size
}

#[derive(Debug, Copy, Clone)]
pub struct DistortionCoordinates {
    pub red: [f32; 2],
    pub green: [f32; 2],
    pub blue: [f32; 2],
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Eye {
    Left, Right
}


impl Eye {
    /// Convert a eye to a HmdEye
    fn to_raw(&self) -> openvr_sys::Hmd_Eye {
        match self {
            &Eye::Left => openvr_sys::Hmd_Eye::Left,
            &Eye::Right => openvr_sys::Hmd_Eye::Right,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TextureBounds {
    pub u_min: f32,
    pub u_max: f32,
    pub v_min: f32,
    pub v_max: f32
}

impl TextureBounds {
    /// Convert a bounds to a openvr_bounds
    fn to_raw(self) -> openvr_sys::VRTextureBounds_t {
        openvr_sys::VRTextureBounds_t{
            uMin: self.u_min,
            uMax: self.u_max,
            vMin: self.v_min,
            vMax: self.v_max
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct TrackedDevicePose {
    pub to_device: [[f32; 4]; 3],
    pub velocity: [f32; 3],
    pub angular_velocity: [f32; 3],
    pub is_valid: bool,
    pub is_connected: bool,
}

#[derive(Debug, Copy, Clone)]
pub struct TrackedDevicePoses {
    pub count: usize,
    pub poses: [TrackedDevicePose; 16],
}

impl TrackedDevicePoses {
    pub fn as_slice(&self) -> &[TrackedDevicePose] {
        &self.poses[0..self.count]
    }
}

impl IVRSystem {
    /// Initialize the IVR System
    pub fn init() -> Result<IVRSystem, openvr_sys::HmdError> {
        let mut err = openvr_sys::HmdError::None;
        let ptr = unsafe {
            openvr_sys::VR_Init(&mut err as *mut openvr_sys::HmdError)
        };
        if ptr.is_null() {
            Err(err)
        } else {
            Ok(IVRSystem(ptr))
        }
    }

    /// Get the window bounds
    pub fn bounds(&self) -> Rectangle {
        unsafe {
            let mut size = Size{width: 0, height: 0};
            let mut pos = Position{x: 0, y: 0};
            openvr_sys::VR_IVRSystem_GetWindowBounds(
                self.0,
                &mut pos.x,
                &mut pos.y,
                &mut size.width,
                &mut size.height
            );
            Rectangle {
                position: pos,
                size: size
            }
        }
    }

    /// Get the recommended render target size
    pub fn recommended_render_target_size(&self) -> Size {
        unsafe {
            let mut size = Size{width: 0, height: 0};
            openvr_sys::VR_IVRSystem_GetRecommendedRenderTargetSize(
                self.0,
                &mut size.width,
                &mut size.height
            );
            size
        }
    }

    /// Get eye viewport size
    pub fn eye_viewport(&self, eye: Eye) -> Rectangle {
        use std::mem;
        unsafe {
            let mut size = Size{width: 0, height: 0};
            let mut pos = Position{x: 0, y: 0};
            openvr_sys::VR_IVRSystem_GetEyeOutputViewport(
                self.0,
                eye.to_raw(),
                mem::transmute(&mut pos.x),
                mem::transmute(&mut pos.y),
                &mut size.width,
                &mut size.height
            );
            Rectangle {
                position: pos,
                size: size
            }
        }
    }

    /// Get the projection matrix for an eye
    /// supply the near and the far position
    /// assumes opengl conventions
    pub fn projection_matrix(&self, eye: Eye, near: f32, far: f32) -> [[f32; 4]; 4] {
        unsafe {
            let mat = openvr_sys::VR_IVRSystem_GetProjectionMatrix(
                self.0,
                eye.to_raw(),
                near,
                far,
                openvr_sys::GraphicsAPIConvention::OpenGL
            );
            mat.m
        }
    }

    /// Computes the distortion caused by the optics 
    pub fn compute_distortion(&self, eye: Eye, u: f32, v: f32) -> DistortionCoordinates {
        unsafe {
            let coord = openvr_sys::VR_IVRSystem_ComputeDistortion(
                self.0,
                eye.to_raw(),
                u, v
            );
            DistortionCoordinates {
                red: coord.rfRed,
                blue: coord.rfBlue,
                green: coord.rfGreen
            }
        }    
    }

    /// Computes the distortion caused by the optics 
    pub fn eye_to_head_transform(&self, eye: Eye) -> [[f32; 4]; 3] {
        unsafe {
            let mat = openvr_sys::VR_IVRSystem_GetEyeToHeadTransform(
                self.0,
                eye.to_raw(),
            );
            mat.m
        }    
    }

    /// Computes the distortion caused by the optics 
    pub fn time_since_last_vsync(&self) -> Option<(f32, u64)> {
        unsafe {
            let mut frame = 0;
            let mut sync = 0.;
            let found = openvr_sys::VR_IVRSystem_GetTimeSinceLastVsync(
                self.0,
                &mut sync,
                &mut frame
            );

            if found {
                Some((sync, frame))
            } else {
                None
            }
        }  
    }

    /// Fetch the tracked results from the HMD
    pub fn tracked_devices(&self, time: f32) -> TrackedDevicePoses {
        unsafe {
            let mut data: [openvr_sys::TrackedDevicePose_t; 16] = std::mem::zeroed();
            openvr_sys::VR_IVRSystem_GetDeviceToAbsoluteTrackingPose(
                self.0,
                openvr_sys::TrackingUniverseOrigin::TrackingUniverseSeated,
                time,
                &mut data[0],
                16
            );

            let mut out: TrackedDevicePoses = std::mem::zeroed();
            for (i, d) in data.iter().enumerate() {
                if d.bDeviceIsConnected {
                    out.count = i + 1;
                }
                out.poses[i].is_connected = d.bDeviceIsConnected;
                out.poses[i].is_valid = d.bPoseIsValid;
                out.poses[i].to_device = d.mDeviceToAbsoluteTracking.m;
                out.poses[i].velocity = d.vVelocity.v;
                out.poses[i].angular_velocity = d.vAngularVelocity.v;
            }

            out
        }
    }

    /// If the device supports a compositor return a instance
    pub fn compositor(&self) -> Result<Compositor, openvr_sys::HmdError> {
        unsafe {
            let mut err = openvr_sys::HmdError::None;
            let name = std::ffi::CString::new("IVRCompositor_006").unwrap();
            let ptr = openvr_sys::VR_GetGenericInterface(name.as_ptr(), &mut err as *mut openvr_sys::HmdError);
            match err {
                openvr_sys::HmdError::None => Ok(Compositor(ptr)),
                err => Err(err)
            }
        }
    }
}

impl Drop for IVRSystem {
    fn drop(&mut self) {
        unsafe {
            println!("Trying to shutdown openvr");
            openvr_sys::VR_Shutdown();
            println!("Should be done now.");
        }
    }
}

/// A VR compositor
pub struct Compositor(*const ());

impl Compositor {
    /// Check to see if the compositor is fullscreen
    pub fn is_fullscreen(&self) -> bool {
        unsafe { openvr_sys::VR_IVRCompositor_IsFullscreen(self.0) }
    }

    /// Check if vsync in enabled
    pub fn get_vsync(&self) -> bool {
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

    /// Set the compositor to gl mode
    pub fn set_graphics_device_gl(&self) {
        unsafe {
            openvr_sys::VR_IVRCompositor_SetGraphicsDevice(
                self.0,
                openvr_sys::Compositor_DeviceType::DeviceType_OpenGL,
                std::ptr::null_mut()
            )
        }
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
                t,
                &mut b as *mut openvr_sys::VRTextureBounds_t
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

            let mut out: TrackedDevicePoses = std::mem::zeroed();
            for (i, d) in data.iter().enumerate() {
                if d.bDeviceIsConnected {
                    out.count = i + 1;
                }
                out.poses[i].is_connected = d.bDeviceIsConnected;
                out.poses[i].is_valid = d.bPoseIsValid;
                out.poses[i].to_device = d.mDeviceToAbsoluteTracking.m;
                out.poses[i].velocity = d.vVelocity.v;
                out.poses[i].angular_velocity = d.vAngularVelocity.v;
            }

            out
        }
    }
}
