use openvr_sys as sys;
use std::ffi::CStr;
use std::{error, fmt};

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum InitError {
    None = sys::EVRInitError_VRInitError_None as isize,
    Unknown = sys::EVRInitError_VRInitError_Unknown as isize,
    Init_InstallationNotFound = sys::EVRInitError_VRInitError_Init_InstallationNotFound as isize,
    Init_InstallationCorrupt = sys::EVRInitError_VRInitError_Init_InstallationCorrupt as isize,
    Init_VRClientDLLNotFound = sys::EVRInitError_VRInitError_Init_VRClientDLLNotFound as isize,
    Init_FileNotFound = sys::EVRInitError_VRInitError_Init_FileNotFound as isize,
    Init_FactoryNotFound = sys::EVRInitError_VRInitError_Init_FactoryNotFound as isize,
    Init_InterfaceNotFound = sys::EVRInitError_VRInitError_Init_InterfaceNotFound as isize,
    Init_InvalidInterface = sys::EVRInitError_VRInitError_Init_InvalidInterface as isize,
    Init_UserConfigDirectoryInvalid =
        sys::EVRInitError_VRInitError_Init_UserConfigDirectoryInvalid as isize,
    Init_HmdNotFound = sys::EVRInitError_VRInitError_Init_HmdNotFound as isize,
    Init_NotInitialized = sys::EVRInitError_VRInitError_Init_NotInitialized as isize,
    Init_PathRegistryNotFound = sys::EVRInitError_VRInitError_Init_PathRegistryNotFound as isize,
    Init_NoConfigPath = sys::EVRInitError_VRInitError_Init_NoConfigPath as isize,
    Init_NoLogPath = sys::EVRInitError_VRInitError_Init_NoLogPath as isize,
    Init_PathRegistryNotWritable =
        sys::EVRInitError_VRInitError_Init_PathRegistryNotWritable as isize,
    Init_AppInfoInitFailed = sys::EVRInitError_VRInitError_Init_AppInfoInitFailed as isize,
    Init_Retry = sys::EVRInitError_VRInitError_Init_Retry as isize,
    Init_InitCanceledByUser = sys::EVRInitError_VRInitError_Init_InitCanceledByUser as isize,
    Init_AnotherAppLaunching = sys::EVRInitError_VRInitError_Init_AnotherAppLaunching as isize,
    Init_SettingsInitFailed = sys::EVRInitError_VRInitError_Init_SettingsInitFailed as isize,
    Init_ShuttingDown = sys::EVRInitError_VRInitError_Init_ShuttingDown as isize,
    Init_TooManyObjects = sys::EVRInitError_VRInitError_Init_TooManyObjects as isize,
    Init_NoServerForBackgroundApp =
        sys::EVRInitError_VRInitError_Init_NoServerForBackgroundApp as isize,
    Init_NotSupportedWithCompositor =
        sys::EVRInitError_VRInitError_Init_NotSupportedWithCompositor as isize,
    Init_NotAvailableToUtilityApps =
        sys::EVRInitError_VRInitError_Init_NotAvailableToUtilityApps as isize,
    Init_Internal = sys::EVRInitError_VRInitError_Init_Internal as isize,
    Init_HmdDriverIdIsNone = sys::EVRInitError_VRInitError_Init_HmdDriverIdIsNone as isize,
    Init_HmdNotFoundPresenceFailed =
        sys::EVRInitError_VRInitError_Init_HmdNotFoundPresenceFailed as isize,
    Init_VRMonitorNotFound = sys::EVRInitError_VRInitError_Init_VRMonitorNotFound as isize,
    Init_VRMonitorStartupFailed =
        sys::EVRInitError_VRInitError_Init_VRMonitorStartupFailed as isize,
    Init_LowPowerWatchdogNotSupported =
        sys::EVRInitError_VRInitError_Init_LowPowerWatchdogNotSupported as isize,
    Init_InvalidApplicationType =
        sys::EVRInitError_VRInitError_Init_InvalidApplicationType as isize,
    Init_NotAvailableToWatchdogApps =
        sys::EVRInitError_VRInitError_Init_NotAvailableToWatchdogApps as isize,
    Init_WatchdogDisabledInSettings =
        sys::EVRInitError_VRInitError_Init_WatchdogDisabledInSettings as isize,
    Init_VRDashboardNotFound = sys::EVRInitError_VRInitError_Init_VRDashboardNotFound as isize,
    Init_VRDashboardStartupFailed =
        sys::EVRInitError_VRInitError_Init_VRDashboardStartupFailed as isize,
    Init_VRHomeNotFound = sys::EVRInitError_VRInitError_Init_VRHomeNotFound as isize,
    Init_VRHomeStartupFailed = sys::EVRInitError_VRInitError_Init_VRHomeStartupFailed as isize,
    Init_RebootingBusy = sys::EVRInitError_VRInitError_Init_RebootingBusy as isize,
    Init_FirmwareUpdateBusy = sys::EVRInitError_VRInitError_Init_FirmwareUpdateBusy as isize,
    Init_FirmwareRecoveryBusy = sys::EVRInitError_VRInitError_Init_FirmwareRecoveryBusy as isize,
    Init_USBServiceBusy = sys::EVRInitError_VRInitError_Init_USBServiceBusy as isize,
    Init_VRWebHelperStartupFailed =
        sys::EVRInitError_VRInitError_Init_VRWebHelperStartupFailed as isize,
    Init_TrackerManagerInitFailed =
        sys::EVRInitError_VRInitError_Init_TrackerManagerInitFailed as isize,
    Init_AlreadyRunning = sys::EVRInitError_VRInitError_Init_AlreadyRunning as isize,
    Init_FailedForVrMonitor = sys::EVRInitError_VRInitError_Init_FailedForVrMonitor as isize,
    Init_PropertyManagerInitFailed =
        sys::EVRInitError_VRInitError_Init_PropertyManagerInitFailed as isize,
    Init_WebServerFailed = sys::EVRInitError_VRInitError_Init_WebServerFailed as isize,
    Init_IllegalTypeTransition = sys::EVRInitError_VRInitError_Init_IllegalTypeTransition as isize,
    Init_MismatchedRuntimes = sys::EVRInitError_VRInitError_Init_MismatchedRuntimes as isize,
    Init_InvalidProcessId = sys::EVRInitError_VRInitError_Init_InvalidProcessId as isize,
    Init_VRServiceStartupFailed =
        sys::EVRInitError_VRInitError_Init_VRServiceStartupFailed as isize,
    Init_PrismNeedsNewDrivers = sys::EVRInitError_VRInitError_Init_PrismNeedsNewDrivers as isize,
    Init_PrismStartupTimedOut = sys::EVRInitError_VRInitError_Init_PrismStartupTimedOut as isize,
    Init_CouldNotStartPrism = sys::EVRInitError_VRInitError_Init_CouldNotStartPrism as isize,
    Init_PrismClientInitFailed = sys::EVRInitError_VRInitError_Init_PrismClientInitFailed as isize,
    Init_PrismClientStartFailed =
        sys::EVRInitError_VRInitError_Init_PrismClientStartFailed as isize,
    Init_PrismExitedUnexpectedly =
        sys::EVRInitError_VRInitError_Init_PrismExitedUnexpectedly as isize,
    Init_BadLuid = sys::EVRInitError_VRInitError_Init_BadLuid as isize,
    Init_NoServerForAppContainer =
        sys::EVRInitError_VRInitError_Init_NoServerForAppContainer as isize,
    Init_DuplicateBootstrapper = sys::EVRInitError_VRInitError_Init_DuplicateBootstrapper as isize,
    Init_VRDashboardServicePending =
        sys::EVRInitError_VRInitError_Init_VRDashboardServicePending as isize,
    Init_VRDashboardServiceTimeout =
        sys::EVRInitError_VRInitError_Init_VRDashboardServiceTimeout as isize,
    Init_VRDashboardServiceStopped =
        sys::EVRInitError_VRInitError_Init_VRDashboardServiceStopped as isize,
    Init_VRDashboardAlreadyStarted =
        sys::EVRInitError_VRInitError_Init_VRDashboardAlreadyStarted as isize,
    Init_VRDashboardCopyFailed = sys::EVRInitError_VRInitError_Init_VRDashboardCopyFailed as isize,
    Init_VRDashboardTokenFailure =
        sys::EVRInitError_VRInitError_Init_VRDashboardTokenFailure as isize,
    Init_VRDashboardEnvironmentFailure =
        sys::EVRInitError_VRInitError_Init_VRDashboardEnvironmentFailure as isize,
    Init_VRDashboardPathFailure =
        sys::EVRInitError_VRInitError_Init_VRDashboardPathFailure as isize,
    Driver_Failed = sys::EVRInitError_VRInitError_Driver_Failed as isize,
    Driver_Unknown = sys::EVRInitError_VRInitError_Driver_Unknown as isize,
    Driver_HmdUnknown = sys::EVRInitError_VRInitError_Driver_HmdUnknown as isize,
    Driver_NotLoaded = sys::EVRInitError_VRInitError_Driver_NotLoaded as isize,
    Driver_RuntimeOutOfDate = sys::EVRInitError_VRInitError_Driver_RuntimeOutOfDate as isize,
    Driver_HmdInUse = sys::EVRInitError_VRInitError_Driver_HmdInUse as isize,
    Driver_NotCalibrated = sys::EVRInitError_VRInitError_Driver_NotCalibrated as isize,
    Driver_CalibrationInvalid = sys::EVRInitError_VRInitError_Driver_CalibrationInvalid as isize,
    Driver_HmdDisplayNotFound = sys::EVRInitError_VRInitError_Driver_HmdDisplayNotFound as isize,
    Driver_TrackedDeviceInterfaceUnknown =
        sys::EVRInitError_VRInitError_Driver_TrackedDeviceInterfaceUnknown as isize,
    Driver_HmdDriverIdOutOfBounds =
        sys::EVRInitError_VRInitError_Driver_HmdDriverIdOutOfBounds as isize,
    Driver_HmdDisplayMirrored = sys::EVRInitError_VRInitError_Driver_HmdDisplayMirrored as isize,
    Driver_HmdDisplayNotFoundLaptop =
        sys::EVRInitError_VRInitError_Driver_HmdDisplayNotFoundLaptop as isize,
    Driver_PeerDriverNotInstalled =
        sys::EVRInitError_VRInitError_Driver_PeerDriverNotInstalled as isize,
    Driver_WirelessHmdNotConnected =
        sys::EVRInitError_VRInitError_Driver_WirelessHmdNotConnected as isize,
    IPC_ServerInitFailed = sys::EVRInitError_VRInitError_IPC_ServerInitFailed as isize,
    IPC_ConnectFailed = sys::EVRInitError_VRInitError_IPC_ConnectFailed as isize,
    IPC_SharedStateInitFailed = sys::EVRInitError_VRInitError_IPC_SharedStateInitFailed as isize,
    IPC_CompositorInitFailed = sys::EVRInitError_VRInitError_IPC_CompositorInitFailed as isize,
    IPC_MutexInitFailed = sys::EVRInitError_VRInitError_IPC_MutexInitFailed as isize,
    IPC_Failed = sys::EVRInitError_VRInitError_IPC_Failed as isize,
    IPC_CompositorConnectFailed =
        sys::EVRInitError_VRInitError_IPC_CompositorConnectFailed as isize,
    IPC_CompositorInvalidConnectResponse =
        sys::EVRInitError_VRInitError_IPC_CompositorInvalidConnectResponse as isize,
    IPC_ConnectFailedAfterMultipleAttempts =
        sys::EVRInitError_VRInitError_IPC_ConnectFailedAfterMultipleAttempts as isize,
    IPC_ConnectFailedAfterTargetExited =
        sys::EVRInitError_VRInitError_IPC_ConnectFailedAfterTargetExited as isize,
    IPC_NamespaceUnavailable = sys::EVRInitError_VRInitError_IPC_NamespaceUnavailable as isize,
    Compositor_Failed = sys::EVRInitError_VRInitError_Compositor_Failed as isize,
    Compositor_D3D11HardwareRequired =
        sys::EVRInitError_VRInitError_Compositor_D3D11HardwareRequired as isize,
    Compositor_FirmwareRequiresUpdate =
        sys::EVRInitError_VRInitError_Compositor_FirmwareRequiresUpdate as isize,
    Compositor_OverlayInitFailed =
        sys::EVRInitError_VRInitError_Compositor_OverlayInitFailed as isize,
    Compositor_ScreenshotsInitFailed =
        sys::EVRInitError_VRInitError_Compositor_ScreenshotsInitFailed as isize,
    Compositor_UnableToCreateDevice =
        sys::EVRInitError_VRInitError_Compositor_UnableToCreateDevice as isize,
    Compositor_SharedStateIsNull =
        sys::EVRInitError_VRInitError_Compositor_SharedStateIsNull as isize,
    Compositor_NotificationManagerIsNull =
        sys::EVRInitError_VRInitError_Compositor_NotificationManagerIsNull as isize,
    Compositor_ResourceManagerClientIsNull =
        sys::EVRInitError_VRInitError_Compositor_ResourceManagerClientIsNull as isize,
    Compositor_MessageOverlaySharedStateInitFailure =
        sys::EVRInitError_VRInitError_Compositor_MessageOverlaySharedStateInitFailure as isize,
    Compositor_PropertiesInterfaceIsNull =
        sys::EVRInitError_VRInitError_Compositor_PropertiesInterfaceIsNull as isize,
    Compositor_CreateFullscreenWindowFailed =
        sys::EVRInitError_VRInitError_Compositor_CreateFullscreenWindowFailed as isize,
    Compositor_SettingsInterfaceIsNull =
        sys::EVRInitError_VRInitError_Compositor_SettingsInterfaceIsNull as isize,
    Compositor_FailedToShowWindow =
        sys::EVRInitError_VRInitError_Compositor_FailedToShowWindow as isize,
    Compositor_DistortInterfaceIsNull =
        sys::EVRInitError_VRInitError_Compositor_DistortInterfaceIsNull as isize,
    Compositor_DisplayFrequencyFailure =
        sys::EVRInitError_VRInitError_Compositor_DisplayFrequencyFailure as isize,
    Compositor_RendererInitializationFailed =
        sys::EVRInitError_VRInitError_Compositor_RendererInitializationFailed as isize,
    Compositor_DXGIFactoryInterfaceIsNull =
        sys::EVRInitError_VRInitError_Compositor_DXGIFactoryInterfaceIsNull as isize,
    Compositor_DXGIFactoryCreateFailed =
        sys::EVRInitError_VRInitError_Compositor_DXGIFactoryCreateFailed as isize,
    Compositor_DXGIFactoryQueryFailed =
        sys::EVRInitError_VRInitError_Compositor_DXGIFactoryQueryFailed as isize,
    Compositor_InvalidAdapterDesktop =
        sys::EVRInitError_VRInitError_Compositor_InvalidAdapterDesktop as isize,
    Compositor_InvalidHmdAttachment =
        sys::EVRInitError_VRInitError_Compositor_InvalidHmdAttachment as isize,
    Compositor_InvalidOutputDesktop =
        sys::EVRInitError_VRInitError_Compositor_InvalidOutputDesktop as isize,
    Compositor_InvalidDeviceProvided =
        sys::EVRInitError_VRInitError_Compositor_InvalidDeviceProvided as isize,
    Compositor_D3D11RendererInitializationFailed =
        sys::EVRInitError_VRInitError_Compositor_D3D11RendererInitializationFailed as isize,
    Compositor_FailedToFindDisplayMode =
        sys::EVRInitError_VRInitError_Compositor_FailedToFindDisplayMode as isize,
    Compositor_FailedToCreateSwapChain =
        sys::EVRInitError_VRInitError_Compositor_FailedToCreateSwapChain as isize,
    Compositor_FailedToGetBackBuffer =
        sys::EVRInitError_VRInitError_Compositor_FailedToGetBackBuffer as isize,
    Compositor_FailedToCreateRenderTarget =
        sys::EVRInitError_VRInitError_Compositor_FailedToCreateRenderTarget as isize,
    Compositor_FailedToCreateDXGI2SwapChain =
        sys::EVRInitError_VRInitError_Compositor_FailedToCreateDXGI2SwapChain as isize,
    Compositor_FailedtoGetDXGI2BackBuffer =
        sys::EVRInitError_VRInitError_Compositor_FailedtoGetDXGI2BackBuffer as isize,
    Compositor_FailedToCreateDXGI2RenderTarget =
        sys::EVRInitError_VRInitError_Compositor_FailedToCreateDXGI2RenderTarget as isize,
    Compositor_FailedToGetDXGIDeviceInterface =
        sys::EVRInitError_VRInitError_Compositor_FailedToGetDXGIDeviceInterface as isize,
    Compositor_SelectDisplayMode =
        sys::EVRInitError_VRInitError_Compositor_SelectDisplayMode as isize,
    Compositor_FailedToCreateNvAPIRenderTargets =
        sys::EVRInitError_VRInitError_Compositor_FailedToCreateNvAPIRenderTargets as isize,
    Compositor_NvAPISetDisplayMode =
        sys::EVRInitError_VRInitError_Compositor_NvAPISetDisplayMode as isize,
    Compositor_FailedToCreateDirectModeDisplay =
        sys::EVRInitError_VRInitError_Compositor_FailedToCreateDirectModeDisplay as isize,
    Compositor_InvalidHmdPropertyContainer =
        sys::EVRInitError_VRInitError_Compositor_InvalidHmdPropertyContainer as isize,
    Compositor_UpdateDisplayFrequency =
        sys::EVRInitError_VRInitError_Compositor_UpdateDisplayFrequency as isize,
    Compositor_CreateRasterizerState =
        sys::EVRInitError_VRInitError_Compositor_CreateRasterizerState as isize,
    Compositor_CreateWireframeRasterizerState =
        sys::EVRInitError_VRInitError_Compositor_CreateWireframeRasterizerState as isize,
    Compositor_CreateSamplerState =
        sys::EVRInitError_VRInitError_Compositor_CreateSamplerState as isize,
    Compositor_CreateClampToBorderSamplerState =
        sys::EVRInitError_VRInitError_Compositor_CreateClampToBorderSamplerState as isize,
    Compositor_CreateAnisoSamplerState =
        sys::EVRInitError_VRInitError_Compositor_CreateAnisoSamplerState as isize,
    Compositor_CreateOverlaySamplerState =
        sys::EVRInitError_VRInitError_Compositor_CreateOverlaySamplerState as isize,
    Compositor_CreatePanoramaSamplerState =
        sys::EVRInitError_VRInitError_Compositor_CreatePanoramaSamplerState as isize,
    Compositor_CreateFontSamplerState =
        sys::EVRInitError_VRInitError_Compositor_CreateFontSamplerState as isize,
    Compositor_CreateNoBlendState =
        sys::EVRInitError_VRInitError_Compositor_CreateNoBlendState as isize,
    Compositor_CreateBlendState =
        sys::EVRInitError_VRInitError_Compositor_CreateBlendState as isize,
    Compositor_CreateAlphaBlendState =
        sys::EVRInitError_VRInitError_Compositor_CreateAlphaBlendState as isize,
    Compositor_CreateBlendStateMaskR =
        sys::EVRInitError_VRInitError_Compositor_CreateBlendStateMaskR as isize,
    Compositor_CreateBlendStateMaskG =
        sys::EVRInitError_VRInitError_Compositor_CreateBlendStateMaskG as isize,
    Compositor_CreateBlendStateMaskB =
        sys::EVRInitError_VRInitError_Compositor_CreateBlendStateMaskB as isize,
    Compositor_CreateDepthStencilState =
        sys::EVRInitError_VRInitError_Compositor_CreateDepthStencilState as isize,
    Compositor_CreateDepthStencilStateNoWrite =
        sys::EVRInitError_VRInitError_Compositor_CreateDepthStencilStateNoWrite as isize,
    Compositor_CreateDepthStencilStateNoDepth =
        sys::EVRInitError_VRInitError_Compositor_CreateDepthStencilStateNoDepth as isize,
    Compositor_CreateFlushTexture =
        sys::EVRInitError_VRInitError_Compositor_CreateFlushTexture as isize,
    Compositor_CreateDistortionSurfaces =
        sys::EVRInitError_VRInitError_Compositor_CreateDistortionSurfaces as isize,
    Compositor_CreateConstantBuffer =
        sys::EVRInitError_VRInitError_Compositor_CreateConstantBuffer as isize,
    Compositor_CreateHmdPoseConstantBuffer =
        sys::EVRInitError_VRInitError_Compositor_CreateHmdPoseConstantBuffer as isize,
    Compositor_CreateHmdPoseStagingConstantBuffer =
        sys::EVRInitError_VRInitError_Compositor_CreateHmdPoseStagingConstantBuffer as isize,
    Compositor_CreateSharedFrameInfoConstantBuffer =
        sys::EVRInitError_VRInitError_Compositor_CreateSharedFrameInfoConstantBuffer as isize,
    Compositor_CreateOverlayConstantBuffer =
        sys::EVRInitError_VRInitError_Compositor_CreateOverlayConstantBuffer as isize,
    Compositor_CreateSceneTextureIndexConstantBuffer =
        sys::EVRInitError_VRInitError_Compositor_CreateSceneTextureIndexConstantBuffer as isize,
    Compositor_CreateReadableSceneTextureIndexConstantBuffer =
        sys::EVRInitError_VRInitError_Compositor_CreateReadableSceneTextureIndexConstantBuffer
            as isize,
    Compositor_CreateLayerGraphicsTextureIndexConstantBuffer =
        sys::EVRInitError_VRInitError_Compositor_CreateLayerGraphicsTextureIndexConstantBuffer
            as isize,
    Compositor_CreateLayerComputeTextureIndexConstantBuffer =
        sys::EVRInitError_VRInitError_Compositor_CreateLayerComputeTextureIndexConstantBuffer
            as isize,
    Compositor_CreateLayerComputeSceneTextureIndexConstantBuffer =
        sys::EVRInitError_VRInitError_Compositor_CreateLayerComputeSceneTextureIndexConstantBuffer
            as isize,
    Compositor_CreateComputeHmdPoseConstantBuffer =
        sys::EVRInitError_VRInitError_Compositor_CreateComputeHmdPoseConstantBuffer as isize,
    Compositor_CreateGeomConstantBuffer =
        sys::EVRInitError_VRInitError_Compositor_CreateGeomConstantBuffer as isize,
    Compositor_CreatePanelMaskConstantBuffer =
        sys::EVRInitError_VRInitError_Compositor_CreatePanelMaskConstantBuffer as isize,
    Compositor_CreatePixelSimUBO =
        sys::EVRInitError_VRInitError_Compositor_CreatePixelSimUBO as isize,
    Compositor_CreateMSAARenderTextures =
        sys::EVRInitError_VRInitError_Compositor_CreateMSAARenderTextures as isize,
    Compositor_CreateResolveRenderTextures =
        sys::EVRInitError_VRInitError_Compositor_CreateResolveRenderTextures as isize,
    Compositor_CreateComputeResolveRenderTextures =
        sys::EVRInitError_VRInitError_Compositor_CreateComputeResolveRenderTextures as isize,
    Compositor_CreateDriverDirectModeResolveTextures =
        sys::EVRInitError_VRInitError_Compositor_CreateDriverDirectModeResolveTextures as isize,
    Compositor_OpenDriverDirectModeResolveTextures =
        sys::EVRInitError_VRInitError_Compositor_OpenDriverDirectModeResolveTextures as isize,
    Compositor_CreateFallbackSyncTexture =
        sys::EVRInitError_VRInitError_Compositor_CreateFallbackSyncTexture as isize,
    Compositor_ShareFallbackSyncTexture =
        sys::EVRInitError_VRInitError_Compositor_ShareFallbackSyncTexture as isize,
    Compositor_CreateOverlayIndexBuffer =
        sys::EVRInitError_VRInitError_Compositor_CreateOverlayIndexBuffer as isize,
    Compositor_CreateOverlayVertexBuffer =
        sys::EVRInitError_VRInitError_Compositor_CreateOverlayVertexBuffer as isize,
    Compositor_CreateTextVertexBuffer =
        sys::EVRInitError_VRInitError_Compositor_CreateTextVertexBuffer as isize,
    Compositor_CreateTextIndexBuffer =
        sys::EVRInitError_VRInitError_Compositor_CreateTextIndexBuffer as isize,
    Compositor_CreateMirrorTextures =
        sys::EVRInitError_VRInitError_Compositor_CreateMirrorTextures as isize,
    Compositor_CreateLastFrameRenderTexture =
        sys::EVRInitError_VRInitError_Compositor_CreateLastFrameRenderTexture as isize,
    Compositor_CreateMirrorOverlay =
        sys::EVRInitError_VRInitError_Compositor_CreateMirrorOverlay as isize,
    Compositor_FailedToCreateVirtualDisplayBackbuffer =
        sys::EVRInitError_VRInitError_Compositor_FailedToCreateVirtualDisplayBackbuffer as isize,
    Compositor_DisplayModeNotSupported =
        sys::EVRInitError_VRInitError_Compositor_DisplayModeNotSupported as isize,
    Compositor_CreateOverlayInvalidCall =
        sys::EVRInitError_VRInitError_Compositor_CreateOverlayInvalidCall as isize,
    Compositor_CreateOverlayAlreadyInitialized =
        sys::EVRInitError_VRInitError_Compositor_CreateOverlayAlreadyInitialized as isize,
    Compositor_FailedToCreateMailbox =
        sys::EVRInitError_VRInitError_Compositor_FailedToCreateMailbox as isize,
    Compositor_WindowInterfaceIsNull =
        sys::EVRInitError_VRInitError_Compositor_WindowInterfaceIsNull as isize,
    Compositor_SystemLayerCreateInstance =
        sys::EVRInitError_VRInitError_Compositor_SystemLayerCreateInstance as isize,
    Compositor_SystemLayerCreateSession =
        sys::EVRInitError_VRInitError_Compositor_SystemLayerCreateSession as isize,
    Compositor_CreateInverseDistortUVs =
        sys::EVRInitError_VRInitError_Compositor_CreateInverseDistortUVs as isize,
    Compositor_CreateBackbufferDepth =
        sys::EVRInitError_VRInitError_Compositor_CreateBackbufferDepth as isize,
    Compositor_CannotDRMLeaseDisplay =
        sys::EVRInitError_VRInitError_Compositor_CannotDRMLeaseDisplay as isize,
    Compositor_CannotConnectToDisplayServer =
        sys::EVRInitError_VRInitError_Compositor_CannotConnectToDisplayServer as isize,
    Compositor_GnomeNoDRMLeasing =
        sys::EVRInitError_VRInitError_Compositor_GnomeNoDRMLeasing as isize,
    Compositor_FailedToInitializeEncoder =
        sys::EVRInitError_VRInitError_Compositor_FailedToInitializeEncoder as isize,
    Compositor_CreateBlurTexture =
        sys::EVRInitError_VRInitError_Compositor_CreateBlurTexture as isize,
    VendorSpecific_UnableToConnectToOculusRuntime =
        sys::EVRInitError_VRInitError_VendorSpecific_UnableToConnectToOculusRuntime as isize,
    VendorSpecific_WindowsNotInDevMode =
        sys::EVRInitError_VRInitError_VendorSpecific_WindowsNotInDevMode as isize,
    VendorSpecific_OculusLinkNotEnabled =
        sys::EVRInitError_VRInitError_VendorSpecific_OculusLinkNotEnabled as isize,
    VendorSpecific_HmdFound_CantOpenDevice =
        sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_CantOpenDevice as isize,
    VendorSpecific_HmdFound_UnableToRequestConfigStart =
        sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UnableToRequestConfigStart as isize,
    VendorSpecific_HmdFound_NoStoredConfig =
        sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_NoStoredConfig as isize,
    VendorSpecific_HmdFound_ConfigTooBig =
        sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_ConfigTooBig as isize,
    VendorSpecific_HmdFound_ConfigTooSmall =
        sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_ConfigTooSmall as isize,
    VendorSpecific_HmdFound_UnableToInitZLib =
        sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UnableToInitZLib as isize,
    VendorSpecific_HmdFound_CantReadFirmwareVersion =
        sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_CantReadFirmwareVersion as isize,
    VendorSpecific_HmdFound_UnableToSendUserDataStart =
        sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UnableToSendUserDataStart as isize,
    VendorSpecific_HmdFound_UnableToGetUserDataStart =
        sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UnableToGetUserDataStart as isize,
    VendorSpecific_HmdFound_UnableToGetUserDataNext =
        sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UnableToGetUserDataNext as isize,
    VendorSpecific_HmdFound_UserDataAddressRange =
        sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UserDataAddressRange as isize,
    VendorSpecific_HmdFound_UserDataError =
        sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UserDataError as isize,
    VendorSpecific_HmdFound_ConfigFailedSanityCheck =
        sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_ConfigFailedSanityCheck as isize,
    VendorSpecific_OculusRuntimeBadInstall =
        sys::EVRInitError_VRInitError_VendorSpecific_OculusRuntimeBadInstall as isize,
    VendorSpecific_HmdFound_UnexpectedConfiguration_1 =
        sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UnexpectedConfiguration_1 as isize,
    Steam_SteamInstallationNotFound =
        sys::EVRInitError_VRInitError_Steam_SteamInstallationNotFound as isize,
    LastError = sys::EVRInitError_VRInitError_LastError as isize,
}

