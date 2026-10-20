import Semantics.Definitions
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

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.Rules

private def damageResultCheck (rule : DamageResult) : List Refusal :=
  StaticSpec.check (DRS.ofList []) (.damageResult rule)

/-! ## State-based actions -/

/-- [CR#704.5f] "If a creature has toughness 0 or less, it's put into its owner's graveyard." -/
def toughnessZero : SbaRule :=
  { scope := isCreature,
    when := .compareAmt (toughnessOf .this) .atMost (.lit 0),
    then_ := putInto .this graveyard }
theorem okToughnessZero : SbaRule.check toughnessZero = [] := by decide

/-- [CR#704.5i] "If a planeswalker has loyalty 0, it's put into its owner's graveyard."
A planeswalker's loyalty on the battlefield is its loyalty-counter count [CR#306.5c,122.1e],
so the condition reads the counters and not the printed characteristic. -/
def loyaltyZero : SbaRule :=
  { scope := .hasType .planeswalker,
    when := .compareAmt (countersOn (.named "loyaltyCounter") .this) .eq (.lit 0),
    then_ := putInto .this graveyard }
theorem okLoyaltyZero : SbaRule.check loyaltyZero = [] := by decide

/-- [CR#704.5w,310.8] "If a non-Siege battle has defense 0, it's put into its owner's
graveyard." A battle's defense on the battlefield is its defense-counter count [CR#310.4c], and
the Siege exclusion is a subtype read, not a seam. -/
def battleDefenseZero : SbaRule :=
  { scope := .and [.hasType .battle, .not (.hasSubtype (.of .battle "Siege"))],
    when := .compareAmt (countersOn (.named "defenseCounter") .this) .eq (.lit 0),
    then_ := putInto .this graveyard }
theorem okBattleDefenseZero : SbaRule.check battleDefenseZero = [] := by decide

/-- A scope binds objects or players; a color is neither. -/
theorem badSbaScopeKind :
    SbaRule.check
      { scope := .qualityNoun .color none,
        when := .compareAmt (toughnessOf .this) .atMost (.lit 0),
        then_ := putInto .this graveyard }
      = [.kindLte (.quality .color) (.join .object .player)] := by
  decide

/-! ## Conferrals -/

/-- [CR#306.5b] "A planeswalker has the intrinsic ability 'This permanent enters with a number
of loyalty counters on it equal to its printed loyalty number.'" A replacement effect
[CR#614.1c] whose count is the printed loyalty [CR#209.1,306.5a]. -/
def planeswalkerLoyalty : ConferralRule :=
  { scope := .hasType .planeswalker,
    confer :=
      .static
        (entersWithCounters .this (.statOf (.stat .loyalty) .this) (.named "loyaltyCounter")) }
theorem okPlaneswalkerLoyalty : ConferralRule.check planeswalkerLoyalty = [] := by decide

/-- A conferral scopes what can have an ability — an object, or a player granted one
[CR#113.1a,113.1b] — and a color is neither. -/
theorem badConferralScopeKind :
    ConferralRule.check { scope := .qualityNoun .color none, confer := keyword (.named "flying") }
      = [.kindLte (.quality .color) (.join .object .player)] := by
  decide

/-! ## Damage results -/

/-- [CR#120.3c] "Damage dealt to a planeswalker causes that many loyalty counters to be removed
from that planeswalker." -/
def planeswalkerLoyaltyDamage : DamageResult :=
  { kind := .any, source := .any, recipient := .hasType .planeswalker, mode := .intrinsic,
    does := loseCounters (some (.printed (.named "loyaltyCounter"))) (some .thatMuch) .this }
theorem okPlaneswalkerLoyaltyDamage :
    damageResultCheck planeswalkerLoyaltyDamage = [] := by decide

/-- [CR#120.3h] "Damage dealt to a battle causes that many defense counters to be removed from
that battle." -/
def battleDefenseDamage : DamageResult :=
  { kind := .any, source := .any, recipient := .hasType .battle, mode := .intrinsic,
    does := loseCounters (some (.printed (.named "defenseCounter"))) (some .thatMuch) .this }
theorem okBattleDefenseDamage : damageResultCheck battleDefenseDamage = [] := by decide

/-- A counter kind is a registry key, and "loyalty" is not the declared name (`loyaltyCounter`). -/
theorem badDamageResultUndeclaredCounter :
    damageResultCheck { planeswalkerLoyaltyDamage with
      does := loseCounters (some (.printed (.named "loyalty"))) (some .thatMuch) .this }
      = [.knownCounter] := by
  decide

/-! ## Registry definitions -/

/-- [CR#122.1] "A counter is a marker placed on an object or player." A charge counter is a
marker and nothing else: the CR gives it no behavior of its own, so it confers nothing. -/
def chargeCounter : Definition := .counter (.named "chargeCounter") .object []
theorem okChargeCounter : Definition.check chargeCounter = [] := by decide

/-- [CR#122.1b] "A keyword counter on a permanent … causes that object to gain that keyword."
The grant states a quality of the bearer rather than giving it an ability of its own
[CR#113.12], so it is the ability-free `property` flavor. -/
def flyingCounter : Definition :=
  .counter (.named "flyingCounter") .object [.property (Templates.abilityGrant .this (keyword (.named "flying")))]
theorem okFlyingCounter : Definition.check flyingCounter = [] := by decide

/-- The conferral helpers are the raw conferrals: `grants` is `flyingCounter`'s, and `boost` is
the one stat change carrying both deltas. -/
theorem grantsIsTheKeywordConferral :
    grants (.named "flying") =
      .property (.characteristicChange .this [.addedAbilities [keyword (.named "flying")]]) := rfl
theorem boostIsOneStatChange :
    boost (.up (.lit 1)) (.up (.lit 1)) =
      .property (.characteristicChange .this
        [.stat .adds .power (.lit 1), .stat .adds .toughness (.lit 1)]) := rfl

/-- [CR#122.1a] "A +X/+Y counter on a creature … adds X to that object's power and Y to that
object's toughness." The definition states what ONE counter confers, "+1/+1" in layer 7c
[CR#613.4c], and the model applies it once per counter held, so the conferral reads no count.

The conferral is about its bearer wherever the bearer is: a +1/+1 counter also counts "on a
creature card in a zone other than the battlefield" [CR#122.1a], and the checker asserts no zone
for the bearer of a stat change, a definition's or a card's (`lean-checker-sweep-zone`). -/
def p1p1CounterDefinition : Definition :=
  .counter (.named "p1p1Counter") .object [boost (.up (.lit 1)) (.up (.lit 1))]
theorem p1p1CounterDefinitionZone :
    Definition.check p1p1CounterDefinition = [] := by decide

/-- [CR#122.1a]: the minus counter's twin of `p1p1CounterDefinition`, subtracting 1 from each
stat. -/
def m1m1CounterDefinition : Definition :=
  .counter (.named "m1m1Counter") .object [boost (.down (.lit 1)) (.down (.lit 1))]
theorem m1m1CounterDefinitionZone :
    Definition.check m1m1CounterDefinition = [] := by decide

/-- The retired spelling of `p1p1Counter`'s conferral, which multiplied each stat by the count of
its own kind on the bearer, in two single-stat changes. Nothing is refused for reading its own
count or for not reading it, because multiplicity is the model's reading of a counter definition
and the checker computes none. -/
def ownCount : Amount := .statOf (.counter (.named "p1p1Counter")) .this
def p1p1CounterCountMultiplied : Definition :=
  .counter (.named "p1p1Counter") .object
    [ .property (Templates.modification .this .power (.up ownCount)),
      .property (Templates.modification .this .toughness (.up ownCount)) ]
theorem p1p1CounterCountMultipliedTwin :
    Definition.check p1p1CounterCountMultiplied = [] := by decide

/-- [CR#122.1f] "If a player has ten or more poison counters, that player loses the game." A
poison counter is a player's. -/
theorem okPoisonCounter : Definition.check (.counter (.named "poison") .player []) = [] := by
  decide

/-- A counter is placed on an object or a player [CR#122.1] and on nothing else; a color is
neither. -/
theorem badCounterHolder :
    Definition.check (.counter (.named "chargeCounter") (.quality .color) [])
      = [.definitionHolder (.quality .color)] := by
  decide

/-- Counter names distinguish kinds [CR#122.1]. This registry uses nonempty names as lookup
keys, so it refuses an unnamed row. -/
theorem badNamelessCounter :
    Definition.check (.counter (.named "") .object []) = [.definitionNamed] := by decide

/-- Subtypes are type-line vocabulary [CR#205.3a,205.3b]. This catalog row supplies the word
without an intrinsic conferral. -/
def gargoyle : Definition := .subtype (creatureType "Gargoyle") []
theorem okGargoyle : Definition.check gargoyle = [] := by decide

/-- Subtypes are written as words [CR#205.3b]. The registry requires a nonempty word as its
lookup key. -/
theorem badNamelessSubtype :
    Definition.check (.subtype (creatureType "") []) = [.definitionNamed] := by decide

/-- Artifact is a card type [CR#205.2a] and a permanent type [CR#110.4]. This registry row
needs no intrinsic conferral. -/
def artifactTypeDefinition : Definition := .cardType .artifact true []
theorem okArtifactType : Definition.check artifactTypeDefinition = [] := by decide

/-- [CR#508.1a] A creature is a Combatant: the creature-like combat rules apply to it. The role is
a quality, not an ability [CR#113.12]. -/
def creatureTypeDefinition : Definition := .cardType .creature true [combatant]
theorem okCreatureType : Definition.check creatureTypeDefinition = [] := by decide

/-- [CR#305.1] "A player who has priority may play a land card from their hand during a main
phase of their turn when the stack is empty." -/
def landTypeDefinition : Definition := .cardType .land true [playableFrom yourHand asSorcery]
theorem okLandType : Definition.check landTypeDefinition = [] := by decide

/-- [CR#304.1] "A player who has priority may cast an instant card from their hand." -/
def instantTypeDefinition : Definition :=
  .cardType .instant false [castableFrom yourHand asInstant]
theorem okInstantType : Definition.check instantTypeDefinition = [] := by decide

/-- [CR#309.2c] "Dungeon cards are not permanents. They can't be cast." -/
theorem okDungeonType : Definition.check (.cardType .dungeon false []) = [] := by decide

/-! ## The rules-defined subtypes (ruling 2026-10-10, `semantics-v2-subtype-rules`) -/

/-- "This Aura/Equipment/Fortification is attached": the attachment relation the engine's
attach-permission model keeps [CR#704.5m,704.5n]. -/
def isAttached (subject : NounPhrase) : Condition :=
  .matches subject (.attachment .attachment none none)

/-- [CR#301.5,704.5n] An Equipment may be attached to a creature, and one attached to no legal
host becomes unattached. -/
def equipmentDefinition : Definition :=
  .subtype (artifactType "Equipment")
    [ .property (mayBeAttachedTo thisEquipment (allOf isCreature)),
      .stateBased (.not (isAttached thisEquipment)) (.attachment .unattached thisEquipment none) ]
theorem okEquipmentDefinition : Definition.check equipmentDefinition = [] := by decide

/-- [CR#301.6,704.5n] The Land twin of Equipment. -/
def fortificationDefinition : Definition :=
  .subtype (artifactType "Fortification")
    [ .property (mayBeAttachedTo thisFortification (allOf isLand)),
      .stateBased (.not (isAttached thisFortification))
        (.attachment .unattached thisFortification none) ]
theorem okFortificationDefinition : Definition.check fortificationDefinition = [] := by decide

/-- [CR#704.5m] An Aura that is not attached is put into its owner's graveyard. -/
def auraDefinition : Definition :=
  .subtype (enchantmentType "Aura")
    [ .stateBased (.not (isAttached thisAura))
        (.move thisAura (.zone .battlefield none) (some graveyard) .arrival none []) ]
theorem okAuraDefinition : Definition.check auraDefinition = [] := by decide

/-- [CR#714.3a,714.3c,714.4] A Saga enters with a lore counter unless it has read ahead, gets one
as its controller's precombat main phase begins, and is sacrificed once its lore counters reach
its final chapter number. -/
def sagaDefinition : Definition :=
  .subtype (enchantmentType "Saga")
    [ .ability (.static (onlyUnless (entersWithCounters thisSaga (.lit 1) (.named "loreCounter"))
        (.matches thisSaga (.hasKeyword (.the (.named "readAhead")))))),
      .turnBased .firstMain
        (.putCounters (.lit 1) (.printed (.named "loreCounter")) thisSaga .one true),
      .stateBased
        (.and [ .compareAmt (finalChapterNumber thisSaga) .atLeast (.lit 1),
                .compareAmt (countersOn (.named "loreCounter") thisSaga) .atLeast
                  (finalChapterNumber thisSaga) ])
        (sacrifice thisSaga) ]
theorem okSagaDefinition : Definition.check sagaDefinition = [] := by decide

/-- The type conferral helpers are the raw conferrals. -/
theorem combatantIsTheRole : combatant = .property (.combatant .this) := rfl
theorem playableFromIsThePlayPermission :
    playableFrom yourHand asSorcery =
      .property (.can (.actor .you) (.procedure .play .this .itsOwnCost none (some yourHand) none) { timing := (some asSorcery) }) := rfl

/-- [CR#725.1] "The monarch is a designation a player can have. There is no monarch in a game
until an effect instructs a player to become the monarch" — so the designation is effectful. -/
def monarch : Definition := .designation (.named "monarch") (.heldBy .player) true none none none
theorem okMonarch : Definition.check monarch = [] := by decide

/-- [CR#709.5c] "'Left half unlocked' and 'right half unlocked' are designations that a
permanent on the battlefield can have." The room half narrows the object that holds it. -/
def leftHalfUnlocked : Definition :=
  .designation (.named "leftHalfUnlocked") (.heldBy .object) true (some .battlefield) none (some .left)
theorem okLeftHalfUnlocked : Definition.check leftHalfUnlocked = [] := by decide

/-- The zone, card type and room half describe the object holding the designation, so a
designation the game holds [CR#731.1] narrows nothing. -/
theorem badNarrowedGameDesignation :
    Definition.check (.designation (.named "day") .heldByGame true (some .battlefield) none none)
      = [.definitionScoped (.named "day")] := by
  decide

/-- Goaded is one named designation [CR#701.15b]. This registry uses nonempty designation
names as lookup keys. -/
theorem badNamelessDesignation :
    Definition.check (.designation (.named "") (.heldBy .player) true none none none)
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
                    [ .mana [generic 1], .tapSymbol, .perform (discard (a (.and [.isCard, .in_ yourHand]))),
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
                    [.spendNotOn [.toCast (.and [.not isArtifact, isSpell])]]) ] } } }
theorem okPowerstone : PredefinedToken.check powerstone = [] := by decide

/-- [CR#111.10w] "A Vibranium token is a colorless Vibranium artifact token with indestructible
and '{T}: Add {C}. This mana can't be spent to cast a nonartifact spell.'" -/
def vibranium : PredefinedToken :=
  { name := "Vibranium",
    token :=
      { characteristics :=
          { types := [.artifact], subtypes := [artifactType "Vibranium"],
            text :=
              [ keyword (.named "indestructible"),
                activated .tapSymbol
                  (.addMana (.lit 1) (.runs [[.colorless]])
                    [.spendNotOn [.toCast (.and [.not isArtifact, isSpell])]]) ] } } }
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
