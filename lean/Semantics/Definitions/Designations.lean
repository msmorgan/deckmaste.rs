import Semantics.Primitives

/-!
# Semantics.Definitions.Designations

The rules-defined designation labels: each declaration's name is its `DesignationLabel`. A leaf
catalog, spelled over primitives only. `Semantics.Templates` names none of its
entries.

Two rules-defined designations are not here. The sector designations [CR#702.158b] exist only
for space sculptor, on an Unfinity card, so they are deferred with the Un-sets. A battle's
protector [CR#310.9] is a player designated by the battle, a link from an object to a player,
which neither a designation label (one holder's marker) nor a Relation (two objects) writes; it
is deferred until a bench card reads a battle's protector (the engine side is `engine-battles`).
-/

namespace Semantics.Definitions

/-! ## Designations -/

semantic_definition citysBlessing : DesignationLabel := .named "citysBlessing" (.heldBy .player)
semantic_definition commander : DesignationLabel := .named "commander" .heldByCard
/-- Day and night, the designations the game itself holds, exactly one once either is held
[CR#731.1]. -/
semantic_definition day : DesignationLabel := .named "day" .heldByGame
semantic_definition night : DesignationLabel := .named "night" .heldByGame
semantic_definition enduringStory : DesignationLabel := .named "enduringStory" (.heldBy .player)
/-- Foretold, a state of a card in exile: a card put into exile by the foretell special action is
a foretold card, an effect may make a card in exile become one, and its owner may look at it and
cast it for a foretell cost on a later turn [CR#702.143a,702.143c,702.143d]. -/
semantic_definition foretold : DesignationLabel := .named "foretold" (.heldBy .object)
semantic_definition goaded : DesignationLabel := .named "goaded" (.heldBy .object)
semantic_definition harnessed : DesignationLabel := .named "harnessed" (.heldBy .object)
semantic_definition initiative : DesignationLabel := .named "initiative" (.heldBy .player)
semantic_definition leftHalfUnlocked : DesignationLabel :=
  .named "leftHalfUnlocked" (.heldBy .object)
semantic_definition level : DesignationLabel := .named "level" (.heldBy .object)
semantic_definition monarch : DesignationLabel := .named "monarch" (.heldBy .player)
semantic_definition monstrous : DesignationLabel := .named "monstrous" (.heldBy .object)
/-- Plotted, a state of a card in exile: a card exiled by the plot special action "becomes a
plotted card", an effect may make a card in exile become one, and its owner may cast it without
paying its mana cost on a later turn [CR#702.170a,702.170c,702.170d]. -/
semantic_definition plotted : DesignationLabel := .named "plotted" (.heldBy .object)
semantic_definition prepared : DesignationLabel := .named "prepared" (.heldBy .object)
semantic_definition renowned : DesignationLabel := .named "renowned" (.heldBy .object)
semantic_definition rightHalfUnlocked : DesignationLabel :=
  .named "rightHalfUnlocked" (.heldBy .object)
semantic_definition ringBearer : DesignationLabel := .named "ringBearer" (.heldBy .object)
semantic_definition saddled : DesignationLabel := .named "saddled" (.heldBy .object)
semantic_definition DesignationLabel.solved : DesignationLabel := .named "solved" (.heldBy .object)
semantic_definition suspected : DesignationLabel := .named "suspected" (.heldBy .object)

end Semantics.Definitions
