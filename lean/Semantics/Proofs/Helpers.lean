import Semantics.Check.Card

/-!
# Semantics.Proofs.Helpers

Scaffolding the pin suites under `Semantics.Proofs` share: projections of a checker step's
context (at the fuel the `.check` entry points use, 64), the clause-ranking lemmas behind the
universally quantified resolution pins, and small card and context fixtures.
-/

namespace Semantics.Proofs

/-- The context after checking `action` as a resolving instruction, the step `Action.check`
takes. -/
def actionResult (drs : DRS) (action : Action) : DRS :=
  (stepAction 64 .postEvent .obj drs action).1

/-- The context after checking `noun`, by default as an `.other` mention outside an argument slot,
the step `NounPhrase.check` takes; pass `.obj true` for a direct object in an argument slot. -/
def nounResult (drs : DRS) (noun : NounPhrase) (role : GramRole := .other)
    (inArgSlot : Bool := false) : DRS :=
  (stepNounPhrase 64 drs role inArgSlot noun).1

/-- The zone of the first referent `noun` selects, if it has one. -/
def nounZone (drs : DRS) (noun : NounPhrase) : Option Zone :=
  let result := stepNounPhrase 64 drs .other false noun
  result.2.1.head? >>= fun id => (result.1.findById? id).bind (·.zone)

/-- Folding `Nat.max` over referents all from clauses older than `clause` leaves `clause`. -/
theorem olderClauseMaximum (rs : List Referent) (clause : Nat)
    (older : ∀ r ∈ rs, r.clause < clause) :
    rs.foldl (fun acc r => Nat.max acc r.clause) clause = clause := by
  induction rs with
  | nil => rfl
  | cons r rs ih =>
    have hr := older r (by simp)
    have ht : ∀ r ∈ rs, r.clause < clause := by
      intro r h
      exact older r (by simp [h])
    simp only [List.foldl_cons, Nat.max_eq_left (Nat.le_of_lt hr)]
    exact ih ht

/-- No referent from a clause older than `clause` is in `clause`. -/
theorem olderClauseFilter (rs : List Referent) (clause : Nat)
    (older : ∀ r ∈ rs, r.clause < clause) :
    rs.filter (fun r => r.clause == clause) = [] := by
  induction rs with
  | nil => rfl
  | cons r rs ih =>
    have hr := older r (by simp)
    have ht : ∀ r ∈ rs, r.clause < clause := by
      intro r h
      exact older r (by simp [h])
    simp [Nat.ne_of_lt hr, ih ht]

/-- With no direct-argument mention buffered in the current clause, `D′` ranks every referent by
the clause and role it was introduced or last foregrounded in. -/
theorem rankingWithoutPending (drs : DRS) (h : drs.pendingRoles = []) :
    drs.rankingClause = (·.clause) ∧ drs.rankingRole = (·.role.rank) := by
  constructor <;> funext ref <;> simp [DRS.rankingClause, DRS.rankingRole, DRS.isClauseMate, h]

/-- The context no text has been checked in. -/
def emptyContext : DRS := DRS.ofList []

/-- Kind, number and determiner of each binding, newest first. -/
def shape (bs : DRS) : List (Kind × Plurality × Determiner) :=
  bs.referents.map fun b => (b.kind, b.plur, b.det)

/-- A card's printed abilities (a single-faced card). -/
def printedText (card : Spelled) : List Ability :=
  match card.card with
  | .singleFaced face => face.characteristics.text
  | _ => []

/-- An enchantment with the given text. -/
def enchantmentWith (text : List Ability) : Card :=
  .singleFaced { characteristics := { name := "", types := [.enchantment], text } }

/-- The halves of a keyword declaration's body, a `byCardClass` pair as both of its halves. -/
def halves : Ability → List Ability
  | .keyword _ _ body => body.flatMap fun
    | .byCardClass onPermanent onInstantOrSorcery => [onPermanent, onInstantOrSorcery]
    | ab => [ab]
  | _ => []

end Semantics.Proofs
