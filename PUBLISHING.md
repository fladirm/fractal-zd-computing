# Publication workflow

**English** · [Čeština](PUBLISHING_CS.md)

1. `python3 src/python/verify.py`, and `cargo test` under `src/rust/` when Cargo is available.
2. Inspect four PDF documents (`papers/{cs,en}`), open `demo/index.html` **directly** in Chromium, test each interactive scene, and check `docs/` preview.
3. Make one public GitHub repository named `fractal-zd-computing`; commit sources, documents, demo and provenance; publish a tagged release `v1.0.0` attaching the paper PDFs.
4. Enable GitHub Pages **from `main` branch `/docs`** when supported; check HTTPS URL and actually open the demo. `docs/` is self-contained and has links to sibling resources only through correct relative copying.
5. Publish a technical English LinkedIn post to the **confirmed personal profile** (see `linkedin/PROFILE_TARGET.md`) with `linkedin/LINKEDIN_CAROUSEL_EN.pdf`. Only one attached document per post. Include repository/demo links resolved from real GitHub URLs.
6. Optionally publish the Czech companion post with `linkedin/LINKEDIN_CAROUSEL_CS.pdf` later; do not spam both language variants simultaneously.
7. Record actual resulting repo, Pages, release and LinkedIn permalinks in `PUBLICATION_RESULTS.md`. Never claim something was posted without verifying its URL.

A public GitHub repo and LinkedIn post are external publication actions. When Codex encounters authorization/login/2FA/captcha or platform moderation, it should prompt the user to complete it, not circumvent controls. Never expose passwords, tokens or browser storage in the repo. A private preview/draft is acceptable before final publish; the user has separately requested that Codex carry out publication.

Useful official sources:
- https://cli.github.com/manual/gh_repo_create
- https://cli.github.com/manual/gh_release_create
- https://www.linkedin.com/help/linkedin/answer/a518909
- https://www.linkedin.com/help/linkedin/answer/a519826

LinkedIn official document constraints at preparation: PDF accepted, ≤100 MB and ≤300 pages, **one document per post**. Verify them if the platform has changed. This kit uses two separate short 4:5 PDF document decks.
