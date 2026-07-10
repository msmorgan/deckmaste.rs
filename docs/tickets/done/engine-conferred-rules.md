---
needs: []
---
Predicate-scoped conferral of intrinsic abilities and replacement effects as
rules-as-data, generalizing subtype `confers` (which is the only conferral axis today;
the `Type` enum has no `confers`). A `ConferralRule { scope: Predicate, confer: Property }`
loaded from `rules/grant/` (templated on the `rules/sba/` `SbaRule` loader), injected
into `GameState` after construction like `sba_rules`, and matched against battlefield
objects via `crate::matches`. Must make conferred abilities — including
`Static(Replacement(Also { would: ThisEnters, also: ... }))` — visible everywhere printed
abilities are, closing the gap that the enters-with-counters fold (`as_enters_status`)
reads only printed face abilities. Ships the `Property::Ability` (incl. replacement) path
end to end, proven by an engine test that confers a replacement onto a card type and
observes it applied at entry, independent of any card.
