# Publication results

**English** · [Čeština](PUBLICATION_RESULTS_CS.md)

Author: **Vladimír Bíža**. Verification timestamp: **2026-10-25T04:12:12.381Z**.

**GitHub publication is complete. The author reports having published on LinkedIn manually and requested no further LinkedIn changes. Its permalink has not been supplied or independently verified.**

| Deliverable | Status | Verified actual URL |
|---|---|---|
| GitHub account | VERIFIED | https://github.com/fladirm |
| Public repository | PUBLISHED | https://github.com/fladirm/fractal-zd-computing |
| GitHub release v1.0.0 | PUBLISHED; four original paper PDFs | https://github.com/fladirm/fractal-zd-computing/releases/tag/v1.0.0 |
| GitHub Pages landing | LIVE; HTTP 200 | https://fladirm.github.io/fractal-zd-computing/ |
| Interactive demo | LIVE; 24 interaction checks passed | https://fladirm.github.io/fractal-zd-computing/demo/ |
| English LinkedIn post | AUTHOR-REPORTED PUBLISHED; not independently verified | Permalink not supplied; no further action requested |
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

## EN/CZ interface and documentation update

The single-file offline demo and Pages landing now include an **EN / CZ** switch, with explicit language links:

- English demo: https://fladirm.github.io/fractal-zd-computing/demo/?lang=en
- Czech demo: https://fladirm.github.io/fractal-zd-computing/demo/?lang=cs
- English landing: https://fladirm.github.io/fractal-zd-computing/?lang=en
- Czech landing: https://fladirm.github.io/fractal-zd-computing/?lang=cs
- Bilingual reader documentation and screenshots: [language index](LANGUAGES.md).

All four scenes, controls, calculations, explanations, status messages and canvas labels are available in both languages. Language changes preserve inputs, the current scene, the selected probe and the recovery outcome. Both languages remain embedded in the offline file, with no external dependency. **62 offline browser checks passed**, with no JavaScript errors or network resources; see [bilingual checks](release/bilingual-demo-checks.json). The Python oracle, educational benchmark and five Rust tests also passed after the interface changes.

Bilingual deployment verification: **LIVE / PASS**, checked at **2026-10-25T04:12:12.381Z**. All four EN/CZ landing/demo URLs returned HTTP 200 and matched the deployed source. **23 live interaction checks passed** with no page errors. [HTTP evidence](release/bilingual-pages-http-checks.json), [live interaction evidence](release/bilingual-live-checks.json). [Bilingual update CI](https://github.com/fladirm/fractal-zd-computing/actions/runs/40160937512) and [Pages deployment](https://github.com/fladirm/fractal-zd-computing/actions/runs/40160937130) both succeeded. The existing `v1.0.0` tag and its original four PDFs remain unchanged; the interface update is published from `main`.

## LinkedIn status reported by the author

Confirmed intended personal profile: https://www.linkedin.com/in/vladim%C3%ADr-b%C3%AD%C5%BEa-84bb11102/

Automated publication was not completed. The author subsequently reported that the post was already published manually and explicitly requested leaving LinkedIn as it is. No further LinkedIn access, edits or publication attempts are part of this update. The public post permalink has not been provided, so the reported publication is not independently verified and no URL is invented here.

The Czech companion remains a repository draft; Codex did not publish or schedule it. Prepared files and bilingual manual instructions are retained for reference.
