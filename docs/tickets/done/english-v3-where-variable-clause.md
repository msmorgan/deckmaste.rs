---
needs: []
---
# Read the *where X is* clause that defines a variable

## Why

*Where X is …* never reads. On change `xxknlzypsnwy` (32,828 supported faces,
17,322 covered, 15,506 unread; recon of 2026-10-07), it touches **1,119**
unread faces and is the sole cause on **149**. Counts are unread faces
*touched* (at least one localised failing unit matches) / *sole* (every failing
unit matches and no other recon STRONG bucket does). They are surface counts,
not gain forecasts.

Probes (admitted roots): "This creature gets +X/+X until end of turn." 1; "…
until end of turn, where X is its power." 0; "Create X 1/1 green Saproling
creature tokens." 1; "Draw X cards, where X is your life total." 0. The host
with *X* reads; the clause does not. `vocab:Adverb/Where` exists in `core.ron`.

## Goal

A clause-final, comma-separated *where X is NP* reads as a supplementary
dependent of the clause (or of the keyword label, *Firebending X, where X is
…*) whose content clause is a specifying *be* clause with Subject *X*. One
analysis, attached once; *X* in the host stays a numeral-like quantity.

## Analysis

CGEL lists *where* among the items that govern non-expandable content clauses
(Ch. 11, §4.8, p. 971, [57]). In the fused-relative discussion of *when*,
*where* and *while*, it notes an alternative analysis treating them as
prepositions that take content clauses as complements, like *before* or
*whereas* (Ch. 12, §6, p. 1078, [30]). Neither passage discusses the
variable-defining use; treating *where X is NP* as a preposition with a
content-clause Complement in supplementary function is the project's analysis,
to be recorded as such. The relative *where* of Ch. 12, §3.5.3, p. 1050, [51]
takes locative antecedents and is not this construction.

## Witnesses

Each witness's failing units all match this construction and no other recon
STRONG bucket (`xxknlzypsnwy`):

- Chameleon Colossus: "{2}{G}{G}: This creature gets +X/+X until end of turn,
  where X is its power."
- Wild Beastmaster: "Whenever this creature attacks, each other creature you
  control gets +X/+X until end of turn, where X is this creature's power."
- Hemosymbic Mite: "Whenever this creature becomes tapped, another target
  creature you control gets +X/+X until end of turn, where X is this creature's
  power."
- Elenda, the Dusk Rose: "When Elenda dies, create X 1/1 white Vampire creature
  tokens with lifelink, where X is Elenda's power."
- Tip the Scales: "When you do, all creatures get -X/-X until end of turn,
  where X is the sacrificed creature's toughness."

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. Before: `cargo xtask english-v3 --all --workers 12 --samples-per-face 0
   --output target/english-v3/english-v3-where-variable-clause-before.json` on
   the claim parent, stamped with its change id and covered count; after: the
   same command to
   `target/english-v3/english-v3-where-variable-clause-after.json` on the final
   tree. Evidence lives under the workspace's ignored `target/english-v3/`,
   never `/tmp`.
2. Write each witness as a test first. Iterate on `--face-id` selectors; verify
   on `--all` at the end.
3. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
4. Compare per-face Reading counts against the claim-parent census. Any
   decrease on a previously covered face is restored, or named and justified in
   the landing record.
5. Spot-check at least 10 newly covered faces and list them by name with their
   Reading. The *where* clause read as a relative clause on the nearest NP
   (*end of turn, where …*) or as a locative Adjunct is a defect. A wrong
   analysis that parses is a defect, not a gain.
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

- What *X* is equated to when that NP fails on its own (*the greatest power
  among …*, *the number of …*, *the amount of life you gained this turn*): note
  how many become readable; those NPs have other owners.
- Magma Sliver's scalar Subject *X* (`english-v3-systemic-residuals`).
- Semantic binding of *X*: `docs/decisions/oracle-text-is-forward-anaphoric.md`
  governs it; this ticket is grammar only.

## Landing record

In addition to the standard record: the category and attachment chosen and
which section of the record states it as a project analysis; how many faces
remain on the equated NP; before/after counts stamped with change ids; timings
as integer ns and ns/B with host load and worker count.


### Project analysis

The variable-defining *where* is a Preposition selecting a non-expandable
Finite Clause. Its phrase is supplementary to its **anchor**, the completed
Clause or Keyword Phrase on a punctuated keyword line. The `Supplementation` schema
keeps that phrase beside its completed clause/keyword expression and attaches
it once; it is neither a Relative Clause nor a locative Adjunct of an NP.
The specifying clause uses the existing nominal predicative frame of *be*,
with a singular, third-person variable Subject. Host quantities retain their
existing Cardinal, Unsigned Scalar and Measure Phrase projections.

This application is the ticket's project analysis. CGEL Ch. 11 §4.8, p. 971
supports non-expandable content-clause selection by *where*; Ch. 12 §6,
p. 1078 discusses the preposition-plus-content-clause alternative for other
uses. Neither passage asserts the variable-defining analysis. CGEL Ch. 4
§5.5.1, p. 266 describes the general specifying use of *be*. CGEL Ch. 15
§5.1, pp. 1350–1351 describes supplements as separate from headed syntax;
its general account does not determine this project's Oracle analysis.

### PROVE — retained Readings and structural laws

The claim parent `wlmqosnolmnxutrtvvumxqnznmqoymkr` (covered 20,280;
code-equivalent baseline measured on `toxwntsklvowwxrtzwrrzupnttovxxxo`)
and finished feature `toxwntsklvowwxrtzwrrzupnttovxxxo` (covered 20,858)
use the same pinned corpus: 32,828 supported faces. Coverage rises by 578;
zero identities lose coverage, and every previously covered face retains
exactly its prior Reading count: zero decreases and zero increases. All
30,333 additional Readings occur on newly covered faces. The exhaustive
comparison is `previous-reading-deltas.json` (empty); no retirement or
re-coverage debt is created.

The full census enumerates and validates 651,577 Readings, with zero duplicate
Readings, byte roundtrip failures, lexical ownership failures, construction or
leaf traversal failures, materialization failures, or other issues. Independent
keyword-supplement values additionally establish exact value roundtrip and
traversal identity; unlicensed prepositions fail in both admission and
realization. The witness tests require exactly the Cartesian product of host
and definition Readings and exactly one attachment beside the completed host.
Elenda's two catalog name identities remain distinct grammatical Readings.
Multiple Readings are preserved under the current lexical-analysis contract;
there is no destructive selection or unresolved-tie rejection.

Word-named licensing guards added: zero. Admission reads declared
`PrepositionFunctionLicence`, `ScalarVariable`, and grammatical punctuation
features. Lexical source loading and generated GrammarEnvironment construction
succeed; existing loader/compiler error handling is unchanged. V3 emits no
legacy coverage-lock `covered` value or permitted-licensing-checker total;
these retired-parser figures are unavailable, rather than inferred as zero.
The full-corpus covered total above is the current V3 measure.

Assurance: 9 tests added, 0 restored, 6 existing tests re-spelled, 0 removed,
0 newly ignored. The gate retains the existing ignored
`macros::templates::tests::macro_schema_census_count_matches_21`, whose
attribute says "cross-checks the live corpus against the census; run on demand".
Re-spelled tests retain their exact values, cards and outcomes:
`independent_frequency_values_preserve_quantity_number_and_count_sense`,
`independent_frequency_attachment_preserves_the_intransitive_frame`,
`independent_measure_values_preserve_operator_structure_and_notation`,
`independent_slash_consumers_preserve_count_components_and_leaf_order`,
`independently_constructed_maximum_determiners_preserve_number_and_lexical_owners`,
and `independently_constructed_property_values_preserve_both_roundtrip_laws`.

### DISCLOSE — scope, separate licence effect and residuals

The separate six-worker, complete-enumeration `sacrificed-before-licence.json`
and `sacrificed-after-licence.json` runs (snapshots of `toxwntsklvow`;
whole-corpus covered 20,792 before the licence, derived by subtraction, and
20,858 after it; lexical hashes in `measurement-stamps.json`) differ by the single sacrifice
licence patch, with the finished where grammar held constant. They select 229
faces and increase covered faces from 57 to 123. Exactly 66 faces change:
54 gains need the licence alone and 12 need both the licence and where
supplement. No baseline-covered or where-only-covered face changes its
Reading count because of the licence, so the list requiring individual
judgment is empty. Every changed face contains *sacrificed*; the sampled trees
on all 66 use the existing past-participial premodifier construction with the
existing sacrifice lexeme. Full named deltas and judgments are in
`sacrifice-licence-deltas.json` and `licence-gain-spotchecks.json`.

The gains partition into 506 where-supplement-only faces (29,327 Readings),
54 sacrifice-licence-only faces (333), 12 joint faces (32), and 6 ordinary
variable-Subject faces (641). Thus the where feature alone gains 512 faces,
of which 506 actually use the supplement. These are measurements of the
finished landing, not the old sole-cause estimate: **149**, stamped
`xxknlzypsnwy` / covered **17,322**, counted faces where the missing route was
the only blocker on that older tree. Later landings cleared co-blockers before
the implementation baseline (covered 20,280); the resulting 506 supplement
gains plus 6 ordinary variable-Subject gains explain the 512 where-feature
figure. The two inventories measure different trees and different causes.
The post-landing fix below supersedes the gain totals and Kraul analysis.
The last six are Bargaining Table,
Soul Immolation, Mortarion, Daemon Primarch, Chromatic Armor, Shanna, Purifying
Blade, and Spoils of War. Their sampled trees use the same singular,
third-person variable Subject and existing nominal predicative *be* frame;
the modal negative cases use the existing comparative complement. They add no
new frame or duplicate constituency and have no Supplementation node.

Of 1,119 variable-where faces, 518 are covered and 601 remain unread.
The first-pass noun-phrase probes reported 322 of those 601 as containing an
independently unread equated constituent; this did not assert sole causation.
The post-landing boundary audit below corrects ten fragments that included
an outside continuation, reducing this diagnostic to 312 actual NP blockers. The other excluded NP work remains excluded. The 361 distinct
constituent probes and the refreshed sacrifice-containing probes have zero
internal/validation errors and are named in `equated-np-residuals-final.json`.
Magma Sliver is naturally covered through its existing nominal predicative
NP (*the number of Slivers on the battlefield*), not a new scalar-denotation
frame; `X is 2` remains rejected. Semantic variable binding is untouched. The named residual face/constituent
ledger routes to the live [english-v3-systemic-residuals](../planned/english-v3-systemic-residuals.md)
follow-up; this landing creates no lost-Reading obligation.

At least twelve where gains were checked for the specified attachment and
lexical owner: Magma Sliver, Ogre Battlecaster, Whiplash, Vengeful Engineer,
Hurl into History, Recantation, Lydia Frye, Yew Spirit, Sunflare Shaman,
Ghoul's Feast, Elder of Laurels, Anim Pakal, Thousandth Moon, and Axebane
Guardian. All 66 licence gains and the six extra Subject gains were also
inspected; sampled heads, construction identities and fingerprints are retained.
The named analysis ledger and table below cover every newly covered identity.

### Deviations and additions; STOP resolution

STOP raised before adding the licence: required witness Tip the Scales had an
independently unread equated NP, while the ticket excluded repairs of such NPs.
Resolution: **orchestrator ruling, 2026-10-07**, explicitly authorizes
`PastParticipialPremodifier = Yes` on the existing sacrifice lexeme, under the
attestation/pruning rule. This is an orchestrator resolution, not a CGEL claim.
There are **128 supported raw-text faces**, of which **127 retain the phrase
after reminder stripping**, containing *sacrificed* immediately followed
by a noun: creature/creatures, artifact, permanent/permanents, enchantment,
land or card. Five witnesses are **Fling, Thud, Tip the Scales, Altar of
Dementia, Greater Good**. The complete face names and source strings are in
`sacrificed-noun-attestations.json` (127 analyzed-source faces) and
`sacrificed-noun-raw-attestations.json` (128 raw-text faces). No grammar feature or construction was
added for this resolution; the ticket's other exclusions remain.

Added features: none. Added feature value:
`PrepositionFunctionLicence::Supplement` in the existing set feature.
Added table: `licence_supplement`. Added policies:
`UnlabelledSupplementation`, `VariableCountProperties`,
`ScalarVariableProperties`, `VariableSubjectProperties`.
Added named constructions/schemas: `Supplementation` and shared
`ScalarVariable`. The latter replaces ordinary `VariableCount`,
`VariableScalar`, and `ScalarVariable`, preserving their quantity uses and
adding the variable Subject projection. Added frames and categories: none.
The existing `NoFeatures` policy, clausal Preposition Phrase and *be* frame are
reused. Two instance matrices are added for those two shared schemas.

The old `vocab:Adverb/Where` lexicon route is retired as
`vocab:Preposition/Where`, including ownership. No dedicated old grammar rule
existed; the generic Adverb construction retains its other live heads and no
unreachable where declaration remains. The six extra ordinary Subject gains
are disclosed above as a consequence of the necessary shared Subject
projection, rather than special constructions for those cards. The independent
value and rejection tests extend the ticket's witnesses to establish both
roundtrip laws. No test is deleted. The Supplement glossary entry and explicit
supplementary extension of Preposition Function Licence close the terminology
gap, with CGEL and project authority distinguished.

### REPORT — declaration economy, census and inventories

Claim-parent versus landed nonblank lines of
`crates/deckmaste_english_v3/src/declarations.rs`: **3,269 → 3,300; net +31**.
The inherited grammar already exceeds its 2,800-line ceiling. Named
construction/schema count decreases **249 → 248** (206 ordinary + 43 schemas
→ 203 ordinary + 45 schemas). Feature count stays 94, categories stay 144,
tables 72 → 73, policies 57 → 61. These figures are stamped with the claim
parent / covered 20,280 and finished feature / covered 20,858 identified above;
`declaration-inventory-final.json` records the same comparison. Every added
feature value, table, policy and construction is listed above.

| Measured tree / covered | Unique faces | Multiple faces | Unread faces | Exact Readings |
|---|---:|---:|---:|---:|
| Claim parent `wlmqosnolmnx` / 20,280 | 7,431 | 12,849 | 12,548 | 621,244 |
| Finished feature `toxwntsklvow` / 20,858 | 7,477 | 13,381 | 11,970 | 651,577 |
| Refreshed final `wzuorztqvmnl` / 20,858 | 7,477 | 13,381 | 11,970 | 651,577 |

Specificity-resolved selection is not a V3 census category; all Readings are
retained. The 46 newly unique and 532 newly multiple faces account for the
whole increase; no previously covered face moves between these categories.
Declared spellings: 38,374; named homographs: 1,127, unchanged. The complete
named homograph, form-literal and empty form-literal/vocabulary-overlap lists
are `inventories-before.json` and `inventories-after.json`; the licence patch
leaves `spellings-after.json` byte-identical to `spellings-final.json`.
The sole owner changes are *where* and *Where*, Adverb/Where → Preposition/Where,
in `lexical-owner-delta.json`. No duplicate lexical identity is introduced.

| Measured tree / covered | Workers | Host load (1/5/15 min) | Corpus wall ns | Checked-text thread CPU |
|---|---:|---|---:|---:|
| Claim runtime `toxwntsklvow` / 20,280 | 6 | 9.015/13.008/11.099 | 150,121,668,066 | 244,546 ns/B |
| Finished feature `toxwntsklvow` / 20,858 | 6 | 6.090/7.062/11.304 | 121,275,881,407 | 269,914 ns/B |
| Refreshed final `wzuorztqvmnl` / 20,858 | 6 | 3.109/6.278/9.081 | 120,059,944,397 | 269,779 ns/B |

