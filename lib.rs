#[crate_id = "ovr-rs#0.1"];
#[crate_type = "lib"];
#[feature(link_args)];

use std::ptr;

#[link(name="ovr_wrapper")]
extern {}

#[link(name="ovr")]
extern {}

mod ll {
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
