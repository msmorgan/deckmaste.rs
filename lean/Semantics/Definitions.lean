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

/-! ## Attach, create, and counter -/

namespace Action

semantic_definition attachTo (subject host : NounPhrase) : Semantics.Action :=
  .ownScope (some (.action "attach"))
    (.enact (.action "attach") (.attachment .attached (.inCaller (.firstMention "attachTo.subject" subject)) (some (.inCaller (.firstMention "attachTo.host" host)))) none none)

semantic_definition create (count : Amount) (token : TokenSpec) (riders : List MoveRider) :
    Semantics.Action :=
  .ownScope (some (.action "create"))
    (.enact (.action "create") (.createObject (.inCaller count) (.token (.inCaller token) (riders.map MoveRider.inCaller)) true none) none none)

/-- Counter is one labeled stack-to-graveyard move [CR#701.6a]. -/
semantic_definition counterSpell (subject : NounPhrase) : Semantics.Action :=
  .ownScope (some (.action "counter"))
    (.enact (.action "counter")
      (.move (.inCaller (.firstMention "counterSpell.subject" subject)) (.zone .stack none) (some (.zone .graveyard none)) .arrival none [])
      none none)

end Action

/-! ## Counters -/

/-- The bench's spelling of a counter declaration's name read as its kind, as a bare
`p1p1Counter` reads in `plugins_v2`. Any other kind is written `.named "<declaration name>"`. -/
semantic_definition p1p1Counter : CounterKind := .named "p1p1Counter"
semantic_definition flyingCounter : CounterKind := .named "flyingCounter"
semantic_definition m1m1Counter : CounterKind := .named "m1m1Counter"
semantic_definition acornCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "acornCounter")
semantic_definition aegisCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "aegisCounter")
semantic_definition ageCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "ageCounter")
semantic_definition aimCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "aimCounter")
semantic_definition arrowCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "arrowCounter")
semantic_definition arrowheadCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "arrowheadCounter")
semantic_definition awakeningCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "awakeningCounter")
semantic_definition baitCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "baitCounter")
semantic_definition blazeCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "blazeCounter")
semantic_definition blessingCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "blessingCounter")
semantic_definition bloodCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "bloodCounter")
semantic_definition bloodlineCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "bloodlineCounter")
semantic_definition bloodstainCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "bloodstainCounter")
semantic_definition bookCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "bookCounter")
semantic_definition boreCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "boreCounter")
semantic_definition bountyCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "bountyCounter")
semantic_definition brainCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "brainCounter")
semantic_definition briberyCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "briberyCounter")
semantic_definition brickCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "brickCounter")
semantic_definition burdenCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "burdenCounter")
semantic_definition cageCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "cageCounter")
semantic_definition carrionCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "carrionCounter")
semantic_definition cellCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "cellCounter")
semantic_definition chargeCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "chargeCounter")
semantic_definition chorusCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "chorusCounter")
semantic_definition collectionCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "collectionCounter")
semantic_definition componentCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "componentCounter")
semantic_definition conquerorCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "conquerorCounter")
semantic_definition contestedCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "contestedCounter")
semantic_definition corpseCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "corpseCounter")
semantic_definition corruptionCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "corruptionCounter")
semantic_definition creditCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "creditCounter")
semantic_definition croakCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "croakCounter")
semantic_definition crystalCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "crystalCounter")
semantic_definition cubeCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "cubeCounter")
semantic_definition currencyCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "currencyCounter")
semantic_definition deathtouchCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "deathtouchCounter")
semantic_definition decayedCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "decayedCounter")
semantic_definition defenseCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "defenseCounter")
semantic_definition delayCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "delayCounter")
semantic_definition depletionCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "depletionCounter")
semantic_definition descentCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "descentCounter")
semantic_definition despairCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "despairCounter")
semantic_definition discoveryCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "discoveryCounter")
semantic_definition divinityCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "divinityCounter")
semantic_definition doomCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "doomCounter")
semantic_definition doubleStrikeCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "doubleStrikeCounter")
semantic_definition dreadCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "dreadCounter")
semantic_definition dreamCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "dreamCounter")
semantic_definition dutyCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "dutyCounter")
semantic_definition eggCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "eggCounter")
semantic_definition elixirCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "elixirCounter")
semantic_definition emberCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "emberCounter")
semantic_definition energy : Semantics.CounterKind :=
  (Semantics.CounterKind.named "energy")
semantic_definition enlightenedCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "enlightenedCounter")
semantic_definition eonCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "eonCounter")
semantic_definition exaltedCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "exaltedCounter")
semantic_definition experience : Semantics.CounterKind :=
  (Semantics.CounterKind.named "experience")
semantic_definition eyeballCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "eyeballCounter")
semantic_definition fadeCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "fadeCounter")
semantic_definition fateCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "fateCounter")
semantic_definition featherCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "featherCounter")
semantic_definition feedingCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "feedingCounter")
semantic_definition fellowshipCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "fellowshipCounter")
semantic_definition fetchCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "fetchCounter")
semantic_definition filibusterCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "filibusterCounter")
semantic_definition filmCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "filmCounter")
semantic_definition finalityCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "finalityCounter")
semantic_definition fireCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "fireCounter")
semantic_definition firstStrikeCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "firstStrikeCounter")
semantic_definition flameCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "flameCounter")
semantic_definition floodCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "floodCounter")
semantic_definition foreshadowCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "foreshadowCounter")
semantic_definition fungusCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "fungusCounter")
semantic_definition furyCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "furyCounter")
semantic_definition fuseCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "fuseCounter")
semantic_definition gemCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "gemCounter")
semantic_definition ghostformCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "ghostformCounter")
semantic_definition glyphCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "glyphCounter")
semantic_definition goldCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "goldCounter")
semantic_definition growthCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "growthCounter")
semantic_definition harmonyCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "harmonyCounter")
semantic_definition hasteCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "hasteCounter")
semantic_definition hatchingCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "hatchingCounter")
semantic_definition hatchlingCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "hatchlingCounter")
semantic_definition hexproofCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "hexproofCounter")
semantic_definition hitCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "hitCounter")
semantic_definition honeCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "honeCounter")
semantic_definition hoofprintCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "hoofprintCounter")
semantic_definition hopeCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "hopeCounter")
semantic_definition hourCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "hourCounter")
semantic_definition hourglassCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "hourglassCounter")
semantic_definition hungerCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "hungerCounter")
semantic_definition iceCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "iceCounter")
semantic_definition impostorCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "impostorCounter")
semantic_definition incarnationCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "incarnationCounter")
semantic_definition incubationCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "incubationCounter")
semantic_definition indestructibleCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "indestructibleCounter")
semantic_definition infectionCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "infectionCounter")
semantic_definition influenceCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "influenceCounter")
semantic_definition ingenuityCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "ingenuityCounter")
semantic_definition ingredientCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "ingredientCounter")
semantic_definition intelCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "intelCounter")
semantic_definition interventionCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "interventionCounter")
semantic_definition invasionCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "invasionCounter")
semantic_definition invitationCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "invitationCounter")
semantic_definition isolationCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "isolationCounter")
semantic_definition javelinCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "javelinCounter")
semantic_definition judgmentCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "judgmentCounter")
semantic_definition kiCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "kiCounter")
semantic_definition kickCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "kickCounter")
semantic_definition knowledgeCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "knowledgeCounter")
semantic_definition landmarkCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "landmarkCounter")
semantic_definition levelCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "levelCounter")
semantic_definition lifelinkCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "lifelinkCounter")
semantic_definition lootCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "lootCounter")
semantic_definition loreCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "loreCounter")
semantic_definition loyaltyCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "loyaltyCounter")
semantic_definition luckCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "luckCounter")
semantic_definition magnetCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "magnetCounter")
semantic_definition manifestationCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "manifestationCounter")
semantic_definition mannequinCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "mannequinCounter")
semantic_definition matrixCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "matrixCounter")
semantic_definition memoryCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "memoryCounter")
semantic_definition menaceCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "menaceCounter")
semantic_definition mineCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "mineCounter")
semantic_definition miningCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "miningCounter")
semantic_definition mireCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "mireCounter")
semantic_definition musicCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "musicCounter")
semantic_definition musterCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "musterCounter")
semantic_definition nestCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "nestCounter")
semantic_definition netCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "netCounter")
semantic_definition nightCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "nightCounter")
semantic_definition oilCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "oilCounter")
semantic_definition omenCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "omenCounter")
semantic_definition oreCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "oreCounter")
semantic_definition pageCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "pageCounter")
semantic_definition painCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "painCounter")
semantic_definition palliationCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "palliationCounter")
semantic_definition paralyzationCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "paralyzationCounter")
semantic_definition petalCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "petalCounter")
semantic_definition petrificationCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "petrificationCounter")
semantic_definition phylacteryCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "phylacteryCounter")
semantic_definition phyresisCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "phyresisCounter")
semantic_definition pinCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "pinCounter")
semantic_definition plagueCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "plagueCounter")
semantic_definition planCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "planCounter")
semantic_definition pointCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "pointCounter")
semantic_definition poison : Semantics.CounterKind :=
  (Semantics.CounterKind.named "poison")
semantic_definition polypCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "polypCounter")
semantic_definition possessionCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "possessionCounter")
semantic_definition pressureCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "pressureCounter")
semantic_definition preyCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "preyCounter")
semantic_definition pupaCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "pupaCounter")
semantic_definition questCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "questCounter")
semantic_definition radCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "radCounter")
semantic_definition rallyCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "rallyCounter")
semantic_definition reachCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "reachCounter")
semantic_definition refineCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "refineCounter")
semantic_definition rejectionCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "rejectionCounter")
semantic_definition reprieveCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "reprieveCounter")
semantic_definition revCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "revCounter")
semantic_definition revivalCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "revivalCounter")
semantic_definition ribbonCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "ribbonCounter")
semantic_definition ritualCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "ritualCounter")
semantic_definition ropeCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "ropeCounter")
semantic_definition rustCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "rustCounter")
semantic_definition saurianCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "saurianCounter")
semantic_definition screamCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "screamCounter")
semantic_definition shadowCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "shadowCounter")
semantic_definition shellCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "shellCounter")
semantic_definition shieldCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "shieldCounter")
semantic_definition shredCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "shredCounter")
semantic_definition silverCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "silverCounter")
semantic_definition skewerCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "skewerCounter")
semantic_definition sleepCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "sleepCounter")
semantic_definition sleightCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "sleightCounter")
semantic_definition slimeCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "slimeCounter")
semantic_definition slumberCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "slumberCounter")
semantic_definition sootCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "sootCounter")
semantic_definition soulCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "soulCounter")
semantic_definition spiteCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "spiteCounter")
semantic_definition sporeCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "sporeCounter")
semantic_definition stashCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "stashCounter")
semantic_definition storageCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "storageCounter")
semantic_definition storyCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "storyCounter")
semantic_definition strifeCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "strifeCounter")
semantic_definition strikeCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "strikeCounter")
semantic_definition studyCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "studyCounter")
semantic_definition stunCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "stunCounter")
semantic_definition supplyCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "supplyCounter")
semantic_definition suspectCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "suspectCounter")
semantic_definition takeoverCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "takeoverCounter")
semantic_definition taskCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "taskCounter")
semantic_definition theftCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "theftCounter")
semantic_definition tideCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "tideCounter")
semantic_definition timeCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "timeCounter")
semantic_definition towerCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "towerCounter")
semantic_definition trampleCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "trampleCounter")
semantic_definition trapCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "trapCounter")
semantic_definition treasureCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "treasureCounter")
semantic_definition unityCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "unityCounter")
semantic_definition valorCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "valorCounter")
semantic_definition velocityCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "velocityCounter")
semantic_definition verseCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "verseCounter")
semantic_definition vigilanceCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "vigilanceCounter")
semantic_definition vitalityCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "vitalityCounter")
semantic_definition voidCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "voidCounter")
semantic_definition vortexCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "vortexCounter")
semantic_definition vowCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "vowCounter")
semantic_definition voyageCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "voyageCounter")
semantic_definition wageCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "wageCounter")
semantic_definition winchCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "winchCounter")
semantic_definition windCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "windCounter")
semantic_definition wishCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "wishCounter")
/-! ## Actions -/

