extern crate html2md;

use html2md::parse;
use std::fs::File;
use std::io::prelude::*;

#[test]
fn test_marcfs() {
    let mut html = String::new();
    let mut html_file = File::open("test-samples/marcfs-readme.html").unwrap();
    html_file.read_to_string(&mut html).expect("File must be readable");
    let result = parse(&html);
    println!("{}", result);
}