//! Tracked device property handling.
//!
//! This module provides a comprehensive list of all known tracked device properties. Use the `TrackingDevice`'s `get_property` method to use this in a convenient and readable way.

use openvr_sys as sys;
use super::{tracking::TrackedDevice, get_string};
use std::{fmt, mem};

/// The properties of a tracked device.
// TODO: Add documentation for each of these properties. Right now they only have type info.
// TODO: Add support for array properties. These should not be handled as usual properties: we need a separate enum an a separate set of methods for that.
#[repr(u32)]
#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Property {
    /// Will always fail when querying, and thus does not have a type.
    Invalid = sys::ETrackedDeviceProperty_Prop_Invalid,
    /// *Type: `String`*
    TrackingSystemName = sys::ETrackedDeviceProperty_Prop_TrackingSystemName_String,
    /// *Type: `String`*
    ModelNumber = sys::ETrackedDeviceProperty_Prop_ModelNumber_String,
    /// *Type: `String`*
    SerialNumber = sys::ETrackedDeviceProperty_Prop_SerialNumber_String,
    /// *Type: `String`*
    RenderModelName = sys::ETrackedDeviceProperty_Prop_RenderModelName_String,
    /// *Type: `bool`*
    WillDriftInYaw = sys::ETrackedDeviceProperty_Prop_WillDriftInYaw_Bool,
    /// *Type: `String`*
    ManufacturerName = sys::ETrackedDeviceProperty_Prop_ManufacturerName_String,
    /// *Type: `String`*
    TrackingFirmwareVersion = sys::ETrackedDeviceProperty_Prop_TrackingFirmwareVersion_String,
    /// *Type: `String`*
    HardwareRevision = sys::ETrackedDeviceProperty_Prop_HardwareRevision_String,
    /// *Type: `String`*
    AllWirelessDongleDescriptions = sys::ETrackedDeviceProperty_Prop_AllWirelessDongleDescriptions_String,
    /// *Type: `String`*
    ConnectedWirelessDongle = sys::ETrackedDeviceProperty_Prop_ConnectedWirelessDongle_String,
    /// *Type: `bool`*
    DeviceIsWireless = sys::ETrackedDeviceProperty_Prop_DeviceIsWireless_Bool,
    /// *Type: `bool`*
    DeviceIsCharging = sys::ETrackedDeviceProperty_Prop_DeviceIsCharging_Bool,
    /// *Type: `f32`*
    DeviceBatteryPercentage = sys::ETrackedDeviceProperty_Prop_DeviceBatteryPercentage_Float,
    /// *Type: 3x4 matrix ([`Matrix34`][m34])*
    ///
    /// [m34]: type.Matrix34.html "Matrix34 — a 3x4 matrix, as a tracked device property value"
    StatusDisplayTransform = sys::ETrackedDeviceProperty_Prop_StatusDisplayTransform_Matrix34,
    /// *Type: `bool`*
    FirmwareUpdateAvailable = sys::ETrackedDeviceProperty_Prop_Firmware_UpdateAvailable_Bool,
    /// *Type: `bool`*
    FirmwareManualUpdate = sys::ETrackedDeviceProperty_Prop_Firmware_ManualUpdate_Bool,
    /// *Type: `String`*
    FirmwareManualUpdateURL = sys::ETrackedDeviceProperty_Prop_Firmware_ManualUpdateURL_String,
    /// *Type: `u64`*
    HardwareRevisionNumber = sys::ETrackedDeviceProperty_Prop_HardwareRevision_Uint64,
    /// *Type: `u64`*
    FirmwareVersion = sys::ETrackedDeviceProperty_Prop_FirmwareVersion_Uint64,
    /// *Type: `u64`*
    FPGAVersion = sys::ETrackedDeviceProperty_Prop_FPGAVersion_Uint64,
    /// *Type: `u64`*
    VRCVersion = sys::ETrackedDeviceProperty_Prop_VRCVersion_Uint64,
    /// *Type: `u64`*
    RadioVersion = sys::ETrackedDeviceProperty_Prop_RadioVersion_Uint64,
    /// *Type: `u64`*
    DongleVersion = sys::ETrackedDeviceProperty_Prop_DongleVersion_Uint64,
    /// *Type: `bool`*
    BlockServerShutdown = sys::ETrackedDeviceProperty_Prop_BlockServerShutdown_Bool,
    /// *Type: `bool`*
    CanUnifyCoordinateSystemWithHmd = sys::ETrackedDeviceProperty_Prop_CanUnifyCoordinateSystemWithHmd_Bool,
    /// *Type: `bool`*
    ContainsProximitySensor = sys::ETrackedDeviceProperty_Prop_ContainsProximitySensor_Bool,
    /// *Type: `bool`*
    DeviceProvidesBatteryStatus = sys::ETrackedDeviceProperty_Prop_DeviceProvidesBatteryStatus_Bool,
    /// *Type: `bool`*
    DeviceCanPowerOff = sys::ETrackedDeviceProperty_Prop_DeviceCanPowerOff_Bool,
    /// *Type: `String`*
    FirmwareProgrammingTarget = sys::ETrackedDeviceProperty_Prop_Firmware_ProgrammingTarget_String,
    /// *Type: `i32`*
    DeviceClass = sys::ETrackedDeviceProperty_Prop_DeviceClass_Int32,
    /// *Type: `bool`*
    HasCamera = sys::ETrackedDeviceProperty_Prop_HasCamera_Bool,
    DriverVersion = sys::ETrackedDeviceProperty_Prop_DriverVersion_String,
    /// *Type: `bool`*
    FirmwareForceUpdateRequired = sys::ETrackedDeviceProperty_Prop_Firmware_ForceUpdateRequired_Bool,
    /// *Type: `bool`*
    ViveSystemButtonFixRequired = sys::ETrackedDeviceProperty_Prop_ViveSystemButtonFixRequired_Bool,
    ParentDriver = sys::ETrackedDeviceProperty_Prop_ParentDriver_Uint64,
    /// *Type: `String`*
    ResourceRoot = sys::ETrackedDeviceProperty_Prop_ResourceRoot_String,
    /// *Type: `bool`*
    ReportsTimeSinceVSync = sys::ETrackedDeviceProperty_Prop_ReportsTimeSinceVSync_Bool,
    /// *Type: `f32`*
    SecondsFromVsyncToPhotons = sys::ETrackedDeviceProperty_Prop_SecondsFromVsyncToPhotons_Float,
    /// *Type: `f32`*
    DisplayFrequency = sys::ETrackedDeviceProperty_Prop_DisplayFrequency_Float,
    /// *Type: `f32`*
    UserIpdMeters = sys::ETrackedDeviceProperty_Prop_UserIpdMeters_Float,
    /// *Type: `u64`*
    CurrentUniverseId = sys::ETrackedDeviceProperty_Prop_CurrentUniverseId_Uint64,
    /// *Type: `u64`*
    PreviousUniverseId = sys::ETrackedDeviceProperty_Prop_PreviousUniverseId_Uint64,
    /// *Type: `u64`*
    DisplayFirmwareVersion = sys::ETrackedDeviceProperty_Prop_DisplayFirmwareVersion_Uint64,
    /// *Type: `bool`*
    IsOnDesktop = sys::ETrackedDeviceProperty_Prop_IsOnDesktop_Bool,
    /// *Type: `i32`*
    DisplayMCType = sys::ETrackedDeviceProperty_Prop_DisplayMCType_Int32,
    /// *Type: `f32`*
    DisplayMCOffset = sys::ETrackedDeviceProperty_Prop_DisplayMCOffset_Float,
    /// *Type: `f32`*
    DisplayMCScale = sys::ETrackedDeviceProperty_Prop_DisplayMCScale_Float,
    /// *Type: `i32`*
    EdidVendorID = sys::ETrackedDeviceProperty_Prop_EdidVendorID_Int32,
    /// *Type: `String`*
    DisplayMCImageLeft = sys::ETrackedDeviceProperty_Prop_DisplayMCImageLeft_String,
    /// *Type: `String`*
    DisplayMCImageRight = sys::ETrackedDeviceProperty_Prop_DisplayMCImageRight_String,
    DisplayGCBlackClamp = sys::ETrackedDeviceProperty_Prop_DisplayGCBlackClamp_Float,
    EdidProductID = sys::ETrackedDeviceProperty_Prop_EdidProductID_Int32,
    /// *Type: 3x4 matrix ([`Matrix34`][m34])*
    ///
    /// [m34]: type.Matrix34.html "Matrix34 — a 3x4 matrix, as a tracked device property value"
    CameraToHeadTransform = sys::ETrackedDeviceProperty_Prop_CameraToHeadTransform_Matrix34,
    DisplayGCType = sys::ETrackedDeviceProperty_Prop_DisplayGCType_Int32,
    /// *Type: `f32`*
    DisplayGCOffset = sys::ETrackedDeviceProperty_Prop_DisplayGCOffset_Float,
    /// *Type: `f32`*
    DisplayGCScale = sys::ETrackedDeviceProperty_Prop_DisplayGCScale_Float,
    /// *Type: `f32`*
    DisplayGCPrescale = sys::ETrackedDeviceProperty_Prop_DisplayGCPrescale_Float,
    DisplayGCImage = sys::ETrackedDeviceProperty_Prop_DisplayGCImage_String,
    /// *Type: `f32`*
    LensCenterLeftU = sys::ETrackedDeviceProperty_Prop_LensCenterLeftU_Float,
    /// *Type: `f32`*
    LensCenterLeftV = sys::ETrackedDeviceProperty_Prop_LensCenterLeftV_Float,
    /// *Type: `f32`*
    LensCenterRightU = sys::ETrackedDeviceProperty_Prop_LensCenterRightU_Float,
    /// *Type: `f32`*
    LensCenterRightV = sys::ETrackedDeviceProperty_Prop_LensCenterRightV_Float,
    /// *Type: `f32`*
    UserHeadToEyeDepthMeters = sys::ETrackedDeviceProperty_Prop_UserHeadToEyeDepthMeters_Float,
    /// *Type: `u64`*
    CameraFirmwareVersion = sys::ETrackedDeviceProperty_Prop_CameraFirmwareVersion_Uint64,
    /// *Type: `String`*
    CameraFirmwareDescription = sys::ETrackedDeviceProperty_Prop_CameraFirmwareDescription_String,
    /// *Type: `u64`*
    DisplayFPGAVersion = sys::ETrackedDeviceProperty_Prop_DisplayFPGAVersion_Uint64,
    /// *Type: `u64`*
    DisplayBootloaderVersion = sys::ETrackedDeviceProperty_Prop_DisplayBootloaderVersion_Uint64,
    /// *Type: `u64`*
    DisplayHardwareVersion = sys::ETrackedDeviceProperty_Prop_DisplayHardwareVersion_Uint64,
    /// *Type: `u64`*
    AudioFirmwareVersion = sys::ETrackedDeviceProperty_Prop_AudioFirmwareVersion_Uint64,
    /// *Type: `i32`*
    CameraCompatibilityMode = sys::ETrackedDeviceProperty_Prop_CameraCompatibilityMode_Int32,
    /// *Type: `f32`*
    ScreenshotHorizontalFieldOfViewDegrees = sys::ETrackedDeviceProperty_Prop_ScreenshotHorizontalFieldOfViewDegrees_Float,
    /// *Type: `f32`*
    ScreenshotVerticalFieldOfViewDegrees = sys::ETrackedDeviceProperty_Prop_ScreenshotVerticalFieldOfViewDegrees_Float,
    /// *Type: `bool`*
    DisplaySuppressed = sys::ETrackedDeviceProperty_Prop_DisplaySuppressed_Bool,
    /// *Type: `bool`*
    DisplayAllowNightMode = sys::ETrackedDeviceProperty_Prop_DisplayAllowNightMode_Bool,
    /// *Type: `i32`*
    DisplayMCImageWidth = sys::ETrackedDeviceProperty_Prop_DisplayMCImageWidth_Int32,
    /// *Type: `i32`*
    DisplayMCImageHeight = sys::ETrackedDeviceProperty_Prop_DisplayMCImageHeight_Int32,
    /// *Type: `i32`*
    DisplayMCImageNumChannels = sys::ETrackedDeviceProperty_Prop_DisplayMCImageNumChannels_Int32,
    // FIXME: No idea what to do here.
    //DisplayMCImageData_Binary = sys::ETrackedDeviceProperty_Prop_DisplayMCImageData_Binary,
    /// *Type: `f32`*
    SecondsFromPhotonsToVblank = sys::ETrackedDeviceProperty_Prop_SecondsFromPhotonsToVblank_Float,
    /// *Type: `bool`*
    DriverDirectModeSendsVsyncEvents = sys::ETrackedDeviceProperty_Prop_DriverDirectModeSendsVsyncEvents_Bool,
    /// *Type: `bool`*
    DisplayDebugMode = sys::ETrackedDeviceProperty_Prop_DisplayDebugMode_Bool,
    /// *Type: `u64`*
    GraphicsAdapterLuid = sys::ETrackedDeviceProperty_Prop_GraphicsAdapterLuid_Uint64,
    /// *Type: `String`*
    AttachedDeviceId = sys::ETrackedDeviceProperty_Prop_AttachedDeviceId_String,
    /// *Type: `u64`*
    SupportedButtons = sys::ETrackedDeviceProperty_Prop_SupportedButtons_Uint64,
    /// *Type: `i32`*
    Axis0Type = sys::ETrackedDeviceProperty_Prop_Axis0Type_Int32,
    /// *Type: `i32`*
    Axis1Type = sys::ETrackedDeviceProperty_Prop_Axis1Type_Int32,
    /// *Type: `i32`*
    Axis2Type = sys::ETrackedDeviceProperty_Prop_Axis2Type_Int32,
    /// *Type: `i32`*
    Axis3Type = sys::ETrackedDeviceProperty_Prop_Axis3Type_Int32,
    /// *Type: `i32`*
    Axis4Type = sys::ETrackedDeviceProperty_Prop_Axis4Type_Int32,
    ControllerRoleHint = sys::ETrackedDeviceProperty_Prop_ControllerRoleHint_Int32,
    /// *Type: `f32`*
    FieldOfViewLeftDegrees = sys::ETrackedDeviceProperty_Prop_FieldOfViewLeftDegrees_Float,
    /// *Type: `f32`*
    FieldOfViewRightDegrees = sys::ETrackedDeviceProperty_Prop_FieldOfViewRightDegrees_Float,
    /// *Type: `f32`*
    FieldOfViewTopDegrees = sys::ETrackedDeviceProperty_Prop_FieldOfViewTopDegrees_Float,
    /// *Type: `f32`*
    FieldOfViewBottomDegrees = sys::ETrackedDeviceProperty_Prop_FieldOfViewBottomDegrees_Float,
    /// *Type: `f32`*
    TrackingRangeMinimumMeters = sys::ETrackedDeviceProperty_Prop_TrackingRangeMinimumMeters_Float,
    /// *Type: `f32`*
    TrackingRangeMaximumMeters = sys::ETrackedDeviceProperty_Prop_TrackingRangeMaximumMeters_Float,
    /// *Type: `String`*
    ModeLabel = sys::ETrackedDeviceProperty_Prop_ModeLabel_String,
    /// *Type: `String`*
    IconPathName = sys::ETrackedDeviceProperty_Prop_IconPathName_String,
    /// *Type: `String`*
    NamedIconPathDeviceOff = sys::ETrackedDeviceProperty_Prop_NamedIconPathDeviceOff_String,
    /// *Type: `String`*
    NamedIconPathDeviceSearching = sys::ETrackedDeviceProperty_Prop_NamedIconPathDeviceSearching_String,
    /// *Type: `String`*
    NamedIconPathDeviceSearchingAlert = sys::ETrackedDeviceProperty_Prop_NamedIconPathDeviceSearchingAlert_String,
    /// *Type: `String`*
    NamedIconPathDeviceReady = sys::ETrackedDeviceProperty_Prop_NamedIconPathDeviceReady_String,
    /// *Type: `String`*
    NamedIconPathDeviceReadyAlert = sys::ETrackedDeviceProperty_Prop_NamedIconPathDeviceReadyAlert_String,
    /// *Type: `String`*
    NamedIconPathDeviceNotReady = sys::ETrackedDeviceProperty_Prop_NamedIconPathDeviceNotReady_String,
    /// *Type: `String`*
    NamedIconPathDeviceStandby = sys::ETrackedDeviceProperty_Prop_NamedIconPathDeviceStandby_String,
    /// *Type: `String`*
    NamedIconPathDeviceAlertLow = sys::ETrackedDeviceProperty_Prop_NamedIconPathDeviceAlertLow_String,
    /// *Type: `String`*
    UserConfigPath = sys::ETrackedDeviceProperty_Prop_UserConfigPath_String,
    /// *Type: `String`*
    InstallPath = sys::ETrackedDeviceProperty_Prop_InstallPath_String,
    /// *Type: `bool`*
    HasDisplayComponent = sys::ETrackedDeviceProperty_Prop_HasDisplayComponent_Bool,
    /// *Type: `bool`*
    HasControllerComponent = sys::ETrackedDeviceProperty_Prop_HasControllerComponent_Bool,
    /// *Type: `bool`*
    HasCameraComponent = sys::ETrackedDeviceProperty_Prop_HasCameraComponent_Bool,
    /// *Type: `bool`*
    HasDriverDirectModeComponent = sys::ETrackedDeviceProperty_Prop_HasDriverDirectModeComponent_Bool,
    /// *Type: `bool`*
    HasVirtualDisplayComponent = sys::ETrackedDeviceProperty_Prop_HasVirtualDisplayComponent_Bool,
}
impl Into<u32> for Property {
    // In Rust 2018, we would've written this as `From<Property> for u32`. This crate is on 2015, though.
    fn into(self) -> u32 {
        // This is guaranteed to be 100% safe, thanks to #[repr(u32)].
        unsafe {std::mem::transmute(self)}
    }
}

