use openvr_sys;
use openvr_sys::Enum_EVREye::*;

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
    /// Convert a bounds to a openvr_bounds
    pub fn to_raw(self) -> openvr_sys::VRTextureBounds_t {
        openvr_sys::VRTextureBounds_t{
            uMin: self.u_min,
            uMax: self.u_max,
            vMin: self.v_min,
            vMax: self.v_max
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct TrackedDevicePose {
    pub to_device: [[f32; 4]; 3],
    pub velocity: [f32; 3],
    pub angular_velocity: [f32; 3],
    pub is_valid: bool,
    pub is_connected: bool,
}

#[derive(Debug, Copy, Clone)]
pub struct TrackedDevicePoses {
    pub count: usize,
    pub poses: [TrackedDevicePose; 16],
}

impl TrackedDevicePoses {
    pub fn as_slice(&self) -> &[TrackedDevicePose] {
        &self.poses[0..self.count]
    }
}

pub unsafe fn to_tracked(data: [openvr_sys::TrackedDevicePose_t; 16]) -> TrackedDevicePoses {
    use std;
    let mut out: TrackedDevicePoses = std::mem::zeroed();
    for (i, d) in data.iter().enumerate() {
        if d.bDeviceIsConnected > 0 {
            out.count = i + 1;
        }
        out.poses[i].is_connected = d.bDeviceIsConnected > 0;
        out.poses[i].is_valid = d.bPoseIsValid > 0;
        out.poses[i].to_device = d.mDeviceToAbsoluteTracking.m;
        out.poses[i].velocity = d.vVelocity.v;
        out.poses[i].angular_velocity = d.vAngularVelocity.v;
    }
    out
}
