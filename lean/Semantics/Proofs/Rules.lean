import Semantics.Macros
import Semantics.Check.Rules

/-!
# Semantics.Proofs.Rules

The rules-table bench and its pins. Not a port of an Idris family — Idris never modelled the
tables — so the sentences here are the Comprehensive Rules' own, and the bench items are the
rows `plugins/builtin/rules` and `plugins/builtin/tokens` actually write.

Each `ok…` fixes the empty refusal list for a real row; each `bad…` fixes the complete list a
malformed one earns. `distinctPredefinedTokens` is asserted over the catalog the same way
`Proofs/Tables.lean` asserts the checker's own label tables.
-/

open Semantics Semantics.Macros

namespace Semantics.Proofs.Rules

/-! ## State-based actions -/

/-- [CR#704.5f] "If a creature has toughness 0 or less, it's put into its owner's graveyard." -/
def toughnessZero : SbaRule :=
  { scope := creature,
    when := .compareAmt (toughnessOf .this) .atMost (.lit 0),
    then_ := move .this graveyard }
theorem okToughnessZero : SbaRule.check toughnessZero = [] := by decide

/-- [CR#704.5i] "If a planeswalker has loyalty 0, it's put into its owner's graveyard."
A planeswalker's loyalty on the battlefield is its loyalty-counter count [CR#306.5c,122.1e],
so the condition reads the counters and not the printed characteristic. -/
def loyaltyZero : SbaRule :=
  { scope := .hasType .planeswalker,
    when := .compareAmt (countersOn (.named "Loyalty") .this) .eq (.lit 0),
    then_ := move .this graveyard }
theorem okLoyaltyZero : SbaRule.check loyaltyZero = [] := by decide

/-- [CR#704.5w,310.8] "If a non-Siege battle has defense 0, it's put into its owner's
graveyard." A battle's defense on the battlefield is its defense-counter count [CR#310.4c], and
the Siege exclusion is a subtype read, not a seam. -/
def battleDefenseZero : SbaRule :=
  { scope := .and [.hasType .battle, .not (.hasSubtype (.of .battle "Siege"))],
    when := .compareAmt (countersOn (.named "Defense") .this) .eq (.lit 0),
    then_ := move .this graveyard }
theorem okBattleDefenseZero : SbaRule.check battleDefenseZero = [] := by decide

/-- A state-based action doesn't use the stack [CR#704.1], and a target is chosen as a spell or
ability is put on the stack [CR#601.2c], so nothing it does is targeted. -/
theorem badSbaThatTargets :
    SbaRule.check
      { scope := creature,
        when := .compareAmt (toughnessOf .this) .atMost (.lit 0),
        then_ := move (target creature) graveyard }
      = [.nontarget] := by
  decide

/-- A scope binds objects or players; a color is neither. -/
theorem badSbaScopeKind :
    SbaRule.check
      { scope := .qualityNoun .color none,
        when := .compareAmt (toughnessOf .this) .atMost (.lit 0),
        then_ := move .this graveyard }
      = [.kindLte (.quality .color) ruleScopeBound] := by
  decide

/-! ## Conferrals -/

/-- [CR#306.5b] "A planeswalker has the intrinsic ability 'This permanent enters with a number
of loyalty counters on it equal to its printed loyalty number.'" A replacement effect
[CR#614.1c] whose count is the printed loyalty [CR#209.1,306.5a]. -/
def planeswalkerLoyalty : ConferralRule :=
  { scope := .hasType .planeswalker,
    confer :=
      .static
        (.replacement (Primitives.GameEvent.enters .this none) [] none
          (.putCounters (.statOf (.stat .loyalty) .this) (.printed (.named "Loyalty")) .this)
          .repeatedly none) }
theorem okPlaneswalkerLoyalty : ConferralRule.check planeswalkerLoyalty = [] := by decide

/-- A conferral scopes what can have an ability — an object, or a player granted one
[CR#113.1a,113.1b] — and a color is neither. -/
theorem badConferralScopeKind :
    ConferralRule.check { scope := .qualityNoun .color none, confer := keyword "Flying" }
      = [.kindLte (.quality .color) ruleScopeBound] := by
  decide

/-- A spell ability is followed as instructions while an instant or sorcery resolves
[CR#113.3a]; there is nothing a rule could confer one onto. -/
theorem badConferralOfSpellAbility :
    ConferralRule.check
      { scope := .hasType .planeswalker, confer := .spell none (draw (.lit 1)) }
      = [.grantable] := by
  decide

/-! ## Damage results -/

/-- [CR#120.3c] "Damage dealt to a planeswalker causes that many loyalty counters to be removed
from that planeswalker." -/
def planeswalkerLoyaltyDamage : DamageResultRule :=
  { recipient := .hasType .planeswalker, remove := .named "Loyalty" }
theorem okPlaneswalkerLoyaltyDamage :
    DamageResultRule.check planeswalkerLoyaltyDamage = [] := by decide

/-- [CR#120.3h] "Damage dealt to a battle causes that many defense counters to be removed from
that battle." -/
def battleDefenseDamage : DamageResultRule :=
  { recipient := .hasType .battle, remove := .named "Defense" }
theorem okBattleDefenseDamage : DamageResultRule.check battleDefenseDamage = [] := by decide

/-- Damage can't be dealt to an object that isn't a battle, a creature, or a planeswalker
[CR#120.1a]. -/
theorem badDamageResultRecipient :
    DamageResultRule.check { recipient := enchantment, remove := .named "Loyalty" }
      = [.damageRecipient] := by
  decide

/-- The counter registry declares who holds each counter [CR#122.1]: an energy counter is a
player's, so no damage to a permanent removes one. -/
theorem badDamageResultCounterHolder :
    DamageResultRule.check { recipient := .hasType .planeswalker, remove := .named "Energy" }
      = [.counterKindNamed .object] := by
  decide

/-- A counter kind is a registry key, and "loyalty" is not the declared label. -/
theorem badDamageResultUndeclaredCounter :
    DamageResultRule.check { recipient := .hasType .planeswalker, remove := .named "loyalty" }
      = [.knownCounter] := by
  decide

/-! ## Registry definitions -/

/-- [CR#122.1] "A counter is a marker placed on an object or player." A charge counter is a
marker and nothing else: the CR gives it no behavior of its own, so it confers nothing. -/
def chargeCounter : Definition := .counter (.named "Charge") .object []
theorem okChargeCounter : Definition.check chargeCounter = [] := by decide

/-- [CR#122.1b] "A keyword counter on a permanent … causes that object to gain that keyword."
The grant states a quality of the bearer rather than giving it an ability of its own
[CR#113.12], so it is the ability-free `property` flavor. -/
def flyingCounter : Definition :=
  .counter (.keyword "Flying") .object [.property (.abilityGrant .this (keyword "Flying"))]
theorem okFlyingCounter : Definition.check flyingCounter = [] := by decide

/-- [CR#122.1f] "If a player has ten or more poison counters, that player loses the game." A
poison counter is a player's. -/
theorem okPoisonCounter : Definition.check (.counter (.named "Poison") .player []) = [] := by
  decide

/-- A counter is placed on an object or a player [CR#122.1] and on nothing else; a color is
neither. -/
theorem badCounterHolder :
    Definition.check (.counter (.named "Charge") (.quality .color) [])
      = [.definitionHolder (.quality .color)] := by
  decide

/-- A counter looked up by label declares one [CR#122.1]; the registry has no way to reach a
nameless row. -/
theorem badNamelessCounter :
    Definition.check (.counter (.named "") .object []) = [.definitionNamed] := by decide

/-- Most subtypes are inert vocabulary [CR#205.3]: the catalog admits the word and the rules
define nothing for it. -/
def gargoyle : Definition := .subtype (creatureType "Gargoyle") []
theorem okGargoyle : Definition.check gargoyle = [] := by decide

/-- A subtype is looked up by the word it declares [CR#205.3], so a nameless one is
unreachable. -/
theorem badNamelessSubtype :
    Definition.check (.subtype (creatureType "") []) = [.definitionNamed] := by decide

/-- A conferral gives an ordinary ability [CR#305.6], and a spell ability is followed while a
spell resolves [CR#113.3a]: no type rule could put one on a permanent. -/
theorem badSubtypeConfersASpellAbility :
    Definition.check (.subtype (artifactType "Equipment") [.ability (.spell none (draw (.lit 1)))])
      = [.grantable] := by
  decide

/-- [CR#725.1] "The monarch is a designation a player can have. There is no monarch in a game
until an effect instructs a player to become the monarch" — so the designation is effectful. -/
def monarch : Definition := .designation "the monarch" (.heldBy .player) true none none none
theorem okMonarch : Definition.check monarch = [] := by decide

/-- [CR#709.5c] "'Left half unlocked' and 'right half unlocked' are designations that a
permanent on the battlefield can have." The room half narrows the object that holds it. -/
def leftHalfUnlocked : Definition :=
  .designation "left half unlocked" (.heldBy .object) true (some .battlefield) none (some .left)
theorem okLeftHalfUnlocked : Definition.check leftHalfUnlocked = [] := by decide

/-- The zone, card type and room half describe the object holding the designation, so a
designation the game holds [CR#731.1] narrows nothing. -/
theorem badNarrowedGameDesignation :
    Definition.check (.designation "day" .heldByGame true (some .battlefield) none none)
      = [.definitionScoped "day"] := by
  decide

/-- A designation is identified by name [CR#701.15b], so a nameless one is unreachable. -/
theorem badNamelessDesignation :
    Definition.check (.designation "" (.heldBy .player) true none none none)
      = [.definitionNamed] := by
  decide

/-! ## The predefined-token catalog -/

/-- [CR#111.10a] "A Treasure token is a colorless Treasure artifact token with '{T}, Sacrifice
this token: Add one mana of any color.'" -/
def treasure : PredefinedToken :=
  { name := "Treasure",
    token :=
      { characteristics :=
          { types := [.artifact], subtypes := [artifactType "Treasure"],
            text :=
              [ activated (.compound [.tapSymbol, .perform (sacrifice .this)])
                  (.addMana (.lit 1) (.anyColor .sameColor) []) ] } } }
theorem okTreasure : PredefinedToken.check treasure = [] := by decide

/-- [CR#111.10b] "A Food token is a colorless Food artifact token with '{2}, {T}, Sacrifice
this token: You gain 3 life.'" -/
def food : PredefinedToken :=
  { name := "Food",
    token :=
      { characteristics :=
          { types := [.artifact], subtypes := [artifactType "Food"],
            text :=
              [ activated (.compound [.mana [generic 2], .tapSymbol, .perform (sacrifice .this)])
                  (gainLife (.lit 3)) ] } } }
theorem okFood : PredefinedToken.check food = [] := by decide

/-- [CR#111.10c] "A Gold token is a colorless Gold artifact token with 'Sacrifice this token:
Add one mana of any color.'" -/
def gold : PredefinedToken :=
  { name := "Gold",
    token :=
      { characteristics :=
          { types := [.artifact], subtypes := [artifactType "Gold"],
            text :=
              [ activated (.perform (sacrifice .this))
                  (.addMana (.lit 1) (.anyColor .sameColor) []) ] } } }
theorem okGold : PredefinedToken.check gold = [] := by decide

/-- [CR#111.10f] "A Clue token is a colorless Clue artifact token with '{2}, Sacrifice this
token: Draw a card.'" -/
def clue : PredefinedToken :=
  { name := "Clue",
    token :=
      { characteristics :=
          { types := [.artifact], subtypes := [artifactType "Clue"],
            text :=
              [ activated (.compound [.mana [generic 2], .perform (sacrifice .this)])
                  (draw (.lit 1)) ] } } }
theorem okClue : PredefinedToken.check clue = [] := by decide

/-- [CR#111.10g] "A Blood token is a colorless Blood artifact token with '{1}, {T}, Discard a
card, Sacrifice this token: Draw a card.'" -/
def blood : PredefinedToken :=
  { name := "Blood",
    token :=
      { characteristics :=
          { types := [.artifact], subtypes := [artifactType "Blood"],
            text :=
              [ activated
                  (.compound
                    [ .mana [generic 1], .tapSymbol, .perform (discard (a (.inZone hand))),
                      .perform (sacrifice .this) ])
                  (draw (.lit 1)) ] } } }
theorem okBlood : PredefinedToken.check blood = [] := by decide

/-- [CR#111.10h] "A Powerstone token is a colorless Powerstone artifact token with '{T}: Add
{C}. This mana can't be spent to cast a nonartifact spell.'" [CR#106.6] -/
def powerstone : PredefinedToken :=
  { name := "Powerstone",
    token :=
      { characteristics :=
          { types := [.artifact], subtypes := [artifactType "Powerstone"],
            text :=
              [ activated .tapSymbol
                  (.addMana (.lit 1) (.runs [[.colorless]])
                    [.spendNotOn [.toCast (.and [.not artifact, spell])]]) ] } } }
theorem okPowerstone : PredefinedToken.check powerstone = [] := by decide

/-- [CR#111.10w] "A Vibranium token is a colorless Vibranium artifact token with indestructible
and '{T}: Add {C}. This mana can't be spent to cast a nonartifact spell.'" -/
def vibranium : PredefinedToken :=
  { name := "Vibranium",
    token :=
      { characteristics :=
          { types := [.artifact], subtypes := [artifactType "Vibranium"],
            text :=
              [ keyword "Indestructible",
                activated .tapSymbol
                  (.addMana (.lit 1) (.runs [[.colorless]])
                    [.spendNotOn [.toCast (.and [.not artifact, spell])]]) ] } } }
theorem okVibranium : PredefinedToken.check vibranium = [] := by decide

/-- The seven predefined tokens the builtin plugin carries. -/
def builtinPredefinedTokens : List PredefinedToken :=
  [blood, clue, food, gold, powerstone, treasure, vibranium]

theorem builtinPredefinedTokensCheck :
    builtinPredefinedTokens.flatMap PredefinedToken.check = [] := by decide

theorem builtinPredefinedTokensDistinct :
    distinctPredefinedTokens builtinPredefinedTokens = true := by decide

/-- An effect creates a predefined token by name [CR#111.10], so an unnamed entry is
unreachable. -/
theorem badUnnamedPredefinedToken :
    PredefinedToken.check { treasure with name := "" } = [.tokenNamed] := by decide

/-- A token represents a permanent [CR#111.1] and every permanent has one of the six permanent
types [CR#110.4]; a bundle that writes none is not a token. -/
theorem badUntypedPredefinedToken :
    PredefinedToken.check
      { name := "Treasure", token := { characteristics := { subtypes := [artifactType "Treasure"] } } }
      = [.tokenTyped, .subsFitLine] := by
  decide

/-! ## The builtin tables -/

/-- The rules tables the builtin plugin carries, less the two lethal-damage rows
[CR#704.5g,704.5h], whose marked damage and "since state-based actions were last checked"
lookback the grammar does not yet spell. -/
def builtinTables : RulesTables :=
  { sba := [toughnessZero, loyaltyZero, battleDefenseZero],
    conferral := [planeswalkerLoyalty],
    damageResult := [planeswalkerLoyaltyDamage, battleDefenseDamage] }

theorem okBuiltinTables : RulesTables.check builtinTables = [] := by decide

end Semantics.Proofs.Rules
