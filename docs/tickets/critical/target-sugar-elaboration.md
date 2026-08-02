---
needs: [macro-author-surface]
---
**Inline target sugar and scope elaboration in `deckmaste_lowering`: the
explicit indexed binder stays the semantic normal form; sugar is accepted
input.** Design:
`docs/decisions/authoring-spelling-lowering.md`
(§7, §9). The elaborator is the ONLY scope introducer; idiom bodies stay
scope-free.

## Scope

- `Target(spec)` sugar at exactly-one Reference positions; `Targets(spec)`
  at Selection positions; no type-directed overloading. Recognized by the
  authoring reader/elaborator BEFORE ordinary macro expansion (variant-
  first dispatch would otherwise consume the ident as an index read).
- Eligibility: exactly one primary referential occurrence in the
  unexpanded term, no `Distinct` edge on the slot — counting neither
  English mentions nor expanded-core reads.
- Mixed scopes via the explicit-prefix rule (explicit slots own `0..E`;
  inline sites append in textual order; authored indices `< E`;
  cross-constrained components promote to explicit; validator rejects an
  authored index landing on a generated slot).
- Normalize to the engine's single top-level `Targeted`; hoisting stops at
  genuine announcement boundaries (modal modes, delayed triggers)
  [CR#601.2c].
- `Distinct` canonical well-formedness: sibling indices strictly earlier,
  sorted, deduplicated, edge stored on the later slot.

## Gates

Standard constraints apply. For every sugared fixture,
`lower(sugar) == lower(explicit)` byte-for-byte on core; the spec §7 card
list (Bolt, Terminate, Seeds of Strength, Arc Trail, Do or Die, Rabid
Bite) lands as fixtures; negative fixtures for each eligibility violation
and for an authored index colliding with a generated slot.
