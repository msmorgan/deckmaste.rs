---
needs: [english-v3-vocabulary-batch-1]
---
# Vocabulary batch 2: the next unknown words that existing routes already read

Priority: high. Same contract as `english-v3-vocabulary-batch-1`: its rulings
V1–V9 apply verbatim, with these deltas.

## Why

After batch 1 the blocker report (`cargo xtask english-v3 blockers`) still
lists several hundred unknown words; at trunk `vwlkmvns`, before batch 1
landed, 487 distinct unknown words were the sole blocker on 1,148 faces and
1,673 unread faces were vocabulary-only.

## Goal

Declare the next vocabulary-only words, most valuable first.

## Analysis

Orchestrator rulings 2026-10-10 (deltas to batch 1):

- W1. **Candidates** come from the `word:` table of
  `/home/msmorgan/Projects/deckmaste.rs/target/english-v3/blockers.json`,
  ranked by sole-blocker faces, skipping every word already known on the
  claim parent and every word on batch 1's follow-up lists ("needs
  construction" and "blocked on declaration path"). Work down until 80 words
  are declared, the next word is sole blocker on fewer than 3 faces, or the
  time box ends.
- W2. A word that fails the vocabulary-only test is listed and skipped (V2,
  V6, V7); never STOP for a single word.
- W3. Adjectives that are attested only attributively go in the
  `AttributiveAdjective` class (V9 notes that class does not yet block
  predicative use; that is a separate grammar-hygiene follow-up).
- W4. Adverbs get every attested position licence, not only the first one
  that makes a witness read (batch 1 under-declared *secretly*).
- W5. Test rows pin the new word's role (the verb's predicate span, or the NP
  headed by the noun), never the whole unit at its own start category.

- W6. **Expanded tail and withdrawn minimum.** Orchestrator ruling 2026-10-10:
  the fifteen-word minimum is withdrawn. After the initial fourteen
  candidates, continue down the supplied `word:` table through every word
  touching at least two unread faces, even with one or zero sole-blocker
  faces; then words touching one face if time remains. Stop at 100 declared
  words or the three-hour time box. Never STOP for one word. The six
  unavailable declaration paths remain follow-ups, each naming its owner
  and missing path. This supersedes W1's three-sole-blocker cutoff.

## Method

As batch 1: passes of about 15 words; one bounded census at the end on every
face containing a declared word plus 300 random covered controls; zero lost;
zero increases on covered faces; full suites and clippy for every crate whose
code or consumed data changed; `cargo xtask facts check` if a keyword
definition's `grammar:` changed. Time box: three hours, then finish the pass
and verify.

## Landing record

