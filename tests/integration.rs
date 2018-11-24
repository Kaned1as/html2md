extern crate html2md;

use html2md::parse_html;
use std::fs::File;
use std::io::prelude::*;

#[test]
fn test_marcfs() {
    let mut html = String::new();
    let mut html_file = File::open("test-samples/marcfs-readme.html").unwrap();
    html_file.read_to_string(&mut html).expect("File must be readable");
    let result = parse_html(&html);
    println!("{}", result);
}

#[test]
fn test_cheatsheet() {
    let mut html = String::new();
    let mut md = String::new();
    let mut html_file = File::open("test-samples/markdown-cheatsheet.html").unwrap();
    let mut md_file = File::open("test-samples/markdown-cheatsheet.md").unwrap();
    html_file.read_to_string(&mut html).expect("File must be readable");
    md_file.read_to_string(&mut md).expect("File must be readable");
    let md_parsed = parse_html(&html);
    println!("{}", md_parsed);
    //assert_eq!(md, md_parsed);
}

#[test]
fn test_dybr() {
    let mut html = String::new();
    let mut html_file = File::open("test-samples/dybr-test2.html").unwrap();
    html_file.read_to_string(&mut html).expect("File must be readable");
    let result = parse_html(&html);
    println!("{}", result);
}
