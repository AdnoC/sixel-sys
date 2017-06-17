#![allow(non_camel_case_types)]

use std::os::raw::{c_void, c_int, c_char, c_uchar};

pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}

pub const VERSION: Version = Version {
    major: 1,
    minor: 7,
    patch: 3,
};

pub const ABI_VERSION: Version = Version {
    major: 1,
    minor: 6,
    patch: 0,
};


pub const OUTPUT_PACKET_SIZE: u16 = 16384;

pub const PALETTE_MIN: u16 = 2;
pub const PALETTE_MAX: u16 = 256;

pub const USE_DEPRECATED_SYMBOLS: bool = true;


// In impl crate: function to convert these to enums

/// Describes why a function returned
pub type Status = c_int;

/// Suceeded
pub const OK: Status = 0x0000;
/// Failed
///
/// Renamed from "FALSE"
pub const ERR: Status = 0x1000;

pub const RUNTIME_ERROR: Status = ERR | 0x0100;
pub const LOGIC_ERROR: Status = ERR | 0x0200;
pub const FEATURE_ERROR: Status = ERR | 0x0300;
pub const LIBC_ERROR: Status = ERR | 0x0400;
pub const CURL_ERROR: Status = ERR | 0x0500;
pub const JPEG_ERROR: Status = ERR | 0x0600;
pub const PNG_ERROR: Status = ERR | 0x0700;
pub const GDK_ERROR: Status = ERR | 0x0800;
pub const GD_ERROR: Status = ERR | 0x0900;
pub const STBI_ERROR: Status = ERR | 0x0a00;
pub const STBIW_ERROR: Status = ERR | 0x0b00;

/// Interrupted by a signal
pub const INTERRUPTED: Status = OK | 0x0001;

/// `malloc()` failed
pub const BAD_ALLOCATION: Status = RUNTIME_ERROR | 0x0001;
pub const BAD_ARGUMENT: Status = RUNTIME_ERROR | 0x0002;
pub const BAD_INPUT: Status = RUNTIME_ERROR | 0x0002;

/// Feature not implemented
pub const NOT_IMPLEMENTED: Status = FEATURE_ERROR | 0x0001;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
/// Output character size
pub enum CharacterSize {
    SevenBit = 0,
    EightBit = 1,
}


