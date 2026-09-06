---
needs: []
---
# Fold attachment predicates and read current face state

Implement two local predicate folds. Apply the
[campaign contract](lean-core-bindings-and-composition.md#campaign-contract).
Scope narrowed by the user on 2026-09-06; no general relation-query or numeric
binding mechanism is required.

## Attachment predicate

Fold `Predicate.isAttached`, `attachedBy`, and `attachedTo` into one attachment
predicate family with explicit direction, attachment-kind restriction where
present, and an optional counterpart. Keep the named forms as macros.

The candidate is the host for the first two existing forms and the attachment
for `attachedTo`. Bare forms test existence; a counterpart can be a Reference
or a quantified Selection. Preserve "enchanted by at least two Auras", both
directions, object/player host domains, and nested counterpart targets that
later clauses can reference. An omitted attachment word must not decide direction.

Carry the existing attachment-kind/type evidence and mention ownership through
the small shared payload. Audit the current asymmetry that the host noun admits
an enchanted player while two predicate forms require an object: preserve
rules-meaningful player hosts and name any corrected refusal. This does not
require a registry of arbitrary relations or changes to ownership/designation
noun projections.

## Current face

Replace `Predicate.isTransformed` with a macro over a generic current-face
predicate. It describes a double-faced permanent with its back face up,
not a history of transformation. The read concerns the whole object, not a
back-face-up component of a merged or melded object [CR#701.27g]. Copying
back-face characteristics alone is insufficient. Keep front/back separate
from the physical face-up/face-down `Status` axis. Retain `turnOver` as the
lower face-switching operation used by the transform macro.

## Completion evidence and deferred work

Re-spell the affected cards and pins, including counted attachments, an
appropriate player host, retained type evidence, nested target publication,
and Mutagen Connoisseur's current-face read. Run `lean/scripts/build`.
Standard constraints apply.

[General relation queries](../maybe/lean-core-relation-queries.md),
[numeric folds and reads](../maybe/lean-core-numeric-bindings-and-aggregation.md),
and [result collections](../maybe/lean-core-result-collections.md) are parked
separately. The current noun projections, aggregates, and random-result forms
remain in this ticket's implementation.
