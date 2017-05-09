//! The `Compositor` implements distortion, prediction, synchronization and other subtle issues that can be a challenge to
//! get operating properly for a solid VR experience.
//!
//! Applications call WaitGetPoses to get the set of poses used to render the camera and other tracked objects, render
//! the left and right eyes as normal (using the info provided by `System`) and finally `submit` those undistorted
//! textures for the `Compositor` to display on the output device.
//!
//! It is recommended that you continue presenting your application's own window, reusing either the left or right eye
//! camera render target to draw a single quad (perhaps cropped to a lower fov to hide the hidden area mask).

use std::{mem, ptr, error, fmt};
use std::ffi::CString;

use openvr_sys as sys;

use super::*;

impl<'a> Compositor<'a> {
    pub fn vulkan_instance_extensions_required(&self) -> Vec<CString> {
        let temp = unsafe {
            let n = (self.0.GetVulkanInstanceExtensionsRequired.unwrap())(ptr::null_mut(), 0);
            let mut buffer: Vec<u8> = Vec::new();
            buffer.resize(n as usize, mem::uninitialized());
            (self.0.GetVulkanInstanceExtensionsRequired.unwrap())(buffer.as_mut_ptr() as *mut i8, n);
            buffer
        };
        temp.split(|&x| x == b' ').map(|x| CString::new(x.to_vec()).expect("extension name contained null byte")).collect()
    }

    pub fn vulkan_device_extensions_required(&self, physical_device: *mut VkPhysicalDevice_T) -> Vec<CString> {
        let temp = unsafe {
            let n = (self.0.GetVulkanDeviceExtensionsRequired.unwrap())(physical_device, ptr::null_mut(), 0);
            let mut buffer: Vec<u8> = Vec::new();
            buffer.resize(n as usize, mem::uninitialized());
            (self.0.GetVulkanDeviceExtensionsRequired.unwrap())(physical_device as *mut _, buffer.as_mut_ptr() as *mut i8, n);
            buffer
        };
        temp.split(|&x| x == b' ').map(|x| CString::new(x.to_vec()).expect("extension name contained null byte")).collect()
    }

    /// Sets tracking space returned by WaitGetPoses
    pub fn set_tracking_space(&self, origin: TrackingUniverseOrigin) {
        unsafe { (self.0.SetTrackingSpace.unwrap())(origin as sys::ETrackingUniverseOrigin) }
    }

    /// Block until a few milliseconds before the next vsync, then return poses for the next step of rendering and game
    /// logic.
    ///
    /// Poses are relative to the origin set by `set_tracking_space`.
    pub fn wait_get_poses(&self) -> Result<WaitPoses, CompositorError> {
        unsafe {
            let mut result: WaitPoses = mem::uninitialized();
            let e = (self.0.WaitGetPoses.unwrap())(result.render.data.as_mut().as_mut_ptr() as *mut _, result.render.data.len() as u32,
                                                   result.game.data.as_mut().as_mut_ptr() as *mut _, result.game.data.len() as u32);
            if e == sys::EVRCompositorError_EVRCompositorError_VRCompositorError_None {
                Ok(result)
            } else {
                Err(CompositorError(e))
            }
        }
    }

    /// Display the supplied texture for the next frame.
    ///
    /// If `bounds` is None, the entire texture will be used. Lens distortion is handled by the OpenVR implementation.
    pub fn submit(&self, eye: Eye, texture: &Texture, bounds: Option<&TextureBounds>) -> Result<(), CompositorError> {
        use self::TextureHandle::*;
        let texture = sys::Texture_t {
            handle: match texture.handle {
                Vulkan(ref x) => x as *const _ as *mut _,
            },
            eType: match texture.handle {
                Vulkan(_) => sys::ETextureType_ETextureType_TextureType_Vulkan,
            },
            eColorSpace: texture.color_space as sys::EColorSpace,
        };
        let e = unsafe {
            (self.0.Submit.unwrap())(eye as sys::EVREye,
                                     &texture as *const _ as *mut _,
                                     bounds.map(|x| x as *const _ as *mut TextureBounds as *mut _).unwrap_or(ptr::null_mut()),
                                     sys::EVRSubmitFlags_EVRSubmitFlags_Submit_Default)
        };
        if e == sys::EVRCompositorError_EVRCompositorError_VRCompositorError_None {
            Ok(())
        } else {
            Err(CompositorError(e))
        }
    }

    pub fn post_present_handoff(&self) {
        unsafe { (self.0.PostPresentHandoff.unwrap())() };
    }
}

#[derive(Debug, Copy, Clone)]
pub struct WaitPoses {
    /// Predicted to the point they will be at the upcoming frame.
    pub render: TrackedDevicePoses,
    /// Predicted to the point they will be at the frame after the upcoming frame, for use in game logic.
    pub game: TrackedDevicePoses,
}