1. **Census delta** — measured on `nqnztuzooxpwynzzurywltuwmpsxpory` against the data of claim parent `wuxrtpwpzwrxptqqvwmkwzztzqoroxpk`: 661 bounded faces, covered 392 → 476, Readings 11,825 → 15,863; 84 gained. Lost faces: none. Decreases, covered-face increases, digest-only changes, source changes and identity mismatches: none. One hundred word/lemma groups, comprising 104 declarations. The selector is the union of every declared surface and inflection in `target/english-v3/english-v3-vocabulary-batch-2/selectors.json`, attested through `cargo xtask english-v3 grep` over supported analysed text: 361 surface faces plus 300 covered controls selected by built-in `diff --manifest --random 300 --seed 20261008`, 661 distinct identities in `manifest.json`. Evidence is `before.json`, `after.json`, `diff.json` and `diff.log` in that directory. Both census carriers name the feature change because the parent data was replayed there before restoring the feature data. The table’s unread counts are the claim-parent bounded census filtered by `grep --census before.json --unread`; gains intersect surface identities with the built-in diff and overlap between words. Category decisions follow project rulings V4/W3/W4, not an uncited CGEL passage.

   | Word / lemma group | Category / existing frame or feature | Declaration owner / source | Parent unread surface faces | Gained surface faces |
   |---|---|---|---:|---:|
   | plotted | Owned participial adjective | `lexeme:keyword_ability/plot/adjective (plot.ron)` | 7 | 7 |
   | reselect | Verb / ObjectNP | `core-verb:Reselect (verbs.ron)` | 6 | 1 |
   | count | Verb / ObjectNP | `core-verb:Count (verbs.ron)` | 7 | 5 |
   | khans | Opaque proper Name | `lexeme:Name/Khan (core.ron)` | 5 | 5 |
   | heartwood | Count noun / NounPremodifier=Yes | `lexeme:CommonNoun/Heartwood (core.ron)` | 6 | 5 |
   | double-faced | AttributiveAdjective (W3) | `vocab:AttributiveAdjective/DoubleFaced (core.ron)` | 5 | 4 |
   | twin | Opaque proper Name | `lexeme:Name/StanggTwin (core.ron)`; `lexeme:Name/Twin (core.ron)` | 3 | 1 |
   | walker | Count noun / NounPremodifier=Yes | `lexeme:CommonNoun/Walker (core.ron)` | 3 | 3 |
   | active | AttributiveAdjective (W3) | `vocab:AttributiveAdjective/Active (core.ron)` | 6 | 1 |
   | give | Verb / ObjectNP + ObjectNP | `core-verb:Give (verbs.ron)` | 6 | 1 |
   | protect | Verb / ObjectNP | `core-verb:Protect (verbs.ron)` | 4 | 4 |
   | axe | Opaque proper Name | `lexeme:Name/Axe (core.ron)` | 2 | 0 |
   | boo | Opaque proper Name | `lexeme:Name/Boo (core.ron)` | 2 | 0 |
   | doombot | Opaque proper Name | `lexeme:Name/Doombot (core.ron)` | 2 | 2 |
   | moloid | Opaque proper Name | `lexeme:Name/Moloid (core.ron)` | 2 | 2 |
   | reorder | Verb / ObjectNP | `core-verb:Reorder (verbs.ron)` | 2 | 1 |
   | sturdy | Opaque proper Name | `lexeme:Name/SturdyShield (core.ron)` | 2 | 0 |
   | simultaneously | Adverb / VP-final, clause-initial, central preverbal and imperative (W4) | `vocab:Adverb/Simultaneously (core.ron)` | 6 | 1 |
   | quality | Count noun | `lexeme:CommonNoun/Quality (core.ron)` | 8 | 0 |
   | direction | Count noun | `lexeme:CommonNoun/Direction (core.ron)` | 5 | 0 |
   | foe | Count noun | `lexeme:CommonNoun/Foe (core.ron)` | 5 | 0 |
   | bid | Verb / ObjectNP / gerund bidding (overrides.ron); Count noun | `core-verb:Bid (verbs.ron)`; `lexeme:CommonNoun/BidNoun (core.ron)` | 3 | 0 |
   | bidding | Mass noun | `lexeme:CommonNoun/Bidding (core.ron)` | 3 | 0 |
   | chapter | Count noun / NounPremodifier=Yes | `lexeme:CommonNoun/Chapter (core.ron)` | 3 | 2 |
   | nearest | AttributiveAdjective (W3) | `vocab:AttributiveAdjective/Nearest (core.ron)` | 3 | 0 |
   | money | Mass noun | `lexeme:CommonNoun/Money (core.ron)` | 2 | 0 |
   | voter | Count noun | `lexeme:CommonNoun/Voter (core.ron)` | 2 | 0 |
   | final | AttributiveAdjective (W3) | `vocab:AttributiveAdjective/Final (core.ron)` | 4 | 2 |
   | high | AttributiveAdjective (W3) | `vocab:AttributiveAdjective/High (core.ron)` | 4 | 0 |
   | bidder | Count noun | `lexeme:CommonNoun/Bidder (core.ron)` | 3 | 0 |
   | affect | Verb / ObjectNP | `core-verb:Affect (verbs.ron)` | 2 | 2 |
   | above | Preposition / NP / modifier and predicative locative | `vocab:Preposition/Above (core.ron)` | 5 | 2 |
   | natural | AttributiveAdjective (W3) | `vocab:AttributiveAdjective/Natural (core.ron)` | 2 | 0 |
   | lethal | AttributiveAdjective (W3) | `vocab:AttributiveAdjective/Lethal (core.ron)` | 2 | 1 |
   | receive | Verb / ObjectNP | `core-verb:Receive (verbs.ron)` | 2 | 0 |
   | yet | Adverb / VP-final | `vocab:Adverb/Yet (core.ron)` | 4 | 4 |
   | guess | Verb / intransitive | `core-verb:Guess (verbs.ron)` | 7 | 0 |
   | ballistic boulder | Opaque proper Name | `lexeme:Name/BallisticBoulder (core.ron)` | 2 | 2 |
   | cordyceps infected | Opaque proper Name | `lexeme:Name/CordycepsInfected (core.ron)` | 2 | 0 |
   | koma's coil | Opaque proper Name | `lexeme:Name/KomasCoil (core.ron)` | 2 | 2 |
   | plaguebearer of nurgle | Opaque proper Name | `lexeme:Name/PlaguebearerOfNurgle (core.ron)` | 2 | 1 |
   | marit lage | Opaque proper Name | `lexeme:Name/MaritLage (core.ron)` | 2 | 0 |
   | vecna | Opaque proper Name | `lexeme:Name/Vecna (core.ron)` | 2 | 0 |
   | fewest | AttributiveAdjective (W3) / QuantitativeAdjectiveComplementUse=Yes | `vocab:AttributiveAdjective/Fewest (core.ron)` | 6 | 1 |
   | wrong | Adverb / VP-final | `vocab:Adverb/Wrong (core.ron)` | 4 | 0 |
   | following | AttributiveAdjective (W3) | `vocab:AttributiveAdjective/Following (core.ron)` | 2 | 1 |
   | characteristic | Count noun | `lexeme:CommonNoun/Characteristic (core.ron)` | 2 | 0 |
   | head | Count noun | `lexeme:CommonNoun/Head (core.ron)` | 7 | 0 |
   | tail | Count noun | `lexeme:CommonNoun/Tail (core.ron)` | 8 | 0 |
   | beneath | Preposition / NP / modifier and predicative locative | `vocab:Preposition/Beneath (core.ron)` | 3 | 0 |
   | lock | Verb / ObjectNP / PastParticipialPremodifier=Yes | `core-verb:Lock (verbs.ron)` | 4 | 3 |
   | mark | Verb / ObjectNP | `core-verb:Mark (verbs.ron)` | 3 | 1 |
   | start | Verb / present second singular only / ObjectNP | `core-verb:Start (core.ron)` | 13 | 0 |
   | stand | Verb / present third singular only / intransitive | `core-verb:Stand (core.ron)` | 3 | 0 |
   | expansion | Count noun | `lexeme:CommonNoun/AntiquitiesExpansion (core.ron)`; `lexeme:CommonNoun/HomelandsExpansion (core.ron)`; `lexeme:CommonNoun/ArabianNightsExpansion (core.ron)` | 3 | 0 |
   | abzan | Opaque proper Name | `lexeme:Name/Abzan (core.ron)` | 2 | 2 |
   | mardu | Opaque proper Name | `lexeme:Name/Mardu (core.ron)` | 3 | 1 |
   | jeskai | Opaque proper Name | `lexeme:Name/Jeskai (core.ron)` | 2 | 1 |
   | sultai | Opaque proper Name | `lexeme:Name/Sultai (core.ron)` | 2 | 2 |
   | temur | Opaque proper Name | `lexeme:Name/Temur (core.ron)` | 2 | 2 |
   | print | Verb / ObjectNP | `core-verb:Print (verbs.ron)` | 3 | 0 |
   | redhorn pass | Opaque proper Name | `lexeme:Name/RedhornPass (core.ron)` | 1 | 0 |
   | jaces | Owned count noun / Jaces plural | `lexeme:planeswalker_subtype/jace (jace.ron)` | 39 | 1 |
   | angelo | Opaque proper Name | `lexeme:Name/Angelo (core.ron)` | 1 | 0 |
   | banana | Opaque proper Name | `lexeme:Name/Banana (core.ron)` | 1 | 1 |
   | butterfly | Opaque proper Name | `lexeme:Name/Butterfly (core.ron)` | 1 | 1 |
   | carnivore | Opaque proper Name | `lexeme:Name/Carnivore (core.ron)` | 1 | 1 |
   | contract | Opaque proper Name | `lexeme:Name/Contract (core.ron)` | 1 | 1 |
   | cragflame | Opaque proper Name | `lexeme:Name/Cragflame (core.ron)` | 1 | 0 |
   | darkstar | Opaque proper Name | `lexeme:Name/Darkstar (core.ron)` | 1 | 0 |
   | humble merchant | Opaque proper Name | `lexeme:Name/HumbleMerchant (core.ron)` | 1 | 0 |
   | kaldra | Opaque proper Name | `lexeme:Name/Kaldra (core.ron)` | 1 | 0 |
   | keimi | Opaque proper Name | `lexeme:Name/Keimi (core.ron)` | 1 | 0 |
   | kelp | Opaque proper Name | `lexeme:Name/Kelp (core.ron)` | 1 | 1 |
   | lotus | Opaque proper Name | `lexeme:Name/Lotus (core.ron)` | 1 | 1 |
   | munitions | Opaque proper Name | `lexeme:Name/Munitions (core.ron)` | 1 | 1 |
   | tamiyo's notebook | Opaque proper Name | `lexeme:Name/TamiyosNotebook (core.ron)` | 1 | 0 |
   | jumblebones | Opaque proper Name | `lexeme:Name/Jumblebones (core.ron)` | 1 | 0 |
   | icy manalith | Opaque proper Name | `lexeme:Name/IcyManalith (core.ron)` | 1 | 1 |
   | stoneforged blade | Opaque proper Name | `lexeme:Name/StoneforgedBlade (core.ron)` | 1 | 0 |
   | lightning rager | Opaque proper Name | `lexeme:Name/LightningRager (core.ron)` | 1 | 1 |
   | vitu-ghazi | Opaque proper Name | `lexeme:Name/VituGhazi (core.ron)` | 1 | 1 |
   | mishra's warform | Opaque proper Name | `lexeme:Name/MishrasWarform (core.ron)` | 1 | 1 |
   | mechtitan | Opaque proper Name | `lexeme:Name/Mechtitan (core.ron)` | 1 | 0 |
   | zabu | Opaque proper Name | `lexeme:Name/Zabu (core.ron)` | 1 | 0 |
   | hornet | Opaque proper Name | `lexeme:Name/Hornet (core.ron)` | 1 | 1 |
   | wood | Opaque proper Name | `lexeme:Name/Wood (core.ron)` | 1 | 1 |
   | recent | AttributiveAdjective (W3) | `vocab:AttributiveAdjective/Recent (core.ron)` | 1 | 0 |
   | redistribute | Verb / ObjectNP | `core-verb:Redistribute (verbs.ron)` | 1 | 1 |
   | subtract | Verb / ObjectNP | `core-verb:Subtract (verbs.ron)` | 1 | 0 |
   | find | Verb / ObjectNP; ObjectNP + Among + ObjectNP | `core-verb:Find (verbs.ron)` | 4 | 0 |
   | redwing | Opaque proper Name | `lexeme:Name/Redwing (core.ron)` | 2 | 0 |
   | rock | Opaque proper Name | `lexeme:Name/Rock (core.ron)` | 1 | 0 |
   | settlement | Opaque proper Name | `lexeme:Name/Settlement (core.ron)` | 1 | 0 |
   | reliquary dragon | Opaque proper Name | `lexeme:Name/ReliquaryDragon (core.ron)` | 1 | 1 |
   | wirefly | Opaque proper Name | `lexeme:Name/Wirefly (core.ron)` | 1 | 0 |
   | zeppelin | Opaque proper Name | `lexeme:Name/Zeppelin (core.ron)` | 1 | 1 |
   | nalaar aetherjet | Opaque proper Name | `lexeme:Name/NalaarAetherjet (core.ron)` | 1 | 0 |
   | urami | Opaque proper Name | `lexeme:Name/Urami (core.ron)` | 1 | 0 |
   | tuktuk | Opaque proper Name | `lexeme:Name/TuktukTheReturned (core.ron)` | 1 | 0 |

