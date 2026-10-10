---
needs: []
---
# Report why each unread face is unread: `cargo xtask english-v3 blockers`

Priority: high. Tooling only; no grammar or lexicon change.

## Why

The census records nothing about an unread face except its unknown words:
`lines` is empty, and no failing line, sentence or position is recorded. Work
has therefore been chosen by regex recon, which left 3,417 unread faces in no
bucket (wave-5 recon, 2026-10-08) and can only find constructions somebody
already suspected. At the per-line baseline (`sskyxnyvmruk`, 32,828 supported
faces, 6,292 unread) 2,027 unread faces carry at least one unknown word, and
nobody knows how many of them are unread for that reason alone. Owner
decision 2026-10-10: build the report before minting more construction
tickets.

## Goal

A subcommand `cargo xtask english-v3 blockers --census FILE [--json OUT]
[--limit N]` that, for every unread face in a schema-8 census, says which
units fail and where, and ranks the causes.

1. **Per-line verdict.** Split the face's analysed source into its ability
   lines and analyse each line on its own, with the face's card scope (the
   same scope `card_scope::for_face` gives the census and `probe --as-card`).
   Record for each line: reads or not, Reading count if it reads, and its
   unknown words.
2. **Localisation of a failing line.** For a line that does not read and has
   no unknown word, record the furthest input position the chart reached
   (the largest token index at which any item was scanned or completed from
   the line start), with the token at that position and the one before it.
   If the chart does not expose that today, expose it read-only from the
   chart statistics; do not change parsing.
3. **Blocker key.** Each failing line gets one key: `word:<lemma or lowercased
   text>` for each unknown word, otherwise `at:<previous token> <failing
   token>` from item 2. Keys are data for ranking, not grammar.
4. **Ranking.** The summary lists blocker keys by the number of faces they
   touch and by the number of faces where they are the *sole* blocker (the
   face has exactly one failing line and that line has exactly one key), with
   up to three example faces each. It also prints: unread faces, faces with
   exactly one failing line, faces whose every failing line is
   vocabulary-only, and line-level coverage over the whole census (lines that
   read / lines).
5. **JSON.** `--json` writes the per-face records (face id and name, per-line
   verdicts, keys) and the ranked table, so tickets can cite it and a later
   run can be compared.

## Analysis

Orchestrator rulings 2026-10-10:

- B1. Reuse the census's own line analysis (`analyze_line` and the segment
  logic that `english-v3-per-line-readings` added); the report must agree
  with the census: a face it calls fully reading must be covered in the
  census, and a covered face has no failing line. Assert that on the run and
  fail loudly on a mismatch.
- B2. Line splitting for an unread face cannot come from a Document parse
  (there is none). Split on the analysed source's newlines; where the census
  segments a covered face differently (quoted multi-line abilities, modal
  bullet lists), record the disagreement count in the summary rather than
  special-casing by card.
- B3. The report is diagnostic. It adds no field to the census schema and no
  gate; nothing may read it to admit or reject a Reading.
- B4. No Readings are materialised beyond what a line's count needs; run with
  the same worker pool as the census and stay under the census's peak memory.

- B5 (review ruling, 2026-10-10). Report keys use word tokens: maximal runs
  of letters, digits, apostrophes and hyphens; brace groups are single tokens;
  other non-space characters are individual tokens. The token containing the
  furthest position fails; whitespace and boundaries select the next token.
  Lowercase keys, preserving numerals. Each failing line records two tokens
  before and after it as context. Print three separately ranked tables: top
  60 vocabulary keys, top 80 position keys, top 40 failing tokens (ignoring
  previous tokens), with all three complete in JSON. Parsing is unchanged.

- B6 (orchestrator ruling, 2026-10-10). B1 remains strict. The baseline was
  stale after refresh. Authorize one additional full census at the refreshed
  head, prebuilt binary, six workers, no samples, under `/usr/bin/time -v`;
  wait if `free -g` shows less than 40 GB available. Replace the baseline and
  reports, record the census change id, commit, refresh and integrate; no
  further census after that refresh. Leave the mismatch diagnostic unchanged
  unless distinguishing the binary's grammar revision is a small change.

