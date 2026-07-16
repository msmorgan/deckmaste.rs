---
needs: []
---
Parse the look-at-top-of-library manipulation family: `Look at the top N
cards of your library. Put one of them into your hand and the rest on the
bottom of your library [in any order].`, `… You may reveal a <filter> card
from among them and put it into your hand …`, `… exile any number of them
and put the rest back`, plus the single-card forms (`Look at the top card of
your library. You may put it into your graveyard.`) and `Exile the top card
of your library.` moves.

The grammar already binds peeked cards — `Existing(TopOfLibrary(count: …))`
is exactly what the `Scry`/`Surveil` macros are built on
(`plugins/builtin/macros/action/Scry.ron`) — and the arrangement semantics
were settled by `engine-scry-surveil-explore` (done). This is the effect
production lowering the English chains onto that binder + move/reveal modes.

The IMPULSE tail (`… You may play it this turn.`) additionally needs the
play-from-exile permission tracked by `engine-cast-from-zones` (planned);
cards with that tail stay blocked until it lands — the rest of the family
graduates without it, so it is not a hard dependency here.

**~484 of 17,022 one-away cards** (2026-07-16 tally).

Verify: `cargo xtask generate plugins/wizards` graduation delta;
`cargo xtask fidelity` on a dig-to-hand and an exile-the-rest card.
