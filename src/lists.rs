use super::TagHandler;
use super::StructuredParser;

use markdown::mdast;
use markup5ever_rcdom::Handle;
use markup5ever_rcdom::NodeData;

#[derive(Default)]
pub struct ListHandler;

impl TagHandler for ListHandler {

    /// we're entering "ul" or "ol" tag, no "li" handling here
    fn before_handle(&mut self, tag: &Handle, printer: &mut StructuredParser) {
        let list_type = match tag.data {
            NodeData::Element { ref name, .. } => name.local.to_string(),
            _ => String::new()
        };

        // insert an extra newline for non-nested lists
        let mut node = mdast::List{children: Vec::default(), position: None, spread: false, ordered: false, start: None};
        match list_type.as_ref() {
            "ul" | "menu" => node.ordered = false,
            "ol" => node.ordered = false,
            _ => {}
        }
        printer.add_child(mdast::Node::List(node));
    }

    /// indent now-ready list
    fn after_handle(&mut self, _printer: &mut StructuredParser) {
    }
}

#[derive(Default)]
pub struct ListItemHandler;

impl TagHandler for ListItemHandler {

    fn before_handle(&mut self, _tag: &Handle, printer: &mut StructuredParser) {
        let node = mdast::ListItem{children: Vec::default(), position: None, spread: false, checked: None};
        printer.add_child(mdast::Node::ListItem(node));
    }

    fn after_handle(&mut self, printer: &mut StructuredParser) {
    }
}
