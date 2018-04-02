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
        return true; // dummy handler can process anything, but it should be last resort
    }
}

#[derive(Default)]
pub(super) struct IdentityHandler {
    tag_name: String
}

impl TagHandler for IdentityHandler {

    fn handle(&mut self, tag: &NodeData, printer: &mut StructuredPrinter) {
        self.tag_name = match tag {
            &NodeData::Element { ref name, .. } => name.local.to_string(),
            _ => String::new()
        };

       // possibly we can add attr-handling here too,
       // any use-cases?

       printer.insert_str(&format!("<{}>", self.tag_name));
    }

    fn after_handle(&mut self, printer: &mut StructuredPrinter) {
        printer.insert_str(&format!("</{}>", self.tag_name));
    }

    fn is_applicable(&self, tag_name: String) -> bool {
        return tag_name == "sub" || tag_name == "sup";
    }
}