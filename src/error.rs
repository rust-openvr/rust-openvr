use openvr_sys;
use subsystems::*;

pub trait RawError {
    fn is_err(&self) -> bool;
    fn message(&self) -> String;
}

#[derive(Debug)]
pub struct Error<Err: RawError + Copy> {
    raw: Err
}

impl<Err: RawError + Copy> Error<Err> {
    /// Creates a new error object using the raw openvr_sys error
    pub fn from_raw(raw: Err) -> Self {
        Error {
            raw: raw
        }
    }

    /// Turns managed error into raw enum from binding
    pub fn to_raw(&self) -> Err {
        self.raw
    }

    /// Gets an human-readable error message (if available)
    pub fn message(&self) -> String {
        self.raw.message()
    }

    /// Returns true when current object is not an error
    pub fn is_ok(&self) -> bool {
        !self.raw.is_err()
    }

    /// Return true when current object is an error
    pub fn is_err(&self) -> bool {
        self.raw.is_err()
    }
}

// OpenVR implement per error type a new function to get a error string
//  for easier use, this macro will generate easily the RawError trait
macro_rules! impl_raw_error {
    ($subsystem:ident, $fntable: ident, $get:ident, $raw_name:ident, $none_name:ident) => {
        impl RawError for $raw_name {
            fn is_err(&self) -> bool {
                match *self {
                    $none_name => {
                       false
                    },
                    _ => {
                       true
                    }
                }
            }

            fn message(&self) -> String {
                let sstr = unsafe {
                    let sub = * { $subsystem().unwrap().0 as *mut openvr_sys::$fntable};
                    CStr::from_ptr(sub.$get.unwrap()(*self)).to_str().unwrap()
                };

                String::from(sstr)
            }
        }
    }
}

use std::ffi::CStr;
use openvr_sys::*;
use openvr_sys::ETrackedPropertyError::*;
use openvr_sys::EVRInitError::*;
use openvr_sys::EVRRenderModelError::*;
use openvr_sys::EVRTrackedCameraError::*;

impl_raw_error!(
    system,
    VR_IVRSystem_FnTable,
    GetPropErrorNameFromEnum,
    ETrackedPropertyError,
    ETrackedPropertyError_TrackedProp_Success);

impl_raw_error!(
    render_models,
    VR_IVRRenderModels_FnTable,
    GetRenderModelErrorNameFromEnum,
    EVRRenderModelError,
    EVRRenderModelError_VRRenderModelError_None);

impl_raw_error!(
    tracked_camera,
    VR_IVRTrackedCamera_FnTable,
    GetCameraErrorNameFromEnum,
    EVRTrackedCameraError,
    EVRTrackedCameraError_VRTrackedCameraError_None);

// The init error has some special function to retrieve string
impl RawError for EVRInitError {
    fn is_err(&self) -> bool {
        match *self {
            EVRInitError_VRInitError_None => {
                true
            },
            _ => {
                false
            }
        }
    }

    fn message(&self) -> String {
        let sstr = unsafe {
            CStr::from_ptr(openvr_sys::VR_GetVRInitErrorAsEnglishDescription(*self)).to_str().unwrap()
        };

        String::from(sstr)
    }
}