All three shared-host coverage runs exceed the 16,260,000,000 ns quiet-host advisory
ceiling. The final run overlaps the gate; host load differs, so the lower wall
time and higher per-byte thread CPU do not isolate the causal cost of the change.
This is a performance advisory. Corpus provenance and exact load values are
retained in the JSON reports. Measurement stamps distinguish the claim-equivalent
runtime before implementation from the finished tree sharing its working-copy
change id; those are different snapshots, not contradictory measurements.

### Verification and evidence

Full census commands use `cargo xtask english-v3 --all --workers 6
--samples-per-face 0 --output target/english-v3/english-v3-where-variable-clause/before.json`
and the same command for `after.json`. Only selector subsets are used for
iteration and samples. Exactly three full runs were used: claim-equivalent baseline, finished
feature, and refreshed final. The final command uses the same flags with
`--output target/english-v3/english-v3-where-variable-clause/after-refresh.json`.

The reverse-dependency gate is `cargo xtask gate --changed --from wlmqosnolmnx
--run`, deriving `cargo test -p deckmaste_lexical_source -p
deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask`. Final result:
exit 0; 841 passed, 0 failed, 1 inherited ignored across 96 suites. Clippy on that package closure, all targets and
`-D warnings`, passes. `cargo fmt -p deckmaste_english_v3 --check` passes.
Citation checks report **0 noncompliant and 0 stale**; the piped diff audit
selects 0 CR citation sites because no CR citation is changed.

All census/evidence files are ignored under
`target/english-v3/english-v3-where-variable-clause/`, including the full gain
ledger, exact per-face comparisons, focused reports, inventories, NP probes,
CGEL extracts and verification logs. Final census: **`target/english-v3/english-v3-where-variable-clause/after-refresh.json`**.

`kata refresh` exited 0 without conflicts and incorporated sibling ticket
moves only. The refreshed runtime stamp is
`wzuorztqvmnlxpmrkqkzsvwtzosrktzv` / covered 20,858. The refreshed base
declarations are byte-identical to the captured claim parent; both feature
code patches and the final lexical inventory are unchanged. All 32,828
per-face Reading counts match the finished-feature census exactly. Relative
to the refreshed base, every previously covered face retains its count:
zero Reading decreases, zero removed Readings, and zero trunk-attributed
decreases. `refresh-proof.json` records the comparison. The successful gate,
clippy and formatting checks remain applicable to this unchanged code.
Citation checks on the completed record report 0 noncompliant and 0 stale.
Evidence is preserved under the same ignored path in default before workspace
retirement; lifecycle closure uses a final unconditional refresh, integrate,
and drop from default.

### Newly covered identities and sampled grammatical analyses

The route labels below are disjoint accounting groups; multiple retained
Readings within a face are preserved. Each durable identity was unread in the
claim census. The selected sample is an inspected grammatical Reading, not a
replacement for the other retained Readings.

