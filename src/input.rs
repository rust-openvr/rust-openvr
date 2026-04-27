use enumset::{EnumSet, EnumSetType};

use crate::{errors::VRInputError, pose, Input};

use std::{
    ffi::{CStr, CString},
    mem::MaybeUninit,
    path::Path,
    time::Duration,
};
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum TrackedControllerRole {
    Invalid = openvr_sys::ETrackedControllerRole_TrackedControllerRole_Invalid as isize,
    LeftHand = openvr_sys::ETrackedControllerRole_TrackedControllerRole_LeftHand as isize,
    RightHand = openvr_sys::ETrackedControllerRole_TrackedControllerRole_RightHand as isize,
    OptOut = openvr_sys::ETrackedControllerRole_TrackedControllerRole_OptOut as isize,
    Treadmill = openvr_sys::ETrackedControllerRole_TrackedControllerRole_Treadmill as isize,
    Stylus = openvr_sys::ETrackedControllerRole_TrackedControllerRole_Stylus as isize,
}
impl From<openvr_sys::ETrackedControllerRole> for TrackedControllerRole {
    fn from(item: openvr_sys::ETrackedControllerRole) -> Self {
        match item {
            openvr_sys::ETrackedControllerRole_TrackedControllerRole_Invalid => Self::Invalid,
            openvr_sys::ETrackedControllerRole_TrackedControllerRole_LeftHand => Self::LeftHand,
            openvr_sys::ETrackedControllerRole_TrackedControllerRole_RightHand => Self::RightHand,
            openvr_sys::ETrackedControllerRole_TrackedControllerRole_OptOut => Self::OptOut,
            openvr_sys::ETrackedControllerRole_TrackedControllerRole_Treadmill => Self::Treadmill,
            openvr_sys::ETrackedControllerRole_TrackedControllerRole_Stylus => Self::Stylus,
            _ => unreachable!(),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct VRActionHandle(pub openvr_sys::VRActionHandle_t);
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VRActionSetHandle(pub openvr_sys::VRActionHandle_t);
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VRInputValueHandle(pub openvr_sys::VRInputValueHandle_t);
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct VRActiveActionSet(pub openvr_sys::VRActiveActionSet_t);
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct VRDigitalActionData(pub openvr_sys::InputDigitalActionData_t);
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct VRAnalogActionData(pub openvr_sys::InputAnalogActionData_t);
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct VRPoseActionData(pub openvr_sys::InputPoseActionData_t);
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct VROriginInfo(pub openvr_sys::InputOriginInfo_t);
type Result<T> = std::result::Result<T, VRInputError>;
#[derive(EnumSetType, Debug)]
#[enumset(repr = "u32")]
pub enum InputString {
    Hand,
    ControllerType,
    InputSource,
    // TODO: openvr allows you to pass a u32 with all bits set to get a string that has all information, current and future.
    //       is there a good way to represent that with enumset? do we care?
}

impl Input {
    // ---- Handle Management ----

    pub fn set_action_manifest(&mut self, path: &Path) -> Result<()> {
        let path = if let Ok(s) = CString::new(path.to_string_lossy().as_bytes()) {
            s
        } else {
            return Err(VRInputError::InvalidParam);
        };
        self.set_action_manifest_raw(&path)
    }

    pub fn set_action_manifest_raw(&mut self, path: &CStr) -> Result<()> {
        let err = unsafe { self.0.SetActionManifestPath.unwrap()(path.as_ptr().cast_mut().cast()) };
        VRInputError::new(err)
    }

    pub fn get_action_set_handle(&mut self, name: &str) -> Result<VRActionSetHandle> {
        let name = if let Ok(s) = CString::new(name) {
            s
        } else {
            return Err(VRInputError::InvalidParam);
        };

        self.get_action_set_handle_raw(&name)
    }

    pub fn get_action_set_handle_raw(&mut self, name: &CStr) -> Result<VRActionSetHandle> {
        let mut handle: VRActionSetHandle = VRActionSetHandle(0);

        let err = unsafe {
            self.0.GetActionSetHandle.unwrap()(name.as_ptr().cast_mut().cast(), &mut handle.0)
        };

        VRInputError::new(err)?;
        Ok(handle)
    }

    pub fn get_action_handle(&mut self, name: &str) -> Result<VRActionHandle> {
        let name = if let Ok(s) = CString::new(name) {
            s
        } else {
            unreachable!()
            // return VRInputError::new(sys::VRInputError::VRInputError_InvalidParam)
            //     .map(|_| unreachable!());
        };

        self.get_action_handle_raw(&name)
    }

    pub fn get_action_handle_raw(&mut self, name: &CStr) -> Result<VRActionHandle> {
        let mut handle: VRActionHandle = VRActionHandle(0);

        let err = unsafe {
            self.0.GetActionHandle.unwrap()(name.as_ptr().cast_mut().cast(), &mut handle.0)
        };

        VRInputError::new(err)?;
        Ok(handle)
    }

    pub fn get_input_source_handle(&mut self, name: &str) -> Result<VRInputValueHandle> {
        let name = if let Ok(s) = CString::new(name) {
            s
        } else {
            unreachable!()
            // return VRInputError::new(sys::VRInputError::VRInputError_InvalidParam)
            //     .map(|_| unreachable!());
        };

        self.get_input_source_handle_raw(&name)
    }

    pub fn get_input_source_handle_raw(&mut self, name: &CStr) -> Result<VRInputValueHandle> {
        let mut handle: VRInputValueHandle = VRInputValueHandle(0);

        let err = unsafe {
            self.0.GetInputSourceHandle.unwrap()(name.as_ptr().cast_mut().cast(), &mut handle.0)
        };

        VRInputError::new(err)?;
        Ok(handle)
    }

    // ---- Read Action State ----

    pub fn update_actions(&mut self, sets: &mut [VRActiveActionSet]) -> Result<()> {
        let err = unsafe {
            self.0.UpdateActionState.unwrap()(
                sets.as_mut_ptr().cast(),
                size_of::<VRActiveActionSet>() as u32,
                sets.len() as u32,
            )
        };

        VRInputError::new(err)
    }

    pub fn get_digital_action_data(
        &mut self,
        action: VRActionHandle,
        restrict: VRInputValueHandle,
    ) -> Result<VRDigitalActionData> {
        let mut data: MaybeUninit<VRDigitalActionData> = MaybeUninit::uninit();
        let err = unsafe {
            self.0.GetDigitalActionData.unwrap()(
                action.0,
                data.as_mut_ptr().cast(),
                size_of::<VRDigitalActionData>() as u32,
                restrict.0,
            )
        };
        VRInputError::new(err)?;
        Ok(VRDigitalActionData(unsafe { data.assume_init().0 }))
    }

    pub fn get_analog_action_data(
        &mut self,
        action: VRActionHandle,
        restrict: VRInputValueHandle,
    ) -> Result<VRAnalogActionData> {
        let mut data: MaybeUninit<VRAnalogActionData> = MaybeUninit::uninit();
        let err = unsafe {
            self.0.GetAnalogActionData.unwrap()(
                action.0,
                data.as_mut_ptr().cast(),
                size_of::<VRAnalogActionData>() as u32,
                restrict.0,
            )
        };
        VRInputError::new(err)?;
        Ok(VRAnalogActionData(unsafe { data.assume_init().0 }))
    }

    pub fn get_pose_action_data_relative_to_now(
        &mut self,
        action: VRActionHandle,
        universe: pose::TrackingUniverseOrigin,
        seconds_from_now: f32,
        restrict: VRInputValueHandle,
    ) -> Result<VRPoseActionData> {
        let mut data: MaybeUninit<VRPoseActionData> = MaybeUninit::uninit();
        let err = unsafe {
            self.0.GetPoseActionDataRelativeToNow.unwrap()(
                action.0,
                universe,
                seconds_from_now,
                data.as_mut_ptr().cast(),
                size_of::<VRPoseActionData>() as u32,
                restrict.0,
            )
        };

        VRInputError::new(err)?;
        Ok(VRPoseActionData(unsafe { data.assume_init().0 }))
    }

    // ---- Action Origins ----

    pub fn get_action_origins(
        &mut self,
        action_set: VRActionSetHandle,
        digital_action_handle: VRActionHandle,
    ) -> Result<[u64; 16]> {
        let mut origins: [u64; 16] = unsafe { std::mem::zeroed() };
        let err = unsafe {
            self.0.GetActionOrigins.unwrap()(
                action_set.0,
                digital_action_handle.0,
                origins.as_mut_ptr().cast(),
                16,
            )
        };
        VRInputError::new(err)?;
        Ok(origins)
    }

    pub fn get_origin_localized_name(
        &mut self,
        origin: VRInputValueHandle,
        bits: EnumSet<InputString>,
    ) -> Result<String> {
        let mut name: [::std::os::raw::c_char; 128usize] = unsafe { ::std::mem::zeroed() };

        let err = unsafe {
            self.0.GetOriginLocalizedName.unwrap()(
                origin.0,
                name.as_mut_ptr() as *mut i8,
                128,
                bits.as_repr() as i32,
            )
        };

        VRInputError::new(err)?;
        let trimmed_str = name
            .iter()
            .map(|&c| c as u8)
            .take_while(|&x| x != 0)
            .collect();

        Ok(String::from_utf8(trimmed_str).expect("Could not parse string from name array"))
    }

    pub fn get_origin_tracked_device_info(
        &mut self,
        origin: VRInputValueHandle,
    ) -> Result<VROriginInfo> {
        let mut data: MaybeUninit<openvr_sys::InputOriginInfo_t> = MaybeUninit::uninit();
        let err = unsafe {
            self.0.GetOriginTrackedDeviceInfo.unwrap()(
                origin.0,
                data.as_mut_ptr(),
                std::mem::size_of::<openvr_sys::InputOriginInfo_t>() as u32,
            )
        };

        VRInputError::new(err)?;
        Ok(VROriginInfo(unsafe { data.assume_init() }))
    }

    pub fn show_action_origins(
        &mut self,
        set: VRActionSetHandle,
        action: VRActionHandle,
    ) -> Result<()> {
        let err = unsafe { self.0.ShowActionOrigins.unwrap()(set.0, action.0) };

        VRInputError::new(err)
    }

    pub fn show_bindings_for_action_set(
        &mut self,
        sets: &mut [VRActiveActionSet],
        origin: VRInputValueHandle,
    ) -> Result<()> {
        let err = unsafe {
            self.0.ShowBindingsForActionSet.unwrap()(
                sets.as_mut_ptr().cast(),
                std::mem::size_of::<VRActiveActionSet>() as u32,
                sets.len() as u32,
                origin.0,
            )
        };

        VRInputError::new(err)
    }

    pub fn trigger_haptic_vibration_action(
        &mut self,
        action: VRActionHandle,
        start_seconds_from_now: f32,
        duration: Duration,
        frequency: f32,
        amplitude: f32,
        restrict: VRInputValueHandle,
    ) -> Result<()> {
        let err = unsafe {
            self.0.TriggerHapticVibrationAction.unwrap()(
                action.0,
                start_seconds_from_now,
                duration.as_secs_f32(),
                frequency,
                amplitude,
                restrict.0,
            )
        };

        VRInputError::new(err)
    }

    pub fn open_binding_ui(
        &mut self,
        app_key: Option<&str>,
        action_set: Option<VRActionSetHandle>,
        input_device: VRInputValueHandle,
        show_on_desktop: bool,
    ) -> Result<()> {
        let app_key_cstr_ptr = app_key
            .map(|s| CString::new(s).unwrap())
            .map(|cstr| cstr.as_ptr())
            .unwrap_or(std::ptr::null());
        let action_set = match action_set {
            Some(s) => s.0,
            None => 0,
        };
        let err = unsafe {
            self.0.OpenBindingUI.unwrap()(
                app_key_cstr_ptr.cast_mut().cast(),
                action_set,
                input_device.0,
                show_on_desktop,
            )
        };
        VRInputError::new(err)
    }

    pub fn get_action_binding_info(
        &mut self,
        action: VRActionHandle,
    ) -> std::result::Result<Vec<openvr_sys::InputBindingInfo_t>, VRInputError> {
        let mut data: [openvr_sys::InputBindingInfo_t; 16] = unsafe { std::mem::zeroed() };
        let mut count: MaybeUninit<u32> = MaybeUninit::uninit();

        let err = unsafe {
            self.0.GetActionBindingInfo.unwrap()(
                action.0,
                data.as_mut_ptr().cast(),
                size_of::<openvr_sys::InputBindingInfo_t>() as u32,
                16,
                count.as_mut_ptr().cast(),
            )
        };
        VRInputError::new(err)?;

        let mut data_vec = vec![];

        for i in 0..unsafe { count.assume_init() } {
            let info = unsafe { data.get_unchecked(i as usize).clone() };
            data_vec.push(openvr_sys::InputBindingInfo_t {
                rchDevicePathName: info.rchDevicePathName,
                rchInputPathName: info.rchInputPathName,
                rchModeName: info.rchModeName,
                rchSlotName: info.rchSlotName,
                rchInputSourceType: info.rchInputSourceType,
            });
        }

        std::result::Result::Ok(data_vec)
    }
}
