use openvr_sys as sys;
use super::{System, property::{Property, PropertyValue, TrackedDevicePropertyResult}};

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

/// A tracked device.
#[derive(Copy, Clone)]
pub struct TrackedDevice<'a> {
    index: u32,
    system: &'a System
}
impl<'a> TrackedDevice<'a> {
    /// Constructs a `TrackedDevice` belonging to the specified `System` with the specified index.
    #[inline(always)]
    pub fn from_system_and_index(system: &'a System, index: u32) -> Self {
        Self {system, index}
    }
    /// Returns the `System` this tracked device belongs to.
    #[inline(always)]
    pub fn system(&self) -> &System {
        self.system
    }
    /// Returns the tracked device index for this device.
    #[inline(always)]
    pub fn index(self) -> u32 {
        self.index
    }

    /// Returns the tracked device class for this device.
    #[inline]
    pub fn class(self) -> TrackedDeviceClass {
        use self::TrackedDeviceClass::*;
        match unsafe { self.system.0.GetTrackedDeviceClass.unwrap()(self.index) } {
            sys::ETrackedDeviceClass_TrackedDeviceClass_Invalid => Invalid,
            sys::ETrackedDeviceClass_TrackedDeviceClass_HMD => HMD,
            sys::ETrackedDeviceClass_TrackedDeviceClass_Controller => Controller,
            sys::ETrackedDeviceClass_TrackedDeviceClass_GenericTracker => GenericTracker,
            sys::ETrackedDeviceClass_TrackedDeviceClass_TrackingReference => TrackingReference,
            sys::ETrackedDeviceClass_TrackedDeviceClass_DisplayRedirect => DisplayRedirect,
            _ => Invalid,
        }
    }
    /// Returns the controller role associated with this tracked device, or `None` if it's not a tracked controller or another problem with fetching the role ocurred.
    #[inline]
    pub fn role(self) -> Option<TrackedControllerRole> {
        let x = unsafe { self.system().0.GetControllerRoleForTrackedDeviceIndex.unwrap()(self.index) };
        match x {
            sys::ETrackedControllerRole_TrackedControllerRole_LeftHand => {
                Some(TrackedControllerRole::LeftHand)
            }
            sys::ETrackedControllerRole_TrackedControllerRole_RightHand => {
                Some(TrackedControllerRole::RightHand)
            }
            _ => None,
        }
    }
    /// Returns `true` if the tracked device is connected, i.e. corresponds to a real tracked device which is ready to use; `false` otherwise.
    #[inline]
    pub fn is_connected(self) -> bool {
        unsafe { self.system().0.IsTrackedDeviceConnected.unwrap()(self.index) }
    }
    /// Returns the specified property, or `TrackedDevicePropertyError` if something went wrong.
    ///
    /// Consult the `TrackedDevicePropertyResult` for more on what could fail here.
    ///
    /// *This method is intended to be used with the turbofish syntax:*
    /// ```rust,no-run
    /// let my_string_property = tracked_device.get_property::<String>(Property::AnInterestingStringProperty);
    /// ```
    /// Alternatively, you can annotate the type of the variable you're binding to.
    #[inline(always)]
    pub fn get_property<T: PropertyValue>(self, property: Property) -> TrackedDevicePropertyResult<T> {
        T::get_property(self, property)
    }
}