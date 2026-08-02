---
needs: [cards-elaborator-tables, cards-elab-load-gate]
---
**Statics deltas (prevention class, two-slot deed agents) + open-vocabulary
registries (subtypes, counters, designations, keyword shapes) + closed mana
symbols.** The third grammar seam: everything that is a vocabulary or a static
ability shape rather than an event or a verb.

Related: [[engine-prevention]] (engine execution of the class minted here),
[[filter-head-subtype-validation]] (the parser-side fallback this registry
finally validates against).

## Prevention as its own class

```rust
Prevention(Prevention),        // PreventNext { n, from, to, duration }
                               //   | PreventNextInstance | PreventAll
CantPrevent { from: Filter, to: Filter },   // gates the Prevention class [CR#615.12]
```

Prevention [CR#615] is structurally distinct from generic `Replacement`
(Instead/Skip/Also) — "can't be prevented" gates exactly the `Prevention`
class and nothing else, by construction. Reject fixture: a damage-prevention
effect written as a generic `Instead`.

## Two-slot deed agents

```rust
struct Deed { relation: Relation, agent: DeedAgent, patient: Participant }
struct DeedAgent { stack_object: Option<Filter>, source: Option<Filter> }
Filter::FromSource(Box<Filter>)   // lifts a quality to a stack ability's SOURCE
```

The CR's two-armed wording [CR#702.11d,702.16b] as two slots: shroud sets
`stack_object` (spells and abilities); hexproof-from-red sets `source:
ColorIs(Red)`; protection sets both arms. Checker: at least one arm present;
patient kind gated per relation by an emitted `patientScope` table row.

## Registries (open vocabularies as data)

```ron
SubtypeDecl( name: "Island", category: Land, confers: [ Innate(mana-tap-U) ] )  // [CR#305.6]
CounterDecl( name: "time",  scope: Object )
CounterDecl( name: "poison", scope: Player )
DesignationDecl( name: "Monarch", scope: Player )
DesignationDecl( name: "Commander", scope: Object )
KeywordDecl( name: "Ward", shape: Costed )
enum ParamShape { None, Counted, Costed, CountedCost, Predicated,
                  PredicatedCosted, Named }
```

- Subtypes, counter kinds, designations, and keywords become registry ROWS,
  not enum members; rows carry the dependent index (category/scope/shape) as
  data. The elaborator enforces counter scope, designation scope, and subtype
  category against the loaded registries (`E-KIND-*`). This unblocks the
  vocabulary ceiling: ~420 corpus subtypes vs 32 enum members today, ~244
  counter kinds vs 12 — the largest single graduation blocker.
- `Ability::Keyword(KeywordUse)` = registry row + typed args validated against
  the declared `ParamShape`; a bare parameterized keyword (e.g. `Keyword(Morph)`
  with no cost) is a load error.
- **Innate conferral emission:** every registry conferral (basic-land mana per
  [CR#305.6], counter-conferred abilities, subtype statics) emits
  `Ability::Innate(…)` — a rule of the object, immune to ability-removal in
  the layer system [CR#113.12]. One emission path, no per-registry special
  case.

## Closed structural vocabularies

- `ManaSymbol` closes the printed-symbol set [CR#107.4]: `White..Green |
  Colorless | Generic(Uint) | Variable | Snow | Hybrid(HybridPair) |
  MonoHybrid(Color) /* {2/W} */ | Phyrexian(PhyrexianSym) |
  HybridPhyrexian(HybridPair)` — `HybridPair` enumerates the ten color pairs,
  so `{5/W}` and `{W/W}` have no wire form. Printed vs produced mana stay
  separate types.
- `ObjectKind` gains `CardCopy` [CR#109.1]; `ColorOrColorless` is a real enum;
  counter specs are always named (never an anonymous option).
- `SpendAsThough { mana_from: Filter, as_: SymbolPred }` static — the
  "spend as though it were mana of any color" family (~48 cards).

## Done

- Shapes and registries land in `deckmaste_core`/`deckmaste_plugin` with
  round-trip tests; registry files load as plugin data.
- Elaborator enforces: DeedAgent ≥ 1 arm, patientScope rows, registry scopes,
  keyword shapes, closed symbol set — one reject fixture each.
- Innate emission covered by a layer test: conferred basic-land mana ability
  survives a lose-all-abilities effect [CR#113.12].
- Scope tables emitted from Idris, rows CR-cited.

## Verification

- `idris2 --build mtg.ipkg` + `idris2 --exec emitTables` (in `idris/`).
- `cargo test --workspace`; `cargo xtask validate` clean; wizards regenerated
  (`cargo xtask generate plugins/wizards`) and re-validated.
- `cargo xtask elaborate --lock` re-blessed deliberately.
- `cargo xtask cite check` — 0 stale, `--list-noncompliant` empty.
