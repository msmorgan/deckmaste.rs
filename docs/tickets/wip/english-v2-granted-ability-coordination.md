---
needs: []
---
# Granted keyword lines and quoted abilities coordinate through the general machinery

**R6 — Group R.** Takes over the granted-line residue that
`english-v2-with-preposition` routed to
`english-v2-underspecified-adjunct-attachment`; that routing is superseded for
this item only (the attachment residue stays there).

Defect. `granted_keyword_line`
(`crates/deckmaste_english_v2/src/constructions.rs:4780`) is
`items: seq KeywordLineItem separated by " and "` — one uniform separator and one
member type. So a comma-separated line fails
(`with trample, haste, and "This creature can't block."`), a line mixing keyword
items with a quoted ability has no path (`Quoted` is a sibling
`PrepositionalComplement` arm, not a line member), and the postmodifier form
fails on `…a creature card with deathtouch, hexproof, reach, or trample…` while
`with deathtouch` parses. Same shape on the verb side: `Have`'s quoted-ability
frame takes one `QuotedAbility`, so `Enchanted creature has "…" and "…"` fails.

Pinned shape. Coordination is general machinery. Route the granted line through
the ordinary nominal/predicate coordination algebra — the positional separator
sequence with `and` / `or` / `and-or` as distinct semantic constructions, per the
rewrite ADR's declaration-language section — over a member category that admits
both a keyword-line item and a quoted ability. No per-grant coordination family,
no second separator table, no `require` naming a keyword.

Fences. A coordination family scoped to grants. A closed list of coordinable
keywords. A `checked by` naming a keyword lexeme or a card. Adding the `or` arm
only because a witness was found — all three coordinators are declared regardless
of counts.

Glossary: Coordination, Coordinator, Keyword Line Item, Quoted Ability, Granted
Ability. Record any gap.

Baseline, measured on change `oulzkkoqmvuv` — re-measure at claim. Standard
constraints apply.

## Landing record

Measured on feature change `xzkulzuy` after the required `kata refresh`;
the coverage lock on this tree contains **17,289** identities. Refresh initially
refused while sibling workspace `workbench-dedup-tables` was divergent, then
succeeded on retry without a content conflict. Work started
`2026-09-04T15:51:11-07:00` and finished
`2026-09-04T17:56:38-07:00`.

### PROVE

- **No silent loss:** the exact parent-tip ambiguity comparison and report-mode
  coverage delta show **0 identities no longer covered**. The initially observed
  12 losses belonged to an earlier, discarded single-ability rewrite; retaining
  the established single `QuotedAbility` and `KeywordAbility` frames removed
  that regression before the final gates.
- **Structural laws:** coverage reports 17,289 selected and covered,
  0 selected-uncovered, 0 unresolved ties, 0 internal failures, 0 exception
  resolutions/uses, 0 roundtrip mismatches, 0 ownership failures, 0 construction
  traversal failures, 0 leaf traversal failures, 0 gaps, 0 overlaps, 0 synthetic
  claims, and 0 provenance-plan mismatches. Construction traversal is
  732,410/732,410 and leaf traversal is 257,208/257,208. The independent
  roundtrip gate reports 17,289 clean of 17,289 accepted.
- **No word-naming:** coverage reports 20 permitted licensing checkers and
  **0 forbidden**. No added `checked by` or `require` names a keyword lexeme,
  construction, verb, noun, preposition, or card. The existing
  `environment.rs` load-error assurance passed in the workspace suite.
- **Positive gates:** `cargo fmt --all` exited 0 (only the repository's
  stable-rustfmt warnings for nightly options);
  `CARGO_BUILD_JOBS=8 cargo clippy -p deckmaste_construction_core
  -p deckmaste_english_v2 -p xtask --all-targets -- -D warnings` exited 0;
  `CARGO_BUILD_JOBS=8 cargo test --workspace` exited 0, including
  `test result: ok. 753 passed; 0 failed; 1 ignored` for construction core,
  `test result: ok. 144 passed; 0 failed` for english-v2, and
  `test result: ok. 453 passed; 0 failed; 1 ignored` for xtask.
  `DECKMASTE_COVERAGE_LOCK=report ... coverage --check --workers 8` and
  `--bless` exited 0; `ambiguity --json --require-resolved --workers 8`
  exited 0; `roundtrip --require-clean --workers 8` exited 0. No citation
  changed, so cite gates were not required.
- A parent-tip probe initially reused the feature target and caused one stale
  construction-core artifact error. Only that disposable package artifact was
  cleaned; the final workspace suite above rebuilt and passed on the final tree.

### DISCLOSE

Selection census against the exact reflinked parent tip:

| census | parent | feature | delta |
|---|---:|---:|---:|
| selected / covered | 17,114 | 17,289 | +175 |
| parse failures | 15,527 | 15,352 | -175 |
| unique selections | 13,468 | 13,613 | +145 |
| specificity-resolved | 3,646 | 3,676 | +30 |
| exception-resolved | 0 | 0 | 0 |
| unresolved ties | 0 | 0 | 0 |

The per-unit JSON comparison found **175 gained selections, 0 lost selections,
24 changed selected construction paths, and 0 resolution changes**. The
specificity increase is in the new
`AbilityExpressionAndAbilityCoordination` route, reached through
`PrepositionalComplement::Ability` or
`VerbPhraseAbilityExpressionPredicate`; it replaces the uniform
`GrantedKeywordLineGrantedKeywordLine` analysis where an already-covered
`and` pair changes path. Every gain contains one of the three new coordination
arms: 100 verb-side `and`, 43 verb-side mixed keyword/quoted `and`, 20
nominal `or`, 7 nominal mixed `and`, 2 nominal keyword `and`, 1
verb-side nested quoted `and`, 1 nominal `and/or`, and 1 nested nominal
`or`. None is a negative oracle or a wrong analysis.

The report-mode `coverage --check` delta follows verbatim; these are all 175
new identities with the tool's selected analysis:

