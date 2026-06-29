---
needs: []
---
**Core: factor `Modification`'s per-axis ops into shared op enums (keep the axis
at the variant level — a partial unification).** From the 2026-06-28 idris↔rust
model audit follow-up. Supersedes the audit's "do NOT port unified `Modification`"
note with a middle path.

Today `Modification` (`crates/deckmaste_core/src/continuous.rs`) spells the
per-axis set/pump/element ops as ~dozen-plus flat variants — `SetPower`,
`AddPower`, `SubtractPower`, `SetToughness`, `AddToughness`, `SubtractToughness`,
`SetColors`, `AddColors`, `SetCardTypes`, `AddCardTypes`, `SetSubtypes`,
`AddSubtypes`, `SetSupertypes`, `AddSupertypes`, `SetBaseDefense`, … — with
irregular coverage (e.g. no element-`Remove` on colors/subtypes).

Idris fully unifies these to `Alter (Characteristic) (ModificationOp)`, with the
op gated by `Numeric`/`Collection` type-classes (`idris/src/Core.idr`). We are
**not** adopting that full unification: in Rust the op↔axis match would lose its
compile-time gate (it's a dependent proof that erases) and become a runtime
nonsense-check, and `Alter(Power, Up(1))` reads worse than the alternative.

**This ticket's middle path:** keep the axis named at the *variant* level, but
factor the operations into two small shared op enums:

- `NumericOp = Set(Count) | Up(Count) | Down(Count)` — the numeric axes (Power,
  Toughness, Defense, base loyalty).
- `CollectionOp<T> = Set(Vec<T>) | Add(T) | Remove(T)` — the set-shaped axes
  (Colors `<Color>`, CardTypes `<Type>`, Subtypes `<Subtype>`, Supertypes
  `<Supertype>`). *(If the `SupportsMacros`/`Expand` derive doesn't play well
  with a generic enum, fall back to per-axis op enums — slightly more
  duplication, same shape.)*

Then `Modification::Power(NumericOp)`, `Toughness(NumericOp)`,
`Colors(CollectionOp<Color>)`, `Subtypes(CollectionOp<Subtype>)`, etc.

**Why this is sound without dependent types:** the op↔axis validity is enforced by
the variant↔op-enum pairing in Rust's own types — a `Colors` variant takes a
`CollectionOp`, which has no `Up`, so "raise a color" is unrepresentable at the
type level with no runtime check. That is exactly the soundness the full `Alter`
buys from its `Numeric`/`Collection` gates, recovered structurally.

**Scope — leave these as their own variants** (not axis-op shaped):
`SwitchPowerToughness`, `AllCreatureTypes`, `BecomeBasicLandType`,
`GainAbility`/`LoseAbility`/`LoseAllAbilities`/`CantHaveAbility`, `SetController`,
`SetText`, `Several`. The factoring only touches the per-axis
Set/Up/Down/Add/Remove cluster.

Verdict: **neutral → mild improvement** — cuts the per-axis variant count,
regularizes the op vocabulary (and fills the `Down`/`Remove` symmetry gaps for
free), preserves soundness structurally, and keeps RON readable
(`Power(Up(1))` / `Colors(Add(Blue))`). Effort: **M** — the enum refactor + RON
spellings + parser + every pump/anthem card (the migration churn is the main
cost). Related: the *read* side already unified via
`idris-characteristic-read-unification` (done/); this is the deliberately-partial
write side.
