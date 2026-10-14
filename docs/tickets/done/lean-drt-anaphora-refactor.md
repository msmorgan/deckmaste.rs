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

### Macro bodies that re-mention a moved parameter (ruling 2026-10-08)

A body that moves a parameter's referent to another zone reads it back
afterwards by anaphora, never by the parameter [CR#400.7,400.7j]
(`docs/decisions/semantics-v2.md` §7, "A moved referent is read back by
anaphora"). Swept 2026-10-08 over `plugins_v2/`:

- `keyword_actions/airbend.ron` [CR#701.65a]: re-spelled; its later mentions
  are `it`. "It" agrees with one object, so several airbended objects are
  refused (`[anaphor bare one 0]` ×3, pinned `airbendItRefusesSeveral` in
  `lean/Semantics/Proofs/BindingIdentity.lean`) until the per-card loop in
  "Routed from `semantics-v2-per-member-handoff`" below replaces the one
  handoff. No anaphor checks for both numbers under the strict rule: `them`,
  `that(card)` and `theVerbed(exile, …)` each fix a number.
- `instructions/discardThenForEachNonland.ron` (connive's discard, `discard`
  then a loop over `allFromAmong(not(land), Param(discarded))`
  [CR#701.50a]): not re-spelled, connive being another landing's. The
  description "each nonland card discarded this way", `allOf(and([not(land),
  card, happenedTo(verbedEvent(discard, relative(Object)), ThisWay)]))`,
  checks for connive 1 and connive 2 alike (measured in a scratch Lean file,
  `[]` for both).

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

## Routed from `semantics-v2-per-member-handoff` (2026-10-08)

Airbend's per-card spelling [CR#701.65a], the plural reminder's "while each
one is exiled, its owner may cast it", loops over the cards exiled this way,
not over `Param(0)` (ruling 2026-10-08: the exiled cards are new objects
[CR#400.7,400.7j]): `sequentially([exile(Param(0)), doForEach(allOf(and([card,
happenedTo(verbedEvent(exile, relative(Object)), ThisWay)])), act(ownerOf(it),
establish(mayCastFrom(it, exileZone, PayingInstead(mana([2]))),
forAsLongAs(matches(it, inZone(exileZone))))))])`. The description has no
fixed number, so it checks for several objects; for one, each "it" in the loop
reaches both the loop's card and the exiled object, the same object, and ties.
The nearest-clause rule resolves it to the loop's card. Flip the pin
`airbendByCardOneTiesUnderStrictRule`
(`lean/Semantics/Proofs/BindingIdentity.lean`) to `[]`; the airbend body in
`plugins_v2/builtin/macros/keyword_actions/airbend.ron` then takes this
spelling in place of its one handoff, and `airbendItRefusesSeveral` is
re-spelled against it. The group `theVerbed(exile, Many, card)` ("the exiled
cards") was measured and does not serve: a stamped pronoun fixes its number,
so for one exiled object it is refused `[anaphor (verbed (action "exile"))
many 0]` by number agreement, which no resolution rule here lifts.

## Routed from `semantics-v2-piles` (2026-10-08)

Ripple [CR#702.60a] stays bodyless; cascade [CR#702.85a] got its body
("cards exiled this way that weren't cast" is `allFromAmong(not(wasCast),
theVerbed(exile, Many))` and checks clean). Ripple's last clause, "put all
revealed cards not cast this way on the bottom of your library in any order",
has no reading the checker accepts. Measured on the body `may(reveal(slice(Top,
N, library(you))), then: cast someOf(anyNumber, named(sameAs(this)), them)
without paying, then move <the rest> onto the bottom in any order)`:

- "revealed cards" as `theVerbed(reveal, Many)`: refused `[anaphor (verbed
  (action "reveal") none attributive) many 0]`. The reveal deed stamps no
  patient, as Protection's `Attach` and Indestructible's `Destroy` rows do not.
- "those cards": refused `[anaphor (word card) many 2]`, the revealed slice
  and the cards cast both in reach.
- The slice re-mentioned by Binding Identity checks clean, but the cast has
  moved part of it, so the ruling "A moved referent is read back by anaphora"
  (2026-10-08) bars it.

The resolution rules here may settle the second; the first needs the reveal
row to stamp what it reveals. Either one gives ripple its body.

## Architectural resolution and ticket decomposition (2026-10-14)

Designing the clean-sheet DRT checker surfaced two coupled invariants that
greatly simplify the checker and macro layer when solved together:

1. **`Definition` vs. `Template` split:** Rules-defined vocabulary (all CR §701
   keyword actions — atomic and composite — CR §702 keyword abilities, and
   registry definitions) are `Definition`s evaluated in their own Reference
   Scope (`ownScope` + `inCaller`), whereas Oracle phrasing helpers are
   transparent `Template`s evaluated in the caller's DRS scope (retaining
   `firstMention`/`laterMention` parameter identity when a parameter is
   mentioned $>1$ time before a zone change).
2. **Unified Action-Clause Spine:** Every game action is defined once as an
   `Instruction`, and the six clause moods embed `(subj : Subject, action :
   Instruction)` directly (`instruct`, `does`, `instead(would(...))`,
   `cant`/`must`/`may`/`gatedBy`, `did` + `tally`), with `Subject` encoding
   active (`np`), unspecified-agent passive (`any`), and explicit-agent passive
   (`by(np)`), `any : α` (`AnyValue`) across wildcardable slots, and `which`
   (`NounPhrase.which`) replacing `NounPhrase.gap` in relative clauses.

### Landed in this ticket

- **Authoritative ADR:** [`docs/decisions/semantics-v3.md`](../../decisions/semantics-v3.md),
  updated [`docs/decisions/README.md`](../../decisions/README.md),
  [`CLAUDE.md`](../../../CLAUDE.md), and [`lean/CONTRACTS.md`](../../../lean/CONTRACTS.md).
- **Archived superseded docs (`docs/archive/`, excluded from search via `.ignore` and from `cite-config.json`):**
  `semantics-v2.md`, `oracle-text-is-forward-anaphoric.md`,
  `idris-is-a-soundness-gate.md`, and `idris-workbench-closure-tables.md`
  (while keeping `builtin-v2-macro-spelling-and-grammar.md`,
  `card-authoring-binds-no-implicits.md`, and
  `workbench-ron-shaped-and-label-rulings.md` active in `docs/decisions/`).
- **Deleted 15 obsolete v2/Idris/workbench tickets (with useful residues folded into the absorbing `v3` tickets):**
  - Folded into `lean-v3-action-clause-spine`: `lean-constructor-collapse`, `plugins-v2-menace-bound-helper`
  - Folded into `lean-v3-drt-checker`: `lean-checker-binding-ids`
  - Folded into `lean-v3-definitions-and-templates`: `semantics-v2-per-member-handoff`
  - Folded into `semantics-v3-dsl-and-plugins`: `lean-macros-cutover`, `plugins-v2-bodies-through-helpers`, `semantics-v2-embed-candidates`
  - Purely obsolete Idris/workbench scaffolding deleted: `docs-workbench-drift`, `idris-distinct-position-proof`, `idris-emittables-drift-control`, `idris-composite-card-types`, `idris-emit-twoface-card`, `idris-event-model-coverage-gaps`, `workbench-obligation-cycle-totality`, and `workbench-segment-indexed-telescope`.
- **Ported / marked `[port-required: semantics-v3]` on tickets holding design decisions for unimplemented keywords and mechanisms:**
  - `semantics-v2-damage-results` (`DamageResult` with `source : Subject`, `does : Instruction`), `lean-marked-damage-projection` (`ProjAxis.markedDamage` + `did` lookback to `stateBasedActionsChecked`), `lean-defense-projection` (`Stat.defense`), `plugins-v2-canon`, `lean-hand-bench-retirement`, `lowering-v2`, `semantics-v2-parity`, `semantics-v1-cutover`, `semantics-v2-designation-storage-columns`, `semantics-v2-english-translation`, `lean-core-action-provenance`, `lean-core-as-event-instructions`, `workbench-last-shapes` (Alaundo, `"all cards with the same name"`, linked-exile law, Vedalken Squirrel-Whacker, Tahngarth), `workbench-corpus-frame-gate`, `research-deed-subjects`, `idris-mirror-enum-gaps` (`"doesn't untap"` = `cant`, `EnterRider` inventory, `Linked`, `Arrangement`), `idris-retire-cards-idr` (71-card coverage inventory + Donate), `idris-copy-grammar-tail` (copy-exception doctrine), and `idris-coalesce-payer` (`anyTarget` toll payer coalescence).
  - Marked `[port-required: semantics-v3 (contingent on open DSL question in semantics-v3-dsl-and-plugins)]`: `macro-ron-raw-value-restriction-drop`, `macro-ron-single-pass`, `macro-keyword-templates`, `memoize-macro-invocations`, `macro-collision-diagnostic`, and `target-sugar-elaboration`.
- **Glossary & Style Guide amendments:** Updated **Reference Scope** in
  [`docs/contexts/game-model/CONTEXT.md`](../../contexts/game-model/CONTEXT.md),
  **Clause-mate** in
  [`docs/contexts/oracle-english/CONTEXT.md`](../../contexts/oracle-english/CONTEXT.md),
  and the corpus-measured **Pronouns** and **This, that, those, and the chosen**
  rules in [`docs/oracle-style-guide.md`](../../oracle-style-guide.md) §5.
- **Lean-first `semantics-v3` implementation tickets (`docs/tickets/planned/`):**
  1. `lean-v3-action-clause-spine` — Refactor `lean/Semantics/` syntax onto the
     Unified Action-Clause Spine and retire `Window` and `withBindings`.
  2. `lean-v3-drt-checker` — Replace `Bindings`, positional stack addresses, and
     `MoveProfile`/`StepProfile` in `lean/Semantics/Check/` with the clean-sheet
     `DRS` and the `J1`, `J2`, `D′`, and `AC>R1` resolution gates.
  3. `lean-v3-definitions-and-templates` — Enforce the `Definition` vs.
     `Template` split in Lean, retire the 9 hand-coded window macros, and land
     the `airbend` (`[CR#701.65a]`) and `ripple` (`[CR#702.60a]`) bodies.
  4. `lean-v3-proofs-and-cards` — Re-spell `lean/Semantics/Proofs/` and
     `lean/Semantics/Cards/`, prove the DRT witness lemmas, flip the 6 resolved
     ambiguity pins, and add the 11 known-wrong `AC>R1` exception pins.
  5. `semantics-v3-dsl-and-plugins` — Create `crates/deckmaste_semantics_v3` and
     `plugins_v3/` mirroring the proven Lean v3 specification.

