use openvr_sys as sys;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TrackingUniverseOrigin {
    Seated = sys::ETrackingUniverseOrigin_TrackingUniverseSeated as isize,
    Standing = sys::ETrackingUniverseOrigin_TrackingUniverseStanding as isize,
    RawAndUncalibrated = sys::ETrackingUniverseOrigin_TrackingUniverseRawAndUncalibrated as isize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TrackedDevicePose(sys::TrackedDevicePose_t);

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

pub type TrackedDeviceIndex = sys::TrackedDeviceIndex_t;

pub mod tracked_device_index {
    use super::*;
    pub const HMD: TrackedDeviceIndex = sys::k_unTrackedDeviceIndex_Hmd;
    pub const INVALID: TrackedDeviceIndex = sys::k_unTrackedDeviceIndexInvalid;
}

pub type TrackedDeviceProperty = sys::ETrackedDeviceProperty;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TrackedControllerRole {
    LeftHand = sys::ETrackedControllerRole_TrackedControllerRole_LeftHand as isize,
    RightHand = sys::ETrackedControllerRole_TrackedControllerRole_RightHand as isize,
}

pub const MAX_TRACKED_DEVICE_COUNT: usize = sys::k_unMaxTrackedDeviceCount as usize;

pub type TrackedDevicePoses = [TrackedDevicePose; MAX_TRACKED_DEVICE_COUNT];
