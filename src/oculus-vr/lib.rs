#![crate_id = "oculus-vr#0.1"]
#![crate_type = "lib"]
#![feature(link_args)]

extern crate cgmath;
extern crate libc;

use libc::{c_int, c_uint, c_void, c_float};
use std::str::raw::from_c_str;
use std::default::Default;
use std::ptr;

use cgmath::quaternion::Quaternion;
use cgmath::vector::{Vector2, Vector3};
use cgmath::matrix::{Matrix4};

#[cfg(target_os = "linux")]
#[link(name="ovr")]
#[link(name="stdc++")]
#[link(name="udev")]
#[link(name="Xinerama")]
#[link(name="Xrandr")]
#[link(name="X11")]
#[link(name="GL")]
extern {}
 
#[cfg(target_os = "macos")]
#[link(name="ovr")]
#[link(name="stdc++")]
#[link(name = "Cocoa", kind = "framework")]
#[link(name = "IOKit", kind = "framework")]
#[link(name = "CoreFoundation", kind = "framework")]
extern {}


pub mod ll {
    use libc::{c_uint, c_int, c_float, c_char, c_void, c_double, c_short};
    use std::ptr;
    use std::default::Default;

    #[deriving(Clone, Default)]
    pub struct Vector2i {
        pub x: c_int,
        pub y: c_int
    }

    #[deriving(Clone, Default)]
    pub struct Sizei {
        pub x: c_int,
        pub y: c_int
    }

    #[deriving(Clone, Default)]
    pub struct Recti {
        pub pos: Vector2i,
        pub size: Sizei
    }


    #[deriving(Clone, Default)]
    pub struct FovPort {
        pub upTan: c_float,
        pub downTan: c_float,
        pub leftTan: c_float,
        pub rightTan: c_float
    }

    #[deriving(Clone, Default)]
    pub struct Vector2f {pub x: c_float, pub y: c_float}

    #[deriving(Clone, Default)]
    pub struct Vector3f {pub x: c_float, pub y: c_float, pub z: c_float}

    #[deriving(Clone, Default)]
    pub struct Quaternionf {pub x: c_float, pub y: c_float, pub z: c_float, pub w: c_float}

    #[deriving(Clone, Default)]
    pub struct Matrix4f {pub m11: c_float, pub m12: c_float, pub m13: c_float, pub m14: c_float,
                         pub m21: c_float, pub m22: c_float, pub m23: c_float, pub m24: c_float,
                         pub m31: c_float, pub m32: c_float, pub m33: c_float, pub m34: c_float,
                         pub m41: c_float, pub m42: c_float, pub m43: c_float, pub m44: c_float}


    #[deriving(Clone, Default)]
    pub struct Posef {
        pub orientation: Quaternionf,
        pub position: Vector3f
    }

    #[deriving(Clone, Default)]
    pub struct PoseState {
        pub pose: Posef,
        pub angular_velocity: Vector3f,
        pub linear_velocity: Vector3f,
        pub angular_acceleration: Vector3f,
        pub linear_acceleration: Vector3f,
        pub time_in_seconds: c_double
    }

    #[deriving(Clone, Default)]
    pub struct SensorState {
        pub predicted: PoseState,
        pub recorded: PoseState,
        pub temperature: c_float,
        pub status_flags: c_uint
    }

    pub enum Hmd {}

    pub struct HmdDesc {
        pub handle: *Hmd,
        pub hmd_type: c_int,
        pub product_name: *c_char,
        pub manufacture: *c_char,
        pub hmd_capabilities: c_uint,
        pub sensor_capabilities: c_uint,
        pub distortion_capabilities: c_uint,
        pub resolution: Sizei,
        pub window_position: Vector2i,
        pub default_eye_fov: [FovPort, ..2],
        pub max_eye_fov: [FovPort, ..2],
        pub eye_render_order: [c_uint, ..2],
        pub display_device_name: *c_char,
        pub display_id: c_int
    }

    impl Default for HmdDesc {
        fn default() -> HmdDesc {
            HmdDesc {
                handle: ptr::null(),
                hmd_type: 0,
                product_name: ptr::null(),
                manufacture: ptr::null(),
                hmd_capabilities: 0,
                sensor_capabilities: 0,
                distortion_capabilities: 0,
                resolution: Default::default(),
                window_position: Default::default(),
                default_eye_fov: [Default::default(), ..2],
                max_eye_fov: [Default::default(), ..2],
                eye_render_order: [0, ..2],
                display_device_name: ptr::null(),
                display_id: 0
            }
        }
    }

