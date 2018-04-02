use super::TagHandler;
use super::StructuredPrinter;

use html5ever::rcdom::NodeData;

#[derive(Default)]
pub(super) struct HeaderHandler {
    header_type: String,
}

impl TagHandler for HeaderHandler {

    fn handle(&mut self, tag: &NodeData, printer: &mut StructuredPrinter) {
        self.header_type = match tag {
            &NodeData::Element { ref name, .. } => name.local.to_string(),
            _ => String::new()
        };

        printer.insert_newline(); 
        match self.header_type.as_ref() {
            "h3" => printer.insert_str("### "),
            "h4" => printer.insert_str("#### "),
            "h5" => printer.insert_str("##### "),
            "h6" => printer.insert_str("###### "),
            _ => {}
        }
    }

    fn after_handle(&mut self, printer: &mut StructuredPrinter) {
        match self.header_type.as_ref() {
            "h1" => printer.insert_str("\n==========\n"),
            "h2" => printer.insert_str("\n----------\n"),
            "h3" => printer.insert_str(" ###\n"),
            "h4" => printer.insert_str(" ####\n"),
            "h5" => printer.insert_str(" #####\n"),
            "h6" => printer.insert_str(" ######\n"),
            _ => {}
        }
    }
}