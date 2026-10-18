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
//! # Scope (M1, honest)
//!
//! In: headings, paragraphs, emphasis/strong/strike, inline + fenced code,
//! links, images, lists (incl. task lists), blockquotes, rules, and island
//! fences.
//! Out (Wave 2, per the ecosystem survey): search indexing (Pagefind),
//! versioning, i18n, math (`latex2mathml`), diagrams (D2), HTML
//! sanitization for untrusted content (`ammonia`), and the dev server.
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

pub use error::DocsError;
pub use island::{Client, Island};
pub use render::render;

/// The crate's version, for callers that report it.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
