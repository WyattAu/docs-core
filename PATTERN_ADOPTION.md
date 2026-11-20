# Pattern adoption: docs-core

Relationships to the pattern canon (pattern-standards), verified against code.
Format: pattern-standards/references/adoption-format.md.

| Pattern | Status | Evidence |
|---|---|---|
| T1 | partial | src/sanitize.rs:1 (untrusted markdown is stripped against an ammonia allowlist pre-render; island props stay a trusted-author surface, so the typed-parse boundary stops at the island seam) |
| P59 | adopted | src/island.rs:1 (emits the astro-island protocol shape so the Rust core hosts the estate's existing SolidJS islands instead of re-implementing them) |
