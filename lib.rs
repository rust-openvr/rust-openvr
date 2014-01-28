#[crate_id = "ovr-rs#0.1"];
#[crate_type = "lib"];
#[feature(link_args)];

extern mod cgmath;

use std::libc::{c_float, time_t};
use std::c_str::ToCStr;

use cgmath::quaternion::Quat;
use cgmath::vector::Vec3;
use cgmath::matrix::Mat4;

#[link(name="ovr_wrapper")]
extern {}

#[link(name="ovr")]
extern {}

pub mod ll {
    use std::libc::{c_uint, c_int, c_float, c_long, c_char, time_t};

    pub enum DeviceManager {}
    pub enum HMDInfo {}
    pub enum HMDDevice {}
    pub enum SensorDevice {}
    pub enum SensorFusion {}

    pub struct Vector3f {x: c_float, y: c_float, z: c_float}
    pub struct Quatf {x: c_float, y: c_float, z: c_float, w: c_float}
    pub struct Matrix4f {m11: c_float, m12: c_float, m13: c_float, m14: c_float,
                         m21: c_float, m22: c_float, m23: c_float, m24: c_float,
                         m31: c_float, m32: c_float, m33: c_float, m34: c_float,
                         m41: c_float, m42: c_float, m43: c_float, m44: c_float}

    extern "C" {
        pub fn OVR_system_init();
        pub fn OVR_DeviceManager_Create() -> *DeviceManager;
        pub fn OVR_DeviceManager_EnumerateDevices(dm :*DeviceManager) -> *HMDDevice;
        pub fn OVR_HDMDevice_GetDeviceInfo(hmd: *HMDDevice) -> *HMDInfo;
        pub fn OVR_HDMDevice_GetSensor(hmd: *HMDDevice) -> *SensorDevice;

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
        //pub fn OVR_SensorFusion_OnMessage(sf: *SensorFusion, msg: *MessageBodyFrame)
        //pub fn OVR_SensorFusion_SetDelegateMessageHandler(sf: *SensorFusion, handle: *MessageHandler)
        
        pub fn OVR_HMDInfo_GetScreenHResolution(info: *HMDInfo) -> c_uint;
        pub fn OVR_HMDInfo_GetScreenVResolution(info: *HMDInfo) -> c_uint;
        pub fn OVR_HMDInfo_GetHScreenSize(info: *HMDInfo) -> c_float;
        pub fn OVR_HMDInfo_GetVScreenSize(info: *HMDInfo) -> c_float;
        pub fn OVR_HMDInfo_GetVScreenCenter(info: *HMDInfo) -> c_float;
        pub fn OVR_HMDInfo_GetEyeToScreenDistance(info: *HMDInfo) -> c_float;
        pub fn OVR_HMDInfo_GetLensSeparationDistance(info: *HMDInfo) -> c_float;
        pub fn OVR_HMDInfo_GetInterpupillaryDistance(info: *HMDInfo) -> c_float;
        pub fn OVR_HMDInfo_GetDistortionK(info: *HMDInfo, idx: c_int) -> c_float;
        pub fn OVR_HMDInfo_GetChromaAbCorrection(info: *HMDInfo, idx: c_int) -> c_float;
        pub fn OVR_HMDInfo_GetDesktopX(info: *HMDInfo) -> c_uint;
        pub fn OVR_HMDInfo_GetDesktopY(info: *HMDInfo) -> c_uint;
        pub fn OVR_HMDInfo_GetDisplayDeviceName(info: *HMDInfo) -> *c_char;
        pub fn OVR_HMDInfo_GetDisplayId(info: *HMDInfo) -> c_long;
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
    pub fn get_info(&self) -> Option<HMDInfo>
    {
        unsafe {
            let ptr = ll::OVR_HDMDevice_GetDeviceInfo(self.ptr);

            if ptr.is_null() {
                None
            } else {
                Some(HMDInfo{
                    ptr: ptr
                })
            }
        }        
    }

    pub fn get_sensor(&self) -> Option<SensorDevice>
    {
        unsafe {
            let ptr = ll::OVR_HDMDevice_GetSensor(self.ptr);

            if ptr.is_null() {
                None
            } else {
                Some(SensorDevice{
                    ptr: ptr
                })
            }
        }        
    }
}

pub struct HMDInfo {
    priv ptr: *ll::HMDInfo
}

impl HMDInfo
{
    pub fn resolution(&self) -> (uint, uint)
    {
        unsafe {
            let h = ll::OVR_HMDInfo_GetScreenHResolution(self.ptr);
            let v = ll::OVR_HMDInfo_GetScreenVResolution(self.ptr);

            (h as uint, v as uint)
        }
    }