/// A 3x4 matrix, as a tracked device property value.
pub type Matrix34 = [[f32; 4]; 3];

/// For types which can be extracted from a tracked device property.
pub trait PropertyValue: Sized {
    /// Returns the specified tracked device property.
    ///
    /// # Errors
    /// See `TrackedPropertyError`.
    fn get_property(device: TrackedDevice, property: Property) -> Result<Self, TrackedDevicePropertyError>;
}

impl PropertyValue for bool {
    fn get_property(device: TrackedDevice, property: Property) -> Result<Self, TrackedDevicePropertyError> {
        unsafe {
            let mut error: TrackedDevicePropertyError = mem::uninitialized();
            let r = device.system().0.GetBoolTrackedDeviceProperty.unwrap()(device.index(), property.into(), &mut error.0);
            if error == TrackedDevicePropertyError::SUCCESS {
                Ok(r)
            } else {
                Err(error)
            }
        }
    }
}
impl PropertyValue for f32 {
    fn get_property(device: TrackedDevice, property: Property) -> Result<Self, TrackedDevicePropertyError> {
        unsafe {
            let mut error: TrackedDevicePropertyError = mem::uninitialized();
            let r = device.system().0.GetFloatTrackedDeviceProperty.unwrap()(device.index(), property.into(), &mut error.0);
            if error == TrackedDevicePropertyError::SUCCESS {
                Ok(r)
            } else {
                Err(error)
            }
        }
    }
}
impl PropertyValue for i32 {
    fn get_property(device: TrackedDevice, property: Property) -> TrackedDevicePropertyResult<Self> {
        unsafe {
            let mut error: TrackedDevicePropertyError = mem::uninitialized();
            let r = device.system().0.GetInt32TrackedDeviceProperty.unwrap()(device.index(), property.into(), &mut error.0);
            if error == TrackedDevicePropertyError::SUCCESS {
                Ok(r)
            } else {
                Err(error)
            }
        }
    }
}
impl PropertyValue for u64 {
    fn get_property(device: TrackedDevice, property: Property) -> TrackedDevicePropertyResult<Self> {
        unsafe {
            let mut error: TrackedDevicePropertyError = mem::uninitialized();
            let r = device.system().0.GetUint64TrackedDeviceProperty.unwrap()(device.index(), property.into(), &mut error.0);
            if error == TrackedDevicePropertyError::SUCCESS {
                Ok(r)
            } else {
                Err(error)
            }
        }
    }
}
impl PropertyValue for Matrix34 {
    fn get_property(device: TrackedDevice, property: Property) -> TrackedDevicePropertyResult<Self> {
        unsafe {
            let mut error: TrackedDevicePropertyError = mem::uninitialized();
            let r =
                device.system().0.GetMatrix34TrackedDeviceProperty.unwrap()(device.index(), property.into(), &mut error.0);
            if error == TrackedDevicePropertyError::SUCCESS {
                Ok(r.m)
            } else {
                Err(error)
            }
        }
    }
}
impl PropertyValue for std::ffi::CString {
    fn get_property(device: TrackedDevice, property: Property) -> TrackedDevicePropertyResult<Self> {
        unsafe {
            let mut error = mem::uninitialized();
            let res = get_string(|ptr, n| {
                device.system().0.GetStringTrackedDeviceProperty.unwrap()(device.index(), property.into(), ptr, n, &mut error)
            });
            res.map_or(Err(TrackedDevicePropertyError(error)), Ok)
        }
    }
}

