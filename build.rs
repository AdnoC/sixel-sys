#![allow(unused_variables)]

// extern crate bindgen;
extern crate make_cmd;

use make_cmd::make;

use std::env;
use std::path::Path;
use std::fs::canonicalize;
use std::process::Command;


const LIBSIXEL_DIR: &str = "libsixel";

fn main() {

    let testing_build = false;


    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);



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
    let sixel_build_dir = sixel_dir.join("build");
    let sixel_build_dir_prefix = if std::env::var_os("CARGO_CFG_WINDOWS").is_some() {
        println!("cargo::warning=Detected Windows compilation. Attempting to use MinGW compilers.");
        sixel_prefix("build")
    } else { sixel_build_dir.clone().into_os_string().into_string().expect("Could not convert OS path to utf8") };


    {
        let mut cmd = Command::new("sh");
        cmd.current_dir(sixel_dir)
            .arg("configure")

            .arg("--prefix")
            .arg(sixel_build_dir_prefix.clone());

        //cmd.arg("-fPIC");

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
        if python_interface {
            cmd.arg("--enable-python");
        }

        cmd.status().expect("Failed to execute ./configure");
          
        make()
            .arg("install")
            .current_dir(sixel_dir)
            .status().expect("Failed to execute make");

    }

//println!("cargo::warning=p1: {}", canonicalize(&sixel_build_dir_prefix).unwrap().display());
println!("cargo::warning=p2: {}", canonicalize(&sixel_build_dir.join("lib")).unwrap().display());

    // generate_bindings(out_dir);
    println!("cargo:rustc-link-lib=static=sixel");
    // println!("cargo:rustc-link-lib=static=sixel");
    println!("cargo:rustc-link-search=native={}", canonicalize(&sixel_build_dir).unwrap().display()); //out_dir.join(".libs").display());

    println!("cargo:rustc-link-search=native={}", canonicalize(&sixel_build_dir.join("lib")).unwrap().display()); //out_dir.join(".libs").display());


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

fn sixel_prefix(directory: &str) -> String {
    let cmd = Command::new("pwd").output().expect("Could not run `pwd`");
    let base_path = std::str::from_utf8(&cmd.stdout)
      .expect("Could not turn libsixel path into utf8")
      .trim()
      .to_string();
    let path = base_path + "/" + LIBSIXEL_DIR + "/" + "build";
    path
}
