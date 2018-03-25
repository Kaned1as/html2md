extern crate html2md;

use html2md::parse;

#[test]
fn test() {
    parse("<p>aaaaa</p>");
}

#[test]
fn test_anchor() {
    parse("<p><a href=\"http://ya.ru\">mumumu</a></p>");
}
