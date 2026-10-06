---
needs: []
---
# Read "mana of any color" and "As an additional cost" phrases

## Why

Two short, highly repeated phrases never read:

- Add-mana Noun Phrases with an *of*/*in* tail: "Add one mana of any color.",
  "Add two mana of any one color.", "Add two mana in any combination of
  colors.", "an additional one mana of the chosen color". On change
  `wlvwtnppyovn` (32,828 supported faces, 13,716 covered, 19,112 unread) they
  occur in a failing unit of 701 unread faces and are the only recognised
  cause on 241. The exact failing unit "{T}: Add one mana of any color." occurs
  on 183 unread faces, and on 70 it is the face's only failing unit. Bare
  symbol strings already read: "{T}: Add {C}{C}.", "{T}: Add {R}{R}{R}." and
  "{T}: Add {G} or {W}." each have one admitted root, and are out of scope.
- The opener "As an additional cost to cast this spell, …": 317 unread faces
  touched, 158 sole. 308 unread faces have a failing unit beginning with that
  exact phrase. "As an additional cost to cast this spell, sacrifice a
  creature." has 0 admitted roots, while "Sacrifice a creature." has 1.

These are surface-bucket counts, not gain forecasts. (The minting brief quoted
131 and 58 for the two verbatim counts. The figures above were re-measured from
the same reconnaissance on the same change. Re-measure on the claim parent.)

## Goal

Both phrases read with structural analyses, and the mana NP is consumed by
Add's existing frame.

## Analysis

In *one mana of any color*, *of any color* is an NP-internal PP dependent of
the noun *mana*, the *a school of this type* pattern (CGEL, Ch. 5, §14.2,
p. 446, [14i]). The ADR licenses *of* only NP-internally, so this is the only
admissible attachment. *In any combination of colors* is a PP dependent of
the same Nominal. Check whether it is a Modifier or a Complement of *mana*
before declaring it. *As an additional cost to cast this spell* is a PP headed
by *as* with a predicative NP Complement, in Adjunct function, the *As
treasurer, I recommend …* pattern (CGEL, Ch. 7, §5.1, pp. 636–637, [4ii]). The
infinitival *to cast this spell* is NP-internal to *an additional cost*. The
glossary has no entry for a PP headed by *as* with a predicative Complement.
Add one with that citation if the implementation needs a term.

## Witnesses

- Glimmervoid: "{T}: Add one mana of any color."
- Implements of Sacrifice: "{1}, {T}, Sacrifice this artifact: Add two mana of
  any one color."
- Terrarion: "{2}, {T}, Sacrifice this artifact: Add two mana in any
  combination of colors."
- Altar's Reap: "As an additional cost to cast this spell, sacrifice a
  creature."
- Magmatic Insight: "As an additional cost to cast this spell, discard a land
  card."
- Lightning Axe: "As an additional cost to cast this spell, discard a card or
  pay {5}."

Each witness's only failing unit is the one quoted.

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. Before: `cargo xtask english-v3 --all --workers 12 --samples-per-face 0
   --output target/english-v3/fixed-cost-phrases-before.json` on the claim
   parent, stamped with its change id.
2. Write the witnesses as tests first.
3. Iterate on `--face-id` selectors; verify on `--all` at the end.
4. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
5. Spot-check every newly covered face's selected analysis. The landing record
   lists at least 10 by name with their Reading. A whole-phrase opaque leaf, or
   *of any color* attached to the clause, is a defect, not a gain.
6. No guard may name a lexeme, construction, verb, noun, preposition or card;
   read declared features only. Tests are re-spelled, never deleted. No new
   xtask subcommands, flags, fixtures or tooling.
7. May edit `grammar:` frame fields in `plugins_v2` keyword actions and lexicon
   frames in `crates/deckmaste_lexical_source/lexicon/*.ron`; plugin bodies
   untouched. Lexicon additions must be attested in the supported corpus
   (pruning rule).
8. Gate: `cargo xtask gate --changed --from <claim> --run`.

## Out of scope

- "Activate only …", "Cast this spell only during …", "only if" restrictions.
  No live ticket other than `english-v3-systemic-residuals` owns them (its
  "only-if/only-during" line). They stay there.
- Cost modification ("This spell costs {1} less to cast …"; Cost's
  `ManaAmount, ComparisonDirection, ControlledCostAction` frame).
- Spend restrictions on the produced mana ("Spend this mana only …").

## Landing record

### Result and PROVE

The six ticket witnesses read structurally. Add's existing Object NP frame
consumes the mana NP; the mana noun remains mass-only. The color/combination
PPs remain NP-internal. The cost opener is a preposed, comma-separated PP with
a predicative NP Complement; the noun's selected to-infinitival Complement
contains Cast's existing Object NP frame. Pay's CostSymbols frame supports
Lightning Axe's alternative payment. No opaque phrase leaf is introduced.

The final census and identity comparison below cover all supported faces.
Every counted Reading passes byte-exact realization,
lexical ownership, construction and leaf traversal identity, with zero
validation issues or internal failures. Independent complete-value witnesses
also preserve the parse(realize(value)) law. All licensed ambiguity is retained;
there is no destructive selection or specificity-resolved census in v3.

No new production guard names a word, construction, or card identity. Guards
read declared features and frames. Runtime lexical loading is exercised by the
corpus and gate. The active v3 command does not emit the legacy permitted
licensing-checker total; no such legacy number is claimed as v3 authority.

### STOP — comparative admission (2026-10-06, resolved by orchestrator)

At the STOP, this ticket was unfinished and did not integrate. The requested six witness
faces read, but corpus review found newly admitted wrong comparative Readings.
For Counterbore, the selected tree takes *with the same name* inside the
searched-for NP and then takes *as that spell* as a predicative PP Adjunct of
*Search*. The comparison between names is absent. Living Breakthrough has the
corresponding *same mana value as that spell* defect. These are defects, not
coverage gains. The ADR's “A newly admitted wrong reading is still a defect”
rule applies even if another Reading survives; preference cannot repair
admission. A construction/word/card guard is forbidden.

The claimant stopped before landing and requested a scope ruling. Orchestrator
resolution, 2026-10-06: “Scope is NOT extended to comparative licensing”;
“License it there only” refers to the preposed, clause-initial, comma-separated
predicative-as Adjunct. The wrong clause-final/VP Adjunct Reading must be
excluded by declared features; a correct comparative Reading beside it would
not repair Admission. The implementation therefore adds PreposedAdjunct to
Preposition Function Licence and uses the existing InitialPreposition consumer.
The predicative-NP PP projects only the declared PreposedAdjunct licence.
As retains its unrestricted Adjunct licence for the independent finite-clause
Complement use (for example, *as it attacks*); this is not a widening of
predicative-as. Other previously initial-licensed prepositions explicitly
retain PreposedAdjunct. No guard names a word, construction, or card.

- *same … as* comparison stays with its existing residuals owner. Its landing
  owns the non-scalar equality comparative *as* Complement licensed by *same*
  (CGEL, Ch. 13 §1.1, p. 1101), not an Adjunct licence.
- *Activate only as a sorcery* stays with the existing only-restriction item in
  residuals. That landing widens the position licence for clause-final
  predicative-as, measured.

Both deferred widenings are recorded in the “Batch records and what they leave
open” list of `english-v3-systemic-residuals`.

The affected candidate identities at the STOP (then selected with a
PredicativeComplementPreposition instead of the required comparison) and their
final Reading counts are:

| Face | Identity | Final Reading count |
| --- | --- | ---: |
| Counterbore | `1198ec53-9050-417d-812f-692e1378d346#card` | 0 |
| Sowing Salt | `131d2b70-959d-4814-b0c8-38031f141718#card` | 0 |
| Quash | `255131a2-616c-4c8d-a36a-14e5b5ab345e#card` | 0 |
| Harness the Storm | `38eaf0ea-8cdd-45f4-bfb9-0f8d0fcf863a#card` | 0 |
| Hint of Insanity | `4db59824-6bcb-4707-977b-0ff699d8662b#card` | 0 |
| Reflector Mage | `5398362d-c45e-4d56-a622-8be1124730b9#card` | 0 |
| Wake of Destruction | `562f7f99-ee67-4a14-b5e2-1a4ae710726d#card` | 0 |
| Dragonlord Kolaghan | `5ff89859-ddfd-48e5-8423-57b3956173ac#card` | 0 |
| Homing Lightning | `6e7f8e25-49a9-49a0-9bf4-129439344550#card` | 0 |
| Key to the Side-Door | `76c036df-44f5-4021-b7fc-656147351341#card` | 0 |
| Eradicate | `782ff740-ff43-4c07-af0b-433cb9770661#card` | 0 |
| Splinter | `90f01e9f-68f1-4e41-97dc-8de990a91504#card` | 0 |
| Maelstrom Pulse | `95ce305f-34bc-4d6d-b7ba-ffd4b2a25336#card` | 0 |
| Candles of Leng | `a12ac37d-02b8-4b6c-9595-74c42cfecb0a#card` | 0 |
| Reap Intellect | `aa97eb3d-8b8d-418b-8615-839812d60fe0#card` | 0 |
| Mishra, Artificer Prodigy | `b38adaac-3714-4348-9fec-cb2e2ad518b1#card` | 0 |
| Sever the Bloodline | `b9019332-2f02-48d9-be0a-a2b41b79e136#card` | 0 |
| Crumble to Dust | `c13da0f4-917f-4e07-b484-21cb4bec4da6#card` | 0 |
| Circu, Dimir Lobotomist | `c87d547b-00a4-4fdd-ade4-f3f032e5ea3b#card` | 0 |
| Echoing Calm | `d0bc217b-1149-4a33-a4a1-29e07597dc41#card` | 0 |
| Izzet Staticaster | `d63adbed-07d3-44e4-83f8-1cbdfc186df4#card` | 0 |
| Hour of Glory | `d97fd43b-146f-4d13-9c45-51bceaff742e#card` | 0 |
| Legions to Ashes | `dc29cb25-57f5-4cac-9764-efa644b41296#card` | 0 |
| Echoing Ruin | `e04ca2dc-873e-4d80-b1d7-4873ffdb3af5#card` | 0 |
| Living Breakthrough | `e711a213-2a6b-4b34-a788-95b6c0b7ba48#face:1` | 0 |
| Scour | `f18622da-81f3-4854-a1bb-8d658751d8db#card` | 0 |
| Shimian Specter | `f20f0bf4-c74b-4df7-a849-270c35f5d99e#card` | 0 |
| Mindreaver | `f606ab91-1f20-4351-acd3-6366b1895f72#card` | 0 |


### Repairs, assurance and deviations

The initial Mana-as-count attempt admitted *any number of mana*, violating the
retained negative oracle. It was withdrawn: Mana's original mass declaration
is unchanged. The broad cardinal-measure attempt then admitted *Draw two
damage*, violating another retained negative oracle. Cardinal Measurement Use
is now a separate declared lexical licence. Both tests remain unchanged and
must pass. These are repairs, not waivers.

Three attempt-only *the next X damage* parses used VariableCount under
CardinalMeasuredNominal rather than scalar measurement. Those wrong analyses
are retired; scalar measurement inside a determined Nominal remains re-coverage
work for `english-v3-systemic-residuals`. None was covered on the claim parent:
Panacea (`08ee7879-3c9d-43c6-a44e-f81180f938fd#card`), Alabaster Potion
(`2fcca686-03c8-4407-9f9f-2b7462470044#card`), Festival of the Guildpact
(`f8d1fbec-6ec1-47fa-ba02-fcee906db0d7#card`).

The first position repair also restricted finite-clause As and lost 22 baseline
faces. That attempt was not landed. The final repair preserves finite-clause
permissions and projects only PreposedAdjunct from the predicative-NP PP,
requiring that declared licence on its head. Lost attempt identities are named
in ignored `target/english-v3/fixed-cost-position-initial-losses.json`; the
final baseline comparison below is the landing authority. Deputy of Detention
and Banishment's pre-existing incorrect finite-clause comparative analyses are
already disclosed by the residuals owner; this ticket neither gains nor fixes
them. It excludes the newly admitted predicative-NP comparative analyses.

Deviations and additions:

- Five ordinary Constructions added, none deleted: InfinitiveComplementNominal
  distinguishes a noun-selected infinitival Complement;
  PredicativeComplementPreposition distinguishes a predicative NP from an
  Object; CardinalPremodifiedNominal distinguishes an internal cardinal under
  an outer Determiner (CGEL, Ch. 5 §7.6, p. 386); CardinalMeasuredNounPhrase and
  CardinalMeasuredNominal preserve the mass lexical head and singular quantity
  conceptualization, including a quantity under an outer Determiner (CGEL,
  Ch. 5 §3.4, p. 354). No shared schema or Category is added or deleted.
- Determiner Requirement propagates through nominal modification/coordination
  and blocks a duplicate bare count-NP analysis. MeasurePosition gains None
  and propagates through Nominals. Cardinal Measurement Use is independent of
  scalar measurement and admits cardinal quantification, including VariableCount,
  for the declared mass measurement use.
- Combination is added with singular/plural forms, attested on 48 previously
  unread supported faces. Cost declares bare and to-infinitival nominal frames;
  the noun's bare use remains available. Pay gains CostSymbols for the explicit
  Lightning Axe witness; existing Pay frames and all Add frames remain.
- Preposition Function Licence gains PreposedAdjunct, as the orchestrator ruled.
  Existing initial permissions are made explicit for previously Adjunct-licensed
  prepositions. InitialPreposition consumes that licence; the predicative-NP PP
  projects only it, while finite-clause permissions remain independent. This
  repair adds no Construction and licenses no comparative or only-restriction
  syntax.
- Ten fixed-cost tests added (nine original witnesses/mutations/independent-value
  tests and the position regression); one existing Noctis fixture re-spelled to
  BareFramedNoun for Cost's bare frame. The same card, source, complete value
  comparisons, and construction/word traversal assertions remain. Restored 0;
  re-spelled 1; ignored 0; added 10; removed 0. Each main witness and the
  clause-final position defect was observed red before its repair. No test was
  weakened, deleted or newly ignored.
- Glossary gaps filled: Predicative Complement of a Preposition, Predicand,
  Determiner Requirement, Measured Noun Phrase, Measured Nominal, Cardinal
  Measurement Use. Preposition Function Licence describes position restriction.
  Entries use CGEL citations; no Comprehensive Rules citation changes.
- The residuals record gains the two expressly requested dated deferral bullets.
  No new xtask command, flag, fixture, or tracked evidence/tooling was added.

### Verification and REPORT

Baseline authority is claim parent `lxztntxo`, with grammar unchanged in the
baseline report stamped `qunnvtrz`, covered 14,425. It examined 32,828 supported
faces: 18,403 unread, 6,528 unique, 7,897 multiple; 204,973 exact checked
Readings, zero validation issues/internal failures. The older ticket figures
are superseded by this claim-parent measurement. Baseline performance advisory:
36,224,222,881 ns corpus wall; 12 workers; host load
[6.4990234375, 6.90625, 5.171875]; 207,696 ns/B checked-text thread CPU.

The derived gate is invoked as
`cargo xtask gate --changed --from lxztntxouosvzuymsloqmrtsrqrkorou --run`:

```sh
cargo test -p deckmaste_lexical_source -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask
```

Final results, gains, and inventories follow. All census, analysis and log
evidence is under ignored `target/english-v3/`; earlier logs were copied there.
The active v3 corpus command has no legacy coverage lock. The unchanged tracked
legacy lock supplies no v3 admission authority. Measurements identify the
change and its corpus covered count; they are provenance, not fitted-to gates.


#### Final census (`qunnvtrz`, covered 15,167)

The complete `--all` run examined 32,828 supported faces: 17,661 unread,
6,772 unique, 8,395 multiple, 0 undetermined. All 219,518 counted Readings
are exact; validation issues, internal failures, duplicates and cycles are
zero. Enumeration is complete on all faces, with no limit or failed face.
The first retained-analysis gain selector has 742 faces: 248 unique and 494
multiple, zero issues. The final comparison against `lxztntxo` has **zero lost
covered identities and zero decreased baseline Reading counts**. Every one of
the 28 STOP-table identities has **0 Readings**, so none retains the wrong
predicative Adjunct. None is credited as a gain.

All 742 newly covered identities are listed with their first retained analysis
and Reading count in the ignored
[complete gain inventory](../../../target/english-v3/fixed-cost-phrases/fixed-cost-final-gain-inventory.md).
Each face is mapped to the distinct contextual subtrees reviewed by family:
157 subtrees across measured quantities, internal cardinals, initial predicative
cost PPs, noun-selected infinitivals and Pay/CostSymbols, plus Wizard's Rockets'
lexical Combination gain under its existing scalar measurement. Final sampled
root fingerprints are unchanged from the reviewed, surviving candidate trees.
This review retains licensed alternatives; it does not equate a singleton or
first Reading with correctness.

First retained-analysis families: P 293; M 195; C 134; N 70; M+N 17; C+P 27;
N+P 1; M+P 4; L 1. The letters resolve to named Constructions and structural
roles in the inventory, not semantic or card-name admission guards.

The six other attempt-only, clause-final predicative-NP identities remain
unlicensed by the orchestrator's position ruling. Their re-coverage belongs to
`english-v3-systemic-residuals`' measured position widening; they were unread on
the claim parent and are not final gains:

- Enthralling Hold: `2f94cb83-1f48-4293-b219-67830c074aeb#card`.
- Mosswood Dreadknight: `5783facf-d26e-4aae-a753-b9dc553361c1#face:0`.
- Hildibrand Manderville: `6df03db9-e4b3-4931-94e3-c582d2b04110#face:0`.
- Essence of the Wild: `9d105f83-583d-438b-8f26-b39e818ed295#card`.
- Dream Leash: `b7fde102-7b89-48c7-818c-bdf0eb33c2ee#card`.
- Teferi's Time Twist: `f5479d4f-bc4b-4452-a944-41be623f8367#card`.

#### Named whole-face spot checks (`qunnvtrz`, covered 15,167)

| Face | Readings | Retained analysis checked |
| --- | ---: | --- |
| Glimmervoid | 1 | Add's existing Object NP frame consumes CardinalMeasuredNounPhrase; Of/Any Color modifies the mass Mana Nominal. |
| Implements of Sacrifice | 1 | CardinalMeasuredNounPhrase with internal Of PP; Any One Color uses CardinalPremodifiedNominal below Any. |
| Terrarion | 12 | Measured quantity and structural Combination/Colors PP; the test independently pins the NP-internal In analysis. Existing In Adjunct alternatives remain. |
| Altar's Reap | 1 | InitialPreposition → PredicativeComplementPreposition → predicative indefinite NP → InfinitiveComplementNominal → Cast/This Spell. |
| Magmatic Insight | 1 | Same preposed cost PP; Discard retains Land as a noun Premodifier of Card. |
| Lightning Axe | 3 | Same initial cost PP; coordinated Discard/Card and Pay/CostSymbols `{5}` action alternatives. |
| Sol Grail | 1 | CardinalMeasuredNounPhrase with Of/The Chosen Color; Chosen remains a verbal Premodifier. |
| Utopia Sprawl | 9 | Additional One Mana uses CardinalMeasuredNominal below an indefinite NP, retaining Of/The Chosen Color. |
| Fertile Ground | 9 | Additional One Mana uses CardinalMeasuredNominal below an indefinite NP, with internal Of/Any Color. |
| Market Festival | 36 | Additional Two Mana is a singular measured entity under an indefinite NP; Combination and Colors remain structural NPs/PPs. |
| Labyrinth Adversary | 3 | Pay frame 2 consumes CostSymbols `{1}{R}` under the existing May predicate. |
| Strategic Planning | 3 | The Top Three Cards uses CardinalPremodifiedNominal under The; the existing split-destination Put frame remains. |
| Wizard's Rockets | 18 | Existing scalar MeasuredNounPhrase X/Mana; newly declared Combination fills the lexical gap in its In PP. NP-internal and existing Adjunct alternatives remain. |

These are full-face counts, including other ability lines. All six ticket
witnesses retain at least one Reading. Tests pin exact phrase constituents and
independently authored full values; the table does not replace those assertions.

#### Final provenance and performance

The named grammar inventory is 186 ordinary Constructions plus 44 shared
schemas = 230 constructor names, 135 Categories (`qunnvtrz`, covered 15,167).
This is five ordinary Constructions above the claim parent, with no deletion.
The active v3 corpus has no coverage lock and emits no legacy licensing-checker
total; the unchanged legacy lock is not a v3 gate. Source review finds zero new
word/construction/card-naming production guards, and corpus lexical loading
reports no error.

Performance advisory on `qunnvtrz`, covered 15,167: 52,753,385,803 ns corpus
wall; 6,082,997,663 ns setup; 12 workers; host load
[3.76416015625, 6.04296875, 5.0546875]; 226,581 ns/B checked-text thread CPU.
Total Cargo command wall is 357,941,584,202 ns including lock waits and
recompilation. Corpus wall exceeds the 16,260,000,000 ns quiet-host advisory
ceiling; the occupied-host measurement is disclosed, not fitted to or used as
a grammar gate.


Lexical inventory on `qunnvtrz`, covered 15,167: 121 inherited ambiguous
surfaces, comprising 119 surfaces with distinct lexical owners and the two
same-owner agreement ambiguities `'d` and `’d`. No ambiguous surface is added;
Combination's singular/plural surfaces and Mana each have one owner and one
alternative in the fresh probe. Full owners are in ignored
`fixed-cost-final-lexical-inventory.json` and its probe. Form-literal/vocabulary
overlap inventory: **[]**, count 0, before and after; new forms add only
separators, not alphabetic literals. These are provenance inventories, not
admission filters.

Named ambiguous surfaces: `'d`, `'s`, `Adamant`, `Addendum`, `Alliance`, `Battalion`, `Bloodrush`, `Celebration`, `Channel`, `Chroma`, `Cohort`, `Constellation`, `Converge`, `Council's dilemma`, `Coven`, `Delirium`, `Descend 4`, `Descend 8`, `Disappear`, `Domain`, `Eerie`, `Eminence`, `Enrage`, `Fateful hour`, `Fathomless descent`, `Ferocious`, `Flurry`, `Formidable`, `Grandeur`, `Hellbent`, `Heroic`, `I`, `Imprint`, `Infusion`, `Inspired`, `Join forces`, `Kinship`, `Landfall`, `Lieutenant`, `Magecraft`, `Metalcraft`, `Morbid`, `More Than Meets the Eye`, `Opus`, `Pack tactics`, `Paradox`, `Parley`, `Radiance`, `Raid`, `Rally`, `Renew`, `Repartee`, `Revolt`, `Secret council`, `Spell mastery`, `Strive`, `Survival`, `Sweep`, `Tempting offer`, `Threshold`, `Undergrowth`, `Valiant`, `Vivid`, `Void`, `Will of the council`, `control`, `copies`, `copy`, `cost`, `costs`, `counter`, `counters`, `cycling`, `deathtouch`, `decayed`, `die`, `double strike`, `draw`, `draws`, `exalted`, `exile`, `exiles`, `first strike`, `flying`, `goaded`, `harnessed`, `haste`, `her`, `hexproof`, `his`, `if`, `indestructible`, `instead`, `it`, `less`, `lifelink`, `menace`, `name`, `names`, `one`, `reach`, `shadow`, `solved`, `suspected`, `tapped`, `target`, `targets`, `that`, `time`, `to`, `trample`, `turn`, `turns`, `untap`, `untapped`, `up`, `vigilance`, `you`, `’d`, `’s`, `∞`.


Final derived gate: **exit 0**, 708 tests passed, 0 failed, 1 inherited
ignored test (`macro_schema_census_count_matches_21`: “cross-checks the live
corpus against the census; run on demand”). No new ignore is introduced.
The exact Cargo test command is recorded above. Targeted Clippy with
`--all-targets -- -D warnings` passes. Stable workspace formatting and nightly
formatting of all three changed compiled Rust files pass; the existing
complement fixture received import/comment formatting only alongside its
re-spelling. Citation checks from the workspace root report 16,059 checked,
**0 stale**, and **0 non-compliant citation-looking strings**. No CR citation
changed, so no bless/audit registration is needed. There is no unresolved STOP.

The complete gain inventory, corpus reports, identity comparison, reviewed
subtrees, lexical inventory and verification logs are retained as an ignored
evidence bundle under `target/english-v3/fixed-cost-phrases/` in the default
workspace when this feature is retired. No census/evidence file enters history.


### Post-landing fix (2026-10-06)

Review found overlapping declared Pay frames: ManaPhrase and CostSymbols both
admitted colored symbols with identical constituency. Their category
labels did not express distinct grammatical Readings. The original landing
record did not disclose this duplication. This follow-up retires Pay's
ManaPhrase frame; Pay keeps its NounPhrase and CostSymbols frames (the latter
now has frame index 1), and Add keeps its ManaPhrase frame. No Construction, feature value or production guard is
added. CostSymbols covers every previously covered attested Pay object: the
1,790-face Pay subset retains all 527 covered faces, with zero issues.

The glossary now attributes the predicative-*as* Adjunct construction to CGEL
Ch. 7 §5.1, p. 637, and the preposed-only position licence to the orchestrator
resolution dated 2026-10-06 above. The residuals batch list adds the relative
clause on *color*/*type* inside the add-mana Nominal: 31 supported faces, zero
covered, measured in the baseline census. No relative-clause licence is widened.

