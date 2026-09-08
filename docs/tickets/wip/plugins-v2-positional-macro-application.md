---
needs: []
---
**A named-signature macro may be applied positionally.** `plugins-v2-cosmetic-conversion`
found that `hasType(Creature)` and `colorIs(Red)` are refused
(`Expected colon`): `read_args` reads `Params::Named` through a map
visitor, so the one-argument positional classification the dialect gives
constructors never reaches macro invocations. `Params::Named` is an
unordered map, so positional application needs a declared parameter order:
give `MacroDef` an ordered parameter list (declaration order in the file is
the order; the drift test's binder order is the contract for ported
helpers) and let `read_args` accept a sequence mapped by that order, with
the mixed form still refused. Then rewrite the 176 named one-field macro
invocations in card source positionally (source-to-source, comments
untouched, `lean/Generated` byte-identical as the oracle). Standard
constraints apply.

## Landing record (2026-09-08)

All figures below were measured at change `ttnnsnkquowk`. The untouched
English-v2 coverage lock contains 20,254 covered identities; no coverage lock
is in play for this macro/corpus change.

### PROVE

**No silent loss.** The pre-fix regression test
`named_parameters_may_invoke_positionally_in_declaration_order` failed on
`hasType(Creature)` with ron's `ExpectedMapColon` / “Expected colon”. It passes
after the reader change and proves expansion plus positional-provenance
round-trip. The declaration-order test uses reverse-alphabetical parameter
names, so a sorted or hash iteration cannot pass accidentally. Both mixed
forms are negative witnesses.

The source conversion rewrote exactly 176 invocation sites in 74 card files.
Its declaration-driven scanner asserted that comments were byte-identical in
every changed file; a second scan found 0 remaining eligible named calls.
`cargo test -p deckmaste_semantics_v2 --test corpus` passed all 6 corpus laws.
The live Lean oracle proved `plugins_v2/canon` 118/118 and
`plugins_v2/testing` 2/2 before and after the rewrite, and recursive comparison
of the two `lean/Generated` trees produced no diff.

**Structural laws.** The 146-test `macro_ron` suite covers the call grammar,
declaration order, expansion, remembered spelling, and mixed-form refusal.
The refreshed 17-package reverse-dependency gate passed, as did the four
Rust↔Lean syntax drift tests, generated-fact drift, strict clippy, and the
80-job Lean build.

**No word-naming.** The reader is generic over registered kinds, macro names,
and declared parameter order. The one-off converter selected candidates from
declaration files and their binders and was deleted after use; no lexical,
construction, verb, noun, preposition, or card-name guard entered production.
The full gate includes the production zero-word-naming-checker and environment
load-error tests.

### DISCLOSE

- `Params::Named` now retains `(Ident, ParamType)` pairs in file declaration
  order and rejects duplicate declaration keys during deserialization.
- With `reading_positional_arguments` enabled, a named-signature macro call is
  classified from its argument-list shape before ron consumes its one-shot
  variant visitor. Positional arguments map to declaration names for
  `Param(name)` substitution, while remembered expansion provenance serializes
  the original positional form. Named calls remain named; mixed forms fail.
- The default switch remains off, and v1 consumers that treat named signatures
  as sets still canonicalize them before comparison.
- The 176 converted calls span 23 current-tree macro heads:
  `mana` 34, `hasType` 26, `and` 23, `up` 15, `static` 14, `inZone` 13,
  `sequentially` 11, `keyword` 7, `hasSubtype` 4, `written` 4, `not` 4,
  `compound` 3, `or` 3, `named` 3, `abilityHead` 2, `down` 2, `perform` 2,
  and one each of `colorIs`, `otherThan`, `exists`, `themVerbed`, `countOf`,
  and `gap`. The earlier dialect handoff forecast 24 heads; the authoritative
  current declarations contain 23 eligible heads while retaining the ticket's
  exact 176-site total.

**Deviations and additions.** Three tests were added: the original failing
one-field call and provenance witness, a multi-parameter declaration-order
witness, and mixed-form refusal. The existing remembered-default test was
extended to cover omitted and explicit positional defaults. Two existing
assertions were adapted to the ordered collection API, and the computed gate
found one further test-only xtask adapter that the earlier `cargo check` could
not compile. The v1 plugin and legacy renderer received mechanical iterator
adapters. Public reader docs, the xtask canonicalization comment, and
`semantics-v2.md` were updated because the previous text contradicted the new
declared-order contract. No production converter or new corpus ratchet remains.

There were no STOPs and no recorded-ruling contradiction. This landing changes
no English selection identity, construction census, licensing checker,
homograph inventory, or form-literal/vocabulary overlap inventory. No term it
needed is missing from the game-model or oracle-English glossaries; “named
signature” and “declaration order” are local macro-reader vocabulary documented
at the API and in `semantics-v2.md`.

**Assurance counts.** Restored 0 tests, re-spelled 0 tests,
ignored-with-blocker 0, added 3, removed 0. Separately, 176 existing card-source
invocations were re-spelled without changing their expanded values or comments.

### REPORT

- `cargo fmt --all --check`: clean.
- `cargo test -p macro_ron`: 146 passed, 0 failed, 0 ignored.
- `cargo test -p deckmaste_semantics_v2 --test corpus`: 6 passed, 0 failed.
- `cargo xtask gate --changed --run`: the final refreshed 17-package closure
  passed. Its command was `cargo test -p macro_ron -p
  deckmaste_construction_core -p deckmaste_construction -p
  deckmaste_english_v2 -p deckmaste_lexical_source -p deckmaste_semantics -p
  deckmaste_lowering -p deckmaste_plugin -p deckmaste_engine -p
  deckmaste_legacy_render -p deckmaste_migrations -p deckmaste_noncanon -p
  deckmaste_semantics_v2 -p deckmaste_spelling -p deckmaste_tui -p deckmaste
  -p xtask` (about 124 s wall). A pre-refresh closure found the test-only xtask
  `Params::Named::keys()` adapter; after that fix both the pre-refresh and
  refreshed closures passed.
- The matching strict command, `cargo clippy` over those 17 packages with
  `--all-targets -- -D warnings`, passed in 10.89 s after extracting the new
  argument binder to satisfy the 150-line function limit.
- `cargo xtask lean-check plugins_v2/canon plugins_v2/testing`: 118/118 and
  2/2 before and after conversion and after refresh; 67.7 s cold, then 0.4 s
  and 0.8 s warm. A recursive `diff` of the baseline and refreshed-final
  `lean/Generated` trees was empty.
- `cargo test -p deckmaste_semantics_v2 --test lean_drift`: 4 passed.
  `lean/scripts/build`: success, 80 jobs. `cargo xtask facts check`: both
  generated files up to date.
- `cargo xtask cite check --list-noncompliant`: 0. `cargo xtask cite check`:
  15,848 citations checked against `cr.txt` effective 2026-08-07, 0 stale.
  The feature diff audit selected 0 citation sites.
- Performance advisory: no production English-v2 coverage command was run,
  because this change touches neither its corpus nor its lock. The 16.26 s
  quiet-host ceiling, per-byte CPU telemetry, construction census, homograph
  inventory, and overlap inventories therefore have no new measurement to
  report. Final host: 24 logical CPUs; load average 3.97 / 6.20 / 6.11; the
  applicable macro, corpus, and Lean commands used their defaults.
