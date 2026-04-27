
pub use crate::errors::VROverlayError;
use crate::pose::Matrix3x4;
use crate::Overlay;
use crate::TextureBounds;
use crate::TrackingUniverseOrigin;
use crate::{sys, ColorTint, TrackedDeviceIndex};



impl Overlay {

    pub fn create_overlay(
        &mut self,
        key: &str,
        friendly_name: &str,
    ) -> Result<OverlayHandle, VROverlayError> {
        let mut handle = sys::VROverlayHandle_t::default();
        let err = unsafe {
            self.0.CreateOverlay.unwrap()(key.as_ptr().cast_mut().cast(),friendly_name.as_ptr().cast_mut().cast(),&mut handle)
        };

        VROverlayError::new(err)?;
        Ok(OverlayHandle(handle))
    }

    pub fn set_visibility(
        &mut self,
        overlay: OverlayHandle,
        is_visible: bool,
    ) -> Result<(), VROverlayError> {
        let err = if is_visible {
            unsafe { self.0.ShowOverlay.unwrap()(overlay.0) }
        } else {
            unsafe { self.0.HideOverlay.unwrap()(overlay.0) }
        };
        VROverlayError::new(err)
    }

    pub fn is_visible(&mut self, overlay: OverlayHandle) -> bool {
        unsafe { self.0.IsOverlayVisible.unwrap()(overlay.0) }
    }

    /// Set the curvature of the overlay, with 0 being a quad and 1 being a cylinder.
    /// # Panics
    /// Panics if `curvature` is not in `[0,1]`
    pub fn set_curvature(
        &mut self,
        overlay: OverlayHandle,
        curvature: f32,
    ) -> Result<(), VROverlayError> {
        // if !(0.0..=1.0).contains(&curvature) {
        //     panic!("`curvature` must lie in range [0,1]")
        // }
        let err = unsafe {
            self.0
                .SetOverlayCurvature.unwrap()(overlay.0, curvature)
        };
        VROverlayError::new(err)
    }

    pub fn curvature(&mut self, overlay: OverlayHandle) -> Result<f32, VROverlayError> {
        let mut curvature = 0.0;
        let err = unsafe {
            self.0
                .GetOverlayCurvature.unwrap()(overlay.0, &mut curvature)
        };
        VROverlayError::new(err)?;
        Ok(curvature)
    }

    /// Sets the opacity of the overlay. `alpha` ranges from 0.0 (transparent) to 1.0 (opaque).
    /// # Panics
    /// Panics if `alpha` is not in `[0,1]`
    pub fn set_opacity(
        &mut self,
        overlay: OverlayHandle,
        alpha: f32,
    ) -> Result<(), VROverlayError> {
        if !(0.0..=1.0).contains(&alpha) {
            panic!("`alpha` must be in range [0,1]");
        }
        let err = unsafe { self.0.SetOverlayAlpha.unwrap()(overlay.0, alpha) };
        VROverlayError::new(err)
    }

    pub fn opacity(&mut self, overlay: OverlayHandle) -> Result<f32, VROverlayError> {
        let mut alpha = 0.0;
        let err = unsafe { self.0.GetOverlayAlpha.unwrap()(overlay.0, &mut alpha) };
        VROverlayError::new(err)?;
        Ok(alpha)
    }

    pub fn width(&mut self, overlay: OverlayHandle) -> Result<f32, VROverlayError> {
        let mut width = 0.0;
        let err = unsafe {
            self.0
                .GetOverlayWidthInMeters.unwrap()(overlay.0, &mut width)
        };
        VROverlayError::new(err)?;
        Ok(width)
    }

    pub fn set_width(
        &mut self,
        overlay: OverlayHandle,
        width_in_meters: f32,
    ) -> Result<(), VROverlayError> {
        let err = unsafe {
            self.0
                .SetOverlayWidthInMeters.unwrap()(overlay.0, width_in_meters)
        };
        VROverlayError::new(err)
    }

    pub fn sort_order(&mut self, overlay: OverlayHandle) -> Result<u32, VROverlayError> {
        let mut sort_order: u32 = 0;
        let err = unsafe {
            self.0
                .GetOverlaySortOrder.unwrap()(overlay.0, &mut sort_order)
        };
        VROverlayError::new(err)?;
        Ok(sort_order)
    }

    pub fn set_sort_order(
        &mut self,
        overlay: OverlayHandle,
        sort_order: u32,
    ) -> Result<(), VROverlayError> {
        let err = unsafe {
            self.0
                .SetOverlaySortOrder.unwrap()(overlay.0, sort_order)
        };
        VROverlayError::new(err)
    }

    pub fn tint(&mut self, overlay: OverlayHandle) -> Result<ColorTint, VROverlayError> {
        let mut tint = ColorTint::default();
        unsafe {
            let err = self.0.GetOverlayColor.unwrap()(
                overlay.0,
                &mut tint.r,
                &mut tint.g,
                &mut tint.b,
            );
            VROverlayError::new(err)?;
            let err = self.0.GetOverlayAlpha.unwrap()(overlay.0, &mut tint.a);
            VROverlayError::new(err)?
        };
        Ok(tint)
    }

    pub fn set_tint(
        &mut self,
        overlay: OverlayHandle,
        tint: ColorTint,
    ) -> Result<(), VROverlayError> {
        unsafe {
            let err = self.0
                .SetOverlayColor.unwrap()(overlay.0, tint.r, tint.g, tint.b);
            VROverlayError::new(err)?;
            let err = self.0.SetOverlayAlpha.unwrap()(overlay.0, tint.a);
            VROverlayError::new(err)?;
        }
        Ok(())
    }

