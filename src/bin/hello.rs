use ferris_says::say;
use std::{println, io::{stdout, BufWriter}};

fn main() {
    let out = "Hello fellow Rustaceans!";
    let printText = "Hello Rustaceans!";
    let width = 24;
    println!("{}", printText);

    let mut writer = BufWriter::new(stdout());
    say(out, width, &mut writer).unwrap();
}