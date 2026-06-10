# Publish prep: getting this repo ready for GitHub

2026-06-10

Goal: publish this repo publicly after a README pass. This doc is the
checklist and the reasoning; work through it top to bottom. Items are ordered
hard-blockers first. Repo rules in CLAUDE.md (jj, citations, workspaces)
apply to all of this work.

## 1. LICENSE (hard blocker)

Without a license file the code is all-rights-reserved and nobody can legally
use it. Add the standard Rust dual license:

- `LICENSE-MIT` and `LICENSE-APACHE` at the repo root.
- `license = "MIT OR Apache-2.0"` in the workspace `Cargo.toml` (use
  `[workspace.package]` and have crates inherit with `license.workspace =
  true`).

The dual license covers the **code only**. The card encodings are a separate
question — see §2.

## 2. IP note for card data (hard blocker)

The repo touches Wizards of the Coast IP in three tiers:

- `data/` — the raw MTGJSON dump (~600 MB) and CR snapshot. Already
  gitignored; never publish. The README should point at
  `scripts/fetch_data.fish` so users fetch their own copy.
- `plugins/wizards/` — generated card stubs/todos. Already gitignored and
  regenerated via `cargo xtask generate plugins/wizards`; never publish.
- `plugins/canon/` (and friends) — hand-written RON encodings that reference
  real card names and derive from oracle text. These **are** published, so
  the README needs a fan-content section.

Add to the README (verbatim boilerplate WotC asks for):

> Deckmaste is unofficial Fan Content permitted under the
> [Fan Content Policy](https://company.wizards.com/en/legal/fancontentpolicy).
> Not approved/endorsed by Wizards. Portions of the materials used are
> property of Wizards of the Coast. © Wizards of the Coast LLC.

Also state the split explicitly: code is MIT/Apache-2.0; card names, oracle
text, and the Comprehensive Rules remain WotC property; this project is
non-commercial.

## 3. README

Lead with what works **today**; honest scoping reads better than letting
people discover it. Suggested skeleton:

1. **One-paragraph pitch** — a Magic: The Gathering rules engine in Rust,
   built around a typed card-encoding language rather than per-card scripts.
2. **What's implemented** — the engine's hard parts by name: the seven-layer
   continuous-effects system, replacement effects, last-known information,
   state-based actions, the stack, triggers, combat, turn structure. People
   who know MTG know these are where hobby engines die; name them.
3. **Honest scope line** — ~2,057 of ~31k cards encoded so far (recount at
   publish time: `find plugins -name '*.ron' | wc -l`). In-progress, and the
   taxonomy work (`docs/rules-taxonomy.md`) is the plan for the rest.
4. **Architecture** — one line per crate: `deckmaste_core` (the card
   language: abilities, effects, costs, zones…), `deckmaste_engine` (game
   state and rules), `deckmaste_cards` (corpus + suite), `deckmaste_migrations`
   (data pipeline), `macro_ron`/`macro_ron_derive` (RON macro-expansion DSL),
   `xtask` (tooling).
5. **The cite-check system** — a paragraph on `cargo xtask cite check` /
   `cite bless` and `cr-citations.lock`: code cites CR rule numbers in
   `[CR#…]` form and CI catches stale or unregistered citations against the
   rules snapshot. This is the most novel dev-tooling idea in the repo;
   don't bury it.
6. **Getting started** — build prereqs, `scripts/fetch_data.fish` (note the
   ~600 MB download), `cargo xtask generate plugins/wizards`, then how to run
   the test suite. Flag which steps need the data and which don't.
7. **Fan-content / license section** — from §2 above.

Selling-point numbers as of this writing, for flavor if wanted: 7 workspace
crates, ~29k lines of Rust, 434 tests, workspace-wide `clippy::pedantic`.

## 4. CI

There is no `.github/` yet. Add one workflow: `cargo fmt --check`, clippy
(deny warnings — pedantic is already on workspace-wide), `cargo test`.

The complication: parts of the `deckmaste_cards` suite load
`plugins/wizards`, which is generated from the 600 MB `data/` dump that CI
won't have. Investigate which tests actually need it, then pick one:

- **(a)** run only the crates/tests that don't need data (e.g.
  `cargo test --workspace --exclude deckmaste_cards`, or an env-var/feature
  gate on the data-dependent tests) — simplest, recommended first pass;
- **(b)** fetch + cache the dataset in CI (actions/cache can hold it, but
  the first-fetch and restore costs are real);
- **(c)** commit a tiny fixture subset of generated wizards files used only
  in CI.

Whichever is chosen, `cargo xtask cite check` should also run in CI — it's
fast and it's the repo's signature check.

## 5. Hygiene sweep

- Confirm `.idea/` is ignored (it exists locally).
- Decide whether `docs/todo.md` and `CLAUDE.md` stay public. Both are
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

## 6. At publish time

- Recount the numbers quoted in the README (cards encoded, tests, LOC) so
  they're true on day one.
- Verify a fresh clone builds by the README's own instructions — the README
  is the test; follow it literally on a clean checkout.
- Tag or bookmark the published state per the repo's jj conventions (ask the
  owner — CLAUDE.md forbids bookmark moves without direction).
