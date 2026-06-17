---
needs: []
---
# Targets on effects (`Targeted`) — Implementation Plan

> **For agentic workers:** implement task-by-task with TDD. Steps use `- [ ]`.

**Goal:** Move a targeted ability's `targets: Vec<TargetSpec>` declaration off the
ability struct and onto a new `Effect::Targeted` wrapper that scopes
`Reference::Target(n)` over its inner effect — the rules-faithful home for
targets ([CR#115.1,601.2c,611.2,608.2b]).

**Architecture:** Expand → migrate → contract. (A) Add the `Targeted`
type and teach the engine to *also* read targets from a top-level wrapper
(dual-read) and resolve through it. (B) Flip the parser to emit wrappers,
migrate the 9 authored RON files, retarget the renderer. (C) Remove the
`targets` field from the 4 ability structs, drop the dual-read, fix fallout,
regenerate wizards.

**Tech stack:** Rust workspace; `macro_ron` derive (`SupportsMacros`/`Expand`)
for RON round-trip; `cargo xtask generate plugins/wizards`; `cargo xtask cite check`.

## Global constraints
- Run jj only via `scripts/jj`; never bare `jj`/`git`. Work in `../engine-targets-on-effects`.
- CR citations use the repo bracket form; after citation changes the noncompliant list must be empty and stale count 0; bless any new rule.
- Never edit `plugins/wizards/` by hand — it is regenerated.
- Keep resolution semantics identical for the single-top-level-wrapper case
  (every real card today): `frame.targets` is populated at announce, so
  `Target(n)` resolves unchanged. Nested/multiple wrappers = a loud `todo!` seam.

## Design decisions (settled in brainstorming)
- `targets: Vec<TargetSpec>` on the wrapper (keep `Quantity` + `Distinct`; the
  `Distinct` co-target indices reference siblings within this same list, [CR#115.7e]).
- `Target(n)` index scope = the enclosing `Targeted` (local). Single
  wrapper ⇒ local == global announce order; the engine flattens to `frame.targets`.
- Multi-pick specs (`AtMost(n)`) admitted by the shape; enforcement deferred.
- Fight/Exchange all-or-nothing illegal-target atomicity (the Fight and
  Exchange keyword-action rules) is **out of scope** — its citations land with
  the code whenever those actions are built.

---

### Task 1: Core — `Targeted` type + `Effect` variant

**Files:**
- Modify: `crates/deckmaste_core/src/effect.rs` (add struct + enum arm near `ContinuouslyEffect`, effect.rs:73-79 / enum effect.rs:39-71)
- Test: `crates/deckmaste_core/src/effect.rs` `mod tests`

**Produces:** `Effect::Targeted(Targeted)`, `struct Targeted { targets: Vec<TargetSpec>, effect: Box<Effect> }`.

- [ ] **Step 1 — failing round-trip test.** Add to effect.rs tests:
```rust
#[test]
fn targeted_effect_round_trips() {
    use crate::target_spec::TargetSpec;
    let ron = "Targeted(targets: [AnyTarget], effect: DealDamage(Target(0), 3))";
    let e: Effect = crate::ron::from_str(ron).expect("parse");
    assert!(matches!(&e, Effect::Targeted(t) if t.targets.len() == 1));
    assert_eq!(crate::ron::to_string(&e).unwrap(), ron); // bidirectional
}
```
(Confirm the exact `crate::ron` reader/writer entry points used by sibling tests in this file — mirror them.)

- [ ] **Step 2 — run, expect FAIL** (`Effect::Targeted` unknown variant): `cargo test -p deckmaste_core targeted_effect_round_trips`

- [ ] **Step 3 — implement.** In effect.rs, mirror `ContinuouslyEffect`:
```rust
/// Target-scoping wrapper ([CR#115.1,601.2c]): declares the targets its inner
/// effect consumes, scoping `Reference::Target(n)` to this list. Targets are
/// chosen at announcement and stored on the stack object; at resolution this
/// node is transparent — the inner effect runs with `frame.targets` already
/// bound. Per-instance illegal-target handling ([CR#608.2b]) reads each inner
/// instruction's referenced targets. `effect` is boxed to break the
/// `Effect`→`Targeted`→`Effect` size cycle (mirrors `MayEffect`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, Expand)]
pub struct Targeted {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub targets: Vec<crate::target_spec::TargetSpec>,
    pub effect: Box<Effect>,
}
```
And add to the `Effect` enum (after `Modal(ModalEffect)`):
```rust
    /// Targets scoped over an inner effect ([CR#115.1,601.2c]).
    Targeted(Targeted),
```

- [ ] **Step 4 — run, expect PASS.**
- [ ] **Step 5 — commit** (`scripts/jj describe`/`commit`): `core: add Effect::Targeted target-scoping wrapper [CR#115.1,601.2c]`

---

### Task 2: Engine — resolve through `Targeted`

**Files:**
- Modify: `crates/deckmaste_engine/src/resolve.rs` (the effect-dispatch `match` containing `Effect::Sequence`/`Effect::Continuously`; near resolve.rs:396)
- Test: `crates/deckmaste_engine/tests/` (a wrapper-shaped targeted spell deals damage to its target)

**Consumes:** `Effect::Targeted` (Task 1).

- [ ] **Step 1 — failing test:** build a spell whose effect is `Targeted { targets: [AnyTarget], effect: DealDamage(Target(0), 3) }`, cast it at a creature, resolve, assert the creature took 3. (Adapt an existing Lightning-Bolt-style engine test; locate it first — `rg -n "DealDamage" crates/deckmaste_engine/tests`.)
- [ ] **Step 2 — run, expect FAIL** (unhandled `Effect::Targeted` arm / `todo!`/non-exhaustive match).
- [ ] **Step 3 — implement.** Add the arm to the effect-dispatch match, mirroring how `Sequence` recurses (confirm the exact recursion fn + args by reading the surrounding arms):
```rust
Effect::Targeted(te) => {
    // Targets were chosen at announce and already live in `frame.targets`;
    // a single top-level wrapper is the only shape today. Descend.
    // (Nested wrappers would need a target-scope stack — loud seam.)
    /* recurse into &te.effect with the same (frame, …) the Sequence arm uses */
}
```
- [ ] **Step 4 — run, expect PASS.**
- [ ] **Step 5 — commit:** `engine: resolve through Targeted (transparent; frame.targets pre-bound)`

---

### Task 3: Engine — discover targets from the wrapper (dual-read)

**Files:**
- Modify: `crates/deckmaste_engine/src/resolve.rs` (`spell_targets`, `spell_targets_list` ~1513-1540)
- Modify: `crates/deckmaste_engine/src/cast.rs` (`announce_targets` ~633-656; the `StackObject::Activated { ability, .. } => ability.targets.clone()` at ~640)
- Modify: `crates/deckmaste_engine/src/trigger.rs` (`trigger_targets` ~798-800; `Ability::Triggered(t) => t.targets.clone()`)
- Test: announce surfaces targets for a wrapper-shaped ability (spell + activated + triggered).

**Architecture note:** add a shared helper `fn top_targets(effect: &Effect) -> &[TargetSpec]` that returns the targets of a top-level `Effect::Targeted` (peeling `Expanded`), else `&[]`. Each discovery site reads `top_targets(&ability.effect)` **in addition to** the existing `ability.targets` field (dual-read) so both shapes work during migration. Concretely: `let specs = if !ability.targets.is_empty() { ability.targets.clone() } else { top_targets(&ability.effect).to_vec() };`

- [ ] **Step 1 — failing test** (announce finds targets from a wrapper, with the ability `targets` field empty).
- [ ] **Step 2 — run, expect FAIL.**
- [ ] **Step 3 — implement** `top_targets` + dual-read at the three discovery sites.
- [ ] **Step 4 — run, expect PASS** (and existing announce tests still green).
- [ ] **Step 5 — commit:** `engine: discover targets from Targeted wrapper (dual-read during migration)`

---

### Task 4: Parser — emit `Targeted` wrappers

**Files:**
- Modify: `crates/deckmaste_migrations/src/parsers/spell_ability.rs:27-37`
- Modify: `crates/deckmaste_migrations/src/parsers/activated_ability.rs:36-47`
- Modify: `crates/deckmaste_migrations/src/parsers/triggered_ability.rs:57-67`
- Test: the existing per-file parser unit tests (update expected strings).

**Change:** when `parsed.targets` is non-empty, wrap the effect instead of adding a top-level `targets:` field. Spell example:
```rust
fn render(parsed: &ParsedEffect) -> String {
    if parsed.targets.is_empty() {
        format!("Spell(effect: {})", parsed.effect)
    } else {
        format!(
            "Spell(effect: Targeted(targets: [{}], effect: {}))",
            parsed.targets.join(", "),
            parsed.effect
        )
    }
}
```
Activated keeps `cost:` on the ability and wraps only the effect:
`Activated(cost: [..], effect: Targeted(targets: [..], effect: ..))`.
Triggered: `Triggered(event: .., effect: Targeted(targets: [..], effect: ..))`.

- [ ] **Step 1 — update the failing parser unit tests** to expect the wrapped shape (they currently assert `targets: [...]` on the ability).
- [ ] **Step 2 — run, expect FAIL** (parsers still emit old shape): `cargo test -p deckmaste_migrations`
- [ ] **Step 3 — implement** the three render changes.
- [ ] **Step 4 — run, expect PASS.** Then a generate smoke on one card: confirm a known targeted card (e.g. Lightning Bolt) parses+reparses cleanly.
- [ ] **Step 5 — commit:** `migrations: parser emits Targeted wrapper for targeted abilities`

---

### Task 5: Migrate the 9 authored RON files + their assertions

**Files (authored RON — wrap each `targets:` into the effect):**
- `plugins/canon/cards/Lightning Bolt.ron`, `Footlight Fiend.ron`, `Goblin Medics.ron`
- `plugins/testing/cards/Creature tap-activated DealDamage AnyTarget.ron`, `Sorcery X DealDamage AnyTarget.ron`
- `plugins/builtin/macros/keyword/Equip.ron`, `Enchant.ron`, `Fortify.ron`, `Reconfigure.ron`
- Modify tests: `crates/deckmaste_cards/tests/canon.rs:90-111` (`spell.targets[0]` → reach into the wrapper), `tests/keywords.rs:107,162` (`.targets.is_empty()` checks).

Example (Lightning Bolt): `Spell(targets: [AnyTarget], effect: DealDamage(Target(0), 3))`
→ `Spell(effect: Targeted(targets: [AnyTarget], effect: DealDamage(Target(0), 3)))`.
Equip: move `targets: [...]` inside → `Activated(cost: Param(0), window: SorcerySpeed, effect: Targeted(targets: [Target(Exactly(Literal(1)), AllOf([Creature, ControlledBy(Ref(You))]))], effect: Attach(what: This, to: Target(0))))`.

- [ ] **Step 1 — update canon/keyword tests** to read targets through the wrapper (failing).
- [ ] **Step 2 — run, expect FAIL.**
- [ ] **Step 3 — migrate the 9 RON files.**
- [ ] **Step 4 — run, expect PASS:** `cargo test -p deckmaste_cards canon && cargo test -p deckmaste_cards keywords`
- [ ] **Step 5 — commit:** `data: migrate authored RON (canon/testing/builtin keywords) to Targeted`

---

### Task 6: Renderer — read targets from the wrapper

**Files:**
- Modify: `crates/deckmaste_cards/src/render/ability.rs:25` (`targets: &t.targets` → from the wrapper)
- Modify: `crates/deckmaste_cards/src/render/mod.rs:69-94` (`RenderCtx.targets`), `render/fragment.rs:48,123` (`ctx.targets.get(i)` for `Reference::Target(i)`)
- Test: `crates/deckmaste_cards/tests/render.rs` (targeted card renders identically).

**Change:** when rendering an ability whose effect is a top-level `Targeted`, set `ctx.targets` from that wrapper and render its inner effect; non-targeted abilities pass `&[]`. Reuse `top_targets` from Task 3 if exposed, else a local peel.

- [ ] **Step 1 — render test** for a targeted card asserting the prior rendered text (failing if ctx.targets no longer found).
- [ ] **Step 2 — run, expect FAIL.**
- [ ] **Step 3 — implement** the wrapper-aware ctx setup.
- [ ] **Step 4 — run, expect PASS:** `cargo test -p deckmaste_cards render`
- [ ] **Step 5 — commit:** `cards: renderer reads Target(n) specs from the Targeted wrapper`

---

### Task 7: Contract — remove the `targets` field from abilities

**Files:**
- Modify: `crates/deckmaste_core/src/ability.rs` — delete `targets: Vec<TargetSpec>` from `SpellAbility`(20-25), `ActivatedAbility`(30-49), `TriggeredAbility`(66-79), `Mode`(115-122).
- Modify: drop the dual-read in resolve.rs/cast.rs/trigger.rs (Task 3) → `top_targets(&ability.effect)` only; `StackObject::Activated`/`Ability::Triggered` arms read the wrapper.
- Fix fallout: ~111 `targets: vec![]` literals (delete the line) + `.targets` reads across `crates/` (compiler-listed). Test fixtures in core ability.rs tests, engine `tests/{skeleton,stack,activate}.rs`, cards `tests/{tokens,render}.rs` + `src/validate.rs`, condition.rs.

- [ ] **Step 1 — delete the 4 fields.**
- [ ] **Step 2 — `cargo build --workspace`**, collect every error.
- [ ] **Step 3 — fix mechanically:** empty `targets: vec![]` literals → delete; populated literals → wrap effect in `Targeted`; discovery sites → wrapper-only.
- [ ] **Step 4 — `cargo test --workspace`** green.
- [ ] **Step 5 — commit:** `core+engine: remove ability.targets field; targets live only on Targeted`

---

### Task 8: Regenerate wizards + full verification

- [ ] **Step 1 — regenerate:** `rm -rf plugins/wizards && cargo xtask generate plugins/wizards` (clean regen; the generator is incremental).
- [ ] **Step 2 — `scripts/jj st`** clean except wizards content; spot-check a generated targeted card has the wrapper shape.
- [ ] **Step 3 — full suite:** `cargo test --workspace` (incl. canon 0-mismatch), `cargo clippy --workspace`, `cargo xtask cite check --list-noncompliant` (empty) + `cite check` (0 stale).
- [ ] **Step 4 — commit:** `wizards: regenerate under Targeted; full suite green`
- [ ] **Step 5 — `scripts/workflow refresh engine-targets-on-effects`** (from default) to keep current with trunk; integrate when satisfied.

## Self-review notes
- Spec coverage: type (T1), resolve (T2), discover (T3), parse (T4), data (T5),
  render (T6), contract (T7), regen/verify (T8). Render path (recon §2 hotspot 4)
  covered by T6; parser (hotspot 2) by T4; `eval_reference` (hotspot 1) needs **no
  change** for single wrapper — `frame.targets` unchanged (noted in T2).
- Seams left loud: nested/multiple `Targeted`, modal per-mode targets
  (now expressible as a wrapper inside the chosen mode's effect — still gated on
  the existing modal announce seam), Fight/Exchange atomicity.
