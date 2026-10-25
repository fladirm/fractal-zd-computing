# Publication preflight — 25 October 2026

**English** · [Čeština](PREFLIGHT_CS.md)

Author: **Vladimír Bíža**. Manuscript equations, proof structure and attribution are unchanged.

## Reproduction

- `python3 src/python/verify.py`: PASS. K16 = 84/315 candidate pairs, K32 = 1260, K64 = 13020; signed witness in both orientations; both associator contexts; all four lifts in both orientations; exact higher-π fractions.
- `python3 src/python/benchmark.py`: PASS. Explicit and factorized operators return identical integer outputs for m = 1, 2, 4, 8, 16, 32, 64, 128.
- `cargo test --manifest-path src/rust/Cargo.toml`: PASS, five unit tests. The signed plaquette is checked against actual signed products for every candidate at K16/K32/K64, along with the witness, lifts, root probes, moments, invalid-address rejection and explicit/factorized application with and without a local delta.
- `cargo run --release --manifest-path src/rust/Cargo.toml`: PASS. All five probes at m = 32 have maximum factorized/explicit deviation 0.
- K2048: `(2048−2)(2048−4)(2048−8)/16 = 533,208,060`, checked in Python, Rust and the browser. This evaluates the manuscript census formula for the defined two-term assessor atlas; it is not exhaustive enumeration of all K2048 edges or continuous zero divisors.

Environment: Python 3.14.4; rustc 1.96.0; Cargo 1.96.0; Linux 7.1.3. These are correctness runs, not a competitive hardware benchmark. The supplied benchmark is educational; CSR/GraphBLAS and optimized matrix-free comparisons remain separate work as described in `BENCHMARKS.md`.

## Offline browser verification

Chromium opened `demo/index.html` directly with the `file:` protocol. Network was explicitly set offline during interaction. No resource requests or page errors occurred. All nine built-in checks and 24 interactive checks passed; see [machine-readable results](offline-demo-checks.json).

1. Address oracle: K2048 start, all five probes at K16, invalid equal addresses, new addresses, actual 3,000-query timing and reset to χ = 1025.
2. Recovery: exact recovery at 70% requested loss (10 survivors) and at the 75% boundary (8 survivors); honest failure at 80% and 85% requested loss (6 and 5 survivors). New data also reconstructs when rank suffices. Loss is rounded to an integer number of 32 storage symbols. This is conventional erasure coding over GF(257).
3. Associator: `XY = YX = 0`; `[X,e1,Y] = −4e7`; `[X,e8,Y] = 0`.
4. Birth: K16 = 84; K32 = 1260 total / 924 newborn; K2048 = 533,208,060 total.

Desktop (1440×1080) and mobile (390×844) checked; no horizontal overflow. Screenshots: [desktop](../assets/demo-desktop.png), [mobile](../assets/demo-mobile.png), [insufficient recovery rank](../assets/recovery-insufficient.png). The demo and Pages copy are byte-identical, dependency-free, and require no server, CDN, account or API key.

## Documents and packaging

All pages of four paper PDFs and both six-page LinkedIn PDFs were rendered and inspected. Author attribution is present. All 30 manuscript/format/deck files match the original supplied SHA-256 checksums. Paper A: EN 7 pages / CS 8 pages; Paper B: EN 5 pages / CS 5 pages. The LinkedIn decks are each approximately 1.48 MB, with uniform 720×900 pt pages. Their PDF metadata says `anonymous`, but visible author attribution is correctly Vladimír Bíža; original document bytes were preserved.

Pages manuscript copies are identical to the corresponding source documents. Local relative links were checked. HTML papers contain no external resource references. Markdown sources are served unchanged using `docs/.nojekyll`.

## Discrepancies resolved or documented

- Fixed Rust syntax `weight:.75` to `weight:0.75`, which previously prevented compilation.
- Removed the stale Rust TOE package/banner branding and the unauthorized MIT metadata. The package is `fractal-zd-computing` v1.0.0 with the correct author and registry publication disabled; all rights remain reserved.
- Required valid disjoint assessor addresses and equal XOR colors before the Rust/browser plaquette predicate, matching Paper A.
- Fixed reset of the K2048 demo to χ = 1025 and the first probe. Replaced the misleading scene label `0 × 0` with `XY = 0`; clarified the selected-atlas count.
- Extended mathematical regression coverage without altering manuscript statements.
- The original Czech Paper B describes pre-packaging paths (`code/verify.py`, `code/benchmark.py`, `code/src/main.rs`) and the then-unavailable Rust compiler. Current runnable paths are `src/python/verify.py`, `src/python/benchmark.py` and `src/rust/`; current compilation results are above. This historical packaging prose was retained with the original PDF/Markdown pair.

No peer-review status, arbitrary-workload speed superiority, encryption, tenant isolation or consensus claim is inferred from these checks.

## Public deployment verification

GitHub Pages at https://fladirm.github.io/fractal-zd-computing/ uses `main` / `docs` and branch publishing. Landing, demo and all 12 paper-format URLs (PDF/HTML/Markdown, EN/CS) returned HTTP 200. Demo and all paper-format responses matched local SHA-256 checksums. The live demo passed the same 24 interactive checks and nine internal checks: [live results](live-demo-checks.json).

Initial GitHub [Python/Rust CI](https://github.com/fladirm/fractal-zd-computing/actions/runs/40158855273) and [Pages deployment](https://github.com/fladirm/fractal-zd-computing/actions/runs/40158863207) succeeded. A harmless request for the account-root favicon returned 404 on the first landing-page load; an inline empty favicon now prevents that request. Final URLs and subsequent deployment status are maintained in `PUBLICATION_RESULTS.md`.

## Bilingual interface update

Both the project website and the single-file offline demo now have an EN/CZ switch. All four scenes, controls, status/error messages, explanatory text, comparisons and canvas labels are translated. The default is English, with explicit `?lang=en` / `?lang=cs` links and a saved preference. Language changes preserve the scene, inputs, selected probe and recovery outcome. Python verification, the educational benchmark and all five Rust tests pass. **62 bilingual browser checks pass**, including mobile layouts, complete English scene text, state preservation and offline navigation. See [bilingual check results](bilingual-demo-checks.json) and the [language index](../LANGUAGES.md).
