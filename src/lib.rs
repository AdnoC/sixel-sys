//! Bindings to libsixel
//!
//! For additional information, please check [its repo](https://github.com/saitoha/libsixel)

#![allow(non_camel_case_types)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use std::ffi::CString;

    #[test]
    fn it_works() {
        println!("Bindings file is {}", concat!(env!("OUT_DIR"), "/bindings.rs"));

        println!("Libsixel version is {:?}", super::LIBSIXEL_VERSION);
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

        let assert_ok = |actual: SIXELSTATUS| {
            assert_eq!(SIXEL_OK,
                       actual as c_uint,
                       "when de/encoding snake from a {} file, sixel returned the status {} instead of ok",
                       extension,
                       actual);
        };

        let snake_sixel = "snake_test.six";
        let snake_new = "snake_test.".to_string() + extension;
        let snake_six_out = CString::new(snake_sixel).unwrap();
        let snake_new_out = CString::new(snake_new).unwrap();


        unsafe {
            let mut encoder: *mut sixel_encoder_t = ptr::null_mut() as *mut _;

            let result = sixel_encoder_new(&mut encoder,
                                           ptr::null_mut() as *mut sixel_allocator_t);
            assert_ok(result);


            let result = sixel_encoder_setopt(encoder,
                                              SIXEL_OPTFLAG_OUTFILE as c_int,
                                              snake_six_out.as_ptr());
            assert_ok(result);

            let snake_path = snake_of(extension);

            let result = sixel_encoder_encode(encoder, snake_path.as_ptr());
            assert_ok(result);

            sixel_encoder_unref(encoder);
        }

        unsafe {
            let mut decoder: *mut sixel_decoder_t = ptr::null_mut() as *mut _;

            let result = sixel_decoder_new(&mut decoder,
                                           ptr::null_mut() as *mut sixel_allocator_t);
            assert_ok(result);


            let result = sixel_decoder_setopt(decoder,
                                              SIXEL_OPTFLAG_INPUT as c_int,
                                              snake_six_out.as_ptr());
            assert_ok(result);

            let result = sixel_decoder_setopt(decoder,
                                              SIXEL_OPTFLAG_OUTFILE as c_int,
                                              snake_new_out.as_ptr());
            assert_ok(result);

            let result = sixel_decoder_decode(decoder);
            assert_ok(result);

            sixel_decoder_unref(decoder);
        }
    }
}
