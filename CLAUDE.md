# CLAUDE.md

## Version control

- This repo uses **jj** (Jujutsu), not git. For any jj behaviour not covered here or by the session's injected jj guidance, load **`/jj-sensei:knowledge`** (authoritative, version-matched command help) or **`/jj-sensei:wisdom`** (history-shaping idioms: splitting, revset selection, placement) rather than inferring from git. Never refer to changes by git commit-ish. Never add `Co-Authored-By:` trailers; keep jj descriptions brief.
- **Immutability mechanism** (the jj-not-git rule and the `--config`/`--config-file`/`--ignore-immutable` bans live in the jj-kata skill and are PreToolUse-hook-enforced): trunk protection is a set of four repo-config revset aliases installed by **jj-sensei**, whose net effect is `immutable_heads() = builtin_immutable_heads() | only_if(not_default(), other_workspaces())`. `@` resolves per workspace, so from any feature workspace every OTHER working copy and all its ancestors — trunk, the whole default line, and every sibling feature — is immutable (jj refuses per-op any rewrite reaching shared history or another feature), while YOUR OWN feature work (commits above your claim bookmark, `name+::name@`) stays freely rewritable. In `default` the custom term collapses to `none()`, so the coordinator stays open. jj-kata verifies these aliases by exact text and refuses every lifecycle command if they are missing or reworded — never hand-edit them; re-run the **`/jj-sensei:boundaries`** skill instead, and audit with its `--check`.
- **The line is the op log, not history rewriting.** Off limits, always, agents and subagents alike: `jj op ...` (`op revert`/`op restore`/`op abandon`/`op integrate`), `jj undo`, `jj redo`. Those rewind shared state under every workspace and are the user's alone. Everything else is yours to use normally — `jj edit`, `jj abandon`, `jj squash`, `jj split`, `jj rebase`, `jj describe` — because the `immutable_heads()` alias is what enforces safety: from a feature workspace the whole default line is immutable, so a rewrite that would reach trunk, a claim commit, or another feature is refused per-op. You cannot damage shared history with these; you can only rewrite your own feature work. If jj says a commit is immutable, you targeted the wrong rev — fix the rev, never `--ignore-immutable`.
- A **stale working copy** is routine (a sibling workspace advanced the shared op log): run `jj workspace update-stale`. A **working-copy divergence** — two commits sharing `@`'s change id — is likewise ordinary; jj-kata has no repair command, so reach for the **`/jj-sensei:harmony`** skill, which resolves stale state, divergent successors, and file conflicts oldest-first. Keep the half with your work, found by content — never by the `/N` index or a remembered hash, both of which shift on rebase. When the halves hold genuinely different work, **resolve it by hand**: diff the two (`jj diff --from A --to B`), decide which half to keep on the evidence, then `jj edit <keep>` and `jj abandon <drop>`. A formatter pass racing an edit is the common cause, and the halves usually differ only in formatting plus one real change — keep the half with the real change and re-run `jj fix`. (The recurring `working-copy commit in workspace 'default' became immutable` warning is benign.)
- STOP and ask only when the evidence runs out: you cannot tell what a divergent half contains, `@` is somewhere you cannot account for, or a rewrite is refused as immutable and you do not understand why. "I might damage something" is not a reason to stop — the guard already prevents that.
- **Superpowers artifacts never enter version control.** `.superpowers/` (SDD plans, briefs, progress and task reports) and `docs/superpowers/` are ignored and must stay untracked: never `jj file track` them past the ignore, and never hand-add them as part of a wider snapshot. They are session scratch, not repo history. Undoing a slip here is not a tip-side deletion commit — the blobs stay in every commit that touched them, so it costs a split-and-abandon pass over that whole stretch of history. The same rule covers source-embedded process artifacts: per-task evidence snapshots, progress TSVs, and plan-scoped verifier code must never enter `crates/` (e.g. as `include_str!` data) — the tracked gate authority is the coverage lock alone; gates read any other evidence from gitignored paths at runtime (ruling 2026-08-26, after Plan 09 accreted 7.1MB of per-task TSVs into xtask).

## Feature workflow

The lifecycle (`claim`/`start` → work in `.workspaces/NAME` → `integrate`, then `drop`) is the **jj-kata** plugin — the auto-loading **`/jj-kata:kata` skill** is the reference for every command, flag, and exit code, and **`/jj-kata:kanban`** covers ticket inspection. Don't restate them here; this section is only what the plugin can't know about *this* repo. Two guardrails the skills omit: **never `refresh --all`** — human-only; it rewrites every workspace's claim at once and can race a concurrent `integrate` into divergence. And jj-kata requires jj-sensei's boundary aliases (above); if a lifecycle command refuses with "kata needs its teacher", run the `/jj-sensei:boundaries` skill rather than editing repo config by hand.

