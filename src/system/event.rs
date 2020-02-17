pub struct EventInfo {
    /// The tracked device index of the event. For events that aren't connected to a tracked device this is
    /// k_unTrackedDeviceIndexInvalid
    pub tracked_device_index: TrackedDeviceIndex,

    /// The age of the event in seconds.
    pub age: f32,

    /// More information about the event.
    pub event: Event,
}

impl From<sys::VREvent_t> for EventInfo {
    #[allow(unused_unsafe)]
    fn from(x: sys::VREvent_t) -> Self {
        EventInfo {
            tracked_device_index: x.trackedDeviceIndex,
            age: x.eventAgeSeconds,
            event: Event::from_sys(x.eventType as sys::EVREventType, unsafe { &x.data }),
        }
    }
}

trait FromEventData {
    unsafe fn from_event_data(x: &sys::VREvent_Data_t) -> Self;
}

use super::*;

#[derive(Debug, Copy, Clone)]
/// Controller button events
pub struct Controller {
    pub button: u32,
}

impl FromEventData for Controller {
    unsafe fn from_event_data(x: &sys::VREvent_Data_t) -> Self {
        Controller {
            button: x.controller.button,
        }
    }
}

#[derive(Debug, Copy, Clone)]
/// Simulated mouse events in overlay space
pub struct Mouse {
    /// Absolute position in texcoords, with the origin at the bottom left.
    pub position: (f32, f32),
    /// Bitfield
    pub button: u32,
}

impl FromEventData for Mouse {
    unsafe fn from_event_data(x: &sys::VREvent_Data_t) -> Self {
        Mouse {
            position: (x.mouse.x, x.mouse.y),
            button: x.mouse.button,
        }
    }
}

#[derive(Debug, Copy, Clone)]
/// Simulated mouse wheel scroll in overlay space
///
/// Coordinates are fraction of the touchpad traversed since last scroll event.
pub struct Scroll {
    pub delta: (f32, f32),
    pub viewportscale: f32,
}

impl FromEventData for Scroll {
    unsafe fn from_event_data(x: &sys::VREvent_Data_t) -> Self {
        Scroll {
            delta: (x.scroll.xdelta, x.scroll.ydelta),
            viewportscale: x.scroll.viewportscale,
        }
    }
}

#[derive(Debug, Copy, Clone)]
/// When in mouse input mode you can receive data from the touchpad, these events are only sent if the user's finger
/// is on the touchpad (or just released from it)
pub struct TouchPadMove {
    /// if the user's finger is detected on the touch pad
    pub finger_down: bool,
    /// How long the finger has been down in seconds
    pub seconds_finger_down: f32,
    /// Starting finger position (so you can do some basic swipe stuff)
    pub first: (f32, f32),
    /// This is the raw sampled coordinate without deadzoning
    pub raw: (f32, f32),
}

impl FromEventData for TouchPadMove {
    unsafe fn from_event_data(x: &sys::VREvent_Data_t) -> Self {
        TouchPadMove {
            finger_down: x.touchPadMove.bFingerDown,
            seconds_finger_down: x.touchPadMove.flSecondsFingerDown,
            first: (x.touchPadMove.fValueXFirst, x.touchPadMove.fValueYFirst),
            raw: (x.touchPadMove.fValueXRaw, x.touchPadMove.fValueYRaw),
        }
    }
}

#[derive(Debug, Copy, Clone)]
/// notification related events. Details will still change at this point
pub struct Notification {
    pub user_value: u64,
    pub notification_id: u32,
}

#[derive(Debug, Copy, Clone)]
pub struct Process {
    pub pid: u32,
    pub old_pid: u32,
    pub forced: bool,
}

