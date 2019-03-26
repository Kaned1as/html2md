extern crate html2md;

use html2md::parse_html;

#[test]
fn test_iframe_simple() {
    let md = parse_html("<iframe src='https://www.youtube.com/embed/zE-dmXZp3nU?wmode=opaque' class='fr-draggable' width='640' height='360'></iframe>");
    assert_eq!(md, "[![Embedded video](https://img.youtube.com/vi/zE-dmXZp3nU/0.jpg)](https://www.youtube.com/watch?v=zE-dmXZp3nU)\n\n")
}