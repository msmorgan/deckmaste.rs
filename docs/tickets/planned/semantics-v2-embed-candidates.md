---
needs: [plugins-v2-dialect]
---
**Which further constructors are injections.** Design-bearing; standard
constraints apply.

`semantics-v2.md` §11.1's injection chain is hand-chosen and short:
`ManaSymbol::Simple`, `SimpleManaSymbol::Specific`, `ColorOrColorless::Of`,
`ColorTerm::Lit` — v1's chain mapped onto v2. The rule is one embed per enum
and never an enumeration from the mirror, so every further site is a ruling.
`plugins-v2-dialect`'s second landing deferred them until the conversion
showed which positions suffer; this ticket carries the marking.

Decide each of the positions below, then mark the ones that carry, and
convert the cards that wanted them.

## The candidates, by how often `plugins_v2/canon` writes the constructor out

Counted on the canon corpus at the second landing (change `zkootrrlnrox`),
one-field constructors only, and unchanged by the third:

| Constructor | Written | The card that wants it |
| --- | --- | --- |
| `Card::SingleFaced { face }` | 117 | every single-faced card — `Grizzly Bears` writes `SingleFaced(face: (…))` around its whole body |
| `Cost::Mana { cost }` | 34 | every card with an activated ability — `Abbey Gargoyles`' cost line |
| `Predicate::HasType { type }` | 26 | every "destroy target creature" — a bare `Creature` at a predicate position |
| `NounPhrase::Target { quantity }` | 23 | as above, the determiner half |
| `Predicate::And { … }` | 23 | every multi-modifier description |
| `Subtype::Type { … }` | 18 | every typed token |
| `Delta::Up { amount }` | 15 | every "+1/+1" and every life gain |
| `StaticSpec::Static { spec }` | 14 | every static ability |
| `Predicate::InZone { … }` | 13 | every zone conjunct |
| `Instruction::Sequentially { … }` | 11 | every multi-sentence effect |

Two of these cannot be injections as the mechanism stands, and that is the
first thing to decide: `SingleFaced`'s payload is a STRUCT (`CardFace`) and
`Cost::Mana`'s is a `Vec`, while `#[macro_ron(embed)]` needs `SupportsMacros`
on the payload, which is an enum-only derive. Either the derive grows those
two payload shapes or those two positions keep their constructor.

The rest are semantic claims rather than pure injections, and the one-embed
-per-enum rule makes each an exclusive choice: marking `Predicate::HasType`
makes a bare `Creature` a predicate and spends `Predicate`'s only embed slot,
which `And`, `InZone` and a dozen other one-field constructors also want.
Rank them before marking.

## What the fourth landing's conversion showed

The conversion landed (`plugins-v2-dialect`, fourth landing) as a macro
substitution, not a reserialise: every constructor a card wrote became the
macro of that name, and nothing was elided. So the census above still stands
unchanged as the evidence — no position was measured twice — and the
conversion adds one observation rather than a new table:

- A card now writes `mana(cost: […])`, `singleFaced(face: (…))`,
  `hasType(type: Creature)` and `target(quantity: …)` as MACROS. An injection
  would elide the same constructors a second way, so each row above is now a
  choice between the alias macro that already covers it and an embed that
  removes the write entirely. Decide the pair, not the constructor alone: an
  embed at a position whose alias macro is already the spelling a card writes
  buys nothing but a second spelling.