```text
newly covered 175 corpus identities
newly covered	007e43a50387762fdbba311fcfd49eaffff2b6278343c9e3cd0bf3f40340a4cc	card "Living Conundrum"	selected_analysis "Hexproof\nIf you would draw a card while your library has no cards in it, skip that draw instead.\nAs long as there are no cards in your library, this creature has base power and toughness 10/10 and has flying and vigilance."
newly covered	01cd23d4f4e03949d9ece36d2ad50b9fe3e8601a8d34094b741d977c596456b6	card "Argivian Avenger"	selected_analysis "{1}: Until end of turn, this creature gets -1/-1 and gains your choice of flying, vigilance, deathtouch, or haste."
newly covered	049226a185fa95bc14a2f48960ec6782725ca5746728d1befdc81fc75ce144f2	card "Urza's Avenger"	selected_analysis "{0}: This creature gets -1/-1 and gains your choice of banding, flying, first strike, or trample until end of turn."
newly covered	06b7c305313127d3c977cd102d58714f686f40dde3cdf31e47122f79ba384260	card "Face of Divinity"	selected_analysis "Enchant creature\nEnchanted creature gets +2/+2.\nAs long as another Aura is attached to enchanted creature, it has first strike and lifelink."
newly covered	0a380e60332a7f4e56aac134ace026adc0b709ce18ca726b6264990595ee3656	card "Take Flight"	selected_analysis "Enchant creature\nEnchanted creature gets +1/+0 and has flying and \"Whenever this creature attacks, draw a card.\""
newly covered	0a9e00780d37c9796a64f3374defd5233440af3f86fe897b6ab08ca5c1edbb7e	card "Fleetfeather Sandals"	selected_analysis "Equipped creature has flying and haste.\nEquip {2}"
newly covered	0b949060035588ef5bf3b6237311927cc7f850232e484d4872db54faf22a034e	card "Return to Action"	selected_analysis "Until end of turn, target creature gets +1/+0 and gains lifelink and \"When this creature dies, return it to the battlefield tapped under its owner's control.\""
newly covered	0c071b19d25cfd77590e8aec0ef1242744b69ed99ddec15711da052ff02ee496	card "Arlinn, the Pack's Hope // Arlinn, the Moon's Fury (Arlinn, the Moon's Fury)"	selected_analysis "Nightbound\n[+2]: Add {R}{G}.\n[0]: Until end of turn, Arlinn becomes a 5/5 Werewolf creature with trample, indestructible, and haste."
newly covered	0dae41c34f0350d5fc5101e71f21c8580bde0b73af0be2764974cd2b62537f19	card "Triumph of the Hordes"	selected_analysis "Until end of turn, creatures you control get +1/+1 and gain trample and infect."
newly covered	0e5089c0d563efc55e18be543fcdbe07608866a225b353c43c45ad1265aa4820	card "Pain 101"	selected_analysis "Until end of turn, target creature gains deathtouch and \"When this creature dies, return it to the battlefield tapped under its owner's control.\""
newly covered	0e917701a1d06262ec38baa421705c4d80bd38639dfac6ef80fd5d6c96e548b0	card "Nyxborn Hydra"	selected_analysis "Bestow {X}{G}{G}\nReach, trample\nThis permanent enters with X +1/+1 counters on it.\nEnchanted creature gets +1/+1 for each +1/+1 counter on this Aura and has reach and trample."
newly covered	10ebb3e98c314e9d249de7d470287d7edb5d019be129f66adefb8f9d17377fcb	card "Cathar's Call"	selected_analysis "Enchant creature\nEnchanted creature has vigilance and \"At the beginning of your end step, create a 1/1 white Human creature token.\""
newly covered	11faf26a9fa7f9499e1bd7d1eabf39fce30d647937ecef47da8da7ddbd05760f	card "Dreadmaw's Ire"	selected_analysis "Until end of turn, target attacking creature gets +2/+2 and gains trample and \"Whenever this creature deals combat damage to a player, destroy target artifact that player controls.\""
newly covered	12def992ae10e3fa7b2548d7f22117017033b36635b70d398c0cd6820d735519	card "Behemoth Sledge"	selected_analysis "Equipped creature gets +2/+2 and has trample and lifelink.\nEquip {3}"
newly covered	13b2669326c4b4c08f44aab15080ed634654c09211fc8617a69f113c80df404a	card "Eye of Nidhogg"	selected_analysis "Enchant creature\nEnchanted creature is a black Dragon with base power and toughness 4/2, has flying and deathtouch, and is goaded.\nWhen Eye of Nidhogg is put into a graveyard from the battlefield, return it to its owner's hand."
newly covered	176ca72de052cf3f9639a3a18e3a9dd920afddc5cad353449235f79120fee29b	card "Cori-Steel Cutter"	selected_analysis "Equipped creature gets +1/+1 and has trample and haste.\nFlurry — Whenever you cast your second spell each turn, create a 1/1 white Monk creature token with prowess. You may attach this Equipment to it.\nEquip {1}{R}"
newly covered	1eae00d34e8b5179fcf66185f4614ee66e31c9a71aac3b2ff676ac4d0b38c0e9	card "Cloudshredder Sliver"	selected_analysis "Sliver creatures you control have flying and haste."
newly covered	2016e58d9860eeb80bd1dbfd61ea79b207807790ddbedccb3d1afee0d2e4d16a	card "The Reaver Cleaver"	selected_analysis "Equipped creature gets +1/+1 and has trample and \"Whenever this creature deals combat damage to a player or planeswalker, create that many Treasure tokens.\"\nEquip {3}"
newly covered	204a108c3f8c082ebf7901bf01274bb885797ce89fd21723497820fd1809f648	card "Hero's Heirloom"	selected_analysis "Equipped creature gets +2/+1.\nAs long as equipped creature is legendary, it has trample and haste.\nEquip {2}"
newly covered	226c381e39ab280c6a4bea7d1178b932e2518d4513ec645bcca825ec8bbef5f1	card "Spire Tracer"	selected_analysis "This creature can't be blocked except by creatures with flying or reach."
newly covered	238d9ab29780ee962eaf899971c5fa200fbfdf8a3eb9d6aaf57ef19b4f600e01	card "Root Manipulation"	selected_analysis "Until end of turn, creatures you control get +2/+2 and gain menace and \"Whenever this creature attacks, you gain 1 life.\""
newly covered	23ca334cbf36fed72375944a356f95d8e71e9382d24a49cd7c44f7f6e956443f	card "Falcon's Wing Harness"	selected_analysis "When this Equipment enters, attach it to target creature you control.\nEquipped creature gets +1/+1 and has flying and ward {1}.\nEquip {2}{U}"
newly covered	252b3b18dc058fb431d4b307abec1d7de93c9a59accd2ec9e2fdce76b1575fc2	card "Ezrim, Agency Chief"	selected_analysis "Flying\nWhen Ezrim enters, investigate twice.\n{1}, Sacrifice an artifact: Ezrim gains your choice of vigilance, lifelink, or hexproof until end of turn."
newly covered	2608138cfba1a6d4487dc990dc2b154b78509c130b8ed46b6e182403710fcce1	card "Giant Ankheg"	selected_analysis "Trample\nWard {2}\nOther creatures you control have trample and ward {2}."
newly covered	2704254bdcb5b17566c46569e4ee5e49a2957798c4e10de6716f704af13f0449	card "Ulvenwald Oddity // Ulvenwald Behemoth (Ulvenwald Behemoth)"	selected_analysis "Trample, haste\nOther creatures you control get +1/+1 and have trample and haste."
newly covered	28b18fae966103e1c2d21f08fe730d9ffd469465cead9a371b76776f25dfd46b	card "Dragon Throne of Tarkir"	selected_analysis "Equipped creature has defender and \"{2}, {T}: Other creatures you control gain trample and get +X/+X until end of turn, where X is this creature's power.\"\nEquip {3}"
newly covered	2ca697602e53198568ccb13f5d15624d2a9b887c4eea4e5726a1ac4006fa5d9f	card "Hunter's Prowess"	selected_analysis "Until end of turn, target creature gets +3/+3 and gains trample and \"Whenever this creature deals combat damage to a player, draw that many cards.\""
newly covered	2d9166255aba133f4a796bc66b7f561507a91820e3c20ea4023c2ceec90d469f	card "Dissection Tools"	selected_analysis "When this Equipment enters, manifest dread, then attach this Equipment to that creature.\nEquipped creature gets +2/+2 and has deathtouch and lifelink.\nEquip—Sacrifice a creature."
newly covered	2e2958ff1b13b8c334de713fbde6edf7b4b6a2cf275c5f47816b6d1de9ec9acc	card "Sokka, Tenacious Tactician"	selected_analysis "Menace, prowess\nOther Allies you control have menace and prowess.\nWhenever you cast a noncreature spell, create a 1/1 white Ally creature token."
newly covered	30337c973e40c07f8683c226681c95f459c95d87cc6c06d2158f1b3ec8f3295e	card "Canopy Cover"	selected_analysis "Enchant creature\nEnchanted creature can't be blocked except by creatures with flying or reach.\nEnchanted creature can't be the target of spells or abilities your opponents control."
newly covered	306c2e7d0781abd6064e2f8b5eb59b7ed9a883572fb9a13e96a20c42e07ff788	card "Forebear's Blade"	selected_analysis "Equipped creature gets +3/+0 and has vigilance and trample.\nWhenever equipped creature dies, attach this Equipment to target creature you control.\nEquip {3}"
newly covered	3159286c7cf19f12a9bfb5ed6a36816cbd292f8daa09cbc745762f10b1492ab6	card "Henrika Domnathi // Henrika, Infernal Seer (Henrika, Infernal Seer)"	selected_analysis "Flying, deathtouch, lifelink\n{1}{B}{B}: Each creature you control with flying, deathtouch, and/or lifelink gets +1/+0 until end of turn."
newly covered	31da682bf7310c1eb9c5a02f4c6f7176ab2858dc166b215482159f4ff0b3e298	card "Super State"	selected_analysis "Enchant creature you control\nEnchanted creature has base power and toughness 9/9 and has flying, first strike, trample, and haste.\nWhenever enchanted creature deals combat damage to an opponent, it deals that much damage to each other opponent."
newly covered	3248c2b6ac7562b558c83f0fc4c1784f62aae4ce1fccbd64a74b2bd7197bfa3a	card "Dragonsoul Knight"	selected_analysis "First strike\n{W}{U}{B}{R}{G}: Until end of turn, this creature becomes a Dragon, gets +5/+3, and gains flying and trample."
newly covered	3332f536907ef86d56fff5602a1abc821ae68316d1d4eb389773eef689f6eb41	card "Reptil, Dinomorpher"	selected_analysis "Brontosaurus — {3}: Until end of turn, Reptil becomes a Dinosaur Hero with base power and toughness 3/5 and gains reach and vigilance.\nTyrannosaurus Rex — {6}: Until end of turn, Reptil becomes a Dinosaur Hero with base power and toughness 6/6 and gains trample."
newly covered	36ab374720d578e8b3fd248bd26712a19aa168b94495376ec636eb0cf51e5a10	card "Peregrine Mask"	selected_analysis "Equipped creature has defender, flying, and first strike.\nEquip {2}"
newly covered	3c77d1b30772dbc15ed14b6476cd1d67f1c2743dfb218b70049cbd70a02216ea	card "Lightning Greaves"	selected_analysis "Equipped creature has haste and shroud.\nEquip {0}"
newly covered	3d3129cedaa8d252d90dd9814f161a162017c8e48a601ccb2ea49a58ed426b8b	card "Hunter's Bow"	selected_analysis "When this Equipment enters, attach it to target creature you control. That creature deals damage equal to its power to up to one target creature you don't control.\nEquipped creature has reach and ward {2}.\nEquip {1}"
newly covered	3dceebe870f6e0e33c2f2eb72cbfc32dc0968aac4370b93e93379a4af91d07f2	card "Nazgûl Battle-Mace"	selected_analysis "Equipped creature has menace, deathtouch, annihilator 1, and \"Whenever an opponent sacrifices a nontoken permanent, put that card onto the battlefield under your control unless that player pays 3 life.\"\nEquip {3}"
newly covered	3e4bb8bfb4106f1754936ac8a946588cd632bd25f6dd618d60e54f949a6195be	card "Veiled Apparition"	selected_analysis "When an opponent casts a spell, if this permanent is an enchantment, it becomes a 3/3 Illusion creature with flying and \"At the beginning of your upkeep, sacrifice this creature unless you pay {1}{U}.\""
newly covered	40d0d6da7355c8a1703520e45a3082818107bfd711a3b60655066bb6c77753b1	card "Orchard Spirit"	selected_analysis "This creature can't be blocked except by creatures with flying or reach."
newly covered	411db658739087d486351df878f5f8a95666590ebf7986b89639161f26f85522	card "Dragonloft Idol"	selected_analysis "As long as you control a Dragon, this creature gets +1/+1 and has flying and trample."
newly covered	4350cf2840900337ef2e95f4d2d1f5021e806c6e76c0809767510f661ededc75	card "Vibranium Strike Gauntlets"	selected_analysis "Flash\nWhen this Equipment enters, attach it to target creature you control.\nEquipped creature gets +3/+0 and has trample and \"Whenever this creature deals combat damage to a player, draw a card.\"\nEquip {3}"
newly covered	43957c0f7f8ea48dab8fbda5b9deb93d6b4db80f7210f13147b85b3b1fe3c626	card "Zephyr Net"	selected_analysis "Enchant creature\nEnchanted creature has defender and flying."
newly covered	44040169cfc4b419303403d34c1532fe0f8d07665699317369d79e3e3f0f468b	card "Daybreak Coronet"	selected_analysis "Enchant creature with another Aura attached to it\nEnchanted creature gets +3/+3 and has first strike, vigilance, and lifelink."
newly covered	4438e138abd1d52fafb67b708af64a34f159569254059bc3823f46888978894f	card "Wrench"	selected_analysis "Equipped creature gets +1/+1 and has vigilance and \"{3}, {T}: Tap target creature.\"\n{2}, Sacrifice this Equipment: Draw a card.\nEquip {2}"
newly covered	44c0f9eac11ee263c054449f474afa8fc7613532eeec761988a2f13e09c1fc4a	card "Walking Sponge"	selected_analysis "{T}: Target creature loses your choice of flying, first strike, or trample until end of turn."
newly covered	4625972071a2f0a90312a1e436b5b912591688830c24d4ee4d06d12d3e37a7cc	card "Kaldra Compleat"	selected_analysis "Living weapon\nIndestructible\nEquipped creature gets +5/+5 and has first strike, trample, indestructible, haste, and \"Whenever this creature deals combat damage to a creature, exile that creature.\"\nEquip {7}"
newly covered	481a7e7bee6ef6794531af0078824838e8b7fc525878b857dceed66ac7319500	card "Tenacious Hunter"	selected_analysis "As long as a creature has a -1/-1 counter on it, this creature has vigilance and deathtouch."
newly covered	48db21e15e48ac5cb88b8dc8f7f3f34d5465e4d2818177d06c8cec7057317183	card "Batterskull"	selected_analysis "Living weapon\nEquipped creature gets +4/+4 and has vigilance and lifelink.\n{3}: Return this Equipment to its owner's hand.\nEquip {5}"
newly covered	4b3eae42d24ccb6ae45370b9a1bba1426c811d937eaec8473df95625dc162300	card "Angelic Overseer"	selected_analysis "Flying\nAs long as you control a Human, this creature has hexproof and indestructible."
newly covered	4dd82ba201ef18b0d525a5ad3c4891dca078f89d46c3d175418f5832f8dba525	card "Titanic Ultimatum"	selected_analysis "Until end of turn, creatures you control get +5/+5 and gain first strike, trample, and lifelink."
newly covered	4e2028d936e66b231f8d1ce7049904899ada6e61684b1217223393c88f4f811e	card "Mirror Shield"	selected_analysis "Equipped creature gets +0/+2 and has hexproof and \"Whenever a creature with deathtouch blocks or becomes blocked by this creature, destroy that creature.\"\nEquip {2}"
newly covered	4e68ba143ea7f7233bf8f20adbd254a7c29e5eab3e8a07241216a5c7f5828d84	card "Cloudform"	selected_analysis "When this enchantment enters, it becomes an Aura with enchant creature. Manifest the top card of your library and attach this enchantment to it.\nEnchanted creature has flying and hexproof."
newly covered	4f08d23cf5fa1bb47b22c4b22ce05d2228a5580838456c021ee1e48720e71ff6	card "On Serra's Wings"	selected_analysis "Enchant creature\nEnchanted creature is legendary, gets +1/+1, and has flying, vigilance, and lifelink."
newly covered	4f15cb13c0736b9b2e897a00d079c510ce1ce2671691127f418cde3ab041e0b6	card "Manifold Mouse"	selected_analysis "Offspring {2}\nAt the beginning of combat on your turn, target Mouse you control gains your choice of double strike or trample until end of turn."
newly covered	4fd2bb0954ff513ccabb5307c3420c838ed97f23c276c7d75e5b5b8914994fd0	card "Lightform"	selected_analysis "When this enchantment enters, it becomes an Aura with enchant creature. Manifest the top card of your library and attach this enchantment to it.\nEnchanted creature has flying and lifelink."
newly covered	502387722c212b46dc18553359472d2e403b62596e3ebd1cbda79992cd410565	card "Junkyard Genius"	selected_analysis "When this creature enters, create a tapped Powerstone token.\n{1}{B}{R}, Sacrifice another creature or artifact: Until end of turn, other creatures you control get +1/+0 and gain menace and haste."
newly covered	50d167702b08200d681480a9ee8b3b35dbf70c2241aa210cc55c6475dc1cf2ee	card "Teferi, Temporal Pilgrim"	selected_analysis "Whenever you draw a card, put a loyalty counter on Teferi.\n[0]: Draw a card.\n[−2]: Create a 2/2 blue Spirit creature token with vigilance and \"Whenever you draw a card, put a +1/+1 counter on this token.\"\n[−12]: Target opponent chooses a permanent they control and returns it to its owner's hand. Then they shuffle each nonland permanent they control into its owner's library."
newly covered	516fb1b53b6632dc3ad4ffd4c9bb98ac4c7cbde114909ec13b0f114b0665be1a	card "Super Strength"	selected_analysis "Enchant creature\nEnchanted creature gets +4/+4 and has trample and ward {1}."
newly covered	52c7f8a177887be2352f63f0bee9c8e9edb0831a240db802c95c32fd076aca50	card "Eldrazi Conscription"	selected_analysis "Enchant creature\nEnchanted creature gets +10/+10 and has trample and annihilator 2."
newly covered	52d0f1b8181047756002fa04d62e510a6a58ba71087035aecc302dc21aee16f8	card "Cloak of the Bat"	selected_analysis "Equipped creature has flying and haste.\nEquip {2}"
newly covered	53c1793a27b2c9cf91a7379c54871aa8186ec71e3265474a1975323f7ff595c4	card "Celestial Archon"	selected_analysis "Bestow {5}{W}{W}\nFlying, first strike\nEnchanted creature gets +4/+4 and has flying and first strike."
newly covered	5459dab7424c10e41170fc7bf83df54e437bc8fa4c40816b1602e59e4fdfd52e	card "True Conviction"	selected_analysis "Creatures you control have double strike and lifelink."
newly covered	54edc3bd3c43a1fa0ce2d96f71ba128161b26ccaa4c3426b74a7b0532cce4615	card "Ballroom Brawlers"	selected_analysis "Whenever this creature attacks, this creature and up to one other target creature you control both gain your choice of first strike or lifelink until end of turn."
newly covered	54f866959d8a93ae794e438663593a570b22bdaedeef23821172c77cc0a5d2de	card "Swiftfoot Boots"	selected_analysis "Equipped creature has hexproof and haste.\nEquip {1}"
newly covered	55c122fe37c841538df5995cd0aeef8b8a21eb690c1a14b5f7f611539325012a	card "Fallen Ideal"	selected_analysis "Enchant creature\nEnchanted creature has flying and \"Sacrifice a creature: This creature gets +2/+1 until end of turn.\"\nWhen this Aura is put into a graveyard from the battlefield, return it to its owner's hand."
newly covered	58d430e671b67d3a95b272e9390c92c7fde54bff2cc86b518c42a90affeae548	card "Experimental Armor"	selected_analysis "Equipped creature gets +1/+1 and has flying and haste.\nEquip {2}"
newly covered	5b8c460b58fd69f27447ba0d8be09f4ae8c2ba5b4c9d442265e760169958030b	card "Avarice Amulet"	selected_analysis "Equipped creature gets +2/+0 and has vigilance and \"At the beginning of your upkeep, draw a card.\"\nWhenever equipped creature dies, target opponent gains control of this Equipment.\nEquip {2}"
newly covered	5b917f8f8d965feedd5f2b67db15ea35aefdfb2c7ad7d5575fff05194680256b	card "Sticky Fingers"	selected_analysis "Enchant creature\nEnchanted creature has menace and \"Whenever this creature deals combat damage to a player, create a Treasure token.\"\nWhen enchanted creature dies, draw a card."
newly covered	5b9b23b899b162c349ffa47590e996d7abeeb4d7beab6897197d0ae630249673	card "Prophetic Ravings"	selected_analysis "Enchant creature\nEnchanted creature has haste and \"{T}, Discard a card: Draw a card.\""
newly covered	5d9c55b1b70eba6058d5394956af67bf56a5af808a259d18eb43cc336a94e9a2	card "Glaive of the Guildpact"	selected_analysis "Equipped creature gets +1/+0 for each Gate you control and has vigilance and menace.\nEquip {3}"
newly covered	5db0912e9f6174ac5f7a183a9e2cfcff97e8471ac0637683fae674d93452a1bf	card "Lavaspur Boots"	selected_analysis "Equipped creature gets +1/+0 and has haste and ward {1}.\nEquip {1}"
newly covered	5f58f7243a217cbff1a661c809c9e4e25cf34a13b6b37fe6c5d889dc5a215493	card "Winged Hive Tyrant"	selected_analysis "Flying, haste\nThe Will of the Hive Mind — Other creatures you control with counters on them have flying and haste."
newly covered	60cfc2db31af8d87e91a65b88331dcc23d4924200a3c433c3ea2e7ef6601a45f	card "Lotus Ring"	selected_analysis "Indestructible\nEquipped creature gets +3/+3 and has vigilance and \"{T}, Sacrifice this creature: Add three mana of any one color.\"\nEquip {3}"
newly covered	637786ef84973071dbe378910f4d93684af92fba5e555a0544a1759fe285928f	card "Thran Golem"	selected_analysis "As long as this creature is enchanted, it gets +2/+2 and has flying, first strike, and trample."
newly covered	63fbb9284d0edd9121b3a3f07ef383677031222e01ab73c416508246a22dbafd	card "Staggering Insight"	selected_analysis "Enchant creature\nEnchanted creature gets +1/+1 and has lifelink and \"Whenever this creature deals combat damage to a player, draw a card.\""
newly covered	667096a6f6feb438312d9639f018879596244f475fc5c3599ca3c7b23693ddbd	card "Paragon of the Amesha"	selected_analysis "First strike\n{W}{U}{B}{R}{G}: Until end of turn, this creature becomes an Angel, gets +3/+3, and gains flying and lifelink."
newly covered	67814baec8854889e66100357c64c81608dd35ce5ba23edd42e3eeb7d9bc7d6a	card "Gatebreaker Ram"	selected_analysis "This creature gets +1/+1 for each Gate you control.\nAs long as you control two or more Gates, this creature has vigilance and trample."
newly covered	6788f386661080804e128c3825ccdc19523febb7d48b7155acdecc3965f781ec	card "Secret Identity"	selected_analysis "Choose one —\n• Conceal — Until end of turn, target creature you control becomes a Citizen with base power and toughness 1/1 and gains hexproof.\n• Reveal — Until end of turn, target creature you control becomes a Hero with base power and toughness 3/4 and gains flying and vigilance."
newly covered	6966056c10b168f3aa871f0fb0801c84c7abad0b6f71b54b3d6e4216fb0a04dc	card "Syr Ginger, the Meal Ender"	selected_analysis "Syr Ginger has trample, hexproof, and haste as long as an opponent controls a planeswalker.\nWhenever another artifact you control is put into a graveyard from the battlefield, put a +1/+1 counter on Syr Ginger and scry 1.\n{2}, {T}, Sacrifice Syr Ginger: You gain life equal to its power."
newly covered	6c5f87502fa2ac3f5a5fa3d877f6d1d353b653eefe481dc1428b395982ff8ec9	card "Wingnut, Bat on the Belfry"	selected_analysis "Alliance — Whenever another creature you control enters, Wingnut gains your choice of flying, menace, or haste until end of turn.\nWhenever Wingnut attacks, each other attacking creature gets +1/+0 until end of turn."
newly covered	6d7ceaf967c26aaee1520b2222636443fc7e07b871142f0eecbfc5350ef508ef	card "Arlinn Kord // Arlinn, Embraced by the Moon (Arlinn Kord)"	selected_analysis "[+1]: Until end of turn, up to one target creature gets +2/+2 and gains vigilance and haste.\n[0]: Create a 2/2 green Wolf creature token. Transform Arlinn Kord."
newly covered	727fb18f32fa2fa3170f2ffb83f383e2277e449c743ca3100fd7d66742171c97	card "Surestrike Trident"	selected_analysis "Equipped creature has first strike and \"{T}, Unattach Surestrike Trident: This creature deals damage equal to its power to target player or planeswalker.\"\nEquip {4}"
newly covered	75ed78572bf0c0e14faba9d9f6af8976c65a63233554d7c0df7348b56700a297	card "Strength of Will"	selected_analysis "Until end of turn, target creature you control gains indestructible and \"Whenever this creature is dealt damage, put that many +1/+1 counters on it.\""
newly covered	79926dd7057139e6cfed83dce8ed3e4d5b21fe3a656dba8fd3065ad12ba65f48	card "Dragonwing Glider"	selected_analysis "For Mirrodin!\nEquipped creature gets +2/+2 and has flying and haste.\nEquip {3}{R}{R}"
newly covered	7c638973147db6d282fa7492edeb76876b7fb8c919328e38b1305f811d4b0345	card "Homura, Human Ascendant // Homura's Essence (Homura's Essence)"	selected_analysis "Creatures you control get +2/+2 and have flying and \"{R}: This creature gets +1/+0 until end of turn.\""
newly covered	7d84810d7a002026af0fc7e98eb49cd3ee4e032810898cf56e645401ad0bc07b	card "Chariot of Victory"	selected_analysis "Equipped creature has first strike, trample, and haste.\nEquip {1}"
newly covered	7e6124f47701c3774ae90b10e15935c38e5179ae9d13c8e408c43ecd5e94dcbe	card "Vraska, Swarm's Eminence"	selected_analysis "Whenever a creature you control with deathtouch deals damage to a player or planeswalker, put a +1/+1 counter on that creature.\n[−2]: Create a 1/1 black Assassin creature token with deathtouch and \"Whenever this token deals damage to a planeswalker, destroy that planeswalker.\""
newly covered	7ecdd32632b54b102168354bc289ce5f053671ae37c320ec40cafce1a56a155b	card "Shackles of Treachery"	selected_analysis "Gain control of target creature until end of turn. Untap that creature. Until end of turn, it gains haste and \"Whenever this creature deals damage, destroy target Equipment attached to it.\""
newly covered	7ed95b19667ea1aa71e2d5265c0646c4c3fa0420992df6096ffbfe83ee516d8f	card "Web-Shooters"	selected_analysis "Equipped creature gets +1/+1 and has reach and \"Whenever this creature attacks, tap target creature an opponent controls.\"\nEquip {2}"
newly covered	7f4032c51f4913710e32f76f8a894a44ea441e1cebfb40caf04d7c7bb70f47e6	card "Sparkshaper Visionary"	selected_analysis "At the beginning of combat on your turn, choose any number of target planeswalkers you control. Until end of turn, they become 3/3 blue Bird creatures with flying, hexproof, and \"Whenever this creature deals combat damage to a player, scry 1.\""
newly covered	8089b0462414e27e96fe2cb02e9c836b63b01b078022b8f0cd6eb3922919fe5a	card "Golem Artisan"	selected_analysis "{2}: Target artifact creature gets +1/+1 until end of turn.\n{2}: Target artifact creature gains your choice of flying, trample, or haste until end of turn."
newly covered	812eb5fa104c034729f97d5f3a15ddbaecd888479fa10b94eea267669c7d3cce	card "The Squadron Sinister"	selected_analysis "Flying, haste\nOther Villains you control get +2/+2 and have flying and haste.\nMayhem {3}{U}{R}"
newly covered	8358719dc1b4fe7f1b12e4232e3b2c6b3798b807bf61ac15cfb644413f411176	card "Shao Jun"	selected_analysis "Leap Strike — During your turn, Shao Jun has flying and first strike.\nRope Dart — Tap two untapped artifacts you control: Shao Jun deals 1 damage to each opponent."
newly covered	83e6aa5af48002c94c7a4d7ecfbf8b2873c5b90ff10210c4d329173d4dc778f7	card "Chronicle of Victory"	selected_analysis "As Chronicle of Victory enters, choose a creature type.\nCreatures you control of the chosen type get +2/+2 and have first strike and trample.\nWhenever you cast a spell of the chosen type, draw a card."
newly covered	8704a2c42e30d77705858d9d8e61057ff9fdcef7353c3714c51a8f5921d8609d	card "Wings of Aesthir"	selected_analysis "Enchant creature\nEnchanted creature gets +1/+0 and has flying and first strike."
newly covered	8925775b90c8f96a3d3d90fb2493fde9f15851760a1bb99cc370ab460824f5e6	card "Sword of Vengeance"	selected_analysis "Equipped creature gets +2/+0 and has first strike, vigilance, trample, and haste.\nEquip {3}"
newly covered	89cebba32ca7c1837b0698359e53eae57d68d0a4c9a6ada2526ddc1742290654	card "Krang, Utrom Warlord"	selected_analysis "Flying, trample, indestructible, haste\nOther artifact creatures you control have flying, trample, indestructible, and haste."
newly covered	89e006e0da21b46c54cfdb5e4696972a37c441068a18448d6768374f19a89497	card "Gift of Orzhova"	selected_analysis "Enchant creature\nEnchanted creature gets +1/+1 and has flying and lifelink."
newly covered	8a2c62bde15b2942ba63f07afaffa0b90a5935f450c75578b942f934bcf4be8a	card "Short Bow"	selected_analysis "Equipped creature gets +1/+1 and has reach and vigilance.\nEquip {1}"
newly covered	8bc22d85e843346befac20fa6636132b3404ea93c2dee201772c6b8077790f10	card "Bladed Pinions"	selected_analysis "Equipped creature has flying and first strike.\nEquip {2}"
newly covered	8eb2c96dde36cb8b5ef64ecd85e5632aad76929275fe7a5b324b54fc8c80649d	card "Messenger's Speed"	selected_analysis "Enchant creature\nEnchanted creature has trample and haste."
newly covered	8f961338c004e4e62a522e9377d6e3b253ee719354994b875372c2cbcf8f8530	card "Butcher of the Horde"	selected_analysis "Flying\nSacrifice another creature: This creature gains your choice of vigilance, lifelink, or haste until end of turn."
newly covered	916632230e35a6c9ada51f03de9d7365421dc09cceacf2da1980fb003b9bbd4f	card "Sejiri Merfolk"	selected_analysis "As long as you control a Plains, this creature has first strike and lifelink."
newly covered	92a8eebf0506a85d17e8623743d2304578a7e9e9f88db6915c3e70fbce8aafa3	card "Flayer of Loyalties"	selected_analysis "When you cast this spell, gain control of target creature until end of turn. Untap that creature. Until end of turn, it has base power and toughness 10/10 and gains trample, annihilator 2, and haste.\nAnnihilator 2\nTrample"
newly covered	96650e9896c5143a302c169854bba3621083b09a539a0be8293fda50dcac9293	card "Infantry Shield"	selected_analysis "Equipped creature has menace and mobilize X, where X is its power.\nEquip {2}"
newly covered	97d2f20c2f0d20d8405c5b3b8305d2284155d3a03e3c27e5cf96a1544f9283d9	card "Midnight Angel Armor"	selected_analysis "When this Equipment enters, create a 1/1 white Soldier creature token, then attach this Equipment to it.\nEquipped creature gets +3/+3 and has flying and vigilance.\nEquip {3}"
newly covered	99f4c2704d46fe429c16d5d36ace30ad7f524208054c684e486ae15824f5a6bd	card "Fly"	selected_analysis "Enchant creature\nEnchanted creature has flying and \"Whenever this creature deals combat damage to a player, venture into the dungeon.\""
newly covered	9a4fc19757ced86bdde0e901ce61e7caea681c3f9c2a61b8359920636ff45e4c	card "Esika, God of the Tree // The Prismatic Bridge (Esika, God of the Tree)"	selected_analysis "Vigilance\n{T}: Add one mana of any color.\nOther legendary creatures you control have vigilance and \"{T}: Add one mana of any color.\""
newly covered	9ab9d0ce31a511e806afff7b5ff0f4bb42bd94ba89e3365068e241e7323e5018	card "Practiced Offense"	selected_analysis "Put a +1/+1 counter on each creature target player controls. Target creature gains your choice of double strike or lifelink until end of turn.\nFlashback {1}{W}"
newly covered	9bc6bc4e3b865d2d006247dfd950fe2a744ac90c0179c2a63e2d04f0ce60506b	card "Maul of the Skyclaves"	selected_analysis "When this Equipment enters, attach it to target creature you control.\nEquipped creature gets +2/+2 and has flying and first strike.\nEquip {2}{W}{W}"
newly covered	9caa5ec87eae3dded5ad7f76c50ffdf6a2bf812a4bac8da30639a14fe038b3d2	card "Driven // Despair (Despair)"	selected_analysis "Aftermath\nUntil end of turn, creatures you control gain menace and \"Whenever this creature deals combat damage to a player, that player discards a card.\""
newly covered	9ebc8bbae3bff23bd315644b3c3574a4a534a9d0e974f2cafd0d0e63db679b31	card "Orcish Medicine"	selected_analysis "Target creature gains your choice of lifelink or indestructible until end of turn.\nAmass Orcs 1."
newly covered	9ee9facb22e6a789d0f739de89a68e7402f0677aca0bd5aa991f1e3d614a7642	card "Zephid's Embrace"	selected_analysis "Enchant creature\nEnchanted creature gets +2/+2 and has flying and shroud."
newly covered	9f3063cd336322fd1cbdc86fd47a570d15ef11217c0a73bbb54dcfc9a147088b	card "Loxodon Warhammer"	selected_analysis "Equipped creature gets +3/+0 and has trample and lifelink.\nEquip {3}"
newly covered	a13fa54b5e6e12dfddbf947e20b0e2dedaf761582b238b5f654995aaa6a6b723	card "Demonic Ruckus"	selected_analysis "Enchant creature\nEnchanted creature gets +1/+1 and has menace and trample.\nWhen this Aura is put into a graveyard from the battlefield, draw a card.\nPlot {R}"
newly covered	a36271e577655f8de05300fad5049408a2b55435773f0f5e861a0adfde9ab60d	card "Invasion of Pyrulea // Gargantuan Slabhorn (Gargantuan Slabhorn)"	selected_analysis "Trample, ward {2}\nOther transformed permanents you control have trample and ward {2}."
newly covered	a3de05d36145e0b71a0a139930150161f3d960411f619a15540ceb1676db7956	card "Eternal Thirst"	selected_analysis "Enchant creature\nEnchanted creature has lifelink and \"Whenever a creature an opponent controls dies, put a +1/+1 counter on this creature.\""
newly covered	a3f0dcf6748146c121efe963206016c3d2502831696276031d75996d69c42e3c	card "Tower Above"	selected_analysis "Until end of turn, target creature gets +4/+4 and gains trample, wither, and \"When this creature attacks, target creature blocks it this turn if able.\""
newly covered	a6108791ddea85cd62ec17a9ef570075ab039e205bc62e7551a7cd856bebb906	card "Assassin Initiate"	selected_analysis "{1}: This creature gains your choice of flying, deathtouch, or lifelink until end of turn."
newly covered	a98b2eb045e290ebb195e074864fc73d235d6a6b2e2b2fd3f7acce61efc51c04	card "Dragon Egg"	selected_analysis "Defender\nWhen this creature dies, create a 2/2 red Dragon creature token with flying and \"{R}: This token gets +1/+0 until end of turn.\""
newly covered	ada9cf9dac977aadf45bb386dedd8e3d92d6d555010017b9878c700b20b95903	card "Jodah's Avenger"	selected_analysis "{0}: Until end of turn, this creature gets -1/-1 and gains your choice of double strike, protection from red, vigilance, or shadow."
newly covered	aed83366ef51667e0f2032d447689d692c848a851c3a74d81b4f1af0a7e2c2a8	card "Pheres-Band Warchief"	selected_analysis "Vigilance, trample\nOther Centaur creatures you control get +1/+1 and have vigilance and trample."
newly covered	af08770cf22ebd20d7f8ccff696da368e7dda6020e7e77babc2dd471545115fb	card "Mask of Griselbrand"	selected_analysis "Equipped creature has flying and lifelink.\nWhenever equipped creature dies, you may pay X life, where X is its power. If you do, draw X cards.\nEquip {3}"
newly covered	b1d3f50ff65e6b3568314d2ff05294ba752ff220d6e9a011dfeb7ae95d60920b	card "Gray Slaad // Entropic Decay (Gray Slaad)"	selected_analysis "As long as there are four or more creature cards in your graveyard, this creature has menace and deathtouch."
newly covered	b2c3aaa565d9ef2377d408bbd9252d9f9a434d9071e5a92ebe629439a61f7785	card "Brilliant Wings"	selected_analysis "Flash\nEnchant creature you control\nEnchanted creature has flying and hexproof.\nWhenever a creature you control enters, you may pay {1}. If you do, attach this Aura to that creature."
newly covered	b9f3b0ae6a880d591f0759e4c94bd475927467dc489a8f5fdd235f93a277d81f	card "Run Wild"	selected_analysis "Until end of turn, target creature gains trample and \"{G}: Regenerate this creature.\""
newly covered	bbbfe2650ba39e62d8843cffba0d7a04c45a99027e0f2ecc9bb374bdf09667a7	card "Lightning Prowess"	selected_analysis "Enchant creature\nEnchanted creature has haste and \"{T}: This creature deals 1 damage to any target.\""
newly covered	c4226c0e5b2b66e94246e382fa84a2a61a4c23644a071e5bad569737aecb82f4	card "Gleaming Overseer"	selected_analysis "When this creature enters, amass Zombies 1.\nZombie tokens you control have hexproof and menace."
newly covered	c7fbe8eb1087fa6dcf3508114f219c1964a7a9efe21212a96415c7ad433b0329	card "Honored Hierarch"	selected_analysis "Renown 1\nAs long as this creature is renowned, it has vigilance and \"{T}: Add one mana of any color.\""
newly covered	c9ea99c4f0aa4961f8393c83d9a10fb54d4cc38258fa6323b2e7638de7450c06	card "Hexgold Halberd"	selected_analysis "For Mirrodin!\nDuring your turn, equipped creature has first strike and trample.\nEquip {2}{R}"
newly covered	cc50b8c6920876f3c0e3b24f78104f797d63b4e11dd87e5a31bc75172039e1f0	card "Commanding Presence"	selected_analysis "Enchant creature\nEnchanted creature gets +2/+2 and has first strike and \"Whenever this creature deals combat damage to a player, create a 1/1 white Human Soldier creature token.\""
newly covered	cccacc4e921d5073a0a6d48534cf7921f39f5885938f52eb18f86c7043e5cfe7	card "Djeru and Hazoret"	selected_analysis "As long as you have one or fewer cards in hand, Djeru and Hazoret has vigilance and haste.\nWhenever Djeru and Hazoret attacks, look at the top six cards of your library. You may exile a legendary creature card from among them. Put the rest on the bottom of your library in a random order. Until end of turn, you may cast the exiled card without paying its mana cost."
newly covered	cd344384b9f205247f6ffb4d12a35646c4cbe29680ea313193dfc6817e824050	card "Serra's Embrace"	selected_analysis "Enchant creature\nEnchanted creature gets +2/+2 and has flying and vigilance."
newly covered	cf3603ad3a1ab173f9c5640dc4743a6321a58648f20b48561ffea94e080956a5	card "Shifting Ceratops"	selected_analysis "This spell can't be countered.\nProtection from blue\n{G}: This creature gains your choice of reach, trample, or haste until end of turn."
newly covered	cf36cd2fcdc59702c4d50b63474cef3e1c27583e92dd07e33a8f23f627aa9175	card "Batterbone"	selected_analysis "Living weapon\nEquipped creature gets +1/+1 and has vigilance and lifelink.\nEquip {5}"
newly covered	cfeaaa8cf4b831dd275d2383540ed5db9ecf662155516f6ef56a0a3247431f42	card "Basilisk Collar"	selected_analysis "Equipped creature has deathtouch and lifelink.\nEquip {2}"
newly covered	cff0ca2add4793083f6444f3798b19510b3d0525e15d73afe1ee65916ca533f1	card "Asha's Favor"	selected_analysis "Enchant creature\nEnchanted creature has flying, first strike, and vigilance."
newly covered	cff0d4bbbe0c9ccca4645c5c82d70b5a8ed39f5f01dc6cfdaf97e83f44cd1bcf	card "Extus, Oriq Overlord // Awaken the Blood Avatar (Awaken the Blood Avatar)"	selected_analysis "As an additional cost to cast this spell, you may sacrifice any number of creatures. This spell costs {2} less to cast for each creature sacrificed this way.\nEach opponent sacrifices a creature of their choice. Create a 3/6 black and red Avatar creature token with haste and \"Whenever this token attacks, it deals 3 damage to each opponent.\""
newly covered	d032b8731d11868a51357c84ef29a9838f86460e977dea48b6c151cf1ab08dd4	card "Driven // Despair (Driven)"	selected_analysis "Until end of turn, creatures you control gain trample and \"Whenever this creature deals combat damage to a player, draw a card.\""
newly covered	d3d659af4ad8a1c3e95df958b24747bfa4bd03fcafc966b3d50f4949109ab417	card "Akroma's Memorial"	selected_analysis "Creatures you control have flying, first strike, vigilance, trample, haste, and protection from black and from red."
newly covered	d581c7b757f6976bff4a7f86809a495c7f617ee489811bfefdcb3f863fa330b8	card "Wake the Dragon"	selected_analysis "Create a 6/6 black and red Dragon creature token with flying, menace, and \"Whenever this token deals combat damage to a player, gain control of target artifact that player controls.\"\nFlashback {6}{B}{R}"
newly covered	d6aeb83040d5b141bbac51181bb5291ba8e9f37430a482e57c62d42533598164	card "Aspirant's Ascent"	selected_analysis "Until end of turn, target creature gets +1/+3 and gains flying and toxic 1."
newly covered	d76fe409c96afcd0fcbcb603ab60070b5c36b7649a260b037fb3dfd205ab8370	card "Legolas's Quick Reflexes"	selected_analysis "Split second\nUntap target creature. Until end of turn, it gains reach, hexproof, and \"Whenever this creature becomes tapped, it deals damage equal to its power to up to one target creature.\""
newly covered	d81411876b9638669eecbfb721d592bb936204f40b322e717708148045ae89f2	card "Chromanticore"	selected_analysis "Bestow {2}{W}{U}{B}{R}{G}\nFlying, first strike, vigilance, trample, lifelink\nEnchanted creature gets +4/+4 and has flying, first strike, vigilance, trample, and lifelink."
newly covered	da9564bdc142807816a5417de7c0506d613a20d17f3a9cd5b79fed763f9eb364	card "Water Wings"	selected_analysis "Until end of turn, target creature you control has base power and toughness 4/4 and gains flying and hexproof."
newly covered	dada4f64d4e2d43b6692a2f19f9073ac46df5aaefca07a0a3083dc832078f919	card "Embercleave"	selected_analysis "Flash\nThis spell costs {1} less to cast for each attacking creature you control.\nWhen Embercleave enters, attach it to target creature you control.\nEquipped creature gets +1/+1 and has double strike and trample.\nEquip {3}"
newly covered	deb33e4257dec5b2076f51fb6e2c38c44939d8b57d9889733d8c8ba46732d88f	card "Depthshaker Titan"	selected_analysis "When this creature enters, any number of target noncreature artifacts you control become 3/3 artifact creatures. Sacrifice them at the beginning of the next end step.\nEach artifact creature you control has melee, trample, and haste."
newly covered	df35cc56348d781f7deff08b72d0fa296f6af5020bff58b3e39ca73bab2ee07c	card "Dropkick Bomber"	selected_analysis "Other Goblins you control get +1/+1.\n{R}: Until end of turn, another target Goblin you control gains flying and \"When this creature deals combat damage, sacrifice it.\""
newly covered	df67d23326836ca4853170ded8bb189599ce57819f29a83c35f63fccb30a9045	card "Furnace Reins"	selected_analysis "Gain control of target creature until end of turn. Untap that creature. Until end of turn, it gains haste and \"Whenever this creature deals combat damage to a player or battle, create a Treasure token.\""
newly covered	e2163fcd47122cb8b6de639cd1e4a645a52c2589900670af79c07912ec1f5f96	card "Signal Pest"	selected_analysis "Battle cry\nThis creature can't be blocked except by creatures with flying or reach."
newly covered	e312f034fb7c22af0ca7a6e1138e38c841019cdf6b19bfd55f4f0389a97b48ac	card "Duelist's Flame"	selected_analysis "Until end of turn, target blocked creature you control gets +X/+0 and gains trample and \"Whenever this creature deals combat damage to a player, look at that many cards from the top of your library. Exile up to one nonland card from among them and put the rest on the bottom of your library in a random order. You may cast the exiled card without paying its mana cost.\""
newly covered	e386f30a1ae16353f6be9c25b582ad2aad7f413de2c1c1d3384284574c96942a	card "Alchemist's Gift"	selected_analysis "Target creature gets +1/+1 and gains your choice of deathtouch or lifelink until end of turn."
newly covered	e4685ce85e6ff28467bd3235a4908792c4068f650cee3fa7d186e8d6f220efb2	card "Consecrated by Blood"	selected_analysis "Enchant creature\nEnchanted creature gets +2/+2 and has flying and \"Sacrifice two other creatures: Regenerate this creature.\""
newly covered	e620db32f5dbe092316c5dc4cc0bb42ecb1d4bbf16bbd1bda1732e98aaed2864	card "Lunar Avenger"	selected_analysis "Sunburst\nRemove a +1/+1 counter from this creature: This creature gains your choice of flying, first strike, or haste until end of turn."
newly covered	eba41a365b18558ed449355faa0bb08b43738635cbcb00c787cbfdf52cb3d163	card "Glasswing Grace // Age-Graced Chapel (Glasswing Grace)"	selected_analysis "Enchant creature\nEnchanted creature gets +2/+2 and has flying and lifelink."
newly covered	ebe9d5860b3b9ed88853cacca0205874f13b143934bc84845dd3e4a9768b7a40	card "Aggression"	selected_analysis "Enchant non-Wall creature\nEnchanted creature has first strike and trample.\nAt the beginning of the end step of enchanted creature's controller, destroy that creature if it didn't attack this turn."
newly covered	ec14a26338c5f466cafb699660da616063b37f9fb3e1e2a6b786fe04cadfecbe	card "Sokka's Charge"	selected_analysis "During your turn, Allies you control have double strike and lifelink."
newly covered	ec2358d805a16a959164ab37c5faba1278e2769bdade0951bcf5ce53bb420c93	card "Hungry for More"	selected_analysis "Create a 3/1 black and red Vampire creature token with trample, lifelink, and haste. Sacrifice it at the beginning of the next end step.\nFlashback {1}{B}{R}"
newly covered	edf7e13bda8883d77231b00840550aed66cbb79207c5bccc039f42a578f20884	card "Rocket-Powered Goblin Glider"	selected_analysis "When this Equipment enters, if it was cast from your graveyard, attach it to target creature you control.\nEquipped creature gets +2/+0 and has flying and haste.\nEquip {2}\nMayhem {2}"
newly covered	f03102dc033235b24130e411702b72894f3c8a0b36ac5c5c4f874fad960d5543	card "Nalfeshnee"	selected_analysis "Flying\nWhenever you cast a spell from exile, copy it. You may choose new targets for the copy. If it's a permanent spell, the copy gains haste and \"At the beginning of the end step, sacrifice this permanent.\""
newly covered	f1f16dc17a0b487291d599be7863adb834140e81468e2cadb80d06cfb9038885	card "Sonic the Hedgehog"	selected_analysis "Haste\nGotta Go Fast — Whenever Sonic the Hedgehog attacks, put a +1/+1 counter on each creature you control with flash or haste.\nWhenever a creature you control with flash or haste is dealt damage, create a tapped Treasure token."
newly covered	f254d5c51bef1e14697ad12d889df3430cc1bac97194968c5f89b3d801a09a6e	card "Careful Cultivation"	selected_analysis "Enchant artifact or creature\nAs long as enchanted permanent is a creature, it gets +1/+3 and has reach and \"{T}: Add {G}{G}.\"\nChannel — {1}{G}, Discard this card: Create a 1/1 green Human Monk creature token with \"{T}: Add {G}.\""
newly covered	f3e7769ff338461d24b1dea8cc487c9aa3ae2fe9411624c1b34cc07e6a0712f5	card "Crossway Troublemakers"	selected_analysis "Attacking Vampires you control have deathtouch and lifelink.\nWhenever a Vampire you control dies, you may pay 2 life. If you do, draw a card."
newly covered	f40b8c48f3321d3f7b427032424fbac0a696bd66b4cc94ca380496f794ceb97b	card "Voice of the Blessed"	selected_analysis "Whenever you gain life, put a +1/+1 counter on this creature.\nAs long as this creature has four or more +1/+1 counters on it, it has flying and vigilance.\nAs long as this creature has ten or more +1/+1 counters on it, it has indestructible."
newly covered	f692d7915d8d869dbf07edc5bf8c9bf0cee2c05348d91b3fa61a52cb0c304643	card "Infuse with Vitality"	selected_analysis "Until end of turn, target creature gains deathtouch and \"When this creature dies, return it to the battlefield tapped under its owner's control.\"\nYou gain 2 life."
newly covered	f74baf101a3871e05403b0d28bf7b64514dec7678f7003527d0344e3413a6ffe	card "Unflinching Courage"	selected_analysis "Enchant creature\nEnchanted creature gets +2/+2 and has trample and lifelink."
newly covered	f762b9197465b05d09b395ed4ed9e9b7b07307149f022e19fcad6866eafbb79e	card "Aven Wind Guide"	selected_analysis "Flying, vigilance\nCreature tokens you control have flying and vigilance.\nEmbalm {4}{W}{U}"
newly covered	f783a16de4af406a14b70c3553b791b5adc9449e04acab92a319e3f6b1c3c37e	card "Winged Boots"	selected_analysis "Equipped creature has flying and ward {4}.\nEquip {1}"
newly covered	f7b88b788833b210fb7cb50db97d714552f6cc7a45617fa478e29a0bdb28ca76	card "Sphere Grid"	selected_analysis "Whenever a creature you control deals combat damage to a player, put a +1/+1 counter on that creature.\nUnlock Ability — Creatures you control with +1/+1 counters on them have reach and trample."
newly covered	fbc0ce6bb9452f36a832d8717b161525ca6dcb84a9d0ae72e1284e91c786088c	card "Sunspear Shikari"	selected_analysis "As long as this creature is equipped, it has first strike and lifelink."
newly covered	fbc66d513f9e18eb37516e045d3767466473b6ed310f5eb9d936d23cac802b02	card "Haunted Cloak"	selected_analysis "Equipped creature has vigilance, trample, and haste.\nEquip {1}"
newly covered	fc6799d4bb2342432acc9eb1ff0581e93f2f0285bb1f83ea4c072c7c13bce89a	card "Power Fist"	selected_analysis "Equipped creature has trample and \"Whenever this creature deals combat damage to a player, put that many +1/+1 counters on it.\"\nEquip {2}"
newly covered	fe0ef8eb213b2223edd22f161b2fa7e7e9bc44844dc4316f3c3feae5b3c8ea70	card "Swift Justice"	selected_analysis "Until end of turn, target creature gets +1/+0 and gains first strike and lifelink."
```