2. **Witnesses** — all 211 rows below were probed with `cargo xtask english-v3 probe --as-card CARD --text TEXT --category CATEGORY --readings 128 --json > OUT` before their own declaration: original zero, same-category substitution control positive, complete enumeration, no issues. Staged dependencies were retained. After counts below are measured on `nqnztuzooxpwynzzurywltuwmpsxpory`; all enumerate completely with no issues. `before-IDENTITY-INDEX-original.json`, `before-IDENTITY-INDEX-control.json`, `declared-plan.json` and `declared-after.json` hold exact texts, categories, controls, roles and counts under the evidence directory. The English test checks each count and the new word’s constituent role; proper-name fragments pin the nested Name, not their root NP.

   | Word / declaration | Attested card / unit | After Readings |
   |---|---|---:|
   | plotted / plot | Lilah, Undefeated Slickshot: If you do, it becomes plotted. | 1 |
   | plotted / plot | Make Your Own Luck: If you do, it becomes plotted. | 1 |
   | reselect / Reselect | Chef's Kiss: reselect the targets at random for the spell and the copy | 3 |
   | reselect / Reselect | Grip of Chaos: reselect its target at random | 2 |
   | count / Count | Touch of the Eternal: At the beginning of your upkeep, count the number of permanents you control. | 2 |
   | count / Count | Chaos Moon: At the beginning of each upkeep, count the number of permanents. | 1 |
   | khans / Khan | Monastery Siege: As this enchantment enters, choose Khans or Dragons. | 1 |
   | khans / Khan | Citadel Siege: As this enchantment enters, choose Khans or Dragons. | 1 |
   | heartwood / Heartwood | Aerid Konstrari: When Aerid Konstrari enters or dies, create a Heartwood token. | 1 |
   | heartwood / Heartwood | Hungering Puppetbeast: When this creature enters, create a Heartwood token. | 1 |
   | double-faced / DoubleFaced | Invasion of Pyrulea: If it's a land or double-faced card, draw a card. | 1 |
   | double-faced / DoubleFaced | Nick Fury, Agent of S.H.I.E.L.D.: If it's a double-faced card, you may transform it. | 1 |
   | twin / StanggTwin | Stangg: create Stangg Twin | 1 |
   | twin / StanggTwin | Stangg, Echo Warrior: create Stangg Twin | 1 |
   | twin / Twin | Gemini Engine: create a colorless Construct artifact creature token named Twin | 5 |
   | twin / Twin | Gemini Engine: a colorless Construct artifact creature token named Twin that's attacking | 15 |
   | walker / Walker | Enkira, Hostile Scavenger: When Enkira, Hostile Scavenger enters, create two Walker tokens. | 1 |
   | walker / Walker | Gisa's Favorite Shovel: If they do, you create a Walker token. | 1 |
   | active / Active | Siren's Call: Creatures the active player controls attack this turn if able. | 4 |
   | active / Active | Belbe, Corrupted Observer: At the beginning of each postcombat main phase, the active player adds {C}{C} for each of your opponents who lost life this turn. | 12 |
   | give / Give | Powerful Broker: give that permanent or player another counter of that kind | 1 |
   | give / Give | Maulfist Revolutionary: give that permanent or player another counter of that kind | 1 |
   | protect / Protect | Etched Host Doombringer: an opponent protects it | 1 |
   | protect / Protect | Portent Tracker: an opponent protects it | 1 |
   | axe / Axe | Dáin Ironfoot: create a colorless Equipment artifact token named Axe | 4 |
   | axe / Axe | Iron Hills Blacksmith: create a colorless Equipment artifact token named Axe | 4 |
   | boo / Boo | Minsc, Beloved Ranger: create Boo | 1 |
   | boo / Boo | Minsc & Boo, Timeless Heroes: create Boo | 1 |
   | doombot / Doombot | Castle Doom: Create a 3/3 colorless Robot Villain artifact creature token named Doombot. | 7 |
   | doombot / Doombot | Doctor Doom: When Doctor Doom enters, create two 3/3 colorless Robot Villain artifact creature tokens named Doombot. | 7 |
   | moloid / Moloid | Mole Man, Moloid Master: create a 1/1 green Minion creature token named Moloid | 5 |
   | moloid / Moloid | Return of the Mole Man: Create X 1/1 green Minion creature tokens named Moloid | 5 |
   | reorder / Reorder | Search for Survivors: Reorder your graveyard at random. | 2 |
   | reorder / Reorder | Fossil Find: reorder your graveyard as you choose | 2 |
   | sturdy / SturdyShield | U.S.Agent, John Walker: create a colorless Equipment artifact token named Sturdy Shield | 4 |
   | sturdy / SturdyShield | Origin of Captain America: Create a colorless Equipment artifact token named Sturdy Shield | 4 |
   | simultaneously / Simultaneously | Goblin Game: all players reveal them simultaneously | 1 |
   | simultaneously / Simultaneously | Goblin Welder: that player simultaneously sacrifices the artifact | 1 |
   | simultaneously / Simultaneously | Breaking Wave: Simultaneously untap all tapped creatures and tap all untapped creatures. | 5 |
   | quality / Quality | Thanos, the Mad Titan: of the chosen quality | 1 |
   | quality / Quality | Extinction Event: of the chosen quality | 1 |
   | direction / Direction | Mystic Barrier: in the last chosen direction | 1 |
   | direction / Direction | Order of Succession: in the chosen direction | 1 |
   | foe / Foe | Pir's Whim: Each foe sacrifices an artifact or enchantment of their choice. | 2 |
   | foe / Foe | Virtus's Maneuver: Each foe sacrifices a creature of their choice. | 1 |
   | bid / Bid | Pain's Reward: Each player may bid life. | 1 |
   | bid / Bid | Mages' Contest: You and target spell's controller bid life. | 2 |
   | bid / BidNoun | Pain's Reward: with a bid of any number | 1 |
   | bid / BidNoun | Mages' Contest: with a bid | 1 |
   | bidding / Bidding | Pain's Reward: The bidding | 1 |
   | bidding / Bidding | Mages' Contest: If you win the bidding, counter that spell. | 1 |
   | chapter / Chapter | Narci, Fable Singer: chapter ability of a Saga you control | 5 |
   | chapter / Chapter | Tom Bombadil: chapter ability of a Saga you control | 5 |
   | nearest / Nearest | Mystic Barrier: the nearest opponent | 1 |
   | nearest / Nearest | Teyo, Geometric Tactician: the nearest opponent | 1 |
   | money / Money | Master of Ceremonies: For each player who chose money, you and that player each create a Treasure token. | 1 |
   | money / Money | Expropriate: for time or money | 2 |
   | voter / Voter | Elrond of the White Council: the voter chooses a creature they control | 1 |
   | voter / Voter | Expropriate: choose a permanent owned by the voter and gain control of it | 7 |
   | final / Final | Narci, Fable Singer: the final chapter ability of a Saga you control | 9 |
   | final / Final | Historian's Boon: the final chapter ability of a Saga you control | 9 |
   | high / High | Pain's Reward: the high bid | 1 |
   | high / High | Mages' Contest: the high bid | 1 |
   | bidder / Bidder | Pain's Reward: The high bidder loses life equal to the high bid and draws four cards. | 1 |
   | bidder / Bidder | Mages' Contest: The high bidder loses life equal to the high bid. | 1 |
   | affect / Affect | Fallaji Wayfarer: This ability doesn't affect its color identity. | 1 |
   | affect / Affect | Undergrowth: this effect doesn't affect combat damage that would be dealt by red creatures | 12 |
   | above / Above | Ashen Ghoul: three or more creature cards are above this card | 1 |
   | above / Above | Ashen Ghoul: Activate only during your upkeep and only if three or more creature cards are above this card. | 4 |
   | natural / Natural | Netherese Puzzle-Ward: a die's highest natural result | 1 |
   | natural / Natural | Netherese Puzzle-Ward: Whenever you roll a die's highest natural result, draw a card. | 1 |
   | lethal / Lethal | Ogre Enforcer: by lethal damage | 1 |
   | lethal / Lethal | Zilortha, Strength Incarnate: Lethal damage dealt to creatures you control | 12 |
   | receive / Receive | Círdan the Shipwright: they received | 1 |
   | receive / Receive | Mob Verdict: an opponent received | 1 |
   | yet / Yet | Karakyk Guardian: This creature has hexproof as long as it hasn't dealt damage yet. | 5 |
   | yet / Yet | Palladia-Mors, the Ruiner: Palladia-Mors has hexproof as long as it hasn't dealt damage yet. | 10 |
   | guess / Guess | Gollum, Scheming Guide: they guessed | 1 |
   | guess / Guess | The Seventh Doctor: they guessed | 1 |
   | ballistic boulder / BallisticBoulder | Fire Navy Trebuchet: a 2/1 colorless Construct artifact creature token with flying named Ballistic Boulder that's tapped and attacking | 56 |
   | ballistic boulder / BallisticBoulder | Mordor Trebuchet: a 2/1 colorless Construct artifact creature token with flying named Ballistic Boulder that's tapped and attacking | 56 |
   | cordyceps infected / CordycepsInfected | Ellie, Brick Master: a tapped 1/1 black Fungus Zombie creature token named Cordyceps Infected that's attacking that opponent | 28 |
   | cordyceps infected / CordycepsInfected | Abby, Merciless Soldier: 1/1 black Fungus Zombie creature tokens named Cordyceps Infected | 6 |
   | koma's coil / KomasCoil | Koma, Cosmos Serpent: a 3/3 blue Serpent creature token named Koma's Coil | 5 |
   | koma's coil / KomasCoil | Koma, World-Eater: four 3/3 blue Serpent creature tokens named Koma's Coil | 5 |
   | plaguebearer of nurgle / PlaguebearerOfNurgle | Nurgle's Rot: a 1/3 black Demon creature token named Plaguebearer of Nurgle | 5 |
   | plaguebearer of nurgle / PlaguebearerOfNurgle | Great Unclean One: a 1/3 black Demon creature token named Plaguebearer of Nurgle | 5 |
   | marit lage / MaritLage | Marit Lage's Slumber: create Marit Lage | 1 |
   | marit lage / MaritLage | Dark Depths: create Marit Lage | 1 |
   | vecna / Vecna | The Book of Vile Darkness: Create Vecna | 1 |
   | vecna / Vecna | Hand of Vecna: a creature you control named Vecna | 1 |
   | fewest / Fewest | Gor Muldrak, Amphinologist: At the beginning of your end step, each player who controls the fewest creatures creates a 4/3 blue Salamander Warrior creature token. | 1 |
   | fewest / Fewest | Gor Muldrak, Amphinologist: each player who controls the fewest creatures | 1 |
   | wrong / Wrong | The Seventh Doctor: If they guessed wrong, you may cast it without paying its mana cost. | 4 |
   | wrong / Wrong | Liar's Pendulum: If you do and your opponent guessed wrong, draw a card. | 1 |
   | following / Following | Torment of Hailfire: the following process | 1 |
   | following / Following | Protection Racket: the following process | 1 |
   | characteristic / Characteristic | Outlaws' Merriment: with those characteristics | 1 |
   | characteristic / Characteristic | Genku, Future Shaper: with those characteristics | 1 |
   | head / Head | Molten Sentry: heads | 1 |
   | head / Head | Two-Headed Giant: heads | 1 |
   | tail / Tail | Molten Sentry: tails | 1 |
   | tail / Tail | Goblin Assassin: tails | 1 |
   | beneath / Beneath | Quarry Colossus: beneath the top X cards of that library | 3 |
   | beneath / Beneath | Unexpectedly Absent: beneath the top X cards of that library | 3 |
   | lock / Lock | Marina Vendrell: Lock or unlock a door of target Room you control | 2 |
   | lock / Lock | Keys to the House: Lock or unlock a door of target Room you control | 2 |
   | lock / Lock | Ghostly Dancers: a locked door of a Room you control | 5 |
   | lock / Lock | Ghostly Keybearer: a locked door of up to one target Room you control | 7 |
   | mark / Mark | Ogre Enforcer: lethal damage dealt by a single source is marked on it | 8 |
   | mark / Mark | Pyramids: all damage marked on it | 2 |
   | start / Start | Pain's Reward: You start the bidding | 1 |
   | start / Start | Mages' Contest: You start the bidding | 1 |
   | stand / Stand | Pain's Reward: the high bid stands | 1 |
   | stand / Stand | Mages' Contest: the high bid stands | 1 |
   | expansion / AntiquitiesExpansion | Golgothian Sylex: in the Antiquities expansion | 1 |
   | expansion / AntiquitiesExpansion | Golgothian Sylex: the Antiquities expansion | 1 |
   | expansion / HomelandsExpansion | Apocalypse Chime: in the Homelands expansion | 1 |
   | expansion / HomelandsExpansion | Apocalypse Chime: the Homelands expansion | 1 |
   | expansion / ArabianNightsExpansion | City in a Bottle: in the Arabian Nights expansion | 1 |
   | expansion / ArabianNightsExpansion | City in a Bottle: the Arabian Nights expansion | 1 |
   | abzan / Abzan | Barrensteppe Siege: Abzan | 1 |
   | abzan / Abzan | Hollowmurk Siege: Abzan | 1 |
   | mardu / Mardu | Barrensteppe Siege: Mardu | 1 |
   | mardu / Mardu | Windcrag Siege: Mardu | 1 |
   | jeskai / Jeskai | Windcrag Siege: Jeskai | 1 |
   | jeskai / Jeskai | Frostcliff Siege: Jeskai | 1 |
   | sultai / Sultai | Hollowmurk Siege: Sultai | 1 |
   | sultai / Sultai | Glacierwood Siege: Sultai | 1 |
   | temur / Temur | Glacierwood Siege: Temur | 1 |
   | temur / Temur | Frostcliff Siege: Temur | 1 |
   | print / Print | Golgothian Sylex: printed in the Antiquities expansion | 1 |
   | print / Print | Apocalypse Chime: printed in the Homelands expansion | 1 |
   | redhorn pass / RedhornPass | Travel Through Caradhras: for Redhorn Pass or Mines of Moria | 1 |
   | redhorn pass / RedhornPass | Travel Through Caradhras: Redhorn Pass | 1 |
   | jaces / JacePlural | Jace, Multiverse Architect: Jaces you control | 1 |
   | jaces / JacePlural | Jace, Reality Sculptor: among Jaces you control | 1 |
   | angelo / Angelo | Rinoa Heartilly: create Angelo | 1 |
   | angelo / Angelo | Rinoa Heartilly: Angelo | 1 |
   | banana / Banana | Kibo, Uktabi Prince: a colorless artifact token named Banana | 3 |
   | banana / Banana | Kibo, Uktabi Prince: Banana | 1 |
   | butterfly / Butterfly | Giant Caterpillar: a 1/1 green Insect creature token with flying named Butterfly | 15 |
   | butterfly / Butterfly | Giant Caterpillar: Butterfly | 1 |
   | carnivore / Carnivore | Tooth and Claw: a 3/1 red Beast creature token named Carnivore | 5 |
   | carnivore / Carnivore | Tooth and Claw: Carnivore | 1 |
   | contract / Contract | Scriv, the Obligator: a white Aura enchantment token named Contract | 4 |
   | contract / Contract | Scriv, the Obligator: Contract | 1 |
   | cragflame / Cragflame | Mabel, Heir to Cragflame: create Cragflame | 1 |
   | cragflame / Cragflame | Mabel, Heir to Cragflame: Cragflame | 1 |
   | darkstar / Darkstar | Rufus Shinra: create Darkstar | 1 |
   | darkstar / Darkstar | Rufus Shinra: Darkstar | 1 |
   | humble merchant / HumbleMerchant | Honest Work: named Humble Merchant | 1 |
   | humble merchant / HumbleMerchant | Honest Work: Humble Merchant | 1 |
   | kaldra / Kaldra | Helm of Kaldra: create Kaldra | 1 |
   | kaldra / Kaldra | Helm of Kaldra: Kaldra | 1 |
   | keimi / Keimi | Tatsunari, Toad Rider: create Keimi | 1 |
   | keimi / Keimi | Tatsunari, Toad Rider: Keimi | 1 |
   | kelp / Kelp | Wall of Kelp: a 0/1 blue Plant Wall creature token with defender named Kelp | 21 |
   | kelp / Kelp | Wall of Kelp: Kelp | 1 |
   | lotus / Lotus | Kwia Vigorbloom: a colorless artifact token named Lotus | 3 |
   | lotus / Lotus | Kwia Vigorbloom: Lotus | 1 |
   | munitions / Munitions | Weapons Manufacturing: a colorless artifact token named Munitions | 3 |
   | munitions / Munitions | Weapons Manufacturing: Munitions | 1 |
   | tamiyo's notebook / TamiyosNotebook | Tamiyo, Compleated Sage: Create Tamiyo's Notebook | 1 |
   | tamiyo's notebook / TamiyosNotebook | Tamiyo, Compleated Sage: Tamiyo's Notebook | 1 |
   | jumblebones / Jumblebones | Ozox, the Clattering King: create Jumblebones | 1 |
   | jumblebones / Jumblebones | Ozox, the Clattering King: Jumblebones | 1 |
   | icy manalith / IcyManalith | Svella, Ice Shaper: a colorless snow artifact token named Icy Manalith | 4 |
   | icy manalith / IcyManalith | Svella, Ice Shaper: Icy Manalith | 1 |
   | stoneforged blade / StoneforgedBlade | Nahiri, the Lithomancer: a colorless Equipment artifact token named Stoneforged Blade | 4 |
   | stoneforged blade / StoneforgedBlade | Nahiri, the Lithomancer: Stoneforged Blade | 1 |
   | lightning rager / LightningRager | Rite of the Raging Storm: Creatures named Lightning Rager | 1 |
   | lightning rager / LightningRager | Rite of the Raging Storm: Lightning Rager | 1 |
   | vitu-ghazi / VituGhazi | Awakening of Vitu-Ghazi: a legendary 0/0 Elemental creature with haste named Vitu-Ghazi | 10 |
   | vitu-ghazi / VituGhazi | Awakening of Vitu-Ghazi: Vitu-Ghazi | 1 |
   | mishra's warform / MishrasWarform | Mishra, Eminent One: its name is Mishra's Warform | 1 |
   | mishra's warform / MishrasWarform | Mishra, Eminent One: Mishra's Warform | 1 |
   | mechtitan / Mechtitan | Mechtitan Core: Create Mechtitan | 1 |
   | mechtitan / Mechtitan | Mechtitan Core: Mechtitan | 1 |
   | zabu / Zabu | Ka-Zar of the Savage Land: create Zabu | 1 |
   | zabu / Zabu | Ka-Zar of the Savage Land: Zabu | 1 |
   | hornet / Hornet | Hornet Cannon: a 1/1 colorless Insect artifact creature token with flying and haste named Hornet | 21 |
   | hornet / Hornet | Hornet Cannon: Hornet | 1 |
   | wood / Wood | Jungle Patrol: a token named Wood | 1 |
   | wood / Wood | Jungle Patrol: Wood | 1 |
   | recent / Recent | Rocket Launcher: recent turn | 1 |
   | recent / Recent | Rocket Launcher: recent | 1 |
   | redistribute / Redistribute | Reverse the Sands: Redistribute any number of players' life totals. | 1 |
   | redistribute / Redistribute | Reverse the Sands: Redistribute any number of players' life totals | 1 |
   | subtract / Subtract | The Deck of Many Things: subtract the number of cards in your hand | 4 |
   | subtract / Subtract | The Deck of Many Things: Roll a d20 and subtract the number of cards in your hand. | 10 |
   | find / Find | Opposition Agent: they find | 1 |
   | find / Find | Odric, Blood-Cursed: found among creatures you control | 1 |
   | redwing / Redwing | Falcon, Winged Wonder: create Redwing | 1 |
   | redwing / Redwing | Falcon, Winged Wonder: Redwing | 1 |
   | rock / Rock | Toggo, Goblin Weaponsmith: named Rock | 1 |
   | rock / Rock | Toggo, Goblin Weaponsmith: Rock | 1 |
   | settlement / Settlement | Preston Garvey, Minuteman: named Settlement | 1 |
   | settlement / Settlement | Preston Garvey, Minuteman: Settlement | 1 |
   | reliquary dragon / ReliquaryDragon | Dragonbroods' Relic: named Reliquary Dragon | 1 |
   | reliquary dragon / ReliquaryDragon | Dragonbroods' Relic: Reliquary Dragon | 1 |
   | wirefly / Wirefly | Wirefly Hive: named Wirefly | 1 |
   | wirefly / Wirefly | Wirefly Hive: Wirefly | 1 |
   | zeppelin / Zeppelin | Lita, Mechanical Engineer: named Zeppelin | 1 |
   | zeppelin / Zeppelin | Lita, Mechanical Engineer: Zeppelin | 1 |
   | nalaar aetherjet / NalaarAetherjet | Pia Nalaar, Chief Mechanic: named Nalaar Aetherjet | 1 |
   | nalaar aetherjet / NalaarAetherjet | Pia Nalaar, Chief Mechanic: Nalaar Aetherjet | 1 |
   | urami / Urami | Tomb of Urami: Create Urami | 1 |
   | urami / Urami | Tomb of Urami: Urami | 1 |
   | tuktuk / TuktukTheReturned | Tuktuk the Explorer: named Tuktuk the Returned | 1 |
   | tuktuk / TuktukTheReturned | Tuktuk the Explorer: Tuktuk the Returned | 1 |

