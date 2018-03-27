use super::TagHandler;
use super::StructuredPrinter;

use html5ever::rcdom::NodeData;

#[derive(Default)]
pub struct StyleHandler {
    style_type: String
}

impl TagHandler for StyleHandler {
    
    fn handle(&mut self, tag: &NodeData, printer: &mut StructuredPrinter) {
        self.style_type = match tag {
            &NodeData::Element { ref name, .. } => name.local.to_string(),
            _ => String::new()
        };

        // starting markup
        match self.style_type.as_ref() {
            "b" | "strong" => { printer.data.insert_str(printer.position, "**"); printer.position += 2; }
            "i" | "em" => { printer.data.insert_str(printer.position, "*"); printer.position += 1; }
            "s" | "del" => { printer.data.insert_str(printer.position, "~~"); printer.position += 2; }
            _ => {}
        }
    }

    fn after_handle(&mut self, printer: &mut StructuredPrinter) {
        // finishing markup
        match self.style_type.as_ref() {
            "b" | "strong" => { printer.data.insert_str(printer.position, "**"); printer.position += 2; }
            "i" | "em" => { printer.data.insert_str(printer.position, "*"); printer.position += 1; }
            "s" | "del" => { printer.data.insert_str(printer.position, "~~"); printer.position += 2; }
            _ => {}
        }
    }

    fn is_applicable(&self, tag_name: String) -> bool {
        return tag_name == "b" || tag_name == "i" || tag_name == "s" || tag_name == "strong" || tag_name == "em" || tag_name == "del";
    }
}