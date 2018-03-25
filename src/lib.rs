//#![feature(alloc_system)]

extern crate html5ever;

use std::boxed::Box;

use html5ever::parse_document;
use html5ever::rcdom::{RcDom, Handle, NodeData};
use html5ever::driver::ParseOpts;
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::TreeBuilderOpts;
use html5ever::tree_builder::QuirksMode;
use html5ever::tokenizer::TokenizerOpts;

mod dummy;
mod anchors;
mod paragraph;
mod images;

use dummy::DummyHandler;
use paragraph::ParagraphHandler;
use anchors::AnchorHandler;
use images::ImgHandler;

pub fn parse(html: &str) -> StructuredPrinter {
    let opts = ParseOpts {
        tree_builder: TreeBuilderOpts {
            exact_errors: false,
            scripting_enabled: false,
            iframe_srcdoc: false,
            drop_doctype: true,
            ignore_missing_rules: true,
            quirks_mode: QuirksMode::NoQuirks
        },
        tokenizer: TokenizerOpts {
            exact_errors: false,
            discard_bom: true,
            profile: false,
            initial_state: None,
            last_start_tag_name: None
        }
    };
    let dom = parse_document(RcDom::default(), opts).from_utf8().read_from(&mut html.as_bytes()).unwrap();
    let mut result = StructuredPrinter{ data: String::new(), position: 0 };
    walk(dom.document, &mut result);
    println!("{:?}", result);
    return result;
}

fn walk(input: Handle, result: &mut StructuredPrinter) {
     let mut handler : Box<TagHandler> = Box::new(DummyHandler {});
    match input.data {
        NodeData::Document | NodeData::Doctype {..} | NodeData::ProcessingInstruction {..} => {},
        NodeData::Text { ref contents }  => result.data.insert_str(result.position, &contents.borrow()),
        NodeData::Comment { ref contents } => println!("<!-- {} -->", contents),
        NodeData::Element { ref name, .. } => {
            handler = match name.local.to_string().as_ref() {
                "html" | "head" | "body" => Box::new(DummyHandler {}),
                "p" => Box::new(ParagraphHandler {}),
                "a" => Box::new(AnchorHandler {}),
                "img" => Box::new(ImgHandler {}),
                _ => Box::new(DummyHandler {})
            };
            println!("element {}", name.local);
            handler.before_handle(result);
            handler.handle(&input.data, result);
        }
    }

    for child in input.children.borrow().iter() {
        walk(child.clone(), result);
    }

    handler.after_handle(result);
}

#[derive(Debug)]
pub struct StructuredPrinter {
    data: String,
    position: usize
}

trait TagHandler: Sync {
    fn before_handle(&mut self, printer: &mut StructuredPrinter);
    fn handle(&mut self, tag: &NodeData, printer: &mut StructuredPrinter);
    fn after_handle(&mut self, printer: &mut StructuredPrinter);

    /// is this tag handler applicable for specified tag
    fn is_applicable(&self, tag_name: String) -> bool;
}