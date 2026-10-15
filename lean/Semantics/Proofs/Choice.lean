import Semantics.Macros
import Semantics.Check.Card

/-!
# Semantics.Proofs.Choice

Port of `idris/src/Experimental/Proofs/Choice.idr`: the pins of the Choice family, in theorem
form, each closed by `decide`. The names and the sentences are the Idris ones.
-/

open Semantics Semantics.Macros

namespace Semantics.Proofs.Choice

private def instructionResult (drs : DRS) (instruction : Instruction) : DRS :=
  (stepInstruction 64 .postEvent .obj drs instruction).1

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
    Instruction.check (DRS.ofList []) (.sequentially [choose (target creature), gainLife (powerOf it)])
      = [] := by
  decide

/-- "Choose two target creatures. You gain life equal to their power." -/
theorem badGroupPower :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [choose (.described (.target (exactly 2)) creature), gainLife (powerOf them)])
      = [.singular] := by
  decide

/-- "Choose target creature. Its owner loses 1 life." -/
theorem okSingleOwner :
    Instruction.check (DRS.ofList [])
      (.sequentially [choose (target creature), instruct (ownerOf it) (loseLife (.lit 1))]) = [] := by
  decide

/-- "Choose two target creatures. Their owners each lose 1 life." -/
theorem okGroupOwners :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [choose (.described (.target (exactly 2)) creature), instruct (ownerOf them) (loseLife (.lit 1))])
      = [] := by
  decide

/-- "Choose target creature." -/
theorem okTargetCreature : Instruction.check (DRS.ofList []) (choose (target creature)) = [] := by decide

/-- "Choose target color." -/
theorem badTargetColor :
    Instruction.check (DRS.ofList []) (choose (target (quality .color)))
      = [.targetable (.quality .color)] := by
  decide

/-- "Choose two target creatures." -/
theorem okTwoGroup :
    Instruction.check (DRS.ofList []) (choose (.described (.target (exactly 2)) creature)) = [] := by decide

/-- "Choose zero target creatures." -/
theorem badZeroGroup :
    Instruction.check (DRS.ofList []) (choose (.described (.target (exactly 0)) creature)) = [.nonZeroQ] := by
  decide

/-- "Choose up to one — Destroy target artifact; or destroy target enchantment." -/
theorem okModalTwoModes :
    Instruction.check (DRS.ofList [])
      (.chooseModes (upTo 1)
        [(none, none, destroy (target artifact)), (none, none, destroy (target enchantment))]) = [] := by
  decide

/-- "Choose one — Destroy target artifact." -/
theorem badModalOneMode :
    Instruction.check (DRS.ofList []) (.chooseModes (upTo 1) [(none, none, destroy (target artifact))])
      = [.atLeastTwo] := by
  decide

/-- "Choose two — Destroy target artifact; or destroy target enchantment." -/
theorem okModalTwoOfTwo :
    Instruction.check (DRS.ofList [])
      (.chooseModes (exactly 2)
        [(none, none, destroy (target artifact)), (none, none, destroy (target enchantment))]) = [] := by
  decide

/-- "Choose three — Destroy target artifact; or destroy target enchantment." -/
theorem badModalOverreach :
    Instruction.check (DRS.ofList [])
      (.chooseModes (exactly 3)
        [(none, none, destroy (target artifact)), (none, none, destroy (target enchantment))])
      = [.modesFit] := by
  decide

/-- "Choose one or more — + {1} — destroy target artifact; + {1} — destroy target enchantment."
[CR#702.172a] -/
theorem okSpreeBothCosted :
    Instruction.check (DRS.ofList [])
      (chooseSpree
        [ (some (.mana [generic 1]), destroy (target artifact)),
          (some (.mana [generic 1]), destroy (target enchantment)) ]) = [] := by
  decide

/-- "Choose one — Destroy target artifact; or tap it." The Idris pin refutes the zone; the
checker refuses the unresolved `it`. -/
theorem badModalReadsAcrossModes :
    Instruction.check (DRS.ofList [])
      (chooseModes (exactly 1) [destroy (target artifact), .setStatus .tapped it])
      = [.anaphor .bare .one 0] := by
  decide

theorem badReadsAfterModal :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [ chooseModes (exactly 1) [destroy (target artifact), destroy (target enchantment)],
          .setStatus .tapped it ]) = [.anaphor .bare .one 0] := by
  decide

