Fold the parallel `macro_ron::v2` declaration dialect back into the ordinary
macro-def model. The v2 module reified a design shorthand into a second source
language — its own `ron_options` dialect, its own `DeclarationKind` enum, and
a separate reader bypassing `MacroDef` — when the intended change was small:
`KeywordAction` and friends stay meta-macros in the existing reader, and the
v2 additions (`spelling`, `grammar`) land as a few new fields or per-def
custom metadata on the existing declaration model.

End state:

- One declaration reader and one RON dialect. The separate v2 source-type
  layer (`macro_ron/src/v2.rs` source values, its private `ron_options`) is
  deleted; `plugins/builtin_v2/macros/stubs/**` files migrate mechanically to
  the unified form.
- The builtin-v2 spelling/grammar ADR's *field semantics* are unchanged:
  positional `Param(n)` only, spelling as the holey semantic frame, grammar as
  lexical category + valence + realized surfaces, attestation-as-provenance.
  Only the packaging moves. Record a dated supersession note in that ADR for
  the separate-dialect packaging.
- This does not reinstate v1's `template:` language, text offsets, or `kinds:`
  registry — the banned content models stay banned; reuse of the reader is
  packaging, not content.
- The normalized-row boundary the english_v2 parser environment consumes may
  survive as an internal representation; what is removed is the parallel
  *source* dialect, not the one normalized parser boundary.
- No new dependency edges; the `macro_ron -> deckmaste_features` cutover debt
  must not grow (shrinking it here is in scope if convenient).

Sequencing: coordinate with the in-flight stage-5 Plan 10 work, which authors
stubs in the current dialect — land after its stub authoring settles, with a
mechanical migration of whatever stubs exist by then. Coverage, byte-exact
laws, and the ratchet are unchanged throughout; standard constraints apply.
