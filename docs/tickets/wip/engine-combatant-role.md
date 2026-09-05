---
needs: [ability-conferral-without-innate]
---
**Replace the `May(Attack)` proxy with an explicit conferred Combatant role.**
The candidate [Game Model glossary](../../contexts/game-model/CONTEXT.md) defines Combatant as the
bundled creature-like role: power and toughness matter for relevant damage;
the Permanent is in the domains for attacking, blocking, and fighting; and the
continuous-control rule informally called summoning sickness applies to its
attacks and tap/untap-symbol abilities. Individual permissions and
restrictions determine whether it may perform a particular action; they do not
define or revoke the role.

The current implementation from `engine-combatant-capability` uses presence of
the Creature type's `May(Attack)` conferral as a witness for the whole bundle:
`is_combatant` delegates to `attackable`, while `May(Block)` and the
summoning-sickness `Cant` pair are separately conferred alongside it. That is a
working invariant for today's Creature definition, but it makes one permission
the accidental identity of the larger concept. A source that confers only
`May(Attack)` or only `May(Block)` would silently acquire or fail to acquire
unrelated damage, fight, and summoning-sickness semantics.

Represent Combatant as an ordinary conferred rules property in the type and
subtype registries after the generic `Innate` wrapper is removed. The Creature
Card Type normally confers that property while the Object is a Permanent.
Derive the role's ordinary attack and block permissions, its eligibility for
creature-only combat and fight rules, creature damage marking, and the
applicability of the summoning-sickness `Cant` pair from the role. Preserve
independent `May(Attack)` and `May(Block)` expressions for effects that grant
only an action permission; neither one alone confers Combatant.

