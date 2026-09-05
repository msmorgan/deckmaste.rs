module Experimental.Proofs.Choice

import Experimental
import Experimental.Macros
import Experimental.Unspellable

%default total

%unbound_implicits off


||| "the chosen number", one number choice standing
public export
okChosenNumberOneStanding : Amount [qualityB Number]
okChosenNumberOneStanding = Macros.chosenNumber

||| "... is equal to the chosen number"
public export
badChosenNumberTwoStanding :
  Unspellable (Amount [qualityB Number, qualityB Number]) (\ok => Macros.chosenNumber {ok})
badChosenNumberTwoStanding Refl impossible

||| "of the chosen color"
public export
okChosenColorRead :
  Predicate [MkBinding AD (Quality Color) OneOf QualityP] Object
okChosenColorRead = Macros.ofChosen Color

||| "of the chosen number"
public export
badChosenNumberRead :
  Unspellable
    (Predicate [MkBinding AD (Quality Number) OneOf QualityP] Object)
    (\ok => Macros.ofChosen Number {read = ok})
badChosenNumberRead Oh impossible

||| "Choose target creature. You gain life equal to its power."
public export
okSinglePower : Instruction []
okSinglePower =
  Sequentially [Choose Nothing Nothing (Macros.target Macros.creature) Openly Nothing,
                Macros.gainsLife You (StatOf Power (Macros.It))]

||| "Choose two target creatures. You gain life equal to their power."
public export
badGroupPower : Unspellable (Instruction []) (\ok =>
  Sequentially [Choose Nothing Nothing (Described (TargetDet (Macros.exactly 2)) Macros.creature) Openly Nothing,
                Macros.gainsLife You (StatOf Power ((Macros.Them)) {one = ok})])
badGroupPower Refl impossible

||| "Choose target creature. Its owner loses 1 life."
public export
okSingleOwner : Instruction []
okSingleOwner =
  Sequentially [Choose Nothing Nothing (Macros.target Macros.creature) Openly Nothing,
                Macros.losesLife (Macros.ownerOf (Macros.It)) (Lit 1)]

||| "Choose two target creatures. Their owners each lose 1 life."
public export
okGroupOwners : Instruction []
okGroupOwners =
  Sequentially [Choose Nothing Nothing (Described (TargetDet (Macros.exactly 2)) Macros.creature) Openly Nothing,
                Macros.losesLife (Macros.ownerOf (Macros.Them)) (Lit 1)]

||| "Choose target creature."
public export
okTargetCreature : Instruction []
okTargetCreature = Choose Nothing Nothing (Macros.target Macros.creature) Openly Nothing

||| "Choose target color."
public export
badTargetColor : Unspellable (Instruction []) (\ok =>
  Choose Nothing Nothing (Macros.target (QualityNoun Color Nothing) {tk = ok}) Openly Nothing)
badTargetColor ObjectTgt impossible

||| "Choose two target creatures."
public export
okTwoGroup : Instruction []
okTwoGroup =
  Choose Nothing Nothing (Described (TargetDet (Macros.exactly 2)) Macros.creature) Openly Nothing

||| "Choose zero target creatures."
public export
badZeroGroup : Unspellable (Instruction []) (\ok =>
  Choose Nothing Nothing (Described (TargetDet (Macros.exactly 0)) Macros.creature {ok}) Openly Nothing)
badZeroGroup (MaxAtLeastOne, _, _) impossible

||| "Choose up to one — Destroy target artifact; or destroy target enchantment."
public export
okModalTwoModes : Instruction []
okModalTwoModes =
  Modal (Macros.upTo 1) [(Nothing, Macros.destroy (Macros.target Macros.artifact)),
                         (Nothing, Macros.destroy (Macros.target Macros.enchantment))]

||| "Choose one — Destroy target artifact."
public export
badModalOneMode : Unspellable (Instruction []) (\ok =>
  Modal (Macros.upTo 1) [(Nothing, Macros.destroy (Macros.target Macros.artifact))] {tw = ok})
badModalOneMode Oh impossible

||| "Choose two — Destroy target artifact; or destroy target enchantment."
public export
okModalTwoOfTwo : Instruction []
okModalTwoOfTwo =
  Modal (Macros.exactly 2) [(Nothing, Macros.destroy (Macros.target Macros.artifact)),
                            (Nothing, Macros.destroy (Macros.target Macros.enchantment))]

