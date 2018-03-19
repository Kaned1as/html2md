//#![feature(alloc_system)]

extern crate html5ever;

use html5ever::parse_document;
use html5ever::rcdom::{RcDom, Handle, NodeData};
use html5ever::driver::ParseOpts;
use html5ever::tendril::TendrilSink;

fn parse(html: &str) -> String {
    let opts = ParseOpts::default();
    let dom = parse_document(RcDom::default(), opts).from_utf8().read_from(&mut html.as_bytes()).unwrap();
    walk(dom.document);
    return String::new();
}

fn walk(input: Handle) {
    match input.data {
        NodeData::Document => {},
        NodeData::Doctype { .. } => {},
        NodeData::Text { ref contents }
            => println!("#text: {}", &contents.borrow()),

        NodeData::Comment { ref contents }
            => println!("<!-- {} -->", contents),

        NodeData::Element { ref name, ref attrs, .. } => {
            
            for attr in attrs.borrow().iter() {
                print!(" {}=\"{}\"", attr.name.local, attr.value);
            }
            println!(">");
        }

        NodeData::ProcessingInstruction { .. } => unreachable!()
    }

    for child in input.children.borrow().iter() {
        walk(child.clone());
    }
}

#[cfg(test)]
fn test() {
    parse("<p>aaaaa</p>");
}