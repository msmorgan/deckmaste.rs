---
needs: []
---
**`deckmaste_core`'s doc comments still describe Idris obligations core no
longer carries.** ~100 references across `crates/deckmaste_core/src/`
("Mirrors Idris `Static : StaticEffect -> Ability`", "rejected by the Idris
re-emit gate", "the Idris `Predicate`", …) are fork residue: they were
accurate when the mirror attached to core, and `idris-mirror-semantics`
reattached it to `deckmaste_semantics` without touching them. Every one of
those sentences now names a relationship core does not have — spec §10:
"`deckmaste_core` carries no proofs, deliberately."

The claims are not merely stale but actively wrong in the load-bearing cases:
a reader who follows "rejected by the Idris re-emit gate" on a core type
concludes an invariant is enforced when nothing enforces it. That exact class
of false claim is what `idris-mirror-semantics` had to resolve for
`EnterRider`; the rest were left because sweeping ~100 prose edits into a
reattachment that had to stay auditable as content-preserving would have
buried the diff.

## Scope

- Sweep `crates/deckmaste_core/src/` for Idris references (`grep -ri idris`).
  Each is one of three things, and the treatment differs:
  - **A claimed proof or gate** ("rejected by the Idris re-emit gate",
    "proven by the Idris re-emit gate") — DELETE the claim, or restate it as
    a pointer to the semantic mirror where the obligation actually lives.
    These are the false ones; do them first.
  - **A shape citation** ("Mirrors Idris `Static : StaticEffect -> Ability`")
    — provenance for why the variant looks as it does. Keep the fact, aim it
    at `deckmaste_semantics` (which core mirrors) rather than at Idris, or
    drop it where the shape is self-evident.
  - **A `Semantics.idr` path reference** — already repointed by
    `idris-mirror-semantics`; verify, don't re-edit.
- The same sentences exist in `deckmaste_semantics` (the fork copied them).
  There they are still TRUE — leave them, and use the semantics copy as the
  wording to point at.
- Not a rename: no code changes, no gate changes. If a sweep item turns out
  to need a real proof rather than a prose fix, book it against
  `idris-mirror-enum-gaps` instead of minting it here.

## Gates

Standard constraints apply. No behavior change: `cargo xtask idris-check
plugins/canon` baseline unchanged, `idris/scripts/build` PASS.
