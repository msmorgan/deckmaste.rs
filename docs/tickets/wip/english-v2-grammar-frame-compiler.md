---
needs: [english-v2-grammar-migration-design]
---
# Compile declared frame schemas with the existing Earley engine

Implement the runtime-frame representation in [the migration design](../../english-grammar-design.md#runtime-frame-representation).
Keep `constructions!` as the owner of typed ASTs, checked builders, rules,
renderers and total visitors. Use the existing Earley engine; ordered-choice,
PEG, recursive-descent and a fallback parser are not substitutes.

Unify core/plugin frame atoms at `construction_core::macro_def`, with grammatical
relation, category, fixed lexical marker, marked Complement and optional item.
Generate a checked heterogeneous frame sequence and a schema registry. Prepare
finite `(schema, position)` rules in an environment-owned rule table; adapt the
engine's static RHS borrow, scanner dispatch, root IDs and build dispatch.
Retain source-bearing lexical leaves and declaration identity. Dynamic grammar
IDs are internal and cannot add Construction identities or preference weights.
Reject unknown roles/categories/lexical references at environment construction.

This ticket introduces the compiler capability through a compiled consumer,
without registering a second production grammar. The existing tail-codec path
is deleted by `english-v2-lexeme-owned-verb-frames`, its immediate consumer.
No speculative general-purpose runtime extraction is required.

Acceptance through `deckmaste_construction/tests/compiled_consumer.rs`: two
lexemes use one schema; a new ordered NP/marked-PP/measure schema works by data
alone; required/optional items render with exact ownership; wrong child category,
missing required child and a schema from the wrong head reject; a nonempty
frame-part sequence coordinates for all three existing Coordinators. Include
the inherited `FrameComplementPair` fixture gap. The feature-independent engine
suite still exercises left recursion, forest completeness on its fixtures and
structured failures. Report prepared-rule count and preparation/parse cost on
these fixtures. This is the first check of the runtime-rule engineering choice;
report a demonstrated obstacle instead of adding another per-tail codec.

Standard constraints apply. Production correspondence and the applicable
[obligations](../../english-grammar-migration-obligations.md) are part of this
ticket; re-spell existing tests by their independently justified outcomes.
