---
needs: [engine-combatant-role]
---
**Adopt the candidate root [context map](../../../CONTEXT-MAP.md),
[Game Model glossary](../../contexts/game-model/CONTEXT.md), and
[Oracle English glossary](../../contexts/oracle-english/CONTEXT.md) after the
nomenclature correction family lands.** (2026-09-04: the three renames still
open — `construction-dsl-node`, `english-v2-grammatical-relations`,
`english-v2-person-number-agreement` — are recorded by the audit as open
aliases rather than waited on.) This is the conformance and activation
ticket, not another compatibility pass.

Scope (ruling 2026-09-04): the Idris workbench, core, engine, and lowering's
core-facing output only; `deckmaste_semantics` and `idris/src/Semantics.idr`
are deletion-bound and untouched; their side converges at cutover when the RON
re-emit path points at `Experimental`.

Audit public names and explanatory prose across `deckmaste_core`, the engine,
the Idris workbench (`idris/src/Experimental/`), construction, English-v2,
data, the core-facing RON, generated fixtures, and contributor docs. Core, the
engine, and the workbench should use the same Game Model term for the same
concept; Oracle English keeps standard linguistic terms even where a spelling
such as Object or Predicate legitimately means something different in the Game
Model. Fix remaining contradictory aliases, stale comments, and generated
vocabulary. Update the owning context's `CONTEXT.md` only where implementation
evidence exposed a real flaw in a definition, not to accommodate an old
identifier.

Only after that audit, add the stable instruction in `CLAUDE.md` to read the
root `CONTEXT-MAP.md` before terminology-sensitive repository work, follow it
to each relevant glossary, and keep those documents current through the
domain-modeling workflow. Until this ticket lands, the map and glossaries
remain unactivated proposals referenced by this ticket family only.

Done when: the audited surfaces carry one Game Model term per concept, the
`CLAUDE.md` instruction is in place, the workspace is green, and `cd idris &&
./scripts/build` is green at its module count.

## As landed

The map and both glossaries are activated: `CLAUDE.md` gains a **Terminology**
section (before `## CR citations`) telling every session to read the root
`CONTEXT-MAP.md` before terminology-sensitive work, follow it to the owning
glossary, honour each entry's `_Avoid_` line, and keep both documents current
through the `domain-modeling` skill. Neither `CONTEXT-MAP.md` nor either
glossary ever carried an "unactivated proposal" caveat, so there was none to
remove there; the only statement of it is this ticket's own self-limiting
sentence above ("Until this ticket lands…"), left as the record.

### Audit — renames done

