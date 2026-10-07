---
needs: [semantics-v2-anaphor-resolution-heuristics]
---
# Rebuild the checker's anaphora context on discourse-representation prior art

**Build what the accepted anaphor resolution rules read, and apply them.**
Promoted from `maybe/` on 2026-10-07, when the owner accepted the rules the
measurement in `semantics-v2-anaphor-resolution-heuristics` (now `done/`) put
to them. The ruling is §7 of `docs/decisions/semantics-v2.md`, 2026-10-07,
"Anaphor resolution: measured rules replace the strict rule as the target".
Standard constraints apply.

## The accepted rules (2026-10-07)

`it` and "that N" keep different rules. Both run after accessibility.

- **Bare `it`: clause recency, with grammatical role as the tie-break** (the
  measurement's D′): the referent in the nearest clause; among referents in
  the same clause, subject over object over other; a true tie refuses. 60
  correct on the 61 ambiguous sampled sites under both coders' features
  (60 / 1 / 0 with coder 2's, Grip of Phyresis the one wrong; 60 / 0 / 1
  with coder 1's, Cocoon refused). The sample projects to ~1,529 ambiguous
  occurrences of 3,759.
- **"that N": clause-mate exclusion, then most recent** (AC>R1): remove the
  anaphor's clause-mates and what a pronoun in its own clause resolved to,
  then take the most recent remaining referent. 76 / 11 / 0 on the 87
  ambiguous sites.

## What the context must define (owner, 2026-10-07, provisional)

Both judgments were accepted "ok to both I guess. we'll try this way for now
anyhow"; build them as stated and keep them easy to revisit.

- **Accessibility (J1).** A mention the consuming verb cannot apply to by
  zone or kind is inaccessible, before any resolution rule runs: a creature
  that died cannot take a counter, a non-permanent cannot be sacrificed
  [CR#109.2,701.21a]. Bindings already carry zone and kind, so this is a
  condition on the referent in the DRT sense. It extends the carrier scoping
  of `ItAt` to every read, "that N" included.
- **Clause-mates (J2).** The clause-mates of an anaphor are the co-arguments
  of its verb only. A possessor ("its power") and the object of a small clause
  ("counters on it") are not clause-mates. This keeps the pin
  `badItAcrossOwnSlot` and the Weeping Angel corpus case ("that creature's
  owner shuffles it"). The glossary term is **Clause-mate**
  (`docs/contexts/oracle-english/CONTEXT.md`).

## Known-wrong sites: a required pin list

Under the target rule these 11 "that N" sites resolve to the wrong referent,
silently: the rendered English is identical, so the round-trip cannot catch
them. Each is pinned as "resolves to the wrong referent under the target
rule; the card must use an explicit selector":

Tahngarth, First Mate; Unpredictable Cyclone; Eriette, the Beguiler;
Runesword; Scythe of the Wretched; Bronze Bombshell; Gisela, Blade of
Goldnight; Solphim, Mayhem Dominus; Ram Through; Mangara's Equity; Noetic
Scales.

The first six are a newer mention inside the intended referent's modifier;
Gisela and Solphim are the parallel disjunct "an opponent or a permanent an
opponent controls"; the last three are kind f. The intended referents are in
the measurement's answer key (`docs/evidence/anaphora-measurement-2026-10-06/
key_that.txt`).

## Provenance

Decided 2026-10-06: the ticket stayed parked in `maybe/` until the owner
accepted a rule the measurement put to them, which happened 2026-10-07.

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

- **The rules are the owner's, accepted 2026-10-07.** This ticket implements
  them and nothing else.
- **Carrier-scoping and fold-state stay.** They are filters the new rule
  composes with (`docs/decisions/semantics-v2.md` §§3–4); J1 makes them part
  of accessibility.
- **Refuse rather than guess.** Where the accepted rule ranks candidates, a
  true tie still refuses.
- **Nothing `decide` cannot reduce.** `decide` fails on a one-entry
  `Std.HashMap` lookup in this toolchain (tested 2026-10-05).
- **Derive, do not annotate.** The checker already walks the clause it is
  checking, so clause membership and grammatical role should be read from
  that structure, not written on cards or added as syntax. If a syntax
  constructor has to change, the Rust mirror, the RON registry and
  `plugins_v2` move in the same landing (`semantics-v2.md` §10).
- **Rulings to amend before claiming.** Partly done 2026-10-07: the §7
  ruling supersedes `semantics-v2.md` §4's "no nearest-wins tiebreak" and the
  strict statement in `lean/CONTRACTS.md` as the target, and
  `docs/decisions/oracle-text-is-forward-anaphoric.md` carries a dated note
  pointing at it. Still owed when the checker changes: rewrite that ADR's
  clause 3, the "NOT a preference" passage and the `countBy` witness lemma
  requirement for the ranked gates; decide whether the 2026-09-04 ruling
  "Reads are one `Pro` over a window" still holds; amend the Game Model
  **Reference Scope** entry, which still states the strict uniqueness rule;
  and land the Pronouns amendment to `docs/oracle-style-guide.md` the
  measurement ticket's Proof asked for.
- **Glossary.** Antecedent status (what `it` may pick up and "that N" may
  avoid) needs a term of its own in `docs/contexts/oracle-english/CONTEXT.md`
  if the implementation names it. The literature's "in focus" collides with
  the existing **Focus** entry; do not use it.
- **Out of scope, named.** The reciprocal gap: "fight" ("Each of those
  creatures deals damage equal to its power to the other creature"
  [CR#701.14a], 145 paragraphs / 142 cards), "fight each other" (11 / 11) and
  "one another" (2 / 2) need a binder over a pair that reads "the other
  member". No accepted rule supplies it, and this ticket does not build it.

## Hand-built windows to retire or generalise

`lean/Semantics/Macros.lean`, lines as of 2026-10-07. Each either becomes the
general rule or stays, with a sentence saying what else it encodes:

| Macro | Line | Window today |
|---|---|---|
| `itPrior` | 37 | `.introduced` over what the previous instruction announced |
| `itCondSubject` | 44 | `.introduced` over what the condition introduced |
| `attachToIt` | 670 | `.outsideIntroduced`, a hand-coded clause-mate exclusion |
| `requireBlockIt` | 696 | `.outsideIntroduced`, a hand-coded clause-mate exclusion |
| `agentRef` | 724 | `.introduced` over the agent's own bindings |
| `itsOther` | 747 | `sameWindow` over `selfSubjIntro` (possessive, so J2 keeps it) |
| `ownSubject` | 761 | `.introduced` over what the subject announced |
| `controllerSacrifices` | 766 | `.introduced [.player, …]`, or `ownSubject` |
| `lookedCards` | 776 | `.introduced` over the slice |

## The work

1. The context carries what the rules read: clause membership and
   grammatical role for every binding, and the zone and kind J1 reads ("What
   each rule reads (step 6)" in the measurement ticket).
2. The `.pro` gate applies the rule, separately for `it` and `that N` where
   the rule separates them.
3. The nine macros above lose their hand-coded windows where the general
   rule reproduces them; any that stays says what else it encodes. Audit the
   `Window.top` users (`comparesOwnStat`, `dealDamageOwnPower`) the same way.
4. The new gate arrives with its witness lemmas in `lean/Semantics/Proofs/`,
   as the forward-anaphora ADR's Consequences section requires of any new
   gate.
5. Each ambiguity pin the rule resolves is re-spelled as an acceptance that
   asserts the intended referent from the measurement's answer key (6 of the
   21 flip to the intended referent, 0 wrong, 9 undeterminable; P07, P08,
   P09, P12, P15, P16 stay refused). None is deleted. The 11 known-wrong
   sites above are added as pins.
6. `crates/deckmaste_semantics_v2/src/reads.rs` ports the same reads in the
   same landing.

## Proof

- The refused-reference counts from the measurement, re-run on the landed
  checker: before and after, with every wrong resolution named.
- The existing pins for Feral Contest (`Cards/Counters.lean`) and Cloud,
  Ex-SOLDIER (`Cards/Trigger.lean`) still pass once their macros use the
  general rule. New pins for Monstrous Step and Kalitas, Bloodchief of Ghet
  resolve to the intended referents by `decide`. Open: the measurement found
  that Principle B proper (its Aarg, which J2's co-arguments-only boundary
  resembles) costs Kalitas and Broken Visage; if J2 as built refuses or
  misresolves them, report that against J2 rather than bending the boundary.
- The same-syntax pair is settled by J1, not syntax: Vampiric Embrace
  resolves to the embedded creature by the zone exclusion, and Scythe of the
  Wretched resolves wrongly and is pinned as known-wrong.
- The 11 known-wrong pins and the `it`-sample error (Grip of Phyresis) are
  the only wrong resolutions on the measured sites; any other is a defect.
- `lean/scripts/build` and `cargo xtask lean check` pass.

## Related

`lean-checker-binding-ids` is an independent cleanup of how frame slots address
bindings. Neither ticket needs the other.

## Routed from `semantics-v2-macro-capture-and-plurality` (2026-10-07)

Fight's rule text, "Each of those creatures deals damage equal to its power to
the other creature" [CR#701.14a], is a reciprocal the model lacks. The
`plugins_v2` fight body writes two damage events by Binding Identity instead;
the reciprocal was routed to `semantics-v2-anaphor-resolution-heuristics`,
which closed without it.

## Also routed from `semantics-v2-macro-capture-and-plurality` (2026-10-07): the `reads.rs` mirror

That landing added two checker reads Rust does not port. Lean records a
Binding Identity on `Binding.idents` (`lean/Semantics/Check/Words.lean`,
`structure Binding`) and reads it with `identityAddress`/`identityBinding`;
`crates/deckmaste_semantics_v2/src/reads.rs:627` (`pub struct Binding`) has no
such field and no identity read. Lean opens a keyword body's own scope
(`enterOwnScope`, `leaveOwnScope`, the frameless and own-scope cases of
`enterCaller`); `reads.rs` `enter_caller` (line 817) ports only the capture
frame. No card behaves differently today: `reads.rs` refuses nothing and no
lowering reader resolves a `LaterMention` or an `OwnScope` yet, so the gap is a
mirror gap, not a divergence a card can observe. Item 6 above, which ports this
refactor's reads to `reads.rs` in the same landing, ports these with them.