/-- "you pay 1 life" -/
theorem okMatchedPayer :
    Instruction.check (DRS.ofList [])
      (Primitives.Instruction.offer (.pay (payLife 1) .once .paid none none) none (some (draw (.lit 1)))) = [] := by
  decide

/-- "Choose new targets for target instant or sorcery spell." -/
theorem okRetargetStackSpell :
    Instruction.check (DRS.ofList []) (.chooseNewTargets (target (.and [instantOrSorcery, spell]))) = [] := by
  decide

/-- "Your opponents can't gain life." -/
theorem okStaticPlayerCant :
    Ability.check (DRS.ofList []) (.static (playerCant (.core .gainLife) (.playerGroup .yourOpponents)))
      = [] := by
  decide

/-- "the basic land type of your choice" -/
def chosenBasicLandType : QualityPayload :=
  .chosenQuality (.ofYourChoice (.subtype .land) (some .basicTypesOnly))

/-- "Target land becomes the basic land type of your choice until end of turn." -/
theorem okChosenBasicTypeOnLand :
    Instruction.check (DRS.ofList [])
      (.establish (Primitives.StaticSpec.qualityChange (target land) .sets chosenBasicLandType) (some untilEndOfTurn))
      = [] := by
  decide

/-- "Lands you control are Mountains." -/
theorem okLandsAreMountains :
    StaticSpec.check (DRS.ofList [])
      (Primitives.StaticSpec.qualityChange (allOf (.and [land, .hasPossessor .controller .you])) .sets
        (.bundle { characteristics := { subtypes := [landType "Mountain"] } } none)) = [] := by
  decide

/-- "Choose a creature type other than Wall." -/
theorem okCreatureTypeExclusion :
    sortedDomainCheck (DRS.ofList []) (.quality (.subtype .creature)) (some (.typeOtherThan (creatureType "Wall")))
      = [] := by
  decide

/-- "Choose a creature type other than Equipment." -/
theorem badNonCreatureTypeExclusion :
    sortedDomainCheck (DRS.ofList []) (.quality (.subtype .creature)) (some (.typeOtherThan (artifactType "Equipment")))
      = [.subtypeType] := by
  decide

/-- "Target land becomes every basic land type until end of turn." -/
theorem setsEveryBasicLandType :
    Instruction.check (DRS.ofList [])
      (.establish (Primitives.StaticSpec.qualityChange (target land) .sets (.everyTypeOf .basicLand))
        (some untilEndOfTurn)) = [] := by
  decide

/-- "Target creature loses the creature type of your choice until end of turn." -/
theorem losesChosenCreatureType :
    Instruction.check (DRS.ofList [])
      (.establish
        (Primitives.StaticSpec.qualityChange (target creature) .loses
          (.chosenQuality (.ofYourChoice (.subtype .creature) none))) (some untilEndOfTurn))
      = [] := by
  decide

/-- "Target creature loses all colors until end of turn." -/
theorem losesAllColors :
    Instruction.check (DRS.ofList [])
      (.establish (Primitives.StaticSpec.qualityChange (target creature) .loses (.colored .every)) (some untilEndOfTurn))
      = [] := by
  decide

/-- "Target creature is white." -/
theorem okAddsAColor :
    StaticSpec.check (DRS.ofList []) (Primitives.StaticSpec.qualityChange (target creature) .adds (.colored (.some [.white]))) = [] :=
        by
  decide

theorem badAddsNoColor :
    StaticSpec.check (DRS.ofList []) (Primitives.StaticSpec.qualityChange (target creature) .adds (.colored (.some [])))
      = [.nonEmpty] := by
  decide

/-- "Target creature loses colorless until end of turn." -/
theorem badLosesNoColor :
    StaticSpec.check (DRS.ofList []) (Primitives.StaticSpec.qualityChange (target creature) .loses (.colored (.some [])))
      = [.nonEmpty] := by
  decide

def afterChoiceMade : DRS := instructionResult (DRS.ofList []) (choose (counted (upTo 1) creature))

/-- "Choose up to one creature. Destroy the rest." -/
theorem okChoiceRestStands :
    NounPhrase.checkAt (some .object) afterChoiceMade (theRest .object) = [] := by decide

