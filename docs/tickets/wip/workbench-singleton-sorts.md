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

## Landing record

As landed:

- `LoseCause` — kept; `[CR#704.5a..704.5c]` on the declaration.
- `Causer` — kept; `[CR#614.16]` on the declaration (the rule that names "an effect").
- `TurnPoint` — kept, escape hatch taken: `[CR#506.7]` opens the axis by
  enumerating "[a particular point in the combat phase]" — attackers declared,
  blockers declared, the combat damage step, the end of combat step, the combat
  phase, combat — and `[CR#506.7g]` extends it to "Activate only" limits.
- `Ordinal` — kept, escape hatch taken: `[CR#401.7]` reads "Nth" with N a
  variable. The singleton premise did not hold: `Nth : (n : Nat) -> {auto 0 nz :
  IsSucc n}` has one constructor but unboundedly many inhabitants (`Nth 1`,
  `Nth 2`, …, read at `NthCastBy`, `NthOccurrence`, `ByNthKeyword`,
  `LibraryAt`, the `nthFromTop` macros); it is a positivity refinement of the
  same kind as the out-of-scope proof wrappers, and the `badZerothFromTop` pin
  is its `IsSucc` gate. Collapsing would spread that gate over five consumer
  signatures and a `Maybe` slot for no shape gain.

Gates: `./scripts/build` 23/23, no Warning lines; `cite check` 0 stale;
`cite bless` registered [CR#506.7] (read against its claim); `cite audit --diff`
4 sites, all on-topic; `--list-noncompliant` reports one pre-existing non-compliant string in
`docs/tickets/done/workbench-facts-tables.md:95`, not this round's.

Deviations and additions: none. Tests/pins restored 0, re-spelled 0, ignored 0,
added 0, removed 0. No STOP taken.
