use super::TagHandler;
use super::StructuredPrinter;

use html5ever::rcdom::NodeData;

#[derive(Default)]
pub struct QuoteHandler {
    start_pos: usize
}

impl TagHandler for QuoteHandler {
    
    fn handle(&mut self, _tag: &NodeData, printer: &mut StructuredPrinter) {
        self.start_pos = printer.position;
    }

    fn after_handle(&mut self, printer: &mut StructuredPrinter) {
        // replace all newlines with newline + > 
    }

    fn is_applicable(&self, tag_name: String) -> bool {
        return tag_name == "q" || tag_name == "blockquote";
    }
}