| Face / durable identity | Readings | Gain route | Sampled analysis |
|---|---:|---|---|
| Aang and Katara / `481c3e14-b670-4fab-aa9f-6ce5b514096d#card` | 250 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of tapped artifacts and/or creatures you control |
| Abaddon the Despoiler / `8dedb9a7-0afa-4dff-b124-2344c170f11a#card` | 180 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the total amount of life your opponents have lost this turn |
| Academy Elite / `ba6c3c72-c014-45c6-a0b4-59eb9a65303e#card` | 40 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of instant and sorcery cards in all graveyards |
| Accelerated Mutation / `f8a9c279-0f11-4a22-8651-f9caf013ca3c#card` | 10 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the greatest mana value among permanents you control |
| Access Denied / `353dd446-2ea6-41c9-a10f-5a03e4de19b4#card` | 8 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: that spell's mana value |
| Accomplished Alchemist / `97e18a8d-9068-4782-a744-280d28c68228#card` | 4 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the amount of life you gained this turn |
| Aerith Gainsborough / `dd9a6f6b-f9b7-4292-bc80-b7da326de032#card` | 6 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of +1/+1 counters on Aerith Gainsborough |
| Aether Mutation / `6697fe5b-90ac-4321-aa2f-cdc6ec283cb4#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: that creature's mana value |
| Agency Coroner / `3686fd97-49ff-4a92-9cb2-7d8e9438d945#card` | 3 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Airdrop Condor / `25fc274f-d5af-4174-b8e8-c2e65bb4c13f#card` | 5 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Akroan Hoplite / `e15db59f-4477-4139-8e7e-741ab4584e7f#card` | 9 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of attacking creatures you control |
| Alibou, Ancient Witness / `e5e8e116-10fe-48b0-b3d8-6edb39bd5f90#card` | 36 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of tapped artifacts you control |
| All-Seeing Arbiter / `f15460d9-0133-436c-b0fe-1661989124f0#card` | 108 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of different mana values among cards in your graveyard |
| Aloy, Savior of Meridian / `f0554a8f-32de-4069-9f47-5e06ceb3f09d#card` | 4 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the greatest power among them |
| Alpine Houndmaster / `88939cb3-5f61-4857-95cb-9446c6c1f1aa#card` | 207 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of other attacking creatures |
| Altar of Dementia / `d64e9152-ef24-4394-aeb0-9c3befc56549#card` | 1 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Ambitious Dragonborn / `934094d6-c897-4ae2-89cd-08d106600d3c#card` | 175 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the greatest power among creatures you control and creature cards in your graveyard |
| Amy Rose / `b2d597b4-f9ba-44b0-a947-0c2ecda2d308#card` | 3 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: Amy Rose's power |
| An-Havva Inn / `3cd72848-7b99-4d46-96f5-53f297c9d2a0#card` | 4 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of green creatures on the battlefield |
| Angelic Exaltation / `507d2f27-08a9-4938-b191-5734cda42ef2#card` | 6 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creatures you control |
| Anim Pakal, Thousandth Moon / `05551d91-50c6-46d1-86a4-cd3d177d0923#card` | 54 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of +1/+1 counters on Anim Pakal |
| Animal Boneyard / `6fa9debf-c60e-432d-86c8-aed16556c532#card` | 2 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Animus of Night's Reach / `8b1e453a-35a6-4dff-bd37-498f34d0470e#face:1` | 21 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creature cards in defending player's graveyard |
| Anzrag's Rampage / `ce690a21-8117-420e-be00-cd6d9923a812#card` | 528 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of artifacts that were put into graveyards from the battlefield this turn |
| Appeal / `3ecc4b57-3e0c-48b8-9952-e4b63b6bc45d#face:0` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creatures you control |
| Arabella, Abandoned Doll / `7c4bbb1b-29c4-4e06-aed0-b361293a585b#card` | 14 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creatures you control with power 2 or less |
| Arachnogenesis / `b655bee5-52d3-467e-b16c-cfc2edf2b1a1#card` | 588 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creatures attacking you |
| Arahbo, Roar of the World / `66944a11-40a1-4f3a-9f83-52324e0edfef#card` | 256 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: its power |
| Arbor Adherent / `9640225a-8f31-495f-b9de-53ed348ff24b#card` | 7 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the greatest toughness among other creatures you control |
| Armored Armadillo / `dca25245-523a-4587-a059-a14ee2ef908d#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: its toughness |
| Arni, Renowned Champion / `1a929c4f-ae1f-44d9-a0a7-2ca2dc928dee#card` | 9 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: that creature's power |
| Artifact Mutation / `e77e8768-3ea8-44f9-aba6-1e023e006946#card` | 1 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: that artifact's mana value |
| Ashcoat of the Shadow Swarm / `9cc69ea5-42a2-4306-ac6d-cff5adb20bcb#card` | 84 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Rats you control |
| Ashroot Animist / `3f008b57-6471-4785-9571-3b90193da7e2#card` | 8 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: this creature's power |
| Assemble the Entmoot / `1f1ad44c-1811-40e3-976b-03f19866abd5#card` | 4 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the amount of life you gained this turn |
| Atogatog / `b9fdb740-e5d7-4464-b378-2e0514ca28d8#card` | 2 | where supplement and sacrifice licence | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple; completed-Clause PP supplement with nominal specifying be clause; equated NP: the sacrificed creature's power |
| Aura Mutation / `19bb51dd-2b93-45f0-927d-bbafddadd0a3#card` | 1 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: that enchantment's mana value |
| Auriok Bladewarden / `2884e332-707c-4a97-adc1-e1317a513ff4#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: this creature's power |
| Avatar Kyoshi / `e41e8754-028c-4399-b530-de9e134d04b0#face:1` | 5 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the greatest power among creatures you control |
| Avenger of the Fallen / `158d0272-a850-4399-8afa-d0caa143c3cb#card` | 4 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creature cards in your graveyard |
| Axebane Guardian / `060e0378-e335-4b3d-82aa-ae36639339f6#card` | 42 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creatures you control with defender |
| Azog, Moria's Ruin / `a8b018a7-0350-4ee0-9582-8d391018bdee#card` | 1 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: that creature's power |
| Bag End Porter / `b80473f7-f1d4-4aa3-988e-1a8610ffbeee#card` | 9 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of legendary creatures you control |
| Baldur's Gate / `da307ea2-4df7-4d6b-be0f-9dc6ac93db61#card` | 3 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of other Gates you control |
| Bargaining Table / `151d9035-c8e8-4794-8405-dc7744eb99a2#card` | 6 | variable Subject projection | ordinary singular third-person variable Subject; existing nominal predicative be frame |
| Barrage Tyrant / `f3b70d61-e33e-4243-aac9-e7bd9ba40acb#card` | 5 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Barrin's Codex / `f7c237e6-17a4-4e72-8cc3-456aec738394#card` | 3 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of page counters on this artifact |
| Baru, Fist of Krosa / `b6021700-992a-4faa-8f83-a9af03dc7296#card` | 88 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of lands you control |
| Bast, Panther Goddess / `e3233686-7703-460c-80ee-8cd37b3014ff#card` | 42 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creatures you control |
| Battletide Alchemist / `4867cc47-de84-4958-b1de-4a0a5050210e#card` | 4 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Clerics you control |
| Be'lakor, the Dark Master / `3690ca72-c925-42ab-acc2-7cc1fda05789#card` | 12 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Demons you control |
| Become the Avalanche / `e5fcbbaf-9431-4fa8-ac11-12ace1f2054c#card` | 144 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of cards in your hand |
| Beifong's Bounty Hunters / `8561b088-56b3-4732-82e9-8da1e3affd55#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: that creature's power |
| Blight Pile / `1c0fc9b4-177e-4529-827f-a1e17ff7a752#card` | 3 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creatures with defender you control |
| Blight Titan / `f2c75577-c9be-48f3-8398-3020c79bd34e#card` | 12 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creature cards in your graveyard |
| Bloated Processor / `d1a26310-8c14-424b-8a7f-36793769c2a7#card` | 1 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: its power |
| Blood-Chin Fanatic / `b704f231-f64c-49fc-84f6-0fc53081bcd0#card` | 1 | where supplement and sacrifice licence | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple; completed-Clause PP supplement with nominal specifying be clause; equated NP: the sacrificed creature's power |
| Bloodfire Infusion / `d5bd7c0b-c412-47ed-afb8-1a9ba04be77c#card` | 5 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Bloodshot Cyclops / `84d47913-53ac-4d4d-bac3-bc0306d43e12#card` | 5 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Blossoming Bogbeast / `30f3c3be-0fe9-463d-a245-e44701aec7f2#card` | 44 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the amount of life you gained this turn |
| Body of Research / `e0c21b32-f672-4496-91ed-7b7f351eb539#card` | 3 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of cards in your library |
| Bone Devourer / `0ee41ef9-0b39-4d00-a68e-7914839a7175#card` | 342 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of +1/+1 counters on it |
| Bonehoard / `83c69c05-173d-4c1b-9541-1dde474fef5f#card` | 4 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creature cards in all graveyards |
| Boon of Boseiju / `44e1946d-01b5-45db-abdc-2f6d6480cf60#card` | 10 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the greatest mana value among permanents you control |
| Bosh, Iron Golem / `2fb6f65f-a0fb-4d45-be1b-d6405e61d47a#card` | 5 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Brambleguard Captain / `104baa4e-a3f3-45dc-979d-4c58b659ba1a#card` | 3 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: this creature's power |
| Briar Hydra / `caaedd67-e074-47c4-a956-da88a01f8996#card` | 84 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of basic land types among lands you control |
| Brimaz, Blight of Oreskos / `a6e7cbfc-6d5a-4a67-9dad-d512d3c97bda#card` | 6 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: that spell's mana value |
| Brion Stoutarm / `b816b3cc-ae4b-4fb7-8b1a-e01ab83459a3#card` | 7 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Bronze Cudgels / `82ebd4fa-9db1-4723-8305-8b8a9ce958e2#card` | 6 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of times this ability has resolved this turn |
| Bushmeat Poacher / `0287d541-3f73-4c8b-9a77-87f99898758d#card` | 2 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Cait Sith, Fortune Teller / `31851cb6-0caf-4575-aa5e-41aeadccafc1#card` | 36 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: that card's mana value |
| Call for Blood / `f2782c21-ed99-4821-8938-cc539e783ec7#card` | 2 | where supplement and sacrifice licence | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple; completed-Clause PP supplement with nominal specifying be clause; equated NP: the sacrificed creature's power |
| Camaraderie / `9aab386c-d48c-4611-b757-aa69b26cc1b1#card` | 8 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creatures you control |
| Captain America, Wings of Freedom / `942ac2ac-7528-4768-8398-8520a8ee15e5#card` | 384 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: Captain America's toughness |
| Carrion / `84ca8d74-95ec-4a9c-8793-7a195ad28be9#card` | 1 | where supplement and sacrifice licence | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple; completed-Clause PP supplement with nominal specifying be clause; equated NP: the sacrificed creature's power |
| Carrion Grub / `c54b375d-a456-4886-96f2-8dc7a0024a5b#card` | 9 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the greatest power among creature cards in your graveyard |
| Catapult Captain / `aa2c0d23-8df8-408b-a8c0-c4cd005f078d#face:1` | 2 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Cavalier of Flame / `98b19afc-7696-46b9-96f0-7f7c60003e1c#card` | 72 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of land cards in your graveyard |
| Celebrate the Harvest / `de3764b9-9572-4f5c-8c90-82dedec47557#card` | 144 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of different powers among creatures you control |
| Celestine, the Living Saint / `6503ed78-6499-47f6-856b-ee1003ce4a4f#card` | 32 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the amount of life you gained this turn |
| Cerebral Download / `f27889b9-40d9-4cc7-b8af-8716c53c44f5#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of artifacts you control |
| Chain Reaction / `086b2564-9114-4ba2-94fd-b490f98f38a7#card` | 3 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creatures on the battlefield |
| Chainsaw / `3db40361-5f55-417e-a7cd-7e360cc91b4d#card` | 3 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of rev counters on this Equipment |
| Chameleon Colossus / `7b8f1458-c638-4907-9363-f7cc22d04605#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: its power |
| Champion of Dusk / `5f7f0a38-fb7e-4ee8-a712-9aa911f17dae#card` | 4 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Vampires you control |
| Chromatic Armor / `65b3a40e-1d2b-4b24-996f-c935414ded7d#card` | 480 | variable Subject projection | ordinary singular third-person variable Subject; existing nominal predicative be frame |
| Chrome Host Seedshark / `837c5d0e-0a33-4f9d-ae25-68a687bdf640#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: that spell's mana value |
| Cinder Seer / `64b63797-9db0-41fa-b2eb-fbf51de0e337#card` | 20 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of cards revealed this way |
| Citadel of Pain / `2233c079-5ca6-4992-8bd2-84dbf9ed53a9#card` | 6 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of untapped lands they control |
| Cleaver Skaab / `9fa66e03-7b20-43f6-8c53-1ce18169c382#card` | 2 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Cleaving Skyrider / `eb1ab332-5549-4ff4-8afa-b385a236fb9b#card` | 1 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of attacking creatures |
| Cloudhoof Kirin / `82d8732b-8246-4fbf-96aa-e192daf527f1#card` | 4 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: that spell's mana value |
| Collective Restraint / `51f561db-ca7f-4661-bc04-99b8af02b8c9#card` | 252 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of basic land types among lands you control |
| Collision Course / `49ca0ca8-0733-4aa9-bd0c-1c661725b81b#card` | 5 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of permanents you control that are creatures and/or Vehicles |
| Commence the Endgame / `fedb1947-9fa5-4655-8d02-b370fa63dd16#card` | 6 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of cards in your hand |
| Commissar Severina Raine / `c00074ec-5b31-44d3-b69b-e2bf6f6a2477#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of other attacking creatures |
| Conifer Wurm / `8f780497-23bb-44c3-982c-e2b8a7a29999#card` | 6 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of snow permanents you control |
| Consult the Star Charts / `e921839f-9d91-41a9-bc89-016af3c757aa#card` | 18 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of lands you control |
| Consuming Corruption / `3bc0bd36-70be-4180-9fb3-b10ce054107b#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Swamps you control |
| Coral Colony / `5e186931-992b-491c-bffc-6877a613d5c3#card` | 7 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creatures you control with defender |
| Corpse Augur / `06eb43af-637a-4f73-8071-a7036d4c8c3d#card` | 14 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creature cards in target player's graveyard |
| Corpse Cobble / `5b0b6c8f-472a-4ee2-8368-3b17a2df498d#card` | 14 | where supplement and sacrifice licence | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple; completed-Clause PP supplement with nominal specifying be clause; equated NP: the total power of the sacrificed creatures |
| Cover of Winter / `f85a157f-d7ec-4e61-97df-2ca851b6c666#card` | 24 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of age counters on this enchantment |
| Cranial Ram / `e958b3c5-0666-4a99-a933-e5cb8b0da9da#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of artifacts you control |
| Craterclaw Colossus / `71355b51-e70e-47b0-8460-933af3567141#card` | 22 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of artifacts you control |
| Craterhoof Behemoth / `8c52bd39-0586-48ca-b263-17210cf9feb6#card` | 22 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creatures you control |
| Cream of the Crop / `b61a87e3-dc98-4db9-abed-b47b677d81ab#card` | 45 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: that creature's power |
| Cruel Calculations / `69b96fe5-9733-4b43-bd66-075742e142d9#card` | 13 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of cards that were put into target player's graveyard from their library this turn |
| Cryptborn Horror / `f5d62baa-648d-4030-87f9-e795ed665084#card` | 50 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the total life lost by your opponents this turn |
| Cultivator of Blades / `ca4c0945-7035-47dc-aba8-fbda351d9b7b#card` | 5 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: this creature's power |
| Curious Herd / `e6f2e363-e361-4571-af37-6c63da04bdbf#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of artifacts that player controls |
| Curry Favor / `11ad6e75-9bbd-40ea-b04a-e6a61db3bcda#face:1` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Knights you control |
| Curse of Vengeance / `b9b1c644-041a-4b61-9d6d-bc73a3a95bb8#card` | 9 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of spite counters on this Aura |
| Custodi Soulbinders / `c7a8daff-9913-4970-8765-52ba9d9d01db#card` | 140 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of other creatures on the battlefield |
| Custodi Soulcaller / `dc25bdde-1c73-4c61-99d8-b772aae278cd#card` | 32 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of players you attacked this combat |
| Cybermat / `ca173221-1661-41f6-a353-29a32d6ccce0#card` | 3 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of attacking artifact creatures |
| Cyclops Electromancer / `638ecc20-a275-4810-9669-89a0730c5362#card` | 4 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of instant and sorcery cards in your graveyard |
| Dai Li Agents / `8c05e716-b6eb-4f95-866b-7e43a898ebe2#card` | 138 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creatures you control with +1/+1 counters on them |
| Dance of the Tumbleweeds / `7c57d4aa-d221-4b36-a544-9fa89d63ffbe#card` | 18 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of lands you control |
| Dancing from Dark to Dawn / `5ba482e9-fbb0-4d9f-a3a9-414892bcdfed#card` | 4 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: that spell's mana value |
| Dead of Winter / `4d877c28-5276-4ea6-91c6-eeed890ca545#card` | 6 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of snow permanents you control |
| Death Mutation / `6f705338-b00b-4a6d-924f-443e7c5670b7#card` | 1 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: that creature's mana value |
| Death's Approach / `7a1798d3-33fb-4092-a843-45a25f3b4e82#card` | 7 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creature cards in its controller's graveyard |
| Death's Presence / `d0db5996-f749-48d2-8fb3-a69104113828#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the power of the creature that died |
| Deathbloom Ritualist / `b52148c3-fe97-4cc0-a814-a5a0f5d426f6#card` | 4 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creature cards in your graveyard |
| Deekah, Fractal Theorist / `6b071146-2f83-4fda-a55a-46fcb74e6f28#card` | 24 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: that spell's mana value |
| Defiling Daemogoth / `d3f7095a-b287-4858-9d08-242cf18e87cb#card` | 12 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the amount of life you gained this turn |
| Deploy to the Front / `135cb4af-f4b6-455b-8fea-84fcef78ee03#card` | 3 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creatures on the battlefield |
| Desmond Miles / `654d84bb-f801-4d03-898c-78952f1c5a73#card` | 210 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the amount of damage it dealt to that player |
| Diamond Valley / `84cef34a-c3e1-4059-b4cd-c481938a53a5#card` | 2 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Dina, Soul Steeper / `2121bcb4-f5fe-447f-9112-c7e5b5c45baf#card` | 2 | where supplement and sacrifice licence | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple; completed-Clause PP supplement with nominal specifying be clause; equated NP: the sacrificed creature's power |
| Dinosaur Egg / `ac4d5a97-6177-4eac-b0f2-cf10531ad879#card` | 1 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: its toughness |
| Disciple of Bolas / `8f2c8498-5b10-4ec7-8678-598c90987556#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: that creature's power |
| Disciple of Freyalise / `2699005b-a471-429f-a9d8-fbf2077ee2fd#face:0` | 3 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: that creature's power |
| Disciple of Griselbrand / `2d92a035-dd7a-4426-a8c0-f04e0b836dad#card` | 2 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Discordant Dirge / `edd4741b-3ee6-421a-8bac-d60736755356#card` | 24 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of verse counters on this enchantment |
| Disturbing Conversion / `adde6737-516c-4d6f-9363-388ad37d4114#card` | 5 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of cards in its controller's graveyard |
| Dockside Extortionist / `697bcfe1-ecbf-42a1-bfc7-0766d48ca56b#card` | 6 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of artifacts and enchantments your opponents control |
| Dokai, Weaver of Life / `cb952c81-c7bb-4670-938d-296d35551986#face:1` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of lands you control |
| Doorkeeper / `8a960cc2-3802-42bc-ba28-941bfe9743be#card` | 7 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creatures you control with defender |
| Dovescape / `d9f4af37-c85e-44ab-9fea-3c67e2f3cf27#card` | 7 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the spell's mana value |
| Dragon Tempest / `b9bacb46-fd1d-459b-81c2-d7d08fe73848#card` | 12 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Dragons you control |
| Dragon Throne of Tarkir / `93e40d56-18fd-42e1-a598-636aab79b836#card` | 12 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: this creature's power |
| Dragonscale General / `9fc6e1db-cf09-4b8c-be4f-bb02ffe7a188#card` | 3 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of tapped creatures you control |
| Draining Whelk / `8ce7f05c-6352-49e8-95b3-8ab87106b75e#card` | 1 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: that spell's mana value |
| Drakestown Forgotten / `b3c8d9ef-4ffb-482b-8859-486e7dc4621e#card` | 40 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creature cards in all graveyards |
| Dreadwaters / `e6e70eb3-3da5-4052-9f62-2960a30d4b41#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of lands you control |
| Dreamborn Muse / `68d7f338-c6e7-4372-b41c-901e39e08bde#card` | 6 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of cards in their hand |
| Dying Wish / `302407c8-0842-4f8e-a5aa-5c11a69851c4#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: its power |
| Elder of Laurels / `0431b67e-ce09-4009-ba5b-9fd41e6f1df4#card` | 4 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creatures you control |
| Eldrazi Linebreaker / `db432387-58d3-4473-9442-b5b586dbea38#card` | 8 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Eldrazi you control |
| Eldritch Pact / `281ea9b6-055a-4274-90b3-c74311a6f863#card` | 3 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of cards in their graveyard |
| Elemental Mastery / `4234f07e-583f-40db-8b47-7e16c7316e19#card` | 14 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: this creature's power |
| Elenda's Hierophant / `cb99c675-adc5-4dad-8be8-524f5f921213#card` | 8 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: its power |
| Elenda, the Dusk Rose / `ce35f262-b93f-4409-8b33-26a7ed7abac8#card` | 64 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: Elenda's power |
| Elturel Survivors / `373ccd23-5c98-4368-995a-e6503cd0dbd8#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of lands defending player controls |
| Elvish Branchbender / `6e9fe674-44c8-4ab4-a0ef-ae71d65bbcc0#card` | 24 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Elves you control |
| Embereth Skyblazer / `783f8411-328a-4a00-9610-a058400cbd6d#card` | 6 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of opponents you have |
| Emissary Escort / `7d8b53f4-bdb5-4ba1-b6c0-f4846d8d3565#card` | 7 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the greatest mana value among other artifacts you control |
| Empowered Autogenerator / `6832f9e2-294a-44a2-8af9-eb16ccfdfc36#card` | 3 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of charge counters on this artifact |
| Endemic Plague / `db982577-1c75-4bc9-ab15-1888ea0be16d#card` | 7 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Endrek Sahr, Master Breeder / `47a0079f-3544-45bc-a32a-bd93844c8c43#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: that spell's mana value |
| Entrapment Maneuver / `2244bef9-1e4e-44ed-9b27-810f04f9e611#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: that creature's toughness |
| Eriette of the Charmed Apple / `f43f5855-f297-4433-984b-18263b734cfc#card` | 8 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Auras you control |
| Errant Minion / `80459049-bd89-4736-82f8-03c603644033#card` | 20 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the amount of mana that player paid this way |
| Erratic Cyclops / `19a11c33-269a-45d3-86c7-827965053f26#card` | 12 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: that spell's mana value |
| Erratic Mutation / `26e01261-2cf1-4746-b066-c2e65e7882d7#card` | 32 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: that card's mana value |
| Escaped Experiment / `d1faaa3e-8e99-48e9-bb04-98bd46b7217c#card` | 6 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of artifacts you control |
| Esper Sentinel / `5def9f38-0a0b-4e8d-9f9d-29dcb46520b4#card` | 6 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: this creature's power |
| Essence Harvest / `ae9a398e-f3a7-4878-a14d-0f6c2dac3d83#card` | 5 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the greatest power among creatures you control |
| Essence Pulse / `0d4bd5e0-8346-4025-8694-d2e1897ee0c2#card` | 8 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the amount of life you gained this turn |
| Estinien Varlineau / `29cf1dc2-3a6a-4a8b-8d61-4b6f0e74f271#card` | 720 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of your opponents who were dealt combat damage by Estinien Varlineau or a Dragon this turn |
| Ethereal Investigator / `24038629-7c12-45ef-8ec4-0d2bb3e2265c#card` | 32 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of opponents you have |
| Excise the Imperfect / `d3264482-2a5a-45e8-987f-593b8aceac12#card` | 1 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: its mana value |
| Exoskeletal Armor / `29358dcd-b26a-4720-9daa-7baa648e3540#card` | 4 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creature cards in all graveyards |
| Exotic Disease / `9480e142-289a-439d-b6a5-2666fb0e5be3#card` | 28 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of basic land types among lands you control |
| Experimental Overload / `7851e4c8-020c-4a3d-88ed-0c97609cdfed#card` | 48 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of instant and sorcery cards in your graveyard |
| Exploding Borders / `1a19fc9e-a822-49ca-ade0-d58054f3caf6#card` | 392 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of basic land types among lands you control |
| Explosive Prodigy / `22d059fc-4e47-44b5-9e21-4a56f1dff6a7#card` | 10 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of colors among permanents you control |
| Faerie Bladecrafter / `164b064e-de50-468a-b874-b054897a5be9#card` | 6 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: its power |
| Faith Healer / `c657e110-c204-48df-a396-9822ac48798d#card` | 2 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Falkenrath Aristocrat / `cd2a87c4-d1c3-4a9e-b2df-e6fe251421c4#card` | 2 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Falkenrath Torturer / `f578a5e0-77e2-4c35-90b2-6628ed9b218e#card` | 2 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Fang, Roku's Companion / `6665bfe2-942c-4b3d-a976-776fe00c99f4#card` | 396 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: Fang's power |
| Fatal Frenzy / `332ccaac-e3f2-4c49-824b-ebbef8a81447#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: its power |
| Fear of Death / `5187c0b2-1996-42ea-bd77-3031b3add596#card` | 3 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of cards in your graveyard |
| Feeding Frenzy / `d14fa263-a6ae-4ab4-b391-2f1ff356fa54#card` | 6 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Zombies on the battlefield |
| Fell Beast of Mordor / `3b64e0a8-7d11-41c5-b439-edec19928b7d#card` | 6 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of +1/+1 counters on it |
| Feral Animist / `beebed34-3a9b-4b51-aacb-1e85d76cf326#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: its power |
| Festive Funeral / `85d7c8d4-6f99-4b1d-9e0e-ff41bbeee682#card` | 6 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of cards in your graveyard |
| Field-Tested Frying Pan / `0c9e3e6a-0bab-4619-b135-d2dbe824b1f0#card` | 66 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the amount of life you gained |
| Final Strike / `4d98aea2-b4ff-4903-ba28-a53fbfaad6b1#card` | 14 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Firebending Student / `1ad3be51-f68b-46d6-b58e-400b50e384f1#card` | 1 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: this creature's power |
| Firebird, Blazing Ranger / `3d917331-9f49-4d0c-bb2d-4853e3ff6ca1#card` | 3 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: Firebird's power |
| Firefist Adept / `0f513242-b175-4b4d-a05f-78b1b457a020#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Wizards you control |
| Flesh / `0741151a-fec0-4ed3-9295-0d0f56d24165#face:0` | 8 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the power of the card you exiled |
| Flesh Carver / `8fcaaf1c-c99b-4f28-bf95-7115a11e803d#card` | 1 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: this creature's power |
| Fling / `24227761-b50e-4b9e-93a2-e82d053b3e3d#card` | 5 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Flurry of Wings / `259b3d11-02e2-43c6-9eed-4ffabb6f9fcf#card` | 8 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of attacking creatures |
| Font of Progress / `ec1fcec2-ba7a-44d7-b251-22a7cd417493#card` | 15 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of oil counters on this artifact |
| Forge Armor / `7d4d9bfa-7e85-461b-bd62-1de97690a110#card` | 1 | where supplement and sacrifice licence | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple; completed-Clause PP supplement with nominal specifying be clause; equated NP: the sacrificed artifact's mana value |
| Formless Genesis / `b978bdfe-a73e-4e99-90d7-4f99182c45ee#card` | 28 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of land cards in your graveyard |
| Fortifying Draught / `a34c78c6-ffbc-4461-a66d-1253a7aa963b#card` | 8 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the amount of life you gained this turn |
| Foundry Helix / `f4f66558-3c99-4488-ad2d-90626a922042#card` | 1 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Freelance Muscle / `8a749efc-e333-448e-8652-2dc4e6633e6b#card` | 57 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the greatest power and/or toughness among other creatures you control |
| Frontline Rush / `b80d21de-2ace-4e0d-92b2-6f0273512621#card` | 4 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creatures you control |
| Fruit of the First Tree / `8d579f04-b494-41a1-ba7c-b9779f75b818#card` | 3 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: its toughness |
| Fungal Sprouting / `303d4533-0c6b-4f93-80e8-b1f5926f4cd4#card` | 5 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the greatest power among creatures you control |
| Furnace Gremlin / `bcbef58b-0c16-40f2-9b5c-14dfcc00f5e5#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: its power |
| Galadhrim Ambush / `3b22d21f-e19a-40df-840b-a2b7f11f79c2#card` | 42 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of attacking creatures |
| Gates Ablaze / `3b97bf8e-d5c4-4eae-a453-73456e0461a1#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Gates you control |
| Gempalm Incinerator / `467ea3e3-7767-405a-9d1a-8fa5de696376#card` | 3 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Goblins on the battlefield |
| General Tazri / `b0f19cba-1339-4518-8320-d7b1dcaf2eb0#card` | 176 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of colors among those creatures |
| Ghost-Spider, Gwen Stacy / `1d95b5c0-d193-4a03-9954-23861ac1c762#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of attacking creatures |
| Ghoul's Feast / `042e0533-faf4-475e-be7b-438d23c6e605#card` | 8 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creature cards in your graveyard |
| Ghoulcaller Gisa / `0dd894cb-1968-471c-af08-ea7ec5ce8428#card` | 1 | where supplement and sacrifice licence | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple; completed-Clause PP supplement with nominal specifying be clause; equated NP: the sacrificed creature's power |
| Glint Raker / `e0a95a8e-5867-481b-9e85-e2d5386362b8#card` | 180 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the greatest mana value among artifacts you control |
| Glissa's Retriever / `d099be2e-1879-4bb0-a27d-a386308387c0#card` | 95 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of opponents who have three or more poison counters |
| Glister Bairn / `6a7229a8-1634-4278-ab50-0ec2b51edac1#card` | 60 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of colors among permanents you control |
| Glittering Stockpile / `feaf9dcb-d845-4cf9-b35d-cd58175162e7#card` | 3 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of stash counters on this artifact |
| Gnarlroot Pallbearer / `18da85d4-2a51-45c0-a18c-bce9bc39fa64#card` | 12 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creature cards in your graveyard |
| Go-Shintai of Ancient Wars / `d145e4d4-a190-4c0b-8dfd-401c0111e995#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Shrines you control |
| Go-Shintai of Hidden Cruelty / `ff8cf669-a38a-44ae-baff-420f59492371#card` | 8 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Shrines you control |
| Go-Shintai of Lost Wisdom / `f6a2092a-093e-4cd6-9eb0-d8ed32da896c#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Shrines you control |
| Gornog, the Red Reaper / `adfd6c2d-a7f8-4f5c-bc31-d65c8150ed49#card` | 4 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Cowards your opponents control |
| Grakmaw, Skyclave Ravager / `9c93acf3-d3da-4b4b-947f-ec8fc99d0d34#card` | 30 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of +1/+1 counters on Grakmaw |
| Graveborn Muse / `d2a65ff4-ab82-42f7-9d90-feb29132121e#card` | 6 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Zombies you control |
| Great Defender / `c84496bc-6421-4930-a118-b0f9ee7e13f6#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: its mana value |
| Greater Good / `dc0593c2-ccb4-4648-a592-c5bcd121dc72#card` | 2 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Grindclock / `6b7de1c0-4b97-47e1-8ea7-ba55a8d16a9b#card` | 3 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of charge counters on this artifact |
| Gruul Beastmaster / `924771b2-8566-4bdf-b089-85c3257b9900#card` | 6 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: this creature's power |
| Gríma Wormtongue / `8f98806e-c6b7-44af-b436-88acf36d25ec#card` | 1 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Gumdrop Poisoner / `4f5ebbb8-f49b-488f-bb85-888aaa918bdd#face:0` | 12 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the amount of life you gained this turn |
| Halana and Alena, Partners / `b347be2d-9aa1-42a9-b652-8fb032832f09#card` | 4 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: Halana and Alena's power |
| Halimar Excavator / `fd3e37c9-93bf-4f3e-a279-22afbffd8d43#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Allies you control |
| Hallowed Spiritkeeper / `5852ed2f-4332-4c30-b145-373deea471d9#card` | 32 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creature cards in your graveyard |
| Hamletback Goliath / `ae45b04e-c52d-45e1-a752-0b76f64d7c8d#card` | 1 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: that creature's power |
| Hancock, Ghoulish Mayor / `adf9aed9-dd63-48da-b799-ea94839fc22a#card` | 21 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of counters on Hancock |
| Hapatra, the Desert Fang / `6894345f-52a6-46e4-b278-0783087daee1#card` | 42 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the greatest mana value among cards in your graveyard |
| Harabaz Druid / `ead985ec-f29f-4a3b-b8b1-061142cc5bd1#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Allies you control |
| Hare Raising / `3ef02f2b-b611-40c3-a1d5-8b34989643cf#face:1` | 6 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creatures you control |
| Harpoon Sniper / `2856844d-f727-48aa-8387-cb6079afb40c#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Merfolk you control |
| Harsh Sustenance / `f5cf471f-032c-4ebb-823c-8841de209216#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creatures you control |
| Haunt the Network / `7e074b14-0ec8-439b-9b83-5f7c1a904384#card` | 32 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of artifacts you control |
| Havoc Eater / `769c511c-37ce-4a9d-85f8-1ba91c2daf82#card` | 20 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the total power of creatures goaded this way |
| Hazezon Tamar / `d298df4e-7b60-4495-9773-cc81954b7dd9#card` | 280 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of lands you control at that time |
| Hedron Matrix / `dc748941-fd91-4a0e-b0c4-04c9092ff158#card` | 1 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: its mana value |
| Hellfire / `e7b1975d-9574-4333-8530-33167948078a#card` | 4 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creatures that died this way |
| Hellish Sideswipe / `d919d8e9-d1ba-42de-9884-12e38dca78ac#card` | 1 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Hellkite Igniter / `12aba1d4-477f-4d36-96bb-c3c7bbd51b75#card` | 4 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of artifacts you control |
| Hemosymbic Mite / `103e0b63-fbde-4054-91f7-dd04b40b59ef#card` | 6 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: this creature's power |
| Heroes' Bane / `bccd722a-b610-4017-93bd-313f08448f5e#card` | 5 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: its power |
| Heronblade Elite / `e3e255e0-ab04-4488-bdc1-bb1542f51e16#card` | 1 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: this creature's power |
| Hew the Entwood / `8818bbf8-58ec-4070-a81a-c7ce248057ab#card` | 3456 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of lands sacrificed this way |
| Hoard-Smelter Dragon / `cf9db2be-3c95-45a2-ab85-694e3bd3bf1d#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: that artifact's mana value |
| Homarid Spawning Bed / `173f2b8e-8072-4fdd-a469-8a1fb0bb6e64#card` | 1 | where supplement and sacrifice licence | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple; completed-Clause PP supplement with nominal specifying be clause; equated NP: the sacrificed creature's mana value |
| Hope Estheim / `187a110c-032a-42e1-be2d-3cfaaa9495f8#card` | 4 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the amount of life you gained this turn |
| Horn of Gondor / `88777fc0-286d-47fc-8e94-1e289b4964c4#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Humans you control |
| Hurl into History / `016718df-2eb5-499f-adea-18231c0375b3#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: that spell's mana value |
| Hurska Sweet-Tooth / `8d49b6a1-4ae2-4443-a6b3-9ef806cb6fc4#card` | 6 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the amount of life you gained |
| Hydra Trainer / `c428cbe2-17fd-4bd2-9810-de2561519f14#card` | 54 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of counters on permanents you control |
| Ichor Explosion / `d33aa11b-011b-4d12-85a9-4f956153fb1d#card` | 2 | where supplement and sacrifice licence | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple; completed-Clause PP supplement with nominal specifying be clause; equated NP: the sacrificed creature's power |
| Idolized / `8cbf0173-2349-4205-b720-378643c347a4#card` | 9 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of nonland permanents you control |
| Ill-Timed Explosion / `6e3b3a3f-e32f-4c72-a9fb-dd1e877876b6#card` | 10 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the greatest mana value among cards discarded this way |
| Illuminor Szeras / `10f274f9-9044-447b-81c9-d4906ad9e6be#card` | 2 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Immolating Gyre / `1888b473-e26a-4882-ae18-5a0a6caee410#card` | 8 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of instant and sorcery cards in your graveyard |
| Impelled Giant / `eb7c9442-7bfa-4bcf-a394-b5de187f224a#card` | 48 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the power of the creature tapped this way |
| In the Eye of Chaos / `6a0da9f3-cb06-42b1-ae73-b142c0eedaff#card` | 3 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: its mana value |
| Incendiary / `a893bee3-ea5c-4619-a875-e4af6d4396fe#card` | 3 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of fuse counters on this Aura |
| Infantry Shield / `71c68002-1489-4858-a8fb-1f575641983e#card` | 1 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: its power |
| Inferno Project / `36d641d4-d3f5-4394-81d7-3209955a1628#card` | 45 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the total mana value of instant and sorcery cards in your graveyard |
| Inner Calm, Outer Strength / `b7bdbae5-549f-403b-83ca-4f9a1ac93e93#card` | 6 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of cards in your hand |
| Intelligence Bobblehead / `16e6bd4b-fbf1-4bbe-9271-a74f59320e0d#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Bobbleheads you control |
| Invade the City / `583a4744-89fb-4bd6-8d4c-c3d845e2e19c#card` | 4 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of instant and sorcery cards in your graveyard |
| Invasion of Lorwyn / `c877a430-75bc-45ac-b11d-2365d11ed341#face:0` | 22 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of lands you control |
| Invasion of Tarkir / `5c7f02ad-1daf-4d1a-bef0-0b2064f9b67e#face:0` | 24 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of cards revealed this way |
| Iron-Craw Crusher / `b04f45de-cb35-45a4-b039-0bf008c3ca56#card` | 3 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: this creature's power |
| Ivy Seer / `f61591d5-0f2a-48cd-a091-863d2e93e019#card` | 40 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of cards revealed this way |
| Jarad, Golgari Lich Lord / `87e65e36-9483-49fe-b644-2caca092107f#card` | 42 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Jazal Goldmane / `609d13dc-c850-4877-8ad3-e84171b58069#card` | 4 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of attacking creatures |
| Jessica Jones, Private Eye / `6677cd5a-0785-499b-ba99-f6bdcfb196d5#card` | 6 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: Jessica Jones's power |
| Jor Kadeen, First Goldwarden / `967897db-a34b-493c-bd48-04e2650902d2#card` | 9 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of equipped creatures you control |
| Judgment Bolt / `aad9f9f2-4c6b-48d0-80bd-77b73a9ea5f1#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Equipment you control |
| Junkyo Bell / `2935b603-d459-4f50-b244-0746febaaacd#card` | 30 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creatures you control |
| Kagemaro's Clutch / `f83f6ba0-bb23-4c62-8a16-b17f5fbc0740#card` | 3 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of cards in your hand |
| Kate Stewart / `c1b9475e-a272-4820-b5f0-25f7a94ff834#card` | 30 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of time counters among permanents you control |
| Kazuul's Fury / `f8410804-632b-4f18-9a73-6dccc7e4582d#face:0` | 5 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Keening Stone / `d0f06250-25a5-4003-8f3a-6d85ef32ea10#card` | 5 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of cards in that player's graveyard |
| Keldon Battlewagon / `4a2df44b-5d36-4c4e-b982-836d9d7e0505#card` | 48 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the power of the creature tapped this way |
| Kheru Dreadmaw / `8c8c9f16-a3d0-49f2-a95b-dfef756de0b3#card` | 2 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Kin-Tree Invocation / `fbb74ff8-7381-4de9-bf62-e5d181a187bc#card` | 5 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the greatest toughness among creatures you control |
| Kinbinding / `da35de40-6fde-4018-bda8-b0ae91ed872a#card` | 13 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creatures that entered the battlefield under your control this turn |
| Kinscaer Sentry / `fdf0fba8-f335-419c-b96b-e99037727c43#card` | 9 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of attacking creatures you control |
| Kithkeeper / `5c5766f9-befd-4426-9527-d50af0699ca1#card` | 60 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of colors among permanents you control |
| Kitsune Loreweaver / `d989d8aa-2b2a-4fd2-a8c9-1d182909987c#card` | 6 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of cards in your hand |
| Kraul Harpooner / `332b6ed7-90de-46c0-85ad-1ad708b2c1e1#card` | 208 historical wrong Readings, retired; corrected count below | where supplement alone, corrected by follow-up | Medial PP supplement of the left Clause anchor; specifying be clause; equated NP **the number of creature cards in your graveyard** (definite NP headed by *number*, *of creature cards* and *in your graveyard* postmodifiers; 3 independently admitted NP Readings). The intended graveyard restriction is inside the creature-card NP; higher postmodifier attachment remains a grammatical Reading. The `then you may have …` Clause continues the outer host and is outside the definition. |
| Krenko, Mob Boss / `68418069-f615-40ef-ae0d-764192acae00#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Goblins you control |
| Kresh the Bloodbraided / `6343cf6f-b1e3-49ce-8933-8fe6c9dc83f3#card` | 1 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: that creature's power |
| Kry Shield / `2c4bd475-b8af-4916-b7a0-68abb8994138#card` | 64 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: its mana value |
| Kuldotha Cackler / `e1a41091-6ff1-42bb-b248-90040107efe1#card` | 69 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of permanents you control with oil counters on them |
| Kura, the Boundless Sky / `6426e73a-7e11-48ab-ac5c-15647c6cf9ba#card` | 14 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of lands you control |
| Kyoshi Island Plaza / `c0cdc333-6c5a-401c-9810-220d2027f2fe#card` | 960 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Shrines you control |
| Leyline Invocation / `9f2e713f-c7d5-4c91-9934-9e36888244fd#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of lands you control |
| Lich Lord of Unx / `6643fc8e-548e-48db-9f67-ffbca9e57016#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Zombies you control |
| Life's Legacy / `cb7baa46-7963-4d45-9a6e-e3c34db3c0e6#card` | 1 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Liliana's Standard Bearer / `7b600c0e-5ad2-48a2-a147-346ca78c8629#card` | 9 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creatures that died under your control this turn |
| Lilting Refrain / `f91a71b7-9bc0-4609-a312-ffede52dc0ad#card` | 6 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of verse counters on this enchantment |
| Living Armor / `7a32b0d1-d9dc-4d24-ae60-3f3a0e18ec03#card` | 1 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: that creature's mana value |
| Lydia Frye / `0315f5e7-2867-428e-8e43-66dda0db57a0#card` | 57 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of tapped Assassins you control |
| Lys Alana Scarblade / `b7ebd207-43e3-49eb-86ca-b5987ef9e22f#card` | 4 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Elves you control |
| Lyzolda, the Blood Witch / `0d80caeb-d2c2-4d0a-b903-dea170fee2cf#card` | 4 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Maester Seymour / `e7e41303-69a8-4617-b418-142cf41edb9b#card` | 40 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of counters among creatures you control |
| Magma Sliver / `008ac258-763c-4379-bfcd-b9cc4d7296dd#card` | 6 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Slivers on the battlefield |
| Manaplasm / `9061e2ae-7385-494a-a511-0b6531d1fe0e#card` | 6 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: that spell's mana value |
| Manifestation Sage / `d8bcd896-cae9-4733-a437-165644b8b0e8#card` | 3 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of cards in your hand |
| Marrow-Gnawer / `2b934761-5720-477b-94b7-d1d91dc581a6#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Rats you control |
| Marshal of the Lost / `af762373-e159-4a36-83b5-a82b8b16fa41#card` | 3 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of attacking creatures |
| Mask of Griselbrand / `b46da6bd-578c-4040-97c3-0005b78e31be#card` | 1 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: its power |
| Mask of the Schemer / `584f9c45-9737-47bb-8ba4-a7cd9b0b2c1f#card` | 15 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the amount of damage it dealt to that player |
| Master Pakku / `b1727c07-c829-4d64-a1c5-8b53c743888a#card` | 4 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Lesson cards in your graveyard |
| Mausoleum Wanderer / `1803850e-9499-4bf7-a4d9-6bf655cbe87d#card` | 12 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: this creature's power |
| Meishin, the Mind Cage / `3b8eea0b-6f8c-449c-9ab8-a22a035465d6#card` | 3 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of cards in your hand |
| Mercy Killing / `4d80bac5-ce62-4c5c-873a-ce81d2697842#card` | 1 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: that creature's power |
| Midnight Snack / `3ddcaee9-a11e-477a-bd19-f5ee10512549#card` | 16 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the amount of life you gained this turn |
| Midsummer Revel / `d3579282-5045-4b37-9998-9a282e9932b9#card` | 3 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of verse counters on this enchantment |
| Miming Slime / `50e7f705-c347-440b-8159-81a52bbfbdfb#card` | 5 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the greatest power among creatures you control |
| Mind Extraction / `0077740a-528b-4ee3-b331-fac321b95302#card` | 1 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Minions' Murmurs / `fd755ec1-6727-4165-a7fd-5d9b59754ada#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creatures you control |
| Miren, the Moaning Well / `03fe19bb-8e22-4030-8299-2ddd2d5a7eb2#card` | 2 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Mirkwood Channeler / `30f373d8-cd8c-4fb2-9383-9969fc547df9#card` | 8 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Forests you control |
| Mister Gutsy / `09763d9a-056f-49db-a72b-1c7325075789#card` | 12 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of +1/+1 counters on it |
| Mob Mentality / `90e8fb2c-6473-42c3-92f4-f46a744920fa#card` | 6 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of attacking creatures |
| Momentous Fall / `6a5ce1b0-ac78-4a74-9eb7-4064e1687bf1#card` | 1 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Mona Lisa, Science Geek / `e6490e98-37a9-4a8b-a04c-cceeb22b0c35#card` | 1 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: Mona Lisa's power |
| Monkey Cage / `33a11c01-89a6-446b-84da-e48fd1f27446#card` | 3 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: that creature's mana value |
| Monoist Circuit-Feeder / `de70a6bb-32ed-4a89-a522-c2a71f08fb02#card` | 6 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of artifacts you control |
| Monsoon / `a86e7de8-8448-406c-b162-22c62bdc3813#card` | 32 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Islands tapped this way |
| Monstrous Vortex / `6e27956e-ba2f-41a9-8e39-ec54424591b6#card` | 6 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: that spell's mana value |
| Monumental Corruption / `0b49924b-cf83-46d9-b665-9c718a40e16c#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of artifacts you control |
| Moodmark Painter / `f5068b54-b1f8-4751-9a09-fc1b554e8115#card` | 32 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creature cards in your graveyard |
| Moonshaker Cavalry / `b3f16f34-d78c-4b61-821b-05fc6bd0da37#card` | 22 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creatures you control |
| Moonveil Regent / `61c2a2cb-6424-4a20-81fb-f8e15a8f95ce#card` | 40 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of colors among permanents you control |
| Morbid Curiosity / `59c620fb-a58d-4038-a7e2-0185324b5123#card` | 2 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Mortarion, Daemon Primarch / `588eecbf-dcfc-4b97-b81c-8b9f3d39e08f#card` | 54 | variable Subject projection | ordinary singular third-person variable Subject; existing nominal predicative be frame; modal negative comparative complement |
| Moseo, Vein's New Dean / `c2e2f21a-0338-44d7-96a5-5a277f0ec419#card` | 600 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the amount of life you gained this turn |
| Murder Investigation / `42ca56c5-a9c4-4a2c-ae05-bce47aa6e16c#card` | 1 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: its power |
| Myrel, Shield of Argive / `503094aa-6f8f-42c1-ad5a-9305934e5f0c#card` | 32 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Soldiers you control |
| Mystic Genesis / `58940a7e-3827-43fa-8bf5-cc88dc295df0#card` | 1 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: that spell's mana value |
| Nantuko Mentor / `b79378e7-99db-403f-8f63-4d71ebdb3f6c#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: that creature's power |
| Nasty End / `f73da5d8-fd15-4315-ad3c-c86c28087285#card` | 3 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Naya Soulbeast / `5ea0c608-2c56-4889-a5d3-d435df515950#card` | 200 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the total mana value of all cards revealed this way |
| Necrotic Wound / `a30159ae-f6a6-4e29-bca2-769d3657d310#card` | 96 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creature cards in your graveyard |
| Nested Shambler / `bfa882ee-de18-4ef8-8957-3e04ed6e3c1e#card` | 1 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: this creature's power |
| Nightmare's Thirst / `9ecf56b9-1f36-4030-8333-1675ccc663d9#card` | 8 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the amount of life you gained this turn |
| Nightmarish End / `70f5e932-0dc5-4c34-b7be-16cd80ec10b7#card` | 6 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of cards in your hand |
| Nightshade Seer / `9bcf66d0-8a4d-4c57-b4de-4cc35399465e#card` | 40 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of cards revealed this way |
| Nils, Discipline Enforcer / `ff6b52b7-ae3c-465b-b10e-901160e5230e#card` | 48 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of counters on that creature |
| Northern Air Temple / `28a2ad5b-d637-4634-8d4b-889e1fc88319#card` | 8 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Shrines you control |
| Nyla, Shirshu Sleuth / `3492cee8-b854-449a-ada4-a4e8a4333d87#card` | 72 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: that card's mana value |
| O-Kagachi Made Manifest / `4d23b22b-31a5-4eec-b894-2dd5a7138c5a#face:1` | 20 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the mana value of that card |
| Oath of Jace / `dc92b420-8ac7-4478-a287-3d897357be08#card` | 6 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of planeswalkers you control |
| Oboro Envoy / `be70c6e8-6f9f-49fb-ab40-c6ce0ec2077c#card` | 12 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of cards in your hand |
| Ogre Battlecaster / `00ba0c24-a671-493e-ba46-13e45d1818f1#card` | 2340 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: that spell's mana value |
| Olivia's Wrath / `9aebd891-c8c5-4309-a07b-6cab1b204172#card` | 4 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Vampires you control |
| Omega, Heartless Evolution / `3173ca98-d46a-4de3-a95e-3eaad28d4017#card` | 9 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of nonbasic lands you control |
| Onward / `9a52c1ea-35ac-4028-8c1f-b7971c1df886#face:0` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: its power |
| Opaline Bracers / `a4d6cb66-698d-4b76-b922-f799ad9d8805#card` | 3 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of charge counters on this Equipment |
| Orcish Siegemaster / `d852099f-46c6-4676-860c-20a582f733d1#card` | 90 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the greatest power among creatures you control |
| Ossuary Rats / `a1380d07-f582-46dd-a63a-9806123ebcd3#card` | 8 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creature cards in your graveyard |
| Ouroboroid / `50d6fd91-23d3-4d32-804f-6233e4386904#card` | 1 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: this creature's power |
| Overwhelming Stampede / `e1e96802-fd0c-41f1-aa21-3287d75a0e88#card` | 15 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the greatest power among creatures you control |
| Oviya Pashiri, Sage Lifecrafter / `41e03224-bcdd-4127-a19a-fe66196339b9#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creatures you control |
| Pack Attack / `84b385cd-9dfd-4513-af3c-20fc1cc8fea0#card` | 4 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of players being attacked |
| Pact of the Serpent / `f275039e-3dba-49b0-92aa-92721b1c7f95#card` | 3 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creatures they control of the chosen type |
| Painbringer / `6ca49880-4e12-494b-b14e-e49385d0f7f5#card` | 24 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of cards exiled this way |
| Paradox Zone / `2fff8441-1f5a-423b-ab98-93016266f5c5#card` | 75 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of growth counters on this enchantment |
| Parting Thoughts / `57fd34d3-3e30-4467-bace-3f4dc2a85c80#card` | 3 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of counters on that creature |
| Pathbreaker Ibex / `7e52e91b-70e6-4f26-a5d4-b8e33d1debbf#card` | 55 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the greatest power among creatures you control |
| Phantasmal Sphere / `4230c7a3-4591-411e-afab-d2bfb58446de#card` | 2976 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of +1/+1 counters on this creature |
| Phyrexian Processor / `36c800cb-b1ca-4432-ad3c-4d8b90337f4c#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the life paid as this artifact entered |
| Phyrexian Rebirth / `6ef3c75d-6af2-4ea0-b98d-96c5d7d3af58#card` | 8 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creatures destroyed this way |
| Pia, Determined Rebuilder / `a49c0c64-ea87-44f4-9acf-e7181962aeb2#card` | 72 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of artifacts you control |
| Plasm Capture / `3673219d-d2b5-43ad-b0c8-58b583491158#card` | 32 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: that spell's mana value |
| Point the Way / `0ffe9c8b-e0cb-4a29-bc14-649dbec5d1f3#card` | 32 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: your speed |
| Power Leak / `dc2f0000-870b-487f-9623-618fc8eb9765#card` | 20 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the amount of mana that player paid this way |
| Power Surge / `156b2228-f7b2-4816-b894-c4953a32c05f#card` | 24 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of untapped lands they controlled at the beginning of this turn |
| Priest of Yawgmoth / `cb6465f9-dcf8-4258-aa18-661ad252b58b#card` | 2 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Priest of the Crossing / `524e5098-9c6d-43ad-80cb-52fd6d14ef53#card` | 9 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creatures that died under your control this turn |
| Prime Speaker Zegana / `311e9368-696a-47e7-aa2f-3ef1b1a92e2b#card` | 35 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the greatest power among other creatures you control |
| Prismabasher / `f75be4f2-d1a7-40c7-a0e4-ba8b03556ac6#card` | 60 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of colors among permanents you control |
| Prismatic Geoscope / `be838f57-4e74-44ad-aee1-506e63b0ff37#card` | 168 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of basic land types among lands you control |
| Prismatic Undercurrents / `27a3aa64-1de7-4b0c-af23-52886e7d258a#card` | 500 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of colors among permanents you control |
| Profane Prayers / `6eb427e9-e679-4891-a29e-ea54b1935892#card` | 3 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Clerics on the battlefield |
| Promise of Power / `fde459b2-f45f-40d2-b328-58f65440f494#card` | 21 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of cards in your hand |
| Pummeler for Hire / `ffa23c3a-76a8-414f-aa3b-fc09aabd78c5#card` | 5 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the greatest power among Giants you control |
| Pure Reflection / `fee53820-a17c-4f34-9971-581bf8e12fd6#card` | 1 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the mana value of that spell |
| Putrid Cyclops / `8bcb0f45-902d-41d6-a10c-60bc27befd30#card` | 12 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: that card's mana value |
| Pyrrhic Blast / `98ec58aa-8776-4a19-bff8-e288a9bd6bac#card` | 5 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Qala, Ajani's Pridemate / `1fc1b0d7-7f15-4d00-bc8f-e06489afd425#card` | 27 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of counters on Qala |
| Quartzwood Crasher / `178a79e2-64fd-4777-95fe-98b7834a642a#card` | 270 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the amount of damage those creatures dealt to that player |
| Quirion Beastcaller / `d777e428-cc50-40b1-a241-41576d2575d1#card` | 60 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of +1/+1 counters on this creature |
| Rabble-Rouser / `1d591c00-7086-4337-bdb8-a5f35568f04e#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: this creature's power |
| Radha, Coalition Warlord / `fbf0bde7-cb13-402b-b3ab-81c787ce4827#card` | 504 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of basic land types among lands you control |
| Raffine's Silencer / `2901948c-2a5a-4135-9135-27406e9d8652#card` | 3 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: this creature's power |
| Raffine, Scheming Seer / `9307f7e9-bcd9-41f9-88f8-0fb9700a6b12#card` | 1 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of attacking creatures |
| Railway Brawler / `b653db44-ddae-44e8-8fa6-6af3232cd586#card` | 1 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: its power |
| Realm Seekers / `ccb66cdc-909a-42c1-a10f-3886e9ed1936#card` | 245 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the total number of cards in all players' hands |
| Reap / `7fcb8472-1e77-4dcd-b18f-8abf8806da08#card` | 60 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of black permanents target opponent controls as you cast this spell |
| Recantation / `02269353-2949-4599-b5a3-f20fdb501285#card` | 6 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of verse counters on this enchantment |
| Reckoner's Bargain / `044ea111-ce87-49ab-98f7-ae8447f48ac5#card` | 2 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Reign of the Pit / `3e1a6337-bb29-450f-bf29-ff71b357a983#card` | 70 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the total power of the creatures sacrificed this way |
| Rescue from the Underworld / `7217ea53-3685-48d6-9df1-58a551f8c979#card` | 48 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Resilient Khenra / `eb23f999-e7f9-4082-b322-5b00ba64b458#card` | 5 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: this creature's power |
| Rethink / `ede61683-fa78-4103-8248-e0e64162ed49#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: its mana value |
| Rhovanion Rampager / `008a11c1-d283-49fe-abd7-ff4fe8b1fe79#card` | 2 | where supplement and sacrifice licence | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple; completed-Clause PP supplement with nominal specifying be clause; equated NP: this creature's power |
| Riptide Replicator / `f526ce8a-27f5-4785-aed3-944b080c9c1a#card` | 180 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of charge counters on this artifact |
| Rise of the Varmints / `d981977d-acfa-432a-b1d7-1808a89a5998#card` | 4 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creature cards in your graveyard |
| Rite of Consumption / `327121a1-f193-44c6-a834-802095abec84#card` | 42 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Rockalanche / `d1ecfad3-e79a-447b-a932-68ef728eaf1f#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Forests you control |
| Rodolf Duskbringer / `26a716b2-a2e2-405b-accd-df572416d8bf#card` | 96 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the amount of life you gained this turn |
| Rolling Hamsphere / `a6c3aa17-8338-4daa-889e-d36dc715e546#card` | 8 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Hamsters you control |
| Rot Hulk / `36117e99-a2bc-46f9-8e17-bd2e2a9c31f4#card` | 14 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of opponents you have |
| Rubblebelt Rioters / `d750f1d1-a3f9-4180-a903-2bd3f4cb9674#card` | 15 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the greatest power among creatures you control |
| Ruinous Intrusion / `4f945baa-40a9-4494-b2f9-f1926f3fec84#card` | 4 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the mana value of the permanent exiled this way |
| Rumbling Crescendo / `2e7e19c2-5399-433d-9653-d956f11838f5#card` | 3 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of verse counters on this enchantment |
| Runechanter's Pike / `2efea460-c3e1-4578-a587-134e13bdf264#card` | 4 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of instant and sorcery cards in your graveyard |
| Rush of Blood / `f0bacbf1-3c35-436e-b0bf-10613b60b726#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: its power |
| Sacrifice / `068b3692-411b-44d4-a7e9-005262760cfc#card` | 2 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Sanctum Weaver / `acfe7ec0-0606-4d5e-b1fa-25f0c7aeec47#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of enchantments you control |
| Sanctum of Calm Waters / `1cd1e3a8-e09d-4b61-8244-92b317132d6a#card` | 8 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Shrines you control |
| Sanctum of Fruitful Harvest / `132859dd-de66-45c6-8af4-ab5e202a17b0#card` | 8 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Shrines you control |
| Sanctum of Shattered Heights / `5e39d311-efb5-4f51-82cd-e8079e32907a#card` | 4 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Shrines you control |
| Sanctum of Stone Fangs / `a1d8fcb9-26b2-4d09-9db7-e644294749f8#card` | 16 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Shrines you control |
| Sardian Avenger / `6db90f86-205a-4e0b-944b-74742bc4e59d#card` | 18 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of artifacts your opponents control |
| Sardian Cliffstomper / `20f3bc44-c7fa-4d5a-adb9-2dd1d2d66ffe#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Mountains you control |
| Saruman, the White Hand / `2bade11e-04e0-42a2-8861-9257c99a7c08#card` | 6 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: that spell's mana value |
| Savanti Romero, Time's Exile / `6074632b-4520-4ae8-bcdc-26b1eff71af9#card` | 9 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of counters on Savanti Romero |
| Scent of Cinder / `ff50d002-834e-4b5e-8db0-b0c8b4efb21f#card` | 20 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of cards revealed this way |
| Scent of Ivy / `b6db81e5-b7ca-45dc-a72d-1ff12935a3e4#card` | 40 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of cards revealed this way |
| Scent of Nightshade / `2e88f39d-6177-42d9-b756-144219dc0961#card` | 40 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of cards revealed this way |
| Scourge of Fleets / `57308fc8-8915-4211-a5fd-9363caea9ab9#card` | 8 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Islands you control |
| Scourge of Skola Vale / `725d7374-4ea8-4aef-ac09-8dcb5d8dcad1#card` | 10 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Scourge of Valkas / `0a147009-a57c-4ac1-ab81-d8bd8b737631#card` | 4 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Dragons you control |
| Seed Guardian / `728045bf-0959-44ff-8de9-16ec35fd248f#card` | 4 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creature cards in your graveyard |
| Self-Destruct / `98bf6311-bc9b-4bf8-ac83-c041973b86d0#card` | 1 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: its power |
| Selvala, Heart of the Wilds / `1d725121-e50c-42f0-9128-56802f07c89e#card` | 120 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the greatest power among creatures you control |
| Serra's Liturgy / `17b7b7e4-4ae8-495d-bf0c-d18697f250ea#card` | 6 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of verse counters on this enchantment |
| Servant of the Scale / `7d404420-e3de-47a8-a159-2f634a73f2c9#card` | 15 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of +1/+1 counters on this creature |
| Severed Strands / `25f0527c-1340-4218-970f-9e4f84ef96e8#card` | 2 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Shadow, Mysterious Assassin / `13ca49a2-8a78-4be4-976a-b684086864bb#card` | 12 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Shagrat, Loot Bearer / `23db92aa-d4db-49e3-a5bc-7b120e02822f#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Equipment attached to Shagrat |
| Shanna, Purifying Blade / `7677c7ce-1db3-41f5-8f86-e68718ea2d11#card` | 6 | variable Subject projection | ordinary singular third-person variable Subject; existing nominal predicative be frame; modal negative comparative complement |
| Shark Typhoon / `8c0520fa-276b-4d21-b4a9-dce1fce59f6b#card` | 128 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: that spell's mana value |
| Shield of the Avatar / `a4433da6-78b7-4669-a0dd-e7ea9672b0bc#card` | 4 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creatures you control |
| Shimmercreep / `52244c23-7680-4bed-8ef8-8d10e289d3f7#card` | 20 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of colors among permanents you control |
| Shriekwood Devourer / `56314980-5f18-4b93-a9a0-6002ee8a880c#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the greatest power among those creatures |
| Siani, Eye of the Storm / `ecf808fd-4828-497e-b84b-1cb018a9924b#card` | 4 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of attacking creatures with flying |
| Sierra, Nuka's Biggest Fan / `e4fb15b6-02bb-4923-953a-69784ca780b8#card` | 81 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of quest counters on Sierra |
| Sigardian Zealot / `30aecffc-bbbf-44d0-8002-0ee5982f27b9#card` | 15 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: this creature's power |
| Silvergill Douser / `b24f2917-b96c-4348-b033-4926df3bd4b0#card` | 12 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Merfolk and/or Faeries you control |
| Sokenzan Spellblade / `26e47cc5-9dc7-4d54-80a6-1eb77587df36#card` | 6 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of cards in your hand |
| Soul Immolation / `338747e8-bed8-4e60-8b19-5c2b80799477#card` | 5 | variable Subject projection | ordinary singular third-person variable Subject; existing nominal predicative be frame; modal negative comparative complement |
| Soul Tithe / `60ff5af4-eb95-4544-9f63-d42c22112486#card` | 15 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: its mana value |
| Soul's Might / `a3538a5e-3d08-4c61-bcb4-ef1f5e5b659d#card` | 1 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: that creature's power |
| Soulblast / `18d4c57b-e2bf-47a0-8823-c4a79498a7ff#card` | 14 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Soulshriek / `77ae796e-fc56-47b9-b8a0-97e24fbd60f7#card` | 16 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creature cards in your graveyard |
| Southern Air Temple / `987c1144-69f6-4441-b0a4-d23f931df81f#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Shrines you control |
| Sparksmith / `b27bc09a-ebea-498c-87f2-753852569a73#card` | 3 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Goblins on the battlefield |
| Specter of Mortality / `caff40b1-28d1-4ef6-90d1-51a1dc06cbcd#card` | 72 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of cards exiled this way |
| Spectral Deluge / `bb9bb65e-504f-4edd-8d47-d0efa03e7d17#card` | 8 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Islands you control |
| Spell Rupture / `934c50aa-7d2e-4c9e-b91b-1e8c4055e1c6#card` | 10 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the greatest power among creatures you control |
| Spell Swindle / `81fa8e6a-5fb8-43de-802f-df5b985c6a7d#card` | 1 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: that spell's mana value |
| Spellstutter Sprite / `32e60fb4-e841-4f82-9c83-5e63766e8e6f#card` | 8 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Faeries you control |
| Sphere of Safety / `92f6c063-a740-4c3c-a60a-569fd298854d#card` | 30 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of enchantments you control |
| Spider-Man Noir / `2efd6494-c5e1-41de-bc6e-8490ff046cd1#card` | 3 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of counters on it |
| Splitting the Powerstone / `178828a5-c202-4503-9755-fc6bb2390209#card` | 1 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Spoils of Blood / `26516566-c837-428d-bc81-582c74da2c28#card` | 4 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creatures that died this turn |
| Spoils of War / `f3240a86-28b6-4338-9a59-1fb6f795d713#card` | 90 | variable Subject projection | ordinary singular third-person variable Subject; existing nominal predicative be frame |
| Spontaneous Mutation / `ab78fa13-f599-4d08-bd27-d3d8740ae58f#card` | 3 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of cards in your graveyard |
| Spotter Thopter / `ccf13531-60bd-4ebd-9a5b-68e2d05ce620#card` | 1 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: its power |
| Spymaster's Vault / `69ddca4b-5cc0-45f3-b2e6-a047c8d601be#card` | 8 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creatures that died this turn |
| Staff of Titania / `f99f4b52-21ae-47a2-8ae6-b0aa7386723a#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Forests you control |
| Stag Beetle / `75b2da25-e551-4add-a252-6d4789366b5f#card` | 20 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of other creatures on the battlefield |
| Starlit Sanctum / `d16298ac-67bd-4f9d-9979-23c1b7e4b359#card` | 4 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Stirring Honormancer / `3bcf54f8-fcd7-4ec0-b011-92b367411cdb#card` | 6 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creatures you control |
| Stitcher Geralf / `da394a25-79c9-4b75-9ce6-741f64ae6913#card` | 40 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the total power of the cards exiled this way |
| Storm, Queen of Wakanda / `d8e585c5-1de3-4558-8a2f-0d9fb07edad6#card` | 81 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: Storm's power |
| Stormchaser Chimera / `fccc57f6-2555-4ab3-b85a-83b524abe892#card` | 8 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: that card's mana value |
| Strength from the Fallen / `23643af9-00b7-45e1-8471-7031d08bbe30#card` | 24 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creature cards in your graveyard |
| Strength in Numbers / `23195903-04e1-4461-a4ac-f0ce39f21c20#card` | 1 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of attacking creatures |
| Strength of Cedars / `61b29c75-00d0-4ddb-9e27-cfd47302830e#card` | 4 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of lands you control |
| Subdue / `81bac4b8-277b-415a-9064-a80a68fd7051#card` | 56 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: its mana value |
| Summon: Titan / `66ce0c60-f406-4ece-915f-675da4e8b16d#card` | 28 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of lands you control |
| Summon: Yojimbo / `ff70d743-280e-476f-a3b8-92026419419d#card` | 210 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of opponents who control a creature with power 4 or greater |
| Sun Warriors / `513a2fca-fea4-434e-9b25-b0c0d7f322b2#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creatures you control |
| Sunbringer's Touch / `bbfced8f-0f83-4cc9-9143-aa2473828b48#card` | 30 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of cards in your hand |
| Sunfall / `fb3f5097-0da0-458c-8508-60823567e2da#card` | 4 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creatures exiled this way |
| Sunflare Shaman / `0391691f-6708-481a-93a5-6677e6fa2ff3#card` | 4 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Elemental cards in your graveyard |
| Surge of Strength / `bd3e55cf-ffca-402c-91ea-2420035a9dd1#card` | 3 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: that creature's mana value |
| Surrakar Spellblade / `e87813a3-893d-4416-9263-2ea2e3140490#card` | 36 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of charge counters on it |
| Syr Faren, the Hengehammer / `2158fb0d-c199-4cb1-848c-077113348980#card` | 3 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: Syr Faren's power |
| Tarox Bladewing / `e195f2f8-2db7-46b8-948c-c3c27f2e5825#card` | 4 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: its power |
| Team Avatar / `f95f7b0c-2dd6-46e1-ae02-f51d275e7120#card` | 84 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creatures you control |
| Temple of Aclazotz / `be2a4bc4-8af6-48c5-9421-32d26272e71a#face:1` | 2 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Tendrils of Corruption / `84ee0da2-d886-4603-9aaa-bc5326a424d7#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Swamps you control |
| Terra Ravager / `c7686204-0433-48cf-bbfb-5d32b6a25cc3#card` | 6 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of lands defending player controls |
| Thalisse, Reverent Medium / `77cf4fb1-593f-4240-96d9-6ae9084baeac#card` | 32 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of tokens you created this turn |
| The Boulder, Ready to Rumble / `d33ee141-a880-4368-a274-a6ae2f2c624b#card` | 7 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creatures you control with power 4 or greater |
| The Final Days / `f2b88031-bfb7-46c4-abdd-3b6b5f5acfa4#card` | 8 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creature cards in your graveyard |
| The Legend of Kyoshi / `e41e8754-028c-4399-b530-de9e134d04b0#face:0` | 2376 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of cards in your hand |
| The Mouth of Sauron / `49e3396a-a125-4cca-b02a-4a96102bcb16#card` | 7 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of instant and sorcery cards in that player's graveyard |
| The Speed Demon / `c9fb22b4-dc22-4291-91f2-89c07cee7569#card` | 6 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: your speed |
| The Triumph of Anax / `eea6ba94-8755-45c0-94ea-cde68e4a1e7c#card` | 6 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of lore counters on this Saga |
| The Unbeatable Squirrel Girl / `80312910-5d53-44f1-9982-e46dc7532abb#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Squirrels you control |
| The Wandering Minstrel / `e6e6e3d2-f2fb-46a1-8798-13a2f2efe8cc#card` | 32 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Towns you control |
| Themberchaud / `a422a5b0-1aff-4d07-bcc5-edfbda09dc44#card` | 36 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Mountains you control |
| Thieving Sprite / `4c5b2be9-5d48-4e62-ae67-d61f3708bb4f#card` | 8 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Faeries you control |
| Thornmantle Striker / `39f17e13-018c-4a00-ab25-1ac5a904de63#card` | 8 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Elves you control; the number of Elves you control |
| Thoughtweft Imbuer / `9dee8c0b-71e0-4a94-b411-78b29b599847#card` | 6 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Kithkin you control |
| Thud / `5d2a9859-1353-4431-9343-f5999450acd1#card` | 5 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Thunder of Hooves / `4b505a4c-ccc3-4461-bbc4-ad5d39f9b879#card` | 3 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Beasts on the battlefield |
| Timberwatch Elf / `50cee3ac-cba0-4abb-babf-de1928b1590e#card` | 6 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Elves on the battlefield |
| Tip the Scales / `f7cac691-960f-4e36-8eae-f24bcdc1269b#card` | 3 | where supplement and sacrifice licence | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple; completed-Clause PP supplement with nominal specifying be clause; equated NP: the sacrificed creature's toughness |
| Tivash, Gloom Summoner / `b5641fcf-135e-43e7-a547-344fac61e279#card` | 32 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the amount of life you gained this turn |
| Tomoya the Revealer / `2b565791-1b94-4dad-a48d-be7fc2198e19#face:1` | 3 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of cards in your hand |
| Torch Song / `69826a0d-3ec9-47e7-b129-2fc13b41a07e#card` | 3 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of verse counters on this enchantment |
| Tormented Thoughts / `18aee20d-ab73-4fdb-a65c-101beae5fcf5#card` | 4 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Towering Titan / `a95309ac-f920-474d-81b5-0f68c8a54f34#card` | 140 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the total toughness of other creatures you control |
| Traverse the Outlands / `0d49cf51-af1f-4c17-9cf4-b82bc7c2c72e#card` | 80 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the greatest power among creatures you control |
| Treeguard Duo / `d7dfb4d5-8510-4904-abb7-834ec65d9bf3#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creatures you control |
| Tribal Flames / `c82eda61-d195-49c9-8ff0-a78dfa650689#card` | 28 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of basic land types among lands you control |
| Tumbleweed Rising / `ef574346-1d98-447d-acb6-4d8d53c5f651#card` | 5 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the greatest power among creatures you control |
| Tuya Bearclaw / `63e8ac70-0ec3-4bef-832f-e3f4c8a692ad#card` | 21 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the greatest power among other creatures you control |
| Tymaret Calls the Dead / `1d09ebb7-c4c9-4bb7-b6bf-3c8330b954e8#card` | 52 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Zombies you control |
| Tymna the Weaver / `d15642e4-e61c-4d29-af48-de837991245e#card` | 9 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of opponents that were dealt combat damage this turn |
| Tyvar, the Pummeler / `5a242eab-204c-4411-860f-c0f3582a8f38#card` | 40 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the greatest power among creatures you control |
| Undercity Upheaval / `c19f4a43-85bb-4070-a237-171597c94ec3#card` | 320 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creature cards in your graveyard as you cast this spell |
| Unquenchable Fury / `1c69a41e-122a-42c1-98d4-56035d68cfc8#card` | 18 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of cards in their hand |
| Urabrask's Anointer / `a09c134b-7802-4526-9250-ec5cc637d9c0#card` | 23 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of permanents you control with oil counters on them |
| Urabrask's Forge / `7c4b2203-a937-4d91-8e39-2283fe686b45#card` | 174 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of oil counters on this artifact |
| Urgent Necropsy / `378a6203-0d47-4165-bdc2-43dfdfb4d15d#card` | 5 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the total mana value of the permanents this spell targets |
| Valley Rotcaller / `8da0599a-8f63-4dba-857f-b4e10b7191b6#card` | 20 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of other Squirrels, Bats, Lizards, and Rats you control |
| Venerable Warsinger / `e286d1aa-bb74-4568-98b5-a693ac7a369d#card` | 120 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the amount of damage this creature dealt to that player |
| Vigorspore Wurm / `be843255-7408-45b9-b0ca-c79342daa353#card` | 160 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creature cards in your graveyard |
| Vile Deacon / `123147b4-57d0-44cd-bdd5-a449ac86c1cb#card` | 9 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Clerics on the battlefield |
| Vile Requiem / `a72213e7-16f0-4fbe-91d3-750ca51a4336#card` | 3 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of verse counters on this enchantment |
| Viridian Lorebearers / `080c0557-59ee-4c46-afd9-5baabf0dc3d3#card` | 4 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of artifacts your opponents control |
| Voda Sea Scavenger / `81a9da9d-c422-4293-b210-d89222633b4c#card` | 84 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of basic land types among lands you control |
| Voja, Jaws of the Conclave / `d94ff0b8-5499-4eeb-80f0-20c7fbf07499#card` | 8 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Elves you control |
| Voldaren Ambusher / `897f666e-0dda-4577-a3d7-792e3ac2f460#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Vampires you control |
| Wall of Limbs / `1aabb6bd-8061-42a5-9a71-f9eac718bd22#card` | 1 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: this creature's power |
| War Dance / `ecfc81df-1271-47ca-9184-49bf5798e195#card` | 6 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of verse counters on this enchantment |
| War Machine, Legacy of Iron / `ae6d1aa3-4ceb-4fc1-a3a6-033afd4b3bce#card` | 18 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: War Machine's power |
| Warfire Javelineer / `33a7a1e9-23b4-4cde-baf1-b1c0165c6729#card` | 4 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of instant and sorcery cards in your graveyard |
| Warped Physique / `f45fdef0-15b1-472b-9485-465c49a48a85#card` | 6 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of cards in your hand |
| Welcome the Dead / `affa0b6c-a9fd-49fc-b9ce-9a3486af8320#card` | 57 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of cards that were put into your graveyard from your hand or library this turn |
| Well of Lost Dreams / `b0394cf2-12a0-4d4f-87e0-fe8937e6faff#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: less than or equal to the amount of life you gained |
| Whiplash, Vengeful Engineer / `00ed7aca-b3c1-4d48-acd5-b1596daeb4c3#card` | 6 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Equipment attached to him |
| Wick's Patrol / `29423a9a-31a6-4605-8b49-a75895038a3a#card` | 21 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the greatest mana value among cards in your graveyard |
| Wick, the Whorled Mind / `5af729e8-8a40-4991-8270-612e377451de#card` | 15 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Wickersmith's Tools / `c89cd328-b4cc-4760-8b6c-59e82eed8dac#card` | 3 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of charge counters on this artifact |
| Wild Beastmaster / `52058c39-4edf-40f5-af18-e6de93f302a3#card` | 6 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: this creature's power |
| Wildwood Mentor / `f4e61b03-e6c8-4751-9720-b91fdecd57da#card` | 3 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: this creature's power |
| Wine of Blood and Iron / `22d85711-485b-47a2-b161-eaed135f1ddc#card` | 4 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: its power |
| Winged Temple of Orazca / `93b91d18-6acf-42e5-9a31-bc6e01f90c1f#face:1` | 3 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: its power |
| Wirewood Channeler / `8badfd34-9b63-4ada-8012-b8cddaf5b492#card` | 3 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Elves on the battlefield |
| Wirewood Pride / `ff19f10c-777c-4688-b1ab-99e53afaf629#card` | 6 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of Elves on the battlefield |
| Witch's Oven / `8fa0fe02-2452-4386-8e0c-165757b0f0a3#card` | 3 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Witch-king, Sky Scourge / `84137013-7a23-4bc8-b855-04889373fdae#card` | 6 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: their total power |
| Worthy Cause / `b24063f7-157f-47c0-919a-33856d53b902#card` | 2 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Wreath of Geists / `b11c31ac-9a52-4faa-b03e-5a78129fd43f#card` | 4 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of creature cards in your graveyard |
| Xathrid Demon / `dd2bbc90-0474-42b7-afe4-3655a120ab02#card` | 8 | sacrifice licence alone | sacrificed: existing ParticipialPremodifier > VerbalPremodifier, lexical sacrifice/PastParticiple |
| Yew Spirit / `0333cb81-5f44-4f31-9ee7-725e4f6af7b3#card` | 2 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: its power |
| Yuriko, Hope from the Shadows / `983f4fde-ab04-44f0-a294-c3b472de0b19#card` | 6 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of cards in your graveyard |
| Zedruu the Greathearted / `514179c0-a50b-4565-8e71-9dad256edd85#card` | 18 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of permanents you own that your opponents control |
| Zenith Flare / `ae005f00-3817-40ca-b8be-069a4085cfee#card` | 12 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: the number of cards with a cycling ability in your graveyard |
| Zoyowa's Justice / `17e11f0c-4ac7-4060-9e0b-27cd5c151e50#card` | 5 | where supplement alone | PP supplement of completed Clause/Keyword Phrase; nominal specifying be clause; equated NP: its mana value |


