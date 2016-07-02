use openvr_sys;

use tracking::*;
use error::*;
use subsystems::*;

pub struct IVRTrackedCamera(pub *const ());

#[derive(Debug, Copy, Clone)]
pub struct CameraFrameSize {
    pub width: u32,
    pub height: u32,
    pub buffer: u32
}

#[derive(Debug, Copy, Clone)]
pub enum CameraFrameType {
    Distorted,
    Undistorted,
    MaximumUndistorted,
    MaximumFrameTypes
}

#[derive(Debug, Copy, Clone)]
pub struct CameraIntriniscs {
    pub focal_length: [f32; 2],
    pub center: [f32; 2]
}

#[derive(Debug, Copy, Clone)]
pub struct CameraFrameHeader {
    pub width: u32,
    pub height: u32,
    pub bytes_per_pixel: u32,
    pub frame_sequence: u32,
    pub pose: TrackedDevicePose
}

#[derive(Debug)]
pub struct CameraFrame {
    pub framebuffer: Vec<u8>,
    pub header: CameraFrameHeader
}

pub struct CameraStream {
    pub handle: u64,
    pub owner: TrackedDevicePose
}

impl Drop for CameraStream {
    fn drop(&mut self) {
        unsafe {
            let cam = *{ tracked_camera().unwrap().0 as *mut openvr_sys::VR_IVRTrackedCamera_FnTable };

            let error = Error::from_raw(
                cam.ReleaseVideoStreamingService.unwrap()(self.handle));

            if error.is_err() {
                println!("Failed to drop camera stream! Possible memory leak! {}", error.message());
            }
        }
    }
}

impl CameraStream {
    /// reads current camera frame
    pub fn read(&self, ctype: CameraFrameType) -> Result<CameraFrame, Error<openvr_sys::EVRTrackedCameraError>> {
        use std::mem;
        use std;

        unsafe {
            // get subsystems
            let cam = *{ tracked_camera().unwrap().0 as *mut openvr_sys::VR_IVRTrackedCamera_FnTable };
            let size = tracked_camera().unwrap().frame_size(&self.owner, ctype).unwrap();

            // create raw buffer where openvr can store it's data into
            let mut buffer = Vec::<u8>::with_capacity(size.buffer as usize);
            let raw_buffer = buffer.as_mut_ptr();
            mem::forget(buffer);

            // create header
            let mut header = openvr_sys::CameraVideoStreamFrameHeader_t::default();

            let error = Error::from_raw(
                cam.GetVideoStreamFrameBuffer.unwrap()(
                    self.handle,
                    ctype.to_raw(),
                    raw_buffer as *mut std::os::raw::c_void,
                    size.buffer,
                    &mut header,
                    mem::size_of::<openvr_sys::CameraVideoStreamFrameHeader_t>() as u32
                ));

            if error.is_ok() {
                // bring framebuffer back into rusts controll
                let buffer = Vec::from_raw_parts(raw_buffer, size.buffer as usize, size.buffer as usize);

                return Ok(CameraFrame {
                    framebuffer: buffer,
                    header: CameraFrameHeader {
                        width: header.nWidth,
                        height: header.nHeight,
                        bytes_per_pixel: header.nBytesPerPixel,
                        frame_sequence: header.nFrameSequence,
                        pose: TrackedDevicePose::from_raw(self.owner.index, header.standingTrackedDevicePose)
                    }
                });
            } else {
                return Err(error);
            }
        }
    }
}

impl CameraFrameType {
    pub fn to_raw(&self) -> openvr_sys::EVRTrackedCameraFrameType {
        use openvr_sys::EVRTrackedCameraFrameType::*;

        match self {
            &CameraFrameType::Distorted => EVRTrackedCameraFrameType_VRTrackedCameraFrameType_Distorted,
            &CameraFrameType::Undistorted => EVRTrackedCameraFrameType_VRTrackedCameraFrameType_Undistorted,
            &CameraFrameType::MaximumUndistorted => EVRTrackedCameraFrameType_VRTrackedCameraFrameType_MaximumUndistorted ,
            &CameraFrameType::MaximumFrameTypes => EVRTrackedCameraFrameType_MAX_CAMERA_FRAME_TYPES
        }
    }
}

impl IVRTrackedCamera {
    pub unsafe fn from_raw(ptr: *const ()) -> Self {
        IVRTrackedCamera(ptr as *mut ())
    }

    /// checks whether the current system has a camera
    pub fn has_camera(&self, device: &TrackedDevicePose) -> Result<bool, Error<openvr_sys::EVRTrackedCameraError>> {
        unsafe {
            let cam = * { self.0 as *mut openvr_sys::VR_IVRTrackedCamera_FnTable };
            let mut has_cam = 0i32;

            let error = Error::from_raw(
                cam.HasCamera.unwrap()(device.index as u32, &mut has_cam as *mut i32));

            if error.is_ok() {
                return Ok(has_cam > 0i32);
            } else {
                return Err(error);
            }
        }
    }

    /// gets frame buffer information of camera
    pub fn frame_size(&self, device: &TrackedDevicePose, ctype: CameraFrameType)
        -> Result<CameraFrameSize, Error<openvr_sys::EVRTrackedCameraError>>
    {
        unsafe {
            let mut result = CameraFrameSize {
                width: 0,
                height: 0,
                buffer: 0,
            };

            let cam = *{ self.0 as *mut openvr_sys::VR_IVRTrackedCamera_FnTable };

            let error = Error::from_raw(
                cam.GetCameraFrameSize.unwrap()(device.index as u32,
                                       ctype.to_raw(),
                                       &mut result.width,
                                       &mut result.height,
                                       &mut result.buffer));

            if error.is_ok() {
                return Ok(result);
            } else {
                return Err(error);
            }
        }
    }

    // gets camera intrinsic
    pub fn intrinisics(&self, device: &TrackedDevicePose, ctype: CameraFrameType)
        -> Result<CameraIntriniscs, Error<openvr_sys::EVRTrackedCameraError>>
    {
        unsafe {
            let mut focal = openvr_sys::HmdVector2_t { v: [0.0, 0.0] };
            let mut center = openvr_sys::HmdVector2_t { v: [0.0, 0.0] };

            let cam = *{ self.0 as *mut openvr_sys::VR_IVRTrackedCamera_FnTable };

            let error = Error::from_raw(
                cam.GetCameraIntrinisics.unwrap()(device.index as u32,
                                                ctype.to_raw(),
                                                &mut focal,
                                                &mut center));

            if error.is_ok() {
                return Ok(CameraIntriniscs {
                    focal_length: focal.v,
                    center: center.v
                });
            } else {
                return Err(error);
            }
        }
    }

    /// aquires a stream to the given camera device
    pub fn stream(&self, device: &TrackedDevicePose) -> Result<CameraStream, Error<openvr_sys::EVRTrackedCameraError>> {
        unsafe {
            let cam = *{ self.0 as *mut openvr_sys::VR_IVRTrackedCamera_FnTable };
            let mut handle = 0u64;

            let error = Error::from_raw(
                cam.AcquireVideoStreamingService.unwrap()(device.index as u32, &mut handle));

            if error.is_ok() {
                return Ok(CameraStream {
                            handle: handle,
                            owner: *device
                        });
            } else {
                return Err(error);
            }
        }
    }
}
