use super::TagHandler;
use super::StructuredPrinter;

use html5ever::rcdom::NodeData;

#[derive(Default)]
pub struct CodeHandler {
    code_type: String
}

impl CodeHandler {
    fn do_handle(&mut self, printer: &mut StructuredPrinter) {
        let immediate_parent = printer.parent_chain.last().unwrap().to_owned();
        if self.code_type == "code" && immediate_parent == "pre" {
            // we are already in "code" mode, just add newline
            printer.insert_newline();
            return;
        }

        if self.code_type == "pre" {
            // switch to code mode
            printer.insert_str("```");
        }

        if self.code_type == "code" {
            // switch to inline code mode
            printer.insert_str("`");
        }
    }
}

impl TagHandler for CodeHandler {
    
    fn handle(&mut self, tag: &NodeData, printer: &mut StructuredPrinter) {
        self.code_type = match tag {
            &NodeData::Element { ref name, .. } => name.local.to_string(),
            _ => String::new()
        };

        self.do_handle(printer);
    }
    fn after_handle(&mut self, printer: &mut StructuredPrinter) {
        self.do_handle(printer);
    }

    fn is_applicable(&self, tag_name: String) -> bool {
        return tag_name == "code" || tag_name == "pre";
    }
}