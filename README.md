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
payment from the keyboard. It needs no external data; the committed cards are
enough.

The engine's behavior is also covered by a test suite that exercises specific
interactions, and `crates/deckmaste_engine/examples/full_game_1k.rs` runs a
thousand complete games.

---

## Architecture

Eight crates, plus a thin root binary that launches the client:

- **`deckmaste_core`** — the card-encoding language: the typed vocabulary of
  abilities, effects, costs, zones, durations, and conditions.
- **`deckmaste_engine`** — the rules engine: game state and the rules systems
  listed above.
- **`deckmaste_cards`** — the card corpus, its plugin loader and conformance
  suite, and the card-text renderer.
- **`deckmaste_tui`** — the interactive terminal client, built on ratatui.
- **`deckmaste_migrations`** — the data pipeline (extract, resolve, graduate)
  that turns oracle text into encodings.
- **`macro_ron`** / **`macro_ron_derive`** — the RON macro-expansion layer the
  encoding language is built on.
- **`xtask`** — repository tooling: corpus generation, validation, and the
  citation checker.

---

## Scope

The implemented card set is a deliberate vertical slice rather than a complete
pool. Development has prioritized the correctness of the rules systems over
breadth of card coverage. A curated set of real cards is encoded by hand and
graduated through the data pipeline, and the encoding grammar continues to expand
toward the remaining mechanics. `docs/rules-taxonomy.md` records the plan for
that work.

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

The second is method. The engine was a way to develop a working practice for
agentic development in a domain with little tolerance for imprecision: Magic's
rules resolve to definite outcomes, so an encoding is either correct or it
produces a wrong result in play. That makes the domain a demanding test of
whether AI agents can be directed toward sustained correctness across many
interacting cases rather than a plausible but shallow approximation. Most of the
code was produced under that workflow, against an architecture and review
process I maintained. Version control is jj (Jujutsu), chosen for its
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

Everything but the full card pipeline runs with no external data:

```sh
cargo run                  # launch the interactive client (a hotseat demo)
cargo test                 # the engine's interaction suite
cargo xtask cite check     # validate CR citations against the rules snapshot
```

The full card pipeline requires the dataset, which is not committed, as it is
Wizards of the Coast property (see below):

```sh
scripts/fetch_data                       # ~600 MB: MTGJSON and a CR snapshot
cargo xtask generate plugins/wizards     # build the stub corpus from it
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