def afterChoiceRestDisposed : DRS :=
  instructionResult (DRS.ofList [])
    (.sequentially [choose (counted (upTo 1) creature), destroy (theRest .object)])

/-- "Choose up to one creature. Destroy the rest. Destroy the rest." -/
theorem badChoiceRestDisposedTwice :
    NounPhrase.checkAt (some .object) afterChoiceRestDisposed (theRest .object)
      = [.theRestFits .object] := by
  decide

/-- "up to one creature they control" -/
def upToOneTheyControl : NounPhrase :=
  counted (upTo 1) (.and [creature, .hasPossessor .controller they])

/-- "Each player chooses up to one creature they control, then sacrifices the rest." -/
def distributedRestOfOwnChoice : Instruction :=
  .sequentially
    [ instruct (each .anyPlayer) (choose upToOneTheyControl),
      instruct (each .anyPlayer) (sacrifice (theRest .object)) ]

/-- "Each player chooses up to one creature they control, then sacrifices the rest." The
party selection repeats a choice for each listed creature type and player [CR#700.8d]. This
fixture closes only the partitives the same distributive chooser published,
so nothing the table shares is spent. -/
theorem okDistributedRestOfOwnChoice : Instruction.check (DRS.ofList []) distributedRestOfOwnChoice = [] := by
  decide

def afterDistributedChoice : DRS :=
  instructionResult (DRS.ofList []) (instruct (each .anyPlayer) (choose upToOneTheyControl))

/-- "Each player chooses up to one creature they control, then sacrifices the rest." -/
theorem okDistributedRestStands :
    NounPhrase.checkAt (some .object) afterDistributedChoice (theRest .object) = [] := by decide

def afterDistributedRestSacrificed : DRS := instructionResult (DRS.ofList []) distributedRestOfOwnChoice

/-- "Each player chooses up to one creature they control, then sacrifices the rest, then
sacrifices the rest.": the per-agent rest closes the partitives it spent, so no rest stands to
spend again. -/
theorem badDistributedRestDisposedTwice :
    NounPhrase.checkAt (some .object) afterDistributedRestSacrificed (theRest .object)
      = [.theRestFits .object] := by
  decide

/-- "Tap all creatures. Each player chooses up to one creature they control." -/
theorem okSharedGroupWithoutARest :
    Instruction.check (DRS.ofList [])
      (.sequentially [tap (allOf creature), instruct (each .anyPlayer) (choose upToOneTheyControl)])
      = [] := by
  decide

/-- "For each player, choose target permanent that player controls. Those players sacrifice
those permanents.": the plural deed re-places only the plural bindings the same loop published
[CR#701.21a]. -/
theorem okDistributedLoopParts :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [ .doForEach (each .anyPlayer)
            (choose (target (.and [permanent, .hasPossessor .controller they]))),
          instruct (those .player) (sacrifice (those .permanent)) ]) = [] := by
  decide

/-- "Tap all creatures. Each player chooses up to one creature they control, then sacrifices
the rest.": the rest here spends the group the whole table shares, and a player can't
sacrifice a permanent they don't control ([CR#701.21a]; `okDistributedRestOfOwnChoice` is the
same deed with only the chooser's own partitives standing). -/
theorem badDistributedRestOfSharedGroup :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [ tap (allOf creature),
          instruct (each .anyPlayer) (choose upToOneTheyControl),
          instruct (each .anyPlayer) (sacrifice (theRest .object)) ]) = [.enactKeepsOuter] := by
  decide

/-- "Choose up to one creature. Each player sacrifices the rest.": one chooser leaves one
shared leftover, not a partition per player, so the other players would sacrifice permanents
they don't control [CR#701.21a]. -/
theorem badDistributedRestOfSingularChoice :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [choose (counted (upTo 1) creature), instruct (each .anyPlayer) (sacrifice (theRest .object))])
      = [.enactKeepsOuter] := by
  decide

/-- "an opponent who controls more lands than you control" -/
theorem okComparisonBoundOutsideTheMember :
    Predicate.checkAt .player (DRS.ofList [])
      (.compareOver .opponent (countOf (.and [land, .hasPossessor .controller they])) .greater
        (countOf (.and [land, .hasPossessor .controller .you]))) = [] := by
  decide

