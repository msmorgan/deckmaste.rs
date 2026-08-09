---
needs: []
---

# Publish the oracle-typing experiment as a paper

Write up the binder-unification-probe experiment — the dependently-typed
oracle-English workbench where well-typed = editorially printable — for an
academic venue. Speculative until that claim integrates and the machinery
stabilizes; actionable after.

Decisions and framing already settled (2026-08-08):

- **Thesis:** surface-faithful dependent typing scales to an industrial
  controlled natural language, and the type layer checks *discourse*
  well-formedness (referent introduction and accessibility) — the layer
  oracle templating actually regulates. Poster example: a disjunctive target
  ("target creature an opponent controls or land you control") followed by
  "That player" refuses to typecheck — no unique player referent survives the
  disjunction, the classic DRT accessibility barrier, enforced by the
  compiler.
- **Secondary result:** mechanization exposes CR under-determination (no rule
  defines what a block-causing effect does; "could block" exists only as a
  Gatherer ruling) — pairs with the mtg-rules UD registry practice.
- **Venue ladder:** arXiv preprint first (cs first-submission endorsement
  needed), then TyDe (ICFP workshop, Idris home turf) or an ICFP functional
  pearl; the CNL workshop fits the corpus framing. The witness/pin bench is
  the artifact-evaluation artifact — reviewers can build it and watch the
  refusals fail with pinned errors.
- **Author status:** independent author is viable — double-blind review,
  "Independent Researcher" affiliation, precedent in this exact niche (the
  MTG Turing-completeness paper's lead author). AI assistance disclosed per
  venue policy; authorship is the user's.
- **Write-up debts:** related-work positioning against Ranta's
  type-theoretical grammar / GF and the CNL literature; evaluation framed via
  the vintage-gauntlet primitive-inventory census rather than a flat coverage
  percentage.
