extern crate echo_adb;
extern crate regex;

use std::env;
use std::io;
use std::io::Read;

use echo_adb::*;

fn main() {
    if let Some(input_str) = env::args().nth(1).or_else(|| {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).map(|_| input).ok()
    }) {
        let escaped_input = escape_text(input_str);
        let output = echo_adb(escaped_input);
        println!(
            "Status: {:?}\nStdout: {:?}\nStderr: {:?}",
            output.status, output.stdout, output.stderr
        );
    }
}
