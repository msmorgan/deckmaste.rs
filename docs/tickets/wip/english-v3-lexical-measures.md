---
needs: [english-v3-whole-grammar-activation, scryfall-oracle-card-ingestion]
---
# Repair declared lexical gaps and scalar notation

First residual batch: repair the six observed word gaps below through v3-owned
declarations in `crates/deckmaste_lexical_source/lexicon/core.ron` and its source
adapter, then add compositional power/toughness slash pairs to v3 measures.
Do not broaden this into the entire unknown-word inventory. The activation
report at `kkmxslkn` found 8,468 failed faces with unknown words and 10,803
with slash notation; these overlapping groups are not promised coverage gains.

Pinned shape: independently declared Lexemes, applicable Word Forms, count/mass
uses and selected frames; a slash pair retains two ordered scalar components
and each component's sign/notation. Consume the pair in nominal modifiers and
selected measure complements, preserving numeric, signed and variable forms
without arithmetic evaluation. Keep the lexical model data-only, morphology
and codecs in `deckmaste_lexical`, source adaptation in
`deckmaste_lexical_source`, and composition in generated v3 declarations.

| Witness | Required local outcome |
|---|---|
| Zhou Yu, Chief Commander | Declared analysis for `defending`; preserve its licensed form and distribution. |
| Blanket of Night | Declared noun `addition`; retain the compositional prepositional host. |
| Powerstone Shard | Declared `named` analysis with its selected name complement. |
| Nix | Past-participle `spent` from a declared paradigm, without guessed POS. |
| Wish | Declared noun `game` with its licensed countability. |
| Viridian Joiner | Declared noun `amount` with its measure-complement distribution. |
| Giant Growth | Signed `+3/+3` pair in the predicate's measure complement. |
| Goblin Offensive; Gelatinous Genesis | `1/1` and `X/X` modifiers retain both components and the separate token count. |

Also settle the duplicate small-Arabic representation: current grouped and
ungrouped codecs both realize a small digit, and both scalar Productions admit
it. `Gain 1 life.` must not acquire distinct linguistic Readings solely from
that invisible codec choice. Preserve meaningful retained variants and the
existing canonical distinction at the thousands boundary; prove independently
constructed numeral values roundtrip after any identity normalization. Do not
deduplicate unrelated Lexemes or different grammatical structures by text.
The pinned rules-text baseline has no literal `*/*`; do not claim that form's
re-coverage from the slash-group count. Its absence is not a grammatical ban;
classify additional notation and pin its own evidence before expanding this
batch.

Acceptance: verify exact supported card text at implementation and pin focused
positive/negative analysis and generated-AST tests. Reject malformed slash
pairs, lost signs, extra components and unlicensed inflections. Exercise both
the measure root and its real nominal/predicate consumers; distinguish local
success from whole-card failures owned by later batches. Account for all six
word gaps individually and audit new POS readings. Snapshot all nine named
frame-coordination cases from `english-v3-frame-coordination`, separating any
numeral-only multiplicity reduction from frame repair.

