

extern crate ovr = "ovr-rs";

fn main()
{
    ovr::init();

    let dm = ovr::DeviceManager::new().unwrap();
    let dev = dm.enumerate().unwrap();
    let info = dev.get_info().unwrap();

    let sf = ovr::SensorFusion::new().unwrap();
    let sensor = dev.get_sensor().unwrap();

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
}
