#[cfg(feature = "submit_d3d11")]
extern crate windows;

extern crate openvr_sys;
#[macro_use]
extern crate lazy_static;
use std::fmt::Debug;
use std::ffi::CString;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{fmt, ptr};

use openvr_sys as sys;
mod tracking;
pub mod application;
pub mod chaperone;
pub mod compositor;
mod init_error;
pub mod property;
pub mod render_models;
pub mod system;

pub use init_error::InitError;
pub use tracking::*;

pub use sys::VkDevice_T;
pub use sys::VkInstance_T;
pub use sys::VkPhysicalDevice_T;
pub use sys::VkQueue_T;
pub mod input;
pub mod errors;
pub mod settings;
pub mod overlay;
pub mod pose;

static INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Initialize OpenVR
///
/// # Panics
///
/// When the library has already been initialized
///
/// # Safety
///
/// The `Context` MUST be dropped or shut down with `Context::shutdown` before shutting down the graphics API. No OpenVR
/// calls may be made on object derived from a `Context` after the `Context` has been dropped or explicitly shut down.
pub unsafe fn init(ty: ApplicationType) -> Result<Context, InitError> {
    let mut error = sys::EVRInitError_VRInitError_None;
    sys::VR_InitInternal(&mut error, ty as sys::EVRApplicationType);
    if error != sys::EVRInitError_VRInitError_None {
        return Err(error.into());
    }
    if !sys::VR_IsInterfaceVersionValid(sys::IVRSystem_Version.as_ptr() as *const i8) {
        sys::VR_ShutdownInternal();
        return Err(InitError::Init_InterfaceNotFound);
    }

    if INITIALIZED.swap(true, Ordering::Acquire) {
        panic!("OpenVR has already been initialized!");
    }

    Ok(Context {
        live: AtomicBool::new(true),
    })
}
pub fn is_hmd_present() -> bool {
    unsafe { sys::VR_IsHmdPresent() }
}
pub fn is_runtime_installed() -> bool {
    unsafe { sys::VR_IsRuntimeInstalled() }
}


