# Publication results

Author: **Vladimír Bíža**. Verified at **2026-10-25T03:39:50+00:00**.

| Deliverable | Status | Actual URL / artifact |
|---|---|---|
| GitHub account | VERIFIED | https://github.com/fladirm |
| Public repository | PUBLISHED | https://github.com/fladirm/fractal-zd-computing |
| GitHub Pages landing | LIVE; HTTP 200 | https://fladirm.github.io/fractal-zd-computing/ |
| Interactive demo | LIVE; 24 interaction checks passed | https://fladirm.github.io/fractal-zd-computing/demo/ |
| GitHub release v1.0.0 | NOT PUBLISHED — next workflow step | Four original paper PDFs prepared |
| English LinkedIn document post | NOT PUBLISHED — authentication blocked | [Ready text](linkedin/POST_EN_READY.txt) + [EN document](linkedin/LINKEDIN_CAROUSEL_EN.pdf) |
| Czech follow-up | READY DRAFT / NOT PUBLISHED | [Ready text](linkedin/POST_CS_READY.txt) + [CS document](linkedin/LINKEDIN_CAROUSEL_CS.pdf) |

## Verification

- `gh auth status` and the authenticated user API verified **fladirm**. The repository is public, its default branch is `main`, and `origin` is `git@github.com:fladirm/fractal-zd-computing.git`.
- Pages is configured for **main /docs** using branch publishing. Landing, demo and all 12 paper-format URLs returned HTTP 200. Paper PDFs, HTML and Markdown match the source files.
- Python oracle and educational benchmark: PASS. Rust: **5/5 tests PASS**. [GitHub algebraic validation](https://github.com/fladirm/fractal-zd-computing/actions/runs/40158855273): SUCCESS.
- Offline Chromium `file:` demo with network disabled: **9/9 internal checks + 24/24 interactive checks PASS**, zero network resources. Live HTTPS demo: **24/24 PASS**.
- Exact K2048 formula result: **533,208,060** edges in the defined two-term assessor atlas. Recovery succeeds with sufficient surviving redundancy and correctly fails with fewer than eight independent symbols.
- Four paper PDFs and both LinkedIn PDFs inspected; all original manuscript/format/deck checksums preserved. Details, screenshots and resolved discrepancies: [preflight report](release/PREFLIGHT.md).
- License: **RIGHTS RESERVED**. The stale MIT field was removed from Cargo metadata; no open-source license was granted.

## LinkedIn blocker and continuation

Confirmed intended personal profile: https://www.linkedin.com/in/vladim%C3%ADr-b%C3%AD%C5%BEa-84bb11102/

The browser was not authenticated. The requested Google sign-in route was opened, but Google rejected that browser/application with an insufficient-security message. Manual sign-in in a supported browser and access to its authenticated session were requested. No credentials, cookies, authentication URLs or private profile data are included here. No authenticated author identity could be verified, no post was submitted, and there is **no English LinkedIn post permalink** to report.

The English text, single PDF and exact document title are ready in [publication notes](linkedin/PUBLISHING_NOTES.md). After authentication, verify the exact personal profile, publish with **Anyone** visibility, then capture and verify the resulting permalink. The Czech companion is a complete repository draft for a later publication, not an internal LinkedIn draft or a scheduled post.
