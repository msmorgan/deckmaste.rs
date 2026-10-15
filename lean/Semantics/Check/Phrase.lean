import Semantics.Phrase
import Semantics.Check.Events
import Semantics.Check.CardFacts
import Semantics.Check.Creation
import Semantics.Check.Choices
import Semantics.Check.DeonticPatients
import Semantics.Check.LocalShape
import Semantics.Check.Possessor
import Semantics.Check.TextChanges
import Semantics.Check.Selections
import Semantics.Check.PredicateShape
import Semantics.Check.Refusal

/-!
# Semantics.Check.Phrase

Minimal core DRT and action-clause checker (`docs/decisions/semantics-v3.md` §1.2–§1.5, §3.2–§3.8).
-/

namespace Semantics

def refuse (ok : Bool) (r : Refusal) : List Refusal := if ok then [] else [r]


/-- Stepping phase (`semantics-v3.md` §3.5):
- `preEvent` (`would` [CR#614.1,614.6]): introduces participants in their pre-action zone/status
  and prospective magnitudes (`thatMuch`, `thatMany`), without introducing post-completion product
  referents or clearing `idents`.
- `postEvent` (`does`, `did`, imperative): transitions patient referents to their post-action
  zone/status stamped with `.actedOn deed` (clearing `r.idents := []` on zone changes
  [CR#400.7,400.7j]) and introduces post-completion product referents. -/
inductive StepPhase where
  | preEvent
  | postEvent
  deriving DecidableEq, Repr

def joinKinds (a b : Kind) : Kind := if a == b then a else .join a b

def DetPhrase.det : DetPhrase → Determiner
  | .target _ => .target
  | .a _ => .a
  | .each => .each
  | .all => .all
  | .the => .the
  | .count _ _ => .count
  | .bare => .a

def Amount.plur (a : Amount) (fuel : Nat := 16) : Plurality :=
  match fuel, a with
  | n + 1, .inCaller body | n + 1, .upTo body => body.plur n
  | _ + 1, .lit 1 => .one
  | _, _ => .many
termination_by structural fuel

def Quantity.plur (q : Quantity) (fuel : Nat := 16) : Plurality :=
  match fuel, q with
  | 0, _ => .many
  | n + 1, .inCaller body => body.plur n
  | _ + 1, .range _ (some 1) => .one
  | n + 1, .exactlyOf amount => amount.plur n
  | _ + 1, _ => .many
termination_by structural fuel

def DetPhrase.plur : DetPhrase → Plurality
  | .target q | .count q _ => q.plur
  | .a _ | .each | .the => .one
  | .all | .bare => .many

def SliceCount.plur : SliceCount → Plurality
  | .counted q => q.plur
  | .whole | .totalling .. => .many

def Pile.sort : Pile → Option Zone
  | .library _ => some .library
  | .graveyard _ => some .graveyard
  | .that => none
  | .exiledWith _ => some .exile

def ZoneExpr.sort (z : ZoneExpr) (fuel : Nat := 16) : Option Zone :=
  match fuel, z with
  | 0, _ => none
  | n + 1, .inCaller body => body.sort n
  | _ + 1, .zone zn _ => some zn
  | _ + 1, .place _ p => p.sort
  | _ + 1, .wherever | _ + 1, .anyZone => none
termination_by structural fuel

def Predicate.kind? (p : Predicate) (fuel : Nat := 32) : Option Kind :=
  match fuel, p with
  | 0, _ => none
  | n + 1, .inCaller body | n + 1, .bracketed body | n + 1, .not body => body.kind? n
  | _ + 1, .anyPlayer | _ + 1, .opponent | _ + 1, .chosenPlayer _ | _ + 1, .choseExtreme _ => some .player
  | _ + 1, .qualityNoun q _ => some (.quality q)
  | _ + 1, .counterKindOn _ => some (.quality .counterKind)
  | _ + 1, .hasDesignation d _ => d.holder
  | _ + 1, .compare axes _ _ => (axes.head?).map (·.scope)
  | n + 1, .superlative _ ax dom => dom.kind? n <|> some ax.scope
  | n + 1, .compareOver dom _ _ _ => dom.kind? n
  | n + 1, .and ps => ps.findSome? (·.kind? n)
  | n + 1, .or ps =>
    ps.foldl (fun acc q =>
      match acc, q.kind? n with
      | some k, some k' => some (joinKinds k k')
      | some k, none => some k
      | none, k' => k') none
  | _ + 1, .hasPossessor _ _ | _ + 1, .did _ | _ + 1, .other | _ + 1, .notChosen
  | _ + 1, .otherThan _ | _ + 1, .isSource | _ + 1, .withMostVotes | _ + 1, .coinCameUp _ => none
  | _ + 1, _ => some .object
termination_by structural fuel

def Predicate.kindOr (d : Kind) (p : Predicate) : Kind := p.kind?.getD d

def NounPhrase.kind? (np : NounPhrase) (fuel : Nat := 32) : Option Kind :=
  match fuel, np with
  | 0, _ => some .object
  | n + 1, .inCaller body | n + 1, .firstMention _ body | n + 1, .laterMention _ body => body.kind? n
  | _ + 1, .you | _ + 1, .actor | _ + 1, .playerGroup _ | _ + 1, .possessorOf _ _ | _ + 1, .combatPlayer _ => some .player
  | _ + 1, .pileOf _ _ => some .pile
  | _ + 1, .attachHost _ word => some word.kind
  | _ + 1, .designated d _ => d.holder
  | _ + 1, .described _ p => p.kind?
  | _ + 1, .pro r _ => some r.kind
  | _ + 1, .theRest k _ => some k
  | _ + 1, .paid f _ => some f.entityKind
  | _ + 1, _ => some .object
termination_by structural fuel

def NounPhrase.kindIn (drs : DRS) (d : Kind) (np : NounPhrase) (fuel : Nat := 32) : Kind :=
  match fuel, np with
  | 0, _ => d
  | n + 1, .inCaller body => body.kindIn (enterCaller drs) d n
  | n + 1, .firstMention _ body | n + 1, .laterMention _ body => body.kindIn drs d n
  | _ + 1, .actor => (actorShape? drs).elim .player (·.kind)
  | _ + 1, .which => (drs.relativeBinder.bind drs.findById?).elim d (·.kind)
  | _ + 1, .refl pl => ((firstReflexive pl drs).map (·.kind)).getD d
  | _ + 1, n => n.kind?.getD d
termination_by structural fuel

def Predicate.seedZone (p : Predicate) (fuel : Nat := 32) : Option Zone :=
  match fuel, p with
  | 0, _ => none
  | n + 1, .inCaller body | n + 1, .bracketed body => body.seedZone n
  | _ + 1, .inZone z => z.sort
  | _ + 1, .inCombat .attackedBy _ => none
  | _ + 1, .isToken | _ + 1, .inCombat .. | _ + 1, .attachment ..
  | _ + 1, .currentFace _ => some .battlefield
  | _ + 1, .abilityHead _ | _ + 1, .abilityOf _ | _ + 1, .activatedBy _
  | _ + 1, .isManaAbility | _ + 1, .targets _ _ => some .stack
  | _ + 1, .isEmblem => some .command
  | _ + 1, .exiledWith _ => some .exile
  | _ + 1, .hasStatus _ => some .battlefield
  | n + 1, .and ps => ps.findSome? (·.seedZone n)
  | n + 1, .or (p :: ps) =>
    let zone := p.seedZone n
    if ps.all (fun p => p.seedZone n == zone) then zone else none
  | _ + 1, _ => none
termination_by structural fuel

def Predicate.seedsToken (p : Predicate) (fuel : Nat := 32) : Bool :=
  match fuel, p with
  | 0, _ => false
  | n + 1, .inCaller body | n + 1, .bracketed body => body.seedsToken n
  | _ + 1, .isToken => true
  | n + 1, .and ps => ps.any (·.seedsToken n)
  | _ + 1, _ => false
termination_by structural fuel

def NounPhrase.plur (np : NounPhrase) (fuel : Nat := 32) : Plurality :=
  match fuel, np with
  | 0, _ => .one
  | n + 1, .inCaller body | n + 1, .firstMention _ body | n + 1, .laterMention _ body => body.plur n
  | _ + 1, .described d _ => d.plur
  | _ + 1, .someOf c _ _ | _ + 1, .pileOf c _ => c.plur
  | _ + 1, .theRest _ pl | _ + 1, .pro _ pl | _ + 1, .refl pl => pl
  | n + 1, .slice _ (.lit 1) (.library (.possessedBy who))
  | n + 1, .slice _ (.lit 1) (.graveyard (.possessedBy who)) => who.plur n
  | _ + 1, .slice _ (.lit 1) _ => .one
  | _ + 1, .slice .. | _ + 1, .run .. | _ + 1, .oneEachOf .. => .many
  | _ + 1, .and _ => .many
  | n + 1, .or (x :: _) => x.plur n
  | _ + 1, .playerGroup .allPlayers | _ + 1, .playerGroup .yourOpponents | _ + 1, .playerGroup .yourTeam => .many
  | _ + 1, _ => .one
termination_by structural fuel

def NounPhrase.det (np : NounPhrase) (fuel : Nat := 64) : Option Determiner :=
  match fuel, np with
  | n + 1, .inCaller b | n + 1, .firstMention _ b | n + 1, .agree _ _ b => b.det n
  | _ + 1, .described d _ => some d.det
  | _ + 1, .eachOf _ => some .each
  | _ + 1, .slice .. | _ + 1, .run .. | _ + 1, .theRest .. => some .the
  | _ + 1, .someOf .. | _ + 1, .pileOf .. => some .part
  | _ + 1, .oneEachOf .. => some .a
  | _, _ => none
termination_by structural fuel

def NounPhrase.agentChoosable (phrase : NounPhrase) (fuel : Nat := 64) : Bool :=
  match fuel, phrase with
  | 0, _ => false
  | n+1, .inCaller body | n+1, .firstMention _ body => body.agentChoosable n
  | _+1, .someOf .. | _+1, .pileOf .. => true
  | _+1, phrase => phrase.det == some .a || phrase.det == some .target || phrase.det == some .count
termination_by structural fuel

def NounPhrase.perMemberOk (np : NounPhrase) : Bool :=
  np.det == some .each || np.det == some .all || np.plur == .one

def Predicate.headIsPlaceless (p : Predicate) (fuel : Nat := 64) : Bool :=
  match fuel, p with
  | n + 1, .inCaller b | n + 1, .bracketed b => b.headIsPlaceless n
  | _ + 1, .isSource | _ + 1, .isCard => true
  | n + 1, .and ps => ps.any (fun p => p.headIsPlaceless n)
  | _, _ => false
termination_by structural fuel

def Predicate.phraseZone (k : Kind) (p : Predicate) : Option Zone :=
  if !Kind.lte k .object then none
  else if p.headIsPlaceless then
    match p with
    | .inZone z => z.sort
    | .and ps => ps.findSome? (fun p => match p with | .inZone z => z.sort | _ => none)
    | _ => none
  else p.seedZone.orElse (fun _ => some .battlefield)

def Predicate.seedsAbility (p : Predicate) (fuel : Nat := 64) : Bool :=
  match fuel, p with
  | n + 1, .inCaller b | n + 1, .bracketed b | n + 1, .compareOver b _ _ _ =>
    b.seedsAbility n
  | _ + 1, .abilityHead _ | _ + 1, .abilityOf _ | _ + 1, .activatedBy _
  | _ + 1, .isManaAbility => true
  | n + 1, .and ps => ps.any (fun p => p.seedsAbility n)
  | n + 1, .or ps => !ps.isEmpty && ps.all (fun p => p.seedsAbility n)
  | _, _ => false
termination_by structural fuel

def predicateOfKind (k : Kind) (p : Predicate) : Predicate :=
  match p with
  | .or ps => .or (ps.filter (fun p => p.kind?.all (· == k)))
  | p => p

def predicatePayload (drs : DRS) (k : Kind) (p : Predicate) (fuel : Nat := 64) : Payload :=
  match fuel, k with
  | 0, _ => .gap
  | _ + 1, .object =>
    let p := predicateOfKind .object p
    if p.seedsAbility then .ability none (some .stack)
    else .object p.seedTy (p.phraseZone .object) none
      (if p.seedsToken then some .token else none) none (p.seedPossessor drs)
  | _ + 1, .player => .player false
  | _ + 1, .quality q => .quality q
  | _ + 1, .letter l => .letter l
  | _ + 1, .turnRef => .turnRef
  | _ + 1, .pile => .pile none none none
  | n + 1, .join l r => .join (predicatePayload drs l p n) (predicatePayload drs r p n)
  | _ + 1, _ => .gap
termination_by structural fuel

def bindFor (drs : DRS) (det : Determiner) (plur : Plurality) (k : Kind) (p : Predicate) : Referent :=
  { det, plur, payload := predicatePayload drs k p }

/-- Whether `reach` is a bare pronoun governed by Rule `D′` (`it`, `them`, `they`). -/
def Reach.isBarePronoun : Reach → Bool
  | .bare | .atSlot _ => true
  | _ => false

/-- Clear `r.idents` on departed referents and transition them to `destZone` stamped with `deed`
(`semantics-v3.md` §1.2, `[CR#400.7,400.7j]`). -/
def applyZoneMove (drs : DRS) (refIds : List ReferentId) (deed : Option Deed)
    (destZone : Option Zone) : DRS :=
  refIds.eraseDups.foldl (fun (state : DRS) oldId =>
    match state.findById? oldId with
    | none => state
    | some old =>
      let payload : Option Payload := match old.payload with
        | .object ty oldZone _ orig size poss =>
          some (.object ty destZone (mkStamp deed oldZone true) orig size
            (keptPossessor deed oldZone destZone poss))
        | .ability orig _ => some (.ability orig destZone)
        | _ => none
      match payload with
      | none => state
      | some payload =>
        let freshId := state.nextId
        let remap := fun id => if id == oldId then freshId else id
        let remapRole := fun (id, role) => (remap id, role)
        let refs := state.referents.map fun r =>
          if r.id == oldId then { r with id := freshId, idents := [], payload }
          else match r.payload with
            | .actor shape =>
              { r with payload := .actor { shape with refIds := shape.refIds.map remap } }
            | _ => r
        let frames := state.scopeStack.map fun frame =>
          let callerOwned := frame.preEntryIds.elem oldId || frame.callerReferentIds.elem oldId
          { frame with
            maskedIds := frame.maskedIds.map remap
            callerPending := frame.callerPending.map remapRole
            callerReferentIds := (frame.callerReferentIds.map remap ++
              if callerOwned then [freshId] else []).eraseDups }
        { state with
          referents := refs
          nextId := freshId + 1
          scopeStack := frames
          pendingRoles := state.pendingRoles.map remapRole,
          resolvedIds := state.resolvedIds.map remap,
          performer := state.performer.map remap,
          relativeBinder := state.relativeBinder.map remap }) drs

/-- Stamp `refIds` in place with `deed` without changing zone. -/
def applyInPlaceStamp (drs : DRS) (refIds : List ReferentId) (deed : Option Deed) : DRS :=
  match deed, refIds with
  | none, _ | _, [] => drs
  | some v, _ =>
    refIds.foldl (fun d rid =>
      d.updateById rid fun r =>
        let newPayload := match r.payload with
          | .object ty zn oldProv orig sz poss =>
            let newStamp := match oldProv with
              | some ⟨_, wasPerm, moved⟩ => some ⟨v, wasPerm, moved⟩
              | none => mkStamp (some v) zn false
            .object ty zn newStamp orig sz poss
          | other => other
        { r with payload := newPayload }) drs

/-- Infer the canonical `Deed` of an instruction for `.actedOn` stamping. -/
def Instruction.inferredDeed (inst : Instruction) (fuel : Nat := 32) : Option Deed :=
  match fuel, inst with
  | 0, _ => none
  | n + 1, .inCaller body | n + 1, .bracketed body => body.inferredDeed n
  | _ + 1, .ownScope (some d) _ => some d
  | n + 1, .ownScope none body => body.inferredDeed n
  | _ + 1, .enact verb _ _ _ => some verb
  | _ + 1, .move _ _ (some (.zone .exile _)) _ _ _ => some (.action "exile")
  | _ + 1, .move _ _ (some (.zone .graveyard _)) _ _ _ => some (.core .put)
  | _ + 1, .move _ _ (some (.zone .hand _)) _ _ _ => some (.core .return_)
  | _ + 1, .setStatus .tapped _ => some (.action "tap")
  | _ + 1, .setStatus .untapped _ => some (.action "untap")
  | _ + 1, .draw _ => some (.core .draw)
  | _ + 1, .trigger => some (.core .trigger)
  | _ + 1, .crew _ => some (.ofAbility (.named "crew"))
  | _ + 1, .saddle _ => some (.ofAbility (.named "saddle"))
  | _ + 1, .fullyUnlock _ => some (.core .fullyUnlock)
  | _ + 1, .createObject .. => some (.action "create")
  | _ + 1, .expose .reveal _ => some (.action "reveal")
  | _ + 1, _ => none
termination_by structural fuel

/-- Voice and `GramRole` assignment (`semantics-v3.md` §3.2):
- Active (`.actor np`): primary patient inside `action` gets `.obj`.
- Passive (`.any`, `.by np`): promotes primary patient inside `action` to `.subj`. -/
def Subject.patientRole : Subject → GramRole
  | .actor _ => .obj
  | .any | .by _ => .subj

/-! ## Core DRT Stepping and Checking -/

structure StepOps where
  pred : DRS → Predicate → DRS × List Refusal
  np   : DRS → GramRole → Bool → NounPhrase → DRS × List ReferentId × List Refusal
  amt  : DRS → Amount → DRS × List Refusal
  cond : DRS → Condition → DRS × List Refusal
  ev   : DRS → GameEvent → DRS × List Refusal
  inst : StepPhase → GramRole → DRS → Instruction → DRS × List Refusal
  spec : DRS → StaticSpec → DRS × List Refusal
  cost : DRS → Cost → DRS × List Refusal
  ab   : DRS → Ability → List Refusal

def stepZoneScopeWith (ops : StepOps) (drs : DRS) (s : ZoneScope) : DRS × List Refusal :=
  match s with
  | .bare => (drs, [])
  | .possessedBy np =>
    let (drs', _, errs) := ops.np drs .other false np
    (drs', errs)

def stepPileWith (ops : StepOps) (drs : DRS) (p : Pile) : DRS × List Refusal :=
  match p with
  | .library s | .graveyard s => stepZoneScopeWith ops drs s
  | .that => (drs, [])
  | .exiledWith src =>
    let (drs', _, errs) := ops.np drs .other false src
    (drs', errs)

def stepZoneExprWith (ops : StepOps) (drs : DRS) (z : ZoneExpr) (fuel : Nat := 8) : DRS × List Refusal :=
  match fuel, z with
  | 0, _ => (drs, [.recursionLimit])
  | n + 1, .inCaller body =>
    let (callerDrs, errs) := stepZoneExprWith ops (enterCaller drs) body n
    (leaveCaller drs callerDrs, errs)
  | _ + 1, .zone _ s => stepZoneScopeWith ops drs s
  | _ + 1, .place _ p => stepPileWith ops drs p
  | _ + 1, .wherever | _ + 1, .anyZone => (drs, [])
termination_by structural fuel

def stepZoneExprListWith (ops : StepOps) (drs : DRS) (zs : List ZoneExpr) : DRS × List Refusal :=
  zs.foldl (fun (d, es) z =>
    let (d', e') := stepZoneExprWith ops d z
    (d', es ++ e')) (drs, [])

def stepEventSourceWith (ops : StepOps) (drs : DRS) (src : EventSource) : DRS × List Refusal :=
  match src with
  | .anywhere => (drs, [])
  | .zones zs | .anywhereBut zs => stepZoneExprListWith ops drs zs

def stepQuantityWith (ops : StepOps) (drs : DRS) (q : Quantity) (fuel : Nat := 8) : DRS × List Refusal :=
  match fuel, q with
  | 0, _ => (drs, [.recursionLimit])
  | n + 1, .inCaller body =>
    let (callerDrs, errs) := stepQuantityWith ops (enterCaller drs) body n
    (leaveCaller drs callerDrs, errs)
  | _ + 1, .range (some lo) (some hi) => (drs, if lo ≤ hi then [] else [.wellFormedQ])
  | _ + 1, .range _ _ | _ + 1, .worth _ => (drs, [])
  | _ + 1, .upToOf a | _ + 1, .exactlyOf a => ops.amt drs a
termination_by structural fuel

def ChoiceDomain.sort : ChoiceDomain → ChoiceSort
  | .nameOfCard _ => .quality .cardName
  | .colorOtherThan _ => .quality .color
  | .typeOtherThan _ => .quality (.subtype .creature)
  | .basicTypesOnly | .nonbasicTypesOnly => .quality (.subtype .land)
  | .number _ => .quality .number
  | .players _ => .player
  | .abilitiesAmong _ => .quality .ability

def stepUsageLimitWith (ops : StepOps) (drs : DRS) : UsageLimit → DRS × List Refusal
  | .mk (.atLeast amount) _ | .mk (.moreThan amount) _ => ops.amt drs amount
  | .mk (.additional quantity) _ => stepQuantityWith ops drs quantity

def NounPhrase.selfDefinedOk (np : NounPhrase) (fuel : Nat := 32) : Bool :=
  match fuel, np with
  | n + 1, .inCaller body | n + 1, .firstMention _ body
  | n + 1, .laterMention _ body | n + 1, .asType _ body _ => body.selfDefinedOk n
  | _ + 1, .this => true
  | _, _ => false
termination_by structural fuel

def Condition.negated (condition : Condition) (fuel : Nat := 32) : Bool :=
  match fuel, condition with
  | n + 1, .inCaller body => body.negated n
  | _ + 1, .not _ => true
  | _, _ => false
termination_by structural fuel

def markingOk : CondMarking → Condition → Bool
  | .asLongAs, _ | .ifSo, _ => true
  | .unless_, condition => condition.negated

def stepChoiceDomainWith (ops : StepOps) (drs : DRS) (cd : ChoiceDomain) : DRS × List Refusal :=
  match cd with
  | .nameOfCard p =>
    let (_, errs) := ops.pred (enterOwnScope drs) p
    (drs, errs ++ (match p.kind? with
      | some k => refuse (k == .object) (.kindMismatch .object k)
      | none => []))
  | .players p =>
    let (out, errs) := ops.pred drs p
    (out, errs ++ (match p.kind? with
      | some k => refuse (k == .player) (.kindMismatch .player k)
      | none => []))
  | .number q => stepQuantityWith ops drs q
  | .typeOtherThan subtype => (drs, refuse (subtype.type == some .creature) .subtypeType)
  | .abilitiesAmong terms =>
    (drs, refuse (!terms.isEmpty) .nonEmpty ++
      refuse (allKnownKeywordTerms terms) .knownKeywordTerm)
  | .colorOtherThan _ | .basicTypesOnly | .nonbasicTypesOnly => (drs, [])

def sortedDomainCheckWith (ops : StepOps) (drs : DRS) (sort : ChoiceSort)
    (domain : Option ChoiceDomain) : DRS × List Refusal :=
  match domain with
  | none => (drs, [])
  | some domain =>
    let (out, errs) := stepChoiceDomainWith ops drs domain
    (out, errs ++ refuse (domain.sort == sort) .kindAxisSort)

def stepColorTermWith (ops : StepOps) (drs : DRS) (ct : ColorTerm) : DRS × List Refusal :=
  match ct with
  | .lit _ => (drs, [])
  | .chosen ref =>
    let n := countChoice (.quality .color) drs
    (drs, if ref.ok n then [] else [.choiceRef ref (.quality .color) n])
  | .colorOf obj =>
    let (drs', _, errs) := ops.np drs .other false obj
    (drs', errs)

def stepDetPhraseWith (ops : StepOps) (drs : DRS) (d : DetPhrase) : DRS × List Refusal :=
  match d with
  | .target q | .count q _ => stepQuantityWith ops drs q
  | _ => (drs, [])

def stepSliceCountWith (ops : StepOps) (drs : DRS) (sc : SliceCount) : DRS × List Refusal :=
  match sc with
  | .counted q => stepQuantityWith ops drs q
  | .whole => (drs, [])
  | .totalling _ _ b => ops.amt drs b

def stepSubjectBeforeWith (ops : StepOps) (drs : DRS) (s : Subject) : DRS × List Refusal :=
  match s with
  | .actor np =>
    let (drs1, refIds, e1) := ops.np drs .subj true np
    let shape : ActorShape := {
      kind := (refIds.head?.bind drs1.findById?).elim (np.kindIn drs1 .player) (·.kind),
      isYou := np.isYou,
      selfDefined := false, ascribable := false, twoParties := false, opponentOnly := np.opponentOnly,
      refIds := refIds
    }
    let drs2 := drs1.push { det := .the, plur := .one, payload := .actor shape } .other false
    (drs2, e1)
  | .any | .by _ => (drs, [])

def stepSubjectAfterWith (ops : StepOps) (drs : DRS) (s : Subject) : DRS × List Refusal :=
  match s with
  | .actor _ => (dropActorFrame drs, [])
  | .any => (drs, [])
  | .by np =>
    let (drs', _, e) := ops.np drs .other true np
    (drs', e)

def durationPossessorOk : Option NounPhrase → Bool
  | none => true
  | some noun => noun.plur.isOne && noun.det.elim true (fun det =>
      det == .the || det == .target)

def boundaryPossessorOk : Option GameEvent → Bool
  | some (.beginningOf _ whose _) | some (.endOf _ whose _) => durationPossessorOk whose
  | _ => true

def countTurns (drs : DRS) : Nat :=
  (drs.visibleReferents.filter (fun ref => Kind.lte .turnRef ref.kind && ref.plur.isOne)).length

private def turnOwnerKindRefusals (noun : NounPhrase) : List Refusal :=
  noun.kind?.toList.flatMap (fun kind =>
    refuse (kind == .player) (.kindMismatch .player kind))

def stepTurnRefWith (ops : StepOps) (drs : DRS) (turn : TurnRef) : DRS × List Refusal :=
  match turn with
  | .next (some owner) | .each (some owner) =>
    let (out, _, errors) := ops.np drs .other false owner
    (out, errors ++ turnOwnerKindRefusals owner)
  | .that => (drs, refuse (countTurns drs == 1) (.turnAntecedent (countTurns drs)))
  | .current | .previous | .later | .next none | .each none => (drs, [])

def stepDurationWith (ops : StepOps) (drs : DRS) (d : Duration) : DRS × List Refusal :=
  match d with
  | .between s e =>
    let (drs1, e1) := match s with | some ev => ops.ev drs ev | none => (drs, [])
    let (drs2, e2) := match e with | some ev => ops.ev drs1 ev | none => (drs1, [])
    (drs2, e1 ++ e2 ++ refuse (boundaryPossessorOk s && boundaryPossessorOk e)
      .durationPossessor)
  | .forAsLongAs c => ops.cond (nextClause drs) c

def stepTimingWith (ops : StepOps) (drs : DRS) (tm : Timing) (fuel : Nat := 8) : DRS × List Refusal :=
  match fuel, tm with
  | 0, _ => (drs, [.recursionLimit])
  | _ + 1, .inPart _ whose turn =>
    let (drs1, errors) := match whose with
      | none => (drs, [])
      | some owner =>
        let (out, _, errors) := ops.np drs .other false owner
        (out, errors ++ turnOwnerKindRefusals owner)
    let (drs2, turnErrors) := stepTurnRefWith ops drs1 turn
    (drs2, errors ++ turnErrors)
  | _ + 1, .anyTiming => (drs, [])
  | _ + 1, .relativeToEvent _ ev | _ + 1, .whileDoing ev => ops.ev drs ev
  | _ + 1, .onlyIf c => ops.cond drs c
  | n + 1, .not t =>
    let (_, errs) := stepTimingWith ops drs t n
    (drs, errs)
  | n + 1, .and ts | n + 1, .or ts =>
    ts.foldl (fun (d, es) t =>
      let (d', e') := stepTimingWith ops d t n
      (d', es ++ e')) (drs, [])
termination_by structural fuel

def stepLookbackSpanWith (ops : StepOps) (drs : DRS) (sp : LookbackSpan) : DRS × List Refusal :=
  match sp with
  | .duration d t =>
    let (drs1, e1) := stepDurationWith ops drs d
    let (drs2, e2) := match t with
      | some tm => stepTimingWith ops drs1 tm
      | none => (drs1, [])
    (drs2, e1 ++ e2)
  | .thisWay | .triggering => (drs, [])

def kindCheck (expected : Kind) (found : Option Kind) : List Refusal :=
  found.toList.flatMap fun k => refuse (k == expected) (.kindMismatch expected k)

def stepAsThoughWith (ops : StepOps) (drs : DRS) (asThough : AsThough) :
    DRS × List Refusal :=
  match asThough with
  | .of predicate =>
    let (out, errors) := ops.pred drs predicate
    (out, errors ++ kindCheck .object predicate.kind?)
  | .greater _ amount =>
    let (out, errors) := ops.amt drs amount
    (out, errors ++ refuse (out.nextId == drs.nextId) .bindingless)
  | .mana .. => (drs, [])

private def stepDeonticAsThoughWith (ops : StepOps) (drs : DRS) (mods : DeonticMods) :
    DRS × List Refusal :=
  match mods.asThough with
  | none => (drs, [])
  | some asThough => stepAsThoughWith ops drs asThough

def deedAgentDomain (deed : Deed) : Option EntityDomain :=
  deedFacts deed >>= fun facts => facts.agentRole.sort.map (·.domain)

def enactActorKind (deed : Deed) : Kind :=
  if deedAgentDomain deed == some .object then .object else .player

private def instructionActorRefusals (drs : DRS) (expected : Kind) : List Refusal :=
  match actorShape? drs with
  | none => []
  | some shape => refuse (shape.kind == expected) .eventAgent

private def triggerActorRefusals (drs : DRS) : List Refusal :=
  match actorShape? drs with
  | none => []
  | some shape =>
    let refs := shape.refIds.filterMap drs.findById?
    refuse (shape.kind == .object && (refs.isEmpty || refs.all (fun ref =>
      match ref.payload with | .ability .. => true | _ => false))) .eventAgent

def Instruction.performed (action : Instruction) : Bool :=
  match action.scopeBody with
  | .conclude _ | .separateIntoPiles .. | .choose .. | .vote ..
  | .copy .. | .changeLife _ | .addMana .. | .draw _ | .expose ..
  | .search .. | .shuffle _ | .flipCoins .. | .rollDice .. | .rerollStored ..
  | .createObject .. | .enact .. | .pay .. | .skipPart .. | .addTurnPart ..
  | .procedure .. | .withContinuation .optional .. => true
  | _ => false

def clauseActorKind (action : Instruction) : Option Kind :=
  match action.scopeBody with
  | .draw _ | .procedure .. | .flipCoins .. | .rollDice .. => some .player
  | .dealDamage .. => some .object
  | _ => none

def stepDidClauseWith (ops : StepOps) (drs : DRS) (c : DidClause) : DRS × List Refusal :=
  match c with
  | ⟨subj, action, within⟩ =>
    let headErr := if action.eventHeadOk then [] else [.eventHead]
    let locusErrors := match action.scopeBody with
      | .enact verb _ _ (some zone) =>
        refuse (zone.sort.any (fun z => (actLociOf verb).elem z)) .lookbackLocus
      | _ => []
    let (drs1, e1) := stepSubjectBeforeWith ops drs subj
    let actorErrors := match clauseActorKind action, actorShape? drs1 with
      | some expected, some shape => kindCheck expected (some shape.kind)
      | _, _ => []
    let (drs2, e2) := ops.inst .postEvent subj.patientRole { drs1 with actionPattern := true } action
    let (drs3, e3) := stepSubjectAfterWith ops drs2 subj
    let (drs4, e4) := stepLookbackSpanWith ops drs3 within
    ({ drs4 with actionPattern := drs.actionPattern }, headErr ++ locusErrors ++ e1 ++ actorErrors ++ e2 ++ e3 ++ e4)

def stepCausingWith (ops : StepOps) (drs : DRS) (c : Causing) : DRS × List Refusal :=
  match c with
  | .source src =>
    let (drs', _, e) := ops.np drs .other false src
    (drs', e)
  | .event ev => ops.ev drs ev
  | .anEffect => (drs, [])

def stepWouldClauseWith (ops : StepOps) (drs : DRS) (wc : WouldClause) (fuel : Nat := 8) : DRS × List Refusal :=
  match fuel, wc with
  | 0, _ => (drs, [.recursionLimit])
  | _ + 1, .would subj action =>
    let headErr := if action.eventHeadOk then [] else [.eventHead]
    let (drs1, e1) := stepSubjectBeforeWith ops drs subj
    let (drs2, e2) := ops.inst .preEvent subj.patientRole { drs1 with actionPattern := true } action
    let (drs3, e3) := stepSubjectAfterWith ops drs2 subj
    ({ drs3 with actionPattern := drs.actionPattern }, headErr ++ e1 ++ e2 ++ e3)
  | n + 1, .wouldAny cs =>
    cs.foldl (fun (d, es) c =>
      let (d', e') := stepWouldClauseWith ops d c n
      (d', es ++ e')) (drs, [])
  | n + 1, .causedBy cause clause =>
    let (drs1, errors) := stepCausingWith ops drs cause
    let (drs2, clauseErrors) := stepWouldClauseWith ops drs1 clause n
    (drs2, errors ++ clauseErrors)
termination_by structural fuel

def stepCombatParticipationWith (ops : StepOps) (drs : DRS) (cp : CombatParticipation) : DRS × List Refusal :=
  match cp with
  | .attacking (some defn) =>
    let (drs', _, e) := ops.np drs .other true defn
    (drs', e)
  | .attacking none | .blocked _ | .outsideCombat => (drs, [])

def stepCombatUpdateWith (ops : StepOps) (drs : DRS) (cu : CombatUpdate) : DRS × List Refusal :=
  match cu with
  | .participation state => stepCombatParticipationWith ops drs state
  | .blocking _ atk =>
    let (drs', _, e) := ops.np drs .other true atk
    (drs', e)

def stepDoorWith (ops : StepOps) (drs : DRS) (d : Door) : DRS × List Refusal :=
  match d with
  | .thisDoor => (drs, [])
  | .doorOf _ room =>
    let (drs', _, e) := ops.np drs .other false room
    (drs', e)

def stepDividedVerbWith (ops : StepOps) (drs : DRS) (v : DividedVerb) : DRS × List Refusal :=
  match v with
  | .damage src =>
    let (drs', _, e) := ops.np drs .other false src
    (drs', e)
  | .counters _ => (drs, [])

def stepCounterKindSourceWith (ops : StepOps) (drs : DRS) (k : CounterKindSource) : DRS × List Refusal :=
  match k with
  | .sameAs src =>
    let (drs', _, e) := ops.np drs .other false src
    (drs', e)
  | .printed kind => (drs, refuse kind.known .knownCounter)
  | _ => (drs, [])

def stepAbilityListWith (ops : StepOps) (drs : DRS) (abs : List Ability) : DRS × List Refusal :=
  (drs, abs.foldl (fun es a => es ++ ops.ab drs a) [])

def stepNounAlternativesWith (ops : StepOps) (drs : DRS) (role : GramRole)
    (inArgSlot : Bool) (alternatives : List NounPhrase) : DRS × List ReferentId × List Refusal :=
  alternatives.foldl (fun (out, ids, errs) phrase =>
    let branch := { drs with nextId := out.nextId }
    let (result, branchIds, branchErrors) := ops.np branch role inArgSlot phrase
    let additions := result.referents.filter (fun r => r.id ≥ out.nextId)
    let earlier := out.referents.filter (fun r => r.id ≥ drs.nextId)
    let inherited := (result.referents.filter (fun r => r.id < drs.nextId)).map fun r =>
      match out.findById? r.id with
      | none => r
      | some prior =>
        { r with
          pronounResolved := prior.pronounResolved || r.pronounResolved
          idents := mergeIdents prior.idents r.idents }
    let merged := { result with
      referents := additions ++ earlier ++ inherited
      pendingRoles := (out.pendingRoles ++ result.pendingRoles).eraseDups
      resolvedIds := (out.resolvedIds ++ result.resolvedIds).eraseDups
      relativeUses := out.relativeUses + (result.relativeUses - drs.relativeUses) }
    (merged, ids ++ branchIds, errs ++ branchErrors))
    (drs, [], refuse (alternatives.length ≥ 2) .atLeastTwo)

def stepGameEventListWith (ops : StepOps) (drs : DRS) (evs : List GameEvent) : DRS × List Refusal :=
  evs.foldl (fun (d, es) e =>
    let (d', e') := ops.ev d e
    (d', es ++ e')) (drs, [])

def ManaSymbol.ok : ManaSymbol → Bool
  | .hybrid left right => halvesDistinct left right
  | .phyrexian color second => phyrexianDistinct color second
  | _ => true

def ManaSymbol.check (m : ManaSymbol) : List Refusal := refuse m.ok .manaSymbolOk

def ManaCost.check (c : ManaCost) : List Refusal := c.flatMap ManaSymbol.check

def TypeLineChanges.hasWrites (c : TypeLineChanges) : Bool :=
  c.supertypes.isSome || c.types.isSome || c.subtypes.isSome

def TypeLineChanges.hasValues (c : TypeLineChanges) : Bool :=
  hasAnyTypeCharacteristic (c.supertypes.getD []) (c.types.getD []) (c.subtypes.getD [])

def TypeLineChanges.canonical (c : TypeLineChanges) : Bool :=
  supersDistinct (c.supertypes.getD []) && typesDistinct (c.types.getD [])

def CharacteristicEdit.writesLine : CharacteristicEdit → Bool
  | .typeLine _ c => c.hasWrites
  | _ => true

def colorOpOk : QualityOp → ColorSpec → Bool
  | .sets, _ => true
  | _, .some [] => false
  | _, _ => true

def Predicate.qualityReadOk (p : Predicate) (fuel : Nat := 64) : Bool :=
  match fuel with
  | 0 => false
  | n + 1 => match p with
    | .bracketed body => body.qualityReadOk n
    | .ofChosen _ _ | .ofYourChoice _ _ | .named _ | .shares _ _ _ => true
    | _ => false
termination_by structural fuel

def CharacteristicEdit.shape : CharacteristicEdit → List Refusal
  | .typeLine op c =>
    refuse (op == .sets || !c.hasWrites || c.hasValues) .lineNonEmpty ++
      refuse c.canonical .distinct
  | .colors op cs => refuse cs.ok .distinct ++ refuse (colorOpOk op cs) .nonEmpty
  | .chosenQuality _ q => refuse q.qualityReadOk .qualityRead
  | _ => []

def CharacteristicEdit.shapeAll (edits : List CharacteristicEdit) : List Refusal :=
  edits.flatMap CharacteristicEdit.shape

def KeywordParam.shape : KeywordParam → KeywordParamShape
  | .cost _ => .cost
  | .quality _ => .quality
  | .subject _ => .subject
  | .number _ => .number
  | .deckCondition _ => .deckCondition
  | .ability _ => .ability

def keywordSchemaFor (k : KeywordLabel) (params : List KeywordParam) : Option KeywordSchema :=
  (keywordSchemas k).find? (fun schema => schema.shapes == params.map KeywordParam.shape)

def keywordParamFits (k : KeywordLabel) (params : List KeywordParam) : Bool :=
  (keywordSchemaFor k params).isSome

def checkKeywordParamsWith (ops : StepOps) (drs : DRS) (schema : KeywordSchema)
    (params : List KeywordParam) : List Refusal :=
  (params.zipIdx).flatMap fun (param, i) =>
    match param with
    | .cost c => (ops.cost (enterOwnScope drs) c).2
    | .quality p =>
      let expected := ((schema[i]?).bind (·.qualityDomain)).getD (p.kindOr .object)
      (ops.pred drs p).2 ++ kindCheck expected p.kind?
    | .subject p => (ops.pred (enterOwnScope drs) p).2
    | .number a => (ops.amt (enterOwnScope drs) a).2
    | .deckCondition _ => []
    | .ability a => ops.ab drs a

def stepAbilitySelectionWith (ops : StepOps) (drs : DRS) (sel : AbilitySelection) : DRS × List Refusal :=
  match sel with
  | .specified als =>
    (drs, refuse (!als.isEmpty) .nonEmpty ++ als.foldl (fun es al =>
      match al with
      | .written ab => es ++ ops.ab drs ab
      | .term term => es ++ refuse term.known .knownKeywordTerm) [])
  | .allExcept (some p) => ops.pred drs p
  | .family family => (drs, refuse family.known .knownKeywordTerm)
  | .printedAfter k => (drs, refuse (knownKeyword k) .knownKeywordTerm)
  | .allExcept none => (drs, [])

def stepCharacteristicEditWith (ops : StepOps) (drs : DRS) (ed : CharacteristicEdit) : DRS × List Refusal :=
  match ed with
  | .manaCost cost => (drs, (cost.map ManaCost.check).getD [])
  | .stat _ _ amt => ops.amt drs amt
  | .chosenQuality _ p => ops.pred drs p
  | .addedAbilities abs => stepAbilityListWith ops (choiceContext drs) abs
  | .removedAbilities sel => stepAbilitySelectionWith ops drs sel
  | _ => (drs, [])

def stepCharacteristicEditListWith (ops : StepOps) (drs : DRS) (eds : List CharacteristicEdit) : DRS × List Refusal :=
  eds.foldl (fun (d, es) ed =>
    let (d', e') := stepCharacteristicEditWith ops d ed
    (d', es ++ e')) (drs, [])

def stepCopyExceptWith (ops : StepOps) (drs : DRS) (ce : CopyExcept) : DRS × List Refusal :=
  match ce with
  | .edits eds =>
    let (_, es) := stepCharacteristicEditListWith ops drs eds
    (drs, es ++ refuse (!eds.isEmpty && eds.all CharacteristicEdit.writesLine) .lineNonEmpty ++
      CharacteristicEdit.shapeAll eds)
  | .ability ab => (drs, ops.ab (choiceContext drs) ab)
  | .thisAbility => (drs, [])
  | .entersWithCounters amt kind _ =>
    let (_, es) := ops.amt drs amt
    (drs, es ++ refuse kind.known .knownCounter)

def stepCopyExceptListWith (ops : StepOps) (drs : DRS) (ces : List CopyExcept) : DRS × List Refusal :=
  ces.foldl (fun (d, es) c =>
    let (d', e') := stepCopyExceptWith ops d c
    (d', es ++ e')) (drs, [])

def stepTextChangeWith (ops : StepOps) (drs : DRS) (tc : TextChange) : DRS × List Refusal :=
  match tc with
  | .appendTextOf src | .exchangeWith src =>
    let (drs', _, e) := ops.np drs .other false src
    (drs', e ++ kindCheck .object src.kind?)
  | .replaceWord .. => (drs, tc.shape)
  | .removeBracketed | .targetToEach => (drs, [])

def stepSearchScopeWith (ops : StepOps) (drs : DRS) (sc : SearchScope) : DRS × List Refusal :=
  match sc with
  | .oneZone z => stepZoneExprWith ops drs z
  | .someZones (some whose) _ =>
    let (drs', _, e) := ops.np drs .other false whose
    (drs', e)
  | .someZones none _ => (drs, [])

def stepVisibleThingWith (ops : StepOps) (drs : DRS) (vt : VisibleThing) : DRS × List Refusal :=
  match vt with
  | .topOfLibrary | .wholeHand => (drs, [])
  | .objects np =>
    let (drs', _, e) := ops.np drs .obj true np
    (drs', e)

def stepExchangedWith (ops : StepOps) (drs : DRS) (ex : Exchanged) : DRS × List Refusal :=
  match ex with
  | .lifeTotals parties =>
    let (drs1, _, e1) := ops.np drs .obj true parties
    let drs2 := (drs1.push (outcomeB .lifeGained) .other false).push (outcomeB .lifeLost) .other false
    (drs2, e1)
  | .controlOf l r | .cardsAcross l r =>
    let (drs1, _, e1) := ops.np drs .obj true l
    let (drs2, _, e2) := ops.np drs1 .other true r
    (drs2, e1 ++ e2)
  | .zones l r =>
    let (drs1, e1) := stepZoneExprWith ops drs l
    let (drs2, e2) := stepZoneExprWith ops drs1 r
    (drs2, e1 ++ e2)
  | .values l r =>
    let (drs1, e1) := ops.amt drs l
    let (drs2, e2) := ops.amt drs1 r
    (drs2, e1 ++ e2)

def stepFlipScopeWith (ops : StepOps) (drs : DRS) (fs : FlipScope) : DRS × List Refusal :=
  match fs with
  | .count a => ops.amt drs a
  | .per np =>
    let (drs', _, e) := ops.np drs .other false np
    (drs', e)

def stepIgnoredOutcomesWith (ops : StepOps) (drs : DRS) (io : IgnoredOutcomes) : DRS × List Refusal :=
  match io with
  | .extreme _ | .allBut _ => (drs, [])
  | .chosen optChooser amt =>
    let (drs1, e1) := match optChooser with
      | some c => let (d, _, e) := ops.np drs .subj true c; (d, e)
      | none => (drs, [])
    let (drs2, e2) := ops.amt drs1 amt
    (drs2, e1 ++ e2)

def stepBallotWith (ops : StepOps) (drs : DRS) (b : Ballot) : DRS × List Refusal :=
  match b with
  | .byLabel labels => (drs, refuse (ballotLabelsOk labels) .ballotLabelsOk)
  | .byCandidate np =>
    let (drs', _, e) := ops.np drs .obj true np
    (drs', e ++ refuse (np.det == some .a || np.det == some .target || np.det == some .count)
      .choosable)

def stepRollRowListWith (ops : StepOps) (drs : DRS) (rows : List RollRow) : DRS × List Refusal :=
  rows.foldl (fun (d, es) ⟨q, inst⟩ =>
    let (d1, e1) := stepQuantityWith ops d q
    let (d2, e2) := ops.inst .postEvent .obj (nextClause d1) inst
    (d2, es ++ e1 ++ e2)) (drs, [])

def stepRepetitionWith (ops : StepOps) (drs : DRS) (rep : Repetition) : DRS × List Refusal :=
  match rep with
  | .again | .anyNumber | .againExcludingChosen => (drs, [])
  | .moreTimes times => ops.amt drs times
  | .untilCond cond => ops.cond drs cond
  | .fixed times body =>
    let (drs1, e1) := ops.amt drs times
    let (drs2, e2) := ops.inst .postEvent .obj (nextClause drs1) body
    let drs3 := drs2.push (outcomeB .repeatCount) .other false
    (drs3, e1 ++ e2)

def stepModeChangeWith (ops : StepOps) (drs : DRS) (mc : ModeChange) : DRS × List Refusal :=
  match mc with
  | .allInstead optCost optCond =>
    let (drs1, e1) := match optCost with | some c => ops.cost drs c | none => (drs, [])
    let (drs2, e2) := match optCond with | some cond => ops.cond drs1 cond | none => (drs1, [])
    (drs2, e1 ++ e2)
  | .extraModesCost c => ops.cost drs c
  | .setQuantity q => stepQuantityWith ops drs q
  | .repeatable => (drs, [])
  | .notChosenBefore dur => stepDurationWith ops drs dur

def stepModeListWith (ops : StepOps) (drs : DRS) (modes : List (Option Cost × Option Nat × Instruction)) : DRS × List Refusal :=
  modes.foldl (fun (d, es) (optCost, _, inst) =>
    let (d1, e1) := match optCost with | some c => ops.cost drs c | none => (drs, [])
    let (d2, e2) := ops.inst .postEvent .obj (nextClause d1) inst
    let _ := d
    (d2, es ++ e1 ++ e2)) (drs, [])

def stepProducedManaWith (ops : StepOps) (drs : DRS) (pm : ProducedMana) : DRS × List Refusal :=
  let (out, errors) :=
    match pm with
    | .asPrintedCost subj | .producedByEvent subj | .couldProduce subj
    | .amongColorsOf subj | .lastNoted subj =>
      let (drs', _, e) := ops.np drs .other false subj
      (drs', e)
    | _ => (drs, [])
  (out, errors ++ pm.localChecks drs)

def stepSpendPurposeWith (ops : StepOps) (drs : DRS) (sp : SpendPurpose) : DRS × List Refusal :=
  match sp with
  | .toCast p | .toActivate (some p) => ops.pred drs p
  | .toActivate none | .toPay _ => (drs, [])

def stepSpendPurposeListWith (ops : StepOps) (drs : DRS) (sps : List SpendPurpose) : DRS × List Refusal :=
  sps.foldl (fun (d, es) sp =>
    let (d', e') := stepSpendPurposeWith ops d sp
    (d', es ++ e')) (drs, [])

def stepManaRiderWith (ops : StepOps) (drs : DRS) (mr : ManaRider) : DRS × List Refusal :=
  let (out, errors) :=
    match mr with
    | .spendOnly sps | .spendNotOn sps => stepSpendPurposeListWith ops drs sps
    | .onSpent _ _ sp says =>
      let (drs1, _, e1) := ops.np drs .subj true sp
      let (drs2, e2) := ops.inst .postEvent .obj (nextClause drs1) says
      (drs2, e1 ++ e2)
  (out, errors ++ mr.localChecks)

def stepManaRiderListWith (ops : StepOps) (drs : DRS) (mrs : List ManaRider) : DRS × List Refusal :=
  mrs.foldl (fun (d, es) mr =>
    let (d', e') := stepManaRiderWith ops d mr
    (d', es ++ e')) (drs, [])

def stepPipPaidListWith (ops : StepOps) (drs : DRS) (ps : List PipPaid) : DRS × List Refusal :=
  ps.foldl (fun (d, es) p =>
    match p with
    | .generic => (d, es)
    | .ofColor ct =>
      let (d', e') := stepColorTermWith ops d ct
      (d', es ++ e')) (drs, [])

def stepManaSubstituteWith (ops : StepOps) (drs : DRS) (ms : ManaSubstitute) : DRS × List Refusal :=
  let (drs1, e1) := ops.inst .postEvent .obj drs ms.action
  let (drs2, e2) := stepPipPaidListWith ops drs1 ms.pays
  (drs2, e1 ++ e2)

def stepCostShiftWith (ops : StepOps) (drs : DRS) (cs : CostShift) : DRS × List Refusal :=
  let (out, errors) :=
    match cs with
    | .less amt optFl =>
      let (drs1, e1) := ops.amt drs amt
      let (drs2, e2) := match optFl with | some fl => ops.amt drs1 fl | none => (drs1, [])
      (drs2, e1 ++ e2)
    | .more amt => ops.amt drs amt
    | .bySymbols (.lit _) _ _ => (drs, [])
    | .bySymbols (.manaCostOf obj) _ _ =>
      let (drs', _, e) := ops.np drs .other false obj
      (drs', e)
  (out, errors ++ cs.localChecks)

def stepDamageAgentWith (ops : StepOps) (drs : DRS) (da : DamageAgent) : DRS × List Refusal :=
  match da with
  | .unattributed => (drs, [])
  | .dealtBy src =>
    let (drs', _, e) := ops.np drs .subj true src
    (drs', e)

def stepDamageScopeWith (ops : StepOps) (drs : DRS) (ds : DamageScope) : DRS × List Refusal :=
  match ds with
  | .everywhere => (drs, [])
  | .toRecipient rec =>
    let (drs', _, e) := ops.np drs .obj true rec
    (drs', e)

def stepPreventCutWith (ops : StepOps) (drs : DRS) (pc : PreventCut) : DRS × List Refusal :=
  match pc with
  | .all | .half _ => (drs, [])
  | .some amt | .shield amt | .allBut amt => ops.amt drs amt

def stepDamageOpWith (ops : StepOps) (drs : DRS) (dop : DamageOp) : DRS × List Refusal :=
  match dop with
  | .prevent cut optAlso =>
    let (drs1, e1) := stepPreventCutWith ops drs cut
    let drs2 := drs1.push (outcomeB .damagePrevented) .other false
    let (drs3, e2) := match optAlso with
      | some inst => ops.inst .postEvent .obj (nextClause drs2) inst
      | none => (drs2, [])
    (drs3, e1 ++ e2)
  | .redirect cut to =>
    let (drs1, e1) := stepPreventCutWith ops drs cut
    let (drs2, _, e2) := ops.np drs1 .other true to
    (drs2, e1 ++ e2)
  | .scale (.multiplied _) | .scale (.halved _) => (drs, [])
  | .scale (.shifted _ amt) => ops.amt drs amt

def stepEntryRiderWith (ops : StepOps) (drs : DRS) (r : EntryRider) : DRS × List Refusal :=
  match r with
  | .entersAs _ | .entersTransformed | .entersMelded _ | .faceDown _ => (drs, [])
  | .entersAttacking (some defn) =>
    let (drs', _, e) := ops.np drs .other true defn
    (drs', e)
  | .entersAttacking none => (drs, [])
  | .withCounters amt ks _ =>
    let (drs1, e1) := ops.amt drs amt
    let (drs2, e2) := stepCounterKindSourceWith ops drs1 ks
    (drs2, e1 ++ e2)
  | .under ctrl =>
    let (drs', _, errs) := ops.np drs .other false ctrl
    (drs', errs)
  | .asCopyOf _ src exs =>
    let (drs1, _, e1) := ops.np drs .other false src
    let (drs2, e2) := stepCopyExceptListWith ops drs1 exs
    (drs2, e1 ++ e2)

def stepEntryRiderListWith (ops : StepOps) (drs : DRS) (rs : List EntryRider) : DRS × List Refusal :=
  rs.foldl (fun (d, es) r =>
    let (d', e') := stepEntryRiderWith ops d r
    (d', es ++ e')) (drs, [])

def stepPilePlacementWith (ops : StepOps) (drs : DRS) (pp : PilePlacement) : DRS × List Refusal :=
  let (out, errors) :=
    match pp with
    | .inOrder _ | .nthFromEnd _ | .orOtherEnd | .faceDown _ => (drs, [])
    | .withCounters amt ks _ =>
      let (drs1, e1) := ops.amt drs amt
      let (drs2, e2) := stepCounterKindSourceWith ops drs1 ks
      (drs2, e1 ++ e2)
  (out, errors ++ pp.localChecks)

def stepMoveRiderWith (ops : StepOps) (drs : DRS) (mr : MoveRider) : DRS × List Refusal :=
  match mr with
  | .entry r => stepEntryRiderWith ops drs r
  | .placement pp => stepPilePlacementWith ops drs pp

def stepMoveRiderListWith (ops : StepOps) (drs : DRS) (mrs : List MoveRider) : DRS × List Refusal :=
  mrs.foldl (fun (d, es) mr =>
    let (d', e') := stepMoveRiderWith ops d mr
    (d', es ++ e')) (drs, [])

def stepCharacteristicsWith (ops : StepOps) (drs : DRS) (c : Characteristics) : DRS × List Refusal :=
  let (out, errs) := [c.power, c.toughness, c.loyalty, c.defense].foldl
    (fun (d, es) a => match a with
      | none => (d, es)
      | some a => let (d', e) := ops.amt d a; (d', es ++ e)) (drs, [])
  let abilityErrors := (stepAbilityListWith ops (choiceContext drs) c.text).2
  (out, errs ++ abilityErrors)

def stepCharacteristicBundleWith (ops : StepOps) (drs : DRS)
    (bundle : CharacteristicBundle) : DRS × List Refusal :=
  let (out, errs) := stepCharacteristicsWith ops drs bundle.characteristics
  bundle.qualities.foldl (fun (d, es) q => match q with
    | .withEveryType _ => (d, es)
    | .withQuality p =>
      let (d', e) := ops.pred d p
      (d', es ++ e ++ refuse p.qualityReadOk .qualityRead)) (out, errs)

def stepTokenSpecMetadataWith (ops : StepOps) (drs : DRS) (ts : TokenSpec)
    (fuel : Nat := 64) : DRS × List CardType × List Refusal :=
  match fuel, ts with
  | 0, _ => (drs, [], [.recursionLimit])
  | n + 1, .inCaller body =>
    let (out, types, errors) := stepTokenSpecMetadataWith ops (enterCaller drs) body n
    (leaveCaller drs out, types, errors)
  | _ + 1, .written bundle =>
    let (out, errors) := stepCharacteristicBundleWith ops drs bundle
    let c := bundle.characteristics
    (out, normalizeTypes c.types, errors ++ refuse
      (colorsDistinct c.colors && typesDistinct c.types && supersDistinct c.supertypes) .distinct)
  | _ + 1, .doubleFaced front back =>
    let (out, frontErrors) := stepCharacteristicBundleWith ops drs front
    let (_, backErrors) := stepCharacteristicBundleWith ops drs back
    (out, normalizeTypes front.characteristics.types,
      frontErrors ++ backErrors ++ [front.characteristics, back.characteristics].flatMap (fun c =>
        refuse (colorsDistinct c.colors && typesDistinct c.types && supersDistinct c.supertypes) .distinct))
  | _ + 1, .asThose => (drs, tyOfThoseAny .token drs, [])
  | _ + 1, .copyOf source exceptions =>
    let (selected, ids, errors) := ops.np drs .other false source
    let sources : List Referent := ids.filterMap selected.findById?
    let types := match sources with
      | [] => []
      | first :: rest => rest.foldl (fun acc (ref : Referent) => commonTypes acc ref.ty) first.ty
    let (out, exceptionErrors) := stepCopyExceptListWith ops selected exceptions
    (out, types, errors ++ exceptionErrors)
termination_by structural fuel

def stepTokenSpecWith (ops : StepOps) (drs : DRS) (ts : TokenSpec) : DRS × List Refusal :=
  let (out, _, errors) := stepTokenSpecMetadataWith ops drs ts
  (out, errors)

def stepCreationSpecMetadataWith (ops : StepOps) (drs : DRS)
    (spec : CreationSpec) : DRS × List CardType × List Refusal :=
  match spec with
  | .token token riders =>
    let (afterToken, types, errors) := stepTokenSpecMetadataWith ops drs token
    let (out, riderErrors) := stepEntryRiderListWith ops afterToken riders
    (out, types, errors ++ riderErrors ++ spec.localChecks)
  | .emblem _ abilities =>
    (drs, [], (stepAbilityListWith ops (choiceContext drs) abilities).2 ++ spec.localChecks)

def stepCreationSpecWith (ops : StepOps) (drs : DRS) (spec : CreationSpec) : DRS × List Refusal :=
  let (out, _, errors) := stepCreationSpecMetadataWith ops drs spec
  (out, errors)

def stepPlayPaymentWith (ops : StepOps) (drs : DRS) (pay : PlayPayment) : DRS × List Refusal :=
  match pay with
  | .itsOwnCost | .withoutPaying => (drs, [])
  | .payingInstead c | .faceDown _ c => ops.cost drs c

def stepDeltaWith (ops : StepOps) (drs : DRS) (d : Delta Amount) : DRS × List Refusal :=
  match d with
  | .up a | .down a | .set a => ops.amt drs a

def stepOps : Nat → StepOps
  | 0 => {
      pred := fun drs _ => (drs, [.recursionLimit]),
      np   := fun drs _ _ _ => (drs, [], [.recursionLimit]),
      amt  := fun drs _ => (drs, [.recursionLimit]),
      cond := fun drs _ => (drs, [.recursionLimit]),
      ev   := fun drs _ => (drs, [.recursionLimit]),
      inst := fun _ _ drs _ => (drs, [.recursionLimit]),
      spec := fun drs _ => (drs, [.recursionLimit]),
      cost := fun drs _ => (drs, [.recursionLimit]),
      ab   := fun _ _ => [.recursionLimit]
    }
  | fuel + 1 =>
    let ops := stepOps fuel
    {
      pred := fun drs p =>
        match p with
        | .inCaller body =>
          let (callerDrs, errs) := ops.pred (enterCaller drs) body
          (leaveCaller drs callerDrs, errs)
        | .bracketed body | .not body => ops.pred drs body
        | .chosenPlayer ref =>
          let n := countChoice .player drs
          (drs, if ref.ok n then [] else [.choiceRef ref .player n])
        | .qualityNoun q (some dom) | .ofYourChoice q (some dom) =>
          sortedDomainCheckWith ops drs (.quality q) (some dom)
        | .qualityNoun _ none | .ofYourChoice _ none => (drs, [])
        | .ofChosen ref q =>
          let n := countChoice (.quality q) drs
          (drs, if ref.ok n then [] else [.choiceRef ref (.quality q) n])
        | .hasPossessor _ np =>
          let (drs', _, errs) := ops.np drs .other false np
          (drs', errs)
        | .did clause =>
          let subDrs := nextClause drs
          let k := drs.relativeKind.getD .object
          let whichRef := kindBinding k
          let binderId := subDrs.nextId
          let subDrs' := { subDrs.push whichRef .other false with
            relativeBinder := some binderId, relativeUses := 0 }
          let (out, errs) := stepDidClauseWith ops subDrs' clause
          (leaveRelative drs out, errs ++ refuse (out.relativeUses != 0) .lookbackSubject)
        | .compare _ _ b => ops.amt drs b
        | .superlative _ _ dom => ops.pred drs dom
        | .withMostVotes =>
          let n := countOutcomes .voteHeld drs
          (drs, if n == 1 then [] else [.outcomeInScope .voteHeld n])
        | .choseExtreme _ =>
          let drs' := drs.push (outcomeB .namedNumber) .other false
          (drs', if countManys (.quality .number) drs != 0 then [] else [.numberChoiceInScope])
        | .compareOver dom m _ b =>
          let (drs1, e1) := ops.pred drs dom
          let (drs2, e2) := ops.amt drs1 m
          let (drs3, e3) := ops.amt drs2 b
          (drs3, e1 ++ e2 ++ e3)
        | .inZone z => stepZoneExprWith ops drs z
        | .inPile pile =>
          let (drs', _, errs) := ops.np drs .other false pile
          (drs', errs)
        | .inCombat _ (some cp) | .hasDesignation _ (some cp) | .related _ (some cp) =>
          let (drs', _, errs) := ops.np drs .other false cp
          (drs', errs)
        | .shares _ _ src =>
          let (drs', ids, errs) := ops.np drs .other false src
          let found := ids.head?.bind drs'.findById?
          (drs', errs ++ kindCheck .object (found.map (·.kind)))
        | .hasKeyword term => (drs, refuse term.known .knownKeywordTerm)
        | .abilityHead family => (drs, refuse family.known .knownKeywordTerm)
        | .counterKindOn src | .abilityOf src
        | .activatedBy src | .exiledWith src | .otherThan src =>
          let (drs', _, errs) := ops.np drs .other false src
          (drs', errs)
        | .targets src _ =>
          let (out, ids, errors) := ops.np drs .other false src
          let targetKind := (ids.head?.bind out.findById?).map (·.kind) |>.getD .object
          let headKind := drs.relativeKind.getD .object
          (out, errors ++ refuse targetKind.targetable (.targetable targetKind) ++
            refuse headKind.targeter (.targeter headKind))
        | .coinCameUp _ => (drs, if coinFlipInScope drs then [] else [.coinFlipInScope])
        | .and ps | .or ps =>
          let headKind := p.kind? <|> drs.relativeKind
          let (out, errs) := ps.foldl (fun (d, es) q =>
            let (d', e') := ops.pred { d with relativeKind := headKind } q
            (d', es ++ e')) (drs, [])
          let shapeErrors := match p with
            | .and _ => refuse (Predicate.contradictionFree ps) .contradictionFree
            | _ => []
          ({ out with relativeKind := drs.relativeKind }, errs ++ shapeErrors)
        | _ => (drs, []),

      np := fun drs role inArgSlot np =>
        match np with
        | .inCaller body =>
          let callerDrs := enterCaller drs
          let (callerDrs', refIds, errs) := ops.np callerDrs role inArgSlot body
          (leaveCaller drs callerDrs' refIds, refIds, errs)
        | .firstMention id body =>
          let (drs', refIds, errs) := ops.np drs role inArgSlot body
          let drs'' := refIds.foldl (fun d rid => recordIdentityById id d rid) drs'
          (drs'', refIds, errs)
        | .laterMention id _ =>
          let matching := drs.visibleReferents.filter (fun r => r.idents.elem id)
          match matching with
          | [] => (drs, [], [.bindingIdentity id])
          | rs =>
            let ids := rs.map (·.id)
            let drs' := if inArgSlot then ids.foldl (fun d rid => d.recordMention rid role true) drs else drs
            (drs', ids, [])
        | .pro reach plur =>
          let winners := resolveCandidates reach plur drs inArgSlot
          match winners with
          | [w] =>
            let drs1 := drs.recordMention w.id role inArgSlot reach.isBarePronoun
            (drs1, [w.id], [])
          | _ => (drs, [], [.anaphor reach plur winners.length])
        | .refl plur =>
          let winners := resolveReflexive plur drs
          match winners with
          | [w] =>
            let drs' := drs.recordMention w.id role inArgSlot
            (drs', [w.id], [])
          | _ => (drs, [], [.anaphor (.word .permanent) plur winners.length])
        | .which =>
          match drs.relativeBinder.bind (fun id =>
              if drs.isVisible id then drs.findById? id else none) with
          | some r =>
            let drs' := drs.recordMention r.id role inArgSlot
            ({ drs' with relativeUses := drs.relativeUses + 1 }, [r.id], [])
          | none => (drs, [], [.anaphor (.word .permanent) .one 0])
        | .described detP pred =>
          let (drs1, e1) := stepDetPhraseWith ops drs detP
          let (drs2, e2) := ops.pred { drs1 with relativeKind := pred.kind? } pred
          let ref := bindFor drs2 detP.det detP.plur (pred.kindOr .object) pred
          let rid := drs2.nextId
          let drs3 := drs2.push ref role inArgSlot
          (drs3, [rid], e1 ++ e2)
        | .and ns =>
          ns.foldl (fun (d, ids, es) n =>
            let (d', ids', e') := ops.np d role inArgSlot n
            (d', ids ++ ids', es ++ e')) (drs, [], refuse (ns.length ≥ 2) .atLeastTwo)
        | .or ns => stepNounAlternativesWith ops drs role inArgSlot ns
        | .eachOf grp =>
          let (drs1, _, e1) := ops.np drs .other false grp
          let ref : Referent := { det := .each, plur := .one, payload := .object [] none none none none none }
          let rid := drs1.nextId
          let drs2 := drs1.push ref role inArgSlot
          (drs2, [rid], e1)
        | .someOf sc optP grp =>
          let (drs1, e1) := stepSliceCountWith ops drs sc
          let (drs2, e2) := match optP with
            | some p => ops.pred drs1 p
            | none => (drs1, [])
          let (drs3, grpIds, e3) := ops.np drs2 .other false grp
          let groupRef := grpIds.head?.bind drs3.findById?
          let ref : Referent := {
            det := .part, plur := sc.plur,
            payload := selectionPayload (optP.map Predicate.seedTy |>.getD []) groupRef sc
          }
          let rid := drs3.nextId
          let drs4 := drs3.push ref role inArgSlot
          (drs4, [rid], e1 ++ e2 ++ e3)
        | .slice _ amt p =>
          let (drs1, e1) := ops.amt drs amt
          let (drs2, e2) := stepPileWith ops drs1 p
          let ref : Referent := {
            det := .the, plur := np.plur,
            payload := slicePayload p.sort amt
          }
          let rid := drs2.nextId
          let drs3 := drs2.push ref role inArgSlot
          (drs3, [rid], e1 ++ e2)
        | .run _ stop p =>
          let (drs1, e1) := ops.pred drs stop
          let (drs2, e2) := stepPileWith ops drs1 p
          let payloads := runPayloads p.sort stop.seedTy
          let stoppedRef : Referent := {
            det := .part, plur := .one,
            payload := payloads.2
          }
          let stoppedId := drs2.nextId
          let drs3 := drs2.push stoppedRef .other false
          let runRef : Referent := {
            det := .the, plur := np.plur,
            payload := payloads.1
          }
          let runId := drs3.nextId
          let drs4 := drs3.push runRef role inArgSlot
          (drs4, [runId, stoppedId], e1 ++ e2)
        | .pileOf q optBy =>
          let (drs1, e1) := stepSliceCountWith ops drs q
          let (drs2, e2) := match optBy with
            | some b => let (d', _, e) := ops.np drs1 .other false b; (d', e)
            | none => (drs1, [])
          let zn := zoneOfReach (.word .pile) .many drs2
          let fc := faceOfReach (.word .pile) .many drs2
          let ref : Referent := {
            det := .part, plur := q.plur,
            payload := .pile zn none fc
          }
          let rid := drs2.nextId
          let drs3 := drs2.push ref role inArgSlot
          let n := countReach (.word .pile) .many drs2
          let e3 := if n == 1 then [] else [.anaphor (.word .pile) .many n]
          (drs3, [rid], e1 ++ e2 ++ e3)
        | .oneEachOf roles pool =>
          let (drs1, e1) := roles.foldl (fun (d, es) r =>
            let (d', e') := ops.pred d r
            (d', es ++ e')) (drs, [])
          let (drs2, poolIds, e2) := ops.np drs1 .other false pool
          let (poolTy, poolZone) := match poolIds.head?.bind drs2.findById? with
            | some r => (r.ty, r.zone)
            | none => ([], none)
          let ref : Referent := {
            det := .a, plur := .many,
            payload := .object poolTy poolZone none none none none
          }
          let rid := drs2.nextId
          let drs3 := drs2.push ref role inArgSlot
          (drs3, [rid], e1 ++ e2)
        | .theRest k pl =>
          let ok := theRestOk k drs
          let zn := zoneOfGroup k drs
          let ty := tyOfGroup k drs
          let payload := if k == .player then .player false else match zn, ty with | z, t => .object t z none none none none
          let ref : Referent := { det := .the, plur := pl, payload }
          let rid := drs.nextId
          let drs' := drs.push ref role inArgSlot
          (drs', [rid], if ok then [] else [.theRestFits k])
        | .possessorOf axis subj =>
          let (drs1, ids, e1) := ops.np drs .other false subj
          let kind := ((ids.head?.bind drs1.findById?).map (·.kind)).getD (subj.kindIn drs1 .object)
          let e1 := e1 ++ refuse (Kind.lte kind .object) (.possessorKind axis kind)
          let ref : Referent := { det := .the, plur := .one, payload := .player false }
          let rid := drs1.nextId
          let drs2 := drs1.push ref role inArgSlot
          (drs2, [rid], e1)
        | .combatPlayer _ =>
          let ref : Referent := { det := .the, plur := .one, payload := .player false }
          let rid := drs.nextId
          let drs' := drs.push ref role inArgSlot
          (drs', [rid], [])
        | .paid facet subj =>
          let (drs1, _, e1) := ops.np drs .other false subj
          let ref := facet.binding
          let rid := drs1.nextId
          let drs2 := drs1.push ref role inArgSlot
          (drs2, [rid], e1)
        | .asType t subj _ =>
          let (drs1, ids, e1) := ops.np drs role inArgSlot subj
          let errors := kindCheck .object ((ids.head?.bind drs1.findById?).map (·.kind))
          let refined := ids.foldl (fun state id => state.updateById id fun ref =>
            match ref.payload with
            | .object types zone stamp origin size possessor =>
              { ref with
                payload := .object (mergeTypes types [t])
                  (zone.orElse (fun _ => if t.permanent then some .battlefield else none))
                  stamp origin size possessor }
            | _ => ref) drs1
          (refined, ids, e1 ++ errors)
        | .agree _ _ subj =>
          let (drs1, ids, errs) := ops.np drs role inArgSlot subj
          (drs1, ids, errs ++ refuse (subj.plur == .many) .plural ++
            refuse (subj.det == some .count || subj.det == some .target) .countedMention)
        | .asMarker marker subj =>
          let (out, ids, errs) := ops.np drs role inArgSlot subj
          let refined := ids.foldl (fun d id => d.updateById id fun r =>
            let payload := match r.payload with
              | .object ty _ stamp origin size possessor =>
                match marker with
                | .ability => .ability origin (some .stack)
                | .spell => .object ty (some .stack) stamp origin size possessor
                | .token => .object ty (some .battlefield) stamp (some .token) size possessor
                | .permanent => .object ty (some .battlefield) stamp origin size possessor
                | .emblem => .object ty (some .command) stamp origin size possessor
              | payload => payload
            { r with payload }) out
          (refined, ids, errs)
        | .resolvedPermanent subj =>
          let (out, ids, errors) := ops.np drs role inArgSlot subj
          let source := ids.head?.bind out.findById?
          let errors := errors ++ kindCheck .object (source.map (·.kind)) ++
            refuse (source.any (fun r => permanentSpellType r.ty)) .permanentSpellType
          let out := ids.foldl (fun d id => d.updateById id fun r =>
            { r with payload := match r.payload with
              | .object ty _ stamp origin size possessor =>
                .object ty (some .battlefield) stamp origin size possessor
              | payload => payload }) out
          (out, ids, errors)
        | .becomes subj | .counterpart _ subj =>
          let (drs1, srcIds, e1) := ops.np drs .other false subj
          let e1 := e1 ++ (match np with
            | .becomes _ => kindCheck .object ((srcIds.head?.bind drs1.findById?).map (·.kind))
            | _ => [])
          let ref : Referent := {
            det := .the, plur := .one,
            payload := .object [] (some .battlefield) none none none none
          }
          let rid := drs1.nextId
          let drs2 := drs1.push ref role inArgSlot
          (drs2, [rid], e1)
        | .attachHost attachment word =>
          let payload := if word.kind == .player then Payload.player false
            else Payload.object word.attachHostTy word.attachHostZone none none none none
          let ref : Referent := { det := .the, plur := .one, payload, attachmentHost := true }
          let rid := drs.nextId
          let drs' := drs.push ref role inArgSlot
          (drs', [rid], refuse (attachment == .enchanted || Kind.lte word.kind .object)
            (.kindLte word.kind .object))
        | .theGrantor _ =>
          let ref : Referent := {
            det := .the, plur := .one,
            payload := .object [] (some .battlefield) none none none none
          }
          let rid := drs.nextId
          let drs' := drs.push ref role inArgSlot
          (drs', [rid], [])
        | .designated d whose =>
          let (drs1, _, e1) := ops.np drs .other false whose
          let ref := kindBinding (d.holder.getD .object)
          let rid := drs1.nextId
          let drs2 := drs1.push ref role inArgSlot
          (drs2, [rid], e1)
        | .you | .playerGroup _ =>
          let ref : Referent := { det := .the, plur := np.plur, payload := .player false }
          let rid := drs.nextId
          let drs' := drs.push ref role inArgSlot
          (drs', [rid], [])
        | .actor =>
          match actorShape? drs with
          | some shape =>
            if shape.refIds.isEmpty then
              let ref := kindBinding shape.kind
              let rid := drs.nextId
              let drs' := drs.push ref role inArgSlot
              (drs', [rid], [])
            else
              let drs' := if inArgSlot then shape.refIds.foldl (fun d rid => d.recordMention rid role true) drs else drs
              (drs', shape.refIds, [])
          | none =>
            let ref : Referent := { det := .the, plur := .one, payload := .player false }
            let rid := drs.nextId
            let drs' := drs.push ref role inArgSlot
            (drs', [rid], [])
        | .this =>
          let ref : Referent := { det := .the, plur := .one, payload := .object [] (some .battlefield) none none none none }
          let rid := drs.nextId
          let drs' := drs.push ref role inArgSlot
          (drs', [rid], [])
        | .any => (drs, [], []),

      amt := fun drs a =>
        match a with
        | .inCaller body =>
          let (callerDrs, errs) := ops.amt (enterCaller drs) body
          (leaveCaller drs callerDrs, errs)
        | .lit _ | .anyAmount => (drs, [])
        | .statOf axis subj =>
          let (out, ids, errs) := ops.np { drs with characteristicRead := true } .other false subj
          let refs := ids.filterMap out.findById?
          let actual := (refs.head?).map (·.kind)
          let plural := subj.plur != .one || refs.any (fun r => r.plur != .one)
          let axisErrors := match axis with
            | .counter kind => refuse kind.known .knownCounter
            | _ => []
          ({ out with characteristicRead := drs.characteristicRead },
            axisErrors ++ errs ++ kindCheck axis.scope actual ++ refuse (!plural) .singular)
        | .countOf subj | .aggregate _ _ subj
        | .paid _ subj | .greatestStoredMatch subj | .distinctCount _ subj =>
          let (drs', _, errs) := ops.np drs .other false subj
          (drs', errs)
        | .tally clause mode =>
          let (_, e1) := stepDidClauseWith ops drs clause
          let e2 := if tallyOk mode clause then [] else [.tallyOk]
          (drs, e1 ++ e2)
        | .thatMuch =>
          if countAmountOutcomes drs == 1 then (drs, []) else (drs, [.quantOutcomeInScope (countAmountOutcomes drs)])
        | .chosenNumber ref =>
          let n := countChoice (.quality .number) drs
          (drs, if ref.ok n then [] else [.choiceRef ref (.quality .number) n])
        | .votesFor _ =>
          let n := countOutcomes .voteHeld drs
          (drs, if n == 1 then [] else [.outcomeInScope .voteHeld n])
        | .theOutcome s =>
          if outcomeInScope s drs then (drs, []) else (drs, [.outcomeInScope s (countOutcomes s drs)])
        | .coinsShowing _ => (drs, if coinFlipInScope drs then [] else [.coinFlipInScope])
        | .groupSize =>
          let n := countManysAny drs
          (drs, if n == 1 then [] else [.groupSizeInScope n])
        | .theDifference =>
          let n := countGaps drs
          (drs, if n == 1 then [] else [.gapInScope n])
        | .letter l =>
          let drs' := (introducedLetters l drs).foldl (fun d r => d.push r .other false) drs
          (drs', [])
        | .arith _ l r =>
          let (drs1, e1) := ops.amt drs l
          let (drs2, e2) := ops.amt drs1 r
          (drs2, e1 ++ e2)
        | .devotion pl c optSec =>
          let (drs1, _, e1) := ops.np drs .other false pl
          let (drs2, e2) := stepColorTermWith ops drs1 c
          let (drs3, e3) := match optSec with | some d => stepColorTermWith ops drs2 d | none => (drs2, [])
          (drs3, e1 ++ e2 ++ e3)
        | .half _ inner | .upTo inner => ops.amt drs inner
        | .aggregateOver _ dom body =>
          let (drs1, e1) := ops.pred drs dom
          let k := dom.kindOr .object
          let drs2 := drs1.push (bindFor drs1 .the .one k dom) .other false
          let (drs3, e2) := ops.amt drs2 body
          (drs3, e1 ++ e2)
        | .aggregateOf _ as =>
          as.foldl (fun (d, es) x =>
            let (d', e') := ops.amt d x
            (d', es ++ e')) (drs, []),

      cond := fun drs c =>
        match c with
        | .inCaller body =>
          let (callerDrs, errs) := ops.cond (enterCaller drs) body
          (leaveCaller drs callerDrs, errs)
        | .duringPart _ (some who) =>
          let (drs', _, errs) := ops.np drs .other false who
          (drs', errs)
        | .duringPart _ none => (drs, [])
        | .exists_ subj =>
          let (drs', _, errs) := ops.np drs .subj true subj
          (drs', errs)
        | .did clause =>
          let (out, errs) := stepDidClauseWith ops
            { drs with relativeBinder := none, relativeUses := 0 } clause
          ({ out with relativeBinder := drs.relativeBinder, relativeUses := drs.relativeUses }, errs)
        | .matches subj pred =>
          let (drs1, ids, e1) := ops.np drs .subj true subj
          let (drs2, e2) := ops.pred { drs1 with relativeKind := pred.kind? } pred
          let refined := ids.foldl (fun d id => d.updateById id (markTy pred.seedTy)) drs2
          let kindErrors := match pred.kind?, ids.head?.bind drs1.findById? with
            | some expected, some r => kindCheck expected (some r.kind)
            | _, _ => []
          (refined, e1 ++ e2 ++ kindErrors ++ refuse pred.says .predSays)
        | .compareAmt l _ r =>
          let (drs1, e1) := ops.amt drs l
          let (drs2, e2) := ops.amt drs1 r
          (drs2.push gapB .other false, e1 ++ e2 ++ refuse l.read .readAmount)
        | .dealtThisWay pred =>
          let (drs1, e1) := ops.pred drs pred
          (drs1, e1 ++ (if damageDealtInScope drs then [] else [.damageDealtInScope]))
        | .choseThisWay who pred =>
          let (drs1, _, e1) := ops.np drs .subj true who
          let (drs2, e2) := ops.pred drs1 pred
          (drs2, e1 ++ e2)
        | .preventedFromSource pred =>
          let (drs1, e1) := ops.pred drs pred
          let n := countOutcomes .damagePrevented drs
          (drs1, e1 ++ (if n == 1 then [] else [.outcomeInScope .damagePrevented n]))
        | .flipCalled caller _ =>
          let (drs1, _, e1) := ops.np drs .subj true caller
          (drs1, e1 ++ (if coinFlipInScope drs then [] else [.coinFlipInScope]))
        | .flipFace _ => (drs, if coinFlipInScope drs then [] else [.coinFlipInScope])
        | .voteLead _ _ =>
          let n := countOutcomes .voteHeld drs
          (drs, if n == 1 then [] else [.outcomeInScope .voteHeld n])
        | .anyResultIs _ bound =>
          let (drs1, e1) := ops.amt drs bound
          let n := countOutcomes .rollResult drs
          (drs1, e1 ++ (if n == 1 then [] else [.outcomeInScope .rollResult n]))
        | .rolledDoubles =>
          let n := countOutcomes .rollResult drs
          (drs, if n == 1 then [] else [.outcomeInScope .rollResult n])
        | .gameIs designation => (drs, refuse designation.gameWide (.designationScope designation))
        | .noHolder designation =>
          (drs, refuse (designation.scope == some (.heldBy .player)) (.designationScope designation))
        | .hasPriority p =>
          let (drs', _, errs) := ops.np drs .subj true p
          (drs', errs)
        | .not inner =>
          match inner with
          | .matches subj predicate =>
            if subj.selfDefinedOk then
              let (subjectDrs, ids, e1) := ops.np drs .subj true subj
              let (_, e2) := ops.pred { subjectDrs with relativeKind := predicate.kind? } predicate
              let kindErrors := match predicate.kind?, ids.head?.bind subjectDrs.findById? with
                | some expected, some ref => kindCheck expected (some ref.kind)
                | _, _ => []
              (subjectDrs, e1 ++ e2 ++ kindErrors)
            else
              let (_, errs) := ops.cond drs inner
              (drs, errs)
          | _ =>
            let (_, errs) := ops.cond drs inner
            (drs, errs)
        | .and cs | .or cs =>
          let shapeErrors := refuse (cs.length ≥ 2) .atLeastTwo ++
            (match c with
              | .and _ => refuse (cs.all fun part => !part.isAnd) .flatConjuncts
              | _ => refuse (cs.all fun part => !part.isOr) .flatDisjuncts)
          cs.foldl (fun (d, es) x =>
            let (d', e') := ops.cond d x
            (d', es ++ e')) (drs, shapeErrors),

      ev := fun drs ev =>
        match ev with
        | .inCaller body =>
          let (callerDrs, errs) := ops.ev (enterCaller drs) body
          (leaveCaller drs callerDrs, errs)
        | .does subj action =>
          let headErr := if action.eventHeadOk then [] else [.eventHead]
          let (drs1, e1) := stepSubjectBeforeWith ops drs subj
          let (drs2, e2) := ops.inst .postEvent subj.patientRole { drs1 with actionPattern := true } action
          let (drs3, e3) := stepSubjectAfterWith ops drs2 subj
          ({ drs3 with actionPattern := drs.actionPattern }, headErr ++ e1 ++ e2 ++ e3)
        | .beginningOf _ whose turn | .endOf _ whose turn =>
          let (drs1, errors) := match whose with
            | none => (drs, [])
            | some owner =>
              let (out, _, errors) := ops.np drs .subj true owner
              (out, errors ++ turnOwnerKindRefusals owner)
          let (drs2, turnErrors) := stepTurnRefWith ops drs1 turn
          (drs2, errors ++ turnErrors)
        | .chapterMark ns => (drs, refuse (chapterMarksOk ns) .chapterMarks)
        | .gameBecomes _ | .stateBasedActionsChecked | .now => (drs, [])
        | .statBecomes subj _ val =>
          let (drs1, _, e1) := ops.np drs .subj true subj
          let (drs2, e2) := ops.amt drs1 val
          (drs2, e1 ++ e2)
        | .stateHolds c => ops.cond drs c
        | .triggers ab optOf optBy =>
          let (drs1, _, e1) := ops.np drs .subj true ab
          let (drs2, e2) := match optOf with
            | some o => let (d', _, e) := ops.np drs1 .other false o; (d', e)
            | none => (drs1, [])
          let (drs3, e3) := match optBy with
            | some b => let (d', _, e) := ops.np drs2 .other false b; (d', e)
            | none => (drs2, [])
          (drs3, e1 ++ e2 ++ e3)
        | .nthOccurrence _ _ ev => ops.ev drs ev
        | .causes c ev =>
          let (drs1, e1) := stepCausingWith ops drs c
          let (drs2, e2) := ops.ev drs1 ev
          (drs2, e1 ++ e2),

      inst := fun phase patientRole drs inst =>
        match inst with
        | .inCaller body =>
          let callerDrs := enterCaller drs
          let (callerDrs', errs) := ops.inst phase patientRole callerDrs body
          (leaveCaller drs callerDrs', errs)
        | .ownScope deed body =>
          let innerDrs := enterOwnScope drs
          let (innerDrs', errs) := ops.inst phase patientRole innerDrs body
          let outDrs := leaveOwnScope innerDrs'
          let _ := deed
          (outDrs, errs)
        | .bracketed body => ops.inst phase patientRole drs body
        | .enact verb body becomes locus =>
          let actor := actorShape? drs
          let defaultsToSource := enactActorKind verb == .object && actor.all (·.isYou)
          let (inner, sourceErrors) := if defaultsToSource then
            let (out, ids, errors) := ops.np drs .subj true (.asMarker .permanent .this)
            let shape : ActorShape := {
              kind := .object, isYou := false,
              selfDefined := true, ascribable := true, twoParties := false,
              opponentOnly := false, refIds := ids }
            (out.push { det := .the, plur := .one, payload := .actor shape } .other false, errors)
            else (drs, [])
          let actorErrors := match actorShape? inner with
            | some shape => match deedAgentDomain verb with
              | some domain => if domain.admits shape.kind then [] else
                kindCheck (enactActorKind verb) (some shape.kind)
              | none => []
            | none => []
          let possessorErrors := refuse (enactPatientPossessorOk inner verb (some .actor) body)
            .patientPossessor
          let priorMentions := inner.pendingRoles.length
          let (drs1, e1) := ops.inst phase patientRole inner body
          let bodyMentions := if drs1.currentClause == inner.currentClause then
              drs1.pendingRoles.take (drs1.pendingRoles.length - priorMentions)
            else drs1.pendingRoles
          let actedIds := bodyMentions.filterMap fun (id, role) =>
            if role == patientRole then some id else none
          let drs1 := if phase == .postEvent then applyInPlaceStamp drs1 actedIds (some verb) else drs1
          let (drs2, e2) := match becomes with | some p => ops.pred drs1 p | none => (drs1, [])
          let (drs3, e3) := match locus with | some z => stepZoneExprWith ops drs2 z | none => (drs2, [])
          let out := if defaultsToSource then dropActorFrame drs3 else drs3
          let becomesErrors := refuse (becomes.isNone || (deedFacts verb).any (·.intransitive)) .verbBecomesOk
          (out, actorErrors ++ sourceErrors ++ possessorErrors ++ e1 ++ e2 ++ e3 ++ becomesErrors)
        | .instruct subj body =>
          let (drs1, refIds, e1) := ops.np drs .subj true subj
          let shape : ActorShape := {
            kind := (refIds.filterMap drs1.findById?).foldl
              (fun kind ref => match kind with
                | none => some ref.kind
                | some k => some (joinKinds k ref.kind)) none |>.getD (subj.kindIn drs1 .player),
            isYou := subj.isYou,
            selfDefined := false, ascribable := false, twoParties := false, opponentOnly := subj.opponentOnly,
            refIds := refIds
          }
          let handoffErrors := refuse (shape.kind == .player ||
            (shape.kind == .object && refIds.all (fun id =>
              (drs1.findById? id).any (fun r => r.zone == some .battlefield)))) .handoffPerformer
          let memberWise := subj.det == some .each || (subj.plur == .many && body.performed)
          let memberDrs := if memberWise then
            { drs1 with referents := drs1.referents.map fun r =>
                if refIds.contains r.id then { r with plur := .one } else r }
            else drs1
          let drs2 := memberDrs.push { det := .the, plur := .one, payload := .actor shape } .other false
          let (drs3, e2) := ops.inst phase .obj drs2 body
          let out := dropActorFrame drs3
          let out := if memberWise then
            { out with referents := out.referents.map fun r =>
                if refIds.contains r.id then { r with plur := .many } else r }
            else out
          let distributes := subj.det == some .each || subj.plur == .many
          let out := if distributes then
            { out with referents := out.referents.map fun r =>
                if !(drs.referents.any (·.id == r.id)) && r.kind != .outcome
                  then pluralizeBinding r else r }
            else out
          (out, e1 ++ handoffErrors ++ e2)
        | .dealDamage amt recipient _ =>
          let (drs1, e1) := ops.amt drs amt
          let (drs2, _, e2) := ops.np drs1 patientRole true recipient
          let drs3 := drs2.push (outcomeB .damageDealt) .other false
          (drs3, e1 ++ e2 ++ refuse recipient.perMemberOk .perMember)
        | .markDamage subj amt | .skipUntap subj amt =>
          let (drs1, _, e1) := ops.np drs patientRole true subj
          let (drs2, e2) := ops.amt drs1 amt
          (drs2, e1 ++ e2)
        | .skipPart _ cnt | .addTurnPart _ _ cnt _ => ops.amt drs cnt
        | .turnFaceDown subj _ =>
          let (out, ids, errs) := ops.np drs patientRole true subj
          (out, errs ++ kindCheck .object ((ids.head?.bind out.findById?).map (·.kind)))
        | .clearDamage subj | .turnOver subj
        | .target subj | .changeControl subj | .chooseNewTargets subj =>
          let (drs1, refIds, e1) := ops.np drs patientRole true subj
          let drs2 := if phase == .postEvent then applyInPlaceStamp drs1 refIds inst.inferredDeed else drs1
          (drs2, e1)
        | .trigger => (drs, triggerActorRefusals drs)
        | .crew host | .saddle host | .fullyUnlock host =>
          let expected := match inst with | .fullyUnlock _ => Kind.player | _ => Kind.object
          let actorErrors := instructionActorRefusals drs expected
          let (out, ids, errors) := ops.np drs patientRole true host
          let typeErrors := kindCheck .object ((ids.head?.bind out.findById?).map (·.kind))
          let out := if phase == .postEvent then applyInPlaceStamp out ids inst.inferredDeed else out
          (out, actorErrors ++ errors ++ typeErrors)
        | .tapForMana perm optType =>
          let (drs1, _, e1) := ops.np drs patientRole true perm
          let (drs2, e2) := match optType with
            | some (.ofColor ct) => stepColorTermWith ops drs1 ct
            | _ => (drs1, [])
          (drs2.push (outcomeB .manaProduced) .other false, e1 ++ e2)
        | .gainDesignation subj _ optDur =>
          let (drs1, refIds, e1) := ops.np drs patientRole true subj
          let (drs2, e2) := match optDur with | some d => stepDurationWith ops drs1 d | none => (drs1, [])
          let drs3 := if phase == .postEvent then applyInPlaceStamp drs2 refIds inst.inferredDeed else drs2
          (drs3, e1 ++ e2)
        | .doAndForbid body _ subj =>
          let (drs1, e1) := ops.inst phase patientRole drs body
          let (drs2, _, e2) := ops.np drs1 .other true subj
          (drs2, e1 ++ e2)
        | .setStatus _ subj =>
          let (drs1, refIds, e1) := ops.np drs patientRole true subj
          let drs2 := if phase == .postEvent then applyInPlaceStamp drs1 refIds inst.inferredDeed else drs1
          (drs2, e1)
        | .combat subj upd =>
          let (drs1, _, e1) := ops.np drs patientRole true subj
          let (drs2, e2) := stepCombatUpdateWith ops drs1 upd
          (drs2, e1 ++ e2)
        | .attack (some defn) | .attackUnblocked (some defn) =>
          let (drs1, _, e1) := ops.np drs patientRole true defn
          (drs1, e1)
        | .attack none | .attackUnblocked none => (drs, [])
        | .attackWith atks optDef =>
          let (drs1, _, e1) := ops.np drs patientRole true atks
          let (drs2, e2) := match optDef with
            | some d => let (d', _, e) := ops.np drs1 .other true d; (d', e)
            | none => (drs1, [])
          (drs2, e1 ++ e2)
        | .block atk =>
          let (drs1, _, e1) := ops.np drs patientRole true atk
          (drs1, e1)
        | .attachment _ subj optHost =>
          let (drs1, _, e1) := ops.np drs patientRole true subj
          let (drs2, e2) := match optHost with
            | some h => let (d', _, e) := ops.np drs1 .other true h; (d', e)
            | none => (drs1, [])
          (drs2, e1 ++ e2)
        | .unlock d => stepDoorWith ops drs d
        | .setGameDesignation _ | .commitCrime | .conclude _ | .drawGame | .restartGame | .revealChoices _ => (drs, [])
        | .separateIntoPiles grp piles faces =>
          let (drs1, grpIds, e1) := ops.np drs patientRole true grp
          let grpZone := match grpIds.head?.bind drs1.findById? with | some r => r.zone | none => none
          let drs2 := partsClosed drs1
          let pileRef : Referent := {
            det := .the, plur := .many,
            payload := .pile grpZone (some piles) (pileMentionFace faces)
          }
          let drs3 := if phase == .postEvent then drs2.push pileRef .other false else drs1
          let e2 := if facesFit faces piles then [] else [.facesFit]
          (drs3, e1 ++ e2)
        | .relate subj _ cp | .merge subj cp _ | .copyTargets subj cp =>
          let (drs1, _, e1) := ops.np drs patientRole true subj
          let (drs2, _, e2) := ops.np drs1 .other true cp
          (drs2, e1 ++ e2)
        | .choose first chosen _ optWhen =>
          let (drs1, e1) := match first with
            | some f => let (d, _, e) := ops.np drs .subj true f; (d, e)
            | none => (drs, [])
          let (drs2, ids, e2) := ops.np drs1 patientRole true chosen
          let announces := chosen.det == some .a || chosen.det == some .count
          let drs2 := if announces then
            ids.foldl (fun context id => context.updateById id fun ref =>
              match ref.payload with
              | .player _ => { ref with payload := .player true }
              | .object .. => { ref with det := .part }
              | _ => ref) drs2
            else drs2
          let (drs3, e3) := match optWhen with | some t => stepTimingWith ops drs2 t | none => (drs2, [])
          (drs3, e1 ++ e2 ++ refuse chosen.agentChoosable .choiceClause ++ e3)
        | .vote first _ ballot =>
          let (drs1, e1) := match first with
            | some f => let (d, _, e) := ops.np drs .subj true f; (d, e)
            | none => (drs, [])
          let (drs2, e2) := stepBallotWith ops drs1 ballot
          let drs3 := drs2.push (outcomeB .voteHeld) .other false
          (drs3, e1 ++ e2)
        | .move subj from_ to _ optCause riders =>
          let (drs1, refIds, e1) := ops.np drs patientRole true subj
          let e1 := e1 ++ kindCheck .object ((refIds.head?.bind drs1.findById?).map (·.kind))
          let (drs2, e2) := stepEventSourceWith ops drs1 from_
          let (drs3, e3) := match to with | some z => stepZoneExprWith ops drs2 z | none => (drs2, [])
          let (drs4, e4) := match optCause with | some c => stepCausingWith ops drs3 c | none => (drs3, [])
          let (drs5, e5) := stepMoveRiderListWith ops drs4 riders
          let destZone := to.bind (·.sort)
          let drs6 := if phase == .postEvent then
            applyZoneMove drs5 refIds inst.inferredDeed destZone
          else drs5
          let allowed := to.bind (fun z => match z with
            | .zone .battlefield _ => some RiderKind.entry
            | .zone _ _ | .place _ _ => some RiderKind.placement
            | _ => none)
          let riderErrors := riders.flatMap fun r =>
            let found := match r with | .entry _ => RiderKind.entry | .placement _ => .placement
            allowed.toList.flatMap (fun k => refuse (k == found) (.riderKind k found))
          (drs6, e1 ++ e2 ++ e3 ++ e4 ++ e5 ++ riderErrors)
        | .copy sort subj times exs =>
          let (drs1, sourceIds, e1) := ops.np drs patientRole true subj
          let (drs2, e2) := ops.amt drs1 times
          let (drs3, e3) := stepCopyExceptListWith ops drs2 exs
          let drs4 := if phase == .postEvent then
            let source := sourceIds.head?.bind drs1.findById?
            let sourceZone := source.bind (·.zone)
            let isAbility := source.any (fun r => match r.payload with | .ability .. => true | _ => false)
            let copies := match times with | .lit 1 => .one | _ => .many
            let copyRef : Referent := {
              det := .the, plur := copies,
              payload := copyPayloadIn (source.map (·.kind) |>.getD .object) isAbility
                (source.map (·.ty) |>.getD []) (sort.landsIn sourceZone)
            }
            drs3.push copyRef patientRole true
          else drs3
          (drs4, e1 ++ e2 ++ e3)
        | .changeLife delta =>
          let (drs1, e1) := stepDeltaWith ops drs delta
          let drs2 := match delta.outcome with
            | some out => drs1.push (outcomeB out) .other false
            | none => drs1
          (drs2, e1)
        | .exchange ex => stepExchangedWith ops drs ex
        | .addMana amt prod riders =>
          let (drs1, e1) := ops.amt drs amt
          let (drs2, e2) := stepProducedManaWith ops drs1 prod
          let withOutcome := drs2.push (outcomeB .manaAdded) .other false
          let (drs3, e3) := stepManaRiderListWith ops withOutcome riders
          (drs3, e1 ++ e2 ++ e3)
        | .draw amt =>
          let actorErrors := match actorShape? drs with
            | some shape => kindCheck .player (some shape.kind)
            | none => []
          let (drs1, e1) := ops.amt drs amt
          let drs2 := if phase == .postEvent then
            let cardRef : Referent := {
              det := .the, plur := .one,
              payload := .object [] (some .hand) (mkStamp (some (.core .draw)) (some .hand) false) none none none
            }
            drs1.push cardRef patientRole true
          else drs1
          (drs2, actorErrors ++ e1)
        | .expose _ exposed =>
          match exposed with
          | .cards np =>
            let (drs1, refIds, e1) := ops.np drs patientRole true np
            let drs2 := if phase == .postEvent then applyInPlaceStamp drs1 refIds inst.inferredDeed else drs1
            (drs2, e1)
          | .zone z => stepZoneExprWith ops drs z
          | .choice _ => (drs, [])
        | .search sc q p =>
          let (drs1, e1) := stepSearchScopeWith ops drs sc
          let (drs2, e2) := stepQuantityWith ops drs1 q
          let (drs3, e3) := ops.pred drs2 p
          let scZone := match sc with | .oneZone z => z.sort | .someZones _ _ => none
          let drs4 := if phase == .postEvent then
            let foundRef : Referent := {
              det := .a, plur := q.plur,
              payload := .object p.seedTy scZone (mkStamp (some (.action "search")) none false) none none none
            }
            drs3.push foundRef patientRole true
          else drs3
          (drs4, e1 ++ e2 ++ e3)
        | .shuffle pile =>
          let (drs1, e1) := stepPileWith ops drs pile
          let drs2 := if phase == .postEvent then afterShuffle drs1 else drs1
          (drs2, e1)
        | .flipCoins count _ _ =>
          let (drs1, e1) := stepFlipScopeWith ops drs count
          let drs2 := drs1.push (outcomeB .coinFlipped) .other false
          (drs2, e1)
        | .rollDice count sides batch watch =>
          let (drs1, e1) := ops.amt drs count
          let (drs2, e2) := match watch with
            | .anyResult | .highestNatural => (drs1, [])
            | .resultIn q => stepQuantityWith ops drs1 q
          let drs3 := drs2.push (outcomeB .rollResult) .other false
          let drs4 := if batch == .many then drs3.push (outcomeB .diceRolled) .other false else drs3
          (drs4, e1 ++ e2 ++ sides.localChecks drs ++ watch.localChecks ++
            refuse count.nonZero .nonZeroQ)
        | .applyResultsTable rows =>
          let cnt := countOutcomes .rollResult drs
          let e0 := if cnt == 1 then [] else [.outcomeInScope .rollResult cnt]
          let (drs1, e1) := stepRollRowListWith ops drs rows
          (drs1, e0 ++ e1)
        | .ignoreOutcomes which =>
          let (drs1, e1) := stepIgnoredOutcomesWith ops drs which
          let e2 := if ignorableInScope drs1 then [] else [.ignorableFor]
          (drs1, e1 ++ e2)
        | .shiftResult _ amt =>
          let (drs1, e1) := ops.amt drs amt
          let cnt := countOutcomes .rollResult drs1
          let e2 := if cnt == 1 then [] else [.outcomeInScope .rollResult cnt]
          (drs1, e1 ++ e2)
        | .storeResults on =>
          let (drs1, _, e1) := ops.np drs patientRole true on
          let cnt := countOutcomes .rollResult drs1
          let e2 := if cnt == 1 then [] else [.outcomeInScope .rollResult cnt]
          (drs1, e1 ++ e2)
        | .rerollStored q whose =>
          let (drs1, e1) := stepQuantityWith ops drs q
          let (drs2, _, e2) := ops.np drs1 .other false whose
          (drs2, e1 ++ e2)
        | .establish spec dur =>
          let (drs1, e1) := ops.spec drs spec
          let (drs2, e2) := match dur with | some d => stepDurationWith ops drs1 d | none => (drs1, [])
          (drs2, e1 ++ e2)
        | .createObject cnt spec _ optUnder =>
          let (drs1, e1) := ops.amt drs cnt
          let (drs2, sourceTypes, e2) := stepCreationSpecMetadataWith ops drs1 spec
          let (drs3, e3) := match optUnder with
            | some u => let (d', _, e) := ops.np drs2 .other false u; (d', e)
            | none => (drs2, [])
          let drs4 := if phase == .postEvent then
            let performerPlur := ((actorShape? drs).bind fun actor =>
              (actor.refIds.head?.bind drs.findById?).map (·.plur)).getD .one
            drs3.push (spec.createdReferent cnt.plur performerPlur sourceTypes) patientRole true
          else drs3
          (drs4, e1 ++ e2 ++ e3)
        | .putCounters amt ks on _ _ =>
          let (drs1, e1) := ops.amt drs amt
          let (drs2, e2) := stepCounterKindSourceWith ops drs1 ks
          let (drs3, _, e3) := ops.np drs2 patientRole true on
          let drs4 := drs3.push (outcomeB .countersPut) .other false
          (drs4, e1 ++ e2 ++ e3)
        | .removeCounters optQ optKs from_ _ _ =>
          let (drs1, e1) := match optQ with | some q => stepQuantityWith ops drs q | none => (drs, [])
          let (drs2, e2) := match optKs with | some ks => stepCounterKindSourceWith ops drs1 ks | none => (drs1, [])
          let (drs3, _, e3) := ops.np drs2 patientRole true from_
          let drs4 := drs3.push (outcomeB .countersRemoved) .other false
          (drs4, e1 ++ e2 ++ e3)
        | .distribute v amt among =>
          let (drs1, e1) := stepDividedVerbWith ops drs v
          let (drs2, e2) := ops.amt drs1 amt
          let (drs3, _, e3) := ops.np drs2 patientRole true among
          (drs3, e1 ++ e2 ++ e3)
        | .moveCounters amt optKs src dst =>
          let (drs1, e1) := ops.amt drs amt
          let (drs2, e2) := match optKs with | some ks => stepCounterKindSourceWith ops drs1 ks | none => (drs1, [])
          let (drs3, _, e3) := ops.np drs2 patientRole true src
          let (drs4, _, e4) := ops.np drs3 .other true dst
          (drs4, e1 ++ e2 ++ e3 ++ e4)
        | .doubleCounters on =>
          let (drs1, _, e1) := ops.np drs patientRole true on
          (drs1, e1)
        | .pay cost _ _ whose _ =>
          let (drs1, e1) := match whose with
            | some w => let (d, _, e) := ops.np drs .subj true w; (d, e)
            | none => (drs, [])
          let (drs2, e2) := ops.cost drs1 cost
          (drs2, e1 ++ e2)
        | .withContinuation _ body ifDid ifNot =>
          let (drs1, e1) := ops.inst phase patientRole drs body
          let (drs2, e2) := match ifDid with
            | some b => ops.inst phase .obj (nextClause drs1) b
            | none => (drs1, [])
          let (drs3, e3) := match ifNot with
            | some b => ops.inst phase .obj (nextClause drs) b
            | none => (drs, [])
          ({ drs with nextId := max drs2.nextId drs3.nextId }, e1 ++ e2 ++ e3)
        | .doOnlyIf inst cond otherwise =>
          let (drs1, e1) := ops.inst phase patientRole drs inst
          let (drs2, e2) := ops.cond drs1 cond
          let (drs3, e3) := match otherwise with
            | some b => ops.inst phase .obj (nextClause drs) b
            | none => (drs, [])
          ({ drs with nextId := max drs2.nextId drs3.nextId }, e1 ++ e2 ++ e3)
        | .limited inst limit =>
          let (out, errors) := ops.inst phase patientRole drs inst
          let (out', limitErrors) := stepUsageLimitWith ops out limit
          (out', errors ++ limitErrors)
        | .doIf cond thenInst elseInst =>
          let (drs1, e1) := ops.cond drs cond
          let (drs2, e2) := ops.inst phase .obj (nextClause drs1) thenInst
          match elseInst with
          | none => (drs2, e1 ++ e2)
          | some el =>
            let (_, e3) := ops.inst phase .obj (nextClause drs) el
            (drs, e1 ++ e2 ++ e3)
        | .doForEach grp body =>
          let (drs1, grpIds, e1) := ops.np drs .other false grp
          let pluralityErrors := refuse (grp.plur == .many || grp.det == some .each) .plural
          let group := grpIds.head?.bind drs1.findById?
          let drs1 := grpIds.foldl (fun state id =>
            state.updateById id (fun ref => { ref with plur := .many })) drs1
          let preLoopMaxId := drs1.nextId
          let loopRef : Referent := {
            det := .each, plur := .one,
            payload := group.map (·.payload) |>.getD (.object [] none none none none none)
          }
          let drs2 := drs1.push loopRef .subj true
          let (drs3, e2) := ops.inst phase .obj (nextClause drs2) body
          let kept := drs3.referents.filter (·.id < preLoopMaxId)
          let keptIds := kept.map (·.id)
          let closedDrs : DRS := { drs3 with
            referents := kept
            pendingRoles := drs3.pendingRoles.filter (fun mention => keptIds.contains mention.1)
            resolvedIds := drs3.resolvedIds.filter keptIds.contains
            performer := drs1.performer }
          (closedDrs, e1 ++ pluralityErrors ++ e2)
        | .doForEachKind _ dom q body =>
          let (drs1, e1) := match dom with
            | some d => let (d', _, e) := ops.np drs .other false d; (d', e)
            | none => (drs, [])
          let preLoopMaxId := drs1.nextId
          let drs2 := drs1.push { det := .each, plur := .one, payload := .quality q } .other false
          let (drs3, e2) := ops.inst phase .obj (nextClause drs2) body
          let closedDrs : DRS := { drs3 with referents := drs3.referents.filter (·.id < preLoopMaxId) }
          (closedDrs, e1 ++ e2)
        | .repeat_ rep => stepRepetitionWith ops drs rep
        | .sequentially steps =>
          match steps with
          | [] => (drs, [.nonEmpty])
          | i :: rest =>
            let (drs1, e1) := ops.inst phase .obj drs i
            rest.foldl (fun (d, es) step =>
              let (d', e') := ops.inst phase .obj (nextClause d) step
              (d', es ++ e')) (drs1, e1)
        | .simultaneously steps =>
          steps.foldl (fun (d, es) step =>
            let (d', e') := ops.inst phase .obj d step
            (d', es ++ e')) (drs, refuse (!steps.isEmpty) .nonEmpty)
        | .chooseModes q modes =>
          let (drs1, e1) := stepQuantityWith ops drs q
          let (drs2, e2) := stepModeListWith ops drs1 modes
          (drs2, e1 ++ e2)
        | .delay ev alts optDur body =>
          let (drs1, e1) := ops.ev drs ev
          let (_, eAlts) := stepGameEventListWith ops drs alts
          let (drs2, e2) := match optDur with | some d => stepDurationWith ops drs1 d | none => (drs1, [])
          let (drs3, e3) := ops.inst phase .obj (nextClause drs2) body
          (drs3, e1 ++ eAlts ++ e2 ++ e3)
        | .replace repld repl =>
          let (drs1, e1) := ops.inst .preEvent patientRole drs repld
          let (drs2, e2) := ops.inst .postEvent .obj (nextClause drs1) repl
          (drs2, e1 ++ e2)
        | .holdUntil inst ev =>
          let (drs1, e1) := ops.inst phase patientRole drs inst
          let (drs2, e2) := ops.ev drs1 ev
          (drs2, e1 ++ e2)
        | .triggerReflexively body trig =>
          let (drs1, e1) := ops.inst phase patientRole drs body
          let (drs2, e2) := ops.inst .postEvent .obj (nextClause drs1) trig
          (drs2, e1 ++ e2)
        | .triggerThisWay body ev trig =>
          let (drs1, e1) := ops.inst phase patientRole drs body
          let (drs2, e2) := ops.ev (nextClause drs1) ev
          let (drs3, e3) := ops.inst .postEvent .obj (nextClause drs2) trig
          (drs3, e1 ++ e2 ++ e3)
        | .procedure _ subj pay res from_ of_ =>
          let (drs1, refIds, e1) := ops.np drs patientRole true subj
          let e1 := e1 ++ kindCheck .object ((refIds.head?.bind drs1.findById?).map (·.kind))
          let (drs2, e2) := stepPlayPaymentWith ops drs1 pay
          let (drs3, e3) := match res with | some p => ops.pred drs2 p | none => (drs2, [])
          let (drs4, e4) := match from_ with | some f => stepEventSourceWith ops drs3 f | none => (drs3, [])
          let (drs5, e5) := match of_ with
            | some o => let (d, _, e) := ops.np drs4 .other false o; (d, e)
            | none => (drs4, [])
          let drs6 := if phase == .postEvent then applyZoneMove drs5 refIds (some (.action "cast")) (some .stack) else drs5
          (drs6, e1 ++ e2 ++ e3 ++ e4 ++ e5),

      spec := fun drs s =>
        match s with
        | .inCaller body =>
          let (callerDrs, errs) := ops.spec (enterCaller drs) body
          (leaveCaller drs callerDrs, errs)
        | .modification subj _ d =>
          let (drs1, _, e1) := ops.np drs .subj true subj
          let (drs2, e2) := stepDeltaWith ops drs1 d
          (drs2, e1 ++ e2)
        | .ptModification subj p t =>
          let (drs1, _, e1) := ops.np drs .subj true subj
          let (drs2, e2) := stepDeltaWith ops drs1 p
          let (drs3, e3) := stepDeltaWith ops drs2 t
          (drs3, e1 ++ e2 ++ e3)
        | .ptDefinition subj _ amt =>
          let (drs1, _, e1) := ops.np drs .subj true subj
          let (drs2, e2) := ops.amt drs1 amt
          (drs2, e1 ++ e2 ++ refuse subj.selfDefinedOk .selfDefinedOk)
        | .manaRetention subj held =>
          let (out, _, errors) := ops.np drs .subj true subj
          (out, errors ++ held.localChecks out)
        | .ptSwitch subj | .partSkip subj _ | .combatant subj =>
          let (drs1, _, e1) := ops.np drs .subj true subj
          (drs1, e1)
        | .costShift subj sh =>
          let (drs1, _, e1) := ops.np drs .subj true subj
          let (drs2, e2) := stepCostShiftWith ops drs1 sh
          (drs2, e1 ++ e2)
        | .altCost subj optCost =>
          let (drs1, _, e1) := ops.np drs .subj true subj
          let (drs2, e2) := match optCost with | some c => ops.cost drs1 c | none => (drs1, [])
          (drs2, e1 ++ e2)
        | .addedCost c _ _ => ops.cost drs c
        | .paySubstitute subj sub =>
          let (drs1, _, e1) := ops.np drs .subj true subj
          let (drs2, e2) := stepManaSubstituteWith ops drs1 sub
          (drs2, e1 ++ e2)
        | .letterDefinition l amt =>
          let (drs1, e1) := ops.amt drs amt
          (defineLetter l drs1, e1 ++ refuse (anyOpenLetter l drs) (.openLetter l))
        | .abilityGrant subj ab =>
          let (drs1, _, e1) := ops.np drs .subj true subj
          let e2 := ops.ab (choiceContext drs1) ab
          (drs1, e1 ++ e2)
        | .abilityGrantFrom subj _ src optEx =>
          let (drs1, _, e1) := ops.np drs .subj true subj
          let (drs2, _, e2) := ops.np drs1 .other false src
          let (drs3, e3) := match optEx with | some p => ops.pred drs2 p | none => (drs2, [])
          (drs3, e1 ++ e2 ++ e3)
        | .abilityGrantSelected subj src sel =>
          let (drs1, _, e1) := ops.np drs .subj true subj
          let (drs2, _, e2) := ops.np drs1 .other false src
          let (drs3, e3) := stepAbilitySelectionWith ops drs2 sel
          (drs3, e1 ++ e2 ++ e3)
        | .characteristicChange subj edits =>
          let (drs1, _, e1) := ops.np drs .subj true subj
          let (drs2, e2) := stepCharacteristicEditListWith ops drs1 edits
          (drs2, e1 ++ e2 ++ refuse (!edits.isEmpty) .nonEmpty ++
            refuse (edits.all CharacteristicEdit.writesLine) .lineNonEmpty ++
            CharacteristicEdit.shapeAll edits)
        | .copyChange subj src exs =>
          let (drs1, _, e1) := ops.np drs .subj true subj
          let (drs2, _, e2) := ops.np drs1 .other false src
          let (drs3, e3) := stepCopyExceptListWith ops drs2 exs
          (drs3, e1 ++ e2 ++ e3)
        | .retention spec subj =>
          let (drs1, e1) := ops.spec drs spec
          let (drs2, _, e2) := ops.np drs1 .other false subj
          (drs2, e1 ++ e2)
        | .addsTarget subj desc =>
          let (drs1, _, e1) := ops.np drs .subj true subj
          let (drs2, e2) := ops.pred drs1 desc
          (drs2, e1 ++ e2)
        | .textChange subj tc =>
          let (drs1, _, e1) := ops.np drs .subj true subj
          let (drs2, e2) := stepTextChangeWith ops drs1 tc
          (drs2, e1 ++ e2)
        | .controlGrant player subj =>
          let (drs1, _, e1) := ops.np drs .subj true player
          let (drs2, _, e2) := ops.np drs1 .obj true subj
          (drs2, e1 ++ e2)
        | .entryRider subj r =>
          let (drs1, _, e1) := ops.np drs .subj true subj
          let (drs2, e2) := stepEntryRiderWith ops drs1 r
          (drs2, e1 ++ e2)
        | .choice _ subj q dom _ =>
          let (drs1, _, e1) := ops.np drs .subj true subj
          let (drs2, e2) := sortedDomainCheckWith ops drs1 q dom
          let choiceRef : Referent := {
            det := .the, plur := .one,
            payload := match q with | .player => .player true | .quality qs => .quality qs
          }
          let drs3 := drs2.push choiceRef .other false
          (drs3, e1 ++ e2)
        | .modeChoice mc => stepModeChangeWith ops drs mc
        | .cant subj action mods =>
          let headErr := if action.deonticHeadOk then [] else [.eventHead]
          let (drs1, e1) := stepSubjectBeforeWith ops drs subj
          let (drs2, e2) := ops.inst .preEvent subj.patientRole { drs1 with actionPattern := true } action
          let e2 := e2 ++ refuse (deonticPatientOk drs1 drs2 action subj.patientRole) .deonticPatientOk
          let (drs3, e3) := stepDeonticAsThoughWith ops drs2 mods
          let (_, e4) := stepSubjectAfterWith ops drs3 subj
          (drs, headErr ++ e1 ++ e2 ++ e3 ++ e4)
        | .must subj action mods | .may subj action mods =>
          let headErr := if action.deonticHeadOk then [] else [.eventHead]
          let (drs1, e1) := stepSubjectBeforeWith ops drs subj
          let (drs2, e2) := ops.inst .preEvent subj.patientRole { drs1 with actionPattern := true } action
          let e2 := e2 ++ refuse (deonticPatientOk drs1 drs2 action subj.patientRole) .deonticPatientOk
          let (drs3, e3) := stepDeonticAsThoughWith ops drs2 mods
          let (drs4, e4) := stepSubjectAfterWith ops drs3 subj
          ({ drs4 with actionPattern := drs.actionPattern }, headErr ++ e1 ++ e2 ++ e3 ++ e4)
        | .gatedBy cost subj action mods =>
          let headErr := if action.deonticHeadOk then [] else [.eventHead]
          let payerId := drs.nextId
          let payer := kindBinding .player
          let (withPayer, e1) := ops.cost (drs.push payer .other false) cost
          let drs1 := { withPayer with referents := withPayer.referents.filter (·.id != payerId) }
          let (drs2, e2) := stepSubjectBeforeWith ops (nextClause drs1) subj
          let (drs3, e3) := ops.inst .preEvent subj.patientRole { drs2 with actionPattern := true } action
          let e3 := e3 ++ refuse (deonticPatientOk drs2 drs3 action subj.patientRole) .deonticPatientOk
          let (drs4, e4) := stepDeonticAsThoughWith ops drs3 mods
          let (drs5, e5) := stepSubjectAfterWith ops drs4 subj
          ({ drs5 with actionPattern := drs.actionPattern }, headErr ++ e1 ++ e2 ++ e3 ++ e4 ++ e5)
        | .instead ev repl tm _ _ =>
          let (drs1, e1) := stepWouldClauseWith ops drs ev
          let (drs2, e2) := ops.inst .postEvent .obj (nextClause drs1) repl
          let (_, e3) := match tm with | some t => stepTimingWith ops drs2 t | none => (drs2, [])
          (drs, e1 ++ e2 ++ e3)
        | .damageResult r =>
          match r with
          | ⟨_, source, recipient, _, does⟩ =>
            let (drs1, e1) := ops.pred drs recipient
            let (drs2, e2) := stepSubjectBeforeWith ops (nextClause drs1) source
            let (drs3, e3) := ops.inst .postEvent source.patientRole drs2 does
            let (_, e4) := stepSubjectAfterWith ops drs3 source
            (drs, e1 ++ e2 ++ e3 ++ e4)
        | .damageRule _ da ds dop _ =>
          let (drs1, e1) := stepDamageAgentWith ops drs da
          let (drs2, e2) := stepDamageScopeWith ops drs1 ds
          let (drs3, e3) := stepDamageOpWith ops drs2 dop
          (drs3, e1 ++ e2 ++ e3)
        | .preventionBan _ (.described da ds) _ =>
          let (drs1, e1) := stepDamageAgentWith ops drs da
          let (drs2, e2) := stepDamageScopeWith ops drs1 ds
          (drs2, e1 ++ e2)
        | .preventionBan _ .thatDamage _ => (drs, [])
        | .additionalTriggers ev q =>
          let (drs1, e1) := ops.ev drs ev
          let (drs2, e2) := stepQuantityWith ops drs1 q
          (drs2, e1 ++ e2)
        | .conditional spec cond marking =>
          let (drs1, e1) := ops.spec drs spec
          let (drs2, e2) := ops.cond (nextClause drs1) cond
          (drs2, e1 ++ e2 ++ refuse (markingOk marking cond) .markingOk)
        | .visibility _ player what =>
          let (drs1, _, e1) := ops.np drs .subj true player
          let (drs2, e2) := stepVisibleThingWith ops drs1 what
          (drs2, e1 ++ e2)
        | .conjunction optSubj parts =>
          let (drs1, e1) := match optSubj with
            | some sub => let (d, _, e) := ops.np drs .subj true sub; (d, e)
            | none => (drs, [])
          let (drs2, e2) := parts.foldl (fun (d, es) s =>
            let (d', e') := ops.spec d s
            (d', es ++ e')) (drs1, [])
          (drs2, e1 ++ e2 ++ refuse (!parts.isEmpty) .nonEmpty)
        | .specialAction tm cost does =>
          let (drs1, e1) := match tm with | some t => stepTimingWith ops drs t | none => (drs, [])
          let (drs2, e2) := ops.cost drs1 cost
          let (drs3, e3) := ops.inst .postEvent .obj (nextClause (publicOnly drs2)) does
          (drs3, e1 ++ e2 ++ e3)
        | .asEvent ev does =>
          let (drs1, e1) := ops.ev drs ev
          let (drs2, e2) := ops.inst .postEvent .obj (nextClause drs1) does
          (drs2, e1 ++ e2)
        | .whileHolds cond does =>
          let (drs1, e1) := ops.cond drs cond
          let (drs2, e2) := ops.inst .postEvent .obj (nextClause drs1) does
          (drs2, e1 ++ e2),

      cost := fun drs c =>
        let (out, errors) :=
          match c with
          | .inCaller body =>
            let (callerDrs, errs) := ops.cost (enterCaller drs) body
            (leaveCaller drs callerDrs, errs)
          | .anyCost => (drs, refuse drs.actionPattern .concreteCost)
          | .mana cost => (drs, ManaCost.check cost)
          | .tapSymbol | .untapSymbol | .loyaltySymbol _ | .itsManaCost => (drs, [])
          | .scaled c amt =>
            let (drs1, e1) := ops.cost drs c
            let (drs2, e2) := ops.amt drs1 amt
            (drs2, e1 ++ e2)
          | .perform inst => ops.inst .postEvent .obj drs inst
          | .compound cs =>
            cs.foldl (fun (d, es) x =>
              let (d', e') := ops.cost d x
              (d', es ++ e')) (drs, refuse (!cs.isEmpty) .nonEmpty)
          | .or cs =>
            let errors := cs.flatMap fun c =>
              (ops.cost drs c).2 ++ (match c with | .compound _ => [.notCompound] | _ => [])
            (drs, refuse (cs.length ≥ 2) .atLeastTwo ++ errors)
          | .withSubstitute c sub =>
            let (drs1, e1) := ops.cost drs c
            let (drs2, e2) := stepManaSubstituteWith ops drs1 sub
            (drs2, e1 ++ e2)
        (out, errors ++ c.localChecks),

      ab := fun drs ab =>
        match ab with
        | .keyword k params body =>
          checkKeywordParamsWith ops drs ((keywordSchemaFor k params).getD []) params ++
            (stepAbilityListWith ops (enterOwnScope drs) body).2 ++
            refuse (keywordParamFits k params) (.keywordParamFits k) ++
            refuse (keywordBodyFits k body) (.keywordBodyFits k)
        | .activated cost inst tm _ guard activator =>
          let (drs0, e0) := match activator with
            | some a => let (d, _, e) := ops.np drs .subj true a; (d, e)
            | none => (drs, [])
          let (drs1, e1) := ops.cost drs0 cost
          let colonDrs := nextClause (publicOnly drs1)
          let (drs2, e2) := ops.inst .postEvent .obj colonDrs inst
          let (_, e3) := match tm with | some t => stepTimingWith ops drs2 t | none => (drs2, [])
          let (_, e4) := match guard with | some g => ops.cond drs2 g | none => (drs2, [])
          e0 ++ e1 ++ e2 ++ e3 ++ e4
        | .triggered ev alts _ _ tm _ interv inst =>
          let (drs1, e1) := ops.ev drs ev
          let (_, eAlts) := stepGameEventListWith ops drs alts
          let (drs2, e2) := match interv with
            | some c => ops.cond (nextClause drs1) c
            | none => (drs1, [])
          let (drs3, e3) := ops.inst .postEvent .obj (nextClause drs2) inst
          let (_, e4) := match tm with | some t => stepTimingWith ops drs3 t | none => (drs3, [])
          e1 ++ eAlts ++ e2 ++ e3 ++ e4
        | .static spec => (ops.spec drs spec).2
        | .spell tm inst =>
          let (drs1, e1) := match tm with | some t => stepTimingWith ops drs t | none => (drs, [])
          let (_, e2) := ops.inst .postEvent .obj drs1 inst
          e1 ++ e2
        | .mayBeginOnBattlefield | .thatAbility _ => []
        | .alsoForKeywords inner _ | .italicHead _ inner => ops.ab drs inner
        | .qualified inner tm limit guard _ =>
          let e1 := ops.ab drs inner
          let (_, e2) := match tm with | some t => stepTimingWith ops drs t | none => (drs, [])
          let (_, e3) := match guard with | some g => ops.cond drs g | none => (drs, [])
          e1 ++ e2 ++ e3 ++ refuse (inner.limitFits limit) .qualifiedLimit
    }

def stepPredicate (fuel : Nat) (drs : DRS) (p : Predicate) : DRS × List Refusal :=
  (stepOps fuel).pred drs p

def stepNounPhrase (fuel : Nat) (drs : DRS) (role : GramRole) (inArgSlot : Bool) (np : NounPhrase) :
    DRS × List ReferentId × List Refusal :=
  (stepOps fuel).np drs role inArgSlot np

def stepAmount (fuel : Nat) (drs : DRS) (a : Amount) : DRS × List Refusal :=
  (stepOps fuel).amt drs a

def stepCondition (fuel : Nat) (drs : DRS) (c : Condition) : DRS × List Refusal :=
  (stepOps fuel).cond drs c

def stepGameEvent (fuel : Nat) (drs : DRS) (ev : GameEvent) : DRS × List Refusal :=
  (stepOps fuel).ev drs ev

def stepInstruction (fuel : Nat) (phase : StepPhase) (patientRole : GramRole) (drs : DRS)
    (inst : Instruction) : DRS × List Refusal :=
  (stepOps fuel).inst phase patientRole drs inst

def stepStaticSpec (fuel : Nat) (drs : DRS) (s : StaticSpec) : DRS × List Refusal :=
  (stepOps fuel).spec drs s

def stepCost (fuel : Nat) (drs : DRS) (c : Cost) : DRS × List Refusal :=
  (stepOps fuel).cost drs c

def checkAbility (fuel : Nat) (drs : DRS) (ab : Ability) : List Refusal :=
  (stepOps fuel).ab drs ab

def stepDidClause (fuel : Nat) (drs : DRS) (c : DidClause) : DRS × List Refusal :=
  stepDidClauseWith (stepOps fuel) drs c

def stepWouldClause (fuel : Nat) (drs : DRS) (wc : WouldClause) : DRS × List Refusal :=
  stepWouldClauseWith (stepOps fuel) drs wc

def stepAbilityList (fuel : Nat) (drs : DRS) (abs : List Ability) : DRS × List Refusal :=
  stepAbilityListWith (stepOps fuel) drs abs

def stepGameEventList (fuel : Nat) (drs : DRS) (evs : List GameEvent) : DRS × List Refusal :=
  stepGameEventListWith (stepOps fuel) drs evs

def NounPhrase.check (drs : DRS) (np : NounPhrase) : List Refusal :=
  (stepNounPhrase 64 drs .other false np).2.2

def Predicate.check (drs : DRS) (p : Predicate) : List Refusal :=
  (stepPredicate 64 drs p).2

def Amount.check (drs : DRS) (a : Amount) : List Refusal :=
  (stepAmount 64 drs a).2

def Condition.check (drs : DRS) (c : Condition) : List Refusal :=
  (stepCondition 64 drs c).2

def Instruction.check (drs : DRS) (i : Instruction) : List Refusal :=
  (stepInstruction 64 .postEvent .obj drs i).2

def StaticSpec.check (drs : DRS) (s : StaticSpec) : List Refusal :=
  (stepStaticSpec 64 drs s).2

def Ability.check (drs : DRS := DRS.ofList []) (a : Ability) : List Refusal :=
  checkAbility 64 drs a

def NounPhrase.checkAt (expected : Option Kind) (drs : DRS)
    (np : NounPhrase) : List Refusal :=
  np.check drs ++ expected.toList.flatMap (fun k => kindCheck k np.kind?)

def Predicate.checkAt (expected : Kind) (drs : DRS) (p : Predicate) : List Refusal :=
  p.check { drs with relativeKind := some expected } ++ kindCheck expected p.kind?

end Semantics
