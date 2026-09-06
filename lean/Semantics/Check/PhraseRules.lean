import Semantics.Check.EventContext

/-!
# Semantics.Check.PhraseRules

The rules of the phrase layer: for each constructor, the obligations the Idris put in its
type, as a list of refusals over the stack it is read against. Every list is complete, not
first-failure, so a pin can state that one obligation and no other refuses.

`NounPhrase.check ctx bs n` takes the kind the context expects, if the Idris field fixed one
(`NounPhrase bs Player`), and refuses a `kindMismatch` where the noun fixes a different kind.
`Predicate.check k bs p` takes the resolved kind of the phrase it describes.
-/

namespace Semantics

/-- Refuse with `r` unless `ok`. -/
def refuse (ok : Bool) (r : Refusal) : List Refusal := if ok then [] else [r]

/-- The kind a phrase fixes must be the kind its context expects. -/
def kindCheck (expected : Kind) : Option Kind → List Refusal
  | none => []
  | some found => refuse (found == expected) (.kindMismatch expected found)

/-- The context's expected kind, if any, against the kind the noun fixes. -/
def ctxCheck (ctx : Option Kind) (fixed : Option Kind) : List Refusal :=
  match ctx with
  | none => []
  | some c => kindCheck c fixed

def CounterKind.check (c : CounterKind) : List Refusal := refuse c.known .knownCounter