- **Claim/start eagerly — before any exploration, design, or subagents.** A fresh workspace lacks the gitignored fixtures the build needs — `data`, ignored local `docs/` directories such as `docs/superpowers` and `docs/memory`, and ~31k generated `plugins/wizards` — until `scripts/provision-workspace` runs on `claim`/`start`, so building/testing from an unprovisioned workspace wastes the work. And `default` holds other sessions' live claims and edits (expected coordinator state) — claim your own item first so that pre-existing state isn't a mystery to reconcile. Then `cd .workspaces/NAME` — this repo sets `workspace_dir = ".workspaces"` in `kata.toml` so feature workspaces stay inside the coordinator checkout for Codex sandbox compatibility. EnterWorktree is wired to this repo (jj-kata's `WorktreeCreate`/`WorktreeRemove` hooks, registered in the untracked `.claude/settings.local.json` — the hook path is machine-specific, so it must never go in the tracked `.claude/settings.json`; re-add it per checkout from the plugin's `hooks/claude-project-hooks.example.json`), so isolating that way mints a real feature workspace and provisions identically — the path background-job sessions use.
- **Tickets** live at `docs/tickets/<status>/<slug>.md`, folder = status; `kata kanban ready` lists the next claimable items. **`docs/tickets/README.md`** is the full model (folders, priority order, the `kanban` command set). `kata.toml` sets `[items] driver = "kanban"`, so jj-kata's bundled folder driver owns the ticket moves — this repo ships no ticket tooling of its own. `docs/tickets/census.md` is a prioritisation reference table only; it is not a graph source and nothing reads it mechanically.
- **Name your session after the item** (NAME) so the session/job list maps one-to-one onto the active claim.
- **Every completed ticket carries a `## Landing record`** written by the claimant before integrate, in three tiers (rewrite ADR, "Amendment: what a landing proves, discloses, and reports (2026-09-04)"). **PROVE** — (a) *no silent loss*: every identity that stopped being covered is named, classified (wrong analysis retired / re-coverage owed to `<live ticket>` / regression) and routed; an UNEXPLAINED loss is the defect, a decrease with a complete list is normal, and `DECKMASTE_COVERAGE_LOCK=report` is the normal mode (the add-only ratchet and the retirement-manifest/obligation-line ceremony are retired, not suspended); (b) the *structural laws*: byte-exact roundtrip, lexical ownership, construction and leaf traversal identity, zero unresolved ties, zero internal failures; (c) *no word-naming*: forbidden licensing checkers at zero plus the `environment.rs` load errors. A guard that names a lexeme, construction, verb, noun, preposition, or card identity (`head != CoreVerbIdentity::X`, `require x is <Construction>`, a comment naming a card) is a STOP-and-report, never shipped — the permitted guard reads a declared feature. **DISCLOSE** — every identity newly covered listed with its selected analysis (a negative oracle or a wrong analysis that starts parsing is a defect and a STOP, never a coverage gain — the exact shape the 2026-09-03 require-through landing shipped); the selection census before/after (unique / specificity-resolved) with the construction pair named if the specificity share rose; the permitted licensing-checker total emitted by `coverage`; a **Deviations and additions** list (every construction or test added or deleted beyond the ticket's letter, with justification); every STOP with its resolution; and any glossary gap — a term the landing needed that `docs/contexts/oracle-english/CONTEXT.md` does not define. **REPORT** (provenance, never fitted to, never a gate) — the lock `covered` count and construction count, the homograph and form-literal/vocabulary overlap inventories as named lists, and a **performance advisory**: coverage-command wall time against the 16.26s quiet-host ceiling plus the per-byte thread-CPU telemetry line, written as an integer in nanoseconds per byte, e.g. `103,762 ns/B`, never `103.762 µs/B`, which the cite checker reads as a rule number, each stated with host load and worker count. All numbers are **stamped with the change id and lock `covered` count of the tree they were measured on** (an integrate rebase can move every corpus figure while both lock hashes still pass). A landing without a record is a review finding; a ticket-vs-ruling contradiction resolved without a STOP is a HIGH finding — **even when the resolution is disclosed and well-measured**: stop on a contradiction with a recorded ruling even if you can resolve it, because disclosure is not authority. Ledger residue routes to live tickets at integrate.
- **Semantic conflicts:** when the conflict is that another feature moved a ticket to `done/` you also hold, reconcile the *meaning* — move/mint the right tickets — not just the markers.

## Crate fates (english_v2 rewrite)

Authority: `docs/decisions/english-v2-rewrite.md` (cutover plan). Until cutover:

- **A `_v2` sibling is a replacement marker, not a permanent variant.** If
  `thing_v2` exists beside `thing`, `thing` is on the chopping block and is
  retained only to hold the roof up while the actively developed `thing_v2`
  reaches cutover. Put new architecture and features in `thing_v2`; touch
  `thing` only to keep current users functioning or to enable cutover. Never
  treat the pair as parallel long-term implementations or copy the v2 design
  back into the deletion-bound one for parity.

- **Deleted at cutover:** `deckmaste_english` (v2 takes its name),
  `deckmaste_construction_compiler`, `deckmaste_constructions_macro`, and
  `deckmaste_spelling`'s splice-and-reparse render/compile machinery (that
  crate itself survives). `deckmaste_legacy_render` is legacy independently
  of the rewrite.
  `deckmaste_features` (v1's feature vocabulary: strata, witness metadata,
  `chart_feature!`, first-character onset) also deletes at cutover — its last
  non-legacy use is two frame re-exports in `macro_ron`, which the frame seam
  replaces. v2 never depends on it: `Onset` is a sealed compiler feature like
  `Number`, normalized rows carry effective onset as data, and the pronunciation
  recipe is v2-owned code.
