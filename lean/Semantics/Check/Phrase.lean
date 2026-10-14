import Semantics.Phrase
import Semantics.Check.Events
import Semantics.Check.Keywords
import Semantics.Check.Refusal

/-!
# Semantics.Check.Phrase

Minimal core DRT and action-clause checker (`docs/decisions/semantics-v3.md` §1.2–§1.5, §3.2–§3.8).
-/

namespace Semantics

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

def Quantity.plur : Quantity → Plurality
  | .inCaller body => body.plur
  | .range (some 1) (some 1) => .one
  | _ => .many

def DetPhrase.plur : DetPhrase → Plurality
  | .target q | .count q _ => q.plur
  | .a _ | .each | .the => .one
  | .all | .bare => .many

def SliceCount.plur : SliceCount → Plurality
  | .counted q => q.plur
  | .whole => .many

def Pile.sort : Pile → Option Zone
  | .library _ => some .library
  | .graveyard _ => some .graveyard
  | .that => none
  | .exiledWith _ => some .exile

def ZoneExpr.sort : ZoneExpr → Option Zone
  | .inCaller body => body.sort
  | .zone z _ => some z
  | .place _ p => p.sort
  | .wherever | .anyZone => none

mutual
  def Predicate.kind? : Predicate → Option Kind
    | .inCaller body | .bracketed body | .not body => body.kind?
    | .anyPlayer | .opponent => some .player
    | .hasDesignation d _ => d.holder
    | .compare axes _ _ => (axes.head?).map (·.scope)
    | .compareOver dom _ _ _ => dom.kind?
    | .and ps => Predicate.kindOfAny ps
    | .or ps => Predicate.kindOfAll ps
    | .hasPossessor _ _ | .did _ | .other | .otherThan _ | .isSource => none
    | _ => some .object

  def Predicate.kindOfAny : List Predicate → Option Kind
    | [] => none
    | p :: ps =>
      match p.kind? with
      | some k => some k
      | none => Predicate.kindOfAny ps

  def Predicate.kindOfAll : List Predicate → Option Kind
    | [] => none
    | p :: ps =>
      match p.kind?, Predicate.kindOfAll ps with
      | some k, some k' => some (joinKinds k k')
      | some k, none => some k
      | none, k' => k'
end

def Predicate.kindOr (d : Kind) (p : Predicate) : Kind := p.kind?.getD d

def NounPhrase.kind? : NounPhrase → Option Kind
  | .inCaller body | .firstMention _ body | .laterMention _ body => body.kind?
  | .you | .actor | .playerGroup _ | .possessorOf _ _ | .combatPlayer _ => some .player
  | .designated d _ => d.holder
  | .described _ p => p.kind?
  | .pro r _ => some r.kind
  | .theRest k _ => some k
  | .paid f _ => some f.entityKind
  | _ => some .object

def NounPhrase.kindIn (drs : DRS) (d : Kind) : NounPhrase → Kind
  | .inCaller body => body.kindIn (enterCaller drs) d
  | .firstMention _ body | .laterMention _ body => body.kindIn drs d
  | .actor => (actorShape? drs).elim .player (·.kind)
  | .refl pl => ((firstReflexive pl drs).map (·.kind)).getD d
  | n => n.kind?.getD d

mutual
  def Predicate.seedTy : Predicate → List CardType
    | .inCaller body | .bracketed body => body.seedTy
    | .hasType t => [t]
    | .hasSubtype s => s.type.toList
    | .compareOver dom _ _ _ => dom.seedTy
    | .and ps => Predicate.seedTyAll ps
    | .or ps => Predicate.seedTyJoin ps
    | _ => []

  def Predicate.seedTyAll : List Predicate → List CardType
    | [] => []
    | p :: ps => mergeTypes p.seedTy (Predicate.seedTyAll ps)

  def Predicate.seedTyJoin : List Predicate → List CardType
    | [] => []
    | [p] => p.seedTy
    | p :: ps => commonTypes p.seedTy (Predicate.seedTyJoin ps)
end

mutual
  def Predicate.seedZone : Predicate → Option Zone
    | .inCaller body | .bracketed body => body.seedZone
    | .inZone z => z.sort
    | .hasType t => if t.permanent then some .battlefield else none
    | .hasSubtype s => if (s.type.map CardType.permanent).getD false then some .battlefield else none
    | .isToken | .inCombat .. | .attachment .. => some .battlefield
    | .isEmblem => some .command
    | .exiledWith _ => some .exile
    | .hasStatus s => if s.word then some .battlefield else none
    | .and ps => Predicate.seedZoneAll ps
    | _ => none

  def Predicate.seedZoneAll : List Predicate → Option Zone
    | [] => none
    | p :: ps => p.seedZone <|> Predicate.seedZoneAll ps
end

mutual
  def Predicate.seedsToken : Predicate → Bool
    | .inCaller body | .bracketed body => body.seedsToken
    | .isToken => true
    | .and ps => Predicate.seedsTokenAny ps
    | _ => false

  def Predicate.seedsTokenAny : List Predicate → Bool
    | [] => false
    | p :: ps => p.seedsToken || Predicate.seedsTokenAny ps
end

def NounPhrase.plur : NounPhrase → Plurality
  | .inCaller body | .firstMention _ body | .laterMention _ body => body.plur
  | .described d _ => d.plur
  | .someOf c _ _ => c.plur
  | .theRest _ pl | .pro _ pl | .refl pl => pl
  | .slice _ (.lit 1) _ => .one
  | .slice .. => .many
  | .and _ => .many
  | .or (n :: _) => n.plur
  | .playerGroup .allPlayers | .playerGroup .yourOpponents | .playerGroup .yourTeam => .many
  | _ => .one

def bindFor (_drs : DRS) (det : Determiner) (plur : Plurality) (k : Kind) (p : Predicate) : Referent :=
  match k with
  | .object =>
    { det, plur, payload := .object p.seedTy p.seedZone none
        (if p.seedsToken then some .token else none) none none }
  | .player => { det, plur, payload := .player false }
  | .quality q => { det, plur, payload := .quality q }
  | .letter l => { det, plur, payload := .letter l }
  | .turnRef => { det, plur, payload := .turnRef }
  | .pile => { det, plur, payload := .pile none none none }
  | _ => { det, plur, payload := .gap }

