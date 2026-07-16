---
needs: []
---
Migrate the repo from its vendored workflow tooling (`scripts/jj`,
`scripts/workflow`, `scripts/conflicts`, `scripts/hooks/jj_guard.fish`) to the
extracted **jj-workflow** plugin (github: `msmorgan/jj-workflow`), which is the
maintained superset of the same scripts (claim/integrate lifecycle, repair /
converge / resolve, `claim --into`, plus: `claim --or-start`, savepoint/rollback
on command failure, `abandon` that refuses un-integrated work without
`--force`, `integrate` that keeps the workspace parked on the integrated tip,
`conflicts auto` for alphabetized-list conflicts, worktree hooks bridging
Claude Code isolation to real jj workspaces, and smoke/pytest coverage).

Key semantic change: immutability moves from the per-op wrapper config to one
repo-config alias — `immutable_heads() = builtin_immutable_heads() |
(default@ ~ @)` — set once by `/jj-workflow:setup`. That *intentionally* locks
`default@` from feature workspaces (the wrapper's `default@-` leaves it
targetable — ruled wrong 2026-07-16) while `~ @` keeps the coordinator open.
Bare `jj` becomes legal; the plugin-registered guard hook only bans `git` and
the `--config`/`--config-file`/`--ignore-immutable` flags. This also retires
the drift class where wrapper semantics, CLAUDE.md prose, and the
locally-registered hook fall out of sync, and fixes the hook living only in
untracked `.claude/settings.local.json`.

Motivating incident (2026-07-16): a `workflow abandon` racing a concurrent
`integrate` resolved its stack revset across a foreign claim reordered above
it and abandoned two other features' commits (recovered via
`op revert`). The extraction's savepoint/rollback and refuse-by-default
abandon are designed to fail safe there.

Steps:
1. Install the plugin (`/plugin marketplace add msmorgan/jj-workflow`,
   `/plugin install jj-workflow@jj-workflow`), run `/jj-workflow:setup`.
2. Write `scripts/provision-workspace` from `scripts/workflow`'s
   `__provision_ws` (data/superpowers symlinks + wizards regen) and
   `jjworkflow.toml` (`todo_cmd = "scripts/todo"`).
3. Delete `scripts/jj`, `scripts/workflow`, `scripts/conflicts`,
   `scripts/hooks/jj_guard.fish`. Keep `scripts/todo` + `scripts/lib/`
   (the extraction consumes them via `todo_cmd`; `todo_graph.pl` is already
   byte-identical upstream).
4. Rewrite CLAUDE.md's Version-control / Feature-workflow sections for the
   bare-jj + repo-config model; update session memories to match.
5. Verify parity before cutover: watch for `default@` snapshot churn / empty
   pile-up under the new alias with two concurrent workspaces (the reason the
   wrapper retreated to `default@-` — commit `a2284a4e`); check `refresh
   --all` and `.integrated`/`.abandoned` archiving deltas (upstream keeps
   workspaces instead of archiving).
6. Migrate at a quiet moment: in-flight claims use the same claim-bookmark
   convention and survive, but concurrent sessions during the cutover do not.
