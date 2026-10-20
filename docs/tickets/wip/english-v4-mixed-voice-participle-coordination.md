---
needs: []
---
# Restrict mixed-voice `VerbPhrase` coordination to non-participial verb phrases

Stratum 6 added a mixed-voice `VerbPhrase` coordination schema
(`crates/deckmaste_english_v4/grammar/english.ron:1184`,
`unlike: Some("Voice")`, `unshared: ["VerbKind", "Voice", ...]`) for finite
mixed-voice predicates (*enters or is put into a graveyard*). Because that
schema does not exclude `PastParticiple`, any coordination of two past
participles (*attacked or blocked by this creature*, *damage dealt or
prevented*) also matches `unlike: Some("Voice")` by pairing an active past
participle with a passive one, leaving `Voice = UNSET` on the coordinated
`VerbPhrase`.

Meanwhile, `Passive` (`english.ron:536`), `ParticipialSupplement` (`513`),
`SupplementedVerbalCost` (`855`), `PastParticipialConjunct` (`1028`),
`ParticipialPostmodifier` (`1037`), and `SingleParticipialPostmodifier` (`1044`)
check `Require("....Voice", "Passive")` rather than `Carry`, which succeeds when
`Voice` is `UNSET`—doubling derivations on every passive participle
coordination in the corpus.

## Change

- Exclude `PastParticiple` from the mixed-voice `VerbPhrase` coordination
  schema (`english.ron:1184`: `Exclude("$.VerbForm", "PastParticiple")`), and
  use `Carry("....Voice", "Passive")` on passive-participle consumers
  (`Passive`, `ParticipialSupplement`, `SupplementedVerbalCost`,
  `PastParticipialConjunct`, `ParticipialPostmodifier`,
  `SingleParticipialPostmodifier`) so an unset `Voice` never satisfies a
  passive-only construction.

## Done when

- *creature attacked or blocked by this creature* and *was attacked or blocked
  this turn* no longer get a second derivation through mixed-voice participle
  coordination.
- All mixed active/passive finite and infinitival coordinations (*dies or is
  put into exile*, *enters or is put into a graveyard*) still read, with no
  face lost.

Standard constraints apply.

## Record

- The mixed-voice `VerbPhrase` coordination now excludes past participles
  (`Exclude("first.VerbForm", "PastParticiple")`; an equation over `$` is not
  available for an `Exclude`, and the conjuncts share `VerbForm`).
- The `Require` → `Carry` change on the six passive-participle consumers is
  not made. With the schema fixed, no participle coordination leaves `Voice`
  unset, and the change measured zero census difference on top of the fix. It
  would only add consumer-side checks.
- The ticket's example phrases are not Oracle text. `+` Judgments on real cards
  in `mixed_voice_participles.txt` cover the intent: Elspeth, Undaunted Hero
  (*a card named Sunlit Hoplite and put it onto the battlefield* is no
  participle coordination), God-Eternal Bontu (*dies or is put into exile*)
  and Mephitic Draught (*enters or is put into a graveyard*).

1. **Census delta** (measured on `mvrxnorx`): faces read 28,709 → 28,709;
   total Readings 382,152 → 381,876; lost faces: none.
2. **Judgment set**: 2,902 enforced at `COMPLETED_STRATUM` 6, all hold; 190
   pending.
