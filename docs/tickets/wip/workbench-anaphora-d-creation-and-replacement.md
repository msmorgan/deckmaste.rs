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

---

## As landed (2026-08-27)

### D0's probe — RUN FIRST, GREEN. The definition-channel item is DELIVERED.

The probe was the round's first act, exactly as the settled ruling directs.
`InsteadOf (Create six 1/1 white Kor Soldier tokens) (If <kicked> (Create
twelve TokenAsThose) Nothing)` **typechecks against `idris/scripts/build`
unchanged**. `replacedCtx e = deedDelta e ++ annIntro e` threads the
token-origin binding `deedDelta (Create …)` mints, and `countTokenSpecs`
counts it: the parent's "`InsteadOf`'s replacement arm cannot see the batch
the replaced clause made" premise is dead, as §0 said.

Not vacuous — two controls were run:

| control | result |
|---|---|
| `InsteadOf (Draw You (Lit 2)) (Create You (Lit 3) TokenAsThose [])` | **refused** — "Can't find an implementation for `countTokenSpecs [] = 1`" |
| `InsteadOf (create six kor) (destroy Them)` | **accepted** — see the ledger; the object mention is readable in the replacement arm |

So the parent's five acceptance bullets for *"the definition channel — what
`InsteadOf` withholds, 16 lines"* move to **delivered**: `annIntro`'s
withholding of the object mention is untouched (`annIntro (Create …) =
specDelta spec ++ amtIntro count`, no object binding), and the definition
travels on `deedDelta`/`replacedCtx` instead. `Payload`'s object arm grew no
definition flag and the announcement channel gained no second row — neither
was needed. **Nothing was built for this item**; two card witnesses were
written to hold the fact down (below).

The consequence for the round: D shrank exactly as D0 said it would, and the
round's remaining budget went to D1, D2, and the re-measurement that showed
the last two items are blocked at their root.

### Per-item dispositions

| item | disposition |
|---|---|
| definition channel (16 lines) | **DELIVERED before the round** — probe green; witnessed by `conquerorsPledge`, `prismariPianist` |
| D2 — `TokenAsThose`'s plural demand | **LANDED** — gate widened, ProofsAnaphora §3 debt paid, witnessed by `prismariPianist` |
| D1 — "first time … each turn" cap | **LANDED** — `Intercepts` grew `(limit : Maybe UsageLimit)`, witnessed by `moonlitMeditation` |
| described-ability causer (3 lines) | **NAMED, not landed** — blocked at its root, three ways; ledgered below |
| `TokenPhrase` relative-clause determiner (1 line) | **NAMED at its one-line size** — blocked on a construction that does not exist; ledgered below |

### D2 — the singular admission

