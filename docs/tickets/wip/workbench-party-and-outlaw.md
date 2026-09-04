---
needs: []
---
**Add `Noun.OneEachOf` for party and spell outlaw as a macro.** Fresh
workbench review 2026-09-03, R4, resolved by ruling.

**Ruling (settled 2026-09-03): outlaw is a macro, party is a constructor.**
"Outlaw" is a plain disjunction over creature types and needs no grammar: a
macro spelling that disjunction is the whole of it, the way
`Phrase.Predicate.IsHistoric` (`Phrase.idr:289`) is a leaf for its own defined
term. 20 supported cards say it (e.g. At Knifepoint).

Party is not a disjunction. `Noun.OneEachOf (roles : List (Predicate bs
Object)) (pool : Noun bs Object)` denotes **the joint maximal sub-group of
`pool` with at most one member per role and each member counted once**,
computed by the game per [CR#700.8a..700.8b] — not by the term. `party`,
`partySize`, `fullParty` and "a creature in your party" are macros over it.
31 supported cards read a party (e.g. Archpriest of Iona).

**Banned compositions**, both of which get the wrong answer and neither of
which may be substituted for `OneEachOf`:

- Independent per-role finds. Four separate "a Cleric you control", "a Rogue
  you control", … double-count a creature that is more than one of those
  types, so a lone Cleric Rogue would be a party of two.
- Greedy consuming finds. Taking each role in turn and removing the creature
  it matched is not maximal: an early role can consume the only creature a
  later role could have used.

Pin the small case in both directions: a single Cleric Rogue is a party of
**one**, and the term must not admit a reading that makes it two.

Stick Together is unaffected and keeps its `Choose (UpTo 1)` per role
[CR#700.8d] — that card chooses creatures, it does not read the party.

Size: M.

Done when: Archpriest of Iona and a `fullParty` card are typechecking bench
witnesses, and At Knifepoint reads through the outlaw macro; `partySize` of a
lone Cleric Rogue is one, recorded as a bench witness rather than a comment;
a pin refuses the double-counting reading, probed non-vacuous; Stick Together
still typechecks unchanged; the build is 44/44 with 0 errors and 0 warnings.
Standard constraints apply, plus the RON-shaped constraint: a core constructor
is admissible only if the RON re-emitter can produce it from a RON node, and a
macro only if it names a RON macro
(`docs/decisions/workbench-ron-shaped-and-label-rulings.md`).

## As landed

- `Phrase.Noun.OneEachOf (roles : List (Predicate bs Object)) (pool : Noun bs
  Object)` — the joint one-each group, with `{auto 0 pl : nounPlur pool =
  ManyOf}` and `{auto 0 rk : RolesOk roles}`. `RolesOk` is a named data
  witness (`RolesAre`) over the new `rolesDistinct`/`roleElem`, so an empty
  role list and a repeated role fail with different messages.
- Projections: `nounDelta` introduces one `BareD`/`ManyOf` object binding
  carrying the pool's type and zone, then the roles' and pool's deltas (the
  bare-plural group shape); `nounDet = Just BareD`; `nounPlur = ManyOf`;
  `nounZone`/`nounTy`/`nounHeadTys` read the pool; `nounEqRef = False`;
  `moveIntro` re-zones the head like `SomeOf`; `groupMention = True`.
- `Macros`: `outlaw`, `outlawYouControl`, `partyRoles`, `partyOf`, `party`,
  `partySizeOf`, `partySize`, `creatureInYourParty`, `fullPartyOf`,
  `fullParty`.
- Bench (`Cards/Description.idr`): `partyCount` (the maximum-matching model)
  with `loneClericRogueIsPartyOfOne`, `clericBesideClericRogueIsTwo`,
  `oneOfEachRoleIsFullParty`; `archpriestOfIonaPower`,
  `archpriestOfIonaFullParty`, `squadCommanderTokens`, `atKnifepointOutlaws`.
  `Cards/Choice.idr`: `stickTogether`.
- Pins (`ProofsDescription.idr`): `badRepeatedPartyRole`,
  `badEmptyPartyRoles`, with the shared twin `okPartyOfFourRoles`.
- **Merge with `workbench-distributive-mutation-hole`.** Took (a), the guard is
  right: `moveIntro (TheRest _)` is `groupSpent bs`, a spend of bindings the
  agent phrase never introduced, while `doesInstrIntro ManyOf` republishes
  `nomIntro s` over it — so the spend would be dropped and "the rest" would
  stay spellable exactly where `ProofsChoice.badChoiceRestDisposedTwice`
  refuses it undistributed. `stickTogether` is re-spelled as one
  `ForEachOf (Macros.each AnyPlayer)` scope holding the four `Choose (UpTo 1)`
  roles and `Macros.sacrifice Macros.They Macros.theRest`, since only there do
  the same bindings that are spent get introduced by the body (wrapping the
  sacrifice alone is refused at `ForEachOf`'s own `KeepsOuter`, probed); the
  choices gained `HasPossessor ControllerAx Macros.They`, which closes the
  chooser-scope gap noted below — [CR#700.8d] reads "each player chooses up to
  one creature they control of that type". The cost is that the printed
  "Each player …" shape is now read as the loop "For each player, …": a
  distributive deed cannot host a group spend at all, and no predicate spells
  "that weren't chosen this way", so the alternative would have dropped the
  exception clause and made the witness read a different card.
- Merge extras: `caughtInTheCrossfire`'s two hand-written five-type
  disjunctions now go through `Macros.outlaw` (a drop-in — the macro is that
  exact `Or`). Gates re-run on the merged tree: clean `./scripts/build` 46/46,
  0 Error/0 Warning, `1m20.767s` real; `cite check --list-noncompliant` 0;
  `cite check` 14200 citations, 0 stale; `cite audit --diff` 29 sites, no
  cite added or changed.

## Landing record

Gates (foreground):

- `cd idris && ./scripts/build` (after `rm -rf build`) —
  `46/46: Building Cards (src/Cards.idr)`; 0 Error lines, 0 Warning lines,
  bench brace lint green. Module count 46/46.
- `cargo xtask cite check --list-noncompliant` —
  `0 non-compliant citation-looking string(s)`.
- `cargo xtask cite check` —
  `checked 14174 citations against cr.txt (eff. 2026-08-07); 0 stale`.
- `cargo xtask cite bless` — registered [CR#700.12a] (read against its claim).
- `jj --no-pager diff --git > /tmp/po.diff && cargo xtask cite audit --diff
  < /tmp/po.diff` — `audited 20 citation site(s) — read each rule text
  against its claim`; every site read.

Pin probes (each mis-stated once, message changed, then restored):

- `badRepeatedPartyRole` with the second role changed to Rogue →
  `Error: badRepeatedPartyRole RolesAre is not a valid impossible case.`
- `badEmptyPartyRoles` with a one-role list →
  `Error: badEmptyPartyRoles RolesAre is not a valid impossible case.`
- `loneClericRogueIsPartyOfOne` restated as `= 2` →
  `Mismatch between: S (assert_total (integerToNat 0)) and 0.`

Assurance counts: restored 0, re-spelled 0, ignored-with-blocker 0, added 12
(1 model function, 3 equational witnesses, 5 card witnesses, 2 pins, 1 twin),
removed 0.

### Deviations and additions

- **`partyCount` and its three equations.** The ticket asks that "`partySize`
  of a lone Cleric Rogue is one" be recorded as a bench witness rather than a
  comment. `OneEachOf`'s denotation is computed by the game, not by the term,
  so the only compiler-checked record is a small model: `partyCount` takes each
  member as the list of role indices it could fill and returns the maximum
  assignment of distinct roles to distinct members. `clericBesideClericRogueIsTwo`
  records the greedy ban (taking the Cleric Rogue for Cleric first leaves one);
  `oneOfEachRoleIsFullParty` records [CR#700.8c]. Added beyond the ticket's
  letter because the ticket bans both wrong compositions and only the first was
  named as a witness.
- **`groupMention (OneEachOf _ _) = True`** in addition to `nounDet = Just
  BareD`. The design note says the row mirrors the bare-plural group, which
  alone gives `CountableGroup` (so `partySize`) but not `PartitiveBase`; the
  ticket's fourth macro, "a creature in your party", is a `SomeOf` partitive
  and needs it. The party is a definite computed group, so the group-mention
  reading is the right one.
- **`SoleHolder` threaded.** `partyOf`/`partySizeOf`/`fullPartyOf` carry
  `{auto 0 sh : SoleHolder who}`, forwarded to the pool's `HasPossessor
  ControllerAx who`.
- **Stick Together's controller restriction is carried by the chooser.**
  `Effect.Choose` takes its noun in `bs`, not in `agentIntro agent`, so a
  chosen noun cannot refer to the choosing player; `stickTogether` therefore
  reads "each player chooses up to one Cleric …" with "they control" implied by
  the `each AnyPlayer` chooser, then `sacrifice (each AnyPlayer) theRest`. The
  `Choose (UpTo 1)`-per-role structure the ruling is about is exact. The
  agent-scoped-choice gap is Choice-family, outside this region.
- **At Knifepoint benched as its first ability only.** `atKnifepointOutlaws`
  is `OnlyDuring Turn (Just You) (Gains (allOf outlawYouControl) …)`. The
  card's second ability needs "commits a crime" [CR#700.13], for which the
  workbench has no event — outside this region.
- **Archpriest's CDA power** is spelled `hasBasePt thisCreature partySize
  (Lit 2)`; the workbench's only base-P/T setter sets both, and 2 is the
  printed toughness.
- **Squad Commander** added as the second party-reading card (partySize in an
  amount position); Archpriest's second ability is the `fullParty` witness.
- **`cr-citations.lock`.** `cite bless` registered [CR#700.12a] and
  [CR#700.13]. Across the two bless runs it also pruned the [CR#702.147a],
  [CR#702.28a] and [CR#702.85a] entries, all three of which are still cited in
  `crates/xtask/src/facts.rs` and none of which `cite check` reported stale —
  a pre-existing bless/check scan mismatch, out of region. Those three lines
  were restored by hand so the lock diff is additive-only (two entries added,
  none removed); `cite check` reports 0 stale with them restored.

No STOP was taken.