The following 24 previously covered identities changed selected construction
path. Each now selects the ordinary positional `and` ability coordination;
the full selected analysis is included:

- Akoum Stonewaker — `8e6e22d5263a52d72f8594e0613eac68f3dd53f9bd794b3dd8882e7478740fc1` — selected `AbilityExpressionAndAbilityCoordination`; `selected_analysis "Landfall — Whenever a land you control enters, you may pay {2}{R}. If you do, create a 3/1 red Elemental creature token with trample and haste. Exile that token at the beginning of the next end step."`
- Dancing Sword — `300839963a5efe2cde20b9ef7f3fe7ae3bd3a090b6e4088963f8fc3291adb160` — selected `AbilityExpressionAndAbilityCoordination`; `selected_analysis "Equipped creature gets +2/+1.\nWhen equipped creature dies, you may have this Equipment become a 2/1 Construct artifact creature with flying and ward {1}. If you do, it isn't an Equipment.\nEquip {1}"`
- Daring Piracy — `569eb3c2a303f811e48e9ba6270ab760c731825f8bd00826038bb49bd655e848` — selected `AbilityExpressionAndAbilityCoordination`; `selected_analysis "At the beginning of combat on your turn, create a 1/1 red Pirate creature token with menace and haste. Exile it at the beginning of the next end step."`
- Dragon Broodmother — `793c83aff79c9387a3e087b6e2c1d7fc65d87687a7276b7d5ffd76dbc60c81b6` — selected `AbilityExpressionAndAbilityCoordination`; `selected_analysis "Flying\nAt the beginning of each upkeep, create a 1/1 red and green Dragon creature token with flying and devour 2."`
- Drider — `c1327317649681d92dfb2f2dda7f342f4cdcad5f4a77486282baf908f39a45a6` — selected `AbilityExpressionAndAbilityCoordination`; `selected_analysis "Reach\nWhenever this creature deals combat damage to a player, create a 2/1 black Spider creature token with reach and menace."`
- Elemental Eruption — `9026b8260ca68277918e25afb26edd91666f061f55e7f3232e211560a187c545` — selected `AbilityExpressionAndAbilityCoordination`; `selected_analysis "Create a 4/4 red Dragon Elemental creature token with flying and prowess.\nStorm"`
- Force of Rage — `24f00453cc194b43f6c72d3afea659889afcc10a95add43a8835dc7c16c4ccf1` — selected `AbilityExpressionAndAbilityCoordination`; `selected_analysis "If it's not your turn, you may exile a red card from your hand rather than pay this spell's mana cost.\nCreate two 3/1 red Elemental creature tokens with trample and haste. Sacrifice those tokens at the beginning of your next upkeep."`
- Hornet Nest — `a987341080a5398766b2d8e2622131129c0c4c9f11214d4b0d4548788ca954c6` — selected `AbilityExpressionAndAbilityCoordination`; `selected_analysis "Defender\nWhenever this creature is dealt damage, create that many 1/1 green Insect creature tokens with flying and deathtouch."`
- Hornet Queen — `912be9bbdf4702715188ffe327a32fc638c84a4e0f4fc7e9251d52b89fd8d0db` — selected `AbilityExpressionAndAbilityCoordination`; `selected_analysis "Flying, deathtouch\nWhen this creature enters, create four 1/1 green Insect creature tokens with flying and deathtouch."`
- Jenson Carthalion, Druid Exile — `992999c3dcba011794b31f449249aa0447ef73ffba348cf89b92d5c32eb31490` — selected `AbilityExpressionAndAbilityCoordination`; `selected_analysis "Whenever you cast a multicolored spell, scry 1. If that spell was all colors, create a 4/4 white Angel creature token with flying and vigilance.\n{5}, {T}: Add {W}{U}{B}{R}{G}."`
- Oath of Eorl — `19dc572cd3fcd424f88f60f241d0804cfd1f8535267c15f985405f656150f054` — selected `AbilityExpressionAndAbilityCoordination`; `selected_analysis "I — Create two 1/1 white Human Soldier creature tokens.\nII — Create two 2/2 red Human Knight creature tokens with trample and haste.\nIII — Put an indestructible counter on up to one target Human. You become the monarch."`
- Opal Archangel — `6cc35d220ef46286978b00cd888b7969bb25568599bb39ab1f538c47652d8016` — selected `AbilityExpressionAndAbilityCoordination`; `selected_analysis "When an opponent casts a creature spell, if this permanent is an enchantment, it becomes a 5/5 Angel creature with flying and vigilance."`
- Opal Guardian — `63a2b6227571aefa6987ffa83f9126030eedcad2d0bdc1d4744bce7a0de77b04` — selected `AbilityExpressionAndAbilityCoordination`; `selected_analysis "When an opponent casts a creature spell, if this permanent is an enchantment, this enchantment becomes a 3/4 Gargoyle creature with flying and protection from red."`
- Queen Marchesa — `0148d3aef6f9ecd5e818baac2e8bf0417ab1e0fabdce4b0b81e2d611301a7585` — selected `AbilityExpressionAndAbilityCoordination`; `selected_analysis "Deathtouch, haste\nWhen Queen Marchesa enters, you become the monarch.\nAt the beginning of your upkeep, if an opponent is the monarch, create a 1/1 black Assassin creature token with deathtouch and haste."`
- Rampage of the Valkyries — `c4486507e5627a8c6a33fd550732293044afd98d8d1d990556e23f24a2379d76` — selected `AbilityExpressionAndAbilityCoordination`; `selected_analysis "When this enchantment enters, create a 4/4 white Angel creature token with flying and vigilance.\nWhenever an Angel you control dies, each other player sacrifices a creature of their choice."`
- Riders of Rohan — `0782804fabe5cf8db073e65481d6a8cfbbcd152f406c08acae64afd7bcb9f08c` — selected `AbilityExpressionAndAbilityCoordination`; `selected_analysis "When this creature enters, create two 2/2 red Human Knight creature tokens with trample and haste.\nDash {4}{R}{W}"`
- Sorin the Mirthless — `bd1b6297c83404c2376c49eac48f6418022c1dfd6e276b263cf19cb3798099ce` — selected `AbilityExpressionAndAbilityCoordination`; `selected_analysis "[+1]: Look at the top card of your library. You may reveal that card and put it into your hand. If you do, you lose life equal to its mana value.\n[−2]: Create a 2/3 black Vampire creature token with flying and lifelink.\n[−7]: Sorin deals 13 damage to any target. You gain 13 life."`
- The Legend of Roku // Avatar Roku — `e476b6fae4f3892cdd79e35fcf325fd04e52e017ba6aa3e1e408e5f6f33b933b` — selected `AbilityExpressionAndAbilityCoordination`; `selected_analysis "Firebending 4\n{8}: Create a 4/4 red Dragon creature token with flying and firebending 4."`
- The Locust God — `d76b569ea4dc0bf2311ed4918cd33554d2ce74238453072fb225c25b9e5ae819` — selected `AbilityExpressionAndAbilityCoordination`; `selected_analysis "Flying\nWhenever you draw a card, create a 1/1 blue and red Insect creature token with flying and haste.\n{2}{U}{R}: Draw a card, then discard a card.\nWhen The Locust God dies, return it to its owner's hand at the beginning of the next end step."`
- Thunderheads — `60c2873abdb5d985c44d04282fb82c57fa880a763171528157c593b30b6107d5` — selected `AbilityExpressionAndAbilityCoordination`; `selected_analysis "Replicate {2}{U}\nCreate a 3/3 blue Weird creature token with defender and flying. Exile it at the beginning of the next end step."`
- Valduk, Keeper of the Flame — `a8242394e24bf4f3491fc7b343a3c5c59f7ae5b2327c320c756a7bb25c5a479e` — selected `AbilityExpressionAndAbilityCoordination`; `selected_analysis "At the beginning of combat on your turn, for each Aura and Equipment attached to Valduk, create a 3/1 red Elemental creature token with trample and haste. Exile those tokens at the beginning of the next end step."`
- Valkyrie's Sword — `effb1d60df12b47db3066f7ef17ab4c86d920fa338ed5afc3966119f9cae5ea7` — selected `AbilityExpressionAndAbilityCoordination`; `selected_analysis "When this Equipment enters, you may pay {4}{W}. If you do, create a 4/4 white Angel Warrior creature token with flying and vigilance, then attach this Equipment to it.\nEquipped creature gets +2/+1.\nEquip {3}"`
- Warden of the First Tree — `c51e684f38f6eff22dbfc9da20a3b106e19721e64388e913161387c8841d46d4` — selected `AbilityExpressionAndAbilityCoordination`; `selected_analysis "{1}{W/B}: This creature becomes a Human Warrior with base power and toughness 3/3.\n{2}{W/B}{W/B}: If this creature is a Warrior, it becomes a Human Spirit Warrior with trample and lifelink.\n{3}{W/B}{W/B}{W/B}: If this creature is a Spirit, put five +1/+1 counters on it."`
- Warrant // Warden — `d4c90ad6dc40f981de562eaa4b6b85f2e8533e4e3bedad44c94b6ca23e7e0ada` — selected `AbilityExpressionAndAbilityCoordination`; `selected_analysis "Create a 4/4 white and blue Sphinx creature token with flying and vigilance."`

