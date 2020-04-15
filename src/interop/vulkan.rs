//! Types used for interoperation between OpenVR and Vulkan.

/// A Vulkan object handle. The object type behind it is unspecified.
pub type Handle = *const ();

/// Handle to a Vulkan instance.
///
/// Consult the Vulkan documentation on what this exactly is.
pub type Instance = Handle;
/// Handle to a Vulkan device.
///
/// Consult the Vulkan documentation on what this exactly is.
pub type Device = Handle;
/// Handle to a Vulkan physical device.
///
/// Consult the Vulkan documentation on what this exactly is.
pub type PhysicalDevice = Handle;

/// Handle to a Vulkan queue.
///
/// Consult the Vulkan documentation on what this exactly is.
pub type Queue = Handle;

/// Description of a texture, used by the compositor.
///
/// You need to fill out this structure yourself according to the parameters of the texture you want to use, thus all functions which rely on the contents are unsafe.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Texture {
    pub image: u64,
    pub device: Device,
    pub physical_device: PhysicalDevice,
    pub instance: Instance,
    pub queue: Queue,
    pub queue_family_index: u32,
    pub width: u32,
    pub height: u32,
    pub format: u32,
    pub sample_count: u32,
}
// These two are no longer needed
//unsafe impl Send for Texture {}
//unsafe impl Sync for Texture {}