//! The `System` interface provides access to display configuration information, tracking data, controller state,
//! events, and device properties. It is the main interface of OpenVR.

use std::mem;

use openvr_sys as sys;

use super::*;

impl<'a> System<'a> {
    /// Provides the game with the minimum size that it should use for its offscreen render target to minimize pixel
    /// stretching. This size is matched with the projection matrix and distortion function and will change from display
    /// to display depending on resolution, distortion, and field of view.
    pub fn recommended_render_target_size(&self) -> (u32, u32) {
        unsafe {
            let mut result: (u32, u32) = mem::uninitialized();
            (self.0.GetRecommendedRenderTargetSize.unwrap())(&mut result.0, &mut result.1);
            result
        }
    }

    /// Returns the projection matrix to use for the specified eye.
    ///
    /// Clip plane distances are in meters.
    pub fn projection_matrix(&self, eye: Eye, near_z: f32, far_z: f32) -> [[f32; 4]; 4] {
        unsafe { (self.0.GetProjectionMatrix.unwrap())(eye as sys::EVREye, near_z, far_z) }.m
    }

    /// Returns the raw project values to use for the specified eye. Most games should use GetProjectionMatrix instead
    /// of this method, but sometimes a game needs to do something fancy with its projection and can use these values to
    /// compute its own matrix.
    pub fn projection_raw(&self, eye: Eye) -> RawProjection {
        unsafe {
            let mut result: RawProjection = mem::uninitialized();
            (self.0.GetProjectionRaw.unwrap())(eye as sys::EVREye, &mut result.left, &mut result.right, &mut result.top, &mut result.bottom);
            result
        }
    }

    /// Returns the transform between the view space and eye space. Eye space is the per-eye flavor of view space that
    /// provides stereo disparity. Instead of Model * View * Projection the model is Model * View * Eye *
    /// Projection. Normally View and Eye will be multiplied together and treated as View in your application.
    pub fn eye_to_head_transform(&self, eye: Eye) -> [[f32; 4]; 3] {
        unsafe { (self.0.GetEyeToHeadTransform.unwrap())(eye as sys::EVREye) }.m
    }

    /// Returns the number of elapsed seconds since the last recorded vsync event and the global number of frames that
    /// have been rendered. Timing information will come from a vsync timer event in the timer if possible or from the
    /// application-reported time if that is not available. If no vsync times are available the function will return
    /// None.
    pub fn time_since_last_vsync(&self) -> Option<(f32, u64)> {
        unsafe {
            let mut result: (f32, u64) = mem::uninitialized();
            if (self.0.GetTimeSinceLastVsync.unwrap())(&mut result.0, &mut result.1) {
                Some(result)
            } else {
                None
            }
        }
    }

    /// Calculates updated poses for all devices.
    ///
    /// The pose that the tracker thinks that the HMD will be in at the specified number of seconds into the
    /// future. Pass 0 to get the state at the instant the method is called. Most of the time the application should
    /// calculate the time until the photons will be emitted from the display and pass that time into the method.
    ///
    /// This is roughly analogous to the inverse of the view matrix in most applications, though many games will need to
    /// do some additional rotation or translation on top of the rotation and translation provided by the head pose.
    ///
    /// Seated experiences should call this method with TrackingUniverseSeated and receive poses relative to the seated
    /// zero pose. Standing experiences should call this method with TrackingUniverseStanding and receive poses relative
    /// to the chaperone soft bounds. TrackingUniverseRawAndUncalibrated should probably not be used unless the
    /// application is the chaperone calibration tool itself, but will provide poses relative to the hardware-specific
    /// coordinate system in the driver.
    pub fn device_to_absolute_tracking_pose(&self, origin: TrackingUniverseOrigin, predicted_seconds_to_photons_from_now: f32) -> TrackedDevicePoses {
        unsafe {
            let mut result: TrackedDevicePoses = mem::uninitialized();
            (self.0.GetDeviceToAbsoluteTrackingPose.unwrap())(origin as sys::ETrackingUniverseOrigin, predicted_seconds_to_photons_from_now,
                                                              result.data.as_mut().as_mut_ptr() as *mut _, result.data.len() as u32);
            result
        }
    }

