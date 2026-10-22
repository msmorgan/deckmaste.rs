import Semantics.Primitives

/-!
# Semantics.Definitions.Designations

The rules-defined designation labels: each declaration's name is its `DesignationLabel`. A leaf
catalog, spelled over primitives only, so `Semantics.Templates` can name its entries.

Two rules-defined designations are not here. The sector designations [CR#702.158b] exist only
for space sculptor, on an Unfinity card, so they are deferred with the Un-sets. A battle's
protector [CR#310.9] is a player designated by the battle, a link from an object to a player,
which neither a designation label (one holder's marker) nor a Relation (two objects) writes; it
is deferred until a bench card reads a battle's protector (the engine side is `engine-battles`).
-/

namespace Semantics.Definitions

/-! ## Designations -/

semantic_definition citysBlessing : DesignationLabel := .named "citysBlessing"
semantic_definition commander : DesignationLabel := .named "commander"
/-- Day and night, the designations the game itself holds, exactly one once either is held
[CR#731.1]. -/
semantic_definition day : DesignationLabel := .named "day"
semantic_definition night : DesignationLabel := .named "night"
semantic_definition enduringStory : DesignationLabel := .named "enduringStory"
semantic_definition goaded : DesignationLabel := .named "goaded"
semantic_definition harnessed : DesignationLabel := .named "harnessed"
semantic_definition initiative : DesignationLabel := .named "initiative"
semantic_definition leftHalfUnlocked : DesignationLabel := .named "leftHalfUnlocked"
semantic_definition level : DesignationLabel := .named "level"
semantic_definition monarch : DesignationLabel := .named "monarch"
semantic_definition monstrous : DesignationLabel := .named "monstrous"
semantic_definition prepared : DesignationLabel := .named "prepared"
semantic_definition renowned : DesignationLabel := .named "renowned"
semantic_definition rightHalfUnlocked : DesignationLabel := .named "rightHalfUnlocked"
semantic_definition ringBearer : DesignationLabel := .named "ringBearer"
semantic_definition saddled : DesignationLabel := .named "saddled"
semantic_definition DesignationLabel.solved : DesignationLabel := .named "solved"
semantic_definition suspected : DesignationLabel := .named "suspected"

end Semantics.Definitions
