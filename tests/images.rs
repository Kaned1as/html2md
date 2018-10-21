extern crate html2md;

use html2md::parse_html;

#[test]
fn test_image_native_simple() {
    let md = parse_html("<img src=\"https://i.redd.it/vesfbmwfkz811.png\" alt=\"image of Linus holding his laptop\" title=\"Daddy Linus\" />");
    assert_eq!(md, "![image of Linus holding his laptop](https://i.redd.it/vesfbmwfkz811.png \"Daddy Linus\")")
}

#[test]
fn test_image_native_without_title() {
    let md = parse_html("<img src=\"https://i.redd.it/l0ne52x7fh611.png\" alt=\"image of usual kill -9 sequence\" />");
    assert_eq!(md, "![image of usual kill -9 sequence](https://i.redd.it/l0ne52x7fh611.png)")
}

#[test]
fn test_image_embedded_html() {
    let md = parse_html("<img src=\"https://i.redd.it/un4h28uwtp711.png\" alt=\"comics about Mac and GNU/Linux\" title=\"Look at me, brother\" height=\"150\" width=\"150\" />");
    assert_eq!(md, "<img alt=\"comics about Mac and GNU/Linux\" src=\"https://i.redd.it/un4h28uwtp711.png\" title=\"Look at me, brother\" height=\"150\" width=\"150\" />")
}

#[test]
fn test_image_embedded_with_unsupported_html() {
    // srcset is unsupported in Markdown
    let md = parse_html("<img src=\"https://i.redd.it/07onlc10x5711.png\" alt=\"HACKERMAN\" title=\"When you reboot instead of exiting vim\" height=\"150\" width=\"150\" srcset=\"image1 image2\" align=\"center\" />");
    assert_eq!(md, "<img alt=\"HACKERMAN\" src=\"https://i.redd.it/07onlc10x5711.png\" title=\"When you reboot instead of exiting vim\" height=\"150\" width=\"150\" align=\"center\" />")
}

#[test]
fn test_image_src_issue() {
    let md = parse_html("<img src=\"https://dybr.ru/img/43/1532265494_android-Kanedias\" width=\"auto\" height=\"500\" >");
    assert_eq!(md, "<img src=\"https://dybr.ru/img/43/1532265494_android-Kanedias\" height=\"500\" width=\"auto\" />")
}

#[test]
fn test_image_with_space_issue() {
    let md = parse_html("<img src=\"https://i.redd.it/l0ne 52x7f h611.png\" alt=\"image of usual kill -9 sequence\" />");
    assert_eq!(md, "![image of usual kill -9 sequence](https://i.redd.it/l0ne%2052x7f%20h611.png)")
}
