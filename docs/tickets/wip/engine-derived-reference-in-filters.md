---
needs: []
---
**A derived reference inside a predicate (`Ref(ControllerOf(…))`) trips the
frameless matcher's invariant assert.** Observed while writing
`core-regions-witness-fixtures`. Standard constraints apply.

## The gap

`crates/deckmaste_engine/src/target.rs`'s `matches_with_activation` handles
`Predicate::Ref(Reference::Reg(_))` and the `This`/`You` anchors, then ends in

```rust
Predicate::Ref(r) => { debug_assert!(false, "engine invariant violated: …"); false }
```

Every DERIVED reference lands there: `ControllerOf`, `OwnerOf`,
`AttachHostOf`, `Coalesce`, `Single`. The ADR keeps those nestable as pure
expressions evaluated at the read site (law 4), so a filter is a legitimate
read site — but the matcher holds only a watcher, never a resolving frame, and
in a test build the assert aborts rather than degrading.

Two witness fixtures reached it and were re-spelled around it:
`ControlledBy(Ref(ControllerOf(Target(0))))` became
`ControlledBy(Controls(Ref(Target(0))))`, and a history read's
`Damage(to: Ref(ControllerOf(It)))` became `Damage(to: Controls(Ref(It)))`.
Both alternatives are faithful, so nothing is blocked today — but the natural
spelling of "controlled by the same player as X" aborts the process, and the
relation-predicate detour is not discoverable.

## Acceptance

A filter naming a derived reference resolves it (threading the frame the way
`resolve_count` already does) or is refused at LOAD with a diagnostic naming
the card. A `debug_assert!(false)` reachable from authored card text is the
thing to remove.
