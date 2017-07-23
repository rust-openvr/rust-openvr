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
    let context = match unsafe { openvr::init(openvr::ApplicationType::Other) } {
        Ok(ivr) => ivr,
        Err(err) => {
            println!("Failed to initialize openvr {:?}", err);
            return;
        }
    };

    println!("OpenVR was initialized successfully..");

    let system = match context.system() {
        Ok(sys) => sys,
        Err(err) => {
            println!("Failed to get system interface {:?}", err);
            return;
        }
    };

    println!("\tRecommended size: {:?}", system.recommended_render_target_size());
    println!("\tVSync: {:?}", system.time_since_last_vsync());

    print!("\tProjection matrix left  ");
    print_matrix_4x4(31, system.projection_matrix(openvr::Eye::Left, 0.1, 100.));
    print!("\tProjection matrix right ");
    print_matrix_4x4(31, system.projection_matrix(openvr::Eye::Right, 0.1, 100.));

    print!("\tEye_to_head ");
    print_matrix_4x3(8+12, system.eye_to_head_transform(openvr::Eye::Left));

    print!("\tPoses ");
    let poses = system.device_to_absolute_tracking_pose(openvr::TrackingUniverseOrigin::RawAndUncalibrated, 0.0);
    for pose in poses.iter() {
        print_matrix_4x3(8+6, *pose.device_to_absolute_tracking());
        break;
    }

    println!("\tDistortion example");
    for u in 0..2 {
        for v in 0..2 {
            let pos = system.compute_distortion(
                openvr::Eye::Left,
                u as f32 / 4.,
                v as f32 / 4.,
            ).unwrap();
            print!("\t\t({:2.4}, {:2.4}) ", pos.red[0], pos.red[1]);
        }
        println!("");
    }

    /*
    let ext = match context.extended_display() {
        Ok(ext) => ext,
        Err(err) => {
            println!("Failed to create IVRExtendedDisplay subsystem {:?}", err);
            return;
        }
    };
    println!("\nIVRExtendedDisplay was created");

    println!("\tBounds: {:?}", ext.window_bounds());
    println!("\tEye output: {:?} {:?}", ext.eye_viewport(openvr::Eye::Left), ext.eye_viewport(openvr::Eye::Right));
    */

    let comp = match context.compositor() {
        Ok(ext) => ext,
        Err(err) => {
            println!("Failed to create IVRCompositor subsystem {:?}", err);
            return;
        }
    };

    println!("\nIVRCompositor was created");
    println!("\tIs fullscreen    = {}", comp.is_fullscreen());
    println!("\tInstance Extensions:");
    for ext in comp.vulkan_instance_extensions_required() {
        println!("\t\t{:?}", ext);
    }

    /*
    let model = match context.render_models() {
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
    */

    println!("Done! \\o/");
}
