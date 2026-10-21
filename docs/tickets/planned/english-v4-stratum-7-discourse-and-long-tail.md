---
needs: [english-v4-stratum-6-coordination-and-rnr, english-v4-retire-adj-target, english-v4-slash-measure-overgeneration, english-v4-noun-phrase-coordination-determination, english-v4-valued-noun-phrases, english-v4-adverb-so-overgeneration, english-v4-mixed-voice-participle-coordination, english-v4-unset-number-and-countability, english-v4-banish-grammar-policing, english-v4-spurious-ambiguity-multipliers, english-v4-clause-tense-selection, english-v4-zero-plural-determiner-number, english-v4-passive-by-agent-vs-by-extent, english-v4-amount-of-excess-bare-nominal, english-v4-color-word-noun-vs-adjective, english-v4-temporal-np-as-direct-object, english-v4-unpunctuated-clause-fragment, english-v4-existential-locative-ambiguity, english-v4-coordinated-clause-tense, english-v4-next-time-replacement-clause, english-v4-omitted-idiom-complement-split, english-v4-temporal-adjunct-tense-harmony, english-v4-case-solve-condition, english-v4-wave1-review-cleanups]
---
# Stratum 7: Discourse sequencing, replacement effects, and the corpus long tail

Implement **Stratum 7** in `crates/deckmaste_english_v4` per
[`docs/decisions/english-v4.md`](../../decisions/english-v4.md) (§6):

- **Replacement and Prevention Effects:** `If <Clause> would <VP>, <Clause>
  instead` and `Prevent the next N damage that would be dealt ...`.
- **Discourse Sequencing and Pro-Forms:** `Then ...`, `Otherwise, ...`, `If you
  do, ...`, `If you don't, ...`, `When you do, ...`, `Repeat this process ...`,
  and auxiliary stranding / `do so`.
- **Anaphora, Supplements, and Quoted Grants:** Reflexive/reciprocal pronouns
  (`itself`, `themselves`, `each other`), `with` / `without` counter and ability
  supplements (`enters with two +1/+1 counters on it`), and embedded quoted
  abilities (`gains "<Document>"` and `"<Ability>," "<Ability>," and "<Ability>"`).
- **Deferred from Stratum 6:** a Preposition Phrase complement extraposed past a
  relative clause (*Equip abilities you activate of other Equipment*, Bladehold
  War-Whip, in `deferred.txt`); and coordinated counter-kind specifiers before
  *counter* (*your choice of a +1/+1, first strike, or vigilance counter*), which
  needs the specifier exposed without splitting the declared compound counter
  lexemes (five units in `deferred.txt`).
