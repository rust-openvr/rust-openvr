#[allow(non_camel_case_types)];

#[crate_id = "steamworks-vr#0.1"];
#[crate_type = "lib"];
#[feature(link_args)];


extern crate cgmath;
use std::libc::{c_float};

#[link(name="steam_api")]
#[link(name="steamvr_wrapper")]
extern {}

pub mod ll {
    use std::libc::{c_float, c_char};

    pub enum IHmd {}

    #[repr(C)]
    pub enum HmdEye {
        Eye_Left = 0,
        Eye_Right = 1
    }

    #[repr(C)]
    pub enum HmdTrackingResult {
        TrackingResult_Uninitialized            = 1,
        TrackingResult_Calibrating_InProgress   = 100,
        TrackingResult_Calibrating_OutOfRange   = 101,
        TrackingResult_Running_OK               = 200,
        TrackingResult_Running_OutOfRange       = 201,
    }

    #[repr(C)]
    pub enum HmdError {
        HmdError_None = 0,

        HmdError_Init_InstallationNotFound  = 100,
        HmdError_Init_InstallationCorrupt   = 101,
        HmdError_Init_VRClientDLLNotFound   = 102,
        HmdError_Init_FileNotFound          = 103,
        HmdError_Init_FactoryNotFound       = 104,
        HmdError_Init_InterfaceNotFound     = 105,
        HmdError_Init_InvalidInterface      = 106,
        HmdError_Init_UserConfigDirectoryInvalid = 107,
        HmdError_Init_HmdNotFound           = 108,
        HmdError_Init_NotInitialized        = 109,

        HmdError_Driver_Failed = 200,

        HmdError_IPC_ServerInitFailed       = 300,
        HmdError_IPC_ConnectFailed          = 301,
        HmdError_IPC_SharedStateInitFailed  = 302,
    }

    #[repr(C)]
    pub enum GraphicsAPIConvention {
        API_DirectX = 0,
        API_OpenGL = 1
    }

    pub struct HmdMatrix44_t {
        m: [[c_float, ..4], ..4]
    }

    impl HmdMatrix44_t {
        pub fn zero() -> HmdMatrix44_t
        {
            HmdMatrix44_t {
                m:  [[0., 0., 0., 0.],
                     [0., 0., 0., 0.],
                     [0., 0., 0., 0.],
                     [0., 0., 0., 0.]]
            }
        }
    }

    pub struct HmdMatrix34_t {
        m: [[c_float, ..4], ..3]
    }

    impl HmdMatrix34_t {
        pub fn zero() -> HmdMatrix34_t
        {
            HmdMatrix34_t {
                m:  [[0., 0., 0., 0.],
                     [0., 0., 0., 0.],
                     [0., 0., 0., 0.]]
            }
        }
    }

    pub struct DistortionCoordinates_t {
        rfRed: [c_float, ..2],
        rfGreen: [c_float, ..2],
        rfBlue: [c_float, ..2]
    }

    extern "C" {
        pub fn VR_Init(peError: *mut HmdError) -> *IHmd;
        pub fn VR_Shutdown();
        pub fn VR_IHmd_Version() -> *c_char;

        pub fn VR_IHmd_GetWindowBounds(ihmd: *IHmd, pnX: *mut i32, pnY: *mut i32, pnWidth: *mut u32, pnHeight: *mut u32);
        pub fn VR_IHmd_GetRecommendedRenderTargetSize(ihmd: *IHmd, pnWidth: *mut u32, pnHeight: *mut u32);
        pub fn VR_IHmd_GetEyeOutputViewport(ihmd: *IHmd, eye: HmdEye, pnX: *mut u32, pnY: *mut u32, pnWidth: *mut u32, pnHeight: *mut u32);
        pub fn VR_IHmd_GetProjectionMatrix(ihmd: *IHmd, eye: HmdEye, fNearZ: c_float, fFarZ: c_float, eProjType: GraphicsAPIConvention) -> HmdMatrix44_t;
        pub fn VR_IHmd_GetProjectionRaw(ihmd: *IHmd, eye: HmdEye, pfLeft: *mut c_float, pfRight: *mut c_float, pfTop: *mut c_float, pfBottom: *mut c_float);
        pub fn VR_IHmd_ComputeDistortion(ihmd: *IHmd, eye: HmdEye, fU: c_float, fV: c_float) -> DistortionCoordinates_t;
        pub fn VR_IHmd_GetEyeMatrix(ihmd: *IHmd, eye: HmdEye) -> HmdMatrix44_t;
        pub fn VR_IHmd_GetViewMatrix(ihmd: *IHmd, fSecondsFromNow: c_float, pMatLeftView: *mut HmdMatrix44_t, pMatRightView: *mut HmdMatrix44_t, peResult: *mut HmdTrackingResult) -> bool;
        pub fn VR_IHmd_GetWorldFromHeadPose(ihmd: *IHmd, fPredictedSecondsFromNow: c_float, pmPose: *mut HmdMatrix34_t, peResult: *mut HmdTrackingResult) -> bool;
        pub fn VR_IHmd_GetLastWorldFromHeadPose(ihmd: *IHmd, pmPose: *HmdMatrix34_t) -> bool;
        pub fn VR_IHmd_WillDriftInYaw(ihmd: *IHmd) -> bool;
        pub fn VR_IHmd_ZeroTracker(ihmd: *IHmd);
        pub fn VR_IHmd_GetDriverId(ihmd: *IHmd, pchBuffer: *mut c_char, unBufferLen: u32) -> u32;
        pub fn VR_IHmd_GetDisplayId(ihmd: *IHmd, pchBuffer: *mut c_char, unBufferLen: u32) -> u32;
    }
}

