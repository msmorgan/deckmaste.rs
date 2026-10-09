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

1. **Census delta** — Pre-refresh Document census: `rqtsuqxv` claim-parent source (the empty initial `yppxtnmu`) → implementation `yppxtnmu`: 419 → 423 covered faces; 158,772 → 154,679 Readings. Same bounded identity manifest: all 495 supported faces containing ` time` (including ` times`) or `resolved`, plus 300 random covered faces, seed 20261008; four overlaps, 791 distinct faces. Stock `english-v3 diff`: lost 0, gained 4, decreases 8, increases 3, digest-only 14, unchanged 762; no source changes, issues, limits, duplicate Readings or internal failures. Lost faces: none. All eight decreases retire wrong lexical-*have* Object-gap / Bare Passive *resolved* analyses: Belladonna Took 672 → 640; Bronze Cudgels 6 → 3; Harvestrite Host 252 → 240; Omnath, Locus of Creation 1,344 → 1,280; Rose Room Treasurer 336 → 320; Saint Traft and Rem Karolus 86,016 → 81,920; Vito, Fanatic of Aclazotz 6,048 → 5,760; Zimone, Mystery Unraveler 168 → 160. Increases: Burn at the Stake 93 → 108 (temporal *times* relative with finite *the number of creatures tapped this way*, alongside retained multiplicative analyses); Moraug, Fury of Akoum 72 → 240 (perfect *each time it has attacked*); Spiny Starfish 45 → 54 (finite *each time it regenerated*). Evidence: `target/english-v3/english-v3-perfect-relative-adverbial-gap/{manifest,before,after,diff}.json`. After refresh, the same 791-face manifest on `mqllpupn` retains 423 covered faces, with 90,699 reported line Readings and zero issues, duplicates or internal failures. Stock `refresh-diff.json` against the pre-refresh feature: lost 0 / gained 0 / decreases 63 / increases 193 / digest-only 167 / unchanged 368. Every refresh decrease (names in that stock diff), increase and digest change is attributed to trunk `yntwupzymxxl`, whose census switches Document products to sums of Ability-line Readings; these are different reporting units, with the face Reading products retained.
2. **Witnesses** — Sentence probes, measured on `yppxtnmu`, before → after: Saint Traft and Rem Karolus 84 → 80; Belladonna Took 84 → 80; Nissa, Leyline Tamer 432 distinct → 432; Teething Wurmlet 24 → 24; Bronze Cudgels 6 → 3. Every corrected Reading contains `AdverbialGapRelativeClause` and `PerfectAuxiliaryPredicate`, with zero `FiniteObjectGap[Have]` or `PassivePredicate[Resolve]`. Controls: Saint Traft's second-time unit 32 → 32; Aurelia 4 → 4; Felisa 280 → 280, retaining lexical-*have* Object gaps.
   Nissa (`yppxtnmu`): 432 = first Adjective/Ordinal 2 × time Count/Mass 2 × premodifier scope 2 × this-turn attachment 3 × 18 independent outer configurations: five Then/Until scope positions permit respectively 1, 5, 5, 4, 3 From-PP positions (Clause, predicate, or nominal attachments); each outer configuration has all 24 distinct condition trees (`nissa-factors.json`).
   Teething Wurmlet (`yppxtnmu`): 24 = first Adjective/Ordinal 2 × time Count/Mass 2 × premodifier scope 2 × this-turn attachment 3 (matrix copula predicate, relative perfect predicate, or its secondary lexical predicate); these are distinct constituent placements, not two labels for one constituency; `teething-factors.json` verifies all 24 tuples exactly once.
   Captain America (`yppxtnmu`): 0 → 24; all use bare adverbial-gap relative `[that creature has become tapped]`, demonstrative *that*, and perfect auxiliary *has* with *become tapped* Complement. Shapes: 8 attach *this turn* to the matrix copula; 8 to the relative finite perfect predicate; 8 to its secondary *become* predicate; each group has the independent first, countability and premodifier-scope choices above. Relative-marker *that* does not parse. The first-time initial sentence stays 0; its NP `the first time you cast a spell each turn` has 16 NounPhrase Readings, all adverbial-gap relatives. Its determiner-free constituent `first time you cast a spell each turn` has 16 at Nominal; the full NP has 0 at Nominal because *the* belongs to the NounPhrase. Archipelagore stays 0, blocked by undeclared *mutated*. Probe evidence: `before-probe-*.json`, `after-probe-*.json`, `first-time-{np,nominal,bare-nominal}.json`, `archipelagore-np.json` under the evidence directory above.
