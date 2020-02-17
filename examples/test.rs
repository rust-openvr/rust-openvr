extern crate openvr;

fn print_matrix<M, N>(offset: u32, mat: M)
where
    M: AsRef<[N]>,
    N: AsRef<[f32]>,
{
    let offset: String = (0..offset).map(|_| ' ').collect();
    let mut is_first_row = true;
    for row in mat.as_ref() {
        if !is_first_row {
            print!("{}", offset);
        }
        let row_interior: Vec<_> = row.as_ref().iter().map(|x| format!("{:7.4}", x)).collect();
        println!("[{}]", row_interior.join(", "));
        is_first_row = false;
    }
}

fn main() {
    let context = match unsafe { openvr::init(openvr::ApplicationType::Other) } {
        Ok(ivr) => ivr,
        Err(err) => {
            println!("Failed to initialize openvr: {}", err);
            return;
        }
    };

    println!("OpenVR was initialized successfully..");

    let system = match context.system() {
        Ok(sys) => sys,
        Err(err) => {
            println!("Failed to get system interface: {}", err);
            return;
        }
    };

    println!(
        "\tRecommended size: {:?}",
        system.recommended_render_target_size()
    );
    println!("\tVSync: {:?}", system.time_since_last_vsync());

    print!("\tProjection matrix left  ");
    print_matrix(32, system.projection_matrix(openvr::Eye::Left, 0.1, 100.));
    print!("\tProjection matrix right ");
    print_matrix(32, system.projection_matrix(openvr::Eye::Right, 0.1, 100.));

    print!("\tEye to head left ");
    print_matrix(25, system.eye_to_head_transform(openvr::Eye::Left));

    print!("\tPoses ");
    let poses = system
        .device_to_absolute_tracking_pose(openvr::TrackingUniverseOrigin::RawAndUncalibrated, 0.0);
    for pose in poses.iter() {
        print_matrix(8 + 6, pose.device_to_absolute_tracking());
        break;
    }

    println!("\tDistortion example");
    for u in 0..2 {
        for v in 0..2 {
            let pos = system
                .compute_distortion(openvr::Eye::Left, u as f32 / 4., v as f32 / 4.)
                .unwrap();
            print!("\t\t({:7.4}, {:7.4}) ", pos.red[0], pos.red[1]);
        }
        println!();
    }

    let comp = match context.compositor() {
        Ok(ext) => ext,
        Err(err) => {
            println!("Failed to create IVRCompositor subsystem: {}", err);
            return;
        }
    };
    println!();

    println!("IVRCompositor was created");
    println!("\tIs fullscreen = {}", comp.is_fullscreen());
    println!("\tVulkan Instance Extensions:");
    for ext in comp.vulkan_instance_extensions_required() {
        println!("\t\t{:?}", ext);
    }
    println!();

    println!("IVRChaperone was created");
    let chaperone = match context.chaperone() {
        Ok(sys) => sys,
        Err(err) => {
            println!("Failed to get chaperone interface: {}", err);
            return;
        }
    };
    println!(
        "\tCalibration state: {:?}",
        chaperone.get_calibration_state()
    );
    println!("\tPlay area size: {:?}", chaperone.get_play_area_size());
    print!("\tPlay area rect: ");
    if let Some(play_area_rect) = chaperone.get_play_area_rect() {
        print_matrix(24, &play_area_rect);
    } else {
        println!("None");
    }
    println!(
        "\tAre bounds visible = {:?}",
        chaperone.are_bounds_visible()
    );
    println!();

    println!("Done! \\o/");
}
