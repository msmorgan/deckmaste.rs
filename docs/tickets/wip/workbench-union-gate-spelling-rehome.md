---
needs: []
---
# Re-home the union family's measured gates as spelling-boundary content

The marked union constructions' measured tables are **spelling-boundary
knowledge** — facts about what English writes — and their home is english_v2's
construction declarations, not a semantics constructor. This ticket moves them,
verbatim, and it must land **before** the constructors it drains are migrated
away, because the ADR's own consequence is that *a measured zero dropped in the
move becomes a silent yes*.

Authority:
[The kind index joins; union marking is spelling](../../decisions/kind-index-joins-union-marking-is-spelling.md)
(see also [English grammar is derived](../../decisions/english-grammar-is-derived.md)
and [Builtin-v2 macro spelling and grammar](../../decisions/builtin-v2-macro-spelling-and-grammar.md)
for the declaration shape).

## The inventory to move — the full measured content

`Predicate.KindJoin`, the cross-kind union head, 326 supported occurrences:

- The **2×4 admissible-pair grid**, all eight cells attested: `JoinedPlayer`
  (player 279, opponent 45) × `JoinedClass` (planeswalker 205, permanent 100,
  battle 17, creature 4). Two parameters rather than a flat enum of the
  combinations somebody happened to write.
- **Not coordinable**: nesting one in `Or` "would spell the same union twice
  over" (`badKindJoinInOr`).
- **Not negatable**: a union of two classes is not a property a thing can lack.
- **Modifier-bearing in only 11 of 326.**
- **Order is free variation** and the covariance test fails: permanent 92
  object-first / 8 player-first; battle 16 player-first / 1 object-first.

`Noun.YouAnd`, the mixed group, 35 supported sentences:

