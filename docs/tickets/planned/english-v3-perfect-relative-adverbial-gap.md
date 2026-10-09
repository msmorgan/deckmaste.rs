---
needs: []
---
# Read *the first time this ability has resolved* as a perfect relative clause with an adverbial gap

Priority: medium. This is a defect: an attested string has no correct parse,
and the Readings it does have are a wrong analysis that parses. The ticket is
independent of the per-line census.

## Why

26 faces contain "time this ability has resolved". Examples are Saint Traft
and Rem Karolus, Belladonna Took, Nissa, Leyline Tamer, Teething Wurmlet,
Omnath, Locus of Creation and Zimone, Mystery Unraveler. Bronze Cudgels has
"the number of times this ability has resolved this turn". At minting, 33 faces
match `cargo xtask english-v3 grep '\btimes? (this|that) (ability|creature|permanent) (has|have)\b'`.

Every Reading of these units takes *has* as lexical *have* with an Object gap,
and *resolved* as a separate Bare Passive postmodifier:

```
ParticipialPostmodifiedNominal[
  ObjectRelativeNominal[ PremodifiedNominal[First Time]
    ZeroObjectRelativeClause[ this ability  FiniteObjectGap[Have] ] ]
  PassivePredicate[Resolve] ]
```

That is "[the first time [this ability has __]] [resolved]". It was probed at
minting (provisioned `cargo-xtask`, trunk `zunvpkot` lexicon, Sentence
category):

| Unit | Readings | With lexical *have* + `PassivePredicate[Resolve]` |
| --- | ---: | --- |
| Saint Traft and Rem Karolus, "Whenever Saint Traft and Rem Karolus becomes tapped, create a 1/1 red Human creature token if this is the first time this ability has resolved this turn." | 84 | 84 |
| Belladonna Took, "Whenever a token you control enters, you gain 1 life if this is the first time this ability has resolved this turn." | 84 | 84 |
| Nissa, Leyline Tamer, "Then if this is the first time this ability has resolved this turn, reveal cards from the top of your library until you reveal a creature card." | `reading_count` 744 | all 432 distinct trees in `--json` |
| Teething Wurmlet, "If this is the first time this ability has resolved this turn, put a +1/+1 counter on this creature." | 24 | 24 |
| Bronze Cudgels, "Until end of turn, equipped creature gets +X/+0, where X is the number of times this ability has resolved this turn." | 6 | 6 |

The correct analysis is a perfect-aspect relative clause whose gap is a
temporal Adjunct: "the first time [(that) this ability has resolved __ this
turn]". *has* is the perfect auxiliary, and *resolved* is its past-participial
Complement. CGEL, Ch. 12 §3.3(e), pp. 1045–1046 (page-checked): in *Do you
remember the day we met Kim at the races —?* ([31ii]), "the gap … is
functioning as adjunct in its clause". [32i] lists the time type, *It was a
time in my life when everything seemed to be going right —*, and the passage
contrasts this with relativising only a preposition's complement (§3.3(d)).
[31ii] is itself a bare relative with no *that*.

The construction does not exist for any temporal head today. Three probes
read 0 at minting:

- "The first time you cast a spell each turn, draw a card.";
- Captain America, Living Legend, "If it's the first time that creature has
  become tapped this turn, untap it.";
- Archipelagore, "Tap up to X target creatures, where X is the number of times
  this creature has mutated.".

