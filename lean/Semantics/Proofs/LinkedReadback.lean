import Semantics.Definitions
import Semantics.Check.Card
import Semantics.Check.Rules
import Semantics.Cards.Anaphora
import Semantics.Cards.Keyword
import Semantics.Cards.Turn

/-!
# Semantics.Proofs.LinkedReadback

A linked ability reads back what the ability it is linked to did [CR#607.1,607.2a]: a cost's
payment through a Linked Facet (`taken`, and the amount facets pinned in `Proofs/Description.lean`),
and the objects it acted on through a Participial Description, written as the same ability would
write it of its own deed ("the exiled card", "the sacrificed creature"). The objects a printed
triggered or activated ability, an additional or alternative cost, or an added target acted on are
published to the abilities after it (`Ability.textContext`); a description with no span written
reads them, and must find exactly one. A named link between two objects is a declared Relation
(`docs/decisions/semantics-v2.md` §7, ruling 2026-10-09). The three relation forms, `merge`,
`addsTarget`, and the mana-cost term of `CostShift.bySymbols` are pinned here too.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.LinkedReadback

/-- Sacrificing, as the refusals name the deed. -/
def sacrificing : Deed := .action "sacrifice"
/-- Exiling, as the refusals name the deed. -/
def exiling : Deed := .action "exile"
def haunting : RelationLabel := .named "haunting"
def paired : RelationLabel := .named "paired"

/-- "the sacrificed creature" [CR#702.119a]. -/
def theSacrificed : NounPhrase := the (and [was (sacrifice which), isCreature])

/-- "the exiled card" [CR#607.2a]. -/
def theExiledCard : NounPhrase := the (and [was (exile which), isCard])

/-- "As an additional cost to cast this spell, sacrifice a creature." -/
def sacrificeACreature : Ability :=
  static (.addedCost (.perform (sacrifice (a isCreature))) false .once)

/-- "When this enters, exile target card from a graveyard." -/
def exileGraveyardCard : Ability :=
  when (does thisPermanent enter) (exile (target (and [isCard, in_ graveyard])))

/-- "When this enters, exile up to two target cards from graveyards." -/
def exileGraveyardCards : Ability :=
  when (does thisPermanent enter) (exile (counted (upTo 2) (and [isCard, in_ graveyard])))

/-- "When this leaves the battlefield, return <subject> to the battlefield." -/
def returnToTheBattlefield (subject : NounPhrase) : Ability :=
  when (does thisPermanent leave) (returnToBattlefield subject [])

/-- Required pin: "the sacrificed creature's mana value" reads back the creature an additional
cost sacrificed [CR#702.119a,607.2a]. -/
theorem okSacrificedCreaturesManaValue :
    Ability.checkText (DRS.ofList [])
      [ sacrificeACreature,
        spell (draw (statOf (.stat .manaValue) theSacrificed)) ] = [] := by
  decide

/-- Charge of the Forever-Beast: "As an additional cost to cast this spell, reveal a creature card
from your hand. This spell deals damage to target creature or planeswalker equal to the revealed
card's power." Revealing a card leaves it in its zone [CR#701.20b] and it stays revealed while
needed [CR#701.20a]: the effect reads the card the cost revealed in its owner's hand (ruling
2020-04-17). -/
theorem okChargeOfTheForeverBeast :
    Ability.checkText (DRS.ofList [])
      [ static (.addedCost (.perform (reveal (a (and [isCreature, isCard, in_ yourHand])))) false
          .once),
        spell (instruct thisSpell
          (dealDamage (powerOf (the (and [was (reveal which), isCard])))
            (target (or [isCreature, isPlaneswalker])))) ] = [] := by
  decide

/-- Required pin: "the exiled card" reads back the card the linked ability exiled [CR#607.2a]. -/
theorem okExiledCardReadBack :
    Ability.checkText (DRS.ofList [])
      [exileGraveyardCard, returnToTheBattlefield theExiledCard] = [] := by
  decide

/-- Required pin: the description demands what the object was when the linked ability acted on it
[CR#607.2a,608.2h]: "the exiled creature" names a permanent, and the card exiled from a graveyard
never was one, so there is nothing to return. -/
theorem badZoneRestriction :
    Ability.checkText (DRS.ofList [])
      [ exileGraveyardCard,
        returnToTheBattlefield (the (and [was (exile which), isCreature])) ] =
      [.participialDescription (.deed exiling) 0] := by
  decide

/-- Required pin: a description demanding a zone or carrier other than the one the linked ability
put the object into is refused [CR#607.2a]: the card the linked ability exiled is in exile, not on
the battlefield nor in a graveyard. -/
theorem badLinkedObjectInAnotherZone :
    Ability.checkText (DRS.ofList [])
      [ exileGraveyardCard,
        returnToTheBattlefield (the (and [was (exile which), isCard, in_ battlefield])) ] =
      [.participialDescription (.deed exiling) 0] ∧
    Ability.checkText (DRS.ofList [])
      [ exileGraveyardCard,
        returnToTheBattlefield (the (and [was (exile which), isCard, in_ graveyard])) ] =
      [.participialDescription (.deed exiling) 0] ∧
    Ability.checkText (DRS.ofList [])
      [ exileGraveyardCard,
        returnToTheBattlefield (the (and [was (exile which), isCard, in_ exileZone])) ] = [] := by
  decide

/-- Required pin: two abilities of the object exile cards, so "the exiled card" could be either's
[CR#607.2a]. -/
theorem badAmbiguousExiledCard :
    Ability.checkText (DRS.ofList [])
      [exileGraveyardCard, exileGraveyardCard, returnToTheBattlefield theExiledCard] =
      [.participialDescription (.deed exiling) 2] := by
  decide

/-- Required pin: a singular "the exiled card" reads each card the linked ability exiled
[CR#607.3]. -/
theorem okSingularReadsLinkedGroup :
    Ability.checkText (DRS.ofList [])
      [exileGraveyardCards, returnToTheBattlefield theExiledCard] = [] := by
  decide

/-- With no linked ability that exiled anything, there is no exiled card to read. -/
theorem badNothingExiled :
    Ability.checkText (DRS.ofList []) [returnToTheBattlefield theExiledCard] =
      [.participialDescription (.deed exiling) 0] := by
  decide

/-- The linked ability exiled; it sacrificed nothing [CR#607.2a]. -/
theorem badDeedTheLinkedAbilityNeverPerformed :
    Ability.checkText (DRS.ofList [])
      [ exileGraveyardCard,
        returnToTheBattlefield (the (and [was (sacrifice which), isCard])) ] =
      [.participialDescription (.deed sacrificing) 0] := by
  decide

/-- A linked ability's objects are read back by a Participial Description, never by a
demonstrative or a pronoun: "that card" in a later line does not reach the exiled card. -/
theorem badDemonstrativeReachesNoLinkedObject :
    Ability.checkText (DRS.ofList [])
      [exileGraveyardCard, returnToTheBattlefield (that .card)] =
      [.anaphor (.word .card) .one 0] := by
  decide

/-- Read back, the linked object is mentioned: "return the exiled card to the battlefield under
its owner's control". -/
theorem okPronounAfterReadBack :
    Ability.checkText (DRS.ofList [])
      [ exileGraveyardCard,
        when (does thisPermanent leave)
          (returnToBattlefield theExiledCard [.under (ownerOf it)]) ] = [] := by
  decide

/-- "With 'this way' written, only the current ability's own deed": "the card exiled this way"
does not reach the linked ability's, because "exiled this way" is no linking phrase
[CR#607.1,607.2a]. -/
theorem badThisWayReadsNoLinkedObject :
    Ability.checkText (DRS.ofList [])
      [ exileGraveyardCard,
        returnToTheBattlefield (the (and [isCard, was (exile which) thisWay])) ] =
      [.participialDescription (.deed exiling) 0] := by
  decide

/-- "This way" reads no event a linked ability caused: "If a card was exiled this way, draw a
card" on a second ability reads nothing the first ability's exile did [CR#607.1,607.2a]. -/
theorem thisWayReadsNoLinkedEvent :
    Ability.checkText (DRS.ofList [])
      [ exileGraveyardCard,
        activated tapSymbol (doIf (was (exile (a isCard)) thisWay) (draw 1)) ] =
      [.thisWayUngrounded] := by
  decide

/-- Only what the linked ability did is linked [CR#607.1]: a creature its trigger event names
("whenever you sacrifice a creature") was sacrificed by something else. -/
theorem badTriggerEventObjectNotLinked :
    Ability.checkText (DRS.ofList [])
      [ when (does you (sacrifice (a isCreature))) (draw 1),
        spell (draw (statOf (.stat .manaValue) theSacrificed)) ] =
      [.participialDescription (.deed sacrificing) 0] := by
  decide

/-- The arms of a handoff are read too: "target opponent may exile a card from a graveyard".
-/
theorem okExiledInsideHandoff :
    Ability.checkText (DRS.ofList [])
      [ when (does thisPermanent enter)
          (instruct (target isOpponent) (may (exile (target (and [isCard, in_ graveyard]))))),
        returnToTheBattlefield theExiledCard ] = [] := by
  decide

/-- Arms the linked ability takes instead of each other are one place: "you may exile target
card from a graveyard; if you don't, exile target card from your hand" exiles one card. -/
theorem okExclusiveArmsAreOnePlace :
    Ability.checkText (DRS.ofList [])
      [ when (does thisPermanent enter)
          (mayOrElse (exile (target (and [isCard, in_ graveyard])))
            (exile (target (and [isCard, in_ yourHand])))),
        returnToTheBattlefield theExiledCard ] = [] := by
  decide

/-- Each step's arms are published on their own, so no number of options in sequence hides an
arm: after four optional choices, the exile on the first choice's second arm is still read. -/
theorem okArmAfterManyOptions :
    Ability.checkText (DRS.ofList [])
      [ when (does thisPermanent enter)
          (sequentially
            [ mayOrElse (draw 1) (exile (target (and [isCard, in_ graveyard]))),
              may (draw 1), may (draw 1), may (draw 1) ]),
        returnToTheBattlefield theExiledCard ] = [] := by
  decide

/-- Gap, recorded in the ticket: a battlefield location without "card" is read as "permanent",
what the object was, not as a zone it is in now, since `isPermanent` is that location. The card
exiled from the battlefield is read though it is in exile. -/
theorem gapBattlefieldLocationIsNotAZone :
    Ability.checkText (DRS.ofList [])
      [ when (does thisPermanent enter) (exile (target isCreature)),
        returnToTheBattlefield (the (and [was (exile which), in_ battlefield])) ] = [] := by
  decide

/-- The sacrificed creature is an object: a player context refuses it. -/
theorem badObjectsAsPlayer :
    NounPhrase.checkAt (some .player) (Ability.textContext (DRS.ofList []) sacrificeACreature)
      theSacrificed = [.kindMismatch .player .object] := by
  decide

/-- "The exiled cards" read as one object refuse a singular stat read. -/
theorem badManyObjectsStat :
    Amount.check (Ability.textContext (DRS.ofList []) exileGraveyardCards)
      (.statOf (.stat .power) (thePlural (and [was (exile which), isCard]))) = [.singular] := by
  decide

/-- Required pin: "if tribute wasn't paid" reads whether the offered choice was taken
[CR#702.104b]. A keyword with a paid cost names a source. -/
theorem okChoiceTaken :
    Condition.check (DRS.ofList []) (.not (choiceWasTaken (.byKeyword (.named "kicker")) .this)) = [] := by
  decide

/-- A linked cost is read back by its keyword's label, whichever object's cost it is: *Ertai's
Trickery* ("Counter target spell if it was kicked.") and *Merfolk Falconer* ("Whenever you cast a
kicked spell, scry 2.") read the kicker cost of a spell neither prints [CR#702.33d], and both are
spelled and check clean. -/
theorem kickedReadsAnotherSpellsKicker :
    Cards.ertaisTrickery.card.check = [] ∧ Cards.merfolkFalconer.card.check = [] :=
  ⟨Cards.ertaisTrickery.ok, Cards.merfolkFalconer.ok⟩

/-- Required pin: "exile it haunting target creature" relates two objects [CR#702.55a]. -/
theorem okRelateHaunting :
    Action.check (DRS.ofList []) (.relate .this haunting (target isCreature)) = [] := by
  decide

/-- Required pin: `relate` against a declared kind mismatch is refused: a haunting's second side
is an object, never a player [CR#702.55b]. -/
theorem badRelateKindMismatch :
    Action.check (DRS.ofList []) (.relate .this haunting .you) = [.kindMismatch .object .player] := by
  decide

/-- "the creature it haunts" [CR#702.55b]. -/
theorem okCounterpart :
    NounPhrase.checkAt (some .object) (DRS.ofList []) (.counterpart haunting .this) = [] := by
  decide

/-- "Paired" and "paired with this creature" are object predicates [CR#702.95b]. -/
theorem okRelated :
    (Condition.check (DRS.ofList []) (.matches thisCreature (.related paired none)),
      Condition.check (DRS.ofList []) (.exists_ (.described (.a .unmarked) (.and [isCreature, .related paired (some thisCreature)]))))
      = ([], []) := by
  decide

/-- The relation definitions the registry declares check clean, and a nameless one is
refused. -/
theorem okRelationDefinition :
    (RegistryDefinition.check (.relation haunting), RegistryDefinition.check (.relation (.named "")))
      = ([], [.definitionNamed]) := by
  decide

/-- Required pin: a mutating spell merges with one permanent [CR#730.2]. -/
theorem okMerge :
    Action.check (DRS.ofList []) (.merge .this (target isCreature) .top) = [] := by
  decide

/-- A merge onto a group is refused: one permanent is merged with. -/
theorem badMergeOntoGroup :
    Action.check (DRS.ofList []) (.merge .this (allOf isCreature) .bottom) = [.singular] := by
  decide

/-- Required pin: mutate's target, gained with its alternative cost [CR#702.140a]. -/
theorem okAddsTarget :
    StaticSpec.check (DRS.ofList []) (.addsTarget thisSpell
      (.and [isCreature, .not (.hasSubtype (human))])) = [] := by
  decide

/-- Required pin: offering's reduction by the sacrificed permanent's mana cost [CR#702.48a]. -/
theorem okReducedByManaCost :
    StaticSpec.check
      (Ability.textContext (DRS.ofList [])
        (static (.addedCost (.perform (sacrifice (a (and [hasSubtype goblin, isPermanent]))))
          true .once)))
      (reducedBySymbols thisSpell
        (manaCostOf (the (and [was (sacrifice which), isPermanent])))) = [] := by
  decide

/-- A written cost shift of no symbols is refused, as it was under `run`. -/
theorem badEmptySymbols :
    StaticSpec.check (DRS.ofList []) (.costShift thisSpell (.bySymbols (.down (.lit [])) false))
      = [.nonEmpty] := by
  decide

/-! ## The seven bodies

Each keyword's Definition (`Semantics.Definitions`) checks clean as an ability. -/

namespace Bodies

def cost : Cost := .mana [generic 2, pip .blue]

/-- Required pin: emerge [CR#702.119a]. -/
theorem okEmerge : Ability.check (DRS.ofList []) (emerge cost) = [] := by decide
/-- Required pin: emerge from [quality] [CR#702.119b]. -/
theorem okEmergeFrom : Ability.check (DRS.ofList []) (emergeFrom isArtifact cost) = [] := by decide
/-- Required pin: offering [CR#702.48a]. -/
theorem okOffering :
    Ability.check (DRS.ofList []) (offering (.hasSubtype (goblin))) = [] := by decide
/-- Required pin: harmonize [CR#702.180a]. -/
theorem okHarmonize : Ability.check (DRS.ofList []) (harmonize cost) = [] := by decide
/-- Required pin: haunt [CR#702.55a]. -/
theorem okHaunt : Ability.check (DRS.ofList []) haunt = [] := by decide
/-- Required pin: soulbond [CR#702.95a]. -/
theorem okSoulbond : Ability.check (DRS.ofList []) soulbond = [] := by decide
/-- Required pin: cipher [CR#702.99a]. -/
theorem okCipher : Ability.check (DRS.ofList []) cipher = [] := by decide
/-- Required pin: mutate [CR#702.140a]. -/
theorem okMutate : Ability.check (DRS.ofList []) (mutate cost) = [] := by decide
/-- Required pin: champion [CR#702.72a,607.2k]: its leaves-the-battlefield ability returns the
card its enters ability exiled. -/
theorem okChampion : Ability.check (DRS.ofList []) (champion isCreature) = [] := by decide

end Bodies

/-! ## Mutate's target

Mutate's merge reads back the target its static ability adds to the spell [CR#702.140a]: only an
added target is published stamped by targeting, so "the target" is never another ability's. -/

namespace MutateTarget

/-- "If you cast this spell for its mutate cost, put it over or under target non-Human creature you
own", reading the target back as `description` does. -/
def merges (description : NounPhrase) : Ability :=
  static
    (.replace (would thisSpell enter)
      (instead (chooseOne (.merge thisSpell description .top) (.merge thisSpell description .bottom)))
      none .repeatedly none)

def addsCreatureTarget : Ability :=
  static (.addsTarget thisSpell (and [isCreature, not (.hasSubtype human)]))

/-- Required pin: the merge reads the target added to this spell. -/
theorem okReadsAddedTarget :
    Ability.checkText (DRS.ofList []) [addsCreatureTarget, merges (the (was (target which)))] = [] := by
  decide

/-- The same target, read with this spell as the subject that targets it. -/
theorem okReadsTargetOfThisSpell :
    Ability.checkText (DRS.ofList [])
      [addsCreatureTarget, merges (the (did thisSpell (target which)))] = [] := by
  decide

/-- Required pin: with no target added, there is none to read. -/
theorem badNoAddedTarget :
    Ability.checkText (DRS.ofList []) [merges (the (was (target which)))] =
      [.participialDescription (.deed (.core .target)) 0, .participialDescription (.deed (.core .target)) 0] := by
  decide

/-- Required pin: another ability's target is not this spell's: an activated ability that taps
target creature publishes the tapped creature, not a target. -/
theorem badUnrelatedTarget :
    Ability.checkText (DRS.ofList [])
      [ activated (.mana [generic 1]) (tap (target isCreature)),
        merges (the (was (target which))) ] =
      [.participialDescription (.deed (.core .target)) 0, .participialDescription (.deed (.core .target)) 0] := by
  decide

end MutateTarget

/-- What a linked ability published is read back only by a Participial Description: a
creature card the first ability put onto the battlefield is no Contrast Anchor for the second
ability's bare "another target creature", nor a group "the rest" is taken from. The anchor is
the Source. -/
theorem linkedTargetAnchorsNoBareOther :
    let first := when (does thisCreature enter)
      (putOntoBattlefield (target (and [isCreature, in_ yourGraveyard])))
    let tapOther := tap (target (and [isCreature, isOther]))
    let context := Ability.textContext (DRS.ofList []) first
    Ability.checkText (DRS.ofList []) [first, activated (mana [generic 1]) tapOther] = [] ∧
      (stepAction 64 .postEvent .obj (nextClause context) tapOther).1.contrastAnchors
        = [.source] := by
  decide

/-! ## Linked across a created token and its source

Abilities on a token, an emblem or a permanent and on the source of the ability that created it
or put it onto the battlefield can be linked [CR#607.1d], and an ability granted to the object
the granting ability is printed on counts as printed on it [CR#607.1a]: so the text the creating
ability puts on or grants to its token reads back "the exiled card" that ability exiled
[CR#607.2a]. A grant onto another object ("that creature gains '… copy the exiled card'", Massimo,
the Magician) is read as Oracle writes it. Only a Participial Description reads the exiled card
there, never a pronoun. -/

/-- "When this token leaves the battlefield, return the exiled card to its owner's graveyard." -/
def hofriTokenAbility (exiled : NounPhrase) : Ability :=
  when (does thisToken leave) (returnTo exiled (graveyardOf (ownerOf it)) [])

/-- Hofri Ghostforge: "Whenever another nontoken creature you control dies, exile it. If you do,
create a token that's a copy of that creature, except it's a Spirit in addition to its other
types and it has 'When this token leaves the battlefield, return the exiled card to its owner's
graveyard.'" -/
def hofriGhostforge (exiled : NounPhrase) : Ability :=
  whenever (does (a (and [isCreature, isNontoken, youControl, isOther])) die)
    (doIfDone (exile it)
      (some (create 1 (copyOf (that creature)
        [isAlso [] [spirit], alsoHas (hofriTokenAbility exiled)]) [])))

/-- Required pin: Hofri Ghostforge's trigger as printed. "A copy of that creature" reads the
creature that died by its copiable values, its last known information, though it has since been
exiled [CR#707.2,608.2h]; the token's ability reads back the card Hofri's ability exiled
[CR#607.1d,607.2a], and "that card" finds nothing there. -/
theorem okHofriGhostforge :
    Ability.check (DRS.ofList []) (hofriGhostforge theExiledCard) = [] ∧
      Ability.check (DRS.ofList []) (hofriGhostforge (that card)) =
        [.anaphor (.word .card) .one 0] := by
  decide

/-- Brenard, Ginger Sculptor: "Whenever another nontoken creature you control dies, you may exile
it. If you do, create a token that's a copy of that creature, except it's a 1/1 Food Golem
artifact creature in addition to its other types and it has '{2}, {T}, Sacrifice this token: You
gain 3 life.'" The copy reads the creature that died, exiled since [CR#707.2,608.2h]. -/
theorem okBrenardGingerSculptor :
    Ability.check (DRS.ofList [])
      (whenever (does (a (and [isCreature, isNontoken, youControl, isOther])) die)
        (may (exile it)
          (some (create 1 (copyOf (that creature)
            [ isAlso [.artifact, .creature] [food, golem], hasPT 1 1,
              alsoHas (activated (compound [[generic 2], tapSymbol, sacrifice thisToken])
                (gainLife 3)) ]) [])))) = [] := by
  decide

/-- Cemetery Puca: "Whenever a creature dies, you may pay {1}. If you do, this creature becomes a
copy of that creature, except it has this ability." [CR#707.2,608.2h] -/
theorem okCemeteryPuca :
    Ability.check (DRS.ofList [])
      (whenever (does (a isCreature) die)
        (may (pay [generic 1] .once)
          (some (.establish (copyChange thisCreature (that creature) [exceptThisAbility])
            none)))) = [] := by
  decide

/-- Timothar, Baron of Bats: "Whenever another nontoken Vampire you control dies, you may pay {1}
and exile it. If you do, create a 1/1 black Bat creature token with flying. It gains 'When this
token deals combat damage to a player, sacrifice it and return the exiled card to the
battlefield tapped.'" -/
def timotharBaronOfBats : Ability :=
  whenever (does (a (and [vampire, isNontoken, youControl, isOther])) die)
    (may (sequentially [pay [generic 1] .once, exile it])
      (some (sequentially
        [ create 1 (written { characteristics :=
            { colors := [.black], types := [.creature], subtypes := [bat],
              text := [flying], power := stat 1, toughness := stat 1 } }),
          gain it
            (when (does thisToken (dealDamage (to := a isPlayer) (kind := some .combat)))
              (sequentially
                [ sacrifice it,
                  returnToBattlefield theExiledCard [.entersAs .tapped] ])) ])))

/-- Required pin: the ability the token gains reads back the card Timothar's ability exiled
[CR#607.1a,607.1d,607.2a]. -/
theorem okTimotharBaronOfBats : Ability.check (DRS.ofList []) timotharBaronOfBats = [] := by
  decide

/-- An emblem's abilities are linked to its creator's the same way [CR#607.1d]: "exile it. If
you do, you get an emblem with 'When this leaves …, return the exiled card …'" reads the exiled
card back, and not by a pronoun. -/
theorem okEmblemReadsTheExiledCard :
    let emblemOver (exiled : NounPhrase) : Ability :=
      when (does thisCreature die) (doIfDone (exile it)
        (some (getEmblem [whenever (does you (cast (a isSpell)))
          (returnTo exiled (graveyardOf (ownerOf it)) [])])))
    Ability.check (DRS.ofList []) (emblemOver theExiledCard) = [] ∧
      Ability.check (DRS.ofList []) (emblemOver (that card)) = [.anaphor (.word .card) .one 0] := by
  decide

/-! ## Put onto the battlefield with this

"Enchant creature put onto the battlefield with Necromancy" reads the creature Necromancy's
entering ability puts onto the battlefield [CR#607.2c]: the ability granting the enchant ability
is printed on the object it is granted to, so the two are linked [CR#607.1a], and the enchant
ability is linked to the whole granting ability, the step after the grant that puts the creature
included [CR#607.1]. In a keyword's quality, "put onto the battlefield with [this object]" reads
the objects the linked ability put there, as a Participial Description does; anywhere else, or by
any other agent, it looks back over nothing. -/

namespace LinkedEntry

open Semantics.Cards (creatureCardInAGraveyard)

/-- "creature put onto the battlefield [by <agent>]" -/
def putBy (agent : Subject) : Predicate := and [isCreature, did agent (putOntoBattlefield which)]

/-- "it becomes an Aura with 'enchant creature put onto the battlefield [by <agent>]'" -/
def becomesAuraEnchanting (agent : Subject) : Action :=
  establish (conjunction [addSubtype it aura, abilityGrant it (enchant (putBy agent))]
    (subject := some it))

/-- "Put target creature card from a graveyard onto the battlefield under your control." -/
def putTargetCreatureCard : Action :=
  putOntoBattlefieldUnderYourControl (target creatureCardInAGraveyard)

/-- "When this enchantment enters, <steps>." -/
def entering (steps : List Action) : Ability :=
  when (does thisEnchantment enter) (sequentially steps)

/-- Required pin: Necromancy as printed [CR#607.2c,603.7a]. -/
theorem okNecromancy : Semantics.Cards.necromancy.card.check = [] :=
  Semantics.Cards.necromancy.ok

/-- Required pin: Animate Dead as printed; its return puts the creature onto the battlefield
[CR#607.2c]. -/
theorem okAnimateDead : Semantics.Cards.animateDead.card.check = [] :=
  Semantics.Cards.animateDead.ok

/-- Required pin: Dance of the Dead as printed. -/
theorem okDanceOfTheDead : Semantics.Cards.danceOfTheDead.card.check = [] :=
  Semantics.Cards.danceOfTheDead.ok

/-- The granted quality reads the put whichever side of the grant it is on [CR#607.1,607.1a]. -/
theorem okPutBeforeOrAfterTheGrant :
    Ability.check (DRS.ofList [])
        (entering [becomesAuraEnchanting (by_ this_), putTargetCreatureCard]) = [] ∧
      Ability.check (DRS.ofList [])
        (entering [putTargetCreatureCard, becomesAuraEnchanting (by_ this_)]) = [] := by
  decide

/-- With nothing linked put onto the battlefield, the quality reads nothing: the object whose
entering triggered the ability was not put there by it [CR#607.1]. Each of enchant's three reads
of its quality is refused. -/
theorem badNothingPutWithThis :
    Ability.check (DRS.ofList []) (entering [becomesAuraEnchanting (by_ this_)]) =
      List.replicate 3 (.participialDescription (.zoneChange none .battlefield) 0) := by
  decide

/-- Required pin: outside a keyword's quality, "put onto the battlefield with this" is still an
unanchored lookback, in a later line of the card's text and in text the ability grants. -/
theorem badLookbackOutsideAKeywordQuality :
    let destroyPut := activated (mana [generic 1]) (destroy (target (putBy (by_ this_))))
    Ability.checkText (DRS.ofList []) [entering [putTargetCreatureCard], destroyPut] =
        [.unanchoredLookback] ∧
      Ability.check (DRS.ofList [])
        (entering [establish (abilityGrant it destroyPut), putTargetCreatureCard]) =
        [.unanchoredLookback] := by
  decide

/-- Required pin: an agent other than this object is no linking phrase: "put onto the
battlefield by an artifact" is refused, and so is the quality with no agent written. -/
theorem badAgentOtherThanThis :
    Ability.check (DRS.ofList [])
        (entering [becomesAuraEnchanting (by_ (a isArtifact)), putTargetCreatureCard]) =
        [.unanchoredLookback, .unanchoredLookback, .participialAgent] ∧
      Ability.check (DRS.ofList [])
        (entering [becomesAuraEnchanting .any, putTargetCreatureCard]) =
        [.unanchoredLookback, .unanchoredLookback] := by
  decide

/-- Two abilities of the object put creatures onto the battlefield, so the quality could read
either's [CR#607.2c]. -/
theorem badTwoLinkedPuts :
    Ability.checkText (DRS.ofList [])
      [ entering [putTargetCreatureCard], entering [putTargetCreatureCard],
        enchant (putBy (by_ this_)) ] =
      List.replicate 2 (.participialDescription (.zoneChange none .battlefield) 2) := by
  decide

/-- Granted text reads the exiled card its granting ability exiles after the grant, as it does
one exiled before [CR#607.1a,607.2a]. -/
theorem okGrantReadsLaterExile :
    let grant := establish (abilityGrant it
      (when (does thisEnchantment leave) (returnToBattlefield theExiledCard [])))
    let exileCard := exile (target creatureCardInAGraveyard)
    Ability.check (DRS.ofList []) (entering [grant, exileCard]) = [] ∧
      Ability.check (DRS.ofList []) (entering [exileCard, grant]) = [] := by
  decide

end LinkedEntry

/-- Stormscape Battlemage: "When this creature enters, if it was kicked with its first kicker,
you gain 3 life." [CR#702.33f]. -/
theorem okFirstKickerRead :
    Ability.check (DRS.ofList [])
      (when (does thisCreature enter)
        (if_ := costWasPaid (.byNthKeyword (.nth 1) kicker) none thisCreature) (gainLife 3)) = [] := by
  decide

/-- "… if it was kicked with its zeroth kicker …": a Link Source's ordinal counts from the first
of its keyword's costs [CR#702.33f], so the zeroth names none. -/
theorem badZerothKickerRead :
    Ability.check (DRS.ofList [])
      (when (does thisCreature enter)
        (if_ := costWasPaid (.byNthKeyword (.nth 0) kicker) none thisCreature) (gainLife 3)) =
      [.linkedFacetNamed] := by
  decide

end Semantics.Proofs.LinkedReadback
