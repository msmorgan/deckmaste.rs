import Semantics.Macros
import Semantics.Check.Card

/-!
# Semantics.Proofs.Choice

Port of `idris/src/Experimental/Proofs/Choice.idr`: the pins of the Choice family, in theorem
form, each closed by `decide`. The names and the sentences are the Idris ones.
-/

open Semantics Semantics.Macros

namespace Semantics.Proofs.Choice

/-- "the chosen number", one number choice standing -/
theorem okChosenNumberOneStanding : Amount.check [qualityB .number] chosenNumber = [] := by decide

/-- "... is equal to the chosen number" -/
theorem badChosenNumberTwoStanding :
    Amount.check [qualityB .number, qualityB .number] chosenNumber
      = [.choiceRef .theChoice (.quality .number) 2] := by
  decide

/-- "of the chosen color" -/
theorem okChosenColorRead : Predicate.check .object [qualityB .color] (ofChosen .color) = [] := by
  decide

/-- "of the chosen number" -/
theorem badChosenNumberRead :
    Predicate.check .object [qualityB .number] (ofChosen .number)
      = [.chosenQualityRead .number] := by
  decide

/-- "Choose target creature. You gain life equal to its power." -/
theorem okSinglePower :
    Instruction.check [] (.sequentially [choose (target creature), gainLife (powerOf it) (agent :=
        .you)])
      = [] := by
  decide

/-- "Choose two target creatures. You gain life equal to their power." -/
theorem badGroupPower :
    Instruction.check []
      (.sequentially
        [choose (.described (.target (exactly 2)) creature), gainLife (powerOf them) (agent :=
            .you)])
      = [.singular] := by
  decide

/-- "Choose target creature. Its owner loses 1 life." -/
theorem okSingleOwner :
    Instruction.check []
      (.sequentially [choose (target creature), loseLife (.lit 1) (agent := (ownerOf it))]) = [] := by
  decide

/-- "Choose two target creatures. Their owners each lose 1 life." -/
theorem okGroupOwners :
    Instruction.check []
      (.sequentially
        [choose (.described (.target (exactly 2)) creature), loseLife (.lit 1) (agent := (ownerOf
            them))])
      = [] := by
  decide

/-- "Choose target creature." -/
theorem okTargetCreature : Instruction.check [] (choose (target creature)) = [] := by decide

/-- "Choose target color." -/
theorem badTargetColor :
    Instruction.check [] (choose (target (quality .color)))
      = [.targetable (.quality .color)] := by
  decide

/-- "Choose two target creatures." -/
theorem okTwoGroup :
    Instruction.check [] (choose (.described (.target (exactly 2)) creature)) = [] := by decide

/-- "Choose zero target creatures." -/
theorem badZeroGroup :
    Instruction.check [] (choose (.described (.target (exactly 0)) creature)) = [.nonZeroQ] := by
  decide

/-- "Choose up to one — Destroy target artifact; or destroy target enchantment." -/
theorem okModalTwoModes :
    Instruction.check []
      (.chooseModes (upTo 1)
        [(none, destroy (target artifact)), (none, destroy (target enchantment))]) = [] := by
  decide

/-- "Choose one — Destroy target artifact." -/
theorem badModalOneMode :
    Instruction.check [] (.chooseModes (upTo 1) [(none, destroy (target artifact))])
      = [.atLeastTwo] := by
  decide

/-- "Choose two — Destroy target artifact; or destroy target enchantment." -/
theorem okModalTwoOfTwo :
    Instruction.check []
      (.chooseModes (exactly 2)
        [(none, destroy (target artifact)), (none, destroy (target enchantment))]) = [] := by
  decide

/-- "Choose three — Destroy target artifact; or destroy target enchantment." -/
theorem badModalOverreach :
    Instruction.check []
      (.chooseModes (exactly 3)
        [(none, destroy (target artifact)), (none, destroy (target enchantment))])
      = [.modesFit] := by
  decide

