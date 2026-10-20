import Semantics.Abilities
import Semantics.Check.Events
import Semantics.Check.HeaderContext
import Semantics.Check.CardFacts
import Semantics.Check.Creation
import Semantics.Check.Choices
import Semantics.Check.LocalShape
import Semantics.Check.LoopResults
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
  | .bare => .bare

def Amount.plur (a : Amount) (fuel : Nat := 16) : Plurality :=
  match fuel, a with
  | n + 1, .inCaller body => body.plur n
  | _ + 1, .lit 1 => .one
  | _, _ => .many
termination_by structural fuel

def Quantity.plur (q : Quantity) (fuel : Nat := 16) : Plurality :=
  match fuel, q with
  | 0, _ => .many
  | n + 1, .inCaller body => body.plur n
  | _ + 1, .range _ (some 1) => .one
  | n + 1, .exactlyOf amount | n + 1, .upToOf amount => amount.plur n
  | n + 1, .additional q => q.plur n
  | _ + 1, _ => .many
termination_by structural fuel

def DetPhrase.plur : DetPhrase → Plurality
  | .target q | .count q _ => q.plur
  | .a _ | .each | .the => .one
  | .all | .bare => .many

def SliceCount.plur : SliceCount → Plurality
  | .counted q => q.plur
  | .whole | .totalling .. => .many

def Predicate.kind? (p : Predicate) (fuel : Nat := 32) : Option Kind :=
  match fuel, p with
  | 0, _ => none
  | n + 1, .inCaller body | n + 1, .bracketed body | n + 1, .not body => body.kind? n
  | _ + 1, .anyPlayer | _ + 1, .isOpponentOf _ | _ + 1, .isTeammateOf _ | _ + 1, .chosenPlayer _ | _ + 1, .choseExtreme _ => some .player
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
  | _ + 1, .otherThan _ | _ + 1, .isSource | _ + 1, .withMostVotes | _ + 1, .coinCameUp _
  | _ + 1, .attachment .host _ _ | _ + 1, .inCombat .attackedBy _ => none
  | _ + 1, _ => some .object
termination_by structural fuel

def Predicate.kindOr (d : Kind) (p : Predicate) : Kind := p.kind?.getD d

def NounPhrase.kind? (np : NounPhrase) (fuel : Nat := 32) : Option Kind :=
  match fuel, np with
  | 0, _ => some .object
  | n + 1, .inCaller body | n + 1, .firstMention _ body | n + 1, .laterMention _ body => body.kind? n
  | _ + 1, .you | _ + 1, .actor | _ + 1, .possessorOf _ _ | _ + 1, .combatPlayer _ => some .player
  | n + 1, .someOf _ _ group => group.kind? n
  | _ + 1, .pileOf _ _ => some .pile
  | _ + 1, .attachHost _ word => some word.kind
  | n + 1, .or alternatives =>
    alternatives.foldl (fun kind phrase =>
      match kind, phrase.kind? n with
      | some left, some right => some (joinKinds left right)
      | some left, none => some left
      | none, right => right) none
  | _ + 1, .designated _ _ => some .object
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

private def NounPhrase.selectionKindOverride? (phrase : NounPhrase)
    (fuel : Nat := 32) : Option Kind :=
  match fuel, phrase with
  | 0, _ => none
  | n+1, .inCaller body | n+1, .firstMention _ body | n+1, .laterMention _ body =>
    body.selectionKindOverride? n
  | _+1, .pro (.unionHalf word) _ | _+1, .pro (.word word) _ => some word.kind
  | _+1, .or _ => phrase.kind?
  | _+1, _ => none
termination_by structural fuel

def NounPhrase.selectedKind (phrase : NounPhrase) (drs : DRS)
    (ids : List ReferentId) : Kind :=
  match phrase.selectionKindOverride? with
  | some kind => kind
  | none => ((ids.head?.bind drs.findById?).map (·.kind)).getD (phrase.kindIn drs .object)

def Predicate.seedZone (p : Predicate) (fuel : Nat := 32) : Option Zone :=
  match fuel, p with
  | 0, _ => none
  | n + 1, .inCaller body | n + 1, .bracketed body => body.seedZone n
  | _ + 1, .in_ z => z.sort
  | _ + 1, .inCombat .attackedBy _ => none
  | _ + 1, .isToken | _ + 1, .inCombat .. | _ + 1, .attachment ..
  | _ + 1, .currentFace _ => some .battlefield
  | _ + 1, .abilityHead _ | _ + 1, .abilityOf _ | _ + 1, .activatedBy _
  | _ + 1, .targets _ _ => some .stack
  | _ + 1, .isEmblem => some .command
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

def NounPhrase.plur (np : NounPhrase) (fuel : Nat := 32) : Plurality :=
  match fuel, np with
  | 0, _ => .one
  | n + 1, .inCaller body | n + 1, .firstMention _ body | n + 1, .laterMention _ body => body.plur n
  | _ + 1, .described d _ => d.plur
  | _ + 1, .someOf c _ _ | _ + 1, .pileOf c _ => c.plur
  | _ + 1, .theRest _ pl | _ + 1, .pro _ pl | _ + 1, .refl pl
  | _ + 1, .paid (.objects _ _ pl) _ => pl
  | n + 1, .slice _ (.lit 1) (.zone .library (some who))
  | n + 1, .slice _ (.lit 1) (.zone .graveyard (some who)) =>
    -- "the top card of each player's library": one per player, collected
    if who.det n == some .each then .many else who.plur n
  | _ + 1, .slice _ (.lit 1) _ => .one
  | _ + 1, .slice .. | _ + 1, .run .. | _ + 1, .oneEachOf .. => .many
  | _ + 1, .and _ => .many
  | n + 1, .or (x :: _) => x.plur n
  | _ + 1, _ => .one
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

def NounPhrase.anchorPhrase (phrase : NounPhrase) (fuel : Nat := 32) : Bool :=
  match fuel, phrase with
  | n + 1, .inCaller body | n + 1, .firstMention _ body => body.anchorPhrase n
  | n + 1, .or alternatives => alternatives.all (·.anchorPhrase n)
  | _ + 1, .and _ => false
  | _ + 1, phrase => phrase.det.all (· == .target)
  | _, _ => false
termination_by structural fuel

def NounPhrase.groupMention (phrase : NounPhrase) (fuel : Nat := 32) : Bool :=
  match fuel, phrase with
  | n + 1, .inCaller body | n + 1, .firstMention _ body => body.groupMention n
  | _ + 1, .laterMention _ body => body.plur != .one
  | _ + 1, .slice .. | _ + 1, .run .. | _ + 1, .and _ | _ + 1, .oneEachOf .. => true
  | _ + 1, .pro _ plur => plur != .one
  | _ + 1, .paid (.objects _ _ plur) _ => plur != .one
  | _ + 1, phrase => phrase.det == some .target
  | _, _ => false
termination_by structural fuel

def NounPhrase.countableGroup (phrase : NounPhrase) : Bool :=
  phrase.det == some .bare || phrase.det == some .count || phrase.groupMention

def NounPhrase.existentialMention (phrase : NounPhrase) (fuel : Nat := 32) : Bool :=
  match fuel, phrase with
  | n + 1, .inCaller body | n + 1, .firstMention _ body => body.existentialMention n
  | _ + 1, .agree _ _ group => group.det == some .count || group.det == some .target
  | _ + 1, phrase => phrase.det == some .bare
  | _, _ => false
termination_by structural fuel

def NounPhrase.pileMention (phrase : NounPhrase) (fuel : Nat := 32) : Bool :=
  match fuel, phrase with
  | n + 1, .inCaller body | n + 1, .firstMention _ body
  | n + 1, .laterMention _ body => body.pileMention n
  | _ + 1, .pileOf .. | _ + 1, .pro (.word .pile) _ => true
  | _, _ => false
termination_by structural fuel

def NounPhrase.partitiveBase (phrase : NounPhrase) (fuel : Nat := 32) : Bool :=
  match fuel, phrase with
  | n + 1, .inCaller body | n + 1, .firstMention _ body => body.partitiveBase n
  | _ + 1, .laterMention .. => true
  | _ + 1, phrase => phrase.det == some .all || phrase.groupMention
  | _, _ => false
termination_by structural fuel

/-- A group of players written by description, read as a whole: "all players", "your opponents"
("all" over a description of players). Each member holds alone what the group is said to hold. -/
def NounPhrase.playerGroupMention (phrase : NounPhrase) (fuel : Nat := 8) : Bool :=
  match fuel, phrase with
  | n + 1, .inCaller body | n + 1, .firstMention _ body | n + 1, .laterMention _ body =>
    body.playerGroupMention n
  | _ + 1, .described .all p => p.kind? == some .player
  | _, _ => false
termination_by structural fuel

def NounPhrase.soleHolderOk (phrase : NounPhrase) : Bool :=
  phrase.playerGroupMention || phrase.plur == .one

def NounPhrase.slicePossessorOk (phrase : NounPhrase) : Bool :=
  phrase.det == some .each || phrase.playerGroupMention || phrase.plur == .one

def Predicate.headIsPlaceless (p : Predicate) (fuel : Nat := 64) : Bool :=
  match fuel, p with
  | n + 1, .inCaller b | n + 1, .bracketed b => b.headIsPlaceless n
  | _ + 1, .isSource | _ + 1, .isCard => true
  | n + 1, .and ps => ps.any (fun p => p.headIsPlaceless n)
  | _, _ => false
termination_by structural fuel

def Predicate.explicitZone (predicate : Predicate) (fuel : Nat := 32) : Option Zone :=
  match fuel, predicate with
  | n + 1, .inCaller body | n + 1, .bracketed body => body.explicitZone n
  | _ + 1, .in_ zone => zone.sort
  | n + 1, .and predicates => predicates.findSome? (·.explicitZone n)
  | n + 1, .or (first :: rest) =>
    let zone := first.explicitZone n
    if rest.all (fun predicate => predicate.explicitZone n == zone) then zone else none
  | _, _ => none
termination_by structural fuel

def Predicate.phraseZone (kind : Kind) (predicate : Predicate) : Option Zone :=
  if !Kind.lte kind .object then none
  else if predicate.headIsPlaceless then predicate.explicitZone
  else predicate.seedZone.orElse (fun _ => some .battlefield)

def Predicate.seedsAbility (p : Predicate) (fuel : Nat := 64) : Bool :=
  match fuel, p with
  | n + 1, .inCaller b | n + 1, .bracketed b | n + 1, .compareOver b _ _ _ =>
    b.seedsAbility n
  | _ + 1, .abilityHead _ | _ + 1, .abilityOf _ | _ + 1, .activatedBy _ => true
  | n + 1, .and ps => ps.any (fun p => p.seedsAbility n)
  | n + 1, .or ps => !ps.isEmpty && ps.all (fun p => p.seedsAbility n)
  | _, _ => false
termination_by structural fuel

def NounPhrase.eventAgentOk (phrase : NounPhrase) (fuel : Nat := 32) : Bool :=
  match fuel, phrase with
  | n + 1, .inCaller body | n + 1, .firstMention _ body
  | n + 1, .headed _ body => body.eventAgentOk n
  | _ + 1, .you | _ + 1, .this | _ + 1, .actor | _ + 1, .pro ..
  | _ + 1, .which | _ + 1, .refl _ | _ + 1, .laterMention .. => true
  | _ + 1, phrase => phrase.playerGroupMention
  | _, _ => false
termination_by structural fuel

def NounPhrase.tokenPhrase (phrase : NounPhrase) (fuel : Nat := 32) : Bool :=
  match fuel, phrase with
  | n + 1, .inCaller body | n + 1, .firstMention _ body => body.tokenPhrase n
  | _ + 1, .described (.count _ _) predicate | _ + 1, .described (.a _) predicate => predicate.seedsToken
  | _, _ => false
termination_by structural fuel

def TokenSpec.eventSourceOk (spec : TokenSpec) (fuel : Nat := 32) : Bool :=
  match fuel, spec with
  | n + 1, .inCaller body => body.eventSourceOk n
  | _ + 1, .copyOf source _ => source.tokenPhrase
  | _ + 1, _ => true
  | _, _ => false
termination_by structural fuel

def predicateOfKind (k : Kind) (p : Predicate) : Predicate :=
  match p with
  | .or ps => .or (ps.filter (fun p => p.kind?.all (· == k)))
  | p => p

def predicatePayload (k : Kind) (p : Predicate) (fuel : Nat := 64) : Payload :=
  match fuel, k with
  | 0, _ => .difference
  | _ + 1, .object =>
    let p := predicateOfKind .object p
    if p.seedsAbility then .ability none (p.explicitZone.orElse (fun _ => some .stack))
    else .object p.seedTy (p.phraseZone .object) none
      (if p.seedsToken then some .token else none) none
  | _ + 1, .player => .player false
  | _ + 1, .quality q => .quality q
  | _ + 1, .letter l => .letter l
  | _ + 1, .turnRef => .turnRef
  | _ + 1, .pile => .pile none none none
  | n + 1, .join l r => .join (predicatePayload l p n) (predicatePayload r p n)
  | _ + 1, _ => .difference
termination_by structural fuel

def bindFor (det : Determiner) (plur : Plurality) (k : Kind) (p : Predicate) : Referent :=
  { det, plur, payload := predicatePayload k p }

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
      if state.historicalReferentIds.contains oldId then state else
      let payload : Option Payload := match old.payload with
        | .object ty oldZone _ orig size =>
          some (.object ty destZone (mkStamp deed oldZone true) orig size)
        | .ability orig _ => some (.ability orig destZone)
        | _ => none
      match payload with
      | none => state
      | some payload =>
        let freshId := state.nextId
        let remap := fun id => if id == oldId then freshId else id
        let remapRole := fun (id, role) => (remap id, role)
        let refs := state.referents.map fun r =>
          if r.id == oldId then
            { r with
              id := freshId
              idents := []
              payload := payload
              librarySearchResult := false }
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
          selectionEvidence := state.selectionEvidence.map (fun evidence =>
            { evidence with sourceGroup := evidence.sourceGroup.map remap })
          outerReferentIds := state.outerReferentIds.map (fun ids => ids.map remap)
          groupSizeScope := state.groupSizeScope.map (fun ids => ids.map remap)
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
          | .object ty zn oldProv orig sz =>
            let newStamp := match oldProv with
              | some ⟨_, wasPerm, moved⟩ => some ⟨v, wasPerm, moved⟩
              | none => mkStamp (some v) zn false
            .object ty zn newStamp orig sz
          | other => other
        { r with payload := newPayload }) drs

