use super::TagHandler;
use super::StructuredPrinter;

use html5ever::rcdom::{Handle,NodeData};

#[derive(Default)]
pub(super) struct ParagraphHandler {
    paragraph_type: String
}

impl TagHandler for ParagraphHandler {

    fn handle(&mut self, tag: &Handle, printer: &mut StructuredPrinter) {
        self.paragraph_type = match tag.data {
            NodeData::Element { ref name, .. } => name.local.to_string(),
            _ => String::new()
        };

        // insert newlines at the start of paragraph
        match self.paragraph_type.as_ref() {
            "p" => { printer.insert_newline(); printer.insert_newline(); }
            _ => {}
        }
    }

    fn after_handle(&mut self, printer: &mut StructuredPrinter) {
        // insert newlines at the end of paragraph
        match self.paragraph_type.as_ref() {
            "p" => { printer.insert_newline(); printer.insert_newline(); }
            "hr" => { printer.insert_newline(); printer.insert_str("---"); printer.insert_newline(); }
            "br" => printer.insert_str("  \n"),
            _ => {}
        }
    }
}