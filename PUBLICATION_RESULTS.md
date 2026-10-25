# Publication results

Author: **Vladimír Bíža**. Verification timestamp: **2026-10-25T03:42:43+00:00**.

**GitHub publication is complete. English LinkedIn publication is blocked on authentication. The Czech follow-up is a complete repository draft.**

| Deliverable | Status | Verified actual URL |
|---|---|---|
| GitHub account | VERIFIED | https://github.com/fladirm |
| Public repository | PUBLISHED | https://github.com/fladirm/fractal-zd-computing |
| GitHub release v1.0.0 | PUBLISHED; four original paper PDFs | https://github.com/fladirm/fractal-zd-computing/releases/tag/v1.0.0 |
| GitHub Pages landing | LIVE; HTTP 200 | https://fladirm.github.io/fractal-zd-computing/ |
| Interactive demo | LIVE; 24 interaction checks passed | https://fladirm.github.io/fractal-zd-computing/demo/ |
| English LinkedIn post | **NOT PUBLISHED** | No public post permalink exists; authentication blocked |
| English prepared text | READY | https://github.com/fladirm/fractal-zd-computing/blob/main/linkedin/POST_EN_READY.txt |
| English document | READY | https://github.com/fladirm/fractal-zd-computing/blob/main/linkedin/LINKEDIN_CAROUSEL_EN.pdf |
| Czech follow-up draft | READY DRAFT / **NOT PUBLISHED** | https://github.com/fladirm/fractal-zd-computing/blob/main/linkedin/POST_CS_READY.txt |
| Czech document | READY | https://github.com/fladirm/fractal-zd-computing/blob/main/linkedin/LINKEDIN_CAROUSEL_CS.pdf |

## Release assets

Published at **2026-10-25T03:40:12Z**. All four public downloads returned HTTP 200 and their SHA-256 checksums matched the original supplied PDFs; see [asset verification](release/published-assets.json).

| PDF | Bytes | Verified public download |
|---|---:|---|
| Paper_A_Mathematics_CS.pdf | 99109 | [Download](https://github.com/fladirm/fractal-zd-computing/releases/download/v1.0.0/Paper_A_Mathematics_CS.pdf) |
| Paper_A_Mathematics_EN.pdf | 86068 | [Download](https://github.com/fladirm/fractal-zd-computing/releases/download/v1.0.0/Paper_A_Mathematics_EN.pdf) |
| Paper_B_Computing_CS.pdf | 106312 | [Download](https://github.com/fladirm/fractal-zd-computing/releases/download/v1.0.0/Paper_B_Computing_CS.pdf) |
| Paper_B_Computing_EN.pdf | 95420 | [Download](https://github.com/fladirm/fractal-zd-computing/releases/download/v1.0.0/Paper_B_Computing_EN.pdf) |

Release tag `v1.0.0` points to commit **7c901268180e8a20e02ab0a7daa7cb7b1cf0677a**. The subsequent publication-results commit on `main` adds the final verification records; it does not change the release assets or mathematical implementation.

## Verification and repository state

- `gh auth status` and the authenticated user API verified **fladirm**. The repository is public, its default branch is **main**, and `origin` is `git@github.com:fladirm/fractal-zd-computing.git`. Local `main` tracks `origin/main`; `v1.0.0` was fetched and its remote commit verified.
- GitHub Pages is configured for **main /docs**, branch publishing (`legacy`), public visibility, and HTTPS enforcement. The final landing, demo and all 12 paper-format URLs returned HTTP 200 and matched the committed source. [HTTP and checksum results](release/pages-http-checks.json).
- Python oracle and educational explicit/factorized benchmark: **PASS**. Rust: **5/5 tests PASS**. Optimized Rust demo: all five probes agree with the explicit reference, maximum deviation 0.
- [Release-commit GitHub algebraic validation](https://github.com/fladirm/fractal-zd-computing/actions/runs/40159041343): **SUCCESS**, both Python and Rust jobs. [Release-commit Pages deployment](https://github.com/fladirm/fractal-zd-computing/actions/runs/40159041071): **SUCCESS**. [Current workflow history](https://github.com/fladirm/fractal-zd-computing/actions/workflows/verify.yml).
- Offline Chromium opened the HTML directly with `file:` and network disabled: **9/9 internal checks + 24/24 interactive checks PASS**, no external resources. Live HTTPS demo: **24/24 interaction checks PASS**. Desktop and mobile layouts were checked. [Offline results](release/offline-demo-checks.json), [live results](release/live-demo-checks.json), [screenshots and full preflight](release/PREFLIGHT.md).
- Exact K2048 formula result: **533,208,060** edges in the defined two-term assessor atlas. K16/K32/K64 candidate censuses and actual signed products were checked independently. The K2048 check evaluates the manuscript formula; it does not enumerate all 533 million edges or continuous zero divisors.
- Recovery succeeds only with sufficient surviving redundancy. At 70% requested loss ten symbols survive; at 85% only five survive and recovery correctly fails. The threshold is eight independent observations.
- Four paper PDFs and both LinkedIn documents were visually inspected. Original manuscript equations, proof structure, attribution and all manuscript/format/deck file hashes were preserved. Resolved issues include Rust compilation syntax, stale package/banner/license metadata, plaquette input validation and demo reset. Historical Czech Paper B packaging paths are explained in the [preflight report](release/PREFLIGHT.md).
- No secrets, login e-mail, tokens, cookies, authentication URLs or private profile data were staged. Browser logs, local build output and transient test artifacts are ignored. `SHA256SUMS.txt` covers the published tracked files except itself.
- License: **RIGHTS RESERVED**. No open-source or unrestricted-reuse license was granted. Author: **Vladimír Bíža**.

## LinkedIn blocker and continuation

Confirmed intended personal profile: https://www.linkedin.com/in/vladim%C3%ADr-b%C3%AD%C5%BEa-84bb11102/

The browser was not authenticated. The requested Google sign-in route was opened, but Google rejected that browser/application with an insufficient-security message. Manual sign-in in a supported browser and access to the authenticated session were requested. The available session still shows the login/rejection pages. No authenticated author identity could be verified, no post was submitted, and there is **no English LinkedIn post permalink** to report.

The English text and single PDF are ready in [publication notes](linkedin/PUBLISHING_NOTES.md). Use the exact document title **533 Million Zero-Product Relations — Fractal ZD Computing**. After authentication, verify the exact personal profile, publish with **Anyone** visibility, then capture and verify the resulting permalink here.

The Czech companion is a ready-to-paste repository draft for a later document publication, with real source/demo links and its own PDF. It is **not** an internal LinkedIn draft and **not** a scheduled or published post. No near-duplicate Czech post was sent immediately after the English preparation.