/-- Whether `reach` is a bare pronoun governed by Rule `D′` (`it`, `them`, `they`). -/
def Reach.isBarePronoun : Reach → Bool
  | .bare | .atSlot _ => true
  | _ => false

/-- Clear `r.idents` on departed referents and transition them to `destZone` stamped with `deed`
(`semantics-v3.md` §1.2, `[CR#400.7,400.7j]`). -/
def applyZoneMove (drs : DRS) (refIds : List ReferentId) (deed : Option Deed)
    (destZone : Option Zone) : DRS :=
  { drs with referents := drs.referents.map fun r =>
      if refIds.elem r.id then
        let newPayload := match r.payload with
          | .object ty oldZn oldProv orig sz poss =>
            let prov := match deed with
              | some v => mkStamp (some v) (destZone <|> oldZn) (oldZn == some .battlefield)
              | none => oldProv
            .object ty (destZone <|> oldZn) prov orig sz poss
          | other => other
        { r with
          idents := if destZone.isSome then [] else r.idents
          payload := newPayload }
      else r }

/-- Stamp `refIds` in place with `deed` without changing zone. -/
def applyInPlaceStamp (drs : DRS) (refIds : List ReferentId) (deed : Option Deed) : DRS :=
  match deed with
  | none => drs
  | some v =>
    refIds.foldl (fun d rid =>
      d.updateById rid fun r =>
        let newPayload := match r.payload with
          | .object ty zn _ orig sz poss =>
            .object ty zn (mkStamp (some v) zn (zn == some .battlefield)) orig sz poss
          | other => other
        { r with payload := newPayload }) drs

/-- Infer the canonical `Deed` of an instruction for `.actedOn` stamping. -/
def Instruction.inferredDeed : Instruction → Option Deed
  | .inCaller body | .bracketed body => body.inferredDeed
  | .ownScope (some d) _ => some d
  | .ownScope none body => body.inferredDeed
  | .enact verb _ _ _ => some verb
  | .move _ _ (some (.zone .exile _)) _ _ _ => some (.action "exile")
  | .move _ _ (some (.zone .graveyard _)) _ _ _ => some (.core .put)
  | .move _ _ (some (.zone .hand _)) _ _ _ => some (.core .return_)
  | .setStatus .tapped _ => some (.action "tap")
  | .setStatus .untapped _ => some (.action "untap")
  | .draw _ => some (.core .draw)
  | .createObject .. => some (.action "create")
  | .expose .reveal _ => some (.action "reveal")
  | _ => none

/-- Voice and `GramRole` assignment (`semantics-v3.md` §3.2):
- Active (`.actor np`): primary patient inside `action` gets `.obj`.
- Passive (`.any`, `.by np`): promotes primary patient inside `action` to `.subj`. -/
def Subject.patientRole : Subject → GramRole
  | .actor _ => .obj
  | .any | .by _ => .subj

/-! ## Core DRT Stepping and Checking -/

