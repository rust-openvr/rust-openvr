#![allow(non_upper_case_globals)]

use super::TrackedDeviceProperty;
use openvr_sys as sys;

pub const Invalid: TrackedDeviceProperty = sys::ETrackedDeviceProperty_Prop_Invalid;
pub const TrackingSystemName_String: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_TrackingSystemName_String;
pub const ModelNumber_String: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_ModelNumber_String;
pub const SerialNumber_String: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_SerialNumber_String;
pub const RenderModelName_String: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_RenderModelName_String;
pub const WillDriftInYaw_Bool: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_WillDriftInYaw_Bool;
pub const ManufacturerName_String: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_ManufacturerName_String;
pub const TrackingFirmwareVersion_String: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_TrackingFirmwareVersion_String;
pub const HardwareRevision_String: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_HardwareRevision_String;
pub const AllWirelessDongleDescriptions_String: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_AllWirelessDongleDescriptions_String;
pub const ConnectedWirelessDongle_String: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_ConnectedWirelessDongle_String;
pub const DeviceIsWireless_Bool: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_DeviceIsWireless_Bool;
pub const DeviceIsCharging_Bool: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_DeviceIsCharging_Bool;
pub const DeviceBatteryPercentage_Float: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_DeviceBatteryPercentage_Float;
pub const StatusDisplayTransform_Matrix34: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_StatusDisplayTransform_Matrix34;
pub const Firmware_UpdateAvailable_Bool: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_Firmware_UpdateAvailable_Bool;
pub const Firmware_ManualUpdate_Bool: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_Firmware_ManualUpdate_Bool;
pub const Firmware_ManualUpdateURL_String: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_Firmware_ManualUpdateURL_String;
pub const HardwareRevision_Uint64: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_HardwareRevision_Uint64;
pub const FirmwareVersion_Uint64: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_FirmwareVersion_Uint64;
pub const FPGAVersion_Uint64: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_FPGAVersion_Uint64;
pub const VRCVersion_Uint64: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_VRCVersion_Uint64;
pub const RadioVersion_Uint64: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_RadioVersion_Uint64;
pub const DongleVersion_Uint64: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_DongleVersion_Uint64;
pub const BlockServerShutdown_Bool: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_BlockServerShutdown_Bool;
pub const CanUnifyCoordinateSystemWithHmd_Bool: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_CanUnifyCoordinateSystemWithHmd_Bool;
pub const ContainsProximitySensor_Bool: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_ContainsProximitySensor_Bool;
pub const DeviceProvidesBatteryStatus_Bool: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_DeviceProvidesBatteryStatus_Bool;
pub const DeviceCanPowerOff_Bool: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_DeviceCanPowerOff_Bool;
pub const Firmware_ProgrammingTarget_String: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_Firmware_ProgrammingTarget_String;
pub const DeviceClass_Int32: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_DeviceClass_Int32;
pub const HasCamera_Bool: TrackedDeviceProperty = sys::ETrackedDeviceProperty_Prop_HasCamera_Bool;
pub const DriverVersion_String: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_DriverVersion_String;
pub const Firmware_ForceUpdateRequired_Bool: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_Firmware_ForceUpdateRequired_Bool;
pub const ViveSystemButtonFixRequired_Bool: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_ViveSystemButtonFixRequired_Bool;
pub const ParentDriver_Uint64: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_ParentDriver_Uint64;
pub const ResourceRoot_String: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_ResourceRoot_String;
pub const ReportsTimeSinceVSync_Bool: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_ReportsTimeSinceVSync_Bool;
pub const SecondsFromVsyncToPhotons_Float: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_SecondsFromVsyncToPhotons_Float;
pub const DisplayFrequency_Float: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_DisplayFrequency_Float;
pub const UserIpdMeters_Float: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_UserIpdMeters_Float;
pub const CurrentUniverseId_Uint64: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_CurrentUniverseId_Uint64;
pub const PreviousUniverseId_Uint64: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_PreviousUniverseId_Uint64;
pub const DisplayFirmwareVersion_Uint64: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_DisplayFirmwareVersion_Uint64;
pub const IsOnDesktop_Bool: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_IsOnDesktop_Bool;
pub const DisplayMCType_Int32: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_DisplayMCType_Int32;
pub const DisplayMCOffset_Float: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_DisplayMCOffset_Float;
pub const DisplayMCScale_Float: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_DisplayMCScale_Float;
pub const EdidVendorID_Int32: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_EdidVendorID_Int32;
pub const DisplayMCImageLeft_String: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_DisplayMCImageLeft_String;
pub const DisplayMCImageRight_String: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_DisplayMCImageRight_String;
pub const DisplayGCBlackClamp_Float: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_DisplayGCBlackClamp_Float;
pub const EdidProductID_Int32: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_EdidProductID_Int32;
pub const CameraToHeadTransform_Matrix34: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_CameraToHeadTransform_Matrix34;
pub const DisplayGCType_Int32: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_DisplayGCType_Int32;
pub const DisplayGCOffset_Float: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_DisplayGCOffset_Float;
pub const DisplayGCScale_Float: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_DisplayGCScale_Float;
pub const DisplayGCPrescale_Float: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_DisplayGCPrescale_Float;
pub const DisplayGCImage_String: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_DisplayGCImage_String;
pub const LensCenterLeftU_Float: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_LensCenterLeftU_Float;
pub const LensCenterLeftV_Float: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_LensCenterLeftV_Float;
pub const LensCenterRightU_Float: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_LensCenterRightU_Float;
pub const LensCenterRightV_Float: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_LensCenterRightV_Float;
pub const UserHeadToEyeDepthMeters_Float: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_UserHeadToEyeDepthMeters_Float;
pub const CameraFirmwareVersion_Uint64: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_CameraFirmwareVersion_Uint64;
pub const CameraFirmwareDescription_String: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_CameraFirmwareDescription_String;
pub const DisplayFPGAVersion_Uint64: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_DisplayFPGAVersion_Uint64;
pub const DisplayBootloaderVersion_Uint64: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_DisplayBootloaderVersion_Uint64;
pub const DisplayHardwareVersion_Uint64: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_DisplayHardwareVersion_Uint64;
pub const AudioFirmwareVersion_Uint64: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_AudioFirmwareVersion_Uint64;
pub const CameraCompatibilityMode_Int32: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_CameraCompatibilityMode_Int32;
pub const ScreenshotHorizontalFieldOfViewDegrees_Float: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_ScreenshotHorizontalFieldOfViewDegrees_Float;
pub const ScreenshotVerticalFieldOfViewDegrees_Float: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_ScreenshotVerticalFieldOfViewDegrees_Float;
pub const DisplaySuppressed_Bool: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_DisplaySuppressed_Bool;
pub const DisplayAllowNightMode_Bool: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_DisplayAllowNightMode_Bool;
pub const DisplayMCImageWidth_Int32: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_DisplayMCImageWidth_Int32;
pub const DisplayMCImageHeight_Int32: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_DisplayMCImageHeight_Int32;
pub const DisplayMCImageNumChannels_Int32: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_DisplayMCImageNumChannels_Int32;
pub const DisplayMCImageData_Binary: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_DisplayMCImageData_Binary;
pub const SecondsFromPhotonsToVblank_Float: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_SecondsFromPhotonsToVblank_Float;
pub const DriverDirectModeSendsVsyncEvents_Bool: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_DriverDirectModeSendsVsyncEvents_Bool;
pub const DisplayDebugMode_Bool: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_DisplayDebugMode_Bool;
pub const GraphicsAdapterLuid_Uint64: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_GraphicsAdapterLuid_Uint64;
pub const AttachedDeviceId_String: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_AttachedDeviceId_String;
pub const SupportedButtons_Uint64: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_SupportedButtons_Uint64;
pub const Axis0Type_Int32: TrackedDeviceProperty = sys::ETrackedDeviceProperty_Prop_Axis0Type_Int32;
pub const Axis1Type_Int32: TrackedDeviceProperty = sys::ETrackedDeviceProperty_Prop_Axis1Type_Int32;
pub const Axis2Type_Int32: TrackedDeviceProperty = sys::ETrackedDeviceProperty_Prop_Axis2Type_Int32;
pub const Axis3Type_Int32: TrackedDeviceProperty = sys::ETrackedDeviceProperty_Prop_Axis3Type_Int32;
pub const Axis4Type_Int32: TrackedDeviceProperty = sys::ETrackedDeviceProperty_Prop_Axis4Type_Int32;
pub const ControllerRoleHint_Int32: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_ControllerRoleHint_Int32;
pub const FieldOfViewLeftDegrees_Float: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_FieldOfViewLeftDegrees_Float;
pub const FieldOfViewRightDegrees_Float: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_FieldOfViewRightDegrees_Float;
pub const FieldOfViewTopDegrees_Float: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_FieldOfViewTopDegrees_Float;
pub const FieldOfViewBottomDegrees_Float: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_FieldOfViewBottomDegrees_Float;
pub const TrackingRangeMinimumMeters_Float: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_TrackingRangeMinimumMeters_Float;
pub const TrackingRangeMaximumMeters_Float: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_TrackingRangeMaximumMeters_Float;
pub const ModeLabel_String: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_ModeLabel_String;
pub const IconPathName_String: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_IconPathName_String;
pub const NamedIconPathDeviceOff_String: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_NamedIconPathDeviceOff_String;
pub const NamedIconPathDeviceSearching_String: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_NamedIconPathDeviceSearching_String;
pub const NamedIconPathDeviceSearchingAlert_String: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_NamedIconPathDeviceSearchingAlert_String;
pub const NamedIconPathDeviceReady_String: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_NamedIconPathDeviceReady_String;
pub const NamedIconPathDeviceReadyAlert_String: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_NamedIconPathDeviceReadyAlert_String;
pub const NamedIconPathDeviceNotReady_String: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_NamedIconPathDeviceNotReady_String;
pub const NamedIconPathDeviceStandby_String: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_NamedIconPathDeviceStandby_String;
pub const NamedIconPathDeviceAlertLow_String: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_NamedIconPathDeviceAlertLow_String;
pub const DisplayHiddenArea_Binary_Start: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_DisplayHiddenArea_Binary_Start;
pub const DisplayHiddenArea_Binary_End: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_DisplayHiddenArea_Binary_End;
pub const UserConfigPath_String: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_UserConfigPath_String;
pub const InstallPath_String: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_InstallPath_String;
pub const HasDisplayComponent_Bool: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_HasDisplayComponent_Bool;
pub const HasControllerComponent_Bool: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_HasControllerComponent_Bool;
pub const HasCameraComponent_Bool: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_HasCameraComponent_Bool;
pub const HasDriverDirectModeComponent_Bool: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_HasDriverDirectModeComponent_Bool;
pub const HasVirtualDisplayComponent_Bool: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_HasVirtualDisplayComponent_Bool;
pub const VendorSpecific_Reserved_Start: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_VendorSpecific_Reserved_Start;
pub const VendorSpecific_Reserved_End: TrackedDeviceProperty =
    sys::ETrackedDeviceProperty_Prop_VendorSpecific_Reserved_End;
