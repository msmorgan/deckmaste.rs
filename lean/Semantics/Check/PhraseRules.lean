import Semantics.Check.Phrase

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

mutual
  def Predicate.check (k : Kind) (bs : Bindings) : Predicate → List Refusal
    | .hasType _ | .hasSubtype _ | .wasCast | .colorIs _ | .hasSupertype _ | .isAttached _
    | .isCard | .isToken | .isEmblem | .isCopyOfACard | .isTransformed
    | .hasStatus _ | .isSource | .isManaAbility => kindCheck k (some .object)
    | .manaCostHas m => kindCheck k (some .object) ++ ManaSymbol.check m
    | .anyPlayer | .opponent => kindCheck k (some .player)
    | .chosenPlayer ref =>
      let n := countChoice .player bs
      kindCheck k (some .player) ++ refuse (ref.ok n) (.choiceRef ref .player n)
    | .qualityNoun q dom => kindCheck k (some (.quality q)) ++ OptChoiceDomain.check bs dom
    | .counterKindOn n =>
      kindCheck k (some (.quality .counterKind)) ++ NounPhrase.check (some .object) bs n ++
        refuse (zoneIsB (NounPhrase.zone bs n) .battlefield) (.zoneIs .battlefield)
    | .ofChosen ref q =>
      let n := countChoice (.quality q) bs
      kindCheck k (some .object) ++ refuse (ref.ok n) (.choiceRef ref (.quality q) n) ++
        refuse q.chosenReadOk (.chosenQualityRead q)
    | .ofYourChoice q dom =>
      kindCheck k (some .object) ++ refuse q.chosenReadOk (.chosenQualityRead q) ++
        OptChoiceDomain.check bs dom
    | .hasKeyword kt => kindCheck k (some .object) ++ refuse kt.known .knownKeywordTerm
    | .hasPossessor ax n =>
      NounPhrase.check (some .player) bs n ++ refuse n.soleHolderOk .soleHolder ++
        refuse (possessorKind ax k) (.possessorKind ax k)
    | .castBy n rank =>
      kindCheck k (some .object) ++ NounPhrase.check (some .player) bs n ++
        refuse n.soleHolderOk .soleHolder ++
        refuse (OptOrdinal.ok (rank.map (·.1))) .ordinalNonZero
    | .castFrom z =>
      kindCheck k (some .object) ++ ZoneExpr.check bs z ++
        refuse (playableFrom (some z.sort)) .playableFrom
    | .inCombat r none => kindCheck k (some .object) ++ refuse r.bare .combatRelOk
    | .inCombat r (some m) =>
      NounPhrase.check none bs m ++
        refuse (combatRelOk r k (m.kindOr .object) (NounPhrase.zone bs m) (NounPhrase.tys bs m)) .combatRelOk
    | .happenedTo lb => LookbackClause.check k bs lb
    | .colorCount r n => kindCheck k (some .object) ++ refuse (colorBoundOk r n) .colorBoundOk
    | .named src => kindCheck k (some .object) ++ NameSource.check bs src
    | .hasDesignation d holder =>
      refuse (d.holder == some k) (.designationHolder d k) ++
        refuse (holder.isNone || d.possessorOk) (.designationPossessorFits d) ++
        OptNoun.check (some .player) bs holder
    | .attachedBy _ by_ => kindCheck k (some .object) ++ NounPhrase.check (some .object) bs by_
    | .attachedTo host =>
      let kh := host.kindOr .object
      kindCheck k (some .object) ++ NounPhrase.check none bs host ++
        refuse (Kind.lte kh (.join .object .player)) (.kindLte kh (.join .object .player))
    | .hasCounters kind =>
      kindCheck k (some .object) ++ OptCounterKind.check kind ++
        refuse (counterKindNamed .object kind) (.counterKindNamed .object)
    | .compare axes _ bound =>
      refuse (axesAt k axes) (.axesAt k) ++ axes.flatMap ProjAxis.check ++ Amount.check bs bound
    | .superlative op ax dom =>
      refuse op.isExtremal .isExtremal ++ refuse (ax.scope == k) (.projScope k) ++ ax.check ++
        Predicate.check k bs dom
    | .withMostVotes =>
      let n := countOutcomes .voteHeld bs
      refuse (n == 1) (.outcomeInScope .voteHeld n)
    /- "each player who chose the highest number": a plural choice, not a vote [CR#701.38c],
    so the gate counts the pluralised number choice. -/
    | .choseExtreme op =>
      kindCheck k (some .player) ++ refuse op.isExtremal .isExtremal ++
        refuse (countManys (.quality .number) bs != 0) .numberChoiceInScope
    | .compareOver dom measure _ bound =>
      refuse k.phrasal (.phrasal k) ++ Predicate.check k bs dom ++
        Amount.check (bindFor .the .one k dom :: (Predicate.delta bs dom ++ bs)) measure ++
        Amount.check bs bound
    | .inZone z => kindCheck k (some .object) ++ ZoneExpr.check bs z
    | .inPile pile =>
      kindCheck k (some .object) ++ NounPhrase.check (some .pile) bs pile ++
        refuse pile.pileMention .pileMention
    | .exiledWith src =>
      kindCheck k (some .object) ++ NounPhrase.check (some .object) bs src ++
        refuse src.linkSource .linkSource
    | .and ps =>
      Predicate.checkAll k bs ps ++ refuse (zonesOk ps) .zoneCoherent ++
        refuse (contradictionFree ps) .contradictionFree ++
        refuse (otherAnchorOk bs k (Predicate.headTysAll ps) ps) .otherAnchored
    | .or ps =>
      if Predicate.joins ps then
        kindCheck k (Predicate.kindOfAll ps) ++ Predicate.checkEach k bs ps
      else
        Predicate.checkAll k bs ps ++ refuse (!ps.isEmpty) .nonEmpty ++
          refuse (parallelDisjuncts ps) .parallelDisjuncts ++
          refuse (coordinableAll ps) .coordinableDisjuncts
    | .not p => Predicate.check k bs p ++ refuse p.negatable .negatable
    | .other => refuse (anyTargeted k bs) (.anyTargeted k)
    | .notChosen => refuse (countParts k bs != 0) (.choiceInScope k)
    | .otherThan n =>
      NounPhrase.check none bs n ++ refuse (n.anchorPhrase && n.plur.isOne) .complementAnchor
    | .coinCameUp _ =>
      refuse (coinFlipInScope bs) .coinFlipInScope ++
        refuse (Kind.lte k (.join .object .player)) (.kindLte k (.join .object .player))
    | .abilityHead cls =>
      kindCheck k (some .object) ++
        (match cls with
         | .keyword kw => refuse (knownKeyword kw) (.knownKeyword kw)
         | _ => [])
    | .abilityOf src => kindCheck k (some .object) ++ NounPhrase.check (some .object) bs src
    | .activatedBy who =>
      kindCheck k (some .object) ++ NounPhrase.check (some .player) bs who ++
        refuse who.soleHolderOk .soleHolder
    | .targets m _ =>
      let km := m.kindOr .object
      NounPhrase.check none bs m ++ refuse km.targetable (.targetable km) ++
        refuse k.targeter (.targeter k)
  termination_by structural p => p

  def Predicate.checkAll (k : Kind) (bs : Bindings) : List Predicate → List Refusal
    | [] => []
    | p :: ps => Predicate.check k bs p ++ Predicate.checkAll k bs ps
  termination_by structural ps => ps

  /-- Each disjunct of a joined disjunction is checked at the kind it names. -/
  def Predicate.checkEach (k : Kind) (bs : Bindings) : List Predicate → List Refusal
    | [] => []
    | p :: ps => Predicate.check (p.kindOr k) bs p ++ Predicate.checkEach k bs ps
  termination_by structural ps => ps

  def OptChoiceDomain.check (_bs : Bindings) : Option ChoiceDomain → List Refusal
    | none => []
    | some (.nameOfCard p) => Predicate.check .object [] p
    | some (.typeOtherThan s) => refuse (s.type == some .creature) .subtypeType
    | some (.number q) => refuse q.wellFormed .wellFormedQ ++ Quantity.check _bs q
    | some (.players p) => Predicate.check .player _bs p
    | some (.abilitiesAmong ks) =>
      refuse (!ks.isEmpty) .nonEmpty ++ refuse (allKnownKeywordTerms ks) .knownKeywordTerm
    | some _ => []
  termination_by structural d => d

  def OptCounterKind.check : Option CounterKind → List Refusal
    | none => []
    | some c => c.check

  def NounPhrase.check (ctx : Option Kind) (bs : Bindings) : NounPhrase → List Refusal
    | .this | .theGrantor _ => ctxCheck ctx (some .object)
    | .you | .combatPlayer _ | .playerGroup _ => ctxCheck ctx (some .player)
    | .asType t n sub =>
      ctxCheck ctx (some .object) ++ NounPhrase.check (some .object) bs n ++
        refuse n.ascribable .ascribable ++ refuse (ascriptionOk t sub) .ascriptionOk
    | .asMarker _ n =>
      ctxCheck ctx (some .object) ++ NounPhrase.check (some .object) bs n ++
        refuse n.ascribable .ascribable
    | .resolvedPermanent spell =>
      ctxCheck ctx (some .object) ++ NounPhrase.check (some .object) bs spell ++
        refuse (zoneIsB (NounPhrase.zone bs spell) .stack) (.zoneIs .stack) ++
        refuse (permanentSpellType (NounPhrase.ty bs spell)) .permanentSpellType
    | .described d p =>
      let k := p.kindOr (ctx.getD .object)
      ctxCheck ctx p.kind? ++ refuse k.phrasal (.phrasal k) ++ Predicate.check k bs p ++
        DetPhrase.check k bs d p
    | .eachOf g =>
      NounPhrase.check ctx bs g ++ refuse (g.plur == .many) .plural ++
        refuse g.groupMention .groupMention
    | .both l r =>
      ctxCheck ctx (some (joinKinds (l.kindOr .object) (r.kindOr .object))) ++
        NounPhrase.check none bs l ++ NounPhrase.check none (nomIntro bs l) r
    | .eitherOf l r =>
      ctxCheck ctx (some (joinKinds (l.kindOr .object) (r.kindOr .object))) ++
        NounPhrase.check none bs l ++ NounPhrase.check none bs r
    | .librarySlice _ amt whose =>
      ctxCheck ctx (some .object) ++ Amount.check bs amt ++ NounPhrase.check (some .player) bs whose ++
        refuse whose.slicePossessorOk .slicePossessor
    | .someOf q d g =>
      ctxCheck ctx (some .object) ++ SliceCount.check bs q ++ OptPredicate.check .object bs d ++
        NounPhrase.check (some .object) bs g ++ refuse g.partitiveBase .partitiveBase
    | .namesAgree _ g =>
      ctxCheck ctx (some .object) ++ NounPhrase.check (some .object) bs g ++
        refuse g.countedMention .countedMention ++ refuse (g.plur == .many) .plural
    | .theRest k pl => ctxCheck ctx (some k) ++ refuse (theRestFits k pl bs) (.theRestFits k)
    | .pileOf q by_ =>
      let n := countReach (.word .pile) .many bs
      ctxCheck ctx (some .pile) ++ SliceCount.check bs q ++ OptNoun.check (some .player) bs by_ ++
        refuse (n == 1) (.anaphor (.word .pile) .many n)
    | .pro r pl w =>
      let n := countReach r pl (view w bs)
      ctxCheck ctx (some r.kind) ++ refuse (n == 1) (.anaphor r pl n)
    | .attachHost w h => ctxCheck ctx (some h.kind) ++ refuse (attachHeadOk w h) .attachHeadOk
    | .possessorOf ax n =>
      let kn := n.kindOr .object
      ctxCheck ctx (some .player) ++ NounPhrase.check none bs n ++
        refuse (possessorKind ax kn) (.possessorKind ax kn)
    | .designated d whose =>
      ctxCheck ctx (some .object) ++ NounPhrase.check (some .player) bs whose ++
        refuse d.heldByItsCard (.designationScope d)
    | .oneEachOf roles pool =>
      ctxCheck ctx (some .object) ++ Predicate.checkAll .object bs roles ++
        NounPhrase.check (some .object) bs pool ++ refuse (pool.plur == .many) .plural ++
        refuse (rolesOk roles) .rolesOk
  termination_by structural n => n

  def OptNoun.check (ctx : Option Kind) (bs : Bindings) : Option NounPhrase → List Refusal
    | none => []
    | some n => NounPhrase.check ctx bs n
  termination_by structural n => n

  def OptPredicate.check (k : Kind) (bs : Bindings) : Option Predicate → List Refusal
    | none => []
    | some p => Predicate.check k bs p
  termination_by structural p => p

  /-- Idris `detOk`, plus the choice-mode reads inside a determiner. -/
  def DetPhrase.check (k : Kind) (bs : Bindings) (d : DetPhrase) (p : Predicate) : List Refusal :=
    match d with
    | .target q =>
      refuse q.nonZero .nonZeroQ ++ refuse q.wellFormed .wellFormedQ ++
        refuse k.targetable (.targetable k) ++ Quantity.check bs q
    | .count q m =>
      refuse q.nonZero .nonZeroQ ++ refuse q.wellFormed .wellFormedQ ++ Quantity.check bs q ++
        (match m with
         | none => []
         | some m => ChoiceMode.check bs m)
    | .a m => ChoiceMode.check bs m
    | .the => refuse p.uniquifies .uniquifying
    | _ => []

  def Quantity.check (bs : Bindings) : Quantity → List Refusal
    | .range _ _ => []
    | .upToOf a => Amount.check bs a
    | .exactlyOf a => Amount.check bs a
  termination_by structural q => q

  def SliceCount.check (bs : Bindings) : SliceCount → List Refusal
    | .counted q => refuse q.nonZero .nonZeroQ ++ refuse q.wellFormed .wellFormedQ ++ Quantity.check bs q
    | .whole => []
  termination_by structural q => q

  def Amount.check (bs : Bindings) : Amount → List Refusal
    | .lit _ | .letter _ => []
    | .statOf axis n =>
      axis.check ++ NounPhrase.check (some axis.scope) bs n ++ refuse n.plur.isOne .singular ++
        (match axis with
         | .stat c => refuse (statHeadTysOk c (NounPhrase.headTys bs n)) .statHeadTysOk
         | _ => [])
    | .countOf g =>
      NounPhrase.check none bs g ++ refuse (g.plur == .many) .plural ++
        refuse g.countableGroup .countableGroup
    | .aggregate _ ax g =>
      let k := g.kindOr .object
      NounPhrase.check none bs g ++ ax.check ++ refuse (ax.scope == k) (.projScope k) ++
        refuse (g.plur == .many) .plural
    | .paid f n =>
      refuse f.named .paidFacetNamed ++ NounPhrase.check (some .object) bs n ++
        refuse (n.paidSubjectOk bs) .paidSubject ++ refuse n.plur.isOne .singular
    | .eventTally op who lb =>
      NounPhrase.check none bs who ++ LookbackClause.check (who.kindOr .object) (nomIntro bs who) lb ++
        refuse (tallyOk op lb.event) .tallyOk
    | .arith op a b =>
      Amount.check bs a ++ Amount.check (Amount.intro bs a) b ++
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
    | .greatestStoredMatch n => NounPhrase.check (some .object) bs n ++ refuse n.plur.isOne .singular
    | .groupSize =>
      let n := countManysAny bs
      refuse (n == 1) (.groupSizeInScope n)
    | .theDifference =>
      let n := countOnes .gap bs
      refuse (n == 1) (.gapInScope n)
    | .devotion who c d =>
      NounPhrase.check (some .player) bs who ++ refuse who.plur.isOne .singular ++ ColorTerm.check bs c ++
        (match d with
         | none => []
         | some d => ColorTerm.check bs d)
    | .half _ a => Amount.check bs a
    | .aggregateOver _ dom body =>
      let k := dom.kindOr .object
      refuse k.phrasal (.phrasal k) ++ Predicate.check k bs dom ++
        Amount.check (bindFor .the .one k dom :: (Predicate.delta bs dom ++ bs)) body
    | .distinctCount ax dom =>
      refuse ax.ok .kindAxisSort ++ NounPhrase.check (some .object) bs dom
    | .upTo b => Amount.check bs b
  termination_by structural a => a

  def ZoneScope.check (z : Zone) (bs : Bindings) : ZoneScope → List Refusal
    | .bare => []
    | .possessedBy n => refuse z.possessable (.possessable z) ++ NounPhrase.check (some .player) bs n
  termination_by structural s => s

  def LibraryPlace.check (bs : Bindings) : LibraryPlace → List Refusal
    | .eitherEnd chooser =>
      OptNoun.check (some .player) bs chooser ++ refuse (eventAgentOk bs chooser) .eventAgent
    | _ => []
  termination_by structural p => p

  def ZoneExpr.check (bs : Bindings) : ZoneExpr → List Refusal
    | .zone z scope => ZoneScope.check z bs scope
    | .library place ord off scope =>
      LibraryPlace.check bs place ++ refuse (place.arrangementOk ord) .placeArrangementFits ++
        refuse (place.ordinalOk off) .placeOrdinalFits ++
        refuse (OptOrdinal.ok off) .ordinalNonZero ++ ZoneScope.check .library bs scope
  termination_by structural z => z

  def ZoneExpr.checkAll (bs : Bindings) : List ZoneExpr → List Refusal
    | [] => []
    | z :: zs => ZoneExpr.check bs z ++ ZoneExpr.checkAll bs zs
  termination_by structural zs => zs

  def NameSource.check (bs : Bindings) : NameSource → List Refusal
    | .printed _ => []
    | .chosen =>
      let n := countChoice (.quality .cardName) bs
      refuse (n == 1) (.choiceRef .theChoice (.quality .cardName) n)
    | .sameAs n => NounPhrase.check (some .object) bs n
  termination_by structural s => s

  def EventSource.check (bs : Bindings) : EventSource → List Refusal
    | .anywhere => []
    | .zones zs => ZoneExpr.checkAll bs zs
    | .anywhereBut zs => ZoneExpr.checkAll bs zs
  termination_by structural s => s

  def OptComplement.check (ev : EventName) (ks : Kind) (bs : Bindings) :
      Option EventComplement → List Refusal
    | none => []
    | some (.involving what) =>
      NounPhrase.check none bs what ++
        refuse (lookbackComplementOk ev ks (what.kindOr .object)) .lookbackComplement
    | some (.fromZones src what) =>
      EventSource.check bs src ++ refuse (complementPlain what) .complementPlain ++
        refuse (lookbackSourceOk ev src) .lookbackSource ++ OptComplement.check ev ks bs what
    | some (.intoZone to what) =>
      ZoneExpr.check bs to ++ refuse (complementSourced what) .complementSourced ++
        refuse (lookbackDestOk ev to.sort) .lookbackDest ++ OptComplement.check ev ks bs what
    | some (.atZone z) => ZoneExpr.check bs z ++ refuse (lookbackLocusOk ev z.sort) .lookbackLocus
  termination_by structural c => c

  def LookbackClause.check (k : Kind) (bs : Bindings) : LookbackClause → List Refusal
    | .mk ev _ what =>
      OptComplement.check ev k bs what ++
        refuse (what.isSome || bareLookbackOk ev k) .complementWritten ++
        refuse (lookbackSubjectOk ev k) .lookbackSubject
  termination_by structural lb => lb
end


mutual
  def Condition.check (bs : Bindings) : Condition → List Refusal
    | .exists_ n => NounPhrase.check none bs n ++ refuse n.existentialMention .existentialMention
    | .happened who lb =>
      NounPhrase.check none bs who ++ LookbackClause.check (who.kindOr .object) (nomIntro bs who) lb
    | .gameIs d =>
      refuse d.gameWide (.designationScope d) ++ refuse d.checked (.designationChecked d)
    | .noHolder d =>
      refuse (d.scope == some (.heldBy .player)) (.designationScope d) ++
        refuse d.checked (.designationChecked d)
    | .matches n p =>
      let k := n.kindOr .object
      NounPhrase.check none bs n ++ Predicate.check k bs p ++ refuse (n.testSubjectOk bs) .testSubject ++
        refuse p.says .predSays ++ refuse (zoneFits (NounPhrase.zone bs n) p.seedZone) .zoneFits ++
        refuse (attachWordsOk p.attachWordsIn (NounPhrase.headTys bs n)) .attachFits
    | .compareAmt subj _ bound =>
      Amount.check bs subj ++ Amount.check (Amount.intro bs subj) bound ++
        refuse subj.read .readAmount
    | .dealtThisWay p =>
      let k := p.kindOr .object
      Predicate.check k bs p ++ refuse (damageDealtInScope bs) .damageDealtInScope ++
        refuse (Kind.lte k (.join .object .player)) (.kindLte k (.join .object .player)) ++
        refuse (!zoneIsB p.seedZone .stack) (.zoneIs .stack)
    | .choseThisWay who p =>
      let k := p.kindOr .object
      NounPhrase.check (some .player) bs who ++ Predicate.check k bs p ++
        refuse (countParts k bs != 0) (.choiceInScope k)
    | .preventedFromSource p =>
      let n := countOutcomes .damagePrevented bs
      Predicate.check .object bs p ++ refuse (n == 1) (.outcomeInScope .damagePrevented n)
    | .flipCalled who _ => NounPhrase.check (some .player) bs who ++ refuse (coinFlipInScope bs) .coinFlipInScope
    | .flipFace _ => refuse (coinFlipInScope bs) .coinFlipInScope
    | .voteLead _ _ =>
      let n := countOutcomes .voteHeld bs
      refuse (n == 1) (.outcomeInScope .voteHeld n)
    | .anyResultIs _ bound =>
      let n := countOutcomes .rollResult bs
      Amount.check bs bound ++ refuse (n == 1) (.outcomeInScope .rollResult n)
    | .rolledDoubles =>
      let n := countOutcomes .rollResult bs
      refuse (n == 1) (.outcomeInScope .rollResult n)
    | .not c => Condition.check bs c
    | .and cs =>
      Condition.checkAll bs cs ++ refuse (atLeastTwoCs cs) .atLeastTwo ++
        refuse (cs.all fun c => !c.isAnd) .flatConjuncts
    | .or cs =>
      Condition.checkAll bs cs ++ refuse (atLeastTwoCs cs) .atLeastTwo ++
        refuse (cs.all fun c => !c.isOr) .flatDisjuncts
  termination_by structural c => c

  def Condition.checkAll (bs : Bindings) : List Condition → List Refusal
    | [] => []
    | c :: cs => Condition.check bs c ++ Condition.checkAll bs cs
  termination_by structural cs => cs
end

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
