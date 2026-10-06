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
