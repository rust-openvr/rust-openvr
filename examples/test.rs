extern crate vr;

fn main() {
    let ivr = vr::IVRSystem::init().unwrap();
    println!("bounds: {:?}", ivr.bounds());
    println!("recommended size: {:?}", ivr.recommended_render_target_size());
    println!("eye output: {:?} {:?}", ivr.eye_viewport(vr::Eye::Left), ivr.eye_viewport(vr::Eye::Right));
    println!("projection matrix left {:?}", ivr.projection_matrix(vr::Eye::Left, 0.1, 100.));
    println!("projection matrix right {:?}", ivr.projection_matrix(vr::Eye::Right, 0.1, 100.));

    for u in 0..4 {
        for v in 0..4 {
            let pos = ivr.compute_distortion(
                vr::Eye::Left,
                u as f32 / 4.,
                v as f32 / 4.,
            );
            print!("({:2.4}, {:2.4}) ", pos.red[0], pos.red[1]);
        }
        println!("");
    }

    println!("eye_to_head: {:?}", ivr.eye_to_head_transform(vr::Eye::Left));
    println!("vsync: {:?}", ivr.time_since_last_vsync());
    println!("poses {:?}", ivr.tracked_devices(0.).as_slice());
    println!("Done! \\o/");
}