---
needs: []
---
# Read the existential clause: there are seven or more cards in your graveyard

## Why

Dummy *there* never reads. On claim parent `pvmzprszmnpt` (32,828 supported faces, 21,486 covered,
11,342 unread; wave-3 recon of 2026-10-07), *there is/are* touches **438** unread
faces and is the sole cause on **276** (recon bucket "there is/are"; it was 22
sole on the wave-2 recon, before the comparative-quantity, *as long as* and
cost buckets it co-occurred with were discharged). Counts are unread faces *touched* (at least one localised failing unit
matches) / *sole* (every failing unit matches and no other recon STRONG bucket
does). They are surface counts, not gain forecasts.

Probes (admitted roots): "There is a creature." 0; "There are two
creatures." 0; "There are no creatures on the battlefield." 0; "This creature
can't attack unless there is a Mountain on the battlefield." 0, while "… unless
you control a Mountain." 1; "Enchanted creature gets +3/+0 as long as there are
seven or more cards in your graveyard." 0, while "… as long as you have seven
or more cards in your graveyard." 1.

This ticket takes over from `english-v3-systemic-residuals` the existential
*there* host obligation recorded for Shadowborn Demon ("No implementation
ticket owns existential *there*").

## Goal

A finite clause with dummy *there* as Subject, *be* as verb and a displaced
Subject NP reads, with an optional locative PP after the displaced Subject
(*on the battlefield*, *in your graveyard*, *on this land*). It reads in every
host where an ordinary finite Clause already reads: main clause, *if*/*when*
conditional and temporal clauses, *unless*, *as long as*. Finite *be* agrees
in number with the displaced Subject (*there is a Desert card*, *there are
four or more card types*); the mismatched forms do not read.

## Analysis

Dummy *there* is a pronoun filling the subject position while the element
that would be subject in the basic version is displaced to post-verbal
position, as a **displaced subject** that is an internal complement of the
verb (CGEL, Ch. 16, §6.1, p. 1391, [3]). Existential clauses are defined
syntactically as the combination of *there* and *be* (Ch. 16, §6.2, p. 1392);
presentationals with other verbs (*There remain many problems*, p. 1390) are
not attested in these witnesses and are not added (pruning rule). The
agreement claim in the Goal is the ticket's requirement; consult Ch. 16 §6
for its CGEL basis before citing it.

## Witnesses

Each witness's failing units all match this construction and no other recon
STRONG bucket (`pvmzprszmnpt`, 0 Readings each):

- Sandstone Needle: "If there are no depletion counters on this land, sacrifice
  it." (control "If this land has no depletion counters on it, sacrifice it."
  1)
- Glacial Crasher: "This creature can't attack unless there is a Mountain on the
  battlefield."
- Kamahl's Desire: "Threshold — Enchanted creature gets +3/+0 as long as there
  are seven or more cards in your graveyard."
- Case of the Shifting Visage: "To solve — There are fifteen or more cards in
  your graveyard."
- Drop of Honey: "When there are no creatures on the battlefield, sacrifice this
  enchantment."

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
   Reading. *there* read as a locative Adjunct or as Subject of an
   ordinary predicate, the displaced Subject read as an Object, or a
   singular/plural mismatch that reads, is a defect. A wrong analysis that parses is a defect, not a gain.
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

- Presentational *there* with verbs other than *be*, and *there* raised under a
  catenative (*there seems to be*): unattested here.
- Relativised displaced Subjects (*everything there is*) and existential
  interrogatives.
- Other blockers on *card types among …* hosts (Delirium faces): measure them,
  do not build them here.

## Landing record

In addition to the standard record: the Subject/displaced-Subject structure
chosen and how agreement is enforced (declared features); Reading counts of
each witness and of the agreement-mismatch negatives; the residual item
discharged; before/after counts stamped with change ids; timings as integer ns
and ns/B with host load and worker count.


### Delivered analysis and provenance (2026-10-08)

Measured feature **`vqoxvykpqkoupsorotsrnuurwvwvwnov` / covered 22,307**;
claim parent **`zkmuyqxwxtpmotkunrottxrllwsllptl` / covered 22,088**. The reused full census
was measured on **`rtrzusnnwovvsvqynnsywtqxmmrmqwlu` / covered 22,088**. Every recorded
census source matches the claim parent except the Airbend semantic body;
`baseline-provenance.json` carries the earlier grammar/lexicon equivalence
proof for that body-only change (`xnxwytku`). The input SHA-256 is unchanged:
`49dc966bda6ef588fc68e8d6972de25df1b660ed2ea654904584698263007ebb`. V3 has no coverage lock; the full census supplies the
covered count. Evidence is under `target/english-v3/english-v3-existential-there/`.

`ExistentialClause` fills Subject with the invariant dummy Pronoun, retaining
its absence of inherent Person/Number. Its `displaced_subject` field is an NP
internal Complement of the verb; `AccusativePhrase` constrains Case, not the
Object function (CGEL, Ch. 16 §6.1, p. 1391; Ch. 4 §3.2.2, pp. 241–242).
The selected lexical frame has kind `Existential`, a **Complement** NP slot
and an optional **Complement** locative slot. Frame equality licenses the
whole signature; it does not name the verb. The optional extension is outside
the NP, while the existing nominal PP Modifier Reading survives (CGEL,
Ch. 16 §6.2.1, pp. 1393–1394). Number and Person equations correlate the finite
verb with the displaced NP. Present, preterite and clitic forms retain their
lexical Inflectional Form, Tense and Finiteness.

The strict Number agreement requirement, including clitic forms, comes from
this ticket's Oracle-English contract. CGEL Ch. 4 §3.2.2, p. 242 supports
inheritance for independent copular forms and separately reports informal
singular *there’s* with plural NPs; that informal variation is **not** being
attributed to CGEL as ungrammatical. Bare vs extended existential structure,
Case, and agreement inheritance are the claims the cited passages support.

The never-reading `vocab:Adverb/There` declaration is retired, not superseded (see Review notes). Dummy Subject Use
is declared on `vocab:DummyPronoun/There`, and both ordinary pronoun NP routes
exclude that declared Use. The general Adverb construction still serves
other adverbs; no there-specific grammar declaration remains unreachable.
The two existing copular lexemes receive one existential frame each. Their
old ordinary/auxiliary/locative/scalar frames remain reachable for their
existing uses. No other lexeme, frame, verb declaration or plugin body changes.

### PROVE: no silent loss and structural laws

| Full census | Claim-parent equivalent | Finished feature |
| --- | ---: | ---: |
| Supported faces | 32,828 | 32,828 |
| No Reading | 10,740 | 10,521 |
| One Reading | 7,540 | 7,540 |
| Multiple Readings | 14,548 | 14,767 |
| Exact Readings | 984,393 | 1,000,364 |

`comparison.json` compares **every face identity**, analyzed source hash,
exact Reading count and `readings_sha256`. There are **219 gains, zero lost
faces, zero removed Readings, zero increases on previously covered faces,
and zero same-count digest changes**. All **22,088** previously covered faces
retain their complete counts and digests. All **32,609** non-gain faces retain
their complete digests. No decrease is inherited from refresh: the imported
changes are Lean/checker work and coordinator claims; grammar, lexicon and
all English census source contents remain unchanged by that refresh.

The full run completely enumerates **1,000,364** Readings: **zero issues,
zero internal failures, zero duplicate Readings and zero cyclic derivations**.
Every counted Reading passes declaration admission, byte-exact realization,
lexical ownership, structural traversal and leaf identity validation.
Independently built bare and extended existentials additionally establish the
reverse constructed-value roundtrip for those witnesses, with exact value,
construction traversal and lexical traversal comparisons. Negative independent
values reject wrong agreement, a non-dummy pronoun, and an unlicensed frame.
Retained ambiguity replaces the superseded destructive-selection/zero-tie
requirement; no grammatical Reading is pruned by preference.

New word/card/construction-identity licensing guards: **0**. The seven new
requirements and two agreement equations read declared features, including
complete frame equality. Source loading succeeds; the **87 inherited mapping/
morphology diagnostics are byte-identical to the parent**. Nine inherited
unsupported frame signatures remain: obsolete `Object`, `MassNoun`,
`DistributionPhrase`, `VerbPhrase`, `FrameComplement` and
`ResultativeComplement` slot combinations are named in `source-diagnostics.json`.
The new existential signature adds none. These inherited debts remain with
`english-v3-systemic-residuals`. The v3 command does not emit the retired
parser's permitted-checker total; none is invented and the derived gate is gone.

### DISCLOSE: witnesses, ambiguity, scope and residuals

| Witness | Isolated witness Document Readings | Whole-face Readings |
| --- | ---: | ---: |
| Sandstone Needle | 2 | 10 |
| Glacial Crasher | 12 | 12 |
| Kamahl's Desire | 14 | 14 |
| Case of the Shifting Visage | 3 | 12 |
| Drop of Honey | 2 | 0 |

Every witness starts at zero on the claim-parent equivalent. All five isolated
witnesses read with dummy Subject / displaced Subject structure. Document
hosts also retain previously licensed higher PP attachments. The standalone
Drop of Honey condition has exactly **two** Readings: NP-internal PP Modifier,
or separate existential locative Complement. The eight agreement/finiteness
negatives in `agreement_mismatches_do_not_read` each have **0** Readings,
including singular/plural mismatches, plain *be*, first-person *am*, mismatched
*there’s*, and mismatched preterites. The ordinary-NP/Adverb and presentational
negative controls each have **0** Readings.

Shadowborn Demon's residual host is **discharged**: its full face has **3**
Readings, including displaced Subject *fewer than six creature cards* and
locative *in your graveyard*. The live systemic residual ticket is updated.

Drop of Honey remains unread as a whole because its pre-existing *If two or
more creatures are tied for least power, you choose one of them* clause has
**0** Readings; its least-power destruction and regeneration sentences read
with **4** and **1**, and its existential condition now reads with **2**.
Baba Lysaga, Night Witch remains **0**: *card types*, even without *among …*,
is independently blocked because *card* lacks declared noun-premodifier
permission. The Goal's *four or more card types* agreement example therefore
remains a pre-existing NP gap under the explicit out-of-scope host exclusion;
this landing supplies agreement for admitted NPs without widening that scope.
The attested *were* inflection remains declared. World at War also remains
**0** as a whole; its isolated *there’s an additional combat phase* reads with
**2**, preserving the compound-noun and noun-premodification analyses of
*combat phase*. The **254** remaining there-containing faces are named in
`remaining-there-faces.json` and routed to `english-v3-systemic-residuals` for
cause audit, including the explicitly deferred hosts. They are not gains or
regressions from this change.

The census is **7,540 one / 14,767 multiple** after, versus **7,540 one /
14,548 multiple** before. All 219 gains are multiple; no previously covered
face changes its ambiguity status. Specificity resolution is not an admission
stage. The new attachment distinction is `ExistentialExtension` versus
NP-internal `PostmodifiedNominal`; existing higher `PrepositionPredicate` /
`PrepositionClause` attachments remain. `gain-analysis-ledger.json` records a
selected retained value fingerprint and its actual Subject, verb, displaced NP
and extension for every gain. Selection here is an inspection choice, not
admission or preference pruning.

At least ten newly covered faces were inspected by name and constituent:
Excavating Anurid, Gran-Gran, Locked in the Cemetery, Sanguine Spy, Tourach's
Gate, Krosan Beast, Finest Hour, Shower of Coals, Tifa, Martial Artist, and
Psychic Rebuttal. Their precise analyses appear in the complete table below.
Every inspected Subject has the dummy-pronoun identity; no displaced NP is
read as an Object, no dummy as an Adverb/ordinary predicate Subject, and no
agreement mismatch is counted as coverage.

**Deviations and additions:**

- Added `ExistentialClause` and `ExistentialExtension`, one optional extension
  Category, one declared frame signature and one pronoun Use feature; these
  distinguish the required Subject/Complement functions with shared existing
  NP and locative grammar. No construction or frame is deleted.
- Added the attested contracted form, beyond the five listed full-form
  witnesses. No new spelling is invented. The corpus has nine *there’s*
  tokens and one *there were* token; unattested negative-copula, presentational,
  interrogative and raised-subject routes are not added.
- Seven tests beyond the five witness tests cover agreement exclusions,
  constituency, retired routes, contracted ambiguity/preterite vocabulary,
  independent bare and extended construction, and independent rejections.
  Assurance relative to the claim parent: **0 restored, 0 existing tests
  re-spelled, 0 ignored, 12 added, 0 removed**.
- During authoring, the direct verb slot exposed four unrelated copular frame
  choices as spurious duplicates. The code was corrected to require the new
  complete existential frame before the full census. The newly authored
  contracted-count expectation was corrected to retain the independently
  verified compound/noun-premodifier ambiguity; the newly authored Baba Lysaga
  positive expectation was localized to its already blocked NP and recorded
  as the excluded residual, rather than shipping it as a coverage claim.
  No pre-existing test is deleted or weakened.
- Filled glossary gaps: Dummy Pronoun, Displaced Subject, Existential Clause,
  Existential Use (project term), and Existential Extension. CGEL-backed
  definitions and project permissions are distinguished. No CR citation is
  introduced. **STOPs: none**; the spurious-frame defect was fixed within scope.

### REPORT: declarations, inventories, verification and performance

At `vqoxvykpqkoupsorotsrnuurwvwvwnov` / covered **22,307**: **204 ordinary
constructions + 50 schemas = 254**, **335 instance rows**, **717 compiled
Productions**, **148 Categories**, **3,530 nonblank declaration lines**.
Claim-parent equivalent `rtrzusnnwovvsvqynnsywtqxmmrmqwlu` / covered **22,088**:
202 ordinary constructions + 50 schemas = 252, 335 rows, 712 Productions,
147 Categories, 3,503 nonblank lines. The inherited tree already exceeds the
accepted decision's eventual economy ceilings; two explicit grammatical
functions are added, without treating those ceilings as a fitted coverage gate.
Observed construction names are **212 → 216**: the two new constructions plus
previously declared `GroupedScalarNumeral` and `LargeCount` newly witnessed on
gains. No additional declarations hide behind those observed names.

Both stamped trees have **38,404 spellings, 1,139 named homographs, 1,092
case-folded homographs, 23 form literals, zero literal/vocabulary overlaps**.
The complete named owner maps are `spellings.json`, `homographs.json`,
`case-folded-homographs.json`, `form-literals.json`, and
`form-literal-vocabulary-overlaps.json`. Homograph and literal lists are
byte-for-byte structurally unchanged. Only spelling ownership of *there* and
*There* changes, from the retired adverb to the dummy pronoun. The only lexical
fingerprint deltas are the two copular frame additions and that owner/category
replacement; no spelling is removed.

- `cargo test -p deckmaste_english_v3 -p deckmaste_lexical_source`: **418 English,
  23 lexical-source unit + 34 lexical-source integration passed**, zero
  failed/ignored. This runs both required package test suites.
- `cargo clippy -p deckmaste_english_v3 -p deckmaste_lexical_source --tests -- -D warnings`:
  passed. No other crate code or consumed data changed.
- `cargo xtask cite check --list-noncompliant`: **0 non-compliant**;
  `cargo xtask cite check`: **16,366 checked, 0 stale** after this record;
  the feature diff piped to `cargo xtask cite audit --diff`: **0 changed CR
  citation sites**. No citations were blessed.
- One full, non-incremental census, exactly
  `cargo xtask english-v3 --all --workers 12 --samples-per-face 0 --output target/english-v3/english-v3-existential-there/after.json`.
  Iteration used only the explicit 473-face manifest and isolated fragments.
  Every subset face's final count/digest matches the full census. No xtask
  tests, Lean checks, derived gate or second full census ran.

Performance advisory, integer ns and **12 workers**:

| Tree / covered | Setup ns | Corpus wall ns | Checked-text thread CPU ns/B | Host load (1/5/15 min) |
| --- | ---: | ---: | ---: | --- |
| `rtrzusnnwovvsvqynnsywtqxmmrmqwlu` / 22,088 | 2382352480 | 146674736352 | 428533 | [7.890625, 9.90087890625, 8.96240234375] |
| `vqoxvykpqkoupsorotsrnuurwvwvwnov` / 22,307 | 4162111218 | 99031374696 | 402729 | [10.681640625, 8.5498046875, 7.46826171875] |

Feature command wall is approximately **105113934457 ns**, measured from shell log
creation to its final report write, including Cargo and report serialization
(`command-wall-log-span.json`); setup + corpus stages report
**103193485914 ns**. Both corpus stages exceed the
**16,260,000,000 ns** quiet-host advisory ceiling. These loaded-host timings
are provenance and an advisory, never a gate or a claim of quiet-host compliance.

### Every newly covered identity and selected analysis

All **219** rows below are measured at `vqoxvykpqkoupsorotsrnuurwvwvwnov` / covered
**22,307**, against `rtrzusnnwovvsvqynnsywtqxmmrmqwlu` / covered **22,088**. Each
listed Subject is dummy *there*; the verb selects the displaced NP as an
internal Complement. “None” denotes a bare existential in this retained
Reading, possibly with an NP-internal or higher host PP. Full selected value
fingerprints and source text are in `gain-analysis-ledger.json`.

| Face | Durable identity | Readings | Verb | Displaced Subject NP | Separate locative Complement |
| --- | --- | ---: | --- | --- | --- |
| Aang, A Lot to Learn | `854278bc-2bcf-45dd-abd5-6aca7a1e5fc9#card` | 225 | 's | a Lesson card | None |
| Aboshan's Desire | `95db05eb-2c25-43f2-8e18-dfc64f490634#card` | 14 | are | seven or more cards | None |
| Anurid Barkripper | `2cdd85c8-fa60-44ca-bc96-d38f029d2184#card` | 14 | are | seven or more cards | None |
| Arnim Zola, Bio-Fanatic | `95161e23-e4c4-411f-bce5-58a02abc0958#card` | 72 | are | two or more creature cards | None |
| Aurelia, the Warleader | `0f5a3a09-2f07-4774-9e0f-e99d9a444166#card` | 16 | is | an additional combat phase | None |
| Aven Heartstabber | `fee73267-b0ef-4f43-a0bd-7728691d6f17#card` | 12 | are | five or more mana values among cards | in your graveyard |
| Barbarian Ring | `eeb9377b-72c1-4214-9a66-0f55577c17d1#card` | 14 | are | seven or more cards | None |
| Battlewise Aven | `f165e482-5b87-4062-87ac-bbe8303f74c9#card` | 4 | are | seven or more cards | in your graveyard |
| Billowing Shriekmass | `ce45d6bd-74e3-401b-983d-028d3fb942da#card` | 14 | are | seven or more cards | None |
| Bloodcurdler | `3f53fd3e-9116-4599-81df-a9390461b549#card` | 16 | are | seven or more cards | in your graveyard |
| Bloodsworn Squire | `a6375c5b-ec78-4ee8-b681-8d92797a5b43#face:0` | 6 | are | four or more creature cards | in your graveyard |
| Boneshard Slasher | `1098ff82-8cad-4283-8493-5492f12ea200#card` | 8 | are | seven or more cards | in your graveyard |
| Breath of Fury | `fdaa0faf-e7ee-48b3-9330-de0fc3792ad3#card` | 36 | is | an additional combat phase | None |
| Budoka Pupil | `6f571092-7244-4223-aa14-f5133cc000d8#face:0` | 8 | are | two or more ki counters | on this creature |
| Cabal Initiate | `2fc19fa3-c6c2-42c0-85fa-8d13e1baa145#card` | 28 | are | seven or more cards | None |
| Cabal Pit | `92392467-a22f-4dd7-a0eb-393bef956dc0#card` | 28 | are | seven or more cards | None |
| Cabal Ritual | `5b5bf1fa-6502-4790-b66b-f0f8504ebc7c#card` | 20 | are | seven or more cards | None |
| Cabal Torturer | `fbd295ee-545a-4eab-b000-c0b228dd412d#card` | 56 | are | seven or more cards | None |
| Calculated Dismissal | `d8e63065-9ebc-419d-b7af-5867fd81edff#card` | 12 | are | two or more instant and/or sorcery cards | in your graveyard |
| Callow Jushi | `7ceaddba-6301-45f8-86a1-ad4c6e6d3297#face:0` | 8 | are | two or more ki counters | on this creature |
| Case of the Shattered Pact | `d00a089b-7d0d-4b89-902b-7b914e892178#card` | 216 | are | five colors among permanents you control | None |
| Case of the Shifting Visage | `409697d5-ce3c-46aa-a97d-969d143a2a65#card` | 12 | are | fifteen or more cards | None |
| Centaur Chieftain | `3010ff0c-8ee6-4df3-96c1-a58096586469#card` | 44 | are | seven or more cards | in your graveyard |
| Centaur Garden | `5cf92fd4-7c0b-4d8e-92f1-53dc2e0476fc#card` | 28 | are | seven or more cards | None |
| Cephalid Coliseum | `c733873e-77db-471f-8061-139db24f7e7c#card` | 14 | are | seven or more cards | None |
| Cephalid Inkmage | `8d979a0b-ba04-4abb-8f78-c707ce15351a#card` | 60 | are | seven or more cards | None |
| Cephalid Sage | `29182a24-3bdb-4fd0-8a1d-a196340c0858#card` | 12 | are | seven or more cards | in your graveyard |
| Chainflinger | `cfaf4871-2157-495d-bca8-d169bd1f7ae3#card` | 14 | are | seven or more cards | None |
| Chakra Meditation | `12c57ebe-35a7-4cb3-936b-99c148a3cfed#card` | 960 | are | three or more Lesson cards | None |
| Childhood Horror | `29bebbaf-d47c-4369-8ff9-81816758da86#card` | 4 | are | seven or more cards | in your graveyard |
| Chlorophant | `ad80d9a8-187f-4ecf-94be-e23ac3a57179#card` | 4 | are | seven or more cards | in your graveyard |
| Cleansing Meditation | `658bccf8-fe73-4d6a-b37b-7a58034e5e5d#card` | 64 | are | seven or more cards | in your graveyard |
| Comforting Counsel | `5fd193c0-ed9f-4afb-b6cd-c1a20008fc0b#card` | 2 | are | five or more growth counters | on this enchantment |
| Crashing Centaur | `d13a9cb1-1c76-4ac1-80d8-3c471753a588#card` | 8 | are | seven or more cards | in your graveyard |
| Creakwood Safewright | `f9b7bbea-ea20-400b-a5b7-4d6993d8777b#card` | 30 | is | an Elf card | in your graveyard |
| Crypt Feaster | `4c22257e-8a00-40b7-a29b-55fcec7ddbd4#card` | 16 | are | seven or more cards | in your graveyard |
| Cunning Bandit | `3d416f83-32b8-4dad-8e4d-3acd182bbe44#face:0` | 8 | are | two or more ki counters | on this creature |
| Curious Homunculus | `c38db10f-c0b7-4391-b49a-77079a4d087c#face:0` | 6 | are | three or more instant and/or sorcery cards | in your graveyard |
| Cyclonus, Cybertronian Fighter | `7ead01b9-def3-4f9d-a860-af64e94cb45e#face:1` | 12 | is | an additional beginning phase | None |
| Dark Petition | `79acdb37-8d52-4bc8-a7be-a7e92b6166e5#card` | 42 | are | two or more instant and/or sorcery cards | in your graveyard |
| Dawnhand Eulogist | `57bbd80b-290f-461e-8dc6-26ba4d673fa9#card` | 9 | is | an Elf card | in your graveyard |
| Deathbonnet Sprout | `c92d6f26-5856-42dc-9ee5-21292d78a1fb#face:0` | 3 | are | three or more creature cards | in your graveyard |
| Deathbringer Regent | `71b5fdf2-4475-4102-85eb-9be164a9ae29#card` | 6 | are | five or more other creatures | on the battlefield |
| Decree of Silence | `920684d0-aef3-4408-a66b-a0c9394c58f4#card` | 6 | are | three or more depletion counters | on this enchantment |
| Deep-Sea Terror | `36775412-0280-4acc-bae8-3ea5d4e341be#card` | 12 | are | seven or more cards | None |
| Demoralize | `d0741810-0772-47f3-a709-4739fbf85576#card` | 24 | are | seven or more cards | in your graveyard |
| Desert's Hold | `a3639381-e401-409e-8f07-0a3bc736c50d#card` | 9 | is | a Desert card | in your graveyard |
| Dirty Wererat | `7fbd5f69-f6b4-4ad7-8e9e-47a798324f6c#card` | 4 | are | seven or more cards | in your graveyard |
| Dreadwing Scavenger | `bbfd0de0-3558-4833-bae4-8efae44b7fff#card` | 72 | are | seven or more cards | None |
| Edifice of Authority | `7c16707e-24da-4a7e-9d24-d3731987f675#card` | 147 | are | three or more brick counters | None |
| Elvish Reclaimer | `94f01534-7bb3-4b1e-8f77-e84156408686#card` | 126 | are | three or more land cards | None |
| Emet-Selch, Unsundered | `3b102ccc-7629-457c-aba6-e9b00fd50c85#face:0` | 6 | are | fourteen or more cards | in your graveyard |
| Epicenter | `264cb7f6-6d36-4e58-87f4-83ccaa998a32#card` | 46 | are | seven or more cards | None |
| Excavating Anurid | `0239b661-80be-4b52-b3b7-fca229192b58#card` | 4 | are | seven or more cards | in your graveyard |
| Exquisite Firecraft | `e83c617f-1e07-40db-9b12-ae01122a3e24#card` | 6 | are | two or more instant and/or sorcery cards | in your graveyard |
| Eye of Jace | `c8f802ad-4621-4f71-b52d-fbc55853dc88#card` | 6 | are | seven or more cards | in your graveyard |
| Faithful Squire | `2358ffc2-6663-4ef3-b3a4-b036a4733ac6#face:0` | 8 | are | two or more ki counters | on this creature |
| Far Wanderings | `2ef797ef-c05f-4309-85c7-1bd151dac3fa#card` | 2016 | are | seven or more cards | in your graveyard |
| Fiery Impulse | `27c84ad1-1491-459b-bf8a-0146062084dc#card` | 18 | are | two or more instant and/or sorcery cards | in your graveyard |
| Finest Hour | `05678253-a40f-4c11-8ac4-335b3a02532e#card` | 20 | is | an additional combat phase | None |
| Fire Nation Cadets | `5ebfa3f5-f80a-4845-bba7-75c4ddb88682#card` | 18 | 's | a Lesson card | None |
| First-Time Flyer | `cbbea4e2-4b36-4965-89b6-00d2a564f146#card` | 9 | 's | a Lesson card | None |
| Fledgling Dragon | `be007706-3da7-4dd3-84db-dbb5d8831bba#card` | 8 | are | seven or more cards | in your graveyard |
| Force Bubble | `655ae8e7-372b-4d8d-b33f-4aca46831abb#card` | 6 | are | four or more depletion counters | on this enchantment |
| Foreboding Statue | `d8c0f103-4658-4f4f-8429-c06bdd017846#face:0` | 8 | are | three or more omen counters | on this creature |
| Frightcrawler | `96b611e9-38cc-4326-bb38-73084e99e6f5#card` | 4 | are | seven or more cards | in your graveyard |
| Gaea's Courser | `5d13aa6a-5a04-45e0-bb33-daa707ffaf9f#card` | 3 | are | three or more creature cards | in your graveyard |
| Genji Glove | `8242cd78-6675-43b5-8924-0a88c3aa6f76#card` | 20 | is | an additional combat phase | None |
| Ghitu Lavarunner | `8e3f4987-6f31-4bc5-8bcd-6adb0440ee7c#card` | 3 | are | two or more instant and/or sorcery cards | in your graveyard |
| Gideon's Phalanx | `f9489831-a9a2-47ab-abae-2f389c9fae2c#card` | 126 | are | two or more instant and/or sorcery cards | in your graveyard |
| Gilded Cerodon | `93dd5f2e-aa13-4616-bfe6-466524b274ff#card` | 9 | is | a Desert card | in your graveyard |
| Gixian Skullflayer | `0b2a59a3-cf87-48fe-99e2-4c2063119b55#card` | 3 | are | three or more creature cards | in your graveyard |
| Glacial Crasher | `42441362-d638-423a-8928-ab3fd9b2d4f2#card` | 12 | is | a Mountain | None |
| Gloomdrifter | `cfc1156c-b787-4212-96e0-a8e3609069b7#card` | 12 | are | seven or more cards | in your graveyard |
| Godo, Bandit Warlord | `f890d51a-dc90-4179-8423-4738a72a527c#card` | 176 | is | an additional combat phase | None |
| Gorilla Titan | `9ce80c55-a995-4226-8506-901f6cf06efc#card` | 7 | are | no cards | None |
| Gran-Gran | `0259a9ed-0558-4183-b737-871ed5573c32#card` | 360 | are | three or more Lesson cards | None |
| Graveyard Shift | `da8a93c6-66e2-492d-8894-397ba9ec5a87#card` | 70 | are | five or more mana values | among cards |
| Gray Slaad | `788897f5-d49d-4294-8006-0009a3124ad0#face:0` | 3 | are | four or more creature cards | in your graveyard |
| Great Train Heist | `afe2f7a4-9440-4d93-801f-a18b627efb21#card` | 240 | is | an additional combat phase | None |
| Grim Reaper's Sprint | `99a6d7fe-40a1-4f2f-a3f6-6777884d5d1a#card` | 96 | is | an additional combat phase | None |
| Grizzled Angler | `d37effd3-d194-4609-9994-6d70038a68f9#face:0` | 4 | is | a colorless creature card | in your graveyard |
| Grizzly Fate | `f3073e4f-ca7e-4fde-88a4-bd7b6ebfbc8c#card` | 20 | are | seven or more cards | None |
| HYDRA Troopers | `6de67fad-0d01-443d-9ae0-4126ae903069#card` | 118 | are | two or more creature cards | None |
| Hellkite Charger | `d4f28a4b-d821-4132-bdec-4c528318f8e2#card` | 4 | is | an additional combat phase | None |
| Hexplate Wallbreaker | `f7f78825-10d6-441a-ae26-51b77cc271cd#card` | 20 | is | an additional combat phase | None |
| Hickory Woodlot | `26259c65-8f4e-42a6-b8a7-f65c36d35c4d#card` | 10 | are | no depletion counters | on this land |
| Hired Muscle | `ea31c9a4-af4f-4cab-b53c-77ef4241d4a6#face:0` | 8 | are | two or more ki counters | on this creature |
| Hour of Revelation | `920c1dd4-c3f6-4020-a6f0-2e8acad2c212#card` | 15 | are | ten or more nonland permanents | None |
| Hunting Grounds | `2f3fec87-3dd2-4e5d-835c-eae535da4ecb#card` | 8 | are | seven or more cards | in your graveyard |
| Igneous Elemental | `4dc8e91a-2c00-4de2-bada-80f323a4ec9d#card` | 15 | is | a land card | None |
| Illuminate History | `c7c1e7cf-0be2-4129-9bdf-0862be2fba7f#card` | 4 | are | seven or more cards | in your graveyard |
| Impending Disaster | `cf0799f3-ef64-4fcc-924a-5265e52c9727#card` | 8 | are | seven or more lands | on the battlefield |
| Infected Vermin | `448396b8-b691-457a-8569-1282c3fca65a#card` | 14 | are | seven or more cards | None |
| Jace, Vryn's Prodigy | `594f6881-c059-46f8-aa4e-7151d502de73#face:0` | 784 | are | five or more cards | in your graveyard |
| Kamahl's Desire | `6cbe54e0-5d62-4aad-ad9c-98b70defbf96#card` | 14 | are | seven or more cards | None |
| Kamahl's Sledge | `defa9b92-4b54-4b00-852e-6c5ec4015643#card` | 4 | are | seven or more cards | in your graveyard |
| Killmonger, Scourge of Wakanda | `1c3ba54a-a8ef-4838-b3aa-cb6df827a619#card` | 24 | are | two or more creature cards | in your graveyard |
| Kirtar's Desire | `4f21a664-96e3-49be-ba9f-4af4b6bd7475#card` | 24 | are | seven or more cards | None |
| Kirtar's Wrath | `4f66d82a-492f-4638-9f77-190d4a33ad7f#card` | 148 | are | seven or more cards | in your graveyard |
| Krosan Avenger | `8991b367-6fc1-4de7-a1ee-4edb57c44fc2#card` | 14 | are | seven or more cards | None |
| Krosan Beast | `042a665d-2f63-4f7e-ac89-0d560da68085#card` | 14 | are | seven or more cards | None |
| Krosan Restorer | `ea845951-0158-4646-acd9-3b0b28da40b1#card` | 14 | are | seven or more cards | None |
| Leaves from the Vine | `7c4abd72-29b0-4c82-9e67-b481cf7e8775#card` | 80 | 's | a creature or Lesson card | None |
| Lightning Surge | `dc55e69e-e1b8-4129-902c-c71bcb952418#card` | 12 | are | seven or more cards | in your graveyard |
| Locked in the Cemetery | `035f0aae-9c23-4b22-8c87-0d66b84b7962#card` | 12 | are | five or more cards | in your graveyard |
| Lux Artillery | `cbd76b22-d04e-48e7-bcab-c7f5466d67d8#card` | 22 | are | thirty or more counters among artifacts and creatures you control | None |
| Luxa River Shrine | `1eb64899-8865-48cc-bd58-529b1294ec99#card` | 7 | are | three or more brick counters | None |
| Lys Alana Dignitary | `ce7582a2-e014-41fb-8753-00d29cffc81a#card` | 27 | is | an Elf card | None |
| Magmatic Channeler | `382267da-c63c-432e-8a11-f803cf0e3a3a#card` | 36 | are | four or more instant and/or sorcery cards | in your graveyard |
| Mana Vortex | `98b88734-09e2-4209-a787-9b5b345391f9#card` | 24 | are | no lands | on the battlefield |
| Mask of Intolerance | `990d4798-3f59-462d-952c-777da5af1c41#card` | 20 | are | four or more basic land types among lands that player controls | None |
| Masked Gorgon | `f41b1a5b-bc99-4c51-9665-e251f04ad83f#card` | 84 | are | seven or more cards | None |
| Mazemind Tome | `800fb917-8898-4eba-9f94-aec6e9d75236#card` | 2 | are | four or more page counters | on this artifact |
| Metamorphic Wurm | `512477f0-1083-423b-9675-a8bc852f7d88#card` | 14 | are | seven or more cards | None |
| Mind Drill Assailant | `3b10ab39-3050-475c-af8b-f293794d498c#card` | 4 | are | seven or more cards | in your graveyard |
| Mindwhisker | `f823b122-94ac-42ba-b511-9e54d64db112#card` | 4 | are | seven or more cards | in your graveyard |
| Moraug, Fury of Akoum | `6288c0cb-cf32-4ae9-abf0-104ec71654fa#card` | 72 | 's | an additional combat phase | None |
| Most Decrepit Old Bird | `34d34f2c-06ff-4acf-8f2f-ff867774b777#face:0` | 14 | are | seven or more cards | None |
| Murasa Behemoth | `0dfdeaa0-fa12-49cf-b361-efa49d3ffcb2#card` | 9 | is | a land card | None |
| Mystic Crusader | `937d663d-c9a7-4312-a4cc-fcb54faa92bc#card` | 4 | are | seven or more cards | in your graveyard |
| Mystic Enforcer | `cf3b79cc-ce97-48e9-a578-625e323d1b62#card` | 4 | are | seven or more cards | in your graveyard |
| Mystic Familiar | `9d3931a2-baba-4e6e-b923-44df00900e51#card` | 4 | are | seven or more cards | in your graveyard |
| Mystic Penitent | `d3d59704-569d-42a7-8b31-8a31b9c0395f#card` | 4 | are | seven or more cards | in your graveyard |
| Mystic Visionary | `bde74bcc-41b8-4014-b28e-2f6326e20799#card` | 14 | are | seven or more cards | None |
| Mystic Zealot | `5a47b99d-19dd-4830-89ee-35f4a74aa656#card` | 4 | are | seven or more cards | in your graveyard |
| Najeela, the Blade-Blossom | `09619943-6aec-4080-ace5-a0c6ebb23f1c#card` | 40 | is | an additional combat phase | None |
| Nantuko Blightcutter | `c40bd057-d1be-45e2-9ebf-4ab176195f17#card` | 88 | are | seven or more cards | None |
| Nantuko Calmer | `d4409b15-b9f8-4bdb-9857-5cd70a0d3193#card` | 14 | are | seven or more cards | None |
| Necromantic Summons | `2f921740-de98-4276-8196-47a38a1e4591#card` | 432 | are | two or more instant and/or sorcery cards | in your graveyard |
| Nightwhorl Hermit | `99007c40-1eb5-4c3a-9889-6d12417d8191#card` | 8 | are | seven or more cards | in your graveyard |
| Nimble Mongoose | `1059f089-2693-4c89-92c5-39fe49beebb3#card` | 14 | are | seven or more cards | None |
| Nomad Decoy | `73c40477-7061-47db-af8f-2a5cf6c22c9b#card` | 14 | are | seven or more cards | None |
| Nomad Stadium | `4034bec6-e3c7-4d3f-81df-7c903977a606#card` | 14 | are | seven or more cards | None |
| Nut Collector | `4e1bc03c-e131-4422-beba-0a45a26fdf43#card` | 14 | are | seven or more cards | None |
| Olórin's Searing Light | `e6f2ba06-2c92-48b7-a95f-ea6bf35aa0fe#card` | 2622 | are | two or more instant and/or sorcery cards | in your graveyard |
| Pardic Arsonist | `9ece4b78-9554-4e4e-8491-68a6091f9c6a#card` | 4 | are | seven or more cards | in your graveyard |
| Path to the Festival | `e54d4ecf-ebcc-4374-98e7-c682f83c82a9#card` | 140 | are | three or more basic land types among lands you control | None |
| Peat Bog | `a176924c-78fc-4151-b2b5-1547b1114a40#card` | 10 | are | no depletion counters | on this land |
| Persistent Marshstalker | `77eb76cf-a361-44e0-aeee-e8ef4b695799#card` | 16 | are | seven or more cards | in your graveyard |
| Planar Collapse | `82bccc9f-f700-4f1f-b89e-bea63d973161#card` | 8 | are | four or more creatures | on the battlefield |
| Portcullis | `6230c179-0263-4fa4-b0ff-f2cb621de147#card` | 48 | are | two or more other creatures | on the battlefield |
| Possessed Aven | `1a5f1670-3e71-4379-90ee-a883c8dc6070#card` | 4 | are | seven or more cards | in your graveyard |
| Possessed Barbarian | `ee51365b-9f36-44a7-8706-ae5b2f86bd29#card` | 4 | are | seven or more cards | in your graveyard |
| Possessed Centaur | `2fd1707b-917c-4842-a5c6-dfffbedcf918#card` | 4 | are | seven or more cards | in your graveyard |
| Possessed Nomad | `9bf9870e-32a3-494c-bd42-f9f013463619#card` | 4 | are | seven or more cards | in your graveyard |
| Profane Procession | `a656ad7f-133f-4d93-919a-43bcf1f815f3#face:0` | 2 | are | three or more cards exiled with Profane Procession | None |
| Proft, Sinister Mastermind | `a350856c-87f2-4605-999f-4e26aa362a55#card` | 48 | are | seven or more cards | None |
| Psychic Rebuttal | `08dd393f-12d8-4801-882c-6d76009b4f12#card` | 450 | are | two or more instant and/or sorcery cards | in your graveyard |
| Puca's Eye | `499b4cf9-2aaa-45a0-8362-77b727b7f7cc#card` | 18 | are | five colors among permanents you control | None |
| Putrid Imp | `7192ff07-9a84-49ad-a40c-ce255ce21544#card` | 8 | are | seven or more cards | in your graveyard |
| Pyramid of the Pantheon | `efc66685-94cf-4d3f-82e0-9967c6ab2e5c#card` | 7 | are | three or more brick counters | None |
| Quest for Renewal | `cba4ab80-09e8-4868-a082-a9a3bade9571#card` | 16 | are | four or more quest counters | on this enchantment |
| Quest for Ula's Temple | `434807c5-5948-4181-a46c-2c834d0a2f0b#card` | 140 | are | three or more quest counters | on this enchantment |
| Raiyuu, Storm's Edge | `9028f726-50c7-49f3-8386-e584415dcba2#card` | 80 | is | an additional combat phase | None |
| Ramunap Hydra | `a2dadaf5-eaa1-4c43-9e21-bb2334ceb918#card` | 18 | is | a Desert card | None |
| Rancid Earth | `c1972a20-5308-4ee4-b234-751b805d30e4#card` | 12 | are | seven or more cards | in your graveyard |
| Raphael, Tag Team Tough | `5a7f09d3-e4a8-4987-a731-a68fc748277d#card` | 1120 | is | an additional combat phase | None |
| Reborn Hero | `5562b4bd-1485-47d4-a087-d93dafc37272#card` | 36 | are | seven or more cards | in your graveyard |
| Reclusive Taxidermist | `7a0168c5-e258-4c50-96aa-95deb2198e6f#card` | 9 | are | four or more creature cards | None |
| Remote Farm | `2c38f4c7-1b3f-42b4-a175-edab7acd6cc6#card` | 10 | are | no depletion counters | on this land |
| Repentant Vampire | `a5dabc98-0db9-4b4f-b2d2-f74dbd022d22#card` | 8 | are | seven or more cards | in your graveyard |
| River Serpent | `d6f8c69f-e603-41f5-8f25-da8a102f7c8d#card` | 12 | are | five or more cards | None |
| Sand Strangler | `c429716b-bd3a-4060-b8a3-82e3d6aecd88#card` | 3 | is | a Desert card | in your graveyard |
| Sandstone Needle | `c8e0a1a5-8188-4677-9d8a-a18eb593343a#card` | 10 | are | no depletion counters | on this land |
| Sanguine Spy | `03af4b10-900e-4013-bd5d-dd8d132d50fb#card` | 4 | are | five or more mana values among cards | in your graveyard |
| Saprazzan Skerry | `e4e6e796-39ce-4a63-8c61-c7c956d75d78#card` | 10 | are | no depletion counters | on this land |
| Sarcomancy | `bd1e93da-7524-4bdd-9738-accd76bda7f7#card` | 2 | are | no Zombies | on the battlefield |
| Savage Beating | `89d1cd45-8148-4396-a9b8-e0f495e94b5b#card` | 24 | is | an additional combat phase | None |
| Send to Sleep | `9e3fd1e9-7db6-40de-b1de-cd8cc9f60590#card` | 48 | are | two or more instant and/or sorcery cards | in your graveyard |
| Seton's Scout | `7b2a32ca-ef5b-40a5-acf7-aefac1d09e5b#card` | 14 | are | seven or more cards | None |
| Sewer Crocodile | `b01e9ff2-865a-401e-a839-568cd4a0451d#card` | 120 | are | five or more mana values | among cards |
| Shadow of the Second Sun | `2ee9d018-02c7-43e7-ba2e-95e36d8ac0e8#card` | 6 | is | an additional beginning phase | None |
| Shadowborn Demon | `7dacc5aa-ea4d-414e-8324-6a7d24bb7671#card` | 3 | are | fewer than six creature cards | in your graveyard |
| Shadows of the Past | `a5ba544f-b223-47c8-a408-b00512772107#card` | 9 | are | four or more creature cards | None |
| Shauku, Endbringer | `49478593-29ff-4e5a-b1e8-6462c2d6176d#card` | 24 | 's | another creature | None |
| Shoreline Looter | `c15cad77-e70f-42cc-a9f5-50e1fd8c9c96#card` | 144 | are | seven or more cards | None |
| Shower of Coals | `08261613-1af3-4bd7-9a4d-a366fd508d5f#card` | 60 | are | seven or more cards | None |
| Sidewinder Naga | `92247856-35a4-4c1d-90bc-fa0d28524ecd#card` | 3 | is | a Desert card | in your graveyard |
| Sigil of Myrkul | `ae7e14cf-a275-4260-8d78-324429a042b7#card` | 36 | are | four or more creature cards | in your graveyard |
| Silver Seraph | `bcc7e32a-9cf9-4fdf-a0a4-e90ae42f1353#card` | 28 | are | seven or more cards | None |
| Snooping Newsie | `347a5faa-5656-4a36-af44-ab5a3ed267f4#card` | 4 | are | five or more mana values among cards | in your graveyard |
| Solitary Camel | `2eac0c6a-b289-4b75-9571-59869e62d180#card` | 19 | is | a Desert card | None |
| Spelleater Wolverine | `a3b226b9-f149-4578-882b-77447608380a#card` | 9 | are | three or more instant and/or sorcery cards | None |
| Sphinx of the Second Sun | `516101be-be39-4d84-8fee-d8a79930dd0a#card` | 6 | is | an additional beginning phase | None |
| Spirit Mirror | `42e041c0-ffbc-4bf8-a5aa-f0d14aad7112#card` | 3 | are | no Reflection tokens | on the battlefield |
| Springing Tiger | `2f8c4805-694d-4408-8eb5-9fb7ce278ded#card` | 14 | are | seven or more cards | None |
| Stitch Together | `bc3d5911-3580-4132-9daf-2826495b5739#card` | 300 | are | seven or more cards | None |
| Swimmer in Nightmares | `64ca54ee-1502-4fae-b919-7e4457e641c0#card` | 49 | are | ten or more cards | None |
| Swirling Sandstorm | `dc5cb229-82c9-4a87-9a10-490350cbdfc0#card` | 34 | are | seven or more cards | None |
| Syndicate Infiltrator | `0d6be8a4-b05b-40fb-a653-5aabf71809e0#card` | 4 | are | five or more mana values among cards | in your graveyard |
| Tainted Indulgence | `6944092f-22c6-4b65-840d-8c1b70883127#card` | 24 | are | five or more mana values | among cards |
| Take the Bait | `5d39cce9-afa1-422a-8e6c-8f50c710a681#card` | 480 | is | an additional combat phase | None |
| Teroh's Vanguard | `32b0fc43-b5ce-4869-a780-946289305e5a#card` | 12 | are | seven or more cards | in your graveyard |
| Tersa Lightshatter | `9918eb7b-054e-4b1c-afc7-1163ab642d50#card` | 120 | are | seven or more cards | in your graveyard |
| The Incredible Hulk | `b6c0ee14-2028-4e8b-9acd-ff78821bca9b#face:1` | 20 | is | an additional combat phase | None |
| The Lion-Turtle | `fff728a1-56f5-4e97-91e2-19aba5772909#card` | 37 | are | three or more Lesson cards | None |
| The Millennium Calendar | `1f250443-8d5e-46c9-920e-8e9373780e32#card` | 32 | are | 1,000 or more time counters | on The Millennium Calendar |
| The Warring Triad | `df0b5995-117b-4ba8-964d-ea3c592621ef#card` | 2 | are | fewer than eight cards | in your graveyard |
| Thermal Blast | `52ad1a31-3ad8-4ead-ba77-675cad7e4a14#card` | 20 | are | seven or more cards | None |
| Tidecaller Mentor | `e75e40a5-a9ae-4789-96c1-0e19d1ce59c5#card` | 8 | are | seven or more cards | in your graveyard |
| Tifa, Martial Artist | `0836c8d1-c453-4a3a-a52a-eb66839b8542#card` | 240 | is | an additional combat phase | None |
| Tourach's Gate | `03e27ebd-6141-4002-b41f-0bf9d6ec2908#card` | 32 | are | no time counters | on this Aura |
| Toxic Stench | `9c3be33b-0be9-49ec-8160-898cbcab8814#card` | 8 | are | seven or more cards | in your graveyard |
| Treacherous Vampire | `1d33d49f-ba12-4bf6-878b-04cee5d5b48d#card` | 48 | are | seven or more cards | in your graveyard |
| Treacherous Werewolf | `59681998-c395-4d37-a419-2a78cf894506#card` | 4 | are | seven or more cards | in your graveyard |
| Undercover Skrull | `33b1379f-b329-42cd-92ed-4811d44ed8dd#card` | 3 | are | two or more creature cards | in your graveyard |
| Unholy Hunger | `ed8d8eb5-fbc8-4b7f-9b75-53ab118e72de#card` | 6 | are | two or more instant and/or sorcery cards | in your graveyard |
| Unquenchable Thirst | `5f55d41a-ef93-4d4f-ab55-13ea887e7af2#card` | 18 | is | a Desert card | in your graveyard |
| Veteran Survivor | `cc5b1276-d13e-4392-988d-1315bc90a9b1#card` | 192 | are | three or more cards exiled with this creature | None |
| Vigilant Sentry | `9bb66d53-a83c-4313-91c3-4df09c248558#card` | 8 | are | seven or more cards | in your graveyard |
| Wall of Forgotten Pharaohs | `7ab15fcd-8371-4b3c-b534-e9ef44249213#card` | 19 | is | a Desert card | None |
| Walltop Sentries | `fb3a0910-a582-41ef-b5b9-3cda1f1cd5ce#card` | 3 | 's | a Lesson card | in your graveyard |
| Wayward Angel | `bf169a7a-e3cf-44b2-83eb-c2597bcf22d9#card` | 4 | are | seven or more cards | in your graveyard |
| Webweaver Changeling | `c8cf9387-54df-45b1-ad8d-9de57ed8916b#card` | 3 | are | three or more creature cards | in your graveyard |
| Werebear | `96cf3320-c730-4618-8768-28a179f4d9c0#card` | 14 | are | seven or more cards | None |
| Wretched Camel | `525d1562-4fb6-4df3-ab6d-ab6a94c866e0#card` | 3 | is | a Desert card | in your graveyard |
| Y'shtola Rhul | `a6a7bf77-0560-4572-a826-3bc9df1f78d1#card` | 444 | is | an additional end step | None |
| Éomer, Marshal of Rohan | `b2d95950-18b3-463f-94f4-299e420751dc#card` | 8 | is | an additional combat phase | None |

### Review notes (2026-10-08)

- **Locative *there* is retired, not superseded.** CGEL, Ch. 16 §6.1, p. 1391
  treats locative *there* as a separate word (an intransitive preposition)
  that can co-occur with dummy *there* (its [1]). The old
  `vocab:Adverb/There` never read (`VPFinalAdjunct: No`,
  `ClauseInitialAdjunct: No`), so no Reading was lost, but nothing replaced
  it either: dummy *there* is a different lexeme. Locative *there* is attested
  on about 38 sentences among the 254 remaining *there* faces (a regex count
  over `remaining-there-faces.json` finds 36 sentences on 35 faces whose only
  *there* is locative), e.g. Wreck Hunter, The Fourteenth Doctor and Bringer
  of the Last Gift ("…put there from the battlefield"). It is routed to
  `english-v3-systemic-residuals`; no lexeme declares it today.
- **219 gained vs the 276 sole-cause estimate.** The 276 figure was the recon
  bucket's sole-cause estimate; 219 faces were gained and **254** faces
  containing *there* remain unread (`remaining-there-faces.json`). Named
  blockers: Drop of Honey — its *If two or more creatures are tied for least
  power, you choose one of them* clause (0 Readings); Baba Lysaga, Night Witch
  — *card types* (the *card* noun-premodifier permission gap); World at War —
  the host outside the existential: *After the second main phase this turn,*
  and the *followed by an additional main phase* postmodifier each leave the
  sentence at 0 (with *After this main phase,* and no *followed by* clause the
  existential sentence reads 4), and its *At the beginning of that combat,
  untap all creatures that attacked this turn.* sentence reads 0.
  The remaining-faces file carries no blocker field, so this tally is from
  probes of a 10-face sample (every 25th face by id), run with the record's
  `probe-documents` binary against the current lexicon on 2026-10-08
  (classified by first localized blocker; untracked, not a corpus figure):

  | Blocker class | Faces | Sample |
  | --- | ---: | --- |
  | *card types among cards …* (card noun-premodifier, Baba Lysaga's gap) | 4 | Dusk Feaster, Moldgraf Scavenger, Hand That Feeds, Pick the Brain |
  | Undeclared counter-kind nouns (*ribbon*, *plot*, *tower*); the same sentences read with *depletion* | 3 | Prize Pig, Deadly Designs, Helix Pinnacle (also blocked on its *100 or more … you win the game* host) |
  | Locative *there* plus host (*extra turn*; *if you cast it from your hand*) | 2 | Gerrard's Hourglass Pendant, Myojin of Grim Betrayal |
  | Existential host outside the clause (*if it's your main phase, there is … followed by …*) | 1 | All-Out Assault (its bare *There is an additional combat phase after this phase.* reads 2) |

- **Three structures for a locative existential.** *There is a creature on the
  battlefield.* has three Readings: the PP as extension Complement, as NP
  Postmodifier, and as the pre-existing ClausalPreposition Adjunct. All three
  are retained under `docs/decisions/english-lexical-analysis.md`.
- The `crates/deckmaste_english_v3/src/declarations.rs` comment (now ~:1807)
  "an internal Complement, not an Object" overstates CGEL p. 1391, which says
  only that the displaced NP is an internal complement that is not a subject.
  Code comment left as is (docs-only landing); flagged for the next touch.
- *There is also me to consider* (CGEL p. 1392) is blocked by the
  person-agreement equation. Out of scope; no corpus witness requires it.
