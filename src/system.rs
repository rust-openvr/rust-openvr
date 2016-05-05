use openvr_sys;
use openvr_sys::Enum_EGraphicsAPIConvention::*;
use openvr_sys::Enum_ETrackingUniverseOrigin::*;

use common::*;
use tracking::*;

pub struct IVRSystem(pub *const ());

impl IVRSystem {
    pub unsafe fn from_raw(ptr: *const ()) -> Self {
        IVRSystem(ptr as *mut ())
    }

    /// Get the recommended render target size
    pub fn recommended_render_target_size(&self) -> Size {
        unsafe {
            let system = * { self.0 as *mut openvr_sys::Struct_VR_IVRSystem_FnTable };

            let mut size = Size{width: 0, height: 0};
            system.GetRecommendedRenderTargetSize.unwrap()(
                &mut size.width,
                &mut size.height
            );
            size
        }
    }


    /// Get the projection matrix for an eye
    /// supply the near and the far position
    /// assumes opengl conventions
    pub fn projection_matrix(&self, eye: Eye, near: f32, far: f32) -> [[f32; 4]; 4] {
        unsafe {
            let system = * { self.0 as *mut openvr_sys::Struct_VR_IVRSystem_FnTable };

            let mat = system.GetProjectionMatrix.unwrap()(
                eye.to_raw(),
                near,
                far,
                EGraphicsAPIConvention_API_OpenGL
            );
            mat.m
        }
    }

    /// Computes the distortion caused by the optics
    pub fn compute_distortion(&self, eye: Eye, u: f32, v: f32) -> DistortionCoordinates {
        unsafe {
            let system = * { self.0 as *mut openvr_sys::Struct_VR_IVRSystem_FnTable };
            let coord = system.ComputeDistortion.unwrap()(
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
            let system = * { self.0 as *mut openvr_sys::Struct_VR_IVRSystem_FnTable };
            let mat = system.GetEyeToHeadTransform.unwrap()(
                eye.to_raw(),
            );
            mat.m
        }
    }

    /// Computes the distortion caused by the optics
    pub fn time_since_last_vsync(&self) -> Option<(f32, u64)> {
        unsafe {
            let system = * { self.0 as *mut openvr_sys::Struct_VR_IVRSystem_FnTable };
            let mut frame = 0;
            let mut sync = 0.;
            let found = system.GetTimeSinceLastVsync.unwrap()(
                &mut sync,
                &mut frame
            );

            if found > 0 {
                Some((sync, frame))
            } else {
                None
            }
        }
    }

    /// Fetch the tracked results from the HMD
    /// when time is bigger than 0, it will give you the predicted poses for that time
    /// Time is counted in photons, see https://github.com/ValveSoftware/openvr/wiki/IVRSystem::GetDeviceToAbsoluteTrackingPose
    ///  for time to photons conversion
    pub fn tracked_devices(&self, time: f32) -> TrackedDevicePoses {
        use std;

        unsafe {
            let system = * { self.0 as *mut openvr_sys::Struct_VR_IVRSystem_FnTable };
            let mut data: [openvr_sys::TrackedDevicePose_t; 16] = std::mem::zeroed();
            system.GetDeviceToAbsoluteTrackingPose.unwrap()(
                ETrackingUniverseOrigin_TrackingUniverseSeated,
                time,
                &mut data[0],
                16
            );
            to_tracked(data)
        }
    }
}