- **The rewrite's crates:** `deckmaste_english_v2` (takes the
  `deckmaste_english` name at cutover), the future `deckmaste_construction`
  declaration compiler, and the stable shared layer `deckmaste_data`
  (snapshot models) + `deckmaste_catalogs` (catalog extraction, inventory,
  line-file I/O — its `legacy` adapter module deletes together with
  `deckmaste_english` at cutover). Plan 07 adds no dependency on
  deletion-bound `deckmaste_features`; v2's dependency set is unchanged. v2
  must never add a direct dependency on a crate slated for deletion. It does
  not depend directly on `deckmaste_catalogs`: xtask alone adapts catalog
  contents into the frozen typed provider rows v2 consumes. Since the macro-ron
  fold-back (2026-09-02) `deckmaste_english_v2` no longer depends on
  `macro_ron` at all (it depends directly on `deckmaste_construction_core`,
  which owns the spelling/grammar metadata type); `macro_ron`'s own
  `deckmaste_features` edge lives only in its legacy `frames.rs` re-exports
  and deletes with v1.
- **`deckmaste_migrations` survives as a function** (card
  extract→resolve→graduate, snapshot ingestion). Its oracle-text extraction
  is its own regex pipeline — it does not consume the construction parser —
  but it depends on deletion-slated `deckmaste_legacy_render` and must shed
  that by cutover. It consumes `deckmaste_data` (temporarily) for its
  surviving extraction work; its former catalog module is gone.
  Re-pointing extraction at english_v2 is a separate, not-yet-scheduled
  decision. Do not home new english_v2 infrastructure there — or in any
  crate marked for deletion — without recording the deviation in the
  rewrite ADR.
- Everything else is unaffected by the rewrite.

## Terminology

- Before terminology-sensitive work — naming a public type, variant, field or
  function, writing an explanatory doc comment, or drafting a ticket, plan or
  ADR — read the root **`CONTEXT-MAP.md`** and follow it to the owning
  glossary: `docs/contexts/game-model/CONTEXT.md` for Magic and engine-semantic
  concepts, `docs/contexts/oracle-english/CONTEXT.md` for grammar and
  realization. Core, the engine and the Idris workbench use ONE Game Model term
  per concept; Oracle English keeps its linguistic terms even where a spelling
  (Object, Predicate) means something else in the Game Model. Honour each
  entry's `_Avoid_` line.
- Keep those documents current through the **`domain-modeling`** skill: when a
  landing needs a term the owning glossary does not define, or implementation
  evidence exposes a flaw in a definition, amend that glossary with its CR
  citation as part of the landing. Never reword an entry to accommodate an
  existing identifier — rename the identifier.

## CR citations

- Cite Comprehensive Rules in the `[CR#…]` bracket format — e.g. `[CR#704.5g]`, a list `[CR#601.2g,106.4]` (comma-separated, no spaces), a range `[CR#601.2a..601.2b]`. Never write a bare `CR 704.5g` or a loose `704.5g` in prose; the checker flags both.
- After adding or changing citations: `cargo xtask cite check --list-noncompliant` must be empty, and `cargo xtask cite check` must report 0 stale. When you cite a rule not yet in `cr-citations.lock`, run `cargo xtask cite bless` to register it.
- Rule numbers come from the CR, never from memory. Before committing citation changes, run `jj diff --git | cargo xtask cite audit --diff` and read each rule's text against the claim citing it — the hash checker can NOT catch a right-number-wrong-topic cite. The command reads its diff from STDIN: run bare (no pipe), it silently audits 0 citation sites and still exits 0, so always pipe a diff in. Give `bless`'s newly-registered list the same read.

