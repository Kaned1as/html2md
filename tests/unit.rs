extern crate html2md;

use html2md::parse;

#[test]
fn test_dumb() {
    let md = parse("<p>CARTHAPHILUS</p>");
    println!("{}", md);
}

#[test]
fn test_anchor() {
    let md = parse(r#"<p><a href="http://ya.ru">APOSIMZ</a></p>"#);
    println!("{}", md);
}

#[test]
fn test_anchor2() {
    let md = parse(r#"<p><a href="http://ya.ru">APOSIMZ</a><a href="http://yandex.ru">SIDONIA</a></p>"#);
    println!("{}", md);
}

#[test]
fn test_anchor3() {
    let md = parse(r#"<p><a href="http://ya.ru">APOSIMZ</a><p/><a href="http://yandex.ru">SIDONIA</a></p>"#);
    println!("{}", md);
}


#[test]
fn test_image() {
    let md = parse(r#"<p><a href="https://gitter.im/MARC-FS/Lobby?utm_source=badge&amp;utm_medium=badge&amp;utm_campaign=pr-badge&amp;utm_content=badge"><img src="https://img.shields.io/gitter/room/MARC-FS/MARC-FS.svg" alt="Gitter"></a><br>"#);
    println!("{}", md);
}


#[test]
fn test_headers() {
    let md = parse(r#"<h1 id="marc-fs">MARC-FS</h1><p><a href="http://Mail.ru">Mail.ru</a> Cloud filesystem written for FUSE</p><h2 id="synopsis">Synopsis</h2>"#);
    println!("{}", md);
}

#[test]
fn test_list() {
    let md = parse(r#"<p><ul><li>Seven things has lady Lackless</li><li>Keeps them underneath her black dress</li><li>One a thing that's not for wearing</li></ul></p>"#);
    println!("{}", md);
}

#[test]
fn test_list2() {
    let md = parse(r#"<p><ul><li>Seven things has lady Lackless<ul><li>Kingkiller<li></ul></li>Keeps them underneath her black dress</li><li>One a thing that's not for wearing</li></ul></p>"#);
    println!("{}", md);
}