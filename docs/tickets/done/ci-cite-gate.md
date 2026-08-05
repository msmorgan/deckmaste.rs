---
needs: []
---
**Make citation checking repository-owned and run it in CI.** The workflow
currently runs only `cargo xtask cite coverage --check`, the self-contained
path-heuristic ratchet. The checker the README advertises, `cargo xtask cite
check`, is a shim over an uncommitted
`~/.claude/skills/mtg-rules/scripts/cite`; a cold clone cannot run it even
after fetching the complete dataset.

Move the checker into `xtask` and make this repository its authoritative
implementation. Preserve the existing command surface (`check
[--list-noncompliant]`, `bless`, `list`, `show`, `diff`, and `audit`) and the
`cite-config.json`/lockfile formats so the migration does not silently weaken
the gate. Remove the home-directory lookup and `MTG_RULES_CITE` escape hatch;
the mtg-rules skill may wrap the repository command when operating here, but
CI and contributors must not require the skill.

The CI job already stages and digest-verifies `data/rules/cr.txt`. Reuse that
exact file for citation checking; do not fetch a second CR or choose an
independent latest-version policy inside this ticket. Run `cargo xtask cite
check` after the coverage ratchet and require both zero stale citations and an
empty `--list-noncompliant` result.

Acceptance:

- focused fixtures cover range expansion, stale text hashes, malformed and
  noncompliant citations, and lockfile updates;
- the full command surface works from a cold clone after
  `scripts/fetch_data`, with no files expected under `$HOME`;
- CI runs the real checker against the same pinned CR used to derive its
  catalogs; and
- the ci.yml comment documenting the missing skill is deleted with the gap.
