use openvr_sys;

use tracking::*;
use error::*;

pub struct IVRTrackedCamera(pub *const ());

impl IVRTrackedCamera {
    pub unsafe fn from_raw(ptr: *const ()) -> Self {
        IVRTrackedCamera(ptr as *mut ())
    }

    /// checks whether the current system has a camera
    pub fn has_camera(&self, device: &TrackedDevicePose) -> Result<bool, Error<openvr_sys::EVRTrackedCameraError>> {
        unsafe {
            let cam = * { self.0 as *mut openvr_sys::VR_IVRTrackedCamera_FnTable };
            let mut has_cam = 0i32;

            let error = Error::from_raw(
                cam.HasCamera.unwrap()(device.index as u32, &mut has_cam as *mut i32));

            if error.is_ok() {
                return Ok(has_cam > 0i32);
            } else {
                return Err(error);
            }
        }
    }
}
