---
needs: []
---
# Read explicit partitive *each of*: each of them, each of up to two target creatures

## Why

*Each of* never reads. On change `xxknlzypsnwy` (32,828 supported faces, 17,322
covered, 15,506 unread; recon of 2026-10-07), it touches **356** unread faces
and is the sole cause on **146** (*each of them/those/these* 142 touched, *each
of your …* 99, *each of up to/one/two … target* 90). Counts are unread faces
*touched* (at least one localised failing unit matches) / *sole* (every failing
unit matches and no other recon STRONG bucket does). They are surface counts,
not gain forecasts.

Probes (admitted roots): "Put a +1/+1 counter on each creature." 1; "… on each
of them." 0; "… on each of those creatures." 0; "… on each of two target
creatures." 0; "Each of them gets +1/+1." 0; "Draw a card for each of them." 0;
"You may play an additional land on each turn." 1; "… on each of your turns." 0.

## Goal

*each of NP* reads as an explicitly partitive fused-head NP: *each* as fused
determiner-head with an *of* + partitive-oblique Complement, in every NP
position where *each N* already reads (Object, Subject, Complement of *on*,
*to*, *for*, *from*). The partitive oblique is a plural NP: a pronoun (*them*),
a determined NP (*those creatures*, *your turns*, *your opponents*) or a
targeted NP (*up to two target creatures*). Singular agreement follows the
fused head (*Each of them gets*).

## Analysis

In the explicitly partitive fused-head construction the head is followed by a
Complement of *of* + a partitive oblique, and the matrix NP denotes a subset of
the set the oblique denotes (*some of the books*, *all of them*; CGEL, Ch. 5,
§9.1, p. 411, [6]). CGEL places *each* with the determinatives that occur in
this partitive subtype (§9.2, p. 413). Targeted obliques (*up to two target
creatures*) are Oracle-specific; admitting them is the project's decision and
is recorded as such.

## Witnesses

Each witness's failing units all match this construction and no other recon
STRONG bucket (`xxknlzypsnwy`):

- Succumb to the Cold: "Put a stun counter on each of them."
- Involuntary Cooldown: "Put two stun counters on each of them."
- Reap What Is Sown: "Put a +1/+1 counter on each of up to three target
  creatures."
- Felidar Savior: "When this creature enters, put a +1/+1 counter on each of up
  to two other target creatures you control."
- Exploration: "You may play an additional land on each of your turns."
- Absolute Virtue: "You have protection from each of your opponents."

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. Before: `cargo xtask english-v3 --all --workers 12 --samples-per-face 0
   --output target/english-v3/english-v3-each-of-partitive-before.json` on the
   claim parent, stamped with its change id and covered count; after: the same
   command to `target/english-v3/english-v3-each-of-partitive-after.json` on
   the final tree. Evidence lives under the workspace's ignored
   `target/english-v3/`, never `/tmp`.
2. Write each witness as a test first. Iterate on `--face-id` selectors; verify
   on `--all` at the end.
3. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
4. Compare per-face Reading counts against the claim-parent census. Any
   decrease on a previously covered face is restored, or named and justified in
   the landing record.
5. Spot-check at least 10 newly covered faces and list them by name with their
   Reading. *of them* read as a postmodifier of a non-fused *each*, or plural
   agreement with *each of them*, is a defect. A wrong analysis that parses is
   a defect, not a gain.
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
12. Gate: `cargo xtask gate --changed --from <claim> --run`.

## Out of scope

- Distributive *each* after a plural Subject (*they each get*; recon bucket 74
  / 39) and quantificational adjunct *each* (§9.2, p. 413).
- Other partitive heads (*one of*, *any of*, *all of*, *none of*): note how
  many read after this change; widen only if the same declared feature licenses
  them, and record it.
- Coordinated targeted obliques (*up to one target creature, up to one target
  player, and/or …*): composition of the oblique is not built here.

## Landing record

In addition to the standard record: the fused-head structure and the feature
that licenses *each* in it; agreement evidence; per oblique type (pronoun /
determined / targeted) before/after counts stamped with change ids; timings as
integer ns and ns/B with host load and worker count.

Completed 2026-10-07 on `tttowsmuymqzuptxnqruknxyovrrvzrk` (`tttowsmu`). All final figures below
are stamped **`tttowsmu` / 20,030 covered**; baseline figures are stamped
**`tttowsmu` / 19,826 covered**, whose consumed grammar and lexical sources
are identical to claim parent `knmouqmzpspztunzuonnqvkwsossssmz`. The baseline
reports the claimant because witness tests had already been written. The ticket's
historical recon is not this landing's baseline. Evidence was produced exclusively
in this workspace's ignored `target/english-v3/`; after integration it is preserved
under the coordinator workspace's ignored
`target/english-v3/english-v3-each-of-partitive-tttowsmu/`.

### PROVE

On the same 32,828 supported faces: **204 gains, zero lost faces, zero decreased
Reading counts on previously covered faces**. No covered identity or Reading is
retired, regressed or owed re-coverage. `each-of-delta.json` records the identity
comparison; the complete gain list and representative analysis are below.

The whole-corpus command was `cargo xtask english-v3 --all --workers 6
--samples-per-face 0 --output target/english-v3/each-of-after.json` (with
`each-of-before.json` for the baseline). All **597,766 complete
Readings** pass declaration admission, lexical ownership/context, byte-exact
realization and construction/leaf traversal validation. Zero unresolved census
states, validation issues, duplicate derivations, cyclic derivations and internal
failures. Grammatical ambiguity is retained under the current ADR; destructive
specificity resolution is zero. Independent expected NP and Clause values prove
both roundtrip laws and exact node/word traversal equality.

`each of them` has one independently constructed Reading, containing a lexical
Determinative fused head and a selected `PrepositionPhrase(of,
AccusativePhrase(AccusativePronoun(them)))`, with no omitted noun or nominal
Postmodifier analysis. The lexical permission is the existing
`NominalComplementMarker = Of`, paired with the Complement's marker;
`DeterminerUse = SingularCount` licenses singular matrix Number while the
existing `ObliqueNumber = Plural` constrains the oblique. `Each of them gets
+1/+1` admits exactly the expected singular-third Clause; the independently
constructed plural Clause fails admission and realization and its text has no
Reading. The six ticket witnesses pass, as do attested Object, Subject and
on/to/for/from positions.

No overlapping route is added: the existing `PartitiveNounPhrase` has a Cardinal
head, while the new route has a licensed lexical Determinative. Their strings
do not overlap. No old route is superseded and no obsolete lexical frame or
grammar declaration is left behind. Independent expected sets retain exactly
one projection for `two other target creatures`, preserve the existing targeting
scope distinctions, and retain the singular direct/plural nominal boundary.

Forbidden word-, lexeme-, card- or construction-named admission guards added:
**zero**. Admission reads declared features. Source/environment loading has zero
errors; lexical export checks **42,538** independent values (42,534 at baseline).
V3 has no legacy `environment.rs`, coverage lock or emitted legacy permitted
licensing-checker total; those obsolete counters are not substituted for its
source loader and declaration admission. The unchanged legacy coverage data is
not v3 admission authority. The 10 source frames outside the current grammar
remain unchanged; no unsupported frame is introduced.

Assurance: **12 tests added; zero restored, re-spelled, ignored or removed**
relative to the claim parent. Six separately named tests retain the ticket
witnesses; four additional tests establish independent values, agreement,
negative licences and shared NP positions, and one added test in the existing targeting suite
checks adjective scope above the marker. One lexical test independently
checks the native word-compound owner and exact singular/plural values. No
pre-existing test is weakened.

Gate: `cargo xtask gate --changed --from knmouqmz --clippy --run` derives:

```sh
cargo test -p deckmaste_lexical_source -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask
cargo clippy -p deckmaste_lexical_source -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask --all-targets -- -D warnings
```

Both derived commands pass, including all Lean integration tests and Clippy with warnings denied. Formatting passes. Citation checking reports zero noncompliant
strings and zero stale citations; the actual jj diff was piped into `cite audit
--diff` and contains zero changed citation sites. No citation blessing is needed.

### DISCLOSE

| Measured tree / covered | No Reading | Unique | Multiple | Complete Readings |
|---|---:|---:|---:|---:|
| `tttowsmu` baseline / 19,826 | 13,002 | 7,331 | 12,495 | 584,148 |
| `tttowsmu` final / 20,030 | 12,798 | 7,371 | 12,659 | 597,766 |

