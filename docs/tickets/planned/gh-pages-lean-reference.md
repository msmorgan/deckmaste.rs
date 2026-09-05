---
needs: [ci-lean-gate]
---
**Publish the Lean workbench reference on the pages site.** The `gh-pages`
bookmark is a hand-maintained static landing page (`index.html`, a TUI
screenshot, `CNAME`) that no workflow builds or deploys, and it says nothing
about either workbench. Add a `pages` job to `.github/workflows/ci.yml`,
gated on the `lean` job, that runs `doc-gen4` over the `Semantics` and
`SemanticsProofs` libraries (add the `doc-gen4` dependency to
`lean/lakefile.toml` behind a docs-only facet so `lean/scripts/build` does not
pull it), places the output under `reference/` beside the landing page, and
deploys with `actions/deploy-pages` from the default line. The landing page
gains one link to the reference. Docstrings are the content, so the module
docstrings that still list unported Idris pins by name are rewritten in
present tense first (they are the top of every generated module page).
Decisions already made: the pages bookmark stays hand-authored for the landing
page; generated output is never committed to it.
