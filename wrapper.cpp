#include "OVR.h"

extern "C"
{
    void OVR_system_init(void)
    {
        OVR::System::Init(OVR::Log::ConfigureDefaultLog(OVR::LogMask_All));
    }

    OVR::DeviceManager* OVR_DeviceManager_Create(void)
    {
        return OVR::DeviceManager::Create();
    }

    OVR::HMDDevice* OVR_DeviceManager_EnumerateDevices(OVR::DeviceManager* pManager)
    {
        return pManager->EnumerateDevices<OVR::HMDDevice>().CreateDevice();
    }

    OVR::HMDInfo* OVR_HDMDevice_GetDeviceInfo(OVR::HMDDevice* pHMD)
    {
        OVR::HMDInfo *hdm = new OVR::HMDInfo;
        pHMD->GetDeviceInfo(hdm);
        return hdm;
    }

    OVR::SensorDevice* OVR_HDMDevice_GetSensor(OVR::HMDDevice* pHMD)
    {
        return pHMD->GetSensor();
    }

    OVR::SensorFusion* OVR_SensorFusion(OVR::HMDDevice* pHMD)
    {
        OVR::SensorFusion* SFusion = new OVR::SensorFusion;
        return SFusion;
    }

    bool OVR_SensorFusion_AttachToSensor(OVR::SensorFusion* SFusion, OVR::SensorDevice *pSensor)
    {
        return (*SFusion).AttachToSensor(pSensor);
    }


    unsigned OVR_HMDInfo_GetScreenHResolution(OVR::HMDInfo* info)
    {
        return info->HResolution;
    }

    unsigned OVR_HMDInfo_GetScreenVResolution(OVR::HMDInfo* info)
    {
        return info->VResolution;
    }

    float OVR_HMDInfo_GetHScreenSize(OVR::HMDInfo* info)
    {
        return info->HScreenSize;
    }

    float OVR_HMDInfo_GetVScreenSize(OVR::HMDInfo* info)
    {
        return info->VScreenSize;
    }

    float OVR_HMDInfo_GetVScreenCenter(OVR::HMDInfo* info)
    {
        return info->VScreenCenter;
    }

    float OVR_HMDInfo_GetEyeToScreenDistance(OVR::HMDInfo* info)
    {
        return info->EyeToScreenDistance;
    }

    float OVR_HMDInfo_GetLensSeparationDistance(OVR::HMDInfo* info)
    {
        return info->LensSeparationDistance;
    }

    float OVR_HMDInfo_GetInterpupillaryDistance(OVR::HMDInfo* info)
    {
        return info->InterpupillaryDistance;
    }

    float OVR_HMDInfo_GetDistortionK(OVR::HMDInfo* info, int idx)
    {
        return info->DistortionK[idx];
    }

    float OVR_HMDInfo_GetChromaAbCorrection(OVR::HMDInfo* info, int idx)
    {
        return info->ChromaAbCorrection[idx];
    }

    unsigned OVR_HMDInfo_GetDesktopX(OVR::HMDInfo* info)
    {
        return info->HResolution;
    }

    unsigned OVR_HMDInfo_GetDesktopY(OVR::HMDInfo* info)
    {
        return info->DesktopY;
    }

    char* OVR_HMDInfo_GetDisplayDeviceName(OVR::HMDInfo* info)
    {
        return info->DisplayDeviceName;
    }

    long OVR_HMDInfo_GetDisplayId(OVR::HMDInfo* info)
    {
        return info->DisplayId;
    }
}