Specificity-resolved selections remain zero. A cheapest sample is a presentation
representative and removes no Reading. All 204 gains were re-enumerated with
one retained sample per face, validating their 13,618 Readings without issues.
Of these, 140 faces have the new fused-head analysis; 64 gain the existing licensed
adjective premodification above a targeting marker.

Oblique-type surface buckets over reminder-stripped source, using the same
baseline/final stamps above (faces may occur in more than one bucket):

| Oblique type | Faces containing it | Baseline covered | Final covered | Baseline / final Readings on those faces |
|---|---:|---:|---:|---:|
| Pronoun | 70 | 1 | 22 | 2 / 7,066 |
| Determined | 204 | 0 | 68 | 0 / 3,087 |
| Targeted / counted targets | 86 | 0 | 47 | 0 / 507 |

These are face/source buckets, not disjoint phrase inventories or a claim that
all contained phrases already parsed at baseline. The pronoun baseline includes
Hunted by The Family's existing generic label Reading. The reproducible bucket
patterns are in `compare-censuses.py` alongside the reports.

Other partitive-head face/source census, with the same stamps: *one of* occurs
on 297 faces, covered 65 → 66; *any of* on 19, covered 0 → 0; *all of* on 8,
covered 0 → 0; *none of* on 3, covered 0 → 0. Ezuri's Predation supplies the one
additional *one of* face because its *Each of those tokens* now reads; the
existing *one of those creatures* route is unchanged. Direct NP probes give
*one of them*: one Reading, and *any/all/none of them*: zero. No other fused-head
licence is widened.

Manually inspected representative Readings (all final stamp above):

| Face | Readings | Selected analysis of the relevant constituent |
|---|---:|---|
| Succumb to the Cold | 2 | Fused each + selected of + accusative plural them, inside on PP. |
| Involuntary Cooldown | 2 | The same fused NP with a plural pronoun oblique, inside on PP. |
| Reap What Is Sown | 1 | Fused each + of + counted NP; nested up/to supplies its quantitative Determiner over targeted plural creatures. |
| Felidar Savior | 3 | The counted oblique retains other above the targeting marker and the you-control Relative Clause; grammatical attachment scopes survive. |
| Exploration | 5 | Fused each + of + possessive plural turns, Complement of on. |
| Absolute Virtue | 1 | Fused each + of + possessive plural opponents, inside protection's from Quality PP. |
| Blessing of the Nephilim | 2 | Fused each + of + possessive plural colors, Complement of for. |
| Stall for Time | 1 | Fused each + of + those-determined plural creatures, inside on PP. |
| Meteor Blast | 1 | Fused each + of + X-counted plural Target Noun, inside to PP. |
| Drakuseth, Maw of Flames | 1 | Fused each + of + up-to-two-counted other targets; targets retains its count-noun identity. |
| Hope and Glory | 2 | Fused each NP as Subject; singular-third gets with the SlashPair Complement and retained temporal scopes. |
| Marang River Regent | 2 | Existing PremodifiedNominal places other above TargetedNominal inside the counted Object. |

CGEL, Ch. 5 §§9.1–9.2, pp. 411–413 supports the explicit fused-head partitive
Complement and each's partitive use. The Oracle-specific admission of targeted
obliques is this ticket's project decision (Goal/Analysis), not a CGEL claim.

**Deviations and additions**:

- Use six census workers per the user's host-load instruction, overriding the
  ticket's historical twelve-worker command.
- Add `DeterminerHeadPartitiveNounPhrase`; reuse `NominalComplementMarker`, `DeterminerUse`,
  `ObliqueNumber`, `ThirdPersonCommonCase` and `PartitiveModifier`. No new
  policy, category or frame is declared.
- Add the orthogonal `TargetingPremodifierUse` feature and the
  `targeting_premodifier` and `coordinated_targeting_premodifier` tables. Declare its permission on lexical *other* and ordinal Adjective
  Phrases, and propagate it through existing adjective coordination. Existing
  features distinguish structure and targeting presence, but cannot express
  this positional licence. No identity guard is used.
- Permit licensed simple Adjective Phrases above `TargetedNominal` in
  `PremodifiedNominal`. Felidar Savior's pinned witness requires this
  composition; the old blanket Targeting prohibition made its plural oblique
  unread. Participial/noun restrictions and the targeting projection ruling
  are unchanged. Its A-labelled additional face gains are explicitly listed below.
- Add the attested *necrodermis counter* compound-noun recipe using the existing
  Counter head. It fixes a pre-existing vocabulary gap exposed by the added full
  Object-position witness from The War in Heaven. No plugin body is changed.
  That witness now reads; the complete card remains unread for other existing
  causes, so no whole-card gain is claimed for it.
- Add the six tests beyond the six ticket witnesses described above; no
  construction or pre-existing test is deleted.
- Clarify the existing glossary entry **Nominal Complement Marker** to include a
  fused determiner-head's selected Complement. This corrects its former
  noun-only wording. Define **Targeting Premodifier Use** as a project licence
  and cite this landing, without attributing that restriction to CGEL. No
  Game Model/CR claim is introduced.

STOPs: none. The two initial failing added witnesses were repaired in scope.
The first full gate caught the existing negative witness *White target
creatures attack.* when the adjective restriction was relaxed indiscriminately.
The feature licence repairs that regression while retaining the test unchanged.
The subsequent gate caught the new native word-compound using the plugin
counter-class owner prefix. Its declaration now uses the existing core
`CommonNoun` owner namespace; the original plugin inventory test and all 73
plugin-owned counter compounds are unchanged. A new independent lexical test
checks that native ownership and both number forms. Clippy then required the
new roundtrip helper to borrow its independently constructed Reading; its
assertions are unchanged. `clippy-before-gate.log` records that repaired style
finding.
The intermediate census, delta, samples and gate log prefixed `unlicensed-` are
superseded by the final census and gate. The `ownership-before-` reports and
`ownership-gate.log` precede the native-owner repair. `witnesses-licensed.log` supersedes the
earlier witness logs/reports. Refresh completed without conflicts. The refreshed base has no consumed grammar or lexical-source changes; the measured baseline therefore remains the refreshed-base census. There are no trunk Reading decreases to attribute.

### REPORT

V3 measured covered: **20,030**, from **19,826** at baseline (stamps above).
There is no v3 coverage lock. Source declarations: **205 ordinary Constructions
+ 43 shared schemas = 248 constructor names**, from 204 + 43 = 247. The 43
instance blocks instantiate 315 category instances, unchanged; the final
compiled grammar has 690 Productions. These are distinct inventories, not
additional constructor names or coverage gates.

`crates/deckmaste_english_v3/src/declarations.rs` has **3,259 nonblank lines**,
from claim-parent **3,227**, net **+32**. The new fused-head construction is the
only added construction. Added feature: **TargetingPremodifierUse**; tables: **targeting_premodifier**
and **coordinated_targeting_premodifier**; policies: **none**; frames: **none**.
The existing `AdjectiveStructureMerge` policy propagates the new permission;
`PremodifiedNominal` replaces its blanket restriction with the declared licence.
The two existing adjective categories carry that permission; no category is added.

Homograph surface inventory is **123**, unchanged. Declared-case independently
realized values exclude metadata Catalog entries and require distinct owners;
`homographs-before.json` and `homographs-after.json` name every owner. The added
compound contributes *necrodermis counter* and *necrodermis counters*, neither a
homograph. Form-literal/vocabulary overlap inventory: **none**, unchanged.
Homograph surfaces:

