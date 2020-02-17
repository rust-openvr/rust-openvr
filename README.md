rust-openvr
=====

[![Build Status](https://travis-ci.org/rust-openvr/rust-openvr.svg?branch=master)](https://travis-ci.org/rust-openvr/rust-openvr)
[![Join the chat at https://gitter.im/rust-openvr/rust-openvr](https://badges.gitter.im/rust-openvr/rust-openvr.svg)](https://gitter.im/rust-openvr/rust-openvr?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

High-level bindings for OpenVR.

[API documentation](https://docs.rs/openvr/*/openvr/)

[C/C++ API documentation](https://github.com/ValveSoftware/openvr/wiki/API-Documentation) for reference purposes.

# Requirements

`openvr-sys` requires CMake and C++ to compile and statically link the OpenVR library.

**Imporant**: OpenVR does not support MinGW on Windows, i.e., you have to use the MSVC Rust toolchain and C++ compiler.

# Initializing

```rust
extern crate openvr;

fn main() {
    // Initialize OpenVR.
    let context = unsafe { openvr::init(openvr::ApplicationType::Scene) }.unwrap();

    // Access subsystem.
    let system = context.system().unwrap();

    // See examples/test.rs for a more detailed example.
}
```