- The **player half is fixed at `You`, 35/35** [CR#109.5]; "target player and
  creatures they control" is 0 lines. The object half takes the full object-noun
  range — bare plurals, complement, distributive, counted groups, the indefinite,
  the self, a target mention, the class word.
- It **refuses itself** (`NotMixedGroup`, `badNestedYouAnd`).
- "and" vs "and/or" is free variation (23 vs 12, exact crossing pair Channel
  Harm / Refraction Trap); order likewise.

`Predicate.AnyTarget`, the class word [CR#115.4]:

- One lexical item, nullary. `negatable AnyTarget = False` — the class word is
  never negated, the rule defining it positively.
- `AnyTargetLone`, its modifier discipline, and `AnyTargetAtCount`.
- The standing question "is this the union head under another spelling?" was
  answered **NO** on three measurements: no cross-kind head joins two classes to
  a player as this word (the 2 cards writing "creature, planeswalker, or player"
  are this word's own pre-battle spelling); the rules name it as one lexical
  item; the two obey different modifier disciplines.

The anaphor — `That JoinW` / `Payload.UnionP` / `wordReaches JoinW`, 48 supported
readbacks:

- **The 33 whose antecedent is a union head echo that head's pair exactly, 33 of
  33 with no crossing in either direction.** The other 15 have no pair to echo
  and **all 15 write the generic "permanent or player"**.
- The class word and the union head share **one payload**: 33 union-headed
  antecedents echo their own pair, 10 class-word antecedents write the generic
  one, **all 43 with the same demonstrative and no card distinguishing them**.
- Not one of the 48 is a bare "that player"/"that permanent" pointing at a union
  antecedent; the 25 that name a half spell it as a coordination of two
  demonstratives ("that player or that planeswalker's controller"), a noun
  disjunction this grammar does not have.

The measured zeros that today ride a constructor's shape: the seven-verb
battlefield refusal, **0 supported sentences** across destroy, exile, tap, untap,
return, counter and sacrifice (`badDestroyKindJoin`).

## Out of scope — do not write these down

Facts a joined kind proves are not re-homed anywhere. `headIsPlaceless`'s two
union disjuncts, `nounSpansPlayers`, `bindFor`'s union branch, and
`DamageRecipient`'s three union rows [CR#615.7] all follow from the joined kind
having no zone and no type. Writing a derivable fact down is the failure this
direction exists to stop.

## Consumption boundary

`crates/deckmaste_english_v2/src/constructions.rs` (the declaration set, built on
`deckmaste_construction`'s `constructions!`), with the Idris sources
(`idris/src/Experimental.idr`, `idris/src/Experimental/Words.idr`) as the source
text — every number above is quoted from a docstring there and can be re-read at
its site. The declaration set is an early slice and the union families are not
declared in it yet; where a family cannot be declared today, the deliverable is
the measured content in the form the declaration author consumes, homed with the
declarations rather than left in an Idris docstring.

Do not touch any crate slated for deletion at the english_v2 cutover.

## Acceptance

- Every measurement above is present in the re-homed content, with its count, and
  none rounded or restated from memory.
- Nothing in the Idris workbench is deleted by this ticket — draining is
  `workbench-union-family-macros`'s job, and this ticket is its prerequisite.

Standard constraints apply.

## As-landed

**The drain this ticket was to precede has already landed.**
`Predicate.AnyTarget`, `Predicate.KindJoin` and `Noun.YouAnd` left the core in
`workbench-union-family-macros`, replaced by `Macros.anyTarget` / `kindJoin` /
`youAnd` / `thatJoin` over a joined kind `Object \/ Player`, and the Idris
docstrings carrying the measured tables were deleted with them. The re-homing
is therefore still the deliverable — and the only one — but no number could be
quoted from its site; every figure below was re-measured from the corpus.

**Landed:** `crates/deckmaste_english_v2/docs/union-spellings.md` (407 lines),
plus a seven-line `//` pointer above `construction demonstrative` in
`crates/deckmaste_english_v2/src/constructions.rs`. No Idris change. The
declaration set is still the 204-line early slice with no union family
declarable, so the content is homed in the form the declaration author
consumes, beside the declarations, per this ticket's own consumption boundary.

**Method.** Distinct supported oracle lines via the `mtg-rules` skill's
`scripts/corpus --match`, re-measured 2026-08-22, every regex printed beside
its count in the file. The deleted docstrings' regexes are unrecoverable, so
the two measurements are not reconcilable line by line; the file states both
figures wherever they differ.

**Re-measured against this ticket's inventory** (ticket → mine):

- Union head total 326 → **319**. Grid: player 279 → **258**, opponent 45 →
  **61**; planeswalker 205 → **236**, permanent 100 → **63**, battle 17 →
  **16**, creature 4 → **4**. All eight cells attested either way. The
  ticket's own subtotals do not agree with each other (279 + 45 = 324, not
  326). The planeswalker and permanent errors run opposite ways while the
  totals nearly match — consistent with the docstring counting some
  demonstrative readbacks as heads.
- Order: permanent 92 object-first / 8 player-first → **46 / 17**; battle 16
  player-first / 1 object-first → **15 / 1**. Covariance still fails.
- Head modifier-bearing 11 of 326 → **22 of 319** (21 genuine; one is a PP
  attaching to "attack"). Nine distinct spellings, enumerated in the file.
- Not coordinable, not negatable: both **0**, agreeing.
- Mixed group 35 sentences → **36**; "and" 23 / "and/or" 12 → **28 / 8**. The
  crossing pair Channel Harm / Refraction Trap verified by `scripts/card`.
  "target player and creatures they control" **0**, agreeing.
- Class word: "creature, planeswalker, or player" **2**, agreeing;
  "creature, player, or planeswalker" **0**. "any target" itself 748 lines.
- Anaphor 48 readbacks → **68**; 33 union-headed → **51**; 10 class-word →
  **10**, agreeing exactly; 15 no-pair → **17**.
- Seven-verb zero: **0** for all of destroy, exile, tap, untap, return,
  counter and sacrifice, agreeing. A looser window returns 29 lines, all
  false positives; the file records the anchoring the regex needs.

**Two of this ticket's claims are wrong on their own terms**, and the file
says so rather than restating them:

1. "`AnyTargetLone` … the class word bears a modifier in none." Four printed
   lines carry a restrictive relative clause on the class word ("any target
   that isn't a Dragon", "…that isn't a Dinosaur", "…that isn't a commander",
   "…that was dealt damage this turn"). The discipline that measures 0 is the
   narrower one: no pre-nominal modifier, no control restriction. "any other
   target" (16 lines) is the complement, not a modifier.
2. "The player half is fixed at `You`, 35/35." True of the 36 "you and …"
   lines, and true of the exact spelling the ticket tested. But 14 printed
   lines coordinate a non-"you" player half with a distributive object half —
   "deals N damage to target player and each creature that player controls",
   "to each opponent and each creature they control". A declaration that
   hard-wires "you" refuses them.

**A third divergence, recorded in the file.** The head's class inventory is
not [CR#115.4]'s. That rule names creatures, players, planeswalkers and
battles — the class word's list — while the head's second-largest cell is
"permanent" (63 lines), a word the rule does not use. The two heads do not
draw on the same inventory, which is a fourth independent answer to this
ticket's "is the class word the union head under another spelling?".

**Out of scope, honoured.** `headIsPlaceless`, `nounSpansPlayers`, `bindFor`'s
union branch and `DamageRecipient`'s union rows are not written down; the file
closes by naming them as derivable from the joined kind.

**Gates.** `cargo fmt`; `cargo clippy -p deckmaste_english_v2` 0 warnings;
`cargo test -p deckmaste_english_v2` green; `cargo xtask cite check
--list-noncompliant` empty; `cite check` 0 stale; `cite bless`; `cite audit
--diff` read over every citation site. Not committed.
