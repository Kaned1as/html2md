use super::TagHandler;
use super::StructuredPrinter;

use html5ever::rcdom::NodeData;

pub struct DummyHandler {

}

impl TagHandler for DummyHandler {

    fn before_handle(&mut self, printer: &mut StructuredPrinter) {
        
    }

    fn handle(&mut self, tag: &NodeData, printer: &mut StructuredPrinter) {
       
    }

    fn after_handle(&mut self, printer: &mut StructuredPrinter) {
        
    }

    fn is_applicable(&self, tag_name: String) -> bool {
        return false;
    }
}