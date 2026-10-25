import Semantics.Definitions.Actions

/-!
# Semantics.Definitions.SubtypeRules

What a subtype's type rule confers on the objects of that subtype: a `Definition.subtype` row per
subtype whose rules define behavior. The `Subtype` value (`Semantics.Definitions.Subtypes`) is a
leaf over primitives and carries no conferrals; the rows live here, above `Semantics.Templates`,
as the card types' rows do (`Semantics.Definitions.CardTypeRules`). A row is read with `this`
bound to the object of the subtype.
-/

namespace Semantics.Definitions

open Semantics.Templates

/-! ## Siege -/

/-- [CR#310.12a] "As a Siege enters the battlefield, its controller must choose its protector
from among their opponents. Only an opponent of a Siege's controller can be its protector." A
rule and no ability of the Siege [CR#113.12], made as part of its entering [CR#614.1c,614.12]. -/
def siegeProtector : Conferral := .property (entersChoosingPlayer thisSiege anOpponent)

/-- [CR#310.12b] "Sieges have the intrinsic ability 'When the last defense counter is removed
from this permanent, exile it, then you may cast it transformed without paying its mana cost.'"
Cast transformed, the resulting spell has its back face up [CR#712.8c]. -/
def siegeLastCounter : Conferral :=
  .ability
    (when (does any (removeCounters theLast defenseCounter thisPermanent))
      (sequentially
        [ exile it,
          may (cast it (payment := withoutPaying) (resulting := some isTransformed)) ]))

/-- "It isn't the source of an ability that has triggered but not yet left the stack", over the
row's own subject [CR#704.5v,310.7,113.7]. -/
def siegeHasNoWaitingTrigger : Condition :=
  .not (exists_ (.and [.abilityOf .this, triggeredNotLeftStack]))

/-- [CR#704.5v] "If a Siege battle has defense 0 and it isn't the source of an ability that has
triggered but not yet left the stack, it's put into its owner's graveyard." Its defense on the
battlefield is its defense-counter count [CR#310.4c]. -/
def siegeDefenseZero : Conferral :=
  .stateBased
    (.and [ .compareAmt (countersOn defenseCounter .this) .eq 0, siegeHasNoWaitingTrigger ])
    (putInto .this graveyard)

/-- [CR#310.12a,310.12b,704.5v] A Siege's protector is an opponent of its controller, it is
exiled and may be cast transformed when its last defense counter is removed, and it is put into
its owner's graveyard at defense 0 once no ability of it waits to leave the stack. -/
def siegeDefinition : Definition :=
  .subtype siege [siegeProtector, siegeLastCounter, siegeDefenseZero]

/-! ## Saga -/

/-- "This Saga has one or more chapter abilities": its final chapter number is at least I
[CR#714.2d]. -/
def sagaHasChapterAbilities : Condition :=
  .compareAmt (finalChapterNumber thisSaga) .atLeast 1

/-- "This Saga is the source of a chapter ability that has triggered but not yet left the stack"
[CR#714.4,113.7]. -/
def sagaHasTriggeredChapter : Condition :=
  exists_ (.and [chapterAbilityOf thisSaga, triggeredNotLeftStack])

/-- "Put a lore counter on this Saga", done by the game as a turn-based action [CR#714.3c]. -/
def sagaLoreIncrement : Action :=
  .putCounters 1 (.printed (.named "loreCounter")) thisSaga

/-- [CR#714.3a,714.3c,714.4] A Saga enters with a lore counter unless it has read ahead, gets one
as its controller's precombat main phase begins if it has one or more chapter abilities (a
turn-based action, no effect [CR#703.1,609.1]), and is sacrificed once its lore counters reach its
final chapter number, unless it is the source of a chapter ability that has triggered but not yet
left the stack. -/
def sagaDefinition : Definition :=
  .subtype saga
    [ .ability (.static (onlyUnless (entersWithCounters thisSaga 1 (.named "loreCounter"))
        (.matches thisSaga (.hasKeyword (.the (.named "readAhead")))))),
      .turnBased .firstMain
        (doIf sagaHasChapterAbilities sagaLoreIncrement),
      .stateBased
        (.and [ sagaHasChapterAbilities,
                .compareAmt (countersOn (.named "loreCounter") thisSaga) .atLeast
                  (finalChapterNumber thisSaga),
                .not sagaHasTriggeredChapter ])
        (sacrifice thisSaga) ]

/-- Every subtype row, in declaration order. -/
def subtypeDefinitions : List Definition := [siegeDefinition, sagaDefinition]

end Semantics.Definitions