`'s`, `Adamant`, `Addendum`, `Alliance`, `Battalion`, `Bloodrush`, `Celebration`, `Channel`, `Chroma`, `Cohort`, `Constellation`, `Converge`, `Council's dilemma`, `Coven`, `Delirium`, `Descend 4`, `Descend 8`, `Disappear`, `Domain`, `Eerie`, `Eminence`, `Enrage`, `Fateful hour`, `Fathomless descent`, `Ferocious`, `Flurry`, `Formidable`, `Grandeur`, `Hellbent`, `Heroic`, `I`, `Imprint`, `Infusion`, `Inspired`, `Join forces`, `Kinship`, `Landfall`, `Lieutenant`, `Magecraft`, `Metalcraft`, `Morbid`, `More Than Meets the Eye`, `Opus`, `Pack tactics`, `Paradox`, `Parley`, `Radiance`, `Raid`, `Rally`, `Renew`, `Repartee`, `Revolt`, `Secret council`, `Spell mastery`, `Strive`, `Survival`, `Sweep`, `Tempting offer`, `Threshold`, `Undergrowth`, `Valiant`, `Vivid`, `Void`, `Will of the council`, `X`, `as`, `bottom`, `control`, `copies`, `copy`, `cost`, `costs`, `counter`, `counters`, `cycling`, `deathtouch`, `decayed`, `die`, `double strike`, `draw`, `draws`, `exalted`, `exile`, `exiles`, `first strike`, `flying`, `goaded`, `harnessed`, `haste`, `her`, `hexproof`, `his`, `if`, `indestructible`, `it`, `less`, `lifelink`, `menace`, `name`, `names`, `one`, `reach`, `shadow`, `solved`, `suspected`, `tapped`, `target`, `targets`, `that`, `then`, `time`, `to`, `top`, `trample`, `turn`, `turns`, `untap`, `untapped`, `up`, `vigilance`, `you`, `’s`, `∞`.

Performance advisory (whole-corpus measurements, six workers):

| Tree / covered | Corpus wall ns | Checked-text thread CPU ns/B | Host load (1 / 5 / 15 minutes) |
|---|---:|---:|---|
| `tttowsmu` baseline / 19,826 | 108,976,137,413 | 251,303 ns/B | 24.019 / 16.439 / 11.815 |
| `tttowsmu` final / 20,030 | 108,705,164,478 | 250,381 ns/B | 15.155 / 15.470 / 13.278 |

Both exceed the 16,260,000,000 ns quiet-host advisory ceiling. Measured wall time
changes by -0.2% and per-byte CPU by -0.4%; the host loads differ and the final
run overlaps focused compilation and the gate's test run. These are shared-host observations, not a
quiet-host speed certificate or a discarded gate result. Completed Readings
increase by 13,618; completion work is 20,556,916 → 20,754,825. No timing gate is
fitted to these figures.

### Newly covered identities and representative analyses

Each row names its durable face identity and one fully checked representative
Reading from `gains-samples.json`; its complete alternatives remain in the
census. **D** denotes the new `DeterminerHeadPartitiveNounPhrase` with lexical
each and a selected of PP over a plural oblique. **A** denotes the existing
`PremodifiedNominal` over `TargetedNominal` with a simple Adjective Phrase.
Both labels refer to the actual sampled tree; A includes lexical *other* and
ordinal Adjective Phrases. All rows have the final stamp
`tttowsmu` / 20,030 covered.

