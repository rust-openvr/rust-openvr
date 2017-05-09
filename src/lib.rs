extern crate openvr_sys;

use std::sync::atomic::{Ordering, AtomicBool, ATOMIC_BOOL_INIT};
use std::{fmt, error, slice};
use std::ffi::CStr;

use openvr_sys as sys;

mod tracking;

mod system;
mod compositor;

pub use tracking::*;

static INITIALIZED: AtomicBool = ATOMIC_BOOL_INIT;

/// Initialize OpenVR
///
/// # Panics
/// When the library has already been initialized
pub fn init(ty: ApplicationType) -> Result<Context, InitError> {
    if INITIALIZED.swap(true, Ordering::AcqRel) {
        panic!("OpenVR has already been initialized!");
    }

    let mut error = sys::EVRInitError_EVRInitError_VRInitError_None;
    unsafe { sys::VR_InitInternal(&mut error, ty as sys::EVRApplicationType) };
    if error != sys::EVRInitError_EVRInitError_VRInitError_None {
        return Err(InitError(error));
    }
    if !unsafe { sys::VR_IsInterfaceVersionValid(sys::IVRSystem_Version.as_ptr() as *const i8) } {
        unsafe { sys::VR_ShutdownInternal() }
        return Err(InitError(sys::EVRInitError_EVRInitError_VRInitError_Init_InterfaceNotFound));
    }
    Ok(unsafe { Context::new() }?)
}

pub struct System<'a>(&'a sys::VR_IVRSystem_FnTable);
pub struct Compositor<'a>(&'a sys::VR_IVRCompositor_FnTable);

/// Entry points into OpenVR.
///
/// At most one of this object may exist at a time.
pub struct Context {
    system: *const sys::VR_IVRSystem_FnTable,
    compositor: *const sys::VR_IVRCompositor_FnTable,
}

impl Context {
    /// Must be called after sys::VR_InitInternal
    unsafe fn new() -> Result<Self, InitError> {
        fn load<T>(suffix: &[u8]) -> Result<*const T, InitError> {
            let mut magic = Vec::from(b"FnTable:".as_ref());
            magic.extend(suffix);
            let mut error = sys::EVRInitError_EVRInitError_VRInitError_None;
            let result = unsafe { sys::VR_GetGenericInterface(magic.as_ptr() as *const i8, &mut error) };
            if error != sys::EVRInitError_EVRInitError_VRInitError_None {
                return Err(InitError(sys::EVRInitError_EVRInitError_VRInitError_Init_InterfaceNotFound));
            }
            Ok(result as *const T)
        }

        Ok(Context {
            system: load(sys::IVRSystem_Version)?,
            compositor: load(sys::IVRCompositor_Version)?,
        })
    }

    pub fn system(&self) -> System { unsafe { System(&*self.system) } }
    pub fn compositor(&self) -> Compositor { unsafe { Compositor(&*self.compositor) } }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { sys::VR_ShutdownInternal() }
        INITIALIZED.store(false, Ordering::AcqRel);
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ApplicationType {
    /// Some other kind of application that isn't covered by the other entries 
    Other = sys::EVRApplicationType_EVRApplicationType_VRApplication_Other as isize,
    /// Application will submit 3D frames
    Scene = sys::EVRApplicationType_EVRApplicationType_VRApplication_Scene as isize,
    /// Application only interacts with overlays
    Overlay = sys::EVRApplicationType_EVRApplicationType_VRApplication_Overlay as isize,
    /// Application should not start SteamVR if it's not already running, and should not keep it running if everything
    /// else quits.
    Background = sys::EVRApplicationType_EVRApplicationType_VRApplication_Background as isize,
    /// Init should not try to load any drivers. The application needs access to utility interfaces (like IVRSettings
    /// and IVRApplications) but not hardware.
    Utility = sys::EVRApplicationType_EVRApplicationType_VRApplication_Utility as isize,
    /// Reserved for vrmonitor
    VRMonitor = sys::EVRApplicationType_EVRApplicationType_VRApplication_VRMonitor as isize,
    /// Reserved for Steam
    SteamWatchdog = sys::EVRApplicationType_EVRApplicationType_VRApplication_SteamWatchdog as isize,
    /// Start up SteamVR
    Bootstrapper = sys::EVRApplicationType_EVRApplicationType_VRApplication_Bootstrapper as isize,
}

pub struct InitError(sys::EVRInitError);

impl fmt::Debug for InitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = unsafe {
            CStr::from_ptr(sys::VR_GetVRInitErrorAsSymbol(self.0))
        };
        f.pad(msg.to_str().expect("OpenVR init error symbol was not valid UTF-8"))
    }
}

impl error::Error for InitError {
    fn description(&self) -> &str {
        let msg = unsafe {
            CStr::from_ptr(sys::VR_GetVRInitErrorAsEnglishDescription(self.0))
        };
        msg.to_str().expect("OpenVR init error description was not valid UTF-8")
    }
}

impl fmt::Display for InitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(error::Error::description(self))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Eye {
    Left = sys::EVREye_EVREye_Eye_Left as isize,
    Right = sys::EVREye_EVREye_Eye_Right as isize,
}

#[derive(Debug, Copy, Clone)]
pub struct TrackedDevicePoses {
    data: [TrackedDevicePose; sys::k_unMaxTrackedDeviceCount as usize]
}

impl TrackedDevicePoses {
    pub fn iter(&self) -> slice::Iter<TrackedDevicePose> { self.data.iter() }
    pub fn len(&self) -> usize { self.data.len() }
}
