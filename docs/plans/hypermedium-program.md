# hypermedium-program

status: active
updated: 2026-11-09  rev: 1

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
| 5 | docs-search: FST+roaring wasm index (L0) | planned | SPEC.md REQ-Q | scaffold after SPEC review |
| 6 | docs-proof + docs-symbols: certificate + symbol anchors (L0) | planned | SPEC.md REQ-P / REQ-S | design directive grammar first |
| 7 | Register crates in estate.yml; estate-audit green | planned | engineering-standards/estate.yml | after first crate publishes |

## Next action

User reviews SPEC.md; on approval, scaffold docs-witness + docs-search (M2)
as separate Omni-template repos per SPEC §3.

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
