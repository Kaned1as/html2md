use super::TagHandler;
use super::StructuredPrinter;

use html5ever::rcdom::NodeData;

#[derive(Default)]
pub struct ListHandler {
    list_type: String
}

impl TagHandler for ListHandler {

    fn handle(&mut self, tag: &NodeData, _printer: &mut StructuredPrinter) {
        self.list_type = match tag {
            &NodeData::Element { ref name, .. } => name.local.to_string(),
            _ => String::new()
        };
    }

    fn after_handle(&mut self, printer: &mut StructuredPrinter) {
        match self.list_type.as_ref() {
            "ul" => { printer.data.insert_str(printer.position, "\n\n");  printer.position += 2; } // unordered list: *, *, *
            "ol" => { printer.data.insert_str(printer.position, "\n"); printer.position += 1; } // ordered list: 1, 2, 3
            _ => {}
        }
    }

    fn is_applicable(&self, tag_name: String) -> bool {
        return tag_name == "ul" || tag_name == "ol";
    }
}