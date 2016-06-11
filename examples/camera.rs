extern crate openvr;

pub fn main () {
    // init vr system
    let system = match openvr::init() {
        Ok(ivr) => ivr,
        Err(err) => {
            println!("Failed to create IVRSystem subsystem {:?}", err);
            return;
        }
    };

    let camera = match openvr::subsystems::tracked_camera() {
        Ok(ivr) => ivr,
        Err(err) => {
            println!("Failed to create IVRTrackedCamera subsystem {:?}", err);
            return;
        }
    };

    for device in system.tracked_devices(0.0).connected_iter() {
        println!("Device found: {}", device.index);
        println!("\t{:?}", device.device_class());
        println!("\t{:?}", camera.has_camera(&device));
    }
}
