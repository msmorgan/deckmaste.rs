---
needs: []
---
# Name the mtg-rules skill in CLAUDE.md for rules-modeling work

Add one line (about 60 tokens) to the "CR citations" section of `CLAUDE.md` (`AGENTS.md` is a
symlink to it, so Codex reads the same line). Agents here never load the mtg-rules skill: session
transcripts show 0 mtg-rules Skill calls from deckmaste.rs Claude subagents against 165 jj-sensei
calls, because the current instructions route CR work only to `cargo xtask cite`.

What the line says:

- `cargo xtask cite` checks that cited rules exist and pins their text.
- The mtg-rules skill supplies rule text, card data, rulings and maps of the rules a subsystem
  spans. Before designing or auditing anything that encodes the rules, read the matching map
  section (`scripts/ref <map> <section>`).
- Cite rules, not maps.

Gate: the line names `scripts/ref`, which ships in mtg-rules v1.11.0 (the local jj stack in
`~/Projects/agent-plugins/mtg-rules`, not yet pushed). As of 2026-10-21 the installed plugin is
1.10.1. Land the line once `~/.claude/plugins` has 1.11.0; landing earlier, the line must say that
`ref` needs mtg-rules ≥ 1.11.0.

Use the `writing-for-agents` skill for the edit.
