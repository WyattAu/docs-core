# SPEC: docs-core extension program (hypermedium slices)

Status: CURRENT (adopted 2026-11-12; gates green at tier k, PR #2 merged)
Tier: k (library kits; composition binaries inherit Tier B CI unless concurrency-primitive)
Owner: WyattAu
Scope: the four "hypermedium" slices that extend docs-core (ADR-0004 §2.5):
proof-carrying documentation, symbol-anchored includes, deterministic
execution witnesses, and client-side WASM search. Each slice is a new estate
crate; docs-core itself stays minimal.

## 1. Purpose

docs-core M1 renders CommonMark+GFM to static HTML with island emission and
names its Wave 2 (search, math, sanitization, dev server) as deliberately
absent. This SPEC defines how four Wave 2-adjacent capabilities land as
independent crates in the `docs-*` domain (naming Rule 2), composed by an
L3 compiler binary, without growing docs-core's dependency surface beyond
`pulldown-cmark`.

Design center: the static-shell contract is not negotiable. Every slice
emits plain HTML/static payloads at build time; client interactivity rides
the existing `<docs-island>` seam. No slice may introduce a runtime JS
framework or move prose rendering into WASM.

## 2. Where the truth lives

| Concern | Truth | Projections |
|---|---|---|
| docs-core rendering contract | docs-core `src/render.rs`, `src/island.rs` | docs-core README (M1 scope) |
| Directive grammars (witness/include/proof) | this SPEC §4 | crate docs, README examples |
| Estate registration + pins | engineering-standards/estate.yml | estate-audit reports |
| Layer assignments | engineering-standards/docs/layers.md + scripts/estate-tiers.json | Cargo.toml `[package.metadata.layers]` |
| Milestone status | docs/plans/hypermedium-program.md (ledger) | - |

## 3. Crate topology

| Crate (repo = crate, Rule 1) | Layer | Deps (new external) | Role |
|---|---|---|---|
| `docs-core` (exists) | L2 | pulldown-cmark | HTML emission + island fences; unchanged through M4 |
| `docs-witness` | L1 | wasmtime | deterministic sandboxed snippet execution; witness payload model |
| `docs-search` | L0 | fst, roaring (leaf crates; zero estate deps) | FST + roaring-bitmap index format; wasm32-clean; quantized vector option behind feature |
| `docs-symbols` | L0 | tree-sitter (+per-lang grammars) | symbol-anchored extraction; line-shift-invariant spans |
| `docs-proof` | L0 | none (serde only) | proof-certificate model + verifier; Lean never linked |
| `docs-compiler` | L3 | all above + docs-core | CLI: resolves directives, invokes lean out-of-process, emits site |

Directive resolution happens in `docs-compiler` before docs-core rendering:
fences are expanded to plain HTML (witness output blocks, extracted symbol
spans, proof badges), so docs-core's grammar and dependency floor stay
frozen through M4. Rationale: docs-core carries one external dep by design
(src/lib.rs:31); directive semantics belong to the compiler, not the emitter.

## 4. Normative requirements

### 4.1 Execution witnesses (REQ-W)

- REQ-W1: a ```witness fence executes exactly once per build inside a
  fresh wasmtime instance with no network, no filesystem, and a fuel/step
  limit; build fails if the snippet traps, times out, or exceeds the limit.
  Oracle: docs-witness integration test suite (lands M2).
- REQ-W2: the emitted page contains the captured stdout, exit status, and
  wall/fuel spend as static HTML; nothing re-executes client-side.
  Oracle: golden-output test in docs-witness (M2).
- REQ-W3: ```witness fences accept `assert_stdout` / `assert_exit`;
  assertion mismatch fails the build. No-oracle: lands with M2 tests.
- REQ-W4: execution is deterministic across runs on the same host; the
  witness payload records toolchain + wasmtime version. Oracle: repeated
  run equality test (M2).

### 4.2 Symbol-anchored includes (REQ-S)

- REQ-S1: an ```include-symbol fence names a file and a symbol path; the
  compiler extracts the exact syntactic span via tree-sitter. Extraction is
  invariant to line shifts above the symbol. Oracle: line-shift fixture
  corpus test (lands M3).
- REQ-S2: unknown file or unresolvable symbol fails the build with the
  symbol path in the error. Oracle: negative test (M3).
- REQ-S3: extracted spans carry language and stable anchor IDs; IDs derive
  from symbol path, never line numbers. Oracle: ID stability test (M3).

### 4.3 Proof-carrying documentation (REQ-P)

- REQ-P1: Lean 4 is invoked out-of-process (CLI + `lake env`); no Lean
  runtime is linked into any estate crate, ever. Oracle: docs-proof has
  zero build-dependencies on Lean artifacts (structural; CI check, M4).
- REQ-P2: a ```lean-check fence references a named theorem in the repo's
  Lean package; the compiler runs Lean, and invalid or missing proofs fail
  the build. Oracle: docs-compiler negative test against a fixture package
  (M4).
- REQ-P3: docs-proof defines a compact, versioned certificate record
  (theorem name, source hash, Lean toolchain hash, kernel result) that the
  emitted page embeds as a static badge. Oracle: certificate round-trip
  test (M4).
- REQ-P4: client-side certificate re-verification (tiny wasm checker) is a
  non-goal for M4; recorded as a future phase, not silent absence.
- REQ-P5: frontmatter and body schemas validated by docs-proof are typed
  in Rust first (serde + typed errors); Lean-typed schemas are a future
  phase. Rationale: schema validation must not require a Lean toolchain.

### 4.4 Client search (REQ-Q)

- REQ-Q1: docs-search emits a single versioned binary index (FST +
  roaring bitmaps; optional 1-bit/8-bit quantized vector section behind a
  feature). No JSON index is produced. Oracle: format round-trip + golden
  vector tests (lands M2).
- REQ-Q2: index builds are wasm32-clean (`cargo build --target
  wasm32-unknown-unknown` in CI for docs-search). Oracle: CI job, M2.
- REQ-Q3: acceptance budget on the reference corpus (docs-core README +
  estate index, ~10k pages synthetic): index < 350 KB compressed; query
  p50 < 2 ms, p99 < 10 ms on a mid-tier laptop in-browser.
  No-oracle: corpus generator lands in M2; budget enforced by a benchmark
  test before M2 closes.
- REQ-Q4: search runtime hydrates through the island seam as a
  `docs-island` component; it must not ship a framework runtime.
  Oracle: island test asserts bundle has no framework import (M2).

## 5. Milestones

| Milestone | Slices | Closes when |
|---|---|---|
| M2 | docs-witness, docs-search | REQ-W1..4, REQ-Q1..4 oracles green; crates registered in estate.yml; estate-audit green |
| M3 | docs-symbols | REQ-S1..3 oracles green; docs-compiler scaffold emits include-symbol pages |
| M4 | docs-proof, docs-compiler | REQ-P1..3 oracles green; end-to-end site builds with all four directive kinds |

## 6. Non-goals

- DOM-free/WebGPU prose rendering; prose stays static HTML (fresco-based
  interactive canvas islands remain the GPU path, composed via the island
  seam).
- Lean FFI / FBIP memory bridging (out-of-process only, REQ-P1).
- Live telemetry overlays, SMT contradiction arbitration, TDA coverage
  analysis: future phases, out of scope here.
- Adopting Pagefind for search (superseded by docs-search; docs-core
  README's Wave 2 wording is amended by this SPEC, not silently).

## 7. Estate registration

Each new crate repo: derived from the Omni template; Makefile verb set;
layer declared in `[package.metadata.layers]`; registered in
engineering-standards/estate.yml with `area: docs`, honest status (`support`
until a suite pins it); `estate-audit.py` run after publish. CI tier: B for
L0/L2 kits; docs-witness and any lock-free component graduate to Tier A
(loom/miri) when concurrency primitives land.
