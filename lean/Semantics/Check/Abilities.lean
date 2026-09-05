import Semantics.Abilities
import Semantics.Check.Triggers

/-!
# Semantics.Check.Abilities

The effect layer of the checker: port of the functions of `Effect.idr`. The instruction
profile (`pre`, `announced`, `rider`, `deed`) is what threads the antecedent stack through a
sequence; every `Instruction`, `StaticSpec`, `Cost`, token, and ability constructor gets its
Idris obligations as rules.
-/

namespace Semantics

/-! ## Small attributes -/

def AsThough.sort : AsThough → PremiseSort
  | .of _ => .object
  | .mana _ _ _ => .mana
  | .greater _ _ => .value

def twoPartiesOk : NounPhrase → Bool
  | .both l r => l.plur == .one && r.plur == .one
  | .described d _ => (d.quant >>= Quantity.exact) == some 2
  | _ => false

def controlExchangeZone (z : Option Zone) : Bool := onFieldZone z || onStackZone z

def cardSwapZonesOk : Option Zone → Option Zone → Bool
  | some a, some b => a != b
  | _, _ => false

def zoneSwapOk (a b : Zone) : Bool := isCardZone (some a) && isCardZone (some b) && a != b

def Exchanged.intro (bs : Bindings) : Exchanged → Bindings
  | .lifeTotals parties => nomIntro bs parties
  | .controlOf a b => nomIntro (nomIntro bs a) b
  | .cardsAcross a b => nomIntro (nomIntro bs a) b
  | .zones a b => ZoneExpr.delta (ZoneExpr.delta bs a ++ bs) b ++ ZoneExpr.delta bs a ++ bs
  | .values a b => Amount.intro (Amount.intro bs a) b
  | .textBoxes a b => nomIntro (nomIntro bs a) b

def Exchanged.deed : Exchanged → List Binding
  | .lifeTotals _ => [outcomeB .lifeGained, outcomeB .lifeLost]
  | _ => []

def Exchanged.costOk : Exchanged → Bool
  | .lifeTotals parties => parties.costNounOk
  | .controlOf a b => a.costNounOk && b.costNounOk
  | .cardsAcross a b => a.costNounOk && b.costNounOk
  | .zones _ _ => true
  | .values _ _ => true
  | .textBoxes a b => a.costNounOk && b.costNounOk

