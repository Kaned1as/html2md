use super::TagHandler;
use super::StructuredPrinter;

use html5ever::rcdom::NodeData;

#[derive(Default)]
pub struct HeaderHandler {
    header_type: String,
}

impl TagHandler for HeaderHandler {

    fn before_handle(&mut self, parent_handler: &TagHandler) {
        
    }

    fn handle(&mut self, tag: &NodeData, printer: &mut StructuredPrinter) {
        self.header_type = match tag {
            &NodeData::Element { ref name, .. } => name.local.to_string(),
            _ => String::new()
        };

        if self.header_type == "h3" {
            printer.data.insert_str(printer.position, "### ");
            printer.position += 4;
        }
    }

    fn after_handle(&mut self, printer: &mut StructuredPrinter) {
        match self.header_type.as_ref() {
            "h1" => { printer.data.insert_str(printer.position, "\n==========\n"); printer.position += 12; }
            "h2" => { printer.data.insert_str(printer.position, "\n----------\n"); printer.position += 12; }
            "h3" => { printer.data.insert_str(printer.position, " ###\n"); printer.position += 5; }
            _ => {}
        }
    }

    fn is_applicable(&self, tag_name: String) -> bool {
        return tag_name == "h1" || tag_name == "h2" || tag_name == "h3";
    }
}