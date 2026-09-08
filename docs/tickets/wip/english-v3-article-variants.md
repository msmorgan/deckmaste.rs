---
needs: [english-v3-lexical-measures]
---
# Admit indefinite articles with onset and capitalization constraints

Replace the blanket indefinite-determiner exclusion in v3
`DeterminedNounPhrase` with general countability, Number and realized-onset
constraints. Complete grammatical capitalization admission using the lexical
values' retained surface variants. The `kkmxslkn` baseline has 19,516 failed
faces containing article-shaped text; this is an overlapping observation,
not a count of failures caused solely by articles.

Pinned shape: derive onset from the first pronounced constituent, including
premodifiers, numerals, names and bound/multiword forms. Normalize declared
pronunciation/defaults and overrides at the lexical realization boundary and
carry the effective value through grammatical summaries. Initial orthographic
letters alone are not onset authority. Preserve the chosen `a`/`an` and casing
variants in generated values; checked construction and parsing enforce the
same compatibility relation. Retain correlated onset/variant alternatives
through packing and any deferred admission. Update the Oracle English Onset
glossary entry's v2-specific wording to the established v3 ownership.

Capitalization depends on the actual sentence, keyword-line, quote and bound
word position. Preserve exact names and lexical identities; do not lowercase
the input or infer quote casing solely from closing punctuation. A bound
quality and keyword are cased as one word. Keep lexical alternatives available
independently before grammar checks their distribution.

Witnesses: Opt for a clause-final indefinite count noun; Krark-Clan Ironworks
for a vocalic article in an action cost; Viridian Joiner for an indefinite
measure noun; Animate Dead for a keyword subject at top level and inside a
quote; Ogre Marauder and Takklemaggot for contrasting quoted starts; Secret
Tunnel, Prisoner Zero and Three Dog for numeral/name casing. Fetch supported
Oracle text and retain each relevant constituent as a focused test even when
another part of the card still fails.

Acceptance pins positive and negative pairs for `a creature`/`an artifact`,
wrong article/onset combinations, plural/mass misuse and the first pronounced
modifier changing the required article. Include declared pronunciation
overrides that disagree with first-letter spelling. Test sentence starts,
ordinary interiors, keyword lists and both quote cases, preserving valid
homographs. Test independently constructed article/variant values, byte-exact
rendering, analysis identity and total traversal. The baseline synthetic
`Draw a card.` fails while `Draw the card.` and lowercase `draw the card.`
parse; use these as separate article and positional-casing discriminators.

