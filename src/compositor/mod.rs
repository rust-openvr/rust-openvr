//! The `Compositor` implements distortion, prediction, synchronization and other subtle issues that can be a challenge to
//! get operating properly for a solid VR experience.
//!
//! Applications call WaitGetPoses to get the set of poses used to render the camera and other tracked objects, render
//! the left and right eyes as normal (using the info provided by `System`) and finally `submit` those undistorted
//! textures for the `Compositor` to display on the output device.
//!
//! It is recommended that you continue presenting your application's own window, reusing either the left or right eye
//! camera render target to draw a single quad (perhaps cropped to a lower fov to hide the hidden area mask).

use std::ffi::CString;
use std::{error, fmt, mem, ptr};

use openvr_sys as sys;

pub mod texture;

pub use self::texture::Texture;

use super::*;

impl Compositor {
    pub fn vulkan_instance_extensions_required(&self) -> Vec<CString> {
        let temp = unsafe {
            get_string(|ptr, n| self.0.GetVulkanInstanceExtensionsRequired.unwrap()(ptr, n))
        }
        .unwrap();
        temp.as_bytes()
            .split(|&x| x == b' ')
            .map(|x| CString::new(x.to_vec()).expect("extension name contained null byte"))
            .collect()
    }

    /// Safety: physical_device must be a valid VkPhysicalDevice
    pub unsafe fn vulkan_device_extensions_required(
        &self,
        physical_device: *mut VkPhysicalDevice_T,
    ) -> Vec<CString> {
        let temp = get_string(|ptr, n| {
            self.0.GetVulkanDeviceExtensionsRequired.unwrap()(physical_device, ptr, n)
        })
        .unwrap();
        temp.as_bytes()
            .split(|&x| x == b' ')
            .map(|x| CString::new(x.to_vec()).expect("extension name contained null byte"))
            .collect()
    }

    /// Sets tracking space returned by WaitGetPoses
    pub fn set_tracking_space(&self, origin: TrackingUniverseOrigin) {
        unsafe { self.0.SetTrackingSpace.unwrap()(origin as sys::ETrackingUniverseOrigin) }
    }

    /// Block until a few milliseconds before the next vsync, then return poses for the next step of rendering and game
    /// logic.
    ///
    /// Poses are relative to the origin set by `set_tracking_space`.
    pub fn wait_get_poses(&self) -> Result<WaitPoses, CompositorError> {
        unsafe {
            let mut result: WaitPoses = mem::uninitialized();
            let e = self.0.WaitGetPoses.unwrap()(
                result.render.as_mut().as_mut_ptr() as *mut _,
                result.render.len() as u32,
                result.game.as_mut().as_mut_ptr() as *mut _,
                result.game.len() as u32,
            );
            if e == sys::EVRCompositorError_VRCompositorError_None {
                Ok(result)
            } else {
                Err(CompositorError(e))
            }
        }
    }

    /// Display the supplied texture for the next frame.
    ///
    /// If `bounds` is None, the entire texture will be used. Lens distortion is handled by the OpenVR implementation.
    ///
    /// # Safety
    ///
    /// The handles you supply must be valid and comply with the graphics API's synchronization requirements.
    pub unsafe fn submit(
        &self,
        eye: Eye,
        texture: &Texture,
        bounds: Option<&texture::Bounds>,
        pose: Option<[[f32; 4]; 3]>,
    ) -> Result<(), CompositorError> {
        use self::texture::Handle::*;
        let flags = match texture.handle {
            Vulkan(_) => sys::EVRSubmitFlags_Submit_Default,
            OpenGLTexture(_) => sys::EVRSubmitFlags_Submit_Default,
            OpenGLRenderBuffer(_) => sys::EVRSubmitFlags_Submit_GlRenderBuffer,
        } | if pose.is_some() {
            sys::EVRSubmitFlags_Submit_TextureWithPose
        } else {
            0
        };
        let texture = sys::VRTextureWithPose_t_real {
            handle: match texture.handle {
                Vulkan(ref x) => x as *const _ as *mut _,
                OpenGLTexture(x) => x as *mut _,
                OpenGLRenderBuffer(x) => x as *mut _,
            },
            eType: match texture.handle {
                Vulkan(_) => sys::ETextureType_TextureType_Vulkan,
                OpenGLTexture(_) => sys::ETextureType_TextureType_OpenGL,
                OpenGLRenderBuffer(_) => sys::ETextureType_TextureType_OpenGL,
            },
            eColorSpace: texture.color_space as sys::EColorSpace,
            mDeviceToAbsoluteTracking: sys::HmdMatrix34_t {
                m: pose.unwrap_or([[0.0; 4]; 3]),
            },
        };
        let e = self.0.Submit.unwrap()(
            eye as sys::EVREye,
            &texture as *const _ as *mut _,
            bounds
                .map(|x| x as *const _ as *mut texture::Bounds as *mut _)
                .unwrap_or(ptr::null_mut()),
            flags,
        );
        if e == sys::EVRCompositorError_VRCompositorError_None {
            Ok(())
        } else {
            Err(CompositorError(e))
        }
    }