impl FromEventData for Process {
    unsafe fn from_event_data(x: &sys::VREvent_Data_t) -> Self {
        Process {
            pid: x.process.pid,
            old_pid: x.process.oldPid,
            forced: x.process.bForced,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Overlay {
    pub overlay_handle: u64,
}

impl FromEventData for Overlay {
    unsafe fn from_event_data(x: &sys::VREvent_Data_t) -> Self {
        Overlay {
            overlay_handle: x.overlay.overlayHandle,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Status {
    pub status_state: u32,
}

#[derive(Debug, Copy, Clone)]
pub struct Keyboard {
    pub new_input: [u8; 8],
    pub user_value: u64,
}

impl FromEventData for Keyboard {
    unsafe fn from_event_data(x: &sys::VREvent_Data_t) -> Self {
        let x = &*(&x.keyboard as *const _ as *const sys::VREvent_Keyboard_t_real);
        Keyboard {
            new_input: *(x.cNewInput.as_ptr() as *const _),
            user_value: x.uUserValue,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Ipd {
    pub ipd_meters: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct Chaperone {
    pub previous_universe: u64,
    pub current_universe: u64,
}

#[derive(Debug, Copy, Clone)]
pub struct Property {
    pub container: PropertyContainerHandle,
    pub property: TrackedDeviceProperty,
}

impl FromEventData for Property {
    unsafe fn from_event_data(x: &sys::VREvent_Data_t) -> Self {
        let x: &sys::VREvent_Property_t = &*(x as *const _ as *const _); // Field is missing from union
        Property {
            container: x.container,
            property: x.prop,
        }
    }
}

#[allow(non_camel_case_types, deprecated)]
#[derive(Debug, Copy, Clone)]
pub enum Event {
    TrackedDeviceActivated,
    TrackedDeviceDeactivated,
    TrackedDeviceUpdated,
    TrackedDeviceUserInteractionStarted,
    TrackedDeviceUserInteractionEnded,
    IpdChanged,
    EnterStandbyMode,
    LeaveStandbyMode,
    TrackedDeviceRoleChanged,
    WatchdogWakeUpRequested,
    LensDistortionChanged,
    PropertyChanged(Property),
    WirelessDisconnect,
    WirelessReconnect,
    ButtonPress(Controller),
    ButtonUnpress(Controller),
    ButtonTouch(Controller),
    ButtonUntouch(Controller),
    DualAnalog_Press,
    DualAnalog_Unpress,
    DualAnalog_Touch,
    DualAnalog_Untouch,
    DualAnalog_Move,
    DualAnalog_ModeSwitch1,
    DualAnalog_ModeSwitch2,
    DualAnalog_Cancel,
    MouseMove(Mouse),
    MouseButtonDown(Mouse),
    MouseButtonUp(Mouse),
    FocusEnter(Overlay),
    FocusLeave(Overlay),
    ScrollDiscrete(Scroll),
    TouchPadMove(TouchPadMove),
    /// global event
    OverlayFocusChanged(Overlay),
    ReloadOverlays,
    ScrollSmooth(Scroll),
    #[deprecated]
    InputFocusCaptured(Process),
    #[deprecated]
    InputFocusReleased(Process),
    SceneFocusLost(Process),
    SceneFocusGained(Process),
    /// The app actually drawing the scene changed (usually to or from the compositor)
    SceneApplicationChanged(Process),
    /// New app got access to draw the scene
    SceneFocusChanged(Process),
    InputFocusChanged(Process),
    SceneApplicationSecondaryRenderingStarted(Process),
    SceneApplicationUsingWrongGraphicsAdapter,
    ActionBindingReloaded,
    /// Sent to the scene application to request hiding render models temporarily
    HideRenderModels,
    /// Sent to the scene application to request restoring render model visibility
    ShowRenderModels,
    ConsoleOpened,
    ConsoleClosed,
    OverlayShown,
    OverlayHidden,
    DashboardActivated,
    DashboardDeactivated,
    /// Sent to the overlay manager - data is overlay
    DashboardRequested,
    /// Send to the overlay manager
    ResetDashboard,
    /// Send to the dashboard to render a toast - data is the notification ID
    RenderToast,
    /// Sent to overlays when a SetOverlayRaw or SetOverlayFromFile call finishes loading
    ImageLoaded,
    /// Sent to keyboard renderer in the dashboard to invoke it
    ShowKeyboard,
    /// Sent to keyboard renderer in the dashboard to hide it
    HideKeyboard,
    /// Sent to an overlay when IVROverlay::SetFocusOverlay is called on it
    OverlayGamepadFocusGained,
    /// Send to an overlay when it previously had focus and IVROverlay::SetFocusOverlay is called on something else
    OverlayGamepadFocusLost,
    OverlaySharedTextureChanged,
    /// Screenshot button combo was pressed, Dashboard should request a screenshot
    ScreenshotTriggered,
    /// Sent to overlays when a SetOverlayRaw or SetOverlayfromFail fails to load
    ImageFailed,
    DashboardOverlayCreated,
    SwitchGamepadFocus,
    /// Sent by vrclient application to compositor to take a screenshot
    RequestScreenshot,
    /// Sent by compositor to the application that the screenshot has been taken
    ScreenshotTaken,
    /// Sent by compositor to the application that the screenshot failed to be taken
    ScreenshotFailed,
    /// Sent by compositor to the dashboard that a completed screenshot was submitted
    SubmitScreenshotToDashboard,
    /// Sent by compositor to the dashboard that a completed screenshot was submitted
    ScreenshotProgressToDashboard,
    PrimaryDashboardDeviceChanged,
    RoomViewShown,
    RoomViewHidden,
    ShowUI,
    ShowDevTools,
    Notification_Shown,
    Notification_Hidden,
    Notification_BeginInteraction,
    Notification_Destroyed,
    /// The application has been asked to quit
    Quit(Process),
    ProcessQuit(Process),
    QuitAborted_UserPrompt(Process),
    QuitAcknowledged(Process),
    /// The driver has requested that SteamVR shut down
    DriverRequestedQuit,
    RestartRequested,
    ChaperoneDataHasChanged,
    ChaperoneUniverseHasChanged,
    ChaperoneTempDataHasChanged,
    ChaperoneSettingsHaveChanged,
    SeatedZeroPoseReset,
    ChaperoneFlushCache,
    ChaperoneRoomSetupStarting,
    ChaperoneRoomSetupFinished,
    AudioSettingsHaveChanged,
    BackgroundSettingHasChanged,
    CameraSettingsHaveChanged,
    ReprojectionSettingHasChanged,
    ModelSkinSettingsHaveChanged,
    EnvironmentSettingsHaveChanged,
    EnableHomeAppSettingsHaveChanged,
    PowerSettingsHaveChanged,
    SteamVRSectionSettingChanged,
    LighthouseSectionSettingChanged,
    NullSectionSettingChanged,
    UserInterfaceSectionSettingChanged,
    NotificationsSectionSettingChanged,
    KeyboardSectionSettingChanged,
    PerfSectionSettingChanged,
    DashboardSectionSettingChanged,
    WebInterfaceSectionSettingChanged,
    TrackersSectionSettingChanged,
    LastKnownSectionSettingChanged,
    DismissedWarningsSectionSettingChanged,
    StatusUpdate,
    WebInterface_InstallDriverCompleted,
    MCImageUpdated,
    FirmwareUpdateStarted,
    FirmwareUpdateFinished,
    KeyboardClosed,
    KeyboardCharInput(Keyboard),
    /// Sent when DONE button clicked on keyboard
    KeyboardDone,
    ApplicationTransitionStarted,
    ApplicationTransitionAborted,
    ApplicationTransitionNewAppStarted,
    ApplicationListUpdated,
    ApplicationMimeTypeLoad,
    ApplicationTransitionNewAppLaunchComplete,
    ProcessConnected,
    ProcessDisconnected,
    Compositor_MirrorWindowShown,
    Compositor_MirrorWindowHidden,
    Compositor_ChaperoneBoundsShown,
    Compositor_ChaperoneBoundsHidden,
    Compositor_DisplayDisconnected,
    Compositor_DisplayReconnected,
    Compositor_HDCPError,
    Compositor_ApplicationNotResponding,
    Compositor_ApplicationResumed,
    Compositor_OutOfVideoMemory,
    TrackedCamera_StartVideoStream,
    TrackedCamera_StopVideoStream,
    TrackedCamera_PauseVideoStream,
    TrackedCamera_ResumeVideoStream,
    TrackedCamera_EditingSurface,
    PerformanceTest_EnableCapture,
    PerformanceTest_DisableCapture,
    PerformanceTest_FidelityLevel,
    MessageOverlay_Closed,
    MessageOverlayCloseRequested,
    Input_HapticVibration,
    Input_BindingLoadFailed,
    Input_BindingLoadSuccessful,
    Input_ActionManifestReloaded,
    Input_ActionManifestLoadFailed,
    Input_ProgressUpdate,
    Input_TrackerActivated,
    Input_BindingsUpdated,
    SpatialAnchors_PoseUpdated,
    SpatialAnchors_DescriptorUpdated,
    SpatialAnchors_RequestPoseUpdate,
    SpatialAnchors_RequestDescriptorUpdate,
    SystemReport_Started,

    VendorSpecific(sys::EVREventType),
    Unknown(sys::EVREventType),
}

impl Event {
    fn from_sys(ty: sys::EVREventType, data: &sys::VREvent_Data_t) -> Self {
        use self::Event::*;

        fn get<T: FromEventData>(x: &sys::VREvent_Data_t) -> T {
            unsafe { T::from_event_data(x) }
        }

        #[allow(deprecated)]
        match ty {
            sys::EVREventType_VREvent_TrackedDeviceActivated => TrackedDeviceActivated,
            sys::EVREventType_VREvent_TrackedDeviceDeactivated => TrackedDeviceDeactivated,
            sys::EVREventType_VREvent_TrackedDeviceUpdated => TrackedDeviceUpdated,
            sys::EVREventType_VREvent_TrackedDeviceUserInteractionStarted => {
                TrackedDeviceUserInteractionStarted
            }
            sys::EVREventType_VREvent_TrackedDeviceUserInteractionEnded => {
                TrackedDeviceUserInteractionEnded
            }
            sys::EVREventType_VREvent_IpdChanged => IpdChanged,
            sys::EVREventType_VREvent_EnterStandbyMode => EnterStandbyMode,
            sys::EVREventType_VREvent_LeaveStandbyMode => LeaveStandbyMode,
            sys::EVREventType_VREvent_TrackedDeviceRoleChanged => TrackedDeviceRoleChanged,
            sys::EVREventType_VREvent_WatchdogWakeUpRequested => WatchdogWakeUpRequested,
            sys::EVREventType_VREvent_LensDistortionChanged => LensDistortionChanged,
            sys::EVREventType_VREvent_PropertyChanged => PropertyChanged(get(data)),
            sys::EVREventType_VREvent_WirelessDisconnect => WirelessDisconnect,
            sys::EVREventType_VREvent_WirelessReconnect => WirelessReconnect,
            sys::EVREventType_VREvent_ButtonPress => ButtonPress(get(data)),
            sys::EVREventType_VREvent_ButtonUnpress => ButtonUnpress(get(data)),
            sys::EVREventType_VREvent_ButtonTouch => ButtonTouch(get(data)),
            sys::EVREventType_VREvent_ButtonUntouch => ButtonUntouch(get(data)),
            sys::EVREventType_VREvent_DualAnalog_Press => DualAnalog_Press,
            sys::EVREventType_VREvent_DualAnalog_Unpress => DualAnalog_Unpress,
            sys::EVREventType_VREvent_DualAnalog_Touch => DualAnalog_Touch,
            sys::EVREventType_VREvent_DualAnalog_Untouch => DualAnalog_Untouch,
            sys::EVREventType_VREvent_DualAnalog_Move => DualAnalog_Move,
            sys::EVREventType_VREvent_DualAnalog_ModeSwitch1 => DualAnalog_ModeSwitch1,
            sys::EVREventType_VREvent_DualAnalog_ModeSwitch2 => DualAnalog_ModeSwitch2,
            sys::EVREventType_VREvent_DualAnalog_Cancel => DualAnalog_Cancel,
            sys::EVREventType_VREvent_MouseMove => MouseMove(get(data)),
            sys::EVREventType_VREvent_MouseButtonDown => MouseButtonDown(get(data)),
            sys::EVREventType_VREvent_MouseButtonUp => MouseButtonUp(get(data)),
            sys::EVREventType_VREvent_FocusEnter => FocusEnter(get(data)),
            sys::EVREventType_VREvent_FocusLeave => FocusLeave(get(data)),
            sys::EVREventType_VREvent_ScrollDiscrete => ScrollDiscrete(get(data)),
            sys::EVREventType_VREvent_TouchPadMove => TouchPadMove(get(data)),
            sys::EVREventType_VREvent_OverlayFocusChanged => OverlayFocusChanged(get(data)),
            sys::EVREventType_VREvent_ReloadOverlays => ReloadOverlays,
            sys::EVREventType_VREvent_ScrollSmooth => ScrollSmooth(get(data)),
            sys::EVREventType_VREvent_InputFocusCaptured => InputFocusCaptured(get(data)),
            sys::EVREventType_VREvent_InputFocusReleased => InputFocusReleased(get(data)),
            sys::EVREventType_VREvent_SceneFocusLost => SceneFocusLost(get(data)),
            sys::EVREventType_VREvent_SceneFocusGained => SceneFocusGained(get(data)),
            sys::EVREventType_VREvent_SceneApplicationChanged => SceneApplicationChanged(get(data)),
            sys::EVREventType_VREvent_SceneFocusChanged => SceneFocusChanged(get(data)),
            sys::EVREventType_VREvent_InputFocusChanged => InputFocusChanged(get(data)),
            sys::EVREventType_VREvent_SceneApplicationSecondaryRenderingStarted => {
                SceneApplicationSecondaryRenderingStarted(get(data))
            }
            sys::EVREventType_VREvent_SceneApplicationUsingWrongGraphicsAdapter => {
                SceneApplicationUsingWrongGraphicsAdapter
            }
            sys::EVREventType_VREvent_ActionBindingReloaded => ActionBindingReloaded,
            sys::EVREventType_VREvent_HideRenderModels => HideRenderModels,
            sys::EVREventType_VREvent_ShowRenderModels => ShowRenderModels,
            sys::EVREventType_VREvent_ConsoleOpened => ConsoleOpened,
            sys::EVREventType_VREvent_ConsoleClosed => ConsoleClosed,
            sys::EVREventType_VREvent_OverlayShown => OverlayShown,
            sys::EVREventType_VREvent_OverlayHidden => OverlayHidden,
            sys::EVREventType_VREvent_DashboardActivated => DashboardActivated,
            sys::EVREventType_VREvent_DashboardDeactivated => DashboardDeactivated,
            sys::EVREventType_VREvent_DashboardRequested => DashboardRequested,
            sys::EVREventType_VREvent_ResetDashboard => ResetDashboard,
            sys::EVREventType_VREvent_RenderToast => RenderToast,
            sys::EVREventType_VREvent_ImageLoaded => ImageLoaded,
            sys::EVREventType_VREvent_ShowKeyboard => ShowKeyboard,
            sys::EVREventType_VREvent_HideKeyboard => HideKeyboard,
            sys::EVREventType_VREvent_OverlayGamepadFocusGained => OverlayGamepadFocusGained,
            sys::EVREventType_VREvent_OverlayGamepadFocusLost => OverlayGamepadFocusLost,
            sys::EVREventType_VREvent_OverlaySharedTextureChanged => OverlaySharedTextureChanged,
            sys::EVREventType_VREvent_ScreenshotTriggered => ScreenshotTriggered,
            sys::EVREventType_VREvent_ImageFailed => ImageFailed,
            sys::EVREventType_VREvent_DashboardOverlayCreated => DashboardOverlayCreated,
            sys::EVREventType_VREvent_SwitchGamepadFocus => SwitchGamepadFocus,
            sys::EVREventType_VREvent_RequestScreenshot => RequestScreenshot,
            sys::EVREventType_VREvent_ScreenshotTaken => ScreenshotTaken,
            sys::EVREventType_VREvent_ScreenshotFailed => ScreenshotFailed,
            sys::EVREventType_VREvent_SubmitScreenshotToDashboard => SubmitScreenshotToDashboard,
            sys::EVREventType_VREvent_ScreenshotProgressToDashboard => {
                ScreenshotProgressToDashboard
            }
            sys::EVREventType_VREvent_PrimaryDashboardDeviceChanged => {
                PrimaryDashboardDeviceChanged
            }
            sys::EVREventType_VREvent_RoomViewShown => RoomViewShown,
            sys::EVREventType_VREvent_RoomViewHidden => RoomViewHidden,
            sys::EVREventType_VREvent_ShowUI => ShowUI,
            sys::EVREventType_VREvent_ShowDevTools => ShowDevTools,
            sys::EVREventType_VREvent_Notification_Shown => Notification_Shown,
            sys::EVREventType_VREvent_Notification_Hidden => Notification_Hidden,
            sys::EVREventType_VREvent_Notification_BeginInteraction => {
                Notification_BeginInteraction
            }
            sys::EVREventType_VREvent_Notification_Destroyed => Notification_Destroyed,
            sys::EVREventType_VREvent_Quit => Quit(get(data)),
            sys::EVREventType_VREvent_ProcessQuit => ProcessQuit(get(data)),
            sys::EVREventType_VREvent_QuitAborted_UserPrompt => QuitAborted_UserPrompt(get(data)),
            sys::EVREventType_VREvent_QuitAcknowledged => QuitAcknowledged(get(data)),
            sys::EVREventType_VREvent_DriverRequestedQuit => DriverRequestedQuit,
            sys::EVREventType_VREvent_RestartRequested => RestartRequested,
            sys::EVREventType_VREvent_ChaperoneDataHasChanged => ChaperoneDataHasChanged,
            sys::EVREventType_VREvent_ChaperoneUniverseHasChanged => ChaperoneUniverseHasChanged,
            sys::EVREventType_VREvent_ChaperoneTempDataHasChanged => ChaperoneTempDataHasChanged,
            sys::EVREventType_VREvent_ChaperoneSettingsHaveChanged => ChaperoneSettingsHaveChanged,
            sys::EVREventType_VREvent_SeatedZeroPoseReset => SeatedZeroPoseReset,
            sys::EVREventType_VREvent_ChaperoneFlushCache => ChaperoneFlushCache,
            sys::EVREventType_VREvent_ChaperoneRoomSetupStarting => ChaperoneRoomSetupStarting,
            sys::EVREventType_VREvent_ChaperoneRoomSetupFinished => ChaperoneRoomSetupFinished,
            sys::EVREventType_VREvent_AudioSettingsHaveChanged => AudioSettingsHaveChanged,
            sys::EVREventType_VREvent_BackgroundSettingHasChanged => BackgroundSettingHasChanged,
            sys::EVREventType_VREvent_CameraSettingsHaveChanged => CameraSettingsHaveChanged,
            sys::EVREventType_VREvent_ReprojectionSettingHasChanged => {
                ReprojectionSettingHasChanged
            }
            sys::EVREventType_VREvent_ModelSkinSettingsHaveChanged => ModelSkinSettingsHaveChanged,
            sys::EVREventType_VREvent_EnvironmentSettingsHaveChanged => {
                EnvironmentSettingsHaveChanged
            }
            sys::EVREventType_VREvent_PowerSettingsHaveChanged => PowerSettingsHaveChanged,
            sys::EVREventType_VREvent_EnableHomeAppSettingsHaveChanged => {
                EnableHomeAppSettingsHaveChanged
            }
            sys::EVREventType_VREvent_SteamVRSectionSettingChanged => SteamVRSectionSettingChanged,
            sys::EVREventType_VREvent_LighthouseSectionSettingChanged => {
                LighthouseSectionSettingChanged
            }
            sys::EVREventType_VREvent_NullSectionSettingChanged => NullSectionSettingChanged,
            sys::EVREventType_VREvent_UserInterfaceSectionSettingChanged => {
                UserInterfaceSectionSettingChanged
            }
            sys::EVREventType_VREvent_NotificationsSectionSettingChanged => {
                NotificationsSectionSettingChanged
            }
            sys::EVREventType_VREvent_KeyboardSectionSettingChanged => {
                KeyboardSectionSettingChanged
            }
            sys::EVREventType_VREvent_PerfSectionSettingChanged => PerfSectionSettingChanged,
            sys::EVREventType_VREvent_DashboardSectionSettingChanged => {
                DashboardSectionSettingChanged
            }
            sys::EVREventType_VREvent_WebInterfaceSectionSettingChanged => {
                WebInterfaceSectionSettingChanged
            }
            sys::EVREventType_VREvent_TrackersSectionSettingChanged => {
                TrackersSectionSettingChanged
            }
            sys::EVREventType_VREvent_LastKnownSectionSettingChanged => {
                LastKnownSectionSettingChanged
            }
            sys::EVREventType_VREvent_DismissedWarningsSectionSettingChanged => {
                DismissedWarningsSectionSettingChanged
            }
            sys::EVREventType_VREvent_StatusUpdate => StatusUpdate,
            sys::EVREventType_VREvent_WebInterface_InstallDriverCompleted => {
                WebInterface_InstallDriverCompleted
            }
            sys::EVREventType_VREvent_MCImageUpdated => MCImageUpdated,
            sys::EVREventType_VREvent_FirmwareUpdateStarted => FirmwareUpdateStarted,
            sys::EVREventType_VREvent_FirmwareUpdateFinished => FirmwareUpdateFinished,
            sys::EVREventType_VREvent_KeyboardClosed => KeyboardClosed,
            sys::EVREventType_VREvent_KeyboardCharInput => KeyboardCharInput(get(data)),
            sys::EVREventType_VREvent_KeyboardDone => KeyboardDone,
            sys::EVREventType_VREvent_ApplicationTransitionStarted => ApplicationTransitionStarted,
            sys::EVREventType_VREvent_ApplicationTransitionAborted => ApplicationTransitionAborted,
            sys::EVREventType_VREvent_ApplicationTransitionNewAppStarted => {
                ApplicationTransitionNewAppStarted
            }
            sys::EVREventType_VREvent_ApplicationListUpdated => ApplicationListUpdated,
            sys::EVREventType_VREvent_ApplicationMimeTypeLoad => ApplicationMimeTypeLoad,
            sys::EVREventType_VREvent_ApplicationTransitionNewAppLaunchComplete => {
                ApplicationTransitionNewAppLaunchComplete
            }
            sys::EVREventType_VREvent_ProcessConnected => ProcessConnected,
            sys::EVREventType_VREvent_ProcessDisconnected => ProcessDisconnected,
            sys::EVREventType_VREvent_Compositor_MirrorWindowShown => Compositor_MirrorWindowShown,
            sys::EVREventType_VREvent_Compositor_MirrorWindowHidden => {
                Compositor_MirrorWindowHidden
            }
            sys::EVREventType_VREvent_Compositor_ChaperoneBoundsShown => {
                Compositor_ChaperoneBoundsShown
            }
            sys::EVREventType_VREvent_Compositor_ChaperoneBoundsHidden => {
                Compositor_ChaperoneBoundsHidden
            }
            sys::EVREventType_VREvent_Compositor_DisplayDisconnected => {
                Compositor_DisplayDisconnected
            }
            sys::EVREventType_VREvent_Compositor_DisplayReconnected => {
                Compositor_DisplayReconnected
            }
            sys::EVREventType_VREvent_Compositor_HDCPError => Compositor_HDCPError,
            sys::EVREventType_VREvent_Compositor_ApplicationNotResponding => {
                Compositor_ApplicationNotResponding
            }
            sys::EVREventType_VREvent_Compositor_ApplicationResumed => {
                Compositor_ApplicationResumed
            }
            sys::EVREventType_VREvent_Compositor_OutOfVideoMemory => Compositor_OutOfVideoMemory,
            sys::EVREventType_VREvent_TrackedCamera_StartVideoStream => {
                TrackedCamera_StartVideoStream
            }
            sys::EVREventType_VREvent_TrackedCamera_StopVideoStream => {
                TrackedCamera_StopVideoStream
            }
            sys::EVREventType_VREvent_TrackedCamera_PauseVideoStream => {
                TrackedCamera_PauseVideoStream
            }
            sys::EVREventType_VREvent_TrackedCamera_ResumeVideoStream => {
                TrackedCamera_ResumeVideoStream
            }
            sys::EVREventType_VREvent_TrackedCamera_EditingSurface => TrackedCamera_EditingSurface,
            sys::EVREventType_VREvent_PerformanceTest_EnableCapture => {
                PerformanceTest_EnableCapture
            }
            sys::EVREventType_VREvent_PerformanceTest_DisableCapture => {
                PerformanceTest_DisableCapture
            }
            sys::EVREventType_VREvent_PerformanceTest_FidelityLevel => {
                PerformanceTest_FidelityLevel
            }
            sys::EVREventType_VREvent_MessageOverlay_Closed => MessageOverlay_Closed,
            sys::EVREventType_VREvent_MessageOverlayCloseRequested => MessageOverlayCloseRequested,
            sys::EVREventType_VREvent_Input_HapticVibration => Input_HapticVibration,
            sys::EVREventType_VREvent_Input_BindingLoadFailed => Input_BindingLoadFailed,
            sys::EVREventType_VREvent_Input_BindingLoadSuccessful => Input_BindingLoadSuccessful,
            sys::EVREventType_VREvent_Input_ActionManifestReloaded => Input_ActionManifestReloaded,
            sys::EVREventType_VREvent_Input_ActionManifestLoadFailed => {
                Input_ActionManifestLoadFailed
            }
            sys::EVREventType_VREvent_Input_ProgressUpdate => Input_ProgressUpdate,
            sys::EVREventType_VREvent_Input_TrackerActivated => Input_TrackerActivated,
            sys::EVREventType_VREvent_Input_BindingsUpdated => Input_BindingsUpdated,
            sys::EVREventType_VREvent_SpatialAnchors_PoseUpdated => SpatialAnchors_PoseUpdated,
            sys::EVREventType_VREvent_SpatialAnchors_DescriptorUpdated => {
                SpatialAnchors_DescriptorUpdated
            }
            sys::EVREventType_VREvent_SpatialAnchors_RequestPoseUpdate => {
                SpatialAnchors_RequestPoseUpdate
            }
            sys::EVREventType_VREvent_SpatialAnchors_RequestDescriptorUpdate => {
                SpatialAnchors_RequestDescriptorUpdate
            }
            sys::EVREventType_VREvent_SystemReport_Started => SystemReport_Started,
            x if x >= sys::EVREventType_VREvent_VendorSpecific_Reserved_Start
                && x <= sys::EVREventType_VREvent_VendorSpecific_Reserved_End =>
            {
                VendorSpecific(x)
            }
            x => Unknown(x),
        }
    }
}

pub use sys::PropertyContainerHandle_t as PropertyContainerHandle;