## Witnesses

Probe-level expectations, each written as an xtask test on a synthetic
two- or three-line card (never a pinned corpus count):

- a face whose second line contains an undeclared word: line 1 reads, line 2
  fails with key `word:<that word>`, sole blocker;
- a face whose only line is declared vocabulary in an ungrammatical order:
  key `at:<prev> <token>` at the first token the chart cannot pass;
- a covered face: no failing line;
- two failing lines: listed under both keys, sole blocker for neither.

## Method

Standard constraints apply. Deltas:

1. No grammar, lexicon or census-schema change. `cargo test -p xtask
   english_v3` and `cargo clippy -p xtask --tests -- -D warnings`; if the
   chart statistic needs a field in `deckmaste_english_v3`, that crate's full
   suite and clippy too.
2. Run it once on a current full census. If none of the claim parent exists,
   run one `--all --workers 6 --samples-per-face 0` census under
   `/usr/bin/time -v` (about 4 minutes) and keep it at
   `/home/msmorgan/Projects/deckmaste.rs/target/english-v3/baseline-blockers.json`
   with the report beside it as `blockers.json`.
3. Tests assert relations (which line fails, which key, sole or not), never
   incidental numbers or Debug-printed trees.
4. Document the command in the same place `diff` and `grep` are documented.

## Out of scope

- Sentence-level splitting inside a line, suggestions of fixes, and any
  clustering smarter than the two key kinds above. Mint a follow-up if the
  `at:` keys prove too coarse.

## Landing record

