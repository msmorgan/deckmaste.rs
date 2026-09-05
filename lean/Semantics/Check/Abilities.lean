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
  | .zones a b => ZoneExpr.introduced (ZoneExpr.introduced bs a ++ bs) b ++ ZoneExpr.introduced bs a ++ bs
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
  | .lit n => n == 1
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
  | some (p, t) => Amount.introduced (Amount.intro bs p) t ++ Amount.introduced bs p

def Stat.modifyOk : Stat → Bool
  | .power | .toughness => true
  | _ => false

def CostShift.introduced (bs : Bindings) : CostShift → List Binding
  | .less a _ => Amount.introduced bs a
  | .more a => Amount.introduced bs a
  | .run _ _ _ => []

def gatePayer : Binding := ⟨.the, .one, .player false⟩

def CountBound.introduced (bs : Bindings) : CountBound → List Binding
  | .moreThan k => Amount.introduced bs k
  | .additional q => Quantity.introduced bs q

def deonticBoundOk (bs : Bindings) (ds : Deeds) : Option CountBound → Bool
  | none => true
  | some b => ds.all deedBoundedOk && (b.introduced bs).isEmpty

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
  | .pro r pl .whole => !r.tracksObject || countReach r pl (NounPhrase.introduced bs n) == 0
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

mutual
  /-- Whether a player phrase can only name opponents: "an opponent", "your opponents", "each
  opponent". -/
  def Predicate.opponentOnly : Predicate → Bool
    | .opponent => true
    | .and ps => Predicate.anyOpponentOnly ps
    | _ => false
  def Predicate.anyOpponentOnly : List Predicate → Bool
    | [] => false
    | p :: ps => p.opponentOnly || Predicate.anyOpponentOnly ps
end

def NounPhrase.opponentOnly : NounPhrase → Bool
  | .described _ p => p.opponentOnly
  | .playerGroup .yourOpponents => true
  | .eachOf g => g.opponentOnly
  | _ => false

/-- The library a look opens: the possessor of the slice looked at. -/
def Instruction.lookedLibraryOwner : Instruction → Option NounPhrase
  | .sequentially (.expose .lookAt _ (.cards (.librarySlice _ _ whose)) :: _) => some whose
  | .expose .lookAt _ (.cards (.librarySlice _ _ whose)) => some whose
  | _ => none

/-- A deed the table marks as opening an opponent's library looks at one [CR#701.29a]. -/
def enactLibraryOwnerOk (v : Deed) (e : Instruction) : Bool :=
  !actOpponentsLibrary v || e.lookedLibraryOwner.elim true NounPhrase.opponentOnly

