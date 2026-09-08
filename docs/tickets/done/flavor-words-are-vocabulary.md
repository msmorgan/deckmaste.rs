---
needs: []
---
**Delete the flavor-word declarations.** Ruling (user, 2026-09-07). A
flavor word is an open slot: `flavorWord (word : FlavorWordLabel)
(ability : Ability)` in `Macros.lean`, ported as the builtin `flavorWord`
helper, so a card writes `flavorWord("Kowabunga", ability)` with the label
as a string. Nothing about a flavor word is enumerable, so the 630
per-word declarations under `plugins_v2/builtin/macros/flavor_words/`
model nothing. Delete the family, the `FlavorWord` meta,
`DeclarationKind::FlavorWord` and the `flavor_words` entry in
construction_core's `BUILTIN_FAMILIES`, and whatever in the v2 reader or
`xtask` registers or counts the family. The corpus test states the new
count.

english_v2 breakage this causes is accepted: English v3 is the horizon
version and english_v2 is not corrected for this. Record what broke
(coverage-lock identities that stopped parsing, failing tests by name)
in the landing record and route it to the v3 tickets, not to english_v2
fixes. The tests that fail because their subject is gone are re-spelled
against `deckmaste_lexical`'s open slot where a v3 subject exists, and
otherwise listed with the ticket that will re-cover them.

