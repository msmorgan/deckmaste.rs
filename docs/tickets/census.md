# Census tables

## 4. Card shapes (layouts)

Extraction currently reads `normal` and `modal_dfc` only; each row needs
grammar (§2 `core-card-shapes`), extraction, and engine behavior. Slug:
`shape-<layout>`.

| Layout | Modern cards | Work |
|---|---|---|
| transform | 365 | two faces + transform lifecycle (`engine-transform`) |
| saga | 106 | chapters (`core-saga-chapters`, `engine-sagas`) |
| modal_dfc | 93 | grammar done; engine play-either-face + back-face casting rules |
| adventure | 102 | second spell face + exile-then-cast-creature state |
| split | 80 | two halves; fuse; characteristics on the stack |
| reversible_card | 56 | cosmetic duplicate — normalize to the real layout |
| prepare | 36 | paired prepare/instant face (TLA) |
| mutate | 30 | merged-permanent stacks **[design]** |
| class | 27 | level designations + paid level-up statics |
| aftermath | 26 | split with graveyard-castable half |
| leveler | 25 | level counters + level-band characteristics |
| meld | 21 | two cards → one melded back face |
| flip | 20 | single card, flipped half (Kamigawa style) |
| prototype | 19 | alternate cost/characteristics while casting |
| case | 12 | Case enchantments: solve condition + solved state |

## 6. Keyword abilities

Per-keyword work = macro body (stub exists under `plugins/wizards/macros/`),
any engine machinery (right column names the §3 item it rides on), parser
coverage, and graduating its cards. Slug: `kw-<kebab-case>`.

