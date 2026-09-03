---
needs: []
---
**Replace the per-constructor total tables with one profile record, one
generic optional-witness wrapper, wildcard fallbacks and index-based `Eq`.**
Cleanroom review 2026-09-03, F6 + F8 + F9 + F19 — the mechanical half of "what
is out of hand". Four passes; land them in this order.

**F6 — one `effProfile` for the nine `Effect` tables.** `Effect` (87
constructors) is projected by nine ~90-clause functions, ~800 lines:
`heldUntilOk` (`Effect.idr:1319–1403`), `reflexEncloseUse` (`:1417–1506`),
`thisWayOutcomeOk` (`:1520–1602`), `costActionOk` (`:1625–1711`), `effEq`
(`:1746–1857`), `effIntro` (`:1887–2001`), `preIntro` (`:2013–2101`),
`annIntro` (`:2135–2226`), `deedDelta` (`:2260–2357`). `preIntro`/`annIntro`
differ in five clauses (`ExtraTurn`, `SeparateIntoPiles`, `Sequentially`,
`Simultaneously`, `Does`); `replacedCtx e = deedDelta e ++ annIntro e`
(`:2248`) is the identity `effIntro` obeys wherever fold-state does not move;
`thisWayOutcomeOk` is `True` for 84 constructors written out and `heldUntilOk`
`False` for 86. Fix: one `effProfile : Effect bs -> EffProfile` record
(`pre`, `deed`, `moved`, `scheduled : Bool`, `agentive : Bool`) with the
predicates derived from it. `effEq` is deleted by
`workbench-modal-mode-gate`; the four `does*Intro` helper tables (`:2004,
2104, 2128, 2229`) go with the `Does`/`Enact` fold in
`workbench-axis-pairs` — do not re-home either here.

**F8 — wildcard fallbacks where a wildcard is total.** `Words.wordReaches`
(`Words.idr:1844–1974`, 130 clauses; `halfReaches` two lines above already
does `_ _ = False`), `attachHeadOk` (`:3175–3232`), `Eq Kind` (`:222–304`),
`verbedWordOk` (`:2020`), `attachHostZone`/`attachHostTy` (`:3245–3276`),
`Phrase.predSays` (`Phrase.idr:1224–1290`), `predNegFree` (`:1302–1368`),
`hasHead` (`:561–627`), `seedZone` (`:419–455`), `seedType` (`:504–534`),
`condNegated` (`:2538–2556`), plus F6's `costActionOk`/`heldUntilOk`/
`thisWayOutcomeOk`. `%default total` accepts a final wildcard; the cost is
only where it hides a case that should have been decided, so name any clause
kept explicit and why. Delete outright, no callers: `kindLteTrans`
(`Words.idr:423–523`) and with it `sameLetterEq`'s only use;
`halfDesignationInjective`/`designationHalfInverse` (`:3040–3051`).

**F9 — one generic `OptOk`, and the dead indices.** Sixteen `data F : Maybe X
-> Type` families with `FNone`/`FSome {auto 0 ok}`: `Triggers.SpanOk`
(`Triggers.idr:620`), `DelaySpanOk` (`:625`), `AddedPartWritten` (`:582`),
`TurnDeixis` (`:588`), `AltEvent` (`:654`), `Phrase.ComplementWritten`
(`Phrase.idr:185`), `ChoiceClause` (`:2285`), `EventAgent` (`:2633`),
`TokenPhrase` (`:2639`), `CostSubject` (`:2385`), `LinkSource` (`:2252`),
`DiscardOk` (`:2841`), `Effect.AltPayment` (`Effect.idr:2900`),
`CtrlOverrideOk` (`:842`), `Words.CounterKindNamed` (`Words.idr:3451`),
`DesignationHolder` (`:3156`). Fix: one `data OptOk : (a -> Type) -> Maybe a
-> Type where Absent : OptOk p Nothing; Present : {auto 0 ok : p x} -> OptOk p
(Just x)` — kept as `data`, per
`docs/memory/architecture/so-synonym-loses-index-inference.md`, so call-site
index inference survives; a wrapper whose index is load-bearing stays. Dead
indices and vacuous gates to delete: `SpanOk`'s `StaticKind` is never
consulted (so `SpanOk` and `DelaySpanOk` are one type); `PartTriggerable p h`
(`Triggers.idr:574`) ignores `h`; `altRunWritten` (`Words.idr:2775–2777`) is
`True` on both branches, so `AltRunWritten` on `ProducedMana.OfChosenColor`
(`Effect.idr:948`) gates nothing. `Copiable` and `Counterable`
(`Phrase.idr:2426`/`:2398`) are the same family character for character —
auditor C replaced both with one `StackActOn (Kind -> Bool) n`, every refusal
kept, bench passes, 15.7 s. Lift the agreement pairs with `All`/`OptOk`:
`KnownAct`/`KnownActs`, `CounterSourceScope`/`OptCounterSourceScope`,
`WellFormedQ`/`OptWellFormedQ`, `SingleRecipient` (`Phrase.idr:2837`, =
`nounPlur n = OneOf`); the six token `So` aliases and six `Macros.create`
slots become one `TokenWellFormed`; `Macros.card` (`Macros.idr:216–222`)
re-lists the seven laws `FaceLaws` (`Card.idr:319`) bundles — keep `FaceLaws`
alone.

**F19 — hand-written `Eq`.** 48 instances in `Words.idr` (≈350 lines), all
`(==) A A = True; (==) A _ = False`. `cardTypeIx`/`qualityIx` are the index
idiom; use it, `Eq Kind` and `kindLte` included.

Size: M.

Done when: the build is 23/23 with 0 errors and 0 warnings; the nine `Effect`
tables are one `effProfile` plus derived predicates; the sixteen families are
`OptOk` (or a documented load-bearing exception),
`Copiable`/`Counterable` are one `StackActOn`, and the three dead indices and
`AltRunWritten` are gone; `kindLteTrans`, `halfDesignationInjective` and
`designationHalfInverse` are deleted; the listed tables end in a wildcard with
every retained explicit clause justified in the landing record; the `Eq`
instances read through an index; no pin's refusal changes reason and no
witness is deleted. Standard constraints apply.
