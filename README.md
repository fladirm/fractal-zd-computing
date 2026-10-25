# Fractal Zero-Divisor Computing

**English · [Čeština](README_CS.md)**

**An addressable, signed information forest from the Cayley–Dickson tower.**  
**Author: Vladimír Bíža · Independent mathematics and computer science · 2026**

> **A graph stores edges. This algebra computes which edges exist, which are inherited, which are new, and what information a binary edge discards.**

## Try the demo first

**[English demo](https://fladirm.github.io/fractal-zd-computing/demo/?lang=en)** · [České demo](https://fladirm.github.io/fractal-zd-computing/demo/?lang=cs) · [Project website](https://fladirm.github.io/fractal-zd-computing/) · [GitHub repository](https://github.com/fladirm/fractal-zd-computing) · [v1.0.0 — four paper PDFs](https://github.com/fladirm/fractal-zd-computing/releases/tag/v1.0.0)

**[Launch HYPERION-ZD (offline, one HTML file)](demo/index.html)** · [English screenshot](assets/demo-desktop-en.png) · [Český screenshot](assets/demo-desktop-cs.png)

Open `demo/index.html` with any modern Chromium-based browser. No Node, npm, backend, account, server, connection or GPU requirement. The HTML includes both languages and the math engine in a single file. Use the **EN / CZ** switch; changing language preserves the current scene, inputs and recovery result.

Four interactive demonstrations: (1) choose two pairs of basis addresses and compute exact signed zero-product incidence; (2) a rank-dependent reconstruction test after simulated data loss, including failure when recovery information is absent; (3) an exact nonassociative associator with zero pairwise product; (4) fourfold inherited edges versus genuinely newborn edges across dimension doublings.

## The research

| Paper | English | Česky |
|---|---|---|
| **A — Mathematics** · zero-divisor predicate, K16 roots, 4-lift, new-edge law, higher π, norm moments | [PDF](papers/en/Paper_A_Mathematics_EN.pdf) · [Markdown](papers/en/Paper_A_Mathematics_EN.md) · [HTML](papers/en/Paper_A_Mathematics_EN.html) | [PDF](papers/cs/Paper_A_Mathematics_CS.pdf) · [Markdown](papers/cs/Paper_A_Mathematics_CS.md) · [HTML](papers/cs/Paper_A_Mathematics_CS.html) |
| **B — Computing** · implicit operators, GPU, persistence, signed probes, testing | [PDF](papers/en/Paper_B_Computing_EN.pdf) · [Markdown](papers/en/Paper_B_Computing_EN.md) · [HTML](papers/en/Paper_B_Computing_EN.html) | [PDF](papers/cs/Paper_B_Computing_CS.pdf) · [Markdown](papers/cs/Paper_B_Computing_CS.md) · [HTML](papers/cs/Paper_B_Computing_CS.html) |

The English editions preserve the mathematics and numbered expressions of the Czech technical source while presenting the exposition in English. Language editions are not certified as word-for-word translations; mathematical reproducibility is the shared contract.

## The core in three lines

1. `epsilon(k,a,b)` computes the Cayley–Dickson multiplication sign of `e_a * e_b` with a bit-recursive rule, not an N×N table.
2. A four-address signed plaquette classifies a selected two-term annihilating relation; its **full key** includes both assessor pairs, signs and orientation.
3. Existing relations lift fourfold while **new relations are born separately**. An inherited component obeys `A_t = A_0 ⊗ J_(2^t)`; the entire forest does not.

For the designated *unsigned two-term assessor atlas*, the census at `K2048` is **533,208,060 zero-product edges**. This is a combinatorial count, not a physically instantiated 533-million-edge cluster and not the number of all continuous zero divisors.

A further witness in `K16` has `x*y=0` but `[x,e₁,y] = −4e₇`. The third-context information is absent from a boolean pairwise adjacency.

## Reproduce

```bash
python3 src/python/verify.py
python3 src/python/benchmark.py
cd src/rust && cargo test && cargo run --release
```

Python uses only the standard library. The Rust reference should be compiled with a local Rust toolchain; a GitHub CI workflow also runs it. The benchmark contrasts explicit and factorized application on **matching structured workloads**. It is not a claim of beating optimized tensor, GraphBLAS or cuSPARSE implementations on arbitrary tasks.

### Engineering scope

The core contribution is **signed algebraic generation + typed higher-order information + genealogical birth**, not just Kronecker compression. Memory-free incidence does not imply memory-free application data. A zero product is not encryption or hardware tenant isolation. A Schur complement recovers a missing component only when enough independent measurement/right-hand-side information survives. Distributed consensus requirements remain independent.

## Publish, cite, reuse

- [English/Czech documentation index](LANGUAGES.md)
- [Publication and release guide](PUBLISHING.md)
- [Manual LinkedIn publication steps](linkedin/MANUAL_PUBLISHING_EN.md)
- [Benchmark methods and limitations](BENCHMARKS.md)
- [Verified publication preflight](release/PREFLIGHT.md)
- [Publication results](PUBLICATION_RESULTS.md)
- [Reproducible source](src/)
- [Suggested citations](CITATION.cff)
- [Bilingual promotional posts and LinkedIn PDF documents](linkedin/)

Copyright © 2026 Vladimír Bíža. **All rights reserved unless a separate license is added.** Publishing on GitHub does not by itself grant a patent, commercial reuse or code redistribution license. The author may elect MIT/Apache-2.0/CC BY licensing later.
