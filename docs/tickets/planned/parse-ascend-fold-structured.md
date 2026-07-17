---
needs: []
---
**Spell-side Ascend is folded by string-splicing already-rendered RON, and the
gate it splices in is a hand-copied duplicate of the Ascend macro's expansion.**
Two band-aids in one function; the codebase's macro system exists to make both
unnecessary.

`crates/deckmaste_migrations/src/resolve.rs:82-111`, `fold_spell_ascend`, runs as
a post-render pass over the flat `Spell(...)` string: it `find("effect: ")`s,
slices the string into `head` + `effect_val` on the assumption the ability starts
`Spell(` and ends `)`, and splices the two back together around an inlined gate:

```rust
let head = &s[..idx + "effect: ".len()];
let effect_val = &s[idx + "effect: ".len()..s.len() - 1];
*ability = TodoAbility::Parsed(format!(
    "{head}Sequentially([If(condition: {ASCEND_GATE}, then: GetDesignation(\"CitysBlessing\")), {effect_val}]))"
));
```

Same defect class as [[parse-positional-target-reads]]'s `IT_READ` and the
`fold_spell_ascend`-cited pre-pass in `modal.rs`: patch the emitted string instead
of building the structure. It is brittle by construction — the slice math is
pinned to `Spell(` … `)` and would mis-splice any frame whose rendering differs.

Worse, the gate itself is duplicated. `resolve.rs:74-77`:

```rust
/// The Ascend gate ([CR#702.131a,702.131b]) — KEEP IN SYNC with
/// `plugins/builtin/macros/keyword/Ascend.ron`.
const ASCEND_GATE: &str = "And([Compare(CountOf(Objects(And([InZone(Battlefield), \
ControlledBy(Ref(You))]))), AtLeast, 10), Not(Matches(You, Designated(\"CitysBlessing\")))])";
```

A hand-maintained copy of a macro body, with a "KEEP IN SYNC" comment and a
drift-guard test (`resolve.rs:481-524`) that parses `ASCEND_GATE` and asserts it
matches the canonical gate. The guard is proof the duplication is a known hazard —
the macro system is supposed to eliminate exactly this.

## Fix

Two independent corrections:

1. **Fold structurally, not textually.** Do the Ascend wrap at the
   `ParsedEffect`/structured level before the frame is rendered — wrap the effect
   value in the `Sequentially([If(...), <effect>])` shell as data, so no `find`
   /slice on rendered RON. (This dovetails with [[parse-targeted-wrapper-dedup]];
   a structured effect makes the wrap trivial.)
2. **Source the gate from the macro, not a const.** Reference the Ascend macro's
   expansion (or emit the macro invocation and let expansion supply the gate)
   instead of the inlined `ASCEND_GATE` string. One source of the City's-Blessing
   gate, no KEEP-IN-SYNC comment, no drift-guard test needed.

Verify: an Ascend spell (e.g. a `Keyword(Ascend)` + spell-effect fixture) still
folds to `Sequentially([If(condition: <gate>, then: GetDesignation("CitysBlessing")),
<effect>])` and drops the keyword; edit the Ascend macro and confirm the folded
gate follows automatically (the drift-guard test can be deleted); `cargo xtask
generate plugins/wizards` re-emits Ascend spells unchanged; `cargo test
--workspace` green.
