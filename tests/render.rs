//! Rendering + island emission tests.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic
)]

use docs_core::{render, Client, DocsError, Island};

#[test]
fn headings_paragraphs_and_inline_markup() {
    let html = render("# Title\n\nSome *em* and **strong** and `code`.\n", "/").unwrap();
    assert!(html.contains("<h1>Title</h1>"), "{html}");
    assert!(html.contains("<em>em</em>"));
    assert!(html.contains("<strong>strong</strong>"));
    assert!(html.contains("<code>code</code>"));
    assert!(html.contains("<p>"));
}

#[test]
fn heading_levels_close_correctly() {
    let html = render("## Two\n\n### Three\n", "/").unwrap();
    assert!(html.contains("<h2>Two</h2>"));
    assert!(html.contains("<h3>Three</h3>"));
}

#[test]
fn raw_inline_html_passes_through_and_text_is_escaped() {
    // Standard Markdown contract: raw HTML in first-party docs content is
    // preserved (mdBook/Zola/Starlight behave the same); *text* is escaped.
    let passthrough = render("A <abbr title=\"x\">HTML</abbr> note\n", "/").unwrap();
    assert!(passthrough.contains("<abbr"), "{passthrough}");

    let escaped = render("5 < 7 and 3 > 2\n", "/").unwrap();
    assert!(escaped.contains("5 &lt; 7"), "{escaped}");
}

#[test]
fn island_fences_become_astro_island_compatible_elements() {
    let md = "```island nav-search client=load\n{\"count\": 3}\n```\n";
    let html = render(md, "/_assets").unwrap();
    assert!(
        html.contains("<docs-island component-url=\"/_assets/islands/nav-search.js\" client=\"load\">"),
        "{html}"
    );
    assert!(html.contains("data-island-props"), "{html}");
    assert!(html.contains("{\"count\": 3}"));
}

#[test]
fn island_props_cannot_break_out_of_the_script_tag() {
    let md = "```island evil\n{\"x\": \"</script><img src=x>\"}\n```\n";
    let html = render(md, "/").unwrap();
    assert!(
        !html.contains("</script><img"),
        "a props payload must never terminate the script element: {html}"
    );
    assert!(html.contains("\\u003c"), "the < is escaped as a JSON unicode escape");
}

#[test]
fn malformed_island_fences_are_errors() {
    assert!(matches!(
        render("```island\n{}\n```\n", "/"),
        Err(DocsError::IslandWithoutName(_))
    ));
    assert!(matches!(
        render("```island widget\nnot json\n```\n", "/"),
        Err(DocsError::InvalidIslandProps(_))
    ));
}

#[test]
fn island_parsing_rules() {
    let i = Island::parse("island nav client=idle", "").unwrap();
    assert_eq!(i.name, "nav");
    assert_eq!(i.client, Client::Idle);
    assert_eq!(i.props, "{}");
    // Unknown client hints fall back to visible (documented, total).
    assert_eq!(Client::parse("whenever"), Client::Visible);
}

#[test]
fn ordinary_fenced_code_still_renders() {
    let html = render("```rust\nfn main() {}\n```\n", "/").unwrap();
    assert!(html.contains("<pre><code>"), "{html}");
    assert!(html.contains("fn main"));
    assert!(html.contains("</code></pre>"));
}

#[test]
fn lists_and_task_lists_render() {
    let html = render("- one\n- two\n\n- [x] done\n", "/").unwrap();
    assert!(html.contains("<li>"), "{html}");
    assert!(html.contains("</ul>"));
    assert!(html.contains("type=\"checkbox\""), "{html}");
}

#[test]
fn rendering_is_deterministic() {
    let md = "# X\n\ntext\n";
    assert_eq!(render(md, "/").unwrap(), render(md, "/").unwrap());
}
