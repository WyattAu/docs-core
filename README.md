# docs-core

The Rust docs pipeline core (ADR-0004 §2.5) — consolidating what Tachyon
(16 crates), `docs-pipeline`, and the eight `starlight-*` plugins do in three
languages, with the one seam the ecosystem has no answer for:
**astro-island-compatible island emission**, so a Rust-rendered shell can
hydrate the estate's existing SolidJS bundles (Leptos/Dioxus islands are
Rust/WASM-only and cannot).

````markdown
```island nav-search client=visible
{"count": 3}
```
````

renders to:

```html
<docs-island component-url="/_assets/islands/nav-search.js" client="visible">
  <script type="application/json" data-island-props>{"count":3}</script>
</docs-island>
```

and `islands/island.mjs` (dependency-free, ~120 lines) hydrates it: props
parsed with `JSON.parse`, each component URL imported at most once, `load` /
`visible` / `idle` hints, `docs:hydrated` events.

## M1 scope

- **In**: CommonMark+GFM → HTML (headings, paragraphs, emphasis/strong/
  strike, inline + fenced code, links, images, lists incl. task lists,
  blockquotes, rules) and island fences. Raw HTML passes through (the
  mdBook/Zola/Starlight contract for first-party content); island props are
  validated Rust-side *and* JS-side, and `<` is escaped so a props payload
  can never terminate the script element.
- **Wave 2** (deliberately absent, not silently): search indexing (adopt
  Pagefind), versioning, i18n, math (`latex2mathml`), diagrams (D2),
  sanitization for untrusted content (`ammonia`), dev server.

## Tests

`cargo test` (10 tests + doctest, clippy `-D warnings` clean) and
`bun test islands/` (9 headless tests of the shim's pure half).

Licensed under MIT OR Apache-2.0.