pub struct Hmd
{
    priv ptr: *ll::IHmd
}

pub enum HmdErrorInit
{
    InstallationNotFound,
    InstallationCorrupt,
    VRClientDLLNotFound,
    FileNotFound,
    FactoryNotFound,
    InterfaceNotFound,
    InvalidInterface,
    UserConfigDirectoryInvalid,
    HmdNotFound,
    NotInitialized,    
}

pub enum IPCError
{
    ServerInitFailed,
    ConnectFailed,
    SharedStateInitFailed,    
}

pub enum HmdError {
    Init(HmdErrorInit),
    Driver_Failed,
    IPC(IPCError),
    Unknown
}

impl HmdError
{
    pub fn new_from_code(err: ll::HmdError) -> HmdError
    {
        match err {
            ll::HmdError_Init_InstallationNotFound => Init(InstallationNotFound),
            ll::HmdError_Init_InstallationCorrupt  => Init(InstallationCorrupt),
            ll::HmdError_Init_VRClientDLLNotFound  => Init(VRClientDLLNotFound),
            ll::HmdError_Init_FileNotFound         => Init(FileNotFound),
            ll::HmdError_Init_FactoryNotFound      => Init(FactoryNotFound),
            ll::HmdError_Init_InterfaceNotFound    => Init(InterfaceNotFound),
            ll::HmdError_Init_InvalidInterface     => Init(InvalidInterface),
            ll::HmdError_Init_UserConfigDirectoryInvalid  => Init(UserConfigDirectoryInvalid),
            ll::HmdError_Init_HmdNotFound          => Init(HmdNotFound),
            ll::HmdError_Init_NotInitialized       => Init(NotInitialized),

            ll::HmdError_Driver_Failed             => Driver_Failed,

            ll::HmdError_IPC_ServerInitFailed       => IPC(ServerInitFailed),
            ll::HmdError_IPC_ConnectFailed          => IPC(ConnectFailed),
            ll::HmdError_IPC_SharedStateInitFailed  => IPC(SharedStateInitFailed),

            _ => Unknown
        }
    }
}

pub enum HmdEye
{
    EyeLeft,
    EyeRight
}

impl HmdEye {
    fn to_ll(&self) -> ll::HmdEye
    {
        match *self {
            EyeLeft => ll::Eye_Left,
            EyeRight => ll::Eye_Right
        }
    }
}

pub enum GraphicsAPIConvention
{
    DirectX,
    OpenGL
}

impl GraphicsAPIConvention
{
    fn to_ll(&self) -> ll::GraphicsAPIConvention
    {
        match *self {
            DirectX => ll::API_DirectX,
            OpenGL => ll::API_OpenGL
        }
    }
}

pub struct DistortionCoordinates
{
    red: [f32, ..2],
    green: [f32, ..2],
    blue: [f32, ..2]    
}

pub enum HmdTrackingResultCalibrating
{
    Calibrating_InProgress,
    Calibrating_OutOfRange
}

pub enum HmdTrackingResultRunning
{
    Running_OK,
    Running_OutOfRange
}

