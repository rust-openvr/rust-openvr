rust-openvr
=====

[![Build Status](https://travis-ci.org/Auruss/rust-openvr.svg?branch=master)](https://travis-ci.org/Auruss/rust-openvr)

rust-openvr is a binding for openvr. It's still in progress. Tests are automatically ran by travis.
Also my private jenkins is running these test on Ubuntu 14.04 as well, every successful build will be pushed to its branch (stable, nightly, beta). For good practice always use either stable, beta or nightly instead of master!

Building
--------

rust-openvr is fully cargoized. to Compile

    cargo build

To add as a dependency using cargo Cargo add the following to your `Cargo.toml`. Will be added to crates.io once we've a stable version. For nightly/beta use nightly/beta branch instead.

    [dependencies.openvr]
    git = "https://github.com/auruss/rust-openvr.git"
    branch = "stable"


Using rust-openvr
-----------

# Initializing

```rust

extern crate openvr;

use openvr::{SensorCapabilities, Ovr};

fn main() {
    // Initalize the Oculus VR library
    let ovr = match Ovr::init() {
        Some(ovr) => ovr,
        None => {
             println!("Could not initialize OpenVR SDK");
            return;           
        }
    };

    // get the first available HMD device, returns None
    // if no HMD device is currently plugged in
    let hmd = match ovr.first_hmd() {
        Some(hmd) => hmd,
        None => {
            println!("Could not get hmd");
            return;
        }
    };

    // start the sensor recording, Require orientation tracking
    let started = hmd.start_sensor(SensorCapabilities::new().set_orientation(true),
                                   SensorCapabilities::new().set_orientation(true));
    if !started {
        println!("Could not start sensor");
        return;
    }
}
```

# Render loop

The OpenVR SDK will handle most of the heavy lifting of the barrel distortion.

```rust
fn render(frame_index: uint, hmd: &ovr::Hmd, base_view: &Matrix4<f32>) {
    // start a new frame, the frame_index should increment each frame
    let frame_timing = hmd.begin_frame(frame_index);
    let desc = hmd.get_description();

    for &eye in [ovr::EyeLeft, ovr::EyeRight].iter() {
        // start rendering a new eye, this will give the most current
        // copy of the pose from the HMD tracking sensor
        let pose = self.window.get_hmd().begin_eye_render(eye);

        // base_view * pose * eye_view_adjustment
        let view = base_view.mul_m(&pose.orientation.to_matrix4())
                            .mul_m(&Matrix4::translate(&eye.view_adjust));
        let projection = desc.eye_fovs.eye(eye).default_eye_fov;

        // render to texture
        render();

        let texture = ovr::Texture(width, height,
                                   viewport_offset_x, viewport_offset_y,
                                   viewport_width, viewport_height,
                                   opengl_texture_id);
        hmd.end_eye_render(eye, pose, &texture);
    }

    // this will swap the buffers and frame sync
    hmd.end_frame();
}
```