/-- The halves of a hybrid or Phyrexian symbol are distinct [CR#107.4e,107.4f]. -/
def ManaSymbol.ok : ManaSymbol → Bool
  | .hybrid left right => halvesDistinct left right
  | .phyrexian color second => phyrexianDistinct color second
  | _ => true

def ManaSymbol.check (m : ManaSymbol) : List Refusal := refuse m.ok .manaSymbolOk

def ManaCost.check (c : ManaCost) : List Refusal := c.flatMap ManaSymbol.check

def ProjAxis.check : ProjAxis → List Refusal
  | .counter c => c.check
  | _ => []

def ChoiceMode.check (bs : Bindings) : ChoiceMode → List Refusal
  | .theirChoice w =>
    let n := countChoosers (view w bs)
    refuse (n == 1) (.chooserInScope n)
  | _ => []

def ColorTerm.check (bs : Bindings) : ColorTerm → List Refusal
  | .lit _ => []
  | .chosen ref =>
    let n := countChoice (.quality .color) bs
    refuse (ref.ok n) (.choiceRef ref (.quality .color) n)

/-- Idris `ChoiceDomain : ChoiceSort → Type`: the domain's sort must be the announced one. -/
def ChoiceDomain.sort : ChoiceDomain → ChoiceSort
  | .nameOfCard _ => .quality .cardName
  | .colorOtherThan _ => .quality .color
  | .typeOtherThan _ => .quality (.subtype .creature)
  | .basicTypesOnly | .nonbasicTypesOnly => .quality (.subtype .land)
  | .number _ => .quality .number
  | .players _ => .player
  | .abilitiesAmong _ => .quality .ability

/-- Internal checker output: diagnostics and use of the currently bound participant.
A nested lookback consumes its own gap use before returning to its parent. -/
structure CheckResult where
  refusals : List Refusal
  usedGap : Bool

instance : Coe (List Refusal) CheckResult := ⟨fun rs => ⟨rs, false⟩⟩

instance : HAppend CheckResult CheckResult CheckResult :=
  ⟨fun a b => ⟨a.refusals ++ b.refusals, a.usedGap || b.usedGap⟩⟩

instance : HAppend CheckResult (List Refusal) CheckResult :=
  ⟨fun a b => ⟨a.refusals ++ b, a.usedGap⟩⟩

instance : HAppend (List Refusal) CheckResult CheckResult :=
  ⟨fun a b => ⟨a ++ b.refusals, b.usedGap⟩⟩

/-- Aura hosts can be objects or players; the other attachment words require objects. -/
def attachmentHostKindCheck (word : Option AttachWord) (k : Kind) : List Refusal :=
  refuse (Kind.lte k (.join .object .player)) (.kindLte k (.join .object .player)) ++
    refuse (word.elim true (fun w => w == .enchanted || Kind.lte k .object)) .attachFits

mutual
  def Predicate.checkIn (gap : Option Kind) (k : Kind) (bs : Bindings) : Predicate → CheckResult
    | .hasType _ | .hasSubtype _ | .wasCast | .colorIs _ | .hasSupertype _
    | .isCard | .isToken | .isEmblem | .isCopyOfACard | .currentFace _
    | .hasStatus _ | .isSource | .isManaAbility => kindCheck k (some .object)
    | .manaCostHas m => kindCheck k (some .object) ++ ManaSymbol.check m
    | .anyPlayer | .opponent => kindCheck k (some .player)
    | .chosenPlayer ref =>
      let n := countChoice .player bs
      kindCheck k (some .player) ++ refuse (ref.ok n) (.choiceRef ref .player n)
    | .qualityNoun q dom => kindCheck k (some (.quality q)) ++ sortedDomainCheckIn gap bs (.quality
      q) dom
    | .counterKindOn n =>
      kindCheck k (some (.quality .counterKind)) ++ NounPhrase.checkIn gap (some .object) bs n ++
        refuse (zoneIsB (NounPhrase.zone bs n) .battlefield) (.zoneIs .battlefield)
    | .ofChosen ref q =>
      let n := countChoice (.quality q) bs
      kindCheck k (some .object) ++ refuse (ref.ok n) (.choiceRef ref (.quality q) n) ++
        refuse q.chosenReadOk (.chosenQualityRead q)
    | .ofYourChoice q dom =>
      kindCheck k (some .object) ++ refuse q.chosenReadOk (.chosenQualityRead q) ++
        sortedDomainCheckIn gap bs (.quality q) dom
    | .hasKeyword kt => kindCheck k (some .object) ++ refuse kt.known .knownKeywordTerm
    | .hasPossessor ax n =>
      NounPhrase.checkIn gap (some .player) bs n ++ refuse n.soleHolderOk .soleHolder ++
        refuse (possessorKind ax k) (.possessorKind ax k)
    | .castBy n rank =>
      kindCheck k (some .object) ++ NounPhrase.checkIn gap (some .player) bs n ++
        refuse n.soleHolderOk .soleHolder ++
        refuse (OptOrdinal.ok (rank.map (·.1))) .ordinalNonZero
    | .castFrom z =>
      kindCheck k (some .object) ++ ZoneExpr.checkIn gap bs z ++
        refuse (playableFrom (some z.sort)) .playableFrom
    | .inCombat r none => kindCheck k (some .object) ++ refuse r.bare .combatRelOk
    | .inCombat r (some m) =>
      NounPhrase.checkIn gap none bs m ++
        refuse (combatRelOk r k (m.kindOr .object) (NounPhrase.zone bs m) (NounPhrase.tys bs m)) .combatRelOk
    | .happenedTo lb => LookbackClause.checkIn gap k bs lb
    | .colorCount r n => kindCheck k (some .object) ++ refuse (colorBoundOk r n) .colorBoundOk
    | .named src => kindCheck k (some .object) ++ NameSource.checkIn gap bs src
    | .hasDesignation d holder =>
      refuse (d.holder == some k) (.designationHolder d k) ++
        refuse (holder.isNone || d.possessorOk) (.designationPossessorFits d) ++
        OptNoun.checkIn gap (some .player) bs holder
    | .attachment side word counterpart =>
      (match side with
       | .host => attachmentHostKindCheck word k
       | .attachment => kindCheck k (some .object)) ++
        (match counterpart with
         | none => (⟨[], false⟩ : CheckResult)
         | some n =>
           match side with
           | .host => NounPhrase.checkIn gap (some .object) bs n
           | .attachment => NounPhrase.checkIn gap none bs n ++
               attachmentHostKindCheck word (n.kindOr .object) ++
               refuse (attachWordsOk word.toList (NounPhrase.headTys bs n)) .attachFits)
    | .hasCounters kind =>
      kindCheck k (some .object) ++ OptCounterKind.checkIn gap kind ++
        refuse (counterKindNamed .object kind) (.counterKindNamed .object)
    | .compare axes _ bound =>
      refuse (axesAt k axes) (.axesAt k) ++ axes.flatMap ProjAxis.check ++ Amount.checkIn gap bs
        bound
    | .superlative op ax dom =>
      refuse op.isExtremal .isExtremal ++ refuse (ax.scope == k) (.projScope k) ++ ax.check ++
        Predicate.checkIn gap k bs dom
    | .withMostVotes =>
      let n := countOutcomes .voteHeld bs
      refuse (n == 1) (.outcomeInScope .voteHeld n)
    /- "each player who chose the highest number": a plural choice, not a vote [CR#701.38c],
    so the gate counts the pluralised number choice. -/
    | .choseExtreme op =>
      kindCheck k (some .player) ++ refuse op.isExtremal .isExtremal ++
        refuse (countManys (.quality .number) bs != 0) .numberChoiceInScope
    | .compareOver dom measure _ bound =>
      refuse k.phrasal (.phrasal k) ++ Predicate.checkIn gap k bs dom ++
        Amount.checkIn gap (bindFor .the .one k dom :: (Predicate.introduced bs dom ++ bs)) measure
          ++
        Amount.checkIn gap bs bound
    | .inZone z => kindCheck k (some .object) ++ ZoneExpr.checkIn gap bs z
    | .inPile pile =>
      kindCheck k (some .object) ++ NounPhrase.checkIn gap (some .pile) bs pile ++
        refuse pile.pileMention .pileMention
    | .exiledWith src =>
      kindCheck k (some .object) ++ NounPhrase.checkIn gap (some .object) bs src ++
        refuse src.linkSource .linkSource
    | .and ps =>
      Predicate.checkAllIn gap k bs ps ++ refuse (zonesOk ps) .zoneCoherent ++
        refuse (contradictionFree ps) .contradictionFree ++
        refuse (otherAnchorOk bs k (Predicate.headTysAll ps) ps) .otherAnchored
    | .or ps =>
      if Predicate.joins ps then
        kindCheck k (Predicate.kindOfAll ps) ++ Predicate.checkEachIn gap k bs ps
      else
        Predicate.checkAllIn gap k bs ps ++ refuse (!ps.isEmpty) .nonEmpty ++
          refuse (parallelDisjuncts ps) .parallelDisjuncts ++
          refuse (coordinableAll ps) .coordinableDisjuncts
    | .not p => Predicate.checkIn gap k bs p ++ refuse p.negatable .negatable
    | .other => refuse (anyTargeted k bs) (.anyTargeted k)
    | .notChosen => refuse (countParts k bs != 0) (.choiceInScope k)
    | .otherThan n =>
      NounPhrase.checkIn gap none bs n ++ refuse (n.anchorPhrase && n.plur.isOne) .complementAnchor
    | .coinCameUp _ =>
      refuse (coinFlipInScope bs) .coinFlipInScope ++
        refuse (Kind.lte k (.join .object .player)) (.kindLte k (.join .object .player))
    | .abilityHead cls =>
      kindCheck k (some .object) ++
        (match cls with
         | .keyword kw => refuse (knownKeyword kw) (.knownKeyword kw)
         | _ => (⟨[], false⟩ : CheckResult))
    | .abilityOf src => kindCheck k (some .object) ++ NounPhrase.checkIn gap (some .object) bs src
    | .activatedBy who =>
      kindCheck k (some .object) ++ NounPhrase.checkIn gap (some .player) bs who ++
        refuse who.soleHolderOk .soleHolder
    | .targets m _ =>
      let km := m.kindOr .object
      NounPhrase.checkIn gap none bs m ++ refuse km.targetable (.targetable km) ++
        refuse k.targeter (.targeter k)
  termination_by structural p => p

  def Predicate.checkAllIn (gap : Option Kind) (k : Kind) (bs : Bindings) : List Predicate →
    CheckResult
    | [] => (⟨[], false⟩ : CheckResult)
    | p :: ps => Predicate.checkIn gap k bs p ++ Predicate.checkAllIn gap k bs ps
  termination_by structural ps => ps

  /-- Each disjunct of a joined disjunction is checked at the kind it names. -/
  def Predicate.checkEachIn (gap : Option Kind) (k : Kind) (bs : Bindings) : List Predicate →
    CheckResult
    | [] => (⟨[], false⟩ : CheckResult)
    | p :: ps => Predicate.checkIn gap (p.kindOr k) bs p ++ Predicate.checkEachIn gap k bs ps
  termination_by structural ps => ps

  /-- Check both the contents and the sort of a choice domain at every consumer. -/
  def sortedDomainCheckIn (gap : Option Kind) (bs : Bindings) (q : ChoiceSort) : Option ChoiceDomain
    → CheckResult
    | none => (⟨[], false⟩ : CheckResult)
    | some d =>
      (match d with
       | .nameOfCard p => Predicate.checkIn gap .object [] p
       | .typeOtherThan s => refuse (s.type == some .creature) .subtypeType
       | .number n => refuse n.wellFormed .wellFormedQ ++ Quantity.checkIn gap bs n
       | .players p => Predicate.checkIn gap .player bs p
       | .abilitiesAmong ks =>
         refuse (!ks.isEmpty) .nonEmpty ++ refuse (allKnownKeywordTerms ks) .knownKeywordTerm
       | _ => (⟨[], false⟩ : CheckResult)) ++ refuse (d.sort == q) .kindAxisSort
  termination_by structural d => d

  def OptCounterKind.checkIn (_gap : Option Kind) : Option CounterKind → CheckResult
    | none => (⟨[], false⟩ : CheckResult)
    | some c => c.check

  def NounPhrase.checkIn (gap : Option Kind) (ctx : Option Kind) (bs : Bindings) : NounPhrase →
    CheckResult
    | .gap k => ⟨ctxCheck ctx (some k) ++ refuse (gap == some k) .lookbackSubject, true⟩
    | .this | .theGrantor _ => ctxCheck ctx (some .object)
    | .you | .combatPlayer _ | .playerGroup _ => ctxCheck ctx (some .player)
    | .asType t n sub =>
      ctxCheck ctx (some .object) ++ NounPhrase.checkIn gap (some .object) bs n ++
        refuse n.ascribable .ascribable ++ refuse (ascriptionOk t sub) .ascriptionOk
    | .asMarker _ n =>
      ctxCheck ctx (some .object) ++ NounPhrase.checkIn gap (some .object) bs n ++
        refuse n.ascribable .ascribable
    | .resolvedPermanent spell =>
      ctxCheck ctx (some .object) ++ NounPhrase.checkIn gap (some .object) bs spell ++
        refuse (zoneIsB (NounPhrase.zone bs spell) .stack) (.zoneIs .stack) ++
        refuse (permanentSpellType (NounPhrase.ty bs spell)) .permanentSpellType
    | .described d p =>
      let k := p.kindOr (ctx.getD .object)
      ctxCheck ctx p.kind? ++ refuse k.phrasal (.phrasal k) ++ Predicate.checkIn gap k bs p ++
        DetPhrase.checkIn gap k bs d p
    | .eachOf g =>
      NounPhrase.checkIn gap ctx bs g ++ refuse (g.plur == .many) .plural ++
        refuse g.groupMention .groupMention
    | .and ns =>
      ctxCheck ctx (some (NounPhrase.kindOfAll ns)) ++
        refuse (atLeastTwo ns.length) .atLeastTwo ++ NounPhrase.checkCoordIn true gap bs ns
    | .or ns =>
      ctxCheck ctx (some (NounPhrase.kindOfAll ns)) ++
        refuse (atLeastTwo ns.length) .atLeastTwo ++ NounPhrase.checkCoordIn false gap bs ns
    | .librarySlice _ amt whose =>
      ctxCheck ctx (some .object) ++ Amount.checkIn gap bs amt ++ NounPhrase.checkIn gap (some
        .player) bs whose ++
        refuse whose.slicePossessorOk .slicePossessor
    | .someOf q d g =>
      ctxCheck ctx (some .object) ++ SliceCount.checkIn gap bs q ++ OptPredicate.checkIn gap .object
        bs d ++
        NounPhrase.checkIn gap (some .object) bs g ++ refuse g.partitiveBase .partitiveBase
    | .namesAgree _ g =>
      ctxCheck ctx (some .object) ++ NounPhrase.checkIn gap (some .object) bs g ++
        refuse g.countedMention .countedMention ++ refuse (g.plur == .many) .plural
    | .theRest k pl => ctxCheck ctx (some k) ++ refuse (theRestFits k pl bs) (.theRestFits k)
    | .pileOf q by_ =>
      let n := countReach (.word .pile) .many bs
      ctxCheck ctx (some .pile) ++ SliceCount.checkIn gap bs q ++ OptNoun.checkIn gap (some .player)
        bs by_ ++
        refuse (n == 1) (.anaphor (.word .pile) .many n)
    | .pro r pl w =>
      let n := countReach r pl (view w bs)
      ctxCheck ctx (some r.kind) ++ refuse (n == 1) (.anaphor r pl n)
    | .attachHost w h => ctxCheck ctx (some h.kind) ++ refuse (attachHeadOk w h) .attachHeadOk
    | .possessorOf ax n =>
      let kn := n.kindOr .object
      ctxCheck ctx (some .player) ++ NounPhrase.checkIn gap none bs n ++
        refuse (possessorKind ax kn) (.possessorKind ax kn)
    | .designated d whose =>
      ctxCheck ctx (some .object) ++ NounPhrase.checkIn gap (some .player) bs whose ++
        refuse d.heldByItsCard (.designationScope d)
    | .oneEachOf roles pool =>
      ctxCheck ctx (some .object) ++ Predicate.checkAllIn gap .object bs roles ++
        NounPhrase.checkIn gap (some .object) bs pool ++ refuse (pool.plur == .many) .plural ++
        refuse (rolesOk roles) .rolesOk
  termination_by structural n => n

  def NounPhrase.checkCoordIn (sequential : Bool) (gap : Option Kind) (bs : Bindings) :
      List NounPhrase → CheckResult
    | [] => (⟨[], false⟩ : CheckResult)
    | n :: ns => NounPhrase.checkIn gap none bs n ++
        NounPhrase.checkCoordIn sequential gap (if sequential then nomIntro bs n else bs) ns
  termination_by structural ns => ns

  def OptNoun.checkIn (gap : Option Kind) (ctx : Option Kind) (bs : Bindings) : Option NounPhrase →
    CheckResult
    | none => (⟨[], false⟩ : CheckResult)
    | some n => NounPhrase.checkIn gap ctx bs n
  termination_by structural n => n

  def OptPredicate.checkIn (gap : Option Kind) (k : Kind) (bs : Bindings) : Option Predicate →
    CheckResult
    | none => (⟨[], false⟩ : CheckResult)
    | some p => Predicate.checkIn gap k bs p
  termination_by structural p => p

  /-- Idris `detOk`, plus the choice-mode reads inside a determiner. -/
  def DetPhrase.checkIn (gap : Option Kind) (k : Kind) (bs : Bindings) (d : DetPhrase) (p :
    Predicate) : CheckResult :=
    match d with
    | .target q =>
      refuse q.nonZero .nonZeroQ ++ refuse q.wellFormed .wellFormedQ ++
        refuse k.targetable (.targetable k) ++ Quantity.checkIn gap bs q
    | .count q m =>
      refuse q.nonZero .nonZeroQ ++ refuse q.wellFormed .wellFormedQ ++ Quantity.checkIn gap bs q ++
        (match m with
         | none => (⟨[], false⟩ : CheckResult)
         | some m => ChoiceMode.check bs m)
    | .a m => ChoiceMode.check bs m
    | .the => refuse p.uniquifies .uniquifying
    | _ => (⟨[], false⟩ : CheckResult)

  def Quantity.checkIn (gap : Option Kind) (bs : Bindings) : Quantity → CheckResult
    | .range _ _ => (⟨[], false⟩ : CheckResult)
    | .upToOf a => Amount.checkIn gap bs a
    | .exactlyOf a => Amount.checkIn gap bs a
  termination_by structural q => q

  def SliceCount.checkIn (gap : Option Kind) (bs : Bindings) : SliceCount → CheckResult
    | .counted q => refuse q.nonZero .nonZeroQ ++ refuse q.wellFormed .wellFormedQ ++
      Quantity.checkIn gap bs q
    | .whole => (⟨[], false⟩ : CheckResult)
  termination_by structural q => q

  def Amount.checkIn (gap : Option Kind) (bs : Bindings) : Amount → CheckResult
    | .lit _ | .letter _ => (⟨[], false⟩ : CheckResult)
    | .statOf axis n =>
      axis.check ++ NounPhrase.checkIn gap (some axis.scope) bs n ++ refuse n.plur.isOne .singular
        ++
        (match axis with
         | .stat c => refuse (statHeadTysOk c (NounPhrase.headTys bs n)) .statHeadTysOk
         | _ => (⟨[], false⟩ : CheckResult))
    | .countOf g =>
      NounPhrase.checkIn gap none bs g ++ refuse (g.plur == .many) .plural ++
        refuse g.countableGroup .countableGroup
    | .aggregate _ ax g =>
      let k := g.kindOr .object
      NounPhrase.checkIn gap none bs g ++ ax.check ++ refuse (ax.scope == k) (.projScope k) ++
        refuse (g.plur == .many) .plural
    | .paid f n =>
      refuse f.named .paidFacetNamed ++ NounPhrase.checkIn gap (some .object) bs n ++
        refuse (n.paidSubjectOk bs) .paidSubject ++ refuse n.plur.isOne .singular
    | .eventTally op who lb =>
      NounPhrase.checkIn gap none bs who ++ LookbackClause.checkIn gap (who.kindOr .object)
        (nomIntro bs who) lb ++
        refuse (tallyOk op lb.event) .tallyOk
    | .arith op a b =>
      Amount.checkIn gap bs a ++ Amount.checkIn gap (Amount.intro bs a) b ++
        refuse (op != .times || a.nonZero) .amtNonZero
    | .thatMuch =>
      let n := countAmountOutcomes bs
      refuse (n == 1) (.quantOutcomeInScope n)
    | .chosenNumber ref =>
      let n := countChoice (.quality .number) bs
      refuse (ref.ok n) (.choiceRef ref (.quality .number) n)
    | .votesFor _ =>
      let n := countOutcomes .voteHeld bs
      refuse (n == 1) (.outcomeInScope .voteHeld n)
    | .theOutcome s =>
      let n := countOutcomes s bs
      refuse (n == 1) (.outcomeInScope s n)
    | .coinsShowing _ => refuse (coinFlipInScope bs) .coinFlipInScope
    | .greatestStoredMatch n => NounPhrase.checkIn gap (some .object) bs n ++ refuse n.plur.isOne
      .singular
    | .groupSize =>
      let n := countManysAny bs
      refuse (n == 1) (.groupSizeInScope n)
    | .theDifference =>
      let n := countOnes .gap bs
      refuse (n == 1) (.gapInScope n)
    | .devotion who c d =>
      NounPhrase.checkIn gap (some .player) bs who ++ refuse who.plur.isOne .singular ++
        ColorTerm.check bs c ++
        (match d with
         | none => (⟨[], false⟩ : CheckResult)
         | some d => ColorTerm.check bs d)
    | .half _ a => Amount.checkIn gap bs a
    | .aggregateOver _ dom body =>
      let k := dom.kindOr .object
      refuse k.phrasal (.phrasal k) ++ Predicate.checkIn gap k bs dom ++
        Amount.checkIn gap (bindFor .the .one k dom :: (Predicate.introduced bs dom ++ bs)) body
    | .distinctCount ax dom =>
      refuse ax.ok .kindAxisSort ++ NounPhrase.checkIn gap (some .object) bs dom
    | .upTo b => Amount.checkIn gap bs b
  termination_by structural a => a

  def ZoneScope.checkIn (gap : Option Kind) (z : Zone) (bs : Bindings) : ZoneScope → CheckResult
    | .bare => (⟨[], false⟩ : CheckResult)
    | .possessedBy n => refuse z.possessable (.possessable z) ++ NounPhrase.checkIn gap (some
      .player) bs n
  termination_by structural s => s

  def LibraryPlace.checkIn (gap : Option Kind) (bs : Bindings) : LibraryPlace → CheckResult
    | .eitherEnd chooser =>
      OptNoun.checkIn gap (some .player) bs chooser ++ refuse (eventAgentOk bs chooser) .eventAgent
    | _ => (⟨[], false⟩ : CheckResult)
  termination_by structural p => p

  def ZoneExpr.checkIn (gap : Option Kind) (bs : Bindings) : ZoneExpr → CheckResult
    | .zone z scope => ZoneScope.checkIn gap z bs scope
    | .library place ord off scope =>
      LibraryPlace.checkIn gap bs place ++ refuse (place.arrangementOk ord) .placeArrangementFits ++
        refuse (place.ordinalOk off) .placeOrdinalFits ++
        refuse (OptOrdinal.ok off) .ordinalNonZero ++ ZoneScope.checkIn gap .library bs scope
  termination_by structural z => z

  def ZoneExpr.checkAllIn (gap : Option Kind) (bs : Bindings) : List ZoneExpr → CheckResult
    | [] => (⟨[], false⟩ : CheckResult)
    | z :: zs => ZoneExpr.checkIn gap bs z ++ ZoneExpr.checkAllIn gap bs zs
  termination_by structural zs => zs

  def NameSource.checkIn (gap : Option Kind) (bs : Bindings) : NameSource → CheckResult
    | .printed _ => (⟨[], false⟩ : CheckResult)
    | .chosen =>
      let n := countChoice (.quality .cardName) bs
      refuse (n == 1) (.choiceRef .theChoice (.quality .cardName) n)
    | .sameAs n => NounPhrase.checkIn gap (some .object) bs n
  termination_by structural s => s

  def EventSource.checkIn (gap : Option Kind) (bs : Bindings) : EventSource → CheckResult
    | .anywhere => (⟨[], false⟩ : CheckResult)
    | .zones zs => ZoneExpr.checkAllIn gap bs zs
    | .anywhereBut zs => ZoneExpr.checkAllIn gap bs zs
  termination_by structural s => s

  def LookbackClause.checkIn (_gap : Option Kind) (k : Kind) (bs : Bindings) : LookbackClause →
    CheckResult
    | .mk ev _ =>
      let checked := GameEvent.checkIn (some k) bs ev
      ⟨checked.refusals ++ refuse checked.usedGap .lookbackSubject, false⟩
  termination_by structural lb => lb


  def Condition.checkIn (gap : Option Kind) (bs : Bindings) : Condition → CheckResult
    | .exists_ n => NounPhrase.checkIn gap none bs n ++ refuse n.existentialMention
      .existentialMention
    | .happened who lb =>
      NounPhrase.checkIn gap none bs who ++ LookbackClause.checkIn gap (who.kindOr .object)
        (nomIntro bs who) lb
    | .gameIs d =>
      refuse d.gameWide (.designationScope d) ++ refuse d.checked (.designationChecked d)
    | .noHolder d =>
      refuse (d.scope == some (.heldBy .player)) (.designationScope d) ++
        refuse d.checked (.designationChecked d)
    | .matches n p =>
      let k := n.kindOr .object
      NounPhrase.checkIn gap none bs n ++ Predicate.checkIn gap k bs p ++ refuse (n.testSubjectOk
        bs) .testSubject ++
        refuse p.says .predSays ++ refuse (zoneFits (NounPhrase.zone bs n) p.seedZone) .zoneFits ++
        refuse (attachWordsOk p.attachWordsIn (NounPhrase.headTys bs n)) .attachFits
    | .compareAmt subj _ bound =>
      Amount.checkIn gap bs subj ++ Amount.checkIn gap (Amount.intro bs subj) bound ++
        refuse subj.read .readAmount
    | .dealtThisWay p =>
      let k := p.kindOr .object
      Predicate.checkIn gap k bs p ++ refuse (damageDealtInScope bs) .damageDealtInScope ++
        refuse (Kind.lte k (.join .object .player)) (.kindLte k (.join .object .player)) ++
        refuse (!zoneIsB p.seedZone .stack) (.zoneIs .stack)
    | .choseThisWay who p =>
      let k := p.kindOr .object
      NounPhrase.checkIn gap (some .player) bs who ++ Predicate.checkIn gap k bs p ++
        refuse (countParts k bs != 0) (.choiceInScope k)
    | .preventedFromSource p =>
      let n := countOutcomes .damagePrevented bs
      Predicate.checkIn gap .object bs p ++ refuse (n == 1) (.outcomeInScope .damagePrevented n)
    | .flipCalled who _ => NounPhrase.checkIn gap (some .player) bs who ++ refuse (coinFlipInScope
      bs) .coinFlipInScope
    | .flipFace _ => refuse (coinFlipInScope bs) .coinFlipInScope
    | .voteLead _ _ =>
      let n := countOutcomes .voteHeld bs
      refuse (n == 1) (.outcomeInScope .voteHeld n)
    | .anyResultIs _ bound =>
      let n := countOutcomes .rollResult bs
      Amount.checkIn gap bs bound ++ refuse (n == 1) (.outcomeInScope .rollResult n)
    | .rolledDoubles =>
      let n := countOutcomes .rollResult bs
      refuse (n == 1) (.outcomeInScope .rollResult n)
    | .not c => Condition.checkIn gap bs c
    | .and cs =>
      Condition.checkAllIn gap bs cs ++ refuse (atLeastTwoCs cs) .atLeastTwo ++
        refuse (cs.all fun c => !c.isAnd) .flatConjuncts
    | .or cs =>
      Condition.checkAllIn gap bs cs ++ refuse (atLeastTwoCs cs) .atLeastTwo ++
        refuse (cs.all fun c => !c.isOr) .flatDisjuncts
  termination_by structural c => c

  def Condition.checkAllIn (gap : Option Kind) (bs : Bindings) : List Condition → CheckResult
    | [] => (⟨[], false⟩ : CheckResult)
    | c :: cs => Condition.checkIn gap bs c ++ Condition.checkAllIn gap bs cs
  termination_by structural cs => cs



def AttackDefender.checkIn (gap : Option Kind) (bs : Bindings) : Option NounPhrase → CheckResult
  | none => (⟨[], false⟩ : CheckResult)
  | some m =>
    NounPhrase.checkIn gap none bs m ++ refuse m.plur.isOne .singular ++ refuse (m.attackable bs)
      .attackable

def DamagePatient.checkIn (gap : Option Kind) (bs : Bindings) : Option NounPhrase → CheckResult
  | none => (⟨[], false⟩ : CheckResult)
  | some m => NounPhrase.checkIn gap none bs m ++ refuse (m.damageRecipient bs) .damageRecipient

def Door.checkIn (gap : Option Kind) (bs : Bindings) : Door → CheckResult
  | .thisDoor => (⟨[], false⟩ : CheckResult)
  | .doorOf _ room =>
    NounPhrase.checkIn gap (some .object) bs room ++
      refuse (zoneIsB (NounPhrase.zone bs room) .battlefield) (.zoneIs .battlefield)

def RollWatch.checkIn (gap : Option Kind) (bs : Bindings) : RollWatch → CheckResult
  | .resultIn q =>
    refuse q.nonZero .nonZeroQ ++ refuse q.wellFormed .wellFormedQ ++ refuse q.literal .quantLiteral
      ++
      Quantity.checkIn gap bs q
  | _ => (⟨[], false⟩ : CheckResult)

def HeaderPossessor.checkIn (gap : Option Kind) (bs : Bindings) : HeaderPossessor → CheckResult
  | .noPossessor => (⟨[], false⟩ : CheckResult)
  | .byPlayer n => NounPhrase.checkIn gap (some .player) bs n
  | .byTurn n => NounPhrase.checkIn gap (some .turnRef) bs n

def ManaTypeTerm.checkIn (_gap : Option Kind) (bs : Bindings) : ManaTypeTerm → CheckResult
  | .colorless => (⟨[], false⟩ : CheckResult)
  | .ofColor c => ColorTerm.check bs c

def OptManaTypeTerm.checkIn (gap : Option Kind) (bs : Bindings) : Option ManaTypeTerm → CheckResult
  | none => (⟨[], false⟩ : CheckResult)
  | some t => ManaTypeTerm.checkIn gap bs t

def OptEventSource.checkIn (gap : Option Kind) (bs : Bindings) : Option EventSource → CheckResult
  | none => (⟨[], false⟩ : CheckResult)
  | some src => EventSource.checkIn gap bs src

def OptZoneExpr.checkIn (gap : Option Kind) (bs : Bindings) : Option ZoneExpr → CheckResult
  | none => (⟨[], false⟩ : CheckResult)
  | some z => ZoneExpr.checkIn gap bs z

  def GameEvent.checkIn (gap : Option Kind) (bs : Bindings) : GameEvent → CheckResult
    | .dies n =>
      NounPhrase.checkIn gap (some .object) bs n ++ refuse (zoneIsB (NounPhrase.zone bs n)
        .battlefield) (.zoneIs .battlefield)
    | .leaves n from_ =>
      NounPhrase.checkIn gap (some .object) bs n ++ OptEventSource.checkIn gap bs from_ ++
        refuse (zoneFits (NounPhrase.zone bs n) (sourceZone from_)) .zoneFits
    | .isDealtDamage _ to => NounPhrase.checkIn gap none bs to ++ refuse (to.damageRecipient bs)
      .damageRecipient
    | .draws who => NounPhrase.checkIn gap (some .player) bs who
    | .losesGame who => NounPhrase.checkIn gap (some .player) bs who
    | .enters n from_ =>
      NounPhrase.checkIn gap (some .object) bs n ++ OptEventSource.checkIn gap bs from_ ++
        refuse (zoneFits (NounPhrase.zone bs n) (some .battlefield)) .zoneFits ++
        refuse (entrySourceOk from_) .lookbackSource
    | .combat .attackerOf n whom =>
      NounPhrase.checkIn gap none bs n ++ AttackDefender.checkIn gap (nomIntro bs n) whom ++
        refuse (n.attackerOk bs) .attacker
    | .combat r n counterpart =>
      let bs' := nomIntro bs n
      NounPhrase.checkIn gap (some .object) bs n ++ OptNoun.checkIn gap (some .object) bs'
        counterpart ++
        refuse (zoneIsB (NounPhrase.zone bs n) .battlefield) (.zoneIs .battlefield) ++
        refuse (patientZoneIsB bs' counterpart .battlefield) (.zoneIs .battlefield) ++
        refuse r.eventRelation .combatRelOk
    | .attacksWith who whom attackers =>
      let bs' := nomIntro bs who
      let bs'' := optIntro bs' whom
      NounPhrase.checkIn gap (some .player) bs who ++ AttackDefender.checkIn gap bs' whom ++
        NounPhrase.checkIn gap (some .object) bs'' attackers ++
        refuse (zoneIsB (NounPhrase.zone bs'' attackers) .battlefield) (.zoneIs .battlefield)
    | .attachment .attached n host =>
      let kh := host.kindOr .object
      NounPhrase.checkIn gap (some .object) bs n ++ NounPhrase.checkIn gap none (nomIntro bs n) host
        ++
        refuse (zoneIsB (NounPhrase.zone bs n) .battlefield) (.zoneIs .battlefield) ++
        refuse (Kind.lte kh (.join .object .player)) (.kindLte kh (.join .object .player))
    | .attachment .unattached n host =>
      NounPhrase.checkIn gap (some .object) bs n ++ NounPhrase.checkIn gap (some .object) (nomIntro
        bs n) host ++
        refuse (zoneIsB (NounPhrase.zone bs n) .battlefield) (.zoneIs .battlefield)
    | .dealsDamage kind n to =>
      NounPhrase.checkIn gap (some .object) bs n ++ DamagePatient.checkIn gap (nomIntro bs n) to ++
        refuse (zoneFits (NounPhrase.zone bs n) kind.sourceZone) .zoneFits
    | .beginningOf _ part whose =>
      HeaderPossessor.checkIn gap bs whose ++ refuse (part.proper && whose.ok) .windowOk
    | .casts who what from_ =>
      let bs' := nomIntro bs who
      NounPhrase.checkIn gap (some .player) bs who ++ OptNoun.checkIn gap (some .object) bs' what ++
        OptEventSource.checkIn gap (optIntro bs' what) from_ ++
        (match what with
         | none => (⟨[], false⟩ : CheckResult)
         | some n =>
           refuse (zoneIsB (NounPhrase.zone bs' n) .stack) (.zoneIs .stack) ++
             refuse n.plur.isOne .singular ++ refuse (!n.targeted) .nontarget) ++
        refuse (castSourceOk from_) .playableFrom
    | .becomesTarget n by_ =>
      let kn := n.kindOr .object
      let kb := by_.kindOr .object
      NounPhrase.checkIn gap none bs n ++ NounPhrase.checkIn gap none (nomIntro bs n) by_ ++
        refuse kn.targetable (.targetable kn) ++ refuse kb.targeter (.targeter kb)
    | .statusEvent n v =>
      NounPhrase.checkIn gap (some .object) bs n ++
        refuse (zoneIsB (NounPhrase.zone bs n) .battlefield) (.zoneIs .battlefield) ++
        refuse v.markable .statusMarkable
    | .gameBecomes d => refuse d.gameWide (.designationScope d)
    | .stateHolds c => Condition.checkIn gap bs c
    | .putInto n to from_ =>
      NounPhrase.checkIn gap (some .object) bs n ++ ZoneExpr.checkIn gap bs to ++
        OptEventSource.checkIn gap bs from_ ++
        refuse (putDestOk to) .lookbackDest ++ refuse (putSourceOk from_) .lookbackSource ++
        refuse (zoneFits (NounPhrase.zone bs n) (sourceZone from_)) .zoneFits
    | .counterEvent dir kind n batch by_ byEffect =>
      let k := n.kindOr .object
      NounPhrase.checkIn gap none bs n ++ OptCounterKind.checkIn gap kind ++
        refuse (counterKindNamed k kind) (.counterKindNamed k) ++
        OptNoun.checkIn gap (some .player) bs by_ ++ refuse (eventAgentOk bs by_) .eventAgent ++
        refuse (counterBatchOk batch dir kind) .counterBatchOk ++ refuse (causedByOk byEffect by_)
          .causedByOk
    | .tokensCreated n byEffect by_ under =>
      NounPhrase.checkIn gap (some .object) bs n ++ refuse n.tokenPhrase .tokenPhrase ++
        OptNoun.checkIn gap (some .player) bs by_ ++ OptNoun.checkIn gap (some .player) bs under ++
        refuse (creationVoiceOk bs byEffect by_ under) .verbedVoiceOk
    | .chapterMark ns => refuse (chapterMarksOk ns) .chapterMarks
    | .activates who what =>
      NounPhrase.checkIn gap (some .player) bs who ++ NounPhrase.checkIn gap (some .object)
        (nomIntro bs who) what ++
        refuse what.plur.isOne .singular ++ refuse (!what.targeted) .nontarget
    | .statBecomes n _ v =>
      NounPhrase.checkIn gap (some .object) bs n ++ Amount.checkIn gap (nomIntro bs n) v ++
        refuse (zoneIsB (NounPhrase.zone bs n) .battlefield) (.zoneIs .battlefield)
    | .flipsCoin who _ => NounPhrase.checkIn gap (some .player) bs who
    | .rollsDice who _ _ res => NounPhrase.checkIn gap (some .player) bs who ++ RollWatch.checkIn
      gap bs res
    | .paysCost who _ whose kw =>
      OptNoun.checkIn gap (some .player) bs who ++ NounPhrase.checkIn gap (some .object)
        (optAgentIntro bs who) whose ++
        refuse (keywordCosts kw) (.keywordCost kw) ++ refuse whose.plur.isOne .singular
    | .paysLife who => NounPhrase.checkIn gap (some .player) bs who
    | .lifeChanges who _ => NounPhrase.checkIn gap (some .player) bs who
    | .verbedEvent who v what becomes locus =>
      let bs' := optAgentIntro bs who
      OptNoun.checkIn gap (some .player) bs who ++ OptNoun.checkIn gap (some .object) bs' what ++
        OptPredicate.checkIn gap .object bs' becomes ++ refuse (knownAct v) (.knownAct v) ++
        refuse (verbPatientOk v what) .verbPatientOk ++
        refuse (zoneFits (patientZone bs' what) (actZoneOf v)) .zoneFits ++
        refuse (verbedVoiceOk v who what) .verbedVoiceOk ++
        refuse (verbBecomesOk v becomes) .verbBecomesOk ++
        OptZoneExpr.checkIn gap bs' locus ++
        refuse (!actNamesLocus v || locus.isSome) .complementWritten ++
        (match locus with
         | none => (⟨[], false⟩ : CheckResult)
         | some z => refuse ((actLociOf v).elem z.sort) .lookbackLocus)
    | .tappedForMana who what ty =>
      let bs' := optAgentIntro bs who
      OptNoun.checkIn gap (some .player) bs who ++ NounPhrase.checkIn gap (some .object) bs' what ++
        refuse (zoneFits (NounPhrase.zone bs' what) (some .battlefield)) .zoneFits ++
        OptManaTypeTerm.checkIn gap (nomIntro bs' what) ty
    | .unlocksDoor who door => NounPhrase.checkIn gap (some .player) bs who ++ Door.checkIn gap
      (nomIntro bs who) door
    | .nthOccurrence ordinal _ ev =>
      refuse ordinal.ok .ordinalNonZero ++ GameEvent.checkIn gap bs ev
    | .triggers what =>
      NounPhrase.checkIn gap (some .object) bs what ++ refuse what.plur.isOne .singular ++
        refuse (!what.targeted) .nontarget
    | .commitsCrime who => NounPhrase.checkIn gap (some .player) bs who ++ refuse who.plur.isOne
      .singular
    | .causes by_ what => Causing.checkIn gap bs by_ ++ GameEvent.checkIn gap (by_.intro bs) what
  termination_by structural ev => ev

  def Causing.checkIn (gap : Option Kind) (bs : Bindings) : Causing → CheckResult
    | .source src => NounPhrase.checkIn gap none bs src
    | .event ev => GameEvent.checkIn gap bs ev
    | .anEffect => (⟨[], false⟩ : CheckResult)
  termination_by structural c => c
end

abbrev Predicate.check := fun k bs value => (Predicate.checkIn none k bs value).refusals
abbrev Predicate.checkAll := fun k bs value => (Predicate.checkAllIn none k bs value).refusals
abbrev Predicate.checkEach := fun k bs value => (Predicate.checkEachIn none k bs value).refusals
abbrev sortedDomainCheck := fun bs q value => (sortedDomainCheckIn none bs q value).refusals
abbrev OptCounterKind.check := fun value => (OptCounterKind.checkIn none value).refusals
abbrev NounPhrase.check := fun ctx bs value => (NounPhrase.checkIn none ctx bs value).refusals
abbrev OptNoun.check := fun ctx bs value => (OptNoun.checkIn none ctx bs value).refusals
abbrev OptPredicate.check := fun k bs value => (OptPredicate.checkIn none k bs value).refusals
abbrev DetPhrase.check := fun k bs d p => (DetPhrase.checkIn none k bs d p).refusals
abbrev Quantity.check := fun bs value => (Quantity.checkIn none bs value).refusals
abbrev SliceCount.check := fun bs value => (SliceCount.checkIn none bs value).refusals
abbrev Amount.check := fun bs value => (Amount.checkIn none bs value).refusals
abbrev ZoneScope.check := fun z bs value => (ZoneScope.checkIn none z bs value).refusals
abbrev LibraryPlace.check := fun bs value => (LibraryPlace.checkIn none bs value).refusals
abbrev ZoneExpr.check := fun bs value => (ZoneExpr.checkIn none bs value).refusals
abbrev ZoneExpr.checkAll := fun bs value => (ZoneExpr.checkAllIn none bs value).refusals
abbrev NameSource.check := fun bs value => (NameSource.checkIn none bs value).refusals
abbrev EventSource.check := fun bs value => (EventSource.checkIn none bs value).refusals
abbrev LookbackClause.check := fun k bs value => (LookbackClause.checkIn none k bs value).refusals
abbrev Condition.check := fun bs value => (Condition.checkIn none bs value).refusals
abbrev Condition.checkAll := fun bs value => (Condition.checkAllIn none bs value).refusals
abbrev AttackDefender.check := fun bs value => (AttackDefender.checkIn none bs value).refusals
abbrev DamagePatient.check := fun bs value => (DamagePatient.checkIn none bs value).refusals
abbrev Door.check := fun bs value => (Door.checkIn none bs value).refusals
abbrev RollWatch.check := fun bs value => (RollWatch.checkIn none bs value).refusals
abbrev HeaderPossessor.check := fun bs value => (HeaderPossessor.checkIn none bs value).refusals
abbrev ManaTypeTerm.check := fun bs value => (ManaTypeTerm.checkIn none bs value).refusals
abbrev OptManaTypeTerm.check := fun bs value => (OptManaTypeTerm.checkIn none bs value).refusals
abbrev OptEventSource.check := fun bs value => (OptEventSource.checkIn none bs value).refusals
abbrev OptZoneExpr.check := fun bs value => (OptZoneExpr.checkIn none bs value).refusals
abbrev GameEvent.check := fun bs value => (GameEvent.checkIn none bs value).refusals
abbrev Causing.check := fun bs value => (Causing.checkIn none bs value).refusals

def OptCondition.check (bs : Bindings) : Option Condition → List Refusal
  | none => []
  | some c => Condition.check bs c

def SearchScope.check (bs : Bindings) : SearchScope → List Refusal
  | .oneZone z => ZoneExpr.check bs z
  | .someZones whose zs =>
    OptNoun.check (some .player) bs whose ++ refuse (atLeastTwoZones zs) .atLeastTwoZones

def FlipScope.check (bs : Bindings) : FlipScope → List Refusal
  | .count n => Amount.check bs n
  | .per each =>
    let k := each.kindOr .object
    NounPhrase.check none bs each ++ refuse (each.plur == .many) .plural ++
      refuse (Kind.lte k (.join .object .player)) (.kindLte k (.join .object .player))

def IgnoredOutcomes.check (bs : Bindings) : IgnoredOutcomes → List Refusal
  | .chosen chooser n =>
    OptNoun.check (some .player) bs chooser ++ refuse (eventAgentOk bs chooser) .eventAgent ++
      Amount.check bs n
  | _ => []

def Ballot.check (bs : Bindings) : Ballot → List Refusal
  | .byLabel opts => refuse (ballotLabelsOk opts) .ballotLabelsOk
  | .byCandidate n => NounPhrase.check none bs n ++ refuse n.choosable .choosable

def Exposed.check (bs : Bindings) : Exposed → List Refusal
  | .cards n => NounPhrase.check (some .object) bs n
  | .zone z => ZoneExpr.check bs z ++ refuse z.sort.exposable (.exposableZone z.sort)
  | .choice q =>
    let n := countChoice q bs
    refuse (n == 1) (.choiceRef .theChoice q n)

def VisibleThing.check (bs : Bindings) : VisibleThing → List Refusal
  | .objects n => NounPhrase.check (some .object) bs n
  | _ => []

end Semantics
