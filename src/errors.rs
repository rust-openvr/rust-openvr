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
        match val {
            openvr_sys::EVRApplicationError_VRApplicationError_None => Self::None,
            openvr_sys::EVRApplicationError_VRApplicationError_AppKeyAlreadyExists => {
                Self::AppKeyAlreadyExists
            }
            openvr_sys::EVRApplicationError_VRApplicationError_NoManifest => Self::NoManifest,
            openvr_sys::EVRApplicationError_VRApplicationError_NoApplication => Self::NoApplication,
            openvr_sys::EVRApplicationError_VRApplicationError_InvalidIndex => Self::InvalidIndex,
            openvr_sys::EVRApplicationError_VRApplicationError_UnknownApplication => {
                Self::UnknownApplication
            }
            openvr_sys::EVRApplicationError_VRApplicationError_IPCFailed => Self::IPCFailed,
            openvr_sys::EVRApplicationError_VRApplicationError_ApplicationAlreadyRunning => {
                Self::ApplicationAlreadyRunning
            }
            openvr_sys::EVRApplicationError_VRApplicationError_InvalidManifest => {
                Self::InvalidManifest
            }
            openvr_sys::EVRApplicationError_VRApplicationError_InvalidApplication => {
                Self::InvalidApplication
            }
            openvr_sys::EVRApplicationError_VRApplicationError_LaunchFailed => Self::LaunchFailed,
            openvr_sys::EVRApplicationError_VRApplicationError_ApplicationAlreadyStarting => {
                Self::ApplicationAlreadyRunning
            }
            openvr_sys::EVRApplicationError_VRApplicationError_LaunchInProgress => {
                Self::LaunchInProgress
            }
            openvr_sys::EVRApplicationError_VRApplicationError_OldApplicationQuitting => {
                Self::OldApplicationQuitting
            }
            openvr_sys::EVRApplicationError_VRApplicationError_TransitionAborted => {
                Self::TransitionAborted
            }
            openvr_sys::EVRApplicationError_VRApplicationError_IsTemplate => Self::IsTemplate,
            openvr_sys::EVRApplicationError_VRApplicationError_SteamVRIsExiting => {
                Self::SteamVRIsExiting
            }
            openvr_sys::EVRApplicationError_VRApplicationError_BufferTooSmall => {
                Self::BufferTooSmall
            }
            openvr_sys::EVRApplicationError_VRApplicationError_PropertyNotSet => {
                Self::PropertyNotSet
            }
            openvr_sys::EVRApplicationError_VRApplicationError_UnknownProperty => {
                Self::UnknownProperty
            }
            openvr_sys::EVRApplicationError_VRApplicationError_InvalidParameter => {
                Self::InvalidParameter
            }
            openvr_sys::EVRApplicationError_VRApplicationError_NotImplemented => {
                Self::NotImplemented
            }
            _ => unreachable!(),
        }
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
        match value {
            openvr_sys::EVRInputError_VRInputError_None => Self::None,
            openvr_sys::EVRInputError_VRInputError_NameNotFound => Self::NameNotFound,
            openvr_sys::EVRInputError_VRInputError_WrongType => Self::WrongType,
            openvr_sys::EVRInputError_VRInputError_InvalidHandle => Self::InvalidHandle,
            openvr_sys::EVRInputError_VRInputError_InvalidParam => Self::InvalidParam,
            openvr_sys::EVRInputError_VRInputError_NoSteam => Self::NoSteam,
            openvr_sys::EVRInputError_VRInputError_MaxCapacityReached => Self::MaxCapacityReached,
            openvr_sys::EVRInputError_VRInputError_IPCError => Self::IPCError,
            openvr_sys::EVRInputError_VRInputError_NoActiveActionSet => Self::NoActiveActionSet,
            openvr_sys::EVRInputError_VRInputError_InvalidDevice => Self::InvalidDevice,
            openvr_sys::EVRInputError_VRInputError_InvalidSkeleton => Self::InvalidSkeleton,
            openvr_sys::EVRInputError_VRInputError_InvalidBoneCount => Self::InvalidBoneCount,
            openvr_sys::EVRInputError_VRInputError_InvalidCompressedData => {
                Self::InvalidCompressedData
            }
            openvr_sys::EVRInputError_VRInputError_NoData => Self::NoData,
            openvr_sys::EVRInputError_VRInputError_BufferTooSmall => Self::BufferTooSmall,
            openvr_sys::EVRInputError_VRInputError_MismatchedActionManifest => {
                Self::MismatchedActionManifest
            }
            openvr_sys::EVRInputError_VRInputError_MissingSkeletonData => Self::MissingSkeletonData,
            openvr_sys::EVRInputError_VRInputError_InvalidBoneIndex => Self::InvalidBoneIndex,
            openvr_sys::EVRInputError_VRInputError_InvalidPriority => Self::InvalidPriority,
            openvr_sys::EVRInputError_VRInputError_PermissionDenied => Self::PermissionDenied,
            openvr_sys::EVRInputError_VRInputError_InvalidRenderModel => Self::InvalidRenderModel,
            _ => unreachable!(),
        }
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
        match value {
            openvr_sys::EVRSettingsError_VRSettingsError_None => Self::None,
            openvr_sys::EVRSettingsError_VRSettingsError_IPCFailed => Self::IPCFailed,
            openvr_sys::EVRSettingsError_VRSettingsError_WriteFailed => Self::WriteFailed,
            openvr_sys::EVRSettingsError_VRSettingsError_ReadFailed => Self::ReadFailed,
            openvr_sys::EVRSettingsError_VRSettingsError_JsonParseFailed => Self::JsonParseFailed,
            openvr_sys::EVRSettingsError_VRSettingsError_UnsetSettingHasNoDefault => {
                Self::UnsetSettingHasNoDefault
            }
            openvr_sys::EVRSettingsError_VRSettingsError_AccessDenied => Self::AccessDenied,
            _ => unreachable!(),
        }
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
        match value {
            openvr_sys::EVROverlayError_VROverlayError_None => Self::None,
            openvr_sys::EVROverlayError_VROverlayError_UnknownOverlay => Self::UnknownOverlay,
            openvr_sys::EVROverlayError_VROverlayError_InvalidHandle => Self::InvalidHandle,
            openvr_sys::EVROverlayError_VROverlayError_PermissionDenied => Self::PermissionDenied,
            openvr_sys::EVROverlayError_VROverlayError_OverlayLimitExceeded => {
                Self::OverlayLimitExceeded
            }
            openvr_sys::EVROverlayError_VROverlayError_WrongVisibilityType => {
                Self::WrongVisibilityType
            }
            openvr_sys::EVROverlayError_VROverlayError_KeyTooLong => Self::KeyTooLong,
            openvr_sys::EVROverlayError_VROverlayError_NameTooLong => Self::NameTooLong,
            openvr_sys::EVROverlayError_VROverlayError_KeyInUse => Self::KeyInUse,
            openvr_sys::EVROverlayError_VROverlayError_WrongTransformType => {
                Self::WrongTransformType
            }
            openvr_sys::EVROverlayError_VROverlayError_InvalidTrackedDevice => {
                Self::InvalidTrackedDevice
            }
            openvr_sys::EVROverlayError_VROverlayError_InvalidParameter => Self::InvalidParameter,
            openvr_sys::EVROverlayError_VROverlayError_ThumbnailCantBeDestroyed => {
                Self::ThumbnailCantBeDestroyed
            }
            openvr_sys::EVROverlayError_VROverlayError_ArrayTooSmall => Self::ArrayTooSmall,
            openvr_sys::EVROverlayError_VROverlayError_RequestFailed => Self::RequestFailed,
            openvr_sys::EVROverlayError_VROverlayError_InvalidTexture => Self::InvalidTexture,
            openvr_sys::EVROverlayError_VROverlayError_UnableToLoadFile => Self::UnableToLoadFile,
            openvr_sys::EVROverlayError_VROverlayError_KeyboardAlreadyInUse => {
                Self::KeyboardAlreadyInUse
            }
            openvr_sys::EVROverlayError_VROverlayError_NoNeighbor => Self::NoNeighbor,
            openvr_sys::EVROverlayError_VROverlayError_TooManyMaskPrimitives => {
                Self::TooManyMaskPrimitives
            }
            openvr_sys::EVROverlayError_VROverlayError_BadMaskPrimitive => Self::BadMaskPrimitive,
            openvr_sys::EVROverlayError_VROverlayError_TextureAlreadyLocked => {
                Self::TextureAlreadyLocked
            }
            openvr_sys::EVROverlayError_VROverlayError_TextureLockCapacityReached => {
                Self::TextureLockCapacityReached
            }
            openvr_sys::EVROverlayError_VROverlayError_TextureNotLocked => Self::TextureNotLocked,
            openvr_sys::EVROverlayError_VROverlayError_TimedOut => Self::TimedOut,
            _ => unreachable!(),
        }
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