    pub fn set_image(
        &mut self,
        overlay: OverlayHandle,
        img_path: &std::ffi::CStr,
    ) -> Result<(), VROverlayError> {
        let err = unsafe {
            self.0
                .SetOverlayFromFile.unwrap()(overlay.0, img_path.as_ptr().cast_mut())
        };
        VROverlayError::new(err)
    }

    pub fn set_raw_data(
        &mut self,
        overlay: OverlayHandle,
        data: &[u8],
        width: usize,
        height: usize,
        bytes_per_pixel: usize,
    ) -> Result<(), VROverlayError> {
        let err = unsafe {
            let ptr: *const std::ffi::c_void = data.as_ptr().cast();
            // I think there is a typo in the API, and it actually needs a const
            // ptr. *IF* this is true, the following line is safe.
            let ptr = ptr as *mut std::ffi::c_void;

            self.0.SetOverlayRaw.unwrap()(
                overlay.0,
                ptr.cast(),
                width as u32,
                height as u32,
                bytes_per_pixel as u32,
            )
        };
        VROverlayError::new(err)
    }

    /// Get aspect ratio, with aspect expressed as width / height.
    pub fn texel_aspect(&mut self, overlay: OverlayHandle) -> Result<f32, VROverlayError> {
        let mut aspect = 0.0;
        let err = unsafe {
            self.0
                .GetOverlayTexelAspect.unwrap()(overlay.0, &mut aspect)
        };
        VROverlayError::new(err)?;
        Ok(aspect)
    }

    /// Set aspect ratio, with aspect expressed as width / height.
    ///
    /// Note that too extreme of an aspect ratio will cause an error to be returned.
    pub fn set_texel_aspect(
        &mut self,
        overlay: OverlayHandle,
        aspect: f32,
    ) -> Result<(), VROverlayError> {
        let err = unsafe { self.0.SetOverlayTexelAspect.unwrap()(overlay.0, aspect) };
        VROverlayError::new(err)
    }

    /// Sets an absolute transform for this overlay.
    ///
    /// Wraps c++ `SetOverlayTransformAbsolute`.
    pub fn set_transform_absolute(
        &mut self,
        overlay: OverlayHandle,
        origin: TrackingUniverseOrigin,
        origin_to_overlay: &Matrix3x4,
    ) -> Result<(), VROverlayError> {
        let origin_to_overlay: &sys::HmdMatrix34_t = origin_to_overlay.into();
        
        let err = unsafe {
            self.0
                .SetOverlayTransformAbsolute.unwrap()(overlay.0, origin.into(), (&raw const *origin_to_overlay).cast_mut()) 
        };
        VROverlayError::new(err)
    }

    /// Gets the absolute transform for this overlay.
    ///
    /// Wraps c++ `GetOverlayTransformAbsolute`.
    pub fn get_transform_absolute(
        &mut self,
        overlay: OverlayHandle,
        origin_to_overlay: &mut Matrix3x4,
    ) -> Result<TrackingUniverseOrigin, VROverlayError> {
        // Some random value just to initialize the data
        let mut origin = TrackingUniverseOrigin::Standing;
        let origin_to_overlay: &mut sys::HmdMatrix34_t = origin_to_overlay.into();
        let err = unsafe {
            self.0.GetOverlayTransformAbsolute.unwrap()(
                overlay.0,
                (&raw mut origin).cast(),
                origin_to_overlay,
            )
        };
        VROverlayError::new(err).map(|_| origin)
    }

    /// Sets the transform for this overlay, relative to a tracked device.
    ///
    /// Wraps c++ `SetOverlayTransformTrackedDeviceRelative`.
    pub fn set_transform_tracked_device_relative(
        &mut self,
        overlay: OverlayHandle,
        index: TrackedDeviceIndex,
        device_to_overlay: &Matrix3x4,
    ) -> Result<(), VROverlayError> {
        let device_to_overlay: &sys::HmdMatrix34_t = device_to_overlay.into();
        let err = unsafe {
            self.0
                .SetOverlayTransformTrackedDeviceRelative.unwrap()(overlay.0, index.0, (&raw const *device_to_overlay).cast_mut())
        };
        VROverlayError::new(err)
    }

    /// Gets the transform for this overlay, relative to a tracked device.
    ///
    /// Wraps c++ `GetOverlayTransformTrackedDeviceRelative`.
    pub fn get_transform_tracked_device_relative(
        &mut self,
        overlay: OverlayHandle,
        device_to_overlay: &mut Matrix3x4,
    ) -> Result<TrackedDeviceIndex, VROverlayError> {
        let mut index = sys::TrackedDeviceIndex_t::default();
        let device_to_overlay: &mut sys::HmdMatrix34_t = device_to_overlay.into();
        let err = unsafe {
            self.0
                .GetOverlayTransformTrackedDeviceRelative.unwrap()(overlay.0, &mut index, device_to_overlay)
        };
        VROverlayError::new(err)?;
        // TODO: is the error ever really going to be delayed to here? (Can we successfully return an invalid handle?)
        if index==sys::k_unTrackedDeviceIndexInvalid as u32{
            return Err(VROverlayError::RequestFailed);
        }
        Ok(crate::tracking::TrackedDeviceIndex(index))
    }

    pub fn set_texture_bounds(
        &mut self,
        overlay: OverlayHandle,
        bounds: &TextureBounds,
    ) -> Result<(), VROverlayError> {
        let err = unsafe {
            self.0
                .SetOverlayTextureBounds.unwrap()(overlay.0, (&raw const bounds.0).cast_mut())
        };
        VROverlayError::new(err)
    }

    pub fn is_dashboard_visible(&mut self) -> bool {
        unsafe { self.0.IsDashboardVisible.unwrap()() }
    }
}
unsafe impl Send for Overlay {}
unsafe impl Sync for Overlay {}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct OverlayHandle(pub sys::VROverlayHandle_t);
