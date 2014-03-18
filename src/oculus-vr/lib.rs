#[crate_id = "oculus-vr#0.1"];
#[crate_type = "lib"];
#[feature(link_args)];

extern crate cgmath;

use std::libc::{c_float, time_t};
use std::c_str::ToCStr;

use cgmath::quaternion::Quat;
use cgmath::vector::Vec3;
use cgmath::matrix::{Matrix, Mat4, ToMat4};
use cgmath::projection::perspective;
use cgmath::transform::Transform3D;
use cgmath::angle::rad;

#[cfg(target_os = "linux")]
#[link(name="ovr_wrapper")]
#[link(name="ovr")]
#[link(name="stdc++")]
#[link(name="udev")]
#[link(name="Xinerama")]
extern {}

#[cfg(target_os = "macos")]
#[link(name="ovr_wrapper")]
#[link(name="OculusVR")]
#[link(name="stdc++")]
extern {}

pub mod ll {
    use std::libc::{c_uint, c_int, c_float, c_long, c_char, time_t, c_void};

    pub enum DeviceManager {}

    pub struct HMDInfo {
        horizontal_resolution: c_uint,
        vertical_resolution: c_uint,
        horizontal_screen_size: c_float,
        vertical_screen_size: c_float,
        vertical_screen_center: c_float,
        eye_to_screen_distance: c_float,
        lens_separation_distance: c_float,
        interpupillary_distance: c_float,
        distortion_k: [c_float, ..4],
        chroma_ab_correction: [c_float, ..4],
        desktop_x: c_int,
        desktop_y: c_int,
        display_device_name: [c_char, ..32],
        display_id: c_long

    }

    impl Clone for HMDInfo {
        fn clone(&self) -> HMDInfo
        {
            HMDInfo {
                horizontal_resolution: self.horizontal_resolution,
                vertical_resolution: self.vertical_resolution,
                horizontal_screen_size: self.horizontal_screen_size,
                vertical_screen_size: self.vertical_screen_size,
                vertical_screen_center: self.vertical_screen_center,
                eye_to_screen_distance: self.eye_to_screen_distance,
                lens_separation_distance: self.lens_separation_distance,
                interpupillary_distance: self.interpupillary_distance,
                distortion_k: self.distortion_k,
                chroma_ab_correction: self.chroma_ab_correction,
                desktop_x: self.desktop_x,
                desktop_y: self.desktop_y,
                display_device_name: self.display_device_name,
                display_id: self.display_id,
            }
        }
    }

    pub enum HMDDevice {}
    pub enum SensorDevice {}
    pub enum SensorFusion {}
    pub enum MessageHandler {}

    #[deriving(Clone)]
    pub struct Vector3f {x: c_float, y: c_float, z: c_float}

    #[deriving(Clone)]
    pub struct Quatf {x: c_float, y: c_float, z: c_float, w: c_float}

    #[deriving(Clone)]
    pub struct Matrix4f {m11: c_float, m12: c_float, m13: c_float, m14: c_float,
                         m21: c_float, m22: c_float, m23: c_float, m24: c_float,
                         m31: c_float, m32: c_float, m33: c_float, m34: c_float,
                         m41: c_float, m42: c_float, m43: c_float, m44: c_float}

    #[deriving(Clone)]
    pub enum MessageBodyFrame {}


