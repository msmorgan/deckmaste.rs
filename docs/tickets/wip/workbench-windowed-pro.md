---
needs: []
---
**Make every read one `Pro` over a window; delete `Own` and `ItOtherThan`.**
Ruling 2026-09-04 on reads (cleanroom review 3, D-Q1; D-Q18 folds in).

- **One constructor.** `Phrase.Noun.ItOtherThan co rest {sp : bs = co ++
  rest}` and `Phrase.Noun.Own r pl own outer {sp : bs = own ++ outer}` carry
  the binding stack as term data. Both go, and the surviving read is
  `Pro : (r : Reach) -> (pl : Plurality) -> (w : Window) ->
  {auto 0 ok : countReach r pl (view w bs) = 1} -> Noun bs (reachKind r)`,
  with `Window = Whole | Top n | Below n` over a `Nat` and `view` taking or
  dropping. Counted uniqueness over the window is the only anaphora gate; no
  `bs = _ ++ _` obligation survives anywhere in the tree.
- **The number is derived, never printed.** Macros compute it:
  `ownSubject n = Pro Bare pl (Top (length (selfSubjDelta n ++ nounDelta n)))`,
  `itPrior prev = Pro Bare OneOf (Top (length (instrDelta prev)))`.
  `Macros.agentRef` re-reads the agent through the window rather than
  splitting on which reference the agent is, and `Macros.itPrior` stops
  re-quoting the whole antecedent instruction (D-Q18 — `Counters.thranduilsCompany`,
  `Counters.stunningShot`).
- **The proof machinery goes with the list arguments.** `appendAssociative` in
  `Macros.ownSubject`, the `replace {p = …} (sym nd)` in `Macros.lookedSlice`,
  and the nine obligations `Macros.scry`, `surveil` and `fateseal` each
  restate to forward them (`lookedSpilled` over `agentLookedDelta`/
  `agentLookedOuter`/`agentLookedPlur`) — one obligation on the window
  replaces the lot. The dead scry cluster listed in `workbench-dedup-tables`
  is resolved here.
- **Re-spell the sites.** The bench and pins carry 12 `Own …` spellings, 7
  `{sp = …}` handles and 5 `Refl` handles; the bench carries no implicit
  handles at all. Re-spell each as printed and re-probe every touched pin.

Size: M. Done when: `Own` and `ItOtherThan` are gone from the tree; `Pro` is
the only anaphoric noun and takes a window; no `bs = _ ++ _` obligation
remains; `grep -E '\{[a-zA-Z_]+ *= ' idris/src/Experimental/Cards/*.idr` is
empty; every re-spelled pin is probed non-vacuous; build at its module count.
Standard constraints apply, including the RON-shaped constraint.

## As landed

- **One constructor.** `Phrase.Noun.ItOtherThan` and `Phrase.Noun.Own` are
  deleted; the surviving read is
  `Pro : (r : Reach) -> (pl : Plurality) -> (w : Window) ->
  {auto 0 ok : countReach r pl (view w bs) = 1} -> Noun bs (reachKind r)`.
  `Words.Window = Whole | Top Nat | Below Nat` sits beside `Reach`, with
  `view` taking/dropping and `overWindow` re-threading a rewrite of the
  windowed stretch (used by `moveIntro`). Every `Noun` projection that split
  on the three read constructors is now one `Pro` clause over `view w bs`
  (`nounDelta`, `nounPlur`, `nounProv`, `nounZone`, `nounTy`, `nounEqRef`,
  `remarkTest`, `counterMemoryOk`, `moveDestOk`, `moveIntro`, `nounIsAbility`,
  `groupMention`, `partitiveBase`, `costNounOk`, `PileMention`). No
  `bs = _ ++ _` obligation is left on any read anywhere in the tree.
- **The number is derived, never printed.** `Macros.ownSubject n` is
  `Pro Bare (nounPlur n) (Top (length (selfSubjDelta n ++ nounDelta n)))`;
  `Macros.itPrior prev` is `Pro Bare OneOf (Top (length (instrDelta prev)))`;
  `mustBlockIt`/`attachToIt` read `Below (length (nounDelta n))`;
  `comparesOwnStat` reads `Top 1`; `dealsDamageOwnPower` reads
  `Top (length (nounDelta src))`. No `Cards/*.idr` line spells a window: the
  bench needed no edit at all, because every site already read through a macro.
- **The proof machinery.** `appendAssociative` is gone from the tree (0 uses);
  `itPrior` lost its `KeepsOuter` obligation entirely. The look cluster is
  re-indexed on (scope, count) instead of (own-list, outer-list):
  `lookedGroup`/`lookedParted`/`lookedSpilled` now take `(bs : Bindings)` and
  `(n : Nat)`, `lookedSliceDelta` became `lookedSliceCount` (+ the new
  `lookedScope`), and the three `agentLooked*` wrappers were inlined and
  deleted, so the exported look cluster went 14 -> 12 names.
  **Not done:** the `replace {p = …} (sym nd)` in `Macros.lookedSlice` and
  the eight forwarded obligations on `scry`/`surveil`/`fateseal` survive —
  see the STOP below.
- **Re-spell the sites.** All 15 `Own …`/`ItOtherThan …` spellings and all 12
  `{sp = …}`/`Refl` handles that carried a list split are re-spelled or gone;
  `grep -E '\{[a-zA-Z_]+ *= ' idris/src/Experimental/Cards/*.idr` is empty
  (build script gate). Pins re-probed: `badOwnEmptyDelta`, `badOwnTwoInDelta`,
  `badSharedSubjectTwoInDelta` (ProofsAnaphora), `badSharedSubjectEmptyDelta`
  (ProofsKeyword) — each mis-stated once and each answered
  `… Refl is not a valid impossible case`. Positive twin
  `okOwnReadsOneInDelta` re-spelled to the window and still admits.