    pub fn tracked_device_class(&self, index: TrackedDeviceIndex) -> TrackedDeviceClass {
        use self::TrackedDeviceClass::*;
        match unsafe { (self.0.GetTrackedDeviceClass.unwrap())(index) } {
            sys::ETrackedDeviceClass_ETrackedDeviceClass_TrackedDeviceClass_Invalid => Invalid,
            sys::ETrackedDeviceClass_ETrackedDeviceClass_TrackedDeviceClass_HMD => HMD,
            sys::ETrackedDeviceClass_ETrackedDeviceClass_TrackedDeviceClass_Controller => Controller,
            sys::ETrackedDeviceClass_ETrackedDeviceClass_TrackedDeviceClass_GenericTracker => GenericTracker,
            sys::ETrackedDeviceClass_ETrackedDeviceClass_TrackedDeviceClass_TrackingReference => TrackingReference,
            sys::ETrackedDeviceClass_ETrackedDeviceClass_TrackedDeviceClass_DisplayRedirect => DisplayRedirect,
            _ => Invalid,
        }
    }

    pub fn is_tracked_device_connected(&self, index: TrackedDeviceIndex) -> bool {
        unsafe { (self.0.IsTrackedDeviceConnected.unwrap())(index) }
    }

    pub fn poll_next_event_with_pose(&self, origin: TrackingUniverseOrigin) -> Option<(EventInfo, TrackedDevicePose)> {
        let mut event = unsafe { mem::uninitialized() };
        let mut pose = unsafe { mem::uninitialized() };
        if unsafe { self.0.PollNextEventWithPose.unwrap()(origin as sys::ETrackingUniverseOrigin,
                                                          &mut event, mem::size_of_val(&event) as u32,
                                                          &mut pose as *mut _ as *mut _) }
        {
            Some((EventInfo {
                tracked_device_index: event.trackedDeviceIndex,
                age: event.eventAgeSeconds,
                event: Event::from_sys(event.eventType, &event.data)
            }, pose))
        } else {
            None
        }
    }
}

/// Values represent the tangents of the half-angles from the center view axis
#[derive(Debug, Copy, Clone)]
pub struct RawProjection {
    /// tangent of the half-angle from center axis to the left clipping plane
    pub left: f32,
    /// tangent of the half-angle from center axis to the right clipping plane
    pub right: f32,
    /// tangent of the half-angle from center axis to the top clipping plane
    pub top: f32,
    /// tangent of the half-angle from center axis to the bottom clipping plane
    pub bottom: f32,
}

pub struct EventInfo {
    /// The tracked device index of the event. For events that aren't connected to a tracked device this is
    /// k_unTrackedDeviceIndexInvalid
    pub tracked_device_index: TrackedDeviceIndex,

    /// The age of the event in seconds.
    pub age: f32,

    /// More information about the event.
    pub event: Event,
}

pub trait FromEventData {
    unsafe fn from_event_data(x: &sys::VREvent_Data_t) -> Self;
}

pub mod event {
    use super::*;

    #[derive(Debug, Copy, Clone)]
    /// Controller button events
    pub struct Controller {
        pub button: u32,
    }

    impl FromEventData for Controller {
        unsafe fn from_event_data(x: &sys::VREvent_Data_t) -> Self {
            let x = x.controller.as_ref();
            Controller { button: x.button }
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
            let x = x.mouse.as_ref();
            Mouse { position: (x.x, x.y), button: x.button }
        }
    }


    #[derive(Debug, Copy, Clone)]
    /// Simulated mouse wheel scroll in overlay space
    ///
    /// Coordinates are fraction of the touchpad traversed since last scroll event.
    pub struct Scroll {
        pub delta: (f32, f32),
        pub repeat_count: u32,
    }

