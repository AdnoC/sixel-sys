#![allow(unused_variables)]

extern crate make_cmd;

use make_cmd::make;

use std::env;
use std::path::Path;
use std::process::Command;


const LIBSIXEL_DIR: &str = "libsixel";

fn main() {

    // println!("--- BEGIN VARS ---");
    // for (key, val) in env::vars() {
    //     println!("{}: {}", key, val);
    // }
    // println!("--- END VARS ---");
    let curl = has_feature("curl");
    let jpeg = has_feature("jpeg");
    let pixbuf = has_feature("pixbuf");
    let png = has_feature("png");
    let gd = has_feature("gd");
    let python_interface = has_feature("python_interface");


    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);
    let sixel_dir = Path::new(LIBSIXEL_DIR);


    {
        // ./configure
        // make
        // make install
        println!("Running ./configure");
        let mut cmd = Command::new("./configure");
        cmd.current_dir(sixel_dir)
            .arg("--prefix")
            .arg(out_dir);

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


        // cmd.status();
    }
}

const FEATURE_PREFIX: &str = "CARGO_FEATURE_";
fn has_feature(feature: &'static str) -> bool {
    let feature = feature.to_owned().to_uppercase();
    let mut name = FEATURE_PREFIX.to_owned();
    name.push_str(&feature);
    env::var(name).is_ok()
}
