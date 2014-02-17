
#include "steamvr.h"

extern "C"
{
    vr::IHmd* VR_Init(vr::HmdError *peError)
    {
        vr::VR_Init(peError);
    }

    void VR_Shutdown(void)
    {
        vr::VR_Shutdown();
    }

    const char* const VR_IHmd_Version(void)
    {
        return vr::IHmd_Version;
    }

    void VR_IHmd_GetWindowBounds(vr::IHmd *ihmd, int32_t *pnX, int32_t *pnY, uint32_t *pnWidth, uint32_t *pnHeight)
    {
        ihmd->GetWindowBounds(pnX, pnY, pnWidth, pnHeight);
    }

    void VR_IHmd_GetRecommendedRenderTargetSize(vr::IHmd *ihmd, uint32_t *pnWidth, uint32_t *pnHeight)
    {
        ihmd->GetRecommendedRenderTargetSize(pnWidth, pnHeight);
    }

    void VR_IHmd_GetEyeOutputViewport(vr::IHmd *ihmd, vr::Hmd_Eye eEye, uint32_t *pnX, uint32_t *pnY, uint32_t *pnWidth, uint32_t *pnHeight)
    {
#ifdef STEAMWORKS 
        ihmd->GetEyeOutputViewport(eEye, vr::API_OpenGL, pnX, pnY, pnWidth, pnHeight);
#else
        ihmd->GetEyeOutputViewport(eEye, pnX, pnY, pnWidth, pnHeight);
#endif
    }

    vr::HmdMatrix44_t VR_IHmd_GetProjectionMatrix(vr::IHmd *ihmd, vr::Hmd_Eye eEye, float fNearZ, float fFarZ, vr::GraphicsAPIConvention eProjType)
    {
        ihmd->GetProjectionMatrix(eEye, fNearZ, fFarZ, eProjType);
    }

    void VR_IHmd_GetProjectionRaw(vr::IHmd *ihmd, vr::Hmd_Eye eEye, float *pfLeft, float *pfRight, float *pfTop, float *pfBottom)
    {
        ihmd->GetProjectionRaw(eEye, pfLeft, pfRight, pfTop, pfBottom);
    }

    vr::DistortionCoordinates_t VR_IHmd_ComputeDistortion(vr::IHmd *ihmd, vr::Hmd_Eye eEye, float fU, float fV)
    {
        ihmd->ComputeDistortion(eEye, fU, fV);
    }

    vr::HmdMatrix44_t VR_IHmd_GetEyeMatrix(vr::IHmd *ihmd, vr::Hmd_Eye eEye)
    {
        return ihmd->GetEyeMatrix(eEye);
    }

    bool VR_IHmd_GetViewMatrix(vr::IHmd *ihmd, float fSecondsFromNow, vr::HmdMatrix44_t *pMatLeftView, vr::HmdMatrix44_t *pMatRightView, vr::HmdTrackingResult *peResult)
    {
        ihmd->GetViewMatrix(fSecondsFromNow, pMatLeftView, pMatRightView, peResult);
    }

    bool VR_IHmd_GetWorldFromHeadPose(vr::IHmd *ihmd, float fPredictedSecondsFromNow, vr::HmdMatrix34_t *pmPose, vr::HmdTrackingResult *peResult)
    {
        ihmd->GetWorldFromHeadPose(fPredictedSecondsFromNow, pmPose, peResult);
    }

    bool VR_IHmd_GetLastWorldFromHeadPose(vr::IHmd *ihmd, vr::HmdMatrix34_t *pmPose)
    {
        ihmd->GetLastWorldFromHeadPose(pmPose);   
    }

    bool VR_IHmd_WillDriftInYaw(vr::IHmd *ihmd)
    {
        ihmd->WillDriftInYaw();
    }

    void VR_IHmd_ZeroTracker(vr::IHmd *ihmd)
    {
        ihmd->ZeroTracker();
    }

    uint32_t VR_IHmd_GetDeviceId(vr::IHmd *ihmd, char *pchBuffer, uint32_t unBufferLen )
    {
        ihmd->GetDriverId(pchBuffer, unBufferLen);
    }

    uint32_t VR_IHmd_GetDisplayId(vr::IHmd *ihmd, char *pchBuffer, uint32_t unBufferLen )
    {
        ihmd->GetDisplayId(pchBuffer, unBufferLen);
    }
}