**Deviations and additions:**

- The ticket needs a continuation-space owner before either an unspaced keyword
  item or an unspaced quoted ability, while forbidding a second separator table.
  The construction declaration language had no unnested form atom that owns
  structural bytes while preserving the current case position. A
  `structural(" ")` form atom was added to construction core, documented in
  the rewrite ADR, emitted as a form-literal provenance owner with
  `StructuralTransition::Preserve`, and covered by three core tests. It does
  not relax the ADR's prohibition on continuation form atoms.
- The synthetic predicate fixture gained ordinary declarations for
  Deathtouch, Enchant, Haste, Reach, and Trample so the required exact
  nominal, coordinator, and `Enchanted creature has "…" and "…"` witnesses
  use declared vocabulary. This is test data, not a closed production keyword
  list or a guard.
- No other construction or test was added or deleted beyond the ticket's
  coordination witnesses and the structural-atom support they require.

**Assurance:** restored 0; re-spelled 2 existing environment test functions
(core frame census and exact special-frame membership); ignored 0; added 4 test
functions (3 construction-core structural-form tests, 1 english-v2 coordination
test containing four positive arms, a single-item comma regression witness, and
one negative); removed 0.

**Glossary gaps:** `Keyword Line Item`, `Quoted Ability`, and
`Granted Ability` are used by the ticket but are not defined in
`docs/contexts/oracle-english/CONTEXT.md`. `Coordination`,
`Coordinator`, and `Conjunct` are defined there.

