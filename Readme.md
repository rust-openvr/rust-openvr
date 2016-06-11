rust-openvr
=====

[![Join the chat at https://gitter.im/rust-openvr/rust-openvr](https://badges.gitter.im/rust-openvr/rust-openvr.svg)](https://gitter.im/rust-openvr/rust-openvr?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

[![Build Status](https://travis-ci.org/rust-openvr/rust-openvr.svg?branch=master)](https://travis-ci.org/rust-openvr/rust-openvr)

rust-openvr is a binding for openvr.

## [Link to the documentation](http://rust-openvr.github.io/rust-openvr/openvr/index.html)
## Current sdk version: OpenVR SDK 0.9.19


Using rust-openvr
-----------

# Requirements

When trying to start a program that uses rust-openvr you will probably get an error message because it can't find openvr.dll (or openvr.so)
You can download the latest version here (https://github.com/ValveSoftware/openvr/tree/master/bin). After downloading please add it into your project folder (also for production releases!).

# Initializing

```rust

extern crate openvr;

fn main() {
    // Initialize system subsystem
    let system = match openvr::init() {
        Ok(sys) => sys,
        Err(err) => {
            println!("Could not initialize OpenVR SDK: \n\t{:?}", err);
            return;           
        }
    };

    // accessing other sub systems
    let ext = openvr::extended_display();

    // ..
}
```

# Examples
For data collection examples/test.rs can be used.
For an actual opengl implementation see examples/opengl.rs (WIP)