    pub struct SensorDesc {
        pub vendor_id: c_short,
        pub product_id: c_short,
        pub serial_number: [c_char, ..24]
    }

    #[deriving(Clone, Default)]
    pub struct EyeRenderDesc {
        pub eye: c_uint,
        pub fov: FovPort,
        pub distorted_viewport: Recti,
        pub pixels_per_tan_angle_at_center: Vector2f,
        pub view_adjust: Vector3f
    }

    pub struct RenderApiConfigHeader {
        pub render_api_type: c_uint,
        pub rt_size: Sizei,
        pub multisample: c_int,
    }

    pub struct RenderApiConfig {
        pub header: RenderApiConfigHeader,
        pub display: *c_void,
        pub window: *c_void,
        pub padd: [*c_void, ..6]
    }

    pub struct FrameTiming {
        pub delta_seconds: f32,
        pub this_frame_seconds: f64,
        pub timewarp_point_seconds: f64,
        pub next_frame_seconds: f64,
        pub scanout_midpoint_seconds: f64,
        pub eye_scanout_seconds: [f64, ..2]        
    }

    pub struct TextureHeader {
        pub render_api_type: c_uint,
        pub size: Sizei,
        pub viewport: Recti    
    }

    pub struct Texture {
        pub header: TextureHeader,
        pub texture_id: u32,
        pub padd: [*c_void, ..7]
    }

    pub static Hmd_None                      : c_int = 0;
    pub static Hmd_DK1                       : c_int = 3;
    pub static Hmd_DKHD                      : c_int = 4;
    pub static Hmd_CrystalCoveProto          : c_int = 5;
    pub static Hmd_DK2                       : c_int = 6;
    pub static Hmd_Other                     : c_int = 7;

    pub static HmdCap_Present                : c_uint = 0x0001;
    pub static HmdCap_Available              : c_uint = 0x0002;
    pub static HmdCap_LowPersistence         : c_uint = 0x0080;
    pub static HmdCap_LatencyTest            : c_uint = 0x0100;
    pub static HmdCap_DynamicPrediction      : c_uint = 0x0200;
    pub static HmdCap_NoVSync                : c_uint = 0x1000;
    pub static HmdCap_NoRestore              : c_uint = 0x4000;
    pub static HmdCap_Writable_Mask          : c_uint = 0x1380;

    pub static SensorCap_Orientation         : c_uint = 0x0010;
    pub static SensorCap_YawCorrection       : c_uint = 0x0020;
    pub static SensorCap_Position            : c_uint = 0x0040;

    pub static Status_OrientationTracked     : c_uint = 0x0001;
    pub static Status_PositionTracked        : c_uint = 0x0002;
    pub static Status_PositionConnected      : c_uint = 0x0020;
    pub static Status_HmdConnected           : c_uint = 0x0080;

    pub static DistortionCap_Chromatic       : c_uint = 0x01;
    pub static DistortionCap_TimeWarp        : c_uint = 0x02;
    pub static DistortionCap_Vignette        : c_uint = 0x08;

    pub static Eye_Left                      : c_uint = 0;
    pub static Eye_Right                     : c_uint = 1;

    pub static RenderAPI_None                : c_uint = 0;
    pub static RenderAPI_OpenGL              : c_uint = 1;
    pub static RenderAPI_Android_GLES        : c_uint = 2;
    pub static RenderAPI_D3D9                : c_uint = 3;
    pub static RenderAPI_D3D10               : c_uint = 4;
    pub static RenderAPI_D3D11               : c_uint = 5;
    pub static RenderAPI_Count               : c_uint = 6;