### Post-landing fix (2026-10-07)

The post-landing HIGH finding is resolved with a medial supplement. In Kraul
Harpooner, `MedialSupplementation` attaches the Where PP to the left Clause
anchor, and the comma plus sequencing coordinator introduces the continuing
Clause. Its equated NP is **the number of creature cards in your graveyard**,
with 3 independently admitted nominal Readings; the continuation is outside
that NP and outside Where's content clause. The historical table row above
is corrected. All 208 historical Kraul Readings had the wrong attachment and
are explicitly retired as wrong analyses, rather than counted as valid gains.

The declaration reads the existing supplementary licence and coordination
features; it names no lexeme in a licensing guard. The existing
`FiniteClauseComplement` feature gains the value `Nonsequencing`, assigned to
the existing Where lexeme. `finite_clause_complement` admits both values for
an ordinary finite complement. A coordinated complement exports the existing
`GeneralCoordination` feature through `ClauseCoordinationProperties` across
simple, serial and correlative routes. Its mutually exclusive forms keep
general coordination available under both licences and sequencing available
under the unrestricted licence. Thus the old wrong Where sequencing route
is retired on both the lexical and grammar sides, without leaving a dead
construction or labelling the same constituency twice.

This is the reviewer's preferred generic medial-position resolution, not a
blanket pruning of coordinated definitions. Three supported faces attest
coordinated definitions: Aspect of Wolf, Phyrexian Ingester and Bioplasm.
Twelve faces contain host sequencing after a definition; the named sources
are in `where-coordination-attestations.json`. Sardian Cliffstomper's two
Readings retain coordination under **As long as** in the anchor, with its
Where definition remaining a separate, final supplement. The restriction on
sequencing is a project analysis of these source boundaries, not a CGEL claim.
CGEL Ch. 15 §5.1, p. 1351 supplies the term **anchor**, and p. 1355 permits
interpolation among supplement positions; those passages alone are cited for
those claims. The Supplement glossary entry and the existing Supplementation
schema field now use `anchor`.