    extern "C" {
        pub fn OVR_system_init();
        pub fn OVR_DeviceManager_Create() -> *DeviceManager;
        pub fn OVR_DeviceManager_EnumerateDevices(dm :*DeviceManager) -> *HMDDevice;
        pub fn OVR_DeviceManager_drop(dm: *DeviceManager);

        pub fn OVR_HMDDevice_GetDeviceInfo(hmd: *HMDDevice) -> HMDInfo;
        pub fn OVR_HMDDevice_GetSensor(hmd: *HMDDevice) -> *SensorDevice;

        pub fn OVR_MessageHandler(ptr: *c_void, cb: extern "C" fn(ptr: *c_void, msg: *MessageBodyFrame)) -> *MessageHandler;
        pub fn OVR_MessageHandler_move_ptr(mh: *MessageHandler) -> *c_void;
        pub fn OVR_MessageHandler_drop(mh: *MessageHandler);

        pub fn OVR_SensorDevice_SetMessageHandler(sd: *SensorDevice, mh: *MessageHandler);
        pub fn OVR_SensorDevice_drop(sd: *SensorDevice);

        pub fn OVR_SensorFusion() -> *SensorFusion;
        pub fn OVR_SensorFusion_AttachToSensor(sf: *SensorFusion, sd: *SensorDevice) -> bool;
        pub fn OVR_SensorFusion_IsAttachedToSensor(sf: *SensorFusion) -> bool;
        pub fn OVR_SensorFusion_GetOrientation(sf: *SensorFusion) -> Quatf;
        pub fn OVR_SensorFusion_GetPredictedOrientation(sf: *SensorFusion) -> Quatf;
        pub fn OVR_SensorFusion_GetPredictedOrientation_opt(sf: *SensorFusion, dt: c_float) -> Quatf;
        pub fn OVR_SensorFusion_GetAcceleration(sf: *SensorFusion) -> Vector3f;
        pub fn OVR_SensorFusion_GetAngularVelocity(sf: *SensorFusion) -> Vector3f;
        pub fn OVR_SensorFusion_GetMagnetometer(sf: *SensorFusion) -> Vector3f;
        pub fn OVR_SensorFusion_GetCalibratedMagnetometer(sf: *SensorFusion) -> Vector3f;
        pub fn OVR_SensorFusion_Reset(sf: *SensorFusion);
        pub fn OVR_SensorFusion_EnableMotionTracking(sf: *SensorFusion, enable: bool);
        pub fn OVR_SensorFusion_IsMotionTrackingEnabled(sf: *SensorFusion) -> bool;
        pub fn OVR_SensorFusion_GetPredictionDelta(sf: *SensorFusion) -> c_float;
        pub fn OVR_SensorFusion_SetPrediction(sf: *SensorFusion, dt: c_float, enable: bool);
        pub fn OVR_SensorFusion_SetPredictionEnabled(sf: *SensorFusion, enable: bool);
        pub fn OVR_SensorFusion_IsPredictionEnabled(sf: *SensorFusion) -> bool;
        pub fn OVR_SensorFusion_SetGravityEnabled(sf: *SensorFusion, enableGravity: bool);
        pub fn OVR_SensorFusion_IsGravityEnabled(sf: *SensorFusion) -> bool;
        pub fn OVR_SensorFusion_GetAccelGain(sf: *SensorFusion) -> c_float;
        pub fn OVR_SensorFusion_SetAccelGain(sf: *SensorFusion, ag: c_float);
        pub fn OVR_SensorFusion_SaveMagCalibration(sf: *SensorFusion, calibrationName: *c_char) -> bool;
        pub fn OVR_SensorFusion_LoadMagCalibration(sf: *SensorFusion, calibrationName: *c_char) -> bool;
        pub fn OVR_SensorFusion_SetYawCorrectionEnabled(sf: *SensorFusion, enable: bool);
        pub fn OVR_SensorFusion_IsYawCorrectionEnabled(sf: *SensorFusion) -> bool;
        pub fn OVR_SensorFusion_SetMagCalibration(sf: *SensorFusion, m: *Matrix4f);
        pub fn OVR_SensorFusion_GetMagCalibration(sf: *SensorFusion) -> Matrix4f;
        pub fn OVR_SensorFusion_GetMagCalibrationTime(sf: *SensorFusion) -> time_t;
        pub fn OVR_SensorFusion_HasMagCalibration(sf: *SensorFusion) -> bool;
        pub fn OVR_SensorFusion_ClearMagCalibration(sf: *SensorFusion);
        pub fn OVR_SensorFusion_ClearMagReferences(sf: *SensorFusion, rawMag: *Vector3f);
        pub fn OVR_SensorFusion_GetCalibratedMagValue(sf: *SensorFusion) -> Vector3f;
        pub fn OVR_SensorFusion_OnMessage(sf: *SensorFusion, msg: *MessageBodyFrame);
        //pub fn OVR_SensorFusion_SetDelegateMessageHandler(sf: *SensorFusion, handle: *MessageHandler)
        pub fn OVR_SensorFusion_drop(sf: *SensorFusion);
        
    }
}

