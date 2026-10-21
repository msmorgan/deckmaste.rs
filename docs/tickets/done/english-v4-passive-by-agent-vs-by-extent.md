---
needs: []
---
# Passive *by* phrase: agent or extent

*Its foretell cost is equal to its mana cost reduced by {2}* reads *by {2}*
two ways. It is the agent of the passive of *reduce*'s `Monotransitive(NP)`
frame (`/passive+by`). It is also the `PP[by]` Complement of its
`TransitiveWithPP(NP,PP[by|to])` frame, which is right. The same split gives
33 surplus Readings across the corpus, measured in
`english-v4-byte-identical-duplicate-readings`. The agent Reading is wrong
because an amount or mana symbol (*{2}*, *1*) is never an agent.

## Change

- When the verb's frame selects a `PP[by]` Complement, its agentive
  passive-plus-*by* Reading should not also take an amount or a mana symbol as
  the agent. Do this in frame compilation or the passive Schema, through the
  categories involved (Numeral, mana Symbol, measure Noun Phrase), never
  through a flag specific to one verb.
- Keep the agent where it is genuine (*dealt damage by a red instant or
  sorcery spell*, *attacked this turn by an Assassin*).

## Done when

- *reduced by {2}* has only the `PP[by]` Complement Reading.
- The 33 surplus Readings are gone.
- No face is lost.

Standard constraints apply.

Census delta (measured on uprxkopx): faces read 28,709 → 28,709; total Readings 328,602 → 328,186; lost faces: none.
Judgment set: 2,966 enforced at `COMPLETED_STRATUM` 6, all hold; 195 pending.