/-- "an opponent who controls more lands than they control" -/
theorem badMemberInComparisonBound :
    Predicate.checkAt .player (DRS.ofList [])
      (.compareOver .opponent (countOf (.and [land, .hasPossessor .controller .you])) .greater
        (countOf (.and [land, .hasPossessor .controller they])))
      = [.anaphor (.word .player) .one 0] := by
  decide

/-- "Target creature gets +3/+3 until end of turn." -/
theorem okGetsBattlefield :
    Instruction.check (DRS.ofList [])
      (get (target creature) (.up (.lit 3)) (.up (.lit 3)) (some untilEndOfTurn)) = [] := by
  decide

/-- "Choose a creature. This deals 3 damage to each creature not chosen this way." -/
theorem okNotChosenAfterOneChoice :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [ choose (a creature),
          (Instruction.instruct .this (.dealDamage (.lit 3) (each (.and [creature, .notChosen])) .any)) ]) = [] := by
  decide

/-- "This deals 3 damage to each creature not chosen this way." Nothing was chosen, so "this
way" reads back no choice (`okNotChosenAfterOneChoice` is the same read with one standing). -/
theorem badNotChosenWithoutAChoice :
    Instruction.check (DRS.ofList []) (Instruction.instruct .this (.dealDamage (.lit 3) (each (.and [creature, .notChosen])) .any))
      = [.choiceInScope .object] := by
  decide

/-- "Choose a creature. Choose a creature. This deals 3 damage to each creature not chosen this
way.": "this way" names the manner, so both standing choices are excluded together
[CR#700.8d]. -/
theorem okNotChosenAfterTwoChoices :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [ choose (a creature),
          choose (a creature),
          (Instruction.instruct .this (.dealDamage (.lit 3) (each (.and [creature, .notChosen])) .any)) ]) = [] := by
  decide

/-- "Choose a creature you control, then each opponent chooses a creature they control.
Destroy each creature not chosen this way.": Sculpted Sunburst's exclusion, whose two standing
choices have different choosers [CR#101.4]. -/
theorem okNotChosenAcrossChoosers :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [ choose (a creatureYouControl),
          instruct (each .opponent) (choose (a (.and [creature, .hasPossessor .controller they]))),
          destroy (each (.and [creature, .notChosen])) ]) = [] := by
  decide

