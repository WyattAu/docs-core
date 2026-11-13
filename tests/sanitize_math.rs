//! Wave 2 oracles: build-time math and the untrusted-content boundary.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic
)]

use docs_core::{render, render_untrusted, sanitize, DocsError};

#[test]
fn inline_and_display_math_become_build_time_mathml() {
    let html = render(
        "Euler: $e^{i\\pi} + 1 = 0$ inline.\n\n$$\\frac{a}{b}$$\n",
        "/",
    )
    .unwrap();
    assert!(html.contains("<math"), "inline math emitted MathML: {html}");
    assert!(html.contains("display=\"inline\""), "{html}");
    assert!(html.contains("display=\"block\""), "display math: {html}");
    assert!(
        html.contains("<mi>e</mi>") || html.contains("mi"),
        "LaTeX content converted: {html}"
    );
    assert!(
        !html.contains("$e^"),
        "the LaTeX source must not leak: {html}"
    );
}

#[test]
fn malformed_latex_fails_the_render_not_the_page() {
    // latex2mathml 0.2 is strict about structure (unclosed groups error)
    // but lenient about unknown commands (they pass through) - the oracle
    // uses the class of input the renderer can actually reject.
    let err = render("$\\left(x$\n", "/").unwrap_err();
    assert!(
        matches!(err, DocsError::BadMath(ref s) if s.contains("\\left")),
        "{err}"
    );
}

#[test]
fn render_untrusted_strips_island_fences_and_kills_raw_html() {
    let md = "Prose.\n\n```island nav-search client=load\n{\"count\": 3}\n```\n\n<script>alert(1)</script>\n\n<img src=x onerror=alert(2)>\n";
    let html = render_untrusted(md).unwrap();
    assert!(
        !html.contains("docs-island"),
        "untrusted content cannot declare islands: {html}"
    );
    assert!(!html.contains("nav-search"), "{html}");
    assert!(!html.contains("<script"), "scripts die: {html}");
    assert!(!html.contains("onerror"), "event handlers die: {html}");
    assert!(html.contains("<p>Prose.</p>"), "prose survives: {html}");
}

#[test]
fn sanitize_keeps_math_and_basic_markup() {
    let rendered = render("$$x^2$$\n", "/").unwrap();
    let clean = sanitize(&rendered);
    assert!(
        clean.contains("<math"),
        "math survives its own sanitizer: {clean}"
    );
    assert!(clean.contains("display=\"block\""), "{clean}");

    let clean =
        sanitize("<b>ok</b><script>evil()</script><math display=\"block\"><mi>x</mi></math>");
    assert!(clean.contains("<b>ok</b>"), "{clean}");
    assert!(!clean.contains("script"), "{clean}");
    assert!(clean.contains("<math"), "{clean}");
}
