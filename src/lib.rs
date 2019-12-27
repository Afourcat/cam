#![crate_name = "cam"]
#![deny(missing_docs)]

//! A library for 3D camera and navigation.

extern crate cgmath;

pub use camera::{
    Camera,
    CameraPerspective,
    model_view_projection,
};

mod camera;
