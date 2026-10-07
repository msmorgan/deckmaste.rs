---
needs: []
---
# Read quoted granted abilities as complements of has, gain and with

## Why

Quoted ability text granted by *has*/*have*, *gains*/*gain*, or attached to a
token by *with* never reads. Even trivial cases fail: `It has "This token
can't block."` has zero admitted roots, while the inner `This token can't
block.` has one. Likewise `Enchanted creature has "{2}, Sacrifice this
creature: You gain 2 life."` has 0 and its inner ability has 1 (probe on
`wlvwtnppyovn`). On that change (32,828 supported faces, 13,716 covered,
19,112 unread) a quotation mark occurs in a failing unit of 1,145 unread faces
and is the only recognised cause on 396. These are surface-bucket counts, not
gain forecasts.

The machinery is half-built, in two routes. `verbs.ron` declares
`Predicate([Role("GrantedAbility")])` for Gain and Have, and the generic frame
consumer reports that shape as unsupported (the `Complement(GrantedAbility)`
row of the unsupported-inventory table in the done
`english-v3-generic-frame-consumption`). Separately, the `frame_additions`
block of `crates/deckmaste_lexical_source/lexicon/core.ron` gives Have and Gain
an `Object(QuotedText)` frame. `crates/deckmaste_english_v3/src/declarations.rs`
has `QuotedText`, `QuotedClause` and `QuotedKeyword` constructions over
`Document`, `Clause` and `KeywordPhrase`. This ticket takes over the
GrantedAbility row from `english-v3-systemic-residuals`.

## Goal