/-- "destroy <subject>" (`Definition`, [CR#701.8]). -/
semantic_definition destroy (subject : NounPhrase) : Action :=
  .ownScope (some (.action "destroy"))
    (.enact (.action "destroy")
      (.move (.inCaller (.firstMention "destroy.subject" subject)) (.zone .battlefield none) (some graveyard) .departure none [])
      none none)
/-- "exile <subject>" (`Definition`, [CR#701.13]). -/
semantic_definition exile (subject : NounPhrase) : Action :=
  .ownScope (some (.action "exile"))
    (.enact (.action "exile")
      (.move (.inCaller (.firstMention "exile.subject" subject)) .anyLocation (some exileZone) .arrival none [])
      none none)
/-- "exile <subject> face down" (`Definition`, [CR#701.13,406.3]). -/
semantic_definition exileFaceDown (subject : NounPhrase) : Action :=
  .ownScope (some (.action "exile"))
    (.enact (.action "exile")
      (.move (.inCaller (.firstMention "exileFaceDown.subject" subject)) .anyLocation (some exileZone) .arrival none
        [(.faceDown exiledFaceDown)])
      none none)
/-- "exile <subject> with N <kind> counters on it" (`Definition`, [CR#701.13]). -/
semantic_definition exileWithCounters (subject : NounPhrase) (amount : Amount) (kind : CounterKind) :
    Action :=
  .ownScope (some (.action "exile"))
    (.enact (.action "exile")
      (.move (.inCaller (.firstMention "exileWithCounters.subject" subject)) .anyLocation (some exileZone) .arrival none
        [(.withCounters (.inCaller amount) (.printed kind) .fresh)])
      none none)
/-- "sacrifice <subject>" (`Definition`, [CR#701.21]). -/
semantic_definition sacrifice (subject : NounPhrase) : Action :=
  .ownScope (some (.action "sacrifice"))
    (.enact (.action "sacrifice")
      (.move (.inCaller (.firstMention "sacrifice.subject" subject)) (.zone .battlefield none) (some graveyard) .departure none [])
      none none)
/-- "transform <subject>" (`Definition`, [CR#701.27]). -/
semantic_definition transform (subject : NounPhrase) : Action :=
  .ownScope (some (.action "transform"))
    (.enact (.action "transform") (.turnOver (.inCaller (.firstMention "transform.subject" subject))) none none)
/-- "meld <subject> into <name>" (`Definition`, [CR#701.42]). -/
semantic_definition meldInto (subject : NounPhrase) (into : String) : Action :=
  .ownScope (some (.action "meld"))
    (.enact (.action "meld")
      (.move (.inCaller (.firstMention "meldInto.subject" subject)) .anyLocation (some battlefield) .arrival none [(.entersMelded into)])
      none none)
/-- Placement-only fragment of manifest for the anaphora bench [CR#701.40a]. -/
semantic_definition manifestPlacement (subject : NounPhrase) : Action :=
  .ownScope (some (.action "manifest"))
    (.enact (.action "manifest") (.move (.inCaller (.firstMention "manifestPlacement.subject" subject)) .anyLocation (some battlefield) .arrival none []) none none)
/-- "tap <subject>" (`Definition`, [CR#701.26a]). -/
semantic_definition tap (subject : NounPhrase) : Action :=
  .ownScope (some (.action "tap"))
    (.enact (.action "tap") (.setStatus .tapped (.inCaller (.firstMention "tap.subject" subject))) none none)
/-- "discard <subject>" (`Definition`, [CR#701.9a]). -/
semantic_definition discard (subject : NounPhrase) : Action :=
  .ownScope (some (.action "discard"))
    (.enact (.action "discard")
      (.move (.inCaller (.firstMention "discard.subject" subject)) (.zone .hand none) (some graveyard) .arrival none [])
      none none)
/-- A bare "shuffle": the performer's own library (`Definition`, [CR#701.24a]). -/
semantic_definition shuffle : Action :=
  .ownScope (some (.action "shuffle"))
    (.enact (.action "shuffle") (.shuffle (libraryOf .actor)) none none)
/-- "untap <subject>" (`Definition`, [CR#701.26b]). -/
semantic_definition untap (subject : NounPhrase) : Action :=
  .ownScope (some (.action "untap"))
    (.enact (.action "untap") (.setStatus .untapped (.inCaller (.firstMention "untap.subject" subject))) none none)
/-- "mill <amount> cards" from <whose> library (`Definition`, [CR#701.17]). -/
semantic_definition mill (amount : Amount) (whose : NounPhrase) : Action :=
  .ownScope (some (.action "mill"))
    (.enact (.action "mill")
      (.move (.slice .top (.inCaller amount) (libraryOf (.inCaller (.firstMention "mill.whose" whose))))
        (.zone .library none) (some graveyard) .arrival none [])
      none none)
/-- "Search your library for <quantity> <p>" (`Definition`, [CR#701.23]). -/
semantic_definition searchLibraryFor (quantity : Quantity) (p : Predicate) : Action :=
  .ownScope (some (.action "search"))
    (.enact (.action "search") (.search (yourLibrary) (.inCaller quantity) (.inCaller p)) none none)
/-- "search <whose>'s graveyard, hand, and library for <q> <p>" (`Definition`, [CR#701.23]). -/
semantic_definition searchZonesOf (whose : NounPhrase) (quantity : Quantity) (p : Predicate) : Action :=
  .ownScope (some (.action "search"))
    (.enact (.action "search")
      (.search
        (.or [.zone .graveyard (some (.inCaller (.firstMention "searchZonesOf.whose" whose))),
          .zone .hand (some (.laterMention "searchZonesOf.whose" (.inCaller whose))),
          .zone .library (some (.laterMention "searchZonesOf.whose" (.inCaller whose)))])
        (.inCaller quantity) (.inCaller p))
      none none)
/-- "search your library and/or graveyard for a <p>" (`Definition`, [CR#701.23]). -/
semantic_definition searchLibraryOrGraveyard (p : Predicate) : Action :=
  .ownScope (some (.action "search"))
    (.enact (.action "search")
      (.search (.or [.zone .library (some .you), .zone .graveyard (some .you)]) (exactly 1)
        (.inCaller p))
      none none)
/-- "reveal their hand" -/
semantic_definition revealTheirHand : Action :=
  .ownScope (some (.action "reveal"))
    (.enact (.action "reveal") (.expose .reveal (.zone (handOf (.inCaller they)))) none none)
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
semantic_definition revealCards (cards : NounPhrase) : Action :=
  .ownScope (some (.action "reveal"))
    (.enact (.action "reveal") (.expose .reveal (.cards (.inCaller (.firstMention "revealCards.cards" cards)))) none none)
/-- "shuffle <subject> into its owner's library": the move, then the shuffle of that library,
which happens even if nothing moved (`Definition`, [CR#701.24c,701.24d]). -/
semantic_definition shuffleInto (subject : NounPhrase) : Action :=
  .ownScope (some (.action "shuffle"))
    (.sequentially
      [ .enact (.action "shuffle")
          (.move (.inCaller (.firstMention "shuffleInto.subject" subject)) .anyLocation (some (.zone .library none)) .arrival none []) none none,
        .shuffle (.zone .library none) ])
/-- "vote for <ballot>" (`Definition`, [CR#701.38]). -/
semantic_definition vote (disclosure : Disclosure) (ballot : Ballot) : Action :=
  .ownScope (some (.action "vote"))
    (.enact (.action "vote") (.vote none disclosure ballot none) none none)
/-- "Starting with <first>, vote for <ballot>" (`Definition`, [CR#701.38]). -/
semantic_definition voteStartingWith (first : NounPhrase) (disclosure : Disclosure) (ballot : Ballot) :
    Action :=
  .ownScope (some (.action "vote"))
    (.enact (.action "vote") (.vote (some (.inCaller (.firstMention "voteStartingWith.first" first))) disclosure ballot none) none none)
/-- "create N <token>" (`Definition`, [CR#701.7]). -/
semantic_definition create (count : Amount) (token : CharacteristicBundle) : Action :=
  Action.create count (.written token) []
/-- "create N <token> tapped and attacking" (`Definition`, [CR#701.7]). -/
semantic_definition createTappedAttacking (count : Amount) (token : CharacteristicBundle) : Action :=
  Action.create count (.written token) [.entersAs .tapped, .entersAttacking none]
namespace Actor

/-- "<performer> discards <subject>": a move from the performer's hand to the graveyard
(`Definition`, [CR#701.9a]). -/
semantic_definition discard (subject : NounPhrase) : Action :=
  .ownScope (some (.action "discard"))
    (.enact (.action "discard")
      (.move (.inCaller (.firstMention "discard.subject" subject)) (.zone .hand (some .actor)) (some graveyard) .arrival none [])
      none none)
end Actor

/-- "scry N" (`Definition`, [CR#701.22a]): the performer's own library. -/
semantic_definition scry (amount : Amount) : Action :=
  .ownScope (some (.action "scry"))
    (.enact (.action "scry") (lookAndSort .actor (.inCaller amount)) none none)
/-- "fateseal N" (`Definition`, [CR#701.29a]). -/
semantic_definition fateseal (whose : NounPhrase) (amount : Amount) : Action :=
  .ownScope (some (.action "fateseal"))
    (.enact (.action "fateseal") (lookAndSort (.inCaller (.firstMention "fateseal.whose" whose)) (.inCaller amount)) none none)
/-- "surveil N" (`Definition`, [CR#701.25a]): the performer's own library. -/
semantic_definition surveil (amount : Amount) : Action :=
  .ownScope (some (.action "surveil"))
    (.enact (.action "surveil") (lookAndSortInto .actor (.inCaller amount) graveyard) none none)
/-- "Proliferate" with its reminder text (`Definition`, [CR#701.34a]). -/
semantic_definition proliferate : Action :=
  .ownScope (some (.action "proliferate"))
    (.enact (.action "proliferate") (.sequentially
      [ .choose none (counted anyNumber
            (.or [ .and [isPermanent, .hasCounters none],
                   .compare [.anyCounter .player] .atLeast (.lit 1) ])) .openly none,
        .putCounters (.lit 1) .own (.eachOf (those .join)) .one true ])
      none none)
/-- "amass <subtype> N" with its reminder text (`Definition`, [CR#701.47a]). -/
semantic_definition amass (subtype : String) (count : Nat) : Action :=
  .ownScope (some (.action "amass"))
    (.enact (.action "amass") (.sequentially
      [ .doIf (.not (exists_ (and [hasSubtype (creatureType "Army"), isCreature, youControl])))
          (create (.lit 1)
            (creatureToken 0 0 [.black] [creatureType subtype, creatureType "Army"]))
          none,
        choose (a (and [hasSubtype (creatureType "Army"), isCreature, youControl])),
        .putCounters (.lit count) (.printed p1p1Counter) (that (.type .creature)) .one true,
        .doIf (.not (.matches it (.hasSubtype (creatureType subtype))))
          (.establish
            (Primitives.StaticSpec.qualityChange it .adds
              (.bundle { characteristics := { subtypes := [creatureType subtype] } } none))
            none)
          none ])
      none none)
/-- "monstrosity N" with its reminder text (`Definition`, [CR#701.37a]). -/
semantic_definition makeMonstrous (amount : Amount) : Action :=
  .ownScope (some (.action "monstrosity"))
    (.enact (.action "monstrosity")
      (.doIf (.not (.matches thisPermanent (.hasDesignation (.named "monstrous") none)))
        (.sequentially
          [ .putCounters (.inCaller amount) (.printed p1p1Counter) thisPermanent .one true,
            .gainDesignation thisPermanent (.named "monstrous") none ])
        none)
      none none)
semantic_definition doubleCounters (on : Semantics.NounPhrase) : Semantics.Action :=
  (Semantics.Action.ownScope (some (Semantics.Deed.action "double")) (Semantics.Action.doubleCounters (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "doubleCounters.on" on))))
semantic_definition exchangeCards (left : Semantics.NounPhrase) (right : Semantics.NounPhrase) : Semantics.Action :=
  (Semantics.Action.ownScope (some (Semantics.Deed.action "exchange")) (Semantics.Action.enact (Semantics.Deed.action "exchange") (Semantics.Action.exchange (Semantics.Exchanged.cardsAcross (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "exchangeCards.left" left)) (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "exchangeCards.right" right)))) none none))
semantic_definition exchangeControl (left : Semantics.NounPhrase) (right : Semantics.NounPhrase) : Semantics.Action :=
  (Semantics.Action.ownScope (some (Semantics.Deed.action "exchange")) (Semantics.Action.enact (Semantics.Deed.action "exchange") (Semantics.Action.exchange (Semantics.Exchanged.controlOf (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "exchangeControl.left" left)) (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "exchangeControl.right" right)))) none none))
semantic_definition exchangeLifeTotals (parties : Semantics.NounPhrase) : Semantics.Action :=
  (Semantics.Action.ownScope (some (Semantics.Deed.action "exchange")) (Semantics.Action.enact (Semantics.Deed.action "exchange") (Semantics.Action.exchange (Semantics.Exchanged.lifeTotals (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "exchangeLifeTotals.parties" parties)))) none none))
semantic_definition exchangeTextBoxes (left : Semantics.NounPhrase) (right : Semantics.NounPhrase) : Semantics.Action :=
  (Semantics.Action.ownScope (some (Semantics.Deed.action "exchange")) (Semantics.Action.enact (Semantics.Deed.action "exchange") (Semantics.Action.establish (Semantics.StaticSpec.textChange (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "exchangeTextBoxes.left" left)) (Semantics.TextChange.exchangeWith (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "exchangeTextBoxes.right" right)))) none) none none))
semantic_definition exchangeValues (left : Semantics.Amount) (right : Semantics.Amount) : Semantics.Action :=
  (Semantics.Action.ownScope (some (Semantics.Deed.action "exchange")) (Semantics.Action.enact (Semantics.Deed.action "exchange") (Semantics.Action.exchange (Semantics.Exchanged.values (Semantics.Amount.inCaller left) (Semantics.Amount.inCaller right))) none none))
semantic_definition exchangeZones (left : Semantics.Location) (right : Semantics.Location) : Semantics.Action :=
  (Semantics.Action.ownScope (some (Semantics.Deed.action "exchange")) (Semantics.Action.enact (Semantics.Deed.action "exchange") (Semantics.Action.exchange (Semantics.Exchanged.zones (Semantics.Location.inCaller left) (Semantics.Location.inCaller right))) none none))
semantic_definition exileFrom (subject : Semantics.NounPhrase) (from_ : Semantics.Location) : Semantics.Action :=
  (Semantics.Action.ownScope (some (Semantics.Deed.action "exile")) (Semantics.Action.enact (Semantics.Deed.action "exile") (Semantics.Action.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "exileFrom.subject" subject)) ((Semantics.Location.inCaller from_)) (some (Semantics.Location.zone Semantics.Zone.exile none)) Semantics.ObservationPoint.arrival none []) none none))
/-! ## Text Changes -/

namespace Actor

/-- "<performer> reveals their hand" (`Definition`, [CR#701.20]). -/
semantic_definition revealHand : Action :=
  .ownScope (some (.action "reveal"))
    (.enact (.action "reveal") (.expose .reveal (.zone (handOf .actor))) none none)
/-- "<performer> amasses <subtype> N" (`Definition`, [CR#701.47a]): resolved via `that (.type .creature)`
and `it` inside `ownScope` without `itPrior` window overrides. -/
semantic_definition amass (subtype : String) (count : Nat) : Action :=
  .ownScope (some (.action "amass"))
    (.enact (.action "amass") (.sequentially
      [ .doIf (.not (exists_ Actor.army))
          (create (.lit 1) (creatureToken 0 0 [.black] [creatureType subtype, creatureType "Army"]))
          none,
        choose (a Actor.army),
        .putCounters (.lit count) (.printed p1p1Counter) (that (.type .creature)) .one true,
        .doIf (.not (.matches it (.hasSubtype (creatureType subtype))))
          (.establish
            (Primitives.StaticSpec.qualityChange it .adds
              (.bundle { characteristics := { subtypes := [creatureType subtype] } } none))
            none)
          none ])
      none none)
/-- "<performer> collects evidence N" (`Definition`, [CR#701.59a]). -/
semantic_definition collectEvidence (amount : Amount) : Action :=
  .ownScope (some (.action "collectEvidence"))
    (.enact (.action "collectEvidence")
      (move (Actor.withTotal (.stat .manaValue) .atLeast (.inCaller amount)
        (allOf (.and [.isCard, .in_ (graveyardOf .actor)]))) exileZone)
      none none)
/-- "<performer> casts <subject> without paying its mana cost [if the resulting spell is …]"
(`Definition`, [CR#701.5,601.2]). -/
semantic_definition castWithoutPaying (subject : NounPhrase) (resulting : Option Predicate) :
    Action :=
  .ownScope (some (.action "cast"))
    (.enact (.action "cast")
      (.procedure .cast (.inCaller (.firstMention "castWithoutPaying.subject" subject)) .withoutPaying (resulting.map Predicate.inCaller) none none)
      none none)
/-- "<performer> discovers N" (`Definition`, [CR#701.57a]). -/
semantic_definition discover (amount : Amount) : Action :=
  let fits : Predicate := .compare [.stat .manaValue] .atMost (.inCaller amount)
  .ownScope (some (.action "discover"))
    (.enact (.action "discover") (.sequentially
      [ move (.run .top (.and [.not isLand, fits]) (libraryOf .actor)) exileZone,
        .withContinuation .optional (castWithoutPaying (that .card) (some fits)) none
          (some (move (that .card) hand)),
        .move (theRest .object) .anyLocation (some onBottom) .arrival none
          [(.inOrder .randomOrder)] ])
      none none)
/-- "<performer> incubates N" (`Definition`, [CR#701.53a,701.53b]). -/
semantic_definition incubate (amount : Amount) : Action :=
  .ownScope (some (.action "incubate"))
    (.enact (.action "incubate") (.createObject (.lit 1) (.token
      (.doubleFaced
        { characteristics :=
            { types := [.artifact], subtypes := [artifactType "Incubator"],
              text := [.activated (.mana [generic 2]) (transform .this) none none none none] } }
        { characteristics :=
            { name := some "Phyrexian Token", types := [.artifact, .creature],
              subtypes := [creatureType "Phyrexian"], power := some (.lit 0),
              toughness := some (.lit 0) } })
      [.withCounters (.inCaller amount) (.printed p1p1Counter) .fresh]) true none)
      none none)
/-- "<performer> waterbends <cost>" (`Definition`, [CR#701.67a,701.67b]). -/
semantic_definition waterbend (cost : Cost) : Action :=
  .ownScope (some (.action "waterbend"))
    (.enact (.action "waterbend")
      (.pay (.withSubstitute (.inCaller cost)
        (substitute (tap (a (.and [isUntapped, .or [isArtifact, isCreature], actorControls]))) [.generic]))
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
        move (theRest .object) graveyard ])
      none none)
/-- "support N" (`Definition`, [CR#701.41a]). -/
semantic_definition support (amount : Amount) : Action :=
  .ownScope (some (.action "support"))
    (.enact (.action "support") (.putCounters (.lit 1) (.printed p1p1Counter)
      (.eachOf (.described (.target (.upToOf (.inCaller amount))) (.and [isCreature, .otherThan .this])))
      .one true)
      none none)
end Actor

/-- "<performer> explores" (`Definition`, [CR#701.44a]). -/
semantic_definition explore : Action :=
  .ownScope (some (.action "explore"))
    (.enact (.action "explore")
      (.instruct (controllerOf (.firstMention "explore.permanent" .actor))
        (.sequentially
          [ revealCards (.slice .top (.lit 1) (libraryOf .actor)),
            .doIf (.matches (that .card) isLand)
              (move (that .card) (.zone .hand (some (ownerOf (that .card)))))
              (some (.sequentially
                [ .putCounters (.lit 1) (.printed p1p1Counter)
                    (.laterMention "explore.permanent" .actor) .one true,
                  offer (move (that .card) graveyard) ])) ]))
      none none)
/-- "<performer> connives [N]" (`Definition`, [CR#701.50a,701.50d]): loops over
`"each nonland card discarded this way"` (`allOf(and([not(land), card, did(any, discard(which), thisWay)]))`). -/
semantic_definition connive (amount : Amount := .lit 1) : Action :=
  .ownScope (some (.action "connive"))
    (.enact (.action "connive")
      (.instruct (controllerOf (.firstMention "connive.permanent" .actor))
        (.sequentially
          [ .draw (.inCaller amount),
            Actor.discard (counted (.exactlyOf (.inCaller amount)) .isCard),
            .doForEach (allOf (.and [.not isLand, .isCard,
                .did ⟨.any, Actor.discard .which, .thisWay⟩]))
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
                (mayPlayDeed (.action "cast") (ownerOf it) it none
                  (some exileZone) none (.payingInstead (.mana [generic 2])))
                (some (.forAsLongAs (.matches it (.in_ exileZone)))))) ])
      none none)
/-- "double <subject>'s power" (`Definition`, [CR#701.10b,701.10c]): it gets +X/+0, X its power. -/
semantic_definition doublePower (subject : NounPhrase) (duration : Option Duration) : Action :=
  .ownScope (some (.action "double"))
    (.establish (.ptModification (.firstMention "doublePower.0" (.inCaller subject))
      (.up (powerOf (.laterMention "doublePower.0" (.inCaller subject)))) (.up (.lit 0)))
      duration)
/-- "triple <subject>'s power and toughness" (`Definition`, [CR#701.11b,701.11c]). -/
semantic_definition triplePowerAndToughness (subject : NounPhrase) (duration : Option Duration) :
    Action :=
  .ownScope (some (.action "triple"))
    (.establish (.ptModification (.firstMention "triplePT.0" (.inCaller subject))
      (.up (times (.lit 2) (powerOf (.laterMention "triplePT.0" (.inCaller subject)))))
      (.up (times (.lit 2) (toughnessOf (.laterMention "triplePT.0" (.inCaller subject))))))
      duration)
/-- "double <performer>'s life total" (`Definition`, [CR#701.10d]), written as that total's set [CR#119.5]. -/
semantic_definition doubleLifeTotal : Action :=
  .ownScope (some (.action "double"))
    (.changeLife (.set (times (.lit 2) (lifeTotalOf .actor))))
/-! ## Abilities -/

/-- "Companion — <condition>" -/
semantic_definition companion (condition : DeckCondition) : Ability :=
  .keyword (.named "companion") [.deckCondition condition] []
/-- "Level up [cost]" [CR#702.87a] -/
semantic_definition levelUp (cost : Cost) : Ability := keywordCosting (.named "levelUp") cost
/-- Renown N's reminder text: "When this creature deals combat damage to a player, if it isn't
renowned, put N +1/+1 counters on it and it becomes renowned." [CR#702.112a] -/
semantic_definition renownExpansion (count : Nat) : Ability :=
  triggeredIf (dealsCombatDamage thisCreature (a .anyPlayer))
    (.not (.matches thisCreature (.hasDesignation (.named "renowned") none)))
    (.sequentially
      [ .putCounters (.lit count) (.printed p1p1Counter) thisCreature .one true,
        .gainDesignation thisCreature (.named "renowned") none ])
/-- Storm's reminder text: "When you cast this spell, copy it for each other spell that was cast
before it this turn. You may choose new targets for the copies." [CR#702.40a] -/
semantic_definition stormExpansion : Ability :=
  when (.does (.actor .you) (.procedure .cast thisSpell .itsOwnCost none none none))
    (.sequentially
      [ .copy .fromStack thisSpell (eventCount (.procedure .cast (a (.and [isSpell, .otherThan thisSpell])) .itsOwnCost none none none) (.actor (a .anyPlayer)) earlierThisTurn) [],
        offer (.chooseNewTargets (.pro (.word .copy) .many)) ])
/-- "Storm" with its reminder text. -/
semantic_definition storm : Ability := .keyword (.named "storm") [] [stormExpansion]
/-- Cascade's reminder text [CR#702.85a]. -/
semantic_definition cascadeExpansion : Ability :=
  let cheaper : Predicate :=
    .compare [.stat .manaValue] .less (.statOf (.stat .manaValue) thisSpell)
  when (.does (.actor .you) (.procedure .cast thisSpell .itsOwnCost none none none))
    (.sequentially
      [ exile (.run .top (.and [.not isLand, cheaper]) (libraryOf .you)),
        offer (Actor.castWithoutPaying (that .card) (some cheaper)),
        .move (allFromAmong (.not (.did ⟨.any, .procedure .cast .which .itsOwnCost none none none, .thisWay⟩)) (theVerbed (.action "exile") .many)) .anyLocation (some onBottom)
          .arrival none [(.inOrder .randomOrder)] ])
/-- "Cascade" with its reminder text. -/
semantic_definition cascade : Ability := .keyword (.named "cascade") [] [cascadeExpansion]
/-- Ripple N's definition [CR#702.60a]: "When you cast this spell, you may reveal the top N cards
of your library. If you do, you may cast any of those cards with the same name as this spell
without paying their mana costs, then put all revealed cards not cast this way on the bottom of
your library in any order." -/
semantic_definition rippleExpansion (amount : Amount) : Ability :=
  when (.does (.actor .you) (.procedure .cast thisSpell .itsOwnCost none none none))
    (Primitives.Action.offer (revealCards (.slice .top amount (libraryOf .you)))
      (some (.sequentially
        [ Actor.castWithoutPaying
            (.someOf (.counted anyNumber) (some (.shares .cardName .share .this)) (those .card)) none,
          .move (allFromAmong (.not (.did ⟨.any, .procedure .cast .which .itsOwnCost none none none, .thisWay⟩)) (theVerbed (.action "reveal") .many (some .card))) .anyLocation
            (some onBottom) .arrival none [(.inOrder .anyOrder)] ]))
      none)
/-- "Ripple N" with its definition [CR#702.60a]. -/
semantic_definition ripple (amount : Amount) : Ability :=
  .keyword (.named "ripple") [.number (Semantics.Amount.inCaller amount)] [rippleExpansion (Semantics.Amount.inCaller amount)]
/-- "Renown N" with its reminder text. -/
semantic_definition renown (count : Nat) : Ability :=
  .keyword (.named "renown") [.number (.lit count)] [renownExpansion count]
/-- Cumulative upkeep's reminder text [CR#702.24a]. -/
semantic_definition cumulativeUpkeepExpansion (cost : Cost) : Ability :=
  triggeredIf (.beginningOf .upkeep (some .you) (.each none))
    (.matches thisPermanent isPermanent)
    (.sequentially
      [ .putCounters (.lit 1) (.printed (.named "ageCounter")) thisPermanent .one true,
        Primitives.Action.offer (.pay (.scaled cost (times (.lit 1) (countersOn (.named "ageCounter") thisPermanent))) .once .paid none none)
          none (some (sacrifice thisPermanent)) ])
/-- "Cumulative upkeep [cost]" with its reminder text. -/
semantic_definition cumulativeUpkeep (cost : Cost) : Ability :=
  .keyword (.named "cumulativeUpkeep") [.cost (Semantics.Cost.inCaller cost)] [cumulativeUpkeepExpansion (Semantics.Cost.inCaller cost)]
/-- Flying's definition, a static ability: "This creature can't be blocked except by creatures
with flying and/or reach." [CR#702.9b] -/
semantic_definition flyingExpansion : Ability :=
  .static (.cant .any (.block thisCreature)
    { except := some (.byObjects (.and [isCreature,
        .or [.hasKeyword (.the (.named "flying")), .hasKeyword (.the (.named "reach"))]])) })
/-- "Flying" with its definition. -/
semantic_definition flying : Ability := .keyword (.named "flying") [] [flyingExpansion]
/-- Bushido N's definition, a triggered ability [CR#702.45a]. -/
semantic_definition bushidoExpansion (count : Nat) : Ability :=
  triggeredOr (blocks thisCreature none) [becomesBlocked thisCreature none]
    (get thisCreature (.up (.lit count)) (.up (.lit count)) (some untilEndOfTurn))
/-- "Bushido N" with its definition. -/
semantic_definition bushido (count : Nat) : Ability :=
  .keyword (.named "bushido") [.number (.lit count)] [bushidoExpansion count]
/-- Modular N's definition, a static ability and a triggered one [CR#702.43a]. -/
semantic_definition modularExpansion (count : Nat) : List Ability :=
  [ .static (entersWithCounters thisPermanent (.lit count) p1p1Counter),
    when (Primitives.GameEvent.dies thisPermanent)
      (offer (.putCounters (countersOn p1p1Counter thisPermanent) (.printed p1p1Counter)
        (target (.and [isArtifact, isCreature])) .one true)) ]
/-- "Modular N" with its definition. -/
semantic_definition modular (count : Nat) : Ability :=
  .keyword (.named "modular") [.number (.lit count)] (modularExpansion count)
/-- Cycling [cost]'s definition, an activated ability: "[Cost], Discard this card: Draw a card."
[CR#702.29a] -/
semantic_definition cyclingExpansion (cost : Cost) : Ability :=
  activated (.compound [cost, .perform (discard .this)]) (draw (.lit 1))
/-- Flash's definition: "You may play this card any time you could cast an instant."
[CR#702.8a] -/
semantic_definition flashExpansion : Ability :=
  .static (mayPlayDeed (.action "play") .you .this none
    none (some asInstant) .itsOwnCost)
/-- Sneak [cost]'s definition [CR#702.190a]. -/
semantic_definition sneakExpansion (cost : Cost) : Ability :=
  .static (mayPlayDeed (.action "cast") .you .this none
    none (some (.and [asInstant, duringPart .declareBlockers (some .you)])) (.payingInstead (.compound [cost,
        .perform (returnTo (a (.and [isCreature, isUnblocked, .hasPossessor .controller .you])) hand [])])))
/-- Morph [cost]'s definition [CR#702.37a,702.37e,116.2b]. -/
semantic_definition morphExpansion (cost : Cost) : List Ability :=
  [ .static (mayPlayDeed (.action "cast") .you .this none
      none none (.faceDown faceDownCreature (.mana [generic 3]))),
    .static (.specialAction none cost (.setStatus .faceUp .this)) ]
/-- Megamorph [cost]'s definition [CR#702.37b]. -/
semantic_definition megamorphExpansion (cost : Cost) : List Ability :=
  [ .static (mayPlayDeed (.action "cast") .you .this none
      none none (.faceDown faceDownCreature (.mana [generic 3]))),
    .static (.specialAction none cost
      (.sequentially [.setStatus .faceUp .this, .putCounters (.lit 1) (.printed p1p1Counter) .this .one true])) ]
/-- Disguise [cost]'s definition [CR#702.168a,702.168d]. -/
semantic_definition disguiseExpansion (cost : Cost) : List Ability :=
  [ .static (mayPlayDeed (.action "cast") .you .this none
      none none (.faceDown faceDownWardedCreature (.mana [generic 3]))),
    .static (.specialAction none cost (.setStatus .faceUp .this)) ]
/-- Foretell [cost]'s definition [CR#702.143a,116.2h]. -/
semantic_definition foretellExpansion (cost : Cost) : List Ability :=
  [ .static (.specialAction (some (duringPart .turn (some .you))) (.mana [generic 2])
      (.sequentially
        [ exileFaceDown .this,
          .establish (.visibility .lookAt .you (.objects it))
            (some (.forAsLongAs (.matches it (.in_ exileZone)))),
          .establish
            (mayPlayDeed (.action "cast") .you it none
              (some exileZone) none (.payingInstead cost))
            (some onALaterTurn) ])) ]
/-- Plot [cost]'s definition [CR#702.170a,702.170d,116.2k]. -/
semantic_definition plotExpansion (cost : Cost) : List Ability :=
  [ .static (.specialAction (some asSorcery) cost
      (.sequentially
        [ exile .this,
          .establish
            (mayPlayDeed (.action "cast") .you it none
              (some exileZone) (some asSorcery) .withoutPaying)
            (some onALaterTurn) ])) ]
/-- Hideaway N's definition [CR#702.75a]. -/
semantic_definition hideawayExpansion (amount : Amount) : List Ability :=
  let exiledCard : NounPhrase := theVerbed (.action "exile") .one (some .card)
  [ when (Primitives.GameEvent.enters thisPermanent none)
      (.sequentially
        [ .expose .lookAt (.cards (.slice .top amount (libraryOf .you))),
          exileFaceDown (someOf (exactly 1) them),
          .move (theRest .object) .anyLocation (some onBottom) .arrival none [(.inOrder .randomOrder)],
          .establish (.visibility .lookAt .you (.objects exiledCard))
            (some (.forAsLongAs (.matches exiledCard (.in_ exileZone)))) ]) ]
/-- "Cycling [cost]" with its definition. -/
semantic_definition cycling (cost : Cost) : Ability :=
  .keyword (.named "cycling") [.cost (Semantics.Cost.inCaller cost)] [cyclingExpansion (Semantics.Cost.inCaller cost)]
/-! ## Ability words -/

semantic_definition adamant (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Adamant") arg0)
semantic_definition addendum (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Addendum") arg0)
semantic_definition alliance (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Alliance") arg0)
semantic_definition battalion (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Battalion") arg0)
semantic_definition bloodrush (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Bloodrush") arg0)
semantic_definition celebration (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Celebration") arg0)
semantic_definition channel (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Channel") arg0)
semantic_definition chroma (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Chroma") arg0)
semantic_definition cohort (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Cohort") arg0)
semantic_definition constellation (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Constellation") arg0)
semantic_definition converge (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Converge") arg0)
semantic_definition councilsDilemma (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Council's dilemma") arg0)
semantic_definition coven (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Coven") arg0)
semantic_definition delirium (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Delirium") arg0)
semantic_definition descend4 (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Descend 4") arg0)
semantic_definition descend8 (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Descend 8") arg0)
semantic_definition disappear (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Disappear") arg0)
semantic_definition domain (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Domain") arg0)
semantic_definition eerie (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Eerie") arg0)
semantic_definition eminence (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Eminence") arg0)
semantic_definition enrage (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Enrage") arg0)
semantic_definition fatefulHour (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Fateful hour") arg0)
semantic_definition fathomlessDescent (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Fathomless descent") arg0)
semantic_definition ferocious (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Ferocious") arg0)
semantic_definition flurry (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Flurry") arg0)
semantic_definition formidable (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Formidable") arg0)
semantic_definition grandeur (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Grandeur") arg0)
semantic_definition hellbent (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Hellbent") arg0)
semantic_definition heroic (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Heroic") arg0)
semantic_definition imprint (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Imprint") arg0)
semantic_definition infusion (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Infusion") arg0)
semantic_definition inspired (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Inspired") arg0)
semantic_definition joinForces (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Join forces") arg0)
semantic_definition kinship (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Kinship") arg0)
semantic_definition landfall (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Landfall") arg0)
semantic_definition lieutenant (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Lieutenant") arg0)
semantic_definition magecraft (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Magecraft") arg0)
semantic_definition metalcraft (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Metalcraft") arg0)
semantic_definition morbid (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Morbid") arg0)
semantic_definition opus (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Opus") arg0)
semantic_definition packTactics (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Pack tactics") arg0)
semantic_definition paradox (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Paradox") arg0)
semantic_definition parley (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Parley") arg0)
semantic_definition radiance (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Radiance") arg0)
semantic_definition raid (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Raid") arg0)
semantic_definition rally (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Rally") arg0)
semantic_definition renew (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Renew") arg0)
semantic_definition repartee (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Repartee") arg0)
semantic_definition revolt (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Revolt") arg0)
semantic_definition secretCouncil (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Secret council") arg0)
semantic_definition spellMastery (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Spell mastery") arg0)
semantic_definition strive (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Strive") arg0)
semantic_definition survival (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Survival") arg0)
semantic_definition sweep (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Sweep") arg0)
semantic_definition temptingOffer (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Tempting offer") arg0)
semantic_definition threshold (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Threshold") arg0)
semantic_definition undergrowth (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Undergrowth") arg0)
semantic_definition valiant (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Valiant") arg0)
semantic_definition vivid (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Vivid") arg0)
semantic_definition void (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Void") arg0)
semantic_definition willOfTheCouncil (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Will of the council") arg0)
/-! ## Designations -/

semantic_definition citysBlessing : Semantics.DesignationLabel :=
  (Semantics.DesignationLabel.named "citysBlessing")
semantic_definition commander : Semantics.DesignationLabel :=
  (Semantics.DesignationLabel.named "commander")
semantic_definition enduringStory : Semantics.DesignationLabel :=
  (Semantics.DesignationLabel.named "enduringStory")
semantic_definition goaded : Semantics.DesignationLabel :=
  (Semantics.DesignationLabel.named "goaded")
semantic_definition harnessed : Semantics.DesignationLabel :=
  (Semantics.DesignationLabel.named "harnessed")
semantic_definition initiative : Semantics.DesignationLabel :=
  (Semantics.DesignationLabel.named "initiative")
semantic_definition leftHalfUnlocked : Semantics.DesignationLabel :=
  (Semantics.DesignationLabel.named "leftHalfUnlocked")
semantic_definition level : Semantics.DesignationLabel :=
  (Semantics.DesignationLabel.named "level")
semantic_definition monarch : Semantics.DesignationLabel :=
  (Semantics.DesignationLabel.named "monarch")
semantic_definition monstrous : Semantics.DesignationLabel :=
  (Semantics.DesignationLabel.named "monstrous")
semantic_definition prepared : Semantics.DesignationLabel :=
  (Semantics.DesignationLabel.named "prepared")
semantic_definition renowned : Semantics.DesignationLabel :=
  (Semantics.DesignationLabel.named "renowned")
semantic_definition rightHalfUnlocked : Semantics.DesignationLabel :=
  (Semantics.DesignationLabel.named "rightHalfUnlocked")
semantic_definition ringBearer : Semantics.DesignationLabel :=
  (Semantics.DesignationLabel.named "ringBearer")
semantic_definition saddled : Semantics.DesignationLabel :=
  (Semantics.DesignationLabel.named "saddled")
semantic_definition DesignationLabel.solved : Semantics.DesignationLabel :=
  (Semantics.DesignationLabel.named "solved")
semantic_definition suspected : Semantics.DesignationLabel :=
  (Semantics.DesignationLabel.named "suspected")
/-! ## Keyword abilities -/

semantic_definition absorb (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "absorb") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.damageRule Semantics.DamageKind.any Semantics.DamageAgent.unattributed (Semantics.DamageScope.toRecipient (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) (Semantics.DamageOp.prevent (Semantics.PreventCut.some (Semantics.Amount.inCaller arg0)) none) Semantics.ReplUse.repeatedly))])
semantic_definition affinity (arg0 : Semantics.Predicate) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "affinity") [(Semantics.KeywordParam.quality (Semantics.Predicate.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.costShift (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) (Semantics.CostShift.less (Semantics.Amount.arith Semantics.ArithOp.times (Semantics.Amount.lit 1) (Semantics.Amount.countOf (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.and [(Semantics.Predicate.inCaller arg0), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)])))) none)))])
semantic_definition afflict (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "afflict") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0))] [(Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Action.block (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none))) [] none [] none none none (Semantics.Action.instruct (Semantics.NounPhrase.combatPlayer Semantics.CombatRole.defending) (Semantics.Action.changeLife (Semantics.Delta.down (Semantics.Amount.inCaller arg0)))))])
semantic_definition afterlife (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "afterlife") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0))] [(Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Action.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) ((Semantics.Location.zone Semantics.Zone.battlefield none)) (some (Semantics.Location.zone Semantics.Zone.graveyard none)) Semantics.ObservationPoint.departure none [])) [] none [] none none none (Semantics.Action.createObject (Semantics.Amount.inCaller arg0) (Semantics.CreationSpec.token (Semantics.TokenSpec.written ({ characteristics := ({ name := none, cost := none, colors := [Semantics.Color.white, Semantics.Color.black], supertypes := [], types := [Semantics.CardType.creature], subtypes := [(Semantics.Subtype.of Semantics.CardType.creature "Spirit")], text := [(Semantics.Ability.keyword (Semantics.KeywordLabel.named "flying") [] [(Semantics.Ability.static (Semantics.StaticSpec.cant Semantics.Subject.any (Semantics.Templates.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.block) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) ({ except := (some (Semantics.DeonticExcept.byObjects (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.or [(Semantics.Predicate.hasKeyword (Semantics.KeywordTerm.the (Semantics.KeywordLabel.named "flying"))), (Semantics.Predicate.hasKeyword (Semantics.KeywordTerm.the (Semantics.KeywordLabel.named "reach")))])]))) } : Semantics.DeonticMods)))])], power := (some (Semantics.Amount.lit 1)), toughness := (some (Semantics.Amount.lit 1)), loyalty := none, defense := none } : Semantics.Characteristics), qualities := [] } : Semantics.CharacteristicBundle)) []) true none))])
semantic_definition aftermath : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "aftermath") [] [(Semantics.Ability.static (Semantics.StaticSpec.can (Semantics.Subject.actor Semantics.NounPhrase.actor) (Semantics.Action.procedure Semantics.Procedure.cast Semantics.NounPhrase.this Semantics.PlayPayment.itsOwnCost none (some (Semantics.Location.zone Semantics.Zone.graveyard (some Semantics.NounPhrase.you))) none) ({} : Semantics.DeonticMods))), (Semantics.Ability.static (Semantics.StaticSpec.cant (Semantics.Subject.actor Semantics.NounPhrase.actor) (Semantics.Action.procedure Semantics.Procedure.cast Semantics.NounPhrase.this Semantics.PlayPayment.itsOwnCost none (some (Semantics.Location.not (Semantics.Location.zone Semantics.Zone.graveyard (some Semantics.NounPhrase.you)))) none) ({} : Semantics.DeonticMods))), (Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.instead (Semantics.WouldClause.would Semantics.Subject.any (Semantics.Action.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) ((Semantics.Location.zone Semantics.Zone.stack none)) none Semantics.ObservationPoint.departure none [])) (Semantics.Action.ownScope (some (Semantics.Deed.action "exile")) (Semantics.Action.enact (Semantics.Deed.action "exile") (Semantics.Action.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.exile none)) Semantics.ObservationPoint.arrival none []) none none)) none Semantics.ReplUse.repeatedly none) (Semantics.Condition.matches (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) (Semantics.Predicate.castFrom (Semantics.Location.zone Semantics.Zone.graveyard none))) Semantics.CondMarking.asLongAs))])
semantic_definition amplify (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "amplify") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.asEvent (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Action.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.battlefield none)) Semantics.ObservationPoint.arrival none [])) (Semantics.Action.sequentially [(Semantics.Action.expose Semantics.ExposeVerb.reveal (Semantics.Exposed.cards (Semantics.NounPhrase.described (Semantics.DetPhrase.count (Semantics.Quantity.range none none) none) (Semantics.Predicate.and [(Semantics.Predicate.and [Semantics.Predicate.isCard, (Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.hand (some Semantics.NounPhrase.you)))]), (Semantics.Predicate.shares (Semantics.QualitySort.subtype Semantics.CardType.creature) Semantics.Agreement.share (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this))])))), (Semantics.Action.establish (Semantics.StaticSpec.entryRider (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.MoveRider.withCounters (Semantics.Amount.arith Semantics.ArithOp.times (Semantics.Amount.inCaller arg0) (Semantics.Amount.countOf (Semantics.NounPhrase.pro Semantics.Reach.bare Semantics.Plurality.many))) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) Semantics.EntryCounterMark.fresh)) none)])))])
semantic_definition annihilator (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "annihilator") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0))] [(Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) (Semantics.Action.attack none)) [] none [] none none none (Semantics.Action.instruct (Semantics.NounPhrase.combatPlayer Semantics.CombatRole.defending) (Semantics.Action.ownScope (some (Semantics.Deed.action "sacrifice")) (Semantics.Action.enact (Semantics.Deed.action "sacrifice") (Semantics.Action.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.described (Semantics.DetPhrase.count (Semantics.Quantity.exactlyOf (Semantics.Amount.inCaller arg0)) none) (Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.battlefield none)))) ((Semantics.Location.zone Semantics.Zone.battlefield none)) (some (Semantics.Location.zone Semantics.Zone.graveyard none)) Semantics.ObservationPoint.departure none []) none none))))])
semantic_definition ascend : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "ascend") [] [(Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.whileHolds (Semantics.Condition.and [(Semantics.Condition.compareAmt (Semantics.Amount.countOf (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.and [(Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.battlefield none)), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)]))) Semantics.Comparator.atLeast (Semantics.Amount.lit 10)), (Semantics.Condition.not (Semantics.Condition.matches Semantics.NounPhrase.you (Semantics.Predicate.hasDesignation (Semantics.DesignationLabel.named "citysBlessing") none)))]) (Semantics.Action.gainDesignation Semantics.NounPhrase.you (Semantics.DesignationLabel.named "citysBlessing") (some (Semantics.Duration.between none none)))) (Semantics.Condition.matches Semantics.NounPhrase.this (Semantics.Predicate.or [(Semantics.Predicate.hasType Semantics.CardType.artifact), (Semantics.Predicate.hasType Semantics.CardType.battle), (Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasType Semantics.CardType.enchantment), (Semantics.Predicate.hasType Semantics.CardType.land), (Semantics.Predicate.hasType Semantics.CardType.planeswalker)])) Semantics.CondMarking.ifSo)), (Semantics.Ability.spell none (Semantics.Action.doIf (Semantics.Condition.matches Semantics.NounPhrase.this (Semantics.Predicate.or [(Semantics.Predicate.hasType Semantics.CardType.instant), (Semantics.Predicate.hasType Semantics.CardType.sorcery)])) (Semantics.Action.doIf (Semantics.Condition.and [(Semantics.Condition.compareAmt (Semantics.Amount.countOf (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.and [(Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.battlefield none)), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)]))) Semantics.Comparator.atLeast (Semantics.Amount.lit 10)), (Semantics.Condition.not (Semantics.Condition.matches Semantics.NounPhrase.you (Semantics.Predicate.hasDesignation (Semantics.DesignationLabel.named "citysBlessing") none)))]) (Semantics.Action.gainDesignation Semantics.NounPhrase.you (Semantics.DesignationLabel.named "citysBlessing") (some (Semantics.Duration.between none none))) none) none))])
semantic_definition assist : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "assist") [] [])
semantic_definition auraSwap (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "auraSwap") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.activated (Semantics.Cost.inCaller arg0) (Semantics.Action.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Action.exchange (Semantics.Exchanged.cardsAcross (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [Semantics.Predicate.isCard, (Semantics.Predicate.hasSubtype (Semantics.Subtype.of Semantics.CardType.enchantment "Aura")), (Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.hand (some Semantics.NounPhrase.you)))])))) none none) none none none none)])
semantic_definition awaken (arg0 : Semantics.Amount) (arg1 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "awaken") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0)), (Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg1))] [(Semantics.Ability.static (Semantics.StaticSpec.can (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Action.procedure Semantics.Procedure.cast Semantics.NounPhrase.this (Semantics.PlayPayment.payingInstead (Semantics.Cost.inCaller arg1)) none none none) ({} : Semantics.DeonticMods))), (Semantics.Ability.spell none (Semantics.Action.doIf (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "awaken")) none) Semantics.NounPhrase.this) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) (Semantics.Action.sequentially [(Semantics.Action.putCounters (Semantics.Amount.inCaller arg0) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) (Semantics.NounPhrase.described (Semantics.DetPhrase.target (Semantics.Quantity.range (some 1) (some 1))) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.land), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)])) Semantics.CounterBatch.one true), (Semantics.Action.establish (Semantics.StaticSpec.characteristicChange (Semantics.NounPhrase.pro (Semantics.Reach.word (Semantics.NounWord.type Semantics.CardType.land)) Semantics.Plurality.one) [(Semantics.CharacteristicEdit.typeLine Semantics.QualityOp.adds ({ supertypes := none, types := (some [Semantics.CardType.creature]), subtypes := (some [(Semantics.Subtype.of Semantics.CardType.creature "Elemental")]), retained := none } : Semantics.TypeLineChanges)), (Semantics.CharacteristicEdit.stat Semantics.QualityOp.sets Semantics.CharacteristicStat.power (Semantics.Amount.lit 0)), (Semantics.CharacteristicEdit.stat Semantics.QualityOp.sets Semantics.CharacteristicStat.toughness (Semantics.Amount.lit 0)), (Semantics.CharacteristicEdit.addedAbilities [(Semantics.Ability.keyword (Semantics.KeywordLabel.named "haste") [] [])])]) none)]) none))])
semantic_definition backup (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "backup") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0))] [(Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Action.move (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.battlefield none)) Semantics.ObservationPoint.arrival none [])) [] none [] none none none (Semantics.Action.sequentially [(Semantics.Action.putCounters (Semantics.Amount.inCaller arg0) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) (Semantics.NounPhrase.described (Semantics.DetPhrase.target (Semantics.Quantity.range (some 1) (some 1))) (Semantics.Predicate.hasType Semantics.CardType.creature)) Semantics.CounterBatch.one true), (Semantics.Action.doIf (Semantics.Condition.matches (Semantics.NounPhrase.pro (Semantics.Reach.word (Semantics.NounWord.type Semantics.CardType.creature)) Semantics.Plurality.one) (Semantics.Predicate.otherThan (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none))) (Semantics.Action.establish (Semantics.StaticSpec.abilityGrantSelected (Semantics.NounPhrase.pro (Semantics.Reach.word (Semantics.NounWord.type Semantics.CardType.creature)) Semantics.Plurality.one) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none) (Semantics.AbilitySelection.printedAfter (Semantics.KeywordLabel.named "backup"))) (some (Semantics.Duration.between none (some (Semantics.GameEvent.endOf Semantics.TurnPart.turn none Semantics.TurnRef.current))))) none)]))])
semantic_definition banding : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "banding") [] [])
semantic_definition bargain : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "bargain") [] [(Semantics.Ability.static (Semantics.StaticSpec.addedCost (Semantics.Cost.perform (Semantics.Action.ownScope (some (Semantics.Deed.action "sacrifice")) (Semantics.Action.enact (Semantics.Deed.action "sacrifice") (Semantics.Action.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.or [(Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.artifact), (Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.battlefield none))]), (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.enchantment), (Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.battlefield none))]), Semantics.Predicate.isToken]))) ((Semantics.Location.zone Semantics.Zone.battlefield none)) (some (Semantics.Location.zone Semantics.Zone.graveyard none)) Semantics.ObservationPoint.departure none []) none none))) true Semantics.PayTimes.once))])
semantic_definition battleCry : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "battleCry") [] [(Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) (Semantics.Action.attack none)) [] none [] none none none (Semantics.Action.establish (Semantics.StaticSpec.modification (Semantics.NounPhrase.described Semantics.DetPhrase.each (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.inCombat Semantics.CombatRelation.attackerOf none), (Semantics.Predicate.otherThan (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none))])) Semantics.Stat.power (Semantics.Delta.up (Semantics.Amount.lit 1))) (some (Semantics.Duration.between none (some (Semantics.GameEvent.endOf Semantics.TurnPart.turn none Semantics.TurnRef.current))))))])
semantic_definition bestow (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "bestow") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.altCost (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) (some (Semantics.Cost.inCaller arg0))))])
semantic_definition blitz (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "blitz") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.can (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Action.procedure Semantics.Procedure.cast Semantics.NounPhrase.this (Semantics.PlayPayment.payingInstead (Semantics.Cost.inCaller arg0)) none none none) ({} : Semantics.DeonticMods))), (Semantics.Ability.static (Semantics.StaticSpec.asEvent (Semantics.GameEvent.does (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Action.pay Semantics.Cost.itsManaCost Semantics.PayTimes.once Semantics.PaymentOutcome.paid (some (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) (some (Semantics.KeywordLabel.named "blitz")))) (Semantics.Action.delay (Semantics.GameEvent.beginningOf Semantics.TurnPart.endStep none (Semantics.TurnRef.next none)) [] none (Semantics.Action.ownScope (some (Semantics.Deed.action "sacrifice")) (Semantics.Action.enact (Semantics.Deed.action "sacrifice") (Semantics.Action.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.becomes (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this))) ((Semantics.Location.zone Semantics.Zone.battlefield none)) (some (Semantics.Location.zone Semantics.Zone.graveyard none)) Semantics.ObservationPoint.departure none []) none none))))), (Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.abilityGrant (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.Ability.keyword (Semantics.KeywordLabel.named "haste") [] [])) (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "blitz")) none) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) Semantics.CondMarking.asLongAs)), (Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.abilityGrant (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Action.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) ((Semantics.Location.zone Semantics.Zone.battlefield none)) (some (Semantics.Location.zone Semantics.Zone.graveyard none)) Semantics.ObservationPoint.departure none [])) [] none [] none none none (Semantics.Action.draw (Semantics.Amount.lit 1)))) (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "blitz")) none) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) Semantics.CondMarking.asLongAs))])
semantic_definition bloodthirst (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "bloodthirst") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.entryRider (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.MoveRider.withCounters (Semantics.Amount.inCaller arg0) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) Semantics.EntryCounterMark.fresh)) (Semantics.Condition.did (Semantics.DidClause.mk (Semantics.Subject.actor (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.isOpponentOf Semantics.NounPhrase.you))) (Semantics.Action.dealDamage Semantics.Amount.anyAmount Semantics.NounPhrase.which Semantics.DamageKind.any) (Semantics.LookbackSpan.duration (Semantics.Duration.between (some (Semantics.GameEvent.beginningOf Semantics.TurnPart.turn none Semantics.TurnRef.current)) (some Semantics.GameEvent.now)) none))) Semantics.CondMarking.asLongAs))])
semantic_definition boast (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "boast") [(Semantics.KeywordParam.ability arg0)] [(Semantics.Ability.qualified arg0 none (some (Semantics.UsageLimit.mk (Semantics.CountBound.moreThan (Semantics.Amount.lit 1)) Semantics.Period.turn)) (some (Semantics.Condition.matches (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none) (Semantics.Predicate.did (Semantics.DidClause.mk (Semantics.Subject.actor Semantics.NounPhrase.which) (Semantics.Action.attack none) (Semantics.LookbackSpan.duration (Semantics.Duration.between (some (Semantics.GameEvent.beginningOf Semantics.TurnPart.turn none Semantics.TurnRef.current)) (some Semantics.GameEvent.now)) none))))) none)])
semantic_definition buyback (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "buyback") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.addedCost (Semantics.Cost.inCaller arg0) true Semantics.PayTimes.once)), (Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.instead (Semantics.WouldClause.would Semantics.Subject.any (Semantics.Action.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) ((Semantics.Location.zone Semantics.Zone.stack none)) (some (Semantics.Location.zone Semantics.Zone.graveyard none)) Semantics.ObservationPoint.departure none [])) (Semantics.Action.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.hand none)) Semantics.ObservationPoint.arrival none []) none Semantics.ReplUse.repeatedly none) (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "buyback")) none) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) Semantics.CondMarking.asLongAs))])
semantic_definition casualty (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "casualty") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.addedCost (Semantics.Cost.perform (Semantics.Action.ownScope (some (Semantics.Deed.action "sacrifice")) (Semantics.Action.enact (Semantics.Deed.action "sacrifice") (Semantics.Action.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.compare [(Semantics.ProjAxis.stat Semantics.Stat.power)] Semantics.Comparator.atLeast (Semantics.Amount.inCaller arg0))]))) ((Semantics.Location.zone Semantics.Zone.battlefield none)) (some (Semantics.Location.zone Semantics.Zone.graveyard none)) Semantics.ObservationPoint.departure none []) none none))) true Semantics.PayTimes.once)), (Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Action.procedure Semantics.Procedure.cast (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) Semantics.PlayPayment.itsOwnCost none none none)) [] none [] none none (some (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "casualty")) none) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) Semantics.Comparator.atLeast (Semantics.Amount.lit 1))) (Semantics.Action.sequentially [(Semantics.Action.copy Semantics.CopySort.fromStack (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) (Semantics.Amount.lit 1) []), (Semantics.Action.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Action.chooseNewTargets (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.copy) Semantics.Plurality.one)) none none)]))])
semantic_definition champion (arg0 : Semantics.Predicate) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "champion") [(Semantics.KeywordParam.subject (Semantics.Predicate.inCaller arg0))] [(Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Action.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.battlefield none)) Semantics.ObservationPoint.arrival none [])) [] none [] none none none (Semantics.Action.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Action.pay (Semantics.Cost.perform (Semantics.Action.ownScope (some (Semantics.Deed.action "exile")) (Semantics.Action.enact (Semantics.Deed.action "exile") (Semantics.Action.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.inCaller arg0), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you), (Semantics.Predicate.otherThan (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this))]))) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.exile none)) Semantics.ObservationPoint.arrival none []) none none))) Semantics.PayTimes.once Semantics.PaymentOutcome.paid none none) none (some (Semantics.Action.ownScope (some (Semantics.Deed.action "sacrifice")) (Semantics.Action.enact (Semantics.Deed.action "sacrifice") (Semantics.Action.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) ((Semantics.Location.zone Semantics.Zone.battlefield none)) (some (Semantics.Location.zone Semantics.Zone.graveyard none)) Semantics.ObservationPoint.departure none []) none none))))), (Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Action.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) ((Semantics.Location.zone Semantics.Zone.battlefield none)) none Semantics.ObservationPoint.departure none [])) [] none [] none none none (Semantics.Action.enact (Semantics.Deed.core Semantics.CoreDeed.return_) (Semantics.Action.move (Semantics.NounPhrase.paid (Semantics.LinkedFacet.objects (Semantics.LinkSource.byDeed (Semantics.Deed.action "exile")) (Semantics.Deed.action "exile") Semantics.Plurality.one) Semantics.NounPhrase.this) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.battlefield none)) Semantics.ObservationPoint.arrival none [((Semantics.MoveRider.under (Semantics.NounPhrase.possessorOf Semantics.PossessorAxis.owner (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.card) Semantics.Plurality.one))))]) none none))])
semantic_definition changeling : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "changeling") [] [(Semantics.Ability.static (Semantics.StaticSpec.characteristicChange (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none) [(Semantics.CharacteristicEdit.everyTypeOf Semantics.QualityOp.adds Semantics.SubtypeSpace.creature)]))])
semantic_definition chooseABackground : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "chooseABackground") [] [])
semantic_definition cipher : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "cipher") [] [(Semantics.Ability.spell none (Semantics.Action.doIf (Semantics.Condition.matches Semantics.NounPhrase.this Semantics.Predicate.isCard) (Semantics.Action.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Action.sequentially [(Semantics.Action.ownScope (some (Semantics.Deed.action "exile")) (Semantics.Action.enact (Semantics.Deed.action "exile") (Semantics.Action.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.exile none)) Semantics.ObservationPoint.arrival none []) none none)), (Semantics.Action.relate Semantics.NounPhrase.this (Semantics.RelationLabel.named "encodedOn") (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)])))]) none none) none)), (Semantics.Ability.static (Semantics.StaticSpec.abilityGrant (Semantics.NounPhrase.counterpart (Semantics.RelationLabel.named "encodedOn") Semantics.NounPhrase.this) (Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) (Semantics.Action.dealDamage Semantics.Amount.anyAmount (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) Semantics.Predicate.anyPlayer) Semantics.DamageKind.combatOnly)) [] none [] none none none (Semantics.Action.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Action.sequentially [(Semantics.Action.copy Semantics.CopySort.fromCardZone (Semantics.NounPhrase.described Semantics.DetPhrase.the (Semantics.Predicate.and [Semantics.Predicate.isCard, (Semantics.Predicate.related (Semantics.RelationLabel.named "encodedOn") (some (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)))])) (Semantics.Amount.lit 1) []), (Semantics.Action.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Action.enact (Semantics.Deed.action "cast") (Semantics.Action.procedure Semantics.Procedure.cast (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.copy) Semantics.Plurality.one) Semantics.PlayPayment.withoutPaying none none none) none none) none none)]) none none))))])
semantic_definition cleave (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "cleave") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.can (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Action.procedure Semantics.Procedure.cast Semantics.NounPhrase.this (Semantics.PlayPayment.payingInstead (Semantics.Cost.inCaller arg0)) none none none) ({} : Semantics.DeonticMods))), (Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.textChange (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) Semantics.TextChange.removeBracketed) (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "cleave")) none) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) Semantics.CondMarking.asLongAs))])
semantic_definition compleated : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "compleated") [] [])
semantic_definition conspire : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "conspire") [] [(Semantics.Ability.static (Semantics.StaticSpec.addedCost (Semantics.Cost.perform (Semantics.Action.ownScope (some (Semantics.Deed.action "tap")) (Semantics.Action.enact (Semantics.Deed.action "tap") (Semantics.Action.setStatus Semantics.Status.tapped (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.described (Semantics.DetPhrase.count (Semantics.Quantity.range (some 2) (some 2)) none) (Semantics.Predicate.and [(Semantics.Predicate.hasStatus Semantics.Status.untapped), (Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you), (Semantics.Predicate.shares Semantics.QualitySort.color Semantics.Agreement.share (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this))])))) none none))) true Semantics.PayTimes.once)), (Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Action.procedure Semantics.Procedure.cast (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) Semantics.PlayPayment.itsOwnCost none none none)) [] none [] none none (some (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "conspire")) none) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) Semantics.Comparator.atLeast (Semantics.Amount.lit 1))) (Semantics.Action.sequentially [(Semantics.Action.copy Semantics.CopySort.fromStack (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) (Semantics.Amount.lit 1) []), (Semantics.Action.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Action.chooseNewTargets (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.copy) Semantics.Plurality.one)) none none)]))])
semantic_definition convoke : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "convoke") [] [(Semantics.Ability.static (Semantics.StaticSpec.paySubstitute (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) ({ action := (Semantics.Action.ownScope (some (Semantics.Deed.action "tap")) (Semantics.Action.enact (Semantics.Deed.action "tap") (Semantics.Action.setStatus Semantics.Status.tapped (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.hasStatus Semantics.Status.untapped), (Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)])))) none none)), pays := [Semantics.PipPaid.generic, (Semantics.PipPaid.ofColor (Semantics.ColorTerm.colorOf (Semantics.NounPhrase.pro (Semantics.Reach.word (Semantics.NounWord.type Semantics.CardType.creature)) Semantics.Plurality.one)))] } : Semantics.ManaSubstitute)))])
semantic_definition craft (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "craft") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [])
semantic_definition crew (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "crew") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0))] [(Semantics.Ability.activated (Semantics.Cost.perform (Semantics.Action.ownScope (some (Semantics.Deed.action "tap")) (Semantics.Action.enact (Semantics.Deed.action "tap") (Semantics.Action.setStatus Semantics.Status.tapped (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.someOf (Semantics.SliceCount.totalling (Semantics.ProjAxis.stat Semantics.Stat.power) Semantics.Comparator.atLeast (Semantics.Amount.inCaller arg0)) none (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.and [(Semantics.Predicate.hasStatus Semantics.Status.untapped), (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you), (Semantics.Predicate.otherThan (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this))])]))))) none none))) (Semantics.Action.establish (Semantics.StaticSpec.characteristicChange (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) [(Semantics.CharacteristicEdit.typeLine Semantics.QualityOp.adds ({ supertypes := none, types := (some [Semantics.CardType.artifact, Semantics.CardType.creature]), subtypes := none, retained := none } : Semantics.TypeLineChanges))]) (some (Semantics.Duration.between none (some (Semantics.GameEvent.endOf Semantics.TurnPart.turn none Semantics.TurnRef.current))))) none none none none)])
semantic_definition dash (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "dash") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.can (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Action.procedure Semantics.Procedure.cast Semantics.NounPhrase.this (Semantics.PlayPayment.payingInstead (Semantics.Cost.inCaller arg0)) none none none) ({} : Semantics.DeonticMods))), (Semantics.Ability.static (Semantics.StaticSpec.asEvent (Semantics.GameEvent.does (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Action.pay Semantics.Cost.itsManaCost Semantics.PayTimes.once Semantics.PaymentOutcome.paid (some (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) (some (Semantics.KeywordLabel.named "dash")))) (Semantics.Action.delay (Semantics.GameEvent.beginningOf Semantics.TurnPart.endStep none (Semantics.TurnRef.next none)) [] none (Semantics.Action.enact (Semantics.Deed.core Semantics.CoreDeed.return_) (Semantics.Action.move (Semantics.NounPhrase.becomes (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.hand none)) Semantics.ObservationPoint.arrival none []) none none)))), (Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.abilityGrant (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.Ability.keyword (Semantics.KeywordLabel.named "haste") [] [])) (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "dash")) none) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) Semantics.CondMarking.asLongAs))])
semantic_definition daybound : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "daybound") [] [(Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.entryRider (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.MoveRider.entersTransformed) (Semantics.Condition.gameIs (Semantics.DesignationLabel.named "night")) Semantics.CondMarking.ifSo)), (Semantics.Ability.static (Semantics.StaticSpec.asEvent (Semantics.GameEvent.gameBecomes (Semantics.DesignationLabel.named "night")) (Semantics.Action.doIf (Semantics.Condition.matches (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.Predicate.currentFace Semantics.CardFaceSide.front)) (Semantics.Action.ownScope (some (Semantics.Deed.action "transform")) (Semantics.Action.enact (Semantics.Deed.action "transform") (Semantics.Action.turnOver (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this))) none none)) none))), (Semantics.Ability.static (Semantics.StaticSpec.cant Semantics.Subject.any (Semantics.Templates.deonticActionFor (Semantics.Deed.action "transform") (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) ({ except := (some (Semantics.DeonticExcept.byKeyword (Semantics.KeywordLabel.named "daybound"))) } : Semantics.DeonticMods)))])
semantic_definition deathtouch : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "deathtouch") [] [])
semantic_definition decayed : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "decayed") [] [(Semantics.Ability.static (Semantics.StaticSpec.cant (Semantics.Subject.actor (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) (Semantics.Templates.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.block) Semantics.NounPhrase.any) ({} : Semantics.DeonticMods))), (Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) (Semantics.Action.attack none)) [] none [] none none none (Semantics.Action.delay (Semantics.GameEvent.beginningOf Semantics.TurnPart.endOfCombat none (Semantics.TurnRef.next none)) [] none (Semantics.Action.ownScope (some (Semantics.Deed.action "sacrifice")) (Semantics.Action.enact (Semantics.Deed.action "sacrifice") (Semantics.Action.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) ((Semantics.Location.zone Semantics.Zone.battlefield none)) (some (Semantics.Location.zone Semantics.Zone.graveyard none)) Semantics.ObservationPoint.departure none []) none none))))])
semantic_definition defender : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "defender") [] [(Semantics.Ability.static (Semantics.StaticSpec.cant (Semantics.Subject.actor (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) (Semantics.Templates.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.attack) Semantics.NounPhrase.any) ({} : Semantics.DeonticMods)))])
semantic_definition delve : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "delve") [] [(Semantics.Ability.static (Semantics.StaticSpec.paySubstitute (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) ({ action := (Semantics.Action.ownScope (some (Semantics.Deed.action "exile")) (Semantics.Action.enact (Semantics.Deed.action "exile") (Semantics.Action.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [Semantics.Predicate.isCard, (Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.graveyard (some Semantics.NounPhrase.you)))]))) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.exile none)) Semantics.ObservationPoint.arrival none []) none none)), pays := [Semantics.PipPaid.generic] } : Semantics.ManaSubstitute)))])
semantic_definition demonstrate : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "demonstrate") [] [(Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Action.procedure Semantics.Procedure.cast (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) Semantics.PlayPayment.itsOwnCost none none none)) [] none [] none none none (Semantics.Action.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Action.sequentially [(Semantics.Action.copy Semantics.CopySort.fromStack (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) (Semantics.Amount.lit 1) []), (Semantics.Action.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Action.chooseNewTargets (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.copy) Semantics.Plurality.one)) none none)]) (some (Semantics.Action.sequentially [(Semantics.Action.choose none (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.isOpponentOf Semantics.NounPhrase.you)) Semantics.Disclosure.openly none), (Semantics.Action.instruct (Semantics.NounPhrase.described Semantics.DetPhrase.the (Semantics.Predicate.chosenPlayer Semantics.ChoiceRef.theChoice)) (Semantics.Action.copy Semantics.CopySort.fromStack (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) (Semantics.Amount.lit 1) [])), (Semantics.Action.instruct (Semantics.NounPhrase.described Semantics.DetPhrase.the (Semantics.Predicate.chosenPlayer Semantics.ChoiceRef.theChoice)) (Semantics.Action.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Action.chooseNewTargets (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.copy) Semantics.Plurality.one)) none none))])) none))])
semantic_definition dethrone : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "dethrone") [] [(Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) (Semantics.Action.attack (some (Semantics.NounPhrase.described Semantics.DetPhrase.the (Semantics.Predicate.and [Semantics.Predicate.anyPlayer, (Semantics.Predicate.superlative Semantics.AggregateOp.max (Semantics.ProjAxis.playerStat Semantics.PlayerStat.lifeTotal) Semantics.Predicate.anyPlayer)]))))) [] none [] none none none (Semantics.Action.putCounters (Semantics.Amount.lit 1) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none) Semantics.CounterBatch.one true))])
semantic_definition devoid : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "devoid") [] [(Semantics.Ability.static (Semantics.StaticSpec.characteristicChange Semantics.NounPhrase.this [(Semantics.CharacteristicEdit.colors Semantics.QualityOp.sets (Semantics.ColorSpec.some []))]))])
semantic_definition devourQuality (arg0 : Predicate) (arg1 : Amount) : Ability :=
  .keyword (.named "devour") [.quality (.inCaller arg0), .number (.inCaller arg1)]
    [.static (.asEvent
      (.does .any (.move thisPermanent .anyLocation (some battlefield) .arrival none []))
      (.withContinuation .optional
        (sacrifice (.described (.count anyNumber none)
          (.and [.inCaller arg0, .in_ battlefield])))
        (some (.establish
          (.entryRider thisPermanent (.withCounters
            (.arith .times (.inCaller arg1)
              (.countOf (theVerbed (.action "sacrifice") .many)))
            (.printed p1p1Counter) .fresh)) none))
        none))]
