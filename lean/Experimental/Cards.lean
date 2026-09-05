import Experimental
import Experimental.Macros
import Experimental.Check.Card

/-!
# Experimental.Cards

The printed-card bench: cards written in the grammar. A handful of the Idris bench
(`idris/src/Experimental/Cards/*.idr`), spelled with the same macros, to show the shape. A
leading dot marks a constructor; a bare name is a macro.
Each card is a `Spelled`: its definition runs the checker by `decide`, so a refused spelling
does not define, as it did not elaborate in Idris.
-/

open Mtg Mtg.Macros

namespace Mtg.Cards

/-- Aerial Volley {G} — Instant. "Aerial Volley deals 3 damage divided as you choose among
one, two, or three target creatures with flying." -/
def aerialVolley : Spelled := spelled <|
  card "Aerial Volley" (some [pip .green]) ⟨[], [.instant], []⟩
    [ .spell none (dealsDivided .this (.lit 3)
        (.described (.target (oneThrough 3))
          (.and [creature, .hasKeyword (.the "Flying")]))) ]

/-- Topple {2}{W} — Sorcery. "Exile target creature with the greatest power among creatures
on the battlefield." -/
def topple : Spelled := spelled <|
  card "Topple" (some [generic 2, pip .white]) ⟨[], [.sorcery], []⟩
    [ .spell none (exile (target
        (.and [creature,
               .superlative .max (.stat .power) (.and [creature, .inZone battlefieldZ])]))) ]

/-- Disarm {U} — Instant. "Unattach all Equipment from target creature." -/
def disarm : Spelled := spelled <|
  card "Disarm" (some [pip .blue]) ⟨[], [.instant], []⟩
    [ .spell none (.unattach (allOf
        (.and [.hasSubtype (artifactType "Equipment"),
               .attachedTo (target creature)]))) ]

/-- Mana Leak {1}{U} — Instant. "Counter target spell unless its controller pays {3}." -/
def manaLeak : Spelled := spelled <|
  card "Mana Leak" (some [generic 1, pip .blue]) ⟨[], [.instant], []⟩
    [ .spell none (unlessPays (controllerOf (target spell))
        (.counterSpell it)
        (.mana [generic 3])) ]

/-- Oust {W} — Sorcery. "Put target creature into its owner's library second from the top.
Its controller gains 3 life." -/
def oust : Spelled := spelled <|
  card "Oust" (some [pip .white]) ⟨[], [.sorcery], []⟩
    [ .spell none (.sequentially
        [ move (target creature) (nthFromTop (.nth 2)),
          gainsLife (controllerOf it) (.lit 3) ]) ]

/-- Blinding Angel {3}{W}{W} — Creature — Angel 2/4. "Flying. Whenever Blinding Angel deals
combat damage to a player, that player skips their next combat phase." -/
def blindingAngel : Spelled := spelled <|
  card "Blinding Angel" (some [generic 3, pip .white, pip .white])
    ⟨[], [.creature], [creatureType "Angel"]⟩
    [ keyword "Flying",
      triggered .whenever
        (dealsCombatDamage thisCreature (a .anyPlayer))
        (.skipsNext (that .player) .combat (.lit 1)) ]
    (some (2, 4))

/-- Wormfang Manta {5}{U}{U} — Creature — Nightmare Fish Beast 6/1. "Flying. When Wormfang
Manta enters, you skip your next turn. When Wormfang Manta leaves the battlefield, you take an
extra turn after this one." -/
def wormfangManta : Spelled := spelled <|
  card "Wormfang Manta" (some [generic 5, pip .blue, pip .blue])
    ⟨[], [.creature], [creatureType "Nightmare", creatureType "Fish", creatureType "Beast"]⟩
    [ keyword "Flying",
      triggered .when (.enters thisCreature none) (.skipsNext .you .turn (.lit 1)),
      triggered .when (leavesBattlefield thisCreature) (.extraTurn .you (.lit 1)) ]
    (some (6, 1))

end Mtg.Cards
