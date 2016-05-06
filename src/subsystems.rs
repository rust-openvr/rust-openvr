extern crate openvr_sys;
use openvr_sys::Enum_EVRInitError::*;

use system::IVRSystem;
use extended_display::IVRExtendedDisplay;
use compositor::IVRCompositor;
use render_models::IVRRenderModels;

use std;

/// gets the current vr system interface (initialization is required beforehand)
pub fn system() -> Result<IVRSystem, openvr_sys::HmdError> {
    let mut err = EVRInitError_VRInitError_None;
    let name = std::ffi::CString::new("FnTable:IVRSystem_012").unwrap();
    let ptr = unsafe {
        openvr_sys::VR_GetGenericInterface(name.as_ptr(), &mut err)
    };

    match err {
        EVRInitError_VRInitError_None => {
            unsafe {
                return Ok(IVRSystem::from_raw(ptr as *const ()));
            }
        },
        _ => {
            return Err(err);
        }
    }
}

/// gets the current vr extended display interface (initialization is required beforehand)
pub fn extended_display() -> Result<IVRExtendedDisplay, openvr_sys::HmdError> {
    let mut err = EVRInitError_VRInitError_None;
    let name = std::ffi::CString::new("FnTable:IVRExtendedDisplay_001").unwrap();
    let ptr = unsafe {
        openvr_sys::VR_GetGenericInterface(name.as_ptr(), &mut err)
    };

    match err {
        EVRInitError_VRInitError_None => {
            unsafe {
                return Ok(IVRExtendedDisplay::from_raw(ptr as *const ()));
            }
        },
        _ => {
            return Err(err);
        }
    }
}

/// gets the current vr extended display interface (initialization is required beforehand)
pub fn compositor() -> Result<IVRCompositor, openvr_sys::HmdError> {
    let mut err = EVRInitError_VRInitError_None;
    let name = std::ffi::CString::new("FnTable:IVRCompositor_013").unwrap();
    let ptr = unsafe {
        openvr_sys::VR_GetGenericInterface(name.as_ptr(), &mut err)
    };

    match err {
        EVRInitError_VRInitError_None => {
            unsafe {
                return Ok(IVRCompositor::from_raw(ptr as *const ()));
            }
        },
        _ => {
            return Err(err);
        }
    }
}

/// gets the current vr extended display interface (initialization is required beforehand)
pub fn render_models() -> Result<IVRRenderModels, openvr_sys::HmdError> {
    let mut err = EVRInitError_VRInitError_None;
    let name = std::ffi::CString::new("FnTable:IVRRenderModels_005").unwrap();
    let ptr = unsafe {
        openvr_sys::VR_GetGenericInterface(name.as_ptr(), &mut err)
    };

    match err {
        EVRInitError_VRInitError_None => {
            unsafe {
                return Ok(IVRRenderModels::from_raw(ptr as *const ()));
            }
        },
        _ => {
            return Err(err);
        }
    }
}
