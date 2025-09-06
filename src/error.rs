pub enum VRApplicationError {
    None = openvr_sys::EVRApplicationError_VRApplicationError_None as isize,
    AppKeyAlreadyExists =
        openvr_sys::EVRApplicationError_VRApplicationError_AppKeyAlreadyExists as isize,
    NoManifest = openvr_sys::EVRApplicationError_VRApplicationError_NoManifest as isize,
    NoApplication = openvr_sys::EVRApplicationError_VRApplicationError_NoApplication as isize,
    InvalidIndex = openvr_sys::EVRApplicationError_VRApplicationError_InvalidIndex as isize,
    UnknownApplication =
        openvr_sys::EVRApplicationError_VRApplicationError_UnknownApplication as isize,
    IPCFailed = openvr_sys::EVRApplicationError_VRApplicationError_IPCFailed as isize,
    ApplicationAlreadyRunning =
        openvr_sys::EVRApplicationError_VRApplicationError_ApplicationAlreadyRunning as isize,
    InvalidManifest = openvr_sys::EVRApplicationError_VRApplicationError_InvalidManifest as isize,
    InvalidApplication =
        openvr_sys::EVRApplicationError_VRApplicationError_InvalidApplication as isize,
    LaunchFailed = openvr_sys::EVRApplicationError_VRApplicationError_LaunchFailed as isize,
    ApplicationAlreadyStarting =
        openvr_sys::EVRApplicationError_VRApplicationError_ApplicationAlreadyStarting as isize,
    LaunchInProgress = openvr_sys::EVRApplicationError_VRApplicationError_LaunchInProgress as isize,
    OldApplicationQuitting =
        openvr_sys::EVRApplicationError_VRApplicationError_OldApplicationQuitting as isize,
    TransitionAborted =
        openvr_sys::EVRApplicationError_VRApplicationError_TransitionAborted as isize,
    IsTemplate = openvr_sys::EVRApplicationError_VRApplicationError_IsTemplate as isize,
    SteamVRIsExiting = openvr_sys::EVRApplicationError_VRApplicationError_SteamVRIsExiting as isize,
    BufferTooSmall = openvr_sys::EVRApplicationError_VRApplicationError_BufferTooSmall as isize,
    PropertyNotSet = openvr_sys::EVRApplicationError_VRApplicationError_PropertyNotSet as isize,
    UnknownProperty = openvr_sys::EVRApplicationError_VRApplicationError_UnknownProperty as isize,
    InvalidParameter = openvr_sys::EVRApplicationError_VRApplicationError_InvalidParameter as isize,
    NotImplemented = openvr_sys::EVRApplicationError_VRApplicationError_NotImplemented as isize,
}
impl From<openvr_sys::EVRApplicationError> for VRApplicationError{
    fn from(val: openvr_sys::EVRApplicationError) -> Self {
        //too much typing
        unsafe {core::mem::transmute(val as i16)}
    }
}
