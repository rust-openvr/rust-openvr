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

    // not pointer on purpose!
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
}