    extern "C" {
        pub fn ovr_Initialize() -> bool;
        pub fn ovr_Shutdown();
        pub fn ovrHmd_Detect() -> c_int;
        pub fn ovrHmd_Create(index: c_int) -> *Hmd;
        pub fn ovrHmd_Destroy(hmd: *Hmd);
        pub fn ovrHmd_CreateDebug(hmd_type: c_int) -> *Hmd;
        pub fn ovrHmd_GetLastError(hmd: *Hmd) -> *c_char;
        pub fn ovrHmd_GetEnabledCaps(hmd: *Hmd) -> c_uint;
        pub fn ovrHmd_SetEnabledCaps(hmd: *Hmd, flags: c_uint);
        pub fn ovrHmd_StartSensor(hmd: *Hmd,
                                  supported: c_uint,
                                  required: c_uint) -> bool;
        pub fn ovrHmd_StopSensor(hmd: *Hmd);
        pub fn ovrHmd_ResetSensor(hmd: *Hmd);
        pub fn ovrHmd_GetSensorState(hmd: *Hmd,
                                     absTime: c_double) -> SensorState;
        pub fn ovrHmd_GetSensorDesc(hmd: *Hmd,
                                    sensor_desc: *SensorDesc) -> bool;
        pub fn ovrHmd_GetDesc(hmd: *Hmd,
                              size: *HmdDesc);
        pub fn ovrHmd_GetFovTextureSize(hmd: *Hmd,
                                        eye: c_uint,
                                        fov: FovPort,
                                        pixels: c_float) -> Sizei;
        pub fn ovrHmd_ConfigureRendering(hmd: *Hmd,
                                         apiConfig: *RenderApiConfig,
                                         distortionCaps: c_uint,
                                         fov_in: *FovPort,
                                         render_desc_out: *EyeRenderDesc) -> bool;
        pub fn ovrHmd_BeginFrame(hmd: *Hmd,
                                 frame_index: c_uint) -> FrameTiming;
        pub fn ovrHmd_EndFrame(hmd: *Hmd);
        pub fn ovrHmd_BeginEyeRender(hmd: *Hmd, eye: c_uint) -> Posef;
        pub fn ovrHmd_EndEyeRender(hmd: *Hmd, eye: c_uint, 
                                   pose: Posef, texture: *Texture);
        pub fn ovrMatrix4f_Projection(fov: FovPort,
                                      znear: c_float,
                                      zfar: c_float,
                                      right_handed: bool) -> Matrix4f;
    }
}

#[repr(C)]
pub enum HmdType {
    HmdNone                 = ll::Hmd_None,
    HmdDK1                  = ll::Hmd_DK1,
    HmdDKHD                 = ll::Hmd_DKHD,
    HmdCrystalCoveProto     = ll::Hmd_CrystalCoveProto,
    HmdDK2                  = ll::Hmd_DK2,
    HmdOther                = ll::Hmd_Other
}

impl HmdType {
    fn from_ll(c: c_int) -> HmdType {
        match c {
            ll::Hmd_None => {HmdNone}
            ll::Hmd_DK1 => {HmdDK1}
            ll::Hmd_DKHD => {HmdDKHD}
            ll::Hmd_CrystalCoveProto => {HmdCrystalCoveProto}
            ll::Hmd_DK2 => {HmdDK2}
            _ => {HmdOther}   
        }
    }
}

pub struct Ovr;

impl Ovr {
    pub fn init() -> Option<Ovr> {
        unsafe {
            if ll::ovr_Initialize() {
                Some(Ovr)
            } else {
                None
            }
        }
    }

    // return a count of the number of Hmd devices
    pub fn detect(&self) -> int {
        unsafe { ll::ovrHmd_Detect() as int }
    }

    pub fn create_hmd(&self, index: int) -> Option<Hmd> {
        unsafe {
            let ptr = ll::ovrHmd_Create(index as i32);
            if !ptr.is_null() {
                Some(Hmd { ptr: ptr })
            } else {
                None
            }
        }
    }

    pub fn first_hmd(&self) -> Option<Hmd> {
        if self.detect() >= 1 {
            self.create_hmd(0)
        } else {
            None
        }
    }

    pub fn create_hmd_debug(&self, hmd_type: HmdType) -> Option<Hmd> {
        unsafe {
            let ptr = ll::ovrHmd_CreateDebug(hmd_type as c_int);
            if !ptr.is_null() {
                Some(Hmd{ptr:ptr})
            } else {
                None
            }
        }   
    }
}

impl Drop for Ovr {
    fn drop(&mut self) {
        unsafe { ll::ovr_Shutdown(); }
    }
}

pub struct Hmd {
    ptr: *ll::Hmd
}

impl Drop for Hmd {
    fn drop(&mut self) {
        unsafe {ll::ovrHmd_Destroy(self.ptr)}
    }
}

impl Hmd {
    pub fn get_last_error(&self) -> Result<(), ~str> {
        unsafe {
            let ptr = ll::ovrHmd_GetLastError(self.ptr);
            if ptr.is_null() {
                Ok(())
            } else {
                Err(from_c_str(ptr))
            }
        }
    }

