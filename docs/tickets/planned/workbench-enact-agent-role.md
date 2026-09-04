---
needs: []
---
**Gate `Effect.Enact`'s agent slot on the facts table's agent role.**
Cleanroom review 3, 2026-09-04, finding U3.

- `Effect.Enact` takes `subj : Maybe (Noun bs Player)` with no voice check,
  while `Triggers.VerbedEvent` gates on `So (verbedVoiceOk v who what)`. So
  `Enact (Just You) "Destroy" (Move (target creature) graveyardZ [])` is
  admitted although `Words.actFacts "Destroy"` gives Destroy no agent role.
  Add `{auto 0 ag : So (enactAgentOk subj v)}`, with `enactAgentOk Nothing v =
  True` and `enactAgentOk (Just _) v = elem Player (roleKinds (agentRole
  facts))`.
- The macros then disagree with the table in the other direction:
  `Macros.exile` enacts `Just agent` over `"Exile"`, whose row is `noRole`, so
  13 bench sites spell "Exile target creature" with an agent the card never
  prints, while `Macros.destroy` is agentless. Make `exile` agentless and keep
  a separate `exiles agent n` for the "you exile"/"that player exiles"
  sentences that do print; re-spell each of the 13 by what its card prints.
- Pin an agented enact of an agentless verb, with its positive twin in the
  same module.

Size: S. Done when: `enactAgentOk` gates every `Enact`; no witness carries an
agent its card does not print; the pin probes non-vacuous; build at its module
count. Standard constraints apply, including the RON-shaped constraint.
