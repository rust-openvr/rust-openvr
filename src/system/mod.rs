//! The `System` interface provides access to display configuration information, tracking data, controller state,
//! events, and device properties. It is the main interface of OpenVR.

use std::{mem, ptr, slice, marker::PhantomData};

use openvr_sys as sys;

pub mod event;

use super::*;
pub use self::event::{Event, EventInfo};

impl System {
    /// Provides the game with the minimum size that it should use for its offscreen render target to minimize pixel
    /// stretching. This size is matched with the projection matrix and distortion function and will change from display
    /// to display depending on resolution, distortion, and field of view.
    pub fn recommended_render_target_size(&self) -> (u32, u32) {
        unsafe {
            let mut result: (u32, u32) = mem::uninitialized();
            self.0.GetRecommendedRenderTargetSize.unwrap()(&mut result.0, &mut result.1);
            result
        }
    }

    /// Returns the projection matrix to use for the specified eye.
    ///
    /// Clip plane distances are in meters.
    pub fn projection_matrix(&self, eye: Eye, near_z: f32, far_z: f32) -> [[f32; 4]; 4] {
        unsafe { self.0.GetProjectionMatrix.unwrap()(eye as sys::EVREye, near_z, far_z) }.m
    }

    /// Returns the raw project values to use for the specified eye. Most games should use GetProjectionMatrix instead
    /// of this method, but sometimes a game needs to do something fancy with its projection and can use these values to
    /// compute its own matrix.
    pub fn projection_raw(&self, eye: Eye) -> RawProjection {
        unsafe {
            let mut result: RawProjection = mem::uninitialized();
            self.0.GetProjectionRaw.unwrap()(
                eye as sys::EVREye,
                &mut result.left,
                &mut result.right,
                &mut result.top,
                &mut result.bottom,
            );
            result
        }
    }

    /// Returns the transform between the view space and eye space. Eye space is the per-eye flavor of view space that
    /// provides stereo disparity. Instead of Model * View * Projection the model is Model * View * Eye *
    /// Projection. Normally View and Eye will be multiplied together and treated as View in your application.
    pub fn eye_to_head_transform(&self, eye: Eye) -> [[f32; 4]; 3] {
        unsafe { (self.0.GetEyeToHeadTransform.unwrap())(eye as sys::EVREye) }.m
    }

    /// Returns the number of elapsed seconds since the last recorded vsync event and the global number of frames that
    /// have been rendered. Timing information will come from a vsync timer event in the timer if possible or from the
    /// application-reported time if that is not available. If no vsync times are available the function will return
    /// None.
    pub fn time_since_last_vsync(&self) -> Option<(f32, u64)> {
        unsafe {
            let mut result: (f32, u64) = mem::uninitialized();
            if self.0.GetTimeSinceLastVsync.unwrap()(&mut result.0, &mut result.1) {
                Some(result)
            } else {
                None
            }
        }
    }

    /// Calculates updated poses for all devices.
    ///
    /// The pose that the tracker thinks that the HMD will be in at the specified number of seconds into the
    /// future. Pass 0 to get the state at the instant the method is called. Most of the time the application should
    /// calculate the time until the photons will be emitted from the display and pass that time into the method.
    ///
    /// This is roughly analogous to the inverse of the view matrix in most applications, though many games will need to
    /// do some additional rotation or translation on top of the rotation and translation provided by the head pose.
    ///
    /// Seated experiences should call this method with TrackingUniverseSeated and receive poses relative to the seated
    /// zero pose. Standing experiences should call this method with TrackingUniverseStanding and receive poses relative
    /// to the chaperone soft bounds. TrackingUniverseRawAndUncalibrated should probably not be used unless the
    /// application is the chaperone calibration tool itself, but will provide poses relative to the hardware-specific
    /// coordinate system in the driver.
    pub fn device_to_absolute_tracking_pose(
        &self,
        origin: TrackingUniverseOrigin,
        predicted_seconds_to_photons_from_now: f32,
    ) -> TrackedDevicePoses {
        unsafe {
            let mut result: TrackedDevicePoses = mem::uninitialized();
            self.0.GetDeviceToAbsoluteTrackingPose.unwrap()(
                origin as sys::ETrackingUniverseOrigin,
                predicted_seconds_to_photons_from_now,
                result.as_mut().as_mut_ptr() as *mut _,
                result.len() as u32,
            );
            result
        }
    }

