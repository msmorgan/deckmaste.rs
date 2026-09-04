---
needs: []
---
**Refuse a distributive deed whose body mutates the enclosing stack.** Residue
of `workbench-loop-delta` (2026-09-04): the loop twin is closed by
`KeepsOuter`, but `Macros.exile (Macros.each Opponent) (Macros.It OneOf)`
followed by `untap (It OneOf)` is still admitted — `doesInstrIntro ManyOf` /
`distributedDelta` republish `nomIntro s` over a mutated stack. Guarding
`Enact` directly refuses the plurality-polymorphic macros at their definitions
(`exile`, `sacrifice`, `discard`, `puts`, `mills`, `scry`, `surveil`,
`shuffleInto`) because `nounPlur agent` is abstract there; the fix threads the
`KeepsOuter`-style obligation through that macro family.

Size: M. Done when: the sentence above is refused by a pin (non-vacuous), the
eight macros keep their surface, every bench use still typechecks, build at
its module count. Standard constraints apply.

## As landed

- `Effect.KeepsOuterOf outer out = out = take (length out - length outer) out ++ outer`
  — `KeepsOuter`'s proposition, lifted off `Instruction`'s own index so the
  same shape can be demanded of a stack that is not `bs`. `KeepsOuter {bs} e =
  KeepsOuterOf bs (instrIntro e)`; the loop obligations are unchanged and were
  re-probed.
- `Effect.KeepsOuterEach : Plurality -> Bindings -> Bindings -> Type` —
  `OneOf` is `()`, `ManyOf` is `KeepsOuterOf`. `Effect.EnactKeepsOuter subj e`
  is `()` for `Nothing` and `KeepsOuterEach (nounPlur s) (agentIntro s)
  (instrIntro e)` for `Just s`.
- `Enact` carries `{auto 0 ke : EnactKeepsOuter subj e}`. A distributive deed
  whose body mutates the agent stack — a `Move`/`SetStatus` reaching an object
  bound outside the agent phrase — no longer typechecks, so
  `doesInstrIntro ManyOf` can never republish `nomIntro s` over a stack the
  body has already changed. The obligation is stated over `instrIntro e`, the
  same value `distributedDelta` slices in the `Move`/`SetStatus` clauses and
  that `deedDelta e ++ nomIntro s` is built beside in the general one, so
  `doesPreIntro`, `doesRiderIntro` and `doesAnnIntro` — which republish
  `nomIntro s` over the same stack — are covered by the same premise.
- Because `nounPlur agent` is abstract at a plurality-polymorphic macro's
  definition, the obligation is threaded, one erased `ke` each, through
  `Macros.exile`, `sacrifice`, `discard`, `puts`, `mills`, `scry`, `surveil`,
  `shuffleInto` — plus `sacrificeIt`, a thin wrapper over `sacrifice`. Every
  macro keeps its surface; no call site in the 46-module tree needed a change.
- `Macros.scryBody` / `Macros.surveilBody`: `scry` and `surveil` dispatch on
  `LookReq`, so their four clause bodies were factored into named
  `public export` functions and the macros became single-clause. That lets the
  shared signature name the body it enacts —
  `EnactKeepsOuter (Just agent) (Macros.scryBody agent amt req)` — instead of
  four different reduced obligations. No clause body changed.
