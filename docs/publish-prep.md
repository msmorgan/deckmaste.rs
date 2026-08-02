# Publish prep: historical completion record

2026-06-10

This was the planning checklist used to prepare the repository for publication.
It is retained as a decision record, not as an active checklist. Publication
prep subsequently chose a PolyForm Noncommercial license, added the README's
fan-content and license sections, and added CI. Current instructions and claims
live in `README.md`, `LICENSE.md`, `Cargo.toml`, and `.github/workflows/ci.yml`.

## 1. License — completed with a different decision

The initial proposal was the standard Rust MIT/Apache-2.0 dual license. The
project instead adopted PolyForm Noncommercial 1.0.0. `LICENSE.md` contains the
license and the root package declares `PolyForm-Noncommercial-1.0.0` in
`Cargo.toml`. The card-data policy remains a separate concern; see §2 and
`docs/card-data.md`.

## 2. IP note for card data — completed

The repo touches Wizards of the Coast IP in three tiers:

- `data/` — the raw MTGJSON dump (~600 MB) and CR snapshot. Already
  gitignored; never publish. The README should point at
  `scripts/fetch_data` so users fetch their own copy.
- `plugins/wizards/` — generated card stubs/todos. Already gitignored and
  regenerated via `cargo xtask generate plugins/wizards`; never publish.
- `plugins/canon/` (and friends) — hand-written RON encodings that reference
  real card names and derive from oracle text. These **are** published, so
  the README needs a fan-content section.

The plan required the README to include the Fan Content Policy notice:

> Deckmaste is unofficial Fan Content permitted under the
> [Fan Content Policy](https://company.wizards.com/en/legal/fancontentpolicy).
> Not approved/endorsed by Wizards. Portions of the materials used are
> property of Wizards of the Coast. © Wizards of the Coast LLC.

The README now states the current PolyForm Noncommercial license and the WotC
ownership/fan-content split. `docs/card-data.md` is the current data policy.

## 3. README — completed

The README pass followed this outline, with current scope rather than the
temporary card-count estimate:

1. **One-paragraph pitch** — a Magic: The Gathering rules engine in Rust,
   built around a typed card-encoding language rather than per-card scripts.
2. **What's implemented** — the engine's hard parts by name: the seven-layer
   continuous-effects system, replacement effects, last-known information,
   state-based actions, the stack, triggers, combat, turn structure. People
   who know MTG know these are where hobby engines die; name them.
3. **Honest scope line** — describe the curated vertical slice without a
   quickly stale card-count total.
4. **Architecture** — one line per crate: `deckmaste_core` (the card
   language: abilities, effects, costs, zones…), `deckmaste_engine` (game
   state and rules), `deckmaste_plugin` (corpus + suite), `deckmaste_migrations`
   (data pipeline), `macro_ron`/`macro_ron_derive` (RON macro-expansion DSL),
   `xtask` (tooling).
5. **The cite-check system** — a paragraph on `cargo xtask cite check` /
   `cite bless` and `cr-citations.lock`: code cites CR rule numbers in
   bracketed `CR#…` form and the local checker catches stale or unregistered
   citations against the rules snapshot. This is the most novel dev-tooling idea in the repo;
   don't bury it.
6. **Getting started** — build prereqs, `scripts/fetch_data` (note the
   ~600 MB download), `cargo xtask generate plugins/wizards`, then how to run
   the test suite. Flag which steps need the data and which don't.
7. **Fan-content / license section** — from §2 above.

The old selling-point counts (7 crates, ~29k lines of Rust, 434 tests) were
point-in-time planning figures and are intentionally not current claims. The
workspace now has ten library/tool crates plus the thin root binary.

## 4. CI — completed

`.github/workflows/ci.yml` now runs formatting, workspace-wide clippy with
warnings denied, and workspace tests. Data-dependent corpus tests are ignored
when their local generated inputs are absent.

The complication considered at the time was that parts of the
`deckmaste_plugin` suite loaded `plugins/wizards`, generated from the 600 MB
`data/` dump unavailable to CI. The alternatives considered were:

- **(a)** run only the crates/tests that don't need data (e.g.
  `cargo test --workspace --exclude deckmaste_plugin`, or an env-var/feature
  gate on the data-dependent tests) — simplest, recommended first pass;
- **(b)** fetch + cache the dataset in CI (actions/cache can hold it, but
  the first-fetch and restore costs are real);
- **(c)** commit a tiny fixture subset of generated wizards files used only
  in CI.

The implemented workflow uses conditional ignores for tests whose generated
inputs are absent, so a clean checkout can still run the plain workspace test
command.

Citation checking remains a local gate because CI does not install the
mtg-rules skill or its CR snapshot; the workflow documents that limitation.

## 5. Hygiene sweep — historical checklist

- Confirm `.idea/` is ignored (it exists locally).
- Decide whether `docs/tickets/` and `CLAUDE.md` stay public. Both are
  harmless — arguably good signal that the repo is actively developed with
  agent workflows — but it's a deliberate choice, not a default.
- Skim `docs/superpowers` (symlinked, shared) — make sure nothing in it is
  machine-local or private before it ships.
- Grep for anything personal: absolute home paths, email addresses, API
  keys (`rg -i 'msmorgan|home/|api[_-]?key' --glob '!target'` is a decent
  first pass).
- Check `Cargo.toml` package metadata for publishability: `description`,
  `repository`, `keywords` on the root package (even if never pushed to
  crates.io, GitHub renders nicer with them).

## 6. Publication verification — historical checklist

- Recount the numbers quoted in the README (cards encoded, tests, LOC) so
  they're true on day one.
- Verify a fresh clone builds by the README's own instructions — the README
  is the test; follow it literally on a clean checkout.
- Tag or bookmark the published state per the repo's jj conventions (ask the
  owner — CLAUDE.md forbids bookmark moves without direction).