Apply [Lexical analysis](../../decisions/english-lexical-analysis.md#lexical-analysis),
[Source and roundtripping](../../decisions/english-lexical-analysis.md#source-and-roundtripping)
and [Chart admission and ambiguity](../../decisions/english-lexical-analysis.md#chart-admission-and-ambiguity):
keep Earley parsing, packed partial/complete derivations and declaration-driven
checked constructors, renderer and traversal. Both roundtrip laws and all
grammatical Readings are required. Update Lean before changing its grammatical
judgments. STOP and report rather than remove the exclusion without its
replacement constraints, name words in admission, echo source, construct eager
AST products or discard valid Readings. V2 remains untouched; unrelated
document/quote gaps route to `english-v3-systemic-residuals`. Standard
constraints apply.

## Landing record

Measured change `vnzvnyxo`, parent claim `vpqklzyq`; final refresh was a no-op.
The baseline implementation is `lolyrvsq`: the intervening changes through the
claim affect tickets/documentation only. The v2 coverage lock remains **19,953**.
This landing completes local article and casing admission; it does not claim
whole-Document recovery for the nine motivating faces.

### Requirements and independent evidence

| Requirement | Implementation and evidence |
|---|---|
| Indefinite count, Number and onset agreement | `IndefiniteNounPhrase` requires singular determiner, singular count Nominal, and agreement between the selected article's `article_onset` and the Nominal's realized `onset`. Ordinary determiners retain their existing Construction. `articles.rs` independently constructs both casing forms of the valid article values and rejects wrong onset, plural and mass uses. |
| First pronounced constituent | Lexical normalization supplies bounded pronunciation defaults and spelling-keyed overrides. Generated summaries compose the first constituent in declared surface order, including premodifiers, numerals, multiword identities, bound forms and optional/repeated fields. Unknown first onset stays unknown. Explicit sign Constructions declare consonantal onset. `pronunciation.rs`, `articles.rs` and the compiler `surface_features.rs` test defaults, overrides, numeric/sign contrasts and bound-word composition. |
| Correlated alternatives | Onset and casing capabilities enter partial states and complete summaries; the forest remains packed and AST materialization stays lazy. Independently constructed `a xenic artifact` and `an xenic artifact` select different homographic lexical owners whose pronunciations differ. Neither owner is discarded lexically; the generated agreement selects the compatible article/owner pair. |
| Positional capitalization | The connected grammar declares `capitalization Positional`. Sentence, cost, keyword-line, ability-word and modal boundaries require initial capability. Quote fragments require interior capability; quoted Documents retain sentence boundaries. Repeated/bound constituents permit an initial realization only at the beginning of the whole word/phrase. Names retain declared case and identity. Generic grammars opt into this distribution policy explicitly. |
| Quotes independent of final punctuation | `QuotedClause` admits a lowercase quoted Clause; `QuotedKeyword` admits a lowercase keyword with or without an internal period. The existing quoted Document retains uppercase sentence/keyword boundaries. Tests distinguish `"this creature can't be blocked"`, `"Draw a card."`, `"flying."`, and their invalid casing counterparts. No quote rule branches on source text or a card name. |
| Both roundtrip laws, identity and traversal | Independent article, modifier, homograph, name and compiler-fixture values compare exact complete Reading sets after realization and parsing. Generated realization performs checked admission and independent lexical boundary validation. Whole-corpus validation separately compares byte-exact realization, lexical leaves and Construction traversal against materialization traces for every counted Reading. No source buffer is retained in a Reading. |
| Lean first | `Features.leadingOnset`, lexical pronunciation declarations, article constraints and `Casing` refine admission. `ArticleWitnesses` supplies inhabited article environments, wrong-onset exclusions, modifier onset and distinct casing boundaries; `GrammarWitnesses.interior_initial_auxiliary_excluded` pins the new negative. Existing concrete admissions retain their outcomes after casing proofs were supplied. |
| Ownership and terminology | Effective pronunciation belongs to `deckmaste_lexical`, shared finite features to `deckmaste_lexical_model`, and compositional admission to generated grammar. The Onset glossary no longer assigns v3 behavior to v2. V2 implementation and lock are untouched. |

Supported Oracle text was checked against the pinned Scryfall snapshot and card
lookups. Prisoner Zero is a token named in **The Eleventh Hour**, not a separate
card entry. All nine selected full Documents remain No Reading with zero
mechanical issues. Focused evidence and residual ownership are:

| Witness | Relevant tested constituent; remaining scope |
|---|---|
| Opt | `Draw a card.` succeeds independently; `Draw the card.` remains positive and `draw the card.` is now negative. The Scry/reminder body remains outside this ticket. |
| Krark-Clan Ironworks | `Sacrifice an artifact` succeeds as an action CostComponent, with lowercase start and capitalized interior negatives. The independent `Add {C}{C}.` complement gap remains. |
| Viridian Joiner | `an amount of {G}` succeeds as a NounPhrase. The complete comparative/possessive body remains unresolved. |
| Animate Dead | `in a graveyard` succeeds; generic lowercase quoted keywords preserve the same casing distribution. Consuming the complete Enchant subject, at top level and inside quotes, belongs to `english-v3-keyword-labels`; this landing does not claim that consumer. |
| Ogre Marauder | The exact lowercase quote `"this creature can't be blocked"` succeeds; `a creature` succeeds. The remaining `of their choice` host is unresolved. |
| Takklemaggot | Its uppercase quoted start is retained in the `At the beginning` constituent. Generic quoted sentence admission is tested separately. The complete `At the beginning of that player's upkeep` host remains blocked by possessive composition, so this is not a claim that the full named quote parses. |
| Secret Tunnel | `Two target creatures` retains the initial Cardinal variant. The complete relative/coordination body is not discharged. |
| Prisoner Zero / The Eleventh Hour | An independently supplied exact catalog identity roundtrips as `Prisoner Zero`; lowercase is rejected. The workspace source currently has no contextual token-name provider, so full-card recovery is not claimed. |
| Three Dog, Galaxy News DJ | The full catalog name, `an Aura`, and an independently supplied exact `Three Dog` identity roundtrip. The shortened self-reference is absent from the current provider; no ad hoc runtime name row was added. |

Except for the explicit keyword consumer owner, these remaining source,
possessive and document/complement gaps route to the cause audit in
`english-v3-systemic-residuals`. That ticket mints bounded repairs; they have
not been folded into this implementation. None is a newly lost identity.

### Corpus reconciliation and audit

The baseline and final reports use Scryfall JSONL SHA-256
`ba4952fd7eae58b49f5dd8277968169902f79f8a289644c1ba2b4376cc53bb22`.
All 32,568 identities and source hashes match. The final lexical inventory hash
is `188894255b0986d3c2efbd7a27daf59afa1f3b24cfc28077f5e89f4dc09bc5ee`;
pronunciation metadata changes it without adding or removing lexical values.

| Tree / field | No Reading | One | Multiple | Mechanical issues |
|---|---:|---:|---:|---:|
| `lolyrvsq`, text | 30,875 | 1,223 | 470 | 0 |
| `vnzvnyxo`, text | 30,376 | 1,437 | 755 | 0 |
| `vnzvnyxo`, Type Line | 0 | 32,568 | 0 | 0 |

**Covered-to-uncovered identities: empty.** There are **499** newly covered
identities, listed below, and **zero** reading-count changes among previously
covered faces. All 4,838 retained baseline sample fingerprints are present in
the final sets. The old artifacts did not retain every tree for 85 faces, so
that fingerprint comparison is not claimed as a complete old/new set equality.
No new Construction occurs in any Reading of a previously covered face.

Complete enumeration retains **15,860** rules-text Readings, including **10,100**
on the newly covered faces. All newly covered identities contain the new
indefinite Construction. Audit inspected all 499 raw source texts, all 126
introduced Nominal-shape/lexical-owner inventory entries, and representative
full parent structures, including Avatar of the Resolute, Perplexing Chimera,
Highcliff Felidar, Well-Laid Plans, Golden Ratio, Chain of Smog and Ashes of the
Abhorrent. No confirmed new syntactic invalidity was found. This is not a claim
that every attachment expresses a card's intended game meaning.

The largest new counts are Avatar of the Resolute (4,944) and Perplexing Chimera
(1,792). Nominal/NounPhrase/predicate/Clause attachment choices and grammatical
Number alternatives remain retained. Ellipsis retains local form/voice and
leaves its discourse antecedent unresolved, as explicitly ruled in the
`english-v3-whole-grammar-activation` ticket; this audit does not silently add an
antecedent-selection requirement. There is no specificity selection or
preference filter, and no discarded multiple-Reading result.

Partial/complete packing telemetry on `vnzvnyxo` (lock 19,953):
15,043,036 items/intermediate nodes; 15,104,871 intermediate edges;
793,163 completed nodes; 811,439 completed families; 15,916,310 total families;
2,012,441 completion work; 2,170,418 lexical alternatives; 3,307,695 lexical
projections; 500,511 materializer builds. Cyclic derivations, duplicate Readings,
internal failures, limited enumerations and undetermined faces are all zero.
These are aggregate corpus measurements, not an unbounded complexity guarantee.

### Deviations, additions and validation

Three Constructions were added: `IndefiniteNounPhrase`, `QuotedClause`, and
`QuotedKeyword`; none was removed. The two quote fragments are necessary to
express the ticket's distinct quoted starts without inferring case from final
punctuation. Existing sentence/cost/label/modal and sign Constructions acquired
declarative surface obligations. Compiler support includes an explicit module
capitalization policy and finite surface summaries for both chart helpers and
checked collections. Pronunciation normalization contains lexical defaults;
grammatical admission contains no word-named guard or handwritten licensing
callback. There are 155 feature requirements and 14 agreement equations.

Rust tests: **9 added**, zero existing tests removed, re-spelled, restored or
newly ignored. Lean: **11 explicit theorems added**, 52 existing theorem blocks
updated for the refined admission, zero removed. Two generalized helpers now
require the needed interior capability (`clause_valid` for its auxiliary and
`pair_admitted` for its right constituent); every existing concrete witness
still has its original outcome. No test was weakened to evade a failure.

During validation, enforcing Oracle casing in the intentionally unconstrained
generic compiler slice exposed a scope mismatch. Positional policy became an
explicit grammar declaration, and the existing generic tests were kept intact;
a separate independent fixture proves the enabled policy. Clippy's new compiler
findings were repaired by ordinary condition simplification and parser/domain
helpers. No unresolved STOP or confirmed new invalid Reading remains. The
named full-card failures above are disclosed residuals, not green tests of
unsupported full Documents.

The reverse-dependency gate derives these packages from the changed paths:
`deckmaste_lexical_model`, `deckmaste_construction_v3_core`, `deckmaste_lexical`,
`deckmaste_lexical_source`, `deckmaste_construction_v3`, `deckmaste_english_v3`,
`xtask`. **506 tests passed, zero failed, one preexisting ignored**. Strict
all-target clippy passed for the same derived closure. Lean's 50-job build and
`english/scripts/axioms` passed: **3,066 theorems, zero disallowed axiom uses**.
Nightly formatting checks passed. Citation checks report zero noncompliant
strings and zero stale citations; the diff adds no citation sites. The lexical
inventory independently checks all **38,661** values; its complete
lexeme inventory matches the baseline. The full homograph inventory remains
**877** spellings (named list in the ignored report archive); introduced and
retired homograph lists are empty. Form-literal/vocabulary overlaps are empty.
The final grammar has **154 Constructions and 57 Categories**.

### Performance and reproduction

All timings below are stamped `vnzvnyxo`, v2 lock 19,953, one worker. Rules text
uses **25.01 seconds** of corpus wall and **152,017 ns/B** of checked-text thread
CPU, at host load 5.68/4.00/3.22. Type Lines use **2.95 seconds** and
**4,365 ns/B**, at load 4.52/3.89/3.21. Rules text exceeds the older 16.26-second
advisory. The user clarified on 2026-09-08 that anything under one minute is
acceptable while lower remains preferred; no performance-driven reading filter
or unrelated optimization was added. Setup and report serialization are outside
the corpus-wall metric. These measurements retained every tree for inspection.

Reports and audit aids are ignored, under
`data/reports/english-v3/vnzvnyxo/`; they are reproducible evidence, not admission
inputs. Commands (choose output paths outside tracked source):

```sh
cargo xtask gate --changed --from vpqklzyq --clippy --run
english/scripts/axioms
cargo build --release -p xtask --bin cargo-xtask
target/release/cargo-xtask english-v3 --all --workers 1 --samples-per-face 1000000 --output /tmp/article-final-text.json
target/release/cargo-xtask english-v3 --all --field type-line --workers 1 --samples-per-face 1000000 --output /tmp/article-final-type-lines.json
target/release/cargo-xtask lexical --card-name Opt --workers 1 --output /tmp/article-lexical-report.json --export /tmp/article-lexicon.ron
```

### Newly covered identities

Every row was No Reading before this landing. The last column displays one
newly admitted article-head structure, using lexical owner suffixes for
readability; it is an inspection representative, not a selected-only result.
All other article hosts, attachments, lexical values and complete trees remain
in the report. Counts include every Reading.

| Face | Identity | Readings | Representative indefinite head |
|---|---|---:|---|
| Abandon Attachments | `82333385-631f-4abf-b159-bb367f1c6fd9#card` | 8 | `Noun(Card)` |
| Abyssal Specter | `51833cff-6519-4806-8d91-0040e9a02189#card` | 3 | `Noun(Card)` |
| Accursed Centaur | `b1e6921e-e460-4e58-82b0-d98ee31c279d#card` | 1 | `Noun(creature)` |
| Aether Flash | `a3c35742-e306-49b6-b042-db4f685c6f86#card` | 4 | `Noun(creature)` |
| Airborne Aid | `7f715f72-5444-4ab3-a6ca-3d5bfd0d4851#card` | 18 | `Noun(Card)` |
| Ajani's Welcome | `4a782bf9-4051-4613-8852-33b0d85a0edd#card` | 4 | `ObjectRelativeNominal(creature, You, core-verb:Control)` |
| Akki Lavarunner | `47795817-73e5-4af6-bd1e-d69b193e8e9e#face:0` | 3 | `Noun(Opponent)` |
| Alaborn Zealot | `0d0d97dd-2653-4d37-b475-ac3b50c5ee53#card` | 1 | `Noun(creature)` |
| Alchemist's Apprentice | `de085c51-2555-450e-a479-ddbb3bea9e8c#card` | 1 | `Noun(Card)` |
| Alchemist's Vial | `f574bd9f-4246-42cb-bf4e-d2888daa44c9#card` | 1 | `Noun(Card)` |
| Altar of the Brood | `c3aafcdd-c890-4971-b8a9-5bfcad794c0b#card` | 2 | `Noun(Card)` |
| Angel's Feather | `3d0d4ba4-d5aa-4d9f-89ab-46687637fecf#card` | 2 | `Noun(Player)` |
| Arcane Encyclopedia | `d91ea728-2d69-42cb-bb5d-e2b058f0d7b1#card` | 1 | `Noun(Card)` |
| Archivist | `d137586f-83b0-40af-8100-443460b07ac0#card` | 1 | `Noun(Card)` |
| Arena Athlete | `4dfea3bf-dfd1-4abd-a17e-f3a07488a069#card` | 4 | `Noun(Opponent)` |
| Armorcraft Judge | `d7f49243-a96e-499f-b2d7-8e9842432420#card` | 370 | `Noun(Card)` |
| Arms Dealer | `8ebc4198-7317-4ffb-b8b8-14733c2077ff#card` | 3 | `Noun(goblin)` |
| Army Ants | `c39112ab-ee1f-4f00-b18b-692b5fe32b80#card` | 1 | `Noun(land)` |
| Artificer's Epiphany | `ed10bb4c-f7ef-4046-8dde-465041b55078#card` | 2 | `Noun(Card)` |
| Ash Zealot | `ab9f03b4-eb67-4709-904d-abf4d491f0ee#card` | 12 | `Noun(Spell)` |
| Ashes of the Abhorrent | `a75d5b54-5cc9-49c4-8e8c-3bef22d4c01c#card` | 398 | `Noun(creature)` |
| Ashiok's Adept | `80eca17e-5dfd-4051-9f94-0c8aaf3747bf#card` | 4 | `Noun(Card)` |
| Aura Blast | `4e3c3bdc-667d-42ec-b960-159a39c53cb3#card` | 1 | `Noun(Card)` |
| Aura Fracture | `3495d83a-b103-42be-8708-9ce971b352bd#card` | 1 | `Noun(land)` |
| Avatar of the Resolute | `15fd66db-f9dd-40ab-92bc-9e0575bd7489#card` | 4944 | `SlashModifiedNominal(1, 1, Counter)` |
| Azure Mage | `bbf9fd47-10e1-4749-b06f-4fe72200afc7#card` | 1 | `Noun(Card)` |
| Baleful Strix | `37688720-03de-4eca-a82d-a0afe8d58adc#card` | 1 | `Noun(Card)` |
| Ballynock Trapper | `96232342-3ec7-4740-a114-969537b8f2fc#card` | 8 | `PremodifiedNominal(White, Spell)` |
| Banisher Priest | `9f560b83-32d4-4bb4-a956-8f5db18599db#card` | 2 | `Noun(Opponent)` |
| Bard, Heir of Girion | `b0e88bd6-50a3-48d7-a15f-ec51f6bced0d#card` | 8 | `Noun(Card)` |
| Bargain | `a0dd88f6-6e36-40ce-bac2-a0db2b0117b6#card` | 2 | `Noun(Card)` |
| Barrage Ogre | `d556c71e-ce16-4229-880b-744790f93797#card` | 3 | `Noun(artifact)` |
| Barrage of Expendables | `da7600f6-5d4c-46bf-936a-9e0f0d508e19#card` | 3 | `Noun(creature)` |
| Benalish Heralds | `09ee1332-741f-4c55-abc3-8bcff9031cd9#card` | 1 | `Noun(Card)` |
| Benthic Criminologists | `5539d78c-1acc-46dd-ba6d-cc0cdad6ff44#card` | 8 | `Noun(artifact)` |
| Blaster Mage | `e5b35b54-77d9-4cb0-86a1-efc8e49aea09#card` | 1 | `Noun(Card)` |
| Blasting Station | `3a38d2d1-c4ff-4088-b1df-5feb9602ee2e#card` | 6 | `Noun(creature)` |
| Blood Rites | `69757aaf-182e-41a0-a5b4-4404e4c81c45#card` | 3 | `Noun(creature)` |
| Bloodied Ghost | `1195de01-f0ee-42c1-8e9e-a4bb90dd4932#card` | 9 | `SlashModifiedNominal(1, 1, Counter)` |
| Bloodrage Brawler | `8603e2d4-93f3-491a-91e7-a187fc6db599#card` | 1 | `Noun(Card)` |
| Bola Warrior | `13f41253-364f-40a5-a7cd-84d8785b2460#card` | 1 | `Noun(Card)` |
| Book of Rass | `a5e3ca74-ec33-4679-9366-8b1a45048783#card` | 1 | `Noun(Card)` |
| Brass Secretary | `0d8c2d7b-7cce-4115-a3f8-18c23731544e#card` | 1 | `Noun(Card)` |
| Brimstone Trebuchet | `d8dd601f-3615-4cef-b837-52329cb361f8#card` | 6 | `ObjectRelativeNominal(knight, You, core-verb:Control)` |
| Burglar Rat | `2f807301-37df-4724-871a-08e3512b07b3#card` | 1 | `Noun(Card)` |
| Bushmaster, Coiled Henchman | `21067a25-f749-4b79-bc6d-820a91af2834#card` | 48 | `SlashModifiedNominal(1, 1, Counter)` |
| Buzz Bots | `434e720f-2bfa-49b6-a5ac-fe0c0b24764d#card` | 1 | `Noun(Card)` |
| Cackling Fiend | `2029954b-6fa7-40d7-bb19-d7534c62be5d#card` | 1 | `Noun(Card)` |
| Caltrops | `b13f97b3-56ae-4131-a93b-b162c8ff7cca#card` | 4 | `Noun(creature)` |
| Canker Abomination | `4440e41c-042f-4e13-925a-c1e40b77e010#card` | 48 | `Noun(Opponent)` |
| Captivating Unicorn | `22005b01-84b1-4d4b-9193-3c561bd422ce#card` | 2 | `Noun(Opponent)` |
| Carnage Altar | `aa05900f-0f04-407e-931c-fea8f91e78e3#card` | 1 | `Noun(creature)` |
| Carven Caryatid | `bddf5e9a-7c41-45a8-8fa1-18093c408e15#card` | 1 | `Noun(Card)` |
| Cathartic Adept | `44d30168-8203-4a8e-a11f-b4a068231e55#card` | 1 | `Noun(Card)` |
| Centaur Veteran | `bd4c9726-c7bb-4da4-99e5-e98d174d6a52#card` | 1 | `Noun(Card)` |
| Cephalid Scout | `0ee042ce-7cb2-47a8-9a48-6ffc07ba07b3#card` | 1 | `Noun(land)` |
| Chain of Smog | `ea14c26b-bf2f-48b4-b879-6e63069ded1f#card` | 7 | `PremodifiedNominal(New, Target)` |
| Charging Strifeknight | `b0b9a7c1-f515-4627-a050-b595d6641611#card` | 1 | `Noun(Card)` |
| Chrome Prowler | `6d16c2fa-4118-4b81-92ca-6c8e9efe03b6#card` | 1 | `Noun(Opponent)` |
| Chronicler of Heroes | `c3779271-1186-4754-8b8d-0ee18e585bb6#card` | 104 | `Noun(creature)` |
| Circuit Mender | `1665ca9f-176d-40f1-a4e9-42da4f1236e9#card` | 2 | `Noun(Card)` |
| Claws of Gix | `c4d384d7-f294-4b2d-9971-a4689c150255#card` | 2 | `Noun(Permanent)` |
| Cleanup Crew | `12e0321b-1cd7-4519-8929-3261cf11a1a9#card` | 8 | `Noun(Graveyard)` |
| Cloudkin Seer | `09bd4e1c-9861-481f-80dc-4de955c8d3af#card` | 1 | `Noun(Card)` |
| Collective Unconscious | `38abf769-e4ef-4b47-9c79-c8cda0b90f6c#card` | 10 | `Noun(Card)` |
| Compulsion | `3fafb6b2-5cae-45b6-8550-3ff8daa02802#card` | 1 | `Noun(Card)` |
| Confessor | `b78973fa-a4cb-4927-8a75-6b1c963c1c0c#card` | 2 | `Noun(Card)` |
| Confound | `ca8a0e81-f885-47ee-82bb-fb25ec61bb1c#card` | 1 | `Noun(creature)` |
| Consecrate | `20e7a93f-77ce-466b-8586-35d390689d0c#face:0` | 4 | `Noun(Card)` |
| Consecrated Sphinx | `311a449d-dc74-46e6-9a47-6a597931f736#card` | 2 | `Noun(Card)` |
| Contemplation | `fa7efcef-a688-4e25-a823-4d53b2e96508#card` | 8 | `Noun(Spell)` |
| Contradict | `a48f259a-99a0-49c7-a11b-6e796f8a12f4#card` | 1 | `Noun(Card)` |
| Cornered Crook | `4477eeab-a51a-4dc2-9c97-83f2551d8b54#card` | 32 | `Noun(artifact)` |
| Corrupt Court Official | `ec84201d-757f-4a7a-b3a2-85ccd5ee81ae#card` | 1 | `Noun(Card)` |
| Corrupted Harvester | `f105ccdd-65bc-4880-bbfe-f57fc0e060ea#card` | 1 | `Noun(creature)` |
| Council of Advisors | `ff427657-173a-4845-89af-6f0a93467130#card` | 1 | `Noun(Card)` |
| Court Street Denizen | `34312179-f3f9-47ca-bdf4-5abbd493876e#card` | 4 | `Noun(Opponent)` |
| Cremate | `c6a2e410-b182-48d1-aeb2-bc8de27e9cd2#card` | 4 | `Noun(Card)` |
| Crook of Condemnation | `afc8df6d-7291-400e-b768-0c0c3e3fdc32#card` | 4 | `Noun(Graveyard)` |
| Crowned Ceratok | `5cdeb1fe-9ce3-4088-a35c-dcfb762f76d8#card` | 18 | `SlashModifiedNominal(1, 1, Counter)` |
| Crypt Creeper | `00b3971b-5bf7-4a3f-9607-6265f9af9098#card` | 4 | `Noun(Graveyard)` |
| Cyclopean Giant | `d02427fa-3ef0-484f-acad-63a1d5218727#card` | 1 | `Noun(swamp)` |
| Daemogoth Titan | `a52e73bb-47cb-4912-9b81-4ae40641eabb#card` | 1 | `Noun(creature)` |
| Daring Sleuth | `69ca69c7-403b-4b5d-be08-d9dce8ce410e#face:0` | 2 | `Noun(clue)` |
| Dark Heart of the Wood | `c44f40da-867e-4237-b4b1-ed6feb1f37b7#card` | 2 | `Noun(forest)` |
| Darkslick Drake | `cef11518-ecb2-4250-996c-a03fa9dfb09e#card` | 1 | `Noun(Card)` |
| Deadapult | `5e0fea29-0fd5-4535-b1df-cd66e50662cc#card` | 3 | `Noun(zombie)` |
| Deadbridge Shaman | `a57389ad-7f2a-47a6-8ae6-2821503cbb28#card` | 1 | `Noun(Card)` |
| Deadeye Tormentor | `ab735456-f831-4988-84c6-50ca14351990#card` | 2 | `Noun(Card)` |
| Debt to the Kami | `c6c1e576-7197-4067-8cd1-aa535188e1b8#card` | 1 | `ObjectRelativeNominal(creature, They, core-verb:Control)` |
| Deep Sight | `702e871d-90d8-4468-8f69-5ae42af2c9d3#face:1` | 2 | `Noun(Card)` |
| Defiant Thundermaw | `5c7f02ad-1daf-4d1a-bef0-0b2064f9b67e#face:1` | 8 | `ObjectRelativeNominal(dragon, You, core-verb:Control)` |
| Delta Bloodflies | `8a64cf8e-ae7f-4697-93fc-9c946eb1088b#card` | 24 | `Noun(creature)` |
| Demon's Horn | `ee38377f-8d3a-402e-a5d7-a01cfe3a5322#card` | 2 | `Noun(Player)` |
| Depose | `c547c9ab-a303-48fb-9579-37f9de9a558b#face:0` | 1 | `Noun(Card)` |
| Desperate Castaways | `0432e07d-baf6-4c20-8646-08338c7dc950#card` | 2 | `Noun(artifact)` |
| Dinosaur Hunter | `5c58f2eb-2617-4d10-a393-6e4a9eddd1dd#card` | 3 | `Noun(dinosaur)` |
| Discerning Peddler | `a6584af7-583d-4d89-9b46-d61be8236413#card` | 8 | `Noun(Card)` |
| Dismiss | `5c828a5e-10ae-4f63-86fd-2f160af43cc3#card` | 1 | `Noun(Card)` |
| Dismissive Pyromancer | `f040310d-d6ac-4d77-9f76-ca325eddf306#card` | 3 | `Noun(Card)` |
| Dosan's Oldest Chant | `59aeeb18-d263-426f-aebc-0f687b09801b#card` | 2 | `Noun(Card)` |
| Dragon's Claw | `bd8d707d-1201-42c6-bce6-fbda12ce5ea8#card` | 2 | `Noun(Player)` |
| Dress Down | `c1cbbda0-e02b-4b9c-8669-ea87ca629520#card` | 2 | `Noun(Card)` |
| Drowned Secrets | `cab305d9-27fb-4a41-adf1-892e15167cb9#card` | 4 | `PremodifiedNominal(Blue, Spell)` |
| Early Winter | `fbf2653e-9cc4-442f-bc73-6e2ba6e067cb#card` | 1 | `ObjectRelativeNominal(enchantment, They, core-verb:Control)` |
| Earsplitting Rats | `6c14ecbf-0097-4fd8-97a3-a32b532c21e0#card` | 1 | `Noun(Card)` |
| Earthblighter | `86b8fdad-0025-4b6f-8527-23686820a5cd#card` | 1 | `Noun(goblin)` |
| Earthshaker Dreadmaw | `454cbc6a-b7f0-445e-842c-5db267917a18#card` | 22 | `Noun(Card)` |
| Ebon Drake | `db4242ab-fdf3-487a-bc77-42bd2ef973c3#card` | 2 | `Noun(Spell)` |
| Eidolon of Blossoms | `77ccbea1-70af-4194-adad-39a904221c75#card` | 2 | `Noun(Card)` |
| Elderfang Disciple | `8db63f1a-0d63-4b11-966d-818d83a082f9#card` | 1 | `Noun(Card)` |
| Elite Guardmage | `92dfeeb2-1117-422b-87ea-08a589f1134b#card` | 2 | `Noun(Card)` |
| Elite Skirmisher | `66578806-9148-479d-80e1-6c3e3b67e452#card` | 8 | `SubjectRelativeNominal(Spell, That, Target, This, creature)` |
| Elusive Tormentor | `78b86f23-650b-4347-917c-3ee8c6007e4e#face:0` | 1 | `Noun(Card)` |
| Elven Raft-Steerer | `d0f62148-1015-49b8-b61f-a4332325a6b0#card` | 4 | `Noun(Opponent)` |
| Elvish Doomsayer | `5fba553e-81b9-419b-aa20-b2a7cfcd042b#card` | 1 | `Noun(Card)` |
| Elvish Visionary | `c6a3a882-a127-4590-93d7-679ef4313efe#card` | 1 | `Noun(Card)` |
| Embraal Gear-Smasher | `01e81249-6c45-4fae-86ee-9ef5fbec99ec#card` | 3 | `Noun(artifact)` |
| Etherium Astrolabe | `8d7194f7-1525-4f6e-b974-b947a254f437#card` | 1 | `Noun(artifact)` |
| Eumidian Terrabotanist | `92226fd2-ad93-4722-89a0-ca88ea03e1b4#card` | 4 | `ObjectRelativeNominal(land, You, core-verb:Control)` |
| Eunuchs' Intrigues | `e34f8aae-bf55-4bba-9524-c841591bb14b#card` | 2 | `ObjectRelativeNominal(creature, They, core-verb:Control)` |
| Excavation | `9f7b2ec6-a828-499b-a2ac-766bdaaf30c1#card` | 1 | `Noun(land)` |
| Execute | `24d2cbf6-6268-4da3-8413-6c76679576fe#card` | 1 | `Noun(Card)` |
| Exiled Boggart | `8c07878f-348c-4fd9-955f-87e975ed61fa#card` | 1 | `Noun(Card)` |
| Exultant Cultist | `e802442e-6a78-4c15-9034-fc8872470208#card` | 1 | `Noun(Card)` |
| Faerie Miscreant | `dd2510cd-356c-4d5a-8e5d-e44872eba60b#card` | 2 | `Noun(Card)` |
| Fairgrounds Warden | `388b7b0f-b26d-4de9-bfa6-8c3cbcc2e284#card` | 2 | `Noun(Opponent)` |
| Fanatical Devotion | `34e56412-4649-481d-be51-357b331d0b75#card` | 1 | `Noun(creature)` |
| Farsight Mask | `c82091b2-f2f1-4a85-80d6-91ab218f0a94#card` | 12 | `Noun(Card)` |
| Fate Unraveler | `7d66f67d-3148-4636-ac5a-2cf8a51d5e50#card` | 4 | `Noun(Card)` |
| Fateful Discovery | `72c6174c-fa2c-4ff2-b76e-eb6e890e97c9#card` | 2 | `Noun(Card)` |
| Fell Specter | `d655a1c9-b84d-4069-a2ca-3aa0dacab3e1#card` | 1 | `Noun(Card)` |
| Feral Prowler | `5bebba2d-867f-4ed1-a308-904fac32c8b6#card` | 1 | `Noun(Card)` |
| Filigree Familiar | `b544f690-e4bf-4a5b-984d-9256518fd574#card` | 2 | `Noun(Card)` |
| Fissure Wizard | `e3474632-6d81-43a8-9329-4c7e0c631755#card` | 8 | `Noun(Card)` |
| Flame Channeler | `ca8950ce-43d9-47bd-85f3-58c1fad42420#face:0` | 2 | `ObjectRelativeNominal(Spell, You, core-verb:Control)` |
| Flow of Ideas | `dd397a32-06b0-4530-b075-412e2881aa53#card` | 10 | `Noun(Card)` |
| Fodder Cannon | `aaf171bd-a4bb-4ce4-836a-da193c94f42e#card` | 3 | `Noun(creature)` |
| Forced Fruition | `448b27a5-7c0c-4ab6-bddf-bd62b920aacc#card` | 1 | `Noun(Spell)` |
| Foul Spirit | `d8177d08-a603-42e9-9350-890eac05e184#card` | 1 | `Noun(land)` |
| Fragment of Konda | `6ba8e099-5871-4ee3-8a65-06fd8ccec340#face:1` | 1 | `Noun(Card)` |
| Friendly Teddy | `02a8b849-e708-427c-92fe-9a41bb0ec7fe#card` | 1 | `Noun(Card)` |
| Futurist Forge | `0e5f97b5-bb8c-4688-a141-5db4d4eeccf0#card` | 1 | `Noun(Card)` |
| Gallant Citizen | `2fcb1c1c-6157-4955-b899-e8a8b337d188#card` | 1 | `Noun(Card)` |
| Gatekeeper Gargoyle | `a175de44-3a7e-4165-8f9a-8173c0df13d6#card` | 96 | `SlashModifiedNominal(1, 1, Counter)` |
| Generous Stray | `5722a13f-7d70-438c-94ba-97a3f53fbc5c#card` | 1 | `Noun(Card)` |
| Geth's Grimoire | `ef809e99-34a2-4471-8269-f56bf8037686#card` | 2 | `Noun(Card)` |
| Ghoulcaller's Bell | `36e1485d-3b8c-4c23-9d12-6937fd216b25#card` | 1 | `Noun(Card)` |
| Gibbering Barricade | `c6106021-08ae-49c8-b32c-fdc7397522e7#card` | 2 | `Noun(creature)` |
| Goblin Battle Jester | `eba01ee1-55c4-48db-843b-fd6d48c32eda#card` | 4 | `PremodifiedNominal(Red, Spell)` |
| Goblin Boarders | `4c64feae-95d7-426d-9e42-d3b0841d2054#card` | 18 | `SlashModifiedNominal(1, 1, Counter)` |
| Goblin Bombardment | `edad60c6-80de-4033-af1b-a703ac332983#card` | 3 | `Noun(creature)` |
| Goblin Chirurgeon | `ea55db87-a5c3-49f7-b968-77510cdeb469#card` | 1 | `Noun(goblin)` |
| Goblin Firebug | `ee337677-d898-40a5-9453-405d07ec12b1#card` | 1 | `Noun(land)` |
| Goblin Picker | `efc0aa25-9ac2-4a25-956c-fff39ef536be#card` | 1 | `Noun(Card)` |
| Goblin Trashmaster | `0283bf5e-ddf2-4a4a-a7cf-d3e27eed7e7d#card` | 4 | `Noun(goblin)` |
| Goblin Turncoat | `297c2ddf-6d2d-4ed4-ab37-716514d5e3b6#card` | 1 | `Noun(goblin)` |
| Goblin War Cry | `a6c8c660-2b06-4395-8c86-dd540760d9ae#card` | 2 | `ObjectRelativeNominal(creature, They, core-verb:Control)` |
| Golden Ratio | `724700ad-1e4e-4cbc-af7f-0b3b87ed4bc8#card` | 76 | `Noun(Card)` |
| Golgari Rotwurm | `75df9e6a-a17c-4311-a4e7-ff0d75728337#card` | 1 | `Noun(creature)` |
| Graf Mole | `5b2c6b44-99b3-496d-93ca-d5211222038f#card` | 4 | `Noun(clue)` |
| Grazing Gladehart | `f19f28e5-9cad-4398-b2d4-9e7fefb23cb4#card` | 4 | `ObjectRelativeNominal(land, You, core-verb:Control)` |
| Greed | `1ff62220-be95-4901-b8d8-812b9a1a1b0a#card` | 1 | `Noun(Card)` |
| Grinding Station | `0fcd476f-4db8-4293-9388-1678a0043c9e#card` | 2 | `Noun(artifact)` |
| Ground Seal | `13f6f960-ef79-4f2c-8874-90fe9e77099e#card` | 14 | `Noun(Card)` |
| Gruul Ragebeast | `e5fa93a3-0b2b-4477-ad3f-4fc1b325ca19#card` | 2 | `Noun(Opponent)` |
| Gryff Vanguard | `3bcdf832-4ce2-4043-b46a-0c5fd7019fcd#card` | 1 | `Noun(Card)` |
| Guildsworn Prowler | `72e2aa4f-b4b4-42ab-adc0-d0ac09cd5f92#card` | 1 | `Noun(Card)` |
| Gutless Ghoul | `5ebc166d-98c5-4ab7-9610-bc5bcd231d80#card` | 2 | `Noun(creature)` |
| Hardened Tactician | `ca1dc6f0-3ec2-4f62-a08a-1395201bd93f#card` | 1 | `Noun(Card)` |
| Havoc | `d95aeeb0-b722-4f70-85fc-6866be4854bd#card` | 1 | `Noun(Opponent)` |
| Havoc Jester | `58f5bbd2-b876-4eb9-aa79-f4d3511961e0#card` | 8 | `Noun(Permanent)` |
| Healing Hands | `3cc48835-3ac0-4774-b380-f9b21d2dc974#card` | 1 | `Noun(Card)` |
| Heap Doll | `aa7f0f15-bcbb-4af5-ad31-ff215dd81e71#card` | 4 | `Noun(Graveyard)` |
| Heavy Infantry | `d07707ef-fa4e-423f-8a1f-095d19100321#card` | 1 | `Noun(Opponent)` |
| Hecteyes | `7a11b4ed-430d-4ef8-b0c1-6a022f12448c#card` | 1 | `Noun(Card)` |
| Helpful Hunter | `c0864adb-e9aa-40b6-91ff-a0646193e887#card` | 1 | `Noun(Card)` |
| Hero in Training | `87f1a840-021c-4699-a2c4-f2926daf98eb#card` | 4 | `Noun(Card)` |
| Hesitation | `11eb31d9-cbcb-4ec5-abab-dfa646addbba#card` | 1 | `Noun(Spell)` |
| Highcliff Felidar | `43296f8b-58d9-446e-a538-1c4921552c41#card` | 61 | `Noun(creature)` |
| Hindering Light | `945ac15c-780b-44a9-8a8e-011cf1c30876#card` | 12 | `Noun(Card)` |
| Holy Justiciar | `9edcf49b-2767-48bd-8fbe-968527cbddf4#card` | 1 | `Noun(zombie)` |
| Horizon Chimera | `a1ed77f9-51ac-4cca-a9ce-d3874f3948ad#card` | 4 | `Noun(Card)` |
| Horn of Greed | `b8181d53-1954-4f46-8670-8696440208e8#card` | 1 | `Noun(land)` |
| Howling Golem | `75269a31-3819-4410-8bb7-10de16810a5e#card` | 1 | `Noun(Card)` |
| Hydromorph Guardian | `c426c2b0-82fa-4ea0-bbdb-3a40e60c495e#card` | 4 | `Noun(creature)` |
| Hydromorph Gull | `124fea1b-c0e9-4e56-b026-82dddee0d36e#card` | 4 | `Noun(creature)` |
| Ice | `ae92942b-919c-4ea9-b693-85fcef765d5a#face:1` | 1 | `Noun(Card)` |
| Illusory Demon | `91d0a557-7a80-46d5-b823-392a84a6be44#card` | 4 | `Noun(Spell)` |
| Illvoi Galeblade | `d6273621-c36c-491b-85ca-37445ed30097#card` | 1 | `Noun(Card)` |
| Immersturm Raider | `e3179dd6-5558-4385-8883-f78f9d1b1c4f#card` | 8 | `Noun(Card)` |
| Impact Tremors | `9242cd3e-1a71-4700-8182-9c1005616033#card` | 8 | `ObjectRelativeNominal(creature, You, core-verb:Control)` |
| Imperial Edict | `ac2b3adb-45bb-4f56-9d52-e08771164b35#card` | 1 | `ObjectRelativeNominal(creature, They, core-verb:Control)` |
| Implode | `bfde73b2-46bd-4fdb-9468-35819af609f7#card` | 1 | `Noun(Card)` |
| Insight | `c77afbeb-cc9b-4770-9a48-e7460562895f#card` | 2 | `Noun(Card)` |
| Inspiring Overseer | `d646e42b-5635-4798-b633-29c093b66a55#card` | 2 | `Noun(Card)` |
| Instant Ramen | `2283e409-c6c7-4de9-899b-2b3caea5f35e#card` | 2 | `Noun(Card)` |
| Intervene | `380b386f-90c8-45c5-a611-1ed6803b52d2#card` | 1 | `Noun(creature)` |
| Intruding Soulrager | `7673d0db-07d6-4b40-a32e-f2c98d7ea7c1#card` | 3 | `Noun(Card)` |
| Isperia, Supreme Judge | `c46718dc-24dc-4b77-b455-aa4c89570b8d#card` | 8 | `Noun(creature)` |
| Ithilien Kingfisher | `a2dd89a1-9b3e-4bd1-9b48-36516d63726a#card` | 1 | `Noun(Card)` |
| Jaddi Offshoot | `20faefbd-059c-4aae-81e3-31683bf9f7bf#card` | 4 | `ObjectRelativeNominal(land, You, core-verb:Control)` |
| Jaded Response | `23917517-7561-45b3-b156-77cd2f340466#card` | 10 | `Noun(creature)` |
| Jayemdae Tome | `39ee576a-0803-4063-9c84-f2b537e4d44c#card` | 1 | `Noun(Card)` |
| Joraga Visionary | `7854f475-5470-4f2b-99df-af9026895595#card` | 1 | `Noun(Card)` |
| Jungle Barrier | `dcf65293-14a0-46a5-a0b0-ca2b7fda8bef#card` | 1 | `Noun(Card)` |
| K'un-Lun Warrior | `0618a217-2a6e-4fa1-bfe6-89c6612a594d#card` | 16 | `Noun(artifact)` |
| Kapsho Kitefins | `b9b205db-068e-44c7-9169-5d8f13d4b73a#card` | 2 | `Noun(Opponent)` |
| Kavu Climber | `91320afd-1d42-4cb5-ae40-4eed2fa91dfe#card` | 1 | `Noun(Card)` |
| Kazandu Nectarpot | `263b526c-8b81-44ee-a7c4-5c24bf9b42a4#card` | 4 | `ObjectRelativeNominal(land, You, core-verb:Control)` |
| Keep Safe | `5602c14d-cc22-47d7-a1b0-6491749f00dd#card` | 4 | `Noun(Card)` |
| Keldon Raider | `004f82f0-74cc-4bc7-ab22-ab1a5247bc29#card` | 8 | `Noun(Card)` |
| Kindly Customer | `3faab84f-7055-4554-b948-181928d8d4e2#card` | 1 | `Noun(Card)` |
| Kingfisher | `2a70f3d7-9b5c-455e-8c4b-55870d77bd7d#card` | 1 | `Noun(Card)` |
| Kjeldoran Dead | `61e00946-7df7-4298-91b2-c656169e6601#card` | 1 | `Noun(creature)` |
| Kor Entanglers | `6fedd98c-9c58-4ca5-b77b-dcb4cd88d238#card` | 2 | `Noun(Opponent)` |
| Kraken's Eye | `14a21d8a-2097-4150-8af5-8c155049477a#card` | 2 | `Noun(Player)` |
| Krark-Clan Ogre | `5de63529-7dea-4018-bf46-afb4bc8b69fd#card` | 1 | `Noun(artifact)` |
| Kraul Whipcracker | `b563ae31-338e-4bcc-a05f-d7ed7c3fe46e#card` | 1 | `Noun(Opponent)` |
| Kris Mage | `d53623b0-f7d8-401f-a8f1-80034f6f20df#card` | 3 | `Noun(Card)` |
| Labyrinth Champion | `e1a22f41-0543-4e85-8fb8-93c018b461aa#card` | 16 | `SubjectRelativeNominal(Spell, That, Target, This, creature)` |
| Lagonna-Band Elder | `60e3ccf2-63f5-4435-a752-1d82fefb55d7#card` | 4 | `Noun(enchantment)` |
| Laid to Rest | `ae6dc45c-b8c9-4cbf-892f-8c00f4199aaf#card` | 72 | `Noun(Card)` |
| Lamplighter of Selhoff | `a88c5540-a242-49bc-a725-39b9fda085c3#card` | 16 | `Noun(Card)` |
| Leave No Trace | `a9367efe-82f2-43e4-9692-205567be6a0c#card` | 17 | `Noun(Color)` |
| Leonin Elder | `f85a1752-c6fa-481b-997d-6b5dd4c6b54b#card` | 2 | `Noun(artifact)` |
| Lesser Gargadon | `7e162cc1-9482-4812-8897-209ec2927ec0#card` | 1 | `Noun(land)` |
| Library Larcenist | `eb0e6f63-2861-4392-ac0e-64380fbbd8ae#card` | 1 | `Noun(Card)` |
| Lifegift | `c99d4be7-244c-4fb5-877a-8aee961a2666#card` | 2 | `Noun(land)` |
| Liliana's Caress | `a4aec0d6-13fa-4709-b1a9-2f483f032744#card` | 1 | `Noun(Card)` |
| Liliana's Specter | `1a7e0e55-41c3-4fd8-b718-55b59b5cab90#card` | 1 | `Noun(Card)` |
| Limestone Golem | `0dddf5a9-fdf3-49e5-8103-a8cfd62f55c6#card` | 1 | `Noun(Card)` |
| Living Lies of Loki | `cbbf61b0-5dd2-4643-8857-b7a10ec76ec8#card` | 8 | `Noun(Card)` |
| Lobber Crew | `84dffb4b-02d7-48e4-9a95-43f88cd52bf3#card` | 12 | `PremodifiedNominal(Multicolored, Spell)` |
| Loch Dragon | `e40d47cf-6232-41ec-8865-d12a335dfcee#card` | 8 | `Noun(Card)` |
| Lovestruck Beast | `7e84aff5-2cb6-4214-befd-3d2de31229e5#face:0` | 2 | `SlashModifiedNominal(1, 1, creature)` |
| Loyal Sentry | `54b1a496-1f8e-4581-8260-bad57b25c6e3#card` | 1 | `Noun(creature)` |
| Lumengrid Sentinel | `ede92fb6-dfd6-40b4-b58f-af1f7a155efc#card` | 4 | `ObjectRelativeNominal(artifact, You, core-verb:Control)` |
| Lunar Force | `31b18e10-c74d-4384-a0e2-def0b3ec4799#card` | 1 | `Noun(Spell)` |
| Mad Prophet | `91081181-aec6-47de-b8ef-241a2f4fe880#card` | 1 | `Noun(Card)` |
| Makeshift Binding | `942f3f06-1df0-40b4-8537-71cbec111852#card` | 4 | `Noun(Opponent)` |
| Makindi Ox | `e4f0fa12-542b-485f-a792-3a8aba933522#card` | 2 | `Noun(Opponent)` |
| Malcator's Watcher | `b0493924-dc6d-4655-865f-7e9b40f547ea#card` | 1 | `Noun(Card)` |
| Manabarbs | `0f1afedd-c60f-454f-b84a-c8117aec0128#card` | 12 | `Noun(land)` |
| Martyred Rusalka | `21bf5ae1-d7a0-406d-88fd-009dfa919096#card` | 1 | `Noun(creature)` |
| Masked Meower | `a0037ff8-64db-45dd-bba1-db5c49095083#card` | 1 | `Noun(Card)` |
| Mass Appeal | `aef067b4-cd9c-4f18-8c63-fbbc39b63376#card` | 10 | `Noun(Card)` |
| Mayhem Devil | `4709f11c-aef8-45ac-b2bf-e640c568dfac#card` | 4 | `Noun(Player)` |
| Medicine Bag | `5801f773-e214-413d-968d-b84bf99f0642#card` | 1 | `Noun(Card)` |
| Megrim | `633ad9e2-9f55-4a1c-9248-661ad4b0e1dc#card` | 4 | `Noun(Card)` |
| Meltstrider Eulogist | `046b60da-0a14-40b0-a36c-5328f6b8972b#card` | 18 | `Noun(Card)` |
| Memory Erosion | `f4a96881-586d-44ad-b427-5cdf8988f9a1#card` | 1 | `Noun(Spell)` |
| Mental Discipline | `b22080d6-a9ed-4bdd-a604-058e0e3e9463#card` | 1 | `Noun(Card)` |
| Mental Note | `e8d5f31c-7abf-4fbb-977e-8353a97daf7a#card` | 1 | `Noun(Card)` |
| Merchant of Secrets | `f6aebd42-0150-4741-84c2-4c85893640e9#card` | 1 | `Noun(Card)` |
| Merchant of the Vale | `f88a096e-7497-4297-ab65-f18769b1cc47#face:0` | 1 | `Noun(Card)` |
| Merrow Witsniper | `79f8378a-fb50-4178-be53-b6926bc1cba5#card` | 1 | `Noun(Card)` |
| Messenger Drake | `4c62dce6-dc1f-4d2d-95dd-295810ada6a6#card` | 1 | `Noun(Card)` |
| Messenger Falcons | `01fda82a-227d-4e3d-8481-a13c3812392a#card` | 1 | `Noun(Card)` |
| Miasmic Mummy | `b5b049d4-242a-48ec-85d1-1ef66c8cfa4c#card` | 1 | `Noun(Card)` |
| Mind Sludge | `86e64a29-6ed1-451b-a114-f74341322eeb#card` | 10 | `Noun(Card)` |
| Mindless Null | `22ff4e16-5c66-4076-94cc-32c281d42f03#card` | 2 | `Noun(vampire)` |
| Mnemonic Sphere | `53bfffae-b99e-44bd-8954-6cf236f93f26#card` | 1 | `Noun(Card)` |
| Moldervine Reclamation | `68639a3d-2192-4921-8298-c76bb0cd6b02#card` | 4 | `Noun(Card)` |
| Molten Nursery | `92abb79d-3c60-48f7-87a0-13c25a5e1128#card` | 16 | `PremodifiedNominal(Colorless, Spell)` |
| Moonglove Extractor | `980affa7-6308-4acd-af64-f804e574a89b#card` | 2 | `Noun(Card)` |
| Moonlit Wake | `c93965a8-c527-4b1f-8322-b043f910ffb7#card` | 2 | `Noun(creature)` |
| Mortiphobia | `4fb8312c-058a-415c-93ab-59d37ebad270#card` | 16 | `Noun(Card)` |
| Murder of Crows | `babab2f5-fd1f-4e0a-a692-6c2ce35df708#card` | 8 | `Noun(Card)` |
| Muse Drake | `04c9b3d7-83ba-400e-9837-2ba0119e9445#card` | 1 | `Noun(Card)` |
| Mysterious Tome | `b32253d6-5f00-40a8-a7e5-dc4655cd288c#face:0` | 1 | `Noun(Card)` |
| Nebelgast Herald | `cf336e4c-a0d3-43aa-ad23-b98effb2b751#card` | 2 | `Noun(Opponent)` |
| Necrogen Spellbomb | `34993bd9-53bf-46b2-bc20-b6daf82382d9#card` | 1 | `Noun(Card)` |
| Netter en-Dal | `8cd65dcd-e36d-47b1-b011-961c79004cb0#card` | 1 | `Noun(Card)` |
| Nettle Drone | `cc4e8b8e-f615-4842-84c5-d6154986b666#card` | 12 | `PremodifiedNominal(Colorless, Spell)` |
| Nexus Wardens | `a2652158-e63f-477b-b328-cae7ef2263bd#card` | 4 | `ObjectRelativeNominal(enchantment, You, core-verb:Control)` |
| Nezumi Informant | `a70ffefb-840c-48c3-a59d-e12306086bb6#card` | 1 | `Noun(Card)` |
| Nim Shambler | `83d2ff7f-4d6e-4cc0-b8a9-3c5b3236eea0#card` | 4 | `Noun(creature)` |
| Nimble Innovator | `30929713-47dc-40f0-883c-4bd41aa399d5#card` | 1 | `Noun(Card)` |
| No Mercy | `b9538d53-480b-481a-abbf-83ab17e1a45b#card` | 6 | `Noun(creature)` |
| Noble Stand | `1aa1e34c-b2ff-4750-9e73-5707a8f49bfb#card` | 4 | `ObjectRelativeNominal(creature, You, core-verb:Control)` |
| Noxious Toad | `2fdc484e-b3c3-4f4e-99a1-26a1134aa1cd#card` | 1 | `Noun(Card)` |
| Oculus | `e793ede9-044a-44e9-bc9c-99b0034503eb#card` | 2 | `Noun(Card)` |
| Ogre Recluse | `17ff0155-0006-464a-a65f-b8e1b9aeba0b#card` | 1 | `Noun(Spell)` |
| Oppression | `f488f679-f5d5-4e61-b0c8-0b0eb622df0d#card` | 1 | `Noun(Card)` |
| Orcish Bloodpainter | `259bba2f-8c68-4267-8f51-0b2144778653#card` | 3 | `Noun(creature)` |
| Orcish Mechanics | `0ec58835-de2d-4064-89a9-f92db80bc276#card` | 3 | `Noun(artifact)` |
| Orcish Vandal | `933a1811-7e1a-4173-b1c9-34b2c858ba68#card` | 3 | `Noun(artifact)` |
| Oread of Mountain's Blaze | `ebbcbaa4-9dc2-4098-b6a2-393375d86f16#card` | 1 | `Noun(Card)` |
| Ormendahl, the Corrupter | `20d0df30-013c-47c0-b70b-d628d427d30b#face:1` | 1 | `Noun(Card)` |
| Outlaw Medic | `637be5f7-ab39-4e3a-a5eb-c6e48fca00a9#card` | 1 | `Noun(Card)` |
| Outwit | `eac02e36-15c8-4b6d-b9b4-da549a722336#card` | 1 | `Noun(Player)` |
| Overgrown Estate | `4d52c4a5-e5c8-4fb4-be50-78d5482dd1ae#card` | 2 | `Noun(land)` |
| Pain | `433fcba3-06b8-4f1a-a709-300838251439#face:0` | 1 | `Noun(Card)` |
| Painful Quandary | `c37051cc-6683-4dbb-b5ff-5c3a5bdab1df#card` | 2 | `Noun(Card)` |
| Palace Familiar | `dab9e40f-b73a-4c55-8bb3-63b32c23f07b#card` | 1 | `Noun(Card)` |
| Parcel Myr | `9e660f66-0a94-4694-95e0-25296799b193#card` | 1 | `Noun(Card)` |
| Peace of Mind | `4f8c5fd7-f280-4b0c-bb84-6ff9b258c50f#card` | 2 | `Noun(Card)` |
| Pendulum of Patterns | `992075d5-f413-4896-b75e-7bb2d589c50e#card` | 2 | `Noun(Card)` |
| Perplexing Chimera | `7d075b8a-a606-4590-b52b-b4ef3a9e342f#card` | 1792 | `Noun(Spell)` |
| Phyrexian Vault | `b628150b-08a1-4ea3-978d-60255dfb0b7e#card` | 1 | `Noun(creature)` |
| Plant Elemental | `e822bf3d-3a29-4a02-9ae8-e2830ce70f15#card` | 4 | `Noun(forest)` |
| Plundering Predator | `dfd260f2-90e6-464b-91e2-a66bb3877918#card` | 8 | `Noun(Card)` |
| Pond Prophet | `6eabebdf-c1a3-4789-a67f-859f0957c5df#card` | 1 | `Noun(Card)` |
| Potion of Healing | `11fb3fee-ae84-4bf3-b834-63d3b8987df3#card` | 2 | `Noun(Card)` |
| Powerstone Minefield | `3a8a67ad-e4ff-4b55-9976-a2f13cc9b41b#card` | 4 | `Noun(creature)` |
| Pressure Point | `d4bbdd0c-e578-49ed-b2f7-b1decb04ee87#card` | 1 | `Noun(Card)` |
| Priest of Ancient Lore | `a89b0fd5-84b2-487c-8b03-e78d97276fd3#card` | 2 | `Noun(Card)` |
| Promising Duskmage | `44958014-7e30-4e70-9aef-b148843f9c45#card` | 4 | `Noun(Card)` |
| Psychic Corrosion | `328b42f1-d679-4f9c-80e3-38fe3b965d10#card` | 2 | `Noun(Card)` |
| Psychic Membrane | `5a2deb96-01f4-48d5-bec8-44e4f141f551#card` | 2 | `Noun(Card)` |
| Purple-Crystal Crab | `df4569d7-ee75-4ba0-bbc3-7766ac686523#card` | 1 | `Noun(Card)` |
| Pyrite Spellbomb | `2c10cae2-951a-4f4f-94e4-8713b58d07dd#card` | 3 | `Noun(Card)` |
| Pyroconvergence | `8da5c156-3688-4d6e-b5c4-95c082d1b426#card` | 16 | `PremodifiedNominal(Multicolored, Spell)` |
| Quagmire Druid | `560e1e81-6675-4d08-8e82-cdf1abd4b3d0#card` | 1 | `Noun(creature)` |
| Quicksmith Genius | `bca0c136-3845-4cb9-8d80-f504a01ad50a#card` | 16 | `Noun(Card)` |
| Ravenous Baloth | `ee771e66-72f8-480f-9920-92c68ab93c3b#card` | 2 | `Noun(beast)` |
| Ravenous Chupacabra | `7b459306-149b-4f43-abc1-2dd70c748c0e#card` | 1 | `Noun(Opponent)` |
| Ravenous Rats | `2fa1bbfd-92b5-482c-b32d-4cdc286474c4#card` | 1 | `Noun(Card)` |
| Raving Oni-Slave | `024ea14f-48b7-485d-8426-317d59f8262d#card` | 8 | `Noun(demon)` |
| Reach Through Mists | `c81ca8ff-92f3-481e-82f7-0673b6c74ea0#card` | 1 | `Noun(Card)` |
| Rebuff the Wicked | `f5822e53-ddec-4c77-bcf4-091e35c1e731#card` | 4 | `Noun(Permanent)` |
| Reckless Fireweaver | `180e1a7e-890d-477c-80a5-da8a5f2857b3#card` | 8 | `ObjectRelativeNominal(artifact, You, core-verb:Control)` |
| Reef Pirates | `13fb5413-0057-4022-84da-2c90ce065ed1#card` | 3 | `Noun(Card)` |
| Refocus | `0101f891-e54c-43e5-86f3-263d95060fcd#card` | 1 | `Noun(Card)` |
| Refresh | `1a4c6643-c48b-4e83-b0c4-96873c13408c#card` | 1 | `Noun(Card)` |
| Regal Force | `e264ffe3-0252-49d6-b990-dbb3654325a5#card` | 22 | `Noun(Card)` |
| Reki, the History of Kamigawa | `8a7d68ae-ac43-46a6-9dc8-d6b07cc0333c#card` | 4 | `Noun(Card)` |
| Renegade Tactics | `7fbcd256-c132-406f-a490-df9709835504#card` | 1 | `Noun(Card)` |
| Reparations | `3cb86deb-f4e5-41c5-bfcd-614a24f3499c#card` | 24 | `Noun(creature)` |
| Resistance Squad | `a16eeef9-e03c-4ebb-b986-325252a9491f#card` | 2 | `Noun(Card)` |
| Resupply | `129ec223-e76d-491d-b299-07ac91168d31#card` | 2 | `Noun(Card)` |
| Return to Nature | `777b8ec4-a783-4297-96b7-4f200d0eb734#card` | 4 | `Noun(Graveyard)` |
| Revitalize | `b1385b03-cb4b-4812-857f-7421f1df39af#card` | 2 | `Noun(Card)` |
| Reviving Dose | `ad470afc-d1a5-4e64-8192-fb6308ed9bfe#card` | 2 | `Noun(Card)` |
| Rewards of Diversity | `064c5a14-c2f2-4b7a-8e0b-01b9995e8f89#card` | 2 | `Noun(Opponent)` |
| Rhox Meditant | `832c6834-2cc7-41d5-b881-37c24c13ab8a#card` | 2 | `Noun(Card)` |
| Rhox Oracle | `d26d1cce-3bcf-48d4-abce-8b12ca7b7432#card` | 1 | `Noun(Card)` |
| Ribbons of the Reikai | `22b76ab7-9658-41a3-b8bb-fc02de0631a4#card` | 10 | `Noun(Card)` |
| Rigging Runner | `72b600e3-d671-4279-8ca6-9a3d6e774b53#card` | 18 | `SlashModifiedNominal(1, 1, Counter)` |
| Righteous Cause | `218f835b-4195-4ee8-a4ef-e5ba58fa5374#card` | 2 | `Noun(creature)` |
| Rimefur Reindeer | `66dc3237-3543-4fc2-96f1-c8d1eff7c04c#card` | 2 | `Noun(Opponent)` |
| Ripchain Razorkin | `ce000c0b-db42-4569-855f-f4eae0431c09#card` | 1 | `Noun(land)` |
| Riptide Crab | `8ae8a583-9843-474a-bd46-8090df883621#card` | 1 | `Noun(Card)` |
| Riptide Director | `f7ae67c7-9005-4806-8130-5895dcd9879a#card` | 10 | `Noun(Card)` |
| Ritual of Rejuvenation | `7c9d8065-9ab0-4468-a11c-383bd036029f#card` | 2 | `Noun(Card)` |
| River Hoopoe | `058bbc0a-9c09-4c98-8735-83255a71bbca#card` | 2 | `Noun(Card)` |
| Rogue Elephant | `bed7ac55-fe40-46d0-bc22-1c8d11f41459#card` | 4 | `Noun(forest)` |
| Rook Turret | `c5a6d752-9975-4059-8552-55c6ffb3ab68#card` | 16 | `Noun(Card)` |
| Rootwater Alligator | `2bf84938-6049-4d03-b09e-85bd0763a187#card` | 1 | `Noun(forest)` |
| Rottenheart Ghoul | `f9535aa3-0c07-4fad-8dc9-9838a9ebd304#card` | 1 | `Noun(Card)` |
| Roving Harper | `6474f6b3-4e8e-4056-a01c-2f5e5e59c8de#card` | 1 | `Noun(Card)` |
| Ruinous Minotaur | `ece9d3e5-79e5-4c56-9e8d-8abcd819c041#card` | 3 | `Noun(land)` |
| Rummaging Goblin | `2055eb91-ee36-4752-a6dc-581eaef335c8#card` | 1 | `Noun(Card)` |
| Runed Servitor | `4e5e81e1-ab47-4f08-9c42-dba0adc5cf9a#card` | 1 | `Noun(Card)` |
| Runewing | `cc300649-ae50-47e7-ad0f-ffbdffddb989#card` | 1 | `Noun(Card)` |
| Rushwood Herbalist | `2e35bcef-505a-4559-a1fe-f42f4543197f#card` | 1 | `Noun(Card)` |
| Rusted Slasher | `098cc111-493b-470b-929c-67deb981240c#card` | 1 | `Noun(artifact)` |
| Sabotender | `65385311-1158-4e94-892a-683997706ca8#card` | 8 | `ObjectRelativeNominal(land, You, core-verb:Control)` |
| Sage of Lat-Nam | `f34be3cc-ff47-4415-a6a8-ed142891dc0c#card` | 1 | `Noun(artifact)` |
| Sage of Mysteries | `62c949c8-0011-4ab7-8388-e643be79df83#card` | 2 | `ObjectRelativeNominal(enchantment, You, core-verb:Control)` |
| Saltwater Stalwart | `04472c4f-0df3-4de8-9cc2-882d95b822be#card` | 3 | `Noun(Card)` |
| Sanctimony | `2fc7740a-dfa4-4279-8822-3f7c93e6b685#card` | 6 | `Noun(Opponent)` |
| Sangromancer | `920445ab-0ac2-4de7-bc1c-f5e58eb4424c#card` | 4 | `Noun(Card)` |
| Sapphire Drake | `1987bb61-a31b-4d5a-910f-88e9cbc6c8b3#card` | 18 | `SlashModifiedNominal(1, 1, Counter)` |
| Scald | `1de0b522-f12a-4597-9d5f-9a246443487e#card` | 12 | `Noun(Player)` |
| Scepter of Insight | `e2725411-a600-4ff3-94fb-5d52f5eda414#card` | 1 | `Noun(Card)` |
| Scholar of Stars | `0753aee4-33db-48c5-9854-16a9d91535b2#card` | 2 | `Noun(artifact)` |
| Screeching Buzzard | `89acea5a-5601-4341-83a0-bde281c4bd85#card` | 1 | `Noun(Card)` |
| Scroll of Avacyn | `f728c4f1-178f-421b-a713-21f85892f051#card` | 4 | `Noun(Card)` |
| Scroll of Griselbrand | `c961373a-be0e-467c-92e0-33a0c953736a#card` | 2 | `Noun(Card)` |
| Scythe Tiger | `4d0fa514-87fc-46ff-bf49-4cd332396978#card` | 28 | `Noun(land)` |
| Sea Gate Loremaster | `6eed122b-9760-47fd-8ba2-adeda8054e0d#card` | 10 | `Noun(Card)` |
| Seer of the Last Tomorrow | `dc5cb189-9eff-4b4e-a46d-7a8e62529c89#card` | 1 | `Noun(Card)` |
| Seismic Mage | `a1f7e43a-f126-47d8-bd59-339ae052e81b#card` | 1 | `Noun(Card)` |
| Selhoff Occultist | `d72031e1-c8cf-4e57-b15b-6a5eaacff1e2#card` | 1 | `Noun(Card)` |
| Serum Raker | `7bb7b937-a7ad-4f5c-b173-df71556c48e9#card` | 1 | `Noun(Card)` |
| Servant of Volrath | `4e389b61-7b31-42ba-a19b-2502f63fd34a#card` | 1 | `Noun(creature)` |
| Setessan Battle Priest | `e2886098-0309-4da5-b2e5-a06af5cfe5b8#card` | 8 | `SubjectRelativeNominal(Spell, That, Target, This, creature)` |
| Settlement Blacksmith | `f98d0f8a-c8b5-4fb8-b598-e70ffab3c0ff#card` | 2 | `Noun(Card)` |
| Shadowfeed | `7681ea90-be9d-4a43-845e-ba051b438429#card` | 8 | `Noun(Graveyard)` |
| Shaman of Spring | `b1ee0250-83ea-45a9-b5e5-3fa74bc7f26b#card` | 1 | `Noun(Card)` |
| Shattered Angel | `e339c6cb-c75d-4dd4-8c1a-78f844983242#card` | 2 | `Noun(Opponent)` |
| Shattergang Brothers | `7fb63d9a-8d90-4b43-8390-924de2d7e32c#card` | 1 | `Noun(artifact)` |
| Sheoldred, the Apocalypse | `34f34409-326d-4994-a0ea-1a69aa278f03#card` | 4 | `Noun(Card)` |
| Shipwreck Looter | `ead1accc-b90e-424a-a55f-2f4771726ce1#card` | 16 | `Noun(Card)` |
| Shire Shirriff | `14f36157-f55f-495d-9ffe-f38135a59750#card` | 16 | `Noun(Token)` |
| Shoal Kraken | `be700da5-123e-461e-8192-88f9d86925b9#card` | 16 | `Noun(Card)` |
| Shrapnel Slinger | `b400536c-ea8b-4237-90da-1d3ac139881d#card` | 8 | `Noun(creature)` |
| Shroudstomper | `36415fe3-6bf3-4fec-909c-6c16584c44eb#card` | 2 | `Noun(Card)` |
| Sibsig Icebreakers | `4880c976-b417-48e4-98a2-9e5a8da0d758#card` | 1 | `Noun(Card)` |
| Silent Gravestone | `803815c5-12be-48b3-a101-f416c1ef9b7b#card` | 70 | `Noun(Card)` |
| Silverback Shaman | `fc011947-b496-400e-99b8-b368068ba79b#card` | 1 | `Noun(Card)` |
| Sinister Concoction | `ffc3beec-6485-428d-a251-fa0bc21263a5#card` | 1 | `Noun(Card)` |
| Skeletal Kathari | `5bcb63b4-6236-45c5-884d-188f8f5e8f02#card` | 1 | `Noun(creature)` |
| Skirsdag Cultist | `a7fe5473-fae5-490e-8600-b7e497b9989e#card` | 3 | `Noun(creature)` |
| Skirsdag Flayer | `38ea058d-f8cd-4bd9-9142-8f0d3d56d571#card` | 1 | `Noun(human)` |
| Skirsdag Supplicant | `c7d97311-6e34-4679-a678-e107022c785c#card` | 1 | `Noun(Card)` |
| Skull Catapult | `eef931d8-4048-4c33-bd8c-0f67d1083ee6#card` | 3 | `Noun(creature)` |
| Skullmead Cauldron | `b6864a26-9c13-4c6d-b0e0-d5b5ed9864e8#card` | 4 | `Noun(Card)` |
| Skyscanner | `974f788a-039f-4310-a2fe-16b14a1e2d35#card` | 1 | `Noun(Card)` |
| Skyship Buccaneer | `5951f734-05d3-41ad-94ee-19bd310077d2#card` | 2 | `Noun(Card)` |
| Skyswimmer Koi | `53b9a2cf-2e6e-4ce0-bb9f-f33fb27b7b5e#card` | 16 | `Noun(Card)` |
| Slay | `64418e1f-4fb1-4029-a31e-135b2d2c0ab9#card` | 1 | `Noun(Card)` |
| Slinking Skirge | `82875793-b264-4ceb-8525-ef3b6d086072#card` | 1 | `Noun(Card)` |
| Smash | `1602f8c7-fe00-4bc9-b4fc-b26b7223ac75#card` | 1 | `Noun(Card)` |
| Snare Tactician | `efee89a6-b730-45b6-bb84-33de0f25ad47#card` | 2 | `Noun(Card)` |
| Soaring Show-Off | `5ca0f4f3-6133-4f53-9124-f63942796a07#card` | 1 | `Noun(Card)` |
| Soldevi Sentry | `d2a61132-d881-4c80-92f1-a151d835832a#card` | 1 | `Noun(Card)` |
| Soulreaper of Mogis | `508ce3a9-9050-4494-9adf-7f9cd6cc2f9f#card` | 1 | `Noun(creature)` |
| Spellscorn Coven | `9357fc7f-c9d0-4ce2-a6dd-0ff1f24bfc56#face:0` | 1 | `Noun(Card)` |
| Spellshock | `6579bb00-3886-447b-a265-3e72e98bbe2c#card` | 4 | `Noun(Spell)` |
| Spined Fluke | `3c880ac2-0b22-4f7e-a55a-381de235e971#card` | 1 | `Noun(creature)` |
| Spirited Companion | `9c5f0d91-9d86-4e66-94fd-4af93ad01838#card` | 1 | `Noun(Card)` |
| Spiritual Asylum | `91924536-7a2b-44ec-9835-2efa402c83f9#card` | 28 | `ObjectRelativeNominal(creature, You, core-verb:Control)` |
| Spitfire Lagac | `7215ca87-bbff-4c14-a1f5-de5ddd97c875#card` | 8 | `ObjectRelativeNominal(land, You, core-verb:Control)` |
| Spore Crawler | `fa7c13c0-350b-4ed6-8c16-590d2b988184#card` | 1 | `Noun(Card)` |
| Spreading Plague | `6c134ef7-5c2f-4e4c-ae40-ef338fbf41e6#card` | 17 | `Noun(creature)` |
| Stadium Tidalmage | `1c45e872-3af8-47a4-9dc7-a99c293338f8#card` | 8 | `Noun(Card)` |
| Staff of Domination | `d7888719-647d-4022-a211-822fa09f0791#card` | 2 | `Noun(Card)` |
| Starved Rusalka | `0a57ac2e-276b-41ea-89f4-287f530114c6#card` | 2 | `Noun(creature)` |
| Steamclaw | `8cbc0fa4-4f62-47e1-90dc-c21d2682a493#card` | 16 | `Noun(Graveyard)` |
| Stern Constable | `eb3f493a-a5d6-4b9b-9ea0-3c295a67f730#card` | 1 | `Noun(Card)` |
| Stiltzkin, Moogle Merchant | `958c2a27-9fb1-42b4-a0ef-296584763a98#card` | 72 | `Noun(Card)` |
| Stone Haven Outfitter | `8e646a6b-77ba-4f7d-8ee9-b0b084453f77#card` | 16 | `Noun(Card)` |
| Stone-Seeder Hierophant | `ccaea194-4b21-4485-ba08-d8d030c9418d#card` | 2 | `ObjectRelativeNominal(land, You, core-verb:Control)` |
| Stony-Voiced Goblins | `d1dceaa7-d33e-4067-9e1b-8a8978a12933#card` | 1 | `Noun(Card)` |
| Storm Fleet Aerialist | `b128d0cf-77d9-4751-99c6-0f3bc3660318#card` | 18 | `SlashModifiedNominal(1, 1, Counter)` |
| Storm Fleet Spy | `42081a57-b521-45c2-928d-0a0a4641818d#card` | 2 | `Noun(Card)` |
| Stormchaser Drake | `a9530416-d5fb-4ddd-ae6f-0eefb9f276c5#card` | 8 | `Noun(Card)` |
| Striped Bears | `ac856e91-1b88-464b-9420-dc311ce814ea#card` | 1 | `Noun(Card)` |
| Stun | `d3491972-44c5-4962-a680-42b79357a189#card` | 1 | `Noun(Card)` |
| Summit Sentinel | `e7b88da3-7dd1-45f8-b555-98ad838b2917#card` | 1 | `Noun(Card)` |
| Sunbeam Spellbomb | `014c94d8-2c39-4d33-902b-fa2398406fd5#card` | 2 | `Noun(Card)` |
| Surveilling Sprite | `449142ff-7190-471b-99a5-f77095a827dd#card` | 2 | `Noun(Card)` |
| Swaggering Corsair | `48fa47c0-544e-4374-a0a6-ef8f65cdf0dd#card` | 18 | `SlashModifiedNominal(1, 1, Counter)` |
| Takenuma Bleeder | `ee397a87-6933-4d2d-aac6-8dc4e96d524b#card` | 8 | `Noun(demon)` |
| Tanglespan Lookout | `d6a767c2-8bc8-4686-899a-0ba3a525977a#card` | 2 | `Noun(Card)` |
| Tar Pitcher | `e6991ab8-36f7-40c4-ad37-06e847d92664#card` | 3 | `Noun(goblin)` |
| Tatyova, Benthic Druid | `0715e860-3b3b-4331-9718-207973e94fee#card` | 4 | `Noun(Card)` |
| Tel-Jilad Lifebreather | `1ef49064-eb2a-49ae-917e-3d441ba14bf9#card` | 1 | `Noun(forest)` |
| Temple Bell | `fc8032e9-c83a-4cf9-92e1-b1d2d9642695#card` | 1 | `Noun(Card)` |
| Territorial Hammerskull | `a690c1de-158a-4f33-a916-ed2a130f92eb#card` | 1 | `Noun(Opponent)` |
| Tezzeret's Ambition | `91486e70-94e4-431f-8899-fef019bb206e#card` | 2 | `Noun(Card)` |
| Thallid Soothsayer | `eee801ea-e2dc-4b03-a9f9-e87566cb2ae8#card` | 1 | `Noun(creature)` |
| Thieving Otter | `5dee663f-9e2e-4503-8086-fc51664fdf29#card` | 3 | `Noun(Card)` |
| Thing from the Deep | `9542bd7f-bb99-4b59-a4e3-b88ed9f798bf#card` | 4 | `Noun(island)` |
| Thought Scour | `83101ba8-a569-4827-8c53-9ca0dfcd59a7#card` | 1 | `Noun(Card)` |
| Thoughtrender Lamia | `d97c4bca-6433-4cb0-b63a-fdf8360fc29c#card` | 2 | `Noun(Card)` |
| To Arms! | `90c97fc4-597c-45cf-96c6-3ba9558ede0d#card` | 2 | `Noun(Card)` |
| Tome Raider | `ecd9aa9a-06b2-4d8d-9264-fda2036ff142#card` | 1 | `Noun(Card)` |
| Tonic Peddler | `9039f59a-915b-4b16-a5c4-1a5fbfb0b639#card` | 1 | `Noun(Card)` |
| Totem Speaker | `61aa0e56-e8d9-4223-895f-e54cdc5a3f0f#card` | 2 | `Noun(beast)` |
| Tranquil Path | `a9595011-8ddb-403a-ba35-5a79d346557e#card` | 1 | `Noun(Card)` |
| Treasure Trove | `3c4a6b34-a9bb-426d-908c-7ae0011591df#card` | 1 | `Noun(Card)` |
| Triton Fortune Hunter | `f84637f0-1a04-4fd1-85d9-8e8f5d444800#card` | 4 | `Noun(Card)` |
| Tunneler Wurm | `31370fdf-c666-40d2-a7ff-3ba1dfe350cd#card` | 1 | `Noun(Card)` |
| Tunneling Geopede | `78cd6d20-033f-471f-a86e-59ec8307ae1f#card` | 8 | `ObjectRelativeNominal(land, You, core-verb:Control)` |
| Turn Aside | `4325ae73-af43-4a7d-a552-9909f0dc77db#card` | 4 | `Noun(Permanent)` |
| Underworld Dreams | `967cf377-ae26-464d-85ac-8448b5a911f7#card` | 4 | `Noun(Card)` |
| Unhinge | `cb1ae4f2-4fec-4cac-8992-616af82e0ade#card` | 1 | `Noun(Card)` |
| Vampire Slayer | `bae41d41-9dbe-4d6b-bf5d-8a7d7041462c#card` | 3 | `Noun(vampire)` |
| Vampiric Rites | `660de988-b6fb-4f36-8006-42af3e7f908d#card` | 2 | `Noun(creature)` |
| Vault Plunderer | `80e5d768-cc1f-48f8-b0fe-ed974e4d1a56#card` | 1 | `Noun(Card)` |
| Vedalken Heretic | `001c6369-df13-427d-89df-718d5c09f382#card` | 6 | `Noun(Card)` |
| Vedalken Plotter | `3f85b92d-7212-4fc7-a5e8-6350e1acfdd3#card` | 18 | `Noun(Opponent)` |
| Vengeful Tracker | `fe540a23-0357-44fa-8e1c-0a5c2a04cb01#card` | 4 | `Noun(artifact)` |
| Vessel of Paramnesia | `f6894486-0bac-4b72-92b7-5ce99fa426e6#card` | 1 | `Noun(Card)` |
| Viashino Racketeer | `18b09c2d-20cb-40e9-8209-8a1ec2edde97#card` | 8 | `Noun(Card)` |
| Viashino Skeleton | `a85e893c-6027-42eb-8d05-14c745cc8c95#card` | 1 | `Noun(Card)` |
| Vigilant Martyr | `122dfe9f-e0e2-4f69-9637-289c956fd54a#card` | 1 | `Noun(enchantment)` |
| Vigilante Justice | `9610176a-dcd6-4117-89db-45f56b16cdd6#card` | 8 | `ObjectRelativeNominal(human, You, core-verb:Control)` |
| Village Elder | `adea6cb4-1e74-4137-9c80-1dde004a6b26#card` | 1 | `Noun(forest)` |
| Vindictive Mob | `af55096c-af6b-479f-8257-144e596e89d8#card` | 4 | `Noun(creature)` |
| Virus Beetle | `e1e489f6-37e5-469e-bca3-0e9d3fc3fa95#card` | 1 | `Noun(Card)` |
| Vision of Love | `70c96b50-ac2b-493d-bd15-fc66eb767111#card` | 16 | `Noun(artifact)` |
| Vulshok War Boar | `4641e86f-5370-4b25-bd76-07bea856c032#card` | 4 | `Noun(artifact)` |
| Wake of Vultures | `2948b1d5-c12c-45c4-a3e8-aa470fcee1af#card` | 1 | `Noun(creature)` |
| Wall of Blossoms | `ef4d5fb3-70a3-433d-a9d3-18b2beb8d79f#card` | 1 | `Noun(Card)` |
| Wall of Mulch | `afd2141f-1b0f-46b5-b1ac-aa28982d16c0#card` | 1 | `Noun(Card)` |
| Wall of Omens | `5f601f48-d24b-4883-9fde-b3f620e7c9ea#card` | 1 | `Noun(Card)` |
| War Falcon | `406bccb6-6602-4486-a556-e0b550650253#card` | 2 | `Noun(knight)` |
| Warleader's Call | `a751c07b-fc21-4854-96e7-f71abf4e86c9#card` | 16 | `ObjectRelativeNominal(creature, You, core-verb:Control)` |
| Warmth | `cccc9664-0e3d-44c4-959c-abeaba25bbab#card` | 2 | `Noun(Opponent)` |
| Weaponize the Monsters | `fa2154fb-6756-4cf8-85a6-68264fb7a0b2#card` | 3 | `Noun(creature)` |
| Wei Assassins | `e87dd341-416c-4e93-8eb4-1519ecd2674c#card` | 1 | `ObjectRelativeNominal(creature, They, core-verb:Control)` |
| Well-Laid Plans | `50343947-d127-4479-9c1b-8f5040a97afb#card` | 42 | `Noun(creature)` |
| Wirecat | `57828973-df3d-4288-988d-147a3016120e#card` | 6 | `Noun(enchantment)` |
| Wirewood Savage | `45579312-499f-41de-b16e-a231d65a2053#card` | 2 | `Noun(Card)` |
| Wistful Selkie | `4503d0fb-842d-49f1-9769-8509ff0f2ce1#card` | 1 | `Noun(Card)` |
| Witch's Cauldron | `cb65ee08-bfdb-4c18-8967-8bcf31fcabfa#card` | 2 | `Noun(creature)` |
| Withered Wretch | `29e13ec1-a239-4217-a2bd-24ff90745ec0#card` | 4 | `Noun(Graveyard)` |
| Wojek Embermage | `ac8630ac-defb-4774-ac71-d35fe02a7324#card` | 68 | `Noun(Color)` |
| Woodland Acolyte | `ba610f08-61a7-47fe-9a80-32b5ce51b118#face:0` | 1 | `Noun(Card)` |
| Woodland Liege | `df63e124-1542-48d6-b255-cf45855f1e93#card` | 2 | `Noun(Card)` |
| Wurm's Tooth | `69476326-fda5-48c8-a506-e71eb7c81c2f#card` | 2 | `Noun(Player)` |
| Xira Arien | `4a6e367c-7bc9-44a3-8ede-f2d0651abad3#card` | 1 | `Noun(Card)` |
| Yuyan Archers | `63e5180f-16d0-4524-94bf-ecbf3c417814#card` | 8 | `Noun(Card)` |
| Zuran Orb | `08cb8a30-9cb4-4517-bee5-8848aa60d1a2#card` | 2 | `Noun(land)` |
