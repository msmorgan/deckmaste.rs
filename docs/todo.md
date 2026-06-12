# Modern-support TODO catalog

2026-06-10. The gap between what the engine, grammar, and card pipeline support
today and what *every Modern-legal card and mechanic* needs. Census source: the
local MTGJSON snapshot (22,050 distinct Modern-legal card names) intersected
with the Scryfall keyword catalogs; rules references are the CR snapshot in
`data/rules/`. "Cards" columns count distinct Modern-legal card names that use
a mechanic — use them to prioritize. Mechanic names only in this census; the
policy on committing real cards (the ~100–200-card canon slice, hand-written
edge cases) is `card-data.md`.

Already in place, for orientation: the full turn/priority/stack loop, casting
with mana payment and targets, zone-change pipeline with LKI, core combat with
seven native keywords (the true intrinsics first/double strike, deathtouch,
trample, plus flying, vigilance, lifelink), layers 4–7 with timestamps and CDAs, zone-move /
step / attacking triggers, a handful of SBAs, the core grammar on a combined
`SupportsMacros` derive, and an extract→resolve→graduate card pipeline with
mana/keyword/spell/triggered parsers.

## Priorities

When picking "the next" item, work down this ordering: take the highest tier
that has an unclaimed, non-conflicting item; within a tier, use the "Cards"
counts. This is an ordering, not an exclusion list, and it is not exhaustive —
anything unlisted (e.g. §10 format/runner) ranks below these unless the user
says otherwise.

0. ~~**Skill alignment, core-first**~~ — **CLOSED 2026-06-12** (all eight
   waves done; meter: docs/conformance.md). What remains of it lives in
   two named backlogs, picked up under the priorities below: the SEAM
   inventory (`rg 'todo!\("P0\.' crates/` — convert to behavior, mostly
   priority 1) and the post-P0 GRAMMAR backlog (conformance rows tagged
   "post-P0 grammar backlog" — each needs a design dialogue first).
1. **Engine happy path** — the engine supports the normal resolution path of
   ~90% of MTG abilities (§3, plus whichever §2 grammar that path needs).
2. **Oracle-text coverage** — extraction and parsers graduate an increasing
   subset of oracle text (§9 parsers, §4 layouts).
3. **Keyword authoring** — keyword abilities, keyword actions, and ability
   words get real macro bodies (§6–§8).
4. **Convenience macros** — shared macros for common mechanics (intertwined
   with 2 and 3).
5. **Noncanon tests** — keep the noncanon suite growing alongside engine work.
6. **Performance** — optimization passes.

## How to claim an item (instructions for agents)

When starting work on a todo item:

1. **Make a new jj change in `default@` marking the item in-progress.**
   In the default workspace: `jj new`, then tick the item's checkbox from
   `- [ ]` to `- [x]`; for a table row (no checkbox), prefix its first cell
   with `WIP:`. Then `jj describe -m "todo: claim <slug>"`.
2. **Bookmark that change:** `jj bookmark create <slug> -r @`.
3. **Start a new workspace** on that bookmark for the actual work:
   `jj workspace add --name <slug> -r <slug> ../deckmaste.<slug>`, then do the
   shared-dir setup from CLAUDE.md ("New jj workspaces") before building.

A ticked box / `WIP:` prefix means claimed and in progress — pick the
highest-priority (see Priorities) unclaimed item that doesn't conflict with
the claimed ones (same files, same engine subsystem, or one item's right
column naming the other). Claim bookmarks carry
the item's slug, so `jj bookmark list` shows what's active. Items leave this
file only when their work lands in the default line.

Bullets carry an explicit `slug`; for table rows derive it from the mechanic
name (`kw-…` for keyword abilities, `ka-…` for keyword actions, `aw-…` for
ability words), e.g. `kw-split-second`. Items tagged **[design]** require a
design dialogue with the user before implementation — claiming one means
opening that conversation, not coding solo. All CLAUDE.md jj constraints apply
in full.

## 1. Parked work to integrate

- [ ] `tokens-design` — token extraction parked twice (`tokens-shelved`,
  `tokens-revived` bookmarks). Blocked on a keyword-defs design the user has
  not yet approved. **[design]**

