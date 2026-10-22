import Semantics.Definitions
import Semantics.Check.Card

/-!
# Semantics.Proofs.Choice

Port of `idris/src/Experimental/Proofs/Choice.idr`: the pins of the Choice family, in theorem
form, each closed by `decide`. The names and the sentences are the Idris ones.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.Choice

private def actionResult (drs : DRS) (action : Action) : DRS :=
  (stepAction 64 .postEvent .obj drs action).1

/-- "the chosen number", one number choice standing -/
theorem okChosenNumberOneStanding : Amount.check [qualityB .number] chosenNumber = [] := by decide

/-- "... is equal to the chosen number" -/
theorem badChosenNumberTwoStanding :
    Amount.check [qualityB .number, qualityB .number] chosenNumber
      = [.choiceRef .theChoice (.quality .number) 2] := by
  decide

/-- "of the chosen color" -/
theorem okChosenColorRead : Predicate.checkAt .object [qualityB .color] (ofChosen .color) = [] := by
  decide

/-- "of the chosen number" -/
theorem badChosenNumberRead :
    Predicate.checkAt .object [qualityB .number] (ofChosen .number)
      = [.chosenQualityRead .number] := by
  decide

/-- "Choose target creature. You gain life equal to its power." -/
theorem okSinglePower :
    Action.check (DRS.ofList []) (.sequentially [choose (target isCreature), gainLife (powerOf it)])
      = [] := by
  decide

/-- "Choose two target creatures. You gain life equal to their power." -/
theorem badGroupPower :
    Action.check (DRS.ofList [])
      (.sequentially
        [choose (.described (.target (exactly 2)) isCreature), gainLife (powerOf them)])
      = [.singular] := by
  decide

/-- "Choose target creature. Its owner loses 1 life." -/
theorem okSingleOwner :
    Action.check (DRS.ofList [])
      (.sequentially [choose (target isCreature), instruct (ownerOf it) (loseLife 1)]) = [] := by
  decide

/-- "Choose two target creatures. Their owners each lose 1 life." -/
theorem okGroupOwners :
    Action.check (DRS.ofList [])
      (.sequentially
        [choose (.described (.target (exactly 2)) isCreature), instruct (ownerOf them) (loseLife 1)])
      = [] := by
  decide

/-- "Choose target creature." -/
theorem okTargetCreature : Action.check (DRS.ofList []) (choose (target isCreature)) = [] := by decide

/-- "Choose target color." -/
theorem badTargetColor :
    Action.check (DRS.ofList []) (choose (target (quality .color)))
      = [.targetable (.quality .color)] := by
  decide

/-- "Choose two target creatures." -/
theorem okTwoGroup :
    Action.check (DRS.ofList []) (choose (.described (.target (exactly 2)) isCreature)) = [] := by decide

/-- "Choose zero target creatures." -/
theorem badZeroGroup :
    Action.check (DRS.ofList []) (choose (.described (.target (exactly 0)) isCreature)) = [.nonZeroQ] := by
  decide

/-- "Choose up to one — Destroy target artifact; or destroy target enchantment." -/
theorem okModalTwoModes :
    Action.check (DRS.ofList [])
      (.chooseModes (upTo 1)
        [(none, none, destroy (target isArtifact)), (none, none, destroy (target isEnchantment))]) = [] := by
  decide

/-- "Choose one — Destroy target artifact." -/
theorem badModalOneMode :
    Action.check (DRS.ofList []) (.chooseModes (upTo 1) [(none, none, destroy (target isArtifact))])
      = [.atLeastTwo] := by
  decide

/-- "Choose two — Destroy target artifact; or destroy target enchantment." -/
theorem okModalTwoOfTwo :
    Action.check (DRS.ofList [])
      (.chooseModes (exactly 2)
        [(none, none, destroy (target isArtifact)), (none, none, destroy (target isEnchantment))]) = [] := by
  decide

/-- "Choose three — Destroy target artifact; or destroy target enchantment." -/
theorem badModalOverreach :
    Action.check (DRS.ofList [])
      (.chooseModes (exactly 3)
        [(none, none, destroy (target isArtifact)), (none, none, destroy (target isEnchantment))])
      = [.modesFit] := by
  decide

