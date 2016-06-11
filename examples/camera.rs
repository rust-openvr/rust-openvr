extern crate openvr;

pub fn main () {
    {
        // init vr system
        let system = match openvr::init() {
            Ok(ivr) => ivr,
            Err(err) => {
                println!("Failed to create IVRSystem subsystem {:?}", err);
                return;
            }
        };

        // init camera subsystem
        let camera = match openvr::subsystems::tracked_camera() {
            Ok(ivr) => ivr,
            Err(err) => {
                println!("Failed to create IVRTrackedCamera subsystem {:?}", err);
                return;
            }
        };

        // look for tracked devices with a camera
        let mut camera_device = None;
        for device in system.tracked_devices(0.0).connected_iter() {
            if camera.has_camera(&device).unwrap_or(false) {
                println!("Tracked Device with camera found, ID: {}", device.index);
                println!("\t{:?}", device.device_class());
                println!("\t{:?}", camera.frame_size(&device, openvr::tracked_camera::CameraFrameType::MaximumUndistorted));
                println!("\t{:?}", camera.intrinisics(&device, openvr::tracked_camera::CameraFrameType::MaximumUndistorted));

                camera_device = Some(device.clone());
            }
        }

        // make sure camera is available
        if camera_device.is_none() {
            println!("No tracked device with camera found. Exiting..");

            openvr::shutdown();
            return;
        }

        // create stream
        let stream = camera.stream(&camera_device.unwrap()).unwrap_or_else(|err| {
            println!("Could not start stream to camera: {}", err.message());
            openvr::shutdown();
            panic!("");
        });

        let frame = stream.read(openvr::tracked_camera::CameraFrameType::MaximumUndistorted).unwrap_or_else(|err| {
            println!("Could not read from camera stream: {}", err.message());
            openvr::shutdown();
            panic!("");
        });

        println!("Frame Data recieved! {:?}", frame);
    }

    openvr::shutdown();
}
