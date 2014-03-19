#[crate_id = "steamworks-info#0.1"];

extern crate vr = "steamworks-vr";

fn main()
{
    let device = vr::Hmd::new();

    let device = match device {
        Ok(device) => device,
        Err(err) => {
            println!("Could not create device error: {:?}", err);
            return;
        }
    };

    println!("{:?}", device.window_bounds());
    println!("{:?}", device.recommended_render_target_size());
    println!("{:?}", device.get_eye_output_viewport(vr::EyeLeft));
    println!("{:?}", device.get_eye_output_viewport(vr::EyeRight));
    println!("{:?}", device.will_drift_in_yaw());
    println!("{:?}", device.get_driver_id());
}