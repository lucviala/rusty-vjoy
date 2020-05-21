extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    //https://doc.rust-lang.org/rustc/command-line-arguments.html
    println!(r"cargo:rustc-link-search=C:\Program Files\vJoy\x64");
    println!("cargo:rustc-link-lib=vJoyInterface");

    //https://rust-lang.github.io/rust-bindgen
    let bindings = bindgen::Builder::default()
        .header("wrapper.hpp")
        //
        .whitelist_function("GetvJoyVersion")
        .whitelist_function("vJoyEnabled")
        .whitelist_function("GetvJoyProductString")
        .whitelist_function("GetvJoyManufacturerString")
        .whitelist_function("GetvJoySerialNumberString")
        .whitelist_function("DriverMatch")
        .whitelist_function("RegisterRemovalCB")
        .whitelist_function("vJoyFfbCap")
        .whitelist_function("GetvJoyMaxDevices")
        .whitelist_function("GetNumberExistingVJD")
        //
        .whitelist_function("GetVJDButtonNumber")
        .whitelist_function("GetVJDDiscPovNumber")
        .whitelist_function("GetVJDContPovNumber")
        .whitelist_function("GetVJDAxisExist")
        .whitelist_function("GetVJDAxisMax")
        .whitelist_function("GetVJDAxisMin")
        .whitelist_function("GetVJDStatus")
        .whitelist_function("isVJDExists")
        .whitelist_function("GetOwnerPid")
        //
        .whitelist_function("AcquireVJD")
        .whitelist_function("RelinquishVJD")
        .whitelist_function("UpdateVJD")
        //
        .whitelist_function("ResetVJD")
        .whitelist_function("ResetAll")
        .whitelist_function("ResetButtons")
        .whitelist_function("ResetPovs")
        //
        .whitelist_function("SetAxis")
        .whitelist_function("SetBtn")
        .whitelist_function("SetDiscPov")
        .whitelist_function("SetContPov")
        //
        //.whitelist_var("DEVICENAME_STRING")
        //.whitelist_var("NTDEVICE_NAME_STRING")
        //.whitelist_var("SYMBOLIC_NAME_STRING")
        //.whitelist_var("VER_X_")
        //.whitelist_var("VER_H_")
        //.whitelist_var("VER_M_")
        //.whitelist_var("VER_L_")
        //.whitelist_var("DOS_FILE_NAME")
        //.whitelist_var("VJOY_INTERFACE")
        //.whitelist_var("VENDOR_N_ID")
        //.whitelist_var("PRODUCT_N_ID")
        //.whitelist_var("VERSION_N")
        //.whitelist_var("VENDOR_STR_ID")
        //.whitelist_var("PRODUCT_STR_ID")
        //.whitelist_var("F_.*")
        //.whitelist_var("HID_USAGE_.*")
        //.whitelist_var("HID_ID_.*")
        //
        //.whitelist_type("HID_DEVICE_ATTRIBUTES")
        .whitelist_type("JOYSTICK_POSITION")
        .whitelist_type("JOYSTICK_POSITION_V2")
        .whitelist_type("DEVCTRLS")
        .whitelist_type("DeviceStat")
        .whitelist_type("DEV_INFO")
        //
        .rustified_enum("VjdStat")
        //
        .generate_comments(false)
        .layout_tests(false)
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        //.write_to_file(Path::new("../src").join("bindings.rs"))
        .expect("Couldn't write to file");
}
