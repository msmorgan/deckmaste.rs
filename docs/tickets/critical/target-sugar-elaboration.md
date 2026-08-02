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
  one-site rule is enforced syntactically. Acceptance-time validation and
  canonical-writer contraction are separately labeled directions — this
  ticket implements acceptance only.
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
