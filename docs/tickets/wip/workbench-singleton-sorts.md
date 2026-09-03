---
needs: []
---
**Keep `LoseCause` and `Causer`; collapse `TurnPoint` and `Ordinal` unless a
rule opens them.** Ruling 2026-09-02 on audit item R7. Authority for what each
slot fixes and why: `docs/idris-workbench-closure-tables.md §3` (its anchors
are stale — see `workbench-table-hygiene`; its content is not).

Four value sorts have one inhabitant each and no recorded decision:

- `Words.LoseCause = ZeroOrLessLife:1555` (sole consumer `NoLossFrom:411`) —
  **keep.** [CR#704.5a..704.5c] lists distinct loss causes, so the axis is
  rule-mandated even though only one is modelled today.
- `Words.Causer = AnEffect:703` — **keep.** Already re-probed and kept
  (closure tables §3).
- `Words.TurnPoint = AttackersDeclared:4006` (sole consumer
  `Timing.BeforePoint:679`) — **collapse** into its consumer.
- `Words.Ordinal:770` (`Nth`) — **collapse**.

The escape hatch on the last two: if the implementer can name the rule that
opens the axis — as [CR#107.1a] mandates `RoundMode` — then keep it and cite
that rule on the declaration instead of collapsing. Naming no rule means
collapse; a general sense that "more will come" is not a rule.

Out of scope: the proof-wrapper singletons `OnBattlefield`, `OnStack`,
`AtLeastTwo`, `ChoiceStands`, `DeedParticipant`, `MarkingOk` and the `*Laws`
bundles. Those are `data`-for-inference, not value sorts, and are not in
question.

Size: S.

Done when: build is 23/23; `TurnPoint` and `Ordinal` are either gone (their
consumers taking the value directly) or carry a `[CR#…]` citation naming the
rule that opens them; `LoseCause` and `Causer` carry their citations
([CR#704.5a..704.5c] on the first) on the declaration; `cargo xtask cite check
--list-noncompliant` is empty and reports 0 stale. Standard constraints apply.
