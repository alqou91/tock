//! Drivers and chip support for ESP32-C3.

#![feature(asm, const_fn_trait_bound, naked_functions)]
#![no_std]
#![crate_name = "esp32_c3"]
#![crate_type = "rlib"]

pub mod chip;
pub mod intc;
