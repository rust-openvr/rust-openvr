#[crate_id = "ovr-rs#0.1"];
#[crate_type = "lib"];
#[feature(link_args)];

extern crate cgmath;

use std::libc::{c_float, time_t, c_void};
use std::c_str::ToCStr;
use std::cast::transmute;

use cgmath::quaternion::Quat;
use cgmath::vector::Vec3;
use cgmath::matrix::{Matrix, Mat4, ToMat4};
use cgmath::projection::perspective;
use cgmath::transform::Transform3D;
use cgmath::angle::rad;

#[cfg(target_os = "linux")]
#[link(name="ovr_wrapper")]
#[link(name="OculusVR")]
#[link(name="stdc++")]
#[link(name="udev")]
#[link(name="Xinerama")]
#[link(name="edid")]
extern {}

pub mod ll {
    use std::libc::{c_uint, c_int, c_float, c_long, c_char, time_t, c_void};

    pub enum DeviceManager {}

    pub struct HMDInfo {
        HResolution: c_uint,
        VResolution: c_uint,
        HScreenSize: c_float,
        VScreenSize: c_float,
        VScreenCenter: c_float,
        EyeToScreenDistance: c_float,
        LensSeparationDistance: c_float,
        InterpupillaryDistance: c_float,
        DistortionK: [c_float, ..4],
        ChromaAbCorrection: [c_float, ..4],
        DesktopX: c_int,
        DesktopY: c_int,
        DisplayDeviceName: [c_char, ..32],
        DisplayId: c_long

    }

    impl Clone for HMDInfo {
        fn clone(&self) -> HMDInfo
        {
            HMDInfo {
                HResolution: self.HResolution,
                VResolution: self.VResolution,
                HScreenSize: self.HScreenSize,
                VScreenSize: self.VScreenSize,
                VScreenCenter: self.VScreenCenter,
                EyeToScreenDistance: self.EyeToScreenDistance,
                LensSeparationDistance: self.LensSeparationDistance,
                InterpupillaryDistance: self.InterpupillaryDistance,
                DistortionK: self.DistortionK,
                ChromaAbCorrection: self.ChromaAbCorrection,
                DesktopX: self.DesktopX,
                DesktopY: self.DesktopY,
                DisplayDeviceName: self.DisplayDeviceName,
                DisplayId: self.DisplayId,
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
    pub struct MessageBodyFrame {
        Acceleration: Vector3f,
        RotationRate: Vector3f,
        MagneticField: Vector3f,
        Temperature: f32,
        TimeDelta: f32,
    }

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
            println!("{:?}", self);
            HMDInfo{
                dat: ll::OVR_HMDDevice_GetDeviceInfo(self.ptr)
            }
        }        
    }

    pub fn get_sensor(&self) -> Option<SensorDevice>
    {
        unsafe {
            println!("{:?}", self);
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
        (self.dat.HResolution as uint, self.dat.VResolution as uint)
    }

    pub fn size(&self) -> (f32, f32)
    {
        (self.dat.HScreenSize as f32, self.dat.VScreenSize as f32)
    }

    pub fn desktop(&self) -> (int, int)
    {
        (self.dat.DesktopX as int, self.dat.DesktopY as int)
    }

    pub fn vertical_center(&self) -> f32
    {
        self.dat.VScreenCenter as f32
    }

    pub fn eye_to_screen_distance(&self) -> f32
    {
        self.dat.EyeToScreenDistance as f32
    }

    pub fn lens_separation_distance(&self) -> f32
    {
        self.dat.LensSeparationDistance as f32
    }

    pub fn interpupillary_distance(&self) -> f32
    {
        self.dat.InterpupillaryDistance as f32
    }

    pub fn distortion_K(&self) -> [f32, ..4]
    {
        [self.dat.DistortionK[0] as f32,
         self.dat.DistortionK[1] as f32,
         self.dat.DistortionK[2] as f32,
         self.dat.DistortionK[3] as f32]
    }

    pub fn chroma_ab_correction(&self) -> [f32, ..4]
    {
        [self.dat.ChromaAbCorrection[0] as f32,
         self.dat.ChromaAbCorrection[1] as f32,
         self.dat.ChromaAbCorrection[2] as f32,
         self.dat.ChromaAbCorrection[3] as f32]
    }

    pub fn name(&self) -> ~str
    {
        unsafe {
        std::str::raw::from_c_str(&self.dat.DisplayDeviceName[0])
        }
    }

    pub fn id(&self) -> int
    {
        self.dat.DisplayId as int
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

    pub fn on_message(&self, msg: &Message)
    {
        unsafe {
            match *msg {
                MsgBody(ref body) => {
                    ll::OVR_SensorFusion_OnMessage(self.ptr, transmute(body));
                }
            }
        }
    }
}

extern "C" fn chan_callback(chan: *c_void, msg: *ll::MessageBodyFrame)
{
    if chan.is_null() || msg.is_null() {
        return;
    }

    unsafe {
        let chan: &Chan<Message> = transmute(chan);
        let msg: &ll::MessageBodyFrame = transmute(msg);

        chan.send(MsgBody(MessageBody::from_raw(msg)));
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
            // free chan
            match self.msg {
                Some(msg) => {
                    let chan = ll::OVR_MessageHandler_move_ptr(msg);
                    let _: ~Chan<Message> = transmute(chan);
                    ll::OVR_MessageHandler_drop(msg);
                },
                None => ()
            }

            ll::OVR_SensorDevice_drop(self.ptr); 
        }
    }
}

impl SensorDevice {
    pub fn register_chan(&mut self, chan: ~Chan<Message>)
    {
        if self.msg.is_none() {
            unsafe {
                self.msg = Some(ll::OVR_MessageHandler(transmute(chan), chan_callback));
                let msg = self.msg.as_ref().unwrap();

                ll::OVR_SensorDevice_SetMessageHandler(self.ptr, *msg);
            }
        }
    }
}

struct MessageBody {
    acceleration: Vec3<f32>,
    rotation_rate: Vec3<f32>,
    magnetic_field: Vec3<f32>,
    temperature: f32,
    time_delta: f32
}

impl MessageBody {
    fn from_raw(mbf: &ll::MessageBodyFrame) -> MessageBody
    {
        unsafe {
            transmute(*mbf)
        }
    }
}

pub enum Message {
    MsgBody(MessageBody)
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
uniform vec2 ScreenCenter;
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


    if (!all(equal(clamp(tcBlue, ScreenCenter-vec2(0.25,0.5), ScreenCenter+vec2(0.25,0.5)), tcBlue))) {
        color = vec4(0.);
        return;
    }


    float blue = texture2D(Texture0, tcBlue).b;
    
    vec2  tcGreen = LensCenter + ScaleOut * theta1;
    vec4  center = texture2D(Texture0, tcGreen);
    
    vec2  thetaRed = theta1 * (ChromAbParam.x + ChromAbParam.y * rSq);
    vec2  tcRed = LensCenter + ScaleOut * thetaRed;
    float red = texture2D(Texture0, tcRed).r;
    
    color = vec4(red, center.g, blue, center.a);
}
";