/-- Infer the canonical `Deed` of an action for `.actedOn` stamping. -/
def Action.inferredDeed (inst : Action) (fuel : Nat := 32) : Option Deed :=
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
  inst : StepPhase → GramRole → DRS → Action → DRS × List Refusal
  spec : DRS → StaticSpec → DRS × List Refusal
  cost : DRS → Cost → DRS × List Refusal
  ab   : DRS → Ability → List Refusal

def otherAnchorOkWith (ops : StepOps) (drs : DRS) (kind : Kind)
    (predicates : List Predicate) : Bool :=
  let flattened := Predicate.flatten predicates
  if (flattened.filter Predicate.isOther).length > 1 then false
  else if predicates.any Predicate.hasBareOther then anyTargeted kind drs
  else
    let headTypes := predicates.flatMap (fun predicate => predicate.headTyAlts.flatten)
    flattened.all fun predicate => match predicate with
      | .otherThan anchor =>
        let (out, ids, _) := ops.np drs .other false anchor
        let anchorTypes := (ids.filterMap out.findById?).flatMap (·.ty)
        anchorTyFits headTypes anchorTypes
      | _ => true

def stepQuantityWith (ops : StepOps) (drs : DRS) (q : Quantity) (fuel : Nat := 8) : DRS × List Refusal :=
  match fuel, q with
  | 0, _ => (drs, [.recursionLimit])
  | n + 1, .inCaller body =>
    let (callerDrs, errs) := stepQuantityWith ops (enterCaller drs) body n
    (leaveCaller drs callerDrs, errs)
  | _ + 1, .range (some lo) (some hi) => (drs, if lo ≤ hi then [] else [.wellFormedQ])
  | _ + 1, .range _ _ | _ + 1, .worth _ => (drs, [])
  | _ + 1, .upToOf a | _ + 1, .exactlyOf a | _ + 1, .moreThanOf a => ops.amt drs a
  | n + 1, .additional q => stepQuantityWith ops drs q n
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
  | .mk times _ => ops.amt drs times

def NounPhrase.ascribableIn (drs : DRS) (noun : NounPhrase) (fuel : Nat := 32) : Bool :=
  match fuel, noun with
  | n + 1, .inCaller body => body.ascribableIn (enterCaller drs) n
  | n + 1, .firstMention _ body | n + 1, .laterMention _ body => body.ascribableIn drs n
  | _ + 1, .this | _ + 1, .designated _ _ | _ + 1, .which => true
  | _ + 1, .actor => (actorShape? drs).any (·.ascribable)
  | _, _ => false

def NounPhrase.restContext? (drs : DRS) (noun : NounPhrase) (fuel : Nat := 32) :
    Option (Kind × Plurality × DRS) :=
  match fuel, noun with
  | n + 1, .inCaller body => body.restContext? (enterCaller drs) n
  | n + 1, .firstMention _ body => body.restContext? drs n
  | _ + 1, .theRest kind plurality => some (kind, plurality, drs)
  | _, _ => none

def NounPhrase.selfDefinedOk (np : NounPhrase) (fuel : Nat := 32) : Bool :=
  match fuel, np with
  | n + 1, .inCaller body | n + 1, .firstMention _ body
  | n + 1, .laterMention _ body | n + 1, .headed (.type _) body
  | n + 1, .headed (.subtype _) body => body.selfDefinedOk n
  | _ + 1, .this => true
  | _, _ => false
termination_by structural fuel

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
  | .typeOtherThan _ => (drs, [])
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

def choiceModeChecks (drs : DRS) : ChoiceMode → List Refusal
  | .theirChoice =>
    let count := match actorShape? drs with
      | some actor => if actor.kind == .player then
          (actor.refIds.eraseDups.filterMap drs.findById?).filter (fun ref => ref.kind == .player) |>.length
        else 0
      | none => countChoosers drs
    refuse (count == 1) (.chooserInScope count)
  | _ => []

def stepDetPhraseWith (ops : StepOps) (drs : DRS) (det : DetPhrase) : DRS × List Refusal :=
  match det with
  | .target quantity =>
    let (out, errors) := stepQuantityWith ops drs quantity
    (out, refuse quantity.nonZero .nonZeroQ ++ errors)
  | .count quantity mode =>
    let (out, errors) := stepQuantityWith ops drs quantity
    (out, refuse quantity.nonZero .nonZeroQ ++ errors ++ mode.toList.flatMap (choiceModeChecks drs))
  | .a mode => (drs, choiceModeChecks drs mode)
  | _ => (drs, [])

def stepSliceCountWith (ops : StepOps) (drs : DRS) (sc : SliceCount) : DRS × List Refusal :=
  match sc with
  | .counted q => stepQuantityWith ops drs q
  | .whole => (drs, [])
  | .totalling _ _ b => ops.amt drs b

private def publishLoopChoiceHistory (closed body : DRS) : DRS :=
  body.selectionEvidence.foldl (fun out evidence =>
    if evidence.scope == body.selectionScope && evidence.chosen && evidence.sourceGroup.isNone
    then out.recordSelection evidence.kind none true
    else out) closed

def stepSubjectBeforeWith (ops : StepOps) (drs : DRS) (s : Subject) : DRS × List Refusal :=
  match s with
  | .actor np =>
    let (drs1, refIds, e1) := ops.np drs .subj true np
    let shape : ActorShape := {
      kind := (refIds.head?.bind drs1.findById?).elim (np.kindIn drs1 .player) (·.kind),
      isYou := np.isYou,
      ascribable := false,
      refIds := refIds
    }
    let drs2 := drs1.push { det := .the, plur := np.plur, payload := .actor shape } .other false
    (drs2, e1)
  | .any | .by _ => (drs, [])

