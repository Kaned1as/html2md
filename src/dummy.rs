use super::TagHandler;
use super::StructuredPrinter;

use html5ever::rcdom::NodeData;

#[derive(Default)]
pub(super) struct DummyHandler;

impl TagHandler for DummyHandler {

    fn handle(&mut self, _tag: &NodeData, _printer: &mut StructuredPrinter) {
       
    }

    fn after_handle(&mut self, _printer: &mut StructuredPrinter) {
        
    }

    fn is_applicable(&self, _tag_name: String) -> bool {
        return false;
    }
}