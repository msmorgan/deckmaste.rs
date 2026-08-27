---
needs: []
---
# Build the n-ary event disjunction seat

Both arms the seat waited on are landed (`BecomesTarget`, `Leaves` with its
source slot — event-zone sub-rounds 3, close 2026-08-26), so the recorded
shape is now implementable. The shape (settled in
`done/workbench-payment-events-and-replacement-disjunction`, restated on the
`GameEvent`/`AltEvent` docstrings — do not re-derive): a SEAT slot, n-ary,
never a `GameEvent` row; `AltEvent`'s binary becomes an arm LIST gated arm by
arm; `headerCtx`'s whole-agreement (`sameBindings`) read becomes a fold over
the list; `Delayed` and `Intercepts` gain the same slot (they have none
today). `Interceptable` distributes arm-by-arm. Carriers: Giggling
Skitterspike, Trouble in Pairs, Syr Konrad (header); Repeated Reverberation
(delayed); Illusionary Mask (replacement `would`) — 5 lines, 3 seats;
re-verify each against current vocabulary before benching.

## Consumption boundary

`idris/src/Experimental/Triggers.idr` (`AltEvent`, `headerCtx`), `Effect.idr`
(`Triggered`, `Delayed`, `Intercepts`), `Cards.idr` bench, `Proofs*.idr`.
No Rust crate.

## Acceptance

- At least one carrier per seat benches its disjunction, or the seat's
  remaining blocker is named per carrier.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

## As-landed

**The slot is n-ary at all three seats, gated arm by arm with the gate that
seat already puts on its head event, and read back by one shared fold.**

### Signatures

- `AltEvent : TriggerWord -> List (GameEvent bs) -> Type`, with `NoAlt` for
  the empty list and `MoreAlt` consing one arm under `HeaderNontarget e` plus
  the tail's own `AltEvent`. The binary `OneAlt` is gone; the list generalizes
  it.
- `Triggered : (word) -> (ev) -> (alts : List (GameEvent bs)) -> (window) ->
  (limit) -> (intervening : Maybe (Condition (headerCtx alts ev))) -> (eff) ->
  … {auto 0 ae : AltEvent word alts} -> {auto 0 cd : ChapterDefaults ev alts …}`.
- `Delayed : (ev) -> (alts : List (GameEvent bs)) -> (span) ->
  Effect (delayedCtx alts ev) -> …`.
- `Intercepts : (ev) -> (alts : List (GameEvent bs)) ->
  (repl : Effect (interceptCtx alts ev)) -> (use) -> {auto 0 ok : Interceptable ev} ->
  {auto 0 oks : InterceptableArms alts}`.

The arms sit immediately after the head event at every seat, because the body's
type depends on them. `Interceptable` distributes through `interceptArmsOk`,
which asks each arm the same `interceptOk (eventName …)` question the head is
asked — the `eventName` story verdict 2 recorded, in code. `Delayed` puts no
per-arm gate on its arms because it puts none on its head.

### The fold

One helper serves all three seats:

- `armsAgree read ds alts` — every arm's `read` agrees with `ds` outright
  (`sameBindings`), arm by arm.
- `sharedCtx read alts ev = if armsAgree read (read ev) alts then read ev else bs`.

