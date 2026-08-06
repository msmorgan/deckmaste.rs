---
needs: [target-sugar-elaboration, idris-mirror-semantics]
design: true
---
**Retire search-based anaphora in core: an explicit per-scope slot
environment (De Bruijn levels — a telescope), resolved once in lowering.**
Direction owner-settled 2026-08-06: the two grammars diverge here by
design — semantics KEEPS the English-shaped anaphors (`It`/`That(Sort)`,
per [Authored card surface](../../decisions/authored-card-surface.md));
core stops carrying a resolution rule; `deckmaste_lowering` resolves and
records the drop in the divergence ledger. The slot encoding below is the
settled default; the plan prices alternatives only where a latitude names
one.

## Today

`deckmaste_core::Reference` is five binding channels, each with its own
resolution rule and engine threading: the indexed announce channel
(`Target(n)` / `Selection::Targets(n)`); the antecedent stack (`It`,
`That(Sort)`, `Selection::They`/`Them` — R1 nearest-compatible + R2
uniqueness, resolved dynamically at engine eval time); ambient event roles
(`EventObject`/`EventPatient`/`EventActor`/`DefendingPlayer`);
`Bound(Ident)`; `Linked(Ident)`. The anaphoric channel is sound (R1/R2,
proven by the Idris re-emit gate), but the soundness is bought at runtime:
every `resolve/*` path threads antecedent state; the only static check is
the batch Idris gate (canon-scoped, and only the emitter-covered slice —
65/75, wizards never); the Rust side cannot even lint unbound reads
without reimplementing the walk
(`validate-unbound-anaphor-lint`'s stated blocker); and R2's
conservatism refuses recency-resolvable English a static resolver could
commit to.

## Direction (settled default)

- **One statement-level index space per scope region** (an ability body, a
  modal mode, a carried body): absolute slots, telescope invariant — slot
  i's spec/predicate may reference strictly-earlier slots only, the same
  rule `target-sugar-elaboration` settles for announce slots and
  `idris-distinct-position-proof` generalizes. Announced targets keep the
  explicit prefix `0..E` [CR#601.2c]; bound products and choices append in
  textual order; discourse `It`/`That(Sort)` reads lower to slot reads.
- **Provenance rider per slot** (announced target #k, move-product of the
  action at path p, binder choice, event role): slots unify reference
  SYNTAX only. Dynamic denotation stays provenance-dispatched — announced
  slots keep partial-fizzle skip reads [CR#608.2b], move products stay new
  objects [CR#400.7], LKI machinery unchanged. Physically unifying the
  engine's backing stores (announce list, frame antecedents, event data,
  linked records) is follow-up latitude, not day one.
- **NOT relative De Bruijn indices.** Core is interpreted
  (environment-passing) and macro expansion is semantics-side, so no core
  term-into-term substitution exists — distance counting buys nothing here
  and costs renumber-on-insert (unstable diffs and fixtures) plus shift
  arithmetic at carried bodies. Levels are what `Target(n)` already is.
- **The candidate tier stays.** `It` inside
  `Each`/`Distribute`/`With`/`Where`/`Pick` remains the innermost-candidate
  read (de facto depth 0). Depth beyond 0 only if the corpus produces a
  card needing an outer-candidate read from a nested predicate — the design
  phase catalogs this before adding any.
- **Carried bodies are closed.** `Composite{name, body}`,
  `CreateReplacement`'s floating replacement, and delayed-trigger bodies
  each enter a fresh region with an explicit capture list at the boundary —
  never index adjustment on splice; matches creation-time snapshot
  semantics.
- **R1/R2 move to lowering diagnostics.** Recency resolves statically by
  the English reading; ambiguity becomes a per-card lowering error with
  card context, not an eval-time refusal. `validate-unbound-anaphor-lint`
  collapses: its hard problem (a second scope walk to drift) dissolves —
  the walk IS lowering's resolver, and the residual core-side check is a
  bare range check (no walk).

## Latitudes

Event roles as trigger-region slots vs staying ambient nullary; whether
`Bound(Ident)`/`Linked(Ident)` fold into the space or stay named; the
slot-read spelling must coordinate with `target-sugar-elaboration`'s
`Announced(n)` READ-companion call (dual spellings forbidden); how
lowering's computed resolution is validated against the mirror's
certification (differential fixtures vs an emitted resolution oracle).

## Idris

The semantic mirror keeps the whole certification job and more — after
`idris-mirror-semantics` reattaches it, the gate certifies semantic INPUT
(unbound-anaphor soundness R1/R2, target-read range and cardinality,
`Distinct` ranges), and `deckmaste_core` carries no Idris obligations.
This ticket adds the executable counterpart, not a replacement: the
mirror certifies that a card's anaphora resolve uniquely; lowering
computes that resolution — two independent derivations of one rule,
gate-compared, which is the soundness-gate pattern by design. Coverage
inverts in lowering's favor: the resolver runs on every card at lower
time (wizards included), while the Idris gate remains the canon-slice
certifier. The strictly-earlier slot rule generalizes
`idris-distinct-position-proof` on the semantic mirror; a `Fin`-typed
core telescope exists only under the spec's thin-mirror escape hatch,
if an engine-side dependent invariant ever appears.

## Gates

Standard constraints apply. Canon re-lowers green (engine suites + Idris
re-emit gate); byte-stability of core RON across this change is NOT
expected — behavior equivalence is the bar. Fixtures: exile-and-return
(slot 0 announced, slot 1 move-product [CR#400.7]); an insertion negative
fixture (a new producing clause between binder and mention re-resolves at
lowering, never silently rebinds core); a carried body with a capture
list; nested `Where`-in-`Pick`; one multi-sentence anaphora card from
`parse-cross-sentence-anaphora`'s staged shapes (that ticket consumes this
design).
