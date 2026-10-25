import Semantics.Definitions.Actions

/-!
# Semantics.Definitions.DesignationRules

The inherent triggered abilities of a designation: one `Definition.designation` row per
designation that has them, the monarch [CR#725.2] and the initiative [CR#726.2]. The label is a
leaf over primitives (`Semantics.Definitions.Designations`); the rows live here, above
`Semantics.Templates`, as the card types' and subtypes' rows do, because a row holds abilities.

The abilities have no source, and each is controlled by the player who held the designation when
it triggered, an exception to [CR#113.8]; that is the row's reading
(`Definition.controllingDesignation?`). Each is written as the CR words it, so it names the
designation's holder ("the monarch") and never "this".
-/

namespace Semantics.Definitions

open Semantics.Templates

/-! ## The monarch -/

/-- [CR#725.2] "At the beginning of the monarch's end step, that player draws a card." -/
def monarchDraw : Conferral :=
  .ability (at_ (beginningOfPossessed .endStep theMonarch) (instruct (that player) (draw 1)))

/-- [CR#725.2] "Whenever a creature deals combat damage to the monarch, its controller becomes
the monarch." -/
def monarchSteal : Conferral :=
  .ability
    (whenever (does (a isCreature) (dealDamage (to := theMonarch) (kind := some .combat)))
      (gainDesignation (controllerOf it) monarch))

/-- [CR#725.1,725.2] The monarch: an effect makes a player the monarch, and the monarch draws a
card at the beginning of their end step and loses the designation to the controller of a
creature that deals them combat damage. -/
def monarchDefinition : Definition :=
  .designation monarch [monarchDraw, monarchSteal]

/-! ## The initiative

Two of the initiative's three abilities venture into Undercity [CR#726.2,701.49d], and the
grammar has no venture action: venture into the dungeon [CR#701.49] needs a dungeon card frame
with its rooms and the venture marker's position [CR#309.4,309.5] (`lean-v3-venture-dungeon`).
The row carries the third until they can be written. -/

/-- [CR#726.2] "Whenever one or more creatures a player controls deal combat damage to the player
who has the initiative, the controller of those creatures takes the initiative." -/
def initiativeSteal : Conferral :=
  .ability
    (whenever
      (does (counted (orMore 1) (and [isCreature, controlledBy (a isPlayer)]))
        (dealDamage (to := theInitiativeHolder) (kind := some .combat)))
      (gainDesignation (controllerOf (those creature)) initiative))

/-- [CR#726.1,726.2] The initiative: an effect makes a player take it, and the controller of
creatures that deal combat damage to the player who has it takes it. -/
def initiativeDefinition : Definition :=
  .designation initiative [initiativeSteal]

/-- Every designation row with inherent abilities, in declaration order. -/
def designationDefinitions : List Definition := [initiativeDefinition, monarchDefinition]

end Semantics.Definitions
