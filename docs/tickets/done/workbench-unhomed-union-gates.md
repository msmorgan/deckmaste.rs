---
needs: [workbench-join-shape-flat-or-pair]
---
# Answer the two union facts that survive under neither design

Two facts survive under **neither** the marked-construction design nor a naive
lattice, and any lattice design must answer both explicitly before it can claim
to reproduce the corpus. They are unrelated in mechanism but identical in status:
both are load-bearing, both are measured, and neither has a home yet.

Authority:
[The kind index joins; union marking is spelling](../../decisions/kind-index-joins-union-marking-is-spelling.md).

## 1. The shared demonstrative surface

`bindFor` gives the class word [CR#115.4] and the union head `KindJoin` **one
payload**, `Payload.UnionP`, which carries nothing (`bindingZone = Nothing`,
`bindingTy = Nothing`) on purpose: the mention is marked, not merely empty. The
measurement behind it, over the 48 supported anaphor readbacks:

- **33 of 33** union-headed antecedents echo their antecedent's kind pair
  exactly, no crossing in either direction.
- **15 of 15** with no pair to echo write the generic "permanent or player".
- **All 43** — the 33 plus the 10 class-word antecedents — use the same
  demonstrative, and **no card distinguishes the two antecedents**.

A flat join has no pair and cannot produce 33 of the 48 at all. A powerset join
gives the class word the set `{Creature, Player, Planeswalker, Battle}` and the
union head `{Permanent, Player}` — two different sets, hence two different
echoes — where the corpus has one. So a powerset join needs a **collapse rule**
(set → demonstrative words) mapping a four-element set and several two-element
sets onto one generic surface while preserving the 33 exact echoes. That rule is
not part of any lattice; it is a spelling table, and which words a demonstrative
spells is a spelling decision.

The risk here is **not** overgeneration — it is undergeneration and
mis-generation. Deliver the table, not a refusal.

## 2. `AnyTargetLone`'s modifier discipline

Once the class word and the union head share a kind, `AnyTargetLone` has **no
index to key on**, and "any red target", "any target you control", "any target
other than that creature" all become spellable at **zero corpus lines**. This is
not honestly tolerable as overgeneration: the discipline is closed, carries
`badDestroyAnyTarget`-class pins, and dropping it opens an unbounded modifier
grid rather than a handful of named cells. The two heads genuinely differ here —
the union head carries a modifier in 11 of its 326 occurrences and the class word
in none — and a shared kind erases the distinction the discipline is written
against.

The obvious answer is a lexical marker, an `IsClassWord` witness — the marked
construction re-created inside the lattice as a gate. Take that or beat it, and
say which.

Gates that must survive with it: `negatable AnyTarget = False`,
`AnyTargetAtCount`, and `anyTargetFree` with its five siblings
(`nounAnyTargetFree`, `zoneAnyTargetFree`, `amtAnyTargetFree`,
`complementAnyTargetFree`, `nameSrcAnyTargetFree`), every one of which exists
solely to find this word in a subterm.

## 3. Ledger from `workbench-join-shape-flat-or-pair`

- `Object \/ Ability` is `joinable = False` as **unsurveyed**, not refused, and
  carries no pin: "target spell or ability" is attested (155 oracle lines) and
  would be that join once `Targetable` admits `Ability`. Measure and admit it
  here, or record why it is not a kind join.
- `bareLookbackOk _ _ = True` (`Events.idr`) silently admits `ObjectOrPlayer`
  for every event; unreachable today because `lookbackSubjectOk _ ObjectOrPlayer
  = False`, but a joined-kind lookback subject must not inherit that silent yes.

## Consumption boundary

The collapse table is spelling-boundary content and lands with
`crates/deckmaste_english_v2/src/constructions.rs`; the lexical marker is a
semantics gate in `idris/src/Experimental.idr` and
`idris/src/Experimental/Words.idr`. Coordinate with
`workbench-union-gate-spelling-rehome`, which owns the verbatim re-homing of the
measured rows — this ticket owns the two that no measurement alone answers.

## Acceptance

- All 48 readbacks are reproducible from the delivered rule: the 33 exact echoes
  and the 15 generic ones, with no card distinguishing the class-word and
  union-headed antecedents.
- The modifier discipline still refuses everything it refuses today, with its
  pins intact.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

## As-landed

Same situation as the sibling ticket: the drain landed first.
`AnyTarget` / `KindJoin` / `YouAnd` left the core in
`workbench-union-family-macros` for `Macros.anyTarget` / `kindJoin` / `youAnd`
/ `thatJoin` at kind `Object \/ Player`, and the docstrings carrying the
measurements were deleted with them. Under the doctrine — the semantics admits
anything the rules make CR-meaningful, attestation lives at the spelling
boundary (`docs/memory/rulings/measurements-live-in-pins.md`) — §1's table is
spelling-boundary content and was the only thing left to build.

**§1, the shared demonstrative surface — DELIVERED.** The collapse table lands
as §4 of `crates/deckmaste_english_v2/docs/union-spellings.md`: antecedent
spelling → demonstrative words, ten rows, with the four rules it encodes. Its
substance, re-measured 2026-08-22 (this ticket's figures → mine):

- 48 readbacks → **68**; 33 union-headed → **51**; 15 with no pair → **17**;
  **10** class-word antecedents → **10**, reproducing exactly.
- **The generic surface is shared and nothing distinguishes the two
  antecedents.** 8 class-word lines and 28 permanent-headed lines write the
  identical "that permanent or player". The half-naming form does not
  separate them either: Chain Lightning and Chain of Plasma announce "any
  target" and read it back as "that player or that **permanent's**
  controller", the same construction a permanent-pair head uses.
- **The echo is exact in both directions.** A planeswalker head with a generic
  readback: 0 lines. A permanent head with a planeswalker-naming readback: 0
  lines. No crossing.
- Naming a half is always a coordination of two demonstratives reaching the
  object half through its controller (23 lines); not one of the 68 is a bare
  "that player"/"that permanent" over a union antecedent.
- One demonstrative word throughout; the single "those" line is ordinary
  number agreement.

The table is delivered as a rule, not a refusal: it fixes what a powerset
join's `{creature, player, planeswalker, battle}` and `{permanent, player}`
must both spell, so the risk this ticket named — undergeneration and
mis-generation — is answered.

**§2, `IsClassWord` / modifier discipline — DROPPED by conductor ruling.**
Modifier discipline is corpus knowledge, a spelling fact; it gets no semantics
gate and no lexical marker. Recorded spelling-side instead, in §1 and §3 of
the same file: the union head bears a modifier in **22 of 319** lines (this
ticket's figure: 11 of 326), the class word in **0** lines of the kind that
matters — no pre-nominal modifier, no control restriction, so "any red
target" and "any target you control" are both unattested. This ticket's flat
"the class word in none" is not quite right and the file says so: four lines
carry a restrictive relative clause on the class word ("any target that isn't
a Dragon", "…that isn't a Dinosaur", "…that isn't a commander", "…that was
dealt damage this turn"), and "any other target" (16 lines) is the complement
rather than a modifier. The gates this section listed to preserve —
`anyTargetFree` and its five siblings, `AnyTargetLone`, `AnyTargetAtCount`,
`negatable AnyTarget = False` — were retired in
`workbench-union-family-macros` as vacuous or unpinnable once the kind index
did the refusing; nothing here reinstates them.

**§3, the ledger — RESOLVED, and not by this ticket.**
`docs/tickets/done/workbench-join-is-a-constructor.md` settled both rows:
`Object \/ Ability` is admitted, witnessed by `spellOrAbilityJoin` in ProofsG
(Bolt Bend's "target spell or ability") with `abilityUnderSpellOrAbility`
reading the `Ability` half back through `kindLteJoinR`; and
`lookbackSubjectOk (a \/ b)` is now both halves rather than a silent yes, so
the joined-kind lookback subject inherits nothing.

**No Idris change in this ticket, and no lexical marker minted.** The
acceptance clause "the modifier discipline still refuses everything it refuses
today, with its pins intact" is superseded: those pins were retired by the
ruling that a count is never a refusal, and the discipline is now declared at
the spelling boundary instead.

**Gates.** Rust-side only, as the deliverable is: `cargo fmt`; `cargo clippy
-p deckmaste_english_v2` 0 warnings; `cargo test -p deckmaste_english_v2`
green; `cargo xtask cite check --list-noncompliant` empty; `cite check` 0
stale; `cite bless`; `cite audit --diff` read. Not committed.
