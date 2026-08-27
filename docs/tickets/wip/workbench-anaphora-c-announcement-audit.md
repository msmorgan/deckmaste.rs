---
needs: []
---
# What a clause announces: the intro/delta audit

Sub-round C of the five-way split of
`docs/tickets/planned/workbench-anaphora-mentions-and-creation.md`, split
2026-08-27 per the completed split analysis. **Size: L. Interlocks with A** —
every item here *adds* a singular object binding to some context, which is
exactly what makes `It` ambiguous. **Land A's rule first, or C must gate each
new mint on not creating a second `OneOf Object`.**

---

## §0. Corrections — the parent's pointers have drifted badly

The tree moved a lot since the parent ticket's body was written. Four of the
parent's item premises are dead, and one whole section is substantially
delivered. An implementer who trusts the parent will build things that
already exist. Carried verbatim from the split report so this sub-round works
from the same corrected pointers as its siblings.

| Parent says | Tree says |
|---|---|
| Consumption boundary: "`idris/src/Experimental.idr` (`effIntro`, `preIntro`, `condIntro`, `Effect.If`, `Conditionally`, … `Gets`, `DefinesPt`, `nomIntro`, `selfSubjIntro` …)" | **`Experimental.idr` is an 11-line re-export barrel** with no definitions. Real homes: `Effect.idr` (`effIntro`:1843, `preIntro`:1941, `annIntro`:2022, `replacedCtx`:2110, `otherwiseCtx`:2125, `deedDelta`:2140, `staticIntro`:486, `costIntro`:628, `Gets`:144, `DefinesPt`:149, `Gains`:168, `Intercepts`:233, `Conditionally`:267, `OnlyWhile`:279, `Search`:874, `InsteadOf`:1269, `Repeat`:1237, `Repeated`:1252, `TokenSpec`/`TokenAsThose`:66/73); `Phrase.idr` (`Noun` and **every** mention constructor — `It`:1521, `That`:1525, `TheVerbed`:1528, `ThoseVerbed`:1532, `Indefinite`:1458, `CountedGroup`:1467, `LibrarySlice`:1498, `SomeOf`:1502, `EitherOf`:1497; `nounDelta`:1610, `nomIntro`:1741, `condDelta`:2757, `selfSubjIntro`:2846, `condIntro`:2857, `moveIntro`:3148, `TokenPhrase`:2833, `GroupSize`:1878, `ThatMuch`:1831); `Triggers.idr` (`eventIntro`:516, `eventAfter`:570, `delayedCtx`:667, `UsageLimit`:790); `Words.idr` (`Payload`:948, `Binding`:964, `countOnes`:1120, `settleTargets`:1361, `itReaches`:1580, `NounWord`/`PermanentW`:1609, `onFieldZone`:1644, `wordReaches`:1696, `stampedBy`:1794, `afterShuffle`:1826, `countWord`:1894, `countTokenSpecs`:1905, `Causer`:692, `Determiner`/`SelfD`:737). |
| "`InsteadOf`, whose replacement arm is typed at `annIntro replaced` and so cannot see the batch the replaced clause made" | **Dead premise.** `InsteadOf : (replaced : Effect bs) -> (repl : Effect (replacedCtx replaced))` (`Effect.idr:1269`), and `replacedCtx e = deedDelta e ++ annIntro e` (`:2110-2116`), with `replacedCtx (Sequentially es) = annSeqs es` and `annSeqs (e::es) = deedDelta e ++ annSeqs es`. `deedDelta (Create …)` (`:2193`) mints the `AD Object ManyOf (ObjectP … (Just TokenOrigin))` binding that `countTokenSpecs` counts. **`TokenAsThose` inside an `InsteadOf` replacement arm looks writable today.** The 16-line ask must be re-probed before the sub-round is claimed. |
| "`badInsteadReadsReplacedSequenceOutcome` is the same discipline one step over" / "still refuses" | **No such pin.** Zero hits in `idris/src/`. The live pin in this family is `badAnaphoricTokenAfterNonToken` (`ProofsE.idr:331`), and it already carries the parent's own design argument in its docstring: *"'Those tokens' points at a definition of characteristics [CR#111.3], which ordinary creatures leave none of."* The definition-vs-object framing is **landed**, not open. |
| "`badSliceOfGroupPossessor`" is in "`ProofsC.idr` and `ProofsD.idr`" | It is in `Proofs.idr:72` (the base module). `SlicePossessor` gate at `Phrase.idr:2538`. |
| "the mention denoting the union over loop iterations … **Nothing in the grammar denotes that union**" | **False.** `Repeated` (`Effect.idr:1252`) exports exactly it: `effIntro (Repeated n body) = outcomeB RepeatCount :: (pluralizeDelta (effDelta body) ++ amtIntro n)` (`:1926`), and its docstring states the design verbatim — "one summary mention per mention the body introduced, same payload and same stamp — so 'the discarded cards' and 'that many' read the whole batch." `pluralizeDelta` is `Words.idr:1547`. The real gap is three sibling loops that export nothing: `effIntro (ForEachOf _ _) = bs`, `(ForEachKindOf …) = bs`, `(Repeat _) = bs` (`:1923-1925`). The item shrinks from *build a mention* to *extend an existing export*. |
| "`Conditionally : (c : Condition bs) -> (se : StaticEffect (condIntro c))` threads the condition INTO the statement, so an adverbial reading its own statement's subject is an argument-order question" (the 16 "as long as" lines) | **Dead premise — the postposed twin landed.** `OnlyWhile : (se : StaticEffect bs) -> (c : Condition (staticIntro se))` (`Effect.idr:279`), whose docstring is the ask itself: *"the condition is written after the statement and reads the statement's own subject ('has hexproof as long as IT's untapped')."* `staticIntro (Gets n …) = … ++ selfSubjIntro n` (`:488`). Bonds of Faith is probably writable now. Re-probe. |
| "`Effect.Repeat`'s until-condition cell" | `Repeat : (rep : Repetition bs)`, `Repetition = Again \| MoreTimes n \| AnyNumber` (`Effect.idr:699-703`). Correct that there is no until-condition slot; wrong that the slot would go on `Repeat` — it goes on `Repetition`. |
| "`It` has no antecedent after a discard cost: `Do (discards You This)` introduces no object binding" | The constructor is `Does` (`Effect.idr:1160`), the cost wrapper is `Cost.Do`, and **`costIntro (Do e) = effIntro e` threads fine** (`:636`). The actual hole is one clause: **`moveIntro p This z = bs`** (`Phrase.idr:3169`) — a bare `This` moved to a zone mints nothing, while `moveIntro p (AsType t n _) z` mints a `TheD` object (`:3175`). Twinshot Sniper is one clause away, not a threading redesign. |
| "`VerbName`" (three sites, incl. "No `VerbName` row is minted for 'return'") | **Not an Idris symbol.** Zero hits in `idris/src/Experimental/`. The type is `VerbLabel = String` gated by `KnownVerb` over the `verbFacts` data list (`Words.idr:764`, `788`). Restate the RETURN fence in `verbFacts` terms. |
| "`StaticEffect.DefinesLetter`" | Renamed: `StaticEffect.Define` (`Effect.idr:166`), twin of `Effect.Define` (`:1203`). |
| "the counted-group SIZE as a readable amount … (Screeching Scorchbeast, The Wise Mothman, Bruvac's 'twice that many')" (routed 2026-08-26) | **`GroupSize` exists** (`Phrase.idr:1878`, gate `countManysAny bs = 1`), has a ProofsAnaphora entry (`:583-593`), and is written on the bench today (`Cards.idr:2680, 2964, 2975, 2993` — `Times 2 GroupSize`, Doubling Season / Anointed Procession). `OutcomeSort`'s own comment (`Words.idr:653-655`) names it as the batch-size reader. Probe the three carriers before scheduling anything. |
| "`Gets`' slots are typed at `nomIntro n` and would move to `selfSubjIntro n`" | **Accurate, and still open.** `Gets : (n : Noun bs Object) -> (pow : PtShift (nomIntro n)) -> (tou : PtShift (shiftIntro pow))` (`:144`); same for `DefinesPt`'s `(amt : Amount (nomIntro n))` (`:149`). Note `staticIntro` for both *already* uses `selfSubjIntro n` — the drift is between the constructor's slot types and its own intro. |
| "`Effect.Search` takes no `Quantity` and mints no mention" (routed) | Confirmed verbatim: `Search : (who : Noun bs Player) -> (sc : SearchScope (nomIntro who)) -> (p : Predicate (nomIntro who) Object) -> …` (`:874`). No quantity, no destination. |
| "`condDelta (Happened …) = []`" (routed) | Confirmed verbatim (`Phrase.idr:2760`). Note the *precedent for fixing it* is three lines below: `condDelta (Matches (AsType t This _) _)` already mints a `SelfD` binding (`:2762`). |
| "`eventAfter` announces both participants … `countOnes Object = 1` fails" (routed) | Confirmed verbatim: `eventAfter (BecomesTarget n by) = nounDelta by ++ selfSubjIntro n` (`Triggers.idr:594`), on `Attacks`'s model (`:582`). |
| "`afterShuffle` is owner-blind" (routed) | Confirmed: `afterShuffle` filters by `survivesShuffle b` alone (`Words.idr:1826-1829`); `Payload.ObjectP` carries `(ty, zone, prov, orig)` and **no owner** (`:949`). |
| "`Effect.CantBe` … `StaticEffect.ObjectCant`" | Both exist (`Effect.idr:826`, `:183`). No drift. |
| "`badConditionAntecedent`'s case still refuses" | Exists, `ProofsB.idr:275`. No drift. |

