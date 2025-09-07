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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum VROverlayError {
    None = openvr_sys::EVROverlayError_VROverlayError_None as isize,
    UnknownOverlay = openvr_sys::EVROverlayError_VROverlayError_UnknownOverlay as isize,
    InvalidHandle = openvr_sys::EVROverlayError_VROverlayError_InvalidHandle as isize,
    PermissionDenied = openvr_sys::EVROverlayError_VROverlayError_PermissionDenied as isize,
    OverlayLimitExceeded = openvr_sys::EVROverlayError_VROverlayError_OverlayLimitExceeded as isize,
    WrongVisibilityType = openvr_sys::EVROverlayError_VROverlayError_WrongVisibilityType as isize,
    KeyTooLong = openvr_sys::EVROverlayError_VROverlayError_KeyTooLong as isize,
    NameTooLong = openvr_sys::EVROverlayError_VROverlayError_NameTooLong as isize,
    KeyInUse = openvr_sys::EVROverlayError_VROverlayError_KeyInUse as isize,
    WrongTransformType = openvr_sys::EVROverlayError_VROverlayError_WrongTransformType as isize,
    InvalidTrackedDevice = openvr_sys::EVROverlayError_VROverlayError_InvalidTrackedDevice as isize,
    InvalidParameter = openvr_sys::EVROverlayError_VROverlayError_InvalidParameter as isize,
    ThumbnailCantBeDestroyed =
        openvr_sys::EVROverlayError_VROverlayError_ThumbnailCantBeDestroyed as isize,
    ArrayTooSmall = openvr_sys::EVROverlayError_VROverlayError_ArrayTooSmall as isize,
    RequestFailed = openvr_sys::EVROverlayError_VROverlayError_RequestFailed as isize,
    InvalidTexture = openvr_sys::EVROverlayError_VROverlayError_InvalidTexture as isize,
    UnableToLoadFile = openvr_sys::EVROverlayError_VROverlayError_UnableToLoadFile as isize,
    KeyboardAlreadyInUse = openvr_sys::EVROverlayError_VROverlayError_KeyboardAlreadyInUse as isize,
    NoNeighbor = openvr_sys::EVROverlayError_VROverlayError_NoNeighbor as isize,
    TooManyMaskPrimitives =
        openvr_sys::EVROverlayError_VROverlayError_TooManyMaskPrimitives as isize,
    BadMaskPrimitive = openvr_sys::EVROverlayError_VROverlayError_BadMaskPrimitive as isize,
    TextureAlreadyLocked = openvr_sys::EVROverlayError_VROverlayError_TextureAlreadyLocked as isize,
    TextureLockCapacityReached =
        openvr_sys::EVROverlayError_VROverlayError_TextureLockCapacityReached as isize,
    TextureNotLocked = openvr_sys::EVROverlayError_VROverlayError_TextureNotLocked as isize,
    TimedOut = openvr_sys::EVROverlayError_VROverlayError_TimedOut as isize,
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
