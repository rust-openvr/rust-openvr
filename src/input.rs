pub type ActiveActionSet = openvr_sys::VRActiveActionSet_t;
pub type ActionHandle = openvr_sys::VRActionHandle_t;
pub type ActionSetHandle = openvr_sys::VRActionSetHandle_t;
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
            openvr_sys::ETrackedControllerRole_TrackedControllerRole_Invalid => {
                TrackedControllerRole::Invalid
            }
            openvr_sys::ETrackedControllerRole_TrackedControllerRole_LeftHand => {
                TrackedControllerRole::LeftHand
            }
            openvr_sys::ETrackedControllerRole_TrackedControllerRole_RightHand => {
                TrackedControllerRole::RightHand
            }
            openvr_sys::ETrackedControllerRole_TrackedControllerRole_OptOut => {
                TrackedControllerRole::OptOut
            }
            openvr_sys::ETrackedControllerRole_TrackedControllerRole_Treadmill => {
                TrackedControllerRole::Treadmill
            }
            openvr_sys::ETrackedControllerRole_TrackedControllerRole_Stylus => {
                TrackedControllerRole::Stylus
            }
            _ => unreachable!(),
        }
    }
}