Routing is corrected in
[english-v3-systemic-residuals](../planned/english-v3-systemic-residuals.md):
Magma Sliver and the six ordinary variable-Subject gains are discharged;
the historical 601 unread variable-Where faces and the reported 322 NP
diagnostics remain routed there for cause audit. Ten of those NP diagnostics
incorrectly included a continuing Clause in the nominal probe, so the corrected
independent-NP blocker list has 312 faces. Five examples
name their constituent blockers, not merely their face names. Those diagnostic
counts are historical and overlapping, not sole-cause claims; the current
Where residue is measured below. The 149 sole-cause estimate on
`xxknlzypsnwy` / covered 17,322 and the later 512 feature-caused gains are
reconciled above by their distinct trees and cleared co-blockers.

Binding remains outside this grammar landing. “Target creature gets +X/+X,
where Y is its power.” has 1 Reading; “Target creature gets +1/+1 until end of
turn, where X is its power.” has 2. The ignored direct fragment probes retain
the exact trees. These admit grammatical syntax without proving a matching
variable occurrence in the anchor; a later variable-binding check is routed
to systemic residuals under `oracle-text-is-forward-anaphoric`.

The ten false NP-blocker diagnostics are Collective Voyage, Jaheira's Respite,
Descendant of Soramaro, Sword of the Ages, Sylvan Primordial, Information
Dealer, Ugin's Insight, Rampant Rejuvenator, Boundless Realms and Harvest
Season. Their equated NPs independently admit a root when stopped before the
outside continuation; they retain any other whole-face blocker. The correction
also separates X and Y's equated constituents on the three attested coordinated
definitions. Aspect of Wolf's half-number expression and the exiled-card possessive
expression on Phyrexian Ingester and Bioplasm still fail the independent NP
probe. Original NP
probes whose boundaries and grammar are unchanged are reused; all corrected
boundaries are freshly probed. Named evidence is `corrected-np-boundaries.json`
and `np-residual-reconciliation.json`, under the follow-up evidence directory.

