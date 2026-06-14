---
needs: []
---
Generator task. Parse durational pump effects — "<subject> gets ±N/±N [and
gain(s) <kw…>] until end of turn" (and keyword-only "<subject> gain(s) <kw> until
end of turn") — into a one-shot `Continuously(effect: Modify(of: <scope>, changes:
[...]), duration: FixedUntil(EndOfTurn))` [CR#611.2] in the shared effect grammar
(`effect.rs`), so spells/triggered/activated frames all frame them. Subjects: team
("Creatures you control" → `Matching(<filter>)` via filter.rs), self ("~" →
`Of(This)`), single target ("target creature" → `Of(Target(0))` +
`TargetOne(<filter>)`). Extracts the shared ±N/±N + keyword-grant + subject/scope
grammar out of `static_ability` into `parsers/modify.rs` (static anthems keep
wrapping in `Static`; durational pumps wrap in `Continuously`). Reuses the proven
`Continuously` encoding (the existing `PumpThisUntilEot` effect macro). Emits
`changes: [AddPower(Literal(N)), AddToughness(Literal(N))]` inline (consistent
with `PumpThisUntilEot`'s own body) — an `AddPowerToughness` macro is NOT viable:
`Modification` is not a macro kind and macros can't splice into a `changes` list,
so it couldn't compose with the keyword grant Overrun needs. (A core
`Modification::AddPowerToughness(Count, Count)` enum variant could bundle them,
but that's an engine change, not a macro; deferred unless asked.) Overrun
("Creatures you control get +3/+3 and gain trample
until end of turn") is the worked example; also unblocks combat tricks (Giant
Growth), team pumps, and firebreathing. Feeds canon-goblins-elves; part of
tui-client.