    pub fn size(&self) -> (f32, f32)
    {
        unsafe {
            let h = ll::OVR_HMDInfo_GetHScreenSize(self.ptr);
            let v = ll::OVR_HMDInfo_GetVScreenSize(self.ptr);

            (h as f32, v as f32)
        }
    }

    pub fn desktop(&self) -> (int, int)
    {
        unsafe {
            let h = ll::OVR_HMDInfo_GetDesktopX(self.ptr);
            let v = ll::OVR_HMDInfo_GetDesktopY(self.ptr);

            (h as int, v as int)
        }
    }

    pub fn vertical_center(&self) -> f32
    {
        unsafe {
            ll::OVR_HMDInfo_GetVScreenCenter(self.ptr) as f32
        }
    }

    pub fn eye_to_screen_distance(&self) -> f32
    {
        unsafe {
            ll::OVR_HMDInfo_GetEyeToScreenDistance(self.ptr) as f32
        }
    }

    pub fn lens_separation_distance(&self) -> f32
    {
        unsafe {
            ll::OVR_HMDInfo_GetLensSeparationDistance(self.ptr) as f32
        }
    }

    pub fn interpupillary_distance(&self) -> f32
    {
        unsafe {
            ll::OVR_HMDInfo_GetInterpupillaryDistance(self.ptr) as f32
        }
    }

    pub fn distortion_K(&self) -> [f32, ..4]
    {
        unsafe {
            [ll::OVR_HMDInfo_GetDistortionK(self.ptr, 0) as f32,
             ll::OVR_HMDInfo_GetDistortionK(self.ptr, 1) as f32,
             ll::OVR_HMDInfo_GetDistortionK(self.ptr, 2) as f32,
             ll::OVR_HMDInfo_GetDistortionK(self.ptr, 3) as f32]
        }
    }

    pub fn chroma_ab_correction(&self) -> [f32, ..4]
    {
        unsafe {
            [ll::OVR_HMDInfo_GetChromaAbCorrection(self.ptr, 0) as f32,
             ll::OVR_HMDInfo_GetChromaAbCorrection(self.ptr, 1) as f32,
             ll::OVR_HMDInfo_GetChromaAbCorrection(self.ptr, 2) as f32,
             ll::OVR_HMDInfo_GetChromaAbCorrection(self.ptr, 3) as f32]
        }
    }

    pub fn name(&self) -> ~str
    {
        unsafe {
            std::str::raw::from_c_str(ll::OVR_HMDInfo_GetDisplayDeviceName(self.ptr))
        }
    }

    pub fn id(&self) -> int
    {
        unsafe {
            ll::OVR_HMDInfo_GetDisplayId(self.ptr) as int
        }
    }
}

pub struct SensorFusion {
    priv ptr: *ll::SensorFusion
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
            Quat::new(out.x, out.y, out.z, out.w)
        }
    }

    pub fn get_predicted_orientation(&self, dt: Option<f32>) -> Quat<f32>
    {
        unsafe {
            let out = match dt {
                Some(dt) => ll::OVR_SensorFusion_GetPredictedOrientation_opt(self.ptr, dt as c_float),
                None => ll::OVR_SensorFusion_GetPredictedOrientation(self.ptr)
            };
            Quat::new(out.x, out.y, out.z, out.w)
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
    priv ptr: *ll::SensorDevice
}