semantic_definition devour (arg0 : Amount) : Ability :=
  .keyword (.named "devour") [.number (.inCaller arg0)]
    [devourQuality isCreature arg0]
semantic_definition disguise (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "disguise") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.can (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Action.procedure Semantics.Procedure.cast Semantics.NounPhrase.this (Semantics.PlayPayment.faceDown ({ characteristics := (some ({ name := none, cost := none, colors := [], supertypes := [], types := [Semantics.CardType.creature], subtypes := [], text := [(Semantics.Ability.keyword (Semantics.KeywordLabel.named "ward") [(Semantics.KeywordParam.cost (Semantics.Cost.mana [(Semantics.ManaSymbol.simple (Semantics.SimpleManaSymbol.generic 2))]))] [])], power := (some (Semantics.Amount.lit 2)), toughness := (some (Semantics.Amount.lit 2)), loyalty := none, defense := none } : Semantics.Characteristics)) } : Semantics.FaceDown) (Semantics.Cost.mana [(Semantics.ManaSymbol.simple (Semantics.SimpleManaSymbol.generic 3))])) none none none) ({} : Semantics.DeonticMods))), (Semantics.Ability.static (Semantics.StaticSpec.specialAction none (Semantics.Cost.inCaller arg0) (Semantics.Action.setStatus Semantics.Status.faceUp Semantics.NounPhrase.this)))])
semantic_definition disturb (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "disturb") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [])
semantic_definition doctorsCompanion : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "doctorsCompanion") [] [])
semantic_definition doubleStrike : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "doubleStrike") [] [])
semantic_definition dredge (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "dredge") [(Semantics.KeywordParam.number arg0)] [(Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.instead (Semantics.WouldClause.would (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Action.draw Semantics.Amount.anyAmount)) (Semantics.Action.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Action.sequentially [(Semantics.Action.ownScope (some (Semantics.Deed.action "mill")) (Semantics.Action.enact (Semantics.Deed.action "mill") (Semantics.Action.move (Semantics.NounPhrase.slice Semantics.PileEnd.top (Semantics.Amount.inCaller arg0) (Semantics.Location.zone Semantics.Zone.library (some Semantics.NounPhrase.actor))) ((Semantics.Location.zone Semantics.Zone.library none)) (some (Semantics.Location.zone Semantics.Zone.graveyard none)) Semantics.ObservationPoint.arrival none []) none none)), (Semantics.Action.enact (Semantics.Deed.core Semantics.CoreDeed.return_) (Semantics.Action.move Semantics.NounPhrase.this ((Semantics.Location.zone Semantics.Zone.graveyard (some Semantics.NounPhrase.you))) (some (Semantics.Location.zone Semantics.Zone.hand none)) Semantics.ObservationPoint.arrival none []) none none)]) none none) none Semantics.ReplUse.repeatedly none) (Semantics.Condition.compareAmt (Semantics.Amount.countOf (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.and [Semantics.Predicate.isCard, (Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.library (some Semantics.NounPhrase.you)))]))) Semantics.Comparator.atLeast arg0) Semantics.CondMarking.asLongAs))])
semantic_definition echo (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "echo") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [])
semantic_definition embalm (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "embalm") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.activated (Semantics.Cost.compound [(Semantics.Cost.inCaller arg0), (Semantics.Cost.perform (Semantics.Action.enact (Semantics.Deed.action "exile") (Semantics.Action.move Semantics.NounPhrase.this ((Semantics.Location.zone Semantics.Zone.graveyard (some Semantics.NounPhrase.you))) (some (Semantics.Location.zone Semantics.Zone.exile none)) Semantics.ObservationPoint.arrival none []) none none))]) (Semantics.Action.createObject (Semantics.Amount.lit 1) (Semantics.CreationSpec.token (Semantics.TokenSpec.copyOf Semantics.NounPhrase.this [(Semantics.CopyExcept.edits [(Semantics.CharacteristicEdit.colors Semantics.QualityOp.sets (Semantics.ColorSpec.some [Semantics.Color.white])), (Semantics.CharacteristicEdit.manaCost none), (Semantics.CharacteristicEdit.typeLine Semantics.QualityOp.adds ({ supertypes := none, types := none, subtypes := (some [(Semantics.Subtype.of Semantics.CardType.creature "Zombie")]), retained := none } : Semantics.TypeLineChanges))])]) []) true none) (some (Semantics.Timing.and [(Semantics.Timing.inPart Semantics.TurnPart.mainPhase (some Semantics.NounPhrase.you) Semantics.TurnRef.current), (Semantics.Timing.onlyIf (Semantics.Condition.not (Semantics.Condition.exists_ (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.stack none))))))])) none none none)])
semantic_definition emerge (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "emerge") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.can (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Action.procedure Semantics.Procedure.cast Semantics.NounPhrase.this (Semantics.PlayPayment.payingInstead (Semantics.Cost.compound [(Semantics.Cost.inCaller arg0), (Semantics.Cost.perform (Semantics.Action.ownScope (some (Semantics.Deed.action "sacrifice")) (Semantics.Action.enact (Semantics.Deed.action "sacrifice") (Semantics.Action.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.hasType Semantics.CardType.creature))) ((Semantics.Location.zone Semantics.Zone.battlefield none)) (some (Semantics.Location.zone Semantics.Zone.graveyard none)) Semantics.ObservationPoint.departure none []) none none)))])) none none none) ({} : Semantics.DeonticMods))), (Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.costShift (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) (Semantics.CostShift.less (Semantics.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.manaValue) (Semantics.NounPhrase.paid (Semantics.LinkedFacet.objects Semantics.LinkSource.theAlternative (Semantics.Deed.action "sacrifice") Semantics.Plurality.one) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this))) none)) (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "emerge")) none) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) Semantics.CondMarking.asLongAs))])
semantic_definition emergeFrom (arg0 : Semantics.Predicate) (arg1 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "emergeFrom") [(Semantics.KeywordParam.quality (Semantics.Predicate.inCaller arg0)), (Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg1))] [(Semantics.Ability.static (Semantics.StaticSpec.can (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Action.procedure Semantics.Procedure.cast Semantics.NounPhrase.this (Semantics.PlayPayment.payingInstead (Semantics.Cost.compound [(Semantics.Cost.inCaller arg1), (Semantics.Cost.perform (Semantics.Action.ownScope (some (Semantics.Deed.action "sacrifice")) (Semantics.Action.enact (Semantics.Deed.action "sacrifice") (Semantics.Action.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.inCaller arg0), (Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.battlefield none))]))) ((Semantics.Location.zone Semantics.Zone.battlefield none)) (some (Semantics.Location.zone Semantics.Zone.graveyard none)) Semantics.ObservationPoint.departure none []) none none)))])) none none none) ({} : Semantics.DeonticMods))), (Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.costShift (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) (Semantics.CostShift.less (Semantics.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.manaValue) (Semantics.NounPhrase.paid (Semantics.LinkedFacet.objects Semantics.LinkSource.theAlternative (Semantics.Deed.action "sacrifice") Semantics.Plurality.one) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this))) none)) (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "emergeFrom")) none) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) Semantics.CondMarking.asLongAs))])
semantic_definition enchant (arg0 : Semantics.Predicate) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "enchant") [(Semantics.KeywordParam.subject (Semantics.Predicate.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.addsTarget (Semantics.NounPhrase.asType Semantics.CardType.enchantment Semantics.NounPhrase.this (some (Semantics.Subtype.of Semantics.CardType.enchantment "Aura"))) (Semantics.Predicate.inCaller arg0))), (Semantics.Ability.static (Semantics.StaticSpec.can Semantics.Subject.any (Semantics.Action.attachment Semantics.AttachMove.attached (Semantics.NounPhrase.asType Semantics.CardType.enchantment Semantics.NounPhrase.this (some (Semantics.Subtype.of Semantics.CardType.enchantment "Aura"))) (some (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.inCaller arg0)))) ({} : Semantics.DeonticMods))), (Semantics.Ability.static (Semantics.StaticSpec.asEvent (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Action.move (Semantics.NounPhrase.asType Semantics.CardType.enchantment Semantics.NounPhrase.this (some (Semantics.Subtype.of Semantics.CardType.enchantment "Aura"))) (Semantics.Location.not (Semantics.Location.zone Semantics.Zone.stack none)) (some (Semantics.Location.zone Semantics.Zone.battlefield none)) Semantics.ObservationPoint.arrival none [])) (Semantics.Action.ownScope (some (Semantics.Deed.action "attach")) (Semantics.Action.enact (Semantics.Deed.action "attach") (Semantics.Action.attachment Semantics.AttachMove.attached (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asType Semantics.CardType.enchantment Semantics.NounPhrase.this (some (Semantics.Subtype.of Semantics.CardType.enchantment "Aura")))) (some (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.inCaller arg0))))) none none))))])
semantic_definition encore (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "encore") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [])
semantic_definition enlist : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "enlist") [] [])
semantic_definition entwine (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "entwine") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.modeChoice (Semantics.ModeChange.allInstead (some (Semantics.Cost.inCaller arg0)) none)))])
semantic_definition epic : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "epic") [] [(Semantics.Ability.spell none (Semantics.Action.establish (Semantics.StaticSpec.cant (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Templates.deonticActionFor (Semantics.Deed.action "cast") Semantics.NounPhrase.any) ({} : Semantics.DeonticMods)) (some (Semantics.Duration.between none none)))), (Semantics.Ability.spell none (Semantics.Action.delay (Semantics.GameEvent.beginningOf Semantics.TurnPart.upkeep (some Semantics.NounPhrase.you) (Semantics.TurnRef.each none)) [] (some (Semantics.Duration.between none none)) (Semantics.Action.sequentially [(Semantics.Action.copy Semantics.CopySort.fromStack (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) (Semantics.Amount.lit 1) [(Semantics.CopyExcept.edits [(Semantics.CharacteristicEdit.removedAbilities (Semantics.AbilitySelection.specified [(Semantics.AbilityLost.term (Semantics.KeywordTerm.the (Semantics.KeywordLabel.named "epic")))]))])]), (Semantics.Action.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Action.chooseNewTargets (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.copy) Semantics.Plurality.one)) none none)])))])
semantic_definition equip (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "equip") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.activated (Semantics.Cost.inCaller arg0) (Semantics.Action.ownScope (some (Semantics.Deed.action "attach")) (Semantics.Action.enact (Semantics.Deed.action "attach") (Semantics.Action.attachment Semantics.AttachMove.attached (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) (some (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.described (Semantics.DetPhrase.target (Semantics.Quantity.range (some 1) (some 1))) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)]))))) none none)) (some (Semantics.Timing.and [(Semantics.Timing.inPart Semantics.TurnPart.mainPhase (some Semantics.NounPhrase.you) Semantics.TurnRef.current), (Semantics.Timing.onlyIf (Semantics.Condition.not (Semantics.Condition.exists_ (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.stack none))))))])) none none none)])
semantic_definition equipQuality (arg0 : Semantics.Predicate) (arg1 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "equipQuality") [(Semantics.KeywordParam.quality (Semantics.Predicate.inCaller arg0)), (Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg1))] [])
semantic_definition escalate (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "escalate") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.modeChoice (Semantics.ModeChange.extraModesCost (Semantics.Cost.inCaller arg0))))])
semantic_definition escape (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "escape") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.can (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Action.procedure Semantics.Procedure.cast Semantics.NounPhrase.this (Semantics.PlayPayment.payingInstead (Semantics.Cost.inCaller arg0)) none (some (Semantics.Location.zone Semantics.Zone.graveyard (some Semantics.NounPhrase.you))) none) ({} : Semantics.DeonticMods)))])
semantic_definition eternalize (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "eternalize") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.activated (Semantics.Cost.compound [(Semantics.Cost.inCaller arg0), (Semantics.Cost.perform (Semantics.Action.enact (Semantics.Deed.action "exile") (Semantics.Action.move Semantics.NounPhrase.this ((Semantics.Location.zone Semantics.Zone.graveyard (some Semantics.NounPhrase.you))) (some (Semantics.Location.zone Semantics.Zone.exile none)) Semantics.ObservationPoint.arrival none []) none none))]) (Semantics.Action.createObject (Semantics.Amount.lit 1) (Semantics.CreationSpec.token (Semantics.TokenSpec.copyOf Semantics.NounPhrase.this [(Semantics.CopyExcept.edits [(Semantics.CharacteristicEdit.colors Semantics.QualityOp.sets (Semantics.ColorSpec.some [Semantics.Color.black])), (Semantics.CharacteristicEdit.stat Semantics.QualityOp.sets Semantics.CharacteristicStat.power (Semantics.Amount.lit 4)), (Semantics.CharacteristicEdit.stat Semantics.QualityOp.sets Semantics.CharacteristicStat.toughness (Semantics.Amount.lit 4)), (Semantics.CharacteristicEdit.manaCost none), (Semantics.CharacteristicEdit.typeLine Semantics.QualityOp.adds ({ supertypes := none, types := none, subtypes := (some [(Semantics.Subtype.of Semantics.CardType.creature "Zombie")]), retained := none } : Semantics.TypeLineChanges))])]) []) true none) (some (Semantics.Timing.and [(Semantics.Timing.inPart Semantics.TurnPart.mainPhase (some Semantics.NounPhrase.you) Semantics.TurnRef.current), (Semantics.Timing.onlyIf (Semantics.Condition.not (Semantics.Condition.exists_ (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.stack none))))))])) none none none)])
semantic_definition evoke (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "evoke") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.can (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Action.procedure Semantics.Procedure.cast Semantics.NounPhrase.this (Semantics.PlayPayment.payingInstead (Semantics.Cost.inCaller arg0)) none none none) ({} : Semantics.DeonticMods))), (Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Action.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.battlefield none)) Semantics.ObservationPoint.arrival none [])) [] none [] none none (some (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "evoke")) none) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) Semantics.Comparator.atLeast (Semantics.Amount.lit 1))) (Semantics.Action.instruct (Semantics.NounPhrase.possessorOf Semantics.PossessorAxis.controller (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) (Semantics.Action.ownScope (some (Semantics.Deed.action "sacrifice")) (Semantics.Action.enact (Semantics.Deed.action "sacrifice") (Semantics.Action.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) ((Semantics.Location.zone Semantics.Zone.battlefield none)) (some (Semantics.Location.zone Semantics.Zone.graveyard none)) Semantics.ObservationPoint.departure none []) none none))))])
semantic_definition evolve : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "evolve") [] [(Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Action.move (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)])) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.battlefield none)) Semantics.ObservationPoint.arrival none [])) [] none [] none none (some (Semantics.Condition.or [(Semantics.Condition.compareAmt (Semantics.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.power) (Semantics.NounPhrase.pro (Semantics.Reach.word (Semantics.NounWord.type Semantics.CardType.creature)) Semantics.Plurality.one)) Semantics.Comparator.greater (Semantics.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.power) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none))), (Semantics.Condition.compareAmt (Semantics.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.toughness) (Semantics.NounPhrase.pro (Semantics.Reach.word (Semantics.NounWord.type Semantics.CardType.creature)) Semantics.Plurality.one)) Semantics.Comparator.greater (Semantics.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.toughness) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)))])) (Semantics.Action.putCounters (Semantics.Amount.lit 1) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none) Semantics.CounterBatch.one true))])
semantic_definition exalted : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "exalted") [] [(Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)]))) (Semantics.Action.attack none)) [] none [] none none (some (Semantics.Condition.compareAmt (Semantics.Amount.countOf (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.inCombat Semantics.CombatRelation.attackerOf none)]))) Semantics.Comparator.eq (Semantics.Amount.lit 1))) (Semantics.Action.establish (Semantics.StaticSpec.ptModification (Semantics.NounPhrase.pro (Semantics.Reach.word (Semantics.NounWord.type Semantics.CardType.creature)) Semantics.Plurality.one) (Semantics.Delta.up (Semantics.Amount.lit 1)) (Semantics.Delta.up (Semantics.Amount.lit 1))) (some (Semantics.Duration.between none (some (Semantics.GameEvent.endOf Semantics.TurnPart.turn none Semantics.TurnRef.current))))))])
semantic_definition exhaust (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "exhaust") [(Semantics.KeywordParam.ability arg0)] [(Semantics.Ability.qualified arg0 none (some (Semantics.UsageLimit.mk (Semantics.CountBound.moreThan (Semantics.Amount.lit 1)) Semantics.Period.thisObject)) none none)])
semantic_definition exploit : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "exploit") [] [(Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Action.move (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.battlefield none)) Semantics.ObservationPoint.arrival none [])) [] none [] none none none (Semantics.Action.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Action.ownScope (some (Semantics.Deed.action "sacrifice")) (Semantics.Action.enact (Semantics.Deed.action "sacrifice") (Semantics.Action.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.hasType Semantics.CardType.creature))) ((Semantics.Location.zone Semantics.Zone.battlefield none)) (some (Semantics.Location.zone Semantics.Zone.graveyard none)) Semantics.ObservationPoint.departure none []) none none)) none none))])
semantic_definition extort : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "extort") [] [(Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Action.procedure Semantics.Procedure.cast (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.stack none)), (Semantics.Predicate.not (Semantics.Predicate.abilityHead Semantics.AbilityClass.anyOnStack))])) Semantics.PlayPayment.itsOwnCost none none none)) [] none [] none none none (Semantics.Action.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Action.pay (Semantics.Cost.mana [(Semantics.ManaSymbol.hybrid (Semantics.SimpleManaSymbol.specific (Semantics.ColorOrColorless.of Semantics.Color.white)) Semantics.Color.black)]) Semantics.PayTimes.once Semantics.PaymentOutcome.paid none none) (some (Semantics.Action.sequentially [(Semantics.Action.instruct (Semantics.NounPhrase.described Semantics.DetPhrase.each (Semantics.Predicate.isOpponentOf Semantics.NounPhrase.you)) (Semantics.Action.changeLife (Semantics.Delta.down (Semantics.Amount.lit 1)))), (Semantics.Action.changeLife (Semantics.Delta.up Semantics.Amount.thatMuch))])) none))])
semantic_definition fabricate (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "fabricate") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0))] [(Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Action.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.battlefield none)) Semantics.ObservationPoint.arrival none [])) [] none [] none none none (Semantics.Action.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Action.putCounters (Semantics.Amount.inCaller arg0) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.CounterBatch.one true) none (some (Semantics.Action.createObject (Semantics.Amount.inCaller arg0) (Semantics.CreationSpec.token (Semantics.TokenSpec.written ({ characteristics := ({ name := none, cost := none, colors := [], supertypes := [], types := [Semantics.CardType.artifact, Semantics.CardType.creature], subtypes := [(Semantics.Subtype.of Semantics.CardType.creature "Servo")], text := [], power := (some (Semantics.Amount.lit 1)), toughness := (some (Semantics.Amount.lit 1)), loyalty := none, defense := none } : Semantics.Characteristics), qualities := [] } : Semantics.CharacteristicBundle)) []) true none))))])
semantic_definition fading (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "fading") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.entryRider (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.MoveRider.withCounters (Semantics.Amount.inCaller arg0) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "fadeCounter")) Semantics.EntryCounterMark.fresh))), (Semantics.Ability.triggered (Semantics.GameEvent.beginningOf Semantics.TurnPart.upkeep (some Semantics.NounPhrase.you) (Semantics.TurnRef.each none)) [] none [] none none none (Semantics.Action.withContinuation Semantics.ContinuationPolicy.required (Semantics.Action.removeCounters (some (Semantics.Quantity.range (some 1) (some 1))) (some (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "fadeCounter"))) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.CounterBatch.one true) none (some (Semantics.Action.ownScope (some (Semantics.Deed.action "sacrifice")) (Semantics.Action.enact (Semantics.Deed.action "sacrifice") (Semantics.Action.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) ((Semantics.Location.zone Semantics.Zone.battlefield none)) (some (Semantics.Location.zone Semantics.Zone.graveyard none)) Semantics.ObservationPoint.departure none []) none none)))))])
semantic_definition fear : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "fear") [] [(Semantics.Ability.static (Semantics.StaticSpec.cant Semantics.Subject.any (Semantics.Templates.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.block) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) ({ except := (some (Semantics.DeonticExcept.byObjects (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.or [(Semantics.Predicate.hasType Semantics.CardType.artifact), (Semantics.Predicate.colorIs Semantics.Color.black)])]))) } : Semantics.DeonticMods)))])
semantic_definition firebending (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "firebending") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0))] [(Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) (Semantics.Action.attack none)) [] none [] none none none (Semantics.Action.sequentially [(Semantics.Action.addMana (Semantics.Amount.inCaller arg0) (Semantics.ProducedMana.runs [[(Semantics.ColorOrColorless.of Semantics.Color.red)]]) []), (Semantics.Action.establish (Semantics.StaticSpec.manaRetention Semantics.NounPhrase.you Semantics.ManaHeld.thisMana) (some (Semantics.Duration.between none (some (Semantics.GameEvent.endOf Semantics.TurnPart.combat none Semantics.TurnRef.current)))))]))])
semantic_definition firstStrike : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "firstStrike") [] [])
semantic_definition flanking : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "flanking") [] [(Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.by (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.not (Semantics.Predicate.hasKeyword (Semantics.KeywordTerm.the (Semantics.KeywordLabel.named "flanking"))))]))) (Semantics.Action.block (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none))) [] none [] none none none (Semantics.Action.establish (Semantics.StaticSpec.ptModification (Semantics.NounPhrase.pro (Semantics.Reach.word (Semantics.NounWord.type Semantics.CardType.creature)) Semantics.Plurality.one) (Semantics.Delta.down (Semantics.Amount.lit 1)) (Semantics.Delta.down (Semantics.Amount.lit 1))) (some (Semantics.Duration.between none (some (Semantics.GameEvent.endOf Semantics.TurnPart.turn none Semantics.TurnRef.current))))))])
semantic_definition flash : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "flash") [] [(Semantics.Ability.static (Semantics.StaticSpec.can (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Templates.deonticActionFor (Semantics.Deed.action "play") Semantics.NounPhrase.this) ({ timing := (some (Semantics.Timing.onlyIf (Semantics.Condition.hasPriority Semantics.NounPhrase.you))) } : Semantics.DeonticMods)))])
semantic_definition flashback (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "flashback") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.can (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Action.procedure Semantics.Procedure.cast Semantics.NounPhrase.this (Semantics.PlayPayment.payingInstead (Semantics.Cost.inCaller arg0)) none (some (Semantics.Location.zone Semantics.Zone.graveyard (some Semantics.NounPhrase.you))) none) ({} : Semantics.DeonticMods))), (Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.instead (Semantics.WouldClause.would Semantics.Subject.any (Semantics.Action.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) ((Semantics.Location.zone Semantics.Zone.stack none)) none Semantics.ObservationPoint.departure none [])) (Semantics.Action.ownScope (some (Semantics.Deed.action "exile")) (Semantics.Action.enact (Semantics.Deed.action "exile") (Semantics.Action.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.exile none)) Semantics.ObservationPoint.arrival none []) none none)) none Semantics.ReplUse.repeatedly none) (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "flashback")) none) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) Semantics.CondMarking.asLongAs))])
semantic_definition forMirrodin : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "forMirrodin") [] [(Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Action.move (Semantics.NounPhrase.asType Semantics.CardType.artifact Semantics.NounPhrase.this (some (Semantics.Subtype.of Semantics.CardType.artifact "Equipment"))) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.battlefield none)) Semantics.ObservationPoint.arrival none [])) [] none [] none none none (Semantics.Action.sequentially [(Semantics.Action.createObject (Semantics.Amount.lit 1) (Semantics.CreationSpec.token (Semantics.TokenSpec.written ({ characteristics := ({ name := none, cost := none, colors := [Semantics.Color.red], supertypes := [], types := [Semantics.CardType.creature], subtypes := [(Semantics.Subtype.of Semantics.CardType.creature "Rebel")], text := [], power := (some (Semantics.Amount.lit 2)), toughness := (some (Semantics.Amount.lit 2)), loyalty := none, defense := none } : Semantics.Characteristics), qualities := [] } : Semantics.CharacteristicBundle)) []) true none), (Semantics.Action.ownScope (some (Semantics.Deed.action "attach")) (Semantics.Action.enact (Semantics.Deed.action "attach") (Semantics.Action.attachment Semantics.AttachMove.attached (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asType Semantics.CardType.artifact Semantics.NounPhrase.this (some (Semantics.Subtype.of Semantics.CardType.artifact "Equipment")))) (some (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.pro Semantics.Reach.tokenBorn Semantics.Plurality.one)))) none none))]))])
semantic_definition forecast (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "forecast") [(Semantics.KeywordParam.ability arg0)] [(Semantics.Ability.qualified arg0 (some (Semantics.Timing.inPart Semantics.TurnPart.upkeep (some (Semantics.NounPhrase.possessorOf Semantics.PossessorAxis.owner Semantics.NounPhrase.this)) Semantics.TurnRef.current)) (some (Semantics.UsageLimit.mk (Semantics.CountBound.moreThan (Semantics.Amount.lit 1)) Semantics.Period.turn)) none (some Semantics.Zone.hand))])
semantic_definition foretell (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "foretell") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.specialAction (some (Semantics.Timing.inPart Semantics.TurnPart.turn (some Semantics.NounPhrase.you) Semantics.TurnRef.current)) (Semantics.Cost.mana [(Semantics.ManaSymbol.simple (Semantics.SimpleManaSymbol.generic 2))]) (Semantics.Action.sequentially [(Semantics.Action.enact (Semantics.Deed.action "exile") (Semantics.Action.move Semantics.NounPhrase.this Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.exile none)) Semantics.ObservationPoint.arrival none [((Semantics.MoveRider.faceDown ({ characteristics := none } : Semantics.FaceDown)))]) none none), (Semantics.Action.establish (Semantics.StaticSpec.visibility Semantics.ExposeVerb.lookAt Semantics.NounPhrase.you (Semantics.VisibleThing.objects (Semantics.NounPhrase.pro Semantics.Reach.bare Semantics.Plurality.one))) (some (Semantics.Duration.forAsLongAs (Semantics.Condition.matches (Semantics.NounPhrase.pro Semantics.Reach.bare Semantics.Plurality.one) (Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.exile none)))))), (Semantics.Action.establish (Semantics.StaticSpec.can (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Action.procedure Semantics.Procedure.cast (Semantics.NounPhrase.pro Semantics.Reach.bare Semantics.Plurality.one) (Semantics.PlayPayment.payingInstead (Semantics.Cost.inCaller arg0)) none (some (Semantics.Location.zone Semantics.Zone.exile none)) none) ({} : Semantics.DeonticMods)) (some (Semantics.Duration.between (some (Semantics.GameEvent.endOf Semantics.TurnPart.turn none Semantics.TurnRef.current)) none)))])))])
semantic_definition fortify (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "fortify") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.activated (Semantics.Cost.inCaller arg0) (Semantics.Action.ownScope (some (Semantics.Deed.action "attach")) (Semantics.Action.enact (Semantics.Deed.action "attach") (Semantics.Action.attachment Semantics.AttachMove.attached (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) (some (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.described (Semantics.DetPhrase.target (Semantics.Quantity.range (some 1) (some 1))) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.land), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)]))))) none none)) (some (Semantics.Timing.and [(Semantics.Timing.inPart Semantics.TurnPart.mainPhase (some Semantics.NounPhrase.you) Semantics.TurnRef.current), (Semantics.Timing.onlyIf (Semantics.Condition.not (Semantics.Condition.exists_ (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.stack none))))))])) none none none)])
semantic_definition freerunning (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "freerunning") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [])
semantic_definition frenzy (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "frenzy") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0))] [])
semantic_definition fuse : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "fuse") [] [])
semantic_definition gift (arg0 : Semantics.Action) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "gift") [] [(Semantics.Ability.static (Semantics.StaticSpec.addedCost (Semantics.Cost.perform (Semantics.Action.choose none (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.isOpponentOf Semantics.NounPhrase.you)) Semantics.Disclosure.openly none)) true Semantics.PayTimes.once)), (Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Action.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.battlefield none)) Semantics.ObservationPoint.arrival none [])) [] none [] none none (some (Semantics.Condition.matches Semantics.NounPhrase.this (Semantics.Predicate.or [(Semantics.Predicate.hasType Semantics.CardType.artifact), (Semantics.Predicate.hasType Semantics.CardType.battle), (Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasType Semantics.CardType.enchantment), (Semantics.Predicate.hasType Semantics.CardType.land), (Semantics.Predicate.hasType Semantics.CardType.planeswalker)]))) (Semantics.Action.doIf (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift")) none) Semantics.NounPhrase.this) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) (Semantics.Action.instruct (Semantics.NounPhrase.paid (Semantics.LinkedFacet.chosenPlayer (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift"))) Semantics.NounPhrase.this) (Semantics.Action.inCaller arg0)) none)), (Semantics.Ability.spell none (Semantics.Action.doIf (Semantics.Condition.matches Semantics.NounPhrase.this (Semantics.Predicate.or [(Semantics.Predicate.hasType Semantics.CardType.instant), (Semantics.Predicate.hasType Semantics.CardType.sorcery)])) (Semantics.Action.doIf (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift")) none) Semantics.NounPhrase.this) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) (Semantics.Action.instruct (Semantics.NounPhrase.paid (Semantics.LinkedFacet.chosenPlayer (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift"))) Semantics.NounPhrase.this) (Semantics.Action.inCaller arg0)) none) none))])
semantic_definition giftACard : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "gift") [] [(Semantics.Ability.static (Semantics.StaticSpec.addedCost (Semantics.Cost.perform (Semantics.Action.choose none (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.isOpponentOf Semantics.NounPhrase.you)) Semantics.Disclosure.openly none)) true Semantics.PayTimes.once)), (Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Action.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.battlefield none)) Semantics.ObservationPoint.arrival none [])) [] none [] none none (some (Semantics.Condition.matches Semantics.NounPhrase.this (Semantics.Predicate.or [(Semantics.Predicate.hasType Semantics.CardType.artifact), (Semantics.Predicate.hasType Semantics.CardType.battle), (Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasType Semantics.CardType.enchantment), (Semantics.Predicate.hasType Semantics.CardType.land), (Semantics.Predicate.hasType Semantics.CardType.planeswalker)]))) (Semantics.Action.doIf (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift")) none) Semantics.NounPhrase.this) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) (Semantics.Action.instruct (Semantics.NounPhrase.paid (Semantics.LinkedFacet.chosenPlayer (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift"))) Semantics.NounPhrase.this) (Semantics.Action.draw (Semantics.Amount.lit 1))) none)), (Semantics.Ability.spell none (Semantics.Action.doIf (Semantics.Condition.matches Semantics.NounPhrase.this (Semantics.Predicate.or [(Semantics.Predicate.hasType Semantics.CardType.instant), (Semantics.Predicate.hasType Semantics.CardType.sorcery)])) (Semantics.Action.doIf (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift")) none) Semantics.NounPhrase.this) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) (Semantics.Action.instruct (Semantics.NounPhrase.paid (Semantics.LinkedFacet.chosenPlayer (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift"))) Semantics.NounPhrase.this) (Semantics.Action.draw (Semantics.Amount.lit 1))) none) none))])
semantic_definition giftAFood : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "gift") [] [(Semantics.Ability.static (Semantics.StaticSpec.addedCost (Semantics.Cost.perform (Semantics.Action.choose none (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.isOpponentOf Semantics.NounPhrase.you)) Semantics.Disclosure.openly none)) true Semantics.PayTimes.once)), (Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Action.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.battlefield none)) Semantics.ObservationPoint.arrival none [])) [] none [] none none (some (Semantics.Condition.matches Semantics.NounPhrase.this (Semantics.Predicate.or [(Semantics.Predicate.hasType Semantics.CardType.artifact), (Semantics.Predicate.hasType Semantics.CardType.battle), (Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasType Semantics.CardType.enchantment), (Semantics.Predicate.hasType Semantics.CardType.land), (Semantics.Predicate.hasType Semantics.CardType.planeswalker)]))) (Semantics.Action.doIf (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift")) none) Semantics.NounPhrase.this) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) (Semantics.Action.instruct (Semantics.NounPhrase.paid (Semantics.LinkedFacet.chosenPlayer (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift"))) Semantics.NounPhrase.this) (Semantics.Action.createObject (Semantics.Amount.lit 1) (Semantics.CreationSpec.token (Semantics.TokenSpec.written ({ characteristics := ({ name := none, cost := none, colors := [], supertypes := [], types := [Semantics.CardType.artifact], subtypes := [(Semantics.Subtype.of Semantics.CardType.artifact "Food")], text := [(Semantics.Ability.activated (Semantics.Cost.compound [(Semantics.Cost.mana [(Semantics.ManaSymbol.simple (Semantics.SimpleManaSymbol.generic 2))]), Semantics.Cost.tapSymbol, (Semantics.Cost.perform (Semantics.Action.ownScope (some (Semantics.Deed.action "sacrifice")) (Semantics.Action.enact (Semantics.Deed.action "sacrifice") (Semantics.Action.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asType Semantics.CardType.artifact Semantics.NounPhrase.this none)) ((Semantics.Location.zone Semantics.Zone.battlefield none)) (some (Semantics.Location.zone Semantics.Zone.graveyard none)) Semantics.ObservationPoint.departure none []) none none)))]) (Semantics.Action.changeLife (Semantics.Delta.up (Semantics.Amount.lit 3))) none none none none)], power := none, toughness := none, loyalty := none, defense := none } : Semantics.Characteristics), qualities := [] } : Semantics.CharacteristicBundle)) []) true none)) none)), (Semantics.Ability.spell none (Semantics.Action.doIf (Semantics.Condition.matches Semantics.NounPhrase.this (Semantics.Predicate.or [(Semantics.Predicate.hasType Semantics.CardType.instant), (Semantics.Predicate.hasType Semantics.CardType.sorcery)])) (Semantics.Action.doIf (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift")) none) Semantics.NounPhrase.this) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) (Semantics.Action.instruct (Semantics.NounPhrase.paid (Semantics.LinkedFacet.chosenPlayer (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift"))) Semantics.NounPhrase.this) (Semantics.Action.createObject (Semantics.Amount.lit 1) (Semantics.CreationSpec.token (Semantics.TokenSpec.written ({ characteristics := ({ name := none, cost := none, colors := [], supertypes := [], types := [Semantics.CardType.artifact], subtypes := [(Semantics.Subtype.of Semantics.CardType.artifact "Food")], text := [(Semantics.Ability.activated (Semantics.Cost.compound [(Semantics.Cost.mana [(Semantics.ManaSymbol.simple (Semantics.SimpleManaSymbol.generic 2))]), Semantics.Cost.tapSymbol, (Semantics.Cost.perform (Semantics.Action.ownScope (some (Semantics.Deed.action "sacrifice")) (Semantics.Action.enact (Semantics.Deed.action "sacrifice") (Semantics.Action.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asType Semantics.CardType.artifact Semantics.NounPhrase.this none)) ((Semantics.Location.zone Semantics.Zone.battlefield none)) (some (Semantics.Location.zone Semantics.Zone.graveyard none)) Semantics.ObservationPoint.departure none []) none none)))]) (Semantics.Action.changeLife (Semantics.Delta.up (Semantics.Amount.lit 3))) none none none none)], power := none, toughness := none, loyalty := none, defense := none } : Semantics.Characteristics), qualities := [] } : Semantics.CharacteristicBundle)) []) true none)) none) none))])
semantic_definition giftATappedFish : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "gift") [] [(Semantics.Ability.static (Semantics.StaticSpec.addedCost (Semantics.Cost.perform (Semantics.Action.choose none (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.isOpponentOf Semantics.NounPhrase.you)) Semantics.Disclosure.openly none)) true Semantics.PayTimes.once)), (Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Action.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.battlefield none)) Semantics.ObservationPoint.arrival none [])) [] none [] none none (some (Semantics.Condition.matches Semantics.NounPhrase.this (Semantics.Predicate.or [(Semantics.Predicate.hasType Semantics.CardType.artifact), (Semantics.Predicate.hasType Semantics.CardType.battle), (Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasType Semantics.CardType.enchantment), (Semantics.Predicate.hasType Semantics.CardType.land), (Semantics.Predicate.hasType Semantics.CardType.planeswalker)]))) (Semantics.Action.doIf (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift")) none) Semantics.NounPhrase.this) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) (Semantics.Action.instruct (Semantics.NounPhrase.paid (Semantics.LinkedFacet.chosenPlayer (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift"))) Semantics.NounPhrase.this) (Semantics.Action.createObject (Semantics.Amount.lit 1) (Semantics.CreationSpec.token (Semantics.TokenSpec.written ({ characteristics := ({ name := none, cost := none, colors := [Semantics.Color.blue], supertypes := [], types := [Semantics.CardType.creature], subtypes := [(Semantics.Subtype.of Semantics.CardType.creature "Fish")], text := [], power := (some (Semantics.Amount.lit 1)), toughness := (some (Semantics.Amount.lit 1)), loyalty := none, defense := none } : Semantics.Characteristics), qualities := [] } : Semantics.CharacteristicBundle)) [(Semantics.MoveRider.entersAs Semantics.Status.tapped)]) true none)) none)), (Semantics.Ability.spell none (Semantics.Action.doIf (Semantics.Condition.matches Semantics.NounPhrase.this (Semantics.Predicate.or [(Semantics.Predicate.hasType Semantics.CardType.instant), (Semantics.Predicate.hasType Semantics.CardType.sorcery)])) (Semantics.Action.doIf (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift")) none) Semantics.NounPhrase.this) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) (Semantics.Action.instruct (Semantics.NounPhrase.paid (Semantics.LinkedFacet.chosenPlayer (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift"))) Semantics.NounPhrase.this) (Semantics.Action.createObject (Semantics.Amount.lit 1) (Semantics.CreationSpec.token (Semantics.TokenSpec.written ({ characteristics := ({ name := none, cost := none, colors := [Semantics.Color.blue], supertypes := [], types := [Semantics.CardType.creature], subtypes := [(Semantics.Subtype.of Semantics.CardType.creature "Fish")], text := [], power := (some (Semantics.Amount.lit 1)), toughness := (some (Semantics.Amount.lit 1)), loyalty := none, defense := none } : Semantics.Characteristics), qualities := [] } : Semantics.CharacteristicBundle)) [(Semantics.MoveRider.entersAs Semantics.Status.tapped)]) true none)) none) none))])
semantic_definition giftATreasure : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "gift") [] [(Semantics.Ability.static (Semantics.StaticSpec.addedCost (Semantics.Cost.perform (Semantics.Action.choose none (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.isOpponentOf Semantics.NounPhrase.you)) Semantics.Disclosure.openly none)) true Semantics.PayTimes.once)), (Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Action.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.battlefield none)) Semantics.ObservationPoint.arrival none [])) [] none [] none none (some (Semantics.Condition.matches Semantics.NounPhrase.this (Semantics.Predicate.or [(Semantics.Predicate.hasType Semantics.CardType.artifact), (Semantics.Predicate.hasType Semantics.CardType.battle), (Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasType Semantics.CardType.enchantment), (Semantics.Predicate.hasType Semantics.CardType.land), (Semantics.Predicate.hasType Semantics.CardType.planeswalker)]))) (Semantics.Action.doIf (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift")) none) Semantics.NounPhrase.this) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) (Semantics.Action.instruct (Semantics.NounPhrase.paid (Semantics.LinkedFacet.chosenPlayer (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift"))) Semantics.NounPhrase.this) (Semantics.Action.createObject (Semantics.Amount.lit 1) (Semantics.CreationSpec.token (Semantics.TokenSpec.written ({ characteristics := ({ name := none, cost := none, colors := [], supertypes := [], types := [Semantics.CardType.artifact], subtypes := [(Semantics.Subtype.of Semantics.CardType.artifact "Treasure")], text := [(Semantics.Ability.activated (Semantics.Cost.compound [Semantics.Cost.tapSymbol, (Semantics.Cost.perform (Semantics.Action.ownScope (some (Semantics.Deed.action "sacrifice")) (Semantics.Action.enact (Semantics.Deed.action "sacrifice") (Semantics.Action.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asType Semantics.CardType.artifact Semantics.NounPhrase.this none)) ((Semantics.Location.zone Semantics.Zone.battlefield none)) (some (Semantics.Location.zone Semantics.Zone.graveyard none)) Semantics.ObservationPoint.departure none []) none none)))]) (Semantics.Action.addMana (Semantics.Amount.lit 1) (Semantics.ProducedMana.anyColor Semantics.ColorFreedom.sameColor) []) none none none none)], power := none, toughness := none, loyalty := none, defense := none } : Semantics.Characteristics), qualities := [] } : Semantics.CharacteristicBundle)) []) true none)) none)), (Semantics.Ability.spell none (Semantics.Action.doIf (Semantics.Condition.matches Semantics.NounPhrase.this (Semantics.Predicate.or [(Semantics.Predicate.hasType Semantics.CardType.instant), (Semantics.Predicate.hasType Semantics.CardType.sorcery)])) (Semantics.Action.doIf (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift")) none) Semantics.NounPhrase.this) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) (Semantics.Action.instruct (Semantics.NounPhrase.paid (Semantics.LinkedFacet.chosenPlayer (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift"))) Semantics.NounPhrase.this) (Semantics.Action.createObject (Semantics.Amount.lit 1) (Semantics.CreationSpec.token (Semantics.TokenSpec.written ({ characteristics := ({ name := none, cost := none, colors := [], supertypes := [], types := [Semantics.CardType.artifact], subtypes := [(Semantics.Subtype.of Semantics.CardType.artifact "Treasure")], text := [(Semantics.Ability.activated (Semantics.Cost.compound [Semantics.Cost.tapSymbol, (Semantics.Cost.perform (Semantics.Action.ownScope (some (Semantics.Deed.action "sacrifice")) (Semantics.Action.enact (Semantics.Deed.action "sacrifice") (Semantics.Action.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asType Semantics.CardType.artifact Semantics.NounPhrase.this none)) ((Semantics.Location.zone Semantics.Zone.battlefield none)) (some (Semantics.Location.zone Semantics.Zone.graveyard none)) Semantics.ObservationPoint.departure none []) none none)))]) (Semantics.Action.addMana (Semantics.Amount.lit 1) (Semantics.ProducedMana.anyColor Semantics.ColorFreedom.sameColor) []) none none none none)], power := none, toughness := none, loyalty := none, defense := none } : Semantics.Characteristics), qualities := [] } : Semantics.CharacteristicBundle)) []) true none)) none) none))])
semantic_definition giftAnExtraTurn : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "gift") [] [(Semantics.Ability.static (Semantics.StaticSpec.addedCost (Semantics.Cost.perform (Semantics.Action.choose none (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.isOpponentOf Semantics.NounPhrase.you)) Semantics.Disclosure.openly none)) true Semantics.PayTimes.once)), (Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Action.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.battlefield none)) Semantics.ObservationPoint.arrival none [])) [] none [] none none (some (Semantics.Condition.matches Semantics.NounPhrase.this (Semantics.Predicate.or [(Semantics.Predicate.hasType Semantics.CardType.artifact), (Semantics.Predicate.hasType Semantics.CardType.battle), (Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasType Semantics.CardType.enchantment), (Semantics.Predicate.hasType Semantics.CardType.land), (Semantics.Predicate.hasType Semantics.CardType.planeswalker)]))) (Semantics.Action.doIf (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift")) none) Semantics.NounPhrase.this) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) (Semantics.Action.instruct (Semantics.NounPhrase.paid (Semantics.LinkedFacet.chosenPlayer (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift"))) Semantics.NounPhrase.this) (Semantics.Action.addTurnPart Semantics.TurnPart.turn none (Semantics.Amount.lit 1) none)) none)), (Semantics.Ability.spell none (Semantics.Action.doIf (Semantics.Condition.matches Semantics.NounPhrase.this (Semantics.Predicate.or [(Semantics.Predicate.hasType Semantics.CardType.instant), (Semantics.Predicate.hasType Semantics.CardType.sorcery)])) (Semantics.Action.doIf (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift")) none) Semantics.NounPhrase.this) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) (Semantics.Action.instruct (Semantics.NounPhrase.paid (Semantics.LinkedFacet.chosenPlayer (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift"))) Semantics.NounPhrase.this) (Semantics.Action.addTurnPart Semantics.TurnPart.turn none (Semantics.Amount.lit 1) none)) none) none))])
semantic_definition giftAnOctopus : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "gift") [] [(Semantics.Ability.static (Semantics.StaticSpec.addedCost (Semantics.Cost.perform (Semantics.Action.choose none (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.isOpponentOf Semantics.NounPhrase.you)) Semantics.Disclosure.openly none)) true Semantics.PayTimes.once)), (Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Action.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.battlefield none)) Semantics.ObservationPoint.arrival none [])) [] none [] none none (some (Semantics.Condition.matches Semantics.NounPhrase.this (Semantics.Predicate.or [(Semantics.Predicate.hasType Semantics.CardType.artifact), (Semantics.Predicate.hasType Semantics.CardType.battle), (Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasType Semantics.CardType.enchantment), (Semantics.Predicate.hasType Semantics.CardType.land), (Semantics.Predicate.hasType Semantics.CardType.planeswalker)]))) (Semantics.Action.doIf (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift")) none) Semantics.NounPhrase.this) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) (Semantics.Action.instruct (Semantics.NounPhrase.paid (Semantics.LinkedFacet.chosenPlayer (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift"))) Semantics.NounPhrase.this) (Semantics.Action.createObject (Semantics.Amount.lit 1) (Semantics.CreationSpec.token (Semantics.TokenSpec.written ({ characteristics := ({ name := none, cost := none, colors := [Semantics.Color.blue], supertypes := [], types := [Semantics.CardType.creature], subtypes := [(Semantics.Subtype.of Semantics.CardType.creature "Octopus")], text := [], power := (some (Semantics.Amount.lit 8)), toughness := (some (Semantics.Amount.lit 8)), loyalty := none, defense := none } : Semantics.Characteristics), qualities := [] } : Semantics.CharacteristicBundle)) []) true none)) none)), (Semantics.Ability.spell none (Semantics.Action.doIf (Semantics.Condition.matches Semantics.NounPhrase.this (Semantics.Predicate.or [(Semantics.Predicate.hasType Semantics.CardType.instant), (Semantics.Predicate.hasType Semantics.CardType.sorcery)])) (Semantics.Action.doIf (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift")) none) Semantics.NounPhrase.this) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) (Semantics.Action.instruct (Semantics.NounPhrase.paid (Semantics.LinkedFacet.chosenPlayer (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift"))) Semantics.NounPhrase.this) (Semantics.Action.createObject (Semantics.Amount.lit 1) (Semantics.CreationSpec.token (Semantics.TokenSpec.written ({ characteristics := ({ name := none, cost := none, colors := [Semantics.Color.blue], supertypes := [], types := [Semantics.CardType.creature], subtypes := [(Semantics.Subtype.of Semantics.CardType.creature "Octopus")], text := [], power := (some (Semantics.Amount.lit 8)), toughness := (some (Semantics.Amount.lit 8)), loyalty := none, defense := none } : Semantics.Characteristics), qualities := [] } : Semantics.CharacteristicBundle)) []) true none)) none) none))])
semantic_definition graft (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "graft") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.entryRider (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.MoveRider.withCounters (Semantics.Amount.inCaller arg0) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) Semantics.EntryCounterMark.fresh))), (Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Action.move (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.otherThan (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this))])) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.battlefield none)) Semantics.ObservationPoint.arrival none [])) [] none [] none none (some (Semantics.Condition.matches (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.Predicate.hasCounters (some (Semantics.CounterKind.named "p1p1Counter"))))) (Semantics.Action.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Action.moveCounters (Semantics.Amount.lit 1) (some (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter"))) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.NounPhrase.pro (Semantics.Reach.word (Semantics.NounWord.type Semantics.CardType.creature)) Semantics.Plurality.one)) none none))])
semantic_definition gravestorm : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "gravestorm") [] [(Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Action.procedure Semantics.Procedure.cast (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) Semantics.PlayPayment.itsOwnCost none none none)) [] none [] none none none (Semantics.Action.sequentially [(Semantics.Action.copy Semantics.CopySort.fromStack (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) (Semantics.Amount.tally (Semantics.DidClause.mk Semantics.Subject.any (Semantics.Action.move (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.battlefield none))) ((Semantics.Location.zone Semantics.Zone.battlefield none)) (some (Semantics.Location.zone Semantics.Zone.graveyard none)) Semantics.ObservationPoint.departure none []) (Semantics.LookbackSpan.duration (Semantics.Duration.between (some (Semantics.GameEvent.beginningOf Semantics.TurnPart.turn none Semantics.TurnRef.current)) (some Semantics.GameEvent.now)) none)) Semantics.TallyMode.events) []), (Semantics.Action.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Action.chooseNewTargets (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.copy) Semantics.Plurality.many)) none none)]))])
semantic_definition harmonize (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "harmonize") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.can (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Action.procedure Semantics.Procedure.cast Semantics.NounPhrase.this (Semantics.PlayPayment.payingInstead (Semantics.Cost.compound [(Semantics.Cost.inCaller arg0), (Semantics.Cost.perform (Semantics.Action.ownScope (some (Semantics.Deed.action "tap")) (Semantics.Action.enact (Semantics.Deed.action "tap") (Semantics.Action.setStatus Semantics.Status.tapped (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.described (Semantics.DetPhrase.count (Semantics.Quantity.range none (some 1)) none) (Semantics.Predicate.and [(Semantics.Predicate.hasStatus Semantics.Status.untapped), (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)])])))) none none)))])) none (some (Semantics.Location.zone Semantics.Zone.graveyard (some Semantics.NounPhrase.you))) none) ({} : Semantics.DeonticMods))), (Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.costShift (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) (Semantics.CostShift.less (Semantics.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.power) (Semantics.NounPhrase.paid (Semantics.LinkedFacet.objects Semantics.LinkSource.theAlternative (Semantics.Deed.action "tap") Semantics.Plurality.one) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this))) none)) (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "harmonize")) none) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) Semantics.CondMarking.asLongAs)), (Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.instead (Semantics.WouldClause.would Semantics.Subject.any (Semantics.Action.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) ((Semantics.Location.zone Semantics.Zone.stack none)) none Semantics.ObservationPoint.departure none [])) (Semantics.Action.ownScope (some (Semantics.Deed.action "exile")) (Semantics.Action.enact (Semantics.Deed.action "exile") (Semantics.Action.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.exile none)) Semantics.ObservationPoint.arrival none []) none none)) none Semantics.ReplUse.repeatedly none) (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "harmonize")) none) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) Semantics.CondMarking.asLongAs))])
semantic_definition haste : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "haste") [] [])
semantic_definition haunt : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "haunt") [] [(Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Action.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) ((Semantics.Location.zone Semantics.Zone.battlefield none)) (some (Semantics.Location.zone Semantics.Zone.graveyard none)) Semantics.ObservationPoint.departure none [])) [] none [] none none (some (Semantics.Condition.matches Semantics.NounPhrase.this (Semantics.Predicate.or [(Semantics.Predicate.hasType Semantics.CardType.artifact), (Semantics.Predicate.hasType Semantics.CardType.battle), (Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasType Semantics.CardType.enchantment), (Semantics.Predicate.hasType Semantics.CardType.land), (Semantics.Predicate.hasType Semantics.CardType.planeswalker)]))) (Semantics.Action.sequentially [(Semantics.Action.ownScope (some (Semantics.Deed.action "exile")) (Semantics.Action.enact (Semantics.Deed.action "exile") (Semantics.Action.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.pro Semantics.Reach.bare Semantics.Plurality.one)) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.exile none)) Semantics.ObservationPoint.arrival none []) none none)), (Semantics.Action.relate (Semantics.NounPhrase.pro Semantics.Reach.bare Semantics.Plurality.one) (Semantics.RelationLabel.named "haunting") (Semantics.NounPhrase.described (Semantics.DetPhrase.target (Semantics.Quantity.range (some 1) (some 1))) (Semantics.Predicate.hasType Semantics.CardType.creature)))])), (Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Action.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) ((Semantics.Location.zone Semantics.Zone.stack none)) (some (Semantics.Location.zone Semantics.Zone.graveyard none)) Semantics.ObservationPoint.departure none [])) [] none [] none none (some (Semantics.Condition.matches Semantics.NounPhrase.this (Semantics.Predicate.or [(Semantics.Predicate.hasType Semantics.CardType.instant), (Semantics.Predicate.hasType Semantics.CardType.sorcery)]))) (Semantics.Action.sequentially [(Semantics.Action.ownScope (some (Semantics.Deed.action "exile")) (Semantics.Action.enact (Semantics.Deed.action "exile") (Semantics.Action.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.pro Semantics.Reach.bare Semantics.Plurality.one)) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.exile none)) Semantics.ObservationPoint.arrival none []) none none)), (Semantics.Action.relate (Semantics.NounPhrase.pro Semantics.Reach.bare Semantics.Plurality.one) (Semantics.RelationLabel.named "haunting") (Semantics.NounPhrase.described (Semantics.DetPhrase.target (Semantics.Quantity.range (some 1) (some 1))) (Semantics.Predicate.hasType Semantics.CardType.creature)))]))])
semantic_definition hexproof : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "hexproof") [] [(Semantics.Ability.static (Semantics.StaticSpec.cant (Semantics.Subject.by (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.and [(Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.stack none)), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.isOpponentOf Semantics.NounPhrase.you)))]))) (Semantics.Templates.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.target) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) ({} : Semantics.DeonticMods)))])
semantic_definition hideaway (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "hideaway") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0))] [(Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Action.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.battlefield none)) Semantics.ObservationPoint.arrival none [])) [] none [] none none none (Semantics.Action.sequentially [(Semantics.Action.expose Semantics.ExposeVerb.lookAt (Semantics.Exposed.cards (Semantics.NounPhrase.slice Semantics.PileEnd.top (Semantics.Amount.inCaller arg0) (Semantics.Location.zone Semantics.Zone.library (some Semantics.NounPhrase.you))))), (Semantics.Action.enact (Semantics.Deed.action "exile") (Semantics.Action.move (Semantics.NounPhrase.someOf (Semantics.SliceCount.counted (Semantics.Quantity.range (some 1) (some 1))) none (Semantics.NounPhrase.pro Semantics.Reach.bare Semantics.Plurality.many)) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.exile none)) Semantics.ObservationPoint.arrival none [((Semantics.MoveRider.faceDown ({ characteristics := none } : Semantics.FaceDown)))]) none none), (Semantics.Action.move (Semantics.NounPhrase.theRest Semantics.Kind.object Semantics.Plurality.many) Semantics.Location.anyLocation (some (Semantics.Location.fromEnd Semantics.PileEnd.bottom (Semantics.Ordinal.nth 1) (Semantics.Location.zone Semantics.Zone.library none))) Semantics.ObservationPoint.arrival none [((Semantics.MoveRider.inOrder Semantics.Arrangement.randomOrder))]), (Semantics.Action.establish (Semantics.StaticSpec.visibility Semantics.ExposeVerb.lookAt Semantics.NounPhrase.you (Semantics.VisibleThing.objects (Semantics.NounPhrase.pro (Semantics.Reach.verbed (Semantics.Deed.action "exile") (some Semantics.NounWord.card) Semantics.VerbedMarking.attributive) Semantics.Plurality.one))) (some (Semantics.Duration.forAsLongAs (Semantics.Condition.matches (Semantics.NounPhrase.pro (Semantics.Reach.verbed (Semantics.Deed.action "exile") (some Semantics.NounWord.card) Semantics.VerbedMarking.attributive) Semantics.Plurality.one) (Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.exile none))))))]))])
semantic_definition horsemanship : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "horsemanship") [] [(Semantics.Ability.static (Semantics.StaticSpec.cant (Semantics.Subject.by (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.not (Semantics.Predicate.hasKeyword (Semantics.KeywordTerm.the (Semantics.KeywordLabel.named "horsemanship"))))]))) (Semantics.Templates.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.block) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) ({} : Semantics.DeonticMods)))])
semantic_definition impending (arg0 : Semantics.Amount) (arg1 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "impending") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0)), (Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg1))] [(Semantics.Ability.static (Semantics.StaticSpec.can (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Action.procedure Semantics.Procedure.cast Semantics.NounPhrase.this (Semantics.PlayPayment.payingInstead (Semantics.Cost.inCaller arg1)) none none none) ({} : Semantics.DeonticMods))), (Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.entryRider (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.MoveRider.withCounters (Semantics.Amount.inCaller arg0) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "timeCounter")) Semantics.EntryCounterMark.fresh)) (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "impending")) none) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) Semantics.CondMarking.asLongAs)), (Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.characteristicChange (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) [(Semantics.CharacteristicEdit.typeLine Semantics.QualityOp.loses ({ supertypes := none, types := (some [Semantics.CardType.creature]), subtypes := none, retained := none } : Semantics.TypeLineChanges))]) (Semantics.Condition.and [(Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "impending")) none) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)), (Semantics.Condition.matches (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.Predicate.hasCounters (some (Semantics.CounterKind.named "timeCounter"))))]) Semantics.CondMarking.asLongAs)), (Semantics.Ability.triggered (Semantics.GameEvent.beginningOf Semantics.TurnPart.endStep (some Semantics.NounPhrase.you) (Semantics.TurnRef.each none)) [] none [] none none (some (Semantics.Condition.and [(Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "impending")) none) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)), (Semantics.Condition.matches (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.Predicate.hasCounters (some (Semantics.CounterKind.named "timeCounter"))))])) (Semantics.Action.removeCounters (some (Semantics.Quantity.range (some 1) (some 1))) (some (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "timeCounter"))) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.CounterBatch.one true))])
semantic_definition improvise : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "improvise") [] [(Semantics.Ability.static (Semantics.StaticSpec.paySubstitute (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) ({ action := (Semantics.Action.ownScope (some (Semantics.Deed.action "tap")) (Semantics.Action.enact (Semantics.Deed.action "tap") (Semantics.Action.setStatus Semantics.Status.tapped (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.hasStatus Semantics.Status.untapped), (Semantics.Predicate.hasType Semantics.CardType.artifact), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)])))) none none)), pays := [Semantics.PipPaid.generic] } : Semantics.ManaSubstitute)))])
semantic_definition increment : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "increment") [] [(Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Action.procedure Semantics.Procedure.cast (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.stack none)), (Semantics.Predicate.not (Semantics.Predicate.abilityHead Semantics.AbilityClass.anyOnStack))])) Semantics.PlayPayment.itsOwnCost none none none)) [] none [] none none (some (Semantics.Condition.and [(Semantics.Condition.matches (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.Predicate.hasType Semantics.CardType.creature)), (Semantics.Condition.or [(Semantics.Condition.compareAmt (Semantics.Amount.paid Semantics.LinkedFacet.manaValueSpent (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.spell) Semantics.Plurality.one)) Semantics.Comparator.greater (Semantics.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.power) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none))), (Semantics.Condition.compareAmt (Semantics.Amount.paid Semantics.LinkedFacet.manaValueSpent (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.spell) Semantics.Plurality.one)) Semantics.Comparator.greater (Semantics.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.toughness) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)))])])) (Semantics.Action.putCounters (Semantics.Amount.lit 1) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none) Semantics.CounterBatch.one true))])
semantic_definition indestructible : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "indestructible") [] [(Semantics.Ability.static (Semantics.StaticSpec.cant Semantics.Subject.any (Semantics.Templates.deonticActionFor (Semantics.Deed.action "destroy") (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) ({} : Semantics.DeonticMods)))])
semantic_definition infect : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "infect") [] [])
semantic_definition infinity (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "infinity") [(Semantics.KeywordParam.ability arg0)] [(Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.abilityGrant (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) arg0) (Semantics.Condition.matches (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.Predicate.hasDesignation (Semantics.DesignationLabel.named "harnessed") none)) Semantics.CondMarking.asLongAs))])
semantic_definition ingest : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "ingest") [] [(Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) (Semantics.Action.dealDamage Semantics.Amount.anyAmount (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) Semantics.Predicate.anyPlayer) Semantics.DamageKind.combatOnly)) [] none [] none none none (Semantics.Action.instruct (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.player) Semantics.Plurality.one) (Semantics.Action.ownScope (some (Semantics.Deed.action "exile")) (Semantics.Action.enact (Semantics.Deed.action "exile") (Semantics.Action.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.slice Semantics.PileEnd.top (Semantics.Amount.lit 1) (Semantics.Location.zone Semantics.Zone.library (some (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.player) Semantics.Plurality.one))))) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.exile none)) Semantics.ObservationPoint.arrival none []) none none))))])
semantic_definition intimidate : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "intimidate") [] [(Semantics.Ability.static (Semantics.StaticSpec.cant Semantics.Subject.any (Semantics.Templates.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.block) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) ({ except := (some (Semantics.DeonticExcept.byObjects (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.or [(Semantics.Predicate.hasType Semantics.CardType.artifact), (Semantics.Predicate.shares Semantics.QualitySort.color Semantics.Agreement.share (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none))])]))) } : Semantics.DeonticMods)))])
semantic_definition jobSelect : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "jobSelect") [] [(Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Action.move (Semantics.NounPhrase.asType Semantics.CardType.artifact Semantics.NounPhrase.this (some (Semantics.Subtype.of Semantics.CardType.artifact "Equipment"))) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.battlefield none)) Semantics.ObservationPoint.arrival none [])) [] none [] none none none (Semantics.Action.sequentially [(Semantics.Action.createObject (Semantics.Amount.lit 1) (Semantics.CreationSpec.token (Semantics.TokenSpec.written ({ characteristics := ({ name := none, cost := none, colors := [], supertypes := [], types := [Semantics.CardType.creature], subtypes := [(Semantics.Subtype.of Semantics.CardType.creature "Hero")], text := [], power := (some (Semantics.Amount.lit 1)), toughness := (some (Semantics.Amount.lit 1)), loyalty := none, defense := none } : Semantics.Characteristics), qualities := [] } : Semantics.CharacteristicBundle)) []) true none), (Semantics.Action.ownScope (some (Semantics.Deed.action "attach")) (Semantics.Action.enact (Semantics.Deed.action "attach") (Semantics.Action.attachment Semantics.AttachMove.attached (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asType Semantics.CardType.artifact Semantics.NounPhrase.this (some (Semantics.Subtype.of Semantics.CardType.artifact "Equipment")))) (some (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.pro Semantics.Reach.tokenBorn Semantics.Plurality.one)))) none none))]))])
semantic_definition jumpStart : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "jumpStart") [] [])
semantic_definition kicker (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "kicker") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.addedCost (Semantics.Cost.inCaller arg0) true Semantics.PayTimes.once))])
semantic_definition landwalk (arg0 : Semantics.Predicate) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "landwalk") [(Semantics.KeywordParam.quality (Semantics.Predicate.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.cant Semantics.Subject.any (Semantics.Templates.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.block) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) ({} : Semantics.DeonticMods)) (Semantics.Condition.exists_ (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.land), (Semantics.Predicate.inCaller arg0), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller (Semantics.NounPhrase.combatPlayer Semantics.CombatRole.defending))]))) Semantics.CondMarking.asLongAs))])
semantic_definition lifelink : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "lifelink") [] [])
semantic_definition livingMetal : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "livingMetal") [] [(Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.characteristicChange (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) [(Semantics.CharacteristicEdit.typeLine Semantics.QualityOp.adds ({ supertypes := none, types := (some [Semantics.CardType.artifact, Semantics.CardType.creature]), subtypes := none, retained := none } : Semantics.TypeLineChanges))]) (Semantics.Condition.duringPart Semantics.TurnPart.turn (some Semantics.NounPhrase.you)) Semantics.CondMarking.asLongAs))])
semantic_definition livingWeapon : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "livingWeapon") [] [(Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Action.move (Semantics.NounPhrase.asType Semantics.CardType.artifact Semantics.NounPhrase.this (some (Semantics.Subtype.of Semantics.CardType.artifact "Equipment"))) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.battlefield none)) Semantics.ObservationPoint.arrival none [])) [] none [] none none none (Semantics.Action.sequentially [(Semantics.Action.createObject (Semantics.Amount.lit 1) (Semantics.CreationSpec.token (Semantics.TokenSpec.written ({ characteristics := ({ name := none, cost := none, colors := [Semantics.Color.black], supertypes := [], types := [Semantics.CardType.creature], subtypes := [(Semantics.Subtype.of Semantics.CardType.creature "Phyrexian"), (Semantics.Subtype.of Semantics.CardType.creature "Germ")], text := [], power := (some (Semantics.Amount.lit 0)), toughness := (some (Semantics.Amount.lit 0)), loyalty := none, defense := none } : Semantics.Characteristics), qualities := [] } : Semantics.CharacteristicBundle)) []) true none), (Semantics.Action.ownScope (some (Semantics.Deed.action "attach")) (Semantics.Action.enact (Semantics.Deed.action "attach") (Semantics.Action.attachment Semantics.AttachMove.attached (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asType Semantics.CardType.artifact Semantics.NounPhrase.this (some (Semantics.Subtype.of Semantics.CardType.artifact "Equipment")))) (some (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.pro Semantics.Reach.tokenBorn Semantics.Plurality.one)))) none none))]))])
semantic_definition madness (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "madness") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.instead (Semantics.WouldClause.would (Semantics.Subject.actor (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) Semantics.Predicate.anyPlayer)) (Semantics.Action.enact (Semantics.Deed.action "discard") (Semantics.Templates.deonticActionFor (Semantics.Deed.action "discard") Semantics.NounPhrase.this) none none)) (Semantics.Action.enact (Semantics.Deed.action "discard") (Semantics.Action.move Semantics.NounPhrase.this ((Semantics.Location.zone Semantics.Zone.hand none)) (some (Semantics.Location.zone Semantics.Zone.exile none)) Semantics.ObservationPoint.arrival none []) none none) none Semantics.ReplUse.repeatedly none)), (Semantics.Ability.triggered (Semantics.GameEvent.causes (Semantics.Causing.event (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Action.enact (Semantics.Deed.action "discard") (Semantics.Templates.deonticActionFor (Semantics.Deed.action "discard") Semantics.NounPhrase.this) none none))) (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Action.move Semantics.NounPhrase.this ((Semantics.Location.zone Semantics.Zone.hand none)) (some (Semantics.Location.zone Semantics.Zone.exile none)) Semantics.ObservationPoint.arrival none []))) [] none [] none none none (Semantics.Action.instruct (Semantics.NounPhrase.possessorOf Semantics.PossessorAxis.owner Semantics.NounPhrase.this) (Semantics.Action.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Action.establish (Semantics.StaticSpec.can (Semantics.Subject.actor Semantics.NounPhrase.actor) (Semantics.Action.procedure Semantics.Procedure.cast Semantics.NounPhrase.this (Semantics.PlayPayment.payingInstead (Semantics.Cost.inCaller arg0)) none (some (Semantics.Location.zone Semantics.Zone.exile none)) none) ({} : Semantics.DeonticMods)) none) none (some (Semantics.Action.move Semantics.NounPhrase.this Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.graveyard none)) Semantics.ObservationPoint.arrival none [])))))])
semantic_definition maxSpeed (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "maxSpeed") [(Semantics.KeywordParam.ability arg0)] [(Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.abilityGrant Semantics.NounPhrase.this arg0) (Semantics.Condition.compareAmt (Semantics.Amount.statOf (Semantics.ProjAxis.playerStat Semantics.PlayerStat.speed) Semantics.NounPhrase.you) Semantics.Comparator.eq (Semantics.Amount.lit 4)) Semantics.CondMarking.asLongAs))])
semantic_definition mayhem (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "mayhem") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.can (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Action.procedure Semantics.Procedure.cast Semantics.NounPhrase.this (Semantics.PlayPayment.payingInstead (Semantics.Cost.inCaller arg0)) none (some (Semantics.Location.zone Semantics.Zone.graveyard (some Semantics.NounPhrase.you))) none) ({} : Semantics.DeonticMods)) (Semantics.Condition.did (Semantics.DidClause.mk (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Action.enact (Semantics.Deed.action "discard") (Semantics.Templates.deonticActionFor (Semantics.Deed.action "discard") Semantics.NounPhrase.this) none none) (Semantics.LookbackSpan.duration (Semantics.Duration.between (some (Semantics.GameEvent.beginningOf Semantics.TurnPart.turn none Semantics.TurnRef.current)) (some Semantics.GameEvent.now)) none))) Semantics.CondMarking.asLongAs))])
semantic_definition megamorph (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "megamorph") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.can (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Action.procedure Semantics.Procedure.cast Semantics.NounPhrase.this (Semantics.PlayPayment.faceDown ({ characteristics := (some ({ name := none, cost := none, colors := [], supertypes := [], types := [Semantics.CardType.creature], subtypes := [], text := [], power := (some (Semantics.Amount.lit 2)), toughness := (some (Semantics.Amount.lit 2)), loyalty := none, defense := none } : Semantics.Characteristics)) } : Semantics.FaceDown) (Semantics.Cost.mana [(Semantics.ManaSymbol.simple (Semantics.SimpleManaSymbol.generic 3))])) none none none) ({} : Semantics.DeonticMods))), (Semantics.Ability.static (Semantics.StaticSpec.specialAction none (Semantics.Cost.inCaller arg0) (Semantics.Action.sequentially [(Semantics.Action.setStatus Semantics.Status.faceUp Semantics.NounPhrase.this), (Semantics.Action.putCounters (Semantics.Amount.lit 1) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) Semantics.NounPhrase.this Semantics.CounterBatch.one true)])))])
semantic_definition melee : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "melee") [] [(Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) (Semantics.Action.attack none)) [] none [] none none none (Semantics.Action.establish (Semantics.StaticSpec.ptModification (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none) (Semantics.Delta.up (Semantics.Amount.arith Semantics.ArithOp.times (Semantics.Amount.lit 1) (Semantics.Amount.tally (Semantics.DidClause.mk (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Action.attackWith (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.hasType Semantics.CardType.creature)) (some Semantics.NounPhrase.which)) (Semantics.LookbackSpan.duration (Semantics.Duration.between (some (Semantics.GameEvent.beginningOf Semantics.TurnPart.combat none Semantics.TurnRef.current)) (some Semantics.GameEvent.now)) none)) Semantics.TallyMode.events))) (Semantics.Delta.up (Semantics.Amount.arith Semantics.ArithOp.times (Semantics.Amount.lit 1) (Semantics.Amount.tally (Semantics.DidClause.mk (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Action.attackWith (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.hasType Semantics.CardType.creature)) (some Semantics.NounPhrase.which)) (Semantics.LookbackSpan.duration (Semantics.Duration.between (some (Semantics.GameEvent.beginningOf Semantics.TurnPart.combat none Semantics.TurnRef.current)) (some Semantics.GameEvent.now)) none)) Semantics.TallyMode.events)))) (some (Semantics.Duration.between none (some (Semantics.GameEvent.endOf Semantics.TurnPart.turn none Semantics.TurnRef.current))))))])
semantic_definition menace : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "menace") [] [(Semantics.Ability.static (Semantics.StaticSpec.cant (Semantics.Subject.by (Semantics.NounPhrase.described (Semantics.DetPhrase.count (Semantics.Quantity.moreThanOf (Semantics.Amount.lit 1)) none) (Semantics.Predicate.hasType Semantics.CardType.creature))) (Semantics.Templates.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.block) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) ({} : Semantics.DeonticMods)))])
semantic_definition mentor : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "mentor") [] [(Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) (Semantics.Action.attack none)) [] none [] none none none (Semantics.Action.putCounters (Semantics.Amount.lit 1) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) (Semantics.NounPhrase.described (Semantics.DetPhrase.target (Semantics.Quantity.range (some 1) (some 1))) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.inCombat Semantics.CombatRelation.attackerOf none), (Semantics.Predicate.compare [(Semantics.ProjAxis.stat Semantics.Stat.power)] Semantics.Comparator.less (Semantics.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.power) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)))])) Semantics.CounterBatch.one true))])
semantic_definition miracle (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "miracle") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [])
semantic_definition mobilize (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "mobilize") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0))] [(Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) (Semantics.Action.attack none)) [] none [] none none none (Semantics.Action.sequentially [(Semantics.Action.createObject (Semantics.Amount.inCaller arg0) (Semantics.CreationSpec.token (Semantics.TokenSpec.written ({ characteristics := ({ name := none, cost := none, colors := [Semantics.Color.red], supertypes := [], types := [Semantics.CardType.creature], subtypes := [(Semantics.Subtype.of Semantics.CardType.creature "Warrior")], text := [], power := (some (Semantics.Amount.lit 1)), toughness := (some (Semantics.Amount.lit 1)), loyalty := none, defense := none } : Semantics.Characteristics), qualities := [] } : Semantics.CharacteristicBundle)) [(Semantics.MoveRider.entersAs Semantics.Status.tapped), (Semantics.MoveRider.entersAttacking none)]) true none), (Semantics.Action.delay (Semantics.GameEvent.beginningOf Semantics.TurnPart.endStep none (Semantics.TurnRef.next none)) [] none (Semantics.Action.ownScope (some (Semantics.Deed.action "sacrifice")) (Semantics.Action.enact (Semantics.Deed.action "sacrifice") (Semantics.Action.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.pro Semantics.Reach.tokenBorn Semantics.Plurality.many)) ((Semantics.Location.zone Semantics.Zone.battlefield none)) (some (Semantics.Location.zone Semantics.Zone.graveyard none)) Semantics.ObservationPoint.departure none []) none none)))]))])
semantic_definition moreThanMeetsTheEye (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "moreThanMeetsTheEye") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [])
semantic_definition morph (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "morph") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.can (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Action.procedure Semantics.Procedure.cast Semantics.NounPhrase.this (Semantics.PlayPayment.faceDown ({ characteristics := (some ({ name := none, cost := none, colors := [], supertypes := [], types := [Semantics.CardType.creature], subtypes := [], text := [], power := (some (Semantics.Amount.lit 2)), toughness := (some (Semantics.Amount.lit 2)), loyalty := none, defense := none } : Semantics.Characteristics)) } : Semantics.FaceDown) (Semantics.Cost.mana [(Semantics.ManaSymbol.simple (Semantics.SimpleManaSymbol.generic 3))])) none none none) ({} : Semantics.DeonticMods))), (Semantics.Ability.static (Semantics.StaticSpec.specialAction none (Semantics.Cost.inCaller arg0) (Semantics.Action.setStatus Semantics.Status.faceUp Semantics.NounPhrase.this)))])
semantic_definition multikicker (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "multikicker") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [])
semantic_definition mutate (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "mutate") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.can (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Action.procedure Semantics.Procedure.cast Semantics.NounPhrase.this (Semantics.PlayPayment.payingInstead (Semantics.Cost.inCaller arg0)) none none none) ({} : Semantics.DeonticMods))), (Semantics.Ability.static (Semantics.StaticSpec.addsTarget (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.not (Semantics.Predicate.hasSubtype (Semantics.Subtype.of Semantics.CardType.creature "Human"))), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.owner (Semantics.NounPhrase.possessorOf Semantics.PossessorAxis.owner (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)))]))), (Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.instead (Semantics.WouldClause.would Semantics.Subject.any (Semantics.Action.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.battlefield none)) Semantics.ObservationPoint.arrival none [])) (Semantics.Action.chooseModes (Semantics.Quantity.range (some 1) (some 1)) [(none, none, (Semantics.Action.merge (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) (Semantics.NounPhrase.paid (Semantics.LinkedFacet.objects Semantics.LinkSource.theAlternative (Semantics.Deed.core Semantics.CoreDeed.target) Semantics.Plurality.one) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) Semantics.MergePosition.top)), (none, none, (Semantics.Action.merge (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) (Semantics.NounPhrase.paid (Semantics.LinkedFacet.objects Semantics.LinkSource.theAlternative (Semantics.Deed.core Semantics.CoreDeed.target) Semantics.Plurality.one) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) Semantics.MergePosition.bottom))]) none Semantics.ReplUse.repeatedly none) (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "mutate")) none) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) Semantics.CondMarking.asLongAs))])
semantic_definition myriad : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "myriad") [] [(Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) (Semantics.Action.attack none)) [] none [] none none none (Semantics.Action.sequentially [(Semantics.Action.doForEach (Semantics.NounPhrase.described Semantics.DetPhrase.each (Semantics.Predicate.and [(Semantics.Predicate.isOpponentOf Semantics.NounPhrase.you), (Semantics.Predicate.otherThan (Semantics.NounPhrase.combatPlayer Semantics.CombatRole.defending))])) (Semantics.Action.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Action.createObject (Semantics.Amount.lit 1) (Semantics.CreationSpec.token (Semantics.TokenSpec.copyOf (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none) []) [(Semantics.MoveRider.entersAs Semantics.Status.tapped), (Semantics.MoveRider.entersAttacking (some (Semantics.NounPhrase.or [(Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.player) Semantics.Plurality.one), (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.planeswalker), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.player) Semantics.Plurality.one))]))])))]) true none) none none)), (Semantics.Action.doIf (Semantics.Condition.compareAmt (Semantics.Amount.countOf (Semantics.NounPhrase.pro Semantics.Reach.tokenBorn Semantics.Plurality.many)) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) (Semantics.Action.delay (Semantics.GameEvent.beginningOf Semantics.TurnPart.endOfCombat none (Semantics.TurnRef.next none)) [] none (Semantics.Action.ownScope (some (Semantics.Deed.action "exile")) (Semantics.Action.enact (Semantics.Deed.action "exile") (Semantics.Action.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.pro Semantics.Reach.tokenBorn Semantics.Plurality.many)) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.exile none)) Semantics.ObservationPoint.arrival none []) none none))) none)]))])
semantic_definition nightbound : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "nightbound") [] [(Semantics.Ability.static (Semantics.StaticSpec.asEvent (Semantics.GameEvent.gameBecomes (Semantics.DesignationLabel.named "day")) (Semantics.Action.doIf (Semantics.Condition.matches (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.Predicate.currentFace Semantics.CardFaceSide.back)) (Semantics.Action.ownScope (some (Semantics.Deed.action "transform")) (Semantics.Action.enact (Semantics.Deed.action "transform") (Semantics.Action.turnOver (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this))) none none)) none))), (Semantics.Ability.static (Semantics.StaticSpec.cant Semantics.Subject.any (Semantics.Templates.deonticActionFor (Semantics.Deed.action "transform") (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) ({ except := (some (Semantics.DeonticExcept.byKeyword (Semantics.KeywordLabel.named "nightbound"))) } : Semantics.DeonticMods)))])
semantic_definition ninjutsu (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "ninjutsu") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.activated (Semantics.Cost.compound [(Semantics.Cost.inCaller arg0), (Semantics.Cost.perform (Semantics.Action.expose Semantics.ExposeVerb.reveal (Semantics.Exposed.cards Semantics.NounPhrase.this))), (Semantics.Cost.perform (Semantics.Action.enact (Semantics.Deed.core Semantics.CoreDeed.return_) (Semantics.Action.move (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.inCombat Semantics.CombatRelation.attackerOf none), (Semantics.Predicate.not (Semantics.Predicate.inCombat Semantics.CombatRelation.blockedBy none)), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)])) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.hand none)) Semantics.ObservationPoint.arrival none []) none none))]) (Semantics.Action.move Semantics.NounPhrase.this ((Semantics.Location.zone Semantics.Zone.hand (some Semantics.NounPhrase.you))) (some (Semantics.Location.zone Semantics.Zone.battlefield none)) Semantics.ObservationPoint.arrival none [((Semantics.MoveRider.entersAs Semantics.Status.tapped)), ((Semantics.MoveRider.entersAttacking none))]) none none none none)])
semantic_definition offering (arg0 : Semantics.Predicate) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "offering") [(Semantics.KeywordParam.quality (Semantics.Predicate.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.addedCost (Semantics.Cost.perform (Semantics.Action.ownScope (some (Semantics.Deed.action "sacrifice")) (Semantics.Action.enact (Semantics.Deed.action "sacrifice") (Semantics.Action.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.inCaller arg0), (Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.battlefield none))]))) ((Semantics.Location.zone Semantics.Zone.battlefield none)) (some (Semantics.Location.zone Semantics.Zone.graveyard none)) Semantics.ObservationPoint.departure none []) none none))) true Semantics.PayTimes.once)), (Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.costShift (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) (Semantics.CostShift.bySymbols (Semantics.ManaCostTerm.manaCostOf (Semantics.NounPhrase.paid (Semantics.LinkedFacet.objects Semantics.LinkSource.theAdditional (Semantics.Deed.action "sacrifice") Semantics.Plurality.one) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this))) false false)) (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "offering")) none) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) Semantics.CondMarking.asLongAs)), (Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.can (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Templates.deonticActionFor (Semantics.Deed.action "cast") Semantics.NounPhrase.this) ({ timing := (some (Semantics.Timing.onlyIf (Semantics.Condition.hasPriority Semantics.NounPhrase.you))) } : Semantics.DeonticMods)) (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "offering")) none) Semantics.NounPhrase.this) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) Semantics.CondMarking.asLongAs))])
semantic_definition offspring (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "offspring") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.addedCost (Semantics.Cost.inCaller arg0) true Semantics.PayTimes.once)), (Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Action.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.battlefield none)) Semantics.ObservationPoint.arrival none [])) [] none [] none none (some (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "offspring")) none) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) Semantics.Comparator.atLeast (Semantics.Amount.lit 1))) (Semantics.Action.createObject (Semantics.Amount.lit 1) (Semantics.CreationSpec.token (Semantics.TokenSpec.copyOf (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) [(Semantics.CopyExcept.edits [(Semantics.CharacteristicEdit.stat Semantics.QualityOp.sets Semantics.CharacteristicStat.power (Semantics.Amount.lit 1)), (Semantics.CharacteristicEdit.stat Semantics.QualityOp.sets Semantics.CharacteristicStat.toughness (Semantics.Amount.lit 1))])]) []) true none))])
semantic_definition outlast (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "outlast") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.activated (Semantics.Cost.compound [(Semantics.Cost.inCaller arg0), Semantics.Cost.tapSymbol]) (Semantics.Action.putCounters (Semantics.Amount.lit 1) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none) Semantics.CounterBatch.one true) (some (Semantics.Timing.and [(Semantics.Timing.inPart Semantics.TurnPart.mainPhase (some Semantics.NounPhrase.you) Semantics.TurnRef.current), (Semantics.Timing.onlyIf (Semantics.Condition.not (Semantics.Condition.exists_ (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.stack none))))))])) none none none)])
semantic_definition overload (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "overload") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.can (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Action.procedure Semantics.Procedure.cast Semantics.NounPhrase.this (Semantics.PlayPayment.payingInstead (Semantics.Cost.inCaller arg0)) none none none) ({} : Semantics.DeonticMods))), (Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.textChange (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) Semantics.TextChange.targetToEach) (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "overload")) none) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) Semantics.CondMarking.asLongAs))])
semantic_definition paradigm : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "paradigm") [] [])
semantic_definition partner : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "partner") [] [])
semantic_definition partnerCharacterSelect : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "partnerCharacterSelect") [] [])
semantic_definition partnerFatherAndSon : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "partnerFatherAndSon") [] [])
semantic_definition partnerFriendsForever : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "partnerFriendsForever") [] [])
semantic_definition partnerSurvivors : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "partnerSurvivors") [] [])
semantic_definition partnerWith (arg0 : Semantics.Predicate) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "partnerWith") [(Semantics.KeywordParam.quality (Semantics.Predicate.inCaller arg0))] [])
semantic_definition persist : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "persist") [] [(Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Action.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) ((Semantics.Location.zone Semantics.Zone.battlefield none)) (some (Semantics.Location.zone Semantics.Zone.graveyard none)) Semantics.ObservationPoint.departure none [])) [] none [] none none (some (Semantics.Condition.not (Semantics.Condition.matches (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.Predicate.hasCounters (some (Semantics.CounterKind.named "m1m1Counter")))))) (Semantics.Action.enact (Semantics.Deed.core Semantics.CoreDeed.return_) (Semantics.Action.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.battlefield none)) Semantics.ObservationPoint.arrival none [((Semantics.MoveRider.under (Semantics.NounPhrase.possessorOf Semantics.PossessorAxis.owner (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)))), ((Semantics.MoveRider.withCounters (Semantics.Amount.lit 1) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "m1m1Counter")) Semantics.EntryCounterMark.fresh))]) none none))])
semantic_definition phasing : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "phasing") [] [])
semantic_definition plot (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "plot") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.specialAction (some (Semantics.Timing.and [(Semantics.Timing.inPart Semantics.TurnPart.mainPhase (some Semantics.NounPhrase.you) Semantics.TurnRef.current), (Semantics.Timing.onlyIf (Semantics.Condition.not (Semantics.Condition.exists_ (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.stack none))))))])) (Semantics.Cost.inCaller arg0) (Semantics.Action.sequentially [(Semantics.Action.ownScope (some (Semantics.Deed.action "exile")) (Semantics.Action.enact (Semantics.Deed.action "exile") (Semantics.Action.move (Semantics.NounPhrase.inCaller Semantics.NounPhrase.this) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.exile none)) Semantics.ObservationPoint.arrival none []) none none)), (Semantics.Action.establish (Semantics.StaticSpec.can (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Action.procedure Semantics.Procedure.cast (Semantics.NounPhrase.pro Semantics.Reach.bare Semantics.Plurality.one) Semantics.PlayPayment.withoutPaying none (some (Semantics.Location.zone Semantics.Zone.exile none)) none) ({ timing := (some (Semantics.Timing.and [(Semantics.Timing.inPart Semantics.TurnPart.mainPhase (some Semantics.NounPhrase.you) Semantics.TurnRef.current), (Semantics.Timing.onlyIf (Semantics.Condition.not (Semantics.Condition.exists_ (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.stack none))))))])) } : Semantics.DeonticMods)) (some (Semantics.Duration.between (some (Semantics.GameEvent.endOf Semantics.TurnPart.turn none Semantics.TurnRef.current)) none)))])))])
semantic_definition poisonous (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "poisonous") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0))] [(Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) (Semantics.Action.dealDamage Semantics.Amount.anyAmount (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) Semantics.Predicate.anyPlayer) Semantics.DamageKind.combatOnly)) [] none [] none none none (Semantics.Action.putCounters (Semantics.Amount.inCaller arg0) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "poison")) (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.player) Semantics.Plurality.one) Semantics.CounterBatch.one true))])
semantic_definition powerUp (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "powerUp") [(Semantics.KeywordParam.ability arg0)] [(Semantics.Ability.qualified arg0 none (some (Semantics.UsageLimit.mk (Semantics.CountBound.moreThan (Semantics.Amount.lit 1)) Semantics.Period.thisObject)) none none), (Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.costShift (Semantics.NounPhrase.asMarker Semantics.MarkerWord.ability Semantics.NounPhrase.this) (Semantics.CostShift.bySymbols (Semantics.ManaCostTerm.manaCostOf (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) false false)) (Semantics.Condition.matches (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.Predicate.did (Semantics.DidClause.mk Semantics.Subject.any (Semantics.Action.move Semantics.NounPhrase.which Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.battlefield none)) Semantics.ObservationPoint.arrival none []) (Semantics.LookbackSpan.duration (Semantics.Duration.between (some (Semantics.GameEvent.beginningOf Semantics.TurnPart.turn none Semantics.TurnRef.current)) (some Semantics.GameEvent.now)) none)))) Semantics.CondMarking.asLongAs))])
semantic_definition protection (arg0 : Semantics.Predicate) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "protection") [(Semantics.KeywordParam.quality (Semantics.Predicate.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.cant (Semantics.Subject.by (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.or [(Semantics.Predicate.and [(Semantics.Predicate.and [(Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.stack none)), (Semantics.Predicate.not (Semantics.Predicate.abilityHead Semantics.AbilityClass.anyOnStack))]), (Semantics.Predicate.inCaller arg0)]), (Semantics.Predicate.abilityOf (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.and [Semantics.Predicate.isSource, (Semantics.Predicate.inCaller arg0)])))]))) (Semantics.Templates.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.target) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) ({} : Semantics.DeonticMods))), (Semantics.Ability.static (Semantics.StaticSpec.cant Semantics.Subject.any (Semantics.Action.attachment Semantics.AttachMove.attached (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.and [(Semantics.Predicate.hasSubtype (Semantics.Subtype.of Semantics.CardType.enchantment "Aura")), (Semantics.Predicate.inCaller arg0)])) (some (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this))) ({} : Semantics.DeonticMods))), (Semantics.Ability.static (Semantics.StaticSpec.cant Semantics.Subject.any (Semantics.Action.attachment Semantics.AttachMove.attached (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.and [(Semantics.Predicate.or [(Semantics.Predicate.hasSubtype (Semantics.Subtype.of Semantics.CardType.artifact "Equipment")), (Semantics.Predicate.hasSubtype (Semantics.Subtype.of Semantics.CardType.artifact "Fortification"))]), (Semantics.Predicate.inCaller arg0)])) (some (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this))) ({} : Semantics.DeonticMods))), (Semantics.Ability.static (Semantics.StaticSpec.damageRule Semantics.DamageKind.any (Semantics.DamageAgent.dealtBy (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.and [Semantics.Predicate.isSource, (Semantics.Predicate.inCaller arg0)]))) (Semantics.DamageScope.toRecipient (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) (Semantics.DamageOp.prevent Semantics.PreventCut.all none) Semantics.ReplUse.repeatedly)), (Semantics.Ability.static (Semantics.StaticSpec.cant (Semantics.Subject.by (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.inCaller arg0)]))) (Semantics.Templates.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.block) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) ({} : Semantics.DeonticMods)))])
set_option linter.unusedVariables false in
semantic_definition prototype (arg0 : Semantics.Cost) (arg1 : Semantics.Amount) (arg2 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "prototype") [] [])
semantic_definition provoke : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "provoke") [] [(Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) (Semantics.Action.attack none)) [] none [] none none none (Semantics.Action.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Action.establish (Semantics.StaticSpec.must (Semantics.Subject.actor (Semantics.NounPhrase.described (Semantics.DetPhrase.target (Semantics.Quantity.range (some 1) (some 1))) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller (Semantics.NounPhrase.combatPlayer Semantics.CombatRole.defending))]))) (Semantics.Templates.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.block) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) ({} : Semantics.DeonticMods)) (some (Semantics.Duration.between none (some (Semantics.GameEvent.endOf Semantics.TurnPart.combat none Semantics.TurnRef.current))))) (some (Semantics.Action.ownScope (some (Semantics.Deed.action "untap")) (Semantics.Action.enact (Semantics.Deed.action "untap") (Semantics.Action.setStatus Semantics.Status.untapped (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.pro (Semantics.Reach.word (Semantics.NounWord.type Semantics.CardType.creature)) Semantics.Plurality.one))) none none))) none))])
semantic_definition prowess : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "prowess") [] [(Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Action.procedure Semantics.Procedure.cast (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.stack none)), (Semantics.Predicate.not (Semantics.Predicate.hasType Semantics.CardType.creature))])) Semantics.PlayPayment.itsOwnCost none none none)) [] none [] none none none (Semantics.Action.establish (Semantics.StaticSpec.ptModification (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none) (Semantics.Delta.up (Semantics.Amount.lit 1)) (Semantics.Delta.up (Semantics.Amount.lit 1))) (some (Semantics.Duration.between none (some (Semantics.GameEvent.endOf Semantics.TurnPart.turn none Semantics.TurnRef.current))))))])
semantic_definition prowl (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "prowl") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.can (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Action.procedure Semantics.Procedure.cast Semantics.NounPhrase.this (Semantics.PlayPayment.payingInstead (Semantics.Cost.inCaller arg0)) none none none) ({} : Semantics.DeonticMods)) (Semantics.Condition.did (Semantics.DidClause.mk (Semantics.Subject.actor (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.shares (Semantics.QualitySort.subtype Semantics.CardType.creature) Semantics.Agreement.share (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)]))) (Semantics.Action.dealDamage Semantics.Amount.anyAmount Semantics.NounPhrase.which Semantics.DamageKind.combatOnly) (Semantics.LookbackSpan.duration (Semantics.Duration.between (some (Semantics.GameEvent.beginningOf Semantics.TurnPart.turn none Semantics.TurnRef.current)) (some Semantics.GameEvent.now)) none))) Semantics.CondMarking.asLongAs))])
semantic_definition rampage (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "rampage") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0))] [(Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Action.block (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none))) [] none [] none none none (Semantics.Action.establish (Semantics.StaticSpec.ptModification (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none) (Semantics.Delta.up (Semantics.Amount.arith Semantics.ArithOp.times (Semantics.Amount.inCaller arg0) (Semantics.Amount.arith Semantics.ArithOp.minus (Semantics.Amount.countOf (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.inCombat Semantics.CombatRelation.blockerOf (some (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)))]))) (Semantics.Amount.lit 1)))) (Semantics.Delta.up (Semantics.Amount.arith Semantics.ArithOp.times (Semantics.Amount.inCaller arg0) (Semantics.Amount.arith Semantics.ArithOp.minus (Semantics.Amount.countOf (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.inCombat Semantics.CombatRelation.blockerOf (some (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)))]))) (Semantics.Amount.lit 1))))) (some (Semantics.Duration.between none (some (Semantics.GameEvent.endOf Semantics.TurnPart.turn none Semantics.TurnRef.current))))))])
semantic_definition ravenous : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "ravenous") [] [(Semantics.Ability.static (Semantics.StaticSpec.entryRider (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.MoveRider.withCounters (Semantics.Amount.letter Semantics.Letter.x) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) Semantics.EntryCounterMark.fresh))), (Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Action.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.battlefield none)) Semantics.ObservationPoint.arrival none [])) [] none [] none none (some (Semantics.Condition.compareAmt (Semantics.Amount.letter Semantics.Letter.x) Semantics.Comparator.atLeast (Semantics.Amount.lit 5))) (Semantics.Action.draw (Semantics.Amount.lit 1)))])
semantic_definition reach : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "reach") [] [])
semantic_definition readAhead : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "readAhead") [] [(Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.cant (Semantics.Subject.actor (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.and [(Semantics.Predicate.and [(Semantics.Predicate.abilityOf (Semantics.NounPhrase.asType Semantics.CardType.enchantment Semantics.NounPhrase.this (some (Semantics.Subtype.of Semantics.CardType.enchantment "Saga")))), (Semantics.Predicate.compare [Semantics.ProjAxis.chapter] Semantics.Comparator.atLeast (Semantics.Amount.lit 1))]), (Semantics.Predicate.not (Semantics.Predicate.compare [Semantics.ProjAxis.chapter] Semantics.Comparator.eq (Semantics.Amount.statOf (Semantics.ProjAxis.counter (Semantics.CounterKind.named "loreCounter")) (Semantics.NounPhrase.asType Semantics.CardType.enchantment Semantics.NounPhrase.this (some (Semantics.Subtype.of Semantics.CardType.enchantment "Saga"))))))]))) (Semantics.Templates.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.trigger) Semantics.NounPhrase.any) ({} : Semantics.DeonticMods)) (Semantics.Condition.did (Semantics.DidClause.mk Semantics.Subject.any (Semantics.Action.move (Semantics.NounPhrase.asType Semantics.CardType.enchantment Semantics.NounPhrase.this (some (Semantics.Subtype.of Semantics.CardType.enchantment "Saga"))) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.battlefield none)) Semantics.ObservationPoint.arrival none []) (Semantics.LookbackSpan.duration (Semantics.Duration.between (some (Semantics.GameEvent.beginningOf Semantics.TurnPart.turn none Semantics.TurnRef.current)) (some Semantics.GameEvent.now)) none))) Semantics.CondMarking.asLongAs))])
semantic_definition rebound : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "rebound") [] [(Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.instead (Semantics.WouldClause.would Semantics.Subject.any (Semantics.Action.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) ((Semantics.Location.zone Semantics.Zone.stack none)) (some (Semantics.Location.zone Semantics.Zone.graveyard none)) Semantics.ObservationPoint.departure none [])) (Semantics.Action.sequentially [(Semantics.Action.ownScope (some (Semantics.Deed.action "exile")) (Semantics.Action.enact (Semantics.Deed.action "exile") (Semantics.Action.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.exile none)) Semantics.ObservationPoint.arrival none []) none none)), (Semantics.Action.delay (Semantics.GameEvent.beginningOf Semantics.TurnPart.upkeep (some Semantics.NounPhrase.you) (Semantics.TurnRef.next none)) [] none (Semantics.Action.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Action.enact (Semantics.Deed.action "cast") (Semantics.Action.procedure Semantics.Procedure.cast Semantics.NounPhrase.this Semantics.PlayPayment.withoutPaying none none none) none none) none none))]) none Semantics.ReplUse.repeatedly none) (Semantics.Condition.matches (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) (Semantics.Predicate.castFrom (Semantics.Location.zone Semantics.Zone.hand (some Semantics.NounPhrase.you)))) Semantics.CondMarking.ifSo))])
semantic_definition reconfigure (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "reconfigure") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.activated (Semantics.Cost.inCaller arg0) (Semantics.Action.ownScope (some (Semantics.Deed.action "attach")) (Semantics.Action.enact (Semantics.Deed.action "attach") (Semantics.Action.attachment Semantics.AttachMove.attached (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) (some (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.described (Semantics.DetPhrase.target (Semantics.Quantity.range (some 1) (some 1))) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.otherThan (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)]))))) none none)) (some (Semantics.Timing.and [(Semantics.Timing.inPart Semantics.TurnPart.mainPhase (some Semantics.NounPhrase.you) Semantics.TurnRef.current), (Semantics.Timing.onlyIf (Semantics.Condition.not (Semantics.Condition.exists_ (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.stack none))))))])) none none none), (Semantics.Ability.activated (Semantics.Cost.inCaller arg0) (Semantics.Action.enact (Semantics.Deed.action "attach") (Semantics.Action.attachment Semantics.AttachMove.unattached (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) none) none none) (some (Semantics.Timing.and [(Semantics.Timing.inPart Semantics.TurnPart.mainPhase (some Semantics.NounPhrase.you) Semantics.TurnRef.current), (Semantics.Timing.onlyIf (Semantics.Condition.not (Semantics.Condition.exists_ (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.stack none))))))])) none (some (Semantics.Condition.matches (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.Predicate.attachment Semantics.AttachmentSide.attachment none (some (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.hasType Semantics.CardType.creature)))))) none)])
semantic_definition recover (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "recover") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Action.move (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.hasType Semantics.CardType.creature)) ((Semantics.Location.zone Semantics.Zone.battlefield none)) (some (Semantics.Location.zone Semantics.Zone.graveyard (some Semantics.NounPhrase.you))) Semantics.ObservationPoint.departure none [])) [] none [] none none none (Semantics.Action.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Action.pay (Semantics.Cost.inCaller arg0) Semantics.PayTimes.once Semantics.PaymentOutcome.paid none none) (some (Semantics.Action.enact (Semantics.Deed.core Semantics.CoreDeed.return_) (Semantics.Action.move Semantics.NounPhrase.this ((Semantics.Location.zone Semantics.Zone.graveyard (some Semantics.NounPhrase.you))) (some (Semantics.Location.zone Semantics.Zone.hand none)) Semantics.ObservationPoint.arrival none []) none none)) (some (Semantics.Action.ownScope (some (Semantics.Deed.action "exile")) (Semantics.Action.enact (Semantics.Deed.action "exile") (Semantics.Action.move (Semantics.NounPhrase.inCaller Semantics.NounPhrase.this) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.exile none)) Semantics.ObservationPoint.arrival none []) none none)))))])
semantic_definition reinforce (arg0 : Semantics.Amount) (arg1 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "reinforce") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0)), (Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg1))] [(Semantics.Ability.activated (Semantics.Cost.compound [(Semantics.Cost.inCaller arg1), (Semantics.Cost.perform (Semantics.Action.ownScope (some (Semantics.Deed.action "discard")) (Semantics.Action.enact (Semantics.Deed.action "discard") (Semantics.Action.move (Semantics.NounPhrase.inCaller Semantics.NounPhrase.this) ((Semantics.Location.zone Semantics.Zone.hand (some Semantics.NounPhrase.actor))) (some (Semantics.Location.zone Semantics.Zone.graveyard none)) Semantics.ObservationPoint.arrival none []) none none)))]) (Semantics.Action.putCounters (Semantics.Amount.inCaller arg0) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) (Semantics.NounPhrase.described (Semantics.DetPhrase.target (Semantics.Quantity.range (some 1) (some 1))) (Semantics.Predicate.hasType Semantics.CardType.creature)) Semantics.CounterBatch.one true) none none none none)])
semantic_definition replicate (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "replicate") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.addedCost (Semantics.Cost.inCaller arg0) true Semantics.PayTimes.anyNumberOfTimes)), (Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Action.procedure Semantics.Procedure.cast (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) Semantics.PlayPayment.itsOwnCost none none none)) [] none [] none none (some (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "replicate")) none) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) Semantics.Comparator.atLeast (Semantics.Amount.lit 1))) (Semantics.Action.sequentially [(Semantics.Action.copy Semantics.CopySort.fromStack (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) (Semantics.Amount.paid (Semantics.LinkedFacet.timesPaid (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "replicate"))) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) []), (Semantics.Action.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Action.chooseNewTargets (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.copy) Semantics.Plurality.many)) none none)]))])
semantic_definition retrace : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "retrace") [] [(Semantics.Ability.static (Semantics.StaticSpec.can (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Action.procedure Semantics.Procedure.cast Semantics.NounPhrase.this (Semantics.PlayPayment.payingInstead (Semantics.Cost.compound [Semantics.Cost.itsManaCost, (Semantics.Cost.perform (Semantics.Action.ownScope (some (Semantics.Deed.action "discard")) (Semantics.Action.enact (Semantics.Deed.action "discard") (Semantics.Action.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [Semantics.Predicate.isCard, (Semantics.Predicate.hasType Semantics.CardType.land), (Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.hand (some Semantics.NounPhrase.you)))]))) ((Semantics.Location.zone Semantics.Zone.hand (some Semantics.NounPhrase.actor))) (some (Semantics.Location.zone Semantics.Zone.graveyard none)) Semantics.ObservationPoint.arrival none []) none none)))])) none (some (Semantics.Location.zone Semantics.Zone.graveyard (some Semantics.NounPhrase.you))) none) ({} : Semantics.DeonticMods)))])
semantic_definition riot : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "riot") [] [(Semantics.Ability.static (Semantics.StaticSpec.asEvent (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Action.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.battlefield none)) Semantics.ObservationPoint.arrival none [])) (Semantics.Action.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Action.establish (Semantics.StaticSpec.entryRider (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.MoveRider.withCounters (Semantics.Amount.lit 1) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) Semantics.EntryCounterMark.additional)) none) none (some (Semantics.Action.establish (Semantics.StaticSpec.abilityGrant (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.Ability.keyword (Semantics.KeywordLabel.named "haste") [] [])) none)))))])
semantic_definition saddle (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "saddle") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0))] [(Semantics.Ability.activated (Semantics.Cost.perform (Semantics.Action.ownScope (some (Semantics.Deed.action "tap")) (Semantics.Action.enact (Semantics.Deed.action "tap") (Semantics.Action.setStatus Semantics.Status.tapped (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.someOf (Semantics.SliceCount.totalling (Semantics.ProjAxis.stat Semantics.Stat.power) Semantics.Comparator.atLeast (Semantics.Amount.inCaller arg0)) none (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.and [(Semantics.Predicate.hasStatus Semantics.Status.untapped), (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you), (Semantics.Predicate.otherThan (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this))])]))))) none none))) (Semantics.Action.gainDesignation (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.DesignationLabel.named "saddled") (some (Semantics.Duration.between none (some (Semantics.GameEvent.endOf Semantics.TurnPart.turn none Semantics.TurnRef.current))))) (some (Semantics.Timing.and [(Semantics.Timing.inPart Semantics.TurnPart.mainPhase (some Semantics.NounPhrase.you) Semantics.TurnRef.current), (Semantics.Timing.onlyIf (Semantics.Condition.not (Semantics.Condition.exists_ (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.stack none))))))])) none none none)])
semantic_definition scavenge (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "scavenge") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.activated (Semantics.Cost.compound [(Semantics.Cost.inCaller arg0), (Semantics.Cost.perform (Semantics.Action.enact (Semantics.Deed.action "exile") (Semantics.Action.move Semantics.NounPhrase.this ((Semantics.Location.zone Semantics.Zone.graveyard (some Semantics.NounPhrase.you))) (some (Semantics.Location.zone Semantics.Zone.exile none)) Semantics.ObservationPoint.arrival none []) none none))]) (Semantics.Action.putCounters (Semantics.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.power) (Semantics.NounPhrase.pro (Semantics.Reach.verbed (Semantics.Deed.action "exile") (some Semantics.NounWord.card) Semantics.VerbedMarking.attributive) Semantics.Plurality.one)) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) (Semantics.NounPhrase.described (Semantics.DetPhrase.target (Semantics.Quantity.range (some 1) (some 1))) (Semantics.Predicate.hasType Semantics.CardType.creature)) Semantics.CounterBatch.one true) (some (Semantics.Timing.and [(Semantics.Timing.inPart Semantics.TurnPart.mainPhase (some Semantics.NounPhrase.you) Semantics.TurnRef.current), (Semantics.Timing.onlyIf (Semantics.Condition.not (Semantics.Condition.exists_ (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.stack none))))))])) none none none)])
semantic_definition shadow : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "shadow") [] [(Semantics.Ability.static (Semantics.StaticSpec.cant (Semantics.Subject.by (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.not (Semantics.Predicate.hasKeyword (Semantics.KeywordTerm.the (Semantics.KeywordLabel.named "shadow"))))]))) (Semantics.Templates.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.block) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) ({} : Semantics.DeonticMods))), (Semantics.Ability.static (Semantics.StaticSpec.cant (Semantics.Subject.by (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasKeyword (Semantics.KeywordTerm.the (Semantics.KeywordLabel.named "shadow")))]))) (Semantics.Templates.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.block) (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.not (Semantics.Predicate.hasKeyword (Semantics.KeywordTerm.the (Semantics.KeywordLabel.named "shadow"))))]))) ({} : Semantics.DeonticMods)))])
semantic_definition shroud : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "shroud") [] [(Semantics.Ability.static (Semantics.StaticSpec.cant Semantics.Subject.any (Semantics.Templates.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.target) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) ({} : Semantics.DeonticMods)))])
semantic_definition skulk : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "skulk") [] [(Semantics.Ability.static (Semantics.StaticSpec.cant (Semantics.Subject.by (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.compare [(Semantics.ProjAxis.stat Semantics.Stat.power)] Semantics.Comparator.greater (Semantics.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.power) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)))]))) (Semantics.Templates.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.block) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) ({} : Semantics.DeonticMods)))])
semantic_definition sneak (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "sneak") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.can (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Action.procedure Semantics.Procedure.cast Semantics.NounPhrase.this (Semantics.PlayPayment.payingInstead (Semantics.Cost.compound [(Semantics.Cost.inCaller arg0), (Semantics.Cost.perform (Semantics.Action.enact (Semantics.Deed.core Semantics.CoreDeed.return_) (Semantics.Action.move (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.not (Semantics.Predicate.inCombat Semantics.CombatRelation.blockedBy none)), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)])) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.hand none)) Semantics.ObservationPoint.arrival none []) none none))])) none none none) ({ timing := (some (Semantics.Timing.and [(Semantics.Timing.onlyIf (Semantics.Condition.hasPriority Semantics.NounPhrase.you)), (Semantics.Timing.inPart Semantics.TurnPart.declareBlockers (some Semantics.NounPhrase.you) Semantics.TurnRef.current)])) } : Semantics.DeonticMods)))])
semantic_definition solved (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "solved") [(Semantics.KeywordParam.ability arg0)] [(Semantics.Ability.qualified arg0 none none (some (Semantics.Condition.matches (Semantics.NounPhrase.asType Semantics.CardType.enchantment Semantics.NounPhrase.this (some (Semantics.Subtype.of Semantics.CardType.enchantment "Case"))) (Semantics.Predicate.hasDesignation (Semantics.DesignationLabel.named "solved") none))) none)])
semantic_definition soulbond : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "soulbond") [] [(Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Action.move (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.battlefield none)) Semantics.ObservationPoint.arrival none [])) [] none [] none none (some (Semantics.Condition.and [(Semantics.Condition.matches (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none) (Semantics.Predicate.and [(Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you), (Semantics.Predicate.not (Semantics.Predicate.related (Semantics.RelationLabel.named "paired") none))])), (Semantics.Condition.exists_ (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.and [(Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you), (Semantics.Predicate.otherThan (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none))]), (Semantics.Predicate.not (Semantics.Predicate.related (Semantics.RelationLabel.named "paired") none))])))])) (Semantics.Action.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Action.relate (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none) (Semantics.RelationLabel.named "paired") (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you), (Semantics.Predicate.otherThan (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none))]), (Semantics.Predicate.not (Semantics.Predicate.related (Semantics.RelationLabel.named "paired") none))]))) none none)), (Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Action.move (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you), (Semantics.Predicate.otherThan (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none))])) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.battlefield none)) Semantics.ObservationPoint.arrival none [])) [] none [] none none (some (Semantics.Condition.and [(Semantics.Condition.matches (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none) (Semantics.Predicate.and [(Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you), (Semantics.Predicate.not (Semantics.Predicate.related (Semantics.RelationLabel.named "paired") none))])), (Semantics.Condition.matches (Semantics.NounPhrase.pro (Semantics.Reach.word (Semantics.NounWord.type Semantics.CardType.creature)) Semantics.Plurality.one) (Semantics.Predicate.and [(Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you), (Semantics.Predicate.not (Semantics.Predicate.related (Semantics.RelationLabel.named "paired") none))]))])) (Semantics.Action.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Action.relate (Semantics.NounPhrase.pro (Semantics.Reach.word (Semantics.NounWord.type Semantics.CardType.creature)) Semantics.Plurality.one) (Semantics.RelationLabel.named "paired") (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) none none))])
semantic_definition soulshift (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "soulshift") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0))] [(Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Action.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) ((Semantics.Location.zone Semantics.Zone.battlefield none)) (some (Semantics.Location.zone Semantics.Zone.graveyard none)) Semantics.ObservationPoint.departure none [])) [] none [] none none none (Semantics.Action.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Action.enact (Semantics.Deed.core Semantics.CoreDeed.return_) (Semantics.Action.move (Semantics.NounPhrase.described (Semantics.DetPhrase.target (Semantics.Quantity.range (some 1) (some 1))) (Semantics.Predicate.and [Semantics.Predicate.isCard, (Semantics.Predicate.hasSubtype (Semantics.Subtype.of Semantics.CardType.creature "Spirit")), (Semantics.Predicate.compare [(Semantics.ProjAxis.stat Semantics.Stat.manaValue)] Semantics.Comparator.atMost (Semantics.Amount.inCaller arg0)), (Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.graveyard (some Semantics.NounPhrase.you)))])) ((Semantics.Location.zone Semantics.Zone.graveyard (some Semantics.NounPhrase.you))) (some (Semantics.Location.zone Semantics.Zone.hand none)) Semantics.ObservationPoint.arrival none []) none none) none none))])
semantic_definition spectacle (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "spectacle") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.can (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Action.procedure Semantics.Procedure.cast Semantics.NounPhrase.this (Semantics.PlayPayment.payingInstead (Semantics.Cost.inCaller arg0)) none none none) ({} : Semantics.DeonticMods)) (Semantics.Condition.did (Semantics.DidClause.mk (Semantics.Subject.actor (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.isOpponentOf Semantics.NounPhrase.you))) (Semantics.Action.changeLife (Semantics.Delta.down Semantics.Amount.anyAmount)) (Semantics.LookbackSpan.duration (Semantics.Duration.between (some (Semantics.GameEvent.beginningOf Semantics.TurnPart.turn none Semantics.TurnRef.current)) (some Semantics.GameEvent.now)) none))) Semantics.CondMarking.asLongAs))])
semantic_definition splice_ (arg0 : Semantics.Predicate) (arg1 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "splice") [(Semantics.KeywordParam.quality (Semantics.Predicate.inCaller arg0)), (Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg1))] [(Semantics.Ability.static (Semantics.StaticSpec.instead (Semantics.WouldClause.would (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Action.procedure Semantics.Procedure.cast (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.and [(Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.stack none)), (Semantics.Predicate.not (Semantics.Predicate.abilityHead Semantics.AbilityClass.anyOnStack))]), (Semantics.Predicate.inCaller arg0)])) Semantics.PlayPayment.itsOwnCost none none none)) (Semantics.Action.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Action.expose Semantics.ExposeVerb.reveal (Semantics.Exposed.cards Semantics.NounPhrase.this)) (some (Semantics.Action.establish (Semantics.StaticSpec.conjunction none [(Semantics.StaticSpec.addedCost (Semantics.Cost.inCaller arg1) false Semantics.PayTimes.once), (Semantics.StaticSpec.textChange (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.spell) Semantics.Plurality.one) (Semantics.TextChange.appendTextOf Semantics.NounPhrase.this))]) none)) none) none Semantics.ReplUse.repeatedly none))])
semantic_definition splitSecond : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "splitSecond") [] [(Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.conjunction none [(Semantics.StaticSpec.cant (Semantics.Subject.actor (Semantics.NounPhrase.described Semantics.DetPhrase.all Semantics.Predicate.anyPlayer)) (Semantics.Action.procedure Semantics.Procedure.cast (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.and [(Semantics.Predicate.and [(Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.stack none)), (Semantics.Predicate.not (Semantics.Predicate.abilityHead Semantics.AbilityClass.anyOnStack))]), (Semantics.Predicate.otherThan (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this))])) Semantics.PlayPayment.itsOwnCost none none none) ({} : Semantics.DeonticMods)), (Semantics.StaticSpec.cant (Semantics.Subject.actor (Semantics.NounPhrase.described Semantics.DetPhrase.all Semantics.Predicate.anyPlayer)) (Semantics.Action.procedure Semantics.Procedure.activate (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.and [(Semantics.Predicate.abilityHead Semantics.AbilityClass.anyActivated), (Semantics.Predicate.not Semantics.Predicate.isManaAbility)])) Semantics.PlayPayment.itsOwnCost none none none) ({} : Semantics.DeonticMods))]) (Semantics.Condition.matches (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) (Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.stack none))) Semantics.CondMarking.asLongAs))])
semantic_definition spree : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "spree") [] [(Semantics.Ability.static (Semantics.StaticSpec.modeChoice (Semantics.ModeChange.setQuantity (Semantics.Quantity.range (some 1) none))))])
semantic_definition squad (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "squad") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.addedCost (Semantics.Cost.inCaller arg0) true Semantics.PayTimes.anyNumberOfTimes)), (Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Action.move (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.battlefield none)) Semantics.ObservationPoint.arrival none [])) [] none [] none none (some (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "squad")) none) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) Semantics.Comparator.atLeast (Semantics.Amount.lit 1))) (Semantics.Action.createObject (Semantics.Amount.paid (Semantics.LinkedFacet.timesPaid (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "squad"))) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) (Semantics.CreationSpec.token (Semantics.TokenSpec.copyOf (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none) []) []) true none))])
semantic_definition startYourEngines : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "startYourEngines") [] [])
semantic_definition station : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "station") [] [(Semantics.Ability.activated (Semantics.Cost.perform (Semantics.Action.enact (Semantics.Deed.action "tap") (Semantics.Action.setStatus Semantics.Status.tapped (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.otherThan (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)), (Semantics.Predicate.hasStatus Semantics.Status.untapped), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)]))) none none)) (Semantics.Action.putCounters (Semantics.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.power) (Semantics.NounPhrase.pro (Semantics.Reach.verbed (Semantics.Deed.action "tap") (some (Semantics.NounWord.type Semantics.CardType.creature)) Semantics.VerbedMarking.attributive) Semantics.Plurality.one)) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "chargeCounter")) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.CounterBatch.one true) (some (Semantics.Timing.and [(Semantics.Timing.inPart Semantics.TurnPart.mainPhase (some Semantics.NounPhrase.you) Semantics.TurnRef.current), (Semantics.Timing.onlyIf (Semantics.Condition.not (Semantics.Condition.exists_ (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.stack none))))))])) none none none)])
semantic_definition storied : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "storied") [] [(Semantics.Ability.static (Semantics.StaticSpec.whileHolds (Semantics.Condition.and [(Semantics.Condition.compareAmt (Semantics.Amount.countOf (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.and [(Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.battlefield none)), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you), (Semantics.Predicate.or [(Semantics.Predicate.hasType Semantics.CardType.artifact), (Semantics.Predicate.hasSubtype (Semantics.Subtype.of Semantics.CardType.enchantment "Saga")), (Semantics.Predicate.hasSupertype Semantics.Supertype.legendary)])]))) Semantics.Comparator.atLeast (Semantics.Amount.lit 3)), (Semantics.Condition.not (Semantics.Condition.matches Semantics.NounPhrase.you (Semantics.Predicate.hasDesignation (Semantics.DesignationLabel.named "enduringStory") none)))]) (Semantics.Action.gainDesignation Semantics.NounPhrase.you (Semantics.DesignationLabel.named "enduringStory") (some (Semantics.Duration.between none none)))))])
semantic_definition sunburst : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "sunburst") [] [(Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.entryRider (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.MoveRider.withCounters (Semantics.Amount.paid Semantics.LinkedFacet.colorsSpent (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) Semantics.EntryCounterMark.fresh)) (Semantics.Condition.matches (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.Predicate.hasType Semantics.CardType.creature)) Semantics.CondMarking.asLongAs)), (Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.entryRider (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.MoveRider.withCounters (Semantics.Amount.paid Semantics.LinkedFacet.colorsSpent (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "chargeCounter")) Semantics.EntryCounterMark.fresh)) (Semantics.Condition.not (Semantics.Condition.matches (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.Predicate.hasType Semantics.CardType.creature))) Semantics.CondMarking.asLongAs))])
semantic_definition surge (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "surge") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.can (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Action.procedure Semantics.Procedure.cast Semantics.NounPhrase.this (Semantics.PlayPayment.payingInstead (Semantics.Cost.inCaller arg0)) none none none) ({} : Semantics.DeonticMods)) (Semantics.Condition.did (Semantics.DidClause.mk (Semantics.Subject.actor (Semantics.NounPhrase.or [Semantics.NounPhrase.you, (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.isTeammateOf Semantics.NounPhrase.you))])) (Semantics.Action.procedure Semantics.Procedure.cast (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.and [(Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.stack none)), (Semantics.Predicate.not (Semantics.Predicate.abilityHead Semantics.AbilityClass.anyOnStack))]), (Semantics.Predicate.otherThan (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this))])) Semantics.PlayPayment.itsOwnCost none none none) (Semantics.LookbackSpan.duration (Semantics.Duration.between (some (Semantics.GameEvent.beginningOf Semantics.TurnPart.turn none Semantics.TurnRef.current)) (some Semantics.GameEvent.now)) none))) Semantics.CondMarking.asLongAs))])
semantic_definition suspend (arg0 : Semantics.Amount) (arg1 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "suspend") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0)), (Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg1))] [])
semantic_definition teamwork (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "teamwork") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.addedCost (Semantics.Cost.perform (Semantics.Action.ownScope (some (Semantics.Deed.action "tap")) (Semantics.Action.enact (Semantics.Deed.action "tap") (Semantics.Action.setStatus Semantics.Status.tapped (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.someOf (Semantics.SliceCount.totalling (Semantics.ProjAxis.stat Semantics.Stat.power) Semantics.Comparator.atLeast (Semantics.Amount.inCaller arg0)) none (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)]))))) none none))) true Semantics.PayTimes.once))])
semantic_definition tiered : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "tiered") [] [(Semantics.Ability.static (Semantics.StaticSpec.modeChoice (Semantics.ModeChange.setQuantity (Semantics.Quantity.range (some 1) (some 1)))))])
semantic_definition toxic (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "toxic") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0))] [])
semantic_definition training : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "training") [] [(Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor (Semantics.NounPhrase.and [(Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none), (Semantics.NounPhrase.described (Semantics.DetPhrase.count (Semantics.Quantity.range (some 1) none) none) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.otherThan (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)), (Semantics.Predicate.compare [(Semantics.ProjAxis.stat Semantics.Stat.power)] Semantics.Comparator.greater (Semantics.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.power) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)))]))])) (Semantics.Action.attack none)) [] none [] none none none (Semantics.Action.putCounters (Semantics.Amount.lit 1) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none) Semantics.CounterBatch.one true))])
semantic_definition trample : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "trample") [] [])
semantic_definition transfigure (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "transfigure") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.activated (Semantics.Cost.compound [(Semantics.Cost.inCaller arg0), (Semantics.Cost.perform (Semantics.Action.ownScope (some (Semantics.Deed.action "sacrifice")) (Semantics.Action.enact (Semantics.Deed.action "sacrifice") (Semantics.Action.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) ((Semantics.Location.zone Semantics.Zone.battlefield none)) (some (Semantics.Location.zone Semantics.Zone.graveyard none)) Semantics.ObservationPoint.departure none []) none none)))]) (Semantics.Action.sequentially [(Semantics.Action.search ((Semantics.Location.zone Semantics.Zone.library (some Semantics.NounPhrase.you))) (Semantics.Quantity.range (some 1) (some 1)) (Semantics.Predicate.and [Semantics.Predicate.isCard, (Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.compare [(Semantics.ProjAxis.stat Semantics.Stat.manaValue)] Semantics.Comparator.eq (Semantics.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.manaValue) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)))])), (Semantics.Action.move (Semantics.NounPhrase.pro (Semantics.Reach.verbed (Semantics.Deed.action "search") none Semantics.VerbedMarking.attributive) Semantics.Plurality.one) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.battlefield none)) Semantics.ObservationPoint.arrival none []), (Semantics.Action.shuffle (Semantics.Location.zone Semantics.Zone.library (some Semantics.NounPhrase.actor)))]) (some (Semantics.Timing.and [(Semantics.Timing.inPart Semantics.TurnPart.mainPhase (some Semantics.NounPhrase.you) Semantics.TurnRef.current), (Semantics.Timing.onlyIf (Semantics.Condition.not (Semantics.Condition.exists_ (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.stack none))))))])) none none none)])
semantic_definition transmute (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "transmute") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.activated (Semantics.Cost.compound [(Semantics.Cost.inCaller arg0), (Semantics.Cost.perform (Semantics.Action.ownScope (some (Semantics.Deed.action "discard")) (Semantics.Action.enact (Semantics.Deed.action "discard") (Semantics.Action.move (Semantics.NounPhrase.inCaller Semantics.NounPhrase.this) ((Semantics.Location.zone Semantics.Zone.hand (some Semantics.NounPhrase.actor))) (some (Semantics.Location.zone Semantics.Zone.graveyard none)) Semantics.ObservationPoint.arrival none []) none none)))]) (Semantics.Action.sequentially [(Semantics.Action.search ((Semantics.Location.zone Semantics.Zone.library (some Semantics.NounPhrase.you))) (Semantics.Quantity.range (some 1) (some 1)) (Semantics.Predicate.and [Semantics.Predicate.isCard, (Semantics.Predicate.compare [(Semantics.ProjAxis.stat Semantics.Stat.manaValue)] Semantics.Comparator.eq (Semantics.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.manaValue) (Semantics.NounPhrase.pro (Semantics.Reach.verbed (Semantics.Deed.action "discard") (some Semantics.NounWord.card) Semantics.VerbedMarking.attributive) Semantics.Plurality.one)))])), (Semantics.Action.expose Semantics.ExposeVerb.reveal (Semantics.Exposed.cards (Semantics.NounPhrase.pro (Semantics.Reach.verbed (Semantics.Deed.action "search") none Semantics.VerbedMarking.attributive) Semantics.Plurality.one))), (Semantics.Action.move (Semantics.NounPhrase.pro (Semantics.Reach.verbed (Semantics.Deed.action "search") none Semantics.VerbedMarking.attributive) Semantics.Plurality.one) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.hand none)) Semantics.ObservationPoint.arrival none []), (Semantics.Action.shuffle (Semantics.Location.zone Semantics.Zone.library (some Semantics.NounPhrase.actor)))]) (some (Semantics.Timing.and [(Semantics.Timing.inPart Semantics.TurnPart.mainPhase (some Semantics.NounPhrase.you) Semantics.TurnRef.current), (Semantics.Timing.onlyIf (Semantics.Condition.not (Semantics.Condition.exists_ (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.stack none))))))])) none none none)])
semantic_definition tribute (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "tribute") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.asEvent (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Action.move (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.battlefield none)) Semantics.ObservationPoint.arrival none [])) (Semantics.Action.sequentially [(Semantics.Action.choose none (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.isOpponentOf Semantics.NounPhrase.you)) Semantics.Disclosure.openly none), (Semantics.Action.instruct (Semantics.NounPhrase.described Semantics.DetPhrase.the (Semantics.Predicate.chosenPlayer Semantics.ChoiceRef.theChoice)) (Semantics.Action.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Action.establish (Semantics.StaticSpec.entryRider (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none) (Semantics.MoveRider.withCounters (Semantics.Amount.inCaller arg0) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) Semantics.EntryCounterMark.additional)) none) none none))])))])
semantic_definition typecycling (arg0 : Semantics.Predicate) (arg1 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "typecycling") [(Semantics.KeywordParam.quality (Semantics.Predicate.inCaller arg0)), (Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg1))] [])
semantic_definition umbraArmor : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "umbraArmor") [] [(Semantics.Ability.static (Semantics.StaticSpec.instead (Semantics.WouldClause.would Semantics.Subject.any (Semantics.Action.enact (Semantics.Deed.action "destroy") (Semantics.Templates.deonticActionFor (Semantics.Deed.action "destroy") (Semantics.NounPhrase.attachHost Semantics.AttachWord.enchanted Semantics.NounWord.permanent)) none none)) (Semantics.Action.sequentially [(Semantics.Action.clearDamage (Semantics.NounPhrase.attachHost Semantics.AttachWord.enchanted Semantics.NounWord.permanent)), (Semantics.Action.ownScope (some (Semantics.Deed.action "destroy")) (Semantics.Action.enact (Semantics.Deed.action "destroy") (Semantics.Action.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asType Semantics.CardType.enchantment Semantics.NounPhrase.this (some (Semantics.Subtype.of Semantics.CardType.enchantment "Aura")))) ((Semantics.Location.zone Semantics.Zone.battlefield none)) (some (Semantics.Location.zone Semantics.Zone.graveyard none)) Semantics.ObservationPoint.departure none []) none none))]) none Semantics.ReplUse.repeatedly none))])
semantic_definition undaunted : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "undaunted") [] [(Semantics.Ability.static (Semantics.StaticSpec.costShift (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) (Semantics.CostShift.less (Semantics.Amount.arith Semantics.ArithOp.times (Semantics.Amount.lit 1) (Semantics.Amount.countOf (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.isOpponentOf Semantics.NounPhrase.you)))) none)))])
semantic_definition undying : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "undying") [] [(Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Action.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) ((Semantics.Location.zone Semantics.Zone.battlefield none)) (some (Semantics.Location.zone Semantics.Zone.graveyard none)) Semantics.ObservationPoint.departure none [])) [] none [] none none (some (Semantics.Condition.not (Semantics.Condition.matches (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.Predicate.hasCounters (some (Semantics.CounterKind.named "p1p1Counter")))))) (Semantics.Action.enact (Semantics.Deed.core Semantics.CoreDeed.return_) (Semantics.Action.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.battlefield none)) Semantics.ObservationPoint.arrival none [((Semantics.MoveRider.under (Semantics.NounPhrase.possessorOf Semantics.PossessorAxis.owner (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)))), ((Semantics.MoveRider.withCounters (Semantics.Amount.lit 1) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) Semantics.EntryCounterMark.fresh))]) none none))])
semantic_definition unearth (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "unearth") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.activated (Semantics.Cost.inCaller arg0) (Semantics.Action.sequentially [(Semantics.Action.enact (Semantics.Deed.core Semantics.CoreDeed.return_) (Semantics.Action.move Semantics.NounPhrase.this ((Semantics.Location.zone Semantics.Zone.graveyard (some Semantics.NounPhrase.you))) (some (Semantics.Location.zone Semantics.Zone.battlefield none)) Semantics.ObservationPoint.arrival none []) none none), (Semantics.Action.establish (Semantics.StaticSpec.abilityGrant (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.Ability.keyword (Semantics.KeywordLabel.named "haste") [] [])) none), (Semantics.Action.delay (Semantics.GameEvent.beginningOf Semantics.TurnPart.endStep none (Semantics.TurnRef.next none)) [] none (Semantics.Action.ownScope (some (Semantics.Deed.action "exile")) (Semantics.Action.enact (Semantics.Deed.action "exile") (Semantics.Action.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.exile none)) Semantics.ObservationPoint.arrival none []) none none))), (Semantics.Action.establish (Semantics.StaticSpec.instead (Semantics.WouldClause.would Semantics.Subject.any (Semantics.Action.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) ((Semantics.Location.zone Semantics.Zone.battlefield none)) none Semantics.ObservationPoint.departure none [])) (Semantics.Action.ownScope (some (Semantics.Deed.action "exile")) (Semantics.Action.enact (Semantics.Deed.action "exile") (Semantics.Action.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.exile none)) Semantics.ObservationPoint.arrival none []) none none)) none Semantics.ReplUse.repeatedly none) none)]) (some (Semantics.Timing.and [(Semantics.Timing.inPart Semantics.TurnPart.mainPhase (some Semantics.NounPhrase.you) Semantics.TurnRef.current), (Semantics.Timing.onlyIf (Semantics.Condition.not (Semantics.Condition.exists_ (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.stack none))))))])) none none none)])
semantic_definition unleash : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "unleash") [] [(Semantics.Ability.static (Semantics.StaticSpec.asEvent (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Action.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.battlefield none)) Semantics.ObservationPoint.arrival none [])) (Semantics.Action.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Action.establish (Semantics.StaticSpec.entryRider (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.MoveRider.withCounters (Semantics.Amount.lit 1) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) Semantics.EntryCounterMark.additional)) none) none none))), (Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.cant (Semantics.Subject.actor (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) (Semantics.Templates.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.block) Semantics.NounPhrase.any) ({} : Semantics.DeonticMods)) (Semantics.Condition.matches (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.Predicate.hasCounters (some (Semantics.CounterKind.named "p1p1Counter")))) Semantics.CondMarking.asLongAs))])
semantic_definition vanishing (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "vanishing") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.entryRider (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.MoveRider.withCounters (Semantics.Amount.inCaller arg0) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "timeCounter")) Semantics.EntryCounterMark.fresh))), (Semantics.Ability.triggered (Semantics.GameEvent.beginningOf Semantics.TurnPart.upkeep (some Semantics.NounPhrase.you) (Semantics.TurnRef.each none)) [] none [] none none (some (Semantics.Condition.matches (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.Predicate.hasCounters (some (Semantics.CounterKind.named "timeCounter"))))) (Semantics.Action.removeCounters (some (Semantics.Quantity.range (some 1) (some 1))) (some (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "timeCounter"))) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.CounterBatch.one true)), (Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Action.removeCounters none (some (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "timeCounter"))) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.CounterBatch.emptying false)) [] none [] none none none (Semantics.Action.ownScope (some (Semantics.Deed.action "sacrifice")) (Semantics.Action.enact (Semantics.Deed.action "sacrifice") (Semantics.Action.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) ((Semantics.Location.zone Semantics.Zone.battlefield none)) (some (Semantics.Location.zone Semantics.Zone.graveyard none)) Semantics.ObservationPoint.departure none []) none none)))])
semantic_definition vigilance : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "vigilance") [] [])
semantic_definition ward (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "ward") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.by (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.stack none)), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.isOpponentOf Semantics.NounPhrase.you)))]))) (Semantics.Action.target (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this))) [] none [] none none none (Semantics.Action.instruct (Semantics.NounPhrase.possessorOf Semantics.PossessorAxis.controller (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.stack) Semantics.Plurality.one)) (Semantics.Action.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Action.pay (Semantics.Cost.inCaller arg0) Semantics.PayTimes.once Semantics.PaymentOutcome.paid none none) none (some (Semantics.Action.instruct Semantics.NounPhrase.you (Semantics.Action.ownScope (some (Semantics.Deed.action "counter")) (Semantics.Action.enact (Semantics.Deed.action "counter") (Semantics.Action.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.stack) Semantics.Plurality.one)) ((Semantics.Location.zone Semantics.Zone.stack none)) (some (Semantics.Location.zone Semantics.Zone.graveyard none)) Semantics.ObservationPoint.arrival none []) none none)))))))])
semantic_definition warp (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "warp") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.can (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Action.procedure Semantics.Procedure.cast Semantics.NounPhrase.this (Semantics.PlayPayment.payingInstead (Semantics.Cost.inCaller arg0)) none (some (Semantics.Location.zone Semantics.Zone.hand (some Semantics.NounPhrase.you))) none) ({} : Semantics.DeonticMods))), (Semantics.Ability.static (Semantics.StaticSpec.asEvent (Semantics.GameEvent.does (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Action.pay Semantics.Cost.itsManaCost Semantics.PayTimes.once Semantics.PaymentOutcome.paid (some (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) (some (Semantics.KeywordLabel.named "warp")))) (Semantics.Action.delay (Semantics.GameEvent.beginningOf Semantics.TurnPart.endStep none (Semantics.TurnRef.next none)) [] none (Semantics.Action.sequentially [(Semantics.Action.ownScope (some (Semantics.Deed.action "exile")) (Semantics.Action.enact (Semantics.Deed.action "exile") (Semantics.Action.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.becomes (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this))) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.exile none)) Semantics.ObservationPoint.arrival none []) none none)), (Semantics.Action.establish (Semantics.StaticSpec.can (Semantics.Subject.actor (Semantics.NounPhrase.possessorOf Semantics.PossessorAxis.owner (Semantics.NounPhrase.pro Semantics.Reach.bare Semantics.Plurality.one))) (Semantics.Action.procedure Semantics.Procedure.cast (Semantics.NounPhrase.pro Semantics.Reach.bare Semantics.Plurality.one) Semantics.PlayPayment.itsOwnCost none (some (Semantics.Location.zone Semantics.Zone.exile none)) none) ({} : Semantics.DeonticMods)) (some (Semantics.Duration.between (some (Semantics.GameEvent.endOf Semantics.TurnPart.turn none Semantics.TurnRef.current)) none)))]))))])
semantic_definition webSlinging (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "webSlinging") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.can (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Action.procedure Semantics.Procedure.cast Semantics.NounPhrase.this (Semantics.PlayPayment.payingInstead (Semantics.Cost.compound [(Semantics.Cost.inCaller arg0), (Semantics.Cost.perform (Semantics.Action.enact (Semantics.Deed.core Semantics.CoreDeed.return_) (Semantics.Action.move (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasStatus Semantics.Status.tapped), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)])) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.hand none)) Semantics.ObservationPoint.arrival none []) none none))])) none none none) ({} : Semantics.DeonticMods)))])
semantic_definition wither : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "wither") [] [])
/-! ## Keyword actions -/

