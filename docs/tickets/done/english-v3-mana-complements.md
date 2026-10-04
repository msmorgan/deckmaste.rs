---
needs: []
---
# Realize declared mana complements and fill ordinary lexical gaps

Connect the already declared ManaPhrase complement frame to structured mana
symbols, finite predicates and secondary verb phrases. Preserve cost-symbol
licensing separately from produced mana. Positive authentic witnesses include
Ancient Den, Wastes, Llanowar Elves and Nantuko Elder; negative witnesses must
exclude tap, energy and variable cost symbols from mana output. Selected frames,
lexical ownership, agreement and independent constructed-value laws apply.

Connect the separately declared numeric Amount complement used by Scry and
Connive to structured quantities and finite/secondary hosts. Preserve the
macros as verb owners and the selected frame rather than adding a named-verb
exception. Authentic witnesses include Reason and Sigiled Starfish.

Add missing ordinary Assign and Change verb declarations and Single and Extra
adjectives with complete regular morphology and grammatical features. Keyword
action verbs remain declared by their macros. Corpus counts, unknown-word
coverage and complete grammatical coverage are distinct measures.

Standard constraints apply. Verify selected subsets first and the complete
supported corpus with at least eight workers before landing. Disclose every new
covered identity and selected analysis, any removed analyses or identities,
structural checks and performance evidence in the landing record.

## Landing record

### PROVE

The required reverse-dependency gate derives
`cargo test -p deckmaste_lexical_source -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask`.
The gate passes, including the existing Lean integration checks. Independently constructed
lexical paradigms, mana sequences, finite/secondary selected complements and an
Amount imperative assert exact values, frames, realization and traversal.
All prior tests remain; added 8, removed 0, restored 0, re-spelled 0, newly
ignored 0. Existing ignored tests remain unchanged.

The complete eight-worker text corpus has zero issues, internal failures,
cycles, duplicate derivations, incomplete enumeration or undetermined counts.
Every admitted Reading is counted and validated. No previously covered face
is lost; all 3,774 previously covered faces retain identical exact counts and
identical cheapest-sample fingerprints. The declaration edits are additive:
old productions and frame signatures remain unchanged, and existing cost
symbols retain their licensing. No word/card/construction-named admission
checker or source-buffer catch-all was added.

All 706 trees of the 309 new covered identities were stored and structurally
reviewed. New mana hosts are Add/Pay only; all 832 mana-symbol occurrences are
W/U/B/R/G/C, with no tap, untap, energy, variable, snow or numeric-cost-symbol
leak. New Amount hosts in this corpus are Scry only, with no unintended
Draw/Deal admission. The four lexical gains use Change's transitive frame and
Single as an adjective. No definite invalid newly admitted analysis was found.
Surrounding nominal/PP scope, coordination and ellipsis alternatives remain
retained; differing game interpretations alone do not make them invalid.
Type lines all retain exactly one Reading with zero validation issues.

### DISCLOSE

New covered faces classify as 238 mana-only, 65 Amount-only, 2 both, and 4
ordinary lexical gains; 241 have one Reading and 68 have multiple. Every
identity and its added analysis is listed below. Assign and Extra clear lexical
gaps but yield no fully parsed face in this batch. No silent retirements or
STOP resolutions are required by this additive change.

Deviations and additions: numeric Amount realization was added after triage
exposed the shared missing frame behind Scry. It preserves the macros as verb
owners and supports word-cardinal quantities as well as unsigned Arabic/grouped
notation and declared variables. Added constructions: ManaPhrase,
NamedManaSymbol, FiniteManaComplement, SecondaryManaComplement, CardinalAmount,
ScalarAmount, FiniteAmount and SecondaryAmount. Added categories: ManaPhrase,
ManaSymbol and Amount. Added feature: ManaSymbolUse, authored on six existing
symbol identities without replacing SymbolUse=Cost. Added test files: mana.rs
(3 tests), amount_complements.rs (2) and next_lexical.rs (3). No existing
construction or test was removed. The glossary now defines Mana Phrase and
Amount Complement.

Scope limits: mana choice coordination, generic/hybrid/phyrexian costs in Pay,
and prose such as mana-of-any-color remain grammar obligations, not inferred
from output symbols. These and the existing until-end-of-turn and compound/
modifier gaps remain with english-v3-systemic-residuals. The keyword-label WIP
owned by keyword-bodies-over-helpers is untouched. Current cheapest readings
of Swerve/Shunt/Deflection attach with-a-single-target to the outer target noun;
the intended spell attachment is also retained. Cost ranking needs further
tuning; preference does not erase these grammatical alternatives.

### REPORT

Measured 2026-10-04, change ymppovouvtrznkpsrrvtwszptqlnytvl, eight workers,
32,828 Vintage-supported faces. Source SHA-256:
49dc966bda6ef588fc68e8d6972de25df1b660ed2ea654904584698263007ebb.
Lexical inventory SHA-256:
8f59014de9866aaff5f93bcfdfdf8e2bc3217e446ecb1e518772cbaccf847b65.
No English coverage-lock field exists in this pipeline; CR lock coverage is
unrelated. Construction count: 170 before, 178 after.

| Text census | Before | After |
| --- | ---: | ---: |
| No Reading | 29,054 | 28,745 |
| One Reading | 2,644 | 2,885 |
| Multiple Readings | 1,130 | 1,198 |
| Covered faces | 3,774 | 4,083 |
| Exact Readings | 12,308 | 13,014 |
| Faces with normalized vocabulary gaps | 3,298 | 3,138 |
| Distinct missing spellings, case-folded | 1,201 | 1,193 |

Vocabulary gaps completely clear on 160 faces; 156 still fail grammar. Exact
case-sensitive missing spellings decrease 1,266 to 1,256: Assign, Change,
Changing, assign, assigned, assigns, change, changed, extra, single are removed.
The manifest has 34,832 owners, including 1,222 noncatalog-source owners,
120 verbs, 551 nouns and 44 adjectives. All 43,110 independent lexical values
pass. Raw lexical analysis includes reminders: 9,182 unknown occurrences across
1,280 spellings; this differs from normalized parser-input accounting.
New lexical forms retain any existing homographs; catalog-name recognition does
not license a Verb projection. No form-literal vocabulary admission override
was introduced. Keyword-action lexical ownership and morphology remain in their
macro declarations; the new Assign/Change paradigms use regular defaults.

