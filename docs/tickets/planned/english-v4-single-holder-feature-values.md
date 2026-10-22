---
needs: []
---
# Report single-holder feature values, and fold or justify each one

## Owner's ruling (round 8, treatment item 3)

Enforce english-lexical-analysis §3 ("A class with one member, or a feature
with one holder, is reported as a finding") on feature values too. Extend
`compile.rs`'s fewer-than-two-members check from lexical classes to feature
values. Then fold the single-word values (NounDistribution `Serial`, `Valued`,
`BareSingular`; the `LocativeQuantifier` class) and the one-value features
(`AmountKind`, `Targeting`, `Conversion`). `docs/decisions/english-v4.md` §4.4,
item 5, applies the finding to new values too.

## Evidence

Re-grepped on `qynmxmro`. Line numbers drift.

**The check today.** `CompiledGrammar::bind` (`crates/deckmaste_syntax/src/compile.rs:551–590`)
warns into `LoadReport::warnings` when a declared or typed lexical class has
fewer than two members. Nothing checks feature values.

**The values.** `english.ron` line numbers; holders from
`crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron`.

| Value | Holders | Consumers | What depends on it |
|---|---|---|---|
| NounDistribution `Serial` (`:44`; `schema.rs:758–761`) | *time* (`nouns.ron:143`) | `TemporalLocation` excludes it (`:750`); `SerialTemporal` requires it (`:752`); `MultipliedDeterminer` and `MultipliedNounPhrase` require it (`:917`, `:920`) | Frequency Noun Phrase Adjuncts (*any time*, *three times*), multipliers (*three times X*). The landed `english-v4-temporal-np-as-direct-object` fix (`Determined`'s two `ExcludeTogether`s, `:1090–1091`) keys on `NounDistribution` `Ordinary`, so *time* escapes it only because it is `Serial`. The round-7 Option A for `english-v4-frequency-np-direct-object` named `Serial` too; that ticket is now a ranking ticket. |
| NounDistribution `Valued` (`schema.rs:753–757`) | *power*, *toughness* (`nouns.ron:119, 149`) | `AttributiveNoun` excludes it (`:1034`); the bare property Noun Phrases require it (`:1068`, `:1077`, `:1083`); `BarePlural` (`:1140`) | *power 3 or less*, *base power and toughness 4/4*, *its power*; no attributive *power* |
| NounDistribution `BareSingular` (`schema.rs:749`) | *attacking player*, *defending player* (`nouns.ron:176, 178`) | `BareSingularTerm` (`:1027`) and `BareSingularPossessor` (`:1198`) require it; `SingularNominalQuality` (`:513`) and `BarePrepositionalNominal` (`:1065`) exclude it | *defending player controls*, *defending player's* |
| Lexical class `LocativeQuantifier` (`:342`) | `det:anywhere` | `FusedLocative` (`:1033`) | *from anywhere*. The existing class check already reports it. |
| `AmountKind` = [`ComparativeBounded`] (`:46`) | set only by `ComparativeBoundedAmount` (`:876`) | `NumericNounPhrase` excludes it (`:434`) | *power 3 or less* is a noun's value, never a numeric Noun Phrase |
| `Targeting` = [`Marked`] (`:49`) | set by `MarkedDeterminer` and `MarkedMaximum` (`:1176`, `:1178`) | `TargetedNounPhrase` (`:1134`), the Noun Phrase Coordinations (`:1491–1502`) and the Determiner Phrase Coordination (`:1534`) | the Targeting Marker after an outer determiner (*two target*, *up to one target*) |
| `Conversion` = [`Converted`] (`:62`) | derived by `deckmaste_lexical_source_v4/src/build.rs:450–467` for every free noun/adjective lemma pair (colour words, *top*, *total*) | `AdjectivalQuality` excludes it (`:518`); `NominalPredicative` excludes it with a bare determination (`:1371`) | colour words as Quality and as predicative |

## Change

1. **The finding.** In `bind`, add two `LoadReport` warnings: a declared
   feature with fewer than two values, and a feature value with fewer than two
   holders. A holder is a lexeme whose feature bundle carries the value, or,
   for a value no lexeme carries, a Production that sets it on its mother. The
   warning names the feature, the value and its holders. Add a test in
   `deckmaste_syntax` for each warning.
2. **Fold or justify each value in the table.**
   - *Fold*: express the distinction through an existing feature, value,
     class or Frame, so the value goes. Examples: a declared class or frame on
     the lexemes instead of a distribution value; a one-value feature replaced
     by an existing feature's value.
   - *Justify*: keep it, with a comment in `english.ron` or the lexicon citing
     CGEL or `docs/oracle-style-guide.md` for the grammatical distinction it
     carries. Also say why one holder is the whole class in Oracle English.
     A value whose only job is to exclude a dispreferred Reading cannot be
     justified (`english-v4.md` §4.4, item 2). Its exclusion goes, and the
     Ranking Key orders what it kept out.
3. Record each decision in the landing, with what moved for its dependents
   above.

## Done when

- `bind` reports single-holder feature values and one-value features, with
  tests.
- Each value in the table is folded or carries its justification. The load
  report lists only justified ones.
- No face is lost.
- Every judgment holds.
- `cargo xtask english-v4 strata --check` passes.
- The landing reports the cheapest tier: ambiguous units with a tier of one,
  the tiers' sum and the largest tier, before and after. No judgment-set entry
  goes from an all-endorsed cheapest tier to one that is not.
- No new `Summary`, `State` or `ProductionDecl` field, and no new `closure.rs`
  pass.

Standard constraints apply.

Census delta (measured on `<change>` → this landing): faces read, total Readings and Surplus before and after; lost faces by name, or "none".
Judgment set: the result of running it.
