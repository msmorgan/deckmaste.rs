---
needs: [english-v2-grammar-migration-design, english-v2-grammar-family-breadth]
---
# Declare and consume type-line order at the open Type inventory

The first shared interface is supplied by `english-v2-grammar-family-breadth`.
Complete the acceptance below against those interfaces; other families need
not finish their inventories before this work begins.

Implement the Type Line construction, its consumed order declaration and their
tests together. Add linearization order to normalized open Type metadata and
populate it in `plugins/builtin_v2`; parser, renderer and diagnostics consume
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
byte-exact ownership and traversal. This models the textual Type Line only,
not card legality. Use the reviewed type collection and extend its order
witnesses as needed. Source evidence: style-guide §7 “Types, subtypes, and
supertypes as nouns and modifiers”. The old STOP record remains available in
history; its no-consumer/no-schema-change fences are superseded by this ticket.

Standard constraints apply. Production correspondence and the applicable
[obligations](../../english-grammar-migration-obligations.md) are part of this
ticket; re-spell existing tests by their independently justified outcomes.
