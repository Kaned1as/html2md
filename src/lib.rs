//#![feature(alloc_system)]

extern crate html5ever;
extern crate regex;

use std::boxed::Box;
use std::borrow::Borrow;
use std::collections::HashMap;

use regex::Regex;

use html5ever::parse_document;
use html5ever::rcdom::{RcDom, Handle, NodeData};
use html5ever::driver::ParseOpts;
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::TreeBuilderOpts;
use html5ever::tree_builder::QuirksMode;
use html5ever::tokenizer::TokenizerOpts;

mod dummy;
mod anchors;
mod paragraphs;
mod images;
mod headers;
mod lists;
mod styles;
mod codes;

use dummy::DummyHandler;
use paragraphs::ParagraphHandler;
use anchors::AnchorHandler;
use images::ImgHandler;
use headers::HeaderHandler;
use lists::ListItemHandler;
use lists::ListHandler;
use styles::StyleHandler;
use codes::CodeHandler;

pub fn parse(html: &str) -> String {
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
    let indent_matcher = Regex::new("(?m)^\\s+").unwrap();
    let html_without_indents = indent_matcher.replace_all(html, ""); // delete indents

    let newline_tag_matcher = Regex::new(">\n<").unwrap();
    let compact_html = newline_tag_matcher.replace_all(&html_without_indents, "><"); // delete newlines between tags

    let dom = parse_document(RcDom::default(), opts).from_utf8().read_from(&mut compact_html.as_bytes()).unwrap();
    let mut result = StructuredPrinter::default();
    walk(&dom.document, &mut result);
    return result.data;
}

fn walk(input: &Handle, result: &mut StructuredPrinter) {
    let mut handler : Box<TagHandler> = Box::new(DummyHandler::default());
    let mut tag_name = String::default();
    match input.data {
        NodeData::Document | NodeData::Doctype {..} | NodeData::ProcessingInstruction {..} => {},
        NodeData::Text { ref contents }  => {
            result.insert_str(&contents.borrow());
        }
        NodeData::Comment { ref contents } => println!("<!-- {} -->", contents),
        NodeData::Element { ref name, .. } => {
            tag_name = name.local.to_string();
            handler = match tag_name.as_ref() {
                "html" | "head" | "body" => Box::new(DummyHandler::default()),
                "p" | "br" => Box::new(ParagraphHandler::default()),
                "a" => Box::new(AnchorHandler::default()),
                "img" => Box::new(ImgHandler::default()),
                "h1" | "h2" | "h3" | "h4" => Box::new(HeaderHandler::default()),
                "ol" | "ul" | "menu" => Box::new(ListHandler::default()),
                "li" => Box::new(ListItemHandler::default()),
                "b" | "i" | "s" | "strong" | "em" | "del" => Box::new(StyleHandler::default()),
                "pre" | "code" => Box::new(CodeHandler::default()),
                _ => Box::new(DummyHandler::default())
            };

            //println!("element {}", name.local);
        }
    }

    //result.siblings.get_mut(k)

    // handle this tag
    handler.handle(&input.data, result);

    // save this tag name as parent for child nodes
    result.parent_chain.push(tag_name.to_string());     // e.g. it was ["body"] and now it's ["body", "p"]
    let current_depth = result.parent_chain.len();      // e.g. it was 1 and now it's 2

    // create space for siblings of next level
    result.siblings.insert(current_depth, vec![]);

    for child in input.children.borrow().iter() {
        walk(child.borrow(), result);

        match child.data {
            NodeData::Element { ref name, .. } => result.siblings.get_mut(&current_depth).unwrap().push(name.local.to_string()),
            _ => {}
        };

        //println!("{:?}", result);
    }

    // clear siblings of next level
    result.siblings.remove(&current_depth);

    // release parent tag
    result.parent_chain.pop();

    handler.after_handle(result);
}

#[derive(Debug, Default)]
pub struct StructuredPrinter {
    /// Chain of parents leading to upmost <html> tag
    parent_chain: Vec<String>,

    /// Siblings of currently processed tag in order where they're appearing in html
    siblings: HashMap<usize, Vec<String>>,

    /// resulting markdown document
    data: String,

    /// Position in [data] for tracking non-appending cases
    position: usize,

    indent: usize
}

impl StructuredPrinter {
    fn insert_newline(&mut self) {
        self.insert_str("\n");
        let indentation = " ".repeat(self.indent);
        self.insert_str(&indentation);
    }

    fn insert_str(&mut self, it: &str) {
        self.data.insert_str(self.position, it);
        self.position += it.len();
    }
}

trait TagHandler {
    /// Handle tag encountered when walking HTML tree
    fn handle(&mut self, tag: &NodeData, printer: &mut StructuredPrinter);

    /// Executed after all children of this tag have been processed
    fn after_handle(&mut self, printer: &mut StructuredPrinter);

    /// is this tag handler applicable for specified tag
    fn is_applicable(&self, tag_name: String) -> bool;
}