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

openvr-sys needs cmake and a C++ compiler so that it can compile and statically link the OpenVR client library.

# Building on Windows

Rust provides 2 pre-compiled version for windows. MSVC ABI and GNU ABI. The proprietary OpenVR library which is loaded
behind the scenes by the client library is MSVC only, and therefore MSVC is required! For more informations about the
ABI in Rust see https://www.rust-lang.org/en-US/downloads.html#win-foot

# Initializing

```rust

extern crate openvr;

fn main() {
    // Initialize OpenVR
    let context = openvr::init(openvr::ApplicationType::Scene).unwrap();

    // accessing subsystems
    let system = context.system().unwrap();

    // ..
}
```

# Examples
For data collection examples/test.rs can be used.
For an actual opengl implementation see examples/opengl.rs (WIP)
