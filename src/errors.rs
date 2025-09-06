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
impl VRApplicationError {
    pub fn new(value: openvr_sys::EVRInputError) -> Result<(), Self> {
        let err = Self::from(value);
        match err {
            VRApplicationError::None => return Ok(()),
            _ => return Err(err),
        }
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
impl VRInputError {
    pub fn new(value: openvr_sys::EVRInputError) -> Result<(), Self> {
        let err = Self::from(value);
        match err {
            VRInputError::None => return Ok(()),
            _ => return Err(err),
        }
    }
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum VRSettingsError {
    None = openvr_sys::EVRSettingsError_VRSettingsError_None as isize,
    IPCFailed = openvr_sys::EVRSettingsError_VRSettingsError_IPCFailed as isize,
    WriteFailed = openvr_sys::EVRSettingsError_VRSettingsError_WriteFailed as isize,
    ReadFailed = openvr_sys::EVRSettingsError_VRSettingsError_ReadFailed as isize,
    JsonParseFailed = openvr_sys::EVRSettingsError_VRSettingsError_JsonParseFailed as isize,
    UnsetSettingHasNoDefault =
        openvr_sys::EVRSettingsError_VRSettingsError_UnsetSettingHasNoDefault as isize,
    AccessDenied = openvr_sys::EVRSettingsError_VRSettingsError_AccessDenied as isize,
}
impl From<openvr_sys::EVRSettingsError> for VRSettingsError {
    fn from(value: openvr_sys::EVRSettingsError) -> Self {
        unsafe { core::mem::transmute(value as i8) }
    }
}
impl VRSettingsError {
    pub fn new(value: openvr_sys::EVRSettingsError) -> Result<(), Self> {
        let err = Self::from(value);
        match err {
            VRSettingsError::None => return Ok(()),
            _ => return Err(err),
        }
    }
}
// pub const EVROverlayError_VROverlayError_None = 0 as isize,
// pub const EVROverlayError_VROverlayError_UnknownOverlay = 10 as isize,
// pub const EVROverlayError_VROverlayError_InvalidHandle = 11 as isize,
// pub const EVROverlayError_VROverlayError_PermissionDenied = 12 as isize,
// pub const EVROverlayError_VROverlayError_OverlayLimitExceeded = 13 as isize,
// pub const EVROverlayError_VROverlayError_WrongVisibilityType = 14 as isize,
// pub const EVROverlayError_VROverlayError_KeyTooLong = 15 as isize,
// pub const EVROverlayError_VROverlayError_NameTooLong = 16 as isize,
// pub const EVROverlayError_VROverlayError_KeyInUse = 17 as isize,
// pub const EVROverlayError_VROverlayError_WrongTransformType = 18 as isize,
// pub const EVROverlayError_VROverlayError_InvalidTrackedDevice = 19 as isize,
// pub const EVROverlayError_VROverlayError_InvalidParameter = 20 as isize,
// pub const EVROverlayError_VROverlayError_ThumbnailCantBeDestroyed = 21 as isize,
// pub const EVROverlayError_VROverlayError_ArrayTooSmall = 22 as isize,
// pub const EVROverlayError_VROverlayError_RequestFailed = 23 as isize,
// pub const EVROverlayError_VROverlayError_InvalidTexture = 24 as isize,
// pub const EVROverlayError_VROverlayError_UnableToLoadFile = 25 as isize,
// pub const EVROverlayError_VROverlayError_KeyboardAlreadyInUse = 26 as isize,
// pub const EVROverlayError_VROverlayError_NoNeighbor = 27 as isize,
// pub const EVROverlayError_VROverlayError_TooManyMaskPrimitives = 29 as isize,
// pub const EVROverlayError_VROverlayError_BadMaskPrimitive = 30 as isize,
// pub const EVROverlayError_VROverlayError_TextureAlreadyLocked = 31 as isize,
// pub const EVROverlayError_VROverlayError_TextureLockCapacityReached = 32 as isize,
// pub const EVROverlayError_VROverlayError_TextureNotLocked = 33 as isize,
// pub const EVROverlayError_VROverlayError_TimedOut = 34 as isize,
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum VROverlayError {
    None=openvr_sys::EVROverlayError_VROverlayError_None as isize,
    UnknownOverlay = 10 as isize,
    InvalidHandle = 11 as isize,
    PermissionDenied = 12 as isize,
    OverlayLimitExceeded = 13 as isize,
    WrongVisibilityType = 14 as isize,
    KeyTooLong = 15 as isize,
    NameTooLong = 16 as isize,
    KeyInUse = 17 as isize,
    WrongTransformType = 18 as isize,
    InvalidTrackedDevice = 19 as isize,
    InvalidParameter = 20 as isize,
    ThumbnailCantBeDestroyed = 21 as isize,
    ArrayTooSmall = 22 as isize,
    RequestFailed = 23 as isize,
    InvalidTexture = 24 as isize,
    UnableToLoadFile = 25 as isize,
    KeyboardAlreadyInUse = 26 as isize,
    NoNeighbor = 27 as isize,
    TooManyMaskPrimitives = 29 as isize,
    BadMaskPrimitive = 30 as isize,
    TextureAlreadyLocked = 31 as isize,
    TextureLockCapacityReached = 32 as isize,
    TextureNotLocked = 33 as isize,
    TimedOut = 34 as isize,
    
}
impl From<openvr_sys::EVROverlayError> for VROverlayError {
    fn from(value: openvr_sys::EVROverlayError) -> Self {
        unsafe { core::mem::transmute(value as i8) }
    }
}
impl VROverlayError {
    pub fn new(value: openvr_sys::EVROverlayError) -> Result<(), Self> {
        let err = Self::from(value);
        match err {
            VROverlayError::None => return Ok(()),
            _ => return Err(err),
        }
    }
}