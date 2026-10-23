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


/-! ## State-based actions -/

/-- [CR#704.5f] "If a creature has toughness 0 or less, it's put into its owner's graveyard." -/
def toughnessZero : SbaRule :=
  { scope := isCreature,
    when := .compareAmt (toughnessOf .this) .atMost 0,
    then_ := putInto .this graveyard }
theorem okToughnessZero : SbaRule.check toughnessZero = [] := by decide

/-- [CR#704.5i] "If a planeswalker has loyalty 0, it's put into its owner's graveyard."
A planeswalker's loyalty on the battlefield is its loyalty-counter count [CR#306.5c,122.1e],
so the condition reads the counters and not the printed characteristic. -/
def loyaltyZero : SbaRule :=
  { scope := .hasType .planeswalker,
    when := .compareAmt (countersOn (.named "loyaltyCounter") .this) .eq 0,
    then_ := putInto .this graveyard }
theorem okLoyaltyZero : SbaRule.check loyaltyZero = [] := by decide

/-- [CR#704.5w,310.8] "If a non-Siege battle has defense 0, it's put into its owner's
graveyard." A battle's defense on the battlefield is its defense-counter count [CR#310.4c], and
the Siege exclusion is a subtype read, not a seam. -/
def battleDefenseZero : SbaRule :=
  { scope := .and [.hasType .battle, .not (.hasSubtype siege)],
    when := .compareAmt (countersOn (.named "defenseCounter") .this) .eq 0,
    then_ := putInto .this graveyard }
theorem okBattleDefenseZero : SbaRule.check battleDefenseZero = [] := by decide

/-- A scope binds objects or players; a color is neither. -/
theorem badSbaScopeKind :
    SbaRule.check
      { scope := .qualityNoun .color none,
        when := .compareAmt (toughnessOf .this) .atMost 0,
        then_ := putInto .this graveyard }
      = [.kindLte (.quality .color) (.join .object .player)] := by
  decide

/-! ## Conferrals -/

/-- [CR#306.5b] "A planeswalker has the intrinsic ability 'This permanent enters with a number
of loyalty counters on it equal to its printed loyalty number.'" A replacement effect
[CR#614.1c] whose count is the planeswalker's loyalty, `statOf (.stat .loyalty) .this`: off the
battlefield a planeswalker card's loyalty is its printed number [CR#306.5a], so the printed
number needs no projection of its own. When the entering object is read is the engine's
concern, not the term's (ADR §0.1 items 4 and 5; owner ruling 2026-10-22). -/
def planeswalkerLoyalty : ConferralRule :=
  { scope := .hasType .planeswalker,
    confer :=
      .ability (.static
        (entersWithCounters .this (.statOf (.stat .loyalty) .this) (.named "loyaltyCounter"))) }
theorem okPlaneswalkerLoyalty : ConferralRule.check planeswalkerLoyalty = [] := by decide

/-- [CR#310.4b] "A battle has the intrinsic ability 'This permanent enters with a number of
defense counters on it equal to its printed defense number.'" The battle row of the same shape
[CR#614.1c], counting the battle's defense, which off the battlefield is its printed number
[CR#310.4a]. -/
def battleDefense : ConferralRule :=
  { scope := .hasType .battle,
    confer :=
      .ability (.static
        (entersWithCounters .this (.statOf (.stat .defense) .this) (.named "defenseCounter"))) }
theorem okBattleDefense : ConferralRule.check battleDefense = [] := by decide

/-- A conferral scopes what can have an ability — an object, or a player granted one
[CR#113.1a,113.1b] — and a color is neither. -/
theorem badConferralScopeKind :
    ConferralRule.check
      { scope := .qualityNoun .color none, confer := .ability (keyword (.named "flying")) }
      = [.kindLte (.quality .color) (.join .object .player)] := by
  decide

/-- [CR#702.170d] "A plotted card's owner may cast it from exile without paying its mana cost
during their main phase while the stack is empty during any turn after the turn in which it became
plotted. … A plotted card may be cast this way even if it doesn't have the plot ability while in
exile." A rule for every plotted card, not part of plot's text [CR#702.170a]; "you" on a card in
exile is its owner [CR#109.5]. -/
def plottedCardCast : ConferralRule :=
  { scope := .and [.isCard, hasDesignation plotted, in_ exileZone],
    confer :=
      .ability (static
        (onlyUnless
          (can you (cast this_ (from_ := exileZone) (payment := withoutPaying))
            (timing := asSorcery))
          (did any (gainDesignation this_ plotted) soFarThisTurn))) }
theorem okPlottedCardCast : ConferralRule.check plottedCardCast = [] := by decide

/-- [CR#702.143d] "That card's owner may look at that card as long as it remains in exile": the
look right of every foretold card, whatever made it foretold. Its cast right, "for any foretell
cost it has", needs a cost term naming a keyword's cost, which the grammar does not have. -/
def foretoldCardLook : ConferralRule :=
  { scope := .and [.isCard, hasDesignation foretold, in_ exileZone],
    confer := .ability (static (visibility .lookAt you (objects this_))) }
theorem okForetoldCardLook : ConferralRule.check foretoldCardLook = [] := by decide

/-! ## The results of damage

Each result is a rule, conferred as a property on what the rules give it to, so no effect that
removes abilities switches it off; an `asEvent` does it as the damage is dealt [CR#120.4c]. A
replacement of a result replaces this row's action, not the damage: wither and infect replace the
marking and the life loss [CR#120.3d,702.80a,702.90b,702.90c]. Lifelink, an ability, is the
keyword's own body [CR#702.15b]. -/

/-- The results of damage dealt to `this` the scoped object, `result` reading the damage dealt as
`thatMuch`. -/
def damageResultRow (scope : Predicate) (result : Action) : ConferralRule :=
  { scope, confer := .property (asEvent (does any (dealDamage anyAmount this_)) result) }

/-- [CR#120.3a] "Damage dealt to a player by a source without infect causes that player to lose
that much life." In a row scoped to players, "you" is the player. Infect replaces the loss
(`infect`) [CR#702.90b]. -/
def playerDamageLifeLoss : ConferralRule :=
  { scope := isPlayer,
    confer := .property (asEvent (does any (dealDamage anyAmount you)) (loseLife thatMuch)) }
theorem okPlayerDamageLifeLoss : ConferralRule.check playerDamageLifeLoss = [] := by decide

/-- [CR#120.3e] "Damage dealt to a creature by a source with neither wither nor infect causes
that much damage to be marked on that creature." Wither and infect replace the marking
[CR#120.3d]. -/
def creatureDamageMarking : ConferralRule :=
  damageResultRow isCreature (markDamage this_ thatMuch)
theorem okCreatureDamageMarking : ConferralRule.check creatureDamageMarking = [] := by decide

/-- [CR#120.3c] "Damage dealt to a planeswalker causes that many loyalty counters to be removed
from that planeswalker." -/
def planeswalkerLoyaltyDamage : ConferralRule :=
  damageResultRow (.hasType .planeswalker)
    (loseCounters (some (.printed (.named "loyaltyCounter"))) (some .thatMuch) .this)
theorem okPlaneswalkerLoyaltyDamage : ConferralRule.check planeswalkerLoyaltyDamage = [] := by
  decide

/-- [CR#120.3h] "Damage dealt to a battle causes that many defense counters to be removed from
that battle." -/
def battleDefenseDamage : ConferralRule :=
  damageResultRow (.hasType .battle)
    (loseCounters (some (.printed (.named "defenseCounter"))) (some .thatMuch) .this)
theorem okBattleDefenseDamage : ConferralRule.check battleDefenseDamage = [] := by decide

/-- [CR#120.3g,702.164c] "Combat damage dealt to a player by a creature with toxic causes that
creature's controller to give the player a number of poison counters equal to that creature's
total toxic value, in addition to the damage's other results." One row for the creature,
reading the sum of its toxic abilities' N values [CR#702.164b], not one per toxic ability: the
keyword is bodiless, so combat damage from a creature with toxic 2 and toxic 1 gives three poison
counters once. -/
def toxicPoison : ConferralRule :=
  { scope := .and [isCreature, .hasKeyword (.the (.named "toxic"))],
    confer := .property
      (asEvent (does this_ (dealDamage anyAmount (a isPlayer) (some .combat)))
        (putCounters (totalToxicValue this_) poison (that player))) }
theorem okToxicPoison : ConferralRule.check toxicPoison = [] := by decide

/-- A creature with two toxic abilities has one result: each ability is the bare keyword, with
no body of its own, and the one row reads the total toxic value [CR#702.164b,702.164c]. -/
theorem toxicIsOneResultReadingTheTotal :
    toxic 2 = .keyword (.named "toxic") [.number (.inCaller 2)] [] ∧
      toxic 1 = .keyword (.named "toxic") [.number (.inCaller 1)] [] ∧
      toxicPoison.confer = .property
        (asEvent (does this_ (dealDamage anyAmount (a isPlayer) (some .combat)))
          (putCounters (.aggregate .sum (.keywordNumber (.named "toxic")) this_) poison
            (that player))) :=
  ⟨rfl, rfl, rfl⟩

/-- The results of damage to a planeswalker are no ability of it: they are a property the rules
confer, so an effect that makes a permanent lose all abilities leaves them [CR#120.3c,113.12]. -/
theorem planeswalkerDamageResultIsNotAnAbility :
    (match planeswalkerLoyaltyDamage.confer with
      | .property _ => true
      | _ => false) = true := rfl

/-! ## Registry definitions -/

/-- [CR#122.1] "A counter is a marker placed on an object or player." A charge counter is a
marker and nothing else: the CR gives it no behavior of its own, so it confers nothing. -/
def chargeCounter : Definition := .counter (.named "chargeCounter") []
theorem okChargeCounter : Definition.check chargeCounter = [] := by decide

/-- [CR#122.1b] "A keyword counter on a permanent … causes that object to gain that keyword."
The counter is not an ability, so its conferral is the ability-free `property` flavor; but its
bearer gains the keyword in layer 6 [CR#613.1f], and that gained ability can be removed by
another effect [CR#113.12], applied in timestamp order [CR#613.7,613.7c]. -/
def flyingCounter : Definition :=
  .counter (.named "flyingCounter") [.property (Templates.abilityGrant .this (keyword (.named "flying")))]
theorem okFlyingCounter : Definition.check flyingCounter = [] := by decide

/-- The conferral helpers are the raw conferrals: `grants` is `flyingCounter`'s, and `boost` is
the one stat change carrying both deltas. -/
theorem grantsIsTheKeywordConferral :
    grants (.named "flying") =
      .property (.characteristicChange .this [.addedAbilities [keyword (.named "flying")]]) := rfl
theorem boostIsOneStatChange :
    boost (.up 1) (.up 1) =
      .property (.characteristicChange .this
        [.stat .adds .power 1, .stat .adds .toughness 1]) := rfl

/-- [CR#122.1a] "A +X/+Y counter on a creature … adds X to that object's power and Y to that
object's toughness." The definition states what ONE counter confers, "+1/+1" in layer 7c
[CR#613.4c], and the model applies it once per counter held, so the conferral reads no count.

The conferral is about its bearer wherever the bearer is: a +1/+1 counter also counts "on a
creature card in a zone other than the battlefield" [CR#122.1a], and the checker asserts no zone
for the bearer of a stat change, a definition's or a card's (`lean-checker-sweep-zone`). -/
def p1p1CounterDefinition : Definition :=
  .counter (.named "p1p1Counter") [boost (.up 1) (.up 1)]
theorem p1p1CounterDefinitionZone :
    Definition.check p1p1CounterDefinition = [] := by decide

/-- [CR#122.1a]: the minus counter's twin of `p1p1CounterDefinition`, subtracting 1 from each
stat. -/
def m1m1CounterDefinition : Definition :=
  .counter (.named "m1m1Counter") [boost (.down 1) (.down 1)]
theorem m1m1CounterDefinitionZone :
    Definition.check m1m1CounterDefinition = [] := by decide

/-- The retired spelling of `p1p1Counter`'s conferral, which multiplied each stat by the count of
its own kind on the bearer, in two single-stat changes. Nothing is refused for reading its own
count or for not reading it, because multiplicity is the model's reading of a counter definition
and the checker computes none. -/
def ownCount : Amount := .statOf (.counter (.named "p1p1Counter")) .this
def p1p1CounterCountMultiplied : Definition :=
  .counter (.named "p1p1Counter")
    [ .property (Templates.modification .this .power (.up ownCount)),
      .property (Templates.modification .this .toughness (.up ownCount)) ]
theorem p1p1CounterCountMultipliedTwin :
    Definition.check p1p1CounterCountMultiplied = [] := by decide

/-- [CR#122.1f] "If a player has ten or more poison counters, that player loses the game." A
poison counter is a player's. -/
theorem okPoisonCounter : Definition.check (.counter (.named "poison" .player) []) = [] := by
  decide

/-- A counter is placed on an object or a player [CR#122.1] and on nothing else; a color is
neither. -/
theorem badCounterHolder :
    Definition.check (.counter (.named "chargeCounter" (.quality .color)) [])
      = [.definitionHolder (.quality .color)] := by
  decide

/-- Counter names distinguish kinds [CR#122.1]. This registry uses nonempty names as lookup
keys, so it refuses an unnamed row. -/
theorem badNamelessCounter :
    Definition.check (.counter (.named "") []) = [.definitionNamed] := by decide

/-- Subtypes are type-line vocabulary [CR#205.3a,205.3b]. This catalog row supplies the word
without an intrinsic conferral. -/
def gargoyle : Definition := .subtype Semantics.Definitions.gargoyle []
theorem okGargoyle : Definition.check gargoyle = [] := by decide

/-- Subtypes are written as words [CR#205.3b]. The registry requires a nonempty word as its
lookup key. -/
theorem badNamelessSubtype :
    Definition.check (.subtype (.of .creature "") []) = [.definitionNamed] := by decide

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
  .subtype Semantics.Definitions.equipment
    [ .property (mayBeAttachedTo thisEquipment (allOf isCreature)),
      .stateBased (.not (isAttached thisEquipment)) (.attachment .unattached thisEquipment none) ]
theorem okEquipmentDefinition : Definition.check equipmentDefinition = [] := by decide

/-- [CR#301.6,704.5n] The Land twin of Equipment. -/
def fortificationDefinition : Definition :=
  .subtype Semantics.Definitions.fortification
    [ .property (mayBeAttachedTo thisFortification (allOf isLand)),
      .stateBased (.not (isAttached thisFortification))
        (.attachment .unattached thisFortification none) ]
theorem okFortificationDefinition : Definition.check fortificationDefinition = [] := by decide

/-- [CR#704.5m] An Aura that is not attached is put into its owner's graveyard. -/
def auraDefinition : Definition :=
  .subtype Semantics.Definitions.aura
    [ .stateBased (.not (isAttached thisAura))
        (.move thisAura (.zone .battlefield none) (some graveyard) .arrival []) ]
theorem okAuraDefinition : Definition.check auraDefinition = [] := by decide

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
  .subtype Semantics.Definitions.saga
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
theorem okSagaDefinition : Definition.check sagaDefinition = [] := by decide

/-- The lore increment is a turn-based action [CR#714.3c], a game action and no effect
[CR#703.1,609.1]: the `turnBased` conferral that encloses it says so, and the put itself carries
no cause. Doubling Season's "if an effect would put" is `causedBy .anEffect` [CR#614.16], so it
doubles a Saga's entry counter, put by a replacement effect [CR#714.3a], and not the increment.
The checker has no matcher, so this pins the row's spelling; the matching is the engine's. -/
theorem sagaLoreIncrementIsTurnBased :
    (∃ parts, sagaDefinition = .subtype Semantics.Definitions.saga parts ∧
      parts[1]? = some (.turnBased .firstMain (doIf sagaHasChapterAbilities sagaLoreIncrement))) ∧
    sagaLoreIncrement = .putCounters 1 (.printed (.named "loreCounter")) thisSaga :=
  ⟨⟨_, rfl, rfl⟩, rfl⟩

/-- The triggered-chapter guard reads the Saga's chapter abilities that have triggered and not yet
left the stack, waiting ones included [CR#714.4,704.5s,113.7]. -/
theorem okSagaHasTriggeredChapter :
    Condition.check (DRS.ofList []) sagaHasTriggeredChapter = [] := by
  decide

/-- "An ability that has triggered but not yet left the stack" is an ability, so it binds an
ability referent alone, as the Siege's guard writes it [CR#704.5v,310.7]. -/
theorem triggeredNotLeftStackSeedsAnAbility : triggeredNotLeftStack.seedsAbility = true := rfl

/-- The Siege's guard, "it isn't the source of an ability that has triggered but not yet left the
stack", over the row's own subject [CR#704.5v,310.7]. -/
theorem okSiegeGuard :
    Condition.check (DRS.ofList [])
      (.not (exists_ (.and [.abilityOf .this, triggeredNotLeftStack]))) = [] := by
  decide

/-- The type conferral helpers are the raw conferrals. -/
theorem combatantIsTheRole : combatant = .property (.combatant .this) := rfl
theorem playableFromIsThePlayPermission :
    playableFrom yourHand asSorcery =
      .property (.can (.actor .you) (.procedure .play .this .itsOwnCost none (some yourHand) none) { timing := (some asSorcery) }) := rfl

/-- [CR#725.1] "The monarch is a designation a player can have. There is no monarch in a game
until an effect instructs a player to become the monarch" — so the designation is effectful. -/
def monarch : Definition := .designation (.named "monarch" (.heldBy .player)) true
theorem okMonarch : Definition.check monarch = [] := by decide

/-- [CR#709.5c] "'Left half unlocked' and 'right half unlocked' are designations that a
permanent on the battlefield can have." -/
def leftHalfUnlocked : Definition := .designation (.named "leftHalfUnlocked" (.heldBy .object)) true
theorem okLeftHalfUnlocked : Definition.check leftHalfUnlocked = [] := by decide

/-- Goaded is one named designation [CR#701.15b]. This registry uses nonempty designation
names as lookup keys. -/
theorem badNamelessDesignation :
    Definition.check (.designation (.named "" (.heldBy .player)) true)
      = [.definitionNamed] := by
  decide

/-! ## The predefined-token catalog -/

/-- [CR#111.10a] "A Treasure token is a colorless Treasure artifact token with '{T}, Sacrifice
this token: Add one mana of any color.'" -/
def treasure : PredefinedToken :=
  { name := "Treasure",
    token :=
      { characteristics :=
          { types := [.artifact], subtypes := [Semantics.Definitions.treasure],
            text :=
              [ activated (.compound [.tapSymbol, .perform (sacrifice .this)])
                  (.addMana 1 (.anyColor .sameColor) []) ] } } }
theorem okTreasure : PredefinedToken.check treasure = [] := by decide

/-- [CR#111.10b] "A Food token is a colorless Food artifact token with '{2}, {T}, Sacrifice
this token: You gain 3 life.'" -/
def food : PredefinedToken :=
  { name := "Food",
    token :=
      { characteristics :=
          { types := [.artifact], subtypes := [Semantics.Definitions.food],
            text :=
              [ activated (.compound [.mana [generic 2], .tapSymbol, .perform (sacrifice .this)])
                  (gainLife 3) ] } } }
theorem okFood : PredefinedToken.check food = [] := by decide

/-- [CR#111.10c] "A Gold token is a colorless Gold artifact token with 'Sacrifice this token:
Add one mana of any color.'" -/
def gold : PredefinedToken :=
  { name := "Gold",
    token :=
      { characteristics :=
          { types := [.artifact], subtypes := [Semantics.Definitions.gold],
            text :=
              [ activated (.perform (sacrifice .this))
                  (.addMana 1 (.anyColor .sameColor) []) ] } } }
theorem okGold : PredefinedToken.check gold = [] := by decide

/-- [CR#111.10f] "A Clue token is a colorless Clue artifact token with '{2}, Sacrifice this
token: Draw a card.'" -/
def clue : PredefinedToken :=
  { name := "Clue",
    token :=
      { characteristics :=
          { types := [.artifact], subtypes := [Semantics.Definitions.clue],
            text :=
              [ activated (.compound [.mana [generic 2], .perform (sacrifice .this)])
                  (draw 1) ] } } }
theorem okClue : PredefinedToken.check clue = [] := by decide

/-- [CR#111.10g] "A Blood token is a colorless Blood artifact token with '{1}, {T}, Discard a
card, Sacrifice this token: Draw a card.'" -/
def blood : PredefinedToken :=
  { name := "Blood",
    token :=
      { characteristics :=
          { types := [.artifact], subtypes := [Semantics.Definitions.blood],
            text :=
              [ activated
                  (.compound
                    [ .mana [generic 1], .tapSymbol, .perform (discard (a (.and [.isCard, .in_ yourHand]))),
                      .perform (sacrifice .this) ])
                  (draw 1) ] } } }
theorem okBlood : PredefinedToken.check blood = [] := by decide

/-- [CR#111.10h] "A Powerstone token is a colorless Powerstone artifact token with '{T}: Add
{C}. This mana can't be spent to cast a nonartifact spell.'" [CR#106.6] -/
def powerstone : PredefinedToken :=
  { name := "Powerstone",
    token :=
      { characteristics :=
          { types := [.artifact], subtypes := [Semantics.Definitions.powerstone],
            text :=
              [ activated .tapSymbol
                  (.addMana 1 (.runs [[.colorless]])
                    [.spendNotOn [.toCast (.and [.not isArtifact, isSpell])]]) ] } } }
theorem okPowerstone : PredefinedToken.check powerstone = [] := by decide

/-- [CR#111.10w] "A Vibranium token is a colorless Vibranium artifact token with indestructible
and '{T}: Add {C}. This mana can't be spent to cast a nonartifact spell.'" -/
def vibranium : PredefinedToken :=
  { name := "Vibranium",
    token :=
      { characteristics :=
          { types := [.artifact], subtypes := [Semantics.Definitions.vibranium],
            text :=
              [ keyword (.named "indestructible"),
                activated .tapSymbol
                  (.addMana 1 (.runs [[.colorless]])
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
    conferral :=
      [ planeswalkerLoyalty, battleDefense, plottedCardCast, foretoldCardLook,
        playerDamageLifeLoss, creatureDamageMarking, planeswalkerLoyaltyDamage,
        battleDefenseDamage, toxicPoison ] }

theorem okBuiltinTables : RulesTables.check builtinTables = [] := by decide

end Semantics.Proofs.Rules
