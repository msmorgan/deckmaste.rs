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