#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
/// Method for finding the largest dimension for splitting,
/// and sorting by that component.
pub enum MethodForLargest {
    Auto = 0,
    /// Simply comparing the range in RGB space
    Normal = 1,
    /// Transforming into luminosities before the comparison
    Luminosity = 2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
/// Method for choosing the a color from the box
pub enum MethodForRepColor {
    Auto = 0,
    CenterOfBox = 1,
    /// Method is described in Heckbert's paper
    AverageColor = 2,
    AveragePixels = 3,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DiffusionMethod {
    Auto = 0,
    /// Don't diffuse
    None = 1,
    /// Use Bill Atkinson's method
    Atkinson = 2,
    /// Use Floyd-Steinberg method
    FS = 3,
    /// Use Jarvis, Judice, & Ninke method
    JaJuNi = 4,
    /// Use Stucki's method
    Stucki = 5,
    /// Use Burkes' method
    Burkes = 6,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
/// Quality of palette
pub enum QualityMode {
    Auto = 0,
    High = 1,
    Low = 2,
    Full = 3,
    HighColor = 4,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BuiltinDither {
    /// Monochrome terminal with dark background
    MonoDark = 0,
    /// Monochrome terminal with light background
    ///
    /// Note: libsixel documentation says it is for a dark background...
    MonoLight = 1,
    XTerm16 = 2,
    XTerm256 = 3,
    VT340Mono = 4,
    VT340Color = 5,
    /// 1 bit grayscale
    G1 = 6,
    /// 2 bit grayscale
    G2 = 7,
    /// 4 bit grayscale
    G4 = 8,
    /// 8 bit grayscale
    G8 = 9,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
/// Offset value used for the values of PixelFormat
pub enum FormatType {
    Color = 0,
    Grayscale = 64,
    Palette = 128,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
/// Pixel format used in input image
pub enum PixelFormat {
    /// 15bpp
    RGB555 = FormatType::Color as isize | 0x01,
    /// 16bpp
    RGB565 = FormatType::Color as isize | 0x02,
    /// 24bpp
    RGB888 = FormatType::Color as isize | 0x03,
    /// 15bpp
    BGR555 = FormatType::Color as isize | 0x04,
    /// 16bpp
    BGR565 = FormatType::Color as isize | 0x05,
    /// 24bpp
    BGR888 = FormatType::Color as isize | 0x06,
    /// 32bpp
    ARGB8888 = FormatType::Color as isize | 0x10,
    /// 32bpp
    RGBA8888 = FormatType::Color as isize | 0x11,
    /// 32bpp
    ABGR8888 = FormatType::Color as isize | 0x12,
    /// 32bpp
    BGRA8888 = FormatType::Color as isize | 0x13,
    /// 1bpp grayscale
    G1 = FormatType::Grayscale as isize | 0x00,
    /// 2bpp grayscale
    G2 = FormatType::Grayscale as isize | 0x01,
    /// 4bpp grayscale
    G4 = FormatType::Grayscale as isize | 0x02,
    /// 8bpp grayscale
    G8 = FormatType::Grayscale as isize | 0x03,
    /// 16bpp grayscale with alpha
    AG88 = FormatType::Grayscale as isize | 0x13,
    /// 16bpp grayscale with alpha
    GA88 = FormatType::Grayscale as isize | 0x23,
    /// 1bpp palette
    Pal1 = FormatType::Palette as isize | 0x00,
    /// 2bpp palette
    Pal2 = FormatType::Palette as isize | 0x01,
    /// 4bpp palette
    Pal4 = FormatType::Palette as isize | 0x02,
    /// 8bpp palette
    Pal8 = FormatType::Palette as isize | 0x03,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PaletteType {
    Auto = 0,
    /// HLS colorspace
    HLS = 1,
    /// RGB colorspace
    RGB = 2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
/// Policy used when encoding
pub enum EncodePolicy {
    Auto = 0,
    /// Encode as fast as posible
    Fast = 1,
    /// Encode to the smallest sixel sequence as possible
    Size = 2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
/// What filter to use when resampling
pub enum ResamplingMethod {
    Nearest = 0,
    Gaussian = 1,
    Hanning = 2,
    Hamming = 3,
    Bilinear = 4,
    Welsh = 5,
    Bicubic = 6,
    Lanczos2 = 7,
    Lanczos3 = 8,
    Lanczos4 = 9,
}


#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ImageFormat {
    GIF = 0,
    PNG = 1,
    BMP = 2,
    JPG = 3,
    TGA = 4,
    WBMP = 5,
    TIFF = 6,
    SIXEL = 7,
    PNM = 8,
    GD2 = 9,
    PSD = 10,
    FORMAT_HDR = 11,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
/// How to treat GIF animations that loop
pub enum LoopMode {
    /// Honor the setting of the GIF header
    Auto = 0,
    /// Always loop
    Force = 1,
    /// Never loop
    Disable = 2,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
/// Flags used in the easy encoder/decoder API.
///
/// Flags are the same as the ones used in the img2sixel executable
pub enum Optflag {
    // CONFLICTS WITH INVERT FLAG
    // /// Specify input file name
    // Input = b'i',
    /// Specify output file name
    Output = b'o',
    // OutFile = b'o',
    /// Use sixel images for 7 bit terminals or printers
    ///
    /// Default of the 2 bit mode flags
    UseSevenBitMode = b'7',
    /// Use sixel images for 8 bit terminals or printers
    UseEightBitMode = b'8',
    /// Limit the arguments of DECGRI('!') to 255
    HasGRIArgLimit = b'R',
    /// Specify the number of colors to reduce the image to
    ///
    /// Defaults to 256
    ///
    /// Renamed from "SIXEL_OPTFLAG_COLORS"
    NumColors = b'p',
    /// Give a file that specifies a set of colors
    ///
    /// Transforms the image to match the set
    Mapfile = b'm',
    /// Output a monochrome sixel image
    ///
    /// Assumes the terminal background color is black
    Monochrome = b'e',
    /// Connect to SSL sites without certs
    ///
    /// Only applicable if libcurl is used
    Insecure = b'k',
    /// Assume the terminal background color is white
    ///
    /// Only applicable when also using `Optflag::Monochrome`
    InvertBackground = b'i',
    /// Output a 15bpp sixel image
    UseHighColor = b'I',
    /// Use DECDMAC and DEVINVM sequences to optimize GIF animation rendering
    UseMacro = b'u',
    /// Specify a macro register number
    MacroNumber = b'n',
    /// Specify the number of arguments for the score of complexion correction
    ///
    /// Must be 1 or more
    ComplexionScore = b'C',
    /// Ignore delay when rendering GIF animations
    IgnoreGIFDelay = b'g',
    /// Render an animated GIF as a static image
    StaticGIF = b'S',
    /// Choose a diffusion method to be used with the NumColors option
    ///
    /// Should be a [`DiffusionMethod`]
    /// [`DiffusionMethod`]: enum.DiffusionMethod.html
    Diffusion = b'd',
    /// Choose a method for finding the largest dimension of median cut boxes
    /// for splitting
    ///
    /// Should be a [`MethodForLargest`]
    /// [`MethodForLargest`]: enum.MethodForLargest.html
    FindLargest = b'f',
    /// Choose the method for selecting the representative color from each
    /// median-cut box.
    ///
    /// Only makes sense when the `Optflag::NumColors` is used
    ///
    /// Should be a [`MethodForRepColor`]
    /// [`MethodForRepColor`]: enum.MethodForRepColor.html
    SelectColor = b's',
    /// Crop a source image to fit some geometry
    ///
    /// String format representation is "%dx%d+%d+%d", which is width, height,
    /// x, y
    CropRegion = b'c',
    /// Resize an image to a specified width
    ///
    /// Uses the following syntax:
    ///
    /// * `auto`
    ///
    /// * `<number>%`  scale with a percentage
    ///
    /// * `<number>`   scale a number of pixel counts
    ///
    /// * `<number>px` scale a number of pixel counts
    Width = b'w',
    /// Resize an image to a specified height
    ///
    /// Uses the following syntax:
    ///
    /// * `auto`
    ///
    /// * `<number>%`  scale with a percentage
    ///
    /// * `<number>`   scale a number of pixel counts
    ///
    /// * `<number>px` scale a number of pixel counts
    Height = b'h',
    /// Choose a filter for resampling when scaling the image due to
    /// `Optflag::Width` or `Optflag::Height`
    ///
    /// Should be a [`ResamplingMethod`]
    /// [`ResamplingMethod`]: enum.ResamplingMethod.html
    Resampling = b'r',
    /// Selects quality of color quanlization
    ///
    /// Should be a [`QualityMode`]
    /// [`QualityMode`]: enum.QualityMode.html
    QualityMode = b'q',
    /// Select loop behavior for animated GIFs
    ///
    /// Should be a [`LoopMode`]
    /// [`LoopMode`]: enum.LoopMode.html
    LoopMode = b'l',
    /// Select a palette color space
    ///
    /// Should be a [`PaletteType`]
    /// [`PaletteType`]: enum.PaletteType.html
    PaletteType = b't',
    /// Choose a built-in palette type
    ///
    /// Should be a [`BuiltinDither`]
    /// [`BuiltinDither`]: enum.BuiltinDither.html
    BuiltinPalette = b'b',
    /// Choose an encoding policy
    ///
    /// Should be a [`EncodePolicy`]
    /// [`EncodePolicy`]: enum.EncodingPolicy.html
    EncodingPolicy = b'E',
    /// Specify a background color
    ///
    /// Represented by the following syntax:
    ///
    /// ```
    /// #rgb
    /// #rrggbb
    /// #rrrgggbbb
    /// #rrrrggggbbbb
    /// rgb:r/g/b
    /// rgb:rr/gg/bb
    /// rgb:rrr/ggg/bbb
    /// rgb:rrrr/gggg/bbbb
    /// ```
    BackgroundColor = b'B',
    /// Penetrate GNU Screen using DCS pass-through sequence
    PenetrateScreen = b'P',
    /// Read source images from stdin continuously
    PipeInput = b'D',
    /// Print debugging info
    Verbose = b'v',
    /// Print version and license info
    Version = b'V',
    /// Print a help message
    Help = b'H',
}

pub type malloc_function = Option<unsafe extern "C" fn(size: usize) -> *mut c_void>;
pub type calloc_function = Option<unsafe extern "C" fn(num_items: usize, size: usize) -> *mut c_void>;
pub type realloc_function = Option<unsafe extern "C" fn(object: *mut c_void, new_size: usize) -> *mut c_void>;
pub type free_function = Option<unsafe extern "C" fn(object: *mut c_void)>;

/// Can be passed to API functions to control allocation
pub enum Allocator {}

extern "C" {
    /// Create a new allocator
    pub fn sixel_allocator_new(ppallocator: *mut *mut Allocator,
                               fn_malloc: malloc_function,
                               fn_calloc: calloc_function,
                               fn_realloc: realloc_function,
                               fn_free: free_function)
                               -> Status;
    pub fn sixel_allocator_ref(allocator: *mut Allocator);
    pub fn sixel_allocator_unref(allocator: *mut Allocator);
    pub fn sixel_allocator_malloc(allocator: *mut Allocator, n: usize) -> *mut c_void;
    pub fn sixel_allocator_calloc(allocator: *mut Allocator,
                                  nelm: usize,
                                  elsize: usize)
                                  -> *mut c_void;
    pub fn sixel_allocator_realloc(allocator: *mut Allocator,
                                   p: *mut c_void,
                                   n: usize)
                                   -> *mut c_void;
    pub fn sixel_allocator_free(allocator: *mut Allocator, p: *mut c_void);
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Output([u8; 0]);
pub type sixel_write_function = ::std::option::Option<unsafe extern "C" fn(data: *mut c_char,
                                                                           size: c_int,
                                                                           priv_: *mut c_void)
                                                                           -> c_int>;
extern "C" {
    pub fn sixel_output_new(output: *mut *mut Output,
                            fn_write: sixel_write_function,
                            priv_: *mut c_void,
                            allocator: *mut Allocator)
                            -> Status;
}
extern "C" {
    pub fn sixel_output_create(fn_write: sixel_write_function,
                               priv_: *mut c_void)
                               -> *mut Output;
}
extern "C" {
    pub fn sixel_output_destroy(output: *mut Output);
}
extern "C" {
    pub fn sixel_output_ref(output: *mut Output);
}
extern "C" {
    pub fn sixel_output_unref(output: *mut Output);
}
extern "C" {
    pub fn sixel_output_get_8bit_availability(output: *mut Output) -> c_int;
}
extern "C" {
    pub fn sixel_output_set_8bit_availability(output: *mut Output, availability: c_int);
}
extern "C" {
    pub fn sixel_output_set_gri_arg_limit(output: *mut Output, value: c_int);
}
extern "C" {
    pub fn sixel_output_set_penetrate_multiplexer(output: *mut Output, penetrate: c_int);
}
extern "C" {
    pub fn sixel_output_set_skip_dcs_envelope(output: *mut Output, skip: c_int);
}
extern "C" {
    pub fn sixel_output_set_palette_type(output: *mut Output, palettetype: c_int);
}
extern "C" {
    pub fn sixel_output_set_encode_policy(output: *mut Output, encode_policy: c_int);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dither([u8; 0]);
extern "C" {
    pub fn sixel_dither_new(ppdither: *mut *mut Dither,
                            ncolors: c_int,
                            allocator: *mut Allocator)
                            -> Status;
}
extern "C" {
    pub fn sixel_dither_create(ncolors: c_int) -> *mut Dither;
}
extern "C" {
    pub fn sixel_dither_get(builtin_dither: c_int) -> *mut Dither;
}
extern "C" {
    pub fn sixel_dither_destroy(dither: *mut Dither);
}
extern "C" {
    pub fn sixel_dither_ref(dither: *mut Dither);
}
extern "C" {
    pub fn sixel_dither_unref(dither: *mut Dither);
}
extern "C" {
    pub fn sixel_dither_initialize(dither: *mut Dither,
                                   data: *mut c_uchar,
                                   width: c_int,
                                   height: c_int,
                                   pixelformat: c_int,
                                   method_for_largest: c_int,
                                   method_for_rep: c_int,
                                   quality_mode: c_int)
                                   -> Status;
}
extern "C" {
    pub fn sixel_dither_set_diffusion_type(dither: *mut Dither,
                                           method_for_diffuse: c_int);
}
extern "C" {
    pub fn sixel_dither_get_num_of_palette_colors(dither: *mut Dither) -> c_int;
}
extern "C" {
    pub fn sixel_dither_get_num_of_histogram_colors(dither: *mut Dither) -> c_int;
}
extern "C" {
    pub fn sixel_dither_get_num_of_histgram_colors(dither: *mut Dither) -> c_int;
}
extern "C" {
    pub fn sixel_dither_get_palette(dither: *mut Dither) -> *mut c_uchar;
}
extern "C" {
    pub fn sixel_dither_set_palette(dither: *mut Dither, palette: *mut c_uchar);
}
extern "C" {
    pub fn sixel_dither_set_complexion_score(dither: *mut Dither, score: c_int);
}
extern "C" {
    pub fn sixel_dither_set_body_only(dither: *mut Dither, bodyonly: c_int);
}
extern "C" {
    pub fn sixel_dither_set_optimize_palette(dither: *mut Dither, do_opt: c_int);
}
extern "C" {
    pub fn sixel_dither_set_pixelformat(dither: *mut Dither, pixelformat: c_int);
}
extern "C" {
    pub fn sixel_dither_set_transparent(dither: *mut Dither, transparent: c_int);
}
pub type sixel_allocator_function = ::std::option::Option<unsafe extern "C" fn(size: usize)
                                                                               -> *mut c_void>;
extern "C" {
    pub fn sixel_encode(pixels: *mut c_uchar,
                        width: c_int,
                        height: c_int,
                        depth: c_int,
                        dither: *mut Dither,
                        context: *mut Output)
                        -> Status;
}
extern "C" {
    pub fn sixel_decode_raw(p: *mut c_uchar,
                            len: c_int,
                            pixels: *mut *mut c_uchar,
                            pwidth: *mut c_int,
                            pheight: *mut c_int,
                            palette: *mut *mut c_uchar,
                            ncolors: *mut c_int,
                            allocator: *mut Allocator)
                            -> Status;
}
extern "C" {
    pub fn sixel_decode(sixels: *mut c_uchar,
                        size: c_int,
                        pixels: *mut *mut c_uchar,
                        pwidth: *mut c_int,
                        pheight: *mut c_int,
                        palette: *mut *mut c_uchar,
                        ncolors: *mut c_int,
                        fn_malloc: sixel_allocator_function)
                        -> Status;
}
extern "C" {
    pub fn sixel_helper_set_additional_message(message: *const c_char);
}
extern "C" {
    pub fn sixel_helper_get_additional_message() -> *const c_char;
}
extern "C" {
    pub fn sixel_helper_format_error(status: Status) -> *const c_char;
}
extern "C" {
    pub fn sixel_helper_compute_depth(pixelformat: c_int) -> c_int;
}
extern "C" {
    pub fn sixel_helper_normalize_pixelformat(dst: *mut c_uchar,
                                              dst_pixelformat: *mut c_int,
                                              src: *const c_uchar,
                                              src_pixelformat: c_int,
                                              width: c_int,
                                              height: c_int)
                                              -> Status;
}
extern "C" {
    pub fn sixel_helper_scale_image(dst: *mut c_uchar,
                                    src: *const c_uchar,
                                    srcw: c_int,
                                    srch: c_int,
                                    pixelformat: c_int,
                                    dstw: c_int,
                                    dsth: c_int,
                                    method_for_resampling: c_int,
                                    allocator: *mut Allocator)
                                    -> Status;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Frame([u8; 0]);
extern "C" {
    pub fn sixel_frame_new(ppframe: *mut *mut Frame,
                           allocator: *mut Allocator)
                           -> Status;
}
extern "C" {
    pub fn sixel_frame_create() -> *mut Frame;
}
extern "C" {
    pub fn sixel_frame_ref(frame: *mut Frame);
}
extern "C" {
    pub fn sixel_frame_unref(frame: *mut Frame);
}
extern "C" {
    pub fn sixel_frame_init(frame: *mut Frame,
                            pixels: *mut c_uchar,
                            width: c_int,
                            height: c_int,
                            pixelformat: c_int,
                            palette: *mut c_uchar,
                            ncolors: c_int)
                            -> Status;
}
extern "C" {
    pub fn sixel_frame_get_pixels(frame: *mut Frame) -> *mut c_uchar;
}
extern "C" {
    pub fn sixel_frame_get_palette(frame: *mut Frame) -> *mut c_uchar;
}
extern "C" {
    pub fn sixel_frame_get_width(frame: *mut Frame) -> c_int;
}
extern "C" {
    pub fn sixel_frame_get_height(frame: *mut Frame) -> c_int;
}
extern "C" {
    pub fn sixel_frame_get_ncolors(frame: *mut Frame) -> c_int;
}
extern "C" {
    pub fn sixel_frame_get_pixelformat(frame: *mut Frame) -> c_int;
}
extern "C" {
    pub fn sixel_frame_get_transparent(frame: *mut Frame) -> c_int;
}
extern "C" {
    pub fn sixel_frame_get_multiframe(frame: *mut Frame) -> c_int;
}
extern "C" {
    pub fn sixel_frame_get_delay(frame: *mut Frame) -> c_int;
}
extern "C" {
    pub fn sixel_frame_get_frame_no(frame: *mut Frame) -> c_int;
}
extern "C" {
    pub fn sixel_frame_get_loop_no(frame: *mut Frame) -> c_int;
}
extern "C" {
    pub fn sixel_frame_strip_alpha(frame: *mut Frame, bgcolor: *mut c_uchar) -> c_int;
}
extern "C" {
    pub fn sixel_frame_resize(frame: *mut Frame,
                              width: c_int,
                              height: c_int,
                              method_for_resampling: c_int)
                              -> Status;
}
extern "C" {
    pub fn sixel_frame_clip(frame: *mut Frame,
                            x: c_int,
                            y: c_int,
                            width: c_int,
                            height: c_int)
                            -> Status;
}
pub type sixel_load_image_function =
    ::std::option::Option<unsafe extern "C" fn(frame: *mut Frame, context: *mut c_void)
                                               -> Status>;
extern "C" {
    pub fn sixel_helper_load_image_file(filename: *const c_char,
                                        fstatic: c_int,
                                        fuse_palette: c_int,
                                        reqcolors: c_int,
                                        bgcolor: *mut c_uchar,
                                        loop_control: c_int,
                                        fn_load: sixel_load_image_function,
                                        finsecure: c_int,
                                        cancel_flag: *const c_int,
                                        context: *mut c_void,
                                        allocator: *mut Allocator)
                                        -> Status;
}
extern "C" {
    pub fn sixel_helper_write_image_file(data: *mut c_uchar,
                                         width: c_int,
                                         height: c_int,
                                         palette: *mut c_uchar,
                                         pixelformat: c_int,
                                         filename: *const c_char,
                                         imageformat: c_int,
                                         allocator: *mut Allocator)
                                         -> Status;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Encoder([u8; 0]);
extern "C" {
    pub fn sixel_encoder_new(ppencoder: *mut *mut Encoder,
                             allocator: *mut Allocator)
                             -> Status;
}
extern "C" {
    pub fn sixel_encoder_create() -> *mut Encoder;
}
extern "C" {
    pub fn sixel_encoder_ref(encoder: *mut Encoder);
}
extern "C" {
    pub fn sixel_encoder_unref(encoder: *mut Encoder);
}
extern "C" {
    pub fn sixel_encoder_set_cancel_flag(encoder: *mut Encoder,
                                         cancel_flag: *mut c_int)
                                         -> Status;
}
extern "C" {
    pub fn sixel_encoder_setopt(encoder: *mut Encoder,
                                arg: c_int,
                                optarg: *const c_char)
                                -> Status;
}
extern "C" {
    pub fn sixel_encoder_encode(encoder: *mut Encoder, filename: *const c_char) -> Status;
}
extern "C" {
    pub fn sixel_encoder_encode_bytes(encoder: *mut Encoder,
                                      bytes: *mut c_uchar,
                                      width: c_int,
                                      height: c_int,
                                      pixelformat: c_int,
                                      palette: *mut c_uchar,
                                      ncolors: c_int)
                                      -> Status;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Decoder([u8; 0]);
extern "C" {
    pub fn sixel_decoder_new(ppdecoder: *mut *mut Decoder,
                             allocator: *mut Allocator)
                             -> Status;
}
extern "C" {
    pub fn sixel_decoder_create() -> *mut Decoder;
}
extern "C" {
    pub fn sixel_decoder_ref(decoder: *mut Decoder);
}
extern "C" {
    pub fn sixel_decoder_unref(decoder: *mut Decoder);
}
extern "C" {
    pub fn sixel_decoder_setopt(decoder: *mut Decoder,
                                arg: c_int,
                                optarg: *const c_char)
                                -> Status;
}
extern "C" {
    pub fn sixel_decoder_decode(decoder: *mut Decoder) -> Status;
}
