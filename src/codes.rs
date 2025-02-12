use crate::common::get_tag_attr;

use super::TagHandler;
use super::StructuredParser;

use markdown::mdast;
use markup5ever_rcdom::{Handle,NodeData};

#[derive(Default)]
pub struct CodeHandler;

impl TagHandler for CodeHandler {

    fn before_handle(&mut self, tag: &Handle, printer: &mut StructuredParser) {
        let code_type = match tag.data {
            NodeData::Element { ref name, .. } => name.local.to_string(),
            _ => String::new()
        };

        match code_type.as_ref() {
            "pre" => {
                let lang = get_tag_attr(tag, "lang");
                let node = mdast::Code{value: String::new(), lang, meta: None, position: None};
                printer.add_child(mdast::Node::Code(node));
            },
            "code" | "samp" => {
                let node = mdast::InlineCode{value: String::new(), position: None};
                printer.add_child(mdast::Node::InlineCode(node));
            }
            _ => {}
        }
    }
    
    fn after_handle(&mut self, printer: &mut StructuredParser) {
    }
}
