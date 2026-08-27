---
needs: []
---
# Partitive, possessor and determiner surfaces

Sub-round E of the five-way split of
`docs/tickets/planned/workbench-anaphora-mentions-and-creation.md`, split
2026-08-27 per the completed split analysis. **Size: S. Fully independent of
A–D.**

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

`Phrase.idr` (`SomeOf`:1502, `LibrarySlice`:1498, `Indefinite`:1458,
`CountedGroup`:1467, `SlicePossessor`:2538), `Words.idr`, `Proofs.idr`
(`badSliceOfGroupPossessor`:72), `Macros.idr`:1072, `Cards.idr`.

## Owned items

- *"The mill's AMONG-restriction — 20+ lines across mill, reveal and exile …
  Its neighbour is `SomeOf`, the partitive this grammar has. THE DESIGN
  QUESTION: whether 'from among' is that determiner under a different spelling,
  or a second construction."* Plus routed: *"Tayam's 'from among' distributive
  removal over a described group belongs with the mill's AMONG-restriction."*
- *"The plural-GROUP library slice — 5 supported lines … held today by
  `badSliceOfGroupPossessor`"* (Field of Dreams, Lantern of Insight, Breeches,
  Shared Fate) — *"`LibrarySlice` spells one library and pluralises only the
  card word, so the phrase is a second surface, not a cell."*
- Routed (event-zone-1): *"the plural at-random determiner — 'two cards at
  random' (Tourach's whole-card blocker): `Indefinite` carries `ChoiceMode` but
  is singular; `CountedGroup` counts but has no mode slot."*
- *"RETURN is not a `VerbName` row — do not re-propose it"* — carried as a fence
  (restated in `verbFacts` terms per §0), including *"What IS open here is
  smaller: … If a witness ever wants 'the returned card', it wants a STAMP
  WITHOUT A TAG — decide that only against such a witness."*

## Settled design — do not reopen (user-ratified 2026-08-27)

The report that split this ticket left three items as OPEN PINs. All three
are now SETTLED by the user's ruling below.

### E1 — "from among": `SomeOf` spelling or a second construction: SETTLED, do not reopen

A spelling of `SomeOf`, whose `grp` slot already takes a `GroupMention`, and
`ThoseVerbed Mill CardW` is one. A second constructor would duplicate
`NonZeroQ`/`WellFormedQ`/`GroupMention` plumbing for a different preposition.
Count the mill/reveal/exile carriers before ratifying the spelling in-round —
the design choice is settled; the population sizing that governs its exact
shape still happens in-round.

### E2 — where the at-random mode goes: SETTLED, do not reopen

A `Maybe ChoiceMode` slot on `CountedGroup`, not a plural `Indefinite`.
`nounPlur (Indefinite _ _) = OneOf` is definitional and its delta mints at `AD
OneOf` (`Phrase.idr:1616`); `CountedGroup` already carries `quantPlur q` and
mints at `CountD`. The mode is orthogonal to the count. Check
`TokenPhrase.OneToken`, which pattern-matches `Indefinite m p` (`:2840`).

### E3 — retire `badSliceOfGroupPossessor` how far: SETTLED, do not reopen

Retire it only against the five lines it was holding, and re-pin whatever
genuinely stays refused — its docstring cites [CR#400.1] ("a slice names ONE
library"), a real rules ground that survives the plural surface. Per the
parent: "retired only against the lines it was holding, and the zone-word
plural is spelled."

## ProofsAnaphora debt

Transcribed from the split report's summary table:

> **None**, unless E1 mints a new mention constructor — in which case one §2
> identity + one §3 pair, additive. E2 and E3 owe nothing (no gate changes).

## Sequencing

B/D/E parallel-safe with A and with each other, and with the rest of the
split. This sub-round is fully independent of A–D.

Standard constraints apply.