3. **Spot checks** — fifteen newly covered faces across fifteen declaration groups, measured on `nqnztuzooxpwynzzurywltuwmpsxpory`. Each selected Reading was inspected with `probe --category document --as-card CARD --readings 1`; all fifteen returned one Reading and no issues (`spot-checks.json`, `spot-0.json` through `spot-14.json`). Full enumeration and both roundtrip laws were checked by the bounded census.

   | Face | Selected analysis |
   |---|---|
   | Aerid Konstrari | Heartwood NounPremodifier |
   | Ashen Ghoul | above NP as copular locative PP |
   | Belbe, Corrupted Observer | active adjective Premodifier |
   | Citadel Siege | Khans mode Name within chosen NP |
   | Etched Host Doombringer | protect finite object-gap relative on battle |
   | Fallaji Wayfarer | affect transitive negative finite predicate |
   | Gisa’s Favorite Shovel | Walker NounPremodifier |
   | Gor Muldrak, Amphinologist | fewest quantitative attributive adjective |
   | Grip of Chaos | reselect transitive predicate |
   | Invincible Hymn | count transitive predicate |
   | Karakyk Guardian | yet VP-final adjunct |
   | Make Your Own Luck | plugin-owned plotted adjective predicate |
   | Mole Man, Moloid Master | Moloid named-token Name |
   | Ghostly Dancers | locked past-participial Premodifier |
   | Koma, Cosmos Serpent | Koma’s Coil named-token Name |