---

## §1. Constraints binding every sub-round of this split

### 1.1 A change to an anaphor GATE costs `ProofsAnaphora.idr` — and how much depends on shape

`idris/src/Experimental/ProofsAnaphora.idr` (1016 lines) is compiled by the
package (`idris/mtg.ipkg:26`). Its five sections and the exact obligation each
change incurs:

| Section | Lines | Content | What it costs you |
|---|---|---|---|
| §1 folds | 23–91 | `countBy`/`anyBy` + `countBySplit`, `anyBySplit`, `countByWitness`, `anyByWitness`, `countByEmpty` | Nothing, **if** your gate is still a `countBy`/`anyBy` fold. A gate that is not costs a new witness lemma paralleling `countByWitness`. |
| §2 identities | 97–358 | one `…IsFold` proof per counter (`countOnesIsFold`:107, `countWordIsFold`:238, `countVerbedIsFold`:265, `countManysAny`:133, `countGroups`:283, …) | A **new counter** owes a new `…IsFold`. A **changed** counter breaks its existing one. |
| §3 per-constructor | 366–821 | a `…ReadsOnlyPrefix` typechecked witness + a `…ResolvesInPrefix` proof per gate: `It`/`They`/`Them` 381–420, `That` 423–436, `Those` 439–451, `TheVerbed` 454–471, `ThoseVerbed` 474–489, `GroupSize` 583–593, `TokenAsThose` 773–790 | A **new mention constructor** owes both. A **changed** gate owes both rewritten. |
| §4 deictics | 827–857 | `This`, `You`, `PlayerGroup`, `AttachHost`, `LetterVal` witnessed at `[]` | Anything that makes a deictic read context moves it out of §4. |
| §5 threading | 864–1016 | `nomIntroIsDeltaThenPrefix` (869, `Refl`), `condIntroIsDeltaThenPrefix` (875, `Refl`), `gateSplitsAtNomIntro` (893), `gateSplitsAtCondIntro` (907), telescope equations for `Effects`/`SimEffects`/`CostSeq`/`StaticParts`/`OnlyIf`/`If`/`OnlyWhile`/`ThisWay` (917–990), `defineIntroIsRemark` (992), `letterValDeltaIsPrefixFold` (1011) | A **delta change** that keeps `intro = delta ++ bs` re-typechecks free (the `Refl`s hold, the split lemmas compose). A delta change that breaks that shape breaks the `Refl` and everything built on it. |

