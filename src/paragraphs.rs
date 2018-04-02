use super::TagHandler;
use super::StructuredPrinter;

use html5ever::rcdom::NodeData;

#[derive(Default)]
pub(super) struct ParagraphHandler {
    paragraph_type: String
}

impl TagHandler for ParagraphHandler {

    fn handle(&mut self, tag: &NodeData, _printer: &mut StructuredPrinter) {
        self.paragraph_type = match tag {
            &NodeData::Element { ref name, .. } => name.local.to_string(),
            _ => String::new()
        };
    }

    fn after_handle(&mut self, printer: &mut StructuredPrinter) {
        match self.paragraph_type.as_ref() {
            "p" => { printer.insert_newline(); printer.insert_newline(); }
            "hr" => { printer.insert_newline(); printer.insert_str("---"); printer.insert_newline(); }
            "br" => printer.insert_newline(),
            _ => {}
        }
    }
}