pub fn init()
{
    unsafe {
        ll::OVR_system_init();
    }
}

pub struct DeviceManager {
    priv ptr: *ll::DeviceManager
}

impl Drop for DeviceManager {
    fn drop(&mut self)
    {
        unsafe {
            ll::OVR_DeviceManager_drop(self.ptr);
        }
    }
}

impl DeviceManager {
    pub fn new() -> Option<DeviceManager>
    {
        unsafe {
            let ptr = ll::OVR_DeviceManager_Create();

            if ptr.is_null() {
                None
            } else {
                Some(DeviceManager{
                    ptr: ptr
                })
            }
        }
    }

    pub fn enumerate(&self) -> Option<HMDDevice>
    {
        unsafe {
            let ptr = ll::OVR_DeviceManager_EnumerateDevices(self.ptr);

            if ptr.is_null() {
                None
            } else {
                Some(HMDDevice{
                    ptr: ptr
                })
            }
        }
    }
}

pub struct HMDDevice {
    priv ptr: *ll::HMDDevice
}

impl HMDDevice {
    pub fn get_info(&self) -> HMDInfo
    {
        unsafe {
            HMDInfo{
                dat: ll::OVR_HMDDevice_GetDeviceInfo(self.ptr)
            }
        }        
    }

    pub fn get_sensor(&self) -> Option<SensorDevice>
    {
        unsafe {
            let ptr = ll::OVR_HMDDevice_GetSensor(self.ptr);

            if ptr.is_null() {
                None
            } else {
                Some(SensorDevice{
                    ptr: ptr,
                    msg: None
                })
            }
        }        
    }
}

#[deriving(Clone)]
pub struct HMDInfo {
    priv dat: ll::HMDInfo
}

impl HMDInfo
{
    pub fn resolution(&self) -> (uint, uint)
    {
        (self.dat.horizontal_resolution as uint, self.dat.vertical_resolution as uint)
    }

    pub fn size(&self) -> (f32, f32)
    {
        (self.dat.horizontal_screen_size as f32, self.dat.vertical_screen_size as f32)
    }

    pub fn desktop(&self) -> (int, int)
    {
        (self.dat.desktop_x as int, self.dat.desktop_y as int)
    }

    pub fn vertical_center(&self) -> f32
    {
        self.dat.vertical_screen_center as f32
    }

    pub fn eye_to_screen_distance(&self) -> f32
    {
        self.dat.eye_to_screen_distance as f32
    }

    pub fn lens_separation_distance(&self) -> f32
    {
        self.dat.lens_separation_distance as f32
    }

    pub fn interpupillary_distance(&self) -> f32
    {
        self.dat.interpupillary_distance as f32
    }

    pub fn distortion_K(&self) -> [f32, ..4]
    {
        [self.dat.distortion_k[0] as f32,
         self.dat.distortion_k[1] as f32,
         self.dat.distortion_k[2] as f32,
         self.dat.distortion_k[3] as f32]
    }

    pub fn chroma_ab_correction(&self) -> [f32, ..4]
    {
        [self.dat.chroma_ab_correction[0] as f32,
         self.dat.chroma_ab_correction[1] as f32,
         self.dat.chroma_ab_correction[2] as f32,
         self.dat.chroma_ab_correction[3] as f32]
    }

    pub fn name(&self) -> ~str
    {
        unsafe {
        std::str::raw::from_c_str(&self.dat.display_device_name[0])
        }
    }

    pub fn id(&self) -> int
    {
        self.dat.display_id as int
    }
}

pub struct SensorFusion {
    priv ptr: *ll::SensorFusion
}

