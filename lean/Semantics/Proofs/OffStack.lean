import Semantics.Definitions
import Semantics.Check.Card

/-!
# Semantics.Proofs.OffStack

Off-Stack Statics (`semantics-v2-off-stack-statics`, `docs/decisions/semantics-v2.md` §7, ruling
2026-10-09): "as [event], do" (`StaticSpec.asEvent`) and "any time [condition] holds, do"
(`StaticSpec.whileHolds`), which carry an action that never uses the stack [CR#113.9]; the delayed
triggers dash, blitz and warp create as the permanent is put onto the battlefield
[CR#608.3g,603.7d]; the entering an Aura's enchant ability and sunburst replace instead
[CR#614.1c,303.4f,303.4g,702.44a]; the object a spell becomes (`NounPhrase.becomes`, [CR#400.7]);
and a deontic rule's one exempt source (`DeonticExcept`, [CR#702.145b,702.13b]). The keyword bodies are
read through their declarations (`Semantics.Definitions`). Each pin is closed by `decide` or `rfl`.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.OffStack

/-! ## The nodes -/

/-- "As this permanent enters, you may draw a card": an action carried out as part of the
entering it replaces, which still happens [CR#614.1c]. -/
theorem okAsEnteringAlso :
    StaticSpec.check (DRS.ofList [])
      (replace (would thisPermanent enter) (also (may (draw 1)))) = [] := by
  decide

/-- "As it becomes night, transform this permanent": an action carried out as an event that is
no permanent's entering happens [CR#702.145b]. -/
theorem okAsEventBecomesNight :
    StaticSpec.check (DRS.ofList [])
      (.asEvent (gameBecomes night) (transform thisPermanent)) = [] := by
  decide

/-- "Any time you control ten or more permanents and you don't have the city's blessing, you get
the city's blessing for the rest of the game" [CR#702.131b]. -/
theorem okWhileHoldsCitysBlessing :
    StaticSpec.check (DRS.ofList [])
      (.whileHolds
        (.and
          [ .compareAmt (.countOf (.described .all (.and [isPermanent, .hasPossessor .controller .you])))
              .atLeast 10,
            .not (.matches .you (.hasDesignation (.named "citysBlessing" (.heldBy .player)) none)) ])
        (.gainDesignation .you (.named "citysBlessing" (.heldBy .player)) (some (.between none none)))) = [] := by
  decide

/-- "Return the permanent this spell becomes to its owner's hand" [CR#400.7,702.109a]. -/
theorem okBecomesThisSpell :
    Action.check (DRS.ofList []) (putInto (.becomes thisSpell) hand) = [] := by decide

/-- Only an object changes zones [CR#400.7]: what a player "becomes" is refused as a kind. -/
theorem badBecomesOfAPlayer :
    NounPhrase.checkAt none (DRS.ofList []) (.becomes .you) = [.kindMismatch .object .player] := by decide

/-- "This permanent can't transform except due to its daybound ability" [CR#702.145b]. -/
theorem okExceptByKeyword :
    StaticSpec.check (DRS.ofList [])
      (.cant .any (transform thisPermanent)
        { except := some (.byKeyword (.named "daybound")) }) = [] := by
  decide

/-- "This creature can't be blocked except by artifact creatures and/or black creatures"
[CR#702.36b]. -/
theorem okExceptByObjects :
    StaticSpec.check (DRS.ofList [])
      (cant any (block thisCreature)
        (except := byObjects (bare (and [isCreature, or [isArtifact, hasColor .black]])))) = [] := by
  decide

/-- An exception by objects describes objects: "except by opponents" is refused as a kind. -/
theorem badExceptByPlayers :
    StaticSpec.check (DRS.ofList [])
      (cant any (block thisCreature) (except := byObjects (bare isOpponent))) =
        [.kindMismatch .object .player] := by
  decide

/-! ## The keyword bodies -/

/-- Dash {1}{R} [CR#702.109a]. -/
theorem okDash : Ability.check (DRS.ofList []) (dash (.mana [generic 1, pip .red])) = [] := by decide

/-- Blitz {2}{R} [CR#702.152a]. -/
theorem okBlitz : Ability.check (DRS.ofList []) (blitz (.mana [generic 2, pip .red])) = [] := by decide

/-- Warp {3} [CR#702.185a]. -/
theorem okWarp : Ability.check (DRS.ofList []) (warp (.mana [generic 3])) = [] := by decide

/-- Rebound [CR#702.88a]. -/
theorem okRebound : Ability.check (DRS.ofList []) rebound = [] := by decide

/-- Ascend [CR#702.131a,702.131b]. -/
theorem okAscend : Ability.check (DRS.ofList []) ascend = [] := by decide

/-- Storied [CR#702.195a]. -/
theorem okStoried : Ability.check (DRS.ofList []) storied = [] := by decide

/-- Daybound [CR#702.145b]. -/
theorem okDaybound : Ability.check (DRS.ofList []) daybound = [] := by decide

/-- Nightbound [CR#702.145e]. -/
theorem okNightbound : Ability.check (DRS.ofList []) nightbound = [] := by decide

/-- Devour 1 [CR#702.82a]. -/
theorem okDevour : Ability.check (DRS.ofList []) (devour 1) = [] := by decide

/-- Devour artifact 2 [CR#702.82c]: devour's general form, labelled devour. Re-spelled from
`okDevourQualityExpansion`, which checked the same definition as a helper before the general form
was declared (`semantics-v2-devour-quality-keyword`). -/
theorem okDevourQuality :
    Ability.check (DRS.ofList []) (devourQuality isArtifact 2) = [] := by decide

/-- Riot [CR#702.136a]. -/
theorem okRiot : Ability.check (DRS.ofList []) riot = [] := by decide

/-- Tribute 3 [CR#702.104a]. -/
theorem okTribute : Ability.check (DRS.ofList []) (tribute 3) = [] := by decide

/-- Unleash [CR#702.98a]. -/
theorem okUnleash : Ability.check (DRS.ofList []) unleash = [] := by decide

/-- Fear, re-spelled onto its Deontic Exception [CR#702.36b]. -/
theorem okFear : Ability.check (DRS.ofList []) fear = [] := by decide

/-- Intimidate, re-spelled onto its Deontic Exception [CR#702.13b]. -/
theorem okIntimidate : Ability.check (DRS.ofList []) intimidate = [] := by decide

/-- Amplify 1 [CR#702.38a]. -/
theorem okAmplify : Ability.check (DRS.ofList []) (amplify 1) = [] := by decide

/-! ## The readbacks (`semantics-v2-linked-readback`) -/

/-- "When this creature enters, draw a card for each creature it devoured." -/
def drawForEachDevoured : Ability :=
  when (does thisCreature enter)
    (doForEach (each (and [isCreature, did it (devour which)])) (draw 1))

/-- Skullmulcher: "Devour 1 … When this creature enters, draw a card for each creature it
devoured." [CR#702.82b]: "it devoured" reads back the creatures devour's entry sacrificed, the
objects of a linked ability. -/
theorem okItDevouredCount :
    Ability.checkText (DRS.ofList []) [devour 1, drawForEachDevoured] = [] := by
  decide

/-- Voracious Dragon: "When this creature enters, it deals damage to any target equal to twice
the number of Goblins it devoured." [CR#702.82b] -/
theorem okGoblinsItDevoured :
    Ability.checkText (DRS.ofList [])
      [ devour 1,
        when (does thisCreature enter)
          (dealDamage (times 2 (countOf (and [hasSubtype goblin, did it (devour which)])))
            (target isAnyTarget)) ] = [] := by
  decide

/-- Without devour, it devoured nothing; and a creature another ability sacrificed was not
devoured [CR#702.82b]. -/
theorem badDevouredWithoutDevour :
    Ability.checkText (DRS.ofList []) [drawForEachDevoured] =
        [.participialDescription (.deed (.ofAbility (.named "devour"))) 0] ∧
      Ability.checkText (DRS.ofList [])
        [ static (.addedCost (.perform (sacrifice (a isCreature))) false .once),
          drawForEachDevoured ] =
        [.participialDescription (.deed (.ofAbility (.named "devour"))) 0] := by
  decide

/-- "If tribute wasn't paid" [CR#702.104b]: the tribute choice not taken. -/
theorem okTributeWasntPaid :
    Condition.check (DRS.ofList []) (.not (choiceWasTaken (.byKeyword (.named "tribute")) thisCreature)) = [] := by
  decide

/-! ## Delayed triggers created as the permanent enters -/

/-- "as that permanent is put onto the battlefield" from the stack, as the spell resolves
[CR#608.3a,608.3g]. -/
def putOntoTheBattlefieldAsItResolves : GameEvent :=
  does .any (.move thisPermanent stack battlefield .arrival [])

/-- The Off-Stack Statics of a keyword's body: their events and actions. -/
def offStackStatics : Ability → List (GameEvent × Action)
  | .keyword _ _ body => body.filterMap fun
    | .static (.asEvent event action) => some (event, action)
    | _ => none
  | _ => []

/-- Dash, blitz and warp each create their delayed trigger as the permanent is put onto the
battlefield as the spell resolves, not as the cost is paid [CR#608.3g]: a countered spell, which
never becomes a permanent, creates none, and a copy, which becomes a token permanent as it is
put onto the battlefield [CR#608.3f], creates its own. -/
theorem delayedTriggersWaitForThePermanent :
    ((offStackStatics (dash (mana [generic 1]))).map (·.1),
      (offStackStatics (blitz (mana [generic 1]))).map (·.1),
      (offStackStatics (warp (mana [generic 1]))).map (·.1))
      = ([putOntoTheBattlefieldAsItResolves], [putOntoTheBattlefieldAsItResolves],
        [putOntoTheBattlefieldAsItResolves]) := by
  rfl

/-- Blitz's delayed trigger, guarded by its own cost and naming the permanent directly: its
controller is the spell's controller as it resolved, who controls the permanent it sacrifices
[CR#603.7d,701.21a]. -/
theorem blitzSacrificesThePermanent :
    (offStackStatics (blitz (mana [generic 1]))).map (·.2)
      = [doIf (keywordCostPaid (.named "blitz") thisPermanent)
          (atNext .endStep (sacrifice thisPermanent))] := by
  rfl

theorem okDashBlitzWarp :
    (Ability.check (DRS.ofList []) (dash (mana [generic 1])),
      Ability.check (DRS.ofList []) (blitz (mana [generic 1])),
      Ability.check (DRS.ofList []) (warp (mana [generic 1]))) = ([], [], []) := by
  decide

/-! ## An Aura's entry -/

/-- The replacements of a keyword's body: what each watches and what it makes of the event. -/
def replacements : Ability → List (WouldClause × Replacement)
  | .keyword _ _ body => body.filterMap fun
    | .static (.replace event replacement _ _ _) => some (event, replacement)
    | _ => none
  | _ => []

/-- An Aura with no legal object or player to enchant, under its enchant ability and any other
applicable effects, doesn't enter: instead it stays where it is, or goes to its owner's graveyard
from the stack [CR#303.4g]; with one, an Aura entering other than by resolving, by an effect that
doesn't name its host, enters attached to a legal host [CR#303.4f]; an Aura put onto the
battlefield attached to a host the effect names that isn't a legal host, or is undefined, doesn't
enter either [CR#303.4i]. Legality is denoted (`legalHostOf`), not approximated by the enchant
quality, which protection [CR#702.16c] can overrule. None is an Off-Stack Static. -/
theorem enchantReplacesTheEntering :
    (offStackStatics (enchant isCreature), replacements (enchant isCreature))
      = ([],
        [ (would thisAura enter,
            doIf (not (exists_ (legalHostOf thisAura)))
              (instead (doIf (is thisAura (in_ stack)) (putInto thisAura graveyard)))),
          (would .any
              (.move thisAura (not stack) battlefield .arrival [.not (.attachedTo .any)]),
            doIf (exists_ (legalHostOf thisAura))
              (also (attachTo thisAura (a (legalHostOf thisAura))))),
          (would thisAura
              (enter (riders := [.attachedTo .any, .not (.attachedTo (a (legalHostOf thisAura)))])),
            instead (doIf (is thisAura (in_ stack)) (putInto thisAura graveyard))) ]) := by
  rfl

/-- The [CR#303.4f] choice applies only where "the effect putting it onto the battlefield doesn't
specify the object or player the Aura will enchant": its pattern excludes every entering that
names a host, and only such an entering keys [CR#303.4i]. The [CR#303.4g] entering names no
riders, so it watches every entering. -/
theorem enchantChoiceExcludesANamedHost :
    ((replacements (enchant isCreature)).map (·.1)).filterMap (fun
      | .would _ (.move _ _ _ _ riders) => some riders
      | _ => none)
      = [ [],
          [.not (.attachedTo .any)],
          [.attachedTo .any, .not (.attachedTo (a (legalHostOf thisAura)))] ] := by
  rfl

/-- A negated rider is named only in an event pattern: an action carries the riders its text
writes. The host an effect names is an object or a player [CR#303.4b,701.3a]: a player may
be one, a color may not. -/
theorem riderNegationIsOnlyAPattern :
    Action.check (DRS.ofList [])
        (putWith (target isCreature) battlefield [.not (.entersAs .tapped)]) =
      [.riderNegationPattern] ∧
    Action.check (DRS.ofList [])
        (putOntoBattlefieldAttachedTo (a (and [hasSubtype aura, cardIn yourHand]))
          (target isPlayer)) = [] ∧
    Action.check (DRS.ofList [])
        (putOntoBattlefieldAttachedTo (a (and [hasSubtype aura, cardIn yourHand]))
          (a (quality .color))) =
      [.kindLte (.quality .color) (.join .object .player)] := by
  decide

theorem okEnchant : Ability.check (DRS.ofList []) (enchant isCreature) = [] := by decide

/-! ## Sunburst -/

/-- Sunburst asks whether its object enters as a creature with type-changing effects ignored,
and adds counters only if colored mana was spent to cast it [CR#702.44a,702.44b]. -/
theorem sunburstIgnoresTypeChanges :
    (replacements sunburst).map (·.2)
      = [ doIf
            (and
              [ is thisPermanent (hasTypeIgnoringTypeChanges .creature),
                coloredManaSpentToCast thisSpell ])
            (withRiders [.withCounters (colorsSpentToCast thisSpell) p1p1Counter .fresh]),
          doIf
            (and
              [ not (is thisPermanent (hasTypeIgnoringTypeChanges .creature)),
                coloredManaSpentToCast thisSpell ])
            (withRiders [.withCounters (colorsSpentToCast thisSpell) chargeCounter .fresh]) ] := by
  rfl

theorem okSunburst : Ability.check (DRS.ofList []) sunburst = [] := by decide
end Semantics.Proofs.OffStack
