extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    //https://doc.rust-lang.org/rustc/command-line-arguments.html
    println!(r"cargo:rustc-link-search=C:\Program Files\vJoy\x64");
    println!("cargo:rustc-link-lib=vJoyInterface");

    //https://rust-lang.github.io/rust-bindgen
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_args(&["-x", "c++"])
        //
        .allowlist_function("GetvJoyVersion")
        .allowlist_function("vJoyEnabled")
        .allowlist_function("GetvJoyProductString")
        .allowlist_function("GetvJoyManufacturerString")
        .allowlist_function("GetvJoySerialNumberString")
        .allowlist_function("DriverMatch")
        .allowlist_function("RegisterRemovalCB")
        .allowlist_function("vJoyFfbCap")
        .allowlist_function("GetvJoyMaxDevices")
        .allowlist_function("GetNumberExistingVJD")
        //
        .allowlist_function("GetVJDButtonNumber")
        .allowlist_function("GetVJDDiscPovNumber")
        .allowlist_function("GetVJDContPovNumber")
        .allowlist_function("GetVJDAxisExist")
        .allowlist_function("GetVJDAxisMax")
        .allowlist_function("GetVJDAxisMin")
        .allowlist_function("GetVJDStatus")
        .allowlist_function("isVJDExists")
        .allowlist_function("GetOwnerPid")
        //
        .allowlist_function("AcquireVJD")
        .allowlist_function("RelinquishVJD")
        .allowlist_function("UpdateVJD")
        //
        .allowlist_function("ResetVJD")
        .allowlist_function("ResetAll")
        .allowlist_function("ResetButtons")
        .allowlist_function("ResetPovs")
        //
        // .allowlist_function("SetAxis")
        // .allowlist_function("SetBtn")
        // .allowlist_function("SetDiscPov")
        // .allowlist_function("SetContPov")
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
        //.allowlist_type("HID_DEVICE_ATTRIBUTES")
        // .allowlist_type("JOYSTICK_POSITION")
        // .allowlist_type("JOYSTICK_POSITION_V2")
        // .allowlist_type("DEVCTRLS")
        // .allowlist_type("DeviceStat")
        // .allowlist_type("DEV_INFO")
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