1. **Report summary** (`vwlkmvns`; full 32,828 supported faces; schema 8).
   The B6 census has 27,036 covered faces and 923,976 validated Readings,
   zero undetermined faces and zero issues. The report agrees with every
   face's census coverage, with zero internal analysis failures. Census and
   report used six workers. Report peak RSS 11,346,216 KiB is below census
   peak 16,115,972 KiB (`vwlkmvns`). Coordinator artifacts:
   `target/english-v3/baseline-blockers.json`, `blockers.json`, `blockers.txt`.
   JSON retains all 487 vocabulary keys, 1,019 position keys and 244 failing
   tokens (`vwlkmvns`); the
   three text tables below apply B5's limits and include three examples.

    ```text
    5792 unread faces; 4947 with exactly one failing line; 1673 vocabulary-only faces
    Line coverage: 52296 / 59428; 440 covered-face segmentation disagreements
    Vocabulary keys (sole-blocker faces, touched faces; examples):
    word:historic: 40, 56; The Eighth Doctor | Serra Disciple | Cabal Paladin
    word:modified: 40, 47; One with the Kami | Ian the Reckless | Costume Closet
    word:declared: 33, 48; Hua Tuo, Honored Physician | Stern Marshal | Capricious Sorcerer
    word:starting: 31, 84; Collective Voyage | Angel of Destiny | Magister of Worth
    word:exactly: 28, 45; Spry and Mighty | Deadly Wanderings | Mighty Servant of Leuk-o
    word:party: 28, 40; Acquisitions Expert | Cascade Seer | Deadly Alliance
    word:move: 28, 32; Afiya Grove | Costume Closet | Weapon Rack
    word:door: 27, 33; Cramped Vents | Painter's Studio | Underwater Tunnel
    word:mutates: 27, 32; Illuna, Apex of Wishes | Vulpikeet | Dreamtail Heron
    word:excess: 27, 30; Rhino's Rampage | Cramped Vents | Lacerate Flesh
    word:paired: 26, 27; Donna Noble | Joint Assault | Tandem Lookout
    word:take: 25, 40; Lost Isle Calling | Explore the Underdark | Wormfang Manta
    word:difference: 24, 32; Spry and Mighty | Vraska, Betrayal's Sting | Sovereign Okinec Ahau
    word:promised: 24, 24; Mind Spiral | Wildfire Howl | Dewdrop Cure
    word:kind: 23, 35; Tawnos's Coffin | Aetheric Amplifier | Powerful Broker
    word:switch: 23, 25; Dwarven Thaumaturgist | Merfolk Thaumaturgist | Valakut Fireboar
    word:exploits: 21, 23; Fell Stinger | Rakshasa Gravecaller | Infernal Captor
    word:using: 20, 32; Go Nuts! | Scarlet Spider, Ben Reilly | Crossover Collaboration
    word:true: 20, 25; Escaped Shapeshifter | Encroaching Mycosynth | Arcane Adaptation
    word:bargained: 19, 20; Thunderous Debut | Torch the Tower | Stonesplitter Bolt
    word:tied: 18, 51; Sulam Djinn | Magister of Worth | Zanam Djinn
    word:multikicker: 18, 19; Spell Contortion | Everflowing Chalice | Strength of the Tajuru
    word:unblocked: 14, 17; Ydwen Efreet | Mirror Strike | False Orders
    word:fully: 14, 16; Optimistic Scavenger | Fear of Infinity | Scrabbling Skullcrab
    word:warped: 14, 15; Close Encounter | Decode Transmissions | Temporal Intervention
    word:unspent: 13, 20; Fangorn, Tree Shepherd | Drain Power | Pygmy Hippo
    word:else: 13, 16; The Eighth Doctor | Storm Herald | Llanowar Greenwidow
    word:haven't: 13, 16; Wrangler of the Damned | Carpet of Flowers | Acererak the Archlich
    word:deck: 11, 28; Persistent Petitioners | Susan Foreman | Relentless Rats
    word:outlaw: 11, 14; Discreet Retreat | Shoot the Sheriff | Take the Fall
    word:expend: 11, 12; Bark-Knuckle Boxer | Junkblade Bruiser | Roughshod Duo
    word:team: 10, 14; Combo Attack | Regna, the Redeemer | Rushblade Commander
    word:descended: 10, 12; Child of the Volcano | Corpses of the Lost | Enterprising Scallywag
    word:activation: 9, 13; Protective Sphere | Ice Cauldron | Heirloom Epic
    word:crewed: 9, 11; Mindlink Mech | Mighty Servant of Leuk-o | Mobilizer Mech
    word:differently: 9, 10; Sandsteppe War Riders | Survey Mechan | Awakened Amalgam
    word:wicked: 9, 10; Witch's Mark | Price of Beauty | Eriette's Whisper
    word:followed: 9, 9; World at War | Waves of Aggression | Aggravated Assault
    word:haunts: 9, 9; Cry of Contrition | Orzhov Euthanist | Benediction of Moons
    word:crews: 8, 25; Deathless Pilot | Reef Roads | Reckless Velocitaur
    word:empower: 8, 19; Theorist's Proxy | Avatar of Burgeoning Echoes | Way of the Warlord
    word:suspended: 8, 16; Venser's Diffusion | Rose Tyler | Shivan Sand-Mage
    word:cadet: 8, 13; Peer Review | Command the Stage | Hexhaven Battalion
    word:opening: 7, 27; Chancellor of the Forge | Leyline of Abundance | Leyline of Punishment
    word:result: 7, 26; Underdark Rift | Ebony Fly | Wild Endeavor
    word:identity: 7, 9; Fallaji Wayfarer | Command Tower | Commander's Sphere
    word:longer: 7, 9; Melting | Haunted Plate Mail | Serene Sleuth
    word:bands: 7, 8; Cathedral of Serra | Shelkin Brownie | Tolaria
    word:monster: 7, 8; Syr Armont, the Redeemer | Monstrous Rage | Curse of the Werefox
    word:symbols: 7, 8; Heartlash Cinder | Outrage Shaman | Baron Helmut Zemo
    word:plotted: 7, 7; Lilah, Undefeated Slickshot | Make Your Own Luck | Longhorn Sharpshooter
    word:completed: 6, 17; Nadaar, Selfless Paladin | Gloom Stalker | Ravenloft Adventurer
    word:night: 6, 16; Sunrise Cavalier | Gavony Dawnguard | Wolf Strike
    word:colored: 6, 9; Void Mirror | Ragemonger | Charmed Pendant
    word:used: 6, 9; Apex Observatory | Dinosaur Headdress | The Grim Captain
    word:takes: 6, 8; Walk the Aeons | Emrakul, the Promised End | Karn's Temporal Sundering
    word:convoked: 6, 6; Zephyr Singer | Everything Comes to Dust | Ancient Imperiosaur
    word:devoured: 6, 6; Hellkite Hatchling | Skullmulcher | Voracious Dragon
    word:reselect: 6, 6; Capricopian | Chef's Kiss | Portal Mage
    word:secretly: 5, 17; Círdan the Shipwright | Wheel of Misfortune | Malik, Grim Manipulator
    Position keys (sole-blocker faces, touched faces; examples):
    at:. <end>: 152, 180; Goblin Barrage | Cabal Therapy | Bloodthorn Flail
    at:order .: 95, 104; Worldly Counsel | Muxus, Goblin Grandee | Karn, Living Legacy
    at:turn .: 94, 103; Sanguine Indulgence | Flashback | Eye of Duskmantle
    at:— <end>: 43, 196; Saheeli's Artistry | Dawnbringer Cleric | Astarion, the Decadent
    at:flip ,: 42, 46; Goblin Bomb | Goblin Lyre | Ral, Monsoon Mage
    at:only creatures: 42, 42; Welkin Tern | Departed Soulkeeper | Cloud Elemental
    at:the ring: 39, 47; Elrond, Lord of Rivendell | Uruk-hai Berserker | Horses of the Bruinen
    at:life .: 38, 44; Abandoned Campground | Erosion | Indulging Patrician
    at:time .: 35, 50; One with the Multiverse | The Lunar Whale | Cemetery Illuminator
    at:way .: 35, 40; Coalition Relic | Fountain of Cho | Subterranean Hangar
    at:choose not: 34, 45; Tawnos's Coffin | Giant Oyster | Frightshroud Courier
    at:basic landcycling: 34, 36; Absorb Vis | Infernal Rebirth | Battle Plan
    at:transforms into: 32, 32; Ulrich of the Krallenhorde | Ulrich, Uncontested Alpha | Grub, Storied Matriarch
    at:you next: 30, 34; Zevlor, Elturel Exile | The Clone Saga | Jeong Jeong, the Deserter
    at:graveyard .: 29, 49; Glimpse of Freedom | Sentinel's Eyes | Phoenix of Ash
    at:, you: 28, 32; Phial of Galadriel | Convalescent Care | Hoarding Broodlord
    at:other into: 28, 29; Tower Geist | Kodama's Reach | Flare of Cultivation
    at:creatures able: 26, 28; Elvish Bard | Breaker of Armies | Nessian Boar
    at:more lands: 25, 29; Beza, the Bounding Spring | Scouting Hawk | Weathered Wayfarer
    at:them back: 25, 26; Ponder | Sensei's Divining Top | Descendant of Soramaro
    at:exiled .: 24, 27; Gustha's Scepter | Brainstealer Dragon | Jacob Hauken, Inspector
    at:first .: 24, 25; Hour of Need | Polymorphous Rush | Solidarity of Heroes
    at:take an: 23, 25; Twice Upon a Time | Temporal Extortion | Alchemist's Gambit
    at:play with: 23, 23; Melek, Izzet Paragon | Magus of the Future | Vampire Nocturnus
    at:background <end>: 22, 32; Halsin, Emerald Archdruid | Karlach, Fury of Avernus | Jaheira, Friend of the Forest
    at:double that: 22, 25; The Sound of Drums | Charging Tuskodon | Goblin Goliath
    at:and dealt: 21, 22; Moonlight Geist | Sandskin | Soratami Cloud Chariot
    at:it's not: 21, 22; Glademuse | Tezzeret, Cruel Captain | Force of Despair
    at:once each: 20, 27; Hired Claw | Ebon Praetor | Maralen, Fae Ascendant
    at:is less: 20, 23; Phenax, God of Deception | Erebos, God of the Dead | Elderscale Wurm
    at:put there: 20, 23; Ghost of Ramirez DePietro | Wreck Hunter | The Fourteenth Doctor
    at:" <end>: 20, 20; Mercurial Pretender | Pirated Copy | Mathas, Fiend Seeker
    at:abilities .: 18, 18; Damping Matrix | Blood Sun | Faith's Fetters
    at:combat .: 17, 20; Observed Stasis | Gustcloak Runner | Stunning Strike
    at:more life: 17, 19; Namor, Atlantean King | Oath of Mages | Glorious Enforcer
    at:once during: 16, 21; Kotis, Sibsig Champion | One with the Multiverse | Hauken's Insight
    at:total .: 16, 21; Volcano Hellion | Aettir and Priwen | Ajani, Caller of the Pride
    at:" and: 16, 19; Mindbender Spores | Roxanne, Starfall Savant | Red Mage's Rapier
    at:, that: 16, 19; Tunnel Vision | Traveling Chocobo | Ancient Greenwarden
    at:all the: 16, 18; Heartwarming Redemption | Awaken the Erstwhile | Chandra, Flamecaller
    at:more cards: 16, 17; Death of a Thousand Stings | Akuta, Born of Ash | Robber of the Rich
    at:it .: 15, 19; Apocalypse Hydra | Enthralling Hold | Jinxed Choker
    at:no more: 15, 19; Silent Arbiter | The Eternal Wanderer | Crawlspace
    at:card .: 15, 18; Spell Snuff | Kutzil, Malamet Exemplar | Inga and Esika
    at:game .: 15, 18; Empyrial Storm | Fury Storm | Henzie "Toolbox" Torre
    at:to twice: 15, 17; Duggan, Private Detective | Deathforge Shaman | Melek, Reforged Researcher
    at:end .: 15, 16; Colossal Plow | Tanuki Transplanter | Rousing Refrain
    at:return to: 15, 16; Gerrard's Hourglass Pendant | Storrev, Devkarin Lich | Scrap Trawler
    at:<start> waterbend: 14, 14; Ruthless Waterbender | Katara, Water Tribe's Hope | Watery Grasp
    at:you may: 14, 14; Xanathar, Guild Kingpin | Case of the Locked Hothouse | Flood of Tears
    at:hand .: 13, 17; Tazri, Stalwart Survivor | Halimar Tidecaller | Surge of Brilliance
    at:, this: 13, 14; Guul Draz Vampire | Harsh Mentor | Burning-Eye Zubera
    at:any of: 13, 14; Mindblade Render | Pit of Offerings | Tazri, Stalwart Survivor
    at:to that: 13, 14; Nyssa of Traken | Hexhaven Invigorator | Loamcrafter Faun
    at:and attacking: 13, 13; Furnace-Blessed Conqueror | Shaun, Father of Synths | Loki, the Deceiver
    at:is twice: 13, 13; Gandalf, Party Guest | Jovial Evil | Reflecting Mirror
    at:phases out: 12, 15; Blink Dog | Creeping Inn | Robe of Stars
    at:many plus: 12, 13; Benevolent Hydra | Michelangelo, Weirdness to 11 | Mowu, Loyal Companion
    at:step .: 12, 13; Storage Matrix | Nalathni Dragon | Initiates of the Ebon Hand
    at:sorcery .: 12, 12; Cloud Key | Scandalmonger | Balustrade Wurm
    at:<start> doctor's: 11, 27; Nyssa of Traken | Donna Noble | Barbara Wright
    at:creature .: 11, 16; Greater Werewolf | Giant Oyster | Bloodspore Thrinax
    at:with {x}: 11, 14; Lattice Library | Helga, Skittish Seer | Troyan, Gutsy Explorer
    at:aura .: 11, 12; Floating Shield | Flickering Ward | Pentarch Ward
    at:effect .: 11, 12; Quickening Licid | Tempting Licid | Transmogrifying Licid
    at:lesser mana: 11, 12; Rashmi, Eternities Crafter | Kethek, Crucible Goliath | Havi, the All-Father
    at:only a: 11, 12; Muck Drubb | Radiate | Frontline Heroism
    at:if tribute: 11, 11; Oracle of Bones | Ornitharch | Nessian Demolok
    at:library .: 11, 11; Green Sun's Twilight | Memories Returning | The Raven's Warning
    at:other .: 11, 11; Arena | Blizzard Brawl | Grim Contest
    at:suspend ,: 11, 11; The Tenth Doctor | Taigam, Master Opportunist | The Parting of the Ways
    at:more creatures: 10, 15; Heidegger, Shinra Executive | Pulse of the Tangle | Wild Mammoth
    at:black .: 10, 12; Abhorrent Overlord | Gray Merchant of Asphodel | Blight-Breath Catoblepas
    at:phase out: 10, 12; Taniwha | Clever Concealment | Ripples of Potential
    at:random from: 10, 12; Tomb Tyrant | Charmbreaker Devils | Viconia, Drow Apostate
    at:times .: 10, 12; Primal Adversary | Spectral Adversary | Piper Wright, Publick Reporter
    at:, it: 10, 11; Venomsac Lagac | Congregation Gryff | Dracosaur Auxiliary
    at:, the: 10, 11; Don & Raph, Hard Science | Spider Climb | Lightning Reflexes
    at:1 of: 10, 11; Thunderstaff | Heart-Shaped Herb | Valkmira, Protector's Shield
    at:create a: 10, 11; The Book of Exalted Deeds | Valkyrie Harbinger | Resplendent Angel
    Failing tokens (sole-blocker faces, touched faces; examples):
    .: 941, 1100; Coalition Relic | Sanguine Indulgence | Syndicate Heavy
    <end>: 304, 539; Saheeli's Artistry | Goblin Barrage | Dawnbringer Cleric
    ,: 96, 105; Goblin Bomb | Goblin Lyre | Minimus Containment
    not: 73, 89; Glademuse | Tawnos's Coffin | Giant Oyster
    that: 70, 80; Nyssa of Traken | The Sound of Drums | Eye of Ojer Taq
    into: 60, 61; Ulrich of the Krallenhorde | Ulrich, Uncontested Alpha | Tower Geist
    creatures: 57, 62; Welkin Tern | Departed Soulkeeper | Cloud Elemental
    twice: 55, 64; Doubling Season | Fangorn, Tree Shepherd | Duggan, Private Detective
    to: 52, 55; Gerrard's Hourglass Pendant | Embermaw Hellion | Parallax Wave
    of: 51, 58; Kotis, Sibsig Champion | Mindblade Render | Pit of Offerings
    ring: 39, 47; Elrond, Lord of Rivendell | Uruk-hai Berserker | Horses of the Bruinen
    a: 39, 44; Muck Drubb | Foul Emissary | The Book of Exalted Deeds
    you: 37, 42; Phial of Galadriel | Convalescent Care | Hoarding Broodlord
    the: 36, 41; Heartwarming Redemption | Don & Raph, Hard Science | Awaken the Erstwhile
    this: 36, 38; Guul Draz Vampire | Treasure Map | Neglected Heirloom
    landcycling: 35, 37; Absorb Vis | Infernal Rebirth | Battle Plan
    plus: 34, 38; Benevolent Hydra | Goblin Gathering | Memorial Vault
    noncombat: 33, 38; Soul-Scar Mage | Fear of Burning Alive | Command the Stage
    back: 32, 33; Ponder | Sealed Fate | Sensei's Divining Top
    with: 32, 33; Gorex, the Tombshell | Karn, Scion of Urza | Melek, Izzet Paragon
    more: 30, 36; Asinine Antics | Oakshade Stalker | Silent Arbiter
    next: 30, 34; Zevlor, Elturel Exile | The Clone Saga | Jeong Jeong, the Deserter
    until: 30, 33; Veil of Summer | Strength of Night | Sworn Defender
    it: 30, 31; Burning Wish | Prize Pig | Brightfield Mustang
    and: 29, 32; Akawalli, the Seething Tower | Mindbender Spores | Roxanne, Starfall Savant
    an: 29, 31; Twice Upon a Time | Caged Sun | Temporal Extortion
    {e}: 28, 35; Nissa, Worldsoul Speaker | Wrath of the Skies | Lightning Runner
    able: 28, 31; Elvish Bard | Breaker of Armies | Nessian Boar
    from: 27, 30; Tomb Tyrant | Rootcoil Creeper | Mirko, Obsessive Theorist
    waterbend: 27, 27; Ruthless Waterbender | Katara, Seeking Revenge | Benevolent River Spirit
    lands: 25, 30; Beza, the Bounding Spring | Scouting Hawk | Weathered Wayfarer
    ": 24, 29; Spara's Adjudicators | Vraska, Betrayal's Sting | Furystoke Giant
    mana: 24, 25; Rashmi, Eternities Crafter | Lukka, Coppercoat Outcast | Kethek, Crucible Goliath
    +: 23, 38; Wedgelight Rammer | Rescue Skiff | The Eternity Elevator
    dealt: 23, 24; Moonlight Geist | Sandskin | Soratami Cloud Chariot
    each: 22, 29; Hired Claw | Ebon Praetor | Maralen, Fae Ascendant
    {r}: 22, 27; Archangel of Wrath | Goblin Clearcutter | Grand Warlord Radha
    out: 22, 26; Blink Dog | Creeping Inn | Taniwha
    {g}: 21, 28; Saltcrusted Steppe | Leyline of Abundance | Mythos of Nethroi
    less: 20, 23; Phenax, God of Deception | Erebos, God of the Dead | Elderscale Wurm
    ```

