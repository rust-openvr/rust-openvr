
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[link(name = "openvr_api")]
extern {}

pub type TrackedDeviceIndex_t = u32;
pub type VREvent_Data_t = [u8; 16];
pub type VRControllerState_t = VRControllerState001_t;
pub type VROverlayHandle_t = u64;
pub type VRNotificationId = u32;
#[repr(C)]
pub enum Hmd_Eye {
	Left = 0,
	Right = 1,
}

#[repr(C)]
pub enum GraphicsAPIConvention {
	DirectX = 0,
	OpenGL = 1,
}

#[repr(C)]
pub enum HmdTrackingResult {
	Uninitialized = 1,
	Calibrating_InProgress = 100,
	Calibrating_OutOfRange = 101,
	Running_OK = 200,
	Running_OutOfRange = 201,
}

#[repr(C)]
pub enum TrackedDeviceClass {
	Invalid = 0,
	HMD = 1,
	Controller = 2,
	TrackingReference = 4,
	Other = 1000,
}

#[repr(C)]
pub enum TrackingUniverseOrigin {
	TrackingUniverseSeated = 0,
	TrackingUniverseStanding = 1,
	TrackingUniverseRawAndUncalibrated = 2,
}

#[repr(C)]
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
	ReportsTimeSinceVSync_Bool = 2000,
	SecondsFromVsyncToPhotons_Float = 2001,
	DisplayFrequency_Float = 2002,
	UserIpdMeters_Float = 2003,
	CurrentUniverseId_Uint64 = 2004,
	PreviousUniverseId_Uint64 = 2005,
	DisplayFirmwareVersion_String = 2006,
	IsOnDesktop_Bool = 2007,
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
}

#[repr(C)]
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
pub enum EVREventType {
	None = 0,
	TrackedDeviceActivated = 100,
	TrackedDeviceDeactivated = 101,
	TrackedDeviceUpdated = 102,
	ButtonPress = 200,
	ButtonUnpress = 201,
	ButtonTouch = 202,
	ButtonUntouch = 203,
	MouseMove = 300,
	MouseButtonDown = 301,
	MouseButtonUp = 302,
	InputFocusCaptured = 400,
	InputFocusReleased = 401,
	SceneFocusLost = 402,
	SceneFocusGained = 403,
	OverlayShown = 500,
	OverlayHidden = 501,
	SystemOverlayActivated = 502,
	SystemOverlayDeactivated = 503,
	SystemOverlayThumbSelected = 504,
	Notification_Dismissed = 600,
	Notification_BeginInteraction = 601,
	Notification_Scroll = 602,
	Notification_ClickOn = 603,
	Notification_ClickOff = 604,
}

#[repr(C)]
pub enum EVRButtonId {
	EButton_System = 0,
	EButton_ApplicationMenu = 1,
	EButton_Grip = 2,
	EButton_Axis0 = 32,
	EButton_Axis1 = 33,
	EButton_Axis2 = 34,
	EButton_Axis3 = 35,
	EButton_Axis4 = 36,
	EButton_Max = 64,
}

#[repr(C)]
pub enum EVRMouseButton {
	Left = 1,
	Right = 2,
	Middle = 4,
}

#[repr(C)]
pub enum EVRControllerAxisType {
	eControllerAxis_None = 0,
	eControllerAxis_TrackPad = 1,
	eControllerAxis_Joystick = 2,
	eControllerAxis_Trigger = 3,
}

#[repr(C)]
pub enum EVRControllerEventOutputType {
	OSEvents = 0,
	VREvents = 1,
}

#[repr(C)]
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
	Driver_Failed = 200,
	Driver_Unknown = 201,
	Driver_HmdUnknown = 202,
	Driver_NotLoaded = 203,
	Driver_RuntimeOutOfDate = 204,
	Driver_HmdInUse = 205,
	IPC_ServerInitFailed = 300,
	IPC_ConnectFailed = 301,
	IPC_SharedStateInitFailed = 302,
	IPC_CompositorInitFailed = 303,
	IPC_MutexInitFailed = 304,
	VendorSpecific_UnableToConnectToOculusRuntime = 1000,
	Steam_SteamInstallationNotFound = 2000,
}