/-- "Each player chooses a creature they control." [CR#700.8d] -/
theorem okAgentScopedChoice :
    Instruction.check (DRS.ofList [])
      (instruct (each .anyPlayer) (choose (a (.and [creature, .hasPossessor .controller they])))) = [] := by
  decide

/-- "Choose a creature they control.": the chooserless spelling has no antecedent for "they"
(`okAgentScopedChoice` is the same noun under a chooser, which publishes one). -/
theorem badUnchooseredTheyControl :
    Instruction.check (DRS.ofList []) (choose (a (.and [creature, .hasPossessor .controller they])))
      = [.anaphor (.word .player) .one 0] := by
  decide

/-- "Starting with you, each player chooses a creature they control." [CR#101.4] -/
theorem okChoiceStartingWithYou :
    Instruction.check (DRS.ofList [])
      (instruct (each .anyPlayer) (.choose (some .you)
        (a (.and [creature, .hasPossessor .controller they])) .openly none)) = [] := by
  decide

/-- "Starting with you, target player chooses a creature.": one player makes the choice, so
there is no order for "starting with" to fix [CR#101.4] (`okChoiceStartingWithYou` is the
same order over a distributive chooser). -/
theorem badOrderedSingularChooser :
    Instruction.check (DRS.ofList [])
      (instruct (target .anyPlayer) (.choose (some .you) (a creature) .openly none))
      = [.choiceOrder] := by
  decide

/-- "Choose a creature. If you chose a creature this way, draw a card." -/
theorem okChoseThisWayAfterChoice :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [choose (a creature), doIf (.choseThisWay .you creature) (draw (.lit 1))])
      = [] := by
  decide

/-- "If you chose a creature this way, draw a card.": nothing was chosen, so "this way" reads
back no choice (`okChoseThisWayAfterChoice` is the same read with one standing). -/
theorem badChoseThisWayWithoutAChoice :
    Instruction.check (DRS.ofList []) (doIf (.choseThisWay .you creature) (draw (.lit 1)))
      = [.choiceInScope .object] := by
  decide

/-- "Each player chooses a creature. Exile them." -/
theorem okDistributedChoiceReadsAsGroup :
    Instruction.check (DRS.ofList []) (.sequentially [instruct (each .anyPlayer) (choose (a creature)), exile
        them])
      = [] := by
  decide

/-- "Each player chooses a creature. Exile it.": a distributive choice stands as one per
chooser, so the singular read has no antecedent (`okDistributedChoiceReadsAsGroup` is the
plural read). -/
theorem badDistributedChoiceReadSingular :
    Instruction.check (DRS.ofList []) (.sequentially [instruct (each .anyPlayer) (choose (a creature)), exile
        it])
      = [.anaphor .bare .one 0] := by
  decide

/-- "Look at the top card of your library. You may put that card into your graveyard." No
printed card on the bench. -/
theorem lookAtTopThenBin :
    Instruction.check (DRS.ofList [])
      (.sequentially [lookAt (topSlice (.lit 1)), offer (move (that .card) graveyard)])
      = [] := by
  decide

/-- "Until end of turn, you may play lands and cast spells from your graveyard." No printed
card on the bench. -/
theorem playAndCastFromGraveyardThisTurn :
    Instruction.check (DRS.ofList [])
      (.establish
        (.conjunction none
          [ mayPlayDeed (.action "play") .you (allOf land) none
              (.play (some (graveyardOf .you)) none none false .itsOwnCost),
            mayPlayDeed (.action "cast") .you (allOf spell) none
              (.play (some (graveyardOf .you)) none none false .itsOwnCost) ])
        (some untilEndOfTurn)) = [] := by
  decide

/-- "Each player may play an additional land on each of their turns." No printed card on the
bench. -/
theorem eachPlayerPlaysAdditionalLand :
    StaticSpec.check (DRS.ofList []) (mayPlayAdditionalLands (each .anyPlayer) (exactly 1)) = [] := by decide

/-- "Mill three cards. You may put an artifact card from among the cards milled this way into
your hand." No printed card on the bench. -/
theorem millThenPutFromAmongMilled :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [ mill (.lit 3) .you,
          offer
            (move (fromAmong (exactly 1) artifact (theVerbed (.action "mill") .many (some .card) .thisWay))
              hand) ]) = [] := by
  decide

/-- "Each player may shuffle their hand and graveyard into their library." No printed card on
the bench. -/
theorem eachPlayerMayShuffleTheirHandAndGraveyard :
    Instruction.check (DRS.ofList [])
      (instruct (each .anyPlayer) (offer
        (instruct they (shuffleInto
          (Primitives.NounPhrase.both (allOf (.and [.isCard, .inZone (handOf they)])) (allOf (.and [.isCard, .inZone (graveyardOf they)])))))))
      = [] := by
  decide

/-- "Each player may discard their hand and draw seven cards." No printed card on the bench. -/
theorem eachPlayerMayDiscardTheirHandAndDrawSeven :
    Instruction.check (DRS.ofList [])
      (instruct (each .anyPlayer) (offer
        (.sequentially [discard (allOf (.and [.isCard, .inZone (handOf they)])), instruct they (draw (.lit 7))])))
      = [] := by
  decide

/-- "Each player may …": the offer subject and the two counts its context publishes. No
printed card on the bench. -/
def eachPlayerOffered : NounPhrase := each .anyPlayer

theorem eachPlayerOfferBindsOneMember :
    Instruction.check (DRS.ofList [])
      (instruct eachPlayerOffered (Primitives.Instruction.offer (instruct they (draw (.lit 1))) none none)) = [] := by decide

theorem eachPlayerOfferDropsTheGroup :
    Instruction.check (DRS.ofList [])
      (instruct eachPlayerOffered (Primitives.Instruction.offer
        (instruct (those .player) (draw (.lit 1))) none none)) = [.anaphor (.word .player) .many 0] := by decide

/-- "target opponent who has more life than you do" -/
def opponentWithMoreLife : NounPhrase :=
  target (.and [.opponent, .compare [.playerStat .lifeTotal] .greater (lifeTotalOf .you)])

