//! Prelude

use std::ffi::CStr;

/// Stdout

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__write_bool(x: bool) {
    print!("{}", x);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__write_char(x: char) {
    print!("{}", x);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__write_f32(x: f32) {
    print!("{}", x);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__write_f64(x: f64) {
    print!("{}", x);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__write_i8(x: i8) {
    print!("{}", x);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__write_i16(x: i16) {
    print!("{}", x);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__write_i32(x: i32) {
    print!("{}", x);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__write_i64(x: i64) {
    print!("{}", x);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__write_string(string: *const i8) {
    print!("{}", unsafe { CStr::from_ptr(string).to_str().unwrap() });
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__write_u8(x: u8) {
    print!("{}", x);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__write_u16(x: u16) {
    print!("{}", x);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__write_u32(x: u32) {
    print!("{}", x);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__write_u64(x: u64) {
    print!("{}", x);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__write_usize(x: usize) {
    print!("{}", x);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__writeln_bool(x: bool) {
    println!("{}", x);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__writeln_char(x: char) {
    println!("{}", x);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__writeln_f32(x: f32) {
    println!("{}", x);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__writeln_f64(x: f64) {
    println!("{}", x);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__writeln_i8(x: i8) {
    println!("{}", x);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__writeln_i16(x: i16) {
    println!("{}", x);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__writeln_i32(x: i32) {
    println!("{}", x);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__writeln_i64(x: i64) {
    println!("{}", x);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__writeln_string(string: *const i8) {
    println!("{}", unsafe { CStr::from_ptr(string).to_str().unwrap() });
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__writeln_u8(x: u8) {
    println!("{}", x);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__writeln_u16(x: u16) {
    println!("{}", x);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__writeln_u32(x: u32) {
    println!("{}", x);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__writeln_u64(x: u64) {
    println!("{}", x);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__writeln_usize(x: usize) {
    println!("{}", x);
}

/// Arithmetic

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__add_f32(x: f32, y: f32) -> f32 {
    x + y
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__add_f64(x: f64, y: f64) -> f64 {
    x + y
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__add_i8(x: i8, y: i8) -> i8 {
    x + y
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__add_i16(x: i16, y: i16) -> i16 {
    x + y
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__add_i32(x: i32, y: i32) -> i32 {
    x + y
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__add_i64(x: i64, y: i64) -> i64 {
    x + y
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__add_u8(x: u8, y: u8) -> u8 {
    x + y
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__add_u16(x: u16, y: u16) -> u16 {
    x + y
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__add_u32(x: u32, y: u32) -> u32 {
    x + y
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__add_u64(x: u64, y: u64) -> u64 {
    x + y
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__add_usize(x: usize, y: usize) -> usize {
    x + y
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__addeq_f32(x: &mut f32, y: f32) {
    *x = *x + y
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__addeq_f64(x: &mut f64, y: f64) {
    *x = *x + y
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__addeq_i8(x: &mut i8, y: i8) {
    *x = *x + y
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__addeq_i16(x: &mut i16, y: i16) {
    *x = *x + y
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__addeq_i32(x: &mut i32, y: i32) {
    *x = *x + y
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__addeq_i64(x: &mut i64, y: i64) {
    *x = *x + y
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__addeq_u8(x: &mut u8, y: u8) {
    *x = *x + y
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__addeq_u16(x: &mut u16, y: u16) {
    *x = *x + y
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__addeq_u32(x: &mut u32, y: u32) {
    *x = *x + y
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__addeq_u64(x: &mut u64, y: u64) {
    *x = *x + y
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__addeq_usize(x: &mut usize, y: usize) {
    *x = *x + y
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__mul_f32(x: f32, y: f32) -> f32 {
    x * y
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__mul_f64(x: f64, y: f64) -> f64 {
    x * y
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__mul_i8(x: i8, y: i8) -> i8 {
    x * y
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__mul_i16(x: i16, y: i16) -> i16 {
    x * y
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__mul_i32(x: i32, y: i32) -> i32 {
    x * y
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__mul_i64(x: i64, y: i64) -> i64 {
    x * y
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__mul_u8(x: u8, y: u8) -> u8 {
    x * y
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__mul_u16(x: u16, y: u16) -> u16 {
    x * y
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__mul_u32(x: u32, y: u32) -> u32 {
    x * y
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__mul_u64(x: u64, y: u64) -> u64 {
    x * y
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __libprelude__mul_usize(x: usize, y: usize) -> usize {
    x * y
}