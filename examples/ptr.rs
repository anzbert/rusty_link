use rusty_link::TestStruct;
use std::os::raw::c_void;

fn main() {
    let mut test_struct = TestStruct { number: 99 };

    let to_rust_reference = &mut test_struct;

    let to_c_ptr = to_rust_reference as *mut _ as *mut c_void;

    let back_to_rust_ref = unsafe { &mut *(to_c_ptr as *mut TestStruct) };

    println!("state: {}", back_to_rust_ref.number);
}
