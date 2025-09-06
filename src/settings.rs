
use crate::{errors::VRSettingsError, Settings};
use std::{ffi::CStr, mem::MaybeUninit};



impl Settings {

	pub fn remove_section<'ret, 'manager: 'ret>(
		&'manager mut self,
		pch_section: &CStr,
	) -> Result<(), VRSettingsError> {
		unsafe {
			let mut error: MaybeUninit<openvr_sys::EVRSettingsError> = MaybeUninit::uninit();
            self.0.RemoveSection.unwrap()(pch_section.as_ptr().cast_mut().cast(),error.as_mut_ptr().cast());
			let error = error.assume_init();
            VRSettingsError::new(error)
		}
	}

	pub fn remove_key_in_section<'ret, 'manager: 'ret>(
		&'manager mut self,
		pch_section: &CStr,
		pch_settings_key: &CStr,
	) -> Result<(), VRSettingsError> {
		unsafe {
			let mut error: MaybeUninit<openvr_sys::EVRSettingsError> = MaybeUninit::uninit();
			self.0.RemoveKeyInSection.unwrap()(
				pch_section.as_ptr() as *mut _,
				pch_settings_key.as_ptr() as *mut _,
				error.as_mut_ptr().cast(),
			);
			let error = error.assume_init();
            VRSettingsError::new(error)
		}
	}

	pub fn get_int32<'ret, 'manager: 'ret>(
		&'manager mut self,
		pch_section: &CStr,
		pch_settings_key: &CStr,
	) -> Result<i32, VRSettingsError> {
		unsafe {
			let mut error: MaybeUninit<openvr_sys::EVRSettingsError> = MaybeUninit::uninit();
			let result = self.0.GetInt32.unwrap()(
				pch_section.as_ptr() as *mut _,
				pch_settings_key.as_ptr() as *mut _,
				error.as_mut_ptr() as * mut openvr_sys::EVRSettingsError,
			);
			let error = error.assume_init();
            VRSettingsError::new(error)?;
			return Ok(result);
		};
	}

	pub fn set_int32<'ret, 'manager: 'ret>(
		&'manager mut self,
		pch_section: &CStr,
		pch_settings_key: &CStr,
		value: i32,
	) -> Result<(), VRSettingsError> {
		unsafe {
			let mut error: MaybeUninit<openvr_sys::EVRSettingsError> = MaybeUninit::uninit();
			self.0.SetInt32.unwrap()(
				pch_section.as_ptr() as *mut _,
				pch_settings_key.as_ptr() as *mut _,
				value,
				error.as_mut_ptr().cast(),
			);
			let error = error.assume_init();
			VRSettingsError::new(error)
		}
	}

	pub fn get_float<'ret, 'manager: 'ret>(
		&'manager mut self,
		pch_section: &CStr,
		pch_settings_key: &CStr,
	) -> Result<f32, VRSettingsError> {
		unsafe {
			let mut error: MaybeUninit<openvr_sys::EVRSettingsError> = MaybeUninit::uninit();
			let result = self.0.GetFloat.unwrap()(
				pch_section.as_ptr() as *mut _,
				pch_settings_key.as_ptr() as *mut _,
				error.as_mut_ptr().cast(),
			);
			let error = error.assume_init();
			VRSettingsError::new(error)?;
			return Ok(result);
		};
	}

	pub fn set_float<'ret, 'manager: 'ret>(
		&'manager mut self,
		pch_section: &CStr,
		pch_settings_key: &CStr,
		value: f32,
	) -> Result<(), VRSettingsError> {
		unsafe {
			let mut error: MaybeUninit<openvr_sys::EVRSettingsError> = MaybeUninit::uninit();
			self.0.SetFloat.unwrap()(
				pch_section.as_ptr() as *mut _,
				pch_settings_key.as_ptr() as *mut _,
				value,
				error.as_mut_ptr().cast()
			);
			let error = error.assume_init();
			VRSettingsError::new(error)
		}
	}

	pub fn get_bool<'ret, 'manager: 'ret>(
		&'manager mut self,
		pch_section: &CStr,
		pch_settings_key: &CStr,
	) -> Result<bool, VRSettingsError> {
		unsafe {
			let mut error: MaybeUninit<openvr_sys::EVRSettingsError> = MaybeUninit::uninit();
			let result = self.0.GetBool.unwrap()(
				pch_section.as_ptr() as *mut _,
				pch_settings_key.as_ptr() as *mut _,
				error.as_mut_ptr().cast(),
			);
			let error = error.assume_init();
			VRSettingsError::new(error)?;
			return Ok(result);
		};
	}

	pub fn set_bool<'ret, 'manager: 'ret>(
		&'manager mut self,
		pch_section: &CStr,
		pch_settings_key: &CStr,
		value: bool,
	) -> Result<(), VRSettingsError> {
		unsafe {
			let mut error: MaybeUninit<openvr_sys::EVRSettingsError> = MaybeUninit::uninit();
			self.0.SetBool.unwrap()(
				pch_section.as_ptr() as *mut _,
				pch_settings_key.as_ptr() as *mut _,
				value,
				error.as_mut_ptr().cast(),
			);
			let error = error.assume_init();
			VRSettingsError::new(error)
		}
	}
}