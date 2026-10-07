---
needs: []
---
# Read *become* with an adjectival passive Complement: whenever this creature becomes blocked

## Why

*Becomes blocked* never reads. On change `xxknlzypsnwy` (32,828 supported
faces, 17,322 covered, 15,506 unread; recon of 2026-10-07), it touches **178**
unread faces and is the sole cause on **109** (recon bucket "becomes blocked /
blocks or becomes": 180 / 109). Counts are unread faces *touched* (at least one
localised failing unit matches) / *sole* (every failing unit matches and no
other recon STRONG bucket does). They are surface counts, not gain forecasts.

Probes (admitted roots): "Whenever this creature becomes blocked, draw a card."
0; "… becomes blocked by a creature, …" 0; "Whenever this creature is blocked,
draw a card." 1; "Whenever this creature becomes tapped, draw a card." 1;
"Whenever this creature blocks, draw a card." 1.

## Goal

A past participle reads as the predicative Complement of *become* (an
adjectival passive), with an optional *by* phrase where attested (*becomes
blocked by a creature*), in finite clauses and coordinated with *blocks*
(*blocks or becomes blocked*). Whatever route already reads *becomes tapped* is
either this route or is retired in its favour, recorded (Method 8): one
analysis per string.

## Analysis

A past participle that is the Complement of *become* is adjectival: *It became
magnetised* is an adjectival passive, with the change of state supplied by
*become* (CGEL, Ch. 16, §10.1.3, p. 1438, [39ii]). Adjectival passives also
occur with other verbs taking predicative complements (p. 1437). *By* phrase
complements are found in adjectival as well as verbal passives but are much
more restricted in the adjectival construction (p. 1439, [41]); whether
*becomes blocked by a creature* is admitted under that restriction is the
landing's decision, recorded.

## Witnesses

Each witness's failing units all match this construction and no other recon
STRONG bucket (`xxknlzypsnwy`):

- Flint Golem: "Whenever this creature becomes blocked, defending player mills
  three cards."
- Vedalken Ghoul: "Whenever this creature becomes blocked, defending player
  loses 4 life."
- Trained Cheetah: "Whenever this creature becomes blocked, it gets +1/+1 until
  end of turn."
- Somberwald Alpha: "Whenever a creature you control becomes blocked, it gets
  +1/+1 until end of turn."
- Talruum Champion: "Whenever this creature blocks or becomes blocked by a
  creature, that creature loses first strike until end of turn."

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. Before: `cargo xtask english-v3 --all --workers 12 --samples-per-face 0
   --output target/english-v3/english-v3-become-adjectival-passive-before.json`
   on the claim parent, stamped with its change id and covered count; after:
   the same command to
   `target/english-v3/english-v3-become-adjectival-passive-after.json` on the
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
   Reading. A *becomes blocked* Reading with *blocked* as a finite or
   Object-taking verb, or two analyses of *becomes tapped*, is a defect. A
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
12. Gate: `cargo xtask gate --changed --from <claim> --run`.

## Out of scope

- *for each creature blocking it beyond the first* (Jungle Wurm and twins).
- *becomes the target of*, *becomes a copy of*, *becomes untapped* beyond what
  the same route gives; record any that start to read.
- *is blocked* (already reads) and the verbal *be* passive.

## Landing record

### Analysis and declared routes

*Become* retains its existing Predicative frame and AdjectivalComplement.
The existing participial-adjective class now includes Block, producing an
invariant `core-verb:Block/adjective` owner with no verbal inflection features.
Its empty Predicate frame selects IntransitiveAdjective; its marked Adjectival
frame selects ComplementedAdjective. The unframed Adjective construction does
not consume this framed owner. *Tapped* and *untapped* retain their existing
unframed Adjective route: all three simple finite predicates have exactly one
Reading, checked against independently authored complete values.
No shipped route was superseded; no lexical or grammatical declaration was
retired or left unreachable. Both new Block/adjective frames have attested
consumers. The marked frame reuses InternalisedComplementMarker; its By
assignment is declared data, not a word-naming guard.

