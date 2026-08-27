---
needs: []
---
# What a bare "it" resolves to — sort-scoped anaphor resolution

Sub-round A of the five-way split of
`docs/tickets/planned/workbench-anaphora-mentions-and-creation.md`, split
2026-08-27 per the completed split analysis. **Size: L. The architectural
round. Everything else can run without it; it should run first.**

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

`Words.idr` (`countOnes`:1120, `countWord`:1894, `itReaches`:1580,
`wordReaches`:1696), `Phrase.idr` (`It`:1521, `That`:1525, `moveIntro`:3148),
`ProofsAnaphora.idr` §2+§3+§5, `Macros.idr` (`splitOverPlaneswalker`:97),
`Cards.idr`.

## Owned items

- *"English resolves a bare 'it' by preferring the subject, where this grammar
  refuses two object mentions outright. That was an over-generation and is now a
  **blocker**"* — Bioplasm.
- Routed (event-zone-3): *"bare 'it' after a becomes-target header is refused —
  `eventAfter` announces both participants, so `countOnes Object = 1` fails when
  the targeter is object-kinded; costs '…, sacrifice it' (18 lines, the largest
  body family). The proposed fix is an anaphora decision: let `It` PREFER the
  `SelfD` mention over a non-self singular."*
- Routed (distinct-kind-count): *"`Gains` introduces no readable object binding
  when the trigger already bound a subject — `It` finds two singular objects
  (Perrie, the Pulverizer)."*
- Routed (payment-events): *"Heart of Bogardan's BODY is blocked card-level on
  demonstrative uniqueness — the header announces the non-payer, so the split
  read … finds a second singular player mention and `countWord PlayerW bs = 1`
  fails."*
- Routed (ability-word-primitive): *"`It` has no antecedent after a discard
  cost"* — Twinshot Sniper (per §0, one `moveIntro` clause).
- *"The permanent word after a zone change — a recorded TENSION, not a gap"* —
  Soul of Emancipation; `wordReaches PermanentW` demands `onFieldZone`. It is a
  `countWord` reach decision, so it belongs with the demonstrative here.
  *"This round owns the decision … Do NOT patch it from the binder's side."*
- *"Measure `It`'s other readers before widening — it is the most-read noun in
  the grammar."*

## Settled design — do not reopen (user-ratified 2026-08-27)

The report that split this ticket left three items as OPEN PINs. All three are
now SETTLED by the user's ruling below. **Do not re-derive or re-litigate
these; implement them.**

### A1 (the big one) — sort-scoped anaphor resolution: option (c)+(b)

