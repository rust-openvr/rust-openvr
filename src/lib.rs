extern crate openvr_sys;
use openvr_sys::Enum_EVRInitError::*;
use openvr_sys::Enum_EVRApplicationType::*;

pub mod common;
pub mod system;
pub mod extended_display;

pub use system::IVRSystem;
pub use extended_display::IVRExtendedDisplay;

pub use common::Eye;

/// Inits the open vr interface and returns the system
pub fn init() ->  Result<system::IVRSystem, openvr_sys::HmdError> {
    let mut err = EVRInitError_VRInitError_None;
    let app_type = EVRApplicationType_VRApplication_Scene;

    // try to initialize base vr eco
    unsafe {
        openvr_sys::VR_InitInternal(&mut err, app_type);
    };

    // check for errors
    match err {
        EVRInitError_VRInitError_None => {
            // get system
            let result = system();
            match result {
                Ok(sys) => {
                    return Ok(sys);
                },
                Err(err) => {
                    return Err(err);
                }
            }
        },
        _ => {
            return Err(err);
        }
    };
}

/// Shutdowns all openvr related systems
pub fn shutdown() {
    unsafe {
        openvr_sys::VR_ShutdownInternal();
    }
}

/// gets the current vr system interface (initialization is required beforehand)
pub fn system() -> Result<system::IVRSystem, openvr_sys::HmdError> {
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