/// The result of fetching a property of a tracked device.
pub type TrackedDevicePropertyResult<T> = Result<T, TrackedDevicePropertyError>;
/// The error that occurs whenever retrieving a tracked device property fails.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrackedDevicePropertyError(sys::TrackedPropertyError);
impl TrackedDevicePropertyError {
    pub const SUCCESS: Self =
        TrackedDevicePropertyError(sys::ETrackedPropertyError_TrackedProp_Success);
    pub const WRONG_DATA_TYPE: Self =
        TrackedDevicePropertyError(sys::ETrackedPropertyError_TrackedProp_WrongDataType);
    pub const WRONG_DEVICE_CLASS: Self =
        TrackedDevicePropertyError(sys::ETrackedPropertyError_TrackedProp_WrongDeviceClass);
    pub const BUFFER_TOO_SMALL: Self =
        TrackedDevicePropertyError(sys::ETrackedPropertyError_TrackedProp_BufferTooSmall);
    pub const UNKNOWN_PROPERTY: Self =
        TrackedDevicePropertyError(sys::ETrackedPropertyError_TrackedProp_UnknownProperty);
    pub const INVALID_DEVICE: Self =
        TrackedDevicePropertyError(sys::ETrackedPropertyError_TrackedProp_InvalidDevice);
    pub const COULD_NOT_CONTACT_SERVER: Self =
        TrackedDevicePropertyError(sys::ETrackedPropertyError_TrackedProp_CouldNotContactServer);
    pub const VALUE_NOT_PROVIDED_BY_DEVICE: Self =
        TrackedDevicePropertyError(sys::ETrackedPropertyError_TrackedProp_ValueNotProvidedByDevice);
    pub const STRING_EXCEEDS_MAXIMUM_LENGTH: Self =
        TrackedDevicePropertyError(sys::ETrackedPropertyError_TrackedProp_StringExceedsMaximumLength);
    pub const NOT_YET_AVAILABLE: Self =
        TrackedDevicePropertyError(sys::ETrackedPropertyError_TrackedProp_NotYetAvailable);
    pub const PERMISSION_DENIED: Self =
        TrackedDevicePropertyError(sys::ETrackedPropertyError_TrackedProp_PermissionDenied);
    pub const INVALID_OPERATION: Self =
        TrackedDevicePropertyError(sys::ETrackedPropertyError_TrackedProp_InvalidOperation);
}