impl Drop for SensorFusion {
    fn drop(&mut self) {
        unsafe {
            ll::OVR_SensorFusion_drop(self.ptr);
        }
    }
}

impl SensorFusion {
    pub fn new() -> Option<SensorFusion>
    {
        unsafe {
            let ptr = ll::OVR_SensorFusion();

            if ptr.is_null() {
                None
            } else {
                Some(SensorFusion{
                    ptr: ptr
                })
            }
        } 
    }

    pub fn attach_to_sensor(&self, sensor: &SensorDevice) -> bool
    {
        unsafe {
            ll::OVR_SensorFusion_AttachToSensor(self.ptr, sensor.ptr)
        } 
    }

    pub fn is_attached_to_sensor(&self) -> bool
    {
        unsafe {
            ll::OVR_SensorFusion_IsAttachedToSensor(self.ptr)
        }
    }

    pub fn get_orientation(&self) -> Quat<f32>
    {
        unsafe {
            let out = ll::OVR_SensorFusion_GetOrientation(self.ptr);
            Quat::new(out.w, out.x, out.y, out.z)
        }
    }

    pub fn get_predicted_orientation(&self, dt: Option<f32>) -> Quat<f32>
    {
        unsafe {
            let out = match dt {
                Some(dt) => ll::OVR_SensorFusion_GetPredictedOrientation_opt(self.ptr, dt as c_float),
                None => ll::OVR_SensorFusion_GetPredictedOrientation(self.ptr)
            };
            Quat::new(out.w, out.x, out.y, out.z)
        }
    }

    pub fn get_acceleration(&self) -> Vec3<f32>
    {
        unsafe {
            let out = ll::OVR_SensorFusion_GetAcceleration(self.ptr);
            Vec3::new(out.x, out.y, out.z)
        }
    }

    pub fn get_angular_velocity(&self) -> Vec3<f32>
    {
        unsafe {
            let out = ll::OVR_SensorFusion_GetAngularVelocity(self.ptr);
            Vec3::new(out.x, out.y, out.z)
        }
    }

    pub fn get_magnetometer(&self) -> Vec3<f32>
    {
        unsafe {
            let out = ll::OVR_SensorFusion_GetMagnetometer(self.ptr);
            Vec3::new(out.x, out.y, out.z)
        }
    }

    pub fn get_calibrated_magnetometer(&self) -> Vec3<f32>
    {
        unsafe {
            let out = ll::OVR_SensorFusion_GetCalibratedMagnetometer(self.ptr);
            Vec3::new(out.x, out.y, out.z)
        }
    }

    pub fn reset(&self)
    {
        unsafe {
            ll::OVR_SensorFusion_Reset(self.ptr);
        }
    }

    pub fn enable_motion_tracking(&self, enable: bool)
    {
        unsafe {
            ll::OVR_SensorFusion_EnableMotionTracking(self.ptr, enable)
        }
    }

    pub fn is_motion_tracking_enabled(&self) -> bool
    {
        unsafe {
            ll::OVR_SensorFusion_IsMotionTrackingEnabled(self.ptr)
        }
    }

    pub fn get_prediction_delta(&self) -> f32
    {
        unsafe {
            ll::OVR_SensorFusion_GetPredictionDelta(self.ptr) as f32
        }
    }

    pub fn set_prediction(&self, dt: f32, enable: bool)
    {
        unsafe {
            ll::OVR_SensorFusion_SetPrediction(self.ptr, dt as c_float, enable)
        }
    }

    pub fn set_prediction_enabled(&self, enable: bool)
    {
        unsafe {
            ll::OVR_SensorFusion_SetPredictionEnabled(self.ptr, enable)
        }
    }

    pub fn is_prediction_enabled(&self) -> bool
    {
        unsafe {
            ll::OVR_SensorFusion_IsPredictionEnabled(self.ptr)
        }       
    }

    pub fn set_gravity_enabled(&self, enable_gravity: bool)
    {
        unsafe {
            ll::OVR_SensorFusion_SetGravityEnabled(self.ptr, enable_gravity)
        }
    }

