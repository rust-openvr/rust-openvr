#![crate_id = "oculus-info#0.1"]

extern crate ovr = "oculus-vr";

use ovr::{SensorCapabilities, Ovr};

fn main() {
    let ovr = match Ovr::init() {
        Some(ovr) => ovr,
        None => {
             println!("Could not initialize Oculus SDK");
            return;           
        }
    };

    let hmd = match ovr.first_hmd() {
        Some(hmd) => hmd,
        None => {
            println!("Could not get hmd");
            return;
        }
    };

    let started = hmd.start_sensor(SensorCapabilities::new().set_orientation(true),
                                   SensorCapabilities::new().set_orientation(true));

    if !started {
        println!("Could not start sensor");
        return;
    }


    match hmd.get_sensor_description() {
        Some(sd) => {
            println!("Vendor id: {:x}", sd.vendor_id);
            println!("Product id: {:x}", sd.product_id);
            println!("Serial number: {:s}", sd.serial_number);
        }
        None => println!("Failed to get sensor description"),
    }

    let hmd_desc = hmd.get_description();

    println!("Hmd Type: {:?}", hmd_desc.hmd_type);
    println!("Product Name: {:s}", hmd_desc.product_name);
    println!("Manufacture: {:s}", hmd_desc.manufacture);
    println!("Hmd Capabilities: {:?}", hmd_desc.hmd_capabilities);
    println!("Sensor Capabilities: {:?}", hmd_desc.sensor_capabilities);
    println!("Distorion Capabilities: {:?}", hmd_desc.distortion_capabilities);
    println!("Resolution: {:?}", hmd_desc.resolution);
    println!("Window Position: {:?}", hmd_desc.window_position);
    println!("right: {:?}", hmd_desc.eye_fovs.right);
    println!("left {:?}", hmd_desc.eye_fovs.left);
    println!("Eyes render order: {:?}", hmd_desc.eye_render_order);
    println!("Display device name: {:s}", hmd_desc.display_device_name);
    println!("Display idr: {}", hmd_desc.display_id);


    /*let dm = match ovr::DeviceManager::new() {
        Some(dm) => dm,
        None => {s
            println!("Could not initialize Oculus Device Manager");
            return;
        }
    };

    let dev = match dm.enumerate() {
        Some(d) => d,
        None => {
            println!("Could not enumerate Oculus HMD.");
            println!("Was it unplugged?");
            return;
        }
    };

    let sf = match ovr::SensorFusion::new() {
        Some(sf) => sf,
        None => {
            println!("Could not allocate Sensor Fusion")
            return;
        }
    };

    let sensor = match dev.get_sensor() {
        Some(sensor) => sensor,
        None => {
            println!("Could not get the sensor from HMD");
            return;
        }
    };

    let info = dev.get_info();
    sf.attach_to_sensor(&sensor);

    match info.resolution() {
        (w, h) => println!("Resolution: {}x{}", w, h)
    };
    match info.size() {
        (w, h) => println!("Size: {}x{}", w, h)
    };

    println!("Vertical Center: {}", info.vertical_center());
    println!("Eye to screen distance: {}", info.eye_to_screen_distance());
    println!("Lens separation distance: {}", info.lens_separation_distance());
    println!("Interpupillary distance: {}", info.interpupillary_distance());
    println!("distortion K: {:?}", info.distortion_K());
    println!("Chroma Ab Correction: {:?}", info.chroma_ab_correction());
    println!("display name: {:s}", info.name());
    println!("display id: {:?}", info.id());
    println!("display x,y {:?}", info.desktop());*/
}