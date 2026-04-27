use openvr_sys as sys;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TrackingUniverseOrigin {
    Seated = sys::ETrackingUniverseOrigin_TrackingUniverseSeated as isize,
    Standing = sys::ETrackingUniverseOrigin_TrackingUniverseStanding as isize,
    RawAndUncalibrated = sys::ETrackingUniverseOrigin_TrackingUniverseRawAndUncalibrated as isize,
}
impl From<TrackingUniverseOrigin> for isize{
    fn from(value: TrackingUniverseOrigin) -> Self {
        match value {
            TrackingUniverseOrigin::Seated => sys::ETrackingUniverseOrigin_TrackingUniverseSeated as isize,
            TrackingUniverseOrigin::Standing => sys::ETrackingUniverseOrigin_TrackingUniverseStanding as isize,
            TrackingUniverseOrigin::RawAndUncalibrated => sys::ETrackingUniverseOrigin_TrackingUniverseRawAndUncalibrated as isize,
        }
    }
}
//fixme:
//for some reason bindings to SetOverlayTransformAbsolute expect origin to be u32 on linux (at least on my OS) and i32 on windows
//this enum can be: 0,1,2 so it shouldn't cause any issues as underlaying bits are the same
impl From<TrackingUniverseOrigin> for i32{
    fn from(value: TrackingUniverseOrigin) -> Self {
       match value {
            TrackingUniverseOrigin::Seated => sys::ETrackingUniverseOrigin_TrackingUniverseSeated as i32,
            TrackingUniverseOrigin::Standing => sys::ETrackingUniverseOrigin_TrackingUniverseStanding as i32,
            TrackingUniverseOrigin::RawAndUncalibrated => sys::ETrackingUniverseOrigin_TrackingUniverseRawAndUncalibrated as i32,
        }
    }
}
impl From<TrackingUniverseOrigin> for u32{
    fn from(value: TrackingUniverseOrigin) -> Self {
       match value {
            TrackingUniverseOrigin::Seated => sys::ETrackingUniverseOrigin_TrackingUniverseSeated as u32,
            TrackingUniverseOrigin::Standing => sys::ETrackingUniverseOrigin_TrackingUniverseStanding as u32,
            TrackingUniverseOrigin::RawAndUncalibrated => sys::ETrackingUniverseOrigin_TrackingUniverseRawAndUncalibrated as u32,
        }
    }
}
#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct TrackedDevicePose(pub sys::TrackedDevicePose_t);

impl TrackedDevicePose {
    pub fn device_to_absolute_tracking(&self) -> &[[f32; 4]; 3] {
        &self.0.mDeviceToAbsoluteTracking.m
    }
    pub fn velocity(&self) -> &[f32; 3] {
        &self.0.vVelocity.v
    }
    pub fn angular_velocity(&self) -> &[f32; 3] {
        &self.0.vAngularVelocity.v
    }
    pub fn tracking_result(&self) -> TrackingResult {
        use self::TrackingResult::*;
        match self.0.eTrackingResult {
            sys::ETrackingResult_TrackingResult_Uninitialized => Uninitialized,
            sys::ETrackingResult_TrackingResult_Calibrating_InProgress => CalibratingInProgress,
            sys::ETrackingResult_TrackingResult_Calibrating_OutOfRange => CalibratingOutOfRange,
            sys::ETrackingResult_TrackingResult_Running_OK => OK,
            sys::ETrackingResult_TrackingResult_Running_OutOfRange => RunningOutOfRange,
            _ => panic!("unrecognized tracking result"),
        }
    }
    pub fn pose_is_valid(&self) -> bool {
        self.0.bPoseIsValid
    }
    pub fn device_is_connected(&self) -> bool {
        self.0.bDeviceIsConnected
    }
}

impl From<sys::TrackedDevicePose_t> for TrackedDevicePose {
    fn from(x: sys::TrackedDevicePose_t) -> Self {
        TrackedDevicePose(x)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TrackingResult {
    Uninitialized = sys::ETrackingResult_TrackingResult_Uninitialized as isize,
    CalibratingInProgress = sys::ETrackingResult_TrackingResult_Calibrating_InProgress as isize,
    CalibratingOutOfRange = sys::ETrackingResult_TrackingResult_Calibrating_OutOfRange as isize,
    OK = sys::ETrackingResult_TrackingResult_Running_OK as isize,
    RunningOutOfRange = sys::ETrackingResult_TrackingResult_Running_OutOfRange as isize,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TrackedDeviceClass {
    Invalid = sys::ETrackedDeviceClass_TrackedDeviceClass_Invalid as isize,
    HMD = sys::ETrackedDeviceClass_TrackedDeviceClass_HMD as isize,
    Controller = sys::ETrackedDeviceClass_TrackedDeviceClass_Controller as isize,
    GenericTracker = sys::ETrackedDeviceClass_TrackedDeviceClass_GenericTracker as isize,
    TrackingReference = sys::ETrackedDeviceClass_TrackedDeviceClass_TrackingReference as isize,
    DisplayRedirect = sys::ETrackedDeviceClass_TrackedDeviceClass_DisplayRedirect as isize,
}
#[derive(Clone, Copy,PartialEq,Debug)]
pub struct TrackedDeviceIndex(pub sys::TrackedDeviceIndex_t);

pub mod tracked_device_index {
    use super::*;
    pub const HMD: TrackedDeviceIndex = TrackedDeviceIndex(sys::k_unTrackedDeviceIndex_Hmd as u32);
    pub const INVALID: TrackedDeviceIndex = TrackedDeviceIndex(sys::k_unTrackedDeviceIndexInvalid as u32);
}
#[derive(Clone, Copy,PartialEq,Debug)]
pub struct TrackedDeviceProperty(pub sys::ETrackedDeviceProperty);

#[derive(Clone, Copy,PartialEq,Debug)]
pub enum TrackedControllerRole {
    LeftHand = sys::ETrackedControllerRole_TrackedControllerRole_LeftHand as isize,
    RightHand = sys::ETrackedControllerRole_TrackedControllerRole_RightHand as isize,
    Invalid= sys::ETrackedControllerRole_TrackedControllerRole_Invalid as isize,
    OptOut= sys::ETrackedControllerRole_TrackedControllerRole_OptOut as isize,
    Treadmill= sys::ETrackedControllerRole_TrackedControllerRole_Treadmill as isize,
    Stylus= sys::ETrackedControllerRole_TrackedControllerRole_Stylus as isize,

}

pub const MAX_TRACKED_DEVICE_COUNT: usize = sys::k_unMaxTrackedDeviceCount as usize;
pub type TrackedDevicePoses=[TrackedDevicePose; MAX_TRACKED_DEVICE_COUNT];
