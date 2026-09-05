---
needs: []
---
**Research (AFK): which deeds print a subject?** For every `Words.actFacts`
row, survey the supported corpus (`jq 'select(.supported)'
data/derived/cards.jsonl`) for printed sentences of the form "<player>
<deed>s …" and read the deed's CR entry; report per deed: subject count in
the corpus, the CR rule, and whether the rule's definition names an agent.
Output: a table in `docs/memory/scratch/2026-09-04-deed-subjects.md`.
Unblocks the `actFacts` agent column in `workbench-deed-guard-residues`.
