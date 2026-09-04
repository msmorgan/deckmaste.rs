---
needs: []
---
**One `scry`, one player ref, one clause.** Ruling 2026-09-04 on the look
bodies (review R6, fused-variants-3 STOP 2). `Macros.scryBody` (40 lines,
four clauses) and `surveilBody` (36 lines) match on the `LookAgent`/`LookReq`
witnesses (50 more lines) to pick "you" versus "that player", because `You`
leaves no binding a pronoun can read; `fateseal` hard-codes `anOpponent`.
About 125 lines for two keyword actions whose printed text is one sentence.

Fix: a marker noun for "the player performing this keyword action" — one
core `Noun bs Player` constructor, no delta, singular, resolved by the
enclosing `Enact` (RON-shaped: keyword-action expansions already substitute
the agent, so the re-emitter produces it). Then `scry agent amt`,
`surveil agent amt` and `fateseal agent amt` are each one macro over one
player noun whose body is one clause written agent-relatively, the way the
RON expansion is written [CR#701.22a,701.29a,701.25a] (read each); the
obligations live on the macro's type and are searched at the card site.
Delete `LookAgent`, `LookReq`, `scryBody`, `surveilBody`, `lookAgentTop`,
`lookedAgentTop` and the `You`/`They` case split. Re-spell every bench site
(16 today) and the look pins; probe non-vacuous. No matching on implicits
anywhere in the result.

Size: M. Done when: the three macros are one clause each over one player
ref; the witness types are gone from the tree; every scry/surveil/fateseal
witness typechecks; build at its module count. Standard constraints apply,
including the RON-shaped constraint.
