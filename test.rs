

extern mod ovr = "ovr-rs";

fn main()
{
    ovr::init();

    let dm = ovr::DeviceManager::new().unwrap();
    let dev = dm.enumerate().unwrap();
    let info = dev.get_info();

    let sf = ovr::SensorFusion::new().unwrap();
    let sensor = dev.get_sensor().unwrap();

    sf.attach_to_sensor(&sensor);

}