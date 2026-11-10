# hypermedium-program

status: active
updated: 2026-11-10  rev: 2

Program: extend docs-core (ADR-0004 §2.5) toward the "verifiable, reactive
documentation hypermedium": proof-carrying docs, symbol-anchored includes,
deterministic execution witnesses, client WASM SIMD search. Grounded in the
estate survey of 2026-11-09 (see Log entry 1).

## State

| # | Task | Status | Evidence | Next |
|---|------|--------|----------|------|
| 1 | Estate survey: find dogfood assets + contract | done | engineering-standards/estate.yml:437; ADR-0004 docs/adr/0004 §2.5 | - |
| 2 | Clone + read docs-core (M1 scope, API, layers) | done | local clone; src/lib.rs:20-32 (Wave 2 list); Cargo.toml:18-19 (L2) | - |
| 3 | Draft SPEC.md (4 slices, REQ anchors, milestones) | done | SPEC.md; doc-lint PASS (tier k), 2 advisories | user review of SPEC.md |
| 4 | docs-witness: deterministic wasmtime runner (L1) | planned | SPEC.md REQ-W | scaffold after SPEC review |
| 5 | docs-search: FST+roaring wasm index (L0) | done | WyattAu/docs-search@828ee1c; test/clippy/fmt/vet green; wasm32 build green | crates.io publish + estate.yml registration (row 7) |
| 6 | docs-proof + docs-symbols: certificate + symbol anchors (L0) | planned | SPEC.md REQ-P / REQ-S | design directive grammar first |
| 7 | Register crates in estate.yml; estate-audit green | planned | engineering-standards/estate.yml | after first crate publishes |

## Next action

Scaffold docs-witness (L1, wasmtime) per SPEC §4.1: ledger row 4 goes
active next session. docs-search publish (cargo publish) is a separate
gate: do it together with the estate.yml entry so GitHub, crates.io, and
the registry agree in one audit pass (estate-audit contract).

## Blocked

- none

## Log

- 2026-11-09 rev 1: estate survey complete; dogfood map established (docs-core
  hub; Tachyon predecessor; cas-kit/crdts-kit/shm-rings/formula-lang/polyfont/
  fresco as composition inputs; starlight-* is consolidation input, not
  target). SPEC.md drafted in-repo; ledger created.
- 2026-11-09 rev 1: SPEC gate run: doc-lint PASS at tier k; advisories:
  README truth table lacks path rows; llms.txt missing (SC-10) — both become
  M2 close-out items. Next: user review, then M2 scaffolds.
- 2026-11-10 rev 2: user approved; M2 started. docs-search scaffolded from
  the cal-model derivation pattern (single crate, pinned rust-kit.yml CI
  @ e23c50cf, tier b + wasm job). SPEC §3 docs-search deps amended (fst +
  roaring leaf crates; roaring latest is 0.12, not 1.0). Note: OmniLean-
  template exists upstream for the M4 Lean package. Copier is phase-1
  (answers contract only); derivation is clone-and-adapt until content is
  templated. Copier is phase-1 (answers contract only); derivation is
  clone-and-adapt until content is templated.
- 2026-11-10 rev 2: docs-search complete and pushed (828ee1c): FST +
  roaring 0.12 postings, BM25-shaped unit-tf ranking, deterministic v1
  format, total decoding. Corruption property test found fst 0.4.7
  panics on hostile dictionaries (raw/node.rs OOB/overflow); all fst
  entry points now sit behind catch_unwind boundaries (documented in
  SECURITY.md; no panic="abort"). Off-by-one in the truncation sweep
  itself (0..=len included the full blob) fixed to 0..len. Gates: test
  (8 integration + 2 property + doctests), clippy -D warnings, fmt,
  cargo vet (59 exempted), release + wasm32 builds. CI in progress on
  GitHub. Ledger and SPEC amend committed to spec/hypermedium-program;
  PR #2 open for review.
