mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

fn main() {
    println!("xd");
    unsafe {
        bindings::sleep(1);
    }
    println!("xd");
}