Debug text corpus: 42.420 seconds wall, 512,605 ns/B checked-text thread CPU,
host load [7.538, 3.580, 1.577]. Debug type-line corpus: 4.003 seconds,
48,673 ns/B, load [9.063, 6.306, 2.837]. Both use eight workers. These runs
coincided with dependency compilation/other verification on a loaded host;
they are above the 16.26-second quiet-host advisory and are not optimized
performance/cutover claims.

Scratch evidence: /tmp/english-v3-next-final-text.json,
/tmp/english-v3-next-final-types.json, /tmp/english-v3-next-gain-trees.json,
/tmp/english-v3-next-delta.json, /tmp/lexical-next-final.json. Reproduce full
checks with `cargo xtask english-v3 --all --workers 8 --samples-per-face 1`
and `cargo xtask lexical --all --workers 8`; full gain enumeration uses the
identity manifest and samples-per-face 1000000 without a Reading limit.
Citation audit reports zero noncompliant sites and zero stale citations.
Changed Rust files pass nightly formatting. Whole-package formatting and
strict clippy still report pre-existing unrelated violations; no unrelated
formatting or warning repairs were included.

### New covered identities

| Face | Oracle identity | Readings | Added analysis |
| --- | --- | ---: | --- |
| Adventurer's Inn | 232bd88c-ecdb-43dd-b34a-d381cb3bedf2#card | 2 | selected mana complement |
| Agent of Stromgald | 099fd4b3-92fd-4b5f-bb4f-eefa6f09700c#card | 1 | selected mana complement |
| Akki Rockspeaker | 01cd52f9-7fac-4396-bec7-d06bf683e011#card | 1 | selected mana complement |
| Ancient Den | 02f16726-f2f6-4943-b71a-93f8e26251d3#card | 1 | selected mana complement |
| Ancient Tomb | 23467047-6dba-4498-b783-1ebc4f74b8c2#card | 4 | selected mana complement |
| Apprentice Wizard | d16d2451-1c21-4a6b-8a4a-40ac8cd9fe08#card | 1 | selected mana complement |
| Archive Dragon | cddbd819-4895-4712-b44b-d6b51f3d8646#card | 1 | selected Scry Amount |
| Ashnod's Altar | 4d18bcba-a346-445e-a182-6cc30b7e066d#card | 1 | selected mana complement |
| Augury Owl | 44c5f8e1-d9b8-4067-a60a-1ecc8bd11145#card | 1 | selected Scry Amount |
| Automatic Librarian | eb570331-35da-44b8-b711-ccf7444d8e39#card | 1 | selected Scry Amount |
| Avacyn's Pilgrim | 069f6530-e65c-4d52-85f3-e0a2acd148c5#card | 1 | selected mana complement |
| Azorius Signet | e018773f-95b3-49a3-9674-6f04ddef2092#card | 1 | selected mana complement |
| Barkchannel Pathway | 59d22de5-e310-44d7-89cf-ef3529e40cef#face:0 | 1 | selected mana complement |
| Basal Thrull | 9da50130-3f83-4968-983c-7dcba257cf1b#card | 1 | selected mana complement |
| Battle Hymn | 4b977117-741c-4157-8fcf-719e475a8a4c#card | 4 | selected mana complement |
| Black Tom Cassidy | c0d20e66-deb6-4a20-a2c1-8a79514e3792#card | 4 | selected mana complement |
| Blasted Landscape | 9c8007ac-4b3d-4444-93e9-f583185e5d81#card | 1 | selected mana complement |
| Blighted Cataract | 4d722521-0396-48f9-88b2-08b6249f970d#card | 1 | selected mana complement |
| Blighted Gorge | c2cb0afd-781f-4cfa-b680-ed1edfa81868#card | 2 | selected mana complement |
| Blighted Steppe | db16a2fb-dc42-4086-9928-52076043097f#card | 8 | selected mana complement |
| Blightsoil Druid | 351091cb-1263-4202-9447-021c6019cff1#card | 1 | selected mana complement |
| Blightstep Pathway | e580a229-e800-4746-9d37-c32fcef8de28#face:0 | 1 | selected mana complement |
| Blood Pet | e05c6c80-a91a-45e0-b991-0014fd5a6472#card | 1 | selected mana complement |
| Blood Vassal | f792f40d-1d37-4854-98da-e4020c6a44d4#card | 1 | selected mana complement |
| Bog Initiate | 23f93411-f83c-4ed2-abed-99cf905f7d7f#card | 1 | selected mana complement |
| Bog Witch | 85573cba-07ae-4421-a167-a8569f85c0f7#card | 1 | selected mana complement |
| Boreal Druid | 2fcc69ff-8ab5-4e14-afe3-db892049a872#card | 1 | selected mana complement |
| Boros Signet | 41c84665-1f99-40ab-aaca-1188649eb263#card | 1 | selected mana complement |
| Boulderloft Pathway | 7c304547-a4b1-46c9-baed-16d2bfbe16eb#face:1 | 1 | selected mana complement |
| Branchloft Pathway | 7c304547-a4b1-46c9-baed-16d2bfbe16eb#face:0 | 1 | selected mana complement |
| Brightclimb Pathway | 1c633e02-95ef-445e-b4e0-fbfbc5ed9cc9#face:0 | 1 | selected mana complement |
| Brightstone Ritual | 08e90e85-4103-4acb-a8a7-e1329b460aa7#card | 5 | selected mana complement |
| Burning-Tree Emissary | 327d9679-0049-4401-8dab-e0fb362306bd#card | 1 | selected mana complement |
| Cabal Coffers | 7358e164-5704-4e78-9b21-6a9bf2a968ce#card | 4 | selected mana complement |
| Candy Trail | 7c8e4f29-53e0-4242-8ef8-7a9476dce178#card | 4 | selected Scry Amount |
| Canopy Tactician | 8b20d6f4-6322-4435-92d5-acaae74774f4#card | 4 | selected mana complement |
| Carnival of Souls | 95b10ca7-7360-4da5-bd93-686ae3051833#card | 6 | selected mana complement |
| Catalyst Elemental | 7779525d-080d-4669-bb65-af532bf0983e#card | 1 | selected mana complement |
| Cathodion | fcd4f816-2de1-4b30-82fb-cb87f45747ea#card | 1 | selected mana complement |
| Channel the Suns | d1b1be96-b2aa-4386-97d2-eb0a8cc47210#card | 1 | selected mana complement |
| Chorus of the Tides | 48925016-8a55-4efb-84c7-dc5001baa58e#card | 4 | selected Scry Amount |
| Chrome Cat | 0e2f8758-7457-4b54-ae61-a9c68cea4a23#card | 1 | selected Scry Amount |
| Circle of Dreams Druid | 1c857fa4-e823-494d-9ea4-7eb9e719c9b6#card | 4 | selected mana complement |
| City of Traitors | f161111d-9747-47b3-bb10-3c8bded32e21#card | 2 | selected mana complement |
| Clearwater Pathway | 144119bc-7fd1-45c5-9e29-f742e7c255ac#face:0 | 1 | selected mana complement |
| Cloudreader Sphinx | b5a4bf91-4d31-4557-9ec4-a96ff4d9be6b#card | 1 | selected Scry Amount |
| Coal Golem | 64b63847-27dd-469b-aad3-58e061f92817#card | 1 | selected mana complement |
| Composite Golem | 784970de-ef8c-4672-b5d0-24a4b0a978a3#card | 1 | selected mana complement |
| Contraband Kingpin | 680da572-6cd5-482a-b01d-32944566d8c4#card | 2 | selected Scry Amount |
| Copper Myr | 8b52f30c-5e38-4333-88ab-901b37105b36#card | 1 | selected mana complement |
| Cragcrown Pathway | 727ca426-f4cc-4218-8ae5-8c427af2e816#face:0 | 1 | selected mana complement |
| Crosis's Attendant | 6223b1c0-bfe0-490c-b2d4-28537b05f571#card | 1 | selected mana complement |
| Crystal Ball | bd85fe4d-1d62-416f-ac2d-e287911c84e3#card | 1 | selected Scry Amount |
| Crystal Quarry | ca68648f-fe3a-4770-9842-a3dc2310f099#card | 1 | selected mana complement |
| Crystal Vein | 616d6013-24f4-4999-9bf3-5b0764e52fa6#card | 1 | selected mana complement |
| Curator of Mysteries | 04858b93-0a2a-44d0-942d-7fadf589cbb7#card | 2 | selected Scry Amount |
| Darigaaz's Attendant | c0ad16d7-2550-47e0-8581-0e22b27cb0d0#card | 1 | selected mana complement |
| Dark Ritual | 53f7c868-b03e-4fc2-8dcf-a75bbfa3272b#card | 1 | selected mana complement |
| Darkbore Pathway | 868e6e68-4367-4073-a864-235d5961ae56#face:0 | 1 | selected mana complement |
| Darksteel Citadel | 8dc067bf-f78f-4ac4-b6e7-b305c42cf0bc#card | 1 | selected mana complement |
| Darksteel Pendant | 431838a8-f020-4e4e-a6f4-2d4ca27c56df#card | 1 | selected Scry Amount |
| Darkwater Catacombs | 4869a530-757f-4364-8d8e-4dc8001f433c#card | 1 | selected mana complement |
| Darkwater Egg | c4893a24-3cbc-4011-bef4-50e0c4dce16e#card | 1 | selected mana complement |
| Deconstruct | 36a8ceb1-148b-41e0-a7bf-ceb879bf08e7#card | 1 | selected mana complement |
| Deflection | ec7ae9ed-dc5b-47ed-a4ad-086f3c7c377c#card | 4 | transitive Change; Single adjective |
| Deranged Assistant | 9e1b0034-bd2c-4412-8d81-db3306c8814f#card | 1 | selected mana complement |
| Deserted Temple | 9f12bf9a-6e1a-4377-b4af-e8cabd3ee58a#card | 1 | selected mana complement |
| Desolate Mire | 3edf9201-265f-4cd9-b27b-8073bf1a4cf2#card | 1 | selected mana complement |
| Dimir Signet | 7d881c57-0bd9-4c57-aa4a-b10808b86143#card | 1 | selected mana complement |
| Dissolve | 2f0741f9-802b-4761-8094-7b1f3ea1a424#card | 1 | selected Scry Amount |
| Drainpipe Vermin | b6d39905-e074-4517-b806-d081d1069f74#card | 8 | selected mana complement |
| Dream Beavers | a1cec8e8-cf3c-42a4-b625-ca0cd148664e#card | 4 | selected Scry Amount |
| Dreamstone Hedron | 0e2575be-c596-4c8c-bf07-7941ca065721#card | 1 | selected mana complement |
| Dromar's Attendant | 91d36a9d-3a3f-4cb7-837d-b1e8f6730512#card | 1 | selected mana complement |
| Druid of the Cowl | b1793c3b-25d6-4fae-a99d-cfdd2210ca67#card | 1 | selected mana complement |
| Duskmantle, House of Shadow | 67b2cd0c-ecc8-4129-b1ac-820c9924190c#card | 1 | selected mana complement |
| Eager Construct | 39363190-7f79-4fae-bcfc-b17ec4e07bfe#card | 1 | selected Scry Amount |
| Elephant Graveyard | 8ada7388-fd8b-434c-a17a-bce19cf3e615#card | 1 | selected mana complement |
| Elves of Deep Shadow | 20347559-95a9-4689-bb79-c5bb3809b719#card | 4 | selected mana complement |
| Elvish Mystic | 3f3b2c10-21f8-4e13-be83-4ef3fa36e123#card | 1 | selected mana complement |
| Eye of Ramos | a1bdea9f-56d0-411a-9da8-601dd6dc6d32#card | 1 | selected mana complement |
| Faerie Seer | b2e65e8b-5f08-4cc2-ab1d-00f8903dbea2#card | 1 | selected Scry Amount |
| Ferrous Lake | 62c15af0-40e1-407d-b056-7a3d909e3fdb#card | 1 | selected mana complement |
| Fill with Fright | 2abaab80-1d0d-4670-8e9e-f4dfc1778f98#card | 1 | selected Scry Amount |
| Fire Sprites | fc5e42b5-4da2-4777-828b-138c0a5d234f#card | 1 | selected mana complement |
| Fogwell's Gym | 850bb6f7-48d3-4d65-9220-b0bec5ee6b64#card | 4 | selected mana complement |
| Frenzied Goblin | 6ac470a1-c2be-4971-a5ca-10bb189ebe4d#card | 8 | selected mana complement |
| Fyndhorn Elder | 507bdb1a-90b4-4fd3-a1a3-e8be316e97f3#card | 1 | selected mana complement |
| Fyndhorn Elves | df317532-7d36-40fd-938f-e972749c8792#card | 1 | selected mana complement |
| Gaea's Cradle | 7c427c3d-ecd8-45ef-bebd-8f10f4a311db#card | 4 | selected mana complement |
| Galadhrim Guide | 9204de14-cbb6-40e5-9dcc-44cce3723e8e#card | 1 | selected Scry Amount |
| Get the Point | 57c704bc-8dff-4b90-b4e1-38d12e41b3d1#card | 1 | selected Scry Amount |
| Glider Kids | ff5ce28b-25be-4136-a7b2-8f25c08a6455#card | 1 | selected Scry Amount |
| Glimmerpost | 92c9aad6-35ec-425d-be7d-393328992820#card | 18 | selected mana complement |
| Glimpse the Sun God | 87cc3a67-b4f0-41f0-9e34-745ef23aa479#card | 1 | selected Scry Amount |
| Gold Myr | bd6af7b3-b30f-4a65-a18f-8655f778e76a#card | 1 | selected mana complement |
| Golden Hind | 82679258-345c-4062-ac62-e94a1e5ace78#card | 1 | selected mana complement |
| Golgari Signet | 1cf51f50-24e4-48d0-95b3-1dad3ffa4bf5#card | 1 | selected mana complement |
| Goobbue Gardener | b5c64331-2ba5-43ad-b77c-979875cc39e3#card | 1 | selected mana complement |
| Great Furnace | f4819061-b0b5-48ab-af7b-6525c3d2eab7#card | 1 | selected mana complement |
| Greenhouse Propagator | b6b77cb9-49a5-4d1a-ae2d-02ae176dc1fa#card | 4 | selected mana complement |
| Greenweaver Druid | 5e35a7c9-1e6e-41a7-9f59-5ea1bc53bc65#card | 1 | selected mana complement |
| Grey Havens Navigator | 14291205-1ac2-4804-b365-4f65377d5803#card | 1 | selected Scry Amount |
| Grim Backwoods | 5effaa94-7f87-4485-8959-473d584c5034#card | 1 | selected mana complement |
| Grimclimb Pathway | 1c633e02-95ef-445e-b4e0-fbfbc5ed9cc9#face:1 | 1 | selected mana complement |
| Groundchuck & Dirtbag | da344e46-6bdf-4032-8590-bac850d5713d#card | 4 | selected mana complement |
| Gruul Signet | d36e0c9f-c025-4dfe-9644-9cad2461ce38#card | 1 | selected mana complement |
| Gyre Engineer | 27eef5c1-259a-4d37-93af-7451f50905af#card | 1 | selected mana complement |
| Gyre Sage | e3aa16cf-079b-4737-9ffd-7bfdffef0cb2#card | 7 | selected mana complement |
| Haazda Snare Squad | 2a174adc-0536-4e12-a6fe-c44305822714#card | 8 | selected mana complement |
| Heart Warden | babcc551-39bb-4b1c-92a0-e60b236a59b2#card | 1 | selected mana complement |
| Heart of Ramos | 4c774c6e-c5a0-4018-b494-d3c521d2cac3#card | 1 | selected mana complement |
| Hedron Archive | 32263baa-d3f0-463f-92b3-4e9938476add#card | 1 | selected mana complement |
| Hedron Crawler | 0b9a4e06-b21d-4cbe-906f-9dbd08dbe5d3#card | 1 | selected mana complement |
| Hengegate Pathway | 461b3f2f-fcee-4160-abfa-061f8b6a784f#face:0 | 1 | selected mana complement |
| Hierophant's Chalice | 52732f93-b951-43c3-bb02-c254996606e5#card | 4 | selected mana complement |
| High Market | 86fb3749-37d6-48a6-8524-71e996850307#card | 2 | selected mana complement |
| Hithlain Knots | 2c80fc55-86a0-403e-82ce-09ee03095d8a#card | 1 | selected Scry Amount |
| Holy Cow | 748003fd-84c5-4c99-b146-d7d382eef64d#card | 6 | selected Scry Amount |
| Homeward Path | cb8ec2e4-8223-4172-8f2c-37c918a573fa#card | 2 | selected mana complement |
| Horizon Scholar | 29933a32-3738-4ec5-aa6d-ce3684e2e5a3#card | 1 | selected Scry Amount |
| Horn of Ramos | 3b9bcf88-7304-48c5-bd46-3e37a93967a5#card | 1 | selected mana complement |
| Infernal Idol | a0802918-e18a-4716-befb-96ae6667f2f3#card | 4 | selected mana complement |
| Ipnu Rivulet | c17d799f-adc9-4c41-87cf-b243b5ea3be1#card | 1 | selected mana complement |
| Iridescent Tiger | ce09c5b0-9dd4-4f73-9f3f-dfbd829813e4#card | 4 | selected mana complement |
| Iron Myr | 6c5cbab6-ee27-46f5-97a7-df85698d1e9f#card | 1 | selected mana complement |
| Itlimoc, Cradle of the Sun | ea9c459a-6047-43aa-968f-a582be4000e8#face:1 | 4 | selected mana complement |
| Izzet Signet | 2fda4fe7-8b0c-489c-a000-6d358e614e34#card | 1 | selected mana complement |
| Kalastria Highborn | 77e319da-d782-4258-bf0e-626755e5adc9#card | 64 | selected mana complement |
| Kaleidostone | 498ec48f-cb28-48d4-96f4-9d60bb4bb45c#card | 1 | selected mana complement |
| Karn's Bastion | 9fb8cd81-403a-4988-8f1c-b8eccf8abd9c#card | 1 | selected mana complement |
| Knight of the Mists | e9893534-3d05-42c2-93ff-f28ffae89aa3#card | 16 | selected mana complement |
| Knotvine Mystic | 5d045695-e4b3-4daa-bfab-d77640fedfd6#card | 1 | selected mana complement |
| Kozilek's Channeler | 33826f91-7680-4a4c-be39-db6c2c0bd465#card | 1 | selected mana complement |
| Krark-Clan Ironworks | 68e1f7e0-a9b3-437f-8086-0c0cb85f2880#card | 1 | selected mana complement |
| Krark-Clan Stoker | 756b4bc9-6f3f-4eae-b6b0-63c94ed8482f#card | 1 | selected mana complement |
| Lavaglide Pathway | 4924b3a4-a218-4783-8a4d-82361fdecc78#face:1 | 1 | selected mana complement |
| Leaden Myr | f62cabf0-df0d-4c4f-a93a-9340967d1775#card | 1 | selected mana complement |
| Leaf Gilder | 61324e37-4b79-4325-bf46-621b4270afe2#card | 1 | selected mana complement |
| Lightning Cloud | 4a574140-0657-4f49-b497-331447f17b29#card | 24 | selected mana complement |
| Liturgy of Blood | aabea4a2-5107-4abd-a0c7-ac0024d98ae0#card | 1 | selected mana complement |
| Llanowar Dead | 3a48541a-240b-4406-945c-1d0d032eca25#card | 1 | selected mana complement |
| Llanowar Elves | 68954295-54e3-4303-a6bc-fc4547a4e3a3#card | 1 | selected mana complement |
| Llanowar Tribe | cfd0f0e6-1bbf-4450-97d2-c54907abb7e4#card | 1 | selected mana complement |
| Llanowar Visionary | f75ed312-3a23-4624-80c5-03980aa22d0b#card | 1 | selected mana complement |
| Lost Legion | b8f3b5b5-8835-400f-a626-94ff0a007e56#card | 1 | selected Scry Amount |
| Lothlórien Lookout | ed839fae-3429-4ac5-997c-286d8e8cf05f#card | 1 | selected Scry Amount |
| Magnifying Glass | f2ce9e55-07c0-4f36-a59e-127693fc0f62#card | 1 | selected mana complement |
| Magus of the Coffers | 23dd895c-92bd-4af0-8b1a-7d76ca49178f#card | 4 | selected mana complement |
| Manakin | d2343af0-468b-42bc-8a0c-347c10f7e2f3#card | 1 | selected mana complement |
| Mardu Warshrieker | fe9e9e2e-f304-4da3-a257-6886b0b1896b#card | 2 | selected mana complement |
| Memory Drain | 1a61929e-14bd-4f21-b173-00818d49743d#card | 1 | selected Scry Amount |
| Merfolk Falconer | 77d5d435-d794-46c0-b630-a460973cf75a#card | 4 | selected Scry Amount |
| Mikokoro, Center of the Sea | a4580a1d-141e-449b-9018-e0258130634b#card | 1 | selected mana complement |
| Millikin | fa4dffda-6f04-4d0b-829d-28a1a5794dee#card | 1 | selected mana complement |
| Minamo, School at Water's Edge | 17784f90-89a1-47a5-83ef-ae60dfc30bd1#card | 1 | selected mana complement |
| Mind Stone | c97361b5-af16-4a7b-af85-a429dbaf4ad2#card | 1 | selected mana complement |
| Mistgate Pathway | 461b3f2f-fcee-4160-abfa-061f8b6a784f#face:1 | 1 | selected mana complement |
| Morgue Toad | c615540b-bf85-4e16-8ac2-9d541cba732f#card | 1 | selected mana complement |
| Mossfire Egg | 364b3231-c0e7-45a8-90b9-a1cdb584dd7c#card | 1 | selected mana complement |
| Mossfire Valley | 23bbd091-f6ff-4514-97aa-42c08164b4eb#card | 1 | selected mana complement |
| Mouth of Ronom | 7c05d239-39fc-4d34-a853-e3d591f4a235#card | 2 | selected mana complement |
| Mox Emerald | 376ee366-e082-402f-b4db-6592fcfcacd2#card | 1 | selected mana complement |
| Mox Jet | 0677f49e-f8bf-4349-af52-2ccde9287c2e#card | 1 | selected mana complement |
| Mox Pearl | 824597b8-c89a-47ec-8526-7efc6e24ef0e#card | 1 | selected mana complement |
| Mox Ruby | ed85fa82-e4fa-434b-92a8-36b6075708d1#card | 1 | selected mana complement |
| Mox Sapphire | d5ed1233-df87-4b90-8918-13922ec95249#card | 1 | selected mana complement |
| Murkwater Pathway | 144119bc-7fd1-45c5-9e29-f742e7c255ac#face:1 | 1 | selected mana complement |
| Myr Moonvessel | 3e922661-80df-4e84-a12a-524bc74e6c9d#card | 1 | selected mana complement |
| Mystic Speculation | a58adf82-d9e2-4b47-933e-68180f280ba2#card | 1 | selected Scry Amount |
| Nantuko Elder | 001c233f-2959-479b-a82a-64a25ac60830#card | 1 | selected mana complement |
| Needleverge Pathway | a9b8d020-4d72-4934-8942-df29ef19fc1d#face:0 | 1 | selected mana complement |
| Nephalia Drownyard | 6429b4ed-1845-4643-9a3d-85f7c12f2bba#card | 1 | selected mana complement |
| Noxious Newt | 1232c461-643c-40a3-901e-09b665f817da#card | 1 | selected mana complement |
| Octoprophet | af6cfc23-b64b-4c8a-94b4-69d2b01cb3b9#card | 1 | selected Scry Amount |
| Oggyar Battle-Seer | 645005dd-3438-4b88-bb19-561f9679a022#card | 1 | selected Scry Amount |
| Omen of the Forge | e11e2dc8-b28e-415d-9a5d-0eb46da8945c#card | 3 | selected Scry Amount |
| Omenspeaker | 43d7fcf3-acc2-4e9b-a466-c73b1b6c58af#card | 1 | selected Scry Amount |
| Opt | 713332c1-5bd8-400f-bfff-c1ca0697a043#card | 1 | selected Scry Amount |
| Orochi Sustainer | d3039b42-25d5-4fe2-955a-0d910d7395bd#card | 1 | selected mana complement |
| Orzhov Signet | de3dcb5d-775a-479f-99f5-d1883ed9b1b5#card | 1 | selected mana complement |
| Orzhova, the Church of Deals | 8551a9cf-c54b-42d4-92d6-550f4890a3d7#card | 2 | selected mana complement |
| Overeager Apprentice | 3ea71eed-e1ae-4e28-a4f5-44ff115949f4#card | 1 | selected mana complement |
| Overflowing Basin | 5ac8e01c-b0a7-4855-a122-1cd26b07c4a5#card | 1 | selected mana complement |
| Palladium Myr | 7b0767b8-b504-456e-93bd-218502f73b3d#card | 1 | selected mana complement |
| Phyrexia's Core | b6cc062c-eb39-46ee-bd6d-17f1db0ac50d#card | 2 | selected mana complement |
| Phyrexian Tower | 1861e642-21d5-4232-89f3-b5557f2946c1#card | 1 | selected mana complement |
| Phyrexian Vivisector | 9c99ce14-5059-47a9-b874-637b39473062#card | 2 | selected Scry Amount |
| Pillarverge Pathway | a9b8d020-4d72-4934-8942-df29ef19fc1d#face:1 | 1 | selected mana complement |
| Plague Myr | 2f328e05-5edf-4b21-9c2a-50dcf1e7b3ec#card | 1 | selected mana complement |
| Powerstone Shard | 769c45ae-5d49-4815-944e-98790ad4efb0#card | 4 | selected mana complement |
| Priest of Gix | d93f82ce-0eed-45cc-a7b1-50fd4cbb6152#card | 1 | selected mana complement |
| Priest of Titania | 3a198a16-17b9-481e-b516-5bc945c7e247#card | 5 | selected mana complement |
| Priest of Urabrask | e4bd8910-770b-4220-8a26-2673491f4a3e#card | 1 | selected mana complement |
| Princess Lucrezia | dd1d21f5-f5cd-43e4-8155-3cca85da80a9#card | 1 | selected mana complement |
| Pristine Talisman | 1b3d7fce-e9fe-4176-9d9a-472415826cdd#card | 2 | selected mana complement |
| Prophet of the Peak | eaa1634e-6ba1-41c2-91c0-967b36d13e14#card | 1 | selected Scry Amount |
| Pyretic Ritual | 86ef6474-613f-41fb-931c-d4279b03ed99#card | 1 | selected mana complement |
| Radiant Fountain | 6db442e5-fbcc-4456-a4c5-bea1aee3fc8e#card | 2 | selected mana complement |
| Rakdos Signet | 3adb7681-977f-4a32-9ec8-51481b958268#card | 1 | selected mana complement |
| Ramunap Ruins | d0d35864-1edc-4af1-9b89-3d7e94908011#card | 2 | selected mana complement |
| Rats' Feast | 327c5ca2-4eeb-49cb-84fb-f6a4b06b24bc#card | 4 | Single adjective in source PP |
| Reason | e2b152ea-82d5-465e-8e13-3d62aacd6692#face:0 | 1 | selected Scry Amount |
| Reckless Barbarian | a621ae15-122a-4e11-bfcf-11dfc09784c0#card | 1 | selected mana complement |
| Rishadan Port | f3e8dc56-2810-474e-a6a7-9c3555f94ae9#card | 1 | selected mana complement |
| Rith's Attendant | 781542b7-155b-4a66-aa52-eec0ebb29bb2#card | 1 | selected mana complement |
| Riven Turnbull | 2742d506-897f-4d30-ba43-ce0374984849#card | 1 | selected mana complement |
| Riverglide Pathway | 4924b3a4-a218-4783-8a4d-82361fdecc78#face:0 | 1 | selected mana complement |
| Roadside Reliquary | 2fb13687-0518-4ba0-a5ae-dd609464b026#card | 36 | selected mana complement |
| Rofellos, Llanowar Emissary | 3015882b-4897-4d2c-8e33-6731c85a0d03#card | 4 | selected mana complement |
| Rubble Reading | 894d96b1-c82a-4141-a2f3-76da505bd5e5#card | 1 | selected Scry Amount |
| Rumbling Sentry | 7126772a-d586-4859-81ec-46bbe9d1a678#card | 1 | selected Scry Amount |
| Sage's Row Savant | 6eaf5cdc-2843-4167-bd18-df4c2ac73ac0#card | 1 | selected Scry Amount |
| Sapphire Dragon | 53015c41-af2d-43e2-a690-d8877537b8dd#face:0 | 1 | selected Scry Amount |
| Satyr Hedonist | a75bf714-ed3e-4403-a009-3a81f430bc64#card | 1 | selected mana complement |
| Scavenger Grounds | 5ece7d03-9ee7-4953-a06e-9d8e41874903#card | 1 | selected mana complement |
| Sea Scryer | f17954dd-c8e0-4983-b81e-5440cca41afc#card | 1 | selected mana complement |
| Searstep Pathway | e580a229-e800-4746-9d37-c32fcef8de28#face:1 | 1 | selected mana complement |
| Seaside Haven | 4adc39dd-8de1-4298-947c-ff666ec3adeb#card | 1 | selected mana complement |
| Season of Growth | 3f4e300a-ec5c-42f3-a97b-d58e62abe22b#card | 32 | selected Scry Amount |
| Seat of the Synod | 39451b4d-cd7a-40da-b457-cb51b609173f#card | 1 | selected mana complement |
| Seer's Lantern | 4b3dd7e5-e0b5-4773-bb16-37b757ae74d8#card | 1 | selected mana complement; selected Scry Amount |
| Seething Song | 1bf505a7-292e-4642-83b6-8ff41ebe5d51#face:1 | 1 | selected mana complement |
| Seething Song | 64bf8929-f5f2-4d50-8667-13b1d007bcfc#card | 1 | selected mana complement |
| Seismic Spike | 25ec6309-0ccb-4f27-80f1-ec6dcc6461fa#card | 1 | selected mana complement |
| Selesnya Signet | 1436dd81-496e-42a5-b210-fb5b9cdf073f#card | 1 | selected mana complement |
| Senate Griffin | 7ddf9c12-8f75-481c-a218-d42f925c6503#card | 1 | selected Scry Amount |
| Sentinel Totem | 5432bd43-e92c-464b-9ad2-3515f3042799#card | 1 | selected Scry Amount |
| Seraph Sanctuary | 0b504dc6-61cc-4a72-907c-145fa4c72466#card | 8 | selected mana complement |
| Serra's Sanctum | 34187c71-6033-4058-aadc-2bc266f762be#card | 4 | selected mana complement |
| Serum Visions | 56956afd-db53-4542-816b-490c8b0bbcf7#card | 1 | selected Scry Amount |
| Shadowblood Egg | 9b58e7fa-4259-4d6b-8b5d-33fb37fe489f#card | 1 | selected mana complement |
| Shadowblood Ridge | 15687ee3-3cdb-4a8f-a726-46b73bceb792#card | 1 | selected mana complement |
| Shunt | e714bcfb-b451-4b7b-ab60-4cb845c75647#card | 4 | transitive Change; Single adjective |
| Sigiled Skink | 4c17f6c3-48d3-4bd0-b0dd-76229d9c5b09#card | 1 | selected Scry Amount |
| Sigiled Starfish | 94a4f70a-8aac-412a-99f0-933e3ff87fe0#card | 1 | selected Scry Amount |
| Silver Myr | 66e8f7f8-3a6d-46ba-837c-b9713ddf7f40#card | 1 | selected mana complement |
| Silver Raven | 4da64806-8a1d-4926-a168-695df43309cf#card | 1 | selected Scry Amount |
| Simic Signet | 44503105-3e13-408d-a44f-37d503c61d72#card | 1 | selected mana complement |
| Sisay's Ring | 8f5822ae-651f-410e-9316-5522eeb52d72#card | 1 | selected mana complement |
| Sisters of the Flame | 389a9d46-d3fb-47f0-91cd-f6d487636916#card | 1 | selected mana complement |
| Skirge Familiar | ba95f24d-42da-48ce-bcf1-1b7c4b3c45b5#card | 1 | selected mana complement |
| Skirk Prospector | c18013e4-0b99-44e3-a2b2-027ace68723a#card | 1 | selected mana complement |
| Skull of Ramos | b5ae2532-e642-47d1-bb5f-53f408e2fdc2#card | 1 | selected mana complement |
| Skycloud Egg | 1c4b6543-777e-4c3b-a9fb-5b7210d458d5#card | 1 | selected mana complement |
| Skycloud Expanse | 76f335d0-7f71-4b1a-b60d-73de954cbe2c#card | 1 | selected mana complement |
| Skyshroud Troopers | b5a40cd8-e1f9-4294-99ef-26fb455d4f95#card | 1 | selected mana complement |
| Slitherbore Pathway | 868e6e68-4367-4073-a864-235d5961ae56#face:1 | 1 | selected mana complement |
| Snapping Voidcraw | e495033b-f727-41e3-b7fc-7f5fa8abdd87#card | 1 | selected mana complement |
| Snow-Covered Wastes | 46a07b53-ff58-4bd6-80dd-ded2eb0e29a3#card | 1 | selected mana complement |
| Sol Ring | 6ad8011d-3471-4369-9d68-b264cc027487#card | 1 | selected mana complement |
| Spined Megalodon | 3a124fe5-f525-43e5-9746-501c68a98330#card | 1 | selected Scry Amount |
| Spotcycle Scouter | 0f1ada92-538b-40cb-9bf3-5c415f5325e6#card | 1 | selected Scry Amount |
| Steward of Valeron | ca5d0a56-69c1-4bdb-95f0-af0dec3604a5#card | 1 | selected mana complement |
| Stormcaller of Keranos | 16a66bbe-90f6-4c29-a69d-6ff254082956#card | 1 | selected Scry Amount |
| Strip Mine | d21a89eb-7c5b-459a-acc7-12b20b13bf79#card | 1 | selected mana complement |
| Su-Chi | 9a28d53e-c789-47de-8f3e-a251843ac596#card | 1 | selected mana complement |
| Sunastian Falconer | 3d6314b5-3ace-4793-9549-7b17db2a735d#card | 1 | selected mana complement |
| Sungrass Egg | 80a49a1b-a202-4c14-b093-dc76eb0f42c7#card | 1 | selected mana complement |
| Sungrass Prairie | 0a28dff0-2bd6-4105-b73a-b6c4735833fd#card | 1 | selected mana complement |
| Sunscorched Divide | 8d2b2675-19df-4f40-9e8e-196ec097b91c#card | 1 | selected mana complement |
| Swerve | c68627b1-025c-48c9-9646-98eb2d268e71#card | 4 | transitive Change; Single adjective |
| Sylvan Anthem | 5ab5eefd-47bf-4cdf-bc29-b517e4f6adc0#card | 16 | selected Scry Amount |
| Take a Glance | fa85f9e6-69ff-4669-b70a-440aa7b16c97#face:1 | 1 | selected Scry Amount |
| Taken by Nightmares | e5557c9f-e4c2-41af-9fcc-371409e1dec6#card | 2 | selected Scry Amount |
| Tangled Florahedron | 53542c79-a62a-4d6a-97db-5296e9c68302#face:0 | 1 | selected mana complement |
| Tel-Jilad Justice | 6c853b95-3d98-44fd-b05a-02d05d5eb0b8#card | 1 | selected Scry Amount |
| Thaumaturge's Familiar | 74a1a484-7c35-466a-b097-9aaf6fc2073b#card | 1 | selected Scry Amount |
| Thran Dynamo | a699c663-8131-4045-9265-a83e86609374#card | 1 | selected mana complement |
| Tidechannel Pathway | 59d22de5-e310-44d7-89cf-ef3529e40cef#face:1 | 1 | selected mana complement |
| Tidepool Turtle | ac2668a3-7a26-47b1-9ad4-56e23643de80#card | 1 | selected Scry Amount |
| Timbercrown Pathway | 727ca426-f4cc-4218-8ae5-8c427af2e816#face:1 | 1 | selected mana complement |
| Tocasia's Dig Site | a91f93fd-e428-4a36-b1b3-604b47a34287#card | 1 | selected mana complement |
| Tolarian Academy | dba4fd31-8931-42dd-bd86-45479c2abf74#card | 4 | selected mana complement |
| Tomb of the Spirit Dragon | 22f6391e-2634-440f-af1b-9581d1bff818#card | 16 | selected mana complement |
| Tooth of Ramos | fa4c57b3-6eaa-4938-b41a-0dad3e774d49#card | 1 | selected mana complement |
| Treasonous Ogre | 826d4279-1576-4898-a1c2-26fd547fb0d0#card | 1 | selected mana complement |
| Treasure Cove | 0b55eac6-a745-4bf4-8926-5ce83bc38d7d#face:1 | 1 | selected mana complement |
| Tree of Tales | 8b4aa971-b919-4750-8388-33d4f42c9280#card | 1 | selected mana complement |
| Trenchpost | 42f1ccb8-eda0-4828-ac07-82d4e950d7e1#card | 8 | selected mana complement |
| Treva's Attendant | ff7d927f-0a01-4243-ad00-c65686ee86bb#card | 1 | selected mana complement |
| Turn to Dust | 56828166-eaa3-4711-91b6-401a3e3b733f#card | 3 | selected mana complement |
| Ulvenwald Abomination | 0272ca81-e727-4f4b-b06e-072d70bb5558#face:1 | 1 | selected mana complement |
| Ulvenwald Captive | 0272ca81-e727-4f4b-b06e-072d70bb5558#face:0 | 1 | selected mana complement |
| Unstable Obelisk | 060ae1bb-956a-4264-b405-a33151f31493#card | 1 | selected mana complement |
| Ur-Golem's Eye | fb34fc00-e60d-41fc-9393-ca4248ec0a1c#card | 1 | selected mana complement |
| Valleymaker | 57098671-016e-4adb-ba7d-ab2dd2bc34e3#card | 2 | selected mana complement |
| Vault of Whispers | 09496421-74e4-466a-9546-56f2a0c8eef4#card | 1 | selected mana complement |
| Vessel of Volatility | c5a4d485-ab9c-48bb-866f-448b1f7707cc#card | 1 | selected mana complement |
| Vine Trellis | 57de8fe7-3d1b-41cd-8354-38aff3d2d052#card | 1 | selected mana complement |
| Viridescent Bog | 6bd6d259-1af7-4dff-a79c-48a616d2a36e#card | 1 | selected mana complement |
| Viscera Seer | f82a4e85-526d-4456-b700-7760043a31be#card | 1 | selected Scry Amount |
| Wakandan Drone Flock | 282d6e61-3240-4f1e-9c2d-8ddb59b03af4#card | 1 | selected Scry Amount |
| Wall of Runes | 35dfe1c7-48ab-436f-9a6e-44835ea01cf9#card | 1 | selected Scry Amount |
| Warden of Geometries | 448da389-81db-4931-9cf7-3fde0ce9392f#card | 1 | selected mana complement |
| Warteye Witch | 594a57f6-7827-41c7-aa37-00b498720b16#card | 2 | selected Scry Amount |
| Wastes | 05d24b0c-904a-46b6-b42a-96a4d91a0dd4#card | 1 | selected mana complement |
| Watchful Automaton | 6f5e6799-cb0f-4403-9c2a-69abeac08b79#card | 1 | selected Scry Amount |
| Weaver of Currents | 9aabb520-7e97-47b2-975c-11f457d0b870#card | 1 | selected mana complement |
| Willow-Wind | ee3fd92c-48e0-4651-aeda-7eb2c75acbb0#card | 1 | selected Scry Amount |
| Wirewood Elf | 4aab6794-506f-413c-91e0-d886e0019dce#card | 1 | selected mana complement |
| Wirewood Lodge | 1275653f-de4e-4fe9-aad8-88555fa11680#card | 1 | selected mana complement |
| Witching Well | baadfc17-42fd-4771-aabe-027ef7b3bf69#card | 1 | selected Scry Amount |
| Witness of Tomorrows | b46adfcd-fd7a-40c5-85f4-1cfee43a4c48#card | 1 | selected Scry Amount |
| Woodland Mystic | 6996c354-d3fa-40c3-8534-27d2f5b74756#card | 1 | selected mana complement |
| Yavimaya Hollow | 53d6113d-acdb-4754-9641-f7991a96c7b9#card | 1 | selected mana complement |
| Zhalfirin Void | 13aab4fc-4c89-45e6-8275-b074b00d0ee9#card | 1 | selected mana complement; selected Scry Amount |
| Zhur-Taa Druid | 5979310e-38c8-489f-ab9e-2723af98a3a5#card | 12 | selected mana complement |
| Zoetic Cavern | 3763de30-28e1-4689-a71c-07d2fea3a466#card | 1 | selected mana complement |