## 2. Core grammar gaps

- [ ] `core-loyalty-costs` — a loyalty-cost component (+N / −N / −X) for activated
  abilities; prerequisite for 1,512 planeswalker faces.
- [ ] `core-alt-costs` — the SHAPES landed in P0.W2 (`AlternativeCost` on
  `May(Cast(cost: …))`, `CostChange::Additional{optional}`); remaining: the
  keyword macros over them (flashback, evoke, overload, madness, kicker)
  and "if the X cost was paid" linkage. **[design]**
- [ ] `core-card-shapes` — `Card` variants beyond `Normal`/`ModalDfc` (see §4).
- [ ] `core-copy-grammar` — copy effects: `CopySpell` (copy-on-stack) landed
  P0.W4; remaining: cast-a-copy ([CR#707.12], decides UD-4), token copies,
  enters-as-a-copy, "becomes a copy of" (layer 1 input).
- [ ] `core-emblems` — `GetEmblem(Vec<Ability>)` + `ObjectKind::Emblem`
  landed in P0.W5; remaining: engine minting (the P0.W5 resolve seam) and
  the parser/macro story for "you get an emblem with …".
- [ ] `core-saga-chapters` — chapter-ability structure (I/II/III markers, ranges,
  read-ahead compatibility).
- [ ] `core-as-enters-choices` — "as this enters, choose …" (color/type/number/
  opponent) with stored, linked choices readable by other abilities.
- [ ] `core-casting-restrictions` — "can't be countered" and split-second-style
  stack lockout; "cast only …" timing landed in P0.W1
  (`DeonticAction::Cast{window}` — consumption is the engine seam).
- [ ] `core-count-query` — `Count::Query(QueryKey)` landed in P0.W4
  (CardsDrawn/LandsPlayed evaluate live; StormCount is a seam); remaining:
  more keys as cards force them, and aggregate sums (devotion-style) still
  unsolved. **[design]**
- [ ] `core-intrinsic-keywords-policy` — which keywords graduate from plugin
  macros to intrinsic `KeywordAbility` variants, and the template-param story
  for parameterized ones (ward, protection, typed cycling). **[design]**

## 3. Engine machinery

### Resolution breadth (`crates/deckmaste_engine/src/resolve.rs` todo!s)

- [x] `engine-resolve-playeractions` — resolve the remaining `PlayerAction`s:
  GainLife, Discard, AddMana, Create, Sacrifice, Exile, Untap, PutInLibrary
  (verb landed, resolution still todo).
- [ ] `engine-resolve-actions` — `Destroy` resolves since P0.W7 (cause-tagged,
  DIRECT — no replacement window); remaining: the `WillDestroy` intent for
  regeneration/indestructible (converts the kw-indestructible sweep
  guard), `ReturnToHand`, and `Counter` ([CR#701.6a], ward's verb).
- [ ] `engine-resolve-effects` — May, If/Unless, ForEach, Modal, Delayed,
  Reflexive effect frames; resolution-time choices surfaced as decisions.
- [ ] `engine-resolve-counts` — X, CountOf(Filter), StatOf, ThatMuch.
- [ ] `engine-resolve-selections` — Choose/Random selections as surfaced
  decisions; remaining `Reference` variants (Bound, Linked, ControllerOf,
  OwnerOf, attachment refs).
- [ ] `engine-filter-breadth` — evaluate Named, Stat, Relation (controller/owner/
  opponent/attached), StateFilter (Status, RelatedBy, Targets/TargetCount),
  and Ref(Reference) filters (`target.rs`, `trigger.rs` snapshot matching).
  `HasCounter` and `Designated` read live since P0.W5/W6.

### Triggers and conditions (`trigger.rs`)

- [ ] `engine-trigger-events` — match the remaining event shapes: CAUSE-PATTERN
  matching (the P0.W3 seam — unlocks the sacrificed/discarded/played/
  `Destroyed` named views, whose emitters all carry causes now), Performed,
  DamageDealt, spell-cast, becomes-tapped/untapped, becomes-blocked (needs
  once-per-attacker dedup), becomes-targeted (prerequisite for ward/heroic),
  OneOf, and the new becomes-deltas (phased/turned-face/designated/
  controlled-by).
- [ ] `engine-trigger-conditions` — Condition::Is, Compare, Happened; intervening
  "if" rechecked on resolution [CR#603.4].
- [ ] `engine-trigger-limits` — OncePerTurn and friends.
- [ ] `engine-history-tallies` — turn/game history the condition layer can query:
  spells cast this turn (storm), lands played, life lost/gained, permanents
  that left, "entered this turn", attack/cast ordinals. The Tallies registry
  exists; wire it broadly and extend `Window` beyond ThisTurn.

### Layers and continuous effects (`layer.rs` seams)

- [ ] `engine-layers-1-3` — layer 1 copy, layer 2 control change, layer 3 text
  change; face-down characteristics; dependency ordering [CR#613.8].
- [ ] `engine-layers-misc` — condition evaluation on static abilities, Of/These
  scope resolution (currently locked empty), subtype changes in layer 4 with
  confers data (incl. changeling's `AllCreatureTypes` fill — the
  kw-changeling seam), become-basic-land-type, base loyalty/defense.

### SBAs and counters (`sba.rs`)

- [ ] `engine-sba-breadth` — the remaining [CR#704.5] sweeps (poison landed in
  P0.W6, live off the proxy counter map): toughness ≤ 0, loyalty 0, illegal
  auras, unattached-equipment legality, legend rule, battle with no defense,
  spell-copy cleanup. (Token-ceases already sweeps.)
- [ ] `engine-counters-api` — apply the counter events (verbs/events landed in
  P0.W3; storage already lives on objects, players via their proxy);
  enters-with-counters; +1/+1 vs −1/−1 annihilation SBA; keyword-counter
  payload registry [CR#122.1] (stun, shield, finality, flying, …).

### Replacements and prevention (`replace.rs`)

- [ ] `engine-replacements` — a general replacement registry beyond enters-tapped:
  Instead/Skip/Also, zone-destination redirects, enters-with-counters,
  enters-as-choices, draw replacements (dredge/miracle window), dies→exile,
  lifegain/damage modification, self-replacement ordering.
- [ ] `engine-prevention` — prevention shields and windows [CR#615.1], including
  "can't be prevented".

### Costs and casting (`cast.rs`, `legal.rs`)

- [ ] `engine-deontic-polarities` — evaluate the deontic rows still behind
  the narrowed presence guards: `May(Cast(window: …))` lifts (flash
  [CR#702.8a] — the LOUDEST guard now: any flash permanent on the
  battlefield trips `legal_actions` at every priority window; convert in
  `can_cast` first), `Must(Attack/Block)` requirements (goad, must-block),
  non-Cant `Target` rows, and the announce-time targeting-object seam (an
  activated ability's stack identity isn't minted at announce — its
  source stands in; stack-zone-keyed `by` rows need the real id).

- [ ] `engine-x-costs` — X announcement, X in effects, zero-X edge cases.
- [ ] `engine-cost-payment` — hybrid/Phyrexian payment choices, snow, life
  payment, non-mana additional costs (sacrifice/discard/exile/tap others).
- [ ] `engine-cost-modification` — apply CostModifier statics in the cost
  calculation pipeline [CR#601.2f] (affinity, convoke, improvise, delve,
  reducers/taxers).
- [ ] `engine-alt-costs` — choosing an alternative cost at announcement; "without
  paying its mana cost" (cascade, suspend, plot).
- [x] `engine-activated-abilities` — activating non-mana abilities (only mana
  abilities are legal today, `legal.rs`): general costs, timing restrictions
  ("only as a sorcery", once-per-turn, once-per-game), loyalty abilities.
- [ ] `engine-mana-system` — mana pool provenance/riders (spend-only-on
  restrictions, doesn't-empty), triggered mana abilities, conditional
  production.
- [ ] `engine-cast-from-zones` — casting from graveyard/exile/library via
  permissions; linked "the exiled card" access; timing windows (miracle,
  forecast).

### Zones and objects

- [ ] `engine-attach` — the attachment relation behind the `Attach` verb's
  seam: storage on objects, `Attach` resolution ([CR#701.3a..701.3b] —
  no-op on illegal/duplicate attach), enchant/equip attach-legality via
  the `Cant(Attach)` rows, and the illegal-attachment sweeps
  ([CR#704.5m..704.5n]); takes Equip and Enchant end-to-end with the
  Aura-targeting half already live.

- [ ] `engine-exile-command` — wire Exile (face-up/face-down, with counters,
  linked retrieval) and Command zones as zone-pipeline sources/destinations.
- [x] `engine-tokens` — resolve Create; token objects on the battlefield; token
  copies; predefined tokens (Treasure, Food, Clue, Blood, Map, Powerstone,
  Incubator, Role); token SBA on zone leave.
- [ ] `engine-copy-spells` — spell copies on the stack with target re-choice
  (storm, replicate, casualty, conspire, magecraft sources).
- [ ] `engine-attachments` — attach/unattach state, enchant/equip legality
  checks, aura cast targeting, attach SBAs, equip/reconfigure/fortify
  activation.
- [ ] `engine-face-down` — face-down spells and permanents (morph/disguise/
  manifest/cloak), turn-face-up special action, face-down exile.
- [ ] `engine-planeswalkers` — loyalty enters/activation/SBA, attacking
  planeswalkers (attack-target choice), compleated.
- [ ] `engine-battles` — defense counters, protector designation, attacking
  battles, defeated trigger + back-face casting.
- [ ] `engine-sagas` — lore counters, chapter firing, final-chapter sacrifice,
  read-ahead.
- [ ] `engine-transform` — transforming DFC permanents (front/back face state,
  transforms-with-day-night, meld pairs).
- [ ] `engine-phasing` — phasing status and untap-step phasing events.

### Combat

- [ ] `engine-combat-restrictions` — enforce evasion/restriction statics in
  attack/block legality: flying/reach, menace, fear/intimidate/shadow/skulk,
  landwalk, defender, "can't attack/block", protection-from in blocking.
- [ ] `engine-combat-requirements` — must-attack/must-block requirements, goad,
  attack taxes/costs.
- [ ] `engine-multi-defender` — attack targets per attacker (player, planeswalker,
  battle) and per-defender damage routing.

### Turn, game, determinism

- [ ] `engine-turn-modification` — extra turns, extra combat/main phases, skip
  steps/phases, "end the turn".
- [ ] `engine-win-alterations` — can't-lose/can't-win statics, alternate win/loss
  effects, restart-the-game, controlling another player's turn. **[design]**
- [ ] `engine-randomness` — seeded shuffle, coin flips, random discard; keep runs
  reproducible for the sim harness.

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

## 5. Game-wide systems

- [ ] `noncanon-keyword-decks` — grow the noncanon matchup decks with the
  now-live keywords (fliers/reach, menace, defender, hexproof) so the
  50-game gate regression-tests keyword interactions systemically, not
  just per-test. Lives in the noncanon workspace/feature.

- [ ] `engine-day-night` — day/night game state, daybound/nightbound transforms
  (~236 cards).
- [ ] `engine-dungeons` — dungeon objects outside the game, venture, room
  progression, completion (76 cards).
- [ ] `engine-energy` — {E} energy counters on players (105 cards).
- [ ] `engine-ring` — the Ring emblem and tempts-you progression (50 cards).
- [ ] `engine-speed` — speed counters, start-your-engines, max speed (~97 cards).
- [ ] `engine-citys-blessing` — ascend / city's blessing designation (71 cards).
- [ ] `engine-poison` — poison counters, infect/toxic/corrupted hooks, poison SBA
  (~190 cards).
- [ ] `engine-party` — party count condition (5 cards).

## 6. Keyword abilities

Per-keyword work = macro body (stub exists under `plugins/wizards/macros/`),
any engine machinery (right column names the §3 item it rides on), parser
coverage, and graduating its cards. Slug: `kw-<kebab-case>`.

| Keyword | Cards | Machinery |
|---|---|---|
| Enchant | 2,176 | aura targeting + attach (`engine-attachments`) |
| Haste | 1,584 | summoning-sickness bypass |
| Flash | 1,424 | timing permission (grammar exists; wire `legal.rs`) |
| Equip | 1,381 | activated attach, sorcery timing (`engine-attachments`) |
| Cycling (+ typed/land variants) | ~1,366 | activated from hand, discard cost; typed variants search |
| Menace | 843 | block-count restriction (`engine-combat-restrictions`) |
| Reach | 829 | flying-blocking permission |
| Defender | 696 | attack restriction |
| Flashback | 576 | graveyard alt cost + exile replacement |
| Kicker / Multikicker | 502 | additional cost + "if kicked" linkage |
| Ward | 475 | becomes-targeted trigger + tax (`engine-trigger-events`) |
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
| Exalted | 90 | attacks-alone trigger |
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
| Decayed | 4 | can't block; sacrifice after attack |
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

- [ ] `aw-prefix-parsing` — strip "Ability word —" prefixes during extraction and
  preserve them for rendering.

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

## 9. Pipeline and parsers

- [ ] `parser-keyword-arg-breadth` — keyword-line argument shapes beyond
  single quality words and pure-mana costs: compound qualities
  ("protection from everything", "from artifacts", multi-quality "from
  red and from white" [CR#702.16g]), non-mana keyword costs
  ("Ward—Pay 3 life.", "Cycling—Discard a card."), and variable costs
  ("Ward {X}").
- [ ] `macro-bare-defaulted-invocations` — let a bare macro name read as a
  zero-arg invocation when every param is defaulted (`Hexproof` vs
  today's required `Hexproof()`); needs a probe-free path through serde's
  one-shot `VariantAccess` (or a macro-layer pre-scan), which is why the
  parser currently owns the convention instead.
- [ ] `cards-keyword-name-lint` — the punted post-parse validator: a
  `Filter` visitor in `validate.rs` (beside `lint_card_subtypes`)
  checking `Has`/keyword-reference names against the keyword namespace
  (native enum ∪ `KeywordAbility`-kind macros). Bar: "doesn't assert
  nonsense" — benign extras are ignored by design.

- [x] `parse-activated` — activated-ability frame (`cost: effect`), including the
  cost grammar; registry slot exists, parser doesn't.
- [ ] `parse-static` — static-prose abilities (gets/has/can't sentences) into
  Continuously/StaticEffect.
- [ ] `parse-replacement` — "if … would …, instead …" / "as … enters" /
  "… enters tapped" templates.
- [ ] `parse-modal` — "Choose one —" bullet lists, escalate/spree-style modal
  costs.
- [ ] `parse-filters` — natural-language object descriptions to Filter ASTs
  beyond the current handful (control/zone/type/stat qualifiers).
- [ ] `macro-keyword-templates` — template parameters for keyword macros
  (ward cost, protection quality, typed cycling) so the ~190 keyword stubs
  can expand per-card.
- [x] `macro-subtype-params` — subtype macro registers under its printed-string
  argument, not its filename; parametric subtype refs miss it and their cards
  stall as todos. Needs the template-param refactor.
- [ ] `macro-keyword-actions` — implement the 66 keyword-action macro stubs over
  engine primitives.
- [ ] `pipeline-fixpoint` — dependency-ordered re-graduation (subtypes → keywords
  → cards) instead of today's single pass.
- [ ] `pipeline-layout-extraction` — extract layouts beyond normal/modal_dfc
  (see §4).
- [ ] `canon-slice` — grow `plugins/canon/` toward the ~100–200-card mechanics
  slice (`card-data.md`): as each mechanic lands, graduate a few real cards
  exercising it; hand-write (and mark) the genuinely unparseable ones.
- [ ] `macro-param-validation` — type-check macro params (Color, Filter, …) at
  the graduate gate.
- [ ] `tokens-templates` — token definitions (predefined + ad-hoc) and granted
  quoted abilities; resume from the parked bookmarks (§1). **[design]**

## 10. Format and runner

- [ ] `format-deck-validation` — Modern deck legality: banlist via mtgjson
  legalities (derived data only), 4-of rule, sideboard size.
- [ ] `runner-outside-game` — sideboard/outside-the-game access (companion,
  Lesson fetch, wish-style effects).
- [ ] `runner-hidden-info` — per-player redaction stays a runner-layer projection
  by design (core is full-information). The face-down mechanics in §6 are the
  forcing function; revisit the boundary then. **[design]**
