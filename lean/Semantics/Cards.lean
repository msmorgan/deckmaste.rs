import Semantics
import Semantics.Macros
import Semantics.Check.Card
import Semantics.Cards.Anaphora
import Semantics.Cards.Copy
import Semantics.Cards.Description
import Semantics.Cards.Faces
import Semantics.Cards.Piles
import Semantics.Cards.Turn

/-!
# Semantics.Cards

The printed-card bench: the Idris bench (`idris/src/Experimental/Cards/*.idr`) spelled with
the same macros, one `Semantics.Cards.<Family>` module per Idris family, imported here. A
leading dot marks a constructor; a bare name is a macro; a card is a `CardFace` around a
`Characteristics` literal naming the fields it prints.
Each card is a `Spelled`: its definition runs the checker by `decide`, so a refused spelling
does not define, as it did not elaborate in Idris.
-/

open Semantics Semantics.Macros

namespace Semantics.Cards

/-- Blinding Angel {3}{W}{W} — Creature — Angel 2/4. "Flying. Whenever Blinding Angel deals
combat damage to a player, that player skips their next combat phase." -/
def blindingAngel : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Blinding Angel", cost := some [generic 3, pip .white, pip .white],
      types := [.creature], subtypes := [creatureType "Angel"],
      text :=
        [ keyword "Flying",
          whenever
            (dealsCombatDamage thisCreature (a .anyPlayer))
            (.skipsNext (that .player) .combat (.lit 1)) ],
      power := stat 2, toughness := stat 4 } }

/-- Wormfang Manta {5}{U}{U} — Creature — Nightmare Fish Beast 6/1. "Flying. When Wormfang
Manta enters, you skip your next turn. When Wormfang Manta leaves the battlefield, you take an
extra turn after this one." -/
def wormfangManta : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Wormfang Manta", cost := some [generic 5, pip .blue, pip .blue],
      types := [.creature],
      subtypes := [creatureType "Nightmare", creatureType "Fish", creatureType "Beast"],
      text :=
        [ keyword "Flying",
          when (.enters thisCreature none) (.skipsNext .you .turn (.lit 1)),
          when (leavesBattlefield thisCreature) (.extraTurn .you (.lit 1)) ],
      power := stat 6, toughness := stat 1 } }

end Semantics.Cards