pub enum HmdTrackingResult
{
    Uninitialized,
    Calibration(HmdTrackingResultCalibrating),
    Running(HmdTrackingResultRunning)
}

impl HmdTrackingResult
{
    fn from_ll(tr: ll::HmdTrackingResult) -> HmdTrackingResult
    {
        match tr {
            ll::TrackingResult_Uninitialized            => Uninitialized,
            ll::TrackingResult_Calibrating_InProgress   => Calibration(Calibrating_InProgress),
            ll::TrackingResult_Calibrating_OutOfRange   => Calibration(Calibrating_OutOfRange),
            ll::TrackingResult_Running_OK               => Running(Running_OK),
            ll::TrackingResult_Running_OutOfRange       => Running(Running_OutOfRange)
        }
    }
}

fn to_mat4(mat: &ll::HmdMatrix44_t) -> cgmath::matrix::Mat4<f32>
{
    cgmath::matrix::Mat4::new(
        mat.m[0][0], mat.m[0][1], mat.m[0][2], mat.m[0][3],
        mat.m[1][0], mat.m[1][1], mat.m[1][2], mat.m[1][3],
        mat.m[2][0], mat.m[2][1], mat.m[2][2], mat.m[2][3],
        mat.m[3][0], mat.m[3][1], mat.m[3][2], mat.m[3][3],
    )
}

fn to_mat4_from34(mat: &ll::HmdMatrix34_t) -> cgmath::matrix::Mat4<f32>
{
    cgmath::matrix::Mat4::new(
        mat.m[0][0], mat.m[0][1], mat.m[0][2], mat.m[0][3],
        mat.m[1][0], mat.m[1][1], mat.m[1][2], mat.m[1][3],
        mat.m[2][0], mat.m[2][1], mat.m[2][2], mat.m[2][3],
        0.,          0.,          0.,          1.,
    )
}

fn to_mat34_mat4(mat: &cgmath::matrix::Mat4<f32>) -> ll::HmdMatrix34_t
{
    ll::HmdMatrix34_t {
        m: [[mat.x.x, mat.x.y, mat.x.z, mat.x.w],
            [mat.y.x, mat.y.y, mat.x.z, mat.y.w],
            [mat.z.x, mat.z.y, mat.x.z, mat.z.w]]
    }
}

impl Hmd
{
    pub fn new() -> Result<Hmd, HmdError>
    {
        let mut error = ll::HmdError_None;

        let ptr = unsafe {
            ll::VR_Init(&mut error as *mut ll::HmdError)
        };

        if ptr.is_null() {
            Err(HmdError::new_from_code(error))
        } else {
            Ok(Hmd{ptr: ptr})
        }
    }

    pub fn window_bounds(&self) -> ((int, int), (uint, uint))
    {
        let mut x = 0;
        let mut y = 0;
        let mut width = 0;
        let mut height = 0;

        unsafe {
            ll::VR_IHmd_GetWindowBounds(
                self.ptr,
                &mut x as *mut i32,
                &mut y as *mut i32,
                &mut width as *mut u32,
                &mut height as *mut u32
            );
        }

        ((x as int, y as int), (width as uint, height as uint))
    }

    pub fn recommended_render_target_size(&self) -> (uint, uint)
    {
        let mut width = 0;
        let mut height = 0;

        unsafe {
            ll::VR_IHmd_GetRecommendedRenderTargetSize(
                self.ptr,
                &mut width as *mut u32,
                &mut height as *mut u32
            );
        }

        (width as uint, height as uint)       
    }

    pub fn get_eye_output_viewport(&self, eye: HmdEye) -> ((uint, uint), (uint, uint))
    {
        let mut x = 0;
        let mut y = 0;
        let mut width = 0;
        let mut height = 0;

        unsafe {
            ll::VR_IHmd_GetEyeOutputViewport(
                self.ptr,
                eye.to_ll(),
                &mut x as *mut u32,
                &mut y as *mut u32,
                &mut width as *mut u32,
                &mut height as *mut u32
            );
        }

        ((x as uint, y as uint), (width as uint, height as uint))       
    }

    pub fn get_projection_matrix(&self, eye: HmdEye, near: f32, far: f32, proj: GraphicsAPIConvention) -> cgmath::matrix::Mat4<f32>
    {
        let mat = unsafe {
            ll::VR_IHmd_GetProjectionMatrix(
                self.ptr,
                eye.to_ll(),
                near,
                far,
                proj.to_ll()
            )
        };

        to_mat4(&mat)
    }

