use super::TagHandler;
use super::StructuredPrinter;

use html5ever::rcdom::Handle;
use html5ever::serialize;
use html5ever::serialize::{SerializeOpts, TraversalScope};

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
}

impl TagHandler for IdentityHandler {

    fn handle(&mut self, tag: &Handle, printer: &mut StructuredPrinter) {
        let mut buffer = vec![];

        let options = SerializeOpts { traversal_scope: TraversalScope::IncludeNode, .. Default::default() };
        let result = serialize(&mut buffer, tag, options);
        if result.is_err() {
            // couldn't serialize the tag
            return;
        }

        let conv = String::from_utf8(buffer);
        if conv.is_err() {
            // is non-utf8 string possible in html5ever? 
            return;
        }

        printer.append_str(&conv.unwrap());
    }

    fn skip_descendants(&self) -> bool {
        return true;
    }

    fn after_handle(&mut self, _printer: &mut StructuredPrinter) {

    }
}