# CLAUDE.md

## Version control

- This repo uses **jj** (Jujutsu), not git. ALWAYS load the /jj-guide skill IMMEDIATELY. Never refer to changes by git commit-ish. Never add `Co-Authored-By:` trailers; keep jj descriptions brief.
- **Immutability mechanism** (the jj-not-git rule and the `--config`/`--config-file`/`--ignore-immutable` bans live in the jj-workflow skill and are PreToolUse-hook-enforced): trunk protection is a repo-config alias, `immutable_heads() = builtin_immutable_heads() | (default@ ~ @)`. `@` resolves per workspace, so from any feature workspace the whole default line — trunk, every claim commit, and `default@` itself — is immutable (jj refuses per-op any rewrite reaching shared history or another feature), while YOUR OWN feature work (commits above your claim bookmark, `name+::name@`) stays freely rewritable. In `default` the alias falls back to trunk-only, so the coordinator stays open.
- **The line is the op log, not history rewriting.** Off limits, always, agents and subagents alike: `jj op ...` (`op revert`/`op restore`/`op abandon`/`op integrate`), `jj undo`, `jj redo`. Those rewind shared state under every workspace and are the user's alone. Everything else is yours to use normally — `jj edit`, `jj abandon`, `jj squash`, `jj split`, `jj rebase`, `jj describe` — because the `immutable_heads()` alias is what enforces safety: from a feature workspace the whole default line is immutable, so a rewrite that would reach trunk, a claim commit, or another feature is refused per-op. You cannot damage shared history with these; you can only rewrite your own feature work. If jj says a commit is immutable, you targeted the wrong rev — fix the rev, never `--ignore-immutable`.
- A **stale working copy** is routine (a sibling workspace advanced the shared op log): run `jj workspace update-stale`. A **working-copy divergence** — two commits sharing `@`'s change id — is likewise ordinary: `workflow converge` resolves it in one pass when one half is an empty artifact or a byte-identical copy, keeping the half with your work (found by content — never by the `/N` index or a remembered hash, both of which shift on rebase). When converge refuses because the halves hold genuinely different work, **resolve it by hand**: diff the two (`jj diff --from A --to B`), decide which half to keep on the evidence, then `jj edit <keep>` and `jj abandon <drop>`. A formatter pass racing an edit is the common cause, and the halves usually differ only in formatting plus one real change — keep the half with the real change and re-run `jj fix`. (The recurring `working-copy commit in workspace 'default' became immutable` warning is benign.)
- STOP and ask only when the evidence runs out: you cannot tell what a divergent half contains, `@` is somewhere you cannot account for, or a rewrite is refused as immutable and you do not understand why. "I might damage something" is not a reason to stop — the guard already prevents that.

## Feature workflow

The lifecycle (`claim`/`start` → work in `.workspaces/NAME` → `integrate`, then `drop`; plus `refresh`/`repair`/`conflicts`) is the **jj-workflow** plugin — the auto-loading **`/jj-workflow` skill** is the reference for every command, flag, and exit code. Don't restate it here; this section is only what the plugin can't know about *this* repo. (One guardrail the auto-loaded skill omits: **never `refresh --all`** — human-only; it rewrites every workspace's claim at once and can race a concurrent `integrate` into divergence.)

- **Claim/start eagerly — before any exploration, design, or subagents.** A fresh workspace lacks the gitignored fixtures the build needs — `data`, ignored local `docs/` directories such as `docs/superpowers` and `docs/memory`, and ~31k generated `plugins/wizards` — until `scripts/provision-workspace` runs on `claim`/`start`, so building/testing from an unprovisioned workspace wastes the work. And `default` holds other sessions' live claims and edits (expected coordinator state) — claim your own item first so that pre-existing state isn't a mystery to reconcile. Then `cd .workspaces/NAME` — this repo sets `workspace_dir = ".workspaces"` in `jjworkflow.toml` (feature workspaces stay inside the coordinator checkout for Codex sandbox compatibility), so they are NOT at the plugin's default `../NAME`. EnterWorktree is wired to this repo (the plugin's `WorktreeCreate`/`WorktreeRemove` hooks, registered in the untracked `.claude/settings.local.json` — the plugin path is machine-specific, so it must never go in the tracked `.claude/settings.json`; re-add it per checkout), so isolating that way mints a real feature workspace and provisions identically — the path background-job sessions use.
- **Tickets** live at `docs/tickets/<status>/<slug>.md`, folder = status; `scripts/todo ready` lists the next claimable items and the census is `docs/tickets/census.md`. **`docs/tickets/README.md`** is the full model (folders, priority order, the `scripts/todo` command set); `jjworkflow.toml` points `todo_cmd` at `scripts/todo`.
- **Name your session after the item** (NAME) so the session/job list maps one-to-one onto the active claim.
- **Semantic conflicts:** when the conflict is that another feature moved a ticket to `done/` you also hold, reconcile the *meaning* — move/mint the right tickets — not just the markers.

## CR citations

- Cite Comprehensive Rules in the `[CR#…]` bracket format — e.g. `[CR#704.5g]`, a list `[CR#601.2g,106.4]` (comma-separated, no spaces), a range `[CR#601.2a..601.2b]`. Never write a bare `CR 704.5g` or a loose `704.5g` in prose; the checker flags both.
- After adding or changing citations: `cargo xtask cite check --list-noncompliant` must be empty, and `cargo xtask cite check` must report 0 stale. When you cite a rule not yet in `cr-citations.lock`, run `cargo xtask cite bless` to register it.
- Rule numbers come from the CR, never from memory. Before committing citation changes, run `jj diff --git | cargo xtask cite audit --diff` and read each rule's text against the claim citing it — the hash checker can NOT catch a right-number-wrong-topic cite. The command reads its diff from STDIN: run bare (no pipe), it silently audits 0 citation sites and still exits 0, so always pipe a diff in. Give `bless`'s newly-registered list the same read.

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
  wizards regen — they live here); write "standard constraints apply" plus deltas
  only. Context sections cite prior docs and describe deltas; re-derived subsystem
  prose is a review flag.
- Dispatching agents: explore once, pass the brief — paste it as a byte-identical
  prompt prefix across the fan-out (prompt-cache-shared, question at the tail), or
  send follow-ups to an agent that already holds the context instead of spawning
  fresh; include the relevant settled rulings in design/audit agent prompts rather
  than letting them re-derive (or contradict) them.

## New jj workspaces

`workflow start`/`claim` provisions a new workspace's gitignored shared dirs for
you by running `scripts/provision-workspace` (the `jjworkflow.toml` provision
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
