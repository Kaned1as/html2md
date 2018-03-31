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
        printer.insert_newline();
    }

    fn after_handle(&mut self, printer: &mut StructuredPrinter) {
        // replace all newlines with newline + > 
        let quote = "> ";
        let mut index = printer.data.len();
        while index >= self.start_pos {
            if printer.data.as_bytes().iter().nth(index) == Some(&b'\n') {
                printer.data.insert_str(index + 1, &quote);
                printer.position += quote.len();
            }
            index -= 1;
        }

        printer.insert_newline();
    }

    fn is_applicable(&self, tag_name: String) -> bool {
        return tag_name == "q" || tag_name == "cite" || tag_name == "blockquote";
    }
}