2. **Witnesses** (original counts `sqnyzpnl`; B5 keys/tests `rwplnpkk`). Synthetic line Reading counts: undeclared
   second line `[1, 0]`, key `word:zzyzx`, sole blocker; declared words in
   wrong order (`Draw Draw.`) `[0]`, key `at:draw .`, sole blocker; covered
   two-line face `[1, 1]`, no failing line; two failing lines `[1, 0, 0]`,
   keys `word:zzyzx` and `word:qqxyz`, sole blocker for neither. Additional
   tests preserve face-level deduplication for repeated keys and covered
   modal-list segmentation with a reported newline disagreement.

3. **Spot checks** (original probes `sqnyzpnl`; B6 report counts/keys
   `vwlkmvns`). Ten faces checked line by line with
   `probe --as-card NAME --text LINE --readings 8 --json`: Ancestral Recall
   `[1]`; Lightning Bolt `[1]`; Black Lotus `[1]`; Counterspell `[1]`;
   Brainstorm `[2]`; Guul Draz Vampire `[0]`, `at:, this`; Nissa, Worldsoul
   Speaker `[2, 0]`, `at:eight {e}`; Syndicate Heavy `[1, 0]`,
   `at:investigate .`; Goblin Barrage `[0, 1]`, `at:. <end>`; Hua Tuo,
   Honored Physician `[0]`, `word:declared`. Original probes enumerated
   completely, had zero issues, and every inspected Reading realized
   exactly. The B6 report preserves these line verdicts/counts and gives
   the word-level keys shown above. Neoform now has `[1, 262]` and no
   failing line (`vwlkmvns`), confirming resolution of the stale baseline.
   Probe evidence: `target/english-v3/english-v3-blocker-report/spotcheck-*.json`.

