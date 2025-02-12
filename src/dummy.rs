use crate::StructuredParser;

use super::TagHandler;

use html5ever::serialize;
use html5ever::serialize::{SerializeOpts, TraversalScope};
use markdown::mdast;
use markup5ever_rcdom::{Handle, NodeData, SerializableHandle};

#[derive(Default)]
pub struct DummyHandler;

impl TagHandler for DummyHandler {

    fn before_handle(&mut self, _tag: &Handle, _printer: &mut StructuredParser) {

    }

    fn after_handle(&mut self, _printer: &mut StructuredParser) {

    }
}

/// Handler that completely copies tag to printer as HTML with all descendants
#[derive(Default)]
pub(super) struct HtmlHandler;

impl TagHandler for HtmlHandler {

    fn before_handle(&mut self, tag: &Handle, printer: &mut StructuredParser) {
        let mut buffer = vec![];

        let options = SerializeOpts { traversal_scope: TraversalScope::IncludeNode, .. Default::default() };
        let to_be_serialized = SerializableHandle::from(tag.clone());
        let result = serialize(&mut buffer, &to_be_serialized, options);
        if result.is_err() {
            // couldn't serialize the tag
            return;
        }

        let conv = String::from_utf8(buffer);
        if conv.is_err() {
            // is non-utf8 string possible in html5ever?
            return;
        }

        let node = mdast::Html{value: conv.unwrap(), position: None};
        printer.add_child(mdast::Node::Html(node));
    }

    fn skip_descendants(&self) -> bool {
        return true;
    }

    fn after_handle(&mut self, _printer: &mut StructuredParser) {

    }
}