#[repr(C)]
pub enum CameraImageResult {
	OK = 0,
	Uninitalized = 1,
	NotReady = 2,
	SameFrame = 3,
}

#[repr(C)]
pub enum ChaperoneCalibrationState {
	OK = 1,
	Warning = 100,
	Warning_BaseStationMayHaveMoved = 101,
	Warning_BaseStationRemoved = 102,
	Warning_SeatedBoundsInvalid = 103,
	Error = 200,
	Error_BaseStationUninitalized = 201,
	Error_BaseStationConflict = 202,
	Error_SoftBoundsInvalid = 203,
	Error_HardBoundsInvalid = 204,
}

#[repr(C)]
pub enum Compositor_DeviceType {
	DeviceType_None = 0,
	DeviceType_D3D9 = 1,
	DeviceType_D3D9Ex = 2,
	DeviceType_D3D10 = 3,
	DeviceType_D3D11 = 4,
	DeviceType_OpenGL = 5,
}

#[repr(C)]
pub enum VRCompositorError {
	None = 0,
	IncompatibleVersion = 100,
	DoNotHaveFocus = 101,
}

#[repr(C)]
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
}

#[repr(C)]
pub enum VROverlayInputMethod {
	None = 0,
	Mouse = 1,
}

#[repr(C)]
pub enum VROverlayVisibility {
	Manual = 0,
	SystemOverlay = 1,
}

#[repr(C)]
pub enum VROverlayTransformType {
	Absolute = 0,
	TrackedDeviceRelative = 1,
	SystemOverlay = 2,
}

#[repr(C)]
pub enum VROverlayFlags {
	None = 0,
	Curved = 1,
	RGSS4X = 2,
}

