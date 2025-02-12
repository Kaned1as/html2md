use super::TagHandler;
use super::StructuredParser;

use markdown::mdast;
use markup5ever_rcdom::Handle;

#[derive(Default)]
pub struct QuoteHandler {
    start_pos: usize
}

impl TagHandler for QuoteHandler {

    fn before_handle(&mut self, _tag: &Handle, printer: &mut StructuredParser) {
        let node = mdast::Blockquote{children: Vec::new(), position: None};
        printer.add_child(mdast::Node::Blockquote(node));
    }

    fn after_handle(&mut self, printer: &mut StructuredParser) {
    }
}