mutual
  def stepZoneScope (drs : DRS) (s : ZoneScope) : DRS × List Refusal :=
    match s with
    | .bare => (drs, [])
    | .possessedBy np =>
      let (drs', _, errs) := stepNounPhrase drs .other false np
      (drs', errs)
  termination_by structural s

  def stepPile (drs : DRS) (p : Pile) : DRS × List Refusal :=
    match p with
    | .library s | .graveyard s => stepZoneScope drs s
    | .that => (drs, [])
    | .exiledWith src =>
      let (drs', _, errs) := stepNounPhrase drs .other false src
      (drs', errs)
  termination_by structural p

  def stepZoneExpr (drs : DRS) (z : ZoneExpr) : DRS × List Refusal :=
    match z with
    | .inCaller body =>
      let (callerDrs, errs) := stepZoneExpr (enterCaller drs) body
      (leaveCaller drs callerDrs, errs)
    | .zone _ s => stepZoneScope drs s
    | .place _ p => stepPile drs p
    | .wherever | .anyZone => (drs, [])
  termination_by structural z

  def stepEventSource (drs : DRS) (src : EventSource) : DRS × List Refusal :=
    match src with
    | .anywhere => (drs, [])
    | .zones zs | .anywhereBut zs => stepZoneExprList drs zs

  def stepZoneExprList (drs : DRS) (zs : List ZoneExpr) : DRS × List Refusal :=
    match zs with
    | [] => (drs, [])
    | z :: rest =>
      let (drs1, e1) := stepZoneExpr drs z
      let (drs2, e2) := stepZoneExprList drs1 rest
      (drs2, e1 ++ e2)
  termination_by structural zs

  def stepPredicate (drs : DRS) (p : Predicate) : DRS × List Refusal :=
    match p with
    | .inCaller body =>
      let (callerDrs, errs) := stepPredicate (enterCaller drs) body
      (leaveCaller drs callerDrs, errs)
    | .bracketed body | .not body => stepPredicate drs body
    | .hasPossessor _ np =>
      let (drs', _, errs) := stepNounPhrase drs .other false np
      (drs', errs)
    | .did clause =>
      let subDrs := nextClause drs
      let whichRef : Referent := {
        det := .the, plur := .one, clause := subDrs.currentClause, role := .subj,
        payload := .object [] (some .battlefield) none none none none
      }
      let subDrs' := subDrs.push whichRef .subj true
      let (_, errs) := stepDidClause subDrs' clause
      (drs, errs)
    | .compare _ _ b => stepAmount drs b
    | .compareOver dom m _ b =>
      let (drs1, e1) := stepPredicate drs dom
      let (drs2, e2) := stepAmount drs1 m
      let (drs3, e3) := stepAmount drs2 b
      (drs3, e1 ++ e2 ++ e3)
    | .inZone z => stepZoneExpr drs z
    | .inCombat _ (some cp) | .hasDesignation _ (some cp) | .related _ (some cp) =>
      let (drs', _, errs) := stepNounPhrase drs .other false cp
      (drs', errs)
    | .exiledWith src | .otherThan src =>
      let (drs', _, errs) := stepNounPhrase drs .other false src
      (drs', errs)
    | .and ps | .or ps => stepPredicateList drs ps
    | _ => (drs, [])
  termination_by structural p

  def stepPredicateList (drs : DRS) (ps : List Predicate) : DRS × List Refusal :=
    match ps with
    | [] => (drs, [])
    | p :: rest =>
      let (drs1, e1) := stepPredicate drs p
      let (drs2, e2) := stepPredicateList drs1 rest
      (drs2, e1 ++ e2)
  termination_by structural ps

  def stepDetPhrase (drs : DRS) (d : DetPhrase) : DRS × List Refusal :=
    match d with
    | .target q | .count q _ => stepQuantity drs q
    | _ => (drs, [])

  def stepSliceCount (drs : DRS) (sc : SliceCount) : DRS × List Refusal :=
    match sc with
    | .counted q => stepQuantity drs q
    | .whole => (drs, [])

  def stepNounPhrase (drs : DRS) (role : GramRole) (inArgSlot : Bool)
      (np : NounPhrase) : DRS × List ReferentId × List Refusal :=
    match np with
    | .inCaller body =>
      let callerDrs := enterCaller drs
      let (callerDrs', refIds, errs) := stepNounPhrase callerDrs role inArgSlot body
      (leaveCaller drs callerDrs', refIds, errs)
    | .firstMention id body =>
      let (drs', refIds, errs) := stepNounPhrase drs role inArgSlot body
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
      match drs.visibleReferents.head? with
      | some r =>
        let drs' := drs.recordMention r.id role inArgSlot
        (drs', [r.id], [])
      | none => (drs, [], [.anaphor (.word .permanent) .one 0])
    | .described detP pred =>
      let (drs1, e1) := stepDetPhrase drs detP
      let (drs2, e2) := stepPredicate drs1 pred
      let ref := bindFor drs2 detP.det detP.plur (pred.kindOr .object) pred
      let rid := drs2.nextId
      let drs3 := drs2.push ref role inArgSlot
      (drs3, [rid], e1 ++ e2)
    | .and ns | .or ns => stepNounPhraseList drs role inArgSlot ns
    | .eachOf grp =>
      let (drs1, _, e1) := stepNounPhrase drs .other false grp
      let ref : Referent := { det := .each, plur := .one, payload := .object [] none none none none none }
      let rid := drs1.nextId
      let drs2 := drs1.push ref role inArgSlot
      (drs2, [rid], e1)
    | .someOf sc optP grp =>
      let (drs1, e1) := stepSliceCount drs sc
      let (drs2, e2) := match optP with
        | some p => stepPredicate drs1 p
        | none => (drs1, [])
      let (drs3, grpIds, e3) := stepNounPhrase drs2 .other false grp
      let grpZone := (grpIds.head?.bind (drs3.findById?)).bind (·.zone)
      let grpTy := (grpIds.head?.bind (drs3.findById?)).map (·.ty) |>.getD []
      let ref : Referent := {
        det := .part, plur := sc.plur,
        payload := .object grpTy grpZone none none none none
      }
      let rid := drs3.nextId
      let drs4 := drs3.push ref role inArgSlot
      (drs4, [rid], e1 ++ e2 ++ e3)
    | .slice _ amt p =>
      let (drs1, e1) := stepAmount drs amt
      let (drs2, e2) := stepPile drs1 p
      let ref : Referent := {
        det := .the, plur := .many,
        payload := .object [] p.sort none none none none
      }
      let rid := drs2.nextId
      let drs3 := drs2.push ref role inArgSlot
      (drs3, [rid], e1 ++ e2)
    | .theRest k pl =>
      let zn := zoneOfGroup k drs
      let ty := tyOfGroup k drs
      let ref : Referent := {
        det := .the, plur := pl,
        payload := if k == .player then .player false else .object ty zn none none none none
      }
      let rid := drs.nextId
      let drs' := drs.push ref role inArgSlot
      (drs', [rid], if theRestOk k drs then [] else [.theRestFits k])
    | .possessorOf _ subj =>
      let (drs1, _, e1) := stepNounPhrase drs .other false subj
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
      let (drs1, _, e1) := stepNounPhrase drs .other false subj
      let ref := facet.binding
      let rid := drs1.nextId
      let drs2 := drs1.push ref role inArgSlot
      (drs2, [rid], e1)
    | .asType t subj _ =>
      let (drs1, _, e1) := stepNounPhrase drs .other false subj
      let ref : Referent := {
        det := .the, plur := .one,
        payload := .object [t] (if t.permanent then some .battlefield else none) none none none none
      }
      let rid := drs1.nextId
      let drs2 := drs1.push ref role inArgSlot
      (drs2, [rid], e1)
    | .asMarker _ subj | .resolvedPermanent subj | .becomes subj | .counterpart _ subj =>
      let (drs1, _, e1) := stepNounPhrase drs .other false subj
      let ref : Referent := {
        det := .the, plur := .one,
        payload := .object [] (some .battlefield) none none none none
      }
      let rid := drs1.nextId
      let drs2 := drs1.push ref role inArgSlot
      (drs2, [rid], e1)
    | .theGrantor _ | .attachHost _ _ =>
      let ref : Referent := {
        det := .the, plur := .one,
        payload := .object [] (some .battlefield) none none none none
      }
      let rid := drs.nextId
      let drs' := drs.push ref role inArgSlot
      (drs', [rid], [])
    | .designated d whose =>
      let (drs1, _, e1) := stepNounPhrase drs .other false whose
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
    | .any => (drs, [], [])
  termination_by structural np

  def stepNounPhraseList (drs : DRS) (role : GramRole) (inArgSlot : Bool)
      (ns : List NounPhrase) : DRS × List ReferentId × List Refusal :=
    match ns with
    | [] => (drs, [], [])
    | n :: rest =>
      let (drs1, ids1, e1) := stepNounPhrase drs role inArgSlot n
      let (drs2, ids2, e2) := stepNounPhraseList drs1 role inArgSlot rest
      (drs2, ids1 ++ ids2, e1 ++ e2)
  termination_by structural ns

  def stepAmount (drs : DRS) (a : Amount) : DRS × List Refusal :=
    match a with
    | .inCaller body =>
      let (callerDrs, errs) := stepAmount (enterCaller drs) body
      (leaveCaller drs callerDrs, errs)
    | .lit _ | .anyAmount => (drs, [])
    | .statOf _ subj | .countOf subj =>
      let (drs', _, errs) := stepNounPhrase drs .other false subj
      (drs', errs)
    | .tally clause mode =>
      let (_, e1) := stepDidClause drs clause
      let e2 := if tallyOk mode clause then [] else [.tallyOk]
      (drs, e1 ++ e2)
    | .thatMuch =>
      if countAmountOutcomes drs == 1 then (drs, []) else (drs, [.quantOutcomeInScope (countAmountOutcomes drs)])
    | .theOutcome s =>
      if outcomeInScope s drs then (drs, []) else (drs, [.outcomeInScope s (countOutcomes s drs)])
    | .letter l =>
      if anyOpenLetter l drs then (drs, []) else (drs, [.openLetter l])
    | .arith _ l r =>
      let (drs1, e1) := stepAmount drs l
      let (drs2, e2) := stepAmount drs1 r
      (drs2, e1 ++ e2)
    | .upTo b => stepAmount drs b
  termination_by structural a

  def stepQuantity (drs : DRS) (q : Quantity) : DRS × List Refusal :=
    match q with
    | .inCaller body =>
      let (callerDrs, errs) := stepQuantity (enterCaller drs) body
      (leaveCaller drs callerDrs, errs)
    | .range (some lo) (some hi) => (drs, if lo ≤ hi then [] else [.wellFormedQ])
    | .range _ _ => (drs, [])
    | .upToOf a | .exactlyOf a => stepAmount drs a
  termination_by structural q

  def stepCondition (drs : DRS) (c : Condition) : DRS × List Refusal :=
    match c with
    | .inCaller body =>
      let (callerDrs, errs) := stepCondition (enterCaller drs) body
      (leaveCaller drs callerDrs, errs)
    | .duringPart _ (some who) =>
      let (drs', _, errs) := stepNounPhrase drs .other false who
      (drs', errs)
    | .duringPart _ none => (drs, [])
    | .exists_ subj =>
      let (drs', _, errs) := stepNounPhrase drs .subj true subj
      (drs', errs)
    | .did clause => stepDidClause drs clause
    | .matches subj pred =>
      let (drs1, _, e1) := stepNounPhrase drs .subj true subj
      let (drs2, e2) := stepPredicate drs1 pred
      (drs2, e1 ++ e2)
    | .compareAmt l _ r =>
      let (drs1, e1) := stepAmount drs l
      let (drs2, e2) := stepAmount drs1 r
      (drs2, e1 ++ e2)
    | .gameIs _ | .noHolder _ => (drs, [])
    | .hasPriority p =>
      let (drs', _, errs) := stepNounPhrase drs .subj true p
      (drs', errs)
    | .not inner =>
      let (_, errs) := stepCondition drs inner
      (drs, errs)
    | .and cs | .or cs => stepConditionList drs cs
  termination_by structural c

  def stepConditionList (drs : DRS) (cs : List Condition) : DRS × List Refusal :=
    match cs with
    | [] => (drs, [])
    | c :: rest =>
      let (drs1, e1) := stepCondition drs c
      let (drs2, e2) := stepConditionList drs1 rest
      (drs2, e1 ++ e2)
  termination_by structural cs

  def stepSubjectBefore (drs : DRS) (s : Subject) : DRS × List Refusal :=
    match s with
    | .actor np =>
      let (drs1, refIds, e1) := stepNounPhrase drs .subj true np
      let shape : ActorShape := {
        kind := np.kindIn drs1 .player,
        isYou := (np matches .you),
        selfDefined := false, ascribable := false, twoParties := false, opponentOnly := false,
        refIds := refIds
      }
      let drs2 := drs1.push { det := .the, plur := .one, payload := .actor shape } .other false
      (drs2, e1)
    | .any | .by _ => (drs, [])

  def stepSubjectAfter (drs : DRS) (s : Subject) : DRS × List Refusal :=
    match s with
    | .actor _ => (dropActorFrame drs, [])
    | .any => (drs, [])
    | .by np =>
      let (drs', _, e) := stepNounPhrase drs .other true np
      (drs', e)

  def stepDidClause (drs : DRS) (c : DidClause) : DRS × List Refusal :=
    match c with
    | ⟨subj, action, within⟩ =>
      let headErr := if action.eventHeadOk then [] else [.eventHead]
      let (drs1, e1) := stepSubjectBefore drs subj
      let (drs2, e2) := stepInstruction .postEvent subj.patientRole drs1 action
      let (drs3, e3) := stepSubjectAfter drs2 subj
      let (drs4, e4) := stepLookbackSpan drs3 within
      (drs4, headErr ++ e1 ++ e2 ++ e3 ++ e4)
  termination_by structural c

  def stepLookbackSpan (drs : DRS) (sp : LookbackSpan) : DRS × List Refusal :=
    match sp with
    | .duration d t =>
      let (drs1, e1) := stepDuration drs d
      let (drs2, e2) := match t with
        | some tm => stepTiming drs1 tm
        | none => (drs1, [])
      (drs2, e1 ++ e2)
    | .thisWay | .triggering => (drs, [])
  termination_by structural sp

  def stepDuration (drs : DRS) (d : Duration) : DRS × List Refusal :=
    match d with
    | .between s e =>
      let (drs1, e1) := match s with | some ev => stepGameEvent drs ev | none => (drs, [])
      let (drs2, e2) := match e with | some ev => stepGameEvent drs1 ev | none => (drs1, [])
      (drs2, e1 ++ e2)
    | .forAsLongAs c => stepCondition drs c
  termination_by structural d

  def stepTiming (drs : DRS) (tm : Timing) : DRS × List Refusal :=
    match tm with
    | .inPart _ (some who) _ =>
      let (drs', _, errs) := stepNounPhrase drs .other false who
      (drs', errs)
    | .inPart _ none _ | .anyTiming => (drs, [])
    | .relativeToEvent _ ev | .whileDoing ev => stepGameEvent drs ev
    | .onlyIf c => stepCondition drs c
    | .not t =>
      let (_, errs) := stepTiming drs t
      (drs, errs)
    | .and ts | .or ts => stepTimingList drs ts
  termination_by structural tm

  def stepTimingList (drs : DRS) (ts : List Timing) : DRS × List Refusal :=
    match ts with
    | [] => (drs, [])
    | t :: rest =>
      let (drs1, e1) := stepTiming drs t
      let (drs2, e2) := stepTimingList drs1 rest
      (drs2, e1 ++ e2)
  termination_by structural ts

  def stepGameEvent (drs : DRS) (ev : GameEvent) : DRS × List Refusal :=
    match ev with
    | .inCaller body =>
      let (callerDrs, errs) := stepGameEvent (enterCaller drs) body
      (leaveCaller drs callerDrs, errs)
    | .does subj action =>
      let headErr := if action.eventHeadOk then [] else [.eventHead]
      let (drs1, e1) := stepSubjectBefore drs subj
      let (drs2, e2) := stepInstruction .postEvent subj.patientRole drs1 action
      let (drs3, e3) := stepSubjectAfter drs2 subj
      (drs3, headErr ++ e1 ++ e2 ++ e3)
    | .beginningOf _ (some who) _ | .endOf _ (some who) _ =>
      let (drs', _, errs) := stepNounPhrase drs .subj true who
      (drs', errs)
    | .beginningOf _ none _ | .endOf _ none _
    | .gameBecomes _ | .chapterMark _ | .stateBasedActionsChecked | .now => (drs, [])
    | .stateHolds c => stepCondition drs c
  termination_by structural ev

  def stepWouldClause (drs : DRS) (wc : WouldClause) : DRS × List Refusal :=
    match wc with
    | .would subj action =>
      let headErr := if action.eventHeadOk then [] else [.eventHead]
      let (drs1, e1) := stepSubjectBefore drs subj
      let (drs2, e2) := stepInstruction .preEvent subj.patientRole drs1 action
      let (drs3, e3) := stepSubjectAfter drs2 subj
      (drs3, headErr ++ e1 ++ e2 ++ e3)
    | .wouldAny cs => stepWouldClauseList drs cs
  termination_by structural wc

  def stepWouldClauseList (drs : DRS) (cs : List WouldClause) : DRS × List Refusal :=
    match cs with
    | [] => (drs, [])
    | c :: rest =>
      let (drs1, e1) := stepWouldClause drs c
      let (drs2, e2) := stepWouldClauseList drs1 rest
      (drs2, e1 ++ e2)
  termination_by structural cs

  def stepCombatParticipation (drs : DRS) (cp : CombatParticipation) : DRS × List Refusal :=
    match cp with
    | .attacking (some defn) =>
      let (drs', _, e) := stepNounPhrase drs .other true defn
      (drs', e)
    | .attacking none | .blocked _ | .outsideCombat => (drs, [])

  def stepCombatUpdate (drs : DRS) (cu : CombatUpdate) : DRS × List Refusal :=
    match cu with
    | .participation state => stepCombatParticipation drs state
    | .blocking _ atk =>
      let (drs', _, e) := stepNounPhrase drs .other true atk
      (drs', e)

  def stepDoor (drs : DRS) (d : Door) : DRS × List Refusal :=
    match d with
    | .thisDoor => (drs, [])
    | .doorOf _ room =>
      let (drs', _, e) := stepNounPhrase drs .other false room
      (drs', e)

  def stepInstruction (phase : StepPhase) (patientRole : GramRole) (drs : DRS)
      (inst : Instruction) : DRS × List Refusal :=
    match inst with
    | .inCaller body =>
      let callerDrs := enterCaller drs
      let (callerDrs', errs) := stepInstruction phase patientRole callerDrs body
      (leaveCaller drs callerDrs', errs)
    | .ownScope deed body =>
      let innerDrs := enterOwnScope drs
      let (innerDrs', errs) := stepInstruction phase patientRole innerDrs body
      let outDrs := leaveOwnScope innerDrs'
      let _ := deed
      (outDrs, errs)
    | .bracketed body => stepInstruction phase patientRole drs body
    | .enact verb body becomes locus =>
      let (drs1, e1) := stepInstruction phase patientRole drs body
      let (drs2, e2) := match becomes with | some p => stepPredicate drs1 p | none => (drs1, [])
      let (drs3, e3) := match locus with | some z => stepZoneExpr drs2 z | none => (drs2, [])
      let _ := verb
      (drs3, e1 ++ e2 ++ e3)
    | .instruct subj body =>
      let (drs1, refIds, e1) := stepNounPhrase drs .subj true subj
      let shape : ActorShape := {
        kind := subj.kindIn drs1 .player,
        isYou := (subj matches .you),
        selfDefined := false, ascribable := false, twoParties := false, opponentOnly := false,
        refIds := refIds
      }
      let drs2 := drs1.push { det := .the, plur := .one, payload := .actor shape } .other false
      let (drs3, e2) := stepInstruction phase .obj drs2 body
      (dropActorFrame drs3, e1 ++ e2)
    | .dealDamage amt recipient _ =>
      let (drs1, e1) := stepAmount drs amt
      let (drs2, _, e2) := stepNounPhrase drs1 patientRole true recipient
      let drs3 := drs2.push (outcomeB .damageDealt) .other false
      (drs3, e1 ++ e2)
    | .markDamage subj amt =>
      let (drs1, _, e1) := stepNounPhrase drs patientRole true subj
      let (drs2, e2) := stepAmount drs1 amt
      (drs2, e1 ++ e2)
    | .clearDamage subj | .turnOver subj | .turnFaceDown subj _
    | .changeControl subj | .gainDesignation subj _ =>
      let (drs1, refIds, e1) := stepNounPhrase drs patientRole true subj
      let drs2 := if phase == .postEvent then applyInPlaceStamp drs1 refIds inst.inferredDeed else drs1
      (drs2, e1)
    | .setStatus _ subj =>
      let (drs1, refIds, e1) := stepNounPhrase drs patientRole true subj
      let drs2 := if phase == .postEvent then applyInPlaceStamp drs1 refIds inst.inferredDeed else drs1
      (drs2, e1)
    | .combat subj upd =>
      let (drs1, _, e1) := stepNounPhrase drs patientRole true subj
      let (drs2, e2) := stepCombatUpdate drs1 upd
      (drs2, e1 ++ e2)
    | .attack (some defn) | .attackUnblocked (some defn) =>
      let (drs1, _, e1) := stepNounPhrase drs patientRole true defn
      (drs1, e1)
    | .attack none | .attackUnblocked none => (drs, [])
    | .attackWith atks optDef =>
      let (drs1, _, e1) := stepNounPhrase drs patientRole true atks
      let (drs2, e2) := match optDef with
        | some d => let (d', _, e) := stepNounPhrase drs1 .other true d; (d', e)
        | none => (drs1, [])
      (drs2, e1 ++ e2)
    | .block atk =>
      let (drs1, _, e1) := stepNounPhrase drs patientRole true atk
      (drs1, e1)
    | .attachment _ subj optHost =>
      let (drs1, _, e1) := stepNounPhrase drs patientRole true subj
      let (drs2, e2) := match optHost with
        | some h => let (d', _, e) := stepNounPhrase drs1 .other true h; (d', e)
        | none => (drs1, [])
      (drs2, e1 ++ e2)
    | .unlock d => stepDoor drs d
    | .setGameDesignation _ | .commitCrime | .conclude _ | .drawGame | .restartGame => (drs, [])
    | .relate subj _ cp =>
      let (drs1, _, e1) := stepNounPhrase drs patientRole true subj
      let (drs2, _, e2) := stepNounPhrase drs1 .other true cp
      (drs2, e1 ++ e2)
    | .choose first chosen _ =>
      let (drs1, e1) := match first with
        | some f => let (d, _, e) := stepNounPhrase drs .subj true f; (d, e)
        | none => (drs, [])
      let (drs2, _, e2) := stepNounPhrase drs1 patientRole true chosen
      (drs2, e1 ++ e2)
    | .move subj from_ to riders _ _ =>
      let (drs1, refIds, e1) := stepNounPhrase drs patientRole true subj
      let (drs2, e2) := stepEventSource drs1 from_
      let (drs3, e3) := match to with | some z => stepZoneExpr drs2 z | none => (drs2, [])
      let (drs4, e4) := stepEntryRiderList drs3 riders
      let destZone := to.bind (·.sort)
      let drs5 := if phase == .postEvent then
        applyZoneMove drs4 refIds inst.inferredDeed destZone
      else drs4
      (drs5, e1 ++ e2 ++ e3 ++ e4)
    | .changeLife delta =>
      let (drs1, e1) := stepDelta drs delta
      let drs2 := match delta.outcome with
        | some out => drs1.push (outcomeB out) .other false
        | none => drs1
      (drs2, e1)
    | .draw amt =>
      let (drs1, e1) := stepAmount drs amt
      let drs2 := if phase == .postEvent then
        let cardRef : Referent := {
          det := .the, plur := .one,
          payload := .object [] (some .hand) (mkStamp (some (.core .draw)) (some .hand) false) none none none
        }
        drs1.push cardRef patientRole true
      else drs1
      (drs2, e1)
    | .expose _ exposed =>
      match exposed with
      | .cards np =>
        let (drs1, refIds, e1) := stepNounPhrase drs patientRole true np
        let drs2 := if phase == .postEvent then applyInPlaceStamp drs1 refIds inst.inferredDeed else drs1
        (drs2, e1)
      | .zone z => stepZoneExpr drs z
      | .choice _ => (drs, [])
    | .establish spec dur =>
      let (drs1, e1) := stepStaticSpec drs spec
      let (drs2, e2) := match dur with | some d => stepDuration drs1 d | none => (drs1, [])
      (drs2, e1 ++ e2)
    | .createObject cnt spec _ =>
      let (drs1, e1) := stepAmount drs cnt
      let (drs2, e2) := stepCreationSpec drs1 spec
      let drs3 := if phase == .postEvent then
        let tokRef : Referent := {
          det := .the, plur := .one,
          payload := .object [.creature] (some .battlefield) (mkStamp (some (.action "create")) (some .battlefield) false) (some .token) none none
        }
        drs2.push tokRef patientRole true
      else drs2
      (drs3, e1 ++ e2)
    | .putCounters amt _ on =>
      let (drs1, e1) := stepAmount drs amt
      let (drs2, _, e2) := stepNounPhrase drs1 patientRole true on
      let drs3 := drs2.push (outcomeB .countersPut) .other false
      (drs3, e1 ++ e2)
    | .removeCounters optQ _ from_ _ =>
      let (drs1, e1) := match optQ with | some q => stepQuantity drs q | none => (drs, [])
      let (drs2, _, e2) := stepNounPhrase drs1 patientRole true from_
      let drs3 := drs2.push (outcomeB .countersRemoved) .other false
      (drs3, e1 ++ e2)
    | .pay cost _ _ whose _ =>
      let (drs1, e1) := match whose with
        | some w => let (d, _, e) := stepNounPhrase drs .subj true w; (d, e)
        | none => (drs, [])
      let (drs2, e2) := stepCost drs1 cost
      (drs2, e1 ++ e2)
    | .withContinuation _ body ifDid ifNot =>
      let (drs1, e1) := stepInstruction phase patientRole drs body
      let (drs2, e2) := match ifDid with | some b => stepInstruction phase .obj (nextClause drs1) b | none => (drs1, [])
      let (_, e3) := match ifNot with | some b => stepInstruction phase .obj (nextClause drs1) b | none => (drs1, [])
      (drs2, e1 ++ e2 ++ e3)
    | .doIf cond thenInst elseInst =>
      let (drs1, e1) := stepCondition drs cond
      let (drs2, e2) := stepInstruction phase .obj (nextClause drs1) thenInst
      match elseInst with
      | none => (drs2, e1 ++ e2)
      | some el =>
        let (_, e3) := stepInstruction phase .obj (nextClause drs) el
        (drs, e1 ++ e2 ++ e3)
    | .doForEach grp body =>
      let (drs1, grpIds, e1) := stepNounPhrase drs .other false grp
      let preLoopMaxId := drs1.nextId
      let grpZone := (grpIds.head?.bind (drs1.findById?)).bind (·.zone)
      let grpTy := (grpIds.head?.bind (drs1.findById?)).map (·.ty) |>.getD []
      let loopRef : Referent := {
        det := .each, plur := .one,
        payload := .object grpTy grpZone none none none none
      }
      let drs2 := drs1.push loopRef .subj true
      let (drs3, e2) := stepInstruction phase .obj (nextClause drs2) body
      let closedDrs : DRS := { drs3 with referents := drs3.referents.filter (·.id < preLoopMaxId) }
      (closedDrs, e1 ++ e2)
    | .sequentially steps => stepInstructionSeq phase drs steps
    | .simultaneously steps => stepInstructionSimul phase drs steps
    | .procedure _ subj pay res from_ of_ =>
      let (drs1, refIds, e1) := stepNounPhrase drs patientRole true subj
      let (drs2, e2) := stepPlayPayment drs1 pay
      let (drs3, e3) := match res with | some p => stepPredicate drs2 p | none => (drs2, [])
      let (drs4, e4) := match from_ with | some f => stepEventSource drs3 f | none => (drs3, [])
      let (drs5, e5) := match of_ with
        | some o => let (d, _, e) := stepNounPhrase drs4 .other false o; (d, e)
        | none => (drs4, [])
      let drs6 := if phase == .postEvent then applyZoneMove drs5 refIds (some (.action "cast")) (some .stack) else drs5
      (drs6, e1 ++ e2 ++ e3 ++ e4 ++ e5)
  termination_by structural inst

  def stepInstructionSeq (phase : StepPhase) (drs : DRS) (steps : List Instruction) : DRS × List Refusal :=
    match steps with
    | [] => (drs, [])
    | [i] => stepInstruction phase .obj drs i
    | i :: rest =>
      let (drs1, e1) := stepInstruction phase .obj drs i
      let (drs2, e2) := stepInstructionSeq phase (nextClause drs1) rest
      (drs2, e1 ++ e2)
  termination_by structural steps

  def stepInstructionSimul (phase : StepPhase) (drs : DRS) (steps : List Instruction) : DRS × List Refusal :=
    match steps with
    | [] => (drs, [])
    | i :: rest =>
      let (drs1, e1) := stepInstruction phase .obj drs i
      let (drs2, e2) := stepInstructionSimul phase drs1 rest
      (drs2, e1 ++ e2)
  termination_by structural steps

  def stepEntryRider (drs : DRS) (r : EntryRider) : DRS × List Refusal :=
    match r with
    | .entersAs _ | .entersTransformed | .entersMelded _ | .faceDown _ => (drs, [])
    | .entersAttacking (some defn) =>
      let (drs', _, e) := stepNounPhrase drs .other true defn
      (drs', e)
    | .entersAttacking none => (drs, [])
    | .withCounters amt _ _ => stepAmount drs amt
    | .under ctrl =>
      let (drs', _, errs) := stepNounPhrase drs .other false ctrl
      (drs', errs)

  def stepEntryRiderList (drs : DRS) (rs : List EntryRider) : DRS × List Refusal :=
    match rs with
    | [] => (drs, [])
    | r :: rest =>
      let (drs1, e1) := stepEntryRider drs r
      let (drs2, e2) := stepEntryRiderList drs1 rest
      (drs2, e1 ++ e2)
  termination_by structural rs

  def stepCreationSpec (drs : DRS) (spec : CreationSpec) : DRS × List Refusal :=
    match spec with
    | .token _ riders => stepEntryRiderList drs riders
    | .emblem _ abs => stepAbilityList drs abs
  termination_by structural spec

  def stepPlayPayment (drs : DRS) (pay : PlayPayment) : DRS × List Refusal :=
    match pay with
    | .itsOwnCost | .withoutPaying => (drs, [])
    | .payingInstead c | .faceDown _ c => stepCost drs c

  def stepCost (drs : DRS) (c : Cost) : DRS × List Refusal :=
    match c with
    | .inCaller body =>
      let (callerDrs, errs) := stepCost (enterCaller drs) body
      (leaveCaller drs callerDrs, errs)
    | .mana _ | .tapSymbol | .untapSymbol | .loyaltySymbol _ | .itsManaCost => (drs, [])
    | .scaled c amt =>
      let (drs1, e1) := stepCost drs c
      let (drs2, e2) := stepAmount drs1 amt
      (drs2, e1 ++ e2)
    | .perform inst => stepInstruction .postEvent .obj drs inst
    | .compound cs | .or cs => stepCostList drs cs
  termination_by structural c

  def stepCostList (drs : DRS) (cs : List Cost) : DRS × List Refusal :=
    match cs with
    | [] => (drs, [])
    | c :: rest =>
      let (drs1, e1) := stepCost drs c
      let (drs2, e2) := stepCostList drs1 rest
      (drs2, e1 ++ e2)
  termination_by structural cs

  def stepDelta (drs : DRS) (d : Delta Amount) : DRS × List Refusal :=
    match d with
    | .up a | .down a | .set a => stepAmount drs a

  def stepStaticSpec (drs : DRS) (s : StaticSpec) : DRS × List Refusal :=
    match s with
    | .inCaller body =>
      let (callerDrs, errs) := stepStaticSpec (enterCaller drs) body
      (leaveCaller drs callerDrs, errs)
    | .modification subj _ d =>
      let (drs1, _, e1) := stepNounPhrase drs .subj true subj
      let (drs2, e2) := stepDelta drs1 d
      (drs2, e1 ++ e2)
    | .ptModification subj p t =>
      let (drs1, _, e1) := stepNounPhrase drs .subj true subj
      let (drs2, e2) := stepDelta drs1 p
      let (drs3, e3) := stepDelta drs2 t
      (drs3, e1 ++ e2 ++ e3)
    | .letterDefinition l amt =>
      let (drs1, e1) := stepAmount drs amt
      (defineLetter l drs1, e1)
    | .abilityGrant subj ab =>
      let (drs1, _, e1) := stepNounPhrase drs .subj true subj
      let e2 := checkAbility drs1 ab
      (drs1, e1 ++ e2)
    | .controlGrant player subj =>
      let (drs1, _, e1) := stepNounPhrase drs .subj true player
      let (drs2, _, e2) := stepNounPhrase drs1 .obj true subj
      (drs2, e1 ++ e2)
    | .entryRider subj r =>
      let (drs1, _, e1) := stepNounPhrase drs .subj true subj
      let (drs2, e2) := stepEntryRider drs1 r
      (drs2, e1 ++ e2)
    | .combatant subj =>
      let (drs1, _, e1) := stepNounPhrase drs .subj true subj
      (drs1, e1)
    | .cant subj action _ =>
      let headErr := if action.eventHeadOk then [] else [.eventHead]
      let (drs1, e1) := stepSubjectBefore drs subj
      let (drs2, e2) := stepInstruction .postEvent subj.patientRole drs1 action
      let (_, e3) := stepSubjectAfter drs2 subj
      (drs, headErr ++ e1 ++ e2 ++ e3)
    | .must subj action _ | .may subj action _ =>
      let headErr := if action.eventHeadOk then [] else [.eventHead]
      let (drs1, e1) := stepSubjectBefore drs subj
      let (drs2, e2) := stepInstruction .postEvent subj.patientRole drs1 action
      let (drs3, e3) := stepSubjectAfter drs2 subj
      (drs3, headErr ++ e1 ++ e2 ++ e3)
    | .gatedBy cost subj action _ =>
      let headErr := if action.eventHeadOk then [] else [.eventHead]
      let (drs1, e1) := stepCost drs cost
      let (drs2, e2) := stepSubjectBefore (nextClause drs1) subj
      let (drs3, e3) := stepInstruction .postEvent subj.patientRole drs2 action
      let (drs4, e4) := stepSubjectAfter drs3 subj
      (drs4, headErr ++ e1 ++ e2 ++ e3 ++ e4)
    | .instead ev repl tm =>
      let (drs1, e1) := stepWouldClause drs ev
      let (drs2, e2) := stepInstruction .postEvent .obj (nextClause drs1) repl
      let (_, e3) := match tm with | some t => stepTiming drs2 t | none => (drs2, [])
      (drs, e1 ++ e2 ++ e3)
    | .damageResult r =>
      match r with
      | ⟨_, source, recipient, _, does⟩ =>
        let (drs1, e1) := stepPredicate drs recipient
        let (drs2, e2) := stepSubjectBefore (nextClause drs1) source
        let (drs3, e3) := stepInstruction .postEvent source.patientRole drs2 does
        let (_, e4) := stepSubjectAfter drs3 source
        (drs, e1 ++ e2 ++ e3 ++ e4)
    | .conditional spec cond _ =>
      let (drs1, e1) := stepCondition drs cond
      let (drs2, e2) := stepStaticSpec (nextClause drs1) spec
      (drs2, e1 ++ e2)
    | .conjunction optSubj parts =>
      let (drs1, e1) := match optSubj with
        | some sub => let (d, _, e) := stepNounPhrase drs .subj true sub; (d, e)
        | none => (drs, [])
      let (drs2, e2) := stepStaticSpecList drs1 parts
      (drs2, e1 ++ e2)
    | .specialAction tm cost does =>
      let (drs1, e1) := match tm with | some t => stepTiming drs t | none => (drs, [])
      let (drs2, e2) := stepCost drs1 cost
      let (drs3, e3) := stepInstruction .postEvent .obj (nextClause (publicOnly drs2)) does
      (drs3, e1 ++ e2 ++ e3)
    | .asEvent ev does =>
      let (drs1, e1) := stepGameEvent drs ev
      let (drs2, e2) := stepInstruction .postEvent .obj (nextClause drs1) does
      (drs2, e1 ++ e2)
    | .whileHolds cond does =>
      let (drs1, e1) := stepCondition drs cond
      let (drs2, e2) := stepInstruction .postEvent .obj (nextClause drs1) does
      (drs2, e1 ++ e2)
  termination_by structural s

  def stepStaticSpecList (drs : DRS) (ss : List StaticSpec) : DRS × List Refusal :=
    match ss with
    | [] => (drs, [])
    | s :: rest =>
      let (drs1, e1) := stepStaticSpec drs s
      let (drs2, e2) := stepStaticSpecList drs1 rest
      (drs2, e1 ++ e2)
  termination_by structural ss

  def checkAbility (drs : DRS) (ab : Ability) : List Refusal :=
    match ab with
    | .keyword _ _ body => (stepAbilityList drs body).2
    | .activated cost inst tm _ guard activator =>
      let (drs0, e0) := match activator with
        | some a => let (d, _, e) := stepNounPhrase drs .subj true a; (d, e)
        | none => (drs, [])
      let (drs1, e1) := stepCost drs0 cost
      let colonDrs := nextClause (publicOnly drs1)
      let (drs2, e2) := stepInstruction .postEvent .obj colonDrs inst
      let (_, e3) := match tm with | some t => stepTiming drs2 t | none => (drs2, [])
      let (_, e4) := match guard with | some g => stepCondition drs2 g | none => (drs2, [])
      e0 ++ e1 ++ e2 ++ e3 ++ e4
    | .triggered ev alts _ _ tm _ interv inst =>
      let (drs1, e1) := stepGameEvent drs ev
      let (_, eAlts) := stepGameEventList drs alts
      let (drs2, e2) := match interv with
        | some c => stepCondition (nextClause drs1) c
        | none => (drs1, [])
      let (drs3, e3) := stepInstruction .postEvent .obj (nextClause drs2) inst
      let (_, e4) := match tm with | some t => stepTiming drs3 t | none => (drs3, [])
      e1 ++ eAlts ++ e2 ++ e3 ++ e4
    | .static spec => (stepStaticSpec drs spec).2
    | .spell tm inst =>
      let (drs1, e1) := match tm with | some t => stepTiming drs t | none => (drs, [])
      let (_, e2) := stepInstruction .postEvent .obj drs1 inst
      e1 ++ e2
    | .qualified inner tm _ guard _ =>
      let e1 := checkAbility drs inner
      let (_, e2) := match tm with | some t => stepTiming drs t | none => (drs, [])
      let (_, e3) := match guard with | some g => stepCondition drs g | none => (drs, [])
      e1 ++ e2 ++ e3
  termination_by structural ab

  def stepAbilityList (drs : DRS) (abs : List Ability) : DRS × List Refusal :=
    match abs with
    | [] => (drs, [])
    | a :: rest =>
      let e1 := checkAbility drs a
      let (drs2, e2) := stepAbilityList drs rest
      (drs2, e1 ++ e2)
  termination_by structural abs

  def stepGameEventList (drs : DRS) (evs : List GameEvent) : DRS × List Refusal :=
    match evs with
    | [] => (drs, [])
    | e :: rest =>
      let (drs1, e1) := stepGameEvent drs e
      let (drs2, e2) := stepGameEventList drs1 rest
      (drs2, e1 ++ e2)
  termination_by structural evs
end

def NounPhrase.check (drs : DRS) (np : NounPhrase) : List Refusal :=
  (stepNounPhrase drs .other false np).2.2

def Predicate.check (drs : DRS) (p : Predicate) : List Refusal :=
  (stepPredicate drs p).2

def Amount.check (drs : DRS) (a : Amount) : List Refusal :=
  (stepAmount drs a).2

def Condition.check (drs : DRS) (c : Condition) : List Refusal :=
  (stepCondition drs c).2

def Instruction.check (drs : DRS) (i : Instruction) : List Refusal :=
  (stepInstruction .postEvent .obj drs i).2

def StaticSpec.check (drs : DRS) (s : StaticSpec) : List Refusal :=
  (stepStaticSpec drs s).2

def Ability.check (drs : DRS := DRS.ofList []) (a : Ability) : List Refusal :=
  checkAbility drs a

end Semantics
