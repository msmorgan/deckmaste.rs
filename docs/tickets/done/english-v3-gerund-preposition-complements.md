---
needs: []
---
# Read gerund-participial Complements of prepositions: without paying its mana cost

## Why

Prepositions other than *by* reject a gerund-participial Complement.
On claim parent `pvmzprszmnpt` (32,828 supported faces, 21,486 covered,
11,342 unread; wave-3 recon of 2026-10-07), the recon buckets "without paying" and "instead of + gerund" read
together touch **360** unread faces and are the sole cause on **174**.
Counts are unread faces *touched* (at least one localised failing unit
matches) / *sole* (every failing unit matches and no other recon STRONG bucket
does). They are surface counts, not gain forecasts.

Probes (admitted roots): "Draw a card by paying {2}." 1; "Draw a card after
paying {2}." 0; "You may cast it without paying {2}." 0; "You may cast it
without paying its mana cost." 0, while "You may cast it." 1; "If that spell is
countered this way, exile it instead of putting it into its owner's
graveyard." 0, while "… exile it instead." 1.

This ticket takes over from `english-v3-systemic-residuals` the Instead landing
follow-up of 2026-10-07: the missing selected gerund-participial Complement
beneath *of* inside the compound PP *instead of* (53 faces at that stamp).

## Goal

*without* and *of* (inside *instead of*) take a gerund-participial clause as
Complement, licensed by a declared feature on the preposition the way the
existing `GerundComplementPreposition` route licenses *by*; the gerund's Object
and Adjuncts read as in any clause (*putting it into its owner's graveyard*,
*paying their mana costs*).

## Analysis