4. **Deviations and STOPs**. Exposed the permitted read-only chart statistic
   `Metrics::furthest_position`; kept it out of the census schema. No
   grammar, lexicon, construction, frame or gate changed. Runtime tokens
   are Unicode scalars; B5 uses report-only word tokens to locate keys, with
   context kept outside the key. The original report minted
   `english-v3-blocker-word-context` because the `at:\s t` group combined
   *this*, *the* and *twice*; B5 resolves that follow-up within this ticket,
   so the superseded planned ticket is removed. Normalized the existing completed
   `english-v3-derived-punctuation-forms` dependency's quoted frontmatter
   ID; the adapter had treated the quotes as part of the ID. Kanban check
   now passes. Two extra tests cover deduplication and modal segmentation.
   Corrected one new test's mistaken frontier expectation against the packed
   probe: the chart reaches the second *Draw* and stops before the full stop.
   B5 re-spells that witness against the whole previous token, failing token
   and context, replacing the incidental scalar-position count. Three added
   tests cover Unicode/brace/numeral context, boundary selection and grouped
   failing-token deduplication. No glossary gap. STOP: the B5 full report
   fails B1 on Neoform after the grammar refresh; resolved by B6's authorized fresh census at `vwlkmvns`, which passed
   strict B1. The optional stale-versus-disagreement diagnostic is unchanged:
   the binary has no embedded grammar revision, and the census's change id
   alone cannot establish which grammar the binary contains. Tooling exception: no before/after grammar census or red witnesses
   for this implementation. Refresh brought equality-property grammar work
   (`wlstpkpu`); the bounded re-check found no lost faces or removed Readings.

