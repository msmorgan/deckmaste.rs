---
needs: []
---
**A subtype in a card is macro-only.** Ruling (user, 2026-09-07). Canon
cards spell `subtypes: [Of(host: Creature, label: "Gargoyle")]`; the
declaration `subtypes/creature/gargoyle.ron` exists (324 creature subtypes,
seven categories) but the `Subtype` meta emits an empty body, so `gargoyle`
is not invocable, and `Subtype` is not among the reader's restricted
expression kinds, so the raw constructor is accepted in card text.

Three parts: (1) the `Subtype` meta derives the body from the declaration
— `Of(host: <category>, label: <spelling>)` for object subtypes,
`Spell(label: <spelling>)` for spell subtypes — so no declaration writes a
body by hand; (2) `Subtype` joins the restricted kinds in
`deckmaste_semantics_v2::ron` (card text refuses a raw `Of`/`Spell`,
listing the names); (3) every canon and testing card is rewritten to
invoke the declaration (`subtypes: [human, knight]`), by a scripted
name-directed rewrite over the `subtypes:` field only (multi-word
subtypes take the camelCase name under the case rule, e.g. `timeLord`),
with `lean/Generated` byte-identical, `lean-check` 118/118 and 2/2, and
`facts check` byte-identical as the oracles. Counter kinds
(`Named(label: "charge")`) are the same question and a candidate for the
same treatment; do not fold them in without a ruling. Standard
constraints apply.
