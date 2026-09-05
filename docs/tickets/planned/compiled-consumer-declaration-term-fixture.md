---
needs: []
---
**The compiled-consumer fixture has no declaration-term family, so the whole
`Leaf::DeclarationTerm` boundary is unexercised by
`deckmaste_construction`.** `crates/deckmaste_construction/tests/compiled_consumer.rs`
carries `declaration_noun_fixture` and `declaration_verb_fixture` but generates
no `declaration_term` codec, even though `deckmaste_english_v2` runs every
keyword ability through one. Nothing in the compiled-consumer suite covers the
declaration-term codec's `from_environment`/`from_reading` pair, its scanner
arm, or the fixed-keyword parameter the arm now carries
(`english-v2-affinity-quality-surface`, 2026-09-05, whose parameter carrier is
covered only by `deckmaste_english_v2`'s own witnesses).

Mint a `declaration_term_fixture` module beside the existing two: synthetic
keyword-ability declarations (one plain, one declaring a
`parameter: Quality(preposition: (…))`), a `generate declaration_term` codec, a
construction whose keyword role is `checked by` a `role.value` field-check
against the declared marker, and a `run()` asserting the positive parse, the
mismatched-preposition rejection, and the unmarked-keyword rejection. Cover the
`role.value` projection's unknown-role validation error the same way the sibling
`verb_frame_role_prepositions` projection should be covered.

Standard constraints apply.