/-- "Choose target opponent who has more life than you do as you activate this ability.": the
rider times the announcement, not the comparison. -/
theorem okChoiceWithActivationRider :
    Instruction.check (DRS.ofList [])
      (chooseWhile opponentWithMoreLife (.whileDoing (.does (.actor .you) (.procedure .activate thisAbility .itsOwnCost none none none)))) = [] := by
  decide

/-- "Each player secretly chooses a number. Then those numbers are revealed. Each player who
chose the highest number loses that much life." (Menacing Ogre): not a vote [CR#701.38c]; the
gate is the plural choice. -/
theorem okChoseExtremeAfterNumbers :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [ instruct (each .anyPlayer) (choose (disclosure := .secretly) (a (quality .number))),
          .revealChoices .numbers,
          instruct (each (.and [.anyPlayer, .choseExtreme .max])) (loseLife .thatMuch) ]) = [] := by
  decide

/-- The same read with no number chosen anywhere in the text. -/
theorem badChoseExtremeWithoutChoice :
    Instruction.check (DRS.ofList []) (instruct (each (.and [.anyPlayer, .choseExtreme .max])) (loseLife (.lit 1)))
      = [.numberChoiceInScope] := by
  decide

/-! ## A choice domain has the sort its consumer announces -/

theorem badColorNounWithPlayerDomain :
    Predicate.checkAt (.quality .color) (DRS.ofList []) (.qualityNoun .color (some (.players .opponent)))
      = [.kindAxisSort] := by decide

theorem okColorNounWithColorDomain :
    Predicate.checkAt (.quality .color) (DRS.ofList []) (.qualityNoun .color (some (.colorOtherThan .red)))
      = [] := by decide

theorem badColorRefinementWithPlayerDomain :
    Predicate.checkAt .object (DRS.ofList []) (.ofYourChoice .color (some (.players .opponent)))
      = [.kindAxisSort] := by decide

theorem okColorRefinementWithColorDomain :
    Predicate.checkAt .object (DRS.ofList []) (.ofYourChoice .color (some (.colorOtherThan .red)))
      = [] := by
  decide

theorem badCompleteColorChoiceWithPlayerDomain :
    Instruction.check (DRS.ofList []) (.choose none
      (.described (.a .unmarked) (.qualityNoun .color (some (.players .opponent)))) .openly none)
      = [.kindAxisSort] := by decide

theorem okCompleteColorChoiceWithColorDomain :
    Instruction.check (DRS.ofList []) (.choose none
      (.described (.a .unmarked) (.qualityNoun .color (some (.colorOtherThan .red)))) .openly none)
      = [] := by decide

theorem badEntersChoiceDomainSort :
    StaticSpec.check (DRS.ofList [])
      (Primitives.StaticSpec.entryChoice thisCreature (.quality .color) (some (.players .opponent)) .openly)
      = [.kindAxisSort] := by decide

theorem okEntersChoiceDomainSort :
    StaticSpec.check (DRS.ofList [])
      (Primitives.StaticSpec.entryChoice thisCreature (.quality .color) (some (.colorOtherThan .red)) .openly)
      = [] := by decide

theorem badAttachmentChoiceDomainSort :
    StaticSpec.check (DRS.ofList [])
      (Primitives.StaticSpec.attachmentChoice thisCreature (.quality .color) (some (.players .opponent)))
      = [.kindAxisSort] := by decide

theorem okAttachmentChoiceDomainSort :
    StaticSpec.check (DRS.ofList [])
      (Primitives.StaticSpec.attachmentChoice thisCreature (.quality .color) (some (.colorOtherThan .red)))
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
        (some (.exists_ (.described .bare (.and [artifact, .hasPossessor .controller .you]))))))
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
    StaticSpec.check (DRS.ofList []) (.modeChoice (.setQuantity (atLeast 1))) = [] := by decide

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
    Instruction.check (DRS.ofList [])
      (.chooseModes (.worth 5)
        [(none, some 1, draw (.lit 1)), (none, some 2, destroy (target artifact))]) = [] := by
  decide

/-- "Choose up to zero {P} worth of modes" chooses nothing. -/
theorem badModalWorthZero :
    Instruction.check (DRS.ofList [])
      (.chooseModes (.worth 0)
        [(none, some 1, draw (.lit 1)), (none, some 2, destroy (target artifact))])
      = [.nonZeroQ] := by
  decide

end Semantics.Proofs.Choice
