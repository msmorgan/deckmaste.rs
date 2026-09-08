# deckmaste.rs

> **Reading the card explains the card.**

**Project site:** [deckmaste.rs — Magic: The Gathering rules engine](https://deckmaste.rs/)

deckmaste.rs tests the long-standing intuition that Oracle text can serve as an
executable source language for Magic: it builds the whole path — parser, typed
intermediate form, rules engine — and measures, card by card, where recovery
holds and where it fails.

A rules engine should scale with the number of distinct semantic constructions
in Magic, not the number of cards. Every construction is implemented once, as
shared rules machinery; a card is a composition of constructions, never its own
code. Oracle templating enforces that economy in the text; the engine mirrors
it in code.

Parsing compiles a card's Oracle text into a compact typed intermediate form
that contains its complete recovered meaning. That canonical semantic form is
retained as RON, rendered back to English, or lowered into the core primitives
one shared Rust engine executes. The stack, continuous effects, combat, and the
rest are rules systems, not code attached to individual cards.

![The interactive terminal client mid-game: the hotseat demo's board across
every zone, priority and blocker prompts, and card detail text rendered from
the cards' definitions.](docs/assets/tui-demo.webp)

*`cargo run` — the interactive client playing the hotseat demo, Goblins vs.
Elves.*

---

## Start here

The [guided tour](docs/guided_tour.md) follows one card from Oracle text through
the typed cache, shared rules engine, and Lean workbench boundary. It is the
shortest path through the repository; `cargo run` takes the executable side of
that path into the client shown above.

---

## Design

Magic's card pool is large and open-ended — on the order of thirty thousand
printed cards — and a card is defined almost entirely by how it changes the
rules. The usual way to model this is to give each card its own imperative
implementation: a script or a class that manipulates game state directly.
(XMage, for instance, implements each card as a Java class.) deckmaste.rs treats
Oracle text itself as the source language instead. Recovery turns a sentence
into typed, declarative structure composed from rules concepts — zones, costs,
durations, conditions, the deontic modalities (may, must, can't), and effects —
and the engine interprets that shared vocabulary. Abilities, keywords included,
are usually compositions of those primitives. The small intrinsic remainder is
implemented once as shared rules machinery, never as per-card code.

How much of the card pool can complete both paths — parse → render and parse →
play — is the open, card-by-card question the project exists to measure.

The constraint is workable in large part because mechanics that look distinct on
the card often reduce to the same underlying construct:

- **Zone changes** — drawing, discarding, destruction, sacrifice, and exile are
  one event pipeline that records the cause of each move.
- **Permissions and restrictions** — whatever a card may, must, or cannot do is
  a single deontic construct over a typed action.
- **Event interception** — replacement and prevention effects rewrite events
  that would occur; prohibitions such as indestructibility stop them at the
  same boundary without pretending to be replacements.
- **Abilities** — triggered, activated, and static abilities share one structure
  and one route to the stack or the continuous layers.
- **Counting** — "the number of X," wherever it appears, resolves through one
  evaluation path.
- **Last-known information** — objects that have left or changed are read from a
  snapshot carried by the event, not from stale references.

Rules-bearing code cites the Comprehensive Rules by number (for example,
`[CR#603.4]`), and the repository's `cargo xtask cite check` validates them
against the staged rules text and committed rule-level lock, flagging any that
are stale or unregistered. Recovered card descriptions are statically typed:
unsupported text remains an explicit coverage gap, legitimate ambiguity remains
a visible set of candidates, and ill-formed structure cannot reach runtime
behavior.

---

## What's implemented

Concretely, the engine currently implements:

- the continuous-effects (layer) system, with timestamp and dependency ordering
- replacement effects and last-known information
- state-based actions
- the stack, with casting, targeting, and resolution
- triggered, activated, and static abilities
- a mana system — typed pools, color choices, persistent mana
- combat, including the keyword abilities that affect it
- turn structure, priority, tokens, emblems, and counters
- designations and history-window conditions ("died this turn", storm count)
- player choices surfaced as explicit decision points (what the client drives)

---

## Trying it

`cargo run` launches an interactive terminal client (built on ratatui): a
hotseat game — Goblins vs. Elves — with a live board across every zone, where you
drive priority, targeting, attackers and blockers, ability activations, and mana
payment from the keyboard. On first run it downloads the source snapshot and
generates the demo corpus; subsequent runs reuse those local files.

The engine's behavior is also covered by a test suite that exercises specific
interactions, and `crates/deckmaste_engine/examples/full_game_1k.rs` runs a
thousand complete games.

---

## Architecture

For an existing Magic card's rules semantics, **Oracle text is the authoritative
source input**. Its other printed characteristics — name, mana cost, type line,
power and toughness — arrive alongside it as structured metadata. Parsing the
rules text produces an English syntax tree; recovery compiles that tree into the
canonical normalized intermediate form: the single information-complete
representation from which spelling renders English and lowering compiles the
core AST the engine executes. Persisting that form as RON caches the compilation;
it does not create a competing authority.

```
Oracle text
    ⇅ parse / render
deckmaste_english
    ⇅ deckmaste_spelling: recover / spell
deckmaste_semantics ── deckmaste_lowering: lower ──► deckmaste_core
  canonical typed IR                                      engine AST
  persisted as RON
```

This split is recent (settled August 2026, and still completing — see the
roadmap). Before it, one type family was simultaneously the semantics surface,
the English-facing render/parse target, and the engine's input. Separating the
roles lets each layer be what its job requires: the intermediate grammar carries
the macro system and normalization, the English grammar carries genuine syntax,
and core keeps only what the engine executes.

The workspace's crates, grouped by role — plus a thin root binary that
launches the client:

**The card grammar and its projections**

- **`deckmaste_semantics`** — the typed intermediate rules grammar: the
  canonical normalized hub recovered from Oracle text and persisted as RON in
  today's card, token, and rules-table containers, plus its macro layer (sugar
  desugars at load; the explicit form is always the meaning).
- **`deckmaste_spelling`** — the intermediate ⇄ English relation, built on
  *frames*: English templates with
  typed holes. The frames compile into the lexicon that matches real card text
  and reconstructs the corresponding macro invocations and arguments.
- **`deckmaste_lowering`** — the one-way compile from intermediate form to the
  engine's types. Generated as an exact mirror at the fork, it doubles as the
  divergence ledger: every arm that stops being an identity carries its
  justification in place.
- **`deckmaste_core`** — the engine AST: the typed vocabulary of abilities,
  effects, costs, zones, durations, and conditions, in the shapes the rules
  systems execute.
- **`deckmaste_card`** — the engine's unit of card definitions:
  `Card`/`CardFace` and the face layouts, packaging core's loose primitives
  into a playable unit. Depends on core; core never depends on it.

**The engine and its consumers**

- **`deckmaste_engine`** — the rules engine: game state and the rules systems
  listed above.
- **`deckmaste_plugin`** — the card corpus, its plugin loader, and conformance
  suite.
- **`deckmaste_legacy_render`** — the card-text renderer the client uses today
  and its fidelity harness, kept as a measurement oracle until rendering
  through the spelling layer replaces it.
- **`deckmaste_tui`** — the interactive terminal client, built on ratatui.
- **`deckmaste_migrations`** — the regex-era data pipeline (extract, resolve,
  graduate) that turns oracle text into encodings. Still the production path;
  it becomes a shadow oracle as English-grammar recovery crosses its gates.
- **`deckmaste_noncanon`** — a non-canon proving ground: complete WC99 decks
  and matchup tests that exercise the engine end-to-end.

**Language infrastructure**

- **`deckmaste_english`** — the English grammar for Oracle text; its own
  section below.
- **`macro_ron`** / **`macro_ron_derive`** — the RON macro-expansion layer the
  intermediate grammar is built on.
- **`macro_ron_lsp`** — a small language server for the repo's RON card files.
- **`xtask`** — repository tooling: corpus generation, validation, the
  citation checker, and the English-grammar inspection commands.

---

## The intermediate grammar and its two projections

The load-bearing rules for the split are:

- **Oracle text is the authoritative input; the intermediate form is the
  canonical semantics.** Printed characteristics travel alongside it as typed
  metadata. Once compiled, the normalized IR is the minimal,
  information-complete source for both downstream projections. Persisting it as
  RON avoids reparsing the card pool, but it may contain only recoverable meaning:
  anything parsing would have to invent or rendering would discard —
  implementation-only binder names, layout labels, or sugar choices — is
  decoration, not semantics. The result stays small and game-shaped enough that
  a reader who knows the card, rather than the compiler, can inspect a bad
  translation directly. The built-in macro library supplies that compression:
  macros name recurring rules constructions, while their construction frames
  specify how those names recover from and spell as Oracle text. During
  ingestion, matching those frames against the parsed sentence assembles the
  macro terms that become the persisted card. Expanding the macros still yields
  the full explicit structure whenever it is needed.
- **Spelling is ranked, not bijective.** Several intermediate terms may word
  identically, and one sentence may recover to several candidate terms;
  ambiguity surfaces as candidates rather than being resolved by accident of
  rule order. "Faithful" means semantic round-trip plus canonical wording, not
  byte-exact replay — exact replay within the normalized input domain belongs
  to the English layer below.
- **Sugar never replaces structure.** Surface conveniences (like inline target
  sugar) desugar into the explicit form at load, so every convenience is
  removable and semantics are preserved under reduction.
- **Targets and anaphora have separate channels.** Announced targets are read
  by index (`Target(0)`); anaphora ("it", "that creature") resolve through
  deliberately narrow discourse channels. A legacy lone-target fallback for
  an unbound `It` still remains in the engine and is scheduled for removal.
- **Lowering is total, and divergence is governed.** The intermediate → core
  mapping started as a generated identity; an arm may diverge only with its
  justification written in place, and every variant carries a mapping test
  naming the engine shape it expects.

The crate split has landed: `deckmaste_semantics` and `deckmaste_lowering`
exist, and loaders lower cached intermediate terms to core at load. The curated
canon cache is still written and verified by hand, while the broader production
cache comes from the regex-era pipeline. Structural English recovery is
replacing both paths family by family; the remaining stages are in the roadmap
below.

---

## The English grammar (`deckmaste_english`)

`deckmaste_english` parses Oracle text into a genuine grammatical syntax tree —
constituency structure, not keyword-spotting. A chart parser produces a packed
forest; parts of speech are selected by the grammar slots they fill rather than
assigned by a lexer (Magic overloads its vocabulary too heavily for tagging —
"exile" is a noun and a verb, "target" a verb, a noun, and an adjective); and
legitimate ambiguity stays packed and visible instead of being decided by rule
order. Unknown vocabulary and not-yet-covered text are retained explicitly
rather than guessed at. Two contracts govern the crate (both recorded in
`docs/decisions/`):

- **English clauses are structural.** Dependency, valency, voice, modality,
  and attachment are parsed as English before any Magic meaning attaches. The
  tree is a syntax of English card text, not a semantic encoding wearing an
  English costume.
- **Every production ships its inverse.** A grammar slice lands only together
  with its exact renderer inverse: `cargo xtask english roundtrip` re-renders
  the grammar's normalized, name-bearing input and requires byte equality. The
  round-trip gate is a losslessness check; the recovery census separately says
  how much of that input became grammatical structure.

A family of `cargo xtask english` commands (`inspect`, `bracket`, `shapes`,
`unknown`, `lint`, `roundtrip`) exposes parses, coverage, and uncovered
phrases over the card corpus.

The grammar is evolving from handwritten to **derived**
(`docs/decisions/english-grammar-is-derived.md`). Today each construction
family is written twice — a parse path and a render path held in
correspondence by the round-trip gate. The successor makes one Rust-native
construction declaration the defining authority per family — its AST shape,
typed holes, feature constraints, linearization, and the surface witnesses
needed for exact replay — and generates all three projections from it: parse,
render, and validating constructors. Migration is a ratchet, family by family,
each deleting its handwritten paths in the change that derives it.

Together with the spelling layer, this grammar is the successor to the regex
migration pipeline: parse Oracle text into English syntax, then recover
intermediate terms by unifying against frames. A successful match identifies the
macro and fills its typed arguments; nested matches assemble the RON card. Frame
coverage therefore directly drives next-generation ingestion. It grows in
ongoing rounds that attach `frames:` to macro definitions; the regex pipeline
and the legacy renderer remain as shadow oracles — measurements, not authorities
— until each feature family crosses its gates.

---

## The Lean workbench (`lean/`)

[`lean/`](lean/README.md) is the active design and checking workbench for the
semantic card language. Its syntax is built from ordinary Lean inductives;
checker functions compute bindings, kinds, zones and exact refusal lists.
A `Spelled` card carries a kernel-checked proof that `Card.check` accepts it.
The model guides the shared Rust vocabulary; game execution remains in the
Rust engine.

Run `lean/scripts/build` to build the syntax, checker, macros, printed-card
bench and proof-pin suites with warnings treated as failures. Positive and
negative pins use `decide` to fix the exact result of a checker call.
`Semantics/Check/Facts.lean` supplies the registry-derived keyword-action,
keyword-ability, counter, designation and frame-subtype facts;
`cargo xtask facts check` detects generated-table drift.

[Lean is the workbench](docs/decisions/lean-is-the-workbench.md) records the
succession. Idris is retained as a frozen reference and legacy emitter target.
The Rust-to-Lean card-soundness gate is still
[tracked work](docs/tickets/planned/lean-card-soundness-gate.md), rather than a
claim made by the current workbench build.

---

## Present state and roadmap

The implemented card set is a deliberate vertical slice rather than a complete
pool: development has prioritized the correctness of the rules systems over
breadth of card coverage. A curated cache of real cards is encoded and verified
by hand while Oracle recovery expands toward producing it directly, and the
grammar continues to cover the remaining mechanics (`docs/rules-taxonomy.md`
records that plan). The near-term work runs in three strands:

1. **Completing the grammar split.** Landed: the crate splits and renames, the
   semantics fork, the lowering crate, and the loader repoint. Remaining, in
   rough order: moving prose rendering onto the spelling layer and retiring the
   legacy renderer; stripping the macro machinery out of core;
   the intermediate vocabulary (bare engine variants stop being recoverable
   syntax, made near-zero-churn by identity macros); consolidating frames into
   `deckmaste_spelling`; inline target sugar and its scope elaboration; and
   connecting the intermediate grammar to the Lean card-soundness gate.
2. **Deriving the English grammar and growing recovery.** The constructicon
   migration above, alongside the frame-coverage rounds — with the regex
   pipeline and legacy renderer as shadow oracles until recovery replaces them
   family by family.
3. **Engine breadth.** `docs/tickets/census.md` tracks the open card shapes
   (transform, saga, adventure, split, and the rest of the layout table),
   keyword abilities, keyword actions, and ability words. Priority order:
   the engine happy path (the normal resolution path of ~90% of abilities),
   then oracle-text coverage, keyword authoring, and convenience macros, with
   the noncanon suite growing alongside.

`docs/tickets/` is the working queue (folder = status; `kata kanban ready`
lists the next claimable items), and `docs/decisions/` records the design
contracts the roadmap is built on.

---

## Motivation

I'm drawn to language — natural, formal, and programming alike — and a Magic card
has always read to me as a structured data language disguised as English
instructions. Recovering that structure from the cards is something I'd wanted to
try for years; this is that project. It is also why the design looks the way it
does: if a card is structured data under its prose, the engine should treat it as
such.

That recovered representation also makes new keywords and one-off abilities
assemblies of existing primitives rather than new engine behavior — the same
property that should let the plugin architecture support custom sets, a
longer-term aim. The hub is checked in both directions: rendering back to the
Oracle sentence measures language fidelity, while lowering into core produces
playable behavior. The client still uses the partial legacy renderer today;
grammar-backed rendering replaces it as construction coverage grows.

The same boundary leaves the future custom-set authoring interface open. A
plugin can accept prose, check it, and compile it into the same RON form; whether
plugins expose prose, direct RON, or both is an authoring-interface decision, not
an engine architecture change.

---

## Getting started

The committed vertical slice's tests run without downloading external data:

```sh
cargo test --workspace     # the committed Rust test suite
```

`cargo run` launches the interactive hotseat demo. On a clean checkout its
first-run bootstrap downloads the minimal source snapshot and generates the
gitignored Wizards corpus; subsequent runs reuse it. The full card pipeline and
the citation checker require the complete dataset, which is not committed, as
it is Wizards of the Coast property (see below):

```sh
scripts/fetch_data                       # Scryfall Oracle Cards, MTGJSON references, and CR
cargo xtask generate plugins/wizards     # build the stub corpus from it
cargo xtask cite check                   # validate CR citations
```

The repository uses jj rather than git; see `CLAUDE.md` for the workflow
conventions.

---

## License & fan content

The code is licensed under [PolyForm Noncommercial
1.0.0](https://polyformproject.org/licenses/noncommercial/1.0.0/): anyone may
use, modify, and share it for any noncommercial purpose, and commercial use
requires a separate license. The full text is in [`LICENSE.md`](LICENSE.md).

This is also unofficial Fan Content permitted under the [Fan Content
Policy](https://company.wizards.com/en/legal/fancontentpolicy). It is not
approved or endorsed by Wizards. Card names, oracle text, and the Comprehensive
Rules remain the property of Wizards of the Coast; portions of the materials
used are property of Wizards of the Coast. © Wizards of the Coast LLC. The raw
card data and rules text are never committed to this repository.