`deckmaste_lexical` (English v3's lexical crate) is in scope: an italic run
before an ability that is not a declared ability word is a flavor word,
captured verbatim as its label; no `Source` of any `SourceKind` points at
a flavor-word list, and `english-v3-lexical-inventory` excludes the
family from its input. Pin it: an italic run absent from every
declaration analyses as a flavor word, and a declared ability word does
not. Standard constraints apply, except the coverage lock: a decrease
whose every lost identity is listed and routed is this ticket's expected
result.

## Landing record

Measured on `vpzxlqkorpxz` (`analyse a flavor word as the open slot it is`),
coverage lock `covered` = 20254, tree `covered_units` = 20056.

### PROVE

**No silent loss.** 198 corpus identities are no longer covered against
`english-v2-coverage.lock`. Two of them — `Start the TARDIS` and `TARDIS` —
were already uncovered on the claim base (`ktsrluopywwu`, baseline
`covered_units` 20252 against a lock of 20254); they are the Planechase
*planeswalk* surface and belong to `core-retire-planechase-surface`, not to
this landing. The other **196 are this landing's**, each classified **wrong
analysis retired**: every one carries a deleted flavor word as an italic
ability head, and english_v2 parsed it only because that flavor word was
enumerated as a declaration. Re-coverage is owed to
`english-v3-whole-grammar-activation`, which carries a note pointing back
here. Every one of the 196 was checked mechanically against
`data/mtgjson/AtomicCards.json`: the card text carries one of the 630 deleted
spellings immediately before an em dash at an ability head. There is no
unexplained loss and no regression.

The full list, each with the flavor word (or words) it carries:

- `Aberrant` — *Heavy Power Hammer*
- `Acolyte Hybrid` — *Heavy Rock Cutter*
- `Aerith Rescue Mission` — *Take 59 Flights of Stairs*, *Take the Elevator*
- `Air-Cult Elemental` — *Whirlwind*
- `Amarant Coral` — *No Mercy*
- `Arco-Flagellant` — *Endurant*
- `Assault Intercessor` — *Chainsword*
- `Astrid Peth` — *Brand-new Sky*
- `Atalan Jackal` — *Skilled Outrider*
- `Baleful Beholder` — *Antimagic Cone*, *Fear Ray*
- `Bane's Invoker` — *Wind Walk*
- `Barret, Avalanche Leader` — *Avalanche!*
- `Battle Menu` — *Ability*, *Attack*, *Item*, *Magic*
- `Be'lakor, the Dark Master` — *Lord of Torment*, *Prince of Chaos*
- `Bhaal's Invoker` — *Scorching Ray*
- `Black Bolt, Inhuman King` — *Lethal Voice*
- `Black Dragon` — *Acid Breath*
- `Black Market Connections` — *Buy Information*, *Hire a Mercenary*, *Sell Contraband*
- `Bloodcrusher of Khorne` — *Devastating Charge*
- `Blue Dragon` — *Lightning Breath*
- `Canoptek Scarab Swarm` — *Feeder Mandibles*
- `Canoptek Spyder` — *Fabricator Claw Array*
- `Cecil, Dark Knight // Cecil, Redeemed Paladin (Cecil, Redeemed Paladin)` — *Protect*
- `Chain Devil` — *Animate Chains*
- `Chaos Terminator Lord` — *Lord of Chaos*
- `Choose Your Weapon` — *Archery*, *Two-Weapon Fighting*
- `Chronomancer` — *Atomic Transmutation*
- `Cid, Freeflier Pilot` — *Jump*
- `Circle of the Land Druid` — *Natural Recovery*
- `Circle of the Moon Druid` — *Bear Form*
- `Clamavus` — *Proclamator Hailer*
- `Cleopatra, Exiled Pharaoh` — *Allies*, *Betrayal*
- `Cloakwood Swarmkeeper` — *Gathered Swarm*
- `Cloud of Darkness` — *Particle Beam*
- `Commander Sofia Daguerre` — *Crash Landing*
- `Commissar Severina Raine` — *Leading from the Front*, *Summary Execution*
- `Cryptothrall` — *Protector*
- `Cult of Skaro` — *Caan*, *Jast*, *Sec*, *Thay*
- `Cybernetica Datasmith` — *Field Reprogramming*
- `Daily Bugle Reporters` — *Investigative Journalism*, *Puff Piece*
- `Dalek Drone` — *Exterminate!*
- `Damage Control Crew` — *Impound*, *Repair*
- `Dark Apostle` — *Gift of Chaos*
- `Dawnbringer Cleric` — *Cure Wounds*, *Dispel Magic*, *Gentle Repose*
- `Death Tyrant` — *Negative Energy Cone*
- `Devoted Paladin` — *Beacon of Hope*
- `Diamond Weapon` — *Immune*
- `Displacer Beast` — *Displacement*
- `Displacer Kitten` — *Avoidance*
- `Drach'Nyen` — *Daemon Sword*, *Echo of the First Murder*
- `Duggan, Private Detective` — *The Most Important Punch in History*
- `Elder Arthur Maxson` — *Blind Betrayal*
- `Emet-Selch, Unsundered // Hades, Sorcerer of Eld (Hades, Sorcerer of Eld)` — *Echo of the Lost*
- `Epistolary Librarian` — *Veil of Time*
- `Exocrine` — *Bio-plasmic Barrage*
- `Firbolg Flutist` — *Enthralling Performance*
- `Flameskull` — *Rejuvenation*
- `Flash Thompson, Spider-Fan` — *Heckle*, *Hero Worship*
- `Flayed One` — *Flesh Flayer*
- `Gau, Feral Youth` — *Rage*
- `Genestealer Locus` — *Neurotraumal Rod*
- `Ghastly Death Tyrant` — *Death Ray*, *Disintegration Ray*
- `Ghost, Spectral Saboteur` — *Intangibility*
- `Githzerai Monk` — *Psychic Defense*
- `Goliath Truck` — *Stowage*
- `Grey Knight Paragon` — *Rites of Banishment*
- `Grim Wanderer` — *Tragic Backstory*
- `Guild Thief` — *Cunning Action*
- `Half-Elf Monk` — *Stunning Strike*
- `Haruspex` — *Devouring Monster*, *Rapacious Hunger*
- `Herald of Slaanesh` — *Locus of Slaanesh*
- `Hexmark Destroyer` — *Multi-threat Eliminator*
- `Hormagaunt Horde` — *Endless Swarm*
- `Icewind Stalwart` — *Protection Fighting Style*
- `Imotekh the Stormlord` — *Grand Strategist*, *Phaeron*
- `Inquisitor Greyfax` — *Hunt for Heresy*, *Unquestionable Wisdom*
- `Inspiring Bard` — *Bardic Inspiration*, *Song of Rest*
- `Jecht, Reluctant Guardian // Braska's Final Aeon (Braska's Final Aeon)` — *Jecht Beam*, *Ultimate Jecht Shot*
- `Juvenile Mist Dragon` — *Confounding Clouds*
- `Kain, Traitorous Dragoon` — *Jump*
- `Keeper of Secrets` — *Symphony of Pain*
- `Kenku Artificer` — *Homunculus Servant*
- `Knight Paladin` — *Rapid-fire Battle Cannon*
- `Kraven, Proud Predator` — *Top of the Food Chain*
- `Lizard, Connors's Curse` — *Lizard Formula*
- `Locke, Treasure Hunter` — *Mug*
- `Lokhust Heavy Destroyer` — *Enmitic Exterminator*
- `Lord of Change` — *Architect of Deception*
- `Lychguard` — *Guardian Protocols*
- `Magnus the Red` — *Blade of Magnus*, *Unearthly Power*
- `Malanthrope` — *Scavenge the Dead*
- `Manticore` — *Tail Spikes*
- `Marneus Calgar` — *Chapter Master*, *Master Tactician*
- `Mawloc` — *Terror from the Deep*
- `Mind Flayer` — *Dominate Monster*
- `Mold Folk` — *Mold Harvest*
- `Monk of the Open Hand` — *Flurry of Blows*
- `Myconid Spore Tender` — *Infesting Spores*
- `Myrkul's Invoker` — *Psychic Blades*
- `Necron Monolith` — *Eternity Gate*
- `Necron Overlord` — *Relentless March*
- `Night Scythe` — *Invasion Beams*
- `Noise Marine` — *Sonic Blaster*
- `Old One Eye` — *Fast Healing*
- `Owlbear` — *Keen Senses*
- `Pip-Boy 3000` — *Check Map*, *Pick a Perk*, *Sort Inventory*
- `Plague Drone` — *Rot Fly*
- `Plasmancer` — *Dynastic Advisor*
- `Plundering Barbarian` — *Pry It Open*, *Smash the Chest*
- `Poxwalkers` — *Curse of the Walking Pox*
- `Primaris Chaplain` — *Rosarius*
- `Primaris Eliminator` — *Executioner Round*, *Hyperfrag Round*
- `Prosper, Tome-Bound` — *Mystic Arcanum*, *Pact Boon*
- `Psychomancer` — *Harbinger of Despair*
- `Purestrain Genestealer` — *Vanguard Species*
- `Quake, Agent of S.H.I.E.L.D.` — *Seismic Takedown*
- `Red Dragon` — *Fire Breath*
- `Reptil, Dinomorpher` — *Brontosaurus*, *Tyrannosaurus Rex*
- `Rescuer Chwinga` — *Natural Shelter*
- `Royal Warden` — *Phalanx Commander*
- `Sanguinary Priest` — *Blood Chalice*
- `School Daze` — *Do Homework*, *Fight Crime*
- `Scouting Hawk` — *Keen Sight*
- `Scout the City` — *Bring Down*, *Look Around*
- `Screamer-Killer` — *Bio-Plasmic Scream*
- `Secret Identity` — *Conceal*, *Reveal*
- `Shadow of the Goblin` — *Undying Vengeance*, *Unreliable Visions*
- `Shao Jun` — *Leap Strike*, *Rope Dart*
- `Shard of the Void Dragon` — *Matter Absorption*, *Spear of the Void Dragon*
- `Shessra, Death's Whisper` — *Bewitching Whispers*, *Whispers of the Grave*
- `Shocker, Unshakable` — *Vibro-Shock Gauntlets*
- `Sicarian Infiltrator` — *Benediction of the Omnissiah*
- `Sidequest: Play Blitzball // World Champion, Celestial Weapon (World Champion, Celestial Weapon)` — *Double Overdrive*
- `Sister Hospitaller` — *Medicus Ministorum*
- `Sister Repentia` — *Martyrdom*
- `Skorpekh Destroyer` — *Hyperphase Threshers*
- `Skorpekh Lord` — *Command Protocols*
- `Sloppity Bilepiper` — *Jolly Gutpipes*
- `Sonic the Hedgehog` — *Gotta Go Fast*
- `Space Marine Devastator` — *Grav-cannon*
- `Space Marine Scout` — *Concealed Position*
- `Sphere Grid` — *Unlock Ability*
- `Spider-Ham, Peter Porker` — *Animal May-Ham*
- `Spider-Man India` — *Pavitr's Sevā*
- `Spider-Woman, Stunning Savior` — *Venom Blast*
- `Stirge` — *Blood Drain*
- `Storm, Force of Nature` — *Ceaseless Tempest*
- `Summon: Anima` — *Oblivion*, *Pain*
- `Summon: Choco/Mog` — *Stampede!*
- `Summon: Esper Ramuh` — *Judgment Bolt*
- `Summon: Fat Chocobo` — *Kerplunk*, *Wark*
- `Summon: Knights of Round` — *Ultimate End*
- `Summon: Magus Sisters` — *Combine Powers!*, *Defense!*, *Fight!*
- `Summon: Primal Garuda` — *Aerial Blast*, *Slipstream*
- `Summon: Shiva` — *Diamond Dust*, *Heavenly Strike*
- `Szarekh, the Silent King` — *My Will Be Done*
- `Tervigon` — *Spawn Termagants*
- `Thancred Waters` — *Royal Guard*
- `The Eleventh Doctor` — *I. AM. TALKING!*
- `The Emperor of Palamecia // The Lord Master of Hell (The Lord Master of Hell)` — *Starfall*
- `The Fifth Doctor` — *Peaceful Coexistence*
- `The Red Terror` — *Advanced Species*
- `The Thirteenth Doctor` — *Team TARDIS*
- `The Unbeatable Squirrel Girl` — *Do You Like Squirrels?*, *I LOVE Squirrels!*
- `Thunderwolf Cavalry` — *Crushing Teeth*
- `Towering Viewpoint` — *Leap of Faith*
- `Trickster's Talisman` — *Invoke Duplicity*
- `Trygon Prime` — *Subterranean Assault*
- `Tymora's Invoker` — *Sleight of Hand*
- `Tyranid Harridan` — *Shrieking Gargoyles*
- `Tyranid Prime` — *Synapse Creature*
- `Tyrant Guard` — *Shieldwall*
- `Uchuulon` — *Horrific Symbiosis*
- `Undercity Dire Rat` — *Rat Tail*
- `Valor Singer` — *Combat Inspiration*
- `Vanguard Suppressor` — *Suppressing Fire*
- `Venat, Heart of Hydaelyn // Hydaelyn, the Mothercrystal (Hydaelyn, the Mothercrystal)` — *Blessing of Light*
- `Venomcrawler` — *Devourer of Souls*
- `Vrock` — *Toxic Spores*
- `White Dragon` — *Cold Breath*
- `Winged Hive Tyrant` — *The Will of the Hive Mind*
- `Winter Eladrin` — *Gust of Wind*
- `Wraith, Vicious Vigilante` — *Fear Gas*
- `You Come to a River` — *Fight the Current*, *Find a Crossing*
- `You Come to the Gnoll Camp` — *Fend Them Off*, *Intimidate Them*
- `You Find a Cursed Idol` — *Lift the Curse*, *Smash It*, *Steal Its Eyes*
- `You Find the Villains' Lair` — *Foil Their Scheme*, *Learn Their Secrets*
- `You Happen On a Glade` — *Journey On*, *Make Camp*
- `You Hear Something on Watch` — *Rouse the Party*, *Set Off Traps*
- `You Meet in a Tavern` — *Form a Party*, *Start a Brawl*
- `You're Ambushed on the Road` — *Make a Retreat*, *Stand and Fight*
- `You're Confronted by Robbers` — *Call for Aid*, *Stall for Time*
- `You See a Guard Approach` — *Distract the Guard*, *Hide*
- `You See a Pair of Goblins` — *Befriend Them*, *Charge Them*
- `Zenos yae Galvus // Shinryu, Transcendent Rival (Zenos yae Galvus)` — *My First Friend*
- `Zoanthrope` — *Warp Blast*

**Structural laws** (`DECKMASTE_COVERAGE_LOCK=report cargo xtask english_v2
coverage --check`, full corpus): `roundtrip_mismatch_units` 0,
`ownership_failure_units` 0, `nonterminal_nodes` 891903 =
`visited_constructions` 891903 with `traversal_failure_units` 0,
`expected_leaves` 311229 = `visited_leaves` 311229 with
`leaf_traversal_failure_units` 0, `unresolved_ties` 0, `internal_failures` 0,
`gap_spans` 0, `overlap_spans` 0, `provenance_plan_mismatches` 0,
`synthetic_claims` 0.

**No word-naming.** `licensing_checker_forbidden` 0;
`licensing_checker_permitted` 24, every one a `declared_license_feature` or a
`structural_predicate`. `crates/deckmaste_english_v2/src/environment.rs` load
errors: none. The one guard this landing adds —
`Lexicon::analyze_italic_head` — reads a declared feature
(`Lexeme::category == Category::Keyword`) and names no lexeme, construction,
verb, noun, preposition or card identity.

**Byte-exact corpus oracle.** `lean/Generated` is byte-identical before and
after (`diff -r` against a copy taken on the claim base);
`cargo xtask lean-check plugins_v2/canon` 118/118 and
`plugins_v2/testing` 2/2; `cargo xtask facts check` reports both
`Facts.lean` and `FactsGen.idr` up to date.

### DISCLOSE

**Identities newly covered: none.** This landing deletes declarations and
adds no construction, so nothing starts parsing and no negative oracle is
involved.

**Selection census** (`cargo xtask english_v2 ambiguity`, full corpus):

| | before (`ktsrluopywwu`) | after (`vpzxlqkorpxz`) |
|---|---|---|
| selected | 20252 | 20056 |
| unique | 17197 | 17042 |
| specificity_resolved | 3055 | 3014 |
| unresolved_ties | 0 | 0 |
| specificity share | 15.085% | 15.028% |

The specificity share **fell**, so no construction pair is named. The
196 lost units split 155 unique / 41 specificity-resolved.

**Permitted licensing-checker total:** 24 (unchanged from the claim base).

**Deviations and additions.**

- *Deleted beyond the ticket's letter:* `crates/deckmaste_data/src/flavor_words.rs`
  and its `pub mod flavor_words;` in `lib.rs`. The module is the corpus census
  that produced the family; its only two consumers were the deleted xtask
  generator and the deleted `construction_core` nursery test, so it is
  "whatever … counts the family" by the ticket's own words and would otherwise
  be dead public API. This is what pulls `deckmaste_data`'s reverse-dependency
  closure (`deckmaste_english`, `deckmaste_spelling`, `deckmaste_migrations`,
  `deckmaste_catalogs`) into the gate.
- *Deleted as the direct consequence in english_v2* (accepted breakage, not a
  correction): the `FlavorWordTerm` codec, `construction flavor_word_mode_marker`,
  `construction flavor_word_label_term`, the `FlavorWord` variants of
  `abstract sum ModeMarker` and `abstract sum LabelTerm`, four `ast` re-exports
  and two `visit` re-exports. `validate.rs` rejects `kinds = [FlavorWord]`
  outright, so without this english_v2 does not compile and neither does xtask,
  which would make step 2's census unrunnable.
- *Deleted in xtask's diagnostic taxonomy:* `NonterminalKind::FlavorWordModeMarker`
  and `::FlavorWordLabelTerm` with their projection arms. `schema_version` stays
  at 2: the report shape is unchanged and the two categories no longer exist to
  be projected.
- *Added beyond the ticket's letter:* a third assertion in the new pin — a
  declared **verb** (`attack`, a keyword action, and a printed flavor word) is
  still a flavor word at an italic head. The ticket's rule is "not a declared
  **ability word**", so the discriminating case is what makes the pin mean
  anything; two assertions alone are satisfied by "any declared vocabulary".
- *Added beyond the ticket's letter:* three Oracle English glossary entries —
  **Italic Head**, **Ability Word**, **Flavor Word** — with `[CR#207.2c]`,
  `[CR#207.2d]` and `[CR#207.2b]`. See the glossary gap below.
- *Floor changed:* `deckmaste_semantics_v2/tests/corpus.rs::no_source_file_writes_out_an_elided_constructor`
  scanned `>= 2_000` files; the corpus legitimately shrank by 631 files, so the
  floor is now `>= 1_500` against 1584 actual.
- *Not touched:* the v1 flavor-word path (`deckmaste_english`'s Scryfall
  catalog header peel, `deckmaste_catalogs::legacy`, `xtask/src/english/data.rs`,
  `scripts/fetch_data`'s `flavor-words` catalog) is a different mechanism that
  deletes with v1; the Lean/Idris/`english/` workbench spellings of
  `flavorWord` are the open slot itself.

**STOPs:** none. Every crate that broke broke as the deletion's direct
consequence (the ticket's STOP condition was not met), and no
ticket-vs-ruling contradiction arose.

**Glossary gap (now closed).** `docs/contexts/oracle-english/CONTEXT.md`
defined neither *flavor word* nor *ability word* nor any term for the
italicized run they occupy, though the landing turns on the difference
between them. Three entries were added under the `## Language` section, before
**Keyword Quality**: **Italic Head** (project term), **Ability Word**
[CR#207.2c] and **Flavor Word** [CR#207.2d], the last with an `_Avoid_` line
separating it from Flavor Text [CR#207.2b].

### REPORT

Provenance, never a gate, all stamped on `vpzxlqkorpxz` /
tree `covered_units` 20056 / lock `covered` 20254.

**Corpus counts** (`cargo test -p deckmaste_semantics_v2 --test corpus`):

| | before | after |
|---|---|---|
| `plugins_v2/builtin` declarations | 2414 across 31 kinds | 1783 across 30 kinds |
| source files scanned for elided constructors | 2215 | 1584 |
| nullary declarations expanded | 124 (297 at untested kinds) | 124 (297 at untested kinds) |
| `plugins_v2/canon` cards | 118 | 118 |
| `plugins_v2/testing` cards | 2 | 2 |
| cards read/written/read back | 120 | 120 |

The declaration drop is 631: the 630 files of
`plugins_v2/builtin/macros/flavor_words/` plus `macros/meta/FlavorWord.ron`,
which `Plugin::load` reads as a `Macro`-kind declaration. The kind count falls
by one because `FlavorWord` was a kind.

**Homograph and overlap inventories** (unchanged from the claim base):

- `licensed_vocab_lexicon_homographs` 2:
  - vocab `AttributiveAdjective::Untap` beside Verb declaration keyword action
    `untap` from `plugins_v2/builtin/macros/keyword_actions/untap.ron`
  - vocab `TargetingMarker::Target` beside Noun lexeme `CommonNoun::Target`
- `form_literal_vocab_overlaps` 9: `additional` at `additional_cost` atom 2;
  `to` at `up_to_quantifying_determiner` atom 1; `the` at
  `definite_next_mass_quantity_reference` atom 0; `next` at
  `definite_next_mass_quantity_reference` atom 1; `to` at
  `scalar_less_than_or_equal_to` atom 4; `the` at `number_of_scalar_value`
  atom 0; `the` at `greatest_scalar_value` atom 0; `other` at
  `other_than_qualified_reference` atom 1; `the` at `positional_partitive`
  atom 0.
- `longest_form_literal_bytes` 11.

**Performance advisory.** `PERFORMANCE english-v2 gate=coverage workers=24
elapsed_seconds=47.251 ceiling_seconds=16.260 criterion=quiet_host
host_load_1m=11.43 host_load_5m=11.77 host_load_15m=11.86`. Wall time
**47.251 s** against the 16.26 s quiet-host ceiling — over it, on a host that
was **not** quiet (1-minute load 11.43 with 24 workers; sibling workspaces were
building concurrently). Thread CPU is **189,554 ns/B** accepted
(`accepted_units` 20056, `accepted_bytes` 1938089). The claim-base run on the
same host measured 63.405 s at **224,819 ns/B** with `host_load_1m` 15.54, so
the per-byte figure **fell** across the landing; both runs trip the ceiling
warning for host load, not for a code regression, and neither is a quiet-host
measurement.

### Assurance counts

- **restored:** 0
- **re-spelled:** 1 —
  `deckmaste_english_v2/tests/ability_logic.rs::the_weighted_mode_marker_is_one_construction_for_spree_and_pawprint`.
  Its subject (the weighted mode marker for spree and pawprint) still exists;
  only the now-impossible `ModeMarker::FlavorWord(_)` match arm was dropped.
  Same card, same asserted outcome, new spelling.
- **ignored with blockers:** 0 (the one `ignored` in the gate totals is
  pre-existing and untouched)
- **added:** 1 —
  `deckmaste_lexical/tests/analysis.rs::an_italic_run_no_declaration_spells_is_a_flavor_word`
  (the positive and negative pins the ticket asks for, plus the discriminating
  verb case)
- **removed:** 5, each justified by name — every one's subject is the deleted
  enumerated family, and none has a surviving subject to re-spell against:
  1. `deckmaste_english_v2/tests/flavor_words.rs::flavor_words_cover_plain_chapter_and_mode_label_positions`
     — asserted that `intoTheTARDIS`, `brimstone`, `khans`, `dragons` are
     visited as **flavor-word declarations**. There are no flavor-word
     declarations. Re-covered by `english-v3-whole-grammar-activation`.
  2. `deckmaste_english_v2/tests/flavor_words.rs::flavor_word_class_is_open_but_declaration_backed`
     — asserted that `"Undeclared Signal — Draw a card."` is a **parse
     failure**. This ticket's ruling inverts it exactly: an undeclared italic
     run *is* the flavor word. Its re-spelling is the added
     `deckmaste_lexical` pin, which asserts the opposite outcome on the same
     shape of input.
  3. `deckmaste_construction_core/tests/builtin_v2_flavor_words.rs::builtin_v2_flavor_word_nursery_matches_the_corpus_census`
     — checked the nursery against `deckmaste_data::flavor_words::census`.
     Both nursery and census are deleted.
  4. `xtask/tests/flavor_words.rs::checked_in_flavor_word_nursery_passes_the_generator_check`
     — ran `cargo xtask english_v2 flavor-words --check`. The subcommand is
     deleted.
  5. `xtask/src/bin/cargo-xtask.rs::english_v2_flavor_words_requires_exactly_one_action`
     — CLI-argument test for that same deleted subcommand.

### Breakage inventory (step 1, recorded before repair)

Building the closure immediately after deleting the family, the meta,
`DeclarationKind::FlavorWord`, the `flavor_words` entry in `BUILTIN_FAMILIES`
and the v2-reader/xtask registrations produced, by crate:

| crate / target | failure |
|---|---|
| `deckmaste_english_v2` (lib) | `constructions.rs:937` — `declaration_term kinds must be \`KeywordAbility\`, \`AbilityWord\`, \`CounterKind\`, or \`Designation\`` (the `FlavorWordTerm` codec's `kinds = [FlavorWord]`) |
| `xtask` (lib) | `english_v2/diagnostic.rs:1631` and `:1791` — `E0599` no variant `NonterminalCategory::FlavorWordModeMarker` / `::FlavorWordLabelTerm` |
| `deckmaste_english_v2` (test `flavor_words`) | `tests/flavor_words.rs:90` — `E0599` no variant `DeclarationKind::FlavorWord` |
| `deckmaste_english_v2` (test `ability_logic`) | `tests/ability_logic.rs:4296` — `E0599` no variant `ast::ModeMarker::FlavorWord` |
| `deckmaste_semantics_v2` (test `corpus`) | `no_source_file_writes_out_an_elided_constructor` failed: `only 1584 files checked` against a `>= 2_000` scan floor |

No crate outside english_v2, xtask's english_v2 diagnostics, the deleted
family's own tests and the corpus scan floor broke, so the ticket's STOP
condition was not reached.

### `deckmaste_lexical` (step 3)

Files touched, minimal and additive (the `english-v3-lexical-model` workspace
is extracting the model out of this crate concurrently; **`src/model.rs` was
not touched**):

- `crates/deckmaste_lexical/src/analysis.rs` — adds `pub enum ItalicHead`
  (`AbilityWord(LexicalValue)` / `FlavorWord { label: String }`) and
  `Lexicon::analyze_italic_head`; adds `use crate::Category;`.
- `crates/deckmaste_lexical/tests/analysis.rs` — adds the pin.

The rule: a run spelled end-to-end by a declared `Category::Keyword` Lexeme is
an ability word — the ability words are the CR's listed inventory
[CR#207.2c] and the only declared vocabulary an italic head carries. Every
other run is a flavor word whose label is the run **verbatim**, because
flavor words are listed nowhere [CR#207.2d]. No `Source` of any `SourceKind`
can point at a flavor-word list: the `DeclarationKind::FlavorWord` arm that
minted `lexeme:flavor_word/<name>` owners in
`xtask/src/english_v2/lexical_sources/plugins.rs` is gone with the variant.
`english-v3-lexical-inventory` already carried its exclusion note, so no edit
was needed there.

### Gate

`cargo xtask gate --changed --run` derives, and this landing ran:

```
cargo test -p deckmaste -p deckmaste_data -p deckmaste_catalogs \
  -p deckmaste_construction_core -p deckmaste_construction -p deckmaste_english \
  -p deckmaste_english_v2 -p deckmaste_lexical -p deckmaste_migrations \
  -p deckmaste_semantics_v2 -p deckmaste_spelling -p xtask
```

3353 passed, 0 failed, 1 ignored (pre-existing), exit 0. `cargo fmt --all`
clean. `cargo clippy --all-targets` on the changed crates
(`deckmaste_construction_core`, `deckmaste_data`, `deckmaste_english_v2`,
`deckmaste_lexical`, `deckmaste_semantics_v2`, `xtask`) exits 0; the two
`clippy::pedantic` warnings it prints are pre-existing in
`deckmaste_construction_core/tests/builtin_v2_keyword_abilities.rs`, a file
this landing does not touch. `cargo xtask cite check --list-noncompliant`
empty; `cite check` 0 stale after `cite bless` registered [CR#207.2b] (read
against its claim: it defines Flavor Text, which the new **Flavor Word**
entry's `_Avoid_` line distinguishes). `jj diff --git | cargo xtask cite audit
--diff` audited 9 citation sites; each rule's text was read against the claim
citing it and supports it.
