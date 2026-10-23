---
needs: []
---

# english-v4: fold Locative, trim FinalParameter

Implements owner-rulings-12.md findings F3a and F3b.

## Change

- F3a: point slot_categories `LocativeNP` directly at `LocativeHead`; set
  `Require("$.NounKind", "Ordinary")` on FusedLocative; delete
  LocativeNounPhrase, LocativePhrase, `NounKind::Locative` and
  `ObliqueNounKind::Locative` (-1 Category, -1 Production, -2 values).
- F3b: `deckmaste_lexical_source_v4/src/build.rs` projects only the
  FinalParameter values english.ron reads (Cost, SetOffCost, SetOffAbility);
  delete Amount, SetOffCondition, Quality, Name, SetOffMeasure and
  SetOffKeyword from english.ron (-6 values), after confirming by search that
  nothing else reads them.

## Acceptance

Census delta of exactly 0 unit by unit (any change in a unit's Readings or
verdict is a STOP). No face lost, judgments hold, nothing ungrammatical
admitted, `strata --check` passes, no forest explosion. The load report no
longer lists `FinalParameter::Name` or `Locative`.

Standard constraints apply.

Census delta (measured on `<change>` → this landing): faces read, total Readings and Surplus before and after; lost faces by name, or "none".
Judgment set: the result of running it.
