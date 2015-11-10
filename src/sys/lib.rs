
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

#[link(name = "openvr_api")]
extern {}

extern "C" {
	pub fn VR_Init(err: *mut HmdError) -> *const ();
	pub fn VR_Shutdown();
	pub fn VR_IsHmdPresent() -> bool;
	pub fn VR_GetStringForHmdError(err: HmdError) -> *const u8;
	pub fn VR_GetGenericInterface(name: *const u8, err: *mut HmdError) -> *const ();
}

pub type TrackedDeviceIndex_t = u32;
pub type VREvent_Data_t = [u8; 16];
pub type VRControllerState_t = VRControllerState001_t;
pub type VROverlayHandle_t = u64;
pub type VRNotificationId = u32;
#[repr(C)]
#[derive(Debug)]
pub enum Hmd_Eye {
	Left = 0,
	Right = 1,
}

#[repr(C)]
#[derive(Debug)]
pub enum GraphicsAPIConvention {
	DirectX = 0,
	OpenGL = 1,
}

#[repr(C)]
#[derive(Debug)]
pub enum HmdTrackingResult {
	Uninitialized = 1,
	Calibrating_InProgress = 100,
	Calibrating_OutOfRange = 101,
	Running_OK = 200,
	Running_OutOfRange = 201,
}

#[repr(C)]
#[derive(Debug)]
pub enum TrackedDeviceClass {
	Invalid = 0,
	HMD = 1,
	Controller = 2,
	TrackingReference = 4,
	Other = 1000,
}

#[repr(C)]
#[derive(Debug)]
pub enum TrackingUniverseOrigin {
	TrackingUniverseSeated = 0,
	TrackingUniverseStanding = 1,
	TrackingUniverseRawAndUncalibrated = 2,
}

#[repr(C)]
#[derive(Debug)]
pub enum TrackedDeviceProperty {
	TrackingSystemName_String = 1000,
	ModelNumber_String = 1001,
	SerialNumber_String = 1002,
	RenderModelName_String = 1003,
	WillDriftInYaw_Bool = 1004,
	ManufacturerName_String = 1005,
	TrackingFirmwareVersion_String = 1006,
	HardwareRevision_String = 1007,
	AllWirelessDongleDescriptions_String = 1008,
	ConnectedWirelessDongle_String = 1009,
	DeviceIsWireless_Bool = 1010,
	DeviceIsCharging_Bool = 1011,
	DeviceBatteryPercentage_Float = 1012,
	StatusDisplayTransform_Matrix34 = 1013,
	Firmware_UpdateAvailable_Bool = 1014,
	Firmware_ManualUpdate_Bool = 1015,
	Firmware_ManualUpdateURL_String = 1016,
	HardwareRevision_Uint64 = 1017,
	FirmwareVersion_Uint64 = 1018,
	FPGAVersion_Uint64 = 1019,
	VRCVersion_Uint64 = 1020,
	RadioVersion_Uint64 = 1021,
	DongleVersion_Uint64 = 1022,
	BlockServerShutdown_Bool = 1023,
	CanUnifyCoordinateSystemWithHmd_Bool = 1024,
	ReportsTimeSinceVSync_Bool = 2000,
	SecondsFromVsyncToPhotons_Float = 2001,
	DisplayFrequency_Float = 2002,
	UserIpdMeters_Float = 2003,
	CurrentUniverseId_Uint64 = 2004,
	PreviousUniverseId_Uint64 = 2005,
	DisplayFirmwareVersion_String = 2006,
	IsOnDesktop_Bool = 2007,
	DisplayMCType_Int32 = 2008,
	DisplayMCOffset_Float = 2009,
	DisplayMCScale_Float = 2010,
	VendorID_Int32 = 2011,
	DisplayMCImageLeft_String = 2012,
	DisplayMCImageRight_String = 2013,
	DisplayGCBlackClamp_Float = 2014,
	AttachedDeviceId_String = 3000,
	SupportedButtons_Uint64 = 3001,
	Axis0Type_Int32 = 3002,
	Axis1Type_Int32 = 3003,
	Axis2Type_Int32 = 3004,
	Axis3Type_Int32 = 3005,
	Axis4Type_Int32 = 3006,
	FieldOfViewLeftDegrees_Float = 4000,
	FieldOfViewRightDegrees_Float = 4001,
	FieldOfViewTopDegrees_Float = 4002,
	FieldOfViewBottomDegrees_Float = 4003,
	TrackingRangeMinimumMeters_Float = 4004,
	TrackingRangeMaximumMeters_Float = 4005,
	TrackedCamera_IntrinsicsFX_Float = 5000,
	TrackedCamera_IntrinsicsFY_Float = 5001,
	TrackedCamera_IntrinsicsCX_Float = 5002,
	TrackedCamera_IntrinsicsCY_Float = 5003,
	TrackedCamera_IntrinsicsK1_Float = 5004,
	TrackedCamera_IntrinsicsK2_Float = 5005,
	TrackedCamera_IntrinsicsP1_Float = 5006,
	TrackedCamera_IntrinsicsP2_Float = 5007,
	TrackedCamera_IntrinsicsK3_Float = 5008,
}

#[repr(C)]
#[derive(Debug)]
pub enum TrackedPropertyError {
	Success = 0,
	WrongDataType = 1,
	WrongDeviceClass = 2,
	BufferTooSmall = 3,
	UnknownProperty = 4,
	InvalidDevice = 5,
	CouldNotContactServer = 6,
	ValueNotProvidedByDevice = 7,
	StringExceedsMaximumLength = 8,
}

#[repr(C)]
#[derive(Debug)]
pub enum VRSubmitFlags_t {
	Default = 0,
	LensDistortionAlreadyApplied = 1,
	GlRenderBuffer = 2,
}

#[repr(C)]
#[derive(Debug)]
pub enum VRState_t {
	Undefined = -1,
	Off = 0,
	Searching = 1,
	Searching_Alert = 2,
	Ready = 3,
	Ready_Alert = 4,
	NotReady = 5,
}

#[repr(C)]
#[derive(Debug)]
pub enum EVREventType {
	None = 0,
	TrackedDeviceActivated = 100,
	TrackedDeviceDeactivated = 101,
	TrackedDeviceUpdated = 102,
	TrackedDeviceUserInteractionStarted = 103,
	TrackedDeviceUserInteractionEnded = 104,
	ButtonPress = 200,
	ButtonUnpress = 201,
	ButtonTouch = 202,
	ButtonUntouch = 203,
	MouseMove = 300,
	MouseButtonDown = 301,
	MouseButtonUp = 302,
	FocusEnter = 303,
	FocusLeave = 304,
	InputFocusCaptured = 400,
	InputFocusReleased = 401,
	SceneFocusLost = 402,
	SceneFocusGained = 403,
	SceneApplicationChanged = 404,
	SceneFocusChanged = 405,
	OverlayShown = 500,
	OverlayHidden = 501,
	DashboardActivated = 502,
	DashboardDeactivated = 503,
	DashboardThumbSelected = 504,
	DashboardRequested = 505,
	ResetDashboard = 506,
	RenderToast = 507,
	ImageLoaded = 508,
	ShowKeyboard = 509,
	HideKeyboard = 510,
	OverlayGamepadFocusGained = 511,
	OverlayGamepadFocusLost = 512,
	Notification_Show = 600,
	Notification_Dismissed = 601,
	Notification_BeginInteraction = 602,
	Quit = 700,
	ProcessQuit = 701,
	ChaperoneDataHasChanged = 800,
	ChaperoneUniverseHasChanged = 801,
	ChaperoneTempDataHasChanged = 802,
	StatusUpdate = 900,
	MCImageUpdated = 1000,
	FirmwareUpdateStarted = 1100,
	FirmwareUpdateFinished = 1101,
	KeyboardClosed = 1200,
	KeyboardCharInput = 1201,
	ApplicationTransitionStarted = 1300,
	ApplicationTransitionAborted = 1301,
	ApplicationTransitionNewAppStarted = 1302,
}

#[repr(C)]
#[derive(Debug)]
pub enum EDeviceActivityLevel {
	EDeviceActivityLevel_Unknown = -1,
	EDeviceActivityLevel_Idle = 0,
	EDeviceActivityLevel_UserInteraction = 1,
	EDeviceActivityLevel_UserInteraction_Timeout = 2,
}