pub use sys::VkPhysicalDevice_T;
pub use sys::VkDevice_T;
pub use sys::VkInstance_T;
pub use sys::VkQueue_T;

#[derive(Debug, Copy, Clone)]
pub struct Texture {
    pub handle: TextureHandle,
    pub color_space: ColorSpace,
}

pub mod vulkan {
    use super::*;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Texture {
        pub image: u64,
        pub device: *mut VkDevice_T,
        pub physical_device: *mut VkPhysicalDevice_T,
        pub instance: *mut VkInstance_T,
        pub queue: *mut VkQueue_T,
        pub queue_family_index: u32,
        pub width: u32,
        pub height: u32,
        pub format: u32,
        pub sample_count: u32,
    }
}

#[derive(Debug, Copy, Clone)]
pub enum TextureHandle {
    Vulkan(vulkan::Texture),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ColorSpace {
    Auto = sys::EColorSpace_EColorSpace_ColorSpace_Auto as isize,
    Gamma = sys::EColorSpace_EColorSpace_ColorSpace_Gamma as isize,
    Linear = sys::EColorSpace_EColorSpace_ColorSpace_Linear as isize,
}

#[repr(C)]
pub struct TextureBounds {
    pub umin: f32,
    pub vmin: f32,
    pub umax: f32,
    pub vmax: f32,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CompositorError(sys::EVRCompositorError);

pub mod compositor_error {
    use super::*;

    pub const REQUEST_FAILED: CompositorError = CompositorError(sys::EVRCompositorError_EVRCompositorError_VRCompositorError_RequestFailed);
    pub const INCOMPATIBLE_VERSION: CompositorError = CompositorError(sys::EVRCompositorError_EVRCompositorError_VRCompositorError_IncompatibleVersion);
    pub const DO_NOT_HAVE_FOCUS: CompositorError = CompositorError(sys::EVRCompositorError_EVRCompositorError_VRCompositorError_DoNotHaveFocus);
    pub const INVALID_TEXTURE: CompositorError = CompositorError(sys::EVRCompositorError_EVRCompositorError_VRCompositorError_InvalidTexture);
    pub const IS_NOT_SCENE_APPLICATION: CompositorError = CompositorError(sys::EVRCompositorError_EVRCompositorError_VRCompositorError_IsNotSceneApplication);
    pub const TEXTURE_IS_ON_WRONG_DEVICE: CompositorError = CompositorError(sys::EVRCompositorError_EVRCompositorError_VRCompositorError_TextureIsOnWrongDevice);
    pub const TEXTURE_USES_UNSUPPORTED_FORMAT: CompositorError = CompositorError(sys::EVRCompositorError_EVRCompositorError_VRCompositorError_TextureUsesUnsupportedFormat);
    pub const SHARED_TEXTURES_NOT_SUPPORTED: CompositorError = CompositorError(sys::EVRCompositorError_EVRCompositorError_VRCompositorError_SharedTexturesNotSupported);
    pub const INDEX_OUT_OF_RANGE: CompositorError = CompositorError(sys::EVRCompositorError_EVRCompositorError_VRCompositorError_IndexOutOfRange);
    pub const ALREADY_SUBMITTED: CompositorError = CompositorError(sys::EVRCompositorError_EVRCompositorError_VRCompositorError_AlreadySubmitted);
}

impl fmt::Debug for CompositorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(error::Error::description(self))
    }
}

impl error::Error for CompositorError {
    fn description(&self) -> &str {
        use self::compositor_error::*;
        match *self {
            REQUEST_FAILED => "REQUEST_FAILED",
            INCOMPATIBLE_VERSION => "INCOMPATIBLE_VERSION",
            DO_NOT_HAVE_FOCUS => "DO_NOT_HAVE_FOCUS",
            INVALID_TEXTURE => "INVALID_TEXTURE",
            IS_NOT_SCENE_APPLICATION => "IS_NOT_SCENE_APPLICATION",
            TEXTURE_IS_ON_WRONG_DEVICE => "TEXTURE_IS_ON_WRONG_DEVICE",
            TEXTURE_USES_UNSUPPORTED_FORMAT => "TEXTURE_USES_UNSUPPORTED_FORMAT",
            SHARED_TEXTURES_NOT_SUPPORTED => "SHARED_TEXTURES_NOT_SUPPORTED",
            INDEX_OUT_OF_RANGE => "INDEX_OUT_OF_RANGE",
            ALREADY_SUBMITTED => "ALREADY_SUBMITTED",
            _ => "UNKNOWN",
        }
    }
}

impl fmt::Display for CompositorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(error::Error::description(self))
    }
}