- **D-Q18 — not done.** `Macros.itPrior` still takes the antecedent
  `Instruction`, so `Counters.thranduilsCompany` and `Counters.stunningShot`
  still re-quote it. Reason: the window is `Top (length (instrDelta prev))`,
  and `instrDelta prev = take (length (instrIntro prev) - length bs) …`, so
  the count needs the clause's OUTER scope. Only the antecedent instruction
  determines that scope by unification — a shorter anchor (the clause's noun)
  leaves `outer` unsolved, and a literal `Top n` on the bench would print the
  number the ticket forbids. The obligation count did drop from two to one.

## Landing record

Measured on change `pkovtwww` (working copy), parent `pznwukpq fd55a2fc`.

Numbers before/after (`idris/src/Experimental/`):

| | before | after |
| --- | --- | --- |
| `Noun` constructors | 25 | 23 |
| `Own …` / `ItOtherThan …` spellings | 15 | 0 |
| list-split `{sp = …}` / `Refl` handles on reads | 12 | 0 |
| `appendAssociative` uses | 1 | 0 |
| exported look-cluster names | 14 | 12 |
| modules built | 46 | 46 |

Gate lines:

- `cd idris && rm -rf build && ./scripts/build` -> exit 0, last line
  `46/46: Building Cards (src/Cards.idr)`, no `Error` and no `Warning` line;
  2m24.9s clean.
- `cargo xtask cite check --list-noncompliant` -> `0 non-compliant
  citation-looking string(s)`.
- `cargo xtask cite check` -> `checked 14309 citations against cr.txt
  (eff. 2026-08-07); 0 stale`.
- `jj --no-pager diff --git | cargo xtask cite audit --diff` -> `audited 0
  citation site(s) — nothing selected` (the round adds no citation).

Assurance counts: restored 0; re-spelled 16 (`itOtherThanReadsOnlyPrefix`,
`itOtherThanResolvesInPrefix`, `itPriorReadsOnlyPrefix`,
`itPriorResolvesInPrefix`, `ownReadsOnlyPrefix`, `ownResolvesInPrefix`,
`itAtReadsOnlyPrefix`, `itTokenReadsOnlyPrefix`, `thatHalfReadsOnlyPrefix`,
`theVerbedReadsOnlyPrefix`, `thoseVerbedReadsOnlyPrefix`,
`turnInScopeReadsOnlyPrefix`, `okOwnReadsOneInDelta`,
`badSharedSubjectTwoInDelta` in ProofsAnaphora, `badCreatureHalfRead` in
ProofsDamage, and `PileMention`'s two constructors as one entry); ignored 0;
added 4 (`elemInTake`, `elemInDrop`, `elemInView`, `proResolvesInWindow` —
proofs that a read only ever names a binding its own window holds); removed 0.

Deviations and additions:

- Added `Words.Window`, `Words.view`, `Words.overWindow` (the ticket names the
  first two; `overWindow` is the rewrite counterpart `moveIntro` needs, since
  a move must re-thread the untouched half of the stack).
- Added the four window proofs listed above; they replace the
  `elemInPrefix`/`elemInSuffix` reasoning the deleted split theorems used.
  `elemInPrefix`/`elemInSuffix` themselves are kept (other theorems use them).
- Deleted `Macros.agentLookedDelta`, `agentLookedOuter`, `agentLookedPlur`
  (inlined at their four use sites each) and renamed `lookedSliceDelta` ->
  `lookedSliceCount`, adding `lookedScope`. Beyond the ticket's letter; it is
  the "dead scry cluster" bullet, and it is a net -2 on the exported surface.
- `Effect.counterpartNotSelf` and one `ProofsDamage` term needed a one-token
  re-spell for the new arity (`Pro Bare OneOf Whole`). `Effect.idr` is
  otherwise untouched.
- Behaviour of the merged constructor is pinned to the old three where they
  disagreed: `nounEqRef`, `remarkTest`, `groupMention` and `moveDestOk` match
  `Whole` explicitly so a windowed read keeps the old `Own`/`ItOtherThan`
  answer, and `partitiveBase` is `True` exactly on `Top` (the old `Own` row).

STOP taken (and its resolution): the ticket asks that `Macros.agentRef`
re-read the agent through the window "rather than splitting on which reference
the agent is", and that `lookedSlice`'s `replace` go. Both are unreachable as
written. An agent that introduces no binding of its own — `You` and `They`,
which is 14 of the 17 `scry`/`surveil`/`fateseal` witnesses — has an empty
window, and `countReach r pl (view (Top 0) bs) = 0`, so no windowed `Pro` can
spell a deictic subject; `Whole` fails too (at a spell body the stack holds no
player binding at all). Resolution, reported rather than resolved silently:
`Macros.agentSelfOrOwn` keeps one split, but on the DELTA LIST (`[]` vs
`d :: ds`), never on which noun the agent is — the deictic branch writes the
agent again, the anaphoric branch is now
`Pro (Word PlayerW) pl (Top (length (d :: ds)))`, and `Macros.OwnRefOk` states
its obligation over the window. Because that split survives,
`nounDelta (agentRef …)` does not reduce to `[]` by computation, so the `nd`
obligation and the single weakening `replace` in `Macros.lookedSlice` stay,
and with them the forwarded `ps`/`tr`/`pr`/`ke` obligations on the three look
macros. Everything the window CAN do it does: the nine-obligation restatement
is now indexed on (scope, count) rather than on two binding lists, and no
`bs = _ ++ _` proof survives anywhere.
