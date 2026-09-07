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

## Also decide

Whether `ColorTerm::Lit` keeps its mark. It was marked because the second
landing's ruling names it, but NO canon card reaches it: `ColorTerm` is
reached only from `ManaTypeTerm::OfColor` and `Devotion`, which no canon card
writes. The second landing flagged it for veto and nothing has claimed it
since.
