use crate::{common::get_tag_attr, StructuredParser};
use crate::dummy::HtmlHandler;

use super::TagHandler;

use markdown::mdast;
use markup5ever_rcdom::{Handle,NodeData};

#[derive(Default)]
pub struct AnchorHandler;

impl TagHandler for AnchorHandler {
    fn before_handle(&mut self, tag: &Handle, printer: &mut StructuredParser) {
        // Check for a `name` attribute. If it exists, we can't support this
        // in markdown, so we must emit this tag unchanged.
        if get_tag_attr(tag, "name").is_some() {
            let mut identity = HtmlHandler::default();
            identity.before_handle(tag, printer);
            return;
        }
        
        // it's a normal link
        let href = get_tag_attr(tag, "href");
        let node = mdast::Link{children: Vec::new(), url: href.unwrap(), title: None, position: None};
        printer.add_child(mdast::Node::Link(node));
    }

    fn after_handle(&mut self, printer: &mut StructuredParser) {
    }
}

