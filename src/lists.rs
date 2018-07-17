use super::TagHandler;
use super::StructuredPrinter;

use html5ever::rcdom::{Handle,NodeData};

#[derive(Default)]
pub(super) struct ListHandler {
}

impl TagHandler for ListHandler {

    /// we're entering "ul" or "ol" tag, no "li" handling here
    fn handle(&mut self, _tag: &Handle, printer: &mut StructuredPrinter) {
        printer.insert_newline(); 
    }

    /// indent now-ready list
    fn after_handle(&mut self, _printer: &mut StructuredPrinter) {
    }
}

#[derive(Default)]
pub struct ListItemHandler {
    start_pos: usize,
    list_type: String
}

impl TagHandler for ListItemHandler {

    fn handle(&mut self, _tag: &Handle, printer: &mut StructuredPrinter) {
        self.start_pos = printer.position;

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

        let current_depth = printer.parent_chain.len();
        let order = printer.siblings[&current_depth].len() + 1;
        match self.list_type.as_ref() {
            "ul" | "menu" => printer.insert_str("* "), // unordered list: *, *, *
            "ol" => printer.insert_str(&(order.to_string() + ". ")), // ordered list: 1, 2, 3
            _ => {} // never happens
        }
    }

    fn after_handle(&mut self, printer: &mut StructuredPrinter) {
        let padding = match self.list_type.as_ref() {
            "ul" => 2,
            "ol" => 3,
            _ => 4
        };

        // non-nested indentation (padding). Markdown requires that all paragraphs in the
        // list item except first should be indented with at least 1 space
        let mut index = printer.data.len();
        while index > self.start_pos {
            if printer.data.bytes().nth(index) == Some(b'\n') {
                printer.data.insert_str(index + 1, &" ".repeat(padding));
                printer.position += padding;
            }
            index -= 1;
        }
    }
}