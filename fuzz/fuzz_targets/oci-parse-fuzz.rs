#![no_main]
use libfuzzer_sys::fuzz_target;
use oci_spec::image::ImageManifest;
use std::io::Cursor;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    ImageManifest::from_reader(Cursor::new(data));
});