| Keyword | Cards | Machinery |
|---|---|---|
| Enchant | 2,176 | aura targeting + attach (`engine-attach`) |
| Haste | 1,584 | summoning-sickness bypass |
| Flash | 1,424 | timing permission (grammar exists; wire `legal.rs`) |
| Equip | 1,381 | activated attach, sorcery timing (`engine-attach`) |
| Cycling (+ typed/land variants) | ~1,366 | activated from hand, discard cost; typed variants search |
| Menace | 843 | block-count restriction (`engine-combat-restrictions`) |
| Reach | 829 | flying-blocking permission |
| Defender | 696 | attack restriction |
| Flashback | 576 | graveyard alt cost + exile replacement |
| Kicker / Multikicker | 502 | additional cost + "if kicked" linkage |
| Ward | 475 | trigger half live; tax = Unless/Counter (`engine-resolve-effects`) |
| Indestructible | 468 | destroy immunity (SBA + Destroy) |
| Crew | 420 | tap-creatures cost; vehicle animation |
| Protection | 380 | bundle: target/enchant/block/damage immunity |
| Morph / Megamorph / Disguise | 319 | face-down system (`engine-face-down`) |
| Hexproof (+ "hexproof from") | 302 | opponent-only target restriction |
| Landwalk family | 239 | conditional unblockability |
| Daybound / Nightbound | 236 | `engine-day-night` |
| Prowess | 220 | noncreature-cast trigger |
| Changeling | 207 | all-creature-types CDA |
| Devoid | 206 | colorless CDA |
| Suspend | 195 | exile + time counters + delayed free cast |
| Aftermath | 188 | `shape-aftermath` |
| Madness | 181 | discard replacement → exile window + alt cost |
| Affinity | 174 | cost reduction (`engine-cost-modification`) |
| Evoke | 166 | alt cost + sacrifice trigger |
| Ninjutsu | 148 | special action: swap unblocked attacker |
| Disturb | 146 | cast back face from graveyard |
| Mutate | 140 | merged permanents (`shape-mutate`) **[design]** |
| Unearth | 135 | reanimate + exile-at-end replacement |
| Overload | 104 | alt cost + target→each text change (layer 3) |
| Fear | 103 | block restriction |
| Cascade | 94 | cast trigger: exile until cheaper, free cast |
| Infect | 92 | damage as counters/poison (`engine-poison`) |
| Foretell | 91 | exile face down + later alt cost |
| Exalted | 90 | attacks-alone trigger; add ExaltedCounter keyword counter once the keyword lands |
| Storm | 87 | stack copies × cast tally (`engine-copy-spells`) |
| Hideaway | 87 | ETB exile face down + linked play permission |
| Exhaust | 85 | once-per-game activation |
| Bestow | 81 | aura-or-creature dual cast + fallback |
| Station | 77 | tap-creatures: charge counters |
| Max speed | 77 | `engine-speed` |
| Warp | 76 | temporary cast + later recast from exile |
| Rebound | 76 | resolve-replacement: exile + delayed recast |
| Fuse | 74 | cast both halves (`shape-split`) |
| Annihilator | 74 | attack trigger: defender sacrifices |
| Delve | 72 | exile from graveyard as payment |
| Ascend | 71 | `engine-citys-blessing` |
| Dredge | 69 | draw replacement |
| Saddle | 68 | crew-alike for Mounts |
| Echo | 67 | upkeep echo cost |
| Shroud | 66 | target restriction (everyone) |
| Escape | 66 | graveyard alt cost + exile fuel |
| Companion | 66 | deck constraint + outside-game (`runner-outside-game`) |
| Toxic | 65 | poison on combat damage |
| Persist | 62 | dies-return with −1/−1 counter |
| Level Up | 62 | `shape-leveler` |
| Entwine | 62 | all modes for extra cost |
| Bloodthirst | 62 | conditional ETB counters |
| Split second | 61 | stack-wide cast lockout (`core-casting-restrictions`) |
| Improvise | 60 | artifacts help pay |
| Sneak | 59 | (FIN) alt cost |
| Firebending | 59 | (TLA) |
| Living weapon | 57 | ETB token + attach |
| Extort | 56 | cast trigger drain |
| Exploit | 56 | ETB may-sacrifice + trigger |
| Compleated | 56 | Phyrexian loyalty payment |
| Modular | 55 | ETB counters + dies-move-counters |
| Intimidate | 54 | block restriction |
| Splice | 52 | add effect text to a spell for a cost |
| Undying | 51 | dies-return with +1/+1 counter |
| Dash | 51 | alt cost + haste + return at end |
| Reconfigure | 50 | self attach/unattach |
| Mentor | 50 | attack trigger counter |
| Evolve | 50 | bigger-creature-ETB counter |
| Cleave | 50 | alt cost removing clause |
| Bushido | 49 | blocks/blocked pump |
| Spree | 48 | modal with per-mode costs |
| Craft | 48 | exile materials + transform return |
| Vanishing | 47 | time counters + sacrifice |
| Shadow | 47 | evasion class |
| Renown | 47 | combat damage → renowned + counters |
| Prototype | 45 | `shape-prototype` |
| Embalm | 45 | graveyard exile: token copy |
| Wither | 43 | damage as −1/−1 counters |
| Retrace | 43 | recast from graveyard + discard land |
| Outlast | 43 | tap + counter activated |
| Gift | 43 | (BLB) extra-cost promise + opponent reward |
| Umbra armor | 42 | destroy-replacement on enchanted |
| Fabricate | 42 | ETB choice: counters or tokens |
| Eternalize | 42 | graveyard exile: 4/4 token copy |
| Emerge | 42 | alt cost via sacrifice |
| Soulbond | 41 | pairing designation |
| Offspring | 41 | extra cost → 1/1 token copy |
| Cumulative upkeep | 41 | age counters + growing cost |
| Graft | 39 | counters migrate on others' ETB |
| Devour | 39 | ETB sacrifice × counters |
| Boast | 39 | once-per-turn activation if attacked |
| Sunburst | 38 | colors-spent memory |
| Transmute | 37 | discard: tutor same mana value |
| Miracle | 36 | first-draw reveal window + alt cost |
| Bargain | 36 | extra sacrifice cost + condition |
| Awaken | 36 | alt cost: land animation rider |
| Escalate | 35 | per-extra-mode cost |
| Backup | 35 | ETB counters + ability grant |
| Soulshift | 34 | dies: return Spirit |
| Mayhem | 34 | (FIN) cast from graveyard after discard |
| Training | 32 | attacks-with-stronger counter |
| Skulk | 32 | power-based block restriction |
| Blitz | 32 | alt cost: haste, sacrifice, dies-draw |
| Spectacle | 31 | alt cost if opponent lost life |
| Unleash | 30 | counter choice; can't block rider |
| Replicate | 30 | paid stack copies |
| Impending | 30 | time counters defer creature-ness |
| Battle Cry | 30 | attack pump others |
| Afterlife | 30 | dies: Spirit tokens |
| Web-slinging | 27 | (SPM) alt cost: return tapped creature |
| Jump-start | 27 | flashback + discard |
| Casualty | 27 | sacrifice → copy |
| Surge | 26 | alt cost if prior spell this turn |
| Scavenge | 26 | graveyard exile: counters |
| Buyback | 26 | extra cost → return to hand on resolution |
| Riot | 25 | ETB choice: counter or haste |
| Afflict | 25 | becomes-blocked life loss |
| Mobilize | 24 | attack: tapped token attackers |
| Flanking | 24 | blocked-by-nonflanker debuff |
| Champion | 24 | ETB exile linked, return on leave |
| Solved | 23 | `shape-case` |
| Forecast | 23 | activate from hand during upkeep |
| Job select | 22 | (FIN) ETB Hero token + attach |
| Cipher | 22 | encode on creature + linked recast |
| Read Ahead | 21 | saga starting chapter |
| Freerunning | 21 | conditional alt cost |
| Reinforce | 20 | discard: counters |
| For Mirrodin! | 20 | ETB token + attach |
| Enlist | 19 | tap helper to add power |
| Prowl | 18 | type-conditional alt cost |
| Haunt | 17 | exile haunting + linked trigger |
| Harmonize | 17 | (TDM) graveyard cast, tap-creature reduction |
| Tribute | 15 | opponent choice: counters or trigger |
| Paradigm | 15 | (FIN) |
| Increment | 15 | (FIN) |
| Conspire | 15 | tap two → copy |
| Ingest | 13 | combat damage exiles top card |
| Partner / Partner with | 12 | paired tutor trigger |
| Offering | 11 | sacrifice for timing + cost break |
| Recover | 9 | creature-dies: pay or exile |
| Ripple | 7 | reveal top N, free same-name casts |
| Epic | 7 | upkeep copies + cast lockout |
| Tiered | 6 | (FIN) modal cost tiers |
| Rampage | 5 | multi-block pump |
| Decayed | 4 | can't block; sacrifice after attack; add DecayedCounter keyword counter once the keyword lands |
| Wizardcycling | (in Cycling) | — |
| Fortify | 2 | equipment-for-lands |
| Transfigure | 1 | sacrifice: tutor same mana value |
| Aura Swap | 1 | exchange with aura in hand |
| Gravestorm | 1 | storm counting deaths |

