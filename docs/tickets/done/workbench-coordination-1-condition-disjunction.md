# coordination-1: the condition disjunction and the round-1 ledger

Sub-round 1 of [workbench-coordination-family](workbench-coordination-family.md)
(the umbrella — authoritative for measurements and acceptance). Owns its
sections "The condition disjunction \"if X or if Y\"" and "Ledger from round 1
review".

The order is the umbrella's own: **the hand count comes first** — sort the 360
`if…or…` lines the way the conjunction round sorted its 182, report the true
count, and only then design the row. The 11 doubly-marked lines are a lower
bound, not a census.

Pins:
- The user's or-of-ands ruling (recorded in the umbrella, 2026-08-27) is the
  row's shape: disjunction over arms that are atoms or flat conjunctions, one
  level, nothing deeper; the reduplicated "if" is the scope marker.
  `FlatConjuncts` gets a COMPANION predicate, never a copy.
- The row sits beside `AndCond` and reaches all five carriers for free; an
  activation-guard carrier benches (the family's biggest carrier, 6 of 11).
- Round-1 ledger items are decided, not deferred: `otherwiseCtx`'s orientation
  asymmetry gets ONE answer for both orientations ([CR#601.2c]); `condDelta`'s
  docstring is narrowed or its rows extended; the consumerless macros
  (`ifThen`, `ifThenElse`, `onlyIf`, `onlyIfNot`, `whereLetterStatic`) are
  deleted or kept with a recorded reason; the `nn : NotConditional se` gate is
  asserted by a probe or retired with its reason.

Acceptance: the umbrella's first three acceptance lines. `idris/scripts/build`
PASS. Standard constraints apply.

## As landed

### The hand count, first

Population, taken over `jq 'select(.supported)'` alone: every `if`-condition
containing a coordinating "or". The comparator idiom is TOKENISED, not dropped
by line — dropping the line loses Bonecache Overseer and Reptilian Recruiter,
two of the eleven doubly-marked cards, whose disjunction sits beside a "three or
more" / "2 or less" in the same clause. The condition window runs from "if" to
the comma that closes it (leading) or to the end of the sentence (postposed), so
an "or" in the consequent is not counted.

**386 occurrences over 259 distinct condition texts, across 461 supported
cards.** Sorted by hand:

| bucket | occ | texts | cards |
| --- | ---: | ---: | ---: |
| (a) doubly marked, "if X or if Y" | 11 | 7 | 11 |
| (b) singly marked condition disjunction | 98 | 62 | 112 |
| (c) one-condition coordination (the `Predicate.Or` family) | 209 | 160 | 254 |
| (d) other / false positive | 68 | 30 | 85 |

**The true count is (a) + (b): 109 occurrences over 69 distinct condition texts,
123 supported cards.** The umbrella's 11 doubly-marked cards are a lower bound
by a factor of eleven, and the row is sized against 123, not 11.

(b) splits in two, and the split is what the row has to reach:

- **b1, two independent finite clauses — 40 occ / 24 texts / 46 cards.** "if you
  control a Desert or there is a Desert card in your graveyard" (6 cards); "if a
  nonland permanent left the battlefield this turn or a spell was warped this
  turn" (13); the will-of-the-council cycle's "if [choice] gets more votes or the
  vote is tied" (13); Sharp-Eyed Rookie's disjunction of two comparisons; Bazaar
  of Wonders, Churning Reservoir, Anafenza, Avengers Assemble!, Sword of Hours.
- **b2, one subject, two predicates — 58 occ / 38 texts / 66 cards.** The shield
  counter's "If it would be dealt damage or destroyed" (13 occ); "attacked or
  blocked"; "entered from your graveyard or you cast it from your graveyard";
  and the ZONE disjunction on a guard — "is in the command zone or on the
  battlefield" (Arahbo, Edgar Markov, Inalla, Sidar Jabari), "is in your
  graveyard or on the battlefield" (Firemane Angel), "on the battlefield or in
  your graveyard" (Skyblade's Boon), which is Quakebringer's sentence singly
  marked.

(c) is what the umbrella predicted: the largest bucket, and every one of it is a
coordination INSIDE one condition — a noun ("a spell or ability"), a type
("an instant or sorcery card"), a colour, a damage recipient ("a permanent or
player", ~40 replacement-effect lines alone), a characteristic ("greater power
or toughness", the routed Talion question).

(d) breaks down as: 19 occ of banding REMINDER text; 12 occ of the superlative
tie idiom ("the greatest power or tied for the greatest power"); 26 occ of
"and/or", which the umbrella already owns as a third coordinator; 2 amount
disjunctions ("exactly zero or seven cards", "the first or second time"); 2
false positives where the "or" is in the effect ("if able and … can't attack or
block"). No comparator residue survives: tokenising removed it.

Out of the ticket's population but measured in passing, so the next round does
not re-buy it: **"as long as … or …" is 32 occurrences**, and it carries the
same three kinds — b1 ("as long as you control a land creature or a land entered
the battlefield under your control this turn"; "as long as you control a Desert
or there is a Desert card in your graveyard"), b2, and the (c) majority. The row
reaches the static carriers for free, so these are already paid for.

### The row

`Condition.OrCond cs` beside `AndCond`, gated `TwoDisjuncts` (the arity demand,
sharing `atLeastTwoCs` — the count does not care which word joins) and
`FlatDisjuncts`.

`FlatDisjuncts` is `FlatConjuncts`' **companion and not its copy**: it ADMITS
the conjunction its twin refuses and refuses only a nested disjunction, so an
arm is an atom or a flat conjunction — the user's ruling, one level, nothing
deeper. `isCondCoord` split into `isAndCond` / `isOrCond` so each row's gate
names the constructor it actually refuses. Verified by scratch probe: a nested
`OrCond` is refused (`Can't find an implementation for So (flatDisjuncts …)`),
a one-armed `OrCond` is refused, an `OrCond` arm under `AndCond` is admitted.

`condDelta (OrCond _) = []` — a disjunction cannot say which arm held, so
summing the arms would hand the consequent a phrase from an arm that may have
been false. `AndCond` still sums, since every conjunct held. No supported line
reads back from a disjunct. `condNegated (OrCond _) = False`; "unless (A or B)"
is `NotCond (OrCond …)` and stays writable.

Total-table cost paid: three rows (`condDelta`, `condNegated`, the flatness
predicate). `condRemarkAt`'s catch-all already covers a coordination and its
docstring already says why.

**The mirror nesting is left open, deliberately.** `flatConjuncts` refuses only
`AndCond` arms, so `AndCond [c, OrCond […]]` now types — which is Primeval
Spawn, "If this creature would enter and it wasn't cast or no mana was spent to
cast it", the one supported line of that shape. Nothing in print marks its
scope, the user's ruling covers the OR row only, and narrowing it would refuse
an attested sentence. Recorded at the constructor.

### Benches — four, over two carriers and both markings

- `quakebringerDamage` — the or-of-AND whole, on the trigger's intervening "if"
  [CR#603.4]: `OrCond [Matches This (InZone battlefield), AndCond [Matches This
  (InZone (graveyardOf You)), Exists (a Giant you control)]]`. The user's tree,
  term for term.
- `darkFortressMana` — the ACTIVATION GUARD, doubly marked: "{T}: Add {B} or
  {R}. Activate only if this land entered this turn or if you control a basic
  land." Gathering Place, Gleaming Bastion, Hidden Lair and Training Compound
  are the same line.
- `sandStranglerDamage` — b1 singly marked, two independent clauses, on the
  intervening "if". Desert's Hold, Gilded Cerodon, Unquenchable Thirst, Wall of
  Forgotten Pharaohs and Wretched Camel are the same condition.
- `skybladesBoonReturn` — b2's zone disjunction on an activation guard, singly
  marked.

No pin minted. A nested disjunction and a one-armed one are refused by a
canonical-form gate, not by the rules, so under
`docs/memory/rulings/measurements-live-in-pins.md` neither is a pin candidate;
the gates are asserted by the scratch probes above and documented at their site.

### Ledger from round 1 — every item decided

- **`otherwiseCtx`'s orientation asymmetry: ONE answer, "the arm reads what the
  condition announced".** `OnlyIf`'s otherwise arm is now typed at `condDelta c
  ++ otherwiseCtx e`. Leading `If` already had it structurally (its consequent
  is typed at `condIntro c`, so `otherwiseCtx` carries `condDelta c` through);
  the postposed row had to write the same term explicitly because its consequent
  is typed at `bs`. [CR#601.2c] announces targets at casting, before either arm
  runs, so an arm's scope cannot depend on which side of the clause the
  condition was written on. The asymmetry was reachable — postposed "if" with an
  "Otherwise" is attested (Caustic Bronco and kin) — though no printed line
  writes a target inside a postposed condition, so this is a regularity call and
  not a card's blocker. The failed-comparison margin over-generates alike at
  both orientations now; round 1 recorded it at one, this records it at both,
  and stripping it still needs an announcement-delta the grammar lacks.
- **`condDelta`'s docstring narrowed, rows unchanged.** The printed evidence
  says there is no missing row: `Exists` and `DealtThisWay` take a `Predicate`,
  which describes and mentions nothing; `ExistsGroup` is gated
  `CountedExistential` (a counted described group); `Matches` is gated
  `Bindingless`. A target rides a determiner on a mention, and `CompareAmt`'s two
  amounts are the only place a condition writes one. So the table is not
  partial — the docstring was over-broad, and it now says how far the principle
  reaches and why. It was also stale in its last sentence, which still claimed a
  lookback introduces nothing after round 1 gave `Happened` its subject delta;
  corrected.
- **The consumerless macros — two of the five premises were wrong.**
  - `ifThen`: **KEPT.** It has ten consumers (nine in `Cards.idr`, one in
    `ProofsG`); it is not consumerless.
  - `whereLetterStatic`: **nothing to delete.** Neither it nor `whereLetter`
    exists anywhere in the tree; round 1's close records adding them and it is
    stale. The letter definition is `Effect.Define` / `StaticEffect.Define`,
    binder-first, with no English-order macro over it. Whether one is wanted is
    round 1's question re-opened, not this round's.
  - `ifThenElse`: **DELETED.** No consumer, and the name shadows the Prelude
    function `if … then … else` desugars to, so it was a hazard as well as dead.
  - `onlyIf`, `onlyIfNot`: **DELETED.** No consumer; the seven postposed bench
    sites write `OnlyIf … Nothing` directly.
- **`nn : NotConditional se`: KEPT, unasserted, with the reason written at
  `notConditional`.** Neither offered option is honest. A probe would be a proof
  refusing a rules-MEANINGFUL term — [CR#603.4] leaves "if" its normal English
  meaning everywhere it is not an intervening clause — which is exactly the pin
  round 1 retired, and `measurements-live-in-pins.md` forbids it. Retiring the
  gate is not right either: it is not a canonical-form preference, because
  `AndCond` is not the same term. `AndCond` holds every conjunct at `bs`, where
  a nesting would type the inner condition at `condIntro` of the outer and let
  it read what the outer announced. So the gate withholds real width, and no
  supported line pays for it — the eight "as long as … as long as" lines are
  independent statements. It stays until one does, and the docstring now says
  that instead of leaving it silent.

### Remainders, explicitly

- **Mythos of Nethroi does not bench.** Its second disjunct is "if {G}{W} was
  spent to cast this spell", and no mana-spent condition exists in the grammar.
  The disjunction is not its blocker; that row is.
- **Primeval Spawn does not bench** for the same reason ("it wasn't cast or no
  mana was spent to cast it") plus its "would enter … instead" replacement.
- The 112 singly-marked cards are now writable in principle; four benched.
- The closure grid: this round moved no `negatable` cell — `OrCond` is a
  `Condition` constructor, and that census is over `Predicate`.
- Pre-existing gate defect, NOT this round's and untouched: `cargo xtask cite
  check` reported 1 stale — an unlocked opening-hand rule cited in
  `docs/tickets/planned/workbench-turn-structure-and-procedures.md`.
  RESOLVED since; `cite check` reports 0 stale as of 2026-08-27. (The rule
  number was written bare here, which the compliance checker flags in its own
  right; rephrased.)