Follow-up measurements in this section are stamped **`worzsvpnqyms` / covered
20,860**, with the exact grammar and lexical hashes in `measurement-stamps.json`.
The starting parent is `trlolklusowmn` / covered 20,858. Its declarations and
fully loaded baseline lexical inventory are byte-identical to the reused
`wzuorztqvmnl` / covered 20,858 full census; intervening semantic-body and
document changes do not change this baseline. `base-stamp.json` records that
proof. No extra baseline full run was spent.

| Comparison / stamped trees | Covered | Unique | Multiple | Unread | Exact Readings |
|---|---:|---:|---:|---:|---:|
| Reused base `wzuorztqvmnl` / 20,858, equivalent to `trlolklusowmn` | 20,858 | 7,477 | 13,381 | 11,970 | 651,577 |
| Finished follow-up `worzsvpnqyms` / 20,860 | 20,860 | 7,477 | 13,383 | 11,968 | 651,602 |

All 20,858 previously covered faces keep their exact Reading counts, including
Kraul Harpooner (208 → 208) and Sardian Cliffstomper (2 → 2). The count comparison
has **zero decreases, zero coverage losses, and no unnamed changed face**.
Kraul's 208 old wrong-attachment values are explicitly retired and replaced by
208 correct medial values; identical counts do not conceal that structural
replacement. Every Kraul Reading is checked for the same correct anchor,
supplement and continuing Clause. Byte-exact realization, lexical ownership,
construction and leaf traversal identity all pass over **651,602 Readings**;
there are zero duplicate Readings, cyclic derivations, materialization/internal
failures or validation issues. Readings remain preserved under the lexical
analysis contract, without destructive selection. V3 has no specificity-resolved
selection category or legacy coverage-lock/checker-total output. New word-named
licensing guards: **0**. Lexical loading and GrammarEnvironment construction
succeed; declared features supply every new guard.

