# hypermedium-program

status: done
updated: 2026-11-13  rev: 16

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
| 6 | docs-proof + docs-symbols: certificate + symbol anchors (L0) | done | docs-symbols@6f19868 (CI green); docs-proof@published 0.1.0, estate.yml 24de917; audit exit 0 | - |
| 6b | docs-compiler: L3 composition CLI (M3/M4 close) | done | crates.io 0.1.0; CI GREEN (run 43048784970: quality incl. coverage gate + lean-oracle with real kernel); estate.yml 9b81fc7 | merge PR #2 (user) |
| 7 | Register crates in estate.yml; estate-audit green | done | audit exit 0 online after each publish (docs-search, docs-witness, docs-symbols, docs-proof all registered) | - |

None: program complete through every opt-in. SPEC CURRENT; gate
posture current; dogfooded (round 15). Opt-ins closed 2026-11-12:
docs-proof 0.1.2 ships REQ-P4 packaging (browser ABI + JS glue +
63.6 KB bundle under the 80 KB budget, wasmtime-driven oracle in CI);
docs-search 0.2.0 ships tf postings with full BM25 (format v2) and the
quantized-vector section behind the `vectors` feature; all six repos
pin rust-kit@4883fca0; all five crate READMEs carry truth tables
(posture ripple). estate-integration suite green at the new pins.
Flagged outside this program: estate-audit reports omni-cap and
omni-ring published-but-unclaimed (repo-registry bookkeeping for the
owner to resolve). Next program, if any: docs-core Wave 2 (math,
sanitization, dev server) per SPEC §3 - STARTED 2026-11-13 as the
Wave 2 phase below (rev 13).

## Blocked

- none

## Log

### Wave 2 phase (opened 2026-11-13, rev 13)

Scope: the three named Wave 2 items only (math, sanitization, dev
server). Versioning, i18n, diagrams (D2), and the Pagefind question
(closed: docs-search is the answer, native-only) stay out.

| # | Task | Status | Evidence | Next |
|---|------|--------|----------|------|
| W2-1 | docs-core math: latex2mathml -> build-time MathML | done | 0.2.0 published (2e1e99c); ENABLE_MATH arms; BadMath fails the render; scanner skips closers inside unbalanced braces (documented) | - |
| W2-2 | docs-core sanitization: ammonia allowlist | done | sanitize() + render_untrusted(); islands stripped PRE-render; scripts die, math survives (tests/sanitize_math.rs) | - |
| W2-3 | dev server: docs-compiler `serve` (NOT docs-core) | done | 0.2.0 published (2704a93); compile-on-change, traversal guard, fail-loud per page; VERIFIED ON DEVICE via adb reverse: MathML renders in mobile Chrome, zero client JS | - |
| SEC-1 | RUSTSEC-2026-0533 (ammonia MathML/SVG mXSS) | done | docs-core 0.2.1 (ammonia 4.2.5) published; docs-compiler graph updated (b8ffc13); estate =0.2.1 (3ee8573, suite 7/7); advisories gate green - caught by docs-compiler's deny job the day SEO shipped, exactly the gate working | - |
| W2-5 | Diagrams: d2 fences -> static SVG | done | docs-compiler 0.3.1 (0.3.0 yanked): out-of-process d2 v0.9.0, body-carrying Directive, diagram-oracle CI job; blank-line fix (raw-HTML block integrity) caught by on-device rendering | - |
| W2-6 | SEO emission: full documents + head meta + JSON-LD + llms.txt + sitemap + robots (estate-seo floor) | done | docs-compiler 0.4.0 (cf761c3): front-matter inputs with h1/paragraph fallbacks, site.toml site config, recorded AI-crawler policy, attr-escaping (quote breakout found by oracle); verified on phone; round 18 d990f24, suite 7/7 | CI watch |
| W2-4 | dogfood round 16 + pins + audit | done | estate-integration 33211b7: docs-core =0.2.0 (ui_semantics green, no $-prose regressions), docs-compiler =0.2.0; docs_program 5/5 with the Wave 2 interop test; audit: zero docs drift (omni-* errors remain, owner bookkeeping). CI green everywhere at tip after three graph fixes: docs-core had .cargo/config.toml committed (untracked, 8c9b813 - same fleet lesson as estate-integration), vet exemptions for the two new dep trees, and docs-compiler deny.toml admits MPL-2.0 (ammonia/cssparser) + CC0-1.0 (notify) - license-policy delta recorded here | closed |

- 2026-11-12 rev 12: all opt-ins shipped. docs-search 0.2.0
  (1edadad): format v2 (DSIDX2) with (doc, tf) postings - full BM25
  replaces the unit-tf placeholder, roaring leaves the floor - plus the
  REQ-Q1 vectors feature (fixed max-abs i8 quantization, cosine
  query_vector, VectorsUnsupported when the feature is off). docs-proof
  0.1.2 (b328fd7): REQ-P4 packaging - wasm32 C ABI (dps_alloc/free/
  verify + error slot, DPS_* codes), CHECKER_JS glue, committed bundle
  at 63,589 bytes raw (budget 80 KB; the earlier 4 KB figure predated
  serde_json + sha2 reaching the artifact), CI checker-bundle job
  rebuilds/gates/diffs and drives the module through the glue's exact
  call sequence with wasmtime. Honest note: panic=abort + opt-z in the
  cdylib profile (consumer profiles override). Pin sweep: all six
  repos -> rust-kit@4883fca0 (docs-witness 22080ca, docs-symbols
  d0ecfe5, docs-core 735a3c0, docs-compiler 230e37b). Posture ripple:
  Where-the-truth-lives tables in all five crate READMEs.
  estate-integration 21bce99: docs-search =0.2.0, docs-proof =0.1.2,
  suite green. Audit: zero docs drift; omni-cap/omni-ring unclaimed
  errors are outside this program. Follow-ups, same day: rust-kit pins
  must be FULL 40-char SHAs - a 12-char short SHA fails workflow parse
  at 0s in every repo (fixed everywhere, 3470db8/a54f000/a831379/
  788a7fa/3ef2834/4d836e4). estate-integration msrv, red since Nov 7,
  truthed to the lockfile floor: rust-version 1.95 (egui/emath/epaint
  0.37 demand it; wasmtime line 1.86, webhookkit 1.87, zip 1.88 beneath)
  - msrv + CI green at 2dd5272. docs-proof's checker-bundle oracle runs
  against BOTH the CI-built module and the committed asset: rustc
  codegen is not byte-reproducible, so freshness-as-diff is impossible
  (daf0169).

