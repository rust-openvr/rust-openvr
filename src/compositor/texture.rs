use crate::{sys, interop};

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


#[derive(Debug, Copy, Clone)]
pub enum Handle {
    Vulkan(interop::vulkan::Texture),
    OpenGLTexture(usize),
    OpenGLRenderBuffer(usize),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ColorSpace {
    Auto = sys::EColorSpace_ColorSpace_Auto as isize,
    Gamma = sys::EColorSpace_ColorSpace_Gamma as isize,
    Linear = sys::EColorSpace_ColorSpace_Linear as isize,
}
