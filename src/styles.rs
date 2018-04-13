use super::TagHandler;
use super::StructuredPrinter;

use html5ever::rcdom::{Handle,NodeData};

#[derive(Default)]
pub(super) struct StyleHandler {
    style_type: String
}

impl TagHandler for StyleHandler {
    
    fn handle(&mut self, tag: &Handle, printer: &mut StructuredPrinter) {
        self.style_type = match tag.data {
            NodeData::Element { ref name, .. } => name.local.to_string(),
            _ => String::new()
        };

        // starting markup
        match self.style_type.as_ref() {
            "b" | "strong" => printer.insert_str("**"), // bold
            "i" | "em" => printer.insert_str("*"),      // italic
            "s" | "del" => printer.insert_str("~~"),    // strikethrough
            "u" | "ins" => printer.insert_str("__"),    // underline
            _ => {}
        }
    }

    fn after_handle(&mut self, printer: &mut StructuredPrinter) {
        // finishing markup
        match self.style_type.as_ref() {
            "b" | "strong" => printer.insert_str("**"),
            "i" | "em" => printer.insert_str("*"),
            "s" | "del" => printer.insert_str("~~"),
            "u" | "ins" => printer.insert_str("__"),
            _ => {}
        }
    }
}