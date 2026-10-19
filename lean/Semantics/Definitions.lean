import Semantics.Templates

/-!
# Semantics.Definitions

Rules-defined vocabulary (`Definition`s, `semantic_definition`) for `semantics-v3`
(`docs/decisions/semantics-v3.md` §§1–3): CR §701 keyword actions, CR §702 keyword abilities, and
the counters, tokens and designations the rules define. A Definition evaluates in its own
Reference Scope (`ownScope` + `inCaller`) and is spelled over `Semantics.Templates`.
-/

namespace Semantics.Definitions

open Semantics.Templates

/-! ## Counters -/

/-- A counter declaration's name read as its kind (`CounterKind.named`), so a bare
`p1p1Counter` is the +1/+1 counter's kind [CR#122.1]. Any other kind is written
`.named "<declaration name>"`. -/
semantic_definition p1p1Counter : CounterKind := .named "p1p1Counter"
semantic_definition flyingCounter : CounterKind := .named "flyingCounter"
semantic_definition m1m1Counter : CounterKind := .named "m1m1Counter"
semantic_definition acornCounter : CounterKind := .named "acornCounter"
semantic_definition aegisCounter : CounterKind := .named "aegisCounter"
semantic_definition ageCounter : CounterKind := .named "ageCounter"
semantic_definition aimCounter : CounterKind := .named "aimCounter"
semantic_definition arrowCounter : CounterKind := .named "arrowCounter"
semantic_definition arrowheadCounter : CounterKind := .named "arrowheadCounter"
semantic_definition awakeningCounter : CounterKind := .named "awakeningCounter"
semantic_definition baitCounter : CounterKind := .named "baitCounter"
semantic_definition blazeCounter : CounterKind := .named "blazeCounter"
semantic_definition blessingCounter : CounterKind := .named "blessingCounter"
semantic_definition bloodCounter : CounterKind := .named "bloodCounter"
semantic_definition bloodlineCounter : CounterKind := .named "bloodlineCounter"
semantic_definition bloodstainCounter : CounterKind := .named "bloodstainCounter"
semantic_definition bookCounter : CounterKind := .named "bookCounter"
semantic_definition boreCounter : CounterKind := .named "boreCounter"
semantic_definition bountyCounter : CounterKind := .named "bountyCounter"
semantic_definition brainCounter : CounterKind := .named "brainCounter"
semantic_definition briberyCounter : CounterKind := .named "briberyCounter"
semantic_definition brickCounter : CounterKind := .named "brickCounter"
semantic_definition burdenCounter : CounterKind := .named "burdenCounter"
semantic_definition cageCounter : CounterKind := .named "cageCounter"
semantic_definition carrionCounter : CounterKind := .named "carrionCounter"
semantic_definition cellCounter : CounterKind := .named "cellCounter"
semantic_definition chargeCounter : CounterKind := .named "chargeCounter"
semantic_definition chorusCounter : CounterKind := .named "chorusCounter"
semantic_definition collectionCounter : CounterKind := .named "collectionCounter"
semantic_definition componentCounter : CounterKind := .named "componentCounter"
semantic_definition conquerorCounter : CounterKind := .named "conquerorCounter"
semantic_definition contestedCounter : CounterKind := .named "contestedCounter"
semantic_definition corpseCounter : CounterKind := .named "corpseCounter"
semantic_definition corruptionCounter : CounterKind := .named "corruptionCounter"
semantic_definition creditCounter : CounterKind := .named "creditCounter"
semantic_definition croakCounter : CounterKind := .named "croakCounter"
semantic_definition crystalCounter : CounterKind := .named "crystalCounter"
semantic_definition cubeCounter : CounterKind := .named "cubeCounter"
semantic_definition currencyCounter : CounterKind := .named "currencyCounter"
semantic_definition deathtouchCounter : CounterKind := .named "deathtouchCounter"
semantic_definition decayedCounter : CounterKind := .named "decayedCounter"
semantic_definition defenseCounter : CounterKind := .named "defenseCounter"
semantic_definition delayCounter : CounterKind := .named "delayCounter"
semantic_definition depletionCounter : CounterKind := .named "depletionCounter"
semantic_definition descentCounter : CounterKind := .named "descentCounter"
semantic_definition despairCounter : CounterKind := .named "despairCounter"
semantic_definition discoveryCounter : CounterKind := .named "discoveryCounter"
semantic_definition divinityCounter : CounterKind := .named "divinityCounter"
semantic_definition doomCounter : CounterKind := .named "doomCounter"
semantic_definition doubleStrikeCounter : CounterKind := .named "doubleStrikeCounter"
semantic_definition dreadCounter : CounterKind := .named "dreadCounter"
semantic_definition dreamCounter : CounterKind := .named "dreamCounter"
semantic_definition dutyCounter : CounterKind := .named "dutyCounter"
semantic_definition eggCounter : CounterKind := .named "eggCounter"
semantic_definition elixirCounter : CounterKind := .named "elixirCounter"
semantic_definition emberCounter : CounterKind := .named "emberCounter"
semantic_definition energy : CounterKind := .named "energy"
semantic_definition enlightenedCounter : CounterKind := .named "enlightenedCounter"
semantic_definition eonCounter : CounterKind := .named "eonCounter"
semantic_definition exaltedCounter : CounterKind := .named "exaltedCounter"
semantic_definition experience : CounterKind := .named "experience"
semantic_definition eyeballCounter : CounterKind := .named "eyeballCounter"
semantic_definition fadeCounter : CounterKind := .named "fadeCounter"
semantic_definition fateCounter : CounterKind := .named "fateCounter"
semantic_definition featherCounter : CounterKind := .named "featherCounter"
semantic_definition feedingCounter : CounterKind := .named "feedingCounter"
semantic_definition fellowshipCounter : CounterKind := .named "fellowshipCounter"
semantic_definition fetchCounter : CounterKind := .named "fetchCounter"
semantic_definition filibusterCounter : CounterKind := .named "filibusterCounter"
semantic_definition filmCounter : CounterKind := .named "filmCounter"
semantic_definition finalityCounter : CounterKind := .named "finalityCounter"
semantic_definition fireCounter : CounterKind := .named "fireCounter"
semantic_definition firstStrikeCounter : CounterKind := .named "firstStrikeCounter"
semantic_definition flameCounter : CounterKind := .named "flameCounter"
semantic_definition floodCounter : CounterKind := .named "floodCounter"
semantic_definition foreshadowCounter : CounterKind := .named "foreshadowCounter"
semantic_definition fungusCounter : CounterKind := .named "fungusCounter"
semantic_definition furyCounter : CounterKind := .named "furyCounter"
semantic_definition fuseCounter : CounterKind := .named "fuseCounter"
semantic_definition gemCounter : CounterKind := .named "gemCounter"
semantic_definition ghostformCounter : CounterKind := .named "ghostformCounter"
semantic_definition glyphCounter : CounterKind := .named "glyphCounter"
semantic_definition goldCounter : CounterKind := .named "goldCounter"
semantic_definition growthCounter : CounterKind := .named "growthCounter"
semantic_definition harmonyCounter : CounterKind := .named "harmonyCounter"
semantic_definition hasteCounter : CounterKind := .named "hasteCounter"
semantic_definition hatchingCounter : CounterKind := .named "hatchingCounter"
semantic_definition hatchlingCounter : CounterKind := .named "hatchlingCounter"
semantic_definition hexproofCounter : CounterKind := .named "hexproofCounter"
semantic_definition hitCounter : CounterKind := .named "hitCounter"
semantic_definition honeCounter : CounterKind := .named "honeCounter"
semantic_definition hoofprintCounter : CounterKind := .named "hoofprintCounter"
semantic_definition hopeCounter : CounterKind := .named "hopeCounter"
semantic_definition hourCounter : CounterKind := .named "hourCounter"
semantic_definition hourglassCounter : CounterKind := .named "hourglassCounter"
semantic_definition hungerCounter : CounterKind := .named "hungerCounter"
semantic_definition iceCounter : CounterKind := .named "iceCounter"
semantic_definition impostorCounter : CounterKind := .named "impostorCounter"
semantic_definition incarnationCounter : CounterKind := .named "incarnationCounter"
semantic_definition incubationCounter : CounterKind := .named "incubationCounter"
semantic_definition indestructibleCounter : CounterKind := .named "indestructibleCounter"
semantic_definition infectionCounter : CounterKind := .named "infectionCounter"
semantic_definition influenceCounter : CounterKind := .named "influenceCounter"
semantic_definition ingenuityCounter : CounterKind := .named "ingenuityCounter"
semantic_definition ingredientCounter : CounterKind := .named "ingredientCounter"
semantic_definition intelCounter : CounterKind := .named "intelCounter"
semantic_definition interventionCounter : CounterKind := .named "interventionCounter"
semantic_definition invasionCounter : CounterKind := .named "invasionCounter"
semantic_definition invitationCounter : CounterKind := .named "invitationCounter"
semantic_definition isolationCounter : CounterKind := .named "isolationCounter"
semantic_definition javelinCounter : CounterKind := .named "javelinCounter"
semantic_definition judgmentCounter : CounterKind := .named "judgmentCounter"
semantic_definition kiCounter : CounterKind := .named "kiCounter"
semantic_definition kickCounter : CounterKind := .named "kickCounter"
semantic_definition knowledgeCounter : CounterKind := .named "knowledgeCounter"
semantic_definition landmarkCounter : CounterKind := .named "landmarkCounter"
semantic_definition levelCounter : CounterKind := .named "levelCounter"
semantic_definition lifelinkCounter : CounterKind := .named "lifelinkCounter"
semantic_definition lootCounter : CounterKind := .named "lootCounter"
semantic_definition loreCounter : CounterKind := .named "loreCounter"
semantic_definition loyaltyCounter : CounterKind := .named "loyaltyCounter"
semantic_definition luckCounter : CounterKind := .named "luckCounter"
semantic_definition magnetCounter : CounterKind := .named "magnetCounter"
semantic_definition manifestationCounter : CounterKind := .named "manifestationCounter"
semantic_definition mannequinCounter : CounterKind := .named "mannequinCounter"
semantic_definition matrixCounter : CounterKind := .named "matrixCounter"
semantic_definition memoryCounter : CounterKind := .named "memoryCounter"
semantic_definition menaceCounter : CounterKind := .named "menaceCounter"
semantic_definition mineCounter : CounterKind := .named "mineCounter"
semantic_definition miningCounter : CounterKind := .named "miningCounter"
semantic_definition mireCounter : CounterKind := .named "mireCounter"
semantic_definition musicCounter : CounterKind := .named "musicCounter"
semantic_definition musterCounter : CounterKind := .named "musterCounter"
semantic_definition nestCounter : CounterKind := .named "nestCounter"
semantic_definition netCounter : CounterKind := .named "netCounter"
semantic_definition nightCounter : CounterKind := .named "nightCounter"
semantic_definition oilCounter : CounterKind := .named "oilCounter"
semantic_definition omenCounter : CounterKind := .named "omenCounter"
semantic_definition oreCounter : CounterKind := .named "oreCounter"
semantic_definition pageCounter : CounterKind := .named "pageCounter"
semantic_definition painCounter : CounterKind := .named "painCounter"
semantic_definition palliationCounter : CounterKind := .named "palliationCounter"
semantic_definition paralyzationCounter : CounterKind := .named "paralyzationCounter"
semantic_definition petalCounter : CounterKind := .named "petalCounter"
semantic_definition petrificationCounter : CounterKind := .named "petrificationCounter"
semantic_definition phylacteryCounter : CounterKind := .named "phylacteryCounter"
semantic_definition phyresisCounter : CounterKind := .named "phyresisCounter"
semantic_definition pinCounter : CounterKind := .named "pinCounter"
semantic_definition plagueCounter : CounterKind := .named "plagueCounter"
semantic_definition planCounter : CounterKind := .named "planCounter"
semantic_definition pointCounter : CounterKind := .named "pointCounter"
semantic_definition poison : CounterKind := .named "poison"
semantic_definition polypCounter : CounterKind := .named "polypCounter"
semantic_definition possessionCounter : CounterKind := .named "possessionCounter"
semantic_definition pressureCounter : CounterKind := .named "pressureCounter"
semantic_definition preyCounter : CounterKind := .named "preyCounter"
semantic_definition pupaCounter : CounterKind := .named "pupaCounter"
semantic_definition questCounter : CounterKind := .named "questCounter"
semantic_definition radCounter : CounterKind := .named "radCounter"
semantic_definition rallyCounter : CounterKind := .named "rallyCounter"
semantic_definition reachCounter : CounterKind := .named "reachCounter"
semantic_definition refineCounter : CounterKind := .named "refineCounter"
semantic_definition rejectionCounter : CounterKind := .named "rejectionCounter"
semantic_definition reprieveCounter : CounterKind := .named "reprieveCounter"
semantic_definition revCounter : CounterKind := .named "revCounter"
semantic_definition revivalCounter : CounterKind := .named "revivalCounter"
semantic_definition ribbonCounter : CounterKind := .named "ribbonCounter"
semantic_definition ritualCounter : CounterKind := .named "ritualCounter"
semantic_definition ropeCounter : CounterKind := .named "ropeCounter"
semantic_definition rustCounter : CounterKind := .named "rustCounter"
semantic_definition saurianCounter : CounterKind := .named "saurianCounter"
semantic_definition screamCounter : CounterKind := .named "screamCounter"
semantic_definition shadowCounter : CounterKind := .named "shadowCounter"
semantic_definition shellCounter : CounterKind := .named "shellCounter"
semantic_definition shieldCounter : CounterKind := .named "shieldCounter"
semantic_definition shredCounter : CounterKind := .named "shredCounter"
semantic_definition silverCounter : CounterKind := .named "silverCounter"
semantic_definition skewerCounter : CounterKind := .named "skewerCounter"
semantic_definition sleepCounter : CounterKind := .named "sleepCounter"
semantic_definition sleightCounter : CounterKind := .named "sleightCounter"
semantic_definition slimeCounter : CounterKind := .named "slimeCounter"
semantic_definition slumberCounter : CounterKind := .named "slumberCounter"
semantic_definition sootCounter : CounterKind := .named "sootCounter"
semantic_definition soulCounter : CounterKind := .named "soulCounter"
semantic_definition spiteCounter : CounterKind := .named "spiteCounter"
semantic_definition sporeCounter : CounterKind := .named "sporeCounter"
semantic_definition stashCounter : CounterKind := .named "stashCounter"
semantic_definition storageCounter : CounterKind := .named "storageCounter"
semantic_definition storyCounter : CounterKind := .named "storyCounter"
semantic_definition strifeCounter : CounterKind := .named "strifeCounter"
semantic_definition strikeCounter : CounterKind := .named "strikeCounter"
semantic_definition studyCounter : CounterKind := .named "studyCounter"
semantic_definition stunCounter : CounterKind := .named "stunCounter"
semantic_definition supplyCounter : CounterKind := .named "supplyCounter"
semantic_definition suspectCounter : CounterKind := .named "suspectCounter"
semantic_definition takeoverCounter : CounterKind := .named "takeoverCounter"
semantic_definition taskCounter : CounterKind := .named "taskCounter"
semantic_definition theftCounter : CounterKind := .named "theftCounter"
semantic_definition tideCounter : CounterKind := .named "tideCounter"
semantic_definition timeCounter : CounterKind := .named "timeCounter"
semantic_definition towerCounter : CounterKind := .named "towerCounter"
semantic_definition trampleCounter : CounterKind := .named "trampleCounter"
semantic_definition trapCounter : CounterKind := .named "trapCounter"
semantic_definition treasureCounter : CounterKind := .named "treasureCounter"
semantic_definition unityCounter : CounterKind := .named "unityCounter"
semantic_definition valorCounter : CounterKind := .named "valorCounter"
semantic_definition velocityCounter : CounterKind := .named "velocityCounter"
semantic_definition verseCounter : CounterKind := .named "verseCounter"
semantic_definition vigilanceCounter : CounterKind := .named "vigilanceCounter"
semantic_definition vitalityCounter : CounterKind := .named "vitalityCounter"
semantic_definition voidCounter : CounterKind := .named "voidCounter"
semantic_definition vortexCounter : CounterKind := .named "vortexCounter"
semantic_definition vowCounter : CounterKind := .named "vowCounter"
semantic_definition voyageCounter : CounterKind := .named "voyageCounter"
semantic_definition wageCounter : CounterKind := .named "wageCounter"
semantic_definition winchCounter : CounterKind := .named "winchCounter"
semantic_definition windCounter : CounterKind := .named "windCounter"
semantic_definition wishCounter : CounterKind := .named "wishCounter"

/-! ## Designations -/

