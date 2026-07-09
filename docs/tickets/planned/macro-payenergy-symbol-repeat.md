---
needs: []
---
Add a `PayEnergy` cost macro built on a NEW bidirectional template construct —
count-driven literal repetition — so the many `{E}` energy cards graduate off the
macro (the engine resource model landed in `engine-energy`; only the render/parse
surface is missing). Design settled 2026-07-09.

## The new template construct: `${slot*\{LIT\}}`
Repeat an escaped literal `slot`-many times, bidirectionally, in a macro
`template`:
- **Render:** emit the literal (e.g. `{E}`) `slot` times — `PayEnergy(3)` →
  `{E}{E}{E}`; `n=1` → `{E}`; `n=0` → empty.
- **Parse (graduation):** match a run of the literal and fold it back into
  `slot` as a plain number.
Follow the EXISTING escaping convention (backslash-escaped braces, like the
`#…#`/escaping style already in the template grammar — NOT a new quoting rule;
consistent with the Hexproof/Ward optional-param spelling). The construct is
general (any escaped literal after `*`), a sibling to `template-verb-conjugation`.
Wire it through both the render template and the reverse `TemplateIndex` / slot
matcher (the parse side) that the macro pipeline already uses.

## The macro
`plugins/builtin/macros/cost/PayEnergy.ron`:
```
(
    name: "PayEnergy",
    template: "${0*\{E\}}",
    kinds: [CostComponent],
    params: [Uint],                 // a PLAIN literal number, not the Count enum
    body: Do(RemoveCounters(who: You, kind: Energy, count: Literal(${0}))),
)
```
Pin the exact plain-number param type against the grammar (`Uint` vs `Number`);
the body wraps it as `Literal(${0})` since `RemoveCounters.count` is a `Count`.
Energy costs are always a fixed literal count of `{E}` (never "for each …"), so a
plain number is correct — not `Count`.

## Render
Add the `{E}` symbol render arm so the repetition emits the actual glyph (today
`PutCounters`/`RemoveCounters` on `Energy` render generically, e.g. "Remove 2
Energy counters from you", which fails the strict per-line fidelity gate). The
`{E}` symbol already exists (`crates/deckmaste_core/src/symbol.rs`); it needs a
render arm.

## Payoff & scope
Once the construct + macro + `{E}` render land, the graduation pipeline parses
`"Pay {E}{E}"` → `PayEnergy(2)` and the energy "Pay {E}…" cards graduate;
`Aether Hub` (idris demo already present) is the natural first canon graduation.
Report how many graduate with `PayEnergy` alone vs how many also need the
**gain** side ("you get {E}{E}" = `Do(PutCounters(You, Energy, N))`) — a sibling
macro (`GainEnergy`?) riding the SAME `${0*\{E\}}` construct; add it if graduation
needs it (it's trivial once the construct exists). Some energy cards will still
need unrelated mechanics (ETB triggers, "add one mana of any color", search) —
those stay out of scope. Surfaced by `engine-energy` (engine model done) and the
in-conversation design of the `PayEnergy` macro.
