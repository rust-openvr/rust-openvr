#![crate_id = "oculus-info#0.1"]

extern crate ovr = "oculus-vr";

fn main() {
    ovr::init();

    let dm = match ovr::DeviceManager::new() {
        Some(dm) => dm,
        None => {
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
    println!("display x,y {:?}", info.desktop());
}