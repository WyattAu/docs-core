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
        html.contains(
            "<docs-island component-url=\"/_assets/islands/nav-search.js\" client=\"load\">"
        ),
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
    assert!(
        html.contains("\\u003c"),
        "the < is escaped as a JSON unicode escape"
    );
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
fn unordered_lists_open_and_close_matching_elements() {
    let html = render("- one\n- two\n", "/").unwrap();
    assert_eq!(html, "<ul><li>one</li><li>two</li></ul>");
}

#[test]
fn ordered_lists_open_and_close_matching_elements() {
    let html = render("1. one\n2. two\n", "/").unwrap();
    assert_eq!(html, "<ol><li>one</li><li>two</li></ol>");
}

#[test]
fn nested_lists_close_innermost_first() {
    let html = render("- outer\n  1. inner\n- back\n", "/").unwrap();
    assert!(html.contains("<ul><li>outer"), "{html}");
    assert!(html.contains("<ol><li>inner</li></ol>"), "{html}");
    assert!(html.contains("<li>back</li></ul>"), "{html}");
    assert!(html.ends_with("</ul>"), "{html}");
    assert!(
        !html.contains("</li></ul><li"),
        "items must stay inside the list: {html}"
    );
}

#[test]
fn task_list_markers_render_both_states() {
    let html = render("- [x] done\n- [ ] todo\n", "/").unwrap();
    assert!(
        html.contains("<li><input type=\"checkbox\" disabled checked>done</li>"),
        "{html}"
    );
    assert!(
        html.contains("<li><input type=\"checkbox\" disabled>todo</li>"),
        "{html}"
    );
    assert!(html.ends_with("</ul>"), "{html}");
}

#[test]
fn blockquotes_rules_and_breaks_render() {
    let html = render("> quoted\n\n---\n", "/").unwrap();
    assert!(
        html.contains("<blockquote><p>quoted</p></blockquote>"),
        "{html}"
    );
    assert!(html.contains("<hr>"), "{html}");

    let breaks = render("line one\nline two\n\nhard  \nbreak\n", "/").unwrap();
    assert!(
        breaks.contains("line one\nline two"),
        "soft break stays a newline: {breaks}"
    );
    assert!(breaks.contains("<br>"), "hard break renders <br>: {breaks}");
}

#[test]
fn inline_code_strikethrough_and_links_render() {
    let html = render(
        "use `docs_core` and ~~old~~, see [the guide](/guide.html).",
        "/",
    )
    .unwrap();
    assert!(html.contains("<code>docs_core</code>"), "{html}");
    assert!(html.contains("<del>old</del>"), "{html}");
    assert!(
        html.contains("<a href=\"/guide.html\">the guide</a>"),
        "{html}"
    );
}

#[test]
fn images_render_with_and_without_titles() {
    let plain = render("![alt text](/img.png)", "/").unwrap();
    assert_eq!(plain, "<p><img src=\"/img.png\" alt=\"alt text\"></p>");

    let titled = render("![alt](/img.png \"the title\")", "/").unwrap();
    assert_eq!(
        titled,
        "<p><img src=\"/img.png\" alt=\"alt\" title=\"the title\"></p>"
    );
}

#[test]
fn rendering_is_deterministic() {
    let md = "# X\n\ntext\n";
    assert_eq!(render(md, "/").unwrap(), render(md, "/").unwrap());
}