**No entries exist** for `itReaches`, `stampedBy`, `settleTargets`,
`selfSubjIntro`, `eventIntro`, `eventAfter`, `moveIntro`, `predDelta`, or
`effIntro`/`preIntro`/`annIntro` as standalone lemmas — those are used inside
other statements, not proved on their own. Adding a first entry for any of them
is a real addition, not a restatement.

### 1.2 The binder contract's four clauses are the review standard

`docs/decisions/oracle-text-is-forward-anaphoric.md`, restated by every
sub-round that touches a gate or a threading function: (1) gate is a function of
`bs` alone; (2) `bs` at each argument is the reading-order prefix — every
threading function is `delta ++ bs` or an in-place re-mark (`settleTargets`,
`defineLetter`); (3) the gate resolves to a binding **in** `bs`, by counted
uniqueness or counted existence; (4) a sibling's minted binding derives from
earlier arguments only. **Widening a gate to read something other than `bs` is
an ADR change and comes with an ADR amendment.**

### 1.3 No cross-ability discourse — settled, do not reopen

`done/workbench-named-memory-channels.md` as-landed, unified with
`planned/workbench-choice-chosen-and-ascription.md`'s same-day ruling:

> NO cross-ability discourse at all — notes/tags are state on the holder;
> chosen values are CARD-SCOPE LINKAGE [CR#607.2d]: the choice ability mints a
> FORWARD OBLIGATION … checked by a face law folding forward over the
> AbilitySeq (chooser strictly before reader), not by telescope threading. …
> The forward-anaphora binder contract governs discourse within a text only,
> and survives untouched; **ProofsAnaphora is additive-only under this ruling.**

**No sub-round threads a mention between abilities.** Anything that wants one is
a `Card.idr` face law and belongs to the choice ticket.

### 1.4 Pins refuse rules-impossibility only

`done/workbench-pins-refuse-rules-impossibility-only.md`:

> the semantics layer accepts anything the Comprehensive Rules make meaningful.
> A pin … refuses a term only when a rule makes it meaningless … "No card has
> written this" is never a reason to refuse.

An overgeneration measured at zero is **recorded on the cell with its count**,
never pinned. Binds sub-round B directly (finding 1010's 4 lines).

### 1.5 Standard fences

The parent's counts are a prior session's — **re-measure from
`data/derived/cards.jsonl` filtered `jq 'select(.supported)'`**; CR text via the
`mtg-rules` skill's scripts, never memory. `idris/scripts/build` PASS, no
witness lost, no pin silently passing. Standard constraints apply.

---

## This sub-round's files

`Phrase.idr` (`condDelta`:2757, `moveIntro`:3148, `selfSubjIntro`:2846),
`Effect.idr` (`effIntro`:1843 loop rows, `Gets`:144, `DefinesPt`:149,
`If`:1183, `Search`:874), `Triggers.idr` (`eventIntro`:516),
`ProofsAnaphora.idr` §5, `Macros.idr`, `Cards.idr`.

## Owned items

- Routed (event-zone-2): *"intervening conditions announce NOTHING
  (`condDelta (Happened …) = []`), so a trigger body cannot read the condition's
  subject back as 'it' (Whirling Dervish, Dunerider Outlaw print exactly that)."*
- *"The pump's AMOUNT, 9 lines … `Gets`' slots are typed at `nomIntro n` and
  would move to `selfSubjIntro n` exactly as `staticIntro` did"* (Auramancer's
  Guise), and *"Titania's Song and the self-reading definition"* — `DefinesPt`'s
  `amt` slot, same one-line move; *"landing it benches [finding 604's reverse]
  crossing too."*
- *"The conditioned clause's own target — 1 card"* (Savage Swipe):
  *"a **target announced by the clause is chosen at cast and survives the
  conditional either way**. One card found; **measure the population before
  designing**."*
- *"The condition-first conditional — 7 cards, design already done … **Do not
  re-derive the design.** The group the tie condition announces is the counted
  domain's own denotation, minted exactly when that domain `uniquifies`,
  measured 7/7 … The `condIntro` clause was written, compiled, and backed out
  for want of a consumer; it goes back in on the round that mints the carrier."*
- The loop union export (per §0: extend `Repeated`'s `pluralizeDelta` to
  `ForEachOf`) — *"'Those tokens gain haste' (Hate Mirage) … 'Exile those tokens
  at the beginning of the next end step' (Twinflame) … 'Those tokens have
  enchant creature and …' (Smoke Spirits' Aid)"*, plus *"the loop's
  until-condition, 9 cards"* — which lands on `Repetition`, not `Repeat`.
