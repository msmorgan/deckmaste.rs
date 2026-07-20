---
needs: []
design: true
---
Party count (5 cards): the party size among creatures you control, used as a
threshold / scaling value in costs and effects.

## Needs a design pass before claiming (2026-07-09) — NOT a distinct-subtype count

A batch attempt verified against the CR that party is **not** a set-union of
distinct subtypes, so it does **not** fit the existing
`Count::CountDistinct(Characteristic, Countable)` primitive (`count.rs`,
evaluated via `distinct_keys` in `resolve/count.rs`). Encoding it as `CountDistinct`
would be provably wrong: the implementation must model the rule's actual
one-of-each assignment rather than substitute a convenient distinct-count approximation.

- `[CR#700.8]` — a party is *up to one each* of Cleric, Rogue, Warrior, Wizard
  you control; party size ∈ [0,4].
- `[CR#700.8b]` — a creature with multiple qualifying types is counted for only
  ONE role, chosen to maximize the result. This makes party a **maximum
  bipartite matching** (creatures ↔ 4 roles, each creature fills ≤1 role), cap 4.

Concrete divergence: a single "Cleric Wizard" contributes keys `{Cleric, Wizard}`
→ `CountDistinct` = 2, but party = 1 (one creature, one role). Multi-class party
creatures are real in canon, so the error is material. `CountOf(Objects(...))` is
also wrong (counts creatures: two Clerics → 2). **No existing `Count` computes
party.**

### Forks to settle in the design pass
- **A — encoding.** (A1) reuse `CountDistinct` + `Characteristic::PartyClasses` —
  *rejected on correctness* (ignores the `[CR#700.8b]` matching). (A2) new
  `Count::PartySize(Reference)` primitive; engine computes the max matching over
  the 4 classes among that player's creatures, cap 4 — correct, minimal-ish,
  must be wired into BOTH count evaluators (`resolve/count.rs` and `layer.rs`).
  (A3) a generalized "max distinct-role matching" Count taking an explicit
  role-set + object filter — more reusable (future "choose a party" / Stick
  Together, `[CR#700.8d]`), more surface.
- **B — where the 4 class names live.** Hardcoded const (precedent
  `BASIC_LAND_TYPES` in `count.rs`) vs data-driven from the RON subtype registry.
  [Conferrals come from registries](../../decisions/conferrals-come-from-registries.md)
  favors data over hardcoded name matches. Party classes are not a data flag today, so the data route needs a
  new source.
- **C — surface shape.** A scalar `Count` covers "for each creature in your
  party" / X = party. Full-party (`[CR#700.8c]`) is likely a separate
  `Condition` / ability-word (`Compare(PartySize(You), AtLeast, 4)`) — decide
  whether to add that now or defer.

Recommended minimal-correct path if the design pass agrees: **A2 + B(const)** —
`Count::PartySize` + a matching evaluator in both count sites + a `PARTY_CLASSES`
const + `[CR#700.8]` citation + unit tests. The 5 blocked cards were not
enumerated (not needed for the pause). Surfaced by batch execution.
