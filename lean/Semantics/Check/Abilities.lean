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

def Instruction.scopeBody : Instruction → Instruction
  | .withBindings _ _ body | .inCaller _ body | .ownScope _ body => body.scopeBody
  | body => body

def StaticSpec.scopeBody : StaticSpec → StaticSpec
  | .withBindings _ _ body | .inCaller _ body => body.scopeBody
  | body => body


def AsThough.sort : AsThough → PremiseSort
  | .of _ => .object
  | .mana _ _ _ => .mana
  | .greater _ _ => .value

def twoPartiesOk : NounPhrase → Bool
  | .withBindings _ _ body | .inCaller _ body | .firstMention _ body => twoPartiesOk body
  | .pro (.parameter shape) _ _ => shape.twoParties
  | .laterMention _ body => twoPartiesOk body
  | .and [l, r] => l.plur == .one && r.plur == .one
  | .described d _ => (d.quant >>= Quantity.exact) == some 2
  | _ => false

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

/-- The numerical values an exchange can set [CR#701.12g]: a life total, a power or
toughness, or a rolled result. -/
def Amount.settableValue : Amount → Bool
  | .withBindings _ _ body | .inCaller _ body => body.settableValue
  | .statOf (.playerStat .lifeTotal) _ => true
  | .statOf (.stat .power) _ => true
  | .statOf (.stat .toughness) _ => true
  | .theOutcome .rollResult => true
  | _ => false

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
  | .withBindings _ _ body | .inCaller _ body => body.unit
  | .parameter _ _ shape => shape.exact == some 1
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

def colorOpOk : QualityOp → ColorSpec → Bool
  | .sets, _ => true
  | _, .some [] => false
  | _, _ => true

def ptDelta (bs : Bindings) : Option (Amount × Amount) → List Binding
  | none => []
  | some (p, t) => Amount.introduced (Amount.intro bs p) t ++ Amount.introduced bs p

def CostShift.introduced (bs : Bindings) : CostShift → List Binding
  | .less a _ => Amount.introduced bs a
  | .more a => Amount.introduced bs a
  | .run _ _ _ => []

def gatePayer : Binding := ⟨.the, .one, .player false, []⟩

def DeonticPatient.intro (bs : Bindings) : DeonticPatient → Bindings
  | .noPatient => bs
  | .defendingPlayer m => nomIntro bs m
  | .counterpart m => nomIntro bs m
  | .targetedBy m => nomIntro bs m
  | .counterpartsAt _ => bs

/-- A counterpart is of a kind every deed's role `r` takes. -/
def counterpartKindOk (ds : Deeds) (r : Role) (m : NounPhrase) : Bool :=
  ds.all fun d => deedKindOk d r (m.kindOr .object)

/-- A counterpart named after `n` must not be `n` read back. -/
def counterpartNotSelf (bs : Bindings) (n m : NounPhrase) : Bool :=
  let subject := n.result bs
  let counterpart := m.result subject.context
  let added := (counterpart.introduced subject.context).length
  let sameAddress := match subject.address, counterpart.address with
    | some source, some target => shiftAddress added source == target
    | _, _ => false
  !sameAddress && match counterpart.address with
    | some [index] => !(added ≤ index && index < added + (subject.introduced bs).length)
    | _ => true

def complementAtOk (bs : Bindings) (n : NounPhrase) (ds : Deeds) (r : Role) (c : DeedComplement) :
    Bool :=
  ds.elem c.deed && counterpartKindOk [c.deed] r.counter c.counterpart &&
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

def DamageScale.intro (bs : Bindings) : DamageScale → Bindings
  | .shifted _ amt => Amount.intro bs amt
  | _ => bs

def DividedVerb.intro (bs : Bindings) : DividedVerb → Bindings
  | .damage src => nomIntro bs src
  | .counters _ => bs

/-- The object an enacted move is done to; a status change carries its own zone law. -/
def Instruction.enactPatient : Instruction → Option NounPhrase
  | .withBindings scope inputs body => body.enactPatient.map (.withBindings scope inputs)
  | .inCaller scope body => body.enactPatient.map (.inCaller scope)
  | .ownScope _ body => body.enactPatient
  | .move n _ _ _ => some n
  | _ => none

def NounPhrase.bareThis : NounPhrase → Bool
  | .withBindings _ _ body | .inCaller _ body | .firstMention _ body => body.bareThis
  | .pro (.parameter shape) _ _ => shape.bareThis
  | .laterMention _ body => body.bareThis
  | .this => true
  | _ => false

/-- A deed whose patient is the performer's own (`ActFacts.ownPatient`): a permanent the performer
controls [CR#701.21a], a card in the performer's own hand [CR#701.9a]. Both forms read the same
possessor (`NounPhrase.possessor`): the conjunct an indefinite patient's description writes
("sacrifice a creature you don't control") and the binding a definite referent reads ("discard
that card"). It refuses only a possessor known to be a different player than the performer. -/
def enactPatientPossessorOk (bs : Bindings) (v : Deed) (perf : Option NounPhrase)
    (e : Instruction) : Bool :=
  !actOwnPatient v ||
    match e.enactPatient, perf with
    | some n, some who => !Possessor.differs (NounPhrase.possessor bs n) (who.possessorSide bs)
    | _, _ => true

def Amount.forEach : Amount → Bool
  | .withBindings _ _ body | .inCaller _ body => body.forEach
  | .arith .times _ _ => true
  | _ => false


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

/-- What a handoff records about the player or permanent it hands the body to, its kind among
them, with the same structural facts a captured macro subject keeps. Handing off to `actor` keeps
the performer it re-reads. -/
def NounPhrase.performerShape (bs : Bindings) (who : NounPhrase) : NounShape :=
  let who := who.actorView bs
  ⟨who.kindOr .player, false, who.isYou, who.selfDefinedOk, who.ascribable, twoPartiesOk who,
    who.opponentOnly, false⟩

/-- A phrase naming the object its own text is on: `this`, as written or as "this creature" /
"this permanent". -/
def NounPhrase.namesThis : NounPhrase → Bool
  | .withBindings _ _ body | .inCaller _ body | .firstMention _ body => body.namesThis
  | .asType _ n _ | .asMarker _ n => n.namesThis
  | n => n.bareThis

/-- A handoff takes a player or a permanent [CR#110.1]: the rules instruct a permanent itself
to explore or endure [CR#701.44a,701.63a]. Any other object, a card in a graveyard or a spell,
is never handed an instruction. -/
def NounPhrase.handoffPerformerOk (bs : Bindings) (who : NounPhrase) : Bool :=
  match (who.actorView bs).kindOr .player with
  | .player => true
  | .object => NounPhrase.zone bs who == some .battlefield
  | _ => false

/-- What a handoff to `this` puts in view for its body: the permanent itself, as a definite
mention, as a handed-to target is in view through its own introduction. `this` introduces
nothing, and a self mention (`selfSubjIntroduced`) is no antecedent for "that permanent", so
without it the body's "that permanent" would have none. A player or any other performer adds
nothing. -/
def NounPhrase.performerInView (bs : Bindings) (who : NounPhrase) : List Binding :=
  if who.namesThis then
    [⟨.the, .one, .object (NounPhrase.ty bs who) (some .battlefield) none none none none, []⟩]
  else []

/-- The body context of `act who`: the performer's own introduction, as an `enact` agent's or an
`offer` decider's (`agentIntro`, so "each opponent" is read one member at a time), and a
permanent the performer phrase does not itself introduce (`performerInView`), under the handoff
frame that `actor` reads. -/
def actorCtx (bs : Bindings) (who : NounPhrase) : Bindings :=
  let inView := who.performerInView bs ++ agentIntro bs who
  ⟨.the, .one, .actor (who.performerShape bs) (inView.length - bs.length), []⟩ :: inView

/-- An instruction that has a performer of its own: one of the deeds a player (or a permanent)
does, and an offer, whose performer decides. A handoff to a group hands such an instruction to
each member, as its performer. -/
def Instruction.performed : Instruction → Bool
  | .conclude _ | .separateIntoPiles _ _ _ | .choose _ _ _ _ | .vote _ _ _
  | .copy _ _ _ _ | .changeLife _ | .addMana _ _ _ | .draw _ | .expose _ _
  | .search _ _ _ | .shuffle _ | .flipCoins _ | .rollDice _ _ | .rerollStored _ _
  | .createObject _ _ | .enact _ _ | .pay _ _ | .skipPart _ _ | .addTurnPart _ _ _ _
  | .procedure _ _ _ _
  | .withContinuation .optional _ _ _ => true
  | _ => false

/-- The performer of an instruction: the group a handoff hands it to (`Instruction.checkWith`),
otherwise the actor in context, the controller unless an enclosing handoff names another
[CR#109.5]. -/
def performerOf (perf : Option NounPhrase) : NounPhrase := perf.getD .actor

/-- What a handoff to one performer hands the instruction it wraps: the actor, so that a deed
whose facts row names no performer ("the Ring tempts you" [CR#701.54a]) refuses being handed to
anyone; every other rule reads the actor in context either way. -/
def handedOne (body : Instruction) : Option NounPhrase :=
  match body with
  | .enact _ _ => some .actor
  | _ => none

/-- Whether a deed's facts row names a performer of some kind. -/
def deedNamesPerformer (v : Deed) : Bool := (deedRoleOf v .agent).sort.isSome

/-- The performer an enacted deed records: the one a handoff hands it, otherwise the actor in
context where the deed's row names a performer; a deed the rules give no performer, written
with no handoff, records none. -/
def enactPerformer (v : Deed) (perf : Option NounPhrase) : Option NounPhrase :=
  match perf with
  | some p => some p
  | none => if deedNamesPerformer v then some .actor else none


/-- The handoff frame alone, with what it puts in view: the context a group handoff checks the
one instruction it hands each member in, the instruction introducing the group itself. -/
def performerFrame (bs : Bindings) (who : NounPhrase) : Bindings :=
  ⟨.the, .one, .actor (who.performerShape bs) (who.performerInView bs).length, []⟩ ::
    (who.performerInView bs ++ bs)

/-- Leave a handoff: remove its frame and the `n` bindings the handoff put in view under it, and
nothing else. -/
def leaveHandoff (n : Nat) : Bindings → Bindings
  | [] => []
  | b :: bs => if b.isActorFrame then bs.drop n else b :: leaveHandoff n bs

/-- The source permanent, the performer an object-performed deed defaults to. -/
def sourcePermanent : NounPhrase := .asMarker .permanent .this

/-- An object-performed deed whose performer in context is the controller, that is written with
no handoff to a permanent (printed "Adapt N" on a permanent's own ability [CR#701.46a]), is
performed by the source permanent, as a player-performed deed with no handoff is performed by the
controller [CR#109.5]. -/
def enactDefaultsToThis (bs : Bindings) (v : Deed) (s : NounPhrase) : Bool :=
  s.isYouIn bs && !deedKindOk v .agent .player && deedKindOk v .agent .object

/-- The kind an enacted act's performer is checked as: the deed's `deedAgentKind`, or a
player where the performer is the controller defaulted to the source permanent. -/
def enactAgentKind (bs : Bindings) (v : Deed) : Option NounPhrase → Kind
  | some s => if enactDefaultsToThis bs v s then .player else deedAgentKind v
  | none => .player

/-- The body context of an enacted act: its performer's own introduction, or, defaulted to the
source permanent, a handoff to it, so `actor` in the body is that permanent. -/
def enactCtx (bs : Bindings) (v : Deed) : Option NounPhrase → Bindings
  | none => bs
  | some s => if enactDefaultsToThis bs v s then actorCtx bs sourcePermanent else agentIntro bs s

/-- A nested ability's text is performed by that ability's own controller [CR#109.5], not by
the player an enclosing handoff named: inside a handoff, its context starts with a handoff
frame back to "you". Outside every handoff the context is unchanged. -/
def ownPerformerCtx (bs : Bindings) : Bindings :=
  if bs.any Binding.isActorFrame then ⟨.the, .one, .actor (NounPhrase.performerShape [] .you) 0, []⟩ :: bs
  else bs

/-- The context an offer's body is checked in: its decider's, the performer `perf` a handoff hands
the offer or the actor in context. -/
def ContinuationPolicy.contextBy (perf : Option NounPhrase) (bs : Bindings) :
    ContinuationPolicy → Bindings
  | .optional => mayCtx bs (performerOf perf)
  | .required => bs

def ContinuationPolicy.context (bs : Bindings) (policy : ContinuationPolicy) : Bindings :=
  policy.contextBy none bs

/-! ## Deck conditions -/

def ProjAxis.deck : ProjAxis → Bool
  | .stat .manaValue => true
  | _ => false

def Amount.deckBound : Amount → Bool
  | .withBindings _ _ body | .inCaller _ body => body.deckBound
  | .parameter _ _ shape => shape.literal
  | .lit _ => true
  | _ => false

mutual
  /-- A card set aside for the starting deck is outside the game [CR#103.2b], so only its
  characteristics are readable [CR#109.3]. -/
  def Predicate.deckReadable : Predicate → Bool
    | .withBindings _ _ body | .inCaller _ body => body.deckReadable
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

def bodyEventRegime : GameEvent → Option StackRegime
  | .casts _ _ _ => some .atCasting
  | .nthOccurrence _ _ ev | .causes _ ev => bodyEventRegime ev
  | _ => none

def allTermsBare (ks : List KeywordTerm) : Bool := ks.all KeywordTerm.bare

def distinctTerms : List KeywordTerm → Bool
  | [] => true
  | k :: ks => !ks.elem k && distinctTerms ks

/-! ## Tokens -/

/-- A token's power and toughness together, when both are written. -/
def Characteristics.pt (c : Characteristics) : Option (Amount × Amount) :=
  match c.power, c.toughness with
  | some p, some t => some (p, t)
  | _, _ => none

def Characteristics.lineNonEmpty (c : Characteristics) : Bool :=
  hasAnyTypeCharacteristic c.supertypes c.types c.subtypes
def Characteristics.canonical (c : Characteristics) : Bool :=
  colorsDistinct c.colors && typesDistinct c.types && supersDistinct c.supertypes
def Characteristics.headTy (c : Characteristics) : List CardType := normalizeTypes c.types

def TypeLineChanges.hasWrites (c : TypeLineChanges) : Bool :=
  c.supertypes.isSome || c.types.isSome || c.subtypes.isSome

def TypeLineChanges.hasValues (c : TypeLineChanges) : Bool :=
  hasAnyTypeCharacteristic (c.supertypes.getD []) (c.types.getD []) (c.subtypes.getD [])

def TypeLineChanges.canonical (c : TypeLineChanges) : Bool :=
  supersDistinct (c.supertypes.getD []) && typesDistinct (c.types.getD [])

/-- An edit that writes the type line writes something on it, as a copy exception's does. -/
def CharacteristicEdit.writesLine : CharacteristicEdit → Bool
  | .typeLine _ c => c.hasWrites
  | _ => true

/-- The term shape of one characteristic edit: a type-line write that adds or removes writes a
value, an added or removed color list is not empty, no list names a value twice, and a chosen
quality is read as a chosen one. Which edits fit which subject, op and zone are game rules the
checker does not assert (`lean-checker-sweep-game-rules`). -/
def CharacteristicEdit.shape : CharacteristicEdit → List Refusal
  | .typeLine op c =>
    refuse (op == .sets || !c.hasWrites || c.hasValues) .lineNonEmpty ++ refuse c.canonical .distinct
  | .colors op cs => refuse cs.ok .distinct ++ refuse (colorOpOk op cs) .nonEmpty
  | .chosenQuality _ q => refuse q.qualityReadOk .qualityRead
  | _ => []

def CharacteristicEdit.shapeAll (edits : List CharacteristicEdit) : List Refusal :=
  edits.flatMap CharacteristicEdit.shape

def TokenSpec.headTy (bs : Bindings) : TokenSpec → List CardType
  | .written t => t.characteristics.headTy
  | .asThose => tyOfThoseAny .token bs
  | .copyOf src _ => NounPhrase.ty bs src
  | .doubleFaced front _ => front.characteristics.headTy

def TokenSpec.introduced (bs : Bindings) : TokenSpec → List Binding
  | .written t => ptDelta bs t.characteristics.pt
  | _ => []

def EntryRider.intro (bs : Bindings) : EntryRider → Bindings
  | .withCounters amt kind _ => kind.intro (Amount.intro bs amt)
  | .under who => nomIntro bs who
  | _ => bs

/-! ## Static and instruction attributes -/

def StaticSpec.isCoord : StaticSpec → Bool
  | .withBindings _ _ body | .inCaller _ body => body.isCoord
  | .conjunction _ _ => true
  | _ => false

/-- A deontic rule's counterparts, read as references and kinds: each counterpart is of a kind
the deed's other role takes, a complement names one of the rule's own deeds and no deed twice,
and a counterpart named after the subject is not the subject read back (`counterpartNotSelf`).
Which deeds a patient form goes with, and a counterpart's object class, card types and zone, are
game rules the checker does not assert (`lean-checker-sweep-costs`). -/
def deonticPatientOk (bs : Bindings) (n : NounPhrase) (ds : Deeds) (r : Role)
    (patient : DeonticPatient) : Bool :=
  match patient with
  | .noPatient | .defendingPlayer _ | .targetedBy _ => true
  | .counterpart m => counterpartKindOk ds r.counter m && counterpartNotSelf bs n m
  | .counterpartsAt cs => distinctDeeds (cs.map (·.deed)) && cs.all (complementAtOk bs n ds r)

def DamageOp.intro (bs : Bindings) : DamageOp → Bindings
  | .prevent cut _ => cut.intro bs
  | .redirect cut to => nomIntro (cut.intro bs) to
  | .scale sc => sc.intro bs

def Cost.isCompound : Cost → Bool
  | .withBindings _ _ body | .inCaller _ body => body.isCompound
  | .compound _ => true
  | _ => false

def KeywordParam.shape : KeywordParam → KeywordParamShape
  | .cost _ => .cost
  | .quality _ => .quality
  | .subject _ => .subject
  | .number _ => .number
  | .deckCondition _ => .deckCondition

def keywordSchemaFor (k : KeywordLabel) (params : List KeywordParam) : Option KeywordSchema :=
  (keywordSchemas k).find? (fun schema => schema.shapes == params.map KeywordParam.shape)

def keywordParamFits (k : KeywordLabel) (params : List KeywordParam) : Bool :=
  (keywordSchemaFor k params).isSome

def Ability.notWordHeaded : Ability → Bool
  | .italicHead _ _ => false
  | _ => true

/-- The category an ability is written in [CR#113.3]. A word-headed or extended line is a card's
decoration of an ability, not a category of its own, and a bare keyword line has the category its
own definition declares rather than one it can be read off. -/
def Ability.category : Ability → Option AbilityCategory
  | .static _ => some .static
  | .triggered _ _ _ _ _ _ _ _ => some .triggered
  | .activated _ _ _ _ _ _ => some .activated
  | .spell _ _ => some .spell
  | _ => none

/-! ## A definition component's regime

Each ability of a keyword's definition carries the stack regime the rules give it
(`ComponentFacts.regime`, `docs/decisions/semantics-v2.md` §7, ruling 2026-10-06 on gift and the
additional-cost keywords), and its form is checked against that regime. Where the form
determines the regime the declared one must be it: a triggered ability keys on its event's
regime (a cast trigger at casting, an enters trigger off the stack) [CR#702.153a,702.175a]; a
spell ability is followed as the spell resolves [CR#113.3a]; an additional cost or a change to
the cost is determined as the spell is cast [CR#601.2f,702.33a]; an activated ability and any
other static ability function off the stack. A permission to cast the card leaves it open: such
an ability functions on the stack unless it states its zones, and the keyword's own rule says
which [CR#113.6b,113.6d,702.34a,702.117a], so either is admitted, and never the resolution
regime. -/

/-- Whether a static ability's form admits the stack regime declared for it. -/
def StaticSpec.regimeFits (r : Option StackRegime) : StaticSpec → Bool
  | .withBindings _ _ body | .inCaller _ body => body.regimeFits r
  | .conditional se _ _ => se.regimeFits r
  -- A Mana Substitute applies as the total cost is paid [CR#702.51b,601.2h].
  | .addedCost _ _ _ | .costShift _ _ | .paySubstitute _ _ => r == some .atCasting
  | .altCost _ _ | .deonticRule _ .permit _ _ _ _ _ (.play _ _ _ _ _) => r != some .atResolution
  | _ => r == none
termination_by structural se => se

/-- Whether an ability's form admits the stack regime declared for it as a component of a
keyword's definition. -/
def Ability.regimeFits (r : Option StackRegime) : Ability → Bool
  | .static se => se.regimeFits r
  | .triggered ev _ _ _ _ _ _ _ => bodyEventRegime ev == r
  | .activated _ _ _ _ _ _ => r == none
  | .spell _ _ => r == some .atResolution
  | _ => false

/-! ## Card-class guards

A keyword that means one ability on a permanent and another on an instant or sorcery spell
[CR#702.55a,702.131a,702.131b,702.174b] writes one definition holding both halves, each guarded
by the card's class with the ordinary conditional forms: a `doIf`, a trigger's intervening
condition or a conditional static ability whose condition is that this object matches a
predicate implying the class (`docs/decisions/semantics-v2.md` §7, ruling 2026-10-06 on gift and
the additional-cost keywords). The class a guard implies is read off the card types its
predicate demands: an instant or sorcery is never a permanent card [CR#110.4,110.4a]. -/

mutual
  /-- The card class every object the predicate holds of has, where its types settle it
  [CR#110.4a]. -/
  def Predicate.impliedClass : Predicate → Option CardClass
    | .withBindings _ _ body | .inCaller _ body => body.impliedClass
    | .hasType t =>
      if t.isInstantOrSorcery then some .instantOrSorceryCard
      else if t.permanent then some .permanentCard
      else none
    | .or ps => Predicate.disjunctsClass ps
    | .and ps => Predicate.conjunctsClass ps
    | _ => none
  termination_by structural p => p

  /-- The class every disjunct implies, when they all imply the same one. -/
  def Predicate.disjunctsClass : List Predicate → Option CardClass
    | [] => none
    | p :: ps =>
      match p.impliedClass, ps with
      | some a, [] => some a
      | some a, _ :: _ => if Predicate.disjunctsClass ps == some a then some a else none
      | none, _ => none
  termination_by structural ps => ps

  /-- The class the conjuncts imply, when those that imply one agree. -/
  def Predicate.conjunctsClass : List Predicate → Option CardClass
    | [] => none
    | p :: ps =>
      match p.impliedClass, Predicate.conjunctsClass ps with
      | some a, some b => if a == b then some a else none
      | some a, none => some a
      | none, rest => rest
  termination_by structural ps => ps
end

mutual
  /-- The card class a condition demands of this object. -/
  def Condition.impliedClass : Condition → Option CardClass
    | .withBindings _ _ body | .inCaller _ body => body.impliedClass
    | .matches .this p => p.impliedClass
    | .and cs => Condition.conjunctsClass cs
    | _ => none
  termination_by structural c => c

  def Condition.conjunctsClass : List Condition → Option CardClass
    | [] => none
    | c :: cs =>
      match c.impliedClass, Condition.conjunctsClass cs with
      | some a, some b => if a == b then some a else none
      | some a, none => some a
      | none, rest => rest
  termination_by structural cs => cs
end

/-- The class guarding a whole instruction: a `doIf` with no otherwise-branch. -/
def Instruction.classGuard : Instruction → Option CardClass
  | .withBindings _ _ body | .inCaller _ body => body.classGuard
  | .doIf cond _ none => cond.impliedClass
  | _ => none

def StaticSpec.classGuard : StaticSpec → Option CardClass
  | .withBindings _ _ body | .inCaller _ body => body.classGuard
  | .conditional _ _ .unless_ => none
  | .conditional _ cond _ => cond.impliedClass
  | _ => none

/-- The card class an ability of a keyword's definition is guarded by, if any: the half of the
definition that applies only on that class of card. -/
def Ability.classGuard : Ability → Option CardClass
  | .spell _ instr => instr.classGuard
  | .activated _ instr _ _ _ _ => instr.classGuard
  | .triggered _ _ _ _ _ _ intervening instr =>
    (intervening >>= Condition.impliedClass) <|> instr.classGuard
  | .static se => se.classGuard
  | _ => none

/-- A guard names a class the keyword's card may have: one its registry row puts it on. -/
def KeywordFacts.admitsClass (row : KeywordFacts) : CardClass → Bool
  | .permanentCard => row.onPermanentCard
  | .instantOrSorceryCard => row.onInstantOrSorceryCard

/-- One ability of a keyword's definition against the keyword's registry row: it is a
component the row declares, of its category and with a stack regime its form admits
(`Ability.regimeFits`); a class guard names a class the row admits; and a spell ability is the
instant-or-sorcery half [CR#113.3a], so where the keyword also appears on permanents it is
guarded by that class. -/
def KeywordFacts.bodyPartFits (row : KeywordFacts) (ab : Ability) : Bool :=
  match ab.category with
  | none => false
  | some c =>
    row.definition.any (fun part => part.category == c && ab.regimeFits part.regime) &&
      ab.classGuard.all row.admitsClass &&
      (c != .spell || !row.onPermanentCard || ab.classGuard == some .instantOrSorceryCard)

/-- One ability of a keyword's definition, against the keyword's registry row. -/
def keywordBodyPartFits (k : KeywordLabel) (ab : Ability) : Bool :=
  (keywordFactsFor k).elim false (·.bodyPartFits ab)

/-- A keyword ability's definition [CR#702.1]. A card may write the keyword bare, leaving the
definition empty; a written one is the abilities the keyword represents, each of a declared
category. A keyword whose definition is several abilities of different categories — decayed and
modular each pair a static ability with a triggered one [CR#702.147a,702.43a] — writes them as
the several abilities they are. -/
def keywordBodyFits (k : KeywordLabel) (body : List Ability) : Bool :=
  body.all (keywordBodyPartFits k)

def Ability.grantedKeyword : Ability → Option KeywordLabel
  | .keyword k [] _ => if keywordParamless k then some k else none
  | _ => none

def StaticSpec.keyword : StaticSpec → Option KeywordLabel
  | .withBindings _ _ body | .inCaller _ body => body.keyword
  | .conditional se _ _ => se.keyword
  | .abilityGrant _ ab => ab.grantedKeyword
  | _ => none
termination_by structural se => se

def Instruction.keyword : Instruction → Option KeywordLabel
  | .establish se _ => se.keyword
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

def Ability.introducedLetters : Ability → List Binding
  | .keyword _ [.number (.letter l)] _ => [letterB l]
  | _ => []

mutual
  def Cost.offBattlefield : Cost → Bool
    | .withBindings _ _ body | .inCaller _ body => body.offBattlefield
    | .scaled c _ => c.offBattlefield
    | .tapSymbol | .untapSymbol | .loyaltySymbol _ => false
    | .compound cs => Cost.allOffBattlefield cs
    | .or cs => Cost.allOffBattlefield cs
    | _ => true
  termination_by structural c => c

  def Cost.allOffBattlefield : List Cost → Bool
    | [] => true
    | c :: cs => c.offBattlefield && Cost.allOffBattlefield cs
  termination_by structural cs => cs
end

/-- An enacted deed's body without the own scope the loader reads a keyword action body in: the
move or status change a deed's provenance stamps is read through it. -/
def Instruction.unscoped : Instruction → Instruction
  | .ownScope _ body => body.unscoped
  | e => e

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
  | .withBindings _ _ body | .inCaller _ body | .ownScope _ body => body.reflexEncloseUse
  | .skipPart _ _ => .notYetTaken
  | .addTurnPart .turn _ _ _ => .notYetTaken
  | .establish spec _ =>
    match spec.scopeBody with
    | .controlGrant _ _ => .reflexive
    | _ => .agentless
  | .pay _ _ | .enact _ _ | .separateIntoPiles _ _ _ | .chooseNewTargets _ | .createObject _ _
  | .putCounters _ _ _ | .removeCounters _ _ _ | .moveCounters _ _ _ _ | .doubleCounters _
  | .move _ _ _ _ | .turnFaceDown _ _ | .expose _ _ | .addMana _ _ _ | .draw _ | .choose _ _ _ _
  | .vote _ _ _
  | .search _ _ _ | .shuffle _ | .flipCoins _ | .rollDice _ _
  | .rerollStored _ _ | .procedure _ _ _ _ => .reflexive
  | .applyResultsTable _ => .notOneAction
  | .withContinuation _ body none _ => body.reflexEncloseUse
  | .withContinuation _ _ _ _ => .notOneAction
  | .act _ body => body.reflexEncloseUse
  | .limited e _ => e.reflexEncloseUse
  | .doOnlyIf e _ _ => e.reflexEncloseUse
  | .doIf _ _ _ | .doForEach _ _ | .doForEachKind _ _ _ _ => .notOneAction
  | .repeat_ (.fixed _ body) =>
    match body.scopeBody with
    | .pay _ _ => .reflexive
    | .withContinuation .optional payment none _ =>
      match payment.scopeBody with
      | .pay _ _ => .reflexive
      | _ => .notOneAction
    | _ => .notOneAction
  | .repeat_ _ => .notOneAction
  | .sequentially _ | .simultaneously _ | .chooseModes _ _ | .replace _ _ | .triggerReflexively _
      _
  | .triggerThisWay _ _ _ => .notOneAction
  | .delay _ _ _ _ => .notYetTaken
  | .holdUntil _ _ => .notYetTaken
  | _ => .agentless

def Instruction.thisWayOutcomeOk : Instruction → Bool
  | .withBindings _ _ body | .inCaller _ body | .ownScope _ body => body.thisWayOutcomeOk
  | .delay _ _ _ _ => false
  | .withContinuation _ body _ _ => body.thisWayOutcomeOk
  | .act _ body => body.thisWayOutcomeOk
  | .limited e _ => e.thisWayOutcomeOk
  | _ => true

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
  | .many, .move what@(.theRest _ _) _ _ _ =>
    ⟨distributedDelta bs s (nomIntro bs' what) ++ nomIntro bs s,
     partsClosed (nomIntro bs s),
     some (distributedDelta bs s (stampIntro bs' (some v) what) ++ nomIntro bs s), []⟩
  | .many, .move what _ to _ =>
    ⟨distributedDelta bs s (nomIntro bs' what) ++ nomIntro bs s,
     distributedDelta bs s (moveIntro bs' (some v) what to.sort) ++ nomIntro bs s,
     some (distributedDelta bs s (stampIntro bs' (some v) what) ++ nomIntro bs s), []⟩
  | .many, .setStatus _ n =>
    ⟨nomIntro bs s, distributedDelta bs s (stampIntro bs' (some v) n) ++ nomIntro bs s, none, []⟩
  | .many, _ => ⟨nomIntro bs s, nomIntro bs s, none, ep.deed⟩
  | .one, .move what _ to _ =>
    ⟨nomIntro bs' what, moveIntro bs' (some v) what to.sort,
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

/-- An alternative can re-read a stated magnitude without observing the untaken event. -/
def statedNumbers (deeds : Bindings) : Bindings := deeds.filterMap fun b =>
  match b.payload with
  | .outcome sort => if sort.isAmount then some (outcomeB .namedNumber) else none
  | _ => none

/-- One amount outcome, the total of the events that made it, as `countAmountOutcomes` reads
it. -/
def Binding.isAmountTotal : Binding → Bool
  | ⟨_, .one, .outcome s, _⟩ => s.isAmount
  | _ => false

/-- Branch-local declarations do not escape. Facts about the incoming bindings escape
only when both continuations support them. -/
def branchContext (base left right : Bindings) : Bindings :=
  let l := left.drop (left.length - base.length)
  let r := right.drop (right.length - base.length)
  (unionBindings l r).getD []

mutual
  /-- `Instruction.profile` with `perf`, the performer a handoff to a group hands this very
  instruction, read in place of the actor; `none` everywhere else. -/
  def Instruction.profileWith (bs : Bindings) (perf : Option NounPhrase) : Instruction → InstrProfile
    | .withBindings scope inputs body =>
      let p := Instruction.profileWith (captureBindings bs scope inputs) none body
      ⟨closeOperands p.pre, closeOperands p.announced, p.rider.map closeOperands, p.deed⟩
    | .inCaller scope body =>
      let p := Instruction.profileWith (enterCaller scope bs) none body
      ⟨leaveCaller bs p.pre, leaveCaller bs p.announced, p.rider.map (leaveCaller bs), p.deed⟩
    | .ownScope scope body =>
      let p := Instruction.profileWith (enterOwnScope scope bs) perf body
      ⟨leaveOwnScope scope p.pre, leaveOwnScope scope p.announced,
       p.rider.map (leaveOwnScope scope), p.deed⟩
    | .dealDamage src amt to =>
      sameIntro (nomIntro (Amount.introduced (selfSubjIntro bs src) amt ++ nomIntro bs src) to)
        [outcomeB .damageDealt]
    | .distribute v amt among =>
      let bs' := Amount.intro (v.intro bs) amt
      match v with
      | .damage _ => sameIntro (nomIntro bs' among) [outcomeB .damageDealt]
      | .counters _ => sameIntro (nomIntro bs' among) []
    | .turnOver n => sameIntro (nomIntro bs n) []
    | .setStatus _ n => sameIntro (nomIntro bs n) []
    | .turnFaceDown n _ => sameIntro (nomIntro bs n) []
    | .skipUntap n steps => sameIntro (Amount.intro (nomIntro bs n) steps) []
    | .skipPart _ count => let w := performerOf perf; sameIntro (Amount.intro (nomIntro bs w) count) []
    | .addTurnPart part _ count _ => let who := performerOf perf;
      let afterCount := Amount.intro (nomIntro bs who) count
      if part == .turn then ⟨afterCount, turnRefB :: afterCount, none, []⟩
      else sameIntro afterCount []
    | .combat n update =>
      let bs' := nomIntro bs n
      match update with
      | .participation (.attacking whom) => sameIntro (optAgentIntro bs' whom) []
      | .blocking _ what => sameIntro (nomIntro bs' what) []
      | _ => sameIntro bs' []
    | .attachment _ what host => sameIntro (optAgentIntro (nomIntro bs what) host) []
    | .clearDamage n => sameIntro (nomIntro bs n) []
    | .doAndForbid e _ _ => Instruction.profileWith bs none e
    | .gainDesignation n _ _ => sameIntro (nomIntro bs n) []
    | .unlock door => sameIntro (door.intro bs) []
    | .setGameDesignation _ => sameIntro bs []
    | .conclude _ => let who := performerOf perf; sameIntro (nomIntro bs who) []
    | .drawGame => sameIntro bs []
    | .restartGame => sameIntro bs []
    | .separateIntoPiles grp piles faces => let who := performerOf perf;
      let bs' := nomIntro bs who
      ⟨nomIntro bs' grp, partsClosed (nomIntro bs' grp), none,
       [⟨.the, .many, .pile (NounPhrase.zone bs' grp) (some piles) (pileMentionFace faces), []⟩]⟩
    | .copy src what times _ => let agent := performerOf perf;
      let bs' := nomIntro bs agent
      let k := what.kindOr .object
      sameIntro (Amount.intro (nomIntro bs' what) times)
        [⟨.the, outputPlur what.plur times.plur,
          copyPayloadIn k what.isAbility (NounPhrase.ty bs' what) (src.landsIn (NounPhrase.zone bs' what)), []⟩]
    | .chooseNewTargets what => sameIntro (nomIntro bs what) []
    | .copyTargets cp whom => sameIntro (nomIntro (nomIntro bs cp) whom) []
    | .choose _ n _ _ => let by_ := performerOf perf; sameIntro (chosenIntroBy bs by_.plur by_ n) []
    | .revealChoices _ => sameIntro bs []
    | .vote _ _ _ => sameIntro bs [outcomeB .voteHeld]
    | .move what _ to _ =>
      ⟨nomIntro bs what, moveIntro bs none what to.sort, none, []⟩
    | .exchange what => sameIntro (what.intro bs) what.deed
    /- "Gains"/"loses" name the event outright [CR#119.3]; a set total leaves the gain or loss
    to follow from the new total [CR#119.5]. -/
    | .changeLife (.up a) => let who := performerOf perf; sameIntro (Amount.intro (nomIntro bs who) a) [outcomeB .lifeGained]
    | .changeLife (.down a) => let who := performerOf perf; sameIntro (Amount.intro (nomIntro bs who) a) [outcomeB .lifeLost]
    | .changeLife (.set a) => let who := performerOf perf; sameIntro (Amount.intro (nomIntro bs who) a) []
    | .addMana amt _ _ => let who := performerOf perf; sameIntro (Amount.intro (nomIntro bs who) amt) [outcomeB .manaAdded]
    | .draw amt => let who := performerOf perf; sameIntro (Amount.intro (nomIntro bs who) amt) []
    | .expose _ what => let who := performerOf perf; sameIntro (what.intro (nomIntro bs who)) []
    | .search sc q p => let who := performerOf perf;
      let bs' := nomIntro bs who
      sameIntro (Quantity.introduced bs' q ++ Predicate.introduced bs' p ++ sc.introduced bs' ++ bs')
        [⟨.a, q.plur,
          .object p.seedTy sc.zone (mkStamp (some (deedLabel .librarySearch)) none false) none
            none none, []⟩]
    | .shuffle pile => let whose := performerOf perf;
      let bs' := nomIntro bs whose
      ⟨bs', afterShuffle (Pile.introduced bs' pile ++ bs'), none, []⟩
    | .flipCoins count => let who := performerOf perf; sameIntro (count.intro (nomIntro bs who)) [outcomeB .coinFlipped]
    | .rollDice count _ => let who := performerOf perf; sameIntro (Amount.intro (nomIntro bs who) count) [outcomeB
        .rollResult]
    | .applyResultsTable _ => sameIntro bs []
    | .ignoreOutcomes which => sameIntro (which.intro bs) []
    | .shiftResult _ amt => sameIntro (Amount.intro bs amt) []
    | .storeResults on => sameIntro (nomIntro bs on) []
    | .rerollStored _ whose => let who := performerOf perf; sameIntro (nomIntro (nomIntro bs who) whose) []
    | .establish se _ => sameIntro (StaticSpec.intro bs se) []
    | .createObject count spec => let agent := performerOf perf;
      let bs' := Amount.intro (nomIntro bs agent) count
      let (mentions, types, zone, origin) := match spec with
        | .token token _ => (token.introduced bs', token.headTy bs', Zone.battlefield, some Origin.token)
        | .emblem _ _ => ([], [], Zone.command, none)
      sameIntro (mentions ++ bs')
        [⟨.a, outputPlur agent.plur count.plur, .object types (some zone) none origin none none, []⟩]
    | .putCounters amt kind on => sameIntro (nomIntro (kind.intro (Amount.intro bs amt)) on) []
    | .removeCounters q _ from_ => sameIntro (nomIntro (optQuantIntro bs q) from_) [outcomeB .countersRemoved]
    | .moveCounters amt _ src dst => sameIntro (nomIntro (nomIntro (Amount.intro bs amt) src) dst) []
    | .doubleCounters on => sameIntro (nomIntro bs on) []
    | .enact v e =>
      match enactPerformer v perf, e.unscoped with
      | none, .move what _ to _ =>
        ⟨nomIntro bs what, moveIntro bs (some v) what to.sort,
         some (stampIntro bs (some v) what), []⟩
      | none, .setStatus _ n => ⟨nomIntro bs n, stampIntro bs (some v) n, none, []⟩
      | none, _ => Instruction.profileWith bs none e
      | some s, _ =>
      if enactDefaultsToThis bs v s then
        let leave := leaveHandoff (sourcePermanent.performerInView bs).length
        let ep := Instruction.profileWith (enactCtx bs v (some s)) none e
        doesProfile bs s.plur s v e.unscoped
          ⟨leave ep.pre, leave ep.announced, ep.rider.map leave, ep.deed⟩
      else doesProfile bs s.plur s v e.unscoped (Instruction.profileWith (agentIntro bs s) none e)
    | .pay c .once => let who := performerOf perf; ⟨nomIntro bs who, Cost.intro (nomIntro bs who) c, none, []⟩
    /- One performer: the body's own profile outside the handoff frame, so the player the
    handoff introduced stays published. Distributive performers follow `doForEach`: what each
    member's body introduced is published as a group, except its amount outcomes, which stay
    one total each as the deed, as `doesProfile` keeps a plural agent's deed ("each opponent
    loses 1 life and you gain life equal to the total life lost this way" [CR#702.101a]). -/
    | .act who body =>
      if who.plur == .many && body.performed then
        let p := Instruction.profileWith (performerFrame bs who) (some who) body
        ⟨leaveHandoff 0 p.pre, leaveHandoff 0 p.announced, p.rider.map (leaveHandoff 0), p.deed⟩
      else
      let inner := actorCtx bs who
      let bodyP := Instruction.profileWith inner (handedOne body) body
      let leave := leaveHandoff (who.performerInView bs).length
      match who.plur with
      | .one => ⟨leave bodyP.pre, leave bodyP.announced, bodyP.rider.map leave, bodyP.deed⟩
      | .many =>
        let fresh := bodyP.intro.take (bodyP.intro.length - inner.length)
        ⟨bs, pluralizeIntroduced (fresh.filter (!·.isAmountTotal)) ++
          pluralizeIntroduced (NounPhrase.introduced bs who) ++ bs, none,
         fresh.filter Binding.isAmountTotal⟩
    | .pay c _ => let who := performerOf perf; ⟨nomIntro bs who, Cost.intro (nomIntro bs who) c, none, [outcomeB
        .repeatCount]⟩
    | .limited e _ => Instruction.profileWith bs perf e
    | .withContinuation policy body did notd =>
      let bodyP := Instruction.profileWith (policy.contextBy perf bs) none body
      mayProfile bodyP (Instruction.profileOpt bodyP.intro did) notd
    | .doOnlyIf e c other =>
      let primary := Instruction.profileWith bs none e
      let alt := (Instruction.profileOpt (statedNumbers primary.deed ++ c.intro primary.pre) other).map InstrProfile.intro
      ⟨primary.pre, branchContext primary.pre primary.intro (alt.getD primary.pre), none, []⟩
    | .doIf c e other =>
      let input := c.intro bs
      let primary := Instruction.profileWith input none e
      let alt := (Instruction.profileOpt (statedNumbers primary.deed ++ primary.pre) other).map InstrProfile.intro
      ⟨bs, branchContext bs primary.intro (alt.getD bs), none, []⟩
    | .doForEach grp body =>
      let k := grp.kindOr .object
      let bs' := elemIntro bs k grp
      let bodyP := Instruction.profileWith bs' none body
      ⟨bs, pluralizeIntroduced (bodyP.intro.take (bodyP.intro.length - bs'.length)) ++
        pluralizeIntroduced (NounPhrase.introduced bs grp) ++ bs, none, []⟩
    | .doForEachKind _ dom q body =>
      let bs' := kindValueIntro bs q dom
      let bodyP := Instruction.profileWith bs' none body
      ⟨bs, pluralizeIntroduced (bodyP.intro.take (bodyP.intro.length - bs'.length)) ++
        pluralizeIntroduced (dom.elim [] (NounPhrase.introduced bs)) ++ bs, none, []⟩
    | .repeat_ (.fixed n body) =>
      let bs' := Amount.intro bs n
      let bodyP := Instruction.profileWith bs' none body
      ⟨bs', pluralizeIntroduced (bodyP.intro.take (bodyP.intro.length - bs'.length)) ++ bs', none,
       [outcomeB .repeatCount]⟩
    | .repeat_ _ => sameIntro bs []
    | .sequentially es => Instruction.seqProfile bs es
    | .simultaneously es => Instruction.simProfile bs es
    | .chooseModes q _ => sameIntro (Quantity.introduced bs q ++ bs) []
    | .delay _ _ _ _ => sameIntro bs []
    | .triggerReflexively body _ => Instruction.profileWith bs none body
    | .triggerThisWay body _ _ => Instruction.profileWith bs none body
    | .replace replaced _ => sameIntro (Instruction.profileWith bs none replaced).announced []
    | .holdUntil e _ => sameIntro (Instruction.profileWith bs none e).announced []
    | .procedure _ subject _ _ => let who := performerOf perf;
      sameIntro (nomIntro (nomIntro bs who) subject) []
  termination_by structural e => e

  def Instruction.profileOpt (bs : Bindings) : Option Instruction → Option InstrProfile
    | none => none
    | some e => some (Instruction.profileWith bs none e)
  termination_by structural e => e

  def Instruction.seqProfile (bs : Bindings) : List Instruction → InstrProfile
    | [] => ⟨bs, bs, none, []⟩
    | [e] => (Instruction.profileWith bs none e).last
    | e :: es => Instruction.seqProfile (Instruction.profileWith bs none e).intro es
  termination_by structural es => es

  def Instruction.simProfile (bs : Bindings) : List Instruction → InstrProfile
    | [] => ⟨bs, bs, none, []⟩
    | [e] => (Instruction.profileWith bs none e).simLast
    | e :: es =>
      let ep := Instruction.profileWith bs none e
      (Instruction.simProfile ep.announced es).simCons ep.deed
  termination_by structural es => es

  def Cost.intro (bs : Bindings) : Cost → Bindings
    | .withBindings scope inputs body => closeOperands (body.intro (captureBindings bs scope inputs))
    | .inCaller scope body => leaveCaller bs (body.intro (enterCaller scope bs))
    | .mana c => if manaHasX c then letterB .x :: bs else bs
    | .scaled c _ => Cost.intro bs c
    | .withSubstitute c _ => Cost.intro bs c
    | .loyaltySymbol .downX => letterB .x :: bs
    | .perform e => (Instruction.profileWith bs none e).intro
    | .compound cs => Cost.costsIntro bs cs
    | _ => bs
  termination_by structural c => c

  def Cost.costsIntro (bs : Bindings) : List Cost → Bindings
    | [] => bs
    | c :: cs => Cost.costsIntro (Cost.intro bs c) cs
  termination_by structural cs => cs

  def StaticSpec.intro (bs : Bindings) : StaticSpec → Bindings
    | .withBindings scope inputs body => closeOperands (body.intro (captureBindings bs scope inputs))
    | .inCaller scope body => leaveCaller bs (body.intro (enterCaller scope bs))
    | .letterDefinition l amt => defineLetter l (Amount.intro bs amt)
    | .modification n _ d => Delta.introduced (selfSubjIntro bs n) d ++ selfSubjIntro bs n
    | .ptModification n p t =>
      let afterPower := Delta.introduced (selfSubjIntro bs n) p ++ selfSubjIntro bs n
      Delta.introduced afterPower t ++ afterPower
    | .ptDefinition n _ amt =>
      outcomeB .namedNumber :: (Amount.introduced (selfSubjIntro bs n) amt ++ selfSubjIntro bs n)
    | .ptSwitch n => selfSubjIntro bs n
    | .costShift n sh => sh.introduced (selfSubjIntro bs n) ++ selfSubjIntro bs n
    | .altCost n _ => selfSubjIntro bs n
    | .addedCost _ _ _ => bs
    | .paySubstitute n _ => selfSubjIntro bs n
    | .specialAction _ _ _ => bs
    | .abilityGrant n ab => ab.introducedLetters ++ selfSubjIntro bs n
    | .abilityGrantFrom n _ src _ => nomIntro (nomIntro bs n) src
    | .deonticRule n _ _ _ _ _ _ _ => selfSubjIntro bs n
    | .partSkip who _ => nomIntro bs who
    | .manaRetention who _ => nomIntro bs who
    | .characteristicChange n _ => selfSubjIntro bs n
    | .copyChange n _ _ => selfSubjIntro bs n
    | .controlGrant who what => stampIntro (nomIntro bs who) (some (.core .gainControl)) what
    | .replacement ev alts _ _ _ _ => interceptCtx bs alts ev
    | .damageRule _ src scope op _ => op.intro (scope.intro (src.intro bs))
    | .preventionBan _ what _ => what.intro bs
    | .conditional se c _ => c.intro (StaticSpec.intro bs se)
    | .retention se n => nomIntro (StaticSpec.intro bs se) n
    | .visibility _ who what => what.intro (nomIntro bs who)
    | .additionalTriggers _ _ => bs
    | .entryRider n rider => rider.intro (nomIntro bs n)
    | .choice _ n _ _ _ => selfSubjIntro bs n
    | .conjunction subject parts => StaticSpec.partsIntro (subjCtx bs subject) parts
  termination_by structural se => se

  def StaticSpec.partsIntro (bs : Bindings) : List StaticSpec → Bindings
    | [] => bs
    | se :: rest => StaticSpec.partsIntro (StaticSpec.intro bs se) rest
  termination_by structural parts => parts
end

/-- The bindings an instruction leaves in `bs`. -/
def Instruction.profile (bs : Bindings) (e : Instruction) : InstrProfile :=
  Instruction.profileWith bs none e

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
  | .withContinuation policy body _ _ => Instruction.replacedCtx (policy.context bs) body
  | .doOnlyIf e _ _ => Instruction.replacedCtx bs e
  | .limited e _ => Instruction.replacedCtx bs e
  | .doIf _ _ _ => bs
  | e => e.introducedDeeds bs ++ e.annIntro bs

def Instruction.otherwiseCtx (bs : Bindings) (e : Instruction) : Bindings :=
  statedNumbers (e.introducedDeeds bs) ++ e.preIntro bs

def reflexCtx (bs : Bindings) (body : Instruction) : Bindings := settleTargets (body.intro bs)

def thisWayCtx (bs : Bindings) (body : Instruction) (ev : GameEvent) : Bindings :=
  settleTargets (GameEvent.after (body.intro bs) ev)

def Instruction.namesThisDoor : Instruction → Bool
  | .withBindings _ _ body | .inCaller _ body | .ownScope _ body => body.namesThisDoor
  | .delay ev alts _ _ => ev.namesThisDoor || alts.any GameEvent.namesThisDoor
  | .holdUntil _ ev => ev.namesThisDoor
  | .triggerThisWay _ ev _ => ev.namesThisDoor
  | _ => false

def Ability.namesThisDoor : Ability → Bool
  | .activated _ instr _ _ _ _ => instr.namesThisDoor
  | .triggered ev alts while_ joins _ _ _ instr =>
    ev.namesThisDoor || alts.any GameEvent.namesThisDoor || Timing.namesThisDoor while_ ||
      joins.any JoinedHeader.namesThisDoor || instr.namesThisDoor
  | .spell _ instr => instr.namesThisDoor
  | .italicHead _ ab => ab.namesThisDoor
  | .alsoForKeywords ab _ => ab.namesThisDoor
  | _ => false

mutual
  def Instruction.introducedChoices : Instruction → List Binding
    | .withBindings _ _ body | .inCaller _ body | .ownScope _ body => body.introducedChoices
    | .choose _ (.described (.a _) p) _ _ => introducedChoiceAt (p.kindOr .object)
    | .choose _ _ _ _ => []
    | .sequentially es => Instruction.introducedChoicesAll es
    | .withContinuation _ body _ _ => body.introducedChoices
    | .act _ body => body.introducedChoices
    | .limited e _ => e.introducedChoices
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
    | .withBindings scope inputs body => body.introducedChoices (captureBindings bs scope inputs)
    | .inCaller scope body => body.introducedChoices (enterCaller scope bs)
    | .choice _ _ q _ _ => [q.binding]
    | .conjunction _ parts => StaticSpec.partsIntroducedChoices bs parts
    | .addedCost c _ _ => c.introduced bs
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
  | .moreTimes times | .fixed times _ => [(times, .clamped)]

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
def EntryRider.numberSlots : EntryRider → List (Amount × NumberRegime)
  | .entersAs _ | .entersAttacking _ | .entersTransformed | .entersMelded _ => []
  | .withCounters amount _ _ => [(amount, .clamped)]
  | .under _ | .asCopyOf _ _ _ | .faceDown _ => []

def EntryRider.ridersSlots : List EntryRider → List (Amount × NumberRegime)
  | [] => []
  | rider :: rest => rider.numberSlots ++ EntryRider.ridersSlots rest

/-- A placement's own slots: how many counters a card is exiled with, a count [CR#107.1b]. -/
def PilePlacement.numberSlots : PilePlacement → List (Amount × NumberRegime)
  | .withCounters amount _ _ => [(amount, .clamped)]
  | .inOrder _ | .nthFromEnd _ | .orOtherEnd | .faceDown _ => []

def MoveRider.numberSlots : MoveRider → List (Amount × NumberRegime)
  | .entry rider => rider.numberSlots
  | .placement p => p.numberSlots

def MoveRider.ridersSlots : List MoveRider → List (Amount × NumberRegime)
  | [] => []
  | rider :: rest => rider.numberSlots ++ MoveRider.ridersSlots rest

/-- A cost's own slots: "for each" multiplies a cost by a count [CR#107.1b]. A nested cost and
a performed instruction declare their own. -/
def Cost.numberSlots : Cost → List (Amount × NumberRegime)
  | .withBindings _ _ body | .inCaller _ body => body.numberSlots
  | .mana _ => []
  | .scaled _ amount => [(amount, .clamped)]
  | .tapSymbol | .untapSymbol | .loyaltySymbol _ => []
  | .perform _ | .compound _ | .or _ | .itsManaCost | .withSubstitute _ _ => []

/-- The static spec's own slots and the regime each is read in [CR#107.1b]. -/
def StaticSpec.numberSlots : StaticSpec → List (Amount × NumberRegime)
  | .withBindings _ _ body | .inCaller _ body => body.numberSlots
  | .modification _ _ delta => delta.numberSlots
  | .ptModification _ power toughness => power.numberSlots ++ toughness.numberSlots
  -- An effect that sets a power and toughness, the exception [CR#107.1b] names.
  | .ptDefinition _ _ amount => [(amount, .signed)]
  | .ptSwitch _ => []
  | .costShift _ shift => shift.numberSlots
  | .altCost _ _ | .addedCost _ _ _ | .paySubstitute _ _ | .specialAction _ _ _ => []
  -- The letter X, defined by the text: a chosen or defined value is never negative
  -- [CR#107.1b,107.3].
  | .letterDefinition _ amount => [(amount, .clamped)]
  | .abilityGrant _ _ | .abilityGrantFrom _ _ _ _ => []
  | .deonticRule _ _ _ _ bound _ asThough _ =>
    optSlots CountBound.numberSlots bound ++ optSlots AsThough.numberSlots asThough
  | .manaRetention _ _ | .partSkip _ _ | .characteristicChange _ _ => []
  | .retention _ _ => []
  | .copyChange _ _ _ | .controlGrant _ _ => []
  | .replacement _ _ _ _ _ _ => []
  | .damageRule _ _ _ op _ => op.numberSlots
  | .preventionBan _ _ _ => []
  | .conditional _ _ _ => []
  | .visibility _ _ _ | .additionalTriggers _ _ => []
  | .entryRider _ rider => rider.numberSlots
  | .choice _ _ _ _ _ | .conjunction _ _ => []

/-- The instruction's own slots and the regime each is read in [CR#107.1b]. -/
def Instruction.numberSlots : Instruction → List (Amount × NumberRegime)
  | .withBindings _ _ body | .inCaller _ body | .ownScope _ body => body.numberSlots
  -- You can't deal negative damage [CR#107.1b].
  | .dealDamage _ amount _ => [(amount, .clamped)]
  | .setStatus _ _ | .turnOver _ | .combat _ _ => []
  | .attachment _ _ _ | .clearDamage _ | .doAndForbid _ _ _ => []
  | .gainDesignation _ _ _ | .unlock _ | .setGameDesignation _ | .conclude _ => []
  | .drawGame | .restartGame | .separateIntoPiles _ _ _ => []
  | .choose _ _ _ _ | .revealChoices _ | .vote _ _ _ => []
  | .move _ _ _ riders => MoveRider.ridersSlots riders
  | .turnFaceDown _ _ => []
  -- How many copies to make: a count [CR#107.1b].
  | .copy _ _ times _ => [(times, .clamped)]
  | .chooseNewTargets _ | .copyTargets _ _ => []
  -- Gaining and losing life are clamped; "your life total becomes …" is the set the rule
  -- excepts, and doubling a life total is written as that set [CR#107.1b].
  | .changeLife delta => delta.numberSlots
  | .exchange exchanged => exchanged.numberSlots
  | .addMana amount _ _ => [(amount, .clamped)]
  | .draw amount => [(amount, .clamped)]
  | .expose _ _ | .search _ _ _ | .shuffle _ => []
  | .flipCoins count => count.numberSlots
  | .rollDice count _ => [(count, .clamped)]
  | .applyResultsTable _ => []
  | .ignoreOutcomes which => which.numberSlots
  -- The sign is the direction; the amount added to a result is a magnitude [CR#107.1b].
  | .shiftResult _ amount => [(amount, .clamped)]
  | .storeResults _ | .rerollStored _ _ => []
  | .establish spec none => match spec.scopeBody with
    | .letterDefinition _ amount => [(amount, .clamped)]
    | _ => []
  | .establish _ _ => []
  | .createObject count spec => (count, .clamped) ::
      (match spec with
       | .token _ riders => EntryRider.ridersSlots riders
       | .emblem _ _ => [])
  | .putCounters amount _ _ => [(amount, .clamped)]
  | .distribute _ amount _ => [(amount, .clamped)]
  | .removeCounters _ _ _ => []
  | .moveCounters amount _ _ _ => [(amount, .clamped)]
  | .doubleCounters _ => []
  | .enact _ _ | .pay _ _ | .act _ _ => []
  | .withContinuation _ _ _ _ | .doOnlyIf _ _ _ | .doIf _ _ _ | .limited _ _ => []
  | .doForEach _ _ | .doForEachKind _ _ _ _ => []
  | .repeat_ repetition => repetition.numberSlots
  | .sequentially _ | .simultaneously _ | .chooseModes _ _ => []
  | .delay _ _ _ _ | .replace _ _ | .holdUntil _ _ => []
  | .triggerReflexively _ _ | .triggerThisWay _ _ _ => []
  | .skipUntap _ steps => [(steps, .clamped)]
  | .skipPart _ count => [(count, .clamped)]
  | .addTurnPart _ _ count _ => [(count, .clamped)]
  | .procedure _ _ _ _ => []

end Semantics
