

extern mod ovr = "ovr-rs";

fn main()
{
    unsafe {
       ovr::OVR_system_init();

       let dm = ovr::OVR_DeviceManager_Create();
       println!("dm {:?}", dm);

       let em = ovr::OVR_DeviceManager_EnumerateDevices(dm);
       println!("dm {:?}", em);

    }
}