---
needs: [english-v3-number-transparent-concord]
---
# Audit and remove redundant bare-target projection routes

Determine whether the direct TargetNounPhrase and
BarePlural(TargetedNominal) routes for the attested constituent “target creatures”
encode the same grammatical targeting function. The exact probe yields two Noun
Phrase Readings with identical words, Plural/Third Agreement and targeting; the
Nominal probe has one TargetedNominal. No scope-bearing material intervenes here.
Exploration tree qymzymor (parent sqnmlrnz; grammar baseline vwtnryos, covered 13,226).

The glossary deliberately permits determinative and nominal-modifier projections
of the Targeting Marker. Do not remove one legitimate function globally or change
that ruling merely to lower counts. Establish an explicit structural equivalence
for the local overlap; if both routes intend one function, prevent its duplicate
derivation at source through the shared NP pipeline. If they represent genuinely
different functions, document that result and preserve them.

Preserve attested composition in Blinding Beam's “two target creatures”, Sanguine
Indulgence's “two target creature cards”, Bojuka Bog's “target player's graveyard”,
and Dwarven Thaumaturgist's “target creature's power and toughness”. Audit modifiers
above/below targeting, singular/plural, quantifiers, genitives and coordination;
do not collapse those scopes by analogy with the bare case. Avoid a blanket
post-parse semantic quotient or card/word-named guards.

Acceptance: independent expected structural sets with both roundtrip laws;
classify each removed route as redundant versus invalid versus retained-distinct.
Report actual corpus Reading-count changes and identity-level coverage losses,
not a presumed global twofold reduction. Optional scratch: /tmp/english-v3-np-probes.
Standard constraints apply.

## Sequencing

The `needs:` edge is sequencing, not a technical dependency: these grammar
tickets touch overlapping structures and are worked one at a time. Write the
pinned witnesses as tests before implementing.
The landing compares corpus Reading identities before and after on its own tree.

## Prior attempt

The bookmark `archive-english-v3-compact-grammar` contains an unfinished attempt
at this ticket, made in one change together with six others and never verified
against the corpus. It adds a `targeting_projection.rs` test file. Read it for ideas if useful. Do not rebase onto
it, and treat every claim in its ticket notes as unverified.


## Landing record

Completed 2026-10-06. Standard constraints apply. Baseline claim tree
`wnmnkpsq`, covered 13,596; implementation `rqprnwty`, covered
13,596. Both census headers name `rqprnwty`, the working change reused
across the experiment: the before executable was built before the admission
change, from declarations identical to the claim tree. Declaration SHA-256:
before `d454b1982bde0b22dc41f569b229b54c4b7e3117ed9b7e8fc3cbf7d9d8c8a866`;
after `22df6fd061e1ed5ab5a381857f4a26d86f954a12e85ae171ff1868e88e8c0f0c`.
The lexical inventory digest is unchanged:
`3529edcda18b9a654fead1641ec76614ca3c449d2066a0147d337956b031be07`. Every number below belongs to these
measured states. V3 emits no coverage lock; covered here is the complete face
census, not a substituted legacy lock count.

### PROVE — projection boundary and retained identities

