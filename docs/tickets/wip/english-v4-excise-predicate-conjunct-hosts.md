---
needs: []
---
# Remove the one-card host kinds `NominalAndPredicateConjunct` and `PredicateConjunct`

## Owner's ruling (round 8, treatment item 3, Nodule 3)

Remove the one-card enum variants `Hosts::NominalAndPredicateConjunct`
(`frames.rs`) and `HostKind::PredicateConjunct` (`grammar.rs`). They exist
only for *second from the top or on the bottom*. Round 8 also freezes
mechanisms that exist to keep a Reading out (`docs/decisions/english-v4.md`
§4.4).

## Evidence

Re-grepped on `qynmxmro`. Line numbers drift.

- `crates/deckmaste_syntax/src/frames.rs:289–291`:
  `Hosts::NominalAndPredicateConjunct`, documented as "a nominal, or a predicate
  only as a conjunct of a coordination of unlike predicate adjuncts". It is
  also handled in `Hosts::mask` and `Hosts::admits` (`:296–328`) and in
  `host_bit` (`:337`, bit 8).
- `crates/deckmaste_syntax/src/grammar.rs:360–361`: `HostKind::PredicateConjunct`.
- `crates/deckmaste_syntax/src/compile.rs:1411`: `PredicateConjunct` names its
  child `adjunct`, like `Predicate`.
- `crates/deckmaste_lexical_source_v4/lexicon/english/prepositions.ron:167–173`:
  the only user. *on* + NP has `hosts: NominalAndPredicateConjunct`. Its comment
  says a locative *on*-phrase "may also coordinate with another predicate
  adjunct …, never attach alone".
- `crates/deckmaste_english_v4/grammar/english.ron:704–707`: `LocativeAdjunct`
  (`PredicateAdjunct → Hosted("filler", "PrepositionPhrase",
  PredicateConjunct)`), the only Production that takes the host kind. Its only
  consumer is `AdjunctCoordination` (`english.ron:1526`, `unlike:
  Some("ComplementKind")`), beside `PositionAdjunct` (`:682`) for *second from
  the top*.
- `crates/deckmaste_syntax/README.md:150–153` documents the variant.
- The witnesses are judged in
  `crates/deckmaste_english_v4/judgments/coordination.txt:45–49` (three unnamed
  lines, Wan Shi Tong, All-Knowing and Happy Hogan, Bodyguard). They assert
  *second from the top* as an AdjectivePhrase and *on the bottom* as a
  PrepositionPhrase. They also assert that neither *library second from the
  top or on the bottom* is a Nominal nor *into their library second from the
  top* a PrepositionPhrase. Six Oracle records contain *or on the bottom*.

## Change

Delete both variants, `LocativeAdjunct`, the `on` frame's special host, and the
README paragraph. Then find how *puts it into their library second from the top
or on the bottom* reads without them. Expect one of these:

1. **Ranking.** *on* + NP admits `NominalAndPredicate`, like *outside* + NP. The
   locative attaches to predicates generally, and the Ranking Key orders the
   new Readings. Measure how many units grow. Under §4.4 a rise is acceptable
   when the intended Reading stays in the cheapest tier.
2. **Existing coordination.** *on the bottom* already reaches the coordination
   through an existing Category, for example a Position Phrase. The glossary
   defines a Position Phrase as an ordinal with a *from* phrase (*second from
   the top*), so widening it would need a glossary amendment first.

Choose by measurement, and record both measurements in the landing. Add no new
host kind, Category or feature for this one phrase.

## Done when

- `NominalAndPredicateConjunct`, `PredicateConjunct` and `LocativeAdjunct` are
  gone from code, data and READMEs.
- *second from the top or on the bottom* reads with its endorsed analysis in the
  cheapest tier.
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
