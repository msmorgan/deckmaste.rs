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

def Quantity.plur (q : Quantity) (fuel : Nat := 16) : Plurality :=
  match fuel, q with
  | 0, _ => .many
  | n + 1, .inCaller body => body.plur n
  | _ + 1, .range (some 1) (some 1) => .one
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
  | _ + 1, .refl pl => ((firstReflexive pl drs).map (·.kind)).getD d
  | _ + 1, n => n.kind?.getD d
termination_by structural fuel

def Predicate.seedTy (p : Predicate) (fuel : Nat := 32) : List CardType :=
  match fuel, p with
  | 0, _ => []
  | n + 1, .inCaller body | n + 1, .bracketed body => body.seedTy n
  | _ + 1, .hasType t => [t]
  | _ + 1, .hasSubtype s => s.type.toList
  | n + 1, .superlative _ _ dom | n + 1, .compareOver dom _ _ _ => dom.seedTy n
  | n + 1, .and ps => ps.foldr (fun q acc => mergeTypes (q.seedTy n) acc) []
  | _ + 1, .or [] => []
  | n + 1, .or [q] => q.seedTy n
  | n + 1, .or (q :: qs) => qs.foldl (fun acc r => commonTypes acc (r.seedTy n)) (q.seedTy n)
  | _ + 1, _ => []
termination_by structural fuel

def Predicate.seedZone (p : Predicate) (fuel : Nat := 32) : Option Zone :=
  match fuel, p with
  | 0, _ => none
  | n + 1, .inCaller body | n + 1, .bracketed body => body.seedZone n
  | _ + 1, .inZone z => z.sort
  | _ + 1, .hasType t => if t.permanent then some .battlefield else none
  | _ + 1, .hasSubtype s => if (s.type.map CardType.permanent).getD false then some .battlefield else none
  | _ + 1, .isToken | _ + 1, .inCombat .. | _ + 1, .attachment .. => some .battlefield
  | _ + 1, .isEmblem => some .command
  | _ + 1, .exiledWith _ => some .exile
  | _ + 1, .hasStatus s => if s.word then some .battlefield else none
  | n + 1, .and ps => ps.findSome? (·.seedZone n)
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
  | _ + 1, .slice _ (.lit 1) _ => .one
  | _ + 1, .slice .. | _ + 1, .run .. | _ + 1, .oneEachOf .. => .many
  | _ + 1, .and _ => .many
  | n + 1, .or (x :: _) => x.plur n
  | _ + 1, .playerGroup .allPlayers | _ + 1, .playerGroup .yourOpponents | _ + 1, .playerGroup .yourTeam => .many
  | _ + 1, _ => .one
termination_by structural fuel

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
              | some v => mkStamp (some v) oldZn destZone.isSome
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