    pub fn get_enabled_caps(&self) -> HmdCapabilities {
        unsafe {
            let flags = ll::ovrHmd_GetEnabledCaps(self.ptr);
            HmdCapabilities{flags:flags}
        }
    }

    pub fn set_enabled_caps(&self, cap: SensorCapabilities) {
        unsafe {
            let flags = cap.flags;
            ll::ovrHmd_SetEnabledCaps(self.ptr, flags);
        }
    }

    pub fn start_sensor(&self,
                        supported: SensorCapabilities,
                        required: SensorCapabilities) -> bool {
        unsafe {
            ll::ovrHmd_StartSensor(self.ptr, supported.flags, required.flags)
        }
    }

    pub fn stop_sensor(&self) {
        unsafe {
            ll::ovrHmd_StopSensor(self.ptr)
        }
    }

    pub fn reset_sensor(&self) {
        unsafe {
            ll::ovrHmd_StopSensor(self.ptr)
        }
    }

    pub fn get_sensor_state(&self, absTime: f64) -> SensorState {
        unsafe {
            SensorState::from_ll(ll::ovrHmd_GetSensorState(self.ptr, absTime))
        }
    }

    pub fn get_sensor_description(&self) -> Option<SensorDescription> {
        unsafe {
            let c_desc = ll::SensorDesc {
                vendor_id: 0,
                product_id: 0,
                serial_number: [0,.. 24]
            };

            if !ll::ovrHmd_GetSensorDesc(self.ptr, &c_desc as *ll::SensorDesc) {
                None
            } else {
                Some(SensorDescription::from_ll(c_desc))
            }
        }
    }

    pub fn get_description(&self) -> HmdDescription {
        unsafe {
            let c_desc = Default::default();
            ll::ovrHmd_GetDesc(self.ptr, &c_desc);
            HmdDescription::from_ll(c_desc)
        }
    }

    pub fn get_fov_texture_size(&self,
                                eye: EyeType,
                                fov: FovPort,
                                pixels_per_display_pixel: f32) -> ll::Sizei {
        unsafe {
            ll::ovrHmd_GetFovTextureSize(self.ptr,
                                         eye.to_ll(),
                                         fov.to_ll(),
                                         pixels_per_display_pixel)
        }
    }

    pub fn configure_rendering<RC: ToRenderConfig>(&self,
                               api_config: &RC,
                               cap: DistortionCapabilities,
                               eye_fov: PerEye<FovPort>) -> Option<PerEye<EyeRenderDescriptor>> {
        unsafe {
            let out: PerEye<ll::EyeRenderDesc> 
                = PerEye::new(Default::default(),
                              Default::default());
            let was_started = ll::ovrHmd_ConfigureRendering(
                self.ptr,
                &api_config.to_render_config(),
                cap.flags,
                eye_fov.map(|_, d| d.to_ll()).ptr(),
                out.ptr()
            );

            if was_started {
                Some(out.map(|_, d| EyeRenderDescriptor::from_ll(d)))
            } else {
                None
            }
        }
    }

    pub fn begin_frame(&self, frame_index: uint) -> FrameTiming {
        unsafe {
            FrameTiming::from_ll(
                ll::ovrHmd_BeginFrame(self.ptr, frame_index as c_uint)
            )
        }
    }

    pub fn end_frame(&self) {
        unsafe {
            ll::ovrHmd_EndFrame(self.ptr);
        }
    }

    pub fn begin_eye_render(&self, eye: EyeType) -> Pose {
        unsafe {
            Pose::from_ll(ll::ovrHmd_BeginEyeRender(self.ptr, eye.to_ll()))
        }
    }

    pub fn end_eye_render<T: ToTexture>(&self,
                                        eye: EyeType,
                                        pose: Pose,
                                        texture: &T) {
        unsafe {
            ll::ovrHmd_EndEyeRender(self.ptr,
                                    eye.to_ll(),
                                    pose.to_ll(),
                                    &texture.to_texture());
        }
    }
}

pub struct HmdCapabilities {
    flags: c_uint
}

impl HmdCapabilities {
    pub fn present(&self) -> bool {
        self.flags & ll::HmdCap_Present == ll::HmdCap_Present
    }

    pub fn available(&self) -> bool {
        self.flags & ll::HmdCap_Available == ll::HmdCap_Available
    }

