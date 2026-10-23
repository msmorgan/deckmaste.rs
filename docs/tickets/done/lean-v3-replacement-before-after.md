---
needs: []
---
# Split `Replacement.also` into `before` and `after`

Standard constraints apply. Approved in principle by the owner (2026-10-23). The evidence below
found no witness whose action happens after the replaced event: choices come before it, and
counters, attachment and exchanges happen with it. The owner settles the split's shape (before
and with, or choice and effect) before implementation.

## Problem

`Replacement.also` (`lean/Semantics/Abilities.lean`, the constructor beside `instead`) reads "The
event still happens, and `action` is done as part of it". It covers actions that happen before the
replaced event and actions that happen after it. The proposal splits it into `before` and `after`,
by whether the action happens before or after the replaced event.

"As [this] enters" actions are replacement effects [CR#614.1c]; a choice such as a color is made
before the permanent enters [CR#614.12a].

## Today's encodings

Every witness below is currently an `also`; none is an Entry Rider (`withRiders`). Rider
templates (`MoveRider`) are for "enters tapped", "with counters" and the like.

| Witness | Encoding today |
|---|---|
| `entersChoosing` / `entryChoice` | `Templates.lean` ~1739: `.replace (would subject enter) (.also (.choose …))` |
| `devour` | `Keywords.lean` `devourEntry`: `also (may (Action.devour …) (some (putCounters …)))` |
| `megamorph` | `Keywords.lean` ~1060: `replace (would any (turnFaceUp thisPermanent)) (doIf … (also (putCounters …)))`; a turn-face-up event, not an entering |
| `riot` | `Keywords.lean` ~1367: `also (mayOrElse (putCounters …) (gainHaste …))` on `would thisPermanent enter` |
| `unleash` | `Keywords.lean` ~1655: `also (may (putCounters …))` on enter |
| `tribute` | `Keywords.lean` ~1616: `also (sequentially [choose anOpponent, instruct … (may (putCounters …))])` on enter |
| enchant's non-spell entry | `Keywords.lean` ~613: `replace (would .any (.move thisAura … battlefield .arrival …)) (doIf … (also (attachTo …)))` |
| *Deadpool, Trading Card* | `Cards/Keyword.lean` `deadpoolTradingCard`: `also (may (establish (textChange … exchangeWith …)))` on enter |
| *Wolverine, Fierce Fighter* | not authored (no card; only the `wolverine` subtype exists) |

## Candidate witnesses, with the CR and Oracle checked

`before` candidates:

- *Wolverine, Fierce Fighter* (data/derived/cards.jsonl): "If damage would be dealt to Wolverine,
  instead that damage is dealt, but all other damage already dealt to him is healed." It is the
  only card of this shape the owner reported; I did not run a corpus search for others. Oracle
  confirmed. The text uses "instead", so it may be an `instead` of a sequence rather than an
  `also`; whether the healing precedes the damage is not said by the text ("already dealt" implies
  the healing concerns prior damage, so ordering against the new damage is not observable). VERIFY
  result: ordering is not stated by the Oracle.
- `entersChoosing`: [CR#614.12a] says the choice is made before the permanent enters. Ordering
  confirmed by the rule.
- `devour`: [CR#702.82a] "As this object enters, you may sacrifice … This permanent enters with N
  +1/+1 counters on it for each creature sacrificed this way." The sacrifice is before the entering
  and the counters ride the entering, so the single `also` holds both a before part (sacrifice)
  and an ride-along part (counters). Ordering confirmed, but one `also` here spans both.

`after` candidates:

- `megamorph`: [CR#702.37b] "As this permanent is turned face up, put a +1/+1 counter on it". The
  rule says "as", not "after"; the counter goes on the permanent as it turns up. Not an entering
  ([CR#614.1e] family), so not the same event as the others. The CR does not state after.
- `riot` [CR#702.136a]: "You may have this permanent enter with an additional +1/+1 counter on it.
  If you don't, it gains haste." `unleash` [CR#702.98a]: "You may have this permanent enter with an
  additional +1/+1 counter". The counter is part of the entering, and the choice is made before it.
  The rule says "enter with", which is simultaneous with the entering, neither before nor after.
- `tribute` [CR#702.104a]: "As this creature enters, choose an opponent. That player may put an
  additional N +1/+1 counters on it as it enters." The choice is before; the counters are "as it
  enters" (with it), not after. So `tribute` would be a before (choice) and a with-entering
  (counters), in one `also`.
- Enchant's non-spell entry [CR#303.4f]: the player chooses what the Aura will enchant "as the Aura
  enters"; the attachment itself is part of the entering. The rule gives a choice at entry, not an
  after.
- *Deadpool, Trading Card*: "As Deadpool enters, you may exchange his text box and another
  creature's." Oracle confirmed; the text says "as ... enters" ([CR#614.1c]), so the exchange is
  with the entering, not clearly after.

Summary of the VERIFY: the CR and Oracle never say "after" for any of these; they say "as ... enters"
or "enter with". Only the choices ([CR#614.12a], devour's sacrifice, tribute's choice, enchant's
choice) are clearly before. The riot, unleash, tribute and devour counters, Deadpool's exchange
and enchant's attachment are with the entering. So the split's `after` witnesses are, on the rule
text, "with the entering" rather than "after the event". A three-way split (before, with, after),
or a choice/effect split, is one thing the owner may want to consider. This is evidence, not a
decision.

## Ruling (owner, 2026-10-23)

Split `Replacement.also` into `before` and `with`, not `before` and `after`. `before` holds the
choices made before the replaced event (choosing as it enters [CR#614.12a], devour's sacrifice,
tribute's choice, enchant's choice [CR#303.4f]). `with` holds what is done as part of the event
("enters with", "as it enters": the riot, unleash, tribute and devour counters, the Aura's
attachment, *Deadpool*'s exchange). Megamorph's "as turned face up" [CR#614.1e] is a `with` on its
own event. *Wolverine*'s healing, whose order its text does not state, goes in whichever arm the
implementer can justify from the rules, with the reason recorded.

## Tests (once ruled)

- Each witness above keeps its current check result.
- A pin per witness for the arm it lands in, with the rule text above as its citation.
