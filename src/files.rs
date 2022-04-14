use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut File = File::open("test.txt")
        .expect("Cant open file :( F");

    let mut contents = String::new();
    File.read(&mut contents)
        .expect("Cant read file :( F");


    println!("File: {}",  contents);
}