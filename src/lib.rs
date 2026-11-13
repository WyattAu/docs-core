//! docs-core — the Rust docs pipeline core (ADR-0004 §2.5).
//!
//! Consolidates what Tachyon (16 crates), `docs-pipeline`, and the eight
//! `starlight-*` plugins do in three languages: CommonMark+GFM to static
//! HTML, plus **astro-island-compatible island emission** so the estate's
//! existing SolidJS bundles hydrate inside a Rust-rendered shell — the one
//! seam the ecosystem has no answer for (Leptos/Dioxus islands are
//! Rust/WASM-only).
//!
//! ```
//! let html = docs_core::render(
//!     "# Chisel\n\nSome *prose*.\n\n```island nav-search client=visible\n{\"count\": 3}\n```\n",
//!     "/_assets",
//! )
//! .unwrap();
//! assert!(html.contains("<h1>Chisel</h1>"));
//! assert!(html.contains("component-url=\"/_assets/islands/nav-search.js\""));
//! ```
//!
//! # Scope (M1 + Wave 2 math/sanitization, honest)
//!
//! In: headings, paragraphs, emphasis/strong/strike, inline + fenced code,
//! links, images, lists (incl. task lists), blockquotes, rules, island
//! fences, math (`$...$` / `$$...$$` as build-time MathML via
//! `latex2mathml` - no client JS, Wave 2), and sanitization for
//! untrusted content (`sanitize` + `render_untrusted`; islands are
//! stripped pre-render there, Wave 2).
//! Out (Wave 2 remainder): search indexing (Pagefind; closed - docs-search
//! is the answer, native-only per SPEC), versioning, i18n, diagrams (D2),
//! and the dev server (lands in docs-compiler: docs-core is L2, no web
//! framework).
//!
//! # Layer
//!
//! L2 — domain. One external dependency (`pulldown-cmark`); no UI, no web
//! framework — the output is static HTML plus island custom elements.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

mod error;
mod island;
mod render;
mod sanitize;

pub use error::DocsError;
pub use island::{Client, Island};
pub use render::render;
pub use sanitize::{render_untrusted, sanitize};

/// The crate's version, for callers that report it.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