    pub fn poll_next_event_with_pose(
        &self,
        origin: TrackingUniverseOrigin,
    ) -> Option<(EventInfo, TrackedDevicePose)> {
        let mut event = unsafe { mem::uninitialized() };
        let mut pose = unsafe { mem::uninitialized() };
        if unsafe {
            self.0.PollNextEventWithPose.unwrap()(
                origin as sys::ETrackingUniverseOrigin,
                &mut event,
                mem::size_of_val(&event) as u32,
                &mut pose as *mut _ as *mut _,
            )
        } {
            Some(((event, self).into(), pose))
        } else {
            None
        }
    }

    /// Computes the distortion caused by the optics
    /// Gets the result of a single distortion value for use in a distortion map. Input UVs are in a single eye's viewport, and output UVs are for the source render target in the distortion shader.
    pub fn compute_distortion(&self, eye: Eye, u: f32, v: f32) -> Option<DistortionCoordinates> {
        let mut coord = unsafe { mem::uninitialized() };
        let success =
            unsafe { self.0.ComputeDistortion.unwrap()(eye as sys::EVREye, u, v, &mut coord) };

        if !success {
            return None;
        }

        Some(DistortionCoordinates {
            red: coord.rfRed,
            blue: coord.rfBlue,
            green: coord.rfGreen,
        })
    }

    /// Returns the tracked device associated with a specific role, for example the left hand or the right hand.
    pub fn tracked_device_for_controller_role(
        &self,
        role: TrackedControllerRole,
    ) -> Option<TrackedDevice> {
        let x = unsafe {
            self.0.GetTrackedDeviceIndexForControllerRole.unwrap()(
                role as sys::ETrackedControllerRole,
            )
        };
        if x == tracked_device_index::INVALID {
            None
        } else {
            Some(TrackedDevice::from_system_and_index(self, x))
        }
    }

    /// Returns the output device used by OpenVR.
    ///
    /// # Safety
    /// The instance handle must be valid.
    pub unsafe fn vulkan_output_device(
        &self,
        instance: interop::vulkan::Instance,
    ) -> Option<interop::vulkan::PhysicalDevice> {
        unsafe {
            let mut device = mem::uninitialized();
            self.0.GetOutputDevice.unwrap()(
                &mut device,
                sys::ETextureType_TextureType_Vulkan,
                instance as _,
            );
            if device == 0 {
                None
            } else {
                Some(device as _)
            }
        }
    }

    /// Returns the hidden area mesh for the current HMD.
    ///
    /// The pixels covered by this mesh will never be seen by the user after the lens distortion is applied based on
    /// visibility to the panels. If this HMD does not have a hidden area mesh, None is returned.  This mesh is meant to
    /// be rendered into the stencil buffer (or into the depth buffer setting nearz) before rendering each eye's view.
    /// This will improve performance by letting the GPU early-reject pixels the user will never see before running the
    /// pixel shader.
    ///
    /// NOTE: Render this mesh with backface culling disabled since the winding order of the vertices can
    /// be different per-HMD or per-eye.
    ///
    /// Passing `HiddenAreaMeshType::Inverse` will produce the visible area mesh that is commonly used in place of
    /// full-screen quads. The visible area mesh covers all of the pixels the hidden area mesh does not cover.
    // TODO: Handle line loops with a separate method and return type, since HiddenAreaMesh assumes triangles.
    pub fn hidden_area_mesh(&self, eye: Eye, ty: HiddenAreaMeshType) -> Option<HiddenAreaMesh> {
        let mesh = unsafe {
            self.0.GetHiddenAreaMesh.unwrap()(eye as sys::EVREye, ty as sys::EHiddenAreaMeshType)
        };
        if mesh.pVertexData == ptr::null_mut() {
            None
        } else {
            Some(HiddenAreaMesh {
                mesh,
                _phantom: PhantomData,
            })
        }
    }

    /// Looks up the current input state of a controller.
    ///
    /// Returns None if the device is not a controller, or if the user is currently in the system menu.
    ///
    /// Needed for rendering controller components (e.g. trigger) accurately wrt. user input using the `render_models`
    /// API.
    pub fn controller_state(&self, device: TrackedDeviceIndex) -> Option<ControllerState> {
        unsafe {
            let mut state = mem::uninitialized();
            if self.0.GetControllerState.unwrap()(
                device,
                &mut state as *mut _ as *mut _,
                mem::size_of_val(&state) as u32,
            ) {
                Some(state)
            } else {
                None
            }
        }
    }