semantic_definition citysBlessing : DesignationLabel := .named "citysBlessing"
semantic_definition commander : DesignationLabel := .named "commander"
/-- Day and night, the designations the game itself holds, exactly one once either is held
[CR#731.1]. -/
semantic_definition day : DesignationLabel := .named "day"
semantic_definition night : DesignationLabel := .named "night"
semantic_definition enduringStory : DesignationLabel := .named "enduringStory"
semantic_definition goaded : DesignationLabel := .named "goaded"
semantic_definition harnessed : DesignationLabel := .named "harnessed"
semantic_definition initiative : DesignationLabel := .named "initiative"
semantic_definition leftHalfUnlocked : DesignationLabel := .named "leftHalfUnlocked"
semantic_definition level : DesignationLabel := .named "level"
semantic_definition monarch : DesignationLabel := .named "monarch"
semantic_definition monstrous : DesignationLabel := .named "monstrous"
semantic_definition prepared : DesignationLabel := .named "prepared"
semantic_definition renowned : DesignationLabel := .named "renowned"
semantic_definition rightHalfUnlocked : DesignationLabel := .named "rightHalfUnlocked"
semantic_definition ringBearer : DesignationLabel := .named "ringBearer"
semantic_definition saddled : DesignationLabel := .named "saddled"
semantic_definition DesignationLabel.solved : DesignationLabel := .named "solved"
semantic_definition suspected : DesignationLabel := .named "suspected"

/-! ## Relations -/

semantic_definition encodedOn : RelationLabel := .named "encodedOn"
semantic_definition haunting : RelationLabel := .named "haunting"
semantic_definition paired : RelationLabel := .named "paired"

/-! ## Subtypes -/

semantic_definition blood : Subtype := artifactType "Blood"
semantic_definition bobblehead : Subtype := artifactType "Bobblehead"
semantic_definition book : Subtype := artifactType "Book"
semantic_definition clue : Subtype := artifactType "Clue"
semantic_definition contraption : Subtype := artifactType "Contraption"
semantic_definition equipment : Subtype := artifactType "Equipment"
semantic_definition food : Subtype := artifactType "Food"
semantic_definition fortification : Subtype := artifactType "Fortification"
semantic_definition gold : Subtype := artifactType "Gold"
semantic_definition incubator : Subtype := artifactType "Incubator"
semantic_definition Subtype.infinity : Subtype := artifactType "Infinity"
semantic_definition junk : Subtype := artifactType "Junk"
semantic_definition lander : Subtype := artifactType "Lander"
semantic_definition map : Subtype := artifactType "Map"
semantic_definition mutagen : Subtype := artifactType "Mutagen"
semantic_definition powerstone : Subtype := artifactType "Powerstone"
semantic_definition spacecraft : Subtype := artifactType "Spacecraft"
semantic_definition stone : Subtype := artifactType "Stone"
semantic_definition treasure : Subtype := artifactType "Treasure"
semantic_definition vehicle : Subtype := artifactType "Vehicle"
semantic_definition vibranium : Subtype := artifactType "Vibranium"
semantic_definition siege : Subtype := .of .battle "Siege"
semantic_definition advisor : Subtype := creatureType "Advisor"
semantic_definition aetherborn : Subtype := creatureType "Aetherborn"
semantic_definition alien : Subtype := creatureType "Alien"
semantic_definition ally : Subtype := creatureType "Ally"
semantic_definition angel : Subtype := creatureType "Angel"
semantic_definition antelope : Subtype := creatureType "Antelope"
semantic_definition ape : Subtype := creatureType "Ape"
semantic_definition archer : Subtype := creatureType "Archer"
semantic_definition archon : Subtype := creatureType "Archon"
semantic_definition armadillo : Subtype := creatureType "Armadillo"
semantic_definition artificer : Subtype := creatureType "Artificer"
semantic_definition assassin : Subtype := creatureType "Assassin"
semantic_definition assemblyWorker : Subtype := creatureType "Assembly-Worker"
semantic_definition astartes : Subtype := creatureType "Astartes"
semantic_definition atog : Subtype := creatureType "Atog"
semantic_definition aurochs : Subtype := creatureType "Aurochs"
semantic_definition avatar : Subtype := creatureType "Avatar"
semantic_definition azra : Subtype := creatureType "Azra"
semantic_definition badger : Subtype := creatureType "Badger"
semantic_definition balloon : Subtype := creatureType "Balloon"
semantic_definition barbarian : Subtype := creatureType "Barbarian"
semantic_definition bard : Subtype := creatureType "Bard"
semantic_definition basilisk : Subtype := creatureType "Basilisk"
semantic_definition bat : Subtype := creatureType "Bat"
semantic_definition bear : Subtype := creatureType "Bear"
semantic_definition beast : Subtype := creatureType "Beast"
semantic_definition beaver : Subtype := creatureType "Beaver"
semantic_definition beeble : Subtype := creatureType "Beeble"
semantic_definition beholder : Subtype := creatureType "Beholder"
semantic_definition berserker : Subtype := creatureType "Berserker"
semantic_definition bird : Subtype := creatureType "Bird"
semantic_definition bison : Subtype := creatureType "Bison"
semantic_definition blinkmoth : Subtype := creatureType "Blinkmoth"
semantic_definition boar : Subtype := creatureType "Boar"
semantic_definition bringer : Subtype := creatureType "Bringer"
semantic_definition brushwagg : Subtype := creatureType "Brushwagg"
semantic_definition camarid : Subtype := creatureType "Camarid"
semantic_definition camel : Subtype := creatureType "Camel"
semantic_definition capybara : Subtype := creatureType "Capybara"
semantic_definition caribou : Subtype := creatureType "Caribou"
semantic_definition carrier : Subtype := creatureType "Carrier"
semantic_definition cat : Subtype := creatureType "Cat"
semantic_definition centaur : Subtype := creatureType "Centaur"
semantic_definition child : Subtype := creatureType "Child"
semantic_definition chimera : Subtype := creatureType "Chimera"
semantic_definition citizen : Subtype := creatureType "Citizen"
semantic_definition cleric : Subtype := creatureType "Cleric"
semantic_definition clown : Subtype := creatureType "Clown"
semantic_definition cockatrice : Subtype := creatureType "Cockatrice"
semantic_definition construct : Subtype := creatureType "Construct"
semantic_definition coward : Subtype := creatureType "Coward"
semantic_definition coyote : Subtype := creatureType "Coyote"
semantic_definition crab : Subtype := creatureType "Crab"
semantic_definition crocodile : Subtype := creatureType "Crocodile"
semantic_definition ctan : Subtype := creatureType "C'tan"
semantic_definition custodes : Subtype := creatureType "Custodes"
semantic_definition cyberman : Subtype := creatureType "Cyberman"
semantic_definition cyclops : Subtype := creatureType "Cyclops"
semantic_definition dalek : Subtype := creatureType "Dalek"
semantic_definition dauthi : Subtype := creatureType "Dauthi"
semantic_definition demigod : Subtype := creatureType "Demigod"
semantic_definition demon : Subtype := creatureType "Demon"
semantic_definition deserter : Subtype := creatureType "Deserter"
semantic_definition detective : Subtype := creatureType "Detective"
semantic_definition devil : Subtype := creatureType "Devil"
semantic_definition dinosaur : Subtype := creatureType "Dinosaur"
semantic_definition djinn : Subtype := creatureType "Djinn"
semantic_definition doctor : Subtype := creatureType "Doctor"
semantic_definition dog : Subtype := creatureType "Dog"
semantic_definition dragon : Subtype := creatureType "Dragon"
semantic_definition drake : Subtype := creatureType "Drake"
semantic_definition dreadnought : Subtype := creatureType "Dreadnought"
semantic_definition drix : Subtype := creatureType "Drix"
semantic_definition drone : Subtype := creatureType "Drone"
semantic_definition druid : Subtype := creatureType "Druid"
semantic_definition dryad : Subtype := creatureType "Dryad"
semantic_definition dwarf : Subtype := creatureType "Dwarf"
semantic_definition echidna : Subtype := creatureType "Echidna"
semantic_definition efreet : Subtype := creatureType "Efreet"
semantic_definition egg : Subtype := creatureType "Egg"
semantic_definition elder : Subtype := creatureType "Elder"
semantic_definition eldrazi : Subtype := creatureType "Eldrazi"
semantic_definition elemental : Subtype := creatureType "Elemental"
semantic_definition elephant : Subtype := creatureType "Elephant"
semantic_definition elf : Subtype := creatureType "Elf"
semantic_definition elk : Subtype := creatureType "Elk"
semantic_definition employee : Subtype := creatureType "Employee"
semantic_definition eternal : Subtype := creatureType "Eternal"
semantic_definition eye : Subtype := creatureType "Eye"
semantic_definition faerie : Subtype := creatureType "Faerie"
semantic_definition ferret : Subtype := creatureType "Ferret"
semantic_definition fish : Subtype := creatureType "Fish"
semantic_definition flagbearer : Subtype := creatureType "Flagbearer"
semantic_definition fox : Subtype := creatureType "Fox"
semantic_definition fractal : Subtype := creatureType "Fractal"
semantic_definition frog : Subtype := creatureType "Frog"
semantic_definition fungus : Subtype := creatureType "Fungus"
semantic_definition gamer : Subtype := creatureType "Gamer"
semantic_definition gamma : Subtype := creatureType "Gamma"
semantic_definition gargoyle : Subtype := creatureType "Gargoyle"
semantic_definition germ : Subtype := creatureType "Germ"
semantic_definition giant : Subtype := creatureType "Giant"
semantic_definition giraffe : Subtype := creatureType "Giraffe"
semantic_definition gith : Subtype := creatureType "Gith"
semantic_definition glimmer : Subtype := creatureType "Glimmer"
semantic_definition gnoll : Subtype := creatureType "Gnoll"
semantic_definition gnome : Subtype := creatureType "Gnome"
semantic_definition goat : Subtype := creatureType "Goat"
semantic_definition goblin : Subtype := creatureType "Goblin"
semantic_definition god : Subtype := creatureType "God"
semantic_definition golem : Subtype := creatureType "Golem"
semantic_definition gorgon : Subtype := creatureType "Gorgon"
semantic_definition graveborn : Subtype := creatureType "Graveborn"
semantic_definition gremlin : Subtype := creatureType "Gremlin"
semantic_definition griffin : Subtype := creatureType "Griffin"
semantic_definition guest : Subtype := creatureType "Guest"
semantic_definition hag : Subtype := creatureType "Hag"
semantic_definition halfling : Subtype := creatureType "Halfling"
semantic_definition hamster : Subtype := creatureType "Hamster"
semantic_definition harpy : Subtype := creatureType "Harpy"
semantic_definition hedgehog : Subtype := creatureType "Hedgehog"
semantic_definition hellion : Subtype := creatureType "Hellion"
semantic_definition hero : Subtype := creatureType "Hero"
semantic_definition hippo : Subtype := creatureType "Hippo"
semantic_definition hippogriff : Subtype := creatureType "Hippogriff"
semantic_definition homarid : Subtype := creatureType "Homarid"
semantic_definition homunculus : Subtype := creatureType "Homunculus"
semantic_definition horror : Subtype := creatureType "Horror"
semantic_definition horse : Subtype := creatureType "Horse"
semantic_definition human : Subtype := creatureType "Human"
semantic_definition hydra : Subtype := creatureType "Hydra"
semantic_definition hyena : Subtype := creatureType "Hyena"
semantic_definition illusion : Subtype := creatureType "Illusion"
semantic_definition imp : Subtype := creatureType "Imp"
semantic_definition incarnation : Subtype := creatureType "Incarnation"
semantic_definition inhuman : Subtype := creatureType "Inhuman"
semantic_definition inkling : Subtype := creatureType "Inkling"
semantic_definition inquisitor : Subtype := creatureType "Inquisitor"
semantic_definition insect : Subtype := creatureType "Insect"
semantic_definition jackal : Subtype := creatureType "Jackal"
semantic_definition jellyfish : Subtype := creatureType "Jellyfish"
semantic_definition juggernaut : Subtype := creatureType "Juggernaut"
semantic_definition kangaroo : Subtype := creatureType "Kangaroo"
semantic_definition kavu : Subtype := creatureType "Kavu"
semantic_definition kirin : Subtype := creatureType "Kirin"
semantic_definition kithkin : Subtype := creatureType "Kithkin"
semantic_definition knight : Subtype := creatureType "Knight"
semantic_definition kobold : Subtype := creatureType "Kobold"
semantic_definition kor : Subtype := creatureType "Kor"
semantic_definition kraken : Subtype := creatureType "Kraken"
semantic_definition kree : Subtype := creatureType "Kree"
semantic_definition lamia : Subtype := creatureType "Lamia"
semantic_definition lammasu : Subtype := creatureType "Lammasu"
semantic_definition leech : Subtype := creatureType "Leech"
semantic_definition lemur : Subtype := creatureType "Lemur"
semantic_definition leviathan : Subtype := creatureType "Leviathan"
semantic_definition lhurgoyf : Subtype := creatureType "Lhurgoyf"
semantic_definition licid : Subtype := creatureType "Licid"
semantic_definition lizard : Subtype := creatureType "Lizard"
semantic_definition llama : Subtype := creatureType "Llama"
semantic_definition lobster : Subtype := creatureType "Lobster"
semantic_definition manticore : Subtype := creatureType "Manticore"
semantic_definition masticore : Subtype := creatureType "Masticore"
semantic_definition mercenary : Subtype := creatureType "Mercenary"
semantic_definition merfolk : Subtype := creatureType "Merfolk"
semantic_definition metathran : Subtype := creatureType "Metathran"
semantic_definition minion : Subtype := creatureType "Minion"
semantic_definition minotaur : Subtype := creatureType "Minotaur"
semantic_definition mite : Subtype := creatureType "Mite"
semantic_definition mole : Subtype := creatureType "Mole"
semantic_definition monger : Subtype := creatureType "Monger"
semantic_definition mongoose : Subtype := creatureType "Mongoose"
semantic_definition monk : Subtype := creatureType "Monk"
semantic_definition monkey : Subtype := creatureType "Monkey"
semantic_definition moogle : Subtype := creatureType "Moogle"
semantic_definition moonfolk : Subtype := creatureType "Moonfolk"
semantic_definition mount : Subtype := creatureType "Mount"
semantic_definition mouse : Subtype := creatureType "Mouse"
semantic_definition mutant : Subtype := creatureType "Mutant"
semantic_definition myr : Subtype := creatureType "Myr"
semantic_definition mystic : Subtype := creatureType "Mystic"
semantic_definition nautilus : Subtype := creatureType "Nautilus"
semantic_definition necron : Subtype := creatureType "Necron"
semantic_definition nephilim : Subtype := creatureType "Nephilim"
semantic_definition nightmare : Subtype := creatureType "Nightmare"
semantic_definition nightstalker : Subtype := creatureType "Nightstalker"
semantic_definition ninja : Subtype := creatureType "Ninja"
semantic_definition noble : Subtype := creatureType "Noble"
semantic_definition noggle : Subtype := creatureType "Noggle"
semantic_definition nomad : Subtype := creatureType "Nomad"
semantic_definition nymph : Subtype := creatureType "Nymph"
semantic_definition octopus : Subtype := creatureType "Octopus"
semantic_definition ogre : Subtype := creatureType "Ogre"
semantic_definition ooze : Subtype := creatureType "Ooze"
semantic_definition orb : Subtype := creatureType "Orb"
semantic_definition orc : Subtype := creatureType "Orc"
semantic_definition orgg : Subtype := creatureType "Orgg"
semantic_definition otter : Subtype := creatureType "Otter"
semantic_definition ouphe : Subtype := creatureType "Ouphe"
semantic_definition ox : Subtype := creatureType "Ox"
semantic_definition oyster : Subtype := creatureType "Oyster"
semantic_definition pangolin : Subtype := creatureType "Pangolin"
semantic_definition peasant : Subtype := creatureType "Peasant"
semantic_definition pegasus : Subtype := creatureType "Pegasus"
semantic_definition pentavite : Subtype := creatureType "Pentavite"
semantic_definition performer : Subtype := creatureType "Performer"
semantic_definition pest : Subtype := creatureType "Pest"
semantic_definition phelddagrif : Subtype := creatureType "Phelddagrif"
semantic_definition phoenix : Subtype := creatureType "Phoenix"
semantic_definition phyrexian : Subtype := creatureType "Phyrexian"
semantic_definition pilot : Subtype := creatureType "Pilot"
semantic_definition pincher : Subtype := creatureType "Pincher"
semantic_definition pirate : Subtype := creatureType "Pirate"
semantic_definition plant : Subtype := creatureType "Plant"
semantic_definition platypus : Subtype := creatureType "Platypus"
semantic_definition porcupine : Subtype := creatureType "Porcupine"
semantic_definition possum : Subtype := creatureType "Possum"
semantic_definition praetor : Subtype := creatureType "Praetor"
semantic_definition primarch : Subtype := creatureType "Primarch"
semantic_definition prism : Subtype := creatureType "Prism"
semantic_definition processor : Subtype := creatureType "Processor"
semantic_definition qu : Subtype := creatureType "Qu"
semantic_definition rabbit : Subtype := creatureType "Rabbit"
semantic_definition raccoon : Subtype := creatureType "Raccoon"
semantic_definition ranger : Subtype := creatureType "Ranger"
semantic_definition rat : Subtype := creatureType "Rat"
semantic_definition rebel : Subtype := creatureType "Rebel"
semantic_definition reflection : Subtype := creatureType "Reflection"
semantic_definition rhino : Subtype := creatureType "Rhino"
semantic_definition rigger : Subtype := creatureType "Rigger"
semantic_definition robot : Subtype := creatureType "Robot"
semantic_definition rogue : Subtype := creatureType "Rogue"
semantic_definition sable : Subtype := creatureType "Sable"
semantic_definition salamander : Subtype := creatureType "Salamander"
semantic_definition samurai : Subtype := creatureType "Samurai"
semantic_definition sand : Subtype := creatureType "Sand"
semantic_definition saproling : Subtype := creatureType "Saproling"
semantic_definition satyr : Subtype := creatureType "Satyr"
semantic_definition scarecrow : Subtype := creatureType "Scarecrow"
semantic_definition scientist : Subtype := creatureType "Scientist"
semantic_definition scion : Subtype := creatureType "Scion"
semantic_definition scorpion : Subtype := creatureType "Scorpion"
semantic_definition scout : Subtype := creatureType "Scout"
semantic_definition sculpture : Subtype := creatureType "Sculpture"
semantic_definition seal_ : Subtype := creatureType "Seal"
semantic_definition serf : Subtype := creatureType "Serf"
semantic_definition serpent : Subtype := creatureType "Serpent"
semantic_definition servo : Subtype := creatureType "Servo"
semantic_definition shade : Subtype := creatureType "Shade"
semantic_definition shaman : Subtype := creatureType "Shaman"
semantic_definition shapeshifter : Subtype := creatureType "Shapeshifter"
semantic_definition shark : Subtype := creatureType "Shark"
semantic_definition sheep : Subtype := creatureType "Sheep"
semantic_definition shiar : Subtype := creatureType "Shi'ar"
semantic_definition siren : Subtype := creatureType "Siren"
semantic_definition skeleton : Subtype := creatureType "Skeleton"
semantic_definition skrull : Subtype := creatureType "Skrull"
semantic_definition skunk : Subtype := creatureType "Skunk"
semantic_definition slith : Subtype := creatureType "Slith"
semantic_definition sliver : Subtype := creatureType "Sliver"
semantic_definition sloth : Subtype := creatureType "Sloth"
semantic_definition slug : Subtype := creatureType "Slug"
semantic_definition snail : Subtype := creatureType "Snail"
semantic_definition snake : Subtype := creatureType "Snake"
semantic_definition soldier : Subtype := creatureType "Soldier"
semantic_definition soltari : Subtype := creatureType "Soltari"
semantic_definition sorcerer : Subtype := creatureType "Sorcerer"
semantic_definition spawn : Subtype := creatureType "Spawn"
semantic_definition specter : Subtype := creatureType "Specter"
semantic_definition spellshaper : Subtype := creatureType "Spellshaper"
semantic_definition sphinx : Subtype := creatureType "Sphinx"
semantic_definition spider : Subtype := creatureType "Spider"
semantic_definition spike : Subtype := creatureType "Spike"
semantic_definition spirit : Subtype := creatureType "Spirit"
semantic_definition splinter : Subtype := creatureType "Splinter"
semantic_definition sponge : Subtype := creatureType "Sponge"
semantic_definition spy : Subtype := creatureType "Spy"
semantic_definition squid : Subtype := creatureType "Squid"
semantic_definition squirrel : Subtype := creatureType "Squirrel"
semantic_definition starfish : Subtype := creatureType "Starfish"
semantic_definition surrakar : Subtype := creatureType "Surrakar"
semantic_definition survivor : Subtype := creatureType "Survivor"
semantic_definition symbiote : Subtype := creatureType "Symbiote"
semantic_definition synth : Subtype := creatureType "Synth"
semantic_definition tentacle : Subtype := creatureType "Tentacle"
semantic_definition tetravite : Subtype := creatureType "Tetravite"
semantic_definition thalakos : Subtype := creatureType "Thalakos"
semantic_definition thopter : Subtype := creatureType "Thopter"
semantic_definition thrull : Subtype := creatureType "Thrull"
semantic_definition tiefling : Subtype := creatureType "Tiefling"
semantic_definition timeLord : Subtype := creatureType "Time Lord"
semantic_definition toy : Subtype := creatureType "Toy"
semantic_definition treefolk : Subtype := creatureType "Treefolk"
semantic_definition trilobite : Subtype := creatureType "Trilobite"
semantic_definition triskelavite : Subtype := creatureType "Triskelavite"
semantic_definition troll : Subtype := creatureType "Troll"
semantic_definition turtle : Subtype := creatureType "Turtle"
semantic_definition tyranid : Subtype := creatureType "Tyranid"
semantic_definition unicorn : Subtype := creatureType "Unicorn"
semantic_definition utrom : Subtype := creatureType "Utrom"
semantic_definition vampire : Subtype := creatureType "Vampire"
semantic_definition varmint : Subtype := creatureType "Varmint"
semantic_definition vedalken : Subtype := creatureType "Vedalken"
semantic_definition villain : Subtype := creatureType "Villain"
semantic_definition volver : Subtype := creatureType "Volver"
semantic_definition wall : Subtype := creatureType "Wall"
semantic_definition walrus : Subtype := creatureType "Walrus"
semantic_definition warlock : Subtype := creatureType "Warlock"
semantic_definition warrior : Subtype := creatureType "Warrior"
semantic_definition weasel : Subtype := creatureType "Weasel"
semantic_definition weird : Subtype := creatureType "Weird"
semantic_definition werewolf : Subtype := creatureType "Werewolf"
semantic_definition whale : Subtype := creatureType "Whale"
semantic_definition wizard : Subtype := creatureType "Wizard"
semantic_definition wolf : Subtype := creatureType "Wolf"
semantic_definition wolverine : Subtype := creatureType "Wolverine"
semantic_definition wombat : Subtype := creatureType "Wombat"
semantic_definition worm : Subtype := creatureType "Worm"
semantic_definition wraith : Subtype := creatureType "Wraith"
semantic_definition wurm : Subtype := creatureType "Wurm"
semantic_definition yeti : Subtype := creatureType "Yeti"
semantic_definition zombie : Subtype := creatureType "Zombie"
semantic_definition zubera : Subtype := creatureType "Zubera"
semantic_definition aura : Subtype := enchantmentType "Aura"
semantic_definition background : Subtype := enchantmentType "Background"
semantic_definition cartouche : Subtype := enchantmentType "Cartouche"
semantic_definition case : Subtype := enchantmentType "Case"
semantic_definition class_ : Subtype := enchantmentType "Class"
semantic_definition curse : Subtype := enchantmentType "Curse"
semantic_definition plan : Subtype := enchantmentType "Plan"
semantic_definition role : Subtype := enchantmentType "Role"
semantic_definition room : Subtype := enchantmentType "Room"
semantic_definition rune : Subtype := enchantmentType "Rune"
semantic_definition saga : Subtype := enchantmentType "Saga"
semantic_definition shard : Subtype := enchantmentType "Shard"
semantic_definition shrine : Subtype := enchantmentType "Shrine"
semantic_definition cave : Subtype := landType "Cave"
semantic_definition desert : Subtype := landType "Desert"
semantic_definition forest : Subtype := landType "Forest"
semantic_definition gate : Subtype := landType "Gate"
semantic_definition island : Subtype := landType "Island"
semantic_definition lair : Subtype := landType "Lair"
semantic_definition locus : Subtype := landType "Locus"
semantic_definition mine : Subtype := landType "Mine"
semantic_definition mountain : Subtype := landType "Mountain"
semantic_definition plains : Subtype := landType "Plains"
semantic_definition planet : Subtype := landType "Planet"
semantic_definition powerPlant : Subtype := landType "Power-Plant"
semantic_definition sphere : Subtype := landType "Sphere"
semantic_definition swamp : Subtype := landType "Swamp"
semantic_definition tower : Subtype := landType "Tower"
semantic_definition town : Subtype := landType "Town"
semantic_definition urzas : Subtype := landType "Urza's"
semantic_definition ajani : Subtype := planeswalkerType "Ajani"
semantic_definition aminatou : Subtype := planeswalkerType "Aminatou"
semantic_definition angrath : Subtype := planeswalkerType "Angrath"
semantic_definition arlinn : Subtype := planeswalkerType "Arlinn"
semantic_definition ashiok : Subtype := planeswalkerType "Ashiok"
semantic_definition bahamut : Subtype := planeswalkerType "Bahamut"
semantic_definition basri : Subtype := planeswalkerType "Basri"
semantic_definition bolas : Subtype := planeswalkerType "Bolas"
semantic_definition calix : Subtype := planeswalkerType "Calix"
semantic_definition chandra : Subtype := planeswalkerType "Chandra"
semantic_definition comet : Subtype := planeswalkerType "Comet"
semantic_definition dack : Subtype := planeswalkerType "Dack"
semantic_definition dakkon : Subtype := planeswalkerType "Dakkon"
semantic_definition daretti : Subtype := planeswalkerType "Daretti"
semantic_definition davriel : Subtype := planeswalkerType "Davriel"
semantic_definition dellian : Subtype := planeswalkerType "Dellian"
semantic_definition dihada : Subtype := planeswalkerType "Dihada"
semantic_definition domri : Subtype := planeswalkerType "Domri"
semantic_definition dovin : Subtype := planeswalkerType "Dovin"
semantic_definition ellywick : Subtype := planeswalkerType "Ellywick"
semantic_definition elminster : Subtype := planeswalkerType "Elminster"
semantic_definition elspeth : Subtype := planeswalkerType "Elspeth"
semantic_definition estrid : Subtype := planeswalkerType "Estrid"
semantic_definition freyalise : Subtype := planeswalkerType "Freyalise"
semantic_definition garruk : Subtype := planeswalkerType "Garruk"
semantic_definition gideon : Subtype := planeswalkerType "Gideon"
semantic_definition grist : Subtype := planeswalkerType "Grist"
semantic_definition guff : Subtype := planeswalkerType "Guff"
semantic_definition huatli : Subtype := planeswalkerType "Huatli"
semantic_definition jace : Subtype := planeswalkerType "Jace"
semantic_definition jared : Subtype := planeswalkerType "Jared"
semantic_definition jaya : Subtype := planeswalkerType "Jaya"
semantic_definition jeska : Subtype := planeswalkerType "Jeska"
semantic_definition kaito : Subtype := planeswalkerType "Kaito"
semantic_definition karn : Subtype := planeswalkerType "Karn"
semantic_definition kasmina : Subtype := planeswalkerType "Kasmina"
semantic_definition kaya : Subtype := planeswalkerType "Kaya"
semantic_definition kiora : Subtype := planeswalkerType "Kiora"
semantic_definition koth : Subtype := planeswalkerType "Koth"
semantic_definition liliana : Subtype := planeswalkerType "Liliana"
semantic_definition lolth : Subtype := planeswalkerType "Lolth"
semantic_definition lukka : Subtype := planeswalkerType "Lukka"
semantic_definition minsc : Subtype := planeswalkerType "Minsc"
semantic_definition mordenkainen : Subtype := planeswalkerType "Mordenkainen"
semantic_definition nahiri : Subtype := planeswalkerType "Nahiri"
semantic_definition narset : Subtype := planeswalkerType "Narset"
semantic_definition niko : Subtype := planeswalkerType "Niko"
semantic_definition nissa : Subtype := planeswalkerType "Nissa"
semantic_definition nixilis : Subtype := planeswalkerType "Nixilis"
semantic_definition oko : Subtype := planeswalkerType "Oko"
semantic_definition quintorius : Subtype := planeswalkerType "Quintorius"
semantic_definition ral : Subtype := planeswalkerType "Ral"
semantic_definition rowan : Subtype := planeswalkerType "Rowan"
semantic_definition saheeli : Subtype := planeswalkerType "Saheeli"
semantic_definition samut : Subtype := planeswalkerType "Samut"
semantic_definition sarkhan : Subtype := planeswalkerType "Sarkhan"
semantic_definition serra : Subtype := planeswalkerType "Serra"
semantic_definition sivitri : Subtype := planeswalkerType "Sivitri"
semantic_definition sorin : Subtype := planeswalkerType "Sorin"
semantic_definition szat : Subtype := planeswalkerType "Szat"
semantic_definition tamiyo : Subtype := planeswalkerType "Tamiyo"
semantic_definition tasha : Subtype := planeswalkerType "Tasha"
semantic_definition teferi : Subtype := planeswalkerType "Teferi"
semantic_definition teyo : Subtype := planeswalkerType "Teyo"
semantic_definition tezzeret : Subtype := planeswalkerType "Tezzeret"
semantic_definition tibalt : Subtype := planeswalkerType "Tibalt"
semantic_definition tyvar : Subtype := planeswalkerType "Tyvar"
semantic_definition ugin : Subtype := planeswalkerType "Ugin"
semantic_definition urza : Subtype := planeswalkerType "Urza"
semantic_definition venser : Subtype := planeswalkerType "Venser"
semantic_definition vivien : Subtype := planeswalkerType "Vivien"
semantic_definition vraska : Subtype := planeswalkerType "Vraska"
semantic_definition vronos : Subtype := planeswalkerType "Vronos"
semantic_definition will : Subtype := planeswalkerType "Will"
semantic_definition windgrace : Subtype := planeswalkerType "Windgrace"
semantic_definition wrenn : Subtype := planeswalkerType "Wrenn"
semantic_definition xenagos : Subtype := planeswalkerType "Xenagos"
semantic_definition yanggu : Subtype := planeswalkerType "Yanggu"
semantic_definition yanling : Subtype := planeswalkerType "Yanling"
semantic_definition zariel : Subtype := planeswalkerType "Zariel"
semantic_definition adventure : Subtype := .spell "Adventure"
semantic_definition arcane : Subtype := .spell "Arcane"
semantic_definition lesson : Subtype := .spell "Lesson"
semantic_definition omen : Subtype := .spell "Omen"
semantic_definition trap : Subtype := .spell "Trap"

/-! ## Turn parts -/

semantic_definition beginningOfCombatStep : TurnPart := .beginningOfCombat
semantic_definition beginningPhase : TurnPart := .beginningPhase
semantic_definition cleanup : TurnPart := .cleanup
semantic_definition cleanupStep : TurnPart := .cleanup
semantic_definition TurnPart.combat : TurnPart := .combat
semantic_definition combatDamageStep : TurnPart := .combatDamage
semantic_definition combatPhase : TurnPart := .combat
semantic_definition declareAttackersStep : TurnPart := .declareAttackers
semantic_definition declareBlockersStep : TurnPart := .declareBlockers
semantic_definition drawStep : TurnPart := .drawStep
semantic_definition endOfCombatStep : TurnPart := .endOfCombat
semantic_definition endStep : TurnPart := .endStep
semantic_definition endingPhase : TurnPart := .endingPhase
semantic_definition mainPhase : TurnPart := .mainPhase
semantic_definition postcombatMainPhase : TurnPart := .postcombatMain
semantic_definition precombatMainPhase : TurnPart := .firstMain
semantic_definition untapStep : TurnPart := .untapStep
semantic_definition upkeep : TurnPart := .upkeep
semantic_definition upkeepStep : TurnPart := .upkeep

/-! ## Attach, create, and counter -/

/-- "attach <subject> to <host>" (`Definition`, [CR#701.3a]). -/
semantic_definition attachTo (subject host : NounPhrase) : Action :=
  .ownScope (some (.action "attach"))
    (.enact (.action "attach")
      (.attachment .attached (.inCaller (.firstMention "attachTo.subject" subject))
        (some (.inCaller (.firstMention "attachTo.host" host))))
      none none)
/-- "create <count> <token> [riders]": the tokens are put onto the battlefield, entering under
their creator's control (`Definition`, [CR#701.7a,111.2]). -/
semantic_definition create (count : Amount) (token : TokenSpec) (riders : List MoveRider := []) :
    Action :=
  .ownScope (some (.action "create"))
    (.enact (.action "create")
      (.createObject (.inCaller count) (.token (.inCaller token) (riders.map MoveRider.inCaller))
        true none)
      none none)
/-- "counter <subject>": one labeled stack-to-graveyard move (`Definition`, [CR#701.6a]). -/
semantic_definition counter (subject : NounPhrase) : Action :=
  .ownScope (some (.action "counter"))
    (.enact (.action "counter")
      (.move (.inCaller (.firstMention "counter.subject" subject)) (.zone .stack none)
        (some (.zone .graveyard none))
        .arrival none [])
      none none)

/-! ## Actions -/

/-- "cast <subject> [from <from_>] [paying <payment>]" (`Definition`, [CR#701.5a]). -/
semantic_definition cast (subject : NounPhrase) (from_ : Option Location := none)
    (payment : PlayPayment := itsOwnCost)
    (resulting : Option Predicate := none) : Action :=
  .ownScope (some (.action "cast"))
    (.procedure .cast (.inCaller subject) payment.inCaller (resulting.map .inCaller)
      (from_.map .inCaller) none)
/-- "play <subject> [from <from_>] [paying <payment>]" (`Definition`, [CR#701.18a]). -/
semantic_definition play (subject : NounPhrase) (from_ : Option Location := none)
    (payment : PlayPayment := itsOwnCost)
    (resulting : Option Predicate := none) : Action :=
  .ownScope (some (.action "play"))
    (.procedure .play (.inCaller subject) payment.inCaller (resulting.map .inCaller)
      (from_.map .inCaller) none)
/-- "activate <subject> [from <from_>] [paying <payment>]" (`Definition`, [CR#701.2a]). -/
semantic_definition activate (subject : NounPhrase) (from_ : Option Location := none)
    (payment : PlayPayment := itsOwnCost)
    (resulting : Option Predicate := none) : Action :=
  .ownScope (some (.action "activate"))
    (.procedure .activate (.inCaller subject) payment.inCaller (resulting.map .inCaller)
      (from_.map .inCaller) none)
/-- "the regeneration of <subject>": the deed "can't be regenerated" forbids, a shield's
application rather than the action that creates one (`Definition`, [CR#701.19c]). -/
semantic_definition regeneration (subject : NounPhrase) : Action :=
  .ownScope (some (.action "regenerate"))
    (.enact (.action "regenerate") (.clearDamage (.inCaller subject)) none none)
/-- "destroy <subject>" (`Definition`, [CR#701.8]). -/
semantic_definition destroy (subject : NounPhrase) : Action :=
  .ownScope (some (.action "destroy"))
    (.enact (.action "destroy")
      (.move (.inCaller (.firstMention "destroy.subject" subject)) (.zone .battlefield none)
        (some graveyard) .departure none [])
      none none)
/-- "exile <subject>" (`Definition`, [CR#701.13]). -/
semantic_definition exile (subject : NounPhrase) : Action :=
  .ownScope (some (.action "exile"))
    (.enact (.action "exile")
      (.move (.inCaller (.firstMention "exile.subject" subject)) .anyLocation (some exileZone)
        .arrival none [])
      none none)
/-- "exile <subject> face down" (`Definition`, [CR#701.13,406.3]). -/
semantic_definition exileFaceDown (subject : NounPhrase) : Action :=
  .ownScope (some (.action "exile"))
    (.enact (.action "exile")
      (.move (.inCaller (.firstMention "exileFaceDown.subject" subject)) .anyLocation
        (some exileZone) .arrival none
        [.faceDown exiledFaceDown])
      none none)
/-- "exile <subject> with N <kind> counters on it" (`Definition`, [CR#701.13]). -/
semantic_definition exileWithCounters (subject : NounPhrase) (amount : Amount)
    (kind : CounterKind) : Action :=
  .ownScope (some (.action "exile"))
    (.enact (.action "exile")
      (.move (.inCaller (.firstMention "exileWithCounters.subject" subject)) .anyLocation
        (some exileZone) .arrival none
        [.withCounters (.inCaller amount) (.printed kind) .fresh])
      none none)
/-- "sacrifice <subject>" (`Definition`, [CR#701.21]). -/
semantic_definition sacrifice (subject : NounPhrase) : Action :=
  .ownScope (some (.action "sacrifice"))
    (.enact (.action "sacrifice")
      (.move (.inCaller (.firstMention "sacrifice.subject" subject)) (.zone .battlefield none)
        (some graveyard) .departure none [])
      none none)
/-- "transform <subject>" (`Definition`, [CR#701.27]). -/
semantic_definition transform (subject : NounPhrase) : Action :=
  .ownScope (some (.action "transform"))
    (.enact (.action "transform")
      (.turnOver (.inCaller (.firstMention "transform.subject" subject)))
      none none)
/-- "meld <subject> into <name>" (`Definition`, [CR#701.42]). -/
semantic_definition meldInto (subject : NounPhrase) (into : String) : Action :=
  .ownScope (some (.action "meld"))
    (.enact (.action "meld")
      (.move (.inCaller (.firstMention "meldInto.subject" subject)) .anyLocation (some battlefield)
        .arrival none [.entersMelded into])
      none none)
/-- Placement-only fragment of manifest for the anaphora bench [CR#701.40a]. -/
semantic_definition manifestPlacement (subject : NounPhrase) : Action :=
  .ownScope (some (.action "manifest"))
    (.enact (.action "manifest")
      (.move (.inCaller (.firstMention "manifestPlacement.subject" subject)) .anyLocation
        (some battlefield) .arrival none [])
      none none)
/-- "tap <subject>" (`Definition`, [CR#701.26a]). -/
semantic_definition tap (subject : NounPhrase) : Action :=
  .ownScope (some (.action "tap"))
    (.enact (.action "tap") (.setStatus .tapped (.inCaller (.firstMention "tap.subject" subject)))
      none none)
/-- "<performer> discards <subject>": a move from the performer's own hand, the card's owner's,
to the graveyard (`Definition`, [CR#701.9a]). -/
semantic_definition discard (subject : NounPhrase) : Action :=
  .ownScope (some (.action "discard"))
    (.enact (.action "discard")
      (.move (.inCaller (.firstMention "discard.subject" subject)) (.zone .hand (some .actor))
        (some graveyard) .arrival none [])
      none none)
/-- A bare "shuffle": the performer's own library (`Definition`, [CR#701.24a]). -/
semantic_definition shuffle : Action :=
  .ownScope (some (.action "shuffle"))
    (.enact (.action "shuffle") (.shuffle (libraryOf .actor)) none none)
/-- "untap <subject>" (`Definition`, [CR#701.26b]). -/
semantic_definition untap (subject : NounPhrase) : Action :=
  .ownScope (some (.action "untap"))
    (.enact (.action "untap")
      (.setStatus .untapped (.inCaller (.firstMention "untap.subject" subject)))
      none none)
/-- "<performer> mills <amount> cards": from the performer's own library (`Definition`,
[CR#701.17a]). -/
semantic_definition mill (amount : Amount) : Action :=
  .ownScope (some (.action "mill"))
    (.enact (.action "mill")
      (.move (.slice .top (.inCaller amount) (libraryOf .actor))
        (.zone .library none) (some graveyard) .arrival none [])
      none none)
/-- "<cards> are milled": the mill whose patient is a card description, as an event a trigger
watches ("Whenever one or more nonland cards are milled") (`Definition`, [CR#701.17a,701.17c]). -/
semantic_definition millCards (cards : NounPhrase) : Action :=
  .ownScope (some (.action "mill"))
    (.enact (.action "mill")
      (.move (.inCaller (.firstMention "millCards.cards" cards))
        library (some graveyard) .arrival none [])
      none none)
/-- "flip <subject>": a flip card's permanent turns to its flipped half, whose alternative name,
text box, type line, power and toughness apply (`Definition`, [CR#710.1a,710.2]). Flipping is
not a keyword action, so its scope names no deed. -/
semantic_definition flip (subject : NounPhrase := .actor) : Action :=
  .ownScope none (.setStatus .flipped (.inCaller (.firstMention "flip.subject" subject)))
/-- "Search your library for <quantity> <p>" (`Definition`, [CR#701.23]). -/
semantic_definition searchLibraryFor (quantity : Quantity) (p : Predicate) : Action :=
  .ownScope (some (.action "search"))
    (.enact (.action "search") (.search (yourLibrary) (.inCaller quantity) (.inCaller p)) none none)
/-- "search <whose>'s graveyard, hand, and library for <q> <p>" (`Definition`, [CR#701.23]). -/
semantic_definition searchZonesOf (whose : NounPhrase) (quantity : Quantity) (p : Predicate) :
    Action :=
  .ownScope (some (.action "search"))
    (.enact (.action "search")
      (.search
        (.or
          [ .zone .graveyard (some (.inCaller (.firstMention "searchZonesOf.whose" whose))),
            .zone .hand (some (.laterMention "searchZonesOf.whose" (.inCaller whose))),
            .zone .library (some (.laterMention "searchZonesOf.whose" (.inCaller whose))) ])
        (.inCaller quantity) (.inCaller p))
      none none)
/-- "search your library and/or graveyard for a <p>" (`Definition`, [CR#701.23]). -/
semantic_definition searchLibraryOrGraveyard (p : Predicate) : Action :=
  .ownScope (some (.action "search"))
    (.enact (.action "search")
      (.search (.or [.zone .library (some .you), .zone .graveyard (some .you)]) (exactly 1)
        (.inCaller p))
      none none)
/-- "<creature> fights <creature>" (`Definition`, [CR#701.14a]). -/
semantic_definition fight (other : NounPhrase) : Action :=
  .ownScope (some (.action "fight"))
    (.enact (.action "fight")
      (.doIf (.and [
        .matches (.firstMention "fight.0" (.inCaller other)) isCreature,
        .matches (.firstMention "fight.source" .actor) isCreature])
        (.simultaneously [
          .dealDamage (.statOf (.stat .power) (.laterMention "fight.source" .actor))
            (.inCaller (.laterMention "fight.0" other)) .any,
          .instruct (.laterMention "fight.0" (.inCaller other))
            (.dealDamage (.statOf (.stat .power) (.laterMention "fight.0" (.inCaller other)))
              (.laterMention "fight.source" .actor) .any)])
        none)
      none none)
/-- "reveal <cards>" (`Definition`, [CR#701.20]). -/
semantic_definition reveal (cards : NounPhrase) : Action :=
  .ownScope (some (.action "reveal"))
    (.enact (.action "reveal")
      (.expose .reveal (.cards (.inCaller (.firstMention "reveal.cards" cards))))
      none none)
/-- "shuffle <subject> into its owner's library": the move, then the shuffle of that library,
which happens even if nothing moved (`Definition`, [CR#701.24c,701.24d]). -/
semantic_definition shuffleInto (subject : NounPhrase) : Action :=
  .ownScope (some (.action "shuffle"))
    (.sequentially
      [ .enact (.action "shuffle")
          (.move (.inCaller (.firstMention "shuffleInto.subject" subject)) .anyLocation
            (some (.zone .library none))
            .arrival none [])
          none none,
        .shuffle (.zone .library none) ])
/-- "vote for <ballot> [<times> times]" (`Definition`, [CR#701.38]). -/
semantic_definition vote (disclosure : Disclosure := .openly)
    (ballot : Ballot := anyBallot)
    (times : Option Quantity := none) : Action :=
  .ownScope (some (.action "vote"))
    (.enact (.action "vote") (.vote none disclosure ballot times) none none)
/-- "Starting with <first>, vote for <ballot>" (`Definition`, [CR#701.38]). -/
semantic_definition voteStartingWith (first : NounPhrase) (disclosure : Disclosure)
    (ballot : Ballot) : Action :=
  .ownScope (some (.action "vote"))
    (.enact (.action "vote")
      (.vote (some (.inCaller (.firstMention "voteStartingWith.first" first))) disclosure ballot
        none)
      none none)
/-- "scry N" (`Definition`, [CR#701.22a]): the performer's own library. -/
semantic_definition scry (amount : Amount) : Action :=
  .ownScope (some (.action "scry"))
    (.enact (.action "scry")
      (lookAndSortSlice (.slice .top (.inCaller amount) (libraryOf .actor)) onBottom
        [(.inOrder .anyOrder)])
      none none)
/-- "fateseal N" (`Definition`, [CR#701.29a]): the top N cards of an opponent's library, an
opponent of the performer. -/
semantic_definition fateseal (amount : Amount) : Action :=
  .ownScope (some (.action "fateseal"))
    (.enact (.action "fateseal")
      (lookAndSortSlice (.slice .top (.inCaller amount) (libraryOf (a (isOpponentOf .actor))))
        onBottom [(.inOrder .anyOrder)])
      none none)
/-- "surveil N" (`Definition`, [CR#701.25a]): the performer's own library. -/
semantic_definition surveil (amount : Amount) : Action :=
  .ownScope (some (.action "surveil"))
    (.enact (.action "surveil")
      (lookAndSortSlice (.slice .top (.inCaller amount) (libraryOf .actor)) graveyard)
      none none)
/-- "Proliferate" with its reminder text (`Definition`, [CR#701.34a]). -/
semantic_definition proliferate : Action :=
  .ownScope (some (.action "proliferate"))
    (.enact (.action "proliferate") (.sequentially
      [ .choose none (counted anyNumber
            (.or [ .and [isPermanent, .hasCounters none],
                   .compare [.anyCounter .player] .atLeast (.lit 1) ])) .openly none,
        .putCounters (.lit 1) .own (.eachOf (those .join)) .one true ])
      none none)
/-- "<performer> amasses <subtype> N" with its reminder text (`Definition`, [CR#701.47a]): the
chosen Army is `that (.type .creature)` and `it`, resolved inside `ownScope`. -/
semantic_definition amass (subtype : Subtype) (count : Amount) : Action :=
  .ownScope (some (.action "amass"))
    (.enact (.action "amass") (.sequentially
      [ .doIf (.not (exists_ army))
          (create (.lit 1) (.written (creatureToken 0 0 [.black] [subtype, creatureType "Army"])))
          none,
        choose (a army),
        .putCounters (.inCaller count) (.printed p1p1Counter) (that (.type .creature)) .one true,
        .doIf (.not (.matches it (.hasSubtype subtype)))
          (become it { characteristics := { subtypes := [subtype] } } none)
          none ])
      none none)
/-- "monstrosity N" with its reminder text (`Definition`, [CR#701.37a]). -/
semantic_definition monstrosity (amount : Amount) : Action :=
  .ownScope (some (.action "monstrosity"))
    (.enact (.action "monstrosity")
      (.doIf (.not (.matches .actor (.hasDesignation monstrous none)))
        (.sequentially
          [ .putCounters (.inCaller amount) (.printed p1p1Counter) .actor .one true,
            .gainDesignation .actor monstrous none ])
        none)
      none none)
semantic_definition doubleCounters (on : NounPhrase) : Action :=
  .ownScope (some (.action "double"))
    (.doubleCounters (.inCaller (.firstMention "doubleCounters.on" on)))
semantic_definition exchangeCards (left right : NounPhrase) : Action :=
  .ownScope (some (.action "exchange"))
    (enact (.action "exchange")
      (.exchange
        (.cardsAcross (.inCaller (.firstMention "exchangeCards.left" left))
          (.inCaller (.firstMention "exchangeCards.right" right)))))
semantic_definition exchangeControl (left right : NounPhrase) : Action :=
  .ownScope (some (.action "exchange"))
    (enact (.action "exchange")
      (.exchange
        (.controlOf (.inCaller (.firstMention "exchangeControl.left" left))
          (.inCaller (.firstMention "exchangeControl.right" right)))))
semantic_definition exchangeLifeTotals (parties : NounPhrase) : Action :=
  .ownScope (some (.action "exchange"))
    (enact (.action "exchange")
      (.exchange (.lifeTotals (.inCaller (.firstMention "exchangeLifeTotals.parties" parties)))))
semantic_definition exchangeTextBoxes (left right : NounPhrase) : Action :=
  .ownScope (some (.action "exchange"))
    (enact (.action "exchange")
      (.establish
        (.textChange (.inCaller (.firstMention "exchangeTextBoxes.left" left))
          (.exchangeWith (.inCaller (.firstMention "exchangeTextBoxes.right" right))))
        none))
semantic_definition exchangeValues (left right : Amount) : Action :=
  .ownScope (some (.action "exchange"))
    (enact (.action "exchange") (.exchange (.values (.inCaller left) (.inCaller right))))
semantic_definition exchangeZones (left right : Location) : Action :=
  .ownScope (some (.action "exchange"))
    (enact (.action "exchange") (.exchange (.zones (.inCaller left) (.inCaller right))))
semantic_definition exileFrom (subject : NounPhrase) (from_ : Location) : Action :=
  .ownScope (some (.action "exile"))
    (enact (.action "exile")
      (.move (.inCaller (.firstMention "exileFrom.subject" subject)) (.inCaller from_)
        (some exileZone) .arrival none []))
/-- "<performer> reveals their hand" (`Definition`, [CR#701.20]). -/
semantic_definition revealHand : Action :=
  .ownScope (some (.action "reveal"))
    (.enact (.action "reveal") (.expose .reveal (.zone (handOf .actor))) none none)
/-- "<performer> collects evidence N" (`Definition`, [CR#701.59a]). -/
semantic_definition collectEvidence (amount : Amount) : Action :=
  .ownScope (some (.action "collectEvidence"))
    (.enact (.action "collectEvidence")
      (exile
        (withTotal (.stat .manaValue) .atLeast (.inCaller amount)
          (allOf (.and [.isCard, .in_ (graveyardOf .actor)]))))
      none none)
/-- "<performer> casts <subject> without paying its mana cost [if the resulting spell is …]"
(`Definition`, [CR#701.5,601.2]). -/
semantic_definition castWithoutPaying (subject : NounPhrase) (resulting : Option Predicate) :
    Action :=
  .ownScope (some (.action "cast"))
    (.enact (.action "cast")
      (.procedure .cast (.inCaller (.firstMention "castWithoutPaying.subject" subject))
        .withoutPaying
        (resulting.map Predicate.inCaller)
        none none)
      none none)
/-- "<performer> discovers N" (`Definition`, [CR#701.57a]). -/
semantic_definition discover (amount : Amount) : Action :=
  let fits : Predicate := .compare [.stat .manaValue] .atMost (.inCaller amount)
  .ownScope (some (.action "discover"))
    (.enact (.action "discover") (.sequentially
      [ exile (.run .top (.and [.not isLand, fits]) (libraryOf .actor)),
        .withContinuation .optional (castWithoutPaying (that .card) (some fits)) none
          (some (putInto (that .card) hand)),
        .move (theRest .object) .anyLocation (some onBottom) .arrival none
          [(.inOrder .randomOrder)] ])
      none none)
/-- "<performer> incubates N" (`Definition`, [CR#701.53a,701.53b]). -/
semantic_definition incubate (amount : Amount) : Action :=
  .ownScope (some (.action "incubate"))
    (enact (.action "incubate")
      (create (.lit 1)
        (.doubleFaced
          { characteristics :=
              { types := [.artifact], subtypes := [incubator],
                text := [activated (mana [generic 2]) (transform this_)] } }
          { characteristics :=
              { name := some "Phyrexian Token", types := [.artifact, .creature],
                subtypes := [phyrexian], power := some (.lit 0), toughness := some (.lit 0) } })
        [.withCounters (.inCaller amount) (.printed p1p1Counter) .fresh]))
/-- "<performer> waterbends <cost>" (`Definition`, [CR#701.67a,701.67b]). -/
semantic_definition waterbend (cost : Cost) : Action :=
  .ownScope (some (.action "waterbend"))
    (.enact (.action "waterbend")
      (.pay
        (.withSubstitute (.inCaller cost)
          (substitute (tap (a (.and [isUntapped, .or [isArtifact, isCreature], actorControls])))
            [.generic]))
        .once .paid none none)
      none none)
/-- "<performer> manifests <subject>" (`Definition`, [CR#701.40a,708.2,708.2a]). -/
semantic_definition manifest (subject : NounPhrase) : Action :=
  .ownScope (some (.action "manifest"))
    (.enact (.action "manifest") (.sequentially
      [ .turnFaceDown (.inCaller (.firstMention "manifest.subject" subject)) faceDownCreature,
        .move (that .card) .anyLocation (some battlefield) .arrival none
          [(.faceDown faceDownCreature)] ])
      none none)
/-- "<performer> cloaks <subject>" (`Definition`, [CR#701.58a]). -/
semantic_definition cloak (subject : NounPhrase) : Action :=
  .ownScope (some (.action "cloak"))
    (.enact (.action "cloak") (.sequentially
      [ .turnFaceDown (.inCaller (.firstMention "cloak.subject" subject)) faceDownWardedCreature,
        .move (that .card) .anyLocation (some battlefield) .arrival none
          [(.faceDown faceDownWardedCreature)] ])
      none none)
/-- "<performer> manifests dread" (`Definition`, [CR#701.62a]). -/
semantic_definition manifestDread : Action :=
  .ownScope (some (.action "manifestDread"))
    (.enact (.action "manifestDread") (.sequentially
      [ .expose .lookAt (.cards (.slice .top (.lit 2) (libraryOf .actor))),
        manifest (someOf (exactly 1) them),
        putInto (theRest .object) graveyard ])
      none none)
/-- "support N" (`Definition`, [CR#701.41a]). -/
semantic_definition support (amount : Amount) : Action :=
  .ownScope (some (.action "support"))
    (.enact (.action "support")
      (.putCounters (.lit 1) (.printed p1p1Counter)
        (.eachOf
          (.described (.target (.upToOf (.inCaller amount))) (.and [isCreature, .otherThan this_])))
        .one true)
      none none)

/-- "<performer> explores" (`Definition`, [CR#701.44a]). -/
semantic_definition explore : Action :=
  .ownScope (some (.action "explore"))
    (.enact (.action "explore")
      (.instruct (controllerOf (.firstMention "explore.permanent" .actor))
        (.sequentially
          [ reveal (.slice .top (.lit 1) (libraryOf .actor)),
            .doIf (.matches (that .card) isLand)
              (putInto (that .card) (.zone .hand (some (ownerOf (that .card)))))
              (some (.sequentially
                [ .putCounters (.lit 1) (.printed p1p1Counter)
                    (.laterMention "explore.permanent" .actor) .one true,
                  may (putInto (that .card) graveyard) ])) ]))
      none none)
/-- "<performer> connives [N]" (`Definition`, [CR#701.50a,701.50d]): loops over
`"each nonland card discarded this way"`
(`allOf(and([not(land), card, did(any, discard(which), thisWay)]))`). -/
semantic_definition connive (amount : Amount := .lit 1) : Action :=
  .ownScope (some (.action "connive"))
    (.enact (.action "connive")
      (.instruct (controllerOf (.firstMention "connive.permanent" .actor))
        (.sequentially
          [ .draw (.inCaller amount),
            discard (counted (.exactlyOf (.inCaller amount)) .isCard),
            .doForEach (allOf (.and [.not isLand, .isCard,
                .did ⟨.any, discard .which, .thisWay⟩]))
              (.putCounters (.lit 1) (.printed p1p1Counter)
                (.laterMention "connive.permanent" .actor) .one true) ]))
      none none)
/-- "Airbend <subject>" (`Definition`, [CR#701.65a]): per-card `doForEach` loop over
`allOf(and([card, did(any, exile(which), thisWay)]))` where `it` resolves by `D′` to the
singular loop referent. -/
semantic_definition airbend (subject : NounPhrase) : Action :=
  .ownScope (some (.action "airbend"))
    (.enact (.action "airbend")
      (.sequentially
        [ exile (.inCaller (.firstMention "airbend.subject" subject)),
          .doForEach (allOf (.and [.isCard, .did ⟨.any, exile .which, .thisWay⟩]))
            (.instruct (ownerOf it)
              (.establish
                (can (ownerOf it)
                  (cast it (from_ := some exileZone)
                    (payment := .payingInstead (.mana [generic 2]))))
                (some (.forAsLongAs (.matches it (.in_ exileZone)))))) ])
      none none)
/-- "double <subject>'s power" (`Definition`, [CR#701.10b,701.10c]): it gets +X/+0, X its power. -/
semantic_definition doublePower (subject : NounPhrase) (duration : Option Duration) : Action :=
  .ownScope (some (.action "double"))
    (.establish (.ptModification (.firstMention "doublePower.0" (.inCaller subject))
      (.up (powerOf (.laterMention "doublePower.0" (.inCaller subject)))) (.up (.lit 0)))
      duration)
/-- "double <subject>'s power and toughness": it gets +X/+Y, X its power and Y its toughness
(`Definition`, [CR#701.10b,701.10c]). -/
semantic_definition doublePowerAndToughness (subject : NounPhrase)
    (duration : Option Duration := none) : Action :=
  .ownScope (some (.action "double"))
    (get (.firstMention "doublePowerAndToughness.subject" (.inCaller subject))
      (.up (powerOf (.laterMention "doublePowerAndToughness.subject" (.inCaller subject))))
      (.up (toughnessOf (.laterMention "doublePowerAndToughness.subject" (.inCaller subject))))
      duration)
/-- "double <subject>'s toughness": it gets +0/+X, X its toughness (`Definition`,
[CR#701.10b,701.10c]). -/
semantic_definition doubleToughness (subject : NounPhrase) (duration : Option Duration := none) :
    Action :=
  .ownScope (some (.action "double"))
    (get (.firstMention "doubleToughness.subject" (.inCaller subject)) (.up (.lit 0))
      (.up (toughnessOf (.laterMention "doubleToughness.subject" (.inCaller subject))))
      duration)
/-- "triple <subject>'s power and toughness" (`Definition`, [CR#701.11b,701.11c]). -/
semantic_definition triplePowerAndToughness (subject : NounPhrase) (duration : Option Duration) :
    Action :=
  .ownScope (some (.action "triple"))
    (.establish (.ptModification (.firstMention "triplePT.0" (.inCaller subject))
      (.up (times (.lit 2) (powerOf (.laterMention "triplePT.0" (.inCaller subject)))))
      (.up (times (.lit 2) (toughnessOf (.laterMention "triplePT.0" (.inCaller subject))))))
      duration)
/-- "triple <subject>'s power": it gets +X/+0, X twice its power (`Definition`,
[CR#701.11b,701.11c]). -/
semantic_definition triplePower (subject : NounPhrase) (duration : Option Duration := none) :
    Action :=
  .ownScope (some (.action "triple"))
    (get (.firstMention "triplePower.subject" (.inCaller subject))
      (.up (times (.lit 2) (powerOf (.laterMention "triplePower.subject" (.inCaller subject)))))
      (.up (.lit 0)) duration)
/-- "triple <subject>'s toughness": it gets +0/+X, X twice its toughness (`Definition`,
[CR#701.11b,701.11c]). -/
semantic_definition tripleToughness (subject : NounPhrase) (duration : Option Duration := none) :
    Action :=
  .ownScope (some (.action "triple"))
    (get (.firstMention "tripleToughness.subject" (.inCaller subject)) (.up (.lit 0))
      (.up
        (times (.lit 2)
          (toughnessOf (.laterMention "tripleToughness.subject" (.inCaller subject)))))
      duration)
/-- "double <performer>'s life total" (`Definition`, [CR#701.10d]), written as that total's set
[CR#119.5]. -/
semantic_definition doubleLifeTotal : Action :=
  .ownScope (some (.action "double"))
    (.changeLife (.set (times (.lit 2) (lifeTotalOf .actor))))

/-! ## Tokens -/

/-- A Food token's characteristics: "a colorless Food artifact token with '{2}, {T}, Sacrifice
this token: You gain 3 life.'" [CR#111.10b] -/
semantic_definition foodToken : CharacteristicBundle :=
  { characteristics :=
      { types := [.artifact], subtypes := [food],
        text :=
          [ activated (.compound [.mana [generic 2], .tapSymbol, .perform (sacrifice thisArtifact)])
              (gainLife (.lit 3)) ] } }
/-- A Treasure token's characteristics: "a colorless Treasure artifact token with '{T},
Sacrifice this token: Add one mana of any color.'" [CR#111.10a] -/
semantic_definition treasureToken : CharacteristicBundle :=
  { characteristics :=
      { types := [.artifact], subtypes := [treasure],
        text :=
          [ activated (.compound [.tapSymbol, .perform (sacrifice thisArtifact)])
              (.addMana (.lit 1) (.anyColor .sameColor) []) ] } }

/-- A Clue token's characteristics: "a colorless Clue artifact token with '{2}, Sacrifice this
token: Draw a card.'" [CR#111.10f] -/
semantic_definition clueToken : CharacteristicBundle :=
  { characteristics :=
      { types := [.artifact], subtypes := [clue],
        text :=
          [ activated (.compound [.mana [generic 2], .perform (sacrifice thisArtifact)])
              (draw (.lit 1)) ] } }

/-! ## Templates over Definitions -/

/-- "Exile <subject> until <event>." -/
semantic_template exileUntil (subject : NounPhrase) (event : GameEvent) : Action :=
  .holdUntil (exile subject) event
/-- The replacement's application, rather than installation of a new shield
[CR#701.19b,701.19c]. -/
semantic_template regenerationApplication (subject : NounPhrase) : Action :=
  .enact (.action "regenerate")
    (.sequentially
      [ .clearDamage (.firstMention "regenerationApplication.subject" subject),
        .instruct
          (.possessorOf .controller (.laterMention "regenerationApplication.subject" subject))
          (tap (.laterMention "regenerationApplication.subject" subject)),
        .doIf
          (.or
            [ .matches (.laterMention "regenerationApplication.subject" subject)
                (.inCombat .attackerOf none),
              .matches (.laterMention "regenerationApplication.subject" subject)
                (.inCombat .blockerOf none) ])
          (.combat (.laterMention "regenerationApplication.subject" subject)
            (.participation .outsideCombat))
          none ])
    none none
/-- "attach <what> to it": ordinary pronoun `it` resolved by `D′` (no `Window` override). -/
semantic_template attachToIt (what : NounPhrase) : Action :=
  attachTo what it
/-- "<subject>'s controller sacrifices it" (`Template` using `firstMention` / `laterMention`
without `Window` plumbing, [CR#701.21a]). -/
semantic_template controllerSacrifices (subject : NounPhrase) : Action :=
  .instruct (.possessorOf .controller (.firstMention "controllerSacrifices.subject" subject))
    (sacrifice (.laterMention "controllerSacrifices.subject" subject))
/-- "discard <discarded>, then <step> for each nonland card discarded this way" (`Template` using
moved-parameter readback, [CR#701.50a,400.7,400.7j]). -/
semantic_template discardThenForEachNonland (discarded : NounPhrase) (step : Action) :
    Action :=
  .sequentially
    [ discard discarded,
      .doForEach
        (allOf (.and [.not isLand, .isCard, .did ⟨.any, discard .which, .thisWay⟩]))
        step ]
/-- "you may choose not to untap <subject> during [<whose>] untap step" -/
semantic_template mayDeclineUntap (subject : NounPhrase) (whose : Option NounPhrase) : StaticSpec :=
  duringUntapStep whose (.can .any (untap subject) {})
/-- "untap <subject> during [<whose>] untap step" -/
semantic_template untapsDuring (subject : NounPhrase) (whose : Option NounPhrase) : StaticSpec :=
  duringUntapStep whose (.must .any (untap subject) {})
/-- "<subject> may be attached to <host>": the legal-host permission over the attach deed's host
role [CR#701.3a,301.5]. -/
semantic_template mayBeAttachedTo (subject host : NounPhrase) : StaticSpec :=
  .can .any (attachTo subject host) {}
/-- "create <count> <token>" -/
semantic_template createToken (token : CharacteristicBundle) (count : Amount := .lit 1) : Action :=
  create count (.written token)
/-- "create <count> tapped <token>" -/
semantic_template createTappedToken (token : CharacteristicBundle) (count : Amount := .lit 1) :
    Action :=
  create count (.written token) [.entersAs .tapped]
/-- "create <count> <token> tapped and attacking" -/
semantic_template createTappedAttacking (count : Amount) (token : CharacteristicBundle) : Action :=
  create count (.written token) [.entersAs .tapped, .entersAttacking none]
/-- "create a token that's a copy of <source> [riders]" -/
semantic_template createTokenCopy (source : NounPhrase) (riders : List MoveRider := []) : Action :=
  create (.lit 1) (.copyOf source []) riders

/-! The regeneration shield is a Definition, but its replacement applies
`regenerationApplication`, a Template over `tap`, so it follows that Template here. -/

/-- A resolving regeneration action installs a single-use shield for this turn;
only its eventual application carries the Regenerate label (`Definition`, [CR#701.19a,701.19c]). -/
semantic_definition regenerate (subject : NounPhrase) : Action :=
  .ownScope (some (.action "regenerate"))
    (.establish (.instead
      (.would .any (destroy (.firstMention "regenerate.0" (.inCaller subject))))
      (regenerationApplication (.laterMention "regenerate.0" (.inCaller subject)))
      none .nextTimeOnly none) (some thisTurn))

private theorem regenerationEventDescribesTheApplicationRatherThanTheShield :
    let event := does any (regenerationApplication thisCreature)
    let applied := (stepGameEvent 64 (DRS.ofList []) event).1
    let shield := (stepAction 64 .postEvent .obj (DRS.ofList []) (regenerate thisCreature)).1
    GameEvent.check (DRS.ofList []) event = [] ∧
      countOnes .object applied = 1 ∧
      (applied.findById? 0).map (fun ref => (ref.kind, ref.zone)) =
        some (.object, some .battlefield) ∧
      countOnes .object shield = 0 ∧
      Action.check (DRS.ofList [])
        (.triggerThisWay (regenerate thisCreature) event
          (.putCounters (.lit 1) (.printed m1m1Counter) it .many true)) = [] := by decide

/-! ## Abilities -/

/-- "Companion — <condition>" -/
semantic_definition companion (condition : DeckCondition) : Ability :=
  .keyword (.named "companion") [.deckCondition condition] []
/-- Level up [cost]'s definition, an activated ability: "[Cost]: Put a level counter on this
permanent. Activate only as a sorcery." [CR#702.87a] -/
semantic_definition levelUpExpansion (cost : Cost) : Ability :=
  activatedOnlyDuring cost (.putCounters (.lit 1) (.printed levelCounter) thisPermanent .one true)
    asSorcery
/-- "Level up [cost]" with its definition. -/
semantic_definition levelUp (cost : Cost) : Ability :=
  .keyword (.named "levelUp") [.cost (.inCaller cost)] [levelUpExpansion (.inCaller cost)]
/-- Renown N's reminder text: "When this creature deals combat damage to a player, if it isn't
renowned, put N +1/+1 counters on it and it becomes renowned." [CR#702.112a] -/
semantic_definition renownExpansion (count : Amount) : Ability :=
  triggeredIf (does thisCreature (dealDamage (to := a .anyPlayer) (kind := .combatOnly)))
    (.not (.matches thisCreature (.hasDesignation renowned none)))
    (.sequentially
      [ .putCounters count (.printed p1p1Counter) thisCreature .one true,
        .gainDesignation thisCreature renowned none ])
/-- Storm's reminder text: "When you cast this spell, copy it for each other spell that was cast
before it this turn. You may choose new targets for the copies." [CR#702.40a] -/
semantic_definition stormExpansion : Ability :=
  when (.does (.actor .you) (cast thisSpell))
    (.sequentially
      [ .copy .fromStack thisSpell
          (eventCount
            (cast (a (.and [isSpell, .otherThan thisSpell])))
            (.actor (a .anyPlayer)) earlierThisTurn)
          [],
        may (.chooseNewTargets (.pro (.word .copy) .many)) ])
/-- "Storm" with its reminder text. -/
semantic_definition storm : Ability := .keyword (.named "storm") [] [stormExpansion]
/-- Cascade's reminder text [CR#702.85a]. -/
semantic_definition cascadeExpansion : Ability :=
  let cheaper : Predicate :=
    .compare [.stat .manaValue] .less (.statOf (.stat .manaValue) thisSpell)
  when (.does (.actor .you) (cast thisSpell))
    (.sequentially
      [ exile (.run .top (.and [.not isLand, cheaper]) (libraryOf .you)),
        may (castWithoutPaying (that .card) (some cheaper)),
        .move
          (allFromAmong
            (.not (.did ⟨.any, cast .which, .thisWay⟩))
            (theVerbed (.action "exile") .many))
          .anyLocation (some onBottom) .arrival none [(.inOrder .randomOrder)] ])
/-- "Cascade" with its reminder text. -/
semantic_definition cascade : Ability := .keyword (.named "cascade") [] [cascadeExpansion]
/-- Ripple N's definition [CR#702.60a]: "When you cast this spell, you may reveal the top N cards
of your library. If you do, you may cast any of those cards with the same name as this spell
without paying their mana costs, then put all revealed cards not cast this way on the bottom of
your library in any order." -/
semantic_definition rippleExpansion (amount : Amount) : Ability :=
  when (.does (.actor .you) (cast thisSpell))
    (may (reveal (.slice .top amount (libraryOf .you)))
      (some
        (.sequentially
          [ castWithoutPaying
              (.someOf (.counted anyNumber) (some (.shares .cardName .share this_)) (those .card))
              none,
            .move
              (allFromAmong
                (.not (.did ⟨.any, cast .which, .thisWay⟩))
                (theVerbed (.action "reveal") .many (some .card)))
              .anyLocation (some onBottom) .arrival none [.inOrder .anyOrder] ])))
/-- "Ripple N" with its definition [CR#702.60a]. -/
semantic_definition ripple (amount : Amount) : Ability :=
  .keyword (.named "ripple") [.number (.inCaller amount)] [rippleExpansion (.inCaller amount)]
/-- "Renown N" with its reminder text. -/
semantic_definition renown (count : Amount) : Ability :=
  .keyword (.named "renown") [.number (.inCaller count)] [renownExpansion (.inCaller count)]
/-- Cumulative upkeep's reminder text [CR#702.24a]. -/
semantic_definition cumulativeUpkeepExpansion (cost : Cost) : Ability :=
  triggeredIf (.beginningOf .upkeep (some .you) (.each none))
    (.matches thisPermanent isPermanent)
    (.sequentially
      [ .putCounters (.lit 1) (.printed ageCounter) thisPermanent .one true,
        doUnless (sacrifice thisPermanent)
          (.scaled cost (times (.lit 1) (countersOn ageCounter thisPermanent))) ])
/-- "Cumulative upkeep [cost]" with its reminder text. -/
semantic_definition cumulativeUpkeep (cost : Cost) : Ability :=
  .keyword (.named "cumulativeUpkeep") [.cost (.inCaller cost)]
    [cumulativeUpkeepExpansion (.inCaller cost)]
semantic_definition reach : Ability := keyword (.named "reach")
/-- Flying's definition, a static ability: "This creature can't be blocked except by creatures
with flying and/or reach." [CR#702.9b] -/
semantic_definition flyingExpansion : Ability :=
  .static (cant any (block thisCreature)
    (except := byObjects (bare (and [isCreature,
      or [hasKeyword (.the (.named "flying")), hasKeyword reach]]))))
/-- "Flying" with its definition. -/
semantic_definition flying : Ability := .keyword (.named "flying") [] [flyingExpansion]
/-- Bushido N's definition, a triggered ability [CR#702.45a]. -/
semantic_definition bushidoExpansion (count : Amount) : Ability :=
  triggeredOr (does thisCreature block) [does any (block thisCreature)]
    (get thisCreature (.up count) (.up count) (some untilEndOfTurn))
/-- "Bushido N" with its definition. -/
semantic_definition bushido (count : Amount) : Ability :=
  .keyword (.named "bushido") [.number (.inCaller count)] [bushidoExpansion (.inCaller count)]
/-- Modular N's definition, a static ability and a triggered one [CR#702.43a]. -/
semantic_definition modularExpansion (count : Amount) : List Ability :=
  [ .static (entersWithCounters thisPermanent count p1p1Counter),
    when (does thisPermanent die)
      (may (.putCounters (countersOn p1p1Counter thisPermanent) (.printed p1p1Counter)
        (target (.and [isArtifact, isCreature])) .one true)) ]
/-- "Modular N" with its definition. -/
semantic_definition modular (count : Amount) : Ability :=
  .keyword (.named "modular") [.number (.inCaller count)] (modularExpansion (.inCaller count))
/-- Cycling [cost]'s definition, an activated ability: "[Cost], Discard this card: Draw a card."
[CR#702.29a] -/
semantic_definition cyclingExpansion (cost : Cost) : Ability :=
  activated (.compound [cost, .perform (discard this_)]) (draw (.lit 1))
/-- "Cycling [cost]" with its definition. -/
semantic_definition cycling (cost : Cost) : Ability :=
  .keyword (.named "cycling") [.cost (.inCaller cost)] [cyclingExpansion (.inCaller cost)]

/-! ## Ability words -/

semantic_definition adamant (ability : Ability) : Ability := abilityWord "Adamant" ability
semantic_definition addendum (ability : Ability) : Ability := abilityWord "Addendum" ability
semantic_definition alliance (ability : Ability) : Ability := abilityWord "Alliance" ability
semantic_definition battalion (ability : Ability) : Ability := abilityWord "Battalion" ability
semantic_definition bloodrush (ability : Ability) : Ability := abilityWord "Bloodrush" ability
semantic_definition celebration (ability : Ability) : Ability := abilityWord "Celebration" ability
semantic_definition channel (ability : Ability) : Ability := abilityWord "Channel" ability
semantic_definition chroma (ability : Ability) : Ability := abilityWord "Chroma" ability
semantic_definition cohort (ability : Ability) : Ability := abilityWord "Cohort" ability
semantic_definition constellation (ability : Ability) : Ability :=
  abilityWord "Constellation" ability
semantic_definition converge (ability : Ability) : Ability := abilityWord "Converge" ability
semantic_definition councilsDilemma (ability : Ability) : Ability :=
  abilityWord "Council's dilemma" ability
semantic_definition coven (ability : Ability) : Ability := abilityWord "Coven" ability
semantic_definition delirium (ability : Ability) : Ability := abilityWord "Delirium" ability
semantic_definition descend4 (ability : Ability) : Ability := abilityWord "Descend 4" ability
semantic_definition descend8 (ability : Ability) : Ability := abilityWord "Descend 8" ability
semantic_definition disappear (ability : Ability) : Ability := abilityWord "Disappear" ability
semantic_definition domain (ability : Ability) : Ability := abilityWord "Domain" ability
semantic_definition eerie (ability : Ability) : Ability := abilityWord "Eerie" ability
semantic_definition eminence (ability : Ability) : Ability := abilityWord "Eminence" ability
semantic_definition enrage (ability : Ability) : Ability := abilityWord "Enrage" ability
semantic_definition fatefulHour (ability : Ability) : Ability := abilityWord "Fateful hour" ability
semantic_definition fathomlessDescent (ability : Ability) : Ability :=
  abilityWord "Fathomless descent" ability
semantic_definition ferocious (ability : Ability) : Ability := abilityWord "Ferocious" ability
semantic_definition flurry (ability : Ability) : Ability := abilityWord "Flurry" ability
semantic_definition formidable (ability : Ability) : Ability := abilityWord "Formidable" ability
semantic_definition grandeur (ability : Ability) : Ability := abilityWord "Grandeur" ability
semantic_definition hellbent (ability : Ability) : Ability := abilityWord "Hellbent" ability
semantic_definition heroic (ability : Ability) : Ability := abilityWord "Heroic" ability
semantic_definition imprint (ability : Ability) : Ability := abilityWord "Imprint" ability
semantic_definition infusion (ability : Ability) : Ability := abilityWord "Infusion" ability
semantic_definition inspired (ability : Ability) : Ability := abilityWord "Inspired" ability
semantic_definition joinForces (ability : Ability) : Ability := abilityWord "Join forces" ability
semantic_definition kinship (ability : Ability) : Ability := abilityWord "Kinship" ability
semantic_definition landfall (ability : Ability) : Ability := abilityWord "Landfall" ability
semantic_definition lieutenant (ability : Ability) : Ability := abilityWord "Lieutenant" ability
semantic_definition magecraft (ability : Ability) : Ability := abilityWord "Magecraft" ability
semantic_definition metalcraft (ability : Ability) : Ability := abilityWord "Metalcraft" ability
semantic_definition morbid (ability : Ability) : Ability := abilityWord "Morbid" ability
semantic_definition opus (ability : Ability) : Ability := abilityWord "Opus" ability
semantic_definition packTactics (ability : Ability) : Ability := abilityWord "Pack tactics" ability
semantic_definition paradox (ability : Ability) : Ability := abilityWord "Paradox" ability
semantic_definition parley (ability : Ability) : Ability := abilityWord "Parley" ability
semantic_definition radiance (ability : Ability) : Ability := abilityWord "Radiance" ability
semantic_definition raid (ability : Ability) : Ability := abilityWord "Raid" ability
semantic_definition rally (ability : Ability) : Ability := abilityWord "Rally" ability
semantic_definition renew (ability : Ability) : Ability := abilityWord "Renew" ability
semantic_definition repartee (ability : Ability) : Ability := abilityWord "Repartee" ability
semantic_definition revolt (ability : Ability) : Ability := abilityWord "Revolt" ability
semantic_definition secretCouncil (ability : Ability) : Ability :=
  abilityWord "Secret council" ability
semantic_definition spellMastery (ability : Ability) : Ability :=
  abilityWord "Spell mastery" ability
semantic_definition strive (ability : Ability) : Ability := abilityWord "Strive" ability
semantic_definition survival (ability : Ability) : Ability := abilityWord "Survival" ability
semantic_definition sweep (ability : Ability) : Ability := abilityWord "Sweep" ability
semantic_definition temptingOffer (ability : Ability) : Ability :=
  abilityWord "Tempting offer" ability
semantic_definition threshold (ability : Ability) : Ability := abilityWord "Threshold" ability
semantic_definition undergrowth (ability : Ability) : Ability := abilityWord "Undergrowth" ability
semantic_definition valiant (ability : Ability) : Ability := abilityWord "Valiant" ability
semantic_definition vivid (ability : Ability) : Ability := abilityWord "Vivid" ability
semantic_definition void (ability : Ability) : Ability := abilityWord "Void" ability
semantic_definition willOfTheCouncil (ability : Ability) : Ability :=
  abilityWord "Will of the council" ability

/-! ## Keyword abilities -/

semantic_definition absorb (amount : Amount) : Ability :=
  .keyword (.named "absorb") [.number (.inCaller amount)]
    [ .static
        (.damageRule .any .unattributed (.toRecipient thisCreature)
          (.prevent (.some (.inCaller amount)) none)
          .repeatedly) ]
semantic_definition affinity (quality : Predicate) : Ability :=
  .keyword (.named "affinity") [.quality (.inCaller quality)]
    [ .static
        (.costShift thisSpell
          (.less (forEach (.lit 1) (.and [.inCaller quality, youControl])) none)) ]
semantic_definition afflict (amount : Amount) : Ability :=
  .keyword (.named "afflict") [.number (.inCaller amount)]
    [ triggered (.does .any (.block thisCreature))
        (.instruct defendingPlayer (loseLife (.inCaller amount))) ]
semantic_definition afterlife (amount : Amount) : Ability :=
  .keyword (.named "afterlife") [.number (.inCaller amount)]
    [ when (does thisPermanent die)
        (createToken (creatureToken 1 1 [.white, .black] [spirit] [flying]) (.inCaller amount)) ]
semantic_definition aftermath : Ability :=
  .keyword (.named "aftermath") []
    [ .static
        (can (.actor .actor) (cast this_ (from_ := some yourGraveyard))),
      .static
        (cant (.actor .actor)
          (cast this_ (from_ := some (.not yourGraveyard)))),
      .static
        (onlyWhile
          (.instead (would thisSpell (leave stack))
            (exile thisSpell)
            none .repeatedly none)
          (.matches thisSpell (.castFrom graveyard))) ]
semantic_definition amplify (amount : Amount) : Ability :=
  .keyword (.named "amplify") [.number (.inCaller amount)]
    [ .static
        (.asEvent (does thisPermanent enter)
          (.sequentially
            [ .expose .reveal
                (.cards
                  (counted anyNumber
                    (.and [cardIn yourHand, .shares (.subtype .creature) .share thisPermanent]))),
              .establish
                (entersWithCounters thisPermanent (times (.inCaller amount) (.countOf them))
                  p1p1Counter)
                none ])) ]
semantic_definition annihilator (amount : Amount) : Ability :=
  .keyword (.named "annihilator") [.number (.inCaller amount)]
    [ triggered (.does (.actor thisCreature) (.attack none))
        (.instruct defendingPlayer
          (sacrifice (counted (.exactlyOf (.inCaller amount)) isPermanent))) ]
semantic_definition ascend : Ability :=
  .keyword (.named "ascend") []
    [ .static
        (onlyIfSo
          (.whileHolds
            (.and
              [ .compareAmt (countOf (.and [isPermanent, youControl])) .atLeast (.lit 10),
                .not (.matches .you (.hasDesignation citysBlessing none)) ])
            (.gainDesignation .you citysBlessing (some restOfGame)))
          (.matches this_ isPermanentType)),
      .spell none
        (.doIf (.matches this_ isInstantOrSorcery)
          (.doIf
            (.and
              [ .compareAmt (countOf (.and [isPermanent, youControl])) .atLeast (.lit 10),
                .not (.matches .you (.hasDesignation citysBlessing none)) ])
            (.gainDesignation .you citysBlessing (some restOfGame))
            none)
          none) ]
semantic_definition assist : Ability := keyword (.named "assist")
semantic_definition auraSwap (cost : Cost) : Ability :=
  .keyword (.named "auraSwap") [.cost (.inCaller cost)]
    [ activated (.inCaller cost)
        (may
          (.exchange
            (.cardsAcross thisPermanent
              (a (.and [.isCard, .hasSubtype aura, .in_ yourHand]))))) ]
semantic_definition awaken (amount : Amount) (cost : Cost) : Ability :=
  .keyword (.named "awaken") [.number (.inCaller amount), .cost (.inCaller cost)]
    [ .static (can (.actor .you) (cast this_ (payment := .payingInstead (.inCaller cost)))),
      .spell none
        (.doIf (keywordCostPaid (.named "awaken") this_)
          (.sequentially
            [ putCounters (.inCaller amount) (.printed p1p1Counter)
                (target (.and [isLand, youControl])),
              .establish
                (.characteristicChange (that land)
                  [ .typeLine .adds
                      { types := some [.creature], subtypes := some [elemental] },
                    .stat .sets .power (.lit 0),
                    .stat .sets .toughness (.lit 0),
                    .addedAbilities [keyword (.named "haste")] ])
                none ])
          none) ]
semantic_definition backup (amount : Amount) : Ability :=
  .keyword (.named "backup") [.number (.inCaller amount)]
    [ triggered (does thisCreature enter)
        (.sequentially
          [ putCounters (.inCaller amount) (.printed p1p1Counter) (target isCreature),
            .doIf (.matches (that creature) (.otherThan thisCreature))
              (establishThroughout
                (.abilityGrantSelected (that creature) thisCreature
                  (.printedAfter (.named "backup")))
                untilEndOfTurn)
              none ]) ]
semantic_definition banding : Ability := keyword (.named "banding")
semantic_definition bargain : Ability :=
  .keyword (.named "bargain") []
    [ .static
        (.addedCost
          (.perform
            (sacrifice
              (a
                (.or
                  [ .and [isArtifact, isPermanent],
                    .and [isEnchantment, isPermanent],
                    .isToken ]))))
          true .once) ]
semantic_definition battleCry : Ability :=
  .keyword (.named "battleCry") []
    [ triggered (.does (.actor thisCreature) (.attack none))
        (establishThroughout
          (.modification (each (.and [isCreature, isAttacking, .otherThan thisCreature])) .power
            (.up (.lit 1)))
          untilEndOfTurn) ]
semantic_definition bestow (cost : Cost) : Ability :=
  .keyword (.named "bestow") [.cost (.inCaller cost)]
    [.static (.altCost thisSpell (some (.inCaller cost)))]
semantic_definition blitz (cost : Cost) : Ability :=
  .keyword (.named "blitz") [.cost (.inCaller cost)]
    [ .static (can (.actor .you) (cast this_ (payment := .payingInstead (.inCaller cost)))),
      .static
        (.asEvent
          (.does (.actor .you)
            (.pay .itsManaCost .once .paid (some thisSpell) (some (.named "blitz"))))
          (atNext .endStep (sacrifice (.becomes thisSpell)))),
      .static
        (onlyWhile (.abilityGrant thisPermanent (keyword (.named "haste")))
          (keywordCostPaid (.named "blitz") thisPermanent)),
      .static
        (onlyWhile
          (.abilityGrant thisPermanent
            (triggered (does thisPermanent die) (.draw (.lit 1))))
          (keywordCostPaid (.named "blitz") thisPermanent)) ]
semantic_definition bloodthirst (amount : Amount) : Ability :=
  .keyword (.named "bloodthirst") [.number (.inCaller amount)]
    [ .static
        (onlyWhile (entersWithCounters thisPermanent (.inCaller amount) p1p1Counter)
          (happened (.dealDamage .anyAmount .which .any) (.actor anOpponent) soFarThisTurn)) ]
semantic_definition boast (ability : Ability) : Ability :=
  .keyword (.named "boast") [.ability ability]
    [ .qualified ability none (some oncePerTurn)
        (some (.matches thisCreature (happenedTo (.actor .which) (.attack none) soFarThisTurn)))
        none ]
semantic_definition buyback (cost : Cost) : Ability :=
  .keyword (.named "buyback") [.cost (.inCaller cost)]
    [ .static (.addedCost (.inCaller cost) true .once),
      .static
        (onlyWhile
          (.instead (.would .any (.move thisSpell stack (some graveyard) .departure none []))
            (.move thisSpell .anyLocation (some hand) .arrival none [])
            none .repeatedly none)
          (keywordCostPaid (.named "buyback") thisSpell)) ]
semantic_definition casualty (amount : Amount) : Ability :=
  .keyword (.named "casualty") [.number (.inCaller amount)]
    [ .static
        (.addedCost
          (.perform (sacrifice (a (.and [isCreature, hasPower .atLeast (.inCaller amount)]))))
          true .once),
      triggeredIf (.does (.actor .you) (cast thisSpell))
        (keywordCostPaid (.named "casualty") thisSpell)
        (.sequentially [copySpell thisSpell, may (.chooseNewTargets (that .copy))]) ]
semantic_definition champion (quality : Predicate) : Ability :=
  .keyword (.named "champion") [.subject (.inCaller quality)]
    [ triggered (does thisPermanent enter)
        (mayOrElse
          (pay
            (.perform (exile (a (.and [.inCaller quality, youControl, .otherThan thisPermanent]))))
            .once)
          (sacrifice thisPermanent)),
      triggered (does thisPermanent leave)
        (returnTo (linkedObject (.byDeed (.action "exile")) (.action "exile") this_) battlefield
          [.under (ownerOf (that .card))]) ]
semantic_definition changeling : Ability :=
  .keyword (.named "changeling") []
    [.static (.characteristicChange thisCreature [.everyTypeOf .adds .creature])]
semantic_definition chooseABackground : Ability := keyword (.named "chooseABackground")
semantic_definition cipher : Ability :=
  .keyword (.named "cipher") []
    [ .spell none
        (.doIf (.matches this_ .isCard)
          (may
            (.sequentially
              [ exile thisSpell,
                .relate this_ encodedOn (a (.and [isCreature, youControl])) ]))
          none),
      .static
        (.abilityGrant (.counterpart encodedOn this_)
          (triggered
            (.does (.actor thisCreature) (.dealDamage .anyAmount (a .anyPlayer) .combatOnly))
            (may
              (.sequentially
                [ .copy .fromCardZone
                    (the (.and [.isCard, .related encodedOn (some thisCreature)]))
                    (.lit 1) [],
                  may (cast (that .copy) (payment := withoutPaying)) ])))) ]
semantic_definition cleave (cost : Cost) : Ability :=
  .keyword (.named "cleave") [.cost (.inCaller cost)]
    [ .static (can (.actor .you) (cast this_ (payment := .payingInstead (.inCaller cost)))),
      .static
        (onlyWhile (.textChange thisSpell .removeBracketed)
          (keywordCostPaid (.named "cleave") thisSpell)) ]
semantic_definition compleated : Ability := keyword (.named "compleated")
semantic_definition conspire : Ability :=
  .keyword (.named "conspire") []
    [ .static
        (.addedCost
          (.perform
            (tap
              (counted (exactly 2)
                (.and [isUntapped, isCreature, youControl, .shares .color .share thisSpell]))))
          true .once),
      triggeredIf (.does (.actor .you) (cast thisSpell))
        (keywordCostPaid (.named "conspire") thisSpell)
        (.sequentially [copySpell thisSpell, may (.chooseNewTargets (that .copy))]) ]
semantic_definition convoke : Ability :=
  .keyword (.named "convoke") []
    [ .static
        (.paySubstitute thisSpell
          ⟨ tap (a (.and [isUntapped, isCreature, youControl])),
            [.generic, .ofColor (.colorOf (that creature))] ⟩) ]
semantic_definition craft (cost : Cost) : Ability :=
  keywordCosting (.named "craft") (.inCaller cost)
semantic_definition crew (amount : Amount) : Ability :=
  .keyword (.named "crew") [.number (.inCaller amount)]
    [ activated
        (.perform
          (tap
            (withTotal (.stat .power) .atLeast (.inCaller amount)
              (allOf
                (.and
                  [ isUntapped,
                    .and [isCreature, youControl, .otherThan thisPermanent] ])))))
        (establishThroughout
          (.characteristicChange thisPermanent
            [.typeLine .adds { types := some [.artifact, .creature] }])
          untilEndOfTurn) ]
semantic_definition dash (cost : Cost) : Ability :=
  .keyword (.named "dash") [.cost (.inCaller cost)]
    [ .static (can (.actor .you) (cast this_ (payment := .payingInstead (.inCaller cost)))),
      .static
        (.asEvent
          (.does (.actor .you)
            (.pay .itsManaCost .once .paid (some thisSpell) (some (.named "dash"))))
          (atNext .endStep (returnToHand (.becomes thisSpell)))),
      .static
        (onlyWhile (.abilityGrant thisPermanent (keyword (.named "haste")))
          (keywordCostPaid (.named "dash") thisPermanent)) ]
semantic_definition daybound : Ability :=
  .keyword (.named "daybound") []
    [ .static (onlyIfSo (.entryRider thisPermanent .entersTransformed) (.gameIs night)),
      .static
        (.asEvent (.gameBecomes night)
          (.doIf (.matches thisPermanent (.currentFace .front))
            (transform thisPermanent)
            none)),
      .static
        (cant any (transform thisPermanent) (except := some (.byKeyword (.named "daybound")))) ]
semantic_definition deathtouch : Ability := keyword (.named "deathtouch")
semantic_definition decayed : Ability :=
  .keyword (.named "decayed") []
    [ .static (cant (.actor thisCreature) (.block .any)),
      triggered (.does (.actor thisCreature) (.attack none))
        (atNext .endOfCombat (sacrifice thisPermanent)) ]
semantic_definition defender : Ability :=
  .keyword (.named "defender") [] [.static (cant (.actor thisCreature) (.attack none))]
semantic_definition delve : Ability :=
  .keyword (.named "delve") []
    [ .static
        (.paySubstitute thisSpell
          ⟨ exile (a (cardIn yourGraveyard)),
            [.generic] ⟩) ]
semantic_definition demonstrate : Ability :=
  .keyword (.named "demonstrate") []
    [ triggered (.does (.actor .you) (cast thisSpell))
        (may (.sequentially [copySpell thisSpell, may (.chooseNewTargets (that .copy))])
          (some
            (.sequentially
              [ choose anOpponent,
                .instruct (the isChosenPlayer) (copySpell thisSpell),
                .instruct (the isChosenPlayer) (may (.chooseNewTargets (that .copy))) ]))) ]
semantic_definition dethrone : Ability :=
  .keyword (.named "dethrone") []
    [ triggered
        (.does (.actor thisCreature)
          (.attack
            (some
              (the (.and [.anyPlayer, .superlative .max (.playerStat .lifeTotal) .anyPlayer])))))
        (putCounters (.lit 1) (.printed p1p1Counter) thisCreature) ]
semantic_definition devoid : Ability :=
  .keyword (.named "devoid") [] [.static (.characteristicChange this_ [.colors .sets (.some [])])]
semantic_definition devourQuality (quality : Predicate) (amount : Amount) : Ability :=
  .keyword (.named "devour") [.quality (.inCaller quality), .number (.inCaller amount)]
    [ .static
        (.asEvent
          (does thisPermanent enter)
          (may (sacrifice (counted anyNumber (.and [.inCaller quality, .in_ battlefield])))
            (some
              (.establish
                (.entryRider thisPermanent
                  (.withCounters
                    (.arith .times (.inCaller amount)
                      (.countOf (theVerbed (.action "sacrifice") .many)))
                    (.printed p1p1Counter) .fresh))
                none)))) ]
semantic_definition devour (amount : Amount) : Ability :=
  .keyword (.named "devour") [.number (.inCaller amount)] [devourQuality isCreature amount]
semantic_definition disguise (cost : Cost) : Ability :=
  .keyword (.named "disguise") [.cost (.inCaller cost)]
    [ .static
        (can you (cast thisCard (payment := .faceDown faceDownWardedCreature (mana [generic 3])))),
      .static (.specialAction none (.inCaller cost) (turnFaceUp this_)) ]
semantic_definition disturb (cost : Cost) : Ability :=
  keywordCosting (.named "disturb") (.inCaller cost)
semantic_definition doctorsCompanion : Ability := keyword (.named "doctorsCompanion")
semantic_definition doubleStrike : Ability := keyword (.named "doubleStrike")
semantic_definition dredge (amount : Amount) : Ability :=
  .keyword (.named "dredge") [.number amount]
    [ .static
        (onlyWhile
          (.instead (.would (.actor .you) (.draw .anyAmount))
            (may
              (.sequentially
                [ mill amount, returnToHand this_ yourGraveyard ]))
            none .repeatedly none)
          (.compareAmt (countOf (cardIn yourLibrary)) .atLeast amount)) ]
semantic_definition echo (cost : Cost) : Ability := keywordCosting (.named "echo") (.inCaller cost)
semantic_definition embalm (cost : Cost) : Ability :=
  .keyword (.named "embalm") [.cost (.inCaller cost)]
    [ activatedOnlyDuring
        (.compound
          [ .inCaller cost,
            .perform (exileFrom thisCard yourGraveyard) ])
        (create (.lit 1)
          (.copyOf this_
            [ .edits
                [ .colors .sets (.some [.white]),
                  .manaCost none,
                  .typeLine .adds { subtypes := some [zombie] } ] ]))
        asSorcery ]
semantic_definition emerge (cost : Cost) : Ability :=
  .keyword (.named "emerge") [.cost (.inCaller cost)]
    [ .static
        (can (.actor .you)
          (cast this_
            (payment :=
              .payingInstead (.compound [.inCaller cost, .perform (sacrifice (a isCreature))])))),
      .static
        (onlyWhile
          (.costShift thisSpell
            (.less
              (.statOf (.stat .manaValue)
                (linkedObject .theAlternative (.action "sacrifice") thisSpell))
              none))
          (keywordCostPaid (.named "emerge") thisSpell)) ]
semantic_definition emergeFrom (quality : Predicate) (cost : Cost) : Ability :=
  .keyword (.named "emergeFrom") [.quality (.inCaller quality), .cost (.inCaller cost)]
    [ .static
        (can (.actor .you)
          (cast this_
            (payment := .payingInstead
              (.compound
                [ .inCaller cost,
                  .perform (sacrifice (a (.and [.inCaller quality, isPermanent]))) ])))),
      .static
        (onlyWhile
          (.costShift thisSpell
            (.less
              (.statOf (.stat .manaValue)
                (linkedObject .theAlternative (.action "sacrifice") thisSpell))
              none))
          (keywordCostPaid (.named "emergeFrom") thisSpell)) ]
semantic_definition enchant (quality : Predicate) : Ability :=
  .keyword (.named "enchant") [.subject (.inCaller quality)]
    [ .static (.addsTarget thisAura (.inCaller quality)),
      .static (mayBeAttachedTo thisAura (allOf (.inCaller quality))),
      .static
        (.asEvent (.does .any (.move thisAura (.not stack) (some battlefield) .arrival none []))
          (attachTo thisAura (a (.inCaller quality)))) ]
semantic_definition encore (cost : Cost) : Ability :=
  keywordCosting (.named "encore") (.inCaller cost)
semantic_definition enlist : Ability := keyword (.named "enlist")
semantic_definition entwine (cost : Cost) : Ability :=
  .keyword (.named "entwine") [.cost (.inCaller cost)]
    [.static (.modeChoice (.allInstead (some (.inCaller cost)) none))]
semantic_definition epic : Ability :=
  .keyword (.named "epic") []
    [ .spell none
        (establishThroughout (cant (.actor .you) (cast .any)) restOfGame),
      .spell none
        (delayWithin (beginningOfYour .upkeep) restOfGame
          (.sequentially
            [ .copy .fromStack thisSpell (.lit 1)
                [.edits [.removedAbilities (.specified [.term (.the (.named "epic"))])]],
              may (.chooseNewTargets (that .copy)) ])) ]
semantic_definition equip (cost : Cost) : Ability :=
  .keyword (.named "equip") [.cost (.inCaller cost)]
    [ activatedOnlyDuring (.inCaller cost)
        (attachTo thisPermanent (target (.and [isCreature, youControl])))
        asSorcery ]
semantic_definition equipQuality (quality : Predicate) (cost : Cost) : Ability :=
  keywordQualityCosting (.named "equip") (.inCaller quality) (.inCaller cost)
semantic_definition escalate (cost : Cost) : Ability :=
  .keyword (.named "escalate") [.cost (.inCaller cost)]
    [.static (.modeChoice (.extraModesCost (.inCaller cost)))]
semantic_definition escape (cost : Cost) : Ability :=
  .keyword (.named "escape") [.cost (.inCaller cost)]
    [ .static
        (can (.actor .you)
          (cast this_ (from_ := some yourGraveyard) (payment := .payingInstead (.inCaller cost)))) ]
semantic_definition eternalize (cost : Cost) : Ability :=
  .keyword (.named "eternalize") [.cost (.inCaller cost)]
    [ activatedOnlyDuring
        (.compound
          [ .inCaller cost,
            .perform (exileFrom thisCard yourGraveyard) ])
        (create (.lit 1) (.copyOf this_ [ .edits
                  [ .colors .sets (.some [.black]),
                    .stat .sets .power (.lit 4),
                    .stat .sets .toughness (.lit 4),
                    .manaCost none,
                    .typeLine .adds { subtypes := some [zombie] } ] ]) [])
        asSorcery ]
semantic_definition evoke (cost : Cost) : Ability :=
  .keyword (.named "evoke") [.cost (.inCaller cost)]
    [ .static (can (.actor .you) (cast this_ (payment := .payingInstead (.inCaller cost)))),
      triggeredIf (does thisPermanent enter)
        (keywordCostPaid (.named "evoke") thisPermanent)
        (.instruct (controllerOf thisPermanent) (sacrifice thisPermanent)) ]
semantic_definition evolve : Ability :=
  .keyword (.named "evolve") []
    [ triggeredIf (does (a (.and [isCreature, youControl])) enter)
        (.or
          [ .compareAmt (powerOf (that creature)) .greater (powerOf thisCreature),
            .compareAmt (toughnessOf (that creature)) .greater (toughnessOf thisCreature) ])
        (putCounters (.lit 1) (.printed p1p1Counter) thisCreature) ]
semantic_definition exalted : Ability :=
  .keyword (.named "exalted") []
    [ triggeredIf (.does (.actor (a (.and [isCreature, youControl]))) (.attack none))
        (.compareAmt (countOf (.and [isCreature, isAttacking])) .eq (.lit 1))
        (get (that creature) (.up (.lit 1)) (.up (.lit 1)) (some untilEndOfTurn)) ]
semantic_definition exhaust (ability : Ability) : Ability :=
  .keyword (.named "exhaust") [.ability ability]
    [.qualified ability none (some oncePerObject) none none]
semantic_definition exploit : Ability :=
  .keyword (.named "exploit") []
    [ triggered (does thisCreature enter)
        (may (sacrifice (a isCreature))) ]
semantic_definition extort : Ability :=
  .keyword (.named "extort") []
    [ triggered (.does (.actor .you) (cast (a isSpell)))
        (may (pay (.mana [hybridPip .white .black]) .once)
          (some (.sequentially [.instruct eachOpponent (loseLife (.lit 1)), gainLife .thatMuch]))) ]
semantic_definition fabricate (amount : Amount) : Ability :=
  .keyword (.named "fabricate") [.number (.inCaller amount)]
    [ triggered (does thisPermanent enter)
        (mayOrElse (putCounters (.inCaller amount) (.printed p1p1Counter) thisPermanent)
          (createToken (artifactCreatureToken 1 1 [] [servo]) (.inCaller amount))) ]
semantic_definition fading (amount : Amount) : Ability :=
  .keyword (.named "fading") [.number (.inCaller amount)]
    [ .static (entersWithCounters thisPermanent (.inCaller amount) fadeCounter),
      triggered (beginningOfYour .upkeep)
        (doIfDone (removeCounters (oneThrough 1) (some (.printed fadeCounter)) thisPermanent) none
          (some (sacrifice thisPermanent))) ]
/-- "Fear": "This creature can't be blocked except by artifact creatures and/or black
creatures." [CR#702.36b] -/
semantic_definition fear : Ability :=
  .keyword (.named "fear") []
    [ .static (cant any (block thisCreature)
        (except := byObjects (bare (and [isCreature, or [isArtifact, hasColor .black]])))) ]
semantic_definition firebending (amount : Amount) : Ability :=
  .keyword (.named "firebending") [.number (.inCaller amount)]
    [ triggered (.does (.actor thisCreature) (.attack none))
        (.sequentially
          [ .addMana (.inCaller amount) (.runs [[ColorOrColorless.of .red]]) [],
            establishThroughout (.manaRetention .you .thisMana) untilEndOfCombat ]) ]
semantic_definition firstStrike : Ability := keyword (.named "firstStrike")
semantic_definition flanking : Ability :=
  .keyword (.named "flanking") []
    [ triggered
        (.does (.by (a (.and [isCreature, .not (.hasKeyword (.the (.named "flanking")))])))
          (.block thisCreature))
        (get (that creature) (.down (.lit 1)) (.down (.lit 1)) (some untilEndOfTurn)) ]
semantic_definition flash : Ability :=
  .keyword (.named "flash") [] [.static (can you (play thisCard) (timing := some asInstant))]
semantic_definition flashback (cost : Cost) : Ability :=
  .keyword (.named "flashback") [.cost (.inCaller cost)]
    [ .static
        (can (.actor .you)
          (cast this_ (from_ := some yourGraveyard) (payment := .payingInstead (.inCaller cost)))),
      .static
        (onlyWhile
          (.instead (would thisSpell (leave stack))
            (exile thisSpell)
            none .repeatedly none)
          (keywordCostPaid (.named "flashback") thisSpell)) ]
semantic_definition forMirrodin : Ability :=
  .keyword (.named "forMirrodin") []
    [ triggered (does thisEquipment enter)
        (.sequentially
          [ createToken (creatureToken 2 2 [.red] [rebel]),
            attachTo thisEquipment itAsToken ]) ]
semantic_definition forecast (ability : Ability) : Ability :=
  .keyword (.named "forecast") [.ability ability]
    [ .qualified ability (some (duringPart .upkeep (some (ownerOf this_)))) (some oncePerTurn) none
        (some .hand) ]
semantic_definition foretell (cost : Cost) : Ability :=
  .keyword (.named "foretell") [.cost (.inCaller cost)]
    [ .static
        (specialAction (duringPart .turn (some you)) (mana [generic 2])
          (sequentially
            [ exileFaceDown thisCard,
              establishThroughout (visibility .lookAt you (objects it))
                (forAsLongAs (matches_ it (in_ exileZone))),
              establishThroughout
                (can you
                  (cast it (from_ := some exileZone) (payment := payingInstead (.inCaller cost))))
                onALaterTurn ])) ]
semantic_definition fortify (cost : Cost) : Ability :=
  .keyword (.named "fortify") [.cost (.inCaller cost)]
    [ activatedOnlyDuring (.inCaller cost)
        (attachTo thisPermanent (target (.and [isLand, youControl])))
        asSorcery ]
semantic_definition freerunning (cost : Cost) : Ability :=
  keywordCosting (.named "freerunning") (.inCaller cost)
semantic_definition frenzy (amount : Amount) : Ability :=
  keywordNumber (.named "frenzy") (.inCaller amount)
semantic_definition fuse : Ability := keyword (.named "fuse")
semantic_definition gift (promise : Action) : Ability :=
  .keyword (.named "gift") []
    [ .static (.addedCost (.perform (choose anOpponent)) true .once),
      triggeredIf (does thisPermanent enter) (.matches this_ isPermanentType)
        (.doIf (keywordCostPaid (.named "gift") this_)
          (.instruct (chosenAsPaid (.named "gift") this_) (.inCaller promise))
          none),
      .spell none
        (.doIf (.matches this_ isInstantOrSorcery)
          (.doIf (keywordCostPaid (.named "gift") this_)
            (.instruct (chosenAsPaid (.named "gift") this_) (.inCaller promise))
            none)
          none) ]
/-! The licensed phrasings of gift: its templating is not normative ("Gift a card", "Gift a
tapped Fish"), so each is a Template over `gift` [CR#702.174a,702.174d..702.174i]. -/

semantic_template giftACard : Ability := gift (draw (.lit 1))
semantic_template giftAFood : Ability := gift (createToken foodToken)
semantic_template giftATappedFish : Ability :=
  gift (createTappedToken (creatureToken 1 1 [.blue] [fish]))
semantic_template giftATreasure : Ability := gift (createToken treasureToken)
semantic_template giftAnExtraTurn : Ability := gift (extraTurn (.lit 1))
semantic_template giftAnOctopus : Ability :=
  gift (createToken (creatureToken 8 8 [.blue] [octopus]))
semantic_definition graft (amount : Amount) : Ability :=
  .keyword (.named "graft") [.number (.inCaller amount)]
    [ .static (entersWithCounters thisPermanent (.inCaller amount) p1p1Counter),
      triggeredIf
        (does (a (.and [isCreature, .otherThan thisPermanent])) enter)
        (.matches thisPermanent (.hasCounters (some p1p1Counter)))
        (may (.moveCounters (.lit 1) (some (.printed p1p1Counter)) thisPermanent (that creature))) ]
semantic_definition gravestorm : Ability :=
  .keyword (.named "gravestorm") []
    [ triggered (.does (.actor .you) (cast thisSpell))
        (.sequentially
          [ copySpell thisSpell
              (eventCount die (bare isPermanent) soFarThisTurn),
            may (.chooseNewTargets (those .copy)) ]) ]
semantic_definition harmonize (cost : Cost) : Ability :=
  .keyword (.named "harmonize") [.cost (.inCaller cost)]
    [ .static
        (can (.actor .you)
          (cast this_ (from_ := some yourGraveyard)
            (payment := .payingInstead
              (.compound
                [ .inCaller cost,
                  .perform
                    (tap
                      (counted (upTo 1) (.and [isUntapped, .and [isCreature, youControl]]))) ])))),
      .static
        (onlyWhile
          (.costShift thisSpell
            (.less (powerOf (linkedObject .theAlternative (.action "tap") thisSpell)) none))
          (keywordCostPaid (.named "harmonize") thisSpell)),
      .static
        (onlyWhile
          (.instead (would thisSpell (leave stack))
            (exile thisSpell)
            none .repeatedly none)
          (keywordCostPaid (.named "harmonize") thisSpell)) ]
semantic_definition haste : Ability := keyword (.named "haste")
semantic_definition haunt : Ability :=
  .keyword (.named "haunt") []
    [ triggeredIf (does thisPermanent die)
        (.matches this_ isPermanentType)
        (.sequentially
          [ exile it,
            .relate it haunting (target isCreature) ]),
      triggeredIf (.does .any (.move thisSpell stack (some graveyard) .departure none []))
        (.matches this_ isInstantOrSorcery)
        (.sequentially
          [ exile it,
            .relate it haunting (target isCreature) ]) ]
semantic_definition hexproof : Ability :=
  .keyword (.named "hexproof") []
    [.static (cant (.by (allOf (.and [.in_ stack, opponentsControl]))) (.target thisPermanent))]
/-- "Hexproof from [quality]": "This permanent can't be the target of [quality] spells your
opponents control or abilities your opponents control from [quality] sources." It is a hexproof
ability, so it is written as hexproof with a quality (`Definition`, [CR#702.11d]). -/
semantic_definition hexproofFrom (quality : Predicate) : Ability :=
  .keyword (.named "hexproof") [.quality (.inCaller quality)]
    [ .static
        (cant
          (by_
            (allOf
              (and
                [ or
                    [ and [isSpell, .inCaller quality],
                      abilityOf (allOf (and [isSource, .inCaller quality])) ],
                  opponentsControl ])))
          (target thisPermanent)) ]
semantic_definition hideaway (amount : Amount) : Ability :=
  let exiledCard : NounPhrase := theVerbed (.action "exile") .one (some .card)
  .keyword (.named "hideaway") [.number (.inCaller amount)]
    [ when (does thisPermanent enter)
        (sequentially
          [ lookAt (topSlice (.inCaller amount)),
            exileFaceDown (someOf (oneThrough 1) them),
            .move (theRest .object) .anyLocation (some onBottom) .arrival none
              [.inOrder .randomOrder],
            establishThroughout (visibility .lookAt you (objects exiledCard))
              (forAsLongAs (matches_ exiledCard (in_ exileZone))) ]) ]
semantic_definition horsemanship : Ability :=
  .keyword (.named "horsemanship") []
    [ .static
        (cant (.by (allOf (.and [isCreature, .not (.hasKeyword (.the (.named "horsemanship")))])))
          (.block thisCreature)) ]
semantic_definition impending (amount : Amount) (cost : Cost) : Ability :=
  .keyword (.named "impending") [.number (.inCaller amount), .cost (.inCaller cost)]
    [ .static (can (.actor .you) (cast this_ (payment := .payingInstead (.inCaller cost)))),
      .static
        (onlyWhile (entersWithCounters thisPermanent (.inCaller amount) timeCounter)
          (keywordCostPaid (.named "impending") thisPermanent)),
      .static
        (onlyWhile
          (.characteristicChange thisPermanent [.typeLine .loses { types := some [.creature] }])
          (.and
            [ keywordCostPaid (.named "impending") thisPermanent,
              .matches thisPermanent (.hasCounters (some timeCounter)) ])),
      triggeredIf (beginningOfYour .endStep)
        (.and
          [ keywordCostPaid (.named "impending") thisPermanent,
            .matches thisPermanent (.hasCounters (some timeCounter)) ])
        (removeCounters (oneThrough 1) (some (.printed timeCounter)) thisPermanent) ]
semantic_definition improvise : Ability :=
  .keyword (.named "improvise") []
    [ .static
        (.paySubstitute thisSpell
          ⟨ tap (a (.and [isUntapped, isArtifact, youControl])),
            [.generic] ⟩) ]
semantic_definition increment : Ability :=
  .keyword (.named "increment") []
    [ triggeredIf (.does (.actor .you) (cast (a isSpell)))
        (.and
          [ .matches thisPermanent isCreature,
            .or
              [ .compareAmt (manaValueSpentToCast (that .spell)) .greater (powerOf thisCreature),
                .compareAmt (manaValueSpentToCast (that .spell)) .greater
                  (toughnessOf thisCreature) ] ])
        (putCounters (.lit 1) (.printed p1p1Counter) thisCreature) ]
semantic_definition indestructible : Ability :=
  .keyword (.named "indestructible") []
    [ .static
        (cant .any
          (destroy thisPermanent)) ]
semantic_definition infect : Ability := keyword (.named "infect")
semantic_definition infinity (ability : Ability) : Ability :=
  .keyword (.named "infinity") [.ability ability]
    [ .static
        (onlyWhile (.abilityGrant thisPermanent ability)
          (.matches thisPermanent (.hasDesignation harnessed none))) ]
semantic_definition ingest : Ability :=
  .keyword (.named "ingest") []
    [ triggered (.does (.actor thisCreature) (.dealDamage .anyAmount (a .anyPlayer) .combatOnly))
        (.instruct they
          (exile (.slice .top (.lit 1) (libraryOf they)))) ]
/-- "Intimidate": "This creature can't be blocked except by artifact creatures and/or creatures
that share a color with it." [CR#702.13b] -/
semantic_definition intimidate : Ability :=
  .keyword (.named "intimidate") []
    [ .static (cant any (block thisCreature)
        (except := byObjects (bare (and [isCreature,
          or [isArtifact, shares .color .share thisCreature]])))) ]
semantic_definition jobSelect : Ability :=
  .keyword (.named "jobSelect") []
    [ triggered (does thisEquipment enter)
        (.sequentially
          [ createToken (creatureToken 1 1 [] [hero]),
            attachTo thisEquipment itAsToken ]) ]
semantic_definition jumpStart : Ability := keyword (.named "jumpStart")
semantic_definition kicker (cost : Cost) : Ability :=
  .keyword (.named "kicker") [.cost (.inCaller cost)]
    [.static (.addedCost (.inCaller cost) true .once)]
semantic_definition landwalk (quality : Predicate) : Ability :=
  .keyword (.named "landwalk") [.quality (.inCaller quality)]
    [ .static
        (onlyWhile (cant .any (.block thisCreature))
          (exists_ (.and [isLand, .inCaller quality, controlledBy defendingPlayer]))) ]
semantic_definition lifelink : Ability := keyword (.named "lifelink")
semantic_definition livingMetal : Ability :=
  .keyword (.named "livingMetal") []
    [ .static
        (onlyWhile
          (.characteristicChange thisPermanent
            [.typeLine .adds { types := some [.artifact, .creature] }])
          (.duringPart .turn (some .you))) ]
semantic_definition livingWeapon : Ability :=
  .keyword (.named "livingWeapon") []
    [ triggered (does thisEquipment enter)
        (.sequentially
          [ createToken (creatureToken 0 0 [.black] [phyrexian, germ]),
            attachTo thisEquipment itAsToken ]) ]
semantic_definition madness (cost : Cost) : Ability :=
  .keyword (.named "madness") [.cost (.inCaller cost)]
    [ .static
        (.instead (would (a isPlayer) (discard thisCard))
          (enact (.action "discard") (.move thisCard hand (some exileZone) .arrival none []))
          none .repeatedly none),
      triggered
        (.causes (.event (does any (discard thisCard)))
          (does any (.move thisCard hand (some exileZone) .arrival none [])))
        (instruct (ownerOf thisCard)
          (mayOrElse
            (establish
              (can theActor
                (cast thisCard (from_ := some exileZone)
                  (payment := .payingInstead (.inCaller cost)))))
            (putInto thisCard graveyard))) ]
semantic_definition maxSpeed (ability : Ability) : Ability :=
  .keyword (.named "maxSpeed") [.ability ability]
    [.static (onlyWhile (.abilityGrant this_ ability) (.compareAmt (speedOf .you) .eq (.lit 4)))]
semantic_definition mayhem (cost : Cost) : Ability :=
  .keyword (.named "mayhem") [.cost (.inCaller cost)]
    [ .static
        (onlyWhile
          (can you
            (cast thisCard (from_ := some yourGraveyard)
              (payment := .payingInstead (.inCaller cost))))
          (happened (discard thisCard) you soFarThisTurn)) ]
semantic_definition megamorph (cost : Cost) : Ability :=
  .keyword (.named "megamorph") [.cost (.inCaller cost)]
    [ .static (can you (cast thisCard (payment := .faceDown faceDownCreature (mana [generic 3])))),
      .static
        (.specialAction none (.inCaller cost)
          (sequentially [turnFaceUp this_, putCounters (.lit 1) p1p1Counter this_])) ]
semantic_definition melee : Ability :=
  .keyword (.named "melee") []
    [ triggered (.does (.actor thisCreature) (.attack none))
        (get thisCreature
          (.up
            (times (.lit 1)
              (eventCount (.attackWith (a isCreature) (some .which)) (.actor .you) thisCombat)))
          (.up
            (times (.lit 1)
              (eventCount (.attackWith (a isCreature) (some .which)) (.actor .you) thisCombat)))
          (some untilEndOfTurn)) ]
/-- "Menace": "This creature can't be blocked except by two or more creatures."
[CR#702.111b] -/
semantic_definition menace : Ability :=
  .keyword (.named "menace") []
    [ .static (cant any (block thisCreature)
        (except := byObjects (counted (atLeast 2) isCreature))) ]
semantic_definition mentor : Ability :=
  .keyword (.named "mentor") []
    [ triggered (.does (.actor thisCreature) (.attack none))
        (putCounters (.lit 1) (.printed p1p1Counter)
          (target (.and [isCreature, isAttacking, hasPower .less (powerOf thisCreature)]))) ]
semantic_definition miracle (cost : Cost) : Ability :=
  keywordCosting (.named "miracle") (.inCaller cost)
semantic_definition mobilize (amount : Amount) : Ability :=
  .keyword (.named "mobilize") [.number (.inCaller amount)]
    [ triggered (.does (.actor thisCreature) (.attack none))
        (.sequentially
          [ createTappedAttacking (.inCaller amount) (creatureToken 1 1 [.red] [warrior]),
            atNext .endStep
              (sacrifice themAsTokens) ]) ]
semantic_definition moreThanMeetsTheEye (cost : Cost) : Ability :=
  keywordCosting (.named "moreThanMeetsTheEye") (.inCaller cost)
semantic_definition morph (cost : Cost) : Ability :=
  .keyword (.named "morph") [.cost (.inCaller cost)]
    [ .static (can you (cast thisCard (payment := .faceDown faceDownCreature (mana [generic 3])))),
      .static (.specialAction none (.inCaller cost) (turnFaceUp this_)) ]
semantic_definition multikicker (cost : Cost) : Ability :=
  keywordCosting (.named "multikicker") (.inCaller cost)
semantic_definition mutate (cost : Cost) : Ability :=
  .keyword (.named "mutate") [.cost (.inCaller cost)]
    [ .static (can (.actor .you) (cast this_ (payment := .payingInstead (.inCaller cost)))),
      .static
        (.addsTarget thisSpell
          (.and
            [isCreature, .not (.hasSubtype human), ownedBy (ownerOf thisSpell)])),
      .static
        (onlyWhile
          (.instead (.would .any (putOntoBattlefield thisSpell))
            (chooseOne
              (.merge thisSpell (linkedObject .theAlternative (.core .target) thisSpell) .top)
              (.merge thisSpell (linkedObject .theAlternative (.core .target) thisSpell) .bottom))
            none .repeatedly none)
          (keywordCostPaid (.named "mutate") thisSpell)) ]
semantic_definition myriad : Ability :=
  .keyword (.named "myriad") []
    [ triggered (.does (.actor thisCreature) (.attack none))
        (.sequentially
          [ .doForEach (each (.and [isOpponent, .otherThan defendingPlayer]))
              (may
                (createTokenCopy thisCreature
                  [ .entersAs .tapped,
                    .entersAttacking
                      (some (.or [they, a (.and [isPlaneswalker, controlledBy they])])) ])),
            .doIf (.compareAmt (.countOf themAsTokens) .atLeast (.lit 1))
              (atNext .endOfCombat (exile themAsTokens))
              none ]) ]
semantic_definition nightbound : Ability :=
  .keyword (.named "nightbound") []
    [ .static
        (.asEvent (.gameBecomes day)
          (.doIf (.matches thisPermanent isTransformed)
            (transform thisPermanent)
            none)),
      .static
        (cant any (transform thisPermanent) (except := some (.byKeyword (.named "nightbound")))) ]
semantic_definition ninjutsu (cost : Cost) : Ability :=
  .keyword (.named "ninjutsu") [.cost (.inCaller cost)]
    [ activated
        (.compound
          [ .inCaller cost,
            .perform (.expose .reveal (.cards this_)),
            .perform (returnToHand (a (.and [isCreature, isAttacking, isUnblocked, youControl]))) ])
        (putOntoBattlefieldTappedAttacking this_ yourHand) ]
semantic_definition offering (quality : Predicate) : Ability :=
  .keyword (.named "offering") [.quality (.inCaller quality)]
    [ .static
        (.addedCost
          (.perform (sacrifice (a (.and [.inCaller quality, isPermanent]))))
          true .once),
      .static
        (onlyWhile
          (reducedBySymbols thisSpell
            (.manaCostOf (linkedObject .theAdditional (.action "sacrifice") thisSpell)))
          (keywordCostPaid (.named "offering") thisSpell)),
      .static
        (onlyWhile (can (.actor .you) (cast this_) (some asInstant))
          (keywordCostPaid (.named "offering") this_)) ]
semantic_definition offspring (cost : Cost) : Ability :=
  .keyword (.named "offspring") [.cost (.inCaller cost)]
    [ .static (.addedCost (.inCaller cost) true .once),
      triggeredIf (does thisPermanent enter)
        (keywordCostPaid (.named "offspring") thisPermanent)
        (create (.lit 1)
          (.copyOf thisPermanent
            [.edits [.stat .sets .power (.lit 1), .stat .sets .toughness (.lit 1)]])) ]
semantic_definition outlast (cost : Cost) : Ability :=
  .keyword (.named "outlast") [.cost (.inCaller cost)]
    [ activatedOnlyDuring (.compound [.inCaller cost, .tapSymbol])
        (putCounters (.lit 1) (.printed p1p1Counter) thisCreature)
        asSorcery ]
semantic_definition overload (cost : Cost) : Ability :=
  .keyword (.named "overload") [.cost (.inCaller cost)]
    [ .static (can (.actor .you) (cast this_ (payment := .payingInstead (.inCaller cost)))),
      .static
        (onlyWhile (.textChange thisSpell .targetToEach)
          (keywordCostPaid (.named "overload") thisSpell)) ]
semantic_definition paradigm : Ability := keyword (.named "paradigm")
semantic_definition partner : Ability := keyword (.named "partner")
semantic_definition partnerCharacterSelect : Ability := keyword (.named "partnerCharacterSelect")
semantic_definition partnerFatherAndSon : Ability := keyword (.named "partnerFatherAndSon")
semantic_definition partnerFriendsForever : Ability := keyword (.named "partnerFriendsForever")
semantic_definition partnerSurvivors : Ability := keyword (.named "partnerSurvivors")
semantic_definition partnerWith (quality : Predicate) : Ability :=
  keywordQuality (.named "partnerWith") (.inCaller quality)
semantic_definition persist : Ability :=
  .keyword (.named "persist") []
    [ triggeredIf (does thisPermanent die)
        (.not (.matches thisPermanent (.hasCounters (some m1m1Counter))))
        (returnTo thisPermanent battlefield
          [.under (ownerOf thisPermanent), .withCounters (.lit 1) (.printed m1m1Counter) .fresh]) ]
semantic_definition phasing : Ability := keyword (.named "phasing")
semantic_definition plot (cost : Cost) : Ability :=
  .keyword (.named "plot") [.cost (.inCaller cost)]
    [ .static
        (specialAction asSorcery (.inCaller cost)
          (sequentially
            [ exile thisCard,
              establishThroughout
                (can you (cast it (from_ := some exileZone) (payment := withoutPaying))
                  (timing := some asSorcery))
                onALaterTurn ])) ]
semantic_definition poisonous (amount : Amount) : Ability :=
  .keyword (.named "poisonous") [.number (.inCaller amount)]
    [ triggered (.does (.actor thisCreature) (.dealDamage .anyAmount (a .anyPlayer) .combatOnly))
        (putCounters (.inCaller amount) (.printed poison) they) ]
semantic_definition powerUp (ability : Ability) : Ability :=
  .keyword (.named "powerUp") [.ability ability]
    [ .qualified ability none (some oncePerObject) none none,
      .static
        (onlyWhile (reducedBySymbols thisAbility (.manaCostOf thisPermanent))
          (.matches thisPermanent (happenedTo .any (putOntoBattlefield .which) soFarThisTurn))) ]
semantic_definition protection (quality : Predicate) : Ability :=
  .keyword (.named "protection") [.quality (.inCaller quality)]
    [ .static
        (cant
          (.by
            (allOf
              (.or
                [ .and [isSpell, .inCaller quality],
                  .abilityOf (allOf (.and [.isSource, .inCaller quality])) ])))
          (.target thisPermanent)),
      .static
        (cant any (attachTo (allOf (.and [.hasSubtype aura, .inCaller quality])) thisPermanent)),
      .static
        (cant any
          (attachTo
            (allOf
              (.and [.or [.hasSubtype equipment, .hasSubtype fortification], .inCaller quality]))
            thisPermanent)),
      .static
        (.damageRule .any (.dealtBy (allOf (.and [.isSource, .inCaller quality])))
          (.toRecipient thisPermanent)
          (.prevent .all none) .repeatedly),
      .static (cant (.by (allOf (.and [isCreature, .inCaller quality]))) (.block thisCreature)) ]
set_option linter.unusedVariables false in
semantic_definition prototype (cost : Cost) (power toughness : Amount) : Ability :=
  keyword (.named "prototype")
semantic_definition provoke : Ability :=
  .keyword (.named "provoke") []
    [ triggered (.does (.actor thisCreature) (.attack none))
        (may
          (establishThroughout
            (must (.actor (target (.and [isCreature, controlledBy defendingPlayer])))
              (.block thisCreature))
            untilEndOfCombat)
          (some (untap (that creature)))) ]
semantic_definition prowess : Ability :=
  .keyword (.named "prowess") []
    [ triggered
        (.does (.actor .you) (cast (a (.and [.in_ stack, .not isCreature]))))
        (get thisCreature (.up (.lit 1)) (.up (.lit 1)) (some untilEndOfTurn)) ]
semantic_definition prowl (cost : Cost) : Ability :=
  .keyword (.named "prowl") [.cost (.inCaller cost)]
    [ .static
        (onlyWhile (can (.actor .you) (cast this_ (payment := .payingInstead (.inCaller cost))))
          (happened (.dealDamage .anyAmount .which .combatOnly)
            (.actor (a (.and [.shares (.subtype .creature) .share thisSpell, youControl])))
            soFarThisTurn)) ]
semantic_definition rampage (amount : Amount) : Ability :=
  .keyword (.named "rampage") [.number (.inCaller amount)]
    [ triggered (.does .any (.block thisCreature))
        (get thisCreature
          (.up
            (times (.inCaller amount)
              (minus (countOf (.and [isCreature, .inCombat .blockerOf (some thisCreature)]))
                (.lit 1))))
          (.up
            (times (.inCaller amount)
              (minus (countOf (.and [isCreature, .inCombat .blockerOf (some thisCreature)]))
                (.lit 1))))
          (some untilEndOfTurn)) ]
semantic_definition ravenous : Ability :=
  .keyword (.named "ravenous") []
    [ .static (entersWithCounters thisPermanent (.letter .x) p1p1Counter),
      triggeredIf (does thisPermanent enter)
        (.compareAmt (.letter .x) .atLeast (.lit 5))
        (.draw (.lit 1)) ]
semantic_definition readAhead : Ability :=
  .keyword (.named "readAhead") []
    [ .static
        (onlyWhile
          (cant
            (.actor
              (allOf
                (.and
                  [ chapterAbilityOf thisSaga,
                    .not (.compare [.chapter] .eq (countersOn loreCounter thisSaga)) ])))
            .trigger)
          (happened (putOntoBattlefield thisSaga) .any soFarThisTurn)) ]
semantic_definition rebound : Ability :=
  .keyword (.named "rebound") []
    [ .static
        (onlyIfSo
          (.instead (.would .any (.move thisSpell stack (some graveyard) .departure none []))
            (.sequentially
              [ exile thisSpell,
                delay (beginningOf .upkeep (some .you) (.next none))
                  (may (cast this_ (payment := withoutPaying))) ])
            none .repeatedly none)
          (.matches thisSpell (.castFrom yourHand))) ]
semantic_definition reconfigure (cost : Cost) : Ability :=
  .keyword (.named "reconfigure") [.cost (.inCaller cost)]
    [ activatedOnlyDuring (.inCaller cost)
        (attachTo thisPermanent (target (.and [isCreature, .otherThan thisPermanent, youControl])))
        asSorcery,
      .activated (.inCaller cost)
        (enact (.action "attach") (.attachment .unattached thisPermanent none))
        (some asSorcery) none
        (some (.matches thisPermanent (.attachment .attachment none (some (a isCreature)))))
        none ]
semantic_definition recover (cost : Cost) : Ability :=
  .keyword (.named "recover") [.cost (.inCaller cost)]
    [ triggered
        (.does .any (.move (a isCreature) battlefield (some yourGraveyard) .departure none []))
        (may (pay (.inCaller cost) .once) (some (returnToHand this_ yourGraveyard))
          (some (exile this_))) ]
semantic_definition reinforce (amount : Amount) (cost : Cost) : Ability :=
  .keyword (.named "reinforce") [.number (.inCaller amount), .cost (.inCaller cost)]
    [ activated
        (.compound
          [ .inCaller cost,
            .perform (discard this_) ])
        (putCounters (.inCaller amount) (.printed p1p1Counter) (target isCreature)) ]
semantic_definition replicate (cost : Cost) : Ability :=
  .keyword (.named "replicate") [.cost (.inCaller cost)]
    [ .static (.addedCost (.inCaller cost) true .anyNumberOfTimes),
      triggeredIf (.does (.actor .you) (cast thisSpell))
        (keywordCostPaid (.named "replicate") thisSpell)
        (.sequentially
          [ copySpell thisSpell (timesPaid (.byKeyword (.named "replicate")) thisSpell),
            may (.chooseNewTargets (those .copy)) ]) ]
semantic_definition retrace : Ability :=
  .keyword (.named "retrace") []
    [ .static
        (can (.actor .you)
          (cast this_ (from_ := some yourGraveyard)
            (payment := .payingInstead
              (.compound
                [.itsManaCost, .perform (discard (a (.and [.isCard, isLand, .in_ yourHand])))])))) ]
semantic_definition riot : Ability :=
  .keyword (.named "riot") []
    [ .static
        (.asEvent (does thisPermanent enter)
          (mayOrElse
            (.establish (entersWithAdditionalCounters thisPermanent (.lit 1) p1p1Counter) none)
            (gainHaste thisPermanent none))) ]
semantic_definition saddle (amount : Amount) : Ability :=
  .keyword (.named "saddle") [.number (.inCaller amount)]
    [ activatedOnlyDuring
        (.perform
          (tap
            (withTotal (.stat .power) .atLeast (.inCaller amount)
              (allOf
                (.and
                  [ isUntapped,
                    .and [isCreature, youControl, .otherThan thisPermanent] ])))))
        (.gainDesignation thisPermanent saddled (some untilEndOfTurn))
        asSorcery ]
semantic_definition scavenge (cost : Cost) : Ability :=
  .keyword (.named "scavenge") [.cost (.inCaller cost)]
    [ activatedOnlyDuring
        (.compound
          [ .inCaller cost,
            .perform (exileFrom thisCard yourGraveyard) ])
        (putCounters (powerOf (theVerbed (.action "exile") .one (some .card)))
          (.printed p1p1Counter) (target isCreature))
        asSorcery ]
semantic_definition shadow : Ability :=
  .keyword (.named "shadow") []
    [ .static
        (cant (.by (allOf (.and [isCreature, .not (.hasKeyword (.the (.named "shadow")))])))
          (.block thisCreature)),
      .static
        (cant (.by (allOf (.and [isCreature, .hasKeyword (.the (.named "shadow"))])))
          (.block (allOf (.and [isCreature, .not (.hasKeyword (.the (.named "shadow")))])))) ]
semantic_definition shroud : Ability :=
  .keyword (.named "shroud") [] [.static (cant .any (.target thisPermanent))]
semantic_definition skulk : Ability :=
  .keyword (.named "skulk") []
    [ .static
        (cant (.by (allOf (.and [isCreature, hasPower .greater (powerOf thisCreature)])))
          (.block thisCreature)) ]
semantic_definition sneak (cost : Cost) : Ability :=
  .keyword (.named "sneak") [.cost (.inCaller cost)]
    [ .static
        (can you
          (cast thisSpell
            (payment := payingInstead
              (compound
                [ .inCaller cost,
                  perform (returnToHand (a (and [isCreature, isUnblocked, youControl]))) ])))
          (timing := some (and [asInstant, duringPart .declareBlockers (some you)]))) ]
semantic_definition solved (ability : Ability) : Ability :=
  .keyword (.named "solved") [.ability ability]
    [ .qualified ability none none
        (some (.matches thisCase (.hasDesignation DesignationLabel.solved none)))
        none ]
semantic_definition soulbond : Ability :=
  .keyword (.named "soulbond") []
    [ triggeredIf (does thisCreature enter)
        (.and
          [ .matches thisCreature (.and [youControl, .not (.related paired none)]),
            exists_
              (.and
                [ .and [isCreature, youControl, .otherThan thisCreature],
                  .not (.related paired none) ]) ])
        (may
          (.relate thisCreature paired
            (a
              (.and
                [ .and [isCreature, youControl, .otherThan thisCreature],
                  .not (.related paired none) ])))),
      triggeredIf
        (does (a (.and [isCreature, youControl, .otherThan thisCreature])) enter)
        (.and
          [ .matches thisCreature (.and [youControl, .not (.related paired none)]),
            .matches (that creature) (.and [youControl, .not (.related paired none)]) ])
        (may (.relate (that creature) paired thisCreature)) ]
semantic_definition soulshift (amount : Amount) : Ability :=
  .keyword (.named "soulshift") [.number (.inCaller amount)]
    [ triggered (does thisPermanent die)
        (may
          (returnToHand
            (target
              (.and
                [ .isCard,
                  .hasSubtype spirit,
                  hasManaValue .atMost (.inCaller amount),
                  .in_ yourGraveyard ]))
            yourGraveyard)) ]
semantic_definition spectacle (cost : Cost) : Ability :=
  .keyword (.named "spectacle") [.cost (.inCaller cost)]
    [ .static
        (onlyWhile (can (.actor .you) (cast this_ (payment := .payingInstead (.inCaller cost))))
          (happened (loseLife .anyAmount) (.actor anOpponent) soFarThisTurn)) ]
semantic_definition splice_ (quality : Predicate) (cost : Cost) : Ability :=
  .keyword (.named "splice") [.quality (.inCaller quality), .cost (.inCaller cost)]
    [ .static
        (.instead
          (.would (.actor .you)
            (cast (a (.and [isSpell, .inCaller quality]))))
          (may (.expose .reveal (.cards this_))
            (some
              (.establish
                (.conjunction none
                  [ .addedCost (.inCaller cost) false .once,
                    .textChange (that .spell) (.appendTextOf this_) ])
                none)))
          none .repeatedly none) ]
semantic_definition splitSecond : Ability :=
  .keyword (.named "splitSecond") []
    [ .static
        (onlyWhile
          (.conjunction none
            [ cant (bare isPlayer) (cast (allOf anotherSpell)),
              cant (bare isPlayer)
                (activate (allOf (.and [.abilityHead .anyActivated, .not .isManaAbility]))) ])
          (.matches thisSpell (.in_ stack))) ]
semantic_definition spree : Ability :=
  .keyword (.named "spree") [] [.static (.modeChoice (.setQuantity (atLeast 1)))]
semantic_definition squad (cost : Cost) : Ability :=
  .keyword (.named "squad") [.cost (.inCaller cost)]
    [ .static (.addedCost (.inCaller cost) true .anyNumberOfTimes),
      triggeredIf (does thisCreature enter)
        (keywordCostPaid (.named "squad") thisCreature)
        (create (timesPaid (.byKeyword (.named "squad")) thisCreature) (.copyOf thisCreature [])) ]
semantic_definition startYourEngines : Ability := keyword (.named "startYourEngines")
semantic_definition station : Ability :=
  .keyword (.named "station") []
    [ activatedOnlyDuring
        (.perform (tap (a (.and [isCreature, .otherThan thisPermanent, isUntapped, youControl]))))
        (putCounters (powerOf (theVerbed (.action "tap") .one (some creature)))
          (.printed chargeCounter) thisPermanent)
        asSorcery ]
semantic_definition storied : Ability :=
  .keyword (.named "storied") []
    [ .static
        (.whileHolds
          (.and
            [ .compareAmt
                (countOf
                  (.and
                    [ isPermanent,
                      youControl,
                      .or
                        [ isArtifact,
                          .hasSubtype saga,
                          .hasSupertype .legendary ] ]))
                .atLeast (.lit 3),
              .not (.matches .you (.hasDesignation enduringStory none)) ])
          (.gainDesignation .you enduringStory (some restOfGame))) ]
semantic_definition sunburst : Ability :=
  .keyword (.named "sunburst") []
    [ .static
        (onlyWhile (entersWithCounters thisPermanent (colorsSpentToCast thisSpell) p1p1Counter)
          (.matches thisPermanent isCreature)),
      .static
        (onlyWhile (entersWithCounters thisPermanent (colorsSpentToCast thisSpell) chargeCounter)
          (.not (.matches thisPermanent isCreature))) ]
semantic_definition surge (cost : Cost) : Ability :=
  .keyword (.named "surge") [.cost (.inCaller cost)]
    [ .static
        (onlyWhile (can (.actor .you) (cast this_ (payment := .payingInstead (.inCaller cost))))
          (happened (cast (a anotherSpell))
            (.actor (.or [.you, a isTeammate]))
            soFarThisTurn)) ]
semantic_definition suspend (amount : Amount) (cost : Cost) : Ability :=
  keywordNumberCosting (.named "suspend") (.inCaller amount) (.inCaller cost)
semantic_definition teamwork (amount : Amount) : Ability :=
  .keyword (.named "teamwork") [.number (.inCaller amount)]
    [ .static
        (.addedCost
          (.perform
            (tap
              (withTotal (.stat .power) .atLeast (.inCaller amount)
                (allOf (.and [isCreature, youControl])))))
          true .once) ]
semantic_definition tiered : Ability :=
  .keyword (.named "tiered") [] [.static (.modeChoice (.setQuantity (oneThrough 1)))]
semantic_definition toxic (amount : Amount) : Ability :=
  keywordNumber (.named "toxic") (.inCaller amount)
semantic_definition training : Ability :=
  .keyword (.named "training") []
    [ triggered
        (.does
          (.actor
            (.and
              [ thisCreature,
                counted (atLeast 1)
                  (.and
                    [ isCreature,
                      .otherThan thisCreature,
                      hasPower .greater (powerOf thisCreature) ]) ]))
          (.attack none))
        (putCounters (.lit 1) (.printed p1p1Counter) thisCreature) ]
semantic_definition trample : Ability := keyword (.named "trample")
semantic_definition transfigure (cost : Cost) : Ability :=
  .keyword (.named "transfigure") [.cost (.inCaller cost)]
    [ activatedOnlyDuring
        (.compound
          [ .inCaller cost,
            .perform (sacrifice thisPermanent) ])
        (.sequentially
          [ .search yourLibrary (oneThrough 1)
              (.and
                [.isCard, isCreature, hasManaValue .eq (.statOf (.stat .manaValue) thisPermanent)]),
            putOntoBattlefield (theVerbed (.action "search") .one),
            .shuffle (libraryOf .actor) ])
        asSorcery ]
semantic_definition transmute (cost : Cost) : Ability :=
  .keyword (.named "transmute") [.cost (.inCaller cost)]
    [ activatedOnlyDuring
        (.compound
          [ .inCaller cost,
            .perform (discard this_) ])
        (.sequentially
          [ .search yourLibrary (oneThrough 1)
              (.and
                [ .isCard,
                  hasManaValue .eq
                    (.statOf (.stat .manaValue)
                      (theVerbed (.action "discard") .one (some .card))) ]),
            .expose .reveal (.cards (theVerbed (.action "search") .one)),
            putInto (theVerbed (.action "search") .one) hand,
            .shuffle (libraryOf .actor) ])
        asSorcery ]
semantic_definition tribute (amount : Amount) : Ability :=
  .keyword (.named "tribute") [.number (.inCaller amount)]
    [ .static
        (.asEvent (does thisCreature enter)
          (.sequentially
            [ choose anOpponent,
              .instruct (the isChosenPlayer)
                (may
                  (.establish
                    (entersWithAdditionalCounters thisCreature (.inCaller amount) p1p1Counter)
                    none)) ])) ]
semantic_definition typecycling (quality : Predicate) (cost : Cost) : Ability :=
  keywordQualityCosting (.named "typecycling") (.inCaller quality) (.inCaller cost)
semantic_definition umbraArmor : Ability :=
  .keyword (.named "umbraArmor") []
    [ .static
        (.instead (would any (destroy (enchanted .permanent)))
          (sequentially [clearDamage (enchanted .permanent), destroy thisAura])
          none .repeatedly none) ]
semantic_definition undaunted : Ability :=
  .keyword (.named "undaunted") []
    [.static (.costShift thisSpell (.less (forEach (.lit 1) isOpponent) none))]
semantic_definition undying : Ability :=
  .keyword (.named "undying") []
    [ triggeredIf (does thisPermanent die)
        (.not (.matches thisPermanent (.hasCounters (some p1p1Counter))))
        (returnTo thisPermanent battlefield
          [.under (ownerOf thisPermanent), .withCounters (.lit 1) (.printed p1p1Counter) .fresh]) ]
semantic_definition unearth (cost : Cost) : Ability :=
  .keyword (.named "unearth") [.cost (.inCaller cost)]
    [ activatedOnlyDuring (.inCaller cost)
        (.sequentially
          [ returnToBattlefield thisCard (from_ := some yourGraveyard),
            gainHaste thisPermanent none,
            atNext .endStep
              (exile thisPermanent),
            replaceEvent (would thisPermanent leave)
              (exile thisPermanent)
              none ])
        asSorcery ]
semantic_definition unleash : Ability :=
  .keyword (.named "unleash") []
    [ .static
        (.asEvent (does thisPermanent enter)
          (may
            (.establish (entersWithAdditionalCounters thisPermanent (.lit 1) p1p1Counter) none))),
      .static
        (onlyWhile (cant (.actor thisPermanent) (.block .any))
          (.matches thisPermanent (.hasCounters (some p1p1Counter)))) ]
semantic_definition vanishing (amount : Amount) : Ability :=
  .keyword (.named "vanishing") [.number (.inCaller amount)]
    [ .static (entersWithCounters thisPermanent (.inCaller amount) timeCounter),
      triggeredIf (beginningOfYour .upkeep)
        (.matches thisPermanent (.hasCounters (some timeCounter)))
        (removeCounters (oneThrough 1) (some (.printed timeCounter)) thisPermanent),
      triggered (lastCounterRemoved timeCounter thisPermanent)
        (sacrifice thisPermanent) ]
semantic_definition vigilance : Ability := keyword (.named "vigilance")
semantic_definition ward (cost : Cost) : Ability :=
  .keyword (.named "ward") [.cost (.inCaller cost)]
    [ triggered (.does (.by (a (.and [.in_ stack, opponentsControl]))) (.target thisPermanent))
        (.instruct (controllerOf (that .stack))
          (mayOrElse (pay (.inCaller cost) .once)
            (.instruct .you (counter (that .stack))))) ]
semantic_definition warp (cost : Cost) : Ability :=
  .keyword (.named "warp") [.cost (.inCaller cost)]
    [ .static
        (can (.actor .you)
          (cast this_ (from_ := some yourHand) (payment := .payingInstead (.inCaller cost)))),
      .static
        (.asEvent
          (.does (.actor .you)
            (.pay .itsManaCost .once .paid (some thisSpell) (some (.named "warp"))))
          (atNext .endStep
            (.sequentially
              [ exile (.becomes thisSpell),
                establishThroughout
                  (can (.actor (ownerOf it))
                    (cast it (from_ := some exileZone)))
                  onALaterTurn ]))) ]
semantic_definition webSlinging (cost : Cost) : Ability :=
  .keyword (.named "webSlinging") [.cost (.inCaller cost)]
    [ .static
        (can (.actor .you)
          (cast this_ (payment := .payingInstead (.compound
                [ .inCaller cost,
                  .perform (returnToHand (a (.and [isCreature, isTapped, youControl]))) ])))) ]
semantic_definition wither : Ability := keyword (.named "wither")

/-! ## Keyword actions -/

semantic_definition adapt (amount : Amount) : Action :=
  .ownScope (some (.action "adapt"))
    (enact (.action "adapt")
      (.doIf (.not (.matches .actor (.hasCounters (some p1p1Counter))))
        (putCounters (.inCaller amount) (.printed p1p1Counter) .actor)
        none))
semantic_definition behold (quality : Predicate) : Action :=
  .ownScope (some (.action "behold"))
    (enact (.action "behold")
      (chooseOne
        (.expose .reveal (.cards (a (.and [.isCard, .inCaller quality, .in_ (handOf .actor)]))))
        (choose (a (.and [.inCaller quality, isPermanent, actorControls])))))
semantic_definition blight (amount : Amount) : Action :=
  .ownScope (some (.action "blight"))
    (enact (.action "blight")
      (putCounters (.inCaller amount) (.printed m1m1Counter)
        (a (.and [isCreature, actorControls]))))
semantic_definition bolster (amount : Amount) : Action :=
  .ownScope (some (.action "bolster"))
    (enact (.action "bolster")
      (.sequentially
        [ choose
            (a
              (.and
                [ isCreature,
                  actorControls,
                  .superlative .min (.stat .toughness) (.and [isCreature, actorControls]) ])),
          putCounters (.inCaller amount) (.printed p1p1Counter) (that creature) ]))
semantic_definition clash : Action :=
  .ownScope (some (.action "clash"))
    (enact (.action "clash")
      (.sequentially
        [ .expose .reveal (.cards (.slice .top (.lit 1) (libraryOf .actor))),
          may (putInto (that .card) onBottom) ]))
semantic_definition convert (subject : NounPhrase) : Action :=
  .ownScope (some (.action "convert"))
    (enact (.action "convert") (.turnOver (.inCaller (.firstMention "convert.arg0" subject))))
semantic_definition detain (subject : NounPhrase) : Action :=
  .ownScope (some (.action "detain"))
    (enact (.action "detain")
      (establishThroughout
        (.conjunction none
          [ .conjunction none
              [ cant (.actor (.firstMention "detain.0" (.inCaller subject))) (.attack none),
                cant (.actor (.laterMention "detain.0" (.inCaller subject))) (.block .any) ],
            cant .any
              (activate (allOf
                  (.and
                    [ .abilityHead .anyActivated,
                      .abilityOf (.laterMention "detain.0" (.inCaller subject)) ]))) ])
        (untilNextTurnOf (controllerOf this_))))
semantic_definition earthbend (amount : Amount) : Action :=
  .ownScope (some (.action "earthbend"))
    (enact (.action "earthbend")
      (.sequentially
        [ .establish
            (.characteristicChange (target (.and [isLand, actorControls]))
              [ .typeLine .adds { types := some [.creature] },
                .stat .sets .power (.lit 0),
                .stat .sets .toughness (.lit 0),
                .addedAbilities [haste] ])
            none,
          putCounters (.inCaller amount) (.printed p1p1Counter) (that land),
          delay (does (that land) die)
            (returnTo (that land) battlefield [.entersAs .tapped, .under .actor])
            [.does .any (.move (that land) battlefield (some exileZone) .departure none [])] ]))
semantic_definition endure (amount : Amount) : Action :=
  .ownScope (some (.action "endure"))
    (enact (.action "endure")
      (.instruct (controllerOf .actor)
        (chooseOne (putCounters (.inCaller amount) (.printed p1p1Counter) (that .permanent))
          (createToken (creatureTokenOf (.inCaller amount) (.inCaller amount) [.white] [spirit])))))
semantic_definition exert (subject : NounPhrase) : Action :=
  .ownScope (some (.action "exert"))
    (enact (.action "exert") (.skipUntap (.inCaller (.firstMention "exert.arg0" subject)) (.lit 1)))
semantic_definition faceAVillainousChoice (first second : Action) : Action :=
  .ownScope (some (.action "faceAVillainousChoice"))
    (enact (.action "faceAVillainousChoice") (chooseOne (.inCaller first) (.inCaller second)))
semantic_definition forage : Action :=
  .ownScope (some (.action "forage"))
    (enact (.action "forage")
      (chooseOne
        (exileFrom (counted (exactly 3) (cardIn (graveyardOf .actor))) (graveyardOf .actor))
        (sacrifice (a (.and [.hasSubtype food, isPermanent, actorControls])))))
semantic_definition goad (subject : NounPhrase) : Action :=
  .ownScope (some (.action "goad"))
    (enact (.action "goad")
      (.gainDesignation (.inCaller (.firstMention "goad.arg0" subject)) goaded
        (some (untilNextTurnOf (controllerOf this_)))))
semantic_definition harness (subject : NounPhrase) : Action :=
  .ownScope (some (.action "harness"))
    (enact (.action "harness")
      (.doIf
        (.not
          (.matches (.firstMention "harness.0" (.inCaller subject))
            (.hasDesignation harnessed none)))
        (.gainDesignation (.laterMention "harness.0" (.inCaller subject)) harnessed none)
        none))
semantic_definition heal (subject : NounPhrase) : Action :=
  .ownScope (some (.action "heal"))
    (enact (.action "heal") (.clearDamage (.inCaller (.firstMention "heal.arg0" subject))))
semantic_definition investigate : Action :=
  .ownScope (some (.action "investigate")) (enact (.action "investigate") (createToken clueToken))
semantic_definition populate : Action :=
  .ownScope (some (.action "populate"))
    (enact (.action "populate")
      (.sequentially
        [ choose (a (.and [.isToken, isCreature, isPermanent, actorControls])),
          createTokenCopy (that .token) ]))
semantic_definition recruit : Action :=
  .ownScope (some (.action "recruit"))
    (enact (.action "recruit")
      (.sequentially
        [ .draw (.lit 1),
          discard (a (cardIn (handOf .actor))),
          .doIf (.not (.matches (theVerbed (.action "discard") .one (some .card) .thisWay) isLand))
            (createToken (creatureToken 1 1 [.white] [human, soldier]))
            none ]))
semantic_definition search (zone : Location) (quantity : Quantity) (quality : Predicate) : Action :=
  .ownScope (some (.action "search"))
    (enact (.action "search") (.search zone (.inCaller quantity) (.inCaller quality)))
semantic_definition suspect (subject : NounPhrase) : Action :=
  .ownScope (some (.action "suspect"))
    (enact (.action "suspect")
      (.gainDesignation (.inCaller (.firstMention "suspect.arg0" subject)) suspected none))
/-- "The Ring tempts you": the Ring emblem if you have none, then you choose a creature you
control as your Ring-bearer (`Definition`, [CR#701.54a,701.54c]). The emblem's abilities count
the times the Ring has tempted you by the deed's own steps, `tempting`. -/
semantic_definition theRingTemptsYou : Action :=
  let ringBearers : Predicate := and [hasDesignation ringBearer, youControl]
  let tempting : List Action :=
    [ choose (a (and [isCreature, actorControls])), gainDesignation (that creature) ringBearer ]
  let timesTempted : Amount :=
    eventCount
      (.ownScope (some (.action "theRingTemptsYou"))
        (enact (.action "theRingTemptsYou") (sequentially tempting)))
      you thisGame
  .ownScope (some (.action "theRingTemptsYou"))
    (enact (.action "theRingTemptsYou")
      (sequentially
        (doIf (not (exists_ (and [isEmblem, named "The Ring", ownedBy theActor])))
            (getNamedEmblem "The Ring"
              [ static
                  (characteristicChange (allOf ringBearers)
                    [.typeLine .adds { supertypes := some [.legendary] }]),
                static
                  (cant
                    (by_
                      (allOf
                        (and
                          [ isCreature,
                            hasPower .greater
                              (powerOf
                                (a (and [isCreature, hasDesignation ringBearer, youControl]))) ])))
                    (block (allOf (and [isCreature, hasDesignation ringBearer, youControl])))),
                static
                  (onlyWhile
                    (abilityGrant this_
                      (triggered (does (a ringBearers) attack)
                        (sequentially [draw (.lit 1), discard (a (cardIn hand))])))
                    (compareAmt timesTempted .atLeast (.lit 2))),
                static
                  (onlyWhile
                    (abilityGrant this_
                      (triggered
                        (does (by_ (a isCreature)) (block (a ringBearers)))
                        (atNext .endOfCombat
                          (instruct (controllerOf (that creature)) (sacrifice (that creature))))))
                    (compareAmt timesTempted .atLeast (.lit 3))),
                static
                  (onlyWhile
                    (abilityGrant this_
                      (triggered
                        (does (a ringBearers)
                          (dealDamage (to := a isPlayer) (kind := .combatOnly)))
                        (instruct eachOpponent (loseLife (.lit 3)))))
                    (compareAmt timesTempted .atLeast (.lit 4))) ])
            :: tempting)))
semantic_definition timeTravel : Action :=
  .ownScope (some (.action "timeTravel"))
    (enact (.action "timeTravel")
      (.sequentially
        [ choose
            (counted anyNumber
              (.or
                [ .and [isPermanent, actorControls, .hasCounters (some timeCounter)],
                  .and
                    [.isCard, .in_ exileZone, ownedBy .actor, .hasCounters (some timeCounter)] ])),
          .doForEach them
            (chooseOne (putCounters (.lit 1) (.printed timeCounter) it)
              (removeCounters (oneThrough 1) (some (.printed timeCounter)) it)) ]))

end Semantics.Definitions
