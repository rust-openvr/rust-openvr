rust-openvr
=====

[![Build Status](https://travis-ci.org/rust-openvr/rust-openvr.svg?branch=master)](https://travis-ci.org/rust-openvr/rust-openvr)
[![Join the chat at https://gitter.im/rust-openvr/rust-openvr](https://badges.gitter.im/rust-openvr/rust-openvr.svg)](https://gitter.im/rust-openvr/rust-openvr?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

A high-level binding for OpenVR 1.0.10. 

[API documentation](https://docs.rs/openvr/*/openvr/)

High-level documentation can be found at [the OpenVR wiki](https://github.com/ValveSoftware/openvr/wiki/API-Documentation).

Using rust-openvr
-----------

# Requirements

openvr-sys needs cmake and a C++ compiler so that it can compile and statically link the OpenVR client library.

## Windows

Upstream OpenVR does not support MinGW. You must use an MSVC-targeted rust toolchain and C++ compiler.

# Initializing

```rust

extern crate openvr;

fn main() {
    // Initialize OpenVR
    let context = unsafe { openvr::init(openvr::ApplicationType::Scene) }.unwrap();

    // accessing subsystems
    let system = context.system().unwrap();

    // ..
}
```

# Examples
See examples/test.rs for a more detailed example.