4. **Deviations and STOPs** — the initial batch-wide STOP found fewer than fifteen candidates under W1’s old cutoff. Orchestrator ruling W6 (2026-10-10, recorded above) withdrew that minimum and expanded the tail; this resolved the STOP. Work stopped at 100 declared word/lemma groups, within the three-hour declaration time box. No Construction, frame kind, role, feature declaration, loader, runtime Rust code or glossary term was added or removed. Lexicon declarations and two plugin grammar fields use existing paths; finite-only Start preserves the existing starting adjective. Bid’s existing gerund-override path spells bidding. Name compound groups fold the blocker-table components (faced into double-faced, ballistic/boulder, cordyceps/infected, marit/lage, nurgle, coil) rather than declare unrelated senses. Tests add coverage only. The trial free Name/Sword produced a forbidden covered-face increase on Shield of Kaldra, 3 → 5, in the initial bounded run (`initial-diff.json`, measured on `nqnztuzooxpwynzzurywltuwmpsxpory`); that lexicon declaration was retired, no new grammar route existed to retire, and final Shield of Kaldra is restored to three. Tuktuk the Returned replaced it after failing-original/positive-control probes.

   **Follow-up — unavailable declaration paths (V3/V7).** Counts in this list are the supplied blocker ranking on `vwlkmvnsnpumsrysspklpnslppzxurlv`, not claim-parent gains. The six W6 exclusions name both owner and missing path:

   - convoked (6), devoured (6), banded (3): `plugins_v2/builtin/macros/keyword_abilities/{convoke,devour,banding}.ron` own FixedKeyword declarations; no secondary Verb export supports their finite/event/passive uses. The participial-adjective supplement cannot express those Verb uses.
   - young (5), royal (5), cursed (6): Young Hero, Royal and Cursed Role tokens; `plugins_v2/builtin/macros/subtypes/enchantment/role.ron` exports Role alone. Individual Role-token definitions and exports are absent; free core adjectives would bypass the owner.
   - poisoned (5): `plugins_v2/builtin/macros/counter_kinds/poison.ron`, FixedTerm without secondary Adjective export.
   - evolves (2): `plugins_v2/builtin/macros/keyword_abilities/evolve.ron`, FixedKeyword without secondary Verb export.
   - unprepared (2): `plugins_v2/builtin/macros/designations/prepared.ron`, prepared Adjective without negative-state variant/export.
   - tempted (2): `plugins_v2/builtin/macros/keyword_actions/theRingTemptsYou.ron`, FixedClause without independently exported Verb/perfect form.
   - phased (2): no phase-out KeywordAction lexical definition; the related `instructions/phaseOutUntil.ron` is outside the eight lexical-export families. Verb/phrasal and hyphenated adjective paths are absent.
   - character (6), select (6), father (2), son (2): labelled partner variants, owned by `keyword_abilities/partner.ron`; FixedKeyword lacks a labelled PartnerVariant export. The existing Job Select keyword is a separate owned sense.

   **Follow-up — scoped names / covered ambiguity (V5).**

   - mowu (2), voja (3), ragavan (2), and optional one-face avacyn, smaug, galactus: already-scoped shortened CardNames in `data/gen/catalogs/card-name-nicknames.json`; need a shared owner or named-token-only alias path, rather than duplicate global Names.
   - sword (1): proven covered ambiguity on Shield of Kaldra as recorded above. A scoped token-name path must preserve its three existing Readings. Feather is already a parent-known counter word; its named-artifact sense also overlaps a scoped CardName.

   **Follow-up — missing constructions or other vocabulary (V2/V6).**

   - finish (3): gerund complement in finish voting; begin voting substitution also unread. part (3): bare count noun in as part of a gerund phrase; ordinary count-noun substitution also unread. worth (5): paw-amount/worth-of measurement assembly; damage substitution also unread.
   - none (3): plural-compatible determiner-head partitive/concord; the reading Each-of control uses singular concord and cannot express none of them were / none of the abilities have.
   - continuously (6): tested perfect and finite object-gap adjunct positions remain unread with a declared adverb substitution. below (2): scalar complement/intransitive postmodifier; under controls unread. higher (2): bare numeric or-comparative extent; greater controls unread.
   - just (3), directly (2): focusing PP modifiers; declared-adverb controls unread. evenly (2): tested divided participial predicate lacks its selected Among frame and rounding supplement. originally (3): pre-participial modifier inside a bare passive nominal; declared-adverb controls unread.
   - ultimate (2), lightning (2), wings (2): caption/flavour or scoped-name uses require their own route; not free vocabulary declarations.
   - Remaining uses of declared words include fewest fused head, foe in bare friend-or-foe coordination, bid-of scalar complement, and recent in the unparsed perfect-frame context; the table’s declaration is for the attested units that already have a matching route. Already-known Instance and Feather were skipped. Every batch-1 follow-up and concurrent-ticket-owned word in the session brief was skipped.