/-- The numerical values an exchange can set [CR#701.12g]: a life total, a power or
toughness, or a rolled result. -/
def Amount.settableValue : Amount → Bool
  | .statOf (.playerStat .lifeTotal) _ => true
  | .statOf (.stat .power) _ => true
  | .statOf (.stat .toughness) _ => true
  | .theOutcome .rollResult => true
  | _ => false

/-- Two halves naming one value: each would become equal to its own previous value, so the
sentence states nothing. -/
def selfExchanged : Amount → Amount → Bool
  | .theOutcome a, .theOutcome b => a == b
  | .statOf (.playerStat a) .you, .statOf (.playerStat b) .you => a == b
  | .statOf (.stat a) (.asType t .this _), .statOf (.stat b) (.asType u .this _) => a == b && t == u
  | _, _ => false

def TokenQuality.hosted (tys : List CardType) : TokenQuality → Bool
  | .withEveryType space => tys.any fun t => spaceHosted space (some t)
  | .withQuality q =>
    match q.qualityReadHost with
    | none => true
    | some h => tys.elem h

def counterHolderKind (k : Kind) : Bool := Kind.lte k (.join .object .player)

def CounterKindSource.scope : CounterKindSource → Kind → Bool
  | .printed c, k => c.scope == k
  | .chosen menu, k => menu.all fun c => c.scope == k
  | .distinctChosen menu, k => menu.all fun c => c.scope == k
  | .bound, k => counterHolderKind k
  | .those, k => counterHolderKind k
  | .own, k => counterHolderKind k
  | .sameAs _, k => Kind.lte k .object

def optCounterSourceScope : Option CounterKindSource → Kind → Bool
  | none, k => counterHolderKind k
  | some s, k => s.scope k

def Amount.unit : Amount → Bool
  | .lit 1 => true
  | _ => false

/-- "the same number and kind of counters": `sameAs` carries the number of counters as well
as their kinds, so the amount slot is the unit placeholder. -/
def kindAmountOk : Amount → CounterKindSource → Bool
  | a, .sameAs _ => a.unit
  | _, _ => true

def optKindAmountOk : Amount → Option CounterKindSource → Bool
  | _, none => true
  | a, some s => kindAmountOk a s

def CounterKindSource.intro (bs : Bindings) : CounterKindSource → Bindings
  | .sameAs src => nomIntro bs src
  | _ => bs

/-- What an instruction leaves on the stack: what it read (`pre`), what it announced, an
optional rider context, and what it did (`deed`). -/
structure InstrProfile where
  pre : Bindings
  announced : Bindings
  rider : Option Bindings
  deed : List Binding
  deriving Repr, BEq

def InstrProfile.intro (p : InstrProfile) : Bindings := p.deed ++ p.announced
def InstrProfile.riderCtx (p : InstrProfile) : Bindings := p.rider.getD p.pre
/-- The last clause of a sequence publishes its deed with what it announced. -/
def InstrProfile.last (p : InstrProfile) : InstrProfile := ⟨p.pre, p.intro, p.rider, []⟩
/-- A simultaneous batch announces every clause's deed at once. -/
def InstrProfile.simCons (d : List Binding) (p : InstrProfile) : InstrProfile :=
  ⟨p.pre, d ++ p.announced, none, []⟩
/-- An offer announces what its body announced; its deed stays a deed. -/
def InstrProfile.offer (p : InstrProfile) : InstrProfile := ⟨p.intro, p.announced, none, p.deed⟩
def InstrProfile.simLast (p : InstrProfile) : InstrProfile := ⟨p.pre, p.intro, none, []⟩
def sameIntro (b : Bindings) (d : List Binding) : InstrProfile := ⟨b, b, none, d⟩

def ptWritten : Option (Amount × Amount) → Bool
  | none => false
  | some _ => true

def colorOpOk : QualityOp → ColorSpec → Bool
  | .sets, _ => true
  | _, .some [] => false
  | _, _ => true

def ptDelta (bs : Bindings) : Option (Amount × Amount) → List Binding
  | none => []
  | some (p, t) => Amount.delta (Amount.intro bs p) t ++ Amount.delta bs p

def Stat.modifyOk : Stat → Bool
  | .power | .toughness => true
  | _ => false

def deltaKind : Delta Amount → StaticKind
  | .set _ => .basePtSet
  | _ => .ptDelta

def CostShift.delta (bs : Bindings) : CostShift → List Binding
  | .less a _ => Amount.delta bs a
  | .more a => Amount.delta bs a
  | .run _ _ _ => []

def gatePayer : Binding := ⟨.the, .player, .one, .player false⟩

def CountBound.delta (bs : Bindings) : CountBound → List Binding
  | .moreThan k => Amount.delta bs k
  | .additional q => Quantity.delta bs q

def deonticBoundOk (bs : Bindings) (ds : Deeds) : Option CountBound → Bool
  | none => true
  | some b => ds.all deedBoundedOk && (b.delta bs).isEmpty

def DeonticPatient.intro (bs : Bindings) : DeonticPatient → Bindings
  | .noPatient => bs
  | .defendingPlayer m => nomIntro bs m
  | .counterpart m => nomIntro bs m
  | .targetedBy m => nomIntro bs m
  | .counterpartsAt _ => bs

def Role.isAgent : Role → Bool
  | .agent => true
  | .patient => false

def counterpartFits (bs : Bindings) (ds : Deeds) (r : Role) (m : NounPhrase) (moved : Bool) : Bool :=
  deedFits ds r (m.kindOr .object) m.isAbility (NounPhrase.headTys bs m)
    (if moved then none else NounPhrase.zone bs m)

/-- A counterpart named after `n` must not be `n` read back. -/
def counterpartNotSelf (bs : Bindings) (n : NounPhrase) : NounPhrase → Bool
  | .pro r pl .whole => !r.tracksObject || countReach r pl (NounPhrase.delta bs n) == 0
  | _ => true

def complementAtOk (bs : Bindings) (n : NounPhrase) (ds : Deeds) (r : Role) (c : DeedComplement) :
    Bool :=
  ds.elem c.deed && counterpartFits (nomIntro bs n) [c.deed] r.counter c.counterpart false &&
    counterpartNotSelf bs n c.counterpart

def DamageScope.intro (bs : Bindings) : DamageScope → Bindings
  | .everywhere => bs
  | .toRecipient n => nomIntro bs n

def DamageAgent.intro (bs : Bindings) : DamageAgent → Bindings
  | .unattributed => bs
  | .dealtBy n => nomIntro bs n

def Unpreventable.intro (bs : Bindings) : Unpreventable → Bindings
  | .described src scope => scope.intro (src.intro bs)
  | .thatDamage => bs

def PreventCut.intro (bs : Bindings) : PreventCut → Bindings
  | .all => bs
  | .some amt => Amount.intro bs amt
  | .shield amt => Amount.intro bs amt
  | .allBut amt => Amount.intro bs amt
  | .half _ => bs

/-- [CR#615.7] A shield counts damage across events until its amount is spent. -/
def PreventCut.shieldUseOk : PreventCut → ReplUse → Bool
  | .shield _, .repeatedly => true
  | .shield _, .nextTimeOnly => false
  | _, _ => true

def DamageScale.intro (bs : Bindings) : DamageScale → Bindings
  | .shifted _ amt => Amount.intro bs amt
  | _ => bs

def DividedVerb.intro (bs : Bindings) : DividedVerb → Bindings
  | .damage src => nomIntro bs src
  | .counters _ => bs

/-- The object an enacted move is done to; a status change carries its own zone law. -/
def Instruction.enactPatient : Instruction → Option NounPhrase
  | .move n _ _ => some n
  | _ => none

/-- A deed done by name happens where the deed table says its patient lives ("destroy" on the
battlefield [CR#701.8a], "discard" from a hand [CR#701.9a]); "this" is wherever the text is.
The Idris carried this on each verb's macro. -/
def enactPatientZoneOk (bs : Bindings) (v : VerbLabel) (e : Instruction) : Bool :=
  match e.enactPatient with
  | none => true
  | some .this => true
  | some n =>
    match actZoneOf v with
    | none => true
    | some z => zoneIsB (NounPhrase.zone bs n) z

/-- Idris `CtrlOverrideOk`: a single controller, or one per member of a group. -/
def NounPhrase.ctrlOverrideOk : NounPhrase → Bool
  | .possessorOf ax _ => possessorKind ax .object
  | n => n.plur.isOne

def Amount.forEach : Amount → Bool
  | .arith .times _ _ => true
  | _ => false

/-- An enacted act names a subject only where the act's facts row gives its agent role a
player, the way `verbedVoiceOk` gates a verbed event. -/
def enactAgentOk : Option NounPhrase → VerbLabel → Bool
  | none, _ => true
  | some _, v => deedKindOk v .agent .player

def keepsOuterOf (outer out : Bindings) : Bool :=
  out == out.take (out.length - outer.length) ++ outer

/-- A distributive deed either only adds to the agent stack, or closes the partitives its own
agents' choices published [CR#700.8d], which spends nothing the table shares [CR#701.21a]. -/
def closesOwnParts (outer out : Bindings) : Bool :=
  out == partsClosed outer || spentDistributively outer out

def eachStackOk (outer out : Bindings) : Bool :=
  keepsOuterOf outer out || (partsDistributed outer && closesOwnParts outer out)

def distributedDelta (bs : Bindings) (s : NounPhrase) (out : Bindings) : List Binding :=
  pluralizeDelta (out.take (out.length - (agentIntro bs s).length))

def mayCtx (bs : Bindings) (d : NounPhrase) : Bindings := agentIntro bs d

/-! ## Deck conditions -/

def ProjAxis.deck : ProjAxis → Bool
  | .stat .manaValue => true
  | _ => false

def Amount.deckBound : Amount → Bool
  | .lit _ => true
  | _ => false

mutual
  /-- A card set aside for the starting deck is outside the game [CR#103.2b], so only its
  characteristics are readable [CR#109.3]. -/
  def Predicate.deckReadable : Predicate → Bool
    | .isCard | .hasType _ | .hasSubtype _ => true
    | .inZone (.zone .battlefield _) => true
    | .compare axes _ bound => axes.all ProjAxis.deck && bound.deckBound
    | .and ps => Predicate.deckReadableAll ps
    | .not p => p.deckReadable
    | _ => false
  termination_by structural p => p

  def Predicate.deckReadableAll : List Predicate → Bool
    | [] => true
    | p :: ps => p.deckReadable && Predicate.deckReadableAll ps
  termination_by structural ps => ps
end

/-- The characteristics [CR#109.3] a deck condition compares; a number and a counter kind are
not characteristics. -/
def QualitySort.deckComparable : QualitySort → Bool
  | .number | .counterKind => false
  | _ => true

mutual
  def DeckTrait.check : DeckTrait → List Refusal
    | .aCharacteristic p => Predicate.check .object [] p ++ refuse p.deckReadable .deckReadable
    | .hasAbilityOf cls => refuse cls.known .knownKeywordTerm
    | .anyOf ts => refuse (!ts.isEmpty) .nonEmpty ++ DeckTrait.checkAll ts
    | _ => []
  termination_by structural t => t

  def DeckTrait.checkAll : List DeckTrait → List Refusal
    | [] => []
    | t :: ts => t.check ++ DeckTrait.checkAll ts
  termination_by structural ts => ts
end

def DeckCondition.check : DeckCondition → List Refusal
  | .everyCardIs scope trait | .noCardIs scope trait =>
    Predicate.check .object [] scope ++ refuse scope.deckReadable .deckReadable ++ trait.check
  | .cardsDiffer scope ax | .cardsShare scope ax =>
    Predicate.check .object [] scope ++ refuse scope.deckReadable .deckReadable ++
      refuse ax.deckComparable .deckComparable
  | .deckSizeOverMinimum _ => []

/-! ## Keywords and regimes -/

def bodyEventRegime (ev : GameEvent) : Option StackRegime :=
  match ev.name with
  | .spellCast => some .atCasting
  | _ => none

def allTermsBare (ks : List KeywordTerm) : Bool := ks.all KeywordTerm.bare

def distinctTerms : List KeywordTerm → Bool
  | [] => true
  | k :: ks => !ks.elem k && distinctTerms ks

mutual
  def Predicate.regime : Predicate → Option StackRegime
    | .castBy _ _ | .castFrom _ | .wasCast => some .atCasting
    | .hasPossessor .controller _ => some .atResolution
    | .and ps => Predicate.regimeAll ps
    | .or ps => Predicate.regimeAll ps
    | _ => none
  termination_by structural p => p

  def Predicate.regimeAll : List Predicate → Option StackRegime
    | [] => none
    | p :: ps =>
      match p.regime with
      | some r => some r
      | none => Predicate.regimeAll ps
  termination_by structural ps => ps
end

def NounPhrase.regime : NounPhrase → Option StackRegime
  | .described _ p => p.regime
  | .namesAgree _ g => g.regime
  | _ => none

def regimeMatches : Option StackRegime → Option StackRegime → Bool
  | some a, some b => a == b
  | _, _ => false

/-! ## Tokens -/

/-- A token's power and toughness together, when both are written. -/
def Characteristics.pt (c : Characteristics) : Option (Amount × Amount) :=
  match c.power, c.toughness with
  | some p, some t => some (p, t)
  | _, _ => none

def Characteristics.qualsFit (c : Characteristics) : Bool :=
  c.qualities.all (TokenQuality.hosted c.types)
def Characteristics.typed (c : Characteristics) : Bool := !c.types.isEmpty
def Characteristics.ptOk (c : Characteristics) : Bool := !c.types.elem .creature || ptWritten c.pt
def Characteristics.lineNonEmpty (c : Characteristics) : Bool :=
  typeLineNonEmpty c.supertypes c.types c.subtypes
def Characteristics.canonical (c : Characteristics) : Bool :=
  colorsDistinct c.colors && typesDistinct c.types && supersDistinct c.supertypes
def Characteristics.additionUnnamed (c : Characteristics) : Bool := c.name.isNone
def Characteristics.lossWritesTypes (c : Characteristics) : Bool :=
  c.pt.isNone && c.colors.isEmpty && c.text.isEmpty && c.name.isNone && c.qualities.isEmpty &&
    c.lineNonEmpty
def Characteristics.headTy (c : Characteristics) : Option CardType := lastType c.types
def Characteristics.copyBundleSays (c : Characteristics) : Bool :=
  let said := (if ptWritten c.pt then 1 else 0) + (if !c.colors.isEmpty then 1 else 0) +
    (if c.lineNonEmpty then 1 else 0)
  said ≥ 2

def Ability.grantable : Ability → Bool
  | .keyword _ _ _ | .activated _ _ _ _ _ _ | .triggered _ _ _ _ _ _ _ _ _ | .static _ => true
  | .italicHead _ ab => ab.grantable
  | .thatAbility _ => true
  | _ => false

def Characteristics.abilitiesOk (c : Characteristics) : Bool := c.text.all Ability.grantable

def bundleOk (op : QualityOp) (ty : Option CardType) (z : Option Zone) (t : Characteristics)
    (ret : Option CardType) : Bool :=
  match op with
  | .adds =>
    (addsSomething ty t.types t.subtypes || !t.supertypes.isEmpty) &&
      addedFits ty t.types t.subtypes && t.canonical && t.abilitiesOk && t.qualsFit &&
      t.additionUnnamed && ret.isNone
  | .sets =>
    zoneIsB z .battlefield && t.lineNonEmpty && addedFits ty t.types t.subtypes &&
      t.abilitiesOk && t.canonical && t.qualsFit && retentionOk t.types ret
  | .loses => zoneIsB z .battlefield && t.lossWritesTypes && t.canonical && ret.isNone

def becomesOk (bs : Bindings) (op : QualityOp) (n : NounPhrase) : QualityPayload → Bool
  | .bundle t ret => bundleOk op (NounPhrase.ty bs n) (NounPhrase.zone bs n) t ret
  | .everyTypeOf space => zoneIsB (NounPhrase.zone bs n) .battlefield && spaceHosted space (NounPhrase.ty bs n)
  | .chosenQuality q => q.qualityReadOk && hostedRead bs q n
  | .colored cs => cs.ok && colorOpOk op cs

def TokenSpec.headTy (bs : Bindings) : TokenSpec → Option CardType
  | .written t => t.headTy
  | .asThose => tyOfThoseAny .token bs
  | .copyOf src _ => NounPhrase.ty bs src

def TokenSpec.delta (bs : Bindings) : TokenSpec → List Binding
  | .written t => ptDelta bs t.pt
  | _ => []

def TokenRider.intro (bs : Bindings) : TokenRider → Bindings
  | .withCounters amt kind _ => kind.intro (Amount.intro bs amt)
  | .under who => nomIntro bs who
  | _ => bs

def ridersZoneFree : List TokenRider → Bool
  | [] => true
  | .withCounters _ _ _ :: rs => ridersZoneFree rs
  | _ :: _ => false

def ridersFitZone (rs : List TokenRider) (z : Zone) : Bool := ridersZoneFree rs || z == .battlefield

/-! ## Static and instruction attributes -/

def StaticSpec.isCoord : StaticSpec → Bool
  | .andAlso _ _ => true
  | _ => false

def StaticSpec.notExtended : StaticSpec → Bool
  | .alsoOffBattlefield _ => false
  | _ => true

def StaticSpec.notCarvedOut : StaticSpec → Bool
  | .doesntRemove _ _ => false
  | _ => true

def DeonticRider.playRidden : DeonticRider → Bool
  | .noRider => false
  | .play _ _ _ _ _ => true

def deonticPatientOk (bs : Bindings) (n : NounPhrase) (ds : Deeds) (r : Role) (patient : DeonticPatient)
    (rider : DeonticRider) : Bool :=
  match patient with
  | .noPatient => true
  | .defendingPlayer _ => ds.all deedDefendsOk && r.isAgent
  | .counterpart m => counterpartFits (nomIntro bs n) ds r.counter m rider.playRidden && counterpartNotSelf bs n m
  | .targetedBy _ => ds.all deedTargetedOk
  | .counterpartsAt cs => distinctDeeds (cs.map (·.deed)) && cs.all (complementAtOk bs n ds r)

def asThoughOk : Compulsion → Deeds → Option AsThough → Bool
  | _, _, none => true
  | .permit, ds, some a => ds.all (deedPremiseOk a.sort)
  | _, _, some _ => false

def Compulsion.permits : Compulsion → Bool
  | .permit => true
  | _ => false

def deonticRiderOk (bs : Bindings) (ds : Deeds) (r : Role) (c : Compulsion) (pat : DeonticPatient)
    (at_ : Bool) : DeonticRider → Bool
  | .noRider => true
  | .play from_ lim win exc _ =>
    match pat with
    | .counterpart m =>
      c.permits && ds.all deedPlaysOk && r.isAgent && playSourceOk (NounPhrase.zone bs m) from_ at_ &&
        playWindowOk lim win && (!exc || from_.isSome)
    | _ => false

def DamageOp.intro (bs : Bindings) : DamageOp → Bindings
  | .prevent cut _ => cut.intro bs
  | .redirect cut to => nomIntro (cut.intro bs) to
  | .scale sc => sc.intro bs

def DamageOp.useOk : DamageOp → ReplUse → Bool
  | .prevent cut _, use => cut.shieldUseOk use
  | .redirect cut _, use => cut.shieldUseOk use
  | .scale _, _ => true

def Cost.isCompound : Cost → Bool
  | .compound _ => true
  | _ => false

def KeywordParam.shape : Option KeywordParam → KeywordParamShape
  | none => .noParam
  | some (.cost _) => .cost
  | some (.quality _) => .quality
  | some (.subject _) => .subject
  | some (.number _) => .number
  | some (.qualityCost _ _) => .compound .quality
  | some (.numberCost _ _) => .compound .number
  | some (.deckCondition _) => .deckCondition

def keywordParamFits (k : KeywordLabel) (p : Option KeywordParam) : Bool :=
  knownKeyword k && paramShapesFit (keywordParamShapes k) (KeywordParam.shape p)

def Ability.notWordHeaded : Ability → Bool
  | .italicHead _ _ => false
  | _ => true

def keywordBodyFits (k : KeywordLabel) : Option Ability → Bool
  | none => true
  | some (.triggered _ ev _ _ _ _ _ _ _) => keywordBodied k && keywordStackRegime k == bodyEventRegime ev
  | some _ => false

def Ability.grantedKeyword : Ability → Option KeywordLabel
  | .keyword k none _ => if keywordParamless k then some k else none
  | _ => none

mutual
  def StaticSpec.keyword : StaticSpec → Option KeywordLabel
    | .conditionally se _ _ => se.keyword
    | .gains _ ab => ab.grantedKeyword
    | _ => none
  termination_by structural se => se
end

def Instruction.keyword : Instruction → Option KeywordLabel
  | .continuously se _ => se.keyword
  | _ => none

def Ability.lineKeyword : Ability → Option KeywordLabel
  | .static se => se.keyword
  | .triggered _ _ _ _ _ _ _ _ instr => instr.keyword
  | _ => none

def Ability.keywordExtendable (ab : Ability) : Bool := ab.lineKeyword.isSome

def keywordListOk (ab : Ability) (ks : List KeywordTerm) : Bool :=
  match ab.lineKeyword with
  | none => false
  | some base => !ks.isEmpty && allTermsBare ks && distinctTerms ks && !ks.elem (.the base)

def Ability.emblemOk : Ability → Bool
  | .activated _ _ _ _ _ _ | .triggered _ _ _ _ _ _ _ _ _ | .static _ => true
  | .italicHead _ ab => ab.emblemOk
  | _ => false

def emblemAbilitiesOk : List Ability → Bool
  | [] => false
  | abl => abl.all Ability.emblemOk

def Ability.regime : Ability → Option StackRegime
  | .keyword k _ _ => keywordStackRegime k
  | .alsoForKeywords ab _ => ab.regime
  | .italicHead _ ab => ab.regime
  | _ => none

def Ability.functionsOnStack : Ability → Bool
  | .keyword k _ _ => keywordFunctionsOnStack k
  | .alsoForKeywords ab _ => ab.functionsOnStack
  | .italicHead _ ab => ab.functionsOnStack
  | _ => false

def grantSubjectFits (zn : Option Zone) (reg : Option StackRegime) (ab : Ability) : Bool :=
  if onStackZone zn then regimeMatches ab.regime reg else !ab.functionsOnStack

def grantSubjectOk (bs : Bindings) (ab : Ability) (n : NounPhrase) : Bool :=
  grantSubjectFits (NounPhrase.zone bs n) n.regime ab

def Ability.letterDelta : Ability → List Binding
  | .keyword _ (some (.number (.letter l))) _ => [letterB l]
  | _ => []

mutual
  def StaticSpec.clauseOk : StaticSpec → Bool
    | .definesPt _ _ _ => false
    | .altCost _ _ => false
    | .andAlso _ parts => StaticSpec.partsClauseOk parts
    | _ => true
  termination_by structural se => se

  def StaticSpec.partsClauseOk : List StaticSpec → Bool
    | [] => true
    | se :: rest => se.clauseOk && StaticSpec.partsClauseOk rest
  termination_by structural parts => parts
end

/-! How many times a cost turns the source itself over [CR#107.5]. -/
mutual
  def Cost.selfTapUses : Cost → Nat
    | .scaled c _ => c.selfTapUses
    | .tapSymbol => 1
    | .untapSymbol => 1
    | .compound cs => Cost.selfTapCount cs
    | .either l r => max l.selfTapUses r.selfTapUses
    | _ => 0
  termination_by structural c => c

  def Cost.selfTapCount : List Cost → Nat
    | [] => 0
    | c :: cs => c.selfTapUses + Cost.selfTapCount cs
  termination_by structural cs => cs
end

mutual
  def Cost.tapOnce : Cost → Bool
    | .scaled c _ => c.tapOnce
    | .compound cs => Cost.selfTapCount cs ≤ 1
    | .either l r => l.tapOnce && r.tapOnce
    | _ => true
  termination_by structural c => c
end

mutual
  def Cost.paidByYou : Cost → Bool
    | .scaled c _ => c.paidByYou
    /- A life payment targets its payer; granting life may target anyone [CR#119.4]. -/
    | .perform (.changeLife who (.down _)) => who.isYou
    | .perform (.changeLife _ _) => true
    | .perform (.enact (some subj) _ _) => subj.isYou
    | .perform _ => true
    | .compound cs => Cost.allPaidByYou cs
    | .either l r => l.paidByYou && r.paidByYou
    | _ => true
  termination_by structural c => c

  def Cost.allPaidByYou : List Cost → Bool
    | [] => true
    | c :: cs => c.paidByYou && Cost.allPaidByYou cs
  termination_by structural cs => cs
end

def payAgreesOk (who : NounPhrase) (c : Cost) : Bool := !who.isYou || c.paidByYou

mutual
  def Cost.offBattlefield : Cost → Bool
    | .scaled c _ => c.offBattlefield
    | .tapSymbol | .untapSymbol | .loyaltySymbol _ => false
    | .compound cs => Cost.allOffBattlefield cs
    | .either l r => l.offBattlefield && r.offBattlefield
    | _ => true
  termination_by structural c => c

  def Cost.allOffBattlefield : List Cost → Bool
    | [] => true
    | c :: cs => c.offBattlefield && Cost.allOffBattlefield cs
  termination_by structural cs => cs
end

mutual
  def Cost.payable : Cost → Bool
    | .scaled c _ => c.payable
    | .tapSymbol | .untapSymbol | .loyaltySymbol _ => false
    | .either l r => l.payable && r.payable
    | _ => true
  termination_by structural c => c
end

def Cost.isLoyalty : Cost → Bool
  | .loyaltySymbol _ => true
  | _ => false

def allCosted : List (Option Cost × Instruction) → Bool
  | [] => true
  | (none, _) :: _ => false
  | (some _, _) :: es => allCosted es

def Instruction.heldUntilOk : Instruction → Bool
  | .setStatus .phasedOut _ => true
  | .move _ _ _ => true
  | .enact none _ (.move _ _ _) => true
  | _ => false

inductive EncloseUse where
  /-- No player takes the action, so "you do" has no subject. -/
  | agentless
  /-- No single taken action for the pro-verb to abbreviate. -/
  | notOneAction
  /-- The clause schedules its action rather than taking it. -/
  | notYetTaken
  /-- A player's own single action. -/
  | reflexive
  deriving DecidableEq, Repr

def EncloseUse.admitsReflex : EncloseUse → Bool
  | .reflexive => true
  | _ => false

def Instruction.reflexEncloseUse : Instruction → EncloseUse
  | .controllerSacrifices _ => .reflexive
  | .skipsNext _ _ _ => .notYetTaken
  | .extraTurn _ _ => .notYetTaken
  | .additionalPart (some _) _ _ _ _ => .notYetTaken
  | .continuously (.gainsControl _ _) _ => .reflexive
  | .pay _ _ _ | .enact _ _ _ | .separateIntoPiles _ _ _ _ | .chooseNewTargets _ | .create _ _ _ _
  | .putCounters _ _ _ | .removeCounters _ _ _ | .moveCounters _ _ _ _ | .doubleCounters _
  | .move _ _ _ | .expose _ _ _ | .addMana _ _ _ _ | .draw _ _ | .choose _ _ _ _ _ | .vote _ _ _ _
  | .search _ _ _ _ | .shuffle _ | .flipCoins _ _ | .rollDice _ _ _
  | .rerollStored _ _ _ => .reflexive
  | .resultsTable _ => .notOneAction
  | .may _ body none _ => body.reflexEncloseUse
  | .may _ _ _ _ => .notOneAction
  | .ifDone body none _ => body.reflexEncloseUse
  | .ifDone _ _ _ => .notOneAction
  | .onlyIf e _ _ => e.reflexEncloseUse
  | .if_ _ _ _ | .forEachOf _ _ | .forEachKindOf _ _ _ _ | .repeat_ _ => .notOneAction
  | .repeated _ (.pay _ _ _) => .reflexive
  | .repeated _ (.may _ (.pay _ _ _) none _) => .reflexive
  | .repeated _ _ => .notOneAction
  | .sequentially _ | .simultaneously _ | .modal _ _ | .insteadOf _ _ | .reflexively _ _
  | .thisWay _ _ _ => .notOneAction
  | .delayed _ _ _ _ => .notYetTaken
  | .heldUntil _ _ => .notYetTaken
  | _ => .agentless

def Instruction.thisWayOutcomeOk : Instruction → Bool
  | .delayed _ _ _ _ => false
  | .may _ body _ _ => body.thisWayOutcomeOk
  | .ifDone body _ _ => body.thisWayOutcomeOk
  | _ => true

mutual
  def Instruction.costActionOk : Instruction → Bool
    | .dealDamage src _ _ => src.costNounOk
    | .controllerSacrifices n => n.costNounOk
    | .doesntUntapNext n _ => n.costNounOk
    | .extraTurn who _ => who.costNounOk
    | .additionalPart none _ _ _ _ => true
    | .additionalPart (some who) _ _ _ _ => who.costNounOk
    | .distribute _ _ among => among.costNounOk
    | .fights a _ => a.costNounOk
    | .turnOver n => n.costNounOk
    | .setStatus _ n => n.costNounOk
    | .losesCounters who _ _ => who.costNounOk
    | .removeFromCombat n => n.costNounOk
    | .attachTo what _ => what.costNounOk
    | .unattach what => what.costNounOk
    | .becomesBlocking n _ => n.costNounOk
    | .stopsBlocking n _ => n.costNounOk
    | .becomesAttacking n _ => n.costNounOk
    | .regenerate n => n.costNounOk
    | .cantBe e _ _ => e.costActionOk
    | .gainsDesignation n _ _ _ => n.costNounOk
    | .unlock .thisDoor => true
    | .unlock (.doorOf _ room) => room.costNounOk
    | .gameBecomes _ | .concludes _ _ | .gameDrawn | .counterSpell _ | .changeLife _ _ | .draw _ _
    | .getsEmblem _ _ | .define _ _ => true
    | .separateIntoPiles _ grp _ _ => grp.costNounOk
    | .copy _ _ what _ _ => what.costNounOk
    | .chooseNewTargets what => what.costNounOk
    | .copyTargets cp _ => cp.costNounOk
    | .choose _ _ n _ _ => n.costNounOk
    | .move what _ _ => what.costNounOk
    | .exchange what => what.costOk
    | .addMana who _ _ _ => who.costNounOk
    | .expose _ who _ => who.costNounOk
    | .search who _ _ _ => who.costNounOk
    | .shuffle whose => whose.costNounOk
    | .flipCoins who _ => who.costNounOk
    | .rollDice who _ _ => who.costNounOk
    | .rerollStored who _ _ => who.costNounOk
    | .create agent _ _ _ => agent.costNounOk
    | .putCounters _ _ on => on.costNounOk
    | .removeCounters _ _ from_ => from_.costNounOk
    | .moveCounters _ _ src dst => src.costNounOk && dst.costNounOk
    | .doubleCounters on => on.costNounOk
    | .enact _ _ e => e.costActionOk
    | .may _ body ifDid ifNot => body.costActionOk && Instruction.costActionOkOpt ifDid && Instruction.costActionOkOpt ifNot
    | .ifDone body ifDid ifNot => body.costActionOk && Instruction.costActionOkOpt ifDid && Instruction.costActionOkOpt ifNot
    | .onlyIf e _ otherwise => e.costActionOk && Instruction.costActionOkOpt otherwise
    | .if_ _ e otherwise => e.costActionOk && Instruction.costActionOkOpt otherwise
    | .forEachOf _ body => body.costActionOk
    | .forEachKindOf _ _ _ body => body.costActionOk
    | .repeated _ (.sequentially es) => Instruction.costStepsOk es
    | .repeated _ body => body.costActionOk
    | .modal _ modes => Instruction.costModesOk modes
    | _ => false
  termination_by structural e => e

  def Instruction.costActionOkOpt : Option Instruction → Bool
    | none => true
    | some e => e.costActionOk

  def Instruction.costModesOk : List (Option Cost × Instruction) → Bool
    | [] => true
    | (_, e) :: es => e.costActionOk && Instruction.costModesOk es
  termination_by structural es => es

  def Instruction.costStepsOk : List Instruction → Bool
    | [] => true
    | e :: es => e.costActionOk && Instruction.costStepsOk es
  termination_by structural es => es
end

def Instruction.isInstead : Instruction → Bool
  | .insteadOf _ _ => true
  | _ => false

def ifDoneArmed : Option Instruction → Option Instruction → Bool
  | none, none => false
  | _, _ => true

/-! ## Profiles: what an instruction leaves on the stack -/

/-- Idris `doesProfile` over the enacted clause's own profile `ep`, so the block below stays
structural. -/
def doesProfile (bs : Bindings) (pl : Plurality) (s : NounPhrase) (v : VerbLabel) (e : Instruction)
    (ep : InstrProfile) : InstrProfile :=
  let bs' := agentIntro bs s
  match pl, e with
  | .many, .move what@(.theRest _ _) to _ =>
    ⟨distributedDelta bs s (nomIntro bs' what) ++ nomIntro bs s,
     afterMoveTo to (partsClosed (nomIntro bs s)),
     some (distributedDelta bs s (stampIntro bs' (some v) what) ++ nomIntro bs s), []⟩
  | .many, .move what to _ =>
    ⟨distributedDelta bs s (nomIntro bs' what) ++ nomIntro bs s,
     afterMoveTo to (distributedDelta bs s (moveIntro bs' (some v) what (some to.sort)) ++ nomIntro bs s),
     some (distributedDelta bs s (stampIntro bs' (some v) what) ++ nomIntro bs s), []⟩
  | .many, .setStatus _ n =>
    ⟨nomIntro bs s, distributedDelta bs s (stampIntro bs' (some v) n) ++ nomIntro bs s, none, []⟩
  | .many, _ => ⟨nomIntro bs s, nomIntro bs s, none, ep.deed⟩
  | .one, .move what to _ =>
    ⟨nomIntro bs' what, afterMoveTo to (moveIntro bs' (some v) what (some to.sort)),
     some (stampIntro bs' (some v) what), []⟩
  | .one, .setStatus _ n => ⟨nomIntro bs' n, stampIntro bs' (some v) n, none, []⟩
  | .one, _ => ep

/-- Idris `mayProfile` over the body's and the follow-up's own profiles. -/
def mayProfile (bodyP : InstrProfile) (didP : Option InstrProfile) (notd : Option Instruction) :
    InstrProfile :=
  match didP, notd with
  | none, _ => bodyP.offer
  | some dp, none => dp.offer
  | some _, some _ => bodyP.offer

mutual
  def Instruction.profile (bs : Bindings) : Instruction → InstrProfile
    | .dealDamage src amt to =>
      sameIntro (nomIntro (Amount.delta (selfSubjIntro bs src) amt ++ nomIntro bs src) to)
        [outcomeB .damageDealt]
    | .controllerSacrifices n =>
      ⟨⟨.the, .player, .one, .player false⟩ :: selfSubjIntro bs n,
       ⟨.the, .player, .one, .player false⟩ ::
         moveIntro bs (some (deedLabel .sacrificing)) n (some .graveyard),
       none, []⟩
    | .distribute v amt among =>
      let bs' := Amount.intro (v.intro bs) amt
      match v with
      | .damage _ => sameIntro (nomIntro bs' among) [outcomeB .damageDealt]
      | .counters _ => sameIntro (nomIntro bs' among) []
    | .fights a b => sameIntro (nomIntro (nomIntro bs a) b) []
    | .turnOver n => sameIntro (nomIntro bs n) []
    | .setStatus _ n => sameIntro (nomIntro bs n) []
    | .doesntUntapNext n steps => sameIntro (Amount.delta bs steps ++ nomIntro bs n) []
    | .skipsNext w _ count => sameIntro (Amount.delta bs count ++ nomIntro bs w) []
    | .extraTurn w count =>
      ⟨Amount.delta bs count ++ nomIntro bs w, turnRefB :: (Amount.delta bs count ++ nomIntro bs w),
       none, []⟩
    | .additionalPart who _ _ count _ => sameIntro (Amount.delta bs count ++ optAgentIntro bs who) []
    | .losesCounters who _ amt => sameIntro (optAmtIntro (nomIntro bs who) amt) []
    | .removeFromCombat n => sameIntro (nomIntro bs n) []
    | .attachTo what host => sameIntro (nomIntro (nomIntro bs what) host) []
    | .unattach what => sameIntro (nomIntro bs what) []
    | .becomesBlocking n what => sameIntro (nomIntro (nomIntro bs n) what) []
    | .stopsBlocking n what => sameIntro (nomIntro (nomIntro bs n) what) []
    | .becomesAttacking n none => sameIntro (nomIntro bs n) []
    | .becomesAttacking n (some whom) => sameIntro (nomIntro (nomIntro bs n) whom) []
    | .regenerate n => sameIntro (nomIntro bs n) []
    | .cantBe e _ _ => Instruction.profile bs e
    | .gainsDesignation n _ _ _ => sameIntro (nomIntro bs n) []
    | .unlock door => sameIntro (door.intro bs) []
    | .gameBecomes _ => sameIntro bs []
    | .concludes _ who => sameIntro (nomIntro bs who) []
    | .gameDrawn => sameIntro bs []
    | .restartsGame => sameIntro bs []
    | .separateIntoPiles who grp piles faces =>
      let bs' := nomIntro bs who
      ⟨nomIntro bs' grp, partsClosed (nomIntro bs' grp), none,
       [⟨.the, .pile, .many, .pile (NounPhrase.zone bs' grp) (some piles) (pileMentionFace faces)⟩]⟩
    | .counterSpell what => sameIntro (nomIntro bs what) []
    | .copy src agent what times _ =>
      let bs' := nomIntro bs agent
      let k := what.kindOr .object
      sameIntro (Amount.intro (nomIntro bs' what) times)
        [⟨.the, k, outputPlur what.plur times.plur,
          copyPayloadIn k what.isAbility (NounPhrase.ty bs' what) (src.landsIn (NounPhrase.zone bs' what))⟩]
    | .chooseNewTargets what => sameIntro (nomIntro bs what) []
    | .copyTargets cp whom => sameIntro (nomIntro (nomIntro bs cp) whom) []
    | .choose _ by_ n _ _ => sameIntro (chooseIntro bs by_ n) []
    | .choicesRevealed _ => sameIntro bs []
    | .vote _ _ _ _ => sameIntro bs [outcomeB .voteHeld]
    | .move what to _ =>
      ⟨nomIntro bs what, afterMoveTo to (moveIntro bs none what (some to.sort)), none, []⟩
    | .exchange what => sameIntro (what.intro bs) what.deed
    /- "Gains"/"loses" name the event outright [CR#119.3]; a set total leaves the gain or loss
    to follow from the new total [CR#119.5]. -/
    | .changeLife who (.up a) => sameIntro (Amount.intro (nomIntro bs who) a) [outcomeB .lifeGained]
    | .changeLife who (.down a) => sameIntro (Amount.intro (nomIntro bs who) a) [outcomeB .lifeLost]
    | .changeLife who (.set a) => sameIntro (Amount.intro (nomIntro bs who) a) []
    | .addMana who amt _ _ => sameIntro (Amount.intro (nomIntro bs who) amt) [outcomeB .manaAdded]
    | .draw who amt => sameIntro (Amount.intro (nomIntro bs who) amt) []
    | .expose _ who what => sameIntro (what.intro (nomIntro bs who)) []
    | .search who sc q p =>
      let bs' := nomIntro bs who
      sameIntro (Quantity.delta bs' q ++ Predicate.delta bs' p ++ sc.delta bs' ++ bs')
        [⟨.a, .object, q.plur,
          .object p.seedTy sc.zone (mkStamp (some (deedLabel .librarySearch)) none false) none
            none⟩]
    | .shuffle whose => ⟨nomIntro bs whose, afterShuffle (nomIntro bs whose), none, []⟩
    | .flipCoins who count => sameIntro (count.intro (nomIntro bs who)) [outcomeB .coinFlipped]
    | .rollDice who count _ => sameIntro (Amount.intro (nomIntro bs who) count) [outcomeB .rollResult]
    | .resultsTable _ => sameIntro bs []
    | .ignoreOutcomes which => sameIntro (which.intro bs) []
    | .shiftResult _ amt => sameIntro (Amount.intro bs amt) []
    | .storeResults on => sameIntro (nomIntro bs on) []
    | .rerollStored who _ whose => sameIntro (nomIntro (nomIntro bs who) whose) []
    | .continuously se _ => sameIntro (StaticSpec.intro bs se) []
    | .create agent count spec _ =>
      let bs' := Amount.intro (nomIntro bs agent) count
      sameIntro (spec.delta bs' ++ bs')
        [⟨.a, .object, outputPlur agent.plur count.plur,
          .object (spec.headTy bs') (some .battlefield) none (some .token) none⟩]
    | .getsEmblem who _ => sameIntro (nomIntro bs who) []
    | .putCounters amt kind on => sameIntro (nomIntro (kind.intro (Amount.intro bs amt)) on) []
    | .removeCounters q _ from_ => sameIntro (nomIntro (optQuantIntro bs q) from_) [outcomeB .countersRemoved]
    | .moveCounters amt _ src dst => sameIntro (nomIntro (nomIntro (Amount.intro bs amt) src) dst) []
    | .doubleCounters on => sameIntro (nomIntro bs on) []
    | .enact none v (.move what to _) =>
      ⟨nomIntro bs what, afterMoveTo to (moveIntro bs (some v) what (some to.sort)),
       some (stampIntro bs (some v) what), []⟩
    | .enact none v (.setStatus _ n) => ⟨nomIntro bs n, stampIntro bs (some v) n, none, []⟩
    | .enact none _ e => Instruction.profile bs e
    | .enact (some s) v e => doesProfile bs s.plur s v e (Instruction.profile (agentIntro bs s) e)
    | .pay who c .once => ⟨nomIntro bs who, Cost.intro (nomIntro bs who) c, none, []⟩
    | .pay who c _ => ⟨nomIntro bs who, Cost.intro (nomIntro bs who) c, none, [outcomeB .repeatCount]⟩
    | .may d body did notd =>
      let bodyP := Instruction.profile (mayCtx bs d) body
      mayProfile bodyP (Instruction.profileOpt bodyP.intro did) notd
    | .ifDone body did notd =>
      let bodyP := Instruction.profile bs body
      mayProfile bodyP (Instruction.profileOpt bodyP.intro did) notd
    | .onlyIf e _ _ => sameIntro (Instruction.profile bs e).announced []
    | .if_ _ _ _ => sameIntro bs []
    | .define l amt => sameIntro (defineLetter l (Amount.intro bs amt)) []
    | .forEachOf grp body =>
      let k := grp.kindOr .object
      let bs' := elemIntro bs k grp
      let bodyP := Instruction.profile bs' body
      ⟨bs, pluralizeDelta (bodyP.intro.take (bodyP.intro.length - bs'.length)) ++
        pluralizeDelta (NounPhrase.delta bs grp) ++ bs, none, []⟩
    | .forEachKindOf _ dom q body =>
      let bs' := kindValueIntro bs q dom
      let bodyP := Instruction.profile bs' body
      ⟨bs, pluralizeDelta (bodyP.intro.take (bodyP.intro.length - bs'.length)) ++
        pluralizeDelta (dom.elim [] (NounPhrase.delta bs)) ++ bs, none, []⟩
    | .repeat_ _ => sameIntro bs []
    | .repeated n body =>
      let bs' := Amount.intro bs n
      let bodyP := Instruction.profile bs' body
      ⟨bs', pluralizeDelta (bodyP.intro.take (bodyP.intro.length - bs'.length)) ++ bs', none,
       [outcomeB .repeatCount]⟩
    | .sequentially es => Instruction.seqProfile bs es
    | .simultaneously es => Instruction.simProfile bs es
    | .modal q _ => sameIntro (Quantity.delta bs q ++ bs) []
    | .delayed _ _ _ _ => sameIntro bs []
    | .reflexively body _ => Instruction.profile bs body
    | .thisWay body _ _ => Instruction.profile bs body
    | .insteadOf replaced _ => sameIntro (Instruction.profile bs replaced).announced []
    | .heldUntil e _ => sameIntro (Instruction.profile bs e).announced []
  termination_by structural e => e

  def Instruction.profileOpt (bs : Bindings) : Option Instruction → Option InstrProfile
    | none => none
    | some e => some (Instruction.profile bs e)
  termination_by structural e => e

  def Instruction.seqProfile (bs : Bindings) : List Instruction → InstrProfile
    | [] => ⟨bs, bs, none, []⟩
    | [e] => (Instruction.profile bs e).last
    | e :: es => Instruction.seqProfile (Instruction.profile bs e).intro es
  termination_by structural es => es

  def Instruction.simProfile (bs : Bindings) : List Instruction → InstrProfile
    | [] => ⟨bs, bs, none, []⟩
    | [e] => (Instruction.profile bs e).simLast
    | e :: es =>
      let ep := Instruction.profile bs e
      (Instruction.simProfile ep.announced es).simCons ep.deed
  termination_by structural es => es

  def Cost.intro (bs : Bindings) : Cost → Bindings
    | .mana c => if manaHasX c then letterB .x :: bs else bs
    | .scaled c _ => Cost.intro bs c
    | .loyaltySymbol .downX => letterB .x :: bs
    | .perform e => (Instruction.profile bs e).intro
    | .compound cs => Cost.costsIntro bs cs
    | _ => bs
  termination_by structural c => c

  def Cost.costsIntro (bs : Bindings) : List Cost → Bindings
    | [] => bs
    | c :: cs => Cost.costsIntro (Cost.intro bs c) cs
  termination_by structural cs => cs

  def StaticSpec.intro (bs : Bindings) : StaticSpec → Bindings
    | .definesLetter l amt => defineLetter l (Amount.intro bs amt)
    | .modify n _ d => Delta.delta (selfSubjIntro bs n) d ++ selfSubjIntro bs n
    | .definesPt n _ amt =>
      outcomeB .namedNumber :: (Amount.delta (selfSubjIntro bs n) amt ++ selfSubjIntro bs n)
    | .switchesPt n => selfSubjIntro bs n
    | .costs n sh => sh.delta (selfSubjIntro bs n) ++ selfSubjIntro bs n
    | .altCost n _ => selfSubjIntro bs n
    | .addedCost _ _ => bs
    | .gains n ab => ab.letterDelta ++ selfSubjIntro bs n
    | .gainsAbilitiesOf n _ src _ => nomIntro (nomIntro bs n) src
    | .deontic n _ _ _ _ _ _ _ => selfSubjIntro bs n
    | .skips who _ => nomIntro bs who
    | .keepsUnspentMana who _ => nomIntro bs who
    | .becomes n _ _ => selfSubjIntro bs n
    | .becomesCopy n _ _ => selfSubjIntro bs n
    | .losesAllAbilities n _ => selfSubjIntro bs n
    | .losesAbilities n _ => selfSubjIntro bs n
    | .gainsControl who what => stampIntro (nomIntro bs who) (featureLabel .controlGrant) what
    | .intercepts ev alts _ _ _ _ => interceptCtx bs alts ev
    | .damageRule _ src scope op _ => op.intro (scope.intro (src.intro bs))
    | .cantPrevent _ what _ => what.intro bs
    | .onlyDuring _ _ se => StaticSpec.intro bs se
    | .conditionally se c _ => c.intro (StaticSpec.intro bs se)
    | .alsoOffBattlefield se => StaticSpec.intro bs se
    | .doesntRemove se n => nomIntro (StaticSpec.intro bs se) n
    | .noLossFromZeroLife who => nomIntro bs who
    | .visibility _ who what => what.intro (nomIntro bs who)
    | .triggersAdditionally _ _ => bs
    | .entersRider n rider => rider.intro (nomIntro bs n)
    | .entersChoice n _ _ _ => selfSubjIntro bs n
    | .attachChoice n _ _ => selfSubjIntro bs n
    | .andAlso subject parts => StaticSpec.partsIntro (subjCtx bs subject) parts
  termination_by structural se => se

  def StaticSpec.partsIntro (bs : Bindings) : List StaticSpec → Bindings
    | [] => bs
    | se :: rest => StaticSpec.partsIntro (StaticSpec.intro bs se) rest
  termination_by structural parts => parts
end

def Instruction.intro (bs : Bindings) (e : Instruction) : Bindings := (e.profile bs).intro
def Instruction.preIntro (bs : Bindings) (e : Instruction) : Bindings := (e.profile bs).pre
def Instruction.annIntro (bs : Bindings) (e : Instruction) : Bindings := (e.profile bs).announced
def Instruction.riderIntro (bs : Bindings) (e : Instruction) : Bindings := (e.profile bs).riderCtx
def Instruction.deedDelta (bs : Bindings) (e : Instruction) : List Binding := (e.profile bs).deed
def Instruction.delta (bs : Bindings) (e : Instruction) : Bindings :=
  let out := e.intro bs
  out.take (out.length - bs.length)

def keepsOuter (bs : Bindings) (e : Instruction) : Bool := keepsOuterOf bs (e.intro bs)

def enactKeepsOuter (bs : Bindings) : Option NounPhrase → Instruction → Bool
  | none, _ => true
  | some s, e =>
    match s.plur with
    | .one => true
    | .many => eachStackOk (agentIntro bs s) (e.intro (agentIntro bs s))

def Instruction.annSeqs (bs : Bindings) : List Instruction → Bindings
  | [] => bs
  | e :: es => e.deedDelta bs ++ Instruction.annSeqs bs es

def Instruction.replacedCtx (bs : Bindings) : Instruction → Bindings
  | .sequentially es => Instruction.annSeqs bs es
  | .may d body _ _ => Instruction.replacedCtx (mayCtx bs d) body
  | .ifDone body _ _ => Instruction.replacedCtx bs body
  | .onlyIf e _ _ => Instruction.replacedCtx bs e
  | .if_ _ _ _ => bs
  | e => e.deedDelta bs ++ e.annIntro bs

def Instruction.otherwiseCtx (bs : Bindings) (e : Instruction) : Bindings :=
  outcomesOnly (e.deedDelta bs) ++ e.annIntro bs

def reflexCtx (bs : Bindings) (body : Instruction) : Bindings := settleTargets (body.intro bs)

def thisWayCtx (bs : Bindings) (body : Instruction) (ev : GameEvent) : Bindings :=
  settleTargets (GameEvent.after (body.intro bs) ev)

def QualityPayload.kind : QualityOp → QualityPayload → StaticKind
  | _, .colored _ => .colorSet
  | .adds, _ => .typeAddition
  | .sets, _ => .typeSet
  | .loses, _ => .typeLoss

def DamageOp.kind : DamageOp → StaticKind
  | .prevent _ _ => .prevention
  | _ => .replacement

def StaticSpec.kind : StaticSpec → StaticKind
  | .definesLetter _ _ => .letterDefinition
  | .modify _ _ d => deltaKind d
  | .definesPt _ _ _ => .ptDefinition
  | .switchesPt _ => .ptSwitch
  | .costs _ _ | .altCost _ _ | .addedCost _ _ => .costModification
  | .gains _ _ | .gainsAbilitiesOf _ _ _ _ => .keywordGrant
  | .deontic _ _ _ _ _ _ _ _ => .deedRestriction
  | .skips _ _ => .turnSkip
  | .keepsUnspentMana _ _ => .manaPersistence
  | .becomes _ op q => QualityPayload.kind op q
  | .becomesCopy _ _ _ => .copyEffect
  | .losesAllAbilities _ _ | .losesAbilities _ _ => .abilityLoss
  | .gainsControl _ _ => .controlGrant
  | .intercepts _ _ _ _ _ _ => .replacement
  | .damageRule _ _ _ op _ => op.kind
  | .cantPrevent _ _ _ => .prevention
  | .onlyDuring _ _ se => se.kind
  | .conditionally _ _ _ => .conditional
  | .alsoOffBattlefield se => se.kind
  | .doesntRemove se _ => se.kind
  | .noLossFromZeroLife _ => .outcomeImmunity
  | .visibility _ _ _ => .visibilityRider
  | .triggersAdditionally _ _ => .triggerMultiplier
  | .entersRider _ _ | .entersChoice _ _ _ _ => .entryRider
  | .attachChoice _ _ _ => .replacement
  | .andAlso _ _ => .coordination

def Instruction.namesThisDoor : Instruction → Bool
  | .delayed ev alts _ _ => ev.namesThisDoor || alts.any GameEvent.namesThisDoor
  | .heldUntil _ ev => ev.namesThisDoor
  | .thisWay _ ev _ => ev.namesThisDoor
  | _ => false

def Ability.namesThisDoor : Ability → Bool
  | .activated _ instr _ _ _ _ => instr.namesThisDoor
  | .triggered _ ev alts while_ joins _ _ _ instr =>
    ev.namesThisDoor || alts.any GameEvent.namesThisDoor || Concurrent.namesThisDoor while_ ||
      joins.any JoinedHeader.namesThisDoor || instr.namesThisDoor
  | .spell _ instr => instr.namesThisDoor
  | .italicHead _ ab => ab.namesThisDoor
  | .alsoForKeywords ab _ => ab.namesThisDoor
  | _ => false

mutual
  def Instruction.choiceDelta : Instruction → List Binding
    | .choose _ _ (.described (.a _) p) _ _ => choiceDeltaAt (p.kindOr .object)
    | .choose _ _ _ _ _ => []
    | .sequentially es => Instruction.choiceDeltaAll es
    | .may _ body _ _ => body.choiceDelta
    | .ifDone body _ _ => body.choiceDelta
    | _ => []
  termination_by structural e => e

  def Instruction.choiceDeltaAll : List Instruction → List Binding
    | [] => []
    | e :: es => Instruction.choiceDeltaAll es ++ e.choiceDelta
  termination_by structural es => es
end

def Cost.delta (bs : Bindings) (c : Cost) : List Binding :=
  let out := c.intro bs
  out.take (out.length - bs.length)

mutual
  def StaticSpec.choiceDelta (bs : Bindings) : StaticSpec → List Binding
    | .entersChoice _ q _ _ => [q.binding]
    | .attachChoice _ q _ => [q.binding]
    | .andAlso _ parts => StaticSpec.partsChoiceDelta bs parts
    | .addedCost c _ => c.delta bs
    | _ => []
  termination_by structural se => se

  def StaticSpec.partsChoiceDelta (bs : Bindings) : List StaticSpec → List Binding
    | [] => []
    | se :: rest => StaticSpec.partsChoiceDelta bs rest ++ StaticSpec.choiceDelta bs se
  termination_by structural parts => parts
end

/-- The stack after an ability line: the choices its text announced. -/
def Ability.intro (bs : Bindings) : Ability → Bindings
  | .keyword _ _ _ => bs
  | .activated _ instr _ _ _ _ => instr.choiceDelta ++ bs
  | .triggered _ _ _ _ _ _ _ _ instr => instr.choiceDelta ++ bs
  | .static se => se.choiceDelta bs ++ bs
  | .alsoForKeywords ab _ => Ability.intro bs ab
  | .italicHead _ ab => Ability.intro bs ab
  | .spell _ instr => instr.choiceDelta ++ bs
  | .mayBeginOnBattlefield => bs
  | .thatAbility _ => bs

end Semantics
