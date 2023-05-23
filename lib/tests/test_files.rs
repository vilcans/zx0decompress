use std::{
    ffi::OsStr,
    fs::{self, File},
    path::{Path, PathBuf},
};

use zx0decompress;

fn compare(expected_decompressed: &Path, compressed_file: &Path) {
    let Ok(expected) = fs::read(expected_decompressed) else {
        panic!("Failed to open source file {}", expected_decompressed.display());
    };
    let mut input = match File::open(compressed_file) {
        Ok(f) => f,
        Err(_) => panic!(
            "Failed to open {} to compare with {}",
            compressed_file.display(),
            expected_decompressed.display()
        ),
    };
    let decompressed = zx0decompress::decompress(&mut input).unwrap();
    assert_eq!(expected, decompressed);
}

#[test]
fn test_fixtures() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/fixtures");
    let compressed_files = fs::read_dir(path).unwrap();
    for entry in compressed_files {
        let Ok(entry) = entry else {
            continue
        };
        let path = entry.path();
        if path.extension().unwrap_or_default() == OsStr::new("zx0") {
            let expected = path.with_extension("");
            println!(
                "Decompressing {} and comparing with {}",
                path.display(),
                expected.display()
            );
            compare(&expected, &path);
        }
    }
}
