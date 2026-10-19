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