#[repr(C)]
#[derive(Debug)]
pub enum EVRButtonId {
	EButton_System = 0,
	EButton_ApplicationMenu = 1,
	EButton_Grip = 2,
	EButton_DPad_Left = 3,
	EButton_DPad_Up = 4,
	EButton_DPad_Right = 5,
	EButton_DPad_Down = 6,
	EButton_A = 7,
	EButton_Axis0 = 32,
	EButton_Axis1 = 33,
	EButton_Axis2 = 34,
	EButton_Axis3 = 35,
	EButton_Axis4 = 36,
	EButton_Max = 64,
}

#[repr(C)]
#[derive(Debug)]
pub enum EVRMouseButton {
	Left = 1,
	Right = 2,
	Middle = 4,
}

#[repr(C)]
#[derive(Debug)]
pub enum EVRControllerAxisType {
	eControllerAxis_None = 0,
	eControllerAxis_TrackPad = 1,
	eControllerAxis_Joystick = 2,
	eControllerAxis_Trigger = 3,
}

#[repr(C)]
#[derive(Debug)]
pub enum EVRControllerEventOutputType {
	OSEvents = 0,
	VREvents = 1,
}

#[repr(C)]
#[derive(Debug)]
pub enum VROverlayError {
	None = 0,
	UnknownOverlay = 10,
	InvalidHandle = 11,
	PermissionDenied = 12,
	OverlayLimitExceeded = 13,
	WrongVisibilityType = 14,
	KeyTooLong = 15,
	NameTooLong = 16,
	KeyInUse = 17,
	WrongTransformType = 18,
	InvalidTrackedDevice = 19,
	InvalidParameter = 20,
	ThumbnailCantBeDestroyed = 21,
	ArrayTooSmall = 22,
	RequestFailed = 23,
	InvalidTexture = 24,
	UnableToLoadFile = 25,
	KeyboardAlreadyInUse = 26,
	NoNeighbor = 27,
}

#[repr(C)]
#[derive(Debug)]
pub enum EVRApplicationType {
	Other = 0,
	Scene = 1,
	Overlay = 2,
}

#[repr(C)]
#[derive(Debug)]
pub enum VRFirmwareError {
	None = 0,
	Success = 1,
	Fail = 2,
}

#[repr(C)]
#[derive(Debug)]
pub enum HmdError {
	None = 0,
	Unknown = 1,
	Init_InstallationNotFound = 100,
	Init_InstallationCorrupt = 101,
	Init_VRClientDLLNotFound = 102,
	Init_FileNotFound = 103,
	Init_FactoryNotFound = 104,
	Init_InterfaceNotFound = 105,
	Init_InvalidInterface = 106,
	Init_UserConfigDirectoryInvalid = 107,
	Init_HmdNotFound = 108,
	Init_NotInitialized = 109,
	Init_PathRegistryNotFound = 110,
	Init_NoConfigPath = 111,
	Init_NoLogPath = 112,
	Init_PathRegistryNotWritable = 113,
	Init_AppInfoInitFailed = 114,
	Init_Retry = 115,
	Init_InitCanceledByUser = 116,
	Init_AnotherAppLaunching = 117,
	Init_SettingsInitFailed = 118,
	Init_ShuttingDown = 119,
	Init_TooManyObjects = 120,
	Driver_Failed = 200,
	Driver_Unknown = 201,
	Driver_HmdUnknown = 202,
	Driver_NotLoaded = 203,
	Driver_RuntimeOutOfDate = 204,
	Driver_HmdInUse = 205,
	Driver_NotCalibrated = 206,
	Driver_CalibrationInvalid = 207,
	Driver_HmdDisplayNotFound = 208,
	IPC_ServerInitFailed = 300,
	IPC_ConnectFailed = 301,
	IPC_SharedStateInitFailed = 302,
	IPC_CompositorInitFailed = 303,
	IPC_MutexInitFailed = 304,
	IPC_Failed = 305,
	VendorSpecific_UnableToConnectToOculusRuntime = 1000,
	VendorSpecific_HmdFound_But = 1100,
	VendorSpecific_HmdFound_CantOpenDevice = 1101,
	VendorSpecific_HmdFound_UnableToRequestConfigStart = 1102,
	VendorSpecific_HmdFound_NoStoredConfig = 1103,
	VendorSpecific_HmdFound_ConfigTooBig = 1104,
	VendorSpecific_HmdFound_ConfigTooSmall = 1105,
	VendorSpecific_HmdFound_UnableToInitZLib = 1106,
	VendorSpecific_HmdFound_CantReadFirmwareVersion = 1107,
	VendorSpecific_HmdFound_UnableToSendUserDataStart = 1108,
	VendorSpecific_HmdFound_UnableToGetUserDataStart = 1109,
	VendorSpecific_HmdFound_UnableToGetUserDataNext = 1110,
	VendorSpecific_HmdFound_UserDataAddressRange = 1111,
	VendorSpecific_HmdFound_UserDataError = 1112,
	Steam_SteamInstallationNotFound = 2000,
}

#[repr(C)]
#[derive(Debug)]
pub enum EVRApplicationError {
	None = 0,
	AppKeyAlreadyExists = 100,
	NoManifest = 101,
	NoApplication = 102,
	InvalidIndex = 103,
	UnknownApplication = 104,
	IPCFailed = 105,
	ApplicationAlreadyRunning = 106,
	InvalidManifest = 107,
	InvalidApplication = 108,
	LaunchFailed = 109,
	ApplicationAlreadyStarting = 110,
	LaunchInProgress = 111,
	OldApplicationQuitting = 112,
	TransitionAborted = 113,
	BufferTooSmall = 200,
	PropertyNotSet = 201,
	UnknownProperty = 202,
}

#[repr(C)]
#[derive(Debug)]
pub enum EVRApplicationProperty {
	Name_String = 0,
	LaunchType_String = 11,
	WorkingDirectory_String = 12,
	BinaryPath_String = 13,
	Arguments_String = 14,
	URL_String = 15,
	Description_String = 50,
	NewsURL_String = 51,
	ImagePath_String = 52,
	Source_String = 53,
	IsDashboardOverlay_Bool = 60,
}

#[repr(C)]
#[derive(Debug)]
pub enum EVRApplicationTransitionState {
	None = 0,
	OldAppQuitSent = 10,
	WaitingForExternalLaunch = 11,
	NewAppLaunched = 20,
}

#[repr(C)]
#[derive(Debug)]
pub enum ChaperoneCalibrationState {
	OK = 1,
	Warning = 100,
	Warning_BaseStationMayHaveMoved = 101,
	Warning_BaseStationRemoved = 102,
	Warning_SeatedBoundsInvalid = 103,
	Error = 200,
	Error_BaseStationUninitalized = 201,
	Error_BaseStationConflict = 202,
	Error_PlayAreaInvalid = 203,
	Error_CollisionBoundsInvalid = 204,
}

#[repr(C)]
#[derive(Debug)]
pub enum VRCompositorError {
	None = 0,
	IncompatibleVersion = 100,
	DoNotHaveFocus = 101,
	InvalidTexture = 102,
	IsNotSceneApplication = 103,
	TextureIsOnWrongDevice = 104,
}

#[repr(C)]
#[derive(Debug)]
pub enum VROverlayInputMethod {
	None = 0,
	Mouse = 1,
}

#[repr(C)]
#[derive(Debug)]
pub enum VROverlayTransformType {
	Absolute = 0,
	TrackedDeviceRelative = 1,
	SystemOverlay = 2,
}

#[repr(C)]
#[derive(Debug)]
pub enum VROverlayFlags {
	None = 0,
	Curved = 1,
	RGSS4X = 2,
	NoDashboardTab = 3,
	AcceptsGamepadEvents = 4,
}

#[repr(C)]
#[derive(Debug)]
pub enum EGamepadTextInputMode {
	EGamepadTextInputModeNormal = 0,
	EGamepadTextInputModePassword = 1,
}

#[repr(C)]
#[derive(Debug)]
pub enum EGamepadTextInputLineMode {
	EGamepadTextInputLineModeSingleLine = 0,
	EGamepadTextInputLineModeMultipleLines = 1,
}

#[repr(C)]
#[derive(Debug)]
pub enum EOverlayDirection {
	Up = 0,
	Down = 1,
	Left = 2,
	Right = 3,
	Count = 4,
}

