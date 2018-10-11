use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;
use std::env;

mod bridge;
use bridge::*;

extern "C" fn callback(index: i32, barcode_type: *const c_char, barcode_value: *const c_char) {
    unsafe {
        println!(
            "Index {}, {}, {}",
            index,
            CStr::from_ptr(barcode_type).to_str().unwrap(),
            CStr::from_ptr(barcode_value).to_str().unwrap()
        );
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Please input an image file.");
        return
    }

    println!("Hello Dynamsoft Barcode Reader!");
    unsafe {
        register_callback(Some(callback));
        let image_file = CString::new(env::args().nth(1).expect("Missing argument")).unwrap();
        let license = CString::new("t0068NQAAAFKYHV9xSZDEThUtClXNzxXH9TLSj/vYcY8mSKa0RxaGw3qNynyAMJ9Ib8UPxzFsbAMIugqPO313BvfiOdmZFTY=").unwrap();
        c_decodeFile(image_file.as_ptr(), license.as_ptr());
    }

    println!("Bye!");
}
