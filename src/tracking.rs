use openvr_sys;
use openvr_sys::Enum_ETrackedPropertyError::*;

use subsystems::*;
use error::*;
use std::slice;
use std::str;

/// Describes a string property of a tracked device
#[derive(Debug, Copy, Clone)]
pub enum TrackedDeviceStringProperty {
    TrackingSystemName,
    ModelNumber,
    SerialNumber,
    RenderModelName,
    ManufacturerName,
    TrackingFirmwareVersion,
    HardwareRevision,
    AllWirelessDongleDescriptions,
    ConnectedWirelessDongle,
    FirmwareManualUpdateURL,
    FirmwareProgrammingTarget,
    DisplayMCImageLeft,
    DisplayMCImageRight,
    DisplayGCImage,
    CameraFirmwareDescription,
    AttachedDeviceId,
    ModeLabel
}

impl TrackedDeviceStringProperty {
    pub fn to_raw(&self) -> openvr_sys::Enum_ETrackedDeviceProperty {
        use openvr_sys::Enum_ETrackedDeviceProperty::*;
        use self::TrackedDeviceStringProperty::*;

        match *self {
            TrackingSystemName => ETrackedDeviceProperty_Prop_TrackingSystemName_String,
            ModelNumber => ETrackedDeviceProperty_Prop_ModelNumber_String,
            SerialNumber => ETrackedDeviceProperty_Prop_SerialNumber_String,
            RenderModelName => ETrackedDeviceProperty_Prop_RenderModelName_String,
            ManufacturerName => ETrackedDeviceProperty_Prop_ManufacturerName_String,
            TrackingFirmwareVersion => ETrackedDeviceProperty_Prop_TrackingFirmwareVersion_String,
            HardwareRevision => ETrackedDeviceProperty_Prop_HardwareRevision_String,
            AllWirelessDongleDescriptions => ETrackedDeviceProperty_Prop_AllWirelessDongleDescriptions_String,
            ConnectedWirelessDongle => ETrackedDeviceProperty_Prop_ConnectedWirelessDongle_String,
            FirmwareManualUpdateURL => ETrackedDeviceProperty_Prop_Firmware_ManualUpdateURL_String,
            FirmwareProgrammingTarget => ETrackedDeviceProperty_Prop_Firmware_ProgrammingTarget_String,
            DisplayMCImageLeft => ETrackedDeviceProperty_Prop_DisplayMCImageLeft_String,
            DisplayMCImageRight => ETrackedDeviceProperty_Prop_DisplayMCImageRight_String,
            DisplayGCImage => ETrackedDeviceProperty_Prop_DisplayGCImage_String,
            CameraFirmwareDescription => ETrackedDeviceProperty_Prop_CameraFirmwareDescription_String,
            AttachedDeviceId => ETrackedDeviceProperty_Prop_AttachedDeviceId_String,
            ModeLabel => ETrackedDeviceProperty_Prop_ModeLabel_String
        }
    }
}

/// Describes the class of a tracked device
#[derive(Debug, Copy, Clone)]
pub enum TrackedDeviceClass {
    Invalid,
    HMD,
    Controller,
    TrackingReference,
    Other,
}

impl TrackedDeviceClass {
    pub fn to_raw(&self) -> openvr_sys::Enum_ETrackedDeviceClass {
        use self::TrackedDeviceClass::*;
        use openvr_sys::Enum_ETrackedDeviceClass::*;

        match *self {
            Invalid => ETrackedDeviceClass_TrackedDeviceClass_Invalid,
            HMD => ETrackedDeviceClass_TrackedDeviceClass_HMD,
            Controller => ETrackedDeviceClass_TrackedDeviceClass_Controller,
            TrackingReference => ETrackedDeviceClass_TrackedDeviceClass_TrackingReference,
            Other => ETrackedDeviceClass_TrackedDeviceClass_Other,
        }
    }