semantic_definition activate (arg0 : Semantics.NounPhrase) : Semantics.Action :=
  (Semantics.Action.ownScope (some (Semantics.Deed.action "activate")) (Semantics.Action.enact (Semantics.Deed.action "activate") (Semantics.Action.procedure Semantics.Procedure.activate (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "activate.arg0" arg0)) Semantics.PlayPayment.itsOwnCost none none none) none none))
semantic_definition adapt (arg0 : Semantics.Amount) : Semantics.Action :=
  (Semantics.Action.ownScope (some (Semantics.Deed.action "adapt")) (Semantics.Action.enact (Semantics.Deed.action "adapt") (Semantics.Action.doIf (Semantics.Condition.not (Semantics.Condition.matches (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.Predicate.hasCounters (some (Semantics.CounterKind.named "p1p1Counter"))))) (Semantics.Action.putCounters (Semantics.Amount.inCaller arg0) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.CounterBatch.one true) none) none none))
semantic_definition attach (arg0 : Semantics.NounPhrase) (arg1 : Semantics.NounPhrase) : Semantics.Action :=
  (Semantics.Action.ownScope (some (Semantics.Deed.action "attach")) (Semantics.Action.enact (Semantics.Deed.action "attach") (Semantics.Action.attachment Semantics.AttachMove.attached (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "attach.arg0" arg0)) (some (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "attach.arg1" arg1)))) none none))
