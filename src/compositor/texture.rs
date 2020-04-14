use super::sys;

#[derive(Debug, Copy, Clone)]
pub struct Texture {
    pub handle: Handle,
    pub color_space: ColorSpace,
}

#[repr(C)]
pub struct Bounds {
    pub min: (f32, f32),
    pub max: (f32, f32),
}

/// Support types specifically for interactions between the compositor and Vulkan.
#[cfg(feature = "vulkan")]
pub mod vulkan {
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Texture {
        pub image: u64,
        pub device: vk_sys::Device,
        pub physical_device: vk_sys::PhysicalDevice,
        pub instance: vk_sys::Instance,
        pub queue: vk_sys::Queue,
        pub queue_family_index: u32,
        pub width: u32,
        pub height: u32,
        pub format: u32,
        pub sample_count: u32,
    }
    // These two are no longer needed
    //unsafe impl Send for Texture {}
    //unsafe impl Sync for Texture {}
}

#[derive(Debug, Copy, Clone)]
pub enum Handle {
    #[cfg(feature = "vulkan")]
    Vulkan(vulkan::Texture),
    OpenGLTexture(usize),
    OpenGLRenderBuffer(usize),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ColorSpace {
    Auto = sys::EColorSpace_ColorSpace_Auto as isize,
    Gamma = sys::EColorSpace_ColorSpace_Gamma as isize,
    Linear = sys::EColorSpace_ColorSpace_Linear as isize,
}