## 7. Keyword actions

Mostly macros over engine primitives plus a few dedicated subsystems. Slug:
`ka-<kebab-case>`.

| Action | Cards | Machinery |
|---|---|---|
| Transform | 1,926 | `engine-transform` |
| Mill | 1,459 | library→graveyard primitive |
| Scry | 1,307 | look + reorder/bottom decision |
| Treasure | 789 | predefined token (`engine-tokens`) |
| Surveil | 455 | scry-to-graveyard |
| Fight | 373 | mutual damage |
| Double (counters/life/power) | 343 | counter/stat doubling |
| Investigate | 285 | Clue token |
| Food | 266 | predefined token |
| Proliferate | 224 | counter API (`engine-counters-api`) |
| Amass | 136 | Army token + counters |
| Prepared | 128 | `shape-prepare` (TLA) |
| Manifest / Manifest dread / Cloak | 194 | `engine-face-down` |
| Explore | 99 | reveal top, counter-or-graveyard choice |
| Plot | 81 | exile + later free cast |
| Venture into the dungeon | 76 | `engine-dungeons` |
| Role token | 67 | Role aura tokens + one-per-controller SBA |
| Incubate | 63 | Incubator token + transform |
| Monstrosity | 61 | counters + monstrous flag |
| Connive | 61 | draw-discard + conditional counter |
| Exert | 59 | skip-untap rider |
| Earthbend | 57 | (TLA) counters on lands |
| Waterbend | 55 | (TLA) |
| Collect evidence | 53 | exile from graveyard by total mana value |
| Adapt | 53 | conditional counters |
| Bolster | 50 | counters on weakest |
| Discover | 46 | exile until cheaper; cast free or to hand |
| Populate | 44 | token copy of a token |
| Learn | 43 | outside-game Lesson fetch (`runner-outside-game`) |
| Blight | 39 | (TLA) |
| Support | 37 | counters spread |
| Clash | 37 | reveal-compare-reorder |
| Behold | 36 | (TLA) reveal-or-have choice |
| Airbend | 30 | (TLA) |
| Detain | 26 | until-next-turn restriction |
| Meld | 24 | `shape-meld` |
| Suspect | 21 | suspected designation (menace, can't block) |
| Endure | 19 | counters or Spirit token choice |
| Forage | 14 | exile from graveyard or sacrifice Food |
| Triple | 12 | counter/stat tripling |
| Goad | 5 | attack requirement (`engine-combat-requirements`) |
| Fateseal | 5 | scry an opponent's library |
| Assemble | 3 | — |

## 8. Ability words

Ability words carry no rules weight; the work is (a) one umbrella parser item
and (b) the condition/history machinery the marked abilities lean on (mostly
`engine-history-tallies`, `engine-trigger-events`, `engine-trigger-conditions`).
Rows here track that the *patterns* graduate. Slug: `aw-<kebab-case>`.

| Ability word | Cards | Leans on |
|---|---|---|
| Landfall | 583 | land-ETB trigger (exists) |
| Delirium | 186 | graveyard card-type census |
| Domain | 106 | basic-land-type census |
| Start your engines! | 97 | `engine-speed` |
| Channel | 94 | discard-activated from hand |
| Magecraft | 88 | cast/copy trigger (`engine-copy-spells`) |
| Metalcraft | 86 | battlefield census condition |
| Heroic | 84 | becomes-targeted trigger |
| Constellation | 80 | enchantment-ETB trigger |
| Raid | 78 | attacked-this-turn history |
| Imprint | 75 | linked exile |
| Converge | 72 | colors-spent memory |
| Ferocious | 71 | power-threshold condition |
| Morbid | 65 | died-this-turn history |
| Revolt | 60 | left-battlefield history |
| Coven | 56 | distinct-powers condition |
| Hellbent | 48 | empty-hand condition |
| Alliance | 46 | creature-ETB trigger |
| Spell mastery | 45 | graveyard census |
| Enrage | 45 | dealt-damage trigger |
| Threshold | 40 | graveyard count |
| Bloodrush | 40 | discard-activated targeting attacker |
| Battalion | 36 | attacks-together trigger |
| Undergrowth | 35 | graveyard census |
| Descend | 31 | permanents-in-graveyard + history |
| Vivid | 30 | charge counters + any-color mana |
| Addendum | 30 | main-phase-cast condition |
| Corrupted | 29 | `engine-poison` |
| Strive | 28 | per-extra-target cost |
| Renew | 26 | exile from graveyard: counters |
| Eerie | 26 | enchantment-ETB/unlock trigger |
| Survival | 24 | second-main-phase condition |
| Void | 23 | left-battlefield/warped history |
| Pack tactics | 23 | attacking-power condition |
| Inspired | 23 | becomes-untapped trigger |
| Rally | 22 | ally-ETB trigger |
| Formidable | 20 | total-power condition |
| Celebration | 19 | two-nonland-ETB history |
| Adamant | 19 | mono-color-payment memory |
| Repartee | 18 | — |
| Infusion | 18 | — |
| Fathomless descent | 18 | graveyard census |
| Valiant | 17 | becomes-targeted trigger |
| Radiance | 16 | target + all sharing a color |
| Opus | 16 | (FIN) |
| Cohort | 16 | tap-two activated |
| Flurry | 15 | second-spell-cast trigger |
| Fateful hour | 14 | life-threshold condition |
| Kinship | 13 | top-reveal type share |
| Disappear | 12 | (TLA) |
| Grandeur | 11 | discard-same-name activated |
| Chroma | 11 | color-pip census |
| Sweep | 5 | return lands count |