    impl FromEventData for Scroll {
        unsafe fn from_event_data(x: &sys::VREvent_Data_t) -> Self {
            let x = x.scroll.as_ref();
            Scroll { delta: (x.xdelta, x.ydelta), repeat_count: x.repeatCount }
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
            let x = x.touchPadMove.as_ref();
            TouchPadMove { finger_down: x.bFingerDown, seconds_finger_down: x.flSecondsFingerDown,
                           first: (x.fValueXFirst, x.fValueYFirst),
                           raw: (x.fValueXRaw, x.fValueYRaw) }
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
            let x = x.process.as_ref();
            Process { pid: x.pid, old_pid: x.oldPid, forced: x.bForced }
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub struct Overlay {
        pub overlay_handle: u64,
    }

    impl FromEventData for Overlay {
        unsafe fn from_event_data(x: &sys::VREvent_Data_t) -> Self {
            let x = x.overlay.as_ref();
            Overlay { overlay_handle: x.overlayHandle }
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
            let x = &*(x.keyboard.as_ref() as *const _ as *const sys::VREvent_Keyboard_t_real);
            Keyboard { new_input: *(x.cNewInput.as_ptr() as *const _), user_value: x.uUserValue }
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
}

#[allow(non_camel_case_types)]
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
    PropertyChanged(event::Property),

    ButtonPress(event::Controller),
    ButtonUnpress(event::Controller),
    ButtonTouch(event::Controller),
    ButtonUntouch(event::Controller),

    MouseMove(event::Mouse),
    MouseButtonDown(event::Mouse),
    MouseButtonUp(event::Mouse),
    FocusEnter(event::Overlay),
    FocusLeave(event::Overlay),
    Scroll(event::Scroll),
    TouchPadMove(event::TouchPadMove),
    /// global event
    OverlayFocusChanged(event::Overlay),

    #[deprecated]
    InputFocusCaptured(event::Process),
    #[deprecated]
    InputFocusReleased(event::Process),
    SceneFocusLost(event::Process),
    SceneFocusGained(event::Process),
    /// The app actually drawing the scene changed (usually to or from the compositor)
    SceneApplicationChanged(event::Process),
    /// New app got access to draw the scene
    SceneFocusChanged(event::Process),
    InputFocusChanged(event::Process),
    SceneApplicationSecondaryRenderingStarted(event::Process),

    /// Sent to the scene application to request hiding render models temporarily
    HideRenderModels,
    /// Sent to the scene application to request restoring render model visibility
    ShowRenderModels,

    OverlayShown,
    OverlayHidden,
    DashboardActivated,
    DashboardDeactivated,
    /// Sent to the overlay manager - data is overlay
    DashboardThumbSelected,
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
    DashboardGuideButtonDown,
    DashboardGuideButtonUp,
    /// Screenshot button combo was pressed, Dashboard should request a screenshot
    ScreenshotTriggered,
    /// Sent to overlays when a SetOverlayRaw or SetOverlayfromFail fails to load
    ImageFailed,
    DashboardOverlayCreated,

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

    Notification_Shown,
    Notification_Hidden,
    Notification_BeginInteraction,
    Notification_Destroyed,

    Quit(event::Process),
    ProcessQuit(event::Process),
    QuitAborted_UserPrompt(event::Process),
    QuitAcknowledged(event::Process),
    /// The driver has requested that SteamVR shut down
    DriverRequestedQuit,

    ChaperoneDataHasChanged,
    ChaperoneUniverseHasChanged,
    ChaperoneTempDataHasChanged,
    ChaperoneSettingsHaveChanged,
    SeatedZeroPoseReset,

    AudioSettingsHaveChanged,

    BackgroundSettingHasChanged,
    CameraSettingsHaveChanged,
    ReprojectionSettingHasChanged,
    ModelSkinSettingsHaveChanged,
    EnvironmentSettingsHaveChanged,
    PowerSettingsHaveChanged,

    StatusUpdate,

    MCImageUpdated,

    FirmwareUpdateStarted,
    FirmwareUpdateFinished,

    KeyboardClosed,
    KeyboardCharInput(event::Keyboard),
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

    TrackedCamera_StartVideoStream,
    TrackedCamera_StopVideoStream,
    TrackedCamera_PauseVideoStream,
    TrackedCamera_ResumeVideoStream,
    TrackedCamera_EditingSurface,

    PerformanceTest_EnableCapture,
    PerformanceTest_DisableCapture,
    PerformanceTest_FidelityLevel,

    MessageOverlay_Closed,

    VendorSpecific(u32),
    Unknown(u32),
}

impl Event {
    fn from_sys(ty: u32, data: &sys::VREvent_Data_t) -> Self {
        use self::Event::*;

        fn get<T: FromEventData>(x: &sys::VREvent_Data_t) -> T {
            unsafe { T::from_event_data(x) }
        }

        #[allow(deprecated)]
        match ty {
            sys::EVREventType_EVREventType_VREvent_TrackedDeviceActivated => TrackedDeviceActivated,
            sys::EVREventType_EVREventType_VREvent_TrackedDeviceDeactivated => TrackedDeviceDeactivated,
            sys::EVREventType_EVREventType_VREvent_TrackedDeviceUpdated => TrackedDeviceUpdated,
            sys::EVREventType_EVREventType_VREvent_TrackedDeviceUserInteractionStarted => TrackedDeviceUserInteractionStarted,
            sys::EVREventType_EVREventType_VREvent_TrackedDeviceUserInteractionEnded => TrackedDeviceUserInteractionEnded,
            sys::EVREventType_EVREventType_VREvent_IpdChanged => IpdChanged,
            sys::EVREventType_EVREventType_VREvent_EnterStandbyMode => EnterStandbyMode,
            sys::EVREventType_EVREventType_VREvent_LeaveStandbyMode => LeaveStandbyMode,
            sys::EVREventType_EVREventType_VREvent_TrackedDeviceRoleChanged => TrackedDeviceRoleChanged,
            sys::EVREventType_EVREventType_VREvent_WatchdogWakeUpRequested => WatchdogWakeUpRequested,
            sys::EVREventType_EVREventType_VREvent_LensDistortionChanged => LensDistortionChanged,
            sys::EVREventType_EVREventType_VREvent_PropertyChanged => PropertyChanged(get(data)),
            sys::EVREventType_EVREventType_VREvent_ButtonPress => ButtonPress(get(data)),
            sys::EVREventType_EVREventType_VREvent_ButtonUnpress => ButtonUnpress(get(data)),
            sys::EVREventType_EVREventType_VREvent_ButtonTouch => ButtonTouch(get(data)),
            sys::EVREventType_EVREventType_VREvent_ButtonUntouch => ButtonUntouch(get(data)),
            sys::EVREventType_EVREventType_VREvent_MouseMove => MouseMove(get(data)),
            sys::EVREventType_EVREventType_VREvent_MouseButtonDown => MouseButtonDown(get(data)),
            sys::EVREventType_EVREventType_VREvent_MouseButtonUp => MouseButtonUp(get(data)),
            sys::EVREventType_EVREventType_VREvent_FocusEnter => FocusEnter(get(data)),
            sys::EVREventType_EVREventType_VREvent_FocusLeave => FocusLeave(get(data)),
            sys::EVREventType_EVREventType_VREvent_Scroll => Scroll(get(data)),
            sys::EVREventType_EVREventType_VREvent_TouchPadMove => TouchPadMove(get(data)),
            sys::EVREventType_EVREventType_VREvent_OverlayFocusChanged => OverlayFocusChanged(get(data)),
            sys::EVREventType_EVREventType_VREvent_InputFocusCaptured => InputFocusCaptured(get(data)),
            sys::EVREventType_EVREventType_VREvent_InputFocusReleased => InputFocusReleased(get(data)),
            sys::EVREventType_EVREventType_VREvent_SceneFocusLost => SceneFocusLost(get(data)),
            sys::EVREventType_EVREventType_VREvent_SceneFocusGained => SceneFocusGained(get(data)),
            sys::EVREventType_EVREventType_VREvent_SceneApplicationChanged => SceneApplicationChanged(get(data)),
            sys::EVREventType_EVREventType_VREvent_SceneFocusChanged => SceneFocusChanged(get(data)),
            sys::EVREventType_EVREventType_VREvent_InputFocusChanged => InputFocusChanged(get(data)),
            sys::EVREventType_EVREventType_VREvent_SceneApplicationSecondaryRenderingStarted => SceneApplicationSecondaryRenderingStarted(get(data)),
            sys::EVREventType_EVREventType_VREvent_HideRenderModels => HideRenderModels,
            sys::EVREventType_EVREventType_VREvent_ShowRenderModels => ShowRenderModels,
            sys::EVREventType_EVREventType_VREvent_OverlayShown => OverlayShown,
            sys::EVREventType_EVREventType_VREvent_OverlayHidden => OverlayHidden,
            sys::EVREventType_EVREventType_VREvent_DashboardActivated => DashboardActivated,
            sys::EVREventType_EVREventType_VREvent_DashboardDeactivated => DashboardDeactivated,
            sys::EVREventType_EVREventType_VREvent_DashboardThumbSelected => DashboardThumbSelected,
            sys::EVREventType_EVREventType_VREvent_DashboardRequested => DashboardRequested,
            sys::EVREventType_EVREventType_VREvent_ResetDashboard => ResetDashboard,
            sys::EVREventType_EVREventType_VREvent_RenderToast => RenderToast,
            sys::EVREventType_EVREventType_VREvent_ImageLoaded => ImageLoaded,
            sys::EVREventType_EVREventType_VREvent_ShowKeyboard => ShowKeyboard,
            sys::EVREventType_EVREventType_VREvent_HideKeyboard => HideKeyboard,
            sys::EVREventType_EVREventType_VREvent_OverlayGamepadFocusGained => OverlayGamepadFocusGained,
            sys::EVREventType_EVREventType_VREvent_OverlayGamepadFocusLost => OverlayGamepadFocusLost,
            sys::EVREventType_EVREventType_VREvent_OverlaySharedTextureChanged => OverlaySharedTextureChanged,
            sys::EVREventType_EVREventType_VREvent_DashboardGuideButtonDown => DashboardGuideButtonDown,
            sys::EVREventType_EVREventType_VREvent_DashboardGuideButtonUp => DashboardGuideButtonUp,
            sys::EVREventType_EVREventType_VREvent_ScreenshotTriggered => ScreenshotTriggered,
            sys::EVREventType_EVREventType_VREvent_ImageFailed => ImageFailed,
            sys::EVREventType_EVREventType_VREvent_DashboardOverlayCreated => DashboardOverlayCreated,
            sys::EVREventType_EVREventType_VREvent_RequestScreenshot => RequestScreenshot,
            sys::EVREventType_EVREventType_VREvent_ScreenshotTaken => ScreenshotTaken,
            sys::EVREventType_EVREventType_VREvent_ScreenshotFailed => ScreenshotFailed,
            sys::EVREventType_EVREventType_VREvent_SubmitScreenshotToDashboard => SubmitScreenshotToDashboard,
            sys::EVREventType_EVREventType_VREvent_ScreenshotProgressToDashboard => ScreenshotProgressToDashboard,
            sys::EVREventType_EVREventType_VREvent_PrimaryDashboardDeviceChanged => PrimaryDashboardDeviceChanged,
            sys::EVREventType_EVREventType_VREvent_Notification_Shown => Notification_Shown,
            sys::EVREventType_EVREventType_VREvent_Notification_Hidden => Notification_Hidden,
            sys::EVREventType_EVREventType_VREvent_Notification_BeginInteraction => Notification_BeginInteraction,
            sys::EVREventType_EVREventType_VREvent_Notification_Destroyed => Notification_Destroyed,
            sys::EVREventType_EVREventType_VREvent_Quit => Quit(get(data)),
            sys::EVREventType_EVREventType_VREvent_ProcessQuit => ProcessQuit(get(data)),
            sys::EVREventType_EVREventType_VREvent_QuitAborted_UserPrompt => QuitAborted_UserPrompt(get(data)),
            sys::EVREventType_EVREventType_VREvent_QuitAcknowledged => QuitAcknowledged(get(data)),
            sys::EVREventType_EVREventType_VREvent_DriverRequestedQuit => DriverRequestedQuit,
            sys::EVREventType_EVREventType_VREvent_ChaperoneDataHasChanged => ChaperoneDataHasChanged,
            sys::EVREventType_EVREventType_VREvent_ChaperoneUniverseHasChanged => ChaperoneUniverseHasChanged,
            sys::EVREventType_EVREventType_VREvent_ChaperoneTempDataHasChanged => ChaperoneTempDataHasChanged,
            sys::EVREventType_EVREventType_VREvent_ChaperoneSettingsHaveChanged => ChaperoneSettingsHaveChanged,
            sys::EVREventType_EVREventType_VREvent_SeatedZeroPoseReset => SeatedZeroPoseReset,
            sys::EVREventType_EVREventType_VREvent_AudioSettingsHaveChanged => AudioSettingsHaveChanged,
            sys::EVREventType_EVREventType_VREvent_BackgroundSettingHasChanged => BackgroundSettingHasChanged,
            sys::EVREventType_EVREventType_VREvent_CameraSettingsHaveChanged => CameraSettingsHaveChanged,
            sys::EVREventType_EVREventType_VREvent_ReprojectionSettingHasChanged => ReprojectionSettingHasChanged,
            sys::EVREventType_EVREventType_VREvent_ModelSkinSettingsHaveChanged => ModelSkinSettingsHaveChanged,
            sys::EVREventType_EVREventType_VREvent_EnvironmentSettingsHaveChanged => EnvironmentSettingsHaveChanged,
            sys::EVREventType_EVREventType_VREvent_PowerSettingsHaveChanged => PowerSettingsHaveChanged,
            sys::EVREventType_EVREventType_VREvent_StatusUpdate => StatusUpdate,
            sys::EVREventType_EVREventType_VREvent_MCImageUpdated => MCImageUpdated,
            sys::EVREventType_EVREventType_VREvent_FirmwareUpdateStarted => FirmwareUpdateStarted,
            sys::EVREventType_EVREventType_VREvent_FirmwareUpdateFinished => FirmwareUpdateFinished,
            sys::EVREventType_EVREventType_VREvent_KeyboardClosed => KeyboardClosed,
            sys::EVREventType_EVREventType_VREvent_KeyboardCharInput => KeyboardCharInput(get(data)),
            sys::EVREventType_EVREventType_VREvent_KeyboardDone => KeyboardDone,
            sys::EVREventType_EVREventType_VREvent_ApplicationTransitionStarted => ApplicationTransitionStarted,
            sys::EVREventType_EVREventType_VREvent_ApplicationTransitionAborted => ApplicationTransitionAborted,
            sys::EVREventType_EVREventType_VREvent_ApplicationTransitionNewAppStarted => ApplicationTransitionNewAppStarted,
            sys::EVREventType_EVREventType_VREvent_ApplicationListUpdated => ApplicationListUpdated,
            sys::EVREventType_EVREventType_VREvent_ApplicationMimeTypeLoad => ApplicationMimeTypeLoad,
            sys::EVREventType_EVREventType_VREvent_ApplicationTransitionNewAppLaunchComplete => ApplicationTransitionNewAppLaunchComplete,
            sys::EVREventType_EVREventType_VREvent_ProcessConnected => ProcessConnected,
            sys::EVREventType_EVREventType_VREvent_ProcessDisconnected => ProcessDisconnected,
            sys::EVREventType_EVREventType_VREvent_Compositor_MirrorWindowShown => Compositor_MirrorWindowShown,
            sys::EVREventType_EVREventType_VREvent_Compositor_MirrorWindowHidden => Compositor_MirrorWindowHidden,
            sys::EVREventType_EVREventType_VREvent_Compositor_ChaperoneBoundsShown => Compositor_ChaperoneBoundsShown,
            sys::EVREventType_EVREventType_VREvent_Compositor_ChaperoneBoundsHidden => Compositor_ChaperoneBoundsHidden,
            sys::EVREventType_EVREventType_VREvent_TrackedCamera_StartVideoStream => TrackedCamera_StartVideoStream,
            sys::EVREventType_EVREventType_VREvent_TrackedCamera_StopVideoStream => TrackedCamera_StopVideoStream,
            sys::EVREventType_EVREventType_VREvent_TrackedCamera_PauseVideoStream => TrackedCamera_PauseVideoStream,
            sys::EVREventType_EVREventType_VREvent_TrackedCamera_ResumeVideoStream => TrackedCamera_ResumeVideoStream,
            sys::EVREventType_EVREventType_VREvent_TrackedCamera_EditingSurface => TrackedCamera_EditingSurface,
            sys::EVREventType_EVREventType_VREvent_PerformanceTest_EnableCapture => PerformanceTest_EnableCapture,
            sys::EVREventType_EVREventType_VREvent_PerformanceTest_DisableCapture => PerformanceTest_DisableCapture,
            sys::EVREventType_EVREventType_VREvent_PerformanceTest_FidelityLevel => PerformanceTest_FidelityLevel,
            sys::EVREventType_EVREventType_VREvent_MessageOverlay_Closed => MessageOverlay_Closed,
            x if x >= sys::EVREventType_EVREventType_VREvent_VendorSpecific_Reserved_Start
                && x <= sys::EVREventType_EVREventType_VREvent_VendorSpecific_Reserved_End => VendorSpecific(x),
            x => Unknown(x),
        }
    }
}

pub type PropertyContainerHandle = sys::PropertyContainerHandle_t;
