---
needs: []
---
# Read *cost* with its scalar Complement and hollow infinitival: costs {1} less to cast

## Why

Cost modification never reads. On change `xxknlzypsnwy` (32,828 supported
faces, 17,322 covered, 15,506 unread; recon of 2026-10-07), the sentence shape
*X cost(s) {N} less/more to cast/activate* touches **654** unread faces and is
the sole cause on **351** (recon bucket "cost modification": 672 / 360). Counts
are unread faces *touched* (at least one localised failing unit matches) /
*sole* (every failing unit matches and no other recon STRONG bucket does). They
are surface counts, not gain forecasts.

Probes (admitted roots): "This spell costs {1} less to cast." 0; "Spells you
cast cost {1} less." 0; "This spell costs {1} more." 0; even "This spell costs
{1}." 0. The verb is declared
(`crates/deckmaste_lexical_source/lexicon/verbs.ron`, `identity: Cost`) with
one frame of three legacy role slots, `Role("ManaAmount"),
Role("ComparisonDirection"), Role("ControlledCostAction")`. That frame is the
`core-verb:Cost` row of the unsupported-inventory table in the done
`english-v3-generic-frame-consumption`, which reports it as "unsupported slot
category ManaAmount (Complement)"; no consumer admits it.

This ticket takes over from `english-v3-systemic-residuals` (a) the Cost row of
that inventory (the `ManaAmount`/`ComparisonDirection`/`ControlledCostAction`
slot categories) and (b) Neonate's Rush, the one inherited frame-coordination
face recorded there as failing on cost reduction.

## Goal

