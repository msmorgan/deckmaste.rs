---
needs: [semantics-v2-anaphor-resolution-heuristics]
---
# Rebuild the checker's anaphora context on discourse-representation prior art

**Parked until the measurement in `semantics-v2-anaphor-resolution-heuristics`
puts a rule to the owner and the owner accepts one.** Standard constraints
apply.

Owner's motivation (2026-10-05): the checker has reinvented discourse
representation theory piecemeal, and taking the prior art deliberately may fix
references it handles badly today. "Another target creature blocks it" on
Feral Contest checks only because `requireBlockIt` hand-codes the exception
for that one verb; the same shape under another verb is refused. It also gives
`it` and `that N` one rule, although Oracle text uses the demonstrative to
disambiguate where the pronoun would be ambiguous.

## What we have, against the prior art

| The checker | The prior art |
|---|---|
| `Bindings`, one `Binding` per mention | the universe of discourse referents |
| fold-state on a binding (zone, `Stamp`, origin) | conditions on a referent |
| `Window`, `enterCaller`, `Payload.masked` | accessibility (Kamp and Reyle 1993) |
| `countReach … == 1` | resolving a presupposition by binding it to a unique antecedent (van der Sandt 1992) |

That much is the theory, reinvented. What the theory does not supply, and the
checker lacks, is the choice among accessible antecedents:

- **Clause and argument structure.** The context does not record which
  bindings are arguments of the clause being checked, so disjoint reference
  cannot be stated once. It is hand-coded per verb through `outsideIntroduced`
  in `attachToIt` and `requireBlockIt` (`lean/Semantics/Macros.lean`).
- **Antecedent status.** Nothing separates the mention at the centre of
  attention from one merely mentioned earlier, which is the difference between
  what `it` and `that N` may pick up.
- **A form-sensitive gate.** `Reach.bare` and `Reach.word` differ only in
  their word filter; both pass through the same count in
  `Check/PhraseRules.lean`.

## Decisions already made

- **The rule is the owner's, after the measurement.** This ticket implements
  the accepted rule and nothing else. With no accepted rule it stays parked.
- **Carrier-scoping and fold-state stay.** They are filters the new rule
  composes with (`docs/decisions/semantics-v2.md` §§3–4).
- **Refuse rather than guess.** Where the accepted rule ranks candidates, a
  true tie still refuses.
- **Nothing `decide` cannot reduce.** `decide` fails on a one-entry
  `Std.HashMap` lookup in this toolchain (tested 2026-10-05).
- **Derive, do not annotate.** The checker already walks the clause it is
  checking, so clause membership and grammatical role should be read from
  that structure, not written on cards or added as syntax. If a syntax
  constructor has to change, the Rust mirror, the RON registry and
  `plugins_v2` move in the same landing (`semantics-v2.md` §10).
- **Rulings to amend before claiming.** An accepted rule changes
  `docs/decisions/oracle-text-is-forward-anaphoric.md` (clause 3, the "NOT a
  preference" passage, the `countBy` witness lemmas), `semantics-v2.md` §4
  ("no nearest-wins tiebreak"), and possibly the 2026-09-04 ruling "Reads are
  one `Pro` over a window". Claiming with any of these unamended is a STOP.
- **Glossary.** The status in the second gap above needs a term of its own in
  `docs/contexts/oracle-english/CONTEXT.md`. The literature's "in focus"
  collides with the existing **Focus** entry.

## The work, once a rule is accepted

1. The context carries what the rule reads (step 6 of the measurement's report
   lists it).
2. The `.pro` gate applies the rule, separately for `it` and `that N` where
   the rule separates them.
3. `attachToIt` and `requireBlockIt` lose their hand-coded windows if the
   general rule reproduces them. Audit the other `introduced` and `top` windows
   in `Macros.lean` the same way, and keep any that encode something else,
   saying what.
4. The new gate arrives with its witness lemmas in `lean/Semantics/Proofs/`,
   as the forward-anaphora ADR's Consequences section requires of any new
   gate.
5. Each ambiguity pin the rule resolves is re-spelled as an acceptance that
   asserts the intended referent from the measurement's answer key. None is
   deleted.
6. `crates/deckmaste_semantics_v2/src/reads.rs` ports the same reads in the
   same landing.

## Proof

- The refused-reference counts from the measurement, re-run on the landed
  checker: before and after, with every wrong resolution named.
- The existing pins for Feral Contest (`Cards/Counters.lean`) and Cloud,
  Ex-SOLDIER (`Cards/Trigger.lean`) still pass once their macros use the
  general rule. New pins for Monstrous Step and Kalitas, Bloodchief of Ghet
  resolve to the intended referents by `decide`.
- The same-syntax pairs the measurement could not separate (Vampiric Embrace
  against Scythe of the Wretched) still refuse.
- `lean/scripts/build` and `cargo xtask lean-check` pass.

## Related

`lean-checker-binding-ids` is an independent cleanup of how frame slots address
bindings. Neither ticket needs the other.
