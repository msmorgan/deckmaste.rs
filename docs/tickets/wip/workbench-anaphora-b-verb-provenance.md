---
needs: []
---
# Verb provenance on the bare anaphor, and the stamp payload

Sub-round B of the five-way split of
`docs/tickets/planned/workbench-anaphora-mentions-and-creation.md`, split
2026-08-27 per the completed split analysis. **Size: M. Independent of A** (it
narrows what `It` may reach; it does not change how many candidates there are
— but see the sequencing note below).

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

`Words.idr` (`Stamp`:929, `Payload`:948, `stampedBy`:1794, `afterShuffle`:1826,
`provOfIt`:1584), `Phrase.idr` (`It`:1521, `That`:1525), `Events.idr`,
`ProofsAnaphora.idr` §2/§3, `Macros.idr`, `Cards.idr`.

## Owned items

- *"The grammar cannot see WHICH verb stamped the mention a bare `It` reads.
  That is one named overgeneration and three blocked cells, all the same missing
  query. `TheVerbed` already carries `stampedBy`; `It` carries nothing."*
  — the regeneration rider overgeneration (finding 1010, 4 lines); the
  participle read (10 lines, "Creatures destroyed this way can't be
  regenerated"); the demonstrative subject (4 lines, Nekrataal, Scorching Lava);
  and the marked-read/chooser-repeatability payer (finding 1030, 12 carriers).
- Routed (verb-label-residues): *"`afterShuffle` is owner-blind — it drops every
  unstamped library mention whichever library was shuffled, but [CR#701.24b]
  scopes to the shuffled one; the binding payload records a zone, not an owner."*

## Binding rulings

§1.4 governs finding 1010 — the 4-line overgeneration is refused **only if a
rule makes the spanned regeneration rider meaningless**; [CR#701.19c] is *why
the mechanism is not a deed denial* and, per the parent's own fence, *"is NOT
a reason the word cannot sit in a word catalog (finding 1008); do not
re-derive the opposite."* If no rule refuses it, the 4 lines are recorded at
their count, not pinned. PLAGUE SPORES is explicitly **not** this round's
(two targets under one plural anaphor → `workbench-conditional-and-coordination`'s
live successor).

## Settled design — do not reopen (user-ratified 2026-08-27)

The report that split this ticket left two items as OPEN PINs. Both are now
SETTLED by the user's ruling below.

### B1 — how provenance rides `It`: SETTLED, do not reopen

A new gated constructor on `TheVerbed`'s model (reject a `Maybe VerbLabel`
field on `It`). `TheVerbed (v : VerbLabel) (w : NounWord) (marking :
VerbedMarking)` already has the shape and a ProofsAnaphora entry (`:454–471`)
to copy, and it keeps `It`'s own entry (sub-round A's battleground)
untouched. One constructor with two gates, not one constructor carrying an
optional field.

### B2 — where the shuffled library's owner lives: SETTLED, do not reopen

Pass the shuffled library's possessor to `afterShuffle`; do not grow the
payload. `effIntro (Shuffle whose) = afterShuffle (nomIntro whose)` — give
`afterShuffle` the `whose` mention rather than adding an owner field to
`ObjectP`, which is a `Words.idr`-wide change touching
`payloadZone`/`payloadTy`/`JoinP` and every construction site. Cite
[CR#701.24b].

## ProofsAnaphora debt

Transcribed from the split report's summary table:

> One new §2 identity (`countVerbedIt`-style) plus one new §3
> `…ReadsOnlyPrefix`/`…ResolvesInPrefix` pair, on `TheVerbed`'s model
> (`:454–471`). §5 untouched. Additive only.

## Sequencing

B/D/E parallel-safe with A and with each other. Sub-round note: if A lands
its preference/candidate-narrowing mechanism first, B's new constructor must
state its own preference behaviour or explicitly inherit A's — do not leave
the interaction implicit.

Standard constraints apply.

---

## As landed (2026-08-27)

### B1 — the mechanism: verb-scoped counted uniqueness, plus the context that makes it readable

Two pieces, and the second is what made the first useful.

**The read.** `Words.idr`, beside `stampedBy`/`verbedWordOk`:

```idris
stampIs         : VerbLabel -> Maybe Stamp -> Bool     -- no stamp answers False for every label
itVerbedReaches : VerbLabel -> Binding -> Bool         -- itReaches OneOf b && stampIs v (payloadProv b.payload)
countVerbedIt   : VerbLabel -> Bindings -> Nat
provOfVerbedIt, zoneOfVerbedIt, tyOfVerbedIt : VerbLabel -> Bindings -> ...
```

`Phrase.idr` (`Noun`, beside `ItAt`; `It`'s and `ItAt`'s own gates UNTOUCHED):

```idris
ItVerbed : (v : VerbLabel) -> {auto 0 kn : KnownVerb v} ->
           {auto 0 ok : countVerbedIt v bs = 1} -> Noun bs Object
```

plus `setZoneVerbedIt` and one row per new constructor in `nounEqRef`,
`nounDelta`, `anchorPhrase`, `choosable`, `groupMention`, `costNounOk`,
`nounIsYou`, `nounTargeted`, `counterMemoryOk`, `moveDestOk`, `moveIntro`,
`nounZone`, `nounTy`, `nounPlur`. `Macros.itVerbed` is its carrier.

Two gates, as ratified: one context read (`countVerbedIt v bs = 1`) and one
obligation that mentions no context at all (`KnownVerb v`) — `TheVerbed`'s
split exactly, minus the spelling obligation, because the pronoun spells no
participle.

**Interaction with sub-round A (the sequencing note).** `ItVerbed` states its
own behaviour and inherits nothing. It narrows on the OPPOSITE side of the
clause from `ItAt`: `ItAt` narrows by the CONSUMING verb's rule [CR#109.2],
this by the PRODUCING one's stamp. Neither is a preference — both are counted
uniqueness over a smaller candidate set, so two mentions the same label
stamped refuse exactly as two battlefield mentions refuse the carrier-scoped
read. **They do not compose:** there is no `ItAtVerbed`, and the docstring
says so. The ADR needs no amendment — clause 3 as A amended it ("counted
uniqueness … over the narrower set the consuming verb's own rule admits")
already covers a second narrowing of the same shape, and this one is
likewise a `countBy` fold.

### The rider context — `riderIntro`, the piece the round turned on

Every one of this round's blocked cells failed for one reason: `CantBe`
typed its subject at `preIntro e`, which is the host clause's announcement
*before* the clause acted, so no stamp stands there. `Effect.idr` now has a
sibling of `preIntro`:

```idris
riderIntro : {bs : Bindings} -> Effect bs -> Bindings
riderIntro (Enact v (Move what to _))  = stampIntro (Just v) what
riderIntro (Does s v (Move what to _)) = stampIntro (Just v) what
riderIntro e = preIntro e
```

and `CantBe`'s subject is `Noun (riderIntro e) k`.

**Why this shape and not a re-zoning.** [CR#608.2c] reads a card's later text
against its earlier text as one statement — and takes *this very sentence*
("Destroy target creature. It can't be regenerated") as its worked example —
so the rider's subject names the referent as its own clause left it, carrying
that clause's label, and not as a following sentence would find it.
[CR#701.19a] regenerates a PERMANENT, so a subject reachable only in a
graveyard is the wrong subject: `stampIntro` writes the label with the zone
left where it was.

**No ADR amendment, and §5 untouched, for the same reason:** this is an
IN-PLACE RE-MARK, the shape binder-contract clause 2 already licenses
alongside `settleTargets` and `defineLetter`. Nothing is minted, nothing is
dropped, the list keeps its length and order — so the bare pronoun counts
exactly what it counted (`countOnes Object` unchanged at every existing
carrier), and every gate that was a fold over the prefix still is one.

### Per-family outcomes

| family | outcome |
|---|---|
| **the attached rider, bare pronoun (88 occ / 87 cards "it")** | Re-pointed onto the provenance read. **Terror** and **Snuff Out** now write `Macros.itVerbed "Destroy"`, so the fact the rider depends on is stated at the site instead of left to the reader. |
| **the participle read (10 destroy-hosted occ)** | **WRITES**, and `damnDestroyLine` benches it — Damn's first line, `TheVerbed "Destroy" (TypeW Creature) ThisWay`. New macro `theVerbedThisWay`, `thoseVerbedThisWay`'s twin. **6 of the 10 are singular, 4 plural** (Corrosion's is "Artifacts destroyed this way"). Damn's is the one whose host is a bare labeled move and it is the one benched. Most of the rest host their destroy under a wrapper — `Modal` (Catastrophe), `If` (Breaking Point, Soul Rend, Tsabo's Assassin), `Unless` (Essence Vortex), `ForEachOf` + `Unless` (Giant Albatross), a `Sequentially` (Corrosion), a threshold clause (Kirtar's Wrath) — and every one of those `preIntro` rows exports no stamp (`If`/`ForEachOf` give `bs`, `Modal` gives `quantDelta q ++ bs`, `Sequentially` folds `preIntro`), so `riderIntro` falls through and the participle finds nothing. **The read is unblocked; its other carriers are blocked on wrapper threading, which is not this cell.** Ledgered. |
| **the demonstrative subject (5–6 occ)** | **The gate was already open** and the round found it: `That (TypeW Creature)` typechecks as `CantBe`'s subject at an empty prefix, both before and after this round. Finding 1009 recorded it blocked *"probed bare, without this row"* — i.e. as a following sentence, where the destroyed target is in a graveyard and `wordReaches (TypeW t)` demands the battlefield. Inside the rider it never was. **All of its printed carriers are blocked elsewhere**: Nekrataal and Stormscape Battlemage on the enters trigger's own announcement (below), Parallax Dementia on `moveIntro p (AttachHost _ _) z = bs` (destroying the enchanted creature mints nothing), Scorching Lava on a damage host plus a kicker conditional, Lim-Dûl's Cohort and Mageta on a coordinated header. Ledgered. |
| **the marked-read payer rider (finding 1030)** | Recorded on `ChoiceStands`, not gated — the same posture as 1010, which is what "free here" meant. Re-measured: **12 occurrences over 12 cards** (the recorded 14 occurrences does not reproduce; the 12 cards does), every one behind a chooser that can fire more than once. Nothing in the grammar represents a chooser's repeatability, so the row still admits the sentence no line writes. |

**Nekrataal is the round's clearest witness even though its own spelling is
blocked.** In its trigger's rider context, `countWord (TypeW Creature) = 2`
and `countOnes Object = 2` — the entering creature and the destroyed target
both stand — while `countVerbedIt "Destroy" = 1`. All three are recorded as
`Refl`s (`nekrataalTwoCreatureWords`, `nekrataalTwoObjects`,
`nekrataalOneDestroyed`), and `CantBe … Regenerated (Macros.itVerbed
"Destroy")` typechecks there where neither the bare pronoun nor the
demonstrative does. **The verb-scoped read resolves exactly the ambiguity
A's policy (b) leaves as same-carrier residue** — both candidates are
battlefield creatures — because it asks a question the carrier cannot.
What blocks Nekrataal's own line is that `eventAfter (Enters n _)` announces
the entering creature under `TheD` (via `moveIntro p (AsType t n _) z`) where
`selfSubjIntro` and `condDelta` mint `SelfD` for the same phrase, and
`wordNow` skips `SelfD` alone. **That is an announcement defect, sub-round
C's.** Ledgered.

### Finding 1010 — disposition: RE-MEASURED and RECORDED, not pinned

Re-measured over `data/derived/cards.jsonl` `select(.supported)` (32,568
rows), reminder text stripped: **the damage-hosted regeneration denial is 9
occurrences over 9 cards, not 4** — the recorded Engulfing Flames, Rage of
Purphoros, Carbonize and Disintegrate plus Flamebreak, Incinerate, Jaya
Ballard Task Mage, Runesword and Scorching Lava. The 2×2 reproduces exactly
and is now explained:

| host | attached, no span | spanned, "this turn" |
|---|---|---|
| destroy / sacrifice / exile | **138** (134 cards) | **0** |
| damage | **0** | **9** (9 cards) |

(the remaining 9 spanned occurrences hang off no removal clause at all).
Perfect covariance, 138/138 and 9/9.

**Not pinned, and the rule says why the corpus writes it the way it does
rather than why it could not be written.** [CR#704.5g] destroys a lethally
damaged creature as a state-based action and states outright that
"regeneration can replace this event"; [CR#701.19c] makes the denial a
shield-*application* denial, not a deed denial. So a regeneration rider after
a damage clause is rules-meaningful, and §1.4 forbids pinning it. What the
corpus's span is doing is timing: the destruction a damage clause causes is
an SBA taken after the clause finishes, so the denial needs a duration to
still be there — which is why 9/9 write "this turn" and 138/138 do not.
The parent's fence on [CR#701.19c] is restated, not re-derived against.

**What the round buys instead of a pin is that the distinction is WRITABLE.**
`engulfingFlamesRider` records both halves as `Refl`s: `countVerbedIt
"Destroy" = 0` there (a damage clause leaves no destroy stamp, so the
provenance read refuses), while `countOnes Object = 1` (the bare pronoun
still resolves — the overgeneration, at its count, unpinned). A capability,
not a prohibition.

### Bioplasm — NOT benched whole; the block is two-part and only one part was known

The ticket's conditional ("if your constructor's gates naturally cover the
exile-stamped read") is **not met**. Recorded with `Refl`s:

- `countVerbed "Exile" CardW bioplasmAfterExile = 1` — "the exiled card"
  writes (`bioplasmExiledCard`).
- `countVerbedIt "Exile" bioplasmAfterExile = 1` — the verb-scoped pronoun
  writes (`bioplasmExiledPronoun`), where the bare `It` is refused at 2
  (A's `bioplasmTwoCandidates`).
- `countVerbed "Exile" (TypeW Creature) bioplasmAfterExile = 0` — the card's
  own spelling, "the exiled creature card", does not.

Sub-round A's close named one cause (`verbedWordOk (TypeW t)` demands
`wasField`, which a library card lacks). **There is a second and it is the
harder one:** `tyOfVerbedIt "Exile" bioplasmAfterExile = Nothing` — the
mention records NO card type, because "the top card of your library" names
none and the `If it's a creature card` test that follows does not re-mark the
binding it tested. That is an announcement question, not a provenance one.
Both recorded; ledgered to C.

### B2 — `afterShuffle`'s owner-blindness: MEASURED AT ZERO and recorded, NOT built

**This is the round's one deviation from a settled pin, and it is a
measurement result, not a re-opened design.** B2 ruled that the shuffled
library's possessor be passed to `afterShuffle` rather than an owner field
added to `ObjectP`. Reading [CR#701.24b]'s own text — "search a library …
shuffle THAT library … all the cards in that library except those are
shuffled" — the scoping needs BOTH the shuffled library's owner and the
MENTION's owner, and the payload records only a zone. A possessor passed in
has nothing to match against, so the parameter would be read by nothing.

Measured before deciding, over all 32,568 supported rows:

- search-and-shuffle lines whose shuffled possessor differs from the searched
  one: **0** (three apparent mismatches — Demolition Field, Green Sun's
  Zenith, Sadistic Sacrament — are two independent sentences, an unrelated
  "shuffle this card into its owner's library", and a back-reference).
- paragraphs pairing a library-slice mention with a shuffle: **3**, of which
  **0** name different possessors.

So the discriminating case is zero, and §1.4 — binding on every sub-round —
records an overgeneration measured at zero on the cell with its count rather
than gating it. Landed as exactly that: the record sits on `survivesShuffle`
with [CR#701.24b] quoted, the owner field named as what the match would cost,
and the zero stated. **No dead parameter was added.** If a printed line ever
shuffles a library other than the one its paragraph named, the pin's
mechanism is what to build; the ledger carries it.

### ProofsAnaphora

§2 — one new fold identity, `countVerbedItIsFold` at `itVerbedReaches v`. An
ordinary `countBy` fold, which is the point: the verb-scoped read is a count.

§3 — one new pair, `itVerbedReadsOnlyPrefix` / `itVerbedResolvesInPrefix`,
resolving through the existing `countByWitness`. **No new witness lemma was
owed**, for A's reason: the gate is still `= 1`.

§1, §4 unchanged. **§5 HELD and re-typechecked as written** — `riderIntro` is
an in-place re-mark of `preIntro`'s own output, not a delta change, so
`nomIntroIsDeltaThenPrefix`, `condIntroIsDeltaThenPrefix`,
`gateSplitsAtNomIntro`, `gateSplitsAtCondIntro` and every telescope equation
are untouched and unaffected. `It`'s, `ItAt`'s and `TheVerbed`'s own entries
are untouched. Additive only, as the debt was priced.

### Bench

| card / line | outcome |
|---|---|
| **Terror**, whole card | Keeps writing, now on `Macros.itVerbed "Destroy"`. |
| **Snuff Out**, whole card | Same. |
| **Wrath of God** / **Damnation** | Keep writing, unchanged, on the bare plural `Them` — the plural twin is not built (ledger 1). |
| **Damn's first line** (`damnDestroyLine`) | WRITES — the participle read. Overload is the elision; it has no word in this vocabulary. |
| **Nekrataal** | Trigger-context counts benched (2 / 2 / 1). Whole card blocked on the enters trigger's `TheD` self-announcement → C. |
| **Engulfing Flames' rider context** | Both halves of finding 1010 benched as `Refl`s. |
| **Bioplasm** | Two reads benched (`CardW`, `ItVerbed "Exile"`); the typed read and its two causes recorded. Whole card still blocked. |
| **Phyrexian Rebirth**, **Incinerate**, **Hurr Jackal** | Unchanged across the `CantBe` retype. |

Gates: `idris/scripts/build` 23/23 cold, 0 errors, 0 warnings.
`cargo xtask cite check --list-noncompliant` empty; `cite check` 18,030
citations, 0 stale (**0 rules blessed** — every rule cited was already
registered). `cite audit --diff` 11 sites read against their rule text; one
[CR#701.1] site was dropped as right-number-wrong-topic (it grounds the
keyword-action vocabulary, not a claim about stamps) and the sentence
rewritten without a cite.

### Ledger — needs routing

1. **The plural verb-scoped pronoun (`ThemVerbed`).** "They can't be
   regenerated" is **39 occurrences / 38 cards** against "it"'s 88/87. One
   constructor was built, per the priced ProofsAnaphora debt, so Wrath of God
   and Damnation still write the unscoped `Them`. A second constructor is one
   more §2 identity and one more §3 pair — the same shape, no new argument.
2. **`riderIntro` falls through every wrapper.** Nine of the ten participle
   lines and most of the demonstrative ones host their destroy under a
   `Modal`, `If`, `Unless`, threshold or delayed trigger, all of which export
   nothing, so the rider's subject sees no stamp. A modal host also raises a
   real question the round did not answer — which mode's stamp the rider
   reads. → **sub-round C** (announcement audit) or its own cell.
3. **The enters trigger announces its own subject under `TheD`.**
   `eventAfter (Enters n _) = moveIntro Nothing n (Just Battlefield)` and
   `moveIntro p (AsType t n _) z` mints `TheD`, while `selfSubjIntro` and
   `condDelta` mint `SelfD` for the same phrase and `wordNow` skips `SelfD`
   alone. Costs Nekrataal and Stormscape Battlemage outright and inflates
   every same-carrier count taken in an ETB context. → **sub-round C**.
4. **`moveIntro p (AttachHost _ _) z = bs`.** Destroying the enchanted
   creature mints nothing, so no later clause can name it. Costs Parallax
   Dementia. → **sub-round C**.
5. **Bioplasm's untyped exile mention.** "The top card of your library"
   records no card type and the `If it's a creature card` test does not
   re-mark the binding it tested, so the typed participle read finds
   nothing even with the stamp in hand. → **sub-round C**; `verbedWordOk
   (TypeW t)`'s `wasField` demand is the second half and is a
   `verbedWordOk` cell decision.
6. **`afterShuffle`'s owner blindness, measured at zero.** Recorded on
   `survivesShuffle` (above). Reopens on a printed line that shuffles a
   library other than the one its own paragraph named.
7. **Finding 1030's chooser repeatability.** Recorded on `ChoiceStands` at
   12/12. Closing it wants a fact about the chooser, which is the choice
   ticket's subsystem, not an anaphor gate.
8. **`stampMoves` is now over-broad.** Its docstring says "a stamp is
   written only where a labeled action MOVED its patient", but `stampIntro`
   writes one for a status change too ("each creature tapped this way"), so
   `counterMemoryOk` refuses counter memory after a tap that moved nothing.
   Pre-existing, not this round's; noticed while wiring `ItVerbed`'s row.
