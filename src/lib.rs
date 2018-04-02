//#![feature(alloc_system)]

#[macro_use]
extern crate lazy_static;
extern crate html5ever;
extern crate regex;

use std::boxed::Box;
use std::borrow::Borrow;
use std::collections::HashMap;

use std::os::raw::{c_char};
use std::ffi::{CString, CStr};

use regex::Regex;

use html5ever::parse_document;
use html5ever::rcdom::{RcDom, Handle, NodeData};
use html5ever::driver::ParseOpts;
use html5ever::tendril::TendrilSink;

mod dummy;
mod anchors;
mod paragraphs;
mod images;
mod headers;
mod lists;
mod styles;
mod codes;
mod quotes;

use dummy::DummyHandler;
use dummy::IdentityHandler;
use paragraphs::ParagraphHandler;
use anchors::AnchorHandler;
use images::ImgHandler;
use headers::HeaderHandler;
use lists::ListItemHandler;
use lists::ListHandler;
use styles::StyleHandler;
use codes::CodeHandler;
use quotes::QuoteHandler;

lazy_static! {
    static ref EXCESSIVE_WHITESPACE_PATTERN : Regex = Regex::new("\\s{2,}").unwrap();   // for HTML on-the-fly cleanup
    static ref EXCESSIVE_NEWLINE_PATTERN : Regex = Regex::new("\\n{2,}").unwrap();      // for Markdown post-processing
    static ref TRAILING_SPACE_PATTERN : Regex = Regex::new("(?m) +$").unwrap();         // for Markdown post-processing
}

/// FFI variant for HTML -> Markdown conversion for calling from other languages
#[no_mangle]
pub extern fn parse(html: *const c_char) -> *const c_char {
    let in_html = unsafe { CStr::from_ptr(html) };
    let out_md = parse_html(&in_html.to_string_lossy());

    CString::new(out_md).unwrap().into_raw()
}

/// Main function of this library. Parses incoming HTML, converts it into Markdown 
/// and returns converted string.
pub fn parse_html(html: &str) -> String {
    let dom = parse_document(RcDom::default(), ParseOpts::default()).from_utf8().read_from(&mut html.as_bytes()).unwrap();
    let mut result = StructuredPrinter::default();
    walk(&dom.document, &mut result);

    // remove redundant newlines
    let intermediate1 = EXCESSIVE_NEWLINE_PATTERN.replace_all(&result.data, "\n\n");
    let intermediate2 = TRAILING_SPACE_PATTERN.replace_all(&intermediate1, "");

    intermediate2.into_owned()
}

/// Recursively walk through all DOM tree and handle all elements according to 
/// HTML tag -> Markdown syntax mapping. Text content is trimmed to one whitespace according to HTML5 rules.
/// 
/// # Arguments
/// `input` is DOM tree or its subtree
/// `result` is output holder with position and context tracking
fn walk(input: &Handle, result: &mut StructuredPrinter) {
    let mut handler : Box<TagHandler> = Box::new(DummyHandler::default());
    let mut tag_name = String::default();
    match input.data {
        NodeData::Document | NodeData::Doctype {..} | NodeData::ProcessingInstruction {..} => {},
        NodeData::Text { ref contents }  => {
            let text = contents.borrow().to_string();
            let inside_pre = result.parent_chain.iter().any(|tag| tag == "pre");
            if inside_pre {
                // this is preformatted text, insert as-is
                result.insert_str(&text);
            } else if !(text.trim().len() == 0 && result.data.chars().last() == Some('\n')) {
                // in case it's not just a whitespace after the newline
                // regular text, collapse whitespace
                let minified_text = EXCESSIVE_WHITESPACE_PATTERN.replace_all(&text, " ");
                result.insert_str(&minified_text);
            }
        }
        NodeData::Comment { ref contents } => println!("<!-- {} -->", contents),
        NodeData::Element { ref name, .. } => {
            tag_name = name.local.to_string();
            handler = match tag_name.as_ref() {
                // pagination, breaks
                "p" | "br" | "hr" => Box::new(ParagraphHandler::default()),
                "q" | "cite" | "blockquote" => Box::new(QuoteHandler::default()),
                // formatting
                "b" | "i" | "s" | "strong" | "em" | "del" => Box::new(StyleHandler::default()),
                "h1" | "h2" | "h3" | "h4" => Box::new(HeaderHandler::default()),
                "pre" | "code" => Box::new(CodeHandler::default()),
                // images, links
                "img" => Box::new(ImgHandler::default()),
                "a" => Box::new(AnchorHandler::default()),
                // lists
                "ol" | "ul" | "menu" => Box::new(ListHandler::default()),
                "li" => Box::new(ListItemHandler::default()),
                // as-is
                "sub" | "sup" => Box::new(IdentityHandler::default()),
                // other
                "html" | "head" | "body" => Box::new(DummyHandler::default()),
                _ => Box::new(DummyHandler::default())
            };
        }
    }

    // handle this tag, while it's not in parent chain
    // and doesn't have child siblings
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
    }

    // clear siblings of next level
    result.siblings.remove(&current_depth);

    // release parent tag
    result.parent_chain.pop();

    // finish handling of tag - parent chain now doesn't contain this tag itself again
    handler.after_handle(result);
}

/// Intermediate result of HTML -> Markdown conversion.
/// 
/// Holds context in the form of parent tags and siblings chain
/// and resulting string of markup content with current position.
#[derive(Debug, Default)]
pub struct StructuredPrinter {
    /// Chain of parents leading to upmost <html> tag
    parent_chain: Vec<String>,

    /// Siblings of currently processed tag in order where they're appearing in html
    siblings: HashMap<usize, Vec<String>>,

    /// resulting markdown document
    data: String,

    /// Position in [data] for tracking non-appending cases
    position: usize
}

impl StructuredPrinter {

    /// Inserts newline
    fn insert_newline(&mut self) {
        self.insert_str("\n");
    }

    /// Insert string at current position of printer, adjust position to the end of inserted string
    fn insert_str(&mut self, it: &str) {
        self.data.insert_str(self.position, it);
        self.position += it.len();
    }
}

/// Trait interface describing abstract handler of arbitrary HTML tag.
pub trait TagHandler {
    /// Handle tag encountered when walking HTML tree
    fn handle(&mut self, tag: &NodeData, printer: &mut StructuredPrinter);

    /// Executed after all children of this tag have been processed
    fn after_handle(&mut self, printer: &mut StructuredPrinter);

    /// is this tag handler applicable for specified tag
    fn is_applicable(&self, tag_name: String) -> bool;
}

/// Expose the JNI interface for android below
#[cfg(target_os="android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use super::parse_html;

    use self::jni::JNIEnv;
    use self::jni::objects::{JClass, JString};
    use self::jni::sys::{jstring};

    #[no_mangle]
    pub unsafe extern fn Java_com_kanedias_html2md_Html2Markdown_parse(env: JNIEnv, _clazz: JClass, html: JString) -> jstring {
        let html_java : String = env.get_string(html).expect("Couldn't get java string!").into();
        let markdown = parse_html(&html_java);
        let output = env.new_string(markdown).expect("Couldn't create java string!");
        output.into_inner()
    }
}
