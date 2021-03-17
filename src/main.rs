use std::io::Write;
use std::str::from_utf8;

use termcolor::{ColorChoice, StandardStream};

fn main() {
    let test_str = include_str!("test.txt").to_owned();
    from_utf8(&test_str.as_bytes()).unwrap(); // unnecessarily confirm that this is indeed UTF-8
    let mut stdout = StandardStream::stdout(ColorChoice::Never);

    // following line panics with "Windows stdio in console mode does not support writing non-UTF-8 byte sequences"
    write!(&mut stdout, "{}", test_str).unwrap();
}
