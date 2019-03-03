extern crate rayon;
use rayon::prelude::*;

use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn string_array() -> *const *const u8 {
    let v = vec!["Hello\0".as_ptr(), "World\0".as_ptr()];
    let p = v.as_ptr();
    std::mem::forget(v);
    p
}

fn reverse_strings(origin: Vec<String>) -> Vec<String> {
    origin
        .par_iter()
        .map(|s| s.chars().rev().collect::<String>())
        .collect()
}
fn strings_to_pointers(origin: Vec<String>) -> Vec<*const i8> {
    let mut v: Vec<*const i8> = Vec::new();

    for s in &origin {
        v.push(CString::new(s.as_str()).unwrap().into_raw());
    }
    v
}

#[no_mangle]
pub extern "C" fn receive_strings(buffer: *mut *const c_char, length: usize) {
    assert!(!buffer.is_null());
    let seq = unsafe { std::slice::from_raw_parts_mut(buffer, length) };
    let tmp: Vec<&[u8]> = unsafe { seq.iter().map(|x| CStr::from_ptr(*x).to_bytes()).collect() };
    let parsed: Vec<String> = tmp
        .iter()
        .map(|x| std::str::from_utf8(x).unwrap().to_string())
        .collect();
    println!("income: {:?}", parsed);
}

#[no_mangle]
pub extern "C" fn reverse_input(buffer: *mut *const c_char, length: usize) -> *const *const i8 {
    // Must to check that buffer points to something (not null)
    assert!(!buffer.is_null());
    let seq = unsafe { std::slice::from_raw_parts_mut(buffer, length) };
    // Getting a vec with pointers to bytes (as [&u8])
    let tmp: Vec<&[u8]> = unsafe { seq.iter().map(|x| CStr::from_ptr(*x).to_bytes()).collect() };

    // Transforming bytes to Strings, to work with. 
    let origin: Vec<String> = tmp
        .iter()
        .map(|x| std::str::from_utf8(x).unwrap().to_string())
        .collect();
    
    // Reversing each string in vec (string to gnirts)
    let reversed = reverse_strings(origin);
    // Transforming reversed data -- collecting raw pointers on each string
    // Because of it was CString, and not String, each pointer will be *c_char (i8), not u8
    let v = strings_to_pointers(reversed);
    
    // Getting raw pointer on vec with raw pointers on strings
    let p = v.as_ptr();
    // This means to forget to destroy the vec, to be able to read its value from the outside.
    std::mem::forget(v);
    p
}