`headerCtx alts ev = sharedCtx eventAfter alts ev`;
`delayedCtx alts ev = settleTargets (sharedCtx eventAfter alts ev)`;
`interceptCtx alts ev = sharedCtx eventIntro alts ev`. The reader is the seat's
own — `eventAfter` where the event happened [CR#603.6], `eventIntro` where a
replacement keeps it from happening [CR#614.6]. Whole agreement, not a meet:
the empty list collapses to the lone-event reading, and one disagreeing arm
hands the tail the outer discourse bare.

### Migration

Every binary use site kept working by writing its one arm as a one-element
list; no card lost its witness.

- `Macros.triggeredOr` now takes `(alts : List (GameEvent bs))`, so one macro
  writes the two-armed header and the three-armed one. Its five callers —
  Lesser Gargadon, Colossal Grave-Reaver, Chub Toad, Inferno Elemental, Urborg
  Scavengers — bracket their arm.
- `triggered`/`triggeredIf`/`triggeredOnlyDuring`/`triggeredOnlyOnce`,
  `delayed`/`delayedWithin`, `ifWouldInstead`/`nextTimeWouldInstead` pass `[]`.
- 19 direct `Intercepts` sites in the bench, 9 direct `Delayed` sites and 22
  `Triggered` sites across `Proofs*` gained `[]`; the one alt-carrying pin,
  `badAltHeaderMixedReadback`, gained its list.

### The family, re-measured

Over the supported corpus (`supported: true`, 32,568 cards), genuine event
disjunctions with three or more arms — a coordination of EVENTS, not one event
with a disjunctive noun — are **six lines at three seats**:

| seat | line | arms |
|---|---|---|
| header | Giggling Skitterspike | attacks / blocks / becomes the target of a spell |
| header | Syr Konrad, the Grim | dies / put into a graveyard / leaves your graveyard |
| header | Trouble in Pairs | attacks you with two or more / draws their second card / casts their second spell |
| header | Avatar Aang | waterbend / earthbend / firebend / airbend |
| delayed | Repeated Reverberation | cast an instant / cast a sorcery / activate a loyalty ability |
| replacement `would` | Illusionary Mask | assign or deal damage / be dealt damage / become tapped |

Avatar Aang is new since verdict 2's count; the rest are its five. The 48 other
supported "…, …, or …," headers the regex finds are all one event under a
disjunctive noun ("an instant, sorcery, or Wizard spell").

### Carrier status

**Giggling Skitterspike — header writes, tail does not.**
`gigglingSkitterspikeArms` benches the coordination: `Attacks` beside `Blocks`
and `BecomesTarget`, each arm passing `HeaderNontarget`. Its body "it deals
damage equal to its power to each opponent" does not write, and the blocker is
exact: `BecomesTarget` announces its targeter beside the self [the row's own
decision], so whole agreement fails and the tail is handed the outer discourse
bare, where `It`'s `countOnes Object bs = 1` finds nothing. It would find
nothing under a meet either — with both announcements standing the count is 2,
which is the demonstrative-uniqueness rule the `badIt` family names. Pinned as
`badThreeArmHeaderReadback`. The card's blocker, not the seat's.

**Syr Konrad, the Grim — blocked at one arm.** Arms 1 and 3 write (`Dies`, and
`Macros.leavesZone` over a creature card in your graveyard — the arm the seat
waited on), and its tail reads nothing back ("Syr Konrad deals 1 damage to each
opponent" is `DealDamage This …`), so the whole card would bench. Arm 2 does
not: "put into a graveyard **from anywhere other than the battlefield**" needs
an `EventSource` that names every zone but one, and `EventSource` has only
`FromAnywhere` and `FromZone`. Ledger.

**Trouble in Pairs — blocked at two arms.** Arm 1 "an opponent attacks you with
two or more creatures" needs a defender on `AttacksWith`, which announces its
attackers where [CR#508.1b]'s per-creature defender would go and names no
defender at all. Arms 2 and 3 read "their" back to arm 1's subject, which no
arm can do: each arm is a `GameEvent bs` over the outer context, and a
coordination fires on one arm. "Each turn" on the ordinal is a third gap.
Ledger.

**Avatar Aang — blocked at the verb vocabulary.** None of waterbend, earthbend,
firebend or airbend is in `verbFacts`, and the following sentence's "if you've
done all four this turn" counts occurrences across the arms, which no lookback
spells. Ledger.

**Repeated Reverberation (delayed) — arms write, tail does not.** The three
arms are `Casts` an instant, `Casts` a sorcery and `Activates` a loyalty
ability. Their after-discourses disagree — `ObjectP (Just Instant)` against
`ObjectP (Just Sorcery)` against an `Ability` mention — so `delayedCtx` hands
the body `settleTargets bs`, and the body's "copy that spell or ability twice"
is exactly the coordinated anaphor that read names nothing determinate. The
delayed seat therefore carries the slot with no whole-card witness. Ledger.

**Illusionary Mask (replacement) — two of three arms, no subject.** "be dealt
damage" is `IsDealtDamage` and "become tapped" is `StatusEvent … Tapped`, but
"assign or deal damage" has no `GameEvent` row at all: `DamageDealing` is an
`EventName` with no prospective row, as `interceptOk`'s own comment records.
The subject "the creature that spell becomes as it resolves" is unspellable
besides, and the replacement's body is itself a three-way coordination of
effects. No fragment benches that would not misreport a three-armed line as
two-armed. Ledger.

### Landed

- `Experimental/Triggers.idr` — `AltEvent` over a list; `armsAgree`,
  `sharedCtx`, `interceptCtx`, `interceptArmsOk`, `InterceptableArms`;
  `headerCtx`, `delayedCtx` and `chapterDefaultsOk`/`ChapterDefaults` over the
  list. The `GameEvent` head docstring's "scheduled widening" paragraph is now
  the landed shape.
- `Experimental/Effect.idr` — the slot at `Triggered`, `Delayed` and
  `Intercepts`, and their nine tables' arities.
- `Experimental/Macros.idr` — `triggeredOr` over a list; the seven other
  trigger/delay/replacement macros pass `[]`.
- `Experimental/Cards.idr` — `gigglingSkitterspikeArms`.
- `Experimental/Proofs.idr` — `badThreeArmHeaderReadback`.

### Ledger

- **An `EventSource` that names every zone but one** — "from anywhere other
  than the battlefield" (5 cards), "…than a graveyard or exile", "…than exile":
  9 sites over `PutInto`/`Leaves`. The excluded thing is a SET of zones in at
  least one printing, which `ZoneExpr` does not hold, so the shape is a real
  decision and not a third `EventSource` arm by inspection. Unblocks Syr
  Konrad, the header seat's only whole-card three-armed carrier.
- **A defender on `AttacksWith`** — "an opponent attacks you with two or more
  creatures". Trouble in Pairs.
- **A coordination whose arms share a subject** — "an opponent attacks you …,
  draws THEIR second card …, or casts THEIR second spell". Every arm is a
  `GameEvent bs` over the outer context today, so no arm can read another's
  announcement. Trouble in Pairs.
- **An ordinal reset window** — "their second card each turn". Trouble in Pairs.
- **The bending verbs, and a count over a coordination's occurrences** —
  "if you've done all four this turn". Avatar Aang.
- **A coordinated anaphor** — "copy that spell or ability twice" reads a
  mention two arms of different kinds could only jointly make. Repeated
  Reverberation, and the reason the delayed seat has no witness.
- **A prospective row for the damage DEALER** — `DamageDealing` names the event
  and no `GameEvent` row writes it. Illusionary Mask's first arm.
- **The face-down creature a spell became** — "the creature that spell becomes
  as it resolves has not been turned face up". Illusionary Mask's subject.

### Gates

- `idris/scripts/build` — 23/23, 0 errors, 0 warnings (clean `build/`).
- `cargo xtask cite check --list-noncompliant` — 0 non-compliant.
- `cargo xtask cite check` — 17,640 citations, 0 stale; no rule needed blessing.
- `jj diff --git | cargo xtask cite audit --diff` — 4 sites, each read against
  its rule.
