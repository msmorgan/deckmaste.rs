---
needs: []
---
# The Targeting Marker ends a Determiner Phrase

[`english-v4-retire-adj-target`](english-v4-retire-adj-target.md) built every
targeted phrase from a `TargetedNominal` that the Targeting Marker determined.
To keep postmodifiers from attaching above the marker, it placed
`Exclude(head.Targeting, Marked)` on every consumer of a Nominal:
NominalPostmodifier, CoordinatedPostmodifier, TemporalPostmodifier,
PostpositiveAdjective, PostpositiveComparative, ComparativeIndirectComplement,
EquativeIndirectComplement, RelativeTime, the `Adjunct(host: "Nominal")` PP
schema, and the bare Noun Phrase productions (BarePlural, BareMass,
ValuedBareMass, BareSingularTerm). This is the policing pattern that
`docs/decisions/english-lexical-analysis.md` §3 warns against: any new Nominal
attachment would have to remember the exclusion.

## Change

- *target* (`det:target`, now with its own `Targeting` determinative
  distribution) ends a Determiner Phrase. It can stand alone (*target
  creature*), follow an adjective (*other target creatures*, *a second
  target*), or follow an outer determiner (*two target*, *each other target*,
  *[one or two] target*). A maximum Preposition Phrase can also precede it
  (*up to one target*). The marker heads the phrase, and the outer determiner
  supplies its Number.
- A targeted phrase is an ordinary Determined Noun Phrase or possessor.
  Postmodifiers attach inside the Nominal, below the determiner, so the marker
  covers every postmodifier by construction (*target [creature you control]*).
- The following are retired: `TargetedNominal`, `TargetingPremodifier`,
  `TargetedPremodifier`, the `Targeted` unary, `TargetedNounPhrase`, the
  possessor-only targeted nominal and its productions, and every
  `Targeting`-based exclusion on consumers.
- `Targeting: Marked` stays on a determiner that has an outer determiner before
  the marker. Determined carries it to the Noun Phrase for the Noun Phrase
  coordination rules that branch on `first.Targeting`.
- *other*, *second* and similar adjectives before the marker are read by one
  production: an adjective followed by the marker.
- The glossary amends Targeting Marker and retires Targeted Nominal, which no
  longer names a constituent.
- Attributive `adj:double` and `adj:triple` are deleted from `adjectives.ron`.
  They have no Oracle attestation. `*NounPhrase double creatures and
  planeswalkers` is restored.

## Done when

- Every `Targeting`-based Exclude that retire-adj-target added is gone.
- No face is lost, and every change in a unit's derivation count is
  classified.
- All judgments hold, and tests and clippy pass for every crate touched.
- The grammar has fewer rules than before.

Owner ruling: no policing of strings that no card contains. Under this ruling
the following judgments are retired, and the retire-adj-target Done-when items
for them are superseded:
- `*NounPhrase target two creatures`
- `*NounPhrase the or target creature`
- `*NounPhrase target first creature`
- `*NounPhrase only creature`

Findings, accepted by the coordinator under the owner's no-policing ruling:
- `DeterminativeDistribution::Targeting` has one holder, `det:target`. Under
  `english-lexical-analysis.md` §3 a one-holder feature is a finding, so it is
  reported here. It is what keeps an outer Dependent determiner from passing
  its distribution to the composite without any consumer exclusion.
- Three real units regain wrong Readings that retire-adj-target had removed
  with `Exclude(head.Determination, Overt)` on TargetingPremodifier. In each,
  the marker determines a nominal that needs an overt determiner of its own:
  Cloud, Planet's Champion 16 → 18; Frodo, Determined Hero 6 → 8; Eternal
  Flame 8 → 12. Blocking them now would take a consumer exclusion on
  Determined. They are deferred to Stratum 7.

Standard constraints apply.

1. **Census delta** (measured on `xkvlrypx`): faces read 28,705 → 28,705;
   total Readings 382,567 → 382,529; lost faces: none.
2. **Judgment set**: 3,011 enforced at `COMPLETED_STRATUM` 6, all hold; 189
   pending.
