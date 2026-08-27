---
needs: []
---
# The creation clause and the replacement side

Sub-round D of the five-way split of
`docs/tickets/planned/workbench-anaphora-mentions-and-creation.md`, split
2026-08-27 per the completed split analysis. **Size: M — but see §0; it may
be S once re-probed.** Independent of A, B, C.

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

`Effect.idr` (`TokenSpec`:66, `InsteadOf`:1269, `replacedCtx`:2110,
`Intercepts`:233, `Create`), `Phrase.idr` (`TokenPhrase`:2833),
`Words.idr` (`Causer`:692), `Triggers.idr` (`UsageLimit`:790),
`Events.idr` (`ReplUse`:322), `ProofsE.idr`, `Cards.idr`.

## Owned items

- *"The definition channel — what `InsteadOf` withholds, 16 lines"* — Conqueror's
  Pledge and 15 kin; *"Decide whether `Payload`'s object arm grows a definition
  flag or the announcement channel gains a second row."* **Per §0 this premise
  is stale**; the item's first task is the probe (see D0 below).
- *"The described-ability causer — 3 lines, plus the singular anaphor"* (Rain of
  Gore, Unpredictable Cyclone, Zabaz) — *"two of the three write the
  periphrastic verb 'cause … to' rather than the event's own"*; and *"Zabaz is
  ALSO the sole crossing of finding 582's voice covariance … the line that lands
  it is the line that turns `Intercepts`' derived body voice into a slot."*
- *"The SINGULAR anaphor rides here too: 'instead create that token and a
  Treasure token' (Mr. House …) is … attested, so `TokenAsThose`'s plural demand
  is **not pinned** (finding 596)."*
- *"the relative-clause determiner — Crafty Cutpurse's … one line, a determiner
  `TokenPhrase` has no constructor for."*
- *"The 'first time … each turn' replacement word — 3 lines"* (Esix, Mirrormind
  Crown, Moonlit Meditation) — *"it is the replacement-side twin of the landed
  once-each-turn trigger rider (`UsageLimit`)."*

## Binding rulings

[CR#614.6] (the replaced event never happened) and [CR#111.3] (the token's
"text") are both already carried by the live pin `badAnaphoricTokenAfterNonToken`
(`ProofsE.idr:331`) — **read that docstring before designing anything; the
definition-vs-object distinction is landed.** Acceptance stays: *"the
definition channel carries the characteristic definition without relaxing
`annIntro`'s withholding of the object mention"* — restate the pin's name
correctly (per §0, it is `badAnaphoricTokenAfterNonToken`, not
`badInsteadReadsReplacedSequenceOutcome`, which does not exist). Prismari
Pianist is the cheapest witness; the five non-instead lines stay blocked on
dice/quoted abilities/mana.

## Settled design — do not reopen (user-ratified 2026-08-27)

The report that split this ticket left three items as OPEN PINs (one of them
blocking). All three are now SETTLED by the user's ruling below.

### D0 (blocking) — is the 16-line definition-channel item already delivered: SETTLED, do not reopen

This probe is the FIRST ACT of the round — run it before any other work is
claimed. `replacedCtx e = deedDelta e ++ annIntro e` passes the token binding
through; probe one witness — `InsteadOf (Create six …) (Create twelve
TokenAsThose …)` — against `idris/scripts/build`. If it compiles, D shrinks to
the causer + `TokenPhrase` determiner + once-per-turn cap, and the
definition-channel acceptance bullets move to the dispositions table as
**delivered**; record that outcome explicitly in this ticket before continuing.
(The report suggested running this probe as the split's last act, before
dispatch; that did not happen before this ticket was cut, so the settled
ruling moves it to the round's first act instead.)

### D1 — the once-per-turn cap: SETTLED, do not reopen

Reuse `UsageLimit` (`Triggers.idr:790`, `OncePerTurn | OncePerGame`); do not
mint a third `ReplUse` ending (`Repeatedly | NextTimeOnly`, `Events.idr:322` —
the two [CR#614.3] endings, and a cap is neither). If reuse is impossible the
round **records why**, per the parent's acceptance.

### D2 — `TokenAsThose`'s plural demand: SETTLED, do not reopen

Drop it deliberately, Mr. House recorded as the attested singular (finding
596). The gate `countTokenSpecs bs = 1` counts `ManyOf` bindings
(`Words.idr:1905-1909`); admitting `OneOf` is a two-clause change. The
coordination of two specifications and [CR#111.10]'s predefined name are
**separate unbuilt things** — vocabulary-only fence applies.

## ProofsAnaphora debt

Transcribed from the split report's summary table:

> Only if D2 lands: §3 `tokenAsThoseReadsOnlyPrefix` (`:773–790`) and
> `noTokenAsThoseWithoutAntecedent` rewritten for the admitted singular.

D2 is settled as landing (above), so this debt applies.

## Sequencing

B/D/E parallel-safe with A and with each other. D0's probe is this round's
own first act, ahead of everything else in D.

Standard constraints apply.
