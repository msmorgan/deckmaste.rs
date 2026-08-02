---
needs: [macro-author-surface]
---
**Inline target sugar and scope elaboration: the explicit indexed binder
stays the semantic normal form; sugar is accepted input.** Design:
`docs/decisions/authoring-spelling-lowering.md` (§7, §9). Normalization
(desugar + elaboration, authored → authored normal form) lives in
`deckmaste_authoring`; `deckmaste_lowering` invokes it. The elaborator is
the ONLY scope introducer; idiom bodies stay scope-free.

## Scope

- `Target(spec)` sugar at exactly-one Reference positions; `Targets(spec)`
  at Selection positions; no type-directed overloading. Recognized by the
  authoring reader/elaborator BEFORE ordinary macro expansion (variant-
  first dispatch would otherwise consume the ident as an index read).
- **Discriminator** (spec §7): at a Reference/Selection position,
  `Target(<numeral>)`/`Targets(<numeral>)` is an index read; any other
  argument shape is the sugar. Sugar is legal only in card-authored
  provenance text — a macro body may FORWARD sugar through a `Param` hole
  but may never introduce it.
- **Occurrence rule** (spec §7 — no primary/secondary distinction): every
  authored `Target(n)`/`Targets(n)` site counts, including characteristic
  reads; a sugared slot is unreferenceable by construction, so the
  one-site rule is enforced syntactically. Counting is defined over the
  PRE-EXPANSION invocation AST: one caller site is one occurrence no
  matter how many times an idiom body re-reads the `Param` (`Fight` is
  the normative fixture). Implementation commitment: the current reader
  expands during deserialization and re-reads raw argument text per
  `Param`, so this needs a pre-expansion elaboration layer or
  origin-tagged sugar nodes propagated through expansion — price it,
  don't hand-wave it. Traversal order for appended slots is schema order
  (field declaration order, list sequence), so RON named-field reordering
  cannot renumber. Acceptance-time validation and canonical-writer
  contraction are separately labeled directions — this ticket implements
  acceptance only.
- **Naming, owner-settled (2026-08-02)**: the sugar ident is `Announce`
  (`Announce(spec)`, `Announces(spec)`-or-kin for the plural Selection
  form — exact plural spelling is plan latitude). Direction-settled
  family members to shape in the plan: a READ companion
  (`Announced(n)`-style) for previously-announced slots — resolve the
  rename-depth question (canon spells `Target(n)` at ~20 sites: rename
  both grammars with the small canon sweep, or reject the companion and
  keep `Target(n)`; dual spellings are forbidden by decision principle
  2) — and distinctness: `Announce(Distinct([i…], spec))` composes for
  explicit-prefix edges, plus an "other"-flavored form
  (distinct-from-all-earlier announcements, oracle's "any OTHER
  target") for the fully-inline case; explicit indexed `Distinct` stays
  the general mechanism. Arc Trail becomes the other-form fixture;
  Seeds of Strength the non-distinct counterexample (its three
  announcements may legally choose the same creature).
- Mixed scopes via the explicit-prefix rule (explicit slots own `0..E`;
  inline sites append in textual order; authored indices `< E`; validator
  rejects an authored index landing on a generated slot).
- Normalize to the engine's single top-level `Targeted`; hoisting stops at
  genuine announcement boundaries (modal modes, delayed triggers)
  [CR#601.2c].
- `Distinct` canonical well-formedness: sibling indices strictly earlier,
  sorted, deduplicated, edge stored on the later slot.
- Delete the `TargetedDealDamage` differential-baseline fixture (parked by
  `frames-catalog-merge`) once the compositional path covers its cases.

## Gates

Standard constraints apply. For every sugared fixture,
`lower(sugar) == lower(explicit)` byte-for-byte on core; the spec §7 card
list (Bolt, Terminate, Seeds of Strength, Arc Trail, Do or Die, Rabid
Bite — including Rabid Bite's worked verdict: slot 0 explicit, slot 1 may
inline) lands as fixtures; negative fixtures for each eligibility
violation, for a body attempting to introduce sugar, and for an authored
index colliding with a generated slot.