    pub fn is_gravity_enabled(&self) -> bool
    {
        unsafe {
            ll::OVR_SensorFusion_IsGravityEnabled(self.ptr)
        }
    }

    pub fn get_accel_gain(&self) -> f32
    {
        unsafe {
            ll::OVR_SensorFusion_GetAccelGain(self.ptr) as f32
        }
    }

    pub fn set_accel_gain(&self, ag: f32)
    {
        unsafe {
            ll::OVR_SensorFusion_SetAccelGain(self.ptr, ag as c_float)
        }
    }

    pub fn save_mag_calibration(&self, calibration_name: &str) -> bool
    {
        let cn = calibration_name.to_c_str();
        unsafe {
            ll::OVR_SensorFusion_SaveMagCalibration(self.ptr, cn.unwrap())
        }
    }

    pub fn load_mag_calibration(&self, calibration_name: &str) -> bool
    {
        let cn = calibration_name.to_c_str();
        unsafe {
            ll::OVR_SensorFusion_LoadMagCalibration(self.ptr, cn.unwrap())
        }
    }

    pub fn set_yaw_correction_enabled(&self, enable: bool)
    {
        unsafe {
            ll::OVR_SensorFusion_SetYawCorrectionEnabled(self.ptr, enable)
        }
    }

    pub fn is_yaw_correction_enabled(&self) -> bool
    {
        unsafe {
            ll::OVR_SensorFusion_IsYawCorrectionEnabled(self.ptr)
        }
    }

    pub fn set_mag_calibration(&self, m: &Mat4<f32>)
    {
        let mat = ll::Matrix4f{
            m11: m.x.x as c_float, m12: m.x.y as c_float, m13: m.x.z as c_float, m14: m.x.w as c_float,
            m21: m.y.x as c_float, m22: m.y.y as c_float, m23: m.y.z as c_float, m24: m.y.w as c_float,
            m31: m.z.x as c_float, m32: m.z.y as c_float, m33: m.z.z as c_float, m34: m.z.w as c_float,
            m41: m.w.x as c_float, m42: m.x.y as c_float, m43: m.x.z as c_float, m44: m.x.w as c_float
        };

        unsafe {
            ll::OVR_SensorFusion_SetMagCalibration(self.ptr, &mat as *ll::Matrix4f);
        }
    }

    pub fn get_mag_calibration(&self) -> Mat4<f32>
    {
        unsafe {
            let m = ll::OVR_SensorFusion_GetMagCalibration(self.ptr);

            Mat4::new(m.m11 as f32, m.m12 as f32, m.m13 as f32, m.m14 as f32,
                      m.m21 as f32, m.m22 as f32, m.m23 as f32, m.m24 as f32,
                      m.m31 as f32, m.m32 as f32, m.m33 as f32, m.m34 as f32,
                      m.m41 as f32, m.m42 as f32, m.m43 as f32, m.m44 as f32)
        }
    }

    /// TODO this should not return a time_t!
    pub fn get_mag_calibration_time(&self) -> time_t
    {
        unsafe {
            ll::OVR_SensorFusion_GetMagCalibrationTime(self.ptr)
        }
    }

    pub fn has_mag_calibration(&self) -> bool
    {
        unsafe {
            ll::OVR_SensorFusion_HasMagCalibration(self.ptr)
        }
    }

    pub fn clear_mag_calibration(&self)
    {
        unsafe {
            ll::OVR_SensorFusion_ClearMagCalibration(self.ptr)
        }
    }

    pub fn clear_mag_references(&self, vec: &Vec3<f32>)
    {
        let vec = ll::Vector3f{x: vec.x, y: vec.y, z: vec.z};

        unsafe {
            ll::OVR_SensorFusion_ClearMagReferences(self.ptr, &vec as *ll::Vector3f)
        }
    }

    pub fn get_calibrated_mag_value(&self) -> Vec3<f32>
    {
        unsafe {
            let vec = ll::OVR_SensorFusion_GetCalibratedMagValue(self.ptr);
            Vec3::new(vec.x, vec.y, vec.z)
        }
    }
}