- 2026-11-12 rev 11: docs-proof 0.1.1 shipped (REQ-P4 core): `verify_badge` +
  `BadgeError` validate badge JSON structurally client-side; serde_json
  joins the runtime floor; wasm32 cdylib measures 4 KB raw against the
  80 KB budget; wasm: true in CI (run 43232670709 green). Drift caught
  by the audit: estate-integration bumped to docs-proof =0.1.1
  (f9869a1), docs_program suite green at the new version. Fleet lesson
  adopted: CARGO_TARGET_DIR never points into /tmp (the janitor reaps
  1h-idle target dirs - the likely cause of the vanished build trees on
  2026-11-10); the shared ~/.cache/cargo-target is the sanctioned
  target.
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
- 2026-11-11 rev 5: M3 crate shipped. docs-symbols 0.1.0 (ea8cdab, then
  clippy-fix amend 6f19868): tree-sitter 0.26 + rust grammar 0.24;
  ordered-name resolution with impl precedence for deeper paths and all
  impl bodies (inherent + trait) tried in document order; fnv1a64 anchor
  IDs; strict parsing. Findings: tree-sitter 0.26 Point has no byte
  field (use start_byte); preview1-style wasm build of grammar C needs
  a libc sysroot - crate is native-only by design, CI wasm: false
  recorded. Publish + estate.yml (docs-symbols entry) + audit exit 0
  online; canon e9ce960. All three program crates are now on crates.io.
  M4 next: docs-proof (OmniLean-template base) + docs-compiler L3
  scaffold.
- 2026-11-11 rev 6: docs-symbols CI green (run 42955962204). docs-proof
  0.1.0 shipped the same day: certificate record (theorem, sha256
  source hash, toolchain, kernel result), serde round-trip + badge JSON
  differentially tested against serde, total verifier (StaleSource /
  StaleToolchain / Version / NotVerified). Zero Lean linkage in the
  tree - REQ-P1's structural half; the crate never creates claims, only
  records and checks them (SECURITY.md trust model). Published +
  registered; audit exit 0 online (canon 24de917, on top of the user's
  concurrent docs-search registration). Program state: all four
  capability crates (search, witness, symbols, proof) on crates.io +
  registered; M4 remainder is docs-compiler (L3) and the Lean fixture
  package.
- 2026-11-11 rev 8: PROGRAM COMPLETE. docs-compiler CI green after two
  CI-lessons fixed forward: (1) GitHub startup-fails a reusable-workflow
  call with an input the pinned rev lacks - coverage-ignore-regex
  needed the newer rust-kit rev (898aed5); (2) llvm-cov matches
  --ignore-filename-regex against absolute paths, so the process-surface
  regex must be a suffix match. Final shape: main.rs + lean.rs declared
  process surfaces (ADR-0005), coverage ~89%, lean-oracle job runs the
  real kernel e2e (all four directive kinds) green. Program totals: 6
  crates published + registered (core, search, witness, symbols, proof,
  compiler), estate-audit exit 0, all CI green. Outstanding user calls:
  merge PR #2; SPEC stamp; consumer-suite decision.
- 2026-11-11 rev 7: M4 CLOSED (pending CI). docs-compiler 0.1.0 shipped:
  fence scanner + resolution through all three capability crates +
  docs-core render; CLI `docs-compiler <input> [-o] [--root]`. Real
  lean oracle (REQ-P2): local lean 4.35.0 against pinned fixture
  package - positive, kernel-rejection (False := trivial), and sorry
  cases all typed. Findings: Lean 4.35 emits diagnostics on stdout with
  backtick wording (sorry guard scans combined output); CommonMark
  forbids backticks in fence info strings (guard added after the
  e2e/fence tests caught document-swallowing). lean_oracle tests run
  ignored-by-default with a dedicated elan CI job (-- --ignored), so
  the shared quality gate stays lean-free. Published + registered
  (estate.yml 9b81fc7, audit exit 0). Five crates total: core, search,
  witness, symbols, proof, compiler.
- 2026-11-11 rev 4: M2 CLOSED. docs-witness CI green (run 42832126074).
  Both crates published to crates.io (docs-search 0.1.0, docs-witness
  0.1.0). estate.yml updated (docs-search crates list added to the
  pre-existing entry; docs-witness entry added) + --sync; estate-audit
  online: exit 0, 0 errors, 0 drift; both crates in coverage-debt class
  until docs-compiler composes them (M4). Canon pushed ca125d7. M3 next:
  docs-symbols (tree-sitter).