`countTokenSpecs` (`Words.idr`) dropped `ManyOf` from its object clause: the
gate counts a DEFINITION [CR#111.3], and plurality is not one of the fields a
definition has. Two clauses moved, as the settled ruling predicted — the gate
and its head-type reader:

- `countTokenSpecs (MkBinding _ _ ManyOf (ObjectP _ _ _ og) :: bs)` →
  `(MkBinding _ _ _ (ObjectP _ _ _ og) :: bs)`.
- `specHeadTy TokenAsThose` moved from `tyOfThose TokenW bs` (plural-only) to
  a new plurality-blind twin `tyOfThoseAny` beside `tyOfThat`/`tyOfThose`.
  Without this the anaphor would have satisfied the gate on a singular
  antecedent and then read back no head type.

**The attestation is stronger than the ticket recorded.** Mr. House is the
singular ANAPHOR ("instead create that token") and stays blocked on the
coordination of two specifications and [CR#111.10]'s predefined name, exactly
as the vocabulary-only fence says. But **Prismari Pianist is a singular
ANTECEDENT** — "create a 1/1 blue and red Elemental creature token. If that
spell's mana value is 5 or greater, create three of THOSE TOKENS instead" —
and it was *unwritable* before this change and is written on the bench now.
Dropping the plural demand therefore pays a printed line immediately, not
only a fenced one. The one constructor spells both agreements ("those tokens"
after a batch, "that token" after one); that is spelling, not a second word.

ProofsAnaphora §3 debt paid. `tokenAsThoseReadsOnlyPrefix` and
`noTokenAsThoseWithoutAntecedent` keep their types (the gate's *shape* did not
change, only which bindings it counts), their prose is corrected from "three
fields" to "two", and three new checked witnesses record what moved:

- `oneTokenIsOneSpec` — a singular token binding satisfies the gate. **This is
  the line that would not have compiled before the change.**
- `manyTokensAreOneSpec` — the plural half still does.
- `oneNonTokenIsNoSpec` — a non-token object still leaves no definition,
  whatever its plurality, which is the fact the live pin
  `badAnaphoricTokenAfterNonToken` (`ProofsE.idr`) spells as a card. **That
  pin still refuses**, verified on a from-scratch build.

### D1 — the once-per-turn replacement cap

`UsageLimit` reused, per the settled ruling; no third `ReplUse` ending was
minted. `Intercepts` grew a fifth explicit slot:

```idris
Intercepts : (ev : GameEvent bs) -> (alts : List (GameEvent bs)) ->
             (repl : Effect (interceptCtx alts ev)) ->
             (use : ReplUse) ->
             (limit : Maybe UsageLimit) ->
             {auto 0 ok : Interceptable ev} ->
             {auto 0 oks : InterceptableArms alts} -> StaticEffect bs
```

placed after `use` on `Triggered`/`Activated`'s own model, which both carry
`(limit : Maybe UsageLimit)` in the same position. The two words say different
things and are written together: [CR#614.3] governs how long the replacement
STANDS, the rider governs how OFTEN it may apply while it stands — Reed
Richards writes both at once ("The first time you would draw a card each
turn … you draw four cards instead").

**No gate.** `NextTimeOnly` beside a per-turn cap says one thing twice, which
is redundant and not meaningless, so it is admitted on `CreatedByUnder`'s
model rather than pinned (§1.4).

**Re-measured from `data/derived/cards.jsonl` (§1.5), and the ticket's count
was low.** The family is **5 supported lines**, not 3: the three creation
lines the ticket names (Esix Fractal Bloom, Mirrormind Crown, Moonlit
Meditation) plus **two draw-event lines** — Reed Richards, Smartest Man and
Scion of Halaster. That matters, because two event families crossing the cell
is what shows the cap is a rider on `Intercepts` rather than a token-creation
spelling. `OncePerGame` on a replacement is **0 lines** — recorded here with
its count, not pinned, and admitted with the rest of the reused word.

24 `Intercepts` call sites updated (19 `Cards.idr`, 2 `Macros.idr`, 1
`ProofsE.idr`, 1 `ProofsF.idr`, plus `staticKind`/`staticIntro`).

### Witnesses written

| card | what it holds down |
|---|---|
| `conquerorsPledge` | D0: the definition channel across `InsteadOf`, plural antecedent |
| `prismariPianist` | D2: the singular antecedent, unwritable before this round |
| `moonlitMeditation` | D1: the cap, `Repeatedly (Just OncePerTurn)` |

### Why the last two items were named rather than landed

Both were re-probed against the tree before the decision, and both turn out to
be blocked *below* the vocabulary the ticket names — the item is not the
missing word, the missing word is the visible end of something bigger. The
vocabulary-only fence applies to both.

**The described-ability causer.** `Causer = AnEffect` is nullary and
context-free, so widening it to carry an `AbilityClass` looks like one line.
It buys nothing, three ways:

1. **Zabaz needs a keyword-catalog row.** "a modular triggered ability" would
   be `KeywordClass "Modular"`, and `KnownKeyword "Modular"` is **false** —
   Modular is absent from `verbFacts`' keyword table. Adding it is a
   keyword-catalog change with its own facts columns (`docs/keyword-policy.md`
   territory), not anaphora vocabulary. `AbilityClass` also has no *triggered*
   row: its four are `AnyOnStack`/`AnyActivated`/`LoyaltyClass`/`KeywordClass`
   (the closure table's "3 rows" is stale).
2. **Zabaz needs the voice slot at the same time.** Its body is passive
   ("that many plus one +1/+1 counters ARE PUT on it instead") while its
   header names an ability agent — the 1-of-60 crossing of the voice
   covariance. So landing the causer alone yields a writable HEADER and an
   unwritable BODY: still zero cards. The two changes are coupled, and
   together they are two vocabulary changes for exactly one line.
3. **The other two lines need a verb that does not exist.** Rain of Gore and
   Unpredictable Cyclone both write the periphrastic "would CAUSE its
   controller to gain life" / "would CAUSE you to draw a card" rather than the
   event's own verb, and neither the life-gain nor the draw event carries a
   causer slot at all (`CausedBy` sits on `CounterEvent`, `CreationVoice` on
   `TokensCreated`; that is all). A third line, **Silhouette**, writes the
   same periphrasis into the `Prevents` family — so the periphrastic verb is a
   4-line construction of its own, and it, not `Causer`'s arity, is what those
   lines are waiting for.

**The `TokenPhrase` relative-clause determiner.** `TokenPhrase (Each p)` is
three lines of Idris and has **no writable witness**. Crafty Cutpurse is the
only line in the corpus writing "each token …" (re-measured: every other
token-creation header writes "one or more tokens" or "a token"), and what
makes its line real is the relative clause — "each token THAT WOULD BE CREATED
under an opponent's control this turn". No prospective predicate exists:
`HappenedTo` is retrospective, over a `Lookback`. Strip the relative clause
and the remainder is a different card ("each token is created under your
control instead" would take your own tokens too), so there is no reduced form
to write. Landing the determiner arm would add a constructor no card can
reach; naming it at its one-line size is what the ticket authorises.

### Ledger — route these

1. **`replacedCtx` re-admits the object mention that `otherwiseCtx`
   deliberately strips.** Measured, green, and the sharpest thing this round
   turned up. `otherwiseCtx e = outcomesOnly (deedDelta e) ++ annIntro e`,
   whose docstring says outright that "a token it would have created names
   nothing here"; `replacedCtx e = deedDelta e ++ annIntro e` does **not**
   filter, and `itReaches` is determiner-blind, so `InsteadOf (create six kor)
   (destroy Them)` **compiles today** — "they" resolving to a batch [CR#614.6]
   says never existed. Both contexts sit under the same rule, and they answer
   it differently. This is the real form of the parent's definition-channel
   question: `TokenAsThose` and `They` read the same binding, so the
   definition channel and the object channel are not separated, and
   `replacedCtx` cannot be tightened to match `otherwiseCtx` without taking
   `TokenAsThose` down with it. Separating them (a definition flag on
   `Payload`'s object arm, or a second announcement row) is the parent's own
   phrasing of the fix, and it is a real ticket. **Out of scope here: D0 is
   settled do-not-reopen, and the fence is vocabulary-only.**
2. **The periphrastic "would cause X to Y" verb** — 4 supported lines (Rain of
   Gore, Unpredictable Cyclone, Silhouette, and the causer slot Zabaz needs
   beside it). Blocks the described-ability causer; wants its own round with
   the life-gain and draw events' causer slots.
3. **`Intercepts`' body voice as a slot** — finding 582's 1-of-60 crossing,
   Zabaz. Coupled to (2); neither is writable alone.
4. **"Modular" as a keyword-catalog row** — `KnownKeyword "Modular"` is false;
   `AbilityClass` has no triggered row. Keyword-policy territory.
5. **The prospective "that would be created" predicate** — Crafty Cutpurse.
   Unblocks `TokenPhrase (Each p)`, which should land in the same change as
   the predicate rather than ahead of it.
6. **`OncePerGame` on a replacement: 0 lines** — recorded with its count per
   §1.4, admitted with the reused word, not pinned.