Before: trunk `lpzxsvzm`, covered 15,167; its grammar and per-face Reading
counts exactly match the preserved census on `qunnvtrz`, covered 15,167. A fresh
baseline census on `zxlxvukr` before frame retirement independently reproduces
those counts. After: `zxlxvukr`, covered 15,167, following frame retirement.
The two fresh snapshots carry distinct lexical inventory hashes:
`28f1b80bfdacc66b818eaa7c3004def7d31b0b7d95d1e4f721f611bc5eb78f33` before and
`13aa0820f27162671f2e849fd35bad94970b5b3a628bc979ebd60c7b7299470c` after.
All following corpus figures use those stated trees and the
same 32,828 supported identities and input hash.

| Census | Before | After |
| --- | ---: | ---: |
| Covered faces | 15,167 | 15,167 |
| No Reading | 17,661 | 17,661 |
| One Reading | 6,772 | 6,776 |
| Multiple Readings | 8,395 | 8,391 |
| Complete, byte-exact Readings | 219,518 | 217,223 |
| Validation issues / internal failures | 0 / 0 | 0 / 0 |
| Lost faces / gained faces | — | 0 / 0 |

The 74 previously covered doubled faces below all return to their original
`lxztntxo` counts (covered 14,425 on that older tree): 4,584 → 2,292 Readings.
The only other count decrease is Feed the Cycle,
`c338eedb-9b70-4b77-b5d6-8ea74b7bea3e#card`: 6 → 3. All 75 decreases retire
spurious frame alternatives; no face loses coverage. Complete enumeration,
byte-exact realization, lexical ownership and construction/leaf traversal
checks have zero issues, duplicate values, cyclic derivations or internal
failures. Independently authored complete values retain both roundtrip laws.
The standalone Shrouded Lore sentence "You may pay {B}." has exactly one
Reading; the pre-fix probe and regression test both showed two.

