use openvr_sys;
use openvr_sys::EVREye::*;

#[derive(Debug, Copy, Clone)]
pub struct Size {
    pub width: u32,
    pub height: u32
}

#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

#[derive(Debug, Copy, Clone)]
pub struct Rectangle {
    pub position: Position,
    pub size: Size
}

#[derive(Debug, Copy, Clone)]
pub struct DistortionCoordinates {
    pub red: [f32; 2],
    pub green: [f32; 2],
    pub blue: [f32; 2],
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Eye {
    Left, Right
}

impl Eye {
    /// Convert a eye to a HmdEye
    pub fn to_raw(&self) -> openvr_sys::EVREye {
        match self {
            &Eye::Left => EVREye_Eye_Left,
            &Eye::Right => EVREye_Eye_Right,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TextureBounds {
    pub u_min: f32,
    pub u_max: f32,
    pub v_min: f32,
    pub v_max: f32
}

impl TextureBounds {
    pub fn new(u: (f32, f32), v: (f32, f32)) -> Self {
        TextureBounds {
            u_min: u.0,
            u_max: u.1,
            v_min: v.0,
            v_max: v.1
        }
    }

    pub fn full() -> Self {
        TextureBounds {
            u_min: 0.0,
            u_max: 1.0,
            v_min: 0.0,
            v_max: 1.0
        }
    }

    /// Convert a bounds to a openvr_bounds
    pub fn to_raw(self) -> openvr_sys::VRTextureBounds_t {
        openvr_sys::VRTextureBounds_t {
            uMin: self.u_min,
            uMax: self.u_max,
            vMin: self.v_min,
            vMax: self.v_max
        }
    }
}
