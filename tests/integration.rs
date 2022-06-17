#![forbid(unsafe_code, clippy::pedantic)]
#![deny(warnings)]

use std::fs::{remove_file, File};
use std::io::Write;
use std::path::Path;

use iterators_task::iterators_main;

#[test]
fn main_test() {
    type N = i32;

    const A: N = 1;

    const FILENAME: &str = "data.txt";

    const CONTENT: &[u8] = b"1\n3\n 4\n dg\n -5\n2";

    if Path::new(FILENAME).exists() {
        panic!(
            "\"{}\" already exists! Please remove it manually or move \
            it away because this test needs to overwrite it.",
            FILENAME
        );
    }

    File::create(FILENAME).unwrap().write_all(CONTENT).unwrap();

    assert_eq!(iterators_main(A, FILENAME).unwrap(), (2, 3));

    remove_file(FILENAME).unwrap();
}
