use std::convert::From;

use openvr_sys as sys;

use Chaperone;

/// Chaperone warning states
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ChaperoneCalibrationWarningState {
    Warning,
    BaseStationMayHaveMoved,
    BaseStationRemoved,
    SeatedBoundsInvalid,
    Unknown(u32),
}

impl From<sys::ChaperoneCalibrationState> for ChaperoneCalibrationWarningState {
    fn from(state: sys::ChaperoneCalibrationState) -> Self {
        use self::ChaperoneCalibrationWarningState::*;
        match state {
            sys::ChaperoneCalibrationState_Warning => Warning,
            sys::ChaperoneCalibrationState_Warning_BaseStationMayHaveMoved => {
                BaseStationMayHaveMoved
            }
            sys::ChaperoneCalibrationState_Warning_BaseStationRemoved => BaseStationRemoved,
            sys::ChaperoneCalibrationState_Warning_SeatedBoundsInvalid => SeatedBoundsInvalid,
            _ => Unknown(state as u32),
        }
    }
}

/// Chaperone error states
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ChaperoneCalibrationErrorState {
    Error,
    BaseStationUninitialized,
    BaseStationConflict,
    PlayAreaInvalid,
    CollisionBoundsInvalid,
    Unknown(u32),
}

impl From<sys::ChaperoneCalibrationState> for ChaperoneCalibrationErrorState {
    fn from(state: sys::ChaperoneCalibrationState) -> Self {
        use self::ChaperoneCalibrationErrorState::*;
        match state {
            sys::ChaperoneCalibrationState_Error => Error,
            sys::ChaperoneCalibrationState_Error_BaseStationUninitialized => {
                BaseStationUninitialized
            }
            sys::ChaperoneCalibrationState_Error_BaseStationConflict => BaseStationConflict,
            sys::ChaperoneCalibrationState_Error_PlayAreaInvalid => CollisionBoundsInvalid,
            _ => Unknown(state as u32),
        }
    }
}

/// State of Chaperone calibration.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ChaperoneCalibrationState {
    Ok,
    Warning(ChaperoneCalibrationWarningState),
    Error(ChaperoneCalibrationErrorState),
    Unknown(u32),
}

impl From<sys::ChaperoneCalibrationState> for ChaperoneCalibrationState {
    fn from(state: sys::ChaperoneCalibrationState) -> Self {
        use self::ChaperoneCalibrationState::*;
        match state {
            1 => Ok,
            100...199 => Warning(ChaperoneCalibrationWarningState::from(state)),
            200...299 => Error(ChaperoneCalibrationErrorState::from(state)),
            _ => Unknown(state as u32),
        }
    }
}

impl Chaperone {
    /// Get the current state of Chaperone calibration.
    /// This state can change at any time during a session due to physical base station changes.
    /// (NOTE: Some of these error codes are never returned as implementation for the error states
    /// is still a work in progress.)
    pub fn get_calibration_state(&self) -> ChaperoneCalibrationState {
        unsafe { self.0.GetCalibrationState.unwrap()() }.into()
    }

    /// Returns the width and depth of the Play Area.
    pub fn get_play_area_size(&self) -> Option<(f32, f32)> {
        let mut x: f32 = 0.0;
        let mut z: f32 = 0.0;
        let is_ok = unsafe { self.0.GetPlayAreaSize.unwrap()(&mut x, &mut z) };
        if is_ok {
            Some((x, z))
        } else {
            None
        }
    }

    /// Returns the 4 corner positions of the PlayArea.
    pub fn get_play_area_rect(&self) -> Option<[[f32; 3]; 4]> {
        let mut r = sys::HmdQuad_t {
            vCorners: [sys::HmdVector3_t { v: [0.0; 3] }; 4],
        };
        let is_ok = unsafe { self.0.GetPlayAreaRect.unwrap()(&mut r) };
        if is_ok {
            Some([
                r.vCorners[0].v,
                r.vCorners[1].v,
                r.vCorners[2].v,
                r.vCorners[3].v,
            ])
        } else {
            None
        }
    }

    /// Are chaperone bounds visible?
    pub fn are_bounds_visible(&self) -> bool {
        unsafe { self.0.AreBoundsVisible.unwrap()() }
    }

    /// Set chaperone bounds to always be visible. If set to false, chaperone
    /// bounds will only show when near the edge.
    ///
    /// Caution: this change is persistent, even after your program exits.
    pub fn force_bounds_visible(&self, force: bool) {
        unsafe { self.0.ForceBoundsVisible.unwrap()(force) };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn convert_chaperone_state() {
        assert_eq!(
            ChaperoneCalibrationState::from(sys::ChaperoneCalibrationState_OK),
            ChaperoneCalibrationState::Ok
        );
        assert_eq!(
            ChaperoneCalibrationState::from(sys::ChaperoneCalibrationState_Warning),
            ChaperoneCalibrationState::Warning(ChaperoneCalibrationWarningState::Warning)
        );
        assert_eq!(
            ChaperoneCalibrationState::from(199),
            ChaperoneCalibrationState::Warning(ChaperoneCalibrationWarningState::Unknown(199))
        );
        assert_eq!(
            ChaperoneCalibrationState::from(
                sys::ChaperoneCalibrationState_Warning_BaseStationRemoved
            ),
            ChaperoneCalibrationState::Warning(
                ChaperoneCalibrationWarningState::BaseStationRemoved
            )
        );
        assert_eq!(
            ChaperoneCalibrationState::from(sys::ChaperoneCalibrationState_Error),
            ChaperoneCalibrationState::Error(ChaperoneCalibrationErrorState::Error)
        );
        assert_eq!(
            ChaperoneCalibrationState::from(
                sys::ChaperoneCalibrationState_Error_BaseStationUninitialized
            ),
            ChaperoneCalibrationState::Error(
                ChaperoneCalibrationErrorState::BaseStationUninitialized
            )
        );
        assert_eq!(
            ChaperoneCalibrationState::from(299),
            ChaperoneCalibrationState::Error(ChaperoneCalibrationErrorState::Unknown(299))
        );
        assert_eq!(
            ChaperoneCalibrationState::from(2),
            ChaperoneCalibrationState::Unknown(2)
        );
    }
}
