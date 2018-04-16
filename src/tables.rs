use super::TagHandler;
use super::StructuredPrinter;

use html5ever::rcdom::{Handle,NodeData};

#[derive(Default)]
pub(super) struct TableHandler {
    column_count: usize
}

impl TagHandler for TableHandler {
    
    fn handle(&mut self, tag: &Handle, printer: &mut StructuredPrinter) {
        let mut table_markup = String::new();

        // process headers
        let headers = find_children(tag, "th");
        if headers.len() > 0 {
            // initiate header
            self.column_count = headers.len();
            table_markup.push_str("|");

            for header in headers {
                let mut header_text = to_text(&header);
                if header_text.is_empty() {
                    header_text.push(' '); // just a space is enough for Markdown to recognize the header
                }
                table_markup.push_str(&header_text);
                table_markup.push_str("|"); // delimiter
            }
        }
        table_markup.push('\n');

        // process rows
        let mut rows = find_children(tag, "tr");
        {
            // detect row count
            let most_big_row = rows.iter().max_by(|left, right| collect_children(&left, "td").len().cmp(&collect_children(&right, "td").len()));
            if most_big_row.is_none() {
                // we don't have rows with content at all
                return;
            }
            // have rows with content, set column count
            self.column_count = collect_children(&most_big_row.unwrap(), "td").len();
        }

        // add header row
        table_markup.push('|');
        for index in 0..self.column_count {
            table_markup.push_str("-|");
        }
        table_markup.push('\n');

        // remove headers, leave only non-header rows now
        rows.retain(|row| { let children = row.children.borrow(); return children.iter().any(|child| tag_name(&child) == "td"); });
        for row in &rows {
            table_markup.push('|');
            let cells = collect_children(row, "td");
            for index in 0..self.column_count { // we need to fill all cells in a column, even if some rows don't have enough
                if let Some(cell) = cells.get(index) {
                    // have a cell here, fill with its content 
                    println!("{}", tag_name(&cell));
                    let text = to_text(cell);
                    if !text.is_empty() {
                        table_markup.push_str(&to_text(cell));
                        table_markup.push('|');
                        continue;
                    }
                }

                // don't have a cell or cell is empty
                table_markup.push(' ');
                table_markup.push('|');
            }
            table_markup.push('\n');
        }

        printer.insert_newline();
        printer.insert_str(&table_markup);
    }

    fn after_handle(&mut self, printer: &mut StructuredPrinter) {
        
    }

    fn skip_descendants(&self) -> bool {
        return true;
    }
}

fn tag_name(tag: &Handle) -> String {
    return match tag.data {
        NodeData::Element { ref name, .. } => name.local.to_string(),
        _ => String::new()
    }
}

/// Find descendants of this tag with tag name `name`
/// This includes both direct children and descendants
fn  find_children(tag: &Handle, name: &str) -> Vec<Handle> {
    let mut result: Vec<Handle> = vec![];
    let children = tag.children.borrow();
    for child in children.iter() {
        if tag_name(&child) == name {
            result.push(child.clone());
        }

        let mut descendants = find_children(&child, name);
        result.append(&mut descendants);
    }

    return result;
}

/// Collect direct children with same name
/// This doesn't include descendants
fn collect_children(tag: &Handle, name: &str) -> Vec<Handle> {
    let mut result: Vec<Handle> = vec![];
    let children = tag.children.borrow();
    for child in children.iter() {
        if tag_name(&child) == name {
            result.push(child.clone());
        }
    }

    return result;
}

/// Convert html tag to text. This collects all tag children in correct order where they're observed
/// and concatenates their text, recursively.
fn  to_text(tag: &Handle) -> String {
    let mut result = String::new();
    match tag.data {
        NodeData::Text { ref contents }  => result.push_str(&contents.borrow().trim()),
        _ => {}
    }
    let children = tag.children.borrow();
    for child in children.iter() {
        let child_text = to_text(child);
        result.push_str(&child_text);
    }
    
    return result;
}