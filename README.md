# deckmaste.rs

deckmaste.rs is a Magic: The Gathering rules engine written in Rust. Its central
design decision is to represent cards as declarative data interpreted by a shared
rules engine, rather than as per-card scripts. A small typed language describes
what each card does, and the engine implements the rules themselves — the stack,
triggered abilities, continuous effects, combat, and so on — applying them to
that description.

> I'm fond of language — natural and programming alike — and I'm fond of Magic:
> The Gathering. I've long wondered if the rules text of Magic cards was a
> structured data language in disguise. This is that project.

![The interactive terminal client mid-game: the hotseat demo's board across
every zone, priority and blocker prompts, and card detail text rendered from
the cards' authored definitions.](docs/assets/tui-demo.webp)

*`cargo run` — the interactive client playing the hotseat demo, Goblins vs.
Elves.*

---

## Design

Magic's card pool is large and open-ended — on the order of thirty thousand
printed cards — and a card is defined almost entirely by how it changes the
rules. The usual way to model this is to give each card its own imperative
implementation: a script or a class that manipulates game state directly.
(XMage, for instance, implements each card as a Java class.) deckmaste.rs adopts
the opposite constraint: there is no per-card code. A card is composed from
atomic primitives that each stand for a rules concept — zones, costs, durations,
conditions, the deontic modalities (may, must, can't), and effects — and the
engine implements the rules that interpret them. Abilities, keywords included,
are themselves compositions of these primitives, not behavior added to the
engine. The vocabulary of primitives is shared by every card, so it grows at the
level of rules concepts rather than individual cards.

How much of Magic that constraint can capture is the open question the project
exists to explore.

The constraint is workable in large part because mechanics that look distinct on
the card often reduce to the same underlying construct:

- **Zone changes** — drawing, discarding, destruction, sacrifice, and exile are
  one event pipeline that records the cause of each move.
- **Permissions and restrictions** — whatever a card may, must, or cannot do is
  a single deontic construct over a typed action.
- **Replacement effects** — anything that alters or prevents an event, like
  indestructibility, is one family applied where the event would occur.
- **Abilities** — triggered, activated, and static abilities share one structure
  and one route to the stack or the continuous layers.
- **Counting** — "the number of X," wherever it appears, resolves through one
  evaluation path.
- **Last-known information** — objects that have left or changed are read from a
  snapshot carried by the event, not from stale references.

Rules-bearing code cites the Comprehensive Rules by number (for example,
`[CR#603.4]`), and a companion skill,
[mtg-rules](https://github.com/msmorgan/mtg-rules), checks those citations
against a fixed snapshot of the rules text, flagging any that are stale or
unregistered. Card descriptions are statically typed, so
an ill-formed description fails to parse or validate rather than misbehaving at
runtime.

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

A card here has one authoritative representation — the **authored form** — and
every other representation is a projection of it. English text is an input to
recovery and an output of rendering, never stored truth; the AST the engine
executes is a compiled artifact. Two translation layers connect the three
representations:

```
                     spelling                           lowering
deckmaste_english ◄════════════► deckmaste_authoring ─────────────► deckmaste_core
  the English         two-way        the authored rules   one-way      the engine AST
  grammar; text ⇄     ranked         grammar + macro      total        the rules
  syntax tree only    relation       layer                compile      execute
```

This split is recent (settled August 2026, and still completing — see the
roadmap). Before it, one type family was simultaneously the authoring surface,
the English-facing render/parse target, and the engine's input. Separating the
roles lets each layer be what its job requires: the authored grammar carries
the macro system and author conveniences, the English grammar carries genuine
syntax, and core keeps only what the engine executes.

The workspace's crates, grouped by role — plus a thin root binary that
launches the client:

**The card grammar and its projections**

- **`deckmaste_authoring`** — the authored rules grammar: the single source of
  truth every card-content container is written in — card files, token files,
  and the engine's rules tables — plus the macro layer and normalization
  (sugar desugars at load; the explicit form is always the meaning).
- **`deckmaste_frames`** — the authored ⇄ English relation (slated to be
  renamed `deckmaste_spelling`), built on *frames*: English templates with
  typed holes, compiled from a macro definition and unified back against real
  card text.
- **`deckmaste_lowering`** — the one-way compile from authored form to the
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
- **`deckmaste_plugin`** — the card corpus, its plugin loader and conformance
  suite, and the legacy card-text renderer (what the client prints today; kept
  as a measurement oracle until rendering through the spelling layer replaces
  it).
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
  authored grammar is built on.
- **`macro_ron_lsp`** — a small language server for the repo's RON card files.
- **`xtask`** — repository tooling: corpus generation, validation, the
  citation checker, and the English-grammar inspection commands.

---

## The authored grammar and its two translations

The contract for the split lives in
`docs/decisions/authoring-spelling-lowering.md`; the load-bearing rules are
worth stating here because they shape everything else:

- **Authored form is the source; everything else is derived.** The canonical
  form may contain only what round-trips: anything one translation direction
  would have to invent and the other would have to discard — named binder
  variables, layout labels, the author's sugar-versus-explicit choice — is
  decoration and cannot be stored. Card files are data, not a language.
- **Spelling is ranked, not bijective.** Several authored spellings may word
  identically, and one sentence may recover to several candidate terms;
  ambiguity surfaces as candidates rather than being resolved by accident of
  rule order. "Faithful" means semantic round-trip plus canonical wording, not
  byte-exact replay — byte-exactness belongs to the English layer below.
- **Sugar never replaces structure.** Author conveniences (like inline target
  sugar) desugar into the explicit form at load, so every convenience is
  removable and semantics are preserved under reduction.
- **Announced targets are never pronouns.** Targets are read through an
  indexed channel (`Target(0)`), and anaphora ("it", "that creature") resolve
  through separate, deliberately narrow discourse channels — the invariant
  that keeps target reference sound enough to prove things about.
- **Lowering is total, and divergence is governed.** The authored → core
  mapping started as a generated identity; an arm may diverge only with its
  justification written in place, and every variant carries a mapping test
  naming the engine shape it expects.

The fork has landed: `deckmaste_authoring` and `deckmaste_lowering` exist, and
the loaders now parse card containers as authored terms and lower them to core
at load. The remaining stages are in the roadmap below.

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
  parsed text and requires the original bytes back. The round-trip gate is
  what keeps the grammar honest about what it actually covers.

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
authored terms by unifying against frames. Frame coverage grows in ongoing
rounds that attach `frames:` to macro definitions; the regex pipeline and the
legacy renderer remain as shadow oracles — measurements, not authorities —
until each feature family crosses its gates.

---

## The Idris gate (`idris/`)

`idris/` holds a dependently-typed model of the card grammar, written in
Idris 2. It began as a standalone probe and now runs as a soundness gate:
`cargo xtask idris-check` re-emits authored card data into the model and
type-checks it. The grammar is built from closed enums refined by total
type-functions, so whole classes of nonsense are unrepresentable rather than
merely rejected — there is no ill-formed term to write down in the first
place. For example: the "that card" anaphor cannot be named in a trigger whose
event supplies no object; a counter's carrier — a player versus a permanent —
is fixed by the counter's kind; a target index cannot exceed what was
announced. Each is an invariant the Rust types leave to a runtime check.

The model is deliberately not a second engine — it models the *grammar*, not
the rules, and runs no games; runtime semantics live in one engine so two
implementations cannot drift. Its obligations are author-mistake proofs
(unbound-anaphor soundness, target-index range and cardinality, distinctness
constraints), which is why, as part of the grammar split, the mirror is being
reattached to the authored grammar: `deckmaste_core` carries no proof
obligations at all. What carries over to the Rust side is the *shape* of a
sound data model — which distinctions earn their own type, which constructors
are really one parameterized constructor, which invariants ought to hold — and
a mechanic the model cannot express cleanly marks a gap in the shared
vocabulary of primitives. Like the rest of the repository, its rules-bearing
code cites the Comprehensive Rules by number and is checked by
`cargo xtask cite`.

---

## Present state and roadmap

The implemented card set is a deliberate vertical slice rather than a complete
pool: development has prioritized the correctness of the rules systems over
breadth of card coverage. A curated set of real cards is encoded by hand and
graduated through the data pipeline, and the grammar continues to expand toward
the remaining mechanics (`docs/rules-taxonomy.md` records that plan). The
near-term work runs in three strands:

1. **Completing the grammar split.** Landed: the crate splits and renames, the
   authoring fork, the lowering crate, and the loader repoint. Remaining, in
   rough order: moving prose rendering onto authored terms and retiring the
   legacy renderer's engine hooks; stripping the macro machinery out of core;
   the author-surface vocabulary (bare engine variants stop being author
   syntax, made near-zero-churn by identity macros); consolidating frames into
   `deckmaste_spelling`; inline target sugar and its scope elaboration; and
   reattaching the Idris mirror to the authored grammar.
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

`docs/tickets/` is the working queue (folder = status; `scripts/todo ready`
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

Two other interests run through the project. The first is what the representation
makes possible: new keywords and one-off abilities are cheap to add, since they
are assembled from existing primitives rather than written as new engine behavior
— the same property that should let the plugin architecture support custom sets,
a longer-term aim. And because a card's abilities are structured data rather than
prose, the representation runs in reverse too: a renderer reconstructs
approximate card text from the definitions — still partial, but real, and it's
what the client prints on each card.

The second is method. I architected the system: its domain model, crate
boundaries, verification strategy, and review process. The engine was also a
way to develop a working practice for agentic development in a domain with
little tolerance for imprecision: Magic's rules resolve to definite outcomes,
so an encoding is either correct or it produces a wrong result in play. That
makes the domain a demanding test of whether AI agents can be directed toward
sustained correctness across many interacting cases rather than a plausible but
shallow approximation. Most of the implementation was produced under that
workflow, within that architecture and under my review. Version control is jj
(Jujutsu), chosen for its
first-class conflicts, freely rewritable history, and lightweight parallel
workspaces — a good fit for several agents working at once — and much of the
repository's tooling follows from that, including a per-ticket
claim/workspace/integrate workflow and immutability guardrails that let an agent
rewrite its own work but not shared history. Conflict resolution — the usual tax
on parallel work — has stayed cheap: because jj records conflicts in commits
instead of blocking on them, the workflow's `repair`/`converge`/`resolve`
commands let an agent clear a tangle in a few minutes rather than the half hour
it used to take, so far without a failure.

The choice of tools is part of the same experiment. A language model is only
passingly familiar with RON, fish, and jj; it knows their well-trodden neighbors
— JSON and serde, bash, git — far better. Each is nonetheless small, readable,
and more than expressive enough for what this project asks of it. Working in them
deliberately does two things: it surfaces the gaps in the model's knowledge,
which I then have to close, and it gives me grounds to impose my own conventions
rather than the idioms it absorbed passing benchmarks. Left to its own devices,
the same work would have arrived in Python, repackaging MTGJSON into
differently-shaped JSON.

Rust is the lone popular pick in that set — so, preempting the obvious *why not
zig/nim/jai/DreamBerd?*: the answer is fit. Sum types with exhaustive matching
suit an engine that mostly enumerates cases; naive code runs fast enough to keep
the implementation plain for now; and moving the card tooling to Rust was roughly
a 100× speedup.

---

## Getting started

The committed vertical slice's tests run without downloading external data:

```sh
cargo test                 # the engine's interaction suite
```

`cargo run` launches the interactive hotseat demo. On a clean checkout its
first-run bootstrap downloads the minimal source snapshot and generates the
gitignored Wizards corpus; subsequent runs reuse it. The full card pipeline and
the citation checker require the complete dataset, which is not committed, as
it is Wizards of the Coast property (see below):

```sh
scripts/fetch_data                       # ~600 MB: MTGJSON and a CR snapshot
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
