---
needs: []
---
**`bracketed` becomes one polymorphic identity template.** Owner ruling, 2026-10-23: square
brackets in rules text are a fact about the wording, and there is nothing anaphoric to prove about
cleave.

- `semantic_template bracketed {α : Type} (x : α) : α := x` replaces the Core constructors
  `Predicate.bracketed` and `Action.bracketed`, their checker arms, the name-table overload and the
  two per-type templates.
- The bracket survives in the authoring tree: `Authoring.inspect` sees the template call before
  reduction, and the lowering layer reads it there to drive cleave's text change. The Core of
  `bracketed x` is `x`.
- `TextChange.removeBracketed` stays in Core as cleave's effect [CR#702.148a,702.148b]; its doc
  says the bracketed text is read from the authoring tree by lowering, not from a Core marker.
- Pins: the authoring tree of a bracketed card records the `bracketed` call; the Core of
  `bracketed x` is `x`; existing cleave cards still check. Pins that asserted a Core `.bracketed`
  tree are rewritten onto the authoring-tree claim.