Apply [Lexical analysis](../../decisions/english-lexical-analysis.md#lexical-analysis),
[Source and roundtripping](../../decisions/english-lexical-analysis.md#source-and-roundtripping)
and [Chart admission and ambiguity](../../decisions/english-lexical-analysis.md#chart-admission-and-ambiguity):
Earley parsing, correlated packed partial/complete derivations, generated
checked constructors/rendering/total traversal, both roundtrip laws and every
valid Reading. Update Lean witnesses first for changed grammatical judgments.
STOP and report rather than use guessed morphology, word-named admission,
opaque source leaves, eager AST products or destructive selection. V2 remains
untouched; unhandled gaps route to `english-v3-systemic-residuals`. Standard
constraints apply.

## Landing record

Measured feature `lolyrvsq`, based on claim `mmksxwql`; matched coordinator
baseline `mwympzyt`. Kata's final refresh was a no-op. The v2 coverage lock is
unchanged at **19,953** covered identities. This ticket repairs local lexical
and measure composition, not all nine motivating Documents or the later
article, keyword-label and frame-coordination batches.

### Requirements and retained evidence

| Requirement | Implementation and independent witnesses |
|---|---|
| Six lexical gaps | Native declarations in `crates/deckmaste_lexical_source/lexicon/core.ron`: Addition is count/mass; Game and Amount are count nouns; Defend, Spend and Name have full declared paradigms. Spend overrides every preterite and the past participle to `spent`. The source test `lexical_measure_gaps_have_declared_categories_forms_and_countability` checks the six spellings' complete POS alternatives, countability, frames and unlicensed inflections; `every_declared_value_and_frame_survives_realization_and_reanalysis` checks the entire declared inventory. |
| Required lexical consumers | `independent_named_nominal_retains_selected_frame_and_catalog_identity`, `independent_participial_premodifier_preserves_verb_form_and_distribution`, `independent_amount_complement_consumes_its_frame_without_losing_countability`, and `lexical_gap_words_reach_existing_compositional_hosts` in `crates/deckmaste_english_v3/tests/grammar.rs`. |
| Ordered, signed slash notation | Generated scalar components and slash pairs preserve both components, independent signs, signed zero and variables without evaluating arithmetic. `slash_pairs_preserve_ordered_components_and_explicit_signs` includes malformed/extra/double-sign and thousands-boundary negatives. |
| Real nominal/predicate consumers | `slash_pairs_compose_with_nominal_modifiers_and_selected_predicates` and `independent_slash_consumers_preserve_count_components_and_leaf_order` retain the separate token count and selected predicate frame. Scalar-only consumers reject pair quantities, including `Gain 1/1 life.` and `by +3/+3`. |
| Small-Arabic identity | `NumeralCodec::canonical_notation` chooses ungrouped identity below magnitude 1,000. Grouped large values retain commas; codecs otherwise stay unchanged. `small_digits_do_not_multiply_a_predicate_reading` pins exactly one Reading of `Gain 1 life.`; `normalized_numerals_roundtrip_without_erasing_visible_notation` independently exercises signed boundary/extreme values and both visible large notations. |
| Both roundtrip laws and traversal | Independent complete AST values in the tests above check exact Reading sets, realization, frame/countability identity and ordered lexical leaves. The corpus command separately checks every counted Reading's admission, byte-exact realization, lexical ownership and node/word traversal against materialization traces. Debug fingerprints are diagnostic, not test oracles. |
| Lean before Rust | `english/English/MeasureWitnesses.lean` and `LexicalConsumerWitnesses.lean` provide inhabited lexical environments, independent signs and ordered components, count/predicate/name/participial/amount consumers, and negative judgments. The surrounding grammar, lexical and surface relations were updated first. |

Exact supported Oracle wording was checked against the pinned Scryfall snapshot
and card lookups. All nine motivating faces still have zero whole-Document
Readings; the focused enumeration has zero mechanical issues. Their required
local successes are:

| Face | Local success; remaining scope |
|---|---|
| Zhou Yu, Chief Commander | `defending player` retains gerund-participle form and declared attributive use, not a guessed adjective. Shortened self-reference remains outside this ticket. |
| Blanket of Night | `addition` and `in addition to other types` compose; the full possessive/compound host remains unresolved. |
| Powerstone Shard | `artifact named Powerstone Shard` selects the declared Name frame and existing exact catalog identity. |
| Nix | `no mana was spent` retains the declared nonfinite past participle and passive host. |
| Wish | `the game` and `games` retain count-noun behavior. |
| Viridian Joiner | `amount of {G}` consumes the selected nominal symbol complement; a separate empty nominal frame retains bare `amount`. |
| Giant Growth | `Target creature gets +3/+3.` composes; the duration host remains unresolved. |
| Goblin Offensive; Gelatinous Genesis | `X 1/1 tokens` and `X X/X tokens` retain count and pair separately; the full compound/color hosts remain unresolved. |

These residuals are not coverage losses. Article issues stay with
`english-v3-article-variants`; keyword labels and shared frames stay with their
existing tickets. Other unmatched composition and shortened-reference gaps
route to the final cause audit in `english-v3-systemic-residuals`, which mints
bounded repairs rather than implementing an omnibus fix. No literal `*/*`
was established as a rules-text witness; this patch does not add it or declare
it grammatically forbidden.

### Corpus reconciliation and linguistic audit

Both full runs used Scryfall JSONL SHA-256
`ba4952fd7eae58b49f5dd8277968169902f79f8a289644c1ba2b4376cc53bb22`.
There are no changed source texts or added/removed corpus identities.

| Tree/field | No Reading | One | Multiple | Mechanical issues |
|---|---:|---:|---:|---:|
| Coordinator `mwympzyt`, text | 30,957 | 1,166 | 445 | 0 |
| Feature `lolyrvsq`, text | 30,875 | 1,223 | 470 | 0 |
| Feature `lolyrvsq`, Type Line | 0 | 32,568 | 0 | 0 |

**Covered-to-uncovered identity list: empty.** There are 82 newly parsing
identities, disclosed below, and 307 already-covered identities with changed
Reading counts. Re-enumerating all 307 with all trees retained, normalizing
only small Arabic notation and its scalar wrapper maps 4,368 old values onto
exactly the 1,600 new values: 307 equal sets, no missing or extra structures.
This diagnostic comparison supports numeral-only multiplicity reduction;
the independent AST tests remain the value-roundtrip evidence.

The complete new-identity enumeration retains 1,796 Readings with zero
mechanical issues. Audit covered all 82 exact source texts, the complete
introduced-role and lexical-value inventories, and representative parent
structures, including the high-multiplicity Kalonian Hydra, Voracious Hydra
and Herald of Secret Streams counter/relative/preposition hosts. No confirmed
new syntactic invalidity was found. Existing nominal versus phrase/Clause
preposition attachments, relative-modifier scopes, and singular/plural
`you` alternatives remain retained; this is not a claim to have repaired
or semantically disambiguated those structures. The existing declared
`kicked` and `enchanted` adjective lexemes were not introduced by this patch.

The introduced roles are 66 slash-pair predicate identities, 14 counter-modifier
identities, Cleric of the Forward Order's selected Name complement, and
Timberpack Wolf combining a slash predicate with a Name complement.
Name is the only one of the six new Lexemes appearing in these whole-Document
additions: its nonfinite past participle, selected frame 1, with the exact
catalog Name. No guessed POS, admission preference or reading filter was added.

The nine frame-coordination witnesses retain the following complete counts.
Their values agree under the same numeral-only normalization; **none of the
required shared-frame repairs is discharged here**.

| Witness | Before | After |
|---|---:|---:|
| Brothers of Fire | 128 | 32 |
| Fireslinger | 128 | 32 |
| Goblin Artillery | 128 | 32 |
| Orcish Artillery | 128 | 32 |
| Orcish Cannoneers | 128 | 32 |
| Forge Devil | 184 | 46 |
| Psionic Entity | 64 | 16 |
| Reckless Embermage | 64 | 16 |
| Spicy Oatmeal Pizza | 736 | 92 |

### Deviations, additions and STOP resolutions

Seventeen generated Constructions were added, all for the requested consumers:
`BareFramedNoun`, `SymbolComplementNominal`, `ParticipialPremodifier`,
`CatalogName`, `PassiveNamePredicate`, `NamedNominal`,
`SmallUnsignedScalar`, `LargeUnsignedScalar`, `VariableScalar`,
`UnsignedScalar`, `PositiveScalar`, `NegativeScalar`, `SlashPair`,
`SlashMeasure`, `SlashModifiedNominal`, `FiniteSlashMeasure`,
`NonfiniteSlashMeasure`. No Construction was removed. The compiler's new
`numeral_sign` summary feature prevents doubled signs without inspecting a
Lexeme or constructing child AST products. `MeasureKind` keeps existing scalar
consumers from accidentally admitting pairs. Name, attributive form and nominal
complement marker features connect declared lexical properties to general
composition. The empty nominal frame prevents selected-complement readings
from masquerading as unframed nouns.

Rust tests: 11 added, four existing tests updated for normalized numeral
identity, zero restored/deleted/newly ignored. The preexisting ignored test
is retained. The added tests correspond to the requirement witnesses above,
the compiler sign-summary test, and the native lexical inventory test.
The independent consumer fixtures deliberately go beyond parse/render smoke
tests to pin the required structures.

Two STOPs were resolved:
- The initial claim that lexical input lacked names was incorrect:
  `CatalogKind::CardNames` and `load_workspace` already supply full exact
  names. Source inspection resolved this; no new name provider or nickname
  inference was added.
- Coordinator change `nmxrttqo` removed the v2 callers of
  `CorpusSelectionArgs::all` but left the helper, breaking strict clippy.
  The coordinator reproduced the failure. The user explicitly authorized
  deleting that unused helper; `--all` parsing and selection remain intact,
  and strict clippy now passes without a suppression.

No new glossary term is required: the name complement consumes a catalog-backed
Identity Claim; the remaining labels describe notation, Word Forms and existing
grammatical functions. No word-named admission guard was added. Production
declarations contain 151 feature requirements and 13 agreement equations;
they have no custom licensing callbacks. Construction-form literals are
separators/punctuation/signs, not vocabulary: literal/vocabulary overlap list
is empty. The measured grammar has **151 Constructions and 57 Categories**.

Lexical homographic spellings increased **873 → 877**. The complete introduced
homograph list is `Name`, `Names`, `name`, `names`, each pairing
`lexeme:CommonNoun/Name` with `lexeme:Verb/Name`. Other new spellings
(`addition(s)`, `amount(s)`, `game(s)`, `defend(s)/defended/defending`,
`spend(s)/spent/spending`, `named/naming`, including initial capitals)
introduce no cross-Lexeme homographs. The count enumerates
`Lexicon::values()`, realizes each value and groups distinct Lexeme IDs by
exact surface; grammatical inflections of one Lexeme are not homographs.
The capitals are sentence-initial surface variants, not separate proper names:
this is one noun–verb Lexeme pair across four case-sensitive spellings.

### Reproduction and verification results

Set report variables to caller-chosen distinct output paths outside tracked
source. Run on the measured trees above with the matching snapshot; omission
of `--reading-limit` requests complete enumeration.

```sh
cargo xtask gate --changed --from mmksxwql --clippy --run
english/scripts/axioms
cargo run --release -p xtask --bin cargo-xtask -- english-v3 --all --workers 1 --output "$text_report"
cargo run --release -p xtask --bin cargo-xtask -- english-v3 --all --field type-line --workers 1 --output "$type_line_report"
cargo run --release -p xtask --bin cargo-xtask -- lexical --card-name 'Powerstone Shard' --workers 1 --output "$lexical_report" --export "$lexical_inventory"
jj --no-pager diff --git | cargo xtask cite audit --diff
```

The affected-crate gate derives and runs the complete reverse-dependency closure:
`deckmaste_construction_v3_core`, `deckmaste_lexical`,
`deckmaste_lexical_source`, `deckmaste_construction_v3`,
`deckmaste_english_v3`, `xtask`. **497 passed, zero failed, one preexisting
ignored**; strict all-target clippy passed for that closure. Nightly formatting
checks passed. `english/scripts/axioms` reports
`English theorems checked: 2815`, `disallowed axiom uses: 0`, with a clean
48-job `lake build --wfail`. Citation diff audit:
`audited 0 citation site(s) — nothing selected`.
The focused lexical inventory command reports `1 supported faces`,
`0 unknown occurrences`, `35 matched readings`, and
`38661 independent values checked`; the exported inventory covers the whole
declared lexical environment, while occurrence counts cover only the selected
Powerstone Shard face.

For the nine local or nine coordination witnesses, replace `--all` with
repeated `--card-name` selectors. For identity comparisons, select the listed
durable IDs with `--identity-manifest`; set `--samples-per-face 10000` to
retain every Reading at the measured counts. The default two tree samples
are not a complete analysis inventory. Compare input hash and exact source
before joining by identity; classify every covered-to-uncovered identity.
No session-local output file is a prerequisite for this record.

The separate full baseline was needed because activation used MTGJSON but this
ticket's refreshed parent uses Scryfall. The final full text and Type Line runs
followed subset iteration; the later audits used bounded identity selections.

Performance advisory (all at one worker, v2 lock 19,953):
baseline text `mwympzyt` **9.600 s**, **38,330 ns/B**, host load
27.92/16.99/8.26; feature text `lolyrvsq` **10.368 s**, **35,714 ns/B**,
load 8.21/13.15/7.96; feature Type Line **2.664 s**, **4,143 ns/B**,
load 1.02/2.88/4.80. All are below the 16.26 s advisory, but unequal host loads
do not establish a controlled speed comparison.

### Newly parsing identities and selected construction roles

These are complete-Document successes, not a claim that each face has only
one Reading. S = finite Get predicate selecting the ordered signed SlashPair;
C = SlashPair modifying the count noun Counter while preserving any enclosing
count; N = past-participle Name predicate with exact catalog Name complement
modifying the nominal. The exact source below identifies the surrounding
clause/ability and modifier hosts; combinations denote both introduced roles.
Existing valid alternatives are retained, not selected away.

| Face and durable identity | Readings | New role | Exact Oracle text |
|---|---:|---|---|
| A Tale for the Ages — `682a0770-8fe0-4142-aa0c-b60af3628a33#card` | 4 | S | Enchanted creatures you control get +2/+2. |
| Adaptive Shimmerer — `8003ca2d-0c1d-47c1-bcd1-4609615c7a82#card` | 9 | C | Flash<br>This creature enters with three +1/+1 counters on it. |
| Anthem of Champions — `f1d8e9a6-1903-4be5-8609-1009a463f393#card` | 2 | S | Creatures you control get +1/+1. |
| Arvad the Cursed — `d561dcff-0a19-46ac-9dee-f374c840d532#card` | 6 | S | Deathtouch, lifelink<br>Other legendary creatures you control get +2/+2. |
| Bad Moon — `fc5d3341-cbce-49e5-93cc-8add92479dca#card` | 1 | S | Black creatures get +1/+1. |
| Belenon War Anthem — `e462e78d-2d26-4a2d-8d8f-5f31abe5c924#face:1` | 2 | S | Creatures you control get +1/+1. |
| Benalish Honor Guard — `0595b074-8f25-469d-9e82-ed60c4a7a2cd#card` | 8 | S | This creature gets +1/+0 for each legendary creature you control. |
| Benalish Marshal — `2cc439e8-d112-44e6-bc5a-6e99333c519a#card` | 4 | S | Other creatures you control get +1/+1. |
| Bladestitched Skaab — `d72b8254-3df8-431b-955a-ec2aea493e2b#card` | 4 | S | Other Zombies you control get +1/+0. |
| Blessed Orator — `c0856fea-cbdc-427b-ae00-bfd9311f284f#card` | 4 | S | Other creatures you control get +0/+1. |
| Boartusk Liege — `4fcffb56-5ab6-44e2-804e-8b3c55cc5415#card` | 36 | S | Trample<br>Other red creatures you control get +1/+1.<br>Other green creatures you control get +1/+1. |
| Boneclub Berserker — `a08f69f9-de48-46d6-8b75-31886159cf45#card` | 8 | S | This creature gets +2/+0 for each other Goblin you control. |
| Borderland Behemoth — `378a78d8-9001-4aa5-a66b-2aca27fa8180#card` | 8 | S | Trample<br>This creature gets +4/+4 for each other Giant you control. |
| Branch of Boseiju — `ec08aeb3-bba7-4982-9160-68d25bd411d6#face:1` | 4 | S | Reach<br>This creature gets +1/+1 for each land you control. |
| Cleric of the Forward Order — `84a2a592-0e31-434f-b3b4-f2013d86f29d#card` | 16 | N | When this creature enters, you gain 2 life for each creature you control named Cleric of the Forward Order. |
| Collective Blessing — `7f7049e8-49ed-46da-89f7-1e40aefb3b0c#card` | 2 | S | Creatures you control get +3/+3. |
| Cragplate Baloth — `3f599e15-cbc5-4170-b6b4-efd16aa222ac#card` | 15 | C | Kicker {2}{G}<br>This spell can't be countered.<br>Hexproof, haste<br>If this creature was kicked, it enters with four +1/+1 counters on it. |
| Day of Destiny — `64247fcb-0e1b-48c9-bdac-769d7edd8d1d#card` | 4 | S | Legendary creatures you control get +2/+2. |
| Death Pit Offering — `d445032b-b243-4203-9c88-a0ac357fb9b5#card` | 4 | S | When this enchantment enters, sacrifice all creatures you control.<br>Creatures you control get +2/+2. |
| Dictate of Heliod — `b989279c-665d-4f15-afc1-adf3872a4851#card` | 2 | S | Flash<br>Creatures you control get +2/+2. |
| Dread of Night — `b431a73b-cf4c-4154-810f-82b7c6099e66#card` | 1 | S | White creatures get -1/-1. |
| Earth Servant — `7ce30379-3f07-4b94-b2d6-a89b4763971b#card` | 4 | S | This creature gets +0/+1 for each Mountain you control. |
| Endless One — `7a51780a-fa28-4ee0-94c7-4330800ca9cb#card` | 9 | C | This creature enters with X +1/+1 counters on it. |
| Faithful Watchdog — `139fb542-89fd-4b9e-87e6-ec925bcca93a#card` | 9 | C | Vigilance<br>This creature enters with three +1/+1 counters on it. |
| Fire Nation's Conquest — `8893af49-2f90-4aed-bcaf-905bb9e8ed8f#card` | 2 | S | Creatures you control get +1/+0. |
| Flowstone Surge — `fb1755a0-3334-419b-8cb5-5a3ac7fa5b13#card` | 2 | S | Creatures you control get +1/-1. |
| Gaea's Anthem — `3754dce0-3e97-406f-8807-a4942a222c41#card` | 2 | S | Creatures you control get +1/+1. |
| Glass of the Guildpact — `a967921c-cfd4-422c-a08b-42cdd80f89c0#card` | 4 | S | Multicolored creatures you control get +1/+1. |
| Glen Elendra Liege — `946bba74-0951-408c-b06f-167739b10934#card` | 36 | S | Flying<br>Other blue creatures you control get +1/+1.<br>Other black creatures you control get +1/+1. |
| Glorious Anthem — `e3886fe8-9b76-4613-8891-4ec74657c087#card` | 2 | S | Creatures you control get +1/+1. |
| Guidelight Synergist — `c0bc3704-9bf9-4039-b3f8-205c1e5eb6e9#card` | 4 | S | Flying<br>This creature gets +1/+0 for each artifact you control. |
| Herald of Secret Streams — `366a218e-84d3-4cf9-bbc8-f2f8ecce92a3#card` | 18 | C | Creatures you control with +1/+1 counters on them can't be blocked. |
| Hold the Gates — `7e6fcbc0-691a-4bd7-b063-debc86f4506a#card` | 4 | S | Creatures you control get +0/+1 for each Gate you control and have vigilance. |
| Honor of the Pure — `eb4188bb-62df-4309-bde1-f66318fe2f05#card` | 4 | S | White creatures you control get +1/+1. |
| Inspiring Veteran — `aa1a63dd-acb1-465f-8970-667b8d7c57c9#card` | 4 | S | Other Knights you control get +1/+1. |
| Ivy Elemental — `38517711-e570-4337-b269-addcf8bfdd74#card` | 9 | C | This creature enters with X +1/+1 counters on it. |
| Jacques le Vert — `c030ca14-33cb-40b3-a1f5-b6d0cb0efd49#card` | 4 | S | Green creatures you control get +0/+2. |
| Kaervek, the Spiteful — `b338d879-189d-4b74-9d63-42920fdb9855#card` | 1 | S | Other creatures get -1/-1. |
| Kalonian Hydra — `7bd36106-04fe-481f-b16e-e076dcbb183b#card` | 882 | C | Trample<br>This creature enters with four +1/+1 counters on it.<br>Whenever this creature attacks, double the number of +1/+1 counters on each creature you control. |
| Kargan Warleader — `63324e72-e580-456a-91be-766c9f07b7b3#card` | 4 | S | Other Warriors you control get +1/+1. |
| Kaysa — `0fd18a9b-c112-4e48-8fd7-e53fe4500943#card` | 4 | S | Green creatures you control get +1/+1. |
| King of the Pride — `96d0e4dd-6cc2-4349-ac44-785b50f8dd90#card` | 4 | S | Other Cats you control get +2/+1. |
| Kongming, "Sleeping Dragon" — `21e9e1a9-5d6d-473e-adab-6a1e8e2b0ebd#card` | 4 | S | Other creatures you control get +1/+1. |
| Krakilin — `3eb49097-42f8-4388-907d-32a738749d8a#card` | 9 | C | This creature enters with X +1/+1 counters on it.<br>{1}{G}: Regenerate this creature. |
| Legion Lieutenant — `5fa1b2f0-3ba4-49db-9cb5-b6130e4c255e#card` | 4 | S | Other Vampires you control get +1/+1. |
| Lightning Serpent — `d07b4bb6-0c8d-44ea-a5b3-eeb6e36f3631#card` | 18 | C | Trample, haste<br>This creature enters with X +1/+0 counters on it.<br>At the beginning of the end step, sacrifice this creature. |
| Marshal of Zhalfir — `9ad01fb2-2ba7-447a-a5ef-e57d46421635#card` | 4 | S | Other Knights you control get +1/+1.<br>{W}{U}, {T}: Tap another target creature. |
| Meng Huo, Barbarian King — `ea7fdff3-9d45-4a4a-b7fe-84b334f04291#card` | 6 | S | Other green creatures you control get +1/+1. |
| Merfolk Mistbinder — `be56dfea-1ece-4f06-b1cc-d7425b5018b4#card` | 4 | S | Other Merfolk you control get +1/+1. |
| Militant Inquisitor — `d1e326e8-6e9e-4dbd-8c2f-0bef90db2bb7#card` | 4 | S | This creature gets +1/+0 for each Equipment you control. |
| Mogg Squad — `4dac142a-c8f8-45c8-85d3-40e170dee112#card` | 9 | S | This creature gets -1/-1 for each other creature on the battlefield. |
| Muraganda Petroglyphs — `e6eff050-f1f6-49ce-a92e-1d1e13e51084#card` | 2 | S | Creatures with no abilities get +2/+2. |
| Night of Souls' Betrayal — `916bd025-c44f-49c9-8d76-4b7b2f9a8ba3#card` | 1 | S | All creatures get -1/-1. |
| Nim Grotesque — `ed4d4084-739b-43ac-bd2b-a86d1ecd7bbd#card` | 4 | S | This creature gets +1/+0 for each artifact you control. |
| Nim Lasher — `0203e250-d60e-46c6-9c58-d620f0377be0#card` | 4 | S | This creature gets +1/+0 for each artifact you control. |
| Nim Shrieker — `1dff7c82-e49f-45e8-8f66-d5801f420fa6#card` | 4 | S | Flying<br>This creature gets +1/+0 for each artifact you control. |
| Nimbus Swimmer — `ce02ef24-d53e-4025-bc45-ca27aba89fe9#card` | 9 | C | Flying<br>This creature enters with X +1/+1 counters on it. |
| Oathsworn Giant — `d3828907-1321-40ca-9531-30401e82a6b8#card` | 4 | S | Vigilance<br>Other creatures you control get +0/+2 and have vigilance. |
| Power Boost — `fed58ccc-db56-4094-8b56-bd29b00e9327#card` | 2 | S | Creatures you control get +1/+0. |
| Pride of the Perfect — `83e2b251-bd60-4c5c-b9e2-32c22b28fb8b#card` | 2 | S | Elves you control get +2/+0. |
| Pterafractyl — `cca03b76-dd3c-4228-b97a-f7fa20819b2e#card` | 18 | C | Flying<br>This creature enters with X +1/+1 counters on it.<br>When this creature enters, you gain 2 life. |
| Regal Imperiosaur — `83a76601-ef59-4888-af70-5088dd1f504c#card` | 4 | S | Other Dinosaurs you control get +1/+1. |
| Rising of the Day — `434a20f8-3f87-4004-9155-0f196fc2257e#card` | 8 | S | Creatures you control have haste.<br>Legendary creatures you control get +1/+0. |
| Shifting Wall — `7c852dfe-8238-461e-814e-9667807f2cf5#card` | 9 | C | Defender (This creature can't attack.)<br>This creature enters with X +1/+1 counters on it. |
| Shivan Devastator — `b7daa74c-6142-4107-9355-be98af6ccf13#card` | 9 | C | Flying, haste<br>This creature enters with X +1/+1 counters on it. |
| Squirrel Mob — `451486df-6160-4ff7-b47c-dd3760417a31#card` | 9 | S | This creature gets +1/+1 for each other Squirrel on the battlefield. |
| Squirrel Sovereign — `f43d1ea5-8127-4b64-a87b-ee5cc9b9e9fa#card` | 4 | S | Other Squirrels you control get +1/+1. |
| Stronghold Taskmaster — `7a2b134d-9d7e-4d43-98fc-c776fc4bad45#card` | 1 | S | Other black creatures get -1/-1. |
| Supreme Phantom — `9c5f4d02-eedb-4c6c-9f13-1b7a45382483#card` | 4 | S | Flying<br>Other Spirits you control get +1/+1. |
| Thirsting Bloodlord — `b04ba676-a1f5-4216-9d93-3c475b504bfa#card` | 4 | S | Other Vampires you control get +1/+1. |
| Thistledown Liege — `7b07356f-9c3f-4875-939e-aa64eb6b5ce2#card` | 36 | S | Flash<br>Other white creatures you control get +1/+1.<br>Other blue creatures you control get +1/+1. |
| Tide Drifter — `428c18aa-b195-4cf2-a1a9-09723b26eb0a#card` | 6 | S | Devoid (This card has no color.)<br>Other colorless creatures you control get +0/+1. |
| Timberpack Wolf — `c9375dca-2b32-4185-b88a-4bd5dde9a5d6#card` | 12 | S+N | This creature gets +1/+1 for each other creature you control named Timberpack Wolf. |
| Turtle Power! — `15934a19-a29e-4728-816f-20b610eec377#card` | 2 | S | Flash<br>Turtles you control get +2/+2. |
| Ultron, Machine Overlord — `19fd953c-b6d4-457f-b85f-b681c5b0e341#card` | 2 | S | Flying<br>Other Robots and Constructs you control get +2/+2. |
| Urborg Shambler — `7947a210-32d0-4c99-82b4-4b5969447832#card` | 1 | S | Other black creatures get -1/-1. |
| Veteran Armorer — `1b1214e9-1581-4008-b3d4-8b7af6b092fd#card` | 4 | S | Other creatures you control get +0/+1. |
| Voracious Hydra — `ff8f5a4b-112a-425e-b489-7ee26d1d9fb3#card` | 396 | C | Trample<br>This creature enters with X +1/+1 counters on it.<br>When this creature enters, choose one —<br>• Double the number of +1/+1 counters on this creature.<br>• This creature fights target creature you don't control. |
| Wedding Festivity — `259ac30c-cc05-4c04-9b23-71283f84b808#face:1` | 2 | S | Creatures you control get +1/+1. |
| White Lotus Reinforcements — `c55a4b25-ccc4-49de-9f98-a829c547a64d#card` | 4 | S | Vigilance<br>Other Allies you control get +1/+1. |
| Yavimaya Enchantress — `495fc654-7986-4d76-9d5c-bc1484e98de2#card` | 7 | S | This creature gets +1/+1 for each enchantment on the battlefield. |
| Yotian Tactician — `0d9d7f04-6319-4596-8b74-de5a794936e5#card` | 4 | S | Other Soldiers you control get +1/+1. |
