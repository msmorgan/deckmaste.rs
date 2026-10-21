import Semantics.Definitions
import Semantics.Check.Card

/-!
# Semantics.Proofs.KeywordAbilityCensus

The keyword abilities `semantics-v2-keyword-ability-census` bodied from vocabulary that already
existed: saddle and teamwork over a group bounded by its total power (`SliceCount.totalling`, as
crew's cost is written), awaken as an alternative cost and a spell ability, and indestructible as
a prohibition over the destroy deed. Each definition is written as its declaration builds it, checks.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.KeywordAbilityCensus

/-- "Tap any number of other untapped creatures you control with total power 2 or greater: This
permanent becomes saddled until end of turn. Activate only as a sorcery." [CR#702.171a,702.171b] -/
def saddleDefinition : List Ability :=
  [ .activated
      (.perform (tap (withTotal (.stat .power) .atLeast (.lit 2)
        (allOf (.and [isUntapped, and [isCreature, youControl, isOther]])))))
      (.gainDesignation thisPermanent (.named "saddled") (some untilEndOfTurn)) (some asSorcery) none none none ]

/-- "Saddle 2" with its Definition checks [CR#702.171a]. -/
theorem okSaddleDefinition :
    Ability.check (DRS.ofList []) (.keyword (.named "saddle") [.number (.lit 2)] saddleDefinition)
      = [] := by
  decide

/-- Saddling taps other creatures: a total over "creatures you control" with no "other" may
include the saddled permanent itself, and is a different cost. The bound is still an amount: an
unresolved "its power" is refused. -/
theorem badSaddleUnresolvedBound :
    Cost.check (DRS.ofList []) (.perform (tap (withTotal (.stat .power) .atLeast
      (powerOf (that (.type .creature)))
      (allOf (.and [isUntapped, and [isCreature, youControl, isOther]]))))) =
      [.anaphor (.word (.type .creature)) .one 0] := by
  decide

/-- "As an additional cost to cast this spell, you may tap any number of creatures you control
with total power 2 or more." [CR#702.194a] -/
def teamworkDefinition : List Ability :=
  [ .static (.addedCost
      (.perform (tap (withTotal (.stat .power) .atLeast (.lit 2) (allOf (and [isCreature, youControl])))))
      true .once) ]

/-- "Teamwork 2" with its Definition checks
[CR#702.194a]. -/
theorem okTeamworkDefinition :
    Ability.check (DRS.ofList []) (.keyword (.named "teamwork") [.number (.lit 2)] teamworkDefinition)
      = [] := by
  decide

/-- "{5}{U}" -/
def awakenCost : Cost := .mana [generic 5, pip .blue]

/-- "You may pay {5}{U} rather than pay this spell's mana cost as you cast this spell" and "If
this spell's awaken cost was paid, put four +1/+1 counters on target land you control. That land
becomes a 0/0 Elemental creature with haste. It's still a land." [CR#702.113a] -/
def awakenDefinition : List Ability :=
  [ .static (.can (.actor .you) (.procedure .cast .this (.payingInstead awakenCost) none none none)
      {}),
    .spell none
      (doIf (costWasPaid (.byKeyword (.named "awaken")) none .this)
        (.sequentially
          [ .putCounters (.lit 4) (.printed p1p1Counter)
              (target (.and [isLand, .hasPossessor .controller .you])) .one true,
            .establish
              (.characteristicChange (that (.type .land))
                [ .typeLine .adds { types := some [.creature], subtypes := some [elemental] },
                  .stat .sets .power (.lit 0), .stat .sets .toughness (.lit 0),
                  .addedAbilities [keyword (.named "haste")] ])
              none ])) ]

/-- "Awaken 4—{5}{U}" with its Definition checks
[CR#702.113a]. -/
theorem okAwakenDefinition :
    Ability.check (DRS.ofList []) (.keyword (.named "awaken") [.number (.lit 4), .cost awakenCost]
        awakenDefinition)
      = [] := by
  decide

/-- "A permanent with indestructible can't be destroyed." [CR#702.12b] -/
def indestructibleDefinition : List Ability :=
  [ .static (cant any (destroy thisPermanent)) ]

/-- Indestructible with its Definition checks
[CR#702.12b]. -/
theorem okIndestructibleDefinition :
    Ability.check (DRS.ofList []) (.keyword (.named "indestructible") [] indestructibleDefinition)
      = [] := by
  decide

/-- What is destroyed is a permanent [CR#701.8a]: a player can't be named in the destroy deed's
patient role. -/
theorem badDestroyedPlayer :
    (StaticSpec.check (DRS.ofList []) (cant any (destroy .you))).isEmpty = false := by
  decide

end Semantics.Proofs.KeywordAbilityCensus