mutual

  def stepZoneScope (fuel : Nat) (drs : DRS) (s : ZoneScope) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match s with
      | .bare => (drs, [])
      | .possessedBy np =>
        let (drs', _, errs) := stepNounPhrase fuel drs .other false np
        (drs', errs)
  termination_by structural fuel

  def stepPile (fuel : Nat) (drs : DRS) (p : Pile) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match p with
      | .library s | .graveyard s => stepZoneScope fuel drs s
      | .that => (drs, [])
      | .exiledWith src =>
        let (drs', _, errs) := stepNounPhrase fuel drs .other false src
        (drs', errs)
  termination_by structural fuel

  def stepZoneExpr (fuel : Nat) (drs : DRS) (z : ZoneExpr) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match z with
      | .inCaller body =>
        let (callerDrs, errs) := stepZoneExpr fuel (enterCaller drs) body
        (leaveCaller drs callerDrs, errs)
      | .zone _ s => stepZoneScope fuel drs s
      | .place _ p => stepPile fuel drs p
      | .wherever | .anyZone => (drs, [])
  termination_by structural fuel

  def stepEventSource (fuel : Nat) (drs : DRS) (src : EventSource) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match src with
      | .anywhere => (drs, [])
      | .zones zs | .anywhereBut zs => stepZoneExprList fuel drs zs
  termination_by structural fuel

  def stepZoneExprList (fuel : Nat) (drs : DRS) (zs : List ZoneExpr) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match zs with
      | [] => (drs, [])
      | z :: rest =>
        let (drs1, e1) := stepZoneExpr fuel drs z
        let (drs2, e2) := stepZoneExprList fuel drs1 rest
        (drs2, e1 ++ e2)
  termination_by structural fuel

  def stepChoiceDomain (fuel : Nat) (drs : DRS) (cd : ChoiceDomain) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match cd with
      | .nameOfCard p | .players p => stepPredicate fuel drs p
      | .number q => stepQuantity fuel drs q
      | .colorOtherThan _ | .typeOtherThan _ | .basicTypesOnly | .nonbasicTypesOnly | .abilitiesAmong _ => (drs, [])
  termination_by structural fuel

  def stepColorTerm (fuel : Nat) (drs : DRS) (ct : ColorTerm) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match ct with
      | .lit _ => (drs, [])
      | .chosen ref =>
        let n := countChoice (.quality .color) drs
        (drs, if ref.ok n then [] else [.choiceRef ref (.quality .color) n])
      | .colorOf obj =>
        let (drs', _, errs) := stepNounPhrase fuel drs .other false obj
        (drs', errs)
  termination_by structural fuel

  def stepPredicate (fuel : Nat) (drs : DRS) (p : Predicate) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match p with
      | .inCaller body =>
        let (callerDrs, errs) := stepPredicate fuel (enterCaller drs) body
        (leaveCaller drs callerDrs, errs)
      | .bracketed body | .not body => stepPredicate fuel drs body
      | .chosenPlayer ref =>
        let n := countChoice .player drs
        (drs, if ref.ok n then [] else [.choiceRef ref .player n])
      | .qualityNoun _ (some dom) | .ofYourChoice _ (some dom) => stepChoiceDomain fuel drs dom
      | .qualityNoun _ none | .ofYourChoice _ none => (drs, [])
      | .ofChosen ref q =>
        let n := countChoice (.quality q) drs
        (drs, if ref.ok n then [] else [.choiceRef ref (.quality q) n])
      | .hasPossessor _ np =>
        let (drs', _, errs) := stepNounPhrase fuel drs .other false np
        (drs', errs)
      | .did clause =>
        let subDrs := nextClause drs
        let whichRef : Referent := {
          det := .the, plur := .one, clause := subDrs.currentClause, role := .subj,
          payload := .object [] (some .battlefield) none none none none
        }
        let subDrs' := subDrs.push whichRef .subj true
        let (_, errs) := stepDidClause fuel subDrs' clause
        (drs, errs)
      | .compare _ _ b => stepAmount fuel drs b
      | .superlative _ _ dom => stepPredicate fuel drs dom
      | .withMostVotes =>
        let n := countOutcomes .voteHeld drs
        (drs, if n == 1 then [] else [.outcomeInScope .voteHeld n])
      | .choseExtreme _ =>
        let drs' := drs.push (outcomeB .namedNumber) .other false
        (drs', if countManys (.quality .number) drs != 0 then [] else [.numberChoiceInScope])
      | .compareOver dom m _ b =>
        let (drs1, e1) := stepPredicate fuel drs dom
        let (drs2, e2) := stepAmount fuel drs1 m
        let (drs3, e3) := stepAmount fuel drs2 b
        (drs3, e1 ++ e2 ++ e3)
      | .inZone z => stepZoneExpr fuel drs z
      | .inPile pile =>
        let (drs', _, errs) := stepNounPhrase fuel drs .other false pile
        (drs', errs)
      | .inCombat _ (some cp) | .hasDesignation _ (some cp) | .related _ (some cp) =>
        let (drs', _, errs) := stepNounPhrase fuel drs .other false cp
        (drs', errs)
      | .counterKindOn src | .shares _ _ src | .abilityOf src
      | .activatedBy src | .targets src _ | .exiledWith src | .otherThan src =>
        let (drs', _, errs) := stepNounPhrase fuel drs .other false src
        (drs', errs)
      | .coinCameUp _ => (drs, if coinFlipInScope drs then [] else [.coinFlipInScope])
      | .and ps | .or ps => stepPredicateList fuel drs ps
      | _ => (drs, [])
  termination_by structural fuel

  def stepPredicateList (fuel : Nat) (drs : DRS) (ps : List Predicate) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match ps with
      | [] => (drs, [])
      | p :: rest =>
        let (drs1, e1) := stepPredicate fuel drs p
        let (drs2, e2) := stepPredicateList fuel drs1 rest
        (drs2, e1 ++ e2)
  termination_by structural fuel

  def stepDetPhrase (fuel : Nat) (drs : DRS) (d : DetPhrase) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match d with
      | .target q | .count q _ => stepQuantity fuel drs q
      | _ => (drs, [])
  termination_by structural fuel

  def stepSliceCount (fuel : Nat) (drs : DRS) (sc : SliceCount) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match sc with
      | .counted q => stepQuantity fuel drs q
      | .whole => (drs, [])
      | .totalling _ _ b => stepAmount fuel drs b
  termination_by structural fuel

  def stepNounPhrase (fuel : Nat) (drs : DRS) (role : GramRole) (inArgSlot : Bool) (np : NounPhrase) : DRS × List ReferentId × List Refusal :=
    match fuel with
    | 0 => (drs, [], [])
    | fuel + 1 =>
      match np with
      | .inCaller body =>
        let callerDrs := enterCaller drs
        let (callerDrs', refIds, errs) := stepNounPhrase fuel callerDrs role inArgSlot body
        (leaveCaller drs callerDrs', refIds, errs)
      | .firstMention id body =>
        let (drs', refIds, errs) := stepNounPhrase fuel drs role inArgSlot body
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
        let (drs1, e1) := stepDetPhrase fuel drs detP
        let (drs2, e2) := stepPredicate fuel drs1 pred
        let ref := bindFor drs2 detP.det detP.plur (pred.kindOr .object) pred
        let rid := drs2.nextId
        let drs3 := drs2.push ref role inArgSlot
        (drs3, [rid], e1 ++ e2)
      | .and ns | .or ns => stepNounPhraseList fuel drs role inArgSlot ns
      | .eachOf grp =>
        let (drs1, _, e1) := stepNounPhrase fuel drs .other false grp
        let ref : Referent := { det := .each, plur := .one, payload := .object [] none none none none none }
        let rid := drs1.nextId
        let drs2 := drs1.push ref role inArgSlot
        (drs2, [rid], e1)
      | .someOf sc optP grp =>
        let (drs1, e1) := stepSliceCount fuel drs sc
        let (drs2, e2) := match optP with
          | some p => stepPredicate fuel drs1 p
          | none => (drs1, [])
        let (drs3, grpIds, e3) := stepNounPhrase fuel drs2 .other false grp
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
        let (drs1, e1) := stepAmount fuel drs amt
        let (drs2, e2) := stepPile fuel drs1 p
        let ref : Referent := {
          det := .the, plur := .many,
          payload := .object [] p.sort none none none none
        }
        let rid := drs2.nextId
        let drs3 := drs2.push ref role inArgSlot
        (drs3, [rid], e1 ++ e2)
      | .run _ stop p =>
        let (drs1, e1) := stepPredicate fuel drs stop
        let (drs2, e2) := stepPile fuel drs1 p
        let stoppedRef : Referent := {
          det := .part, plur := .one,
          payload := .object stop.seedTy p.sort none none (some 1) none
        }
        let stoppedId := drs2.nextId
        let drs3 := drs2.push stoppedRef .other false
        let runRef : Referent := {
          det := .the, plur := .many,
          payload := .object [] p.sort none none none none
        }
        let runId := drs3.nextId
        let drs4 := drs3.push runRef role inArgSlot
        (drs4, [runId, stoppedId], e1 ++ e2)
      | .pileOf q optBy =>
        let (drs1, e1) := stepSliceCount fuel drs q
        let (drs2, e2) := match optBy with
          | some b => let (d', _, e) := stepNounPhrase fuel drs1 .other false b; (d', e)
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
        let (drs1, e1) := stepPredicateList fuel drs roles
        let (drs2, poolIds, e2) := stepNounPhrase fuel drs1 .other false pool
        let poolZone := (poolIds.head?.bind (drs2.findById?)).bind (·.zone)
        let poolTy := (poolIds.head?.bind (drs2.findById?)).map (·.ty) |>.getD []
        let ref : Referent := {
          det := .a, plur := .many,
          payload := .object poolTy poolZone none none none none
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
        let (drs1, _, e1) := stepNounPhrase fuel drs .other false subj
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
        let (drs1, _, e1) := stepNounPhrase fuel drs .other false subj
        let ref := facet.binding
        let rid := drs1.nextId
        let drs2 := drs1.push ref role inArgSlot
        (drs2, [rid], e1)
      | .asType t subj _ =>
        let (drs1, _, e1) := stepNounPhrase fuel drs .other false subj
        let ref : Referent := {
          det := .the, plur := .one,
          payload := .object [t] (if t.permanent then some .battlefield else none) none none none none
        }
        let rid := drs1.nextId
        let drs2 := drs1.push ref role inArgSlot
        (drs2, [rid], e1)
      | .asMarker _ subj | .resolvedPermanent subj | .becomes subj | .counterpart _ subj | .agree _ _ subj =>
        let (drs1, _, e1) := stepNounPhrase fuel drs .other false subj
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
        let (drs1, _, e1) := stepNounPhrase fuel drs .other false whose
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
  termination_by structural fuel

  def stepNounPhraseList (fuel : Nat) (drs : DRS) (role : GramRole) (inArgSlot : Bool) (ns : List NounPhrase) : DRS × List ReferentId × List Refusal :=
    match fuel with
    | 0 => (drs, [], [])
    | fuel + 1 =>
      match ns with
      | [] => (drs, [], [])
      | n :: rest =>
        let (drs1, ids1, e1) := stepNounPhrase fuel drs role inArgSlot n
        let (drs2, ids2, e2) := stepNounPhraseList fuel drs1 role inArgSlot rest
        (drs2, ids1 ++ ids2, e1 ++ e2)
  termination_by structural fuel

  def stepAmount (fuel : Nat) (drs : DRS) (a : Amount) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match a with
      | .inCaller body =>
        let (callerDrs, errs) := stepAmount fuel (enterCaller drs) body
        (leaveCaller drs callerDrs, errs)
      | .lit _ | .anyAmount => (drs, [])
      | .statOf _ subj | .countOf subj | .aggregate _ _ subj
      | .paid _ subj | .greatestStoredMatch subj | .distinctCount _ subj =>
        let (drs', _, errs) := stepNounPhrase fuel drs .other false subj
        (drs', errs)
      | .tally clause mode =>
        let (_, e1) := stepDidClause fuel drs clause
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
        if anyOpenLetter l drs then (drs, []) else (drs, [.openLetter l])
      | .arith _ l r =>
        let (drs1, e1) := stepAmount fuel drs l
        let (drs2, e2) := stepAmount fuel drs1 r
        (drs2, e1 ++ e2)
      | .devotion pl c optSec =>
        let (drs1, _, e1) := stepNounPhrase fuel drs .other false pl
        let (drs2, e2) := stepColorTerm fuel drs1 c
        let (drs3, e3) := match optSec with | some d => stepColorTerm fuel drs2 d | none => (drs2, [])
        (drs3, e1 ++ e2 ++ e3)
      | .half _ inner | .upTo inner => stepAmount fuel drs inner
      | .aggregateOver _ dom body =>
        let (drs1, e1) := stepPredicate fuel drs dom
        let k := dom.kindOr .object
        let drs2 := drs1.push (bindFor drs1 .the .one k dom) .other false
        let (drs3, e2) := stepAmount fuel drs2 body
        (drs3, e1 ++ e2)
      | .aggregateOf _ as => stepAmountList fuel drs as
  termination_by structural fuel

  def stepAmountList (fuel : Nat) (drs : DRS) (as : List Amount) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match as with
      | [] => (drs, [])
      | a :: rest =>
        let (drs1, e1) := stepAmount fuel drs a
        let (drs2, e2) := stepAmountList fuel drs1 rest
        (drs2, e1 ++ e2)
  termination_by structural fuel

  def stepQuantity (fuel : Nat) (drs : DRS) (q : Quantity) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match q with
      | .inCaller body =>
        let (callerDrs, errs) := stepQuantity fuel (enterCaller drs) body
        (leaveCaller drs callerDrs, errs)
      | .range (some lo) (some hi) => (drs, if lo ≤ hi then [] else [.wellFormedQ])
      | .range _ _ | .worth _ => (drs, [])
      | .upToOf a | .exactlyOf a => stepAmount fuel drs a
  termination_by structural fuel

  def stepCondition (fuel : Nat) (drs : DRS) (c : Condition) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match c with
      | .inCaller body =>
        let (callerDrs, errs) := stepCondition fuel (enterCaller drs) body
        (leaveCaller drs callerDrs, errs)
      | .duringPart _ (some who) =>
        let (drs', _, errs) := stepNounPhrase fuel drs .other false who
        (drs', errs)
      | .duringPart _ none => (drs, [])
      | .exists_ subj =>
        let (drs', _, errs) := stepNounPhrase fuel drs .subj true subj
        (drs', errs)
      | .did clause => stepDidClause fuel drs clause
      | .matches subj pred =>
        let (drs1, _, e1) := stepNounPhrase fuel drs .subj true subj
        let (drs2, e2) := stepPredicate fuel drs1 pred
        (drs2, e1 ++ e2)
      | .compareAmt l _ r =>
        let (drs1, e1) := stepAmount fuel drs l
        let (drs2, e2) := stepAmount fuel drs1 r
        (drs2, e1 ++ e2)
      | .dealtThisWay pred =>
        let (drs1, e1) := stepPredicate fuel drs pred
        (drs1, e1 ++ (if damageDealtInScope drs then [] else [.damageDealtInScope]))
      | .choseThisWay who pred =>
        let (drs1, _, e1) := stepNounPhrase fuel drs .subj true who
        let (drs2, e2) := stepPredicate fuel drs1 pred
        (drs2, e1 ++ e2)
      | .preventedFromSource pred =>
        let (drs1, e1) := stepPredicate fuel drs pred
        let n := countOutcomes .damagePrevented drs
        (drs1, e1 ++ (if n == 1 then [] else [.outcomeInScope .damagePrevented n]))
      | .flipCalled caller _ =>
        let (drs1, _, e1) := stepNounPhrase fuel drs .subj true caller
        (drs1, e1 ++ (if coinFlipInScope drs then [] else [.coinFlipInScope]))
      | .flipFace _ => (drs, if coinFlipInScope drs then [] else [.coinFlipInScope])
      | .voteLead _ _ =>
        let n := countOutcomes .voteHeld drs
        (drs, if n == 1 then [] else [.outcomeInScope .voteHeld n])
      | .anyResultIs _ bound =>
        let (drs1, e1) := stepAmount fuel drs bound
        let n := countOutcomes .rollResult drs
        (drs1, e1 ++ (if n == 1 then [] else [.outcomeInScope .rollResult n]))
      | .rolledDoubles =>
        let n := countOutcomes .rollResult drs
        (drs, if n == 1 then [] else [.outcomeInScope .rollResult n])
      | .gameIs _ | .noHolder _ => (drs, [])
      | .hasPriority p =>
        let (drs', _, errs) := stepNounPhrase fuel drs .subj true p
        (drs', errs)
      | .not inner =>
        let (_, errs) := stepCondition fuel drs inner
        (drs, errs)
      | .and cs | .or cs => stepConditionList fuel drs cs
  termination_by structural fuel

  def stepConditionList (fuel : Nat) (drs : DRS) (cs : List Condition) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match cs with
      | [] => (drs, [])
      | c :: rest =>
        let (drs1, e1) := stepCondition fuel drs c
        let (drs2, e2) := stepConditionList fuel drs1 rest
        (drs2, e1 ++ e2)
  termination_by structural fuel

  def stepSubjectBefore (fuel : Nat) (drs : DRS) (s : Subject) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match s with
      | .actor np =>
        let (drs1, refIds, e1) := stepNounPhrase fuel drs .subj true np
        let shape : ActorShape := {
          kind := np.kindIn drs1 .player,
          isYou := (np matches .you),
          selfDefined := false, ascribable := false, twoParties := false, opponentOnly := false,
          refIds := refIds
        }
        let drs2 := drs1.push { det := .the, plur := .one, payload := .actor shape } .other false
        (drs2, e1)
      | .any | .by _ => (drs, [])
  termination_by structural fuel

  def stepSubjectAfter (fuel : Nat) (drs : DRS) (s : Subject) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match s with
      | .actor _ => (dropActorFrame drs, [])
      | .any => (drs, [])
      | .by np =>
        let (drs', _, e) := stepNounPhrase fuel drs .other true np
        (drs', e)
  termination_by structural fuel

  def stepDidClause (fuel : Nat) (drs : DRS) (c : DidClause) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match c with
      | ⟨subj, action, within⟩ =>
        let headErr := if action.eventHeadOk then [] else [.eventHead]
        let (drs1, e1) := stepSubjectBefore fuel drs subj
        let (drs2, e2) := stepInstruction fuel .postEvent subj.patientRole drs1 action
        let (drs3, e3) := stepSubjectAfter fuel drs2 subj
        let (drs4, e4) := stepLookbackSpan fuel drs3 within
        (drs4, headErr ++ e1 ++ e2 ++ e3 ++ e4)
  termination_by structural fuel

  def stepLookbackSpan (fuel : Nat) (drs : DRS) (sp : LookbackSpan) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match sp with
      | .duration d t =>
        let (drs1, e1) := stepDuration fuel drs d
        let (drs2, e2) := match t with
          | some tm => stepTiming fuel drs1 tm
          | none => (drs1, [])
        (drs2, e1 ++ e2)
      | .thisWay | .triggering => (drs, [])
  termination_by structural fuel

  def stepDuration (fuel : Nat) (drs : DRS) (d : Duration) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match d with
      | .between s e =>
        let (drs1, e1) := match s with | some ev => stepGameEvent fuel drs ev | none => (drs, [])
        let (drs2, e2) := match e with | some ev => stepGameEvent fuel drs1 ev | none => (drs1, [])
        (drs2, e1 ++ e2)
      | .forAsLongAs c => stepCondition fuel drs c
  termination_by structural fuel

  def stepTiming (fuel : Nat) (drs : DRS) (tm : Timing) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match tm with
      | .inPart _ (some who) _ =>
        let (drs', _, errs) := stepNounPhrase fuel drs .other false who
        (drs', errs)
      | .inPart _ none _ | .anyTiming => (drs, [])
      | .relativeToEvent _ ev | .whileDoing ev => stepGameEvent fuel drs ev
      | .onlyIf c => stepCondition fuel drs c
      | .not t =>
        let (_, errs) := stepTiming fuel drs t
        (drs, errs)
      | .and ts | .or ts => stepTimingList fuel drs ts
  termination_by structural fuel

  def stepTimingList (fuel : Nat) (drs : DRS) (ts : List Timing) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match ts with
      | [] => (drs, [])
      | t :: rest =>
        let (drs1, e1) := stepTiming fuel drs t
        let (drs2, e2) := stepTimingList fuel drs1 rest
        (drs2, e1 ++ e2)
  termination_by structural fuel

  def stepGameEvent (fuel : Nat) (drs : DRS) (ev : GameEvent) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match ev with
      | .inCaller body =>
        let (callerDrs, errs) := stepGameEvent fuel (enterCaller drs) body
        (leaveCaller drs callerDrs, errs)
      | .does subj action =>
        let headErr := if action.eventHeadOk then [] else [.eventHead]
        let (drs1, e1) := stepSubjectBefore fuel drs subj
        let (drs2, e2) := stepInstruction fuel .postEvent subj.patientRole drs1 action
        let (drs3, e3) := stepSubjectAfter fuel drs2 subj
        (drs3, headErr ++ e1 ++ e2 ++ e3)
      | .beginningOf _ (some who) _ | .endOf _ (some who) _ =>
        let (drs', _, errs) := stepNounPhrase fuel drs .subj true who
        (drs', errs)
      | .beginningOf _ none _ | .endOf _ none _
      | .gameBecomes _ | .chapterMark _ | .stateBasedActionsChecked | .now => (drs, [])
      | .statBecomes subj _ val =>
        let (drs1, _, e1) := stepNounPhrase fuel drs .subj true subj
        let (drs2, e2) := stepAmount fuel drs1 val
        (drs2, e1 ++ e2)
      | .stateHolds c => stepCondition fuel drs c
      | .triggers ab optOf optBy =>
        let (drs1, _, e1) := stepNounPhrase fuel drs .subj true ab
        let (drs2, e2) := match optOf with
          | some o => let (d', _, e) := stepNounPhrase fuel drs1 .other false o; (d', e)
          | none => (drs1, [])
        let (drs3, e3) := match optBy with
          | some b => let (d', _, e) := stepNounPhrase fuel drs2 .other false b; (d', e)
          | none => (drs2, [])
        (drs3, e1 ++ e2 ++ e3)
      | .nthOccurrence _ _ ev => stepGameEvent fuel drs ev
      | .causes c ev =>
        let (drs1, e1) := stepCausing fuel drs c
        let (drs2, e2) := stepGameEvent fuel drs1 ev
        (drs2, e1 ++ e2)
  termination_by structural fuel

  def stepCausing (fuel : Nat) (drs : DRS) (c : Causing) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match c with
      | .source src =>
        let (drs', _, e) := stepNounPhrase fuel drs .other false src
        (drs', e)
      | .event ev => stepGameEvent fuel drs ev
      | .anEffect => (drs, [])
  termination_by structural fuel

  def stepWouldClause (fuel : Nat) (drs : DRS) (wc : WouldClause) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match wc with
      | .would subj action =>
        let headErr := if action.eventHeadOk then [] else [.eventHead]
        let (drs1, e1) := stepSubjectBefore fuel drs subj
        let (drs2, e2) := stepInstruction fuel .preEvent subj.patientRole drs1 action
        let (drs3, e3) := stepSubjectAfter fuel drs2 subj
        (drs3, headErr ++ e1 ++ e2 ++ e3)
      | .wouldAny cs => stepWouldClauseList fuel drs cs
  termination_by structural fuel

  def stepWouldClauseList (fuel : Nat) (drs : DRS) (cs : List WouldClause) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match cs with
      | [] => (drs, [])
      | c :: rest =>
        let (drs1, e1) := stepWouldClause fuel drs c
        let (drs2, e2) := stepWouldClauseList fuel drs1 rest
        (drs2, e1 ++ e2)
  termination_by structural fuel

  def stepCombatParticipation (fuel : Nat) (drs : DRS) (cp : CombatParticipation) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match cp with
      | .attacking (some defn) =>
        let (drs', _, e) := stepNounPhrase fuel drs .other true defn
        (drs', e)
      | .attacking none | .blocked _ | .outsideCombat => (drs, [])
  termination_by structural fuel

  def stepCombatUpdate (fuel : Nat) (drs : DRS) (cu : CombatUpdate) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match cu with
      | .participation state => stepCombatParticipation fuel drs state
      | .blocking _ atk =>
        let (drs', _, e) := stepNounPhrase fuel drs .other true atk
        (drs', e)
  termination_by structural fuel

  def stepDoor (fuel : Nat) (drs : DRS) (d : Door) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match d with
      | .thisDoor => (drs, [])
      | .doorOf _ room =>
        let (drs', _, e) := stepNounPhrase fuel drs .other false room
        (drs', e)
  termination_by structural fuel

  def stepDividedVerb (fuel : Nat) (drs : DRS) (v : DividedVerb) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match v with
      | .damage src =>
        let (drs', _, e) := stepNounPhrase fuel drs .other false src
        (drs', e)
      | .counters _ => (drs, [])
  termination_by structural fuel

  def stepCounterKindSource (fuel : Nat) (drs : DRS) (k : CounterKindSource) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match k with
      | .sameAs src =>
        let (drs', _, e) := stepNounPhrase fuel drs .other false src
        (drs', e)
      | _ => (drs, [])
  termination_by structural fuel

  def stepAbilityLost (fuel : Nat) (drs : DRS) (al : AbilityLost) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match al with
      | .written ab => (drs, checkAbility fuel drs ab)
      | .term _ => (drs, [])
  termination_by structural fuel

  def stepAbilityLostList (fuel : Nat) (drs : DRS) (als : List AbilityLost) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match als with
      | [] => (drs, [])
      | a :: rest =>
        let (drs1, e1) := stepAbilityLost fuel drs a
        let (drs2, e2) := stepAbilityLostList fuel drs1 rest
        (drs2, e1 ++ e2)
  termination_by structural fuel

  def stepAbilitySelection (fuel : Nat) (drs : DRS) (sel : AbilitySelection) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match sel with
      | .specified als => stepAbilityLostList fuel drs als
      | .allExcept (some p) => stepPredicate fuel drs p
      | .family _ | .allExcept none | .printedAfter _ => (drs, [])
  termination_by structural fuel

  def stepCharacteristicEdit (fuel : Nat) (drs : DRS) (ed : CharacteristicEdit) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match ed with
      | .stat _ _ amt => stepAmount fuel drs amt
      | .chosenQuality _ p => stepPredicate fuel drs p
      | .addedAbilities abs => stepAbilityList fuel drs abs
      | .removedAbilities sel => stepAbilitySelection fuel drs sel
      | _ => (drs, [])
  termination_by structural fuel

  def stepCharacteristicEditList (fuel : Nat) (drs : DRS) (eds : List CharacteristicEdit) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match eds with
      | [] => (drs, [])
      | ed :: rest =>
        let (drs1, e1) := stepCharacteristicEdit fuel drs ed
        let (drs2, e2) := stepCharacteristicEditList fuel drs1 rest
        (drs2, e1 ++ e2)
  termination_by structural fuel

  def stepCopyExcept (fuel : Nat) (drs : DRS) (ce : CopyExcept) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match ce with
      | .edits eds => stepCharacteristicEditList fuel drs eds
      | .ability ab => (drs, checkAbility fuel drs ab)
      | .thisAbility => (drs, [])
      | .entersWithCounters amt _ _ => stepAmount fuel drs amt
  termination_by structural fuel

  def stepCopyExceptList (fuel : Nat) (drs : DRS) (ces : List CopyExcept) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match ces with
      | [] => (drs, [])
      | c :: rest =>
        let (drs1, e1) := stepCopyExcept fuel drs c
        let (drs2, e2) := stepCopyExceptList fuel drs1 rest
        (drs2, e1 ++ e2)
  termination_by structural fuel

  def stepTextChange (fuel : Nat) (drs : DRS) (tc : TextChange) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match tc with
      | .appendTextOf src | .exchangeWith src =>
        let (drs', _, e) := stepNounPhrase fuel drs .other false src
        (drs', e)
      | .removeBracketed | .targetToEach | .replaceWord .. => (drs, [])
  termination_by structural fuel

  def stepSearchScope (fuel : Nat) (drs : DRS) (sc : SearchScope) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match sc with
      | .oneZone z => stepZoneExpr fuel drs z
      | .someZones (some whose) _ =>
        let (drs', _, e) := stepNounPhrase fuel drs .other false whose
        (drs', e)
      | .someZones none _ => (drs, [])
  termination_by structural fuel

  def stepVisibleThing (fuel : Nat) (drs : DRS) (vt : VisibleThing) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match vt with
      | .topOfLibrary | .wholeHand => (drs, [])
      | .objects np =>
        let (drs', _, e) := stepNounPhrase fuel drs .obj true np
        (drs', e)
  termination_by structural fuel

  def stepExchanged (fuel : Nat) (drs : DRS) (ex : Exchanged) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match ex with
      | .lifeTotals parties =>
        let (drs1, _, e1) := stepNounPhrase fuel drs .obj true parties
        let drs2 := (drs1.push (outcomeB .lifeGained) .other false).push (outcomeB .lifeLost) .other false
        (drs2, e1)
      | .controlOf l r | .cardsAcross l r =>
        let (drs1, _, e1) := stepNounPhrase fuel drs .obj true l
        let (drs2, _, e2) := stepNounPhrase fuel drs1 .other true r
        (drs2, e1 ++ e2)
      | .zones l r =>
        let (drs1, e1) := stepZoneExpr fuel drs l
        let (drs2, e2) := stepZoneExpr fuel drs1 r
        (drs2, e1 ++ e2)
      | .values l r =>
        let (drs1, e1) := stepAmount fuel drs l
        let (drs2, e2) := stepAmount fuel drs1 r
        (drs2, e1 ++ e2)
  termination_by structural fuel

  def stepFlipScope (fuel : Nat) (drs : DRS) (fs : FlipScope) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match fs with
      | .count a => stepAmount fuel drs a
      | .per np =>
        let (drs', _, e) := stepNounPhrase fuel drs .other false np
        (drs', e)
  termination_by structural fuel

  def stepIgnoredOutcomes (fuel : Nat) (drs : DRS) (io : IgnoredOutcomes) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match io with
      | .extreme _ | .allBut _ => (drs, [])
      | .chosen optChooser amt =>
        let (drs1, e1) := match optChooser with
          | some c => let (d, _, e) := stepNounPhrase fuel drs .subj true c; (d, e)
          | none => (drs, [])
        let (drs2, e2) := stepAmount fuel drs1 amt
        (drs2, e1 ++ e2)
  termination_by structural fuel

  def stepBallot (fuel : Nat) (drs : DRS) (b : Ballot) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match b with
      | .byLabel _ => (drs, [])
      | .byCandidate np =>
        let (drs', _, e) := stepNounPhrase fuel drs .obj true np
        (drs', e)
  termination_by structural fuel

  def stepRollRowList (fuel : Nat) (drs : DRS) (rows : List RollRow) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match rows with
      | [] => (drs, [])
      | ⟨q, inst⟩ :: rest =>
        let (drs1, e1) := stepQuantity fuel drs q
        let (drs2, e2) := stepInstruction fuel .postEvent .obj (nextClause drs1) inst
        let (drs3, e3) := stepRollRowList fuel drs2 rest
        (drs3, e1 ++ e2 ++ e3)
  termination_by structural fuel

  def stepRepetition (fuel : Nat) (drs : DRS) (rep : Repetition) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match rep with
      | .again | .anyNumber | .againExcludingChosen => (drs, [])
      | .moreTimes times => stepAmount fuel drs times
      | .untilCond cond => stepCondition fuel drs cond
      | .fixed times body =>
        let (drs1, e1) := stepAmount fuel drs times
        let (drs2, e2) := stepInstruction fuel .postEvent .obj (nextClause drs1) body
        let drs3 := drs2.push (outcomeB .repeatCount) .other false
        (drs3, e1 ++ e2)
  termination_by structural fuel

  def stepModeChange (fuel : Nat) (drs : DRS) (mc : ModeChange) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match mc with
      | .allInstead optCost optCond =>
        let (drs1, e1) := match optCost with | some c => stepCost fuel drs c | none => (drs, [])
        let (drs2, e2) := match optCond with | some cond => stepCondition fuel drs1 cond | none => (drs1, [])
        (drs2, e1 ++ e2)
      | .extraModesCost c => stepCost fuel drs c
      | .setQuantity q => stepQuantity fuel drs q
      | .repeatable => (drs, [])
      | .notChosenBefore dur => stepDuration fuel drs dur
  termination_by structural fuel

  def stepModeList (fuel : Nat) (drs : DRS) (modes : List (Option Cost × Option Nat × Instruction)) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match modes with
      | [] => (drs, [])
      | (optCost, _, inst) :: rest =>
        let (drs1, e1) := match optCost with | some c => stepCost fuel drs c | none => (drs, [])
        let (drs2, e2) := stepInstruction fuel .postEvent .obj (nextClause drs1) inst
        let (_, e3) := stepModeList fuel drs rest
        (drs2, e1 ++ e2 ++ e3)
  termination_by structural fuel

  def stepProducedMana (fuel : Nat) (drs : DRS) (pm : ProducedMana) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match pm with
      | .asPrintedCost subj | .producedByEvent subj | .couldProduce subj
      | .amongColorsOf subj | .lastNoted subj =>
        let (drs', _, e) := stepNounPhrase fuel drs .other false subj
        (drs', e)
      | _ => (drs, [])
  termination_by structural fuel

  def stepSpendPurpose (fuel : Nat) (drs : DRS) (sp : SpendPurpose) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match sp with
      | .toCast p | .toActivate (some p) => stepPredicate fuel drs p
      | .toActivate none | .toPay _ => (drs, [])
  termination_by structural fuel

  def stepSpendPurposeList (fuel : Nat) (drs : DRS) (sps : List SpendPurpose) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match sps with
      | [] => (drs, [])
      | sp :: rest =>
        let (drs1, e1) := stepSpendPurpose fuel drs sp
        let (drs2, e2) := stepSpendPurposeList fuel drs1 rest
        (drs2, e1 ++ e2)
  termination_by structural fuel

  def stepManaRider (fuel : Nat) (drs : DRS) (mr : ManaRider) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match mr with
      | .spendOnly sps | .spendNotOn sps => stepSpendPurposeList fuel drs sps
      | .onSpent _ _ sp says =>
        let (drs1, _, e1) := stepNounPhrase fuel drs .subj true sp
        let (drs2, e2) := stepInstruction fuel .postEvent .obj (nextClause drs1) says
        (drs2, e1 ++ e2)
  termination_by structural fuel

  def stepManaRiderList (fuel : Nat) (drs : DRS) (mrs : List ManaRider) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match mrs with
      | [] => (drs, [])
      | mr :: rest =>
        let (drs1, e1) := stepManaRider fuel drs mr
        let (drs2, e2) := stepManaRiderList fuel drs1 rest
        (drs2, e1 ++ e2)
  termination_by structural fuel

  def stepPipPaidList (fuel : Nat) (drs : DRS) (ps : List PipPaid) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match ps with
      | [] => (drs, [])
      | .generic :: rest => stepPipPaidList fuel drs rest
      | .ofColor ct :: rest =>
        let (drs1, e1) := stepColorTerm fuel drs ct
        let (drs2, e2) := stepPipPaidList fuel drs1 rest
        (drs2, e1 ++ e2)
  termination_by structural fuel

  def stepManaSubstitute (fuel : Nat) (drs : DRS) (ms : ManaSubstitute) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      let (drs1, e1) := stepInstruction fuel .postEvent .obj drs ms.action
      let (drs2, e2) := stepPipPaidList fuel drs1 ms.pays
      (drs2, e1 ++ e2)
  termination_by structural fuel

  def stepCostShift (fuel : Nat) (drs : DRS) (cs : CostShift) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match cs with
      | .less amt optFl =>
        let (drs1, e1) := stepAmount fuel drs amt
        let (drs2, e2) := match optFl with | some fl => stepAmount fuel drs1 fl | none => (drs1, [])
        (drs2, e1 ++ e2)
      | .more amt => stepAmount fuel drs amt
      | .bySymbols (.lit _) _ _ => (drs, [])
      | .bySymbols (.manaCostOf obj) _ _ =>
        let (drs', _, e) := stepNounPhrase fuel drs .other false obj
        (drs', e)
  termination_by structural fuel

  def stepDamageAgent (fuel : Nat) (drs : DRS) (da : DamageAgent) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match da with
      | .unattributed => (drs, [])
      | .dealtBy src =>
        let (drs', _, e) := stepNounPhrase fuel drs .subj true src
        (drs', e)
  termination_by structural fuel

  def stepDamageScope (fuel : Nat) (drs : DRS) (ds : DamageScope) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match ds with
      | .everywhere => (drs, [])
      | .toRecipient rec =>
        let (drs', _, e) := stepNounPhrase fuel drs .obj true rec
        (drs', e)
  termination_by structural fuel

  def stepPreventCut (fuel : Nat) (drs : DRS) (pc : PreventCut) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match pc with
      | .all | .half _ => (drs, [])
      | .some amt | .shield amt | .allBut amt => stepAmount fuel drs amt
  termination_by structural fuel

  def stepDamageOp (fuel : Nat) (drs : DRS) (dop : DamageOp) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match dop with
      | .prevent cut optAlso =>
        let (drs1, e1) := stepPreventCut fuel drs cut
        let drs2 := drs1.push (outcomeB .damagePrevented) .other false
        let (drs3, e2) := match optAlso with
          | some inst => stepInstruction fuel .postEvent .obj (nextClause drs2) inst
          | none => (drs2, [])
        (drs3, e1 ++ e2)
      | .redirect cut to =>
        let (drs1, e1) := stepPreventCut fuel drs cut
        let (drs2, _, e2) := stepNounPhrase fuel drs1 .other true to
        (drs2, e1 ++ e2)
      | .scale (.multiplied _) | .scale (.halved _) => (drs, [])
      | .scale (.shifted _ amt) => stepAmount fuel drs amt
  termination_by structural fuel

  def stepInstruction (fuel : Nat) (phase : StepPhase) (patientRole : GramRole) (drs : DRS) (inst : Instruction) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match inst with
      | .inCaller body =>
        let callerDrs := enterCaller drs
        let (callerDrs', errs) := stepInstruction fuel phase patientRole callerDrs body
        (leaveCaller drs callerDrs', errs)
      | .ownScope deed body =>
        let innerDrs := enterOwnScope drs
        let (innerDrs', errs) := stepInstruction fuel phase patientRole innerDrs body
        let outDrs := leaveOwnScope innerDrs'
        let _ := deed
        (outDrs, errs)
      | .bracketed body => stepInstruction fuel phase patientRole drs body
      | .enact verb body becomes locus =>
        let preIds := drs.pendingRoles.map Prod.fst
        let (drs1, e1) := stepInstruction fuel phase patientRole drs body
        let actedIds := (drs1.pendingRoles.map Prod.fst).filter (!preIds.elem ·)
        let drs1 := if phase == .postEvent then applyInPlaceStamp drs1 actedIds (some verb) else drs1
        let (drs2, e2) := match becomes with | some p => stepPredicate fuel drs1 p | none => (drs1, [])
        let (drs3, e3) := match locus with | some z => stepZoneExpr fuel drs2 z | none => (drs2, [])
        (drs3, e1 ++ e2 ++ e3)
      | .instruct subj body =>
        let (drs1, refIds, e1) := stepNounPhrase fuel drs .subj true subj
        let shape : ActorShape := {
          kind := subj.kindIn drs1 .player,
          isYou := (subj matches .you),
          selfDefined := false, ascribable := false, twoParties := false, opponentOnly := false,
          refIds := refIds
        }
        let drs2 := drs1.push { det := .the, plur := .one, payload := .actor shape } .other false
        let (drs3, e2) := stepInstruction fuel phase .obj drs2 body
        (dropActorFrame drs3, e1 ++ e2)
      | .dealDamage amt recipient _ =>
        let (drs1, e1) := stepAmount fuel drs amt
        let (drs2, _, e2) := stepNounPhrase fuel drs1 patientRole true recipient
        let drs3 := drs2.push (outcomeB .damageDealt) .other false
        (drs3, e1 ++ e2)
      | .markDamage subj amt | .skipUntap subj amt =>
        let (drs1, _, e1) := stepNounPhrase fuel drs patientRole true subj
        let (drs2, e2) := stepAmount fuel drs1 amt
        (drs2, e1 ++ e2)
      | .skipPart _ cnt | .addTurnPart _ _ cnt _ => stepAmount fuel drs cnt
      | .clearDamage subj | .turnOver subj | .turnFaceDown subj _
      | .target subj | .changeControl subj | .chooseNewTargets subj =>
        let (drs1, refIds, e1) := stepNounPhrase fuel drs patientRole true subj
        let drs2 := if phase == .postEvent then applyInPlaceStamp drs1 refIds inst.inferredDeed else drs1
        (drs2, e1)
      | .tapForMana perm optType =>
        let (drs1, _, e1) := stepNounPhrase fuel drs patientRole true perm
        let (drs2, e2) := match optType with
          | some (.ofColor ct) => stepColorTerm fuel drs1 ct
          | _ => (drs1, [])
        (drs2, e1 ++ e2)
      | .gainDesignation subj _ optDur =>
        let (drs1, refIds, e1) := stepNounPhrase fuel drs patientRole true subj
        let (drs2, e2) := match optDur with | some d => stepDuration fuel drs1 d | none => (drs1, [])
        let drs3 := if phase == .postEvent then applyInPlaceStamp drs2 refIds inst.inferredDeed else drs2
        (drs3, e1 ++ e2)
      | .doAndForbid body _ subj =>
        let (drs1, e1) := stepInstruction fuel phase patientRole drs body
        let (drs2, _, e2) := stepNounPhrase fuel drs1 .other true subj
        (drs2, e1 ++ e2)
      | .setStatus _ subj =>
        let (drs1, refIds, e1) := stepNounPhrase fuel drs patientRole true subj
        let drs2 := if phase == .postEvent then applyInPlaceStamp drs1 refIds inst.inferredDeed else drs1
        (drs2, e1)
      | .combat subj upd =>
        let (drs1, _, e1) := stepNounPhrase fuel drs patientRole true subj
        let (drs2, e2) := stepCombatUpdate fuel drs1 upd
        (drs2, e1 ++ e2)
      | .attack (some defn) | .attackUnblocked (some defn) =>
        let (drs1, _, e1) := stepNounPhrase fuel drs patientRole true defn
        (drs1, e1)
      | .attack none | .attackUnblocked none => (drs, [])
      | .attackWith atks optDef =>
        let (drs1, _, e1) := stepNounPhrase fuel drs patientRole true atks
        let (drs2, e2) := match optDef with
          | some d => let (d', _, e) := stepNounPhrase fuel drs1 .other true d; (d', e)
          | none => (drs1, [])
        (drs2, e1 ++ e2)
      | .block atk =>
        let (drs1, _, e1) := stepNounPhrase fuel drs patientRole true atk
        (drs1, e1)
      | .attachment _ subj optHost =>
        let (drs1, _, e1) := stepNounPhrase fuel drs patientRole true subj
        let (drs2, e2) := match optHost with
          | some h => let (d', _, e) := stepNounPhrase fuel drs1 .other true h; (d', e)
          | none => (drs1, [])
        (drs2, e1 ++ e2)
      | .unlock d => stepDoor fuel drs d
      | .setGameDesignation _ | .commitCrime | .conclude _ | .drawGame | .restartGame | .revealChoices _ => (drs, [])
      | .separateIntoPiles grp piles faces =>
        let (drs1, grpIds, e1) := stepNounPhrase fuel drs patientRole true grp
        let grpZone := (grpIds.head?.bind (drs1.findById?)).bind (·.zone)
        let drs2 := partsClosed drs1
        let pileRef : Referent := {
          det := .the, plur := .many,
          payload := .pile grpZone (some piles) (pileMentionFace faces)
        }
        let drs3 := if phase == .postEvent then drs2.push pileRef .other false else drs1
        let e2 := if facesFit faces piles then [] else [.facesFit]
        (drs3, e1 ++ e2)
      | .relate subj _ cp | .merge subj cp _ | .copyTargets subj cp =>
        let (drs1, _, e1) := stepNounPhrase fuel drs patientRole true subj
        let (drs2, _, e2) := stepNounPhrase fuel drs1 .other true cp
        (drs2, e1 ++ e2)
      | .choose first chosen _ optWhen =>
        let (drs1, e1) := match first with
          | some f => let (d, _, e) := stepNounPhrase fuel drs .subj true f; (d, e)
          | none => (drs, [])
        let (drs2, _, e2) := stepNounPhrase fuel drs1 patientRole true chosen
        let (drs3, e3) := match optWhen with | some t => stepTiming fuel drs2 t | none => (drs2, [])
        (drs3, e1 ++ e2 ++ e3)
      | .vote first _ ballot =>
        let (drs1, e1) := match first with
          | some f => let (d, _, e) := stepNounPhrase fuel drs .subj true f; (d, e)
          | none => (drs, [])
        let (drs2, e2) := stepBallot fuel drs1 ballot
        let drs3 := drs2.push (outcomeB .voteHeld) .other false
        (drs3, e1 ++ e2)
      | .move subj from_ to _ optCause riders =>
        let (drs1, refIds, e1) := stepNounPhrase fuel drs patientRole true subj
        let (drs2, e2) := stepEventSource fuel drs1 from_
        let (drs3, e3) := match to with | some z => stepZoneExpr fuel drs2 z | none => (drs2, [])
        let (drs4, e4) := match optCause with | some c => stepCausing fuel drs3 c | none => (drs3, [])
        let (drs5, e5) := stepMoveRiderList fuel drs4 riders
        let destZone := to.bind (·.sort)
        let drs6 := if phase == .postEvent then
          applyZoneMove drs5 refIds inst.inferredDeed destZone
        else drs5
        (drs6, e1 ++ e2 ++ e3 ++ e4 ++ e5)
      | .copy _ subj times exs =>
        let (drs1, _, e1) := stepNounPhrase fuel drs patientRole true subj
        let (drs2, e2) := stepAmount fuel drs1 times
        let (drs3, e3) := stepCopyExceptList fuel drs2 exs
        let drs4 := if phase == .postEvent then
          let copyRef : Referent := {
            det := .the, plur := .one,
            payload := .object [] (some .stack) (mkStamp (some (.core .copy)) (some .stack) false) (some .copy) none none
          }
          drs3.push copyRef patientRole true
        else drs3
        (drs4, e1 ++ e2 ++ e3)
      | .changeLife delta =>
        let (drs1, e1) := stepDelta fuel drs delta
        let drs2 := match delta.outcome with
          | some out => drs1.push (outcomeB out) .other false
          | none => drs1
        (drs2, e1)
      | .exchange ex => stepExchanged fuel drs ex
      | .addMana amt prod riders =>
        let (drs1, e1) := stepAmount fuel drs amt
        let (drs2, e2) := stepProducedMana fuel drs1 prod
        let (drs3, e3) := stepManaRiderList fuel drs2 riders
        (drs3, e1 ++ e2 ++ e3)
      | .draw amt =>
        let (drs1, e1) := stepAmount fuel drs amt
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
          let (drs1, refIds, e1) := stepNounPhrase fuel drs patientRole true np
          let drs2 := if phase == .postEvent then applyInPlaceStamp drs1 refIds inst.inferredDeed else drs1
          (drs2, e1)
        | .zone z => stepZoneExpr fuel drs z
        | .choice _ => (drs, [])
      | .search sc q p =>
        let (drs1, e1) := stepSearchScope fuel drs sc
        let (drs2, e2) := stepQuantity fuel drs1 q
        let (drs3, e3) := stepPredicate fuel drs2 p
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
        let (drs1, e1) := stepPile fuel drs pile
        let drs2 := if phase == .postEvent then afterShuffle drs1 else drs1
        (drs2, e1)
      | .flipCoins count _ _ =>
        let (drs1, e1) := stepFlipScope fuel drs count
        let drs2 := drs1.push (outcomeB .coinFlipped) .other false
        (drs2, e1)
      | .rollDice count _ _ watch =>
        let (drs1, e1) := stepAmount fuel drs count
        let (drs2, e2) := match watch with
          | .anyResult | .highestNatural => (drs1, [])
          | .resultIn q => stepQuantity fuel drs1 q
        let drs3 := drs2.push (outcomeB .rollResult) .other false
        (drs3, e1 ++ e2)
      | .applyResultsTable rows =>
        let cnt := countOutcomes .rollResult drs
        let e0 := if cnt == 1 then [] else [.outcomeInScope .rollResult cnt]
        let (drs1, e1) := stepRollRowList fuel drs rows
        (drs1, e0 ++ e1)
      | .ignoreOutcomes which =>
        let (drs1, e1) := stepIgnoredOutcomes fuel drs which
        let e2 := if ignorableInScope drs1 then [] else [.ignorableFor]
        (drs1, e1 ++ e2)
      | .shiftResult _ amt =>
        let (drs1, e1) := stepAmount fuel drs amt
        let cnt := countOutcomes .rollResult drs1
        let e2 := if cnt == 1 then [] else [.outcomeInScope .rollResult cnt]
        (drs1, e1 ++ e2)
      | .storeResults on =>
        let (drs1, _, e1) := stepNounPhrase fuel drs patientRole true on
        let cnt := countOutcomes .rollResult drs1
        let e2 := if cnt == 1 then [] else [.outcomeInScope .rollResult cnt]
        (drs1, e1 ++ e2)
      | .rerollStored q whose =>
        let (drs1, e1) := stepQuantity fuel drs q
        let (drs2, _, e2) := stepNounPhrase fuel drs1 .other false whose
        (drs2, e1 ++ e2)
      | .establish spec dur =>
        let (drs1, e1) := stepStaticSpec fuel drs spec
        let (drs2, e2) := match dur with | some d => stepDuration fuel drs1 d | none => (drs1, [])
        (drs2, e1 ++ e2)
      | .createObject cnt spec _ optUnder =>
        let (drs1, e1) := stepAmount fuel drs cnt
        let (drs2, e2) := stepCreationSpec fuel drs1 spec
        let (drs3, e3) := match optUnder with
          | some u => let (d', _, e) := stepNounPhrase fuel drs2 .other false u; (d', e)
          | none => (drs2, [])
        let drs4 := if phase == .postEvent then
          let tokRef : Referent := {
            det := .the, plur := .one,
            payload := .object [.creature] (some .battlefield) (mkStamp (some (.action "create")) (some .battlefield) false) (some .token) none none
          }
          drs3.push tokRef patientRole true
        else drs3
        (drs4, e1 ++ e2 ++ e3)
      | .putCounters amt ks on _ _ =>
        let (drs1, e1) := stepAmount fuel drs amt
        let (drs2, e2) := stepCounterKindSource fuel drs1 ks
        let (drs3, _, e3) := stepNounPhrase fuel drs2 patientRole true on
        let drs4 := drs3.push (outcomeB .countersPut) .other false
        (drs4, e1 ++ e2 ++ e3)
      | .removeCounters optQ optKs from_ _ _ =>
        let (drs1, e1) := match optQ with | some q => stepQuantity fuel drs q | none => (drs, [])
        let (drs2, e2) := match optKs with | some ks => stepCounterKindSource fuel drs1 ks | none => (drs1, [])
        let (drs3, _, e3) := stepNounPhrase fuel drs2 patientRole true from_
        let drs4 := drs3.push (outcomeB .countersRemoved) .other false
        (drs4, e1 ++ e2 ++ e3)
      | .distribute v amt among =>
        let (drs1, e1) := stepDividedVerb fuel drs v
        let (drs2, e2) := stepAmount fuel drs1 amt
        let (drs3, _, e3) := stepNounPhrase fuel drs2 patientRole true among
        (drs3, e1 ++ e2 ++ e3)
      | .moveCounters amt optKs src dst =>
        let (drs1, e1) := stepAmount fuel drs amt
        let (drs2, e2) := match optKs with | some ks => stepCounterKindSource fuel drs1 ks | none => (drs1, [])
        let (drs3, _, e3) := stepNounPhrase fuel drs2 patientRole true src
        let (drs4, _, e4) := stepNounPhrase fuel drs3 .other true dst
        (drs4, e1 ++ e2 ++ e3 ++ e4)
      | .doubleCounters on =>
        let (drs1, _, e1) := stepNounPhrase fuel drs patientRole true on
        (drs1, e1)
      | .pay cost _ _ whose _ =>
        let (drs1, e1) := match whose with
          | some w => let (d, _, e) := stepNounPhrase fuel drs .subj true w; (d, e)
          | none => (drs, [])
        let (drs2, e2) := stepCost fuel drs1 cost
        (drs2, e1 ++ e2)
      | .withContinuation _ body ifDid ifNot =>
        let (drs1, e1) := stepInstruction fuel phase patientRole drs body
        let (drs2, e2) := match ifDid with | some b => stepInstruction fuel phase .obj (nextClause drs1) b | none => (drs1, [])
        let (_, e3) := match ifNot with | some b => stepInstruction fuel phase .obj (nextClause drs1) b | none => (drs1, [])
        (drs2, e1 ++ e2 ++ e3)
      | .doOnlyIf inst cond otherwise =>
        let (drs1, e1) := stepInstruction fuel phase patientRole drs inst
        let (drs2, e2) := stepCondition fuel drs1 cond
        let (_, e3) := match otherwise with | some b => stepInstruction fuel phase .obj (nextClause drs2) b | none => (drs2, [])
        (drs2, e1 ++ e2 ++ e3)
      | .limited inst _ => stepInstruction fuel phase patientRole drs inst
      | .doIf cond thenInst elseInst =>
        let (drs1, e1) := stepCondition fuel drs cond
        let (drs2, e2) := stepInstruction fuel phase .obj (nextClause drs1) thenInst
        match elseInst with
        | none => (drs2, e1 ++ e2)
        | some el =>
          let (_, e3) := stepInstruction fuel phase .obj (nextClause drs) el
          (drs, e1 ++ e2 ++ e3)
      | .doForEach grp body =>
        let (drs1, grpIds, e1) := stepNounPhrase fuel drs .other false grp
        let preLoopMaxId := drs1.nextId
        let grpZone := (grpIds.head?.bind (drs1.findById?)).bind (·.zone)
        let grpTy := (grpIds.head?.bind (drs1.findById?)).map (·.ty) |>.getD []
        let loopRef : Referent := {
          det := .each, plur := .one,
          payload := .object grpTy grpZone none none none none
        }
        let drs2 := drs1.push loopRef .subj true
        let (drs3, e2) := stepInstruction fuel phase .obj (nextClause drs2) body
        let closedDrs : DRS := { drs3 with referents := drs3.referents.filter (·.id < preLoopMaxId) }
        (closedDrs, e1 ++ e2)
      | .doForEachKind _ dom q body =>
        let (drs1, e1) := match dom with
          | some d => let (d', _, e) := stepNounPhrase fuel drs .other false d; (d', e)
          | none => (drs, [])
        let preLoopMaxId := drs1.nextId
        let drs2 := drs1.push { det := .each, plur := .one, payload := .quality q } .other false
        let (drs3, e2) := stepInstruction fuel phase .obj (nextClause drs2) body
        let closedDrs : DRS := { drs3 with referents := drs3.referents.filter (·.id < preLoopMaxId) }
        (closedDrs, e1 ++ e2)
      | .repeat_ rep => stepRepetition fuel drs rep
      | .sequentially steps => stepInstructionSeq fuel phase drs steps
      | .simultaneously steps => stepInstructionSimul fuel phase drs steps
      | .chooseModes q modes =>
        let (drs1, e1) := stepQuantity fuel drs q
        let (drs2, e2) := stepModeList fuel drs1 modes
        (drs2, e1 ++ e2)
      | .delay ev alts optDur body =>
        let (drs1, e1) := stepGameEvent fuel drs ev
        let (_, eAlts) := stepGameEventList fuel drs alts
        let (drs2, e2) := match optDur with | some d => stepDuration fuel drs1 d | none => (drs1, [])
        let (drs3, e3) := stepInstruction fuel phase .obj (nextClause drs2) body
        (drs3, e1 ++ eAlts ++ e2 ++ e3)
      | .replace repld repl =>
        let (drs1, e1) := stepInstruction fuel .preEvent patientRole drs repld
        let (drs2, e2) := stepInstruction fuel .postEvent .obj (nextClause drs1) repl
        (drs2, e1 ++ e2)
      | .holdUntil inst ev =>
        let (drs1, e1) := stepInstruction fuel phase patientRole drs inst
        let (drs2, e2) := stepGameEvent fuel drs1 ev
        (drs2, e1 ++ e2)
      | .triggerReflexively body trig =>
        let (drs1, e1) := stepInstruction fuel phase patientRole drs body
        let (drs2, e2) := stepInstruction fuel .postEvent .obj (nextClause drs1) trig
        (drs2, e1 ++ e2)
      | .triggerThisWay body ev trig =>
        let (drs1, e1) := stepInstruction fuel phase patientRole drs body
        let (drs2, e2) := stepGameEvent fuel (nextClause drs1) ev
        let (drs3, e3) := stepInstruction fuel .postEvent .obj (nextClause drs2) trig
        (drs3, e1 ++ e2 ++ e3)
      | .procedure _ subj pay res from_ of_ =>
        let (drs1, refIds, e1) := stepNounPhrase fuel drs patientRole true subj
        let (drs2, e2) := stepPlayPayment fuel drs1 pay
        let (drs3, e3) := match res with | some p => stepPredicate fuel drs2 p | none => (drs2, [])
        let (drs4, e4) := match from_ with | some f => stepEventSource fuel drs3 f | none => (drs3, [])
        let (drs5, e5) := match of_ with
          | some o => let (d, _, e) := stepNounPhrase fuel drs4 .other false o; (d, e)
          | none => (drs4, [])
        let drs6 := if phase == .postEvent then applyZoneMove drs5 refIds (some (.action "cast")) (some .stack) else drs5
        (drs6, e1 ++ e2 ++ e3 ++ e4 ++ e5)
  termination_by structural fuel

  def stepInstructionSeq (fuel : Nat) (phase : StepPhase) (drs : DRS) (steps : List Instruction) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match steps with
      | [] => (drs, [])
      | [i] => stepInstruction fuel phase .obj drs i
      | i :: rest =>
        let (drs1, e1) := stepInstruction fuel phase .obj drs i
        let (drs2, e2) := stepInstructionSeq fuel phase (nextClause drs1) rest
        (drs2, e1 ++ e2)
  termination_by structural fuel

  def stepInstructionSimul (fuel : Nat) (phase : StepPhase) (drs : DRS) (steps : List Instruction) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match steps with
      | [] => (drs, [])
      | i :: rest =>
        let (drs1, e1) := stepInstruction fuel phase .obj drs i
        let (drs2, e2) := stepInstructionSimul fuel phase drs1 rest
        (drs2, e1 ++ e2)
  termination_by structural fuel

  def stepEntryRider (fuel : Nat) (drs : DRS) (r : EntryRider) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match r with
      | .entersAs _ | .entersTransformed | .entersMelded _ | .faceDown _ => (drs, [])
      | .entersAttacking (some defn) =>
        let (drs', _, e) := stepNounPhrase fuel drs .other true defn
        (drs', e)
      | .entersAttacking none => (drs, [])
      | .withCounters amt ks _ =>
        let (drs1, e1) := stepAmount fuel drs amt
        let (drs2, e2) := stepCounterKindSource fuel drs1 ks
        (drs2, e1 ++ e2)
      | .under ctrl =>
        let (drs', _, errs) := stepNounPhrase fuel drs .other false ctrl
        (drs', errs)
      | .asCopyOf _ src exs =>
        let (drs1, _, e1) := stepNounPhrase fuel drs .other false src
        let (drs2, e2) := stepCopyExceptList fuel drs1 exs
        (drs2, e1 ++ e2)
  termination_by structural fuel

  def stepEntryRiderList (fuel : Nat) (drs : DRS) (rs : List EntryRider) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match rs with
      | [] => (drs, [])
      | r :: rest =>
        let (drs1, e1) := stepEntryRider fuel drs r
        let (drs2, e2) := stepEntryRiderList fuel drs1 rest
        (drs2, e1 ++ e2)
  termination_by structural fuel

  def stepPilePlacement (fuel : Nat) (drs : DRS) (pp : PilePlacement) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match pp with
      | .inOrder _ | .nthFromEnd _ | .orOtherEnd | .faceDown _ => (drs, [])
      | .withCounters amt ks _ =>
        let (drs1, e1) := stepAmount fuel drs amt
        let (drs2, e2) := stepCounterKindSource fuel drs1 ks
        (drs2, e1 ++ e2)
  termination_by structural fuel

  def stepMoveRider (fuel : Nat) (drs : DRS) (mr : MoveRider) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match mr with
      | .entry r => stepEntryRider fuel drs r
      | .placement pp => stepPilePlacement fuel drs pp
  termination_by structural fuel

  def stepMoveRiderList (fuel : Nat) (drs : DRS) (mrs : List MoveRider) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match mrs with
      | [] => (drs, [])
      | mr :: rest =>
        let (drs1, e1) := stepMoveRider fuel drs mr
        let (drs2, e2) := stepMoveRiderList fuel drs1 rest
        (drs2, e1 ++ e2)
  termination_by structural fuel

  def stepTokenSpec (fuel : Nat) (drs : DRS) (ts : TokenSpec) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match ts with
      | .written _ | .asThose | .doubleFaced _ _ => (drs, [])
      | .copyOf src exs =>
        let (drs1, _, e1) := stepNounPhrase fuel drs .other false src
        let (drs2, e2) := stepCopyExceptList fuel drs1 exs
        (drs2, e1 ++ e2)
  termination_by structural fuel

  def stepCreationSpec (fuel : Nat) (drs : DRS) (spec : CreationSpec) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match spec with
      | .token ts riders =>
        let (drs1, e1) := stepTokenSpec fuel drs ts
        let (drs2, e2) := stepEntryRiderList fuel drs1 riders
        (drs2, e1 ++ e2)
      | .emblem _ abs => stepAbilityList fuel drs abs
  termination_by structural fuel

  def stepPlayPayment (fuel : Nat) (drs : DRS) (pay : PlayPayment) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match pay with
      | .itsOwnCost | .withoutPaying => (drs, [])
      | .payingInstead c | .faceDown _ c => stepCost fuel drs c
  termination_by structural fuel

  def stepCost (fuel : Nat) (drs : DRS) (c : Cost) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match c with
      | .inCaller body =>
        let (callerDrs, errs) := stepCost fuel (enterCaller drs) body
        (leaveCaller drs callerDrs, errs)
      | .mana _ | .tapSymbol | .untapSymbol | .loyaltySymbol _ | .itsManaCost => (drs, [])
      | .scaled c amt =>
        let (drs1, e1) := stepCost fuel drs c
        let (drs2, e2) := stepAmount fuel drs1 amt
        (drs2, e1 ++ e2)
      | .perform inst => stepInstruction fuel .postEvent .obj drs inst
      | .compound cs | .or cs => stepCostList fuel drs cs
      | .withSubstitute c sub =>
        let (drs1, e1) := stepCost fuel drs c
        let (drs2, e2) := stepManaSubstitute fuel drs1 sub
        (drs2, e1 ++ e2)
  termination_by structural fuel

  def stepCostList (fuel : Nat) (drs : DRS) (cs : List Cost) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match cs with
      | [] => (drs, [])
      | c :: rest =>
        let (drs1, e1) := stepCost fuel drs c
        let (drs2, e2) := stepCostList fuel drs1 rest
        (drs2, e1 ++ e2)
  termination_by structural fuel

  def stepDelta (fuel : Nat) (drs : DRS) (d : Delta Amount) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match d with
      | .up a | .down a | .set a => stepAmount fuel drs a
  termination_by structural fuel

  def stepStaticSpec (fuel : Nat) (drs : DRS) (s : StaticSpec) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match s with
      | .inCaller body =>
        let (callerDrs, errs) := stepStaticSpec fuel (enterCaller drs) body
        (leaveCaller drs callerDrs, errs)
      | .modification subj _ d =>
        let (drs1, _, e1) := stepNounPhrase fuel drs .subj true subj
        let (drs2, e2) := stepDelta fuel drs1 d
        (drs2, e1 ++ e2)
      | .ptModification subj p t =>
        let (drs1, _, e1) := stepNounPhrase fuel drs .subj true subj
        let (drs2, e2) := stepDelta fuel drs1 p
        let (drs3, e3) := stepDelta fuel drs2 t
        (drs3, e1 ++ e2 ++ e3)
      | .ptDefinition subj _ amt =>
        let (drs1, _, e1) := stepNounPhrase fuel drs .subj true subj
        let (drs2, e2) := stepAmount fuel drs1 amt
        (drs2, e1 ++ e2)
      | .ptSwitch subj | .manaRetention subj _ | .partSkip subj _ | .combatant subj =>
        let (drs1, _, e1) := stepNounPhrase fuel drs .subj true subj
        (drs1, e1)
      | .costShift subj sh =>
        let (drs1, _, e1) := stepNounPhrase fuel drs .subj true subj
        let (drs2, e2) := stepCostShift fuel drs1 sh
        (drs2, e1 ++ e2)
      | .altCost subj optCost =>
        let (drs1, _, e1) := stepNounPhrase fuel drs .subj true subj
        let (drs2, e2) := match optCost with | some c => stepCost fuel drs1 c | none => (drs1, [])
        (drs2, e1 ++ e2)
      | .addedCost c _ _ => stepCost fuel drs c
      | .paySubstitute subj sub =>
        let (drs1, _, e1) := stepNounPhrase fuel drs .subj true subj
        let (drs2, e2) := stepManaSubstitute fuel drs1 sub
        (drs2, e1 ++ e2)
      | .letterDefinition l amt =>
        let (drs1, e1) := stepAmount fuel drs amt
        (defineLetter l drs1, e1)
      | .abilityGrant subj ab =>
        let (drs1, _, e1) := stepNounPhrase fuel drs .subj true subj
        let e2 := checkAbility fuel drs1 ab
        (drs1, e1 ++ e2)
      | .abilityGrantFrom subj _ src optEx =>
        let (drs1, _, e1) := stepNounPhrase fuel drs .subj true subj
        let (drs2, _, e2) := stepNounPhrase fuel drs1 .other false src
        let (drs3, e3) := match optEx with | some p => stepPredicate fuel drs2 p | none => (drs2, [])
        (drs3, e1 ++ e2 ++ e3)
      | .abilityGrantSelected subj src sel =>
        let (drs1, _, e1) := stepNounPhrase fuel drs .subj true subj
        let (drs2, _, e2) := stepNounPhrase fuel drs1 .other false src
        let (drs3, e3) := stepAbilitySelection fuel drs2 sel
        (drs3, e1 ++ e2 ++ e3)
      | .characteristicChange subj edits =>
        let (drs1, _, e1) := stepNounPhrase fuel drs .subj true subj
        let (drs2, e2) := stepCharacteristicEditList fuel drs1 edits
        (drs2, e1 ++ e2)
      | .copyChange subj src exs =>
        let (drs1, _, e1) := stepNounPhrase fuel drs .subj true subj
        let (drs2, _, e2) := stepNounPhrase fuel drs1 .other false src
        let (drs3, e3) := stepCopyExceptList fuel drs2 exs
        (drs3, e1 ++ e2 ++ e3)
      | .retention spec subj =>
        let (drs1, e1) := stepStaticSpec fuel drs spec
        let (drs2, _, e2) := stepNounPhrase fuel drs1 .other false subj
        (drs2, e1 ++ e2)
      | .addsTarget subj desc =>
        let (drs1, _, e1) := stepNounPhrase fuel drs .subj true subj
        let (drs2, e2) := stepPredicate fuel drs1 desc
        (drs2, e1 ++ e2)
      | .textChange subj tc =>
        let (drs1, _, e1) := stepNounPhrase fuel drs .subj true subj
        let (drs2, e2) := stepTextChange fuel drs1 tc
        (drs2, e1 ++ e2)
      | .controlGrant player subj =>
        let (drs1, _, e1) := stepNounPhrase fuel drs .subj true player
        let (drs2, _, e2) := stepNounPhrase fuel drs1 .obj true subj
        (drs2, e1 ++ e2)
      | .entryRider subj r =>
        let (drs1, _, e1) := stepNounPhrase fuel drs .subj true subj
        let (drs2, e2) := stepEntryRider fuel drs1 r
        (drs2, e1 ++ e2)
      | .choice _ subj q dom _ =>
        let (drs1, _, e1) := stepNounPhrase fuel drs .subj true subj
        let (drs2, e2) := match dom with | some d => stepChoiceDomain fuel drs1 d | none => (drs1, [])
        let choiceRef : Referent := {
          det := .the, plur := .one,
          payload := match q with | .player => .player true | .quality qs => .quality qs
        }
        let drs3 := drs2.push choiceRef .other false
        (drs3, e1 ++ e2)
      | .modeChoice mc => stepModeChange fuel drs mc
      | .cant subj action _ =>
        let headErr := if action.eventHeadOk then [] else [.eventHead]
        let (drs1, e1) := stepSubjectBefore fuel drs subj
        let (drs2, e2) := stepInstruction fuel .postEvent subj.patientRole drs1 action
        let (_, e3) := stepSubjectAfter fuel drs2 subj
        (drs, headErr ++ e1 ++ e2 ++ e3)
      | .must subj action _ | .may subj action _ =>
        let headErr := if action.eventHeadOk then [] else [.eventHead]
        let (drs1, e1) := stepSubjectBefore fuel drs subj
        let (drs2, e2) := stepInstruction fuel .postEvent subj.patientRole drs1 action
        let (drs3, e3) := stepSubjectAfter fuel drs2 subj
        (drs3, headErr ++ e1 ++ e2 ++ e3)
      | .gatedBy cost subj action _ =>
        let headErr := if action.eventHeadOk then [] else [.eventHead]
        let (drs1, e1) := stepCost fuel drs cost
        let (drs2, e2) := stepSubjectBefore fuel (nextClause drs1) subj
        let (drs3, e3) := stepInstruction fuel .postEvent subj.patientRole drs2 action
        let (drs4, e4) := stepSubjectAfter fuel drs3 subj
        (drs4, headErr ++ e1 ++ e2 ++ e3 ++ e4)
      | .instead ev repl tm _ _ =>
        let (drs1, e1) := stepWouldClause fuel drs ev
        let (drs2, e2) := stepInstruction fuel .postEvent .obj (nextClause drs1) repl
        let (_, e3) := match tm with | some t => stepTiming fuel drs2 t | none => (drs2, [])
        (drs, e1 ++ e2 ++ e3)
      | .damageResult r =>
        match r with
        | ⟨_, source, recipient, _, does⟩ =>
          let (drs1, e1) := stepPredicate fuel drs recipient
          let (drs2, e2) := stepSubjectBefore fuel (nextClause drs1) source
          let (drs3, e3) := stepInstruction fuel .postEvent source.patientRole drs2 does
          let (_, e4) := stepSubjectAfter fuel drs3 source
          (drs, e1 ++ e2 ++ e3 ++ e4)
      | .damageRule _ da ds dop _ =>
        let (drs1, e1) := stepDamageAgent fuel drs da
        let (drs2, e2) := stepDamageScope fuel drs1 ds
        let (drs3, e3) := stepDamageOp fuel drs2 dop
        (drs3, e1 ++ e2 ++ e3)
      | .preventionBan _ (.described da ds) _ =>
        let (drs1, e1) := stepDamageAgent fuel drs da
        let (drs2, e2) := stepDamageScope fuel drs1 ds
        (drs2, e1 ++ e2)
      | .preventionBan _ .thatDamage _ => (drs, [])
      | .additionalTriggers ev q =>
        let (drs1, e1) := stepGameEvent fuel drs ev
        let (drs2, e2) := stepQuantity fuel drs1 q
        (drs2, e1 ++ e2)
      | .conditional spec cond _ =>
        let (drs1, e1) := stepCondition fuel drs cond
        let (drs2, e2) := stepStaticSpec fuel (nextClause drs1) spec
        (drs2, e1 ++ e2)
      | .visibility _ player what =>
        let (drs1, _, e1) := stepNounPhrase fuel drs .subj true player
        let (drs2, e2) := stepVisibleThing fuel drs1 what
        (drs2, e1 ++ e2)
      | .conjunction optSubj parts =>
        let (drs1, e1) := match optSubj with
          | some sub => let (d, _, e) := stepNounPhrase fuel drs .subj true sub; (d, e)
          | none => (drs, [])
        let (drs2, e2) := stepStaticSpecList fuel drs1 parts
        (drs2, e1 ++ e2)
      | .specialAction tm cost does =>
        let (drs1, e1) := match tm with | some t => stepTiming fuel drs t | none => (drs, [])
        let (drs2, e2) := stepCost fuel drs1 cost
        let (drs3, e3) := stepInstruction fuel .postEvent .obj (nextClause (publicOnly drs2)) does
        (drs3, e1 ++ e2 ++ e3)
      | .asEvent ev does =>
        let (drs1, e1) := stepGameEvent fuel drs ev
        let (drs2, e2) := stepInstruction fuel .postEvent .obj (nextClause drs1) does
        (drs2, e1 ++ e2)
      | .whileHolds cond does =>
        let (drs1, e1) := stepCondition fuel drs cond
        let (drs2, e2) := stepInstruction fuel .postEvent .obj (nextClause drs1) does
        (drs2, e1 ++ e2)
  termination_by structural fuel

  def stepStaticSpecList (fuel : Nat) (drs : DRS) (ss : List StaticSpec) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match ss with
      | [] => (drs, [])
      | s :: rest =>
        let (drs1, e1) := stepStaticSpec fuel drs s
        let (drs2, e2) := stepStaticSpecList fuel drs1 rest
        (drs2, e1 ++ e2)
  termination_by structural fuel

  def checkAbility (fuel : Nat) (drs : DRS) (ab : Ability) : List Refusal :=
    match fuel with
    | 0 => []
    | fuel + 1 =>
      match ab with
      | .keyword _ _ body => (stepAbilityList fuel drs body).2
      | .activated cost inst tm _ guard activator =>
        let (drs0, e0) := match activator with
          | some a => let (d, _, e) := stepNounPhrase fuel drs .subj true a; (d, e)
          | none => (drs, [])
        let (drs1, e1) := stepCost fuel drs0 cost
        let colonDrs := nextClause (publicOnly drs1)
        let (drs2, e2) := stepInstruction fuel .postEvent .obj colonDrs inst
        let (_, e3) := match tm with | some t => stepTiming fuel drs2 t | none => (drs2, [])
        let (_, e4) := match guard with | some g => stepCondition fuel drs2 g | none => (drs2, [])
        e0 ++ e1 ++ e2 ++ e3 ++ e4
      | .triggered ev alts _ _ tm _ interv inst =>
        let (drs1, e1) := stepGameEvent fuel drs ev
        let (_, eAlts) := stepGameEventList fuel drs alts
        let (drs2, e2) := match interv with
          | some c => stepCondition fuel (nextClause drs1) c
          | none => (drs1, [])
        let (drs3, e3) := stepInstruction fuel .postEvent .obj (nextClause drs2) inst
        let (_, e4) := match tm with | some t => stepTiming fuel drs3 t | none => (drs3, [])
        e1 ++ eAlts ++ e2 ++ e3 ++ e4
      | .static spec => (stepStaticSpec fuel drs spec).2
      | .spell tm inst =>
        let (drs1, e1) := match tm with | some t => stepTiming fuel drs t | none => (drs, [])
        let (_, e2) := stepInstruction fuel .postEvent .obj drs1 inst
        e1 ++ e2
      | .mayBeginOnBattlefield | .thatAbility _ => []
      | .alsoForKeywords inner _ | .italicHead _ inner => checkAbility fuel drs inner
      | .qualified inner tm _ guard _ =>
        let e1 := checkAbility fuel drs inner
        let (_, e2) := match tm with | some t => stepTiming fuel drs t | none => (drs, [])
        let (_, e3) := match guard with | some g => stepCondition fuel drs g | none => (drs, [])
        e1 ++ e2 ++ e3
  termination_by structural fuel

  def stepAbilityList (fuel : Nat) (drs : DRS) (abs : List Ability) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match abs with
      | [] => (drs, [])
      | a :: rest =>
        let e1 := checkAbility fuel drs a
        let (drs2, e2) := stepAbilityList fuel drs rest
        (drs2, e1 ++ e2)
  termination_by structural fuel

  def stepGameEventList (fuel : Nat) (drs : DRS) (evs : List GameEvent) : DRS × List Refusal :=
    match fuel with
    | 0 => (drs, [])
    | fuel + 1 =>
      match evs with
      | [] => (drs, [])
      | e :: rest =>
        let (drs1, e1) := stepGameEvent fuel drs e
        let (drs2, e2) := stepGameEventList fuel drs1 rest
        (drs2, e1 ++ e2)
  termination_by structural fuel
end

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

end Semantics