- Routed (pins-refuse): *"Replacement-event anaphora. `eventIntro` mints no
  subject binding for the event's own noun, so Clergy's printed 'it' has no
  antecedent (consolidated open gap 7)."*
- Routed (amount-ceiling): *"'Discard up to two cards, then draw that many'
  (12 carriers) — a set ceiling plus a magnitude the discard clause must
  ANNOUNCE for 'that many' to read."* Constraint carried from the originating
  round: **the `UpTo` `Amount` row is scoped to a bare-number ceiling; this
  family is `UpToOf` over a set, so the fix is an announced magnitude, not a
  change to `UpTo`.** Probe first: a `Repeated`-shaped discard already exports
  `pluralizeDelta`, which `GroupSize` reads.
- Routed (distinct-kind-count, related): *"`Effect.Search` takes no `Quantity`
  and mints no mention … 13 of the 24 'with different names' lines write the
  constraint there"* + the DESTINATION slot (Celebrate the Harvest, Boreas
  Charger).

## Settled design — do not reopen (user-ratified 2026-08-27)

The report that split this ticket left three items as OPEN PINs. All three
are now SETTLED by the user's ruling below.

### C1 — may an open-ended loop export at all: SETTLED, do not reopen

`ForEachOf` exports its body's pluralized delta on `Repeated`'s precedent — a
domain-driven loop has a determinate iteration count, so the union is
well-formed and the three token cards land. `Repeat (AnyNumber)` does NOT
export: no bound, and its until-condition is the same missing thing. Two
decisions, ruled separately.

