#[crate_id = "ovr-rs#0.1"];
#[crate_type = "lib"];
#[feature(link_args)];

extern mod cgmath;

#[link(name="ovr_wrapper")]
extern {}

#[link(name="ovr")]
extern {}

mod ll {
    use std::libc::{c_uint, c_int, c_float, c_long, c_char};

    pub enum DeviceManager {}
    pub enum HMDInfo {}
    pub enum HMDDevice {}
    pub enum SensorDevice {}
    pub enum SensorFusion {}

    extern "C" {
        pub fn OVR_system_init();
        pub fn OVR_DeviceManager_Create() -> *DeviceManager;
        pub fn OVR_DeviceManager_EnumerateDevices(dm :*DeviceManager) -> *HMDDevice;
        pub fn OVR_HDMDevice_GetDeviceInfo(hmd: *HMDDevice) -> *HMDInfo;
        pub fn OVR_HDMDevice_GetSensor(hmd: *HMDDevice) -> *SensorDevice;
        pub fn OVR_SensorFusion() -> *SensorFusion;
        pub fn OVR_SensorFusion_AttachToSensor(sf: *SensorFusion, sd: *SensorDevice) -> bool;
        
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
}

pub struct SensorDevice {
    priv ptr: *ll::SensorDevice
}
