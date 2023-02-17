use std::ffi::CStr;

mod bindings {
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