#[repr(C)]
#[derive(Debug)]
pub enum NotificationError_t {
	ENotificationError_OK = 0,
	ENotificationError_Fail = 1,
	eNotificationError_InvalidParam = 2,
}

#[repr(C)]
#[derive(Debug)]
pub enum EVRSettingsError {
	None = 0,
	IPCFailed = 1,
	WriteFailed = 2,
	ReadFailed = 3,
}

#[repr(C)]
#[derive(Debug)]
pub enum ECameraVideoStreamFormat {
	FORMAT_UNKNOWN = 0,
	FORMAT_RAW10 = 1,
	FORMAT_NV12 = 2,
	FORMAT_RGB24 = 3,
	MAX_FORMATS = 4,
}

#[repr(C)]
#[derive(Debug)]
pub enum EChaperoneConfigFile {
	Live = 1,
	Temp = 2,
}

#[repr(C)]
pub struct HmdMatrix34_t {
	//float [3][4]
	pub m: [[f32; 4]; 3],
}
#[repr(C)]
pub struct HmdMatrix44_t {
	//float [4][4]
	pub m: [[f32; 4]; 4],
}
#[repr(C)]
pub struct HmdVector3_t {
	//float [3]
	pub v: [f32; 3],
}
#[repr(C)]
pub struct HmdVector4_t {
	//float [4]
	pub v: [f32; 4],
}
#[repr(C)]
pub struct HmdVector3d_t {
	//double [3]
	pub v: [f64; 3],
}
#[repr(C)]
pub struct HmdVector2_t {
	//float [2]
	pub v: [f32; 2],
}
#[repr(C)]
pub struct HmdQuaternion_t {
	//double
	pub w: f64,
	//double
	pub x: f64,
	//double
	pub y: f64,
	//double
	pub z: f64,
}
#[repr(C)]
pub struct HmdColor_t {
	//float
	pub r: f32,
	//float
	pub g: f32,
	//float
	pub b: f32,
	//float
	pub a: f32,
}
#[repr(C)]
pub struct HmdQuad_t {
	//struct vr::HmdVector3_t [4]
	pub vCorners: [HmdVector3_t ; 4],
}
#[repr(C)]
pub struct DistortionCoordinates_t {
	//float [2]
	pub rfRed: [f32; 2],
	//float [2]
	pub rfGreen: [f32; 2],
	//float [2]
	pub rfBlue: [f32; 2],
}
#[repr(C)]
pub struct TrackedDevicePose_t {
	//struct vr::HmdMatrix34_t
	pub mDeviceToAbsoluteTracking: HmdMatrix34_t,
	//struct vr::HmdVector3_t
	pub vVelocity: HmdVector3_t,
	//struct vr::HmdVector3_t
	pub vAngularVelocity: HmdVector3_t,
	//enum vr::HmdTrackingResult
	pub eTrackingResult: HmdTrackingResult,
	//_Bool
	pub bPoseIsValid: bool,
	//_Bool
	pub bDeviceIsConnected: bool,
}
#[repr(C)]
pub struct RenderModel_Vertex_t {
	//struct vr::HmdVector3_t
	pub vPosition: HmdVector3_t,
	//struct vr::HmdVector3_t
	pub vNormal: HmdVector3_t,
	//float [2]
	pub rfTextureCoord: [f32; 2],
}
#[repr(C)]
pub struct RenderModel_TextureMap_t {
	//uint16_t
	pub unWidth: u16,
	//uint16_t
	pub unHeight: u16,
	//const uint8_t *
	pub rubTextureMapData: *mut u8,
}
#[repr(C)]
pub struct RenderModel_t {
	//uint64_t
	pub ulInternalHandle: u64,
	//const struct vr::RenderModel_Vertex_t *
	pub rVertexData: *mut RenderModel_Vertex_t,
	//uint32_t
	pub unVertexCount: u32,
	//const uint16_t *
	pub rIndexData: *mut u16,
	//uint32_t
	pub unTriangleCount: u32,
	//struct vr::RenderModel_TextureMap_t
	pub diffuseTexture: RenderModel_TextureMap_t,
}
#[repr(C)]
pub struct VRTextureBounds_t {
	//float
	pub uMin: f32,
	//float
	pub vMin: f32,
	//float
	pub uMax: f32,
	//float
	pub vMax: f32,
}
#[repr(C)]
pub struct VREvent_Controller_t {
	//enum vr::EVRButtonId
	pub button: EVRButtonId,
}
#[repr(C)]
pub struct VREvent_Mouse_t {
	//float
	pub x: f32,
	//float
	pub y: f32,
	//enum vr::EVRMouseButton
	pub button: EVRMouseButton,
}
#[repr(C)]
pub struct VREvent_Notification_t {
	//uint64_t
	pub ulUserValue: u64,
	//uint32_t
	pub notificationId: u32,
}
#[repr(C)]
pub struct VREvent_Process_t {
	//uint32_t
	pub pid: u32,
	//uint32_t
	pub oldPid: u32,
}
#[repr(C)]
pub struct VREvent_Overlay_t {
	//uint64_t
	pub overlayHandle: u64,
}
#[repr(C)]
pub struct VREvent_Status_t {
	//enum vr::VRState_t
	pub statusState: VRState_t,
}
#[repr(C)]
pub struct VREvent_Keyboard_t {
	//char [8]
	pub cNewInput: [char ; 8],
	//uint64_t
	pub uUserValue: u64,
}
#[repr(C)]
pub struct VREvent_Reserved_t {
	//uint64_t
	pub reserved0: u64,
	//uint64_t
	pub reserved1: u64,
}
#[repr(C)]
pub struct VREvent_t {
	//enum vr::EVREventType
	pub eventType: EVREventType,
	//TrackedDeviceIndex_t
	pub trackedDeviceIndex: TrackedDeviceIndex_t,
	//VREvent_Data_t
	pub data: VREvent_Data_t,
	//float
	pub eventAgeSeconds: f32,
}
#[repr(C)]
pub struct HiddenAreaMesh_t {
	//const struct vr::HmdVector2_t *
	pub pVertexData: *mut HmdVector2_t,
	//uint32_t
	pub unTriangleCount: u32,
}
#[repr(C)]
pub struct VRControllerAxis_t {
	//float
	pub x: f32,
	//float
	pub y: f32,
}
#[repr(C)]
pub struct VRControllerState001_t {
	//uint32_t
	pub unPacketNum: u32,
	//uint64_t
	pub ulButtonPressed: u64,
	//uint64_t
	pub ulButtonTouched: u64,
	//struct vr::VRControllerAxis_t [5]
	pub rAxis: [VRControllerAxis_t ; 5],
}
#[repr(C)]
pub struct Compositor_OverlaySettings {
	//uint32_t
	pub size: u32,
	//_Bool
	pub curved: bool,
	//_Bool
	pub antialias: bool,
	//float
	pub scale: f32,
	//float
	pub distance: f32,
	//float
	pub alpha: f32,
	//float
	pub uOffset: f32,
	//float
	pub vOffset: f32,
	//float
	pub uScale: f32,
	//float
	pub vScale: f32,
	//float
	pub gridDivs: f32,
	//float
	pub gridWidth: f32,
	//float
	pub gridScale: f32,
	//struct vr::HmdMatrix44_t
	pub transform: HmdMatrix44_t,
}
#[repr(C)]
pub struct ChaperoneSoftBoundsInfo_t {
	//struct vr::HmdQuad_t
	pub quadCorners: HmdQuad_t,
}
#[repr(C)]
pub struct ChaperoneSeatedBoundsInfo_t {
	//struct vr::HmdVector3_t
	pub vSeatedHeadPosition: HmdVector3_t,
	//struct vr::HmdVector3_t [2]
	pub vDeskEdgePositions: [HmdVector3_t ; 2],
}
#[repr(C)]
pub struct Compositor_FrameTiming {
	//uint32_t
	pub size: u32,
	//double
	pub frameStart: f64,
	//float
	pub frameVSync: f32,
	//uint32_t
	pub droppedFrames: u32,
	//uint32_t
	pub frameIndex: u32,
	//vr::TrackedDevicePose_t
	pub pose: TrackedDevicePose_t,
	//float
	pub prediction: f32,
	//float
	pub m_flFrameIntervalMs: f32,
	//float
	pub m_flSceneRenderCpuMs: f32,
	//float
	pub m_flSceneRenderGpuMs: f32,
	//float
	pub m_flCompositorRenderCpuMs: f32,
	//float
	pub m_flCompositorRenderGpuMs: f32,
	//float
	pub m_flPresentCallCpuMs: f32,
	//float
	pub m_flRunningStartMs: f32,
	//float
	pub m_flHandoffStartMs: f32,
	//float
	pub m_flHandoffEndMs: f32,
}
#[repr(C)]
pub struct VROverlayIntersectionParams_t {
	//struct vr::HmdVector3_t
	pub vSource: HmdVector3_t,
	//struct vr::HmdVector3_t
	pub vDirection: HmdVector3_t,
	//enum vr::TrackingUniverseOrigin
	pub eOrigin: TrackingUniverseOrigin,
}
#[repr(C)]
pub struct VROverlayIntersectionResults_t {
	//struct vr::HmdVector3_t
	pub vPoint: HmdVector3_t,
	//struct vr::HmdVector3_t
	pub vNormal: HmdVector3_t,
	//struct vr::HmdVector2_t
	pub vUVs: HmdVector2_t,
	//float
	pub fDistance: f32,
}
#[repr(C)]
pub struct ComponentState_t {
	//struct vr::HmdMatrix34_t
	pub mTrackingToComponentRenderModel: HmdMatrix34_t,
	//struct vr::HmdMatrix34_t
	pub mTrackingToComponentLocal: HmdMatrix34_t,
	//_Bool
	pub bIsStatic: bool,
	//_Bool
	pub bIsVisible: bool,
}
#[repr(C)]
pub struct NotificationBitmap {
	//void *
	pub bytes: *mut (),
	//int32_t
	pub width: i32,
	//int32_t
	pub height: i32,
	//int32_t
	pub depth: i32,
}
#[repr(C)]
pub struct NotificationItem {
	//VRNotificationId
	pub notificationId: VRNotificationId,
}
#[repr(C)]
pub struct CameraVideoStreamFrame_t {
	//enum vr::ECameraVideoStreamFormat
	pub m_nStreamFormat: ECameraVideoStreamFormat,
	//uint32_t
	pub m_nWidth: u32,
	//uint32_t
	pub m_nHeight: u32,
	//uint32_t
	pub m_nFrameSequence: u32,
	//uint32_t
	pub m_nTimeStamp: u32,
	//uint32_t
	pub m_nBufferIndex: u32,
	//uint32_t
	pub m_nBufferCount: u32,
	//uint32_t
	pub m_nImageDataSize: u32,
	//double
	pub m_flFrameTime: f64,
	//_Bool
	pub m_bPoseValid: bool,
	//float [16]
	pub m_HMDPoseMatrix: [f32; 16],
	//void *
	pub m_pImageData: *mut (),
}
#[repr(C)]
pub struct TrackedCameraCalibrationDevOnly_t {
	//double
	pub m_flIntrinsicsFX: f64,
	//double
	pub m_flIntrinsicsFY: f64,
	//double
	pub m_flIntrinsicsCX: f64,
	//double
	pub m_flIntrinsicsCY: f64,
	//double
	pub m_flIntrinsicsK1: f64,
	//double
	pub m_flIntrinsicsK2: f64,
	//double
	pub m_flIntrinsicsP1: f64,
	//double
	pub m_flIntrinsicsP2: f64,
	//double
	pub m_flIntrinsicsK3: f64,
}
extern "C" {
	pub fn VR_IVRSystem_GetWindowBounds(ptr: *const (), pnX: *mut i32, pnY: *mut i32, pnWidth: *mut u32, pnHeight: *mut u32) ;
	pub fn VR_IVRSystem_GetRecommendedRenderTargetSize(ptr: *const (), pnWidth: *mut u32, pnHeight: *mut u32) ;
	pub fn VR_IVRSystem_GetEyeOutputViewport(ptr: *const (), eEye: Hmd_Eye, pnX: *mut u32, pnY: *mut u32, pnWidth: *mut u32, pnHeight: *mut u32) ;
	pub fn VR_IVRSystem_GetProjectionMatrix(ptr: *const (), eEye: Hmd_Eye, fNearZ: f32, fFarZ: f32, eProjType: GraphicsAPIConvention) -> HmdMatrix44_t;
	pub fn VR_IVRSystem_GetProjectionRaw(ptr: *const (), eEye: Hmd_Eye, pfLeft: *mut f32, pfRight: *mut f32, pfTop: *mut f32, pfBottom: *mut f32) ;
	pub fn VR_IVRSystem_ComputeDistortion(ptr: *const (), eEye: Hmd_Eye, fU: f32, fV: f32) -> DistortionCoordinates_t;
	pub fn VR_IVRSystem_GetEyeToHeadTransform(ptr: *const (), eEye: Hmd_Eye) -> HmdMatrix34_t;
	pub fn VR_IVRSystem_GetTimeSinceLastVsync(ptr: *const (), pfSecondsSinceLastVsync: *mut f32, pulFrameCounter: *mut u64) -> bool;
	pub fn VR_IVRSystem_GetD3D9AdapterIndex(ptr: *const (), ) -> i32;
	pub fn VR_IVRSystem_GetDXGIOutputInfo(ptr: *const (), pnAdapterIndex: *mut i32, pnAdapterOutputIndex: *mut i32) ;
	pub fn VR_IVRSystem_AttachToWindow(ptr: *const (), hWnd: *mut ()) -> bool;
	pub fn VR_IVRSystem_GetDeviceToAbsoluteTrackingPose(ptr: *const (), eOrigin: TrackingUniverseOrigin, fPredictedSecondsToPhotonsFromNow: f32, pTrackedDevicePoseArray: *mut TrackedDevicePose_t, unTrackedDevicePoseArrayCount: u32) ;
	pub fn VR_IVRSystem_ResetSeatedZeroPose(ptr: *const (), ) ;
	pub fn VR_IVRSystem_GetSeatedZeroPoseToStandingAbsoluteTrackingPose(ptr: *const (), ) -> HmdMatrix34_t;
	pub fn VR_IVRSystem_GetRawZeroPoseToStandingAbsoluteTrackingPose(ptr: *const (), ) -> HmdMatrix34_t;
	pub fn VR_IVRSystem_GetSortedTrackedDeviceIndicesOfClass(ptr: *const (), eTrackedDeviceClass: TrackedDeviceClass, punTrackedDeviceIndexArray: *mut TrackedDeviceIndex_t, unTrackedDeviceIndexArrayCount: u32, unRelativeToTrackedDeviceIndex: TrackedDeviceIndex_t) -> u32;
	pub fn VR_IVRSystem_GetTrackedDeviceActivityLevel(ptr: *const (), unDeviceId: TrackedDeviceIndex_t) -> EDeviceActivityLevel;
	pub fn VR_IVRSystem_ApplyTransform(ptr: *const (), pOutputPose: *mut TrackedDevicePose_t, trackedDevicePose: *const TrackedDevicePose_t, transform: *const HmdMatrix34_t) ;
	pub fn VR_IVRSystem_GetTrackedDeviceClass(ptr: *const (), unDeviceIndex: TrackedDeviceIndex_t) -> TrackedDeviceClass;
	pub fn VR_IVRSystem_IsTrackedDeviceConnected(ptr: *const (), unDeviceIndex: TrackedDeviceIndex_t) -> bool;
	pub fn VR_IVRSystem_GetBoolTrackedDeviceProperty(ptr: *const (), unDeviceIndex: TrackedDeviceIndex_t, prop: TrackedDeviceProperty, pError: *mut TrackedPropertyError) -> bool;
	pub fn VR_IVRSystem_GetFloatTrackedDeviceProperty(ptr: *const (), unDeviceIndex: TrackedDeviceIndex_t, prop: TrackedDeviceProperty, pError: *mut TrackedPropertyError) -> f32;
	pub fn VR_IVRSystem_GetInt32TrackedDeviceProperty(ptr: *const (), unDeviceIndex: TrackedDeviceIndex_t, prop: TrackedDeviceProperty, pError: *mut TrackedPropertyError) -> i32;
	pub fn VR_IVRSystem_GetUint64TrackedDeviceProperty(ptr: *const (), unDeviceIndex: TrackedDeviceIndex_t, prop: TrackedDeviceProperty, pError: *mut TrackedPropertyError) -> u64;
	pub fn VR_IVRSystem_GetMatrix34TrackedDeviceProperty(ptr: *const (), unDeviceIndex: TrackedDeviceIndex_t, prop: TrackedDeviceProperty, pError: *mut TrackedPropertyError) -> HmdMatrix34_t;
	pub fn VR_IVRSystem_GetStringTrackedDeviceProperty(ptr: *const (), unDeviceIndex: TrackedDeviceIndex_t, prop: TrackedDeviceProperty, pchValue: *const u8, unBufferSize: u32, pError: *mut TrackedPropertyError) -> u32;
	pub fn VR_IVRSystem_GetPropErrorNameFromEnum(ptr: *const (), error: TrackedPropertyError) -> *const u8;
	pub fn VR_IVRSystem_PollNextEvent(ptr: *const (), pEvent: *mut VREvent_t) -> bool;
	pub fn VR_IVRSystem_PollNextEventWithPose(ptr: *const (), eOrigin: TrackingUniverseOrigin, pEvent: *mut VREvent_t, pTrackedDevicePose: *mut TrackedDevicePose_t) -> bool;
	pub fn VR_IVRSystem_GetEventTypeNameFromEnum(ptr: *const (), eType: EVREventType) -> *const u8;
	pub fn VR_IVRSystem_GetHiddenAreaMesh(ptr: *const (), eEye: Hmd_Eye) -> HiddenAreaMesh_t;
	pub fn VR_IVRSystem_GetControllerState(ptr: *const (), unControllerDeviceIndex: TrackedDeviceIndex_t, pControllerState: *mut VRControllerState_t) -> bool;
	pub fn VR_IVRSystem_GetControllerStateWithPose(ptr: *const (), eOrigin: TrackingUniverseOrigin, unControllerDeviceIndex: TrackedDeviceIndex_t, pControllerState: *mut VRControllerState_t, pTrackedDevicePose: *mut TrackedDevicePose_t) -> bool;
	pub fn VR_IVRSystem_TriggerHapticPulse(ptr: *const (), unControllerDeviceIndex: TrackedDeviceIndex_t, unAxisId: u32, usDurationMicroSec: u16) ;
	pub fn VR_IVRSystem_GetButtonIdNameFromEnum(ptr: *const (), eButtonId: EVRButtonId) -> *const u8;
	pub fn VR_IVRSystem_GetControllerAxisTypeNameFromEnum(ptr: *const (), eAxisType: EVRControllerAxisType) -> *const u8;
	pub fn VR_IVRSystem_CaptureInputFocus(ptr: *const (), ) -> bool;
	pub fn VR_IVRSystem_ReleaseInputFocus(ptr: *const (), ) ;
	pub fn VR_IVRSystem_IsInputFocusCapturedByAnotherProcess(ptr: *const (), ) -> bool;
	pub fn VR_IVRSystem_DriverDebugRequest(ptr: *const (), unDeviceIndex: TrackedDeviceIndex_t, pchRequest: *const u8, pchResponseBuffer: *const u8, unResponseBufferSize: u32) -> u32;
	pub fn VR_IVRSystem_PerformFirmwareUpdate(ptr: *const (), unDeviceIndex: TrackedDeviceIndex_t) -> VRFirmwareError;
	pub fn VR_IVRSystem_IsDisplayOnDesktop(ptr: *const (), ) -> bool;
	pub fn VR_IVRSystem_SetDisplayVisibility(ptr: *const (), bIsVisibleOnDesktop: bool) -> bool;
	pub fn VR_IVRApplications_AddApplicationManifest(ptr: *const (), pchApplicationManifestFullPath: *const u8, bTemporary: bool) -> EVRApplicationError;
	pub fn VR_IVRApplications_RemoveApplicationManifest(ptr: *const (), pchApplicationManifestFullPath: *const u8) -> EVRApplicationError;
	pub fn VR_IVRApplications_IsApplicationInstalled(ptr: *const (), pchAppKey: *const u8) -> bool;
	pub fn VR_IVRApplications_GetApplicationCount(ptr: *const (), ) -> u32;
	pub fn VR_IVRApplications_GetApplicationKeyByIndex(ptr: *const (), unApplicationIndex: u32, pchAppKeyBuffer: *const u8, unAppKeyBufferLen: u32) -> EVRApplicationError;
	pub fn VR_IVRApplications_GetApplicationKeyByProcessId(ptr: *const (), unProcessId: u32, pchAppKeyBuffer: *const u8, unAppKeyBufferLen: u32) -> EVRApplicationError;
	pub fn VR_IVRApplications_LaunchApplication(ptr: *const (), pchAppKey: *const u8) -> EVRApplicationError;
	pub fn VR_IVRApplications_LaunchDashboardOverlay(ptr: *const (), pchAppKey: *const u8) -> EVRApplicationError;
	pub fn VR_IVRApplications_IdentifyApplication(ptr: *const (), unProcessId: u32, pchAppKey: *const u8) -> EVRApplicationError;
	pub fn VR_IVRApplications_GetApplicationProcessId(ptr: *const (), pchAppKey: *const u8) -> u32;
	pub fn VR_IVRApplications_GetApplicationsErrorNameFromEnum(ptr: *const (), error: EVRApplicationError) -> *const u8;
	pub fn VR_IVRApplications_GetApplicationPropertyString(ptr: *const (), pchAppKey: *const u8, eProperty: EVRApplicationProperty, pchPropertyValueBuffer: *const u8, unPropertyValueBufferLen: u32, peError: *mut EVRApplicationError) -> u32;
	pub fn VR_IVRApplications_GetApplicationPropertyBool(ptr: *const (), pchAppKey: *const u8, eProperty: EVRApplicationProperty, peError: *mut EVRApplicationError) -> bool;
	pub fn VR_IVRApplications_GetHomeApplication(ptr: *const (), pchAppKeyBuffer: *const u8, unAppKeyBufferLen: u32) -> EVRApplicationError;
	pub fn VR_IVRApplications_SetHomeApplication(ptr: *const (), pchAppKey: *const u8) -> EVRApplicationError;
	pub fn VR_IVRApplications_SetApplicationAutoLaunch(ptr: *const (), pchAppKey: *const u8, bAutoLaunch: bool) -> EVRApplicationError;
	pub fn VR_IVRApplications_GetApplicationAutoLaunch(ptr: *const (), pchAppKey: *const u8) -> bool;
	pub fn VR_IVRApplications_GetStartingApplication(ptr: *const (), pchAppKeyBuffer: *const u8, unAppKeyBufferLen: u32) -> EVRApplicationError;
	pub fn VR_IVRApplications_GetTransitionState(ptr: *const (), ) -> EVRApplicationTransitionState;
	pub fn VR_IVRApplications_PerformApplicationPrelaunchCheck(ptr: *const (), pchAppKey: *const u8) -> EVRApplicationError;
	pub fn VR_IVRApplications_GetApplicationsTransitionStateNameFromEnum(ptr: *const (), state: EVRApplicationTransitionState) -> *const u8;
	pub fn VR_IVRChaperone_GetCalibrationState(ptr: *const (), ) -> ChaperoneCalibrationState;
	pub fn VR_IVRChaperone_GetPlayAreaSize(ptr: *const (), pSizeX: *mut f32, pSizeZ: *mut f32) -> bool;
	pub fn VR_IVRChaperone_GetPlayAreaRect(ptr: *const (), rect: *mut HmdQuad_t) -> bool;
	pub fn VR_IVRChaperone_ReloadInfo(ptr: *const (), ) ;
	pub fn VR_IVRChaperone_SetSceneColor(ptr: *const (), color: HmdColor_t) ;
	pub fn VR_IVRChaperone_GetBoundsColor(ptr: *const (), pOutputColorArray: *mut HmdColor_t, nNumOutputColors: i32) ;
	pub fn VR_IVRChaperone_AreBoundsVisible(ptr: *const (), ) -> bool;
	pub fn VR_IVRChaperone_ForceBoundsVisible(ptr: *const (), bForce: bool) ;
	pub fn VR_IVRCompositor_GetLastError(ptr: *const (), pchBuffer: *const u8, unBufferSize: u32) -> u32;
	pub fn VR_IVRCompositor_SetVSync(ptr: *const (), bVSync: bool) ;
	pub fn VR_IVRCompositor_GetVSync(ptr: *const (), ) -> bool;
	pub fn VR_IVRCompositor_SetGamma(ptr: *const (), fGamma: f32) ;
	pub fn VR_IVRCompositor_GetGamma(ptr: *const (), ) -> f32;
	pub fn VR_IVRCompositor_WaitGetPoses(ptr: *const (), pRenderPoseArray: *mut TrackedDevicePose_t, unRenderPoseArrayCount: u32, pGamePoseArray: *mut TrackedDevicePose_t, unGamePoseArrayCount: u32) -> VRCompositorError;
	pub fn VR_IVRCompositor_Submit(ptr: *const (), eEye: Hmd_Eye, eTextureType: GraphicsAPIConvention, pTexture: *mut (), pBounds: *mut VRTextureBounds_t, nSubmitFlags: VRSubmitFlags_t) -> VRCompositorError;
	pub fn VR_IVRCompositor_ClearLastSubmittedFrame(ptr: *const (), ) ;
	pub fn VR_IVRCompositor_GetFrameTiming(ptr: *const (), pTiming: *mut Compositor_FrameTiming, unFramesAgo: u32) -> bool;
	pub fn VR_IVRCompositor_FadeToColor(ptr: *const (), fSeconds: f32, fRed: f32, fGreen: f32, fBlue: f32, fAlpha: f32, bBackground: bool) ;
	pub fn VR_IVRCompositor_FadeGrid(ptr: *const (), fSeconds: f32, bFadeIn: bool) ;
	pub fn VR_IVRCompositor_SetSkyboxOverride(ptr: *const (), eTextureType: GraphicsAPIConvention, pFront: *mut (), pBack: *mut (), pLeft: *mut (), pRight: *mut (), pTop: *mut (), pBottom: *mut ()) ;
	pub fn VR_IVRCompositor_ClearSkyboxOverride(ptr: *const (), ) ;
	pub fn VR_IVRCompositor_CompositorBringToFront(ptr: *const (), ) ;
	pub fn VR_IVRCompositor_CompositorGoToBack(ptr: *const (), ) ;
	pub fn VR_IVRCompositor_CompositorQuit(ptr: *const (), ) ;
	pub fn VR_IVRCompositor_IsFullscreen(ptr: *const (), ) -> bool;
	pub fn VR_IVRCompositor_SetTrackingSpace(ptr: *const (), eOrigin: TrackingUniverseOrigin) ;
	pub fn VR_IVRCompositor_GetTrackingSpace(ptr: *const (), ) -> TrackingUniverseOrigin;
	pub fn VR_IVRCompositor_GetCurrentSceneFocusProcess(ptr: *const (), ) -> u32;
	pub fn VR_IVRCompositor_CanRenderScene(ptr: *const (), ) -> bool;
	pub fn VR_IVRCompositor_ShowMirrorWindow(ptr: *const (), ) ;
	pub fn VR_IVRCompositor_HideMirrorWindow(ptr: *const (), ) ;
	pub fn VR_IVRCompositor_CompositorDumpImages(ptr: *const (), ) ;
	pub fn VR_IVRCompositor_GetFrameTimeRemaining(ptr: *const (), ) -> f32;
	pub fn VR_IVRCompositor_GetLastFrameRenderer(ptr: *const (), ) -> u32;
	pub fn VR_IVRCompositor_GetLastPoses(ptr: *const (), pRenderPoseArray: *mut TrackedDevicePose_t, unRenderPoseArrayCount: u32, pGamePoseArray: *mut TrackedDevicePose_t, unGamePoseArrayCount: u32) -> VRCompositorError;
	pub fn VR_IVRCompositor_PostPresentHandoff(ptr: *const (), ) ;
	pub fn VR_IVROverlay_FindOverlay(ptr: *const (), pchOverlayKey: *const u8, pOverlayHandle: *mut VROverlayHandle_t) -> VROverlayError;
	pub fn VR_IVROverlay_CreateOverlay(ptr: *const (), pchOverlayKey: *const u8, pchOverlayFriendlyName: *const u8, pOverlayHandle: *mut VROverlayHandle_t) -> VROverlayError;
	pub fn VR_IVROverlay_DestroyOverlay(ptr: *const (), ulOverlayHandle: VROverlayHandle_t) -> VROverlayError;
	pub fn VR_IVROverlay_SetHighQualityOverlay(ptr: *const (), ulOverlayHandle: VROverlayHandle_t) -> VROverlayError;
	pub fn VR_IVROverlay_GetHighQualityOverlay(ptr: *const (), ) -> VROverlayHandle_t;
	pub fn VR_IVROverlay_GetOverlayKey(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, pchValue: *const u8, unBufferSize: u32, pError: *mut VROverlayError) -> u32;
	pub fn VR_IVROverlay_GetOverlayName(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, pchValue: *const u8, unBufferSize: u32, pError: *mut VROverlayError) -> u32;
	pub fn VR_IVROverlay_GetOverlayImageData(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, pvBuffer: *mut (), unBufferSize: u32, punWidth: *mut u32, punHeight: *mut u32) -> VROverlayError;
	pub fn VR_IVROverlay_GetOverlayErrorNameFromEnum(ptr: *const (), error: VROverlayError) -> *const u8;
	pub fn VR_IVROverlay_SetOverlayFlag(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, eOverlayFlag: VROverlayFlags, bEnabled: bool) -> VROverlayError;
	pub fn VR_IVROverlay_GetOverlayFlag(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, eOverlayFlag: VROverlayFlags, pbEnabled: *mut bool) -> VROverlayError;
	pub fn VR_IVROverlay_SetOverlayColor(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, fRed: f32, fGreen: f32, fBlue: f32) -> VROverlayError;
	pub fn VR_IVROverlay_GetOverlayColor(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, pfRed: *mut f32, pfGreen: *mut f32, pfBlue: *mut f32) -> VROverlayError;
	pub fn VR_IVROverlay_SetOverlayAlpha(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, fAlpha: f32) -> VROverlayError;
	pub fn VR_IVROverlay_GetOverlayAlpha(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, pfAlpha: *mut f32) -> VROverlayError;
	pub fn VR_IVROverlay_SetOverlayGamma(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, fGamma: f32) -> VROverlayError;
	pub fn VR_IVROverlay_GetOverlayGamma(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, pfGamma: *mut f32) -> VROverlayError;
	pub fn VR_IVROverlay_SetOverlayWidthInMeters(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, fWidthInMeters: f32) -> VROverlayError;
	pub fn VR_IVROverlay_GetOverlayWidthInMeters(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, pfWidthInMeters: *mut f32) -> VROverlayError;
	pub fn VR_IVROverlay_SetOverlayAutoCurveDistanceRangeInMeters(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, fMinDistanceInMeters: f32, fMaxDistanceInMeters: f32) -> VROverlayError;
	pub fn VR_IVROverlay_GetOverlayAutoCurveDistanceRangeInMeters(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, pfMinDistanceInMeters: *mut f32, pfMaxDistanceInMeters: *mut f32) -> VROverlayError;
	pub fn VR_IVROverlay_SetOverlayTextureBounds(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, pOverlayTextureBounds: *mut VRTextureBounds_t) -> VROverlayError;
	pub fn VR_IVROverlay_GetOverlayTextureBounds(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, pOverlayTextureBounds: *mut VRTextureBounds_t) -> VROverlayError;
	pub fn VR_IVROverlay_GetOverlayTransformType(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, peTransformType: *mut VROverlayTransformType) -> VROverlayError;
	pub fn VR_IVROverlay_SetOverlayTransformAbsolute(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, eTrackingOrigin: TrackingUniverseOrigin, pmatTrackingOriginToOverlayTransform: *mut HmdMatrix34_t) -> VROverlayError;
	pub fn VR_IVROverlay_GetOverlayTransformAbsolute(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, peTrackingOrigin: *mut TrackingUniverseOrigin, pmatTrackingOriginToOverlayTransform: *mut HmdMatrix34_t) -> VROverlayError;
	pub fn VR_IVROverlay_SetOverlayTransformTrackedDeviceRelative(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, unTrackedDevice: TrackedDeviceIndex_t, pmatTrackedDeviceToOverlayTransform: *mut HmdMatrix34_t) -> VROverlayError;
	pub fn VR_IVROverlay_GetOverlayTransformTrackedDeviceRelative(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, punTrackedDevice: *mut TrackedDeviceIndex_t, pmatTrackedDeviceToOverlayTransform: *mut HmdMatrix34_t) -> VROverlayError;
	pub fn VR_IVROverlay_ShowOverlay(ptr: *const (), ulOverlayHandle: VROverlayHandle_t) -> VROverlayError;
	pub fn VR_IVROverlay_HideOverlay(ptr: *const (), ulOverlayHandle: VROverlayHandle_t) -> VROverlayError;
	pub fn VR_IVROverlay_IsOverlayVisible(ptr: *const (), ulOverlayHandle: VROverlayHandle_t) -> bool;
	pub fn VR_IVROverlay_PollNextOverlayEvent(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, pEvent: *mut VREvent_t) -> bool;
	pub fn VR_IVROverlay_GetOverlayInputMethod(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, peInputMethod: *mut VROverlayInputMethod) -> VROverlayError;
	pub fn VR_IVROverlay_SetOverlayInputMethod(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, eInputMethod: VROverlayInputMethod) -> VROverlayError;
	pub fn VR_IVROverlay_GetOverlayMouseScale(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, pvecMouseScale: *mut HmdVector2_t) -> VROverlayError;
	pub fn VR_IVROverlay_SetOverlayMouseScale(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, pvecMouseScale: *mut HmdVector2_t) -> VROverlayError;
	pub fn VR_IVROverlay_ComputeOverlayIntersection(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, pParams: *mut VROverlayIntersectionParams_t, pResults: *mut VROverlayIntersectionResults_t) -> bool;
	pub fn VR_IVROverlay_HandleControllerOverlayInteractionAsMouse(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, unControllerDeviceIndex: TrackedDeviceIndex_t) -> bool;
	pub fn VR_IVROverlay_IsHoverTargetOverlay(ptr: *const (), ulOverlayHandle: VROverlayHandle_t) -> bool;
	pub fn VR_IVROverlay_GetGamepadFocusOverlay(ptr: *const (), ) -> VROverlayHandle_t;
	pub fn VR_IVROverlay_SetGamepadFocusOverlay(ptr: *const (), ulNewFocusOverlay: VROverlayHandle_t) -> VROverlayError;
	pub fn VR_IVROverlay_SetOverlayNeighbor(ptr: *const (), eDirection: EOverlayDirection, ulFrom: VROverlayHandle_t, ulTo: VROverlayHandle_t) -> VROverlayError;
	pub fn VR_IVROverlay_MoveGamepadFocusToNeighbor(ptr: *const (), eDirection: EOverlayDirection, ulFrom: VROverlayHandle_t) -> VROverlayError;
	pub fn VR_IVROverlay_SetOverlayTexture(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, eTextureType: GraphicsAPIConvention, pTexture: *mut ()) -> VROverlayError;
	pub fn VR_IVROverlay_ClearOverlayTexture(ptr: *const (), ulOverlayHandle: VROverlayHandle_t) -> VROverlayError;
	pub fn VR_IVROverlay_SetOverlayRaw(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, pvBuffer: *mut (), unWidth: u32, unHeight: u32, unDepth: u32) -> VROverlayError;
	pub fn VR_IVROverlay_SetOverlayFromFile(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, pchFilePath: *const u8) -> VROverlayError;
	pub fn VR_IVROverlay_CreateDashboardOverlay(ptr: *const (), pchOverlayKey: *const u8, pchOverlayFriendlyName: *const u8, pMainHandle: *mut VROverlayHandle_t, pThumbnailHandle: *mut VROverlayHandle_t) -> VROverlayError;
	pub fn VR_IVROverlay_IsDashboardVisible(ptr: *const (), ) -> bool;
	pub fn VR_IVROverlay_IsActiveDashboardOverlay(ptr: *const (), ulOverlayHandle: VROverlayHandle_t) -> bool;
	pub fn VR_IVROverlay_SetDashboardOverlaySceneProcess(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, unProcessId: u32) -> VROverlayError;
	pub fn VR_IVROverlay_GetDashboardOverlaySceneProcess(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, punProcessId: *mut u32) -> VROverlayError;
	pub fn VR_IVROverlay_ShowDashboard(ptr: *const (), pchOverlayToShow: *const u8) ;
	pub fn VR_IVROverlay_ShowKeyboard(ptr: *const (), eInputMode: EGamepadTextInputMode, eLineInputMode: EGamepadTextInputLineMode, pchDescription: *const u8, unCharMax: u32, pchExistingText: *const u8, bUseMinimalMode: bool, uUserValue: u64) -> VROverlayError;
	pub fn VR_IVROverlay_ShowKeyboardForOverlay(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, eInputMode: EGamepadTextInputMode, eLineInputMode: EGamepadTextInputLineMode, pchDescription: *const u8, unCharMax: u32, pchExistingText: *const u8, bUseMinimalMode: bool, uUserValue: u64) -> VROverlayError;
	pub fn VR_IVROverlay_GetKeyboardText(ptr: *const (), pchText: *const u8, cchText: u32) -> u32;
	pub fn VR_IVROverlay_HideKeyboard(ptr: *const (), ) ;
	pub fn VR_IVRRenderModels_LoadRenderModel(ptr: *const (), pchRenderModelName: *const u8, pRenderModel: *mut RenderModel_t) -> bool;
	pub fn VR_IVRRenderModels_FreeRenderModel(ptr: *const (), pRenderModel: *mut RenderModel_t) ;
	pub fn VR_IVRRenderModels_GetRenderModelName(ptr: *const (), unRenderModelIndex: u32, pchRenderModelName: *const u8, unRenderModelNameLen: u32) -> u32;
	pub fn VR_IVRRenderModels_GetRenderModelCount(ptr: *const (), ) -> u32;
	pub fn VR_IVRRenderModels_GetComponentCount(ptr: *const (), pchRenderModelName: *const u8) -> u32;
	pub fn VR_IVRRenderModels_GetComponentName(ptr: *const (), pchRenderModelName: *const u8, unComponentIndex: u32, pchComponentName: *const u8, unComponentNameLen: u32) -> u32;
	pub fn VR_IVRRenderModels_GetComponentButtonMask(ptr: *const (), pchRenderModelName: *const u8, pchComponentName: *const u8) -> u64;
	pub fn VR_IVRRenderModels_GetComponentRenderModelName(ptr: *const (), pchRenderModelName: *const u8, pchComponentName: *const u8, pchComponentRenderModelName: *const u8, unComponentRenderModelNameLen: u32) -> u32;
	pub fn VR_IVRRenderModels_GetComponentState(ptr: *const (), pchRenderModelName: *const u8, pchComponentName: *const u8, controllerState: VRControllerState_t, pComponentState: *mut ComponentState_t) -> bool;
	pub fn VR_IVRControlPanel_GetDriverCount(ptr: *const (), ) -> u32;
	pub fn VR_IVRControlPanel_GetDriverId(ptr: *const (), unDriverIndex: u32, pchBuffer: *const u8, unBufferLen: u32) -> u32;
	pub fn VR_IVRControlPanel_GetDriverDisplayCount(ptr: *const (), pchDriverId: *const u8) -> u32;
	pub fn VR_IVRControlPanel_GetDriverDisplayId(ptr: *const (), pchDriverId: *const u8, unDisplayIndex: u32, pchBuffer: *const u8, unBufferLen: u32) -> u32;
	pub fn VR_IVRControlPanel_GetDriverDisplayModelNumber(ptr: *const (), pchDriverId: *const u8, pchDisplayId: *const u8, pchBuffer: *const u8, unBufferLen: u32) -> u32;
	pub fn VR_IVRControlPanel_GetDriverDisplaySerialNumber(ptr: *const (), pchDriverId: *const u8, pchDisplayId: *const u8, pchBuffer: *const u8, unBufferLen: u32) -> u32;
	pub fn VR_IVRControlPanel_LoadSharedResource(ptr: *const (), pchResourceName: *const u8, pchBuffer: *const u8, unBufferLen: u32) -> u32;
	pub fn VR_IVRControlPanel_GetIPD(ptr: *const (), ) -> f32;
	pub fn VR_IVRControlPanel_SetIPD(ptr: *const (), fIPD: f32) ;
	//pub fn VR_IVRControlPanel_GetCurrentCompositorInterface(ptr: *const (), pchInterfaceVersion: *const u8) -> *mut class vr::IVRCompositor;
	pub fn VR_IVRControlPanel_QuitProcess(ptr: *const (), pidProcessToQuit: u32) -> bool;
	pub fn VR_IVRControlPanel_StartVRProcess(ptr: *const (), pchExecutable: *const u8, pchArguments: *const *const u8, unArgumentCount: u32, pchWorkingDirectory: *const u8) -> u32;
	pub fn VR_IVRControlPanel_SetMasterProcessToThis(ptr: *const (), ) ;
	pub fn VR_IVRNotifications_GetErrorString(ptr: *const (), error: NotificationError_t, pchBuffer: *const u8, unBufferSize: u32) -> u32;
	pub fn VR_IVRNotifications_CreateNotification(ptr: *const (), ulOverlayHandle: VROverlayHandle_t, ulUserValue: u64, strType: *const u8, strText: *const u8, strCategory: *const u8, photo: *mut NotificationBitmap, notificationId: *mut VRNotificationId) -> NotificationError_t;
	pub fn VR_IVRNotifications_DismissNotification(ptr: *const (), notificationId: VRNotificationId) -> NotificationError_t;
	pub fn VR_IVRSettings_GetSettingsErrorNameFromEnum(ptr: *const (), eError: EVRSettingsError) -> *const u8;
	pub fn VR_IVRSettings_Sync(ptr: *const (), peError: *mut EVRSettingsError) ;
	pub fn VR_IVRSettings_GetBool(ptr: *const (), pchSection: *const u8, pchSettingsKey: *const u8, bDefaultValue: bool, peError: *mut EVRSettingsError) -> bool;
	pub fn VR_IVRSettings_SetBool(ptr: *const (), pchSection: *const u8, pchSettingsKey: *const u8, bValue: bool, peError: *mut EVRSettingsError) ;
	pub fn VR_IVRSettings_GetInt32(ptr: *const (), pchSection: *const u8, pchSettingsKey: *const u8, nDefaultValue: i32, peError: *mut EVRSettingsError) -> i32;
	pub fn VR_IVRSettings_SetInt32(ptr: *const (), pchSection: *const u8, pchSettingsKey: *const u8, nValue: i32, peError: *mut EVRSettingsError) ;
	pub fn VR_IVRSettings_GetFloat(ptr: *const (), pchSection: *const u8, pchSettingsKey: *const u8, flDefaultValue: f32, peError: *mut EVRSettingsError) -> f32;
	pub fn VR_IVRSettings_SetFloat(ptr: *const (), pchSection: *const u8, pchSettingsKey: *const u8, flValue: f32, peError: *mut EVRSettingsError) ;
	pub fn VR_IVRSettings_GetString(ptr: *const (), pchSection: *const u8, pchSettingsKey: *const u8, pchValue: *const u8, unValueLen: u32, pchDefaultValue: *const u8, peError: *mut EVRSettingsError) ;
	pub fn VR_IVRSettings_SetString(ptr: *const (), pchSection: *const u8, pchSettingsKey: *const u8, pchValue: *const u8, peError: *mut EVRSettingsError) ;
	pub fn VR_IVRTrackedCamera_HasCamera(ptr: *const (), nDeviceIndex: TrackedDeviceIndex_t) -> bool;
	pub fn VR_IVRTrackedCamera_GetCameraFirmwareDescription(ptr: *const (), nDeviceIndex: TrackedDeviceIndex_t, pBuffer: *const u8, nBufferLen: u32) -> bool;
	pub fn VR_IVRTrackedCamera_GetCameraFrameDimensions(ptr: *const (), nDeviceIndex: TrackedDeviceIndex_t, nVideoStreamFormat: ECameraVideoStreamFormat, pWidth: *mut u32, pHeight: *mut u32) -> bool;
	pub fn VR_IVRTrackedCamera_SetCameraVideoStreamFormat(ptr: *const (), nDeviceIndex: TrackedDeviceIndex_t, nVideoStreamFormat: ECameraVideoStreamFormat) -> bool;
	pub fn VR_IVRTrackedCamera_GetCameraVideoStreamFormat(ptr: *const (), nDeviceIndex: TrackedDeviceIndex_t) -> ECameraVideoStreamFormat;
	pub fn VR_IVRTrackedCamera_EnableCameraForStreaming(ptr: *const (), nDeviceIndex: TrackedDeviceIndex_t, bEnable: bool) -> bool;
	pub fn VR_IVRTrackedCamera_StartVideoStream(ptr: *const (), nDeviceIndex: TrackedDeviceIndex_t) -> bool;
	pub fn VR_IVRTrackedCamera_StopVideoStream(ptr: *const (), nDeviceIndex: TrackedDeviceIndex_t) -> bool;
	pub fn VR_IVRTrackedCamera_IsVideoStreamActive(ptr: *const (), nDeviceIndex: TrackedDeviceIndex_t) -> bool;
	pub fn VR_IVRTrackedCamera_GetVideoStreamElapsedTime(ptr: *const (), nDeviceIndex: TrackedDeviceIndex_t) -> f32;
	pub fn VR_IVRTrackedCamera_GetVideoStreamFrame(ptr: *const (), nDeviceIndex: TrackedDeviceIndex_t) -> *mut CameraVideoStreamFrame_t;
	pub fn VR_IVRTrackedCamera_ReleaseVideoStreamFrame(ptr: *const (), nDeviceIndex: TrackedDeviceIndex_t, pFrameImage: *mut CameraVideoStreamFrame_t) -> bool;
	pub fn VR_IVRTrackedCamera_SetAutoExposure(ptr: *const (), nDeviceIndex: TrackedDeviceIndex_t, bEnable: bool) -> bool;
	pub fn VR_IVRTrackedCamera_SupportsPauseResume(ptr: *const (), nDeviceIndex: TrackedDeviceIndex_t) -> bool;
	pub fn VR_IVRTrackedCamera_PauseVideoStream(ptr: *const (), nDeviceIndex: TrackedDeviceIndex_t) -> bool;
	pub fn VR_IVRTrackedCamera_ResumeVideoStream(ptr: *const (), nDeviceIndex: TrackedDeviceIndex_t) -> bool;
	pub fn VR_IVRTrackedCamera_IsVideoStreamPaused(ptr: *const (), nDeviceIndex: TrackedDeviceIndex_t) -> bool;
	pub fn VR_IVRChaperoneSetup_CommitWorkingCopy(ptr: *const (), configFile: EChaperoneConfigFile) -> bool;
	pub fn VR_IVRChaperoneSetup_RevertWorkingCopy(ptr: *const (), ) ;
	pub fn VR_IVRChaperoneSetup_GetWorkingPlayAreaSize(ptr: *const (), pSizeX: *mut f32, pSizeZ: *mut f32) -> bool;
	pub fn VR_IVRChaperoneSetup_GetWorkingPlayAreaRect(ptr: *const (), rect: *mut HmdQuad_t) -> bool;
	pub fn VR_IVRChaperoneSetup_GetWorkingCollisionBoundsInfo(ptr: *const (), pQuadsBuffer: *mut HmdQuad_t, punQuadsCount: *mut u32) -> bool;
	pub fn VR_IVRChaperoneSetup_GetLiveCollisionBoundsInfo(ptr: *const (), pQuadsBuffer: *mut HmdQuad_t, punQuadsCount: *mut u32) -> bool;
	pub fn VR_IVRChaperoneSetup_GetWorkingSeatedZeroPoseToRawTrackingPose(ptr: *const (), pmatSeatedZeroPoseToRawTrackingPose: *mut HmdMatrix34_t) -> bool;
	pub fn VR_IVRChaperoneSetup_GetWorkingStandingZeroPoseToRawTrackingPose(ptr: *const (), pmatStandingZeroPoseToRawTrackingPose: *mut HmdMatrix34_t) -> bool;
	pub fn VR_IVRChaperoneSetup_SetWorkingPlayAreaSize(ptr: *const (), sizeX: f32, sizeZ: f32) ;
	pub fn VR_IVRChaperoneSetup_SetWorkingCollisionBoundsInfo(ptr: *const (), pQuadsBuffer: *mut HmdQuad_t, unQuadsCount: u32) ;
	pub fn VR_IVRChaperoneSetup_SetWorkingSeatedZeroPoseToRawTrackingPose(ptr: *const (), matSeatedZeroPoseToRawTrackingPose: *const HmdMatrix34_t) ;
	pub fn VR_IVRChaperoneSetup_SetWorkingStandingZeroPoseToRawTrackingPose(ptr: *const (), matStandingZeroPoseToRawTrackingPose: *const HmdMatrix34_t) ;
	pub fn VR_IVRChaperoneSetup_ReloadFromDisk(ptr: *const (), configFile: EChaperoneConfigFile) ;
}