Gerund-participials occur very freely as complement of a preposition (CGEL,
Ch. 14, §8.3, p. 1262, [21]). *Instead of* is the compound-PP host recorded in
the [Instead post-landing fix](../done/english-v3-instead-replacement.md#post-landing-fix-2026-10-07);
its nominal Complement route stays.

## Witnesses

Each witness's failing units all match this construction and no other recon
STRONG bucket (`pvmzprszmnpt`, 0 Readings each):

- Storm of Memories: "You may cast it without paying its mana cost."
- Spellweaver Volute: "You may cast the copy without paying its mana cost."
  (control "You may cast the copy." 1)
- Knowledge Exploitation: "You may cast that card without paying its mana
  cost."
- Syncopate: "If that spell is countered this way, exile it instead of putting
  it into its owner's graveyard."
- The Five Doctors: "If this spell was kicked, put those cards onto the
  battlefield instead of putting them into your hand."

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. Verification budget (binding): One full census (`--all --workers 12
   --samples-per-face 0`) on the finished feature for the record, compared
   face by face (per-face `readings_sha256` and counts) against the claim
   parent's census; iterate on selector subsets, never on --all. Tests:
   `cargo test -p deckmaste_english_v3` and clippy on it; plus
   `-p deckmaste_lexical_source` only if core.ron/verbs.ron changed. No xtask
   tests, no Lean checks, no post-refresh re-census unless refresh touched
   grammar or lexicon. New tests go under tests/suite/ registered in
   tests/main.rs. Evidence lives under the workspace's ignored
   `target/english-v3/`, never `/tmp`.
2. Write each witness as a test first.
3. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
4. Compare per-face Reading counts against the claim-parent census. Any
   decrease on a previously covered face is restored, or named and justified in
   the landing record; a decrease inherited from trunk is attributed by change
   id.
5. Spot-check at least 10 newly covered faces and list them by name with their
   Reading. The gerund read as a noun Premodifier or nominal Object, or
   the PP attached inside the Object NP, is a defect. A wrong analysis that parses is a defect, not a gain.
6. No guard may name a lexeme, construction, verb, noun, preposition or card;
   read declared features only. Tests are re-spelled, never deleted. No new
   xtask subcommands, flags, fixtures or tooling.
7. May edit `grammar:` frame fields in `plugins_v2` keyword actions and lexicon
   frames in `crates/deckmaste_lexical_source/lexicon/*.ron`; plugin bodies
   untouched. Lexicon additions must be attested in the supported corpus
   (pruning rule).
8. A new frame or construction must not overlap an existing one on the same
   string (two labels for one constituency is a spurious duplicate, not an
   ambiguity); when it supersedes one, retire the old route and re-spell its
   tests.
9. Retire a superseded route on both the lexicon and the grammar side; do not
   leave unreachable declarations.
10. A CGEL citation may back only what the cited passage itself says; a project
    or orchestrator ruling is cited as a ruling, never attributed to CGEL.
11. Timings as integer ns and ns/B, with host load and worker count.

## Out of scope

- *anywhere else* (*instead of putting it anywhere else*): *else* is a vocabulary
  gap; note the faces.
- *rather than paying*: `english-v3-rather-than-complements`.
- Gerunds with genitive Subjects.

## Landing record

In addition to the standard record: which prepositions gain the licence and
from which declared feature; the residual item discharged; Reading counts of
each witness; before/after counts stamped with change ids; timings as integer
ns and ns/B with host load and worker count.


Completed 2026-10-08. All before/after figures below are stamped by
**`oqrvnvwrsoxvnzwkqmmnsktssltmomlt` / 22,669 covered** and
**`zpmurnquqlpkunplvxmymyztmkxlxypw` / 23,005 covered**, respectively.
The claim-parent census is reused by exact source equivalence: all 1,697
recorded consumed-source hashes match the claimed base (`baseline-proof.json`).
The claim parent `vlnpkroz` and completed base `vllryyok` have those same
consumed sources. Refresh imported coordinated-property-values, measured on
`owmovnyynvrxuqynvupomwvtrqnmuutx` / 22,853 covered. All 1,697 source hashes
matched the first refreshed claim. The later follow-up `rwvlplnl` changes
`declarations.rs`, `core.ron` and `native.rs`; all 1,697 source hashes match
the final refreshed claim (`refreshed-base-proof.json`). Its target (454),
retained (624), and gained (184) selector reports preserve every count and
Reading fingerprint relative to `owmovnyy`; the named reports and scope proof
are retained as `trunk-followup-*`. No second full trunk census is claimed.
The accepted full census remains the comparison reference. Its **184 gains and
624 structurally re-spelled Reading sets** are inherited from trunk, with
zero inherited count decreases. Its identity lists are in `trunk-comparison.json`;
the inherited selected analyses and follow-up scope proof are documented in the
[coordinated-property landing](../done/english-v3-coordinated-property-values.md#landing-record).
The registration conflict retained both modules. The adjacent Category conflict
retained trunk’s AdjectiveOrigin and this feature’s PP restriction. Harmony
and refresh finish cleanly; both conflicts were resolved by content.

### PROVE

The full run compares all 32,828 supported identities face by face,
including `readings_sha256`, counts and analyzed-source hashes. It gains
**336 faces overall**, including **152 feature gains** and
184 inherited trunk gains, loses **zero faces**, and removes **zero Readings**.
Every previously covered face retains its exact Reading count. The 624
inherited digest changes are attributed to `owmovnyy`; comparison against
the refreshed base has zero additional count or digest changes on covered
faces. `comparison.json` compares against the claim parent, and
`feature-comparison.json` isolates this feature against the refreshed base. No silent loss or invalid
Reading is counted as a gain.

| Census | Before | After |
| --- | ---: | ---: |
| No Reading | 10,159 | 9,823 |
| Unique Reading | 8,120 | 8,126 |
| Multiple Readings | 14,549 | 14,879 |
| Covered | 22,669 | 23,005 |
| Checked Readings | 956,021 | 1,131,832 |

Complete enumeration has zero limits, failures, internal failures, cyclic
derivations, duplicate Readings and validation issues. Every counted Reading
passes declaration admission, lexical ownership/context, byte-exact
realization, and construction/word traversal identity. The independently
constructed Without PP checks whole-value parse/render equality and exact
lexical leaf order. Licence-removal tests retain the admission exclusions;
nominal, predicative and coordination negatives exercise the declared function
restriction. All grammatical scope alternatives remain; destructive selection
and specificity arbitration are absent.

Exactly one full census ran, on the finished feature:

```sh
cargo xtask english-v3 --all --workers 12 --samples-per-face 0 --output target/english-v3/english-v3-gerund-preposition-complements/after.json
```

Iteration used the explicit selectors `--text-contains without` and
`--text-contains 'instead of'`, selecting 905 supported faces. The final
subset has zero additional Reading count or digest changes on previously
covered faces against the refreshed base;
its counts and digests match the full run for every selected identity.
No xtask tests, Lean checks or retired derived gate ran.

Forbidden word/card/lexeme/construction-named licensing guards: **zero** by
final diff review. Guards read declared features. Lexical/Grammar Environment
loading has zero fatal errors; the existing 87 unmapped declarations and nine
unsupported frame signatures are unchanged (`inventory-comparison.json`).
V3 does not emit the retired coverage-lock or licensing-checker totals; these
are not fabricated from the legacy pipeline.

### DISCLOSE

`GerundClauseComplement = Yes` newly licenses **Without** and **Of**. The
existing By and To licences remain. GerundComplementPreposition is reused:
the Complement is a Secondary Verb Phrase in GerundParticiple form, retaining
its selected Object and Adjuncts. Of remains the full selected PP inside
CompoundPrepositionPhrase; its nominal Complement route remains distinct.
The Instead gerund residual inherited from systemic residuals is discharged
as a grammatical construction, with remaining full-face blockers disclosed
below. No nominal or keyword route is superseded, so none is retired. The
candidate-only nominal permission was replaced on both grammar and lexicon
sides; no unreachable candidate declaration remains.

`ComplementFunctionRestriction` supplies complement-sensitive function
constraints in addition to the preposition's existing function licence.
Without declares Adjunct; Of declares Compound. Other Complement shapes keep
their prior functions. Coordination computes the shared restriction. The
Without restriction prevents Object-NP attachment; the Of restriction prevents
nominal and predicative use of its gerund Complement while retaining nominal
Of phrases, including their existing predicative use.
These restrictions are the ticket's project requirements (Goal and Method),
not CGEL claims. CGEL, Ch. 14 §8.3, p. 1262 [21] supports the general availability
of gerund-participial Complements of prepositions. The compound host and its
nominal route follow the recorded Instead landing.

The first selector exposed four new wrong Readings on Jazal Goldmane:
*the number of attacking creatures* treated *attacking* as a transitive verb
with *creatures* as Object. The complement-function restriction fixes this
within the ticket. Its document stays at **4 → 4** Readings, and the added test
checks that the gerund route is absent. A new test initially over-excluded
nominal predicative *of attacking creatures*; it was corrected to preserve
that nominal Reading and reject gerund predicative *of paying its mana cost*.
The implementation already preserved the nominal predicative route. Candidate
wrong Readings are retained as diagnostic evidence, never reported as gains.
No ticket/ruling contradiction or unresolved STOP remains.

| Witness | Localized Readings | Full-face Readings |
| --- | ---: | ---: |
| Storm of Memories | 0 → 3 | 0 → 369 |
| Spellweaver Volute | 0 → 3 | 0 → 204 |
| Knowledge Exploitation | 0 → 3 | 0 → 27 |
| Syncopate | 0 → 6 | 0 → 12 |
| The Five Doctors | 0 → 3 | 0 → 27 |

Eleven newly covered faces were inspected from their actual sampled trees,
including their gerund parents and selected Objects. Of gerunds are internal
Complements of the Compound PP; Without gerunds attach to a VP or clause,
never to the Object NP. Dying is an intransitive gerund; *being played* retains
its passive Complement. Full trees, word traces and owner paths are in
`spotchecks.json`.

| Spot-check | Full-face Readings | Reading |
| --- | ---: | --- |
| Syncopate | 12 | Of gerund inside CompoundPrepositionPhrase |
| Deny the Divine | 12 | Of gerund inside CompoundPrepositionPhrase |
| Reject | 24 | Of gerund inside CompoundPrepositionPhrase |
| The Five Doctors | 27 | Of gerund inside CompoundPrepositionPhrase |
| Storm of Memories | 369 | Without gerund as VP/clause Adjunct |
| Spellweaver Volute | 204 | Without gerund as VP/clause Adjunct |
| Knowledge Exploitation | 27 | Without gerund as VP/clause Adjunct |
| Three Tree Scribe | 1 | Without gerund as VP/clause Adjunct |
| Deep Gnome Terramancer | 44 | Without gerund as VP/clause Adjunct |
| Dour Port-Mage | 4 | Without gerund as VP/clause Adjunct |
| Imperial Cosmographer | 1 | Without gerund as VP/clause Adjunct |

Out-of-scope *anywhere else* remains a lexical gap on 15 faces:
The Eighth Doctor, Storm Herald, Llanowar Greenwidow, Dreams of the Dead, Nephalia Academy, Moira and Teshar, Whip of Erebos, Realmbreaker, the Invasion Tree, Isareth the Awakener, Kheru Lich Lord, Spirit-Sister's Call, Geth, Thane of Contracts, From the Catacombs, Elemental Expressionist, Gruesome Encore. These are retained with the systemic residuals;
no Else lexeme or unrelated construction is added. The full compound-gerund
residual list, including other unread hosts, is in `compound-residuals.json`.

**Deviations and additions**:

- Add the complement-function feature, its generic parent constraints and
  coordination propagation to prevent the two observed wrong analyses. No
  Construction, schema, Category, frame or verb declaration is added or deleted.
- Add five companion tests beyond the five named witnesses: independently
  authored roundtrip/traversal, lexical licence removal, complement-shape
  function permission, coordination exclusion, and Jazal's regression.
- Define Gerund-participial Clause, Gerund Clause Complement and Complement
  Function Restriction in the owning glossary, distinguishing CGEL evidence
  from project requirements. No glossary gap remains.
- Feature assurance counts relative to the refreshed claim: **restored 0, re-spelled 0,
  newly ignored 0, added 10, removed 0**. No inherited test is deleted or weakened. Imported property tests are
  accounted for in the sibling landing and follow-up records.

Every newly covered identity and its selected analysis follows:

| Face / identity | Readings | New selected analysis |
| --- | ---: | --- |
| Arcane Bombardment (`1e36a42f-58e2-49bc-8c14-7f75e1d8ef98#card`) | 2,160 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Arcane Heist (`839748e7-ccb4-421e-9626-b6d8be9390ab#card`) | 126 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Assert Authority (`085bc7be-bd44-40af-8e8f-a1a8006fe22c#card`) | 12 | compound PP: Instead → Of → gerund-participial Complement |
| Audacious Swap (`40b0b93d-4075-42d6-bcd8-b2f9910cc7d2#card`) | 16 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Baral's Expertise (`a1cdcb8c-1e12-4d39-aa4a-db678a80df9c#card`) | 38 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Bre of Clan Stoutarm (`59384481-3ca0-49dd-b94f-d3680b336f1e#card`) | 6,944 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Breaching Dragonstorm (`5370ee80-0354-454f-a16e-ec133e432ecb#card`) | 207 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Caravan Vigil (`925a3ded-b7bc-4384-8684-79c20c59ab12#card`) | 266 | compound PP: Instead → Of → gerund-participial Complement |
| Cho-Arrim Legate (`780c9cb2-9d98-4789-87e0-cb81db477204#card`) | 4 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Daring Waverider (`e4d52559-7624-48b2-96ba-e52ada6c507a#card`) | 378 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Deep Gnome Terramancer (`d4e3440d-4e34-40d7-8a42-c673225c0332#card`) | 44 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Deepwood Legate (`414eb407-dd39-41d2-bf25-a7a77ff35cb4#card`) | 8 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Deny Existence (`b665a64b-4772-42dd-9fd2-fd8598e689ba#card`) | 6 | compound PP: Instead → Of → gerund-participial Complement |
| Deny the Divine (`0146de73-29bf-415a-b450-11074553f715#card`) | 12 | compound PP: Instead → Of → gerund-participial Complement |
| Descendants' Path (`ba13cbdb-8043-4c1b-8a85-084c86886ac8#card`) | 280 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Devious Cover-Up (`feb221fb-59bf-4671-a53f-1bbe8e9c2ca9#card`) | 12 | compound PP: Instead → Of → gerund-participial Complement |
| Diluvian Primordial (`f21ce158-2925-4658-ab7b-b73718d31965#card`) | 300 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Dissipate (`4b2e9aa9-6f91-41de-84b6-e9a89be06a53#card`) | 6 | compound PP: Instead → Of → gerund-participial Complement |
| Diviner of Mist (`abf4dcd8-5176-4f4a-b7c1-5ce24ff181bf#card`) | 312 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Djinn of Wishes (`a34c3012-f787-4998-93c7-89c4decaa1ba#card`) | 30 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Dour Port-Mage (`cf58e309-00e8-438e-813e-2e1c1002db23#card`) | 4 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Dracogenesis (`9b06f2bf-146d-4b30-b467-e16f1a942253#card`) | 3 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Dream Harvest (`70b9497f-0dd6-4b66-9675-57c7041fe3c0#card`) | 1,584 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Efreet Flamepainter (`fb5e8b8c-bad2-45bb-abb4-2ed454525749#card`) | 360 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Electrodominance (`4a59d86e-405d-4c70-b2a5-c64f5cecfd9e#card`) | 19 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Emergent Ultimatum (`0eecdfb3-3b05-4051-a660-060ff6df80ef#card`) | 78 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Epic Experiment (`08bc10fd-8901-4836-ad4d-a867e1bcba77#card`) | 93 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Epistolary Librarian (`cb4f65b0-2841-42bf-8e18-d04a3b9f1a76#card`) | 34 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Etali, Primal Storm (`078def07-ae5d-4591-8db6-d156834aab97#card`) | 516 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Evercoat Ursine (`65c32889-3d70-449f-af76-05385c78c5a9#card`) | 30 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Extract Brain (`bb0eb9c2-c26d-452d-9e02-fc0bec2655c4#card`) | 27 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Eye of the Storm (`01f8db27-1191-488e-b0d5-172821613c15#card`) | 232 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Faerie Trickery (`bbe370ba-412b-4052-89d9-2d0ae4928118#card`) | 6 | compound PP: Instead → Of → gerund-participial Complement |
| Fallen Shinobi (`4fc5f871-a809-4a4a-adfb-eb4cd52837c8#card`) | 36 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Feather, the Redeemed (`aa219936-661b-4ccb-8741-78b70cff2b1a#card`) | 756 | compound PP: Instead → Of → gerund-participial Complement |
| Fevered Suspicion (`51545f6e-d0e3-4b4c-bc73-c731f85e26b0#card`) | 192 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Flawless Forgery (`0e3cb267-5704-4708-94c8-e2fbce4978af#card`) | 42 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Founding the Third Path (`0eff910a-712d-4652-8b3b-9eddf63eb891#card`) | 693 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Galvanoth (`0b4fa1ba-9597-4db1-a77c-975ed839f57f#card`) | 36 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Gix, Yawgmoth Praetor (`928d977e-cff0-4e0e-83bb-16d73a754f35#card`) | 1,368 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Glorious Purpose (`182baad0-ec6e-4e93-9dfc-7c80b37005de#card`) | 252 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Goblin Dark-Dwellers (`bab0ab8d-74d2-49a3-b258-b93d19925d99#card`) | 378 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Golos, Tireless Pilgrim (`efe9c3e0-7315-408e-9903-9d25a681039b#card`) | 660 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Great Intelligence's Plan (`cb9abc41-07f8-4a06-b7b9-9dd6be526e42#card`) | 14 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Halo Forager (`210d1077-6e60-4f06-a8a4-10b842979ac5#card`) | 378 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Hazoret's Undying Fury (`af4d3489-6bba-4538-928c-50c9a7073813#card`) | 372 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Hellcarver Demon (`90514aa5-84b6-4be7-b6d0-b05529c97140#card`) | 3,294 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Herald of Amity (`6c25c6b5-d98c-4eb4-aacd-b6960a8779a9#card`) | 216 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Hidetsugu and Kairi (`18591d9e-c9f5-4ae8-aee1-8580bc8fa600#card`) | 48 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Horde of Notions (`a7656948-4d7d-47f7-b8a1-fc9716155e37#card`) | 12 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Horribly Awry (`9cac23a1-a0b3-490c-aea2-dc2928f6dc9e#card`) | 24 | compound PP: Instead → Of → gerund-participial Complement |
| Idol of Endurance (`6b946317-8a66-41b7-b902-308784d69b90#card`) | 7,750 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Ignite the Future (`915aa871-4c5e-4799-9a07-d7b855df4e7a#card`) | 42 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Imperial Cosmographer (`b2aa052c-08b7-4334-9ea7-e78da57ff9ea#card`) | 1 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Improvisation Capstone (`fd4f0315-567a-4cc5-bc7e-88a9d2cee910#card`) | 876 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Impulsivity (`02b6900b-219f-4832-8f93-6ef27ee76c0c#card`) | 120 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Invasion of Alara (`8faa4330-07bb-4b65-810a-bbce29d3cd42#face:0`) | 816 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Invoke Calamity (`5b3f041e-ad4a-47ea-bdc4-1be2353f2e18#card`) | 93 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Izzet Chemister (`e4a3d6f3-36ba-4e6d-a254-351bc8886f20#card`) | 162 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Jace's Mindseeker (`8ebf337d-eac0-40de-978c-3a8fc3bbe03c#card`) | 27 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Jeleva, Nephalia's Scourge (`a014f283-c531-415c-ac00-e6773ea5d64d#card`) | 2,460 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Jetsam (`b0c28b2b-a2dd-4b76-bb1c-cec55a0a6784#face:1`) | 54 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Kaho, Minamo Historian (`395a7bd6-7c2d-448e-842c-ca53256d7008#card`) | 1,140 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Kari Zev's Expertise (`e0de0121-6185-45c5-9d88-a415b07347e0#card`) | 190 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Kefka, Dancing Mad (`fe5690d3-547b-4ce9-8e94-77fdc0e9c5c6#card`) | 4,392 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Knowledge Exploitation (`61ad3299-474a-400f-a1c1-b86024aca3e8#card`) | 27 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Knowledge Pool (`cb53bd72-dee8-46fa-b0f9-1eee3a99379a#card`) | 2,004 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Kotis, the Fangkeeper (`2eb2f6a2-e31a-4d3e-93ce-100394899014#card`) | 1,116 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Kyren Legate (`41b46aa5-6091-4bc4-8d17-d2c6b5fe1c5e#card`) | 4 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Leaf-Crowned Elder (`9416fd24-15a2-46d0-8b5f-55f5e7b65465#card`) | 64 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Liquify (`dc9ae094-139f-4f28-9d4e-4d6def765744#card`) | 18 | compound PP: Instead → Of → gerund-participial Complement |
| Lobelia, Defender of Bag End (`0f2e3e2e-9857-4be5-9bf0-62da1cc147fc#card`) | 3,344 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Maelstrom Archangel (`ac50ef98-1791-4dd4-9c1f-8cea7db7ade5#card`) | 42 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Massacre (`ce1866e9-28de-4bcc-9999-11958e6c13a6#card`) | 8 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Memory Plunder (`36b67014-0507-4976-ad52-a2a36676197d#card`) | 42 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Mind's Desire (`b3739b5a-5731-4c64-a244-815a363b0d5c#card`) | 8 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Mind's Dilation (`d4b5e09b-9ca2-4b51-a4f8-080741810df8#card`) | 16 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Mindclaw Shaman (`018985f5-6a67-4f2f-855c-efa7254ad3f8#card`) | 27 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Mindleech Mass (`facc2d6f-3f28-4b87-b643-44fdf939a2b5#card`) | 42 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Mizzix's Mastery (`40362fe0-a1a9-4d76-8c35-eac474b91af5#card`) | 90 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Mnemonic Deluge (`56de4cdc-4db0-48ec-8a16-f66086507233#card`) | 27 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Mogg Salvage (`0923ad85-2ea4-4463-8f71-b7d0a162b486#card`) | 4 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Mysterious Stranger (`4cd2e1c2-5f14-4ac9-9c88-899af6ad9f80#card`) | 4,680 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Nahiri, Forged in Fury (`87966759-206a-4a2d-bf78-86f727c29972#card`) | 80 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Narset, Enlightened Master (`aeaf7768-ed67-487d-8506-e667babb88ba#card`) | 162 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| No Escape (`7c6e0198-edd8-42b8-ba1b-549e1713d188#card`) | 12 | compound PP: Instead → Of → gerund-participial Complement |
| No More Lies (`ead1ee6a-e0da-46d5-89e8-31e8c0270bc2#card`) | 12 | compound PP: Instead → Of → gerund-participial Complement |
| Omen Machine (`dbf05c40-7274-4aaa-94d7-1f28cbe15e09#card`) | 36 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Omniscience (`730e39e6-c61d-48b5-8827-bfd952bf1be7#card`) | 9 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Omnispell Adept (`b575449b-591d-4357-92b7-55137fc7966a#card`) | 27 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Oracle's Vault (`220e3298-1663-4731-b50f-247076e1328b#card`) | 112 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Panoptic Mirror (`505cb78d-b292-4d16-b3f2-110164b4cc93#card`) | 1,728 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Patrician's Scorn (`0dc9acfb-3f8b-4a34-b44c-5cc62689904b#card`) | 8 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Perception Bobblehead (`f0bcdc43-c43d-4ce7-a283-b8dcab4fc497#card`) | 114 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Plargg and Nassari (`9e769a65-7aa5-484d-9493-badb485b5964#card`) | 5,428 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Power Pack (`3d5c98ff-fcf1-42bd-9533-2648791a4f45#card`) | 2,664 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Powerbalance (`8be368dc-6212-4627-989e-077eea78f30d#card`) | 28 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Refreshing Rain (`c6f69168-71c2-41a2-ab95-cfd7baf0ba68#card`) | 4 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Reject (`198fed4c-297e-41c0-a172-e471c7401fbb#card`) | 24 | compound PP: Instead → Of → gerund-participial Complement |
| Renegade Bull (`61747805-0c34-4beb-8cbf-97bac7a998d0#card`) | 1,188 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Reversal of Fortune (`938ba053-35f6-4424-a978-bf915709572c#card`) | 44 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Rishkar's Expertise (`97407cd0-2bd2-4074-94d3-4ec3d243fa78#card`) | 171 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| River Song's Diary (`60a63eb4-b0f7-4930-9bf6-afd42261d4d0#card`) | 3,024 | compound PP: Instead → Of → gerund-participial Complement |
| Rod of Absorption (`d32345c1-ed20-4bfc-a5fa-6ac7b99542ec#card`) | 6,858 | compound PP: Instead → Of → gerund-participial Complement |
| Rona, Tolarian Obliterator (`cf5bd3a4-3899-4c25-ad1f-f1778c1d9c60#face:1`) | 1,080 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Rushwood Legate (`f3623759-bf3a-4846-9ef7-1b732ad4355f#card`) | 4 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Saprazzan Legate (`d20b62fc-e248-4c84-87ac-7319b92531d4#card`) | 4 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Scarlet Witch, Chaotic Avenger (`8c72d54e-97a6-408e-ae77-18d3c96b3bd8#card`) | 40,590 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Scholar of the Lost Trove (`5f1e9098-f554-4505-974b-cef4b4b7b23d#card`) | 600 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Seifer Almasy (`7088a901-0489-41c3-9f2f-633939514de8#card`) | 3,402 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Shell of the Last Kappa (`a8b24b38-019c-45e7-a5f4-a1bc3014f7a2#card`) | 270 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Shiko, Paragon of the Way (`d3bdd3f2-9e38-436a-93a3-f87d9f4d4f86#card`) | 60 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Silent-Blade Oni (`4f515250-0f1f-4fb7-aa75-b703b81ccd4a#card`) | 27 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Sins of the Past (`9059a940-ac28-4322-a11b-af1d107b2edd#card`) | 120 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Sivvi's Ruse (`c5edfaae-3ecd-41ad-970f-8cecb09bf1e4#card`) | 88 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Solstice Revelations (`310c1280-dee8-426f-8440-6132a634a1e0#card`) | 576 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Sorcerous Squall (`a986d83d-22f1-45c3-bc2d-f5b210c539fb#card`) | 180 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Spawnsire of Ulamog (`b90d2f4d-b4ea-40af-aea0-1ab4234ab80f#card`) | 104 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Spell Shrivel (`bfaf376f-90f9-45d9-bfbe-dd84a2a4688b#card`) | 12 | compound PP: Instead → Of → gerund-participial Complement |
| Spellshift (`640fc475-5708-4ee6-a1a9-8f0722e22b48#card`) | 252 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Spelltwine (`307e68e0-5095-474c-af02-bfcb2e463763#card`) | 1,005 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Spellweaver Volute (`3d92e5ce-9d30-4203-9f84-6c36117440cf#card`) | 204 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Sram's Expertise (`5cd17347-5558-46f9-b8bb-ae5deeb1f229#card`) | 19 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Stolen Goods (`9294d4d3-0df0-4070-9261-06f15b758a0d#card`) | 64 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Storm of Memories (`6f8ca795-d6fd-4e3e-911c-c621a942acbb#card`) | 369 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Submerge (`99427ebe-c00d-4206-84ca-9764f6e952c6#card`) | 4 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Sunbird's Invocation (`10d482fd-e034-4187-b148-51fe15885360#card`) | 708 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Sunforger (`d1421070-a3cc-4af3-bf91-0f97372f4161#card`) | 99 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Surtland Elementalist (`b1a3b2ab-d9f8-4a0a-b2e2-aaa9989b2e41#card`) | 480 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Sword of Once and Future (`1e0cf860-6be6-4b4b-9a67-0c19b6018ae1#card`) | 1,134 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Syncopate (`318f7f70-e374-40ef-8afb-3389c10461d8#card`) | 12 | compound PP: Instead → Of → gerund-participial Complement |
| The Dawning Archaic (`a3e10b9b-9349-4b44-a46c-c825293dbd05#card`) | 3,240 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| The Five Doctors (`dbdd93d2-f2c6-43f9-93e6-2185c7df83bc#card`) | 27 | compound PP: Instead → Of → gerund-participial Complement |
| The Great Synthesis (`b70c3adb-64ba-49ea-bcf6-6406a58cb453#face:1`) | 432 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Three Tree Scribe (`7d2a7741-1dff-4fac-92d8-abea24f23f97#card`) | 1 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Thunderblade Charge (`b4f31434-b7c4-489b-9385-70ee0cc3125e#card`) | 12 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Torrential Gearhulk (`a6395447-677d-4c39-8eda-2d57e527c94e#card`) | 54 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Transcendent Dragon (`e83a629e-2d74-48e2-ad4d-f390067cc51a#card`) | 88 | compound PP: Instead → Of → gerund-participial Complement |
| Transforming Flourish (`9fb7e7ca-ad2f-4677-859c-7a9adf8fb5fd#card`) | 1,068 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Unexpected Results (`048bdc4a-4d26-420c-88ed-23a073c30fff#card`) | 32 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Urza, Lord High Artificer (`e87906d2-db1a-4e19-b910-adb4eb339945#card`) | 384 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Victor Timely, Wily Tycoon (`2ad77eb7-9466-432f-87b4-e80e4b66e143#card`) | 378 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Villainous Wealth (`f2fae95d-074b-4c0c-99cd-f9385a424c05#card`) | 93 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Void Shatter (`3d9f854e-1ee5-4aa1-a33b-d3ae08f15dd0#card`) | 6 | compound PP: Instead → Of → gerund-participial Complement |
| Wanda's Vision (`1448eadb-e8b1-4e8f-9697-3bb919085dab#card`) | 276 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Weftwalking (`08af6751-5848-47fb-95f5-57215e899460#card`) | 128 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Wild Evocation (`9cf1af53-ab9f-4cce-bdb5-2c13b805523d#card`) | 108 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Wildfire Devils (`b446b485-c10b-4175-bb49-16ac2f9e7718#card`) | 81 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Wildfire Eternal (`195736a9-d8b0-4c74-abe9-0289aa83c67d#card`) | 80 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Wondrous Crucible (`d0e80414-abde-4f5e-9539-3efc83aa507f#card`) | 78 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| World War Hulk (`8c4da760-8913-46ca-b00c-bba6d877d482#card`) | 64 | Without PP: gerund-participial Complement; VP/clause Adjunct |
| Yahenni's Expertise (`692babfa-9b74-415e-a560-abae4ccb006b#card`) | 38 | Without PP: gerund-participial Complement; VP/clause Adjunct |

### REPORT

At the stamped trees: **207 → 206 ordinary Constructions**, **51 → 52 schemas**,
**150 → 151 Categories**, **798 → 800 compiled Productions**, and
**3,694 → 3,831 nonblank declaration lines**. Ordinary
Constructions plus schemas remain **258**. Those structural count changes
are imported property work; this feature adds no Construction, schema,
Category or compiled Production. The retired lock covered field is
not a v3 metric; the validated corpus covered counts above are authoritative.

Inventories are unchanged: **38,404 spellings**, **1,139 named homographs**,
**1,092 case-folded homographs**, **23 form literals**, and **zero**
literal/vocabulary overlaps. Their complete named lists are `homographs.json`,
`case-folded-homographs.json`, `form-literals.json` and
`form-literal-vocabulary-overlaps.json`. Changed lexical owners: **Of, Without**.
No owner, spelling or frame is added or retired. Corpus SHA-256 is
`49dc966bda6ef588fc68e8d6972de25df1b660ed2ea654904584698263007ebb` in both reports. Lexical inventory hashes and complete
source snapshots are retained in the census reports.

Performance advisory, integer nanoseconds; 12 workers in both runs:

| Tree / covered | Corpus wall ns | Checked-text thread CPU ns/B | Host load (1 / 5 / 15 minute) |
| --- | ---: | ---: | --- |
| `oqrvnvwrsoxvnzwkqmmnsktssltmomlt` / 22,669 | 101,746,289,340 | 421,106 ns/B | `[0.712890625, 3.2890625, 3.72607421875]` |
| `zpmurnquqlpkunplvxmymyztmkxlxypw` / 23,005 | 130,576,279,486 | 468,576 ns/B | `[1.70263671875, 2.47412109375, 4.13720703125]` |

Both exceed the 16,260,000,000 ns quiet-host advisory ceiling; host loads and
worker counts above bound the comparison. Setup wall time is
2,258,207,185 → 2,261,107,388 ns. Measurements are provenance,
not a fitted performance gate. All evidence is ignored under
`target/english-v3/english-v3-gerund-preposition-complements/`, retained there
in the coordinator on workspace retirement.

Validation: `cargo test -p deckmaste_english_v3 -p deckmaste_lexical_source`
passes **454 English tests and 57 lexical-source tests**, zero failures and
zero ignored tests. `cargo clippy -p deckmaste_english_v3 -p deckmaste_lexical_source
--tests -- -D warnings` passes. Changed-file nightly formatting passes.
Citation check and piped citation audit results are recorded below before
integration. No plugin definition/body or Lean source changes.

Citation checks: `cargo xtask cite check --list-noncompliant` reports **0**
non-compliant strings; `cargo xtask cite check` checks **16,311** citations
with **0 stale**. The full feature diff was piped into
`cargo xtask cite audit --diff`; it selects **0 changed CR citation sites**.
No CR citation is introduced or changed, so no bless operation is needed.
