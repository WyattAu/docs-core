# hypermedium-program

status: active
updated: 2026-11-10  rev: 3

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
| 4 | docs-witness: deterministic wasmtime runner (L1) | done | WyattAu/docs-witness@85d5667; 10 tests + doctest, clippy/fmt/vet green; CI pending first run | watch CI; publish passes with estate.yml (row 7) |
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
  README truth table lacks path rows; llms.txt missing (SC-10); both become
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
- 2026-11-10 rev 3: docs-search CI made green (run 42759850282): the
  first push had committed docs-search/.cargo/config.toml (estate-host
  target-dir redirect written by local tooling) which broke every cargo
  step on GitHub-hosted runners (/home/wyatt/.cache not creatable);
  untracked + ignored, and CI moved to the docs-core rust-kit pin
  (024f2b5f) with explicit inputs; its built-in wasm check replaces the
  custom job. cal-model still pins the older e23c50cf. docs-witness
  scaffolded and pushed (85d5667): wasmtime 34.0.2 (newest with MSRV
  1.85; v50 needs newer rust), preview1 sync linker; findings: preview1
  requires an exported "memory" from guest modules; an empty `loop br 0`
  is a Cranelift COMPILE error, not a fuel case; pipe over-capacity
  writes reject with an errno to the guest (no host block) - README and
  WitnessConfig docs match observed behavior. Next: watch docs-witness
  CI; then M2 close-out (publish both + estate.yml in one audit pass).
- 2026-11-10 rev 3: docs-witness CI first run failed on two gates, both
  fixed (dd870a2, 7121534): (1) quality/deny - wasmtime 34.0.2 hit
  RUSTSEC-2025-0118 (shared-memory unsoundness; blocking per
  SECURITY.md) plus unmaintained fxhash via the profiling feature.
  Moved to wasmtime 36.0.20 (patched line; MSRV now 1.86) with default
  features off: no profiling (fxhash/ittapi leave the tree entirely),
  no threads (no shared linear memories at all), no wat (bytes-only
  contract enforced by the build); wasmtime-wasi keeps defaults (p2 is
  the p1 builder substrate; component code is compiled but never
  linked). (2) quality/coverage - Display arms for every error variant
  plus a Link-path fixture. WASI 36 API moves: WasiCtxBuilder at the
  crate root, StdoutStream impls live in cli.rs, preview1 unchanged.
  docs-core PR #2 docs gate was failing on an em-dash in this ledger
  (SC character rule); fixed, PR CI green. Next: confirm docs-witness
  CI, then publish + estate.yml in one audit pass.
