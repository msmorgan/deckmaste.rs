import Semantics.Macros.Declared
import Semantics.Macros
import Semantics.Check.Card

/-!
# Semantics.Proofs.KeywordDefinitionRegimes

A keyword's definition is a list of components, each carrying the stack regime the rules give it
(`ComponentFacts`, `docs/decisions/semantics-v2.md` §7, ruling 2026-10-06 on gift and the
additional-cost keywords), and each component's form is checked against its own regime. Nothing
is exempt. Kicker is the one-component case [CR#702.33a]. Offspring and squad add an enters
trigger [CR#702.175a,702.157a], casualty a cast trigger [CR#702.153a], and gift an enters trigger
on a permanent and a spell ability on an instant or sorcery [CR#702.174a,702.174b].

Casualty declares no definition yet, so its pins hold a row written here, shaped as its will be;
the offspring and squad rows here agree with the registry's since their definitions were declared
(`okOffspringRegistryRow`, `okSquadRegistryRow`), and gift's predates its definition, which
`Proofs/GiftVariants.lean` checks against the registry's own row. The stand-in effects read each
component's form, not what it does; offspring's and squad's declared definitions are pinned with
their real effects.
-/

open Semantics Semantics.Macros

namespace Semantics.Proofs.KeywordDefinitionRegimes

/-- "{2}" -/
def two : Cost := .mana [generic 2]

/-- "You may pay an additional {2} as you cast this spell." -/
def costPart : Ability := .static (.addedCost two true .once)

/-- "When this permanent enters, draw a card." -/
def entersPart : Ability :=
  when (Declared.enters thisPermanent none) (draw (.lit 1))

/-- "When you cast this spell, draw a card." -/
def castPart : Ability := when (.does (.actor .you) (.procedure .cast thisSpell .itsOwnCost none none none)) (draw (.lit 1))

/-- "At the beginning of your end step, draw a card." -/
def endStepPart : Ability :=
  when (.beginningOf .endStep (some .you) (.each none)) (draw (.lit 1))

/-- "You may cast this card by paying {2} rather than its mana cost." -/
def castPermission : Ability :=
  .static (.may (.actor .you) (.procedure .cast .this .itsOwnCost none none none) { rider := (.play none none none false (.payingInstead two)) })

/-- "if this is a permanent card" -/
def onPermanentCard : Condition := .matches .this permanentCard

/-- "if this is an instant or sorcery" -/
def onInstantOrSorcery : Condition := .matches .this instantOrSorcery

/-- Gift's permanent half, guarded by the class. -/
def giftPermanentPart : Ability :=
  triggeredIf (Declared.enters thisPermanent none) onPermanentCard (draw (.lit 1))

/-- Gift's spell half, guarded by the class. -/
def giftSpellPart : Ability := .spell none (doIf onInstantOrSorcery (draw (.lit 1)))

/-- Offspring's components: an additional cost as the spell is cast, and an enters trigger
[CR#702.175a]. -/
def offspringRow : KeywordFacts :=
  { word := .named "offspring", argumentSchemas := [[⟨.cost, none⟩]], functionsOnStack := true,
    paidCost := true, definition := [⟨.static, some .atCasting⟩, ⟨.triggered, none⟩],
    regime := some .atCasting }

/-- Squad's components: an additional cost and an enters trigger [CR#702.157a]. -/
def squadRow : KeywordFacts := { offspringRow with word := .named "squad" }

/-- Casualty's components: an additional cost and a cast trigger, both on the stack
[CR#702.153a]. -/
def casualtyRow : KeywordFacts :=
  { word := .named "casualty", argumentSchemas := [[⟨.number, none⟩]], functionsOnStack := true,
    onInstantOrSorceryCard := true, paidCost := true,
    definition := [⟨.static, some .atCasting⟩, ⟨.triggered, some .atCasting⟩],
    regime := some .atCasting }

/-- Gift's components: the additional cost, the enters trigger a permanent has, and the spell
ability an instant or sorcery has [CR#702.174a,702.174b]. -/
def giftRow : KeywordFacts :=
  { word := .named "gift", argumentSchemas := [[]], functionsOnStack := true,
    onInstantOrSorceryCard := true, paidCost := true, costChoosesPlayer := true,
    definition :=
      [⟨.static, some .atCasting⟩, ⟨.triggered, none⟩, ⟨.spell, some .atResolution⟩],
    regime := some .atCasting }

/-- Every component's regime is its own, so the one regime the old law gave the whole keyword is
the twin of each row here: every triggered component keyed at casting. -/
def singleRegime (row : KeywordFacts) : KeywordFacts :=
  { row with
    definition := row.definition.map fun part =>
      if part.category == .triggered then { part with regime := some .atCasting } else part }

/-- "Kicker {2}": the one-component case [CR#702.33a]. -/
theorem okKickerOneComponent :
    Ability.check (DRS.ofList []) (.keyword (.named "kicker") [.cost two] [costPart]) = [] := by
  decide

/-- Kicker's cost is a casting-time component: a declaration putting it off the stack is
refused. -/
theorem badKickerCostOffTheStack :
    ({ word := .named "kicker", definition := [⟨.static, none⟩] } : KeywordFacts).bodyPartFits
      costPart = false := by
  decide

/-- Offspring: the cost at casting and the enters trigger off the stack each fit their own
regime [CR#702.175a]. -/
theorem okOffspringComponents : [costPart, entersPart].all offspringRow.bodyPartFits = true := by
  decide

/-- Squad's components are offspring's shape [CR#702.157a]. -/
theorem okSquadComponents : [costPart, entersPart].all squadRow.bodyPartFits = true := by
  decide

/-- "create a token that's a copy of it, except it's 1/1" [CR#702.175a] -/
def offspringCopy : Instruction :=
  Primitives.Instruction.create (.lit 1)
    (.copyOf thisPermanent [.edits [.stat .sets .power (.lit 1), .stat .sets .toughness (.lit 1)]])
    []

/-- Offspring's definition as its declaration builds it: the additional cost, and "When this
permanent enters, if its offspring cost was paid, create a token that's a copy of it, except it's
1/1." [CR#702.175a] -/
def offspringDefinition : List Ability :=
  [ costPart,
    triggeredIf (Declared.enters thisPermanent none)
      (costWasPaid (.byKeyword (.named "offspring")) none thisPermanent) offspringCopy ]

/-- The registry's offspring row declares the components the pins above were written against. -/
theorem okOffspringRegistryRow :
    ((keywordFactsFor (.named "offspring")).map (·.definition),
      (keywordFactsFor (.named "offspring")).map (·.regime))
      = (some offspringRow.definition, some offspringRow.regime) := by
  decide

/-- "Offspring {2}" with its declared definition fits the registry's row and checks
[CR#702.175a]. -/
theorem okOffspringDefinition :
    (keywordBodyFits (.named "offspring") offspringDefinition,
      Ability.check (DRS.ofList []) (.keyword (.named "offspring") [.cost two] offspringDefinition))
      = (true, []) := by
  decide

/-- Squad's second ability: "When this creature enters, if its squad cost was paid, create a
token that's a copy of it for each time its squad cost was paid." [CR#702.157a] -/
def squadEntersPart : Ability :=
  triggeredIf (Declared.enters thisCreature none)
    (costWasPaid (.byKeyword (.named "squad")) none thisCreature)
    (Primitives.Instruction.create (timesPaid (.byKeyword (.named "squad")) thisCreature)
      (.copyOf thisCreature []) [])

theorem okSquadSecondAbility :
    (squadRow.bodyPartFits squadEntersPart, Ability.check (DRS.ofList []) squadEntersPart) = (true, []) := by
  decide

/-- Squad's first ability: "As an additional cost to cast this spell, you may pay {2} any number
of times." The cost carries its repetition, so `timesPaid` counts payments of {2} itself
[CR#702.157a] (ruling 2026-10-08). -/
def squadCostPart : Ability := .static (.addedCost two true .anyNumberOfTimes)

/-- Squad's definition as its declaration builds it [CR#702.157a]. -/
def squadDefinition : List Ability := [squadCostPart, squadEntersPart]

/-- The registry's squad row declares the components the pins above were written against. -/
theorem okSquadRegistryRow :
    ((keywordFactsFor (.named "squad")).map (·.definition),
      (keywordFactsFor (.named "squad")).map (·.regime))
      = (some squadRow.definition, some squadRow.regime) := by
  decide

/-- "Squad {2}" with its declared definition fits the registry's row and checks
[CR#702.157a]. -/
theorem okSquadDefinition :
    (keywordBodyFits (.named "squad") squadDefinition,
      Ability.check (DRS.ofList []) (.keyword (.named "squad") [.cost two] squadDefinition))
      = (true, []) := by
  decide

/-- Under one regime for the whole keyword the enters trigger is refused, as it was. -/
theorem badOffspringSingleRegime :
    ((singleRegime offspringRow).bodyPartFits entersPart,
      (singleRegime squadRow).bodyPartFits entersPart) = (false, false) := by
  decide

/-- Casualty: the cost and the cast trigger both function on the stack [CR#702.153a]. -/
theorem okCasualtyComponents : [costPart, castPart].all casualtyRow.bodyPartFits = true := by
  decide

/-- An enters trigger is not casualty's second component. -/
theorem badCasualtyEntersTrigger : casualtyRow.bodyPartFits entersPart = false := by
  decide

/-- Gift on a permanent: the cost and the enters trigger [CR#702.174a,702.174b]. -/
theorem okGiftPermanentComponents :
    [costPart, giftPermanentPart].all giftRow.bodyPartFits = true := by
  decide

/-- Gift on an instant or sorcery: the cost and the spell ability under its class guard
[CR#702.174b,113.3a]. -/
theorem okGiftSpellComponents : [costPart, giftSpellPart].all giftRow.bodyPartFits = true := by
  decide

/-- A spell ability is followed as the spell resolves; one declared at casting is refused. -/
theorem badGiftSpellAtCasting :
    { giftRow with definition := [⟨.spell, some .atCasting⟩] }.bodyPartFits giftSpellPart
      = false := by
  decide

/-- The row's regime is its components' summary, and the registry's gift row agrees with it. -/
theorem okGiftRowSummary :
    (ComponentFacts.summary giftRow.definition, keywordStackRegime (.named "gift"))
      = (some .atCasting, some .atCasting) := by
  decide

/-- Impending's end-step trigger is checked against its own regime and fits the registry's row
[CR#702.176a]. -/
theorem okImpendingEndStepTrigger :
    keywordBodyPartFits (.named "impending") endStepPart = true := by
  decide

/-- The twin: under the keyword's one casting regime the same trigger is refused, the mismatch
the old law carried latent. -/
theorem badImpendingSingleRegime :
    ((keywordFactsFor (.named "impending")).map fun row =>
      (singleRegime row).bodyPartFits endStepPart) = some false := by
  decide

/-- A permission to cast the card leaves its regime to the keyword's rule: impending's functions
on the stack [CR#702.176a], flashback's in the graveyard [CR#702.34a]. Neither is a resolution
regime. -/
theorem okCastPermissionRegimes :
    (keywordBodyPartFits (.named "impending") castPermission,
      keywordBodyPartFits (.named "flashback") castPermission,
      ({ word := .named "impending", definition := [⟨.static, some .atResolution⟩] }
        : KeywordFacts).bodyPartFits castPermission) = (true, true, false) := by
  decide

end Semantics.Proofs.KeywordDefinitionRegimes