### C2 — does `condDelta (Happened …)` announce the subject or the whole event: SETTLED, do not reopen

The SUBJECT only, on the `condDelta (Matches (AsType t This _) _)` model three
lines below (`Phrase.idr:2762`) — one `SelfD` binding. Announcing the
complement too would make Whirling Dervish's "it" ambiguous under A's rule.
CR basis for what an intervening clause checks: [CR#603.4].

### C3 — one round or two for `Search`: SETTLED, do not reopen

`Search`'s quantity + destination slots land in one edit — the telescope is
rewritten either way, so splitting the two slots into separate rounds buys
nothing.

## ProofsAnaphora debt

Transcribed from the split report's summary table:

> §5 only, and **free if the shape holds**. `condIntroIsDeltaThenPrefix`
> (`:875`) is `Refl`: as long as the new announcement is folded into
> `condDelta`'s own output so `condIntro c = condDelta c ++ bs` still holds
> definitionally, it and `gateSplitsAtCondIntro` re-typecheck unchanged. **The
> sub-round owes an explicit restatement that this held** — that is the whole
> content of §5's audit. Same for `nomIntroIsDeltaThenPrefix` if
> `Gets`/`DefinesPt`'s slot types move.

## Sequencing

C after A — each new mint this round makes must be gated against creating an
unresolvable ambiguity under A's mechanism (i.e., it must not silently create
a second `OneOf Object` binding that A's sort-scoping cannot disambiguate).
State explicitly, per item, that this check was made.

Standard constraints apply.

- **Routed from workbench-anaphora-a-bare-it (close, 2026-08-27):** `elemIntro` drops the stamp — a loop member of a moved group loses `wasField`, costing per-member participle reads inside `ForEachOf`. An intro/delta cell, so it lands here. Also: "for each of those X, its Y" reads against the trigger's own self mention (Soul of Emancipation's remaining block — loop member + `SelfD` both battlefield-carrier); fix at the announcement side here if a principled cell exists, else it rests with the recency reserve.

