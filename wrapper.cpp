#include "OVR.h"

extern "C"
{
    struct Vector3f {
        float x, y, z;
    };

    struct Quatf {
        float x, y, z, w;
    };

    struct Matrix4f {
        float m11, m12, m13, m14,
              m21, m22, m23, m24,
              m31, m32, m33, m34,
              m41, m42, m43, m44;
    };

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

    bool OVR_SensorFusion_IsAttachedToSensor(OVR::SensorFusion *sf)
    {
        return sf->IsAttachedToSensor();
    }

    Quatf OVR_SensorFusion_GetOrientation(OVR::SensorFusion *sf)
    {
        OVR::Quatf quat = sf->GetOrientation();
        Quatf out = {quat.x, quat.y, quat.z, quat.w};
        return out;
    }

    Quatf OVR_SensorFusion_GetPredictedOrientation(OVR::SensorFusion *sf)
    {
        OVR::Quatf quat = sf->GetPredictedOrientation();
        Quatf out = {quat.x, quat.y, quat.z, quat.w};
        return out;
    }

    Quatf OVR_SensorFusion_GetPredictedOrientation_opt(OVR::SensorFusion *sf, float dt)
    {
        OVR::Quatf quat = sf->GetPredictedOrientation(dt);
        Quatf out = {quat.x, quat.y, quat.z, quat.w};
        return out;    
    }

    Vector3f OVR_SensorFusion_GetAcceleration(OVR::SensorFusion *sf)
    {
        OVR::Vector3f vec = sf->GetAcceleration();
        Vector3f out = {vec.x, vec.y, vec.z};
        return out; 
    }

    Vector3f OVR_SensorFusion_GetAngularVelocity(OVR::SensorFusion *sf)
    {
        OVR::Vector3f vec = sf->GetAngularVelocity();
        Vector3f out = {vec.x, vec.y, vec.z};
        return out; 
    }

    Vector3f OVR_SensorFusion_GetMagnetometer(OVR::SensorFusion *sf)
    {
        OVR::Vector3f vec = sf->GetMagnetometer();
        Vector3f out = {vec.x, vec.y, vec.z};
        return out; 
    }

    Vector3f OVR_SensorFusion_GetCalibratedMagnetometer(OVR::SensorFusion *sf)
    {
        OVR::Vector3f vec = sf->GetCalibratedMagnetometer();
        Vector3f out = {vec.x, vec.y, vec.z};
        return out; 
    }

    void OVR_SensorFusion_Reset(OVR::SensorFusion *sf)
    {
        sf->Reset();
    }

    void OVR_SensorFusion_EnableMotionTracking(OVR::SensorFusion *sf, bool enable)
    {
        sf->EnableMotionTracking(enable);
    }

    bool OVR_SensorFusion_IsMotionTrackingEnabled(OVR::SensorFusion *sf)
    {
        return sf->IsMotionTrackingEnabled();
    }

    float OVR_SensorFusion_GetPredictionDelta(OVR::SensorFusion *sf)
    {
        return sf->GetPredictionDelta();
    }

    void OVR_SensorFusion_SetPrediction(OVR::SensorFusion *sf, float dt, bool enable)
    {
        sf->SetPrediction(dt, enable);
    }

    void OVR_SensorFusion_SetPredictionEnabled(OVR::SensorFusion *sf, bool enable)
    {
        sf->SetPredictionEnabled(enable);
    }

    bool OVR_SensorFusion_IsPredictionEnabled(OVR::SensorFusion *sf)
    {
        return sf->IsPredictionEnabled();
    }

    void OVR_SensorFusion_SetGravityEnabled(OVR::SensorFusion *sf, bool enableGravity)
    {
        sf->SetGravityEnabled(enableGravity);
    }

    bool OVR_SensorFusion_IsGravityEnabled(OVR::SensorFusion *sf)
    {
        return sf->IsGravityEnabled();
    }

    float OVR_SensorFusion_GetAccelGain(OVR::SensorFusion *sf)
    {
        return sf->GetAccelGain();
    }

    void OVR_SensorFusion_SetAccelGain(OVR::SensorFusion *sf, float ag)
    {
        return sf->SetAccelGain(ag);
    }

    bool OVR_SensorFusion_SaveMagCalibration(OVR::SensorFusion *sf, const char *calibrationName)
    {
        return sf->SaveMagCalibration(calibrationName);
    }

    bool OVR_SensorFusion_LoadMagCalibration(OVR::SensorFusion *sf, const char *calibrationName)
    {
        return sf->LoadMagCalibration(calibrationName);
    }

    void OVR_SensorFusion_SetYawCorrectionEnabled(OVR::SensorFusion *sf, bool enable)
    {
        sf->SetYawCorrectionEnabled(enable);
    }

    bool OVR_SensorFusion_IsYawCorrectionEnabled(OVR::SensorFusion *sf)
    {
        return sf->IsYawCorrectionEnabled();
    }

    void OVR_SensorFusion_SetMagCalibration(OVR::SensorFusion *sf, Matrix4f *m)
    {
        OVR::Matrix4f mat = OVR::Matrix4f(m->m11, m->m12, m->m13, m->m14,
                                          m->m21, m->m22, m->m23, m->m24,
                                          m->m31, m->m32, m->m33, m->m34,
                                          m->m41, m->m42, m->m43, m->m44);
        sf->SetMagCalibration(mat);
    }

    Matrix4f OVR_SensorFusion_GetMagCalibration(OVR::SensorFusion *sf)
    {
        OVR::Matrix4f m = sf->GetMagCalibration();
        Matrix4f out = {
            m.M[0][0], m.M[0][1], m.M[0][2], m.M[0][3],
            m.M[1][0], m.M[1][1], m.M[1][2], m.M[1][3],
            m.M[2][0], m.M[2][1], m.M[2][2], m.M[2][3],
            m.M[3][0], m.M[3][1], m.M[3][2], m.M[3][3],
        };
        return out;
    }

    time_t OVR_SensorFusion_GetMagCalibrationTime(OVR::SensorFusion *sf)
    {
        return sf->GetMagCalibrationTime();
    }

    bool OVR_SensorFusion_HasMagCalibration(OVR::SensorFusion *sf)
    {
        return sf->HasMagCalibration();
    }

    void OVR_SensorFusion_ClearMagCalibration(OVR::SensorFusion *sf)
    {
        sf->ClearMagCalibration();
    }

    void OVR_SensorFusion_ClearMagReferences(OVR::SensorFusion *sf)
    {
        sf->ClearMagReferences();
    }

    Vector3f OVR_SensorFusion_GetCalibratedMagValue(OVR::SensorFusion *sf, const Vector3f *rawMag)
    {
        OVR::Vector3f input = OVR::Vector3f(rawMag->x, rawMag->y, rawMag->z);
        OVR::Vector3f vec = sf->GetCalibratedMagValue(input);
        Vector3f out = {vec.x, vec.y, vec.z};
        return out; 
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