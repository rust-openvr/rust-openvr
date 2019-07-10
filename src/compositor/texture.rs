use super::{sys, VkDevice_T, VkInstance_T, VkPhysicalDevice_T, VkQueue_T};

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
pub enum Handle {
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
