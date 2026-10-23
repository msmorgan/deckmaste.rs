import Semantics.Definitions
import Semantics.Check.Card
import Semantics.Cards.Keyword
import Semantics.Cards.Damage

/-!
# Semantics.Proofs.ReplacementArms

Which arm of a replacement that keeps its event each witness's action lands in (owner ruling
2026-10-23, `lean-v3-replacement-before-after`). `before` holds what is done before the replaced
event: the choices a replacement of an entering requires, made before the permanent enters
[CR#614.12a], and devour's sacrifice. `alongside` ("with") holds what is done as part of the event,
"enters with" and "as [it] enters" text, whose effect occurs as part of the event that puts the
permanent onto the battlefield [CR#603.6d], or as part of its turning face up [CR#614.1e]. Each pin
is closed by `rfl`.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.ReplacementArms

/-- The arm a replacement's action lands in. -/
inductive Arm where
  | before
  | alongside
  deriving DecidableEq, Repr

/-- The arms a replacement takes, in order, read through a condition or a caller's frame. -/
def arms (replacement : Replacement) (fuel : Nat := 8) : List Arm :=
  match fuel, replacement with
  | 0, _ => []
  | n + 1, .inCaller body | n + 1, .onlyIf _ body => arms body n
  | _ + 1, .before _ none => [.before]
  | n + 1, .before _ (some body) => .before :: arms body n
  | _ + 1, .alongside _ => [.alongside]
  | _ + 1, .instead _ | _ + 1, .withRiders _ => []

/-- What a replacement does before its event, read through a condition or a caller's frame. -/
def beforeAction? (replacement : Replacement) (fuel : Nat := 8) : Option Action :=
  match fuel, replacement with
  | 0, _ => none
  | n + 1, .inCaller body | n + 1, .onlyIf _ body => beforeAction? body n
  | _ + 1, .before action _ => some action
  | _ + 1, _ => none

/-- What each replacement a keyword's body makes does before its event. -/
def beforeActions : Ability → List (Option Action)
  | .keyword _ _ body => body.flatMap fun
    | .static (.replace _ replacement _ _ _) => [beforeAction? replacement]
    | _ => []
  | _ => []

/-- The arms of each replacement a static ability makes. -/
def specArms : StaticSpec → List (List Arm)
  | .replace _ replacement _ _ _ => [arms replacement]
  | _ => []

/-- The arms of each replacement among `abilities`, a keyword read through its body. -/
def abilityArms : Ability → List (List Arm)
  | .keyword _ _ body => body.flatMap fun
    | .static spec => specArms spec
    | _ => []
  | .static spec => specArms spec
  | _ => []

/-- The arms of each replacement a single-faced card's text makes. -/
def cardArms : Card → List (List Arm)
  | .singleFaced face => face.characteristics.text.flatMap abilityArms
  | _ => []

/-- "As this creature enters, choose a color": "If a replacement effect that modifies how a
permanent enters the battlefield requires a choice, that choice is made before the permanent
enters the battlefield" [CR#614.12a]. -/
theorem entryChoiceIsBefore : specArms (entersChoosing thisCreature .color) = [[.before]] := rfl

/-- Devour: "As this object enters, you may sacrifice any number of creatures. This permanent
enters with N +1/+1 counters on it for each creature sacrificed this way" [CR#702.82a]. The
sacrifice is a `before`, and the counters ride the entering, `alongside` it. -/
theorem devourSacrificeIsBefore :
    beforeActions (devour 2) =
        [some (may (Action.devour (counted anyNumber (and [isCreature, .in_ battlefield]))))] ∧
      abilityArms (devour 2) = [[.before, .alongside]] := ⟨rfl, rfl⟩

/-- Amplify: "As this object enters, reveal any number of cards from your hand that share a
creature type with it. This permanent enters with N +1/+1 counters on it for each card revealed
this way" [CR#702.38a]: the reveal, like devour's sacrifice, is a `before`, and the counters are
`alongside` the entering. -/
theorem amplifyRevealIsBefore : abilityArms (amplify 1) = [[.before, .alongside]] := rfl

/-- Tribute: "As this creature enters, choose an opponent. That player may put an additional N
+1/+1 counters on it as it enters" [CR#702.104a]: the choice is a `before` [CR#614.12a], and the
counters, put on "as it enters", are `alongside`. -/
theorem tributeChoiceIsBeforeItsCountersAlongside :
    abilityArms (tribute 2) = [[.before, .alongside]] := rfl

/-- Riot: "You may have this permanent enter with an additional +1/+1 counter on it. If you
don't, it gains haste" [CR#702.136a]: what it enters with is `alongside` the entering. -/
theorem riotIsAlongside : abilityArms riot = [[.alongside]] := rfl

/-- Unleash: "You may have this permanent enter with an additional +1/+1 counter on it"
[CR#702.98a]: `alongside` the entering. -/
theorem unleashIsAlongside : abilityArms unleash = [[.alongside]] := rfl

/-- Megamorph: "As this permanent is turned face up, put a +1/+1 counter on it if its megamorph
cost was paid to turn it face up" [CR#702.37b], a replacement effect [CR#614.1e] whose counter
is `alongside` the turning face up, its own event, not an entering. -/
theorem megamorphCounterIsAlongside :
    abilityArms (megamorph (.mana [generic 1])) = [[.alongside]] := rfl

/-- Enchant: an Aura entering other than by resolving, by an effect that doesn't specify what it
will enchant: "that player chooses what it will enchant as the Aura enters the battlefield"
[CR#303.4f]. The choice is a `before` [CR#614.12a], and the attachment `alongside` the entering.
Its other two replacements put something in the entering's place, an `instead` [CR#303.4g,303.4i]. -/
theorem enchantChoiceIsBeforeItsAttachmentAlongside :
    abilityArms (enchant isCreature) = [[], [.before, .alongside], []] := rfl

/-- *Deadpool, Trading Card*: "As Deadpool enters, you may exchange his text box and another
creature's", text read "As [this permanent] enters" [CR#614.1c], whose effect occurs as part of
the entering [CR#603.6d]: `alongside`. -/
theorem deadpoolExchangeIsAlongside :
    cardArms Cards.deadpoolTradingCard.card = [[.alongside]] := rfl

/-- *Wolverine, Fierce Fighter*: "If damage would be dealt to Wolverine, instead that damage is
dealt, but all other damage already dealt to him is healed." Healing removes all damage marked
on a permanent [CR#701.69a]; the damage the event deals is marked by it [CR#120.3e], so the damage
"already dealt" is what is marked before the event, and its healing is a `before`
(`Cards.wolverineFierceFighter`). -/
theorem wolverineHealingIsBefore :
    cardArms Cards.wolverineFierceFighter.card = [[.before]] := rfl

end Semantics.Proofs.ReplacementArms
