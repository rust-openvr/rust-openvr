use openvr_sys;
use openvr_sys::Enum_ETrackedPropertyError::*;
use subsystems::*;

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
    pub fn device_class(&self) -> openvr_sys::Enum_ETrackedDeviceClass {
        unsafe {
            let system = * { system().unwrap().0 as *mut openvr_sys::Struct_VR_IVRSystem_FnTable};
            system.GetTrackedDeviceClass.unwrap()(self.index as u32)
        }
    }

    /// gets a propery as a string
    pub fn get_property_string(&self, property: openvr_sys::Enum_ETrackedDeviceProperty) -> Result<String, openvr_sys::Enum_ETrackedPropertyError> {
        unsafe {
            let system = * { system().unwrap().0 as *mut openvr_sys::Struct_VR_IVRSystem_FnTable};

            let val_out = String::with_capacity(256);
            let mut err = ETrackedPropertyError_TrackedProp_Success;

            let size = system.GetStringTrackedDeviceProperty.unwrap()(
                self.index as u32,
                property,
                val_out.as_ptr() as *mut i8,
                256,
                &mut err
            );

            if size > 0 {
                return Ok(String::from_raw_parts(val_out.as_ptr() as *mut _, (size - 1) as usize, (size - 1) as usize));
            } else {
                return Err(err);
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
