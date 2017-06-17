#![allow(unused_variables)]

// extern crate bindgen;
extern crate make_cmd;

use make_cmd::make;

use std::env;
use std::path::Path;
use std::process::Command;


const LIBSIXEL_DIR: &str = "libsixel";

fn main() {

    let testing_build = false;


    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);

    println!("cargo:rustc-link-lib=dylib=sixel");
    // println!("cargo:rustc-link-lib=static=sixel");
    println!("cargo:rustc-link-search=native={}", out_dir.join("lib").display());


    if testing_build {
        return;
    }


    let curl = has_feature("curl");
    let jpeg = has_feature("jpeg");
    let pixbuf = has_feature("pixbuf");
    let png = has_feature("png");
    let gd = has_feature("gd");
    let python_interface = has_feature("python_interface");


    let sixel_dir = Path::new(LIBSIXEL_DIR);


    {
        let mut cmd = Command::new("./configure");
        cmd.current_dir(sixel_dir)
            .arg("--prefix")
            .arg(out_dir);

        // cmd.arg("-fPIC");

        if curl {
            cmd.arg("--with-libcurl");
        }
        if gd {
            cmd.arg("--with-gd");
        }
        if pixbuf {
            cmd.arg("--with-gdk-pixbuf");
        }
        if jpeg {
            cmd.arg("--with-jpeg");
        }
        if png {
            cmd.arg("--with-png");
        }
        if !python_interface {
            cmd.arg("--without-python");
        }

        cmd.status().expect("Failed to execute ./configure");
          
        make()
            .arg("install")
            .current_dir(sixel_dir)
            .status().expect("Failed to execute make");

    }

    // generate_bindings(out_dir);
    

}

// fn generate_bindings(out_dir: &Path) {
//     let bindings = bindgen::Builder::default()
//         .no_unstable_rust()
//         .header("wrapper.h")
//         .hide_type("max_align_t")
//         .generate()
//         .expect("Unable to generate bindings");
//
//     bindings
//         .write_to_file(out_dir.join("bindings.rs"))
//         .expect("Couldn't write bindings");
// }

const FEATURE_PREFIX: &str = "CARGO_FEATURE_";
fn has_feature(feature: &'static str) -> bool {
    let feature = feature.to_owned().to_uppercase();
    let mut name = FEATURE_PREFIX.to_owned();
    name.push_str(&feature);
    env::var(name).is_ok()
}
