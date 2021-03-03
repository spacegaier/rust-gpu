// Doesn't work, only worked before because I think it got optimized away before
// hitting the backend.

// build-pass
#![no_std]
#![feature(register_attr, ptr_internals)]
#![register_attr(spirv)]

use spirv_std as _;

use core::ptr::Unique;
const POINTER: Unique<[u8;4]> = Unique::<[u8; 4]>::dangling();

#[spirv(fragment)]
pub fn main() {
    let _pointer = POINTER;
}