5. **Tests** (`sqnyzpnl`). `cargo test -p xtask english_v3`: 53 passed (the
   initial run had 52 passes and the mistaken new expectation described
   above; the corrected run passes). `cargo test -p deckmaste_english_v3`:
   712 passed. `cargo clippy -p xtask --tests -- -D warnings` and
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`: both pass.
   Restored 0 / re-spelled 0 / ignored 0 / added 6 / removed 0; removals:
   none. B5 re-spells the wrong-order witness as described above. Citation checks: zero
   noncompliant and zero stale references; no newly cited rules.
   After refresh (`qyllylkq`), `cargo test -p xtask` in full passes:
   404 library, 4 binary, and 14 integration tests (422 total), zero
   failures. The existing ignored
   `macros::templates::tests::macro_schema_census_count_matches_21` remains
   ignored with its existing reason, "cross-checks the live corpus against
   the census; run on demand"; this change introduces no ignores.
   After refresh (`qyllylkq`), `cargo test -p deckmaste_english_v3` in full
   passes: 723 tests, zero failures and zero ignored. The pre-refresh
   clippy checks above passed; refresh verification follows the bounded
   census plus full-crate-test policy.
   B5 (`rwplnpkk`): `cargo test -p xtask english_v3` passes all 56 tests,
   zero failures and zero ignored. Added 3 / re-spelled 1 / restored 0 /
   ignored 0 / removed 0 in the review fix; cumulative added 9. The
   re-spelled wrong-order witness asserts the word-level key and context,
   as required by B5. `cargo clippy -p xtask --tests -- -D warnings`
   passes after replacing one new empty-context assertion with an exact
   empty-vector comparison, as clippy requires. No suppression or weakening.
   The B5 report rerun failed B1 as above. B6 subsequently authorized the
   fresh full census and report; both pass at `vwlkmvns`. Prior test/clippy
   results are retained; B6 changes the record, not report code.

6. **Timing** (`vwlkmvns`). The B6-authorized full census took **449480 ms**,
   six workers, prebuilt refreshed-head binary under `/usr/bin/time -v`,
   samples per face 0. `free -g` showed 75 GB available immediately before
   launch. Timing and verification logs, including the original artifacts,
   are preserved in the coordinator's
   `target/english-v3/english-v3-blocker-report/`.
