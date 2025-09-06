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

#[cfg(feature = "nalgebra")]
mod nalgebra_impls {
    use super::*;

    impl From<&Matrix3x4> for nalgebra::Matrix3x4<f32> {
        fn from(other: &Matrix3x4) -> Self {
            use slice_of_array::SliceFlatExt;
            Self::from_row_slice(other.0.flat())
        }
    }

    impl<Storage>
        From<
            &nalgebra::Matrix<
                f32,
                nalgebra::base::dimension::U3,
                nalgebra::base::dimension::U4,
                Storage,
            >,
        > for Matrix3x4
    where
        Storage: nalgebra::base::storage::Storage<
            f32,
            nalgebra::base::dimension::U3,
            nalgebra::base::dimension::U4,
        >,
    {
        fn from(
            other: &nalgebra::Matrix<
                f32,
                nalgebra::base::dimension::U3,
                nalgebra::base::dimension::U4,
                Storage,
            >,
        ) -> Self {
            Self(other.transpose().data.0)
        }
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

    /// A series of sanity checks to ensure that matrix math, and nalgebra, work as expected
    #[cfg(feature = "nalgebra")]
    #[test]
    fn test_nalgebra_sanity() {
        use slice_of_array::SliceFlatExt;

        // Sanity check that we actually know what row vs column major means, lol
        {
            let nalg1 = nalgebra::Matrix3x4::from_row_slice(DATA_ROW.flat());
            let nalg2 = nalgebra::Matrix3x4::from_column_slice(DATA_COL.flat());
            assert_eq!(nalg1, nalg2);
            assert_eq!(nalg1.data.0, nalg2.data.0);
            assert_eq!(nalg1.data.0, DATA_COL);
            assert_eq!(nalg2.data.0, DATA_COL);
        }

        // Sanity check: nalgebra transpose function actually changes shape of array storage
        {
            let m3_4 = nalgebra::Matrix3x4::from_row_slice(DATA_ROW.flat());
            assert_eq!(m3_4.shape(), (3, 4));
            let m4_3 = m3_4.transpose();
            assert_eq!(m4_3.shape(), (4, 3));
        }

        // Sanity check: you can convert from column major to row major by transposing
        {
            let m3_4 = nalgebra::Matrix3x4::from_row_slice(DATA_ROW.flat());
            let m3_4_inner: [[f32; ROW]; COL] = m3_4.data.0;
            assert_eq!(m3_4_inner, DATA_COL);

            let m4_3 = m3_4.transpose();
            let m4_3_inner: [[f32; COL]; ROW] = m4_3.data.0;
            assert_eq!(m4_3_inner, DATA_ROW);
        }
    }

    #[cfg(feature = "nalgebra")]
    #[test]
    fn test_nalgebra_conversion() {
        use slice_of_array::SliceFlatExt;

        let m_nalg = nalgebra::Matrix3x4::from_row_slice(DATA_ROW.flat());
        let m_ovr = Matrix3x4(DATA_ROW);
        assert_eq!(m_ovr, Matrix3x4::from(&m_nalg));
        assert_eq!(m_nalg, nalgebra::Matrix3x4::from(&m_ovr))
    }
}