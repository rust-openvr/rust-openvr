rust-openvr
=====

[![Build Status](https://travis-ci.org/rust-openvr/rust-openvr.svg?branch=master)](https://travis-ci.org/rust-openvr/rust-openvr)
[![Join the chat at https://gitter.im/rust-openvr/rust-openvr](https://badges.gitter.im/rust-openvr/rust-openvr.svg)](https://gitter.im/rust-openvr/rust-openvr?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

rust-openvr is a binding for openvr.

## [Link to the documentation](http://rust-openvr.github.io/rust-openvr/openvr/index.html)
## Current sdk version: OpenVR SDK 1.0.1


Using rust-openvr
-----------

# Requirements

When trying to start a program that uses rust-openvr you will probably get an error message because it can't find openvr.dll (or openvr.so)
You can download the latest version here (https://github.com/ValveSoftware/openvr/tree/master/bin). After downloading please add it into your project folder (also for production releases!).

# Building on Windows
Rust provides 2 pre-compiled version for windows. MSVC ABI and GNU ABI. OpenVR doesn't have an offical build yet for the GNU Bindung and therefore MSVC is required! For more informations about the ABI in Rust see https://www.rust-lang.org/en-US/downloads.html#win-foot

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
