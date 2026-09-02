# workbench-element-binder-remainders

The two remainders from workbench-element-binder-reads (done 2026-08-27; its
main item needed nothing — the per-member event count already wrote, Thought
Sponge benches whole).

## The ordinal cast read — 5 supported cards

"The second spell you cast each turn" family (Zimone, Infinite Analyst + 4
others; the round confirmed the census). Three blockers, all recorded in the
done ticket:

- `Triggers` imports `Phrase`, so a `Noun` can never take a `GameEvent`; the
  read must be `EventName`-keyed.
- `Lookback` has no recurring (each-turn) window.
- The grant lines are [CR#611.2f]'s regime while the five landed lines are
  [CR#611.3]'s; the two must not be conflated.

Once Upon a Time's "the first spell you've cast this game" travels WITH this
item — it is the same ordinal element read (game-scoped window), no longer an
independent dormant line.

## The each-player singular binder — 1 supported card

Eager Construct: a binder over "each player" whose body reads a SINGULAR member
choice. Measured at 1; blocker pinned as `eachPlayerBindsNoSingular`. Open only
if a second witness appears or the ordinal work makes it free.

- **Routed from workbench-description-2 (close, 2026-08-28):** the loop over a
  PLAYER group — `ForEachOf` takes an object group, so "for each opponent,
  goad target creature that player controls" (Frenzied Gorespawn's real last
  blocker) has no binder. And Oath of Mages' ordinal player read joins the
  ordinal family here.

- **Confirmed still open (workbench-event-zone-and-cast-provenance close,
  2026-09-02):** the umbrella declined the ordinal binder as off-lane — this
  ticket holds it, with the 5 supported cards and the three blockers above.

## As landed

Four items, two built and two ending in written verdicts.

**The ordinal cast read — built and benched whole.** `NthCastBy (ord) (who)
(per)` joins `CastBy` in `Phrase.idr`, with `RankPeriod` (`Words.idr`, beside
`TurnPart`) saying over what period the rank counts. All three recorded
blockers answered:

- *A `Noun` cannot take a `GameEvent`.* It does not have to. The read is a
  PREDICATE on the object, keyed by nothing but the caster and the rank —
  `CastBy`'s own shape, one slot wider. No event term crosses the module
  boundary.
- *`Lookback` has no recurring window.* It does not get one. `RankPeriod`
  splits `RankWithin Lookback` from `RankEach TurnPart`, on the argument
  `NthOccurrence` already makes on the event side: a `Lookback` names a
  stretch a retrospective reader scopes to and `sameWindow` compares two such
  stretches, while "each turn" says a COUNT resets. Adding an `EachTurn` arm
  to `Lookback` would have put a non-stretch into every reader that compares
  stretches.
- *[CR#611.2f] against [CR#611.3].* Kept apart in the row's own docstring and
  in the bench comment. The five landed lines are static abilities, so
  [CR#611.3a]'s "applies at any given moment to whatever its text indicates"
  is what makes the rank readable live; [CR#611.2f]'s "the NEXT spell you
  cast" is a one-shot that begins to apply when the player next puts a spell
  on the stack, and this arm may not be used to spell it.

One further gate was needed and is the interesting one: `Definite` demands
`Uniquifying`, and the rank is uniquifying — [CR#601.2] puts each spell on the
stack in its turn, so a player's casts inside a period are a sequence and the
Nth of it is one member. That is what lets the subject be the printed definite
singular ("THE second spell you cast each turn") instead of the medallions'
`AllOf` class.

Re-measured 2026-09-02: the "second spell you cast each turn" census is **5
supported cards**, confirmed — Alisaie Leveilleur, Highspire Bell-Ringer, Monk
Class, Raging Battle Mouse, Uthros Psionicist. The wider ordinal-cast family
the same read serves is larger (33 lines write "the [ord] [description] spell
[who] cast(s)", 30 of them with "each turn").
Bench: **Highspire Bell-Ringer whole**, plus `onceUponATimeFirstCast` for the
game-period reading (`RankWithin ThisGame`), which travels with the item as
the ticket said.

**The routed player-group loop — free, and benched.** The probe the ticket
asked for came back positive: `ForEachOf` is kind-indexed and `elemIntro`
carries `PhPlayer`, so a player group already loops (Blatant Thievery is the
standing witness). Frenzied Gorespawn's goad line needed no grammar at all —
`frenziedGorespawnGoad` benches "for each opponent, goad target creature that
player controls", the member read back as `That PlayerW`. A fragment: the
card's second line triggers on "one or more creatures attack one of your
opponents", which is not this item's.

**The each-player singular binder — the item OPENS, and is not this tail's.**
Re-probed as instructed. The turn round's kind-indexing did not free it:
`mayCtx (Just d) = nomIntro d`, so an offer whose decider is `Each AnyPlayer`
still hands its body no singular member, where `Does` hands its body
`agentIntro`. Two findings:

1. The one-line half is clear — `mayCtx` wants `agentIntro`, exactly as `Does`
   took it, and `agentIntro (Each p)` already binds the member `TheD OneOf`.
2. It is not enough for a witness. Eager Construct's body is a scry, and
   `Macros.scry` hardcodes `Does You "Scry"` over `You`'s own library. Every
   member-reading offer in the corpus has the same second blocker: the keyword
   -action macros are written at the controller's seat.

And the ticket's gate ("open only if a second witness appears") is met several
times over: **35 supported lines write "each player may [verb]"**, and the
member-reading ones are the majority — "each player may search THEIR library"
(5), "each player may discard THEIR hand and draw" (6), "each player may
shuffle their hand and graveyard" (2), among others. So the item opens on
witness count, but what it wants is a round that re-seats the keyword-action
macros on a written agent, not a one-line change here. Nothing was half-built:
`mayCtx` is untouched, because a row is bought by a witness and this one is
not payable yet. **Route as a planned ticket at integrate.**

**Oath of Mages' ordinal player read — designed, blocked behind a different
clause.** Re-measured 2026-09-02: **6 supported cards** write "the first
player"/"the second player" — Cruel Entertainment and the five Oaths (Druids,
Ghouls, Lieges, Mages, Scholars). The design is settled and recorded here so
the next round need not re-derive it:

> `NthPlayer : (ord : Ordinal) -> {auto 0 ok : So (rankInScope ord (countOnes
> Player bs))} -> Noun bs Player`, a rank over the singular player mentions in
> scope. It RANKS the prefix, which `ItPrior` deliberately refuses to do, and
> the difference is the printed word: a pronoun states no rank, so ranking the
> mentions for it would invent a reading the card does not write, while an
> ordinal states one and the rank is the whole of what these lines say. `They`
> is the spelling wherever one player is in scope and refuses at two, which is
> exactly where this begins.

It was NOT built, because no witness pays for it. All six cards reach the
ordinal only through a clause that is itself unbuilt — the Oath cycle's "that
player chooses target player who has more [X] than they do and is their
opponent", and Cruel Entertainment's player-controls-a-player effect. A row
whose only witnesses are blocked behind a second construction is unbought, so
the ordinal player read waits on that chooser clause. **Route as a planned
ticket at integrate**, together with the design above.
