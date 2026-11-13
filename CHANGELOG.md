
## [0.2.0] - 2026-11-13

### Added

- **Math (Wave 2):** `$...$` and `$$...$$` render to build-time MathML
  via `latex2mathml` - no client JS, the browser renders `<math>`
  natively. Malformed LaTeX fails the render with `DocsError::BadMath`
  (the offending span is named). Note latex2mathml 0.2 is strict about
  structure (unclosed groups, dangling `\left`) but lenient about
  unknown commands; pulldown-cmark's math scanner additionally skips
  closers inside unbalanced braces, so brace-broken spans stay literal
  text rather than erroring.
- **Sanitization (Wave 2):** `sanitize(html)` - ammonia's default
  allowlist plus the MathML element set the renderer emits; and
  `render_untrusted(markdown)` - island fences are dropped BEFORE
  rendering (hydration is a trusted-author surface: a third-party
  document must not choose which bundles a page loads, and the island
  props script cannot be value-constrained by a sanitizer), then the
  output is sanitized. Raw HTML from untrusted sources survives only
  where the allowlist says so.

### Changed

- Dependency floor grows by two (Wave 2 unfreezes the M4 floor, per
  SPEC §3): `latex2mathml`, `ammonia`.

## [0.2.1] - 2026-11-13

### Security

- `ammonia` upgraded to 4.2.5 for RUSTSEC-2026-0533 (mutation XSS via
  MathML/SVG namespace confusion - the exact integration points this
  crate's sanitizer allowlists). No API changes; the sanitizer's
  allowlist is unchanged and the crate's own tests pass unchanged.
  Found by docs-compiler's cargo-deny advisory gate the day the SEO
  emission shipped.