**By decision (this landing, delegated by this ticket's Analysis):** admit
*blocked by a creature* as a complemented adjective denoting the resulting
blocked state, with the creature identified in its Complement. CGEL Ch. 16
§10.1.3 pp. 1436, 1438–1439 supplies the adjective/verbal distinction, the
change-of-state example *became magnetised*, and the restriction of adjectival
By Complements to participles related to verbs with a stative sense. CGEL does
not analyse Magic's *blocked*: applying that distinction to the attested
Oracle wording is this project's landing decision, not a CGEL claim.
The existing matrix By Adjunct also remains grammatical. A complemented
AdjectivePhrase and a simple adjective with a matrix Adjunct differ in
constituency; they are not two labels for one constituency. The standalone
AdjectivePhrase *blocked by a creature* has exactly one Reading. The finite
predicate has two (adjectival Complement versus matrix Adjunct); coordinated
*blocks or becomes blocked by a creature* has three (the preceding two plus
an Adjunct over the coordination). Existing duration attachments account for
the remaining witness multiplicity. None is destructively selected away.

### Prove and disclose

Baseline is the production tree at claim `wssttryk`, parent `wqmloqwu`
(`kata: complete english-v3-where-variable-clause`). The baseline census
records working change `rmrtuzwlllywonozvtzuwmmlrorzkznv` because it ran after
claiming; the declaration snapshot and inventory hash establish that its
production inputs are the unchanged claim-parent tree. Both phases below
use that same feature change id; covered count and inventory identify the
measured tree. The input hash is unchanged:
`49dc966bda6ef588fc68e8d6972de25df1b660ed2ea654904584698263007ebb`.

| Phase / measured change | Covered | Unique | Multiple | No Reading | Readings | Inventory SHA-256 |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| Claim-parent production inputs / rmrtuzwl | 20,858 | 7,477 | 13,381 | 11,970 | 651,577 | 8cb11f9127d3bcfce02b384d0e93e354643004be15197135f07ae25421d675f8 |
| Finished feature / rmrtuzwl | 21,070 | 7,464 | 13,606 | 11,758 | 685,662 | ccca898a5f6374fc8f278745a45b19348bfd712649b99fdcc1debd205a2bbf12 |

All 32,828 face identities and source hashes match. Newly covered: **212**.
Lost previously covered faces: **0**; decreased Reading counts on previously
covered faces: **0**; increased counts on previously covered faces: **389**.
There are no removed Readings to classify or route. Selection is the current
all-Readings census (English lexical-analysis decision); specificity-resolved
selection and unresolved-tie counters are superseded and do not apply.
The additional *be blocked* adjective analysis coexists with the original
verbal passive, as a category/constituency distinction (CGEL p. 1436).
The Sneaky Homunculus regression retains both complete values.

Every counted Reading passed admission, lexical ownership, byte-exact
realization, and construction/node and word/leaf traversal validation.
Both phases have zero issues, incomplete/limited/failed enumerations,
duplicate derivations, cycles, and internal failures. Independent authored
simple and complemented adjective values pass admission, realization,
reparse membership, and exact node/word traversal comparisons. Malformed
frame/marker/head choices are rejected; object-taking and finite/gerund
*blocked* substitutes under *become* do not read. Corpus validation alone is
not independent linguistic validation; the authored-value tests and gain
sample audit provide the separate evidence.

Forbidden word-naming licensing checkers added: **0**. Lexical-source and
GrammarEnvironment loading errors: **0**. The active v3 census has no legacy
coverage lock or permitted-licensing-checker total; no such counter is
invented here. Reported covered counts come from the retained census JSON.

The complete gain selector was checked with two sampled trees per face,
workers 6: counts agree with the full census and validation issues are zero.
Each gain's selected tree includes the following licensed analysis:

- **B0**: IntransitiveAdjective, invariant Block/adjective, empty Predicate
  frame 0, inside a predicative Complement or an attested adjective modifier.
- **B1**: ComplementedAdjective, invariant Block/adjective, Adjectival frame
  1 selecting By plus AccusativePhrase.
- **L1**: SelectedPredicate, Lose frame 1 selecting GrantedAbility.

The 212 gains comprise B0 83, B1 60, L1 67, and B1+L1 2. The latter are
Talruum Champion and Mammoth Harness. Per-face identity, count, and selected
analysis follow; full sampled tree fingerprints and sources are in ignored
`target/english-v3/english-v3-become-adjectival-passive/new-face-analyses.json`
and `new-faces-samples.json`.

| Newly covered face | Identity | Readings | Selected analysis |
| --- | --- | ---: | --- |
| Fear of Falling | `00ba96e0-42a7-432a-a500-5e607d75a358#card` | 4 | L1 |
| Likeness of the Seeker | `02c60b8b-f665-4279-a00f-ae03991df4a6#face:1` | 1 | B0 |
| Engulfing Slagwurm | `0340cfcb-fc96-40d2-8175-31a0093615bb#card` | 6 | B1 |
| Sky Tether | `06548af1-4fda-435c-ac32-bbf7afafb34e#card` | 1 | L1 |
| Flint Golem | `0993c5f1-1e8f-4871-9904-6c5a00405f36#card` | 1 | B0 |
| Vintara Elephant | `0a73b4da-9b5c-480e-b58b-6a82a52ea71e#card` | 2 | L1 |
| Laccolith Rig | `0b583980-1508-4247-8a28-7127241d8ec0#card` | 6 | B0 |
| Laccolith Titan | `0d9ac5e0-2b4e-4e7b-937e-9e9bfce7e96b#card` | 3 | B0 |
| Chambered Nautilus | `0e1267d4-0155-4503-829a-03264c8d14a0#card` | 1 | B0 |
| Deathgazer | `0f772f35-6d98-4ac8-88e5-02539737078b#card` | 9 | B1 |
| Somberwald Alpha | `11b12954-dd1b-4c50-8433-15d8c1a15e29#card` | 6 | B0 |
| Duskworker | `14d85a83-323b-4e42-88bf-71167a8174e3#card` | 2 | B0 |
| Cephalid Snitch | `15093579-8258-41d2-9738-aaa7e3824c40#card` | 2 | L1 |
| Assembled Alphas | `151a681d-a350-4d1d-86cd-b80d58602810#card` | 3 | B1 |
| Dance of the Dead | `15d5b198-edbe-40f3-87e2-d72f7f0c58ab#card` | 4,800 | L1 |
| Leery Fogbeast | `174dfb7f-fe1e-4089-80c4-1788a9307263#card` | 10 | B0 |
| Ogre Leadfoot | `17e00d6e-c57d-48ea-a1a1-0c1097083851#card` | 2 | B1 |
| Canopy Claws | `195c6453-e5cc-4b3b-8518-eaccf29bc4a6#card` | 2 | L1 |
| Wishful Merfolk | `19ff1424-6505-4702-86ad-2d7518b584f6#card` | 4 | L1 |
| Ferocity | `1b1a0f35-3126-4b88-90aa-c56e67850a9d#card` | 1 | B0 |
| Wooden Stake | `1b26dbb7-0ae5-4279-87b6-1b722c30a386#card` | 3 | B1 |
| Sentry Oak | `1b2e0a60-844e-4db7-91fd-323497ab3cbf#card` | 20 | L1 |
| Guardian of the Ages | `1bf88f66-b673-4652-981b-3ea5d28f5c29#card` | 1 | L1 |
| Vertigo | `1dabfade-79e5-44e2-afa6-b20498c6bb8b#card` | 6 | L1 |
| Raging Gorilla | `1dd6df3f-eac4-4f30-b832-4a3f3a97632f#card` | 3 | B0 |
| Lim-Dûl's Cohort | `1e3b97d2-8fda-4510-9697-f36ca9ca2ab0#card` | 9 | B1 |
| Soul Sear | `1ef73b3b-ab4a-494b-b969-dcde995cee34#card` | 2 | L1 |
| Phyrexian Reaper | `245c3681-fc47-47fe-b6e5-34c96315c135#card` | 2 | B1 |
| Thundercloud Elemental | `24b44a35-496a-48e9-91ee-98b327d867e4#card` | 6 | L1 |
| Port Inspector | `25450b06-3065-4b4f-975e-586d045e5815#card` | 1 | B0 |
| Archetype of Aggression | `263408e6-b315-4af5-8cb8-3fd1aa88e48c#card` | 1 | L1 |
| Ghost Hounds | `26957a44-6155-4688-a850-a3b49b011a9f#card` | 9 | B1 |
| Inferno Elemental | `287cb63c-d52d-4871-83d1-792a41ae7adc#card` | 3 | B1 |
| Deeproot Warrior | `2af1930c-a5f2-4e7e-910f-e84458b051ac#card` | 3 | B0 |
| Abomination | `2c57c4e9-0a46-45d6-92db-9203fb722b60#card` | 9 | B1 |
| Infernal Medusa | `2cf5ce1f-d5f6-44cd-96e5-87d990d7e770#card` | 18 | B1 |
| Skewer Slinger | `2e8433a2-ad71-4d30-84fe-12b709197824#card` | 3 | B1 |
| Arcane Lighthouse | `30ac68e6-160a-41f9-9f0f-0e0eef383150#card` | 1 | L1 |
| Somberwald Vigilante | `31c77036-8627-476a-996b-2a6b005c9e12#card` | 2 | B1 |
| Groffskithur | `32a8ca5f-0170-4f5d-97cd-63e2b74fd653#card` | 3 | B0 |
| Goblin Cadets | `32dc847a-d8e9-4bc5-aa73-dfe69ff330a2#card` | 1 | B0 |
| Kolaghan Aspirant | `334178f6-cf4b-4ea6-a24f-c7375d1b38de#card` | 2 | B1 |
| Brushwagg | `33a981ac-878d-4257-936f-55b4e6543c12#card` | 3 | B0 |
| Gravity Well | `33f757ad-c636-4f17-ba9e-40856f83efd0#card` | 3 | L1 |
| Goblin Skycutter | `34ee9883-ed7f-4c63-987f-96a5474c195d#card` | 6 | L1 |
| Acolyte of the Inferno | `35bf67e0-8141-4664-b762-2c613353e1eb#card` | 2 | B1 |
| Smite | `367379f2-e1e4-48e7-b2c0-1003de55adbe#card` | 1 | B0 |
| Smite the Deathless | `37994591-3494-4314-a7da-49c112b0866f#card` | 12 | L1 |
| Witherscale Wurm | `37b7f044-f57b-4d13-82be-46fd01f1dba6#card` | 18 | B1 |
| Alley Grifters | `399af8ea-7a6f-4f43-bd39-dc29a6e75e06#card` | 1 | B0 |
| Deepwood Tantiv | `3b31a9a7-6c97-4e82-bf72-de83034f81a9#card` | 1 | B0 |
| Serra Inquisitors | `3c6a32e8-0c2a-45cb-8b79-c7f022fe5129#card` | 9 | B1 |
| The Fire Nation Drill | `3d4b9d71-4275-4c18-a4dc-9aad234eb92c#card` | 8 | L1 |
| Cunning Evasion | `3e56606d-562e-4c77-9769-213997fea28e#card` | 2 | B0 |
| Battle-Scarred Goblin | `3e765710-e567-4af0-b5f5-7c182649b757#card` | 2 | B0 |
| Snorting Gahr | `3e7f67d6-db12-4034-b8e3-9c9d401c8f57#card` | 3 | B0 |
| Razorclaw Bear | `3ee07899-e6d6-4dcf-8596-4d2420690d7c#card` | 3 | B0 |
| Warpath | `413520c7-5227-406f-8861-e844c153e5b0#card` | 1 | B0 |
| Corrosive Ooze | `416cca4a-b1b2-49d5-9cac-ec347056bdb0#card` | 12 | B1 |
| Seifer, Balamb Rival | `417b9d9c-0a67-4554-a275-a6ae15a04bfb#card` | 24 | B1 |
| Giant Shark | `44a10a63-be9c-4f1d-aad6-b5337112bda5#card` | 168 | B1 |
| Zerapa Minotaur | `4520c630-3650-462f-ab01-215eb2bdbbf5#card` | 2 | L1 |
| Dromosaur | `4576cc5b-7b3a-41a7-9b84-acadf2d4adf5#card` | 3 | B0 |
| Sawtooth Ogre | `473c9f27-2091-4d87-b279-64b3662c9af8#card` | 9 | B1 |
| Torpid Moloch | `48f78611-13a7-4b5b-a52a-317ecf658ea3#card` | 2 | L1 |
| Gift of the Woods | `491c6f69-9b75-4d5d-b41f-a14b2fbfdd07#card` | 5 | B0 |
| Sylvan Basilisk | `4f348e7a-fe13-4621-bd88-5d2fb5b597f2#card` | 2 | B1 |
| Baneblade Scoundrel | `4f5da665-e880-4325-a9e2-6fc4ca1d807a#face:0` | 3 | B0 |
| Baneclaw Marauder | `4f5da665-e880-4325-a9e2-6fc4ca1d807a#face:1` | 3 | B0 |
| Tormentor's Helm | `4f5f0988-0b1f-4f4b-bcee-cc76fa484d9e#card` | 1 | B0 |
| Tolarian Entrancer | `50c8c7e2-1b45-44b2-a50b-3b514d4c4c42#card` | 6 | B1 |
| Retaliation | `51571fa8-fd52-4439-8331-4eb4f651e764#card` | 6 | B1 |
| Ignoble Soldier | `52a61983-53e2-440f-af03-cebe57b102f2#card` | 28 | B0 |
| Aisling Leprechaun | `5456f00c-0bef-4c14-902f-f5c14475f284#card` | 3 | B1 |
| Glittering Lion | `549e6de7-56e9-4f5c-8c88-30e446bc53bb#card` | 4 | L1 |
| Burning Palm Efreet | `55cbc0e8-eeec-433a-b70c-3501238b658b#card` | 9 | L1 |
| Mirror Shield | `57f2858e-5793-41fc-9bbb-1fdd3b99fe79#card` | 3 | B1 |
| Elder Land Wurm | `5948381a-419d-4b8b-8ea2-ea623cb0d606#card` | 1 | L1 |
| Silkenfist Order | `5cca974b-1a6c-485f-adc0-81d2b7dca286#card` | 1 | B0 |
| Crash Landing | `5d36e5ee-1e23-48fe-ad13-081cc153342b#card` | 46 | L1 |
| Short Circuit | `5ea0e3a5-9c38-4da4-98a6-41c14be512d9#card` | 1 | L1 |
| Hezrou | `65d257c2-b3be-4244-a74c-bc4b5d7cedc3#face:0` | 3 | B0 |
| Ib Halfheart, Goblin Tactician | `6742c4b9-8fa6-439f-844d-04a09fa20c45#card` | 2 | B0 |
| Treefolk Mystic | `68a6fd00-9cb0-4518-97aa-b7f475da58c1#card` | 3 | B1 |
| Slinking Giant | `6d86dcaa-2295-4f4d-ada7-216f96138d81#card` | 3 | B0 |
| Slith Strider | `6e52e48f-b7a0-4195-8fff-8baed7b7d3fc#card` | 3 | B0 |
| Rock Basilisk | `6e7bb687-8243-4377-80b5-67ce6b65f9b7#card` | 9 | B1 |
| Pygmy Troll | `6f18f41e-df8d-4b59-87cc-ea167823fbda#card` | 6 | B1 |
| Vedalken Ghoul | `708b13ef-f297-4176-9d09-e633c4d600da#card` | 1 | B0 |
| Tightening Coils | `71af7e54-e22d-4354-a35c-80e4e5f4031c#card` | 1 | L1 |
| Infiltration Lens | `72dfd729-ebba-4bb1-83eb-54c2051fea3b#card` | 2 | B1 |
| Evil Eye of Urborg | `752b5a97-e2f7-43c0-b8c8-2c1efbbfbd2a#card` | 4 | B1 |
| Archetype of Endurance | `79254223-3fa6-4b7c-8163-1e48bf5cb708#card` | 1 | L1 |
| Archetype of Courage | `79b48704-480d-4905-b87a-40b127894670#card` | 1 | L1 |
| Fight to the Death | `7f885ffc-7245-4f00-a903-6e088007c635#card` | 1 | B0 |
| Tidewater Minion | `80b4a9f7-008d-4979-a3df-7637fae68167#card` | 2 | L1 |
| Ichorclaw Myr | `8462a2fa-944b-4fff-a161-10345690b56f#card` | 3 | B0 |
| Benalish Missionary | `84fdcfd3-2b22-4570-af34-7e3f55f97466#card` | 28 | B0 |
| Barrow-Blade | `867c5592-50d5-420c-ac71-50014bc390eb#card` | 12 | B1 |
| Saprazzan Heir | `8685ebfd-d04d-47f5-a919-e118db1e2276#card` | 1 | B0 |
| Dwarven Soldier | `88f9075c-ea96-4f8d-b0d1-e5e244e824c1#card` | 9 | B1 |
| Glittering Lynx | `890ffe31-642f-46e3-9f09-c744351653b5#card` | 4 | L1 |
| Gargoyle Sentinel | `893f319f-6e3d-4230-bb65-4f9179b18fad#card` | 1 | L1 |
| Burn from Within | `8a1ccbdb-3d89-42fb-a731-6db4241acf24#card` | 36 | L1 |
| Shoal Serpent | `8a1d14c2-ca31-4711-b385-f4af0c1c0b04#card` | 6 | L1 |
| Shadowspear | `8b27326f-e7b8-4a4d-b589-df459246d19a#card` | 2 | L1 |
| Escaped Null | `8bc17c50-f4c9-46dd-9294-3036ed46de91#card` | 3 | B0 |
| Exterminatus | `8d90ffb5-63da-48d5-a5a2-7b61062556f6#card` | 4 | L1 |
| Gravity Sphere | `8ddf93fe-980b-4dc4-b56f-6a2ee50100a6#card` | 1 | L1 |
| Venomous Dragonfly | `8e3e5264-d472-41ce-ad65-a53f8b24fc40#card` | 9 | B1 |
| Colossus Hammer | `8ec03b88-8d3a-4a32-8b7c-7da59b0c03d0#card` | 1 | L1 |
| Grozoth | `8ecbc87e-a63e-4afc-b5eb-2fc3dcd685f5#card` | 82 | L1 |
| Kinscaer Harpoonist | `8ee98d6a-6a7e-411d-818c-816d84cfb7cd#card` | 5 | L1 |
| Spectacular Pileup | `8f7479e6-b2bd-46b1-96ad-34deb43565c1#card` | 12 | L1 |
| Ageless Sentinels | `9023c530-f329-4301-8b24-4cf676194265#card` | 2 | L1 |
| Emerald Charm | `904b3db5-70bc-4d49-9887-f504aff8666f#card` | 2 | L1 |
| Basalt Golem | `9155f78e-0ccc-4e08-ae79-ef5b2db1183f#card` | 486 | B1 |
| Xathrid Slyblade | `931c0cd3-06fb-42da-86f2-476d08a238f9#card` | 1 | L1 |
| Talruum Champion | `9352e742-9a22-4463-b41f-3117ff1540ed#card` | 9 | B1 + L1 |
| Flailing Drake | `93b6a170-a816-4556-aa27-a729231715e7#card` | 9 | B1 |
| Ribbon Snake | `98b53af9-af1b-4d5d-ae5a-bd444c1339d2#card` | 2 | L1 |
| Amphibious Kavu | `9c9b1871-e776-4373-ba89-e009131cb6f7#card` | 9 | B1 |
| Rabid Elephant | `9daed342-e773-43ec-a8e2-98126b505e8a#card` | 6 | B0 |
| Whiteout | `9e8de3e1-1159-460f-99b7-1043e5814597#card` | 6 | L1 |
| Goblin Swine-Rider | `9ebe32fc-3b6e-4cb9-9af0-1a6c3c6ad885#card` | 1 | B0 |
| Chub Toad | `a0daace2-92fc-4c7d-b497-94b4ba775559#card` | 3 | B0 |
| Labyrinth Raptor | `a29b2e93-d872-4be8-8b15-57f57282ed45#card` | 16 | B0 |
| Rebel Salvo | `a35b7fd3-fdef-4414-b720-978f7ce70bff#card` | 4 | L1 |
| Arrogant Bloodlord | `a35e1882-b878-49cf-a06c-be12a06d4977#card` | 24 | B1 |
| Archetype of Imagination | `a5458de0-0f61-49a3-a013-d90f92559809#card` | 1 | L1 |
| Goblin Elite Infantry | `a6a8861c-b6fe-45a6-86e4-9160c36362ff#card` | 3 | B0 |
| Berserk Murlodont | `a762e8cb-9d33-41fc-b2e8-0c5ad4d7268e#card` | 6 | B0 |
| Reality Anchor | `a7b4a519-d85f-40ae-bb5f-08bc1d301418#card` | 2 | L1 |
| Canopy Dragon | `a9e59ee4-02a6-498c-82f0-f4f2ff334d65#card` | 3 | L1 |
| Pretender's Claim | `ac285479-46c2-49f3-a7ff-31affe23c21f#card` | 1 | B0 |
| Quick Draw | `ae0d17f6-859c-4b41-8b66-b68abb837bbb#card` | 6 | L1 |
| Elven Warhounds | `ae997fee-2c1f-43c8-9dba-8b409d8e449e#card` | 2 | B1 |
| Cockatrice | `af354337-424c-4c7e-8ca5-6149261368d2#card` | 9 | B1 |
| Tattermunge Witch | `b1696989-7f00-41ec-82a4-fd10aed7a7f9#card` | 3 | B0 |
| Laccolith Whelp | `b16b10ec-a6a3-4bda-a561-9b7bb8eee194#card` | 3 | B0 |
| Norwood Warrior | `b3ae1481-9fa7-45dd-bbf5-de2d815c4227#card` | 3 | B0 |
| Chimeric Sphere | `b4aae26a-44a4-4ef0-9a1d-14a07a0865a3#card` | 7 | L1 |
| Infinite Authority | `b4faba1a-23db-4678-9ce6-a7816105f22a#card` | 96 | B1 |
| Plague Wight | `b7639d86-4c7c-423f-8b13-fde3086d3cbb#card` | 3 | B0 |
| Gang of Elk | `b99d6d51-3ead-4b81-8e0c-f458619ebb24#card` | 6 | B0 |
| Hour of Devastation | `b9ea0d4c-cd71-4817-8213-96899e1a14bb#card` | 2 | L1 |
| Tangle Asp | `b9ea2e9c-a83b-40dd-ad52-b16d65989ba3#card` | 9 | B1 |
| Tattered Ratter | `ba36f27a-08fc-46a5-bc55-2a71091cebcc#card` | 3 | B0 |
| Silkenfist Fighter | `bf5ceba2-9def-4ff4-9291-740ca90c1c4d#card` | 1 | B0 |
| Laccolith Warrior | `bf957e0b-3070-4a85-a63d-3dadd7364d53#card` | 3 | B0 |
| Slashing Tiger | `c07a86de-ec29-478c-b6cb-057eb9d3e126#card` | 3 | B0 |
| Animate Dead | `c0d8fef4-65f4-4769-982d-b397d2b7e977#card` | 144 | L1 |
| Corrupt Official | `c152b677-5e75-43da-be1d-876b7b83e2f4#card` | 3 | B0 |
| Downdraft | `c2508190-5aca-4511-adc8-2072cd228119#card` | 6 | L1 |
| Drelnoch | `c25465a9-a0f3-48be-8bdf-4aa861da7d3e#card` | 1 | B0 |
| Swooping Talon | `c292d5cc-b6f4-4bb5-bc74-ff67e0643293#card` | 2 | L1 |
| Rust Scarab | `c3bd1d38-e73f-4e30-b22a-dca0a0e4d701#card` | 2 | B0 |
| Close Quarters | `c3f841c4-6e12-46fc-802a-3025182e7ff4#card` | 1 | B0 |
| Lim-Dûl's Paladin | `c4161335-1150-4f0d-bd2b-9211efb3bea6#card` | 36 | B0 |
| Thicket Basilisk | `c4822813-cd81-465d-9fe8-3a4c2dcd31ef#card` | 9 | B1 |
| Sparring Golem | `c54879da-0f1d-4ad0-b082-95f0e4d35685#card` | 6 | B0 |
| Slate Street Ruffian | `c74d72ee-4795-41ca-b676-4d5a05bd12b7#card` | 1 | B0 |
| She-Hulk, Wallbreaker | `c8d74a25-78a9-46ad-ab24-97f6221fd8dd#card` | 10 | B0 |
| Elvish Berserker | `c9046281-dedb-4246-8e5d-7786fe0c41c7#card` | 6 | B0 |
| Radjan Spirit | `c9ffce6c-a113-4c9d-9148-5ace68f68793#card` | 2 | L1 |
| Saprazzan Raider | `ca011fb1-fdc6-4eec-9344-c247339292ab#card` | 2 | B0 |
| Deepwood Wolverine | `ca8060a7-30e7-49a4-a720-da5b995dd31f#card` | 3 | B0 |
| Jukai Trainee | `cb9fa34d-1d5f-461c-acad-bc573d41ceb2#card` | 3 | B0 |
| Ornery Goblin | `cbd4e160-9eb1-42de-aee8-6f1d1398f633#card` | 3 | B1 |
| Dwarven Berserker | `cc43507a-ca61-45ae-ac0a-4ddb851c27ef#card` | 4 | B0 |
| Grounded | `cca3557c-c911-494e-95e7-cebec3442d63#card` | 1 | L1 |
| Karplusan Wolverine | `ce690da4-6d22-4013-aef3-596d331813e4#card` | 1 | B0 |
| Hedron Blade | `cf7ca005-410d-40c8-aae4-89f41f81848b#card` | 6 | B1 |
| Grazilaxx, Illithid Scholar | `d22ff377-d282-4a28-9dce-96f25913dc96#card` | 6 | B0 |
| Cave Tiger | `d3bac8b8-0b2a-4c67-b087-7a0bf1359468#card` | 6 | B1 |
| Ashmouth Hound | `d40fd63b-f55d-49f6-b4e5-a8d00f22e8e1#card` | 3 | B1 |
| Grasping Giant | `d601a28d-a02f-45eb-9dab-a3cf50a87ca4#card` | 8 | B1 |
| Order of the Alabaster Host | `d849be9d-0447-4f0b-a382-3f4ceb98f9a5#face:1` | 6 | B1 |
| Bonds of Mortality | `d93ba2f2-bf9b-444b-adbb-3835662734e7#card` | 2 | L1 |
| Dread Specter | `da21c21c-88d8-4a93-b2b4-6d7bb28959c9#card` | 9 | B1 |
| Phyrexian Slayer | `da74cb57-f6de-4d05-a339-6412beb8df25#card` | 2 | B1 |
| Trained Cheetah | `dad3b459-725e-4bf5-bf7a-bd89866ea8c4#card` | 3 | B0 |
| Fire Juggler | `dd1b4235-f053-4c46-8993-ae8d7860a0f3#card` | 8 | B0 |
| Adarkar Windform | `dd66b8ab-14a9-455b-98f6-5356df72a522#card` | 2 | L1 |
| Bestial Fury | `dfc08963-132f-4ebf-9384-15d17d1098f7#card` | 24 | B0 |
| Dead-Iron Sledge | `e1ee79de-e4c3-48f9-87a7-18f72e2f6116#card` | 3 | B1 |
| Tel-Jilad Wolf | `e1fcab73-8a23-43ae-9d8b-9601ffa8663e#card` | 6 | B1 |
| Sacred Prey | `e24110e8-72cf-4dd0-bca0-665758daca88#card` | 1 | B0 |
| Goblin Javelineer | `e35c8206-98d0-44ab-8770-823a5ff7ab72#card` | 2 | B0 |
| Jaded Analyst | `e43400fe-3df5-4942-abc3-b9bba34fc7cd#card` | 8 | L1 |
| Simian Sling | `e49c4d9a-6413-440f-ba5b-396fea6c03d9#card` | 2 | B0 |
| Gloom Sower | `e5cc1492-c9cc-48f0-8c11-02ead883675c#card` | 4 | B1 |
| Gorgon Recluse | `e6c9a699-0468-42f8-aad5-b9d45a3c1d37#card` | 9 | B1 |
| Battering Ram | `e7b91fba-8d96-4040-95e8-f0023b65c497#card` | 18 | B1 |
| Thresher Beast | `e88915fd-7867-45e6-b4b9-ca0af1823c0d#card` | 1 | B0 |
| Earthbind | `e8e35b49-8cfb-4fb5-89aa-8050f15b11bf#card` | 3 | L1 |
| Archetype of Finality | `eb4e9ce9-1917-46b1-b01c-645f5920ef9c#card` | 1 | L1 |
| Rabid Wolverines | `ed076c39-9bfc-424a-8ce8-8572f7c0de73#card` | 6 | B1 |
| Manor Gargoyle | `edb6c7ea-b4e1-4cb7-b5a9-8c5644a2aad9#card` | 2 | L1 |
| Venom | `ee55ca31-73f3-4e9d-8373-0a44009a25bd#card` | 9 | B1 |
| Spined Sliver | `eed9c168-1c78-43c6-a814-9b80054d1373#card` | 6 | B0 |
| Mammoth Harness | `f2e106c6-7666-42cd-9a02-e51327fb80e8#card` | 9 | B1 + L1 |
| Barreling Attack | `f44eab45-8ff9-40eb-9919-28fe5203fc48#card` | 12 | B0 |
| General Marhault Elsdragon | `f49b31ce-910e-446a-b7a7-4274d71e961d#card` | 6 | B0 |
| Viashino Weaponsmith | `f4d17f41-efd6-4173-ab6e-28b6fbcee10d#card` | 6 | B1 |
| Quagmire Lamprey | `f5b374d8-bf22-44f7-a18a-18c096a84640#card` | 2 | B1 |
| Beastmaster's Magemark | `f6889f57-2965-4a82-8556-fd91c6227058#card` | 6 | B0 |
| Loyal Gyrfalcon | `f693cf76-0919-4b89-92de-e619be006abb#card` | 6 | L1 |
| Barbed Foliage | `f9275c99-0616-44b3-a19e-98c683e87ad3#card` | 3 | L1 |
| Laccolith Grunt | `f973daa7-241e-4eb7-9d98-9ea7557caa3a#card` | 3 | B0 |
| Wind Shear | `fa683059-b9de-4063-ae71-597ed4e2df30#card` | 12 | L1 |
| Leering Gargoyle | `fc0907eb-a3fc-40ef-950d-b05aee9d5da7#card` | 3 | L1 |

The following whole-face spot checks inspect the adjective's category,
invariant morphology, selected Complement, coordination, and inherited
attachments. All were unread in the baseline:

| Face | Finished Readings | Selected local analysis |
| --- | ---: | --- |
| Flint Golem | 1 | B0 under become |
| Vedalken Ghoul | 1 | B0 under become |
| Trained Cheetah | 3 | B0; inherited duration attachment |
| Somberwald Alpha | 6 | B0; trigger alone has 3 |
| Talruum Champion | 9 | B1 + L1; By and duration attachments |
| Engulfing Slagwurm | 6 | B1 inside coordinated trigger |
| Ogre Leadfoot | 2 | B1 with artifact-creature NP |
| Inferno Elemental | 3 | B1 inside coordinated trigger |
| Skewer Slinger | 3 | B1 inside coordinated trigger |
| Somberwald Vigilante | 2 | B1 in trigger |
| Kolaghan Aspirant | 2 | B1 in trigger |
| Cave Tiger | 6 | B1 in trigger |
| Tel-Jilad Wolf | 6 | B1 with artifact-creature NP |
| Rabid Wolverines | 6 | B1 in trigger |
| Viashino Weaponsmith | 6 | B1 in trigger |

Fallowsage and Mesmeric Orb remain at one whole-face Reading each. No newly
covered face contains *becomes untapped*, *becomes the target of*, or *becomes
a copy of*. Six other gains use the adjective outside the literal *becomes
blocked* string: Smite, Warpath, Hezrou, Fight to the Death, Benalish
Missionary, and Tattermunge Witch. Their selected category is adjectival;
Hezrou has plural *become blocked*. This is reuse of the class, not a new
verbal passive implementation.

### Deviations and additions

- Add Block to the existing participial-adjective class; add its empty
  Predicate and marked Adjectival frames. New frame alias:
  AdjectivalPrepositionComplement. New construction: ComplementedAdjective.
  New features: **none**; new tables: **none**; new policies: **none**.
- Add Lose's GrantedAbility frame, reusing its existing consumer. Even after
  the blocked fix, the pinned Talruum Champion sentence failed on *loses
  first strike*. Method 7 permits attested lexicon frame edits; this closes
  that independent failure without touching plugin bodies or adding another
  construction. It also explains the 67 L1-only and two mixed gains above.
- Apply frame additions after participial-adjective generation so that
  generated adjective owners can receive declared frames. Dictionary diff:
  one new owner (Block/adjective), two changed old owners (Lose's appended
  frame and By's derived FrameMarker assignment), zero removed owners; no
  other old owner changes.
- The same lexical category enables ordinary adjectival use under Be and
  adjective modifiers. Retain the original verbal *be blocked* Readings;
  re-spell one existing exact-value test to include the additional adjective
  value. No verbal-passive implementation changes.
- Close the glossary gap **Adjectival Passive** in the owning Oracle English
  CONTEXT, using the actual CGEL passages above. No new CR citations.
- No new tracked census fixtures, verifier code, xtask commands/flags,
  features/tables/policies, or plugin-body edits. All census/evidence files
  are ignored under this workspace's `target/english-v3/`.

Declaration economy, claim-parent production inputs (rmrtuzwl, covered
20,858) versus finished feature (rmrtuzwl, covered 21,070): nonblank lines
**3,300 → 3,310, net +10**; ordinary constructions **203 → 204**; schemas
**45 → 45**; combined declaration forms **248 → 249**; compiled productions
**694 → 695**. Feature/table/policy counts stay **94/73/61**; frame aliases
**30 → 31**. The declaration-line ceiling was already exceeded. This change
reuses the lexical class and marker feature and adds one complemented
constituent consumer; it does not introduce a parallel simple route.

Assurance: restored **0**, re-spelled **1**
(`authentic_mixed_coordination_retains_ordinary_and_passive_predicates`, same
Sneaky Homunculus source and original full value retained, additional
adjectival value asserted), added **6** (the new
`become_adjectival_passive` suite), removed **0**, newly ignored **0**.
No STOPs occurred; no ticket-versus-recorded-ruling contradiction was
resolved without a STOP. The ticket explicitly delegates the By decision.

### Inventories and performance advisory

Homograph surfaces **123 → 124**; the only added surface is *blocked*
(Block versus Block/adjective). Finished named owners follow. These exclude
Category Catalog atoms and count declared-case spellings with distinct
owners; unchanged surfaces retain their prior owners. Literal/vocabulary
surface-overlap inventory is **empty before and after**. Full snapshots
are `before-inventory.tsv` and `after-inventory.tsv` in the evidence directory.

| Surface | Finished owners (rmrtuzwl, covered 21,070) |
| --- | --- |
| 's | core-verb:BeContracted ; core-verb:HaveContracted ; vocab:Genitive/Default |
| Adamant | catalog:ability-words.txt/Adamant ; lexeme:ability_word/adamant |
| Addendum | catalog:ability-words.txt/Addendum ; lexeme:ability_word/addendum |
| Alliance | catalog:ability-words.txt/Alliance ; lexeme:ability_word/alliance |
| Battalion | catalog:ability-words.txt/Battalion ; lexeme:ability_word/battalion |
| Bloodrush | catalog:ability-words.txt/Bloodrush ; lexeme:ability_word/bloodrush |
| Celebration | catalog:ability-words.txt/Celebration ; lexeme:ability_word/celebration |
| Channel | catalog:ability-words.txt/Channel ; lexeme:ability_word/channel |
| Chroma | catalog:ability-words.txt/Chroma ; lexeme:ability_word/chroma |
| Cohort | catalog:ability-words.txt/Cohort ; lexeme:ability_word/cohort |
| Constellation | catalog:ability-words.txt/Constellation ; lexeme:ability_word/constellation |
| Converge | catalog:ability-words.txt/Converge ; lexeme:ability_word/converge |
| Council's dilemma | catalog:ability-words.txt/Council's dilemma ; lexeme:ability_word/councilsDilemma |
| Coven | catalog:ability-words.txt/Coven ; lexeme:ability_word/coven |
| Delirium | catalog:ability-words.txt/Delirium ; lexeme:ability_word/delirium |
| Descend 4 | catalog:ability-words.txt/Descend 4 ; lexeme:ability_word/descend4 |
| Descend 8 | catalog:ability-words.txt/Descend 8 ; lexeme:ability_word/descend8 |
| Disappear | catalog:ability-words.txt/Disappear ; lexeme:ability_word/disappear |
| Domain | catalog:ability-words.txt/Domain ; lexeme:ability_word/domain |
| Eerie | catalog:ability-words.txt/Eerie ; lexeme:ability_word/eerie |
| Eminence | catalog:ability-words.txt/Eminence ; lexeme:ability_word/eminence |
| Enrage | catalog:ability-words.txt/Enrage ; lexeme:ability_word/enrage |
| Fateful hour | catalog:ability-words.txt/Fateful hour ; lexeme:ability_word/fatefulHour |
| Fathomless descent | catalog:ability-words.txt/Fathomless descent ; lexeme:ability_word/fathomlessDescent |
| Ferocious | catalog:ability-words.txt/Ferocious ; lexeme:ability_word/ferocious |
| Flurry | catalog:ability-words.txt/Flurry ; lexeme:ability_word/flurry |
| Formidable | catalog:ability-words.txt/Formidable ; lexeme:ability_word/formidable |
| Grandeur | catalog:ability-words.txt/Grandeur ; lexeme:ability_word/grandeur |
| Hellbent | catalog:ability-words.txt/Hellbent ; lexeme:ability_word/hellbent |
| Heroic | catalog:ability-words.txt/Heroic ; lexeme:ability_word/heroic |
| I | vocab:ChapterNumeral/One ; vocab:SubjectPronoun/I |
| Imprint | catalog:ability-words.txt/Imprint ; lexeme:ability_word/imprint |
| Infusion | catalog:ability-words.txt/Infusion ; lexeme:ability_word/infusion |
| Inspired | catalog:ability-words.txt/Inspired ; lexeme:ability_word/inspired |
| Join forces | catalog:ability-words.txt/Join forces ; lexeme:ability_word/joinForces |
| Kinship | catalog:ability-words.txt/Kinship ; lexeme:ability_word/kinship |
| Landfall | catalog:ability-words.txt/Landfall ; lexeme:ability_word/landfall |
| Lieutenant | catalog:ability-words.txt/Lieutenant ; lexeme:ability_word/lieutenant |
| Magecraft | catalog:ability-words.txt/Magecraft ; lexeme:ability_word/magecraft |
| Metalcraft | catalog:ability-words.txt/Metalcraft ; lexeme:ability_word/metalcraft |
| Morbid | catalog:ability-words.txt/Morbid ; lexeme:ability_word/morbid |
| More Than Meets the Eye | catalog:keyword-abilities.txt/More Than Meets the Eye ; lexeme:keyword_ability/moreThanMeetsTheEye |
| Opus | catalog:ability-words.txt/Opus ; lexeme:ability_word/opus |
| Pack tactics | catalog:ability-words.txt/Pack tactics ; lexeme:ability_word/packTactics |
| Paradox | catalog:ability-words.txt/Paradox ; lexeme:ability_word/paradox |
| Parley | catalog:ability-words.txt/Parley ; lexeme:ability_word/parley |
| Radiance | catalog:ability-words.txt/Radiance ; lexeme:ability_word/radiance |
| Raid | catalog:ability-words.txt/Raid ; lexeme:ability_word/raid |
| Rally | catalog:ability-words.txt/Rally ; lexeme:ability_word/rally |
| Renew | catalog:ability-words.txt/Renew ; lexeme:ability_word/renew |
| Repartee | catalog:ability-words.txt/Repartee ; lexeme:ability_word/repartee |
| Revolt | catalog:ability-words.txt/Revolt ; lexeme:ability_word/revolt |
| Secret council | catalog:ability-words.txt/Secret council ; lexeme:ability_word/secretCouncil |
| Spell mastery | catalog:ability-words.txt/Spell mastery ; lexeme:ability_word/spellMastery |
| Strive | catalog:ability-words.txt/Strive ; lexeme:ability_word/strive |
| Survival | catalog:ability-words.txt/Survival ; lexeme:ability_word/survival |
| Sweep | catalog:ability-words.txt/Sweep ; lexeme:ability_word/sweep |
| Tempting offer | catalog:ability-words.txt/Tempting offer ; lexeme:ability_word/temptingOffer |
| Threshold | catalog:ability-words.txt/Threshold ; lexeme:ability_word/threshold |
| Undergrowth | catalog:ability-words.txt/Undergrowth ; lexeme:ability_word/undergrowth |
| Valiant | catalog:ability-words.txt/Valiant ; lexeme:ability_word/valiant |
| Vivid | catalog:ability-words.txt/Vivid ; lexeme:ability_word/vivid |
| Void | catalog:ability-words.txt/Void ; lexeme:ability_word/void |
| Will of the council | catalog:ability-words.txt/Will of the council ; lexeme:ability_word/willOfTheCouncil |
| X | vocab:FixedCostSymbol/Variable ; vocab:Variable/X |
| as | vocab:Adverb/EquativeAs ; vocab:Preposition/As |
| blocked | core-verb:Block ; core-verb:Block/adjective |
| bottom | lexeme:CommonNoun/Bottom ; vocab:Adjective/Bottom |
| control | core-verb:Control ; lexeme:CommonNoun/Control |
| copies | core-verb:Copy ; lexeme:CommonNoun/Copy |
| copy | core-verb:Copy ; lexeme:CommonNoun/Copy |
| cost | core-verb:Cost ; lexeme:CommonNoun/Cost |
| costs | core-verb:Cost ; lexeme:CommonNoun/Cost |
| counter | lexeme:CommonNoun/Counter ; lexeme:keyword_action/counter |
| counters | lexeme:CommonNoun/Counter ; lexeme:keyword_action/counter |
| cycling | core-verb:Cycle ; lexeme:keyword_ability/cycling |
| deathtouch | lexeme:counter_kind/deathtouchCounter ; lexeme:keyword_ability/deathtouch |
| decayed | lexeme:counter_kind/decayedCounter ; lexeme:keyword_ability/decayed |
| die | core-verb:Die ; lexeme:CommonNoun/Die |
| double strike | lexeme:counter_kind/doubleStrikeCounter ; lexeme:keyword_ability/doubleStrike |
| draw | core-verb:Draw ; lexeme:CommonNoun/Draw |
| draws | core-verb:Draw ; lexeme:CommonNoun/Draw |
| exalted | lexeme:counter_kind/exaltedCounter ; lexeme:keyword_ability/exalted |
| exile | lexeme:CommonNoun/Exile ; lexeme:keyword_action/exile |
| exiles | lexeme:CommonNoun/Exile ; lexeme:keyword_action/exile |
| first strike | lexeme:counter_kind/firstStrikeCounter ; lexeme:keyword_ability/firstStrike |
| flying | lexeme:counter_kind/flyingCounter ; lexeme:keyword_ability/flying |
| goaded | lexeme:designation/goaded ; lexeme:keyword_action/goad |
| harnessed | lexeme:designation/harnessed ; lexeme:keyword_action/harness |
| haste | lexeme:counter_kind/hasteCounter ; lexeme:keyword_ability/haste |
| her | vocab:ObjectPronoun/Her ; vocab:PossessiveDeterminerPronoun/Her |
| hexproof | lexeme:counter_kind/hexproofCounter ; lexeme:keyword_ability/hexproof |
| his | vocab:PossessiveAbsolutePronoun/His ; vocab:PossessiveDeterminerPronoun/His |
| if | vocab:Preposition/If ; vocab:Subordinator/If |
| indestructible | lexeme:counter_kind/indestructibleCounter ; lexeme:keyword_ability/indestructible |
| it | vocab:ObjectPronoun/It ; vocab:SubjectPronoun/It |
| less | vocab:Adjective/Less ; vocab:Determinative/Less |
| lifelink | lexeme:counter_kind/lifelinkCounter ; lexeme:keyword_ability/lifelink |
| menace | lexeme:counter_kind/menaceCounter ; lexeme:keyword_ability/menace |
| name | lexeme:CommonNoun/Name ; lexeme:Verb/Name |
| names | lexeme:CommonNoun/Name ; lexeme:Verb/Name |
| one | lexeme:CommonNoun/One ; vocab:CardinalDeterminative/One |
| reach | lexeme:counter_kind/reachCounter ; lexeme:keyword_ability/reach |
| shadow | lexeme:counter_kind/shadowCounter ; lexeme:keyword_ability/shadow |
| solved | lexeme:designation/solved ; lexeme:keyword_ability/solved |
| suspected | lexeme:designation/suspected ; lexeme:keyword_action/suspect |
| tapped | lexeme:keyword_action/tap ; lexeme:keyword_action/tap/adjective |
| target | lexeme:CommonNoun/Target ; lexeme:Verb/Target ; vocab:TargetingMarker/Target |
| targets | lexeme:CommonNoun/Target ; lexeme:Verb/Target |
| that | vocab:SingularDemonstrative/That ; vocab:Subordinator/That |
| then | vocab:Adverb/Then ; vocab:Coordinator/Then |
| time | lexeme:CommonNoun/Time ; lexeme:counter_kind/timeCounter |
| to | vocab:InfinitivalMarker/To ; vocab:Preposition/To |
| top | lexeme:CommonNoun/Top ; vocab:Adjective/Top |
| trample | lexeme:counter_kind/trampleCounter ; lexeme:keyword_ability/trample |
| turn | core-verb:Turn ; lexeme:CommonNoun/Turn |
| turns | core-verb:Turn ; lexeme:CommonNoun/Turn |
| untap | lexeme:keyword_action/untap ; vocab:AttributiveAdjective/Untap |
| untapped | lexeme:keyword_action/untap ; lexeme:keyword_action/untap/adjective |
| up | vocab:Adverb/Up ; vocab:Preposition/Up |
| vigilance | lexeme:counter_kind/vigilanceCounter ; lexeme:keyword_ability/vigilance |
| you | vocab:ObjectPronoun/You ; vocab:SubjectPronoun/You |
| ’s | core-verb:BeContracted ; core-verb:HaveContracted ; vocab:Genitive/Default |
| ∞ | catalog:keyword-abilities.txt/∞ ; lexeme:keyword_ability/infinity |

Both full runs used **6 workers**. Baseline (rmrtuzwl, covered 20,858): host
load 3.2646484375 / 6.03125 / 7.17626953125 (1/5/15 minutes); setup wall
**5,874,773,910 ns**; corpus wall **207,305,157,880 ns**; per-byte thread CPU
**383,410 ns/B**. A separate baseline command-wall timer was not recorded.
Finished feature (rmrtuzwl, covered 21,070): host load 41.37353515625 /
24.2529296875 / 16.927734375; setup wall **19,138,696,736 ns**; corpus wall
**229,926,172,526 ns**; measured command wall **251,384,937,350 ns**; per-byte
thread CPU **420,330 ns/B**. The built xtask binary ran the same full selector
and flags as Cargo, avoiding the concurrently running gate's Cargo lock.
Both corpus walls and the recorded finished command wall exceed the quiet-host
advisory ceiling **16,260,000,000 ns**; this shared-host run makes no
quiet-host performance claim.

Full census budget so far: **2 of 3**. Final feature census:
`target/english-v3/english-v3-become-adjectival-passive/after.json`.
Baseline: the same directory's `before.json`. Reading-count comparison:
`feature-diff.json`; all gains' sampled trees: `new-faces-samples.json`.
These ignored artifacts are copied to the coordinator's same target path
before retiring this workspace, preserving them for review.

### Checks and refresh

Pre-refresh `cargo xtask gate --changed --from wssttryk --run --clippy`
derived `cargo test -p deckmaste_lexical_source -p deckmaste_construction_v3
-p deckmaste_english_v3 -p xtask`: **847 passed, 0 failed, 1 inherited ignore**.
The inherited ignore is
`macros::templates::tests::macro_schema_census_count_matches_21`, with its
existing blocker "cross-checks the live corpus against the census; run on
demand". Clippy initially found two test-helper ownership style issues;
these were corrected without changing any asserted value. Recheck:
`cargo test -p deckmaste_english_v3 --test become_adjectival_passive
--test participial_selection`: **9 passed, 0 failed**. The complete closure's
`cargo clippy -p deckmaste_lexical_source -p deckmaste_construction_v3
-p deckmaste_english_v3 -p xtask --all-targets -- -D warnings`: **exit 0**.
`cargo fmt --all -- --check`: **exit 0**. Citation check: **16,112 checked,
0 stale**; noncompliant citation-looking strings: **0**. Piped jj diff citation
audit: **0 new CR sites**. Unconditional Kata refresh and its final census
follow before integration.