- **Routed from workbench-anaphora-d-creation-and-replacement (close, 2026-08-27):** `replacedCtx` RE-ADMITS the object mention `otherwiseCtx` deliberately strips — `InsteadOf (create six kor) (destroy Them)` compiles with "them" resolving to a batch [CR#614.6] says never existed, while `otherwiseCtx` filters the same case through `outcomesOnly`. Same rule, two answers. Tension: `TokenAsThose` and `They` read the SAME binding, so tightening `replacedCtx` must not take the definition anaphor down — the fix is a determiner-aware filter or a recorded asymmetry, decided here where the context discipline lives.

- **Routed from workbench-anaphora-b-verb-provenance (close, 2026-08-27):** the plural provenance twin `ThemVerbed` (39 occ/38 cards); `riderIntro` falls through every wrapper (`Modal`/`If`/`Unless`/`Sequentially` preIntro rows export no stamp — plus WHICH mode a modal's rider reads); ETB announces its subject under `TheD` not `SelfD`; `moveIntro (AttachHost …) = bs`; Bioplasm's untyped exile mention (the `If it's a creature card` test does not re-mark the binding); `stampMoves` now over-broad (status stamps). All announcement/context cells, so they land here.

---

## As landed (2026-08-27)

Standard constraints met: `idris/scripts/build` 23/23 clean, `cargo xtask
cite check` 0 stale / 0 non-compliant, `cite audit --diff` read site by
site (one cite corrected mid-round — a superlative's denotation is not
what [CR#608.2c] says; it now cites [CR#608.2d] for the choice the
consequent makes). Corpus counts re-measured from
`data/derived/cards.jsonl` filtered `.supported`; CR text via the
`mtg-rules` skill's scripts (a rule number recalled from memory for the
loop's until-condition was caught that way before it landed — the section
it named turned out to be the Case frame, and the row now cites the
order-written rule instead).

### ProofsAnaphora debt: DISCHARGED AT ZERO

`idris/src/Experimental/ProofsAnaphora.idr` is **byte-identical**. The
round's whole delta obligation held:

- `nomIntroIsDeltaThenPrefix` (`:995`, `Refl`) and
  `condIntroIsDeltaThenPrefix` (`:1001`, `Refl`) **re-typecheck
  unchanged**. Every new announcement is folded into its own delta
  function's output: `condDelta` grew rows (`Happened`, a unified
  `Matches`, the comparison's tie set, the negated comparison) and
  `condIntro c = condDelta c ++ bs` still holds definitionally;
  `nomIntro n = nounDelta n ++ bs` was never touched — the `Gets` /
  `DefinesPt` move changed which context the SLOTS are typed at, not
  what `nomIntro` computes.
- The split lemmas built on those `Refl`s —
  `gateSplitsAtNomIntro`, `gateSplitsAtCondIntro`,
  `slotGateSplitsAtNomIntro`, `slotGateSplitsAtCondIntro`,
  `unionHalfGateSplitsAtNomIntro` — re-typecheck unchanged, as do
  `otherwiseCtxIsThenBranchOnly` (`otherwiseCtx` untouched) and the
  telescope equations, `onlyIfThreadsPrefix` included: `effIntro
  (OnlyIf …)` changed, the telescope did not.
- **§1–§4 owe nothing.** No new counter (so no `…IsFold`), no new
  mention constructor and no CHANGED gate (so no
  `…ReadsOnlyPrefix`/`…ResolvesInPrefix` pair) — every change is to what
  a cell MINTS, never to how a gate counts. No deictic was made to read
  context: `selfSubjDelta` is a function of the noun, not of `bs`.

### Landed

| Item | Cell | Witness |
|---|---|---|
| **C2** intervening condition announces its subject | `selfSubjDelta` factored out of `selfSubjIntro` (`Phrase.idr`); `condDelta (Happened _ who _ _) = selfSubjDelta who`; `Matches`' four rows collapse into the same call (its `Bindingless` gate makes the general form identical) | `whirlingDervish` re-benched with the **printed "it"**; Dunerider Outlaw prints the line word for word |
| `Gets`/`DefinesPt` slot types | `PtShift (selfSubjIntro n)` / `Amount (selfSubjIntro n)`; `Macros.gets` follows. `staticIntro` unchanged — the drift was the slots | `bondsOfFaithPump` |
| Replacement-event subject (**gap 7**) | `eventIntro`'s subject rows move to `selfSubjIntro`, as `eventAfter` already had | `clergyOfTheHolyNimbus` re-benched with the **printed "it"** |
| **C1** loop union export | `effIntro (ForEachOf grp body) = pluralizeDelta (effDelta body) ++ bs`, on `Repeated`'s precedent; no count rides along (the group's own size is what `GroupSize` reads). `Repeat` still exports nothing | `hateMirageTokens` — "For each of those creatures, create a token…. Those tokens gain haste." |
| Loop until-condition | `Repetition.Until : Condition bs -> Repetition bs` (beside `MoreTimes`, not wrapping it); exports nothing, for `AnyNumber`'s reason | 9 lines measured; 6 in scope (2 are the fenced "whichever comes first", 1 the fenced tie) |
| `stampMoves` over-broad | `Stamp` gains `moved`, written by the caller holding both zones (`setZone` compares them; the two deictic rows know their subject's); `stampMoves` reads it | a status label no longer reads as a move |
| `elemIntro` drops the stamp | new `nounProv` beside `nounZone` (+ six `provOf…` readers in `Words.idr`); `elemIntro` carries the group's stamp to its members | `soulOfEmancipation` **benches whole** |
| ETB `TheD`→`SelfD` | `moveIntro p (AsType t This _) z` mints `SelfD`, as `moveIntro p This z` already did | `nekrataalWhole` — the **printed "that creature"** writes; `selfSacrificeThenExile` writes |
| `moveIntro (AttachHost …) = bs` | the three `selfSubjDelta` rows, re-zoned and stamped | — |
| **C3** `Search` takes a quantity | `(q : Quantity (nomIntro who))` beside the description; what it finds is announced at `q`'s plurality. `Quantity` gains `ExactlyOf` (the arm `UpToOf`'s own note reserved for a measured line — 2 supported search lines) | `celebrateTheHarvest` **whole**; `boreasChargerSpell` (trigger frame elided, pre-existing reason) |
| Condition-first conditional | `tiedDelta` in `condDelta`'s comparison row: the counted domain's denotation, minted where the description `uniquifies`. A NEGATED comparison unmakes both margin and set, written out rather than filtered | `purgingScythe` **whole** — the printed tie sentence; 7 lines read "one of them" off a tie |
| `riderIntro` falls through wrappers | recurses with itself through `Enact`/`Does`/`CantBe`/`Reflexively`/`ThisWay`/`Sequentially` (new `riderIntros`) | `sequencedRider*` — 28 lines of the shape |
| Savage Swipe's conditioned target | `effIntro (OnlyIf e c oth) = annIntro e` — the condition may fail, but [CR#601.2c] chose the target at cast | `savageSwipeLine` **both sentences**; population re-measured at **5**, not the parent's 1 |

### The `replacedCtx`/`otherwiseCtx` tension: RECORDED ASYMMETRY

Decided against a determiner-aware filter, on evidence. All **14**
supported replaced-creation lines read the replaced clause back as a
DEFINITION or a MAGNITUDE ("instead create those tokens plus an
additional Food token"; "creates twice that many of those tokens
instead"); **zero** name its objects. And the object read (`Them`) rides
the SAME binding as the definition read (`TokenAsThose`) — the
discriminator is which reader asks, not what the binding records, so no
filter over determiners separates them. `replacedCtx` keeps the deed
delta because [CR#111.3]'s definition is what those 14 lines read;
`otherwiseCtx` keeps `outcomesOnly` because an untaken branch wrote no
definition. Recorded on the cell with its count of zero, per
`done/workbench-pins-refuse-rules-impossibility-only.md`.

### Probes (run before building — all three delivered)

- **`GroupSize` / counted-group size**: DELIVERED, nothing built.
  `moonlitMeditation` already writes `Create You GroupSize` under a
  `May` off a counted-batch trigger at `OncePerTurn` — structurally
  Screeching Scorchbeast; `Times 2 GroupSize` is Bruvac's "twice that
  many" (4 sites).
- **`OnlyWhile` / "as long as"**: DELIVERED. `bondsOfFaithPump` benches
  the pump line. The card's "Otherwise, …" is a second statement, not
  this row's business. Drop the parent's argument-order bullet.
- **Discard-then-draw-that-many**: DELIVERED, no new cell.
  `discardUpToTwoThenDrawThatMany` benches: the iterated-singular
  `discardN` exports its passes' batch and `GroupSize` reads its size.
  Re-measured at **21 occurrences** (not 12). The ceiling stays on the
  `UpTo` amount — a bare-number ceiling, exactly its stated scope.

### Ambiguity check against A's mechanism (per the split's sequencing rule)

Every new mint was checked. Three notes for the residue record:

1. **Hate Mirage's fourth sentence** ("Exile them") is a Perrie-class
   pair and stays unwritable: after `ForEachOf` the targeted creatures
   and the created tokens are both plural object mentions, so the bare
   plural pronoun has two candidates where "those tokens" has one. The
   mint is not suppressed; it is noted here. Twinflame prints "Exile
   those tokens" and is unaffected.
2. **`condDelta (Happened …)`** mints only on the DEICTIC rows, so a
   described subject ("if a creature dealt damage…") still announces
   only its own `nounDelta` and adds no new singular candidate.
3. **`tiedDelta`** mints one `TheD … ManyOf`. Where the prefix already
   holds a plural object mention, `Them` refuses — correct, not a
   defect. Nested under `AndCond` inside a `NotCond` the set would
   survive the negation; measured at **0** printed lines (the one
   "unless … tied for" line is a colour predicate, not a count
   comparison).
4. **`eventIntro`'s** sweep is confined to interception replacements
   (the function's only reader) and only differs for a deictic
   participant, so its blast radius is the self-referential replacement
   family.

Two pins moved, both because their premise stopped being true:
`badBareCardRead` is re-pointed at a DESCRIBED sacrifice (its two card
mentions are there, not at the source, which `SelfD` now correctly hides
from the demonstrative); `nekrataalTwoCreatureWords` becomes
`nekrataalOneCreatureWord`, which is what lets the printed rider write.
No witness was lost and no pin silently passes.

### Ledger — routed forward

- **`ThemVerbed`** (39 occ / 38 cards, routed from B): NOT built. It is
  a new mention constructor and owes a §3
  `…ReadsOnlyPrefix`/`…ResolvesInPrefix` pair, which is outside this
  round's stated "§5 only" budget. It is `ItVerbed`'s plural twin
  (`countVerbedThem` beside `countVerbedIt`) and should be a small,
  self-contained round.
- **Bioplasm's untyped exile mention** (routed from B): NOT built, and
  the finding is *why*. The "If it's a creature card" test would have to
  RE-MARK the binding in place. That cannot be a `condDelta` row —
  `condDelta` returns a delta, and an in-place re-mark rewrites the
  whole context, which breaks `condIntroIsDeltaThenPrefix`'s `Refl`.
  Per this ticket's own standard that is a defect, not a lemma to
  weaken. It needs a `settleTargets`-shaped re-mark at `condIntro`
  level, which is a design decision no settled pin covers. Note that the
  card's "it" is separately writable as `ItAt CardSlot` after A.
- **`ForEachKindOf` export**: C1 ruled on `ForEachOf`. The same
  determinacy argument applies to a kind pass, but no printed line was
  named, so it was not extended. One line if a witness appears.
- **A modal's rider**: `riderIntro (Modal …)` deliberately does not
  recurse — which mode's label the rider names is answered by the
  chooser at resolution, not by the text. Recorded on the cell.
- **`effIntro (If c e oth)`**: still `bs`. The `OnlyIf` argument does
  not carry over — a leading conditional's consequent is typed over the
  condition's own delta, which may not have held. Separate question if a
  witness appears.
- **The three-zone search's placeless find**: `searchZone
  (GraveyardHandLibraryOf _) = Nothing`, so a counted three-zone search
  announces a zoneless mention. Untouched by this round.
