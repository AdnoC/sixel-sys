extern crate sixel_sys as sixel;
#[macro_use]
extern crate lazy_static;

#[allow(unused_imports)]
use sixel::*;

use std::path::Path;
use std::ffi::CString;
use std::sync::Mutex;

lazy_static!{
    pub static ref lock: Mutex<()> = Mutex::new(());
}

#[test]
fn it_works() {
    println!("Libsixel version is {:?}", sixel::VERSION);

    let expected_ver: Version = Version {
        major: 1,
        minor: 7,
        patch: 3,
    };

    assert_eq!(expected_ver, sixel::VERSION);
}

// Note: Sixel images do not show up in tmux
fn print_sixel<P: AsRef<Path>>(path: P) {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::stdout;

    let path_str = path.as_ref().to_str().unwrap().to_owned();

    let mut file = File::open(path).unwrap();
    let mut contents: Vec<u8> = vec![];
    file.read_to_end(&mut contents);

    println!("Printing file {:?}", path_str);

    let stdout = stdout();
    let mut handle = stdout.lock();

    handle.write_all(&contents);
}

fn image_root<'a>() -> &'a Path {
    Path::new("libsixel/images/")
}

fn snake_of(extension: &str) -> CString {
    let snake_path = image_root()
        .join("snake")
        .with_extension(extension);

    let snake_path = snake_path.to_str().unwrap();

    CString::new(snake_path).unwrap()
}

/// Just make sure that it works
#[test]
fn convert_snake() {
    round_trip_with_ext("bmp");
    round_trip_with_ext("gif");
    round_trip_with_ext("pbm");
    round_trip_with_ext("pgm");
    round_trip_with_ext("ppm");
    round_trip_with_ext("tga");
    round_trip_with_ext("tiff");

    #[cfg(jpeg)]
    round_trip_with_ext("jpg");
    #[cfg(png)]
    round_trip_with_ext("png");
}


fn round_trip_with_ext(extension: &str) {
    use std::ptr;
    use std::os::raw::c_int;
    use std::os::raw::c_uint;
    println!("encoding with {}", extension);

    let assert_ok = |actual: Status| {
        assert_eq!(sixel::OK,
                   actual,
                   "when de/encoding snake from a {} file, sixel returned the status {} instead of ok",
                   extension,
                   actual);
    };

    let snake_sixel = "snake_test_".to_owned() + extension + ".six";
    let snake_new = "snake_test.".to_string() + extension;
    let snake_six_out = CString::new(snake_sixel.clone()).unwrap();
    let snake_new_out = CString::new(snake_new).unwrap();

    {
        let gaurd = lock.lock().unwrap();

        unsafe {
            let mut encoder: *mut sixel::Encoder = ptr::null_mut() as *mut _;

            let result = sixel_encoder_new(&mut encoder,
                                           ptr::null_mut() as *mut sixel::Allocator);
            assert_ok(result);


            let result = sixel_encoder_setopt(encoder,
                                              Optflag::OutFile,
                                              snake_six_out.as_ptr());
            assert_ok(result);

            let snake_path = snake_of(extension);

            let result = sixel_encoder_encode(encoder, snake_path.as_ptr());
            assert_ok(result);

            sixel_encoder_unref(encoder);
        }

        unsafe {
            let mut decoder: *mut sixel::Decoder = ptr::null_mut() as *mut _;

            let result = sixel_decoder_new(&mut decoder,
                                           ptr::null_mut() as *mut sixel::Allocator);
            assert_ok(result);


            let result = sixel_decoder_setopt(decoder,
                                              DecoderOptflag::Input,
                                              snake_six_out.as_ptr());
            assert_ok(result);

            let result = sixel_decoder_setopt(decoder,
                                              DecoderOptflag::Output,
                                              snake_new_out.as_ptr());
            assert_ok(result);

            let result = sixel_decoder_decode(decoder);
            assert_ok(result);

            sixel_decoder_unref(decoder);
        }
    }
    print_sixel(snake_sixel);
}


