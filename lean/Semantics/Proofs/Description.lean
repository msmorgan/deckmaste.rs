import Semantics.Macros
import Semantics.Check.Card
import Semantics.Proofs.Pin

/-!
# Semantics.Proofs.Description

Port of `idris/src/Experimental/ProofsDescription.idr`: the pins of the Description family.
A `Spelling` states that the checker admits a spelling (`= []`); a `Pin` states the one
obligation the same sentence mis-stated in one place refuses, which is the Idris pin's claim
(the term elaborates but for one obligation) made explicit. Every verdict is closed by
`decide`, so the kernel runs the checker.

The pins keep the Idris names, and each witness carries the sentence it spells.
-/

open Semantics Semantics.Macros

namespace Semantics.Proofs.Description

def controlledByGroup : Pin (Predicate.check .object []) :=
  pin (says "a creature target opponent controls"
        (.hasPossessor .controller (target .opponent)))
      (says "a creature two target opponents control"
        (.hasPossessor .controller (.described (.target (exactly 2)) .opponent)))
      .soleHolder

def eachOfSingular : Pin (NounPhrase.check (some .object) []) :=
  pin (says "each of up to two target creatures"
        (.eachOf (.described (.target (upTo 2)) creature)))
      (says "each of target creature" (.eachOf (target creature)))
      .plural

def sliceOfCountedPossessor : Pin (Instruction.check []) :=
  pin (says "Look at the top card of target player's library."
        (lookAt (.librarySlice .top (.lit 1) (target .anyPlayer))))
      (says "Look at the top card of two target players' library."
        (lookAt (.librarySlice .top (.lit 1) (.described (.target (exactly 2)) .anyPlayer))))
      .slicePossessor

def disjunctAntecedent : Pin (Instruction.check []) :=
  pin (says "Tap target creature an opponent controls. That player loses 1 life."
        (.sequentially
          [ .setStatus .tapped (target (.and [creature, .hasPossessor .controller anOpponent])),
            losesLife (that .player) (.lit 1) ]))
      (says "Tap target creature an opponent controls or land you control. That player loses \
              1 life."
        (.sequentially
          [ .setStatus .tapped (target (.or [.and [creature, .hasPossessor .controller anOpponent],
                                             .and [land, .hasPossessor .controller .you]])),
            losesLife (that .player) (.lit 1) ]))
      (.anaphor (.word .player) .one 0)

def keywordConjunction : Spelling (Instruction.check []) :=
  spelling (says "Tap target creature with flying."
    (.setStatus .tapped (target (.and [creature, .hasKeyword (.the "Flying")]))))

def keywordSelfNegation : Spelling (Instruction.check []) :=
  spelling (says "Tap target creature with flying that doesn't have flying."
    (.setStatus .tapped (target (.and [creature, .hasKeyword (.the "Flying"),
                                       .not (.hasKeyword (.the "Flying"))]))))

def forestNonland : Pin (Instruction.check []) :=
  pin (says "Tap target Forest." (.setStatus .tapped (target (.hasSubtype (landType "Forest")))))
      (says "Tap target nonland Forest."
        (.setStatus .tapped (target (.and [.hasSubtype (landType "Forest"), .not land]))))
      .contradictionFree

def contradictionFreeAnd : Spelling (Predicate.check .object []) :=
  spelling (says "creature with power 2 or less"
    (.and [creature, .compare [.stat .power] .atMost (.lit 2)]))

def noncreatureAttackingOrBlocking : Pin (Predicate.check .object []) :=
  pin (says "creature that is attacking or blocking" (.and [creature, .or [attacking, blocking]]))
      (says "noncreature that is attacking or blocking"
        (.and [.not creature, .or [attacking, blocking]]))
      .contradictionFree

def wrappedStatusLaunder : Pin (Predicate.check .object []) :=
  pin (says "creature that is an attacking artifact or a blocking land"
        (.and [.or [.and [artifact, attacking], .and [land, blocking]], creature]))
      (says "noncreature that is an attacking artifact or a blocking land"
        (.and [.or [.and [artifact, attacking], .and [land, blocking]], .not creature]))
      .contradictionFree

def blockedAndUnblocked : Spelling (Predicate.check .object []) :=
  spelling (says "blocked creature that's unblocked" (.and [creature, blocked, unblocked]))

def descendingRange : Pin (NounPhrase.check (some .object) []) :=
  pin (says "between two and three target creatures"
        (.described (.target (.range (some 2) (some 3))) creature))
      (says "between three and two target creatures"
        (.described (.target (.range (some 3) (some 2))) creature))
      .wellFormedQ