The accepted [Targeting Marker ruling](../../decisions/english-v2-rewrite.md#amendment-one-targeting-marker-with-two-projections-2026-09-04)
assigns bare singular targeting to the direct determinative projection and
zero-determined plural targeting to the nominal-modifier projection. CGEL,
Ch. 5 §4, pp. 354–358 supplies the general distinction between Determiner
function, Determinative Category and bare NP formation; it does not prescribe
this project's Oracle-specific targeting allocation. The glossary and ruling
are preserved. The archived attempt's opposite route restriction was not used.

The explicit local correspondence is
`TargetNounPhrase(marker, nominal) → BarePlural(TargetedNominal(marker, nominal))`
for a plural count Nominal. The identical marker and entire nominal child are
preserved, including their lexical identities, Word Forms, spelling variants
and internal scope. Both routes previously exported Plural/Third, Common Case
and Targeting=Yes. The direct plural route supplied no independently licensed
function under the ruling. Admission now requires Singular Number on the
direct projection, letting plural descriptions use the existing shared
Nominal pipeline. No runtime structural quotient, change to the Preference
policy or Construction Costs, new feature, lexical owner or Construction is
involved. Each retained replacement has one additional default-cost
Construction, so total cost rises by one per replaced occurrence and preferred
presentations can change. This affects optional presentation; admission and
coverage use the complete Reading sets.

Both full censuses completely enumerate 32,828 supported faces on identical
stripped inputs. Covered-face losses: 0. Newly covered faces: 0. Reading
identities decrease from 157,575 to 157,307: exactly 268 redundant routes on
40 faces, with no newly admitted Reading. Every removed identity is named in
`/tmp/targeting-delta.json` and paired with its exact retained identity.
The exhaustive comparison of all 157,575 baseline Reading identities hashes
complete generated Reading Debug without normalization. All 157,307 after
identities already occur in the baseline; the remaining 268 are the removed
routes on the 40 changed faces. For each removal, replacing
only the plural direct targeting node gives a hash actually enumerated on
the after tree. An outer Modifier is never moved across targeting by this
comparison. Some roots contain more than one replaced occurrence.

The unchanged faces have identical complete Reading identity sets, including
Avacyn, Guardian Angel's 33,856 Readings. The full before/after identity streams
are complete and independently checked against every face's census count.
The smaller changed-face manifests record the same exhaustive comparison on
the 40 affected faces. All diagnostic streams and comparison code remain in
scratch paths.

Classification: all 268 removals are redundant routes with retained
counterparts, excluded by the direct projection's singular licence. There
are zero invalid removals lacking counterparts, zero unexplained losses,
zero regressions and zero re-coverage obligations. Retained-distinct structures
include postmodification above versus below targeting, wide versus first-Conjunct
targeting in Coordination, singular versus plural invariant noun forms, and
the two lexical analyses of cardinal one. Their equality is never inferred
from equal text or a common targeting feature.

All counted Readings pass declaration admission, byte-exact realization,
lexical ownership/context and construction/word traversal identity. Parser,
materialization, admission, realization, ownership, duplicate-Reading and
traversal issues: 0 on both trees. Independent expected structural sets
exercise both roundtrip laws and exact node/leaf equality. The now-unlicensed
independent plural direct value fails both admission and realization.
Lexical source loading and GrammarEnvironment construction succeed. Added
word-named licensing guards: 0; the single added guard reads declared Number.
V3 has no legacy permitted-licensing-checker total, so that retired metric
is not represented as a zero checker census.

### DISCLOSE — scopes, census and affected identities

| Complete census | `wnmnkpsq` / 13,596 | `rqprnwty` / 13,596 |
|---|---:|---:|
| No Reading | 19,232 | 19,232 |
| Unique Reading | 6,647 | 6,648 |
| Multiple Readings | 6,949 | 6,948 |
| Undetermined | 0 | 0 |
| Complete Reading total | 157,575 | 157,307 |

No specificity-based Selection census applies to V3; retained multiple
Readings remain successful parsing. No newly covered analysis needs disclosure.
Armory Automaton alone changes from multiple to unique; all other changed
faces retain multiple Readings.

The independent witnesses retain Blinding Beam's two target creatures,
Sanguine Indulgence's two target creature cards, Bojuka Bog's targeted
singular possessor, and Dwarven Thaumaturgist's targeted possessor with its
coordinated power-and-toughness head. Aerial Volley's with-flying Modifier
retains both nominal scopes. Equipment's invariant spelling retains its
Singular direct and Plural nominal projections. Adjective and participial
Premodifiers remain below targeting; the existing exclusions above targeting
are unchanged. Singular, plural, quantified, relative, genitive and coordinated
composition are tested separately, rather than collapsed by analogy with the
bare overlap.

The seven-face subset has 0 issues. Whole-document counts for Blinding Beam
and Bojuka Bog remain 6 and 1; Sway of Illusion changes 8→4 and Eerie Interlude
90→54. Sanguine Indulgence, Dwarven Thaumaturgist and Kaboom! remain at zero on
both trees despite successful pinned constituents. Dwarven Thaumaturgist has
the existing Switch vocabulary gap; the other two have no lexical gaps and
retain unresolved whole-document grammar causes. No fragment success is
reported as face coverage.

Every face whose Reading count changes is listed below. All losses in this
table have the redundant-route classification and retained counterparts above.

| Face identity | Card / face | Before → after Readings |
|---|---|---:|
| `0634091a-a74c-4cea-b6d1-7324a725554a#card` | Eerie Interlude | 90 → 54 |
| `15ec1bfc-b0a0-4058-87bb-8dbaf17b7f3b#card` | Isao, Enlightened Bushi | 3 → 2 |
| `17b4778b-82b1-4845-ad08-00f3ff66877b#card` | Perpetual Timepiece | 23 → 14 |
| `1bd5fd97-f898-4e17-9e8f-5fa93b9eef71#card` | Renewing Touch | 34 → 20 |
| `1e77cf90-ac51-4ac7-b123-be04aabe1688#card` | Blessings of Nature | 10 → 5 |
| `1ed80dd0-0980-4830-aad2-5b78ca373098#card` | Piper's Melody | 34 → 20 |
| `23bf0648-7097-41df-a539-1d36ac42cf9c#card` | Depthshaker Titan | 28 → 16 |
| `23d4f436-a417-4a26-b5b8-c698f11a186b#card` | Acid Web Spider | 3 → 2 |
| `334fba11-e500-40fd-a142-41ff553642b5#face:1` | Stolen Goodies | 27 → 16 |
| `3765e8bb-e70d-4503-b8dc-1e684e434c18#card` | Verdurous Gearhulk | 32 → 19 |
| `39d60cd8-bebc-407d-b361-8e49e70e339a#card` | Black Poplar Shaman | 3 → 2 |
| `436b5989-9b8c-4e03-a82e-046837c891ca#card` | Reign of Chaos | 3 → 2 |
| `54175132-2c44-4749-8dfd-d08dcc63e4b3#face:1` | Grove's Bounty | 27 → 16 |
| `56828166-eaa3-4711-91b6-401a3e3b733f#card` | Turn to Dust | 3 → 2 |
| `5a10c3a5-6724-4e5a-ae4f-b27dde12735a#card` | Brass Squire | 4 → 3 |
| `63e0f03f-16da-4c09-ad03-8945f660942d#card` | Daughter of the Deep | 18 → 12 |
| `6d138115-36bf-4cf8-8033-197cea2d0208#card` | Jugan, the Rising Star | 14 → 7 |
| `6f2ba142-5e7c-4709-ae03-e028cb2a9872#card` | Super-Soldier Serum | 40 → 24 |
| `7424560f-557f-4bc9-a3e7-eb890c73aaa3#card` | Mantle of the Ancients | 60 → 42 |
| `764e8e6e-0ace-4fc8-8ca5-468a10ae3b73#card` | Deeproot Elite | 4 → 3 |
| `77c14618-ced1-4a46-a7fd-06cb2f7f38ff#card` | Cryoclasm | 4 → 3 |
| `8ae3562f-28b7-4462-96ed-be0cf7052ccc#card` | Kor Outfitter | 4 → 3 |
| `9128b952-bd36-4a3a-9566-34d999305991#card` | Jade Guardian | 4 → 3 |
| `99140891-face-4015-aacd-1309e87d8f9f#card` | Display of Power | 15 → 10 |
| `9a7e2298-8855-43a7-8cb1-4b31e4058c3f#card` | Sway of Illusion | 8 → 4 |
| `9b22cc97-003f-4227-acdc-7a0857674b67#card` | Magnetic Theft | 3 → 2 |
| `a45b3934-1c9b-4cff-98b1-c9ac2f7759ea#card` | Rustspore Ram | 3 → 2 |
| `a483b1fd-751a-447e-8e4f-a54b1c194d2c#card` | Ogre Geargrabber | 33 → 24 |
| `ab5b697b-ab62-4ee7-a10e-cc937fef3095#face:1` | Dust | 19 → 13 |
| `bd4ab393-9538-4230-b2a1-bc77099481c9#card` | Iron Hills Stalwart | 8 → 6 |
| `c892d28c-4a78-4a7f-b8d5-c3663b819c5e#card` | Deepchannel Duelist | 8 → 6 |
| `cb806239-32bf-4850-be65-239444c347a5#card` | Iname, Life Aspect | 14 → 8 |
| `cf7352bf-3971-4bb7-88b9-65ab9a219289#card` | Loaming Shaman | 25 → 15 |
| `d32c6f64-c4b0-4451-8986-a0abc1fc2bb3#card` | Descent of the Dragons | 16 → 8 |
| `dff5d16f-3a41-4873-8188-076e77f18c75#face:0` | Leave | 5 → 3 |
| `e4212b05-d397-49fb-af8b-9e52a60e6d1e#card` | Aquatic Incursion | 72 → 48 |
| `eff0a1ad-ecc6-416f-a4ae-96b26cd8a905#card` | The Elderspell | 6 → 3 |
| `f8a06210-de30-4e87-835e-80a6df5716ca#card` | Auriok Windwalker | 4 → 3 |
| `fa3335be-80b1-49d7-9809-5bef13be79f4#card` | Armory Automaton | 2 → 1 |
| `fa360a9d-3cff-4833-9ce2-b2f53d26127f#card` | Unforge | 3 → 2 |

Deviations and additions: no Construction or vocabulary addition/deletion.
Eight test functions were added for the ticket's structural audit, including
invariant Equipment Number, authentic premodification, cardinal lexical
ownership and coordination-scope contrasts beyond the four named pins.
Three existing Number Transparency tests were re-spelled to the retained
plural targeting route: sway_subject_preserves_exact_readings_and_nominal_head,
independently_constructed_sway_clause_requires_plural_third_concord, and
eerie_interlude_keeps_both_relative_clause_scopes_and_plural_concord. All six
existing tests remain, with the same concord, ownership, head and scope outcomes.
Restored: 0; re-spelled: 3; added: 8; newly ignored: 0; removed: 0.
STOPs: none; no accepted ticket/ruling contradiction was resolved. Glossary
gaps: none; no existing term was redefined to fit an identifier.

### REPORT — inventories and performance

Authored grammar counts on both measured trees: 170 ordinary Constructions,
44 shared schemas (214 authored Constructions in total), 308 schema Category
instances, 530 static Productions and 586 workspace-compiled Productions.
Declaration lines: 2,692→2,693. These are provenance figures, not targets fitted
to coverage. The covered count remains 13,596 on both named trees.

Unchanged homograph inventory (120 surfaces):
`'d`, `'s`, `Adamant`, `Addendum`, `Alliance`, `Battalion`, `Bloodrush`, `Celebration`, `Channel`, `Chroma`, `Cohort`, `Constellation`, `Converge`, `Council's dilemma`, `Coven`, `Delirium`, `Descend 4`, `Descend 8`, `Disappear`, `Domain`, `Eerie`, `Eminence`, `Enrage`, `Fateful hour`, `Fathomless descent`, `Ferocious`, `Flurry`, `Formidable`, `Grandeur`, `Hellbent`, `Heroic`, `I`, `Imprint`, `Infusion`, `Inspired`, `Join forces`, `Kinship`, `Landfall`, `Lieutenant`, `Magecraft`, `Metalcraft`, `Morbid`, `More Than Meets the Eye`, `Opus`, `Pack tactics`, `Paradox`, `Parley`, `Radiance`, `Raid`, `Rally`, `Renew`, `Repartee`, `Revolt`, `Secret council`, `Spell mastery`, `Strive`, `Survival`, `Sweep`, `Tempting offer`, `Threshold`, `Undergrowth`, `Valiant`, `Vivid`, `Void`, `Will of the council`, `control`, `copies`, `copy`, `cost`, `costs`, `counter`, `counters`, `cycling`, `deathtouch`, `decayed`, `die`, `double strike`, `draw`, `draws`, `exalted`, `exile`, `exiles`, `first strike`, `flying`, `goaded`, `harnessed`, `haste`, `her`, `hexproof`, `his`, `if`, `indestructible`, `it`, `less`, `lifelink`, `menace`, `name`, `names`, `one`, `reach`, `shadow`, `solved`, `suspected`, `tapped`, `target`, `targets`, `that`, `time`, `to`, `trample`, `turn`, `turns`, `untap`, `untapped`, `up`, `vigilance`, `you`, `’d`, `’s`, `∞`.

Form-literal/vocabulary overlap inventory: empty on both measured trees.
The compiled literal inventory is checked against realized lexical surfaces;
no form or lexical declaration changed.

| Tree / covered | Workers | Host load (1/5/15 min) | Corpus wall ns | Checked-text thread CPU |
|---|---:|---|---:|---:|
| `wnmnkpsq` / 13,596 | 8 | 23.90/16.56/16.06 | 574,842,944,520 | 1,123,358 ns/B |
| `rqprnwty` / 13,596 | 8 | 14.15/16.29/16.12 | 609,980,482,039 | 1,515,068 ns/B |

Both busy-host runs exceed the inherited 16.26s quiet-host ceiling. Concurrent
verification and diagnostic enumeration limit performance attribution; this
is an advisory routed to english-v3-census-tractability, not a coverage gate.

Verification:

```text
cargo xtask gate --changed --clippy --run
cargo test -p deckmaste_construction_core -p deckmaste_lexical_source -p deckmaste_semantics_v2 -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask
cargo clippy -p deckmaste_construction_core -p deckmaste_lexical_source -p deckmaste_semantics_v2 -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask --all-targets -- -D warnings
cargo xtask english-v3 --all --workers 8 --samples-per-face 0 --output /tmp/targeting-after.json
```

The derived reverse-dependency closure passes: 1,207 passed, 0 failed,
2 pre-existing ignored across 93 test/doc-test suites. Strict clippy passes
with warnings denied. The existing ignored tests are
seedborn_muse_retains_relative_clause_attachment (blocked on
english-v3-relative-clause-adjuncts) and macro_schema_census_count_matches_21
(the live-corpus cross-check reserved for an on-demand run); neither changed.
Nightly formatting checks pass on all three changed Rust files. Citation
checks report 0 noncompliant strings and 0 stale citations; the piped diff
audit finds 0 new citation sites, so no blessing is required.


Final refresh incorporated semantics-v2-deed-performer-roles, including its
shared declaration-reader, keyword data and Lean workbench changes. English
V3 declarations, compiler and runtime code remained unchanged. The refreshed
subset tree `vyzkyyrr` (2 selected faces covered; the preserved full census is
13,596) has the same lexical inventory digest and exact Eerie/Sway counts,
54 and 4, with 0 issues. Complete corpus results are preserved because both
grammatical declarations and lexical inputs are unchanged.

Affected checks were rerun after refresh:

```text
cargo test -p deckmaste_construction_core -p deckmaste_lexical_source -p deckmaste_semantics_v2
cargo test -p xtask --test plugins_v2_declarations --test lean_check
cargo clippy -p deckmaste_construction_core -p deckmaste_lexical_source -p deckmaste_semantics_v2 -p xtask --all-targets -- -D warnings
cargo xtask english-v3 --card-name 'Sway of Illusion' --card-name 'Eerie Interlude' --workers 2 --samples-per-face 0 --output /tmp/targeting-refreshed-subset.json
```

Reader/semantics checks: 596 passed, 0 failed, 0 ignored across 30 suites.
Declaration and real Lean gate integration checks: 10 passed, 0 failed,
0 ignored across 2 suites. Refreshed strict clippy passes. The full before/after
identity comparison also completed: every surviving root identity agrees,
and every removed identity has its retained counterpart.

Scratch provenance: /tmp/targeting-before.json, /tmp/targeting-after.json,
/tmp/targeting-subset.json, /tmp/targeting-before-declarations.rs,
/tmp/targeting-before-changed-identities.jsonl,
/tmp/targeting-after-changed-identities.jsonl,
/tmp/targeting-before-identities.jsonl, /tmp/targeting-after-identities.jsonl,
/tmp/targeting-full-accounting.log, /tmp/targeting-delta.json,
/tmp/targeting-before-inventory.json, /tmp/targeting-final-inventory.json,
red/green logs and gate/citation/format logs. All diagnostic exporter code and
identity streams remain in /tmp; no process artifact entered a crate.
