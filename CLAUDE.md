# CLAUDE.md

## Version control

- This repo uses **jj** (Jujutsu), not git. ALWAYS load the /jj-guide skill IMMEDIATELY. Never refer to changes by git commit-ish. Never add `Co-Authored-By:` trailers; keep jj descriptions brief.
- **Run jj only through `scripts/jj` — never bare `jj`, `command jj`, or `git`** (a PreToolUse hook, `scripts/hooks/jj_guard.fish`, refuses all of those). `scripts/jj` pins `-R` to the current workspace and, in any workspace other than `default`, marks the whole default line immutable (`immutable_heads()=default@` — trunk + every claim commit). jj then refuses, per-op, any rebase/abandon/squash that would reach shared history or another feature — but you may freely rewrite YOUR OWN feature work (commits above your claim bookmark, i.e. `name+::name@`). `default` is the unguarded coordinator. Don't pass `--config`/`--config-file`/`--ignore-immutable`; the wrapper rejects them.
- To undo a mistake, prefer `scripts/jj op revert <op>` / `op undo` (surgical, bounded by immutability) over `op restore` (repo-global; it also can't un-forget a workspace). If something is genuinely unexpected — stale/divergent working copy, `@` not where you left it — STOP and ask; never self-recover. Subagents are bound by all of this too.

## Feature workflow (`scripts/workflow`, run from `default`)

Each feature gets a claim commit **in default@'s linear history** (bookmarked NAME) plus an isolated jj workspace at `../NAME`.

- `scripts/workflow claim TODO` ticks TODO's box `[ ]→[/]` in `docs/todo.md` and creates workspace `../TODO`; `start NAME` is the same without a todo.
- `refresh NAME` reorders the claim to just under `default@` (feature current with trunk); `integrate NAME` refreshes, folds the feature into default@, ticks `[/]→[x]`, and archives the workspace to `../.integrated/`; `abandon NAME` discards the feature + claim, archiving to `../.abandoned/`.
- **Conflicts land in the feature workspace, not on trunk** — refresh/integrate exit 2 ("resolve it in ../NAME, then re-run") and never roll back. Resolve *inside* `../NAME` (edit markers + `scripts/jj squash`), then re-integrate. Resolve there, not from `default`: a `jj` pinned to `default` (e.g. a sourced workflow alias) inlines default@ onto the feature instead.

## CR citations

- Cite Comprehensive Rules in the `[CR#…]` bracket format — e.g. `[CR#704.5g]`, a list `[CR#601.2g,106.4]` (comma-separated, no spaces), a range `[CR#601.2a..601.2b]`. Never write a bare `CR 704.5g` or a loose `704.5g` in prose; the checker flags both.
- After adding or changing citations: `cargo xtask cite check --list-noncompliant` must be empty, and `cargo xtask cite check` must report 0 stale. When you cite a rule not yet in `cr-citations.lock`, run `cargo xtask cite bless` to register it.
- Rule numbers come from the CR, never from memory. Before committing citation changes, run `cargo xtask cite audit --diff` and read each rule's text against the claim citing it — the hash checker can NOT catch a right-number-wrong-topic cite. Give `bless`'s newly-registered list the same read.

## New jj workspaces

After creating a new jj workspace, give it the gitignored shared dirs. `data` and
`docs/superpowers` are symlinked back to the main checkout; `plugins/wizards` is
regenerated (it's all generated code). The repo's `data`/`docs/superpowers` ignores
are **dir-only** (trailing slash), which does NOT match a symlink — so without the
exclude step jj snapshots those symlinks into your commits.

```sh
# symlinks — mind the `..` depth (data is at the root; docs/superpowers sits in docs/)
ln -s ../deckmaste.rs/data ./data
ln -s ../../deckmaste.rs/docs/superpowers ./docs/superpowers

# ignore the symlink form (shared store; one-time per repo, not tracked)
printf '/data\n/docs/superpowers\n' >> .git/info/exclude

# wizards: generate a real dir (plugins/*/ already ignores it). Do NOT symlink it —
# the deckmaste_cards suite loads it, and a symlink would make generate write into the
# main checkout.
mkdir -p plugins/wizards && cargo xtask generate plugins/wizards
```

Verify with a real `jj st` (not `jj st --ignore-working-copy`, which skips the
snapshot and hides leaked symlinks): it must report no changes.