Review-count discrepancy: the stated 293 newly gained doubled faces are not
reproduced against `lpzxsvzm`. The original first-analysis P-only gain family
contains 293 identities, all of whose counts remain unchanged. For example,
Labyrinth Adversary's `{1}{R}` object uses only CostSymbols (3 → 3), because
ManaPhrase does not admit its numeral. Of all 742 original gains, only Feed the
Cycle (in C+P) loses duplicate Readings here (6 → 3). Every original gain remains
covered. `P-293-counts.json` and `comparison.json` record the complete lists;
these measured results supersede the review's gain-duplication estimate.

| Face identity | Name | Before | After (original count restored) |
| --- | --- | ---: | ---: |
| `f4490b09-dc55-47ed-a573-4ae0b95725b1#card` | Bloodfeather Phoenix | 120 | 60 |
| `ddc83b35-cf7c-495b-bd30-e409b622e299#card` | Breeding Pit | 6 | 3 |
| `8785f42f-87b9-4a0a-89ce-ea423649ba9c#card` | Chain Lightning | 28 | 14 |
| `d4e8aa93-c0d1-49b7-bbf4-35a155048774#card` | Child of Gaea | 6 | 3 |
| `7c91ae5d-0320-46a7-98d2-df0918202478#card` | Chromium | 12 | 6 |
| `a24e05fb-dffb-4400-b4ca-22fdde45e7a7#card` | Conversion | 12 | 6 |
| `c64c0cb5-5a8c-4a2f-a0fc-8b080aaf90bb#card` | Darba | 6 | 3 |
| `2847c8a0-f6aa-4e4a-a7b8-fc116436a264#card` | Demonic Hordes | 16 | 8 |
| `6288865d-7f64-411f-a044-ca350cf6fb8d#card` | Dragon Tyrant | 12 | 6 |
| `b6d39905-e074-4517-b806-d081d1069f74#card` | Drainpipe Vermin | 2 | 1 |
| `dbe8ad46-d862-4ff6-bf17-238fb8341075#card` | Drifter il-Dal | 6 | 3 |
| `2c04371a-af69-4e42-8cda-096cf8312b5e#card` | Edgar's Awakening | 50 | 25 |
| `36af3c2c-49d7-46ea-ab02-c254b332448e#face:0` | Eirdu, Carrier of Dawn | 32 | 16 |
| `9f679093-86a0-4211-8491-de1f6d81c483#card` | Flight Spellbomb | 12 | 6 |
| `e3c4c27d-f263-4c69-a4fe-2928136ff68b#card` | Force of Nature | 6 | 3 |
| `6ac470a1-c2be-4971-a5ca-10bb189ebe4d#card` | Frenzied Goblin | 6 | 3 |
| `e0d638d2-2c5b-415c-95e7-74bf8d88794f#card` | Freyalise's Charm | 4 | 2 |
| `0eafe734-19f2-492d-bfc2-d0c75ece45b2#card` | Ghastly Remains | 12 | 6 |
| `a4a9c2bb-e6d0-4665-8b2f-e90c78784303#card` | Glaciers | 12 | 6 |
| `50bebbb9-01b7-4eb6-8efd-307c9bcd2517#card` | Goblin Vandal | 4 | 2 |
| `18dcf3db-d6c6-4780-a931-546e71a3ae3f#card` | Gravity Negator | 6 | 3 |
| `2a174adc-0536-4e12-a6fe-c44305822714#card` | Haazda Snare Squad | 2 | 1 |
| `dedfbd92-4f12-423c-9d56-167753f03bff#card` | Hungry Mist | 6 | 3 |
| `bb217f12-532f-4833-a27a-99e290aa47d0#card` | Island Fish Jasconius | 36 | 18 |
| `afda663e-c5f7-4182-86f7-d95d71793717#card` | Junún Efreet | 6 | 3 |
| `77e319da-d782-4258-bf0e-626755e5adc9#card` | Kalastria Highborn | 4 | 2 |
| `e2770ebf-24e3-4787-956b-ca01aaa50a6f#card` | Kami of the Tended Garden | 6 | 3 |
| `fe2e27a9-437e-4d22-aa22-982978793cff#card` | Kels, Fight Fixer | 4 | 2 |
| `e9893534-3d05-42c2-93ff-f28ffae89aa3#card` | Knight of the Mists | 4 | 2 |
| `91b2520e-85b6-4e1f-88cf-a585feeb8e65#card` | Krenko, Baron of Tin Street | 12 | 6 |
| `a0ff742b-f709-43ac-84d2-dc00c723bad6#card` | Krosan Cloudscraper | 6 | 3 |
| `c80a7d22-076a-4fd7-bcf4-4769143647f0#card` | Kuro, Pitlord | 12 | 6 |
| `ef693b9f-11e3-49bf-8387-b8f480b9007a#card` | Leaf-Crowned Visionary | 8 | 4 |
| `98388eba-31ab-4302-be9e-ea67ea2b8c1a#card` | Leshrac's Sigil | 56 | 28 |
| `5f2a3797-28aa-4c7a-ba2b-fd243a1747fd#card` | Lifecrafter's Bestiary | 4 | 2 |
| `4a574140-0657-4f49-b497-331447f17b29#card` | Lightning Cloud | 2 | 1 |
| `ea97fc76-3a32-414c-aa12-87be1a0eb9f3#card` | Masked Admirers | 12 | 6 |
| `58cc7682-f8ce-47cf-a291-0301fea9ddfd#card` | Melancholy | 36 | 18 |
| `93fbf4a7-4283-4eec-afde-6780149b56df#card` | Minion of Tevesh Szat | 12 | 6 |
| `af029853-cfb0-403b-af51-141ba02ae2e4#card` | Mtenda Lion | 116 | 58 |
| `5a647f28-6e26-4262-8593-d582e0a691f7#card` | Nature's Wrath | 54 | 27 |
| `6f16c3ac-a9b8-47e6-b18b-ae37c74d44a0#card` | Nether Traitor | 18 | 9 |
| `e26bf6a9-b31c-4bc0-b55b-c01f2f69be6b#card` | Nicol Bolas | 588 | 294 |
| `b713e49f-1b13-42d1-91f2-cc7a579e7614#card` | Nihil Spellbomb | 6 | 3 |
| `02abe85a-c603-4963-aba4-524cc80847ea#card` | Order of the Golden Cricket | 6 | 3 |
| `3de0e14f-c917-4c00-8341-1170ac9b3cbc#card` | Origin Spellbomb | 6 | 3 |
| `d7121ac7-e425-46cd-b006-1af391d97f87#card` | Palladia-Mors | 12 | 6 |
| `4429a30e-17f1-4bcc-92f0-55ab1315adff#card` | Panic Spellbomb | 18 | 9 |
| `e9832d58-0ee6-4641-a9fd-d34d54b3b11c#card` | Passenger Ferry | 6 | 3 |
| `06a158c6-7e36-49f8-a8e0-a7b7df5fd7ed#card` | Phantasmal Forces | 6 | 3 |
| `dcd13eab-7ed7-4e95-bdae-d140c1ff84df#card` | Piru, the Volatile | 6 | 3 |
| `f536acf3-ff8e-48da-8509-00e7ce4a1678#card` | Pit Spawn | 12 | 6 |
| `1890ef1b-38e5-432d-b040-f33cea7814e7#card` | Punishing Fire | 6 | 3 |
| `42c7e933-f169-444b-917d-b4dca918d989#card` | Puppet Master | 16 | 8 |
| `559463f8-396f-40e0-9515-a1be6397ca31#card` | Serene Steward | 2 | 1 |
| `d3f4aa87-a89e-43f8-abc3-712e6bf08889#card` | Shambling Cie'th | 12 | 6 |
| `7b22f31c-9caa-4afd-8c44-f88346bfa79c#card` | Shu Yun, the Silent Tempest | 12 | 6 |
| `53599f89-1598-4d3d-8aa7-75b397b258eb#card` | Spindrift Drake | 6 | 3 |
| `ae896d87-59c7-4d8d-a137-9e471d3e174e#card` | Spirit Cairn | 16 | 8 |
| `f63278e0-3c67-421d-87ce-67725e5e74df#card` | Spit Flame | 6 | 3 |
| `a8cf1379-0195-4e11-b994-481ef1284245#card` | Stasis | 12 | 6 |
| `23985249-c54e-4bfa-b69a-512422a46a77#card` | String of Disappearances | 28 | 14 |
| `17841133-c526-42a3-a1d2-a874e9bb7138#card` | Sunken City | 6 | 3 |
| `3d3eb043-8ce6-461a-b8bc-67f0ba6cd580#card` | Taeko, the Patient Avalanche | 24 | 12 |
| `ff4ccf8d-7d3d-4a44-a9ab-6bc88e90d6a7#card` | Thelon's Chant | 18 | 9 |
| `f6880f71-a05a-4752-ba33-ffbc450606bf#card` | Thelon's Curse | 1,920 | 960 |
| `61b3ad9d-485f-4479-b18f-506847af43f2#card` | Thirst | 36 | 18 |
| `bdf4e225-23b5-47d9-9936-ddcec8aa1ca4#card` | Tourach's Chant | 36 | 18 |
| `95ee46ec-4b60-4fea-a09b-d15c083916b3#card` | Unconventional Tactics | 18 | 9 |
| `4d2cc2a8-e08a-420a-8922-c63e39129e23#card` | Vaevictis Asmadi | 768 | 384 |
| `ab478ac9-af59-4df1-afef-5e9806c06643#card` | Veinwitch Coven | 10 | 5 |
| `c498f0ff-812a-4106-834b-f43a0a7e0bac#card` | Whipstitched Zombie | 6 | 3 |
| `d5277fe0-b707-41bb-9ff9-62f8be249dd2#card` | Wild Leotau | 6 | 3 |
| `adc47a25-daab-49c7-93a5-5bbf2bc20cf9#card` | Wolfbat | 168 | 84 |


