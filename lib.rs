#[crate_id = "ovr-rs#0.1"];
#[crate_type = "lib"];
#[feature(link_args)];

pub enum DeviceManager {}
pub enum HMDInfo {}
pub enum HMDDevice {}
pub enum SensorDevice {}
pub enum SensorFusion {}


#[link(name="ovr_wrapper")]
extern {}

#[link(name="ovr")]
extern {}

extern "C" {
    pub fn OVR_system_init();
    pub fn OVR_DeviceManager_Create() -> *DeviceManager;
    pub fn OVR_DeviceManager_EnumerateDevices(dm :*DeviceManager) -> *HMDDevice;
    pub fn OVR_HDMDevice_GetDeviceInfo(hmd: *HMDDevice) -> *HMDInfo;
    pub fn OVR_HDMDevice_GetSensor(hmd: *HMDDevice) -> *SensorDevice;
    pub fn OVR_SensorFusion() -> *SensorFusion;
    pub fn OVR_SensorFusion_AttachToSensor(sf: *SensorFusion, sd: *SensorDevice) -> bool;
}