Counterfactual treatment is scoped. If an effect says a non-Combatant Permanent
may block as though it were a 1/1 creature, treat it as a 1/1 Combatant only
while determining and performing that block ([CR#609.4]). It does not become a
Combatant for unrelated purposes: its tap ability is not summoning-sick, it is
not generally a legal fight participant, and damage outside that operation is
not given creature damage semantics merely because it received the blocking
permission.

Acceptance cases:

- A Creature restricted from attacking, blocking, or both remains a Combatant.
- A Combatant with defender remains a Combatant even though it cannot attack.
- Gaining or losing the Creature type gains or loses the role through layered
  registry conferral, unless another current source confers it.
- A fresh noncreature with `{T}: Draw a card` and only the scoped permission to
  block as though it were a 1/1 creature may activate that ability; the scoped
  block treatment does not globally enable summoning sickness.
- Attack, block, fight, damage, and tap/untap-activation tests read the explicit
  role rather than using either individual `May` row as its proxy.

This is a correction to the abstraction boundary established by
`engine-combatant-capability`, not a return to literal `Type::Creature` checks.
The registry remains the source of truth, so custom Card Types may confer the
complete Combatant role without adding hard-coded engine branches.

## As landed

Combatant is a conferred rules property in its own right. `core::StaticSpec`
gains one variant, **`Role { who: Predicate, role: Role }`**, and a `Role` enum
whose sole member is `Combatant` — a quality of the object ([CR#113.12]),
carried on the ability-free `Property::Static` flavor the previous round added.
The `Creature` Card Type confers exactly that, gated on the bearer being a
permanent ([CR#110.1]):

```
Static(Conditionally(Matches(This, Permanent), Role(who: Ref(This), role: Combatant)))
```

- **The role is read, and the rules are derived.** `legal::is_combatant` walks
  the object's DERIVED types and subtypes for a `Role(Combatant)` conferral
  naming it — so layer-4 type changes gain and lose the role with the type, and
  a second current source (another type, a subtype) keeps it.
  `legal::COMBATANT_RULES` holds the four rules a Combatant plays by, and
  `for_each_static` splices them onto every role-holder: `May(Attack)` and
  `May(Block)` ([CR#508.1a,509.1a]) and the summoning-sickness pair —
  `Cant(Attack)` and `Cant(Activate(cost: IncludesTapSymbol))`, both gated on
  `SummoningSick && !Has(Haste)` ([CR#302.6,602.5a,702.10b]). Every deontic
  collector reads them exactly as it reads a printed row, so
  `attackable`/`blockable`/`cant_attack_rows`/`cant_activate` needed no change.
- **`is_combatant` no longer delegates to `attackable`.** Combat-damage marking
  (`step::combat`, [CR#120.3e]) reads the role; a bare `May(Attack)` or
  `May(Block)` grant confers no role, no sickness and no damage marking.
- **`legal::statics_on`'s conferral half is now `legal::conferred_statics`**, a
  reusable iterator over the derived types'/subtypes' `Property::Static` rules —
  the role read and the static walk share it, so the role can never be
  witnessed by a rule it derives.
- **Lowering carries the v1 authoring across.** `lowering::property::conferrals`
  runs over every lowered `TypeDef`/`Subtype` conferral list: a list holding
  BOTH self `May(Attack)` and self `May(Block)` collapses — those two plus the
  gated self `Cant(Attack)`/`Cant(Activate(tap))` rows — into the single
  `Role(Combatant)` conferral. One grant alone is an action permission and
  passes through untouched. v1 has no role row in its vocabulary
  (`deckmaste_semantics` is deletion-bound and untouched this round), so this
  is the adapter that keeps `plugins/builtin` authoring the live registry; it
  deletes with the v1 path at cutover.
- **`plugins/builtin_v2/macros/stubs/types/Creature.ron`** now declares the
  single role conferral directly (four rows → one), with the exact-text
  expectation in `construction_core`'s `builtin_v2_types` updated to match.

### Not done

- The **fight both-or-neither guard** ([CR#701.14b]) and the creature-scoped
  SBAs ([CR#704.5f,704.5g]) stay `Type(Creature)` predicates in v1 card data.
  Pointing them at the role needs a role atom in the PREDICATE vocabulary,
  which only `deckmaste_semantics` (untouchable this round) can author; core
  gains no predicate nothing can spell. Fight-eligibility fixtures therefore
  still read the creature type, while every ENGINE read — attack and block
  declaration, damage marking, the tap gate — reads the role.
- The [CR#609.4] "as though it were a 1/1 creature" counterfactual is honored
  in its negative direction only: the block permission is an ordinary
  `May(Block)` row, and the permanent holding it plays no role (no sickness, no
  damage marking, no attack). The positive half — a 1/1 P/T overlay applying
  while that block is determined and performed — is the existing
  `AsThough::Counterfactual` seam, which realizes keyword premises only.

## Landing record

Measured on change `oxuqpsmk` (working copy at gate time), parent
`tqpzluzt engine-combatant-role`.

### Gates

- `cargo check --workspace --all-targets` → `Finished \`dev\` profile [unoptimized + debuginfo] target(s) in 3.21s`
- `cargo fmt --check` → clean (only the unstable-option warnings rustfmt always prints)
- `cargo clippy --workspace --all-targets` → 11 warnings, all pre-existing: 10 in `deckmaste_lowering/src/card.rs`, 1 in `ability.rs`. No warning in any file this round touched.
- `cargo test --workspace` → `passed 6150 failed 0 ignored 6` (summed over every `test result` line; 7m45s wall)
- `cargo xtask validate` → `plugins/builtin: 12 valid, 0 todos skipped, 0 invalid, 0 canon mismatch(es)`
- `cargo xtask generate plugins/wizards` → tree hash `21f9778a…` before and after: the generator's output is unchanged by this round
- `cargo xtask cite check --list-noncompliant` → `0 non-compliant citation-looking string(s)`
- `cargo xtask cite bless` → `blessed 1432 rules at cr_date 2026-08-07`; `cr-citations.lock` unmodified — no rule newly registered, none pruned
- `cargo xtask cite check` → `checked 14467 citations against cr.txt (eff. 2026-08-07); 0 stale`
- `jj --no-pager diff --git > /tmp/round.diff && cargo xtask cite audit --diff < /tmp/round.diff` → `audited 48 citation site(s)`; every rule read against its claim. One correction made during the read: the `Role::Combatant` doc's toughness comparison now cites [CR#120.3e,704.5g] rather than [CR#120.3e] alone, which covers marking only.

### Performance advisory

No english_v2 surface is touched, so the coverage command, the coverage lock
and the selection census are not in this round's scope and carry no figures.
The engine cost this round adds is one extra pass over an object's derived
type/subtype conferrals per `for_each_static` visit (the role read), whose only
condition is the permanent gate; the summoning-sickness conditions are
evaluated exactly as often as before, since the pair moved from a conferral to
a role-derived splice rather than being duplicated. `cargo test --workspace`
wall time is 7m45s.

### Test counts

- restored: 0
- re-spelled: 3 tests — `legal::is_combatant_reads_grant_presence_not_net_eligibility` → `a_creature_forbidden_to_attack_still_plays_the_role`; `layers::losing_creature_type_removes_the_attack_grant` → `losing_creature_type_removes_the_combatant_role`; `layers::printed_creature_grant_is_not_doubled_by_the_fold` (counts the role conferral now, via the re-spelled helper `conferred_may_attack_rows` → `conferred_combatant_rows`). Plus 3 shared creature fixtures re-spelled to confer the role — `legal::creature_typedef` and the `combatant_creature_def` twins in `tests/damage_result_rules.rs` and `tests/replace_registry.rs` — whose consumers (6, 4 and 7 call sites) assert unchanged outcomes.
- ignored added: 0 (the 6 ignored are pre-existing, each with its blocker named)
- added: 6 — `legal::a_creature_barred_from_both_actions_still_plays_the_role`, `legal::defender_bars_the_attack_but_not_the_role`, `legal::a_subtype_conferral_gives_a_noncreature_the_role`, `legal::a_bare_combat_permission_confers_no_role`, `lowering::property::the_v1_combat_bundle_collapses_to_the_combatant_role`, `lowering::property::a_lone_block_grant_is_left_as_it_is`
- removed: 0

### Acceptance

- **A Creature restricted from attacking, blocking, or both remains a Combatant** — `legal::a_creature_forbidden_to_attack_still_plays_the_role` (attack half) and `legal::a_creature_barred_from_both_actions_still_plays_the_role` (both halves: the role holds while `legal_attackers` and `legal_blockers` both exclude it).
- **A Combatant with defender remains a Combatant** — `legal::defender_bars_the_attack_but_not_the_role`: the keyword's `Cant(Attack)` row ([CR#702.3b]) removes it from `legal_attackers`, the role and the derived block permission stand.
- **Gaining or losing the Creature type gains or loses the role, unless another source confers it** — `layers::animated_enchantment_can_attack` (gaining), `layers::losing_creature_type_removes_the_combatant_role` (losing: 0 conferrals off the derived card types, no longer a legal attacker), `legal::a_subtype_conferral_gives_a_noncreature_the_role` (another source: an Artifact with a Vehicle subtype conferring the role attacks and blocks with no creature type).
- **A fresh noncreature with `{T}: Draw a card` and only the scoped block permission may activate it** — `legal::a_bare_combat_permission_confers_no_role`: a summoning-sick artifact with a `May(Block)` row and a `{T}` ability is `blockable`, is NOT a Combatant, and `cant_activate(…, tap = true)` is false; the `May(Attack)` mirror in the same test is likewise no Combatant.
- **Attack, block, fight, damage and tap/untap-activation tests read the explicit role** — the three creature fixtures confer `Role(Combatant)` and nothing else, so every combat, damage-marking and tap-gate test in `legal.rs`, `tests/damage_result_rules.rs`, `tests/replace_registry.rs` and `tests/layers.rs` is now driven by the role. Fight is the exception recorded under **Not done**: its guard is v1 card data.

### Deviations and additions

- **`StaticSpec::Role`, not a new `Property` flavor.** The ticket asks for an
  ordinary conferred rules property; `Property::Static` already is one, and a
  `StaticSpec` variant inside it inherits the `Conditionally` gate the
  "while the Object is a Permanent" clause needs and the static walk every
  consumer already runs. No new Property flavor, no new walker.
- **`who: Predicate` on the row, read only off the bearer's own registry
  conferrals.** The row names its subject like every other deontic row, but
  `is_combatant` consults the object's derived types and subtypes rather than
  scanning the battlefield: a source that makes another permanent a Combatant
  does it by giving it the creature type ([CR#613.1d] layer 4), and the
  registry conferral is then the one channel. This also keeps the read O(1)
  walks, since `for_each_static` performs it on every visit.
- **The lowering bundle collapse** is an addition the ticket does not spell
  out; without it the live `plugins/builtin` registry — the only authoring path
  the engine loads today — could not express the role at all, since
  `deckmaste_semantics` is out of scope. It is content-directed (the pair of
  self grants), fires on no other conferral, and is exercised in both
  directions by the two new lowering tests.
- `deckmaste_engine/src/step/combat.rs`, `legal.rs`'s attack/block collector
  docs and the `Creature.ron` stub comment were reworded where they described
  the old `May(Attack)`-as-witness model.

### STOPs

**One.** The ticket asks that "eligibility for creature-only combat and fight
rules" derive from the role; the fight guard ([CR#701.14b]) and the creature
SBAs ([CR#704.5f,704.5g]) are v1 card data whose PREDICATE vocabulary only
`deckmaste_semantics` can extend, and the 2026-09-04 scope ruling puts that
crate off limits. The ticket and the ruling cannot both be satisfied this
round. Resolution: the ruling wins — the guards keep their `Type(Creature)`
predicate, no core predicate atom is added that nothing can author, and the
obligation is recorded under **Not done** for the round that gives the
predicate vocabulary a role atom (at the latest, cutover, when v2 authors both
sides). Every engine-side read named by the ticket does derive from the role.
