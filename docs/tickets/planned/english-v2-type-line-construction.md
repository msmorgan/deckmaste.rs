---
needs: [english-v2-grammar-family-breadth]
---
# Declare and consume type-line order at the open Type inventory

Complete this family on the shared breadth interfaces under
[the lexical-analysis contract](../../decisions/english-lexical-analysis.md).

Implement the Type Line construction, its consumed order declaration and their
tests together. Add linearization order to normalized open Type metadata and
populate it in `plugins_v2/builtin`; parser, renderer and diagnostics consume
that same inventory. This explicitly includes `deckmaste_construction_core`
and builtin declarations, resolving the old declaration-only ticket's
consumption/timing STOP. No duplicated closed Type vocabulary or runtime guard
naming a Card Type. Compiler changes required by this consumer are in scope.

Use flat typed supertype/card-type/subtype collections with their structural
separator and case rules. Convert the measured table in
`crates/deckmaste_english_v2/docs/type-line-order.md` into the consumed order
metadata; retain its twelve attested sequences as conformance witnesses,
not a whitelist. Linearization orders any well-formed set of declared types by that metadata,
including an unattested combination. Checked sequence construction requires
that order; parsing noncanonical order rejects rather than normalizes source
bytes. Reject malformed declared ordering metadata. Parsing
and rendering share the order and lexical identities.

Acceptance includes all twelve sequences, the unattested combination, empty
permitted sections, rejection of noncanonical source order,
byte-exact rendering, lexical identity/provenance and structural/leaf traversal.
Derived token positions are provisional, not a separate ownership system to port.
This models the textual Type Line only,
not card legality. Use the reviewed type collection and extend its order
witnesses as needed. Source evidence: style-guide §7 “Types, subtypes, and
supertypes as nouns and modifiers”. The old STOP record remains available in
history; its no-consumer/no-schema-change fences are superseded by this ticket.

Preserve the applicable [inherited witnesses](../../english-grammar-migration-obligations.md)
and check this family's structures under both roundtrip laws. Standard constraints
apply as amended by the lexical-analysis decision; targeted Lean changes belong
here only when needed to settle a changed grammatical constraint.
