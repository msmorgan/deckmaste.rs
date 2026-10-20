# CLAUDE.md

## Scope

Vintage-playable Magic only: whatever is legal in Vintage. No set is out of scope as a set:
Vintage-legal cards printed in Conspiracy or Planechase products must parse like any other.

- Permanently out of scope: the Conspiracy and Plane card types, Vanguard, Scheme cards and ante.
  Do not ask about them, model them, or reserve room for them.
- Deferred, not permanently out: Un-sets (some Un-cards are Vintage-legal).

## Version control

- This repo uses **jj**, never git. The jj-sensei skills are the reference (`wisdom` first,
  `knowledge` for command semantics, `harmony` for stale/divergent/conflicted state, `boundaries`
  for the immutability aliases). Refer to changes by change id; keep descriptions brief; no
  `Co-Authored-By:` trailers.
- **Off limits:** `jj op ...`, `jj undo`, `jj redo` (the user's alone), and `--ignore-immutable`,
  `--config`, `--config-file`. Normal rewrites (`edit`, `abandon`, `squash`, `split`, `rebase`,
  `describe`) are fine: from a feature workspace, everything outside your own work is immutable, so
  an immutability refusal means you targeted the wrong rev.
- Stale working copy → `jj workspace update-stale`. Divergence → `harmony`; keep the half with
  your work, found by content, and re-run `jj fix` if a formatter raced you.
- STOP and ask only when the evidence runs out: you cannot tell what a divergent half contains,
  `@` is somewhere you cannot account for, or a refusal makes no sense.
- **Never track session scratch.** `.superpowers/` and `docs/superpowers/` stay untracked, and no
  per-task evidence, progress TSVs or plan-scoped verifier code enters `crates/`. Gates read such
  evidence from gitignored paths at runtime; the coverage lock is the only tracked gate authority.

## Feature workflow

The jj-kata `kata` and `kanban` skills own the lifecycle. Repo specifics:

- **Claim or start before any exploration, design or subagents**, then work in
  `.workspaces/NAME`. Unprovisioned workspaces lack `data`, local `docs/` dirs and the generated
  `plugins/wizards`. A hand-built workspace needs `scripts/provision-workspace .workspaces/NAME`
  run from `default`; verify with a real `jj st` (no changes).
- **Never `kata refresh --all`** (human-only).
- Tickets: `docs/tickets/<status>/<slug>.md`; `docs/tickets/README.md` is the model.
  `kata kanban ready` lists claimable items. Name your session after the item.
- English grammar tickets end with the two-line note in `docs/tickets/README.md`. Other tickets
  carry no landing record.
- A ticket-vs-ruling contradiction resolved without a STOP is a HIGH finding, even when disclosed.
- If another feature moved a ticket you also hold to `done/`, reconcile the meaning (move or mint
  tickets), not just the conflict markers.

## English grammar

- Authorities: `docs/decisions/english-lexical-analysis.md` and `docs/decisions/english-v4.md`.
  Read them before implementing, planning, reviewing or accounting for the English grammar
  (`deckmaste_english_v4`), syntax engine (`deckmaste_syntax`), or lexical catalogs
  (`deckmaste_lexical_source_v4`).
- No word enumerated by the CR as a catalog item (including all `[CR#701.1]` keyword actions) may
  appear directly in grammar declarations (`schema` or `production`); vocabulary and complement
  frames live in the domain catalogs of `deckmaste_lexical_source_v4` (and later `plugins_v3/`).
- No new work in `deckmaste_english_v3`, `deckmaste_construction_v3*`, `deckmaste_lexical_source`,
  or the legacy `deckmaste_english` stack.

## Crate fates (semantics_v3 and english_v4)

Authorities: `docs/decisions/semantics-v3.md` and `docs/decisions/english-v4.md`.
- `semantics-v3` is executed Lean-first in `lean/Semantics/` and mirrored into
  `deckmaste_semantics_v3` and `plugins_v3/` (deleting `deckmaste_semantics_v2` and
  `plugins_v2/canon/` immediately).
- `english-v4` is built alongside `v3` in `deckmaste_syntax`, `deckmaste_lexical_source_v4`, and
  `deckmaste_english_v4`. At `english-v4-cutover-v3`, `deckmaste_english_v3`,
  `deckmaste_construction_v3*`, `deckmaste_lexical_source`, and `plugins_v2/builtin/` are deleted.
- Deletion-bound at parity (`semantics-v1-cutover` and `english-v4-final-suffix-cutover`):
  `deckmaste_semantics`, `deckmaste_lowering`, `deckmaste_legacy_render`, `deckmaste_plugin`
  (`deck.rs` and `provenance.rs` rehomed), `deckmaste_english` (v1),
  `deckmaste_construction_compiler`, `deckmaste_constructions_macro`, `deckmaste_features`,
  `idris/`, `plugins/builtin`, `plugins/canon`, after which the `_v4` suffixes on
  `deckmaste_english_v4` and `deckmaste_lexical_source_v4` are dropped. Never copy v2/v3/v4 work
  back into v1, and add no new v1 registries.

## Terminology

Before naming a public item, writing an explanatory doc comment, or drafting a ticket, plan or
ADR, read `CONTEXT-MAP.md` and the owning glossary (`docs/contexts/game-model/CONTEXT.md` or
`docs/contexts/oracle-english/CONTEXT.md`). One Game Model term per concept; honour `_Avoid_`
lines. Missing or flawed terms are amended in the glossary (with CR citation) via the
`domain-modeling` skill as part of the landing. Rename identifiers; never reword entries to fit
them.

## CR citations

- Format: `[CR#704.5g]`, `[CR#601.2g,106.4]`, `[CR#601.2a..601.2b]`. Never a bare `CR 704.5g` or
  `704.5g`.
- After changes: `cargo xtask cite check --list-noncompliant` empty, `cargo xtask cite check`
  0 stale, `cargo xtask cite bless` for newly cited rules.
- Rule numbers come from the CR, never memory. Before committing, run
  `jj diff --git | cargo xtask cite audit --diff` (it reads STDIN; bare, it silently audits
  nothing) and read each rule's text against its claim, including `bless`'s new registrations.

## Assurance

- Never make a change pass by removing or weakening the check that would fail. If a test fails
  and what it asserts is still wanted, fix the code. A genuine regression is a successful find:
  fix it; stop only if the fix fails, leaves the ticket, or needs a decision.
- Never enshrine preliminary-investigation misses or model/grammar/checker defects in an ADR,
  contract, ticket, or test as "known-wrong", accepted silent errors, or permanent manual
  workarounds/escape hatches. Fix the underlying defect or ticket the fix.
- A test that asserts something wrong, or pins incidental behaviour (a reading count that
  includes wrong readings, an exact tree for an analysis nobody endorses), is retired or
  rewritten to assert what is wanted. Say which, by name, and why.
- A grammar test asserts the intended analysis. It asserts a count only when the count is the
  claim: exactly one, or exactly the set of analyses each of which is endorsed.
- Never `#[ignore]` without naming the exact blocker.
- Report tests added, rewritten and retired, with a reason for each retirement.

## Tests for a landing

- `cargo test -p <crate>` and `cargo clippy -p <crate> --tests -- -D warnings` for every crate
  whose code or consumed data changed.
- `cargo xtask lean check` only when `lean/` or plugin definitions changed.
- `cargo test --workspace` excludes `slow-tests` (TUI whole-game simulations); CI runs them.

## Bearings

- Symbol questions → rust-analyzer LSP first, grep second (subagents load it via ToolSearch
  `select:LSP`).
- Lean work → load the `lean4:*` skills and Lean LSP MCP tools (ToolSearch
  `select:mcp__lean-lsp__…`) before the first edit; subagent briefs must say so.
- Code shape: `cargo xtask map enums`; for Lean, `lean/Semantics/Words.lean` and
  `lean/Semantics/Abilities.lean`. Current declarations beat inventories in old plans.
- Plans and specs write "standard constraints apply" plus deltas; never restate this file.
- Dispatching agents: explore once, share a byte-identical brief prefix, include settled rulings,
  and send follow-ups to an agent that already holds the context.
