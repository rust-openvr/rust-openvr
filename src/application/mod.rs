use std::ffi::{CStr, CString};
use std::path::Path;

use crate::errors::VRApplicationError;
use crate::Application;

type Result<T> = std::result::Result<T, VRApplicationError>;

impl Application {
    // ---- Handle Management ----

    pub fn add_application_manifest(&mut self, path: &Path, temporary: bool) -> Result<()> {
        let path = if let Ok(s) = CString::new(path.to_string_lossy().as_bytes()) {
            s
        } else {
            return Err(VRApplicationError::InvalidParameter);
        };
        self.add_application_manifest_raw(&path, temporary)
    }

    pub fn add_application_manifest_raw(&mut self, path: &CStr, temporary: bool) -> Result<()> {
        let err = unsafe {
            self.0.AddApplicationManifest.unwrap()(path.as_ptr().cast_mut().cast(), temporary)
        };
        VRApplicationError::new(err)
    }

    pub fn remove_application_manifest(&mut self, path: &Path) -> Result<()> {
        let path = if let Ok(s) = CString::new(path.to_string_lossy().as_bytes()) {
            s
        } else {
            return Err(VRApplicationError::InvalidParameter);
        };
        self.remove_application_manifest_raw(&path)
    }

    pub fn remove_application_manifest_raw(&mut self, path: &CStr) -> Result<()> {
        let err =
            unsafe { self.0.RemoveApplicationManifest.unwrap()(path.as_ptr().cast_mut().cast()) };
        VRApplicationError::new(err)
    }

    pub fn is_application_installed(&mut self, key: &str) -> Result<bool> {
        let name = if let Ok(s) = CString::new(key) {
            s
        } else {
            unreachable!()
            // return VRApplicationError::InvalidParameter.map(|_| unreachable!());
        };

        self.is_application_installed_raw(&name)
    }

    pub fn is_application_installed_raw(&mut self, key: &CStr) -> Result<bool> {
        let installed =
            unsafe { self.0.IsApplicationInstalled.unwrap()(key.as_ptr().cast_mut().cast()) };
        Ok(installed)
    }

    pub fn set_application_auto_launch_raw(&mut self, key: &CStr, auto_launch: bool) -> Result<()> {
        let err = unsafe {
            self.0.SetApplicationAutoLaunch.unwrap()(key.as_ptr().cast_mut().cast(), auto_launch)
        };
        VRApplicationError::new(err)
    }

    pub fn set_application_auto_launch(&mut self, key: &str, auto_launch: bool) -> Result<()> {
        let name = if let Ok(s) = CString::new(key) {
            s
        } else {
            unreachable!()
        };
        self.set_application_auto_launch_raw(&name, auto_launch)
    }

    pub fn get_application_auto_launch_raw(&mut self, key: &CStr) -> Result<bool> {
        let auto_launch =
            unsafe { self.0.GetApplicationAutoLaunch.unwrap()(key.as_ptr().cast_mut().cast()) };
        Ok(auto_launch)
    }

    pub fn get_application_auto_launch(&mut self, key: &str) -> Result<bool> {
        let name = if let Ok(s) = CString::new(key) {
            s
        } else {
            unreachable!()
        };

        self.get_application_auto_launch_raw(&name)
    }
}