    /// See `controller_state`
    pub fn controller_state_with_pose(
        &self,
        origin: TrackingUniverseOrigin,
        device: TrackedDeviceIndex,
    ) -> Option<(ControllerState, TrackedDevicePose)> {
        unsafe {
            let mut state = mem::uninitialized();
            let mut pose = mem::uninitialized();
            if self.0.GetControllerStateWithPose.unwrap()(
                origin as sys::ETrackingUniverseOrigin,
                device,
                &mut state as *mut _ as *mut _,
                mem::size_of_val(&state) as u32,
                &mut pose,
            ) {
                Some((state, pose.into()))
            } else {
                None
            }
        }
    }

    /// Trigger a single haptic pulse on a controller.
    ///
    /// After this call the application may not trigger another haptic pulse on this controller and axis combination for
    /// 5ms.
    ///
    /// Vive controller haptics respond to axis 0. OpenVR seems to reject durations longer than 3999us.
    pub fn trigger_haptic_pulse(&self, device: TrackedDeviceIndex, axis: u32, microseconds: u16) {
        unsafe { self.0.TriggerHapticPulse.unwrap()(device, axis, microseconds) }
    }

    /// Call this to acknowledge to the system that `Event::Quit` has been received and that the process is exiting.
    ///
    /// This extends the timeout until the process is killed.
    pub fn acknowledge_quit_exiting(&self) {
        unsafe {
            self.0.AcknowledgeQuit_Exiting.unwrap()();
        }
    }

    /// Call this to tell the system that the user is being prompted to save data.
    ///
    /// This halts the timeout and dismisses the dashboard (if it was up). Applications should be sure to actually
    /// prompt the user to save and then exit afterward, otherwise the user will be left in a confusing state.
    pub fn acknowledge_quit_user_prompt(&self) {
        unsafe {
            self.0.AcknowledgeQuit_UserPrompt.unwrap()();
        }
    }

    /// Sets the zero pose for the seated tracker coordinate system to the current position and yaw of the HMD.
    ///
    /// After `reset_seated_zero_pose` all `device_to_absolute_tracking_pose` calls that pass
    /// `TrackingUniverseOrigin::Seated` as the origin will be relative to this new zero pose. The new zero coordinate
    /// system will not change the fact that the Y axis is up in the real world, so the next pose returned from
    /// `device_to_absolute_tracking_pose` after a call to `reset_seated_zero_pose` may not be exactly an identity
    /// matrix.
    ///
    /// NOTE: This function overrides the user's previously saved seated zero pose and should only be called as the
    /// result of a user action.  Users are also able to set their seated zero pose via the OpenVR Dashboard.
    pub fn reset_seated_zero_pose(&self) {
        unsafe {
            self.0.ResetSeatedZeroPose.unwrap()();
        }
    }
}

/// Values represent the tangents of the half-angles from the center view axis
#[derive(Debug, Copy, Clone)]
pub struct RawProjection {
    /// tangent of the half-angle from center axis to the left clipping plane
    pub left: f32,
    /// tangent of the half-angle from center axis to the right clipping plane
    pub right: f32,
    /// tangent of the half-angle from center axis to the top clipping plane
    pub top: f32,
    /// tangent of the half-angle from center axis to the bottom clipping plane
    pub bottom: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct DistortionCoordinates {
    pub red: [f32; 2],
    pub green: [f32; 2],
    pub blue: [f32; 2],
}

pub enum HiddenAreaMeshType {
    /// The mesh that covers pixels which cannot be seen by the wearer of the HMD for optical reasons.
    Standard = sys::EHiddenAreaMeshType_k_eHiddenAreaMesh_Standard as isize,
    /// The inverse of `Standard`, useful for doing full-screen render passes such as postprocessing.
    Inverse = sys::EHiddenAreaMeshType_k_eHiddenAreaMesh_Inverse as isize,
}

impl Default for HiddenAreaMeshType {
    fn default() -> Self {
        HiddenAreaMeshType::Standard
    }
}

/// A triangle mesh containing geometry determined by `HiddenAreaMeshType`.
///
/// Render this mesh with backface culling disabled since the winding order of the vertices can be different per-HMD or
/// per-eye.
pub struct HiddenAreaMesh<'a> {
    mesh: sys::HiddenAreaMesh_t,
    _phantom: PhantomData<&'a [[f32; 2]]>,
}

impl<'a> ::std::ops::Deref for HiddenAreaMesh<'a> {
    type Target = [[f32; 2]];
    fn deref(&self) -> &Self::Target {
        unsafe {
            slice::from_raw_parts(
                &(*self.mesh.pVertexData).v,
                self.mesh.unTriangleCount as usize * 3,
            )
        }
    }
}