impl fmt::Debug for TrackedDevicePropertyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(std::error::Error::description(self))
    }
}
impl fmt::Display for TrackedDevicePropertyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(std::error::Error::description(self))
    }
}
impl std::error::Error for TrackedDevicePropertyError {
    fn description(&self) -> &str {
        match *self {
            Self::SUCCESS => "SUCCESS",
            Self::WRONG_DATA_TYPE => "WRONG_DATA_TYPE",
            Self::WRONG_DEVICE_CLASS => "WRONG_DEVICE_CLASS",
            Self::BUFFER_TOO_SMALL => "BUFFER_TOO_SMALL",
            Self::UNKNOWN_PROPERTY => "UNKNOWN_PROPERTY",
            Self::INVALID_DEVICE => "INVALID_DEVICE",
            Self::COULD_NOT_CONTACT_SERVER => "COULD_NOT_CONTACT_SERVER",
            Self::VALUE_NOT_PROVIDED_BY_DEVICE => "VALUE_NOT_PROVIDED_BY_DEVICE",
            Self::STRING_EXCEEDS_MAXIMUM_LENGTH => "STRING_EXCEEDS_MAXIMUM_LENGTH",
            Self::NOT_YET_AVAILABLE => "NOT_YET_AVAILABLE",
            Self::PERMISSION_DENIED => "PERMISSION_DENIED",
            Self::INVALID_OPERATION => "INVALID_OPERATION",
            _ => "UNKNOWN",
        }
    }
}