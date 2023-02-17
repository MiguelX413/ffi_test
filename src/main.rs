use std::ffi::CStr;

mod bindings {
    #![allow(dead_code)]
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

fn main() {
    println!("xd");
    unsafe {
        bindings::sleep(1);
    }
    println!("xd");
    let login = unsafe { CStr::from_ptr(bindings::getlogin()) };
    println!("{}", login.to_string_lossy());
}