def partialZoneJoin : Pin (Predicate.check .object []) :=
  pin (says "attacking artifact or attacking land"
        (.or [.and [artifact, attacking], .and [land, attacking]]))
      (says "attacking artifact or land" (.or [.and [artifact, attacking], land]))
      .parallelDisjuncts

def distinctStructuredDisjuncts : Spelling (Predicate.check .object []) :=
  spelling (says "creature you control or artifact you control"
    (.or [.and [creature, .hasPossessor .controller .you],
          .and [artifact, .hasPossessor .controller .you]]))

def matchesTargetSubject : Pin (Condition.check []) :=
  pin (says "if this creature is attacking" (.matches thisCreature attacking))
      (says "if target creature is an artifact" (.matches (target creature) artifact))
      .testSubject

def matchesNothing : Pin (Instruction.check []) :=
  pin (says "Tap target creature. You gain 1 life if it's an artifact."
        (.sequentially [.setStatus .tapped (target creature),
                        .onlyIf (gainsLife .you (.lit 1)) (.matches it artifact) none]))
      (says "Tap target creature. You gain 1 life if it's."
        (.sequentially [.setStatus .tapped (target creature),
                        .onlyIf (gainsLife .you (.lit 1)) (.matches it (.and [])) none]))
      .predSays

def compareLiteralSubject : Pin (Condition.check []) :=
  pin (says "if the number of artifacts you control is 4 or greater"
        (.compareAmt (countOf (.and [artifact, .hasPossessor .controller .you])) .atLeast (.lit 4)))
      (says "if 3 is 4 or greater" (.compareAmt (.lit 3) .atLeast (.lit 4)))
      .readAmount

def eachOfDistributive : Pin (NounPhrase.check (some .object) []) :=
  pin (says "each of up to two target creatures"
        (.eachOf (.described (.target (upTo 2)) creature)))
      (says "each of each creature" (.eachOf (each creature)))
      .groupMention

def eachOfAll : Pin (NounPhrase.check (some .object) []) :=
  pin (says "each of up to two target creatures"
        (.eachOf (.described (.target (upTo 2)) creature)))
      (says "each of all creatures" (.eachOf (allOf creature)))
      .groupMention

def nestedEachOf : Pin (NounPhrase.check (some .object) []) :=
  pin (says "each of up to two target creatures"
        (.eachOf (.described (.target (upTo 2)) creature)))
      (says "each of each of up to two target creatures"
        (.eachOf (.eachOf (.described (.target (upTo 2)) creature))))
      .groupMention

def colorlessWhite : Pin (NounPhrase.check (some .object) []) :=
  pin (says "target white creature" (target (.and [creature, .colorIs .white])))
      (says "target colorless white creature"
        (target (.and [creature, colorless, .colorIs .white])))
      .contradictionFree

def doubleXRider : Pin (Instruction.check []) :=
  pin (says "Draw X cards, where X is the number of creatures you control."
        (.sequentially [.draw .you (.letter .x), .define .x (countOf creatureYouControl)]))
      (says "Draw X cards, where X is the number of creatures you control and X is the number \
              of creatures."
        (.sequentially [.draw .you (.letter .x), .define .x (countOf creatureYouControl),
                        .define .x (countOf creature)]))
      (.openLetter .x)

def unlicensedY : Pin (Instruction.check []) :=
  pin (says "Draw X cards, where X is the number of creatures you control."
        (.sequentially [.draw .you (.letter .x), .define .x (countOf creatureYouControl)]))
      (says "Draw Y cards, where X is the number of creatures you control."
        (.sequentially [.draw .you (.letter .y), .define .x (countOf creatureYouControl)]))
      (.openLetter .x)