/-- "Choose one or more — + {1} — destroy target artifact; + {1} — destroy target enchantment."
[CR#702.172a] -/
theorem okSpreeBothCosted :
    Action.check (DRS.ofList [])
      (chooseSpree
        [ (some (.mana [generic 1]), destroy (target isArtifact)),
          (some (.mana [generic 1]), destroy (target isEnchantment)) ]) = [] := by
  decide

/-- "Choose one — Destroy target artifact; or tap it." The Idris pin refutes the zone; the
checker refuses the unresolved `it`. -/
theorem badModalReadsAcrossModes :
    Action.check (DRS.ofList [])
      (chooseModes (exactly 1) [destroy (target isArtifact), .setStatus .tapped it])
      = [.anaphor .bare .one 0] := by
  decide

theorem badReadsAfterModal :
    Action.check (DRS.ofList [])
      (.sequentially
        [ chooseModes (exactly 1) [destroy (target isArtifact), destroy (target isEnchantment)],
          .setStatus .tapped it ]) = [.anaphor .bare .one 0] := by
  decide

/-- "you pay 1 life" -/
theorem okMatchedPayer :
    Action.check (DRS.ofList [])
      (may (.pay (payLife 1) .once .paid none none) none (some (draw 1))) = [] := by
  decide

/-- "Choose new targets for target instant or sorcery spell." -/
theorem okRetargetStackSpell :
    Action.check (DRS.ofList []) (.chooseNewTargets (target (.and [isInstantOrSorcery, isSpell]))) = [] := by
  decide

/-- "Your opponents can't gain life." -/
theorem okStaticPlayerCant :
    Ability.check (DRS.ofList []) (.static (cant (actor (.described .all (.isOpponentOf .you))) (gainLife anyAmount)))
      = [] := by
  decide

/-- "the basic land type of your choice" -/
def chosenBasicLandType : QualityPayload :=
  .quality (.chosen (.ofYourChoice (.subtype .land) (some .basicTypesOnly)))

/-- "Target land becomes the basic land type of your choice until end of turn." -/
theorem okChosenBasicTypeOnLand :
    Action.check (DRS.ofList [])
      (.establish (qualityChange (target isLand) .sets chosenBasicLandType) (some untilEndOfTurn))
      = [] := by
  decide

/-- "Lands you control are Mountains." -/
theorem okLandsAreMountains :
    StaticSpec.check (DRS.ofList [])
      (qualityChange (allOf (.and [isLand, .hasPossessor .controller .you])) .sets
        (.bundle { characteristics := { subtypes := [mountain] } } none)) = [] := by
  decide

/-- "Choose a creature type other than Wall." -/
theorem okCreatureTypeExclusion :
    sortedDomainCheck (DRS.ofList []) (.quality (.subtype .creature)) (some (.typeOtherThan (wall)))
      = [] := by
  decide

/-- "Target land becomes every basic land type until end of turn." -/
theorem setsEveryBasicLandType :
    Action.check (DRS.ofList [])
      (.establish (qualityChange (target isLand) .sets (.quality (.everyTypeOf .basicLand)))
        (some untilEndOfTurn)) = [] := by
  decide

/-- "Target creature loses the creature type of your choice until end of turn." -/
theorem losesChosenCreatureType :
    Action.check (DRS.ofList [])
      (.establish
        (qualityChange (target isCreature) .loses
          (.quality (.chosen (.ofYourChoice (.subtype .creature) none)))) (some untilEndOfTurn))
      = [] := by
  decide

/-- "Target creature loses all colors until end of turn." -/
theorem losesAllColors :
    Action.check (DRS.ofList [])
      (.establish (qualityChange (target isCreature) .loses (.colored .every)) (some untilEndOfTurn))
      = [] := by
  decide

/-- "Target creature is white." -/
theorem okAddsAColor :
    StaticSpec.check (DRS.ofList []) (qualityChange (target isCreature) .adds (.colored (.some [.white]))) = [] :=
        by
  decide

theorem badAddsNoColor :
    StaticSpec.check (DRS.ofList []) (qualityChange (target isCreature) .adds (.colored (.some [])))
      = [.nonEmpty] := by
  decide

/-- "Target creature loses colorless until end of turn." -/
theorem badLosesNoColor :
    StaticSpec.check (DRS.ofList []) (qualityChange (target isCreature) .loses (.colored (.some [])))
      = [.nonEmpty] := by
  decide

def afterChoiceMade : DRS := actionResult (DRS.ofList []) (choose (counted (upTo 1) isCreature))

/-- "Choose up to one creature. Destroy the rest." -/
theorem okChoiceRestStands :
    NounPhrase.checkAt (some .object) afterChoiceMade (theRest .object) = [] := by decide

def afterChoiceRestDisposed : DRS :=
  actionResult (DRS.ofList [])
    (.sequentially [choose (counted (upTo 1) isCreature), destroy (theRest .object)])

/-- "Choose up to one creature. Destroy the rest. Destroy the rest." -/
theorem badChoiceRestDisposedTwice :
    NounPhrase.checkAt (some .object) afterChoiceRestDisposed (theRest .object)
      = [.theRestFits .object] := by
  decide

theorem restConsumptionPreservesChoiceHistoryAndOtherSources :
    let chosen : SelectionEvidence :=
      { kind := .object, chosen := true, scope := 0 }
    let otherSource : SelectionEvidence :=
      { kind := .object, sourceGroup := some 1, scope := 0 }
    let otherScope : SelectionEvidence :=
      { kind := .object, sourceGroup := some 0, scope := 1 }
    let otherKind : SelectionEvidence :=
      { kind := .player, sourceGroup := some 0, scope := 0 }
    let before : DRS :=
      { referents := [
          { id := 0, det := .all, plur := .many,
            payload := .object [] none none none none }]
        selectionEvidence :=
          [{ kind := .object, sourceGroup := some 0, scope := 0 },
            chosen, otherSource, otherScope, otherKind] }
    (before.consumeRest before .object).selectionEvidence =
      [chosen, otherSource, otherScope, otherKind] ∧
      (before.consumeRest before .object).choiceWasPossible .object = true := by decide

theorem consumingObjectRestPreservesUnrelatedPlayerChoice :
    let before := (stepNounPhrase 64 (DRS.ofList []) .other false (allOf (.isOpponentOf .you))).1
    let after := actionResult before
      (.sequentially [choose (a (.isOpponentOf .you)), choose (counted (upTo 1) isCreature),
        destroy (theRest .object)])
    NounPhrase.checkAt (some .object) after (theRest .object) = [.theRestFits .object] ∧
      countParts .object after = 0 ∧ countChoice .player after = 1 ∧
      (after.findById? 1).map (·.payload) = some (.player true) ∧
      (after.findById? 0).map (fun ref => (ref.kind, ref.plur)) =
        some (.player, .many) := by decide

/-- "up to one creature they control" -/
def upToOneTheyControl : NounPhrase :=
  counted (upTo 1) (.and [isCreature, .hasPossessor .controller they])

/-- "Each player chooses up to one creature they control, then sacrifices the rest." -/
def distributedRestOfOwnChoice : Action :=
  .sequentially
    [ instruct (each .anyPlayer) (choose upToOneTheyControl),
      instruct (each .anyPlayer) (sacrifice (theRest .object)) ]

/-- "Each player chooses up to one creature they control, then sacrifices the rest." The
party selection repeats a choice for each listed creature type and player [CR#700.8d]. This
fixture closes only the partitives the same distributive chooser published,
so nothing the table shares is spent. -/
theorem okDistributedRestOfOwnChoice : Action.check (DRS.ofList []) distributedRestOfOwnChoice = [] := by
  decide

def afterDistributedChoice : DRS :=
  actionResult (DRS.ofList []) (instruct (each .anyPlayer) (choose upToOneTheyControl))

/-- "Each player chooses up to one creature they control, then sacrifices the rest." -/
theorem okDistributedRestStands :
    NounPhrase.checkAt (some .object) afterDistributedChoice (theRest .object) = [] := by decide

def afterDistributedRestSacrificed : DRS := actionResult (DRS.ofList []) distributedRestOfOwnChoice

/-- "Each player chooses up to one creature they control, then sacrifices the rest, then
sacrifices the rest.": the per-agent rest closes the partitives it spent, so no rest stands to
spend again. -/
theorem badDistributedRestDisposedTwice :
    NounPhrase.checkAt (some .object) afterDistributedRestSacrificed (theRest .object)
      = [.theRestFits .object] := by
  decide

/-- "Tap all creatures. Each player chooses up to one creature they control." -/
theorem okSharedGroupWithoutARest :
    Action.check (DRS.ofList [])
      (.sequentially [tap (allOf isCreature), instruct (each .anyPlayer) (choose upToOneTheyControl)])
      = [] := by
  decide

theorem targetLoopPublishesAFreshPluralResult :
    let loop := Action.doForEach (each .anyPlayer)
      (choose (target (.and [isPermanent, .hasPossessor .controller they])))
    let after := actionResult (DRS.ofList []) loop
    Action.check (DRS.ofList [])
      (.sequentially [loop, instruct (those .player) (sacrifice (those .permanent))]) = [] ∧
      after.visibleReferents.map (fun r => (r.id, r.kind, r.plur)) =
        [(3, .object, .many), (0, .player, .many)] ∧
      (after.findById? 2).isNone = true ∧
      (firstReach (.word .permanent) .many (nextClause after)).map (·.id) = some 3 ∧
      (firstReach (.word .permanent) .one (nextClause after)).isNone = true := by decide

theorem ordinaryLoopChoicesRemainLocal :
    let loop := Action.doForEach (each .anyPlayer)
      (choose (a (.and [isPermanent, .hasPossessor .controller they])))
    let after := actionResult (DRS.ofList []) loop
    Action.check (DRS.ofList [])
      (.sequentially [loop, instruct (those .player) (sacrifice (those .permanent))]) =
        [.anaphor (.word .permanent) .many 0] ∧
      after.visibleReferents.map (fun r => (r.id, r.kind, r.plur)) =
        [(0, .player, .many)] := by decide

theorem distributedRestPreservesTheUnrelatedOuterGroup :
    let tapping := tap (allOf isCreature)
    let sequence := Action.sequentially
      [tapping, instruct (each .anyPlayer) (choose upToOneTheyControl),
        instruct (each .anyPlayer) (sacrifice (theRest .object))]
    let after := actionResult (DRS.ofList []) sequence
    Action.check (DRS.ofList []) sequence = [] ∧
      (after.findById? 7).map (fun ref => (ref.kind, ref.plur, ref.zone, ref.ty)) =
        some (.object, .many, some .graveyard, [.creature]) ∧
      (after.findById? 6).isNone = true ∧
      (after.findById? 0).map (·.payload) =
        ((actionResult (DRS.ofList []) tapping).findById? 0).map (·.payload) := by decide

theorem distributedSacrificeConsumesTheOuterRemainder :
    let sequence := Action.sequentially
      [choose (counted (upTo 1) isCreature),
        instruct (each .anyPlayer) (sacrifice (theRest .object))]
    let after := actionResult (DRS.ofList []) sequence
    Action.check (DRS.ofList []) sequence = [] ∧
      (after.findById? 4).map (fun ref => (ref.kind, ref.plur, ref.zone, ref.ty)) =
        some (.object, .many, some .graveyard, [.creature]) ∧
      (after.findById? 3).isNone = true ∧
      countParts .object after = 0 := by decide

/-- "an opponent who controls more lands than you control" -/
theorem okComparisonBoundOutsideTheMember :
    Predicate.checkAt .player (DRS.ofList [])
      (.compareOver (.isOpponentOf .you) (countOf (.and [isLand, .hasPossessor .controller they])) .greater
        (countOf (.and [isLand, .hasPossessor .controller .you]))) = [] := by
  decide

/-- "an opponent who controls more lands than they control" -/
theorem badMemberInComparisonBound :
    Predicate.checkAt .player (DRS.ofList [])
      (.compareOver (.isOpponentOf .you) (countOf (.and [isLand, .hasPossessor .controller .you])) .greater
        (countOf (.and [isLand, .hasPossessor .controller they])))
      = [.anaphor (.word .player) .one 0] := by
  decide

/-- "Target creature gets +3/+3 until end of turn." -/
theorem okGetsBattlefield :
    Action.check (DRS.ofList [])
      (get (target isCreature) (.up 3) (.up 3) (some untilEndOfTurn)) = [] := by
  decide

/-- "Choose a creature. This deals 3 damage to each creature not chosen this way." -/
theorem okNotChosenAfterOneChoice :
    Action.check (DRS.ofList [])
      (.sequentially
        [ choose (a isCreature),
          (Action.instruct .this (.dealDamage 3 (each (.and [isCreature, .notChosen])))) ]) = [] := by
  decide

/-- "This deals 3 damage to each creature not chosen this way." Nothing was chosen, so "this
way" reads back no choice (`okNotChosenAfterOneChoice` is the same read with one standing). -/
theorem badNotChosenWithoutAChoice :
    Action.check (DRS.ofList []) (Action.instruct .this (.dealDamage 3 (each (.and [isCreature, .notChosen]))))
      = [.choiceInScope .object] := by
  decide

/-- "Choose a creature. Choose a creature. This deals 3 damage to each creature not chosen this
way.": "this way" names the manner, so both standing choices are excluded together
[CR#700.8d]. -/
theorem okNotChosenAfterTwoChoices :
    Action.check (DRS.ofList [])
      (.sequentially
        [ choose (a isCreature),
          choose (a isCreature),
          (Action.instruct .this (.dealDamage 3 (each (.and [isCreature, .notChosen])))) ]) = [] := by
  decide

/-- "Choose a creature you control, then each opponent chooses a creature they control.
Destroy each creature not chosen this way.": Sculpted Sunburst's exclusion, whose two standing
choices have different choosers [CR#101.4]. -/
theorem okNotChosenAcrossChoosers :
    Action.check (DRS.ofList [])
      (.sequentially
        [ choose (a (and [isCreature, youControl])),
          instruct (each (.isOpponentOf .you)) (choose (a (.and [isCreature, .hasPossessor .controller they]))),
          destroy (each (.and [isCreature, .notChosen])) ]) = [] := by
  decide

/-- "Each player chooses a creature they control." [CR#700.8d] -/
theorem okAgentScopedChoice :
    Action.check (DRS.ofList [])
      (instruct (each .anyPlayer) (choose (a (.and [isCreature, .hasPossessor .controller they])))) = [] := by
  decide

/-- "Choose a creature they control.": the chooserless spelling has no antecedent for "they"
(`okAgentScopedChoice` is the same noun under a chooser, which publishes one). -/
theorem badUnchooseredTheyControl :
    Action.check (DRS.ofList []) (choose (a (.and [isCreature, .hasPossessor .controller they])))
      = [.anaphor (.word .player) .one 0] := by
  decide

/-- "Starting with you, each player chooses a creature they control." [CR#101.4] -/
theorem okChoiceStartingWithYou :
    Action.check (DRS.ofList [])
      (instruct (each .anyPlayer) (.choose (some .you)
        (a (.and [isCreature, .hasPossessor .controller they])) .openly none)) = [] := by
  decide

/-- "Starting with you, target player chooses a creature.": one player makes the choice, so
there is no order for "starting with" to fix [CR#101.4] (`okChoiceStartingWithYou` is the
same order over a distributive chooser). -/
theorem badOrderedSingularChooser :
    Action.check (DRS.ofList [])
      (instruct (target .anyPlayer) (.choose (some .you) (a isCreature) .openly none))
      = [.choiceOrder] := by
  decide

/-- "Choose a creature. If you chose a creature this way, draw a card." -/
theorem okChoseThisWayAfterChoice :
    Action.check (DRS.ofList [])
      (.sequentially
        [choose (a isCreature), doIf (.choseThisWay .you isCreature) (draw 1)])
      = [] := by
  decide

/-- "If you chose a creature this way, draw a card.": nothing was chosen, so "this way" reads
back no choice (`okChoseThisWayAfterChoice` is the same read with one standing). -/
theorem badChoseThisWayWithoutAChoice :
    Action.check (DRS.ofList []) (doIf (.choseThisWay .you isCreature) (draw 1))
      = [.choiceInScope .object] := by
  decide

/-- "Each player chooses a creature. Exile them." -/
theorem okDistributedChoiceReadsAsGroup :
    Action.check (DRS.ofList []) (.sequentially [instruct (each .anyPlayer) (choose (a isCreature)), exile
        them])
      = [] := by
  decide

/-- "Each player chooses a creature. Exile it.": a distributive choice stands as one per
chooser, so the singular read has no antecedent (`okDistributedChoiceReadsAsGroup` is the
plural read). -/
theorem badDistributedChoiceReadSingular :
    Action.check (DRS.ofList []) (.sequentially [instruct (each .anyPlayer) (choose (a isCreature)), exile
        it])
      = [.anaphor .bare .one 0] := by
  decide

/-- "Look at the top card of your library. You may put that card into your graveyard." No
printed card on the bench. -/
theorem lookAtTopThenBin :
    Action.check (DRS.ofList [])
      (.sequentially [lookAt (topSlice 1), may (putInto (that .card) graveyard)])
      = [] := by
  decide

/-- "Until end of turn, you may play lands and cast spells from your graveyard." No printed
card on the bench. -/
theorem playAndCastFromGraveyardThisTurn :
    Action.check (DRS.ofList [])
      (.establish
        (.conjunction none
          [ can you (play (allOf isLand) (from_ := some (graveyardOf .you))),
            can you (cast (allOf isSpell) (from_ := some (graveyardOf .you))) ])
        (some untilEndOfTurn)) = [] := by
  decide

/-- "Each player may play an additional land on each of their turns." No printed card on the
bench. -/
theorem eachPlayerPlaysAdditionalLand :
    StaticSpec.check (DRS.ofList []) (can (each .anyPlayer) (play (counted (additional (exactly 1)) isLand))) = [] := by decide

/-- "Mill three cards. You may put an artifact card from among the cards milled this way into
your hand." No printed card on the bench. -/
theorem millThenPutFromAmongMilled :
    Action.check (DRS.ofList [])
      (.sequentially
        [ mill 3,
          may
            (putInto
              (fromAmong (exactly 1) isArtifact
                (thePlural (and [isCard,
                  .did ⟨.any, .ownScope (some (.action "mill"))
                    (.move (.inCaller .which) yourLibrary (some graveyard) .departure none []),
                    some .thisWay⟩])))
              hand) ]) = [] := by
  decide

/-- "Each player may shuffle their hand and graveyard into their library." No printed card on
the bench. -/
theorem eachPlayerMayShuffleTheirHandAndGraveyard :
    Action.check (DRS.ofList [])
      (instruct (each .anyPlayer) (may
        (instruct they (shuffleInto
          (both (allOf (.and [.isCard, .in_ (handOf they)])) (allOf (.and [.isCard, .in_ (graveyardOf they)])))))))
      = [] := by
  decide

/-- "Each player may discard their hand and draw seven cards." No printed card on the bench. -/
theorem eachPlayerMayDiscardTheirHandAndDrawSeven :
    Action.check (DRS.ofList [])
      (instruct (each .anyPlayer) (may
        (.sequentially [discard (allOf (.and [.isCard, .in_ (handOf they)])), instruct they (draw 7)])))
      = [] := by
  decide

/-- "Each player may …": the offer subject and the two counts its context publishes. No
printed card on the bench. -/
def eachPlayerOffered : NounPhrase := each .anyPlayer

theorem eachPlayerOfferBindsOneMember :
    Action.check (DRS.ofList [])
      (instruct eachPlayerOffered (may (instruct they (draw 1)))) = [] := by decide

theorem eachPlayerOfferDropsTheGroup :
    Action.check (DRS.ofList [])
      (instruct eachPlayerOffered (may
        (instruct (those .player) (draw 1)))) = [.anaphor (.word .player) .many 0] := by decide

/-- "target opponent who has more life than you do" -/
def opponentWithMoreLife : NounPhrase :=
  target (.and [(.isOpponentOf .you), .compare [.playerStat .lifeTotal] .greater (lifeTotalOf .you)])

/-- "Choose target opponent who has more life than you do as you activate this ability.": the
rider times the announcement, not the comparison. -/
theorem okChoiceWithActivationRider :
    Action.check (DRS.ofList [])
      (chooseWhile opponentWithMoreLife (.whileDoing (.does (.actor .you) (.procedure .activate thisAbility .itsOwnCost none none none)))) = [] := by
  decide

/-- "Each player secretly chooses a number. Then those numbers are revealed. Each player who
chose the highest number loses that much life." (Menacing Ogre): not a vote [CR#701.38c]; the
gate is the plural choice. -/
theorem okChoseExtremeAfterNumbers :
    Action.check (DRS.ofList [])
      (.sequentially
        [ instruct (each .anyPlayer) (choose (disclosure := .secretly) (a (quality .number))),
          .revealChoices .numbers,
          instruct (each (.and [.anyPlayer, .choseExtreme .max])) (loseLife .thatMuch) ]) = [] := by
  decide

/-- The same read with no number chosen anywhere in the text. -/
theorem badChoseExtremeWithoutChoice :
    Action.check (DRS.ofList []) (instruct (each (.and [.anyPlayer, .choseExtreme .max])) (loseLife 1))
      = [.numberChoiceInScope] := by
  decide

/-! ## A choice domain has the sort its consumer announces -/

theorem badColorNounWithPlayerDomain :
    Predicate.checkAt (.quality .color) (DRS.ofList []) (.qualityNoun .color (some (.players (.isOpponentOf .you))))
      = [.kindAxisSort] := by decide

theorem okColorNounWithColorDomain :
    Predicate.checkAt (.quality .color) (DRS.ofList []) (.qualityNoun .color (some (.colorOtherThan .red)))
      = [] := by decide

theorem badColorRefinementWithPlayerDomain :
    Predicate.checkAt .object (DRS.ofList []) (.ofYourChoice .color (some (.players (.isOpponentOf .you))))
      = [.kindAxisSort] := by decide

theorem okColorRefinementWithColorDomain :
    Predicate.checkAt .object (DRS.ofList []) (.ofYourChoice .color (some (.colorOtherThan .red)))
      = [] := by
  decide

theorem badCompleteColorChoiceWithPlayerDomain :
    Action.check (DRS.ofList []) (.choose none
      (.described (.a .unmarked) (.qualityNoun .color (some (.players (.isOpponentOf .you))))) .openly none)
      = [.kindAxisSort] := by decide

theorem okCompleteColorChoiceWithColorDomain :
    Action.check (DRS.ofList []) (.choose none
      (.described (.a .unmarked) (.qualityNoun .color (some (.colorOtherThan .red)))) .openly none)
      = [] := by decide

theorem badEntersChoiceDomainSort :
    StaticSpec.check (DRS.ofList [])
      (entryChoice thisCreature (a (qualityFrom .color (.players (.isOpponentOf .you)))))
      = [.kindAxisSort] := by decide

theorem okEntersChoiceDomainSort :
    StaticSpec.check (DRS.ofList [])
      (entryChoice thisCreature (a (qualityFrom .color (.colorOtherThan .red))))
      = [] := by decide

theorem badAttachmentChoiceDomainSort :
    StaticSpec.check (DRS.ofList [])
      (attachmentChoice thisCreature (a (qualityFrom .color (.players (.isOpponentOf .you)))))
      = [.kindAxisSort] := by decide

theorem okAttachmentChoiceDomainSort :
    StaticSpec.check (DRS.ofList [])
      (attachmentChoice thisCreature (a (qualityFrom .color (.colorOtherThan .red))))
      = [] := by decide

theorem badLandSubtypeDomainForCreatureType :
    Predicate.checkAt (.quality (.subtype .creature)) (DRS.ofList [])
      (.qualityNoun (.subtype .creature) (some .basicTypesOnly)) = [.kindAxisSort] := by decide

theorem okLandSubtypeDomainForLandType :
    Predicate.checkAt (.quality (.subtype .land)) (DRS.ofList [])
      (.qualityNoun (.subtype .land) (some .basicTypesOnly)) = [] := by decide

theorem badDomainContentsAndSort :
    Predicate.checkAt (.quality .color) (DRS.ofList []) (.qualityNoun .color (some (.abilitiesAmong [])))
      = [.nonEmpty, .kindAxisSort] := by decide

theorem okAbilityDomainContentsAndSort :
    Predicate.checkAt (.quality .ability) (DRS.ofList [])
      (.qualityNoun .ability (some (.abilitiesAmong [.the (.named "flying")]))) = [] := by decide

/-! ## Mode Changes (`semantics-v2-modal-choice`) -/

/-- Entwine {1}: "you may choose all modes instead … pay an additional {1}" [CR#702.42a]. -/
theorem okModeChoiceEntwine :
    StaticSpec.check (DRS.ofList []) (.modeChoice (.allInstead (some (.mana [generic 1])) none)) = [] := by
  decide

/-- "If you control an artifact, you may choose both instead." -/
theorem okModeChoiceConditionalBoth :
    StaticSpec.check (DRS.ofList [])
      (.modeChoice (.allInstead none
        (some (.exists_ (.described .bare (.and [isArtifact, .hasPossessor .controller .you]))))))
      = [] := by
  decide

/-- Escalate {2} [CR#702.120a]. -/
theorem okModeChoiceEscalate :
    StaticSpec.check (DRS.ofList []) (.modeChoice (.extraModesCost (.mana [generic 2]))) = [] := by decide

/-- Escalate's payments are counted by the existing paid facet: "the number of times its
escalate cost was paid" [CR#702.120a]. -/
theorem okEscalateTimesPaid :
    Amount.check (DRS.ofList []) (timesPaid (.byKeyword (.named "escalate")) thisSpell) = [] := by decide

/-- Spree: one or more [CR#702.172a]. -/
theorem okModeChoiceSpree :
    StaticSpec.check (DRS.ofList []) (.modeChoice (.setQuantity (orMore 1))) = [] := by decide

/-- Tiered: exactly one [CR#702.183a]. -/
theorem okModeChoiceTiered :
    StaticSpec.check (DRS.ofList []) (.modeChoice (.setQuantity (exactly 1))) = [] := by decide

/-- A mode count of zero is no choice. -/
theorem badModeChoiceZero :
    StaticSpec.check (DRS.ofList []) (.modeChoice (.setQuantity (upTo 0))) = [.nonZeroQ] := by decide

/-- "You may choose the same mode more than once" [CR#700.2d]. -/
theorem okModeChoiceRepeatable : StaticSpec.check (DRS.ofList []) (.modeChoice .repeatable) = [] := by decide

/-- "Choose one that hasn't been chosen this turn". -/
theorem okModeChoiceNotChosenThisTurn :
    StaticSpec.check (DRS.ofList []) (.modeChoice (.notChosenBefore soFarThisTurn)) = [] := by decide

/-- "Choose up to five {P} worth of modes. {P} — Draw a card. {P}{P} — Destroy target
artifact." [CR#700.2i] -/
theorem okModalWorth :
    Action.check (DRS.ofList [])
      (.chooseModes (.worth 5)
        [(none, some 1, draw 1), (none, some 2, destroy (target isArtifact))]) = [] := by
  decide

/-- "Choose up to zero {P} worth of modes" chooses nothing. -/
theorem badModalWorthZero :
    Action.check (DRS.ofList [])
      (.chooseModes (.worth 0)
        [(none, some 1, draw 1), (none, some 2, destroy (target isArtifact))])
      = [.nonZeroQ] := by
  decide

end Semantics.Proofs.Choice
