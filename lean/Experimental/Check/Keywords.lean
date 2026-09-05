import Experimental.Check.Words

/-!
# Experimental.Check.Keywords

The keyword facts table and its shapes: port of `KeywordShapes.idr` and the generated
`FactsGen.idr` (emitted for Idris by `cargo xtask facts generate` from
`plugins/builtin_v2/macros/stubs/keyword_abilities/*.ron`; this file is converted from that
output until xtask emits Lean too).
-/

namespace Mtg

inductive CompoundHead where
  | quality | number
  deriving DecidableEq, Repr

def CompoundHead.optional : CompoundHead → Bool
  | .quality => true
  | .number => false

/-- `quality` covers "partner with [name]" [CR#702.124j], whose slot is a card name. -/
inductive KeywordParamShape where
  | noParam | cost | quality | subject | number | ability
  | compound (head : CompoundHead)
  | deckCondition
  deriving DecidableEq, Repr

def KeywordParamShape.fits : KeywordParamShape → KeywordParamShape → Bool
  | .compound h, .cost => h.optional
  | want, got => want == got

/-- A written parameter fits a keyword when any of the row's admitted shapes takes it, so a
keyword's CR-defined variants live on one row. -/
def paramShapesFit (wants : List KeywordParamShape) (got : KeywordParamShape) : Bool :=
  wants.any (·.fits got)

inductive StackRegime where
  | atCasting | atResolution
  deriving DecidableEq, Repr

structure KeywordFacts where
  word : KeywordLabel
  /-- Every parameter shape the CR admits for this keyword [CR#702]. -/
  paramShapes : List KeywordParamShape := []
  counterEligible : Bool := false
  regime : Option StackRegime := none
  /-- True where the ability is the spell's own, so no permanent holds it [CR#113.6]. -/
  functionsOnStack : Bool := false
  onPermanentCard : Bool := true
  onSpellCard : Bool := false
  paidCost : Bool := false
  /-- Defined by the CR as a triggered ability with a quoted expansion
  [CR#702.21a,702.24a,702.30a,702.40a,702.45a,702.86a,702.112a,702.135a]. -/
  bodied : Bool := false
  wantsModes : Bool := false
  deriving Repr, BEq

def keywordFacts : List KeywordFacts :=
  [
    { word := "Haste", paramShapes := [.noParam], counterEligible := true },
    { word := "Flying", paramShapes := [.noParam], counterEligible := true },
    { word := "Trample", paramShapes := [.noParam], counterEligible := true },
    { word := "Vigilance", paramShapes := [.noParam], counterEligible := true },
    { word := "Deathtouch", paramShapes := [.noParam], counterEligible := true, regime := some .atResolution },
    { word := "DoubleStrike", paramShapes := [.noParam], counterEligible := true },
    { word := "FirstStrike", paramShapes := [.noParam], counterEligible := true },
    { word := "Reach", paramShapes := [.noParam], counterEligible := true },
    { word := "Defender", paramShapes := [.noParam] },
    { word := "Convoke", paramShapes := [.noParam], regime := some .atCasting, functionsOnStack := true, onSpellCard := true },
    { word := "Improvise", paramShapes := [.noParam], regime := some .atCasting, functionsOnStack := true, onSpellCard := true },
    { word := "Storm", paramShapes := [.noParam], regime := some .atCasting, functionsOnStack := true, onSpellCard := true, bodied := true },
    { word := "Lifelink", paramShapes := [.noParam], counterEligible := true, regime := some .atResolution },
    { word := "Ward", paramShapes := [.cost], paidCost := true, bodied := true },
    { word := "Protection", paramShapes := [.quality] },
    { word := "Enchant", paramShapes := [.subject] },
    { word := "Equip", paramShapes := [.cost, .compound .quality], paidCost := true },
    { word := "Suspend", paramShapes := [.compound .number], onSpellCard := true, paidCost := true },
    { word := "Ascend", paramShapes := [.noParam], onSpellCard := true },
    { word := "Storied", paramShapes := [.noParam] },
    { word := "Renown", paramShapes := [.number], bodied := true },
    { word := "Indestructible", paramShapes := [.noParam], counterEligible := true },
    { word := "Flash", paramShapes := [.noParam], regime := some .atCasting, functionsOnStack := true, onSpellCard := true },
    { word := "Kicker", paramShapes := [.cost], regime := some .atCasting, functionsOnStack := true, onSpellCard := true, paidCost := true },
    { word := "Multikicker", paramShapes := [.cost], regime := some .atCasting, functionsOnStack := true, onSpellCard := true, paidCost := true },
    { word := "CumulativeUpkeep", paramShapes := [.cost], paidCost := true, bodied := true },
    { word := "Echo", paramShapes := [.cost], paidCost := true, bodied := true },
    { word := "Hexproof", paramShapes := [.noParam, .quality], counterEligible := true },
    { word := "Menace", paramShapes := [.noParam], counterEligible := true },
    { word := "Skulk", paramShapes := [.noParam] },
    { word := "Bushido", paramShapes := [.number], bodied := true },
    { word := "Unearth", paramShapes := [.cost], paidCost := true },
    { word := "Flashback", paramShapes := [.cost], onPermanentCard := false, onSpellCard := true, paidCost := true },
    { word := "Dredge", paramShapes := [.number], onSpellCard := true },
    { word := "Retrace", paramShapes := [.noParam], onSpellCard := true },
    { word := "Cycling", paramShapes := [.cost, .compound .quality], onSpellCard := true, paidCost := true },
    { word := "Ninjutsu", paramShapes := [.cost], paidCost := true },
    { word := "Miracle", paramShapes := [.cost], onSpellCard := true, paidCost := true },
    { word := "Warp", paramShapes := [.cost], regime := some .atCasting, functionsOnStack := true, onSpellCard := true, paidCost := true },
    { word := "Afterlife", paramShapes := [.number], bodied := true },
    { word := "Boast", paramShapes := [.ability] },
    { word := "Exhaust", paramShapes := [.ability] },
    { word := "PowerUp", paramShapes := [.ability] },
    { word := "Affinity", paramShapes := [.quality], regime := some .atCasting, functionsOnStack := true, onSpellCard := true },
    { word := "Annihilator", paramShapes := [.number], bodied := true },
    { word := "Fear", paramShapes := [.noParam] },
    { word := "Shroud", paramShapes := [.noParam] },
    { word := "Banding", paramShapes := [.noParam] },
    { word := "BandsWithOther", paramShapes := [.quality] },
    { word := "Landwalk", paramShapes := [.quality] },
    { word := "Changeling", paramShapes := [.noParam], onSpellCard := true },
    { word := "Crew", paramShapes := [.number] },
    { word := "Saddle", paramShapes := [.number] },
    { word := "PartnerWith", paramShapes := [.quality] },
    { word := "Emerge", paramShapes := [.cost], regime := some .atCasting, functionsOnStack := true, onSpellCard := true, paidCost := true },
    { word := "Craft", paramShapes := [.cost], paidCost := true },
    { word := "Madness", paramShapes := [.cost], onSpellCard := true, paidCost := true },
    { word := "Prowl", paramShapes := [.cost], regime := some .atCasting, functionsOnStack := true, onSpellCard := true, paidCost := true },
    { word := "Surge", paramShapes := [.cost], regime := some .atCasting, functionsOnStack := true, onSpellCard := true, paidCost := true },
    { word := "Spectacle", paramShapes := [.cost], regime := some .atCasting, functionsOnStack := true, onSpellCard := true, paidCost := true },
    { word := "Freerunning", paramShapes := [.cost], regime := some .atCasting, functionsOnStack := true, onSpellCard := true, paidCost := true },
    { word := "Sneak", paramShapes := [.cost], regime := some .atCasting, functionsOnStack := true, onSpellCard := true, paidCost := true },
    { word := "Mayhem", paramShapes := [.cost], onSpellCard := true, paidCost := true },
    { word := "Disturb", paramShapes := [.cost], onSpellCard := true, paidCost := true },
    { word := "Morph", paramShapes := [.cost], onSpellCard := true, paidCost := true },
    { word := "Entwine", paramShapes := [.cost], regime := some .atCasting, functionsOnStack := true, onSpellCard := true, paidCost := true, wantsModes := true },
    { word := "Escalate", paramShapes := [.cost], regime := some .atCasting, functionsOnStack := true, onSpellCard := true, paidCost := true, wantsModes := true },
    { word := "Fuse", paramShapes := [.noParam], regime := some .atCasting, functionsOnStack := true, onPermanentCard := false, onSpellCard := true },
    { word := "Escape", paramShapes := [.cost], onSpellCard := true, paidCost := true },
    { word := "Foretell", paramShapes := [.cost], onSpellCard := true, paidCost := true },
    { word := "Bestow", paramShapes := [.cost], paidCost := true },
    { word := "Disguise", paramShapes := [.cost], paidCost := true },
    { word := "Mutate", paramShapes := [.cost], regime := some .atCasting, functionsOnStack := true, paidCost := true },
    { word := "Overload", paramShapes := [.cost], regime := some .atCasting, functionsOnStack := true, onPermanentCard := false, onSpellCard := true, paidCost := true },
    { word := "Dash", paramShapes := [.cost], regime := some .atCasting, functionsOnStack := true, paidCost := true },
    { word := "Evoke", paramShapes := [.cost], paidCost := true },
    { word := "Blitz", paramShapes := [.cost], regime := some .atCasting, functionsOnStack := true, paidCost := true },
    { word := "Cleave", paramShapes := [.cost], regime := some .atCasting, functionsOnStack := true, onPermanentCard := false, onSpellCard := true, paidCost := true },
    { word := "Harmonize", paramShapes := [.cost], onPermanentCard := false, onSpellCard := true, paidCost := true },
    { word := "Impending", paramShapes := [.compound .number], regime := some .atCasting, functionsOnStack := true, paidCost := true },
    { word := "Awaken", paramShapes := [.compound .number], regime := some .atCasting, functionsOnStack := true, onPermanentCard := false, onSpellCard := true, paidCost := true },
    { word := "Buyback", paramShapes := [.cost], regime := some .atCasting, functionsOnStack := true, onPermanentCard := false, onSpellCard := true, paidCost := true },
    { word := "Casualty", paramShapes := [.number], regime := some .atCasting, functionsOnStack := true, onSpellCard := true, paidCost := true },
    { word := "Squad", paramShapes := [.cost], regime := some .atCasting, functionsOnStack := true, paidCost := true },
    { word := "Offspring", paramShapes := [.cost], regime := some .atCasting, functionsOnStack := true, paidCost := true },
    { word := "Gift", paramShapes := [.subject], regime := some .atCasting, functionsOnStack := true, onSpellCard := true, paidCost := true },
    { word := "Replicate", paramShapes := [.cost], regime := some .atCasting, functionsOnStack := true, onPermanentCard := false, onSpellCard := true, paidCost := true },
    { word := "Delve", paramShapes := [.noParam], regime := some .atCasting, functionsOnStack := true, onSpellCard := true },
    { word := "Infect", paramShapes := [.noParam], regime := some .atResolution },
    { word := "Cascade", paramShapes := [.noParam], regime := some .atCasting, functionsOnStack := true, onSpellCard := true, bodied := true },
    { word := "Prowess", paramShapes := [.noParam], regime := some .atCasting, bodied := true },
    { word := "SplitSecond", paramShapes := [.noParam], functionsOnStack := true, onPermanentCard := false, onSpellCard := true },
    { word := "Phasing", paramShapes := [.noParam] },
    { word := "Decayed", paramShapes := [.noParam], counterEligible := true },
    { word := "Exalted", paramShapes := [.noParam], counterEligible := true, bodied := true },
    { word := "Shadow", paramShapes := [.noParam], counterEligible := true },
    { word := "Companion", paramShapes := [.deckCondition] },
    { word := "Absorb", paramShapes := [.number] },
    { word := "Afflict", paramShapes := [.number], bodied := true },
    { word := "Aftermath", paramShapes := [.noParam], onPermanentCard := false, onSpellCard := true },
    { word := "Amplify", paramShapes := [.number] },
    { word := "Assist", paramShapes := [.noParam], regime := some .atCasting, functionsOnStack := true, onSpellCard := true },
    { word := "AuraSwap", paramShapes := [.cost], paidCost := true },
    { word := "Backup", paramShapes := [.number], bodied := true },
    { word := "Bargain", paramShapes := [.noParam], regime := some .atCasting, functionsOnStack := true, onSpellCard := true },
    { word := "BattleCry", paramShapes := [.noParam], bodied := true },
    { word := "Bloodthirst", paramShapes := [.number] },
    { word := "Champion", paramShapes := [.subject] },
    { word := "Cipher", paramShapes := [.noParam], regime := some .atCasting, functionsOnStack := true, onPermanentCard := false, onSpellCard := true },
    { word := "Compleated", paramShapes := [.noParam] },
    { word := "Conspire", paramShapes := [.noParam], regime := some .atCasting, functionsOnStack := true, onSpellCard := true, paidCost := true },
    { word := "Daybound", paramShapes := [.noParam] },
    { word := "Demonstrate", paramShapes := [.noParam], regime := some .atCasting, functionsOnStack := true, onSpellCard := true, bodied := true },
    { word := "Dethrone", paramShapes := [.noParam], bodied := true },
    { word := "Devoid", paramShapes := [.noParam], onSpellCard := true },
    { word := "Devour", paramShapes := [.number] },
    { word := "Embalm", paramShapes := [.cost], paidCost := true },
    { word := "Encore", paramShapes := [.cost], paidCost := true },
    { word := "Enlist", paramShapes := [.noParam] },
    { word := "Epic", paramShapes := [.noParam], onSpellCard := true },
    { word := "Eternalize", paramShapes := [.cost], paidCost := true },
    { word := "Evolve", paramShapes := [.noParam], bodied := true },
    { word := "Exploit", paramShapes := [.noParam], bodied := true },
    { word := "Extort", paramShapes := [.noParam], regime := some .atCasting, bodied := true },
    { word := "Fabricate", paramShapes := [.number], bodied := true },
    { word := "Fading", paramShapes := [.number] },
    { word := "Firebending", paramShapes := [.number], bodied := true },
    { word := "Flanking", paramShapes := [.noParam], bodied := true },
    { word := "ForMirrodin", paramShapes := [.noParam], bodied := true },
    { word := "Forecast", paramShapes := [.ability], onSpellCard := true },
    { word := "Fortify", paramShapes := [.cost], paidCost := true },
    { word := "Frenzy", paramShapes := [.number], bodied := true },
    { word := "Graft", paramShapes := [.number] },
    { word := "Gravestorm", paramShapes := [.noParam], regime := some .atCasting, functionsOnStack := true, onSpellCard := true, bodied := true },
    { word := "Haunt", paramShapes := [.noParam], onSpellCard := true, bodied := true },
    { word := "HiddenAgenda", paramShapes := [.noParam] },
    { word := "Hideaway", paramShapes := [.number], bodied := true },
    { word := "Horsemanship", paramShapes := [.noParam] },
    { word := "Increment", paramShapes := [.noParam], regime := some .atCasting, bodied := true },
    { word := "Infinity", paramShapes := [.ability] },
    { word := "Ingest", paramShapes := [.noParam], bodied := true },
    { word := "Intimidate", paramShapes := [.noParam] },
    { word := "JobSelect", paramShapes := [.noParam], bodied := true },
    { word := "JumpStart", paramShapes := [.noParam], onPermanentCard := false, onSpellCard := true },
    { word := "LevelUp", paramShapes := [.cost], paidCost := true },
    { word := "LivingMetal", paramShapes := [.noParam] },
    { word := "LivingWeapon", paramShapes := [.noParam], bodied := true },
    { word := "MaxSpeed", paramShapes := [.ability] },
    { word := "Melee", paramShapes := [.noParam], bodied := true },
    { word := "Mentor", paramShapes := [.noParam], bodied := true },
    { word := "Mobilize", paramShapes := [.number], bodied := true },
    { word := "Modular", paramShapes := [.number] },
    { word := "MoreThanMeetsTheEye", paramShapes := [.cost], paidCost := true },
    { word := "Myriad", paramShapes := [.noParam], bodied := true },
    { word := "Nightbound", paramShapes := [.noParam] },
    { word := "Offering", paramShapes := [.quality], regime := some .atCasting, functionsOnStack := true },
    { word := "Outlast", paramShapes := [.cost], paidCost := true },
    { word := "Paradigm", paramShapes := [.noParam], onSpellCard := true },
    { word := "Partner", paramShapes := [.noParam] },
    { word := "Persist", paramShapes := [.noParam], bodied := true },
    { word := "Plot", paramShapes := [.cost], onSpellCard := true, paidCost := true },
    { word := "Poisonous", paramShapes := [.number], bodied := true },
    { word := "Provoke", paramShapes := [.noParam], bodied := true },
    { word := "Rampage", paramShapes := [.number], bodied := true },
    { word := "Ravenous", paramShapes := [.noParam] },
    { word := "ReadAhead", paramShapes := [.noParam] },
    { word := "Rebound", paramShapes := [.noParam], regime := some .atCasting, functionsOnStack := true, onPermanentCard := false, onSpellCard := true },
    { word := "Reconfigure", paramShapes := [.cost], paidCost := true },
    { word := "Recover", paramShapes := [.cost], onSpellCard := true, paidCost := true, bodied := true },
    { word := "Reinforce", paramShapes := [.compound .number], onSpellCard := true, paidCost := true },
    { word := "Riot", paramShapes := [.noParam] },
    { word := "Ripple", paramShapes := [.number], regime := some .atCasting, functionsOnStack := true, onSpellCard := true, bodied := true },
    { word := "Scavenge", paramShapes := [.cost], paidCost := true },
    { word := "Solved", paramShapes := [.ability] },
    { word := "Soulbond", paramShapes := [.noParam] },
    { word := "Soulshift", paramShapes := [.number], bodied := true },
    { word := "SpaceSculptor", paramShapes := [.noParam] },
    { word := "Splice", paramShapes := [.compound .quality], onSpellCard := true, paidCost := true },
    { word := "Spree", paramShapes := [.noParam], regime := some .atCasting, functionsOnStack := true, onSpellCard := true, wantsModes := true },
    { word := "StartYourEngines", paramShapes := [.noParam] },
    { word := "Station", paramShapes := [.noParam] },
    { word := "Sunburst", paramShapes := [.noParam] },
    { word := "Teamwork", paramShapes := [.number], regime := some .atCasting, functionsOnStack := true, onSpellCard := true },
    { word := "Tiered", paramShapes := [.noParam], regime := some .atCasting, functionsOnStack := true, onSpellCard := true, wantsModes := true },
    { word := "Toxic", paramShapes := [.number] },
    { word := "Training", paramShapes := [.noParam], bodied := true },
    { word := "Transfigure", paramShapes := [.cost], paidCost := true },
    { word := "Transmute", paramShapes := [.cost], onSpellCard := true, paidCost := true },
    { word := "Tribute", paramShapes := [.number] },
    { word := "UmbraArmor", paramShapes := [.noParam] },
    { word := "Undaunted", paramShapes := [.noParam], regime := some .atCasting, functionsOnStack := true, onSpellCard := true },
    { word := "Undying", paramShapes := [.noParam], bodied := true },
    { word := "Unleash", paramShapes := [.noParam] },
    { word := "Vanishing", paramShapes := [.number] },
    { word := "Visit", paramShapes := [.ability] },
    { word := "WebSlinging", paramShapes := [.cost], regime := some .atCasting, functionsOnStack := true, paidCost := true },
    { word := "Wither", paramShapes := [.noParam], regime := some .atResolution }
  ]

def distinctKeywordWords : List KeywordFacts → Bool
  | [] => true
  | f :: fs => !(fs.map (·.word)).elem f.word && distinctKeywordWords fs

def keywordFactsFor (k : KeywordLabel) : Option KeywordFacts := keywordFacts.find? (·.word == k)
def knownKeyword (k : KeywordLabel) : Bool := (keywordFactsFor k).isSome
def keywordParamShapes (k : KeywordLabel) : List KeywordParamShape :=
  (keywordFactsFor k).elim [] (·.paramShapes)

def KeywordParamShape.costs : KeywordParamShape → Bool
  | .cost => true
  | .compound _ => true
  | _ => false

def keywordCosts (k : KeywordLabel) : Bool := (keywordParamShapes k).any (·.costs)
def keywordBodied (k : KeywordLabel) : Bool := (keywordFactsFor k).elim false (·.bodied)
def keywordParamless (k : KeywordLabel) : Bool :=
  (keywordFactsFor k).elim false (·.paramShapes.elem .noParam)

def KeywordFamily.ok (c : KeywordFamily) : Bool :=
  match keywordFactsFor c.word with
  | none => false
  | some f =>
    match c.sort with
    | none => f.paramShapes.any (· != .noParam)
    | some _ => f.paramShapes.elem .quality

def KeywordTerm.known : KeywordTerm → Bool
  | .the k => knownKeyword k
  | .anyIn c => c.ok
  | .theWith k _ => (keywordParamShapes k).elem .number

def KeywordTerm.bare : KeywordTerm → Bool
  | .the k => keywordParamless k
  | .anyIn c => c.ok
  | .theWith _ _ => false

def allKnownKeywordTerms : List KeywordTerm → Bool
  | [] => true
  | k :: ks => k.known && allKnownKeywordTerms ks

def keywordCounterOk (k : KeywordLabel) : Bool := (keywordFactsFor k).elim false (·.counterEligible)
def keywordStackRegime (k : KeywordLabel) : Option StackRegime := keywordFactsFor k >>= (·.regime)
def keywordFunctionsOnStack (k : KeywordLabel) : Bool :=
  (keywordFactsFor k).elim false (·.functionsOnStack)

def PaidCostName.named : PaidCostName → Bool
  | .byKeyword kw => (keywordFactsFor kw).elim false (·.paidCost)
  | .byNthKeyword _ kw => (keywordFactsFor kw).elim false (·.paidCost)
  | .theAlternative => true
  | .theAdditional => true

def PaidFacet.named : PaidFacet → Bool
  | .colorsSpent => true
  | .manaValueSpent => true
  | .timesPaid which => which.named
  | .readback which _ => which.named

def AbilityClass.known : AbilityClass → Bool
  | .keyword k => knownKeyword k
  | _ => true

def distinctClasses : List AbilityClass → Bool
  | [] => true
  | c :: cs => !cs.elem c && distinctClasses cs

def CostNamed.nameable : CostNamed → Bool
  | .containing _ => true
  | .ofKeyword kw => keywordCosts kw
  | .ofSpecialAction _ => true

/-- Idris `KeywordCounterEligible`, `knownCounter`, `counterShift` folded: is the counter kind
one the tables admit? -/
def CounterKind.known : CounterKind → Bool
  | .boost p t => counterShift p && counterShift t
  | .keyword k => keywordCounterOk k
  | .named l => knownCounter l

end Mtg
