extern crate html2md;

use html2md::parse_html;

#[test]
fn test_dumb() {
    let md = parse_html("<p>CARTHAPHILUS</p>");
    assert_eq!(md, "CARTHAPHILUS\n\n")
}

#[test]
fn test_anchor() {
    let md = parse_html(r#"<p><a href="http://ya.ru">APOSIMZ</a></p>"#);
    assert_eq!(md, "[APOSIMZ](http://ya.ru)\n\n")
}

#[test]
fn test_anchor2() {
    let md = parse_html(r#"<p><a href="http://ya.ru">APOSIMZ</a><a href="http://yandex.ru">SIDONIA</a></p>"#);
    assert_eq!(md, "[APOSIMZ](http://ya.ru)[SIDONIA](http://yandex.ru)\n\n")
}

#[test]
fn test_anchor3() {
    let md = parse_html(r#"<p><a href="http://ya.ru">APOSIMZ</a><p/><a href="http://yandex.ru">SIDONIA</a></p>"#);
    assert_eq!(md, "[APOSIMZ](http://ya.ru)\n\n[SIDONIA](http://yandex.ru)\n\n")
}


#[test]
fn test_image() {
    let md = parse_html(r#"<p><a href="https://gitter.im/MARC-FS/Lobby?utm_source=badge&amp;utm_medium=badge&amp;utm_campaign=pr-badge&amp;utm_content=badge"><img src="https://img.shields.io/gitter/room/MARC-FS/MARC-FS.svg" alt="Gitter"></a><br>"#);
    assert_eq!(md, "[![Gitter](https://img.shields.io/gitter/room/MARC-FS/MARC-FS.svg)](https://gitter.im/MARC-FS/Lobby?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)\n\n")
}


#[test]
fn test_headers() {
    let md = parse_html(r#"<h1 id="marc-fs">MARC-FS</h1><p><a href="http://Mail.ru">Mail.ru</a> Cloud filesystem written for FUSE</p><h2 id="synopsis">Synopsis</h2>"#);
    assert_eq!(md, "\nMARC-FS\n==========\n[Mail.ru](http://Mail.ru) Cloud filesystem written for FUSE\n\nSynopsis\n----------\n")
}

#[test]
fn test_list() {
    let md = parse_html(r#"<p><ul><li>Seven things has lady Lackless</li><li>Keeps them underneath her black dress</li><li>One a thing that's not for wearing</li></ul></p>"#);
    assert_eq!(md, r#"

* Seven things has lady Lackless
* Keeps them underneath her black dress
* One a thing that's not for wearing

"#)
}

#[test]
fn test_list_formatted() {
    // let's use some some broken html
    let md = parse_html(r#"
        <ul><p>
            <li>You should NEVER see this error
                <ul>
                    <li>Broken lines, broken strings
                    <li>Broken threads, broken springs</li>
                    <li>Broken idols, broken heads
                    <li>People sleep in broken beds</li>
                </ul>
            </li>
            <li>Ain't no use jiving</li>
            <li>Ain't no use joking</li>
            <li>EVERYTHING IS BROKEN
    "#);
    assert_eq!(md, r#"

* You should NEVER see this error
    * Broken lines, broken strings
    * Broken threads, broken springs
    * Broken idols, broken heads
    * People sleep in broken beds
* Ain't no use jiving
* Ain't no use joking
* EVERYTHING IS BROKEN"#)
}

#[test]
fn test_quotes() {
    let md = parse_html("<p><blockquote>here's a quote\n next line of it</blockquote></p>");
    assert_eq!(md, "\n\n> here's a quote next line of it\n\n")
}

#[test]
fn test_quotes2() {
    let md = parse_html("<p><blockquote>here's<blockquote>nested quote!</blockquote> a quote\n next line of it</blockquote></p>");
    assert_eq!(md, r#"

> here's
> > nested quote!
>  a quote next line of it

"#)
}


#[test]
fn test_tables() {
    let md = parse_html(r#"<table>
  <thead>
    <tr>
      <th scope='col'>Minor1</th>
      <th scope='col'>Minor2</th>
      <th scope='col'>Minor3</th>
      <th scope='col'>Minor4</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>col1</td>
      <td>col2</td>
      <td>col3</td>
      <td>col4</td>
    </tr>
  </tbody>
</table>"#);
    println!("{}", md);
}