extern crate vr;

fn print_matrix_4x4(offset: u32, mat: [[f32; 4]; 4]) {
    let off: String = (0..offset).map(|_| ' ').collect();
    println!("{:?}",        mat[0]);
    println!("{}{:?}", off, mat[1]);
    println!("{}{:?}", off, mat[2]);
    println!("{}{:?}", off, mat[3]);
}

fn print_matrix_4x3(offset: u32, mat: [[f32; 4]; 3]) {
    let off: String = (0..offset).map(|_| ' ').collect();
    println!("{:?}",        mat[0]);
    println!("{}{:?}", off, mat[1]);
    println!("{}{:?}", off, mat[2]);
}

fn main() {
    let ivr = match vr::IVRSystem::init() {
        Ok(ivr) => ivr,
        Err(err) => {
            println!("Failed to create IVR subsystem {:?}", err);
            return;
        }
    };

    println!("IVR was created");
    println!("\tbounds: {:?}", ivr.bounds());
    println!("\trecommended size: {:?}", ivr.recommended_render_target_size());
    println!("\teye output: {:?} {:?}", ivr.eye_viewport(vr::Eye::Left), ivr.eye_viewport(vr::Eye::Right));
    println!("\tvsync: {:?}", ivr.time_since_last_vsync());

    print!("\tprojection matrix left  ");
    print_matrix_4x4(31, ivr.projection_matrix(vr::Eye::Left, 0.1, 100.));
    print!("\tprojection matrix right ");
    print_matrix_4x4(31, ivr.projection_matrix(vr::Eye::Right, 0.1, 100.));

    print!("\teye_to_head ");
    print_matrix_4x3(8+12, ivr.eye_to_head_transform(vr::Eye::Left));

    print!("\tposes ");
    print_matrix_4x3(8+6, ivr.tracked_devices(0.).as_slice()[0].to_device);


    println!("Distortion example");
    for u in 0..2 {
        for v in 0..2 {
            let pos = ivr.compute_distortion(
                vr::Eye::Left,
                u as f32 / 4.,
                v as f32 / 4.,
            );
            print!("\t({:2.4}, {:2.4}) ", pos.red[0], pos.red[1]);
        }
        println!("");
    }

    println!("Trying to create a compositor");
    match ivr.compositor() {
        Err(err) => println!("Could not create compositor {:?}", err),
        Ok(comp) => {
            println!("\tCreated one!");
            println!("\tis fullscreen    ={}", comp.is_fullscreen());
            println!("\tis vsync         ={}", comp.get_vsync());
            println!("\tcan render scene ={}", comp.can_render_scene());
        }
    }

    println!("Done! \\o/");


}