pub struct System(&'static sys::VR_IVRSystem_FnTable);
pub struct Application(&'static sys::VR_IVRApplications_FnTable);
pub struct Compositor(&'static sys::VR_IVRCompositor_FnTable);
pub struct RenderModels(&'static sys::VR_IVRRenderModels_FnTable);
pub struct Chaperone(&'static sys::VR_IVRChaperone_FnTable);
pub struct Input(&'static sys::VR_IVRInput_FnTable);
pub struct Settings(&'static sys::VR_IVRSettings_FnTable);
pub struct Overlay(&'static sys::VR_IVROverlay_FnTable);

/// Entry points into OpenVR.
///
/// At most one of this object may exist at a time.
///
/// See safety notes in `init`.
pub struct Context {
    live: AtomicBool,
}

fn load<T>(suffix: &[u8]) -> Result<*const T, InitError> {
    let mut magic = Vec::from(b"FnTable:".as_ref());
    magic.extend(suffix);
    let mut error = sys::EVRInitError_VRInitError_None;
    let result = unsafe { sys::VR_GetGenericInterface(magic.as_ptr() as *const i8, &mut error) };
    if error != sys::EVRInitError_VRInitError_None {
        return Err(InitError::Init_InterfaceNotFound);
    }
    Ok(result as *const T)
}

impl Context {
    pub fn system(&self) -> Result<System, InitError> {
        load(sys::IVRSystem_Version).map(|x| unsafe { System(&*x) })
    }
    pub fn application(&self) -> Result<Application, InitError> {
        load(sys::IVRApplications_Version).map(|x| unsafe { Application(&*x) })
    }
    pub fn compositor(&self) -> Result<Compositor, InitError> {
        load(sys::IVRCompositor_Version).map(|x| unsafe { Compositor(&*x) })
    }
    pub fn render_models(&self) -> Result<RenderModels, InitError> {
        load(sys::IVRRenderModels_Version).map(|x| unsafe { RenderModels(&*x) })
    }
    pub fn chaperone(&self) -> Result<Chaperone, InitError> {
        load(sys::IVRChaperone_Version).map(|x| unsafe { Chaperone(&*x) })
    }
    pub fn input(&self) -> Result<Input, InitError> {
        load(sys::IVRInput_Version).map(|x| unsafe { Input(&*x) })
    }
    pub fn settings(&self) -> Result<Settings, InitError> {
        load(sys::IVRSettings_Version).map(|x| unsafe { Settings(&*x) })
    }
    pub fn overlay(&self) -> Result<Overlay, InitError> {
        load(sys::IVROverlay_Version).map(|x| unsafe { Overlay(&*x) })
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { self.shutdown() }
    }
}

impl Context {
    /// Shut down OpenVR. Repeated calls are safe.
    ///
    /// Called implicitly by `Context::drop`.
    ///
    /// # Safety
    ///
    /// This *must* be called *before* shutting down the graphics API, or OpenVR may invoke undefined behavior by
    /// attempting to free graphics resources.
    ///
    /// No calls to other OpenVR methods may be made after this has been called unless a new `Context` is first

    /// constructed.
    pub unsafe fn shutdown(&self) {
        if self.live.swap(false,Ordering::Acquire) {
            sys::VR_ShutdownInternal();
            INITIALIZED.store(false, Ordering::Release);
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ApplicationType {
    /// Some other kind of application that isn't covered by the other entries
    Other = sys::EVRApplicationType_VRApplication_Other as isize,
    /// Application will submit 3D frames
    Scene = sys::EVRApplicationType_VRApplication_Scene as isize,
    /// Application only interacts with overlays
    Overlay = sys::EVRApplicationType_VRApplication_Overlay as isize,
    /// Application should not start SteamVR if it's not already running, and should not keep it running if everything
    /// else quits.
    Background = sys::EVRApplicationType_VRApplication_Background as isize,
    /// Init should not try to load any drivers. The application needs access to utility interfaces (like IVRSettings
    /// and IVRApplications) but not hardware.
    Utility = sys::EVRApplicationType_VRApplication_Utility as isize,
    /// Reserved for vrmonitor
    VRMonitor = sys::EVRApplicationType_VRApplication_VRMonitor as isize,
    /// Reserved for Steam
    SteamWatchdog = sys::EVRApplicationType_VRApplication_SteamWatchdog as isize,
    /// Start up SteamVR
    Bootstrapper = sys::EVRApplicationType_VRApplication_Bootstrapper as isize,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Eye {
    Left = sys::EVREye_Eye_Left as isize,
    Right = sys::EVREye_Eye_Right as isize,
}

/// Helper to call OpenVR functions that return strings
unsafe fn get_string<F: FnMut(*mut std::os::raw::c_char, u32) -> u32>(mut f: F) -> Option<CString> {
    let n = f(ptr::null_mut(), 0);
    if n == 0 {
        return None;
    }

    let mut storage = Vec::with_capacity(n as usize);
    storage.set_len(n as usize); // SAFETY: We're ensuring it will be written into properly

    let n_ = f(storage.as_mut_ptr() as *mut _, n);
    assert!(n == n_);

    storage.truncate((n - 1) as usize); // Strip trailing null
    Some(CString::from_vec_unchecked(storage))
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ControllerAxis {
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ControllerState {
    pub packet_num: u32,
    pub button_pressed: u64,
    pub button_touched: u64,
    pub axis: [ControllerAxis; 5],
}

pub mod button_id {
    use super::sys;
    pub const SYSTEM: sys::EVRButtonId = sys::EVRButtonId_k_EButton_System;
    pub const APPLICATION_MENU: sys::EVRButtonId = sys::EVRButtonId_k_EButton_ApplicationMenu;
    pub const GRIP: sys::EVRButtonId = sys::EVRButtonId_k_EButton_Grip;
    pub const DPAD_LEFT: sys::EVRButtonId = sys::EVRButtonId_k_EButton_DPad_Left;
    pub const DPAD_UP: sys::EVRButtonId = sys::EVRButtonId_k_EButton_DPad_Up;
    pub const DPAD_RIGHT: sys::EVRButtonId = sys::EVRButtonId_k_EButton_DPad_Right;
    pub const DPAD_DOWN: sys::EVRButtonId = sys::EVRButtonId_k_EButton_DPad_Down;
    pub const A: sys::EVRButtonId = sys::EVRButtonId_k_EButton_A;
    pub const PROXIMITY_SENSOR: sys::EVRButtonId = sys::EVRButtonId_k_EButton_ProximitySensor;
    pub const AXIS0: sys::EVRButtonId = sys::EVRButtonId_k_EButton_Axis0;
    pub const AXIS1: sys::EVRButtonId = sys::EVRButtonId_k_EButton_Axis1;
    pub const AXIS2: sys::EVRButtonId = sys::EVRButtonId_k_EButton_Axis2;
    pub const AXIS3: sys::EVRButtonId = sys::EVRButtonId_k_EButton_Axis3;
    pub const AXIS4: sys::EVRButtonId = sys::EVRButtonId_k_EButton_Axis4;
    pub const STEAM_VR_TOUCHPAD: sys::EVRButtonId = sys::EVRButtonId_k_EButton_SteamVR_Touchpad;
    pub const STEAM_VR_TRIGGER: sys::EVRButtonId = sys::EVRButtonId_k_EButton_SteamVR_Trigger;
    pub const DASHBOARD_BACK: sys::EVRButtonId = sys::EVRButtonId_k_EButton_Dashboard_Back;
    pub const MAX: sys::EVRButtonId = sys::EVRButtonId_k_EButton_Max;
}
pub struct TextureBounds(pub sys::VRTextureBounds_t);
impl Clone for TextureBounds {
    fn clone(&self) -> Self {
        Self(sys::VRTextureBounds_t {
            uMin: self.0.uMin,
            vMin: self.0.vMin,
            uMax: self.0.uMax,
            vMax: self.0.vMax,
        })
    }
}
impl Debug for TextureBounds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TextureBounds")
            .field("uMin", &self.0.uMin)
            .field("vMin", &self.0.vMin)
            .field("uMax", &self.0.uMax)
            .field("vMax", &self.0.vMax)
            .finish()
    }
}
/// Tints each color channel by multiplying it with the given f32
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ColorTint {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
impl Default for ColorTint {
    fn default() -> Self {
        Self {
            r: 1.,
            g: 1.,
            b: 1.,
            a: 1.,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::ptr;

    /// Mock function that simulates an OpenVR function returning a string.
    fn mock_success(output: *mut std::os::raw::c_char, size: u32) -> u32 {
        let text = "OpenVR Test";
        let c_str = CString::new(text).unwrap();
        let bytes = c_str.as_bytes_with_nul();

        if size == 0 {
            return bytes.len() as u32; // First call, return required size
        }

        unsafe {
            ptr::copy_nonoverlapping(bytes.as_ptr(), output as *mut u8, bytes.len());
        }

        bytes.len() as u32
    }

    /// Mock function that simulates an OpenVR function returning an empty string (no data).
    fn mock_empty(_: *mut std::os::raw::c_char, _: u32) -> u32 {
        0
    }

    #[test]
    fn test_get_string_success() {
        let result = unsafe { get_string(mock_success) };
        assert!(result.is_some());
        assert_eq!(result.unwrap().to_str().unwrap(), "OpenVR Test");
    }

    #[test]
    fn test_get_string_empty() {
        let result = unsafe { get_string(mock_empty) };
        assert!(result.is_none());
    }

    #[test]
    fn test_get_string_function_called_twice() {
        // Get string should call once for the size and second time to copy
        // https://github.com/ValveSoftware/openvr/wiki/IVRSystem::GetTrackedDeviceProperty

        let mut call_count = 0;

        let mock_resizing = |output: *mut std::os::raw::c_char, size: u32| -> u32 {
            call_count += 1;
            let text = "Resize Test";
            let c_str = CString::new(text).unwrap();
            let bytes = c_str.as_bytes_with_nul();

            if size == 0 {
                return bytes.len() as u32;
            }

            unsafe {
                ptr::copy_nonoverlapping(bytes.as_ptr(), output as *mut u8, bytes.len());
            }

            bytes.len() as u32
        };

        let result = unsafe { get_string(mock_resizing) };

        assert!(result.is_some());
        assert_eq!(result.unwrap().to_str().unwrap(), "Resize Test");
        // Ensure it was called twice (first to get size, second to write)
        assert_eq!(call_count, 2);
    }

    #[test]
    fn test_defaultable_pose() {
        /* we expect this type to be default initialized */
        let pose = TrackedDevicePose::default();

        for x in 0..3 {
            for y in 0..4 {
                assert_eq!(pose.device_to_absolute_tracking()[x][y], 0f32);
            }
        }
    }
}