Only two faces change count against the starting base, both valid new gains:

| Face / identity | Before → after | Judged analysis |
|---|---:|---|
| Sword of the Ages / `201f2434-96b3-408c-a5ce-74d0675920ed#card` | 0 → 20 | Activated ability retains its cost; the medial Where supplement anchors to **This artifact deals X damage to any target**. Equated NP **the total power of the creatures sacrificed this way** has 5 independent Readings: a definite power NP with *total*, an *of* phrase, and the creatures' participial postmodifier. **exile this artifact and those creature cards** is the outside imperative continuation. |
| Ugin's Insight / `ad7b9db0-e061-4360-9b81-0408d18d971d#card` | 0 → 5 | The medial Where supplement anchors to **Scry X**. Equated NP **the greatest mana value among permanents you control** has 5 independent Readings: a definite mana-value NP with the superlative and an *among* phrase containing the object-relative permanent NP. **draw three cards** is the outside imperative continuation. |

The ignored AST spot-check utility verifies all **233** Readings on these two
faces plus Kraul, not merely their samples; `medial-spotchecks.json` records the
exact constituents and independent NP counts. The final candidate selector
contains 52 faces with samples and matches the full census. No other original
gain has the wrong sequencing-inside-Where shape: the original exhaustive
coordinated-complement inventory has only Kraul and Sardian among Where faces,
and Sardian's coordination is inside its **As long as** anchor condition.

Against the original claim baseline `wlmqosnolmnx` / covered 20,280 (the
code-equivalent runtime `toxwntsklvow` snapshot), the corrected result is
**580 gained faces and 30,358 added Readings**, replacing the historical
578 / 30,333 claim. The partition is **508 Where-supplement-only faces /
29,352 Readings**, **6 ordinary variable-Subject faces / 641**, **54
sacrifice-licence-only faces / 333**, and **12 joint faces / 32**. Thus the
Where feature now accounts for **514** gains by itself, versus the historical
512 before this follow-up. The existing sacrifice licence is unchanged;
Sword's *sacrificed* is a postmodifier, so its gain comes from the medial
supplement. The refreshed `final-gain-analysis-ledger.json` contains all **580** named
identities with current sample fingerprints and judgments, including the
corrected Kraul analysis and these two new rows. Its counts match the full
census. Only Sardian retains a coordinated-clause PP on any gain; both
Readings put its coordination under As long as. `comparison.json` holds the
full per-face comparison and gain list. Of 1,119 variable-Where faces, **520** read
and **599** remain unread, including the corrected **312** independent-NP
blocker faces. All residue retains systemic routing.

Deviations and additions: **no feature, category, lexical frame or test is
removed**. Added feature name: **none**; added existing-feature value:
`FiniteClauseComplement::Nonsequencing`. Added table:
`finite_clause_complement`. Added policy: `ClauseCoordinationProperties`.
Added construction: `MedialSupplementation`. The three coordinated-clause
category summaries reuse the existing `GeneralCoordination` feature. The
field rename `Supplementation.host` → `Supplementation.anchor` follows the
requested terminology correction. There is no new ruling contradiction or
unresolved STOP; the existing sacrifice orchestrator resolution and all other
exclusions remain in force.

