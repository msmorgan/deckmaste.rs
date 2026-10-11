---
needs: [english-v3-mana-symbol-nominal]
---
# License symbol notation as the Subject of a locative Verbless Clause

Complete shape (b) returned under MS4 of `english-v3-mana-symbol-nominal`.
Standard constraints apply. The settled rulings are MS2–MS4 and W9-A:
license a symbol class through the governing frame's declared feature, keep
bare symbol notation outside Noun Phrase, preserve existing Mana Phrase and
Cost Readings, and retain independent Ambiguity.

The attempted single-form `with` + Mana Phrase + Locative Complement route
required globally admitting `{X}` and numeric cost symbols as Mana Phrase.
The existing test `mana::independent_mana_values_preserve_symbol_sequence`
found the regression: `{X}` became one bare Mana Phrase Reading instead of
zero; `{2}` is another protected negative in the same test. Those global
licences were removed. A governed Subject projection and its enclosing
preposition form need separate admission; MS4 authorizes this follow-up
instead of expanding the original landing beyond one added form for (b).

Witnesses, both zero on the original claim parent `sprutzvq`:

- Matterbending Mage: “Whenever you cast a spell with {X} in its mana cost,
  this creature can't be blocked this turn.” Its original nominal control
  “Whenever you cast a spell with a counter on it, draw a card.” has 10
  Readings.
- Gaddock Teeg: “Noncreature spells with {X} in their mana costs can't be
  cast.” Its control “Noncreature spells with counters on them can't be
  cast.” has 5 Readings.

Keep `mana::independent_mana_values_preserve_symbol_sequence` unchanged.
Re-spell and enable the existing blocked test
`mana_symbol_nominal::cost_notation_is_the_subject_of_the_licensed_locative_small_clause`
against the final Subject structure, retaining both cards and the asserted
Mana Phrase Subject and locative Predicate roles. Do not delete it.

Use `cargo xtask english-v3 grep '\b(with|contains?) \{X\}'` for the bounded
selector, with the required seeded covered controls. Preserve the independent
ambiguities in `in its mana cost` and `in their mana costs`; `contain(s)`
remains a separate lexical gap. The original attempt also covered Magus Lucea
Kane through its spell-mana-cost and ability-activation-cost Subjects; that
face is a useful additional witness if its other constructions still read.