semantic_definition behold (arg0 : Semantics.Predicate) : Semantics.Action :=
  (Semantics.Action.ownScope (some (Semantics.Deed.action "behold")) (Semantics.Action.enact (Semantics.Deed.action "behold") (Semantics.Action.chooseModes (Semantics.Quantity.range (some 1) (some 1)) [(none, none, (Semantics.Action.expose Semantics.ExposeVerb.reveal (Semantics.Exposed.cards (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [Semantics.Predicate.isCard, (Semantics.Predicate.inCaller arg0), (Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.hand (some Semantics.NounPhrase.actor)))]))))), (none, none, (Semantics.Action.choose none (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.inCaller arg0), (Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.battlefield none)), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.actor)])) Semantics.Disclosure.openly none))]) none none))
semantic_definition blight (arg0 : Semantics.Amount) : Semantics.Action :=
  (Semantics.Action.ownScope (some (Semantics.Deed.action "blight")) (Semantics.Action.enact (Semantics.Deed.action "blight") (Semantics.Action.putCounters (Semantics.Amount.inCaller arg0) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "m1m1Counter")) (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.actor)])) Semantics.CounterBatch.one true) none none))
semantic_definition bolster (arg0 : Semantics.Amount) : Semantics.Action :=
  (Semantics.Action.ownScope (some (Semantics.Deed.action "bolster")) (Semantics.Action.enact (Semantics.Deed.action "bolster") (Semantics.Action.sequentially [(Semantics.Action.choose none (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.actor), (Semantics.Predicate.superlative Semantics.AggregateOp.min (Semantics.ProjAxis.stat Semantics.Stat.toughness) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.actor)]))])) Semantics.Disclosure.openly none), (Semantics.Action.putCounters (Semantics.Amount.inCaller arg0) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) (Semantics.NounPhrase.pro (Semantics.Reach.word (Semantics.NounWord.type Semantics.CardType.creature)) Semantics.Plurality.one) Semantics.CounterBatch.one true)]) none none))