    pub fn low_persistance(&self) -> bool {
        self.flags & ll::HmdCap_LowPersistence 
                == ll::HmdCap_LowPersistence
    }

    pub fn set_low_persistance(&self, flag: bool) -> HmdCapabilities {
        HmdCapabilities{flags: 
            if flag {
                self.flags | ll::HmdCap_LowPersistence
            } else {
                self.flags & !ll::HmdCap_LowPersistence
            }
        }
    }

    pub fn latency_test(&self) -> bool {
        self.flags & ll::HmdCap_LatencyTest == ll::HmdCap_LatencyTest
    }

    pub fn set_latency_test(&self, flag: bool) -> HmdCapabilities {
        HmdCapabilities{flags: 
            if flag {
                self.flags | ll::HmdCap_LatencyTest
            } else {
                self.flags & !ll::HmdCap_LatencyTest
            }
        }
    }

    pub fn dynamic_prediction(&self) -> bool {
        self.flags & ll::HmdCap_DynamicPrediction 
                == ll::HmdCap_DynamicPrediction
    }

    pub fn set_dynamic_prediction(&self, flag: bool) -> HmdCapabilities {
        HmdCapabilities{flags: 
            if flag {
                self.flags | ll::HmdCap_DynamicPrediction
            } else {
                self.flags & !ll::HmdCap_DynamicPrediction
            }
        }
    }

    pub fn no_vsync(&self) -> bool {
        self.flags & ll::HmdCap_NoVSync == ll::HmdCap_NoVSync
    }

    pub fn set_no_vsync(&self, flag: bool) -> HmdCapabilities {
        HmdCapabilities{flags: 
            if flag {
                self.flags | ll::HmdCap_NoVSync
            } else {
                self.flags & !ll::HmdCap_NoVSync
            }
        }
    }

    pub fn no_restore(&self) -> bool {
        self.flags & ll::HmdCap_NoRestore == ll::HmdCap_NoRestore
    }
}

pub struct SensorCapabilities {
    flags: c_uint
}

impl SensorCapabilities {
    pub fn new() -> SensorCapabilities {
        SensorCapabilities {
            flags: 0
        }
    }

    pub fn orientation(&self) -> bool {
        self.flags & ll::SensorCap_Orientation ==
            ll::SensorCap_Orientation
    }

    pub fn yaw_correction(&self) -> bool {
        self.flags & ll::SensorCap_YawCorrection ==
            ll::SensorCap_YawCorrection
    }

    pub fn position(&self) -> bool {
        self.flags & ll::SensorCap_Position ==
            ll::SensorCap_Position
    }

    pub fn set_orientation(&self, flag: bool) -> SensorCapabilities {
        SensorCapabilities {flags: 
            if flag {
                self.flags | ll::SensorCap_Orientation
            } else {
                self.flags & !ll::SensorCap_Orientation
            }
        }
    }

    pub fn set_yaw_correction(&self, flag: bool) -> SensorCapabilities {
        SensorCapabilities {flags: 
            if flag {
                self.flags | ll::SensorCap_YawCorrection
            } else {
                self.flags & !ll::SensorCap_YawCorrection
            }
        }
    }

    pub fn set_position(&self, flag: bool) -> SensorCapabilities {
        SensorCapabilities {flags: 
            if flag {
                self.flags | ll::SensorCap_Position
            } else {
                self.flags & !ll::SensorCap_Position
            }
        }
    }
}

pub struct DistortionCapabilities {
    flags: c_uint
}

impl DistortionCapabilities {
    pub fn new() -> DistortionCapabilities {
        DistortionCapabilities {
            flags: 0
        }
    }

    pub fn chromatic(&self) -> bool {
        self.flags & ll::DistortionCap_Chromatic ==
            ll::DistortionCap_Chromatic
    }

    pub fn timewarp(&self) -> bool {
        self.flags & ll::DistortionCap_TimeWarp ==
            ll::DistortionCap_TimeWarp
    }

    pub fn vignette(&self) -> bool {
        self.flags & ll::DistortionCap_Vignette ==
            ll::DistortionCap_Vignette
    }

    pub fn set_chromatic(&self, flag: bool) -> DistortionCapabilities {
        DistortionCapabilities {flags: 
            if flag {
                self.flags | ll::DistortionCap_Chromatic
            } else {
                self.flags & !ll::DistortionCap_Chromatic
            }
        }
    }

