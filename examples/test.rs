extern crate openvr;

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
    let system = match openvr::init() {
        Ok(ivr) => ivr,
        Err(err) => {
            println!("Failed to create IVR subsystem {:?}", err);
            return;
        }
    };

    println!("IVRSystem was created");

    println!("\trecommended size: {:?}", system.recommended_render_target_size());
    println!("\tvsync: {:?}", system.time_since_last_vsync());

    print!("\tprojection matrix left  ");
    print_matrix_4x4(31, system.projection_matrix(openvr::Eye::Left, 0.1, 100.));
    print!("\tprojection matrix right ");
    print_matrix_4x4(31, system.projection_matrix(openvr::Eye::Right, 0.1, 100.));

    print!("\teye_to_head ");
    print_matrix_4x3(8+12, system.eye_to_head_transform(openvr::Eye::Left));

    print!("\tposes ");
    print_matrix_4x3(8+6, system.tracked_devices(0.).as_slice()[0].to_device);

    println!("Distortion example");
    for u in 0..2 {
        for v in 0..2 {
            let pos = system.compute_distortion(
                openvr::Eye::Left,
                u as f32 / 4.,
                v as f32 / 4.,
            );
            print!("\t({:2.4}, {:2.4}) ", pos.red[0], pos.red[1]);
        }
        println!("");
    }

    let ext = match openvr::extended_display() {
        Ok(ext) => ext,
        Err(err) => {
            println!("Failed to create IVRExtendedDisplay subsystem {:?}", err);
            return;
        }
    };
    println!("\nIVRExtendedDisplay was created");
    println!("\tbounds: {:?}", ext.window_bounds());
    println!("\teye output: {:?} {:?}", ext.eye_viewport(openvr::Eye::Left), ext.eye_viewport(openvr::Eye::Right));

    let comp = match openvr::compositor() {
        Ok(ext) => ext,
        Err(err) => {
            println!("Failed to create IVRCompositor subsystem {:?}", err);
            return;
        }
    };

    println!("\nIVRCompositor was created");
    println!("\tis fullscreen    = {}", comp.is_fullscreen());
    println!("\tcan render scene = {}", comp.can_render_scene());

    let model = match openvr::render_models() {
        Ok(ext) => ext,
        Err(err) => {
            println!("Failed to create IVRRenderModels subsystem {:?}", err);
            return;
        }
    };

    println!("\nIVRRenderModels was created\n Count: {}", model.get_count());
    for i in 0..model.get_count() {
        println!("\t{}", model.get_name(i));
    }

    openvr::shutdown();
    println!("Done! \\o/");


}
