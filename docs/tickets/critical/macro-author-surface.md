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
  identically). Reachable = variants of registry kinds reachable from the
  restricted-container root types via fields/flatten/embed, excluding
  name-erasing loader-tag kinds (spec §5). Scaffolded once by the
  generator, then HAND-OWNED; a coverage gate (variant ↔ def, arities
  verified) replaces wipe-first regeneration. Registration kinds computed
  from the flatten graph; scaffolds mirror arities AND existing
  constructor defaults (byte-identical canon re-parse is the invariant —
  default REMOVAL stays `core-remove-default-args`). Coverage is keyed
  per `(defining type, variant, dispatch kind, signature)` row. At
  `remembers_expansion` kinds the wrappers add invocation provenance to
  previously-bare spellings — "zero behavior change" is measured at
  stored-byte round-trip and lowered core (spec §5).
- **Closed reachability inventory** (spec §4): every reachable
  `(kind, variant)` pair classified identity-covered / native-calculus /
  native-atom (plain enums outside the registry — `Cmp`, phase/step
  enums, `FaceLayout`; canon spells `Eq` and `Beginning(Upkeep)` today);
  suppression applies at registered kinds only, and nothing may be
  unclassified. The identity exemption's trust channel is the COMPILED
  registry (spec §6) — generated Rust table, marker never serialized.
- Suppression must cover BOTH native-candidacy consults: the
  `EnumIntercept` branch AND the untagged-embed pre-scan's own
  `variants.contains` (spec §4).
- **Stragglers**: register `Color` and `Supertype` (and the `Card` root
  scaffold) so the ban is uniform; the closed `Type` enum and counter
  names already conform.
- Negative fixtures: a bare-primitive card fails with a useful error; the
  per-container restriction matrix gets a fixture per container kind.

## Gates

Standard constraints apply. Canon + workspace suites green; idris-check no
regressions; zero behavior change to expansion of existing canon; coverage
gate green; negative fixtures in place.
