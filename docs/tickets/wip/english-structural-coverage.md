---
needs: []
design: true
---
**[design] Finish supported-corpus English coverage through typed grammatical
structure and declarative lexical signatures.** This ticket supersedes
`english-unknown-min3`; the old campaign's maximum `UnknownPhrase` leaf length
was a useful work-queue view, but the wrong correctness target.

A parser can make that number improve merely by splitting one opaque span into
several shorter opaque leaves. Conversely, a long catalog name or other
explicitly licensed opaque lexical item is not necessarily a grammar failure,
while a one-word unknown predicate can be one. Structural completeness is about
which grammatical role fell back and how much source remains recovered, not the
length of each recovery leaf.

This work follows [English clauses are structural](../../decisions/english-clauses-are-structural.md)
and [Regular vocabulary tables](../../english-regular-vocabulary-tables-design.md).
In particular, clause form, predicate valency, complement/adjunct role, voice,
modality, and attachment remain explicit. Ordinary vocabulary stays
slot-directed and data-backed; this ticket does not move part-of-speech choice
into tokenization or add a generic clause/predicate escape hatch.

## Audit baseline

The 2026-07-22 audit used the same 31,685 supported faces as the original
campaign. The intervening grammar slices made real progress, reducing all
unknown leaves from 24,177 to 15,252 and leaves longer than three words from
18,793 to 14,175. The maximum remained 69 words. Those figures remain useful
for triage, but do not define completion.

The audit also found that the previous checks could not enforce their stated
result:

- `cargo xtask english unknown --min-words 4 --limit 0` prints the aggregate
  count, suppresses all detail rows, and exits successfully for any count.
- The supported-card round-trip test proves source-independent rendering, not
  structural coverage: an `UnknownPhrase` owns enough spelling to round-trip.
- That test returns successfully when its local corpus cannot be loaded.

Keep round-trip as an independent invariant. Replace the old length criterion
with a recovery gate that cannot be satisfied by repartitioning the same opaque
text.

## Settle the recovery contract first

Before extending productions, make permitted opacity distinguishable from
parse-failure recovery and expose a machine-readable census. The exact type
shape is part of the design dialogue, but the contract is:

- report recovery by grammatical role (`Clause`, nominal material, activation
  cost, keyword argument, modal header, and embedded rules) and by total
  recovered source tokens; leaf count and maximum leaf length are secondary
  diagnostics only;
- count a recovered span exactly once at its outermost responsible role, so
  nesting or splitting cannot improve the gate;
- keep explicitly licensed opaque lexical identities separate from recovery of
  unsupported syntax;
- make the corpus command fail when an asserted budget is exceeded, and make
  the release corpus test fail when its required data is unavailable;
- check in the accepted baseline/budgets so a grammar slice cannot silently
  trade one recovery role for another or increase total recovered text.

At campaign completion no supported face may require whole-sentence/clause,
modal-header, embedded-rules, activation-cost, or keyword-argument recovery.
Any remaining lexical opacity must be represented at a documented lexical
boundary and reported separately from grammar failure. Do not achieve the gate
by adding opaque semantic leaves or weakening slot constraints.

## Generalize predicate frames

The current recognition features derive nearly all predicate behavior from
three name checks: every verb except `be` accepts a direct object, only `have`
requires one, and `do` is the proform. Grammar rules then offer the same noun,
adjective, prepositional, infinitival, scalar, and particle dependents to almost
every verb. This is too permissive and conflicts with the structural-clause
decision.

Introduce one declarative source of predicate-frame metadata, supporting more
than one frame per lexical identity where English permits it. It must describe
at least:

- forbidden, optional, and required direct objects;
- indirect, adjective, prepositional, infinitival, ability, scalar, and other
  supported complement shapes;
- selected prepositions versus freely attaching PP adjuncts;
- licensed bare-nominal temporal/manner adjuncts;
- licensed verb-particle pairs; and
- proform and other genuinely exceptional lexical behavior.