A quoted granted ability is parsed recursively as one ability line (or
document) of its own, delimited by the quotation marks, and it fills one
selected Complement of *has*/*have*/*gains*/*gain*. A quoted ability after
*with* in a token or emblem description is a Postmodifier of that Nominal.
Coordination with keyword abilities ("has reach and "…"") reuses the existing
Coordination machinery. Reconcile the two routes into one analysis: retire or
re-spell the duplicate rather than keep both, and record which survives and
why.

## Analysis

Quotation marks set off text whose wording is cited rather than freely
composed (CGEL, Ch. 20, §6, p. 1753, [1]). Cited text of this kind can be the
complement of a verb or a supplement, as with embedded direct speech and
citation (CGEL, Ch. 11, §9.2, pp. 1026–1028, [7], [14]–[15]). Its internal
form is that of an independent utterance, not a subordinate clause, which is
why the quoted ability must be parsed by the same start Category as a
top-level ability line. When the quote ends the sentence, the quote's own full
stop is kept and the matrix full stop is suppressed (CGEL, Ch. 20, §6, p. 1755,
[9i]). Oracle text follows that rule: `has "… 2 life."` with no period after
the closing mark. The grammar must realise that byte-exactly in both
roundtrip directions.

## Witnesses

- Compulsory Rest: "Enchanted creature has "{2}, Sacrifice this creature: You
  gain 2 life.""
- Carrier Thrall: "It has "Sacrifice this token: Add {C}.""
- Rain of Filth: "Until end of turn, lands you control gain "Sacrifice this
  land: Add {B}.""
- Heroes of the Revel: "When this creature enters, create a 1/1 red Satyr
  creature token with "This token can't block.""
- Web-Shooters: "Equipped creature gets +1/+1 and has reach and "Whenever this
  creature attacks, tap target creature an opponent controls.""
- Energy Flux: "All artifacts have "At the beginning of your upkeep, sacrifice
  this artifact unless you pay {2}."" (plural *have*)

Each witness's only failing unit is the one quoted. Each inner ability that was
probed separately reads.

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. First find why `It has "This token can't block."` fails while both halves
   read, and record the cause before changing anything.
2. Before: `cargo xtask english-v3 --all --workers 12 --samples-per-face 0
   --output target/english-v3/granted-quotes-before.json` on the claim parent,
   stamped with its change id.
3. Write the six witnesses as tests first. Assert that the quoted span is an
   ability-line constituent with its own internal Reading.
4. Iterate on `--face-id` selectors; verify on `--all` at the end.
5. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
6. Spot-check every newly covered face's selected analysis. The landing record
   lists at least 10 by name with their Reading. A quoted span read as an
   opaque string, or a wrong inner analysis that starts parsing, is a defect,
   not a gain.
7. No guard may name a lexeme, construction, verb, noun, preposition or card;
   read declared features only. Tests are re-spelled, never deleted. No new
   xtask subcommands, flags, fixtures or tooling.
8. May edit `grammar:` frame fields in `plugins_v2` keyword actions and lexicon
   frames in `crates/deckmaste_lexical_source/lexicon/*.ron`; plugin bodies
   untouched. Lexicon additions must be attested in the supported corpus
   (pruning rule).
9. Gate: `cargo xtask gate --changed --from <claim> --run`.

## Out of scope

- Inner-ability failures with their own causes. Psionic Sliver's quote, for
  example, also needs Deal's ordered amount/recipient segments (still in
  `english-v3-systemic-residuals`). Count such a face as a structural gain only
  when its whole face reads.
- Emblem semantics and quoted text in reminder text.

## Landing record

Measurements below describe change `lklruqpnknqkokvnvvulmoptqmpzmuop`, with
15,792 covered faces in the final census. The before measurement describes
claim parent `vxszvmow` with 15,404 covered faces; the census reports the
unchanged feature change identity because measurement follows claim.
Evidence files live in the workspace's ignored `target/english-v3/` directory.
After retirement they are retained under the coordinator's ignored
`target/english-v3/english-v3-granted-ability-quotes/`. No evidence is tracked.

### Cause established before implementation

On claim-parent `vxszvmow` (the unchanged grammar census reports feature
change `lklruqpnknqkokvnvvulmoptqmpzmuop`), the direct probes saved under
`target/english-v3/cause-*.json` admit one root each for the inner Document,
the QuotedText, and the complete matrix Clause. The matrix Document has zero
roots; appending a second, external full stop admits one. `Sentence` always
requires an external full stop, so the absent matrix terminal is the primary
failure. Quotation casing and recursive parsing of this inner ability already
work. The missing prepositional quoted Complement and mixed keyword/quotation
Coordination are additional gaps.

The before census has 15,404 readable faces (6,785 one, 8,619 multiple),
17,424 unread, and zero issues. Ogre Marauder already reads a lowercase
quoted Clause followed by a duration Adjunct; preserve that authentic
fragment quotation while adding independently punctuated quoted Documents.


### PROVE

The six required tests first failed (0 passed, 6 failed); the final quotation
suite passes all 9. The complete supported corpus has 32,828 faces. Readable faces increase from
15,404 to 15,792: 388 named gains below, zero lost identities, and no loss
obligation to route. `granted-quotes-gains.json` and
`granted-quotes-losses.json` hold the exact identity comparison.

The final `--all --workers 12 --samples-per-face 0` census exhausts every
forest: 230,566 → 237,401 retained Readings, 237,401 exact roundtrips, zero issues, zero
duplicate Readings and zero internal failures. The census checks lexical
ownership, exact construction identity and leaf traversal for every Reading.
The independently constructed finite granted Complement and prepositional
quoted Complement tests additionally verify value → text → value membership,
node traversal and word traversal. The compiler test covers repeated quoted
children, a punctuation-only child, an empty optional tail and a nonempty lexical tail which must reset
the terminal feature. Negative quotation witnesses keep mismatched/unpaired
marks, an extra matrix stop and malformed inner syntax unread. Authentic
Ogre Marauder fragment quotation and trailing duration still read.

New guards use declared features only: terminal punctuation, quotation
structure, granted-ability kind, selected frame use, head coordination and
preposition permissions. Diff review finds zero lexical/card/construction
identity licensing guards. Lexical-source loading and GrammarEnvironment
construction succeed without errors. Unsupported frame signatures fall by
the GrantedAbility signature to 17; none of those 17 contains GrantedAbility.
The remaining signatures retain their ownership in
`english-v3-systemic-residuals`. The old coverage command's permitted-checker counter and coverage-lock ratchet
are v2 artifacts; v3 has no such emitted counter or lock, and this record uses
the complete retained-Reading census under the lexical-analysis decision.

### DISCLOSE

The canonical `verbs.ron` GrantedAbility role survives as a selected Complement.
The added Object(KeywordPhrase)/Object(QuotedText) frames on Have and Gain in
`core.ron` are retired. The post-landing follow-up also removes the unused
KeywordObject/QuotedObject grammar frames, their FrameUse values and table rows,
and SharedKeywordComplement/SharedQuotedComplement schemas and instances: no
lexical declaration supplies a head for them. Keyword-only grants project
one existing KeywordPhrase; mixed keyword/quotation grants use the shared
binary and serial Coordination schemas. A declared quotation-structure
feature prevents an already coordinated QuotedText from creating a duplicate
grant route. Quotation remains recursively analyzed as Document, with existing
Clause/Keyword fragments retained for independently attested fragment uses.

Before selection census: 6,785 one-Reading faces and 8,619 multiple-Reading
faces; after: 6,924 one and 8,868 multiple, with zero undetermined in both.
V3 preserves ambiguity and applies no destructive specificity selection, so
unique/specificity-resolved v2 figures do not apply. One sample per newly
covered face supplies a review witness, not a winner that discards alternatives.
All 388 samples contain QuotedText with inner Document; none substitutes
QuotedClause or QuotedKeyword for its independently punctuated inner ability.
All newly covered quoted spans were also independently parsed as QuotedText.

There are 1,198 paired quoted spans in normalized supported source. 748 have
an inner Reading independently of their matrix: 740 Document witnesses and
8 fragment witnesses. 390 quoted spans occur on 389 wholly readable faces
(the 388 gains plus previously readable Ogre Marauder). These are span counts,
not a forecast: the remaining inner failures and unread matrix faces retain
their own causes. Psionic Sliver now reads because the claim base already
supports its ordered damage segments; this ticket changes no Deal frame.

At the original landing, every previously covered keyword grant, such as
*has flying*, changed from
Object(KeywordPhrase) to Complement → KeywordGrantedAbility → KeywordPhrase.
This is an Object → Complement relation change, not just a wrapper rename.
Reading counts remain unchanged on all 15,404 previously covered faces; the
post-landing review accepts the analyses (21 probes correct, zero lost faces).

The 388 measured gains are compared with the ticket's historical 396 sole-cause
estimate on `wlvwtnppyovn` at 13,716 covered. That estimate classified failing
surface units on an earlier tree, whereas the landing measures whole-face
coverage on a claim base already covering 15,404 faces. The eight-face numeric
difference is not an eight-face regression or an identity-matched obligation:
the earlier surface bucket was not a gain forecast. The measured comparison
against the actual claim base has zero losses.

Thirteen supported faces nest single quotes inside a double-quoted granted
ability: Urza's Saga; Nesting Dragon; Reef Worm; Koth of the Hammer; Arlinn Kord;
Mu Yanling; Liliana of the Dark Realms; Old-Growth Troll; Huatli; Preston Garvey;
Harold and Bob; Toggo; Teferi's Talent. Arlinn Kord and Huatli here identify
the back faces Arlinn, Embraced by the Moon and Roar of the Fifth People;
full face names, identities and sources are in the ignored follow-up evidence.
QuotedText declares only double-quote forms, so none of these whole faces reads and none is mis-analysed as a coverage
gain. The nested single-quote gap is routed to `english-v3-systemic-residuals`.

Representative reviewed Readings (each quoted Document recursively contains
its own Ability, Paragraph, Sentence and Clause as appropriate):

| Face | Selected analysis |
| --- | --- |
| Compulsory Rest | Have Complement → QuotedGrantedAbility → QuotedText → Document → ActivatedAbility; SymbolCost plus ActionCost and declarative life gain. |
| Carrier Thrall | Have Complement → quoted Document → ActivatedAbility; sacrifice cost and imperative mana addition. |
| Rain of Filth | Initial duration Adjunct on matrix Clause; Gain Complement → quoted Document → ActivatedAbility. |
| Heroes of the Revel | Token Nominal Postmodifier → QuotedComplementPreposition → quoted Document → OrdinaryAbility; negative modal predicate. |
| Web-Shooters | Coordinated matrix predicates; Have Complement → Coordination of keyword reach and quoted triggered OrdinaryAbility. |
| Energy Flux | Plural Have Complement → quoted Document → OrdinaryAbility; initial temporal Adjunct and sacrifice imperative with unless Clause. |
| Llanowar Mentor | Have Complement in activated ability's second matrix Sentence → quoted Document → ActivatedAbility. |
| A Realm Reborn | Plural Have Complement → quoted Document → ActivatedAbility; tap cost and imperative mana addition. |
| Harmonic Sliver | Plural Have Complement → quoted Document → OrdinaryAbility; initial when Clause and imperative destroy predicate. |
| Psionic Sliver | Plural Have Complement → quoted Document → ActivatedAbility; two ordered damage/recipient clusters. |
| The Girl in the Fireplace | Two token Nominal Postmodifiers; first mixes vanishing and quoted imperative Document, second quotes a declarative Document containing a keyword-only granted Complement. |

Deviations and additions:

- Added KeywordGrantedAbility and QuotedGrantedAbility projections to retain
  the grammatical distinction between a KeywordPhrase and cited wording while
  consuming the existing canonical role. Added QuotedComplementPreposition
  for the distinct preposition Complement/nominal Postmodifier analysis.
- Added SharedGrantedAbilityComplement schema for coordinated selected heads;
  reused the existing Coordination and Series schemas for grants.
- Added derived terminal-punctuation composition in the compiler, including
  dynamic selected-frame rules and independently checked repeated fields.
  Terminal surface presence is independent of lexical casing presence, so
  punctuation-only children compose and empty optional children preserve it.
  The failure is at sentence punctuation, so a lexical frame change alone
  would not fix the witnesses.
- Extended the existing surface-feature compiler fixture rather than adding a
  fixture or xtask tooling. Its first run exposed an unintended trailing space
  in the test's empty Phrase repetition; the Sentence fixture now directly
  contains a Word. The punctuation assertions retain exact value comparisons.
- Re-spelled three artificial quoted examples in the existing grammar/article
  tests by removing their doubled external stop. Their quoted subjects and
  asserted outcomes survive.
- Added authentic fragment preservation, negative quotation, independently
  built value/traversal and compiler-composition checks beyond the six required
  witnesses to exercise both roundtrip laws and the shared compiler seam.
- Glossary gaps: Quotation, Terminal Punctuation and Granted Ability Complement
  now have source-backed Oracle English entries (CGEL pages verified directly).

Assurance counts for this change: restored 0; re-spelled 2 existing test functions (3 surface
examples); ignored 0; added 10 test functions (9 quotation tests and 1 compiler
surface-feature test); removed 0. STOPs: none. Plugin bodies and vocabulary
membership are unchanged.

### REPORT

Counts are stamped with change `lklruqpnknqkokvnvvulmoptqmpzmuop`, final covered
15,792 (before covered 15,404 on claim parent `vxszvmow`). Named declarations:
186 → 189 ordinary Constructions, 44 → 45 shared schemas (230 → 234 total).
Category instances: 310 → 316 rows in 44 → 45 instance groups; declared
Categories: 135 → 137. The final environment has 633 compiled Productions.
Declaration lines: 3,083 → 3,155; this partial breadth landing does not claim
the ultimate 2,800-line ceiling is satisfied.

Homograph inventory: 430 named surfaces with multiple lexical identities,
including catalog projections, restricted to declared-case realizations
attested in the supported corpus. Full named surface/identity lists are in
`granted-quotes-surface-inventories.json`; examples are `'s` (contracted Be,
contracted Have, genitive marker), Adventure (catalog and subtype), and Advisor
(catalog and subtype). Form-literal/vocabulary overlaps: empty named list.
No lexical spellings were added or removed; these inventories are unchanged
by the frame/property edits.

Performance advisory, workers 12:

| Tree | Covered | Corpus wall time | Checked-text thread CPU | Host load (1/5/15 minutes) |
| --- | ---: | ---: | ---: | --- |
| Claim parent vxszvmow, unchanged feature lklruqpn | 15,404 | 42,836,435,899 ns | 227,766 ns/B | 7.478 / 6.662 / 6.667 |
| Feature lklruqpn | 15,792 | 63,491,984,697 ns | 262,542 ns/B | 3.490 / 8.723 / 9.532 |

The final 63.49-second census exceeds the 16.26-second quiet-host advisory
ceiling on this loaded host. These are exhaustive v3 debug-census measurements;
performance is disclosed, not used as a gate. The six-face selector took
111,809,988 ns, 146,938 ns/B, workers 1, load 7.915 / 10.601 / 10.333, on the
same final grammar (covered 15,792). Census artifacts include exact integer
telemetry, per-face evidence and supported-source hashes.

Reverse-dependency closure from claim `vxszvmow` was derived by
`cargo xtask gate --changed --from vxszvmow --run --clippy`. The final gate
ran through the already built `target/debug/cargo-xtask` entry point:

```sh
cargo test -p deckmaste_construction_v3_core -p deckmaste_lexical_source -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask
cargo clippy -p deckmaste_construction_v3_core -p deckmaste_lexical_source -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask --all-targets -- -D warnings
```

Both pass; final gate exits 0: 755 tests pass across 84 result groups, with
one existing ignored test, `macro_schema_census_count_matches_21`, whose
attribute says it cross-checks the live corpus against the census on demand.
This change adds no ignore. The initial Clippy run found `single_match_else`
in the literal emitter; changing that mapping to `if let` preserves emitted
rules and resolves it. Final formatter check passes. Citation checks report
16,059 checked, zero stale and zero noncompliant; diff audit selects zero CR
citation sites because no CR citation changed. CGEL pages were read directly.
Full logs: `granted-quotes-gate-final.log`, `granted-quotes-fmt-check.log`, and
`granted-quotes-cite-*.log`.

### Newly covered identities and reviewed Reading witnesses

Every row projects its cited wording through QuotedText → Document. `C` denotes
selected GrantedAbility Complement; `M` denotes prepositional Complement within
a Nominal Postmodifier. Inner Ability kinds below are taken from the independent
quotation Reading, not inferred from the matrix text. Complete trees and quoted
source spans are in `granted-quotes-gain-review.json` and
`granted-quotes-inner-readings.json` in the ignored evidence directory.

| Identity | Face | Function | Inner Ability kind(s) |
| --- | --- | --- | --- |
| `fd230ab5-874e-4827-9b5c-6d39d079cee6#card` | A Realm Reborn | C | ActivatedAbility |
| `edbf1b87-2d1e-47e6-a04e-a2b1646af7d9#card` | Abnormal Endurance | C | OrdinaryAbility |
| `e8ebf5cb-3a26-4bbd-812b-414c129e4bb7#card` | Abstruse Interference | C | ActivatedAbility |
| `947a2665-2f4d-4193-8768-118f85334549#card` | Abundant Growth | C | ActivatedAbility |
| `a149eeb1-007c-435d-9d55-34af205090e0#card` | Acidic Sliver | C | ActivatedAbility |
| `7a5a81f7-1597-4972-8565-fd2a5667df99#card` | Adverse Conditions | C | ActivatedAbility |
| `9376ccf6-1b18-44e9-8f9d-c5ad38927e2f#card` | Arc Spitter | C | ActivatedAbility |
| `0b633017-bfdf-47cd-a60f-c93816b47e9c#card` | Arcane Teachings | C | ActivatedAbility |
| `e6479195-cbdf-4d2b-9f45-5fd0a6ab4a77#card` | Arm with Aether | C | OrdinaryAbility |
| `c284b1fd-f238-478a-bcf1-c4b8f32f11f3#card` | Armor Sliver | C | ActivatedAbility |
| `4dcd0583-cee0-46a6-ba43-93bca5eb0428#card` | Astrologian's Planisphere | C | OrdinaryAbility |
| `128304af-4555-4276-bf26-638017beb3ff#card` | Aura Flux | C | OrdinaryAbility |
| `4c71074b-b8bf-47f4-87e4-67c275abdbc5#card` | Avarice Amulet | C | OrdinaryAbility |
| `f955bc96-d602-4142-a9a2-87009cc7028c#card` | Awakening Zone | C | ActivatedAbility |
| `6f23621e-eceb-402e-a8d3-8e2b734762b3#card` | Banishing Knack | C | ActivatedAbility |
| `45222123-f5e0-48ff-b80c-858727d155d7#card` | Barbed Field | C | ActivatedAbility |
| `8fbca9ba-8b83-40ca-9f7e-5c483ba76675#card` | Barbed Sliver | C | ActivatedAbility |
| `18e7ba0d-88a5-466b-9228-d3f5aae8f83e#card` | Basal Sliver | C | ActivatedAbility |
| `929c67aa-49ff-4401-85e8-76990fe1a369#card` | Basilica Shepherd | M | OrdinaryAbility |
| `44661108-d72f-4d02-9303-7e4b0ecf7e9c#card` | Bear Umbra | C | OrdinaryAbility |
| `8f351961-69e3-4637-b010-d13f19daca39#card` | Bigger on the Inside | C | ActivatedAbility |
| `33899675-0de2-4adb-bf3f-55e18b493015#card` | Birthing Hulk | C | ActivatedAbility |
| `cd3d1e5d-db12-4878-92a2-23eb8d23eaf3#card` | Blazing Torch | C | ActivatedAbility |
| `75e13f7f-aded-405c-97ce-124fa0cb1929#card` | Blinding Powder | C | ActivatedAbility |
| `2a62b226-d317-4212-9781-9c1d72056f11#card` | Blisterpod | C | ActivatedAbility |
| `690ba846-964e-44ba-8637-9a1ce62f5931#card` | Bone Shaman | C | OrdinaryAbility |
| `d1deba03-b26a-4a0d-b242-24bb3a3f9ea6#card` | Bootleggers' Stash | C | ActivatedAbility |
| `f61c0fc0-7241-455e-bbf5-f1f3862d46e3#face:0` | Brightcap Badger | C | ActivatedAbility |
| `e7d1c762-69d0-401d-8035-6e9744baf9d8#card` | Brood Birthing | C | ActivatedAbility |
| `a20edb8d-219d-4d0f-8346-0c31064eb49f#card` | Brood Butcher | C | ActivatedAbility |
| `9a08bb4b-46df-4b99-8f86-670d07e900ba#card` | Brood Monitor | C | ActivatedAbility |
| `0ec54683-a65c-461d-862e-585b6499d6a5#card` | Burn Down the House | M | OrdinaryAbility |
| `731a70e8-40d6-47b9-b2b3-b16c818e1935#card` | Call the Scions | C | ActivatedAbility |
| `96c12c12-140a-462d-bff0-0554d192b1f4#card` | Callous Bloodmage | M | OrdinaryAbility |
| `2f7b46c1-3d82-4fd9-9882-589315cab47b#card` | Candlestick | C | OrdinaryAbility |
| `1f59b3d9-1535-4e13-a0a1-0f839fe90ac6#card` | Capricious Sliver | C | OrdinaryAbility |
| `39229fbc-1296-4ac7-9f79-65c6f0d09991#card` | Caribou Range | C | ActivatedAbility |
| `b3e0b8a6-5de4-4988-b042-2641b1ed3e32#card` | Carrier Thrall | C | ActivatedAbility |
| `4c2c5f3b-61f3-4772-9ade-f5be80719e9f#card` | Catacomb Sifter | C | ActivatedAbility |
| `433e92d2-899a-4eed-98b9-80de5d25494b#card` | Cathar's Call | C | OrdinaryAbility |
| `2b5f70ec-33da-4dff-9cff-fe05eb906685#card` | Caustic Tar | C | ActivatedAbility |
| `52d04c36-9d38-42b2-9e47-905ab06cb708#card` | Cement Shoes | C | OrdinaryAbility |
| `db221948-fcb9-4ef9-ae69-70266235dd04#card` | Ceremonial Knife | C | OrdinaryAbility |
| `010e0d5c-e751-4f67-b305-8a598cc1f0de#card` | Chamber of Manipulation | C | ActivatedAbility |
| `c1b276b7-f1b3-4f6f-b6a2-1315d805e0f2#card` | Chittering Dispatcher | M | ActivatedAbility |
| `07b28ed7-eab8-4560-9994-3befcba876d3#card` | Choco-Comet | M | OrdinaryAbility |
| `7522f7fd-6fda-4ed3-9131-21bf91616a73#card` | Chocobo Racetrack | M | OrdinaryAbility |
| `539f5396-d99a-417d-a84c-dff7930b5900#card` | Chromatic Lantern | C | ActivatedAbility |
| `2d41fd1f-4a98-45dd-a3c5-f7c5e0152109#card` | Circle of Power | M | OrdinaryAbility |
| `f67df84f-667e-41cf-80f9-911abafe70ac#card` | Citanul Hierophants | C | ActivatedAbility |
| `557dda97-c9e6-4d3d-95d8-8180ab9d87b3#card` | Claim of Erebos | C | ActivatedAbility |
| `2b62a716-7831-4c76-8f2d-b99b04e33350#card` | Clavileño, First of the Blessed | C | OrdinaryAbility |
| `2a885729-0618-4a25-b18d-b0587f14718d#card` | Clot Sliver | C | ActivatedAbility |
| `a34a71eb-26a4-4e37-8d3c-a0352bea491b#card` | Cloudsteel Kirin | C | OrdinaryAbility |
| `3fdbe867-63cd-42c5-9843-46b0992b9628#card` | Commander Mustard | C | OrdinaryAbility |
| `57bc1106-81a0-4d91-94db-25e3a5f989cb#card` | Commander's Authority | C | OrdinaryAbility |
| `03160a74-8b83-4e07-b702-977a91a6311f#card` | Commanding Presence | C | OrdinaryAbility |
| `020225db-5623-49c2-8cdf-1ec2a05b8b0a#card` | Compulsory Rest | C | ActivatedAbility |
| `270508aa-b2e7-4066-be2f-4308cffe2b94#card` | Consecrated by Blood | C | ActivatedAbility |
| `fd26f893-6f45-44df-aa4e-c919fc118aff#card` | Constricting Sliver | C | OrdinaryAbility |
| `16dddf03-449c-4df0-9902-f8901f05656e#card` | Consuming Fervor | C | OrdinaryAbility |
| `20b8911e-6701-485c-ac62-2b3cc79ec288#card` | Coral Net | C | OrdinaryAbility |
| `68a58e0b-1506-492e-8ca3-016e520c10ba#card` | Cornered by Black Mages | M | OrdinaryAbility |
| `5a6c9376-313b-4420-890e-5aa78fa6382f#card` | Corpsehatch | C | ActivatedAbility |
| `99ba0d4f-b2bf-48ea-95d6-b6b85ee077ab#card` | Crawling Chorus | M | OrdinaryAbility |
| `c23f2546-bf9e-4390-b6f7-c616920e04ca#card` | Creeping Crystal Coating | C | OrdinaryAbility |
| `b0bc2d60-20d0-4ca2-be86-65b297226ac7#card` | Crypt Sliver | C | ActivatedAbility |
| `043f869d-b11c-4c0d-9591-2bf0df7bde55#card` | Cryptolith Rite | C | ActivatedAbility |
| `77661282-1c98-4f70-b617-c75633ab04c8#card` | Crystalline Nautilus | C | OrdinaryAbility |
| `52fe8101-82af-4aed-a395-54e7b7639bd2#card` | Curious Inquiry | C | OrdinaryAbility |
| `22a23706-a5fe-46f3-b845-6f58d558c723#card` | Curious Obsession | C | OrdinaryAbility |
| `9a9664d7-2299-43dc-89b6-d74fe69e0e4a#card` | Custody Battle | C | OrdinaryAbility |
| `23a265ef-d651-4d8d-94dd-0072f83541f9#card` | Cybernetica Datasmith | M | OrdinaryAbility |
| `caeb508a-c1fd-45b5-a14e-e0ac7321c9a9#card` | Dance with Devils | C | OrdinaryAbility |
| `88e5b905-fe52-4fd6-8cdf-b7ede5166ebc#card` | Darkheart Sliver | C | ActivatedAbility |
| `ad0f50d5-7b40-4fd8-8d13-4a0493398f53#card` | Deathpact Angel | C | ActivatedAbility |
| `541c6564-ec39-4b14-8c02-dd6a5d122ed3#card` | Debtor's Pulpit | C | ActivatedAbility |
| `ccc2eb6d-21cf-45cf-b4e4-4b2a2517136a#card` | Deconstruction Hammer | C | ActivatedAbility |
| `0decd8ea-a3d6-436b-81e9-bdd836537a1c#card` | Defiling Tears | C | ActivatedAbility |
| `162bcc9d-6edf-4b3b-83e6-2c933e2a5967#card` | Demonic Gifts | C | OrdinaryAbility |
| `87fca901-678d-4c9a-be4a-9dce2a53074b#face:1` | Despair | C | OrdinaryAbility |
| `13aeccd5-cca2-4249-904d-d5516f977440#card` | Deviant Glee | C | ActivatedAbility |
| `ab9e2045-860a-40ab-bec1-0dffced003ef#card` | Devils' Playground | C | OrdinaryAbility |
| `542e7d60-3fc8-46e2-8786-f7c8b3d4c979#card` | Diamond Pick-Axe | C | OrdinaryAbility |
| `e57025ed-3de6-4b2f-92bd-5481fd33b7f1#card` | Digsite Engineer | M | OrdinaryAbility |
| `b76566a9-f998-4a20-b073-4b60862843c9#card` | Dismiss into Dream | C | OrdinaryAbility |
| `af8fba92-f7e5-485d-a1da-a30c46a3992e#card` | Disruption Aura | C | OrdinaryAbility |
| `0123b162-0413-4022-acc0-e5bd061d3fb4#card` | Divergent Growth | C | ActivatedAbility |
| `2bc3d5b2-076f-4e01-87be-28d7eb70a2db#card` | Dormant Sliver | C | OrdinaryAbility |
| `f034137e-c19f-48af-a2f2-f767bf87a275#card` | Draconic Destiny | C | ActivatedAbility |
| `8f561498-66e7-4cbf-8cb9-e6fd4717993d#card` | Dragon Egg | M | ActivatedAbility |
| `0cc5aaa1-07bf-4872-ac72-303cef9fb143#card` | Dragon Mantle | C | ActivatedAbility |
| `5be7a4d5-33b7-464b-8851-d4ad35302e62#card` | Dragon-Cursed Halls | C | OrdinaryAbility |
| `fb16c4b0-9ac4-4664-9b57-97bfc22e5de9#card` | Dragonrage | C | ActivatedAbility |
| `b6821131-9ae1-4a3c-8235-90deae33c05f#card` | Dread Drone | C | ActivatedAbility |
| `781d6981-d389-4677-9eaf-9c46c9192ed6#card` | Dreadmaw's Ire | C | OrdinaryAbility |
| `87fca901-678d-4c9a-be4a-9dce2a53074b#face:0` | Driven | C | OrdinaryAbility |
| `c394d4b6-cfb8-4639-90cb-a322a25c0a2f#card` | Dropkick Bomber | C | OrdinaryAbility |
| `2b3f2e85-5ea4-44d6-86e8-e8ce6d4f98c0#card` | Drowner of Hope | C | ActivatedAbility |
| `7cf5c3dd-5c07-4eaf-8227-6f4e7f9b134d#card` | Drownyard Lurker | M | ActivatedAbility |
| `d63f4eeb-3fa5-4e65-8414-e3fd3c03c128#card` | Drudge Spell | C | ActivatedAbility |
| `4d22b01b-63dd-4698-a221-0acaefb2000b#card` | Dual Casting | C | ActivatedAbility |
| `7811b50d-af76-43cd-8753-43cf65fbefa3#card` | Dune Chanter | C | ActivatedAbility |
| `ef702a1a-4ace-43e0-8e86-dcceb0b1e4d5#card` | Edgewall Pack | M | OrdinaryAbility |
| `ae3e5c80-6a6a-4781-a7b9-4d63357ffa3b#card` | Eldrazi Repurposer | M | ActivatedAbility |
| `b5661289-a03c-40fc-a1e4-f602adb56f81#card` | Eldrazi Skyspawner | C | ActivatedAbility |
| `1ee811fb-c25d-4cf5-b396-539c36e8f90f#card` | Emrakul's Hatcher | C | ActivatedAbility |
| `f1409c9e-f0d9-49e1-b02e-d0f04a658331#card` | Emrakul's Messenger | M | ActivatedAbility |
| `47630b28-c0bc-4911-8d76-33c6fcabf2d7#card` | Endless Whispers | C | OrdinaryAbility |
| `3577c47e-76d3-4659-b922-31c4b74be3a0#card` | Enduring Vitality | C | ActivatedAbility |
| `7a756cd1-29a8-4edf-bb74-fbb5b4020022#card` | Energy Flux | C | OrdinaryAbility |
| `0870e9f8-9e02-4997-9554-41a4e8eec4cb#card` | Ephara's Radiance | C | ActivatedAbility |
| `833fe5f4-cfab-46ce-a4fe-250673896350#card` | Epiphany Storm | C | ActivatedAbility |
| `0fa2cb01-476e-4e82-94e6-9639e53a7743#card` | Equinox | C | ActivatedAbility |
| `92023a5d-a143-4950-a71b-d736e6b8e959#face:0` | Esika, God of the Tree | C | ActivatedAbility |
| `c29b997a-5be6-4d51-a81b-d00677ef401d#card` | Essence Feed | C | ActivatedAbility |
| `dd757328-0593-4fc7-a099-e82f5c0677ff#card` | Essenceknit Scholar | M | OrdinaryAbility |
| `a1b66fbf-14a3-4582-8414-53f432f96dad#card` | Eternal Thirst | C | OrdinaryAbility |
| `e766b774-eaf3-41ce-bc23-fd29fd22bc39#card` | Evanescent Intellect | C | ActivatedAbility |
| `854e0e2c-06dc-4166-a48a-107e939ca91e#card` | Evolution Vat | C | ActivatedAbility |
| `87f82d15-5b6b-400f-8a15-d45a2a735f7d#card` | Experimental Confectioner | M | OrdinaryAbility |
| `ba047b0b-d451-4566-8656-00b00611de8c#card` | Eyeless Watcher | C | ActivatedAbility |
| `ad01df89-29fe-44c7-a133-91425f8ff09c#card` | Fake Your Own Death | C | OrdinaryAbility |
| `4b8183e6-4ff2-4254-9ee0-0ebbf8cef4a3#card` | Fallen Ideal | C | ActivatedAbility |
| `0d71b157-09a0-4fd4-beb9-103117a784ad#card` | Farmstead | C | OrdinaryAbility |
| `85e9bc3e-e0da-4ad4-9631-472f75f91663#card` | Fearsome Temper | C | ActivatedAbility |
| `f718e507-296b-4f22-842b-5fb91322069b#card` | Feign Death | C | OrdinaryAbility |
| `adf731a1-fc4d-4260-bb85-535edb1d82e9#card` | Feral Appetite | M | OrdinaryAbility |
| `860eb9eb-c318-4eae-9b54-088f667115ec#card` | Find the Path | C | ActivatedAbility |
| `9d4372f0-eedf-4897-9f03-f96fb14ac4a6#card` | Fire Whip | C | ActivatedAbility |
| `b5a3c09d-e822-4e26-b569-7f5830a39e6a#card` | Firewake Sliver | C | ActivatedAbility |
| `b45cb1b8-091a-40d2-a53a-2fa95845f4fa#card` | Flame Fusillade | C | ActivatedAbility |
| `da1b633f-82b5-4f3e-884c-58c2fe4fcb0f#card` | Fly | C | OrdinaryAbility |
| `faecfb8d-2f63-4b98-a842-b11448f0d538#card` | Foggy Swamp Spirit Keeper | M | OrdinaryAbility |
| `ca4fdf89-a74e-4086-8109-b0f9dd91710e#card` | Footfall Crater | C | ActivatedAbility |
| `d2853bad-a751-45ee-8269-24b0cdbc5c6b#card` | Forbidden Lore | C | ActivatedAbility |
| `71393988-ad6f-43fd-9978-c0de15ae8e87#card` | Forgotten Monument | C | ActivatedAbility |
| `16f3a09f-e29b-4fb5-95ed-d76ad550c2ad#card` | Foul Presence | C | ActivatedAbility |
| `bb504fb1-1d19-4d0b-958f-e8f2433e0575#card` | Frondland Felidar | C | ActivatedAbility |
| `885c47cb-64fb-4c83-a9ca-4f2eecb7d3e5#card` | Fungus Sliver | C | OrdinaryAbility |
| `b623e61a-cdb9-46b0-a443-547be05db877#card` | Furnace Reins | C | OrdinaryAbility |
| `2c09ca09-8e62-4fe3-9b3d-61573dd2ffbc#card` | Gemhide Sliver | C | ActivatedAbility |
| `02d25219-888d-4032-9e9e-8c6f879a9084#card` | Ghostly Touch | C | OrdinaryAbility |
| `43a86d36-9653-4498-a533-8b30b919cdbf#card` | Gift of Paradise | C | ActivatedAbility |
| `0ffce5e0-6b1d-4d1a-9318-1a1a219df532#card` | Glaring Fleshraker | M | ActivatedAbility |
| `68a91b87-1845-419b-aae7-ee1237554946#card` | Gleam of Authority | C | ActivatedAbility |
| `28b2d31f-1ff7-4374-8ea3-0dd4a96c4a32#card` | Goldmeadow Lookout | C | ActivatedAbility |
| `716b3ea2-45b7-4a8f-af72-de7f4e510eff#card` | Goldspan Dragon | C | ActivatedAbility |
| `c4525b1c-5c3d-494e-b660-ba5a5a28f621#card` | Grafted Growth | C | ActivatedAbility |
| `51310af7-796d-4cf0-871e-5055765ffcdd#card` | Grasp of the Hieromancer | C | OrdinaryAbility |
| `b25962ab-21b5-47f9-bdef-35b911b45671#card` | Grave Birthing | C | ActivatedAbility |
| `79e69a91-d580-47fb-be76-1e32c50d2fa0#card` | Great Divide Guide | C | ActivatedAbility |
| `b36c66d3-24ae-4291-8f8b-8716e354975e#card` | Greater Stone Spirit | C | ActivatedAbility |
| `a341eb75-e6a0-468b-8c83-61102249c648#face:0` | Greenhouse | C | ActivatedAbility |
| `eddfe7d7-6868-4b81-b46c-d0765e5210e7#card` | Gysahl Greens | M | OrdinaryAbility |
| `1c06c093-18ab-49e6-a82d-38e826d25016#card` | Harmonic Sliver | C | OrdinaryAbility |
| `e3542d0f-fcee-4df7-aa95-b55138caa2ab#card` | Harried Spearguard | M | OrdinaryAbility |
| `9ebd9ceb-05a4-4387-b63e-4c62ead2b98b#card` | Heartseeker | C | ActivatedAbility |
| `748fd75b-c61b-4e02-889d-b7dab23319ef#card` | Heavy Arbalest | C | ActivatedAbility |
| `5d307515-6ea1-4b1f-8e4b-49bfebfbdfcc#card` | Hellish Rebuke | C | OrdinaryAbility |
| `792d7818-ee5c-4254-b969-7b50f475c629#card` | Hermetic Study | C | ActivatedAbility |
| `d4f72eac-2161-4fe8-8590-436e241c3b22#card` | Heroes of the Revel | M | OrdinaryAbility |
| `edb8a6d6-9e25-4048-a842-e854eece42c2#card` | Hibernation Sliver | C | ActivatedAbility |
| `739ad367-a4ff-4779-8888-3c27f0f60253#card` | Hollowhead Sliver | C | ActivatedAbility |
| `fa023c32-2cec-416c-90b7-07b7a19c82d4#face:1` | Homura's Essence | C | ActivatedAbility |
| `e7c3bd74-959c-4b07-b13d-30fe0f0ede2a#card` | Hostile Realm | C | ActivatedAbility |
| `53f1bcff-55fb-4d38-b108-dcbed623f488#card` | Hunt for Specimens | M | OrdinaryAbility |
| `a93a8edf-2879-4cae-b3b9-b46298270823#card` | Hydro-Man, Fluid Felon | C | ActivatedAbility |
| `269a2b53-52d7-4032-ba62-7b7742e7a159#card` | Hypervolt Grasp | C | ActivatedAbility |
| `1c4fdd52-dc99-4942-bdaa-93f74dd4ebdc#card` | Iconic Shield | C | OrdinaryAbility |
| `154e865f-1aab-4d72-9e84-44e5316bbf3c#card` | Immobilizing Ink | C | ActivatedAbility |
| `0a992ad5-136f-482d-8390-fdfe745fe764#card` | Incite Hysteria | C | OrdinaryAbility |
| `088d63ec-bd5e-4e80-acf9-2af270a9b395#card` | Incubator Drone | C | ActivatedAbility |
| `9587bdb3-fb6a-47c0-898d-404bf60fcb6b#card` | Indoctrination Attendant | M | OrdinaryAbility |
| `e1a5c389-6a56-4da7-b7c4-e26fdec224e5#card` | Inevitable End | C | OrdinaryAbility |
| `3fc640dd-6292-4d36-82fb-bd366bc00bd3#card` | Infernal Scarring | C | OrdinaryAbility |
| `e9c03ed6-34dd-46f7-ab41-8004348328cc#card` | Infested Fleshcutter | M | OrdinaryAbility |
| `83669321-46cd-4805-b4ce-441c5db0f338#card` | Infinity Formula | C | OrdinaryAbility |
| `6799217a-030c-4c1b-9de3-99725c6ef4cf#card` | Infuse with Vitality | C | OrdinaryAbility |
| `5726c4b1-0613-4724-b1fc-2e7f398bf1af#card` | Instill Furor | C | OrdinaryAbility |
| `139b098c-d4be-4dca-9bde-9eb8ea97c7bf#card` | Joiner Adept | C | ActivatedAbility |
| `c5c19d6e-16d5-4d24-be2d-c63646bdf00c#card` | Jon Irenicus, Shattered One | C | OrdinaryAbility |
| `e86c965b-ac10-4fff-b682-dddd6d9747c6#card` | Judith, Carnage Connoisseur | M | OrdinaryAbility |
| `7359e82b-db79-488d-a1d4-75a00f12a4cf#card` | Kaldra Compleat | C | OrdinaryAbility |
| `fc3e48c9-e303-435c-bdb1-7dc7881a9858#card` | Karametra's Favor | C | ActivatedAbility |
| `27ffa6e9-c161-4f53-8b41-7fae5f199f41#card` | Kataki, War's Wage | C | OrdinaryAbility |
| `e8fd428f-7521-4e6a-86b7-835f20ee3c5c#card` | Kellan, Planar Trailblazer | C | OrdinaryAbility |
| `60808c8f-5257-451b-a9b6-389ba51598d5#card` | Kira, Great Glass-Spinner | C | OrdinaryAbility |
| `452ef7d0-1351-40e6-9379-59cd8bb31da5#card` | Kozilek's Predator | C | ActivatedAbility |
| `334f673e-7cfa-432c-93aa-41adfbb99113#card` | Lavabelly Sliver | C | OrdinaryAbility |
| `b5ff42a1-1ac4-472b-8479-5e3749845305#card` | Leafdrake Roost | C | ActivatedAbility |
| `d5f72164-441e-4e04-8b2b-0d6ab618d22e#card` | Leonin Bola | C | ActivatedAbility |
| `85f40ced-a065-470a-a686-faaca17fa40c#card` | Lightning Prowess | C | ActivatedAbility |
| `93b6083b-f49a-4641-b76f-8ccbec22f91d#card` | Lightning Volley | C | ActivatedAbility |
| `9d3f1b93-ec3b-47eb-bd93-89890fd33ccb#card` | Livewire Lash | C | OrdinaryAbility |
| `bc7b6508-6522-4772-93b0-3985e7e75048#card` | Llanowar Mentor | C | ActivatedAbility |
| `3858a2ff-cdc9-4e40-bc2c-42ed691aae08#card` | Lorehold Apprentice | C | ActivatedAbility |
| `a37adc18-ea11-4972-8d04-936cf3fe2613#card` | Lost in the Spirit World | M | OrdinaryAbility |
| `ca4d3d27-ea3a-44ac-96e9-b52b563129f8#card` | Lotus Ring | C | ActivatedAbility |
| `0da104ae-c46c-4106-919f-d975721d42d9#card` | Lunarch Mantle | C | ActivatedAbility |
| `4cc014f3-05e0-442e-9dee-03eab1aa65a3#face:1` | Mage Siege | M | OrdinaryAbility |
| `94b1e7ed-87f3-44e6-bf34-186dd9a1a0d1#card` | Mage's Attendant | M | ActivatedAbility |
| `8ab4d31e-7187-4ba1-97cb-1a0a11ab28bf#card` | Magus of the Tabernacle | C | OrdinaryAbility |
| `5de50aa5-3b30-4d7d-9203-bc8fc0f8fb34#card` | Make Mischief | C | OrdinaryAbility |
| `a731e87b-8d99-4b64-8ee3-8e540d652366#face:0` | Malakir Rebirth | C | OrdinaryAbility |
| `8aac06d0-c353-4af9-a243-feb59b9d0c54#card` | Malicious Intent | C | ActivatedAbility |
| `bd47398d-da35-4a09-8754-771af91b14f4#card` | Manaweft Sliver | C | ActivatedAbility |
| `cc2eb3e4-084c-471b-9fe0-72aa9ff2019a#card` | Manriki-Gusari | C | ActivatedAbility |
| `eacfe895-5106-437c-b8c1-2ff03c107744#card` | Medic's Kitesail | C | OrdinaryAbility |
| `a6763205-47c7-4d7f-be8c-2f69e8321048#card` | Mephidross Vampire | C | OrdinaryAbility |
| `21c82044-6e16-4e41-a6d0-e4d7cb42f37c#card` | Mesmeric Sliver | C | OrdinaryAbility |
| `f3f2db4b-4a07-453c-ae18-aeec0adc8d3d#card` | Midnight Covenant | C | ActivatedAbility |
| `7a08b371-1353-476d-ae72-379594f801da#card` | Mindlash Sliver | C | ActivatedAbility |
| `d29f59a4-b956-40e0-acd4-c2560e538baf#card` | Mistform Sliver | C | ActivatedAbility |
| `d92411b3-cb25-4123-a78e-4f785dfdae0c#card` | Mite Overseer | M | OrdinaryAbility |
| `1162c7ff-c0cb-469a-ad3c-322b0981ae1d#card` | Mitotic Slime | C | OrdinaryAbility |
| `101a5c2d-b0b4-45a7-b236-df474ac47356#card` | Mnemonic Sliver | C | ActivatedAbility |
| `1a668556-da98-4f1e-b1f7-26b7d63357a1#card` | Molting Snakeskin | C | ActivatedAbility |
| `f48b2349-797e-4e79-aea7-2413e78129e1#card` | Morgul-Knife Wound | C | OrdinaryAbility |
| `77248a76-373c-48fc-939f-aa378ab57f7c#card` | Mortarpod | C | ActivatedAbility |
| `81c223a5-5ff2-43e5-bd0b-25c4f72b2150#card` | Multani's Harmony | C | ActivatedAbility |
| `10039992-d51a-47e7-9a70-02fe2227c163#card` | Mysidian Elder | M | OrdinaryAbility |
| `967eca77-d273-4fc6-97d1-df7d8cc6ff59#card` | Mystic Might | C | ActivatedAbility |
| `1a7e367e-1d8d-463d-a35f-4bad44a5ebea#face:1` | Mystic Monstrosity | C | ActivatedAbility |
| `9d637892-26d7-46f1-bd7d-89bda523f649#card` | Necrotic Plague | C | OrdinaryAbility |
| `9655569d-bfa5-4665-9371-9f275b8d223e#card` | Necrotic Sliver | C | ActivatedAbility |
| `a7584a28-0829-4897-abf4-6be1a9232347#card` | Nest Invader | C | ActivatedAbility |
| `11132bbe-985d-44fa-a5b4-4b4e0dc3a7cf#card` | Nettlevine Blight | C | OrdinaryAbility |
| `8834d3c9-cdb5-423b-bc78-94a94add5e74#card` | New Horizons | C | ActivatedAbility |
| `58c615f3-8c3b-47bf-8032-809fc4232ccb#card` | Ninja Pizza | C | ActivatedAbility |
| `fa5aefaf-432f-41d0-8e54-0e8b7bc7ba98#card` | Ninja's Kunai | C | ActivatedAbility |
| `c75ec3c6-2786-4386-b758-ac03610f2a99#card` | Noxious Field | C | ActivatedAbility |
| `3f03c834-2cb3-4c54-bee6-c64f3a5fce47#card` | Nurturing Presence | C | OrdinaryAbility |
| `e690461b-2925-4484-b6fa-69ca1166f5a2#card` | Oblivion Crown | C | ActivatedAbility |
| `e649614a-ff23-4234-92f4-6f564cbfe648#card` | Ocular Halo | C | ActivatedAbility |
| `8922a91e-3d15-4351-8d77-e0d6bc4de82e#card` | Olivia, Crimson Bride | C | OrdinaryAbility |
| `a871b03e-0218-4c15-8ad8-c8284b5be45f#card` | Opaline Sliver | C | OrdinaryAbility |
| `d9688f6e-7163-408b-b32b-b9645aed1f8c#card` | Open into Wonder | C | OrdinaryAbility |
| `e5818ab3-8c37-4063-9e9d-cbe4ae0bba14#card` | Overlaid Terrain | C | ActivatedAbility |
| `e9975888-c75f-471e-a70d-17449e92226d#card` | Pain 101 | C | OrdinaryAbility |
| `c1121b83-1ba2-473d-89c9-e3bbd4529072#card` | Paradise Mantle | C | ActivatedAbility |
| `cb15bcc7-151f-44a4-90ee-2d39d8fd5299#card` | Pathway Arrows | C | ActivatedAbility |
| `fe902db2-3b1d-4266-88eb-307f878fc317#card` | Pendrell Flux | C | OrdinaryAbility |
| `b18b8415-a0b1-459d-8c6b-896381184935#card` | Pendrell Mists | C | OrdinaryAbility |
| `e0a35a44-e56d-434e-8043-43d5031b2e84#card` | Perigee Beckoner | C | OrdinaryAbility |
| `cb125cd4-c7c2-415d-b802-bdcfc2454403#face:1` | Pest Friend | M | OrdinaryAbility |
| `ecc91e38-90fa-4d89-b262-d5f36dce5be4#face:1` | Pest Problem | M | OrdinaryAbility |
| `f6e0cb77-4115-4948-8e1e-fdb774b1b254#card` | Pest Summoning | M | OrdinaryAbility |
| `2970d09b-4bef-468e-8de5-f994c820ccc0#card` | Pestbrood Sloth | M | OrdinaryAbility |
| `74776491-29c3-46e6-989b-cf9a00bbfcef#card` | Pillory of the Sleepless | C | OrdinaryAbility |
| `ee10dc06-26e1-4f50-a0a6-6d039e71b75f#card` | Plague Sliver | C | OrdinaryAbility |
| `6be3cf35-f774-4afb-8c1f-5c9b616a2b3f#card` | Poultice Sliver | C | ActivatedAbility |
| `df4903cb-50fa-4620-8470-6a6a398fc1ef#card` | Power of Fire | C | ActivatedAbility |
| `ab42398c-f0a1-4b94-ac5f-b8768e1b4e05#card` | Presence of Gond | C | ActivatedAbility |
| `a2d3e3f0-7399-4eb9-a0cf-831710c3d431#card` | Presumed Dead | C | OrdinaryAbility |
| `a22b8220-b41d-4b58-aa68-2c65ce37c517#card` | Professor of Zoomancy | M | OrdinaryAbility |
| `9485db26-7e03-4a66-beeb-627a3bc6f367#card` | Propagator Drone | M | ActivatedAbility |
| `69af7bff-fb8c-4f59-88a1-6bfa87ba400a#card` | Prophetic Ravings | C | ActivatedAbility |
| `842bb10a-f056-4db8-a341-2d20796b99ba#card` | Psionic Gift | C | ActivatedAbility |
| `ea79d4ee-c7b5-4687-9e33-22f98e46e8ec#card` | Psionic Sliver | C | ActivatedAbility |
| `c50354fc-7dfe-4ea7-87dd-488ca6213344#card` | Psychic Overload | C | ActivatedAbility |
| `5870175d-83df-43af-b199-fcd1e60b824e#card` | Psychic Trance | C | ActivatedAbility |
| `cdce8b86-81e3-443c-af91-f6b4f62f5b59#card` | Pursuit of Flight | C | ActivatedAbility |
| `0527b7b5-a55d-4720-af1f-17fc1178f4a6#card` | Queen Brahne | M | OrdinaryAbility |
| `5c77bf76-bb0f-4e44-babe-0e7b7b555e1f#card` | Quicksilver Dagger | C | ActivatedAbility |
| `beb39342-5272-41a4-a470-9f2bf0cd0f92#card` | Quilled Sliver | C | ActivatedAbility |
| `e9685c61-e33d-4ae0-9dd3-7fd0df5eb46f#card` | Quintessential Katana | C | OrdinaryAbility |
| `b7093835-0f84-4ea4-b228-2e39bd2642ab#card` | Racecourse Fury | C | ActivatedAbility |
| `76f5445a-5193-4794-8593-b44f961334b6#card` | Ragost, Deft Gastronaut | C | ActivatedAbility |
| `aa77980d-0434-40f0-990f-1c8c50610174#card` | Rain of Filth | C | ActivatedAbility |
| `43a2bd01-3fce-4191-a08a-1e379c7770b0#card` | Rakdos Riteknife | C | ActivatedAbility |
| `37043e1e-4556-45f7-9484-15dd8ed5272b#card` | Rat Out | M | OrdinaryAbility |
| `f2649e48-4252-43f3-bd1e-c4c6fcb0aa3a#card` | Razor Boomerang | C | ActivatedAbility |
| `bb9ce416-eef1-49e8-89a0-2b6837505070#card` | Realm of Koh | M | OrdinaryAbility |
| `ada48353-4c86-44d6-83a5-e7e3c703b948#card` | Redcap Gutter-Dweller | M | OrdinaryAbility |
| `f34be4b8-126d-4b5c-8ebb-23b5e5370012#card` | Rekindling Phoenix | M | OrdinaryAbility |
| `3609d300-af9f-4ddf-ad0e-8bebeca15a04#card` | Relic Bane | C | OrdinaryAbility |
| `42e4d4d8-e4e0-4cf8-a6d0-887a17513280#card` | Reservoir Kraken | M | OrdinaryAbility |
| `5b60a257-e4f8-412e-b146-b7983a384753#card` | Resplendent Mentor | C | ActivatedAbility |
| `4390e251-e154-4ef8-9929-0c78aa2c8f10#card` | Resuscitate | C | ActivatedAbility |
| `2707f9f4-2b80-47ac-af0a-99fc53af94bf#card` | Retraction Helix | C | ActivatedAbility |
| `9c002b4d-bb6c-45a5-88e1-1df2958dff9f#card` | Return to Action | C | OrdinaryAbility |
| `334ffdc4-9fe6-4a7f-bf5d-bcd88adab715#card` | Ringing Strike Mastery | C | ActivatedAbility |
| `a3b2e0a2-f4ad-457b-ae70-7732bc3479ff#card` | Rite of Belzenlok | M | OrdinaryAbility |
| `d870948e-721a-46fd-86b6-ecd42e3aa9bd#card` | Root Manipulation | C | OrdinaryAbility |
| `91d69d6a-3832-4d20-a02b-1e83c08ac723#card` | Run Wild | C | ActivatedAbility |
| `0d7969b0-193b-4da0-a9eb-bd1c98016ee1#card` | Sachi, Daughter of Seshiro | C | ActivatedAbility |
| `1c55af05-7fbe-4078-9463-689b21c0d3e4#card` | Sadistic Obsession | C | ActivatedAbility |
| `bc243497-d95a-44e0-b7e3-c56b4ff7cd69#card` | Savage Silhouette | C | ActivatedAbility |
| `fa60f3a6-5834-4059-adea-dd17366b7885#card` | Scion Summoner | C | ActivatedAbility |
| `9c0aba12-4460-4150-b74f-93f5f8006cf5#card` | Screaming Shield | C | ActivatedAbility |
| `88485575-e491-4ab7-a020-e52dd92ae085#card` | Screeching Sliver | C | ActivatedAbility |
| `7c3dcfea-f42c-4515-9b7d-6e9d8cb4d7f9#card` | Scuttling Sliver | C | ActivatedAbility |
| `10d8f29a-d483-4f06-bc13-d41d62847ffe#card` | Senator Peacock | C | ActivatedAbility |
| `955bfcb8-6477-4ed0-b6e7-8940874ae5e4#card` | Send in the Pest | M | OrdinaryAbility |
| `3466e0be-3e59-4f5e-812e-3250c8bf19aa#card` | Shackles of Treachery | C | OrdinaryAbility |
| `6b4a4de7-ec26-45ab-a460-e6c005000bf0#card` | Shade's Breath | C | ActivatedAbility |
| `682a6d77-e8cd-4bf2-b777-7793101e0fd3#card` | Shade's Form | C | ActivatedAbility |
| `f2c408f6-3994-4a90-a9f5-f248eb976017#card` | Sheltered Aerie | C | ActivatedAbility |
| `1ef07ae5-3b04-45c8-a080-852b84da4fef#card` | Shoving Match | C | ActivatedAbility |
| `0441a901-31ce-419f-a898-e9a9ad4ab351#card` | Showstopper | C | OrdinaryAbility |
| `c6e92e21-d3e7-4982-b9f3-af56d4014c07#card` | Shuriken | C | ActivatedAbility |
| `dad07f8c-1f24-4f56-b8a6-7c4b82e27e0f#card` | Singing Bell Strike | C | ActivatedAbility |
| `8a5e925f-320d-4bb7-9374-0384e7ac41e7#card` | Sinking Feeling | C | ActivatedAbility |
| `400da7e6-43f8-4539-8959-ec60a3cc431b#card` | Siren Song Lyre | C | ActivatedAbility |
| `3a487f20-6b1d-4be3-b51f-0c8f0a611601#card` | Sisay's Ingenuity | C | ActivatedAbility |
| `25d35e3e-ece8-4c94-9dbc-d076a52fca0d#card` | Sixth Sense | C | OrdinaryAbility |
| `99e03fa0-8e0c-4b5d-880f-d7de201d5f41#card` | Skeletal Grimace | C | ActivatedAbility |
| `2a4cde8f-38e0-439b-9928-95ab23317f70#card` | Skeletonize | M | ActivatedAbility |
| `4ed9bccf-c655-426e-8192-65beab830fcd#card` | Skirk Ridge Exhumer | C | OrdinaryAbility |
| `cf890abd-b282-4dbb-ac86-9e7caec1c1b8#card` | Skittering Invasion | C | ActivatedAbility |
| `48533a2d-1bdb-40a6-939e-9c8337e0dc5a#card` | Sleep with the Fishes | M | OrdinaryAbility |
| `70ed2052-f03c-461d-b32a-8835c97c559f#card` | Snake Umbra | C | OrdinaryAbility |
| `9fe1f3ef-2c30-4ccc-9a67-5349e98e2502#card` | Song of Freyalise | C | ActivatedAbility |
| `5f4d0c87-7d59-41d5-9910-86765a0c144b#card` | Song of Totentanz | M | OrdinaryAbility |
| `8f0ab2fb-297a-4824-8d2c-0120eec09e5e#card` | Sound the Call | C | OrdinaryAbility |
| `d895df97-e7e7-4f15-a79b-0ce8f1a252d3#card` | Spare Dagger | C | OrdinaryAbility |
| `0fbec99f-0e49-4211-a06e-edd783cf47da#card` | Sparkspitter | C | OrdinaryAbility |
| `3641ceef-4f26-4647-ac12-c958524d68ca#card` | Spawn-Gang Commander | M | ActivatedAbility |
| `256a7f54-0e8e-4d22-a0f7-ff2830e8884e#card` | Spawnbed Protector | M | ActivatedAbility |
| `49e43de3-460b-4562-aef6-da43bd56debc#card` | Spawning Bed | C | ActivatedAbility |
| `82ce3682-0514-49a5-a330-2591e9789d36#card` | Spawning Breath | C | ActivatedAbility |
| `1961dd92-db0b-4f02-b9c8-08f760f4051b#card` | Spawning Grounds | C | ActivatedAbility |
| `b08d4381-1a75-4c59-9551-b3133f7495a3#card` | Spectral Sliver | C | ActivatedAbility |
| `b1305916-53cc-4021-897e-bbefc65dce78#card` | Springleaf Parade | C | ActivatedAbility |
| `2d584333-b25e-4291-a2c9-80c6d9f8732a#card` | Squirrel Nest | C | ActivatedAbility |
| `09071f5f-c9cb-43a9-b41d-faf6caf3beff#card` | Staggering Insight | C | OrdinaryAbility |
| `fa8ddc13-5108-4fe9-b3aa-dc28df476f7b#card` | Sticky Fingers | C | OrdinaryAbility |
| `6b1f750c-0f01-4b1c-ad8f-260e26fbbfea#card` | Stinging Hivemaster | M | OrdinaryAbility |
| `bd4810a4-8f86-4d1c-82f1-cbcd7e9479d6#card` | Storm the Citadel | C | OrdinaryAbility |
| `d4453b3e-733e-4038-81da-135a6a03079c#card` | Summoner's Grimoire | C | OrdinaryAbility |
| `d2da26d6-b0f7-4af0-9ce8-fb8d82fe2d85#card` | Sunfire Torch | C | OrdinaryAbility |
| `46569712-1f28-41cb-81be-fd938408c71f#card` | Sunken Field | C | ActivatedAbility |
| `65756e6d-9825-4ed6-91c4-79f1e3bd4ae5#card` | Supernatural Stamina | C | OrdinaryAbility |
| `f5eace41-0f53-4084-8bec-1e079c04287b#card` | Synapse Necromage | M | OrdinaryAbility |
| `b4cf61e5-b2b7-4fb9-addf-db84dec49681#card` | Take Flight | C | OrdinaryAbility |
| `e79b1224-1baf-460f-bf12-92b51cd8071f#card` | Talons of Falkenrath | C | ActivatedAbility |
| `fada8f16-85e8-44f1-9d2c-99da1e27851f#card` | Taunting Sliver | C | OrdinaryAbility |
| `31ec6a73-f742-4dc6-809d-28d3c461c2a0#card` | Telekinetic Sliver | C | ActivatedAbility |
| `536c36d9-5ef4-441c-a6c8-5806644d805c#card` | Tempered Sliver | C | OrdinaryAbility |
| `39068bf1-cae4-4ac9-82ef-4e81bf59446e#card` | The Girl in the Fireplace | M | OrdinaryAbility |
| `69b409b3-fa16-4c79-8b46-215a7036ed46#card` | The Tabernacle at Pendrell Vale | C | OrdinaryAbility |
| `70623348-7eda-4c68-89cd-2356b0896b79#card` | Thorncaster Sliver | C | OrdinaryAbility |
| `7bef91cd-df65-4135-86a6-d17b71470452#card` | Thranduil the Strategist | C | ActivatedAbility |
| `5102b7c8-6886-4004-9992-416d22b4fce7#card` | Tin Street Market | C | ActivatedAbility |
| `490df8a0-182a-40b8-a36e-f2666fa7c7fa#card` | Titan of Eternal Fire | C | ActivatedAbility |
| `b47f7f1e-2d81-4b1b-870d-4c1a4f020505#card` | Trash the Town | C | OrdinaryAbility |
| `17469f58-286e-4410-b717-5908d24eecdd#card` | Trickster's Talisman | C | OrdinaryAbility |
| `397df813-c759-4246-8def-122c7ff82d88#card` | Trollhide | C | ActivatedAbility |
| `3c032e74-69c6-48ca-95d4-51aa3d894c14#card` | Trusty Boomerang | C | ActivatedAbility |
| `dfaa02d4-4f3c-4e5d-b9e5-283a45395a53#face:1` | Turn Stones | M | OrdinaryAbility |
| `44f3241f-af82-4f2e-b15d-7f279bf6e816#card` | Umbral Mantle | C | ActivatedAbility |
| `d749d075-6135-486a-a94a-36c3142960d7#card` | Unbridled Growth | C | ActivatedAbility |
| `39c10798-9dfc-47a2-9da3-b4038509aebe#card` | Underworld Connections | C | ActivatedAbility |
| `8b061702-1bc3-45aa-a758-446b25034801#card` | Undying Malice | C | OrdinaryAbility |
| `63765157-e2ba-4cec-9501-8a405259de5b#card` | Unfathomable Truths | M | ActivatedAbility |
| `2772e3cc-10e0-420f-b183-71ebaf3580e1#card` | Urban Burgeoning | C | OrdinaryAbility |
| `77b73f11-a59a-4f2b-83d3-e6fe5064bc87#card` | Urban Utopia | C | ActivatedAbility |
| `9deced63-80d0-419e-a5ac-71b06ba1459e#card` | Urza, Chief Artificer | M | OrdinaryAbility |
| `917652c7-ac12-4cd7-ab2d-f2260a620d02#card` | Utopia Vow | C | ActivatedAbility |
| `448037c0-20b5-4ca4-9d8f-2702b4697127#card` | Vampiric Sliver | C | OrdinaryAbility |
| `4187fd6d-5660-4ea7-a9dd-402487ec8721#card` | Veiled Apparition | M | OrdinaryAbility |
| `ff59a95c-28de-44cf-abbe-772417851ffa#card` | Veiled Serpent | M | OrdinaryAbility |
| `f0bbd988-50b7-476c-9183-dfabd4a95079#card` | Verdant Embrace | C | OrdinaryAbility |
| `4d59fd2b-7fc6-49a7-8938-d3f08902a5a8#card` | Verdant Field | C | ActivatedAbility |
| `c35540ad-9ab1-4be9-b97d-603813394a26#card` | Verdant Rebirth | C | OrdinaryAbility |
| `a671f8ca-1177-4511-9d51-ca4ff12a5104#card` | Veteran's Armaments | C | OrdinaryAbility |
| `4a9d36eb-db72-459a-8de9-6d04a1d7563c#card` | Vibranium Strike Gauntlets | C | OrdinaryAbility |
| `bd803b1c-1370-40db-a6c6-abad4b3d1602#card` | Victual Sliver | C | ActivatedAbility |
| `b6a005db-e7e2-4b88-bc8a-c257bcc31cf5#card` | Vile Consumption | C | OrdinaryAbility |
| `8b9119b0-9948-497f-aa92-2879d07bed1c#card` | Viridian Longbow | C | ActivatedAbility |
| `7b1599c0-bb4f-48f8-a358-dc60166b13e7#card` | Vishgraz, the Doomhive | M | OrdinaryAbility |
| `c077f939-73a9-455f-8de5-385162121b94#card` | Voltaic Whip | C | OrdinaryAbility |
| `8d36d80e-9934-4b56-bf3c-2b18a6da3d2d#card` | Voracious Vermin | M | OrdinaryAbility |
| `61bfab3f-d8f0-4ff5-9ddf-3fd605070c8f#card` | Vorpal Sword | C | OrdinaryAbility |
| `cad63b31-667b-4518-bc49-be4ffb82bc63#card` | Vraska, Soul of Stone | M | ActivatedAbility |
| `cfa5b168-d112-4b92-ac42-8bba30f15d77#card` | Wake the Dragon | M | OrdinaryAbility |
| `a7736614-cdd3-43bd-ab3e-461e3e24ed40#card` | Warehouse Tabby | M | OrdinaryAbility |
| `0a994401-d37c-4e30-9410-422e7ae5a715#card` | Warped Tusker | M | ActivatedAbility |
| `ec528198-a38a-4ef7-9804-78a25c94ecc1#card` | Web-Shooters | C | OrdinaryAbility |
| `9ce5cc2f-e223-4c3b-adc1-c0109fd31762#card` | Weirding Wood | C | ActivatedAbility |
| `098bbee7-df65-4167-9c65-ffa14fc0c6cc#card` | Whipgrass Entangler | C | OrdinaryAbility |
| `d22196b5-b111-45ce-ae51-a06ca02533b6#card` | Wildfire Awakener | M | OrdinaryAbility |
| `c8d479b8-b96c-4382-97d3-68d85cf371ba#card` | Witches' Eye | C | ActivatedAbility |
| `c6fead73-0473-4b57-899a-9f8c5b4f896d#card` | Wolf's Quarry | M | OrdinaryAbility |
| `fdbac0a7-18c0-48a5-a174-0d4d1473fd9c#card` | Wrench | C | ActivatedAbility |
| `d27bebc9-348d-4b8e-816a-a4da66fb5618#card` | Writhing Chrysalis | M | ActivatedAbility |
| `393d0bf8-5f2e-4eaa-91a1-9132429a7371#card` | Zurgo, Thunder's Decree | C | OrdinaryAbility |

### Post-landing fix (2026-10-06)

The accepted review found 21 correct probes, zero lost faces and unchanged
Reading counts on the 15,404 faces covered before the original landing. This
follow-up removes the remaining dead grammar route: KeywordObject and
QuotedObject frames, the Keyword/Quoted FrameUse values and selected-frame-use
rows, and SharedKeywordComplement/SharedQuotedComplement schemas and instances.
No lexicon or plugins_v2 declaration supplies a head for either retired frame.
The canonical GrantedAbility Complement route remains.

Declaration counts are stamped before on start parent `nmtxxkuv` and after on
`qrkrqyzzlkrpmwmnopnvxmvrtysrxwnu`, each at 15,792 covered. Nonblank lines
(every non-whitespace line, including comments):
**2,868 → 2,842**, a decrease of 26. Total declaration lines: 3,155 → 3,127.
Ordinary Constructions remain 189; shared schemas decrease 45 → 43 (234 → 232
named declarations). Four category-instance rows and two instance groups are
removed. Assurance counts: 0 restored, 1 re-spelled, 1 added, 0 ignored,
0 removed and 0 weakened. The independent atomic-head fixture now selects the
canonical Complement(GrantedAbility), replacing the two retired Object
signatures while retaining exact value comparisons. A new rejection test checks
both retired signatures for finite and secondary heads. This addition beyond
the ticket follows from the derived gate finding the stale fixture; no grammar
scope is added.

DISCLOSE now records the Object → Complement relation change for every prior
keyword grant, reconciles the 388 gains with the earlier 396 surface-bucket
estimate at 13,716 covered, and names/routes the 13 unread nested single-quote
faces. The residuals ticket marks Gain/Have GrantedAbility landed, removes
Psionic Sliver's stale residual claim and owns the nested single-quote gap.

The exhaustive follow-up census is stamped on change `qrkrqyzzlkrpmwmnopnvxmvrtysrxwnu`,
covered **15,792**: 6,924 one-Reading and 8,868 multiple-Reading faces, 17,036
unread and zero undetermined. All **237,401 retained Readings** roundtrip
exactly, with zero issues, duplicate Readings or internal failures. Compared
with the original landing's final census, there are **0 gained, 0 lost and
0 changed per-face Reading counts** on all 15,792 previously covered faces.
The 13 named nested single-quote faces remain unread; none is counted as a gain.

Performance advisory on the same change (covered 15,792):
85,075,391,635 ns corpus wall time, 301,240 ns/B checked-text thread CPU,
workers 12, host load 8.536 / 6.993 / 7.053. The 85.08-second loaded-host run
exceeds the 16.26-second quiet-host advisory; another build was running.

Evidence lives in the workspace's ignored `target/english-v3/quotes-followup/`
and will be retained under the coordinator's ignored
`target/english-v3/english-v3-quotes-followup/`. `after.json`, `comparison.json`,
`declaration-counts.json` and `nested-single-quotes.json` hold the measurements
and exact face identities. No evidence file is tracked.

The derived gate from start parent `nmtxxkuv` runs:

```sh
cargo test -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask
cargo clippy -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask --all-targets -- -D warnings
```

The full command is `cargo xtask gate --changed --from nmtxxkuv --run --clippy`.
The derived test command passes: 675 passed, 0 failed and 1 existing ignored
test (`macro_schema_census_count_matches_21`, which cross-checks the live corpus
on demand). No ignore was added. Clippy and `cargo fmt --all --check` pass
(exit 0). From the workspace root, citation checks report 0 stale and
0 non-compliant strings; `kata kanban check` reports OK. The first derived
gate failed on the stale independent atomic-head fixture; re-spelling it and
adding the rejection witness above resolves that failure without weakening a
value comparison. No unresolved STOP or glossary gap remains.
