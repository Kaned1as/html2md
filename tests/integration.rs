extern crate html2md;
extern crate spectral;

use html2md::parse_html;
use std::fs::File;
use std::io::prelude::*;

use spectral::prelude::*;

#[test]
#[ignore]
fn test_marcfs() {
    let mut html = String::new();
    let mut html_file = File::open("test-samples/marcfs-readme.html").unwrap();
    html_file.read_to_string(&mut html).expect("File must be readable");
    let result = parse_html(&html);
    println!("{}", result);
}

#[test]
#[ignore]
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

/// newlines after list shouldn't be converted into text of the last list element
#[test]
fn test_list_newlines() {
    

    let mut html = String::new();
    let mut html_file = File::open("test-samples/dybr-bug-with-list-newlines.html").unwrap();
    html_file.read_to_string(&mut html).expect("File must be readable");
    let result = parse_html(&html);
    assert_that(&result).contains(".\n\nxxx xxxx");
    assert_that(&result).contains("xx x.\n\nxxxxx:");
}


#[test]
fn test_lists_from_text() {
    let mut html = String::new();
    let mut html_file = File::open("test-samples/dybr-bug-with-lists-from-text.html").unwrap();
    html_file.read_to_string(&mut html).expect("File must be readable");
    let result = parse_html(&html);
    println!("{}", result);
}

#[test]
fn test_strong_inside_link() {
    

    let mut html = String::new();
    let mut html_file = File::open("test-samples/dybr-bug-with-strong-inside-link.html").unwrap();
    html_file.read_to_string(&mut html).expect("File must be readable");
    let result = parse_html(&html);
    assert_that(&result).contains("[**Just God**](http://fanfics.me/ficXXXXXXX)");
}