semantic_definition cast (arg0 : Semantics.NounPhrase) : Semantics.Action :=
  (Semantics.Action.ownScope (some (Semantics.Deed.action "cast")) (Semantics.Action.enact (Semantics.Deed.action "cast") (Semantics.Action.procedure Semantics.Procedure.cast (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "cast.arg0" arg0)) Semantics.PlayPayment.itsOwnCost none none none) none none))
semantic_definition clash : Semantics.Action :=
  (Semantics.Action.ownScope (some (Semantics.Deed.action "clash")) (Semantics.Action.enact (Semantics.Deed.action "clash") (Semantics.Action.sequentially [(Semantics.Action.expose Semantics.ExposeVerb.reveal (Semantics.Exposed.cards (Semantics.NounPhrase.slice Semantics.PileEnd.top (Semantics.Amount.lit 1) (Semantics.Location.zone Semantics.Zone.library (some Semantics.NounPhrase.actor))))), (Semantics.Action.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Action.move (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.card) Semantics.Plurality.one) Semantics.Location.anyLocation (some (Semantics.Location.fromEnd Semantics.PileEnd.bottom (Semantics.Ordinal.nth 1) (Semantics.Location.zone Semantics.Zone.library none))) Semantics.ObservationPoint.arrival none []) none none)]) none none))
