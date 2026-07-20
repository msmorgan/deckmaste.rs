---
needs: []
---
**[design] Idris grammar: open the remaining closed Idris sets that are open in RON —
`CounterKind` and `Designation` — the way `idris-subtype-open-names` opened `Subtype`.** Each is a
closed Idris enum shadowing an OPEN, name-keyed RON registry, mirrored by a hand-maintained
`*_idris` match in `idris_emit`; a name absent from the closed enum/match gaps at the
`cargo xtask idris-check` gate. Same parity burden `Subtype` had, same fix — with one wrinkle
subtypes did NOT have (below).

**The closed sets and their open registries.**

| Idris (closed) | idris_emit mirror | RON registry (open) | value data |
|---|---|---|---|
| `CounterKind` (`Core.idr` ~264: `Loyalty \| Fate \| Charge \| P1P1 \| M1M1 \| Level \| Lore \| Stun \| Shield \| …`) | `counterkind_idris` | `plugin.counters: HashMap<Ident, Counter>` | `Counter { name, scope: CounterScope, confers: Vec<Property> }` |
| `Designation` (`Core.idr` ~400: `Monarch \| TheInitiative \| CitysBlessing \| …`) | `designation_idris` | `plugin.designations: HashMap<Ident, DesignationDecl>` | `DesignationDecl { name, definition }` (confers inside the definition) |

Both carry the same two facets `Subtype` did: a **scope/category** (`counterScope`/`designationScope`
: `… -> RefKind`, object-borne vs player-borne — the counter/designation analogue of a subtype's
`Category`) and **conferrals** (`counterConfers`/`designationConfers` : `… -> List (Ability b)`,
hand-mirrors of the RON `confers`). Follow `idris-subtype-open-names`: replace the closed enum with
an open name-carrying value, put the conferrals ON the value sourced from the RON (never a hardcoded
Idris match — see [Conferrals come from registries](../../decisions/conferrals-come-from-registries.md)), derive scope/category from the RON in
`idris_emit`, and delete the `*_idris` mirrors.

**THE WRINKLE (why this wasn't done with subtypes): scope lives in a DEPENDENT TYPE INDEX.** Unlike
`subtypeCategory` (value-level, used only in the `SubtypesOk` proof), the counter/designation scope
appears in the *type* of their reference-taking constructors:

```idris
CountersOn     : (c : CounterKind) -> Reference b (counterScope c)    -> Count b
HasCounter     : (c : CounterKind) -> Predicate b (counterScope c)
HasDesignation : (d : Designation) -> Predicate b (designationScope d)
GrantDesignation/… : … (designationScope d) …
```

`counterScope c` / `designationScope d` are type-level functions that REDUCE on the closed
constructor to pick `AnObject` vs `APlayer`. If the kind becomes an open `String`, `counterScope
"Foo"` can't reduce to a concrete `RefKind`, so these constructors lose their type. The design must
resolve this — candidate directions to weigh (pick during the design dialogue):
- **Carry the scope in the value** (`MkCounterKind Scope String (List (Ability Base))`, `Scope` a
  small closed enum like `Subtype`'s `Category`) and make the reference kind depend on the value's
  scope field — but a value's field isn't available at the type level the way a constructor is, so
  `CountersOn`/`HasCounter` likely must change shape (e.g. take the scope explicitly, or unify to a
  single `Reference`/`Predicate` kind and check scope at the value level like `SubtypesOk` does).
- **De-index**: drop `counterScope`/`designationScope` from the type and gate scope with a
  value-level well-formedness proof (the `SubtypesOk` pattern), so `CountersOn`/`HasCounter`/
  `HasDesignation` no longer dependently branch on the kind.

This is the whole design problem; the rest is mechanical (mirrors `idris-subtype-open-names`).

**Scope note.** `KeywordSpec` (the fourth closed set, `keywordspec_idris` mirror, `plugin.keywords`)
is deliberately OUT of scope: keywords carry engine semantics (intrinsics like Flying/Trample are
engine-modeled, not mere identity+confers), so opening them is a different problem, not this pattern.

**Verify.** `cargo xtask idris-check` green with a counter/designation name absent from the OLD
closed enum now emitting; a counter/designation with a conferring registry row emits its confers
from the RON; `idris/` builds; workspace green; cite audit if any CR citation moves.

*Serializes with other `idris-*` grammar tickets (all rewrite `idris/src/Core.idr`). `[design]` — the
dependent-type-index change is nontrivial and touches the soundness gate; design dialogue first.
Precedent: `idris-subtype-open-names` (done).*