Performance advisory (`zxlxvukr`, covered 15,167, before/after snapshots):
12 workers in both runs. Before: 47,613,471,720 ns corpus wall,
7,615,346,113 ns setup, 225,926 ns/B checked-text thread CPU, host load
[5.60986328125, 6.982421875, 7.6904296875]; complete Cargo command
214,367,247,633 ns including the build-lock wait. After: 68,865,652,661 ns corpus
wall, 7,785,738,794 ns setup, 294,268 ns/B checked-text thread CPU, host load
[6.87646484375, 7.5400390625, 7.85595703125]; complete Cargo command
78,136,903,031 ns. The after census ran alongside gate compilation. Both exceed
the 16,260,000,000 ns quiet-host advisory ceiling; neither timing is a gate.

Assurance counts: restored tests 0; re-spelled tests 1
(`independent_finite_and_secondary_complements_use_declared_frame`, same Add/Pay
subjects, exact realization and complete-value parse-back assertions retained);
added tests 1 (`colored_payment_objects_have_one_reading`, Shrouded Lore and
Sunken City); removed tests 0; new ignores 0. The re-spelled test also rejects
independently constructed Pay/ManaPhrase values through admission and
realization. Targeted mana tests: 4 passed, zero failed or ignored. The singleton
regression was observed failing before the frame retirement (2 versus 1).

