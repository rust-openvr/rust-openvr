use openvr_sys;

use common::*;

pub struct IVRExtendedDisplay(*const ());

impl IVRExtendedDisplay {
    pub unsafe fn from_raw(ptr: *const ()) -> Self {
        IVRExtendedDisplay(ptr as *mut ())
    }

    /// Get the window bounds
    pub fn window_bounds(&self) -> Rectangle {
        unsafe {
            let ext = * { self.0 as *mut openvr_sys::Struct_VR_IVRExtendedDisplay_FnTable };

            let mut size = Size{width: 0, height: 0};
            let mut pos = Position{x: 0, y: 0};

            ext.GetWindowBounds.unwrap()(
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
    /// Get eye viewport size
    pub fn eye_viewport(&self, eye: Eye) -> Rectangle {
        use std::mem;
        
        unsafe {
            let ext = * { self.0 as *mut openvr_sys::Struct_VR_IVRExtendedDisplay_FnTable };

            let mut size = Size{width: 0, height: 0};
            let mut pos = Position{x: 0, y: 0};

            ext.GetEyeOutputViewport.unwrap()(
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
}
