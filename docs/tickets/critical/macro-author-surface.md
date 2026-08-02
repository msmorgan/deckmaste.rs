---
needs: [plugin-repoint]
---
**Make the macro layer the only author-facing vocabulary in card/token
containers: the parse-position ban, identity macros, and straggler
registration — Stage 2 of the authoring program.** Design (the settled
record; supersedes this ticket's pre-program draft):
`docs/decisions/authoring-spelling-lowering.md`
(§4-§5). Lands entirely in `deckmaste_authoring`.

## Scope

- **The ban**: restricted reads at the card/token (and graduation)
  entries, by native-variant-candidacy SUPPRESSION at the single ident
  dispatch point — not match-then-reject — so mirrored spellings route to
  identity macros, bare-nullary sugar keeps parsing, and no value tree is
  ever built for a banned spelling. The restriction bit follows textual
  provenance (recorded at argument capture, restored on re-read); body
  re-reads are exempt however reached. Restriction is per container:
  cards/tokens restricted; `rules/` tables and macro bodies free.
- **Identity macros**: one def per authoring-reachable variant under the
  variant's own name (mirror-by-default — canon re-parses byte-
  identically). Scaffolded once by the generator, then HAND-OWNED; a
  coverage gate (variant ↔ def, arities verified) replaces wipe-first
  regeneration. Registration kinds computed from the flatten graph.
- **Stragglers**: register `Color` and `Supertype` (and the `Card` root
  scaffold) so the ban is uniform; the closed `Type` enum and counter
  names already conform.
- Negative fixtures: a bare-primitive card fails with a useful error; the
  per-container restriction matrix gets a fixture per container kind.

## Gates

Standard constraints apply. Canon + workspace suites green; idris-check no
regressions; zero behavior change to expansion of existing canon; coverage
gate green; negative fixtures in place.