pub struct SensorDevice {
    priv ptr: *ll::SensorDevice,
    priv msg: Option<*ll::MessageHandler>
}

impl Drop for SensorDevice
{
    fn drop(&mut self)
    {
        unsafe {
            ll::OVR_SensorDevice_drop(self.ptr); 
        }
    }
}


pub fn create_reference_matrices(hmd: &HMDInfo, view_center: &Mat4<f32>, scale: f32) -> ((Mat4<f32>, Mat4<f32>),
                                                                                         (Mat4<f32>, Mat4<f32>))
{
    let (h_res, v_res) = hmd.resolution();
    let (h_size, v_size) = hmd.size();
    let eye_to_screen = hmd.eye_to_screen_distance();
    let interpupillary_distance = hmd.interpupillary_distance();
    let lens_separation_distance = hmd.lens_separation_distance();

    let aspect_ratio = (h_res as f32 * 0.5) / (v_res as f32);
    let half_screen_distance = v_size / 2f32;
    let yfov = 2f32 * (half_screen_distance*scale/eye_to_screen).atan();

    let eye_project_shift = h_size * 0.25 - lens_separation_distance*0.5f32;
    let proj_off_center = 4f32 * eye_project_shift / h_size;

    let proj_center = perspective(rad(yfov), aspect_ratio, 0.1f32, 1000f32);
    let proj_left = Transform3D::new(1., Quat::zero(), Vec3::new(proj_off_center, 0., 0.)).get().to_mat4();
    let proj_right = Transform3D::new(1., Quat::zero(), Vec3::new(-proj_off_center, 0., 0.)).get().to_mat4();
    let proj_left = proj_left.mul_m(&proj_center);
    let proj_right = proj_right.mul_m(&proj_center);

    let halfIPD = interpupillary_distance / 2f32;

    let view_left = Transform3D::new(1., Quat::zero(), Vec3::new(halfIPD, 0., 0.)).get().to_mat4();
    let view_right = Transform3D::new(1., Quat::zero(), Vec3::new(-halfIPD, 0., 0.)).get().to_mat4();
    let view_left = view_left.mul_m(view_center);
    let view_right = view_right.mul_m(view_center);
    ((proj_left, proj_right),
     (view_left, view_right))

}

pub static SHADER_FRAG_CHROMAB: &'static str =
"#version 400
uniform vec2 LensCenter;
uniform vec2 screen_center;
uniform vec2 ScaleIn;
uniform vec2 ScaleOut;
uniform vec4 HmdWarpParam;
uniform vec4 ChromAbParam;
uniform sampler2D Texture0;

in vec2 TexPos;
out vec4 color;

void main()
{
    vec2 pos = vec2(TexPos.x, TexPos.y);
    
    vec2 theta = (pos - LensCenter) * ScaleIn;
    float rSq = theta.x * theta.x + theta.y * theta.y;
    vec2 theta1 = theta * (HmdWarpParam.x + HmdWarpParam.y * rSq +
                            HmdWarpParam.z * rSq * rSq +
                            HmdWarpParam.w * rSq * rSq * rSq);
    
    vec2 thetaBlue = theta1 * (ChromAbParam.z + ChromAbParam.w * rSq);
    vec2 tcBlue = LensCenter + ScaleOut * thetaBlue;


    if (!all(equal(clamp(tcBlue, screen_center-vec2(0.25,0.5), screen_center+vec2(0.25,0.5)), tcBlue))) {
        color = vec4(0.);
        return;
    }


    float blue = texture(Texture0, tcBlue).b;
    
    vec2  tcGreen = LensCenter + ScaleOut * theta1;
    vec4  center = texture(Texture0, tcGreen);
    
    vec2  thetaRed = theta1 * (ChromAbParam.x + ChromAbParam.y * rSq);
    vec2  tcRed = LensCenter + ScaleOut * thetaRed;
    float red = texture(Texture0, tcRed).r;
    
    color = vec4(red, center.g, blue, center.a);
}
";