**STOPs:** none. The discarded broad single-ability rewrite was corrected
before landing; the final tree has no loss, wrong new analysis, negative oracle,
word-naming guard, or selection tie.

### REPORT

- Coverage lock: **17,114 -> 17,289** (**+175 / -0**) in report mode.
- Construction declarations: **385 -> 390**. The five net additions are the
  shared ability-expression predicate, an unspaced quoted-ability member, and
  the three semantic coordinator arms; the established single grant and quoted
  constructions remain.
- Licensed homographs (2): `AttributiveAdjective::Untap` beside declaration
  keyword action `Untap`; `TargetingMarker::Target` beside
  `CommonNoun::Target`.
- Form-literal/vocabulary overlaps (9, reported rather than fitted):
  `additional` at `additional_cost`; `to` at
  `up_to_quantifying_determiner`; `the` and `next` at
  `definite_next_mass_quantity_reference`; `to` at
  `scalar_less_than_or_equal_to`; `the` at
  `number_of_scalar_value`; `the` at `greatest_scalar_value`;
  `other` at `other_than_qualified_reference`; and `the` at
  `positional_partitive`.
- Performance advisory, all with 8 workers and a 16.26 s quiet-host ceiling:
  coverage check **112.381 s / 119,499 ns/B**, host load
  4.75/6.19/6.99; ambiguity **112.666 s / 124,805 ns/B**, host load
  4.10/5.44/6.50; roundtrip **109.442 s / 118,164 ns/B**, host load
  3.83/5.18/6.30. Concurrent-process count is not observable in the sandbox;
  the reviewer supplies the contention stamp. These over-ceiling figures are
  advisory, not fitted gates.