    pub fn from_raw(raw: openvr_sys::Enum_ETrackedDeviceClass) -> Self {
        use self::TrackedDeviceClass::*;
        use openvr_sys::Enum_ETrackedDeviceClass::*;

        match raw {
            ETrackedDeviceClass_TrackedDeviceClass_Invalid => Invalid,
            ETrackedDeviceClass_TrackedDeviceClass_HMD => HMD,
            ETrackedDeviceClass_TrackedDeviceClass_Controller => Controller,
            ETrackedDeviceClass_TrackedDeviceClass_TrackingReference => TrackingReference,
            ETrackedDeviceClass_TrackedDeviceClass_Other => Other,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct TrackedDevicePose {
    pub index: usize,
    pub to_device: [[f32; 4]; 3],
    pub velocity: [f32; 3],
    pub angular_velocity: [f32; 3],
    pub is_valid: bool,
    pub is_connected: bool,
}

impl TrackedDevicePose {
    // returns the device class of the tracked object
    pub fn device_class(&self) -> TrackedDeviceClass {
        unsafe {
            let system = * { system().unwrap().0 as *mut openvr_sys::Struct_VR_IVRSystem_FnTable};
            TrackedDeviceClass::from_raw(system.GetTrackedDeviceClass.unwrap()(self.index as u32))
        }
    }

    /// gets a propery as a string
    pub fn get_property_string(&self, property: TrackedDeviceStringProperty) -> Result<String, Error<openvr_sys::Enum_ETrackedPropertyError>> {
        unsafe {
            let system = * { system().unwrap().0 as *mut openvr_sys::Struct_VR_IVRSystem_FnTable};

            let val_out = String::with_capacity(256);
            let mut err = ETrackedPropertyError_TrackedProp_Success;

            let size = system.GetStringTrackedDeviceProperty.unwrap()(
                self.index as u32,
                property.to_raw(),
                val_out.as_ptr() as *mut i8,
                256,
                &mut err
            );

            if size > 0 {
                let ptr = val_out.as_ptr() as *mut u8;
                let mem = slice::from_raw_parts(ptr, size as usize);
                let str = str::from_utf8(mem).unwrap();
                return Ok(String::from(str));
            } else {
                return Err(Error::from_raw(err));
            }
        }
    }
}


#[derive(Debug, Copy, Clone)]
pub struct TrackedDevicePoses {
    pub count: usize,
    pub poses: [TrackedDevicePose; 16],
}

pub struct TrackedDevicePosesIterator<'a> {
    pub target: &'a TrackedDevicePoses,
    pub index: usize
}

impl TrackedDevicePoses {
    pub fn as_slice(&self) -> &[TrackedDevicePose] {
        &self.poses[0..self.count]
    }

    /// creates an iterator that will iterate over all connected devices
    pub fn connected_iter(&self) -> TrackedDevicePosesIterator {
        TrackedDevicePosesIterator { target: self, index: 0 }
    }
}

impl<'a> Iterator for TrackedDevicePosesIterator<'a> {
    type Item = &'a TrackedDevicePose;

    fn next(&mut self) -> Option<&'a TrackedDevicePose> {
        // end reached
        if self.index == self.target.count {
            return None;
        }

        let res = &self.target.poses[self.index];
        self.index += 1;

        Some(res)
    }
}

pub unsafe fn to_tracked(data: [openvr_sys::TrackedDevicePose_t; 16]) -> TrackedDevicePoses {
    use std;
    let mut out: TrackedDevicePoses = std::mem::zeroed();
    for (i, d) in data.iter().enumerate() {
        if d.bDeviceIsConnected > 0 {
            out.count = i + 1;
        }
        out.poses[i].index = i;
        out.poses[i].is_connected = d.bDeviceIsConnected > 0;
        out.poses[i].is_valid = d.bPoseIsValid > 0;
        out.poses[i].to_device = d.mDeviceToAbsoluteTracking.m;
        out.poses[i].velocity = d.vVelocity.v;
        out.poses[i].angular_velocity = d.vAngularVelocity.v;
    }
    out
}
