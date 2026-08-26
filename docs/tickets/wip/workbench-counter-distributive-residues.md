---
needs: []
---
# The counter family's distributive residues and its two table holes

`docs/tickets/done/workbench-counter-family-residues.md` closed the counter
region and left three constructions explicitly OUT of its scope, plus two table
holes named by neighbouring rounds. Nothing owns them now that the family ticket
is closed.

## 1. The player-side distributive `get` — Winding Constrictor, line 2

From that round's ledger:

> "If you would **get** one or more counters" is the player-side distributive —
> `GetsCounters`' twin of `PutCountersOfThoseKinds`, unbuilt.

`PutCountersOfThoseKinds` landed for the object side; this is the same
kind-blind distributive with a player recipient and the `get` verb.

## 2. Aragorn — the same anaphor, a different frame

> same anaphor as `PutCountersOfThoseKinds`, but the recipient "up to one other
> target creature" has no row and the clause is a trigger body rather than a
> replacement.

Two independent deltas: the "up to one other target" recipient shape, and the
distributive kind anaphor in a trigger body rather than under `Intercepts`.

## 3. "The same number and kind of counters" — Captain Marvel, Denry Klin

> "the same number and kind of counters" / "the same number of each kind of
> counter" are the "counter of that kind" axis, deliberately not swept in.

The counter round drew its boundary here on purpose. This is that axis.

## 4. `EntryCounterMark` has no "fewer" arm

`docs/tickets/done/workbench-pins-refuse-rules-impossibility-only.md` carries it
as consolidated open gap 12, with Nahiri's Compleated as the carrier. The entry
mark rows the more/additional direction only.

## 5. `effEq` ignores `LosesCounters`' amount slot

From `docs/tickets/done/workbench-closure-flip-risks.md`:

> `effEq` ignores `LosesCounters`' new amount slot, matching the neighbouring
> `GetsCounters` row which compares nothing at all. If `effEq` ever becomes
> load-bearing for counter clauses, both rows need the amount.

Latent, not currently wrong. Fix both rows or record why the equality is
deliberately coarse — do not fix one.

## 6. The Suspect counter kind

`docs/tickets/done/workbench-amount-comparison-and-quantity.md` names it as
Investigator's Journal's remaining blocker ("enters with a number of suspect
counters on it equal to …"), outside that ticket's counter scope. One catalog
row; take it with the rest.

## Consumption boundary

`idris/src/Experimental.idr` (`GetsCounters`, `PutCountersOfThoseKinds`,
`EntryCounterMark`, `effEq`, the recipient shapes), `idris/src/Experimental/Words.idr`
(the counter-kind catalog), the pin modules `idris/src/Experimental/Proofs*.idr`,
and the evidence bench `idris/src/Experimental/Cards.idr`. No Rust crate.

## Acceptance

- Winding Constrictor's second line and Aragorn each bench or are reduced to a
  named blocker that is not the distributive.
- The "same number and kind" axis lands or is named at its measured size.
- `effEq`'s two counter rows agree with each other, whichever way.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.