| surface | term (glossary rule) | action taken |
| --- | --- | --- |
| `deckmaste_core/src/continuous.rs` | **Delta** — `_Avoid_`: separate gain/lose and raise/lower vocabularies per property | `PlayerMod::SetTo/Raise/Lower` → `Set/Up/Down`, matching the sibling `NumericOp` and `LifeOp` that already spell the three ops that way |
| `deckmaste_engine/src/player_statics.rs`, `deckmaste_lowering/src/continuous.rs` | same | propagated (7 + 8 sites) plus the doc comments naming the old ops; the `deckmaste_semantics` input spellings are untouched and the `Lower` impl maps across |
| `deckmaste_engine/src/layer.rs` | **Static Spec** — `_Avoid_`: Static Effect | `static_effect_scope` → `static_spec_scope` (4 sites + the `no_dead_grammar` seam note) |
| `deckmaste_engine/src/resolve/targets.rs` | **Predicate** / **Filter** — `_Avoid_`: Predicate as an interchangeable noun | `resolve::target_spec_filter` → `target_spec_predicate` (12 sites) and its four `filter` binders → `predicate`; it returns a `Region<Predicate>` and its twin is already `target_spec_quantity`. The `deckmaste_legacy_render` private twin keeps its own name and is untouched |
| `deckmaste_lowering/src/region.rs` (+ `cost`, `selection`, `reference`, `effect`) | **Execution Frame** — `_Avoid_`: Frame when the qualification matters | `Site::Frame` → `Site::ExecutionFrame` (28 sites); the qualification matters because `Verb Frame` and "spelling frame" are live neighbours |
| `deckmaste_lowering/src/region.rs` | **Intrinsic Ability** — `_Avoid_`: Intrinsic as a synonym for engine-primitive | `in_child`'s `intrinsic` parameter → `supplied`; the region-parameter prose in `deckmaste_core/src/region.rs`, `deckmaste_lowering/src/{region,effect}.rs` reworded "intrinsic" → "engine-supplied" |
| `deckmaste_core/src/action.rs` | **Target** — a Target is declared under [CR#115.1] | `Action::deal_damage(target, …)` → `deal_damage(patient, …)`; the variant doc already said "patient" and now says a patient is not necessarily a [CR#115.1] target |
| workbench `Triggers/Effect/Events/Macros/Proofs/Cards` | **Duration** — `_Avoid_`: span, window | the whole `span` family renamed: `span` → `duration`, `SpanOk` → `DurationOk`, `DelaySpanOk` → `DelayDurationOk`, `spanIntro` → `durationIntro`, `spanEventOk` → `durationEventOk`, `spannable` → `boundsDuration`, and the five pin/bench names ending `…Span` (67 lines, 8 files) |
| workbench `Words.idr`, `Phrase.idr`, `Proofs/Anaphora.idr` | **Amount** — `_Avoid_`: Quantity for a denoted number | `outcomeIsQuantity` → `outcomeIsAmount`, `countQuantOutcomes` → `countAmountOutcomes`, `quantOutcome` → `amountOutcome`, `countQuantOutcomesIsFold` → `countAmountOutcomesIsFold`. The predicate is true exactly of outcomes that yield a number and feeds `ThatMuch : … -> Amount bs` |
| workbench `Card.idr` (+ `Cards/Faces`, `Cards/Turn`, `Proofs/Faces`) | contradictory alias with `deckmaste_card::Card` | `SplitCard` → `Split`, `FlipCard` → `Flip`, matching core's `Card::Split`/`Card::Flip` and the already-bare `Adventurer`/`SingleFaced`/`Transforming`/`ModalDfc` siblings |
| workbench `Words.idr`, `Effect.idr`, `Cards/Static.idr` | **Subtype** / **Card Type** — `_Avoid_`: Type | `TypeSpace` → `SubtypeSpace`: its three members are subtype families keyed off a Card Type ([CR#205.3]), not Card Types |

### Audit — comments and prose reworded

| surface | term | action taken |
| --- | --- | --- |
| `deckmaste_core/src/continuous.rs:297,307` | Static Spec | "an inner static effect" → "an inner Static Spec"; the `Conditionally` doc's "game-state predicate" → "game-state condition" (a `Condition` is not a `Predicate` over a candidate) |
| `deckmaste_lowering/src/property.rs:160` | Static Spec | "Whether a static effect is a deontic row" → "Whether a Static Spec is…" |
| `deckmaste_core/src/reference.rs:115` | Amount | "an amount a `Quantity`" → "an amount a `Count`" — a denoted number is never a Quantity |
| `deckmaste_core/src/region.rs:110` | Execution Frame | "out-of-band frame environment" → "out-of-band execution-frame environment" |
| `deckmaste_engine/src/legal.rs` (5), `layer.rs:418`, `replace_registry.rs` (2) | Static Spec | every "static effect" naming a `&StaticSpec` → "Static Spec" |
| `deckmaste_engine/src/{combat.rs,state.rs,step/mod.rs}` (5) | Designation | "combat designations" / "attacker/blocker designations" → "declarations"; Designation is the named-marker registry that lives in the same crate (`DesignationStore`), so this was a contradictory alias |
| `deckmaste_engine/src/state.rs:428` | Card Type | "The type registry" → "The card-type registry", matching `GameConfig`'s already-correct twin at `:135` |
| `deckmaste_engine/src/resolve/query.rs:260` | Decision Point | "surface a pending choice" → "surface a decision point" — Choice names the rules selection, not the engine boundary |
| `plugins/builtin_v2/.../keyword_actions/Monstrosity.ron` | Inflectional Form | "third-person verb form" → "third-person-singular present inflectional form" |
| workbench `Experimental.idr:2`, `Words.idr:552`, `Macros.idr:1708` | Instruction | "event/effect/ability constructions" → "event/instruction/…"; "verbs in effect position" → "verbs in instruction position"; the `unless` template "[effect] unless…" → "[instruction] unless…" |
| `docs/architecture-review.md`, `docs/keyword-policy.md` (4) | Instruction | `OneShotEffect` → `Instruction` in live normative prose |
| `docs/conformance.md` (2), `docs/decisions/oracle-text-is-forward-anaphoric.md` (3), `docs/decisions/workbench-ron-shaped-and-label-rulings.md` | Static Spec | `StaticEffect` → `StaticSpec` |
| `docs/idris-workbench-closure-tables.md` §2.5 + the §2 header | Static Spec, Subtype | `StaticEffect` → `StaticSpec` in the maintained §2.5 only (the doc's own header demotes every other section to an unmaintained snapshot); `Words.TypeSpace` → `Words.SubtypeSpace` |
| `docs/english-regular-vocabulary-tables-design.md:54` | Inflectional Form | "verb form" → "inflectional form" |
| `docs/oracle-style-guide.md:94` | Static Ability, Instruction | "spell effects and one-shot effects … static effects" → "spell abilities and one-shot instructions … static abilities" |
| `docs/rules-taxonomy.md` | Instruction, Amount, Selection | a dated supersession note at the head instead of rewriting 1,100 lines: this is the 2026-06-06 taxonomy, its `Effect` is now an Instruction, its value-position `Quantity` an Amount, and its `Reference` covers what the glossary splits into Reference, Selection and Amount |

### Audit — open aliases recorded, not started

| alias | glossary term | sites | why recorded |
| --- | --- | --- | --- |
| `effect` as a field/module/function name for executable syntax — core `Continuously.effect`, `May.effect`, `Ability::*.effect`, `Property`'s `effect`, module `deckmaste_core::effect`; engine `run_effect`, `unless_cost_effect`, `run_sba_effect`, `effect_is_deferred`, `triggered_effect_region`, `spell_effect`, module `resolve::effect`; workbench module `Experimental.Effect` and `effectNamesThisDoor`; three builtin_v2 stub `effect:` keys | **Instruction** / **Static Spec** | 1,260 `effect:` sites repo-wide | far over the ~200 ceiling, and the root is core's field name: renaming the engine or workbench leaves them contradicting core. Must move as one coordinated landing |
| `Type` / `TypeDef` / `TypeRef` / `types:` — core `type.rs`, `deckmaste_card::Card.types`, engine `GameConfig.types` and `GameImage.types` (which already spell the same concept `card_types` on `Characteristics`), `construction_core::DeclarationKind::Type`, `plugins/builtin_v2/macros/meta/Type.ron` | **Card Type** (`_Avoid_`: Type) | 2,187 bare `Type` + 1,437 `types:` | one concept across five surfaces; splitting it would create a worse alias than it removes |
| `ObjectId` / `GameObject` / `ObjectStore` for the addressable domain that holds objects **and** players | **Entity** | 1,171 `ObjectId` | the doc comment already concedes the mismatch; far over the ceiling |
| `Count` / `Countable` (core) beside `Amount` (builtin_v2 keyword-ability `params: [Amount]`, `lowering::region::amount`) | **Amount** | 2,614 | one concept, two names on neighbouring surfaces |
| `Property` (core) whose own docs and `confers:` field call every instance a conferral | **Conferral** | 260 | over the ceiling |
| `filter:` fields typed `Region<Predicate>` (core `cost.rs`) | **Predicate** (Filter is the operation) | 330 | over the ceiling |
| english-v2 `UnqualifiedReference` / `PostmodifiedReference` naming plural and coordinated nominals beside a sibling `NounPhrase` category | **Noun Phrase** ("Amounts and collections are not References") | 386 | over the ceiling |
| english-v2 `CountReference` / `ScalarReference` (+ `parse_/analyze_count_reference`, `parse_/analyze_scalar_reference`) naming a denoted number | **Amount** | 55 | construction and category identity in `constructions.rs`; renaming a category is an english-v2 landing that must re-measure the corpus, which this conformance ticket is not |
| english-v2 `CardinalQuantity` used both as a determiner constraint and as a scalar value | **Amount** in the value position | 49 | same reason |
| `VerbFrameAtom::Amount` / `CustomTailAtom::Amount` / `CoreVerbTailAtom::Amount` beside `VerbFrameSet::MeasureComplement` for the identical one-atom schema | **Measure Complement** | ~70 (32 Rust + 39 RON, incl. `core_verbs.ron`) | declaration data feeding the parser; belongs to an english-v2 landing with the full gate and census |
| `element` DSL declaration → `node` | — | 1,046 declarations | owned by the still-open `construction-dsl-node` |
| `Subject`/`Object` as constituent categories, `SubjectPronoun`/`ObjectPronoun` | Oracle English **Subject**/**Object** as relations | 70 + 94 | owned by the still-open `english-v2-grammatical-relations` |
| `third_person:` / `bare:` / `participle:` provider-row keys, `concord_class` | **Inflectional Form**, **Person**, **Number** | 99 | owned by the still-open `english-v2-person-number-agreement` |
| `construction_core::DeclarationKindFamily::SubtypeFamily(SubtypeCategory)` — "Family" and "Category" for one thing, and Category is a grammatical term inside a grammar crate | **Subtype** family ([CR#205.3c]) | 160 | over the ceiling |
| workbench `Deed` and `Act` for what core calls `Action`; `deed` is also house vocabulary in `deckmaste_core::deontic`, `deckmaste_engine::legal` and `xtask::facts` | **Action** / **Keyword Action** | 515 | a glossary gap (neither Deed nor Act is defined) rather than a workbench-only drift; needs a definition before a rename |
| workbench `Kind` (what a `Noun` denotes) vs core `Domain`, while core's own `Kind` is the register shape | **Referent Sort** | 980 | `Kind` means two different things across the two surfaces; needs a coordinated decision |
| workbench `nounDelta`/`amtDelta`/`predDelta`/… using `Delta` for the bindings a constituent introduces, beside the correct `Delta = Up\|Down\|Set` | **Delta** is a numeric change | 443 | homonym; an `Intro`-family renaming is the fix |
| english-v2 `SelectionResolution` / `SelectionDecision` / `SelectionCandidate` / `SelectionLoserReason` for parse disambiguation | collides with Game Model **Selection** | ~250 | glossary gap: Oracle English defines no term for parse disambiguation |

### Audit — checked and deliberately unchanged

- Workbench `StaticKind::CopyEffect` is **not** a Static Effect violation: "copy
  effect" is the CR's own term for the continuous effect ([CR#613.2a,707.2c]),
  and the row sits beside `Replacement`/`Prevention`, which are equally CR
  effect names. Verified against `data/rules/cr.txt` before deciding.
- `docs/decisions/core-explicit-regions.md:161` keeps `OneShotEffect::Targeted`:
  the sentence records that the constructor **was deleted**, so the old type
  name is the accurate history.
- Every "window" and "span" in core and the engine is a timing, priority or
  history-lookback scope, never the Duration of a Continuous Effect; the
  `_Avoid_` is concept-scoped and does not reach them. Workbench `Window`,
  `TriggerWindow`, `PlayWindow` likewise.
- `deckmaste_lowering`'s `object_kind_predicate` / `minimal_object_kind` /
  `StaticEffect` / `PlayerMod::SetTo` occurrences are input-side names of
  deletion-bound `deckmaste_semantics` types; core's own output is already
  `ObjectClass` / `StaticSpec` / `PlayerMod::Set`.
- `deckmaste_core::filter.rs` (`EntityClass`, `ObjectClass`, `Domain`,
  `CollectionDomain`), `decision.rs`, `quantity.rs`, `designation.rs`,
  `keyword.rs` (which explicitly refuses "intrinsic"), `deckmaste_card`
  (Card Face / Characteristics / Alternative Characteristics), and the
  Combatant role in core, the engine and `builtin_v2/.../Creature.ron` all
  already match their entries word for word.
- `"valence"`, `"Numerative"` and `"base verb frame"` in
  `docs/decisions/english-v2-rewrite.md` and
  `docs/decisions/builtin-v2-macro-spelling-and-grammar.md` are retained by
  each document's own dated amendment as the record of superseded vocabulary.
- CR-facing rules verbs (`Choose`, `ChooseValue`, `Search`, `SeparatePiles`,
  `ChoosePile`, the engine's `Choose*` decision-point kinds) are settled by
  `core-explicit-regions` and `decision-terminology` and were not audited for
  renaming.

## Landing record

Measured on change `yzktxwvw`, committed as `terminology: adopt the context
map and glossaries`; `english-v2-coverage.lock` and
`cr-coverage.lock` are **unmodified** — no construction, vocabulary, provider
row or declaration datum changed, so the corpus figures, the selection census
and the construction count are unchanged by construction and were not
re-measured. The one `plugins/builtin_v2` edit is a comment inside
`Monstrosity.ron`; RON comments are discarded at parse, so no generated
vocabulary changed and `cargo xtask generate plugins/wizards` was not run.

### Gates (foreground, last line each)

- `cargo check --workspace --all-targets` — ``Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.31s``
- `cargo fmt --check` — clean (output is only the four nightly-feature notices)
- `cargo clippy --workspace --all-targets` — ``Finished `dev` profile [unoptimized + debuginfo] target(s) in 16.27s``; 11 warnings, all the pre-existing `deckmaste_lowering` set (10 × "this pattern is unneeded as the `..` pattern can match that element", 1 × "items after a test module"). No new warning.
- `cargo test --workspace` — 128 `test result: ok` lines, 0 failed, 0 new ignored
- `cd idris && ./scripts/build` (after `rm -rf build`) — `47/47: Building Cards (src/Cards.idr)`, 0 Error/Warning lines
- `cargo xtask validate` — `plugins/builtin: 12 valid, 0 todos skipped, 0 invalid, 0 canon mismatch(es)`
- `cargo xtask cite check --list-noncompliant` — `0 non-compliant citation-looking string(s)`
- `cargo xtask cite check` — `checked 14498 citations against cr.txt (eff. 2026-08-07); 0 stale`
- `cargo xtask cite bless` — `blessed 1436 rules at cr_date 2026-08-07`; the lock nets **+1**, [CR#707.2c], newly cited by this record. A first pass had pruned [CR#205.3a] and [CR#508.3c], whose only other citations sit in `docs/tickets/done/` — the expected prune shape, pre-existing on the parent; naming them here re-cites them, so both stay registered and the prune is cancelled
- `jj --no-pager diff --git > round.diff && cargo xtask cite audit --diff < round.diff` — `audited 29 citation site(s)`; each rule text read against its claim. The new claims: the Referent Sort amendment's `[CR#102.1,109.1]` — [CR#102.1] defines the player, [CR#109.1] enumerates what an object is, exactly the two-axis split the entry now states; this record's `[CR#613.2a,707.2c]`, both of whose rule texts use the phrase "copy effect"; `[CR#205.3]` and `[CR#205.3c]` for subtypes and their correlation to a card type; and `[CR#115.1]` for a Target being declared as a spell or ability goes on the stack. The remainder are pre-existing citations on lines this round only reworded

### Glossary changes

One, in `docs/contexts/game-model/CONTEXT.md`:

- **Referent Sort** was "the domain classification … such as Player, Card,
  Spell, Permanent, or an Object of a particular Card Type" — one flat list.
  Implementation evidence refutes it: `deckmaste_core` realises the concept as
  two independent axes, `Domain` (`Player` | `Object` | `Entity`) with
  `EntityClass` on one side and `ObjectClass` on the other, and
  `Domain::admits` gates one against the other. The flat wording also
  duplicated the glossary's own **Object Class** entry. The entry now states
  the two axes — the Entity domain ([CR#102.1,109.1]) optionally narrowed by
  an Object Class or a Card Type — and says why a flat list is wrong: only an
  Object has an Object Class. No entry was reworded to accommodate an
  identifier; `ReferentSort` no longer exists in any crate, and that absence is
  recorded as an open alias above rather than blessed into the definition.

### Assurance counts

- restored 0, added 0, removed 0, ignored 0.
- re-spelled 6, all renamed subjects, same asserted outcome: Rust
  `deckmaste_lowering::region::tests::a_nested_region_declares_intrinsics_then_captures_the_enclosing_file`
  → `…_declares_supplied_params_then_captures_the_enclosing_file`; workbench
  `okUntilEndOfTurnSpan`, `okSingularNextTurnSpan`, `badPluralNextTurnSpan`,
  `okCantAttackLandNoSpan`, `vesuvanShapeshifterCopySpan` → their `…Duration`
  spellings. `scripts/check-pin-twins` passes on the renamed pair.

### Deviations and additions

- `docs/rules-taxonomy.md` got a dated supersession note rather than a
  term-by-term rewrite. It is the 2026-06-06 design taxonomy and already
  describes a superseded model (`Selector`, `MacroKind`); rewording only its
  vocabulary would have made stale content read as current.
- `docs/idris-workbench-closure-tables.md` was renamed only in §2.5 and the §2
  header. The document's own header demotes every other section to an
  unmaintained 2026-08-22 snapshot whose cells quote docstrings that are gone.
- `deckmaste_core::region.rs`'s "substrate revision", `designation.rs`'s
  "intrinsic expiry condition" and `damage_result_rule.rs`'s "intrinsic
  result" are ordinary-English uses, not the banned senses, and were left.

### Residue (routes at integrate)

- `docs/decisions/oracle-text-is-forward-anaphoric.md:121` and
  `docs/idris-workbench-closure-tables.md:661` name a workbench constructor
  `OnlyWhile` that no longer exists anywhere in `idris/`. The current
  `Effect.Conditionally` carries `(c : Condition (staticIntro se))` plus a
  `CondMarking`, i.e. the postposed threading the ADR attributes to
  `OnlyWhile`, so the pair appears to have collapsed into one marked
  constructor. This round fixed the type name (`StaticEffect` → `StaticSpec`)
  and did NOT invent a replacement claim; deciding what the ADR's second
  orientation is now is a workbench design question.
- `docs/idris-workbench-closure-tables.md` §2.5's `Effect.Conditionally` row
  still prints the old `(c : Condition bs) -> (se : StaticSpec (condIntro c))`
  signature, drifted beyond a rename.

### STOPs

None taken.