    /// Call immediately after presenting your app's window (i.e. companion window) to unblock the compositor.
    ///
    /// This is an optional call, which only needs to be used if you can't instead call `wait_get_poses` immediately
    /// after submitting frames.  For example, if your engine's render and game loop are not on separate threads, or
    /// blocking the render thread until 3ms before the next vsync would introduce a deadlock of some sort.  This
    /// function tells the compositor that you have finished all rendering after having Submitted buffers for both eyes,
    /// and it is free to start its rendering work.  This should only be called from the same thread you are rendering
    /// on.
    pub fn post_present_handoff(&self) {
        unsafe { (self.0.PostPresentHandoff.unwrap())() };
    }

    /// Return whether the compositor is fullscreen.
    pub fn is_fullscreen(&self) -> bool {
        unsafe { (self.0.IsFullscreen.unwrap())() }
    }

    /// Clears the frame that was sent with the last call to `submit.
    ///
    /// This will cause the compositor to show the grid until `submit` is called again.
    pub fn clear_last_submitted_frame(&self) {
        unsafe { self.0.ClearLastSubmittedFrame.unwrap()() }
    }

    /// Controls whether the application should flag the time at which the frame begins explicitly
    ///
    /// *Vulkan/D3D12 Only*
    /// There are two purposes for SetExplicitTimingMode:
    ///	1. To get a more accurate GPU timestamp for when the frame begins in Vulkan/D3D12 applications.
    ///	2. (Optional) To avoid having WaitGetPoses access the Vulkan queue so that the queue can be accessed from
    ///	another thread while WaitGetPoses is executing.
    ///
    /// More accurate GPU timestamp for the start of the frame is achieved by the application calling
    /// SubmitExplicitTimingData immediately before its first submission to the Vulkan/D3D12 queue.  This is more
    /// accurate because normally this GPU timestamp is recorded during WaitGetPoses.  In D3D11, WaitGetPoses queues a
    /// GPU timestamp write, but it does not actually get submitted to the GPU until the application flushes.  By using
    /// SubmitExplicitTimingData, the timestamp is recorded at the same place for Vulkan/D3D12 as it is for D3D11,
    /// resulting in a more accurate GPU time measurement for the frame.
    ///
    /// Avoiding WaitGetPoses accessing the Vulkan queue can be achieved using SetExplicitTimingMode as well.  If this
    /// is desired, the application *MUST* call PostPresentHandoff itself prior to WaitGetPoses.  If
    /// SetExplicitTimingMode is true and the application calls PostPresentHandoff, then WaitGetPoses is guaranteed not
    /// to access the queue.  Note that PostPresentHandoff and SubmitExplicitTimingData will access the queue, so only
    /// WaitGetPoses becomes safe for accessing the queue from another thread.
    pub fn set_explicit_timing_mode(&self, mode: bool) {
        unsafe { self.0.SetExplicitTimingMode.unwrap()(mode as sys::EVRCompositorTimingMode) }
    }

    pub fn submit_explicit_timing_data(&self) -> Result<(), CompositorError> {
        let e = unsafe { self.0.SubmitExplicitTimingData.unwrap()() };
        if e == sys::EVRCompositorError_VRCompositorError_None {
            Ok(())
        } else {
            Err(CompositorError(e))
        }
    }
}

#[derive(Copy, Clone)]
pub struct WaitPoses {
    /// Predicted to the point they will be at the upcoming frame.
    pub render: TrackedDevicePoses,
    /// Predicted to the point they will be at the frame after the upcoming frame, for use in game logic.
    pub game: TrackedDevicePoses,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CompositorError(sys::EVRCompositorError);

pub mod compositor_error {
    use super::*;

    pub const REQUEST_FAILED: CompositorError =
        CompositorError(sys::EVRCompositorError_VRCompositorError_RequestFailed);
    pub const INCOMPATIBLE_VERSION: CompositorError =
        CompositorError(sys::EVRCompositorError_VRCompositorError_IncompatibleVersion);
    pub const DO_NOT_HAVE_FOCUS: CompositorError =
        CompositorError(sys::EVRCompositorError_VRCompositorError_DoNotHaveFocus);
    pub const INVALID_TEXTURE: CompositorError =
        CompositorError(sys::EVRCompositorError_VRCompositorError_InvalidTexture);
    pub const IS_NOT_SCENE_APPLICATION: CompositorError =
        CompositorError(sys::EVRCompositorError_VRCompositorError_IsNotSceneApplication);
    pub const TEXTURE_IS_ON_WRONG_DEVICE: CompositorError =
        CompositorError(sys::EVRCompositorError_VRCompositorError_TextureIsOnWrongDevice);
    pub const TEXTURE_USES_UNSUPPORTED_FORMAT: CompositorError =
        CompositorError(sys::EVRCompositorError_VRCompositorError_TextureUsesUnsupportedFormat);
    pub const SHARED_TEXTURES_NOT_SUPPORTED: CompositorError =
        CompositorError(sys::EVRCompositorError_VRCompositorError_SharedTexturesNotSupported);
    pub const INDEX_OUT_OF_RANGE: CompositorError =
        CompositorError(sys::EVRCompositorError_VRCompositorError_IndexOutOfRange);
    pub const ALREADY_SUBMITTED: CompositorError =
        CompositorError(sys::EVRCompositorError_VRCompositorError_AlreadySubmitted);
    pub const INVALID_BOUNDS: CompositorError =
        CompositorError(sys::EVRCompositorError_VRCompositorError_InvalidBounds);
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
            INVALID_BOUNDS => "INVALID_BOUNDS",
            _ => "UNKNOWN",
        }
    }
}

impl fmt::Display for CompositorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(error::Error::description(self))
    }
}
