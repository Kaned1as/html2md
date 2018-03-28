use super::TagHandler;
use super::StructuredPrinter;

use html5ever::rcdom::NodeData;

#[derive(Default)]
pub struct HeaderHandler {
    header_type: String,
}

impl TagHandler for HeaderHandler {

    fn handle(&mut self, tag: &NodeData, printer: &mut StructuredPrinter) {
        self.header_type = match tag {
            &NodeData::Element { ref name, .. } => name.local.to_string(),
            _ => String::new()
        };

        if self.header_type == "h3" {
            printer.insert_str("### ");
        }

        if self.header_type == "h4" {
            printer.insert_str("#### ");
        }
    }

    fn after_handle(&mut self, printer: &mut StructuredPrinter) {
        match self.header_type.as_ref() {
            "h1" => printer.insert_str("\n==========\n"),
            "h2" => printer.insert_str("\n----------\n"),
            "h3" => printer.insert_str(" ###\n"),
            "h4" => printer.insert_str(" ####\n"),
            _ => {}
        }
    }

    fn is_applicable(&self, tag_name: String) -> bool {
        return tag_name == "h1" || tag_name == "h2" || tag_name == "h3" || tag_name == "h4";
    }
}