semantic_definition conniveN (arg0 : Semantics.Amount) : Semantics.Action :=
  (Semantics.Definitions.connive arg0)
semantic_definition convert (arg0 : Semantics.NounPhrase) : Semantics.Action :=
  (Semantics.Action.ownScope (some (Semantics.Deed.action "convert")) (Semantics.Action.enact (Semantics.Deed.action "convert") (Semantics.Action.turnOver (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "convert.arg0" arg0))) none none))
semantic_definition counter (arg0 : Semantics.NounPhrase) : Semantics.Action :=
  (Semantics.Action.ownScope (some (Semantics.Deed.action "counter")) (Semantics.Action.enact (Semantics.Deed.action "counter") (Semantics.Action.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "counter.arg0" arg0)) ((Semantics.Location.zone Semantics.Zone.stack none)) (some (Semantics.Location.zone Semantics.Zone.graveyard none)) Semantics.ObservationPoint.arrival none []) none none))
semantic_definition detain (arg0 : Semantics.NounPhrase) : Semantics.Action :=
  (Semantics.Action.ownScope (some (Semantics.Deed.action "detain")) (Semantics.Action.enact (Semantics.Deed.action "detain") (Semantics.Action.establish (Semantics.StaticSpec.conjunction none [(Semantics.StaticSpec.conjunction none [(Semantics.StaticSpec.cant (Semantics.Subject.actor (Semantics.NounPhrase.firstMention "detain.0" (Semantics.NounPhrase.inCaller arg0))) (Semantics.Templates.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.attack) Semantics.NounPhrase.any) ({} : Semantics.DeonticMods)), (Semantics.StaticSpec.cant (Semantics.Subject.actor (Semantics.NounPhrase.laterMention "detain.0" (Semantics.NounPhrase.inCaller arg0))) (Semantics.Templates.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.block) Semantics.NounPhrase.any) ({} : Semantics.DeonticMods))]), (Semantics.StaticSpec.cant Semantics.Subject.any (Semantics.Templates.deonticActionFor (Semantics.Deed.action "activate") (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.and [(Semantics.Predicate.abilityHead Semantics.AbilityClass.anyActivated), (Semantics.Predicate.abilityOf (Semantics.NounPhrase.laterMention "detain.0" (Semantics.NounPhrase.inCaller arg0)))]))) ({} : Semantics.DeonticMods))]) (some (Semantics.Duration.between none (some (Semantics.GameEvent.beginningOf Semantics.TurnPart.turn (some (Semantics.NounPhrase.possessorOf Semantics.PossessorAxis.controller Semantics.NounPhrase.this)) (Semantics.TurnRef.next none)))))) none none))
semantic_definition earthbend (arg0 : Semantics.Amount) : Semantics.Action :=
  (Semantics.Action.ownScope (some (Semantics.Deed.action "earthbend")) (Semantics.Action.enact (Semantics.Deed.action "earthbend") (Semantics.Action.sequentially [(Semantics.Action.establish (Semantics.StaticSpec.characteristicChange (Semantics.NounPhrase.described (Semantics.DetPhrase.target (Semantics.Quantity.range (some 1) (some 1))) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.land), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.actor)])) [(Semantics.CharacteristicEdit.typeLine Semantics.QualityOp.adds ({ supertypes := none, types := (some [Semantics.CardType.creature]), subtypes := none, retained := none } : Semantics.TypeLineChanges)), (Semantics.CharacteristicEdit.stat Semantics.QualityOp.sets Semantics.CharacteristicStat.power (Semantics.Amount.lit 0)), (Semantics.CharacteristicEdit.stat Semantics.QualityOp.sets Semantics.CharacteristicStat.toughness (Semantics.Amount.lit 0)), (Semantics.CharacteristicEdit.addedAbilities [(Semantics.Ability.keyword (Semantics.KeywordLabel.named "haste") [] [])])]) none), (Semantics.Action.putCounters (Semantics.Amount.inCaller arg0) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) (Semantics.NounPhrase.pro (Semantics.Reach.word (Semantics.NounWord.type Semantics.CardType.land)) Semantics.Plurality.one) Semantics.CounterBatch.one true), (Semantics.Action.delay (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Action.move (Semantics.NounPhrase.pro (Semantics.Reach.word (Semantics.NounWord.type Semantics.CardType.land)) Semantics.Plurality.one) ((Semantics.Location.zone Semantics.Zone.battlefield none)) (some (Semantics.Location.zone Semantics.Zone.graveyard none)) Semantics.ObservationPoint.departure none [])) [(Semantics.GameEvent.does Semantics.Subject.any (Semantics.Action.move (Semantics.NounPhrase.pro (Semantics.Reach.word (Semantics.NounWord.type Semantics.CardType.land)) Semantics.Plurality.one) ((Semantics.Location.zone Semantics.Zone.battlefield none)) (some (Semantics.Location.zone Semantics.Zone.exile none)) Semantics.ObservationPoint.departure none []))] none (Semantics.Action.enact (Semantics.Deed.core Semantics.CoreDeed.return_) (Semantics.Action.move (Semantics.NounPhrase.pro (Semantics.Reach.word (Semantics.NounWord.type Semantics.CardType.land)) Semantics.Plurality.one) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.battlefield none)) Semantics.ObservationPoint.arrival none [((Semantics.MoveRider.entersAs Semantics.Status.tapped)), ((Semantics.MoveRider.under Semantics.NounPhrase.actor))]) none none))]) none none))
semantic_definition endure (arg0 : Semantics.Amount) : Semantics.Action :=
  (Semantics.Action.ownScope (some (Semantics.Deed.action "endure")) (Semantics.Action.enact (Semantics.Deed.action "endure") (Semantics.Action.instruct (Semantics.NounPhrase.possessorOf Semantics.PossessorAxis.controller Semantics.NounPhrase.actor) (Semantics.Action.chooseModes (Semantics.Quantity.range (some 1) (some 1)) [(none, none, (Semantics.Action.putCounters (Semantics.Amount.inCaller arg0) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.permanent) Semantics.Plurality.one) Semantics.CounterBatch.one true)), (none, none, (Semantics.Action.createObject (Semantics.Amount.lit 1) (Semantics.CreationSpec.token (Semantics.TokenSpec.written ({ characteristics := ({ name := none, cost := none, colors := [Semantics.Color.white], supertypes := [], types := [Semantics.CardType.creature], subtypes := [(Semantics.Subtype.of Semantics.CardType.creature "Spirit")], text := [], power := (some (Semantics.Amount.inCaller arg0)), toughness := (some (Semantics.Amount.inCaller arg0)), loyalty := none, defense := none } : Semantics.Characteristics), qualities := [] } : Semantics.CharacteristicBundle)) []) true none))])) none none))
semantic_definition exert (arg0 : Semantics.NounPhrase) : Semantics.Action :=
  (Semantics.Action.ownScope (some (Semantics.Deed.action "exert")) (Semantics.Action.enact (Semantics.Deed.action "exert") (Semantics.Action.skipUntap (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "exert.arg0" arg0)) (Semantics.Amount.lit 1)) none none))
semantic_definition faceAVillainousChoice (arg0 : Semantics.Action) (arg1 : Semantics.Action) : Semantics.Action :=
  (Semantics.Action.ownScope (some (Semantics.Deed.action "faceAVillainousChoice")) (Semantics.Action.enact (Semantics.Deed.action "faceAVillainousChoice") (Semantics.Action.chooseModes (Semantics.Quantity.range (some 1) (some 1)) [(none, none, (Semantics.Action.inCaller arg0)), (none, none, (Semantics.Action.inCaller arg1))]) none none))
semantic_definition forage : Semantics.Action :=
  (Semantics.Action.ownScope (some (Semantics.Deed.action "forage")) (Semantics.Action.enact (Semantics.Deed.action "forage") (Semantics.Action.chooseModes (Semantics.Quantity.range (some 1) (some 1)) [(none, none, (Semantics.Action.enact (Semantics.Deed.action "exile") (Semantics.Action.move (Semantics.NounPhrase.described (Semantics.DetPhrase.count (Semantics.Quantity.range (some 3) (some 3)) none) (Semantics.Predicate.and [Semantics.Predicate.isCard, (Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.graveyard (some Semantics.NounPhrase.actor)))])) ((Semantics.Location.zone Semantics.Zone.graveyard (some Semantics.NounPhrase.actor))) (some (Semantics.Location.zone Semantics.Zone.exile none)) Semantics.ObservationPoint.arrival none []) none none)), (none, none, (Semantics.Action.ownScope (some (Semantics.Deed.action "sacrifice")) (Semantics.Action.enact (Semantics.Deed.action "sacrifice") (Semantics.Action.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.hasSubtype (Semantics.Subtype.of Semantics.CardType.artifact "Food")), (Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.battlefield none)), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.actor)]))) ((Semantics.Location.zone Semantics.Zone.battlefield none)) (some (Semantics.Location.zone Semantics.Zone.graveyard none)) Semantics.ObservationPoint.departure none []) none none)))]) none none))
semantic_definition goad (arg0 : Semantics.NounPhrase) : Semantics.Action :=
  (Semantics.Action.ownScope (some (Semantics.Deed.action "goad")) (Semantics.Action.enact (Semantics.Deed.action "goad") (Semantics.Action.gainDesignation (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "goad.arg0" arg0)) (Semantics.DesignationLabel.named "goaded") (some (Semantics.Duration.between none (some (Semantics.GameEvent.beginningOf Semantics.TurnPart.turn (some (Semantics.NounPhrase.possessorOf Semantics.PossessorAxis.controller Semantics.NounPhrase.this)) (Semantics.TurnRef.next none)))))) none none))
semantic_definition harness (arg0 : Semantics.NounPhrase) : Semantics.Action :=
  (Semantics.Action.ownScope (some (Semantics.Deed.action "harness")) (Semantics.Action.enact (Semantics.Deed.action "harness") (Semantics.Action.doIf (Semantics.Condition.not (Semantics.Condition.matches (Semantics.NounPhrase.firstMention "harness.0" (Semantics.NounPhrase.inCaller arg0)) (Semantics.Predicate.hasDesignation (Semantics.DesignationLabel.named "harnessed") none))) (Semantics.Action.gainDesignation (Semantics.NounPhrase.laterMention "harness.0" (Semantics.NounPhrase.inCaller arg0)) (Semantics.DesignationLabel.named "harnessed") none) none) none none))
semantic_definition heal (arg0 : Semantics.NounPhrase) : Semantics.Action :=
  (Semantics.Action.ownScope (some (Semantics.Deed.action "heal")) (Semantics.Action.enact (Semantics.Deed.action "heal") (Semantics.Action.clearDamage (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "heal.arg0" arg0))) none none))
semantic_definition investigate : Semantics.Action :=
  (Semantics.Action.ownScope (some (Semantics.Deed.action "investigate")) (Semantics.Action.enact (Semantics.Deed.action "investigate") (Semantics.Action.createObject (Semantics.Amount.lit 1) (Semantics.CreationSpec.token (Semantics.TokenSpec.written ({ characteristics := ({ name := none, cost := none, colors := [], supertypes := [], types := [Semantics.CardType.artifact], subtypes := [(Semantics.Subtype.of Semantics.CardType.artifact "Clue")], text := [(Semantics.Ability.activated (Semantics.Cost.compound [(Semantics.Cost.mana [(Semantics.ManaSymbol.simple (Semantics.SimpleManaSymbol.generic 2))]), (Semantics.Cost.perform (Semantics.Action.ownScope (some (Semantics.Deed.action "sacrifice")) (Semantics.Action.enact (Semantics.Deed.action "sacrifice") (Semantics.Action.move (Semantics.NounPhrase.inCaller Semantics.NounPhrase.this) ((Semantics.Location.zone Semantics.Zone.battlefield none)) (some (Semantics.Location.zone Semantics.Zone.graveyard none)) Semantics.ObservationPoint.departure none []) none none)))]) (Semantics.Action.draw (Semantics.Amount.lit 1)) none none none none)], power := none, toughness := none, loyalty := none, defense := none } : Semantics.Characteristics), qualities := [] } : Semantics.CharacteristicBundle)) []) true none) none none))
semantic_definition meld (arg0 : Semantics.NounPhrase) (arg1 : String) : Semantics.Action :=
  (Semantics.Action.ownScope (some (Semantics.Deed.action "meld")) (Semantics.Action.enact (Semantics.Deed.action "meld") (Semantics.Action.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "meld.arg0" arg0)) Semantics.Location.anyLocation (some (Semantics.Location.zone Semantics.Zone.battlefield none)) Semantics.ObservationPoint.arrival none [((Semantics.MoveRider.entersMelded arg1))]) none none))
semantic_definition monstrosity (arg0 : Semantics.Amount) : Semantics.Action :=
  (Semantics.Action.ownScope (some (Semantics.Deed.action "monstrosity")) (Semantics.Action.enact (Semantics.Deed.action "monstrosity") (Semantics.Action.doIf (Semantics.Condition.not (Semantics.Condition.matches (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.Predicate.hasDesignation (Semantics.DesignationLabel.named "monstrous") none))) (Semantics.Action.sequentially [(Semantics.Action.putCounters (Semantics.Amount.inCaller arg0) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.CounterBatch.one true), (Semantics.Action.gainDesignation (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.DesignationLabel.named "monstrous") none)]) none) none none))
semantic_definition play (arg0 : Semantics.NounPhrase) : Semantics.Action :=
  (Semantics.Action.ownScope (some (Semantics.Deed.action "play")) (Semantics.Action.enact (Semantics.Deed.action "play") (Semantics.Action.procedure Semantics.Procedure.play (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "play.arg0" arg0)) Semantics.PlayPayment.itsOwnCost none none none) none none))
semantic_definition populate : Semantics.Action :=
  (Semantics.Action.ownScope (some (Semantics.Deed.action "populate")) (Semantics.Action.enact (Semantics.Deed.action "populate") (Semantics.Action.sequentially [(Semantics.Action.choose none (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [Semantics.Predicate.isToken, (Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.battlefield none)), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.actor)])) Semantics.Disclosure.openly none), (Semantics.Action.createObject (Semantics.Amount.lit 1) (Semantics.CreationSpec.token (Semantics.TokenSpec.copyOf (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.token) Semantics.Plurality.one) []) []) true none)]) none none))
semantic_definition recruit : Semantics.Action :=
  (Semantics.Action.ownScope (some (Semantics.Deed.action "recruit")) (Semantics.Action.enact (Semantics.Deed.action "recruit") (Semantics.Action.sequentially [(Semantics.Action.draw (Semantics.Amount.lit 1)), (Semantics.Action.ownScope (some (Semantics.Deed.action "discard")) (Semantics.Action.enact (Semantics.Deed.action "discard") (Semantics.Action.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [Semantics.Predicate.isCard, (Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.hand (some Semantics.NounPhrase.actor)))]))) ((Semantics.Location.zone Semantics.Zone.hand (some Semantics.NounPhrase.actor))) (some (Semantics.Location.zone Semantics.Zone.graveyard none)) Semantics.ObservationPoint.arrival none []) none none)), (Semantics.Action.doIf (Semantics.Condition.not (Semantics.Condition.matches (Semantics.NounPhrase.pro (Semantics.Reach.verbed (Semantics.Deed.action "discard") (some Semantics.NounWord.card) Semantics.VerbedMarking.thisWay) Semantics.Plurality.one) (Semantics.Predicate.hasType Semantics.CardType.land))) (Semantics.Action.createObject (Semantics.Amount.lit 1) (Semantics.CreationSpec.token (Semantics.TokenSpec.written ({ characteristics := ({ name := none, cost := none, colors := [Semantics.Color.white], supertypes := [], types := [Semantics.CardType.creature], subtypes := [(Semantics.Subtype.of Semantics.CardType.creature "Human"), (Semantics.Subtype.of Semantics.CardType.creature "Soldier")], text := [], power := (some (Semantics.Amount.lit 1)), toughness := (some (Semantics.Amount.lit 1)), loyalty := none, defense := none } : Semantics.Characteristics), qualities := [] } : Semantics.CharacteristicBundle)) []) true none) none)]) none none))
semantic_definition reveal (arg0 : Semantics.NounPhrase) : Semantics.Action :=
  (Semantics.Action.ownScope (some (Semantics.Deed.action "reveal")) (Semantics.Action.enact (Semantics.Deed.action "reveal") (Semantics.Action.expose Semantics.ExposeVerb.reveal (Semantics.Exposed.cards (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "reveal.arg0" arg0)))) none none))
semantic_definition search (arg0 : Semantics.Location) (arg1 : Semantics.Quantity) (arg2 : Semantics.Predicate) : Semantics.Action :=
  (Semantics.Action.ownScope (some (Semantics.Deed.action "search")) (Semantics.Action.enact (Semantics.Deed.action "search") (Semantics.Action.search arg0 (Semantics.Quantity.inCaller arg1) (Semantics.Predicate.inCaller arg2)) none none))
semantic_definition suspect (arg0 : Semantics.NounPhrase) : Semantics.Action :=
  (Semantics.Action.ownScope (some (Semantics.Deed.action "suspect")) (Semantics.Action.enact (Semantics.Deed.action "suspect") (Semantics.Action.gainDesignation (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "suspect.arg0" arg0)) (Semantics.DesignationLabel.named "suspected") none) none none))
