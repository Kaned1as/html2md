use super::TagHandler;
use super::StructuredPrinter;

use html5ever::rcdom::NodeData;

pub struct ParagraphHandler {

}

impl TagHandler for ParagraphHandler {

    fn before_handle(&mut self, printer: &mut StructuredPrinter) {
        
    }

    fn handle(&mut self, tag: &NodeData, printer: &mut StructuredPrinter) {
        printer.data.insert_str(printer.position, "\n\n");
    }

    fn after_handle(&mut self, printer: &mut StructuredPrinter) {
        printer.position = printer.data.len();
    }

    fn is_applicable(&self, tag_name: String) -> bool {
        return tag_name == "p";
    }
}