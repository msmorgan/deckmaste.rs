# CLAUDE.md

## Scope

Vintage-playable Magic only: whatever is legal in Vintage. Un-sets (including Unfinity's
eternal-legal cards), Conspiracy, Planechase, Archenemy, Vanguard and ante are permanently out of
scope. Do not ask about them, model them, or reserve room for them.

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
- Every completed ticket carries a **Landing record** before integrate; its six items are
  specified in `docs/tickets/README.md`. A landing without one is a review finding.
- A ticket-vs-ruling contradiction resolved without a STOP is a HIGH finding, even when disclosed.
- If another feature moved a ticket you also hold to `done/`, reconcile the meaning (move or mint
  tickets), not just the conflict markers.

## English grammar

- `deckmaste_english_v3` is the active grammar; read
  `docs/decisions/english-lexical-analysis.md` before implementing, planning, reviewing or
  accounting for it. Preserve every grammatical Reading and both roundtrip laws.
- Vocabulary and verbs live in `deckmaste_lexical_source/lexicon`. No new work in the legacy
  `deckmaste_english` stack.
- No silent loss (the per-face census digest diff enforces it). No guard that names a lexeme,
  construction, verb, noun, preposition or card: that is a STOP. A permitted guard reads a
  declared feature.
- Corpus work iterates on a subset (`cargo xtask english-v3 <selector>`) and verifies once with
  `--all`. Attestation checks use `cargo xtask english-v3 grep`, never raw `rg` over Scryfall.

## Crate fates (semantics_v2)

Authority: `docs/decisions/semantics-v2.md` §§10–16. `deckmaste_semantics_v2` mirrors the Lean
syntax over `plugins_v2/` and depends on `macro_ron` alone. Deletion-bound at parity:
`deckmaste_semantics`, `deckmaste_lowering`, `deckmaste_legacy_render`, `deckmaste_plugin`
(`deck.rs` and `provenance.rs` rehomed), `idris/`, `plugins/builtin`, `plugins/canon`. Never copy
v2 work back into v1, and add no new v1 registries.

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

- Never reach green by removing what could fail. A failing test whose subject still exists is
  fixed in the code. A test whose subject is deliberately retired is **re-spelled** against the
  replacement (same card, same asserted outcome), never deleted.
- Never swap a value comparison for a `discriminant`/`matches!` check. Never `#[ignore]` without
  naming the exact blocker.
- A genuine regression is a successful find: fix it; stop only if the fix fails, leaves the
  ticket, or needs a ruling. Never integrate around it or weaken the test.
- Report restored / re-spelled / ignored / added / removed counts; justify each removal by name.
  (Origin: a 2026-09-02 landing deleted 164 tests and shipped six regressions.)

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