    pub fn set_timewarp(&self, flag: bool) -> DistortionCapabilities {
        DistortionCapabilities {flags: 
            if flag {
                self.flags | ll::DistortionCap_TimeWarp
            } else {
                self.flags & !ll::DistortionCap_TimeWarp
            }
        }
    }

    pub fn set_vignette(&self, flag: bool) -> DistortionCapabilities {
        DistortionCapabilities {flags: 
            if flag {
                self.flags | ll::DistortionCap_Vignette
            } else {
                self.flags & !ll::DistortionCap_Vignette
            }
        }
    }
}

#[deriving(Clone)]
pub struct Status {
    flags: u32
}

impl Status {
    pub fn orientation_tracked(&self) -> bool {
        self.flags & ll::Status_OrientationTracked ==
            ll::Status_OrientationTracked
    }

    pub fn position_tracked(&self) -> bool {
        self.flags & ll::Status_PositionTracked ==
            ll::Status_PositionTracked
    }

    pub fn position_connected(&self) -> bool {
        self.flags & ll::Status_PositionConnected ==
            ll::Status_PositionConnected
    }

    pub fn hmd_connected(&self) -> bool {
        self.flags & ll::Status_HmdConnected ==
            ll::Status_HmdConnected
    }
}

fn to_quat(q: ll::Quaternionf) -> Quaternion<f32> {
    Quaternion::new(q.w, q.x, q.y, q.z)
}

fn to_vec3(v: ll::Vector3f) -> Vector3<f32> {
    Vector3::new(v.x, v.y, v.z)
}

fn to_mat4(ll: ll::Matrix4f) -> Matrix4<f32> {
    Matrix4::new(
        ll.m11, ll.m21, ll.m31, ll.m41,
        ll.m12, ll.m22, ll.m32, ll.m42,
        ll.m13, ll.m23, ll.m33, ll.m43,
        ll.m14, ll.m24, ll.m34, ll.m44
    )
}

fn from_quat(q: Quaternion<f32>) -> ll::Quaternionf {
    ll::Quaternionf {
        x: q.v.x, y: q.v.y, z: q.v.z, w: q.s
    }
}

fn from_vec3(v: Vector3<f32>) -> ll::Vector3f {
    ll::Vector3f {
        x: v.x, y: v.y, z: v.z
    }
}

#[deriving(Clone)]
pub struct Pose {
    pub orientation: Quaternion<f32>,
    pub position: Vector3<f32>
}

impl Pose {
    fn from_ll(pose: ll::Posef) -> Pose {
        Pose {
            orientation: to_quat(pose.orientation),
            position: to_vec3(pose.position),
        }
    }

    fn to_ll(&self) -> ll::Posef {
        ll::Posef {
            orientation: from_quat(self.orientation),
            position: from_vec3(self.position),
        }
    }
}

#[deriving(Clone)]
pub struct PoseState {
    pub pose: Pose,
    pub angular_velocity: Vector3<f32>,
    pub linear_velocity: Vector3<f32>,
    pub angular_acceleration: Vector3<f32>,
    pub linear_acceleration: Vector3<f32>,
    pub time_in_seconds: f64
}

impl PoseState {
    fn from_ll(pose: ll::PoseState) -> PoseState {
        PoseState {
            pose: Pose::from_ll(pose.pose),
            angular_velocity: to_vec3(pose.angular_velocity),
            linear_velocity: to_vec3(pose.linear_velocity),
            angular_acceleration: to_vec3(pose.angular_acceleration),
            linear_acceleration: to_vec3(pose.linear_acceleration),
            time_in_seconds: pose.time_in_seconds as f64
        }
    }
}

#[deriving(Clone)]
pub struct SensorState {
    pub predicted: PoseState,
    pub recorded: PoseState,
    pub temperature: f32,
    pub status_flags: Status
}

impl SensorState {
    fn from_ll(ss: ll::SensorState) -> SensorState {
        SensorState {
            predicted: PoseState::from_ll(ss.predicted),
            recorded: PoseState::from_ll(ss.recorded),
            temperature: ss.temperature as f32,
            status_flags: Status{flags: ss.status_flags}
        }
    }
}
#[deriving(Clone)]
pub struct SensorDescription {
    pub vendor_id: i16,
    pub product_id: i16,
    pub serial_number: ~str,
}

