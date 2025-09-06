#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
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
impl From<openvr_sys::EVRApplicationError> for VRApplicationError {
    fn from(val: openvr_sys::EVRApplicationError) -> Self {
        //too much typing
        unsafe { core::mem::transmute(val as i16) }
    }
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum VRInputError {
    None = openvr_sys::EVRInputError_VRInputError_None as isize,
    NameNotFound = openvr_sys::EVRInputError_VRInputError_NameNotFound as isize,
    WrongType = openvr_sys::EVRInputError_VRInputError_WrongType as isize,
    InvalidHandle = openvr_sys::EVRInputError_VRInputError_InvalidHandle as isize,
    InvalidParam = openvr_sys::EVRInputError_VRInputError_InvalidParam as isize,
    NoSteam = openvr_sys::EVRInputError_VRInputError_NoSteam as isize,
    MaxCapacityReached = openvr_sys::EVRInputError_VRInputError_MaxCapacityReached as isize,
    IPCError = openvr_sys::EVRInputError_VRInputError_IPCError as isize,
    NoActiveActionSet = openvr_sys::EVRInputError_VRInputError_NoActiveActionSet as isize,
    InvalidDevice = openvr_sys::EVRInputError_VRInputError_InvalidDevice as isize,
    InvalidSkeleton = openvr_sys::EVRInputError_VRInputError_InvalidSkeleton as isize,
    InvalidBoneCount = openvr_sys::EVRInputError_VRInputError_InvalidBoneCount as isize,
    InvalidCompressedData = openvr_sys::EVRInputError_VRInputError_InvalidCompressedData as isize,
    NoData = openvr_sys::EVRInputError_VRInputError_NoData as isize,
    BufferTooSmall = openvr_sys::EVRInputError_VRInputError_BufferTooSmall as isize,
    MismatchedActionManifest =
        openvr_sys::EVRInputError_VRInputError_MismatchedActionManifest as isize,
    MissingSkeletonData = openvr_sys::EVRInputError_VRInputError_MissingSkeletonData as isize,
    InvalidBoneIndex = openvr_sys::EVRInputError_VRInputError_InvalidBoneIndex as isize,
    InvalidPriority = openvr_sys::EVRInputError_VRInputError_InvalidPriority as isize,
    PermissionDenied = openvr_sys::EVRInputError_VRInputError_PermissionDenied as isize,
    InvalidRenderModel = openvr_sys::EVRInputError_VRInputError_InvalidRenderModel as isize,
}
impl From<openvr_sys::EVRInputError> for VRInputError {
    fn from(value: openvr_sys::EVRInputError) -> Self {
        unsafe { core::mem::transmute(value as i8) }
    }
}
