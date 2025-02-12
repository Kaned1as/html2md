use lazy_static::lazy_static;

use std::boxed::Box;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use std::os::raw::c_char;
use std::ffi::{CString, CStr};

use regex::Regex;

use html5ever::parse_document;
use html5ever::driver::ParseOpts;
use html5ever::tendril::TendrilSink;

use markdown::mdast::{self, Root};

pub use markup5ever_rcdom::{RcDom, Handle, NodeData};

pub mod common;
pub mod dummy;
pub mod anchors;
pub mod paragraphs;
pub mod images;
pub mod headers;
pub mod lists;
pub mod styles;
pub mod codes;
pub mod quotes;
pub mod tables;
pub mod containers;
pub mod iframes;

use crate::dummy::DummyHandler;
use crate::dummy::HtmlHandler;
use crate::paragraphs::ParagraphHandler;
use crate::anchors::AnchorHandler;
use crate::images::ImgHandler;
use crate::headers::HeaderHandler;
use crate::lists::ListItemHandler;
use crate::lists::ListHandler;
use crate::styles::StyleHandler;
use crate::codes::CodeHandler;
use crate::quotes::QuoteHandler;
use crate::tables::TableHandler;
use crate::containers::ContainerHandler;
use crate::iframes::IframeHandler;

/// Custom variant of main function. Allows to pass custom tag<->tag factory pairs
/// in order to register custom tag hadler for tags you want.
///
/// You can also override standard tag handlers this way
/// # Arguments
/// `html` is source HTML as `String`
/// `custom` is custom tag hadler producers for tags you want, can be empty
pub fn parse_html_custom(html: &str, custom: &HashMap<String, Box<dyn TagHandlerFactory>>) -> String {
    let dom = parse_document(RcDom::default(), ParseOpts::default()).from_utf8().read_from(&mut html.as_bytes()).unwrap();
    let mut result = StructuredParser::new();
    walk(&dom.document, &mut result, custom);

    println!("Result: {:?}", &result.data.borrow());
    return mdast_util_to_markdown::to_markdown(&result.data.borrow()).unwrap();
}

/// Main function of this library. Parses incoming HTML, converts it into Markdown
/// and returns converted string.
/// # Arguments
/// `html` is source HTML as `String`
pub fn parse_html(html: &str) -> String {
    parse_html_custom(html, &HashMap::default())
}

/// Same as `parse_html` but retains all "span" html elements intact
/// Markdown parsers usually strip them down when rendering but they
/// may be useful for later processing
pub fn parse_html_extended(html: &str) -> String {
    struct SpanAsIsTagFactory;
    impl TagHandlerFactory for SpanAsIsTagFactory {
        fn instantiate(&self) -> Box<dyn TagHandler> {
            return Box::new(HtmlHandler::default());
        }
    }

    let mut tag_factory: HashMap<String, Box<dyn TagHandlerFactory>> = HashMap::new();
    tag_factory.insert(String::from("span"), Box::new(SpanAsIsTagFactory{}));
    return parse_html_custom(html, &tag_factory);
}

/// Recursively walk through all DOM tree and handle all elements according to
/// HTML tag -> Markdown syntax mapping. Text content is trimmed to one whitespace according to HTML5 rules.
///
/// # Arguments
/// `input` is DOM tree or its subtree
/// `result` is output holder with position and context tracking
/// `custom` is custom tag hadler producers for tags you want, can be empty
fn walk(input: &Handle, result: &mut StructuredParser, custom: &HashMap<String, Box<dyn TagHandlerFactory>>) {
    let mut handler : Box<dyn TagHandler> = Box::new(DummyHandler::default());
    let mut tag_name = String::default();
    match input.data {
        // ignore ancillary nodes
        NodeData::Document | NodeData::Doctype {..} | NodeData::ProcessingInstruction {..} => {},
        NodeData::Text { ref contents }  => {
            let mut text = contents.borrow().to_string();
            let node = mdast::Text{value: text, position: None};
            result.add_text(node);
            return;
        }
        // ignore comments
        NodeData::Comment { .. } => {}, 
        NodeData::Element { ref name, .. } => {
            tag_name = name.local.to_string();
            if custom.contains_key(&tag_name) {
                // have user-supplied factory, instantiate a handler for this tag
                let factory = custom.get(&tag_name).unwrap();
                handler = factory.instantiate();
            } else {
                // no user-supplied factory, take one of built-in ones
                handler = match tag_name.as_ref() {
                    // containers
                    "div" | "section" | "header" | "footer" => Box::new(ContainerHandler::default()),
                    // pagination, breaks
                    "p" | "br" | "hr" => Box::new(ParagraphHandler::default()),
                    "q" | "cite" | "blockquote" => Box::new(QuoteHandler::default()),
                    // spoiler tag
                    "details" | "summary" => Box::new(DummyHandler::default()),
                    // formatting
                    "b" | "i" | "s" | "strong" | "em" | "del" => Box::new(StyleHandler::default()),
                    "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => Box::new(HeaderHandler::default()),
                    "pre" | "code" => Box::new(CodeHandler::default()),
                    // images, links
                    "img" => Box::new(ImgHandler::default()),
                    "a" => Box::new(AnchorHandler::default()),
                    // lists
                    "ol" | "ul" | "menu" => Box::new(ListHandler::default()),
                    "li" => Box::new(ListItemHandler::default()),
                    // as-is
                    "sub" | "sup" => Box::new(HtmlHandler::default()),
                    // tables, handled fully internally as markdown can't have nested content in tables
                    // supports only single tables as of now
                    "table" => Box::new(TableHandler::default()),
                    "iframe" => Box::new(IframeHandler::default()),
                    // other
                    "html" | "head" | "body" => Box::new(DummyHandler::default()),
                    _ => Box::new(DummyHandler::default())
                };
            }
        }
    }

    // handle this tag, while it's not in parent chain
    // and doesn't have child siblings
    handler.before_handle(&input, result);

    for child in input.children.borrow().iter() {
        if handler.skip_descendants() {
            continue;
        }

        walk(child, result, custom);
    }

    handler.after_handle(result);
    result.pop_child();
}

