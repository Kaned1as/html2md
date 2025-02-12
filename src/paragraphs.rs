use super::TagHandler;
use super::StructuredParser;

use markdown::mdast;
use markup5ever_rcdom::{Handle,NodeData};

#[derive(Default)]
pub struct ParagraphHandler;

impl TagHandler for ParagraphHandler {

    fn before_handle(&mut self, tag: &Handle, printer: &mut StructuredParser) {
        let paragraph_type = match tag.data {
            NodeData::Element { ref name, .. } => name.local.to_string(),
            _ => String::new()
        };
        match paragraph_type.as_ref() {
            "p" => {
                let node = mdast::Paragraph{children: Vec::new(), position: None};
                printer.add_child(mdast::Node::Paragraph(node));
            }
            "hr" => {
                let node = mdast::ThematicBreak{position: None};
                printer.add_child(mdast::Node::ThematicBreak(node));
            }
            "br" => {
                let node = mdast::Break{position: None};
                printer.add_child(mdast::Node::Break(node));
            }
            _ => {}
        }
    }

    fn after_handle(&mut self, printer: &mut StructuredParser) {
    }
}
