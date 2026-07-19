---
needs: []
---
Add **Spinal Parasite** as a canon test card and cover the negative-base-P/T
mechanics end-to-end. The signed `StatValue::Number(Int)` model and the
literal reader's negative-numeral splice landed already (a printed -1/-1 now
parses through `generate` and the card readers); this ticket exercises the
*behavior* those enable.

Spinal Parasite: `{5}` Artifact Creature — Insect, base **-1/-1**, with
**Sunburst** (enters with a +1/+1 counter for each color of mana spent to cast
it, [CR#702.44]) and "**Remove two +1/+1 counters from this creature:** Remove
a counter from target permanent." It is an unusually good single-card probe:
the base P/T is negative, Sunburst scales the counters by colors spent, and the
activated ability's cost consumes those same counters — so counter count,
layer-7 P/T, the toughness-≤-0 SBA ([CR#704.5f]), and cost payability all
interlock.

## Target scenarios (the behavioral assertions)

1. **Cast for 4 colors → dies after two activations.** 4 `+1/+1` counters →
   net 3/3. Activating the ability twice removes 4 counters → 0 counters →
   base -1/-1 → toughness < 1 → creature dies (SBA [CR#704.5f]).
2. **Cast for 5 colors → dies after two activations.** 5 counters → net 4/4.
   Two activations remove 4 → 1 counter → -1/-1 + 1/1 = **0/0** → toughness 0
   → dies (SBA [CR#704.5f]).
3. **Cast for 5 colors with an anthem (+1/+1) → survives, then uncastable
   cost.** 5 counters; a static +1/+1 anthem in play. Two activations → 1
   counter → base -1/-1 + counter +1/+1 + anthem +1/+1 = **1/1** → survives.
   A third activation is impossible: only 1 counter remains and "remove two
   +1/+1 counters" is no longer a payable cost.

## Depends on (verify support before claiming)

- **Sunburst** (census row: 38 cards) — enters-with-a-counter-per-color-of-mana-spent;
  needs cast-time colors-spent memory. Not yet implemented.
- **Remove-N-counters activated cost** — a counter-removal cost component and
  its payability check (scenario 3 hinges on the cost becoming unpayable).
- **Toughness-≤-0 SBA** ([CR#704.5f]) — present, re-verify it fires on the
  negative-base path.
- **Static +1/+1 anthem** for scenario 3.

## Known model gap to resolve here (not before)

The Rust side now carries the signed base value: `StatValue::Number(Int)` and
`NumericOp::Set(StatValue)`, with `StatValue::Count(Count)` for a dynamic base.
But the Idris soundness twin still has `CharValue Power = Count` and Idris
`Count` is itself **non-negative**, so a printed/base -1 is not representable in
the twin yet — the model is subtly wrong exactly where Spinal Parasite lives.
Resolving that (a signed base-value type in `Core.idr`, or modelling printed
negative base P/T distinctly) is part of THIS ticket — it will surface the
moment the card is emitted through `idris-check`.

Follow the card-surface test conventions (assert by card identity across zone
changes, drive resolution through the runner). Behavioral equivalence to the CR
is the bar, not vocabulary.
