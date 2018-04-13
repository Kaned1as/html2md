use super::TagHandler;
use super::StructuredPrinter;

use html5ever::rcdom::{Handle,NodeData};

#[derive(Default)]
pub(super) struct DummyHandler;

impl TagHandler for DummyHandler {

    fn handle(&mut self, _tag: &Handle, _printer: &mut StructuredPrinter) {
       
    }

    fn after_handle(&mut self, _printer: &mut StructuredPrinter) {
        
    }
}

#[derive(Default)]
pub(super) struct IdentityHandler {
    tag_name: String
}

impl TagHandler for IdentityHandler {

    fn handle(&mut self, tag: &Handle, printer: &mut StructuredPrinter) {
        self.tag_name = match tag.data {
            NodeData::Element { ref name, .. } => name.local.to_string(),
            _ => String::new()
        };

       // possibly we can add attr-handling here too,
       // any use-cases?

       printer.insert_str(&format!("<{}>", self.tag_name));
    }

    fn after_handle(&mut self, printer: &mut StructuredPrinter) {
        printer.insert_str(&format!("</{}>", self.tag_name));
    }
}