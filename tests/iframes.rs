extern crate html2md;

use html2md::parse_html;

#[test]
fn test_youtube_simple() {
    let md = parse_html("<iframe src='https://www.youtube.com/embed/zE-dmXZp3nU?wmode=opaque' class='fr-draggable' width='640' height='360'></iframe>");
    assert_eq!(md, "[![Embedded YouTube video](https://img.youtube.com/vi/zE-dmXZp3nU/0.jpg)](https://www.youtube.com/watch?v=zE-dmXZp3nU)")
}

#[test]
fn test_instagram_simple() {
    let md = parse_html("<iframe src='https://www.instagram.com/p/B1BKr9Wo8YX/embed/' width='600' height='600'></iframe>");
    assert_eq!(md, "[![Embedded Instagram post](https://www.instagram.com/p/B1BKr9Wo8YX/media/?size=m)](https://www.instagram.com/p/B1BKr9Wo8YX/embed/)")
}