Named/irregular vocabulary, regular vocabulary, and catalog-derived keyword
actions must enter recognition through the same frame interface. Recognition
and lowering consume the same metadata; do not duplicate classification by
matching `Vocab` variants after lowering.

Migrate the campaign's existing surface patches into that model:

- remove the duplicated `combat | time | turn` / `way` nominal-adjunct checks
  and the `attack | block` attachment check;
- replace globally attachable `in | out` particles with licensed verb-particle
  pairs;
- either make prepositional and indirect-object predicate complements reachable
  through real frames or remove representations that the grammar cannot build;
- add negative fixtures proving that an intransitive frame cannot acquire an
  object and that a selected complement is not silently reclassified as an
  adjunct.

Finite closed classes and real irregular morphology remain explicit code where
appropriate. The target is open-ended lexical behavior, not the elimination of
all enums or spelling tables.

## Give keyword abilities argument signatures

Replace the `protection from` / `affinity for` name checks and first-token
plausibility heuristic with declarative keyword-argument signatures associated
with catalog entries. Signatures must be able to express no argument, quantity,
typed cost or symbol sequence, noun phrase, PP with permitted preposition,
embedded ability, and other corpus-motivated shapes. A keyword must obey the
same signature alone and inside a comma/semicolon-separated keyword list.

As part of that migration:

- replace raw-string `Cost::SymbolList` with the existing structured
  `OracleSymbol`/symbol-sequence representation;
- let quoted or embedded abilities occupy the grammatical phrase/PP-object
  slots that license them, rather than manually splicing only `with "..."`;
- report an unsupported signature as keyword-argument recovery instead of
  accepting it because another list item supplied a separator.

## Move remaining lexical facts into metadata

- Record comparison capability/kind in adjective vocabulary metadata instead
  of matching `fewer | greater | less | more | other` in grammar control flow.
- Carry the canonical lemma/forms of CR-derived keyword actions in catalog or
  vocabulary metadata instead of rewriting only `prepared` to `prepare` while
  resolving a catalog entry.
- After the frame/signature migrations, audit named regular `Vocab` entries and
  keep named variants only where grammar semantics or irregular forms need a
  stable identity. Coordinate any mechanical module split with
  `english-split-word-module`; do not create per-part-of-speech registries.

## Retire the inactive parser tree

The crate's active module tree no longer includes `ast.rs`, `parser.rs`,
`render.rs`, `source_debug.rs`, or `catalogs.rs` (5,198 lines at audit time).
Confirm that no tooling consumes them, then delete them rather than maintaining
a second searchable implementation whose apparent special cases are not live.

## Campaign workflow

Use the role/token recovery census as the live queue. Take coherent,
high-frequency grammatical families, but require each slice to do one of:

- add a reusable production or typed construction;
- populate an existing declarative frame/signature vocabulary; or
- add a documented genuine lexical exception through the shared metadata path.

Each slice includes causal positive and negative syntax tests, source-free
rendering, the enforced corpus recovery gate, and before/after recovery totals
by role and recovered token count. Measure release-parser behavior when chart
search changes materially. Standard constraints apply.

## Completion

- The old `english-unknown-min3` ticket and leaf-length completion claim are
  gone; length remains an optional diagnostic view only.
- Corpus loading and recovery budgets are enforceable failures, and a focused
  test proves that splitting one recovery span cannot improve the gate.
- All supported faces satisfy the recovery contract above, with no structural
  recovery hidden by source-independent round-trip success.
- Predicate valency, complements, nominal adjuncts, and particles use the shared
  frame model; keyword arguments use shared signatures; the audited name/string
  branches are gone or documented as genuine closed-class/irregular cases.
- The inactive parser tree is removed.
- `cargo test -p deckmaste_english`, `cargo test -p xtask english::`, the release
  supported-card gates, and the relevant workspace checks pass. Clippy is clean
  under the project's deliberate unboxed-grammar-enum policy; do not silence
  new findings or box recursive grammar data merely to satisfy a size lint.
