use crate::sys;

/// Row-major 3x4 matrix
#[derive(Debug, PartialEq)]
#[repr(C)]
pub struct Matrix3x4(pub [[f32; 4]; 3]);
impl From<&Matrix3x4> for &sys::HmdMatrix34_t {
    fn from(other: &Matrix3x4) -> Self {
        let other = other as *const Matrix3x4;
        // safety: C++ POD types have same memory layout as order of fields
        //   so it is safe to cast it this way
        unsafe { &*other.cast() }
    }
}
impl From<&sys::HmdMatrix34_t> for &Matrix3x4 {
    fn from(other: &sys::HmdMatrix34_t) -> Self {
        let other = other as *const sys::HmdMatrix34_t;
        // safety: C++ POD types have same memory layout as order of fields
        //   so it is safe to cast it this way
        unsafe { &*other.cast() }
    }
}
impl From<&mut Matrix3x4> for &mut sys::HmdMatrix34_t {
    fn from(other: &mut Matrix3x4) -> Self {
        let other = other as *mut Matrix3x4;
        // safety: C++ POD types have same memory layout as order of fields
        //   so it is safe to cast it this way
        unsafe { &mut *other.cast() }
    }
}
impl From<&mut sys::HmdMatrix34_t> for &mut Matrix3x4 {
    fn from(other: &mut sys::HmdMatrix34_t) -> Self {
        let other = other as *mut sys::HmdMatrix34_t;
        // safety: C++ POD types have same memory layout as order of fields
        //   so it is safe to cast it this way
        unsafe { &mut *other.cast() }
    }
}
impl From<sys::HmdMatrix34_t> for Matrix3x4 {
    fn from(other: sys::HmdMatrix34_t) -> Self {
        // Get shrekt, autocxx 😎
        unsafe { std::mem::transmute(other) }
    }
}
impl From<Matrix3x4> for sys::HmdMatrix34_t {
    fn from(other: Matrix3x4) -> Self {
        unsafe { std::mem::transmute(other) }
    }
}

pub use sys::ETrackingUniverseOrigin as TrackingUniverseOrigin;

#[cfg(test)]
mod tests {
    #[allow(unused)]
    use super::*;

    const ROW: usize = 3;
    const COL: usize = 4;

    #[allow(unused)]
    #[rustfmt::skip]
    const DATA_ROW: [[f32; COL]; ROW] = [
        [0.,  1.,  2.,  3. ],
        [4.,  5.,  6.,  7. ],
        [8.,  9.,  10., 11.],
    ];

    #[allow(unused)]
    #[rustfmt::skip]
    const DATA_COL: [[f32; ROW]; COL] = [
        [0.,  4.,  8. ],
        [1.,  5.,  9. ],
        [2.,  6.,  10.],
        [3.,  7.,  11.],
    ];

    
}