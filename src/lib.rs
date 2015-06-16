

extern crate openvr_sys;

#[derive(Debug)]
pub struct IVRSystem(*const ());

impl IVRSystem {
    /// Initalize the IVR System
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
    pub fn bounds(&self) -> ((i32, i32), (u32, u32)) {
        unsafe {
            let ((mut x, mut y), (mut w, mut h)) = ((0, 0), (0, 0));
            openvr_sys::VR_IVRSystem_GetWindowBounds(
                self.0, &mut x, &mut y, &mut w, &mut h
            );
            ((x, y), (w, h))
        }
    }

}