---
needs: [authoring-crate-fork]
---
**Rename `deckmaste_frames` → `deckmaste_spelling`: the crate responsible
for the authored ⇄ English relation.** Design:
`docs/decisions/authoring-spelling-lowering.md`
(§1, §8). "Frames" survives as what the lexicon's entries are called — the
mechanism inside spelling.

## Scope

- Cargo rename + imports/docs sweep (mechanical).
- Kind/type references retarget to `deckmaste_authoring` (the relation
  pairs English with the authored grammar; core must not appear in this
  crate's dependency graph). Delimitation vs `plugin-repoint`: THIS ticket
  owns the crate's own internal type table (guard.rs and kin); the
  loader-supplied read path is the repoint's (unordered pair; each owns
  its side).
- xtask macro-tooling imports follow.

## Gates

Standard constraints apply. `cargo tree` proves `deckmaste_spelling` has
no path to `deckmaste_core`; existing frames/unify/render test suites
green unchanged.