impl PartialEq<InitError> for &InitError {
    fn eq(&self, other: &InitError) -> bool {
        *self == other
    }
}

impl InitError {
    fn to_sys_err(&self) -> sys::EVRInitError {
        return *self as sys::EVRInitError;
    }
}

impl From<sys::EVRInitError> for InitError {
    fn from(value: sys::EVRInitError) -> Self {
        match value {
            sys::EVRInitError_VRInitError_None => Self::None,
            sys::EVRInitError_VRInitError_Unknown => Self::Unknown,
            sys::EVRInitError_VRInitError_Init_InstallationNotFound => Self::Init_InstallationNotFound,
            sys::EVRInitError_VRInitError_Init_InstallationCorrupt=>Self::Init_InstallationCorrupt,
            sys::EVRInitError_VRInitError_Init_VRClientDLLNotFound=>Self::Init_VRClientDLLNotFound,
            sys::EVRInitError_VRInitError_Init_FileNotFound=>Self::Init_FileNotFound,
            sys::EVRInitError_VRInitError_Init_FactoryNotFound=>Self::Init_FactoryNotFound,
            sys::EVRInitError_VRInitError_Init_InterfaceNotFound=>Self::Init_InterfaceNotFound,
            sys::EVRInitError_VRInitError_Init_InvalidInterface=>Self::Init_InvalidInterface,
            sys::EVRInitError_VRInitError_Init_UserConfigDirectoryInvalid=>Self::Init_UserConfigDirectoryInvalid ,
            sys::EVRInitError_VRInitError_Init_HmdNotFound=>Self::Init_HmdNotFound ,
            sys::EVRInitError_VRInitError_Init_NotInitialized=>Self::Init_NotInitialized ,
            sys::EVRInitError_VRInitError_Init_PathRegistryNotFound=>Self::Init_PathRegistryNotFound ,
            sys::EVRInitError_VRInitError_Init_NoConfigPath=>Self::Init_NoConfigPath ,
            sys::EVRInitError_VRInitError_Init_NoLogPath=>Self::Init_NoLogPath ,
            sys::EVRInitError_VRInitError_Init_PathRegistryNotWritable=>Self::Init_PathRegistryNotWritable ,
            sys::EVRInitError_VRInitError_Init_AppInfoInitFailed=>Self::Init_AppInfoInitFailed ,
            sys::EVRInitError_VRInitError_Init_Retry=>Self::Init_Retry ,
            sys::EVRInitError_VRInitError_Init_InitCanceledByUser=>Self::Init_InitCanceledByUser ,
            sys::EVRInitError_VRInitError_Init_AnotherAppLaunching=>Self::Init_AnotherAppLaunching ,
            sys::EVRInitError_VRInitError_Init_SettingsInitFailed=>Self::Init_SettingsInitFailed ,
            sys::EVRInitError_VRInitError_Init_ShuttingDown=>Self::Init_ShuttingDown ,
            sys::EVRInitError_VRInitError_Init_TooManyObjects=>Self::Init_TooManyObjects ,
            sys::EVRInitError_VRInitError_Init_NoServerForBackgroundApp=>Self::Init_NoServerForBackgroundApp ,
            sys::EVRInitError_VRInitError_Init_NotSupportedWithCompositor=>Self::Init_NotSupportedWithCompositor ,
            sys::EVRInitError_VRInitError_Init_NotAvailableToUtilityApps=>Self::Init_NotAvailableToUtilityApps ,
            sys::EVRInitError_VRInitError_Init_Internal=>Self::Init_Internal ,
            sys::EVRInitError_VRInitError_Init_HmdDriverIdIsNone=>Self::Init_HmdDriverIdIsNone ,
            sys::EVRInitError_VRInitError_Init_HmdNotFoundPresenceFailed=>Self::Init_HmdNotFoundPresenceFailed ,
            sys::EVRInitError_VRInitError_Init_VRMonitorNotFound=>Self::Init_VRMonitorNotFound ,
            sys::EVRInitError_VRInitError_Init_VRMonitorStartupFailed=>Self::Init_VRMonitorStartupFailed ,
            sys::EVRInitError_VRInitError_Init_LowPowerWatchdogNotSupported=>Self::Init_LowPowerWatchdogNotSupported ,
            sys::EVRInitError_VRInitError_Init_InvalidApplicationType=>Self::Init_InvalidApplicationType ,
            sys::EVRInitError_VRInitError_Init_NotAvailableToWatchdogApps=>Self::Init_NotAvailableToWatchdogApps ,
            sys::EVRInitError_VRInitError_Init_WatchdogDisabledInSettings=>Self::Init_WatchdogDisabledInSettings ,
            sys::EVRInitError_VRInitError_Init_VRDashboardNotFound=>Self::Init_VRDashboardNotFound ,
            sys::EVRInitError_VRInitError_Init_VRDashboardStartupFailed=>Self::Init_VRDashboardStartupFailed ,
            sys::EVRInitError_VRInitError_Init_VRHomeNotFound=>Self::Init_VRHomeNotFound ,
            sys::EVRInitError_VRInitError_Init_VRHomeStartupFailed=>Self::Init_VRHomeStartupFailed ,
            sys::EVRInitError_VRInitError_Init_RebootingBusy=>Self::Init_RebootingBusy ,
            sys::EVRInitError_VRInitError_Init_FirmwareUpdateBusy=>Self::Init_FirmwareUpdateBusy ,
            sys::EVRInitError_VRInitError_Init_FirmwareRecoveryBusy=>Self::Init_FirmwareRecoveryBusy ,
            sys::EVRInitError_VRInitError_Init_USBServiceBusy=>Self::Init_USBServiceBusy ,
            sys::EVRInitError_VRInitError_Init_VRWebHelperStartupFailed=>Self::Init_VRWebHelperStartupFailed ,
            sys::EVRInitError_VRInitError_Init_TrackerManagerInitFailed=>Self::Init_TrackerManagerInitFailed ,
            sys::EVRInitError_VRInitError_Init_AlreadyRunning=>Self::Init_AlreadyRunning ,
            sys::EVRInitError_VRInitError_Init_FailedForVrMonitor=>Self::Init_FailedForVrMonitor ,
            sys::EVRInitError_VRInitError_Init_PropertyManagerInitFailed=>Self::Init_PropertyManagerInitFailed ,
            sys::EVRInitError_VRInitError_Init_WebServerFailed=>Self::Init_WebServerFailed ,
            sys::EVRInitError_VRInitError_Init_IllegalTypeTransition=>Self::Init_IllegalTypeTransition ,
            sys::EVRInitError_VRInitError_Init_MismatchedRuntimes=>Self::Init_MismatchedRuntimes ,
            sys::EVRInitError_VRInitError_Init_InvalidProcessId=>Self::Init_InvalidProcessId ,
            sys::EVRInitError_VRInitError_Init_VRServiceStartupFailed=>Self::Init_VRServiceStartupFailed ,
            sys::EVRInitError_VRInitError_Init_PrismNeedsNewDrivers=>Self::Init_PrismNeedsNewDrivers ,
            sys::EVRInitError_VRInitError_Init_PrismStartupTimedOut=>Self::Init_PrismStartupTimedOut ,
            sys::EVRInitError_VRInitError_Init_CouldNotStartPrism=>Self::Init_CouldNotStartPrism ,
            sys::EVRInitError_VRInitError_Init_PrismClientInitFailed=>Self::Init_PrismClientInitFailed ,
            sys::EVRInitError_VRInitError_Init_PrismClientStartFailed=>Self::Init_PrismClientStartFailed ,
            sys::EVRInitError_VRInitError_Init_PrismExitedUnexpectedly=>Self::Init_PrismExitedUnexpectedly ,
            sys::EVRInitError_VRInitError_Init_BadLuid=>Self::Init_BadLuid ,
            sys::EVRInitError_VRInitError_Init_NoServerForAppContainer=>Self::Init_NoServerForAppContainer ,
            sys::EVRInitError_VRInitError_Init_DuplicateBootstrapper=>Self::Init_DuplicateBootstrapper ,
            sys::EVRInitError_VRInitError_Init_VRDashboardServicePending=>Self::Init_VRDashboardServicePending ,
            sys::EVRInitError_VRInitError_Init_VRDashboardServiceTimeout=>Self::Init_VRDashboardServiceTimeout ,
            sys::EVRInitError_VRInitError_Init_VRDashboardServiceStopped=>Self::Init_VRDashboardServiceStopped ,
            sys::EVRInitError_VRInitError_Init_VRDashboardAlreadyStarted=>Self::Init_VRDashboardAlreadyStarted ,
            sys::EVRInitError_VRInitError_Init_VRDashboardCopyFailed=>Self::Init_VRDashboardCopyFailed ,
            sys::EVRInitError_VRInitError_Init_VRDashboardTokenFailure=>Self::Init_VRDashboardTokenFailure ,
            sys::EVRInitError_VRInitError_Init_VRDashboardEnvironmentFailure=>Self::Init_VRDashboardEnvironmentFailure ,
            sys::EVRInitError_VRInitError_Init_VRDashboardPathFailure=>Self::Init_VRDashboardPathFailure ,
            sys::EVRInitError_VRInitError_Driver_Failed=>Self::Driver_Failed ,
            sys::EVRInitError_VRInitError_Driver_Unknown=>Self::Driver_Unknown ,
            sys::EVRInitError_VRInitError_Driver_HmdUnknown=>Self::Driver_HmdUnknown ,
            sys::EVRInitError_VRInitError_Driver_NotLoaded=>Self::Driver_NotLoaded ,
            sys::EVRInitError_VRInitError_Driver_RuntimeOutOfDate=>Self::Driver_RuntimeOutOfDate ,
            sys::EVRInitError_VRInitError_Driver_HmdInUse=>Self::Driver_HmdInUse ,
            sys::EVRInitError_VRInitError_Driver_NotCalibrated=>Self::Driver_NotCalibrated ,
            sys::EVRInitError_VRInitError_Driver_CalibrationInvalid=>Self::Driver_CalibrationInvalid ,
            sys::EVRInitError_VRInitError_Driver_HmdDisplayNotFound=>Self::Driver_HmdDisplayNotFound ,
            sys::EVRInitError_VRInitError_Driver_TrackedDeviceInterfaceUnknown=>Self::Driver_TrackedDeviceInterfaceUnknown ,
            sys::EVRInitError_VRInitError_Driver_HmdDriverIdOutOfBounds=>Self::Driver_HmdDriverIdOutOfBounds ,
            sys::EVRInitError_VRInitError_Driver_HmdDisplayMirrored=>Self::Driver_HmdDisplayMirrored ,
            sys::EVRInitError_VRInitError_Driver_HmdDisplayNotFoundLaptop=>Self::Driver_HmdDisplayNotFoundLaptop ,
            sys::EVRInitError_VRInitError_Driver_PeerDriverNotInstalled=>Self::Driver_PeerDriverNotInstalled ,
            sys::EVRInitError_VRInitError_Driver_WirelessHmdNotConnected=>Self::Driver_WirelessHmdNotConnected ,
            sys::EVRInitError_VRInitError_IPC_ServerInitFailed=>Self::IPC_ServerInitFailed ,
            sys::EVRInitError_VRInitError_IPC_ConnectFailed=>Self::IPC_ConnectFailed ,
            sys::EVRInitError_VRInitError_IPC_SharedStateInitFailed=>Self::IPC_SharedStateInitFailed ,
            sys::EVRInitError_VRInitError_IPC_CompositorInitFailed=>Self::IPC_CompositorInitFailed ,
            sys::EVRInitError_VRInitError_IPC_MutexInitFailed=>Self::IPC_MutexInitFailed ,
            sys::EVRInitError_VRInitError_IPC_Failed=>Self::IPC_Failed ,
            sys::EVRInitError_VRInitError_IPC_CompositorConnectFailed=>Self::IPC_CompositorConnectFailed ,
            sys::EVRInitError_VRInitError_IPC_CompositorInvalidConnectResponse=>Self::IPC_CompositorInvalidConnectResponse ,
            sys::EVRInitError_VRInitError_IPC_ConnectFailedAfterMultipleAttempts=>Self::IPC_ConnectFailedAfterMultipleAttempts ,
            sys::EVRInitError_VRInitError_IPC_ConnectFailedAfterTargetExited=>Self::IPC_ConnectFailedAfterTargetExited ,
            sys::EVRInitError_VRInitError_IPC_NamespaceUnavailable=>Self::IPC_NamespaceUnavailable ,
            sys::EVRInitError_VRInitError_Compositor_Failed=>Self::Compositor_Failed ,
            sys::EVRInitError_VRInitError_Compositor_D3D11HardwareRequired=>Self::Compositor_D3D11HardwareRequired ,
            sys::EVRInitError_VRInitError_Compositor_FirmwareRequiresUpdate=>Self::Compositor_FirmwareRequiresUpdate ,
            sys::EVRInitError_VRInitError_Compositor_OverlayInitFailed=>Self::Compositor_OverlayInitFailed ,
            sys::EVRInitError_VRInitError_Compositor_ScreenshotsInitFailed=>Self::Compositor_ScreenshotsInitFailed ,
            sys::EVRInitError_VRInitError_Compositor_UnableToCreateDevice=>Self::Compositor_UnableToCreateDevice ,
            sys::EVRInitError_VRInitError_Compositor_SharedStateIsNull=>Self::Compositor_SharedStateIsNull ,
            sys::EVRInitError_VRInitError_Compositor_NotificationManagerIsNull=>Self::Compositor_NotificationManagerIsNull ,
            sys::EVRInitError_VRInitError_Compositor_ResourceManagerClientIsNull=>Self::Compositor_ResourceManagerClientIsNull ,
            sys::EVRInitError_VRInitError_Compositor_MessageOverlaySharedStateInitFailure=>Self::Compositor_MessageOverlaySharedStateInitFailure,
            sys::EVRInitError_VRInitError_Compositor_PropertiesInterfaceIsNull=>Self::Compositor_PropertiesInterfaceIsNull ,
            sys::EVRInitError_VRInitError_Compositor_CreateFullscreenWindowFailed=>Self::Compositor_CreateFullscreenWindowFailed ,
            sys::EVRInitError_VRInitError_Compositor_SettingsInterfaceIsNull=>Self::Compositor_SettingsInterfaceIsNull ,
            sys::EVRInitError_VRInitError_Compositor_FailedToShowWindow=>Self::Compositor_FailedToShowWindow ,
            sys::EVRInitError_VRInitError_Compositor_DistortInterfaceIsNull=>Self::Compositor_DistortInterfaceIsNull ,
            sys::EVRInitError_VRInitError_Compositor_DisplayFrequencyFailure=>Self::Compositor_DisplayFrequencyFailure ,
            sys::EVRInitError_VRInitError_Compositor_RendererInitializationFailed=>Self::Compositor_RendererInitializationFailed ,
            sys::EVRInitError_VRInitError_Compositor_DXGIFactoryInterfaceIsNull=>Self::Compositor_DXGIFactoryInterfaceIsNull ,
            sys::EVRInitError_VRInitError_Compositor_DXGIFactoryCreateFailed=>Self::Compositor_DXGIFactoryCreateFailed ,
            sys::EVRInitError_VRInitError_Compositor_DXGIFactoryQueryFailed=>Self::Compositor_DXGIFactoryQueryFailed ,
            sys::EVRInitError_VRInitError_Compositor_InvalidAdapterDesktop=>Self::Compositor_InvalidAdapterDesktop ,
            sys::EVRInitError_VRInitError_Compositor_InvalidHmdAttachment=>Self::Compositor_InvalidHmdAttachment ,
            sys::EVRInitError_VRInitError_Compositor_InvalidOutputDesktop=>Self::Compositor_InvalidOutputDesktop ,
            sys::EVRInitError_VRInitError_Compositor_InvalidDeviceProvided=>Self::Compositor_InvalidDeviceProvided ,
            sys::EVRInitError_VRInitError_Compositor_D3D11RendererInitializationFailed=>Self::Compositor_D3D11RendererInitializationFailed ,
            sys::EVRInitError_VRInitError_Compositor_FailedToFindDisplayMode=>Self::Compositor_FailedToFindDisplayMode ,
            sys::EVRInitError_VRInitError_Compositor_FailedToCreateSwapChain=>Self::Compositor_FailedToCreateSwapChain ,
            sys::EVRInitError_VRInitError_Compositor_FailedToGetBackBuffer=>Self::Compositor_FailedToGetBackBuffer ,
            sys::EVRInitError_VRInitError_Compositor_FailedToCreateRenderTarget=>Self::Compositor_FailedToCreateRenderTarget ,
            sys::EVRInitError_VRInitError_Compositor_FailedToCreateDXGI2SwapChain=>Self::Compositor_FailedToCreateDXGI2SwapChain ,
            sys::EVRInitError_VRInitError_Compositor_FailedtoGetDXGI2BackBuffer=>Self::Compositor_FailedtoGetDXGI2BackBuffer ,
            sys::EVRInitError_VRInitError_Compositor_FailedToCreateDXGI2RenderTarget=>Self::Compositor_FailedToCreateDXGI2RenderTarget ,
            sys::EVRInitError_VRInitError_Compositor_FailedToGetDXGIDeviceInterface=>Self::Compositor_FailedToGetDXGIDeviceInterface ,
            sys::EVRInitError_VRInitError_Compositor_SelectDisplayMode=>Self::Compositor_SelectDisplayMode ,
            sys::EVRInitError_VRInitError_Compositor_FailedToCreateNvAPIRenderTargets=>Self::Compositor_FailedToCreateNvAPIRenderTargets ,
            sys::EVRInitError_VRInitError_Compositor_NvAPISetDisplayMode=>Self::Compositor_NvAPISetDisplayMode ,
            sys::EVRInitError_VRInitError_Compositor_FailedToCreateDirectModeDisplay=>Self::Compositor_FailedToCreateDirectModeDisplay ,
            sys::EVRInitError_VRInitError_Compositor_InvalidHmdPropertyContainer=>Self::Compositor_InvalidHmdPropertyContainer ,
            sys::EVRInitError_VRInitError_Compositor_UpdateDisplayFrequency=>Self::Compositor_UpdateDisplayFrequency ,
            sys::EVRInitError_VRInitError_Compositor_CreateRasterizerState=>Self::Compositor_CreateRasterizerState ,
            sys::EVRInitError_VRInitError_Compositor_CreateWireframeRasterizerState=>Self::Compositor_CreateWireframeRasterizerState ,
            sys::EVRInitError_VRInitError_Compositor_CreateSamplerState=>Self::Compositor_CreateSamplerState ,
            sys::EVRInitError_VRInitError_Compositor_CreateClampToBorderSamplerState=>Self::Compositor_CreateClampToBorderSamplerState ,
            sys::EVRInitError_VRInitError_Compositor_CreateAnisoSamplerState=>Self::Compositor_CreateAnisoSamplerState ,
            sys::EVRInitError_VRInitError_Compositor_CreateOverlaySamplerState=>Self::Compositor_CreateOverlaySamplerState ,
            sys::EVRInitError_VRInitError_Compositor_CreatePanoramaSamplerState=>Self::Compositor_CreatePanoramaSamplerState ,
            sys::EVRInitError_VRInitError_Compositor_CreateFontSamplerState=>Self::Compositor_CreateFontSamplerState ,
            sys::EVRInitError_VRInitError_Compositor_CreateNoBlendState=>Self::Compositor_CreateNoBlendState ,
            sys::EVRInitError_VRInitError_Compositor_CreateBlendState=>Self::Compositor_CreateBlendState ,
            sys::EVRInitError_VRInitError_Compositor_CreateAlphaBlendState=>Self::Compositor_CreateAlphaBlendState ,
            sys::EVRInitError_VRInitError_Compositor_CreateBlendStateMaskR=>Self::Compositor_CreateBlendStateMaskR ,
            sys::EVRInitError_VRInitError_Compositor_CreateBlendStateMaskG=>Self::Compositor_CreateBlendStateMaskG ,
            sys::EVRInitError_VRInitError_Compositor_CreateBlendStateMaskB=>Self::Compositor_CreateBlendStateMaskB ,
            sys::EVRInitError_VRInitError_Compositor_CreateDepthStencilState=>Self::Compositor_CreateDepthStencilState ,
            sys::EVRInitError_VRInitError_Compositor_CreateDepthStencilStateNoWrite=>Self::Compositor_CreateDepthStencilStateNoWrite ,
            sys::EVRInitError_VRInitError_Compositor_CreateDepthStencilStateNoDepth=>Self::Compositor_CreateDepthStencilStateNoDepth ,
            sys::EVRInitError_VRInitError_Compositor_CreateFlushTexture=>Self::Compositor_CreateFlushTexture ,
            sys::EVRInitError_VRInitError_Compositor_CreateDistortionSurfaces=>Self::Compositor_CreateDistortionSurfaces ,
            sys::EVRInitError_VRInitError_Compositor_CreateConstantBuffer=>Self::Compositor_CreateConstantBuffer ,
            sys::EVRInitError_VRInitError_Compositor_CreateHmdPoseConstantBuffer=>Self::Compositor_CreateHmdPoseConstantBuffer ,
            sys::EVRInitError_VRInitError_Compositor_CreateHmdPoseStagingConstantBuffer=>Self::Compositor_CreateHmdPoseStagingConstantBuffer,
            sys::EVRInitError_VRInitError_Compositor_CreateSharedFrameInfoConstantBuffer=>Self::Compositor_CreateSharedFrameInfoConstantBuffer,
            sys::EVRInitError_VRInitError_Compositor_CreateOverlayConstantBuffer=>Self::Compositor_CreateOverlayConstantBuffer ,
            sys::EVRInitError_VRInitError_Compositor_CreateSceneTextureIndexConstantBuffer=>Self::Compositor_CreateSceneTextureIndexConstantBuffer,
            sys::EVRInitError_VRInitError_Compositor_CreateReadableSceneTextureIndexConstantBuffer=>Self::Compositor_CreateReadableSceneTextureIndexConstantBuffer,
            sys::EVRInitError_VRInitError_Compositor_CreateLayerGraphicsTextureIndexConstantBuffer=>Self::Compositor_CreateLayerGraphicsTextureIndexConstantBuffer,
            sys::EVRInitError_VRInitError_Compositor_CreateLayerComputeTextureIndexConstantBuffer=>Self::Compositor_CreateLayerComputeTextureIndexConstantBuffer,
            sys::EVRInitError_VRInitError_Compositor_CreateLayerComputeSceneTextureIndexConstantBuffer=>Self::Compositor_CreateLayerComputeSceneTextureIndexConstantBuffer,
            sys::EVRInitError_VRInitError_Compositor_CreateComputeHmdPoseConstantBuffer=>Self::Compositor_CreateComputeHmdPoseConstantBuffer ,
            sys::EVRInitError_VRInitError_Compositor_CreateGeomConstantBuffer=>Self::Compositor_CreateGeomConstantBuffer ,
            sys::EVRInitError_VRInitError_Compositor_CreatePanelMaskConstantBuffer=>Self::Compositor_CreatePanelMaskConstantBuffer ,
            sys::EVRInitError_VRInitError_Compositor_CreatePixelSimUBO=>Self::Compositor_CreatePixelSimUBO ,
            sys::EVRInitError_VRInitError_Compositor_CreateMSAARenderTextures=>Self::Compositor_CreateMSAARenderTextures ,
            sys::EVRInitError_VRInitError_Compositor_CreateResolveRenderTextures=>Self::Compositor_CreateResolveRenderTextures ,
            sys::EVRInitError_VRInitError_Compositor_CreateComputeResolveRenderTextures=>Self::Compositor_CreateComputeResolveRenderTextures ,
            sys::EVRInitError_VRInitError_Compositor_CreateDriverDirectModeResolveTextures=>Self::Compositor_CreateDriverDirectModeResolveTextures ,
            sys::EVRInitError_VRInitError_Compositor_OpenDriverDirectModeResolveTextures=>Self::Compositor_OpenDriverDirectModeResolveTextures ,
            sys::EVRInitError_VRInitError_Compositor_CreateFallbackSyncTexture=>Self::Compositor_CreateFallbackSyncTexture ,
            sys::EVRInitError_VRInitError_Compositor_ShareFallbackSyncTexture=>Self::Compositor_ShareFallbackSyncTexture ,
            sys::EVRInitError_VRInitError_Compositor_CreateOverlayIndexBuffer=>Self::Compositor_CreateOverlayIndexBuffer ,
            sys::EVRInitError_VRInitError_Compositor_CreateOverlayVertexBuffer=>Self::Compositor_CreateOverlayVertexBuffer ,
            sys::EVRInitError_VRInitError_Compositor_CreateTextVertexBuffer=>Self::Compositor_CreateTextVertexBuffer ,
            sys::EVRInitError_VRInitError_Compositor_CreateTextIndexBuffer=>Self::Compositor_CreateTextIndexBuffer ,
            sys::EVRInitError_VRInitError_Compositor_CreateMirrorTextures=>Self::Compositor_CreateMirrorTextures ,
            sys::EVRInitError_VRInitError_Compositor_CreateLastFrameRenderTexture=>Self::Compositor_CreateLastFrameRenderTexture ,
            sys::EVRInitError_VRInitError_Compositor_CreateMirrorOverlay=>Self::Compositor_CreateMirrorOverlay ,
            sys::EVRInitError_VRInitError_Compositor_FailedToCreateVirtualDisplayBackbuffer=>Self::Compositor_FailedToCreateVirtualDisplayBackbuffer ,
            sys::EVRInitError_VRInitError_Compositor_DisplayModeNotSupported=>Self::Compositor_DisplayModeNotSupported ,
            sys::EVRInitError_VRInitError_Compositor_CreateOverlayInvalidCall=>Self::Compositor_CreateOverlayInvalidCall ,
            sys::EVRInitError_VRInitError_Compositor_CreateOverlayAlreadyInitialized=>Self::Compositor_CreateOverlayAlreadyInitialized ,
            sys::EVRInitError_VRInitError_Compositor_FailedToCreateMailbox=>Self::Compositor_FailedToCreateMailbox ,
            sys::EVRInitError_VRInitError_Compositor_WindowInterfaceIsNull=>Self::Compositor_WindowInterfaceIsNull ,
            sys::EVRInitError_VRInitError_Compositor_SystemLayerCreateInstance=>Self::Compositor_SystemLayerCreateInstance ,
            sys::EVRInitError_VRInitError_Compositor_SystemLayerCreateSession=>Self::Compositor_SystemLayerCreateSession ,
            sys::EVRInitError_VRInitError_Compositor_CreateInverseDistortUVs=>Self::Compositor_CreateInverseDistortUVs ,
            sys::EVRInitError_VRInitError_Compositor_CreateBackbufferDepth=>Self::Compositor_CreateBackbufferDepth ,
            sys::EVRInitError_VRInitError_Compositor_CannotDRMLeaseDisplay=>Self::Compositor_CannotDRMLeaseDisplay ,
            sys::EVRInitError_VRInitError_Compositor_CannotConnectToDisplayServer=>Self::Compositor_CannotConnectToDisplayServer ,
            sys::EVRInitError_VRInitError_Compositor_GnomeNoDRMLeasing=>Self::Compositor_GnomeNoDRMLeasing ,
            sys::EVRInitError_VRInitError_Compositor_FailedToInitializeEncoder=>Self::Compositor_FailedToInitializeEncoder ,
            sys::EVRInitError_VRInitError_Compositor_CreateBlurTexture=>Self::Compositor_CreateBlurTexture ,
            sys::EVRInitError_VRInitError_VendorSpecific_UnableToConnectToOculusRuntime=>Self::VendorSpecific_UnableToConnectToOculusRuntime ,
            sys::EVRInitError_VRInitError_VendorSpecific_WindowsNotInDevMode=>Self::VendorSpecific_WindowsNotInDevMode ,
            sys::EVRInitError_VRInitError_VendorSpecific_OculusLinkNotEnabled=>Self::VendorSpecific_OculusLinkNotEnabled ,
            sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_CantOpenDevice=>Self::VendorSpecific_HmdFound_CantOpenDevice ,
            sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UnableToRequestConfigStart=>Self::VendorSpecific_HmdFound_UnableToRequestConfigStart,
            sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_NoStoredConfig=>Self::VendorSpecific_HmdFound_NoStoredConfig ,
            sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_ConfigTooBig=>Self::VendorSpecific_HmdFound_ConfigTooBig ,
            sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_ConfigTooSmall=>Self::VendorSpecific_HmdFound_ConfigTooSmall ,
            sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UnableToInitZLib=>Self::VendorSpecific_HmdFound_UnableToInitZLib ,
            sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_CantReadFirmwareVersion=>Self::VendorSpecific_HmdFound_CantReadFirmwareVersion ,
            sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UnableToSendUserDataStart=>Self::VendorSpecific_HmdFound_UnableToSendUserDataStart ,
            sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UnableToGetUserDataStart=>Self::VendorSpecific_HmdFound_UnableToGetUserDataStart ,
            sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UnableToGetUserDataNext=>Self::VendorSpecific_HmdFound_UnableToGetUserDataNext ,
            sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UserDataAddressRange=>Self::VendorSpecific_HmdFound_UserDataAddressRange ,
            sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UserDataError=>Self::VendorSpecific_HmdFound_UserDataError ,
            sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_ConfigFailedSanityCheck=>Self::VendorSpecific_HmdFound_ConfigFailedSanityCheck ,
            sys::EVRInitError_VRInitError_VendorSpecific_OculusRuntimeBadInstall=>Self::VendorSpecific_OculusRuntimeBadInstall ,
            sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UnexpectedConfiguration_1=>Self::VendorSpecific_HmdFound_UnexpectedConfiguration_1 ,
            sys::EVRInitError_VRInitError_Steam_SteamInstallationNotFound=>Self::Steam_SteamInstallationNotFound ,
            sys::EVRInitError_VRInitError_LastError=>Self::LastError ,
            _=> Self::Unknown,
        }
    }
}

impl fmt::Debug for InitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = unsafe { CStr::from_ptr(sys::VR_GetVRInitErrorAsSymbol(self.to_sys_err())) };
        f.pad(msg.to_str().unwrap_or(
            "OpenVR init error description was not valid UTF-8, error description is unavailable.",
        ))
    }
}

impl error::Error for InitError {}

impl fmt::Display for InitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = unsafe {
            CStr::from_ptr(sys::VR_GetVRInitErrorAsEnglishDescription(
                self.to_sys_err(),
            ))
        };
        let description = msg.to_str().unwrap_or(
            "OpenVR init error description was not valid UTF-8, error description is unavailable.",
        );
        f.pad(description)
    }
}