/// Intermediate result of HTML -> Markdown conversion.
///
/// Holds context in the form of parent tags and siblings chain
/// and resulting string of markup content with current position.
#[derive(Debug)]
pub struct StructuredParser {
    /// Chain of parents leading to upmost <html> tag
    pub in_progress: Vec<Rc<RefCell<mdast::Node>>>,

    /// resulting markdown document
    pub data: Rc<RefCell<mdast::Node>>,
}

impl StructuredParser {
    pub fn new() -> Self {
        let root = Rc::new(RefCell::new(mdast::Node::Root(mdast::Root {children: vec![], position: None})));
        let parser = StructuredParser {
            data: root.clone(),
            in_progress: vec![
                // start with root node
                root.clone()
            ],
        };
        return parser;
    }

    fn add_child(&mut self, node: mdast::Node) {
        println!("add child! {:?}", node);
        self.in_progress.push(Rc::new(RefCell::new(node)));
    }
    
    fn pop_child(&mut self) {
        println!("pop child!");
        if self.in_progress.len() < 2 {
            return;
        }

        let child = self.in_progress.pop().unwrap();
        let parent = self.in_progress.last().unwrap();

        let child_raw = Rc::into_inner(child).unwrap().into_inner();
        parent.borrow_mut().children_mut().map(|c| c.push(child_raw));
    }

    fn add_text(&mut self, text: mdast::Text) {
        println!("add text!");
        // at this point there are no other references to this node
        let parent = self.in_progress.last().unwrap();

        let mut real_node = parent.borrow_mut();
        match *real_node {
           mdast::Node::Code(ref mut obj) => { obj.value += &text.value; },
           mdast::Node::InlineCode(ref mut obj) => { obj.value += &text.value; },
           mdast::Node::Math(ref mut obj) => { obj.value += &text.value; },
           mdast::Node::InlineMath(ref mut obj) => { obj.value += &text.value; },
           mdast::Node::Html(ref mut obj) => { obj.value += &text.value; },
           mdast::Node::Yaml(ref mut obj) => { obj.value += &text.value; },
           mdast::Node::Toml(ref mut obj) => { obj.value += &text.value; },
           mdast::Node::List(..) => {},
           _ => { real_node.children_mut().map(|c| c.push(mdast::Node::Text(text))); }
        }
    }
}

/// Tag handler factory. This class is required in providing proper
/// custom tag parsing capabilities to users of this library.
///
/// The problem with directly providing tag handlers is that they're not stateless.
/// Once tag handler is parsing some tag, it holds data, such as start position, indent etc.
/// The only way to create fresh tag handler for each tag is to provide a factory like this one.
///
pub trait TagHandlerFactory {
    fn instantiate(&self) -> Box<dyn TagHandler>;
}

/// Trait interface describing abstract handler of arbitrary HTML tag.
pub trait TagHandler {
    /// Handle tag encountered when walking HTML tree.
    /// This is executed before the children processing
    fn before_handle(&mut self, tag: &Handle, printer: &mut StructuredParser);

    /// Executed after all children of this tag have been processed
    fn after_handle(&mut self, printer: &mut StructuredParser);

    fn skip_descendants(&self) -> bool {
        return false;
    }
}

/// FFI variant for HTML -> Markdown conversion for calling from other languages
#[no_mangle]
pub extern fn parse(html: *const c_char) -> *const c_char {
    let in_html = unsafe { CStr::from_ptr(html) };
    let out_md = parse_html(&in_html.to_string_lossy());

    CString::new(out_md).unwrap().into_raw()
}

/// Expose the JNI interface for android below
#[cfg(target_os="android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use super::parse_html;
    use super::parse_html_extended;

    use self::jni::JNIEnv;
    use self::jni::objects::{JClass, JString};
    use self::jni::sys::jstring;

    #[no_mangle]
    pub unsafe extern fn Java_com_kanedias_html2md_Html2Markdown_parse(env: JNIEnv, _clazz: JClass, html: JString) -> jstring {
        let html_java : String = env.get_string(html).expect("Couldn't get java string!").into();
        let markdown = parse_html(&html_java);
        let output = env.new_string(markdown).expect("Couldn't create java string!");
        output.into_inner()
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_kanedias_html2md_Html2Markdown_parseExtended(env: JNIEnv, _clazz: JClass, html: JString) -> jstring {
        let html_java : String = env.get_string(html).expect("Couldn't get java string!").into();
        let markdown = parse_html_extended(&html_java);
        let output = env.new_string(markdown).expect("Couldn't create java string!");
        output.into_inner()
    }
}
