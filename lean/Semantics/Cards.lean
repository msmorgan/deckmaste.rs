import Semantics
import Semantics.Macros
import Semantics.Check.Card
import Semantics.Cards.Copy
import Semantics.Cards.Piles

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

/-- Aerial Volley {G} — Instant. "Aerial Volley deals 3 damage divided as you choose among
one, two, or three target creatures with flying." -/
def aerialVolley : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aerial Volley", cost := some [pip .green], types := [.instant],
      text :=
        [ .spell none (dealsDivided .this (.lit 3)
            (.described (.target (oneThrough 3))
              (.and [creature, .hasKeyword (.the "Flying")]))) ] } }

/-- Topple {2}{W} — Sorcery. "Exile target creature with the greatest power among creatures
on the battlefield." -/
def topple : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Topple", cost := some [generic 2, pip .white], types := [.sorcery],
      text :=
        [ .spell none (exile (target
            (.and [creature,
                   .superlative .max (.stat .power) (.and [creature, permanent])]))) ] } }

/-- Disarm {U} — Instant. "Unattach all Equipment from target creature." -/
def disarm : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Disarm", cost := some [pip .blue], types := [.instant],
      text :=
        [ .spell none (.unattach (allOf
            (.and [.hasSubtype (artifactType "Equipment"),
                   .attachedTo (target creature)]))) ] } }

/-- Mana Leak {1}{U} — Instant. "Counter target spell unless its controller pays {3}." -/
def manaLeak : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mana Leak", cost := some [generic 1, pip .blue], types := [.instant],
      text :=
        [ .spell none (unless_ (controllerOf (target spell))
            (.counterSpell it)
            (.mana [generic 3])) ] } }

/-- Oust {W} — Sorcery. "Put target creature into its owner's library second from the top.
Its controller gains 3 life." -/
def oust : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Oust", cost := some [pip .white], types := [.sorcery],
      text :=
        [ .spell none (.sequentially
            [ move (target creature) (nthFromTop (.nth 2)),
              gainsLife (controllerOf it) (.lit 3) ]) ] } }

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
