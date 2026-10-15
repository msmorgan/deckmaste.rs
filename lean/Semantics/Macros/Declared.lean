-- Ported from `lean/archive/SemanticsV2/Macros/Declared.lean` onto the `semantics-v3` AST and `DRS` checker.
import Semantics.Macros

set_option linter.unusedVariables false

namespace Semantics.Macros.Declared

/-- `abilities/abilityGrant.ron` -/
semantic_template abilityGrant (subject : Semantics.NounPhrase) (ability : Semantics.Ability) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.abilityGrant subject ability)

/-- `abilities/abilityGrantFrom.ron` -/
semantic_template abilityGrantFrom (subject : Semantics.NounPhrase) (source : Semantics.NounPhrase) (classes : (List Semantics.AbilityClass) := []) (except : (Option Semantics.Predicate) := none) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.abilityGrantFrom subject classes source except)

/-- `abilities/abilityGrantSelected.ron` -/
semantic_template abilityGrantSelected (subject : Semantics.NounPhrase) (source : Semantics.NounPhrase) (selection : Semantics.AbilitySelection) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.abilityGrantSelected subject source selection)

/-- `abilities/abilityWord.ron` -/
semantic_template abilityWord (word : String) (ability : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord word) ability)

/-- `abilities/activatedBy.ron` -/
semantic_template Ability.activatedBy (cost : Semantics.Cost) (instruction : Semantics.Instruction) (who : Semantics.NounPhrase) : Semantics.Ability :=
  (Semantics.Ability.activated cost instruction none none none (some who))

/-- `abilities/activatedOnlyDuring.ron` -/
semantic_template activatedOnlyDuring (cost : Semantics.Cost) (instruction : Semantics.Instruction) (timing : Semantics.Timing) : Semantics.Ability :=
  (Semantics.Ability.activated cost instruction (some timing) none none none)

/-- `abilities/activatedOnlyIf.ron` -/
semantic_template activatedOnlyIf (cost : Semantics.Cost) (instruction : Semantics.Instruction) (guard : Semantics.Condition) : Semantics.Ability :=
  (Semantics.Ability.activated cost instruction none none (some guard) none)

/-- `abilities/activatedOnlyOnce.ron` -/
semantic_template activatedOnlyOnce (cost : Semantics.Cost) (instruction : Semantics.Instruction) (limit : Semantics.UsageLimit) : Semantics.Ability :=
  (Semantics.Ability.activated cost instruction none (some limit) none none)

/-- `abilities/activatedOnlyOnceIf.ron` -/
semantic_template activatedOnlyOnceIf (cost : Semantics.Cost) (instruction : Semantics.Instruction) (limit : Semantics.UsageLimit) (guard : Semantics.Condition) : Semantics.Ability :=
  (Semantics.Ability.activated cost instruction none (some limit) (some guard) none)

/-- `abilities/addSubtype.ron` -/
semantic_template addSubtype (subject : Semantics.NounPhrase) (subtype : Semantics.Subtype) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.characteristicChange subject [(Semantics.CharacteristicEdit.typeLine Semantics.QualityOp.adds ({ supertypes := none, types := none, subtypes := (some [subtype]), retained := none } : Semantics.TypeLineChanges))])

/-- `abilities/addedCost.ron` -/
semantic_template addedCost (cost : Semantics.Cost) (offered : Bool) (times : Semantics.PayTimes := Semantics.PayTimes.once) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.addedCost cost offered times)

/-- `abilities/additionalTriggers.ron` -/
semantic_template additionalTriggers (event : Semantics.GameEvent) (quantity : Semantics.Quantity) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.additionalTriggers event quantity)

/-- `abilities/addsTarget.ron` -/
semantic_template addsTarget (subject : Semantics.NounPhrase) (description : Semantics.Predicate) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.addsTarget subject description)

/-- `abilities/allInstead.ron` -/
semantic_template allInstead (cost : (Option Semantics.Cost) := none) (condition : (Option Semantics.Condition) := none) : Semantics.ModeChange :=
  (Semantics.ModeChange.allInstead cost condition)

/-- `abilities/alsoForKeywords.ron` -/
semantic_template alsoForKeywords (ability : Semantics.Ability) (keywords : (List Semantics.KeywordTerm) := []) : Semantics.Ability :=
  (Semantics.Ability.alsoForKeywords ability keywords)

/-- `abilities/altCost.ron` -/
semantic_template altCost (subject : Semantics.NounPhrase) (cost : (Option Semantics.Cost) := none) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.altCost subject cost)

/-- `abilities/asEvent.ron` -/
semantic_template asEvent (event : Semantics.GameEvent) (does : Semantics.Instruction) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.asEvent event does)

/-- `abilities/asInstant.ron` -/
semantic_template asInstant : Semantics.Timing :=
  (Semantics.Timing.onlyIf (Semantics.Condition.hasPriority Semantics.NounPhrase.you))

/-- `abilities/asSorcery.ron` -/
semantic_template asSorcery : Semantics.Timing :=
  (Semantics.Timing.and [(Semantics.Timing.inPart Semantics.TurnPart.mainPhase (some Semantics.NounPhrase.you) Semantics.TurnRef.current), (Semantics.Timing.onlyIf (Semantics.Condition.not (Semantics.Condition.exists_ (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.stack Semantics.ZoneScope.bare))))))])

/-- `abilities/asThose.ron` -/
semantic_template asThose : Semantics.TokenSpec :=
  Semantics.TokenSpec.asThose

/-- `abilities/beforePart.ron` -/
semantic_template beforePart (part : Semantics.TurnPart) (whose : (Option Semantics.NounPhrase) := none) : Semantics.Timing :=
  (Semantics.Timing.relativeToEvent Semantics.EventSide.before (Semantics.GameEvent.beginningOf part whose Semantics.TurnRef.current))

/-- `abilities/cantBeAttachedTo.ron` -/
semantic_template cantBeAttachedTo (subject : Semantics.NounPhrase) (host : Semantics.NounPhrase) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.cant Semantics.Subject.any (Semantics.Instruction.attachment Semantics.AttachMove.attached subject (some host)) ({ bound := none, asThough := none, rider := Semantics.DeonticRider.noRider, except := none } : Semantics.DeonticMods))

/-- `abilities/cantBeBlockedBy.ron` -/
semantic_template cantBeBlockedBy (subject : Semantics.NounPhrase) (blockers : Semantics.Predicate) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.cant (Semantics.Subject.by (Semantics.NounPhrase.described Semantics.DetPhrase.all blockers)) (Semantics.Macros.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.block) subject) ({ bound := none, asThough := none, rider := Semantics.DeonticRider.noRider, except := none } : Semantics.DeonticMods))

/-- `abilities/cantBeBlockedExceptBy.ron` -/
semantic_template cantBeBlockedExceptBy (subject : Semantics.NounPhrase) (blockers : Semantics.Predicate) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.cant Semantics.Subject.any (Semantics.Macros.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.block) subject) ({ bound := none, asThough := none, rider := Semantics.DeonticRider.noRider, except := (some (Semantics.Cause.byObjects blockers)) } : Semantics.DeonticMods))

/-- `abilities/cantExcept.ron` -/
semantic_template cantExcept (deed : Semantics.Deed) (subject : Semantics.NounPhrase) (cause : Semantics.Cause) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.cant Semantics.Subject.any (Semantics.Macros.deonticActionFor deed subject) ({ bound := none, asThough := none, rider := Semantics.DeonticRider.noRider, except := (some cause) } : Semantics.DeonticMods))

/-- `abilities/characteristicChange.ron` -/
semantic_template characteristicChange (subject : Semantics.NounPhrase) (edits : (List Semantics.CharacteristicEdit) := []) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.characteristicChange subject edits)

/-- `abilities/choice.ron` -/
semantic_template choice (occasion : Semantics.ChoiceOccasion) (subject : Semantics.NounPhrase) (sort : Semantics.ChoiceSort) (disclosure : Semantics.Disclosure) (domain : (Option Semantics.ChoiceDomain) := none) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.choice occasion subject sort domain disclosure)

/-- `abilities/compound.ron` -/
semantic_template compound (costs : (List Semantics.Cost) := []) : Semantics.Cost :=
  (Semantics.Cost.compound costs)

/-- `abilities/conditional.ron` -/
semantic_template conditional (spec : Semantics.StaticSpec) (condition : Semantics.Condition) (marking : Semantics.CondMarking) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.conditional spec condition marking)

/-- `abilities/conjunction.ron` -/
semantic_template conjunction (subject : (Option Semantics.NounPhrase) := none) (parts : (List Semantics.StaticSpec) := []) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.conjunction subject parts)

/-- `abilities/controlGrant.ron` -/
semantic_template controlGrant (player : Semantics.NounPhrase) (subject : Semantics.NounPhrase) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.controlGrant player subject)

/-- `abilities/copyChange.ron` -/
semantic_template copyChange (subject : Semantics.NounPhrase) (source : Semantics.NounPhrase) (exceptions : (List Semantics.CopyExcept) := []) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.copyChange subject source exceptions)

/-- `abilities/copyOf.ron` -/
semantic_template copyOf (source : Semantics.NounPhrase) (exceptions : (List Semantics.CopyExcept) := []) : Semantics.TokenSpec :=
  (Semantics.TokenSpec.copyOf source exceptions)

/-- `abilities/costShift.ron` -/
semantic_template costShift (subject : Semantics.NounPhrase) (shift : Semantics.CostShift) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.costShift subject shift)

/-- `abilities/damageRule.ron` -/
semantic_template damageRule (kind : Semantics.DamageKind) (source : Semantics.DamageAgent) (scope : Semantics.DamageScope) (op : Semantics.DamageOp) (use : Semantics.ReplUse) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.damageRule kind source scope op use)

/-- `abilities/duringEachOfYourTurns.ron` -/
semantic_template duringEachOfYourTurns : Semantics.Timing :=
  (Semantics.Timing.inPart Semantics.TurnPart.turn (some Semantics.NounPhrase.you) (Semantics.TurnRef.each none))

/-- `abilities/duringPart.ron` -/
semantic_template Timing.duringPart (part : Semantics.TurnPart) (whose : (Option Semantics.NounPhrase) := none) : Semantics.Timing :=
  (Semantics.Timing.inPart part whose Semantics.TurnRef.current)

/-- `abilities/entersTapped.ron` -/
semantic_template entersTapped (subject : Semantics.NounPhrase) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.entryRider subject (Semantics.EntryRider.entersAs Semantics.Status.tapped))

/-- `abilities/entersWithAdditionalCounters.ron` -/
semantic_template entersWithAdditionalCounters (subject : Semantics.NounPhrase) (amount : Semantics.Amount) (kind : Semantics.CounterKind) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.entryRider subject (Semantics.EntryRider.withCounters amount (Semantics.CounterKindSource.printed kind) Semantics.EntryCounterMark.additional))

/-- `abilities/entersWithCounters.ron` -/
semantic_template entersWithCounters (subject : Semantics.NounPhrase) (amount : Semantics.Amount) (kind : Semantics.CounterKind) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.entryRider subject (Semantics.EntryRider.withCounters amount (Semantics.CounterKindSource.printed kind) Semantics.EntryCounterMark.fresh))

/-- `abilities/entersWithFewerCounters.ron` -/
semantic_template entersWithFewerCounters (subject : Semantics.NounPhrase) (amount : Semantics.Amount) (kind : Semantics.CounterKind) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.entryRider subject (Semantics.EntryRider.withCounters amount (Semantics.CounterKindSource.printed kind) Semantics.EntryCounterMark.fewer))

/-- `abilities/entryRider.ron` -/
semantic_template entryRider (subject : Semantics.NounPhrase) (rider : Semantics.EntryRider) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.entryRider subject rider)

/-- `abilities/extraModesCost.ron` -/
semantic_template extraModesCost (cost : Semantics.Cost) : Semantics.ModeChange :=
  (Semantics.ModeChange.extraModesCost cost)

/-- `abilities/flavorWord.ron` -/
semantic_template flavorWord (word : String) (ability : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.flavorWord word) ability)

/-- `abilities/flyingExpansion.ron` -/
semantic_template flyingExpansion : Semantics.Ability :=
  (Semantics.Ability.static (Semantics.StaticSpec.cant Semantics.Subject.any (Semantics.Macros.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.block) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) ({ bound := none, asThough := none, rider := Semantics.DeonticRider.noRider, except := (some (Semantics.Cause.byObjects (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.or [(Semantics.Predicate.hasKeyword (Semantics.KeywordTerm.the (Semantics.KeywordLabel.named "flying"))), (Semantics.Predicate.hasKeyword (Semantics.KeywordTerm.the (Semantics.KeywordLabel.named "reach")))])]))) } : Semantics.DeonticMods)))

/-- `abilities/insteadOf.ron` -/
semantic_template insteadOf (event : Semantics.WouldClause) (replacement : Semantics.Instruction) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.instead event replacement none Semantics.ReplUse.repeatedly none)

/-- `abilities/italicHead.ron` -/
semantic_template italicHead (word : Semantics.ItalicWord) (ability : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead word ability)

/-- `abilities/itsManaCost.ron` -/
semantic_template itsManaCost : Semantics.Cost :=
  Semantics.Cost.itsManaCost

/-- `abilities/keyword.ron` -/
semantic_template keyword (label : Semantics.KeywordLabel) : Semantics.Ability :=
  (Semantics.Ability.keyword label [] [])

/-- `abilities/keywordCosting.ron` -/
semantic_template keywordCosting (label : Semantics.KeywordLabel) (cost : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword label [(Semantics.KeywordParam.cost cost)] [])

/-- `abilities/keywordNumber.ron` -/
semantic_template keywordNumber (label : Semantics.KeywordLabel) (amount : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword label [(Semantics.KeywordParam.number amount)] [])

/-- `abilities/keywordNumberCosting.ron` -/
semantic_template keywordNumberCosting (label : Semantics.KeywordLabel) (amount : Semantics.Amount) (cost : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword label [(Semantics.KeywordParam.number amount), (Semantics.KeywordParam.cost cost)] [])

/-- `abilities/keywordQuality.ron` -/
semantic_template keywordQuality (label : Semantics.KeywordLabel) (quality : Semantics.Predicate) : Semantics.Ability :=
  (Semantics.Ability.keyword label [(Semantics.KeywordParam.quality quality)] [])

/-- `abilities/keywordQualityCosting.ron` -/
semantic_template keywordQualityCosting (label : Semantics.KeywordLabel) (quality : Semantics.Predicate) (cost : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword label [(Semantics.KeywordParam.quality quality), (Semantics.KeywordParam.cost cost)] [])

/-- `abilities/keywordSubject.ron` -/
semantic_template keywordSubject (label : Semantics.KeywordLabel) (subject : Semantics.Predicate) : Semantics.Ability :=
  (Semantics.Ability.keyword label [(Semantics.KeywordParam.subject subject)] [])

/-- `abilities/letterDefinition.ron` -/
semantic_template letterDefinition (letter : Semantics.Letter) (amount : Semantics.Amount) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.letterDefinition letter amount)

/-- `abilities/loyaltySymbol.ron` -/
semantic_template loyaltySymbol (loyalty : Semantics.LoyaltyCost) : Semantics.Cost :=
  (Semantics.Cost.loyaltySymbol loyalty)

/-- `abilities/mana.ron` -/
semantic_template mana (cost : (List Semantics.ManaSymbol)) : Semantics.Cost :=
  (Semantics.Cost.mana cost)

/-- `abilities/manaCostOf.ron` -/
semantic_template manaCostOf (object : Semantics.NounPhrase) : Semantics.ManaCostTerm :=
  (Semantics.ManaCostTerm.manaCostOf object)

/-- `abilities/manaRetention.ron` -/
semantic_template manaRetention (player : Semantics.NounPhrase) (mana : Semantics.ManaHeld) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.manaRetention player mana)

/-- `abilities/mayBeAttachedTo.ron` -/
semantic_template mayBeAttachedTo (subject : Semantics.NounPhrase) (host : Semantics.NounPhrase) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.may Semantics.Subject.any (Semantics.Instruction.attachment Semantics.AttachMove.attached subject (some host)) ({ bound := none, asThough := none, rider := Semantics.DeonticRider.noRider, except := none } : Semantics.DeonticMods))

/-- `abilities/mayBeginOnBattlefield.ron` -/
semantic_template mayBeginOnBattlefield : Semantics.Ability :=
  Semantics.Ability.mayBeginOnBattlefield

/-- `abilities/mayCastFor.ron` -/
semantic_template mayCastFor (player : Semantics.NounPhrase) (cost : Semantics.Cost) (from_ : (Option Semantics.ZoneExpr) := none) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.may (Semantics.Subject.actor player) (Semantics.Macros.deonticActionFor (Semantics.Deed.action "cast") Semantics.NounPhrase.this) ({ bound := none, asThough := none, rider := (Semantics.DeonticRider.play from_ none none false (Semantics.PlayPayment.payingInstead cost)), except := none } : Semantics.DeonticMods))

/-- `abilities/mayCastFrom.ron` -/
semantic_template mayCastFrom (what : Semantics.NounPhrase) (zone : Semantics.ZoneExpr) (paying : Semantics.PlayPayment := Semantics.PlayPayment.itsOwnCost) (exclusive : Bool := false) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.may (Semantics.Subject.actor Semantics.NounPhrase.actor) (Semantics.Macros.deonticActionFor (Semantics.Deed.action "cast") what) ({ bound := none, asThough := none, rider := (Semantics.DeonticRider.play (some zone) none none exclusive paying), except := none } : Semantics.DeonticMods))

/-- `abilities/modeChoice.ron` -/
semantic_template modeChoice (change : Semantics.ModeChange) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.modeChoice change)

/-- `abilities/modification.ron` -/
semantic_template modification (subject : Semantics.NounPhrase) (stat : Semantics.Stat) (delta : (Semantics.Delta Semantics.Amount)) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.modification subject stat delta)

/-- `abilities/notChosenBefore.ron` -/
semantic_template notChosenBefore (within : Semantics.Duration) : Semantics.ModeChange :=
  (Semantics.ModeChange.notChosenBefore within)

/-- `abilities/onceEachYourTurn.ron` -/
semantic_template onceEachYourTurn : Semantics.UsageLimit :=
  (Semantics.UsageLimit.mk (Semantics.CountBound.moreThan (Semantics.Amount.lit 1)) Semantics.Period.yourTurn)

/-- `abilities/oncePerGame.ron` -/
semantic_template oncePerGame : Semantics.UsageLimit :=
  (Semantics.UsageLimit.mk (Semantics.CountBound.moreThan (Semantics.Amount.lit 1)) Semantics.Period.game)

/-- `abilities/oncePerObject.ron` -/
semantic_template oncePerObject : Semantics.UsageLimit :=
  (Semantics.UsageLimit.mk (Semantics.CountBound.moreThan (Semantics.Amount.lit 1)) Semantics.Period.thisObject)

/-- `abilities/oncePerTurn.ron` -/
semantic_template oncePerTurn : Semantics.UsageLimit :=
  (Semantics.UsageLimit.mk (Semantics.CountBound.moreThan (Semantics.Amount.lit 1)) Semantics.Period.turn)

/-- `abilities/or.ron` -/
semantic_template Cost.or (costs : (List Semantics.Cost) := []) : Semantics.Cost :=
  (Semantics.Cost.or costs)

/-- `abilities/partSkip.ron` -/
semantic_template partSkip (player : Semantics.NounPhrase) (part : Semantics.TurnPart) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.partSkip player part)

/-- `abilities/payLife.ron` -/
semantic_template payLife (amount : Nat) : Semantics.Cost :=
  (Semantics.Cost.perform (Semantics.Instruction.changeLife (Semantics.Delta.down (Semantics.Amount.lit amount))))

/-- `abilities/paySubstitute.ron` -/
semantic_template paySubstitute (subject : Semantics.NounPhrase) (substitute : Semantics.ManaSubstitute) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.paySubstitute subject substitute)

/-- `abilities/perform.ron` -/
semantic_template perform (instruction : Semantics.Instruction) : Semantics.Cost :=
  (Semantics.Cost.perform instruction)

/-- `abilities/preventionBan.ron` -/
semantic_template preventionBan (kind : Semantics.DamageKind) (damage : Semantics.Unpreventable) (ban : Semantics.PreventionBan) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.preventionBan kind damage ban)

/-- `abilities/ptDefinition.ron` -/
semantic_template ptDefinition (subject : Semantics.NounPhrase) (slots : Semantics.DefinedSlots) (amount : Semantics.Amount) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.ptDefinition subject slots amount)

/-- `abilities/ptModification.ron` -/
semantic_template ptModification (subject : Semantics.NounPhrase) (power : (Semantics.Delta Semantics.Amount)) (toughness : (Semantics.Delta Semantics.Amount)) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.ptModification subject power toughness)

/-- `abilities/ptSwitch.ron` -/
semantic_template ptSwitch (subject : Semantics.NounPhrase) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.ptSwitch subject)

/-- `abilities/qualified.ron` -/
semantic_template qualified (ability : Semantics.Ability) (timing : (Option Semantics.Timing) := none) (limit : (Option Semantics.UsageLimit) := none) (guard : (Option Semantics.Condition) := none) (from_ : (Option Semantics.Zone) := none) : Semantics.Ability :=
  (Semantics.Ability.qualified ability timing limit guard from_)

/-- `abilities/reducedBySymbols.ron` -/
semantic_template reducedBySymbols (subject : Semantics.NounPhrase) (cost : Semantics.ManaCostTerm) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.costShift subject (Semantics.CostShift.bySymbols cost false false))

/-- `abilities/repeatable.ron` -/
semantic_template repeatable : Semantics.ModeChange :=
  Semantics.ModeChange.repeatable

/-- `abilities/replacement.ron` -/
semantic_template replacement (event : Semantics.WouldClause) (replacement : Semantics.Instruction) (use : Semantics.ReplUse) (timing : (Option Semantics.Timing) := none) (limit : (Option Semantics.UsageLimit) := none) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.instead event replacement timing use limit)

/-- `abilities/retention.ron` -/
semantic_template retention (spec : Semantics.StaticSpec) (subject : Semantics.NounPhrase) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.retention spec subject)

/-- `abilities/scaled.ron` -/
semantic_template scaled (cost : Semantics.Cost) (amount : Semantics.Amount) : Semantics.Cost :=
  (Semantics.Cost.scaled cost amount)

/-- `abilities/setQuantity.ron` -/
semantic_template setQuantity (quantity : Semantics.Quantity) : Semantics.ModeChange :=
  (Semantics.ModeChange.setQuantity quantity)

/-- `abilities/specialAction.ron` -/
semantic_template specialAction (when : Semantics.Timing) (cost : Semantics.Cost) (does : Semantics.Instruction) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.specialAction (some when) cost does)

/-- `abilities/spell.ron` -/
semantic_template Ability.spell (instruction : Semantics.Instruction) (timing : (Option Semantics.Timing) := none) : Semantics.Ability :=
  (Semantics.Ability.spell timing instruction)

/-- `abilities/static.ron` -/
semantic_template static (spec : Semantics.StaticSpec) : Semantics.Ability :=
  (Semantics.Ability.static spec)

/-- `abilities/substitute.ron` -/
semantic_template substitute (action : Semantics.Instruction) (pays : (List Semantics.PipPaid)) : Semantics.ManaSubstitute :=
  ({ action := action, pays := pays } : Semantics.ManaSubstitute)

/-- `abilities/tapSymbol.ron` -/
semantic_template tapSymbol : Semantics.Cost :=
  Semantics.Cost.tapSymbol

/-- `abilities/textChange.ron` -/
semantic_template textChange (subject : Semantics.NounPhrase) (change : Semantics.TextChange) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.textChange subject change)

/-- `abilities/thatAbility.ron` -/
semantic_template thatAbility (ref : Semantics.ChoiceRef) : Semantics.Ability :=
  (Semantics.Ability.thatAbility ref)

/-- `abilities/triggered.ron` -/
semantic_template triggered (event : Semantics.GameEvent) (instruction : Semantics.Instruction) : Semantics.Ability :=
  (Semantics.Ability.triggered event [] none [] none none none instruction)

/-- `abilities/triggeredIf.ron` -/
semantic_template triggeredIf (event : Semantics.GameEvent) (condition : Semantics.Condition) (instruction : Semantics.Instruction) : Semantics.Ability :=
  (Semantics.Ability.triggered event [] none [] none none (some condition) instruction)

/-- `abilities/triggeredOnlyDuring.ron` -/
semantic_template triggeredOnlyDuring (event : Semantics.GameEvent) (timing : Semantics.Timing) (instruction : Semantics.Instruction) : Semantics.Ability :=
  (Semantics.Ability.triggered event [] none [] (some timing) none none instruction)

/-- `abilities/triggeredOnlyOnce.ron` -/
semantic_template triggeredOnlyOnce (event : Semantics.GameEvent) (limit : Semantics.UsageLimit) (instruction : Semantics.Instruction) : Semantics.Ability :=
  (Semantics.Ability.triggered event [] none [] none (some limit) none instruction)

/-- `abilities/triggeredOr.ron` -/
semantic_template triggeredOr (event : Semantics.GameEvent) (alternatives : (List Semantics.GameEvent)) (instruction : Semantics.Instruction) : Semantics.Ability :=
  (Semantics.Ability.triggered event alternatives none [] none none none instruction)

/-- `abilities/triggeredOrOnlyOnce.ron` -/
semantic_template triggeredOrOnlyOnce (event : Semantics.GameEvent) (alternatives : (List Semantics.GameEvent)) (limit : Semantics.UsageLimit) (instruction : Semantics.Instruction) : Semantics.Ability :=
  (Semantics.Ability.triggered event alternatives none [] none (some limit) none instruction)

/-- `abilities/triggeredWhile.ron` -/
semantic_template triggeredWhile (event : Semantics.GameEvent) (while_ : Semantics.Timing) (instruction : Semantics.Instruction) : Semantics.Ability :=
  (Semantics.Ability.triggered event [] (some while_) [] none none none instruction)

/-- `abilities/untapSymbol.ron` -/
semantic_template untapSymbol : Semantics.Cost :=
  Semantics.Cost.untapSymbol

/-- `abilities/visibility.ron` -/
semantic_template visibility (verb : Semantics.ExposeVerb) (player : Semantics.NounPhrase) (what : Semantics.VisibleThing) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.visibility verb player what)

/-- `abilities/whileDoing.ron` -/
semantic_template whileDoing (event : Semantics.GameEvent) : Semantics.Timing :=
  (Semantics.Timing.whileDoing event)

/-- `abilities/whileHolds.ron` -/
semantic_template whileHolds (condition : Semantics.Condition) (does : Semantics.Instruction) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.whileHolds condition does)

/-- `abilities/whileSearchingYourLibrary.ron` -/
semantic_template whileSearchingYourLibrary : Semantics.Timing :=
  (Semantics.Timing.whileDoing (Semantics.GameEvent.does (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Instruction.enact (Semantics.Deed.action "search") (Semantics.Macros.deonticActionFor (Semantics.Deed.action "search") Semantics.NounPhrase.any) none (some (Semantics.ZoneExpr.zone Semantics.Zone.library (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.you))))))

/-- `abilities/whileTrue.ron` -/
semantic_template whileTrue (condition : Semantics.Condition) : Semantics.Timing :=
  (Semantics.Timing.onlyIf condition)

/-- `abilities/withSubstitute.ron` -/
semantic_template withSubstitute (cost : Semantics.Cost) (substitute : Semantics.ManaSubstitute) : Semantics.Cost :=
  (Semantics.Cost.withSubstitute cost substitute)

/-- `abilities/written.ron` -/
semantic_template written (characteristics : Semantics.CharacteristicBundle) : Semantics.TokenSpec :=
  (Semantics.TokenSpec.written characteristics)

/-- `ability_words/adamant.ron` -/
semantic_definition adamant (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Adamant") arg0)

/-- `ability_words/addendum.ron` -/
semantic_definition addendum (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Addendum") arg0)

/-- `ability_words/alliance.ron` -/
semantic_definition alliance (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Alliance") arg0)

/-- `ability_words/battalion.ron` -/
semantic_definition battalion (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Battalion") arg0)

/-- `ability_words/bloodrush.ron` -/
semantic_definition bloodrush (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Bloodrush") arg0)

/-- `ability_words/celebration.ron` -/
semantic_definition celebration (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Celebration") arg0)

/-- `ability_words/channel.ron` -/
semantic_definition channel (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Channel") arg0)

/-- `ability_words/chroma.ron` -/
semantic_definition chroma (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Chroma") arg0)

/-- `ability_words/cohort.ron` -/
semantic_definition cohort (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Cohort") arg0)

/-- `ability_words/constellation.ron` -/
semantic_definition constellation (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Constellation") arg0)

/-- `ability_words/converge.ron` -/
semantic_definition converge (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Converge") arg0)

/-- `ability_words/councilsDilemma.ron` -/
semantic_definition councilsDilemma (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Council's dilemma") arg0)

/-- `ability_words/coven.ron` -/
semantic_definition coven (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Coven") arg0)

/-- `ability_words/delirium.ron` -/
semantic_definition delirium (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Delirium") arg0)

/-- `ability_words/descend4.ron` -/
semantic_definition descend4 (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Descend 4") arg0)

/-- `ability_words/descend8.ron` -/
semantic_definition descend8 (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Descend 8") arg0)

/-- `ability_words/disappear.ron` -/
semantic_definition disappear (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Disappear") arg0)

/-- `ability_words/domain.ron` -/
semantic_definition domain (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Domain") arg0)

/-- `ability_words/eerie.ron` -/
semantic_definition eerie (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Eerie") arg0)

/-- `ability_words/eminence.ron` -/
semantic_definition eminence (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Eminence") arg0)

/-- `ability_words/enrage.ron` -/
semantic_definition enrage (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Enrage") arg0)

/-- `ability_words/fatefulHour.ron` -/
semantic_definition fatefulHour (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Fateful hour") arg0)

/-- `ability_words/fathomlessDescent.ron` -/
semantic_definition fathomlessDescent (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Fathomless descent") arg0)

/-- `ability_words/ferocious.ron` -/
semantic_definition ferocious (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Ferocious") arg0)

/-- `ability_words/flurry.ron` -/
semantic_definition flurry (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Flurry") arg0)

/-- `ability_words/formidable.ron` -/
semantic_definition formidable (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Formidable") arg0)

/-- `ability_words/grandeur.ron` -/
semantic_definition grandeur (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Grandeur") arg0)

/-- `ability_words/hellbent.ron` -/
semantic_definition hellbent (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Hellbent") arg0)

/-- `ability_words/heroic.ron` -/
semantic_definition heroic (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Heroic") arg0)

/-- `ability_words/imprint.ron` -/
semantic_definition imprint (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Imprint") arg0)

/-- `ability_words/infusion.ron` -/
semantic_definition infusion (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Infusion") arg0)

/-- `ability_words/inspired.ron` -/
semantic_definition inspired (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Inspired") arg0)

/-- `ability_words/joinForces.ron` -/
semantic_definition joinForces (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Join forces") arg0)

/-- `ability_words/kinship.ron` -/
semantic_definition kinship (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Kinship") arg0)

/-- `ability_words/landfall.ron` -/
semantic_definition landfall (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Landfall") arg0)

/-- `ability_words/lieutenant.ron` -/
semantic_definition lieutenant (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Lieutenant") arg0)

/-- `ability_words/magecraft.ron` -/
semantic_definition magecraft (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Magecraft") arg0)

/-- `ability_words/metalcraft.ron` -/
semantic_definition metalcraft (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Metalcraft") arg0)

/-- `ability_words/morbid.ron` -/
semantic_definition morbid (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Morbid") arg0)

/-- `ability_words/opus.ron` -/
semantic_definition opus (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Opus") arg0)

/-- `ability_words/packTactics.ron` -/
semantic_definition packTactics (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Pack tactics") arg0)

/-- `ability_words/paradox.ron` -/
semantic_definition paradox (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Paradox") arg0)

/-- `ability_words/parley.ron` -/
semantic_definition parley (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Parley") arg0)

/-- `ability_words/radiance.ron` -/
semantic_definition radiance (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Radiance") arg0)

/-- `ability_words/raid.ron` -/
semantic_definition raid (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Raid") arg0)

/-- `ability_words/rally.ron` -/
semantic_definition rally (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Rally") arg0)

/-- `ability_words/renew.ron` -/
semantic_definition renew (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Renew") arg0)

/-- `ability_words/repartee.ron` -/
semantic_definition repartee (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Repartee") arg0)

/-- `ability_words/revolt.ron` -/
semantic_definition revolt (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Revolt") arg0)

/-- `ability_words/secretCouncil.ron` -/
semantic_definition secretCouncil (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Secret council") arg0)

/-- `ability_words/spellMastery.ron` -/
semantic_definition spellMastery (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Spell mastery") arg0)

/-- `ability_words/strive.ron` -/
semantic_definition strive (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Strive") arg0)

/-- `ability_words/survival.ron` -/
semantic_definition survival (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Survival") arg0)

/-- `ability_words/sweep.ron` -/
semantic_definition sweep (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Sweep") arg0)

/-- `ability_words/temptingOffer.ron` -/
semantic_definition temptingOffer (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Tempting offer") arg0)

/-- `ability_words/threshold.ron` -/
semantic_definition threshold (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Threshold") arg0)

/-- `ability_words/undergrowth.ron` -/
semantic_definition undergrowth (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Undergrowth") arg0)

/-- `ability_words/valiant.ron` -/
semantic_definition valiant (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Valiant") arg0)

/-- `ability_words/vivid.ron` -/
semantic_definition vivid (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Vivid") arg0)

/-- `ability_words/void.ron` -/
semantic_definition void (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Void") arg0)

/-- `ability_words/willOfTheCouncil.ron` -/
semantic_definition willOfTheCouncil (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.italicHead (Semantics.ItalicWord.abilityWord "Will of the council") arg0)

/-- `amounts/aggregateOf.ron` -/
semantic_template aggregateOf (op : Semantics.AggregateOp) (amounts : (List Semantics.Amount) := []) : Semantics.Amount :=
  (Semantics.Amount.aggregateOf op amounts)

/-- `amounts/aggregateOver.ron` -/
semantic_template aggregateOver (op : Semantics.AggregateOp) (domain : Semantics.Predicate) (body : Semantics.Amount) : Semantics.Amount :=
  (Semantics.Amount.aggregateOver op domain body)

/-- `amounts/arith.ron` -/
semantic_template arith (op : Semantics.ArithOp) (left : Semantics.Amount) (right : Semantics.Amount) : Semantics.Amount :=
  (Semantics.Amount.arith op left right)

/-- `amounts/coinsShowing.ron` -/
semantic_template coinsShowing (face : Semantics.CoinFace) : Semantics.Amount :=
  (Semantics.Amount.coinsShowing face)

/-- `amounts/countersOn.ron` -/
semantic_template countersOn (kind : Semantics.CounterKind) (holder : Semantics.NounPhrase) : Semantics.Amount :=
  (Semantics.Amount.statOf (Semantics.ProjAxis.counter kind) holder)

/-- `amounts/devotion.ron` -/
semantic_template devotion (player : Semantics.NounPhrase) (color : Semantics.ColorTerm) (second : (Option Semantics.ColorTerm) := none) : Semantics.Amount :=
  (Semantics.Amount.devotion player color second)

/-- `amounts/distinctCount.ron` -/
semantic_template distinctCount (axis : Semantics.KindAxis) (domain : Semantics.NounPhrase) : Semantics.Amount :=
  (Semantics.Amount.distinctCount axis domain)

/-- `amounts/down.ron` -/
semantic_template down (amount : Semantics.Amount) : (Semantics.Delta Semantics.Amount) :=
  (Semantics.Delta.down amount)

/-- `amounts/eventTally.ron` -/
semantic_template eventTally (clause : Semantics.DidClause) (mode : Semantics.TallyMode) : Semantics.Amount :=
  (Semantics.Amount.tally clause mode)

/-- `amounts/finalChapterNumber.ron` -/
semantic_template finalChapterNumber (saga : Semantics.NounPhrase) : Semantics.Amount :=
  (Semantics.Amount.aggregate Semantics.AggregateOp.max Semantics.ProjAxis.chapter saga)

/-- `amounts/forEach.ron` -/
semantic_template forEach (per : Semantics.Amount) (p : Semantics.Predicate) : Semantics.Amount :=
  (Semantics.Amount.arith Semantics.ArithOp.times per (Semantics.Amount.countOf (Semantics.NounPhrase.described Semantics.DetPhrase.bare p)))

/-- `amounts/greaterOf.ron` -/
semantic_template greaterOf (a : Semantics.Amount) (b : Semantics.Amount) : Semantics.Amount :=
  (Semantics.Amount.aggregateOf Semantics.AggregateOp.max [a, b])

/-- `amounts/greatestStoredMatch.ron` -/
semantic_template greatestStoredMatch (subject : Semantics.NounPhrase) : Semantics.Amount :=
  (Semantics.Amount.greatestStoredMatch subject)

/-- `amounts/groupSize.ron` -/
semantic_template groupSize : Semantics.Amount :=
  Semantics.Amount.groupSize

/-- `amounts/half.ron` -/
semantic_template half (rounding : Semantics.RoundMode) (amount : Semantics.Amount) : Semantics.Amount :=
  (Semantics.Amount.half rounding amount)

/-- `amounts/letter.ron` -/
semantic_template letter (letter : Semantics.Letter) : Semantics.Amount :=
  (Semantics.Amount.letter letter)

/-- `amounts/lifeTotalOf.ron` -/
semantic_template lifeTotalOf (player : Semantics.NounPhrase) : Semantics.Amount :=
  (Semantics.Amount.statOf (Semantics.ProjAxis.playerStat Semantics.PlayerStat.lifeTotal) player)

/-- `amounts/lit.ron` -/
semantic_template lit (value : Semantics.Amount) : Semantics.Amount :=
  value

/-- `amounts/minus.ron` -/
semantic_template minus (left : Semantics.Amount) (right : Semantics.Amount) : Semantics.Amount :=
  (Semantics.Amount.arith Semantics.ArithOp.minus left right)

/-- `amounts/numberOf.ron` -/
semantic_template numberOf (subject : Semantics.NounPhrase) : Semantics.Amount :=
  (Semantics.Amount.countOf subject)

/-- `amounts/paid.ron` -/
semantic_template paid (facet : Semantics.LinkedFacet) (subject : Semantics.NounPhrase) : Semantics.Amount :=
  (Semantics.Amount.paid facet subject)

/-- `amounts/plus.ron` -/
semantic_template plus (left : Semantics.Amount) (right : Semantics.Amount) : Semantics.Amount :=
  (Semantics.Amount.arith Semantics.ArithOp.plus left right)

/-- `amounts/powerOf.ron` -/
semantic_template powerOf (subject : Semantics.NounPhrase) : Semantics.Amount :=
  (Semantics.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.power) subject)

/-- `amounts/preventedThisWay.ron` -/
semantic_template preventedThisWay : Semantics.Amount :=
  (Semantics.Amount.theOutcome Semantics.OutcomeSort.damagePrevented)

/-- `amounts/set.ron` -/
semantic_template set (amount : Semantics.Amount) : (Semantics.Delta Semantics.Amount) :=
  (Semantics.Delta.set amount)

/-- `amounts/smallerOf.ron` -/
semantic_template smallerOf (a : Semantics.Amount) (b : Semantics.Amount) : Semantics.Amount :=
  (Semantics.Amount.aggregateOf Semantics.AggregateOp.min [a, b])

/-- `amounts/speedOf.ron` -/
semantic_template speedOf (player : Semantics.NounPhrase) : Semantics.Amount :=
  (Semantics.Amount.statOf (Semantics.ProjAxis.playerStat Semantics.PlayerStat.speed) player)

/-- `amounts/statOf.ron` -/
semantic_template statOf (axis : Semantics.ProjAxis) (subject : Semantics.NounPhrase) : Semantics.Amount :=
  (Semantics.Amount.statOf axis subject)

/-- `amounts/thatMuch.ron` -/
semantic_template thatMuch : Semantics.Amount :=
  Semantics.Amount.thatMuch

/-- `amounts/theDifference.ron` -/
semantic_template theDifference : Semantics.Amount :=
  Semantics.Amount.theDifference

/-- `amounts/theOutcome.ron` -/
semantic_template theOutcome (sort : Semantics.OutcomeSort) : Semantics.Amount :=
  (Semantics.Amount.theOutcome sort)

/-- `amounts/times.ron` -/
semantic_template times (per : Semantics.Amount) (amount : Semantics.Amount) : Semantics.Amount :=
  (Semantics.Amount.arith Semantics.ArithOp.times per amount)

/-- `amounts/totalOf.ron` -/
semantic_template totalOf (axis : Semantics.ProjAxis) (group : Semantics.NounPhrase) : Semantics.Amount :=
  (Semantics.Amount.aggregate Semantics.AggregateOp.sum axis group)

/-- `amounts/toughnessOf.ron` -/
semantic_template toughnessOf (subject : Semantics.NounPhrase) : Semantics.Amount :=
  (Semantics.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.toughness) subject)

/-- `amounts/up.ron` -/
semantic_template up (amount : Semantics.Amount) : (Semantics.Delta Semantics.Amount) :=
  (Semantics.Delta.up amount)

/-- `amounts/upTo.ron` -/
semantic_template Amount.upTo (bound : Semantics.Amount) : Semantics.Amount :=
  (Semantics.Amount.upTo bound)

/-- `amounts/votesFor.ron` -/
semantic_template votesFor (label : String) : Semantics.Amount :=
  (Semantics.Amount.votesFor label)

/-- `conditions/and.ron` -/
semantic_template Condition.and (conjuncts : (List Semantics.Condition) := []) : Semantics.Condition :=
  (Semantics.Condition.and conjuncts)

/-- `conditions/anyResultIs.ron` -/
semantic_template anyResultIs (comparator : Semantics.Comparator) (bound : Semantics.Amount) : Semantics.Condition :=
  (Semantics.Condition.anyResultIs comparator bound)

/-- `conditions/choiceWasTaken.ron` -/
semantic_template choiceWasTaken (which : Semantics.LinkSource) (subject : Semantics.NounPhrase) : Semantics.Condition :=
  (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.taken which) subject) Semantics.Comparator.atLeast (Semantics.Amount.lit 1))

/-- `conditions/choseThisWay.ron` -/
semantic_template choseThisWay (chooser : Semantics.NounPhrase) (predicate : Semantics.Predicate) : Semantics.Condition :=
  (Semantics.Condition.choseThisWay chooser predicate)

/-- `conditions/compareAmt.ron` -/
semantic_template compareAmt (subject : Semantics.Amount) (comparator : Semantics.Comparator) (bound : Semantics.Amount) : Semantics.Condition :=
  (Semantics.Condition.compareAmt subject comparator bound)

/-- `conditions/dealtThisWay.ron` -/
semantic_template dealtThisWay (predicate : Semantics.Predicate) : Semantics.Condition :=
  (Semantics.Condition.dealtThisWay predicate)

/-- `conditions/duringPart.ron` -/
semantic_template Condition.duringPart (part : Semantics.TurnPart) (whose : (Option Semantics.NounPhrase) := none) : Semantics.Condition :=
  (Semantics.Condition.duringPart part whose)

/-- `conditions/exists.ron` -/
semantic_template exists_ (p : Semantics.Predicate) : Semantics.Condition :=
  (Semantics.Condition.exists_ (Semantics.NounPhrase.described Semantics.DetPhrase.bare p))

/-- `conditions/flipCalled.ron` -/
semantic_template flipCalled (caller : Semantics.NounPhrase) (call : Semantics.FlipCall) : Semantics.Condition :=
  (Semantics.Condition.flipCalled caller call)

/-- `conditions/flipFace.ron` -/
semantic_template flipFace (face : Semantics.CoinFace) : Semantics.Condition :=
  (Semantics.Condition.flipFace face)

/-- `conditions/gameIs.ron` -/
semantic_template gameIs (designation : Semantics.DesignationLabel) : Semantics.Condition :=
  (Semantics.Condition.gameIs designation)

/-- `conditions/keywordCostPaid.ron` -/
semantic_template keywordCostPaid (label : Semantics.KeywordLabel) (subject : Semantics.NounPhrase) : Semantics.Condition :=
  (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword label) none) subject) Semantics.Comparator.atLeast (Semantics.Amount.lit 1))

/-- `conditions/matches.ron` -/
semantic_template matches_ (subject : Semantics.NounPhrase) (predicate : Semantics.Predicate) : Semantics.Condition :=
  (Semantics.Condition.matches subject predicate)

/-- `conditions/noHolder.ron` -/
semantic_template noHolder (designation : Semantics.DesignationLabel) : Semantics.Condition :=
  (Semantics.Condition.noHolder designation)

/-- `conditions/not.ron` -/
semantic_template Condition.not (condition : Semantics.Condition) : Semantics.Condition :=
  (Semantics.Condition.not condition)

/-- `conditions/or.ron` -/
semantic_template Condition.or (disjuncts : (List Semantics.Condition) := []) : Semantics.Condition :=
  (Semantics.Condition.or disjuncts)

/-- `conditions/preventedFromSource.ron` -/
semantic_template preventedFromSource (predicate : Semantics.Predicate) : Semantics.Condition :=
  (Semantics.Condition.preventedFromSource predicate)

/-- `conditions/rolledDoubles.ron` -/
semantic_template rolledDoubles : Semantics.Condition :=
  Semantics.Condition.rolledDoubles

/-- `conditions/voteLead.ron` -/
semantic_template voteLead (label : String) (orTied : Bool) : Semantics.Condition :=
  (Semantics.Condition.voteLead label orTied)

/-- `conferrals/boost.ron` -/
semantic_template boost (power : (Semantics.Delta Semantics.Amount)) (toughness : (Semantics.Delta Semantics.Amount)) : Semantics.Conferral :=
  (Semantics.Conferral.property (Semantics.StaticSpec.ptModification Semantics.NounPhrase.this power toughness))

/-- `conferrals/castableFrom.ron` -/
semantic_template castableFrom (zone : Semantics.ZoneExpr) (timing : Semantics.Timing) : Semantics.Conferral :=
  (Semantics.Conferral.property (Semantics.StaticSpec.may (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Macros.deonticActionFor (Semantics.Deed.action "cast") Semantics.NounPhrase.this) ({ bound := none, asThough := none, rider := (Semantics.DeonticRider.play (some zone) none (some timing) false Semantics.PlayPayment.itsOwnCost), except := none } : Semantics.DeonticMods)))

/-- `conferrals/combatant.ron` -/
semantic_template combatant : Semantics.Conferral :=
  (Semantics.Conferral.property (Semantics.StaticSpec.combatant Semantics.NounPhrase.this))

/-- `conferrals/grants.ron` -/
semantic_template grants (keyword : Semantics.KeywordLabel) : Semantics.Conferral :=
  (Semantics.Conferral.property (Semantics.StaticSpec.abilityGrant Semantics.NounPhrase.this (Semantics.Ability.keyword keyword [] [])))

/-- `conferrals/playableFrom.ron` -/
semantic_template playableFrom (zone : Semantics.ZoneExpr) (timing : Semantics.Timing) : Semantics.Conferral :=
  (Semantics.Conferral.property (Semantics.StaticSpec.may (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Macros.deonticActionFor (Semantics.Deed.action "play") Semantics.NounPhrase.this) ({ bound := none, asThough := none, rider := (Semantics.DeonticRider.play (some zone) none (some timing) false Semantics.PlayPayment.itsOwnCost), except := none } : Semantics.DeonticMods)))

/-- `conferrals/stateBased.ron` -/
semantic_template stateBased (when : Semantics.Condition) (then_ : Semantics.Instruction) : Semantics.Conferral :=
  (Semantics.Conferral.stateBased when then_)

/-- `counter_kinds/acornCounter.ron` -/
semantic_definition acornCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "acornCounter")

/-- `counter_kinds/aegisCounter.ron` -/
semantic_definition aegisCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "aegisCounter")

/-- `counter_kinds/ageCounter.ron` -/
semantic_definition ageCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "ageCounter")

/-- `counter_kinds/aimCounter.ron` -/
semantic_definition aimCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "aimCounter")

/-- `counter_kinds/arrowCounter.ron` -/
semantic_definition arrowCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "arrowCounter")

/-- `counter_kinds/arrowheadCounter.ron` -/
semantic_definition arrowheadCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "arrowheadCounter")

/-- `counter_kinds/awakeningCounter.ron` -/
semantic_definition awakeningCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "awakeningCounter")

/-- `counter_kinds/baitCounter.ron` -/
semantic_definition baitCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "baitCounter")

/-- `counter_kinds/blazeCounter.ron` -/
semantic_definition blazeCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "blazeCounter")

/-- `counter_kinds/blessingCounter.ron` -/
semantic_definition blessingCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "blessingCounter")

/-- `counter_kinds/bloodCounter.ron` -/
semantic_definition bloodCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "bloodCounter")

/-- `counter_kinds/bloodlineCounter.ron` -/
semantic_definition bloodlineCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "bloodlineCounter")

/-- `counter_kinds/bloodstainCounter.ron` -/
semantic_definition bloodstainCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "bloodstainCounter")

/-- `counter_kinds/bookCounter.ron` -/
semantic_definition bookCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "bookCounter")

/-- `counter_kinds/boreCounter.ron` -/
semantic_definition boreCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "boreCounter")

/-- `counter_kinds/bountyCounter.ron` -/
semantic_definition bountyCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "bountyCounter")

/-- `counter_kinds/brainCounter.ron` -/
semantic_definition brainCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "brainCounter")

/-- `counter_kinds/briberyCounter.ron` -/
semantic_definition briberyCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "briberyCounter")

/-- `counter_kinds/brickCounter.ron` -/
semantic_definition brickCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "brickCounter")

/-- `counter_kinds/burdenCounter.ron` -/
semantic_definition burdenCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "burdenCounter")

/-- `counter_kinds/cageCounter.ron` -/
semantic_definition cageCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "cageCounter")

/-- `counter_kinds/carrionCounter.ron` -/
semantic_definition carrionCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "carrionCounter")

/-- `counter_kinds/cellCounter.ron` -/
semantic_definition cellCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "cellCounter")

/-- `counter_kinds/chargeCounter.ron` -/
semantic_definition chargeCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "chargeCounter")

/-- `counter_kinds/chorusCounter.ron` -/
semantic_definition chorusCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "chorusCounter")

/-- `counter_kinds/collectionCounter.ron` -/
semantic_definition collectionCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "collectionCounter")

/-- `counter_kinds/componentCounter.ron` -/
semantic_definition componentCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "componentCounter")

/-- `counter_kinds/conquerorCounter.ron` -/
semantic_definition conquerorCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "conquerorCounter")

/-- `counter_kinds/contestedCounter.ron` -/
semantic_definition contestedCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "contestedCounter")

/-- `counter_kinds/corpseCounter.ron` -/
semantic_definition corpseCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "corpseCounter")

/-- `counter_kinds/corruptionCounter.ron` -/
semantic_definition corruptionCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "corruptionCounter")

/-- `counter_kinds/creditCounter.ron` -/
semantic_definition creditCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "creditCounter")

/-- `counter_kinds/croakCounter.ron` -/
semantic_definition croakCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "croakCounter")

/-- `counter_kinds/crystalCounter.ron` -/
semantic_definition crystalCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "crystalCounter")

/-- `counter_kinds/cubeCounter.ron` -/
semantic_definition cubeCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "cubeCounter")

/-- `counter_kinds/currencyCounter.ron` -/
semantic_definition currencyCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "currencyCounter")

/-- `counter_kinds/deathtouchCounter.ron` -/
semantic_definition deathtouchCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "deathtouchCounter")

/-- `counter_kinds/decayedCounter.ron` -/
semantic_definition decayedCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "decayedCounter")

/-- `counter_kinds/defenseCounter.ron` -/
semantic_definition defenseCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "defenseCounter")

/-- `counter_kinds/delayCounter.ron` -/
semantic_definition delayCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "delayCounter")

/-- `counter_kinds/depletionCounter.ron` -/
semantic_definition depletionCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "depletionCounter")

/-- `counter_kinds/descentCounter.ron` -/
semantic_definition descentCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "descentCounter")

/-- `counter_kinds/despairCounter.ron` -/
semantic_definition despairCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "despairCounter")

/-- `counter_kinds/discoveryCounter.ron` -/
semantic_definition discoveryCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "discoveryCounter")

/-- `counter_kinds/divinityCounter.ron` -/
semantic_definition divinityCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "divinityCounter")

/-- `counter_kinds/doomCounter.ron` -/
semantic_definition doomCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "doomCounter")

/-- `counter_kinds/doubleStrikeCounter.ron` -/
semantic_definition doubleStrikeCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "doubleStrikeCounter")

/-- `counter_kinds/dreadCounter.ron` -/
semantic_definition dreadCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "dreadCounter")

/-- `counter_kinds/dreamCounter.ron` -/
semantic_definition dreamCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "dreamCounter")

/-- `counter_kinds/dutyCounter.ron` -/
semantic_definition dutyCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "dutyCounter")

/-- `counter_kinds/eggCounter.ron` -/
semantic_definition eggCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "eggCounter")

/-- `counter_kinds/elixirCounter.ron` -/
semantic_definition elixirCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "elixirCounter")

/-- `counter_kinds/emberCounter.ron` -/
semantic_definition emberCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "emberCounter")

/-- `counter_kinds/energy.ron` -/
semantic_definition energy : Semantics.CounterKind :=
  (Semantics.CounterKind.named "energy")

/-- `counter_kinds/enlightenedCounter.ron` -/
semantic_definition enlightenedCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "enlightenedCounter")

/-- `counter_kinds/eonCounter.ron` -/
semantic_definition eonCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "eonCounter")

/-- `counter_kinds/exaltedCounter.ron` -/
semantic_definition exaltedCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "exaltedCounter")

/-- `counter_kinds/experience.ron` -/
semantic_definition experience : Semantics.CounterKind :=
  (Semantics.CounterKind.named "experience")

/-- `counter_kinds/eyeballCounter.ron` -/
semantic_definition eyeballCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "eyeballCounter")

/-- `counter_kinds/fadeCounter.ron` -/
semantic_definition fadeCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "fadeCounter")

/-- `counter_kinds/fateCounter.ron` -/
semantic_definition fateCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "fateCounter")

/-- `counter_kinds/featherCounter.ron` -/
semantic_definition featherCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "featherCounter")

/-- `counter_kinds/feedingCounter.ron` -/
semantic_definition feedingCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "feedingCounter")

/-- `counter_kinds/fellowshipCounter.ron` -/
semantic_definition fellowshipCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "fellowshipCounter")

/-- `counter_kinds/fetchCounter.ron` -/
semantic_definition fetchCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "fetchCounter")

/-- `counter_kinds/filibusterCounter.ron` -/
semantic_definition filibusterCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "filibusterCounter")

/-- `counter_kinds/filmCounter.ron` -/
semantic_definition filmCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "filmCounter")

/-- `counter_kinds/finalityCounter.ron` -/
semantic_definition finalityCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "finalityCounter")

/-- `counter_kinds/fireCounter.ron` -/
semantic_definition fireCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "fireCounter")

/-- `counter_kinds/firstStrikeCounter.ron` -/
semantic_definition firstStrikeCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "firstStrikeCounter")

/-- `counter_kinds/flameCounter.ron` -/
semantic_definition flameCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "flameCounter")

/-- `counter_kinds/floodCounter.ron` -/
semantic_definition floodCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "floodCounter")

/-- `counter_kinds/flyingCounter.ron` -/
semantic_definition flyingCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "flyingCounter")

/-- `counter_kinds/foreshadowCounter.ron` -/
semantic_definition foreshadowCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "foreshadowCounter")

/-- `counter_kinds/fungusCounter.ron` -/
semantic_definition fungusCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "fungusCounter")

/-- `counter_kinds/furyCounter.ron` -/
semantic_definition furyCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "furyCounter")

/-- `counter_kinds/fuseCounter.ron` -/
semantic_definition fuseCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "fuseCounter")

/-- `counter_kinds/gemCounter.ron` -/
semantic_definition gemCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "gemCounter")

/-- `counter_kinds/ghostformCounter.ron` -/
semantic_definition ghostformCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "ghostformCounter")

/-- `counter_kinds/glyphCounter.ron` -/
semantic_definition glyphCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "glyphCounter")

/-- `counter_kinds/goldCounter.ron` -/
semantic_definition goldCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "goldCounter")

/-- `counter_kinds/growthCounter.ron` -/
semantic_definition growthCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "growthCounter")

/-- `counter_kinds/harmonyCounter.ron` -/
semantic_definition harmonyCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "harmonyCounter")

/-- `counter_kinds/hasteCounter.ron` -/
semantic_definition hasteCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "hasteCounter")

/-- `counter_kinds/hatchingCounter.ron` -/
semantic_definition hatchingCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "hatchingCounter")

/-- `counter_kinds/hatchlingCounter.ron` -/
semantic_definition hatchlingCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "hatchlingCounter")

/-- `counter_kinds/hexproofCounter.ron` -/
semantic_definition hexproofCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "hexproofCounter")

/-- `counter_kinds/hitCounter.ron` -/
semantic_definition hitCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "hitCounter")

/-- `counter_kinds/honeCounter.ron` -/
semantic_definition honeCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "honeCounter")

/-- `counter_kinds/hoofprintCounter.ron` -/
semantic_definition hoofprintCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "hoofprintCounter")

/-- `counter_kinds/hopeCounter.ron` -/
semantic_definition hopeCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "hopeCounter")

/-- `counter_kinds/hourCounter.ron` -/
semantic_definition hourCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "hourCounter")

/-- `counter_kinds/hourglassCounter.ron` -/
semantic_definition hourglassCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "hourglassCounter")

/-- `counter_kinds/hungerCounter.ron` -/
semantic_definition hungerCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "hungerCounter")

/-- `counter_kinds/iceCounter.ron` -/
semantic_definition iceCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "iceCounter")

/-- `counter_kinds/impostorCounter.ron` -/
semantic_definition impostorCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "impostorCounter")

/-- `counter_kinds/incarnationCounter.ron` -/
semantic_definition incarnationCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "incarnationCounter")

/-- `counter_kinds/incubationCounter.ron` -/
semantic_definition incubationCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "incubationCounter")

/-- `counter_kinds/indestructibleCounter.ron` -/
semantic_definition indestructibleCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "indestructibleCounter")

/-- `counter_kinds/infectionCounter.ron` -/
semantic_definition infectionCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "infectionCounter")

/-- `counter_kinds/influenceCounter.ron` -/
semantic_definition influenceCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "influenceCounter")

/-- `counter_kinds/ingenuityCounter.ron` -/
semantic_definition ingenuityCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "ingenuityCounter")

/-- `counter_kinds/ingredientCounter.ron` -/
semantic_definition ingredientCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "ingredientCounter")

/-- `counter_kinds/intelCounter.ron` -/
semantic_definition intelCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "intelCounter")

/-- `counter_kinds/interventionCounter.ron` -/
semantic_definition interventionCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "interventionCounter")

/-- `counter_kinds/invasionCounter.ron` -/
semantic_definition invasionCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "invasionCounter")

/-- `counter_kinds/invitationCounter.ron` -/
semantic_definition invitationCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "invitationCounter")

/-- `counter_kinds/isolationCounter.ron` -/
semantic_definition isolationCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "isolationCounter")

/-- `counter_kinds/javelinCounter.ron` -/
semantic_definition javelinCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "javelinCounter")

/-- `counter_kinds/judgmentCounter.ron` -/
semantic_definition judgmentCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "judgmentCounter")

/-- `counter_kinds/kiCounter.ron` -/
semantic_definition kiCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "kiCounter")

/-- `counter_kinds/kickCounter.ron` -/
semantic_definition kickCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "kickCounter")

/-- `counter_kinds/knowledgeCounter.ron` -/
semantic_definition knowledgeCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "knowledgeCounter")

/-- `counter_kinds/landmarkCounter.ron` -/
semantic_definition landmarkCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "landmarkCounter")

/-- `counter_kinds/levelCounter.ron` -/
semantic_definition levelCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "levelCounter")

/-- `counter_kinds/lifelinkCounter.ron` -/
semantic_definition lifelinkCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "lifelinkCounter")

/-- `counter_kinds/lootCounter.ron` -/
semantic_definition lootCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "lootCounter")

/-- `counter_kinds/loreCounter.ron` -/
semantic_definition loreCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "loreCounter")

/-- `counter_kinds/loyaltyCounter.ron` -/
semantic_definition loyaltyCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "loyaltyCounter")

/-- `counter_kinds/luckCounter.ron` -/
semantic_definition luckCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "luckCounter")

/-- `counter_kinds/m1m1Counter.ron` -/
semantic_definition m1m1Counter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "m1m1Counter")

/-- `counter_kinds/magnetCounter.ron` -/
semantic_definition magnetCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "magnetCounter")

/-- `counter_kinds/manifestationCounter.ron` -/
semantic_definition manifestationCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "manifestationCounter")

/-- `counter_kinds/mannequinCounter.ron` -/
semantic_definition mannequinCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "mannequinCounter")

/-- `counter_kinds/matrixCounter.ron` -/
semantic_definition matrixCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "matrixCounter")

/-- `counter_kinds/memoryCounter.ron` -/
semantic_definition memoryCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "memoryCounter")

/-- `counter_kinds/menaceCounter.ron` -/
semantic_definition menaceCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "menaceCounter")

/-- `counter_kinds/mineCounter.ron` -/
semantic_definition mineCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "mineCounter")

/-- `counter_kinds/miningCounter.ron` -/
semantic_definition miningCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "miningCounter")

/-- `counter_kinds/mireCounter.ron` -/
semantic_definition mireCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "mireCounter")

/-- `counter_kinds/musicCounter.ron` -/
semantic_definition musicCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "musicCounter")

/-- `counter_kinds/musterCounter.ron` -/
semantic_definition musterCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "musterCounter")

/-- `counter_kinds/nestCounter.ron` -/
semantic_definition nestCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "nestCounter")

/-- `counter_kinds/netCounter.ron` -/
semantic_definition netCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "netCounter")

/-- `counter_kinds/nightCounter.ron` -/
semantic_definition nightCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "nightCounter")

/-- `counter_kinds/oilCounter.ron` -/
semantic_definition oilCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "oilCounter")

/-- `counter_kinds/omenCounter.ron` -/
semantic_definition omenCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "omenCounter")

/-- `counter_kinds/oreCounter.ron` -/
semantic_definition oreCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "oreCounter")

/-- `counter_kinds/p1p1Counter.ron` -/
semantic_definition p1p1Counter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "p1p1Counter")

/-- `counter_kinds/pageCounter.ron` -/
semantic_definition pageCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "pageCounter")

/-- `counter_kinds/painCounter.ron` -/
semantic_definition painCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "painCounter")

/-- `counter_kinds/palliationCounter.ron` -/
semantic_definition palliationCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "palliationCounter")

/-- `counter_kinds/paralyzationCounter.ron` -/
semantic_definition paralyzationCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "paralyzationCounter")

/-- `counter_kinds/petalCounter.ron` -/
semantic_definition petalCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "petalCounter")

/-- `counter_kinds/petrificationCounter.ron` -/
semantic_definition petrificationCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "petrificationCounter")

/-- `counter_kinds/phylacteryCounter.ron` -/
semantic_definition phylacteryCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "phylacteryCounter")

/-- `counter_kinds/phyresisCounter.ron` -/
semantic_definition phyresisCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "phyresisCounter")

/-- `counter_kinds/pinCounter.ron` -/
semantic_definition pinCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "pinCounter")

/-- `counter_kinds/plagueCounter.ron` -/
semantic_definition plagueCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "plagueCounter")

/-- `counter_kinds/planCounter.ron` -/
semantic_definition planCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "planCounter")

/-- `counter_kinds/pointCounter.ron` -/
semantic_definition pointCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "pointCounter")

/-- `counter_kinds/poison.ron` -/
semantic_definition poison : Semantics.CounterKind :=
  (Semantics.CounterKind.named "poison")

/-- `counter_kinds/polypCounter.ron` -/
semantic_definition polypCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "polypCounter")

/-- `counter_kinds/possessionCounter.ron` -/
semantic_definition possessionCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "possessionCounter")

/-- `counter_kinds/pressureCounter.ron` -/
semantic_definition pressureCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "pressureCounter")

/-- `counter_kinds/preyCounter.ron` -/
semantic_definition preyCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "preyCounter")

/-- `counter_kinds/pupaCounter.ron` -/
semantic_definition pupaCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "pupaCounter")

/-- `counter_kinds/questCounter.ron` -/
semantic_definition questCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "questCounter")

/-- `counter_kinds/radCounter.ron` -/
semantic_definition radCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "radCounter")

/-- `counter_kinds/rallyCounter.ron` -/
semantic_definition rallyCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "rallyCounter")

/-- `counter_kinds/reachCounter.ron` -/
semantic_definition reachCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "reachCounter")

/-- `counter_kinds/refineCounter.ron` -/
semantic_definition refineCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "refineCounter")

/-- `counter_kinds/rejectionCounter.ron` -/
semantic_definition rejectionCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "rejectionCounter")

/-- `counter_kinds/reprieveCounter.ron` -/
semantic_definition reprieveCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "reprieveCounter")

/-- `counter_kinds/revCounter.ron` -/
semantic_definition revCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "revCounter")

/-- `counter_kinds/revivalCounter.ron` -/
semantic_definition revivalCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "revivalCounter")

/-- `counter_kinds/ribbonCounter.ron` -/
semantic_definition ribbonCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "ribbonCounter")

/-- `counter_kinds/ritualCounter.ron` -/
semantic_definition ritualCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "ritualCounter")

/-- `counter_kinds/ropeCounter.ron` -/
semantic_definition ropeCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "ropeCounter")

/-- `counter_kinds/rustCounter.ron` -/
semantic_definition rustCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "rustCounter")

/-- `counter_kinds/saurianCounter.ron` -/
semantic_definition saurianCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "saurianCounter")

/-- `counter_kinds/screamCounter.ron` -/
semantic_definition screamCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "screamCounter")

/-- `counter_kinds/shadowCounter.ron` -/
semantic_definition shadowCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "shadowCounter")

/-- `counter_kinds/shellCounter.ron` -/
semantic_definition shellCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "shellCounter")

/-- `counter_kinds/shieldCounter.ron` -/
semantic_definition shieldCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "shieldCounter")

/-- `counter_kinds/shredCounter.ron` -/
semantic_definition shredCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "shredCounter")

/-- `counter_kinds/silverCounter.ron` -/
semantic_definition silverCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "silverCounter")

/-- `counter_kinds/skewerCounter.ron` -/
semantic_definition skewerCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "skewerCounter")

/-- `counter_kinds/sleepCounter.ron` -/
semantic_definition sleepCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "sleepCounter")

/-- `counter_kinds/sleightCounter.ron` -/
semantic_definition sleightCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "sleightCounter")

/-- `counter_kinds/slimeCounter.ron` -/
semantic_definition slimeCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "slimeCounter")

/-- `counter_kinds/slumberCounter.ron` -/
semantic_definition slumberCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "slumberCounter")

/-- `counter_kinds/sootCounter.ron` -/
semantic_definition sootCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "sootCounter")

/-- `counter_kinds/soulCounter.ron` -/
semantic_definition soulCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "soulCounter")

/-- `counter_kinds/spiteCounter.ron` -/
semantic_definition spiteCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "spiteCounter")

/-- `counter_kinds/sporeCounter.ron` -/
semantic_definition sporeCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "sporeCounter")

/-- `counter_kinds/stashCounter.ron` -/
semantic_definition stashCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "stashCounter")

/-- `counter_kinds/storageCounter.ron` -/
semantic_definition storageCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "storageCounter")

/-- `counter_kinds/storyCounter.ron` -/
semantic_definition storyCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "storyCounter")

/-- `counter_kinds/strifeCounter.ron` -/
semantic_definition strifeCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "strifeCounter")

/-- `counter_kinds/strikeCounter.ron` -/
semantic_definition strikeCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "strikeCounter")

/-- `counter_kinds/studyCounter.ron` -/
semantic_definition studyCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "studyCounter")

/-- `counter_kinds/stunCounter.ron` -/
semantic_definition stunCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "stunCounter")

/-- `counter_kinds/supplyCounter.ron` -/
semantic_definition supplyCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "supplyCounter")

/-- `counter_kinds/suspectCounter.ron` -/
semantic_definition suspectCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "suspectCounter")

/-- `counter_kinds/takeoverCounter.ron` -/
semantic_definition takeoverCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "takeoverCounter")

/-- `counter_kinds/taskCounter.ron` -/
semantic_definition taskCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "taskCounter")

/-- `counter_kinds/theftCounter.ron` -/
semantic_definition theftCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "theftCounter")

/-- `counter_kinds/tideCounter.ron` -/
semantic_definition tideCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "tideCounter")

/-- `counter_kinds/timeCounter.ron` -/
semantic_definition timeCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "timeCounter")

/-- `counter_kinds/towerCounter.ron` -/
semantic_definition towerCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "towerCounter")

/-- `counter_kinds/trampleCounter.ron` -/
semantic_definition trampleCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "trampleCounter")

/-- `counter_kinds/trapCounter.ron` -/
semantic_definition trapCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "trapCounter")

/-- `counter_kinds/treasureCounter.ron` -/
semantic_definition treasureCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "treasureCounter")

/-- `counter_kinds/unityCounter.ron` -/
semantic_definition unityCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "unityCounter")

/-- `counter_kinds/valorCounter.ron` -/
semantic_definition valorCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "valorCounter")

/-- `counter_kinds/velocityCounter.ron` -/
semantic_definition velocityCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "velocityCounter")

/-- `counter_kinds/verseCounter.ron` -/
semantic_definition verseCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "verseCounter")

/-- `counter_kinds/vigilanceCounter.ron` -/
semantic_definition vigilanceCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "vigilanceCounter")

/-- `counter_kinds/vitalityCounter.ron` -/
semantic_definition vitalityCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "vitalityCounter")

/-- `counter_kinds/voidCounter.ron` -/
semantic_definition voidCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "voidCounter")

/-- `counter_kinds/vortexCounter.ron` -/
semantic_definition vortexCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "vortexCounter")

/-- `counter_kinds/vowCounter.ron` -/
semantic_definition vowCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "vowCounter")

/-- `counter_kinds/voyageCounter.ron` -/
semantic_definition voyageCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "voyageCounter")

/-- `counter_kinds/wageCounter.ron` -/
semantic_definition wageCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "wageCounter")

/-- `counter_kinds/winchCounter.ron` -/
semantic_definition winchCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "winchCounter")

/-- `counter_kinds/windCounter.ron` -/
semantic_definition windCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "windCounter")

/-- `counter_kinds/wishCounter.ron` -/
semantic_definition wishCounter : Semantics.CounterKind :=
  (Semantics.CounterKind.named "wishCounter")

/-- `designations/citysBlessing.ron` -/
semantic_definition citysBlessing : Semantics.DesignationLabel :=
  (Semantics.DesignationLabel.named "citysBlessing")

/-- `designations/commander.ron` -/
semantic_definition commander : Semantics.DesignationLabel :=
  (Semantics.DesignationLabel.named "commander")

/-- `designations/enduringStory.ron` -/
semantic_definition enduringStory : Semantics.DesignationLabel :=
  (Semantics.DesignationLabel.named "enduringStory")

/-- `designations/goaded.ron` -/
semantic_definition goaded : Semantics.DesignationLabel :=
  (Semantics.DesignationLabel.named "goaded")

/-- `designations/harnessed.ron` -/
semantic_definition harnessed : Semantics.DesignationLabel :=
  (Semantics.DesignationLabel.named "harnessed")

/-- `designations/initiative.ron` -/
semantic_definition initiative : Semantics.DesignationLabel :=
  (Semantics.DesignationLabel.named "initiative")

/-- `designations/leftHalfUnlocked.ron` -/
semantic_definition leftHalfUnlocked : Semantics.DesignationLabel :=
  (Semantics.DesignationLabel.named "leftHalfUnlocked")

/-- `designations/level.ron` -/
semantic_definition level : Semantics.DesignationLabel :=
  (Semantics.DesignationLabel.named "level")

/-- `designations/monarch.ron` -/
semantic_definition monarch : Semantics.DesignationLabel :=
  (Semantics.DesignationLabel.named "monarch")

/-- `designations/monstrous.ron` -/
semantic_definition monstrous : Semantics.DesignationLabel :=
  (Semantics.DesignationLabel.named "monstrous")

/-- `designations/prepared.ron` -/
semantic_definition prepared : Semantics.DesignationLabel :=
  (Semantics.DesignationLabel.named "prepared")

/-- `designations/renowned.ron` -/
semantic_definition renowned : Semantics.DesignationLabel :=
  (Semantics.DesignationLabel.named "renowned")

/-- `designations/rightHalfUnlocked.ron` -/
semantic_definition rightHalfUnlocked : Semantics.DesignationLabel :=
  (Semantics.DesignationLabel.named "rightHalfUnlocked")

/-- `designations/ringBearer.ron` -/
semantic_definition ringBearer : Semantics.DesignationLabel :=
  (Semantics.DesignationLabel.named "ringBearer")

/-- `designations/saddled.ron` -/
semantic_definition saddled : Semantics.DesignationLabel :=
  (Semantics.DesignationLabel.named "saddled")

/-- `designations/solved.ron` -/
semantic_definition DesignationLabel.solved : Semantics.DesignationLabel :=
  (Semantics.DesignationLabel.named "solved")

/-- `designations/suspected.ron` -/
semantic_definition suspected : Semantics.DesignationLabel :=
  (Semantics.DesignationLabel.named "suspected")

/-- `determiners/a.ron` -/
semantic_template a (p : Semantics.Predicate) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) p)

/-- `determiners/aRandom.ron` -/
semantic_template aRandom (p : Semantics.Predicate) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.atRandom) p)

/-- `determiners/aTheirChoice.ron` -/
semantic_template aTheirChoice (p : Semantics.Predicate) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.theirChoice) p)

/-- `determiners/aYourChoice.ron` -/
semantic_template aYourChoice (p : Semantics.Predicate) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.yourChoice) p)

/-- `determiners/aggregate.ron` -/
semantic_template aggregate (op : Semantics.AggregateOp) (axis : Semantics.ProjAxis) (p : Semantics.Predicate) : Semantics.Amount :=
  (Semantics.Amount.aggregate op axis (Semantics.NounPhrase.described Semantics.DetPhrase.bare p))

/-- `determiners/allOf.ron` -/
semantic_template allOf (p : Semantics.Predicate) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.described Semantics.DetPhrase.all p)

/-- `determiners/bare.ron` -/
semantic_template bare (p : Semantics.Predicate) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.described Semantics.DetPhrase.bare p)

/-- `determiners/countOf.ron` -/
semantic_template countOf (p : Semantics.Predicate) : Semantics.Amount :=
  (Semantics.Amount.countOf (Semantics.NounPhrase.described Semantics.DetPhrase.bare p))

/-- `determiners/counted.ron` -/
semantic_template counted (q : Semantics.Quantity) (p : Semantics.Predicate) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.described (Semantics.DetPhrase.count q none) p)

/-- `determiners/each.ron` -/
semantic_template each (p : Semantics.Predicate) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.described Semantics.DetPhrase.each p)

/-- `determiners/random.ron` -/
semantic_template random (q : Semantics.Quantity) (p : Semantics.Predicate) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.described (Semantics.DetPhrase.count q (some Semantics.ChoiceMode.atRandom)) p)

/-- `determiners/target.ron` -/
semantic_template target (p : Semantics.Predicate) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.described (Semantics.DetPhrase.target (Semantics.Quantity.range (some 1) (some 1))) p)

/-- `determiners/the.ron` -/
semantic_template the (p : Semantics.Predicate) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.described Semantics.DetPhrase.the p)

/-- `durations/between.ron` -/
semantic_template between (start : Semantics.GameEvent) (end_ : Semantics.GameEvent) : Semantics.Duration :=
  (Semantics.Duration.between (some start) (some end_))

/-- `durations/duringNextTurnOf.ron` -/
semantic_template duringNextTurnOf (player : Semantics.NounPhrase) : Semantics.Duration :=
  (Semantics.Duration.between (some (Semantics.GameEvent.beginningOf Semantics.TurnPart.turn (some (Semantics.NounPhrase.firstMention "duringNextTurnOf.player" player)) (Semantics.TurnRef.next none))) (some (Semantics.GameEvent.endOf Semantics.TurnPart.turn (some (Semantics.NounPhrase.laterMention "duringNextTurnOf.player" player)) (Semantics.TurnRef.next none))))

/-- `durations/earlierThisTurn.ron` -/
semantic_template earlierThisTurn : Semantics.Duration :=
  (Semantics.Duration.between (some (Semantics.GameEvent.beginningOf Semantics.TurnPart.turn none Semantics.TurnRef.current)) (some Semantics.GameEvent.now))

/-- `durations/forAsLongAs.ron` -/
semantic_template forAsLongAs (condition : Semantics.Condition) : Semantics.Duration :=
  (Semantics.Duration.forAsLongAs condition)

/-- `durations/lastTurn.ron` -/
semantic_template lastTurn : Semantics.Duration :=
  (Semantics.Duration.between (some (Semantics.GameEvent.beginningOf Semantics.TurnPart.turn none Semantics.TurnRef.previous)) (some (Semantics.GameEvent.endOf Semantics.TurnPart.turn none Semantics.TurnRef.previous)))

/-- `durations/onALaterTurn.ron` -/
semantic_template onALaterTurn : Semantics.Duration :=
  (Semantics.Duration.between (some (Semantics.GameEvent.endOf Semantics.TurnPart.turn none Semantics.TurnRef.current)) none)

/-- `durations/restOfGame.ron` -/
semantic_template restOfGame : Semantics.Duration :=
  (Semantics.Duration.between none none)

/-- `durations/soFarThisTurn.ron` -/
semantic_template soFarThisTurn : Semantics.Duration :=
  (Semantics.Duration.between (some (Semantics.GameEvent.beginningOf Semantics.TurnPart.turn none Semantics.TurnRef.current)) (some Semantics.GameEvent.now))

/-- `durations/thisCombat.ron` -/
semantic_template thisCombat : Semantics.Duration :=
  (Semantics.Duration.between (some (Semantics.GameEvent.beginningOf Semantics.TurnPart.combat none Semantics.TurnRef.current)) (some Semantics.GameEvent.now))

/-- `durations/thisGame.ron` -/
semantic_template thisGame : Semantics.Duration :=
  (Semantics.Duration.between none (some Semantics.GameEvent.now))

/-- `durations/thisTurn.ron` -/
semantic_template thisTurn : Semantics.Duration :=
  (Semantics.Duration.between none (some (Semantics.GameEvent.endOf Semantics.TurnPart.turn none Semantics.TurnRef.current)))

/-- `durations/untilEndOfCombat.ron` -/
semantic_template untilEndOfCombat : Semantics.Duration :=
  (Semantics.Duration.between none (some (Semantics.GameEvent.endOf Semantics.TurnPart.combat none Semantics.TurnRef.current)))

/-- `durations/untilEndOfTurn.ron` -/
semantic_template untilEndOfTurn : Semantics.Duration :=
  (Semantics.Duration.between none (some (Semantics.GameEvent.endOf Semantics.TurnPart.turn none Semantics.TurnRef.current)))

/-- `durations/untilEvent.ron` -/
semantic_template untilEvent (event : Semantics.GameEvent) : Semantics.Duration :=
  (Semantics.Duration.between none (some event))

/-- `durations/untilNextTurnOf.ron` -/
semantic_template untilNextTurnOf (player : Semantics.NounPhrase) : Semantics.Duration :=
  (Semantics.Duration.between none (some (Semantics.GameEvent.beginningOf Semantics.TurnPart.turn (some player) (Semantics.TurnRef.next none))))

/-- `durations/untilYourNextEndStep.ron` -/
semantic_template untilYourNextEndStep : Semantics.Duration :=
  (Semantics.Duration.between none (some (Semantics.GameEvent.beginningOf Semantics.TurnPart.endStep (some Semantics.NounPhrase.you) (Semantics.TurnRef.next none))))

/-- `durations/untilYourNextTurn.ron` -/
semantic_template untilYourNextTurn : Semantics.Duration :=
  (Semantics.Duration.between none (some (Semantics.GameEvent.beginningOf Semantics.TurnPart.turn (some Semantics.NounPhrase.you) (Semantics.TurnRef.next none))))

/-- `events/activates.ron` -/
semantic_template activates (player : Semantics.NounPhrase) (ability : Semantics.NounPhrase) : Semantics.GameEvent :=
  (Semantics.GameEvent.does (Semantics.Subject.actor player) (Semantics.Instruction.procedure Semantics.Procedure.activate ability Semantics.PlayPayment.itsOwnCost none none none))

/-- `events/attachment.ron` -/
semantic_template GameEvent.attachment (move : Semantics.AttachMove) (subject : Semantics.NounPhrase) (host : Semantics.NounPhrase) : Semantics.GameEvent :=
  (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.attachment move subject (some host)))

/-- `events/attacks.ron` -/
semantic_template attacks (subject : Semantics.NounPhrase) : Semantics.GameEvent :=
  (Semantics.GameEvent.does (Semantics.Subject.actor subject) (Semantics.Instruction.attack none))

/-- `events/attacksPlayer.ron` -/
semantic_template attacksPlayer (subject : Semantics.NounPhrase) (whom : Semantics.NounPhrase) : Semantics.GameEvent :=
  (Semantics.GameEvent.does (Semantics.Subject.actor subject) (Semantics.Instruction.attack (some whom)))

/-- `events/attacksWith.ron` -/
semantic_template attacksWith (player : Semantics.NounPhrase) (attackers : Semantics.NounPhrase) (defender : (Option Semantics.NounPhrase) := none) : Semantics.GameEvent :=
  (Semantics.GameEvent.does (Semantics.Subject.actor player) (Semantics.Instruction.attackWith attackers defender))

/-- `events/bareCounterEvent.ron` -/
semantic_template bareCounterEvent (move : Semantics.CounterMove) (batch : Semantics.CounterBatch) (subject : Semantics.NounPhrase) : Semantics.GameEvent :=
  (match move with | Semantics.CounterMove.put => Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.putCounters Semantics.Amount.anyAmount Semantics.CounterKindSource.anyKind subject batch false) | Semantics.CounterMove.removed => Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.removeCounters none none subject batch false))

/-- `events/becomesBlocked.ron` -/
semantic_template becomesBlocked (subject : Semantics.NounPhrase) (by_ : (Option Semantics.NounPhrase) := none) : Semantics.GameEvent :=
  (Semantics.GameEvent.does (match by_ with | some b => Semantics.Subject.by b | none => Semantics.Subject.any) (Semantics.Instruction.block subject))

/-- `events/becomesTarget.ron` -/
semantic_template becomesTarget (subject : Semantics.NounPhrase) (by_ : Semantics.NounPhrase) : Semantics.GameEvent :=
  (Semantics.GameEvent.does (Semantics.Subject.by by_) (Semantics.Instruction.target subject))

/-- `events/beginningOf.ron` -/
semantic_template beginningOf (part : Semantics.TurnPart) (whose : Semantics.NounPhrase) (which : Semantics.TurnRef) : Semantics.GameEvent :=
  (Semantics.GameEvent.beginningOf part (some whose) which)

/-- `events/beginningOfEach.ron` -/
semantic_template beginningOfEach (part : Semantics.TurnPart) : Semantics.GameEvent :=
  (Semantics.GameEvent.beginningOf part none (Semantics.TurnRef.each none))

/-- `events/beginningOfNext.ron` -/
semantic_template beginningOfNext (part : Semantics.TurnPart) : Semantics.GameEvent :=
  (Semantics.GameEvent.beginningOf part none (Semantics.TurnRef.next none))

/-- `events/beginningOfPossessed.ron` -/
semantic_template beginningOfPossessed (part : Semantics.TurnPart) (possessor : Semantics.NounPhrase) : Semantics.GameEvent :=
  (Semantics.GameEvent.beginningOf part (some possessor) (Semantics.TurnRef.each none))

/-- `events/beginningOfThatTurns.ron` -/
semantic_template beginningOfThatTurns (part : Semantics.TurnPart) : Semantics.GameEvent :=
  (Semantics.GameEvent.beginningOf part none Semantics.TurnRef.that)

/-- `events/beginningOfYour.ron` -/
semantic_template beginningOfYour (part : Semantics.TurnPart) : Semantics.GameEvent :=
  (Semantics.GameEvent.beginningOf part (some Semantics.NounPhrase.you) (Semantics.TurnRef.each none))

/-- `events/blocks.ron` -/
semantic_template blocks (subject : Semantics.NounPhrase) (blocked : (Option Semantics.NounPhrase) := none) : Semantics.GameEvent :=
  (Semantics.GameEvent.does (Semantics.Subject.actor subject) (Semantics.Instruction.block (Option.getD blocked Semantics.NounPhrase.any)))

/-- `events/casts.ron` -/
semantic_template casts (player : Semantics.NounPhrase) (spell : (Option Semantics.NounPhrase) := none) (from_ : (Option Semantics.EventSource) := none) : Semantics.GameEvent :=
  (Semantics.GameEvent.does (Semantics.Subject.actor player) (Semantics.Instruction.procedure Semantics.Procedure.cast (Option.getD spell Semantics.NounPhrase.any) Semantics.PlayPayment.itsOwnCost none from_ none))

/-- `events/causes.ron` -/
semantic_template causes (cause : Semantics.Causing) (event : Semantics.GameEvent) : Semantics.GameEvent :=
  (Semantics.GameEvent.causes cause event)

/-- `events/chapterMark.ron` -/
semantic_template chapterMark (chapters : (List Nat) := []) : Semantics.GameEvent :=
  (Semantics.GameEvent.chapterMark chapters)

/-- `events/combat.ron` -/
semantic_template GameEvent.combat (relation : Semantics.CombatRelation) (subject : Semantics.NounPhrase) (counterpart : (Option Semantics.NounPhrase) := none) : Semantics.GameEvent :=
  (Semantics.GameEvent.does (Semantics.Subject.actor subject) (Semantics.Instruction.attack counterpart))

/-- `events/commitsCrime.ron` -/
semantic_template commitsCrime (player : Semantics.NounPhrase) : Semantics.GameEvent :=
  (Semantics.GameEvent.does (Semantics.Subject.actor player) Semantics.Instruction.commitCrime)

/-- `events/counterEvent.ron` -/
semantic_template counterEvent (move : Semantics.CounterMove) (kind : Semantics.CounterKind) (batch : Semantics.CounterBatch) (subject : Semantics.NounPhrase) : Semantics.GameEvent :=
  (match move with | Semantics.CounterMove.put => Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.putCounters Semantics.Amount.anyAmount (Semantics.CounterKindSource.printed kind) subject batch false) | Semantics.CounterMove.removed => Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.removeCounters none (some (Semantics.CounterKindSource.printed kind)) subject batch false))

/-- `events/damage.ron` -/
semantic_template damage (kind : Semantics.DamageKind) (patient : (Option Semantics.NounPhrase) := none) (source : (Option Semantics.NounPhrase) := none) : Semantics.GameEvent :=
  (Semantics.GameEvent.does (match source with | some s => Semantics.Subject.actor s | none => Semantics.Subject.any) (Semantics.Instruction.dealDamage Semantics.Amount.anyAmount (Option.getD patient Semantics.NounPhrase.any) kind))

/-- `events/dealsCombatDamage.ron` -/
semantic_template dealsCombatDamage (source : Semantics.NounPhrase) (patient : Semantics.NounPhrase) : Semantics.GameEvent :=
  (Semantics.GameEvent.does (Semantics.Subject.actor source) (Semantics.Instruction.dealDamage Semantics.Amount.anyAmount patient Semantics.DamageKind.combatOnly))

/-- `events/dies.ron` -/
semantic_template dies (subject : Semantics.NounPhrase) : Semantics.GameEvent :=
  (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.move subject (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.departure none []))

/-- `events/discards.ron` -/
semantic_template discards (player : Semantics.NounPhrase) (card : Semantics.NounPhrase) : Semantics.GameEvent :=
  (Semantics.GameEvent.does (Semantics.Subject.actor player) (Semantics.Instruction.enact (Semantics.Deed.action "discard") (Semantics.Macros.deonticActionFor (Semantics.Deed.action "discard") card) none none))

/-- `events/draws.ron` -/
semantic_template draws (player : Semantics.NounPhrase) : Semantics.GameEvent :=
  (Semantics.GameEvent.does (Semantics.Subject.actor player) (Semantics.Instruction.draw Semantics.Amount.anyAmount))

/-- `events/endOf.ron` -/
semantic_template endOf (part : Semantics.TurnPart) (whose : Semantics.NounPhrase) (which : Semantics.TurnRef) : Semantics.GameEvent :=
  (Semantics.GameEvent.endOf part (some whose) which)

/-- `events/enters.ron` -/
semantic_template enters (subject : Semantics.NounPhrase) (from_ : (Option Semantics.EventSource) := none) : Semantics.GameEvent :=
  (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.move subject (Option.getD from_ Semantics.EventSource.anywhere) (some (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []))

/-- `events/gameBecomes.ron` -/
semantic_template gameBecomes (designation : Semantics.DesignationLabel) : Semantics.GameEvent :=
  (Semantics.GameEvent.gameBecomes designation)

/-- `events/lastCounterRemoved.ron` -/
semantic_template lastCounterRemoved (kind : Semantics.CounterKind) (subject : Semantics.NounPhrase) : Semantics.GameEvent :=
  (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.removeCounters none (some (Semantics.CounterKindSource.printed kind)) subject Semantics.CounterBatch.emptying false))

/-- `events/lastCounterRemovedBy.ron` -/
semantic_template lastCounterRemovedBy (kind : Semantics.CounterKind) (subject : Semantics.NounPhrase) (who : Semantics.NounPhrase) : Semantics.GameEvent :=
  (Semantics.GameEvent.does (Semantics.Subject.by who) (Semantics.Instruction.removeCounters none (some (Semantics.CounterKindSource.printed kind)) subject Semantics.CounterBatch.emptying false))

/-- `events/leavesZone.ron` -/
semantic_template leavesZone (subject : Semantics.NounPhrase) (zone : Semantics.ZoneExpr) : Semantics.GameEvent :=
  (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.move subject (Semantics.EventSource.zones [zone]) none Semantics.ObservationPoint.departure none []))

/-- `events/lifeChanges.ron` -/
semantic_template lifeChanges (player : Semantics.NounPhrase) (move : Semantics.LifeMove) : Semantics.GameEvent :=
  (Semantics.GameEvent.does (Semantics.Subject.actor player) (Semantics.Instruction.changeLife (match move with | Semantics.LifeMove.up => Semantics.Delta.up Semantics.Amount.anyAmount | Semantics.LifeMove.down => Semantics.Delta.down Semantics.Amount.anyAmount)))

/-- `events/losesGame.ron` -/
semantic_template losesGame (player : Semantics.NounPhrase) : Semantics.GameEvent :=
  (Semantics.GameEvent.does (Semantics.Subject.actor player) (Semantics.Instruction.conclude Semantics.OutcomeVerb.loseGame))

/-- `events/manyBareCountersPutBy.ron` -/
semantic_template manyBareCountersPutBy (who : Semantics.NounPhrase) (subject : Semantics.NounPhrase) : Semantics.GameEvent :=
  (Semantics.GameEvent.does (Semantics.Subject.actor who) (Semantics.Instruction.putCounters Semantics.Amount.anyAmount Semantics.CounterKindSource.anyKind subject Semantics.CounterBatch.many false))

/-- `events/manyCountersPutByEffect.ron` -/
semantic_template manyCountersPutByEffect (subject : Semantics.NounPhrase) : Semantics.GameEvent :=
  (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.putCounters Semantics.Amount.anyAmount Semantics.CounterKindSource.anyKind subject Semantics.CounterBatch.many true))

/-- `events/nthOccurrence.ron` -/
semantic_template nthOccurrence (ordinal : Semantics.Ordinal) (event : Semantics.GameEvent) (per : (Option Semantics.TurnPart) := none) : Semantics.GameEvent :=
  (Semantics.GameEvent.nthOccurrence ordinal per event)

/-- `events/paysCost.ron` -/
semantic_template paysCost (outcome : Semantics.PaymentOutcome) (whose : Semantics.NounPhrase) (keyword : Semantics.KeywordLabel) (player : (Option Semantics.NounPhrase) := none) : Semantics.GameEvent :=
  (Semantics.GameEvent.does (match player with | some p => Semantics.Subject.actor p | none => Semantics.Subject.any) (Semantics.Instruction.pay Semantics.Cost.itsManaCost Semantics.PayTimes.once outcome (some whose) (some keyword)))

/-- `events/paysLife.ron` -/
semantic_template paysLife (player : Semantics.NounPhrase) : Semantics.GameEvent :=
  (Semantics.GameEvent.does (Semantics.Subject.actor player) (Semantics.Instruction.changeLife (Semantics.Delta.down Semantics.Amount.anyAmount)))

/-- `events/regenerates.ron` -/
semantic_template regenerates (subject : Semantics.NounPhrase) : Semantics.GameEvent :=
  (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.enact (Semantics.Deed.action "regenerate") (Semantics.Macros.deonticActionFor (Semantics.Deed.action "regenerate") subject) none none))

/-- `events/statBecomes.ron` -/
semantic_template statBecomes (subject : Semantics.NounPhrase) (stat : Semantics.Stat) (value : Semantics.Amount) : Semantics.GameEvent :=
  (Semantics.GameEvent.statBecomes subject stat value)

/-- `events/stateHolds.ron` -/
semantic_template stateHolds (condition : Semantics.Condition) : Semantics.GameEvent :=
  (Semantics.GameEvent.stateHolds condition)

/-- `events/statusEvent.ron` -/
semantic_template statusEvent (subject : Semantics.NounPhrase) (status : Semantics.Status) : Semantics.GameEvent :=
  (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.setStatus status subject))

/-- `events/tappedForMana.ron` -/
semantic_template tappedForMana (source : Semantics.NounPhrase) (player : (Option Semantics.NounPhrase) := none) (type : (Option Semantics.ManaTypeTerm) := none) : Semantics.GameEvent :=
  (Semantics.GameEvent.does (match player with | some p => Semantics.Subject.by p | none => Semantics.Subject.any) (Semantics.Instruction.tapForMana source type))

/-- `events/tokensCreated.ron` -/
semantic_template tokensCreated (tokens : Semantics.NounPhrase) : Semantics.GameEvent :=
  (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.createObject Semantics.Amount.anyAmount (Semantics.CreationSpec.token (Semantics.TokenSpec.copyOf tokens []) []) false none))

/-- `events/tokensCreatedByEffectUnder.ron` -/
semantic_template tokensCreatedByEffectUnder (tokens : Semantics.NounPhrase) (under : Semantics.NounPhrase) : Semantics.GameEvent :=
  (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.createObject Semantics.Amount.anyAmount (Semantics.CreationSpec.token (Semantics.TokenSpec.copyOf tokens []) []) true (some under)))

/-- `events/triggers.ron` -/
semantic_template triggers (ability : Semantics.NounPhrase) : Semantics.GameEvent :=
  (Semantics.GameEvent.triggers ability none none)

/-- `events/unlocksDoor.ron` -/
semantic_template unlocksDoor (player : Semantics.NounPhrase) (door : Semantics.Door) : Semantics.GameEvent :=
  (Semantics.GameEvent.does (Semantics.Subject.actor player) (Semantics.Instruction.unlock door))

/-- `events/verbedEvent.ron` -/
semantic_template verbedEvent (verb : Semantics.Deed) (patient : (Option Semantics.NounPhrase) := none) (agent : (Option Semantics.NounPhrase) := none) (becomes : (Option Semantics.Predicate) := none) (locus : (Option Semantics.ZoneExpr) := none) : Semantics.GameEvent :=
  (Semantics.GameEvent.does (match agent with | some a => Semantics.Subject.actor a | none => Semantics.Subject.any) (Semantics.Instruction.enact verb (Semantics.Macros.deonticActionFor verb (Option.getD patient Semantics.NounPhrase.any)) becomes locus))

/-- `events/youRollHighestNatural.ron` -/
semantic_template youRollHighestNatural : Semantics.GameEvent :=
  (Semantics.GameEvent.does (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Instruction.rollDice (Semantics.Amount.lit 1) (Semantics.DieSides.sides 6) Semantics.DiceBatch.one Semantics.RollWatch.highestNatural))

/-- `events/youRollResultIn.ron` -/
semantic_template youRollResultIn (quantity : Semantics.Quantity) : Semantics.GameEvent :=
  (Semantics.GameEvent.does (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Instruction.rollDice (Semantics.Amount.lit 1) (Semantics.DieSides.sides 6) Semantics.DiceBatch.one (Semantics.RollWatch.resultIn quantity)))

/-- `events/zoneChange.ron` -/
semantic_template zoneChange (subject : Semantics.NounPhrase) (observation : Semantics.ObservationPoint) (from_ : (Option Semantics.EventSource) := none) (to : (Option Semantics.ZoneExpr) := none) : Semantics.GameEvent :=
  (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.move subject (Option.getD from_ Semantics.EventSource.anywhere) to observation none []))

/-- `face_down/exiledFaceDown.ron` -/
semantic_template exiledFaceDown : Semantics.FaceDown :=
  ({ characteristics := none } : Semantics.FaceDown)

/-- `face_down/faceDownCreature.ron` -/
semantic_template faceDownCreature : Semantics.FaceDown :=
  ({ characteristics := (some ({ name := none, cost := none, colors := [], supertypes := [], types := [Semantics.CardType.creature], subtypes := [], text := [], power := (some (Semantics.Amount.lit 2)), toughness := (some (Semantics.Amount.lit 2)), loyalty := none, defense := none } : Semantics.Characteristics)) } : Semantics.FaceDown)

/-- `face_down/faceDownWardedCreature.ron` -/
semantic_template faceDownWardedCreature : Semantics.FaceDown :=
  ({ characteristics := (some ({ name := none, cost := none, colors := [], supertypes := [], types := [Semantics.CardType.creature], subtypes := [], text := [(Semantics.Ability.keyword (Semantics.KeywordLabel.named "ward") [(Semantics.KeywordParam.cost (Semantics.Cost.mana [(Semantics.ManaSymbol.simple (Semantics.SimpleManaSymbol.generic 2))]))] [])], power := (some (Semantics.Amount.lit 2)), toughness := (some (Semantics.Amount.lit 2)), loyalty := none, defense := none } : Semantics.Characteristics)) } : Semantics.FaceDown)

/-- `instructions/act.ron` -/
semantic_template act (performer : Semantics.NounPhrase) (body : Semantics.Instruction) : Semantics.Instruction :=
  (Semantics.Instruction.instruct performer body)

/-- `instructions/addMana.ron` -/
semantic_template addMana (amount : Semantics.Amount) (produced : Semantics.ProducedMana) (riders : (List Semantics.ManaRider) := []) : Semantics.Instruction :=
  (Semantics.Instruction.addMana amount produced riders)

/-- `instructions/addTurnPart.ron` -/
semantic_template addTurnPart (part : Semantics.TurnPart) (count : Semantics.Amount) (anchor : (Option Semantics.TurnPart) := none) (followedBy : (Option Semantics.TurnPart) := none) : Semantics.Instruction :=
  (Semantics.Instruction.addTurnPart part anchor count followedBy)

/-- `instructions/additionalPhase.ron` -/
semantic_template additionalPhase (part : Semantics.TurnPart) (count : Semantics.Amount) (anchor : (Option Semantics.TurnPart) := none) (followedBy : (Option Semantics.TurnPart) := none) : Semantics.Instruction :=
  (Semantics.Instruction.addTurnPart part anchor count followedBy)

/-- `instructions/additionalStep.ron` -/
semantic_template additionalStep (part : Semantics.TurnPart) (count : Semantics.Amount) (anchor : (Option Semantics.TurnPart) := none) : Semantics.Instruction :=
  (Semantics.Instruction.addTurnPart part anchor count none)

/-- `instructions/artifactCreatureToken.ron` -/
semantic_template artifactCreatureToken (power : Nat) (toughness : Nat) (colors : (List Semantics.Color)) (subtypes : (List Semantics.Subtype)) (abilities : (List Semantics.Ability) := []) : Semantics.CharacteristicBundle :=
  ({ characteristics := ({ name := none, cost := none, colors := colors, supertypes := [], types := [Semantics.CardType.artifact, Semantics.CardType.creature], subtypes := subtypes, text := abilities, power := (some (Semantics.Amount.lit power)), toughness := (some (Semantics.Amount.lit toughness)), loyalty := none, defense := none } : Semantics.Characteristics), qualities := [] } : Semantics.CharacteristicBundle)

/-- `instructions/atNext.ron` -/
semantic_template atNext (part : Semantics.TurnPart) (instruction : Semantics.Instruction) : Semantics.Instruction :=
  (Semantics.Instruction.delay (Semantics.GameEvent.beginningOf part none (Semantics.TurnRef.next none)) [] none instruction)

/-- `instructions/attachment.ron` -/
semantic_template Instruction.attachment (move : Semantics.AttachMove) (subject : Semantics.NounPhrase) (host : (Option Semantics.NounPhrase) := none) : Semantics.Instruction :=
  (Semantics.Instruction.attachment move subject host)

/-- `instructions/bracketed.ron` -/
semantic_template Instruction.bracketed (instruction : Semantics.Instruction) : Semantics.Instruction :=
  (Semantics.Instruction.bracketed instruction)

/-- `instructions/canBeTargetedAsThough.ron` -/
semantic_template canBeTargetedAsThough (what : Semantics.NounPhrase) (by_ : Semantics.NounPhrase) (p : Semantics.Predicate) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.may (Semantics.Subject.by by_) (Semantics.Macros.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.target) what) ({ bound := none, asThough := (some (Semantics.AsThough.of p)), rider := Semantics.DeonticRider.noRider, except := none } : Semantics.DeonticMods))

/-- `instructions/canDoAsThough.ron` -/
semantic_template canDoAsThough (n : Semantics.NounPhrase) (deed : Semantics.Deed) (p : Semantics.Predicate) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.may (Semantics.Subject.actor n) (Semantics.Macros.deonticActionFor deed Semantics.NounPhrase.any) ({ bound := none, asThough := (some (Semantics.AsThough.of p)), rider := Semantics.DeonticRider.noRider, except := none } : Semantics.DeonticMods))

/-- `instructions/cantBeTargetedBy.ron` -/
semantic_template cantBeTargetedBy (what : Semantics.NounPhrase) (by_ : Semantics.NounPhrase) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.cant (Semantics.Subject.by by_) (Semantics.Macros.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.target) what) ({ bound := none, asThough := none, rider := Semantics.DeonticRider.noRider, except := none } : Semantics.DeonticMods))

/-- `instructions/cantDoTo.ron` -/
semantic_template cantDoTo (deed : Semantics.Deed) (who : Semantics.NounPhrase) (what : Semantics.NounPhrase) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.cant (Semantics.Subject.actor who) (Semantics.Macros.deonticActionFor deed what) ({ bound := none, asThough := none, rider := Semantics.DeonticRider.noRider, except := none } : Semantics.DeonticMods))

/-- `instructions/cantMoreThan.ron` -/
semantic_template cantMoreThan (player : Semantics.NounPhrase) (deed : Semantics.Deed) (bound : Nat) (p : Semantics.Predicate) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.cant (Semantics.Subject.actor player) (Semantics.Macros.deonticActionFor deed (Semantics.NounPhrase.described Semantics.DetPhrase.all p)) ({ bound := (some (Semantics.CountBound.moreThan (Semantics.Amount.lit bound))), asThough := none, rider := Semantics.DeonticRider.noRider, except := none } : Semantics.DeonticMods))

/-- `instructions/castWithoutPaying.ron` -/
semantic_template castWithoutPaying (subject : Semantics.NounPhrase) (resulting : (Option Semantics.Predicate) := none) : Semantics.Instruction :=
  (Semantics.Instruction.enact (Semantics.Deed.action "cast") (Semantics.Instruction.procedure Semantics.Procedure.cast subject Semantics.PlayPayment.withoutPaying resulting none none) none none)

/-- `instructions/changeLife.ron` -/
semantic_template changeLife (delta : (Semantics.Delta Semantics.Amount)) : Semantics.Instruction :=
  (Semantics.Instruction.changeLife delta)

/-- `instructions/choose.ron` -/
semantic_template choose (subject : Semantics.NounPhrase) (disclosure : Semantics.Disclosure := Semantics.Disclosure.openly) : Semantics.Instruction :=
  (Semantics.Instruction.choose none subject disclosure none)

/-- `instructions/chooseNewTargets.ron` -/
semantic_template chooseNewTargets (subject : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.chooseNewTargets subject)

/-- `instructions/chooseOne.ron` -/
semantic_template chooseOne (first : Semantics.Instruction) (second : Semantics.Instruction) : Semantics.Instruction :=
  (Semantics.Instruction.chooseModes (Semantics.Quantity.range (some 1) (some 1)) [(none, none, first), (none, none, second)])

/-- `instructions/chooseWhile.ron` -/
semantic_template chooseWhile (subject : Semantics.NounPhrase) (while_ : Semantics.Timing) : Semantics.Instruction :=
  (Semantics.Instruction.choose none subject Semantics.Disclosure.openly (some while_))

/-- `instructions/chosenNumber.ron` -/
semantic_template chosenNumber : Semantics.Amount :=
  (Semantics.Amount.chosenNumber Semantics.ChoiceRef.theChoice)

/-- `instructions/clearDamage.ron` -/
semantic_template clearDamage (subject : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.clearDamage subject)

/-- `instructions/coloredManaSpentToCast.ron` -/
semantic_template coloredManaSpentToCast (spell : Semantics.NounPhrase) : Semantics.Condition :=
  (Semantics.Condition.compareAmt (Semantics.Amount.paid Semantics.LinkedFacet.colorsSpent spell) Semantics.Comparator.atLeast (Semantics.Amount.lit 1))

/-- `instructions/colorsSpentToCast.ron` -/
semantic_template colorsSpentToCast (spell : Semantics.NounPhrase) : Semantics.Amount :=
  (Semantics.Amount.paid Semantics.LinkedFacet.colorsSpent spell)

/-- `instructions/combat.ron` -/
semantic_template Instruction.combat (subject : Semantics.NounPhrase) (update : Semantics.CombatUpdate) : Semantics.Instruction :=
  (Semantics.Instruction.combat subject update)

/-- `instructions/conclude.ron` -/
semantic_template conclude (verb : Semantics.OutcomeVerb) : Semantics.Instruction :=
  (Semantics.Instruction.conclude verb)

/-- `instructions/copySpell.ron` -/
semantic_template copySpell (subject : Semantics.NounPhrase) (times : Semantics.Amount := (Semantics.Amount.lit 1)) : Semantics.Instruction :=
  (Semantics.Instruction.copy Semantics.CopySort.fromStack subject times [])

/-- `instructions/copyTargets.ron` -/
semantic_template copyTargets (copy : Semantics.NounPhrase) (original : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.copyTargets copy original)

/-- `instructions/costWasPaid.ron` -/
semantic_template costWasPaid (which : Semantics.LinkSource) (subject : Semantics.NounPhrase) (window : (Option Semantics.Duration) := none) : Semantics.Condition :=
  (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback which window) subject) Semantics.Comparator.atLeast (Semantics.Amount.lit 1))

/-- `instructions/createDoubleFacedToken.ron` -/
semantic_template createDoubleFacedToken (front : Semantics.CharacteristicBundle) (back : Semantics.CharacteristicBundle) (riders : (List Semantics.EntryRider) := []) : Semantics.Instruction :=
  (Semantics.Instruction.createObject (Semantics.Amount.lit 1) (Semantics.CreationSpec.token (Semantics.TokenSpec.doubleFaced front back) riders) true none)

/-- `instructions/createObject.ron` -/
semantic_template createObject (count : Semantics.Amount) (spec : Semantics.CreationSpec) : Semantics.Instruction :=
  (Semantics.Instruction.createObject count spec true none)

/-- `instructions/createTappedAttacking.ron` -/
semantic_template createTappedAttacking (count : Semantics.Amount) (token : Semantics.CharacteristicBundle) : Semantics.Instruction :=
  (Semantics.Instruction.createObject count (Semantics.CreationSpec.token (Semantics.TokenSpec.written token) [(Semantics.EntryRider.entersAs Semantics.Status.tapped), (Semantics.EntryRider.entersAttacking none)]) true none)

/-- `instructions/createTappedToken.ron` -/
semantic_template createTappedToken (characteristics : Semantics.CharacteristicBundle) (count : Semantics.Amount := (Semantics.Amount.lit 1)) : Semantics.Instruction :=
  (Semantics.Instruction.createObject count (Semantics.CreationSpec.token (Semantics.TokenSpec.written characteristics) [(Semantics.EntryRider.entersAs Semantics.Status.tapped)]) true none)

/-- `instructions/createToken.ron` -/
semantic_template createToken (characteristics : Semantics.CharacteristicBundle) (count : Semantics.Amount := (Semantics.Amount.lit 1)) : Semantics.Instruction :=
  (Semantics.Instruction.createObject count (Semantics.CreationSpec.token (Semantics.TokenSpec.written characteristics) []) true none)

/-- `instructions/createTokenCopy.ron` -/
semantic_template createTokenCopy (source : Semantics.NounPhrase) (riders : (List Semantics.EntryRider) := []) : Semantics.Instruction :=
  (Semantics.Instruction.createObject (Semantics.Amount.lit 1) (Semantics.CreationSpec.token (Semantics.TokenSpec.copyOf source []) riders) true none)

/-- `instructions/creatureToken.ron` -/
semantic_template creatureToken (power : Nat) (toughness : Nat) (colors : (List Semantics.Color)) (subtypes : (List Semantics.Subtype)) (abilities : (List Semantics.Ability) := []) : Semantics.CharacteristicBundle :=
  ({ characteristics := ({ name := none, cost := none, colors := colors, supertypes := [], types := [Semantics.CardType.creature], subtypes := subtypes, text := abilities, power := (some (Semantics.Amount.lit power)), toughness := (some (Semantics.Amount.lit toughness)), loyalty := none, defense := none } : Semantics.Characteristics), qualities := [] } : Semantics.CharacteristicBundle)

/-- `instructions/creatureTokenOf.ron` -/
semantic_template creatureTokenOf (power : Semantics.Amount) (toughness : Semantics.Amount) (colors : (List Semantics.Color)) (subtypes : (List Semantics.Subtype)) (abilities : (List Semantics.Ability) := []) : Semantics.CharacteristicBundle :=
  ({ characteristics := ({ name := none, cost := none, colors := colors, supertypes := [], types := [Semantics.CardType.creature], subtypes := subtypes, text := abilities, power := (some power), toughness := (some toughness), loyalty := none, defense := none } : Semantics.Characteristics), qualities := [] } : Semantics.CharacteristicBundle)

/-- `instructions/dealDamage.ron` -/
semantic_template dealDamage (source : Semantics.NounPhrase) (amount : Semantics.Amount) (recipient : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.instruct source (Semantics.Instruction.dealDamage amount recipient Semantics.DamageKind.any))

/-- `instructions/dealDivided.ron` -/
semantic_template dealDivided (source : Semantics.NounPhrase) (amount : Semantics.Amount) (among : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.distribute (Semantics.DividedVerb.damage source) amount among)

/-- `instructions/delay.ron` -/
semantic_template delay (event : Semantics.GameEvent) (instruction : Semantics.Instruction) (alternatives : (List Semantics.GameEvent) := []) : Semantics.Instruction :=
  (Semantics.Instruction.delay event alternatives none instruction)

/-- `instructions/delayWithin.ron` -/
semantic_template delayWithin (event : Semantics.GameEvent) (duration : Semantics.Duration) (instruction : Semantics.Instruction) : Semantics.Instruction :=
  (Semantics.Instruction.delay event [] (some duration) instruction)

/-- `instructions/discardThenForEachNonland.ron` -/
semantic_template discardThenForEachNonland (discarded : Semantics.NounPhrase) (step : Semantics.Instruction) : Semantics.Instruction :=
  (Semantics.Instruction.sequentially [(Semantics.Macros.Actor.discard discarded), (Semantics.Instruction.doForEach (Semantics.Macros.allOf (Semantics.Predicate.and [(Semantics.Predicate.not Semantics.Macros.land), Semantics.Predicate.isCard, (Semantics.Predicate.did (Semantics.DidClause.mk Semantics.Subject.any (Semantics.Macros.Actor.discard Semantics.NounPhrase.which) Semantics.LookbackSpan.thisWay))])) step)])

/-- `instructions/distribute.ron` -/
semantic_template distribute (verb : Semantics.DividedVerb) (amount : Semantics.Amount) (among : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.distribute verb amount among)

/-- `instructions/distributeCounters.ron` -/
semantic_template distributeCounters (amount : Semantics.Amount) (kind : Semantics.CounterKind) (among : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.distribute (Semantics.DividedVerb.counters kind) amount among)

/-- `instructions/doAndForbid.ron` -/
semantic_template doAndForbid (instruction : Semantics.Instruction) (deed : Semantics.Deed) (subject : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.doAndForbid instruction deed subject)

/-- `instructions/doForEach.ron` -/
semantic_template doForEach (group : Semantics.NounPhrase) (body : Semantics.Instruction) : Semantics.Instruction :=
  (Semantics.Instruction.doForEach group body)

/-- `instructions/doForEachKind.ron` -/
semantic_template doForEachKind (axis : Semantics.KindAxis) (sort : Semantics.QualitySort) (body : Semantics.Instruction) (domain : (Option Semantics.NounPhrase) := none) : Semantics.Instruction :=
  (Semantics.Instruction.doForEachKind axis domain sort body)

/-- `instructions/doIf.ron` -/
semantic_template doIf (condition : Semantics.Condition) (instruction : Semantics.Instruction) (otherwise : (Option Semantics.Instruction) := none) : Semantics.Instruction :=
  (Semantics.Instruction.doIf condition instruction otherwise)

/-- `instructions/doIfDone.ron` -/
semantic_template doIfDone (body : Semantics.Instruction) (ifDid : (Option Semantics.Instruction) := none) (ifNot : (Option Semantics.Instruction) := none) : Semantics.Instruction :=
  (Semantics.Instruction.withContinuation Semantics.ContinuationPolicy.required body ifDid ifNot)

/-- `instructions/doOnlyIf.ron` -/
semantic_template doOnlyIf (instruction : Semantics.Instruction) (condition : Semantics.Condition) (otherwise : (Option Semantics.Instruction) := none) : Semantics.Instruction :=
  (Semantics.Instruction.doOnlyIf instruction condition otherwise)

/-- `instructions/doOnlyOnceEachTurn.ron` -/
semantic_template doOnlyOnceEachTurn (instruction : Semantics.Instruction) : Semantics.Instruction :=
  (Semantics.Instruction.limited instruction (Semantics.UsageLimit.mk (Semantics.CountBound.moreThan (Semantics.Amount.lit 1)) Semantics.Period.turn))

/-- `instructions/doUnless.ron` -/
semantic_template doUnless (instruction : Semantics.Instruction) (cost : Semantics.Cost) : Semantics.Instruction :=
  (Semantics.Instruction.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Instruction.pay cost Semantics.PayTimes.once Semantics.PaymentOutcome.paid none none) none (some instruction))

/-- `instructions/doubleCounters.ron` -/
semantic_definition doubleCounters (on : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "double")) (Semantics.Instruction.doubleCounters (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "doubleCounters.on" on))))

/-- `instructions/doubleLifeTotal.ron` -/
semantic_definition doubleLifeTotal : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "double")) (Semantics.Instruction.changeLife (Semantics.Delta.set (Semantics.Amount.arith Semantics.ArithOp.times (Semantics.Amount.lit 2) (Semantics.Amount.statOf (Semantics.ProjAxis.playerStat Semantics.PlayerStat.lifeTotal) Semantics.NounPhrase.actor)))))

/-- `instructions/doublePower.ron` -/
semantic_template doublePower (subject : Semantics.NounPhrase) (duration : (Option Semantics.Duration) := none) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "double")) (Semantics.Instruction.establish (Semantics.StaticSpec.ptModification (Semantics.NounPhrase.firstMention "doublePower.subject" (Semantics.NounPhrase.inCaller subject)) (Semantics.Delta.up (Semantics.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.power) (Semantics.NounPhrase.laterMention "doublePower.subject" (Semantics.NounPhrase.inCaller subject)))) (Semantics.Delta.up (Semantics.Amount.lit 0))) duration))

/-- `instructions/doublePowerAndToughness.ron` -/
semantic_template doublePowerAndToughness (subject : Semantics.NounPhrase) (duration : (Option Semantics.Duration) := none) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "double")) (Semantics.Instruction.establish (Semantics.StaticSpec.ptModification (Semantics.NounPhrase.firstMention "doublePowerAndToughness.subject" (Semantics.NounPhrase.inCaller subject)) (Semantics.Delta.up (Semantics.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.power) (Semantics.NounPhrase.laterMention "doublePowerAndToughness.subject" (Semantics.NounPhrase.inCaller subject)))) (Semantics.Delta.up (Semantics.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.toughness) (Semantics.NounPhrase.laterMention "doublePowerAndToughness.subject" (Semantics.NounPhrase.inCaller subject))))) duration))

/-- `instructions/doubleToughness.ron` -/
semantic_template doubleToughness (subject : Semantics.NounPhrase) (duration : (Option Semantics.Duration) := none) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "double")) (Semantics.Instruction.establish (Semantics.StaticSpec.ptModification (Semantics.NounPhrase.firstMention "doubleToughness.subject" (Semantics.NounPhrase.inCaller subject)) (Semantics.Delta.up (Semantics.Amount.lit 0)) (Semantics.Delta.up (Semantics.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.toughness) (Semantics.NounPhrase.laterMention "doubleToughness.subject" (Semantics.NounPhrase.inCaller subject))))) duration))

/-- `instructions/draw.ron` -/
semantic_template draw (amount : Semantics.Amount) : Semantics.Instruction :=
  (Semantics.Instruction.draw amount)

/-- `instructions/drawGame.ron` -/
semantic_template drawGame : Semantics.Instruction :=
  Semantics.Instruction.drawGame

/-- `instructions/enact.ron` -/
semantic_template enact (verb : Semantics.Deed) (instruction : Semantics.Instruction) : Semantics.Instruction :=
  (Semantics.Instruction.enact verb instruction none none)

/-- `instructions/establish.ron` -/
semantic_template establish (spec : Semantics.StaticSpec) (duration : (Option Semantics.Duration) := none) : Semantics.Instruction :=
  (Semantics.Instruction.establish spec duration)

/-- `instructions/establishFor.ron` -/
semantic_template establishFor (parts : (List Semantics.StaticSpec)) (duration : Semantics.Duration) (subject : (Option Semantics.NounPhrase) := none) : Semantics.Instruction :=
  (Semantics.Instruction.establish (Semantics.StaticSpec.conjunction subject parts) (some duration))

/-- `instructions/establishThroughout.ron` -/
semantic_template establishThroughout (spec : Semantics.StaticSpec) (duration : Semantics.Duration) : Semantics.Instruction :=
  (Semantics.Instruction.establish spec (some duration))

/-- `instructions/eventCount.ron` -/
semantic_template eventCount (action : Semantics.Instruction) (subj : Semantics.Subject) (within : Semantics.Duration) (timing : (Option Semantics.Timing) := none) : Semantics.Amount :=
  (Semantics.Amount.tally (Semantics.DidClause.mk subj action (Semantics.LookbackSpan.duration within timing)) Semantics.TallyMode.events)

/-- `instructions/eventSum.ron` -/
semantic_template eventSum (action : Semantics.Instruction) (subj : Semantics.Subject) (within : Semantics.Duration) (timing : (Option Semantics.Timing) := none) : Semantics.Amount :=
  (Semantics.Amount.tally (Semantics.DidClause.mk subj action (Semantics.LookbackSpan.duration within timing)) Semantics.TallyMode.quantity)

/-- `instructions/exchangeCards.ron` -/
semantic_definition exchangeCards (left : Semantics.NounPhrase) (right : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "exchange")) (Semantics.Instruction.enact (Semantics.Deed.action "exchange") (Semantics.Instruction.exchange (Semantics.Exchanged.cardsAcross (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "exchangeCards.left" left)) (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "exchangeCards.right" right)))) none none))

/-- `instructions/exchangeControl.ron` -/
semantic_definition exchangeControl (left : Semantics.NounPhrase) (right : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "exchange")) (Semantics.Instruction.enact (Semantics.Deed.action "exchange") (Semantics.Instruction.exchange (Semantics.Exchanged.controlOf (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "exchangeControl.left" left)) (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "exchangeControl.right" right)))) none none))

/-- `instructions/exchangeLifeTotals.ron` -/
semantic_definition exchangeLifeTotals (parties : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "exchange")) (Semantics.Instruction.enact (Semantics.Deed.action "exchange") (Semantics.Instruction.exchange (Semantics.Exchanged.lifeTotals (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "exchangeLifeTotals.parties" parties)))) none none))

/-- `instructions/exchangeTextBoxes.ron` -/
semantic_definition exchangeTextBoxes (left : Semantics.NounPhrase) (right : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "exchange")) (Semantics.Instruction.enact (Semantics.Deed.action "exchange") (Semantics.Instruction.establish (Semantics.StaticSpec.textChange (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "exchangeTextBoxes.left" left)) (Semantics.TextChange.exchangeWith (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "exchangeTextBoxes.right" right)))) none) none none))

/-- `instructions/exchangeValues.ron` -/
semantic_definition exchangeValues (left : Semantics.Amount) (right : Semantics.Amount) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "exchange")) (Semantics.Instruction.enact (Semantics.Deed.action "exchange") (Semantics.Instruction.exchange (Semantics.Exchanged.values (Semantics.Amount.inCaller left) (Semantics.Amount.inCaller right))) none none))

/-- `instructions/exchangeZones.ron` -/
semantic_definition exchangeZones (left : Semantics.ZoneExpr) (right : Semantics.ZoneExpr) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "exchange")) (Semantics.Instruction.enact (Semantics.Deed.action "exchange") (Semantics.Instruction.exchange (Semantics.Exchanged.zones (Semantics.ZoneExpr.inCaller left) (Semantics.ZoneExpr.inCaller right))) none none))

/-- `instructions/exileFaceDown.ron` -/
semantic_definition exileFaceDown (subject : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "exile")) (Semantics.Instruction.enact (Semantics.Deed.action "exile") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "exileFaceDown.subject" subject)) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.exile Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none [(Semantics.MoveRider.placement (Semantics.PilePlacement.faceDown ({ characteristics := none } : Semantics.FaceDown)))]) none none))

/-- `instructions/exileFrom.ron` -/
semantic_definition exileFrom (subject : Semantics.NounPhrase) (from_ : Semantics.ZoneExpr) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "exile")) (Semantics.Instruction.enact (Semantics.Deed.action "exile") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "exileFrom.subject" subject)) (Semantics.EventSource.zones [(Semantics.ZoneExpr.inCaller from_)]) (some (Semantics.ZoneExpr.zone Semantics.Zone.exile Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) none none))

/-- `instructions/exileUntil.ron` -/
semantic_template exileUntil (subject : Semantics.NounPhrase) (event : Semantics.GameEvent) : Semantics.Instruction :=
  (Semantics.Instruction.holdUntil (Semantics.Instruction.ownScope (some (Semantics.Deed.action "exile")) (Semantics.Instruction.enact (Semantics.Deed.action "exile") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller subject) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.exile Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) none none)) event)

/-- `instructions/exileWithCounters.ron` -/
semantic_definition exileWithCounters (subject : Semantics.NounPhrase) (amount : Semantics.Amount) (kind : Semantics.CounterKind) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "exile")) (Semantics.Instruction.enact (Semantics.Deed.action "exile") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "exileWithCounters.subject" subject)) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.exile Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none [(Semantics.MoveRider.placement (Semantics.PilePlacement.withCounters (Semantics.Amount.inCaller amount) (Semantics.CounterKindSource.printed kind) Semantics.EntryCounterMark.fresh))]) none none))

/-- `instructions/expose.ron` -/
semantic_template expose (verb : Semantics.ExposeVerb) (exposed : Semantics.Exposed) : Semantics.Instruction :=
  (Semantics.Instruction.expose verb exposed)

/-- `instructions/extraTurn.ron` -/
semantic_template extraTurn (count : Semantics.Amount) : Semantics.Instruction :=
  (Semantics.Instruction.addTurnPart Semantics.TurnPart.turn none count none)

/-- `instructions/flipCoins.ron` -/
semantic_template flipCoins (count : Nat) : Semantics.Instruction :=
  (Semantics.Instruction.flipCoins (Semantics.FlipScope.count (Semantics.Amount.lit count)) none none)

/-- `instructions/flipsCoin.ron` -/
semantic_template flipsCoin (player : Semantics.NounPhrase) : Semantics.GameEvent :=
  (Semantics.GameEvent.does (Semantics.Subject.actor player) (Semantics.Instruction.flipCoins (Semantics.FlipScope.count (Semantics.Amount.lit 1)) none none))

/-- `instructions/foodToken.ron` -/
semantic_template foodToken : Semantics.CharacteristicBundle :=
  ({ characteristics := ({ name := none, cost := none, colors := [], supertypes := [], types := [Semantics.CardType.artifact], subtypes := [(Semantics.Subtype.of Semantics.CardType.artifact "Food")], text := [(Semantics.Ability.activated (Semantics.Cost.compound [(Semantics.Cost.mana [(Semantics.ManaSymbol.simple (Semantics.SimpleManaSymbol.generic 2))]), Semantics.Cost.tapSymbol, (Semantics.Cost.perform (Semantics.Instruction.ownScope (some (Semantics.Deed.action "sacrifice")) (Semantics.Instruction.enact (Semantics.Deed.action "sacrifice") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asType Semantics.CardType.artifact Semantics.NounPhrase.this none)) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.departure none []) none none)))]) (Semantics.Instruction.changeLife (Semantics.Delta.up (Semantics.Amount.lit 3))) none none none none)], power := none, toughness := none, loyalty := none, defense := none } : Semantics.Characteristics), qualities := [] } : Semantics.CharacteristicBundle)

/-- `instructions/forbidAttack.ron` -/
semantic_template forbidAttack (subject : Semantics.NounPhrase) (duration : Semantics.Duration) : Semantics.Instruction :=
  (Semantics.Instruction.establish (Semantics.StaticSpec.cant (Semantics.Subject.actor subject) (Semantics.Macros.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.attack) Semantics.NounPhrase.any) ({ bound := none, asThough := none, rider := Semantics.DeonticRider.noRider, except := none } : Semantics.DeonticMods)) (some duration))

/-- `instructions/forbidBeingBlocked.ron` -/
semantic_template forbidBeingBlocked (subject : Semantics.NounPhrase) (duration : Semantics.Duration) : Semantics.Instruction :=
  (Semantics.Instruction.establish (Semantics.StaticSpec.cant Semantics.Subject.any (Semantics.Macros.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.block) subject) ({ bound := none, asThough := none, rider := Semantics.DeonticRider.noRider, except := none } : Semantics.DeonticMods)) (some duration))

/-- `instructions/forbidBlock.ron` -/
semantic_template forbidBlock (subject : Semantics.NounPhrase) (duration : Semantics.Duration) : Semantics.Instruction :=
  (Semantics.Instruction.establish (Semantics.StaticSpec.cant (Semantics.Subject.actor subject) (Semantics.Macros.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.block) Semantics.NounPhrase.any) ({ bound := none, asThough := none, rider := Semantics.DeonticRider.noRider, except := none } : Semantics.DeonticMods)) (some duration))

/-- `instructions/fromTo.ron` -/
semantic_template fromTo (low : Nat) (high : Nat) : Semantics.Quantity :=
  (Semantics.Quantity.range (some low) (some high))

/-- `instructions/gain.ron` -/
semantic_template gain (subject : Semantics.NounPhrase) (ability : Semantics.Ability) (duration : (Option Semantics.Duration) := none) : Semantics.Instruction :=
  (Semantics.Instruction.establish (Semantics.StaticSpec.abilityGrant subject ability) duration)

/-- `instructions/gainControl.ron` -/
semantic_template gainControl (subject : Semantics.NounPhrase) (duration : Semantics.Duration) : Semantics.Instruction :=
  (Semantics.Instruction.establish (Semantics.StaticSpec.controlGrant Semantics.NounPhrase.actor subject) (some duration))

/-- `instructions/gainDesignation.ron` -/
semantic_template gainDesignation (subject : Semantics.NounPhrase) (designation : Semantics.DesignationLabel) (duration : (Option Semantics.Duration) := none) : Semantics.Instruction :=
  (Semantics.Instruction.gainDesignation subject designation duration)

/-- `instructions/gainHaste.ron` -/
semantic_template gainHaste (subject : Semantics.NounPhrase) (duration : Semantics.Duration) : Semantics.Instruction :=
  (Semantics.Instruction.establish (Semantics.StaticSpec.abilityGrant subject (Semantics.Ability.keyword (Semantics.KeywordLabel.named "haste") [] [])) (some duration))

/-- `instructions/gainLife.ron` -/
semantic_template gainLife (amount : Semantics.Amount) : Semantics.Instruction :=
  (Semantics.Instruction.changeLife (Semantics.Delta.up amount))

/-- `instructions/getNamedEmblem.ron` -/
semantic_template getNamedEmblem (name : String) (abilities : (List Semantics.Ability)) : Semantics.Instruction :=
  (Semantics.Instruction.createObject (Semantics.Amount.lit 1) (Semantics.CreationSpec.emblem (some name) abilities) true none)

/-- `instructions/gets.ron` -/
semantic_template gets (subject : Semantics.NounPhrase) (power : (Semantics.Delta Semantics.Amount)) (toughness : (Semantics.Delta Semantics.Amount)) (duration : (Option Semantics.Duration) := none) : Semantics.Instruction :=
  (Semantics.Instruction.establish (Semantics.StaticSpec.ptModification subject power toughness) duration)

/-- `instructions/getsBoth.ron` -/
semantic_template getsBoth (subject : Semantics.NounPhrase) (delta : (Semantics.Delta Semantics.Amount)) (duration : (Option Semantics.Duration) := none) : Semantics.Instruction :=
  (Semantics.Instruction.establish (Semantics.StaticSpec.ptModification subject delta delta) duration)

/-- `instructions/happened.ron` -/
semantic_template happened (action : Semantics.Instruction) (subj : Semantics.Subject) (within : Semantics.Duration) (timing : (Option Semantics.Timing) := none) : Semantics.Condition :=
  (Semantics.Condition.did (Semantics.DidClause.mk subj action (Semantics.LookbackSpan.duration within timing)))

/-- `instructions/happenedThisWay.ron` -/
semantic_template happenedThisWay (action : Semantics.Instruction) (subj : Semantics.Subject) : Semantics.Condition :=
  (Semantics.Condition.did (Semantics.DidClause.mk subj action Semantics.LookbackSpan.thisWay))

/-- `instructions/happenedTo.ron` -/
semantic_template happenedTo (subj : Semantics.Subject) (action : Semantics.Instruction) (within : Semantics.Duration) (timing : (Option Semantics.Timing) := none) : Semantics.Predicate :=
  (Semantics.Predicate.did (Semantics.DidClause.mk subj action (Semantics.LookbackSpan.duration within timing)))

/-- `instructions/holdUntil.ron` -/
semantic_template holdUntil (instruction : Semantics.Instruction) (event : Semantics.GameEvent) : Semantics.Instruction :=
  (Semantics.Instruction.holdUntil instruction event)

/-- `instructions/ignoreOutcomes.ron` -/
semantic_template ignoreOutcomes (which : Semantics.IgnoredOutcomes) : Semantics.Instruction :=
  (Semantics.Instruction.ignoreOutcomes which)

/-- `instructions/limited.ron` -/
semantic_template limited (instruction : Semantics.Instruction) (limit : Semantics.UsageLimit) : Semantics.Instruction :=
  (Semantics.Instruction.limited instruction limit)

/-- `instructions/lookAndSortSlice.ron` -/
semantic_template lookAndSortSlice (slice : Semantics.NounPhrase) (spill : Semantics.ZoneExpr) (spillRiders : (List Semantics.MoveRider)) : Semantics.Instruction :=
  (Semantics.Instruction.sequentially [(Semantics.Instruction.expose Semantics.ExposeVerb.lookAt (Semantics.Exposed.cards (Semantics.NounPhrase.firstMention "lookAndSortSlice.slice" slice))), (Semantics.Instruction.move (Semantics.NounPhrase.someOf (Semantics.SliceCount.counted (Semantics.Quantity.range none none)) none (Semantics.NounPhrase.laterMention "lookAndSortSlice.slice" slice)) Semantics.EventSource.anywhere (some spill) Semantics.ObservationPoint.arrival none spillRiders), (Semantics.Instruction.move (Semantics.NounPhrase.theRest Semantics.Kind.object Semantics.Plurality.many) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.place Semantics.LibraryEnd.top (Semantics.Pile.library Semantics.ZoneScope.bare))) Semantics.ObservationPoint.arrival none [(Semantics.MoveRider.placement (Semantics.PilePlacement.inOrder Semantics.Arrangement.anyOrder))])])

/-- `instructions/lookAt.ron` -/
semantic_template lookAt (cards : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.expose Semantics.ExposeVerb.lookAt (Semantics.Exposed.cards cards))

/-- `instructions/lookAtHandOf.ron` -/
semantic_template lookAtHandOf (player : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.expose Semantics.ExposeVerb.lookAt (Semantics.Exposed.zone (Semantics.ZoneExpr.zone Semantics.Zone.hand (Semantics.ZoneScope.possessedBy player))))

/-- `instructions/loseLife.ron` -/
semantic_template loseLife (amount : Semantics.Amount) : Semantics.Instruction :=
  (Semantics.Instruction.changeLife (Semantics.Delta.down amount))

/-- `instructions/makeMonstrous.ron` -/
semantic_template makeMonstrous (amount : Semantics.Amount) : Semantics.Instruction :=
  (Semantics.Instruction.doIf (Semantics.Condition.not (Semantics.Condition.matches (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.Predicate.hasDesignation (Semantics.DesignationLabel.named "monstrous") none))) (Semantics.Instruction.sequentially [(Semantics.Instruction.putCounters amount (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.CounterBatch.one true), (Semantics.Instruction.gainDesignation (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.DesignationLabel.named "monstrous") none)]) none)

/-- `instructions/manaValueSpentToCast.ron` -/
semantic_template manaValueSpentToCast (spell : Semantics.NounPhrase) : Semantics.Amount :=
  (Semantics.Amount.paid Semantics.LinkedFacet.manaValueSpent spell)

/-- `instructions/manifestPlacement.ron` -/
semantic_template manifestPlacement (subject : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.enact (Semantics.Deed.action "manifest") (Semantics.Instruction.move subject Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) none none)

/-- `instructions/may.ron` -/
semantic_template may (body : Semantics.Instruction) (ifDid : (Option Semantics.Instruction) := none) (ifNot : (Option Semantics.Instruction) := none) : Semantics.Instruction :=
  (Semantics.Instruction.withContinuation Semantics.ContinuationPolicy.optional body ifDid ifNot)

/-- `instructions/mayBlockAdditional.ron` -/
semantic_template mayBlockAdditional (subject : Semantics.NounPhrase) (quantity : Semantics.Quantity) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.may (Semantics.Subject.actor subject) (Semantics.Macros.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.block) (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.hasType Semantics.CardType.creature))) ({ bound := (some (Semantics.CountBound.additional quantity)), asThough := none, rider := Semantics.DeonticRider.noRider, except := none } : Semantics.DeonticMods))

/-- `instructions/mayOrElse.ron` -/
semantic_template mayOrElse (body : Semantics.Instruction) (otherwise : Semantics.Instruction) : Semantics.Instruction :=
  (Semantics.Instruction.withContinuation Semantics.ContinuationPolicy.optional body none (some otherwise))

/-- `instructions/mayPlayAdditionalLands.ron` -/
semantic_template mayPlayAdditionalLands (player : Semantics.NounPhrase) (quantity : Semantics.Quantity) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.may (Semantics.Subject.actor player) (Semantics.Macros.deonticActionFor (Semantics.Deed.action "play") (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.hasType Semantics.CardType.land))) ({ bound := (some (Semantics.CountBound.additional quantity)), asThough := none, rider := Semantics.DeonticRider.noRider, except := none } : Semantics.DeonticMods))

/-- `instructions/mayPlayDeed.ron` -/
semantic_template mayPlayDeed (deed : Semantics.Deed) (player : Semantics.NounPhrase) (what : Semantics.NounPhrase) (rider : Semantics.DeonticRider) (asThough : (Option Semantics.AsThough) := none) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.may (Semantics.Subject.actor player) (Semantics.Macros.deonticActionFor deed what) ({ bound := none, asThough := asThough, rider := rider, except := none } : Semantics.DeonticMods))

/-- `instructions/maySpendAsThough.ron` -/
semantic_template maySpendAsThough (who : Semantics.NounPhrase) (what : Semantics.ColorOrColorless) (as_ : Semantics.ManaMatch) (purpose : Semantics.SpendPurpose) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.may (Semantics.Subject.actor who) (Semantics.Instruction.pay (Semantics.Cost.mana []) Semantics.PayTimes.once Semantics.PaymentOutcome.paid none none) ({ bound := none, asThough := (some (Semantics.AsThough.mana (some what) as_ (some purpose))), rider := Semantics.DeonticRider.noRider, except := none } : Semantics.DeonticMods))

/-- `instructions/mayVoteAdditional.ron` -/
semantic_template mayVoteAdditional (who : Semantics.NounPhrase) (quantity : Semantics.Quantity) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.may (Semantics.Subject.actor who) (Semantics.Macros.vote Semantics.Disclosure.openly (Semantics.Ballot.byLabel [])) ({ bound := (some (Semantics.CountBound.additional quantity)), asThough := none, rider := Semantics.DeonticRider.noRider, except := none } : Semantics.DeonticMods))

/-- `instructions/meldInto.ron` -/
semantic_template meldInto (subject : Semantics.NounPhrase) (into : String) : Semantics.Instruction :=
  (Semantics.Instruction.enact (Semantics.Deed.action "meld") (Semantics.Instruction.move subject Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none [(Semantics.MoveRider.entry (Semantics.EntryRider.entersMelded into))]) none none)

/-- `instructions/merge.ron` -/
semantic_template merge (subject : Semantics.NounPhrase) (onto : Semantics.NounPhrase) (position : Semantics.MergePosition) : Semantics.Instruction :=
  (Semantics.Instruction.merge subject onto position)

/-- `instructions/move.ron` -/
semantic_template move (subject : Semantics.NounPhrase) (from_ : Semantics.ZoneExpr) (to : Semantics.ZoneExpr) (riders : (List Semantics.MoveRider) := []) : Semantics.Instruction :=
  (Semantics.Instruction.move subject (Semantics.EventSource.zones [from_]) (some to) Semantics.ObservationPoint.arrival none riders)

/-- `instructions/moveCounters.ron` -/
semantic_template moveCounters (amount : Semantics.Amount) (source : Semantics.NounPhrase) (destination : Semantics.NounPhrase) (kind : (Option Semantics.CounterKindSource) := none) : Semantics.Instruction :=
  (Semantics.Instruction.moveCounters amount kind source destination)

/-- `instructions/noColoredManaSpentToCast.ron` -/
semantic_template noColoredManaSpentToCast (spell : Semantics.NounPhrase) : Semantics.Condition :=
  (Semantics.Condition.compareAmt (Semantics.Amount.paid Semantics.LinkedFacet.colorsSpent spell) Semantics.Comparator.eq (Semantics.Amount.lit 0))

/-- `instructions/noManaSpentToCast.ron` -/
semantic_template noManaSpentToCast (spell : Semantics.NounPhrase) : Semantics.Condition :=
  (Semantics.Condition.compareAmt (Semantics.Amount.paid Semantics.LinkedFacet.manaValueSpent spell) Semantics.Comparator.eq (Semantics.Amount.lit 0))

/-- `instructions/objectCant.ron` -/
semantic_template objectCant (deed : Semantics.Deed) (subject : Semantics.NounPhrase) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.cant Semantics.Subject.any (Semantics.Macros.deonticActionFor deed subject) ({ bound := none, asThough := none, rider := Semantics.DeonticRider.noRider, except := none } : Semantics.DeonticMods))

/-- `instructions/onlyIfSo.ron` -/
semantic_template onlyIfSo (spec : Semantics.StaticSpec) (condition : Semantics.Condition) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.conditional spec condition Semantics.CondMarking.ifSo)

/-- `instructions/onlyUnless.ron` -/
semantic_template onlyUnless (spec : Semantics.StaticSpec) (condition : Semantics.Condition) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.conditional spec (Semantics.Condition.not condition) Semantics.CondMarking.unless_)

/-- `instructions/onlyWhile.ron` -/
semantic_template onlyWhile (spec : Semantics.StaticSpec) (condition : Semantics.Condition) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.conditional spec condition Semantics.CondMarking.asLongAs)

/-- `instructions/paidCostRead.ron` -/
semantic_template paidCostRead (which : Semantics.LinkSource) (subject : Semantics.NounPhrase) (window : (Option Semantics.Duration) := none) : Semantics.Amount :=
  (Semantics.Amount.paid (Semantics.LinkedFacet.readback which window) subject)

/-- `instructions/pay.ron` -/
semantic_template pay (cost : Semantics.Cost) (times : Semantics.PayTimes) : Semantics.Instruction :=
  (Semantics.Instruction.pay cost times Semantics.PaymentOutcome.paid none none)

/-- `instructions/phaseOutUntil.ron` -/
semantic_template phaseOutUntil (subject : Semantics.NounPhrase) (event : Semantics.GameEvent) : Semantics.Instruction :=
  (Semantics.Instruction.holdUntil (Semantics.Instruction.setStatus Semantics.Status.phasedOut subject) event)

/-- `instructions/playerCant.ron` -/
semantic_template playerCant (deed : Semantics.Deed) (player : Semantics.NounPhrase) : Semantics.StaticSpec :=
  (Semantics.StaticSpec.cant (Semantics.Subject.actor player) (Semantics.Macros.deonticActionFor deed Semantics.NounPhrase.any) ({ bound := none, asThough := none, rider := Semantics.DeonticRider.noRider, except := none } : Semantics.DeonticMods))

/-- `instructions/preventAll.ron` -/
semantic_template preventAll (kind : Semantics.DamageKind) (scope : Semantics.DamageScope) (duration : Semantics.Duration) : Semantics.Instruction :=
  (Semantics.Instruction.establish (Semantics.StaticSpec.damageRule kind Semantics.DamageAgent.unattributed scope (Semantics.DamageOp.prevent Semantics.PreventCut.all none) Semantics.ReplUse.repeatedly) (some duration))

/-- `instructions/preventAllBy.ron` -/
semantic_template preventAllBy (kind : Semantics.DamageKind) (source : Semantics.NounPhrase) (scope : Semantics.DamageScope) (duration : Semantics.Duration) : Semantics.Instruction :=
  (Semantics.Instruction.establish (Semantics.StaticSpec.damageRule kind (Semantics.DamageAgent.dealtBy source) scope (Semantics.DamageOp.prevent Semantics.PreventCut.all none) Semantics.ReplUse.repeatedly) (some duration))

/-- `instructions/preventNext.ron` -/
semantic_template preventNext (kind : Semantics.DamageKind) (scope : Semantics.DamageScope) (amount : Semantics.Amount) (duration : Semantics.Duration) : Semantics.Instruction :=
  (Semantics.Instruction.establish (Semantics.StaticSpec.damageRule kind Semantics.DamageAgent.unattributed scope (Semantics.DamageOp.prevent (Semantics.PreventCut.shield amount) none) Semantics.ReplUse.repeatedly) (some duration))

/-- `instructions/procedure.ron` -/
semantic_template procedure (kind : Semantics.Procedure) (subject : Semantics.NounPhrase) (payment : Semantics.PlayPayment) (resulting : (Option Semantics.Predicate) := none) : Semantics.Instruction :=
  (Semantics.Instruction.procedure kind subject payment resulting none none)

/-- `instructions/put.ron` -/
semantic_template put (subject : Semantics.NounPhrase) (destination : Semantics.ZoneExpr) : Semantics.Instruction :=
  (Semantics.Instruction.enact (Semantics.Deed.core Semantics.CoreDeed.put) (Semantics.Instruction.move subject Semantics.EventSource.anywhere (some destination) Semantics.ObservationPoint.arrival none []) none none)

/-- `instructions/putCounters.ron` -/
semantic_template putCounters (amount : Semantics.Amount) (kind : Semantics.CounterKindSource) (on : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.putCounters amount kind on Semantics.CounterBatch.one true)

/-- `instructions/putOntoBattlefield.ron` -/
semantic_template putOntoBattlefield (subject : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.move subject Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none [])

/-- `instructions/putOntoBattlefieldTapped.ron` -/
semantic_template putOntoBattlefieldTapped (subject : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.move subject Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none [(Semantics.MoveRider.entry (Semantics.EntryRider.entersAs Semantics.Status.tapped))])

/-- `instructions/putOntoBattlefieldTappedAttacking.ron` -/
semantic_template putOntoBattlefieldTappedAttacking (subject : Semantics.NounPhrase) (from_ : Semantics.ZoneExpr := Semantics.ZoneExpr.wherever) : Semantics.Instruction :=
  (Semantics.Instruction.move subject (Semantics.EventSource.zones [from_]) (some (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none [(Semantics.MoveRider.entry (Semantics.EntryRider.entersAs Semantics.Status.tapped)), (Semantics.MoveRider.entry (Semantics.EntryRider.entersAttacking none))])

/-- `instructions/putOntoBattlefieldUnderYourControl.ron` -/
semantic_template putOntoBattlefieldUnderYourControl (subject : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.move subject Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none [(Semantics.MoveRider.entry (Semantics.EntryRider.under Semantics.NounPhrase.you))])

/-- `instructions/regenerationApplication.ron` -/
semantic_template regenerationApplication (subject : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.enact (Semantics.Deed.action "regenerate") (Semantics.Instruction.sequentially [(Semantics.Instruction.clearDamage (Semantics.NounPhrase.firstMention "regenerationApplication.subject" subject)), (Semantics.Instruction.instruct (Semantics.NounPhrase.possessorOf Semantics.PossessorAxis.controller (Semantics.NounPhrase.laterMention "regenerationApplication.subject" subject)) (Semantics.Instruction.enact (Semantics.Deed.action "tap") (Semantics.Instruction.setStatus Semantics.Status.tapped (Semantics.NounPhrase.laterMention "regenerationApplication.subject" subject)) none none)), (Semantics.Instruction.doIf (Semantics.Condition.or [(Semantics.Condition.matches (Semantics.NounPhrase.laterMention "regenerationApplication.subject" subject) (Semantics.Predicate.inCombat Semantics.CombatRelation.attackerOf none)), (Semantics.Condition.matches (Semantics.NounPhrase.laterMention "regenerationApplication.subject" subject) (Semantics.Predicate.inCombat Semantics.CombatRelation.blockerOf none))]) (Semantics.Instruction.combat (Semantics.NounPhrase.laterMention "regenerationApplication.subject" subject) (Semantics.CombatUpdate.participation Semantics.CombatParticipation.outsideCombat)) none)]) none none)

/-- `instructions/relate.ron` -/
semantic_template relate (subject : Semantics.NounPhrase) (label : Semantics.RelationLabel) (counterpart : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.relate subject label counterpart)

/-- `instructions/relative.ron` -/
semantic_template relative (kind : Semantics.Kind) : Semantics.NounPhrase :=
  Semantics.NounPhrase.which

/-- `instructions/removeAllCounters.ron` -/
semantic_template removeAllCounters (kind : Semantics.CounterKindSource) (from_ : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.removeCounters none (some kind) from_ Semantics.CounterBatch.one true)

/-- `instructions/removeCounters.ron` -/
semantic_template removeCounters (quantity : Semantics.Quantity) (kind : Semantics.CounterKindSource) (from_ : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.removeCounters (some quantity) (some kind) from_ Semantics.CounterBatch.one true)

/-- `instructions/removedThisWay.ron` -/
semantic_template removedThisWay : Semantics.Amount :=
  (Semantics.Amount.theOutcome Semantics.OutcomeSort.countersRemoved)

/-- `instructions/repeat.ron` -/
semantic_template repeat_ (repetition : Semantics.Repetition) : Semantics.Instruction :=
  (Semantics.Instruction.repeat_ repetition)

/-- `instructions/replace.ron` -/
semantic_template replace (replaced : Semantics.Instruction) (replacement : Semantics.Instruction) : Semantics.Instruction :=
  (Semantics.Instruction.replace replaced replacement)

/-- `instructions/replaceEvent.ron` -/
semantic_template replaceEvent (clause : Semantics.WouldClause) (replacement : Semantics.Instruction) (duration : (Option Semantics.Duration) := none) : Semantics.Instruction :=
  (Semantics.Instruction.establish (Semantics.StaticSpec.instead clause replacement none Semantics.ReplUse.repeatedly none) duration)

/-- `instructions/replaceNextEvent.ron` -/
semantic_template replaceNextEvent (clause : Semantics.WouldClause) (replacement : Semantics.Instruction) (duration : Semantics.Duration) : Semantics.Instruction :=
  (Semantics.Instruction.establish (Semantics.StaticSpec.instead clause replacement none Semantics.ReplUse.nextTimeOnly none) (some duration))

/-- `instructions/rerollStored.ron` -/
semantic_template rerollStored (quantity : Semantics.Quantity) (whose : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.rerollStored quantity whose)

/-- `instructions/restartGame.ron` -/
semantic_template restartGame : Semantics.Instruction :=
  Semantics.Instruction.restartGame

/-- `instructions/returnTo.ron` -/
semantic_template returnTo (subject : Semantics.NounPhrase) (destination : Semantics.ZoneExpr) (riders : (List Semantics.MoveRider)) : Semantics.Instruction :=
  (Semantics.Instruction.enact (Semantics.Deed.core Semantics.CoreDeed.return_) (Semantics.Instruction.move subject Semantics.EventSource.anywhere (some destination) Semantics.ObservationPoint.arrival none riders) none none)

/-- `instructions/returnToBattlefield.ron` -/
semantic_template returnToBattlefield (subject : Semantics.NounPhrase) (riders : (List Semantics.MoveRider) := []) (from_ : Semantics.ZoneExpr := Semantics.ZoneExpr.wherever) : Semantics.Instruction :=
  (Semantics.Instruction.enact (Semantics.Deed.core Semantics.CoreDeed.return_) (Semantics.Instruction.move subject (Semantics.EventSource.zones [from_]) (some (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none riders) none none)

/-- `instructions/returnToBattlefieldTransformed.ron` -/
semantic_template returnToBattlefieldTransformed (subject : Semantics.NounPhrase) (controller : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.enact (Semantics.Deed.core Semantics.CoreDeed.return_) (Semantics.Instruction.move subject Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none [(Semantics.MoveRider.entry Semantics.EntryRider.entersTransformed), (Semantics.MoveRider.entry (Semantics.EntryRider.under controller))]) none none)

/-- `instructions/returnToHand.ron` -/
semantic_template returnToHand (subject : Semantics.NounPhrase) (from_ : Semantics.ZoneExpr := Semantics.ZoneExpr.wherever) : Semantics.Instruction :=
  (Semantics.Instruction.enact (Semantics.Deed.core Semantics.CoreDeed.return_) (Semantics.Instruction.move subject (Semantics.EventSource.zones [from_]) (some (Semantics.ZoneExpr.zone Semantics.Zone.hand Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) none none)

/-- `instructions/revealCards.ron` -/
semantic_template revealCards (cards : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.expose Semantics.ExposeVerb.reveal (Semantics.Exposed.cards cards))

/-- `instructions/revealChoices.ron` -/
semantic_template revealChoices (sort : Semantics.HiddenSort) : Semantics.Instruction :=
  (Semantics.Instruction.revealChoices sort)

/-- `instructions/revealHand.ron` -/
semantic_template revealHand : Semantics.Instruction :=
  (Semantics.Instruction.expose Semantics.ExposeVerb.reveal (Semantics.Exposed.zone (Semantics.ZoneExpr.zone Semantics.Zone.hand (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.actor))))

/-- `instructions/rollDice.ron` -/
semantic_template rollDice (count : Nat) (sides : Nat) : Semantics.Instruction :=
  (Semantics.Instruction.rollDice (Semantics.Amount.lit count) (Semantics.DieSides.sides sides) Semantics.DiceBatch.one Semantics.RollWatch.anyResult)

/-- `instructions/searchLibraryFor.ron` -/
semantic_template searchLibraryFor (quantity : Semantics.Quantity) (p : Semantics.Predicate) : Semantics.Instruction :=
  (Semantics.Instruction.search (Semantics.SearchScope.oneZone (Semantics.ZoneExpr.zone Semantics.Zone.library (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.you))) quantity p)

/-- `instructions/searchLibraryOrGraveyard.ron` -/
semantic_template searchLibraryOrGraveyard (p : Semantics.Predicate) : Semantics.Instruction :=
  (Semantics.Instruction.search (Semantics.SearchScope.someZones (some Semantics.NounPhrase.you) [Semantics.Zone.library, Semantics.Zone.graveyard]) (Semantics.Quantity.range (some 1) (some 1)) p)

/-- `instructions/searchZonesOf.ron` -/
semantic_template searchZonesOf (whose : Semantics.NounPhrase) (quantity : Semantics.Quantity) (p : Semantics.Predicate) : Semantics.Instruction :=
  (Semantics.Instruction.search (Semantics.SearchScope.someZones (some whose) [Semantics.Zone.graveyard, Semantics.Zone.hand, Semantics.Zone.library]) quantity p)

/-- `instructions/selectRandom.ron` -/
semantic_template selectRandom (quantity : Semantics.Quantity) (what : Semantics.Predicate) : Semantics.Instruction :=
  (Semantics.Instruction.choose none (Semantics.NounPhrase.described (Semantics.DetPhrase.count quantity (some Semantics.ChoiceMode.atRandom)) what) Semantics.Disclosure.openly none)

/-- `instructions/separateIntoPiles.ron` -/
semantic_template separateIntoPiles (group : Semantics.NounPhrase) (piles : Nat) (faces : (List Semantics.PileFace) := []) : Semantics.Instruction :=
  (Semantics.Instruction.separateIntoPiles group piles faces)

/-- `instructions/sequentially.ron` -/
semantic_template sequentially (steps : (List Semantics.Instruction) := []) : Semantics.Instruction :=
  (Semantics.Instruction.sequentially steps)

/-- `instructions/setGameDesignation.ron` -/
semantic_template setGameDesignation (designation : Semantics.DesignationLabel) : Semantics.Instruction :=
  (Semantics.Instruction.setGameDesignation designation)

/-- `instructions/setLife.ron` -/
semantic_template setLife (amount : Semantics.Amount) : Semantics.Instruction :=
  (Semantics.Instruction.changeLife (Semantics.Delta.set amount))

/-- `instructions/setStatus.ron` -/
semantic_template setStatus (status : Semantics.Status) (subject : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.setStatus status subject)

/-- `instructions/shiftResult.ron` -/
semantic_template shiftResult (amount : Semantics.Amount) : Semantics.Instruction :=
  (Semantics.Instruction.shiftResult none amount)

/-- `instructions/shortOfCeiling.ron` -/
semantic_template shortOfCeiling : Semantics.Amount :=
  (Semantics.Amount.theOutcome Semantics.OutcomeSort.ceilingShortfall)

/-- `instructions/shuffleInto.ron` -/
semantic_template shuffleInto (subject : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.sequentially [(Semantics.Instruction.enact (Semantics.Deed.action "shuffle") (Semantics.Instruction.move subject Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.library Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) none none), (Semantics.Instruction.shuffle (Semantics.Pile.library Semantics.ZoneScope.bare))])

/-- `instructions/simultaneously.ron` -/
semantic_template simultaneously (steps : (List Semantics.Instruction) := []) : Semantics.Instruction :=
  (Semantics.Instruction.simultaneously steps)

/-- `instructions/skipPart.ron` -/
semantic_template skipPart (part : Semantics.TurnPart) (count : Semantics.Amount) : Semantics.Instruction :=
  (Semantics.Instruction.skipPart part count)

/-- `instructions/skipUntap.ron` -/
semantic_template skipUntap (subject : Semantics.NounPhrase) (steps : Semantics.Amount) : Semantics.Instruction :=
  (Semantics.Instruction.skipUntap subject steps)

/-- `instructions/storeResults.ron` -/
semantic_template storeResults (on : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.storeResults on)

/-- `instructions/theLastChosenColor.ron` -/
semantic_template theLastChosenColor : Semantics.ColorTerm :=
  (Semantics.ColorTerm.chosen Semantics.ChoiceRef.theLatestChoice)

/-- `instructions/theLastChosenNumber.ron` -/
semantic_template theLastChosenNumber : Semantics.Amount :=
  (Semantics.Amount.chosenNumber Semantics.ChoiceRef.theLatestChoice)

/-- `instructions/thereIsNo.ron` -/
semantic_template thereIsNo (designation : Semantics.DesignationLabel) : Semantics.Condition :=
  (Semantics.Condition.noHolder designation)

/-- `instructions/timesPaid.ron` -/
semantic_template timesPaid (which : Semantics.LinkSource) (subject : Semantics.NounPhrase) : Semantics.Amount :=
  (Semantics.Amount.paid (Semantics.LinkedFacet.timesPaid which) subject)

/-- `instructions/treasureToken.ron` -/
semantic_template treasureToken : Semantics.CharacteristicBundle :=
  ({ characteristics := ({ name := none, cost := none, colors := [], supertypes := [], types := [Semantics.CardType.artifact], subtypes := [(Semantics.Subtype.of Semantics.CardType.artifact "Treasure")], text := [(Semantics.Ability.activated (Semantics.Cost.compound [Semantics.Cost.tapSymbol, (Semantics.Cost.perform (Semantics.Instruction.ownScope (some (Semantics.Deed.action "sacrifice")) (Semantics.Instruction.enact (Semantics.Deed.action "sacrifice") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asType Semantics.CardType.artifact Semantics.NounPhrase.this none)) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.departure none []) none none)))]) (Semantics.Instruction.addMana (Semantics.Amount.lit 1) (Semantics.ProducedMana.anyColor Semantics.ColorFreedom.sameColor) []) none none none none)], power := none, toughness := none, loyalty := none, defense := none } : Semantics.Characteristics), qualities := [] } : Semantics.CharacteristicBundle)

/-- `instructions/triggerReflexively.ron` -/
semantic_template triggerReflexively (body : Semantics.Instruction) (triggerBody : Semantics.Instruction) : Semantics.Instruction :=
  (Semantics.Instruction.triggerReflexively body triggerBody)

/-- `instructions/triggerThisWay.ron` -/
semantic_template triggerThisWay (body : Semantics.Instruction) (event : Semantics.GameEvent) (triggerBody : Semantics.Instruction) : Semantics.Instruction :=
  (Semantics.Instruction.triggerThisWay body event triggerBody)

/-- `instructions/triplePower.ron` -/
semantic_template triplePower (subject : Semantics.NounPhrase) (duration : (Option Semantics.Duration) := none) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "triple")) (Semantics.Instruction.establish (Semantics.StaticSpec.ptModification (Semantics.NounPhrase.firstMention "triplePower.subject" (Semantics.NounPhrase.inCaller subject)) (Semantics.Delta.up (Semantics.Amount.arith Semantics.ArithOp.times (Semantics.Amount.lit 2) (Semantics.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.power) (Semantics.NounPhrase.laterMention "triplePower.subject" (Semantics.NounPhrase.inCaller subject))))) (Semantics.Delta.up (Semantics.Amount.lit 0))) duration))

/-- `instructions/triplePowerAndToughness.ron` -/
semantic_template triplePowerAndToughness (subject : Semantics.NounPhrase) (duration : (Option Semantics.Duration) := none) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "triple")) (Semantics.Instruction.establish (Semantics.StaticSpec.ptModification (Semantics.NounPhrase.firstMention "triplePowerAndToughness.subject" (Semantics.NounPhrase.inCaller subject)) (Semantics.Delta.up (Semantics.Amount.arith Semantics.ArithOp.times (Semantics.Amount.lit 2) (Semantics.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.power) (Semantics.NounPhrase.laterMention "triplePowerAndToughness.subject" (Semantics.NounPhrase.inCaller subject))))) (Semantics.Delta.up (Semantics.Amount.arith Semantics.ArithOp.times (Semantics.Amount.lit 2) (Semantics.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.toughness) (Semantics.NounPhrase.laterMention "triplePowerAndToughness.subject" (Semantics.NounPhrase.inCaller subject)))))) duration))

/-- `instructions/tripleToughness.ron` -/
semantic_template tripleToughness (subject : Semantics.NounPhrase) (duration : (Option Semantics.Duration) := none) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "triple")) (Semantics.Instruction.establish (Semantics.StaticSpec.ptModification (Semantics.NounPhrase.firstMention "tripleToughness.subject" (Semantics.NounPhrase.inCaller subject)) (Semantics.Delta.up (Semantics.Amount.lit 0)) (Semantics.Delta.up (Semantics.Amount.arith Semantics.ArithOp.times (Semantics.Amount.lit 2) (Semantics.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.toughness) (Semantics.NounPhrase.laterMention "tripleToughness.subject" (Semantics.NounPhrase.inCaller subject)))))) duration))

/-- `instructions/turnFaceDown.ron` -/
semantic_template turnFaceDown (subject : Semantics.NounPhrase) (spec : Semantics.FaceDown) : Semantics.Instruction :=
  (Semantics.Instruction.turnFaceDown subject spec)

/-- `instructions/turnOver.ron` -/
semantic_template turnOver (subject : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.turnOver subject)

/-- `instructions/unlock.ron` -/
semantic_template unlock (door : Semantics.Door) : Semantics.Instruction :=
  (Semantics.Instruction.unlock door)

/-- `instructions/voteStartingWith.ron` -/
semantic_definition voteStartingWith (first : Semantics.NounPhrase) (disclosure : Semantics.Disclosure) (ballot : Semantics.Ballot) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "vote")) (Semantics.Instruction.enact (Semantics.Deed.action "vote") (Semantics.Instruction.vote (some (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "voteStartingWith.first" first))) disclosure ballot) none none))

/-- `instructions/withContinuation.ron` -/
semantic_template withContinuation (policy : Semantics.ContinuationPolicy) (body : Semantics.Instruction) (ifDid : (Option Semantics.Instruction) := none) (ifNot : (Option Semantics.Instruction) := none) : Semantics.Instruction :=
  (Semantics.Instruction.withContinuation policy body ifDid ifNot)

/-- `keyword_abilities/absorb.ron` -/
semantic_definition absorb (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "absorb") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.damageRule Semantics.DamageKind.any Semantics.DamageAgent.unattributed (Semantics.DamageScope.toRecipient (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) (Semantics.DamageOp.prevent (Semantics.PreventCut.some (Semantics.Amount.inCaller arg0)) none) Semantics.ReplUse.repeatedly))])

/-- `keyword_abilities/affinity.ron` -/
semantic_definition affinity (arg0 : Semantics.Predicate) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "affinity") [(Semantics.KeywordParam.quality (Semantics.Predicate.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.costShift (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) (Semantics.CostShift.less (Semantics.Amount.arith Semantics.ArithOp.times (Semantics.Amount.lit 1) (Semantics.Amount.countOf (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.and [(Semantics.Predicate.inCaller arg0), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)])))) none)))])

/-- `keyword_abilities/afflict.ron` -/
semantic_definition afflict (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "afflict") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0))] [(Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.block (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none))) [] none [] none none none (Semantics.Instruction.instruct (Semantics.NounPhrase.combatPlayer Semantics.CombatRole.defending) (Semantics.Instruction.changeLife (Semantics.Delta.down (Semantics.Amount.inCaller arg0)))))])

/-- `keyword_abilities/afterlife.ron` -/
semantic_definition afterlife (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "afterlife") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0))] [(Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.departure none [])) [] none [] none none none (Semantics.Instruction.createObject (Semantics.Amount.inCaller arg0) (Semantics.CreationSpec.token (Semantics.TokenSpec.written ({ characteristics := ({ name := none, cost := none, colors := [Semantics.Color.white, Semantics.Color.black], supertypes := [], types := [Semantics.CardType.creature], subtypes := [(Semantics.Subtype.of Semantics.CardType.creature "Spirit")], text := [(Semantics.Ability.keyword (Semantics.KeywordLabel.named "flying") [] [(Semantics.Ability.static (Semantics.StaticSpec.cant Semantics.Subject.any (Semantics.Macros.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.block) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) ({ bound := none, asThough := none, rider := Semantics.DeonticRider.noRider, except := (some (Semantics.Cause.byObjects (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.or [(Semantics.Predicate.hasKeyword (Semantics.KeywordTerm.the (Semantics.KeywordLabel.named "flying"))), (Semantics.Predicate.hasKeyword (Semantics.KeywordTerm.the (Semantics.KeywordLabel.named "reach")))])]))) } : Semantics.DeonticMods)))])], power := (some (Semantics.Amount.lit 1)), toughness := (some (Semantics.Amount.lit 1)), loyalty := none, defense := none } : Semantics.Characteristics), qualities := [] } : Semantics.CharacteristicBundle)) []) true none))])

/-- `keyword_abilities/aftermath.ron` -/
semantic_definition aftermath : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "aftermath") [] [(Semantics.Ability.static (Semantics.StaticSpec.may (Semantics.Subject.actor Semantics.NounPhrase.actor) (Semantics.Macros.deonticActionFor (Semantics.Deed.action "cast") Semantics.NounPhrase.this) ({ bound := none, asThough := none, rider := (Semantics.DeonticRider.play (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.you))) none none true Semantics.PlayPayment.itsOwnCost), except := none } : Semantics.DeonticMods))), (Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.instead (Semantics.WouldClause.would Semantics.Subject.any (Semantics.Instruction.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.stack Semantics.ZoneScope.bare)]) none Semantics.ObservationPoint.departure none [])) (Semantics.Instruction.ownScope (some (Semantics.Deed.action "exile")) (Semantics.Instruction.enact (Semantics.Deed.action "exile") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.exile Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) none none)) none Semantics.ReplUse.repeatedly none) (Semantics.Condition.matches (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) (Semantics.Predicate.castFrom (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare))) Semantics.CondMarking.asLongAs))])

/-- `keyword_abilities/amplify.ron` -/
semantic_definition amplify (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "amplify") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.asEvent (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none [])) (Semantics.Instruction.sequentially [(Semantics.Instruction.expose Semantics.ExposeVerb.reveal (Semantics.Exposed.cards (Semantics.NounPhrase.described (Semantics.DetPhrase.count (Semantics.Quantity.range none none) none) (Semantics.Predicate.and [(Semantics.Predicate.and [Semantics.Predicate.isCard, (Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.hand (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.you)))]), (Semantics.Predicate.shares (Semantics.QualitySort.subtype Semantics.CardType.creature) Semantics.Agreement.share (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this))])))), (Semantics.Instruction.establish (Semantics.StaticSpec.entryRider (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.EntryRider.withCounters (Semantics.Amount.arith Semantics.ArithOp.times (Semantics.Amount.inCaller arg0) (Semantics.Amount.countOf (Semantics.NounPhrase.pro Semantics.Reach.bare Semantics.Plurality.many))) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) Semantics.EntryCounterMark.fresh)) none)])))])

/-- `keyword_abilities/annihilator.ron` -/
semantic_definition annihilator (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "annihilator") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0))] [(Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) (Semantics.Instruction.attack none)) [] none [] none none none (Semantics.Instruction.instruct (Semantics.NounPhrase.combatPlayer Semantics.CombatRole.defending) (Semantics.Instruction.ownScope (some (Semantics.Deed.action "sacrifice")) (Semantics.Instruction.enact (Semantics.Deed.action "sacrifice") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.described (Semantics.DetPhrase.count (Semantics.Quantity.exactlyOf (Semantics.Amount.inCaller arg0)) none) (Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)))) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.departure none []) none none))))])

/-- `keyword_abilities/ascend.ron` -/
semantic_definition ascend : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "ascend") [] [(Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.whileHolds (Semantics.Condition.and [(Semantics.Condition.compareAmt (Semantics.Amount.countOf (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.and [(Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)]))) Semantics.Comparator.atLeast (Semantics.Amount.lit 10)), (Semantics.Condition.not (Semantics.Condition.matches Semantics.NounPhrase.you (Semantics.Predicate.hasDesignation (Semantics.DesignationLabel.named "citysBlessing") none)))]) (Semantics.Instruction.gainDesignation Semantics.NounPhrase.you (Semantics.DesignationLabel.named "citysBlessing") (some (Semantics.Duration.between none none)))) (Semantics.Condition.matches Semantics.NounPhrase.this (Semantics.Predicate.or [(Semantics.Predicate.hasType Semantics.CardType.artifact), (Semantics.Predicate.hasType Semantics.CardType.battle), (Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasType Semantics.CardType.enchantment), (Semantics.Predicate.hasType Semantics.CardType.land), (Semantics.Predicate.hasType Semantics.CardType.planeswalker)])) Semantics.CondMarking.ifSo)), (Semantics.Ability.spell none (Semantics.Instruction.doIf (Semantics.Condition.matches Semantics.NounPhrase.this (Semantics.Predicate.or [(Semantics.Predicate.hasType Semantics.CardType.instant), (Semantics.Predicate.hasType Semantics.CardType.sorcery)])) (Semantics.Instruction.doIf (Semantics.Condition.and [(Semantics.Condition.compareAmt (Semantics.Amount.countOf (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.and [(Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)]))) Semantics.Comparator.atLeast (Semantics.Amount.lit 10)), (Semantics.Condition.not (Semantics.Condition.matches Semantics.NounPhrase.you (Semantics.Predicate.hasDesignation (Semantics.DesignationLabel.named "citysBlessing") none)))]) (Semantics.Instruction.gainDesignation Semantics.NounPhrase.you (Semantics.DesignationLabel.named "citysBlessing") (some (Semantics.Duration.between none none))) none) none))])

/-- `keyword_abilities/assist.ron` -/
semantic_definition assist : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "assist") [] [])

/-- `keyword_abilities/auraSwap.ron` -/
semantic_definition auraSwap (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "auraSwap") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.activated (Semantics.Cost.inCaller arg0) (Semantics.Instruction.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Instruction.exchange (Semantics.Exchanged.cardsAcross (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [Semantics.Predicate.isCard, (Semantics.Predicate.hasSubtype (Semantics.Subtype.of Semantics.CardType.enchantment "Aura")), (Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.hand (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.you)))])))) none none) none none none none)])

/-- `keyword_abilities/awaken.ron` -/
semantic_definition awaken (arg0 : Semantics.Amount) (arg1 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "awaken") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0)), (Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg1))] [(Semantics.Ability.static (Semantics.StaticSpec.may (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Macros.deonticActionFor (Semantics.Deed.action "cast") Semantics.NounPhrase.this) ({ bound := none, asThough := none, rider := (Semantics.DeonticRider.play none none none false (Semantics.PlayPayment.payingInstead (Semantics.Cost.inCaller arg1))), except := none } : Semantics.DeonticMods))), (Semantics.Ability.spell none (Semantics.Instruction.doIf (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "awaken")) none) Semantics.NounPhrase.this) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) (Semantics.Instruction.sequentially [(Semantics.Instruction.putCounters (Semantics.Amount.inCaller arg0) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) (Semantics.NounPhrase.described (Semantics.DetPhrase.target (Semantics.Quantity.range (some 1) (some 1))) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.land), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)])) Semantics.CounterBatch.one true), (Semantics.Instruction.establish (Semantics.StaticSpec.characteristicChange (Semantics.NounPhrase.pro (Semantics.Reach.word (Semantics.NounWord.type Semantics.CardType.land)) Semantics.Plurality.one) [(Semantics.CharacteristicEdit.typeLine Semantics.QualityOp.adds ({ supertypes := none, types := (some [Semantics.CardType.creature]), subtypes := (some [(Semantics.Subtype.of Semantics.CardType.creature "Elemental")]), retained := none } : Semantics.TypeLineChanges)), (Semantics.CharacteristicEdit.stat Semantics.QualityOp.sets Semantics.CharacteristicStat.power (Semantics.Amount.lit 0)), (Semantics.CharacteristicEdit.stat Semantics.QualityOp.sets Semantics.CharacteristicStat.toughness (Semantics.Amount.lit 0)), (Semantics.CharacteristicEdit.addedAbilities [(Semantics.Ability.keyword (Semantics.KeywordLabel.named "haste") [] [])])]) none)]) none))])

/-- `keyword_abilities/backup.ron` -/
semantic_definition backup (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "backup") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0))] [(Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.move (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none [])) [] none [] none none none (Semantics.Instruction.sequentially [(Semantics.Instruction.putCounters (Semantics.Amount.inCaller arg0) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) (Semantics.NounPhrase.described (Semantics.DetPhrase.target (Semantics.Quantity.range (some 1) (some 1))) (Semantics.Predicate.hasType Semantics.CardType.creature)) Semantics.CounterBatch.one true), (Semantics.Instruction.doIf (Semantics.Condition.matches (Semantics.NounPhrase.pro (Semantics.Reach.word (Semantics.NounWord.type Semantics.CardType.creature)) Semantics.Plurality.one) (Semantics.Predicate.otherThan (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none))) (Semantics.Instruction.establish (Semantics.StaticSpec.abilityGrantSelected (Semantics.NounPhrase.pro (Semantics.Reach.word (Semantics.NounWord.type Semantics.CardType.creature)) Semantics.Plurality.one) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none) (Semantics.AbilitySelection.printedAfter (Semantics.KeywordLabel.named "backup"))) (some (Semantics.Duration.between none (some (Semantics.GameEvent.endOf Semantics.TurnPart.turn none Semantics.TurnRef.current))))) none)]))])

/-- `keyword_abilities/banding.ron` -/
semantic_definition banding : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "banding") [] [])

/-- `keyword_abilities/bargain.ron` -/
semantic_definition bargain : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "bargain") [] [(Semantics.Ability.static (Semantics.StaticSpec.addedCost (Semantics.Cost.perform (Semantics.Instruction.ownScope (some (Semantics.Deed.action "sacrifice")) (Semantics.Instruction.enact (Semantics.Deed.action "sacrifice") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.or [(Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.artifact), (Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare))]), (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.enchantment), (Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare))]), Semantics.Predicate.isToken]))) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.departure none []) none none))) true Semantics.PayTimes.once))])

/-- `keyword_abilities/battleCry.ron` -/
semantic_definition battleCry : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "battleCry") [] [(Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) (Semantics.Instruction.attack none)) [] none [] none none none (Semantics.Instruction.establish (Semantics.StaticSpec.modification (Semantics.NounPhrase.described Semantics.DetPhrase.each (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.inCombat Semantics.CombatRelation.attackerOf none), (Semantics.Predicate.otherThan (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none))])) Semantics.Stat.power (Semantics.Delta.up (Semantics.Amount.lit 1))) (some (Semantics.Duration.between none (some (Semantics.GameEvent.endOf Semantics.TurnPart.turn none Semantics.TurnRef.current))))))])

/-- `keyword_abilities/bestow.ron` -/
semantic_definition bestow (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "bestow") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.altCost (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) (some (Semantics.Cost.inCaller arg0))))])

/-- `keyword_abilities/blitz.ron` -/
semantic_definition blitz (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "blitz") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.may (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Macros.deonticActionFor (Semantics.Deed.action "cast") Semantics.NounPhrase.this) ({ bound := none, asThough := none, rider := (Semantics.DeonticRider.play none none none false (Semantics.PlayPayment.payingInstead (Semantics.Cost.inCaller arg0))), except := none } : Semantics.DeonticMods))), (Semantics.Ability.static (Semantics.StaticSpec.asEvent (Semantics.GameEvent.does (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Instruction.pay Semantics.Cost.itsManaCost Semantics.PayTimes.once Semantics.PaymentOutcome.paid (some (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) (some (Semantics.KeywordLabel.named "blitz")))) (Semantics.Instruction.delay (Semantics.GameEvent.beginningOf Semantics.TurnPart.endStep none (Semantics.TurnRef.next none)) [] none (Semantics.Instruction.ownScope (some (Semantics.Deed.action "sacrifice")) (Semantics.Instruction.enact (Semantics.Deed.action "sacrifice") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.becomes (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this))) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.departure none []) none none))))), (Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.abilityGrant (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.Ability.keyword (Semantics.KeywordLabel.named "haste") [] [])) (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "blitz")) none) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) Semantics.CondMarking.asLongAs)), (Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.abilityGrant (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.departure none [])) [] none [] none none none (Semantics.Instruction.draw (Semantics.Amount.lit 1)))) (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "blitz")) none) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) Semantics.CondMarking.asLongAs))])

/-- `keyword_abilities/bloodthirst.ron` -/
semantic_definition bloodthirst (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "bloodthirst") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.entryRider (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.EntryRider.withCounters (Semantics.Amount.inCaller arg0) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) Semantics.EntryCounterMark.fresh)) (Semantics.Condition.did (Semantics.DidClause.mk (Semantics.Subject.actor (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) Semantics.Predicate.opponent)) (Semantics.Instruction.dealDamage Semantics.Amount.anyAmount Semantics.NounPhrase.which Semantics.DamageKind.any) (Semantics.LookbackSpan.duration (Semantics.Duration.between (some (Semantics.GameEvent.beginningOf Semantics.TurnPart.turn none Semantics.TurnRef.current)) (some Semantics.GameEvent.now)) none))) Semantics.CondMarking.asLongAs))])

/-- `keyword_abilities/boast.ron` -/
semantic_definition boast (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "boast") [(Semantics.KeywordParam.ability arg0)] [(Semantics.Ability.qualified arg0 none (some (Semantics.UsageLimit.mk (Semantics.CountBound.moreThan (Semantics.Amount.lit 1)) Semantics.Period.turn)) (some (Semantics.Condition.matches (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none) (Semantics.Predicate.did (Semantics.DidClause.mk (Semantics.Subject.actor Semantics.NounPhrase.which) (Semantics.Instruction.attack none) (Semantics.LookbackSpan.duration (Semantics.Duration.between (some (Semantics.GameEvent.beginningOf Semantics.TurnPart.turn none Semantics.TurnRef.current)) (some Semantics.GameEvent.now)) none))))) none)])

/-- `keyword_abilities/bushido.ron` -/
semantic_definition bushido (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "bushido") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0))] [(Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) (Semantics.Instruction.block Semantics.NounPhrase.any)) [(Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.block (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)))] none [] none none none (Semantics.Instruction.establish (Semantics.StaticSpec.ptModification (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none) (Semantics.Delta.up (Semantics.Amount.inCaller arg0)) (Semantics.Delta.up (Semantics.Amount.inCaller arg0))) (some (Semantics.Duration.between none (some (Semantics.GameEvent.endOf Semantics.TurnPart.turn none Semantics.TurnRef.current))))))])

/-- `keyword_abilities/buyback.ron` -/
semantic_definition buyback (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "buyback") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.addedCost (Semantics.Cost.inCaller arg0) true Semantics.PayTimes.once)), (Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.instead (Semantics.WouldClause.would Semantics.Subject.any (Semantics.Instruction.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.stack Semantics.ZoneScope.bare)]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.departure none [])) (Semantics.Instruction.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.hand Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) none Semantics.ReplUse.repeatedly none) (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "buyback")) none) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) Semantics.CondMarking.asLongAs))])

/-- `keyword_abilities/cascade.ron` -/
semantic_definition cascade : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "cascade") [] [(Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Instruction.procedure Semantics.Procedure.cast (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) Semantics.PlayPayment.itsOwnCost none none none)) [] none [] none none none (Semantics.Instruction.sequentially [(Semantics.Instruction.ownScope (some (Semantics.Deed.action "exile")) (Semantics.Instruction.enact (Semantics.Deed.action "exile") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.run Semantics.LibraryEnd.top (Semantics.Predicate.and [(Semantics.Predicate.not (Semantics.Predicate.hasType Semantics.CardType.land)), (Semantics.Predicate.compare [(Semantics.ProjAxis.stat Semantics.Stat.manaValue)] Semantics.Comparator.less (Semantics.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.manaValue) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)))]) (Semantics.Pile.library (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.you)))) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.exile Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) none none)), (Semantics.Instruction.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Instruction.enact (Semantics.Deed.action "cast") (Semantics.Instruction.procedure Semantics.Procedure.cast (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.card) Semantics.Plurality.one) Semantics.PlayPayment.withoutPaying (some (Semantics.Predicate.compare [(Semantics.ProjAxis.stat Semantics.Stat.manaValue)] Semantics.Comparator.less (Semantics.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.manaValue) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)))) none none) none none) none none), (Semantics.Instruction.move (Semantics.NounPhrase.someOf Semantics.SliceCount.whole (some (Semantics.Predicate.not (Semantics.Predicate.did (Semantics.DidClause.mk Semantics.Subject.any (Semantics.Instruction.procedure Semantics.Procedure.cast Semantics.NounPhrase.which Semantics.PlayPayment.itsOwnCost none none none) Semantics.LookbackSpan.thisWay)))) (Semantics.NounPhrase.pro (Semantics.Reach.verbed (Semantics.Deed.action "exile") none Semantics.VerbedMarking.attributive) Semantics.Plurality.many)) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.place Semantics.LibraryEnd.bottom (Semantics.Pile.library Semantics.ZoneScope.bare))) Semantics.ObservationPoint.arrival none [(Semantics.MoveRider.placement (Semantics.PilePlacement.inOrder Semantics.Arrangement.randomOrder))])]))])

/-- `keyword_abilities/casualty.ron` -/
semantic_definition casualty (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "casualty") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.addedCost (Semantics.Cost.perform (Semantics.Instruction.ownScope (some (Semantics.Deed.action "sacrifice")) (Semantics.Instruction.enact (Semantics.Deed.action "sacrifice") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.compare [(Semantics.ProjAxis.stat Semantics.Stat.power)] Semantics.Comparator.atLeast (Semantics.Amount.inCaller arg0))]))) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.departure none []) none none))) true Semantics.PayTimes.once)), (Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Instruction.procedure Semantics.Procedure.cast (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) Semantics.PlayPayment.itsOwnCost none none none)) [] none [] none none (some (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "casualty")) none) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) Semantics.Comparator.atLeast (Semantics.Amount.lit 1))) (Semantics.Instruction.sequentially [(Semantics.Instruction.copy Semantics.CopySort.fromStack (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) (Semantics.Amount.lit 1) []), (Semantics.Instruction.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Instruction.chooseNewTargets (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.copy) Semantics.Plurality.one)) none none)]))])

/-- `keyword_abilities/champion.ron` -/
semantic_definition champion (arg0 : Semantics.Predicate) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "champion") [(Semantics.KeywordParam.subject (Semantics.Predicate.inCaller arg0))] [(Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none [])) [] none [] none none none (Semantics.Instruction.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Instruction.pay (Semantics.Cost.perform (Semantics.Instruction.ownScope (some (Semantics.Deed.action "exile")) (Semantics.Instruction.enact (Semantics.Deed.action "exile") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.inCaller arg0), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you), (Semantics.Predicate.otherThan (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this))]))) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.exile Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) none none))) Semantics.PayTimes.once Semantics.PaymentOutcome.paid none none) none (some (Semantics.Instruction.ownScope (some (Semantics.Deed.action "sacrifice")) (Semantics.Instruction.enact (Semantics.Deed.action "sacrifice") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.departure none []) none none))))), (Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)]) none Semantics.ObservationPoint.departure none [])) [] none [] none none none (Semantics.Instruction.enact (Semantics.Deed.core Semantics.CoreDeed.return_) (Semantics.Instruction.move (Semantics.NounPhrase.paid (Semantics.LinkedFacet.objects (Semantics.LinkSource.byDeed (Semantics.Deed.action "exile")) (Semantics.Deed.action "exile") Semantics.Plurality.one) Semantics.NounPhrase.this) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none [(Semantics.MoveRider.entry (Semantics.EntryRider.under (Semantics.NounPhrase.possessorOf Semantics.PossessorAxis.owner (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.card) Semantics.Plurality.one))))]) none none))])

/-- `keyword_abilities/changeling.ron` -/
semantic_definition changeling : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "changeling") [] [(Semantics.Ability.static (Semantics.StaticSpec.characteristicChange (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none) [(Semantics.CharacteristicEdit.everyTypeOf Semantics.QualityOp.adds Semantics.SubtypeSpace.creature)]))])

/-- `keyword_abilities/chooseABackground.ron` -/
semantic_definition chooseABackground : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "chooseABackground") [] [])

/-- `keyword_abilities/cipher.ron` -/
semantic_definition cipher : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "cipher") [] [(Semantics.Ability.spell none (Semantics.Instruction.doIf (Semantics.Condition.matches Semantics.NounPhrase.this Semantics.Predicate.isCard) (Semantics.Instruction.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Instruction.sequentially [(Semantics.Instruction.ownScope (some (Semantics.Deed.action "exile")) (Semantics.Instruction.enact (Semantics.Deed.action "exile") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.exile Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) none none)), (Semantics.Instruction.relate Semantics.NounPhrase.this (Semantics.RelationLabel.named "encodedOn") (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)])))]) none none) none)), (Semantics.Ability.static (Semantics.StaticSpec.abilityGrant (Semantics.NounPhrase.counterpart (Semantics.RelationLabel.named "encodedOn") Semantics.NounPhrase.this) (Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) (Semantics.Instruction.dealDamage Semantics.Amount.anyAmount (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) Semantics.Predicate.anyPlayer) Semantics.DamageKind.combatOnly)) [] none [] none none none (Semantics.Instruction.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Instruction.sequentially [(Semantics.Instruction.copy Semantics.CopySort.fromCardZone (Semantics.NounPhrase.described Semantics.DetPhrase.the (Semantics.Predicate.and [Semantics.Predicate.isCard, (Semantics.Predicate.related (Semantics.RelationLabel.named "encodedOn") (some (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)))])) (Semantics.Amount.lit 1) []), (Semantics.Instruction.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Instruction.enact (Semantics.Deed.action "cast") (Semantics.Instruction.procedure Semantics.Procedure.cast (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.copy) Semantics.Plurality.one) Semantics.PlayPayment.withoutPaying none none none) none none) none none)]) none none))))])

/-- `keyword_abilities/cleave.ron` -/
semantic_definition cleave (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "cleave") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.may (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Macros.deonticActionFor (Semantics.Deed.action "cast") Semantics.NounPhrase.this) ({ bound := none, asThough := none, rider := (Semantics.DeonticRider.play none none none false (Semantics.PlayPayment.payingInstead (Semantics.Cost.inCaller arg0))), except := none } : Semantics.DeonticMods))), (Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.textChange (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) Semantics.TextChange.removeBracketed) (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "cleave")) none) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) Semantics.CondMarking.asLongAs))])

/-- `keyword_abilities/companion.ron` -/
semantic_definition companion (arg0 : Semantics.Condition) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "companion") [] [])

/-- `keyword_abilities/compleated.ron` -/
semantic_definition compleated : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "compleated") [] [])

/-- `keyword_abilities/conspire.ron` -/
semantic_definition conspire : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "conspire") [] [(Semantics.Ability.static (Semantics.StaticSpec.addedCost (Semantics.Cost.perform (Semantics.Instruction.ownScope (some (Semantics.Deed.action "tap")) (Semantics.Instruction.enact (Semantics.Deed.action "tap") (Semantics.Instruction.setStatus Semantics.Status.tapped (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.described (Semantics.DetPhrase.count (Semantics.Quantity.range (some 2) (some 2)) none) (Semantics.Predicate.and [(Semantics.Predicate.hasStatus Semantics.Status.untapped), (Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you), (Semantics.Predicate.shares Semantics.QualitySort.color Semantics.Agreement.share (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this))])))) none none))) true Semantics.PayTimes.once)), (Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Instruction.procedure Semantics.Procedure.cast (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) Semantics.PlayPayment.itsOwnCost none none none)) [] none [] none none (some (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "conspire")) none) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) Semantics.Comparator.atLeast (Semantics.Amount.lit 1))) (Semantics.Instruction.sequentially [(Semantics.Instruction.copy Semantics.CopySort.fromStack (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) (Semantics.Amount.lit 1) []), (Semantics.Instruction.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Instruction.chooseNewTargets (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.copy) Semantics.Plurality.one)) none none)]))])

/-- `keyword_abilities/convoke.ron` -/
semantic_definition convoke : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "convoke") [] [(Semantics.Ability.static (Semantics.StaticSpec.paySubstitute (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) ({ action := (Semantics.Instruction.ownScope (some (Semantics.Deed.action "tap")) (Semantics.Instruction.enact (Semantics.Deed.action "tap") (Semantics.Instruction.setStatus Semantics.Status.tapped (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.hasStatus Semantics.Status.untapped), (Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)])))) none none)), pays := [Semantics.PipPaid.generic, (Semantics.PipPaid.ofColor (Semantics.ColorTerm.colorOf (Semantics.NounPhrase.pro (Semantics.Reach.word (Semantics.NounWord.type Semantics.CardType.creature)) Semantics.Plurality.one)))] } : Semantics.ManaSubstitute)))])

/-- `keyword_abilities/craft.ron` -/
semantic_definition craft (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "craft") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [])

/-- `keyword_abilities/crew.ron` -/
semantic_definition crew (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "crew") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0))] [(Semantics.Ability.activated (Semantics.Cost.perform (Semantics.Instruction.ownScope (some (Semantics.Deed.action "tap")) (Semantics.Instruction.enact (Semantics.Deed.action "tap") (Semantics.Instruction.setStatus Semantics.Status.tapped (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.someOf (Semantics.SliceCount.totalling (Semantics.ProjAxis.stat Semantics.Stat.power) Semantics.Comparator.atLeast (Semantics.Amount.inCaller arg0)) none (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.and [(Semantics.Predicate.hasStatus Semantics.Status.untapped), (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you), (Semantics.Predicate.otherThan (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this))])]))))) none none))) (Semantics.Instruction.establish (Semantics.StaticSpec.characteristicChange (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) [(Semantics.CharacteristicEdit.typeLine Semantics.QualityOp.adds ({ supertypes := none, types := (some [Semantics.CardType.artifact, Semantics.CardType.creature]), subtypes := none, retained := none } : Semantics.TypeLineChanges))]) (some (Semantics.Duration.between none (some (Semantics.GameEvent.endOf Semantics.TurnPart.turn none Semantics.TurnRef.current))))) none none none none)])

/-- `keyword_abilities/cumulativeUpkeep.ron` -/
semantic_definition cumulativeUpkeep (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "cumulativeUpkeep") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.triggered (Semantics.GameEvent.beginningOf Semantics.TurnPart.upkeep (some Semantics.NounPhrase.you) (Semantics.TurnRef.each none)) [] none [] none none (some (Semantics.Condition.matches (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)))) (Semantics.Instruction.sequentially [(Semantics.Instruction.putCounters (Semantics.Amount.lit 1) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "ageCounter")) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.CounterBatch.one true), (Semantics.Instruction.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Instruction.pay (Semantics.Cost.scaled (Semantics.Cost.inCaller arg0) (Semantics.Amount.arith Semantics.ArithOp.times (Semantics.Amount.lit 1) (Semantics.Amount.statOf (Semantics.ProjAxis.counter (Semantics.CounterKind.named "ageCounter")) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)))) Semantics.PayTimes.once Semantics.PaymentOutcome.paid none none) none (some (Semantics.Instruction.ownScope (some (Semantics.Deed.action "sacrifice")) (Semantics.Instruction.enact (Semantics.Deed.action "sacrifice") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.departure none []) none none))))]))])

/-- `keyword_abilities/cycling.ron` -/
semantic_definition cycling (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "cycling") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.activated (Semantics.Cost.compound [(Semantics.Cost.inCaller arg0), (Semantics.Cost.perform (Semantics.Instruction.ownScope (some (Semantics.Deed.action "discard")) (Semantics.Instruction.enact (Semantics.Deed.action "discard") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller Semantics.NounPhrase.this) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.hand (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.actor))]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) none none)))]) (Semantics.Instruction.draw (Semantics.Amount.lit 1)) none none none none)])

/-- `keyword_abilities/dash.ron` -/
semantic_definition dash (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "dash") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.may (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Macros.deonticActionFor (Semantics.Deed.action "cast") Semantics.NounPhrase.this) ({ bound := none, asThough := none, rider := (Semantics.DeonticRider.play none none none false (Semantics.PlayPayment.payingInstead (Semantics.Cost.inCaller arg0))), except := none } : Semantics.DeonticMods))), (Semantics.Ability.static (Semantics.StaticSpec.asEvent (Semantics.GameEvent.does (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Instruction.pay Semantics.Cost.itsManaCost Semantics.PayTimes.once Semantics.PaymentOutcome.paid (some (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) (some (Semantics.KeywordLabel.named "dash")))) (Semantics.Instruction.delay (Semantics.GameEvent.beginningOf Semantics.TurnPart.endStep none (Semantics.TurnRef.next none)) [] none (Semantics.Instruction.enact (Semantics.Deed.core Semantics.CoreDeed.return_) (Semantics.Instruction.move (Semantics.NounPhrase.becomes (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.hand Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) none none)))), (Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.abilityGrant (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.Ability.keyword (Semantics.KeywordLabel.named "haste") [] [])) (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "dash")) none) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) Semantics.CondMarking.asLongAs))])

/-- `keyword_abilities/daybound.ron` -/
semantic_definition daybound : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "daybound") [] [(Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.entryRider (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.EntryRider.entersTransformed) (Semantics.Condition.gameIs (Semantics.DesignationLabel.named "night")) Semantics.CondMarking.ifSo)), (Semantics.Ability.static (Semantics.StaticSpec.asEvent (Semantics.GameEvent.gameBecomes (Semantics.DesignationLabel.named "night")) (Semantics.Instruction.doIf (Semantics.Condition.matches (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.Predicate.currentFace Semantics.CardFaceSide.front)) (Semantics.Instruction.ownScope (some (Semantics.Deed.action "transform")) (Semantics.Instruction.enact (Semantics.Deed.action "transform") (Semantics.Instruction.turnOver (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this))) none none)) none))), (Semantics.Ability.static (Semantics.StaticSpec.cant Semantics.Subject.any (Semantics.Macros.deonticActionFor (Semantics.Deed.action "transform") (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) ({ bound := none, asThough := none, rider := Semantics.DeonticRider.noRider, except := (some (Semantics.Cause.byKeyword (Semantics.KeywordLabel.named "daybound"))) } : Semantics.DeonticMods)))])

/-- `keyword_abilities/deathtouch.ron` -/
semantic_definition deathtouch : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "deathtouch") [] [])

/-- `keyword_abilities/decayed.ron` -/
semantic_definition decayed : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "decayed") [] [(Semantics.Ability.static (Semantics.StaticSpec.cant (Semantics.Subject.actor (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) (Semantics.Macros.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.block) Semantics.NounPhrase.any) ({ bound := none, asThough := none, rider := Semantics.DeonticRider.noRider, except := none } : Semantics.DeonticMods))), (Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) (Semantics.Instruction.attack none)) [] none [] none none none (Semantics.Instruction.delay (Semantics.GameEvent.beginningOf Semantics.TurnPart.endOfCombat none (Semantics.TurnRef.next none)) [] none (Semantics.Instruction.ownScope (some (Semantics.Deed.action "sacrifice")) (Semantics.Instruction.enact (Semantics.Deed.action "sacrifice") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.departure none []) none none))))])

/-- `keyword_abilities/defender.ron` -/
semantic_definition defender : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "defender") [] [(Semantics.Ability.static (Semantics.StaticSpec.cant (Semantics.Subject.actor (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) (Semantics.Macros.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.attack) Semantics.NounPhrase.any) ({ bound := none, asThough := none, rider := Semantics.DeonticRider.noRider, except := none } : Semantics.DeonticMods)))])

/-- `keyword_abilities/delve.ron` -/
semantic_definition delve : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "delve") [] [(Semantics.Ability.static (Semantics.StaticSpec.paySubstitute (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) ({ action := (Semantics.Instruction.ownScope (some (Semantics.Deed.action "exile")) (Semantics.Instruction.enact (Semantics.Deed.action "exile") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [Semantics.Predicate.isCard, (Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.graveyard (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.you)))]))) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.exile Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) none none)), pays := [Semantics.PipPaid.generic] } : Semantics.ManaSubstitute)))])

/-- `keyword_abilities/demonstrate.ron` -/
semantic_definition demonstrate : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "demonstrate") [] [(Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Instruction.procedure Semantics.Procedure.cast (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) Semantics.PlayPayment.itsOwnCost none none none)) [] none [] none none none (Semantics.Instruction.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Instruction.sequentially [(Semantics.Instruction.copy Semantics.CopySort.fromStack (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) (Semantics.Amount.lit 1) []), (Semantics.Instruction.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Instruction.chooseNewTargets (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.copy) Semantics.Plurality.one)) none none)]) (some (Semantics.Instruction.sequentially [(Semantics.Instruction.choose none (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) Semantics.Predicate.opponent) Semantics.Disclosure.openly none), (Semantics.Instruction.instruct (Semantics.NounPhrase.described Semantics.DetPhrase.the (Semantics.Predicate.chosenPlayer Semantics.ChoiceRef.theChoice)) (Semantics.Instruction.copy Semantics.CopySort.fromStack (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) (Semantics.Amount.lit 1) [])), (Semantics.Instruction.instruct (Semantics.NounPhrase.described Semantics.DetPhrase.the (Semantics.Predicate.chosenPlayer Semantics.ChoiceRef.theChoice)) (Semantics.Instruction.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Instruction.chooseNewTargets (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.copy) Semantics.Plurality.one)) none none))])) none))])

/-- `keyword_abilities/dethrone.ron` -/
semantic_definition dethrone : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "dethrone") [] [(Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) (Semantics.Instruction.attack (some (Semantics.NounPhrase.described Semantics.DetPhrase.the (Semantics.Predicate.and [Semantics.Predicate.anyPlayer, (Semantics.Predicate.superlative Semantics.AggregateOp.max (Semantics.ProjAxis.playerStat Semantics.PlayerStat.lifeTotal) Semantics.Predicate.anyPlayer)]))))) [] none [] none none none (Semantics.Instruction.putCounters (Semantics.Amount.lit 1) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none) Semantics.CounterBatch.one true))])

/-- `keyword_abilities/devoid.ron` -/
semantic_definition devoid : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "devoid") [] [(Semantics.Ability.static (Semantics.StaticSpec.characteristicChange Semantics.NounPhrase.this [(Semantics.CharacteristicEdit.colors Semantics.QualityOp.sets (Semantics.ColorSpec.some []))]))])

/-- `keyword_abilities/devourQuality.ron` -/
semantic_definition devourQuality (arg0 : Predicate) (arg1 : Amount) : Ability :=
  .keyword (.named "devour") [.quality (.inCaller arg0), .number (.inCaller arg1)]
    [.static (.asEvent
      (.does .any (.move thisPermanent .anywhere (some battlefield) .arrival none []))
      (.withContinuation .optional
        (sacrifice (.described (.count anyNumber none)
          (.and [.inCaller arg0, .inZone battlefield])))
        (some (.establish
          (.entryRider thisPermanent (.withCounters
            (.arith .times (.inCaller arg1)
              (.countOf (theVerbed (.action "sacrifice") .many)))
            (.printed p1p1Counter) .fresh)) none))
        none))]

/-- `keyword_abilities/devour.ron` -/
semantic_definition devour (arg0 : Amount) : Ability :=
  .keyword (.named "devour") [.number (.inCaller arg0)]
    [devourQuality creature arg0]

/-- `keyword_abilities/disguise.ron` -/
semantic_definition disguise (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "disguise") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.may (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Macros.deonticActionFor (Semantics.Deed.action "cast") Semantics.NounPhrase.this) ({ bound := none, asThough := none, rider := (Semantics.DeonticRider.play none none none false (Semantics.PlayPayment.faceDown ({ characteristics := (some ({ name := none, cost := none, colors := [], supertypes := [], types := [Semantics.CardType.creature], subtypes := [], text := [(Semantics.Ability.keyword (Semantics.KeywordLabel.named "ward") [(Semantics.KeywordParam.cost (Semantics.Cost.mana [(Semantics.ManaSymbol.simple (Semantics.SimpleManaSymbol.generic 2))]))] [])], power := (some (Semantics.Amount.lit 2)), toughness := (some (Semantics.Amount.lit 2)), loyalty := none, defense := none } : Semantics.Characteristics)) } : Semantics.FaceDown) (Semantics.Cost.mana [(Semantics.ManaSymbol.simple (Semantics.SimpleManaSymbol.generic 3))]))), except := none } : Semantics.DeonticMods))), (Semantics.Ability.static (Semantics.StaticSpec.specialAction none (Semantics.Cost.inCaller arg0) (Semantics.Instruction.setStatus Semantics.Status.faceUp Semantics.NounPhrase.this)))])

/-- `keyword_abilities/disturb.ron` -/
semantic_definition disturb (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "disturb") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [])

/-- `keyword_abilities/doctorsCompanion.ron` -/
semantic_definition doctorsCompanion : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "doctorsCompanion") [] [])

/-- `keyword_abilities/doubleStrike.ron` -/
semantic_definition doubleStrike : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "doubleStrike") [] [])

/-- `keyword_abilities/dredge.ron` -/
semantic_definition dredge (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "dredge") [(Semantics.KeywordParam.number arg0)] [(Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.instead (Semantics.WouldClause.would (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Instruction.draw Semantics.Amount.anyAmount)) (Semantics.Instruction.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Instruction.sequentially [(Semantics.Instruction.ownScope (some (Semantics.Deed.action "mill")) (Semantics.Instruction.enact (Semantics.Deed.action "mill") (Semantics.Instruction.move (Semantics.NounPhrase.slice Semantics.LibraryEnd.top (Semantics.Amount.inCaller arg0) (Semantics.Pile.library (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.actor))) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.library Semantics.ZoneScope.bare)]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) none none)), (Semantics.Instruction.enact (Semantics.Deed.core Semantics.CoreDeed.return_) (Semantics.Instruction.move Semantics.NounPhrase.this (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.graveyard (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.you))]) (some (Semantics.ZoneExpr.zone Semantics.Zone.hand Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) none none)]) none none) none Semantics.ReplUse.repeatedly none) (Semantics.Condition.compareAmt (Semantics.Amount.countOf (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.and [Semantics.Predicate.isCard, (Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.library (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.you)))]))) Semantics.Comparator.atLeast arg0) Semantics.CondMarking.asLongAs))])

/-- `keyword_abilities/echo.ron` -/
semantic_definition echo (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "echo") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [])

/-- `keyword_abilities/embalm.ron` -/
semantic_definition embalm (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "embalm") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.activated (Semantics.Cost.compound [(Semantics.Cost.inCaller arg0), (Semantics.Cost.perform (Semantics.Instruction.enact (Semantics.Deed.action "exile") (Semantics.Instruction.move Semantics.NounPhrase.this (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.graveyard (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.you))]) (some (Semantics.ZoneExpr.zone Semantics.Zone.exile Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) none none))]) (Semantics.Instruction.createObject (Semantics.Amount.lit 1) (Semantics.CreationSpec.token (Semantics.TokenSpec.copyOf Semantics.NounPhrase.this [(Semantics.CopyExcept.edits [(Semantics.CharacteristicEdit.colors Semantics.QualityOp.sets (Semantics.ColorSpec.some [Semantics.Color.white])), (Semantics.CharacteristicEdit.manaCost none), (Semantics.CharacteristicEdit.typeLine Semantics.QualityOp.adds ({ supertypes := none, types := none, subtypes := (some [(Semantics.Subtype.of Semantics.CardType.creature "Zombie")]), retained := none } : Semantics.TypeLineChanges))])]) []) true none) (some (Semantics.Timing.and [(Semantics.Timing.inPart Semantics.TurnPart.mainPhase (some Semantics.NounPhrase.you) Semantics.TurnRef.current), (Semantics.Timing.onlyIf (Semantics.Condition.not (Semantics.Condition.exists_ (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.stack Semantics.ZoneScope.bare))))))])) none none none)])

/-- `keyword_abilities/emerge.ron` -/
semantic_definition emerge (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "emerge") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.may (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Macros.deonticActionFor (Semantics.Deed.action "cast") Semantics.NounPhrase.this) ({ bound := none, asThough := none, rider := (Semantics.DeonticRider.play none none none false (Semantics.PlayPayment.payingInstead (Semantics.Cost.compound [(Semantics.Cost.inCaller arg0), (Semantics.Cost.perform (Semantics.Instruction.ownScope (some (Semantics.Deed.action "sacrifice")) (Semantics.Instruction.enact (Semantics.Deed.action "sacrifice") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.hasType Semantics.CardType.creature))) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.departure none []) none none)))]))), except := none } : Semantics.DeonticMods))), (Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.costShift (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) (Semantics.CostShift.less (Semantics.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.manaValue) (Semantics.NounPhrase.paid (Semantics.LinkedFacet.objects Semantics.LinkSource.theAlternative (Semantics.Deed.action "sacrifice") Semantics.Plurality.one) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this))) none)) (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "emerge")) none) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) Semantics.CondMarking.asLongAs))])

/-- `keyword_abilities/emergeFrom.ron` -/
semantic_definition emergeFrom (arg0 : Semantics.Predicate) (arg1 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "emergeFrom") [(Semantics.KeywordParam.quality (Semantics.Predicate.inCaller arg0)), (Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg1))] [(Semantics.Ability.static (Semantics.StaticSpec.may (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Macros.deonticActionFor (Semantics.Deed.action "cast") Semantics.NounPhrase.this) ({ bound := none, asThough := none, rider := (Semantics.DeonticRider.play none none none false (Semantics.PlayPayment.payingInstead (Semantics.Cost.compound [(Semantics.Cost.inCaller arg1), (Semantics.Cost.perform (Semantics.Instruction.ownScope (some (Semantics.Deed.action "sacrifice")) (Semantics.Instruction.enact (Semantics.Deed.action "sacrifice") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.inCaller arg0), (Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare))]))) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.departure none []) none none)))]))), except := none } : Semantics.DeonticMods))), (Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.costShift (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) (Semantics.CostShift.less (Semantics.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.manaValue) (Semantics.NounPhrase.paid (Semantics.LinkedFacet.objects Semantics.LinkSource.theAlternative (Semantics.Deed.action "sacrifice") Semantics.Plurality.one) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this))) none)) (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "emergeFrom")) none) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) Semantics.CondMarking.asLongAs))])

/-- `keyword_abilities/enchant.ron` -/
semantic_definition enchant (arg0 : Semantics.Predicate) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "enchant") [(Semantics.KeywordParam.subject (Semantics.Predicate.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.addsTarget (Semantics.NounPhrase.asType Semantics.CardType.enchantment Semantics.NounPhrase.this (some (Semantics.Subtype.of Semantics.CardType.enchantment "Aura"))) (Semantics.Predicate.inCaller arg0))), (Semantics.Ability.static (Semantics.StaticSpec.may Semantics.Subject.any (Semantics.Instruction.attachment Semantics.AttachMove.attached (Semantics.NounPhrase.asType Semantics.CardType.enchantment Semantics.NounPhrase.this (some (Semantics.Subtype.of Semantics.CardType.enchantment "Aura"))) (some (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.inCaller arg0)))) ({ bound := none, asThough := none, rider := Semantics.DeonticRider.noRider, except := none } : Semantics.DeonticMods))), (Semantics.Ability.static (Semantics.StaticSpec.asEvent (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.move (Semantics.NounPhrase.asType Semantics.CardType.enchantment Semantics.NounPhrase.this (some (Semantics.Subtype.of Semantics.CardType.enchantment "Aura"))) (Semantics.EventSource.anywhereBut [(Semantics.ZoneExpr.zone Semantics.Zone.stack Semantics.ZoneScope.bare)]) (some (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none [])) (Semantics.Instruction.ownScope (some (Semantics.Deed.action "attach")) (Semantics.Instruction.enact (Semantics.Deed.action "attach") (Semantics.Instruction.attachment Semantics.AttachMove.attached (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asType Semantics.CardType.enchantment Semantics.NounPhrase.this (some (Semantics.Subtype.of Semantics.CardType.enchantment "Aura")))) (some (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.inCaller arg0))))) none none))))])

/-- `keyword_abilities/encore.ron` -/
semantic_definition encore (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "encore") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [])

/-- `keyword_abilities/enlist.ron` -/
semantic_definition enlist : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "enlist") [] [])

/-- `keyword_abilities/entwine.ron` -/
semantic_definition entwine (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "entwine") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.modeChoice (Semantics.ModeChange.allInstead (some (Semantics.Cost.inCaller arg0)) none)))])

/-- `keyword_abilities/epic.ron` -/
semantic_definition epic : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "epic") [] [(Semantics.Ability.spell none (Semantics.Instruction.establish (Semantics.StaticSpec.cant (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Macros.deonticActionFor (Semantics.Deed.action "cast") Semantics.NounPhrase.any) ({ bound := none, asThough := none, rider := Semantics.DeonticRider.noRider, except := none } : Semantics.DeonticMods)) (some (Semantics.Duration.between none none)))), (Semantics.Ability.spell none (Semantics.Instruction.delay (Semantics.GameEvent.beginningOf Semantics.TurnPart.upkeep (some Semantics.NounPhrase.you) (Semantics.TurnRef.each none)) [] (some (Semantics.Duration.between none none)) (Semantics.Instruction.sequentially [(Semantics.Instruction.copy Semantics.CopySort.fromStack (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) (Semantics.Amount.lit 1) [(Semantics.CopyExcept.edits [(Semantics.CharacteristicEdit.removedAbilities (Semantics.AbilitySelection.specified [(Semantics.AbilityLost.term (Semantics.KeywordTerm.the (Semantics.KeywordLabel.named "epic")))]))])]), (Semantics.Instruction.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Instruction.chooseNewTargets (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.copy) Semantics.Plurality.one)) none none)])))])

/-- `keyword_abilities/equip.ron` -/
semantic_definition equip (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "equip") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.activated (Semantics.Cost.inCaller arg0) (Semantics.Instruction.ownScope (some (Semantics.Deed.action "attach")) (Semantics.Instruction.enact (Semantics.Deed.action "attach") (Semantics.Instruction.attachment Semantics.AttachMove.attached (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) (some (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.described (Semantics.DetPhrase.target (Semantics.Quantity.range (some 1) (some 1))) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)]))))) none none)) (some (Semantics.Timing.and [(Semantics.Timing.inPart Semantics.TurnPart.mainPhase (some Semantics.NounPhrase.you) Semantics.TurnRef.current), (Semantics.Timing.onlyIf (Semantics.Condition.not (Semantics.Condition.exists_ (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.stack Semantics.ZoneScope.bare))))))])) none none none)])

/-- `keyword_abilities/equipQuality.ron` -/
semantic_definition equipQuality (arg0 : Semantics.Predicate) (arg1 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "equipQuality") [(Semantics.KeywordParam.quality (Semantics.Predicate.inCaller arg0)), (Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg1))] [])

/-- `keyword_abilities/escalate.ron` -/
semantic_definition escalate (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "escalate") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.modeChoice (Semantics.ModeChange.extraModesCost (Semantics.Cost.inCaller arg0))))])

/-- `keyword_abilities/escape.ron` -/
semantic_definition escape (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "escape") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.may (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Macros.deonticActionFor (Semantics.Deed.action "cast") Semantics.NounPhrase.this) ({ bound := none, asThough := none, rider := (Semantics.DeonticRider.play (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.you))) none none false (Semantics.PlayPayment.payingInstead (Semantics.Cost.inCaller arg0))), except := none } : Semantics.DeonticMods)))])

/-- `keyword_abilities/eternalize.ron` -/
semantic_definition eternalize (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "eternalize") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.activated (Semantics.Cost.compound [(Semantics.Cost.inCaller arg0), (Semantics.Cost.perform (Semantics.Instruction.enact (Semantics.Deed.action "exile") (Semantics.Instruction.move Semantics.NounPhrase.this (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.graveyard (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.you))]) (some (Semantics.ZoneExpr.zone Semantics.Zone.exile Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) none none))]) (Semantics.Instruction.createObject (Semantics.Amount.lit 1) (Semantics.CreationSpec.token (Semantics.TokenSpec.copyOf Semantics.NounPhrase.this [(Semantics.CopyExcept.edits [(Semantics.CharacteristicEdit.colors Semantics.QualityOp.sets (Semantics.ColorSpec.some [Semantics.Color.black])), (Semantics.CharacteristicEdit.stat Semantics.QualityOp.sets Semantics.CharacteristicStat.power (Semantics.Amount.lit 4)), (Semantics.CharacteristicEdit.stat Semantics.QualityOp.sets Semantics.CharacteristicStat.toughness (Semantics.Amount.lit 4)), (Semantics.CharacteristicEdit.manaCost none), (Semantics.CharacteristicEdit.typeLine Semantics.QualityOp.adds ({ supertypes := none, types := none, subtypes := (some [(Semantics.Subtype.of Semantics.CardType.creature "Zombie")]), retained := none } : Semantics.TypeLineChanges))])]) []) true none) (some (Semantics.Timing.and [(Semantics.Timing.inPart Semantics.TurnPart.mainPhase (some Semantics.NounPhrase.you) Semantics.TurnRef.current), (Semantics.Timing.onlyIf (Semantics.Condition.not (Semantics.Condition.exists_ (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.stack Semantics.ZoneScope.bare))))))])) none none none)])

/-- `keyword_abilities/evoke.ron` -/
semantic_definition evoke (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "evoke") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.may (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Macros.deonticActionFor (Semantics.Deed.action "cast") Semantics.NounPhrase.this) ({ bound := none, asThough := none, rider := (Semantics.DeonticRider.play none none none false (Semantics.PlayPayment.payingInstead (Semantics.Cost.inCaller arg0))), except := none } : Semantics.DeonticMods))), (Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none [])) [] none [] none none (some (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "evoke")) none) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) Semantics.Comparator.atLeast (Semantics.Amount.lit 1))) (Semantics.Instruction.instruct (Semantics.NounPhrase.possessorOf Semantics.PossessorAxis.controller (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) (Semantics.Instruction.ownScope (some (Semantics.Deed.action "sacrifice")) (Semantics.Instruction.enact (Semantics.Deed.action "sacrifice") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.departure none []) none none))))])

/-- `keyword_abilities/evolve.ron` -/
semantic_definition evolve : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "evolve") [] [(Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.move (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)])) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none [])) [] none [] none none (some (Semantics.Condition.or [(Semantics.Condition.compareAmt (Semantics.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.power) (Semantics.NounPhrase.pro (Semantics.Reach.word (Semantics.NounWord.type Semantics.CardType.creature)) Semantics.Plurality.one)) Semantics.Comparator.greater (Semantics.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.power) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none))), (Semantics.Condition.compareAmt (Semantics.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.toughness) (Semantics.NounPhrase.pro (Semantics.Reach.word (Semantics.NounWord.type Semantics.CardType.creature)) Semantics.Plurality.one)) Semantics.Comparator.greater (Semantics.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.toughness) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)))])) (Semantics.Instruction.putCounters (Semantics.Amount.lit 1) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none) Semantics.CounterBatch.one true))])

/-- `keyword_abilities/exalted.ron` -/
semantic_definition exalted : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "exalted") [] [(Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)]))) (Semantics.Instruction.attack none)) [] none [] none none (some (Semantics.Condition.compareAmt (Semantics.Amount.countOf (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.inCombat Semantics.CombatRelation.attackerOf none)]))) Semantics.Comparator.eq (Semantics.Amount.lit 1))) (Semantics.Instruction.establish (Semantics.StaticSpec.ptModification (Semantics.NounPhrase.pro (Semantics.Reach.word (Semantics.NounWord.type Semantics.CardType.creature)) Semantics.Plurality.one) (Semantics.Delta.up (Semantics.Amount.lit 1)) (Semantics.Delta.up (Semantics.Amount.lit 1))) (some (Semantics.Duration.between none (some (Semantics.GameEvent.endOf Semantics.TurnPart.turn none Semantics.TurnRef.current))))))])

/-- `keyword_abilities/exhaust.ron` -/
semantic_definition exhaust (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "exhaust") [(Semantics.KeywordParam.ability arg0)] [(Semantics.Ability.qualified arg0 none (some (Semantics.UsageLimit.mk (Semantics.CountBound.moreThan (Semantics.Amount.lit 1)) Semantics.Period.thisObject)) none none)])

/-- `keyword_abilities/exploit.ron` -/
semantic_definition exploit : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "exploit") [] [(Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.move (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none [])) [] none [] none none none (Semantics.Instruction.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Instruction.ownScope (some (Semantics.Deed.action "sacrifice")) (Semantics.Instruction.enact (Semantics.Deed.action "sacrifice") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.hasType Semantics.CardType.creature))) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.departure none []) none none)) none none))])

/-- `keyword_abilities/extort.ron` -/
semantic_definition extort : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "extort") [] [(Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Instruction.procedure Semantics.Procedure.cast (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.stack Semantics.ZoneScope.bare)), (Semantics.Predicate.not (Semantics.Predicate.abilityHead Semantics.AbilityClass.anyOnStack))])) Semantics.PlayPayment.itsOwnCost none none none)) [] none [] none none none (Semantics.Instruction.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Instruction.pay (Semantics.Cost.mana [(Semantics.ManaSymbol.hybrid (Semantics.SimpleManaSymbol.specific (Semantics.ColorOrColorless.of Semantics.Color.white)) Semantics.Color.black)]) Semantics.PayTimes.once Semantics.PaymentOutcome.paid none none) (some (Semantics.Instruction.sequentially [(Semantics.Instruction.instruct (Semantics.NounPhrase.described Semantics.DetPhrase.each Semantics.Predicate.opponent) (Semantics.Instruction.changeLife (Semantics.Delta.down (Semantics.Amount.lit 1)))), (Semantics.Instruction.changeLife (Semantics.Delta.up Semantics.Amount.thatMuch))])) none))])

/-- `keyword_abilities/fabricate.ron` -/
semantic_definition fabricate (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "fabricate") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0))] [(Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none [])) [] none [] none none none (Semantics.Instruction.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Instruction.putCounters (Semantics.Amount.inCaller arg0) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.CounterBatch.one true) none (some (Semantics.Instruction.createObject (Semantics.Amount.inCaller arg0) (Semantics.CreationSpec.token (Semantics.TokenSpec.written ({ characteristics := ({ name := none, cost := none, colors := [], supertypes := [], types := [Semantics.CardType.artifact, Semantics.CardType.creature], subtypes := [(Semantics.Subtype.of Semantics.CardType.creature "Servo")], text := [], power := (some (Semantics.Amount.lit 1)), toughness := (some (Semantics.Amount.lit 1)), loyalty := none, defense := none } : Semantics.Characteristics), qualities := [] } : Semantics.CharacteristicBundle)) []) true none))))])

/-- `keyword_abilities/fading.ron` -/
semantic_definition fading (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "fading") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.entryRider (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.EntryRider.withCounters (Semantics.Amount.inCaller arg0) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "fadeCounter")) Semantics.EntryCounterMark.fresh))), (Semantics.Ability.triggered (Semantics.GameEvent.beginningOf Semantics.TurnPart.upkeep (some Semantics.NounPhrase.you) (Semantics.TurnRef.each none)) [] none [] none none none (Semantics.Instruction.withContinuation Semantics.ContinuationPolicy.required (Semantics.Instruction.removeCounters (some (Semantics.Quantity.range (some 1) (some 1))) (some (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "fadeCounter"))) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.CounterBatch.one true) none (some (Semantics.Instruction.ownScope (some (Semantics.Deed.action "sacrifice")) (Semantics.Instruction.enact (Semantics.Deed.action "sacrifice") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.departure none []) none none)))))])

/-- `keyword_abilities/fear.ron` -/
semantic_definition fear : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "fear") [] [(Semantics.Ability.static (Semantics.StaticSpec.cant Semantics.Subject.any (Semantics.Macros.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.block) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) ({ bound := none, asThough := none, rider := Semantics.DeonticRider.noRider, except := (some (Semantics.Cause.byObjects (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.or [(Semantics.Predicate.hasType Semantics.CardType.artifact), (Semantics.Predicate.colorIs Semantics.Color.black)])]))) } : Semantics.DeonticMods)))])

/-- `keyword_abilities/firebending.ron` -/
semantic_definition firebending (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "firebending") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0))] [(Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) (Semantics.Instruction.attack none)) [] none [] none none none (Semantics.Instruction.sequentially [(Semantics.Instruction.addMana (Semantics.Amount.inCaller arg0) (Semantics.ProducedMana.runs [[(Semantics.ColorOrColorless.of Semantics.Color.red)]]) []), (Semantics.Instruction.establish (Semantics.StaticSpec.manaRetention Semantics.NounPhrase.you Semantics.ManaHeld.thisMana) (some (Semantics.Duration.between none (some (Semantics.GameEvent.endOf Semantics.TurnPart.combat none Semantics.TurnRef.current)))))]))])

/-- `keyword_abilities/firstStrike.ron` -/
semantic_definition firstStrike : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "firstStrike") [] [])

/-- `keyword_abilities/flanking.ron` -/
semantic_definition flanking : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "flanking") [] [(Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.by (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.not (Semantics.Predicate.hasKeyword (Semantics.KeywordTerm.the (Semantics.KeywordLabel.named "flanking"))))]))) (Semantics.Instruction.block (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none))) [] none [] none none none (Semantics.Instruction.establish (Semantics.StaticSpec.ptModification (Semantics.NounPhrase.pro (Semantics.Reach.word (Semantics.NounWord.type Semantics.CardType.creature)) Semantics.Plurality.one) (Semantics.Delta.down (Semantics.Amount.lit 1)) (Semantics.Delta.down (Semantics.Amount.lit 1))) (some (Semantics.Duration.between none (some (Semantics.GameEvent.endOf Semantics.TurnPart.turn none Semantics.TurnRef.current))))))])

/-- `keyword_abilities/flash.ron` -/
semantic_definition flash : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "flash") [] [(Semantics.Ability.static (Semantics.StaticSpec.may (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Macros.deonticActionFor (Semantics.Deed.action "play") Semantics.NounPhrase.this) ({ bound := none, asThough := none, rider := (Semantics.DeonticRider.play none none (some (Semantics.Timing.onlyIf (Semantics.Condition.hasPriority Semantics.NounPhrase.you))) false Semantics.PlayPayment.itsOwnCost), except := none } : Semantics.DeonticMods)))])

/-- `keyword_abilities/flashback.ron` -/
semantic_definition flashback (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "flashback") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.may (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Macros.deonticActionFor (Semantics.Deed.action "cast") Semantics.NounPhrase.this) ({ bound := none, asThough := none, rider := (Semantics.DeonticRider.play (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.you))) none none false (Semantics.PlayPayment.payingInstead (Semantics.Cost.inCaller arg0))), except := none } : Semantics.DeonticMods))), (Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.instead (Semantics.WouldClause.would Semantics.Subject.any (Semantics.Instruction.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.stack Semantics.ZoneScope.bare)]) none Semantics.ObservationPoint.departure none [])) (Semantics.Instruction.ownScope (some (Semantics.Deed.action "exile")) (Semantics.Instruction.enact (Semantics.Deed.action "exile") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.exile Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) none none)) none Semantics.ReplUse.repeatedly none) (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "flashback")) none) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) Semantics.CondMarking.asLongAs))])

/-- `keyword_abilities/flying.ron` -/
semantic_definition flying : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "flying") [] [(Semantics.Ability.static (Semantics.StaticSpec.cant Semantics.Subject.any (Semantics.Macros.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.block) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) ({ bound := none, asThough := none, rider := Semantics.DeonticRider.noRider, except := (some (Semantics.Cause.byObjects (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.or [(Semantics.Predicate.hasKeyword (Semantics.KeywordTerm.the (Semantics.KeywordLabel.named "flying"))), (Semantics.Predicate.hasKeyword (Semantics.KeywordTerm.the (Semantics.KeywordLabel.named "reach")))])]))) } : Semantics.DeonticMods)))])

/-- `keyword_abilities/forMirrodin.ron` -/
semantic_definition forMirrodin : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "forMirrodin") [] [(Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.move (Semantics.NounPhrase.asType Semantics.CardType.artifact Semantics.NounPhrase.this (some (Semantics.Subtype.of Semantics.CardType.artifact "Equipment"))) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none [])) [] none [] none none none (Semantics.Instruction.sequentially [(Semantics.Instruction.createObject (Semantics.Amount.lit 1) (Semantics.CreationSpec.token (Semantics.TokenSpec.written ({ characteristics := ({ name := none, cost := none, colors := [Semantics.Color.red], supertypes := [], types := [Semantics.CardType.creature], subtypes := [(Semantics.Subtype.of Semantics.CardType.creature "Rebel")], text := [], power := (some (Semantics.Amount.lit 2)), toughness := (some (Semantics.Amount.lit 2)), loyalty := none, defense := none } : Semantics.Characteristics), qualities := [] } : Semantics.CharacteristicBundle)) []) true none), (Semantics.Instruction.ownScope (some (Semantics.Deed.action "attach")) (Semantics.Instruction.enact (Semantics.Deed.action "attach") (Semantics.Instruction.attachment Semantics.AttachMove.attached (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asType Semantics.CardType.artifact Semantics.NounPhrase.this (some (Semantics.Subtype.of Semantics.CardType.artifact "Equipment")))) (some (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.pro Semantics.Reach.tokenBorn Semantics.Plurality.one)))) none none))]))])

/-- `keyword_abilities/forecast.ron` -/
semantic_definition forecast (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "forecast") [(Semantics.KeywordParam.ability arg0)] [(Semantics.Ability.qualified arg0 (some (Semantics.Timing.inPart Semantics.TurnPart.upkeep (some (Semantics.NounPhrase.possessorOf Semantics.PossessorAxis.owner Semantics.NounPhrase.this)) Semantics.TurnRef.current)) (some (Semantics.UsageLimit.mk (Semantics.CountBound.moreThan (Semantics.Amount.lit 1)) Semantics.Period.turn)) none (some Semantics.Zone.hand))])

/-- `keyword_abilities/foretell.ron` -/
semantic_definition foretell (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "foretell") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.specialAction (some (Semantics.Timing.inPart Semantics.TurnPart.turn (some Semantics.NounPhrase.you) Semantics.TurnRef.current)) (Semantics.Cost.mana [(Semantics.ManaSymbol.simple (Semantics.SimpleManaSymbol.generic 2))]) (Semantics.Instruction.sequentially [(Semantics.Instruction.enact (Semantics.Deed.action "exile") (Semantics.Instruction.move Semantics.NounPhrase.this Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.exile Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none [(Semantics.MoveRider.placement (Semantics.PilePlacement.faceDown ({ characteristics := none } : Semantics.FaceDown)))]) none none), (Semantics.Instruction.establish (Semantics.StaticSpec.visibility Semantics.ExposeVerb.lookAt Semantics.NounPhrase.you (Semantics.VisibleThing.objects (Semantics.NounPhrase.pro Semantics.Reach.bare Semantics.Plurality.one))) (some (Semantics.Duration.forAsLongAs (Semantics.Condition.matches (Semantics.NounPhrase.pro Semantics.Reach.bare Semantics.Plurality.one) (Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.exile Semantics.ZoneScope.bare)))))), (Semantics.Instruction.establish (Semantics.StaticSpec.may (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Macros.deonticActionFor (Semantics.Deed.action "cast") (Semantics.NounPhrase.pro Semantics.Reach.bare Semantics.Plurality.one)) ({ bound := none, asThough := none, rider := (Semantics.DeonticRider.play (some (Semantics.ZoneExpr.zone Semantics.Zone.exile Semantics.ZoneScope.bare)) none none false (Semantics.PlayPayment.payingInstead (Semantics.Cost.inCaller arg0))), except := none } : Semantics.DeonticMods)) (some (Semantics.Duration.between (some (Semantics.GameEvent.endOf Semantics.TurnPart.turn none Semantics.TurnRef.current)) none)))])))])

/-- `keyword_abilities/fortify.ron` -/
semantic_definition fortify (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "fortify") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.activated (Semantics.Cost.inCaller arg0) (Semantics.Instruction.ownScope (some (Semantics.Deed.action "attach")) (Semantics.Instruction.enact (Semantics.Deed.action "attach") (Semantics.Instruction.attachment Semantics.AttachMove.attached (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) (some (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.described (Semantics.DetPhrase.target (Semantics.Quantity.range (some 1) (some 1))) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.land), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)]))))) none none)) (some (Semantics.Timing.and [(Semantics.Timing.inPart Semantics.TurnPart.mainPhase (some Semantics.NounPhrase.you) Semantics.TurnRef.current), (Semantics.Timing.onlyIf (Semantics.Condition.not (Semantics.Condition.exists_ (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.stack Semantics.ZoneScope.bare))))))])) none none none)])

/-- `keyword_abilities/freerunning.ron` -/
semantic_definition freerunning (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "freerunning") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [])

/-- `keyword_abilities/frenzy.ron` -/
semantic_definition frenzy (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "frenzy") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0))] [])

/-- `keyword_abilities/fuse.ron` -/
semantic_definition fuse : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "fuse") [] [])

/-- `keyword_abilities/gift.ron` -/
semantic_definition gift (arg0 : Semantics.Instruction) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "gift") [] [(Semantics.Ability.static (Semantics.StaticSpec.addedCost (Semantics.Cost.perform (Semantics.Instruction.choose none (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) Semantics.Predicate.opponent) Semantics.Disclosure.openly none)) true Semantics.PayTimes.once)), (Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none [])) [] none [] none none (some (Semantics.Condition.matches Semantics.NounPhrase.this (Semantics.Predicate.or [(Semantics.Predicate.hasType Semantics.CardType.artifact), (Semantics.Predicate.hasType Semantics.CardType.battle), (Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasType Semantics.CardType.enchantment), (Semantics.Predicate.hasType Semantics.CardType.land), (Semantics.Predicate.hasType Semantics.CardType.planeswalker)]))) (Semantics.Instruction.doIf (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift")) none) Semantics.NounPhrase.this) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) (Semantics.Instruction.instruct (Semantics.NounPhrase.paid (Semantics.LinkedFacet.chosenPlayer (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift"))) Semantics.NounPhrase.this) (Semantics.Instruction.inCaller arg0)) none)), (Semantics.Ability.spell none (Semantics.Instruction.doIf (Semantics.Condition.matches Semantics.NounPhrase.this (Semantics.Predicate.or [(Semantics.Predicate.hasType Semantics.CardType.instant), (Semantics.Predicate.hasType Semantics.CardType.sorcery)])) (Semantics.Instruction.doIf (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift")) none) Semantics.NounPhrase.this) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) (Semantics.Instruction.instruct (Semantics.NounPhrase.paid (Semantics.LinkedFacet.chosenPlayer (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift"))) Semantics.NounPhrase.this) (Semantics.Instruction.inCaller arg0)) none) none))])

/-- `keyword_abilities/giftACard.ron` -/
semantic_definition giftACard : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "gift") [] [(Semantics.Ability.static (Semantics.StaticSpec.addedCost (Semantics.Cost.perform (Semantics.Instruction.choose none (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) Semantics.Predicate.opponent) Semantics.Disclosure.openly none)) true Semantics.PayTimes.once)), (Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none [])) [] none [] none none (some (Semantics.Condition.matches Semantics.NounPhrase.this (Semantics.Predicate.or [(Semantics.Predicate.hasType Semantics.CardType.artifact), (Semantics.Predicate.hasType Semantics.CardType.battle), (Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasType Semantics.CardType.enchantment), (Semantics.Predicate.hasType Semantics.CardType.land), (Semantics.Predicate.hasType Semantics.CardType.planeswalker)]))) (Semantics.Instruction.doIf (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift")) none) Semantics.NounPhrase.this) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) (Semantics.Instruction.instruct (Semantics.NounPhrase.paid (Semantics.LinkedFacet.chosenPlayer (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift"))) Semantics.NounPhrase.this) (Semantics.Instruction.draw (Semantics.Amount.lit 1))) none)), (Semantics.Ability.spell none (Semantics.Instruction.doIf (Semantics.Condition.matches Semantics.NounPhrase.this (Semantics.Predicate.or [(Semantics.Predicate.hasType Semantics.CardType.instant), (Semantics.Predicate.hasType Semantics.CardType.sorcery)])) (Semantics.Instruction.doIf (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift")) none) Semantics.NounPhrase.this) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) (Semantics.Instruction.instruct (Semantics.NounPhrase.paid (Semantics.LinkedFacet.chosenPlayer (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift"))) Semantics.NounPhrase.this) (Semantics.Instruction.draw (Semantics.Amount.lit 1))) none) none))])

/-- `keyword_abilities/giftAFood.ron` -/
semantic_definition giftAFood : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "gift") [] [(Semantics.Ability.static (Semantics.StaticSpec.addedCost (Semantics.Cost.perform (Semantics.Instruction.choose none (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) Semantics.Predicate.opponent) Semantics.Disclosure.openly none)) true Semantics.PayTimes.once)), (Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none [])) [] none [] none none (some (Semantics.Condition.matches Semantics.NounPhrase.this (Semantics.Predicate.or [(Semantics.Predicate.hasType Semantics.CardType.artifact), (Semantics.Predicate.hasType Semantics.CardType.battle), (Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasType Semantics.CardType.enchantment), (Semantics.Predicate.hasType Semantics.CardType.land), (Semantics.Predicate.hasType Semantics.CardType.planeswalker)]))) (Semantics.Instruction.doIf (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift")) none) Semantics.NounPhrase.this) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) (Semantics.Instruction.instruct (Semantics.NounPhrase.paid (Semantics.LinkedFacet.chosenPlayer (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift"))) Semantics.NounPhrase.this) (Semantics.Instruction.createObject (Semantics.Amount.lit 1) (Semantics.CreationSpec.token (Semantics.TokenSpec.written ({ characteristics := ({ name := none, cost := none, colors := [], supertypes := [], types := [Semantics.CardType.artifact], subtypes := [(Semantics.Subtype.of Semantics.CardType.artifact "Food")], text := [(Semantics.Ability.activated (Semantics.Cost.compound [(Semantics.Cost.mana [(Semantics.ManaSymbol.simple (Semantics.SimpleManaSymbol.generic 2))]), Semantics.Cost.tapSymbol, (Semantics.Cost.perform (Semantics.Instruction.ownScope (some (Semantics.Deed.action "sacrifice")) (Semantics.Instruction.enact (Semantics.Deed.action "sacrifice") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asType Semantics.CardType.artifact Semantics.NounPhrase.this none)) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.departure none []) none none)))]) (Semantics.Instruction.changeLife (Semantics.Delta.up (Semantics.Amount.lit 3))) none none none none)], power := none, toughness := none, loyalty := none, defense := none } : Semantics.Characteristics), qualities := [] } : Semantics.CharacteristicBundle)) []) true none)) none)), (Semantics.Ability.spell none (Semantics.Instruction.doIf (Semantics.Condition.matches Semantics.NounPhrase.this (Semantics.Predicate.or [(Semantics.Predicate.hasType Semantics.CardType.instant), (Semantics.Predicate.hasType Semantics.CardType.sorcery)])) (Semantics.Instruction.doIf (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift")) none) Semantics.NounPhrase.this) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) (Semantics.Instruction.instruct (Semantics.NounPhrase.paid (Semantics.LinkedFacet.chosenPlayer (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift"))) Semantics.NounPhrase.this) (Semantics.Instruction.createObject (Semantics.Amount.lit 1) (Semantics.CreationSpec.token (Semantics.TokenSpec.written ({ characteristics := ({ name := none, cost := none, colors := [], supertypes := [], types := [Semantics.CardType.artifact], subtypes := [(Semantics.Subtype.of Semantics.CardType.artifact "Food")], text := [(Semantics.Ability.activated (Semantics.Cost.compound [(Semantics.Cost.mana [(Semantics.ManaSymbol.simple (Semantics.SimpleManaSymbol.generic 2))]), Semantics.Cost.tapSymbol, (Semantics.Cost.perform (Semantics.Instruction.ownScope (some (Semantics.Deed.action "sacrifice")) (Semantics.Instruction.enact (Semantics.Deed.action "sacrifice") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asType Semantics.CardType.artifact Semantics.NounPhrase.this none)) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.departure none []) none none)))]) (Semantics.Instruction.changeLife (Semantics.Delta.up (Semantics.Amount.lit 3))) none none none none)], power := none, toughness := none, loyalty := none, defense := none } : Semantics.Characteristics), qualities := [] } : Semantics.CharacteristicBundle)) []) true none)) none) none))])

/-- `keyword_abilities/giftATappedFish.ron` -/
semantic_definition giftATappedFish : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "gift") [] [(Semantics.Ability.static (Semantics.StaticSpec.addedCost (Semantics.Cost.perform (Semantics.Instruction.choose none (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) Semantics.Predicate.opponent) Semantics.Disclosure.openly none)) true Semantics.PayTimes.once)), (Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none [])) [] none [] none none (some (Semantics.Condition.matches Semantics.NounPhrase.this (Semantics.Predicate.or [(Semantics.Predicate.hasType Semantics.CardType.artifact), (Semantics.Predicate.hasType Semantics.CardType.battle), (Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasType Semantics.CardType.enchantment), (Semantics.Predicate.hasType Semantics.CardType.land), (Semantics.Predicate.hasType Semantics.CardType.planeswalker)]))) (Semantics.Instruction.doIf (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift")) none) Semantics.NounPhrase.this) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) (Semantics.Instruction.instruct (Semantics.NounPhrase.paid (Semantics.LinkedFacet.chosenPlayer (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift"))) Semantics.NounPhrase.this) (Semantics.Instruction.createObject (Semantics.Amount.lit 1) (Semantics.CreationSpec.token (Semantics.TokenSpec.written ({ characteristics := ({ name := none, cost := none, colors := [Semantics.Color.blue], supertypes := [], types := [Semantics.CardType.creature], subtypes := [(Semantics.Subtype.of Semantics.CardType.creature "Fish")], text := [], power := (some (Semantics.Amount.lit 1)), toughness := (some (Semantics.Amount.lit 1)), loyalty := none, defense := none } : Semantics.Characteristics), qualities := [] } : Semantics.CharacteristicBundle)) [(Semantics.EntryRider.entersAs Semantics.Status.tapped)]) true none)) none)), (Semantics.Ability.spell none (Semantics.Instruction.doIf (Semantics.Condition.matches Semantics.NounPhrase.this (Semantics.Predicate.or [(Semantics.Predicate.hasType Semantics.CardType.instant), (Semantics.Predicate.hasType Semantics.CardType.sorcery)])) (Semantics.Instruction.doIf (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift")) none) Semantics.NounPhrase.this) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) (Semantics.Instruction.instruct (Semantics.NounPhrase.paid (Semantics.LinkedFacet.chosenPlayer (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift"))) Semantics.NounPhrase.this) (Semantics.Instruction.createObject (Semantics.Amount.lit 1) (Semantics.CreationSpec.token (Semantics.TokenSpec.written ({ characteristics := ({ name := none, cost := none, colors := [Semantics.Color.blue], supertypes := [], types := [Semantics.CardType.creature], subtypes := [(Semantics.Subtype.of Semantics.CardType.creature "Fish")], text := [], power := (some (Semantics.Amount.lit 1)), toughness := (some (Semantics.Amount.lit 1)), loyalty := none, defense := none } : Semantics.Characteristics), qualities := [] } : Semantics.CharacteristicBundle)) [(Semantics.EntryRider.entersAs Semantics.Status.tapped)]) true none)) none) none))])

/-- `keyword_abilities/giftATreasure.ron` -/
semantic_definition giftATreasure : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "gift") [] [(Semantics.Ability.static (Semantics.StaticSpec.addedCost (Semantics.Cost.perform (Semantics.Instruction.choose none (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) Semantics.Predicate.opponent) Semantics.Disclosure.openly none)) true Semantics.PayTimes.once)), (Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none [])) [] none [] none none (some (Semantics.Condition.matches Semantics.NounPhrase.this (Semantics.Predicate.or [(Semantics.Predicate.hasType Semantics.CardType.artifact), (Semantics.Predicate.hasType Semantics.CardType.battle), (Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasType Semantics.CardType.enchantment), (Semantics.Predicate.hasType Semantics.CardType.land), (Semantics.Predicate.hasType Semantics.CardType.planeswalker)]))) (Semantics.Instruction.doIf (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift")) none) Semantics.NounPhrase.this) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) (Semantics.Instruction.instruct (Semantics.NounPhrase.paid (Semantics.LinkedFacet.chosenPlayer (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift"))) Semantics.NounPhrase.this) (Semantics.Instruction.createObject (Semantics.Amount.lit 1) (Semantics.CreationSpec.token (Semantics.TokenSpec.written ({ characteristics := ({ name := none, cost := none, colors := [], supertypes := [], types := [Semantics.CardType.artifact], subtypes := [(Semantics.Subtype.of Semantics.CardType.artifact "Treasure")], text := [(Semantics.Ability.activated (Semantics.Cost.compound [Semantics.Cost.tapSymbol, (Semantics.Cost.perform (Semantics.Instruction.ownScope (some (Semantics.Deed.action "sacrifice")) (Semantics.Instruction.enact (Semantics.Deed.action "sacrifice") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asType Semantics.CardType.artifact Semantics.NounPhrase.this none)) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.departure none []) none none)))]) (Semantics.Instruction.addMana (Semantics.Amount.lit 1) (Semantics.ProducedMana.anyColor Semantics.ColorFreedom.sameColor) []) none none none none)], power := none, toughness := none, loyalty := none, defense := none } : Semantics.Characteristics), qualities := [] } : Semantics.CharacteristicBundle)) []) true none)) none)), (Semantics.Ability.spell none (Semantics.Instruction.doIf (Semantics.Condition.matches Semantics.NounPhrase.this (Semantics.Predicate.or [(Semantics.Predicate.hasType Semantics.CardType.instant), (Semantics.Predicate.hasType Semantics.CardType.sorcery)])) (Semantics.Instruction.doIf (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift")) none) Semantics.NounPhrase.this) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) (Semantics.Instruction.instruct (Semantics.NounPhrase.paid (Semantics.LinkedFacet.chosenPlayer (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift"))) Semantics.NounPhrase.this) (Semantics.Instruction.createObject (Semantics.Amount.lit 1) (Semantics.CreationSpec.token (Semantics.TokenSpec.written ({ characteristics := ({ name := none, cost := none, colors := [], supertypes := [], types := [Semantics.CardType.artifact], subtypes := [(Semantics.Subtype.of Semantics.CardType.artifact "Treasure")], text := [(Semantics.Ability.activated (Semantics.Cost.compound [Semantics.Cost.tapSymbol, (Semantics.Cost.perform (Semantics.Instruction.ownScope (some (Semantics.Deed.action "sacrifice")) (Semantics.Instruction.enact (Semantics.Deed.action "sacrifice") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asType Semantics.CardType.artifact Semantics.NounPhrase.this none)) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.departure none []) none none)))]) (Semantics.Instruction.addMana (Semantics.Amount.lit 1) (Semantics.ProducedMana.anyColor Semantics.ColorFreedom.sameColor) []) none none none none)], power := none, toughness := none, loyalty := none, defense := none } : Semantics.Characteristics), qualities := [] } : Semantics.CharacteristicBundle)) []) true none)) none) none))])

/-- `keyword_abilities/giftAnExtraTurn.ron` -/
semantic_definition giftAnExtraTurn : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "gift") [] [(Semantics.Ability.static (Semantics.StaticSpec.addedCost (Semantics.Cost.perform (Semantics.Instruction.choose none (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) Semantics.Predicate.opponent) Semantics.Disclosure.openly none)) true Semantics.PayTimes.once)), (Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none [])) [] none [] none none (some (Semantics.Condition.matches Semantics.NounPhrase.this (Semantics.Predicate.or [(Semantics.Predicate.hasType Semantics.CardType.artifact), (Semantics.Predicate.hasType Semantics.CardType.battle), (Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasType Semantics.CardType.enchantment), (Semantics.Predicate.hasType Semantics.CardType.land), (Semantics.Predicate.hasType Semantics.CardType.planeswalker)]))) (Semantics.Instruction.doIf (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift")) none) Semantics.NounPhrase.this) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) (Semantics.Instruction.instruct (Semantics.NounPhrase.paid (Semantics.LinkedFacet.chosenPlayer (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift"))) Semantics.NounPhrase.this) (Semantics.Instruction.addTurnPart Semantics.TurnPart.turn none (Semantics.Amount.lit 1) none)) none)), (Semantics.Ability.spell none (Semantics.Instruction.doIf (Semantics.Condition.matches Semantics.NounPhrase.this (Semantics.Predicate.or [(Semantics.Predicate.hasType Semantics.CardType.instant), (Semantics.Predicate.hasType Semantics.CardType.sorcery)])) (Semantics.Instruction.doIf (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift")) none) Semantics.NounPhrase.this) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) (Semantics.Instruction.instruct (Semantics.NounPhrase.paid (Semantics.LinkedFacet.chosenPlayer (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift"))) Semantics.NounPhrase.this) (Semantics.Instruction.addTurnPart Semantics.TurnPart.turn none (Semantics.Amount.lit 1) none)) none) none))])

/-- `keyword_abilities/giftAnOctopus.ron` -/
semantic_definition giftAnOctopus : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "gift") [] [(Semantics.Ability.static (Semantics.StaticSpec.addedCost (Semantics.Cost.perform (Semantics.Instruction.choose none (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) Semantics.Predicate.opponent) Semantics.Disclosure.openly none)) true Semantics.PayTimes.once)), (Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none [])) [] none [] none none (some (Semantics.Condition.matches Semantics.NounPhrase.this (Semantics.Predicate.or [(Semantics.Predicate.hasType Semantics.CardType.artifact), (Semantics.Predicate.hasType Semantics.CardType.battle), (Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasType Semantics.CardType.enchantment), (Semantics.Predicate.hasType Semantics.CardType.land), (Semantics.Predicate.hasType Semantics.CardType.planeswalker)]))) (Semantics.Instruction.doIf (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift")) none) Semantics.NounPhrase.this) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) (Semantics.Instruction.instruct (Semantics.NounPhrase.paid (Semantics.LinkedFacet.chosenPlayer (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift"))) Semantics.NounPhrase.this) (Semantics.Instruction.createObject (Semantics.Amount.lit 1) (Semantics.CreationSpec.token (Semantics.TokenSpec.written ({ characteristics := ({ name := none, cost := none, colors := [Semantics.Color.blue], supertypes := [], types := [Semantics.CardType.creature], subtypes := [(Semantics.Subtype.of Semantics.CardType.creature "Octopus")], text := [], power := (some (Semantics.Amount.lit 8)), toughness := (some (Semantics.Amount.lit 8)), loyalty := none, defense := none } : Semantics.Characteristics), qualities := [] } : Semantics.CharacteristicBundle)) []) true none)) none)), (Semantics.Ability.spell none (Semantics.Instruction.doIf (Semantics.Condition.matches Semantics.NounPhrase.this (Semantics.Predicate.or [(Semantics.Predicate.hasType Semantics.CardType.instant), (Semantics.Predicate.hasType Semantics.CardType.sorcery)])) (Semantics.Instruction.doIf (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift")) none) Semantics.NounPhrase.this) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) (Semantics.Instruction.instruct (Semantics.NounPhrase.paid (Semantics.LinkedFacet.chosenPlayer (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "gift"))) Semantics.NounPhrase.this) (Semantics.Instruction.createObject (Semantics.Amount.lit 1) (Semantics.CreationSpec.token (Semantics.TokenSpec.written ({ characteristics := ({ name := none, cost := none, colors := [Semantics.Color.blue], supertypes := [], types := [Semantics.CardType.creature], subtypes := [(Semantics.Subtype.of Semantics.CardType.creature "Octopus")], text := [], power := (some (Semantics.Amount.lit 8)), toughness := (some (Semantics.Amount.lit 8)), loyalty := none, defense := none } : Semantics.Characteristics), qualities := [] } : Semantics.CharacteristicBundle)) []) true none)) none) none))])

/-- `keyword_abilities/graft.ron` -/
semantic_definition graft (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "graft") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.entryRider (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.EntryRider.withCounters (Semantics.Amount.inCaller arg0) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) Semantics.EntryCounterMark.fresh))), (Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.move (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.otherThan (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this))])) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none [])) [] none [] none none (some (Semantics.Condition.matches (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.Predicate.hasCounters (some (Semantics.CounterKind.named "p1p1Counter"))))) (Semantics.Instruction.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Instruction.moveCounters (Semantics.Amount.lit 1) (some (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter"))) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.NounPhrase.pro (Semantics.Reach.word (Semantics.NounWord.type Semantics.CardType.creature)) Semantics.Plurality.one)) none none))])

/-- `keyword_abilities/gravestorm.ron` -/
semantic_definition gravestorm : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "gravestorm") [] [(Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Instruction.procedure Semantics.Procedure.cast (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) Semantics.PlayPayment.itsOwnCost none none none)) [] none [] none none none (Semantics.Instruction.sequentially [(Semantics.Instruction.copy Semantics.CopySort.fromStack (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) (Semantics.Amount.tally (Semantics.DidClause.mk Semantics.Subject.any (Semantics.Instruction.move (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare))) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.departure none []) (Semantics.LookbackSpan.duration (Semantics.Duration.between (some (Semantics.GameEvent.beginningOf Semantics.TurnPart.turn none Semantics.TurnRef.current)) (some Semantics.GameEvent.now)) none)) Semantics.TallyMode.events) []), (Semantics.Instruction.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Instruction.chooseNewTargets (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.copy) Semantics.Plurality.many)) none none)]))])

/-- `keyword_abilities/harmonize.ron` -/
semantic_definition harmonize (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "harmonize") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.may (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Macros.deonticActionFor (Semantics.Deed.action "cast") Semantics.NounPhrase.this) ({ bound := none, asThough := none, rider := (Semantics.DeonticRider.play (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.you))) none none false (Semantics.PlayPayment.payingInstead (Semantics.Cost.compound [(Semantics.Cost.inCaller arg0), (Semantics.Cost.perform (Semantics.Instruction.ownScope (some (Semantics.Deed.action "tap")) (Semantics.Instruction.enact (Semantics.Deed.action "tap") (Semantics.Instruction.setStatus Semantics.Status.tapped (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.described (Semantics.DetPhrase.count (Semantics.Quantity.range none (some 1)) none) (Semantics.Predicate.and [(Semantics.Predicate.hasStatus Semantics.Status.untapped), (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)])])))) none none)))]))), except := none } : Semantics.DeonticMods))), (Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.costShift (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) (Semantics.CostShift.less (Semantics.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.power) (Semantics.NounPhrase.paid (Semantics.LinkedFacet.objects Semantics.LinkSource.theAlternative (Semantics.Deed.action "tap") Semantics.Plurality.one) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this))) none)) (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "harmonize")) none) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) Semantics.CondMarking.asLongAs)), (Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.instead (Semantics.WouldClause.would Semantics.Subject.any (Semantics.Instruction.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.stack Semantics.ZoneScope.bare)]) none Semantics.ObservationPoint.departure none [])) (Semantics.Instruction.ownScope (some (Semantics.Deed.action "exile")) (Semantics.Instruction.enact (Semantics.Deed.action "exile") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.exile Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) none none)) none Semantics.ReplUse.repeatedly none) (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "harmonize")) none) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) Semantics.CondMarking.asLongAs))])

/-- `keyword_abilities/haste.ron` -/
semantic_definition haste : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "haste") [] [])

/-- `keyword_abilities/haunt.ron` -/
semantic_definition haunt : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "haunt") [] [(Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.departure none [])) [] none [] none none (some (Semantics.Condition.matches Semantics.NounPhrase.this (Semantics.Predicate.or [(Semantics.Predicate.hasType Semantics.CardType.artifact), (Semantics.Predicate.hasType Semantics.CardType.battle), (Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasType Semantics.CardType.enchantment), (Semantics.Predicate.hasType Semantics.CardType.land), (Semantics.Predicate.hasType Semantics.CardType.planeswalker)]))) (Semantics.Instruction.sequentially [(Semantics.Instruction.ownScope (some (Semantics.Deed.action "exile")) (Semantics.Instruction.enact (Semantics.Deed.action "exile") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.pro Semantics.Reach.bare Semantics.Plurality.one)) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.exile Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) none none)), (Semantics.Instruction.relate (Semantics.NounPhrase.pro Semantics.Reach.bare Semantics.Plurality.one) (Semantics.RelationLabel.named "haunting") (Semantics.NounPhrase.described (Semantics.DetPhrase.target (Semantics.Quantity.range (some 1) (some 1))) (Semantics.Predicate.hasType Semantics.CardType.creature)))])), (Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.stack Semantics.ZoneScope.bare)]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.departure none [])) [] none [] none none (some (Semantics.Condition.matches Semantics.NounPhrase.this (Semantics.Predicate.or [(Semantics.Predicate.hasType Semantics.CardType.instant), (Semantics.Predicate.hasType Semantics.CardType.sorcery)]))) (Semantics.Instruction.sequentially [(Semantics.Instruction.ownScope (some (Semantics.Deed.action "exile")) (Semantics.Instruction.enact (Semantics.Deed.action "exile") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.pro Semantics.Reach.bare Semantics.Plurality.one)) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.exile Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) none none)), (Semantics.Instruction.relate (Semantics.NounPhrase.pro Semantics.Reach.bare Semantics.Plurality.one) (Semantics.RelationLabel.named "haunting") (Semantics.NounPhrase.described (Semantics.DetPhrase.target (Semantics.Quantity.range (some 1) (some 1))) (Semantics.Predicate.hasType Semantics.CardType.creature)))]))])

/-- `keyword_abilities/hexproof.ron` -/
semantic_definition hexproof : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "hexproof") [] [(Semantics.Ability.static (Semantics.StaticSpec.cant (Semantics.Subject.by (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.and [(Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.stack Semantics.ZoneScope.bare)), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller (Semantics.NounPhrase.playerGroup Semantics.PlayerGroupWord.yourOpponents))]))) (Semantics.Macros.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.target) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) ({ bound := none, asThough := none, rider := Semantics.DeonticRider.noRider, except := none } : Semantics.DeonticMods)))])

/-- `keyword_abilities/hideaway.ron` -/
semantic_definition hideaway (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "hideaway") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0))] [(Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none [])) [] none [] none none none (Semantics.Instruction.sequentially [(Semantics.Instruction.expose Semantics.ExposeVerb.lookAt (Semantics.Exposed.cards (Semantics.NounPhrase.slice Semantics.LibraryEnd.top (Semantics.Amount.inCaller arg0) (Semantics.Pile.library (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.you))))), (Semantics.Instruction.enact (Semantics.Deed.action "exile") (Semantics.Instruction.move (Semantics.NounPhrase.someOf (Semantics.SliceCount.counted (Semantics.Quantity.range (some 1) (some 1))) none (Semantics.NounPhrase.pro Semantics.Reach.bare Semantics.Plurality.many)) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.exile Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none [(Semantics.MoveRider.placement (Semantics.PilePlacement.faceDown ({ characteristics := none } : Semantics.FaceDown)))]) none none), (Semantics.Instruction.move (Semantics.NounPhrase.theRest Semantics.Kind.object Semantics.Plurality.many) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.place Semantics.LibraryEnd.bottom (Semantics.Pile.library Semantics.ZoneScope.bare))) Semantics.ObservationPoint.arrival none [(Semantics.MoveRider.placement (Semantics.PilePlacement.inOrder Semantics.Arrangement.randomOrder))]), (Semantics.Instruction.establish (Semantics.StaticSpec.visibility Semantics.ExposeVerb.lookAt Semantics.NounPhrase.you (Semantics.VisibleThing.objects (Semantics.NounPhrase.pro (Semantics.Reach.verbed (Semantics.Deed.action "exile") (some Semantics.NounWord.card) Semantics.VerbedMarking.attributive) Semantics.Plurality.one))) (some (Semantics.Duration.forAsLongAs (Semantics.Condition.matches (Semantics.NounPhrase.pro (Semantics.Reach.verbed (Semantics.Deed.action "exile") (some Semantics.NounWord.card) Semantics.VerbedMarking.attributive) Semantics.Plurality.one) (Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.exile Semantics.ZoneScope.bare))))))]))])

/-- `keyword_abilities/horsemanship.ron` -/
semantic_definition horsemanship : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "horsemanship") [] [(Semantics.Ability.static (Semantics.StaticSpec.cant (Semantics.Subject.by (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.not (Semantics.Predicate.hasKeyword (Semantics.KeywordTerm.the (Semantics.KeywordLabel.named "horsemanship"))))]))) (Semantics.Macros.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.block) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) ({ bound := none, asThough := none, rider := Semantics.DeonticRider.noRider, except := none } : Semantics.DeonticMods)))])

/-- `keyword_abilities/impending.ron` -/
semantic_definition impending (arg0 : Semantics.Amount) (arg1 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "impending") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0)), (Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg1))] [(Semantics.Ability.static (Semantics.StaticSpec.may (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Macros.deonticActionFor (Semantics.Deed.action "cast") Semantics.NounPhrase.this) ({ bound := none, asThough := none, rider := (Semantics.DeonticRider.play none none none false (Semantics.PlayPayment.payingInstead (Semantics.Cost.inCaller arg1))), except := none } : Semantics.DeonticMods))), (Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.entryRider (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.EntryRider.withCounters (Semantics.Amount.inCaller arg0) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "timeCounter")) Semantics.EntryCounterMark.fresh)) (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "impending")) none) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) Semantics.CondMarking.asLongAs)), (Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.characteristicChange (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) [(Semantics.CharacteristicEdit.typeLine Semantics.QualityOp.loses ({ supertypes := none, types := (some [Semantics.CardType.creature]), subtypes := none, retained := none } : Semantics.TypeLineChanges))]) (Semantics.Condition.and [(Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "impending")) none) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)), (Semantics.Condition.matches (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.Predicate.hasCounters (some (Semantics.CounterKind.named "timeCounter"))))]) Semantics.CondMarking.asLongAs)), (Semantics.Ability.triggered (Semantics.GameEvent.beginningOf Semantics.TurnPart.endStep (some Semantics.NounPhrase.you) (Semantics.TurnRef.each none)) [] none [] none none (some (Semantics.Condition.and [(Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "impending")) none) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)), (Semantics.Condition.matches (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.Predicate.hasCounters (some (Semantics.CounterKind.named "timeCounter"))))])) (Semantics.Instruction.removeCounters (some (Semantics.Quantity.range (some 1) (some 1))) (some (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "timeCounter"))) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.CounterBatch.one true))])

/-- `keyword_abilities/improvise.ron` -/
semantic_definition improvise : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "improvise") [] [(Semantics.Ability.static (Semantics.StaticSpec.paySubstitute (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) ({ action := (Semantics.Instruction.ownScope (some (Semantics.Deed.action "tap")) (Semantics.Instruction.enact (Semantics.Deed.action "tap") (Semantics.Instruction.setStatus Semantics.Status.tapped (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.hasStatus Semantics.Status.untapped), (Semantics.Predicate.hasType Semantics.CardType.artifact), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)])))) none none)), pays := [Semantics.PipPaid.generic] } : Semantics.ManaSubstitute)))])

/-- `keyword_abilities/increment.ron` -/
semantic_definition increment : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "increment") [] [(Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Instruction.procedure Semantics.Procedure.cast (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.stack Semantics.ZoneScope.bare)), (Semantics.Predicate.not (Semantics.Predicate.abilityHead Semantics.AbilityClass.anyOnStack))])) Semantics.PlayPayment.itsOwnCost none none none)) [] none [] none none (some (Semantics.Condition.and [(Semantics.Condition.matches (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.Predicate.hasType Semantics.CardType.creature)), (Semantics.Condition.or [(Semantics.Condition.compareAmt (Semantics.Amount.paid Semantics.LinkedFacet.manaValueSpent (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.spell) Semantics.Plurality.one)) Semantics.Comparator.greater (Semantics.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.power) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none))), (Semantics.Condition.compareAmt (Semantics.Amount.paid Semantics.LinkedFacet.manaValueSpent (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.spell) Semantics.Plurality.one)) Semantics.Comparator.greater (Semantics.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.toughness) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)))])])) (Semantics.Instruction.putCounters (Semantics.Amount.lit 1) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none) Semantics.CounterBatch.one true))])

/-- `keyword_abilities/indestructible.ron` -/
semantic_definition indestructible : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "indestructible") [] [(Semantics.Ability.static (Semantics.StaticSpec.cant Semantics.Subject.any (Semantics.Macros.deonticActionFor (Semantics.Deed.action "destroy") (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) ({ bound := none, asThough := none, rider := Semantics.DeonticRider.noRider, except := none } : Semantics.DeonticMods)))])

/-- `keyword_abilities/infect.ron` -/
semantic_definition infect : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "infect") [] [])

/-- `keyword_abilities/infinity.ron` -/
semantic_definition infinity (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "infinity") [(Semantics.KeywordParam.ability arg0)] [(Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.abilityGrant (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) arg0) (Semantics.Condition.matches (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.Predicate.hasDesignation (Semantics.DesignationLabel.named "harnessed") none)) Semantics.CondMarking.asLongAs))])

/-- `keyword_abilities/ingest.ron` -/
semantic_definition ingest : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "ingest") [] [(Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) (Semantics.Instruction.dealDamage Semantics.Amount.anyAmount (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) Semantics.Predicate.anyPlayer) Semantics.DamageKind.combatOnly)) [] none [] none none none (Semantics.Instruction.instruct (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.player) Semantics.Plurality.one) (Semantics.Instruction.ownScope (some (Semantics.Deed.action "exile")) (Semantics.Instruction.enact (Semantics.Deed.action "exile") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.slice Semantics.LibraryEnd.top (Semantics.Amount.lit 1) (Semantics.Pile.library (Semantics.ZoneScope.possessedBy (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.player) Semantics.Plurality.one))))) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.exile Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) none none))))])

/-- `keyword_abilities/intimidate.ron` -/
semantic_definition intimidate : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "intimidate") [] [(Semantics.Ability.static (Semantics.StaticSpec.cant Semantics.Subject.any (Semantics.Macros.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.block) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) ({ bound := none, asThough := none, rider := Semantics.DeonticRider.noRider, except := (some (Semantics.Cause.byObjects (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.or [(Semantics.Predicate.hasType Semantics.CardType.artifact), (Semantics.Predicate.shares Semantics.QualitySort.color Semantics.Agreement.share (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none))])]))) } : Semantics.DeonticMods)))])

/-- `keyword_abilities/jobSelect.ron` -/
semantic_definition jobSelect : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "jobSelect") [] [(Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.move (Semantics.NounPhrase.asType Semantics.CardType.artifact Semantics.NounPhrase.this (some (Semantics.Subtype.of Semantics.CardType.artifact "Equipment"))) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none [])) [] none [] none none none (Semantics.Instruction.sequentially [(Semantics.Instruction.createObject (Semantics.Amount.lit 1) (Semantics.CreationSpec.token (Semantics.TokenSpec.written ({ characteristics := ({ name := none, cost := none, colors := [], supertypes := [], types := [Semantics.CardType.creature], subtypes := [(Semantics.Subtype.of Semantics.CardType.creature "Hero")], text := [], power := (some (Semantics.Amount.lit 1)), toughness := (some (Semantics.Amount.lit 1)), loyalty := none, defense := none } : Semantics.Characteristics), qualities := [] } : Semantics.CharacteristicBundle)) []) true none), (Semantics.Instruction.ownScope (some (Semantics.Deed.action "attach")) (Semantics.Instruction.enact (Semantics.Deed.action "attach") (Semantics.Instruction.attachment Semantics.AttachMove.attached (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asType Semantics.CardType.artifact Semantics.NounPhrase.this (some (Semantics.Subtype.of Semantics.CardType.artifact "Equipment")))) (some (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.pro Semantics.Reach.tokenBorn Semantics.Plurality.one)))) none none))]))])

/-- `keyword_abilities/jumpStart.ron` -/
semantic_definition jumpStart : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "jumpStart") [] [])

/-- `keyword_abilities/kicker.ron` -/
semantic_definition kicker (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "kicker") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.addedCost (Semantics.Cost.inCaller arg0) true Semantics.PayTimes.once))])

/-- `keyword_abilities/landwalk.ron` -/
semantic_definition landwalk (arg0 : Semantics.Predicate) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "landwalk") [(Semantics.KeywordParam.quality (Semantics.Predicate.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.cant Semantics.Subject.any (Semantics.Macros.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.block) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) ({ bound := none, asThough := none, rider := Semantics.DeonticRider.noRider, except := none } : Semantics.DeonticMods)) (Semantics.Condition.exists_ (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.land), (Semantics.Predicate.inCaller arg0), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller (Semantics.NounPhrase.combatPlayer Semantics.CombatRole.defending))]))) Semantics.CondMarking.asLongAs))])

/-- `keyword_abilities/levelUp.ron` -/
semantic_definition levelUp (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "levelUp") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.activated (Semantics.Cost.inCaller arg0) (Semantics.Instruction.putCounters (Semantics.Amount.lit 1) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "levelCounter")) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.CounterBatch.one true) (some (Semantics.Timing.and [(Semantics.Timing.inPart Semantics.TurnPart.mainPhase (some Semantics.NounPhrase.you) Semantics.TurnRef.current), (Semantics.Timing.onlyIf (Semantics.Condition.not (Semantics.Condition.exists_ (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.stack Semantics.ZoneScope.bare))))))])) none none none)])

/-- `keyword_abilities/lifelink.ron` -/
semantic_definition lifelink : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "lifelink") [] [])

/-- `keyword_abilities/livingMetal.ron` -/
semantic_definition livingMetal : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "livingMetal") [] [(Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.characteristicChange (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) [(Semantics.CharacteristicEdit.typeLine Semantics.QualityOp.adds ({ supertypes := none, types := (some [Semantics.CardType.artifact, Semantics.CardType.creature]), subtypes := none, retained := none } : Semantics.TypeLineChanges))]) (Semantics.Condition.duringPart Semantics.TurnPart.turn (some Semantics.NounPhrase.you)) Semantics.CondMarking.asLongAs))])

/-- `keyword_abilities/livingWeapon.ron` -/
semantic_definition livingWeapon : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "livingWeapon") [] [(Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.move (Semantics.NounPhrase.asType Semantics.CardType.artifact Semantics.NounPhrase.this (some (Semantics.Subtype.of Semantics.CardType.artifact "Equipment"))) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none [])) [] none [] none none none (Semantics.Instruction.sequentially [(Semantics.Instruction.createObject (Semantics.Amount.lit 1) (Semantics.CreationSpec.token (Semantics.TokenSpec.written ({ characteristics := ({ name := none, cost := none, colors := [Semantics.Color.black], supertypes := [], types := [Semantics.CardType.creature], subtypes := [(Semantics.Subtype.of Semantics.CardType.creature "Phyrexian"), (Semantics.Subtype.of Semantics.CardType.creature "Germ")], text := [], power := (some (Semantics.Amount.lit 0)), toughness := (some (Semantics.Amount.lit 0)), loyalty := none, defense := none } : Semantics.Characteristics), qualities := [] } : Semantics.CharacteristicBundle)) []) true none), (Semantics.Instruction.ownScope (some (Semantics.Deed.action "attach")) (Semantics.Instruction.enact (Semantics.Deed.action "attach") (Semantics.Instruction.attachment Semantics.AttachMove.attached (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asType Semantics.CardType.artifact Semantics.NounPhrase.this (some (Semantics.Subtype.of Semantics.CardType.artifact "Equipment")))) (some (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.pro Semantics.Reach.tokenBorn Semantics.Plurality.one)))) none none))]))])

/-- `keyword_abilities/madness.ron` -/
semantic_definition madness (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "madness") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.instead (Semantics.WouldClause.would (Semantics.Subject.actor (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) Semantics.Predicate.anyPlayer)) (Semantics.Instruction.enact (Semantics.Deed.action "discard") (Semantics.Macros.deonticActionFor (Semantics.Deed.action "discard") Semantics.NounPhrase.this) none none)) (Semantics.Instruction.enact (Semantics.Deed.action "discard") (Semantics.Instruction.move Semantics.NounPhrase.this (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.hand Semantics.ZoneScope.bare)]) (some (Semantics.ZoneExpr.zone Semantics.Zone.exile Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) none none) none Semantics.ReplUse.repeatedly none)), (Semantics.Ability.triggered (Semantics.GameEvent.causes (Semantics.Causing.event (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.enact (Semantics.Deed.action "discard") (Semantics.Macros.deonticActionFor (Semantics.Deed.action "discard") Semantics.NounPhrase.this) none none))) (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.move Semantics.NounPhrase.this (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.hand Semantics.ZoneScope.bare)]) (some (Semantics.ZoneExpr.zone Semantics.Zone.exile Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []))) [] none [] none none none (Semantics.Instruction.instruct (Semantics.NounPhrase.possessorOf Semantics.PossessorAxis.owner Semantics.NounPhrase.this) (Semantics.Instruction.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Instruction.establish (Semantics.StaticSpec.may (Semantics.Subject.actor Semantics.NounPhrase.actor) (Semantics.Macros.deonticActionFor (Semantics.Deed.action "cast") Semantics.NounPhrase.this) ({ bound := none, asThough := none, rider := (Semantics.DeonticRider.play (some (Semantics.ZoneExpr.zone Semantics.Zone.exile Semantics.ZoneScope.bare)) none none false (Semantics.PlayPayment.payingInstead (Semantics.Cost.inCaller arg0))), except := none } : Semantics.DeonticMods)) none) none (some (Semantics.Instruction.move Semantics.NounPhrase.this Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none [])))))])

/-- `keyword_abilities/maxSpeed.ron` -/
semantic_definition maxSpeed (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "maxSpeed") [(Semantics.KeywordParam.ability arg0)] [(Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.abilityGrant Semantics.NounPhrase.this arg0) (Semantics.Condition.compareAmt (Semantics.Amount.statOf (Semantics.ProjAxis.playerStat Semantics.PlayerStat.speed) Semantics.NounPhrase.you) Semantics.Comparator.eq (Semantics.Amount.lit 4)) Semantics.CondMarking.asLongAs))])

/-- `keyword_abilities/mayhem.ron` -/
semantic_definition mayhem (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "mayhem") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.may (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Macros.deonticActionFor (Semantics.Deed.action "cast") Semantics.NounPhrase.this) ({ bound := none, asThough := none, rider := (Semantics.DeonticRider.play (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.you))) none none false (Semantics.PlayPayment.payingInstead (Semantics.Cost.inCaller arg0))), except := none } : Semantics.DeonticMods)) (Semantics.Condition.did (Semantics.DidClause.mk (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Instruction.enact (Semantics.Deed.action "discard") (Semantics.Macros.deonticActionFor (Semantics.Deed.action "discard") Semantics.NounPhrase.this) none none) (Semantics.LookbackSpan.duration (Semantics.Duration.between (some (Semantics.GameEvent.beginningOf Semantics.TurnPart.turn none Semantics.TurnRef.current)) (some Semantics.GameEvent.now)) none))) Semantics.CondMarking.asLongAs))])

/-- `keyword_abilities/megamorph.ron` -/
semantic_definition megamorph (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "megamorph") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.may (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Macros.deonticActionFor (Semantics.Deed.action "cast") Semantics.NounPhrase.this) ({ bound := none, asThough := none, rider := (Semantics.DeonticRider.play none none none false (Semantics.PlayPayment.faceDown ({ characteristics := (some ({ name := none, cost := none, colors := [], supertypes := [], types := [Semantics.CardType.creature], subtypes := [], text := [], power := (some (Semantics.Amount.lit 2)), toughness := (some (Semantics.Amount.lit 2)), loyalty := none, defense := none } : Semantics.Characteristics)) } : Semantics.FaceDown) (Semantics.Cost.mana [(Semantics.ManaSymbol.simple (Semantics.SimpleManaSymbol.generic 3))]))), except := none } : Semantics.DeonticMods))), (Semantics.Ability.static (Semantics.StaticSpec.specialAction none (Semantics.Cost.inCaller arg0) (Semantics.Instruction.sequentially [(Semantics.Instruction.setStatus Semantics.Status.faceUp Semantics.NounPhrase.this), (Semantics.Instruction.putCounters (Semantics.Amount.lit 1) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) Semantics.NounPhrase.this Semantics.CounterBatch.one true)])))])

/-- `keyword_abilities/melee.ron` -/
semantic_definition melee : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "melee") [] [(Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) (Semantics.Instruction.attack none)) [] none [] none none none (Semantics.Instruction.establish (Semantics.StaticSpec.ptModification (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none) (Semantics.Delta.up (Semantics.Amount.arith Semantics.ArithOp.times (Semantics.Amount.lit 1) (Semantics.Amount.tally (Semantics.DidClause.mk (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Instruction.attackWith (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.hasType Semantics.CardType.creature)) (some Semantics.NounPhrase.which)) (Semantics.LookbackSpan.duration (Semantics.Duration.between (some (Semantics.GameEvent.beginningOf Semantics.TurnPart.combat none Semantics.TurnRef.current)) (some Semantics.GameEvent.now)) none)) Semantics.TallyMode.events))) (Semantics.Delta.up (Semantics.Amount.arith Semantics.ArithOp.times (Semantics.Amount.lit 1) (Semantics.Amount.tally (Semantics.DidClause.mk (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Instruction.attackWith (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.hasType Semantics.CardType.creature)) (some Semantics.NounPhrase.which)) (Semantics.LookbackSpan.duration (Semantics.Duration.between (some (Semantics.GameEvent.beginningOf Semantics.TurnPart.combat none Semantics.TurnRef.current)) (some Semantics.GameEvent.now)) none)) Semantics.TallyMode.events)))) (some (Semantics.Duration.between none (some (Semantics.GameEvent.endOf Semantics.TurnPart.turn none Semantics.TurnRef.current))))))])

/-- `keyword_abilities/menace.ron` -/
semantic_definition menace : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "menace") [] [(Semantics.Ability.static (Semantics.StaticSpec.cant (Semantics.Subject.by (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.hasType Semantics.CardType.creature))) (Semantics.Macros.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.block) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) ({ bound := (some (Semantics.CountBound.moreThan (Semantics.Amount.lit 1))), asThough := none, rider := Semantics.DeonticRider.noRider, except := none } : Semantics.DeonticMods)))])

/-- `keyword_abilities/mentor.ron` -/
semantic_definition mentor : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "mentor") [] [(Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) (Semantics.Instruction.attack none)) [] none [] none none none (Semantics.Instruction.putCounters (Semantics.Amount.lit 1) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) (Semantics.NounPhrase.described (Semantics.DetPhrase.target (Semantics.Quantity.range (some 1) (some 1))) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.inCombat Semantics.CombatRelation.attackerOf none), (Semantics.Predicate.compare [(Semantics.ProjAxis.stat Semantics.Stat.power)] Semantics.Comparator.less (Semantics.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.power) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)))])) Semantics.CounterBatch.one true))])

/-- `keyword_abilities/miracle.ron` -/
semantic_definition miracle (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "miracle") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [])

/-- `keyword_abilities/mobilize.ron` -/
semantic_definition mobilize (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "mobilize") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0))] [(Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) (Semantics.Instruction.attack none)) [] none [] none none none (Semantics.Instruction.sequentially [(Semantics.Instruction.createObject (Semantics.Amount.inCaller arg0) (Semantics.CreationSpec.token (Semantics.TokenSpec.written ({ characteristics := ({ name := none, cost := none, colors := [Semantics.Color.red], supertypes := [], types := [Semantics.CardType.creature], subtypes := [(Semantics.Subtype.of Semantics.CardType.creature "Warrior")], text := [], power := (some (Semantics.Amount.lit 1)), toughness := (some (Semantics.Amount.lit 1)), loyalty := none, defense := none } : Semantics.Characteristics), qualities := [] } : Semantics.CharacteristicBundle)) [(Semantics.EntryRider.entersAs Semantics.Status.tapped), (Semantics.EntryRider.entersAttacking none)]) true none), (Semantics.Instruction.delay (Semantics.GameEvent.beginningOf Semantics.TurnPart.endStep none (Semantics.TurnRef.next none)) [] none (Semantics.Instruction.ownScope (some (Semantics.Deed.action "sacrifice")) (Semantics.Instruction.enact (Semantics.Deed.action "sacrifice") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.pro Semantics.Reach.tokenBorn Semantics.Plurality.many)) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.departure none []) none none)))]))])

/-- `keyword_abilities/modular.ron` -/
semantic_definition modular (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "modular") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.entryRider (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.EntryRider.withCounters (Semantics.Amount.inCaller arg0) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) Semantics.EntryCounterMark.fresh))), (Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.departure none [])) [] none [] none none none (Semantics.Instruction.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Instruction.putCounters (Semantics.Amount.statOf (Semantics.ProjAxis.counter (Semantics.CounterKind.named "p1p1Counter")) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) (Semantics.NounPhrase.described (Semantics.DetPhrase.target (Semantics.Quantity.range (some 1) (some 1))) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.artifact), (Semantics.Predicate.hasType Semantics.CardType.creature)])) Semantics.CounterBatch.one true) none none))])

/-- `keyword_abilities/moreThanMeetsTheEye.ron` -/
semantic_definition moreThanMeetsTheEye (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "moreThanMeetsTheEye") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [])

/-- `keyword_abilities/morph.ron` -/
semantic_definition morph (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "morph") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.may (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Macros.deonticActionFor (Semantics.Deed.action "cast") Semantics.NounPhrase.this) ({ bound := none, asThough := none, rider := (Semantics.DeonticRider.play none none none false (Semantics.PlayPayment.faceDown ({ characteristics := (some ({ name := none, cost := none, colors := [], supertypes := [], types := [Semantics.CardType.creature], subtypes := [], text := [], power := (some (Semantics.Amount.lit 2)), toughness := (some (Semantics.Amount.lit 2)), loyalty := none, defense := none } : Semantics.Characteristics)) } : Semantics.FaceDown) (Semantics.Cost.mana [(Semantics.ManaSymbol.simple (Semantics.SimpleManaSymbol.generic 3))]))), except := none } : Semantics.DeonticMods))), (Semantics.Ability.static (Semantics.StaticSpec.specialAction none (Semantics.Cost.inCaller arg0) (Semantics.Instruction.setStatus Semantics.Status.faceUp Semantics.NounPhrase.this)))])

/-- `keyword_abilities/multikicker.ron` -/
semantic_definition multikicker (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "multikicker") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [])

/-- `keyword_abilities/mutate.ron` -/
semantic_definition mutate (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "mutate") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.may (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Macros.deonticActionFor (Semantics.Deed.action "cast") Semantics.NounPhrase.this) ({ bound := none, asThough := none, rider := (Semantics.DeonticRider.play none none none false (Semantics.PlayPayment.payingInstead (Semantics.Cost.inCaller arg0))), except := none } : Semantics.DeonticMods))), (Semantics.Ability.static (Semantics.StaticSpec.addsTarget (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.not (Semantics.Predicate.hasSubtype (Semantics.Subtype.of Semantics.CardType.creature "Human"))), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.owner (Semantics.NounPhrase.possessorOf Semantics.PossessorAxis.owner (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)))]))), (Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.instead (Semantics.WouldClause.would Semantics.Subject.any (Semantics.Instruction.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none [])) (Semantics.Instruction.chooseModes (Semantics.Quantity.range (some 1) (some 1)) [(none, none, (Semantics.Instruction.merge (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) (Semantics.NounPhrase.paid (Semantics.LinkedFacet.objects Semantics.LinkSource.theAlternative (Semantics.Deed.core Semantics.CoreDeed.target) Semantics.Plurality.one) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) Semantics.MergePosition.top)), (none, none, (Semantics.Instruction.merge (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) (Semantics.NounPhrase.paid (Semantics.LinkedFacet.objects Semantics.LinkSource.theAlternative (Semantics.Deed.core Semantics.CoreDeed.target) Semantics.Plurality.one) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) Semantics.MergePosition.bottom))]) none Semantics.ReplUse.repeatedly none) (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "mutate")) none) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) Semantics.CondMarking.asLongAs))])

/-- `keyword_abilities/myriad.ron` -/
semantic_definition myriad : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "myriad") [] [(Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) (Semantics.Instruction.attack none)) [] none [] none none none (Semantics.Instruction.sequentially [(Semantics.Instruction.doForEach (Semantics.NounPhrase.described Semantics.DetPhrase.each (Semantics.Predicate.and [Semantics.Predicate.opponent, (Semantics.Predicate.otherThan (Semantics.NounPhrase.combatPlayer Semantics.CombatRole.defending))])) (Semantics.Instruction.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Instruction.createObject (Semantics.Amount.lit 1) (Semantics.CreationSpec.token (Semantics.TokenSpec.copyOf (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none) []) [(Semantics.EntryRider.entersAs Semantics.Status.tapped), (Semantics.EntryRider.entersAttacking (some (Semantics.NounPhrase.or [(Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.player) Semantics.Plurality.one), (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.planeswalker), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.player) Semantics.Plurality.one))]))])))]) true none) none none)), (Semantics.Instruction.doIf (Semantics.Condition.compareAmt (Semantics.Amount.countOf (Semantics.NounPhrase.pro Semantics.Reach.tokenBorn Semantics.Plurality.many)) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) (Semantics.Instruction.delay (Semantics.GameEvent.beginningOf Semantics.TurnPart.endOfCombat none (Semantics.TurnRef.next none)) [] none (Semantics.Instruction.ownScope (some (Semantics.Deed.action "exile")) (Semantics.Instruction.enact (Semantics.Deed.action "exile") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.pro Semantics.Reach.tokenBorn Semantics.Plurality.many)) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.exile Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) none none))) none)]))])

/-- `keyword_abilities/nightbound.ron` -/
semantic_definition nightbound : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "nightbound") [] [(Semantics.Ability.static (Semantics.StaticSpec.asEvent (Semantics.GameEvent.gameBecomes (Semantics.DesignationLabel.named "day")) (Semantics.Instruction.doIf (Semantics.Condition.matches (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.Predicate.currentFace Semantics.CardFaceSide.back)) (Semantics.Instruction.ownScope (some (Semantics.Deed.action "transform")) (Semantics.Instruction.enact (Semantics.Deed.action "transform") (Semantics.Instruction.turnOver (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this))) none none)) none))), (Semantics.Ability.static (Semantics.StaticSpec.cant Semantics.Subject.any (Semantics.Macros.deonticActionFor (Semantics.Deed.action "transform") (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) ({ bound := none, asThough := none, rider := Semantics.DeonticRider.noRider, except := (some (Semantics.Cause.byKeyword (Semantics.KeywordLabel.named "nightbound"))) } : Semantics.DeonticMods)))])

/-- `keyword_abilities/ninjutsu.ron` -/
semantic_definition ninjutsu (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "ninjutsu") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.activated (Semantics.Cost.compound [(Semantics.Cost.inCaller arg0), (Semantics.Cost.perform (Semantics.Instruction.expose Semantics.ExposeVerb.reveal (Semantics.Exposed.cards Semantics.NounPhrase.this))), (Semantics.Cost.perform (Semantics.Instruction.enact (Semantics.Deed.core Semantics.CoreDeed.return_) (Semantics.Instruction.move (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.inCombat Semantics.CombatRelation.attackerOf none), (Semantics.Predicate.not (Semantics.Predicate.inCombat Semantics.CombatRelation.blockedBy none)), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)])) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.hand Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) none none))]) (Semantics.Instruction.move Semantics.NounPhrase.this (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.hand (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.you))]) (some (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none [(Semantics.MoveRider.entry (Semantics.EntryRider.entersAs Semantics.Status.tapped)), (Semantics.MoveRider.entry (Semantics.EntryRider.entersAttacking none))]) none none none none)])

/-- `keyword_abilities/offering.ron` -/
semantic_definition offering (arg0 : Semantics.Predicate) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "offering") [(Semantics.KeywordParam.quality (Semantics.Predicate.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.addedCost (Semantics.Cost.perform (Semantics.Instruction.ownScope (some (Semantics.Deed.action "sacrifice")) (Semantics.Instruction.enact (Semantics.Deed.action "sacrifice") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.inCaller arg0), (Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare))]))) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.departure none []) none none))) true Semantics.PayTimes.once)), (Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.costShift (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) (Semantics.CostShift.bySymbols (Semantics.ManaCostTerm.manaCostOf (Semantics.NounPhrase.paid (Semantics.LinkedFacet.objects Semantics.LinkSource.theAdditional (Semantics.Deed.action "sacrifice") Semantics.Plurality.one) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this))) false false)) (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "offering")) none) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) Semantics.CondMarking.asLongAs)), (Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.may (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Macros.deonticActionFor (Semantics.Deed.action "cast") Semantics.NounPhrase.this) ({ bound := none, asThough := none, rider := (Semantics.DeonticRider.play none none (some (Semantics.Timing.onlyIf (Semantics.Condition.hasPriority Semantics.NounPhrase.you))) false Semantics.PlayPayment.itsOwnCost), except := none } : Semantics.DeonticMods)) (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "offering")) none) Semantics.NounPhrase.this) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) Semantics.CondMarking.asLongAs))])

/-- `keyword_abilities/offspring.ron` -/
semantic_definition offspring (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "offspring") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.addedCost (Semantics.Cost.inCaller arg0) true Semantics.PayTimes.once)), (Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none [])) [] none [] none none (some (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "offspring")) none) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) Semantics.Comparator.atLeast (Semantics.Amount.lit 1))) (Semantics.Instruction.createObject (Semantics.Amount.lit 1) (Semantics.CreationSpec.token (Semantics.TokenSpec.copyOf (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) [(Semantics.CopyExcept.edits [(Semantics.CharacteristicEdit.stat Semantics.QualityOp.sets Semantics.CharacteristicStat.power (Semantics.Amount.lit 1)), (Semantics.CharacteristicEdit.stat Semantics.QualityOp.sets Semantics.CharacteristicStat.toughness (Semantics.Amount.lit 1))])]) []) true none))])

/-- `keyword_abilities/outlast.ron` -/
semantic_definition outlast (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "outlast") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.activated (Semantics.Cost.compound [(Semantics.Cost.inCaller arg0), Semantics.Cost.tapSymbol]) (Semantics.Instruction.putCounters (Semantics.Amount.lit 1) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none) Semantics.CounterBatch.one true) (some (Semantics.Timing.and [(Semantics.Timing.inPart Semantics.TurnPart.mainPhase (some Semantics.NounPhrase.you) Semantics.TurnRef.current), (Semantics.Timing.onlyIf (Semantics.Condition.not (Semantics.Condition.exists_ (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.stack Semantics.ZoneScope.bare))))))])) none none none)])

/-- `keyword_abilities/overload.ron` -/
semantic_definition overload (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "overload") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.may (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Macros.deonticActionFor (Semantics.Deed.action "cast") Semantics.NounPhrase.this) ({ bound := none, asThough := none, rider := (Semantics.DeonticRider.play none none none false (Semantics.PlayPayment.payingInstead (Semantics.Cost.inCaller arg0))), except := none } : Semantics.DeonticMods))), (Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.textChange (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) Semantics.TextChange.targetToEach) (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "overload")) none) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) Semantics.Comparator.atLeast (Semantics.Amount.lit 1)) Semantics.CondMarking.asLongAs))])

/-- `keyword_abilities/paradigm.ron` -/
semantic_definition paradigm : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "paradigm") [] [])

/-- `keyword_abilities/partner.ron` -/
semantic_definition partner : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "partner") [] [])

/-- `keyword_abilities/partnerCharacterSelect.ron` -/
semantic_definition partnerCharacterSelect : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "partnerCharacterSelect") [] [])

/-- `keyword_abilities/partnerFatherAndSon.ron` -/
semantic_definition partnerFatherAndSon : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "partnerFatherAndSon") [] [])

/-- `keyword_abilities/partnerFriendsForever.ron` -/
semantic_definition partnerFriendsForever : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "partnerFriendsForever") [] [])

/-- `keyword_abilities/partnerSurvivors.ron` -/
semantic_definition partnerSurvivors : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "partnerSurvivors") [] [])

/-- `keyword_abilities/partnerWith.ron` -/
semantic_definition partnerWith (arg0 : Semantics.Predicate) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "partnerWith") [(Semantics.KeywordParam.quality (Semantics.Predicate.inCaller arg0))] [])

/-- `keyword_abilities/persist.ron` -/
semantic_definition persist : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "persist") [] [(Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.departure none [])) [] none [] none none (some (Semantics.Condition.not (Semantics.Condition.matches (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.Predicate.hasCounters (some (Semantics.CounterKind.named "m1m1Counter")))))) (Semantics.Instruction.enact (Semantics.Deed.core Semantics.CoreDeed.return_) (Semantics.Instruction.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none [(Semantics.MoveRider.entry (Semantics.EntryRider.under (Semantics.NounPhrase.possessorOf Semantics.PossessorAxis.owner (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)))), (Semantics.MoveRider.entry (Semantics.EntryRider.withCounters (Semantics.Amount.lit 1) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "m1m1Counter")) Semantics.EntryCounterMark.fresh))]) none none))])

/-- `keyword_abilities/phasing.ron` -/
semantic_definition phasing : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "phasing") [] [])

/-- `keyword_abilities/plot.ron` -/
semantic_definition plot (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "plot") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.specialAction (some (Semantics.Timing.and [(Semantics.Timing.inPart Semantics.TurnPart.mainPhase (some Semantics.NounPhrase.you) Semantics.TurnRef.current), (Semantics.Timing.onlyIf (Semantics.Condition.not (Semantics.Condition.exists_ (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.stack Semantics.ZoneScope.bare))))))])) (Semantics.Cost.inCaller arg0) (Semantics.Instruction.sequentially [(Semantics.Instruction.ownScope (some (Semantics.Deed.action "exile")) (Semantics.Instruction.enact (Semantics.Deed.action "exile") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller Semantics.NounPhrase.this) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.exile Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) none none)), (Semantics.Instruction.establish (Semantics.StaticSpec.may (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Macros.deonticActionFor (Semantics.Deed.action "cast") (Semantics.NounPhrase.pro Semantics.Reach.bare Semantics.Plurality.one)) ({ bound := none, asThough := none, rider := (Semantics.DeonticRider.play (some (Semantics.ZoneExpr.zone Semantics.Zone.exile Semantics.ZoneScope.bare)) none (some (Semantics.Timing.and [(Semantics.Timing.inPart Semantics.TurnPart.mainPhase (some Semantics.NounPhrase.you) Semantics.TurnRef.current), (Semantics.Timing.onlyIf (Semantics.Condition.not (Semantics.Condition.exists_ (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.stack Semantics.ZoneScope.bare))))))])) false Semantics.PlayPayment.withoutPaying), except := none } : Semantics.DeonticMods)) (some (Semantics.Duration.between (some (Semantics.GameEvent.endOf Semantics.TurnPart.turn none Semantics.TurnRef.current)) none)))])))])

/-- `keyword_abilities/poisonous.ron` -/
semantic_definition poisonous (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "poisonous") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0))] [(Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) (Semantics.Instruction.dealDamage Semantics.Amount.anyAmount (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) Semantics.Predicate.anyPlayer) Semantics.DamageKind.combatOnly)) [] none [] none none none (Semantics.Instruction.putCounters (Semantics.Amount.inCaller arg0) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "poison")) (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.player) Semantics.Plurality.one) Semantics.CounterBatch.one true))])

/-- `keyword_abilities/powerUp.ron` -/
semantic_definition powerUp (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "powerUp") [(Semantics.KeywordParam.ability arg0)] [(Semantics.Ability.qualified arg0 none (some (Semantics.UsageLimit.mk (Semantics.CountBound.moreThan (Semantics.Amount.lit 1)) Semantics.Period.thisObject)) none none), (Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.costShift (Semantics.NounPhrase.asMarker Semantics.MarkerWord.ability Semantics.NounPhrase.this) (Semantics.CostShift.bySymbols (Semantics.ManaCostTerm.manaCostOf (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) false false)) (Semantics.Condition.matches (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.Predicate.did (Semantics.DidClause.mk Semantics.Subject.any (Semantics.Instruction.move Semantics.NounPhrase.which Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) (Semantics.LookbackSpan.duration (Semantics.Duration.between (some (Semantics.GameEvent.beginningOf Semantics.TurnPart.turn none Semantics.TurnRef.current)) (some Semantics.GameEvent.now)) none)))) Semantics.CondMarking.asLongAs))])

/-- `keyword_abilities/protection.ron` -/
semantic_definition protection (arg0 : Semantics.Predicate) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "protection") [(Semantics.KeywordParam.quality (Semantics.Predicate.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.cant (Semantics.Subject.by (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.or [(Semantics.Predicate.and [(Semantics.Predicate.and [(Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.stack Semantics.ZoneScope.bare)), (Semantics.Predicate.not (Semantics.Predicate.abilityHead Semantics.AbilityClass.anyOnStack))]), (Semantics.Predicate.inCaller arg0)]), (Semantics.Predicate.abilityOf (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.and [Semantics.Predicate.isSource, (Semantics.Predicate.inCaller arg0)])))]))) (Semantics.Macros.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.target) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) ({ bound := none, asThough := none, rider := Semantics.DeonticRider.noRider, except := none } : Semantics.DeonticMods))), (Semantics.Ability.static (Semantics.StaticSpec.cant Semantics.Subject.any (Semantics.Instruction.attachment Semantics.AttachMove.attached (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.and [(Semantics.Predicate.hasSubtype (Semantics.Subtype.of Semantics.CardType.enchantment "Aura")), (Semantics.Predicate.inCaller arg0)])) (some (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this))) ({ bound := none, asThough := none, rider := Semantics.DeonticRider.noRider, except := none } : Semantics.DeonticMods))), (Semantics.Ability.static (Semantics.StaticSpec.cant Semantics.Subject.any (Semantics.Instruction.attachment Semantics.AttachMove.attached (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.and [(Semantics.Predicate.or [(Semantics.Predicate.hasSubtype (Semantics.Subtype.of Semantics.CardType.artifact "Equipment")), (Semantics.Predicate.hasSubtype (Semantics.Subtype.of Semantics.CardType.artifact "Fortification"))]), (Semantics.Predicate.inCaller arg0)])) (some (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this))) ({ bound := none, asThough := none, rider := Semantics.DeonticRider.noRider, except := none } : Semantics.DeonticMods))), (Semantics.Ability.static (Semantics.StaticSpec.damageRule Semantics.DamageKind.any (Semantics.DamageAgent.dealtBy (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.and [Semantics.Predicate.isSource, (Semantics.Predicate.inCaller arg0)]))) (Semantics.DamageScope.toRecipient (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) (Semantics.DamageOp.prevent Semantics.PreventCut.all none) Semantics.ReplUse.repeatedly)), (Semantics.Ability.static (Semantics.StaticSpec.cant (Semantics.Subject.by (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.inCaller arg0)]))) (Semantics.Macros.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.block) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) ({ bound := none, asThough := none, rider := Semantics.DeonticRider.noRider, except := none } : Semantics.DeonticMods)))])

/-- `keyword_abilities/prototype.ron` -/
semantic_definition prototype (arg0 : Semantics.Cost) (arg1 : Semantics.Amount) (arg2 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "prototype") [] [])

/-- `keyword_abilities/provoke.ron` -/
semantic_definition provoke : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "provoke") [] [(Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) (Semantics.Instruction.attack none)) [] none [] none none none (Semantics.Instruction.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Instruction.establish (Semantics.StaticSpec.must (Semantics.Subject.actor (Semantics.NounPhrase.described (Semantics.DetPhrase.target (Semantics.Quantity.range (some 1) (some 1))) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller (Semantics.NounPhrase.combatPlayer Semantics.CombatRole.defending))]))) (Semantics.Macros.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.block) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) ({ bound := none, asThough := none, rider := Semantics.DeonticRider.noRider, except := none } : Semantics.DeonticMods)) (some (Semantics.Duration.between none (some (Semantics.GameEvent.endOf Semantics.TurnPart.combat none Semantics.TurnRef.current))))) (some (Semantics.Instruction.ownScope (some (Semantics.Deed.action "untap")) (Semantics.Instruction.enact (Semantics.Deed.action "untap") (Semantics.Instruction.setStatus Semantics.Status.untapped (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.pro (Semantics.Reach.word (Semantics.NounWord.type Semantics.CardType.creature)) Semantics.Plurality.one))) none none))) none))])

/-- `keyword_abilities/prowess.ron` -/
semantic_definition prowess : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "prowess") [] [(Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Instruction.procedure Semantics.Procedure.cast (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.stack Semantics.ZoneScope.bare)), (Semantics.Predicate.not (Semantics.Predicate.hasType Semantics.CardType.creature))])) Semantics.PlayPayment.itsOwnCost none none none)) [] none [] none none none (Semantics.Instruction.establish (Semantics.StaticSpec.ptModification (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none) (Semantics.Delta.up (Semantics.Amount.lit 1)) (Semantics.Delta.up (Semantics.Amount.lit 1))) (some (Semantics.Duration.between none (some (Semantics.GameEvent.endOf Semantics.TurnPart.turn none Semantics.TurnRef.current))))))])

/-- `keyword_abilities/prowl.ron` -/
semantic_definition prowl (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "prowl") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.may (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Macros.deonticActionFor (Semantics.Deed.action "cast") Semantics.NounPhrase.this) ({ bound := none, asThough := none, rider := (Semantics.DeonticRider.play none none none false (Semantics.PlayPayment.payingInstead (Semantics.Cost.inCaller arg0))), except := none } : Semantics.DeonticMods)) (Semantics.Condition.did (Semantics.DidClause.mk (Semantics.Subject.actor (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.shares (Semantics.QualitySort.subtype Semantics.CardType.creature) Semantics.Agreement.share (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)]))) (Semantics.Instruction.dealDamage Semantics.Amount.anyAmount Semantics.NounPhrase.which Semantics.DamageKind.combatOnly) (Semantics.LookbackSpan.duration (Semantics.Duration.between (some (Semantics.GameEvent.beginningOf Semantics.TurnPart.turn none Semantics.TurnRef.current)) (some Semantics.GameEvent.now)) none))) Semantics.CondMarking.asLongAs))])

/-- `keyword_abilities/rampage.ron` -/
semantic_definition rampage (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "rampage") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0))] [(Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.block (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none))) [] none [] none none none (Semantics.Instruction.establish (Semantics.StaticSpec.ptModification (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none) (Semantics.Delta.up (Semantics.Amount.arith Semantics.ArithOp.times (Semantics.Amount.inCaller arg0) (Semantics.Amount.arith Semantics.ArithOp.minus (Semantics.Amount.countOf (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.inCombat Semantics.CombatRelation.blockerOf (some (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)))]))) (Semantics.Amount.lit 1)))) (Semantics.Delta.up (Semantics.Amount.arith Semantics.ArithOp.times (Semantics.Amount.inCaller arg0) (Semantics.Amount.arith Semantics.ArithOp.minus (Semantics.Amount.countOf (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.inCombat Semantics.CombatRelation.blockerOf (some (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)))]))) (Semantics.Amount.lit 1))))) (some (Semantics.Duration.between none (some (Semantics.GameEvent.endOf Semantics.TurnPart.turn none Semantics.TurnRef.current))))))])

/-- `keyword_abilities/ravenous.ron` -/
semantic_definition ravenous : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "ravenous") [] [(Semantics.Ability.static (Semantics.StaticSpec.entryRider (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.EntryRider.withCounters (Semantics.Amount.letter Semantics.Letter.x) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) Semantics.EntryCounterMark.fresh))), (Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none [])) [] none [] none none (some (Semantics.Condition.compareAmt (Semantics.Amount.letter Semantics.Letter.x) Semantics.Comparator.atLeast (Semantics.Amount.lit 5))) (Semantics.Instruction.draw (Semantics.Amount.lit 1)))])

/-- `keyword_abilities/reach.ron` -/
semantic_definition reach : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "reach") [] [])

/-- `keyword_abilities/readAhead.ron` -/
semantic_definition readAhead : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "readAhead") [] [(Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.cant (Semantics.Subject.actor (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.and [(Semantics.Predicate.and [(Semantics.Predicate.abilityOf (Semantics.NounPhrase.asType Semantics.CardType.enchantment Semantics.NounPhrase.this (some (Semantics.Subtype.of Semantics.CardType.enchantment "Saga")))), (Semantics.Predicate.compare [Semantics.ProjAxis.chapter] Semantics.Comparator.atLeast (Semantics.Amount.lit 1))]), (Semantics.Predicate.not (Semantics.Predicate.compare [Semantics.ProjAxis.chapter] Semantics.Comparator.eq (Semantics.Amount.statOf (Semantics.ProjAxis.counter (Semantics.CounterKind.named "loreCounter")) (Semantics.NounPhrase.asType Semantics.CardType.enchantment Semantics.NounPhrase.this (some (Semantics.Subtype.of Semantics.CardType.enchantment "Saga"))))))]))) (Semantics.Macros.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.trigger) Semantics.NounPhrase.any) ({ bound := none, asThough := none, rider := Semantics.DeonticRider.noRider, except := none } : Semantics.DeonticMods)) (Semantics.Condition.did (Semantics.DidClause.mk Semantics.Subject.any (Semantics.Instruction.move (Semantics.NounPhrase.asType Semantics.CardType.enchantment Semantics.NounPhrase.this (some (Semantics.Subtype.of Semantics.CardType.enchantment "Saga"))) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) (Semantics.LookbackSpan.duration (Semantics.Duration.between (some (Semantics.GameEvent.beginningOf Semantics.TurnPart.turn none Semantics.TurnRef.current)) (some Semantics.GameEvent.now)) none))) Semantics.CondMarking.asLongAs))])

/-- `keyword_abilities/rebound.ron` -/
semantic_definition rebound : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "rebound") [] [(Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.instead (Semantics.WouldClause.would Semantics.Subject.any (Semantics.Instruction.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.stack Semantics.ZoneScope.bare)]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.departure none [])) (Semantics.Instruction.sequentially [(Semantics.Instruction.ownScope (some (Semantics.Deed.action "exile")) (Semantics.Instruction.enact (Semantics.Deed.action "exile") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.exile Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) none none)), (Semantics.Instruction.delay (Semantics.GameEvent.beginningOf Semantics.TurnPart.upkeep (some Semantics.NounPhrase.you) (Semantics.TurnRef.next none)) [] none (Semantics.Instruction.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Instruction.enact (Semantics.Deed.action "cast") (Semantics.Instruction.procedure Semantics.Procedure.cast Semantics.NounPhrase.this Semantics.PlayPayment.withoutPaying none none none) none none) none none))]) none Semantics.ReplUse.repeatedly none) (Semantics.Condition.matches (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) (Semantics.Predicate.castFrom (Semantics.ZoneExpr.zone Semantics.Zone.hand (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.you)))) Semantics.CondMarking.ifSo))])

/-- `keyword_abilities/reconfigure.ron` -/
semantic_definition reconfigure (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "reconfigure") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.activated (Semantics.Cost.inCaller arg0) (Semantics.Instruction.ownScope (some (Semantics.Deed.action "attach")) (Semantics.Instruction.enact (Semantics.Deed.action "attach") (Semantics.Instruction.attachment Semantics.AttachMove.attached (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) (some (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.described (Semantics.DetPhrase.target (Semantics.Quantity.range (some 1) (some 1))) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.otherThan (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)]))))) none none)) (some (Semantics.Timing.and [(Semantics.Timing.inPart Semantics.TurnPart.mainPhase (some Semantics.NounPhrase.you) Semantics.TurnRef.current), (Semantics.Timing.onlyIf (Semantics.Condition.not (Semantics.Condition.exists_ (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.stack Semantics.ZoneScope.bare))))))])) none none none), (Semantics.Ability.activated (Semantics.Cost.inCaller arg0) (Semantics.Instruction.enact (Semantics.Deed.action "attach") (Semantics.Instruction.attachment Semantics.AttachMove.unattached (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) none) none none) (some (Semantics.Timing.and [(Semantics.Timing.inPart Semantics.TurnPart.mainPhase (some Semantics.NounPhrase.you) Semantics.TurnRef.current), (Semantics.Timing.onlyIf (Semantics.Condition.not (Semantics.Condition.exists_ (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.stack Semantics.ZoneScope.bare))))))])) none (some (Semantics.Condition.matches (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.Predicate.attachment Semantics.AttachmentSide.attachment none (some (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.hasType Semantics.CardType.creature)))))) none)])

/-- `keyword_abilities/recover.ron` -/
semantic_definition recover (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "recover") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.move (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.hasType Semantics.CardType.creature)) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.you))) Semantics.ObservationPoint.departure none [])) [] none [] none none none (Semantics.Instruction.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Instruction.pay (Semantics.Cost.inCaller arg0) Semantics.PayTimes.once Semantics.PaymentOutcome.paid none none) (some (Semantics.Instruction.enact (Semantics.Deed.core Semantics.CoreDeed.return_) (Semantics.Instruction.move Semantics.NounPhrase.this (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.graveyard (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.you))]) (some (Semantics.ZoneExpr.zone Semantics.Zone.hand Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) none none)) (some (Semantics.Instruction.ownScope (some (Semantics.Deed.action "exile")) (Semantics.Instruction.enact (Semantics.Deed.action "exile") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller Semantics.NounPhrase.this) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.exile Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) none none)))))])

/-- `keyword_abilities/reinforce.ron` -/
semantic_definition reinforce (arg0 : Semantics.Amount) (arg1 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "reinforce") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0)), (Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg1))] [(Semantics.Ability.activated (Semantics.Cost.compound [(Semantics.Cost.inCaller arg1), (Semantics.Cost.perform (Semantics.Instruction.ownScope (some (Semantics.Deed.action "discard")) (Semantics.Instruction.enact (Semantics.Deed.action "discard") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller Semantics.NounPhrase.this) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.hand (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.actor))]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) none none)))]) (Semantics.Instruction.putCounters (Semantics.Amount.inCaller arg0) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) (Semantics.NounPhrase.described (Semantics.DetPhrase.target (Semantics.Quantity.range (some 1) (some 1))) (Semantics.Predicate.hasType Semantics.CardType.creature)) Semantics.CounterBatch.one true) none none none none)])

/-- `keyword_abilities/renown.ron` -/
semantic_definition renown (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "renown") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0))] [(Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) (Semantics.Instruction.dealDamage Semantics.Amount.anyAmount (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) Semantics.Predicate.anyPlayer) Semantics.DamageKind.combatOnly)) [] none [] none none (some (Semantics.Condition.not (Semantics.Condition.matches (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none) (Semantics.Predicate.hasDesignation (Semantics.DesignationLabel.named "renowned") none)))) (Semantics.Instruction.sequentially [(Semantics.Instruction.putCounters (Semantics.Amount.inCaller arg0) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none) Semantics.CounterBatch.one true), (Semantics.Instruction.gainDesignation (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none) (Semantics.DesignationLabel.named "renowned") none)]))])

/-- `keyword_abilities/replicate.ron` -/
semantic_definition replicate (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "replicate") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.addedCost (Semantics.Cost.inCaller arg0) true Semantics.PayTimes.anyNumberOfTimes)), (Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Instruction.procedure Semantics.Procedure.cast (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) Semantics.PlayPayment.itsOwnCost none none none)) [] none [] none none (some (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "replicate")) none) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) Semantics.Comparator.atLeast (Semantics.Amount.lit 1))) (Semantics.Instruction.sequentially [(Semantics.Instruction.copy Semantics.CopySort.fromStack (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) (Semantics.Amount.paid (Semantics.LinkedFacet.timesPaid (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "replicate"))) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) []), (Semantics.Instruction.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Instruction.chooseNewTargets (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.copy) Semantics.Plurality.many)) none none)]))])

/-- `keyword_abilities/retrace.ron` -/
semantic_definition retrace : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "retrace") [] [(Semantics.Ability.static (Semantics.StaticSpec.may (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Macros.deonticActionFor (Semantics.Deed.action "cast") Semantics.NounPhrase.this) ({ bound := none, asThough := none, rider := (Semantics.DeonticRider.play (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.you))) none none false (Semantics.PlayPayment.payingInstead (Semantics.Cost.compound [Semantics.Cost.itsManaCost, (Semantics.Cost.perform (Semantics.Instruction.ownScope (some (Semantics.Deed.action "discard")) (Semantics.Instruction.enact (Semantics.Deed.action "discard") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [Semantics.Predicate.isCard, (Semantics.Predicate.hasType Semantics.CardType.land), (Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.hand (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.you)))]))) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.hand (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.actor))]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) none none)))]))), except := none } : Semantics.DeonticMods)))])

/-- `keyword_abilities/riot.ron` -/
semantic_definition riot : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "riot") [] [(Semantics.Ability.static (Semantics.StaticSpec.asEvent (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none [])) (Semantics.Instruction.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Instruction.establish (Semantics.StaticSpec.entryRider (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.EntryRider.withCounters (Semantics.Amount.lit 1) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) Semantics.EntryCounterMark.additional)) none) none (some (Semantics.Instruction.establish (Semantics.StaticSpec.abilityGrant (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.Ability.keyword (Semantics.KeywordLabel.named "haste") [] [])) none)))))])

/-- `keyword_abilities/ripple.ron` -/
semantic_definition ripple (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Macros.ripple arg0)

/-- `keyword_abilities/saddle.ron` -/
semantic_definition saddle (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "saddle") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0))] [(Semantics.Ability.activated (Semantics.Cost.perform (Semantics.Instruction.ownScope (some (Semantics.Deed.action "tap")) (Semantics.Instruction.enact (Semantics.Deed.action "tap") (Semantics.Instruction.setStatus Semantics.Status.tapped (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.someOf (Semantics.SliceCount.totalling (Semantics.ProjAxis.stat Semantics.Stat.power) Semantics.Comparator.atLeast (Semantics.Amount.inCaller arg0)) none (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.and [(Semantics.Predicate.hasStatus Semantics.Status.untapped), (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you), (Semantics.Predicate.otherThan (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this))])]))))) none none))) (Semantics.Instruction.gainDesignation (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.DesignationLabel.named "saddled") (some (Semantics.Duration.between none (some (Semantics.GameEvent.endOf Semantics.TurnPart.turn none Semantics.TurnRef.current))))) (some (Semantics.Timing.and [(Semantics.Timing.inPart Semantics.TurnPart.mainPhase (some Semantics.NounPhrase.you) Semantics.TurnRef.current), (Semantics.Timing.onlyIf (Semantics.Condition.not (Semantics.Condition.exists_ (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.stack Semantics.ZoneScope.bare))))))])) none none none)])

/-- `keyword_abilities/scavenge.ron` -/
semantic_definition scavenge (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "scavenge") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.activated (Semantics.Cost.compound [(Semantics.Cost.inCaller arg0), (Semantics.Cost.perform (Semantics.Instruction.enact (Semantics.Deed.action "exile") (Semantics.Instruction.move Semantics.NounPhrase.this (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.graveyard (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.you))]) (some (Semantics.ZoneExpr.zone Semantics.Zone.exile Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) none none))]) (Semantics.Instruction.putCounters (Semantics.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.power) (Semantics.NounPhrase.pro (Semantics.Reach.verbed (Semantics.Deed.action "exile") (some Semantics.NounWord.card) Semantics.VerbedMarking.attributive) Semantics.Plurality.one)) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) (Semantics.NounPhrase.described (Semantics.DetPhrase.target (Semantics.Quantity.range (some 1) (some 1))) (Semantics.Predicate.hasType Semantics.CardType.creature)) Semantics.CounterBatch.one true) (some (Semantics.Timing.and [(Semantics.Timing.inPart Semantics.TurnPart.mainPhase (some Semantics.NounPhrase.you) Semantics.TurnRef.current), (Semantics.Timing.onlyIf (Semantics.Condition.not (Semantics.Condition.exists_ (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.stack Semantics.ZoneScope.bare))))))])) none none none)])

/-- `keyword_abilities/shadow.ron` -/
semantic_definition shadow : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "shadow") [] [(Semantics.Ability.static (Semantics.StaticSpec.cant (Semantics.Subject.by (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.not (Semantics.Predicate.hasKeyword (Semantics.KeywordTerm.the (Semantics.KeywordLabel.named "shadow"))))]))) (Semantics.Macros.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.block) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) ({ bound := none, asThough := none, rider := Semantics.DeonticRider.noRider, except := none } : Semantics.DeonticMods))), (Semantics.Ability.static (Semantics.StaticSpec.cant (Semantics.Subject.by (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasKeyword (Semantics.KeywordTerm.the (Semantics.KeywordLabel.named "shadow")))]))) (Semantics.Macros.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.block) (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.not (Semantics.Predicate.hasKeyword (Semantics.KeywordTerm.the (Semantics.KeywordLabel.named "shadow"))))]))) ({ bound := none, asThough := none, rider := Semantics.DeonticRider.noRider, except := none } : Semantics.DeonticMods)))])

/-- `keyword_abilities/shroud.ron` -/
semantic_definition shroud : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "shroud") [] [(Semantics.Ability.static (Semantics.StaticSpec.cant Semantics.Subject.any (Semantics.Macros.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.target) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) ({ bound := none, asThough := none, rider := Semantics.DeonticRider.noRider, except := none } : Semantics.DeonticMods)))])

/-- `keyword_abilities/skulk.ron` -/
semantic_definition skulk : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "skulk") [] [(Semantics.Ability.static (Semantics.StaticSpec.cant (Semantics.Subject.by (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.compare [(Semantics.ProjAxis.stat Semantics.Stat.power)] Semantics.Comparator.greater (Semantics.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.power) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)))]))) (Semantics.Macros.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.block) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) ({ bound := none, asThough := none, rider := Semantics.DeonticRider.noRider, except := none } : Semantics.DeonticMods)))])

/-- `keyword_abilities/sneak.ron` -/
semantic_definition sneak (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "sneak") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.may (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Macros.deonticActionFor (Semantics.Deed.action "cast") Semantics.NounPhrase.this) ({ bound := none, asThough := none, rider := (Semantics.DeonticRider.play none none (some (Semantics.Timing.and [(Semantics.Timing.onlyIf (Semantics.Condition.hasPriority Semantics.NounPhrase.you)), (Semantics.Timing.inPart Semantics.TurnPart.declareBlockers (some Semantics.NounPhrase.you) Semantics.TurnRef.current)])) false (Semantics.PlayPayment.payingInstead (Semantics.Cost.compound [(Semantics.Cost.inCaller arg0), (Semantics.Cost.perform (Semantics.Instruction.enact (Semantics.Deed.core Semantics.CoreDeed.return_) (Semantics.Instruction.move (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.not (Semantics.Predicate.inCombat Semantics.CombatRelation.blockedBy none)), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)])) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.hand Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) none none))]))), except := none } : Semantics.DeonticMods)))])

/-- `keyword_abilities/solved.ron` -/
semantic_definition solved (arg0 : Semantics.Ability) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "solved") [(Semantics.KeywordParam.ability arg0)] [(Semantics.Ability.qualified arg0 none none (some (Semantics.Condition.matches (Semantics.NounPhrase.asType Semantics.CardType.enchantment Semantics.NounPhrase.this (some (Semantics.Subtype.of Semantics.CardType.enchantment "Case"))) (Semantics.Predicate.hasDesignation (Semantics.DesignationLabel.named "solved") none))) none)])

/-- `keyword_abilities/soulbond.ron` -/
semantic_definition soulbond : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "soulbond") [] [(Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.move (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none [])) [] none [] none none (some (Semantics.Condition.and [(Semantics.Condition.matches (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none) (Semantics.Predicate.and [(Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you), (Semantics.Predicate.not (Semantics.Predicate.related (Semantics.RelationLabel.named "paired") none))])), (Semantics.Condition.exists_ (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.and [(Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you), (Semantics.Predicate.otherThan (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none))]), (Semantics.Predicate.not (Semantics.Predicate.related (Semantics.RelationLabel.named "paired") none))])))])) (Semantics.Instruction.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Instruction.relate (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none) (Semantics.RelationLabel.named "paired") (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you), (Semantics.Predicate.otherThan (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none))]), (Semantics.Predicate.not (Semantics.Predicate.related (Semantics.RelationLabel.named "paired") none))]))) none none)), (Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.move (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you), (Semantics.Predicate.otherThan (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none))])) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none [])) [] none [] none none (some (Semantics.Condition.and [(Semantics.Condition.matches (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none) (Semantics.Predicate.and [(Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you), (Semantics.Predicate.not (Semantics.Predicate.related (Semantics.RelationLabel.named "paired") none))])), (Semantics.Condition.matches (Semantics.NounPhrase.pro (Semantics.Reach.word (Semantics.NounWord.type Semantics.CardType.creature)) Semantics.Plurality.one) (Semantics.Predicate.and [(Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you), (Semantics.Predicate.not (Semantics.Predicate.related (Semantics.RelationLabel.named "paired") none))]))])) (Semantics.Instruction.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Instruction.relate (Semantics.NounPhrase.pro (Semantics.Reach.word (Semantics.NounWord.type Semantics.CardType.creature)) Semantics.Plurality.one) (Semantics.RelationLabel.named "paired") (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) none none))])

/-- `keyword_abilities/soulshift.ron` -/
semantic_definition soulshift (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "soulshift") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0))] [(Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.departure none [])) [] none [] none none none (Semantics.Instruction.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Instruction.enact (Semantics.Deed.core Semantics.CoreDeed.return_) (Semantics.Instruction.move (Semantics.NounPhrase.described (Semantics.DetPhrase.target (Semantics.Quantity.range (some 1) (some 1))) (Semantics.Predicate.and [Semantics.Predicate.isCard, (Semantics.Predicate.hasSubtype (Semantics.Subtype.of Semantics.CardType.creature "Spirit")), (Semantics.Predicate.compare [(Semantics.ProjAxis.stat Semantics.Stat.manaValue)] Semantics.Comparator.atMost (Semantics.Amount.inCaller arg0)), (Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.graveyard (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.you)))])) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.graveyard (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.you))]) (some (Semantics.ZoneExpr.zone Semantics.Zone.hand Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) none none) none none))])

/-- `keyword_abilities/spectacle.ron` -/
semantic_definition spectacle (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "spectacle") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.may (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Macros.deonticActionFor (Semantics.Deed.action "cast") Semantics.NounPhrase.this) ({ bound := none, asThough := none, rider := (Semantics.DeonticRider.play none none none false (Semantics.PlayPayment.payingInstead (Semantics.Cost.inCaller arg0))), except := none } : Semantics.DeonticMods)) (Semantics.Condition.did (Semantics.DidClause.mk (Semantics.Subject.actor (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) Semantics.Predicate.opponent)) (Semantics.Instruction.changeLife (Semantics.Delta.down Semantics.Amount.anyAmount)) (Semantics.LookbackSpan.duration (Semantics.Duration.between (some (Semantics.GameEvent.beginningOf Semantics.TurnPart.turn none Semantics.TurnRef.current)) (some Semantics.GameEvent.now)) none))) Semantics.CondMarking.asLongAs))])

/-- `keyword_abilities/splice.ron` -/
semantic_definition splice_ (arg0 : Semantics.Predicate) (arg1 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "splice") [(Semantics.KeywordParam.quality (Semantics.Predicate.inCaller arg0)), (Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg1))] [(Semantics.Ability.static (Semantics.StaticSpec.instead (Semantics.WouldClause.would (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Instruction.procedure Semantics.Procedure.cast (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.and [(Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.stack Semantics.ZoneScope.bare)), (Semantics.Predicate.not (Semantics.Predicate.abilityHead Semantics.AbilityClass.anyOnStack))]), (Semantics.Predicate.inCaller arg0)])) Semantics.PlayPayment.itsOwnCost none none none)) (Semantics.Instruction.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Instruction.expose Semantics.ExposeVerb.reveal (Semantics.Exposed.cards Semantics.NounPhrase.this)) (some (Semantics.Instruction.establish (Semantics.StaticSpec.conjunction none [(Semantics.StaticSpec.addedCost (Semantics.Cost.inCaller arg1) false Semantics.PayTimes.once), (Semantics.StaticSpec.textChange (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.spell) Semantics.Plurality.one) (Semantics.TextChange.appendTextOf Semantics.NounPhrase.this))]) none)) none) none Semantics.ReplUse.repeatedly none))])

/-- `keyword_abilities/splitSecond.ron` -/
semantic_definition splitSecond : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "splitSecond") [] [(Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.conjunction none [(Semantics.StaticSpec.cant (Semantics.Subject.actor (Semantics.NounPhrase.described Semantics.DetPhrase.all Semantics.Predicate.anyPlayer)) (Semantics.Instruction.procedure Semantics.Procedure.cast (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.and [(Semantics.Predicate.and [(Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.stack Semantics.ZoneScope.bare)), (Semantics.Predicate.not (Semantics.Predicate.abilityHead Semantics.AbilityClass.anyOnStack))]), (Semantics.Predicate.otherThan (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this))])) Semantics.PlayPayment.itsOwnCost none none none) ({ bound := none, asThough := none, rider := Semantics.DeonticRider.noRider, except := none } : Semantics.DeonticMods)), (Semantics.StaticSpec.cant (Semantics.Subject.actor (Semantics.NounPhrase.described Semantics.DetPhrase.all Semantics.Predicate.anyPlayer)) (Semantics.Instruction.procedure Semantics.Procedure.activate (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.and [(Semantics.Predicate.abilityHead Semantics.AbilityClass.anyActivated), (Semantics.Predicate.not Semantics.Predicate.isManaAbility)])) Semantics.PlayPayment.itsOwnCost none none none) ({ bound := none, asThough := none, rider := Semantics.DeonticRider.noRider, except := none } : Semantics.DeonticMods))]) (Semantics.Condition.matches (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) (Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.stack Semantics.ZoneScope.bare))) Semantics.CondMarking.asLongAs))])

/-- `keyword_abilities/spree.ron` -/
semantic_definition spree : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "spree") [] [(Semantics.Ability.static (Semantics.StaticSpec.modeChoice (Semantics.ModeChange.setQuantity (Semantics.Quantity.range (some 1) none))))])

/-- `keyword_abilities/squad.ron` -/
semantic_definition squad (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "squad") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.addedCost (Semantics.Cost.inCaller arg0) true Semantics.PayTimes.anyNumberOfTimes)), (Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.move (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none [])) [] none [] none none (some (Semantics.Condition.compareAmt (Semantics.Amount.paid (Semantics.LinkedFacet.readback (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "squad")) none) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) Semantics.Comparator.atLeast (Semantics.Amount.lit 1))) (Semantics.Instruction.createObject (Semantics.Amount.paid (Semantics.LinkedFacet.timesPaid (Semantics.LinkSource.byKeyword (Semantics.KeywordLabel.named "squad"))) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)) (Semantics.CreationSpec.token (Semantics.TokenSpec.copyOf (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none) []) []) true none))])

/-- `keyword_abilities/startYourEngines.ron` -/
semantic_definition startYourEngines : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "startYourEngines") [] [])

/-- `keyword_abilities/station.ron` -/
semantic_definition station : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "station") [] [(Semantics.Ability.activated (Semantics.Cost.perform (Semantics.Instruction.enact (Semantics.Deed.action "tap") (Semantics.Instruction.setStatus Semantics.Status.tapped (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.otherThan (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)), (Semantics.Predicate.hasStatus Semantics.Status.untapped), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)]))) none none)) (Semantics.Instruction.putCounters (Semantics.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.power) (Semantics.NounPhrase.pro (Semantics.Reach.verbed (Semantics.Deed.action "tap") (some (Semantics.NounWord.type Semantics.CardType.creature)) Semantics.VerbedMarking.attributive) Semantics.Plurality.one)) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "chargeCounter")) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.CounterBatch.one true) (some (Semantics.Timing.and [(Semantics.Timing.inPart Semantics.TurnPart.mainPhase (some Semantics.NounPhrase.you) Semantics.TurnRef.current), (Semantics.Timing.onlyIf (Semantics.Condition.not (Semantics.Condition.exists_ (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.stack Semantics.ZoneScope.bare))))))])) none none none)])

/-- `keyword_abilities/storied.ron` -/
semantic_definition storied : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "storied") [] [(Semantics.Ability.static (Semantics.StaticSpec.whileHolds (Semantics.Condition.and [(Semantics.Condition.compareAmt (Semantics.Amount.countOf (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.and [(Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you), (Semantics.Predicate.or [(Semantics.Predicate.hasType Semantics.CardType.artifact), (Semantics.Predicate.hasSubtype (Semantics.Subtype.of Semantics.CardType.enchantment "Saga")), (Semantics.Predicate.hasSupertype Semantics.Supertype.legendary)])]))) Semantics.Comparator.atLeast (Semantics.Amount.lit 3)), (Semantics.Condition.not (Semantics.Condition.matches Semantics.NounPhrase.you (Semantics.Predicate.hasDesignation (Semantics.DesignationLabel.named "enduringStory") none)))]) (Semantics.Instruction.gainDesignation Semantics.NounPhrase.you (Semantics.DesignationLabel.named "enduringStory") (some (Semantics.Duration.between none none)))))])

/-- `keyword_abilities/storm.ron` -/
semantic_definition storm : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "storm") [] [(Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Instruction.procedure Semantics.Procedure.cast (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) Semantics.PlayPayment.itsOwnCost none none none)) [] none [] none none none (Semantics.Instruction.sequentially [(Semantics.Instruction.copy Semantics.CopySort.fromStack (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) (Semantics.Amount.tally (Semantics.DidClause.mk (Semantics.Subject.actor (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) Semantics.Predicate.anyPlayer)) (Semantics.Instruction.procedure Semantics.Procedure.cast (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.and [(Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.stack Semantics.ZoneScope.bare)), (Semantics.Predicate.not (Semantics.Predicate.abilityHead Semantics.AbilityClass.anyOnStack))]), (Semantics.Predicate.otherThan (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this))])) Semantics.PlayPayment.itsOwnCost none none none) (Semantics.LookbackSpan.duration (Semantics.Duration.between (some (Semantics.GameEvent.beginningOf Semantics.TurnPart.turn none Semantics.TurnRef.current)) (some Semantics.GameEvent.now)) none)) Semantics.TallyMode.events) []), (Semantics.Instruction.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Instruction.chooseNewTargets (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.copy) Semantics.Plurality.many)) none none)]))])

/-- `keyword_abilities/sunburst.ron` -/
semantic_definition sunburst : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "sunburst") [] [(Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.entryRider (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.EntryRider.withCounters (Semantics.Amount.paid Semantics.LinkedFacet.colorsSpent (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) Semantics.EntryCounterMark.fresh)) (Semantics.Condition.matches (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.Predicate.hasType Semantics.CardType.creature)) Semantics.CondMarking.asLongAs)), (Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.entryRider (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.EntryRider.withCounters (Semantics.Amount.paid Semantics.LinkedFacet.colorsSpent (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "chargeCounter")) Semantics.EntryCounterMark.fresh)) (Semantics.Condition.not (Semantics.Condition.matches (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.Predicate.hasType Semantics.CardType.creature))) Semantics.CondMarking.asLongAs))])

/-- `keyword_abilities/surge.ron` -/
semantic_definition surge (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "surge") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.may (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Macros.deonticActionFor (Semantics.Deed.action "cast") Semantics.NounPhrase.this) ({ bound := none, asThough := none, rider := (Semantics.DeonticRider.play none none none false (Semantics.PlayPayment.payingInstead (Semantics.Cost.inCaller arg0))), except := none } : Semantics.DeonticMods)) (Semantics.Condition.did (Semantics.DidClause.mk (Semantics.Subject.actor (Semantics.NounPhrase.playerGroup Semantics.PlayerGroupWord.yourTeam)) (Semantics.Instruction.procedure Semantics.Procedure.cast (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.and [(Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.stack Semantics.ZoneScope.bare)), (Semantics.Predicate.not (Semantics.Predicate.abilityHead Semantics.AbilityClass.anyOnStack))]), (Semantics.Predicate.otherThan (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this))])) Semantics.PlayPayment.itsOwnCost none none none) (Semantics.LookbackSpan.duration (Semantics.Duration.between (some (Semantics.GameEvent.beginningOf Semantics.TurnPart.turn none Semantics.TurnRef.current)) (some Semantics.GameEvent.now)) none))) Semantics.CondMarking.asLongAs))])

/-- `keyword_abilities/suspend.ron` -/
semantic_definition suspend (arg0 : Semantics.Amount) (arg1 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "suspend") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0)), (Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg1))] [])

/-- `keyword_abilities/teamwork.ron` -/
semantic_definition teamwork (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "teamwork") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.addedCost (Semantics.Cost.perform (Semantics.Instruction.ownScope (some (Semantics.Deed.action "tap")) (Semantics.Instruction.enact (Semantics.Deed.action "tap") (Semantics.Instruction.setStatus Semantics.Status.tapped (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.someOf (Semantics.SliceCount.totalling (Semantics.ProjAxis.stat Semantics.Stat.power) Semantics.Comparator.atLeast (Semantics.Amount.inCaller arg0)) none (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)]))))) none none))) true Semantics.PayTimes.once))])

/-- `keyword_abilities/tiered.ron` -/
semantic_definition tiered : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "tiered") [] [(Semantics.Ability.static (Semantics.StaticSpec.modeChoice (Semantics.ModeChange.setQuantity (Semantics.Quantity.range (some 1) (some 1)))))])

/-- `keyword_abilities/toxic.ron` -/
semantic_definition toxic (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "toxic") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0))] [])

/-- `keyword_abilities/training.ron` -/
semantic_definition training : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "training") [] [(Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor (Semantics.NounPhrase.and [(Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none), (Semantics.NounPhrase.described (Semantics.DetPhrase.count (Semantics.Quantity.range (some 1) none) none) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.otherThan (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)), (Semantics.Predicate.compare [(Semantics.ProjAxis.stat Semantics.Stat.power)] Semantics.Comparator.greater (Semantics.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.power) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)))]))])) (Semantics.Instruction.attack none)) [] none [] none none none (Semantics.Instruction.putCounters (Semantics.Amount.lit 1) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none) Semantics.CounterBatch.one true))])

/-- `keyword_abilities/trample.ron` -/
semantic_definition trample : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "trample") [] [])

/-- `keyword_abilities/transfigure.ron` -/
semantic_definition transfigure (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "transfigure") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.activated (Semantics.Cost.compound [(Semantics.Cost.inCaller arg0), (Semantics.Cost.perform (Semantics.Instruction.ownScope (some (Semantics.Deed.action "sacrifice")) (Semantics.Instruction.enact (Semantics.Deed.action "sacrifice") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.departure none []) none none)))]) (Semantics.Instruction.sequentially [(Semantics.Instruction.search (Semantics.SearchScope.oneZone (Semantics.ZoneExpr.zone Semantics.Zone.library (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.you))) (Semantics.Quantity.range (some 1) (some 1)) (Semantics.Predicate.and [Semantics.Predicate.isCard, (Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.compare [(Semantics.ProjAxis.stat Semantics.Stat.manaValue)] Semantics.Comparator.eq (Semantics.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.manaValue) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)))])), (Semantics.Instruction.move (Semantics.NounPhrase.pro (Semantics.Reach.verbed (Semantics.Deed.action "search") none Semantics.VerbedMarking.attributive) Semantics.Plurality.one) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []), (Semantics.Instruction.shuffle (Semantics.Pile.library (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.actor)))]) (some (Semantics.Timing.and [(Semantics.Timing.inPart Semantics.TurnPart.mainPhase (some Semantics.NounPhrase.you) Semantics.TurnRef.current), (Semantics.Timing.onlyIf (Semantics.Condition.not (Semantics.Condition.exists_ (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.stack Semantics.ZoneScope.bare))))))])) none none none)])

/-- `keyword_abilities/transmute.ron` -/
semantic_definition transmute (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "transmute") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.activated (Semantics.Cost.compound [(Semantics.Cost.inCaller arg0), (Semantics.Cost.perform (Semantics.Instruction.ownScope (some (Semantics.Deed.action "discard")) (Semantics.Instruction.enact (Semantics.Deed.action "discard") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller Semantics.NounPhrase.this) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.hand (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.actor))]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) none none)))]) (Semantics.Instruction.sequentially [(Semantics.Instruction.search (Semantics.SearchScope.oneZone (Semantics.ZoneExpr.zone Semantics.Zone.library (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.you))) (Semantics.Quantity.range (some 1) (some 1)) (Semantics.Predicate.and [Semantics.Predicate.isCard, (Semantics.Predicate.compare [(Semantics.ProjAxis.stat Semantics.Stat.manaValue)] Semantics.Comparator.eq (Semantics.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.manaValue) (Semantics.NounPhrase.pro (Semantics.Reach.verbed (Semantics.Deed.action "discard") (some Semantics.NounWord.card) Semantics.VerbedMarking.attributive) Semantics.Plurality.one)))])), (Semantics.Instruction.expose Semantics.ExposeVerb.reveal (Semantics.Exposed.cards (Semantics.NounPhrase.pro (Semantics.Reach.verbed (Semantics.Deed.action "search") none Semantics.VerbedMarking.attributive) Semantics.Plurality.one))), (Semantics.Instruction.move (Semantics.NounPhrase.pro (Semantics.Reach.verbed (Semantics.Deed.action "search") none Semantics.VerbedMarking.attributive) Semantics.Plurality.one) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.hand Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []), (Semantics.Instruction.shuffle (Semantics.Pile.library (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.actor)))]) (some (Semantics.Timing.and [(Semantics.Timing.inPart Semantics.TurnPart.mainPhase (some Semantics.NounPhrase.you) Semantics.TurnRef.current), (Semantics.Timing.onlyIf (Semantics.Condition.not (Semantics.Condition.exists_ (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.stack Semantics.ZoneScope.bare))))))])) none none none)])

/-- `keyword_abilities/tribute.ron` -/
semantic_definition tribute (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "tribute") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.asEvent (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.move (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none [])) (Semantics.Instruction.sequentially [(Semantics.Instruction.choose none (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) Semantics.Predicate.opponent) Semantics.Disclosure.openly none), (Semantics.Instruction.instruct (Semantics.NounPhrase.described Semantics.DetPhrase.the (Semantics.Predicate.chosenPlayer Semantics.ChoiceRef.theChoice)) (Semantics.Instruction.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Instruction.establish (Semantics.StaticSpec.entryRider (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none) (Semantics.EntryRider.withCounters (Semantics.Amount.inCaller arg0) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) Semantics.EntryCounterMark.additional)) none) none none))])))])

/-- `keyword_abilities/typecycling.ron` -/
semantic_definition typecycling (arg0 : Semantics.Predicate) (arg1 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "typecycling") [(Semantics.KeywordParam.quality (Semantics.Predicate.inCaller arg0)), (Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg1))] [])

/-- `keyword_abilities/umbraArmor.ron` -/
semantic_definition umbraArmor : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "umbraArmor") [] [(Semantics.Ability.static (Semantics.StaticSpec.instead (Semantics.WouldClause.would Semantics.Subject.any (Semantics.Instruction.enact (Semantics.Deed.action "destroy") (Semantics.Macros.deonticActionFor (Semantics.Deed.action "destroy") (Semantics.NounPhrase.attachHost Semantics.AttachWord.enchanted Semantics.NounWord.permanent)) none none)) (Semantics.Instruction.sequentially [(Semantics.Instruction.clearDamage (Semantics.NounPhrase.attachHost Semantics.AttachWord.enchanted Semantics.NounWord.permanent)), (Semantics.Instruction.ownScope (some (Semantics.Deed.action "destroy")) (Semantics.Instruction.enact (Semantics.Deed.action "destroy") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asType Semantics.CardType.enchantment Semantics.NounPhrase.this (some (Semantics.Subtype.of Semantics.CardType.enchantment "Aura")))) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.departure none []) none none))]) none Semantics.ReplUse.repeatedly none))])

/-- `keyword_abilities/undaunted.ron` -/
semantic_definition undaunted : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "undaunted") [] [(Semantics.Ability.static (Semantics.StaticSpec.costShift (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this) (Semantics.CostShift.less (Semantics.Amount.arith Semantics.ArithOp.times (Semantics.Amount.lit 1) (Semantics.Amount.countOf (Semantics.NounPhrase.described Semantics.DetPhrase.bare Semantics.Predicate.opponent))) none)))])

/-- `keyword_abilities/undying.ron` -/
semantic_definition undying : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "undying") [] [(Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.departure none [])) [] none [] none none (some (Semantics.Condition.not (Semantics.Condition.matches (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.Predicate.hasCounters (some (Semantics.CounterKind.named "p1p1Counter")))))) (Semantics.Instruction.enact (Semantics.Deed.core Semantics.CoreDeed.return_) (Semantics.Instruction.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none [(Semantics.MoveRider.entry (Semantics.EntryRider.under (Semantics.NounPhrase.possessorOf Semantics.PossessorAxis.owner (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)))), (Semantics.MoveRider.entry (Semantics.EntryRider.withCounters (Semantics.Amount.lit 1) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) Semantics.EntryCounterMark.fresh))]) none none))])

/-- `keyword_abilities/unearth.ron` -/
semantic_definition unearth (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "unearth") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.activated (Semantics.Cost.inCaller arg0) (Semantics.Instruction.sequentially [(Semantics.Instruction.enact (Semantics.Deed.core Semantics.CoreDeed.return_) (Semantics.Instruction.move Semantics.NounPhrase.this (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.graveyard (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.you))]) (some (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) none none), (Semantics.Instruction.establish (Semantics.StaticSpec.abilityGrant (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.Ability.keyword (Semantics.KeywordLabel.named "haste") [] [])) none), (Semantics.Instruction.delay (Semantics.GameEvent.beginningOf Semantics.TurnPart.endStep none (Semantics.TurnRef.next none)) [] none (Semantics.Instruction.ownScope (some (Semantics.Deed.action "exile")) (Semantics.Instruction.enact (Semantics.Deed.action "exile") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.exile Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) none none))), (Semantics.Instruction.establish (Semantics.StaticSpec.instead (Semantics.WouldClause.would Semantics.Subject.any (Semantics.Instruction.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)]) none Semantics.ObservationPoint.departure none [])) (Semantics.Instruction.ownScope (some (Semantics.Deed.action "exile")) (Semantics.Instruction.enact (Semantics.Deed.action "exile") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.exile Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) none none)) none Semantics.ReplUse.repeatedly none) none)]) (some (Semantics.Timing.and [(Semantics.Timing.inPart Semantics.TurnPart.mainPhase (some Semantics.NounPhrase.you) Semantics.TurnRef.current), (Semantics.Timing.onlyIf (Semantics.Condition.not (Semantics.Condition.exists_ (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.stack Semantics.ZoneScope.bare))))))])) none none none)])

/-- `keyword_abilities/unleash.ron` -/
semantic_definition unleash : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "unleash") [] [(Semantics.Ability.static (Semantics.StaticSpec.asEvent (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.move (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none [])) (Semantics.Instruction.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Instruction.establish (Semantics.StaticSpec.entryRider (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.EntryRider.withCounters (Semantics.Amount.lit 1) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) Semantics.EntryCounterMark.additional)) none) none none))), (Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.cant (Semantics.Subject.actor (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) (Semantics.Macros.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.block) Semantics.NounPhrase.any) ({ bound := none, asThough := none, rider := Semantics.DeonticRider.noRider, except := none } : Semantics.DeonticMods)) (Semantics.Condition.matches (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.Predicate.hasCounters (some (Semantics.CounterKind.named "p1p1Counter")))) Semantics.CondMarking.asLongAs))])

/-- `keyword_abilities/vanishing.ron` -/
semantic_definition vanishing (arg0 : Semantics.Amount) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "vanishing") [(Semantics.KeywordParam.number (Semantics.Amount.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.entryRider (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.EntryRider.withCounters (Semantics.Amount.inCaller arg0) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "timeCounter")) Semantics.EntryCounterMark.fresh))), (Semantics.Ability.triggered (Semantics.GameEvent.beginningOf Semantics.TurnPart.upkeep (some Semantics.NounPhrase.you) (Semantics.TurnRef.each none)) [] none [] none none (some (Semantics.Condition.matches (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.Predicate.hasCounters (some (Semantics.CounterKind.named "timeCounter"))))) (Semantics.Instruction.removeCounters (some (Semantics.Quantity.range (some 1) (some 1))) (some (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "timeCounter"))) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.CounterBatch.one true)), (Semantics.Ability.triggered (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.removeCounters none (some (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "timeCounter"))) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.CounterBatch.emptying false)) [] none [] none none none (Semantics.Instruction.ownScope (some (Semantics.Deed.action "sacrifice")) (Semantics.Instruction.enact (Semantics.Deed.action "sacrifice") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.departure none []) none none)))])

/-- `keyword_abilities/vigilance.ron` -/
semantic_definition vigilance : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "vigilance") [] [])

/-- `keyword_abilities/ward.ron` -/
semantic_definition ward (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "ward") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.by (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.stack Semantics.ZoneScope.bare)), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller (Semantics.NounPhrase.playerGroup Semantics.PlayerGroupWord.yourOpponents))]))) (Semantics.Instruction.target (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this))) [] none [] none none none (Semantics.Instruction.instruct (Semantics.NounPhrase.possessorOf Semantics.PossessorAxis.controller (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.stack) Semantics.Plurality.one)) (Semantics.Instruction.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Instruction.pay (Semantics.Cost.inCaller arg0) Semantics.PayTimes.once Semantics.PaymentOutcome.paid none none) none (some (Semantics.Instruction.instruct Semantics.NounPhrase.you (Semantics.Instruction.ownScope (some (Semantics.Deed.action "counter")) (Semantics.Instruction.enact (Semantics.Deed.action "counter") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.stack) Semantics.Plurality.one)) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.stack Semantics.ZoneScope.bare)]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) none none)))))))])

/-- `keyword_abilities/warp.ron` -/
semantic_definition warp (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "warp") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.may (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Macros.deonticActionFor (Semantics.Deed.action "cast") Semantics.NounPhrase.this) ({ bound := none, asThough := none, rider := (Semantics.DeonticRider.play (some (Semantics.ZoneExpr.zone Semantics.Zone.hand (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.you))) none none false (Semantics.PlayPayment.payingInstead (Semantics.Cost.inCaller arg0))), except := none } : Semantics.DeonticMods))), (Semantics.Ability.static (Semantics.StaticSpec.asEvent (Semantics.GameEvent.does (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Instruction.pay Semantics.Cost.itsManaCost Semantics.PayTimes.once Semantics.PaymentOutcome.paid (some (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)) (some (Semantics.KeywordLabel.named "warp")))) (Semantics.Instruction.delay (Semantics.GameEvent.beginningOf Semantics.TurnPart.endStep none (Semantics.TurnRef.next none)) [] none (Semantics.Instruction.sequentially [(Semantics.Instruction.ownScope (some (Semantics.Deed.action "exile")) (Semantics.Instruction.enact (Semantics.Deed.action "exile") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.becomes (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this))) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.exile Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) none none)), (Semantics.Instruction.establish (Semantics.StaticSpec.may (Semantics.Subject.actor (Semantics.NounPhrase.possessorOf Semantics.PossessorAxis.owner (Semantics.NounPhrase.pro Semantics.Reach.bare Semantics.Plurality.one))) (Semantics.Macros.deonticActionFor (Semantics.Deed.action "cast") (Semantics.NounPhrase.pro Semantics.Reach.bare Semantics.Plurality.one)) ({ bound := none, asThough := none, rider := (Semantics.DeonticRider.play (some (Semantics.ZoneExpr.zone Semantics.Zone.exile Semantics.ZoneScope.bare)) none none false Semantics.PlayPayment.itsOwnCost), except := none } : Semantics.DeonticMods)) (some (Semantics.Duration.between (some (Semantics.GameEvent.endOf Semantics.TurnPart.turn none Semantics.TurnRef.current)) none)))]))))])

/-- `keyword_abilities/webSlinging.ron` -/
semantic_definition webSlinging (arg0 : Semantics.Cost) : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "webSlinging") [(Semantics.KeywordParam.cost (Semantics.Cost.inCaller arg0))] [(Semantics.Ability.static (Semantics.StaticSpec.may (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Macros.deonticActionFor (Semantics.Deed.action "cast") Semantics.NounPhrase.this) ({ bound := none, asThough := none, rider := (Semantics.DeonticRider.play none none none false (Semantics.PlayPayment.payingInstead (Semantics.Cost.compound [(Semantics.Cost.inCaller arg0), (Semantics.Cost.perform (Semantics.Instruction.enact (Semantics.Deed.core Semantics.CoreDeed.return_) (Semantics.Instruction.move (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasStatus Semantics.Status.tapped), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)])) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.hand Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) none none))]))), except := none } : Semantics.DeonticMods)))])

/-- `keyword_abilities/wither.ron` -/
semantic_definition wither : Semantics.Ability :=
  (Semantics.Ability.keyword (Semantics.KeywordLabel.named "wither") [] [])

/-- `keyword_actions/activate.ron` -/
semantic_definition activate (arg0 : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "activate")) (Semantics.Instruction.enact (Semantics.Deed.action "activate") (Semantics.Instruction.procedure Semantics.Procedure.activate (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "activate.arg0" arg0)) Semantics.PlayPayment.itsOwnCost none none none) none none))

/-- `keyword_actions/adapt.ron` -/
semantic_definition adapt (arg0 : Semantics.Amount) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "adapt")) (Semantics.Instruction.enact (Semantics.Deed.action "adapt") (Semantics.Instruction.doIf (Semantics.Condition.not (Semantics.Condition.matches (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.Predicate.hasCounters (some (Semantics.CounterKind.named "p1p1Counter"))))) (Semantics.Instruction.putCounters (Semantics.Amount.inCaller arg0) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.CounterBatch.one true) none) none none))

/-- `keyword_actions/airbend.ron` -/
semantic_definition airbend (arg0 : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Macros.airbend arg0)

/-- `keyword_actions/amass.ron` -/
semantic_definition amass (arg0 : Semantics.Subtype) (arg1 : Semantics.Amount) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "amass")) (Semantics.Instruction.enact (Semantics.Deed.action "amass") (Semantics.Instruction.sequentially [(Semantics.Instruction.doIf (Semantics.Condition.not (Semantics.Condition.exists_ (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.and [(Semantics.Predicate.hasSubtype (Semantics.Subtype.of Semantics.CardType.creature "Army")), (Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.actor)])))) (Semantics.Instruction.createObject (Semantics.Amount.lit 1) (Semantics.CreationSpec.token (Semantics.TokenSpec.written ({ characteristics := ({ name := none, cost := none, colors := [Semantics.Color.black], supertypes := [], types := [Semantics.CardType.creature], subtypes := [arg0, (Semantics.Subtype.of Semantics.CardType.creature "Army")], text := [], power := (some (Semantics.Amount.lit 0)), toughness := (some (Semantics.Amount.lit 0)), loyalty := none, defense := none } : Semantics.Characteristics), qualities := [] } : Semantics.CharacteristicBundle)) []) true none) none), (Semantics.Instruction.choose none (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.hasSubtype (Semantics.Subtype.of Semantics.CardType.creature "Army")), (Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.actor)])) Semantics.Disclosure.openly none), (Semantics.Instruction.putCounters (Semantics.Amount.inCaller arg1) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) (Semantics.NounPhrase.pro (Semantics.Reach.word (Semantics.NounWord.type Semantics.CardType.creature)) Semantics.Plurality.one) Semantics.CounterBatch.one true), (Semantics.Instruction.doIf (Semantics.Condition.not (Semantics.Condition.matches (Semantics.NounPhrase.pro Semantics.Reach.bare Semantics.Plurality.one) (Semantics.Predicate.hasSubtype arg0))) (Semantics.Instruction.establish (Semantics.StaticSpec.characteristicChange (Semantics.NounPhrase.pro Semantics.Reach.bare Semantics.Plurality.one) [(Semantics.CharacteristicEdit.typeLine Semantics.QualityOp.adds ({ supertypes := none, types := none, subtypes := (some [arg0]), retained := none } : Semantics.TypeLineChanges))]) none) none)]) none none))

/-- `keyword_actions/attach.ron` -/
semantic_definition attach (arg0 : Semantics.NounPhrase) (arg1 : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "attach")) (Semantics.Instruction.enact (Semantics.Deed.action "attach") (Semantics.Instruction.attachment Semantics.AttachMove.attached (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "attach.arg0" arg0)) (some (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "attach.arg1" arg1)))) none none))

/-- `keyword_actions/behold.ron` -/
semantic_definition behold (arg0 : Semantics.Predicate) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "behold")) (Semantics.Instruction.enact (Semantics.Deed.action "behold") (Semantics.Instruction.chooseModes (Semantics.Quantity.range (some 1) (some 1)) [(none, none, (Semantics.Instruction.expose Semantics.ExposeVerb.reveal (Semantics.Exposed.cards (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [Semantics.Predicate.isCard, (Semantics.Predicate.inCaller arg0), (Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.hand (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.actor)))]))))), (none, none, (Semantics.Instruction.choose none (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.inCaller arg0), (Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.actor)])) Semantics.Disclosure.openly none))]) none none))

/-- `keyword_actions/blight.ron` -/
semantic_definition blight (arg0 : Semantics.Amount) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "blight")) (Semantics.Instruction.enact (Semantics.Deed.action "blight") (Semantics.Instruction.putCounters (Semantics.Amount.inCaller arg0) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "m1m1Counter")) (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.actor)])) Semantics.CounterBatch.one true) none none))

/-- `keyword_actions/bolster.ron` -/
semantic_definition bolster (arg0 : Semantics.Amount) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "bolster")) (Semantics.Instruction.enact (Semantics.Deed.action "bolster") (Semantics.Instruction.sequentially [(Semantics.Instruction.choose none (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.actor), (Semantics.Predicate.superlative Semantics.AggregateOp.min (Semantics.ProjAxis.stat Semantics.Stat.toughness) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.actor)]))])) Semantics.Disclosure.openly none), (Semantics.Instruction.putCounters (Semantics.Amount.inCaller arg0) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) (Semantics.NounPhrase.pro (Semantics.Reach.word (Semantics.NounWord.type Semantics.CardType.creature)) Semantics.Plurality.one) Semantics.CounterBatch.one true)]) none none))

/-- `keyword_actions/cast.ron` -/
semantic_definition cast (arg0 : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "cast")) (Semantics.Instruction.enact (Semantics.Deed.action "cast") (Semantics.Instruction.procedure Semantics.Procedure.cast (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "cast.arg0" arg0)) Semantics.PlayPayment.itsOwnCost none none none) none none))

/-- `keyword_actions/clash.ron` -/
semantic_definition clash : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "clash")) (Semantics.Instruction.enact (Semantics.Deed.action "clash") (Semantics.Instruction.sequentially [(Semantics.Instruction.expose Semantics.ExposeVerb.reveal (Semantics.Exposed.cards (Semantics.NounPhrase.slice Semantics.LibraryEnd.top (Semantics.Amount.lit 1) (Semantics.Pile.library (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.actor))))), (Semantics.Instruction.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Instruction.move (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.card) Semantics.Plurality.one) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.place Semantics.LibraryEnd.bottom (Semantics.Pile.library Semantics.ZoneScope.bare))) Semantics.ObservationPoint.arrival none []) none none)]) none none))

/-- `keyword_actions/cloak.ron` -/
semantic_definition cloak (arg0 : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "cloak")) (Semantics.Instruction.enact (Semantics.Deed.action "cloak") (Semantics.Instruction.sequentially [(Semantics.Instruction.turnFaceDown (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "cloak.arg0" arg0)) ({ characteristics := (some ({ name := none, cost := none, colors := [], supertypes := [], types := [Semantics.CardType.creature], subtypes := [], text := [(Semantics.Ability.keyword (Semantics.KeywordLabel.named "ward") [(Semantics.KeywordParam.cost (Semantics.Cost.mana [(Semantics.ManaSymbol.simple (Semantics.SimpleManaSymbol.generic 2))]))] [])], power := (some (Semantics.Amount.lit 2)), toughness := (some (Semantics.Amount.lit 2)), loyalty := none, defense := none } : Semantics.Characteristics)) } : Semantics.FaceDown)), (Semantics.Instruction.move (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.card) Semantics.Plurality.one) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none [(Semantics.MoveRider.entry (Semantics.EntryRider.faceDown ({ characteristics := (some ({ name := none, cost := none, colors := [], supertypes := [], types := [Semantics.CardType.creature], subtypes := [], text := [(Semantics.Ability.keyword (Semantics.KeywordLabel.named "ward") [(Semantics.KeywordParam.cost (Semantics.Cost.mana [(Semantics.ManaSymbol.simple (Semantics.SimpleManaSymbol.generic 2))]))] [])], power := (some (Semantics.Amount.lit 2)), toughness := (some (Semantics.Amount.lit 2)), loyalty := none, defense := none } : Semantics.Characteristics)) } : Semantics.FaceDown)))])]) none none))

/-- `keyword_actions/collectEvidence.ron` -/
semantic_definition collectEvidence (arg0 : Semantics.Amount) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "collectEvidence")) (Semantics.Instruction.enact (Semantics.Deed.action "collectEvidence") (Semantics.Instruction.move (Semantics.NounPhrase.someOf (Semantics.SliceCount.totalling (Semantics.ProjAxis.stat Semantics.Stat.manaValue) Semantics.Comparator.atLeast (Semantics.Amount.inCaller arg0)) none (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.and [Semantics.Predicate.isCard, (Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.graveyard (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.actor)))]))) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.exile Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) none none))

/-- `keyword_actions/connive.ron` -/
semantic_definition connive : Semantics.Instruction :=
  (Semantics.Macros.connive (Semantics.Amount.lit 1))

/-- `keyword_actions/conniveN.ron` -/
semantic_definition conniveN (arg0 : Semantics.Amount) : Semantics.Instruction :=
  (Semantics.Macros.connive arg0)

/-- `keyword_actions/convert.ron` -/
semantic_definition convert (arg0 : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "convert")) (Semantics.Instruction.enact (Semantics.Deed.action "convert") (Semantics.Instruction.turnOver (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "convert.arg0" arg0))) none none))

/-- `keyword_actions/counter.ron` -/
semantic_definition counter (arg0 : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "counter")) (Semantics.Instruction.enact (Semantics.Deed.action "counter") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "counter.arg0" arg0)) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.stack Semantics.ZoneScope.bare)]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) none none))

/-- `keyword_actions/create.ron` -/
semantic_definition create (arg0 : Semantics.Amount) (arg1 : Semantics.TokenSpec) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "create")) (Semantics.Instruction.enact (Semantics.Deed.action "create") (Semantics.Instruction.createObject (Semantics.Amount.inCaller arg0) (Semantics.CreationSpec.token (Semantics.TokenSpec.inCaller arg1) []) true none) none none))

/-- `keyword_actions/destroy.ron` -/
semantic_definition destroy (arg0 : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "destroy")) (Semantics.Instruction.enact (Semantics.Deed.action "destroy") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "destroy.arg0" arg0)) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.departure none []) none none))

/-- `keyword_actions/detain.ron` -/
semantic_definition detain (arg0 : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "detain")) (Semantics.Instruction.enact (Semantics.Deed.action "detain") (Semantics.Instruction.establish (Semantics.StaticSpec.conjunction none [(Semantics.StaticSpec.conjunction none [(Semantics.StaticSpec.cant (Semantics.Subject.actor (Semantics.NounPhrase.firstMention "detain.0" (Semantics.NounPhrase.inCaller arg0))) (Semantics.Macros.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.attack) Semantics.NounPhrase.any) ({ bound := none, asThough := none, rider := Semantics.DeonticRider.noRider, except := none } : Semantics.DeonticMods)), (Semantics.StaticSpec.cant (Semantics.Subject.actor (Semantics.NounPhrase.laterMention "detain.0" (Semantics.NounPhrase.inCaller arg0))) (Semantics.Macros.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.block) Semantics.NounPhrase.any) ({ bound := none, asThough := none, rider := Semantics.DeonticRider.noRider, except := none } : Semantics.DeonticMods))]), (Semantics.StaticSpec.cant Semantics.Subject.any (Semantics.Macros.deonticActionFor (Semantics.Deed.action "activate") (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.and [(Semantics.Predicate.abilityHead Semantics.AbilityClass.anyActivated), (Semantics.Predicate.abilityOf (Semantics.NounPhrase.laterMention "detain.0" (Semantics.NounPhrase.inCaller arg0)))]))) ({ bound := none, asThough := none, rider := Semantics.DeonticRider.noRider, except := none } : Semantics.DeonticMods))]) (some (Semantics.Duration.between none (some (Semantics.GameEvent.beginningOf Semantics.TurnPart.turn (some (Semantics.NounPhrase.possessorOf Semantics.PossessorAxis.controller Semantics.NounPhrase.this)) (Semantics.TurnRef.next none)))))) none none))

/-- `keyword_actions/discard.ron` -/
semantic_definition discard (arg0 : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "discard")) (Semantics.Instruction.enact (Semantics.Deed.action "discard") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "discard.arg0" arg0)) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.hand (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.actor))]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) none none))

/-- `keyword_actions/discover.ron` -/
semantic_definition discover (arg0 : Semantics.Amount) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "discover")) (Semantics.Instruction.enact (Semantics.Deed.action "discover") (Semantics.Instruction.sequentially [(Semantics.Instruction.move (Semantics.NounPhrase.run Semantics.LibraryEnd.top (Semantics.Predicate.and [(Semantics.Predicate.not (Semantics.Predicate.hasType Semantics.CardType.land)), (Semantics.Predicate.compare [(Semantics.ProjAxis.stat Semantics.Stat.manaValue)] Semantics.Comparator.atMost (Semantics.Amount.inCaller arg0))]) (Semantics.Pile.library (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.actor))) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.exile Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []), (Semantics.Instruction.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Instruction.enact (Semantics.Deed.action "cast") (Semantics.Instruction.procedure Semantics.Procedure.cast (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.card) Semantics.Plurality.one) Semantics.PlayPayment.withoutPaying (some (Semantics.Predicate.compare [(Semantics.ProjAxis.stat Semantics.Stat.manaValue)] Semantics.Comparator.atMost (Semantics.Amount.inCaller arg0))) none none) none none) none (some (Semantics.Instruction.move (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.card) Semantics.Plurality.one) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.hand Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []))), (Semantics.Instruction.move (Semantics.NounPhrase.theRest Semantics.Kind.object Semantics.Plurality.many) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.place Semantics.LibraryEnd.bottom (Semantics.Pile.library Semantics.ZoneScope.bare))) Semantics.ObservationPoint.arrival none [(Semantics.MoveRider.placement (Semantics.PilePlacement.inOrder Semantics.Arrangement.randomOrder))])]) none none))

/-- `keyword_actions/earthbend.ron` -/
semantic_definition earthbend (arg0 : Semantics.Amount) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "earthbend")) (Semantics.Instruction.enact (Semantics.Deed.action "earthbend") (Semantics.Instruction.sequentially [(Semantics.Instruction.establish (Semantics.StaticSpec.characteristicChange (Semantics.NounPhrase.described (Semantics.DetPhrase.target (Semantics.Quantity.range (some 1) (some 1))) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.land), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.actor)])) [(Semantics.CharacteristicEdit.typeLine Semantics.QualityOp.adds ({ supertypes := none, types := (some [Semantics.CardType.creature]), subtypes := none, retained := none } : Semantics.TypeLineChanges)), (Semantics.CharacteristicEdit.stat Semantics.QualityOp.sets Semantics.CharacteristicStat.power (Semantics.Amount.lit 0)), (Semantics.CharacteristicEdit.stat Semantics.QualityOp.sets Semantics.CharacteristicStat.toughness (Semantics.Amount.lit 0)), (Semantics.CharacteristicEdit.addedAbilities [(Semantics.Ability.keyword (Semantics.KeywordLabel.named "haste") [] [])])]) none), (Semantics.Instruction.putCounters (Semantics.Amount.inCaller arg0) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) (Semantics.NounPhrase.pro (Semantics.Reach.word (Semantics.NounWord.type Semantics.CardType.land)) Semantics.Plurality.one) Semantics.CounterBatch.one true), (Semantics.Instruction.delay (Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.move (Semantics.NounPhrase.pro (Semantics.Reach.word (Semantics.NounWord.type Semantics.CardType.land)) Semantics.Plurality.one) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.departure none [])) [(Semantics.GameEvent.does Semantics.Subject.any (Semantics.Instruction.move (Semantics.NounPhrase.pro (Semantics.Reach.word (Semantics.NounWord.type Semantics.CardType.land)) Semantics.Plurality.one) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)]) (some (Semantics.ZoneExpr.zone Semantics.Zone.exile Semantics.ZoneScope.bare)) Semantics.ObservationPoint.departure none []))] none (Semantics.Instruction.enact (Semantics.Deed.core Semantics.CoreDeed.return_) (Semantics.Instruction.move (Semantics.NounPhrase.pro (Semantics.Reach.word (Semantics.NounWord.type Semantics.CardType.land)) Semantics.Plurality.one) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none [(Semantics.MoveRider.entry (Semantics.EntryRider.entersAs Semantics.Status.tapped)), (Semantics.MoveRider.entry (Semantics.EntryRider.under Semantics.NounPhrase.actor))]) none none))]) none none))

/-- `keyword_actions/endure.ron` -/
semantic_definition endure (arg0 : Semantics.Amount) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "endure")) (Semantics.Instruction.enact (Semantics.Deed.action "endure") (Semantics.Instruction.instruct (Semantics.NounPhrase.possessorOf Semantics.PossessorAxis.controller Semantics.NounPhrase.actor) (Semantics.Instruction.chooseModes (Semantics.Quantity.range (some 1) (some 1)) [(none, none, (Semantics.Instruction.putCounters (Semantics.Amount.inCaller arg0) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.permanent) Semantics.Plurality.one) Semantics.CounterBatch.one true)), (none, none, (Semantics.Instruction.createObject (Semantics.Amount.lit 1) (Semantics.CreationSpec.token (Semantics.TokenSpec.written ({ characteristics := ({ name := none, cost := none, colors := [Semantics.Color.white], supertypes := [], types := [Semantics.CardType.creature], subtypes := [(Semantics.Subtype.of Semantics.CardType.creature "Spirit")], text := [], power := (some (Semantics.Amount.inCaller arg0)), toughness := (some (Semantics.Amount.inCaller arg0)), loyalty := none, defense := none } : Semantics.Characteristics), qualities := [] } : Semantics.CharacteristicBundle)) []) true none))])) none none))

/-- `keyword_actions/exert.ron` -/
semantic_definition exert (arg0 : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "exert")) (Semantics.Instruction.enact (Semantics.Deed.action "exert") (Semantics.Instruction.skipUntap (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "exert.arg0" arg0)) (Semantics.Amount.lit 1)) none none))

/-- `keyword_actions/exile.ron` -/
semantic_definition exile (arg0 : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "exile")) (Semantics.Instruction.enact (Semantics.Deed.action "exile") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "exile.arg0" arg0)) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.exile Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) none none))

/-- `keyword_actions/explore.ron` -/
semantic_definition explore : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "explore")) (Semantics.Instruction.enact (Semantics.Deed.action "explore") (Semantics.Instruction.instruct (Semantics.NounPhrase.possessorOf Semantics.PossessorAxis.controller Semantics.NounPhrase.actor) (Semantics.Instruction.sequentially [(Semantics.Instruction.expose Semantics.ExposeVerb.reveal (Semantics.Exposed.cards (Semantics.NounPhrase.slice Semantics.LibraryEnd.top (Semantics.Amount.lit 1) (Semantics.Pile.library (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.actor))))), (Semantics.Instruction.doIf (Semantics.Condition.matches (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.card) Semantics.Plurality.one) (Semantics.Predicate.hasType Semantics.CardType.land)) (Semantics.Instruction.move (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.card) Semantics.Plurality.one) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.hand Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) (some (Semantics.Instruction.sequentially [(Semantics.Instruction.putCounters (Semantics.Amount.lit 1) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.permanent) Semantics.Plurality.one) Semantics.CounterBatch.one true), (Semantics.Instruction.withContinuation Semantics.ContinuationPolicy.optional (Semantics.Instruction.move (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.card) Semantics.Plurality.one) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) none none)])))])) none none))

/-- `keyword_actions/faceAVillainousChoice.ron` -/
semantic_definition faceAVillainousChoice (arg0 : Semantics.Instruction) (arg1 : Semantics.Instruction) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "faceAVillainousChoice")) (Semantics.Instruction.enact (Semantics.Deed.action "faceAVillainousChoice") (Semantics.Instruction.chooseModes (Semantics.Quantity.range (some 1) (some 1)) [(none, none, (Semantics.Instruction.inCaller arg0)), (none, none, (Semantics.Instruction.inCaller arg1))]) none none))

/-- `keyword_actions/fateseal.ron` -/
semantic_definition fateseal (arg0 : Semantics.Amount) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "fateseal")) (Semantics.Instruction.enact (Semantics.Deed.action "fateseal") (Semantics.Instruction.sequentially [(Semantics.Instruction.expose Semantics.ExposeVerb.lookAt (Semantics.Exposed.cards (Semantics.NounPhrase.firstMention "lookAndSortSlice.slice" (Semantics.NounPhrase.slice Semantics.LibraryEnd.top (Semantics.Amount.inCaller arg0) (Semantics.Pile.library (Semantics.ZoneScope.possessedBy (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) Semantics.Predicate.opponent))))))), (Semantics.Instruction.move (Semantics.NounPhrase.someOf (Semantics.SliceCount.counted (Semantics.Quantity.range none none)) none (Semantics.NounPhrase.laterMention "lookAndSortSlice.slice" (Semantics.NounPhrase.slice Semantics.LibraryEnd.top (Semantics.Amount.inCaller arg0) (Semantics.Pile.library (Semantics.ZoneScope.possessedBy (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) Semantics.Predicate.opponent)))))) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.place Semantics.LibraryEnd.bottom (Semantics.Pile.library Semantics.ZoneScope.bare))) Semantics.ObservationPoint.arrival none [(Semantics.MoveRider.placement (Semantics.PilePlacement.inOrder Semantics.Arrangement.anyOrder))]), (Semantics.Instruction.move (Semantics.NounPhrase.theRest Semantics.Kind.object Semantics.Plurality.many) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.place Semantics.LibraryEnd.top (Semantics.Pile.library Semantics.ZoneScope.bare))) Semantics.ObservationPoint.arrival none [(Semantics.MoveRider.placement (Semantics.PilePlacement.inOrder Semantics.Arrangement.anyOrder))])]) none none))

/-- `keyword_actions/fight.ron` -/
semantic_definition fight (arg0 : Semantics.NounPhrase) : Semantics.Instruction :=
  Semantics.Macros.fight arg0

/-- `keyword_actions/forage.ron` -/
semantic_definition forage : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "forage")) (Semantics.Instruction.enact (Semantics.Deed.action "forage") (Semantics.Instruction.chooseModes (Semantics.Quantity.range (some 1) (some 1)) [(none, none, (Semantics.Instruction.enact (Semantics.Deed.action "exile") (Semantics.Instruction.move (Semantics.NounPhrase.described (Semantics.DetPhrase.count (Semantics.Quantity.range (some 3) (some 3)) none) (Semantics.Predicate.and [Semantics.Predicate.isCard, (Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.graveyard (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.actor)))])) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.graveyard (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.actor))]) (some (Semantics.ZoneExpr.zone Semantics.Zone.exile Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) none none)), (none, none, (Semantics.Instruction.ownScope (some (Semantics.Deed.action "sacrifice")) (Semantics.Instruction.enact (Semantics.Deed.action "sacrifice") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.hasSubtype (Semantics.Subtype.of Semantics.CardType.artifact "Food")), (Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.actor)]))) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.departure none []) none none)))]) none none))

/-- `keyword_actions/goad.ron` -/
semantic_definition goad (arg0 : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "goad")) (Semantics.Instruction.enact (Semantics.Deed.action "goad") (Semantics.Instruction.gainDesignation (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "goad.arg0" arg0)) (Semantics.DesignationLabel.named "goaded") (some (Semantics.Duration.between none (some (Semantics.GameEvent.beginningOf Semantics.TurnPart.turn (some (Semantics.NounPhrase.possessorOf Semantics.PossessorAxis.controller Semantics.NounPhrase.this)) (Semantics.TurnRef.next none)))))) none none))

/-- `keyword_actions/harness.ron` -/
semantic_definition harness (arg0 : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "harness")) (Semantics.Instruction.enact (Semantics.Deed.action "harness") (Semantics.Instruction.doIf (Semantics.Condition.not (Semantics.Condition.matches (Semantics.NounPhrase.firstMention "harness.0" (Semantics.NounPhrase.inCaller arg0)) (Semantics.Predicate.hasDesignation (Semantics.DesignationLabel.named "harnessed") none))) (Semantics.Instruction.gainDesignation (Semantics.NounPhrase.laterMention "harness.0" (Semantics.NounPhrase.inCaller arg0)) (Semantics.DesignationLabel.named "harnessed") none) none) none none))

/-- `keyword_actions/heal.ron` -/
semantic_definition heal (arg0 : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "heal")) (Semantics.Instruction.enact (Semantics.Deed.action "heal") (Semantics.Instruction.clearDamage (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "heal.arg0" arg0))) none none))

/-- `keyword_actions/incubate.ron` -/
semantic_definition incubate (arg0 : Semantics.Amount) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "incubate")) (Semantics.Instruction.enact (Semantics.Deed.action "incubate") (Semantics.Instruction.createObject (Semantics.Amount.lit 1) (Semantics.CreationSpec.token (Semantics.TokenSpec.doubleFaced ({ characteristics := ({ name := none, cost := none, colors := [], supertypes := [], types := [Semantics.CardType.artifact], subtypes := [(Semantics.Subtype.of Semantics.CardType.artifact "Incubator")], text := [(Semantics.Ability.activated (Semantics.Cost.mana [(Semantics.ManaSymbol.simple (Semantics.SimpleManaSymbol.generic 2))]) (Semantics.Instruction.ownScope (some (Semantics.Deed.action "transform")) (Semantics.Instruction.enact (Semantics.Deed.action "transform") (Semantics.Instruction.turnOver (Semantics.NounPhrase.inCaller Semantics.NounPhrase.this)) none none)) none none none none)], power := none, toughness := none, loyalty := none, defense := none } : Semantics.Characteristics), qualities := [] } : Semantics.CharacteristicBundle) ({ characteristics := ({ name := (some "Phyrexian Token"), cost := none, colors := [], supertypes := [], types := [Semantics.CardType.artifact, Semantics.CardType.creature], subtypes := [(Semantics.Subtype.of Semantics.CardType.creature "Phyrexian")], text := [], power := (some (Semantics.Amount.lit 0)), toughness := (some (Semantics.Amount.lit 0)), loyalty := none, defense := none } : Semantics.Characteristics), qualities := [] } : Semantics.CharacteristicBundle)) [(Semantics.EntryRider.withCounters (Semantics.Amount.inCaller arg0) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) Semantics.EntryCounterMark.fresh)]) true none) none none))

/-- `keyword_actions/investigate.ron` -/
semantic_definition investigate : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "investigate")) (Semantics.Instruction.enact (Semantics.Deed.action "investigate") (Semantics.Instruction.createObject (Semantics.Amount.lit 1) (Semantics.CreationSpec.token (Semantics.TokenSpec.written ({ characteristics := ({ name := none, cost := none, colors := [], supertypes := [], types := [Semantics.CardType.artifact], subtypes := [(Semantics.Subtype.of Semantics.CardType.artifact "Clue")], text := [(Semantics.Ability.activated (Semantics.Cost.compound [(Semantics.Cost.mana [(Semantics.ManaSymbol.simple (Semantics.SimpleManaSymbol.generic 2))]), (Semantics.Cost.perform (Semantics.Instruction.ownScope (some (Semantics.Deed.action "sacrifice")) (Semantics.Instruction.enact (Semantics.Deed.action "sacrifice") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller Semantics.NounPhrase.this) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.departure none []) none none)))]) (Semantics.Instruction.draw (Semantics.Amount.lit 1)) none none none none)], power := none, toughness := none, loyalty := none, defense := none } : Semantics.Characteristics), qualities := [] } : Semantics.CharacteristicBundle)) []) true none) none none))

/-- `keyword_actions/manifest.ron` -/
semantic_definition manifest (arg0 : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "manifest")) (Semantics.Instruction.enact (Semantics.Deed.action "manifest") (Semantics.Instruction.sequentially [(Semantics.Instruction.turnFaceDown (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "manifest.arg0" arg0)) ({ characteristics := (some ({ name := none, cost := none, colors := [], supertypes := [], types := [Semantics.CardType.creature], subtypes := [], text := [], power := (some (Semantics.Amount.lit 2)), toughness := (some (Semantics.Amount.lit 2)), loyalty := none, defense := none } : Semantics.Characteristics)) } : Semantics.FaceDown)), (Semantics.Instruction.move (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.card) Semantics.Plurality.one) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none [(Semantics.MoveRider.entry (Semantics.EntryRider.faceDown ({ characteristics := (some ({ name := none, cost := none, colors := [], supertypes := [], types := [Semantics.CardType.creature], subtypes := [], text := [], power := (some (Semantics.Amount.lit 2)), toughness := (some (Semantics.Amount.lit 2)), loyalty := none, defense := none } : Semantics.Characteristics)) } : Semantics.FaceDown)))])]) none none))

/-- `keyword_actions/manifestDread.ron` -/
semantic_definition manifestDread : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "manifestDread")) (Semantics.Instruction.enact (Semantics.Deed.action "manifestDread") (Semantics.Instruction.sequentially [(Semantics.Instruction.expose Semantics.ExposeVerb.lookAt (Semantics.Exposed.cards (Semantics.NounPhrase.slice Semantics.LibraryEnd.top (Semantics.Amount.lit 2) (Semantics.Pile.library (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.actor))))), (Semantics.Instruction.ownScope (some (Semantics.Deed.action "manifest")) (Semantics.Instruction.enact (Semantics.Deed.action "manifest") (Semantics.Instruction.sequentially [(Semantics.Instruction.turnFaceDown (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.someOf (Semantics.SliceCount.counted (Semantics.Quantity.range (some 1) (some 1))) none (Semantics.NounPhrase.pro Semantics.Reach.bare Semantics.Plurality.many))) ({ characteristics := (some ({ name := none, cost := none, colors := [], supertypes := [], types := [Semantics.CardType.creature], subtypes := [], text := [], power := (some (Semantics.Amount.lit 2)), toughness := (some (Semantics.Amount.lit 2)), loyalty := none, defense := none } : Semantics.Characteristics)) } : Semantics.FaceDown)), (Semantics.Instruction.move (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.card) Semantics.Plurality.one) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none [(Semantics.MoveRider.entry (Semantics.EntryRider.faceDown ({ characteristics := (some ({ name := none, cost := none, colors := [], supertypes := [], types := [Semantics.CardType.creature], subtypes := [], text := [], power := (some (Semantics.Amount.lit 2)), toughness := (some (Semantics.Amount.lit 2)), loyalty := none, defense := none } : Semantics.Characteristics)) } : Semantics.FaceDown)))])]) none none)), (Semantics.Instruction.move (Semantics.NounPhrase.theRest Semantics.Kind.object Semantics.Plurality.many) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none [])]) none none))

/-- `keyword_actions/meld.ron` -/
semantic_definition meld (arg0 : Semantics.NounPhrase) (arg1 : String) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "meld")) (Semantics.Instruction.enact (Semantics.Deed.action "meld") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "meld.arg0" arg0)) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none [(Semantics.MoveRider.entry (Semantics.EntryRider.entersMelded arg1))]) none none))

/-- `keyword_actions/mill.ron` -/
semantic_definition mill (arg0 : Semantics.Amount) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "mill")) (Semantics.Instruction.enact (Semantics.Deed.action "mill") (Semantics.Instruction.move (Semantics.NounPhrase.slice Semantics.LibraryEnd.top (Semantics.Amount.inCaller arg0) (Semantics.Pile.library (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.actor))) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.library Semantics.ZoneScope.bare)]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) none none))

/-- `keyword_actions/monstrosity.ron` -/
semantic_definition monstrosity (arg0 : Semantics.Amount) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "monstrosity")) (Semantics.Instruction.enact (Semantics.Deed.action "monstrosity") (Semantics.Instruction.doIf (Semantics.Condition.not (Semantics.Condition.matches (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.Predicate.hasDesignation (Semantics.DesignationLabel.named "monstrous") none))) (Semantics.Instruction.sequentially [(Semantics.Instruction.putCounters (Semantics.Amount.inCaller arg0) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) Semantics.CounterBatch.one true), (Semantics.Instruction.gainDesignation (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this) (Semantics.DesignationLabel.named "monstrous") none)]) none) none none))

/-- `keyword_actions/play.ron` -/
semantic_definition play (arg0 : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "play")) (Semantics.Instruction.enact (Semantics.Deed.action "play") (Semantics.Instruction.procedure Semantics.Procedure.play (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "play.arg0" arg0)) Semantics.PlayPayment.itsOwnCost none none none) none none))

/-- `keyword_actions/populate.ron` -/
semantic_definition populate : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "populate")) (Semantics.Instruction.enact (Semantics.Deed.action "populate") (Semantics.Instruction.sequentially [(Semantics.Instruction.choose none (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [Semantics.Predicate.isToken, (Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.actor)])) Semantics.Disclosure.openly none), (Semantics.Instruction.createObject (Semantics.Amount.lit 1) (Semantics.CreationSpec.token (Semantics.TokenSpec.copyOf (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.token) Semantics.Plurality.one) []) []) true none)]) none none))

/-- `keyword_actions/proliferate.ron` -/
semantic_definition proliferate : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "proliferate")) (Semantics.Instruction.enact (Semantics.Deed.action "proliferate") (Semantics.Instruction.sequentially [(Semantics.Instruction.choose none (Semantics.NounPhrase.described (Semantics.DetPhrase.count (Semantics.Quantity.range none none) none) (Semantics.Predicate.or [(Semantics.Predicate.and [(Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)), (Semantics.Predicate.hasCounters none)]), (Semantics.Predicate.compare [(Semantics.ProjAxis.anyCounter Semantics.Kind.player)] Semantics.Comparator.atLeast (Semantics.Amount.lit 1))])) Semantics.Disclosure.openly none), (Semantics.Instruction.putCounters (Semantics.Amount.lit 1) Semantics.CounterKindSource.own (Semantics.NounPhrase.eachOf (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.join) Semantics.Plurality.many)) Semantics.CounterBatch.one true)]) none none))

/-- `keyword_actions/recruit.ron` -/
semantic_definition recruit : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "recruit")) (Semantics.Instruction.enact (Semantics.Deed.action "recruit") (Semantics.Instruction.sequentially [(Semantics.Instruction.draw (Semantics.Amount.lit 1)), (Semantics.Instruction.ownScope (some (Semantics.Deed.action "discard")) (Semantics.Instruction.enact (Semantics.Deed.action "discard") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [Semantics.Predicate.isCard, (Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.hand (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.actor)))]))) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.hand (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.actor))]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) none none)), (Semantics.Instruction.doIf (Semantics.Condition.not (Semantics.Condition.matches (Semantics.NounPhrase.pro (Semantics.Reach.verbed (Semantics.Deed.action "discard") (some Semantics.NounWord.card) Semantics.VerbedMarking.thisWay) Semantics.Plurality.one) (Semantics.Predicate.hasType Semantics.CardType.land))) (Semantics.Instruction.createObject (Semantics.Amount.lit 1) (Semantics.CreationSpec.token (Semantics.TokenSpec.written ({ characteristics := ({ name := none, cost := none, colors := [Semantics.Color.white], supertypes := [], types := [Semantics.CardType.creature], subtypes := [(Semantics.Subtype.of Semantics.CardType.creature "Human"), (Semantics.Subtype.of Semantics.CardType.creature "Soldier")], text := [], power := (some (Semantics.Amount.lit 1)), toughness := (some (Semantics.Amount.lit 1)), loyalty := none, defense := none } : Semantics.Characteristics), qualities := [] } : Semantics.CharacteristicBundle)) []) true none) none)]) none none))

/-- `keyword_actions/regenerate.ron` -/
semantic_definition regenerate (arg0 : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "regenerate")) (Semantics.Instruction.enact (Semantics.Deed.action "regenerate") (Semantics.Instruction.establish (Semantics.StaticSpec.instead (Semantics.WouldClause.would Semantics.Subject.any (Semantics.Instruction.enact (Semantics.Deed.action "destroy") (Semantics.Macros.deonticActionFor (Semantics.Deed.action "destroy") (Semantics.NounPhrase.firstMention "regenerate.0" (Semantics.NounPhrase.inCaller arg0))) none none)) (Semantics.Instruction.enact (Semantics.Deed.action "regenerate") (Semantics.Instruction.sequentially [(Semantics.Instruction.clearDamage (Semantics.NounPhrase.firstMention "regenerationApplication.subject" (Semantics.NounPhrase.laterMention "regenerate.0" (Semantics.NounPhrase.inCaller arg0)))), (Semantics.Instruction.instruct (Semantics.NounPhrase.possessorOf Semantics.PossessorAxis.controller (Semantics.NounPhrase.laterMention "regenerationApplication.subject" (Semantics.NounPhrase.laterMention "regenerate.0" (Semantics.NounPhrase.inCaller arg0)))) (Semantics.Instruction.enact (Semantics.Deed.action "tap") (Semantics.Instruction.setStatus Semantics.Status.tapped (Semantics.NounPhrase.laterMention "regenerationApplication.subject" (Semantics.NounPhrase.laterMention "regenerate.0" (Semantics.NounPhrase.inCaller arg0)))) none none)), (Semantics.Instruction.doIf (Semantics.Condition.or [(Semantics.Condition.matches (Semantics.NounPhrase.laterMention "regenerationApplication.subject" (Semantics.NounPhrase.laterMention "regenerate.0" (Semantics.NounPhrase.inCaller arg0))) (Semantics.Predicate.inCombat Semantics.CombatRelation.attackerOf none)), (Semantics.Condition.matches (Semantics.NounPhrase.laterMention "regenerationApplication.subject" (Semantics.NounPhrase.laterMention "regenerate.0" (Semantics.NounPhrase.inCaller arg0))) (Semantics.Predicate.inCombat Semantics.CombatRelation.blockerOf none))]) (Semantics.Instruction.combat (Semantics.NounPhrase.laterMention "regenerationApplication.subject" (Semantics.NounPhrase.laterMention "regenerate.0" (Semantics.NounPhrase.inCaller arg0))) (Semantics.CombatUpdate.participation Semantics.CombatParticipation.outsideCombat)) none)]) none none) none Semantics.ReplUse.nextTimeOnly none) (some (Semantics.Duration.between none (some (Semantics.GameEvent.endOf Semantics.TurnPart.turn none Semantics.TurnRef.current))))) none none))

/-- `keyword_actions/reveal.ron` -/
semantic_definition reveal (arg0 : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "reveal")) (Semantics.Instruction.enact (Semantics.Deed.action "reveal") (Semantics.Instruction.expose Semantics.ExposeVerb.reveal (Semantics.Exposed.cards (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "reveal.arg0" arg0)))) none none))

/-- `keyword_actions/sacrifice.ron` -/
semantic_definition sacrifice (arg0 : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "sacrifice")) (Semantics.Instruction.enact (Semantics.Deed.action "sacrifice") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "sacrifice.arg0" arg0)) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.departure none []) none none))

/-- `keyword_actions/scry.ron` -/
semantic_definition scry (arg0 : Semantics.Amount) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "scry")) (Semantics.Instruction.enact (Semantics.Deed.action "scry") (Semantics.Instruction.sequentially [(Semantics.Instruction.expose Semantics.ExposeVerb.lookAt (Semantics.Exposed.cards (Semantics.NounPhrase.firstMention "lookAndSortSlice.slice" (Semantics.NounPhrase.slice Semantics.LibraryEnd.top (Semantics.Amount.inCaller arg0) (Semantics.Pile.library (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.actor)))))), (Semantics.Instruction.move (Semantics.NounPhrase.someOf (Semantics.SliceCount.counted (Semantics.Quantity.range none none)) none (Semantics.NounPhrase.laterMention "lookAndSortSlice.slice" (Semantics.NounPhrase.slice Semantics.LibraryEnd.top (Semantics.Amount.inCaller arg0) (Semantics.Pile.library (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.actor))))) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.place Semantics.LibraryEnd.bottom (Semantics.Pile.library Semantics.ZoneScope.bare))) Semantics.ObservationPoint.arrival none [(Semantics.MoveRider.placement (Semantics.PilePlacement.inOrder Semantics.Arrangement.anyOrder))]), (Semantics.Instruction.move (Semantics.NounPhrase.theRest Semantics.Kind.object Semantics.Plurality.many) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.place Semantics.LibraryEnd.top (Semantics.Pile.library Semantics.ZoneScope.bare))) Semantics.ObservationPoint.arrival none [(Semantics.MoveRider.placement (Semantics.PilePlacement.inOrder Semantics.Arrangement.anyOrder))])]) none none))

/-- `keyword_actions/search.ron` -/
semantic_definition search (arg0 : Semantics.SearchScope) (arg1 : Semantics.Quantity) (arg2 : Semantics.Predicate) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "search")) (Semantics.Instruction.enact (Semantics.Deed.action "search") (Semantics.Instruction.search arg0 (Semantics.Quantity.inCaller arg1) (Semantics.Predicate.inCaller arg2)) none none))

/-- `keyword_actions/shuffle.ron` -/
semantic_definition shuffle : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "shuffle")) (Semantics.Instruction.enact (Semantics.Deed.action "shuffle") (Semantics.Instruction.shuffle (Semantics.Pile.library (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.actor))) none none))

/-- `keyword_actions/support.ron` -/
semantic_definition support (arg0 : Semantics.Amount) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "support")) (Semantics.Instruction.enact (Semantics.Deed.action "support") (Semantics.Instruction.putCounters (Semantics.Amount.lit 1) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "p1p1Counter")) (Semantics.NounPhrase.eachOf (Semantics.NounPhrase.described (Semantics.DetPhrase.target (Semantics.Quantity.upToOf (Semantics.Amount.inCaller arg0))) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.otherThan Semantics.NounPhrase.this)]))) Semantics.CounterBatch.one true) none none))

/-- `keyword_actions/surveil.ron` -/
semantic_definition surveil (arg0 : Semantics.Amount) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "surveil")) (Semantics.Instruction.enact (Semantics.Deed.action "surveil") (Semantics.Instruction.sequentially [(Semantics.Instruction.expose Semantics.ExposeVerb.lookAt (Semantics.Exposed.cards (Semantics.NounPhrase.firstMention "lookAndSortSlice.slice" (Semantics.NounPhrase.slice Semantics.LibraryEnd.top (Semantics.Amount.inCaller arg0) (Semantics.Pile.library (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.actor)))))), (Semantics.Instruction.move (Semantics.NounPhrase.someOf (Semantics.SliceCount.counted (Semantics.Quantity.range none none)) none (Semantics.NounPhrase.laterMention "lookAndSortSlice.slice" (Semantics.NounPhrase.slice Semantics.LibraryEnd.top (Semantics.Amount.inCaller arg0) (Semantics.Pile.library (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.actor))))) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []), (Semantics.Instruction.move (Semantics.NounPhrase.theRest Semantics.Kind.object Semantics.Plurality.many) Semantics.EventSource.anywhere (some (Semantics.ZoneExpr.place Semantics.LibraryEnd.top (Semantics.Pile.library Semantics.ZoneScope.bare))) Semantics.ObservationPoint.arrival none [(Semantics.MoveRider.placement (Semantics.PilePlacement.inOrder Semantics.Arrangement.anyOrder))])]) none none))

/-- `keyword_actions/suspect.ron` -/
semantic_definition suspect (arg0 : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "suspect")) (Semantics.Instruction.enact (Semantics.Deed.action "suspect") (Semantics.Instruction.gainDesignation (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "suspect.arg0" arg0)) (Semantics.DesignationLabel.named "suspected") none) none none))

/-- `keyword_actions/tap.ron` -/
semantic_definition tap (arg0 : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "tap")) (Semantics.Instruction.enact (Semantics.Deed.action "tap") (Semantics.Instruction.setStatus Semantics.Status.tapped (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "tap.arg0" arg0))) none none))

/-- `keyword_actions/theRingTemptsYou.ron` -/
semantic_definition theRingTemptsYou : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "theRingTemptsYou")) (Semantics.Instruction.enact (Semantics.Deed.action "theRingTemptsYou") (Semantics.Instruction.sequentially [(Semantics.Instruction.doIf (Semantics.Condition.not (Semantics.Condition.exists_ (Semantics.NounPhrase.described Semantics.DetPhrase.bare (Semantics.Predicate.and [Semantics.Predicate.isEmblem, (Semantics.Predicate.named (Semantics.NameSource.printed "The Ring")), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.owner Semantics.NounPhrase.actor)])))) (Semantics.Instruction.createObject (Semantics.Amount.lit 1) (Semantics.CreationSpec.emblem (some "The Ring") [(Semantics.Ability.static (Semantics.StaticSpec.characteristicChange (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.and [(Semantics.Predicate.hasDesignation (Semantics.DesignationLabel.named "ringBearer") none), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)])) [(Semantics.CharacteristicEdit.typeLine Semantics.QualityOp.adds ({ supertypes := (some [Semantics.Supertype.legendary]), types := none, subtypes := none, retained := none } : Semantics.TypeLineChanges))])), (Semantics.Ability.static (Semantics.StaticSpec.cant (Semantics.Subject.by (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.compare [(Semantics.ProjAxis.stat Semantics.Stat.power)] Semantics.Comparator.greater (Semantics.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.power) (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasDesignation (Semantics.DesignationLabel.named "ringBearer") none), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)]))))]))) (Semantics.Macros.deonticActionFor (Semantics.Deed.core Semantics.CoreDeed.block) (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasDesignation (Semantics.DesignationLabel.named "ringBearer") none), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)]))) ({ bound := none, asThough := none, rider := Semantics.DeonticRider.noRider, except := none } : Semantics.DeonticMods))), (Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.abilityGrant Semantics.NounPhrase.this (Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.hasDesignation (Semantics.DesignationLabel.named "ringBearer") none), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)]))) (Semantics.Instruction.attack none)) [] none [] none none none (Semantics.Instruction.sequentially [(Semantics.Instruction.draw (Semantics.Amount.lit 1)), (Semantics.Instruction.ownScope (some (Semantics.Deed.action "discard")) (Semantics.Instruction.enact (Semantics.Deed.action "discard") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [Semantics.Predicate.isCard, (Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.hand Semantics.ZoneScope.bare))]))) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.hand (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.actor))]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.arrival none []) none none))]))) (Semantics.Condition.compareAmt (Semantics.Amount.tally (Semantics.DidClause.mk (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Instruction.enact (Semantics.Deed.action "theRingTemptsYou") (Semantics.Macros.deonticActionFor (Semantics.Deed.action "theRingTemptsYou") Semantics.NounPhrase.any) none none) (Semantics.LookbackSpan.duration (Semantics.Duration.between none (some Semantics.GameEvent.now)) none)) Semantics.TallyMode.events) Semantics.Comparator.atLeast (Semantics.Amount.lit 2)) Semantics.CondMarking.asLongAs)), (Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.abilityGrant Semantics.NounPhrase.this (Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.by (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.hasType Semantics.CardType.creature))) (Semantics.Instruction.block (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.hasDesignation (Semantics.DesignationLabel.named "ringBearer") none), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)])))) [] none [] none none none (Semantics.Instruction.delay (Semantics.GameEvent.beginningOf Semantics.TurnPart.endOfCombat none (Semantics.TurnRef.next none)) [] none (Semantics.Instruction.instruct (Semantics.NounPhrase.possessorOf Semantics.PossessorAxis.controller (Semantics.NounPhrase.pro (Semantics.Reach.word (Semantics.NounWord.type Semantics.CardType.creature)) Semantics.Plurality.one)) (Semantics.Instruction.ownScope (some (Semantics.Deed.action "sacrifice")) (Semantics.Instruction.enact (Semantics.Deed.action "sacrifice") (Semantics.Instruction.move (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.pro (Semantics.Reach.word (Semantics.NounWord.type Semantics.CardType.creature)) Semantics.Plurality.one)) (Semantics.EventSource.zones [(Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)]) (some (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)) Semantics.ObservationPoint.departure none []) none none)))))) (Semantics.Condition.compareAmt (Semantics.Amount.tally (Semantics.DidClause.mk (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Instruction.enact (Semantics.Deed.action "theRingTemptsYou") (Semantics.Macros.deonticActionFor (Semantics.Deed.action "theRingTemptsYou") Semantics.NounPhrase.any) none none) (Semantics.LookbackSpan.duration (Semantics.Duration.between none (some Semantics.GameEvent.now)) none)) Semantics.TallyMode.events) Semantics.Comparator.atLeast (Semantics.Amount.lit 3)) Semantics.CondMarking.asLongAs)), (Semantics.Ability.static (Semantics.StaticSpec.conditional (Semantics.StaticSpec.abilityGrant Semantics.NounPhrase.this (Semantics.Ability.triggered (Semantics.GameEvent.does (Semantics.Subject.actor (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.hasDesignation (Semantics.DesignationLabel.named "ringBearer") none), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)]))) (Semantics.Instruction.dealDamage Semantics.Amount.anyAmount (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) Semantics.Predicate.anyPlayer) Semantics.DamageKind.combatOnly)) [] none [] none none none (Semantics.Instruction.instruct (Semantics.NounPhrase.described Semantics.DetPhrase.each Semantics.Predicate.opponent) (Semantics.Instruction.changeLife (Semantics.Delta.down (Semantics.Amount.lit 3)))))) (Semantics.Condition.compareAmt (Semantics.Amount.tally (Semantics.DidClause.mk (Semantics.Subject.actor Semantics.NounPhrase.you) (Semantics.Instruction.enact (Semantics.Deed.action "theRingTemptsYou") (Semantics.Macros.deonticActionFor (Semantics.Deed.action "theRingTemptsYou") Semantics.NounPhrase.any) none none) (Semantics.LookbackSpan.duration (Semantics.Duration.between none (some Semantics.GameEvent.now)) none)) Semantics.TallyMode.events) Semantics.Comparator.atLeast (Semantics.Amount.lit 4)) Semantics.CondMarking.asLongAs))]) true none) none), (Semantics.Instruction.choose none (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.actor)])) Semantics.Disclosure.openly none), (Semantics.Instruction.gainDesignation (Semantics.NounPhrase.pro (Semantics.Reach.word (Semantics.NounWord.type Semantics.CardType.creature)) Semantics.Plurality.one) (Semantics.DesignationLabel.named "ringBearer") none)]) none none))

/-- `keyword_actions/timeTravel.ron` -/
semantic_definition timeTravel : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "timeTravel")) (Semantics.Instruction.enact (Semantics.Deed.action "timeTravel") (Semantics.Instruction.sequentially [(Semantics.Instruction.choose none (Semantics.NounPhrase.described (Semantics.DetPhrase.count (Semantics.Quantity.range none none) none) (Semantics.Predicate.or [(Semantics.Predicate.and [(Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.actor), (Semantics.Predicate.hasCounters (some (Semantics.CounterKind.named "timeCounter")))]), (Semantics.Predicate.and [Semantics.Predicate.isCard, (Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.exile Semantics.ZoneScope.bare)), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.owner Semantics.NounPhrase.actor), (Semantics.Predicate.hasCounters (some (Semantics.CounterKind.named "timeCounter")))])])) Semantics.Disclosure.openly none), (Semantics.Instruction.doForEach (Semantics.NounPhrase.pro Semantics.Reach.bare Semantics.Plurality.many) (Semantics.Instruction.chooseModes (Semantics.Quantity.range (some 1) (some 1)) [(none, none, (Semantics.Instruction.putCounters (Semantics.Amount.lit 1) (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "timeCounter")) (Semantics.NounPhrase.pro Semantics.Reach.bare Semantics.Plurality.one) Semantics.CounterBatch.one true)), (none, none, (Semantics.Instruction.removeCounters (some (Semantics.Quantity.range (some 1) (some 1))) (some (Semantics.CounterKindSource.printed (Semantics.CounterKind.named "timeCounter"))) (Semantics.NounPhrase.pro Semantics.Reach.bare Semantics.Plurality.one) Semantics.CounterBatch.one true))]))]) none none))

/-- `keyword_actions/transform.ron` -/
semantic_definition transform (arg0 : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "transform")) (Semantics.Instruction.enact (Semantics.Deed.action "transform") (Semantics.Instruction.turnOver (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "transform.arg0" arg0))) none none))

/-- `keyword_actions/untap.ron` -/
semantic_definition untap (arg0 : Semantics.NounPhrase) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "untap")) (Semantics.Instruction.enact (Semantics.Deed.action "untap") (Semantics.Instruction.setStatus Semantics.Status.untapped (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.firstMention "untap.arg0" arg0))) none none))

/-- `keyword_actions/vote.ron` -/
semantic_definition vote (arg0 : Semantics.Disclosure) (arg1 : Semantics.Ballot) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "vote")) (Semantics.Instruction.enact (Semantics.Deed.action "vote") (Semantics.Instruction.vote none arg0 arg1) none none))

/-- `keyword_actions/waterbend.ron` -/
semantic_definition waterbend (arg0 : Semantics.Cost) : Semantics.Instruction :=
  (Semantics.Instruction.ownScope (some (Semantics.Deed.action "waterbend")) (Semantics.Instruction.enact (Semantics.Deed.action "waterbend") (Semantics.Instruction.pay (Semantics.Cost.withSubstitute (Semantics.Cost.inCaller arg0) ({ action := (Semantics.Instruction.ownScope (some (Semantics.Deed.action "tap")) (Semantics.Instruction.enact (Semantics.Deed.action "tap") (Semantics.Instruction.setStatus Semantics.Status.tapped (Semantics.NounPhrase.inCaller (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) (Semantics.Predicate.and [(Semantics.Predicate.hasStatus Semantics.Status.untapped), (Semantics.Predicate.or [(Semantics.Predicate.hasType Semantics.CardType.artifact), (Semantics.Predicate.hasType Semantics.CardType.creature)]), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.actor)])))) none none)), pays := [Semantics.PipPaid.generic] } : Semantics.ManaSubstitute)) Semantics.PayTimes.once Semantics.PaymentOutcome.paid none none) none none))

/-- `mana/colorlessPip.ron` -/
semantic_template colorlessPip : Semantics.ManaSymbol :=
  (Semantics.ManaSymbol.simple (Semantics.SimpleManaSymbol.specific Semantics.ColorOrColorless.colorless))

/-- `mana/generic.ron` -/
semantic_template ManaSymbol.generic (amount : Nat) : Semantics.ManaSymbol :=
  (Semantics.ManaSymbol.simple (Semantics.SimpleManaSymbol.generic amount))

/-- `mana/hybridPip.ron` -/
semantic_template hybridPip (left : Semantics.Color) (right : Semantics.Color) : Semantics.ManaSymbol :=
  (Semantics.ManaSymbol.hybrid (Semantics.SimpleManaSymbol.specific (Semantics.ColorOrColorless.of left)) right)

/-- `mana/phyrexianPip.ron` -/
semantic_template phyrexianPip (color : Semantics.Color) : Semantics.ManaSymbol :=
  (Semantics.ManaSymbol.phyrexian color none)

/-- `mana/pip.ron` -/
semantic_template pip (color : Semantics.Color) : Semantics.ManaSymbol :=
  (Semantics.ManaSymbol.simple (Semantics.SimpleManaSymbol.specific (Semantics.ColorOrColorless.of color)))

/-- `nouns/actor.ron` -/
semantic_template actor : Semantics.NounPhrase :=
  Semantics.NounPhrase.actor

/-- `nouns/agree.ron` -/
semantic_template agree (axis : Semantics.QualitySort) (agreement : Semantics.Agreement) (group : Semantics.NounPhrase) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.agree axis agreement group)

/-- `nouns/allAmong.ron` -/
semantic_template allAmong (p : Semantics.Predicate) (group : Semantics.NounPhrase) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.someOf Semantics.SliceCount.whole (some p) group)

/-- `nouns/allFromAmong.ron` -/
semantic_template allFromAmong (p : Semantics.Predicate) (group : Semantics.NounPhrase) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.someOf Semantics.SliceCount.whole (some p) group)

/-- `nouns/among.ron` -/
semantic_template among (group : Semantics.NounPhrase) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.someOf Semantics.SliceCount.whole none group)

/-- `nouns/anOpponent.ron` -/
semantic_template anOpponent : Semantics.NounPhrase :=
  (Semantics.NounPhrase.described (Semantics.DetPhrase.a Semantics.ChoiceMode.unmarked) Semantics.Predicate.opponent)

/-- `nouns/and.ron` -/
semantic_template NounPhrase.and (phrases : (List Semantics.NounPhrase) := []) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.and phrases)

/-- `nouns/asMarker.ron` -/
semantic_template asMarker (marker : Semantics.MarkerWord) (subject : Semantics.NounPhrase) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.asMarker marker subject)

/-- `nouns/asType.ron` -/
semantic_template asType (type : Semantics.CardType) (subject : Semantics.NounPhrase) (subtype : (Option Semantics.Subtype) := none) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.asType type subject subtype)

/-- `nouns/attachHost.ron` -/
semantic_template attachHost (word : Semantics.AttachWord) (head : Semantics.NounWord) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.attachHost word head)

/-- `nouns/becomes.ron` -/
semantic_template becomes (subject : Semantics.NounPhrase) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.becomes subject)

/-- `nouns/bottomCard.ron` -/
semantic_template bottomCard : Semantics.NounPhrase :=
  (Semantics.NounPhrase.slice Semantics.LibraryEnd.bottom (Semantics.Amount.lit 1) (Semantics.Pile.library (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.you)))

/-- `nouns/chosenAsPaid.ron` -/
semantic_template chosenAsPaid (label : Semantics.KeywordLabel) (subject : Semantics.NounPhrase) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.paid (Semantics.LinkedFacet.chosenPlayer (Semantics.LinkSource.byKeyword label)) subject)

/-- `nouns/combatPlayer.ron` -/
semantic_template combatPlayer (role : Semantics.CombatRole) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.combatPlayer role)

/-- `nouns/controllerOf.ron` -/
semantic_template controllerOf (subject : Semantics.NounPhrase) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.possessorOf Semantics.PossessorAxis.controller subject)

/-- `nouns/counterpart.ron` -/
semantic_template counterpart (label : Semantics.RelationLabel) (subject : Semantics.NounPhrase) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.counterpart label subject)

/-- `nouns/defendingPlayer.ron` -/
semantic_template defendingPlayer : Semantics.NounPhrase :=
  (Semantics.NounPhrase.combatPlayer Semantics.CombatRole.defending)

/-- `nouns/described.ron` -/
semantic_template described (determiner : Semantics.DetPhrase) (predicate : Semantics.Predicate) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.described determiner predicate)

/-- `nouns/designated.ron` -/
semantic_template designated (designation : Semantics.DesignationLabel) (whose : Semantics.NounPhrase) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.designated designation whose)

/-- `nouns/eachOf.ron` -/
semantic_template eachOf (group : Semantics.NounPhrase) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.eachOf group)

/-- `nouns/everyObject.ron` -/
semantic_template everyObject : Semantics.NounPhrase :=
  (Semantics.NounPhrase.described Semantics.DetPhrase.all (Semantics.Predicate.and []))

/-- `nouns/exiledWithThisArtifact.ron` -/
semantic_template exiledWithThisArtifact : Semantics.Predicate :=
  (Semantics.Predicate.exiledWith (Semantics.NounPhrase.asType Semantics.CardType.artifact Semantics.NounPhrase.this none))

/-- `nouns/fromAmong.ron` -/
semantic_template fromAmong (quantity : Semantics.Quantity) (p : Semantics.Predicate) (group : Semantics.NounPhrase) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.someOf (Semantics.SliceCount.counted quantity) (some p) group)

/-- `nouns/gap.ron` -/
semantic_template gap (kind : Semantics.Kind) : Semantics.NounPhrase :=
  Semantics.NounPhrase.which

/-- `nouns/linkedObject.ron` -/
semantic_template linkedObject (which : Semantics.LinkSource) (deed : Semantics.Deed) (subject : Semantics.NounPhrase) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.paid (Semantics.LinkedFacet.objects which deed Semantics.Plurality.one) subject)

/-- `nouns/linkedObjects.ron` -/
semantic_template linkedObjects (which : Semantics.LinkSource) (deed : Semantics.Deed) (subject : Semantics.NounPhrase) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.paid (Semantics.LinkedFacet.objects which deed Semantics.Plurality.many) subject)

/-- `nouns/oneEachOf.ron` -/
semantic_template oneEachOf (pool : Semantics.NounPhrase) (roles : (List Semantics.Predicate) := []) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.oneEachOf roles pool)

/-- `nouns/onePile.ron` -/
semantic_template onePile : Semantics.NounPhrase :=
  (Semantics.NounPhrase.pileOf (Semantics.SliceCount.counted (Semantics.Quantity.range (some 1) (some 1))) none)

/-- `nouns/or.ron` -/
semantic_template NounPhrase.or (phrases : (List Semantics.NounPhrase) := []) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.or phrases)

/-- `nouns/ownerOf.ron` -/
semantic_template ownerOf (subject : Semantics.NounPhrase) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.possessorOf Semantics.PossessorAxis.owner subject)

/-- `nouns/pileOf.ron` -/
semantic_template pileOf (count : Semantics.SliceCount) (by_ : (Option Semantics.NounPhrase) := none) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.pileOf count by_)

/-- `nouns/pileOfChoice.ron` -/
semantic_template pileOfChoice (player : Semantics.NounPhrase) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.pileOf (Semantics.SliceCount.counted (Semantics.Quantity.range (some 1) (some 1))) (some player))

/-- `nouns/playerGroup.ron` -/
semantic_template playerGroup (group : Semantics.PlayerGroupWord) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.playerGroup group)

/-- `nouns/possessorOf.ron` -/
semantic_template possessorOf (axis : Semantics.PossessorAxis) (subject : Semantics.NounPhrase) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.possessorOf axis subject)

/-- `nouns/resolvedPermanent.ron` -/
semantic_template resolvedPermanent (spell : Semantics.NounPhrase) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.resolvedPermanent spell)

/-- `nouns/run.ron` -/
semantic_template run (end_ : Semantics.LibraryEnd) (stop : Semantics.Predicate) (pile : Semantics.Pile) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.run end_ stop pile)

/-- `nouns/slice.ron` -/
semantic_template slice (end_ : Semantics.LibraryEnd) (amount : Semantics.Amount) (pile : Semantics.Pile) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.slice end_ amount pile)

/-- `nouns/someOf.ron` -/
semantic_template someOf (quantity : Semantics.Quantity) (group : Semantics.NounPhrase) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.someOf (Semantics.SliceCount.counted quantity) none group)

/-- `nouns/theGrantor.ron` -/
semantic_template theGrantor (marker : Semantics.MarkerWord) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.theGrantor marker)

/-- `nouns/theOther.ron` -/
semantic_template theOther (kind : Semantics.Kind) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.theRest kind Semantics.Plurality.one)

/-- `nouns/theRest.ron` -/
semantic_template theRest (kind : Semantics.Kind) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.theRest kind Semantics.Plurality.many)

/-- `nouns/themAsTokens.ron` -/
semantic_template themAsTokens : Semantics.NounPhrase :=
  (Semantics.NounPhrase.pro Semantics.Reach.tokenBorn Semantics.Plurality.many)

/-- `nouns/this.ron` -/
semantic_template this_ : Semantics.NounPhrase :=
  Semantics.NounPhrase.this

/-- `nouns/thisAbility.ron` -/
semantic_template thisAbility : Semantics.NounPhrase :=
  (Semantics.NounPhrase.asMarker Semantics.MarkerWord.ability Semantics.NounPhrase.this)

/-- `nouns/thisArtifact.ron` -/
semantic_template thisArtifact : Semantics.NounPhrase :=
  (Semantics.NounPhrase.asType Semantics.CardType.artifact Semantics.NounPhrase.this none)

/-- `nouns/thisAura.ron` -/
semantic_template thisAura : Semantics.NounPhrase :=
  (Semantics.NounPhrase.asType Semantics.CardType.enchantment Semantics.NounPhrase.this (some (Semantics.Subtype.of Semantics.CardType.enchantment "Aura")))

/-- `nouns/thisCase.ron` -/
semantic_template thisCase : Semantics.NounPhrase :=
  (Semantics.NounPhrase.asType Semantics.CardType.enchantment Semantics.NounPhrase.this (some (Semantics.Subtype.of Semantics.CardType.enchantment "Case")))

/-- `nouns/thisClass.ron` -/
semantic_template thisClass : Semantics.NounPhrase :=
  (Semantics.NounPhrase.asType Semantics.CardType.enchantment Semantics.NounPhrase.this (some (Semantics.Subtype.of Semantics.CardType.enchantment "Class")))

/-- `nouns/thisCreature.ron` -/
semantic_template thisCreature : Semantics.NounPhrase :=
  (Semantics.NounPhrase.asType Semantics.CardType.creature Semantics.NounPhrase.this none)

/-- `nouns/thisEnchantment.ron` -/
semantic_template thisEnchantment : Semantics.NounPhrase :=
  (Semantics.NounPhrase.asType Semantics.CardType.enchantment Semantics.NounPhrase.this none)

/-- `nouns/thisEquipment.ron` -/
semantic_template thisEquipment : Semantics.NounPhrase :=
  (Semantics.NounPhrase.asType Semantics.CardType.artifact Semantics.NounPhrase.this (some (Semantics.Subtype.of Semantics.CardType.artifact "Equipment")))

/-- `nouns/thisFortification.ron` -/
semantic_template thisFortification : Semantics.NounPhrase :=
  (Semantics.NounPhrase.asType Semantics.CardType.artifact Semantics.NounPhrase.this (some (Semantics.Subtype.of Semantics.CardType.artifact "Fortification")))

/-- `nouns/thisLand.ron` -/
semantic_template thisLand : Semantics.NounPhrase :=
  (Semantics.NounPhrase.asType Semantics.CardType.land Semantics.NounPhrase.this none)

/-- `nouns/thisPermanent.ron` -/
semantic_template thisPermanent : Semantics.NounPhrase :=
  (Semantics.NounPhrase.asMarker Semantics.MarkerWord.permanent Semantics.NounPhrase.this)

/-- `nouns/thisPlaneswalker.ron` -/
semantic_template thisPlaneswalker : Semantics.NounPhrase :=
  (Semantics.NounPhrase.asType Semantics.CardType.planeswalker Semantics.NounPhrase.this none)

/-- `nouns/thisRoom.ron` -/
semantic_template thisRoom : Semantics.NounPhrase :=
  (Semantics.NounPhrase.asType Semantics.CardType.enchantment Semantics.NounPhrase.this (some (Semantics.Subtype.of Semantics.CardType.enchantment "Room")))

/-- `nouns/thisSaga.ron` -/
semantic_template thisSaga : Semantics.NounPhrase :=
  (Semantics.NounPhrase.asType Semantics.CardType.enchantment Semantics.NounPhrase.this (some (Semantics.Subtype.of Semantics.CardType.enchantment "Saga")))

/-- `nouns/thisSiege.ron` -/
semantic_template thisSiege : Semantics.NounPhrase :=
  (Semantics.NounPhrase.asType Semantics.CardType.battle Semantics.NounPhrase.this (some (Semantics.Subtype.of Semantics.CardType.battle "Siege")))

/-- `nouns/thisSpacecraft.ron` -/
semantic_template thisSpacecraft : Semantics.NounPhrase :=
  (Semantics.NounPhrase.asType Semantics.CardType.artifact Semantics.NounPhrase.this (some (Semantics.Subtype.of Semantics.CardType.artifact "Spacecraft")))

/-- `nouns/thisSpell.ron` -/
semantic_template thisSpell : Semantics.NounPhrase :=
  (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this)

/-- `nouns/thisVehicle.ron` -/
semantic_template thisVehicle : Semantics.NounPhrase :=
  (Semantics.NounPhrase.asType Semantics.CardType.artifact Semantics.NounPhrase.this (some (Semantics.Subtype.of Semantics.CardType.artifact "Vehicle")))

/-- `nouns/topSlice.ron` -/
semantic_template topSlice (amount : Semantics.Amount) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.slice Semantics.LibraryEnd.top amount (Semantics.Pile.library (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.you)))

/-- `nouns/withDifferentNames.ron` -/
semantic_template withDifferentNames (group : Semantics.NounPhrase) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.agree Semantics.QualitySort.cardName Semantics.Agreement.differ group)

/-- `nouns/withTheSameName.ron` -/
semantic_template withTheSameName (group : Semantics.NounPhrase) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.agree Semantics.QualitySort.cardName Semantics.Agreement.share group)

/-- `nouns/withTotal.ron` -/
semantic_template withTotal (axis : Semantics.ProjAxis) (comparator : Semantics.Comparator) (bound : Semantics.Amount) (group : Semantics.NounPhrase) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.someOf (Semantics.SliceCount.totalling axis comparator bound) none group)

/-- `nouns/you.ron` -/
semantic_template you : Semantics.NounPhrase :=
  Semantics.NounPhrase.you

/-- `nouns/yourCommander.ron` -/
semantic_template yourCommander : Semantics.NounPhrase :=
  (Semantics.NounPhrase.designated (Semantics.DesignationLabel.named "commander") Semantics.NounPhrase.you)

/-- `piles/library.ron` -/
semantic_template library (player : Semantics.NounPhrase) : Semantics.Pile :=
  (Semantics.Pile.library (Semantics.ZoneScope.possessedBy player))

/-- `pips/generic.ron` -/
semantic_template PipPaid.generic : Semantics.PipPaid :=
  Semantics.PipPaid.generic

/-- `pips/ofColor.ron` -/
semantic_template ofColor (color : Semantics.ColorTerm) : Semantics.PipPaid :=
  (Semantics.PipPaid.ofColor color)

/-- `predicates/abilityHead.ron` -/
semantic_template abilityHead (class_ : Semantics.AbilityClass) : Semantics.Predicate :=
  (Semantics.Predicate.abilityHead class_)

/-- `predicates/abilityOf.ron` -/
semantic_template abilityOf (source : Semantics.NounPhrase) : Semantics.Predicate :=
  (Semantics.Predicate.abilityOf source)

/-- `predicates/activatedBy.ron` -/
semantic_template Predicate.activatedBy (activator : Semantics.NounPhrase) : Semantics.Predicate :=
  (Semantics.Predicate.activatedBy activator)

/-- `predicates/actorControls.ron` -/
semantic_template actorControls : Semantics.Predicate :=
  (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.actor)

/-- `predicates/and.ron` -/
semantic_template Predicate.and (conjuncts : (List Semantics.Predicate) := []) : Semantics.Predicate :=
  (Semantics.Predicate.and conjuncts)

/-- `predicates/anotherSpell.ron` -/
semantic_template anotherSpell : Semantics.Predicate :=
  (Semantics.Predicate.and [(Semantics.Predicate.and [(Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.stack Semantics.ZoneScope.bare)), (Semantics.Predicate.not (Semantics.Predicate.abilityHead Semantics.AbilityClass.anyOnStack))]), (Semantics.Predicate.otherThan (Semantics.NounPhrase.asMarker Semantics.MarkerWord.spell Semantics.NounPhrase.this))])

/-- `predicates/anyOtherTarget.ron` -/
semantic_template anyOtherTarget : Semantics.Predicate :=
  (Semantics.Predicate.and [(Semantics.Predicate.or [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasType Semantics.CardType.planeswalker), (Semantics.Predicate.hasType Semantics.CardType.battle), Semantics.Predicate.anyPlayer]), Semantics.Predicate.other])

/-- `predicates/anyPlayer.ron` -/
semantic_template anyPlayer : Semantics.Predicate :=
  Semantics.Predicate.anyPlayer

/-- `predicates/anyTarget.ron` -/
semantic_template anyTarget : Semantics.Predicate :=
  (Semantics.Predicate.or [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasType Semantics.CardType.planeswalker), (Semantics.Predicate.hasType Semantics.CardType.battle), Semantics.Predicate.anyPlayer])

/-- `predicates/armyYouControl.ron` -/
semantic_template armyYouControl : Semantics.Predicate :=
  (Semantics.Predicate.and [(Semantics.Predicate.hasSubtype (Semantics.Subtype.of Semantics.CardType.creature "Army")), (Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)])

/-- `predicates/artifact.ron` -/
semantic_template artifact : Semantics.Predicate :=
  (Semantics.Predicate.hasType Semantics.CardType.artifact)

/-- `predicates/artifactType.ron` -/
semantic_template artifactType (label : String) : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.artifact label)

/-- `predicates/attachment.ron` -/
semantic_template Predicate.attachment (side : Semantics.AttachmentSide) (counterpart : (Option Semantics.NounPhrase) := none) (word : (Option Semantics.AttachWord) := none) : Semantics.Predicate :=
  (Semantics.Predicate.attachment side word counterpart)

/-- `predicates/attacking.ron` -/
semantic_template attacking : Semantics.Predicate :=
  (Semantics.Predicate.inCombat Semantics.CombatRelation.attackerOf none)

/-- `predicates/blocked.ron` -/
semantic_template blocked : Semantics.Predicate :=
  (Semantics.Predicate.inCombat Semantics.CombatRelation.blockedBy none)

/-- `predicates/blocking.ron` -/
semantic_template blocking : Semantics.Predicate :=
  (Semantics.Predicate.inCombat Semantics.CombatRelation.blockerOf none)

/-- `predicates/bracketed.ron` -/
semantic_template Predicate.bracketed (predicate : Semantics.Predicate) : Semantics.Predicate :=
  (Semantics.Predicate.bracketed predicate)

/-- `predicates/card.ron` -/
semantic_template card : Semantics.Predicate :=
  Semantics.Predicate.isCard

/-- `predicates/cardIn.ron` -/
semantic_template cardIn (zone : Semantics.ZoneExpr) : Semantics.Predicate :=
  (Semantics.Predicate.and [Semantics.Predicate.isCard, (Semantics.Predicate.inZone zone)])

/-- `predicates/cardOnTheStack.ron` -/
semantic_template cardOnTheStack : Semantics.Predicate :=
  (Semantics.Predicate.and [Semantics.Predicate.isCard, (Semantics.Predicate.and [(Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.stack Semantics.ZoneScope.bare)), (Semantics.Predicate.not (Semantics.Predicate.abilityHead Semantics.AbilityClass.anyOnStack))])])

/-- `predicates/castBy.ron` -/
semantic_template castBy (player : Semantics.NounPhrase) : Semantics.Predicate :=
  (Semantics.Predicate.castBy player none)

/-- `predicates/castFrom.ron` -/
semantic_template castFrom (zone : Semantics.ZoneExpr) : Semantics.Predicate :=
  (Semantics.Predicate.castFrom zone)

/-- `predicates/chapterAbilityOf.ron` -/
semantic_template chapterAbilityOf (saga : Semantics.NounPhrase) : Semantics.Predicate :=
  (Semantics.Predicate.and [(Semantics.Predicate.abilityOf saga), (Semantics.Predicate.compare [Semantics.ProjAxis.chapter] Semantics.Comparator.atLeast (Semantics.Amount.lit 1))])

/-- `predicates/choseExtreme.ron` -/
semantic_template choseExtreme (op : Semantics.AggregateOp) : Semantics.Predicate :=
  (Semantics.Predicate.choseExtreme op)

/-- `predicates/chosenPlayer.ron` -/
semantic_template chosenPlayer : Semantics.Predicate :=
  (Semantics.Predicate.chosenPlayer Semantics.ChoiceRef.theChoice)

/-- `predicates/coinCameUp.ron` -/
semantic_template coinCameUp (face : Semantics.CoinFace) : Semantics.Predicate :=
  (Semantics.Predicate.coinCameUp face)

/-- `predicates/colorCount.ron` -/
semantic_template colorCount (comparator : Semantics.Comparator) (count : Nat) : Semantics.Predicate :=
  (Semantics.Predicate.colorCount comparator count)

/-- `predicates/colorIs.ron` -/
semantic_template colorIs (color : Semantics.Color) : Semantics.Predicate :=
  (Semantics.Predicate.colorIs color)

/-- `predicates/colorOf.ron` -/
semantic_template colorOf (object : Semantics.NounPhrase) : Semantics.ColorTerm :=
  (Semantics.ColorTerm.colorOf object)

/-- `predicates/colorless.ron` -/
semantic_template colorless : Semantics.Predicate :=
  (Semantics.Predicate.colorCount Semantics.Comparator.eq 0)

/-- `predicates/compare.ron` -/
semantic_template compare (comparator : Semantics.Comparator) (bound : Semantics.Amount) (axes : (List Semantics.ProjAxis) := []) : Semantics.Predicate :=
  (Semantics.Predicate.compare axes comparator bound)

/-- `predicates/compareOver.ron` -/
semantic_template compareOver (domain : Semantics.Predicate) (measure : Semantics.Amount) (comparator : Semantics.Comparator) (bound : Semantics.Amount) : Semantics.Predicate :=
  (Semantics.Predicate.compareOver domain measure comparator bound)

/-- `predicates/comparesOwnStat.ron` -/
semantic_template comparesOwnStat (stat : Semantics.Stat) (domain : Semantics.Predicate) (comparator : Semantics.Comparator) (bound : Semantics.Amount) (member : Semantics.NounPhrase := (Semantics.NounPhrase.pro Semantics.Reach.bare Semantics.Plurality.one)) : Semantics.Predicate :=
  (Semantics.Predicate.compareOver domain (Semantics.Amount.statOf (Semantics.ProjAxis.stat stat) member) comparator bound)

/-- `predicates/controlledBy.ron` -/
semantic_template controlledBy (player : Semantics.NounPhrase) : Semantics.Predicate :=
  (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller player)

/-- `predicates/copyOfACard.ron` -/
semantic_template copyOfACard : Semantics.Predicate :=
  Semantics.Predicate.isCopyOfACard

/-- `predicates/counterKindOn.ron` -/
semantic_template counterKindOn (subject : Semantics.NounPhrase) : Semantics.Predicate :=
  (Semantics.Predicate.counterKindOn subject)

/-- `predicates/creature.ron` -/
semantic_template creature : Semantics.Predicate :=
  (Semantics.Predicate.hasType Semantics.CardType.creature)

/-- `predicates/creatureType.ron` -/
semantic_template creatureType (label : String) : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature label)

/-- `predicates/creatureYouControl.ron` -/
semantic_template creatureYouControl : Semantics.Predicate :=
  (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)])

/-- `predicates/creatureYouDontControl.ron` -/
semantic_template creatureYouDontControl : Semantics.Predicate :=
  (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.not (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you))])

/-- `predicates/creatureYourOpponentsControl.ron` -/
semantic_template creatureYourOpponentsControl : Semantics.Predicate :=
  (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller (Semantics.NounPhrase.playerGroup Semantics.PlayerGroupWord.yourOpponents))])

/-- `predicates/currentFace.ron` -/
semantic_template currentFace (side : Semantics.CardFaceSide) : Semantics.Predicate :=
  (Semantics.Predicate.currentFace side)

/-- `predicates/emblem.ron` -/
semantic_template emblem : Semantics.Predicate :=
  Semantics.Predicate.isEmblem

/-- `predicates/enchantment.ron` -/
semantic_template enchantment : Semantics.Predicate :=
  (Semantics.Predicate.hasType Semantics.CardType.enchantment)

/-- `predicates/enchantmentType.ron` -/
semantic_template enchantmentType (label : String) : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.enchantment label)

/-- `predicates/exiledWith.ron` -/
semantic_template exiledWith (source : Semantics.NounPhrase) : Semantics.Predicate :=
  (Semantics.Predicate.exiledWith source)

/-- `predicates/faceDown.ron` -/
semantic_template faceDown : Semantics.Predicate :=
  (Semantics.Predicate.hasStatus Semantics.Status.faceDown)

/-- `predicates/hasCounters.ron` -/
semantic_template hasCounters (kind : (Option Semantics.CounterKind) := none) : Semantics.Predicate :=
  (Semantics.Predicate.hasCounters kind)

/-- `predicates/hasDesignation.ron` -/
semantic_template hasDesignation (designation : Semantics.DesignationLabel) (holder : (Option Semantics.NounPhrase) := none) : Semantics.Predicate :=
  (Semantics.Predicate.hasDesignation designation holder)

/-- `predicates/hasKeyword.ron` -/
semantic_template hasKeyword (keyword : Semantics.KeywordTerm) : Semantics.Predicate :=
  (Semantics.Predicate.hasKeyword keyword)

/-- `predicates/hasPossessor.ron` -/
semantic_template hasPossessor (axis : Semantics.PossessorAxis) (possessor : Semantics.NounPhrase) : Semantics.Predicate :=
  (Semantics.Predicate.hasPossessor axis possessor)

/-- `predicates/hasStatus.ron` -/
semantic_template hasStatus (status : Semantics.Status) : Semantics.Predicate :=
  (Semantics.Predicate.hasStatus status)

/-- `predicates/hasSubtype.ron` -/
semantic_template hasSubtype (subtype : Semantics.Subtype) : Semantics.Predicate :=
  (Semantics.Predicate.hasSubtype subtype)

/-- `predicates/hasSupertype.ron` -/
semantic_template hasSupertype (supertype : Semantics.Supertype) : Semantics.Predicate :=
  (Semantics.Predicate.hasSupertype supertype)

/-- `predicates/hasType.ron` -/
semantic_template hasType (type : Semantics.CardType) : Semantics.Predicate :=
  (Semantics.Predicate.hasType type)

/-- `predicates/historic.ron` -/
semantic_template historic : Semantics.Predicate :=
  (Semantics.Predicate.or [(Semantics.Predicate.hasType Semantics.CardType.artifact), (Semantics.Predicate.hasSupertype Semantics.Supertype.legendary), (Semantics.Predicate.hasSubtype (Semantics.Subtype.of Semantics.CardType.enchantment "Saga"))])

/-- `predicates/inCombat.ron` -/
semantic_template inCombat (relation : Semantics.CombatRelation) (counterpart : (Option Semantics.NounPhrase) := none) : Semantics.Predicate :=
  (Semantics.Predicate.inCombat relation counterpart)

/-- `predicates/inPile.ron` -/
semantic_template inPile (pile : Semantics.NounPhrase) : Semantics.Predicate :=
  (Semantics.Predicate.inPile pile)

/-- `predicates/inZone.ron` -/
semantic_template inZone (zone : Semantics.ZoneExpr) : Semantics.Predicate :=
  (Semantics.Predicate.inZone zone)

/-- `predicates/instant.ron` -/
semantic_template instant : Semantics.Predicate :=
  (Semantics.Predicate.hasType Semantics.CardType.instant)

/-- `predicates/instantOrSorcery.ron` -/
semantic_template instantOrSorcery : Semantics.Predicate :=
  (Semantics.Predicate.or [(Semantics.Predicate.hasType Semantics.CardType.instant), (Semantics.Predicate.hasType Semantics.CardType.sorcery)])

/-- `predicates/isCopyOfACard.ron` -/
semantic_template isCopyOfACard : Semantics.Predicate :=
  Semantics.Predicate.isCopyOfACard

/-- `predicates/isEmblem.ron` -/
semantic_template isEmblem : Semantics.Predicate :=
  Semantics.Predicate.isEmblem

/-- `predicates/isManaAbility.ron` -/
semantic_template isManaAbility : Semantics.Predicate :=
  Semantics.Predicate.isManaAbility

/-- `predicates/isSource.ron` -/
semantic_template isSource : Semantics.Predicate :=
  Semantics.Predicate.isSource

/-- `predicates/isToken.ron` -/
semantic_template isToken : Semantics.Predicate :=
  Semantics.Predicate.isToken

/-- `predicates/land.ron` -/
semantic_template land : Semantics.Predicate :=
  (Semantics.Predicate.hasType Semantics.CardType.land)

/-- `predicates/landType.ron` -/
semantic_template landType (label : String) : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.land label)

/-- `predicates/manaCostHas.ron` -/
semantic_template manaCostHas (symbol : Semantics.ManaSymbol) : Semantics.Predicate :=
  (Semantics.Predicate.manaCostHas symbol)

/-- `predicates/monocolored.ron` -/
semantic_template monocolored : Semantics.Predicate :=
  (Semantics.Predicate.colorCount Semantics.Comparator.eq 1)

/-- `predicates/multicolored.ron` -/
semantic_template multicolored : Semantics.Predicate :=
  (Semantics.Predicate.colorCount Semantics.Comparator.atLeast 2)

/-- `predicates/named.ron` -/
semantic_template named (source : Semantics.NameSource) : Semantics.Predicate :=
  (Semantics.Predicate.named source)

/-- `predicates/nontoken.ron` -/
semantic_template nontoken : Semantics.Predicate :=
  (Semantics.Predicate.not Semantics.Predicate.isToken)

/-- `predicates/not.ron` -/
semantic_template Predicate.not (predicate : Semantics.Predicate) : Semantics.Predicate :=
  (Semantics.Predicate.not predicate)

/-- `predicates/notChosen.ron` -/
semantic_template notChosen : Semantics.Predicate :=
  Semantics.Predicate.notChosen

/-- `predicates/nthCastBy.ron` -/
semantic_template nthCastBy (ordinal : Semantics.Ordinal) (player : Semantics.NounPhrase) (period : Semantics.Period) : Semantics.Predicate :=
  (Semantics.Predicate.castBy player (some (ordinal, period)))

/-- `predicates/ofChosen.ron` -/
semantic_template ofChosen (sort : Semantics.QualitySort) : Semantics.Predicate :=
  (Semantics.Predicate.ofChosen Semantics.ChoiceRef.theChoice sort)

/-- `predicates/ofTheLastChosen.ron` -/
semantic_template ofTheLastChosen (sort : Semantics.QualitySort) : Semantics.Predicate :=
  (Semantics.Predicate.ofChosen Semantics.ChoiceRef.theLatestChoice sort)

/-- `predicates/ofYourChoice.ron` -/
semantic_template ofYourChoice (sort : Semantics.QualitySort) (domain : (Option Semantics.ChoiceDomain) := none) : Semantics.Predicate :=
  (Semantics.Predicate.ofYourChoice sort domain)

/-- `predicates/opponent.ron` -/
semantic_template opponent : Semantics.Predicate :=
  Semantics.Predicate.opponent

/-- `predicates/or.ron` -/
semantic_template Predicate.or (disjuncts : (List Semantics.Predicate) := []) : Semantics.Predicate :=
  (Semantics.Predicate.or disjuncts)

/-- `predicates/other.ron` -/
semantic_template other : Semantics.Predicate :=
  Semantics.Predicate.other

/-- `predicates/otherCreature.ron` -/
semantic_template otherCreature (n : Semantics.NounPhrase) : Semantics.Predicate :=
  (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.otherThan n)])

/-- `predicates/otherCreatureYouControl.ron` -/
semantic_template otherCreatureYouControl (n : Semantics.NounPhrase) : Semantics.Predicate :=
  (Semantics.Predicate.and [(Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you), (Semantics.Predicate.otherThan n)])

/-- `predicates/otherPlayer.ron` -/
semantic_template otherPlayer : Semantics.Predicate :=
  (Semantics.Predicate.and [Semantics.Predicate.anyPlayer, (Semantics.Predicate.otherThan Semantics.NounPhrase.you)])

/-- `predicates/otherThan.ron` -/
semantic_template otherThan (anchor : Semantics.NounPhrase) : Semantics.Predicate :=
  (Semantics.Predicate.otherThan anchor)

/-- `predicates/outlaw.ron` -/
semantic_template outlaw : Semantics.Predicate :=
  (Semantics.Predicate.or [(Semantics.Predicate.hasSubtype (Semantics.Subtype.of Semantics.CardType.creature "Assassin")), (Semantics.Predicate.hasSubtype (Semantics.Subtype.of Semantics.CardType.creature "Mercenary")), (Semantics.Predicate.hasSubtype (Semantics.Subtype.of Semantics.CardType.creature "Pirate")), (Semantics.Predicate.hasSubtype (Semantics.Subtype.of Semantics.CardType.creature "Rogue")), (Semantics.Predicate.hasSubtype (Semantics.Subtype.of Semantics.CardType.creature "Warlock"))])

/-- `predicates/outlawYouControl.ron` -/
semantic_template outlawYouControl : Semantics.Predicate :=
  (Semantics.Predicate.and [(Semantics.Predicate.or [(Semantics.Predicate.hasSubtype (Semantics.Subtype.of Semantics.CardType.creature "Assassin")), (Semantics.Predicate.hasSubtype (Semantics.Subtype.of Semantics.CardType.creature "Mercenary")), (Semantics.Predicate.hasSubtype (Semantics.Subtype.of Semantics.CardType.creature "Pirate")), (Semantics.Predicate.hasSubtype (Semantics.Subtype.of Semantics.CardType.creature "Rogue")), (Semantics.Predicate.hasSubtype (Semantics.Subtype.of Semantics.CardType.creature "Warlock"))]), (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)])

/-- `predicates/ownedBy.ron` -/
semantic_template ownedBy (player : Semantics.NounPhrase) : Semantics.Predicate :=
  (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.owner player)

/-- `predicates/permanent.ron` -/
semantic_template permanent : Semantics.Predicate :=
  (Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare))

/-- `predicates/permanentCard.ron` -/
semantic_template permanentCard : Semantics.Predicate :=
  (Semantics.Predicate.and [Semantics.Predicate.isCard, (Semantics.Predicate.or [(Semantics.Predicate.hasType Semantics.CardType.artifact), (Semantics.Predicate.hasType Semantics.CardType.battle), (Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasType Semantics.CardType.enchantment), (Semantics.Predicate.hasType Semantics.CardType.land), (Semantics.Predicate.hasType Semantics.CardType.planeswalker)])])

/-- `predicates/permanentType.ron` -/
semantic_template permanentType : Semantics.Predicate :=
  (Semantics.Predicate.or [(Semantics.Predicate.hasType Semantics.CardType.artifact), (Semantics.Predicate.hasType Semantics.CardType.battle), (Semantics.Predicate.hasType Semantics.CardType.creature), (Semantics.Predicate.hasType Semantics.CardType.enchantment), (Semantics.Predicate.hasType Semantics.CardType.land), (Semantics.Predicate.hasType Semantics.CardType.planeswalker)])

/-- `predicates/planeswalker.ron` -/
semantic_template planeswalker : Semantics.Predicate :=
  (Semantics.Predicate.hasType Semantics.CardType.planeswalker)

/-- `predicates/planeswalkerType.ron` -/
semantic_template planeswalkerType (label : String) : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker label)

/-- `predicates/quality.ron` -/
semantic_template quality (sort : Semantics.QualitySort) : Semantics.Predicate :=
  (Semantics.Predicate.qualityNoun sort none)

/-- `predicates/qualityFrom.ron` -/
semantic_template qualityFrom (sort : Semantics.QualitySort) (domain : Semantics.ChoiceDomain) : Semantics.Predicate :=
  (Semantics.Predicate.qualityNoun sort (some domain))

/-- `predicates/qualityNoun.ron` -/
semantic_template qualityNoun (sort : Semantics.QualitySort) (domain : (Option Semantics.ChoiceDomain) := none) : Semantics.Predicate :=
  (Semantics.Predicate.qualityNoun sort domain)

/-- `predicates/related.ron` -/
semantic_template related (label : Semantics.RelationLabel) (counterpart : (Option Semantics.NounPhrase) := none) : Semantics.Predicate :=
  (Semantics.Predicate.related label counterpart)

/-- `predicates/shares.ron` -/
semantic_template shares (axis : Semantics.QualitySort) (agreement : Semantics.Agreement) (with_ : Semantics.NounPhrase) : Semantics.Predicate :=
  (Semantics.Predicate.shares axis agreement with_)

/-- `predicates/sorcery.ron` -/
semantic_template sorcery : Semantics.Predicate :=
  (Semantics.Predicate.hasType Semantics.CardType.sorcery)

/-- `predicates/source.ron` -/
semantic_template source : Semantics.Predicate :=
  Semantics.Predicate.isSource

/-- `predicates/spell.ron` -/
semantic_template Predicate.spell : Semantics.Predicate :=
  (Semantics.Predicate.and [(Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.stack Semantics.ZoneScope.bare)), (Semantics.Predicate.not (Semantics.Predicate.abilityHead Semantics.AbilityClass.anyOnStack))])

/-- `predicates/spellType.ron` -/
semantic_template spellType (label : String) : Semantics.Subtype :=
  (Semantics.Subtype.spell label)

/-- `predicates/superlative.ron` -/
semantic_template superlative (op : Semantics.AggregateOp) (axis : Semantics.ProjAxis) (domain : Semantics.Predicate) : Semantics.Predicate :=
  (Semantics.Predicate.superlative op axis domain)

/-- `predicates/tapped.ron` -/
semantic_template tapped : Semantics.Predicate :=
  (Semantics.Predicate.hasStatus Semantics.Status.tapped)

/-- `predicates/targets.ron` -/
semantic_template targets (subject : Semantics.NounPhrase) (extent : Semantics.TargetExtent) : Semantics.Predicate :=
  (Semantics.Predicate.targets subject extent)

/-- `predicates/targetsOnly.ron` -/
semantic_template targetsOnly (subject : Semantics.NounPhrase) : Semantics.Predicate :=
  (Semantics.Predicate.targets subject Semantics.TargetExtent.only)

/-- `predicates/thatColor.ron` -/
semantic_template thatColor : Semantics.ColorTerm :=
  (Semantics.ColorTerm.chosen Semantics.ChoiceRef.theChoice)

/-- `predicates/theLastChosenPlayer.ron` -/
semantic_template theLastChosenPlayer : Semantics.Predicate :=
  (Semantics.Predicate.chosenPlayer Semantics.ChoiceRef.theLatestChoice)

/-- `predicates/tokenOnTheBattlefield.ron` -/
semantic_template tokenOnTheBattlefield : Semantics.Predicate :=
  (Semantics.Predicate.and [Semantics.Predicate.isToken, (Semantics.Predicate.inZone (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare))])

/-- `predicates/unblocked.ron` -/
semantic_template unblocked : Semantics.Predicate :=
  (Semantics.Predicate.not (Semantics.Predicate.inCombat Semantics.CombatRelation.blockedBy none))

/-- `predicates/untapped.ron` -/
semantic_template untapped : Semantics.Predicate :=
  (Semantics.Predicate.hasStatus Semantics.Status.untapped)

/-- `predicates/wasCast.ron` -/
semantic_template wasCast : Semantics.Predicate :=
  (Semantics.Predicate.did (Semantics.DidClause.mk Semantics.Subject.any (Semantics.Instruction.procedure Semantics.Procedure.cast Semantics.NounPhrase.which Semantics.PlayPayment.itsOwnCost none none none) Semantics.LookbackSpan.thisWay))

/-- `predicates/withMostVotes.ron` -/
semantic_template withMostVotes : Semantics.Predicate :=
  Semantics.Predicate.withMostVotes

/-- `predicates/youControl.ron` -/
semantic_template youControl : Semantics.Predicate :=
  (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.controller Semantics.NounPhrase.you)

/-- `predicates/youOwn.ron` -/
semantic_template youOwn : Semantics.Predicate :=
  (Semantics.Predicate.hasPossessor Semantics.PossessorAxis.owner Semantics.NounPhrase.you)

/-- `pronouns/it.ron` -/
semantic_template it : Semantics.NounPhrase :=
  (Semantics.NounPhrase.pro Semantics.Reach.bare Semantics.Plurality.one)

/-- `pronouns/itAsToken.ron` -/
semantic_template itAsToken : Semantics.NounPhrase :=
  (Semantics.NounPhrase.pro Semantics.Reach.tokenBorn Semantics.Plurality.one)

/-- `pronouns/itCard.ron` -/
semantic_template itCard : Semantics.NounPhrase :=
  (Semantics.NounPhrase.pro (Semantics.Reach.atSlot Semantics.SlotCarrier.card) Semantics.Plurality.one)

/-- `pronouns/itPermanent.ron` -/
semantic_template itPermanent : Semantics.NounPhrase :=
  (Semantics.NounPhrase.pro (Semantics.Reach.atSlot Semantics.SlotCarrier.permanent) Semantics.Plurality.one)

/-- `pronouns/ownMember.ron` -/
semantic_template ownMember : Semantics.NounPhrase :=
  (Semantics.NounPhrase.pro Semantics.Reach.bare Semantics.Plurality.one)

/-- `pronouns/that.ron` -/
semantic_template that (w : Semantics.NounWord) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.pro (Semantics.Reach.word w) Semantics.Plurality.one)

/-- `pronouns/thatJoin.ron` -/
semantic_template thatJoin : Semantics.NounPhrase :=
  (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.join) Semantics.Plurality.one)

/-- `pronouns/theVerbed.ron` -/
semantic_template theVerbed (verb : Semantics.Deed) (plurality : Semantics.Plurality) (word : (Option Semantics.NounWord) := none) (marking : Semantics.VerbedMarking := Semantics.VerbedMarking.attributive) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.pro (Semantics.Reach.verbed verb word marking) plurality)

/-- `pronouns/them.ron` -/
semantic_template them : Semantics.NounPhrase :=
  (Semantics.NounPhrase.pro Semantics.Reach.bare Semantics.Plurality.many)

/-- `pronouns/they.ron` -/
semantic_template they : Semantics.NounPhrase :=
  (Semantics.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.player) Semantics.Plurality.one)

/-- `pronouns/those.ron` -/
semantic_template those (w : Semantics.NounWord) : Semantics.NounPhrase :=
  (Semantics.NounPhrase.pro (Semantics.Reach.word w) Semantics.Plurality.many)

/-- `quantities/anyNumber.ron` -/
semantic_template anyNumber : Semantics.Quantity :=
  (Semantics.Quantity.range none none)

/-- `quantities/atLeast.ron` -/
semantic_template atLeast (n : Nat) : Semantics.Quantity :=
  (Semantics.Quantity.range (some n) none)

/-- `quantities/exactly.ron` -/
semantic_template exactly (n : Nat) : Semantics.Quantity :=
  (Semantics.Quantity.range (some n) (some n))

/-- `quantities/exactlyOf.ron` -/
semantic_template exactlyOf (amount : Semantics.Amount) : Semantics.Quantity :=
  (Semantics.Quantity.exactlyOf amount)

/-- `quantities/oneThrough.ron` -/
semantic_template oneThrough (n : Nat) : Semantics.Quantity :=
  (Semantics.Quantity.range (some 1) (some n))

/-- `quantities/range.ron` -/
semantic_template range (low : (Option Nat) := none) (high : (Option Nat) := none) : Semantics.Quantity :=
  (Semantics.Quantity.range low high)

/-- `quantities/upTo.ron` -/
semantic_template Quantity.upTo (n : Nat) : Semantics.Quantity :=
  (Semantics.Quantity.range none (some n))

/-- `quantities/upToOf.ron` -/
semantic_template upToOf (amount : Semantics.Amount) : Semantics.Quantity :=
  (Semantics.Quantity.upToOf amount)

/-- `quantities/worth.ron` -/
semantic_template worth (n : Nat) : Semantics.Quantity :=
  (Semantics.Quantity.worth n)

/-- `relations/encodedOn.ron` -/
semantic_definition encodedOn : Semantics.RelationLabel :=
  (Semantics.RelationLabel.named "encodedOn")

/-- `relations/haunting.ron` -/
semantic_definition haunting : Semantics.RelationLabel :=
  (Semantics.RelationLabel.named "haunting")

/-- `relations/paired.ron` -/
semantic_definition paired : Semantics.RelationLabel :=
  (Semantics.RelationLabel.named "paired")

/-- `subtypes/artifact/blood.ron` -/
semantic_definition blood : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.artifact "Blood")

/-- `subtypes/artifact/bobblehead.ron` -/
semantic_definition bobblehead : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.artifact "Bobblehead")

/-- `subtypes/artifact/book.ron` -/
semantic_definition book : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.artifact "Book")

/-- `subtypes/artifact/clue.ron` -/
semantic_definition clue : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.artifact "Clue")

/-- `subtypes/artifact/contraption.ron` -/
semantic_definition contraption : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.artifact "Contraption")

/-- `subtypes/artifact/equipment.ron` -/
semantic_definition equipment : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.artifact "Equipment")

/-- `subtypes/artifact/food.ron` -/
semantic_definition food : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.artifact "Food")

/-- `subtypes/artifact/fortification.ron` -/
semantic_definition fortification : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.artifact "Fortification")

/-- `subtypes/artifact/gold.ron` -/
semantic_definition gold : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.artifact "Gold")

/-- `subtypes/artifact/incubator.ron` -/
semantic_definition incubator : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.artifact "Incubator")

/-- `subtypes/artifact/infinity.ron` -/
semantic_definition Subtype.infinity : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.artifact "Infinity")

/-- `subtypes/artifact/junk.ron` -/
semantic_definition junk : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.artifact "Junk")

/-- `subtypes/artifact/lander.ron` -/
semantic_definition lander : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.artifact "Lander")

/-- `subtypes/artifact/map.ron` -/
semantic_definition map : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.artifact "Map")

/-- `subtypes/artifact/mutagen.ron` -/
semantic_definition mutagen : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.artifact "Mutagen")

/-- `subtypes/artifact/powerstone.ron` -/
semantic_definition powerstone : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.artifact "Powerstone")

/-- `subtypes/artifact/spacecraft.ron` -/
semantic_definition spacecraft : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.artifact "Spacecraft")

/-- `subtypes/artifact/stone.ron` -/
semantic_definition stone : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.artifact "Stone")

/-- `subtypes/artifact/treasure.ron` -/
semantic_definition treasure : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.artifact "Treasure")

/-- `subtypes/artifact/vehicle.ron` -/
semantic_definition vehicle : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.artifact "Vehicle")

/-- `subtypes/artifact/vibranium.ron` -/
semantic_definition vibranium : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.artifact "Vibranium")

/-- `subtypes/battle/siege.ron` -/
semantic_definition siege : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.battle "Siege")

/-- `subtypes/creature/advisor.ron` -/
semantic_definition advisor : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Advisor")

/-- `subtypes/creature/aetherborn.ron` -/
semantic_definition aetherborn : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Aetherborn")

/-- `subtypes/creature/alien.ron` -/
semantic_definition alien : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Alien")

/-- `subtypes/creature/ally.ron` -/
semantic_definition ally : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Ally")

/-- `subtypes/creature/angel.ron` -/
semantic_definition angel : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Angel")

/-- `subtypes/creature/antelope.ron` -/
semantic_definition antelope : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Antelope")

/-- `subtypes/creature/ape.ron` -/
semantic_definition ape : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Ape")

/-- `subtypes/creature/archer.ron` -/
semantic_definition archer : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Archer")

/-- `subtypes/creature/archon.ron` -/
semantic_definition archon : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Archon")

/-- `subtypes/creature/armadillo.ron` -/
semantic_definition armadillo : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Armadillo")

/-- `subtypes/creature/army.ron` -/
semantic_definition army : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Army")

/-- `subtypes/creature/artificer.ron` -/
semantic_definition artificer : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Artificer")

/-- `subtypes/creature/assassin.ron` -/
semantic_definition assassin : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Assassin")

/-- `subtypes/creature/assemblyWorker.ron` -/
semantic_definition assemblyWorker : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Assembly-Worker")

/-- `subtypes/creature/astartes.ron` -/
semantic_definition astartes : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Astartes")

/-- `subtypes/creature/atog.ron` -/
semantic_definition atog : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Atog")

/-- `subtypes/creature/aurochs.ron` -/
semantic_definition aurochs : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Aurochs")

/-- `subtypes/creature/avatar.ron` -/
semantic_definition avatar : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Avatar")

/-- `subtypes/creature/azra.ron` -/
semantic_definition azra : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Azra")

/-- `subtypes/creature/badger.ron` -/
semantic_definition badger : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Badger")

/-- `subtypes/creature/balloon.ron` -/
semantic_definition balloon : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Balloon")

/-- `subtypes/creature/barbarian.ron` -/
semantic_definition barbarian : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Barbarian")

/-- `subtypes/creature/bard.ron` -/
semantic_definition bard : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Bard")

/-- `subtypes/creature/basilisk.ron` -/
semantic_definition basilisk : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Basilisk")

/-- `subtypes/creature/bat.ron` -/
semantic_definition bat : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Bat")

/-- `subtypes/creature/bear.ron` -/
semantic_definition bear : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Bear")

/-- `subtypes/creature/beast.ron` -/
semantic_definition beast : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Beast")

/-- `subtypes/creature/beaver.ron` -/
semantic_definition beaver : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Beaver")

/-- `subtypes/creature/beeble.ron` -/
semantic_definition beeble : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Beeble")

/-- `subtypes/creature/beholder.ron` -/
semantic_definition beholder : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Beholder")

/-- `subtypes/creature/berserker.ron` -/
semantic_definition berserker : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Berserker")

/-- `subtypes/creature/bird.ron` -/
semantic_definition bird : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Bird")

/-- `subtypes/creature/bison.ron` -/
semantic_definition bison : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Bison")

/-- `subtypes/creature/blinkmoth.ron` -/
semantic_definition blinkmoth : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Blinkmoth")

/-- `subtypes/creature/boar.ron` -/
semantic_definition boar : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Boar")

/-- `subtypes/creature/bringer.ron` -/
semantic_definition bringer : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Bringer")

/-- `subtypes/creature/brushwagg.ron` -/
semantic_definition brushwagg : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Brushwagg")

/-- `subtypes/creature/camarid.ron` -/
semantic_definition camarid : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Camarid")

/-- `subtypes/creature/camel.ron` -/
semantic_definition camel : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Camel")

/-- `subtypes/creature/capybara.ron` -/
semantic_definition capybara : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Capybara")

/-- `subtypes/creature/caribou.ron` -/
semantic_definition caribou : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Caribou")

/-- `subtypes/creature/carrier.ron` -/
semantic_definition carrier : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Carrier")

/-- `subtypes/creature/cat.ron` -/
semantic_definition cat : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Cat")

/-- `subtypes/creature/centaur.ron` -/
semantic_definition centaur : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Centaur")

/-- `subtypes/creature/child.ron` -/
semantic_definition child : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Child")

/-- `subtypes/creature/chimera.ron` -/
semantic_definition chimera : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Chimera")

/-- `subtypes/creature/citizen.ron` -/
semantic_definition citizen : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Citizen")

/-- `subtypes/creature/cleric.ron` -/
semantic_definition cleric : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Cleric")

/-- `subtypes/creature/clown.ron` -/
semantic_definition clown : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Clown")

/-- `subtypes/creature/cockatrice.ron` -/
semantic_definition cockatrice : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Cockatrice")

/-- `subtypes/creature/construct.ron` -/
semantic_definition construct : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Construct")

/-- `subtypes/creature/coward.ron` -/
semantic_definition coward : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Coward")

/-- `subtypes/creature/coyote.ron` -/
semantic_definition coyote : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Coyote")

/-- `subtypes/creature/crab.ron` -/
semantic_definition crab : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Crab")

/-- `subtypes/creature/crocodile.ron` -/
semantic_definition crocodile : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Crocodile")

/-- `subtypes/creature/ctan.ron` -/
semantic_definition ctan : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "C'tan")

/-- `subtypes/creature/custodes.ron` -/
semantic_definition custodes : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Custodes")

/-- `subtypes/creature/cyberman.ron` -/
semantic_definition cyberman : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Cyberman")

/-- `subtypes/creature/cyclops.ron` -/
semantic_definition cyclops : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Cyclops")

/-- `subtypes/creature/dalek.ron` -/
semantic_definition dalek : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Dalek")

/-- `subtypes/creature/dauthi.ron` -/
semantic_definition dauthi : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Dauthi")

/-- `subtypes/creature/demigod.ron` -/
semantic_definition demigod : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Demigod")

/-- `subtypes/creature/demon.ron` -/
semantic_definition demon : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Demon")

/-- `subtypes/creature/deserter.ron` -/
semantic_definition deserter : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Deserter")

/-- `subtypes/creature/detective.ron` -/
semantic_definition detective : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Detective")

/-- `subtypes/creature/devil.ron` -/
semantic_definition devil : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Devil")

/-- `subtypes/creature/dinosaur.ron` -/
semantic_definition dinosaur : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Dinosaur")

/-- `subtypes/creature/djinn.ron` -/
semantic_definition djinn : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Djinn")

/-- `subtypes/creature/doctor.ron` -/
semantic_definition doctor : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Doctor")

/-- `subtypes/creature/dog.ron` -/
semantic_definition dog : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Dog")

/-- `subtypes/creature/dragon.ron` -/
semantic_definition dragon : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Dragon")

/-- `subtypes/creature/drake.ron` -/
semantic_definition drake : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Drake")

/-- `subtypes/creature/dreadnought.ron` -/
semantic_definition dreadnought : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Dreadnought")

/-- `subtypes/creature/drix.ron` -/
semantic_definition drix : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Drix")

/-- `subtypes/creature/drone.ron` -/
semantic_definition drone : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Drone")

/-- `subtypes/creature/druid.ron` -/
semantic_definition druid : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Druid")

/-- `subtypes/creature/dryad.ron` -/
semantic_definition dryad : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Dryad")

/-- `subtypes/creature/dwarf.ron` -/
semantic_definition dwarf : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Dwarf")

/-- `subtypes/creature/echidna.ron` -/
semantic_definition echidna : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Echidna")

/-- `subtypes/creature/efreet.ron` -/
semantic_definition efreet : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Efreet")

/-- `subtypes/creature/egg.ron` -/
semantic_definition egg : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Egg")

/-- `subtypes/creature/elder.ron` -/
semantic_definition elder : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Elder")

/-- `subtypes/creature/eldrazi.ron` -/
semantic_definition eldrazi : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Eldrazi")

/-- `subtypes/creature/elemental.ron` -/
semantic_definition elemental : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Elemental")

/-- `subtypes/creature/elephant.ron` -/
semantic_definition elephant : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Elephant")

/-- `subtypes/creature/elf.ron` -/
semantic_definition elf : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Elf")

/-- `subtypes/creature/elk.ron` -/
semantic_definition elk : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Elk")

/-- `subtypes/creature/employee.ron` -/
semantic_definition employee : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Employee")

/-- `subtypes/creature/eternal.ron` -/
semantic_definition eternal : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Eternal")

/-- `subtypes/creature/eye.ron` -/
semantic_definition eye : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Eye")

/-- `subtypes/creature/faerie.ron` -/
semantic_definition faerie : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Faerie")

/-- `subtypes/creature/ferret.ron` -/
semantic_definition ferret : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Ferret")

/-- `subtypes/creature/fish.ron` -/
semantic_definition fish : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Fish")

/-- `subtypes/creature/flagbearer.ron` -/
semantic_definition flagbearer : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Flagbearer")

/-- `subtypes/creature/fox.ron` -/
semantic_definition fox : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Fox")

/-- `subtypes/creature/fractal.ron` -/
semantic_definition fractal : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Fractal")

/-- `subtypes/creature/frog.ron` -/
semantic_definition frog : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Frog")

/-- `subtypes/creature/fungus.ron` -/
semantic_definition fungus : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Fungus")

/-- `subtypes/creature/gamer.ron` -/
semantic_definition gamer : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Gamer")

/-- `subtypes/creature/gamma.ron` -/
semantic_definition gamma : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Gamma")

/-- `subtypes/creature/gargoyle.ron` -/
semantic_definition gargoyle : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Gargoyle")

/-- `subtypes/creature/germ.ron` -/
semantic_definition germ : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Germ")

/-- `subtypes/creature/giant.ron` -/
semantic_definition giant : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Giant")

/-- `subtypes/creature/giraffe.ron` -/
semantic_definition giraffe : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Giraffe")

/-- `subtypes/creature/gith.ron` -/
semantic_definition gith : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Gith")

/-- `subtypes/creature/glimmer.ron` -/
semantic_definition glimmer : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Glimmer")

/-- `subtypes/creature/gnoll.ron` -/
semantic_definition gnoll : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Gnoll")

/-- `subtypes/creature/gnome.ron` -/
semantic_definition gnome : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Gnome")

/-- `subtypes/creature/goat.ron` -/
semantic_definition goat : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Goat")

/-- `subtypes/creature/goblin.ron` -/
semantic_definition goblin : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Goblin")

/-- `subtypes/creature/god.ron` -/
semantic_definition god : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "God")

/-- `subtypes/creature/golem.ron` -/
semantic_definition golem : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Golem")

/-- `subtypes/creature/gorgon.ron` -/
semantic_definition gorgon : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Gorgon")

/-- `subtypes/creature/graveborn.ron` -/
semantic_definition graveborn : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Graveborn")

/-- `subtypes/creature/gremlin.ron` -/
semantic_definition gremlin : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Gremlin")

/-- `subtypes/creature/griffin.ron` -/
semantic_definition griffin : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Griffin")

/-- `subtypes/creature/guest.ron` -/
semantic_definition guest : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Guest")

/-- `subtypes/creature/hag.ron` -/
semantic_definition hag : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Hag")

/-- `subtypes/creature/halfling.ron` -/
semantic_definition halfling : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Halfling")

/-- `subtypes/creature/hamster.ron` -/
semantic_definition hamster : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Hamster")

/-- `subtypes/creature/harpy.ron` -/
semantic_definition harpy : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Harpy")

/-- `subtypes/creature/hedgehog.ron` -/
semantic_definition hedgehog : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Hedgehog")

/-- `subtypes/creature/hellion.ron` -/
semantic_definition hellion : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Hellion")

/-- `subtypes/creature/hero.ron` -/
semantic_definition hero : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Hero")

/-- `subtypes/creature/hippo.ron` -/
semantic_definition hippo : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Hippo")

/-- `subtypes/creature/hippogriff.ron` -/
semantic_definition hippogriff : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Hippogriff")

/-- `subtypes/creature/homarid.ron` -/
semantic_definition homarid : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Homarid")

/-- `subtypes/creature/homunculus.ron` -/
semantic_definition homunculus : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Homunculus")

/-- `subtypes/creature/horror.ron` -/
semantic_definition horror : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Horror")

/-- `subtypes/creature/horse.ron` -/
semantic_definition horse : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Horse")

/-- `subtypes/creature/human.ron` -/
semantic_definition human : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Human")

/-- `subtypes/creature/hydra.ron` -/
semantic_definition hydra : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Hydra")

/-- `subtypes/creature/hyena.ron` -/
semantic_definition hyena : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Hyena")

/-- `subtypes/creature/illusion.ron` -/
semantic_definition illusion : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Illusion")

/-- `subtypes/creature/imp.ron` -/
semantic_definition imp : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Imp")

/-- `subtypes/creature/incarnation.ron` -/
semantic_definition incarnation : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Incarnation")

/-- `subtypes/creature/inhuman.ron` -/
semantic_definition inhuman : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Inhuman")

/-- `subtypes/creature/inkling.ron` -/
semantic_definition inkling : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Inkling")

/-- `subtypes/creature/inquisitor.ron` -/
semantic_definition inquisitor : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Inquisitor")

/-- `subtypes/creature/insect.ron` -/
semantic_definition insect : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Insect")

/-- `subtypes/creature/jackal.ron` -/
semantic_definition jackal : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Jackal")

/-- `subtypes/creature/jellyfish.ron` -/
semantic_definition jellyfish : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Jellyfish")

/-- `subtypes/creature/juggernaut.ron` -/
semantic_definition juggernaut : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Juggernaut")

/-- `subtypes/creature/kangaroo.ron` -/
semantic_definition kangaroo : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Kangaroo")

/-- `subtypes/creature/kavu.ron` -/
semantic_definition kavu : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Kavu")

/-- `subtypes/creature/kirin.ron` -/
semantic_definition kirin : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Kirin")

/-- `subtypes/creature/kithkin.ron` -/
semantic_definition kithkin : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Kithkin")

/-- `subtypes/creature/knight.ron` -/
semantic_definition knight : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Knight")

/-- `subtypes/creature/kobold.ron` -/
semantic_definition kobold : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Kobold")

/-- `subtypes/creature/kor.ron` -/
semantic_definition kor : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Kor")

/-- `subtypes/creature/kraken.ron` -/
semantic_definition kraken : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Kraken")

/-- `subtypes/creature/kree.ron` -/
semantic_definition kree : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Kree")

/-- `subtypes/creature/lamia.ron` -/
semantic_definition lamia : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Lamia")

/-- `subtypes/creature/lammasu.ron` -/
semantic_definition lammasu : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Lammasu")

/-- `subtypes/creature/leech.ron` -/
semantic_definition leech : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Leech")

/-- `subtypes/creature/lemur.ron` -/
semantic_definition lemur : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Lemur")

/-- `subtypes/creature/leviathan.ron` -/
semantic_definition leviathan : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Leviathan")

/-- `subtypes/creature/lhurgoyf.ron` -/
semantic_definition lhurgoyf : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Lhurgoyf")

/-- `subtypes/creature/licid.ron` -/
semantic_definition licid : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Licid")

/-- `subtypes/creature/lizard.ron` -/
semantic_definition lizard : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Lizard")

/-- `subtypes/creature/llama.ron` -/
semantic_definition llama : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Llama")

/-- `subtypes/creature/lobster.ron` -/
semantic_definition lobster : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Lobster")

/-- `subtypes/creature/manticore.ron` -/
semantic_definition manticore : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Manticore")

/-- `subtypes/creature/masticore.ron` -/
semantic_definition masticore : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Masticore")

/-- `subtypes/creature/mercenary.ron` -/
semantic_definition mercenary : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Mercenary")

/-- `subtypes/creature/merfolk.ron` -/
semantic_definition merfolk : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Merfolk")

/-- `subtypes/creature/metathran.ron` -/
semantic_definition metathran : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Metathran")

/-- `subtypes/creature/minion.ron` -/
semantic_definition minion : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Minion")

/-- `subtypes/creature/minotaur.ron` -/
semantic_definition minotaur : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Minotaur")

/-- `subtypes/creature/mite.ron` -/
semantic_definition mite : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Mite")

/-- `subtypes/creature/mole.ron` -/
semantic_definition mole : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Mole")

/-- `subtypes/creature/monger.ron` -/
semantic_definition monger : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Monger")

/-- `subtypes/creature/mongoose.ron` -/
semantic_definition mongoose : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Mongoose")

/-- `subtypes/creature/monk.ron` -/
semantic_definition monk : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Monk")

/-- `subtypes/creature/monkey.ron` -/
semantic_definition monkey : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Monkey")

/-- `subtypes/creature/moogle.ron` -/
semantic_definition moogle : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Moogle")

/-- `subtypes/creature/moonfolk.ron` -/
semantic_definition moonfolk : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Moonfolk")

/-- `subtypes/creature/mount.ron` -/
semantic_definition mount : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Mount")

/-- `subtypes/creature/mouse.ron` -/
semantic_definition mouse : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Mouse")

/-- `subtypes/creature/mutant.ron` -/
semantic_definition mutant : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Mutant")

/-- `subtypes/creature/myr.ron` -/
semantic_definition myr : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Myr")

/-- `subtypes/creature/mystic.ron` -/
semantic_definition mystic : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Mystic")

/-- `subtypes/creature/nautilus.ron` -/
semantic_definition nautilus : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Nautilus")

/-- `subtypes/creature/necron.ron` -/
semantic_definition necron : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Necron")

/-- `subtypes/creature/nephilim.ron` -/
semantic_definition nephilim : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Nephilim")

/-- `subtypes/creature/nightmare.ron` -/
semantic_definition nightmare : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Nightmare")

/-- `subtypes/creature/nightstalker.ron` -/
semantic_definition nightstalker : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Nightstalker")

/-- `subtypes/creature/ninja.ron` -/
semantic_definition ninja : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Ninja")

/-- `subtypes/creature/noble.ron` -/
semantic_definition noble : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Noble")

/-- `subtypes/creature/noggle.ron` -/
semantic_definition noggle : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Noggle")

/-- `subtypes/creature/nomad.ron` -/
semantic_definition nomad : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Nomad")

/-- `subtypes/creature/nymph.ron` -/
semantic_definition nymph : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Nymph")

/-- `subtypes/creature/octopus.ron` -/
semantic_definition octopus : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Octopus")

/-- `subtypes/creature/ogre.ron` -/
semantic_definition ogre : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Ogre")

/-- `subtypes/creature/ooze.ron` -/
semantic_definition ooze : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Ooze")

/-- `subtypes/creature/orb.ron` -/
semantic_definition orb : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Orb")

/-- `subtypes/creature/orc.ron` -/
semantic_definition orc : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Orc")

/-- `subtypes/creature/orgg.ron` -/
semantic_definition orgg : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Orgg")

/-- `subtypes/creature/otter.ron` -/
semantic_definition otter : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Otter")

/-- `subtypes/creature/ouphe.ron` -/
semantic_definition ouphe : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Ouphe")

/-- `subtypes/creature/ox.ron` -/
semantic_definition ox : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Ox")

/-- `subtypes/creature/oyster.ron` -/
semantic_definition oyster : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Oyster")

/-- `subtypes/creature/pangolin.ron` -/
semantic_definition pangolin : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Pangolin")

/-- `subtypes/creature/peasant.ron` -/
semantic_definition peasant : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Peasant")

/-- `subtypes/creature/pegasus.ron` -/
semantic_definition pegasus : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Pegasus")

/-- `subtypes/creature/pentavite.ron` -/
semantic_definition pentavite : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Pentavite")

/-- `subtypes/creature/performer.ron` -/
semantic_definition performer : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Performer")

/-- `subtypes/creature/pest.ron` -/
semantic_definition pest : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Pest")

/-- `subtypes/creature/phelddagrif.ron` -/
semantic_definition phelddagrif : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Phelddagrif")

/-- `subtypes/creature/phoenix.ron` -/
semantic_definition phoenix : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Phoenix")

/-- `subtypes/creature/phyrexian.ron` -/
semantic_definition phyrexian : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Phyrexian")

/-- `subtypes/creature/pilot.ron` -/
semantic_definition pilot : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Pilot")

/-- `subtypes/creature/pincher.ron` -/
semantic_definition pincher : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Pincher")

/-- `subtypes/creature/pirate.ron` -/
semantic_definition pirate : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Pirate")

/-- `subtypes/creature/plant.ron` -/
semantic_definition plant : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Plant")

/-- `subtypes/creature/platypus.ron` -/
semantic_definition platypus : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Platypus")

/-- `subtypes/creature/porcupine.ron` -/
semantic_definition porcupine : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Porcupine")

/-- `subtypes/creature/possum.ron` -/
semantic_definition possum : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Possum")

/-- `subtypes/creature/praetor.ron` -/
semantic_definition praetor : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Praetor")

/-- `subtypes/creature/primarch.ron` -/
semantic_definition primarch : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Primarch")

/-- `subtypes/creature/prism.ron` -/
semantic_definition prism : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Prism")

/-- `subtypes/creature/processor.ron` -/
semantic_definition processor : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Processor")

/-- `subtypes/creature/qu.ron` -/
semantic_definition qu : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Qu")

/-- `subtypes/creature/rabbit.ron` -/
semantic_definition rabbit : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Rabbit")

/-- `subtypes/creature/raccoon.ron` -/
semantic_definition raccoon : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Raccoon")

/-- `subtypes/creature/ranger.ron` -/
semantic_definition ranger : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Ranger")

/-- `subtypes/creature/rat.ron` -/
semantic_definition rat : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Rat")

/-- `subtypes/creature/rebel.ron` -/
semantic_definition rebel : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Rebel")

/-- `subtypes/creature/reflection.ron` -/
semantic_definition reflection : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Reflection")

/-- `subtypes/creature/rhino.ron` -/
semantic_definition rhino : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Rhino")

/-- `subtypes/creature/rigger.ron` -/
semantic_definition rigger : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Rigger")

/-- `subtypes/creature/robot.ron` -/
semantic_definition robot : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Robot")

/-- `subtypes/creature/rogue.ron` -/
semantic_definition rogue : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Rogue")

/-- `subtypes/creature/sable.ron` -/
semantic_definition sable : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Sable")

/-- `subtypes/creature/salamander.ron` -/
semantic_definition salamander : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Salamander")

/-- `subtypes/creature/samurai.ron` -/
semantic_definition samurai : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Samurai")

/-- `subtypes/creature/sand.ron` -/
semantic_definition sand : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Sand")

/-- `subtypes/creature/saproling.ron` -/
semantic_definition saproling : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Saproling")

/-- `subtypes/creature/satyr.ron` -/
semantic_definition satyr : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Satyr")

/-- `subtypes/creature/scarecrow.ron` -/
semantic_definition scarecrow : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Scarecrow")

/-- `subtypes/creature/scientist.ron` -/
semantic_definition scientist : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Scientist")

/-- `subtypes/creature/scion.ron` -/
semantic_definition scion : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Scion")

/-- `subtypes/creature/scorpion.ron` -/
semantic_definition scorpion : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Scorpion")

/-- `subtypes/creature/scout.ron` -/
semantic_definition scout : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Scout")

/-- `subtypes/creature/sculpture.ron` -/
semantic_definition sculpture : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Sculpture")

/-- `subtypes/creature/seal.ron` -/
semantic_definition seal_ : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Seal")

/-- `subtypes/creature/serf.ron` -/
semantic_definition serf : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Serf")

/-- `subtypes/creature/serpent.ron` -/
semantic_definition serpent : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Serpent")

/-- `subtypes/creature/servo.ron` -/
semantic_definition servo : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Servo")

/-- `subtypes/creature/shade.ron` -/
semantic_definition shade : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Shade")

/-- `subtypes/creature/shaman.ron` -/
semantic_definition shaman : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Shaman")

/-- `subtypes/creature/shapeshifter.ron` -/
semantic_definition shapeshifter : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Shapeshifter")

/-- `subtypes/creature/shark.ron` -/
semantic_definition shark : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Shark")

/-- `subtypes/creature/sheep.ron` -/
semantic_definition sheep : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Sheep")

/-- `subtypes/creature/shiar.ron` -/
semantic_definition shiar : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Shi'ar")

/-- `subtypes/creature/siren.ron` -/
semantic_definition siren : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Siren")

/-- `subtypes/creature/skeleton.ron` -/
semantic_definition skeleton : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Skeleton")

/-- `subtypes/creature/skrull.ron` -/
semantic_definition skrull : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Skrull")

/-- `subtypes/creature/skunk.ron` -/
semantic_definition skunk : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Skunk")

/-- `subtypes/creature/slith.ron` -/
semantic_definition slith : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Slith")

/-- `subtypes/creature/sliver.ron` -/
semantic_definition sliver : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Sliver")

/-- `subtypes/creature/sloth.ron` -/
semantic_definition sloth : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Sloth")

/-- `subtypes/creature/slug.ron` -/
semantic_definition slug : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Slug")

/-- `subtypes/creature/snail.ron` -/
semantic_definition snail : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Snail")

/-- `subtypes/creature/snake.ron` -/
semantic_definition snake : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Snake")

/-- `subtypes/creature/soldier.ron` -/
semantic_definition soldier : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Soldier")

/-- `subtypes/creature/soltari.ron` -/
semantic_definition soltari : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Soltari")

/-- `subtypes/creature/sorcerer.ron` -/
semantic_definition sorcerer : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Sorcerer")

/-- `subtypes/creature/spawn.ron` -/
semantic_definition spawn : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Spawn")

/-- `subtypes/creature/specter.ron` -/
semantic_definition specter : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Specter")

/-- `subtypes/creature/spellshaper.ron` -/
semantic_definition spellshaper : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Spellshaper")

/-- `subtypes/creature/sphinx.ron` -/
semantic_definition sphinx : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Sphinx")

/-- `subtypes/creature/spider.ron` -/
semantic_definition spider : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Spider")

/-- `subtypes/creature/spike.ron` -/
semantic_definition spike : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Spike")

/-- `subtypes/creature/spirit.ron` -/
semantic_definition spirit : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Spirit")

/-- `subtypes/creature/splinter.ron` -/
semantic_definition splinter : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Splinter")

/-- `subtypes/creature/sponge.ron` -/
semantic_definition sponge : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Sponge")

/-- `subtypes/creature/spy.ron` -/
semantic_definition spy : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Spy")

/-- `subtypes/creature/squid.ron` -/
semantic_definition squid : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Squid")

/-- `subtypes/creature/squirrel.ron` -/
semantic_definition squirrel : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Squirrel")

/-- `subtypes/creature/starfish.ron` -/
semantic_definition starfish : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Starfish")

/-- `subtypes/creature/surrakar.ron` -/
semantic_definition surrakar : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Surrakar")

/-- `subtypes/creature/survivor.ron` -/
semantic_definition survivor : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Survivor")

/-- `subtypes/creature/symbiote.ron` -/
semantic_definition symbiote : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Symbiote")

/-- `subtypes/creature/synth.ron` -/
semantic_definition synth : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Synth")

/-- `subtypes/creature/tentacle.ron` -/
semantic_definition tentacle : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Tentacle")

/-- `subtypes/creature/tetravite.ron` -/
semantic_definition tetravite : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Tetravite")

/-- `subtypes/creature/thalakos.ron` -/
semantic_definition thalakos : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Thalakos")

/-- `subtypes/creature/thopter.ron` -/
semantic_definition thopter : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Thopter")

/-- `subtypes/creature/thrull.ron` -/
semantic_definition thrull : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Thrull")

/-- `subtypes/creature/tiefling.ron` -/
semantic_definition tiefling : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Tiefling")

/-- `subtypes/creature/timeLord.ron` -/
semantic_definition timeLord : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Time Lord")

/-- `subtypes/creature/toy.ron` -/
semantic_definition toy : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Toy")

/-- `subtypes/creature/treefolk.ron` -/
semantic_definition treefolk : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Treefolk")

/-- `subtypes/creature/trilobite.ron` -/
semantic_definition trilobite : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Trilobite")

/-- `subtypes/creature/triskelavite.ron` -/
semantic_definition triskelavite : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Triskelavite")

/-- `subtypes/creature/troll.ron` -/
semantic_definition troll : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Troll")

/-- `subtypes/creature/turtle.ron` -/
semantic_definition turtle : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Turtle")

/-- `subtypes/creature/tyranid.ron` -/
semantic_definition tyranid : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Tyranid")

/-- `subtypes/creature/unicorn.ron` -/
semantic_definition unicorn : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Unicorn")

/-- `subtypes/creature/utrom.ron` -/
semantic_definition utrom : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Utrom")

/-- `subtypes/creature/vampire.ron` -/
semantic_definition vampire : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Vampire")

/-- `subtypes/creature/varmint.ron` -/
semantic_definition varmint : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Varmint")

/-- `subtypes/creature/vedalken.ron` -/
semantic_definition vedalken : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Vedalken")

/-- `subtypes/creature/villain.ron` -/
semantic_definition villain : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Villain")

/-- `subtypes/creature/volver.ron` -/
semantic_definition volver : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Volver")

/-- `subtypes/creature/wall.ron` -/
semantic_definition wall : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Wall")

/-- `subtypes/creature/walrus.ron` -/
semantic_definition walrus : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Walrus")

/-- `subtypes/creature/warlock.ron` -/
semantic_definition warlock : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Warlock")

/-- `subtypes/creature/warrior.ron` -/
semantic_definition warrior : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Warrior")

/-- `subtypes/creature/weasel.ron` -/
semantic_definition weasel : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Weasel")

/-- `subtypes/creature/weird.ron` -/
semantic_definition weird : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Weird")

/-- `subtypes/creature/werewolf.ron` -/
semantic_definition werewolf : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Werewolf")

/-- `subtypes/creature/whale.ron` -/
semantic_definition whale : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Whale")

/-- `subtypes/creature/wizard.ron` -/
semantic_definition wizard : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Wizard")

/-- `subtypes/creature/wolf.ron` -/
semantic_definition wolf : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Wolf")

/-- `subtypes/creature/wolverine.ron` -/
semantic_definition wolverine : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Wolverine")

/-- `subtypes/creature/wombat.ron` -/
semantic_definition wombat : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Wombat")

/-- `subtypes/creature/worm.ron` -/
semantic_definition worm : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Worm")

/-- `subtypes/creature/wraith.ron` -/
semantic_definition wraith : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Wraith")

/-- `subtypes/creature/wurm.ron` -/
semantic_definition wurm : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Wurm")

/-- `subtypes/creature/yeti.ron` -/
semantic_definition yeti : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Yeti")

/-- `subtypes/creature/zombie.ron` -/
semantic_definition zombie : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Zombie")

/-- `subtypes/creature/zubera.ron` -/
semantic_definition zubera : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.creature "Zubera")

/-- `subtypes/enchantment/aura.ron` -/
semantic_definition aura : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.enchantment "Aura")

/-- `subtypes/enchantment/background.ron` -/
semantic_definition background : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.enchantment "Background")

/-- `subtypes/enchantment/cartouche.ron` -/
semantic_definition cartouche : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.enchantment "Cartouche")

/-- `subtypes/enchantment/case.ron` -/
semantic_definition case : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.enchantment "Case")

/-- `subtypes/enchantment/class.ron` -/
semantic_definition class_ : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.enchantment "Class")

/-- `subtypes/enchantment/curse.ron` -/
semantic_definition curse : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.enchantment "Curse")

/-- `subtypes/enchantment/plan.ron` -/
semantic_definition plan : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.enchantment "Plan")

/-- `subtypes/enchantment/role.ron` -/
semantic_definition role : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.enchantment "Role")

/-- `subtypes/enchantment/room.ron` -/
semantic_definition room : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.enchantment "Room")

/-- `subtypes/enchantment/rune.ron` -/
semantic_definition rune : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.enchantment "Rune")

/-- `subtypes/enchantment/saga.ron` -/
semantic_definition saga : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.enchantment "Saga")

/-- `subtypes/enchantment/shard.ron` -/
semantic_definition shard : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.enchantment "Shard")

/-- `subtypes/enchantment/shrine.ron` -/
semantic_definition shrine : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.enchantment "Shrine")

/-- `subtypes/land/cave.ron` -/
semantic_definition cave : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.land "Cave")

/-- `subtypes/land/desert.ron` -/
semantic_definition desert : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.land "Desert")

/-- `subtypes/land/forest.ron` -/
semantic_definition forest : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.land "Forest")

/-- `subtypes/land/gate.ron` -/
semantic_definition gate : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.land "Gate")

/-- `subtypes/land/island.ron` -/
semantic_definition island : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.land "Island")

/-- `subtypes/land/lair.ron` -/
semantic_definition lair : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.land "Lair")

/-- `subtypes/land/locus.ron` -/
semantic_definition locus : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.land "Locus")

/-- `subtypes/land/mine.ron` -/
semantic_definition mine : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.land "Mine")

/-- `subtypes/land/mountain.ron` -/
semantic_definition mountain : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.land "Mountain")

/-- `subtypes/land/plains.ron` -/
semantic_definition plains : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.land "Plains")

/-- `subtypes/land/planet.ron` -/
semantic_definition planet : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.land "Planet")

/-- `subtypes/land/powerPlant.ron` -/
semantic_definition powerPlant : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.land "Power-Plant")

/-- `subtypes/land/sphere.ron` -/
semantic_definition sphere : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.land "Sphere")

/-- `subtypes/land/swamp.ron` -/
semantic_definition swamp : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.land "Swamp")

/-- `subtypes/land/tower.ron` -/
semantic_definition tower : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.land "Tower")

/-- `subtypes/land/town.ron` -/
semantic_definition town : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.land "Town")

/-- `subtypes/land/urzas.ron` -/
semantic_definition urzas : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.land "Urza's")

/-- `subtypes/planeswalker/ajani.ron` -/
semantic_definition ajani : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Ajani")

/-- `subtypes/planeswalker/aminatou.ron` -/
semantic_definition aminatou : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Aminatou")

/-- `subtypes/planeswalker/angrath.ron` -/
semantic_definition angrath : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Angrath")

/-- `subtypes/planeswalker/arlinn.ron` -/
semantic_definition arlinn : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Arlinn")

/-- `subtypes/planeswalker/ashiok.ron` -/
semantic_definition ashiok : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Ashiok")

/-- `subtypes/planeswalker/bahamut.ron` -/
semantic_definition bahamut : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Bahamut")

/-- `subtypes/planeswalker/basri.ron` -/
semantic_definition basri : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Basri")

/-- `subtypes/planeswalker/bolas.ron` -/
semantic_definition bolas : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Bolas")

/-- `subtypes/planeswalker/calix.ron` -/
semantic_definition calix : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Calix")

/-- `subtypes/planeswalker/chandra.ron` -/
semantic_definition chandra : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Chandra")

/-- `subtypes/planeswalker/comet.ron` -/
semantic_definition comet : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Comet")

/-- `subtypes/planeswalker/dack.ron` -/
semantic_definition dack : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Dack")

/-- `subtypes/planeswalker/dakkon.ron` -/
semantic_definition dakkon : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Dakkon")

/-- `subtypes/planeswalker/daretti.ron` -/
semantic_definition daretti : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Daretti")

/-- `subtypes/planeswalker/davriel.ron` -/
semantic_definition davriel : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Davriel")

/-- `subtypes/planeswalker/dellian.ron` -/
semantic_definition dellian : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Dellian")

/-- `subtypes/planeswalker/dihada.ron` -/
semantic_definition dihada : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Dihada")

/-- `subtypes/planeswalker/domri.ron` -/
semantic_definition domri : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Domri")

/-- `subtypes/planeswalker/dovin.ron` -/
semantic_definition dovin : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Dovin")

/-- `subtypes/planeswalker/ellywick.ron` -/
semantic_definition ellywick : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Ellywick")

/-- `subtypes/planeswalker/elminster.ron` -/
semantic_definition elminster : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Elminster")

/-- `subtypes/planeswalker/elspeth.ron` -/
semantic_definition elspeth : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Elspeth")

/-- `subtypes/planeswalker/estrid.ron` -/
semantic_definition estrid : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Estrid")

/-- `subtypes/planeswalker/freyalise.ron` -/
semantic_definition freyalise : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Freyalise")

/-- `subtypes/planeswalker/garruk.ron` -/
semantic_definition garruk : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Garruk")

/-- `subtypes/planeswalker/gideon.ron` -/
semantic_definition gideon : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Gideon")

/-- `subtypes/planeswalker/grist.ron` -/
semantic_definition grist : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Grist")

/-- `subtypes/planeswalker/guff.ron` -/
semantic_definition guff : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Guff")

/-- `subtypes/planeswalker/huatli.ron` -/
semantic_definition huatli : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Huatli")

/-- `subtypes/planeswalker/jace.ron` -/
semantic_definition jace : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Jace")

/-- `subtypes/planeswalker/jared.ron` -/
semantic_definition jared : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Jared")

/-- `subtypes/planeswalker/jaya.ron` -/
semantic_definition jaya : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Jaya")

/-- `subtypes/planeswalker/jeska.ron` -/
semantic_definition jeska : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Jeska")

/-- `subtypes/planeswalker/kaito.ron` -/
semantic_definition kaito : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Kaito")

/-- `subtypes/planeswalker/karn.ron` -/
semantic_definition karn : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Karn")

/-- `subtypes/planeswalker/kasmina.ron` -/
semantic_definition kasmina : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Kasmina")

/-- `subtypes/planeswalker/kaya.ron` -/
semantic_definition kaya : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Kaya")

/-- `subtypes/planeswalker/kiora.ron` -/
semantic_definition kiora : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Kiora")

/-- `subtypes/planeswalker/koth.ron` -/
semantic_definition koth : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Koth")

/-- `subtypes/planeswalker/liliana.ron` -/
semantic_definition liliana : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Liliana")

/-- `subtypes/planeswalker/lolth.ron` -/
semantic_definition lolth : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Lolth")

/-- `subtypes/planeswalker/lukka.ron` -/
semantic_definition lukka : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Lukka")

/-- `subtypes/planeswalker/minsc.ron` -/
semantic_definition minsc : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Minsc")

/-- `subtypes/planeswalker/mordenkainen.ron` -/
semantic_definition mordenkainen : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Mordenkainen")

/-- `subtypes/planeswalker/nahiri.ron` -/
semantic_definition nahiri : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Nahiri")

/-- `subtypes/planeswalker/narset.ron` -/
semantic_definition narset : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Narset")

/-- `subtypes/planeswalker/niko.ron` -/
semantic_definition niko : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Niko")

/-- `subtypes/planeswalker/nissa.ron` -/
semantic_definition nissa : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Nissa")

/-- `subtypes/planeswalker/nixilis.ron` -/
semantic_definition nixilis : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Nixilis")

/-- `subtypes/planeswalker/oko.ron` -/
semantic_definition oko : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Oko")

/-- `subtypes/planeswalker/quintorius.ron` -/
semantic_definition quintorius : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Quintorius")

/-- `subtypes/planeswalker/ral.ron` -/
semantic_definition ral : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Ral")

/-- `subtypes/planeswalker/rowan.ron` -/
semantic_definition rowan : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Rowan")

/-- `subtypes/planeswalker/saheeli.ron` -/
semantic_definition saheeli : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Saheeli")

/-- `subtypes/planeswalker/samut.ron` -/
semantic_definition samut : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Samut")

/-- `subtypes/planeswalker/sarkhan.ron` -/
semantic_definition sarkhan : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Sarkhan")

/-- `subtypes/planeswalker/serra.ron` -/
semantic_definition serra : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Serra")

/-- `subtypes/planeswalker/sivitri.ron` -/
semantic_definition sivitri : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Sivitri")

/-- `subtypes/planeswalker/sorin.ron` -/
semantic_definition sorin : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Sorin")

/-- `subtypes/planeswalker/szat.ron` -/
semantic_definition szat : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Szat")

/-- `subtypes/planeswalker/tamiyo.ron` -/
semantic_definition tamiyo : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Tamiyo")

/-- `subtypes/planeswalker/tasha.ron` -/
semantic_definition tasha : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Tasha")

/-- `subtypes/planeswalker/teferi.ron` -/
semantic_definition teferi : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Teferi")

/-- `subtypes/planeswalker/teyo.ron` -/
semantic_definition teyo : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Teyo")

/-- `subtypes/planeswalker/tezzeret.ron` -/
semantic_definition tezzeret : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Tezzeret")

/-- `subtypes/planeswalker/tibalt.ron` -/
semantic_definition tibalt : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Tibalt")

/-- `subtypes/planeswalker/tyvar.ron` -/
semantic_definition tyvar : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Tyvar")

/-- `subtypes/planeswalker/ugin.ron` -/
semantic_definition ugin : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Ugin")

/-- `subtypes/planeswalker/urza.ron` -/
semantic_definition urza : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Urza")

/-- `subtypes/planeswalker/venser.ron` -/
semantic_definition venser : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Venser")

/-- `subtypes/planeswalker/vivien.ron` -/
semantic_definition vivien : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Vivien")

/-- `subtypes/planeswalker/vraska.ron` -/
semantic_definition vraska : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Vraska")

/-- `subtypes/planeswalker/vronos.ron` -/
semantic_definition vronos : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Vronos")

/-- `subtypes/planeswalker/will.ron` -/
semantic_definition will : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Will")

/-- `subtypes/planeswalker/windgrace.ron` -/
semantic_definition windgrace : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Windgrace")

/-- `subtypes/planeswalker/wrenn.ron` -/
semantic_definition wrenn : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Wrenn")

/-- `subtypes/planeswalker/xenagos.ron` -/
semantic_definition xenagos : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Xenagos")

/-- `subtypes/planeswalker/yanggu.ron` -/
semantic_definition yanggu : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Yanggu")

/-- `subtypes/planeswalker/yanling.ron` -/
semantic_definition yanling : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Yanling")

/-- `subtypes/planeswalker/zariel.ron` -/
semantic_definition zariel : Semantics.Subtype :=
  (Semantics.Subtype.of Semantics.CardType.planeswalker "Zariel")

/-- `subtypes/spell/adventure.ron` -/
semantic_definition adventure : Semantics.Subtype :=
  (Semantics.Subtype.spell "Adventure")

/-- `subtypes/spell/arcane.ron` -/
semantic_definition arcane : Semantics.Subtype :=
  (Semantics.Subtype.spell "Arcane")

/-- `subtypes/spell/lesson.ron` -/
semantic_definition lesson : Semantics.Subtype :=
  (Semantics.Subtype.spell "Lesson")

/-- `subtypes/spell/omen.ron` -/
semantic_definition omen : Semantics.Subtype :=
  (Semantics.Subtype.spell "Omen")

/-- `subtypes/spell/trap.ron` -/
semantic_definition trap : Semantics.Subtype :=
  (Semantics.Subtype.spell "Trap")

/-- `text_changes/appendTextOf.ron` -/
semantic_template appendTextOf (source : Semantics.NounPhrase) : Semantics.TextChange :=
  (Semantics.TextChange.appendTextOf source)

/-- `text_changes/exchangeWith.ron` -/
semantic_template exchangeWith (other : Semantics.NounPhrase) : Semantics.TextChange :=
  (Semantics.TextChange.exchangeWith other)

/-- `text_changes/removeBracketed.ron` -/
semantic_template removeBracketed : Semantics.TextChange :=
  Semantics.TextChange.removeBracketed

/-- `text_changes/replaceWord.ron` -/
semantic_template replaceWord (sorts : (List Semantics.QualitySort)) (from_ : (Option Semantics.TextWord) := none) (to : (Option Semantics.TextWord) := none) : Semantics.TextChange :=
  (Semantics.TextChange.replaceWord sorts from_ to)

/-- `text_changes/targetToEach.ron` -/
semantic_template targetToEach : Semantics.TextChange :=
  Semantics.TextChange.targetToEach

/-- `timings/and.ron` -/
semantic_template Timing.and (conjuncts : (List Semantics.Timing) := []) : Semantics.Timing :=
  (Semantics.Timing.and conjuncts)

/-- `timings/not.ron` -/
semantic_template Timing.not (timing : Semantics.Timing) : Semantics.Timing :=
  (Semantics.Timing.not timing)

/-- `timings/or.ron` -/
semantic_template Timing.or (disjuncts : (List Semantics.Timing) := []) : Semantics.Timing :=
  (Semantics.Timing.or disjuncts)

/-- `turn_parts/beginningOfCombatStep.ron` -/
semantic_definition beginningOfCombatStep : Semantics.TurnPart :=
  Semantics.TurnPart.beginningOfCombat

/-- `turn_parts/beginningPhase.ron` -/
semantic_definition beginningPhase : Semantics.TurnPart :=
  Semantics.TurnPart.beginningPhase

/-- `turn_parts/cleanup.ron` -/
semantic_definition cleanup : Semantics.TurnPart :=
  Semantics.TurnPart.cleanup

/-- `turn_parts/cleanupStep.ron` -/
semantic_definition cleanupStep : Semantics.TurnPart :=
  Semantics.TurnPart.cleanup

/-- `turn_parts/combat.ron` -/
semantic_definition TurnPart.combat : Semantics.TurnPart :=
  Semantics.TurnPart.combat

/-- `turn_parts/combatDamageStep.ron` -/
semantic_definition combatDamageStep : Semantics.TurnPart :=
  Semantics.TurnPart.combatDamage

/-- `turn_parts/combatPhase.ron` -/
semantic_definition combatPhase : Semantics.TurnPart :=
  Semantics.TurnPart.combat

/-- `turn_parts/declareAttackersStep.ron` -/
semantic_definition declareAttackersStep : Semantics.TurnPart :=
  Semantics.TurnPart.declareAttackers

/-- `turn_parts/declareBlockersStep.ron` -/
semantic_definition declareBlockersStep : Semantics.TurnPart :=
  Semantics.TurnPart.declareBlockers

/-- `turn_parts/drawStep.ron` -/
semantic_definition drawStep : Semantics.TurnPart :=
  Semantics.TurnPart.drawStep

/-- `turn_parts/endOfCombatStep.ron` -/
semantic_definition endOfCombatStep : Semantics.TurnPart :=
  Semantics.TurnPart.endOfCombat

/-- `turn_parts/endStep.ron` -/
semantic_definition endStep : Semantics.TurnPart :=
  Semantics.TurnPart.endStep

/-- `turn_parts/endingPhase.ron` -/
semantic_definition endingPhase : Semantics.TurnPart :=
  Semantics.TurnPart.endingPhase

/-- `turn_parts/mainPhase.ron` -/
semantic_definition mainPhase : Semantics.TurnPart :=
  Semantics.TurnPart.mainPhase

/-- `turn_parts/postcombatMainPhase.ron` -/
semantic_definition postcombatMainPhase : Semantics.TurnPart :=
  Semantics.TurnPart.postcombatMain

/-- `turn_parts/precombatMainPhase.ron` -/
semantic_definition precombatMainPhase : Semantics.TurnPart :=
  Semantics.TurnPart.firstMain

/-- `turn_parts/untapStep.ron` -/
semantic_definition untapStep : Semantics.TurnPart :=
  Semantics.TurnPart.untapStep

/-- `turn_parts/upkeep.ron` -/
semantic_definition upkeep : Semantics.TurnPart :=
  Semantics.TurnPart.upkeep

/-- `turn_parts/upkeepStep.ron` -/
semantic_definition upkeepStep : Semantics.TurnPart :=
  Semantics.TurnPart.upkeep

/-- `zones/battlefield.ron` -/
semantic_template battlefield : Semantics.ZoneExpr :=
  (Semantics.ZoneExpr.zone Semantics.Zone.battlefield Semantics.ZoneScope.bare)

/-- `zones/command.ron` -/
semantic_template command : Semantics.ZoneExpr :=
  (Semantics.ZoneExpr.zone Semantics.Zone.command Semantics.ZoneScope.bare)

/-- `zones/commandZone.ron` -/
semantic_template commandZone : Semantics.ZoneExpr :=
  (Semantics.ZoneExpr.zone Semantics.Zone.command Semantics.ZoneScope.bare)

/-- `zones/exileZone.ron` -/
semantic_template exileZone : Semantics.ZoneExpr :=
  (Semantics.ZoneExpr.zone Semantics.Zone.exile Semantics.ZoneScope.bare)

/-- `zones/graveyard.ron` -/
semantic_template graveyard : Semantics.ZoneExpr :=
  (Semantics.ZoneExpr.zone Semantics.Zone.graveyard Semantics.ZoneScope.bare)

/-- `zones/graveyardOf.ron` -/
semantic_template graveyardOf (player : Semantics.NounPhrase) : Semantics.ZoneExpr :=
  (Semantics.ZoneExpr.zone Semantics.Zone.graveyard (Semantics.ZoneScope.possessedBy player))

/-- `zones/hand.ron` -/
semantic_template hand : Semantics.ZoneExpr :=
  (Semantics.ZoneExpr.zone Semantics.Zone.hand Semantics.ZoneScope.bare)

/-- `zones/handOf.ron` -/
semantic_template handOf (player : Semantics.NounPhrase) : Semantics.ZoneExpr :=
  (Semantics.ZoneExpr.zone Semantics.Zone.hand (Semantics.ZoneScope.possessedBy player))

/-- `zones/onBottom.ron` -/
semantic_template onBottom : Semantics.ZoneExpr :=
  (Semantics.ZoneExpr.place Semantics.LibraryEnd.bottom (Semantics.Pile.library Semantics.ZoneScope.bare))

/-- `zones/onTop.ron` -/
semantic_template onTop : Semantics.ZoneExpr :=
  (Semantics.ZoneExpr.place Semantics.LibraryEnd.top (Semantics.Pile.library Semantics.ZoneScope.bare))

/-- `zones/place.ron` -/
semantic_template place (end_ : Semantics.LibraryEnd) (pile : Semantics.Pile) : Semantics.ZoneExpr :=
  (Semantics.ZoneExpr.place end_ pile)

/-- `zones/stack.ron` -/
semantic_template stack : Semantics.ZoneExpr :=
  (Semantics.ZoneExpr.zone Semantics.Zone.stack Semantics.ZoneScope.bare)

/-- `zones/wherever.ron` -/
semantic_template wherever : Semantics.ZoneExpr :=
  Semantics.ZoneExpr.wherever

/-- `zones/yourHand.ron` -/
semantic_template yourHand : Semantics.ZoneExpr :=
  (Semantics.ZoneExpr.zone Semantics.Zone.hand (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.you))

/-- `zones/yourLibrary.ron` -/
semantic_template yourLibrary : Semantics.ZoneExpr :=
  (Semantics.ZoneExpr.zone Semantics.Zone.library (Semantics.ZoneScope.possessedBy Semantics.NounPhrase.you))

/-- `zones/zone.ron` -/
semantic_template zone (zone : Semantics.Zone) (scope : Semantics.ZoneScope) : Semantics.ZoneExpr :=
  (Semantics.ZoneExpr.zone zone scope)

end Semantics.Macros.Declared