Other attested adverbial-gap units on *time* are Edgar, King of Figaro ("The
first time you flip one or more coins each turn, …"), Scion of Halaster,
Mirrormind Crown, Reed Richards, Smartest Man, Esix, Fractal Bloom, Moonlit
Meditation and Auspicious Starrix.

## Goal

1. Add one Construction: the adverbial-gap relative clause on a temporal head
   noun. It reaches the Nominal through the existing Object-relative route
   (`ObjectRelativeNominal`, `crates/deckmaste_english_v3/src/declarations.rs:3958`;
   `ZeroObjectRelativeClause` `:3950`; `ObjectRelativeClause` `:3941`).
   - The relative clause is a complete finite Clause, so its predicate is
     not a `FiniteObjectGap` (`:3920`). The perfect *has resolved* comes
     through the existing `PerfectAuxiliary` frame of `core-verb:Have`
     (`core.ron:1303–1305`) and `PerfectAuxiliaryPredicate` (`:1546`).
   - The head is licensed by a declared lexical property of the head noun,
     for example a relativisable-Adjunct class `Temporal` on
     `lexeme:CommonNoun/Time`. Read it through a table over (gap kind, head
     class): an Object gap takes any head, and an Adjunct gap requires the
     declared class. Never name *time*.
2. Retire the wrong lexical-*have* analysis on these strings, with zero lost
   faces. It is a wrong analysis that parses. The route is a declared one:
   - `PassivePredicate` (`:3887–3896`) requires `head.frame = Transitive`, and
     *resolve* declares `Predicate([ObjectNounPhrase])`
     (`crates/deckmaste_lexical_source/lexicon/verbs.ron:152–155`).
   - At minting, all 27 faces containing *resolved* have it only in "this
     ability has resolved". `grep` finds no *resolve(s/d)* followed by an
     Object NP.
   - Under the ADR pruning rule, retire *resolve*'s Object frame if the census
     confirms zero attested transitive uses and zero lost faces. That removes
     both `PassivePredicate[Resolve]` and the transitive-perfect "has resolved
     [this turn]" Object reading.

   If a lexical-*have* Reading of these strings survives by another route,
   probe it and retire it through a declared feature. If that needs a word
   guard or a second new Construction, STOP.

## Analysis notes

- *that* is ambiguous in "the first time that creature has become tapped"
  (Captain America): it can be the demonstrative Determiner or a relative
  marker. Leave both Readings if both parse. Do not add *that*-relative
  support for adverbial gaps unless a witness needs it, and record it as a
  Deviation if one does.
- Every face that becomes newly covered (Captain America, Archipelagore,
  Auspicious Starrix, Edgar and the other *the first time you …* units) is a
  gain. Inspect each and list it with its selected analysis. A gain whose
  Reading is not the adverbial-gap relative is a defect.
- Name the new Construction and the head feature from glossary terms. Add
  them under **Relative Clause** and **Gap** (`docs/contexts/oracle-english/CONTEXT.md:605`,
  `:1162`), citing the CGEL passage above.

## Witnesses

The units in the table must FALL. After the change, no Reading of these units
contains `FiniteObjectGap[Have]` with `PassivePredicate[Resolve]`. Every
Reading has the adverbial-gap relative with `PerfectAuxiliaryPredicate` *has
resolved*. The expected count is the product of the remaining independent
choices: where *this turn* attaches inside or above the relative; the lexical
*first* split (Adjective or Ordinal); and the premodifier scope, if
`english-v3-temporal-pp-and-premodifier-scope` has not yet landed. The count
is expected to fall, as in Saint Traft and Belladonna Took 84 → fewer. If a
count rises, report the Reading shapes before landing.

1. Saint Traft and Rem Karolus, the unit above: 84 → falls, correct shape
   only.
2. Belladonna Took, the unit above: 84 → falls, correct shape only.
3. Nissa, Leyline Tamer, the unit above: 744 → falls, correct shape only.
4. Teething Wurmlet, the unit above: 24 → falls, correct shape only.
5. Bronze Cudgels, the unit above: 6 → falls, correct shape only.

Controls that must NOT change:

- Saint Traft and Rem Karolus, "If it's the second time, create a 1/1 blue
  Spirit creature token with flying.": 32. This is elliptical *the second
  time* with no relative.
- Aurelia, the Warleader, "Whenever Aurelia attacks for the first time each
  turn, untap all creatures you control.": 4.
- Felisa, Fang of Silverquill, "Whenever a nontoken creature you control dies,
  if it had counters on it, create X tapped 2/1 white and black Inkling
  creature tokens with flying, where X is the number of counters it had on
  it.": 280. Its lexical-*have* Object-gap relative "counters it had __ on it"
  is correct and must stay.

Expected gains, not controls: "The first time you cast a spell each turn, draw
a card." 0 → ≥ 1, with the adverbial-gap relative. The Captain America and
Archipelagore units: 0 → ≥ 1 if nothing else in them is unread.

## Method

Standard constraints apply. Deltas:

1. **Probe first.** Probe every witness and control, write them as tests, and
   run the tests once at the end.
2. **Bounded census.** Selector: faces matching `\btimes?\b` or `\bresolv`
   (via `cargo xtask english-v3 grep … --json`) as an `--identity-manifest`.
   Union it with 300 random covered faces from `diff --manifest … --random 300
   --seed <S>`, and record the seed. Run the claim parent and the feature on
   the same manifest.
3. **Compare.** Use `cargo xtask english-v3 diff`, or `diff --lines` if
   `english-v3-per-line-readings` has landed.
   - Zero lost faces.
   - Name every decrease with the Reading shape it lost. The expected shape is
     lexical-*have* plus Bare Passive *resolved*. Give counts by class in the
     record and names for the top 20.
   - List every gain with its analysis.
4. **Measure.** Run under `/usr/bin/time -v` with `--workers 6`, and record
   wall time and peak RSS.
5. **Construction count.** Report it before and after, counting `construction`
   and `schema` declaration lines in `declarations.rs` as the
   overgeneration-prune record did. This ticket adds exactly one Construction.

## Landing record

The six items in `docs/tickets/README.md` ("Landing record"), plus the Readings total before and after on the
manifest (per line if the per-line census has landed).