## Assurance (a standard constraint)

- **A round never reaches green by removing what could fail.** A test whose
  subject still exists must keep passing; if it fails, the fix is in the code.
  A test whose subject a ticket deliberately retires gets **re-spelled**
  against the replacement shape — same card, same asserted outcome, new
  spelling — never deleted. Never swap a value comparison for a
  `discriminant`/`matches!` check, and never add `#[ignore]` without naming
  the exact blocker in the attribute or an adjacent comment.
- **Report the counts** in the landing record: restored, re-spelled, ignored
  with blockers, added, and removed. A nonzero removed count justifies each
  one by name.
- **Finding a genuine regression is a successful outcome.** Stop and report
  it; do not integrate around it and do not delete the test that found it.
- Why this is a rule: the 2026-09-02 discourse-regions landing removed 164
  tests and added 1, reported every suite green, and shipped six regressions
  the deleted tests had covered — one of which silently broke every "that
  many" replacement, so infect, wither, and doubled mill were all dead on
  trunk until the restoration round.

## Gate scope for compiler changes

- Any diff touching `crates/deckmaste_construction_core/src/emit/` (or the
  emitter↔environment contract the generated code relies on), any declaration
  data under `plugins/builtin_v2/` (keyword-action stubs, catalog rows), or
  `crates/deckmaste_english_v2/src/core_verbs.ron` gates on
  `cargo test --workspace` — never an enumerated `-p` list. Declaration data
  is consumed by the plugin/builtin test crates too: the 2026-09-04
  with-preposition landing converted a stub's `with` literal to the declared
  `Preposition::With` member under a `-p english_v2 -p xtask` gate and left
  `exchange_has_every_attested_representable_tail_shape` red on trunk. The generated
  code is exercised only by downstream consumer crates
  (`deckmaste_construction`'s compiled-consumer fixture among them), so an
  enumerated list cannot reach the break; three landings shipped a red
  workspace suite that way before this rule (2026-09-03).

## Model economy

- **If you are Fable: Fable is expensive.** A sequence of mechanical edits or
  simple tool calls run inline is uneconomical — every turn re-reads the whole
  context, so the cache reads dominate the cost of the work itself. Delegate
  such runs to a Sonnet or Opus subagent as appropriate (Sonnet for mechanical,
  Opus for anything needing judgment), and spend Fable's turns on triage,
  briefing, and verification.

## Bearings (token efficiency)

- Symbol questions (where defined, who calls it, what variants, what signature) →
  rust-analyzer LSP first, grep second. The LSP tool is deferred — subagents must load
  it explicitly (ToolSearch `select:LSP`) before use.
- Current code shape: `cargo xtask map enums` (core taxonomy variant dump) and
  `cargo xtask map idris` (Idris constructor map) regenerate on demand — prefer these
  over re-reading source or trusting prose in old plans/specs, which goes stale.
- Plans/specs: never restate standard constraints (jj, fmt, clippy, CR citations,
  wizards regen, assurance — they live here); write "standard constraints apply" plus deltas
  only. Context sections cite prior docs and describe deltas; re-derived subsystem
  prose is a review flag.
- Dispatching agents: explore once, pass the brief — paste it as a byte-identical
  prompt prefix across the fan-out (prompt-cache-shared, question at the tail), or
  send follow-ups to an agent that already holds the context instead of spawning
  fresh; include the relevant settled rulings in design/audit agent prompts rather
  than letting them re-derive (or contradict) them.

## New jj workspaces

`kata start`/`claim` provisions a new workspace's gitignored shared dirs for
you by running `scripts/provision-workspace` (the `kata.toml` provision
hook). For a workspace you hand-built with `jj workspace add` instead, run the
hook yourself from `default`:

```sh
scripts/provision-workspace .workspaces/NAME
```

What it does (background, in case provisioning needs fixing by hand): `data` is
symlinked back to the `default` checkout. Each directory present under
`default/docs/` but absent from the new tracked workspace is also symlinked;
this shares ignored local directories such as `docs/superpowers` and
`docs/memory` while leaving tracked documentation directories alone.
`plugins/wizards` is generated (it's all generated code — a real dir, never a
symlink: the deckmaste_plugin suite loads it, and a symlink would make generate
write into the main checkout). The workspaces share the repository and global
ignore configuration, so there is no per-workspace exclude step. Provisioning
also CoW-reflinks `default`'s `target/` into the new workspace as a build-cache
pre-warm (best-effort).

Verify with a real `jj st` (not `--ignore-working-copy`, which skips the
snapshot and hides leaked symlinks): it must report no changes.