- **Also deferred from Stratum 6** (each unit in `deferred.txt`, by the
  mechanism it needs):
  - Ellipsis or gapping: Multiclass Baldric; Scion of Draco; Dominaria's Judgment; Akroma, Vision of Ixidor; Selective Adaptation; Detention Vortex; Oft-Nabbed Goat; Hearth Elemental; Frantic Firebolt.
  - Gapped *get* over two Object + Adjunct strings (*each get +1/+1 for each
    creature you control and +1/+1 for each Aura you control*): Eidolon of
    Countless Battles reads only through the wrong *for each [creature you
    control and +1/+1]*. It is not in `deferred.txt`, because a judgment would
    leave only a known-wrong reading. Retire that reading when the gapped
    analysis lands.
  - A determined measure nominal as a nominal conjunct (*your [choice of 5/1]
    or [1/5]*): Aquamorph Entity keeps this Reading beside the endorsed
    *choice of [5/1 or 1/5]*.
  - The Targeting Marker determining a nominal that needs an overt determiner
    of its own (`english-v4-targeting-marker-excludes` findings): Cloud,
    Planet's Champion (*that target Cloud cost {2} less*); Frodo, Determined
    Hero (*target [Equipment … mana value 2] or [3]*); Eternal Flame
    (*[opponent or planeswalker and half] X damage*). Each keeps its endorsed
    Reading beside the wrong one. Fix this without a consumer exclusion.
  - Elided second *with*: Ulasht, the Hate Seed.
  - Floating quantifier: Kutzil, Malamet Exemplar; Angelic Aberration; Leonardo's Technique; Dance of the Manse; Ballroom Brawlers; Keen Duelist; Storyweave.
  - Vote option or metalinguistic mention: Bite of the Black Rose; Lieutenants of the Guard; Capital Punishment; Trial of a Time Lord; Abundant Harvest.
  - Villainous choice dash alternatives: Midnight Crusader Shuttle.
  - *whichever comes first / is greater* supplement: Thought Dissector; Willowdusk, Essence Seer; Triumphant Chomp; Prophet of the Scarab.
  - Counter kinds listed as a choice: Crystalline Giant.
  - Mixed-mood serial clauses whose first conjunct ends in a subordinate clause
    that could take the rest (scope): Opportunistic Dragon.
  - *enters with flying and with X counters*, a *with*-keyword phrase as a
    dependent of *enter* (a complement frame for *enter* breaks Master
    Biomancer's *with … and as a Mutant*; a predicate host for *with* plus a
    keyword grows 400 units): Dralnu's Pet.
  - Fused-head *those* with a relative clause, and the scope of *except* in a
    serial object: Everything Comes to Dust.
  - A frequency NP coordinated with an *until* PP (*that many times or until
    you lose a flip*), and the *whichever comes first* supplement: Squee's
    Revenge.
  - A singular joint coordination under *a* (*is also a Cleric, Rogue,
    Warrior, and Wizard*; CGEL, Ch. 5 §18.4, p. 508): an article over an
    *and*-coordination of singular nominals, even as a predicative only, also
    admits *becomes a [Human Soldier creature with power] and [toughness
    each equal to …]* on ~100 units: Stonework Packbeast; Burakos, Party
    Leader; Tajuru Paragon; Veteran Adventurer; and, under a singular
    determiner, Lush Growth (*a Mountain, Forest, and Plains*) and Wild Shape
    (*that base power and toughness*), which read only through a spurious
    Noun Phrase coordination until Stratum 6 ruled it out.
  - Non-coordination one-off (38 units, approved by the owner): Gorex, the Tombshell; Eye of Duskmantle; Portal Manipulator; Expand the Sphere; Niv-Mizzet Reborn; Spikeshell Harrier; Avatar of Woe; Distended Mindbender; Banshee; Fork; Berserker's Frenzy; Cosmos Charger; Dance with Calamity; Iron Mastiff; Fiery Gambit; Moritte of the Frost; Loxodon Peacekeeper; Celestial Convergence; Impact Resonance; Worms of the Earth; Ob Nixilis, the Adversary; Tameshi, Reality Architect; Multiple Choice; Game of Chaos; Time and Tide; Elenda, Saint of Dusk; The Enigma Jewel; Scrying Glass; Tempt with Mayhem; Withering Wisps; Arcum's Whistle; Tranquil Frillback; All Hallow's Eve; Chrome Replicator; Crabomination; Chaos Defiler; Immard, the Stormcleaver; Shay Cormac.
  - Serial unlike coordination beyond object lists: predicate adjuncts
    (*only during …, only if …, and only once each turn*), Grizzled Wolverine;
    an NP and a PP under *and/or* (*library, graveyard, and/or outside the
    game*), Invasion of Arcavios; an adjective with subtype attributes (*the
    basic, Sphere, and Locus lands*), Monument to Perfection.
  - Bare mentions of choice options coordinated as a bare singular term
    (*choose left or right*, *choose artifact or enchantment*; a single bare
    option reads nowhere either), with the vote and mention family above: Storage Matrix; Stenn, Paranoid Partisan; Mystic Barrier; Teyo, Geometric Tactician; Orchard Elemental; Order of Succession; Virtus's Maneuver; Cloud Key; Dawn-Blessed Pennant; Pramikon, Sky Rampart; Aminatou, the Fateshifter; Season of Gathering; Winding Way; Eclipsed Realms; Turnabout; Zndrsplt's Judgment; Teferi's Realm; Archon of Valor's Reach; Messenger Jays; Sarpadian Empires, Vol. VII; Fatespinner (*draw step, main phase, or combat phase*).
  - A summed total over coordinated property nouns (*total power and
    toughness 5 or less*): *total* modifies [power and toughness] as a whole,
    and the Amount bounds that sum, not each noun. Cut Down and Meek Attack are
    deferred here; their judgments in `coordination.txt` assert that analysis.
  - Felisa, Fang of Silverquill: the second analysis of *the number of
    counters it had on it*, *have* with an object and a locative *on*-phrase
    (*it had __ on it*), without a general *have NP on-PP* frame. Stratum 6
    reads the relative as *it had __*, gapped on *counters*, with *on it* a
    postmodifier of the nominal outside the relative; its judgment in `perfect_relative_adverbial_gap.txt` asserts that
    analysis (`RelativeClause "it had" gap "counters"`) and drops the
    alternative *NounPhrase "counters"* object until this lands.
  - The Targeting Marker's scope over a coordination when the marked Noun
    Phrase sits inside a larger first conjunct (*[control of X target
    creatures] and/or [planeswalkers]*, *[each of up to two target creatures]
    and/or [planeswalkers]*): mark a conjunct whose right edge is a targeted
    Noun Phrase. Twelve units keep a narrow reading beside the wide one:
    Meteor Swarm; Nahiri's Wrath; Saheeli, the Sun's Brilliance; Mass
    Manipulation; Shatterskull Smashing; Ignite Disorder; Volcanic Salvo;
    Zimone, Paradox Sculptor; Captain's Maneuver; Spinning Wheel Kick; Etched
    Slith; Cloudspire Skycycle.
