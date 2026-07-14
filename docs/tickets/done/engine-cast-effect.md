---
needs: []
---
# engine-cast-effect — `Cast(Reference)` cast-as-effect primitive + Chandra graduation

A resolution-time "you may cast that card" is not expressible: casting is only ever a
permission-gated *player action* (`DeonticAction::Cast`), applied as a lingering
continuous permission. There is no `PlayerAction::Cast` / cast `OneShotEffect`. This
ticket adds the missing primitive and exercises it by graduating Chandra, Torch of
Defiance (whose `+1` is the canonical impulse: "Exile the top card of your library.
You may cast that card. If you don't, Chandra deals 2 damage to each opponent.").

## Verified context

- `OneShotEffect::May { effect, if_did, if_not }` — `effect.rs:243`; YesNo drives it at
  `decide.rs:1294` (yes → `effect` [+ `if_did`]; no → `if_not`). This is the else-branch
  for "if you don't" — already wired. **Not the gap.**
- No `Cast` in `deckmaste_core/src/action.rs` or `effect.rs`. Casting is initiated only
  from `legal_actions`, gated by `DeonticAction::Cast { what, by, from, window, cost, tag }`
  (`deontic.rs:206`), collected as `MayCastRow` (`legal.rs:1213`). **The gap.**
- Loyalty activation (costs + rules + enters-with + zero-death) is fully done — Jace
  Beleren (`plugins/canon/cards/Jace Beleren.ron`) proves it end-to-end.
- Chandra's other three abilities are representable today: `+1` `AddMana(RR)`,
  `−3` damage to target creature, `−7` `GetEmblem(Vec<Ability>)` (`action.rs:316`, just
  landed).

## Design (settled: General `Cast(Reference)` verb)

Add a cast-as-effect primitive so `Cast(It)` can sit inside `May.effect`. Surface it the
way existing player-agent effects are surfaced (follow the `Draw`/`Mills`/`Move` idiom —
`PlayerAction::Cast(Reference)` bridged to an effect, or `OneShotEffect::Cast`, whichever
matches the codebase's convention; honor the card-surface contract — positional, macro
atoms, no stringly binders).

**Semantics** (`[CR#601.2,608.2]` — casting during resolution; VERIFY the exact subrule
for "an effect instructs a player to cast; the spell is cast and remains on the stack"
and cite it):
- `Cast(Reference)` resolves the reference to a target object (for Chandra, the just-
  exiled card, bound as "that card"/`It`) and **initiates casting of it** — reusing the
  existing casting entry point (the same flow `legal.rs`/`cast.rs` drive), which runs the
  `[CR#601.2]` subsequence (mode/target/cost decisions via the normal
  `PendingDecision`/continuation machinery). The effect GRANTS the permission, so it
  bypasses the `DeonticAction::Cast` timing/permission gate for this one card.
- The cast spell goes on the stack; the resolving ability finishes; normal priority
  follows (the cast spell is above). Do NOT resolve the cast spell inline.
- **Castability gate:** offer the `May` "yes" branch only when a legal, payable cast of
  the referenced card exists. If none exists, the player cannot choose "yes" → the
  `if_not` branch runs. (So "you may cast … if you don't" stays faithful even when the
  card is uncastable.) Document the yes-then-abort edge (player says yes, backs out mid-
  cast): acceptable v1 = treat commitment at "yes"; note as a follow-up if it needs the
  `if_not` path.
- Engine never panics on a bad/absent reference — `Cast` of a missing object fizzles
  (per engine-never-crashes ruling).

## Chandra graduation

Graduate `plugins/wizards/cards/Chandra, Torch of Defiance.ron.todo` → `plugins/canon/`.
Encode all four abilities (mirror Jace Beleren's `LoyaltyPlus`/`LoyaltyMinus` style):
- `[+1]` mana: `LoyaltyPlus(n: 1, effect: <add {R}{R}>)` — `AddMana`.
- `[+1]` impulse: `LoyaltyPlus(n: 1, effect: <exile top of your library, bind it; then
  May { effect: Cast(that card), if_not: <Chandra deals 2 to each opponent> }>)`. Exile-
  top + anaphor for "that card" + damage-each-opponent (cf. Flame Rift's each-opponent)
  all exist; compose them.
- `[−3]`: `LoyaltyMinus(n: 3, effect: Targeted([TargetOne(Creature)], <deal 4 to it>))`.
- `[−7]`: `LoyaltyMinus(n: 7, effect: GetEmblem([ <triggered: whenever you cast a spell,
  this emblem deals 5 damage to any target> ]))`.

## Acceptance (TDD)

1. **Cast primitive, decline:** activate Chandra `+1` (impulse); top card known; decline
   the cast → assert Chandra dealt 2 to each opponent AND the card stayed in exile.
2. **Cast primitive, accept:** same setup; accept → assert the exiled card is now on the
   stack (being cast) AND no damage was dealt.
3. **Uncastable:** top card has no legal/payable cast → the offer resolves to `if_not`
   (2 damage), no panic.
4. **Chandra full card:** loads through the real plugin loader (like Jace Beleren);
   enters at loyalty 4; each of the four abilities activates at sorcery speed / shared
   once-per-turn; `−7` mints a functioning emblem; `−3` kills a 4-toughness creature.
5. `cargo test -p deckmaste_engine -p deckmaste_cards -p deckmaste_core` green;
   `no_dead_grammar` passes (drop `Cast` from any deferred list; add a render arm — new
   grammar needs bidirectional parse/render).
6. CR citations: `cite check --list-noncompliant` empty, `cite check` 0 stale, `bless`
   new rules + `cite audit --diff` read each (`[CR#601.2,608.2,606.3]`, plus the exact
   cast-during-resolution subrule you verify).

## Cleanup

Fix the stale comment at `crates/deckmaste_engine/src/activate.rs:69` ("loyalty costs
wait for core-loyalty-costs") — that work is done; the comment is false.