- `ProofsZone`: `okDistributedMovesItsOwn` (positive twin — "Each opponent
  exiles a creature they control. Put those cards onto the battlefield.": the
  deed moves only what the agent phrase introduced, and the exiled cards read
  back after it as a plural) and `badDistributedZoneMoveRead` (the ticket's
  sentence, refused at `Enact`'s obligation). Citations `[CR#400.7,701.26b]`.

## Landing record

Numbers before/after: 46/46 modules both. Clean whole-model build
`1m56.152s` real / `1m49.944s` user after, `1m56.984s` / `1m49.637s` before.
`Experimental.Macros` check (full build, then its `.ttc` removed and rechecked,
three runs): before `4.913s` / `3.225s` / `4.696s` real, after `6.573s` /
`5.458s` / `7.268s` real — about +2s, the cost of elaborating each threaded
macro's obligation type.

Gates (all foreground):

- `cd idris && ./scripts/build` → exit 0, last line `46/46: Building Cards
  (src/Cards.idr)`, 0 lines matching `Error|Warning`.
- `cargo xtask cite check --list-noncompliant` → `0 non-compliant
  citation-looking string(s)`.
- `cargo xtask cite check` → `checked 14142 citations against cr.txt
  (eff. 2026-08-07); 0 stale`.
- `cargo xtask cite bless` → `blessed 1410 rules at cr_date 2026-08-07`. It
  registered nothing new for this diff; its only effect on
  `cr-citations.lock` was dropping three keyword-ability rule entries that
  nothing in the tree cites any more — pre-existing drift from another round,
  not this one, so the lock is left unchanged here.
- `jj --no-pager diff --git > /tmp/dm.diff && cargo xtask cite audit --diff <
  /tmp/dm.diff` → `audited 1 citation site(s)`. [CR#400.7] ("An object that
  moves from one zone to another becomes a new object with no memory of, or
  relation to, its previous existence") is why the pre-deed battlefield
  binding is stale once the creature is exiled; [CR#701.26b] ("Only tapped
  permanents can be untapped") is the gate the singular spelling trips one
  clause later. Both point the direction they are cited for.

Probes (scratch module under `src/Experimental/`, deleted after each run):

- before: `Sequentially [tap (target creature), exile (each Opponent) (It
  OneOf), untap (It OneOf)]` typechecked.
- after: it fails at `exile`, the message showing `setZoneReach Bare OneOf
  Nothing (Just Exile) (instrIntro (tap (target creature)))` on the left of the
  obligation against the un-mutated stack on the right;
  `okDistributedMovesItsOwn` typechecks.
- `scry (each Opponent) (Lit 1)` and `surveil (each Opponent) (Lit 1)` still
  typecheck — their bodies introduce their own looked-at slice and leave the
  agent stack alone — so the threading refuses mutation, not plurality.
- `sacrificeIt (each Opponent)` over an outer battlefield creature is now
  refused at the same obligation ("each opponent sacrifices it", one shared
  outer permanent).

Non-vacuity:

- `badDistributedZoneMoveRead`: replacing `It OneOf` with `a creature` (a body
  that introduces what it moves) turns the message into
  `badDistributedZoneMoveRead Refl is not a valid impossible case.`
- `okDistributedMovesItsOwn`: reading the distributed group back as `It OneOf`
  instead of `It ManyOf` fails on `countReach Bare OneOf (instrIntro (exile
  (each Opponent) …)) = 1`, so the twin exercises the pluralized
  republication, not just the constructor.
- re-probed after the `KeepsOuter` re-expression: giving both loop pins a
  non-mutating body yields `badLoopedZoneMoveRead Refl is not a valid
  impossible case.` and `badKindLoopZoneMoveRead Refl is not a valid
  impossible case.`; `badDistributedDiscardSingular` (ProofsAnaphora) with
  `ManyOf` in place of `OneOf` yields `badDistributedDiscardSingular Refl is
  not a valid impossible case.`

Assurance counts: restored 0, re-spelled 0, ignored 0, added 2
(`okDistributedMovesItsOwn`, `badDistributedZoneMoveRead`), removed 0.

### Deviations and additions

- Addition: `sacrificeIt` is threaded too, a ninth macro beyond the ticket's
  eight. It is a wrapper over `sacrifice` with the same abstract agent, so the
  build demands it; its subject is a pronoun, which makes it the second live
  refusal the guard produces.
- Addition: `Macros.scryBody` and `Macros.surveilBody` are new `public export`
  names. They exist because `scry`/`surveil` dispatch on `LookReq` and a single
  shared obligation cannot name four different clause bodies; factoring is the
  alternative to hand-transcribing four reduced `Bindings` expressions into the
  signature. The macro surface is unchanged.
- Deviation: `KeepsOuter` is re-expressed through `KeepsOuterOf` rather than
  left alone. Same proposition (`instrDelta` unfolds to the same `take`), but
  the obligation for `Enact` is over `agentIntro s`, not over the instruction's
  own index, so the shape had to be nameable independently. Both loop pins were
  re-probed.
- Deviation: no new `Cards/` entry. The positive witness lives beside its pin
  in `ProofsZone` (house rule) and names the printed pattern it spells — the
  distributive form of Breach the Multiverse's middle pair, the same sentence
  `okLoopMovesItsOwn` spells as a loop.
- No STOP.
