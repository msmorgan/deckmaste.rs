# CLAUDE.md

## Version control

- This repo uses **jj** (Jujutsu), not git. ALWAYS load the /jj-guide skill IMMEDIATELY. Never refer to changes by git commit-ish. Never add `Co-Authored-By:` trailers; keep jj descriptions brief.
- **Run jj only through `scripts/jj` — never bare `jj`, `command jj`, or `git`** (a PreToolUse hook, `scripts/hooks/jj_guard.fish`, refuses all of those). `scripts/jj` pins `-R` to the current workspace and, in any workspace other than `default`, marks the whole default line immutable (`immutable_heads()=default@` — trunk + every claim commit). jj then refuses, per-op, any rebase/abandon/squash that would reach shared history or another feature — but you may freely rewrite YOUR OWN feature work (commits above your claim bookmark, i.e. `name+::name@`). `default` is the unguarded coordinator. Don't pass `--config`/`--config-file`/`--ignore-immutable`; the wrapper rejects them.
- To undo a mistake, prefer `scripts/jj op revert <op>` / `op undo` (surgical, bounded by immutability) over `op restore` (repo-global; it also can't un-forget a workspace). A **stale working copy** is routine (a sibling workspace advanced the shared op log): run `scripts/jj workspace update-stale` **once** — the immutability guards make this safe, it cannot reach shared history or another feature. If that single update *diverges* (a conflict, a divergent change, or `@`'s content is not what you left), immediately `scripts/jj op revert` it and STOP for help. **One self-recoverable exception:** a working-copy divergence from a concurrent op — two commits sharing `@`'s change id, one holding your work (non-empty) and the other an empty artifact (or a byte-identical copy) — is benign. Run **`scripts/workflow converge`** from the affected workspace: it keeps the half with your work (found by content — never by the `/N` index or a remembered hash, both of which shift on rebase) and drops the rest in one pass. It refuses (and you STOP) only when two halves hold genuinely different work. (The recurring `working-copy commit in workspace 'default' became immutable` warning is also benign.) For anything else genuinely unexpected — `@` not where you left it for a reason update-stale didn't fix, divergent commits with differing content or an unexpected child, anything reaching shared history — STOP and ask; never self-recover. Subagents are bound by all of this too.

## Feature workflow (`scripts/workflow`, run from `default`)

Each feature gets a claim commit **in default@'s linear history** (bookmarked NAME) plus an isolated jj workspace at `../NAME`.

- **Provision eagerly: the moment a task is chosen, `claim`/`start` it — before any exploration, brainstorming, design, or spec work.** Two reasons. (1) Claiming moves *your* ticket into `docs/tickets/wip/` and creates *your* workspace, so you work from a baseline you authored. `default` normally holds other live claims and in-flight edits you didn't make — that is expected coordinator state, not a problem to solve. Claim your own item first and you'll never burn effort forensically reconciling "who moved this ticket / where did these changes come from"; claim late and that pre-existing state reads as a mystery. (2) A fresh workspace has none of the gitignored fixtures the build needs (`data`, `docs/superpowers`, ~31k generated `plugins/wizards` files) until `claim`/`start` provisions them, so exploring/building/testing — or dispatching subagents — from an unprovisioned `default` wastes the work. Do the claim first, then `cd ../NAME` and proceed there.
- **Name your session after the workflow item.** Right after `claim`/`start`, or whenever you pick up work in an already-provisioned workspace `../NAME`, rename this session to `NAME` so the session/job list maps one-to-one onto the active claim and its workspace.
- Work items are per-ticket files under `docs/tickets/<status>/<slug>.md`, where the **folder is the status** (`critical`/`planned`/`maybe` = triage, `wip` = claimed, `done` = integrated). Pick the next claimable item with `scripts/todo ready` (lists items whose dependencies are all in `done/`); the keyword/action/ability-word census stays tabular in `docs/tickets/census.md`.
- `scripts/workflow claim TODO` moves TODO's ticket from its triage folder into `docs/tickets/wip/` (minting a `wip/` ticket if TODO is a census-only mechanic) and creates workspace `../TODO`; `start NAME` is the same without a ticket.
- `refresh NAME` reorders the claim to just under `default@` (feature current with trunk); `integrate NAME` refreshes, folds the feature into default@, moves the ticket `wip/→done/`, and archives the workspace to `../.integrated/`; `abandon NAME` discards the feature + claim, archiving to `../.abandoned/`. `scripts/workflow converge` (run from the feature workspace) heals a working-copy divergence — see Version control.
- **Conflicts land in the feature workspace, not on trunk** — refresh/integrate exit 2 ("resolve it in ../NAME, then re-run") and never roll back. Resolve *inside* `../NAME` with **`scripts/workflow resolve`**, which walks the stack's conflicts oldest-first: each run drops you onto one conflicted commit (exit 1); remove its markers (the files `jj st` lists); re-run to fold your fix in and advance — until exit 0 (clean), then re-integrate. Exit 2 means you left markers behind, or jj broke and was rolled back. (For a working-copy *divergence* rather than a refresh conflict, use **`scripts/workflow converge`** — see Version control.) When the conflict is semantic (e.g. another feature ticked a ticket to `done/` you have elsewhere), reconcile the *meaning* — move/mint the right tickets — not just the markers. Resolve there, not from `default`: a `jj` pinned to `default` (e.g. a sourced workflow alias) inlines default@ onto the feature instead.

## CR citations

- Cite Comprehensive Rules in the `[CR#…]` bracket format — e.g. `[CR#704.5g]`, a list `[CR#601.2g,106.4]` (comma-separated, no spaces), a range `[CR#601.2a..601.2b]`. Never write a bare `CR 704.5g` or a loose `704.5g` in prose; the checker flags both.
- After adding or changing citations: `cargo xtask cite check --list-noncompliant` must be empty, and `cargo xtask cite check` must report 0 stale. When you cite a rule not yet in `cr-citations.lock`, run `cargo xtask cite bless` to register it.
- Rule numbers come from the CR, never from memory. Before committing citation changes, run `cargo xtask cite audit --diff` and read each rule's text against the claim citing it — the hash checker can NOT catch a right-number-wrong-topic cite. Give `bless`'s newly-registered list the same read.

## New jj workspaces

`scripts/workflow start`/`claim` provisions a new workspace's gitignored shared dirs
for you (its `__provision_ws` step). You only need the manual steps below for a
workspace you hand-built with `jj workspace add` instead of going through the workflow.

`data` and `docs/superpowers` are symlinked back to the `default` checkout;
`plugins/wizards` is generated (it's all generated code). The `data`/`docs/superpowers`
ignores are **dir-only** (trailing slash), which does NOT match a symlink — but the
symlink form is already excluded once in `default`'s `.git/info/exclude`, and every
workspace shares that (secondary workspaces have no `.git` of their own), so there is
no per-workspace exclude step.

```sh
# from the new workspace's root — symlinks back to the default checkout. Mind the
# `..` depth: data is at the root, docs/superpowers sits in docs/.
ln -s ../default/data ./data
ln -s ../../default/docs/superpowers ./docs/superpowers

# wizards: generate a real dir (plugins/*/ already ignores it). Do NOT symlink it —
# the deckmaste_cards suite loads it, and a symlink would make generate write into the
# main checkout.
cargo xtask generate plugins/wizards
```

Verify with a real `./scripts/jj st` (not `--ignore-working-copy`, which skips the
snapshot and hides leaked symlinks): it must report no changes.
