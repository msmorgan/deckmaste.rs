---
needs: []
---
# Fold attachment predicates and read current face state

Implement two local predicate folds. Apply the
[campaign contract](../done/lean-core-bindings-and-composition.md#campaign-contract).
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

## Landing record

### PROVE

Measured on change `wunsuuqr`, with the unchanged English coverage lock at
20,254 covered identities. `lean/scripts/build` passes the complete syntax,
checker, card bench, and proof gate with warnings fatal. The existing 2,377
card-module declarations and 1,605 proof-module declarations are retained.
A source comparison against the pre-change tree found no card edits and only
the specified macro substitution in the two existing attachment pins; their
asserted refusal lists are unchanged. No covered identity was removed.

Assurance: restored 0, re-spelled 2 (`okFortifiedLand` and
`badFortifiedCreature`), ignored 0, added 20, removed 0. The new attachment
suite covers both directions, an omitted attachment word, counted Auras,
object/player hosts, invalid equipment hosts, retained creature evidence,
and nested target publication with an unavailable-target negative twin.
Current-face pins check the macro expansion, distinct front/back and
face-down forms, the object domain, and absence of token evidence.

`cargo xtask gate --changed` finds no affected Rust crates. Rust parser
roundtrip, lexical ownership, traversal, tie/internal-failure and licensing
gates were not remeasured: this landing changes Lean and documentation only.
No keyword-name or card-name licensing guard was introduced.

### DISCLOSE

`Predicate` falls from 53 to 51 constructors. Separately, the helper vocabulary
adds two `AttachmentSide` alternatives and two `CardFaceSide` alternatives.
The old predicate names remain registered authoring macros. No arbitrary
relation-query mechanism was added.

Deviations and additions: the ticket explicitly authorizes correcting the
player-host asymmetry. An enchanted-player predicate formerly returned an
object-kind mismatch; it now succeeds. Equipped/fortified player hosts remain
invalid and report `attachFits`. Explicit reverse-direction attachment words
receive the same host-type restrictions as forward predicates. Existing
unnamed reverse-direction forms retain their original behavior.

The current-face payload records a whole single double-faced permanent. It
does not implement runtime evaluation or prove the behavior of merged, melded,
or copying game objects; those meanings are documented for the Rust consumer.
The existing Mutagen Connoisseur definition builds through its unchanged macro.
The game-model glossary now defines Attachment, Attachment Host, and Card Face
Side. No unresolved glossary gap or STOP remains for this ticket.

### REPORT

English lock coverage remains 20,254 on `wunsuuqr`; parser construction counts,
selection census, licensing totals, homograph/overlap inventories, and runtime
performance were not remeasured because their sources did not change. No
coverage gain or performance claim is made. Citation validation reports zero
noncompliant strings and zero stale citations; the diff audit's eight citation
sites were checked against their rule texts.
