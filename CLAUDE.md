# CLAUDE.md

## Version control

- This repo uses **jj** (Jujutsu), not git. ALWAYS load the /jj-guide skill IMMEDIATELY. Never refer to changes by git commit-ish. Never add `Co-Authored-By:` trailers; keep jj descriptions brief.
- **Use `jj` directly; never `git`.** Trunk protection is native repo config, not a wrapper: `immutable_heads() = builtin_immutable_heads() | (default@ ~ @)` (set by jj-workflow setup; verify with `jj config list --repo`). `@` resolves per workspace, so from any feature workspace the whole default line — trunk, every claim commit, and `default@` itself — is immutable: jj refuses, per-op, any rebase/abandon/squash that would reach shared history or another feature, while YOUR OWN feature work (commits above your claim bookmark, i.e. `name+::name@`) stays freely rewritable. In `default` the alias falls back to trunk-only, so the coordinator stays open. Never pass `--config`/`--config-file`/`--ignore-immutable` — each bypasses the alias; the jj-workflow plugin's PreToolUse hook refuses them (and bans `git`).
- To undo a mistake, prefer `jj op revert <op>` / `op undo` (surgical, bounded by immutability) over `op restore` (repo-global; it also can't un-forget a workspace). A **stale working copy** is routine (a sibling workspace advanced the shared op log): run `jj workspace update-stale` **once** — the immutability guards make this safe, it cannot reach shared history or another feature. If that single update *diverges* (a conflict, a divergent change, or `@`'s content is not what you left), immediately `jj op revert` it and STOP for help. **One self-recoverable exception:** a working-copy divergence from a concurrent op — two commits sharing `@`'s change id, one holding your work (non-empty) and the other an empty artifact (or a byte-identical copy) — is benign. Run **`workflow converge`** from the affected workspace: it keeps the half with your work (found by content — never by the `/N` index or a remembered hash, both of which shift on rebase) and drops the rest in one pass. It refuses (and you STOP) only when two halves hold genuinely different work. (The recurring `working-copy commit in workspace 'default' became immutable` warning is also benign.) For anything else genuinely unexpected — `@` not where you left it for a reason update-stale didn't fix, divergent commits with differing content or an unexpected child, anything reaching shared history — STOP and ask; never self-recover. Subagents are bound by all of this too.

## Feature workflow (`workflow`, run from `default` — except `refresh`/`repair`/`converge`/`resolve`, which run from the feature workspace)

The `workflow` (and `conflicts`) commands come from the **jj-workflow** plugin (github `msmorgan/jj-workflow`; its `bin/` is on the Bash PATH — no repo-local install). Repo specifics live in `jjworkflow.toml` (`todo_cmd = "scripts/todo"`); new workspaces are provisioned by `scripts/provision-workspace`. Each feature gets a claim commit **in default@'s linear history** (bookmarked NAME) plus an isolated jj workspace at `../NAME`.