||| "Choose three — Destroy target artifact; or destroy target enchantment."
public export
badModalOverreach : Unspellable (Instruction []) (\ok =>
  Modal (Macros.exactly 3) [(Nothing, Macros.destroy (Macros.target Macros.artifact)),
                            (Nothing, Macros.destroy (Macros.target Macros.enchantment))] {mf = ok})
badModalOverreach Oh impossible

||| "Choose one or more — + {1} — destroy target artifact; + {1} — destroy target enchantment." [CR#702.172a]
public export
okSpreeBothCosted : Instruction []
okSpreeBothCosted =
  Macros.spree [(Just (Mana [Macros.generic 1]), Macros.destroy (Macros.target Macros.artifact)),
                (Just (Mana [Macros.generic 1]), Macros.destroy (Macros.target Macros.enchantment))]

||| "Choose one or more — destroy target artifact; + {1} — destroy target enchantment." — a
||| spree mode with no cost [CR#702.172a]
public export
badSpreeMissingCost : Unspellable (Instruction []) (\ok =>
  Macros.spree [(Nothing, Macros.destroy (Macros.target Macros.artifact)),
                (Just (Mana [Macros.generic 1]), Macros.destroy (Macros.target Macros.enchantment))] {ac = ok})
badSpreeMissingCost Oh impossible

||| "Choose one — Destroy target artifact; or tap it."
public export
badModalReadsAcrossModes : Unspellable (Instruction []) (\ok =>
  Macros.chooseModes (Macros.exactly 1) [Macros.destroy (Macros.target Macros.artifact),
                    SetStatus Tapped ((Macros.It) {ok = Builtin.fst ok}) {ok = Builtin.snd ok}])
badModalReadsAcrossModes (_, Oh) impossible

public export
badReadsAfterModal : Unspellable (Instruction []) (\ok =>
  Sequentially [Macros.chooseModes (Macros.exactly 1) [Macros.destroy (Macros.target Macros.artifact), Macros.destroy (Macros.target Macros.enchantment)],
                SetStatus Tapped ((Macros.It) {ok = Builtin.fst ok}) {ok = Builtin.snd ok}])
badReadsAfterModal (_, Oh) impossible

||| "you pay 1 life"
public export
okMatchedPayer : Instruction []
okMatchedPayer =
  (May You (Pay You (Macros.payLife You 1) PaidOnce) Nothing
       (Just (Draw You (Lit 1))))

||| "you pay"
public export
badMismatchedPayer : Unspellable (Instruction []) (\ok =>
  (May You (Pay You (Macros.payLife Macros.anOpponent 1) PaidOnce {ag = ok}) Nothing (Just (Draw You (Lit 1)))))
badMismatchedPayer Oh impossible

||| "Choose new targets for target instant or sorcery spell."
public export
okRetargetStackSpell : Instruction []
okRetargetStackSpell =
  ChooseNewTargets (Macros.target (And [Macros.instantOrSorcery, Macros.spell]))

||| "Choose new targets for target creature."
public export
badRetargetPermanent : Unspellable (Instruction []) (\ok =>
  ChooseNewTargets (Macros.target Macros.creature) {cp = ok})
badRetargetPermanent StackSpell impossible

||| "Your opponents can't gain life."
public export
okStaticPlayerCant : Ability
okStaticPlayerCant =
  Static (Macros.playerCant "GainLife" (PlayerGroup YourOpponents))

||| "Target player can't gain life."
public export
badStaticPlayerCantTargets : Unspellable Ability (\ok =>
  Static (Macros.playerCant "GainLife" (Macros.target AnyPlayer)) {ut = ok})
badStaticPlayerCantTargets Oh impossible

||| "Target land becomes the basic land type of your choice until end of turn."
public export
okChosenBasicTypeOnLand : Instruction []
okChosenBasicTypeOnLand =
  Continuously (Becomes (Macros.target Macros.land) Sets
                        (ChosenQuality (OfYourChoice (SubtypeQ Land)
                                                     (Just BasicTypesOnly))))
               (Just Macros.untilEndOfTurn)

public export
badChosenBasicTypeOnCreature : Unspellable (Instruction []) (\ok =>
  Continuously (Becomes (Macros.target Macros.creature) Sets (ChosenQuality (OfYourChoice (SubtypeQ Land) (Just BasicTypesOnly)))
                                  {ok = ok})
               (Just Macros.untilEndOfTurn))
badChosenBasicTypeOnCreature Oh impossible

||| "Lands you control are Mountains."
public export
okLandsAreMountains : StaticSpec []
okLandsAreMountains =
  Becomes (Macros.allOf (And [Macros.land, HasPossessor ControllerAx You])) Sets
          (Bundle (MkToken Nothing [] (Macros.basicLandLine [landType "Mountain"]) [] Nothing) Nothing)

||| "Creatures are Mountains."
public export
badCreaturesAreMountains : Unspellable (StaticSpec []) (\ok =>
  Becomes (Macros.allOf Macros.creature) Sets (Bundle (MkToken Nothing [] (Macros.basicLandLine [landType "Mountain"]) [] Nothing) Nothing) {ok = ok})
badCreaturesAreMountains Oh impossible

||| "Choose a creature type other than Wall."
public export
okCreatureTypeExclusion : ChoiceDomain (QSort (SubtypeQ Creature))
okCreatureTypeExclusion = TypeOtherThan (creatureType "Wall")

||| "Choose a creature type other than Equipment."
public export
badNonCreatureTypeExclusion : Unspellable (ChoiceDomain (QSort (SubtypeQ Creature))) (\ok =>
  TypeOtherThan (artifactType "Equipment") {ct = ok})
badNonCreatureTypeExclusion Refl impossible

||| "Target land becomes every basic land type until end of turn."
public export
setsEveryBasicLandType : Instruction []
setsEveryBasicLandType =
  Continuously (Becomes (Macros.target Macros.land) Sets (EveryTypeOf BasicLandSpace))
               (Just Macros.untilEndOfTurn)

||| "Target creature loses the creature type of your choice until end of turn."
public export
losesChosenCreatureType : Instruction []
losesChosenCreatureType =
  Continuously (Becomes (Macros.target Macros.creature) Loses
                        (ChosenQuality (OfYourChoice (SubtypeQ Creature) Nothing)))
               (Just Macros.untilEndOfTurn)

||| "Target creature loses all colors until end of turn."
public export
losesAllColors : Instruction []
losesAllColors =
  Continuously (Becomes (Macros.target Macros.creature) Loses (Colored EveryColor))
               (Just Macros.untilEndOfTurn)

||| "Target creature is white."
public export
okAddsAColor : StaticSpec []
okAddsAColor =
  Becomes (Macros.target Macros.creature) Adds (Colored (SomeColors [White]))

public export
badAddsNoColor : Unspellable (StaticSpec []) (\ok =>
  Becomes (Macros.target Macros.creature) Adds (Colored (SomeColors [])) {ok = ok})
badAddsNoColor Oh impossible

||| "Target creature loses colorless until end of turn."
public export
badLosesNoColor : Unspellable (StaticSpec []) (\ok =>
  Becomes (Macros.target Macros.creature) Loses (Colored (SomeColors [])) {ok = ok})
badLosesNoColor Oh impossible

||| "Equipped permanent isn't a 2/2 creature."
public export
badLosesPt : Unspellable (StaticSpec []) (\ok =>
  Becomes (AttachHost Equipped PermanentW) Loses
          (Bundle (MkToken (Just (Lit 2 ** Lit 2)) [] (Macros.typesOnly [Creature]) [] Nothing)
                  Nothing) {ok = ok})
badLosesPt Oh impossible

public export
badStillOnAddition : Unspellable (StaticSpec []) (\ok =>
  Becomes (Macros.target Macros.creature) Adds
          (Bundle (MkToken Nothing [] (Macros.typesOnly [Artifact]) [] Nothing)
                  (Just Creature)) {ok = ok})
badStillOnAddition Oh impossible

public export
afterChoiceMade : Bindings
afterChoiceMade =
  instrIntro (the (Instruction [])
    (Macros.choose (Macros.counted (Macros.upTo 1) Macros.creature)))

||| "Choose up to one creature. Destroy the rest."
public export
okChoiceRestStands : Noun Choice.afterChoiceMade Object
okChoiceRestStands = Macros.theRest Object

public export
afterChoiceRestDisposed : Bindings
afterChoiceRestDisposed =
  instrIntro (the (Instruction [])
    (Sequentially [ Macros.choose (Macros.counted (Macros.upTo 1) Macros.creature)
                  , Macros.destroy (Macros.theRest Object) ]))

||| "Choose up to one creature. Destroy the rest. Destroy the rest."
public export
badChoiceRestDisposedTwice :
  Unspellable (Noun Choice.afterChoiceRestDisposed Object) (\ok => Macros.theRest Object {ok})
badChoiceRestDisposedTwice Oh impossible

||| "Each player chooses up to one creature they control, then sacrifices the
||| rest." The per-agent rest closes only the partitives the same distributive
||| chooser published [CR#700.8d], so nothing the table shares is spent.
public export
okDistributedRestOfOwnChoice : Instruction []
okDistributedRestOfOwnChoice =
  Sequentially
    [ Macros.chooses (Macros.each AnyPlayer)
        (Macros.counted (Macros.upTo 1)
           (And [Macros.creature, HasPossessor ControllerAx Macros.They]))
    , Macros.sacrifice (Macros.each AnyPlayer) (Macros.theRest Object) ]

public export
afterDistributedChoice : Bindings
afterDistributedChoice =
  instrIntro (the (Instruction [])
    (Macros.chooses (Macros.each AnyPlayer)
       (Macros.counted (Macros.upTo 1)
          (And [Macros.creature, HasPossessor ControllerAx Macros.They]))))

||| "Each player chooses up to one creature they control, then sacrifices the
||| rest."
public export
okDistributedRestStands : Noun Choice.afterDistributedChoice Object
okDistributedRestStands = Macros.theRest Object

public export
afterDistributedRestSacrificed : Bindings
afterDistributedRestSacrificed =
  instrIntro (the (Instruction []) Choice.okDistributedRestOfOwnChoice)

||| "Each player chooses up to one creature they control, then sacrifices the
||| rest, then sacrifices the rest." — the per-agent rest closes the partitives
||| it spent, so no rest stands to spend again.
public export
badDistributedRestDisposedTwice :
  Unspellable (Noun Choice.afterDistributedRestSacrificed Object)
              (\ok => Macros.theRest Object {ok})
badDistributedRestDisposedTwice Oh impossible

||| "Tap all creatures. Each player chooses up to one creature they control,
||| then sacrifices the rest." — the rest here spends the group the whole
||| table shares, and a player can't sacrifice a permanent they don't control
||| ([CR#701.21a]; `okDistributedRestOfOwnChoice` is the same deed with only
||| the chooser's own partitives standing).
||| "Tap all creatures. Each player chooses up to one creature they control."
public export
okSharedGroupWithoutARest : Instruction []
okSharedGroupWithoutARest =
  Sequentially
    [ Macros.tap (Macros.allOf Macros.creature)
    , Macros.chooses (Macros.each AnyPlayer)
        (Macros.counted (Macros.upTo 1)
           (And [Macros.creature, HasPossessor ControllerAx Macros.They])) ]

||| "For each player, choose target permanent that player controls. Those
||| players sacrifice those permanents." — the plural deed re-places only the
||| plural bindings the same loop published [CR#701.21a].
public export
okDistributedLoopParts : Instruction []
okDistributedLoopParts =
  Sequentially
    [ ForEachOf (Macros.each AnyPlayer)
        (Macros.choose (Macros.target (And [Permanent,
                                            HasPossessor ControllerAx Macros.They])))
    , Macros.sacrifice (Macros.Those PlayerW)
                       (Macros.Those PermanentW) ]

public export
badDistributedRestOfSharedGroup : Unspellable (Instruction []) (\ok =>
  Sequentially
    [ Macros.tap (Macros.allOf Macros.creature)
    , Macros.chooses (Macros.each AnyPlayer)
        (Macros.counted (Macros.upTo 1)
           (And [Macros.creature, HasPossessor ControllerAx Macros.They]))
    , Macros.sacrifice (Macros.each AnyPlayer) (Macros.theRest Object) {ke = ok} ])
badDistributedRestOfSharedGroup EachOnlyAdds impossible
badDistributedRestOfSharedGroup EachClosesOwnParts impossible

||| "Choose up to one creature. Each player sacrifices the rest." — one
||| chooser leaves one shared leftover, not a partition per player, so the
||| other players would sacrifice permanents they don't control [CR#701.21a].
public export
badDistributedRestOfSingularChoice : Unspellable (Instruction []) (\ok =>
  Sequentially
    [ Macros.choose (Macros.counted (Macros.upTo 1) Macros.creature)
    , Macros.sacrifice (Macros.each AnyPlayer) (Macros.theRest Object) {ke = ok} ])
badDistributedRestOfSingularChoice EachOnlyAdds impossible
badDistributedRestOfSingularChoice EachClosesOwnParts impossible

||| "an opponent who controls more lands than you control"
public export
okComparisonBoundOutsideTheMember : Predicate [] Player
okComparisonBoundOutsideTheMember =
  CompareOver Opponent
    (Macros.countOf (And [Macros.land, HasPossessor ControllerAx Macros.They]))
    Greater
    (Macros.countOf (And [Macros.land, HasPossessor ControllerAx You]))

||| "an opponent who controls more lands than they control"
public export
badMemberInComparisonBound : Unspellable (Predicate [] Player) (\ok =>
  CompareOver Opponent (Macros.countOf (And [Macros.land, HasPossessor ControllerAx You]))
              Greater (Macros.countOf (And [Macros.land, HasPossessor ControllerAx (They {ok})])))
badMemberInComparisonBound Refl impossible

||| "Target creature gets +3/+3 until end of turn."
public export
okGetsBattlefield : Instruction []
okGetsBattlefield =
  Macros.gets (Macros.target Macros.creature) (Up (Lit 3)) (Up (Lit 3))
              (Just Macros.untilEndOfTurn)

||| "Destroy target creature. It gets +3/+3 until end of turn."
public export
badGetsGraveyard : Unspellable (Instruction []) (\ok =>
  Sequentially [Macros.destroy (Macros.target Macros.creature),
                Macros.gets ((Macros.It)) (Up (Lit 3)) (Up (Lit 3)) (Just Macros.untilEndOfTurn) {ok}])
badGetsGraveyard Oh impossible

||| "Choose a creature. This deals 3 damage to each creature not chosen this
||| way."
public export
okNotChosenAfterOneChoice : Instruction []
okNotChosenAfterOneChoice =
  Sequentially
    [ Macros.chooses You (Macros.a Macros.creature)
    , DealDamage This (Lit 3) (Macros.each (And [Macros.creature, NotChosen])) ]

||| "This deals 3 damage to each creature not chosen this way."
||| Nothing was chosen, so "this way" reads back no choice
||| (`okNotChosenAfterOneChoice` is the same read with one standing).
public export
badNotChosenWithoutAChoice : Unspellable (Instruction []) (\ok =>
  DealDamage This (Lit 3)
             (Macros.each (And [Macros.creature, NotChosen {cs = ok}])))
badNotChosenWithoutAChoice ChoicesStand impossible

||| "Choose a creature. Choose a creature. This deals 3 damage to each
||| creature not chosen this way." — "this way" names the manner, so both
||| standing choices are excluded together [CR#700.8d].
public export
okNotChosenAfterTwoChoices : Instruction []
okNotChosenAfterTwoChoices =
  Sequentially
    [ Macros.chooses You (Macros.a Macros.creature)
    , Macros.chooses You (Macros.a Macros.creature)
    , DealDamage This (Lit 3)
                 (Macros.each (And [Macros.creature, NotChosen])) ]

||| "Choose a creature you control, then each opponent chooses a creature they
||| control. Destroy each creature not chosen this way." — Sculpted Sunburst's
||| exclusion, whose two standing choices have different choosers [CR#101.4].
public export
okNotChosenAcrossChoosers : Instruction []
okNotChosenAcrossChoosers =
  Sequentially
    [ Macros.choose (Macros.a Macros.creatureYouControl)
    , Macros.chooses (Macros.each Opponent)
        (Macros.a (And [Macros.creature, HasPossessor ControllerAx Macros.They]))
    , Macros.destroy (Macros.each (And [Macros.creature, NotChosen])) ]

||| "Each player chooses a creature they control." [CR#700.8d]
public export
okAgentScopedChoice : Instruction []
okAgentScopedChoice =
  Macros.chooses (Macros.each AnyPlayer)
    (Macros.a (And [Macros.creature, HasPossessor ControllerAx Macros.They]))

||| "Choose a creature they control." — the chooserless spelling has no
||| antecedent for "they" (`okAgentScopedChoice` is the same noun under a
||| chooser, which publishes one).
public export
badUnchooseredTheyControl : Unspellable (Instruction []) (\ok =>
  Macros.choose (Macros.a (And [Macros.creature,
                                HasPossessor ControllerAx (Macros.They {ok})])))
badUnchooseredTheyControl Refl impossible

||| "Starting with you, each player chooses a creature they control."
||| [CR#101.4]
public export
okChoiceStartingWithYou : Instruction []
okChoiceStartingWithYou =
  Choose (Just You) (Just (Macros.each AnyPlayer))
    (Macros.a (And [Macros.creature, HasPossessor ControllerAx Macros.They]))
    Openly Nothing

||| "Starting with you, target player chooses a creature." — one player makes
||| the choice, so there is no order for "starting with" to fix [CR#101.4]
||| (`okChoiceStartingWithYou` is the same order over a distributive chooser).
public export
badOrderedSingularChooser : Unspellable (Instruction []) (\ok =>
  Choose (Just You) (Just (Macros.target AnyPlayer)) (Macros.a Macros.creature)
         Openly Nothing {od = ok})
badOrderedSingularChooser Oh impossible

||| "Choose a creature. If you chose a creature this way, draw a card."
public export
okChoseThisWayAfterChoice : Instruction []
okChoseThisWayAfterChoice =
  Sequentially
    [ Macros.chooses You (Macros.a Macros.creature)
    , If (ChoseThisWay You Macros.creature) (Draw You (Lit 1)) Nothing ]

||| "If you chose a creature this way, draw a card." — nothing was chosen, so
||| "this way" reads back no choice (`okChoseThisWayAfterChoice` is the same
||| read with one standing).
public export
badChoseThisWayWithoutAChoice : Unspellable (Instruction []) (\ok =>
  If (ChoseThisWay You Macros.creature {cs = ok}) (Draw You (Lit 1)) Nothing)
badChoseThisWayWithoutAChoice ChoicesStand impossible

||| "Each player chooses a creature. Exile them."
public export
okDistributedChoiceReadsAsGroup : Instruction []
okDistributedChoiceReadsAsGroup =
  Sequentially
    [ Macros.chooses (Macros.each AnyPlayer) (Macros.a Macros.creature)
    , Macros.exile (Macros.Them) ]

||| "Each player chooses a creature. Exile it." — a distributive choice stands
||| as one per chooser, so the singular read has no antecedent
||| (`okDistributedChoiceReadsAsGroup` is the plural read).
public export
badDistributedChoiceReadSingular : Unspellable (Instruction []) (\ok =>
  Sequentially
    [ Macros.chooses (Macros.each AnyPlayer) (Macros.a Macros.creature)
    , Macros.exile ((Macros.It) {ok}) ])
badDistributedChoiceReadSingular Refl impossible

||| "Look at the top card of your library. You may put that card into your
||| graveyard." No printed card on the bench.
lookAtTopThenBin : Instruction []
lookAtTopThenBin =
  Sequentially [Macros.lookAt (Macros.topSlice (Lit 1)), Macros.may You (Macros.move (Macros.That CardW) Macros.graveyardZ)]

||| "Until end of turn, you may play lands and cast spells from your
||| graveyard." No printed card on the bench.
public export
playAndCastFromGraveyardThisTurn : Instruction []
playAndCastFromGraveyardThisTurn =
  Continuously
    (AndAlso Nothing [ (Macros.mayPlayDeed "Play" You (Macros.allOf Macros.land) Nothing (PlayRider (Just (Macros.graveyardOf You)) Nothing Nothing False ItsOwnCost))
             , (Macros.mayPlayDeed "Cast" You (Macros.allOf Macros.spell) Nothing (PlayRider (Just (Macros.graveyardOf You)) Nothing Nothing False ItsOwnCost)) ])
    (Just Macros.untilEndOfTurn)

||| "Each player may play an additional land on each of their turns."
||| No printed card on the bench.
public export
eachPlayerPlaysAdditionalLand : StaticSpec []
eachPlayerPlaysAdditionalLand =
  Macros.mayPlayAdditionalLands (Macros.each AnyPlayer) (Macros.exactly 1)

||| "Mill three cards. You may put an artifact card from among the cards
||| milled this way into your hand." No printed card on the bench.
public export
millThenPutFromAmongMilled : Instruction []
millThenPutFromAmongMilled =
  Sequentially [ Macros.mills You (Lit 3) You
               , Macros.may You
                   (Macros.move (Macros.fromAmong (Macros.exactly 1) Macros.artifact
                                   (Macros.TheVerbed "Mill" CardW ThisWay ManyOf))
                                Macros.handZ) ]

||| "Each player may shuffle their hand and graveyard into their library."
||| No printed card on the bench.
public export
eachPlayerMayShuffleTheirHandAndGraveyard : Instruction []
eachPlayerMayShuffleTheirHandAndGraveyard =
  Macros.may (Macros.each AnyPlayer)
    (Macros.shuffleInto They
       (Both (Macros.allOf (InZone (Macros.handOf They)))
               (Macros.allOf (InZone (Macros.graveyardOf They)))))

||| "Each player may discard their hand and draw seven cards."
||| No printed card on the bench.
public export
eachPlayerMayDiscardTheirHandAndDrawSeven : Instruction []
eachPlayerMayDiscardTheirHandAndDrawSeven =
  Macros.may (Macros.each AnyPlayer)
    (Sequentially [ Macros.discard They (Macros.allOf (InZone (Macros.handOf They)))
                  , Draw They (Lit 7) ])

||| "Each player may …" — the offer subject and the two counts its
||| context publishes. No printed card on the bench.
public export
eachPlayerOffered : Noun [] Player
eachPlayerOffered = Macros.each AnyPlayer

public export
eachPlayerOfferBindsOneMember :
  countOnes Player (mayCtx Choice.eachPlayerOffered) = 1
eachPlayerOfferBindsOneMember = Refl

public export
eachPlayerOfferDropsTheGroup :
  countManys Player (mayCtx Choice.eachPlayerOffered) = 0
eachPlayerOfferDropsTheGroup = Refl

||| "Choose target opponent who has more life than you do as you activate this
||| ability." — the rider times the announcement, not the comparison.
public export
okChoiceWithActivationRider : Instruction []
okChoiceWithActivationRider =
  Macros.chooseWhile
    (Macros.target (And [ Opponent
                        , Compare [PlayerStatAxis LifeTotal] Greater
                                  (PlayerStatOf LifeTotal You) ]))
    (WhileDoing (Activates You Macros.thisAbility))

||| The same choice ridered on a death, which is not an event a choice can be
||| made during.
public export
badChoiceRiderNotUnderway : Unspellable (Instruction []) (\ok =>
  Macros.chooseWhile
    (Macros.target (And [ Opponent
                        , Compare [PlayerStatAxis LifeTotal] Greater
                                  (PlayerStatOf LifeTotal You) ]))
    (WhileDoing (Dies Macros.thisCreature) {up = ok}))
badChoiceRiderNotUnderway Oh impossible

||| "Each player secretly chooses a number. Then those numbers are revealed.
||| Each player who chose the highest number loses that much life."
||| (Menacing Ogre) — not a vote [CR#701.38c]; the gate is the plural choice.
public export
okChoseExtremeAfterNumbers : Instruction []
okChoseExtremeAfterNumbers =
  Sequentially
    [ Macros.secretlyChooses (Macros.each AnyPlayer) (Macros.a (Macros.quality Number))
    , ChoicesRevealed HiddenNumbers
    , Macros.losesLife (Macros.each (And [AnyPlayer, ChoseExtreme MaxOf])) ThatMuch ]

||| The same read with no number chosen anywhere in the text.
public export
badChoseExtremeWithoutChoice : Unspellable (Instruction []) (\ok =>
  Macros.losesLife (Macros.each (And [AnyPlayer, ChoseExtreme MaxOf {nc = ok}]))
                   (Lit 1))
badChoseExtremeWithoutChoice Oh impossible
