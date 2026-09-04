module Experimental.ProofsChoice

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
  Sequentially [Choose Nothing (Macros.target Macros.creature) Openly,
                Macros.gainsLife You (StatOf Power (Macros.It OneOf))]

||| "Choose two target creatures. You gain life equal to their power."
public export
badGroupPower : Unspellable (Instruction []) (\ok =>
  Sequentially [Choose Nothing (Described (TargetDet (Macros.exactly 2)) Macros.creature) Openly,
                Macros.gainsLife You (StatOf Power ((Macros.It ManyOf)) {one = ok})])
badGroupPower Refl impossible

||| "Choose target creature. Its owner loses 1 life."
public export
okSingleOwner : Instruction []
okSingleOwner =
  Sequentially [Choose Nothing (Macros.target Macros.creature) Openly,
                Macros.losesLife (Macros.ownerOf (Macros.It OneOf)) (Lit 1)]

||| "Choose two target creatures. Their owners each lose 1 life."
public export
okGroupOwners : Instruction []
okGroupOwners =
  Sequentially [Choose Nothing (Described (TargetDet (Macros.exactly 2)) Macros.creature) Openly,
                Macros.losesLife (Macros.ownerOf (Macros.It ManyOf)) (Lit 1)]

||| "Choose target creature."
public export
okTargetCreature : Instruction []
okTargetCreature = Choose Nothing (Macros.target Macros.creature) Openly

||| "Choose target color."
public export
badTargetColor : Unspellable (Instruction []) (\ok =>
  Choose Nothing (Macros.target (QualityNoun Color Nothing) {tk = ok}) Openly)
badTargetColor ObjectTgt impossible

||| "Choose two target creatures."
public export
okTwoGroup : Instruction []
okTwoGroup =
  Choose Nothing (Described (TargetDet (Macros.exactly 2)) Macros.creature) Openly

||| "Choose zero target creatures."
public export
badZeroGroup : Unspellable (Instruction []) (\ok =>
  Choose Nothing (Described (TargetDet (Macros.exactly 0)) Macros.creature {ok}) Openly)
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
                    SetStatus Tapped ((Macros.It OneOf) {ok = Builtin.fst ok}) {ok = Builtin.snd ok}])
badModalReadsAcrossModes (_, Oh) impossible

public export
badReadsAfterModal : Unspellable (Instruction []) (\ok =>
  Sequentially [Macros.chooseModes (Macros.exactly 1) [Macros.destroy (Macros.target Macros.artifact), Macros.destroy (Macros.target Macros.enchantment)],
                SetStatus Tapped ((Macros.It OneOf) {ok = Builtin.fst ok}) {ok = Builtin.snd ok}])
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

public export
badChosenBasicTypeOnCreature : Unspellable (Instruction []) (\ok =>
  Continuously (Becomes (Macros.target Macros.creature) Sets (ChosenQuality (OfYourChoice (SubtypeQ Land) (Just BasicTypesOnly)))
                                  {ok = ok})
               (Just Macros.untilEndOfTurn))
badChosenBasicTypeOnCreature Oh impossible

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
afterChoiceRestDisposed : Bindings
afterChoiceRestDisposed =
  instrIntro (the (Instruction [])
    (Sequentially [ Macros.choose (Macros.counted (Macros.upTo 1) Macros.creature)
                  , Macros.destroy (Macros.theRest Object) ]))

||| "Choose up to one creature. Destroy the rest. Destroy the rest."
public export
badChoiceRestDisposedTwice :
  Unspellable (Noun ProofsChoice.afterChoiceRestDisposed Object) (\ok => Macros.theRest Object {ok})
badChoiceRestDisposedTwice Oh impossible

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
  Macros.gets (Macros.target Macros.creature) (PtUp (Lit 3)) (PtUp (Lit 3))
              (Just Macros.untilEndOfTurn)

||| "Destroy target creature. It gets +3/+3 until end of turn."
public export
badGetsGraveyard : Unspellable (Instruction []) (\ok =>
  Sequentially [Macros.destroy (Macros.target Macros.creature),
                Macros.gets ((Macros.It OneOf)) (PtUp (Lit 3)) (PtUp (Lit 3)) (Just Macros.untilEndOfTurn) {ok}])
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
badNotChosenWithoutAChoice OneChoiceStands impossible

||| "Choose a creature. Choose a creature. This deals 3 damage to each
||| creature not chosen this way." — two choices stand, so the exclusion has
||| no unique antecedent.
public export
badNotChosenAfterTwoChoices : Unspellable (Instruction []) (\ok =>
  Sequentially
    [ Macros.chooses You (Macros.a Macros.creature)
    , Macros.chooses You (Macros.a Macros.creature)
    , DealDamage This (Lit 3)
                 (Macros.each (And [Macros.creature, NotChosen {cs = ok}])) ])
badNotChosenAfterTwoChoices OneChoiceStands impossible

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

||| "Each player chooses a creature. Exile them."
public export
okDistributedChoiceReadsAsGroup : Instruction []
okDistributedChoiceReadsAsGroup =
  Sequentially
    [ Macros.chooses (Macros.each AnyPlayer) (Macros.a Macros.creature)
    , Macros.exile You (Macros.It ManyOf) ]

||| "Each player chooses a creature. Exile it." — a distributive choice stands
||| as one per chooser, so the singular read has no antecedent
||| (`okDistributedChoiceReadsAsGroup` is the plural read).
public export
badDistributedChoiceReadSingular : Unspellable (Instruction []) (\ok =>
  Sequentially
    [ Macros.chooses (Macros.each AnyPlayer) (Macros.a Macros.creature)
    , Macros.exile You ((Macros.It OneOf) {ok}) ])
badDistributedChoiceReadSingular Refl impossible
