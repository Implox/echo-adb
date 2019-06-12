extern crate echo_adb;
extern crate regex;

use std::env;
use std::io;
use std::io::Read;
use std::str;

use echo_adb::*;

fn main() {
    if let Some(input_str) = env::args().nth(1).or_else(|| {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).map(|_| input).ok()
    }) {
        let escaped_input = escape_text(input_str);
        let output = echo_adb(escaped_input);
        let stderr_utf8 = str::from_utf8(&output.stderr).unwrap();

        println!("Command exited with status: {:?}", output.status);
        if stderr_utf8.len() > 0 {
            println!("{}", stderr_utf8);
        }
    }
}
