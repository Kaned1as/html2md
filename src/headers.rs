use crate::StructuredParser;

use super::TagHandler;

use markdown::mdast;
use markup5ever_rcdom::{Handle,NodeData};

#[derive(Default)]
pub struct HeaderHandler {
    header_type: String,
}

impl TagHandler for HeaderHandler {

    fn before_handle(&mut self, tag: &Handle, printer: &mut StructuredParser) {
        self.header_type = match tag.data {
            NodeData::Element { ref name, .. } => name.local.to_string(),
            _ => String::new()
        };

        let mut node = mdast::Heading{children: Vec::default(), depth: 1, position: None};
        match self.header_type.as_ref() {
            "h1" => node.depth = 1,
            "h2" => node.depth = 2,
            "h3" => node.depth = 3,
            "h4" => node.depth = 4,
            "h5" => node.depth = 5,
            "h6" => node.depth = 6,
            _ => {}
        }

        printer.add_child(mdast::Node::Heading(node));
    }

    fn after_handle(&mut self, printer: &mut StructuredParser) {
    }
}
