use super::TagHandler;
use super::StructuredPrinter;

use html5ever::rcdom::Handle;

#[derive(Default)]
pub(super) struct ListHandler {
    should_indent: bool,
    start_pos: usize
}

impl TagHandler for ListHandler {

    /// we're entering "ul" or "ol" tag, no "li" handing here
    fn handle(&mut self, _tag: &Handle, printer: &mut StructuredPrinter) {
        {
            let parent_lists: Vec<&String> = printer.parent_chain.iter().rev().filter(|&tag| tag == "ul" || tag == "ol" || tag == "menu").collect();
            self.should_indent = parent_lists.len() > 0;
            self.start_pos = printer.position;
        }

        printer.insert_newline(); 
    }

    /// indent now-ready list
    fn after_handle(&mut self, printer: &mut StructuredPrinter) {
        if !self.should_indent {
            return;
        }

        let indent = 4; // 4 spaces for each indentation level
        let mut index = printer.data.len();
        while index >= self.start_pos {
            if printer.data.as_bytes().iter().nth(index) == Some(&b'\n') {
                printer.data.insert_str(index + 1, &" ".repeat(indent));
                printer.position += indent;
            }
            index -= 1;
        }
    }
}

#[derive(Default)]
pub struct ListItemHandler {
    list_type: String
}

impl TagHandler for ListItemHandler {

    fn handle(&mut self, _tag: &Handle, printer: &mut StructuredPrinter) {
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

    fn after_handle(&mut self, _printer: &mut StructuredPrinter) {
    }
}