/-- "Choose one or more — + {1} — destroy target artifact; + {1} — destroy target enchantment."
[CR#702.172a] -/
theorem okSpreeBothCosted :
    Instruction.check []
      (chooseSpree
        [ (some (.mana [generic 1]), destroy (target artifact)),
          (some (.mana [generic 1]), destroy (target enchantment)) ]) = [] := by
  decide

/-- "Choose one or more — destroy target artifact; + {1} — destroy target enchantment.": a
spree mode with no cost [CR#702.172a] -/
theorem badSpreeMissingCost :
    Instruction.check []
      (chooseSpree
        [ (none, destroy (target artifact)),
          (some (.mana [generic 1]), destroy (target enchantment)) ]) = [.modesCosted] := by
  decide

/-- "Choose one — Destroy target artifact; or tap it." The Idris pin refutes the zone; the
unresolved `it` is refused first. -/
theorem badModalReadsAcrossModes :
    Instruction.check []
      (chooseModes (exactly 1) [destroy (target artifact), .setStatus .tapped it])
      = [.anaphor .bare .one 0, .zoneIs .battlefield] := by
  decide

theorem badReadsAfterModal :
    Instruction.check []
      (.sequentially
        [ chooseModes (exactly 1) [destroy (target artifact), destroy (target enchantment)],
          .setStatus .tapped it ]) = [.anaphor .bare .one 0, .zoneIs .battlefield] := by
  decide

/-- "you pay 1 life" -/
theorem okMatchedPayer :
    Instruction.check []
      (Primitives.Instruction.offer (.pay (payLife .you 1) .once (agent := .you)) none (some (draw (.lit 1) (agent :=
          .you))) (agent := .you)) = [] := by
  decide

/-- "you pay" -/
theorem badMismatchedPayer :
    Instruction.check []
      (Primitives.Instruction.offer (.pay (payLife anOpponent 1) .once (agent := .you)) none (some (draw (.lit 1) (agent
          := .you))) (agent := .you))
      = [.payAgrees] := by
  decide

/-- "Choose new targets for target instant or sorcery spell." -/
theorem okRetargetStackSpell :
    Instruction.check [] (.chooseNewTargets (target (.and [instantOrSorcery, spell]))) = [] := by
  decide

/-- "Choose new targets for target creature." -/
theorem badRetargetPermanent :
    Instruction.check [] (.chooseNewTargets (target creature)) = [.stackActOn] := by decide

/-- "Your opponents can't gain life." -/
theorem okStaticPlayerCant :
    Ability.check [] (.static (playerCant (.core .gainLife) (.playerGroup .yourOpponents)))
      = [] := by
  decide

/-- "Target player can't gain life." -/
theorem badStaticPlayerCantTargets :
    Ability.check [] (.static (playerCant (.core .gainLife) (target .anyPlayer)))
      = [.nontarget] := by
  decide

/-- "the basic land type of your choice" -/
def chosenBasicLandType : QualityPayload :=
  .chosenQuality (.ofYourChoice (.subtype .land) (some .basicTypesOnly))

/-- "Target land becomes the basic land type of your choice until end of turn." -/
theorem okChosenBasicTypeOnLand :
    Instruction.check []
      (.establish (Primitives.StaticSpec.qualityChange (target land) .sets chosenBasicLandType) (some untilEndOfTurn))
      = [] := by
  decide

theorem badChosenBasicTypeOnCreature :
    Instruction.check []
      (.establish (Primitives.StaticSpec.qualityChange (target creature) .sets chosenBasicLandType)
        (some untilEndOfTurn)) = [.becomesOk] := by
  decide

/-- "Lands you control are Mountains." -/
theorem okLandsAreMountains :
    StaticSpec.check []
      (Primitives.StaticSpec.qualityChange (allOf (.and [land, .hasPossessor .controller .you])) .sets
        (.bundle { characteristics := { subtypes := [landType "Mountain"] } } none)) = [] := by
  decide

/-- "Creatures are Mountains." -/
theorem badCreaturesAreMountains :
    StaticSpec.check []
      (Primitives.StaticSpec.qualityChange (allOf creature) .sets (.bundle { characteristics :=
                                                  { subtypes := [landType "Mountain"] } } none))
      = [.becomesOk] := by
  decide

/-- "Choose a creature type other than Wall." -/
theorem okCreatureTypeExclusion :
    sortedDomainCheck [] (.quality (.subtype .creature)) (some (.typeOtherThan (creatureType "Wall")))
      = [] := by
  decide

/-- "Choose a creature type other than Equipment." -/
theorem badNonCreatureTypeExclusion :
    sortedDomainCheck [] (.quality (.subtype .creature)) (some (.typeOtherThan (artifactType "Equipment")))
      = [.subtypeType] := by
  decide

/-- "Target land becomes every basic land type until end of turn." -/
theorem setsEveryBasicLandType :
    Instruction.check []
      (.establish (Primitives.StaticSpec.qualityChange (target land) .sets (.everyTypeOf .basicLand))
        (some untilEndOfTurn)) = [] := by
  decide

/-- "Target creature loses the creature type of your choice until end of turn." -/
theorem losesChosenCreatureType :
    Instruction.check []
      (.establish
        (Primitives.StaticSpec.qualityChange (target creature) .loses
          (.chosenQuality (.ofYourChoice (.subtype .creature) none))) (some untilEndOfTurn))
      = [] := by
  decide

/-- "Target creature loses all colors until end of turn." -/
theorem losesAllColors :
    Instruction.check []
      (.establish (Primitives.StaticSpec.qualityChange (target creature) .loses (.colored .every)) (some untilEndOfTurn))
      = [] := by
  decide

/-- "Target creature is white." -/
theorem okAddsAColor :
    StaticSpec.check [] (Primitives.StaticSpec.qualityChange (target creature) .adds (.colored (.some [.white]))) = [] :=
        by
  decide

theorem badAddsNoColor :
    StaticSpec.check [] (Primitives.StaticSpec.qualityChange (target creature) .adds (.colored (.some [])))
      = [.becomesOk] := by
  decide

/-- "Target creature loses colorless until end of turn." -/
theorem badLosesNoColor :
    StaticSpec.check [] (Primitives.StaticSpec.qualityChange (target creature) .loses (.colored (.some [])))
      = [.becomesOk] := by
  decide

/-- "Equipped permanent isn't a 2/2 creature." -/
theorem badLosesPt :
    StaticSpec.check []
      (Primitives.StaticSpec.qualityChange (.attachHost .equipped .permanent) .loses
        (.bundle { characteristics :=
                   { types := [.creature], power := some (.lit 2), toughness := some (.lit 2) } }
          none)) = [.becomesOk] := by
  decide

theorem badStillOnAddition :
    StaticSpec.check []
      (Primitives.StaticSpec.qualityChange (target creature) .adds
        (.bundle { characteristics := { types := [.artifact] } } (some .creature)))
      = [.becomesOk] := by
  decide

def afterChoiceMade : Bindings := Instruction.intro [] (choose (counted (upTo 1) creature))

/-- "Choose up to one creature. Destroy the rest." -/
theorem okChoiceRestStands :
    NounPhrase.check (some .object) afterChoiceMade (theRest .object) = [] := by decide

def afterChoiceRestDisposed : Bindings :=
  Instruction.intro []
    (.sequentially [choose (counted (upTo 1) creature), destroy (theRest .object)])

/-- "Choose up to one creature. Destroy the rest. Destroy the rest." -/
theorem badChoiceRestDisposedTwice :
    NounPhrase.check (some .object) afterChoiceRestDisposed (theRest .object)
      = [.theRestFits .object] := by
  decide

/-- "up to one creature they control" -/
def upToOneTheyControl : NounPhrase :=
  counted (upTo 1) (.and [creature, .hasPossessor .controller they])

/-- "Each player chooses up to one creature they control, then sacrifices the rest." -/
def distributedRestOfOwnChoice : Instruction :=
  .sequentially
    [ choose upToOneTheyControl (agent := some (each .anyPlayer)),
      sacrifice (theRest .object) (agent := (each .anyPlayer)) ]

/-- "Each player chooses up to one creature they control, then sacrifices the rest." The
per-agent rest closes only the partitives the same distributive chooser published [CR#700.8d],
so nothing the table shares is spent. -/
theorem okDistributedRestOfOwnChoice : Instruction.check [] distributedRestOfOwnChoice = [] := by
  decide

def afterDistributedChoice : Bindings :=
  Instruction.intro [] (choose upToOneTheyControl (agent := some (each .anyPlayer)))

/-- "Each player chooses up to one creature they control, then sacrifices the rest." -/
theorem okDistributedRestStands :
    NounPhrase.check (some .object) afterDistributedChoice (theRest .object) = [] := by decide

def afterDistributedRestSacrificed : Bindings := Instruction.intro [] distributedRestOfOwnChoice

/-- "Each player chooses up to one creature they control, then sacrifices the rest, then
sacrifices the rest.": the per-agent rest closes the partitives it spent, so no rest stands to
spend again. -/
theorem badDistributedRestDisposedTwice :
    NounPhrase.check (some .object) afterDistributedRestSacrificed (theRest .object)
      = [.theRestFits .object] := by
  decide

/-- "Tap all creatures. Each player chooses up to one creature they control." -/
theorem okSharedGroupWithoutARest :
    Instruction.check []
      (.sequentially [tap (allOf creature), choose upToOneTheyControl (agent := some (each
          .anyPlayer))])
      = [] := by
  decide

/-- "For each player, choose target permanent that player controls. Those players sacrifice
those permanents.": the plural deed re-places only the plural bindings the same loop published
[CR#701.21a]. -/
theorem okDistributedLoopParts :
    Instruction.check []
      (.sequentially
        [ .doForEach (each .anyPlayer)
            (choose (target (.and [permanent, .hasPossessor .controller they]))),
          sacrifice (those .permanent) (agent := (those .player)) ]) = [] := by
  decide

/-- "Tap all creatures. Each player chooses up to one creature they control, then sacrifices
the rest.": the rest here spends the group the whole table shares, and a player can't
sacrifice a permanent they don't control ([CR#701.21a]; `okDistributedRestOfOwnChoice` is the
same deed with only the chooser's own partitives standing). -/
theorem badDistributedRestOfSharedGroup :
    Instruction.check []
      (.sequentially
        [ tap (allOf creature),
          choose upToOneTheyControl (agent := some (each .anyPlayer)),
          sacrifice (theRest .object) (agent := (each .anyPlayer)) ]) = [.enactKeepsOuter] := by
  decide

/-- "Choose up to one creature. Each player sacrifices the rest.": one chooser leaves one
shared leftover, not a partition per player, so the other players would sacrifice permanents
they don't control [CR#701.21a]. -/
theorem badDistributedRestOfSingularChoice :
    Instruction.check []
      (.sequentially
        [choose (counted (upTo 1) creature), sacrifice (theRest .object) (agent := (each
            .anyPlayer))])
      = [.enactKeepsOuter] := by
  decide

/-- "an opponent who controls more lands than you control" -/
theorem okComparisonBoundOutsideTheMember :
    Predicate.check .player []
      (.compareOver .opponent (countOf (.and [land, .hasPossessor .controller they])) .greater
        (countOf (.and [land, .hasPossessor .controller .you]))) = [] := by
  decide

/-- "an opponent who controls more lands than they control" -/
theorem badMemberInComparisonBound :
    Predicate.check .player []
      (.compareOver .opponent (countOf (.and [land, .hasPossessor .controller .you])) .greater
        (countOf (.and [land, .hasPossessor .controller they])))
      = [.anaphor (.word .player) .one 0] := by
  decide

/-- "Target creature gets +3/+3 until end of turn." -/
theorem okGetsBattlefield :
    Instruction.check []
      (get (target creature) (.up (.lit 3)) (.up (.lit 3)) (some untilEndOfTurn)) = [] := by
  decide

/-- "Destroy target creature. It gets +3/+3 until end of turn." Both halves of the boost read
the destroyed creature. -/
theorem badGetsGraveyard :
    Instruction.check []
      (.sequentially
        [destroy (target creature), get it (.up (.lit 3)) (.up (.lit 3)) (some untilEndOfTurn)])
      = [.zoneIs .battlefield, .zoneIs .battlefield] := by
  decide

/-- "Choose a creature. This deals 3 damage to each creature not chosen this way." -/
theorem okNotChosenAfterOneChoice :
    Instruction.check []
      (.sequentially
        [ choose (a creature) (agent := some .you),
          .dealDamage .this (.lit 3) (each (.and [creature, .notChosen])) ]) = [] := by
  decide

/-- "This deals 3 damage to each creature not chosen this way." Nothing was chosen, so "this
way" reads back no choice (`okNotChosenAfterOneChoice` is the same read with one standing). -/
theorem badNotChosenWithoutAChoice :
    Instruction.check [] (.dealDamage .this (.lit 3) (each (.and [creature, .notChosen])))
      = [.choiceInScope .object] := by
  decide

/-- "Choose a creature. Choose a creature. This deals 3 damage to each creature not chosen this
way.": "this way" names the manner, so both standing choices are excluded together
[CR#700.8d]. -/
theorem okNotChosenAfterTwoChoices :
    Instruction.check []
      (.sequentially
        [ choose (a creature) (agent := some .you),
          choose (a creature) (agent := some .you),
          .dealDamage .this (.lit 3) (each (.and [creature, .notChosen])) ]) = [] := by
  decide

/-- "Choose a creature you control, then each opponent chooses a creature they control.
Destroy each creature not chosen this way.": Sculpted Sunburst's exclusion, whose two standing
choices have different choosers [CR#101.4]. -/
theorem okNotChosenAcrossChoosers :
    Instruction.check []
      (.sequentially
        [ choose (a creatureYouControl),
          choose (a (.and [creature, .hasPossessor .controller they])) (agent := some (each
              .opponent)),
          destroy (each (.and [creature, .notChosen])) ]) = [] := by
  decide

/-- "Each player chooses a creature they control." [CR#700.8d] -/
theorem okAgentScopedChoice :
    Instruction.check []
      (choose (a (.and [creature, .hasPossessor .controller they])) (agent := some (each
          .anyPlayer))) = [] := by
  decide

/-- "Choose a creature they control.": the chooserless spelling has no antecedent for "they"
(`okAgentScopedChoice` is the same noun under a chooser, which publishes one). -/
theorem badUnchooseredTheyControl :
    Instruction.check [] (choose (a (.and [creature, .hasPossessor .controller they])))
      = [.anaphor (.word .player) .one 0] := by
  decide

/-- "Starting with you, each player chooses a creature they control." [CR#101.4] -/
theorem okChoiceStartingWithYou :
    Instruction.check []
      (.choose (some .you)
        (a (.and [creature, .hasPossessor .controller they])) .openly none (agent := (some (each
            .anyPlayer)))) = [] := by
  decide

/-- "Starting with you, target player chooses a creature.": one player makes the choice, so
there is no order for "starting with" to fix [CR#101.4] (`okChoiceStartingWithYou` is the
same order over a distributive chooser). -/
theorem badOrderedSingularChooser :
    Instruction.check []
      (.choose (some .you) (a creature) .openly none (agent := (some (target .anyPlayer))))
      = [.choiceOrder] := by
  decide

/-- "Choose a creature. If you chose a creature this way, draw a card." -/
theorem okChoseThisWayAfterChoice :
    Instruction.check []
      (.sequentially
        [choose (a creature) (agent := some .you), doIf (.choseThisWay .you creature) (draw (.lit 1)
            (agent := .you))])
      = [] := by
  decide

/-- "If you chose a creature this way, draw a card.": nothing was chosen, so "this way" reads
back no choice (`okChoseThisWayAfterChoice` is the same read with one standing). -/
theorem badChoseThisWayWithoutAChoice :
    Instruction.check [] (doIf (.choseThisWay .you creature) (draw (.lit 1) (agent := .you)))
      = [.choiceInScope .object] := by
  decide

/-- "Each player chooses a creature. Exile them." -/
theorem okDistributedChoiceReadsAsGroup :
    Instruction.check [] (.sequentially [choose (a creature) (agent := some (each .anyPlayer)), exile
        them])
      = [] := by
  decide

/-- "Each player chooses a creature. Exile it.": a distributive choice stands as one per
chooser, so the singular read has no antecedent (`okDistributedChoiceReadsAsGroup` is the
plural read). -/
theorem badDistributedChoiceReadSingular :
    Instruction.check [] (.sequentially [choose (a creature) (agent := some (each .anyPlayer)), exile
        it])
      = [.anaphor .bare .one 0] := by
  decide

/-- "Look at the top card of your library. You may put that card into your graveyard." No
printed card on the bench. -/
theorem lookAtTopThenBin :
    Instruction.check []
      (.sequentially [lookAt (topSlice (.lit 1)), offer (move (that .card) graveyard) (agent := .you)])
      = [] := by
  decide

/-- "Until end of turn, you may play lands and cast spells from your graveyard." No printed
card on the bench. -/
theorem playAndCastFromGraveyardThisTurn :
    Instruction.check []
      (.establish
        (.conjunction none
          [ mayPlayDeed (.action "Play") .you (allOf land) none
              (.play (some (graveyardOf .you)) none none false .itsOwnCost),
            mayPlayDeed (.action "Cast") .you (allOf spell) none
              (.play (some (graveyardOf .you)) none none false .itsOwnCost) ])
        (some untilEndOfTurn)) = [] := by
  decide

/-- "Each player may play an additional land on each of their turns." No printed card on the
bench. -/
theorem eachPlayerPlaysAdditionalLand :
    StaticSpec.check [] (mayPlayAdditionalLands (each .anyPlayer) (exactly 1)) = [] := by decide

/-- "Mill three cards. You may put an artifact card from among the cards milled this way into
your hand." No printed card on the bench. -/
theorem millThenPutFromAmongMilled :
    Instruction.check []
      (.sequentially
        [ mill (.lit 3) .you (agent := .you),
          offer
            (move (fromAmong (exactly 1) artifact (theVerbed (.action "Mill") .card .thisWay .many))
              hand) (agent := .you) ]) = [] := by
  decide

/-- "Each player may shuffle their hand and graveyard into their library." No printed card on
the bench. -/
theorem eachPlayerMayShuffleTheirHandAndGraveyard :
    Instruction.check []
      (offer
        (shuffleInto
          (Primitives.NounPhrase.both (allOf (.inZone (handOf they))) (allOf (.inZone (graveyardOf they)))) (agent :=
              they)) (agent := (each .anyPlayer)))
      = [] := by
  decide

/-- "Each player may discard their hand and draw seven cards." No printed card on the bench. -/
theorem eachPlayerMayDiscardTheirHandAndDrawSeven :
    Instruction.check []
      (offer
        (.sequentially [discard (allOf (.inZone (handOf they))) (agent := they), draw (.lit 7) (agent :=
            they)]) (agent := (each .anyPlayer)))
      = [] := by
  decide

/-- "Each player may …": the offer subject and the two counts its context publishes. No
printed card on the bench. -/
def eachPlayerOffered : NounPhrase := each .anyPlayer

theorem eachPlayerOfferBindsOneMember :
    countOnes .player (mayCtx [] eachPlayerOffered) = 1 := by decide

theorem eachPlayerOfferDropsTheGroup :
    countManys .player (mayCtx [] eachPlayerOffered) = 0 := by decide

/-- "target opponent who has more life than you do" -/
def opponentWithMoreLife : NounPhrase :=
  target (.and [.opponent, .compare [.playerStat .lifeTotal] .greater (lifeTotalOf .you)])

/-- "Choose target opponent who has more life than you do as you activate this ability.": the
rider times the announcement, not the comparison. -/
theorem okChoiceWithActivationRider :
    Instruction.check []
      (chooseWhile opponentWithMoreLife (.whileDoing (.activates .you thisAbility))) = [] := by
  decide

/-- The same choice ridered on a death, which is not an event a choice can be made during. -/
theorem badChoiceRiderNotUnderway :
    Instruction.check [] (chooseWhile opponentWithMoreLife (.whileDoing (.dies thisCreature)))
      = [.eventUnderway] := by
  decide

/-- "Each player secretly chooses a number. Then those numbers are revealed. Each player who
chose the highest number loses that much life." (Menacing Ogre): not a vote [CR#701.38c]; the
gate is the plural choice. -/
theorem okChoseExtremeAfterNumbers :
    Instruction.check []
      (.sequentially
        [ choose (disclosure := .secretly) (a (quality .number)) (agent := some (each .anyPlayer)),
          .revealChoices .numbers,
          loseLife .thatMuch (agent := (each (.and [.anyPlayer, .choseExtreme .max]))) ]) = [] := by
  decide

/-- The same read with no number chosen anywhere in the text. -/
theorem badChoseExtremeWithoutChoice :
    Instruction.check [] (loseLife (.lit 1) (agent := (each (.and [.anyPlayer, .choseExtreme
        .max]))))
      = [.numberChoiceInScope] := by
  decide

/-! ## A choice domain has the sort its consumer announces -/

theorem badColorNounWithPlayerDomain :
    Predicate.check (.quality .color) [] (.qualityNoun .color (some (.players .opponent)))
      = [.kindAxisSort] := by decide

theorem okColorNounWithColorDomain :
    Predicate.check (.quality .color) [] (.qualityNoun .color (some (.colorOtherThan .red)))
      = [] := by decide

theorem badColorRefinementWithPlayerDomain :
    Predicate.check .object [] (.ofYourChoice .color (some (.players .opponent)))
      = [.kindAxisSort] := by decide

theorem okColorRefinementWithColorDomain :
    Predicate.check .object [] (.ofYourChoice .color (some (.colorOtherThan .red)))
      = [] := by
  decide

theorem badCompleteColorChoiceWithPlayerDomain :
    Instruction.check [] (.choose none
      (.described (.a .unmarked) (.qualityNoun .color (some (.players .opponent)))) .openly none
          (agent := (some .you)))
      = [.kindAxisSort] := by decide

theorem okCompleteColorChoiceWithColorDomain :
    Instruction.check [] (.choose none
      (.described (.a .unmarked) (.qualityNoun .color (some (.colorOtherThan .red)))) .openly none
          (agent := (some .you)))
      = [] := by decide

theorem badEntersChoiceDomainSort :
    StaticSpec.check []
      (.entryChoice thisCreature (.quality .color) (some (.players .opponent)) .openly)
      = [.kindAxisSort] := by decide

theorem okEntersChoiceDomainSort :
    StaticSpec.check []
      (.entryChoice thisCreature (.quality .color) (some (.colorOtherThan .red)) .openly)
      = [] := by decide

theorem badAttachmentChoiceDomainSort :
    StaticSpec.check []
      (.attachmentChoice thisCreature (.quality .color) (some (.players .opponent)))
      = [.kindAxisSort] := by decide

theorem okAttachmentChoiceDomainSort :
    StaticSpec.check []
      (.attachmentChoice thisCreature (.quality .color) (some (.colorOtherThan .red)))
      = [] := by decide

theorem badLandSubtypeDomainForCreatureType :
    Predicate.check (.quality (.subtype .creature)) []
      (.qualityNoun (.subtype .creature) (some .basicTypesOnly)) = [.kindAxisSort] := by decide

theorem okLandSubtypeDomainForLandType :
    Predicate.check (.quality (.subtype .land)) []
      (.qualityNoun (.subtype .land) (some .basicTypesOnly)) = [] := by decide

theorem badDomainContentsAndSort :
    Predicate.check (.quality .color) [] (.qualityNoun .color (some (.abilitiesAmong [])))
      = [.nonEmpty, .kindAxisSort] := by decide

theorem okAbilityDomainContentsAndSort :
    Predicate.check (.quality .ability) []
      (.qualityNoun .ability (some (.abilitiesAmong [.the "Flying"]))) = [] := by decide

end Semantics.Proofs.Choice
