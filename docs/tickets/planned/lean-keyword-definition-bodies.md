---
needs: []
---
**A keyword ability's body is its definition; widen the Lean law to admit
it.** Ruling (user, 2026-09-06, resolving the contradiction found while
briefing `semantics-v2-macro-bodies-keyword-abilities`): the v2 `KeywordAbility`
macro for each keyword carries the keyword's rules definition as the
`Ability.keyword … (body : Option Ability)` payload, ported from the 55
definitional bodies under `plugins/builtin/macros/keyword/` (Flying is
`Static(Cant(Block(on: This, by: Not(Or([Has(Flying), Has(Reach)])))))`
[CR#702.9b]). Today `keywordBodyFits` (`lean/Semantics/Check/Abilities.lean`)
admits only `none`, or a triggered body on a keyword whose facts say
`bodied` with a matching stack regime; `badBodyOnBodilessKeyword`
(`Proofs/Faces.lean`) pins that a body on Flying is refused. Both must
change so the definition is admitted and the gate we now run
(`cargo xtask lean-check`) proves cards that invoke the widened macros.

Decide and land, standard constraints applying:

- **The new law.** A body is optional (a card may write bare "flying"); when
  present its regime must agree with the keyword's declared regime from
  `Facts.lean` (a static keyword takes a static body, a triggered one a
  triggered body whose event regime matches, an activated one an activated
  body). Decide whether a keyword may carry more than one ability (v1's
  `Composite` allowed a list; Lean's slot is one `Ability`) and, if so,
  what shape holds them. Decide what the `bodied` facts column now means
  (it meant "reminder text is printed"); retire it or redefine it in
  `crates/xtask/src/facts/` overlays and regenerate, `facts check` clean.
- **Pins.** Re-spell `badBodyOnBodilessKeyword` against the new law (a
  regime mismatch, e.g. a triggered body on Flying, refused as
  `keywordBodyFits`), keep `okRenownWithRenownExpansion` and its siblings,
  add a positive pin for a static definition on a static keyword.
- **A worked definition.** Add `flying` to `Macros.lean` carrying its
  definition, with the `StaticSpec` shape "can't be blocked except by
  creatures with flying and/or reach". If no `StaticSpec` constructor can
  express a block-legality restriction of that form, that is a modelling
  extension inside this ticket's remit; add it with its CR citation and
  pins, and say so in the landing record. Port one more definition of each
  regime (a triggered keyword and an activated keyword) to prove the law
  across regimes; the remaining ports are
  `semantics-v2-macro-bodies-keyword-abilities`'s.
- Mirror the changes into `crates/deckmaste_semantics_v2` if any syntax
  type changes (drift test), and note in `docs/decisions/semantics-v2.md`
  §6 that keyword ability macros carry definitions.
