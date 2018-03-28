use super::TagHandler;
use super::StructuredPrinter;

use html5ever::rcdom::NodeData;

#[derive(Default)]
pub struct ListHandler {
    depth: usize
}

impl TagHandler for ListHandler {

    /// we're entering "ul" pr or "ol" tag, no "li" handing here
    fn handle(&mut self, _tag: &NodeData, printer: &mut StructuredPrinter) {
        let parent_lists: Vec<&String> = printer.parent_chain.iter().rev().filter(|&tag| tag == "ul" || tag == "ol" || tag == "menu").collect();
        self.depth = parent_lists.len();
        if self.depth == 0 {
            // don't indent top-level lists
            return;
        }

        // this is one of inner lists, increase indentation
        printer.indent += 4;
    }

    fn after_handle(&mut self, printer: &mut StructuredPrinter) {
        if self.depth == 0 {
            // don't decrease indent on top-level lists
            return;
        }

        // this is one of inner lists, decrease indentation
        printer.indent -= 4;
    }

    fn is_applicable(&self, tag_name: String) -> bool {
        return tag_name == "ul" || tag_name == "menu" || tag_name == "ol";
    }
}

#[derive(Default)]
pub struct ListItemHandler {
    list_type: String
}

impl TagHandler for ListItemHandler {

    fn handle(&mut self, _tag: &NodeData, printer: &mut StructuredPrinter) {
        {
            let parent_lists: Vec<&String> = printer.parent_chain.iter().rev().filter(|&tag| tag == "ul" || tag == "ol" || tag == "menu").collect();
            let nearest_parent_list = parent_lists.first();
            if nearest_parent_list.is_none() {
                // no parent list
                // should not happen - html5ever cleans html input when parsing
                return;
            }

            self.list_type = nearest_parent_list.unwrap().to_string();
        }

        if printer.data.trim_matches(' ').chars().last() != Some('\n') {
            // insert newline when declaring a list item only in case there isn't any newline at the end of text
            printer.insert_newline(); 
        }

        match self.list_type.as_ref() {
            "ul" | "menu" => printer.insert_str("* "), // unordered list: *, *, *
            "ol" => printer.insert_str("1. "), // ordered list: 1, 2, 3
            _ => {} // never happens
        }
    }

    fn after_handle(&mut self, printer: &mut StructuredPrinter) {
        if printer.data.trim_matches(' ').chars().last() != Some('\n') {
            // insert newline after list item was finished only if internal paragraph didn't do it already
            printer.insert_newline(); 
        }
    }

    fn is_applicable(&self, tag_name: String) -> bool {
        //return tag_name == "ul" || tag_name == "menu" || tag_name == "ol" || tag_name == "li";
        return tag_name == "li";
    }
}