Nonblank declaration lines, claim parent versus finished follow-up:
**3,300 → 3,320; net +20**. Named construction/schema count **248 → 249**
(203 → 204 ordinary; 45 schemas unchanged); features **94**, categories **144**,
tables **73 → 74**, policies **61 → 62**. The inherited 2,800-line ceiling is
still exceeded; the change reuses feature names, lexical categories and
coordination licences. Named spellings **38,374** and homographs **1,127** are
unchanged; `spellings-final.json` matches the original inventory, and
`inventories-final.json` retains the complete named homograph and form-literal
lists and the empty form-literal/vocabulary overlap list. The new form uses
existing comma and space literals. No glossary gap remains after defining the
anchor in Supplement.

Assurance: **4 tests added, 7 existing tests re-spelled, 0 restored, 0 removed,
0 newly ignored**. Added tests are
`kraul_harpooner_keeps_sequencing_outside_the_medial_definition`,
`independently_composed_medial_supplement_retains_its_anchor_and_continuation`,
`sardian_cliffstomper_keeps_coordination_in_the_anchor_condition`, and
`coordinated_definitions_retain_general_coordination`. The five witness tests
(Chameleon Colossus, Wild Beastmaster, Hemosymbic Mite, Elenda, Tip the Scales)
and the two independent-keyword tests retain their exact outcomes through the
anchor field rename; all 9 original tests remain. The new independently
composed medial root uses separately parsed children, and proves exact value,
node and leaf traversal roundtrip; it is not described as a wholly handwritten
tree. The original fully independent keyword tree remains checked. The focused
suite passes **13 tests**.

Performance advisory, `worzsvpnqyms` / covered 20,860: **6 workers**, host load
(1/5/15 min) **38.934/26.580/22.078**, corpus wall **226,555,570,831 ns**, checked-text thread CPU
**343,261 ns/B**. The wall time exceeds the **16,260,000,000 ns** quiet-host
advisory ceiling. The census overlaps gate compilation on a loaded shared host;
these timings do not isolate the causal cost of the grammar change. This is an
advisory, not a gate. All source hashes, exact host load and CPU telemetry are
retained in `after.json`.

The finished-feature full command is `target/debug/cargo-xtask english-v3
--all --workers 6 --samples-per-face 0 --output
target/english-v3/english-v3-where-followup/after.json` (the built `cargo xtask`
CLI). This is **one** full run for the follow-up; selector subsets and fragment
probes supply iteration and spot-checks. All follow-up census/evidence files
are ignored under **`target/english-v3/english-v3-where-followup/`**.

Verification on `worzsvpnqyms` / covered 20,860: `cargo xtask gate --changed
--from trlolklusowmn --run` derives `cargo test -p deckmaste_lexical_source -p
deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask` and exits **0**:
**845 passed, 0 failed, 1 inherited ignored, 96 suites**. The sole inherited
ignore is `macros::templates::tests::macro_schema_census_count_matches_21`,
whose attribute says it cross-checks the live corpus against the census and
runs on demand. Clippy for those four packages, all targets, with `-D warnings`
passes. `cargo fmt -p deckmaste_english_v3 --check` passes. Citation checks
report **0 stale and 0 noncompliant**; the piped jj diff audit selects **0 CR
citation sites**, because this follow-up changes no CR citation. The final
finished-feature census is **`target/english-v3/english-v3-where-followup/after.json`**.


Refresh exits **0 without conflicts**. It incorporates trunk's dev-profile
optimization at **`vsskuolwykss` / covered 20,858**, documentation and citation-
lock additions. The refreshed base's grammar, core lexicon and complete loaded
lexical inventory are byte-identical to the reused baseline; `refresh-proof.json`
records that comparison. **Trunk-attributed Reading decreases: none.**
The optimized build is reverified rather than assigning its changed timings
to this feature.

The first post-refresh census is stamped **`knotusxvzkzm` / covered 20,860**:
**7,477 unique, 13,383 multiple, 11,968 unread, 651,602 Readings, 0 issues**.
Every one of the **32,828** per-face Reading counts matches the finished
feature, including Kraul's 208 and Sardian's 2. Against the refreshed base,
all previously covered faces keep their counts; only Sword of the Ages
(0 → 20) and Ugin's Insight (0 → 5) change. Thus the 580 / 30,358 original-
claim gain comparison, 599 Where residue, 312 corrected NP blockers, and all
named judgments remain unchanged. The final ledger records that its sample
fingerprints come from the identical grammar and lexical inputs before refresh.

First post-refresh performance advisory, **`knotusxvzkzm` / covered 20,860**: **6 workers**,
host load (1/5/15 min) **17.491/12.790/14.735**, corpus wall **244,607,644,935 ns**, checked-text
thread CPU **412,427 ns/B**. This run overlaps gate compilation too and exceeds
the **16,260,000,000 ns** quiet-host advisory ceiling. The profile and host load
differ; these numbers do not isolate the cost of this grammar change.

Exactly **two full corpus runs** were used in this follow-up: the finished
feature and this post-refresh verification. The final command uses the same
flags with `--output target/english-v3/english-v3-where-followup/after-refresh.json`.
**First post-refresh census JSON: `target/english-v3/english-v3-where-followup/after-refresh.json`.**
All evidence remains under the ignored follow-up directory, including the
exhaustive comparisons, corrected gain ledger, NP diagnostics, inventories,
source/hash proofs and gate logs.

The refreshed gate, `cargo xtask gate --changed --from vsskuolwykss --run`,
derives the same four-package Cargo test command and exits **0**: **845 passed,
0 failed, 1 inherited ignored, 96 suites**, stamped `knotusxvzkzm` / covered
20,860. Refreshed all-target clippy with `-D warnings` and formatting pass.
Refreshed citation checks report **0 stale and 0 noncompliant**, and the piped
diff audit selects **0 changed CR citation sites**. `gate-results-refresh.json`
records the outcome. That declaration comparison is **3,300 → 3,320,
net +20**, before the sibling landing incorporated below.

#### Final combined tree after the sibling landing

The next clean refresh incorporates **`rmrtuzwlllyw`**, the completed
`english-v3-become-adjectival-passive` landing, and subsequent ticket-only
changes. Both constructions and both lexicon changes are retained. The
refreshed parent is **`oyxnwpvmkuqt` / covered 21,070**, whose grammar and
loaded lexical inventory match the sibling's retained full census on
**`mtzyynuxpnkt` / covered 21,070**. `combined-scope-proof.json` proves exact
lexical inventory equivalence after reversing only this follow-up's Where
feature value. The declaration diff separately confirms that the sibling's
complemented adjective construction is retained. There were no conflicts.

The two-run full-census budget was already spent. To verify this further
refresh without a third `--all`, the fresh selector conservatively includes
**all 1,296 faces containing “where” in raw text, case-insensitively**. The
loaded inventory confirms Where is the only owner of the supplementary PP
licence and the changed finite-complement value. The medial construction
requires that licence. For every other finite-complement head, the ordinary
table preserves its licence and the two coordinated forms select exactly one
route by the existing general-coordination feature. The category summaries
preserve that feature without adding a derivation. Renaming `host` to `anchor`
changes the field name only. Thus every potentially affected face is freshly
enumerated; the **31,532 untouched faces** reuse the refreshed base's complete
validated census. Source hashes and identities match throughout.

**Final census JSON: `target/english-v3/english-v3-where-followup/final.json`.**
This is explicitly a **composed corpus census**, with fresh Where faces from
`combined-where.json` and untouched faces from the copied
`combined-base.json`; its `composition` object records both paths, hashes,
stamps, selector scope and reuse proof. Its full-run wall time is null because
no third full execution occurred. Per-face telemetry retains its measurement
provenance; mixed aggregate CPU is not presented as a current full-run timing.
`compose-census.py` rebuilds all corpus totals and residual groups. Exactly
**two `--all` executions** were used; all additional work uses selectors or
fragment probes, with **6 workers** for census selectors.

| Comparison / stamped trees | Covered | Unique | Multiple | Unread | Exact Readings |
|---|---:|---:|---:|---:|---:|
| Refreshed base `mtzyynuxpnkt` / 21,070, equivalent to `oyxnwpvmkuqt` | 21,070 | 7,464 | 13,606 | 11,758 | 685,662 |
| Combined follow-up `rvpkstryvlvy` / 21,072 | 21,072 | 7,464 | 13,608 | 11,756 | 685,687 |

Against this refreshed base, **only Sword of the Ages (0 → 20) and Ugin's
Insight (0 → 5)** change Reading count because of this follow-up. No previously
covered face decreases or loses coverage. Kraul remains 208 and Sardian remains
2; Kraul's old 208 wrong attachment values are retired and all 208 replacements
have the correct medial structure. The current AST spot-check independently
checks all **233** Readings across Kraul and the two gains, including equated
NP counts **3 / 5 / 5**, in `medial-spotchecks-combined.json`. The affected
selector has **29,713** validated Readings and zero validation, duplicate,
cycle or internal-failure issues. The composed census likewise has zero such
issues across **685,687** retained or freshly validated Readings. The
independently composed-value roundtrip law is separately covered by the tests.
`combined-comparison.json` lists every own and inherited count change.

**Trunk-attributed Reading decreases: none; `rmrtuzwlllyw` contributes 212
coverage gains and 389 increases on previously covered faces.** Its landing
record owns those analyses; they are not gains of this follow-up. Within the
original Where ticket's corrected 580 gains, the inherited increases are:

| Face | Before → combined Readings | Judgment / attribution |
|---|---:|---|
| Lydia Frye | 57 → 96 | Adjectival blocked analysis, including a selected By Complement; inherited By Adjunct defect remains routed below; `rmrtuzwlllyw` |
| Deekah, Fractal Theorist | 24 → 40 | Additional adjectival blocked analysis; `rmrtuzwlllyw` |
| Vigorspore Wurm | 160 → 288 | Adjectival blocked analysis, including a selected By Complement; inherited By Adjunct defect remains routed below; `rmrtuzwlllyw` |
| Cybermat | 3 → 6 | Additional adjectival blocked analysis; `rmrtuzwlllyw` |
| Glissa's Retriever | 95 → 160 | Adjectival blocked analysis, including a selected By Complement; inherited By Adjunct defect remains routed below; `rmrtuzwlllyw` |

Adjective versus verbal-passive readings have distinct categories under the
lexical analysis contract, rather than a second construction label for one
constituency. Their parser totals also include **inherited wrong By Adjunct
scopes**, which must not be described as grammatical: the sibling's docs-only
post-landing correction records the **orchestrator ruling (2026-10-07)**,
retracts that claim and routes passive By-attachment over-admission to systemic
residuals. Lydia, Vigorspore and Glissa contain this pattern; Deekah and Cybermat
do not. The sibling's landing record owns the affected inherited analyses and
their deferred measured exclusion. This qualification is a project ruling,
not a CGEL attribution. Neither of this follow-up's two newly covered faces
contains By, and their selected analyses above are unaffected by that defect.
All **580 original-ticket gains** are freshly censused again with samples in
`gains-combined.json` and judged in `final-gain-analysis-ledger-combined.json`.
Their current total is **30,609 Readings**: **508 Where-supplement-only faces /
29,603**, **6 ordinary variable-Subject faces / 641**, **54 sacrifice-licence-
only faces / 333**, and **12 joint faces / 32**. The historical corrected
**580 / 30,358** measurement above remains stamped to its pre-sibling tree;
the additional **251** Readings come from these five inherited increases.
The parser-coverage gain set and the **514 Where-feature-alone** attribution
are unchanged; the inherited By defect is disclosed rather than treated as a
new valid analysis of this follow-up.
Only Sardian has a coordinated-clause PP in the refreshed gain samples, under
As long as in its anchor; the affected Kraul trees are checked exhaustively.

Of the same **1,119 variable-Where faces**, **520** read and **599** remain
unread. The independent-NP diagnostic list is now **311**: the fresh probe of
*the power of that blocked creature* gains 1 NP Reading through trunk's new
blocked adjective, discharging that constituent on Glyph of Delusion while
the whole face still has zero Readings. Fresh probes of the five equated NP
strings mentioning the changed Block/Lose inventory record the inherited
effect in `np-combined.json`; unchanged constituent diagnostics reuse their
prior probes. `np-residual-reconciliation-combined.json` names the retained
311 faces and the inherited discharge. All whole-face residue retains its
systemic routing. The historical 601 / 322 counts remain explicitly historical.
The current binding probes still yield **1 / 2**, and the positive coordinated
definition probe still yields **1**; their trees are retained beside the census.

**Declaration lines, claim parent versus landed combined tree: 3,300 → 3,330;
net +30 = own +20 plus inherited +10 (`rmrtuzwlllyw`).** Own additions remain
exactly: feature names **none**, existing-feature value
`FiniteClauseComplement::Nonsequencing`, table `finite_clause_complement`,
policy `ClauseCoordinationProperties`, construction `MedialSupplementation`.
The sibling adds construction `ComplementedAdjective` and frame alias
`AdjectivalPrepositionComplement`; those are inherited additions. Combined
construction/schema count is **250** (205 ordinary, 45 schemas); feature,
category, table and policy counts are **94 / 144 / 74 / 62**. No superseded
declaration remains unreachable. The combined named spelling inventory stays
**38,374**; homographs become **1,128**, solely by inherited
*blocked*: `core-verb:Block` and `core-verb:Block/adjective`. Full named lists
are `inventories-combined.json` and `spellings-combined.json`; literal/vocabulary
overlap remains empty. Follow-up assurance counts remain **4 added, 7 re-spelled,
0 restored, 0 removed, 0 newly ignored**. No new glossary gap or unresolved STOP
arose; the sacrifice ruling and other exclusions remain unchanged.

Current selector performance advisory, **`rvpkstryvlvy` / covered 21,072**:
**6 workers**, host load **14.909 / 16.066 / 20.337**, Where-selector wall
**8,636,363,805 ns**, checked-text thread CPU **414,691 ns/B**. The fresh
580-gain selector has wall **7,989,906,070 ns**, host load
**15.581 / 15.946 / 20.000**, checked-text thread CPU **500,809 ns/B**, also
with 6 workers. These are subset measurements on a shared loaded host;
comparison with the **16,260,000,000 ns** quiet-host full-corpus ceiling is
not valid for subsets. The last full-run advisory remains the separately
stamped `knotusxvzkzm` / 20,860 measurement above.

Combined-tree verification, **`rvpkstryvlvy` / covered 21,072**: `cargo xtask
gate --changed --from oyxnwpvmkuqt --run` derives `cargo test -p
deckmaste_lexical_source -p deckmaste_construction_v3 -p deckmaste_english_v3
-p xtask` and exits **0**: **851 passed, 0 failed, 1 inherited ignored,
97 suites**. The inherited ignore and blocker are unchanged from above.
All 13 Where tests and the six sibling adjectival-passive tests pass. Clippy
for the same four packages, all targets with `-D warnings`, passes;
`cargo fmt --all --check` passes. Citation checks report **0 stale and 0
noncompliant**; the piped diff audit selects **0 changed CR citation sites**.
`gate-results-combined.json` records the check scope and outcomes.
