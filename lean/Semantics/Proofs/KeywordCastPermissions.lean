import Semantics.Definitions
import Semantics.Check.Card
import Semantics.Proofs.Rules

/-!
# Semantics.Proofs.KeywordCastPermissions

Keyword cast permissions and costs encode the kind of rule the CR gives them
(`lean-v3-keyword-cast-permissions-and-costs`). Plot and foretell exile from the hand
[CR#702.170a,702.143a,113.6m] and make the card plotted or foretold, a state of a card in exile
that a trigger, an effect, a spell's history and the rule rows for plotted and foretold cards
(`Proofs/Rules.lean`) read [CR#702.170c,702.170d,702.143c,702.143d]; a megamorph
cost is a morph cost and a multikicker cost a kicker cost [CR#702.37b,702.33c]; mutate's target
exists only once its cost is chosen [CR#601.2c,702.140a]; retrace and jump-start add an
additional cost to the mana cost or an alternative cost [CR#702.81a,702.133a]; splice is a
cast-time reveal, not a replacement of the cast [CR#601.2b,702.47a]; aftermath excludes every zone
but a graveyard [CR#702.127a]; flashback's spell is an instant or sorcery [CR#702.34a].
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.KeywordCastPermissions

def cost : Cost := .mana [generic 1, pip .blue]

/-- The component abilities of a keyword ability's definition. -/
def components : Ability → List Ability
  | .keyword _ _ body => body
  | _ => []

/-- The steps of the special action a keyword's first component takes. -/
def specialActionSteps : Ability → List Action
  | .keyword _ _ (.static (.specialAction _ _ (.sequentially steps)) :: _) => steps
  | _ => []

/-! ## Plot and foretell exile from the hand -/

/-- Plot exiles this card from your hand, so it functions only there [CR#702.170a,113.6m]. -/
theorem plotExilesThisCardFromYourHand :
    (specialActionSteps (plot cost)).head? = some (exileFrom thisCard yourHand) := rfl

/-- Foretell exiles this card from your hand face down [CR#702.143a,113.6m]. -/
theorem foretellExilesThisCardFromYourHandFaceDown :
    (specialActionSteps (foretell cost)).head? = some (exileFaceDownFrom thisCard yourHand) :=
  rfl

/-! ## Plotted and foretold are states of a card in exile -/


/-- After the exile, the card is a foretold card [CR#702.143c]. -/
theorem foretellMakesTheCardForetold :
    (specialActionSteps (foretell cost))[1]? = some (gainDesignation it foretold) := rfl

/-- Plot's text ends at "It becomes a plotted card" [CR#702.170a]: its becoming plotted is an
event of its own, after the exile, and the special action grants no cast right. -/
theorem plotSpecialActionEndsAtThePlottedState :
    specialActionSteps (plot cost) = [exileFrom thisCard yourHand, gainDesignation it plotted] :=
  rfl

/-- The plotted card's cast right is the rule row for every plotted card in exile [CR#702.170d]. -/
theorem plottedCardRuleCarriesTheCastRight :
    Proofs.Rules.plottedCardCast.scope = .and [.isCard, hasDesignation plotted, in_ exileZone] ∧
    Proofs.Rules.plottedCardCast.confer =
      static
        (onlyUnless
          (can you (cast this_ (from_ := exileZone) (payment := withoutPaying))
            (timing := asSorcery))
          (did any (gainDesignation this_ plotted) soFarThisTurn)) := ⟨rfl, rfl⟩

/-- The foretold card's look and cast permissions are keyed on its being foretold
[CR#702.143a,702.143d]. -/
theorem foretellPermissionsAreKeyedOnTheForetoldState :
    (specialActionSteps (foretell cost)).drop 2 =
      [ establishThroughout (visibility .lookAt you (objects it))
          (forAsLongAs (is it (hasDesignation foretold))),
        establishThroughout
          (onlyWhile
            (can you
              (cast it (from_ := exileZone) (payment := payingInstead (.inCaller cost))))
            (is it (hasDesignation foretold)))
          onALaterTurn ] := rfl

/-- "When this card becomes plotted, …": the card's gaining the plotted state is an event. -/
theorem okWhenThisCardBecomesPlotted :
    Ability.check (DRS.ofList []) (when (does any (gainDesignation thisCard plotted)) (draw 1))
      = [] := by
  decide

/-- An effect's "it becomes plotted" [CR#702.170c]. -/
theorem okTargetExiledCardBecomesPlotted :
    Action.check (DRS.ofList [])
      (gainDesignation (target (and [isCard, in_ exileZone])) plotted) = [] := by
  decide

/-- "Each foretold card you own in exile" [CR#702.143c]. -/
theorem okEachForetoldCardYouOwnInExile :
    NounPhrase.check (DRS.ofList [])
      (allOf (and [isCard, hasDesignation foretold, ownedBy you, in_ exileZone])) = [] := by
  decide

/-- "If this spell was foretold": a spell that was a foretold card before it was cast
[CR#702.143c]. -/
theorem okIfThisSpellWasForetold :
    Condition.check (DRS.ofList []) (is thisSpell (wasBeforeCast foretold)) = [] := by
  decide

/-- A spell was a card before it was cast, so the state it held is one an object holds. -/
theorem badSpellWasThePlayerDesignation :
    Condition.check (DRS.ofList []) (is thisSpell (wasBeforeCast monarch)) =
      [.designationHolder monarch .object] := by
  decide

/-! ## Megamorph is a morph cost; multikicker is a kicker cost -/

/-- A megamorph cost is a morph cost [CR#702.37b]. -/
theorem megamorphIsMorph : keywordOf (megamorph cost) = .the (.named "morph") := by decide

/-- A multikicker cost is a kicker cost, so a multikicked spell is kicked [CR#702.33c,702.33d]:
"its kicker cost" names it. -/
theorem multikickerIsKicker : keywordOf (multikicker cost) = keywordOf (kicker cost) := by
  decide

/-- Multikicker's body: an additional cost that may be paid any number of times [CR#702.33c]. -/
theorem multikickerMayBePaidAnyNumberOfTimes :
    components (multikicker cost) =
      [static (.addedCost (.inCaller cost) true .anyNumberOfTimes)] := rfl

/-- Megamorph is morph's two abilities and a replacement of the turning face up that puts the
counter on as the permanent turns [CR#702.37b,614.1e]; its special action only turns it face up. -/
theorem megamorphCounterIsOnAsItTurnsFaceUp :
    components (megamorph cost) =
      components (morph cost) ++
        [ static
            (replace (would any (turnFaceUp thisPermanent))
              (doIf (keywordCostPaid (.named "morph") thisPermanent)
                (also (putCounters 1 p1p1Counter thisPermanent)))) ] := rfl

theorem okMultikicker : Ability.check (DRS.ofList []) (multikicker cost) = [] := by decide

/-! ## Mutate's added target depends on choosing the mutate cost -/

/-- Mutate's target is required only once its alternative cost is chosen [CR#601.2b,601.2c]. -/
theorem mutateTargetsOnlyOnceItsCostIsChosen :
    (components (mutate cost))[1]? =
      some
        (static
          (onlyWhile
            (.addsTarget thisSpell
              (and [isCreature, not (.hasSubtype human), ownedBy (ownerOf thisSpell)]))
            (choiceWasTaken (.byKeyword (.named "mutate")) thisSpell))) := rfl

/-- The merge is gated by the same "if you do" as the target [CR#702.140a]. -/
theorem mutateMergeIsGatedByTheSameChoice :
    (components (mutate cost))[2]? =
      some
        (static
          (.replace (would thisSpell enter)
            (doIf (choiceWasTaken (.byKeyword (.named "mutate")) thisSpell)
              (instead
                (chooseOne
                  (.merge thisSpell (the (was (target which))) .top)
                  (.merge thisSpell (the (was (target which))) .bottom))))
            none .repeatedly none)) := rfl

/-- "If you cast this spell for its mutate cost, …", reading the target the spell gains. -/
def merges (description : NounPhrase) : Ability :=
  static
    (.replace (would thisSpell enter)
      (instead (chooseOne (.merge thisSpell description .top) (.merge thisSpell description .bottom)))
      none .repeatedly none)

/-- The target a conditional `addsTarget` adds is still the one the merge reads back. -/
theorem okConditionalAddedTargetIsReadBack :
    Ability.checkText (DRS.ofList [])
      [ static
          (onlyWhile (.addsTarget thisSpell (and [isCreature, not (.hasSubtype human)]))
            (choiceWasTaken (.byKeyword (.named "mutate")) thisSpell)),
        merges (the (was (target which))) ] = [] := by
  decide

/-! ## Retrace and jump-start pay their own cost plus an additional cost -/

/-- Retrace's discard is added to the mana cost or an alternative cost, so a retrace cast can use
an alternative cost [CR#702.81a,118.9a]. -/
theorem retracePaysPlusAnAdditionalCost :
    components retrace =
      [ static
          (can you
            (cast this_ (from_ := yourGraveyard)
              (payment := plusAdditional (.perform (discard (a (and [.isCard, isLand]))))))) ] :=
  rfl

theorem okRetrace : Ability.check (DRS.ofList []) retrace = [] := by decide

/-- Jump-start's two abilities [CR#702.133a]. -/
theorem okJumpStart : Ability.check (DRS.ofList []) jumpStart = [] := by decide

/-- Jump-start's exile applies only to a spell cast using it. -/
theorem jumpStartExileIsGatedOnAJumpStartCast :
    (components jumpStart)[1]? =
      some
        (static
          (replace (would thisSpell (leave stack))
            (doIf (choiceWasTaken (.byKeyword (.named "jumpStart")) thisSpell)
              (instead (exile thisSpell))))) := rfl

/-! ## Splice adds to the cast -/

/-- Splice reveals the card as the spell is cast, adding a cost and text; nothing replaces the
cast [CR#601.2b,702.47a,702.47b]. -/
theorem spliceIsACastTimeReveal (quality : Predicate) :
    components (Ability.splice quality cost) =
      [ static
          (.revealedAsCast (a (and [isSpell, .inCaller quality]))
            (.conjunction none
              [ .addedCost (.inCaller cost) false .once,
                .textChange (that .spell) (.appendTextOf this_) ])) ] := rfl

/-- Two cards with splice onto Arcane check as two statics. The order of their text is a choice
made as the spell is cast [CR#702.47b], which the term does not encode. -/
theorem okTwoSpliceAbilities :
    Ability.checkAll (DRS.ofList [])
      [Ability.splice (.hasSubtype arcane) cost, Ability.splice (.hasSubtype arcane) cost] = [] := by
  decide

/-- The card is revealed as a spell is cast: a player is no spell, and the granted text change
then has no spell to read. -/
theorem badRevealedAsAPlayerIsCast :
    StaticSpec.check (DRS.ofList [])
      (.revealedAsCast you (.textChange (that .spell) (.appendTextOf this_))) =
      [.kindMismatch .object .player, .anaphor (.word .spell) .one 0] := by
  decide

/-! ## Aftermath: any graveyard; flashback: the resulting spell -/

/-- Aftermath's half can't be cast from any zone other than a graveyard, any player's
[CR#702.127a]. -/
theorem aftermathExcludesEveryZoneButAGraveyard :
    (components aftermath)[1]? =
      some (static (cant theActor (cast this_ (from_ := some (not graveyard))))) := rfl

/-- Flashback casts only an instant or sorcery spell [CR#702.34a]. -/
theorem flashbackResultingSpellIsAnInstantOrSorcery :
    (components (flashback cost)).head? =
      some
        (static
          (can you
            (cast this_ (from_ := yourGraveyard) (payment := .payingInstead (.inCaller cost))
              (resulting := some isInstantOrSorcery)))) := rfl

theorem okAftermath : Ability.check (DRS.ofList []) aftermath = [] := by decide
theorem okFlashback : Ability.check (DRS.ofList []) (flashback cost) = [] := by decide

end Semantics.Proofs.KeywordCastPermissions
