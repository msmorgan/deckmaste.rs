---
needs: []
---
# A "this way" lookback is not checked against an earlier event in the ability

Standard constraints apply.

## Defect (verified 2026-10-21)

`LookbackSpan.thisWay` (`Abilities.lean:140`) is documented as "the event the enclosing action
caused". The checker never looks for that event. Both of these check clean with nothing before
the lookback to refer to:

```lean
sequentially [draw (.lit 1), doIf (did you (search yourLibrary) thisWay) shuffle]
sequentially [draw (.lit 1), doIf (did you (sacrifice (a isPermanent)) thisWay) (draw (.lit 1))]
```

Oracle: "Draw a card. If you search your library this way, shuffle" is meaningless: nothing in
the ability searched ([CR#608.2c]: later text can modify the meaning of earlier text, and "this
way" refers to what the earlier text did). A real search is [CR#701.23a], a real sacrifice
[CR#701.21a]. `stepDidClauseWith` (`Check/Phrase.lean:1056`) checks the clause's own shape
(event head, locus, actor kind) and then `stepLookbackSpanWith` (`:921`), whose `.thisWay |
.triggering` case is `(drs, [])`. `Condition.did` (`:2731`) and `Amount.tally` (`:2663`) reach it
that way. The same holds for discard (`did you (discard (a isCard)) thisWay`) and for an
`eventTally … thisWay` after an unrelated step. `.triggering` is likewise unchecked against the
ability's trigger; include it.

Participial Descriptions ("each card discarded this way") are not affected: they read stamps and
refuse when no Deed or Zone Change is in scope (`participialDescription`). This ticket is the
`Condition.did` and `Amount.tally` forms.

## Sites that rely on it today (all genuine; they must keep passing)

`doIf (did you (search yourLibrary) thisWay) shuffle` at `Cards/Mana.lean:879`,
`Cards/Trigger.lean:691`, `Cards/Choice.lean:1187`, `Cards/Keyword.lean:1145`;
`did which (sacrifice (a isPermanent)) thisWay` at `Cards/Anaphora.lean:689`; and the
`eventTally … thisWay` at `Cards/Anaphora.lean:492`.

## Proposed fix

- Record, on the DRS, the events the ability's earlier steps caused (the Deed or Zone Change and
  its performer), as the participial stamps already record what happened to members.
- A `thisWay` lookback is satisfied only by an earlier recorded event of the same class whose
  performer is compatible with the clause's subject; otherwise refuse with a new
  `thisWayUngrounded` refusal (name at the implementer's discretion, add it to
  `Check/Refusal.lean`).
- "This way" reads only its own ability's events, not those of a linked ability (glossary entry
  **Participial Description**).
- Decide, and write down, how a reflexive "when you do" and a `may (...)` that was declined
  count: a declined optional action caused no event, so "if you do" after it must stay a
  condition on the choice, not on an event. Do not weaken any case in the sites above.

## Tests to add

- The two failing examples above become refusal pins.
- A positive and a negative pin each for search, sacrifice, discard, exile, and tally forms.
- A pin that an event in a different ability (a linked one) does not ground a `thisWay`.
- Re-run every site above unchanged; any that now refuses is a genuine finding, fixed in the
  card or the checker, never by dropping the check.

Gates: `./lean/scripts/build`, `cargo xtask lean check`.