3. **Spot checks** — Only four newly covered faces exist in this bounded run (`yppxtnmu`, pre-refresh Document probes), so all four were inspected exhaustively rather than inventing ten: Botanical Brawler 160, temporal relative with perfect passive *+1/+1 counters have been put on that permanent*; Captain America, Living Legend 24, demonstrative *that creature* and perfect *has become tapped*; Rumbling Aftershocks 18, temporal relative *that spell was kicked* (demonstrative *that*, finite copular predicate); Stalwart Successor 32, temporal relative with perfect passive *counters have been put on that creature*. Every gained Reading has the adverbial-gap relative. All three increased faces were also probed exhaustively; existing analyses remain. Evidence: `spot-0.json` through `spot-6.json`.
4. **Deviations and STOPs** — Orchestrator rulings, 2026-10-09: **R1** resolves the count-forecast STOP: equal counts pass when the wrong shape is absent and every Reading has the correct perfect adverbial relative; the independent Nissa and Teething counts are accounted for above. **R2** resolves the initial-Adjunct / missing-verb STOP: preposed nominal Adjunct routing is outside this ticket, owned by `docs/tickets/planned/english-v3-next-time-relative-adjunct.md`; the Sentence 0 and constituent NP coverage are accepted, as is Archipelagore's undeclared *mutated*, which was not added. **R3** requires Captain America's Reading shapes and both *that* analyses if both parse; only the demonstrative analysis parses, as recorded above. These are project rulings, not CGEL claims. Refresh incorporated trunk `yntwupzymxxl` (per-line census) through completion `sskyxnyvmruk`; its forest/Reading code warranted the bounded subset recheck and targeted English tests. The reporting-unit change is backed by the [owner direction of 2026-10-09](../../decisions/english-lexical-analysis.md#amendment-census-readings-per-top-level-line-2026-10-09), not a grammatical Reading retirement. The existing lexical-source `verb_frames_preserve_complement_kind_and_order` test was re-spelled to assert Resolve's sole intransitive frame after the approved retirement; Trigger's exact two-frame assertion remains. Exactly one Construction added: Adverbial Gap Relative Clause; construction/schema count 284 → 285 (`rqtsuqxv` source → `yppxtnmu`). Object-relative routes declare Object gap kind and the shared nominal route reads declared head/gap features; no lexeme guards or initial nominal Adjunct route added. Resolve's transitive Object frame retired on both lexicon and grammar sides via existing frame selection: the generic passive remains for other verbs, while Resolve can no longer select it. Supported-corpus `english-v3 grep` attestation (`resolve-attestation.json`, `yppxtnmu` before edit) found 45 resolve-family faces / 46 tokens, no transitive use; all 27 *resolved* faces use *this ability has resolved*. This pruning was pre-approved and the bounded diff loses no faces. Glossary gaps filled: Adverbial Gap Relative Clause, Relativisable Adjunct Class and Relative Gap Kind; CGEL Ch. 12 §3.3(e), pp. 1045–1046 backs adjunct gaps and bare temporal relatives, while the declared head licence is a project modelling choice.
5. **Tests** — On `yppxtnmu`: `cargo test -p deckmaste_english_v3` passed (657 integration tests; 0 failures or ignored tests; 0 doctests). `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` initially found two new-test lint issues (semicolon and needless Box replacement); after those corrections the same command passed. `cargo test -p deckmaste_lexical_source` had 34 passes and one failure because `lexical_gaps::verb_frames_preserve_complement_kind_and_order` still expected the retired Resolve Object frame; its exact-value assertion was re-spelled against the retained intransitive frame, and `cargo test -p deckmaste_lexical_source --test main lexical_gaps::verb_frames_preserve_complement_kind_and_order` passed (1 test, 34 unchanged tests filtered). `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` passed before and after that correction. No full suite was rerun. Test changes: restored 0 / re-spelled 1 / ignored 0 / added 4 / removed 0. New integration tests use tests/main.rs → tests/suite and check corrected shapes, unchanged controls, constituent coverage, demonstrative *that*, independent construction roundtrips and head licensing. No test removals. Citation checks: 0 noncompliant, 0 stale; diff audit selected 0 citation sites; no new CR citations. After refresh on `mqllpupn`, only `cargo test -p deckmaste_english_v3 --test main perfect_relative_adverbial_gap::` (4 passed) and `cargo test -p deckmaste_english_v3 --test main packed::` (22 passed) were rerun; zero failures or ignored tests. Lexical-source results and clippy results were retained. Command logs are under the evidence directory above.
6. **Timing** — One post-change bounded census on `yppxtnmu`: 60.08 seconds wall, workers 6, `/usr/bin/time -v` peak RSS 12,254,832 KiB (corpus wall 56.478 seconds). No full corpus census was run; the orchestrator owns that batch check.
