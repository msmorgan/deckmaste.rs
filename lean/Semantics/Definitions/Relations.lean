import Semantics.Primitives

/-!
# Semantics.Definitions.Relations

The rules-defined relation labels: each declaration's name is its `RelationLabel`. A leaf
catalog, spelled over primitives only. `Semantics.Templates` names none of its
entries.
-/

namespace Semantics.Definitions

/-! ## Relations -/

semantic_definition encodedOn : RelationLabel := .named "encodedOn"
semantic_definition haunting : RelationLabel := .named "haunting"
semantic_definition paired : RelationLabel := .named "paired"

end Semantics.Definitions
