---
needs: [engine-lki-robustness]
---
**Engine: process Damage into its Damage Results as events, so damage-caused
life loss and lifelink life gain are `LifeLost`/`LifeGained` events that
triggers and replacements see.** This is the live-engine half of what
`lean-v3-damage-composition` plans for the `semantics-v3` spine (it superseded
`semantics-v3-damage-results`, owner ruling 2026-10-22); it does not wait on
that port.

Provenance: `docs/evidence/mtg-rules-review-2026-10-21/departures-engine.md`
#1, #10, #16.

## What is wrong at head

`crates/deckmaste_engine/src/step/combat.rs`, `impl EventApply for DamageDealt`
(every damage, not only combat damage):

- `:32-34` — `// [CR#120.3a]: damage to a player is life loss.` then
  `g.player_mut(p).life -= …`. No `LifeLost` event.
- `:88-99` — lifelink: `g.player_mut(controller).life += …`. No `LifeGained`
  event. `apply` returns `None`.

`EventFilter::LifeLost`/`LifeGained` (`eval.rs:869-878`) match only
`FactKind::LifeLost`/`LifeGained`, built only from `GameEvent::LifeLost`/
`LifeGained`, which only the resolve and payment paths emit. So:

- "Whenever an opponent loses life" never sees damage; "whenever you gain life"
  (Ajani's Pridemate) never sees lifelink.
- "If you would gain life" replacements (Boon Reflection) never apply to
  lifelink, and "if a source would cause … to lose life" replacements never
  apply to damage.

Under [CR#120.3a] damage to a player "causes that player to lose that much life";
[CR#120.4c] processes dealt damage "into its results, as modified by
replacement effects that interact with those results (such as life loss or
counters)". [CR#119.9] and [CR#119.10] read gain-life triggers and replacements
as "a source causes". [CR#702.15e]: multiple lifelink sources dealing damage at
once "cause separate life gain events".

Infect and wither (`plugins/builtin/macros/keyword/Infect.ron`, `Wither.ron`)
are `Static(Replacement(Instead(would: Damage(...), instead: PutCounters(...))))`;
`replace_registry.rs` notes an `Instead` "replaces the intent away — it never
reaches the apply funnel". So `DamageDealt::apply` never runs for them: the
damage-dealt event never happens, damage-dealt triggers miss it, lifelink on an
infect or wither source gains nothing, and the counters become an ordinary
[CR#616.1] replacement the affected player can order around a damage doubler.
Reachable: 55 live cards carry infect or wither; Flensermite (infect and
lifelink) gains no life.
[CR#120.4b]: damage is dealt "as modified by replacement and prevention effects
… Abilities that trigger when damage is dealt trigger now". [CR#120.3d], [CR#702.90b] and
[CR#702.90c] change only the result ("Rather …"), and [CR#603.2g] says a replaced
event triggers nothing.

The lifelink check reads the source only if it is still in the store
(`g.objects.get(source).is_some_and(...)`, `combat.rs:93`), and
`DamageMark.source_abilities` is empty "when the source had already left"
(`combat.rs:52-63`). A departed source ("When this dies, it deals 2 damage to any
target" on a lifelink or deathtouch creature) loses both keywords. [CR#702.15c]
and [CR#702.2e]: the source's last known information decides whether it had
lifelink or deathtouch.

## Change

1. **Results as events.** After `DamageDealt` applies (the damage is dealt and
   its damage-dealt triggers see it, [CR#120.4b]), schedule its Damage Results
   ([CR#120.4c]) as events in the same simultaneous batch: damage to a player
   emits `LifeLost { player, amount, cause: <damage source> }`; each lifelink
   source emits its own `LifeGained { player: <source's controller, or owner
   if none>, amount, cause: <source> }` ([CR#702.15b,702.15e]). Route both
   through the replacement pipeline and the trigger scan, and delete the direct
   `life` mutations in `combat.rs`.
2. **Infect and wither as result substitutions.** Re-home them as Damage Result
   rules applied at step 1: wither and infect replace the marking result with
   -1/-1 counters on a creature ([CR#120.3d,702.80a,702.90c]); infect replaces a
   player's life loss with poison counters ([CR#120.3b,702.90b]). The
   `DamageDealt` event still occurs and lifelink still applies (the
   [CR#120.4d] example). Delete the `Instead(would: Damage …)` statics. Shape
   these rows the way `lean-v3-damage-composition` records them (wither and
   infect replace the marking or life-loss result; lifelink and toxic are
   additional results) so the later port is a re-spelling, not a redesign.
3. **Departed sources.** Read the source's lifelink, deathtouch, wither and
   infect from the derived view while it is live, and from its LKI snapshot
   when it has left the zone it was expected in 
   ([CR#702.15c,702.2e,702.80b,702.90d]); fill `DamageMark.source_abilities` from the same read. This needs
   the derived snapshot from `engine-lki-robustness`. The lifelink payee is the
   source's derived controller (see `engine-derived-controller-reads`).

## Tests

- Combat damage to a player fires a "whenever an opponent loses life" trigger.
- Two lifelink creatures dealing combat damage at once trigger a gain-life
  watcher twice ([CR#702.15e] names Ajani's Pridemate).
- A "if you would gain life, gain twice that much" replacement doubles lifelink.
- A wither + lifelink creature deals damage: the recipient gets -1/-1 counters,
  the controller gains life, and a damage-dealt trigger fires ([CR#120.4d]).
- A lifelink creature's "When this dies, it deals 2 damage" trigger gains its
  controller 2 life; a deathtouch one destroys the creature it damages.
