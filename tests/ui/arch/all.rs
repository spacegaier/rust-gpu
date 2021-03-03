// Test `OpAll`.

// build-pass
#![no_std]
#![feature(register_attr)]
#![register_attr(spirv)]

use spirv_std::{arch, glam};

#[spirv(fragment)]
pub fn main() {
    let vector = glam::BVec2::new(true, true);
    assert!(arch::all(vector));
}