impl SensorDescription {
    fn from_ll(sd: ll::SensorDesc) -> SensorDescription {
        SensorDescription {
            vendor_id: sd.vendor_id as i16,
            product_id: sd.product_id as i16,
            serial_number: unsafe { from_c_str(&sd.serial_number[0]) }
        }
    }
}

pub enum EyeType {
    EyeLeft,
    EyeRight
}

impl EyeType {
    fn from_ll(c: c_uint) -> EyeType {
        match c {
            ll::Eye_Left => EyeLeft,
            ll::Eye_Right => EyeRight,
            _ => fail!("Invalid eye type {:?}", c)
        }
    }

    fn to_ll(&self) -> c_uint {
        match *self {
            EyeLeft => ll::Eye_Left,
            EyeRight => ll::Eye_Right
        }
    }
}

pub struct PerEye<T> {
    pub left: T,
    pub right: T
}

impl<T> PerEye<T> {
    pub fn new(left: T, right: T) -> PerEye<T> {
        PerEye {
            left: left,
            right: right
        }
    }

    pub fn eye<'a>(&'a self, eye: EyeType) -> &'a T {
        match eye {
            EyeLeft => &self.left,
            EyeRight => &self.right
        }
    }

    pub fn map<U>(&self, f: |EyeType, &T| -> U) -> PerEye<U> {
        PerEye::new(
            f(EyeLeft, &self.left),
            f(EyeRight, &self.right)
        )
    }

    pub unsafe fn ptr(&self) -> *T {
        &self.left as *T
    }

    pub unsafe fn mut_ptr(&mut self) -> *mut T {
        &mut self.left as *mut T
    }
}

pub struct HmdDescriptionEye {
    pub default_eye_fov: FovPort,
    pub max_eye_fov: FovPort,
}

pub struct HmdDescription {
    pub hmd_type: HmdType,
    pub product_name: ~str,
    pub manufacture: ~str,
    pub hmd_capabilities: HmdCapabilities,
    pub sensor_capabilities: SensorCapabilities,
    pub distortion_capabilities: DistortionCapabilities,
    pub resolution: ll::Sizei,
    pub window_position: ll::Vector2i,
    pub eye_fovs: PerEye<HmdDescriptionEye>,
    pub eye_render_order: [EyeType, ..2],
    pub display_device_name: ~str,
    pub display_id: c_int
}

impl HmdDescription {
    fn from_ll(sd: ll::HmdDesc) -> HmdDescription {
        unsafe {
            HmdDescription {
                hmd_type: HmdType::from_ll(sd.hmd_type),
                product_name: from_c_str(sd.product_name),
                manufacture: from_c_str(sd.manufacture),
                hmd_capabilities: HmdCapabilities{
                    flags: sd.hmd_capabilities
                },
                sensor_capabilities: SensorCapabilities{
                    flags: sd.sensor_capabilities
                },
                distortion_capabilities: DistortionCapabilities{
                    flags: sd.distortion_capabilities
                },
                resolution: sd.resolution,
                window_position: sd.window_position,
                eye_fovs: PerEye::new(
                    HmdDescriptionEye {
                        default_eye_fov: FovPort::from_ll(sd.default_eye_fov[ll::Eye_Left as uint]),
                        max_eye_fov: FovPort::from_ll(sd.max_eye_fov[ll::Eye_Left as uint])
                    },
                    HmdDescriptionEye {
                        default_eye_fov: FovPort::from_ll(sd.default_eye_fov[ll::Eye_Right as uint]),
                        max_eye_fov: FovPort::from_ll(sd.max_eye_fov[ll::Eye_Right as uint])
                    }
                ),
                eye_render_order: [EyeType::from_ll(sd.eye_render_order[0]),
                                   EyeType::from_ll(sd.eye_render_order[1])],
                display_device_name: from_c_str(sd.display_device_name),
                display_id: sd.display_id
            }
        }
    }
}

pub struct EyeRenderDescriptor {
    pub eye: EyeType,
    pub fov: FovPort,
    pub distorted_viewport: ll::Recti,
    pub pixels_per_tan_angle_at_center: Vector2<f32>,
    pub view_adjust: Vector3<f32>
}