| Face | Identity | Complete Readings | Representative analysis | Reading fingerprint (SHA-256) |
|---|---|---:|---|---|
| Aang, the Last Airbender | `70564c3a-858f-498e-8b92-acb3ca54ae7e#card` | 150 | A | `02649418321448236c4fc84eb5dc3b3f1dcc1eb421bc13ca987935ac6e562ca3` |
| Aberrant Return | `3a92b235-196b-4f46-9d20-06f4d3653d36#card` | 63 | D | `006363a188a555ab71fa06d04dacec0d5b1b2cf56f55a6b53e42b3c7fe0541af` |
| Abigale, Eloquent First-Year | `48a773d0-c433-4463-b50f-b6d4604a042c#card` | 6 | A | `4f7798f3ff76fa8e76bc18aee52334d5be4dccbb0fb040c8a864a531dbe70126` |
| Absolute Virtue | `d4fddf20-6b3a-42c3-a245-002dacbb4725#card` | 1 | D | `7f89bf3a3adfe42e7e52d154d85a53dbe9a55bc39c171429a5a3bdac70a0f27a` |
| Aesi, Tyrant of Gyre Strait | `6511f317-bd38-46d0-b800-7125a3f420da#card` | 10 | D | `175ed87c7f2b58fe6e8714368d7f522a5afe8ec3e65272c3a6d71f06e3159300` |
| Age of Ultron | `51042581-4add-4487-a1b6-85446539b975#card` | 12 | D | `12eb598e78968420b0c3b21e25aff8f6df9c256b5d4f8be4badfe79be6810b35` |
| Agent Bishop, Man in Black | `f33217cc-175f-45e8-bc69-801f41cb4495#card` | 1 | D | `4dde30c8faecfabc982f8e6c40135611f33089e13f96fc873f6db32b4c0d5ee0` |
| Air-Cult Elemental | `6ec2d957-c6d9-469f-8190-8b621c490238#card` | 2 | A | `de55a85f9e1c24ef6eb0a1f93993df4516f7a5a69370d750c5242e7063a5fbc1` |
| Alpha Brawl | `cfc023e0-9e9d-48ac-bfb5-74bfd5977dee#card` | 27 | D | `106d6fdd35ff1d1cd272e9056d33527951f89570093b56586099b96b45834638` |
| Angelic Quartermaster | `14a555eb-9720-45b4-9eb5-77b739e1045c#card` | 1 | D | `98bb7f93e418c8b36a03a642d1e7aabdfddeb3084ec14ae3aedba9a8905d5a6c` |
| Archangel of Tithes | `ff5caed4-0276-476d-8fae-edae2536df7f#card` | 588 | D | `001ecac361f38812ac70db9bfb1f4fd1f2bd09baaec352b3f34b143242255131` |
| Archon of Absolution | `0e3ac581-3d78-4879-9afb-3db69c5c47a5#card` | 15 | D | `4b3f788efa6e7aa21fed19d77a43e40fff38be1f5028abe6eaa694e9ddbaee41` |
| Arm the Cathars | `c6a1a91f-46b9-4e41-a3d3-3423b02c42a5#card` | 4 | A | `191651b777b44f3f3ff29346308fcf4db2959db657370d8b8bd181fd8dd1a678` |
| Aurelia, the Law Above | `2a800427-ff8c-4b3c-baee-85211b70656d#card` | 4 | D | `04ced0bd8d60de1ec8f1335c34f3ee96381242e4ddc9868f3a5997f3b4be5e4e` |
| Avatar Yangchen | `521a63cf-5e83-4649-9800-c62b2fc474d6#face:1` | 4 | A | `6ca5452fe877082b841da805bf29dd2e32968cdaca3faea77606dd441a3a44f3` |
| Azusa, Lost but Seeking | `6c2c8bf3-9bf8-4a86-89d3-3bb36260dc51#card` | 5 | D | `14529eaaf468254e7e12b95087b0e36739a99ffdcecd3f77421d7e0c458f5051` |
| Baird, Steward of Argive | `69aa6809-5468-413a-94c9-562e6de93957#card` | 15 | D | `053e6dd5b6b315b5d9011b7ceb7e8d94f7331f3b0005f16ce6a84f568367a980` |
| Barrin, Tolarian Archmage | `f98f7cb7-90ca-41e7-9f1d-8054931ecf0e#card` | 10 | A | `089b0bf9ef938dae73a689a65543cbf78b5b3b27f089d2f708414f370c7eb7d4` |
| Basri's Acolyte | `92c824da-2351-43bd-9fa0-d09709ad8205#card` | 3 | D | `0ee244dbcb2af0f67bcac20e3526e074803316e5872fedde2eddc403f4d0de2b` |
| Basri's Aegis | `9825dab8-ca1a-44ea-b1d7-77c66f00351f#card` | 32 | D | `00b282161e52c02fa1b434c077ab7a4e7a48358d0029e26e8fbd89d8d4ccfa48` |
| Battle for Bretagard | `8294539d-7982-4ec9-8392-99eed78542f5#card` | 152 | D | `017ce4c6aedfb80c76e021d797e33f97ccd11eb8cef5948d2b8a74146da77906` |
| Bespoke Bō | `679480aa-f9ad-4a6c-bd79-db9f9f75433d#card` | 2 | A | `c852862e4cc57bb71af2b146e960483124b4e1c24d39c21f82a3ec254cf246e9` |
| Betrayal at the Vault | `f1dce3f0-30c8-488f-bbeb-b0533886c655#card` | 3 | D | `20eeea6605002e61c25ead6ea2d0cc0c127264b14d85d4d0cf17da3d2890d117` |
| Biogenic Upgrade | `30812870-87c0-469c-95a5-811d5a181e72#card` | 20 | D | `0b8277db75e272aefa3243f7c2ce0155dcabdff139505751c669462e3ed836a4` |
| Bionic Blow | `27683e43-7c3f-45b3-8ca4-968428000315#card` | 6 | A | `26c08948e081fb15bba2a78e97896c9723d787688b707c9142d0c4609744b906` |
| Blessing of the Nephilim | `0904cc0e-e2c9-4a85-89c7-4254fd423bf8#card` | 2 | D | `18d4e8d67483f45cfdfa739388d6f5d87fa1b5aad8409a156fbaefbdce90ff14` |
| Blue Dragon | `1d00e54a-5897-43dc-9c38-a0e2ce6a7433#card` | 3 | A | `633f860fa137700a2001cab0c9d167d5dfe12b9ff480fcde235f4847951b8666` |
| Breathe Your Last | `dc3af7cb-03b9-445f-82eb-18798ec92934#card` | 2 | D | `15f1c7460a64a4a690d7124ca01d4c848887027b89826c0be9f6e302da8f980f` |
| Broodlord | `e3a73311-25f7-4bd7-af1d-ab7b83ad01f2#card` | 13 | A | `3db3176be6bec1bab95845e22c3a8a526a418f4f8de0b18e390e7cfb12e76119` |
| Byrke, Long Ear of the Law | `e9c6e4e6-0c3f-4048-93e6-161a1221ea36#card` | 25 | D | `00030ec98fd57424c44bf18c56e15959ecccfff12a8aae44d327568bd34d2ff0` |
| Cast into the Fire | `b24ed296-e17d-4e36-8a86-a370868b0136#card` | 1 | D | `8f444c67842ec31210a1b736efcd4824947125cfe5d7d45ac5bb2e3e287379ca` |
| Cat Collector | `52f1b5a4-62ff-44de-80da-503d4e52d312#card` | 8 | D | `339d57bf2dc03415a858d6a0719ef3a87725963c185aee2bdf3d4f492266b613` |
| Cauldron Haze | `56309bbe-86aa-4003-a531-0ef1e318ba15#card` | 2 | D | `3a30e64d35f7e51cc9c80c895dbb2e279aa54a4d9410e9424df0a2aa91f9c371` |
| Cauldron of Souls | `5811d905-cae5-4f92-a78e-abb43f8864a7#card` | 2 | D | `85ec4968400531c869c49361190ad5263c1ec28e1f09999276bd3bc21f3f200b` |
| Chains of Mephistopheles | `eae87919-6322-4bd2-ae9c-b1ce25d686da#card` | 192 | D | `0009a1ec7ba8135c7c0bb914594b8395b322993dfa1e33422cbea59f188b06d0` |
| Chong and Lily, Nomads | `faffc9fb-32b3-4015-8c3f-e6b45002c050#card` | 18 | D | `03cd10d3eb8735d16cd6d238c2d76208e2b2542f99ec7d8d5f902fb3cb1448ac` |
| Civic Saber | `79441673-6e43-41bc-936c-958dbb9d92da#card` | 2 | D | `336e4a670fb7525fcbca53e1a5f3c38bace998fa25a3b0b2104f567898e654ba` |
| Cleopatra, Exiled Pharaoh | `e7a80464-1522-48d7-bf82-f9894f5c3ae0#card` | 70 | D | `00500658054f475152657817f18a87aefee7578522057269cff9c30d3e62fc12` |
| Cliffside Rescuer | `c53f2d1c-5a53-4020-aace-a1d9f75e47c7#card` | 4 | D | `6bb326e2a049a316434467ac33e4d9cdf45bdf534b70b44bfc2f10fe34e2b61e` |
| Cloudspire Skycycle | `f7467fce-61e4-401a-8a3d-b0a2bc347088#card` | 15 | A | `037fca0648de23e4c024d807705bc11be36d3dca30d5618e9565ebf4a21dd22f` |
| Constable of the Realm | `2a263e61-303f-449d-a431-c26cabdc68e0#card` | 7 | A | `0933bb53ff9521c022a63512e7af906240906629325e4bbe2b008923b128dd58` |
| Cunning Geysermage | `746768bc-9abc-495f-ad00-4b58497dce0d#card` | 2 | A | `cb3581029696dcdccb99c0b718d21d9e2557d97a933fbeb5a8135bb6701911f8` |
| Disruptor of Currents | `e76ae81f-729c-4def-ae83-6870a696cbe8#card` | 2 | A | `113a81cf02ce669a26af969a46a16dea6e48beeeb9f2c07a1ab6d193df0130c6` |
| Divergent Transformations | `f08e835a-7827-4fdc-8dcf-ae74507da052#card` | 10 | D | `005242f3a271421adee5a1289ea0b6e5c7fc6aba4157104e287ff8743877aa36` |
| Donatello, Rad Scientist | `671209d5-8b56-4ce7-94a0-d770d698c0df#card` | 8 | D | `201a29f85f5b7227a7082aab017c321edc0c58ec72f4079641c2452f4edfca47` |
| Doppelgang | `d04c6375-25dd-4882-b8b3-a3b0d3081f32#card` | 2 | D | `2d21e414cca205c312ddb8550ef4bb977e6112456074d4c0c0e2710f28ca59af` |
| Drakuseth, Maw of Flames | `060deaff-44d6-4f03-9568-bcb7add80255#card` | 1 | D | `b61e550776108969229632ad52795f937b7b9fc920ab1bab2a2664a2594f8bda` |
| Dryad of the Ilysian Grove | `bdbde5d0-f5e4-44da-b27c-b4ad6f374cc9#card` | 55 | D | `098b3098ae30ecd44cbde70081acef29d34569024ddba20c4a6e1f3db3b707cb` |
| Dual Shot | `e56056dc-637e-4229-ae2c-dfd06d24086b#card` | 1 | D | `d026fd5fab186097257877068603641811ab7f720b9c9007c2bf640c724b49bc` |
| Earnest Fellowship | `26f87a39-29ed-4649-b5d8-204f6a40a41d#card` | 1 | D | `00f63e360bfca5f0978d29ba83f0aec7a443a70f9849f198f36ac34445bf0c41` |
| Earth Kingdom Soldier | `4289f2d4-3db2-4dd1-9c18-af3f78309844#card` | 2 | D | `1c8d67d23c21caa16ef32004e4c4e4e8bb19689ec86e1afe4b1ec0bca12c6231` |
| Ever After | `86be501c-6f05-4ce0-adb0-7adb135bfb93#card` | 420 | D | `04ef0e5b140c7052705f2f25e336bb6b5c0f8593143207c1eb8e24c6100768f1` |
| Everything Pizza | `aa481983-8f1a-4519-9371-5e7edcd3274f#card` | 12 | D | `d838da43195539f4e35edd0775854d3e920d872d71fde91c2b593950941cc3c3` |
| Exosuit Savior | `8b13ec4d-812a-48b0-8a64-6847a694a03a#card` | 6 | A | `15ad76da87d4252fbf5c1fab64d90ee6569d3adaadb9cc8d398aee46a3beea19` |
| Exploration | `0c2841bb-038c-4fbf-8360-bc0a1522b58d#card` | 5 | D | `0740a14a4b85acdc1f363fd4a695b96214348f370387693a3dfbb54c74e0034d` |
| Ezuri's Predation | `d0dd425b-fdba-41b4-b9e6-f5161610bd7e#card` | 2 | D | `2b3c4b8f354d53b74bff27c00f5c253d10e7c176f5b78526c4cda66d284d4f54` |
| Fall of Gil-galad | `c248537f-e28f-47cb-9e47-a6eb05e1af63#card` | 1 | A | `6bfe60b57b3a09afe88e94357b2a5b9a8ce10d36a07bea98ae6a8039bbda8db0` |
| Fall of the Titans | `55e564b4-7c08-4109-bd67-da5f31fe002e#card` | 1 | D | `84608cf1078ef9f24ff1d5adb48e2f1825660e80d267d2a83387f56130212326` |
| Faller's Faithful | `df7500e8-76b4-49b3-a427-9a28ffa14637#card` | 2 | A | `00651b639304ace6befa39d587c11b7c53a78be487ce7d570f90cd22c0717898` |
| Fastbond | `e27193b7-1a47-4555-865d-b1fd4c6d597f#card` | 40 | D | `049e6a6dc25fe093513702549ee8602b520ec0676d81e7f75448e3d4af77d248` |
| Felidar Savior | `2bee2508-cd5d-4c62-a205-12d8bbb3143d#card` | 3 | D | `0ee244dbcb2af0f67bcac20e3526e074803316e5872fedde2eddc403f4d0de2b` |
| Filigree Vector | `44b90948-e85a-4f69-a9e2-aba63775ba40#card` | 20 | D | `266e0164e26f72a3750ffaa8c0dc006ebec50a2738f5bcb26f4393e7ec17d0ff` |
| Fire Shrine Keeper | `14537df5-e2c8-4906-99ac-742fb628fe08#card` | 1 | D | `6d3f63d601561635445c87e2856ca745a23e33266bc1cc96e1b790d4176b38e1` |
| Firestorm | `157813c8-af83-41b5-9ffb-4abd167c1bfc#card` | 1 | D | `930c1dbeac2b33686fa05f66f7938c53cf13e8dc78ee5e02d0953a9a7a1f56c9` |
| Fleeting Reflection | `c915f815-77d7-4e08-b178-82b29a58bb03#card` | 2 | A | `5f2b579a910e7623b039e69ce7be1ddd82de0dec52bf84872db3430322aab96c` |
| Flickering Hound | `162421d2-8761-437f-bed9-578b61c96f1f#card` | 222 | A | `20da070d65843d31abc6dc3c2dad392f14e885147c82041c1c5056692ba630df` |
| Flock Impostor | `6522070b-7002-44fd-8deb-832fe67ef9a9#card` | 6 | A | `2a924c0532a70b24a34f54bc3019a6d8face143aa80f1c300c109eba94ef3f0d` |
| Forbidding Spirit | `efd8dc6c-8233-45b2-99bd-e38ca2e0b35f#card` | 28 | D | `05d83d49fd29be70463aa3f475d7ef905cfb6064cfbd24b1e3801f592dba5f94` |
| Furious Reprisal | `38e1ab26-95f9-43dd-9bda-13c7d8ba47ea#card` | 1 | D | `2905246813ec67b21d39b5834e4032c61b95fb8ad7fd670433679e73999e25ab` |
| Gavony Silversmith | `a0ee4faa-1ab0-4b4f-a564-ee2358500daf#card` | 1 | D | `9cb5f6921ce59e1f5da24d8aea3438a6391273aaa535cb199bb0faf041fdf43d` |
| Gird for Battle | `bc72d463-8818-4c19-bd5a-f47841d1adb0#card` | 1 | D | `ca705293c16c3d5aab7d3963595286cb6ac31e71d5b29ea9373fcd91bb7508f2` |
| Glorifier of Suffering | `07a8ca73-9890-4c87-8b39-fd9eaea5ef45#card` | 1 | D | `97c5348ec05da91ac57c4eac913d6431338a7b4893a9eb88c2cde4d70033691a` |
| Goldwardens' Gambit | `cf113674-f809-4b97-9e8d-e22385446ddb#card` | 4 | D | `6b273093ce5962cb95f56005777112bb6f554047a87e78f695e29a4e0b6f7a9e` |
| Grind | `ab5b697b-ab62-4ee7-a10e-cc937fef3095#face:0` | 1 | D | `d9065b6c6bac74b90583eb6035a713b88f945cb2c517d5ae6bf5a5b872c39f92` |
| Guardian of Ghirapur | `2553ab57-ceb6-4654-a59d-bc61735da908#card` | 64 | A | `119467b749e6f793889bdff06be9f3571a1773118f6b4eec8441a39aa8784a1f` |
| Hapatra, the Desert Frost | `ac0f161a-e5ec-4a95-9f37-305bdf1ac900#card` | 6 | D | `39b8f5e78d4b41ba309bb0cf8737f0cc04774fde54c1d6554f6ee5de70d634e1` |
| Hate Mirage | `2db23ceb-448b-45d6-a931-d72e8ccdb541#card` | 8 | D | `0ad2f45be05a130412407b493e783f873db061ef6c937d64040cd53408337db8` |
| Heartwood Storyteller | `82e6da87-f8a4-4897-bf0f-c0f2cd06b8b1#card` | 1 | D | `6258a541ec84284688ebe30bf4ff5fad35e8d97c8cf86616e4bfb423a19a3fec` |
| Heroes in a Half Shell | `9f2f12cc-0adb-482d-bbce-05e4ce8850c4#card` | 27 | D | `a8d54d9bb4834125d702ab7151bc7475cec09526fa01b595bbd2c9b9095c41b2` |
| Hive Mind | `f97e405d-4c24-4d4d-9e19-e349c073113c#card` | 10 | D | `1c085836bcc83fd185cd285d688d90efa91c41c0cfd916ed97e05c85eda9434f` |
| Hoarding Recluse | `5fc09704-56cf-4e47-bd96-27ac34434941#card` | 6 | A | `2eca824258f34885adddd19b6d376ee441dd0793d50f957da2647ba27db67950` |
| Homesickness | `ee57fba8-e68b-4c52-8de7-9b9873d84506#card` | 1 | D | `d1fb86c6459606141922fda3fd514c77b53400eaa611471190c5bdad33f49b5a` |
| Hope and Glory | `506ea048-aca5-439a-a5b5-059ddda0672a#card` | 2 | D | `2b90470bf2335461679933ddbc5ce6aaaf6ad4b12825dffc6a1c3bddc4b87441` |
| Hotshot Investigators | `288fcf5c-f5aa-4f62-9dd8-ba295df48f7a#card` | 2 | A | `9edefae3e843649a7f4a5396545e2dcb846181cc57d73c9bc2fcd3ea113c35c9` |
| Hugs, Grisly Guardian | `3c5c227a-335b-4995-9821-d4269a0a233a#card` | 15 | D | `14c0d5f2ba5bb556d8d697367b2fd5c42baae37208d6a84212baddf819e2ffb3` |
| Hullbreacher | `2f533667-e29b-4bec-897a-e7a9eee08314#card` | 32 | D | `025a630e47e4013e478ab04d0ad6a4edd05366446f2a5f787d79ea1dd66a29dc` |
| Icetill Explorer | `109cdefd-e8cc-4ac7-b6ba-2cfdef8d780f#card` | 40 | D | `02afaeca51dc1df9f1ae8184738470e76695b390d21ad852d8b1f009b2d914d6` |
| Incremental Blight | `159cd205-6afc-4d8c-a4b9-501911a9c345#card` | 6 | A | `3b375c05bf0ab7a7b807892ae6ba4d847f3da6d71f8ae24a30dc4cf01e073158` |
| Incremental Growth | `802ff367-e52e-4f79-aa0f-96c45fe477a0#card` | 6 | A | `5623cb74ab0af502c2559a14eb18c1fb16db8313d84cc0baec0b467835e4b9d3` |
| Invoke the Ancients | `868b3c65-d753-4f32-b6ca-31e58782ea92#card` | 2 | D | `0117eede6dc95d206c39144016deb895a78a7064d625b022235e32d080c66dab` |
| Involuntary Cooldown | `4640ce0e-f82f-4bfd-93fc-2d865863fc2e#card` | 2 | D | `23b1e47e98cd4971ecf205dad8302cfb893290f95c6c68e3a0256c549270d0c9` |
| Ioreth of the Healing House | `af6e4c3e-0276-4f72-9a70-25868fe8bba5#card` | 1 | A | `d30a3f04ee878a2da43edff5fa0451c7cc13a65a5f40fe29eaaa22a0c7d2b3ca` |
| Jagged Lightning | `b5d5cfc0-17e0-42d1-b5bd-9f27b11191f9#card` | 1 | D | `25d16392e17430fbdc6fc2e5c60c5921b1120578c0a20ba65bdb9d9ab49397d7` |
| Jaya's Immolating Inferno | `f5f0deb0-070a-45b6-9b81-0bc8143f3040#card` | 1 | D | `26baf1810e8e9f5122fbba6c294b62f739414bb14643b303089ffad27299be38` |
| Jenova, Ancient Calamity | `9370f6bf-d6a0-4c5b-828f-7229f563cc07#card` | 16 | A | `0e296daa1ec0b187339757053978eba7ee1f43493491dfe9a2c8b0f84c24ead9` |
| Jet, Freedom Fighter | `77cd2246-6906-4dce-8580-4c9d2e68c477#card` | 112 | D | `191e9f14aca0bef7650cc39216a64d94c93362c5577868d23a59b0723b0d266b` |
| Jugan Defends the Temple | `e70ac420-8540-4a69-820f-ae3e0274ca1a#face:0` | 132 | D | `17ba58bfeba4740469861aeae9a6d6b3da10c52ea7c87e18f43a2c64576d1d54` |
| Jukai Preserver | `a8ce90be-f0ab-4f8d-896a-cde8aaf24579#card` | 4 | D | `0d654af38139b8bd32b75f9506f910122af9b4038c0c6b48a83fd33687cdc6f2` |
| Juvenile Mist Dragon | `c6867301-296c-4840-aad4-1ad8345c091c#card` | 12 | D | `0aeb15fab7f316bad15fc8af2e8691ef572f892065ceba632c7fcbb961c9466a` |
| Kirri, Talented Sprout | `f03bc648-08e4-4ecd-b339-ff72b00367c2#card` | 216 | D | `282f40d82423ea24115c60568b0d57c250437fa2844951f08d689cd7431c7d6a` |
| Knight of New Alara | `170856f4-7c9d-494b-9bae-20e4d9eba0da#card` | 6 | D | `32c13dcb9d6728fdba4837ff42231c51a6eabda23fd051aad230bdee0d2beab3` |
| Koya, Death from Above | `e7cd87d4-028c-49d9-9416-69f57e7dfec5#card` | 11 | A | `d800e224b874bc308819e034aef31a8f627e5c8df41d37e7a0c0d203b2c8613a` |
| Leo's Guidance | `f386bca6-a08e-462b-b1d9-c3b03929c68d#card` | 1 | D | `b8e400fb3b29b2d5693ea48699beb3c6f315bda23ce6f98178fef2905d63eeb1` |
| Liberated Livestock | `d080cb03-b3e5-42e8-b289-2ce2151b4cb8#card` | 125 | D | `076d4a89419ac6827341b378aa382dabda3abf9968cfcc4ab9dede4813823b2d` |
| Lightmine Field | `20490b29-4f08-48a2-bf54-2ba118e746ac#card` | 11 | D | `30f145d8a5eb22e5f5d00c155363657686dba2e80732182361ae7c3ccb3e8d73` |
| Lockjaw, Slobbering Teleporter | `5efce07e-24c4-4520-a46f-9954b67c4e6e#card` | 18 | A | `029300619c961ebc3ec05a5bf9b65082248d0ca775f5574dfe516ba061f940d0` |
| Lost in the Maze | `3a024389-91a1-460c-b4e8-3f212d2b8400#card` | 2 | D | `bb4ff81fbd3f51955464b36fdf68c4ca13ee255f3bfdeb6a6a02209ce7d69211` |
| Mabel's Mettle | `c657626f-7444-4c26-842c-dda9b9656fa3#card` | 4 | A | `413965233fb224996ba0ce7e39b605c3bb0a29e5a99a914a8e744825dee3eb6f` |
| Marang River Regent | `099b8d2c-b482-48dd-b6bc-dedf546d66c5#face:0` | 2 | A | `531c033c3adfff15489e6ab0ece5dcac5a99049bdea9b5a6d4f6ce7031dc56a6` |
| Master's Guidance | `66de5b97-0de0-422c-a351-bfe55ad4503a#card` | 2 | D | `508855b8b0b07498c8a7a59c5086e961b4ce248fc83b496bd28a241a1066096b` |
| Mega Flare | `891c6ca5-d238-4ac2-8df4-c6dc9a807562#card` | 448 | D | `001d8fc22bac05a01edfdb8e30f32cd84f89f76b3013fbf2527e334ee672a0eb` |
| Metalhead | `7df45a2a-75bc-4343-8dad-3a642efd13ee#card` | 4 | A | `0a12a7e643b0b9810de988ee83cb62f4d59497835005591112765dd281a7badd` |
| Meteor Blast | `0ba1837b-67a1-40d1-b617-8c94834d02ba#card` | 1 | D | `a60732ee0c03bad8518731a94e2868fe81f038109e116d9575ec9cd56c45494c` |
| Might of the Nephilim | `4fda9b70-8da5-4292-b05a-0a0e5a2ad809#card` | 3 | D | `95adb5fa0ed5f10f41863fb22aff76e129331c0b437292da16f802dc91bc804b` |
| Mina and Denn, Wildborn | `e091718c-0c26-46d9-b694-f95effbac3e5#card` | 20 | D | `2be5de2019fc29693f557712249b449f88cacb26001fb4bcc74da9614eaa14a5` |
| Mirkwood Nurturer | `66630640-9605-44da-afdf-eb9d458d056f#card` | 6 | A | `74d4b1a9d80af278aa5e30f5ee3fbad658881a206fea5eb485b2a01504822cde` |
| Mischievous Pup | `e75b187d-0e3b-4d94-a940-8b7e4e7ed1ca#card` | 6 | A | `2431c2a3c267cc0fc11e306b1ebbd861dc7d015eca0c24bc0e33388d4ebd5524` |
| Myojin of Roaring Blades | `20d20254-0f77-429a-a74e-0aec08d18f2d#card` | 32 | D | `01bbda491177067432c99979642548daf046ff9e37652560e24d03ec1968cf19` |
| Nobody | `eddab60f-2eac-4f20-934c-3d9307f885c3#card` | 6 | A | `3eabb3703366896da207ca1c899ceb05673f6ccfde9289f92232f0a5507f9bb9` |
| Norn's Annex | `9a1fbe72-4a17-42be-8e23-d7d30a5e59c1#card` | 15 | D | `056f2a71243f1cb84b5d01046f241e13e6e3f657bd83e04140108a0e9e9d90e4` |
| Notion Thief | `f8dab16e-1d50-443e-9431-8b6f1cf61c9c#card` | 96 | D | `0066e36b80d813e2e2bcebfe6ef02d88f48c9d001b1433f2d3bfa6543eef31fc` |
| Opal Titan | `ce06fe83-a8c5-46b7-b30c-b8189d217741#card` | 7 | D | `0fcad17240ab751a562523dc8f6007fa42a474015267e4baa98fbd909f5ca258` |
| Orcish Bowmasters | `ea5103f5-27e0-4eb1-902c-7f34652d6bf3#card` | 14 | D | `010a12557a8cc671ee1df5bab9f74d8f44c14c01c122f7178856f108f797c2b5` |
| Out Cold | `2d16d51b-47e0-4cdc-83dd-a805ba50bd83#card` | 2 | D | `951c97c3d1db7e54340be88e02888d59df6cd4ff3dcc6a3dcce2b5dd1232fea4` |
| Phantom Blade | `58d5d563-cb60-4977-86cd-f32cd811aa77#card` | 2 | A | `9dbd0c395cb6435f7e0f32566e1902e3a37aec7c5e831869a10f2b935263f0e1` |
| Phelia, Exuberant Shepherd | `5d86a59a-ba1f-45f7-b829-dca6f9f3e624#card` | 11 | A | `04c10443b436d8cace32cf3ed2bc7f94821819e2b76d06dc7c655c41791e4317` |
| Photon, Lady of Light | `2d0aa801-1227-4226-8c27-3bdd441857aa#card` | 222 | A | `333f83492229d3f6722c36baba95db67b9af88c15f383b8d71a253bc8eb8fb32` |
| Pinnacle of Rage | `6ca999d7-48c9-4517-8e63-82ddccdb430e#card` | 1 | D | `6a7f7b6eef09b1c171c47af7efea23937316eb21731520a5d9285df3de56d0de` |
| Pizza Face, Gastromancer | `e71479c9-bb38-4112-bf8a-41b39a3ade51#card` | 48 | A | `088a2dfeb6f3daf4a91c8c8959c8fc36d9c18cc6acec637989846f4d6f7f912c` |
| Predictive Preparations | `d06a5642-861b-4b0d-9ea5-07046b7e1d37#card` | 1 | D | `baa4fe7451b31e50ee51cf12c09cc4ad144a582fc9becfada9633dec77edc231` |
| Prismari Charm | `c3c25a2c-f71c-4cb7-9117-0bcda9248bfd#card` | 4 | D | `b52cc676afa5bbeb42ca3060afe4816a53cb1528fde6fb0f76eeca03e698857a` |
| Protection Magic | `1380f0cc-e875-46ec-a802-c6684bf3965c#card` | 1 | D | `1b54649735589f8c3223d588936d06a99b79ba9a91c0cdbd2675189b7529bd14` |
| Rally Maneuver | `6bbdf771-75d5-4910-bd16-6232e254f7f6#card` | 9 | A | `02c43f78126418f46c84b207b5710b0d72729a54ed824657c9f849a33dcbc446` |
| Reap What Is Sown | `dd6cec11-5c66-4f0e-a12f-9b483a09a777#card` | 1 | D | `5f96062073f7135ca4930193360f230b68b9eb31bbd6fa128b962f3febc5406d` |
| Relic Crush | `92381398-94f6-471f-874a-b5894e6f9487#card` | 1 | A | `b966bc4cdbc9d25dd486125d9f9b1fa5d076792497f24f7906ed90b4715d69ec` |
| Repulsor Bots | `facc26d5-bb53-4f91-9b8c-73bf4dd44fcd#card` | 4 | A | `9132a36f66dfa7a5389104688b44ab2f51a1ca79e6e539f6130dff1a9a14eeda` |
| Rescue, Pepper Potts | `54917578-9493-499e-b200-51d5cf7978ff#card` | 32 | A | `06cd63dcf5faf620c93b9b8495ed854b46a9ca4e52bee86195a2e87d886cca40` |
| Return to Dust | `3029df1d-d02a-4fed-8ab4-000a2096f823#card` | 4 | A | `0bdb5795e8ab5ceb99ab61fcbba8059c8576d75206afc9477c084e8c3ec77985` |
| Rhino, Terrible Trampler | `4e1c9e10-1cae-4ee5-866e-b7b5bb47bbce#card` | 8 | A | `14abee8f99ec526202c993b2116fd7f62a12ecf7158dca7aefc7a9a335586468` |
| Rimekin Recluse | `ad99e430-e13f-4819-a2ee-bf3332ef517d#card` | 2 | A | `6f8eeb5e660649ed9150ffb5823c898b665d699d850e2832381338e504efdc6e` |
| Rishkar, Peema Renegade | `761021ce-4559-464e-aa03-85c2fe78e267#card` | 5 | D | `10ea6f21dce2469fe91802879e9b02df10ab75d5f2690a605ecb80246fb4909e` |
| Rites of Flourishing | `7080fb43-5d93-41f5-87d3-bc1801805ea0#card` | 10 | D | `242936e031680664d77c792b734214a44e82e0549221c1a714940cb2bdf2de63` |
| Run the Play | `10389ff7-2ea4-4413-90cc-0e3ca268c64d#face:1` | 2 | D | `4bb6879a19ebb45175bc92a1c50f6dabe40cd7b8b43180af53e1b4919e9afb14` |
| Samite Elder | `8feb27aa-1f62-46e5-ba20-72c712de0862#card` | 4 | D | `47ea29470308adb7c369bde24fe21b82e09845ed10da080da23a515654101979` |
| Scheming Symmetry | `6dfb50d7-359b-4644-90e3-7ecdacee7c12#card` | 6 | D | `a3f0388f26810ac521d89c046e2109af2b2f4996baed587838d1ca83e8284a2e` |
| Seeds of Innocence | `f5ebad5c-9675-4a37-b76c-760416083939#card` | 1 | D | `7e8726b984b77222d21feb282fa5a5c714aa8bc0d963479341fc56df16d3cf88` |
| Semester's End | `3aeba3c2-5517-4870-8802-29b0fa70ffae#card` | 6,545 | D | `001aa80bcd831b6ec424bc97dc53e00c47a920ddd01627ebf8ae4d3713c633d3` |
| Serpentine Spike | `c983644d-6741-4aa1-aa68-a6e680c26bb6#card` | 6 | A | `126afea82166807ed55711866ce32ea5b3be4bd8333d60ad6df4ed82e2d6cd4d` |
| Shellshock | `f406630b-d9f1-4fc5-b3f3-5327384b2901#card` | 26 | D | `069c9f6b67e63c6fb385fbb6dd49789f11f5014a7fd68488d3cfce7ec7c3772e` |
| Shimmerwing Chimera | `21c768fa-4f1a-4a1a-abb9-e69e17426233#card` | 6 | A | `0cf786877fc591faec986c88bef04c416c33af4d30a626f16b79729cafe1bef5` |
| Silver Surfer, Cosmic Voyager | `3ce1f550-d044-4cb1-991b-e98449f74507#card` | 256 | A | `1240188eb41b7f80219b2d51c7179ddf09526c6fa3cb953cfb0705bd8409e9ca` |
| Skyskipper Duo | `a50db36c-948b-4132-a5e5-84aab1f42612#card` | 48 | A | `27cdbfbaf599191bc0d460e803076755e8482fc8139be8ca174748eb4a1a2f34` |
| Smile at Death | `9228f04c-506f-4453-a335-e66876b8ce8d#card` | 15 | D | `3d17c53c66ecc11ee2940bfdcdc1be302a152a643b47496f84390c657e06046b` |
| Smoke Spirits' Aid | `8fef58ff-4743-4065-9df0-c5edf9aa2502#card` | 20 | D | `1338fc7d96706a9f8ffd77a1bf76338f43ea0fb870e0b5c9fbe77f3af0f45c2c` |
| Smoldering Werewolf | `372a01f7-6e58-4424-9405-0ee4acbb9346#face:0` | 1 | D | `68492781e1ffdb2b4888eede0d32b38ef42ca78552fde9763d57e4258f652bc8` |
| Song of Creation | `75b73eb2-dafd-43db-9e96-7e1995d9849f#card` | 10 | D | `0bd38b68555cf7fb8c2d2dd44505320e0624e5369a86d5ad9f564c91addf84cb` |
| Soul of Emancipation | `e9666fd2-3ebf-4f57-a51d-a80e625f39fa#card` | 8 | D | `4c577113e833087d15f698dcb304b6a0af091b621417523dbcf37c0b58813b4a` |
| Sparkmage's Gambit | `c8df3d1f-130e-4391-9bec-3f2d345a13fb#card` | 3 | D | `376be9d0763618413c914442f8fda93b0d15c28db57d3a6f87f6901628e36eb6` |
| Spinning Wheel Kick | `ed37ce4e-d84e-442a-ac4e-0512f358c138#card` | 12 | D | `dfa00c416eb8e823462a6d521f79e27cdd7d23908487ebc601f4c236ddabd14a` |
| Splashy Spellcaster | `21486218-70ec-4c60-9844-8f7c80912333#card` | 60 | A | `01467426134ee74cf16c0832fc5a67026533e343a75f5a782d060a553476af23` |
| Stall for Time | `09a400bd-83f1-4fac-a218-64c9f041874f#card` | 1 | D | `3cd06c9f86d01b654a898c20e2b26b6d57ba15d888dea7a60c5c4b9c3cc26bbc` |
| Standstill | `fea4cb32-0329-4199-9abe-bff0cc45882a#card` | 1 | D | `3e0ffd029bd3ec62fefef99de8ea91364fcbd129a5d37226d6ced34084fa7259` |
| Stickytongue Sentinel | `899f0931-4e9b-4fa8-9f6d-d13a7de05334#card` | 6 | A | `434329703406cdfe813978b89fc1d4fe7eba0011d0fba7699bc21ef93f410ff7` |
| Storm Cauldron | `5a51b168-02d1-4eb4-8ccc-614ab6f6cffa#card` | 18 | D | `1d8c47d6c28ee2c6a9f463b0a4e83fce296037084f59c2b87a2106ab991891f8` |
| Storm of Souls | `7022ae9a-2589-493b-a050-a230fd02f80b#card` | 70 | D | `390acd4cac8f835b381c76ad4d115cca10003ab81f8b6cc1a94062834d0510fa` |
| Storm of Steel | `d5021db4-64d3-4a0f-9167-499ccbe16e1e#card` | 1 | D | `81447f90fb5c234efa8cd6ba5dd22374aaf23f1a73173f6537fa3f450d10f68c` |
| Succumb to the Cold | `73992788-8084-459c-8375-9ccc8579c5d7#card` | 2 | D | `74ae24cf91b1a725b8d050e70714d6461b0417b0525caf3632f92251ddfe1f7c` |
| Sunpearl Kirin | `f17b9356-a329-4f89-b690-afc2eaff4977#card` | 8 | A | `1ad715aa55f1a192f968f141c1a766f75bc60af24b3f004b67c10ad365ea4b0c` |
| Suspend Aggression | `883267f0-be7e-43c3-b7f8-82981f6d8c23#card` | 8 | D | `017ed55e241b78574859d824b4adb7baeb839ee9f065217478680cd87c22f1c2` |
| Sweet-Gum Recluse | `8ea10a5a-b698-4f7d-a9c8-520f8d835f34#card` | 9 | D | `6b2b2e5adee3c2355d7ba22512e11c8459f65ade62f73a46c3d4e676e9094341` |
| Swelter | `e1169108-1bc8-42ca-a056-a9358fba10cb#card` | 1 | D | `1975849238692a30f3daef8b6a30a93b1b8666d5a8bfda2bb0d5a9bf45056b9c` |
| Sword-Point Diplomacy | `ac5585ce-2959-4eaf-af56-5dca8cdfb2d6#card` | 9 | D | `07b6f46c9dbb6be02cd908f1ee12a5b6ab0ebae6b226978f42b28499cf176b8e` |
| Sylvan Library | `92eed395-62ca-4293-882b-8565c40daab5#card` | 24 | D | `25354817693c42211449e68d3d3a247c902a05c73d748b6fedd6c9f53edd2ecc` |
| Tale of Katara and Toph | `7c376ea9-d50a-4cc3-ae40-7bac3a926f20#card` | 4 | D | `34980a458609af80a169fa58c03697a36e3572cc0d87bfd38dbb4c90bf9feed4` |
| Tam, Mindful First-Year | `096a878e-cff0-430a-90b5-031ecb9bfe1b#card` | 12 | D | `05f7ea057618a92313aa5024f027db55cbe9ae48ba7962740da7fc7e41e3d577` |
| Teferi's Ageless Insight | `1c9e1f75-73f0-4846-b53f-458a0984b1bb#card` | 96 | D | `04cfb28339f602a40849f86cb744335bcf359720da24bfd919e44249e7b7d1a4` |
| Thalia and The Gitrog Monster | `422a76e1-97a2-4a90-b07d-f487a2d8c236#card` | 75 | D | `04efa1f44d87b24755dd3e1be31f1b9445d4a171e2752a8451ccb43e27ae89cf` |
| The Kami War | `4d23b22b-31a5-4eec-b894-2dd5a7138c5a#face:0` | 88 | A | `0c9c3a270c77e09c92e3f70fc278a852150fe11c49b5482ab473b8c4b984f6c2` |
| The Mind Stone | `b175e826-09e8-4fae-9f2e-b902f95b282d#card` | 296 | A | `1df11ddcf6308e122953344555d209e3d72c898dbf06655507b60f6f895d65f5` |
| Thranduil's Company | `2c195485-83f9-424f-854a-50c4f5b04182#card` | 24 | D | `134e6c303a17426682e522c6294fcdd64fa845db5a82375cb4f3e23643716ded` |
| Thrive | `fe3c524e-10b7-485a-9285-973092eeb214#card` | 1 | D | `ed81578c33dc57d9ef5001d01ab5fc74b1370b964744bd7c5e9f3d6034770774` |
| Travel Preparations | `8432729e-5fd4-4661-8b79-e41b1d03eb7b#card` | 1 | D | `83914b06ee83857f24b704cd99f5f248861e67e2b626f63da1f4f2904cf64c83` |
| Triceraton Regenta | `2650d2d5-576e-4b9a-95ac-0f5307f1580f#card` | 1 | A | `a393857c307011292a182fe20f5822cc94b2077b8b526f73cdbdaec204205038` |
| Trick Shot | `843c8485-21b8-45ef-9a0c-4967f07278bb#card` | 1 | A | `b1547d43bbb6cc97d90b6f90376827ba5fce79844fe2f7dae774140d5b2b1d20` |
| Trygon Prime | `6b2d1d7b-a4b4-46c3-a50b-df7e60f39ee2#card` | 21 | A | `07afefbe2af6a39f3b2ac98903f0e7f6d959b812d5e3272662bf68ce80c46076` |
| Twisted Riddlekeeper | `c7ad20a3-51bf-4a22-a09c-fff8cae22765#card` | 2 | D | `0e90fe18fb8b8fa0c118e44a1e3d9c651b92d0d9704db2fcbce1cedb4214f22b` |
| Unbounded Potential | `d9ae08cf-eef0-407a-ad2c-08b4c89f99f0#card` | 1 | D | `009856f223cf09cfb0fd84605a4880e823661395f7265cbe4eb085c09dfe0a9d` |
| Unbreakable Formation | `97435e03-2fff-4fe0-8fa9-69ab0a046a33#card` | 112 | D | `025736957c2c7756fb5d83c835998aac20c009f53f0e862fa63e1e0b80f84e2c` |
| Urge to Feed | `c9f2f0ae-5869-43dc-a225-4dfe3d37c166#card` | 8 | D | `2304922c091509dcf0cbf30988d38cad3a7c9e1d13940f65b8bdc4d4640e5587` |
| Valgavoth's Onslaught | `663d9559-6619-46fe-8ff4-655f1d5e0e4c#card` | 2 | D | `34ef0792739028cb550d5d7056a1464538daf76079817453dc8e9150bae92d23` |
| Vault 13: Dweller's Journey | `586fb987-cd77-420f-978c-2ce140bcdcfb#card` | 1,248 | A | `00b41719747866e1c36f8a542735620d29a044cff9c5f74605428644af487ef5` |
| Venom Blast | `c4b3dc5e-651f-4307-9d76-d728e5054a36#card` | 3 | A | `27d294a5781b842d5c9af12b67dcdd728c3c94f203bb3024d0842370a97d8741` |
| Vineshaper Mystic | `ca514283-c58b-4108-82b2-3b6b15b467f4#card` | 2 | D | `95fff3c4d864680e9498a45a81522bf38766a7f6b15e525bd972608a156f0bae` |
| Vrestin, Menoptra Leader | `1db6a042-303e-4b76-bb6c-63df846fd6c9#card` | 45 | D | `09e6953f534617b26aab739a65bebef816640329d65a43fc82e017ece4657dac` |
| Vulture, Feathered Fiend | `c598d97f-fec6-4238-8eed-8051e9541d31#card` | 18 | D | `41d981da4a93de911a07c08e9679791bc4cd7beeb5626ae43244692a23ed6fdc` |
| Weftblade Enhancer | `4d8ee903-17b3-43c9-9637-ee596955a9f4#card` | 1 | D | `98e44f7599db4ff21896f8fd5a70ca8f0534001e1d0e0141aa3cd07bf0e94af6` |
| Werefox Bodyguard | `d5ee2ced-29f4-430f-962e-2f930b92624c#card` | 7 | A | `3e54b4a9db5da2b2bf0d32f8f063a7f90e7fea3003b6dc5e01a92d675cefd8d2` |
| White Widow, Free Agent | `4f0c055f-f614-44b0-9374-c53541f89185#card` | 12 | D | `3c397bc83ee921811244133f385af4ab7aebdfa27605988d3915cc1fc25b3317` |
| Wingshield Agent | `513034fc-873a-4877-afb2-39ece1ea6a7c#card` | 15 | A | `033dd834ae53e18339fc6cb38179c1608f3930511baedb8e79434bd9d4c25ac9` |
| Winter Blast | `594827de-e85f-4aac-b0e6-d92878f0f26c#card` | 4 | D | `9c013f3697488a587b64e9801af2ab09749564ff879326e06daa81fa49489910` |
| Winter Eladrin | `48ae84e7-125d-4018-afc1-cd78448d1a13#card` | 2 | A | `20797aec0334dd3d67d4f9358184ee4d2cf860b7bdf82b08ebacef060d883b7a` |
| Wrap in Flames | `b46efd59-eda7-4fe6-ace7-67ca8f6c088d#card` | 3 | D | `8ba241b19ea27de13d055e5959fdc1abd90558d495baadfaa4b0823304b1a9dc` |
| Zell Dincht | `f207ce73-e4cd-4dfc-b3b9-5704f5148959#card` | 20 | D | `0500c2a593919ee4f55cea48c0c1949b5072c676520506c130b1dcde04f2e000` |
| Zephyr Sentinel | `dcddd143-a7d9-4a38-a2da-dd6bd2e48205#card` | 6 | A | `46cf94de74903305a0e94f70f1c78098abdbcc530e4a9c2fa7f5496ef2c3d62a` |


### Closure and refresh

The final reverse-dependency gate passes both derived commands, including Lean
integration tests and Clippy with warnings denied. Refresh exits zero with no
conflicts. Only other sessions' ticket claims advanced the coordinator; the
consumed sources and per-face baseline remain unchanged. The claim-parent to
landed declaration delta remains **+32 nonblank lines**.

A local plugin-cache replacement removed the previously resolved launcher and
status hook. The current Kata skill and launcher were resolved again; command
output and exit statuses are retained in the ignored evidence directory despite
the failing post-command status hook. Repository protection is unchanged.
