//! Sanitization for untrusted content (Wave 2).
//!
//! Design, recorded in the plan ledger (W2-2): untrusted markdown cannot
//! declare islands. [`render_untrusted`] strips `island` fences
//! *pre-render* - hydration is a trusted-author surface, a third-party
//! document must not choose which bundles a page loads, and the island
//! props live in a `<script type="application/json">` child that a
//! sanitizer cannot value-constrain (an attribute-presence check would
//! admit executable `<script data-island-props>`). Everything else that
//! survives rendering must also survive the ammonia allowlist below.
//!
//! [`sanitize`] is the allowlist itself, exported for callers holding
//! HTML from other estate sources. It extends ammonia's defaults with
//! the MathML set `latex2mathml` emits, so build-time math (Wave 2,
//! `render`) survives a sanitize pass unchanged.

use crate::error::DocsError;
use ammonia::Builder;
use std::collections::{HashMap, HashSet};

/// Sanitize HTML with the docs allowlist: ammonia's defaults (scripts,
/// event handlers, styles, most attributes die) plus the MathML element
/// set the crate's own math emission produces.
#[must_use]
pub fn sanitize(html: &str) -> String {
    builder().clean(html).to_string()
}

/// Render *untrusted* Markdown: island fences are dropped before
/// rendering, the HTML is then sanitized. Raw HTML from the source
/// survives only where the allowlist says so - this is the boundary the
/// trusted `render` deliberately does not have.
///
/// # Errors
/// [`DocsError`] for malformed non-island content, per `render`.
pub fn render_untrusted(markdown: &str) -> Result<String, DocsError> {
    let html = crate::render::render_inner(markdown, "", false)?;
    Ok(sanitize(&html))
}

/// The docs allowlist.
fn builder() -> Builder<'static> {
    let mut b = Builder::default();
    // MathML: the element set latex2mathml 0.2 emits (plus the standard
    // ones a future renderer version may reach). Token elements carry
    // styling attributes; the root carries the display hint and namespace.
    let token_attrs: HashSet<&str> = ["class", "mathvariant", "mathsize", "mathcolor"]
        .into_iter()
        .collect();
    let math_attrs: HashSet<&str> = ["class", "display", "xmlns"].into_iter().collect();
    let math_tags: HashSet<&str> = [
        "mi",
        "mn",
        "mo",
        "ms",
        "mtext",
        "mrow",
        "mfrac",
        "msqrt",
        "mroot",
        "msub",
        "msup",
        "msubsup",
        "munder",
        "mover",
        "munderover",
        "mtable",
        "mtr",
        "mtd",
        "mspace",
        "mstyle",
        "mmultiscripts",
        "mprescripts",
        "mphantom",
        "menclose",
        "merror",
    ]
    .into_iter()
    .collect();
    b.add_tags(math_tags);
    let mut attrs: HashMap<&str, HashSet<&str>> = HashMap::new();
    for tag in [
        "mi",
        "mn",
        "mo",
        "ms",
        "mtext",
        "mrow",
        "mfrac",
        "msqrt",
        "mroot",
        "msub",
        "msup",
        "msubsup",
        "munder",
        "mover",
        "munderover",
        "mtable",
        "mtr",
        "mtd",
        "mspace",
        "mstyle",
        "mmultiscripts",
        "mprescripts",
        "mphantom",
        "menclose",
        "merror",
    ] {
        attrs.insert(tag, token_attrs.clone());
    }
    b.add_tags(["math"]);
    attrs.insert("math", math_attrs);
    b.tag_attributes(attrs);
    b
}