def stepSubjectAfterWith (ops : StepOps) (drs : DRS) (s : Subject)
    (expectedKind : Option Kind := none) (targeter : Bool := false) : DRS × List Refusal :=
  match s with
  | .actor _ => (dropActorFrame drs, [])
  | .any => (drs, [])
  | .by np =>
    let (drs', ids, e) := ops.np drs .other true np
    let kindErrors := match expectedKind, (ids.head?.bind drs'.findById?).map (·.kind) with
      | some expected, some actual => refuse (expected == actual) (.kindMismatch expected actual)
      | _, _ => []
    let targetErrors := if targeter then
        ((ids.head?.bind drs'.findById?).map (fun ref =>
          refuse ref.kind.targeter (.targeter ref.kind))).getD []
      else []
    (drs', e ++ kindErrors ++ targetErrors)

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

/-- A zone's owner, when one is named, is a player. -/
def stepZoneOwnerWith (ops : StepOps) (drs : DRS) (whose : Option NounPhrase) :
    DRS × List Refusal :=
  match whose with
  | none => (drs, [])
  | some np =>
    let (drs', ids, errs) := ops.np drs .other false np
    let kindErrors := if ids.isEmpty then [] else
      let actual := np.selectedKind drs' ids
      refuse (actual == .player) (.kindMismatch .player actual)
    (drs', errs ++ kindErrors)

def stepLocationWith (ops : StepOps) (drs : DRS) (z : Location) (fuel : Nat := 8) :
    DRS × List Refusal :=
  match fuel, z with
  | 0, _ => (drs, [.recursionLimit])
  | n + 1, .inCaller body =>
    let (callerDrs, errs) := stepLocationWith ops (enterCaller drs) body n
    (leaveCaller drs callerDrs, errs)
  | _ + 1, .zone _ whose => stepZoneOwnerWith ops drs whose
  | n + 1, .fromEnd _ ordinal loc =>
    let (out, errs) := stepLocationWith ops drs loc n
    (out, errs ++ refuse ordinal.ok .ordinalNonZero)
  | _ + 1, .exiledWith src =>
    let (out, _, errors) := ops.np drs .other false src
    (out, errors ++ refuse src.selfDefinedOk .linkSource)
  | _ + 1, .pile made =>
    let (out, ids, errors) := ops.np drs .other false made
    (out, errors ++ kindCheck .pile ((ids.head?.bind out.findById?).map (·.kind)) ++
      refuse made.pileMention .pileMention)
  | _ + 1, .anyLocation => (drs, [])
  | n + 1, .not loc => stepLocationWith ops drs loc n
  | n + 1, .or locs =>
    let (out, errs) := locs.foldl (fun (d, es) loc =>
      let (d', e') := stepLocationWith ops d loc n
      (d', es ++ e')) (drs, [])
    (out, errs ++ refuse (locs.length >= 2) .atLeastTwoZones)
termination_by structural fuel

private def attachmentHostKindCheck (word : Option AttachWord) (kind : Kind) : List Refusal :=
  refuse (Kind.lte kind (.join .object .player)) (.kindLte kind (.join .object .player)) ++
    refuse (word.elim true (fun word => word == .enchanted || Kind.lte kind .object))
      (.kindLte kind .object)

def stepAsThoughWith (ops : StepOps) (drs : DRS) (asThough : AsThough) :
    DRS × List Refusal :=
  match asThough with
  | .condition cond =>
    -- "as though it didn't have defender" is a clause of its own, its pronouns reading the
    -- deontic's participants.
    ops.cond (nextClause drs) cond
  | .greater _ amount =>
    let (out, errors) := ops.amt drs amount
    (out, errors ++ refuse (out.nextId == drs.nextId) .bindingless)
  | .mana .. => (drs, [])

private def stepDeonticModsWith (ops : StepOps) (drs : DRS) (mods : DeonticMods) :
    DRS × List Refusal :=
  let (out, errors) := match mods.asThough with
    | none => (drs, [])
    | some asThough => stepAsThoughWith ops drs asThough
  let (out, timingErrors) := match mods.timing with
    | none => (out, [])
    | some timing => stepTimingWith ops out timing
  let exceptErrors := match mods.except with
    | some (.byObjects objects) =>
      let (selected, ids, errs) := ops.np drs .other false objects
      errs ++ kindCheck .object ((ids.head?.bind selected.findById?).map (·.kind))
    | _ => []
  (out, errors ++ timingErrors ++ exceptErrors)

def eventVoiceChecks (drs : DRS) (subject : Subject) (action : Action) : List Refusal :=
  let agent := match subject with | .any => none | .actor noun | .by noun => some noun
  let agentRead := agent.all NounPhrase.eventAgentOk
  match action.scopeBody with
  | .putCounters _ _ _ _ byEffect | .removeCounters _ _ _ _ byEffect =>
    refuse agentRead .eventAgent ++
      refuse (!(byEffect && agent.isSome)) .causedByOk
  | .createObject _ spec byEffect under =>
    let tokenErrors := match spec with
      | .token token _ => refuse token.eventSourceOk .tokenPhrase
      | .emblem .. => []
    let underKind := under.map (fun noun => noun.kindIn drs .player)
    kindCheck .player underKind ++ tokenErrors ++
      refuse (agentRead && under.all (fun noun => noun.plur == .one && noun.eventAgentOk) &&
        !(byEffect && agent.isSome)) .verbedVoiceOk
  | _ => []

private def actionActorRefusals (drs : DRS) (expected : Kind) : List Refusal :=
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

def Action.performed (action : Action) : Bool :=
  match action.scopeBody with
  | .conclude .. | .separateIntoPiles .. | .choose .. | .vote ..
  | .copy .. | .changeLife _ | .addMana .. | .draw _ | .expose ..
  | .search .. | .shuffle _ | .flipCoins .. | .rollDice .. | .rerollStored ..
  | .createObject .. | .enact .. | .pay .. | .skipPart .. | .addTurnPart ..
  | .procedure .. | .withContinuation .optional .. => true
  | _ => false

def clauseActorKind (action : Action) : Option Kind :=
  match action.scopeBody with
  | .draw _ | .procedure .. | .flipCoins .. | .rollDice ..
  | .createObject .. | .putCounters .. | .removeCounters .. => some .player
  | .dealDamage .. => some .object
  | _ => none

private def eventActionRefusals (action : Action) : List Refusal :=
  match action.scopeBody with
  | .procedure _ subject .. => refuse (subject.plur == .one) .singular
  | _ => []

def stepDidClauseWith (ops : StepOps) (drs : DRS) (c : DidClause) : DRS × List Refusal :=
  match c with
  | ⟨subj, action, within⟩ =>
    let headErr := (if action.eventHeadOk then [] else [.eventHead]) ++ eventActionRefusals action
    let locusErrors := match action.scopeBody with
      | .enact verb _ _ (some zone) =>
        refuse (zone.sort.any (fun z => (actLociOf verb).elem z)) .lookbackLocus
      | _ => []
    let (drs1, e1) := stepSubjectBeforeWith ops drs subj
    let actorErrors := match action.scopeBody with
      | .draw _ => []
      | _ => match clauseActorKind action, actorShape? drs1 with
        | some expected, some shape => kindCheck expected (some shape.kind)
        | _, _ => []
    let historical : DRS := {
      drs1 with
      actionPattern := true
      historicalReferentIds := drs.referents.map (·.id) }
    let (drs2, e2) := ops.inst .postEvent subj.patientRole historical action
    let (drs3, e3) := stepSubjectAfterWith ops drs2 subj (clauseActorKind action) (match action.scopeBody with | .target _ => true | _ => false)
    let (drs4, e4) := stepLookbackSpanWith ops drs3 within
    ({ drs4 with
        actionPattern := drs.actionPattern
        historicalReferentIds := drs.historicalReferentIds }, headErr ++ locusErrors ++ e1 ++ actorErrors ++ e2 ++ e3 ++ e4 ++ eventVoiceChecks drs subj action)

/-- Step a relative clause (`Predicate.did`) with its `which` bound to `whichRef`; the clause
must mention `which`. -/
def stepRelativeDidWith (ops : StepOps) (drs : DRS) (whichRef : Referent) (clause : DidClause) :
    DRS × List Refusal :=
  let subDrs := nextClause drs
  let binderId := subDrs.nextId
  let subDrs' := { subDrs.push whichRef .other false with
    relativeBinder := some binderId, relativeUses := 0 }
  let (out, errs) := stepDidClauseWith ops subDrs' clause
  (leaveRelative drs out, errs ++ refuse (out.relativeUses != 0) .lookbackSubject)

/-- The referent a conjunct relative clause's `which` binds: the enclosing object predicate's
binding, so the clause reads the head's types and zone from its sibling conjuncts; any other
head binds its bare kind. -/
def relativeSeed (headKind : Kind) (enclosing : Predicate) : Referent :=
  if headKind == .object then bindFor .the .one .object enclosing else kindBinding headKind

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
    let headErr := (if action.eventHeadOk then [] else [.eventHead]) ++ eventActionRefusals action
    let (drs1, e1) := stepSubjectBeforeWith ops drs subj
    let actorErrors := match action.scopeBody with
      | .draw _ => []
      | _ => match clauseActorKind action, actorShape? drs1 with
        | some expected, some shape => kindCheck expected (some shape.kind)
        | _, _ => []
    let (drs2, e2) := ops.inst .preEvent subj.patientRole { drs1 with actionPattern := true } action
    let (drs3, e3) := stepSubjectAfterWith ops drs2 subj (clauseActorKind action) (match action.scopeBody with | .target _ => true | _ => false)
    ({ drs3 with actionPattern := drs.actionPattern }, headErr ++ e1 ++ actorErrors ++ e2 ++ e3 ++ eventVoiceChecks drs subj action)
  | n + 1, .wouldAny cs =>
    let (branches, errors) := cs.foldl (fun (branches, errors) c =>
      let (out, errs) := stepWouldClauseWith ops drs c n
      (branches ++ [out], errors ++ errs)) ([], [])
    (commonHeaderContext drs branches, errors)
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

def stepCounterKindSourceWith (ops : StepOps) (drs : DRS) (source : CounterKindSource) : DRS × List Refusal :=
  let (out, errors) := match source with
    | .sameAs subject =>
      let (out, ids, errors) := ops.np drs .other false subject
      let kind := ((ids.head?.bind out.findById?).map (·.kind)).getD (subject.kindIn out .object)
      (out, errors ++ refuse (counterHolderKind kind) .counterHolderKind)
    | _ => (drs, [])
  (out, errors ++ source.localChecks drs)

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

def stepTriggerHeaderWith (ops : StepOps) (drs : DRS) (event : GameEvent)
    (alternatives : List GameEvent) (while_ timing : Option Timing) : DRS × List Refusal :=
  let (branches, errors) := (event :: alternatives).foldl (fun (states, errors) header =>
    let (out, errs) := ops.ev drs header
    (states ++ [nextClause out], errors ++ errs)) ([], [])
  let common := commonHeaderContext drs branches
  let (whileContext, whileErrors) := match while_ with
    | some t => stepTimingWith ops common t
    | none => (common, [])
  let (timingContext, timingErrors) := match timing with
    | some t => stepTimingWith ops common t
    | none => (common, [])
  ({ common with nextId := max whileContext.nextId timingContext.nextId },
    errors ++ whileErrors ++ timingErrors)

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

/-- A chosen quality is read as a chosen one. -/
def WrittenQuality.shape : WrittenQuality → List Refusal
  | .chosen q => refuse q.qualityReadOk .qualityRead
  | .everyTypeOf _ => []

def stepWrittenQualityWith (ops : StepOps) (drs : DRS) : WrittenQuality → DRS × List Refusal
  | .chosen p => ops.pred drs p
  | .everyTypeOf _ => (drs, [])

def CharacteristicEdit.shape : CharacteristicEdit → List Refusal
  | .typeLine op c =>
    refuse (op == .sets || !c.hasWrites || c.hasValues) .lineNonEmpty ++
      refuse c.canonical .distinct
  | .colors op cs => refuse cs.ok .distinct ++ refuse (colorOpOk op cs) .nonEmpty
  | .quality _ q => q.shape
  | _ => []

def CharacteristicEdit.shapeAll (edits : List CharacteristicEdit) : List Refusal :=
  edits.flatMap CharacteristicEdit.shape

/-- An edit a Characteristic-Defining Ability may write: it defines colors, subtypes, power or
toughness [CR#604.3a]. A color or subtype edit adds or sets (a definition never removes); a
power or toughness edit sets. -/
def CharacteristicEdit.definesOk : CharacteristicEdit → Bool
  | .colors op _ | .quality op (.everyTypeOf _) => op != .loses
  | .typeLine op changes =>
    op != .loses && changes.supertypes.isNone && changes.types.isNone && changes.subtypes.isSome
  | .stat .sets .power _ | .stat .sets .toughness _ => true
  | _ => false

/-- Whether a Static Spec writes a Characteristic-Defining Ability. -/
def StaticSpec.writesDefining (spec : StaticSpec) (fuel : Nat := 64) : Bool :=
  match fuel, spec with
  | _ + 1, .characteristicDefining _ _ => true
  | n + 1, .inCaller s | n + 1, .retention s _ | n + 1, .conditional s _ _ => s.writesDefining n
  | n + 1, .conjunction _ parts => parts.any (fun s => s.writesDefining n)
  | _, _ => false
termination_by structural fuel

/-- Whether an ability, as written in a list of abilities, is a Characteristic-Defining
Ability. A keyword's own definition is not looked into: a keyword ability granted is the
keyword, whatever its definition writes. -/
def Ability.writesDefining (ability : Ability) (fuel : Nat := 64) : Bool :=
  match fuel, ability with
  | n + 1, .static spec => spec.writesDefining n
  | n + 1, .italicHead _ a | n + 1, .inCaller a | n + 1, .qualified a _ _ _ _ | n + 1, .alsoForKeywords a _ =>
    a.writesDefining n
  | _, _ => false
termination_by structural fuel

def KeywordParam.shape : KeywordParam → KeywordParamShape
  | .cost _ => .cost
  | .quality _ => .quality
  | .subject _ => .subject
  | .number _ => .number
  | .deckCondition _ => .deckCondition
  | .ability _ => .ability

/-- A prohibition publishes the sources its clause names (a target, a definite mention, an
attachment host) to the text after it. -/
def publishProhibitionSources (before after : DRS) : DRS :=
  let sources := after.referents.filter (fun ref => ref.id ≥ before.nextId &&
    !ref.isActorFrame && !ref.isDifference &&
    (Kind.lte ref.kind (.join .object .player)) &&
    (ref.det == .target || ref.det == .the || ref.attachmentHost))
  let ids := sources.map (·.id)
  { before with
    referents := sources ++ before.referents
    nextId := max before.nextId after.nextId
    nextSelectionScope := max before.nextSelectionScope after.nextSelectionScope
    pendingRoles := after.pendingRoles.filter (fun mention => ids.contains mention.1) ++ before.pendingRoles }

def keywordSchemaFor (k : KeywordLabel) (params : List KeywordParam) : Option KeywordSchema :=
  (keywordSchemas k).find? (fun schema => schema.shapes == params.map KeywordParam.shape)

def keywordParamFits (k : KeywordLabel) (params : List KeywordParam) : Bool :=
  (keywordSchemaFor k params).isSome

def checkDeckTraitWith (ops : StepOps) (trait : DeckTrait) (fuel : Nat := 32) : List Refusal :=
  match fuel, trait with
  | 0, _ => [.recursionLimit]
  | _ + 1, .aCharacteristic predicate =>
    (ops.pred (DRS.ofList []) predicate).2 ++ kindCheck .object predicate.kind?
  | _ + 1, .hasAbilityOf class_ => refuse class_.known .knownKeywordTerm
  | n + 1, .anyOf traits =>
    refuse (!traits.isEmpty) .nonEmpty ++ traits.flatMap (fun t => checkDeckTraitWith ops t n)
  | _, _ => []

def checkDeckConditionWith (ops : StepOps) (condition : DeckCondition) : List Refusal :=
  match condition with
  | .everyCardIs scope trait | .noCardIs scope trait =>
    (ops.pred (DRS.ofList []) scope).2 ++ kindCheck .object scope.kind? ++
      checkDeckTraitWith ops trait
  | .groupAgrees (.agree _ _ group@(.described _ scope)) =>
    (ops.pred (DRS.ofList []) scope).2 ++ kindCheck .object scope.kind? ++
      refuse (group.plur == .many) .plural
  | .groupAgrees group =>
    let (out, ids, errors) := ops.np (DRS.ofList []) .other false group
    errors ++ kindCheck .object ((ids.head?.bind out.findById?).map (·.kind))
  | .deckSizeOverMinimum _ => []

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
    | .deckCondition condition => checkDeckConditionWith ops condition
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
  | .manaCost _ => (drs, [])
  | .stat _ _ amt => ops.amt drs amt
  | .quality _ q => stepWrittenQualityWith ops drs q
  | .addedAbilities abs =>
    -- A granted keyword whose number is a letter ("has toxic X") introduces that letter.
    let out := abs.foldl (fun d ab => match ab with
      | .keyword _ [.number (.letter l)] _
      | .keyword _ [.number (.inCaller (.letter l))] _ =>
        (introducedLetters l drs).foldl (fun d r => d.push r .other false) d
      | _ => d) drs
    (out, (stepAbilityListWith ops (choiceContext drs) abs).2 ++
      refuse (!abs.any (fun ab => ab.writesDefining)) .cdaGranted)
  | .addedAbilitiesOf src _ optEx =>
    let (drs1, _, e1) := ops.np drs .other false src
    let (drs2, e2) := match optEx with | some p => ops.pred drs1 p | none => (drs1, [])
    (drs2, e1 ++ e2)
  | .addedSelectionOf src sel =>
    let (drs1, _, e1) := ops.np drs .other false src
    let (drs2, e2) := stepAbilitySelectionWith ops drs1 sel
    (drs2, e1 ++ e2)
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

/-- A search of several zones names each zone once. -/
def stepSearchLocationWith (ops : StepOps) (drs : DRS) (sc : Location) : DRS × List Refusal :=
  let (out, errors) := stepLocationWith ops drs sc
  let distinct := match sc with
    | .or locs =>
      let zones := locs.map (·.sort)
      zones.eraseDups.length == zones.length
    | _ => true
  (out, errors ++ refuse distinct .atLeastTwoZones)

def stepVisibleThingWith (ops : StepOps) (drs : DRS) (vt : VisibleThing) : DRS × List Refusal :=
  match vt with
  | .topOfLibrary | .wholeHand => (drs, [])
  | .objects np =>
    let (drs', _, e) := ops.np drs .obj true np
    (drs', e)

private def insertReferentById (ref : Referent) : List Referent → List Referent
  | [] => [ref]
  | first :: rest =>
    if ref.id >= first.id then ref :: first :: rest
    else first :: insertReferentById ref rest

def stepExchangedWith (ops : StepOps) (drs : DRS) (ex : Exchanged) : DRS × List Refusal :=
  match ex with
  | .lifeTotals parties =>
    let (drs1, ids, e1) := ops.np drs .obj true parties
    let kind := (ids.head?.bind drs1.findById?).map (·.kind) |>.getD (parties.kindIn drs1 .player)
    let drs2 := (drs1.push (outcomeB .lifeGained) .other false).push (outcomeB .lifeLost) .other false
    (drs2, e1 ++ kindCheck .player (some kind))
  | .controlOf l r | .cardsAcross l r =>
    let (drs1, _, e1) := ops.np drs .obj true l
    let (drs2, _, e2) := ops.np drs1 .other true r
    (drs2, e1 ++ e2)
  | .zones l r =>
    let (drs1, e1) := stepLocationWith ops drs l
    let (drs2, e2) := stepLocationWith ops drs1 r
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
  | .anyBallot => (drs, refuse drs.actionPattern .concreteBallot)
  | .byLabel labels => (drs, refuse (ballotLabelsOk labels) .ballotLabelsOk)
  | .byCandidate np =>
    let (drs', _, e) := ops.np drs .obj true np
    (drs', e ++ refuse (np.det == some .a || np.det == some .target || np.det == some .count)
      .choosable)

def stepRollRowListWith (ops : StepOps) (drs : DRS) (rows : List RollRow) : DRS × List Refusal :=
  rows.foldl (fun (d, es) ⟨q, inst⟩ =>
    let (d1, e1) := stepQuantityWith ops d q
    let (d2, e2) := ops.inst .postEvent .obj (nextClause d1) inst
    (d2, es ++ e1 ++ refuse q.nonZero .nonZeroQ ++ refuse q.literal .quantLiteral ++ e2))
      (drs, [])

def Action.hasRepeatedPayment (action : Action) (fuel : Nat := 32) : Bool :=
  match fuel, action with
  | n + 1, .inCaller body | n + 1, .ownScope _ body | n + 1, .bracketed body =>
    body.hasRepeatedPayment n
  | _ + 1, .pay _ times .paid _ _ => times != .once
  | _, _ => false
termination_by structural fuel

def finishEachSubject (subject : NounPhrase) (ids : List ReferentId) (drs : DRS) : DRS :=
  if subject.det == some .each then
    ids.foldl (fun context id => context.updateById id (fun ref => { ref with plur := .many })) drs
  else drs

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
  | .setQuantity q =>
    let (out, errors) := stepQuantityWith ops drs q
    (out, errors ++ refuse q.nonZero .nonZeroQ)
  | .repeatable => (drs, [])
  | .notChosenBefore dur => stepDurationWith ops drs dur

def stepModeListWith (ops : StepOps) (drs : DRS)
    (modes : List (Option Cost × Option Nat × Action)) : DRS × List Refusal :=
  modes.foldl (fun (out, errors) (cost, _, action) =>
    let branch := { drs with nextId := out.nextId }
    let (afterCost, costErrors) := match cost with
      | some cost => ops.cost branch cost
      | none => (branch, [])
    let (afterBody, bodyErrors) := ops.inst .postEvent .obj (nextClause afterCost) action
    ({ drs with nextId := max out.nextId afterBody.nextId }, errors ++ costErrors ++ bodyErrors)) (drs, [])

def actorPlurality (drs : DRS) : Plurality :=
  ((drs.referents.find? Referent.isActorFrame).map (·.plur)).getD .one

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
  | .toActivate none => (drs, [])
  | .toPay cost => (drs, refuse cost.nameable .costNameable)

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
    | .generic change optFl =>
      let (drs1, e1) := ops.amt drs change.amount
      let (drs2, e2) := match optFl with | some fl => ops.amt drs1 fl | none => (drs1, [])
      (drs2, e1 ++ e2)
    | .bySymbols change _ =>
      match change.amount with
      | .lit _ => (drs, [])
      | .manaCostOf obj =>
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
    let (drs2, ids, e2) := ops.np drs1 .other true to
    let kind := (ids.head?.bind drs2.findById?).map (·.kind) |>.getD (to.kindIn drs2 .object)
    (drs2, e1 ++ e2 ++ refuse (to.plur == .one) .singular ++
      refuse (Kind.lte kind (.join .object .player)) (.kindLte kind (.join .object .player)))
  | .scale (.multiplied _) | .scale (.halved _) => (drs, [])
  | .scale (.shifted change) =>
    let (out, errors) := ops.amt drs change.amount
    (out, errors ++ refuse (!change.isSet) .shiftSets)

private def stepMoveRiderFuel (ops : StepOps) (fuel : Nat) (drs : DRS) (r : MoveRider) : DRS × List Refusal :=
  match fuel with
  | 0 => (drs, [.recursionLimit])
  | fuel + 1 =>
   match r with
  | .inCaller body =>
    let (out, errors) := stepMoveRiderFuel ops fuel (enterCaller drs) body
    (leaveCaller drs out, errors)
  | .entersAs _ | .entersTransformed | .entersMelded _ | .faceDown _ | .inOrder _ => (drs, [])
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

termination_by structural fuel

def stepMoveRiderWith (ops : StepOps) (drs : DRS) (r : MoveRider) : DRS × List Refusal :=
  stepMoveRiderFuel ops 64 drs r

def stepMoveRiderListWith (ops : StepOps) (drs : DRS) (mrs : List MoveRider) : DRS × List Refusal :=
  mrs.foldl (fun (d, es) mr =>
    let (d', e') := stepMoveRiderWith ops d mr
    (d', es ++ e')) (drs, [])

/-- Whether `action` brings a permanent onto the battlefield, read through the wrappers that keep
its event: a move there, or a token's creation [CR#111.1,614.1c]. Only an entering carries Entry
Riders. -/
def Action.isEntering (action : Action) (fuel : Nat := 8) : Bool :=
  match fuel, action with
  | 0, _ => false
  | _ + 1, .move _ _ to _ _ _ => to.bind (·.sort) == some .battlefield
  | _ + 1, .createObject _ (.token _ _) _ _ => true
  | n + 1, .inCaller body | n + 1, .ownScope _ body | n + 1, .instruct _ body
  | n + 1, .enact _ body _ _ => body.isEntering n
  | _ + 1, _ => false
termination_by structural fuel

/-- Whether every event `clause` names is an entering. -/
def WouldClause.isEntering (clause : WouldClause) (fuel : Nat := 8) : Bool :=
  match fuel, clause with
  | 0, _ => false
  | _ + 1, .would _ action => action.isEntering
  | n + 1, .wouldAny clauses => !clauses.isEmpty && clauses.all (fun c => c.isEntering n)
  | n + 1, .causedBy _ clause => clause.isEntering n
termination_by structural fuel

/-- Whether `rider` is a Pile Placement, which only a card put into a zone other than the
battlefield carries: its order there [CR#401.4], or its exile face down as a card with no
characteristics [CR#406.3]. -/
def MoveRider.isPilePlacement (rider : MoveRider) (fuel : Nat := 8) : Bool :=
  match fuel, rider with
  | 0, _ => false
  | n + 1, .inCaller body => body.isPilePlacement n
  | _ + 1, .inOrder _ => true
  | _ + 1, .faceDown spec => spec.characteristics.isNone
  | _ + 1, _ => false
termination_by structural fuel

/-- A replacement, read after the event it replaces, `replacesEntering` when that event is an
entering. `instead` and `also` read their action; `withRiders` reads its riders, which must be
Entry Riders over an entering; `onlyIf` reads its condition, then its body in a new clause after
it, as `doIf` reads its then-branch. A caller's replacement is read in its caller's scope. -/
def stepReplacementWith (ops : StepOps) (drs : DRS) (r : Replacement) (replacesEntering : Bool)
    (fuel : Nat := 8) : DRS × List Refusal :=
  match fuel, r with
  | 0, _ => (drs, [.recursionLimit])
  | n + 1, .inCaller body =>
    let (callerDrs, errs) := stepReplacementWith ops (enterCaller drs) body replacesEntering n
    (leaveCaller drs callerDrs, errs)
  | _ + 1, .instead action | _ + 1, .also action => ops.inst .postEvent .obj drs action
  | _ + 1, .withRiders riders =>
    let (out, errs) := stepMoveRiderListWith ops drs riders
    (out, errs ++ refuse replacesEntering .riddenEntering ++
      refuse (!riders.any (fun rider => rider.isPilePlacement)) .entryRidersOnly)
  | n + 1, .onlyIf condition body =>
    let (drs1, e1) := ops.cond drs condition
    let (drs2, e2) := stepReplacementWith ops (nextClause drs1) body replacesEntering n
    (drs2, e1 ++ e2)
termination_by structural fuel

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
  bundle.qualities.foldl (fun (d, es) q =>
    let (d', e) := stepWrittenQualityWith ops d q
    (d', es ++ e ++ q.shape)) (out, errs)

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
  | _ + 1, .asThose =>
    match drs.prospectiveTokenTypes with
    | [] =>
      let count := countTokenSpecs drs
      (drs, tyOfThoseAny .token drs, refuse (count == 1) (.tokenSpecInScope count))
    | descriptions =>
      let count := descriptions.length
      (drs, descriptions.headD [], refuse (count == 1) (.tokenSpecInScope count))
  | _ + 1, .copyOf source exceptions =>
    let (selected, ids, errors) := ops.np drs .other false source
    let sources : List Referent := ids.filterMap selected.findById?
    let types := match sources with
      | [] => []
      | first :: rest => rest.foldl (fun acc (ref : Referent) => commonTypes acc ref.ty) first.ty
    let (out, exceptionErrors) := stepCopyExceptListWith ops selected exceptions
    (out, types, errors ++ exceptionErrors)
termination_by structural fuel

def stepCreationSpecMetadataWith (ops : StepOps) (drs : DRS)
    (spec : CreationSpec) : DRS × List CardType × List Refusal :=
  match spec with
  | .token token riders =>
    let (afterToken, types, errors) := stepTokenSpecMetadataWith ops drs token
    let (out, riderErrors) := stepMoveRiderListWith ops afterToken riders
    (out, types, errors ++ riderErrors ++ spec.localChecks)
  | .emblem _ abilities =>
    (drs, [], (stepAbilityListWith ops (choiceContext drs) abilities).2 ++ spec.localChecks)

/-- A play's payment: the cost is paid while casting or playing [CR#601.2h], not an argument of
the play. The cost's mentions leave the clause's pending argument roles, and the enclosing frames'
for an `inCaller` cost, as they were (none remain pending when the cost's own steps closed the
clause), so they never stand as patients of the deed, as the land card retrace discards does not
[CR#702.81a]. -/
def stepPlayPaymentWith (ops : StepOps) (drs : DRS) (pay : PlayPayment) : DRS × List Refusal :=
  match pay with
  | .itsOwnCost | .withoutPaying => (drs, [])
  | .payingInstead c | .faceDown _ c =>
    let (paid, errs) := ops.cost drs c
    let pending := if paid.currentClause == drs.currentClause then drs.pendingRoles else []
    let frames := if paid.scopeStack.length == drs.scopeStack.length then
        List.zipWith (fun outer inner => { inner with callerPending := outer.callerPending })
          drs.scopeStack paid.scopeStack
      else paid.scopeStack
    ({ paid with pendingRoles := pending, scopeStack := frames }, errs)

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
        | .bracketed body => ops.pred drs body
        | .not body =>
          let (out, errors) := ops.pred drs body
          ({ drs with nextId := out.nextId }, errors ++ refuse body.negatable .negatable)
        | .chosenPlayer ref =>
          let n := countChoice .player drs
          (drs, if ref.ok n then [] else [.choiceRef ref .player n])
        | .qualityNoun q dom => sortedDomainCheckWith ops drs (.quality q) dom
        | .ofYourChoice q dom =>
          let (out, errs) := sortedDomainCheckWith ops drs (.quality q) dom
          (out, refuse q.chosenReadOk (.chosenQualityRead q) ++ errs)
        | .ofChosen ref q =>
          let n := countChoice (.quality q) drs
          (drs, refuse q.chosenReadOk (.chosenQualityRead q) ++
            (if ref.ok n then [] else [.choiceRef ref (.quality q) n]))
        | .hasCounters kind =>
          -- Objects and players have counters [CR#122.1].
          let headKind := drs.relativeKind.getD .object
          let knownErrors := kind.toList.flatMap (fun counter => refuse counter.known .knownCounter)
          (drs, refuse (counterHolderKind headKind) .counterHolderKind ++ knownErrors)
        | .hasPossessor _ np =>
          let (drs', _, errs) := ops.np drs .other false np
          (drs', errs ++ refuse np.soleHolderOk .soleHolder)
        | .isOpponentOf who | .isTeammateOf who =>
          -- "an opponent" leaves its relatum "you" unwritten: it is read, not mentioned, so it
          -- introduces no referent. A written relatum is a mention like any other.
          let (out, ids, errors) := ops.np drs .other false who
          let errors := errors ++ kindCheck .player ((ids.head?.bind out.findById?).map (·.kind))
          (if who.isYou then drs else out, errors)
        | .did clause =>
          stepRelativeDidWith ops drs (kindBinding (drs.relativeKind.getD .object)) clause
        | .colorCount _ _ => (drs, [])
        | .compare axes _ bound =>
          let (out, errors) := ops.amt drs bound
          let kind := drs.relativeKind.getD .object
          let axisErrors := axes.flatMap fun axis => match axis with
            | .counter counter => refuse counter.known .knownCounter
            | _ => []
          (out, refuse (axesAt kind axes) (.axesAt kind) ++ axisErrors ++ errors)
        | .superlative op axis domain =>
          let (out, errors) := ops.pred drs domain
          let kind := drs.relativeKind.getD .object
          let axisErrors := match axis with
            | .counter counter => refuse counter.known .knownCounter
            | _ => []
          (out, refuse op.isExtremal .isExtremal ++ refuse (axis.scope == kind) (.projScope kind) ++ axisErrors ++ errors)
        | .withMostVotes =>
          let n := countOutcomes .voteHeld drs
          (drs, if n == 1 then [] else [.outcomeInScope .voteHeld n])
        | .choseExtreme _ =>
          let drs' := drs.push (outcomeB .namedNumber) .other false
          (drs', if countManys (.quality .number) drs != 0 then [] else [.numberChoiceInScope])
        | .compareOver dom m _ b =>
          let (domainDrs, domainErrors) := ops.pred drs dom
          let kind := (dom.kind? <|> drs.relativeKind).getD .object
          let member := bindFor .the .one kind dom
          let candidateContext := (nextClause domainDrs).push member .other false
          let (measured, measureErrors) := ops.amt candidateContext m
          let boundContext := { domainDrs with
            referents := domainDrs.referents.filter (fun ref => ref.id < drs.nextId)
            nextId := measured.nextId }
          let (bounded, boundErrors) := ops.amt boundContext b
          let boundAdditions := bounded.referents.filter (fun ref => ref.id ≥ boundContext.nextId)
          let domainAdditions := domainDrs.referents.filter (fun ref => ref.id ≥ drs.nextId)
          let inherited := bounded.referents.filter (fun ref => ref.id < drs.nextId)
          let out := { bounded with referents := boundAdditions ++ domainAdditions ++ inherited }
          (out.push differenceB .other false, refuse kind.phrasal (.phrasal kind) ++
            domainErrors ++ measureErrors ++ boundErrors)
        | .attachment side word counterpart =>
          let kind := drs.relativeKind.getD .object
          let headErrors := match side with
            | .host => attachmentHostKindCheck word kind
            | .attachment => kindCheck kind (some .object)
          let (out, counterpartErrors) := match counterpart with
            | none => (drs, [])
            | some counterpart =>
              let (out, ids, errors) := ops.np drs .other false counterpart
              let counterpartKind := ((ids.head?.bind out.findById?).map (·.kind)).getD
                (counterpart.kindIn out .object)
              let kindErrors := match side with
                | .host => kindCheck .object (some counterpartKind)
                | .attachment => attachmentHostKindCheck word counterpartKind
              (out, errors ++ kindErrors)
          (out, headErrors ++ counterpartErrors)
        | .in_ z => stepLocationWith ops drs z
        | .hasDesignation label holder =>
          let kind := drs.relativeKind.getD .object
          let (out, errors) := match holder with
            | none => (drs, [])
            | some owner =>
              let (out, ids, errors) := ops.np drs .other false owner
              (out, errors ++ kindCheck .player ((ids.head?.bind out.findById?).map (·.kind)))
          (out, refuse (label.holder == some kind) (.designationHolder label kind) ++
            refuse (holder.isNone || label.possessorOk) (.designationPossessorFits label) ++ errors)
        | .inCombat relation (some cp) =>
          let (out, ids, errors) := ops.np drs .other false cp
          let head := drs.relativeKind.getD .object
          let counterpart := (ids.head?.bind out.findById?).map (·.kind)
          let either := Kind.join .object .player
          let headErrors := if relation == .attackedBy then
              refuse (Kind.lte head either) (.kindLte head either)
            else kindCheck .object (some head)
          let counterpartErrors := match counterpart with
            | none => []
            | some kind => if relation == .attackedBy || relation == .attackerOf then
                refuse (Kind.lte kind either) (.kindLte kind either)
              else kindCheck .object (some kind)
          (out, errors ++ headErrors ++ counterpartErrors)
        | .related _ (some cp) =>
          let (drs', _, errs) := ops.np drs .other false cp
          (drs', errs)
        | .shares _ _ src =>
          let (drs', ids, errs) := ops.np drs .other false src
          let found := ids.head?.bind drs'.findById?
          (drs', errs ++ kindCheck .object (found.map (·.kind)))
        | .named .chosen =>
          let count := countChoice (.quality .cardName) drs
          (drs, refuse (count == 1) (.choiceRef .theChoice (.quality .cardName) count))
        | .hasKeyword term => (drs, refuse term.known .knownKeywordTerm)
        | .abilityHead family => (drs, refuse family.known .knownKeywordTerm)
        | .counterKindOn src | .abilityOf src
        | .activatedBy src =>
          let (drs', _, errs) := ops.np drs .other false src
          (drs', errs)
        | .other =>
          let kind := drs.relativeKind.getD .object
          (drs, refuse (anyTargeted kind drs) (.anyTargeted kind))
        | .notChosen =>
          let kind := drs.relativeKind.getD .object
          (drs, refuse (countParts kind drs != 0 || drs.choiceWasPossible kind) (.choiceInScope kind))
        | .otherThan anchor =>
          let (out, _, errors) := ops.np drs .other false anchor
          (out, errors ++ refuse (anchor.anchorPhrase && anchor.plur == .one) .complementAnchor)
        | .targets src _ =>
          let (out, ids, errors) := ops.np drs .other false src
          let targetKind := (ids.head?.bind out.findById?).map (·.kind) |>.getD .object
          let headKind := drs.relativeKind.getD .object
          (out, errors ++ refuse targetKind.targetable (.targetable targetKind) ++
            refuse headKind.targeter (.targeter headKind))
        | .coinCameUp _ =>
          let kind := drs.relativeKind.getD .object
          (drs, refuse (coinFlipInScope drs) .coinFlipInScope ++
            refuse (Kind.lte kind (.join .object .player)) (.kindLte kind (.join .object .player)))
        | .and ps =>
          let headKind := p.kind? <|> drs.relativeKind
          let seed := relativeSeed (headKind.getD .object) p
          let (out, errs) := ps.foldl (fun (d, es) q =>
            let d := { d with relativeKind := headKind }
            let (d', e') := match q with
              | .did clause => stepRelativeDidWith ops d seed clause
              | q => ops.pred d q
            (d', es ++ e')) (drs, [])
          ({ out with relativeKind := drs.relativeKind },
            errs ++ refuse (otherAnchorOkWith ops drs (headKind.getD .object) ps) .otherAnchored)
        | .or ps =>
          let headKind := p.kind? <|> drs.relativeKind
          let (nextId, errors) := ps.foldl (fun (nextId, errors) alternative =>
            let (branch, branchErrors) := ops.pred
              { drs with nextId, relativeKind := alternative.kind? <|> headKind } alternative
            (branch.nextId, errors ++ branchErrors)) (drs.nextId, [])
          let distinctKinds := (ps.filterMap (·.kind?)).eraseDups
          let shapeErrors := if distinctKinds.length > 1 then [] else
            refuse (!ps.isEmpty) .nonEmpty ++ refuse (coordinableAll ps) .coordinableDisjuncts
          ({ drs with nextId }, errors ++ shapeErrors)
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
          let ref := bindFor detP.det detP.plur (pred.kindOr .object) pred
          let rid := drs2.nextId
          let drs3 := drs2.push ref role inArgSlot
          let targetErrors := if detP.det == .target then
            refuse (pred.kindOr .object).targetable (.targetable (pred.kindOr .object)) else []
          (drs3, [rid], e1 ++ e2 ++ targetErrors ++ refuse (detP.det != .the || pred.uniquifies) .uniquifying)
        | .and ns =>
          ns.foldl (fun (d, ids, es) n =>
            let (d', ids', e') := ops.np d role inArgSlot n
            (d', ids ++ ids', es ++ e')) (drs, [], refuse (ns.length ≥ 2) .atLeastTwo)
        | .or ns => stepNounAlternativesWith ops drs role inArgSlot ns
        | .eachOf grp =>
          let (drs1, ids, e1) := ops.np drs .other false grp
          let payload := (ids.head?.bind drs1.findById?).map (·.payload) |>.getD (.object [] none none none none)
          let ref : Referent := { det := .each, plur := .one, payload }
          let rid := drs1.nextId
          let drs2 := drs1.push ref role inArgSlot
          (drs2, [rid], e1 ++ refuse (grp.plur == .many || grp.det == some .each) .plural ++ refuse grp.groupMention .groupMention)
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
          let drs4 := grpIds.foldl (fun context id =>
            context.recordSelection ref.kind (some id) false) drs4
          (drs4, [rid], e1 ++ e2 ++ e3 ++ refuse grp.partitiveBase .partitiveBase)
        | .slice _ amt p =>
          let (drs1, e1) := ops.amt drs amt
          let (drs2, e2) := stepLocationWith ops drs1 p
          let ref : Referent := {
            det := .the, plur := np.plur,
            payload := slicePayload p.sort amt
          }
          let rid := drs2.nextId
          let drs3 := drs2.push ref role inArgSlot
          let ownerErrors := match p with
            | .zone .library (some owner) | .zone .graveyard (some owner) =>
              refuse owner.slicePossessorOk .slicePossessor
            | _ => []
          (drs3, [rid], e1 ++ e2 ++ ownerErrors)
        | .run _ stop p =>
          let (drs1, e1) := ops.pred drs stop
          let (drs2, e2) := stepLocationWith ops drs1 p
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
            payload := .object poolTy poolZone none none none
          }
          let rid := drs2.nextId
          let drs3 := drs2.push ref role inArgSlot
          (drs3, [rid], e1 ++ e2 ++ refuse (!roles.isEmpty) .nonEmpty)
        | .theRest k pl =>
          let ok := theRestFits k pl drs
          let zn := zoneOfGroup k drs
          let ty := tyOfGroup k drs
          let payload := if k == .player then .player false else match zn, ty with | z, t => .object t z none none none
          let ref : Referent := { det := .the, plur := pl, payload }
          let rid := drs.nextId
          let drs' := drs.push ref role inArgSlot
          (drs', [rid], if ok then [] else [.theRestFits k])
        | .possessorOf axis subj =>
          let (read, ids, e1) := ops.np { drs with characteristicRead := true } .other false subj
          let drs1 := { read with characteristicRead := drs.characteristicRead }
          let kind := subj.selectedKind drs1 ids
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
          (drs2, [rid], e1 ++ facet.localChecks true)
        | .headed head subj =>
          let (out, ids, errs) := ops.np drs role inArgSlot subj
          let refined := ids.foldl (fun d id => d.updateById id fun r =>
            let payload := match r.payload with
              | .object ty zone stamp origin size =>
                match head with
                | .type t | .subtype (.of t _) =>
                  .object (mergeTypes ty [t])
                    (zone.orElse (fun _ => if t.permanent then some .battlefield else none))
                    stamp origin size
                | .ability => .ability origin (some .stack)
                | .spell | .subtype (.spell _) => .object ty (some .stack) stamp origin size
                | .token => .object ty (some .battlefield) stamp (some .token) size
                | .permanent => .object ty (some .battlefield) stamp origin size
                | .emblem => .object ty (some .command) stamp origin size
                | _ => r.payload
              | payload => payload
            { r with payload }) out
          (refined, ids, errs ++ kindCheck .object ((ids.head?.bind out.findById?).map (·.kind)) ++
            refuse (subj.ascribableIn drs) .ascribable ++ refuse head.isHead .headNoun)
        | .agree _ _ subj =>
          let (drs1, ids, errs) := ops.np drs role inArgSlot subj
          (drs1, ids, errs ++ refuse (subj.plur == .many) .plural ++
            refuse (subj.det == some .count || subj.det == some .target) .countedMention)
        | .becomes subj =>
          let (out, ids, errors) := ops.np drs role inArgSlot subj
          let source := ids.head?.bind out.findById?
          let errors := errors ++ kindCheck .object (source.map (·.kind))
          let out := ids.foldl (fun d id => d.updateById id fun r =>
            { r with payload := match r.payload with
              | .object ty _ stamp origin size =>
                .object ty (some .battlefield) stamp origin size
              | payload => payload }) out
          (out, ids, errors)
        | .counterpart _ subj =>
          let (drs1, _, e1) := ops.np drs .other false subj
          let ref : Referent := {
            det := .the, plur := .one,
            payload := .object [] (some .battlefield) none none none
          }
          let rid := drs1.nextId
          let drs2 := drs1.push ref role inArgSlot
          (drs2, [rid], e1)
        | .attachHost attachment word =>
          let payload := if word.kind == .player then Payload.player false
            else Payload.object word.attachHostTy word.attachHostZone none none none
          let ref : Referent := { det := .the, plur := .one, payload, attachmentHost := true }
          let rid := drs.nextId
          let drs' := drs.push ref role inArgSlot
          (drs', [rid], refuse ((attachment == .enchanted && !word.refined) ||
            Kind.lte word.kind .object) (.kindLte word.kind .object))
        | .theGrantor head =>
          let ref : Referent := {
            det := .the, plur := .one,
            payload := .object [] (some .battlefield) none none none
          }
          let rid := drs.nextId
          let drs' := drs.push ref role inArgSlot
          (drs', [rid], refuse head.isHead .headNoun)
        | .designated d whose =>
          let (drs1, ownerIds, e1) := ops.np drs .other false whose
          let ownerKind := (ownerIds.head?.bind drs1.findById?).map (·.kind)
          let e1 := e1 ++ kindCheck .player ownerKind ++ refuse d.heldByItsCard (.designationScope d)
          let ref := kindBinding .object
          let rid := drs1.nextId
          let drs2 := drs1.push ref role inArgSlot
          (drs2, [rid], e1)
        | .you =>
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
          let ref : Referent := { det := .the, plur := .one, payload := .object [] none none none none }
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
          let actual := match subj.selectionKindOverride? with
            | some kind => some kind
            | none => (refs.head?).map (·.kind)
          let plural := subj.det == some .each || subj.plur != .one || refs.any (fun r => r.plur != .one)
          let axisErrors := match axis with
            | .counter kind => refuse kind.known .knownCounter
            | .boundCounter =>
              let n := countChoice (.quality .counterKind) drs
              refuse (n == 1) (.choiceRef .theChoice (.quality .counterKind) n)
            | _ => []
          let kindErrors := match axis, actual with
            | .boundCounter, some k => refuse (counterHolderKind k) .counterHolderKind
            | .boundCounter, none => []
            | _, _ => kindCheck axis.scope actual
          ({ out with characteristicRead := drs.characteristicRead },
            axisErrors ++ errs ++ kindErrors ++ refuse (!plural) .singular)
        | .paid facet subj =>
          let (out, _, errors) := ops.np drs .other false subj
          (out, errors ++ facet.localChecks false)
        | .aggregate _ axis subj =>
          let (out, ids, errors) := ops.np drs .other false subj
          let kind := ((ids.head?.bind out.findById?).map (·.kind)).getD (subj.kindIn out .object)
          let plural := subj.det == some .each || subj.plur == .many ||
            (ids.filterMap out.findById?).any (fun ref => ref.plur == .many)
          (out, errors ++ refuse (axis.scope == kind) (.projScope kind) ++
            refuse (plural || axis == .chapter) .plural)
        | .countOf subj =>
          let (out, _, errors) := ops.np drs .other false subj
          (out, errors ++ refuse (subj.plur == .many) .plural ++
            refuse subj.countableGroup .countableGroup)
        | .greatestStoredMatch subj =>
          let (out, _, errors) := ops.np drs .other false subj
          (out, errors)
        | .distinctCount axis subj =>
          let (out, _, errs) := ops.np drs .other false subj
          (out, errs ++ refuse axis.ok .kindAxisSort)
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
          if s == .rollResult then
            -- A dice batch has one numeric result read, without publishing a second outcome.
            let actual := countOutcomes .rollResult drs
            let n := if actual == 0 then countOutcomes .diceRolled drs else actual
            (drs, if n == 1 then [] else [.outcomeInScope .rollResult n])
          else if outcomeInScope s drs then (drs, [])
          else (drs, [.outcomeInScope s (countOutcomes s drs)])
        | .coinsShowing _ => (drs, if coinFlipInScope drs then [] else [.coinFlipInScope])
        | .groupSize =>
          let n := match drs.groupSizeScope with
            | some ids => (ids.eraseDups.filterMap drs.findById?).filter
                (fun ref => !ref.plur.isOne) |>.length
            | none => countManysAny drs
          (drs, if n == 1 then [] else [.groupSizeInScope n])
        | .theDifference =>
          let n := countDifferences drs
          (drs, if n == 1 then [] else [.differenceInScope n])
        | .letter l =>
          let drs' := (introducedLetters l drs).foldl (fun d r => d.push r .other false) drs
          (drs', [])
        | .arith op l r =>
          let (drs1, e1) := ops.amt drs l
          let (drs2, e2) := ops.amt drs1 r
          (drs2, e1 ++ e2 ++ refuse (op != .times || l.nonZero) .amtNonZero)
        | .devotion pl c optSec =>
          let (drs1, ids, e1) := ops.np drs .other false pl
          let e1 := e1 ++ kindCheck .player ((ids.head?.bind drs1.findById?).map (·.kind)) ++
            refuse (pl.plur == .one && (ids.filterMap drs1.findById?).all (·.plur.isOne)) .singular
          let (drs2, e2) := stepColorTermWith ops drs1 c
          let (drs3, e3) := match optSec with | some d => stepColorTermWith ops drs2 d | none => (drs2, [])
          (drs3, e1 ++ e2 ++ e3)
        | .half _ inner => ops.amt drs inner
        | .aggregateOver _ dom body =>
          let (drs1, e1) := ops.pred drs dom
          let k := dom.kindOr .object
          let drs2 := drs1.push (bindFor .the .one k dom) .other false
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
          let (drs', ids, errs) := ops.np drs .other false who
          (drs', errs ++ kindCheck .player ((ids.head?.bind drs'.findById?).map (·.kind)))
        | .duringPart _ none => (drs, [])
        | .exists_ subj =>
          let (out, _, errors) := ops.np drs .subj true subj
          (out, errors ++ refuse subj.existentialMention .existentialMention)
        | .did clause =>
          let (out, errs) := stepDidClauseWith ops
            { drs with relativeBinder := none, relativeUses := 0 } clause
          ({ out with relativeBinder := drs.relativeBinder, relativeUses := drs.relativeUses }, errs)
        | .gameHas designation => (drs, refuse designation.gameWide (.designationScope designation))
        | .matches subj pred =>
          let (drs1, ids, e1) := ops.np drs .subj true subj
          let subjectKind := (ids.head?.bind drs1.findById?).map (·.kind)
          let (drs2, e2) := ops.pred { drs1 with relativeKind := pred.kind? <|> subjectKind } pred
          let refined := ids.foldl (fun d id => d.updateById id (markTy pred.seedTy)) drs2
          let kindErrors := match pred.kind?, ids.head?.bind drs1.findById? with
            | some expected, some r => kindCheck expected (some r.kind)
            | _, _ => []
          -- A player group ("your opponents") is the definite set of those players.
          let subjectOk := subj.playerGroupMention || ids.all fun id =>
            id < drs.nextId || (drs1.findById? id).any (fun ref => ref.det == .the) ||
              drs1.scopeStack.head?.any (fun frame => frame.callerReferentIds.contains id)
          (refined, e1 ++ e2 ++ kindErrors ++ refuse subjectOk .testSubject ++
            refuse pred.says .predSays)
        | .compareAmt l _ r =>
          let (drs1, e1) := ops.amt drs l
          let (drs2, e2) := ops.amt drs1 r
          (drs2.push differenceB .other false, e1 ++ e2 ++ refuse l.read .readAmount)
        | .dealtThisWay pred =>
          let (drs1, e1) := ops.pred drs pred
          (drs1, e1 ++ (if damageDealtInScope drs then [] else [.damageDealtInScope]))
        | .choseThisWay who pred =>
          let kind := pred.kindOr .object
          let (drs1, ids, e1) := ops.np drs .subj true who
          let (drs2, e2) := ops.pred drs1 pred
          (drs2, e1 ++ kindCheck .player ((ids.head?.bind drs1.findById?).map (·.kind)) ++
            e2 ++ refuse (countParts kind drs != 0 || drs.choiceWasPossible kind)
              (.choiceInScope kind))
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
          let actual := countOutcomes .rollResult drs
          let n := if actual == 0 then countOutcomes .diceRolled drs else actual
          (drs1, e1 ++ (if n == 1 then [] else [.outcomeInScope .rollResult n]))
        | .rolledDoubles =>
          let n := countOutcomes .rollResult drs
          (drs, if n == 1 then [] else [.outcomeInScope .rollResult n])
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
          match c with
          | .or _ =>
            let (branches, errors) := cs.foldl (fun (branches, errors) part =>
              let (out, errs) := ops.cond (nextClause drs) part
              (branches ++ [out], errors ++ errs)) ([], shapeErrors)
            (commonHeaderContext drs branches, errors)
          | _ => cs.foldl (fun (d, es) part =>
              let (out, errs) := ops.cond (nextClause d) part
              (out, es ++ errs)) (drs, shapeErrors),

      ev := fun drs ev =>
        match ev with
        | .inCaller body =>
          let (callerDrs, errs) := ops.ev (enterCaller drs) body
          (leaveCaller drs callerDrs, errs)
        | .does subj action =>
          let headErr := (if action.eventHeadOk then [] else [.eventHead]) ++ eventActionRefusals action
          let (drs1, e1) := stepSubjectBeforeWith ops drs subj
          let actorErrors := match action.scopeBody with
            | .draw _ => []
            | _ => match clauseActorKind action, actorShape? drs1 with
              | some expected, some shape => kindCheck expected (some shape.kind)
              | _, _ => []
          let (drs2, e2) := ops.inst .postEvent subj.patientRole { drs1 with actionPattern := true } action
          let (drs3, e3) := stepSubjectAfterWith ops drs2 subj (clauseActorKind action) (match action.scopeBody with | .target _ => true | _ => false)
          ({ drs3 with actionPattern := drs.actionPattern }, headErr ++ e1 ++ actorErrors ++ e2 ++ e3 ++ eventVoiceChecks drs subj action)
        | .beginningOf _ whose turn | .endOf _ whose turn =>
          let (drs1, errors) := match whose with
            | none => (drs, [])
            | some owner =>
              let (out, _, errors) := ops.np drs .subj false owner
              (out, errors ++ turnOwnerKindRefusals owner)
          let (drs2, turnErrors) := stepTurnRefWith ops drs1 turn
          (drs2, errors ++ turnErrors)
        | .chapterMark ns => (drs, refuse (chapterMarksOk ns) .chapterMarks)
        | .stateBasedActionsChecked | .now => (drs, [])
        | .stateHolds c => ops.cond drs c
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
          let defaultsToSource := actPermanentActorOf verb && actor.all (·.isYou)
          let (inner, sourceErrors) := if defaultsToSource then
            let (out, ids, errors) := ops.np drs .subj true (.headed .permanent .this)
            let shape : ActorShape := {
              kind := .object, isYou := false, ascribable := true, refIds := ids }
            (out.push { det := .the, plur := .one, payload := .actor shape } .other false, errors)
            else (drs, [])
          let deedErrors := refuse (knownAct verb) (.knownAct verb)
          let priorMentions := inner.pendingRoles.length
          let (drs1, e1) := ops.inst phase patientRole inner body
          let bodyMentions := if drs1.currentClause == inner.currentClause then
              drs1.pendingRoles.take (drs1.pendingRoles.length - priorMentions)
            else drs1.pendingRoles
          let actedIds := bodyMentions.filterMap fun (id, role) =>
            if role == patientRole then some id else none
          let drs1 := if phase == .postEvent then applyInPlaceStamp drs1 actedIds (some verb) else drs1
          let (drs2, e2) := match becomes with | some p => ops.pred drs1 p | none => (drs1, [])
          let (drs3, e3) := match locus with | some z => stepLocationWith ops drs2 z | none => (drs2, [])
          let out := if defaultsToSource then dropActorFrame drs3 else drs3
          (out, sourceErrors ++ deedErrors ++ e1 ++ e2 ++ e3)
        | .instruct subj body =>
          let tracked := { drs with
            outerReferentIds := (drs.referents.map (·.id)) :: drs.outerReferentIds }
          let (drs1, refIds, e1) := ops.np tracked .subj true subj
          let shape : ActorShape := {
            kind := match subj.selectionKindOverride? with
              | some kind => kind
              | none => (refIds.filterMap drs1.findById?).foldl
                (fun kind ref => match kind with
                  | none => some ref.kind
                  | some k => some (joinKinds k ref.kind)) none |>.getD (subj.kindIn drs1 .player),
            isYou := subj.isYou,
            ascribable := false,
            refIds := refIds
          }
          let handoffErrors := refuse (shape.kind == .player || shape.kind == .object) .handoffPerformer
          let memberWise := subj.det == some .each || (subj.plur == .many && body.performed)
          let memberDrs := if memberWise then
            { drs1 with referents := drs1.referents.map fun r =>
                if refIds.contains r.id then { r with plur := .one } else r }
            else drs1
          let framePlur := if subj.det == some .each then Plurality.many else subj.plur
          let drs2 := memberDrs.push { det := .the, plur := framePlur, payload := .actor shape } .other false
          let (drs3, e2) := ops.inst phase .obj drs2 body
          let outerIds := drs3.outerReferentIds.headD []
          let currentSubjectIds := (actorShape? drs3).map (·.refIds) |>.getD refIds
          let out := dropActorFrame drs3
          let out := { out with outerReferentIds := out.outerReferentIds.drop 1 }
          let out := if memberWise then
            { out with referents := out.referents.map fun r =>
                if currentSubjectIds.contains r.id then { r with plur := .many } else r }
            else out
          let distributes := subj.det == some .each || subj.plur == .many
          let out := if distributes then
            { out with referents := out.referents.map fun r =>
                if !outerIds.contains r.id && r.kind != .outcome
                  then pluralizeBinding r else r }
            else out
          (out, e1 ++ handoffErrors ++ e2)
        | .dealDamage amt recipient _ =>
          let (drs1, e1) := ops.amt drs amt
          let (drs2, ids, e2) := ops.np drs1 patientRole true recipient
          let kind := (ids.head?.bind drs2.findById?).map (·.kind)
            |>.getD (recipient.kindIn drs2 .object)
          let outcome := if phase == .preEvent && !drs.actionPattern then
            OutcomeSort.namedNumber else .damageDealt
          let recipients := if phase == .postEvent && recipient.det == some .each then
              ids.foldl (fun context id => context.updateById id
                (fun ref => { ref with plur := .many })) drs2
            else drs2
          let drs3 := recipients.push (outcomeB outcome) .other false
          (drs3, e1 ++ e2 ++ refuse recipient.perMemberOk .perMember ++
            refuse (Kind.lte kind (.join .object .player)) (.kindLte kind (.join .object .player)))
        | .markDamage subj amt | .skipUntap subj amt =>
          let (drs1, _, e1) := ops.np drs patientRole true subj
          let (drs2, e2) := ops.amt drs1 amt
          (drs2, e1 ++ e2)
        | .skipPart _ cnt => ops.amt drs cnt
        | .addTurnPart part _ cnt _ =>
          let (out, errors) := ops.amt drs cnt
          let actorErrors := match actorShape? drs with
            | some actor => kindCheck .player (some actor.kind)
            | none => []
          let out := if part == .turn then out.push turnRefB .other false else out
          (out, errors ++ actorErrors)
        | .turnFaceDown subj _ =>
          let (out, ids, errs) := ops.np drs patientRole true subj
          (out, errs ++ kindCheck .object ((ids.head?.bind out.findById?).map (·.kind)))
        | .clearDamage subj | .turnOver subj =>
          let (out, ids, errors) := ops.np drs patientRole true subj
          let kind := (ids.head?.bind out.findById?).map (·.kind) |>.getD (subj.kindIn out .object)
          (out, errors ++ kindCheck .object (some kind))
        | .target subj | .changeControl subj | .chooseNewTargets subj =>
          let (drs1, refIds, e1) := ops.np drs patientRole true subj
          let drs2 := if phase == .postEvent then applyInPlaceStamp drs1 refIds inst.inferredDeed else drs1
          (drs2, e1)
        | .trigger => (drs, triggerActorRefusals drs)
        | .crew host | .saddle host | .fullyUnlock host =>
          let expected := match inst with | .fullyUnlock _ => Kind.player | _ => Kind.object
          let actorErrors := actionActorRefusals drs expected
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
        | .gameGains designation => (drs, refuse designation.gameWide (.designationScope designation))
        | .gainDesignation subj designation optDur =>
          let (drs1, refIds, e1) := ops.np drs patientRole true subj
          let source := refIds.head?.bind drs1.findById?
          let kind := source.map (·.kind) |>.getD (subj.kindIn drs1 .object)
          let e1 := e1 ++ refuse (designation.scope == some (.heldBy kind)) (.designationScope designation) ++
            refuse (designationHolderOk designation) (.designationHolder designation kind)
          let (drs2, e2) := match optDur with | some d => stepDurationWith ops drs1 d | none => (drs1, [])
          let drs3 := if phase == .postEvent then applyInPlaceStamp drs2 refIds inst.inferredDeed else drs2
          (drs3, e1 ++ e2)
        | .setStatus _ subj =>
          let (drs1, refIds, e1) := ops.np drs patientRole true subj
          let kind := (refIds.head?.bind drs1.findById?).map (·.kind) |>.getD (subj.kindIn drs1 .object)
          let drs2 := if phase == .postEvent then applyInPlaceStamp drs1 refIds inst.inferredDeed else drs1
          (drs2, e1 ++ kindCheck .object (some kind))
        | .combat subj upd =>
          let (drs1, _, e1) := ops.np drs patientRole true subj
          let (drs2, e2) := stepCombatUpdateWith ops drs1 upd
          (drs2, e1 ++ e2)
        | .attack (some defn) | .attackUnblocked (some defn) =>
          let (drs1, _, e1) := ops.np drs patientRole true defn
          (drs1, e1 ++ refuse (defn.plur == .one) .singular)
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
        | .attachment movement subj optHost =>
          let (drs1, ids, e1) := ops.np drs patientRole true subj
          let subjectKind := ((ids.head?.bind drs1.findById?).map (·.kind)).getD
            (subj.kindIn drs1 .object)
          let (drs2, e2) := match optHost with
            | some host =>
              let (out, ids, errors) := ops.np drs1 .other true host
              let hostKind := ((ids.head?.bind out.findById?).map (·.kind)).getD
                (host.kindIn out .object)
              let hostErrors := match movement with
                | .attached => refuse (Kind.lte hostKind (.join .object .player))
                    (.kindLte hostKind (.join .object .player))
                | .unattached => kindCheck .object (some hostKind)
              (out, errors ++ hostErrors)
            | none => (drs1, [])
          (drs2, e1 ++ kindCheck .object (some subjectKind) ++ e2)
        | .unlock d => stepDoorWith ops drs d
        | .commitCrime =>
          (drs, refuse (!drs.actionPattern || actorPlurality drs == .one) .singular)
        | .conclude .. | .restartGame | .revealChoices _ => (drs, [])
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
        | .relate subj relation cp =>
          let (drs1, ids, e1) := ops.np drs patientRole true subj
          let (drs2, cpIds, e2) := ops.np drs1 .other true cp
          let sourceKind := (ids.head?.bind drs1.findById?).map (·.kind)
          let counterpartKind := (cpIds.head?.bind drs2.findById?).map (·.kind)
            |>.getD (cp.kindIn drs2 .object)
          (drs2, e1 ++ e2 ++ kindCheck .object sourceKind ++
            kindCheck .object (some counterpartKind) ++ refuse relation.known (.knownRelation relation))
        | .merge subj cp _ =>
          let (drs1, ids, e1) := ops.np drs patientRole true subj
          let (drs2, cpIds, e2) := ops.np drs1 .other true cp
          (drs2, e1 ++ e2 ++ kindCheck .object ((ids.head?.bind drs1.findById?).map (·.kind)) ++
            kindCheck .object ((cpIds.head?.bind drs2.findById?).map (·.kind)) ++
            refuse (subj.plur == .one && cp.plur == .one) .singular)
        | .copyTargets subj cp =>
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
          let drs2 := if phase == .postEvent && chosen.det == some .target then
            ids.foldl (fun context id => context.updateById id
              (fun ref => { ref with announcedTargetResult := true })) drs2
            else drs2
          let drs2 := if phase == .postEvent then
            ids.foldl (fun context id => match context.findById? id with
              | some ref => context.recordSelection ref.kind none true
              | none => context) drs2
            else drs2
          let (drs3, e3) := match optWhen with | some t => stepTimingWith ops drs2 t | none => (drs2, [])
          (drs3, e1 ++ e2 ++ refuse chosen.agentChoosable .choiceClause ++
            refuse (first.isNone || actorPlurality drs == .many) .choiceOrder ++ e3)
        | .vote first _ ballot times =>
          let (drs1, e1) := match first with
            | some f => let (d, _, e) := ops.np drs .subj true f; (d, e)
            | none => (drs, [])
          let (drs1, e1) := match times with
            | some q => let (d, e) := stepQuantityWith ops drs1 q; (d, e1 ++ e)
            | none => (drs1, e1)
          let (drs2, e2) := stepBallotWith ops drs1 ballot
          let drs3 := drs2.push (outcomeB .voteHeld) .other false
          (drs3, e1 ++ e2 ++ refuse (first.isNone || actorPlurality drs == .many) .choiceOrder)
        | .move subj from_ to _ optCause riders =>
          let historicalOrigin := match from_.positiveZones? with
            | some zones => zones.any (fun zone => zone.sort == some .battlefield)
            | none => false
          let readContext := if drs.actionPattern && historicalOrigin then
            { drs with historicalMoveFrom := some .battlefield } else drs
          let (selected, refIds, e1) := ops.np readContext patientRole true subj
          let drs1 := { selected with historicalMoveFrom := drs.historicalMoveFrom }
          let actualKind := (refIds.head?.bind drs1.findById?).map (·.kind)
          let moveKind := Kind.join .object .pile
          let e1 := e1 ++ actualKind.toList.flatMap (fun kind =>
            refuse (Kind.lte kind moveKind) (.kindLte kind moveKind))
          let (drs2, e2) := stepLocationWith ops drs1 from_
          let (drs3, e3) := match to with | some z => stepLocationWith ops drs2 z | none => (drs2, [])
          let (drs4, e4) := match optCause with | some c => stepCausingWith ops drs3 c | none => (drs3, [])
          let (drs5, e5) := stepMoveRiderListWith ops drs4 riders
          let destZone := to.bind (·.sort)
          let drs6 := if phase == .postEvent then
            let moved := applyZoneMove drs5 refIds inst.inferredDeed destZone
            match subj.restContext? drs with
            | some (kind, plurality, source) =>
              if theRestFits kind plurality source then moved.consumeRest source kind
              else moved
            | none => moved
          else drs5
          (drs6, e1 ++ e2 ++ e3 ++ e4 ++ e5)
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
          let drs2 := delta.lifeOutcomes.foldl (fun d out => d.push (outcomeB out) .other false) drs1
          (drs2, e1)
        | .exchange ex => stepExchangedWith ops drs ex
        | .addMana amt prod riders =>
          let (drs1, e1) := ops.amt drs amt
          let (drs2, e2) := stepProducedManaWith ops drs1 prod
          let withOutcome := drs2.push (outcomeB .manaAdded) .other false
          let (drs3, e3) := stepManaRiderListWith ops withOutcome riders
          (drs3, e1 ++ e2 ++ e3)
        | .draw quantity =>
          let actorErrors := match actorShape? drs with
            | some shape => kindCheck .player (some shape.kind)
            | none => []
          let (drs1, e1) := stepQuantityWith ops drs quantity
          -- "Up to N": how many fewer than N were drawn is an outcome a later line may read
          -- ("for each card less than two they drew this way").
          let drs1 := match quantity with
            | .upToOf _ => drs1.push (outcomeB .ceilingShortfall) .other false
            | _ => drs1
          let drs2 := if phase == .postEvent then
            let cardRef : Referent := {
              det := .the, plur := quantity.plur,
              payload := .object [] (some .hand) (mkStamp (some (.core .draw)) (some .hand) false) none none
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
          | .zone z => stepLocationWith ops drs z
          | .choice _ => (drs, [])
        | .search sc q p =>
          let (drs1, e1) := stepSearchLocationWith ops drs sc
          let (drs2, e2) := stepQuantityWith ops drs1 q
          let (drs3, e3) := ops.pred drs2 p
          let scZone := sc.sort
          let drs4 := if phase == .postEvent then
            let foundRef : Referent := {
              det := .a, plur := q.plur,
              librarySearchResult := scZone == some .library,
              payload := .object p.seedTy scZone (mkStamp (some (.action "search")) none false) none none
            }
            drs3.push foundRef patientRole true
          else drs3
          (drs4, e1 ++ e2 ++ e3)
        | .shuffle pile =>
          let (drs1, e1) := stepLocationWith ops drs pile
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
          let drs4 := if drs.actionPattern then
            if batch == .many then drs2.push (outcomeB .diceRolled) .other false
            else drs2.push (outcomeB .rollResult) .other false
          else drs2.push (outcomeB .rollResult) .other false
          (drs4, e1 ++ e2 ++ sides.localChecks drs ++ watch.localChecks ++
            refuse count.nonZero .nonZeroQ)
        | .applyResultsTable rows =>
          let cnt := countOutcomes .rollResult drs
          let e0 := if cnt == 1 then [] else [.outcomeInScope .rollResult cnt]
          let (drs1, e1) := stepRollRowListWith ops drs rows
          (drs1, e0 ++ e1)
        | .ignoreOutcomes which =>
          let (drs1, e1) := stepIgnoredOutcomesWith ops drs which
          let allowed := match which with
            | .extreme _ => countOutcomes .rollResult drs1 == 1
            | _ => ignorableInScope drs1
          (drs1, e1 ++ refuse allowed .ignorableFor)
        | .shiftResult changes =>
          let (drs1, e1) := changes.foldl (fun (d, es) change =>
            let (d', e) := ops.amt d change.amount
            (d', es ++ e ++ refuse (!change.isSet) .shiftSets)) (drs, [])
          let cnt := countOutcomes .rollResult drs1
          let e2 := if cnt == 1 then [] else [.outcomeInScope .rollResult cnt]
          (drs1, e1 ++ refuse (!changes.isEmpty) .nonEmpty ++ e2)
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
          (drs2, e1 ++ e2 ++ refuse (!spec.writesDefining) .cdaGranted)
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
          else match spec with
            | .token .. => { drs3 with prospectiveTokenTypes := drs3.prospectiveTokenTypes ++ [sourceTypes] }
            | .emblem .. => drs3
          (drs4, e1 ++ e2 ++ e3)
        | .putCounters amt ks on batch _ =>
          let (drs1, e1) := ops.amt drs amt
          let (drs2, e2) := stepCounterKindSourceWith ops drs1 ks
          let (drs3, ids, e3) := ops.np drs2 patientRole true on
          let kind := ((ids.head?.bind drs3.findById?).map (·.kind)).getD (on.kindIn drs3 .object)
          let drs4 := if drs.actionPattern && batch != .many then drs3
            else drs3.push (outcomeB .countersPut) .other false
          (drs4, e1 ++ e2 ++ e3 ++ refuse on.perMemberOk .perMember ++
            refuse (counterHolderKind kind) .counterHolderKind)
        | .removeCounters optQ optKs from_ batch _ =>
          let (drs1, ids, e1) := ops.np drs patientRole true from_
          let (drs2, e2) := match optQ with | some q => stepQuantityWith ops drs1 q | none => (drs1, [])
          let (drs3, e3) := match optKs with | some ks => stepCounterKindSourceWith ops drs2 ks | none => (drs2, [])
          let kind := ((ids.head?.bind drs3.findById?).map (·.kind)).getD (from_.kindIn drs3 .object)
          let drs4 := if drs.actionPattern && batch != .many then drs3
            else drs3.push (outcomeB .countersRemoved) .other false
          (drs4, e1 ++ e2 ++ e3 ++ refuse (counterHolderKind kind) .counterHolderKind)
        | .distribute v amt among =>
          let (drs1, e1) := stepDividedVerbWith ops drs v
          let (drs2, e2) := ops.amt drs1 amt
          let (drs3, ids, e3) := ops.np drs2 patientRole true among
          let kind := (ids.head?.bind drs3.findById?).map (·.kind)
          let kindErrors := match v, kind with
            | .damage _, some kind =>
              let either := Kind.join .object .player
              refuse (Kind.lte kind either) (.kindLte kind either)
            | .counters _, _ => kindCheck .object kind
            | _, _ => []
          (drs3, e1 ++ e2 ++ e3 ++ refuse among.groupMention .groupMention ++ kindErrors)
        | .pay cost times outcome whose keyword =>
          let (drs1, e1) := match whose with
            | some w => let (d, _, e) := ops.np drs .subj true w; (d, e)
            | none => (drs, [])
          let (drs2, e2) := ops.cost drs1 cost
          let repeated := (Action.pay cost times outcome whose keyword).hasRepeatedPayment
          let out := if phase == .postEvent && repeated then
              drs2.push (outcomeB .repeatCount) .other false
            else drs2
          (out, e1 ++ e2)
        | .withContinuation policy body ifDid ifNot =>
          let (drs1, e1) := ops.inst phase patientRole (nextClause drs) body
          let (drs2, e2) := match ifDid with
            | some b => ops.inst phase .obj (nextClause drs1) b
            | none => (drs1, [])
          let (drs3, e3) := match ifNot with
            | some b => ops.inst phase .obj (nextClause drs) b
            | none => (drs, [])
          let policyErrors := match policy with
            | .optional => []
            | .required => refuse body.reflexEncloseUse.admitsReflex .reflexEnclosure ++
                refuse (ifDoneArmed ifDid ifNot) .ifDoneArmed
          let evidence := if policy == .optional then
              (drs2.selectionEvidence ++ drs3.selectionEvidence).filter (fun evidence =>
                evidence.scope == drs.selectionScope &&
                (evidence.chosen || evidence.sourceGroup.any drs.isVisible)) |>.eraseDups
            else drs.selectionEvidence
          let shortfalls := if policy == .optional then drs1.referents.filter (fun ref =>
              ref.id >= drs.nextId && (ref.payload == .outcome .ceilingShortfall ||
                (body.hasRepeatedPayment && ref.payload == .outcome .repeatCount)))
            else []
          ({ drs with
              referents := shortfalls ++ drs.referents
              nextId := max drs2.nextId drs3.nextId
              nextSelectionScope := max drs2.nextSelectionScope drs3.nextSelectionScope
              selectionEvidence := evidence }, e1 ++ e2 ++ e3 ++ policyErrors)
        | .limited inst limit =>
          let (out, errors) := ops.inst phase patientRole drs inst
          let (out', limitErrors) := stepUsageLimitWith ops out limit
          (out', errors ++ limitErrors)
        | .doIf cond thenInst elseInst =>
          -- The branch's targets are chosen as the spell or ability is put on the stack
          -- [CR#601.2c,115.1], before the condition is checked as it resolves [CR#608.2c]: the
          -- condition reads them back wherever it is printed ("counter target spell if its mana
          -- value is X"). The other branch reads the branch as announced, its targets and the
          -- amounts it names ("otherwise, each opponent loses that much life"), not what it did.
          -- The branch keeps the ids it announced its targets under: it is stepped again from
          -- the ids it took then, and the condition's referents take ids past every id the
          -- branch takes, so a later line reads the target the condition read.
          let (announced, _) := ops.inst .preEvent .obj drs thenInst
          let fresh := announced.referents.filter fun ref => !(drs.referents.any (·.id == ref.id))
          let targets := fresh.filter (·.det == .target)
          let targetIds := targets.map (·.id)
          let branchExtent := if targets.isEmpty then announced.nextId
            else (ops.inst phase .obj (nextClause drs) thenInst).1.nextId
          let withTargets := if targets.isEmpty then drs else
            nextClause { drs with
              referents := targets ++ drs.referents
              nextId := max announced.nextId branchExtent }
          let (drs1, e1) := ops.cond withTargets cond
          let branchStart := if targets.isEmpty then drs1.nextId else drs.nextId
          let (stepped, e2) := ops.inst phase .obj
            (nextClause { drs1 with
              referents := drs1.referents.filter (!targetIds.contains ·.id)
              nextId := branchStart }) thenInst
          let drs2 := { stepped with nextId := max stepped.nextId drs1.nextId }
          match elseInst with
          | none => (drs2, e1 ++ e2)
          | some el =>
            -- The other branch reads the targets and the amounts the branch names, nothing the
            -- branch would choose as it resolves.
            let amounts := fresh.filter fun ref => match ref.plur, ref.payload with
              | .one, .outcome s => s.isAmount
              | _, _ => false
            let (alternative, e3) := ops.inst phase .obj
              (nextClause { withTargets with
                referents := amounts ++ withTargets.referents
                nextId := max withTargets.nextId drs2.nextId }) el
            ({ withTargets with nextId := max drs2.nextId alternative.nextId }, e1 ++ e2 ++ e3)
        | .doForEach grp body =>
          let (read, grpIds, e1) := ops.np { drs with characteristicRead := true } .other false grp
          let drs1 := { read with characteristicRead := drs.characteristicRead }
          let pluralityErrors := refuse (grp.plur == .many || grp.det == some .each) .plural
          let group := grpIds.head?.bind drs1.findById?
          let drs1 := grpIds.foldl (fun state id =>
            state.updateById id (fun ref => { ref with plur := .many })) drs1
          let outerReferentIds := drs1.referents.map (·.id)
          let loopRef : Referent := {
            det := .each, plur := .one,
            payload := group.map (·.payload) |>.getD (.object [] none none none none)
          }
          let drs2 := { (enterSelectionScope drs1) with outerReferentIds := outerReferentIds :: drs1.outerReferentIds }
          let drs2 := drs2.push loopRef .subj true
          let (drs3, e2) := ops.inst phase .obj (nextClause drs2) body
          let outerIds := drs3.outerReferentIds.headD []
          let kept := drs3.referents.filter (fun ref => outerIds.contains ref.id)
          let keptIds := kept.map (·.id)
          let closedDrs : DRS := { drs3 with
            referents := kept
            outerReferentIds := drs3.outerReferentIds.drop 1
            pendingRoles := drs3.pendingRoles.filter (fun mention => keptIds.contains mention.1)
            resolvedIds := drs3.resolvedIds.filter keptIds.contains }
          let closed := publishLoopChoiceHistory (leaveSelectionScope drs1 closedDrs) drs3
          let targets := publishLoopTargetResults closed drs3 outerIds
          let output := publishLoopCreatedResults targets drs3 outerIds
          (output, e1 ++ pluralityErrors ++ e2)
        | .doForEachKind axis dom q body =>
          let (drs1, e1) := match dom with
            | some d => let (d', _, e) := ops.np drs .other false d; (d', e)
            | none => (drs, [])
          let outerReferentIds := drs1.referents.map (·.id)
          let drs2 := { (enterSelectionScope drs1) with outerReferentIds := outerReferentIds :: drs1.outerReferentIds }
          let drs2 := drs2.push { det := .each, plur := .one, payload := .quality q } .other false
          let (drs3, e2) := ops.inst phase .obj (nextClause drs2) body
          let outerIds := drs3.outerReferentIds.headD []
          let closedDrs : DRS := { drs3 with
            referents := drs3.referents.filter (fun ref => outerIds.contains ref.id)
            outerReferentIds := drs3.outerReferentIds.drop 1
            pendingRoles := drs3.pendingRoles.filter (fun mention => outerIds.contains mention.1)
            resolvedIds := drs3.resolvedIds.filter outerIds.contains }
          let closed := publishLoopChoiceHistory (leaveSelectionScope drs1 closedDrs) drs3
          let targets := publishLoopTargetResults closed drs3 outerIds
          let created := publishLoopCreatedResults targets drs3 outerIds
          let output := publishLoopOutcomes created drs3 outerIds
          (output, e1 ++ e2 ++ refuse (axis.sort == some q) .kindAxisSort ++
            refuse (dom.isSome || axis.closed) .kindDomainOk)
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
            let deferred := d.referents.filter fun ref =>
              ref.id >= drs.nextId && (match ref.payload with | .outcome _ => true | _ => false)
            let deferredIds := deferred.map (·.id)
            let announced := { d with
              referents := d.referents.filter (fun ref => !deferredIds.contains ref.id) }
            let (out, errors) := ops.inst phase .obj announced step
            let refs := deferred.foldr insertReferentById out.referents
            ({ out with referents := refs }, es ++ errors)) (drs, refuse (!steps.isEmpty) .nonEmpty)
        | .chooseModes q modes =>
          let (drs1, e1) := stepQuantityWith ops drs q
          let (drs2, e2) := stepModeListWith ops drs1 modes
          (drs2, e1 ++ refuse q.nonZero .nonZeroQ ++
            refuse (modes.length ≥ 2) .atLeastTwo ++ refuse (q.modesFit modes.length) .modesFit ++ e2)
        | .delay ev alts optDur body =>
          let (drs1, e1) := ops.ev drs ev
          let (_, eAlts) := stepGameEventListWith ops drs alts
          let (drs2, e2) := match optDur with | some d => stepDurationWith ops drs1 d | none => (drs1, [])
          let (drs3, e3) := ops.inst phase .obj (nextClause (settleTargets drs2)) body
          ({ drs with nextId := drs3.nextId }, e1 ++ eAlts ++ e2 ++ e3)
        | .replace repld repl =>
          let (drs1, e1) := ops.inst .preEvent patientRole drs repld
          let localDescriptions := drs1.prospectiveTokenTypes.drop drs.prospectiveTokenTypes.length
          let described := { drs1 with prospectiveTokenTypes := localDescriptions }
          let (drs2, e2) := stepReplacementWith ops (nextClause described) repl repld.isEntering
          ({ drs2 with prospectiveTokenTypes := drs.prospectiveTokenTypes }, e1 ++ e2)
        | .holdUntil inst ev =>
          let (drs1, e1) := ops.inst phase patientRole drs inst
          let (drs2, e2) := ops.ev drs1 ev
          (drs2, e1 ++ e2)
        | .triggerReflexively body none trig =>
          let (drs1, e1) := ops.inst phase patientRole drs body
          let (drs2, e2) := ops.inst .postEvent .obj (nextClause (settleTargets drs1)) trig
          ({ drs1 with nextId := drs2.nextId },
            e1 ++ e2 ++ refuse body.reflexEncloseUse.admitsReflex .reflexEnclosure)
        | .triggerReflexively body (some ev) trig =>
          let (drs1, e1) := ops.inst phase patientRole drs body
          let (drs2, e2) := ops.ev (nextClause drs1) ev
          let (drs3, e3) := ops.inst .postEvent .obj (nextClause (settleTargets drs2)) trig
          ({ drs1 with nextId := drs3.nextId },
            e1 ++ e2 ++ e3 ++ refuse body.thisWayOutcomeOk .thisWayOutcome)
        | .procedure _ subj pay res from_ of_ =>
          let (drs1, refIds, e1) := ops.np drs patientRole true subj
          let e1 := e1 ++ kindCheck .object ((refIds.head?.bind drs1.findById?).map (·.kind))
          let (drs2, e2) := stepPlayPaymentWith ops drs1 pay
          let (drs3, e3) := match res with | some p => ops.pred drs2 p | none => (drs2, [])
          let (drs4, e4) := match from_ with | some f => stepLocationWith ops drs3 f | none => (drs3, [])
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
        | .manaRetention subj held =>
          let (out, _, errors) := ops.np drs .subj true subj
          (out, errors ++ held.localChecks out)
        | .partSkip subj _ | .combatant subj =>
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
        | .characteristicChange subj edits =>
          let (drs1, ids, e1) := ops.np drs .subj true subj
          let (drs2, e2) := stepCharacteristicEditListWith ops drs1 edits
          (finishEachSubject subj ids drs2, e1 ++ e2 ++ refuse (!edits.isEmpty) .nonEmpty ++
            refuse (edits.all CharacteristicEdit.writesLine) .lineNonEmpty ++
            CharacteristicEdit.shapeAll edits)
        | .characteristicDefining subj edits =>
          let (drs1, _, e1) := ops.np drs .subj true subj
          -- A stat set names its number, which a later edit's "that number" reads back ("its
          -- toughness is equal to that number plus 1").
          let (drs2, e2) := edits.foldl (fun (d, es) edit =>
            let (d', e') := stepCharacteristicEditWith ops d edit
            let named := match edit with
              | .stat .sets _ _ => d'.push (outcomeB .namedNumber) .other false
              | _ => d'
            (named, es ++ e')) (drs1, [])
          (drs2, e1 ++ e2 ++ refuse (!edits.isEmpty) .nonEmpty ++
            refuse subj.selfDefinedOk .cdaSubject ++
            refuse (edits.all CharacteristicEdit.definesOk) .cdaEdit ++
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
          let (drs2, ids, e2) := ops.np drs1 .obj true subj
          (applyInPlaceStamp drs2 ids (some (.core .gainControl)), e1 ++ e2)
        | .modeChoice mc => stepModeChangeWith ops drs mc
        | .cant subj action mods =>
          let headErr := if action.deonticHeadOk then [] else [.eventHead]
          let (drs1, e1) := stepSubjectBeforeWith ops drs subj
          let (drs2, e2) := ops.inst .preEvent subj.patientRole { drs1 with actionPattern := true } action
          let (drs3, e3) := stepDeonticModsWith ops drs2 mods
          let (drs4, e4) := stepSubjectAfterWith ops drs3 subj
          (publishProhibitionSources drs drs4, headErr ++ e1 ++ e2 ++ e3 ++ e4)
        | .must subj action mods | .can subj action mods =>
          let headErr := if action.deonticHeadOk then [] else [.eventHead]
          let (drs1, e1) := stepSubjectBeforeWith ops drs subj
          let (drs2, e2) := ops.inst .preEvent subj.patientRole { drs1 with actionPattern := true } action
          let (drs3, e3) := stepDeonticModsWith ops drs2 mods
          let (drs4, e4) := stepSubjectAfterWith ops drs3 subj
          ({ drs4 with actionPattern := drs.actionPattern }, headErr ++ e1 ++ e2 ++ e3 ++ e4)
        | .gatedBy cost subj action mods =>
          let headErr := if action.deonticHeadOk then [] else [.eventHead]
          let (drs1, e1) := stepSubjectBeforeWith ops drs subj
          let (drs2, e2) := ops.inst .preEvent subj.patientRole { drs1 with actionPattern := true } action
          let (drs3, e3) := stepDeonticModsWith ops drs2 mods
          let payerId := drs3.nextId
          let payer := kindBinding .player
          let costContext := { (nextClause drs3) with
            actionPattern := drs.actionPattern
            groupSizeScope := (actorShape? drs3).map (fun (shape : ActorShape) => shape.refIds) }
          let (paid, costErrors) := ops.cost (costContext.push payer .other false) cost
          let withoutPayer := { paid with
            referents := paid.referents.filter (·.id != payerId)
            pendingRoles := paid.pendingRoles.filter (fun mention => mention.1 != payerId) }
          let (out, agentErrors) := stepSubjectAfterWith ops withoutPayer subj
          ({ out with actionPattern := drs.actionPattern, groupSizeScope := drs.groupSizeScope },
            headErr ++ e1 ++ e2 ++ e3 ++ costErrors ++ agentErrors)
        | .replace ev repl tm _ _ =>
          let (drs1, e1) := stepWouldClauseWith ops drs ev
          let (drs2, e2) := stepReplacementWith ops (nextClause drs1) repl ev.isEntering
          let (_, e3) := match tm with | some t => stepTimingWith ops drs2 t | none => (drs2, [])
          -- Nothing the `would` clause or the replacement introduces escapes but the letters
          -- they introduce: X has one value on the object, so "enters with X counters, where X is
          -- …" defines it after [CR#107.3i].
          let letters := drs2.visibleReferents.filter fun ref =>
            ref.id ≥ drs.nextId && (match ref.payload with | .letter _ => true | _ => false)
          ({ drs with referents := letters ++ drs.referents, nextId := Nat.max drs.nextId drs2.nextId },
            e1 ++ e2 ++ e3)
        | .damageResult r =>
          match r with
          | ⟨_, source, recipient, _, does⟩ =>
            let (drs1, e1) := ops.pred drs recipient
            let recipientKind := recipient.kindOr .object
            let recipientErrors := refuse (Kind.lte recipientKind (.join .object .player))
              (.kindLte recipientKind (.join .object .player))
            let resultContext := drs1.push (outcomeB .damageDealt) .other false
            let (drs2, e2) := stepSubjectBeforeWith ops (nextClause resultContext) source
            let (drs3, e3) := ops.inst .postEvent source.patientRole drs2 does
            let (_, e4) := stepSubjectAfterWith ops drs3 source
            (drs, e1 ++ recipientErrors ++ e2 ++ e3 ++ e4)
        | .damageRule _ da ds dop _ =>
          let (drs1, e1) := stepDamageAgentWith ops drs da
          let (drs2, e2) := stepDamageScopeWith ops drs1 ds
          let (drs3, e3) := stepDamageOpWith ops drs2 dop
          (drs3, e1 ++ e2 ++ e3)
        | .preventionBan _ (.described da ds) _ =>
          let (drs1, e1) := stepDamageAgentWith ops drs da
          let (drs2, e2) := stepDamageScopeWith ops drs1 ds
          (drs2, e1 ++ e2)
        | .preventionBan _ .thatDamage _ =>
          (drs, refuse (damageDealtInScope drs) .damageDealtInScope)
        | .additionalTriggers ev q =>
          let (drs1, e1) := ops.ev drs ev
          let (drs2, e2) := stepQuantityWith ops drs1 q
          (drs2, e1 ++ e2)
        | .conditional spec cond _ =>
          let (drs1, e1) := ops.spec drs spec
          let (drs2, e2) := ops.cond (nextClause drs1) cond
          (drs2, e1 ++ e2 ++ refuse (!spec.writesDefining) .cdaConditional)
        | .visibility _ player what =>
          let (drs1, _, e1) := ops.np drs .subj true player
          let (drs2, e2) := stepVisibleThingWith ops drs1 what
          (drs2, e1 ++ e2)
        | .conjunction optSubj parts =>
          let (drs1, e1) := match optSubj with
            | some sub => let (d, _, e) := ops.np drs .subj true sub; (d, e)
            | none => (drs, [])
          let (drs2, e2) := parts.foldl (fun (d, es) s =>
            let (d', e') := ops.spec (nextClause d) s
            let shapeErrors := match s with
              | .conjunction _ _ => [.notCoord]
              | _ => []
            (d', es ++ e' ++ shapeErrors)) (drs1, [])
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
          | .mana cost =>
            let out := if manaHasX cost && countLetter .x drs == 0 then
              drs.push (letterB .x) .other false else drs
            (out, [])
          | .loyaltySymbol change =>
            let (out, errors) := ops.amt drs change.amount
            (out, errors ++ refuse (!change.isSet) .shiftSets)
          | .tapSymbol | .untapSymbol | .itsManaCost => (drs, [])
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
        | .inCaller body => ops.ab (enterCaller drs) body
        | .keyword k params body =>
          checkKeywordParamsWith ops drs ((keywordSchemaFor k params).getD []) params ++
            (stepAbilityListWith ops (enterOwnScope drs) body).2 ++
            refuse (keywordParamFits k params) (.keywordParamFits k)
        | .activated cost inst tm _ guard activator =>
          let (drs0, e0) := match activator with
            | some a => let (d, _, e) := ops.np drs .subj true a; (d, e)
            | none => (drs, [])
          let (drs1, e1) := ops.cost (dropLetter .x drs0) cost
          let colonDrs := nextClause (publicOnly drs1)
          let (drs2, e2) := ops.inst .postEvent .obj colonDrs inst
          let (_, e3) := match tm with | some t => stepTimingWith ops drs2 t | none => (drs2, [])
          let (_, e4) := match guard with | some g => ops.cond drs2 g | none => (drs2, [])
          e0 ++ e1 ++ e2 ++ e3 ++ e4
        | .triggered ev alts while_ joins tm limit interv inst =>
          let (primary, primaryErrors) := stepTriggerHeaderWith ops drs ev alts while_ none
          let (headers, joinedErrors) := joins.foldl (fun (states, errors) header =>
            let (out, errs) := stepTriggerHeaderWith ops drs header.event
              header.alternatives header.while_ header.timing
            (states ++ [out], errors ++ errs)) ([primary], [])
          let drs1 := commonHeaderContext drs headers
          let (drs2, e2) := match interv with
            | some c => ops.cond (nextClause drs1) c
            | none => (drs1, [])
          let (drs3, e3) := ops.inst .postEvent .obj (nextClause drs2) inst
          let (_, e4) := match tm with | some t => stepTimingWith ops drs3 t | none => (drs3, [])
          let (_, limitErrors) := match limit with
            | some l => stepUsageLimitWith ops drs l
            | none => (drs, [])
          primaryErrors ++ joinedErrors ++ e2 ++ e3 ++ e4 ++ limitErrors
        | .static spec => (ops.spec drs spec).2
        | .spell tm inst =>
          let (drs1, e1) := match tm with | some t => stepTimingWith ops drs t | none => (drs, [])
          let (_, e2) := ops.inst .postEvent .obj drs1 inst
          e1 ++ e2
        | .mayBeginOnBattlefield => []
        | .thatAbility ref =>
          let count := countChoice (.quality .ability) drs
          refuse (ref.ok count) (.choiceRef ref (.quality .ability) count)
        | .alsoForKeywords inner keywords =>
          ops.ab drs inner ++ refuse inner.keywordExtendable .keywordExtendable ++
            refuse (keywordListOk inner keywords) .keywordListOk
        | .italicHead _ inner =>
          let plain := match inner with | .italicHead .. => false | _ => true
          ops.ab drs inner ++ refuse plain .notWordHeaded
        | .qualified inner tm _ guard _ =>
          let e1 := ops.ab drs inner
          let (_, e2) := match tm with | some t => stepTimingWith ops drs t | none => (drs, [])
          let (_, e3) := match guard with | some g => ops.cond drs g | none => (drs, [])
          e1 ++ e2 ++ e3
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

def stepAction (fuel : Nat) (phase : StepPhase) (patientRole : GramRole) (drs : DRS)
    (inst : Action) : DRS × List Refusal :=
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

def Action.check (drs : DRS) (i : Action) : List Refusal :=
  (stepAction 64 .postEvent .obj drs i).2

def StaticSpec.check (drs : DRS) (s : StaticSpec) : List Refusal :=
  (stepStaticSpec 64 drs s).2

def Ability.check (drs : DRS := DRS.ofList []) (a : Ability) : List Refusal :=
  checkAbility 64 drs a

def NounPhrase.checkAt (expected : Option Kind) (drs : DRS)
    (np : NounPhrase) : List Refusal :=
  np.check drs ++ expected.toList.flatMap (fun k => kindCheck k np.kind?)

def Predicate.checkAt (expected : Kind) (drs : DRS) (p : Predicate) : List Refusal :=
  p.check { drs with relativeKind := some expected } ++
    (match p.scopeBody with
      | .targets .. | .compare .. | .hasDesignation .. | .attachment .. => []
      | _ => kindCheck expected p.kind?)

/-! ## Clause and event checking -/

def DidClause.check (drs : DRS) (c : DidClause) : List Refusal :=
  (stepDidClause 64 drs c).2

def WouldClause.check (drs : DRS) (c : WouldClause) : List Refusal :=
  (stepWouldClause 64 drs c).2

def GameEvent.check (drs : DRS) (ev : GameEvent) : List Refusal :=
  (stepGameEvent 64 drs ev).2

def GameEvent.checkAll (drs : DRS) (evs : List GameEvent) : List Refusal :=
  (stepGameEventList 64 drs evs).2

/-! ## Event-derived `DRS` extension (`docs/decisions/semantics-v3.md` §3.5) -/

def DidClause.extend (drs : DRS) (c : DidClause) : DRS :=
  (stepDidClause 64 drs c).1

def WouldClause.extend (drs : DRS) (c : WouldClause) : DRS :=
  (stepWouldClause 64 drs c).1

def GameEvent.extend (drs : DRS) (ev : GameEvent) : DRS :=
  (stepGameEvent 64 drs ev).1

end Semantics