Deviations and additions: no new grammatical Construction or feature value;
retired one redundant declared Pay frame as requested. One regression test
added and one existing test re-spelled, with unchanged subjects and roundtrip
outcomes. No glossary gap, new lexical owner, form literal, homograph or
licensing checker is introduced. The named grammar inventory remains 186
ordinary Constructions plus 44 schemas = 230 constructor names, 135 Categories.
No word/construction/card-naming production guard is introduced, and lexical
loading succeeds in both corpus runs. No STOP is unresolved.

Verification on `zxlxvukr`, covered 15,167: derived gate
`cargo xtask gate --changed --from lpzxsvzm --run` **exit 0**. The derived Cargo
command is `cargo test -p deckmaste_lexical_source -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask`:
**709 passed, zero failed, one inherited ignore**
(`macro_schema_census_count_matches_21`: "cross-checks the live corpus against
the census; run on demand"). Complete gate command wall: 1,180,644,716,663 ns.
No additional test is ignored.

Clippy over those same four packages, `--all-targets -- -D warnings`, exits 0
(command wall 542,698,519,794 ns including its build-lock wait).
`cargo fmt --all -- --check` and nightly formatting of the changed Rust test
file both exit 0. From this workspace root, `cargo xtask cite check` reports
16,059 checked citations, **0 stale**; `cargo xtask cite check --list-noncompliant`
reports **0 non-compliant citation-looking strings**. No CR citation changed,
so no new citation registration is required.
All evidence is ignored under `target/english-v3/fixed-cost-followup/`; no
census/evidence file is tracked. The bundle is retained in default on retirement.
