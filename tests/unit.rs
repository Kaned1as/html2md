extern crate html2md;

use html2md::parse;

#[test]
fn test() {
    parse("<p>aaaaa</p>");
}

#[test]
fn test_anchor() {
    parse(r#"<p><a href="http://ya.ru">APOSIMZ</a></p>"#);
}

#[test]
fn test_anchor2() {
    parse(r#"<p><a href="http://ya.ru">APOSIMZ</a><a href="http://yandex.ru">SIDONIA</a></p>"#);
}

#[test]
fn test_anchor3() {
    parse(r#"<p><a href="http://ya.ru">APOSIMZ</a><p/><a href="http://yandex.ru">SIDONIA</a></p>"#);
}


#[test]
fn test_image() {
    parse(r#"<p><a href="https://gitter.im/MARC-FS/Lobby?utm_source=badge&amp;utm_medium=badge&amp;utm_campaign=pr-badge&amp;utm_content=badge"><img src="https://img.shields.io/gitter/room/MARC-FS/MARC-FS.svg" alt="Gitter"></a><br>"#);
}


#[test]
fn test_headers() {
    parse(r#"<h1 id="marc-fs">MARC-FS</h1><p><a href="http://Mail.ru">Mail.ru</a> Cloud filesystem written for FUSE</p><h2 id="synopsis">Synopsis</h2>"#);
}

#[test]
fn test_list() {
    parse(r#"<p><ul><li>Seven things has lady Lackless</li><li>Keeps them underneath her black dress</li><li>One a thing that's not for wearing</li></ul></p>"#);
}