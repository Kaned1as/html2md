use super::TagHandler;
use super::StructuredPrinter;

use html5ever::rcdom::NodeData;

#[derive(Default)]
pub struct ListHandler {
    depth: usize,
    elem_type: String,
    list_type: String
}

impl TagHandler for ListHandler {

    fn handle(&mut self, tag: &NodeData, printer: &mut StructuredPrinter) {
        self.elem_type = match tag {
            &NodeData::Element { ref name, .. } => name.local.to_string(),
            _ => String::new()
        };

        if self.elem_type != "li" {
            return;
        }

        let parent_lists: Vec<&String> = printer.parent_chain.iter().rev().filter(|&tag| tag == "ul" || tag == "ol" || tag == "menu").collect();
        self.depth = parent_lists.len() - 1; // don't indent simple lists

        let list_type = parent_lists.first();
        if list_type.is_none() {
            // no parent list
            // should not happen - html5ever cleans html input when parsing
            return;
        }

        
        if printer.data.as_bytes().get(printer.position - 1).unwrap_or(&0) != &b'\n' {
            printer.data.insert_str(printer.position, "\n"); 
            printer.position += 1;
        }

        self.list_type = list_type.unwrap().to_string();
        printer.data.insert_str(printer.position, &" ".repeat(self.depth * 4)); 
        printer.position += self.depth * 4; // indent inner lists

        match self.list_type.as_ref() {
            "ul" | "menu" => { printer.data.insert_str(printer.position, "* ");  printer.position += 2; } // unordered list: *, *, *
            "ol" => { printer.data.insert_str(printer.position, "1. "); printer.position += 3; } // ordered list: 1, 2, 3
            _ => {} // never happens
        }
    }

    fn after_handle(&mut self, printer: &mut StructuredPrinter) {
        if self.elem_type != "li" {
            return;
        }

        printer.data.insert_str(printer.position, "\n");  
        printer.position += 1;
    }

    fn is_applicable(&self, tag_name: String) -> bool {
        //return tag_name == "ul" || tag_name == "menu" || tag_name == "ol" || tag_name == "li";
        return tag_name == "li";
    }
}