semantic_definition theRingTemptsYou : Semantics.Action :=
  (Semantics.Action.ownScope (some (Semantics.Deed.action "theRingTemptsYou")) (Semantics.Action.enact (Semantics.Deed.action "theRingTemptsYou") (Semantics.Action.sequentially [(Semantics.Action.doIf (Semantics.Condition.not (Semantics.Condition.exists_ (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.and [Semantics.Predicate.isEmblem, (Semantics.Predicate.named (Semantics.NameSource.printed "The Ring")), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.owner Semantics.NounPhrase.actor)])))) (Semantics.Action.createObject (Semantics.Amount.lit 1) (Semantics.CreationSpec.emblem (some "The Ring") [(Semantics.Ability.static (Semantics.StaticSpec.characteristicChange (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.and [(Semantics.Predicate.hasDesignation (Semantics.DesignationLabel.named "ringBearer") none), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)])) [(Semantics.CharacteristicEdit.typeLine Semantics.QualityOp.adds ({ supertypes := (some [Semantics.Supertype.legendary]), types := none, subtypes := none, retained := none } : Semantics.TypeLineChanges))])), (Semantics.Ability.static (Semantics.StaticSpec.cant (Semantics.Subject.by (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.compare [(Semantics.ProjAxis.stat Semantics.Stat.power)] Semantics.Comparator.greater (Semantics.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.power) (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasDesignation (Semantics.DesignationLabel.named "ringBearer") none), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)]))))]))) (Semantics.Templates.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.block) (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasDesignation (Semantics.DesignationLabel.named "ringBearer") none), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)]))) ({} : Semantics.DeonticMods))), (Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.abilityGrant Semantics.NounPhrase.this (Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.hasDesignation (Semantics.DesignationLabel.named "ringBearer") none), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)]))) (Semantics.Action.attack none)) [] none [] none none none (Semantics.Action.sequentially [(Semantics.Action.draw (Semantics.Amount.lit 1)), (Semantics.Action.ownScope (some (Semantics.Deed.action "discard")) (Semantics.Action.enact (Semantics.Deed.action "discard") (Semantics.Action.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [Semantics.Predicate.isCard, (Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.hand none))]))) ((Semantics.Location.zone Semantics.Zone.hand (some Semantics.NounPhrase.actor))) (some (Semantics.Location.zone Semantics.Zone.graveyard none)) Semantics.ObservationPoint.arrival none []) none none))]))) (Semantics.Condition.compareAmt (Semantics.Amount.tally (Semantics.DidClause.mk (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Action.enact (Semantics.Deed.action "theRingTemptsYou") (Semantics.Templates.deonticActionFor (Semantics.Deed.action "theRingTemptsYou") Semantics.NounPhrase.any) none none) (Semantics.LookbackSpan.duration (Semantics.Duration.between none (some Semantics.GameEvent.now)) none)) Semantics.TallyMode.events) Semantics.Comparator.atLeast (Semantics.Amount.lit 2)) Semantics.CondMarking.asLongAs)), (Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.abilityGrant Semantics.NounPhrase.this (Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.by (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.hasType Semantics.CardType.creature))) (Semantics.Action.block (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.hasDesignation (Semantics.DesignationLabel.named "ringBearer") none), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)])))) [] none [] none none none (Semantics.Action.delay (Semantics.GameEvent.beginningOf Semantics.TurnPart.endOfCombat none (Semantics.TurnRef.next none)) [] none (Semantics.Action.instruct (Semantics.NounPhrase.possessorOf Semantics.PossessorAxis.controller (Semantics.NounPhrase.pro (Semantics.Reach.word (Semantics.NounWord.type Semantics.CardType.creature)) Semantics.Plurality.one)) (Semantics.Action.ownScope (some (Semantics.Deed.action "sacrifice")) (Semantics.Action.enact (Semantics.Deed.action "sacrifice") (Semantics.Action.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.pro (Semantics.Reach.word (Semantics.NounWord.type Semantics.CardType.creature)) Semantics.Plurality.one)) ((Semantics.Location.zone Semantics.Zone.battlefield none)) (some (Semantics.Location.zone Semantics.Zone.graveyard none)) Semantics.ObservationPoint.departure none []) none none)))))) (Semantics.Condition.compareAmt (Semantics.Amount.tally (Semantics.DidClause.mk (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Action.enact (Semantics.Deed.action "theRingTemptsYou") (Semantics.Templates.deonticActionFor (Semantics.Deed.action "theRingTemptsYou") Semantics.NounPhrase.any) none none) (Semantics.LookbackSpan.duration (Semantics.Duration.between none (some Semantics.GameEvent.now)) none)) Semantics.TallyMode.events) Semantics.Comparator.atLeast (Semantics.Amount.lit 3)) Semantics.CondMarking.asLongAs)), (Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.abilityGrant Semantics.NounPhrase.this (Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.hasDesignation (Semantics.DesignationLabel.named "ringBearer") none), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)]))) (Semantics.Action.dealDamage Semantics.Amount.anyAmount (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) Semantics.Predicate.anyPlayer) Semantics.DamageKind.combatOnly)) [] none [] none none none (Semantics.Action.instruct (Semantics.NounPhrase.described Semantics.DetPhrase.each (Semantics.Predicate.isOpponentOf Semantics.NounPhrase.you)) (Semantics.Action.changeLife (Semantics.Delta.down (Semantics.Amount.lit 3)))))) (Semantics.Condition.compareAmt (Semantics.Amount.tally (Semantics.DidClause.mk (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Action.enact (Semantics.Deed.action "theRingTemptsYou") (Semantics.Templates.deonticActionFor (Semantics.Deed.action "theRingTemptsYou") Semantics.NounPhrase.any) none none) (Semantics.LookbackSpan.duration (Semantics.Duration.between none (some Semantics.GameEvent.now)) none)) Semantics.TallyMode.events) Semantics.Comparator.atLeast (Semantics.Amount.lit 4)) Semantics.CondMarking.asLongAs))]) true none) none), (Semantics.Action.choose none (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.actor)])) Semantics.Disclosure.openly none), (Semantics.Action.gainDesignation (Semantics.NounPhrase.pro (Semantics.Reach.word (Semantics.NounWord.type Semantics.CardType.creature)) Semantics.Plurality.one) (Semantics.DesignationLabel.named "ringBearer") none)]) none none))
semantic_definition timeTravel : Semantics.Action :=
  (Semantics.Action.ownScope (some (Semantics.Deed.action "timeTravel")) (Semantics.Action.enact (Semantics.Deed.action "timeTravel") (Semantics.Action.sequentially [(Semantics.Action.choose none (Semantics.NounPhrase.described (Semantics.DetPhrase.count (Semantics.Quantity.range none none) none) (Semantics.Predicate.or [(Semantics.Predicate.and [(Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.battlefield none)), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.actor), (Semantics.Predicate.hasCounters (some (Semantics.CounterKind.named "timeCounter")))]), (Semantics.Predicate.and [Semantics.Predicate.isCard, (Semantics.Predicate.in_ (Semantics.Location.zone Semantics.Zone.exile none)), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.owner Semantics.NounPhrase.actor), (Semantics.Predicate.hasCounters (some (Semantics.CounterKind.named "timeCounter")))])])) Semantics.Disclosure.openly none), (Semantics.Action.doForEach (Semantics.NounPhrase.pro Semantics.Reach.bare Semantics.Plurality.many) (Semantics.Action.chooseModes (Semantics.Quantity.range (some 1) (some 1)) [(none, none, (Semantics.Action.putCounters (Semantics.Amount.lit 1) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "timeCounter")) (Semantics.NounPhrase.pro Semantics.Reach.bare Semantics.Plurality.one) Semantics.CounterBatch.one true)), (none, none, (Semantics.Action.removeCounters (some (Semantics.Quantity.range (some 1) (some 1))) (some (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "timeCounter"))) (Semantics.NounPhrase.pro Semantics.Reach.bare Semantics.Plurality.one) Semantics.CounterBatch.one true))]))]) none none))
/-! ## Relations -/

semantic_definition encodedOn : Semantics.RelationLabel :=
  (Semantics.RelationLabel.named "encodedOn")
semantic_definition haunting : Semantics.RelationLabel :=
  (Semantics.RelationLabel.named "haunting")
semantic_definition paired : Semantics.RelationLabel :=
  (Semantics.RelationLabel.named "paired")
/-! ## Subtypes -/

semantic_definition blood : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.artifact "Blood")
semantic_definition bobblehead : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.artifact "Bobblehead")
semantic_definition book : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.artifact "Book")
semantic_definition clue : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.artifact "Clue")
semantic_definition contraption : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.artifact "Contraption")
semantic_definition equipment : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.artifact "Equipment")
semantic_definition food : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.artifact "Food")
semantic_definition fortification : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.artifact "Fortification")
semantic_definition gold : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.artifact "Gold")
semantic_definition incubator : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.artifact "Incubator")
semantic_definition Subtype.infinity : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.artifact "Infinity")
semantic_definition junk : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.artifact "Junk")
semantic_definition lander : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.artifact "Lander")
semantic_definition map : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.artifact "Map")
semantic_definition mutagen : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.artifact "Mutagen")
semantic_definition powerstone : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.artifact "Powerstone")
semantic_definition spacecraft : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.artifact "Spacecraft")
semantic_definition stone : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.artifact "Stone")
semantic_definition treasure : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.artifact "Treasure")
semantic_definition vehicle : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.artifact "Vehicle")
semantic_definition vibranium : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.artifact "Vibranium")
semantic_definition siege : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.battle "Siege")
semantic_definition advisor : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Advisor")
semantic_definition aetherborn : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Aetherborn")
semantic_definition alien : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Alien")
semantic_definition ally : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Ally")
semantic_definition angel : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Angel")
semantic_definition antelope : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Antelope")
semantic_definition ape : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Ape")
semantic_definition archer : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Archer")
semantic_definition archon : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Archon")
semantic_definition armadillo : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Armadillo")
semantic_definition artificer : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Artificer")
semantic_definition assassin : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Assassin")
semantic_definition assemblyWorker : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Assembly-Worker")
semantic_definition astartes : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Astartes")
semantic_definition atog : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Atog")
semantic_definition aurochs : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Aurochs")
semantic_definition avatar : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Avatar")
semantic_definition azra : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Azra")
semantic_definition badger : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Badger")
semantic_definition balloon : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Balloon")
semantic_definition barbarian : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Barbarian")
semantic_definition bard : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Bard")
semantic_definition basilisk : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Basilisk")
semantic_definition bat : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Bat")
semantic_definition bear : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Bear")
semantic_definition beast : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Beast")
semantic_definition beaver : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Beaver")
semantic_definition beeble : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Beeble")
semantic_definition beholder : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Beholder")
semantic_definition berserker : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Berserker")
semantic_definition bird : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Bird")
semantic_definition bison : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Bison")
semantic_definition blinkmoth : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Blinkmoth")
semantic_definition boar : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Boar")
semantic_definition bringer : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Bringer")
semantic_definition brushwagg : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Brushwagg")
semantic_definition camarid : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Camarid")
semantic_definition camel : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Camel")
semantic_definition capybara : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Capybara")
semantic_definition caribou : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Caribou")
semantic_definition carrier : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Carrier")
semantic_definition cat : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Cat")
semantic_definition centaur : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Centaur")
semantic_definition child : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Child")
semantic_definition chimera : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Chimera")
semantic_definition citizen : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Citizen")
semantic_definition cleric : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Cleric")
semantic_definition clown : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Clown")
semantic_definition cockatrice : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Cockatrice")
semantic_definition construct : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Construct")
semantic_definition coward : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Coward")
semantic_definition coyote : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Coyote")
semantic_definition crab : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Crab")
semantic_definition crocodile : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Crocodile")
semantic_definition ctan : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "C'tan")
semantic_definition custodes : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Custodes")
semantic_definition cyberman : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Cyberman")
semantic_definition cyclops : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Cyclops")
semantic_definition dalek : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Dalek")
semantic_definition dauthi : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Dauthi")
semantic_definition demigod : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Demigod")
semantic_definition demon : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Demon")
semantic_definition deserter : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Deserter")
semantic_definition detective : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Detective")
semantic_definition devil : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Devil")
semantic_definition dinosaur : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Dinosaur")
semantic_definition djinn : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Djinn")
semantic_definition doctor : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Doctor")
semantic_definition dog : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Dog")
semantic_definition dragon : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Dragon")
semantic_definition drake : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Drake")
semantic_definition dreadnought : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Dreadnought")
semantic_definition drix : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Drix")
semantic_definition drone : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Drone")
semantic_definition druid : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Druid")
semantic_definition dryad : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Dryad")
semantic_definition dwarf : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Dwarf")
semantic_definition echidna : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Echidna")
semantic_definition efreet : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Efreet")
semantic_definition egg : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Egg")
semantic_definition elder : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Elder")
semantic_definition eldrazi : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Eldrazi")
semantic_definition elemental : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Elemental")
semantic_definition elephant : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Elephant")
semantic_definition elf : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Elf")
semantic_definition elk : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Elk")
semantic_definition employee : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Employee")
semantic_definition eternal : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Eternal")
semantic_definition eye : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Eye")
semantic_definition faerie : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Faerie")
semantic_definition ferret : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Ferret")
semantic_definition fish : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Fish")
semantic_definition flagbearer : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Flagbearer")
semantic_definition fox : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Fox")
semantic_definition fractal : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Fractal")
semantic_definition frog : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Frog")
semantic_definition fungus : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Fungus")
semantic_definition gamer : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Gamer")
semantic_definition gamma : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Gamma")
semantic_definition gargoyle : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Gargoyle")
semantic_definition germ : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Germ")
semantic_definition giant : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Giant")
semantic_definition giraffe : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Giraffe")
semantic_definition gith : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Gith")
semantic_definition glimmer : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Glimmer")
semantic_definition gnoll : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Gnoll")
semantic_definition gnome : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Gnome")
semantic_definition goat : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Goat")
semantic_definition goblin : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Goblin")
semantic_definition god : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "God")
semantic_definition golem : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Golem")
semantic_definition gorgon : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Gorgon")
semantic_definition graveborn : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Graveborn")
semantic_definition gremlin : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Gremlin")
semantic_definition griffin : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Griffin")
semantic_definition guest : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Guest")
semantic_definition hag : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Hag")
semantic_definition halfling : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Halfling")
semantic_definition hamster : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Hamster")
semantic_definition harpy : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Harpy")
semantic_definition hedgehog : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Hedgehog")
semantic_definition hellion : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Hellion")
semantic_definition hero : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Hero")
semantic_definition hippo : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Hippo")
semantic_definition hippogriff : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Hippogriff")
semantic_definition homarid : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Homarid")
semantic_definition homunculus : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Homunculus")
semantic_definition horror : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Horror")
semantic_definition horse : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Horse")
semantic_definition human : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Human")
semantic_definition hydra : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Hydra")
semantic_definition hyena : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Hyena")
semantic_definition illusion : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Illusion")
semantic_definition imp : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Imp")
semantic_definition incarnation : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Incarnation")
semantic_definition inhuman : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Inhuman")
semantic_definition inkling : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Inkling")
semantic_definition inquisitor : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Inquisitor")
semantic_definition insect : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Insect")
semantic_definition jackal : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Jackal")
semantic_definition jellyfish : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Jellyfish")
semantic_definition juggernaut : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Juggernaut")
semantic_definition kangaroo : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Kangaroo")
semantic_definition kavu : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Kavu")
semantic_definition kirin : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Kirin")
semantic_definition kithkin : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Kithkin")
semantic_definition knight : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Knight")
semantic_definition kobold : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Kobold")
semantic_definition kor : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Kor")
semantic_definition kraken : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Kraken")
semantic_definition kree : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Kree")
semantic_definition lamia : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Lamia")
semantic_definition lammasu : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Lammasu")
semantic_definition leech : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Leech")
semantic_definition lemur : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Lemur")
semantic_definition leviathan : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Leviathan")
semantic_definition lhurgoyf : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Lhurgoyf")
semantic_definition licid : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Licid")
semantic_definition lizard : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Lizard")
semantic_definition llama : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Llama")
semantic_definition lobster : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Lobster")
semantic_definition manticore : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Manticore")
semantic_definition masticore : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Masticore")
semantic_definition mercenary : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Mercenary")
semantic_definition merfolk : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Merfolk")
semantic_definition metathran : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Metathran")
semantic_definition minion : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Minion")
semantic_definition minotaur : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Minotaur")
semantic_definition mite : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Mite")
semantic_definition mole : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Mole")
semantic_definition monger : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Monger")
semantic_definition mongoose : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Mongoose")
semantic_definition monk : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Monk")
semantic_definition monkey : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Monkey")
semantic_definition moogle : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Moogle")
semantic_definition moonfolk : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Moonfolk")
semantic_definition mount : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Mount")
semantic_definition mouse : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Mouse")
semantic_definition mutant : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Mutant")
semantic_definition myr : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Myr")
semantic_definition mystic : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Mystic")
semantic_definition nautilus : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Nautilus")
semantic_definition necron : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Necron")
semantic_definition nephilim : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Nephilim")
semantic_definition nightmare : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Nightmare")
semantic_definition nightstalker : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Nightstalker")
semantic_definition ninja : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Ninja")
semantic_definition noble : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Noble")
semantic_definition noggle : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Noggle")
semantic_definition nomad : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Nomad")
semantic_definition nymph : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Nymph")
semantic_definition octopus : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Octopus")
semantic_definition ogre : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Ogre")
semantic_definition ooze : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Ooze")
semantic_definition orb : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Orb")
semantic_definition orc : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Orc")
semantic_definition orgg : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Orgg")
semantic_definition otter : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Otter")
semantic_definition ouphe : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Ouphe")
semantic_definition ox : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Ox")
semantic_definition oyster : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Oyster")
semantic_definition pangolin : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Pangolin")
semantic_definition peasant : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Peasant")
semantic_definition pegasus : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Pegasus")
semantic_definition pentavite : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Pentavite")
semantic_definition performer : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Performer")
semantic_definition pest : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Pest")
semantic_definition phelddagrif : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Phelddagrif")
semantic_definition phoenix : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Phoenix")
semantic_definition phyrexian : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Phyrexian")
semantic_definition pilot : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Pilot")
semantic_definition pincher : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Pincher")
semantic_definition pirate : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Pirate")
semantic_definition plant : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Plant")
semantic_definition platypus : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Platypus")
semantic_definition porcupine : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Porcupine")
semantic_definition possum : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Possum")
semantic_definition praetor : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Praetor")
semantic_definition primarch : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Primarch")
semantic_definition prism : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Prism")
semantic_definition processor : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Processor")
semantic_definition qu : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Qu")
semantic_definition rabbit : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Rabbit")
semantic_definition raccoon : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Raccoon")
semantic_definition ranger : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Ranger")
semantic_definition rat : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Rat")
semantic_definition rebel : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Rebel")
semantic_definition reflection : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Reflection")
semantic_definition rhino : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Rhino")
semantic_definition rigger : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Rigger")
semantic_definition robot : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Robot")
semantic_definition rogue : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Rogue")
semantic_definition sable : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Sable")
semantic_definition salamander : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Salamander")
semantic_definition samurai : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Samurai")
semantic_definition sand : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Sand")
semantic_definition saproling : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Saproling")
semantic_definition satyr : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Satyr")
semantic_definition scarecrow : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Scarecrow")
semantic_definition scientist : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Scientist")
semantic_definition scion : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Scion")
semantic_definition scorpion : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Scorpion")
semantic_definition scout : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Scout")
semantic_definition sculpture : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Sculpture")
semantic_definition seal_ : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Seal")
semantic_definition serf : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Serf")
semantic_definition serpent : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Serpent")
semantic_definition servo : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Servo")
semantic_definition shade : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Shade")
semantic_definition shaman : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Shaman")
semantic_definition shapeshifter : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Shapeshifter")
semantic_definition shark : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Shark")
semantic_definition sheep : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Sheep")
semantic_definition shiar : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Shi'ar")
semantic_definition siren : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Siren")
semantic_definition skeleton : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Skeleton")
semantic_definition skrull : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Skrull")
semantic_definition skunk : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Skunk")
semantic_definition slith : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Slith")
semantic_definition sliver : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Sliver")
semantic_definition sloth : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Sloth")
semantic_definition slug : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Slug")
semantic_definition snail : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Snail")
semantic_definition snake : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Snake")
semantic_definition soldier : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Soldier")
semantic_definition soltari : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Soltari")
semantic_definition sorcerer : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Sorcerer")
semantic_definition spawn : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Spawn")
semantic_definition specter : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Specter")
semantic_definition spellshaper : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Spellshaper")
semantic_definition sphinx : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Sphinx")
semantic_definition spider : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Spider")
semantic_definition spike : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Spike")
semantic_definition spirit : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Spirit")
semantic_definition splinter : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Splinter")
semantic_definition sponge : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Sponge")
semantic_definition spy : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Spy")
semantic_definition squid : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Squid")
semantic_definition squirrel : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Squirrel")
semantic_definition starfish : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Starfish")
semantic_definition surrakar : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Surrakar")
semantic_definition survivor : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Survivor")
semantic_definition symbiote : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Symbiote")
semantic_definition synth : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Synth")
semantic_definition tentacle : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Tentacle")
semantic_definition tetravite : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Tetravite")
semantic_definition thalakos : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Thalakos")
semantic_definition thopter : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Thopter")
semantic_definition thrull : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Thrull")
semantic_definition tiefling : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Tiefling")
semantic_definition timeLord : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Time Lord")
semantic_definition toy : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Toy")
semantic_definition treefolk : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Treefolk")
semantic_definition trilobite : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Trilobite")
semantic_definition triskelavite : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Triskelavite")
semantic_definition troll : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Troll")
semantic_definition turtle : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Turtle")
semantic_definition tyranid : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Tyranid")
semantic_definition unicorn : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Unicorn")
semantic_definition utrom : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Utrom")
semantic_definition vampire : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Vampire")
semantic_definition varmint : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Varmint")
semantic_definition vedalken : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Vedalken")
semantic_definition villain : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Villain")
semantic_definition volver : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Volver")
semantic_definition wall : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Wall")
semantic_definition walrus : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Walrus")
semantic_definition warlock : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Warlock")
semantic_definition warrior : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Warrior")
semantic_definition weasel : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Weasel")
semantic_definition weird : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Weird")
semantic_definition werewolf : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Werewolf")
semantic_definition whale : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Whale")
semantic_definition wizard : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Wizard")
semantic_definition wolf : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Wolf")
semantic_definition wolverine : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Wolverine")
semantic_definition wombat : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Wombat")
semantic_definition worm : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Worm")
semantic_definition wraith : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Wraith")
semantic_definition wurm : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Wurm")
semantic_definition yeti : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Yeti")
semantic_definition zombie : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Zombie")
semantic_definition zubera : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Zubera")
semantic_definition aura : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.enchantment "Aura")
semantic_definition background : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.enchantment "Background")
semantic_definition cartouche : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.enchantment "Cartouche")
semantic_definition case : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.enchantment "Case")
semantic_definition class_ : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.enchantment "Class")
semantic_definition curse : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.enchantment "Curse")
semantic_definition plan : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.enchantment "Plan")
semantic_definition role : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.enchantment "Role")
semantic_definition room : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.enchantment "Room")
semantic_definition rune : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.enchantment "Rune")
semantic_definition saga : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.enchantment "Saga")
semantic_definition shard : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.enchantment "Shard")
semantic_definition shrine : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.enchantment "Shrine")
semantic_definition cave : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.land "Cave")
semantic_definition desert : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.land "Desert")
semantic_definition forest : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.land "Forest")
semantic_definition gate : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.land "Gate")
semantic_definition island : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.land "Island")
semantic_definition lair : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.land "Lair")
semantic_definition locus : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.land "Locus")
semantic_definition mine : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.land "Mine")
semantic_definition mountain : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.land "Mountain")
semantic_definition plains : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.land "Plains")
semantic_definition planet : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.land "Planet")
semantic_definition powerPlant : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.land "Power-Plant")
semantic_definition sphere : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.land "Sphere")
semantic_definition swamp : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.land "Swamp")
semantic_definition tower : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.land "Tower")
semantic_definition town : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.land "Town")
semantic_definition urzas : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.land "Urza's")
semantic_definition ajani : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Ajani")
semantic_definition aminatou : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Aminatou")
semantic_definition angrath : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Angrath")
semantic_definition arlinn : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Arlinn")
semantic_definition ashiok : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Ashiok")
semantic_definition bahamut : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Bahamut")
semantic_definition basri : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Basri")
semantic_definition bolas : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Bolas")
semantic_definition calix : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Calix")
semantic_definition chandra : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Chandra")
semantic_definition comet : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Comet")
semantic_definition dack : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Dack")
semantic_definition dakkon : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Dakkon")
semantic_definition daretti : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Daretti")
semantic_definition davriel : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Davriel")
semantic_definition dellian : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Dellian")
semantic_definition dihada : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Dihada")
semantic_definition domri : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Domri")
semantic_definition dovin : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Dovin")
semantic_definition ellywick : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Ellywick")
semantic_definition elminster : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Elminster")
semantic_definition elspeth : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Elspeth")
semantic_definition estrid : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Estrid")
semantic_definition freyalise : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Freyalise")
semantic_definition garruk : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Garruk")
semantic_definition gideon : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Gideon")
semantic_definition grist : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Grist")
semantic_definition guff : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Guff")
semantic_definition huatli : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Huatli")
semantic_definition jace : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Jace")
semantic_definition jared : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Jared")
semantic_definition jaya : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Jaya")
semantic_definition jeska : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Jeska")
semantic_definition kaito : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Kaito")
semantic_definition karn : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Karn")
semantic_definition kasmina : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Kasmina")
semantic_definition kaya : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Kaya")
semantic_definition kiora : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Kiora")
semantic_definition koth : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Koth")
semantic_definition liliana : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Liliana")
semantic_definition lolth : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Lolth")
semantic_definition lukka : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Lukka")
semantic_definition minsc : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Minsc")
semantic_definition mordenkainen : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Mordenkainen")
semantic_definition nahiri : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Nahiri")
semantic_definition narset : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Narset")
semantic_definition niko : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Niko")
semantic_definition nissa : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Nissa")
semantic_definition nixilis : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Nixilis")
semantic_definition oko : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Oko")
semantic_definition quintorius : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Quintorius")
semantic_definition ral : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Ral")
semantic_definition rowan : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Rowan")
semantic_definition saheeli : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Saheeli")
semantic_definition samut : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Samut")
semantic_definition sarkhan : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Sarkhan")
semantic_definition serra : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Serra")
semantic_definition sivitri : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Sivitri")
semantic_definition sorin : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Sorin")
semantic_definition szat : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Szat")
semantic_definition tamiyo : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Tamiyo")
semantic_definition tasha : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Tasha")
semantic_definition teferi : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Teferi")
semantic_definition teyo : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Teyo")
semantic_definition tezzeret : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Tezzeret")
semantic_definition tibalt : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Tibalt")
semantic_definition tyvar : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Tyvar")
semantic_definition ugin : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Ugin")
semantic_definition urza : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Urza")
semantic_definition venser : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Venser")
semantic_definition vivien : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Vivien")
semantic_definition vraska : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Vraska")
semantic_definition vronos : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Vronos")
semantic_definition will : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Will")
semantic_definition windgrace : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Windgrace")
semantic_definition wrenn : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Wrenn")
semantic_definition xenagos : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Xenagos")
semantic_definition yanggu : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Yanggu")
semantic_definition yanling : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Yanling")
semantic_definition zariel : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Zariel")
semantic_definition adventure : Semantics.Subtype :=
  (Semantics.Subtype.spell "Adventure")
semantic_definition arcane : Semantics.Subtype :=
  (Semantics.Subtype.spell "Arcane")
semantic_definition lesson : Semantics.Subtype :=
  (Semantics.Subtype.spell "Lesson")
semantic_definition omen : Semantics.Subtype :=
  (Semantics.Subtype.spell "Omen")
semantic_definition trap : Semantics.Subtype :=
  (Semantics.Subtype.spell "Trap")
/-! ## Turn parts -/

semantic_definition beginningOfCombatStep : Semantics.TurnPart :=
  Semantics.TurnPart.beginningOfCombat
semantic_definition beginningPhase : Semantics.TurnPart :=
  Semantics.TurnPart.beginningPhase
semantic_definition cleanup : Semantics.TurnPart :=
  Semantics.TurnPart.cleanup
semantic_definition cleanupStep : Semantics.TurnPart :=
  Semantics.TurnPart.cleanup
semantic_definition TurnPart.combat : Semantics.TurnPart :=
  Semantics.TurnPart.combat
semantic_definition combatDamageStep : Semantics.TurnPart :=
  Semantics.TurnPart.combatDamage
semantic_definition combatPhase : Semantics.TurnPart :=
  Semantics.TurnPart.combat
semantic_definition declareAttackersStep : Semantics.TurnPart :=
  Semantics.TurnPart.declareAttackers
semantic_definition declareBlockersStep : Semantics.TurnPart :=
  Semantics.TurnPart.declareBlockers
semantic_definition drawStep : Semantics.TurnPart :=
  Semantics.TurnPart.drawStep
semantic_definition endOfCombatStep : Semantics.TurnPart :=
  Semantics.TurnPart.endOfCombat
semantic_definition endStep : Semantics.TurnPart :=
  Semantics.TurnPart.endStep
semantic_definition endingPhase : Semantics.TurnPart :=
  Semantics.TurnPart.endingPhase
semantic_definition mainPhase : Semantics.TurnPart :=
  Semantics.TurnPart.mainPhase
semantic_definition postcombatMainPhase : Semantics.TurnPart :=
  Semantics.TurnPart.postcombatMain
semantic_definition precombatMainPhase : Semantics.TurnPart :=
  Semantics.TurnPart.firstMain
semantic_definition untapStep : Semantics.TurnPart :=
  Semantics.TurnPart.untapStep
semantic_definition upkeep : Semantics.TurnPart :=
  Semantics.TurnPart.upkeep
semantic_definition upkeepStep : Semantics.TurnPart :=
  Semantics.TurnPart.upkeep

/-! ## Tokens -/

/-- A Food token's characteristics: "a colorless Food artifact token with '{2}, {T}, Sacrifice
this token: You gain 3 life.'" [CR#111.10b] -/
semantic_definition foodToken : CharacteristicBundle :=
  { characteristics :=
      { types := [.artifact], subtypes := [artifactType "Food"],
        text :=
          [ activated (.compound [.mana [generic 2], .tapSymbol, .perform (sacrifice thisArtifact)])
              (gainLife (.lit 3)) ] } }
/-- A Treasure token's characteristics: "a colorless Treasure artifact token with '{T},
Sacrifice this token: Add one mana of any color.'" [CR#111.10a] -/
semantic_definition treasureToken : CharacteristicBundle :=
  { characteristics :=
      { types := [.artifact], subtypes := [artifactType "Treasure"],
        text :=
          [ activated (.compound [.tapSymbol, .perform (sacrifice thisArtifact)])
              (.addMana (.lit 1) (.anyColor .sameColor) []) ] } }

/-! ## Templates over Definitions -/

/-- "Exile <subject> until <event>." -/
semantic_template exileUntil (subject : NounPhrase) (event : GameEvent) : Action :=
  .holdUntil (exile subject) event
/-- The replacement's application, rather than installation of a new shield
[CR#701.19b,701.19c]. -/
semantic_template regenerationApplication (subject : NounPhrase) : Action :=
  .enact (.action "regenerate")
    (.sequentially [
      .clearDamage (.firstMention "regenerationApplication.subject" subject),
      .instruct (.possessorOf .controller (.laterMention "regenerationApplication.subject" subject))
        (tap (.laterMention "regenerationApplication.subject" subject)),
      .doIf (.or [.matches (.laterMention "regenerationApplication.subject" subject) (.inCombat .attackerOf none),
                  .matches (.laterMention "regenerationApplication.subject" subject) (.inCombat .blockerOf none)])
        (.combat (.laterMention "regenerationApplication.subject" subject) (.participation .outsideCombat)) none])
    none none
/-- "attach <what> to it": ordinary pronoun `it` resolved by `D′` (no `Window` override). -/
semantic_template attachToIt (what : NounPhrase) : Action :=
  Action.attachTo what it
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
    [ Actor.discard discarded,
      .doForEach
        (allOf (.and [.not isLand, .isCard, .did ⟨.any, Actor.discard .which, .thisWay⟩]))
        step ]
/-- "you may choose not to untap <subject> during [<whose>] untap step" -/
semantic_template mayDeclineUntap (subject : NounPhrase) (whose : Option NounPhrase) : StaticSpec :=
  Primitives.StaticSpec.partScope .untapStep whose
    (.can .any (untap subject) {})
/-- "untap <subject> during [<whose>] untap step" -/
semantic_template untapsDuring (subject : NounPhrase) (whose : Option NounPhrase) : StaticSpec :=
  Primitives.StaticSpec.partScope .untapStep whose
    (.must .any (untap subject) {})
/-- "<subject> may be attached to <host>": the legal-host permission over the attach deed's host
role [CR#701.3a,301.5]. -/
semantic_template mayBeAttachedTo (subject host : NounPhrase) : StaticSpec :=
  .can .any (Action.attachTo subject host) {}
/-- "<subject> regenerates": the verb as an event. -/
semantic_template regenerates (subject : NounPhrase) : GameEvent :=
  .does .any (regenerationApplication subject)
/-- "create N tapped <token>" [CR#111.2] -/
semantic_template createTappedToken (token : CharacteristicBundle) (count : Amount := .lit 1) :
    Action :=
  Action.create count (.written token) [.entersAs .tapped]

/-! The regeneration shield is a Definition, but its replacement applies
`regenerationApplication`, a Template over `tap`, so it follows that Template here. -/

/-- A resolving regeneration instruction installs a single-use shield for this turn;
only its eventual application carries the Regenerate label (`Definition`, [CR#701.19a,701.19c]). -/
semantic_definition regenerate (subject : NounPhrase) : Action :=
  .ownScope (some (.action "regenerate"))
    (.establish (.instead
      (.would .any (destroy (.firstMention "regenerate.0" (.inCaller subject))))
      (regenerationApplication (.laterMention "regenerate.0" (.inCaller subject)))
      none .nextTimeOnly none) (some thisTurn))

private theorem regenerationEventUsesTheApplication :
    regenerates thisCreature = .does .any (regenerationApplication thisCreature) := rfl

private theorem regenerationEventDescribesTheApplicationRatherThanTheShield :
    let event := regenerates thisCreature
    let applied := (stepGameEvent 64 (DRS.ofList []) event).1
    let shield := (stepAction 64 .postEvent .obj (DRS.ofList []) (regenerate thisCreature)).1
    GameEvent.check (DRS.ofList []) event = [] ∧
      countOnes .object applied = 1 ∧
      (applied.findById? 0).map (fun ref => (ref.kind, ref.zone)) =
        some (.object, some .battlefield) ∧
      countOnes .object shield = 0 ∧
      Action.check (DRS.ofList [])
        (.triggerThisWay (regenerate thisCreature) event
          (.putCounters (.lit 1) (.printed (.named "m1m1Counter")) it .many true)) = [] := by decide

end Semantics.Definitions
