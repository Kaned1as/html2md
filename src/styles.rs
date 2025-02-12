use crate::StructuredParser;

use super::TagHandler;

use markdown::mdast;
use markup5ever_rcdom::{Handle,NodeData};

#[derive(Default)]
pub struct StyleHandler;

impl TagHandler for StyleHandler {
    
    fn before_handle(&mut self, tag: &Handle, printer: &mut StructuredParser) {
        let style_type = match tag.data {
            NodeData::Element { ref name, .. } => name.local.to_string(),
            _ => String::new()
        };

        match style_type.as_ref() {
            "b" | "strong" => {
                let node = mdast::Strong{children: Vec::new(), position: None};
                printer.add_child(mdast::Node::Strong(node));
            }
            "i" | "em" => {
                let node = mdast::Emphasis{children: Vec::new(), position: None};
                printer.add_child(mdast::Node::Emphasis(node));
            }
            "s" | "del" => {
                let node = mdast::Delete{children: Vec::new(), position: None};
                printer.add_child(mdast::Node::Delete(node));
            }
            "u" | "ins" => {},
            _ => {}
        }
    }

    fn after_handle(&mut self, printer: &mut StructuredParser) {
    }
}
