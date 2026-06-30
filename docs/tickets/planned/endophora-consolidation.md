---
needs: [engine-anaphor-threading]
---
Rename the binding concept to **`Endophora`** and consolidate the `Frame`'s
scattered bound-reference state into one record.

**Why `Endophora` (not `Anaphora`):** what the record tracks is *text-internal*
reference in BOTH directions — anaphora (antecedent-then-pronoun, "choose a
creature; destroy it") AND cataphora (pronoun-then-antecedent, which the card-text
surface hits constantly: "deal 2 damage to **each creature**", "destroy **all
creatures**"). The cover term for intra-text reference in either direction is
**endophora**. The term also explains the record's membership: everything in it —
`It`, `That`, `Target(n)`, the event roles, the chosen value, `Allotment` — is
endophoric (introduced by an operator and bound for a sub-scope), whereas
`This`/`~`, `You`, `Opponent`, `DefendingPlayer` are **exophoric** (they refer to
the game *situation*, are never bound by an operator, and are always available),
which is exactly why those stay OUTSIDE the record.

**State now:** `engine-anaphor-threading` (#2) landed the binding state as *typed
but separate* `Frame` fields (`it: ItBinding`, `that: ThatBinding{cardinality,
kind}`, `allotment`). That already makes first-of-many unrepresentable — the
SAFETY is done. This ticket is the remaining fidelity/tidiness step.

**Do:**
- Introduce one `Endophora` record and fold the `Frame`'s endophoric fields into
  it — `it`/`that`/`allotment`/`targets`/`chosen`/`x` and the endophoric parts of
  the trigger `bindings`/`TriggerBindings` (the event roles). `This`/`You`/
  `Opponent`/`DefendingPlayer` stay as exophoric refs outside the record.
- Mirrors Idris's single `Bindings` record (renamed `Endophora` there in
  [[idris-oracle-and-naming]]). Shrinks the ~48 `Frame {…}` literal churn surface
  to one nested record.
- Keep `bindIt`/`bindThat`/`bindAllot`'s add-AND-clear semantics intact.

Standalone `Frame` refactor; do NOT change behavior. Full workspace gates.