*Cost* reads with a scalar-location Complement (a mana amount, optionally with
comparative *less*/*more*) and an optional hollow *to*-infinitival Complement
whose Object Gap is the Subject (*This spell costs {1} less to cast —*). The
Cost row is replaced by ordinary typed slots the generic frame consumer admits;
its three legacy role slots are retired (Method 9). Trailing *if* and *for
each* Adjuncts compose as they do elsewhere.

## Analysis

*Cost* is one of the verbs that express location on a scale with an NP
Complement: *A jar of coffee costs $12* beside *is $12* (CGEL, Ch. 8, §5.4, p.
693, [12ib]); the same passage notes that *cost* also permits an Object for the
payer (*That jar of coffee cost me $12*). *Cost* is among the transitive verbs
that take a hollow *to*-infinitival internal Complement: *The car cost over
$1,000 to repair —* (Ch. 14, §6.3(c), p. 1250, [17ii]); the gap in the
infinitival is linked to the Subject. NPs can modify adjectives as degree
expressions, including comparatives (*a great deal smaller*; Ch. 6, §3.2, p.
549, [38]). Whether *{1} less* is a measure NP modifying comparative *less*, or
*less* heads the Complement with *{1}* as its modifier, is for the landing to
settle from the declarations and to record; CGEL does not analyse this
mana-symbol string.

## Project analysis

The mana phrase in *{1} less/more* is a degree Modifier of the comparative
Determinative; the whole phrase supplies one Scalar Location Complement.
This uses the existing Determinative identities for *less* and *more*, retaining
the independently licensed Adjective *less* for its other uses. CGEL, Ch. 13
§4.1.2, pp. 1126–1127 distinguishes comparative degree Determinatives from
adjectival *less*. CGEL, Ch. 5 §11(c), p. 431 describes degree modification
of Determinatives, and Ch. 6 §3.2, p. 549 describes measure/degree NP Modifiers
of Adjectives. Applying that distribution to the mana notation in this
Complement is this project's analysis; none of those passages analyses *{1} less*.
The plain mana phrase is also a Scalar Location Complement. The existing
CostSymbols notation supplies these measures, including generic, variable,
colored and mixed-symbol amounts; the narrower ManaPhrase notation used for
mana production is unchanged.

The optional hollow infinitival is a selected internal Complement, with its
Object Gap linked to the matrix Subject by the selected frame. It contains the
existing plain-form Object-Gap predicate; it neither supplies a purpose Adjunct
nor inserts an ordinary Object into the infinitival. CGEL, Ch. 14 §6.3(c),
p. 1250 supports the hollow Complement of *cost* and the Subject linkage.

## Witnesses

Each witness's failing units all match this construction and no other recon
STRONG bucket (`xxknlzypsnwy`):

- Wizard's Lightning: "This spell costs {2} less to cast if you control a
  Wizard."
- Ghoultree: "This spell costs {1} less to cast for each creature card in your
  graveyard."
- Herald of the Pantheon: "Enchantment spells you cast cost {1} less to cast."
- Blossoming Tortoise: "Activated abilities of lands you control cost {1} less
  to activate."
- Glowrider: "Noncreature spells cost {1} more to cast."
- Inquisitive Glimmer (no infinitival): "Unlock costs you pay cost {1} less."

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. Before: `cargo xtask english-v3 --all --workers 12 --samples-per-face 0
   --output target/english-v3/english-v3-cost-scalar-complement-before.json` on
   the claim parent, stamped with its change id and covered count; after: the
   same command to
   `target/english-v3/english-v3-cost-scalar-complement-after.json` on the
   final tree. Evidence lives under the workspace's ignored
   `target/english-v3/`, never `/tmp`.
2. Write each witness as a test first. Iterate on `--face-id` selectors; verify
   on `--all` at the end.
3. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
4. Compare per-face Reading counts against the claim-parent census. Any
   decrease on a previously covered face is restored, or named and justified in
   the landing record.
5. Spot-check at least 10 newly covered faces and list them by name with their
   Reading. The infinitival read as a purpose Adjunct with an ordinary
   (non-hollow) Object, or *{1}* read as an Object of *cast*, is a defect. A
   wrong analysis that parses is a defect, not a gain.
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
12. Tests: `cargo test -p <crate>` and clippy for every crate changed; Lean checks only if Lean sources or plugin definitions changed.

## Out of scope

- Purpose *to*-infinitival Adjuncts in general (*Pay {1} to draw a card.* 0;
  *Spend this mana only to cast …*): a separate construction, not built here.
- *costs {X} less, where X is …*: `english-v3-where-variable-clause`; the
  *where* clause is not built here.
- *the greatest/least*, *tied for*, *rather than*, *without paying its mana
  cost* (Cost Noun, not the verb).
- Comparative quantity Determiners and comparative governors:
  `english-v3-comparative-quantity-determiners`,
  `english-v3-comparative-complements`.

## Landing record

### PROVE — retained Readings and structural laws

The claim parent is `uolxmkns`. Its reused full census, measured at
`sknznupqkroutwuzmtypzzyxonswulku` / covered **21,074**, has identical parser,
lexicon, plugin and corpus sources: the only source differences are deletion
of the derived gate and its CLI/module dispatch (`baseline-source-proof.json`).
The finished feature's single full census is stamped
`nplzxtwkzkyslwmlysqrnpmmmuvnknxv` / covered **21,486**, with implementation
`zxwkulvp`. Both enumerate the same 32,828 supported face identities with
identical analyzed text, without a Reading limit.

| Census stamp / covered | Unread | Unique | Multiple | Undetermined | Complete Readings |
|---|---:|---:|---:|---:|---:|
| `sknznupq` / 21,074 | 11,754 | 7,464 | 13,610 | 0 | 685,707 |
| `nplzxtwk` / 21,486 | 11,342 | 7,472 | 14,014 | 0 | 737,600 |

**412 gained faces, zero lost faces, zero Reading-count decreases, zero
previously covered face increases.** Every formerly covered identity retains
its exact Reading count. All 51,893 added Readings belong to the gained faces.
`comparison.json` compares every face, including the previously unread ones;
there is no unexplained loss or re-coverage obligation.

Refresh imported keyword-template changes `kuwpwmvu`, `szkkroqv`, `zvrkmqpx`,
`mtkmtkxz` and their landing records, and semantics binding changes through
`ltlrpyvz`. The normalized lexical inventory is byte-identical before/after
refresh, and the 15-face witness subset has identical Reading counts and
Reading digests (`refresh-source-proof.json`, `inventory-refreshed.json`,
`witnesses-refreshed.json`). The full comparison has **zero trunk-attributed
Reading decreases**. The final refresh imported `wnnrmnpk` and `pzwktsnw`: only
xtask Lean-definition/facts checker sources changed, with no parser, lexicon or
corpus change (`final-refresh-source-proof.json`). The measured corpus numbers
therefore remain stamped to their measured tree; no post-refresh census was
run. The lexical adapter was rechecked because refresh changed
it; Lean and plugin verification were outside the user's binding scope.

Every counted Reading passes admission, lexical ownership/context, byte-exact
realization and construction/leaf traversal validation: **zero issues, zero
internal failures, zero duplicate Readings and zero cyclic derivations**.
Independent constructed values test both roundtrip laws, exact sets including
both plain and past-participial *cost*, and node/word traversal identity.
Negative values reject an ordinary Object-bearing infinitival and an unlicensed
comparative head. Valid scope Readings are retained; multiple grammatical
Readings are not destructively selected (English lexical-analysis ruling).

New word-named licensing guards: **0**. The only new condition reads declared
`MeasureModifierUse`; existing environment load-error tests pass. V3 supplies
neither a legacy coverage lock nor a legacy permitted-licensing-checker total;
those fields are unavailable, rather than a reason to run the retired gate.
The corpus command validates structure, not independent linguistic correctness;
independent-value tests and the manual spot-checks below supply that evidence.

### DISCLOSE — selected frame, analysis and route retirement

The sole Cost frame now selects an Argument with relation `Complement` and
category `ScalarLocationComplement`, followed by an Optional Argument with
relation `Complement` and category `HollowInfinitivalComplement`. Its former
`ManaAmount`, `ComparisonDirection` and `ControlledCostAction` Role slots are
all retired from the lexicon. No active V3 grammar declaration provided those
legacy categories. The replacement has one complete scalar Complement and
one optional complete hollow Complement, consumed by the generic frame reader.

The project analysis above settles *{1} less/more*: `MeasuredScalarLocation`
has a CostSymbols degree Modifier and a comparative Determinative head licensed
by `MeasureModifierUse`. Less retains its independent adjectival use; More
retains its comparative-quantity use. A plain amount uses `ScalarMeasureValue`.
`ToInfinitive` binds `BareObjectGap` in its hollow instance; the selected Cost
frame links that Object Gap to the matrix Subject. The infinitival is an
internal Complement, not a purpose Adjunct, and no mana amount is an Object
of *cast*. CGEL supports only the passages stated under Analysis and Project
analysis; applying those distributions to mana notation is project analysis.

The old separate CostSymbols/ManaPhrase constructions are retired on the
grammar side in favor of `SymbolSequence` instances retaining their different
categories. `ScalarPropertyMeasure` is retired in favor of the shared
`ScalarMeasureValue` schema, preserving its ScalarValue instance. The former
single-category ToInfinitive construction is replaced by a shared schema with
ordinary and hollow instances. No duplicate constituency route is retained.
Existing constructions' independently asserted outcomes are re-spelled.

Selection census is unique **7,464 → 7,472**, multiple **13,610 → 14,014**,
undetermined **0 → 0**, stamped by the census IDs/covered counts above.
V3 retains every Reading and emits no specificity-resolved selection tally;
no priority or specificity ordering was added.

Every newly covered identity and selected analysis is listed in the table below.
At `nplzxtwk` / covered 21,486: **C = 358** faces whose selected sample has
Cost's measured ScalarLocationComplement and selected hollow to-infinitival
with Object Gap; **A = 51** with Activate's past-participial Premodifier of
*ability/abilities*; **F = 3** with Foretell's passive predicate and *foretold*
PastParticiple. `gain-analysis-ledger.json` records each selected fingerprint,
source and scalar/hollow subtrees; `gains.json` re-enumerates this explicit
412-face selector with one sample per face and matches all full-census counts.
These samples are diagnostic choices; the full census retains all Readings.

Manual cost spot-checks (same stamp; all selected samples have the degree
Modifier and comparative Determinative, and *cast/activate* as BareObjectGap):

| Face | Readings | Selected analysis |
|---|---:|---|
| Thorn of Amethyst | 2 | C; cast Object Gap |
| Wizard's Lightning | 3 | C; existing if Adjunct |
| Inquisitive Glimmer | 32 | C for creature cost; second Unlock-cost predicate has Optional(None) |
| Sapphire Leech | 8 | C; colored {U} measure |
| Blossoming Tortoise | 420 | C; activate Object Gap and activated participial Premodifier |
| Ghoultree | 12 | C; existing for-each Adjunct |
| Neonate's Rush | 3 | C; inherited frame Coordination and cast Object Gap |
| Blood Funnel | 48 | C; cast Object Gap |
| Goblin Electromancer | 8 | C; cast Object Gap |
| Thalia, Guardian of Thraben | 2 | C; cast Object Gap |
| Herald of the Pantheon | 16 | C; cast Object Gap |
| Glowrider | 2 | C; cast Object Gap |
| Helm of Awakening | 2 | C; cast Object Gap |

Neonate's Rush is covered with three Readings; its inherited frame-coordination
residue and the unsupported Cost row taken over from `english-v3-systemic-residuals`
are resolved here. Cloud Key and Ranar the Ever-Watchful remain unread as whole
faces because of other clauses; focused tests validate the plain scalar amount
and hollow *to foretell* fragments, without claiming their whole clauses.
No new loss is routed to them. Existing residual scope stays with the live
systemic-residuals ticket and the explicitly deferred comparative/where tickets.

**Deviations and additions:**

- Consolidated the three construction families described above to avoid parallel
  routes. Added one construction, `MeasuredScalarLocation`, two selected category
  interfaces, and declared `MeasureModifierUse` rather than naming a Word in a guard.
- Added attested core Verb Foretell (including *foretold*) for the Cost hollow
  witness in Ranar; its transitive frame also admits three genuine passive faces.
  The supported corpus attests *Foretell* 51 times, *foretell* 14, *foretold* 7
  and *Foretelling* once. No unattested vocabulary was added.
- Granted existing Activate `PastParticipialPremodifier` for the required
  Blossoming Tortoise witness. This also covers the 51 A faces listed below.
- Added the attested count Noun Unlock and its NounPremodifier use for the
  required Inquisitive Glimmer witness. Its surface forms already existed with
  other owners; it is not a new keyword-specific grammar route.
- The repository formatter also reorders imports and wraps comments in six
  lexical-source files (`compound.rs`, `error.rs`, `legacy/plugins.rs`, `lib.rs`,
  `native.rs`, `supplement.rs`); those changes are formatting only and were
  included in the measured/tested tree.
- Added five integration tests in `tests/main.rs` → `tests/suite/`; re-spelled
  24 existing tests affected by shared Reading constructors. Assurance counts:
  **0 restored, 24 re-spelled, 0 ignored, 5 added, 0 removed**. Their names and
  helper dependencies are retained in `assurance.json`; no assertion was weakened.
- Added the glossary entries Hollow Infinitival and Measure Modifier Use;
  they distinguish CGEL's construction from the declared project feature. No
  other glossary gap arose and no Comprehensive Rules citation changed.

No linguistic STOP or ticket/ruling contradiction occurred. A routine stale
workspace notification was diagnosed with Harmony: all 22 edited files were
backed up under target, update-stale found it already current, hashes proved
all content unchanged, and the repair check reported no conflicts. No operation
history was rewritten.

### REPORT — inventories, validation and performance advisory

The complete census stamps above replace legacy lock numbers. Declared
construction/schema total **250 → 250** (205 constructions + 45 schemas →
202 constructions + 48 schemas); compiled productions **697 → 704**; census
observed construction names **210 → 210**. Nonblank declaration lines
**3,328 → 3,351**, disclosed as the actual change, not an authority or gate.

Named inventory evidence is `inventory-before.json` and `inventory-refreshed.json`,
stamped `sknznupq` / 21,074 and `nplzxtwk` / 21,486. Spellings **38,374 →
38,380**; homographs **1,136 → 1,139**; case-folded homographs **1,091 → 1,092**.
The complete named lists remain in those evidence files. Homograph additions:
`Unlock`, `unlock` (core-verb:Unlock / lexeme:CommonNoun/Unlock), and `foretell`
(core-verb:Foretell / lexeme:keyword_ability/foretell). Changed `Foretell` gains
core-verb:Foretell beside its catalog and keyword-ability owners. Case-folded
addition is `unlock`; changed `foretell` gains the core Verb owner. No homograph
is removed. Form-literal inventory stays **23**; form-literal/vocabulary overlap
inventory is **[] → []**. Lexical inventory SHA after is
`129686b12fba7894397f39b16fa6661f4af6cb380b119e5e63c0db8180c2b95c`.

Passed on the refreshed implementation:

- `cargo test -p deckmaste_english_v3`: **371 passed**, zero failed/ignored.
- `cargo test -p deckmaste_lexical_source`: **56 passed** (23 unit, 33 integration),
  zero failed/ignored; one test arrived with the trunk lexical-adapter change.
- `cargo clippy -p deckmaste_english_v3 -p deckmaste_lexical_source --tests -- -D warnings`:
  passed.
- Repository nightly formatting/check: passed; CR check has **0 noncompliant,
  0 stale** across 16,200 sites after refresh; actual feature diff piped into
  `cargo xtask cite audit --diff` reports zero changed citation sites.
- The only full feature census was exactly
  `cargo xtask english-v3 --all --workers 12 --samples-per-face 0 --output target/english-v3/english-v3-cost-scalar-complement/after.json`:
  complete, zero issues. Iteration and gained-face analysis used explicit selectors.
  No xtask tests, Lean checks, derived gate or second full census was run.

Performance advisory, not a correctness gate (integer ns throughout):

| Stamp / covered | Workers | Host load | Corpus wall ns | Checked-text thread CPU ns/B |
|---|---:|---|---:|---:|
| `sknznupq` / 21,074 | 6 | 15.27001953125, 14.8515625, 16.4443359375 | 255,314,203,625 | 439,930 |
| `nplzxtwk` / 21,486 | 12 | 13.8583984375, 13.33056640625, 15.5107421875 | 158,351,420,863 | 518,957 |

The finished full run exceeds the **16,260,000,000 ns** quiet-host ceiling;
this was a loaded host and different worker counts, not a quiet-host comparison.
The per-byte CPU increase is reported rather than inferred away. The all-gains
selector took **4,443,759,164 ns**, 12 workers, host load
21.572265625 / 23.7216796875 / 19.6474609375, **557,047 ns/B**, stamped
`nplzxtwk` / whole-corpus covered 21,486 (subset covered 412).
All scratch evidence remains in ignored target/; no verifier or snapshots are
embedded in tracked source.

### Newly covered identity list

All rows are stamped `nplzxtwk` / whole-corpus covered 21,486; C/A/F are the
selected analyses defined above.

| Face identity | Readings | Selected analysis |
|---|---:|---|
| Aang, Master of Elements (`b4872bac-5822-4c35-9b73-38c4e3ffa477#face:1`) | 8 | C |
| Academy Journeymage (`b400700c-1d82-4721-a166-56f88ba6ad19#card`) | 6 | C |
| Academy Loremaster (`7d540860-ae94-4773-a526-aaffca3eb057#card`) | 8 | C |
| Accursed Witch (`80b07882-e144-4815-8b6f-04b3ab343d97#face:0`) | 60 | C |
| Ajani's Response (`bf7ec320-64bb-46ca-aac0-ca3d4224b69e#card`) | 3 | C |
| Alabaster Leech (`8a8b032e-90b7-4782-b854-b575f316d641#card`) | 8 | C |
| Ancient Stone Idol (`5f981cca-5cd9-49e4-ab1a-bbbf6fc7e737#card`) | 27 | C |
| Andradite Leech (`09bfc3b1-cc40-4cb9-87e4-86b0018caa75#card`) | 16 | C |
| Animar, Soul of Elements (`725880b2-1675-414f-b61b-cf6533797dbf#card`) | 144 | C |
| Arachnus Web (`8b25f353-0798-4a03-a928-6a701a919468#card`) | 3 | A |
| Arcane Epiphany (`fe1556cc-32d0-4e51-bb15-f3fce669a5e1#card`) | 3 | C |
| Arcane Melee (`9967b292-1239-4aed-b871-72e40d1f3e26#card`) | 2 | C |
| Archmage of Runes (`28045b32-4c1a-40e5-a15d-524d0f8fe6ec#card`) | 32 | C |
| Armor Wars (`1ad83cac-2c85-477b-ab00-1369f9683db5#card`) | 200 | C |
| Arrest (`81728b98-8cf9-4734-a318-69184bb4d15c#card`) | 3 | A |
| Arwen's Gift (`13ac12db-d04b-4211-90df-886e1c597e8f#card`) | 6 | C |
| Aura of Silence (`e7faf8eb-e829-4109-8dfe-42865a23ba86#card`) | 8 | C |
| Avatar of Fury (`4854f836-2247-43c3-adbc-1d1ee5eaf120#card`) | 2 | C |
| Avatar of Growth (`07db0374-3297-49c3-886d-a6bb42f7bb18#card`) | 9 | C |
| Avenge (`65a64759-3a93-4542-84fc-3af5ccb741f4#card`) | 27 | C |
| Awaken the Blood Avatar (`0b299983-9f0f-404a-acd1-8f142572b1f1#face:1`) | 30 | C |
| Ayesha Tanaka (`35e4ed0e-8ce2-4729-a009-0d6be8f47663#card`) | 7 | A |
| Azorius Guildmage (`16f61f69-cdde-4b6c-b855-c74db9038ceb#card`) | 1 | A |
| Balamb Garden, SeeD Academy (`8b84fec5-617c-4088-8250-2ba1f1f9479a#face:0`) | 6 | C |
| Ballyrush Banneret (`0ad27d0e-b6c3-4c3f-a389-285d889b6a65#card`) | 36 | C |
| Banish from Edoras (`e4508afd-7cc3-4176-8291-cfc2c155ac11#card`) | 3 | C |
| Baral, Chief of Compliance (`baf80db7-2ca8-4bfc-b5b6-25ed51878dd7#card`) | 16 | C |
| Baru, Wurmspeaker (`1cf27d78-d6ae-4fa2-9c8d-c277913f821c#card`) | 10 | C |
| Battlefield Butcher (`2978ba77-c0ce-4411-99c7-ef05dd602c8d#card`) | 12 | C |
| Battlefield Thaumaturge (`70d2f5f7-f727-48e1-9532-8d365b1c1ae5#card`) | 360 | C |
| Bedlam Reveler (`01a7e0ce-c98f-4cf2-aba3-360aa0a9b9a7#card`) | 81 | C |
| Bind (`2b9557df-7158-4ada-be77-5c346851a568#card`) | 1 | A |
| Bitter Downfall (`19c22e08-29b6-4b3d-bf6e-28a5add620bb#card`) | 12 | C |
| Bladegraft Aspirant (`644c785d-3061-44bc-9f78-bc939c565bf4#card`) | 144 | C |
| Blasphemous Act (`7a2484a9-04fd-41a0-8224-610c1c07ed10#card`) | 9 | C |
| Blood Funnel (`81dd22f7-f6c6-4cf2-9744-b1d918dbcf9a#card`) | 48 | C |
| Blossoming Tortoise (`5e1bf23b-7fb0-45ff-8544-fce9fa3eba00#card`) | 420 | C |
| Bolt Bend (`c20a96f7-aa5a-4c15-b8b1-806685c99b27#card`) | 108 | C |
| Bone Picker (`2edf106d-f504-430e-ad9c-50fc95b0ad9c#card`) | 6 | C |
| Bontu's Monument (`940ba435-7abc-40f8-a5af-c1c653b3284e#card`) | 48 | C |
| Boreal Elemental (`68a2e371-ccf0-4c2a-aeef-c66852811e1a#card`) | 4 | C |
| Boseiju, Who Endures (`bf1341dd-41a3-49f6-87ec-63170dde4324#card`) | 1188 | C |
| Bosk Banneret (`b84dedfd-4020-444f-ac5a-54c65c0d0378#card`) | 36 | C |
| Bouncer's Beatdown (`36ac8fc6-98ad-499b-9adf-046433e2c122#card`) | 90 | C |
| Brighthearth Banneret (`04120095-74a5-4284-bfba-aeb0b509dc84#card`) | 36 | C |
| Brown Ouphe (`5b42e91a-e3bd-4f69-9abf-ed6273715fff#card`) | 4 | A |
| Brush Off (`873694c0-c100-4d24-a74f-1ab26442dd18#card`) | 6 | C |
| Bury in Books (`d5e67016-1740-42f8-8be2-9eb9d4adb142#card`) | 12 | C |
| Cactarantula (`18a5fe34-0c97-486c-b09a-80535a99e982#card`) | 15 | C |
| Case of the Ransacked Lab (`f85eea8f-5887-4921-a943-d15e0ad1e028#card`) | 128 | C |
| Cavern-Hoard Dragon (`939e8448-013a-4aef-9a92-7c883a77a1a0#card`) | 120 | C |
| Centaur Omenreader (`76b65b2a-9905-4857-806a-6dbcf195fc81#card`) | 16 | C |
| Charix, the Raging Isle (`c38cc5fe-936a-4d61-a816-9be773ae1d02#card`) | 16 | C |
| Chill (`70a93d0f-9a02-4e60-a2b8-827fd2410604#card`) | 2 | C |
| Chill of the Grave (`2ee4f7ba-7238-44b5-8315-9f50502ed796#card`) | 18 | C |
| Chitin Gravestalker (`acac15c8-877d-49d4-9b42-90181c05a684#card`) | 27 | C |
| Cid, Freeflier Pilot (`d3bdfb25-dfb2-4df4-b502-ef0909a39f86#card`) | 336 | C |
| Cinderslash Ravager (`a1c6bd47-19ef-44f9-921a-e4ffa80b25e1#card`) | 43 | C |
| Clarion Conqueror (`f1266a1b-bae6-4ded-b4d5-7586a5b6c5d0#card`) | 9 | A |
| Closing Statement (`79ac7b6a-1c25-45f6-84b4-5c4a78d6fd6e#card`) | 12 | C |
| Collector Ouphe (`0c4bc9ea-a5fd-4f44-96a1-5448eee228c4#card`) | 2 | A |
| Conduit of Ruin (`cd3d79e9-c72e-4321-bca1-54505d51331a#card`) | 10104 | C |
| Count Nefaria (`5ec4f81e-ede4-4d45-b768-16583f6f5b8d#card`) | 9 | C |
| Cryptic Serpent (`0f26c04a-9bf7-4cbd-a95f-73ee4a3b1af1#card`) | 27 | C |
| Crystallized Serah (`a8a7a3b7-015c-4fbd-8b90-e0ddf71663e4#face:1`) | 32 | C |
| Cunning Nightbonder (`7351317d-62a1-4894-9811-aa950c49eece#card`) | 4 | C |
| Cursed Totem (`6225a704-430a-4f56-ad87-0e8d87f285f5#card`) | 2 | A |
| Cyan, Vengeful Samurai (`5a2fa420-c27f-4c14-b285-e0bb2828e053#card`) | 12 | C |
| Danitha Capashen, Paragon (`4b6377da-83e7-4519-9582-16a9c16b8faa#card`) | 8 | C |
| Dark Endurance (`ded4009d-be23-4e36-a7a3-9e07230712ee#card`) | 9 | C |
| Dark Impostor (`96e25887-e556-423a-8a69-0cd148216f6a#card`) | 92 | A |
| Daru Warchief (`3fce53fb-6914-4f43-b46b-18ca32ab21b2#card`) | 16 | C |
| Death-Rattle Oni (`bea40c67-73ca-4b5e-b902-5de07f21585c#card`) | 36 | C |
| Deepwood Denizen (`4a6887e3-7a65-4f24-be71-183b00a0ad56#card`) | 12 | C |
| Delightful Discovery (`705cc928-d6d4-4294-979b-a385d1b07dc6#card`) | 12 | C |
| Demotion (`c333eb79-300e-4dc0-9126-8dca1bac6ef2#card`) | 1 | A |
| Depower (`33f89437-885b-4850-88b7-239f5dfcc8b1#card`) | 6 | C |
| Derelor (`43f2969e-46eb-4fd4-8b2d-ff81aafcc24a#card`) | 8 | C |
| Detainment Spell (`b46526af-6b88-42c4-b317-4b499f151ff0#card`) | 1 | A |
| Diary of Dreams (`98b43c61-4563-4a29-8cf0-fb8b34f337ce#card`) | 36 | C |
| Diregraf Rebirth (`1920254f-0f65-4426-aabc-f3807476b67b#card`) | 30 | C |
| Distorted Curiosity (`6ebbf025-2996-46f2-ac23-992909251525#card`) | 3 | C |
| Draconic Lore (`440881ad-48cf-4b82-8bb5-f506d1a51755#card`) | 3 | C |
| Dragon's Prey (`86cb340f-ef06-4be9-b457-703784387dbe#card`) | 3 | C |
| Dragonlord's Servant (`faf81ae0-0b35-45ef-a50d-6cecf0c0eeeb#card`) | 8 | C |
| Dragonspeaker Shaman (`6a29f36b-7911-4d84-a3ae-546b131d5159#card`) | 8 | C |
| Dreaded Bat-Cloud (`3cc64ce6-45e2-4dc3-9b28-d7206a131873#card`) | 6 | C |
| Dream Chisel (`7776c114-c02a-4694-9b9d-00fbfad2d2a2#card`) | 12 | C |
| Earthquake Dragon (`1903200d-0350-48a9-ac7c-b851f7df647f#card`) | 15 | C |
| Eiganjo, Seat of the Empire (`7edb3d15-4f70-4ebe-8c5e-caf6a225076d#card`) | 12 | C |
| Elderwood Scion (`99c8647f-e08d-4217-b637-3f8f2bffdaa3#card`) | 16 | C |
| Elspeth Conquers Death (`40c88392-2aa7-4e12-86a4-8d680219e670#card`) | 1152 | C |
| Embalmer's Tools (`5bb4ee32-11c6-4ed4-a071-c3cf48070fff#card`) | 28 | C |
| Embercleave (`4d6120d6-fcce-40bc-9fc6-e1f5beb6c728#card`) | 6 | C |
| Emerald Medallion (`d683024d-8fe7-4509-851b-7655fadc1de7#card`) | 8 | C |
| Encrust (`2e056743-78ff-45b6-80df-7486d52d5df6#card`) | 6 | A |
| Enthusiastic Mechanaut (`c9722086-23e8-43dc-8a8c-541ff6962318#card`) | 8 | C |
| Ephara's Dispersal (`1e0bb57a-f5c7-45e9-b5b9-5bb3ec930108#card`) | 6 | C |
| Eris, Roar of the Storm (`918991a6-7321-419d-8f3d-150fe32d02d2#card`) | 1188 | C |
| Ertai's Scorn (`76e6c0e7-593a-4b4a-9eb6-a912f9c3f26f#card`) | 6 | C |
| Esquire of the King (`3ccaa78e-f3e5-4de4-b53c-cecd73a73b9c#card`) | 6 | C |
| Etherium Sculptor (`96b87445-6362-4a17-91b2-cbf203fd03fd#card`) | 8 | C |
| Even the Score (`25f5cca6-a414-46d9-82e3-ce2fa21b14ca#card`) | 9 | C |
| Experiment Kraj (`93cfa771-067f-44ed-9e29-6caf698ee9aa#card`) | 61 | A |
| Explosive Singularity (`30d79f61-f9d9-441a-b600-83721af8553b#card`) | 18 | C |
| Eye of Ugin (`10d13ff6-c4d0-4753-8939-a8a90f0e92bb#card`) | 84 | C |
| Ezzaroot Channeler (`3e72ec1f-9941-42e6-b832-58b79b44066b#card`) | 32 | C |
| Fantastic Bounce (`71d09e00-e526-4899-beb3-0ee6d9b87f44#card`) | 6 | C |
| Fast Forward (`c92f78c4-c8ec-4bdb-85f4-3a206b709f0f#card`) | 6 | C |
| Fate of the Sun-Cryst (`0ffeec93-89d7-42c2-8abf-802e9f6dc992#card`) | 3 | C |
| Feroz's Ban (`445dd350-5165-4516-8669-9233f5558605#card`) | 2 | C |
| Fluctuator (`5ed0c7cc-9969-4387-90f8-3084dd18026b#card`) | 4 | C |
| Flying Drone (`541f7525-a9a4-493d-b724-354ae6d37de5#card`) | 32 | C |
| Focus the Mind (`695a84b5-96b0-4e48-b615-15624efab3b7#card`) | 18 | C |
| Foundry Inspector (`18f22960-87ec-43cd-82ea-ec5cabf49ad3#card`) | 8 | C |
| Frogtosser Banneret (`91ebedb5-cb5f-4c58-ad30-6c6b0a106aa0#card`) | 36 | C |
| Gargantuan Leech (`073f12d1-eaa1-4d08-bc1d-b216a3dc0c67#card`) | 12 | C |
| Gargos, Vicious Watcher (`378fcd0f-1096-4769-9149-55b4a889ff56#card`) | 16 | C |
| Gathering Stone (`9f6d6e90-7462-46e5-bf2f-725b4badc394#card`) | 40 | C |
| Geist of Saint Thalia (`ef32a4a9-14e2-4738-b4c2-53ce5e1d2a53#card`) | 8 | C |
| Geist-Fueled Scarecrow (`b925c4ed-8767-46b8-a423-a5f094557d94#card`) | 8 | C |
| Gelid Shackles (`45ca8431-befd-4542-9dd7-503bf1afaac5#card`) | 2 | A |
| Ghalta the Unstoppable (`8e5ea773-a60b-4502-9db5-ce93fa61cc89#card`) | 10 | C |
| Ghalta, Primal Hunger (`b0b6be0c-41cf-4757-9f0e-87227b6ba6b3#card`) | 5 | C |
| Ghostfire Slice (`986a7ad0-527a-4079-aa1b-c8acdf087020#card`) | 3 | C |
| Ghoultree (`5f2628e3-e29a-4d77-bc27-6e1ce240d192#card`) | 12 | C |
| Gigastorm Titan (`ef06e10c-553b-45b8-abf0-1d2e87c22128#card`) | 9 | C |
| Glamdring, Foe-hammer (`2802069f-201e-43a7-b5d3-43a95951a2ec#face:0`) | 8 | C |
| Gloom (`4d022f53-b1fb-4071-afcc-0af3214fe604#card`) | 8 | C |
| Glowrider (`aaf825b0-af0f-47a0-8b00-85f0614cef5d#card`) | 2 | C |
| Gnoll War Band (`ff8fc718-6218-48f3-981d-c4d2bc468493#card`) | 9 | C |
| Goblin Anarchomancer (`e9319f13-1f2b-429c-9d61-e58a3cbec86a#card`) | 2 | C |
| Goblin Electromancer (`81f06f84-1580-43c0-89d5-08d34541a519#card`) | 8 | C |
| Goblin Maskmaker (`ff4b9723-d5f1-40d2-aa35-fa3d2a6f4b5c#card`) | 8 | C |
| Goblin Warchief (`39882df0-c20f-469d-94ba-1617224e71a1#card`) | 8 | C |
| God-Pharaoh's Statue (`183954d2-7b40-4c7e-adfa-d80191ab5ad6#card`) | 4 | C |
| Goreclaw, Terror of Qal Sisma (`befb211f-37ca-4083-98d4-9ff1f28be3f2#card`) | 160 | C |
| Grand Arbiter Augustin IV (`1f8d4d5f-e82f-45f3-823e-1bb6b536eb18#card`) | 256 | C |
| Gravebreaker Lamia (`2c1d288a-22ea-482c-947e-f3ab04172da0#card`) | 96 | C |
| Grounded for Life (`9a334434-072a-4486-810a-63efb29dea6d#card`) | 3 | C |
| Grow Extra Arms (`52f03e73-3dba-44c2-b256-10e7e208b961#card`) | 6 | C |
| Gust of Wind (`7cfc14ce-9940-4c61-9daa-2c6dbf1a80ad#card`) | 48 | C |
| Gwaihir the Windlord (`b967fbb8-ddc8-485b-bd18-cbf3ef6b1bf8#card`) | 18 | C |
| Hagra Mauling (`37783ce6-af58-4ef6-8ab4-587079970307#face:0`) | 3 | C |
| Hamza, Guardian of Arashin (`0cde6a29-d597-4feb-8b2b-528d2d57a1f3#card`) | 14792 | C |
| Hardened Berserker (`2894efc6-ee78-4353-b98a-8005a2451139#card`) | 4 | C |
| Haughty Djinn (`44036bb5-f300-40bd-9180-6b246315d886#card`) | 40 | C |
| Haunting Voyage (`334c284e-7bc3-4c18-9f8d-4d846f80008e#card`) | 432 | F |
| Havengul Lich (`1fb02bad-0d1f-437f-a269-f51cc19b280f#card`) | 154 | A |
| Hawkeye's Shot (`6bbd1316-02d3-46ab-ba1a-0ecab51ec4c4#card`) | 3 | C |
| Hazel's Brewmaster (`e8180024-1979-4677-9a0d-e08d4b7c825a#card`) | 552 | A |
| Hazoret's Monument (`f02a4cbf-f0bb-4bc7-99c2-6afa1b41ebb8#card`) | 24 | C |
| Heartless Summoning (`fa1be67e-a07e-42ce-88be-446acd643dc6#card`) | 8 | C |
| Helm of Awakening (`f2fcef65-4873-4b1f-a257-b8648097c990#card`) | 2 | C |
| Herald of Kozilek (`e1890cbc-b15b-44bc-b1c0-ca3d9fd6028a#card`) | 8 | C |
| Herald of Slaanesh (`ea912e4f-c734-4c7b-9e80-8961ca57cdd3#card`) | 16 | C |
| Herald of the Pantheon (`9ef24a79-fade-4c38-b63c-22983206b4ad#card`) | 16 | C |
| Herald of War (`1228bd6f-565d-4ad5-bd24-e5aa1ead5dee#card`) | 324 | C |
| Herald's Horn (`c02c5547-b9c9-4b2d-9d12-e87bfba8f2d2#card`) | 240 | C |
| Hero of Iroas (`0d44bd43-77da-449e-b771-8a14da88b677#card`) | 32 | C |
| Heroic Return (`4b7a8c05-15cd-452e-b9fc-83800e81793d#card`) | 360 | C |
| High Seas (`9100f213-f4c1-4b15-94f2-e280579894c0#card`) | 8 | C |
| Highspire Bell-Ringer (`6d2d250b-cdce-4075-8724-b52ad31664fb#card`) | 8 | C |
| Hinata, Dawn-Crowned (`aee473c7-c659-421c-96b5-05f57ca2ed97#card`) | 144 | C |
| Hold for Questioning (`1095fcd9-8127-4345-9052-fccb350296d3#card`) | 18 | A |
| Honest Rutstein (`439bf6d1-bd65-474b-a70c-7fdd446de4bb#card`) | 40 | C |
| Hum of the Radix (`95803a5a-9f98-4764-b6bd-6cf4cdcb740b#card`) | 3 | C |
| Hunter's Mark (`7def9bfe-6a59-4300-8134-fce7508dca38#card`) | 144 | C |
| Hylda's Crown of Winter (`a8291d04-240e-456b-8c86-1fb4eb460476#card`) | 21 | C |
| Ice Cage (`e37416cc-f343-44cd-8676-4ce4d4068e85#card`) | 6 | A |
| Icefall Regent (`0cf18002-1bbe-40b3-ae15-fa51e61e9391#card`) | 48 | C |
| Infectious Curse (`80b07882-e144-4815-8b6f-04b3ab343d97#face:1`) | 16 | C |
| Inquisitive Glimmer (`4abb227a-534e-4300-9009-065a10f10e8a#card`) | 32 | C |
| Interdict (`93771480-2672-4bc7-81a8-011d3b8878b0#card`) | 21 | A |
| Into the Story (`f290c2e4-ab52-44d4-bdeb-31aeb835b18d#card`) | 12 | C |
| Invasion of the Giants (`807e35c0-8ea1-44a5-abac-51fd516aad03#card`) | 36 | C |
| Irini Sengir (`2727a7ec-1cf7-4cd7-b5d4-955db940ddd9#card`) | 8 | C |
| Iron Lad, Young Avenger (`fedcc722-ac5f-464a-bac9-c5f1d4de28d0#card`) | 8 | C |
| It That Heralds the End (`c14ac951-329d-452b-be75-5c72166cd9c7#card`) | 60 | C |
| Jace's Sanctum (`22d7048e-d538-4fd3-9fb3-2e99e74875fe#card`) | 32 | C |
| Jade Leech (`495e08a8-e654-4a0b-a25f-1e344ae2a0fa#card`) | 8 | C |
| Jet Medallion (`bccb06f1-8839-4104-9ffa-c79a817f8378#card`) | 8 | C |
| Jodah's Codex (`1ba5b108-59cb-4af1-88cb-d3ceaaf00784#card`) | 54 | C |
| Jubilant Skybonder (`aab55af7-8084-47b6-910f-75a75a6313a4#card`) | 8 | C |
| Jukai Naturalist (`c4d1a78f-0cca-4042-862c-8f322ee25d50#card`) | 8 | C |
| Kadena, Slinking Sorcerer (`4da13eb8-ca18-43f1-b54b-d3ddceb980d2#card`) | 32 | C |
| Kaervek's Torch (`c629feae-084c-452f-9c09-4b9716331498#card`) | 2 | C |
| Kaza, Roil Chaser (`0b8517c0-0441-4fd9-99ec-2274e57c3cb6#card`) | 80 | C |
| Kefnet's Monument (`b6294891-79e6-4f2a-a82d-6cffce968356#card`) | 192 | C |
| Kethis, the Hidden Hand (`1bf08ba0-4235-4dea-9722-ed991a3b6044#card`) | 192 | C |
| Khalni Hydra (`d4649dd8-9a30-4f7e-8b9d-8edee8b2dbd7#card`) | 6 | C |
| Killian, Ink Duelist (`e233cb5b-7237-4271-8157-9179cc897b6c#card`) | 4 | C |
| Kinjalli's Caller (`bba6075b-8bbb-4496-80a9-bfa7f003d1e8#card`) | 8 | C |
| Knight of the Stampede (`801d60c1-40b1-48b6-8800-afe1a21386f3#card`) | 8 | C |
| Knockout Blow (`7068637c-b025-41a5-be31-df1850f33b68#card`) | 3 | C |
| Kopala, Warden of Waves (`0237cccc-3161-4d91-8e69-e3d315dcb246#card`) | 32 | C |
| Krasis Incubation (`66e2e816-5197-4151-a6c0-352c34c2eec7#card`) | 6 | A |
| Krosan Drover (`cd8ae400-4f40-42aa-a905-2787de443471#card`) | 20 | C |
| Krosan Warchief (`88350861-244c-4634-9bc9-e9a79401bd02#card`) | 8 | C |
| Lashwhip Predator (`56035e66-e730-436d-8de2-4ded7fd1b51e#card`) | 3 | C |
| Lawmage's Binding (`10f0e797-3887-4c2b-9cff-80f110b952e1#card`) | 3 | A |
| Leyline Binding (`b49642f6-20bd-468d-9dac-e578f9ac3091#card`) | 594 | C |
| Linvala, Keeper of Silence (`88dadc31-dfac-41b3-bf2a-65fa89e3c16d#card`) | 5 | A |
| Lodestone Golem (`a698c9f5-325a-49f6-8902-eef09f4260c5#card`) | 2 | C |
| Long-Forgotten Gohei (`897910d5-13a4-4e1d-8f83-235b773b8322#card`) | 16 | C |
| Longshot, Rebel Bowman (`f93c3468-5b1d-4fa7-a281-1009ea0f2062#card`) | 16 | C |
| Lookout's Dispersal (`e2e44d46-9df4-46f8-acf7-5581b2257620#card`) | 6 | C |
| Loreseeker's Stone (`28a21e56-b97f-487d-9a89-61c248192cec#card`) | 9 | C |
| Lumbering Megasloth (`6492c381-0fa1-4d43-bf72-d8956d8a34b6#card`) | 9 | C |
| Luminous Rebuke (`d96141af-313f-4ea9-a249-d9eb22cd2431#card`) | 3 | C |
| Lyse Hext (`47bce63e-8edb-43c1-b273-4b86da5c616b#card`) | 16 | C |
| Machine Over Matter (`37069c66-c17a-40ac-80f4-14ef49d18008#card`) | 6 | C |
| Maelstrom Muse (`30b03dda-409f-4615-b392-477d66eb49f4#card`) | 20 | C |
| Mage Duel (`763882f9-a7ad-4fb1-bdf4-c9a158d4b68e#card`) | 36 | C |
| Magnificent End (`82bdf9cb-61fe-4eaf-8762-c32002403d96#card`) | 3 | C |
| Magnus the Red (`86c9b01e-18a2-4616-8e04-5343f97d2efe#card`) | 144 | C |
| Mana Matrix (`256cea34-4691-4367-a789-17d15460f664#card`) | 8 | C |
| Marauding Raptor (`b3fe2f61-c58e-4ebc-a5e3-99d6b6f62caf#card`) | 48 | C |
| March of Otherworldly Light (`ed06e421-480c-40c8-8c3e-1e466e5327b2#card`) | 168 | C |
| March of Wretched Sorrow (`6e638332-fba1-4e91-9237-fdff23826640#card`) | 42 | C |
| Mascot Interception (`ae3da0be-2a3f-498a-8854-73d0a5526ec2#card`) | 36 | C |
| Mental Modulation (`448d9391-9d15-4aa1-87b3-a1d755666c03#card`) | 3 | C |
| Metalwork Colossus (`64ffe529-ba53-48ec-8af1-b046fbd1f0cc#card`) | 21 | C |
| Mindsplice Apparatus (`4d0760fb-4e04-4f29-a268-14c13c30232d#card`) | 72 | C |
| Mirran Safehouse (`08826de5-a2d7-4732-a5f7-f580507a7844#card`) | 18 | A |
| Mirror of Galadriel (`7695ef41-fa58-4890-b3fe-de5751831d8e#card`) | 12 | C |
| Mistform Warchief (`97f1ac0d-a3a6-46d8-ae99-b17b636fe47c#card`) | 576 | C |
| Mistmeadow Council (`9acfec0d-05a0-4b28-a883-e6863f1cb773#card`) | 3 | C |
| Mocking Sprite (`013ca531-1ff7-4efc-a280-4bc3ab574072#card`) | 8 | C |
| Molderhulk (`a6fa840a-817c-4e84-9dd3-d28a011f2214#card`) | 120 | C |
| Molten Monstrosity (`ea55886c-48a1-4339-81fd-03dbaa116c28#card`) | 5 | C |
| Momo, Friendly Flier (`a3c56e71-0de6-447a-a9b4-0971f56c2362#card`) | 720 | C |
| Moon-Boy, Dino Rider (`bb7638ce-3802-4198-89d2-6f5b647bd8fb#card`) | 24 | C |
| Mortality Spear (`544acde0-850d-4c4f-8389-e22930d87345#card`) | 6 | C |
| Mutagen Man, Living Ooze (`472d9a18-19a0-46c0-a4bf-328ebec7ce41#card`) | 14 | C |
| Myr Welder (`ea2cbf5a-2def-450b-9db8-40a5b6a604d0#card`) | 224 | A |
| Mystical Dispute (`48390b8d-db41-48a5-9bac-316660ab7252#card`) | 6 | C |
| Nahiri's Binding (`8f674382-78b5-462c-9a7f-c5f613c317d4#card`) | 3 | A |
| Necrotic Ooze (`2fdeb920-2e48-4308-9517-743eec219c04#card`) | 18 | A |
| Nemesis of Mortals (`599fc217-e0a3-45ee-b9aa-cd7f181dcdf3#card`) | 144 | C |
| Neonate's Rush (`721aeedc-1de5-40d0-a04c-a2ebc056d06a#card`) | 3 | C |
| Nightscape Familiar (`57296ea3-3c0d-49b7-bc08-0d0d8414e9ad#card`) | 36 | C |
| No One Left Behind (`afe8fd01-f0c7-46a5-9e9c-e8b9599cd19a#card`) | 75 | C |
| Not of This World (`5da5d8a3-009e-4d8a-977e-6ebc73ba2682#card`) | 465 | C |
| Null Rod (`2f83ca86-e23d-40f7-8085-6928d8cfef9b#card`) | 2 | A |
| Nyxborn Behemoth (`58e20ca7-5fe3-49ff-bee0-0388722e3cab#card`) | 14 | C |
| Oakhame Adversary (`3afe4852-d506-45d7-8a19-ddcbc3dd90d4#card`) | 9 | C |
| Oath of Ajani (`1ec39c8f-eb1e-4881-a749-0c188788b612#card`) | 8 | C |
| Of One Mind (`5a23c734-cbfb-419a-9bf1-126cb07fe427#card`) | 3 | C |
| Oketra's Monument (`0370afa0-07d3-4787-8a5b-10272cb3a486#card`) | 192 | C |
| One Thousand Lashes (`d3b259e9-f9c7-4830-91fa-6e3d9f4cc1e3#card`) | 15 | A |
| Oppressive Rays (`4a17fb46-70db-4c9a-ac0e-dfc774345673#card`) | 28 | C |
| Ore-Scale Guardian (`c4224ae5-4807-45cb-bd4b-70f2c0d9d9f4#card`) | 12 | C |
| Oskar, Rubbish Reclaimer (`14c2c818-0ce2-4809-a5ff-ee4a6253defa#card`) | 108 | C |
| Otawara, Soaring City (`e9b6a394-691c-425a-9307-76d8edc7375e#card`) | 24 | C |
| Otepec Huntmaster (`41623eab-38b5-4058-8de2-e6b8706de7a2#card`) | 16 | C |
| Ouphe Vandals (`7bee3848-24e2-4e12-a341-a759214308c0#card`) | 21 | A |
| Out of Air (`fcea912b-58b1-4068-ac4d-1bc89aa7f33c#card`) | 3 | C |
| Out of the Way (`9300874a-32a9-4869-b3e1-1b996403df4e#card`) | 12 | C |
| Overwhelming Remorse (`bb4507c1-1e5d-48b5-8fc0-1d26bd1c9e72#card`) | 12 | C |
| Patchwork Crawler (`74cbe86b-cc8c-4a5f-93c3-f4eaced50393#card`) | 322 | A |
| Patrician Geist (`1f65c853-5262-4b75-b324-1fe0c165170d#card`) | 16 | C |
| Pearl Medallion (`19015380-1332-4960-8cc6-0732009525a2#card`) | 8 | C |
| Pearl of Wisdom (`91ddd2bd-2dec-4670-8edb-e89c009b2187#card`) | 3 | C |
| Peerless Samurai (`fe3b0393-dc50-4fa6-9b3f-1175a6a825e4#card`) | 8 | C |
| Petrify (`ebec110e-aff3-48aa-abf6-309dd2ffb478#card`) | 3 | A |
| Planar Disruption (`6128679e-7e54-41e4-ab6e-25aeeffed49f#card`) | 3 | A |
| Planar Gate (`632f1034-d2ad-432a-a79a-4bd1307be24a#card`) | 8 | C |
| Poison the Cup (`58020778-554c-4af6-8484-0b90f64bc5d9#card`) | 1 | F |
| Prehistoric Turtlesaurus (`e00aa7d7-3c11-463d-a7f5-8d001ea5c021#card`) | 43 | C |
| Price of Fame (`45d5cd4c-7285-4507-8cbf-eace7a734f41#card`) | 3 | C |
| Primeval Protector (`a9581345-87dc-4fbd-acc1-bb629f2cf40f#card`) | 6 | C |
| Prison Sentence (`6d9f2fe0-5657-45b4-9431-36e0f6c8eba2#card`) | 3 | A |
| Prison Term (`31caa417-12f7-468b-9f3a-ccfb80e5bc2f#card`) | 3 | A |
| Pteramander (`00187de2-bc48-4137-97d8-a9a0fafc76c1#card`) | 27 | C |
| Purple Worm (`179cc807-5150-4513-88b2-73cb05009b7d#card`) | 6 | C |
| Qiqirn Merchant (`560cb970-1c1b-4ba7-8352-bc67b4ea7843#card`) | 6 | C |
| Quicksand Whirlpool (`43fb0f35-8eea-4344-9415-6ecb8aa46491#card`) | 3 | C |
| Racers' Scoreboard (`54c2b3b3-e247-4146-8045-d49480b795f2#card`) | 24 | C |
| Raft Security Officer (`60754d4b-2add-4928-8d30-8ce9c0f54f91#card`) | 12 | C |
| Raging Battle Mouse (`9ffe4d90-b46c-4606-89cb-1b724104f6ea#card`) | 128 | C |
| Razorlash Transmogrant (`580996d4-db9a-4b57-a0e1-a2dbb524121a#card`) | 81 | C |
| Reidane, God of the Worthy (`35ae7b25-6d0f-49f8-a40d-dd2cf88e6f44#face:0`) | 40 | C |
| Reroute (`997f5a5a-9ee1-4b66-9ffc-d1a2e996e615#card`) | 5 | A |
| Revoke Demise (`83f0b573-58a0-457d-80e5-6e0b690e5f4e#card`) | 75 | C |
| Rhonas's Monument (`b9a7c722-6b24-4383-ac6e-2a30f7834269#card`) | 96 | C |
| Ride's End (`89e209f3-c474-453f-9659-ccebfc88bc0f#card`) | 3 | C |
| Rime Chill (`bad84deb-3f2c-46fd-9dcb-4e3ba9732769#card`) | 12 | C |
| Roadside Blowout (`03008c7d-c738-473d-a968-1922b2899d1a#card`) | 48 | C |
| Robaran Mercenaries (`1d8c8c71-d569-450d-adc1-1a1b9aea0123#card`) | 9 | A |
| Rowdy Research (`1a33a5c2-5aa1-48e5-95a8-d52cdce980de#card`) | 9 | C |
| Ruby Leech (`ec6195a0-2701-40a4-882f-eb563668791a#card`) | 8 | C |
| Ruby Medallion (`0c367958-729e-416d-988b-098e90e7a1fd#card`) | 8 | C |
| Rumbleweed (`9cfc01a7-e291-4388-bd09-94846d72e291#card`) | 264 | C |
| Run Behind (`7578a192-82e5-43ff-8dee-225f313c484a#card`) | 15 | C |
| Run Over (`2b1bec38-5aaf-4ca0-9a04-fb4367c59c1e#card`) | 36 | C |
| Rust (`597ca66c-6bb1-4661-8ed9-8236f546e536#card`) | 4 | A |
| Samut, the Driving Force (`ee89c719-5ae8-4316-bfd0-cfe3684b5859#card`) | 64 | C |
| Sanctum of Tranquil Light (`1818b0b1-0104-4673-9be8-acfc773ef87d#card`) | 3 | C |
| Sapphire Leech (`500aff17-0d69-46ac-a4f9-2fc6db061ce7#card`) | 8 | C |
| Sapphire Medallion (`f1ef5698-14bd-42bf-aae5-2870699e4186#card`) | 8 | C |
| Savage Stomp (`2ecda58d-9aa9-4857-bdc5-b6b7b70a79c0#card`) | 3 | C |
| Saved by the Shell (`7ef5c812-ad7d-4d6c-8ee5-68561e7e4bd7#card`) | 6 | C |
| Search Party Captain (`8721b0a6-52ff-4749-92db-82bb36f5a73e#card`) | 12 | C |
| Secure Detention (`ce95e51d-4ef3-44a8-ae47-cf1bf71ebd3f#card`) | 3 | A |
| Seized from Slumber (`7ac608bc-dc2a-43c9-9b3b-1b985c121d60#card`) | 3 | C |
| Serah Farron (`a8a7a3b7-015c-4fbd-8b90-e0ddf71663e4#face:0`) | 16 | C |
| Shadow in the Warp (`4d81db0d-2f56-42ac-92ce-249a34d262d4#card`) | 24 | C |
| Siren Reaver (`bc1c9ca8-2f00-4761-bd85-1329287ae7ac#card`) | 18 | C |
| Slinza, the Spiked Stampede (`8bed095d-1538-4c80-b147-3ec308a8acb1#card`) | 168 | C |
| Sokenzan, Crucible of Defiance (`c5ee72d5-3a9e-4fe5-8802-3286ee612055#card`) | 24 | C |
| Sokka, Swordmaster (`90b4d288-e950-4182-af62-516549a8712c#card`) | 144 | C |
| Spectacle Mage (`71920c05-7061-4ba5-8903-cffd3d2f85f4#card`) | 20 | C |
| Spectral Denial (`f8536726-993b-4100-918c-e3c7177d4c65#card`) | 24 | C |
| Spellbinding Soprano (`dffd5f0f-3a7b-47ed-864e-06c752bdcefc#card`) | 8 | C |
| Spellwild Ouphe (`13979fb6-91ba-4dd2-9bb9-8dbc7307d433#card`) | 2 | C |
| Sphere of Resistance (`09c96077-3804-4f12-a613-5bebc5e0413f#card`) | 2 | C |
| Sphinx of New Prahv (`3bdd3090-4503-4382-9a11-3e1cfcdffd71#card`) | 4 | C |
| Sproutback Trudge (`d9826660-0f34-4d82-8915-cf2796ab8d9a#card`) | 24 | C |
| Squash (`54bd7b7c-4052-437d-b2e4-311339e8e16e#card`) | 3 | C |
| Squeeze (`e18d8326-c3c3-43c2-ac13-6861c0ef3888#card`) | 2 | C |
| Squelch (`b8fe20ee-d52d-4494-acf4-202808a2a22d#card`) | 1 | A |
| Starfield Mystic (`2975590d-6a5b-41e5-8a3b-e9325e71226a#card`) | 24 | C |
| Starnheim Aspirant (`e7b92789-47a9-430a-ad57-be7b6c3d63cb#card`) | 8 | C |
| Starnheim Courser (`01317d6c-8c11-40e1-8f7b-d36050973ada#card`) | 8 | C |
| Starnheim Unleashed (`495ebe60-f0e5-4f0f-9d99-b2c64c95dbf4#card`) | 24 | F |
| Starport Security (`5454dddf-c6e5-4b0d-b96d-4904c13606dc#card`) | 43 | C |
| Stasis Cocoon (`ba11db54-149d-427e-b9ff-d1e29bbcbb11#card`) | 3 | A |
| Static Snare (`b5b742d2-f19b-47c9-84bd-e85d975900e6#card`) | 33 | C |
| Steward of the Harvest (`1bb7a942-c4b9-47a2-9782-3ec3efcdaff5#card`) | 276 | A |
| Stinkdrinker Daredevil (`25af5e13-ebe8-4bc4-b8fe-8b9aee59579c#card`) | 8 | C |
| Stoic Rebuttal (`6dd5782d-2d1a-4d6b-8381-e15d0d5be24f#card`) | 6 | C |
| Stone Calendar (`301a59c1-e372-4c31-a2a8-88c5ba7d822a#card`) | 4 | C |
| Stone Idol Trap (`47cd98e2-0eaf-4292-a840-c5e8d035c83c#card`) | 48 | C |
| Stony Silence (`12c63a45-7eb2-4262-b571-3358182f5610#card`) | 2 | A |
| Storm Skreelix (`0ccd1084-64bb-41ee-81c0-666af434b8df#card`) | 96 | C |
| Stormcatch Mentor (`b2252471-01ca-4d58-aa99-4ab0aa5eae12#card`) | 8 | C |
| Stormscape Familiar (`a8f5a11f-3291-4ee9-a96d-4fa38af14932#card`) | 36 | C |
| Stormwing Entity (`ea43fd04-e9fa-42fb-ba20-22dcc21c6c43#card`) | 18 | C |
| Stratadon (`3b78b672-7773-44c6-9a21-f89c058966ff#card`) | 54 | C |
| Stuck in Summoner's Sanctum (`84d308ce-c3c5-4612-accc-6e6cc68a33be#card`) | 6 | A |
| Stupefying Touch (`566eafea-c8c2-43d6-b491-f20de276cdb6#card`) | 1 | A |
| Sunderflock (`c01b221f-3993-43ab-8cde-b42fb344e2a7#card`) | 20 | C |
| Sunscape Familiar (`615603bb-3899-4419-9444-e4b64c94a438#card`) | 36 | C |
| Suppression Bonds (`e844ded0-821a-4378-8f58-bf1e1c0269d5#card`) | 3 | A |
| Suspicious Detonation (`91f7672c-3090-48f4-8586-0426341c3159#card`) | 9 | C |
| Swampsnare Trap (`fc532d1d-6cf5-4b59-afb3-325d0bc0db77#card`) | 12 | C |
| Swat Away (`79090b00-d920-4134-a838-fb434cf3ee4d#card`) | 30 | C |
| Syr Elenora, the Discerning (`8f6a3dcf-32e0-48c7-9265-0e298df4a5e4#card`) | 16 | C |
| Takenuma, Abandoned Mire (`ac2dd694-d2f1-4025-8400-12332bdc882a#card`) | 288 | C |
| Tale of Momo (`728157a3-fcd4-4295-85ff-1162dbb64f8a#card`) | 144 | C |
| Tam, the Possibility (`c92c7744-0c92-4a36-9ada-4ffca812009e#card`) | 72 | C |
| Tamiyo's Logbook (`d9356199-4ce3-44fa-9463-90fc134088f9#card`) | 6 | C |
| Temur Battlecrier (`fadf984c-c224-432a-87b9-b34f929b8409#card`) | 72 | C |
| Tentative Connection (`d74de74f-be0f-4809-8b7a-1bd58ee85551#card`) | 96 | C |
| Thalia, Guardian of Thraben (`9b7f1d05-707c-4ed3-9f0e-8ced1232c2ee#card`) | 2 | C |
| Thalia, the Survivor (`55ef129e-698e-424f-be3c-3fbba6c2cc3e#card`) | 8 | C |
| Thaumaton Torpedo (`9f9ae015-dc59-4ecb-a2bb-668496b48a48#card`) | 10 | C |
| The Fire Crystal (`57276db8-9d8d-4587-ae88-dfdc44343f17#card`) | 80 | C |
| The Kenriths' Royal Funeral (`b4b0a4b7-b36f-4c96-b07b-a86c1efd333e#card`) | 6720 | C |
| The Lord of the Eagles (`3b38f314-d997-4c2a-b4c9-40dbe780993d#card`) | 19 | C |
| The Mary Janes (`f2f4e14d-1ce8-41b3-b907-03ff561b42f7#card`) | 9 | C |
| The Scarlet Witch (`d8ae8c38-501e-44ea-9faf-cf9d11ba6616#card`) | 20 | C |
| The Sibsig Ceremony (`bbdb731d-c533-48ef-b82a-cf1a8ba36208#card`) | 64 | C |
| This Town Ain't Big Enough (`2550c2ae-834d-46e7-9ab4-d7bf7bfa6a78#card`) | 6 | C |
| Thorn of Amethyst (`0c6c5336-9233-4ab1-9d55-79f20be7ea57#card`) | 2 | C |
| Thornscape Familiar (`3f92c684-f2fd-4fe5-bfb5-e27061d2c45b#card`) | 36 | C |
| Thranduil, the Elvenking (`4a5f76e7-40be-4b06-9935-4a3b2672e1c2#card`) | 90 | A |
| Thryx, the Sudden Storm (`20004398-bd04-4583-8378-9df743246a32#card`) | 8 | C |
| Thunderscape Familiar (`40f6ea6f-71e1-46bd-8d9a-ccb5f437892c#card`) | 36 | C |
| Titanic Brawl (`e8434c77-10df-4456-af27-acb95d624189#card`) | 61 | C |
| Tolarian Terror (`b9d0f2e1-62c2-44fd-ad38-471daf17bb0a#card`) | 27 | C |
| Tombstone, Career Criminal (`6ad2cbb6-2291-420c-bfc8-ed3f079b59bf#card`) | 40 | C |
| Torch Breath (`b6d61d96-a56a-4f4c-a196-05e03fe16ed2#card`) | 3 | C |
| Transcendent Envoy (`f08f7814-4585-41dd-b82e-ed2ef5e24242#card`) | 8 | C |
| Trapped in the Tower (`55746bf4-86e0-489b-b9dc-eba5f5b401dd#card`) | 3 | A |
| Traxos, Academy Guardian (`7b9a574f-6d56-421b-8fbd-94da1e6e3ce1#card`) | 9 | C |
| Trazyn the Infinite (`1ef3b48a-55ef-422e-ac7f-e1c1057eec2c#card`) | 18 | A |
| Truck Toss (`7d765055-da62-43f7-9580-98e9d164753b#card`) | 3 | C |
| Uncle Iroh (`8a98ddac-7341-4118-b188-51d8d03afd1d#card`) | 8 | C |
| Undead Warchief (`e6af56bf-bd78-4801-8f6e-033cdd68de3d#card`) | 16 | C |
| Urza's Filter (`46484242-28ed-43d2-b33a-1e5c1771acca#card`) | 2 | C |
| Urza's Incubator (`b380c04b-0bce-4328-8f0a-1425e48bee72#card`) | 4 | C |
| Uthros Psionicist (`7f51aede-fb86-4b7e-9027-2ba8be62b567#card`) | 8 | C |
| Valeria Richards, Precocious (`89931cf0-f20b-49b5-88c6-2a92aee8c50c#card`) | 32 | C |
| Vanish into Eternity (`c14e422b-6412-45ce-8e82-d47ba61557a0#card`) | 3 | C |
| Vanquish the Horde (`a332e80a-dc51-4dc6-bc85-e114a1c6fdb8#card`) | 9 | C |
| Venom's Hunger (`8388faa0-f64e-4b01-836e-617f99b808a4#card`) | 3 | C |
| Vindictive Flamestoker (`7ed9a887-9942-4482-942e-b3b05feb6342#card`) | 36 | C |
| Vine Gecko (`a9596b05-e0bb-4e70-82fe-5db60f87bf3d#card`) | 24 | C |
| Viper's Kiss (`35dead9a-20ab-45e9-be3e-36857c928ab9#card`) | 1 | A |
| Visions of Villainy (`3b46800b-711b-4cbb-b296-26115b4f6839#card`) | 6 | C |
| Voidmage Husher (`cdb1f944-a5ac-4683-a4eb-eebb82feea8d#card`) | 4 | A |
| Volcanic Salvo (`9c2fd3a3-96f2-4e84-85a9-77698c2f83aa#card`) | 15 | C |
| Voracious Reader (`c38db10f-c0b7-4391-b49a-77079a4d087c#face:1`) | 8 | C |
| Voyager Quickwelder (`87e8c044-5f83-437d-a89e-8a95559045f6#card`) | 8 | C |
| Vryn Wingmare (`c7c241f2-d3cc-4032-a9da-149de73c2539#card`) | 2 | C |
| Wall Off (`fbde72f6-a8b2-4864-ad6d-b91aa03d290e#card`) | 21 | C |
| Warden of Evos Isle (`f424b5e9-8f02-4491-a7d8-c7e088611c6a#card`) | 12 | C |
| Watcher of the Spheres (`d43463d2-4395-4616-acc3-c4f80ebe3242#card`) | 72 | C |
| Waterbending Scroll (`882d032b-db36-401a-8075-a9fabc90c53d#card`) | 3 | C |
| Whack (`19becb14-3774-4732-b792-d2d0e5a94b35#card`) | 6 | C |
| Wildvine Pummeler (`6c34674f-856b-4062-87d5-6ed73d9a9855#card`) | 12 | C |
| Wilt in the Heat (`3bed0b60-5944-44bb-9ddd-c82f323e6d20#card`) | 36 | C |
| Winged Words (`c623aeb1-e6d4-48fe-bd2a-a7a6729aa4df#card`) | 12 | C |
| Witchstalker Frenzy (`f46befcb-6a7f-4f5e-8634-393cbf4b3308#card`) | 9 | C |
| Wizard's Lightning (`2bd81743-84c8-43ec-86f5-68230036827b#card`) | 3 | C |
| Wizard's Retort (`b828251c-86a9-454f-9852-d0876d0f5153#card`) | 3 | C |
| Wolfkin Outcast (`ac3d07cd-88c4-4e22-9d86-f92d34f406d4#face:0`) | 3 | C |
| Wrath of the Bloodmane (`c1994d6a-984d-4b75-9198-1f1822eab6eb#card`) | 3 | C |
| Writhing Necromass (`7f6505a1-ab8b-468b-a5de-33451598b464#card`) | 12 | C |
| Yavimaya Sojourner (`b3e9917e-1bcf-4e8e-9550-7f5897dcb433#card`) | 54 | C |

### Review notes (2026-10-08)

- Gain reconciliation. The 412 gained identities against the pre-landing
  estimate of 654 touched / 351 sole: route C contributes 358 cost gains
  against the 351 sole estimate (+7); route A contributes 51 from the
  disclosed Activate `PastParticipialPremodifier` addition; route F
  contributes 3 from Foretell. About 296–310 touched faces remain unread on
  other clauses: 310 faces still contain *cost(s) … less/more* ({1} 142,
  {2} 79). "costs 3 life more" is attested once and is unsupported.
- Style only: the `use` placement in
  `tests/suite/cost_scalar_complement.rs` (about lines 34–45) sits below
  code rather than at the head of the module. No behavioural consequence.
