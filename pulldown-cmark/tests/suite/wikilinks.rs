// This file is auto-generated by the build script
// Please, do not modify it manually

use super::test_markdown_html;

#[test]
fn wikilinks_test_1() {
    let original = r##"This is a [[WikiLink]].
"##;
    let expected = r##"<p>This is a <a href="/WikiLink/">WikiLink</a>.</p>
"##;

    test_markdown_html(original, expected, false, false, false, false, true);
}

#[test]
fn wikilinks_test_2() {
    let original = r##"This is [[Ambiguous]].

[Ambiguous]: https://example.com/
"##;
    let expected = r##"<p>This is <a href="/Ambiguous/">Ambiguous</a>.</p>
"##;

    test_markdown_html(original, expected, false, false, false, false, true);
}

#[test]
fn wikilinks_test_3() {
    let original = r##"This is [also [[Ambiguous]]](https://example.com/).
"##;
    let expected = r##"<p>This is [also <a href="/Ambiguous/">Ambiguous</a>](https://example.com/).</p>
"##;

    test_markdown_html(original, expected, false, false, false, false, true);
}

#[test]
fn wikilinks_test_4() {
    let original = r##"This is [[WikiLink|a pothole]].
"##;
    let expected = r##"<p>This is <a href="/WikiLink/">a pothole</a>.</p>
"##;

    test_markdown_html(original, expected, false, false, false, false, true);
}

#[test]
fn wikilinks_test_5() {
    let original = r##"This is [[WikiLink|a **strong** pothole]].
"##;
    let expected = r##"<p>This is <a href="/WikiLink/">a <strong>strong</strong> pothole</a>.</p>
"##;

    test_markdown_html(original, expected, false, false, false, false, true);
}

#[test]
fn wikilinks_test_6() {
    let original = r##"This is a cute dog, linked to the page "/WikiLink/"

[[WikiLink|![dog](dog.png)]]
"##;
    let expected = r##"<p>This is a cute dog, linked to the page "/WikiLink/"</p><p><a href="/WikiLink/"><img src="dog.png" alt="dog" /></a></p>
"##;

    test_markdown_html(original, expected, false, false, false, false, true);
}

#[test]
fn wikilinks_test_7() {
    let original = r##"The url of [[This Link]] becomes "/This_Link/"
"##;
    let expected = r##"<p>The url of <a href="/This_Link/">This Link</a> becomes "/This_Link/"</p>
"##;

    test_markdown_html(original, expected, false, false, false, false, true);
}

#[test]
fn wikilinks_test_8() {
    let original = r##"This is a [[WikiLink/In/A/Directory]].
"##;
    let expected = r##"<p>This is a <a href="/WikiLink/In/A/Directory/">Directory</a>.</p>
"##;

    test_markdown_html(original, expected, false, false, false, false, true);
}

#[test]
fn wikilinks_test_9() {
    let original = r##"This is a [[WikiLink/In/A/Directory|WikiLink]].
"##;
    let expected = r##"<p>This is a <a href="/WikiLink/In/A/Directory/">WikiLink</a>.</p>
"##;

    test_markdown_html(original, expected, false, false, false, false, true);
}