#[repr(C)]
pub struct HmdMatrix34_t {
	m: [[f32; 4]; 3],
}
#[repr(C)]
pub struct HmdMatrix44_t {
	m: [[f32; 4]; 4],
}
#[repr(C)]
pub struct HmdVector3_t {
	v: [f32; 3],
}
#[repr(C)]
pub struct HmdVector3d_t {
	v: [f64; 3],
}
#[repr(C)]
pub struct HmdVector2_t {
	v: [f32; 2],
}
#[repr(C)]
pub struct HmdQuaternion_t {
	w: f64,
	x: f64,
	y: f64,
	z: f64,
}
#[repr(C)]
pub struct HmdQuad_t {
	vCorners: HmdVector3_t,
}
#[repr(C)]
pub struct DistortionCoordinates_t {
	rfRed: [f32; 2],
	rfGreen: [f32; 2],
	rfBlue: [f32; 2],
}
#[repr(C)]
pub struct TrackedDevicePose_t {
	mDeviceToAbsoluteTracking: HmdMatrix34_t,
	vVelocity: HmdVector3_t,
	vAngularVelocity: HmdVector3_t,
	eTrackingResult: HmdTrackingResult,
	bPoseIsValid: bool,
	bDeviceIsConnected: bool,
}
#[repr(C)]
pub struct RenderModel_Vertex_t {
	vPosition: HmdVector3_t,
	vNormal: HmdVector3_t,
	rfTextureCoord: [f32; 2],
}
#[repr(C)]
pub struct RenderModel_TextureMap_t {
	unWidth: u16,
	unHeight: u16,
	rubTextureMapData: *mut u8,
}
#[repr(C)]
pub struct RenderModel_t {
	ulInternalHandle: u64,
	rVertexData: RenderModel_Vertex_t,
	unVertexCount: u32,
	rIndexData: *mut u16,
	unTriangleCount: u32,
	diffuseTexture: RenderModel_TextureMap_t,
}
#[repr(C)]
pub struct VRTextureBounds_t {
	uMin: f32,
	vMin: f32,
	uMax: f32,
	vMax: f32,
}
#[repr(C)]
pub struct VREvent_Controller_t {
	button: EVRButtonId,
}
#[repr(C)]
pub struct VREvent_Mouse_t {
	x: f32,
	y: f32,
	button: EVRMouseButton,
}
#[repr(C)]
pub struct VREvent_Notification_t {
	x: f32,
	y: f32,
	notificationId: u32,
}
#[repr(C)]
pub struct VREvent_Process_t {
	pid: u32,
	oldPid: u32,
}
#[repr(C)]
pub struct VREvent_Overlay_t {
	overlayHandle: u64,
}
#[repr(C)]
pub struct VREvent_Reserved_t {
	reserved0: u64,
	reserved1: u64,
}
#[repr(C)]
pub struct VREvent_t {
	eventType: EVREventType,
	trackedDeviceIndex: TrackedDeviceIndex_t,
	data: VREvent_Data_t,
	eventAgeSeconds: f32,
}
#[repr(C)]
pub struct HiddenAreaMesh_t {
	pVertexData: HmdVector2_t,
	unTriangleCount: u32,
}
#[repr(C)]
pub struct VRControllerAxis_t {
	x: f32,
	y: f32,
}
#[repr(C)]
pub struct VRControllerState001_t {
	unPacketNum: u32,
	ulButtonPressed: u64,
	ulButtonTouched: u64,
	rAxis: VRControllerAxis_t,
}
#[repr(C)]
pub struct Compositor_OverlaySettings {
	size: u32,
	curved: bool,
	antialias: bool,
	scale: f32,
	distance: f32,
	alpha: f32,
	uOffset: f32,
	vOffset: f32,
	uScale: f32,
	vScale: f32,
	gridDivs: f32,
	gridWidth: f32,
	gridScale: f32,
	transform: HmdMatrix44_t,
}
#[repr(C)]
pub struct CameraInfo_t {
	width: u32,
	height: u32,
	depth: u32,
	fx: f32,
	cx: f32,
	fy: f32,
	cy: f32,
}
#[repr(C)]
pub struct CameraImage_t {
	frameID: i32,
	pose: HmdMatrix34_t,
	pBuffer: *const u8,
	unBufferLen: u32,
	result: CameraImageResult,
}
#[repr(C)]
pub struct ChaperoneSoftBoundsInfo_t {
	quadCorners: HmdQuad_t,
}
#[repr(C)]
pub struct ChaperoneSeatedBoundsInfo_t {
	vSeatedHeadPosition: HmdVector3_t,
	vDeskEdgePositions: HmdVector3_t,
}
#[repr(C)]
pub struct Compositor_FrameTiming {
	size: u32,
	frameStart: f64,
	frameVSync: f32,
	droppedFrames: u32,
	frameIndex: u32,
	pose: TrackedDevicePose_t,
}
#[repr(C)]
pub struct VROverlayIntersectionParams_t {
	vSource: HmdVector3_t,
	vDirection: HmdVector3_t,
	eOrigin: TrackingUniverseOrigin,
}
#[repr(C)]
pub struct VROverlayIntersectionResults_t {
	vPoint: HmdVector3_t,
	vNormal: HmdVector3_t,
	vUVs: HmdVector2_t,
	fDistance: f32,
}
#[repr(C)]
pub struct NotificationBitmap {
	bytes: *const u8,
	width: i32,
	height: i32,
	depth: i32,
}
extern "C" {
	pub fn VR_IVRSystem_GetWindowBounds( pnX: *mut i32, pnY: *mut i32, pnWidth: *mut u32, pnHeight: *mut u32) ;
	pub fn VR_IVRSystem_GetRecommendedRenderTargetSize( pnWidth: *mut u32, pnHeight: *mut u32) ;
	pub fn VR_IVRSystem_GetEyeOutputViewport( eEye: Hmd_Eye, pnX: *mut u32, pnY: *mut u32, pnWidth: *mut u32, pnHeight: *mut u32) ;
	pub fn VR_IVRSystem_GetProjectionMatrix( eEye: Hmd_Eye, fNearZ: f32, fFarZ: f32, eProjType: GraphicsAPIConvention) -> HmdMatrix44_t;
	pub fn VR_IVRSystem_GetProjectionRaw( eEye: Hmd_Eye, pfLeft: *mut f32, pfRight: *mut f32, pfTop: *mut f32, pfBottom: *mut f32) ;
	pub fn VR_IVRSystem_ComputeDistortion( eEye: Hmd_Eye, fU: f32, fV: f32) -> DistortionCoordinates_t;
	pub fn VR_IVRSystem_GetEyeToHeadTransform( eEye: Hmd_Eye) -> HmdMatrix34_t;
	pub fn VR_IVRSystem_GetTimeSinceLastVsync( pfSecondsSinceLastVsync: *mut f32, pulFrameCounter: *mut u64) -> bool;
	pub fn VR_IVRSystem_GetD3D9AdapterIndex( ) -> i32;
	pub fn VR_IVRSystem_GetDXGIOutputInfo( pnAdapterIndex: *mut i32, pnAdapterOutputIndex: *mut i32) ;
	pub fn VR_IVRSystem_AttachToWindow( hWnd: *mut ()) -> bool;
	pub fn VR_IVRSystem_GetDeviceToAbsoluteTrackingPose( eOrigin: TrackingUniverseOrigin, fPredictedSecondsToPhotonsFromNow: f32, pTrackedDevicePoseArray: TrackedDevicePose_t, unTrackedDevicePoseArrayCount: u32) ;
	pub fn VR_IVRSystem_ResetSeatedZeroPose( ) ;
	pub fn VR_IVRSystem_GetSeatedZeroPoseToStandingAbsoluteTrackingPose( ) -> HmdMatrix34_t;
	pub fn VR_IVRSystem_GetTrackedDeviceClass( unDeviceIndex: TrackedDeviceIndex_t) -> TrackedDeviceClass;
	pub fn VR_IVRSystem_IsTrackedDeviceConnected( unDeviceIndex: TrackedDeviceIndex_t) -> bool;
	pub fn VR_IVRSystem_GetBoolTrackedDeviceProperty( unDeviceIndex: TrackedDeviceIndex_t, prop: TrackedDeviceProperty, pError: *mut TrackedPropertyError) -> bool;
	pub fn VR_IVRSystem_GetFloatTrackedDeviceProperty( unDeviceIndex: TrackedDeviceIndex_t, prop: TrackedDeviceProperty, pError: *mut TrackedPropertyError) -> f32;
	pub fn VR_IVRSystem_GetInt32TrackedDeviceProperty( unDeviceIndex: TrackedDeviceIndex_t, prop: TrackedDeviceProperty, pError: *mut TrackedPropertyError) -> i32;
	pub fn VR_IVRSystem_GetUint64TrackedDeviceProperty( unDeviceIndex: TrackedDeviceIndex_t, prop: TrackedDeviceProperty, pError: *mut TrackedPropertyError) -> u64;
	pub fn VR_IVRSystem_GetMatrix34TrackedDeviceProperty( unDeviceIndex: TrackedDeviceIndex_t, prop: TrackedDeviceProperty, pError: *mut TrackedPropertyError) -> HmdMatrix34_t;
	pub fn VR_IVRSystem_GetStringTrackedDeviceProperty( unDeviceIndex: TrackedDeviceIndex_t, prop: TrackedDeviceProperty, pchValue: *const u8, unBufferSize: u32, pError: *mut TrackedPropertyError) -> u32;
	pub fn VR_IVRSystem_GetPropErrorNameFromEnum( error: TrackedPropertyError) -> *const u8;
	pub fn VR_IVRSystem_PollNextEvent( pEvent: VREvent_t) -> bool;
	pub fn VR_IVRSystem_PollNextEventWithPose( eOrigin: TrackingUniverseOrigin, pEvent: *mut VREvent_t, pTrackedDevicePose: *mut TrackedDevicePose_t) -> bool;
	pub fn VR_IVRSystem_GetEventTypeNameFromEnum( eType: EVREventType) -> *const u8;
	pub fn VR_IVRSystem_GetHiddenAreaMesh( eEye: Hmd_Eye) -> HiddenAreaMesh_t;
	pub fn VR_IVRSystem_GetControllerState( unControllerDeviceIndex: TrackedDeviceIndex_t, pControllerState: *mut VRControllerState_t) -> bool;
	pub fn VR_IVRSystem_GetControllerStateWithPose( eOrigin: TrackingUniverseOrigin, unControllerDeviceIndex: TrackedDeviceIndex_t, pControllerState: *mut VRControllerState_t, pTrackedDevicePose: TrackedDevicePose_t) -> bool;
	pub fn VR_IVRSystem_TriggerHapticPulse( unControllerDeviceIndex: TrackedDeviceIndex_t, unAxisId: u32, usDurationMicroSec: u16) ;
	pub fn VR_IVRSystem_GetButtonIdNameFromEnum( eButtonId: EVRButtonId) -> *const u8;
	pub fn VR_IVRSystem_GetControllerAxisTypeNameFromEnum( eAxisType: EVRControllerAxisType) -> *const u8;
	pub fn VR_IVRSystem_CaptureInputFocus( ) -> bool;
	pub fn VR_IVRSystem_ReleaseInputFocus( ) ;
	pub fn VR_IVRSystem_IsInputFocusCapturedByAnotherProcess( ) -> bool;
	pub fn VR_IVRSystem_DriverDebugRequest( unDeviceIndex: TrackedDeviceIndex_t, pchRequest: *const u8, pchResponseBuffer: *const u8, unResponseBufferSize: u32) -> u32;
	pub fn VR_IVRCameraAccess_GetCameraCount( ) -> u32;
	pub fn VR_IVRCameraAccess_GetCameraId( unCameraIndex: u32, pchBuffer: *const u8, unBufferLen: u32) -> u32;
	pub fn VR_IVRCameraAccess_EnableCamera( unCameraIndex: u32, bEnabled: bool) -> bool;
	pub fn VR_IVRCameraAccess_GetCameraInfo( unCameraIndex: u32, pCameraInfo: CameraInfo_t) -> bool;
	pub fn VR_IVRCameraAccess_GetCameraImage( unCameraIndex: u32, pCameraImage: CameraImage_t) -> bool;
	pub fn VR_IVRChaperone_GetCalibrationState( ) -> ChaperoneCalibrationState;
	pub fn VR_IVRChaperone_GetSoftBoundsInfo( pInfo: ChaperoneSoftBoundsInfo_t) -> bool;
	pub fn VR_IVRChaperone_GetHardBoundsInfo( pQuadsBuffer: HmdQuad_t, punQuadsCount: *mut u32) -> bool;
	pub fn VR_IVRChaperone_GetSeatedBoundsInfo( pInfo: ChaperoneSeatedBoundsInfo_t) -> bool;
	pub fn VR_IVRChaperoneSetup_CommitWorkingCopy( pchCalibrationName: *const u8) -> bool;
	pub fn VR_IVRChaperoneSetup_RevertWorkingCopy( ) ;
	pub fn VR_IVRChaperoneSetup_GetWorkingSoftBoundsInfo( pInfo: ChaperoneSoftBoundsInfo_t) -> bool;
	pub fn VR_IVRChaperoneSetup_GetWorkingHardBoundsInfo( pQuadsBuffer: HmdQuad_t, punQuadsCount: *mut u32) -> bool;
	pub fn VR_IVRChaperoneSetup_GetWorkingSeatedZeroPoseToRawTrackingPose( pmatSeatedZeroPoseToRawTrackingPose: HmdMatrix34_t) -> bool;
	pub fn VR_IVRChaperoneSetup_GetWorkingStandingZeroPoseToRawTrackingPose( pmatStandingZeroPoseToRawTrackingPose: HmdMatrix34_t) -> bool;
	pub fn VR_IVRChaperoneSetup_SetWorkingSoftBoundsInfo( pInfo: ChaperoneSoftBoundsInfo_t) ;
	pub fn VR_IVRChaperoneSetup_SetWorkingHardBoundsInfo( pQuadsBuffer: HmdQuad_t, unQuadsCount: u32) ;
	pub fn VR_IVRChaperoneSetup_SetWorkingSeatedZeroPoseToRawTrackingPose( matSeatedZeroPoseToRawTrackingPose: HmdMatrix34_t) ;
	pub fn VR_IVRChaperoneSetup_SetWorkingStandingZeroPoseToRawTrackingPose( matStandingZeroPoseToRawTrackingPose: HmdMatrix34_t) ;
	pub fn VR_IVRChaperoneSetup_GetWorkingTagPoses( pTagPosesBuffer: HmdMatrix34_t, punTagPosesCount: *mut u32) -> bool;
	pub fn VR_IVRChaperoneSetup_GetWorkingTagPoseScales( pflScaleBuffer: *mut f32, punTagPosesCount: *mut u32) -> bool;
	pub fn VR_IVRChaperoneSetup_GetWorkingTagPoseNameByIndex( nIndex: u32, pchBuffer: *const u8, unBufferSize: u32) -> u32;
	pub fn VR_IVRChaperoneSetup_GetWorkingTagPoseByName( pchTagName: *const u8, pmatTagPose: HmdMatrix34_t, pflScale: *mut f32) -> bool;
	pub fn VR_IVRChaperoneSetup_SetWorkingTagPoseByName( pchTagName: *const u8, matSeatedZeroPoseToRawTrackingPose: HmdMatrix34_t, flScale: f32) ;
	pub fn VR_IVRChaperoneSetup_RemoveWorkingTagPoseByName( pchTagName: *const u8) ;
	pub fn VR_IVRChaperoneSetup_RemoveAllWorkingTagPoses( ) ;
	pub fn VR_IVRChaperoneSetup_ReloadFromDisk( ) ;
	pub fn VR_IVRCompositor_GetLastError( pchBuffer: *const u8, unBufferSize: u32) -> u32;
	pub fn VR_IVRCompositor_SetVSync( bVSync: bool) ;
	pub fn VR_IVRCompositor_GetVSync( ) -> bool;
	pub fn VR_IVRCompositor_SetGamma( fGamma: f32) ;
	pub fn VR_IVRCompositor_GetGamma( ) -> f32;
	pub fn VR_IVRCompositor_SetGraphicsDevice( eType: Compositor_DeviceType, pDevice: *mut ()) ;
	pub fn VR_IVRCompositor_WaitGetPoses( pRenderPoseArray: TrackedDevicePose_t, unRenderPoseArrayCount: u32, pGamePoseArray: TrackedDevicePose_t, unGamePoseArrayCount: u32) -> VRCompositorError;
	pub fn VR_IVRCompositor_Submit( eEye: Hmd_Eye, pTexture: *mut (), pBounds: VRTextureBounds_t) -> VRCompositorError;
	pub fn VR_IVRCompositor_ClearLastSubmittedFrame( ) ;
	pub fn VR_IVRCompositor_GetFrameTiming( pTiming: Compositor_FrameTiming, unFramesAgo: u32) -> bool;
	pub fn VR_IVRCompositor_FadeToColor( fSeconds: f32, fRed: f32, fGreen: f32, fBlue: f32, fAlpha: f32, bBackground: bool) ;
	pub fn VR_IVRCompositor_FadeGrid( fSeconds: f32, bFadeIn: bool) ;
	pub fn VR_IVRCompositor_CompositorBringToFront( ) ;
	pub fn VR_IVRCompositor_CompositorGoToBack( ) ;
	pub fn VR_IVRCompositor_CompositorQuit( ) ;
	pub fn VR_IVRCompositor_IsFullscreen( ) -> bool;
	pub fn VR_IVRCompositor_SetTrackingSpace( eOrigin: TrackingUniverseOrigin) ;
	pub fn VR_IVRCompositor_GetTrackingSpace( ) -> TrackingUniverseOrigin;
	pub fn VR_IVRCompositor_GetCurrentSceneFocusProcess( ) -> u32;
	pub fn VR_IVRCompositor_CanRenderScene( ) -> bool;
	pub fn VR_IVROverlay_FindOverlay( pchOverlayKey: *const u8, pOverlayHandle: *mut VROverlayHandle_t) -> VROverlayError;
	pub fn VR_IVROverlay_CreateOverlay( pchOverlayKey: *const u8, pchOverlayFriendlyName: *const u8, pOverlayHandle: *mut VROverlayHandle_t) -> VROverlayError;
	pub fn VR_IVROverlay_DestroyOverlay( ulOverlayHandle: VROverlayHandle_t) -> VROverlayError;
	pub fn VR_IVROverlay_SetHighQualityOverlay( ulOverlayHandle: VROverlayHandle_t) -> VROverlayError;
	pub fn VR_IVROverlay_GetHighQualityOverlay( ) -> VROverlayHandle_t;
	pub fn VR_IVROverlay_GetOverlayErrorNameFromEnum( error: VROverlayError) -> *const u8;
	pub fn VR_IVROverlay_SetOverlayFlag( ulOverlayHandle: VROverlayHandle_t, eOverlayFlag: VROverlayFlags, bEnabled: bool) -> VROverlayError;
	pub fn VR_IVROverlay_GetOverlayFlag( ulOverlayHandle: VROverlayHandle_t, eOverlayFlag: VROverlayFlags, pbEnabled: *mut bool) -> VROverlayError;
	pub fn VR_IVROverlay_SetOverlayAlpha( ulOverlayHandle: VROverlayHandle_t, fAlpha: f32) -> VROverlayError;
	pub fn VR_IVROverlay_GetOverlayAlpha( ulOverlayHandle: VROverlayHandle_t, pfAlpha: *mut f32) -> VROverlayError;
	pub fn VR_IVROverlay_SetOverlayGamma( ulOverlayHandle: VROverlayHandle_t, fGamma: f32) -> VROverlayError;
	pub fn VR_IVROverlay_GetOverlayGamma( ulOverlayHandle: VROverlayHandle_t, pfGamma: *mut f32) -> VROverlayError;
	pub fn VR_IVROverlay_SetOverlayWidthInMeters( ulOverlayHandle: VROverlayHandle_t, fWidthInMeters: f32) -> VROverlayError;
	pub fn VR_IVROverlay_GetOverlayWidthInMeters( ulOverlayHandle: VROverlayHandle_t, pfWidthInMeters: *mut f32) -> VROverlayError;
	pub fn VR_IVROverlay_SetOverlayTextureBounds( ulOverlayHandle: VROverlayHandle_t, pOverlayTextureBounds: VRTextureBounds_t) -> VROverlayError;
	pub fn VR_IVROverlay_GetOverlayTextureBounds( ulOverlayHandle: VROverlayHandle_t, pOverlayTextureBounds: VRTextureBounds_t) -> VROverlayError;
	pub fn VR_IVROverlay_GetOverlayTransformType( ulOverlayHandle: VROverlayHandle_t, peTransformType: *mut VROverlayTransformType) -> VROverlayError;
	pub fn VR_IVROverlay_SetOverlayTransformAbsolute( ulOverlayHandle: VROverlayHandle_t, eTrackingOrigin: TrackingUniverseOrigin, pmatTrackingOriginToOverlayTransform: HmdMatrix34_t) -> VROverlayError;
	pub fn VR_IVROverlay_GetOverlayTransformAbsolute( ulOverlayHandle: VROverlayHandle_t, peTrackingOrigin: *mut TrackingUniverseOrigin, pmatTrackingOriginToOverlayTransform: HmdMatrix34_t) -> VROverlayError;
	pub fn VR_IVROverlay_SetOverlayTransformTrackedDeviceRelative( ulOverlayHandle: VROverlayHandle_t, unTrackedDevice: TrackedDeviceIndex_t, pmatTrackedDeviceToOverlayTransform: HmdMatrix34_t) -> VROverlayError;
	pub fn VR_IVROverlay_GetOverlayTransformTrackedDeviceRelative( ulOverlayHandle: VROverlayHandle_t, punTrackedDevice: *mut TrackedDeviceIndex_t, pmatTrackedDeviceToOverlayTransform: HmdMatrix34_t) -> VROverlayError;
	pub fn VR_IVROverlay_GetOverlayVisibility( ulOverlayHandle: VROverlayHandle_t, peOverlayVisibility: *mut VROverlayVisibility) -> VROverlayError;
	pub fn VR_IVROverlay_SetOverlayVisibility( ulOverlayHandle: VROverlayHandle_t, eOverlayVisibility: VROverlayVisibility) -> VROverlayError;
	pub fn VR_IVROverlay_ShowOverlay( ulOverlayHandle: VROverlayHandle_t) -> VROverlayError;
	pub fn VR_IVROverlay_HideOverlay( ulOverlayHandle: VROverlayHandle_t) -> VROverlayError;
	pub fn VR_IVROverlay_IsOverlayVisible( ulOverlayHandle: VROverlayHandle_t) -> bool;
	pub fn VR_IVROverlay_PollNextOverlayEvent( ulOverlayHandle: VROverlayHandle_t, pEvent: VREvent_t) -> bool;
	pub fn VR_IVROverlay_GetOverlayInputMethod( ulOverlayHandle: VROverlayHandle_t, peInputMethod: *mut VROverlayInputMethod) -> VROverlayError;
	pub fn VR_IVROverlay_SetOverlayInputMethod( ulOverlayHandle: VROverlayHandle_t, eInputMethod: VROverlayInputMethod) -> VROverlayError;
	pub fn VR_IVROverlay_GetOverlayMouseScale( ulOverlayHandle: VROverlayHandle_t, pvecMouseScale: HmdVector2_t) -> VROverlayError;
	pub fn VR_IVROverlay_SetOverlayMouseScale( ulOverlayHandle: VROverlayHandle_t, pvecMouseScale: HmdVector2_t) -> VROverlayError;
	pub fn VR_IVROverlay_ComputeOverlayIntersection( ulOverlayHandle: VROverlayHandle_t, pParams: VROverlayIntersectionParams_t, pResults: VROverlayIntersectionResults_t) -> bool;
	pub fn VR_IVROverlay_HandleControllerOverlayInteractionAsMouse( ulOverlayHandle: VROverlayHandle_t, unControllerDeviceIndex: TrackedDeviceIndex_t) -> bool;
	pub fn VR_IVROverlay_SetOverlayTexture( ulOverlayHandle: VROverlayHandle_t, pTexture: *mut ()) -> VROverlayError;
	pub fn VR_IVROverlay_SetOverlayRaw( ulOverlayHandle: VROverlayHandle_t, pvBuffer: *mut (), unWidth: u32, unHeight: u32, unDepth: u32) -> VROverlayError;
	pub fn VR_IVROverlay_SetOverlayFromFile( ulOverlayHandle: VROverlayHandle_t, pchFilePath: *const u8) -> VROverlayError;
	pub fn VR_IVROverlay_IsSystemOverlayVisible( ) -> bool;
	pub fn VR_IVROverlay_IsActiveSystemOverlay( ulOverlayHandle: VROverlayHandle_t) -> bool;
	pub fn VR_IVROverlay_SetSystemOverlaySceneProcess( ulOverlayHandle: VROverlayHandle_t, unProcessId: u32) -> VROverlayError;
	pub fn VR_IVROverlay_GetSystemOverlaySceneProcess( ulOverlayHandle: VROverlayHandle_t, punProcessId: *mut u32) -> VROverlayError;
	pub fn VR_IVRRenderModels_LoadRenderModel( pchRenderModelName: *const u8, pRenderModel: RenderModel_t) -> bool;
	pub fn VR_IVRRenderModels_FreeRenderModel( pRenderModel: RenderModel_t) ;
	pub fn VR_IVRRenderModels_GetRenderModelName( unRenderModelIndex: u32, pchRenderModelName: *const u8, unRenderModelNameLen: u32) -> u32;
	pub fn VR_IVRRenderModels_GetRenderModelCount( ) -> u32;
	pub fn VR_IVRNotifications_GetLastError( pchBuffer: *const u8, unBufferSize: u32) -> u32;
	pub fn VR_IVRNotifications_NotificationStart( _type: *const u8, texture: NotificationBitmap, notificationId: *mut VRNotificationId) -> bool;
	pub fn VR_IVRNotifications_UpdateTexture( notificationId: VRNotificationId, texture: NotificationBitmap) -> bool;
	pub fn VR_IVRNotifications_UpdateBitmap( notificationId: VRNotificationId, texture: NotificationBitmap) -> bool;
	pub fn VR_IVRNotifications_GetPointerLocation( notificationId: VRNotificationId, pointerActive: *mut bool, pointerX: *mut i32, pointerY: *mut i32) -> bool;
	pub fn VR_IVRNotifications_DismissNotification( notificationId: VRNotificationId) -> bool;
}