- **Provision eagerly: the moment a task is chosen, `claim`/`start` it — before any exploration, brainstorming, design, or spec work.** Two reasons. (1) Claiming moves *your* ticket into `docs/tickets/wip/` and creates *your* workspace, so you work from a baseline you authored. `default` normally holds other live claims and in-flight edits you didn't make — that is expected coordinator state, not a problem to solve. Claim your own item first and you'll never burn effort forensically reconciling "who moved this ticket / where did these changes come from"; claim late and that pre-existing state reads as a mystery. (2) A fresh workspace has none of the gitignored fixtures the build needs (`data`, `docs/superpowers`, ~31k generated `plugins/wizards` files) until `claim`/`start` provisions them, so exploring/building/testing — or dispatching subagents — from an unprovisioned `default` wastes the work. Do the claim first, then `cd ../NAME` and proceed there.
- **Name your session after the workflow item.** Right after `claim`/`start`, or whenever you pick up work in an already-provisioned workspace `../NAME`, rename this session to `NAME` so the session/job list maps one-to-one onto the active claim and its workspace.
- Work items are per-ticket files under `docs/tickets/<status>/<slug>.md`, where the **folder is the status** (`critical`/`planned`/`maybe` = triage, `wip` = claimed, `done` = integrated). Pick the next claimable item with `scripts/todo ready` (lists items whose dependencies are all in `done/`); the keyword/action/ability-word census stays tabular in `docs/tickets/census.md`. A ticket's `needs:` frontmatter is machine-read graph data: query dependencies with `scripts/todo` (`graph <slug>` for a ticket's deps + dependents, plus `blocked`/`check`) — don't open the dependency tickets to reconstruct status by hand. **`docs/tickets/README.md`** documents the folder/status model, the priority ordering, the claim flow, and the full `scripts/todo` command set.
- `workflow claim TODO` moves TODO's ticket from its triage folder into `docs/tickets/wip/` (minting a `wip/` ticket if TODO is a census-only mechanic) and creates workspace `../TODO`; `start NAME` is the same without a ticket.
- **Taking on an extra todo mid-feature** ("oh, we're also fixing this now"): `workflow claim TODO... --into NAME` folds one or more TODOs into the **existing** workspace NAME's claim instead of spinning up a new workspace — NAME then owns those tickets too. Run it from `default` like any claim. It amends NAME's claim commit, so `../NAME` goes **stale** (rewritten parent) — that's routine; in `../NAME` run `jj workspace update-stale` (commit first) before the next commit. `integrate NAME` finishes **every** todo the claim owns and `abandon --force NAME` reverts them all.
- `refresh` has two shapes by where it's run. **From the feature workspace with no argument** (the common case — an agent's in-place "get current" call; its own private lock, so it never waits) it rebases the feature's stack onto the current trunk tip (`default@-`), **detaching it from its claim** (which stays in default's line; `integrate` re-joins it). **`refresh NAME` from `default`** instead does the old **reorder** — slides the claim to just under `default@`, feature carried along (claim+feature stay together atop default's line). Both bring the feature current with trunk. **Always `refresh` before any review step** (e.g. before `/code-review`, `requesting-code-review`, or handing work to a reviewer) so the diff is against current trunk, not stale history. `refresh --all` reorders every non-default workspace's claim under `default@` (prep for a push), stopping at the first that conflicts or fails — **human-only: never run `refresh --all` (this includes Claude and subagents). It reorders every workspace's claim + un-stales it at once. A Claude touches only its own feature, via `refresh` / `integrate NAME`.** Before any default-side line rewrite (integrate, abandon, from-default refresh) the tool first snapshots every workspace, so sibling sessions' un-snapshotted edits are banked instead of diverged.
- `integrate NAME` (from `default`) detaches-refreshes, **re-joins the claim to the now-current feature**, folds the feature into default@, moves the claim's ticket(s) `wip/→done/` (all of them, for a multi-todo `--into` claim), and **parks the workspace**: its working copy becomes a fresh empty change on the integrated tip and the directory is KEPT for follow-up work. Retire a parked (or untouched ad-hoc) workspace with plain `workflow abandon NAME` — it refuses (exit 2) if the workspace still holds un-integrated work. `abandon --force NAME` discards a feature outright: claim + stack abandoned (bounded to the feature's own commits; recoverable via the op log until gc), every owned ticket auto-reverted to its triage folder, directory deleted. Nothing is archived.
- **Conflicts/divergence land in the feature workspace, not on trunk** — refresh/integrate exit 2 ("resolve it in ../NAME, then re-run") and never roll back. Recover *inside* `../NAME` with the one-stop **`workflow repair`** = `update-stale` + `converge` (heals a working-copy divergence) + `resolve` (walks refresh/integrate conflicts oldest-first). **Exit 0** = clean (re-integrate); **exit 1** = you're now on a conflicted commit — remove its markers (the files `jj st` lists) and re-run `repair`; **exit 2** = needs a human (the two divergent halves hold genuinely different work, or a step broke and was rolled back). `converge`/`resolve` are callable directly too; `conflicts show`/`accept`/`auto` inspect and resolve marker hunks (`auto` merges alphabetized-list conflicts). When a conflict is *semantic* (e.g. another feature moved a ticket to `done/` you have elsewhere), reconcile the meaning — move/mint the right tickets — not just the markers. Recover there, not from `default`: a `jj -R` pinned to `default` inlines default@ onto the feature instead.

## CR citations

- Cite Comprehensive Rules in the `[CR#…]` bracket format — e.g. `[CR#704.5g]`, a list `[CR#601.2g,106.4]` (comma-separated, no spaces), a range `[CR#601.2a..601.2b]`. Never write a bare `CR 704.5g` or a loose `704.5g` in prose; the checker flags both.
- After adding or changing citations: `cargo xtask cite check --list-noncompliant` must be empty, and `cargo xtask cite check` must report 0 stale. When you cite a rule not yet in `cr-citations.lock`, run `cargo xtask cite bless` to register it.
- Rule numbers come from the CR, never from memory. Before committing citation changes, run `cargo xtask cite audit --diff` and read each rule's text against the claim citing it — the hash checker can NOT catch a right-number-wrong-topic cite. Give `bless`'s newly-registered list the same read.

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
scripts/provision-workspace ../NAME
```

What it does (background, in case provisioning needs fixing by hand): `data` and
`docs/superpowers` are symlinked back to the `default` checkout; `plugins/wizards`
is generated (it's all generated code — a real dir, never a symlink: the
deckmaste_cards suite loads it, and a symlink would make generate write into the
main checkout). The `data`/`docs/superpowers` ignores are **dir-only** (trailing
slash), which does NOT match a symlink — but the symlink form is already excluded
once in `default`'s `.git/info/exclude`, and every workspace shares that
(secondary workspaces have no `.git` of their own), so there is no per-workspace
exclude step. It also CoW-reflinks `default`'s `target/` into the new workspace
as a build-cache pre-warm (best-effort).

Verify with a real `jj st` (not `--ignore-working-copy`, which skips the
snapshot and hides leaked symlinks): it must report no changes.
