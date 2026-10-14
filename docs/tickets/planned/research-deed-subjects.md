---
needs: []
---
**[port-required: semantics-v3] Research (AFK): which deeds print a subject?**
For every `Words.actFacts` row (`lean/Semantics/Check/Words.lean`), survey the
supported corpus (`jq 'select(.supported)' data/derived/cards.jsonl`) for
printed sentences of the form "<player> <deed>s …" and read the deed's CR
entry; report per deed: subject count in the corpus, the CR rule, and whether
the rule's definition names an agent. Output: a table in
`docs/memory/scratch/2026-09-04-deed-subjects.md`. Informs the `actFacts` agent
column in `lean/Semantics/Check/Words.lean` under the `semantics-v3` Unified
Action-Clause Spine (`does` / `would` / `cant` / `did` / `instruct` subject
validation, `docs/decisions/semantics-v3.md` §3).