    pub fn get_projection_raw(&self, eye: HmdEye) -> (f32, f32, f32, f32)
    {
        let mut left = 0.;
        let mut right = 0.;
        let mut top = 0.;
        let mut bottom = 0.;

        unsafe {
            ll::VR_IHmd_GetProjectionRaw(
                self.ptr,
                eye.to_ll(),
                &mut left as *mut c_float,
                &mut right as *mut c_float,
                &mut top as *mut c_float,
                &mut bottom as *mut c_float,
            )
        }

        (left as f32, right as f32, top as f32, bottom as f32)
    }

    pub fn compute_distortion(&self, eye: HmdEye, fu: f32, fv: f32) -> DistortionCoordinates
    {
        let dc = unsafe {
            ll::VR_IHmd_ComputeDistortion(
                self.ptr,
                eye.to_ll(),
                fu as c_float,
                fv as c_float
            )
        };

        DistortionCoordinates {
            red: [dc.rfRed[0], dc.rfRed[1]],
            blue: [dc.rfBlue[0], dc.rfBlue[1]],
            green: [dc.rfGreen[0], dc.rfGreen[1]],
        }
    }

    pub fn get_eye_matrix(&self, eye: HmdEye) -> cgmath::matrix::Mat4<f32>
    {
        let mat = unsafe {
            ll::VR_IHmd_GetEyeMatrix(
                self.ptr,
                eye.to_ll(),
            )
        };

        to_mat4(&mat)
    }

    pub fn get_view_matrix(&self, from_now: f32) -> (cgmath::matrix::Mat4<f32>, cgmath::matrix::Mat4<f32>, HmdTrackingResult)
    {
        let mut left = ll::HmdMatrix44_t::zero();
        let mut right = ll::HmdMatrix44_t::zero();
        let mut tr = ll::TrackingResult_Uninitialized;

        unsafe {
            ll::VR_IHmd_GetViewMatrix(
                self.ptr,
                from_now as c_float,
                &mut left as *mut ll::HmdMatrix44_t,
                &mut right as *mut ll::HmdMatrix44_t,
                &mut tr as *mut ll::HmdTrackingResult
            );
        }

        (to_mat4(&left), to_mat4(&right), HmdTrackingResult::from_ll(tr))
    }

    pub fn get_world_from_head_pose(&self, from_now: f32) -> Option<(cgmath::matrix::Mat4<f32>, HmdTrackingResult)>
    {
        let mut mat = ll::HmdMatrix34_t::zero();
        let mut tr = ll::TrackingResult_Uninitialized;

        let is_valid = unsafe {
            ll::VR_IHmd_GetWorldFromHeadPose(
                self.ptr,
                from_now as c_float,
                &mut mat as *mut ll::HmdMatrix34_t,
                &mut tr as *mut ll::HmdTrackingResult
            )
        };

        if is_valid {
            Some((to_mat4_from34(&mat), HmdTrackingResult::from_ll(tr)))
        } else {
            None
        }
    }

    pub fn get_last_world_from_head_pose(&self, mat: &cgmath::matrix::Mat4<f32>) -> bool
    {
        let mat = to_mat34_mat4(mat);

        unsafe {
            ll::VR_IHmd_GetLastWorldFromHeadPose(
                self.ptr,
                &mat as *ll::HmdMatrix34_t
            )
        }
    }

    pub fn will_drift_in_yaw(&self) -> bool
    {
        unsafe {
            ll::VR_IHmd_WillDriftInYaw(self.ptr)
        }
    }

    pub fn zero_tracking(&mut self)
    {
        unsafe {
            ll::VR_IHmd_ZeroTracker(self.ptr)
        }
    }

    pub fn get_driver_id(&self) -> ~str
    {
        let mut buf = std::vec::from_elem(128, 0i8);

        unsafe {
            ll::VR_IHmd_GetDriverId(self.ptr,
                buf.as_mut_ptr(),
                buf.len() as u32
            );
            std::str::raw::from_c_str(buf.as_ptr())
        }
    }

    pub fn get_display_id(&self) -> ~str
    {
        let mut buf = std::vec::from_elem(128, 0i8);

        unsafe {
            ll::VR_IHmd_GetDisplayId(self.ptr,
                buf.as_mut_ptr(),
                buf.len() as u32
            );
            std::str::raw::from_c_str(buf.as_ptr())
        }
    }
}