5. **Tests** — measured on `nqnztuzooxpwynzzurywltuwmpsxpory`: all four full suites and clippy passed (English 732 tests, lexical-source 75, construction-core 468, semantics-v2 104; logs `test-deckmaste_english_v3-roles.log`, `test-deckmaste_lexical_source-fixed.log`, `test-deckmaste_construction_core-fixed.log`, `test-deckmaste_semantics_v2.log`, `clippy-fixed.log`). Commands: `cargo test -p deckmaste_english_v3`; `cargo test -p deckmaste_lexical_source`; `cargo test -p deckmaste_construction_core`; `cargo test -p deckmaste_semantics_v2`; `cargo clippy -p deckmaste_english_v3 -p deckmaste_lexical_source -p deckmaste_construction_core -p deckmaste_semantics_v2 --tests -- -D warnings`. Restored / re-spelled / ignored-with-blocker / added / removed: 0 / 3 / 0 / 5 / 0 (one table-driven English test with 211 rows and four lexical-source tests). The three re-spelled tests are `type_words::declared_type_words_preserve_complete_independent_noun_paradigms`, `builtin_v2_noncreature_subtypes_preserve_declared_spelling_and_category`, and `noncreature_subtype_surfaces_preserve_attested_number_boundaries`: their exact Jace paradigms now include Jaces (and derived non-Jaces where applicable), retaining all value comparisons and singular/boundary checks. Initial runs exposed those obsolete expectations and fixture role errors; full suites were rerun after corrections, with no Reading-count assertion removed or weakened. No test removal. `cargo xtask facts check` passed; `cargo xtask cite check --list-noncompliant` reports zero, `cargo xtask cite check` reports zero stale; `jj --no-pager diff --git | cargo xtask cite audit --diff` passed, no added or changed rule citations (`cite-audit.log`). No new CR citations to bless. No Lean checks or xtask tests were needed.

6. **Timing** — measured on `nqnztuzooxpwynzzurywltuwmpsxpory`: final bounded census command 8 minutes 52.51 seconds wall time (including Cargo build-lock waiting) under `/usr/bin/time -v`, six workers, zero samples per face (`after-census.log`). No full-corpus census was run; the orchestrator owns that batch check. The initial bounded run exposed the Sword ambiguity; a later W3 review corrected Fewest to AttributiveAdjective with its quantitative feature retained. Both fixes justified bounded reruns.
