//! Island emission: the strategic seam (ADR-0004 §2.5).
//!
//! Leptos and Dioxus islands are Rust/WASM-only, so a Rust shell cannot
//! hydrate the estate's existing SolidJS bundles — there is no
//! astro-island-compatible protocol anywhere in Rust. This module emits
//! that protocol's *shape* (a custom element with `component-url` +
//! `renderer-url`, JSON props, a `client:*` hydration hint), and
//! `islands/island.mjs` implements the ~120-line client side. The result:
//! the docs core can be Rust while the islands stay the SolidJS packages
//! pediment/hosting-dashboard already maintain.

use crate::error::DocsError;

/// How an island wants to hydrate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Client {
    /// Immediately.
    Load,
    /// When scrolled into view (the default).
    Visible,
    /// When the browser is idle.
    Idle,
}

impl Client {
    /// Parse the fence-info hint; unknown values fall back to `Visible`
    /// (documented, total).
    #[must_use]
    pub fn parse(src: &str) -> Client {
        match src {
            "load" => Client::Load,
            "idle" => Client::Idle,
            _ => Client::Visible,
        }
    }

    /// The wire value.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Client::Load => "load",
            Client::Visible => "visible",
            Client::Idle => "idle",
        }
    }
}

/// A parsed ```` ```island ```` fence: the island's component name,
/// hydration hint, and raw JSON props from the fence body.
#[derive(Debug, Clone, PartialEq)]
pub struct Island {
    /// Component name (kebab-case; maps to a client bundle).
    pub name: String,
    /// Hydration hint.
    pub client: Client,
    /// Raw JSON props (validated at emission).
    pub props: String,
}

impl Island {
    /// Parse a fence info string like `island nav-search client=load` plus a
    /// JSON props body.
    ///
    /// # Errors
    /// [`DocsError::IslandWithoutName`] when the name is missing;
    /// [`DocsError::InvalidIslandProps`] when the body is not JSON.
    pub fn parse(info: &str, body: &str) -> Result<Island, DocsError> {
        let mut words = info.split_whitespace();
        let keyword = words.next().unwrap_or("");
        if keyword != "island" {
            return Err(DocsError::IslandWithoutName(info.to_owned()));
        }
        let mut name = String::new();
        let mut client = Client::Visible;
        for word in words {
            match word.split_once('=') {
                Some(("client", value)) => client = Client::parse(value),
                Some(_) => {}
                None => name = word.to_owned(),
            }
        }
        if name.is_empty() {
            return Err(DocsError::IslandWithoutName(info.to_owned()));
        }
        let props = body.trim();
        if !props.is_empty() {
            // Validate as JSON without pulling in serde_json: the shim parses
            // it client-side with JSON.parse; we only need to know it's
            // shaped like an object, so a parse error is caught here, at
            // build time, not in the browser.
            let first = props.chars().next();
            let last = props.chars().last();
            if first != Some('{') || last != Some('}') {
                return Err(DocsError::InvalidIslandProps(props.to_owned()));
            }
        }
        Ok(Island {
            name,
            client,
            props: if props.is_empty() {
                "{}".to_owned()
            } else {
                props.to_owned()
            },
        })
    }

    /// The static HTML for this island: a custom element carrying the
    /// component URL, the hydration hint, and the props as a JSON
    /// `<script type="application/json">` (never an inline eval).
    #[must_use]
    pub fn to_html(&self, base: &str) -> String {
        let component_url = format!("{}/islands/{}.js", base.trim_end_matches('/'), self.name);
        format!(
            "<docs-island component-url=\"{component_url}\" client=\"{client}\"><script type=\"application/json\" data-island-props>{props}</script></docs-island>",
            client = self.client.as_str(),
            props = escape_json_for_html(&self.props),
        )
    }
}

/// Escape the JSON so it can live inside an HTML script element: the only
/// character that can terminate the element early is `<`.
fn escape_json_for_html(src: &str) -> String {
    src.replace('<', "\\u003c")
}
