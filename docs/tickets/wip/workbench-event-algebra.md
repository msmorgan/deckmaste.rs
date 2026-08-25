---
needs: []
---
# Decide the event algebra: composition operators, the cause channel, and the two grades of event reader

The trigger header's own holes are
[workbench-trigger-and-interception-residues](workbench-trigger-and-interception-residues.md)'s.
This ticket is the shape one level down: what an event term may be *composed
of*, and what a clause other than a trigger header may say about an event.

## Context

Source: [the v1/v2 comparison](../../memory/scratch/experimental-vs-semantics-comparison.md),
axis 13 and summary finding 2 (2026-08-24), which calls this "the largest
unrecorded shape gap on the list". `semantics-v2.md:§8`'s "reference machinery
before vocabulary breadth" covers more event *names*; nothing records a decision
about the *shape* of event composition, which is why this is a ticket rather
than an entry under a landed round.

## From the v1 comparison (2026-08-24)

> **Crate.** `EventFilter` (`event.rs:283`) is ~32 master forms *plus a
> composable algebra*: `AllOf` (616), `OneOf` (620), `Not` (625), `OneOrMore`
> (629), `Nth { n, of, within }` (633), `When(EventFilter, Condition)` (641),
> `Within(EventFilter, Lookback)` (645), `Before(Reference)` (655). Cause is its
> own narrowing channel: `Cause::Cause(CausePattern { verb, agency, agent })`
> (`event.rs:251,233`) with `Agency` enumerating `CostPayment |
> AttackDeclaration | EffectInstruction | TurnBasedAction | StateBasedAction |
> ManaAbilityResolution | SpecialAction` (`event.rs:133`). One type serves
> triggers (`TriggeredAbility.event`, `ability.rs:91`), conditions
> (`Condition::Happened { event, within }`, `condition.rs:52`), counts
> (`Count::EventCount`/`EventSum`, `count.rs:167`), durations
> (`Duration::UntilEvent`, `continuous.rs:31`), replacements
> (`Replacement::Instead { would, .. }`, `replacement.rs:20`) and
> `StaticEffect::CantHappen` (`continuous.rs:282`).
>
> **What is lost.** The algebra, entirely. There is no `Not`, no `OneOf`, no
> "the second time this turn" (`Nth`), no `When(event, condition)`, no
> `Before(Reference)`. And there is no cause/agency channel:
> `verbMoves`/`verbAgentive` (`Words.idr:~690-712`) are per-verb tables, not a
> `CausePattern` that can narrow "destroyed" from "sacrificed" inside an event
> query.

The report grants that the workbench's `GameEvent` (23 constructors,
`Experimental.idr:2178`) as an English clause is on-contract and nicer than
`ZoneChange { what, from, to, cause }`, and that `eventName`'s projection onto
the 27-member `EventName` (`Events.idr:11`) is "one vocabulary with a
classifier, not two". The composition question is orthogonal to both.

## Two corrections to the report, verified in the workbench

- **A disjunction exists at the header.** `AltEvent`
  (`Experimental.idr:4698-4701`, consumed at 4764) coordinates two events under
  one trigger word, gated by `HeaderNontarget`. So the flat claim "no `OneOf`"
  is wrong at that one site; what is missing is a disjunction that composes
  anywhere else, and the corresponding negation, ordinal and gated forms
  nowhere at all.
- **There are two grades of event reader, and that is the sharper gap.** The
  trigger header takes a full `GameEvent`; every other reader takes only the
  coarse classifier — `HappenedTo : (ev : EventName) -> (w : Lookback) -> …`
  (`Experimental.idr:172`) and `EventCount : (ev : EventName) -> (who : Noun bs
  k) -> …` (`Experimental.idr:1473`). Where the crate asks one type six
  questions, v2 asks a detailed one at the header and a name-only one
  everywhere else. Any algebra decision has to say which grade it operates on,
  or it lands at the header and leaves conditions, counts and durations behind.

## What this round decides

1. **Whether composition is a construction or a spelling.** If `Not`/`OneOf`/
   `Nth`/`When`/`Within`/`Before` become `GameEvent` constructors, each one
   needs its `bs` threading answered — what a disjunct announces when the two
   arms announce different things is the same question
   [workbench-trigger-and-interception-residues](workbench-trigger-and-interception-residues.md)
   already carries as "the coordinated header's readback — 144 of 364", and the
   answer must be one answer, not two.
2. **Whether the two reader grades collapse.** Either the condition/count
   readers are widened to a full event term, or the report's finding is recorded
   as deliberate with the reason.
3. **The cause channel.** Whether narrowing an event by its cause ("destroyed"
   vs "sacrificed", a cost payment vs an effect instruction) is a slot on the
   event or stays distributed across the per-verb tables `verbMoves` and
   `verbAgentive`. Take the `Agency` enumeration as the shape to argue against,
   not as the shape to import.

Measure each operator's corpus surface before minting it. An operator with no
attested line gets a pin, not a constructor — the workbench's own standard.

## Consumption boundary

The Idris workbench only: `idris/src/Experimental.idr` (`GameEvent`,
`eventName`, `AltEvent` and the header's coordination, `HappenedTo`,
`EventCount`, the interception carriers, the duration rows),
`idris/src/Experimental/Events.idr` (`EventName`, `interceptOk`, `spanEventOk`,
`LookbackSubject`, `LookbackComplement`, `eventUse`),
`idris/src/Experimental/Words.idr` (`VerbName`, `verbMoves`, `verbAgentive` if
the cause channel lands), the pin modules `idris/src/Experimental/Proofs*.idr`,
evidence bench `idris/src/Experimental/Cards.idr`. No Rust crate.

## Acceptance

- The composition decision is recorded where `GameEvent` is defined, whichever
  way it goes; "reference machinery before vocabulary breadth" is not reused as
  cover for a shape question it does not reach.
- Each operator that lands names the corpus lines it buys; each that does not
  land is a pin with its measured zero.
- The reader-grade question is answered explicitly: either one event term serves
  the header, the condition, the count and the duration, or the split is
  recorded with its reason.
- The disjunction's announcement rule is the same rule the coordinated header's
  readback gets — one decidable comparison on `Bindings`, not two.
- The cause channel is either a slot with a stated `bs` contribution, or the
  per-verb tables are recorded as the deliberate answer with the narrowing they
  cannot express named.
- `AltEvent` is not duplicated by a general disjunction; if the general form
  lands, the header's row is retired into it.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.
