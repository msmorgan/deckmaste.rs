---
needs: []
---
Widen the typed macro-template slot readers so correctly-bracketed templates
parse without bespoke arms. The routing machinery is done and generic
(`TemplateIndex` matching, `parse_macro_effect` fallthrough): a template
declines only when a slot's declared type has no reader. Today the effect-
level reader (`parsers/effect.rs::macro_slot_reader`) covers `Count` and the
self-reference forms of `Reference`; the keyword-level reader covers
`Predicate` and `Cost`. Probe proof (2026-07-16): `Scry 2.` parses,
`Amass Orcs 1.` fails purely because `String` has no reader — macro,
template, and body are all already correct.

Add readers, each delegating to an existing sub-parser (no new vocabulary):
`String` (subtype names — unlocks Amass today), quality/filter predicates at
effect level, counter kinds, zones, and bare-numeral `Count` at keyword
level where missing. Each new type lights up EVERY template using it, now
and later — this is the standing multiplier for keyword-action authoring.

Out of scope: slots that must DECLARE a target on the frame — that is a codec
capability change, design-gated as `parse-target-hoisted-effect-macros`.

Verify: `Amass Orcs 1.`-bearing cards graduate via
`cargo xtask generate plugins/wizards`; unit tests per reader beside the
existing `macro_slot_reader` tests.