/-- A noncreature permanent has no power [CR#208.3]. -/
def landPower : Pin (Amount.check []) :=
  pin (says "the power of target land creature"
        (.statOf (.stat .power) (target (.and [land, creature]))))
      (says "the power of target land" (.statOf (.stat .power) (target land)))
      .statHeadTysOk

/-- Loyalty is printed on planeswalkers [CR#209.1]. -/
def battleLoyalty : Pin (Amount.check []) :=
  pin (says "the loyalty of target planeswalker"
        (.statOf (.stat .loyalty) (target (.hasType .planeswalker))))
      (says "the loyalty of target battle"
        (.statOf (.stat .loyalty) (target (.hasType .battle))))
      .statHeadTysOk

def powerAmongPlayers : Pin (Amount.check []) :=
  pin (says "the greatest power among creatures" (aggregate .max (.stat .power) creature))
      (says "the greatest power among players" (aggregate .max (.stat .power) .anyPlayer))
      (.projScope .player)

def lifeTotalAmongObjects : Pin (Amount.check []) :=
  pin (says "the greatest power among creatures" (aggregate .max (.stat .power) creature))
      (says "the highest life total among creatures you control"
        (aggregate .max (.playerStat .lifeTotal) creatureYouControl))
      (.projScope .object)

def sumSelection : Pin (Predicate.check .object []) :=
  pin (says "the creature with the greatest power"
        (.superlative .max (.stat .power) creature))
      (says "the creature with the total power among creatures you control"
        (.superlative .sum (.stat .power) creature))
      .isExtremal

def bareDefinite : Pin (NounPhrase.check (some .object) []) :=
  pin (says "the creature with the least toughness among creatures you control"
        (the (.and [creature, .superlative .min (.stat .toughness) creatureYouControl])))
      (says "the creature" (the creature))
      .uniquifying

def playerStatSuperlative : Spelling (Predicate.check .player []) :=
  spelling (says "the player with the highest life total"
    (.superlative .max (.playerStat .lifeTotal) .anyPlayer))

def lifeTotalSuperlative : Pin (Predicate.check .object []) :=
  pin (says "the creature with the greatest power"
        (.superlative .max (.stat .power) creature))
      (says "the creature with the highest life total among creatures you control"
        (.superlative .max (.playerStat .lifeTotal) creature))
      (.projScope .object)

def ascribeInstant : Pin (NounPhrase.check (some .object) []) :=
  pin (says "this creature" (.asType .creature .this none))
      (says "this instant" (.asType .instant .this none))
      .ascriptionOk

def ascribeSorcery : Pin (NounPhrase.check (some .object) []) :=
  pin (says "this creature" (.asType .creature .this none))
      (says "this sorcery" (.asType .sorcery .this none))
      .ascriptionOk

def ascribeKindred : Pin (NounPhrase.check (some .object) []) :=
  pin (says "this creature" (.asType .creature .this none))
      (says "this kindred" (.asType .kindred .this none))
      .ascriptionOk

def ascribeForeignSubtype : Pin (NounPhrase.check (some .object) []) :=
  pin (says "this Aura enchantment"
        (.asType .enchantment .this (some (enchantmentType "Aura"))))
      (says "this Aura land" (.asType .land .this (some (enchantmentType "Aura"))))
      .ascriptionOk

def objectMonarch : Pin (Predicate.check .object []) :=
  pin (says "target creature that's goaded" (.hasDesignation "goaded" none))
      (says "target creature that is the monarch" (.hasDesignation "the monarch" none))
      (.designationHolder "the monarch" .object)

def partitiveOfCountedGroup : Pin (NounPhrase.check (some .object) []) :=
  pin (says "one of the top two cards of your library"
        (.someOf (.counted (exactly 1)) none (.librarySlice .top (.lit 2) .you)))
      (says "one of one or more creatures"
        (.someOf (.counted (exactly 1)) none (counted (atLeast 1) creature)))
      .partitiveBase

def itAfterAntecedent : Spelling (Instruction.check []) :=
  spelling (says "Destroy target creature. Its controller loses life equal to its power."
    (.sequentially
      [destroy (target creature), losesLife (controllerOf it) (.statOf (.stat .power) it)]))

/-- The "otherwise" arm cannot read the leading arm's token: it is not on the battlefield when
the otherwise arm runs, and there is no antecedent for it. -/
def otherwiseReadsLeadingArm : Pin (Instruction.check []) :=
  pin (says "If you control a creature, create a 1/1 black Zombie creature token. Otherwise, \
                tap target creature."
          (.if_ (exists_ creatureYouControl)
            (create (.lit 1) (creatureToken 1 1 [.black] [creatureType "Zombie"]))
            (some (.setStatus .tapped (target creature)))))
      (says "If you control a creature, create a 1/1 black Zombie creature token. Otherwise, \
              tap it."
        (.if_ (exists_ creatureYouControl)
          (create (.lit 1) (creatureToken 1 1 [.black] [creatureType "Zombie"]))
          (some (.setStatus .tapped it))))
      (.anaphor .bare .one 0) [.zoneIs .battlefield]

/-- A player mentioned only inside a leading condition is no antecedent for "that player". -/
def leadingConditionAntecedent : Pin (Instruction.check []) :=
  pin (says "Tap target creature an opponent controls. That player loses 1 life."
        (.sequentially
          [ .setStatus .tapped (target (.and [creature, .hasPossessor .controller anOpponent])),
            losesLife (that .player) (.lit 1) ]))
      (says "If your life total is less than an opponent's life total, you gain 6 life. That \
              player loses 1 life."
        (.sequentially
          [ .if_ (.compareAmt (lifeTotalOf .you) .less
                       (lifeTotalOf anOpponent))
              (gainsLife .you (.lit 6)) none,
            losesLife (that .player) (.lit 1) ]))
      (.anaphor (.word .player) .one 0)

def singletonConjunction : Pin (Condition.check []) :=
  pin (says "If you control an artifact and an enchantment, …"
        (.and [exists_ (.and [artifact, .hasPossessor .controller .you]),
               exists_ (.and [enchantment, .hasPossessor .controller .you])]))
      (says "If you control an artifact, create a token."
        (.and [exists_ (.and [artifact, .hasPossessor .controller .you])]))
      .atLeastTwo

def nestedConjunction : Pin (Condition.check []) :=
  pin (says "If you control an artifact and an enchantment, …"
        (.and [exists_ (.and [artifact, .hasPossessor .controller .you]),
               exists_ (.and [enchantment, .hasPossessor .controller .you])]))
      (says "If you control an artifact and an enchantment, and you control a land, …"
        (.and [ .and [exists_ (.and [artifact, .hasPossessor .controller .you]),
                      exists_ (.and [enchantment, .hasPossessor .controller .you])],
                exists_ (.and [land, .hasPossessor .controller .you]) ]))
      .flatConjuncts

def totalWithoutRoll : Pin (Instruction.check []) :=
  pin (says "Roll two d6. If you rolled 7, sacrifice this creature."
        (.sequentially [ rollDice .you 2 6,
                         if_ (.compareAmt (.theOutcome .rollResult) .eq (.lit 7))
                                (sacrifice .you thisCreature) ]))
      (says "If you rolled 7, sacrifice this creature."
        (if_ (.compareAmt (.theOutcome .rollResult) .eq (.lit 7)) (sacrifice .you thisCreature)))
      (.outcomeInScope .rollResult 0)

/-- "Look at the top card of your library. Put that card into your graveyard." -/
def okReadsLookedAtLibraryCard :
    Spelling
      (NounPhrase.check (some .object) (Instruction.intro [] (lookAt (topSlice (.lit 1))))) :=
  spelling (says "Look at the top card of your library. Put that card into your graveyard."
    (that .card))

/-- The shuffled-away card is read under different bindings from `okReadsLookedAtLibraryCard`,
so the two cannot share a `Pin`. -/
theorem badReadsShuffledIntoLibraryCard :
    NounPhrase.check (some .object)
      (Instruction.intro [] (.sequentially [lookAt (topSlice (.lit 1)), shuffleInto .you .this]))
      (that .card) = [.anaphor (.word .card) .one 0] := by
  decide

def compareCeilingSubject : Pin (Condition.check []) :=
  pin (says "if the number of artifacts you control is 4 or greater"
        (.compareAmt (countOf (.and [artifact, .hasPossessor .controller .you])) .atLeast (.lit 4)))
      (says "if up to three is 4 or greater" (.compareAmt (.upTo (.lit 3)) .atLeast (.lit 4)))
      .readAmount

/-- "target monocolored permanent": exactly one color. -/
def monocoloredIsOneColor : Spelling (Predicate.check .object []) :=
  spelling (says "target monocolored permanent" (.colorCount .eq 1))

/-- Zero colors is how "colorless" is spelled [CR#105.2c]. -/
def exactlySixColors : Pin (Predicate.check .object []) :=
  pin (says "target colorless permanent" (.colorCount .eq 0))
      (says "target permanent that's exactly six colors" (.colorCount .eq 6))
      .colorBoundOk

def paidCostOnCostlessKeyword : Pin (Amount.check []) :=
  pin (says "if this creature's kicker cost was paid"
        (.paid (.readback (.byKeyword "Kicker") none) .this))
      (says "if this creature's flying cost was paid"
        (.paid (.readback (.byKeyword "Flying") none) .this))
      .paidFacetNamed

def timesPaidUnknownKeyword : Pin (Amount.check []) :=
  pin (says "for each time it was kicked"
        (.paid (.timesPaid (.byKeyword "Kicker")) thisCreature))
      (says "for each time it was kickre'd"
        (.paid (.timesPaid (.byKeyword "Kickre")) thisCreature))
      .paidFacetNamed

def paidReadOffStack : Pin (Amount.check []) :=
  pin (says "for each color of mana spent to cast this spell" (colorsSpentToCast .this))
      (says "for each color of mana spent to cast a creature on the battlefield"
        (.paid .colorsSpent (a creature)))
      .paidSubject

/-- "if colored mana was spent to cast it" is a read of `paid colorsSpent` -/
theorem coloredManaSpentIsPaidColorsSpent :
    coloredManaSpentToCast .this = .compareAmt (.paid .colorsSpent .this) .atLeast (.lit 1) := rfl

def spellTargeter : Spelling (Predicate.check .object []) :=
  spelling (says "spell that targets this creature" (.targets thisCreature .someTarget))

/-- The player-domain refusal is checked under `Predicate.check .player`, not the `.object`
kind `spellTargeter` is admitted in, so the two cannot share a `Pin`. -/
theorem badPlayerTargeter :
    Predicate.check .player [] (.targets thisCreature .someTarget) = [.targeter .player] := by
  decide

def mixedAxisComparison : Pin (Predicate.check .object []) :=
  pin (says "a creature with power or toughness 2 or greater"
        (.compare [.stat .power, .stat .toughness] .atLeast (.lit 2)))
      (says "a creature with power or life total 3 or greater"
        (.compare [.stat .power, .playerStat .lifeTotal] .greater (.lit 1)))
      (.axesAt .object)

def playerReadInPlayerDomain : Spelling (Predicate.check .player []) :=
  spelling (says "a player with 13 or less life"
    (.and [.anyPlayer, .compare [.playerStat .lifeTotal] .atMost (.lit 13)]))

def playerReadInObjectDomain : Pin (Predicate.check .object []) :=
  pin (says "an object with 13 or less power" (.compare [.stat .power] .atMost (.lit 13)))
      (says "an object with 13 or less life"
        (.compare [.playerStat .lifeTotal] .atMost (.lit 13)))
      (.axesAt .object)

def cardToken : Pin (Predicate.check .object []) :=
  pin (says "each token on the battlefield" tokenOnTheBattlefield)
      (says "a card token" (.and [.isCard, .isToken]))
      .contradictionFree

def emblemPermanent : Pin (Predicate.check .object []) :=
  pin (says "each token on the battlefield" tokenOnTheBattlefield)
      (says "an emblem permanent" (.and [emblem, permanent]))
      .zoneCoherent

def cardCopyOfACard : Pin (Predicate.check .object []) :=
  pin (says "target card on the stack" cardOnTheStack)
      (says "a card that is a copy of a card" (.and [.isCard, copyOfACard]))
      .contradictionFree

def cantAttackCreature : Spelling (Instruction.check []) :=
  spelling (says "Target creature can't attack this turn."
    (cantAttack (target creature) (some .thisTurn)))

/-- [CR#508.1a,205.1b,208.3a] -/
def cantAttackLand : Spelling (Instruction.check []) :=
  spelling (says "Target land can't attack this turn." (cantAttack (target land) (some .thisTurn)))

def cantAttackLandNoSpan : Spelling (Instruction.check []) :=
  spelling (says "Target land can't attack." (cantAttack (target land) none))

def cantBlockCreature : Spelling (Instruction.check []) :=
  spelling (says "Target creature can't block this turn."
    (cantBlock (target creature) (some .thisTurn)))

/-- [CR#205.1b,208.3a] -/
def cantDisjunctSubject : Spelling (Instruction.check []) :=
  spelling (says "Target creature or land can't block this turn."
    (cantBlock (target (.or [creature, land])) (some .thisTurn)))

/-- A repeated role counts one creature twice [CR#700.8b]. -/
def repeatedPartyRole : Pin (NounPhrase.check (some .object) []) :=
  pin (says "your party" party)
      (says "one each of Cleric and Cleric"
        (.oneEachOf [.hasSubtype (creatureType "Cleric"), .hasSubtype (creatureType "Cleric")]
          (allOf creatureYouControl)))
      .rolesOk

/-- A party needs its roles [CR#700.8]. -/
def emptyPartyRoles : Pin (NounPhrase.check (some .object) []) :=
  pin (says "your party" party)
      (says "one each of nothing" (.oneEachOf [] (allOf creatureYouControl)))
      .rolesOk

/-- Fortification attaches to lands [CR#301.6,301.5]. -/
def fortifiedCreature : Pin (NounPhrase.check (some .object) []) :=
  pin (says "a fortified land" (a (.and [land, .isAttached (some .fortified)])))
      (says "a fortified creature" (a (.and [creature, .isAttached (some .fortified)])))
      .contradictionFree

end Semantics.Proofs.Description