/-- A deed done by name happens where the deed table says its patient lives ("destroy" on the
battlefield [CR#701.8a], "discard" from a hand [CR#701.9a]); "this" is wherever the text is.
The Idris carried this on each verb's macro. -/
def enactPatientZoneOk (bs : Bindings) (v : Deed) (e : Instruction) : Bool :=
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
def enactAgentOk : Option NounPhrase → Deed → Bool
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
  pluralizeIntroduced (out.take (out.length - (agentIntro bs s).length))

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
    | .compare axes _ bound => axes.all ProjAxis.deck && bound.deckBound
    | .and ps | .or ps => Predicate.deckReadableAll ps
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

def Characteristics.typed (c : Characteristics) : Bool := !c.types.isEmpty
def Characteristics.ptOk (c : Characteristics) : Bool := !c.types.elem .creature || ptWritten c.pt
def Characteristics.lineNonEmpty (c : Characteristics) : Bool :=
  hasAnyTypeCharacteristic c.supertypes c.types c.subtypes
def Characteristics.canonical (c : Characteristics) : Bool :=
  colorsDistinct c.colors && typesDistinct c.types && supersDistinct c.supertypes
def Characteristics.additionUnnamed (c : Characteristics) : Bool := c.name.isNone
def Characteristics.lossWritesTypes (c : Characteristics) : Bool :=
  c.pt.isNone && c.colors.isEmpty && c.text.isEmpty && c.name.isNone && c.lineNonEmpty
def Characteristics.headTy (c : Characteristics) : Option CardType := lastType c.types
def Characteristics.copyBundleSays (c : Characteristics) : Bool :=
  let said := (if ptWritten c.pt then 1 else 0) + (if !c.colors.isEmpty then 1 else 0) +
    (if c.lineNonEmpty then 1 else 0)
  said ≥ 2

def Ability.grantable : Ability → Bool
  | .keyword _ _ _ | .activated _ _ _ _ _ _ | .triggered _ _ _ _ _ _ _ _ | .static _ => true
  | .italicHead _ ab => ab.grantable
  | .thatAbility _ => true
  | _ => false

def Characteristics.abilitiesOk (c : Characteristics) : Bool := c.text.all Ability.grantable

/-- Every quality an effect adds is hosted by a type the set writes. -/
def CharacteristicBundle.qualsFit (b : CharacteristicBundle) : Bool :=
  b.qualities.all (TokenQuality.hosted b.characteristics.types)
def CharacteristicBundle.lossWritesTypes (b : CharacteristicBundle) : Bool :=
  b.characteristics.lossWritesTypes && b.qualities.isEmpty

def bundleOk (op : QualityOp) (ty : Option CardType) (z : Option Zone)
    (b : CharacteristicBundle) (ret : Option CardType) : Bool :=
  let t := b.characteristics
  match op with
  | .adds =>
    (addsSomething ty t.types t.subtypes || !t.supertypes.isEmpty) &&
      addedFits ty t.types t.subtypes && t.canonical && t.abilitiesOk && b.qualsFit &&
      t.additionUnnamed && ret.isNone
  | .sets =>
    zoneIsB z .battlefield && t.lineNonEmpty && addedFits ty t.types t.subtypes &&
      t.abilitiesOk && t.canonical && b.qualsFit && retentionOk t.types ret
  | .loses => zoneIsB z .battlefield && b.lossWritesTypes && t.canonical && ret.isNone

def becomesOk (bs : Bindings) (op : QualityOp) (n : NounPhrase) : QualityPayload → Bool
  | .bundle t ret => bundleOk op (NounPhrase.ty bs n) (NounPhrase.zone bs n) t ret
  | .everyTypeOf space => zoneIsB (NounPhrase.zone bs n) .battlefield && spaceHosted space (NounPhrase.ty bs n)
  | .chosenQuality q => q.qualityReadOk && hostedRead bs q n
  | .colored cs => cs.ok && colorOpOk op cs

def TokenSpec.headTy (bs : Bindings) : TokenSpec → Option CardType
  | .written t => t.characteristics.headTy
  | .asThose => tyOfThoseAny .token bs
  | .copyOf src _ => NounPhrase.ty bs src

def TokenSpec.introduced (bs : Bindings) : TokenSpec → List Binding
  | .written t => ptDelta bs t.characteristics.pt
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
  | some (.triggered ev _ _ _ _ _ _ _) => keywordBodied k && keywordStackRegime k == bodyEventRegime ev
  | some _ => false

def Ability.grantedKeyword : Ability → Option KeywordLabel
  | .keyword k none _ => if keywordParamless k then some k else none
  | _ => none

def StaticSpec.keyword : StaticSpec → Option KeywordLabel
  | .conditionally se _ _ => se.keyword
  | .gains _ ab => ab.grantedKeyword
  | _ => none
termination_by structural se => se

def Instruction.keyword : Instruction → Option KeywordLabel
  | .continuously se _ => se.keyword
  | _ => none

def Ability.lineKeyword : Ability → Option KeywordLabel
  | .static se => se.keyword
  | .triggered _ _ _ _ _ _ _ instr => instr.keyword
  | _ => none

def Ability.keywordExtendable (ab : Ability) : Bool := ab.lineKeyword.isSome

def keywordListOk (ab : Ability) (ks : List KeywordTerm) : Bool :=
  match ab.lineKeyword with
  | none => false
  | some base => !ks.isEmpty && allTermsBare ks && distinctTerms ks && !ks.elem (.the base)

def Ability.emblemOk : Ability → Bool
  | .activated _ _ _ _ _ _ | .triggered _ _ _ _ _ _ _ _ | .static _ => true
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

def Ability.introducedLetters : Ability → List Binding
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

/-! How many times a cost turns the source itself over [CR#107.5,107.6]. -/
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

def Cost.tapOnce : Cost → Bool
  | .scaled c _ => c.tapOnce
  | .compound cs => Cost.selfTapCount cs ≤ 1
  | .either l r => l.tapOnce && r.tapOnce
  | _ => true
termination_by structural c => c

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

/-- A keyword whose parameter cost the controller pays ("cumulative upkeep {2}", "ward {2}")
takes a cost payable by you; a deed done by another player is not a payment of it. -/
def keywordCostPaidByYou (k : KeywordLabel) : Option KeywordParam → Bool
  | some (.cost c) => !(keywordFactsFor k).elim false (·.paidCost) || c.paidByYou
  | _ => true


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

def Cost.payable : Cost → Bool
  | .scaled c _ => c.payable
  | .tapSymbol | .untapSymbol | .loyaltySymbol _ => false
  | .either l r => l.payable && r.payable
  | _ => true
termination_by structural c => c

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
def doesProfile (bs : Bindings) (pl : Plurality) (s : NounPhrase) (v : Deed) (e : Instruction)
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
      sameIntro (nomIntro (Amount.introduced (selfSubjIntro bs src) amt ++ nomIntro bs src) to)
        [outcomeB .damageDealt]
    | .controllerSacrifices n =>
      ⟨⟨.the, .one, .player false⟩ :: selfSubjIntro bs n,
       ⟨.the, .one, .player false⟩ ::
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
    | .doesntUntapNext n steps => sameIntro (Amount.introduced bs steps ++ nomIntro bs n) []
    | .skipsNext w _ count => sameIntro (Amount.introduced bs count ++ nomIntro bs w) []
    | .extraTurn w count =>
      ⟨Amount.introduced bs count ++ nomIntro bs w, turnRefB :: (Amount.introduced bs count ++ nomIntro bs w),
       none, []⟩
    | .additionalPart who _ _ count _ => sameIntro (Amount.introduced bs count ++ optAgentIntro bs who) []
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
       [⟨.the, .many, .pile (NounPhrase.zone bs' grp) (some piles) (pileMentionFace faces)⟩]⟩
    | .counterSpell what => sameIntro (nomIntro bs what) []
    | .copy src agent what times _ =>
      let bs' := nomIntro bs agent
      let k := what.kindOr .object
      sameIntro (Amount.intro (nomIntro bs' what) times)
        [⟨.the, outputPlur what.plur times.plur,
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
      sameIntro (Quantity.introduced bs' q ++ Predicate.introduced bs' p ++ sc.introduced bs' ++ bs')
        [⟨.a, q.plur,
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
      sameIntro (spec.introduced bs' ++ bs')
        [⟨.a, outputPlur agent.plur count.plur,
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
      ⟨bs, pluralizeIntroduced (bodyP.intro.take (bodyP.intro.length - bs'.length)) ++
        pluralizeIntroduced (NounPhrase.introduced bs grp) ++ bs, none, []⟩
    | .forEachKindOf _ dom q body =>
      let bs' := kindValueIntro bs q dom
      let bodyP := Instruction.profile bs' body
      ⟨bs, pluralizeIntroduced (bodyP.intro.take (bodyP.intro.length - bs'.length)) ++
        pluralizeIntroduced (dom.elim [] (NounPhrase.introduced bs)) ++ bs, none, []⟩
    | .repeat_ _ => sameIntro bs []
    | .repeated n body =>
      let bs' := Amount.intro bs n
      let bodyP := Instruction.profile bs' body
      ⟨bs', pluralizeIntroduced (bodyP.intro.take (bodyP.intro.length - bs'.length)) ++ bs', none,
       [outcomeB .repeatCount]⟩
    | .sequentially es => Instruction.seqProfile bs es
    | .simultaneously es => Instruction.simProfile bs es
    | .modal q _ => sameIntro (Quantity.introduced bs q ++ bs) []
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
    | .modify n _ d => Delta.introduced (selfSubjIntro bs n) d ++ selfSubjIntro bs n
    | .definesPt n _ amt =>
      outcomeB .namedNumber :: (Amount.introduced (selfSubjIntro bs n) amt ++ selfSubjIntro bs n)
    | .switchesPt n => selfSubjIntro bs n
    | .costs n sh => sh.introduced (selfSubjIntro bs n) ++ selfSubjIntro bs n
    | .altCost n _ => selfSubjIntro bs n
    | .addedCost _ _ => bs
    | .gains n ab => ab.introducedLetters ++ selfSubjIntro bs n
    | .gainsAbilitiesOf n _ src _ => nomIntro (nomIntro bs n) src
    | .deontic n _ _ _ _ _ _ _ => selfSubjIntro bs n
    | .skips who _ => nomIntro bs who
    | .keepsUnspentMana who _ => nomIntro bs who
    | .becomes n _ _ => selfSubjIntro bs n
    | .becomesCopy n _ _ => selfSubjIntro bs n
    | .losesAllAbilities n _ => selfSubjIntro bs n
    | .losesAbilities n _ => selfSubjIntro bs n
    | .gainsControl who what => stampIntro (nomIntro bs who) (some (.core .gainControl)) what
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
def Instruction.introducedDeeds (bs : Bindings) (e : Instruction) : List Binding := (e.profile bs).deed
def keepsOuter (bs : Bindings) (e : Instruction) : Bool := keepsOuterOf bs (e.intro bs)

def enactKeepsOuter (bs : Bindings) : Option NounPhrase → Instruction → Bool
  | none, _ => true
  | some s, e =>
    match s.plur with
    | .one => true
    | .many => eachStackOk (agentIntro bs s) (e.intro (agentIntro bs s))

def Instruction.annSeqs (bs : Bindings) : List Instruction → Bindings
  | [] => bs
  | e :: es => e.introducedDeeds bs ++ Instruction.annSeqs bs es

def Instruction.replacedCtx (bs : Bindings) : Instruction → Bindings
  | .sequentially es => Instruction.annSeqs bs es
  | .may d body _ _ => Instruction.replacedCtx (mayCtx bs d) body
  | .ifDone body _ _ => Instruction.replacedCtx bs body
  | .onlyIf e _ _ => Instruction.replacedCtx bs e
  | .if_ _ _ _ => bs
  | e => e.introducedDeeds bs ++ e.annIntro bs

def Instruction.otherwiseCtx (bs : Bindings) (e : Instruction) : Bindings :=
  outcomesOnly (e.introducedDeeds bs) ++ e.annIntro bs

def reflexCtx (bs : Bindings) (body : Instruction) : Bindings := settleTargets (body.intro bs)

def thisWayCtx (bs : Bindings) (body : Instruction) (ev : GameEvent) : Bindings :=
  settleTargets (GameEvent.after (body.intro bs) ev)

def Instruction.namesThisDoor : Instruction → Bool
  | .delayed ev alts _ _ => ev.namesThisDoor || alts.any GameEvent.namesThisDoor
  | .heldUntil _ ev => ev.namesThisDoor
  | .thisWay _ ev _ => ev.namesThisDoor
  | _ => false

def Ability.namesThisDoor : Ability → Bool
  | .activated _ instr _ _ _ _ => instr.namesThisDoor
  | .triggered ev alts while_ joins _ _ _ instr =>
    ev.namesThisDoor || alts.any GameEvent.namesThisDoor || Concurrent.namesThisDoor while_ ||
      joins.any JoinedHeader.namesThisDoor || instr.namesThisDoor
  | .spell _ instr => instr.namesThisDoor
  | .italicHead _ ab => ab.namesThisDoor
  | .alsoForKeywords ab _ => ab.namesThisDoor
  | _ => false

mutual
  def Instruction.introducedChoices : Instruction → List Binding
    | .choose _ _ (.described (.a _) p) _ _ => introducedChoiceAt (p.kindOr .object)
    | .choose _ _ _ _ _ => []
    | .sequentially es => Instruction.introducedChoicesAll es
    | .may _ body _ _ => body.introducedChoices
    | .ifDone body _ _ => body.introducedChoices
    | _ => []
  termination_by structural e => e

  def Instruction.introducedChoicesAll : List Instruction → List Binding
    | [] => []
    | e :: es => Instruction.introducedChoicesAll es ++ e.introducedChoices
  termination_by structural es => es
end

def Cost.introduced (bs : Bindings) (c : Cost) : List Binding :=
  let out := c.intro bs
  out.take (out.length - bs.length)

mutual
  def StaticSpec.introducedChoices (bs : Bindings) : StaticSpec → List Binding
    | .entersChoice _ q _ _ => [q.binding]
    | .attachChoice _ q _ => [q.binding]
    | .andAlso _ parts => StaticSpec.partsIntroducedChoices bs parts
    | .addedCost c _ => c.introduced bs
    | _ => []
  termination_by structural se => se

  def StaticSpec.partsIntroducedChoices (bs : Bindings) : List StaticSpec → List Binding
    | [] => []
    | se :: rest => StaticSpec.partsIntroducedChoices bs rest ++ StaticSpec.introducedChoices bs se
  termination_by structural parts => parts
end

/-- The stack after an ability line: the choices its text announced. -/
def Ability.intro (bs : Bindings) : Ability → Bindings
  | .keyword _ _ _ => bs
  | .activated _ instr _ _ _ _ => instr.introducedChoices ++ bs
  | .triggered _ _ _ _ _ _ _ instr => instr.introducedChoices ++ bs
  | .static se => se.introducedChoices bs ++ bs
  | .alsoForKeywords ab _ => Ability.intro bs ab
  | .italicHead _ ab => Ability.intro bs ab
  | .spell _ instr => instr.introducedChoices ++ bs
  | .mayBeginOnBattlefield => bs
  | .thatAbility _ => bs


/-! ## Number slots [CR#107.1b]

The declaration surface for numbers: where a constructor's own `Amount` arguments sit in the
two regimes of [CR#107.1b]. A `clamped` slot reads a result below zero as zero — you can't
deal negative damage, gain negative life, draw −1 cards, or choose a negative number. A
`signed` slot lets the negative stand: an effect that *sets*, doubles, or triples a life total
or a creature's power and toughness, and any raw value read or comparison, which is a
calculation and not the result of an effect.

Each function lists only its own constructor's direct slots. It does not descend into a child
`Instruction`, `StaticSpec`, `Ability`, `Cost`, or `TokenSpec` — each of those declares its
own — nor into an `Amount` expression, whose inner reads are calculations. Every arm is
written out, so a constructor that gains an `Amount` cannot slip in unclassified.
-/

/-- A payload a constructor holds directly, when it holds one at all. -/
def optSlots {α : Type} (f : α → List (Amount × NumberRegime)) :
    Option α → List (Amount × NumberRegime)
  | none => []
  | some x => f x

/-- An `Option Amount` slot in the clamped regime. -/
def optClamped : Option Amount → List (Amount × NumberRegime)
  | none => []
  | some amount => [(amount, .clamped)]

/-- A boost or a penalty ("gets +2/+0", "gets −X/−0") is a signed change whose magnitude is
clamped: the sign is the constructor, so the amount inside cannot go below zero [CR#107.1b].
`set` is the exception the rule names — an effect that sets a life total or a power and
toughness reads a negative. -/
def Delta.numberSlots : Delta Amount → List (Amount × NumberRegime)
  | .up amount | .down amount => [(amount, .clamped)]
  | .set amount => [(amount, .signed)]

/-- Exchanging two numerical values [CR#701.12g] sets each side to the other: a set of a life
total or a power and toughness, so both stand below zero [CR#107.1b]. -/
def Exchanged.numberSlots : Exchanged → List (Amount × NumberRegime)
  | .lifeTotals _ | .controlOf _ _ | .cardsAcross _ _ | .zones _ _ | .textBoxes _ _ => []
  | .values left right => [(left, .signed), (right, .signed)]

/-- A cost shift is an amount of mana, and a floor is the least it can be reduced to: both are
counts, so neither goes below zero [CR#107.1b]. -/
def CostShift.numberSlots : CostShift → List (Amount × NumberRegime)
  | .less amount floor => (amount, .clamped) :: optClamped floor
  | .more amount => [(amount, .clamped)]
  | .run _ _ _ => []

/-- "more than one creature each combat": a count of deeds [CR#107.1b]. -/
def CountBound.numberSlots : CountBound → List (Amount × NumberRegime)
  | .moreThan amount => [(amount, .clamped)]
  | .additional _ => []

/-- "as though its power were greater than 4" compares a game value, and a comparison uses a
negative when it needs one [CR#107.1b]. -/
def AsThough.numberSlots : AsThough → List (Amount × NumberRegime)
  | .of _ | .mana _ _ _ => []
  | .greater _ amount => [(amount, .signed)]

/-- How much damage a prevention shield stops: damage is never negative [CR#107.1b]. -/
def PreventCut.numberSlots : PreventCut → List (Amount × NumberRegime)
  | .all | .half _ => []
  | .some amount | .shield amount | .allBut amount => [(amount, .clamped)]

/-- Doubling and tripling damage carries its factor as a `ScaleFactor`, not an `Amount`, so it
declares no slot. A shift is a magnitude with the direction as its sign, and the damage it
adjusts is not negative [CR#107.1b]. -/
def DamageScale.numberSlots : DamageScale → List (Amount × NumberRegime)
  | .multiplied _ | .halved _ => []
  | .shifted _ amount => [(amount, .clamped)]

/-- A damage replacement's own slots. The `also` instruction a prevention runs declares its
own. -/
def DamageOp.numberSlots : DamageOp → List (Amount × NumberRegime)
  | .prevent cut _ | .redirect cut _ => cut.numberSlots
  | .scale damageScale => damageScale.numberSlots

/-- "two more times": a count of repetitions [CR#107.1b]. -/
def Repetition.numberSlots : Repetition → List (Amount × NumberRegime)
  | .again | .anyNumber | .untilCond _ | .againExcludingChosen => []
  | .moreTimes times => [(times, .clamped)]

/-- How many coins are flipped: a count [CR#107.1b]. -/
def FlipScope.numberSlots : FlipScope → List (Amount × NumberRegime)
  | .count amount => [(amount, .clamped)]
  | .per _ => []

/-- How many results a player may ignore: a count [CR#107.1b]. -/
def IgnoredOutcomes.numberSlots : IgnoredOutcomes → List (Amount × NumberRegime)
  | .extreme _ | .allBut _ => []
  | .chosen _ amount => [(amount, .clamped)]

/-- A rider's own slots: how many counters it enters with, a count [CR#107.1b]. The copy
exceptions of `asCopyOf` declare their own. -/
def TokenRider.numberSlots : TokenRider → List (Amount × NumberRegime)
  | .entersAs _ | .entersAttacking _ | .entersTransformed | .entersMelded _ => []
  | .withCounters amount _ _ => [(amount, .clamped)]
  | .under _ | .asCopyOf _ _ _ => []

def TokenRider.ridersSlots : List TokenRider → List (Amount × NumberRegime)
  | [] => []
  | rider :: rest => rider.numberSlots ++ TokenRider.ridersSlots rest

/-- A cost's own slots: "for each" multiplies a cost by a count [CR#107.1b]. A nested cost and
a performed instruction declare their own. -/
def Cost.numberSlots : Cost → List (Amount × NumberRegime)
  | .mana _ => []
  | .scaled _ amount => [(amount, .clamped)]
  | .tapSymbol | .untapSymbol | .loyaltySymbol _ => []
  | .perform _ | .compound _ | .either _ _ | .itsManaCost => []

/-- The static spec's own slots and the regime each is read in [CR#107.1b]. -/
def StaticSpec.numberSlots : StaticSpec → List (Amount × NumberRegime)
  | .modify _ _ delta => delta.numberSlots
  -- An effect that sets a power and toughness, the exception [CR#107.1b] names.
  | .definesPt _ _ amount => [(amount, .signed)]
  | .switchesPt _ => []
  | .costs _ shift => shift.numberSlots
  | .altCost _ _ | .addedCost _ _ => []
  -- The letter X, defined by the text: a chosen or defined value is never negative
  -- [CR#107.1b,107.3].
  | .definesLetter _ amount => [(amount, .clamped)]
  | .gains _ _ | .gainsAbilitiesOf _ _ _ _ => []
  | .deontic _ _ _ _ bound _ asThough _ =>
    optSlots CountBound.numberSlots bound ++ optSlots AsThough.numberSlots asThough
  | .keepsUnspentMana _ _ | .skips _ _ | .becomes _ _ _ => []
  | .alsoOffBattlefield _ | .doesntRemove _ _ => []
  | .becomesCopy _ _ _ | .losesAllAbilities _ _ | .losesAbilities _ _ | .gainsControl _ _ => []
  | .intercepts _ _ _ _ _ _ => []
  | .damageRule _ _ _ op _ => op.numberSlots
  | .cantPrevent _ _ _ => []
  | .conditionally _ _ _ | .onlyDuring _ _ _ => []
  | .noLossFromZeroLife _ | .visibility _ _ _ | .triggersAdditionally _ _ => []
  | .entersRider _ rider => rider.numberSlots
  | .entersChoice _ _ _ _ | .attachChoice _ _ _ | .andAlso _ _ => []

/-- The instruction's own slots and the regime each is read in [CR#107.1b]. -/
def Instruction.numberSlots : Instruction → List (Amount × NumberRegime)
  -- You can't deal negative damage [CR#107.1b].
  | .dealDamage _ amount _ => [(amount, .clamped)]
  | .fights _ _ | .setStatus _ _ | .turnOver _ | .removeFromCombat _ => []
  | .attachTo _ _ | .unattach _ | .becomesBlocking _ _ | .stopsBlocking _ _ => []
  | .becomesAttacking _ _ | .regenerate _ | .cantBe _ _ _ => []
  | .gainsDesignation _ _ _ _ | .unlock _ | .gameBecomes _ | .concludes _ _ => []
  | .gameDrawn | .restartsGame | .separateIntoPiles _ _ _ _ => []
  | .choose _ _ _ _ _ | .choicesRevealed _ | .vote _ _ _ _ => []
  | .move _ _ riders => TokenRider.ridersSlots riders
  | .counterSpell _ => []
  -- How many copies to make: a count [CR#107.1b].
  | .copy _ _ _ times _ => [(times, .clamped)]
  | .chooseNewTargets _ | .copyTargets _ _ => []
  -- Gaining and losing life are clamped; "your life total becomes …" is the set the rule
  -- excepts, and doubling a life total is written as that set [CR#107.1b].
  | .changeLife _ delta => delta.numberSlots
  | .exchange exchanged => exchanged.numberSlots
  | .addMana _ amount _ _ => [(amount, .clamped)]
  | .draw _ amount => [(amount, .clamped)]
  | .expose _ _ _ | .search _ _ _ _ | .shuffle _ => []
  | .flipCoins _ count => count.numberSlots
  | .rollDice _ count _ => [(count, .clamped)]
  | .resultsTable _ => []
  | .ignoreOutcomes which => which.numberSlots
  -- The sign is the direction; the amount added to a result is a magnitude [CR#107.1b].
  | .shiftResult _ amount => [(amount, .clamped)]
  | .storeResults _ | .rerollStored _ _ _ | .continuously _ _ => []
  | .create _ count _ riders => (count, .clamped) :: TokenRider.ridersSlots riders
  | .getsEmblem _ _ => []
  | .putCounters amount _ _ => [(amount, .clamped)]
  | .distribute _ amount _ => [(amount, .clamped)]
  | .removeCounters _ _ _ => []
  | .moveCounters amount _ _ _ => [(amount, .clamped)]
  | .doubleCounters _ => []
  | .losesCounters _ _ amount => optClamped amount
  | .enact _ _ _ | .controllerSacrifices _ | .pay _ _ _ => []
  | .may _ _ _ _ | .ifDone _ _ _ | .onlyIf _ _ _ | .if_ _ _ _ => []
  -- The letter X, defined by the text [CR#107.1b,107.3]: a defined X is a count, not a
  -- game value, so a calculation below zero reads as zero.
  | .define _ amount => [(amount, .clamped)]
  | .forEachOf _ _ | .forEachKindOf _ _ _ _ => []
  | .repeat_ repetition => repetition.numberSlots
  | .repeated times _ => [(times, .clamped)]
  | .sequentially _ | .simultaneously _ | .modal _ _ => []
  | .delayed _ _ _ _ | .insteadOf _ _ | .heldUntil _ _ => []
  | .reflexively _ _ | .thisWay _ _ _ => []
  | .doesntUntapNext _ steps => [(steps, .clamped)]
  | .skipsNext _ _ count => [(count, .clamped)]
  | .extraTurn _ count => [(count, .clamped)]
  | .additionalPart _ _ _ count _ => [(count, .clamped)]

end Semantics
