---
needs: []
---
# engine-storm — Storm keyword + the `Before(Reference)` count primitive

Storm `[CR#702.40]` currently doesn't exist as a keyword, and the count it needs
("each OTHER spell cast BEFORE it this turn") has no primitive. Census row "Storm"
(87 cards) is cross-referenced to `engine-copy-spells` (done: stack copies exist);
this ticket adds the count + keyword on top of that.

## The rule (verified)

`[CR#702.40a]`: "Storm is a triggered ability that functions on the stack. 'Storm'
means 'When you cast this spell, copy it for each other spell that was cast before it
this turn. If the spell has any targets, you may choose new targets for any of the
copies.'"

Why a plain `EventCount(Cast, ThisTurn)` at resolve time is WRONG:
- The storm spell's own `SpellCast` is recorded at cast; the trigger resolves after —
  so the storm spell is ALWAYS in the this-turn tally. "other" excludes it.
- Spells cast in RESPONSE (after the storm spell, before the trigger resolves — e.g.
  cast storm spell, hold priority, cast another) are in the tally but were NOT "cast
  before it". `[CR#603.3]`: the storm trigger is placed topmost the next time a player
  would get priority, so a held-priority spell lands above it yet is cast after.
Count is fixed by CAST ORDER relative to the storm spell, not by resolution time.

## Design (settled)

**`EventFilter::Before(Reference)`** — new combinator in `deckmaste_core/src/event.rs`
(the `EventFilter` enum, ~`:257`). Matches history events that occurred before the
referenced object's OWN cast event this window. For storm, `Reference = This` (the
storm spell). `Before(This)` naturally drops This's own cast (not "before" itself), so
it delivers the "other … before it" semantics directly.

Storm count = `EventCount(AllOf[Cast{who:Any,what:Any}, Before(This)], ThisTurn)`.

**Implement `Before` via cast order, ideally a monotonic per-turn cast index.** Each
`SpellCast` history fact already carries turn info; give casts a per-turn ordinal (or
read positional order within the turn's history lane) so `Before(This)` is
`ordinal(cast) < ordinal(This.cast)`, not an O(n) rescan. This aligns with
`engine-history-tallies-cache` (planned) — if that index doesn't exist yet, add the
minimal version here and note it. Eval lives in `deckmaste_engine/src/resolve/count.rs`
(the `EventCount`/`performed_matches` path around `:669`–`710`, where the current
"−1 self-exclusion deferred" comment sits — REMOVE that deferral).

**Storm keyword** — add `Storm` to `KeywordAbility` (`deckmaste_core/src/keyword.rs`,
~`:148`) as a **Composite triggered** keyword. Expansion:
`When(Cast(This)) → Repeat(<storm count above>, CopySpell(This))` with the optional
"choose new targets for the copies" rider. Uses the existing `Repeat(Count,
Box<OneShotEffect>)` (`effect.rs:179`) and `Action::CopySpell(Reference)`
(`action.rs:326`). Add the keyword macro `plugins/builtin/macros/keyword/Storm.ron`
(mirror an existing composite-triggered keyword macro, e.g. a `When(Cast…)` one).
Remove `plugins/wizards/keyword_abilities/Storm.todo.ron` once real.

## Compiled card (full slice)

Graduate ONE `.ron.todo` Storm card — pick the simplest self-copying one. Candidates
(all `plugins/wizards/*`, `Keyword(Storm)`): Scattershot (deal 1 to target, storm),
Hindering Touch (counter target spell, storm), Weather the Storm (lifegain, storm),
Dragonstorm (search — hardest, avoid). Prefer Scattershot/Weather the Storm.

## Acceptance

1. TDD engine test: cast A, B this turn, then cast storm spell S; assert the storm
   trigger produces exactly **2** copies (not 3, not 1). `[CR#702.40a]`
2. Held-priority test: after S, cast T (a fourth spell) before the storm trigger
   resolves; assert copy count is still **2** (T cast after S → excluded). `[CR#603.3]`
3. `Before(This)` round-trips through parse/render (it's new grammar → RENDER arm).
4. Storm keyword parses from a card and renders back; keyword macro round-trips.
5. `cargo test -p deckmaste_engine -p deckmaste_cards -p deckmaste_core` green;
   `no_dead_grammar` passes (drop any now-live deferrals: `Before`, `Storm`).
6. CR citations: `cite check --list-noncompliant` empty, `cite check` 0 stale, `bless`
   new rules + audit (`[CR#702.40a,603.3]`).
7. One compiled Storm card in the corpus (or documented deferral + reason).