impl EyeRenderDescriptor {
    fn from_ll(d: &ll::EyeRenderDesc) -> EyeRenderDescriptor {
        EyeRenderDescriptor {
            eye: EyeType::from_ll(d.eye),
            fov: FovPort::from_ll(d.fov),
            distorted_viewport: d.distorted_viewport,
            pixels_per_tan_angle_at_center: 
                Vector2::new(d.pixels_per_tan_angle_at_center.x,
                             d.pixels_per_tan_angle_at_center.y),
            view_adjust: Vector3::new(d.view_adjust.x,
                                      d.view_adjust.y,
                                      d.view_adjust.z)
        }
    }
}

pub struct RenderGLConfig {
    pub size: ll::Sizei,
    pub multisample: int,
    pub display: Option<*c_void>,
    pub window: Option<*c_void>
}

pub trait ToRenderConfig {
    fn to_render_config(&self) -> ll::RenderApiConfig;
}

impl ToRenderConfig for RenderGLConfig {
    fn to_render_config(&self) -> ll::RenderApiConfig {
        ll::RenderApiConfig {
            header: ll::RenderApiConfigHeader {
                render_api_type: ll::RenderAPI_OpenGL,
                rt_size: self.size,
                multisample: self.multisample as c_int,
            },
            display: match self.display { Some(p) => p, None => ptr::null() },
            window: match self.window { Some(p) => p, None => ptr::null() },
            padd: [ptr::null(), ptr::null(), ptr::null(),
                   ptr::null(), ptr::null(), ptr::null()]
        }
    }
}

pub struct FrameTiming {
    pub delta_seconds: f32,
    pub this_frame_seconds: f64,
    pub timewarp_point_seconds: f64,
    pub next_frame_seconds: f64,
    pub scanout_midpoint_seconds: f64,
    pub eye_scanout_seconds: PerEye<f64>
}

impl FrameTiming {
    fn from_ll(old: ll::FrameTiming) -> FrameTiming {
        FrameTiming {
            delta_seconds: old.delta_seconds,
            this_frame_seconds: old.this_frame_seconds,
            timewarp_point_seconds: old.timewarp_point_seconds,
            next_frame_seconds: old.next_frame_seconds,
            scanout_midpoint_seconds: old.scanout_midpoint_seconds,
            eye_scanout_seconds: PerEye::new(old.eye_scanout_seconds[ll::Eye_Left as uint],
                                             old.eye_scanout_seconds[ll::Eye_Right as uint])
        }
    }
}

trait ToTexture {
    fn to_texture(&self) -> ll::Texture;
}

pub struct Texture {
    pub size: ll::Sizei,
    pub viewport: ll::Recti,
    pub texture: u32
}

impl Texture {
    pub fn new(width: int,
               height: int,
               viewport_x: int,
               viewport_y: int,
               viewport_width: int,
               viewport_height: int,
               opengl_texture: u32) -> Texture {
        Texture {
            size: ll::Sizei {
                x: width as i32,
                y: height as i32
            },
            viewport: ll::Recti {
                pos: ll::Vector2i {
                    x: viewport_x as i32,
                    y: viewport_y as i32
                },
                size: ll::Sizei {
                    x: viewport_width as i32,
                    y: viewport_height as i32
                }
            },
            texture: opengl_texture
        }
    }
}

impl ToTexture for Texture {
    fn to_texture(&self) -> ll::Texture {
        ll::Texture {
            header: ll::TextureHeader {
                render_api_type: ll::RenderAPI_OpenGL,
                size: self.size,
                viewport: self.viewport,
            },
            texture_id: self.texture,
            padd: [ptr::null(), ptr::null(), ptr::null(), ptr::null(),
                   ptr::null(), ptr::null(), ptr::null()]
        }
    }
}

pub struct FovPort {
    pub up: f32,
    pub down: f32,
    pub left: f32,
    pub right: f32
}

impl FovPort {
    fn from_ll(ll: ll::FovPort) -> FovPort {
        FovPort {
            up: ll.upTan as f32,
            down: ll.downTan as f32,
            left: ll.leftTan as f32,
            right: ll.rightTan as f32
        }
    }

    fn to_ll(&self) -> ll::FovPort {
        ll::FovPort {
            upTan: self.up as c_float,
            downTan: self.down as c_float,
            leftTan: self.left as c_float,
            rightTan: self.right as c_float
        }        
    }

    pub fn projection(&self, znear: f32, zfar: f32, right_handed: bool) -> Matrix4<f32> {
        unsafe {
            let mat = ll::ovrMatrix4f_Projection(self.to_ll(), znear, zfar, right_handed);
            to_mat4(mat)
        }
    }
}