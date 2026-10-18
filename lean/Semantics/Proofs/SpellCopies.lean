import Semantics.Definitions
import Semantics.Check.Card
import Semantics.Check.Rules

/-!
# Semantics.Proofs.SpellCopies

A spell copy's target guard is not written: copy keywords offer new targets unconditionally
(`docs/decisions/semantics-v2.md` §7, ruling 2026-10-09), because writing "if the spell has any
targets" would need a new constructor. The guard is inert: a player who may choose new targets may
leave every target unchanged [CR#707.10c].
Casualty, replicate, epic and conspire are pinned here with gravestorm, whose narrow guard is
re-spelled away, and the extent `TargetExtent.only` ("targets only …" [CR#115.9c]).
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.SpellCopies

/-- Required pin: "a spell that targets only this creature" [CR#115.9c]. -/
theorem okSpellTargetsOnly :
    Predicate.checkAt .object (DRS.ofList []) (targetsOnly thisCreature) = [] := by
  decide

/-- Required pin: a player never targets, so "a player that targets only this creature" is
refused where an object is required. -/
theorem badPlayerTargetsOnly :
    Predicate.checkAt .player (DRS.ofList []) (targetsOnly thisCreature) = [.targeter .player] := by
  decide

/-- "Targets only" is the targeting predicate at the `only` extent, beside `someTarget` and
`soleTarget`. -/
theorem targetsOnlyIsTargetsAtOnly :
    targetsOnly thisCreature = .targets thisCreature .only := rfl

/-! ## Bodies

Each keyword's declared definition, as `cargo xtask lean macros` derives it from its
declaration, checks clean as an ability. -/

namespace Bodies

def cost : Cost := .mana [generic 1, pip .blue]

/-- Required pin: casualty N [CR#702.153a]. -/
theorem okCasualty : Ability.check (DRS.ofList []) (casualty (.lit 2)) = [] := by decide
/-- Required pin: replicate [cost] [CR#702.56a]. -/
theorem okReplicate : Ability.check (DRS.ofList []) (replicate cost) = [] := by decide
/-- Required pin: epic [CR#702.50a]. -/
theorem okEpic : Ability.check (DRS.ofList []) epic = [] := by decide
/-- Required pin: conspire, its cost and its copy [CR#702.78a]. -/
theorem okConspire : Ability.check (DRS.ofList []) conspire = [] := by decide
/-- Required pin: gravestorm, re-spelled without its target guard [CR#702.69a]. -/
theorem okGravestorm : Ability.check (DRS.ofList []) gravestorm = [] := by decide

end Bodies

end Semantics.Proofs.SpellCopies
