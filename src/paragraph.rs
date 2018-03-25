use super::TagHandler;
use super::StructuredPrinter;

use html5ever::rcdom::NodeData;

pub struct ParagraphHandler {

}

impl TagHandler for ParagraphHandler {

    fn before_handle(&mut self, _printer: &mut StructuredPrinter) {
        
    }

    fn handle(&mut self, _tag: &NodeData, printer: &mut StructuredPrinter) {
        printer.data.insert_str(printer.position, "\n\n");
        printer.position += 2; // increase by two line endings that we inserted in handle
    }

    fn after_handle(&mut self, _printer: &mut StructuredPrinter) {
        
    }

    fn is_applicable(&self, tag_name: String) -> bool {
        return tag_name == "p";
    }
}