The consuming verb's own rule bounds its slot's candidates (e.g. sacrifice
takes a permanent — verify [CR#701.21a] wording in-round), implemented as
sort/zone-scoped anaphor reads (`countOnesAt`-style counted-uniqueness gates
over the rule-admitted candidate subset) selected AT THE MACRO LAYER per the
macro-only-carrier doctrine — never a find-first preference. The gate stays
counted uniqueness, so the forward-anaphora ADR's clause 3 gets at most a mild
refinement ("counted uniqueness among candidates the verb's rule admits"), not
a third resolution form.

Residue policy = (b): same-sort ambiguities (Perrie-class — both candidates
battlefield creatures) stay REFUSED; those lines write definite descriptions.
The round MEASURES the same-sort residue's size and records it;
clause-recency (the splitter's option (a), with its analysis below) is
recorded as held-in-reserve if the residue proves large.

**Evidence record — transcribed verbatim from the split report's
three-carrier table** (this is why a flat single tiebreak rule was rejected in
favor of sort-scoping):

| Card | Bindings in scope | Intended referent | `SelfD`-preference gives | recency (list head) gives | "prefer the subject" gives |
|---|---|---|---|---|---|
| becomes-target, 18 lines ("…, sacrifice it") | `nounDelta by` ++ `SelfD` (`Triggers.idr:594`) | the self | ✅ self | ❌ the spell | ✅ self |
| **Bioplasm** — "Whenever this creature attacks, exile the top card of your library. If **it**'s a creature card, …" | `SelfD` (header) + exiled card (body clause 1) | the exiled card | ❌ Bioplasm | ✅ exiled card | ❌ Bioplasm |
| **Perrie** — "…target creature you control gains trample and gets +X/+X…" | `SelfD` (attacker) + `TargetD` (the target) | the target | ❌ Perrie | ✅ the target | ✅ the target (of its own clause) |

The report's own analysis of this table, preserved as background — **superseded
by the settled ruling above, which chose sort-scoping over either flat rule**:
"No flat tiebreak on the determiner wins. Recommendation: clause-recency with
an intra-announcement `SelfD` tie-break — resolve to the most recent mint in
reading order; where two mentions come from the same announcement (an event
header's `nounDelta by ++ selfSubjIntro n`), the `SelfD` half wins. Only rule
of the three that gets all three carriers right, and the list is already
most-recent-first — `nounDelta (Both l r) = nounDelta r ++ nounDelta l`
(`Phrase.idr:1625`) reverses deliberately for exactly this." The report also
noted, as a cost estimate for that (rejected) recency-based fix: "`It`'s gate
stops being `countOnes Object bs = 1`; the proposition becomes 'a preferred
witness exists', so `countByWitness`'s premise form no longer applies,
`itReadsOnlyPrefix`/`itResolvesInPrefix` (`:388–396`) are rewritten, and a new
witness lemma is owed. Recency is a *find-first*, not a count, so
`gateSplitsAtNomIntro`/`gateSplitsAtCondIntro` (`:893`, `:907`) stop applying
to `It`." The settled sort-scoped design keeps the gate a counted-uniqueness
fold over a narrowed candidate set rather than a preference/find-first, so this
specific cost accounting does not transfer wholesale — restate it fresh
against the actual `countOnesAt`-style gate; see ProofsAnaphora debt below.

### A2 — the demonstrative stays separate: SETTLED, do not reopen

Keep the demonstrative separate from A1's mechanism — `EitherOf` arms gated
jointly, no silent `countWord` widening. This matches the report's own
recommendation: Heart of Bogardan fails `countWord PlayerW bs = 1`, not
`countOnes`, and both its player mentions are non-self — no `SelfD` tie-break
(or, now, no sort-scoping rule minted for A1) reaches it by accident.
`EitherOf`'s docstring (`Phrase.idr:1490-1496`) says both arms are read in the
SAME context, so the correct fix is gating `splitOverPlaneswalker`'s two
`That` arms jointly rather than each against the whole prefix.

### A3 — `moveIntro p This z` mints `SelfD`: SETTLED, do not reopen

`moveIntro p This z = bs` should mint `SelfD` — `condDelta`/`selfSubjIntro`
already do so for `AsType t This _`, and a discarded `This` is the same object
under the same determiner. One clause; unblocks Twinshot Sniper's `Channel`
witness; adds an ambiguity A1's mechanism must already handle, so **sequence
this after A1's mechanism lands.**

## ProofsAnaphora debt

Transcribed from the split report's summary table:

> **Yes, heavily** — §2 `countOnesIsFold` (+ `countWordIsFold` if A2 widens);
> §3 `itReadsOnlyPrefix`/`itResolvesInPrefix` rewritten to the new gate shape;
> a **new witness lemma** paralleling `countByWitness` if the gate stops being
> `= 1`; §5 must say what replaces `gateSplitsAtNomIntro`/`gateSplitsAtCondIntro`
> for `It`. **Plus an amendment to `docs/decisions/oracle-text-is-forward-anaphoric.md`
> clause 3.**

(A2 is settled as staying separate — not widening `countWord` — so in
practice the `countWordIsFold` branch of the above does not trigger; the row
is reproduced verbatim as the report recorded it.)

## Sequencing

A first; C after A (each new mint C wants to add must be gated against
creating unresolvable ambiguity under A's mechanism); B/D/E parallel-safe.

The report's own sequencing note, preserved: "A first (it is the architecture,
and C's new mints make its problem worse). B, D, E in parallel with A or with
each other. C after A, or with an explicit note in its brief that each new
mint is gated against creating a second singular object binding. If only one
sub-round runs, it is A: it is the only one whose decision the other four have
to live with."

Standard constraints apply.
