rust-openvr
=====

[![Build Status](https://travis-ci.org/Auruss/rust-openvr.svg?branch=master)](https://travis-ci.org/Auruss/rust-openvr)

rust-openvr is a binding for openvr. It's still in progress. Tests are automatically ran by travis.
Also my private jenkins is running these test on Ubuntu 14.04 as well, every successful build will be pushed to its branch (stable, nightly, beta). For good practice always use either stable, beta or nightly instead of master!

## [Link to the documentation](http://auruss.github.io/rust-openvr/openvr/index.html)
## Current sdk version: OpenVR SDK 0.9.19

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