- **Profiled into Stratum 7 by Stratum 6** (`src/profile.rs`, by rule):
  - *The same is true for …* as a pro-form (25 units; the owner approved
    them for Stratum 7, which owns discourse-anaphoric clauses): Animus of Predation; Arcane Adaptation; Biotransference; Bleeding
    Effect; Cairn Wanderer; Celestial Dawn; Concerted Effort; Conspiracy;
    Death-Mask Duplicant; Eater of Virtue; Encroaching Mycosynth; Escaped
    Shapeshifter; Indominus Rex, Alpha; Leyline of Transformation; Majestic
    Myriarch; Maskwood Nexus; Odric, Lunarch Marshal; Rayami, First of the
    Fallen; Roshan, Hidden Magister; Rukarumel, Biologist; Soulflayer;
    Thunderous Orator; Urborg Scavengers; Ward of Bones; Wretched Bonemass.
  - An elliptical *but not* / *but only* coordinate (12): Armageddon Clock;
    Drain Life; Endbringer's Revel; Haakon, Stromgald Scourge; Infinite
    Hourglass; Lightning Storm; Mana Cache; Scandalmonger; Soul Burn; Task Mage
    Assembly; Volrath's Dungeon; Well of Knowledge.
  - Listed vote options that are no nouns (5): Path of the Animist; Path of the
    Enigma; Path of the Ghosthunter; Path of the Pyromancer; Path of the
    Schemer.
  - Text-changing *replacing … instances of* (13 units): Alter Reality; Artificial
    Evolution; Balduvian Shaman; Crystal Spray; Glamerdye; Magical Hack; Mind
    Bend; New Blood; Sleight of Mind; Spectral Shift; Trait Doctoring; Whim of
    Volrath.
- **Closing the Corpus Tail Under the Long-Tail Rule:** Cover the remaining
  Stratum 7 families without introducing policing flags (`...Use`, `...Licence`,
  `...Present`). If any outlier card's wording cannot be parsed by a clean
  general schema/production or a catalog-declared lexical idiom without a
  policing flag, **STOP and surface the witness to the user** to decide between a
  catalog idiom, a general production, or an explicit documented deferral
  (`unread`).

## Gate

- 100% of non-deferred Vintage-legal Oracle corpus units across Strata 0–7 parse
  as **read** (0 `known-wrong-only`) and satisfy both round-trip laws
  (`v is an admitted Reading of s => render(v) = s` and
  `v is independently constructed and grammatical => v is a Reading of parse(render(v))`).
- Full linguistic judgment suite passes.
- Zero CR-cataloged words appear directly in the grammar data file, and zero
  policing flags exist.

Standard constraints apply.
