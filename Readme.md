VR-RS
=====

[![Build Status](https://travis-ci.org/csherratt/vr-rs.svg?branch=master)](https://travis-ci.org/csherratt/vr-rs)

VR-RS providers bindings for the Oculus's libovr. Currently it only provides bindings for 0.2.5c.

Building
--------

Make sure all submodules were cloned first.

    git submodule update --init --recursive

Building is straight forward.

    ./configure
    make


Using VR-RS
-----------

# Initializing

```rust

extern crate ovr = "oculus-vr";

fn main() {
    // initialize the library
    ovr::init();

    // create a device manager context
    let dm = match ovr::DeviceManager::new() {
        Some(dm) => dm,
        None => {
            println!("Could not initialize Oculus Device Manager");
            return;
        }
    };

    // create hmd instance
    let hmd = match dm.enumerate() {
        Some(d) => d,
        None => {
            println!("Could not enumerate Oculus HMD.");
            println!("Was it unplugged?");
            return;
        }
    };

    // create a sensor fusion, this is used to gather sensor readings
    let sensor_fusion = match ovr::SensorFusion::new() {
        Some(sf) => sf,
        None => {
            println!("Could not allocate Sensor Fusion")
            return;
        }
    };

    // get a sensor device from the hmd.
    let sensor = match hmd.get_sensor() {
        Some(sensor) => sensor,
        None => {
            println!("Could not get the sensor from HMD");
            return;
        }
    };

    // when new sensor readings are ready they will automatically
    // update the sensor fusions state
    sensor_fusion.attach_to_sensor(&sensor);

    let hmd_info = hmd.get_info();
}
```

# Getting head tracking results

```rust
// return a Quaternion of the predicted head's position
let orientation = sensor_fusion.get_predicted_orientation(None);
```

The orientation can be applied to your view matrix create a new view matrix that accurately tracks
the position of the head.

# Rendering the HMD barrel distortion.

vr-rs includes the shader and some utilities that can help create the barrel distortion.

First you must render an undistorted scene into a texture. vr-rs supplies a utility
function `create_reference_matrices` that can be used to create the view / projection matrices.

```rust
let ((projection_left, projection_right),
     (view_left, view_right)) = ovr::create_reference_matrices(&hmd_info, &view_matrix_base, scale);
```

`view_matrix_base` is the view matrix created by applying the head tracking with any other orientation
or positions required to move the camera.

`scale` is the scaled size of the texture compared to the hmd native resolution. `1.` is a good starting
point.

The best resource to drawing the barrel distortion is going to be the
[Oculus VR pdf](http://static.oculusvr.com/sdk-downloads/documents/Oculus_SDK_Overview.pdf#subsection.5.5).