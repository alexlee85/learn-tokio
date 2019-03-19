use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let content = line.unwrap();
        println!("{}", content);

        if content == "end" {
            break;
        }
    }
}
