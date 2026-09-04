---
needs: [ability-kind-taxonomy, idris-sba-not-a-static-ability]
---
**Delete the generic `Innate` Ability wrapper and represent each job it was
performing structurally.** The [`Intrinsic Ability`, `Conferral`, and keyword
definitions](../../contexts/game-model/CONTEXT.md) reserve distinct names for these concepts.
`Innate` currently hides type-conferred pseudo-abilities from visible ability
lists, makes some rows removal-immune, and hides nested keyword components.
Those are three independent concerns; [CR#113.12] does not create a special
removal-immune Ability class for them.

Scope (ruling 2026-09-04): the Idris workbench, core, engine, and lowering's
core-facing output only; `deckmaste_semantics` and `idris/src/Semantics.idr`
are deletion-bound and untouched; their side converges at cutover when the RON
re-emit path points at `Experimental`.

An Ability conferred by a Card Type or Subtype occupies the ordinary Ability
hierarchy. A real type rule is conferred through the appropriate state-based,
turn-based, continuous, attachment, or deontic property rather than being
forged into an Ability. Basic land types confer ordinary mana Abilities without
an `Innate` wrapper; those Abilities retain the CR classification Intrinsic
Ability ([CR#305.6]). Composite Keyword Ability components remain executable
children of the named keyword container but do not independently enter the
carrier's visible Ability list.

Remove `Ability::Innate` from `deckmaste_core` and the engine, and from every
core-facing RON author (`plugins/builtin_v2/macros/stubs/subtypes/`: Aura,
Equipment, Fortification, Saga). The workbench has no `Innate` wrapper and must
not grow one as its conferral shapes fill in. Re-home Aura, Ward, basic-land,
Equipment, Fortification, and other existing uses according to the distinctions
above.

Acceptance checks both `has/lacks [ability]` behavior and `loses all
abilities` behavior so structural hiding is not confused with rules immunity,
with the workspace green and `cd idris && ./scripts/build` green at its module
count.

## As landed

`Ability::Innate` is gone from `deckmaste_core`, and with it `peel_innate` /
`is_innate` and every look-through arm. Its three jobs are now structural.

`core::Property` gains one flavor, **`Static(Arc<Region<StaticSpec>>)`** — the
ability-free static rule of a type or subtype. [CR#113.12] is explicit that an
effect which "states a quality of that object" is "neither granting an ability
nor setting a characteristic", and "an Equipment can be attached to a creature"
([CR#301.5]) is exactly such a statement. `Property::conferred_static()` is its
reader; `Property::conferred_ability()` now emits the ability **unwrapped**.

### Every `Innate` use → job → structural replacement

| site | job it did | replacement |
| --- | --- | --- |
| `core::Ability::Innate` variant + `peel_innate` + `is_innate` | 1 (hide), 2 (immune) | deleted |
| `Ability::as_activated` / `as_triggered` / `mana_profile` / `is_triggered_mana_ability` look-through arms | — (consequence of 1) | deleted; the conferred ability IS the ability |
| `Property::conferred_ability` wrapping in `Innate` | 1 + 2, blanket | emits the ability bare — a conferred ability occupies the ordinary hierarchy ([CR#305.6] intrinsic mana ability, [CR#714.3a] Saga lore-counter replacement) |
| `derive::usable_abilities` (peel) / `derive::abilities` (filter) | 1 | one list; `usable_abilities` delegates to `abilities` |
| `derive::flatten_composites` peel | 3 | composite members stay executable children spliced only into the engine-internal enumeration; the card-facing list carries the container ([CR#702.21a]) |
| `derive::usable_ability_captures`, `activation::capture_ability_runtime_into` | 1 | peel dropped |
| `layer::ability_is_named` Innate arm | 2 | dropped |
| `layer::LoseAllAbilities` retain-`is_innate` | 2 | `retain(|_| false)` — [CR#305.7]: losing abilities loses the type-conferred ones too |
| `layer::LoseAbility` / `CantHaveAbility` `is_innate` guards | 2 | dropped |
| `copy::apply` `LoseAbility` / `LoseAllAbilities` immunity | 2 | dropped |
| `legal::walk_abilities` `in_ability` peel | 1 + 2 | `in_static` hoisted to `legal::walk_static`; `statics_on` now ALSO walks the object's `Property::Static` conferrals off its derived `card_types`/`subtypes` — the one choke point every deontic/SBA read already goes through |
| `combat::has_keyword` peel | 1 | dropped |
| `replace::as_enters_status` peel | 1 | dropped |
| `copy::defines_pt` Innate arm | — | dropped |
| `plugin::validate_ability_regions` Innate arm | — | dropped |
| `lowering::Ability::Innate` → `core::Ability::Innate` | — | wrapper erased: `f0.lower()` |
| `lowering::Property::Ability` | 1 + 2 | `ability_conferral`: a `Sba` static conferral re-homes to `Property::StateBased` ([CR#704.1,704.1a] — an SBA is not an ability), a DEONTIC static conferral (bare or `Conditionally`-gated) re-homes to `Property::Static` ([CR#113.12]), everything else stays `Property::Ability` |

### Per stub, old → new

builtin_v2 (core-facing):

| stub | old | new |
| --- | --- | --- |
| `subtypes/artifact/Equipment.ron` | `Ability(Innate(Static(May(Attach(This, Creature)))))` | `Static(May(Attach(This, Creature)))` [CR#301.5] |
| `subtypes/artifact/Fortification.ron` | `Ability(Innate(Static(May(Attach(This, Land)))))` | `Static(May(Attach(This, Land)))` [CR#301.6] |
| `subtypes/enchantment/Saga.ron` | `Ability(Innate(Static(Replacement(Also(ThisEnters, …)))))` | `Ability(Static(Replacement(Also(ThisEnters, …))))` — a real intrinsic ability [CR#714.3a] |
| `subtypes/enchantment/Aura.ron` | already `StateBased(…)` | unchanged (comment reworded) |
| `types/Land.ron` | `Ability(Static(May(Play(This))))` | `Static(May(Play(This)))` [CR#116.2a,305.9] |
| `types/Creature.ron` | 4 × `Ability(Static(…))` | 4 × `Static(…)` [CR#508.1a,509.1a,302.6] |
| `types/Instant.ron` | `Ability(Static(May(Cast(This, InstantSpeed))))` | `Static(May(Cast(This, InstantSpeed)))` [CR#304.1] |
| basic-land stubs (`subtypes/land/*.ron`) | no `confers` of their own; the mana ability is the `BasicLandType` meta-macro's `Ability(Activated(…))`, previously `Innate`-wrapped at emission | unchanged in the RON; now emitted bare — the Forest HAS "{T}: Add {G}" and loses it to "loses all abilities" ([CR#305.6,305.7]) |

builtin (v1-facing input; `Innate` may still appear there):

| macro | old | new |
| --- | --- | --- |
| `keyword/Ward.ron` | `Composite{abilities:[Innate(Triggered(…))]}` | `Composite{abilities:[Triggered(…)]}` — an executable child, not a second card-facing ability |
| `types/enchantment/Saga.ron` | `Ability(Innate(Static(Replacement(…))))` | `Ability(Static(Replacement(…)))` |
| `types/enchantment/Aura.ron` | `Ability(Innate(Static(Sba(…))))` | unchanged spelling; lowering re-homes it to `Property::StateBased` (a region-free core flavor, so the mapping is done on the LOWERED value — authoring `StateBased` directly in v1 would need an open lowering region the flavor has no place to store) |
| `types/artifact/{Equipment,Fortification}.ron`, `cardtype/{Land,Creature,Instant}.ron` | `Ability([Innate(]Static(<deontic>)[)])` | unchanged spelling; lowering re-homes each to `Property::Static` |

### Engine changes

- `legal::walk_abilities` no longer peels a wrapper; its `in_static` half is
  now the module-level `legal::walk_static`, and `legal::statics_on` walks the
  derived ability list AND the object's `Property::Static` conferrals. That
  single point feeds `attachment_legal`, `confers_may_play`, the attack/block/
  target/cast row collectors, the cost-modifier scan and the [CR#704] sweep, so
  no consumer needed its own change.
- `derive::abilities` and `derive::usable_abilities` are one list again.
- `layer` ability removal is unconditional; conferred abilities are stripped by
  `LoseAllAbilities` like printed text, while ability-free type rules are
  untouched because they never enter the list.

### Undone / not done

- `plugins/builtin` keeps the `Innate` identity macro and the marker on the
  Aura/Equipment/Fortification/Land/Creature/Instant conferrals: those are
  v1-facing input, `deckmaste_semantics` is off limits this round, and the gate
  scopes the ban to `deckmaste_core`, `deckmaste_engine` and `plugins/builtin_v2`.
  Both delete at cutover.
- `deckmaste_semantics`, `idris/`, and `idris/src/Semantics.idr` untouched.

## Landing record

### Gates

- `cargo check --workspace --all-targets` → `Finished \`dev\` profile [unoptimized + debuginfo] target(s) in 3.65s`
- `cargo fmt --check` → clean (only the unstable-option warnings rustfmt always prints)
- `cargo clippy --workspace --all-targets` → no warning in any file this round touched (the 11 remaining are pre-existing in `deckmaste_lowering/src/card.rs` and `ability.rs`)
- `cargo test --workspace` → `passed 6094 failed 0 ignored 6` (summed over every crate's `test result` line)
- `cd idris && ./scripts/build` → `44/44: Building Cards (src/Cards.idr)`, 0 lines matching `warning`
- `cargo xtask cite check --list-noncompliant` → 0 in this diff; 2 pre-existing hits remain in `docs/tickets/done/ability-kind-taxonomy.md:165,168` (a prior round's prose quoting a bless line's bare rule number), untouched here
- `cargo xtask cite bless` → `newly registered rule(s)`: [CR#304.1] only, no prune. Its text — "A player who has priority may cast an instant card from their hand" — is exactly the Instant type's conferred cast permission.
- `cargo xtask cite check` → `checked 18249 citations against cr.txt (eff. 2026-08-07); 0 stale`
- `jj --no-pager diff --git > /tmp/aci.diff && cargo xtask cite audit --diff < /tmp/aci.diff` → `audited 118 citation site(s)` (final run, including this record); every rule read against its claim. One correction made during the read: the Land play permission now cites [CR#116.2a,305.9] rather than [CR#305.9] alone — the latter only settles land-vs-spell precedence, while the former is the permission itself.
- `grep -rn "Innate" crates/deckmaste_core crates/deckmaste_engine plugins/builtin_v2` → empty (exit 1)

### Test counts

- restored: 0
- re-spelled: 22 — `legal`: the 3 walker tests via `sample_tree` (the `Innate` branch became a NESTED composite, same DFS expectation), `attachment_legal_honors_attachment_side_grant` (now a subtype-conferred `Property::Static`), `attachment_legal_false_on_self_and_missing_host`, `attachment_legal_honors_host_side_cant`, `attachment_legal_guard_is_inert_for_a_normal_grant` (printed `May(Attach)` static — Enchant's shape), `innate_before_activated_does_not_desync_the_index` → `static_before_activated_does_not_desync_the_index`; `derive`: `abilities_of_source_peels_innate_triggered` → `abilities_of_source_splices_a_composite_keyword_trigger`; `layer`: `innate_static_survives_lose_all_abilities` → `conferred_static_rule_survives_lose_all_abilities`, `conferred_basic_land_mana_survives_lose_all_abilities` → `conferred_basic_land_mana_is_an_ordinary_ability`, `lose_ability_does_not_remove_innate` → `lose_ability_removes_only_the_named_keyword`, `derive_abilities_filters_innate_out` → `derive_abilities_omits_ability_free_type_rules`, `normal_static_is_removed_by_lose_all_abilities`; `sba`: `ability_less_aura_still_graveyards` (now driven by the subtype's `StateBased`), `wizards_aura_carries_innate_graveyard_sba` → `wizards_aura_carries_conferred_graveyard_rule`; `plugin`: `wizards_attachment_subtypes_carry_innate_confers` → `..._carry_ability_free_confers`; `attach_subtypes`: all three subtype tests; `tests/layers.rs`: `losing_creature_type_removes_the_attack_grant`, `printed_creature_grant_is_not_doubled_by_the_fold` (both now read the conferred rule off the derived `card_types`); `lowering`: `lowers_ability_innate` → `lowers_ability_innate_to_its_inner_ability`; `construction_core`: `rules_defined_conferrals_stay_on_their_subtype_declarations`, `builtin_v2_types_load_with_exact_semantics_and_noun_surfaces` (expected RON updated).
- ignored added: 0 (the 6 ignored are pre-existing, each with its blocker named)
- added: 1 — `lowering::property::deontic_conferral_re_homes_to_property_static`
- removed: 0

### Acceptance

- **has / lacks a type-conferred ability**: `layer::tests::conferred_basic_land_mana_is_an_ordinary_ability` — the Island HAS its conferred `{T}: Add {U}` in `derive::abilities` (len 1), activates it through the real payment protocol, and after `LoseAllAbilities` no longer taps for blue and reads as having no abilities ([CR#305.6,305.7]).
- **loses all abilities removes a conferred ability but not a state-based type rule**: the same test's second half, plus `sba::tests::ability_less_aura_still_graveyards` (the Aura subtype's `Property::StateBased` [CR#704.5m] rule still fires after every ability is stripped) and `layer::tests::conferred_static_rule_survives_lose_all_abilities` (an Equipment-shaped `Property::Static` rule is still read by the static walk after `LoseAllAbilities` removes Trample).
- **composite keyword components do not enter the visible list**:
  `derive::tests::abilities_of_source_splices_a_composite_keyword_trigger` —
  the engine-internal enumeration splices the member, the card-facing list
  carries only the container.

### Deviations and additions

- **Added `core::Property::Static`.** The ticket names "state-based,
  turn-based, continuous, attachment, or deontic" property homes; core had only
  the first three. Every rule that needed re-homing (Equipment/Fortification
  `May(Attach)`, Land `May(Play)`, Creature's `May(Attack)`/`May(Block)` plus
  its `Conditionally`-gated summoning-sickness `Cant` pair, Instant
  `May(Cast)`) is a deontic row, but two of them wear a `Conditionally`
  wrapper, so a bare `Deontic(Deontic)` variant could not hold them. `Static`
  takes the same `Arc<Region<StaticSpec>>` payload `Ability::Static` does, which
  keeps every RON author a one-token edit and lets the engine reuse the existing
  static walker verbatim.
- **Land, Creature and Instant card types were re-homed too**, though the
  ticket names only the four subtype stubs and the basic lands. They are not
  `Innate` in the RON, but they went through the `conferred_ability` emission
  path that wrapped everything, so unwrapping without re-homing them would have
  made a creature that "loses all abilities" unable to attack — wrong under
  [CR#113.12]. This is the "and other existing uses" clause of the ticket.
- **Lowering does a shape-directed re-home** (`ability_conferral`) rather than
  each v1 macro being re-authored, per the ticket's "map it to the structural
  form on the core-facing side". The rule is content-directed and stated in the
  code: a type/subtype-conferred DEONTIC row states a quality ([CR#113.12]) and
  a conferred `Sba` static is a game action ([CR#704.1a]); neither is an
  ability. A conferred `Activated` ([CR#305.6]) or `Replacement` ([CR#714.3a])
  is, and stays one.
- **Two clippy fixes** in code this round reshaped: `manual_let_else` ×2 in
  `core::ability` (the matches lost their `Innate` arm) and `let_and_return` in
  the `layer` test helper `pump_static` (which lost its `innate: bool`
  parameter).
- `crates/deckmaste_lowering/tests/diagnostics.rs` shows a one-line trailing-
  blank-line removal from `cargo fmt`; not a deliberate change.

### STOPs

None. No use required a fourth shape: every `Innate` site mapped onto one of
hiding-by-not-being-an-ability, removal-immunity-by-not-being-an-ability, or
keyword-container membership.