- The one-embed-per-enum ranking is unchanged, and so is the pair of
  positions the mechanism cannot reach (`SingleFaced`'s struct payload and
  `Cost::Mana`'s `Vec`).

The COSMETIC half of the conversion — injections written bare, applications
positional, numerals bare — did not run; it is `plugins-v2-dialect`'s handoff,
and it is the pass that would show a position suffering. Re-read this table
against it when that pass lands.

## Also decide

Whether `ColorTerm::Lit` keeps its mark. It was marked because the second
landing's ruling names it, but NO canon card reaches it: `ColorTerm` is
reached only from `ManaTypeTerm::OfColor` and `Devotion`, which no canon card
writes. The second landing flagged it for veto and nothing has claimed it
since.

## What the writer shows (`plugins-v2-cosmetic-conversion`, 2026-09-07)

The cosmetic conversion did not run — see that ticket's STOP — but its first
step built the evidence this table was waiting for: the dialect's WRITER, run
over all 120 cards and pinned lossless by
`deckmaste_semantics_v2::tests::corpus::every_card_writes_and_reads_back_to_the_same_value`.
Two censuses, and they answer different questions. Measured on
`moxlqulsykty`.

### What a card FILE writes today

Every one-field constructor application in the 118 canon + 2 testing card
sources, by how often it is written. This is what an embed would remove from a
file, so it is the actionable half.

| Constructor | Payload | Written | Cards | Example |
| --- | --- | --- | --- | --- |
| `ManaSymbol::Simple { symbol }` | constructor | 298 | 120 | `Abbey Gargoyles` |
| `Amount::Lit { value }` | numeral | 237 | 106 | `Abbey Gargoyles` |
| `ColorOrColorless::Of { color }` | constructor | 170 | 113 | `Abbey Gargoyles` |
| `SimpleManaSymbol::Specific { color }` | constructor | 170 | 113 | `Abbey Gargoyles` |
| `SimpleManaSymbol::Generic { amount }` | numeral | 128 | 100 | `Abbey Gargoyles` |
| `Card::SingleFaced { face }` | struct | 119 | 119 | `Abbey Gargoyles` |
| `NounPhrase::Target { quantity }` | constructor | 23 | 19 | `Burst of Energy` |
| `Subtype::Type { type }` | constructor | 18 | 8 | `Arcanum Wings` |
| `PossessedBy { possessor }` | constructor | 9 | 9 | `Dangerous Wager` |
| `A { mode }` | constructor | 8 | 7 | `Cirdan the Shipwright` |
| `Core { deed }` | constructor | 5 | 5 | `Capsize` |
| `Action { label }` | string | 4 | 4 | `Glimpse of Freedom` |
| `Printed { name }` | string | 3 | 1 | `Graf Rats` |
| `ByPlayer { player }` | constructor | 2 | 2 | `Graf Rats` |
| six more at 1 each | — | 6 | — | `ByCandidate`, `ByKeyword`, `DefendingPlayer`, `OneZone`, `Printed(kind:)`, `Zones` |

1,200 one-field constructor applications across 20 distinct constructors.
**1,003 of them — 84% — are the five positions ALREADY marked**: the mana
chain and the two numeral leaves. The writer elides every one, so the cosmetic
conversion alone removes five sixths of the written-out one-field constructors
in the corpus without a single new ruling. That is the size of the prize the
marking decision is being measured against, and it argues for running the
conversion before marking anything further.

Of the remaining 197, `SingleFaced` is 119 and cannot be an injection as the
mechanism stands (struct payload), which leaves 78 writes across 14
constructors as the whole realistic gain from every further marking combined.
`Target(quantity:)` at 23 is the largest, and the earlier table's rank order
(`HasType`, `And`, `Mana`, `Static`, `InZone`, `Sequentially`) does NOT survive
into the source census, because the fourth landing's substitution turned each
of those into a MACRO invocation: `hasType(type: Creature)` 26,
`mana(cost: […])` 34, `and(conjuncts: […])` 23, `static(spec: …)` 14,
`inZone(zone: …)` 13, `up(amount: …)` 15, `sequentially(steps: […])` 11 — 176
named one-field macro invocations in total, across 24 macros. So each of those
rows is now the choice §11.1 already framed: an embed there buys a second
spelling for something the alias macro already spells. **The cheaper win at
those positions is positional application of the MACRO — `hasType(Creature)`,
`mana([2, White])` — which the reader already accepts and which needs no
marking at all.**

### What the WRITER writes

The same scan over the writer's output, which is the expanded constructor
basis rather than a card file: 1,336 one-field applications across 89 distinct
constructors. The rank order there is `SingleFaced` 119, `Action(label:)` 89,
`HasType` 88, `And` 86, `Up` 74, `Static` 66, `Mana` 65, `A(mode:)` 40,
`Core(deed:)` 36, `Sequentially` 36, `Target` 35, `InZone` 33 — which is the
census the earlier table reports, and it is NOT evidence about card files. It
is evidence about what a Lean-facing dump costs, and none of these positions
suffers in a card while the macro layer stands between the card and the basis.
Read the first table for the marking decision and this one only for the
constructor-basis view.

### Still to decide

Everything in "Also decide" above, unchanged: `ColorTerm::Lit` still has no
canon card reaching it, and the source census confirms it: `Lit(color:)` is
written 0 times across the 120 card files. The writer's output cannot settle
it either way, because the mark it is being questioned for is what elides it
there.
