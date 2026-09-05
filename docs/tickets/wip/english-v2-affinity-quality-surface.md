---
needs: []
---
**A keyword whose Quality parameter is spelled with a preposition has no
keyword-line shape, and in grant position it silently splits.** `Affinity` is
declared `params: [Quality]` with surface `affinity`, but the printed line is
`Affinity for artifacts` [CR#702.41a]: the quality arrives behind `for`.
`qualified_keyword_line_item` is `lex(keyword) quality` over `abstract sum
KeywordQuality { Reference: Nominal, Coordination: KeywordQualityCoordination }`,
which admits no prepositional quality, so the keyword line does not parse
(Frogmite, Myr Enforcer, Thoughtcast are parse failures).

The grant position is worse than a failure. Since
`english-v2-tail-keyword-ability-grant` (2026-09-05) `Spells you cast have
affinity for artifacts.` **is covered**, selecting the bare
`ReferencedQualityKeywordAbility` carrier for `affinity` and attaching `for
artifacts` as a clause-level prepositional predicate adjunct — the quality
modifies the having rather than naming the affinity. Two units select this
today (Sami, Wildcat Captain; Tezzeret, Master of the Bridge). The shape
predates that landing (the parent reached the same split through the
`params = Any` `lex KeywordAbility` codec) and it was not introduced by it, but
because it is covered rather than failing it will not resurface when the
keyword line is repaired.

`Protection` shows the shape that works: its declared surface is `protection
from`, so the preposition is inside the keyword surface and the quality is a
plain `Nominal`. Whether the fix is to give `Affinity` the same treatment
(surface `affinity for`), or to admit a prepositional arm on `KeywordQuality`,
is the design question; a per-keyword branch in the grammar is not an option.
Whichever is chosen, the two covered grant-position units above must change
analysis, so the landing record has to name them.

Standard constraints apply.

## Landing record

Start wall clock: 2026-09-05T05:30:50-07:00. Measured feature change:
`plqmtymyrvmk`, after `kata refresh` rebased it onto the then-current default
line. This record is provenance from that refreshed tree, not an attestation of
confidence.

### PROVE

The general mechanism is declaration-selected keyword quality. A fixed keyword
may declare `parameter: Quality(preposition, nominal_number)`; the generated
grammar realizes that parameter as `KeywordQuality::Prepositional`, containing
`PrepositionalKeywordQuality { preposition: Preposition, nominal: Nominal }`.
The checked constructor compares the parsed preposition and optional nominal
number with the declaration's data. The same arm serves a keyword line and a
granted ability. Bare referenced-quality keywords accept only unmarked
parameters, so the declared preposition cannot escape into a clause-level
predicate adjunct.

`Affinity` declares `Preposition::For` and plural nominal number. The number is
declaration data needed to resolve the invariant singular/plural surfaces
*Equipment* and *Plains* without a lexeme-naming guard. `Protection` was
migrated from the compound fixed surface to the same mechanism, declaring
`Preposition::From` with no nominal-number selection. This leaves one mechanism
for prepositional keyword quality; there is no per-keyword grammar branch and
no guard naming a word, lexeme, construction, or card.

Affected-subset iteration used 476 cards / 488 faces selected by the Affinity
or Protection surface, parent construction paths, the positive witnesses, and
the mismatched-preposition negatives. The final refreshed subset selected 258,
had zero unresolved ties, 178 parse failures, and round-tripped 258/258 clean.
The `Affinity for Equipment` witness is uniquely selected. `Affinity from
artifacts` and `Protection for black` reject.

The reverse-dependency closure gate was exactly
`deckmaste_construction_core`, `deckmaste_construction`,
`deckmaste_english_v2`, and `xtask`, because this landing changes
`deckmaste_construction_core` and `plugins/builtin_v2`. `cargo fmt --all` and
strict `cargo clippy --all-targets` for those crates passed. The closure test
gate passed; representative positive artifacts were:

```text
test result: ok. 468 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out
test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test production_expansion_is_byte_identical_across_processes ... ok
test checked_in_flavor_word_nursery_passes_the_generator_check ... ok
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

The authoritative refreshed-tree corpus gates, all with 8 workers, passed:

```text
summary {"total_units":32641,"selected_units":19527,"covered_units":19527,"selected_uncovered_units":0,"parse_failures":13114,"unresolved_ties":0,"internal_failures":0,"exception_resolved":0,"exception_uses":0,"roundtrip_mismatch_units":0,"ownership_failure_units":0,"licensed_vocab_lexicon_homographs":2,"form_literal_vocab_overlaps":9,"licensing_checker_permitted":23,"licensing_checker_forbidden":0,"gap_spans":0,"gap_bytes":0,"overlap_spans":0,"overlap_bytes":0,"synthetic_claims":0,"provenance_plan_mismatches":0} lock_mode=report
English v2 accepted-set round trip
  parse accepted         19527
  clean                  19527
  mismatched             0
  not parse accepted     13114
```

`DECKMASTE_COVERAGE_LOCK=report` was present on both coverage commands.
`coverage --check` and `coverage --bless` reported the same delta: lock covered
19,469 -> 19,527 (+58), with no losses. The construction count is 393 -> 394.
The refreshed-parent ambiguity census was 19,469 selected (15,271 unique;
4,198 specificity-resolved; 13,172 failures); the feature census is 19,527
selected (15,320 unique; 4,207 specificity-resolved; 13,114 failures). Both
have zero unresolved ties, zero internal failures, and zero exception uses.
The per-unit diff is 58 gains, zero losses, and 196 selected-to-selected path
changes with zero resolution changes.

Frogmite (`0cda1a1...`), Myr Enforcer (`dac10f4d...`), and Thoughtcast
(`a23e54f7...`) select `KeywordLine > QualifiedKeywordLineItem >
PrepositionalKeywordQuality > NominalBarePlural > HeadPlural` (Thoughtcast
then continues through its unchanged draw sentence). All 58 gains contain the
prepositional keyword-quality arm and none contains
`ReferencedQualityKeywordAbility`; 49 are unique and 9 are
specificity-resolved. Every gain is a positive Affinity oracle. The exact
report-mode lock delta follows:

```text
newly covered 58 corpus identities
newly covered	017fdc472b88b443db174f90a81161d2bfe3f87b781b515d6d9b7cde3e2c3fac	card "Chromescale Drake"	selected_analysis "Affinity for artifacts\nFlying\nWhen this creature enters, reveal the top three cards of your library. Put all artifact cards revealed this way into your hand and the rest into your graveyard."
newly covered	01b17b2c8778d0b60f5628bb4535fb534526ad12f49eb6076a28386033f25af1	card "Angelic Observer"	selected_analysis "Affinity for Citizens\nFlying"
newly covered	09bc3958cc01939abd4059afbe4bfc021e374493d377ca81710803b95fb2e912	card "Panther Robot"	selected_analysis "Affinity for artifacts\nReach, trample"
newly covered	0b3171d02b5258ff770b9c1f4be88fdb216c9f1d8643a01454b07109b76caf26	card "Reality Heist"	selected_analysis "Affinity for artifacts\nLook at the top seven cards of your library. You may reveal up to two artifact cards from among them and put them into your hand. Put the rest on the bottom of your library in a random order."
newly covered	0cda1a1dd90a06b4a5622252e32edf47528dd32bd035a0b51ae5f120c177c35c	card "Frogmite"	selected_analysis "Affinity for artifacts"
newly covered	0fb9c9e1ad3c72177ccac343d8d4877189e8fe6293da837108e429b4970a8458	card "Ethersworn Sphinx"	selected_analysis "Affinity for artifacts\nFlying\nCascade"
newly covered	128554b3f0284e4cac6b187fbf61b3ba1169b29013b545a33f94040aa5d8a7e0	card "Sapling Nursery"	selected_analysis "Affinity for Forests\nLandfall — Whenever a land you control enters, create a 3/4 green Treefolk creature token with reach.\n{1}{G}, Exile this enchantment: Treefolk and Forests you control gain indestructible until end of turn."
newly covered	17b5dd3fd3304dcc2db1ba391cfa43caa8517dc38fd4cebb7d947aca9226432b	card "Gate Colossus"	selected_analysis "Affinity for Gates\nThis creature can't be blocked by creatures with power 2 or less.\nWhenever a Gate you control enters, you may put this card from your graveyard on top of your library."
newly covered	1cd5e44ab79c2dac2e1774fe506bc050bb98ecb11b108824917b30455afc1751	card "Quicksilver Behemoth"	selected_analysis "Affinity for artifacts\nWhen this creature attacks or blocks, return it to its owner's hand at end of combat."
newly covered	1d599a2219f5205e7dd1502473a3bb40aa88bcaf7542ba65c286bb2ad71dbbfb	card "Millicent, Restless Revenant"	selected_analysis "Affinity for Spirits\nFlying\nWhenever Millicent or another nontoken Spirit you control dies or deals combat damage to a player, create a 1/1 white Spirit creature token with flying."
newly covered	1de00f19b2deccee076dd950a6d1f43ab64791fd737e89a39254add6b881174e	card "Somber Hoverguard"	selected_analysis "Affinity for artifacts\nFlying"
newly covered	274e6bb23276c96ed658c12dea5dcd2138a04ebb6f1d7084850acb9d0432b07e	card "Goldwardens' Gambit"	selected_analysis "Affinity for Equipment\nCreate five 2/2 red Rebel creature tokens. They gain haste until end of turn. For each of those tokens, you may attach an Equipment you control to it."
newly covered	2ae59c7d7b3cc2affd0f74091ee870274002a3864e7f63cb1f8f253affaf8efe	card "Furnace Hellkite"	selected_analysis "Affinity for artifacts\nFlying\n{R}: This creature gets +1/+0 until end of turn."
newly covered	2d389f2f5d8fa0eae95e708e08d46ca36f76271e19459295fc5a9046565e6f29	card "Utrom Monitor"	selected_analysis "Affinity for artifacts\nFlying"
newly covered	30b4bed53ee27e61d2069b43c5eabd1d5ff972a18707dcf34c2eb96896a363ee	card "Spire Golem"	selected_analysis "Affinity for Islands\nFlying"
newly covered	3673d670cb4916e34e715755e9e14c7f646a41216da7fd70f356eecdbd005474	card "Dross Golem"	selected_analysis "Affinity for Swamps\nFear"
newly covered	3ab43e0d47128252f77bd9010267b2b49d9988d64f698b52e516b66e96050ef1	card "Witherbloom, the Balancer"	selected_analysis "Affinity for creatures\nFlying, deathtouch\nInstant and sorcery spells you cast have affinity for creatures."
newly covered	3b28f21a4b5a83a21f7d88167e87e5936d7aca46753e4ad9c4ec57a3d7b72ec7	card "Emry, Lurker of the Loch"	selected_analysis "Affinity for artifacts\nWhen Emry enters, mill four cards.\n{T}: Choose target artifact card in your graveyard. You may cast that card this turn."
newly covered	3d580286340107bcd49d5a411795f2b51efcd88e613aad63c9b04173f6cfdfca	card "Qumulox"	selected_analysis "Affinity for artifacts\nFlying"
newly covered	4cb4f216eb9180a0385d9433760b8f4621e5a66ec87b89dd4f1e8b9b3395cae1	card "Voyage Home"	selected_analysis "Affinity for artifacts\nYou draw three cards and gain 3 life."
newly covered	58e6d320f6a24ac4d90be0debd9d73f56cfc0bd2601f41cdfc66aa9781cfb47a	card "Slag Strider"	selected_analysis "Affinity for artifacts\n{1}, Sacrifice an artifact: This creature deals 1 damage to any target."
newly covered	608d8360ef0cd723103056e92a9bdb9ee7aa7d2cfba497a838e7097b98b32028	card "Scale of Chiss-Goria"	selected_analysis "Flash\nAffinity for artifacts\n{T}: Target creature gets +0/+1 until end of turn."
newly covered	65350e809a049cdadc5a14c7da15515b8ca8ddb0b98a7dfa1d4ef2fc3fc3a1b9	card "Argivian Phalanx"	selected_analysis "Affinity for creatures\nVigilance"
newly covered	68e7c91a4342a0b3d3383463d159fcc950e4530438ff94fd6df5611aebfa1ea9	card "Valkyrie Aerial Unit"	selected_analysis "Affinity for artifacts\nFlying\nWhen this creature enters, surveil 2."
newly covered	699e00869e94385107f8a0b2a8c2180ff1b9cb39de3b65e5328e51efebd5922b	card "Tangle Golem"	selected_analysis "Affinity for Forests"
newly covered	6d60a6e19a44af9dbd36f0122bb158f43f235f0b8f52405e0a89301807a994cc	card "Thought Monitor"	selected_analysis "Affinity for artifacts\nFlying\nWhen this creature enters, draw two cards."
newly covered	70c4ebde80d409f516b6c71447d8f7ac2c5ee542d8b8a332c7ab15b5e93ad9a6	card "Cantankerous Keepers"	selected_analysis "Affinity for Elves\nWhen this creature enters, mill four cards, then put all Elf cards from among them into your hand."
newly covered	732ad88ba454b3c1d7e4b3ceb9abeb0ca4b92775b2eb789c8ee01a84bb3268d0	card "Oxidda Golem"	selected_analysis "Affinity for Mountains\nHaste"
newly covered	7561320511292d75028d74d4fb2208db06628f48c4ecae12d840bf409d4aafbb	card "Urza, Chief Artificer"	selected_analysis "Affinity for artifact creatures\nArtifact creatures you control have menace.\nAt the beginning of your end step, create a 0/0 colorless Construct artifact creature token with \"This token gets +1/+1 for each artifact you control.\""
newly covered	789b91ce8714c032c2272e1476f862f3e2fe27ee9389575e92a09f404bac3801	card "Gearseeker Serpent"	selected_analysis "Affinity for artifacts\n{5}{U}: This creature can't be blocked this turn."
newly covered	7c8a8967b79a5f93974dc5fed25634b7055bfdf01fd16a10f7e12f4a339953f6	card "Thrumming Hivepool"	selected_analysis "Affinity for Slivers\nSlivers you control have double strike and haste.\nAt the beginning of your upkeep, create two 1/1 colorless Sliver creature tokens."
newly covered	7e633b47d78985872f7fc2bd8d97b66028d24eaa0d6638fd0a0e5b5f81d01d85	card "Junk Winder"	selected_analysis "Affinity for tokens\nWhenever a token you control enters, tap target nonland permanent an opponent controls. It doesn't untap during its controller's next untap step."
newly covered	7f15ef58075dab97c36872234dc635330b5dff6a39142aa141a0ad0a3be08e7d	card "Travel the Overworld"	selected_analysis "Affinity for Towns\nDraw four cards."
newly covered	85f091b06ef3aece319e2ed7f8bb42c6a6730234e164bd964fcd18502c2ce0a3	card "Lens Flare"	selected_analysis "Affinity for artifacts\nLens Flare deals 5 damage to target attacking or blocking creature."
newly covered	94fa01763f7cfea33879a2c1a2aba65c577df8f5f91b558ca21bccd68d3ba8d8	card "Into Thin Air"	selected_analysis "Affinity for artifacts\nReturn target artifact to its owner's hand."
newly covered	99cd1d74a96553a74531259ca88d97f2415e515150359a52aa4ee2ebc6c569ed	card "Mycosynth Golem"	selected_analysis "Affinity for artifacts\nArtifact creature spells you cast have affinity for artifacts."
newly covered	9a86101fd6172bb48b19493ca62dc7f5b0885eca82e2938bc30eaaccdd9c77d5	card "Polliwallop"	selected_analysis "Affinity for Frogs\nTarget creature you control deals damage equal to twice its power to target creature you don't control."
newly covered	9b015b5090b04b3dd8a99f6c6e09bcc27af536f8fa815e637f9e98e8dfd6d189	card "Claws Out"	selected_analysis "Affinity for Cats\nCreatures you control get +2/+2 until end of turn."
newly covered	a23e54f7db2d552b26ba83482a4c49775d55f8c2f99cd5a3b75bb761f9ad53c4	card "Thoughtcast"	selected_analysis "Affinity for artifacts\nDraw two cards."
newly covered	a54babc9e9472cd8e37ab3f0829161990b7f5eb6894bd07709be04043bc019af	card "Sky-Blessed Samurai"	selected_analysis "Affinity for enchantments\nFlying"
newly covered	ab5c6fe146d97e4d25bc88bfb474d68fe5bbdce59047fb6cd9e8c8f43abb530c	card "Memory Guardian"	selected_analysis "Affinity for artifacts\nFlying"
newly covered	ac3bb4eb80e94d71b4974734fa2972e1200120855b7fab01e569d9d6796c190e	card "Plated Onslaught"	selected_analysis "Affinity for artifacts\nCreatures you control get +2/+1 until end of turn."
newly covered	b423f7a9e1737a638a4ddbe81a803a99e0696325ab2ed37e57d5f06ebd3af5a8	card "Tooth of Chiss-Goria"	selected_analysis "Flash\nAffinity for artifacts\n{T}: Target creature gets +1/+0 until end of turn."
newly covered	cafc986e3a529a5ccbe0db1856d545964e97f93ca066b381617b194839fbd656	card "Scales of Shale"	selected_analysis "Affinity for Lizards\nTarget creature gets +2/+0 and gains lifelink and indestructible until end of turn."
newly covered	cc94e8b665f0db4b6458a1290ae9aa0e5035cce5435ec9e658dce19b365edbeb	card "Steelfin Whale"	selected_analysis "Affinity for artifacts\nWhenever an artifact you control enters, untap this creature."
newly covered	ccec0afbde9aec98a24bd8247df51e2512439603f17e8de4a66f57166a502a71	card "Brine Giant"	selected_analysis "Affinity for enchantments"
newly covered	d223f1c4c637a5d4fbf7dc7fe6113d439935b780ec4ab73971d5a7b1c383cefe	card "Oxidda Finisher"	selected_analysis "Affinity for Equipment\nTrample"
newly covered	d6ff688c6487fbbef9f1c38ec8f04806eee2630c0f50cb070d4a17aabe7f1cf7	card "Leonardo, Worldly Warrior"	selected_analysis "Affinity for creatures\nDouble strike"
newly covered	d97b19fbf1127100eef0b3243b88d1ee5a59702621bb1ff6870b9ce5573260b4	card "Icebreaker Kraken"	selected_analysis "Affinity for snow lands\nWhen this creature enters, artifacts and creatures target opponent controls don't untap during that player's next untap step.\nReturn three snow lands you control to their owner's hand: Return this creature to its owner's hand."
newly covered	dac10f4d8b586f0a95857cb1d89deefd1317c2d845ee00ea06c4a135c30df3af	card "Myr Enforcer"	selected_analysis "Affinity for artifacts"
newly covered	e1983f9bd490a2cfddaa010f9be544dd9bd28984fd37e1bd16706c3f5a28bddb	card "Salt Road Packbeast"	selected_analysis "Affinity for creatures\nWhen this creature enters, draw a card."
newly covered	e38669debb6ef016afeb55b6fb04e5eaa8cdf2db020ba1ea8f5cc544ccb2a6cb	card "Bartz and Boko"	selected_analysis "Affinity for Birds\nWhen Bartz and Boko enters, each other Bird you control deals damage equal to its power to target creature an opponent controls."
newly covered	e6fc35094979a4829a99de6e459184ce38c16bf905b3111fabefe9e32ac17ce4	card "Blinkmoth Infusion"	selected_analysis "Affinity for artifacts\nUntap all artifacts."
newly covered	e97802ffa4fe60f3035fb79a9dddb1f7aaf9fcb7a64877cfd8ff2d55cb4d78f5	card "Chiss-Goria, Forge Tyrant"	selected_analysis "Affinity for artifacts\nFlying, haste\nWhenever Chiss-Goria attacks, exile the top five cards of your library. You may cast an artifact spell from among them this turn. If you do, it has affinity for artifacts."
newly covered	f268eb6ffd8f9a5843727ab111cf0c1c197c1ef8a306becc5cb1a0b860a66565	card "Allies at Last"	selected_analysis "Affinity for Allies\nUp to two target creatures you control each deal damage equal to their power to target creature an opponent controls."
newly covered	f4b97a209ddf16af33bbb36c4cd86de57c7ea8fbe6db12995f10335f5f31ccc2	card "Broodstar"	selected_analysis "Affinity for artifacts\nFlying\nBroodstar's power and toughness are each equal to the number of artifacts you control."
newly covered	f911b763d746b2788d485e5aa81dc36fdfe5b2a7fff8f1d119d3e7790f80275d	card "Razor Golem"	selected_analysis "Affinity for Plains\nVigilance"
newly covered	fd279cf2bc00e8c08960f61e54c8b840c0f45c6e94908b598fb1e4a0c352abbb	card "The Circle of Loyalty"	selected_analysis "Affinity for Knights\nCreatures you control get +1/+1.\nWhenever you cast a legendary spell, create a 2/2 white Knight creature token with vigilance.\n{3}{W}, {T}: Create a 2/2 white Knight creature token with vigilance."
```

The two previously covered grant units retain their resolution but change to
the intended selected analysis:

- `22fda5a34ab34c144c2d329fc3052ae37ec146bd68063b4549980a87c7a9eb06`
  — Sami, Wildcat Captain: unique; `GrantedAbilityLexicalVerbPhrase >
  QualifiedKeywordLineItem > PrepositionalKeywordQuality >
  NominalBarePlural > HeadPlural`. The parent instead selected
  `ReferencedQualityKeywordAbility` plus a clause-level
  `PrepositionalPredicateAdjunct`.
- `fed22a61f45396cab2d15183dbca78632bf00dcfd1db0e4cdc0914316729cd3d`
  — Tezzeret, Master of the Bridge: specificity-resolved; the same intended
  grant analysis. The parent had the same wrong split as Sami.

All 194 selected Protection identities remain selected with the same
unique/specificity resolution. For every unit, deleting only the inserted
`PrepositionalKeywordQuality` node(s) from the feature path exactly reproduces
the parent path. They are named below under their selected analysis:

#### Selection-neutral Protection: specificity, `QualifiedKeywordLineItem > KeywordQualityCoordination > PrepositionalKeywordQuality > Nominal` (6)

- `1186e59776db9c07049197e9181e727dfa16e23c5718f715067bcac50851a175` — Greensleeves, Maro-Sorcerer
- `5763dfe4d501ed59250d7288a6193ec24ce354c6feb6ff6f2b036de6af93a432` — Masked Gorgon
- `95571d278cff593f41033772cff8635a6c145a6b87bc0970ff0000e55b8aa2a1` — Mystic Crusader
- `c7ced2e89ceb43a1b178becab27cbf25bdcbf5a78b0bbb91cb915eeec19c7e66` — Sword of Hearth and Home
- `dd54595d0a87a11fe6da8c271a05bce11c765dbb7005d9b26dff95f7dcf828ef` — Sword of War and Peace
- `0b9717f2713cda3894048f5609a6e1ba0ba67b4f071eb59432e3d48eea8245bc` — Ureni, the Song Unending

#### Selection-neutral Protection: unique, `QualifiedKeywordLineItem > KeywordQualityCoordination > PrepositionalKeywordQuality > Nominal` (28)

- `d3d659af4ad8a1c3e95df958b24747bfa4bd03fcafc966b3d50f4949109ab417` — Akroma's Memorial
- `b6a70cd3104fb0a3fa1b5864a83c5611d5c899c95da120a5e45385bcd1806bc3` — Akroma, Angel of Fury
- `c2f2d85273ebb833bdf21648aaf0426322e9cd8128b45d8c977ad5935243dd40` — Akroma, Angel of Wrath
- `659e6c7b607cd363fb8eba8dbca136be881dac4acacdf621b22d65029b8cc24c` — Animar, Soul of Elements
- `8de19d51c6ec47094419f5cbd56d77b1062e93c3ad2cadf5ccc9461e9fb91a11` — Auriok Champion
- `4d3cf6a60e107abf6abc3f41bee9a637b3365913d586f7f9952660cea1f3edfb` — Baneslayer Angel
- `1d6249de70191215dae8e85ec3fa611881fbe0c46c0fff8bfc5a7991d237ad37` — Devoted Caretaker
- `68cabfc7d6f2080a6782c2e322d50bf6b8d6ac5baeddaac3b98aed77cbc9fe12` — Elite Inquisitor
- `8bbe357061fb4a58360e35878b7959ad92f577762ec262455b1f71b0160d01e2` — Great Sable Stag
- `7d68a0c268ab4d3a29dc05d9354bdd0b43079c70a8aafabbc5f6ed039000b1e5` — Grotesque Hybrid
- `19392114c3f9d63d0bdbede68661ca83965be2fb0d39382a3dae5ca117c7a262` — Kitsune Riftwalker
- `0905d99e148cdcd09eb1df7330f5433ea89e7169a65555943626d2b535ee75c9` — Mask of Law and Grace
- `c6004189f33877536ac70c637922cd15549fffb5ebedcb51af41f0b03d805bbe` — Mirran Crusader
- `d42d25d8e1a89d7a74b788160055766308397f1a76ffae68029be7f28c7f099a` — Oversoul of Dusk
- `d3d8edf91a9f84f8a5fc34620344ba317d421c8b00f8c5fd4cffec0f3e562c31` — Paladin en-Vec
- `1e6e02d77ad7daf53b04c88a267f3539980eb8040cc496896ab6bb66d0485f3c` — Phyrexian Crusader
- `cb3ac67e85971c244dcf86330da0a518f846cb0179230e769d03fa3ab1371614` — Questing Phelddagrif
- `289ac86aba03d8d954d054b0ff02497b626f0ae4356d94ad336c93f62ae6e2f9` — Sabertooth Nishoba
- `2c6ddef6370a2614f0687ec26ca367ca2f72c65829a31e9176b872691ff1901d` — Shield of Duty and Reason
- `631e869b8d28f6813a9b1edfcd4369b1fea7a9289bcfed0270ba08b3e4c56b3c` — Sphinx of the Steel Wind
- `520b6f93000e0d6d45eb493ea3af0f27c41b7c9055393f2f9ed313ca5bd4cf81` — Stillmoon Cavalier
- `67cb57033adce442c5ade303f71100a50d85a2e3934712c4f069dfabcae98d93` — Sword of Body and Mind
- `7ec2e6117707b1fdbe62e9d8c1375540ec9db525ed9c3b7f6c02219fde64940a` — Sword of Feast and Famine
- `31e90ab48bc18a315c67841db4c4d5efe94cf2c5f1c632acdd4f7366c0a5f62b` — Sword of Fire and Ice
- `336ca182f76d992afd5b61dafa999bb585f03653aa4f4f01bb048898f45e0071` — Sword of Forge and Frontier
- `426cb9c8fe4f53bdc424dcefe871a0076f88d9e889d129c81668766ba0375a92` — Sword of Light and Shadow
- `58ba7e0d10e3b7040241fe8b9eb2771830c78297bcaf310b87edeb60a6343027` — Sword of Sinew and Steel
- `aa91178129bd105f776429d62c991a79eda46b0472f8961b8e3fc3147b8eceab` — Sword of Truth and Justice

#### Selection-neutral Protection: specificity, `QualifiedKeywordLineItem > PrepositionalKeywordQuality > Nominal` (11)

- `507aee9f910565459786186a992c159cb26dc3d4d674540eadf3fc61fb06df87` — Basri's Lieutenant
- `ecb61f6220671f1b282c37e6ef08600a2e48e1637f4358fc8861aec9499fcf54` — Dross Harvester
- `dca5829e67330be099d3925208f01a26a8994389ea7f00badeff2553ba6f7584` — Hungry Lynx
- `ea8d431e7ed1cb2391b5d54962360a70ffb651c74cb75eae94952414b5c160cd` — Katilda, Dawnhart Martyr // Katilda's Rising Dawn / Katilda, Dawnhart Martyr
- `00a2a00fd29abe49787e3bbc9d213ee6aa066abaeddcf607896edb2f055fe8b1` — Katilda, Dawnhart Prime
- `a89909610f24dd9188df28a4b33d4a2b1bc8da784a21446e97f8cd3b9207dabf` — Mystic Enforcer
- `0a4c589ea5476d560d6ec00a755efb819530427d596614654386b37492ad3679` — Mystic Familiar
- `6a5383f57ea1b93e929a86bdb34df18221720534e0d59246b753023e05ec12d4` — Nantuko Blightcutter
- `63a2b6227571aefa6987ffa83f9126030eedcad2d0bdc1d4744bce7a0de77b04` — Opal Guardian
- `f28c1a31c05eddec61e8edd3da1ee02523a902afa879c9f1f3ab82cac1cbe5fc` — Progenitus
- `dd7ba9cc04aa68fc7b69604bd1b21a31a7885704537b8b62704e18d2354b722c` — Teroh's Vanguard

#### Selection-neutral Protection: unique, `QualifiedKeywordLineItem > PrepositionalKeywordQuality > Nominal` (149)

- `3aa9e8f06f875f514ce8ed408f9a0b1e6d27d177506ee896fd625accc70d77c9` — Abbey Gargoyles
- `7fa6e1768036e2c8c35d92d785abb7db5ca40f31210a92db69413f1dfa872a4c` — Absolute Grace
- `6cda1e070f50a99b56b0132c2857c2219c539c3c6db7ab4fababa38cdbcfa6a0` — Absolute Law
- `f7a401e2faa765d3f545304b1f6f1eba576e7d9ccd8eca658153c3908a8d9f48` — Angelic Curator
- `316a6bb4d4290b44c850dbacb5489e17696d3cea235beb32eba6a7d03d1d4a47` — Anurid Scavenger
- `6bb9a34b380deb4d49628009696c148a3de323ee89a08b7e694ff8a4c6c44ebd` — Apostle of Purifying Light
- `9bfaff4998ddad35d91dfc36ec0080f8470bea4ecf6ff54c171093408ea546cb` — Aven Smokeweaver
- `72cd7463953556b7f2e95b4cef124b0a3de360f305cf137ec468a370afc0ba84` — Azorius First-Wing
- `9a8765de402a0bf5cc79cbaa65d42101c512646e59903b77725f56c99ffc0881` — Beasts of Bogardan
- `503c1d0a7d819c346ce5ec95ad7276b2a57a7626fb1ee4000dfcb1d714222322` — Beloved Chaplain
- `eaed0438119389ac78661c6adca9b806df891fde9da8a6e85986df128a3a5993` — Black Knight
- `935c9bff925ebe6d58c99ca9ff25648444b626c7a57dfc194326256f3734a432` — Bloated Toad
- `4330a6fc6c4604bb949e2dcc234fd20cf038bf8de93fa4c1a900bd7342a0e901` — Blood Knight
- `166ca85de6662c093ef75211f2555789ef66513c7b110d9395a79ce4d89e8252` — Bog Elemental
- `a4c782eb621632aacfdb746ea6479beddb6e2590e299b15744d49d8113cc0d5f` — Burrenton Forge-Tender
- `824d93d175f988cff0b725172744be0bffa1255fc8d21b98b303402ca54035e8` — Cemetery Gate
- `667380f5ab44c010d9e41c7683b62d43241580b8d1ffb71ac41f5df0ad01619b` — Cerulean Wyvern
- `d68237f2ad76d20e7af9bba85a2a7325da90cca8e06ca1e989a1616ba0f48452` — Chameleon Colossus
- `0d44c748fbb1401d8e6885039bf99b06ec06539859ac84ca4d9d780baaf570f2` — Cho-Arrim Legate
- `a1e05b0ed2d8cb11d7d6a99b87e06b9ba178a9439e8b9bbda2ad81349a95bbd4` — Coast Watcher
- `00bd1a9c054b3679f051f5b5843d141003a691b3a3821414fd5c1216a0cab531` — Commander Eesha
- `63c378fde9cfe4d3cc2a27c9f9082ea95cdb76ff6cd631239e01fdc2320a61ff` — Crimson Acolyte
- `61259da1ce06976eca2d5e25fc9bebd232b42b89971d54f188eaa56b006065de` — Crusading Knight
- `edc9526018a9c25c1018fe11f29b2add8b1461c7a75320d8c6c4e1efc8701742` — Crypt Angel
- `0d43a7db242c2cb5f9befe01bb7a154e2414d1b46f156d499edea14ca2d984e6` — Cybernetica Datasmith
- `1ce50066208a21c6c11a17a7ce3e513bd993358b0ab36a200021f2250b5524f8` — Darkwatch Elves
- `b3942ad572a1817d33bcf6727ba3de931d5e5fca139aa4d60ca3653dfc5b8ee9` — Death Speakers
- `d9a0aa7843a034f8f2062a5a21eddda967042b4ce3953575ca5187451b80a448` — Defender of Chaos
- `38cccb716ac6f380230c7ce232331fcfce50d2e44984fe4c8a080bd8a90eda67` — Defender of Law
- `732da3b47e1c89c120e5ad894164ec5e76fad51905144bf33ccee793a3686150` — Devout Lightcaster
- `a3563c4eb966fb964bd108b6a60161a45ad890cefdada2c34a43e649c80bd0f5` — Disciple of Grace
- `73954964e813be138c62675b6cd5534bf9cf4f8007e85f46e485af78b6ef7c45` — Disciple of Law
- `72f0131888e24e40ff772c2dcfa7d578a0def79fc01dd9b1c1973be0609f556f` — Disciple of Malice
- `84234e456ddb07e43ae53aaf12e38798daa84c8caba723a63fb4719208f9442e` — Dragon Hunter
- `a8c7b0d991ca2497df3fa9aff62abc21df1b0c534dcc19ae1fbf557451427607` — Dragonstalker
- `6e5e2e5d69606dd92a14eb5102ba83dfcb580a46474bf1d1410ca006816483ca` — Duskrider Falcon
- `43421008c51f23b10bf90d9bfe7c1041f0b8702533e15ee32ff3616b7bf8d3a3` — Duskrider Peregrine
- `1b6b8dd8edf82a03de1267406fdd2204fafe4edcd22efa669feabf4655fb615c` — Eight-and-a-Half-Tails
- `98c8c084a3eadbe34def4bbc92aed0c1885a00ccc3eb8c2593c8c2128dfde673` — Enemy of the Guildpact
- `367b5c1815019659c863e5f17d8f06857146b3d89b59d6a3e32fe9078c456f65` — Eviscerator
- `ae19d377718f1c9dbff335d16f539845476a76fd39d1f154f1154deaa6cabb1b` — Fallen Cleric
- `8edcc6458af2c574a6f61a76d63759dec7f2f605865c264b905baa29707d6796` — Feline Sovereign
- `e568362142fbe2b69fc0849c20f43527c846fa193fd6540a42095a16377f2842` — Foothill Guide
- `25555f9cd479840c6e012e810a0c3e6c5caf5bde211de0a73ad3ddf5269ab72c` — Freewind Falcon
- `6901dd798c12f96e4a4211b10d7b9813fb0d5b6c403dc548593d460420bd8531` — Galina's Knight
- `245b98ed1c47aa246e6271422d49e90e117e0372e1812fb30137b65c05188fd6` — Goblin Outlander
- `13f9e532ec1bddb8b3d1cec6d31aa21c40429c1e94bba260ebb161652b53dde0` — Goblin Piledriver
- `b26acba862a7c83bd28e2fc0ff4c9dfb4774f120aa25c49ee71707c16d8f228c` — Goblin Wizard
- `875c3f1c7b35b5cac4fac455fc9a2a0b7c7549b2b1f766b1a28f787f3e9a4711` — Grave Bramble
- `f3160ed59c21db12822e4b5ceebb247ac38ab2a2a5f108ba4a60e734720eb401` — Guardian of the Guildpact
- `494c509e3090a4fdb882c8f68a1421dc2cf7b06509602ed7443d1b5184267cb9` — Guildscorn Ward
- `5affc7f9a4f51a6167b7faa773cce443074ce2fdc9e76901a4f0caceb7c80a0c` — Guma
- `e7a187ad528862a2739469ac5b10dd8db12c2c03f3153f939b6323ffa0efc990` — Hand of Cruelty
- `94eca52a3b46b5a52d8f64d9e995e8d560211ba320c3514c21ed329b95adb924` — Hand of Honor
- `e9fb8f2aae46bdb07c7710ab629d546fa41c7fb51d8918210e2086074b8f6236` — Harbinger of Spring
- `ba2e02d809981023880035c23316abed35f7b5501515737a1b5a2f12f090fdab` — Hazerider Drake
- `72866f341b300bfdcac4fcd461d2e9d0d3df9ec9dc4c4f1ef93e0b4322441267` — Hell-Bent Raider
- `2bdfe297cebbcad44b67fe596a994fc3934322119efcc343d3d998dad46b4b86` — Hexdrinker
- `4947407cbe6422f24a914017589c588731df1771777ec61b0e0009c08add55c2` — Holy Mantle
- `136046ac783864054b3cd2e0573b85a871bb13569d82e0fa0b35d6fe7e889526` — Horizon Drake
- `7fb371f93fc6ada6fb59518be3b4a473f9cb92090ab5c8f71f3bb5f2f6e77e9a` — Hunted Horror
- `83e34226d32e0770fa7cbf2e88cb14fc17838eb11177941c93b3f367c366de1f` — Ihsan's Shade
- `e2931fce364b613a22c86868f3835e83338f6f365ae3aad62f3cf9acac98305e` — Infested Roothold
- `ada9cf9dac977aadf45bb386dedd8e3d92d6d555010017b9878c700b20b95903` — Jodah's Avenger
- `2eb7f2d153a4c4a3003ec481c04dbd7252824ca2aa59336ced9b365fc34bacc3` — Karmic Guide
- `c6c4d5f900f98411942439e5ce38b6d2d85114beea8b1ab449bbc805a1d45536` — Karoo Meerkat
- `96c3e05c39c772ad221fbaf551c17b55864d1124e255a3a83afd9969af391c81` — Keeper of Kookus
- `ca52d3ed390948a299d6c74862cee5801bda100ca8c730e056d216b802cdfaea` — Knight of Glory
- `3e6995797e8d03d2238bbf3d88c44e236042c3e1db636012813790e43a4fa0a0` — Knight of Infamy
- `3b67483f231960e1e0a5bb521afcdae257c0dcbaf11c7b3b8923bc10186ec3a2` — Knight of Stromgald
- `176639b12d7634f9a5fc2254d7e0b2bdb7d44f06bb58be5031bc596fc6d87a4f` — Kor Firewalker
- `b6b60fcd6acaff6134f7661f8ae7a2802a19f7463e9865d17c72785d7b348dd7` — Llanowar Knight
- `6173e392af75a78e507efec238cf406d1c5a6e5f24067e9c19de257a0e5060e1` — Marauding Knight
- `11dd2ee1834da23ddcc196ab6761280c73947bda7aa13837d8a93394bb28cb14` — Melesse Spirit
- `162930be9f4b1ab56a1d0c70d3f1a31e8086bafc98c45a4d23e49a3d13b32d1f` — Midnight Duelist
- `2975eda489530058fdb051c2ec354e179ed58996760e94dd000941454b934e7f` — Mistcutter Hydra
- `5e40a09f4a97b806f41a862287abd80bcf75bcd8ef524a1da5478bc4541a5b4b` — Nacatl Outlander
- `1bd77dfe964d1fdcf11dab3ec33c0f57ec64d4e826533144866b05c10647d146` — Nacatl Savage
- `216843cd65652c74a9af632077c2a40f3fab9d963753135a5b4a7944642d83c9` — Narwhal
- `fa24663524cc877c404e51cfedd85dd6a76ad784963fcc75ae2812a10825e482` — Nath's Buffoon
- `84d7be900a9e218e693a3db90f4fb570d509c9cf483917af4a13ef0023026b53` — Needlebug
- `9910d9fb9e3e386e3c668458b13fc1bf0afc69b92352197059df17d4be8ea8df` — Nightwind Glider
- `6ca62d071ae295983fe5554818bc3398fa068dfcea27e085f21d820c26a26e97` — Obsidian Acolyte
- `0f70bd6ec500e205c765e92e773ceff37b000e5360b19b63aaa3f504484fd533` — Oraxid
- `e1221ff284324f271520a01e6193e689fdcfbbd3c5431b96e8df1d36ae14e2fd` — Order of Leitbur
- `6ff7ffa070d3931659f5ba0c36d8431ddf3d4cc6e98a04ac4959d93b4c06fb31` — Order of the Ebon Hand
- `d18b41e85d47b9c35623b5d5552c32002dfa0df105630e9e10b3c5406750bc75` — Order of the White Shield
- `06da81f10be5883781d191940c352ed433235681836e6308508ba55f873fbc54` — Petrified Wood-Kin
- `68b2cc5bba7644095aea0d44ba1d6f857cb8b7cc874b8cc704162f87a64cdd1b` — Phantom Centaur
- `c186a9713da5ae832100acefc75ee4fa1a56a2bacd04b1cc0f07ba720d588c89` — Repentant Blacksmith
- `044d933cbcd6e561076356d62579992ecc250e4058b3211b6b7f2242a706c884` — Righteous War
- `3cb8ae2fbe37456b8e32029a42d990bbd28834f5d1b2140761d9cb7193cf57dc` — Riptide Biologist
- `32aa2395b9ecc1c20a9b0e5c4cc4650de2adb5aa31ca6324b6068d892a7eb559` — Scalebane's Elite
- `b008f92371da89328249b04ab4b46c735576af000de3c8b6da8c3d8211eb248c` — Scragnoth
- `2ca2ec23c3613599c39ed59c20f499254d4488d7b4c808450b4e0c273c7df25d` — Sea Sprite
- `cf3603ad3a1ab173f9c5640dc4743a6321a58648f20b48561ffea94e080956a5` — Shifting Ceratops
- `0f3bab1459678396c195d8d2189a2e50e344a6646fc7078857fc45b5d49bcbfd` — Shivan Zombie
- `31df5c2f57ce9f72854fd63c3b1e5180af46085c4a9bcb7e8592ca07ccb70f7c` — Shoreline Raider
- `aa074a5b7619825447ee55e3fe471079ad06f353bf6e0e42169624bdb3e9ab65` — Silver Knight
- `dbbfba7dc25514bfc0f01bc1651f4e60740f65f3d8aab01c3f678cb641d1ef2c` — Skirk Fire Marshal
- `dcf051ab8621314e3323bbd504430d18c9e9e000d1afb2ee461d46e3976d461e` — Skylasher
- `d6eb8927b4db66c2832672594e3eb6ada92c6c1e00070f183d454244253ea80b` — Soldier of the Pantheon
- `c7d930481b9ddf2cab06ded791e48fa92f960d44072a1f4b464cad650c0a8850` — Soltari Monk
- `9f5abf033cabe14cfe97c9ca156b07d0e154228eb418f49060a9d86214eebfe3` — Soltari Priest
- `4154a5ca8335e9493ce44300b5adc960cc7872c137d6321127b1033ec203c92d` — Spare from Evil
- `51adc8e76db6e41f0f6c097dc8c3349e158e4569a3aec475789e7c83db306b96` — Sparkhunter Masticore
- `26f4508f624588859d94763f46ca64a4783d2fc68173c23458965cb5acfedee9` — Spectral Lynx
- `8980a25e59ddd3d5d351242e1e0e522ca90690c58c0abdaae71002c99bb62153` — Spectrum Sentinel
- `27340c1071db8a45556457138e6686822148b97bbf73dc7f2a6e8144d8542540` — Spirit Mantle
- `b6fe1d48dab9f3876290297c28e8105099f3597ede4f791ced19fe3fc640b3c6` — Spirit of the Night
- `1de325707d26e513e1a12019d9c6230fb711f22707013154eb15c83ebee4e1d0` — Stonecoil Serpent
- `5534720de45550a6b7f5021b8ebe99b7deb6d973618448272e5a9a8576a8cc82` — Strength of Isolation
- `176b52303c44f7f9b0fc639a304d9dd7c788772f76a9af37e96b041c0bc48083` — Strength of Lunacy
- `80778341d332b55e1737c0dd687e834e828507e003f2ab3f01411f48afc6c8cd` — Stromgald Crusader
- `8f4105ef8065f83a89193c1c9bfba01c2015a7194dd44fb3a230ed93c96d2754` — Tel-Jilad Archers
- `0561185487cdfa6cd0a429e683a320a6c9401533194b098ed908c45ee5c5fbae` — Tel-Jilad Chosen
- `dcd9c7d1faa464e64feeda78cfdf6e9d022c25a412c683ad5d0f25377cfed67b` — Tel-Jilad Defiance
- `960fb6335ac885d609852cfd6280a31371f7616551d40c0bdee6b1597c6cf07b` — Tel-Jilad Fallen
- `5d7a9d1e9bd9fa994511024dac85f8b707222908d55fef02b36598dbf994a440` — Tel-Jilad Outrider
- `a21b6febffce291e2aa1f413b89f7db67b021cb45e86f3bbe54d2211fb35819e` — Teysa, Envoy of Ghosts
- `42441ab601a15c2950c08c8cd496b8bae5ec3a279643850d1ba9f143387dcf36` — The Stasis Coffin
- `6506a6e08bf1f294038616e956d1f5fdc1bac9e0de2eba3d3fe8d8b7875728d9` — Thermal Glider
- `22857708b221eda1317e1879b4501750fb4703363339eb184ada1aeee59ddd77` — Tivadar of Thorn
- `abef24b2de30454d0901bc76b65e0e2c79f10d133e4fcc53f92b59dc70be75f6` — Tower of the Magistrate
- `f1720b4e8025844829c4960809604ed3f46c2a032d14c5c77aead68a67a11e1a` — Treetop Sentinel
- `218cc18782f117a9fc873886039024c74aa2eaccc55f08a41dc39e3c0748e952` — Tsabo Tavoc
- `6241d57a3df25c32dfcf3908fa82aa1c80fdfd12bf4b689f4626baadde0eb0a1` — Unchained Berserker
- `d41ad1974528d25aadc12c6b3924bdba6b8b9b75b1582195d07da389009d879c` — Unquestioned Authority
- `31b74cc44909d05612067906d637d6961503fba198f54b027199e01fa0d627fc` — Valeron Outlander
- `d116dd38f702d7a797605ca2e7736c17fa3471e1f6a426f10443bc21d266bc99` — Vedalken Outlander
- `062a382f93ebdb5c8c5931dc5f8a0941ce2b72a65016e281e22989ed78fd5ce3` — Vodalian Zombie
- `0cc5406b018c2b14d9be0fa6c8a60d80b186d714223a0b9cf11a9092fa3dcbce` — Voice of Duty
- `93e3c887a26eb2d201f3c4d62b4573c8c620196ee849270200fba5fac6dce3e5` — Voice of Grace
- `cc05e4229837ce383a89cd12f59aa07aad70eea78640c80bbaa0d02b8ba31321` — Voice of Law
- `893fd7dc77fa757d0be351c1a640c849f903fdb0cae9b39cb39221b8028e5743` — Voice of Reason
- `0d515de14a9f69a551bb405535eebf72f03b9c2b8558779745a92549d2e883df` — Voice of Truth
- `f05b20e8de180ee319a8390e6d1e165835dcfa8b3d0c153662a7b87fad651cab` — Vulshok Refugee
- `d2608712b362d5f345472a4a1299a1dfd9f55ad5b034176fa100e328ccdc3a93` — Wall of Light
- `97804cbbc2e31a2b64684d928498d40ac4704a0b97d6b61c05c4e1277c1ca940` — Warren-Scourge Elf
- `d36eff123d724f9ad927ee7110ccd34c10a2a4bfa23e4bc165387754f6c28092` — Weatherseed Faeries
- `9a86b80b25dbd70d1e7370e4e5bcbcec4781047d937acf56592d8fde45733e3e` — White Knight
- `90a25f6899713399955ed8a825b7b5689fb923e3ecda4ac2a803a2a8f295f9fd` — White Shield Crusader
- `62eff0e9abc8dffe349135f1328d4daa54572f9e44566c7c441d46c6766527af` — Wildfire Emissary
- `bf72eea4f96a5a7ee585c6e07a4f9235eca6435a079bca8a9a03363f13c44c60` — Willow Priestess
- `88e963c4230b8fade3056e6e8e0f20d31d104d62d0795dc8184f6fa0fb39ca99` — Windreaper Falcon
- `666e46fa945b9697f03c10f93527c35f794ce7a2c58dbb215ac6b7668eb53b39` — Yavimaya Barbarian
- `930e3fb0b29ccdd5eee1b8bcdff0a2f1719a95e1ace98bce9145be8851943833` — Yavimaya Scion
- `6ba242118762c84d2227e808e08905549417197be435a28a8fa49f2f7d0c3364` — Yawgmoth, Thran Physician
- `5646baf3dd434995d055c294d78c1531e9d1d83c163bd134161793e5dec6e0d0` — Zombie Outlander

### DISCLOSE

The declaration compiler gained a general `role.value` field-check projection
so checked constructions can compare a parsed vocabulary value with the
declaration-selected value. The generated runtime carries the selected
preposition as a compact generated ordinal and the optional number; declaration
term indices were narrowed to `u16`. These changes preserve the hot carrier at
72 bytes. The xtask diagnostic learned the new quality arm, and the permitted
licensing-checker census changed 21 -> 23 for the two general checks. These are
ticket-required supporting changes, not fitted inventory.

Development closure runs exposed and then verified fixes for a 72 -> 128 byte
hot-carrier regression, generated-plan inventory counts, and the permitted
checker census. Each failed gate was followed by a code change before rerun.
The full refreshed feature corpus sequence was run once; the only other full
corpus work was the required claim/refreshed-parent ambiguity baseline used for
the per-unit diff.

Deviations and additions: migrating Protection to the shared arm was the
coordinator-pin preference and avoids two mechanisms. Affinity's plural number
selection was added as declaration data after the affected subset revealed
real singular/plural ties for invariant homographs. The general `role.value`
projection and compact generated parameter marker were necessary compiler
support. No construction, frame, or requirement was retained to hold an
inventory number. No citations changed, so the cite gate was not in scope.

Assurance: restored 0; re-spelled 5 existing assertions/matrices; ignored 0;
added 2 tests; removed 0. STOP: none. Glossary gap: none. Decision wanted:
none.

### REPORT

Coverage performance advisory (`--check`): 8 workers, 125,346 ms,
140,995 ns/B, host load 13.58 / 15.92 / 15.36. Ambiguity: 8 workers,
120,129 ms, 131,852 ns/B, host load 11.70 / 15.31 / 15.39. Roundtrip:
8 workers, 103,740 ms, 129,812 ns/B, host load 11.47 / 14.51 / 15.11.
These exceed the 16.26 s quiet-host ceiling under load and are reported, not
fitted. The sandbox cannot observe sibling processes, so the true concurrent
process count is unavailable for this implementer record and remains for the
reviewer to stamp.

Inventory report: 23 permitted licensing checkers, 0 forbidden; 2 licensed
vocabulary/lexicon homographs (`AttributiveAdjective::Untap` beside keyword
action `Untap`; `TargetingMarker::Target` beside `CommonNoun::Target`); 9
form-literal/vocabulary overlaps (`additional`, `to`, `the`, `next`, `to`,
`the`, `the`, `other`, `the` at the reported construction atoms); longest form
literal 11 bytes; 0 mapping layers, 0 handwritten codecs, 0 selection
exceptions, 0 terminal bindings, and 0 checked-constructor bindings.

End wall clock: 2026-09-05T07:00:07-07:00.
