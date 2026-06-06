# Cost + Action Implementation Plan (Plan 2)

**Goal:** Land Cost/CostComponent, the Action intrinsics, ManaSpec, and a Token reader, so the three builtin token files parse and validate — dissolving the blocker that shelved the tokens migration.

**Architecture:** `Action` is the intrinsic verb enum; `Effect` becomes a manual-serde enum (the Filter pattern) whose `Act(Action)` compartment reads flat, so `effect: DrawCards(1)` works today and structural forms (`Sequence`, `May`, …) join the same variant list in Plan 5. `CostComponent` is a plain derive (`Mana`/`Tap`/`Untap`/`Do(Action)`) with `MacroKind::CostComponent` providing the macro fallback that makes `SacrificeThis` a prelude cost macro. The committed token RON dialect is the contract — types bend to it, not vice versa.

**Execution cadence:** three batches, one implementer each; controller verifies directly; one focused review on Batch A's manual serde only.

---

## Batch A — core types (`deckmaste_core`)

1. **`ManaSpec`** in `mana.rs`: `enum ManaSpec { AnyColor }` (usual + serde derives; doc: produced-mana spec, CR 106; variants accrete — `AnyType`, fixed symbols, riders later). Export from lib.rs.

2. **`Action`** in new `action.rs` (alphabetical variants; doc per variant; performer is implicitly the ability's controller until a card needs otherwise):

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Action {
    AddMana(Uint, ManaSpec),
    DealDamage(Selection, Uint),
    DrawCards(Uint),
    GainLife(Uint),
    Sacrifice(Selection),
}

impl Action {
    /// Whether this verb may appear in a cost (`CostComponent::Do`): the
    /// payer performs it, nothing targets (CR 601.2b-c).
    #[must_use]
    pub fn is_cost_eligible(&self) -> bool {
        matches!(self, Action::Sacrifice(_))
    }
}
```

3. **`Effect`** moves from `ability.rs` to new `effect.rs`, manual serde per the Filter pattern: `enum Effect { Act(Action) }`, `VARIANTS` = the Action names (AddMana, DealDamage, DrawCards, GainLife, Sacrifice), Deserialize enters `deserialize_enum("Effect", VARIANTS, …)` dispatching each name into `Effect::Act(Action::…)` via `v.newtype_variant()`/tuple shapes (DealDamage and AddMana are 2-tuples: use `variant.tuple_variant(2, …)`-free approach — match Filter's style; for 2-field variants deserialize a helper tuple `let (a, b) = v.tuple_variant(...)`? Simplest correct: define per-arm seed structs OR deserialize the payload as the matching tuple type: `"DealDamage" => { let (sel, n): (Selection, Uint) = v.tuple_variant(2, TupleVisitor)?; … }` — implementer picks the cleanest compiling form, keeping the entry call and VARIANTS exact). Serialize: `Act(a) => a.serialize(serializer)` (flat). Comment block explaining: same rationale as Filter — structural variants join VARIANTS in Plan 5; `Act` never appears in RON.
   - Tests: flat parse of each verb; `to_string(Act(DrawCards(1))) == "DrawCards(1)"`; round-trip; unknown-name error; a `variants_list_matches_visit_enum` clone using the "Unexpected variant" sentinel (empirically validated in Plan 1).

4. **`CostComponent`** in new `cost.rs` (plain derives — every variant name is real):

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum CostComponent {
    /// Mana payment, e.g. `Mana([Generic(2)])`.
    Mana(ManaCost),
    /// The {T} symbol (CR 107.5).
    Tap,
    /// The {Q} symbol.
    Untap,
    /// Pay by performing a verb: only cost-eligible Actions
    /// (`Action::is_cost_eligible`) belong here — enforced by the cards
    /// crate's validation lint, not the parser.
    Do(Action),
}
```

   Tests: parse `Mana([Generic(2)])`, `Tap`, `Do(Sacrifice(That(This)))`; round-trip.

5. **`ActivatedAbility`** in `ability.rs`: `Activated` unit variant becomes `Activated(ActivatedAbility)`:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct ActivatedAbility {
    pub cost: Vec<CostComponent>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub targets: Vec<Selection>,
    pub effect: Effect,
}
```

   (Targets-and-effect flattened into the ability for now; the `Resolvable` wrapper arrives with Modal. The skip attr + default mirror CardFace — both stay load-bearing.) `use` line gains the new types; `Effect` re-exported from `effect.rs` now.

6. **`Token`** in new `token.rs` — matches the committed token files (`Token(types: [Artifact], subtypes: [Treasure], abilities: […])`; RON writes the struct name):

```rust
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Token {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supertypes: Vec<Supertype>,
    pub types: Vec<Type>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subtypes: Vec<Subtype>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub abilities: Vec<Ability>,
}
```

   (Name/colors/P-T come when a token needs them — the three predefined ones don't.)

Gate: `cargo test -p deckmaste_core` green; cards crate red is expected (Activated shape changed). Commit per module or as one batch commit — implementer's judgment, messages in repo style.

## Batch B — cards crate + prelude data

1. `MacroKind` += `CostComponent`, `Effect` (and from_position; position-names test extended with both core types).
2. New prelude macro `plugins/builtin/macros/cost/SacrificeThis.ron`:

```ron
(
    name: "SacrificeThis",
    template: "Sacrifice this permanent",
    kinds: [CostComponent],
    body: Do(Sacrifice(That(This))),
)
```

3. Builtin subtype declarations for the predefined tokens — `plugins/builtin/types/artifact/{Treasure,Clue,Food}.ron`, each `ArtifactType("…")` (CR 111.10 cites in comments: Treasure 111.10a, Food 111.10b, Clue 111.10f).
4. macros.rs tests: cost-position macro expansion (`SacrificeThis`-shaped, registered inline) and effect-position macro expansion (e.g. nullary `Investigate`-shaped macro body `DrawCards(1)` — placeholder semantics, the point is the position).
5. Existing `enum_positions_expand_unknown_variants` test body uses `value: Static` — still fine (Static is still a unit variant). Anything else referencing `Activated` as unit gets updated.

Gate: `cargo test -p deckmaste_cards --lib` green.

## Batch C — token validation + integration

1. `validate.rs`: also walk `tokens/**/*.ron` (`deckmaste_core::plugin` presumably has or gains a `TOKENS_DIR` const — follow `CARDS_DIR`'s pattern) reading each as `Token` through the plugin's macros; failures collected the same way. Plus the **cost-eligibility lint**: for every parsed Card face and Token, every `CostComponent::Do(action)` must satisfy `is_cost_eligible()` — violations are failures with a clear message.
2. Integration test (`tests/builtin.rs` or sibling): the three token files parse to expected values — Treasure: `cost: [Tap, Do(Sacrifice(That(This)))], effect: Act(AddMana(1, AnyColor))`; Clue: `[Mana([Generic(2)]), Do(…)]` / `Act(DrawCards(1))`; Food: `[Mana([Generic(2)]), Tap, Do(…)]` / `Act(GainLife(3))`.
3. `cargo test --workspace` + `cargo +nightly fmt` + clippy.

Gate: workspace green.

## Deferred / follow-ups

- **Tokens migration revival**: `jj rebase` of bookmark `tokens-shelved` (b502c56) onto the new trunk — separate step after this plan; it renumbers migrations and predates `_005_basic_lands`, so conflicts are expected and resolved by hand, never retyped.
- `Action` macro kind + keyword-action declarations + `Expansion<T>` — Plan 3.
- Quantity (X, CountOf) — first plan that needs a non-literal amount.
