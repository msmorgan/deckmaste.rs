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

---

## As-landed

Standard constraints applied. `idris/scripts/build` **23/23 PASS on a clean
rebuild** (`rm -rf build`). `cargo xtask cite check --list-noncompliant` empty;
`cargo xtask cite check` 0 stale over 18,063 citations; `cite bless` added no
rule; `jj diff --git | cargo xtask cite audit --diff` read at 11 sites (one
citation was **changed** on that read — see below). **One pin retired, one
minted, net zero; none silently passing** — `badPartitiveOfCountedGroup`,
`badChooseSomeOf`, `badEachOfCountedGroup`, `badControlledByGroup` and
`ProofsC`'s partitive-exile pin all re-elaborated against the widened slots and
still refuse. `Experimental/Cards.idr` binds no implicits; no `{default`.
**ProofsAnaphora: untouched, and owes nothing** — `SomeOf`, `CountedGroup`,
`LibrarySlice` and `slicePossessorOk` have zero occurrences in that module
(no §2 counter, no §3 gate pair), and every §5 `Refl` still holds because all
three deltas kept the `intro = delta ++ bs` shape. That is exactly the debt the
split report predicted.

Every count below was re-measured this round from
`data/derived/cards.jsonl` filtered `jq 'select(.supported)'` (32,568 supported
cards), on distinct oracle LINES unless stated.

### E1 — "from among" is `SomeOf` under a second preposition: RATIFIED, and the population is much larger than the parent said

**Measurement first, because the ratification turned on it.** The parent's
"20+ lines across mill, reveal and exile" is a large undercount. "From among"
occurs on **474 supported lines, 465 distinct, over 469 cards**. Splitting it:

| family | distinct lines | what it is |
|---|---|---|
| partitive over an ANAPHORIC batch | **329** | this item — "put a creature card from among them into your hand" |
| play-permission source | 97 occurrences | "you may cast/play … from among …" — `MayPlay`'s `from` is a `ZoneExpr`, not a mention (Ledger) |
| partitive over a DESCRIBED group | 42 | "from among creatures you control" (Ledger; 19 of them are Tayam's counter family) |

Within the 329: mill antecedent 49, exile-antecedent 16 (of which 8 are casts),
reveal 204, look-at 205 (the last two overlap heavily — "look at the top N …
you may reveal X from among them"). The antecedent is spelled four ways —
"them" (302), "the milled cards" (14), "the cards milled this way" (9), "those
cards" (6), plus "the revealed cards" (4) and the exile pair (3).

**The count did not contradict the spelling reading; it sized the slot.** Of
the 333 anaphoric-partitive occurrences, **all but the 30 "any number of" and
the bare-`of` forms describe the slice** — "a creature card" (38), "a land
card" (22), "a permanent card" (20), "an artifact card" (14) and a long tail.
Beside them the corpus writes the *bare* partitive just as heavily under the
plain preposition — "one of them" 143, "any number of them" 125, "one of those
cards" 54. Two surfaces, both attested in bulk, one referent. So the
description is an **optional** slot on the existing constructor, not a second
constructor and not a mandatory argument.

**Landed:** `Phrase.Noun.SomeOf` gains a second positional slot —

    SomeOf : (q : Quantity bs) -> (descr : Maybe (Predicate bs Object)) ->
             (grp : Noun bs Object) -> …

with `Phrase.sliceTy` (the head type: the description's where it names a card
type, else the group's) and `Phrase.sliceDelta` (the description's own
bindings, threaded between the count and the group). Zone is unchanged — the
group locates the members. Macros `Macros.fromAmong` and `Macros.oneFromAmong`;
`oneOf`/`someOf` keep the bare form.

**Citation changed on the audit read.** The first draft cited [CR#109.2] for
"the group is what locates them". [CR#109.2] says the opposite of what that
needed — a zoneless, card-less type description means a battlefield permanent —
so it would have argued *against* the claim. Replaced with [CR#109.2a] and
reframed honestly: [CR#109.2a] locates a card-worded description by the zone
the phrase STATES, and a partitive states a group in that slot instead.

**Witnesses** (all typechecked):

- `collectedCompany` — **whole card**. "Look at the top six cards of your
  library. Put up to two creature cards with mana value 3 or less from among
  them onto the battlefield. Put the rest on the bottom of your library in any
  order." Exercises a non-unit quantity, a compound description and `TheRest`
  after the partitive.
- `communeWithTheGods` — **whole card**. The reveal arm, with a disjunctive
  description (`Or [creature, enchantment]`) and a `may`-wrapped body.
- `bindToLife` — the mill arm (Vastlands Scavenger's adventure), benched as the
  line; the whole is an adventure face pair.
- `millThenPutFromAmongMilled` — the same partitive over the PARTICIPLE
  spelling of the batch, `thoseVerbedThisWay "Mill" CardW`.
- `exileTopThenPutFromAmong` — Lord of the Void's body, the exile arm.

**Contract claims** (typechecked `Refl`s, the round's real receipt):
`describedSliceReadsAsCreature` — the described slice reads back as a creature
card; `bareSliceReadsUntyped` — the bare partitive over the same batch carries
NO card type, which is what the description buys, since a library slice
projects none; `describedSliceKeepsGroupZone` — the description does not
relocate the slice.

**Tayam does NOT ride it, and the reason is structural.** "Remove three
counters from among creatures you control" fails `SomeOf` twice over: the slice
is COUNTERS, and `SomeOf : … -> Noun bs Object`; and "creatures you control" is
a description, where `grp` demands a `GroupMention`. Its neighbour is
`Effect.Distribute` over `DividedVerb`, which has a distribution arm
(`DistributedCounters`) and no removal arm, and whose `among` slot demands the
same `GroupMention`. **19 distinct supported lines over 20 cards** (Tayam,
Eventide's Shadow, Retribution of the Ancients, Elspeth Resplendent, and the
loyalty- and stun-counter variants). Ledgered.

### E2 — the at-random mode as a `Maybe ChoiceMode` on `CountedGroup`: LANDED

**Measured: 24 distinct supported lines** ("two cards at random" ×5 carriers,
"X cards at random" ×6, "three cards at random", "any number of cards at
random", "two creature cards at random", …), 25 occurrences.

**Landed** exactly as pinned: `CountedGroup : (q : Quantity bs) -> (mode :
Maybe (ChoiceMode bs)) -> (p : Predicate bs k) -> …`, mode between the
determiner and the head, `Indefinite`'s slot order with the quantity in the
article's place. `Nothing` written at all 40 existing `Cards.idr` sites and the
five in `Macros`/`Proofs*` — the house idiom for a `Maybe` slot
(`RemoveCounters _ Nothing _`). New macro `Macros.countedAtRandom`.

**`TokenPhrase.OneToken` checked, as the pin asked.** It pattern-matches
`Indefinite m p` mode-**polymorphically**, so `CountedTokens` was made
polymorphic in the new slot the same way rather than pinned at `Nothing`; the
two token phrases stay symmetric.

**Contract claims:** `atRandomModeIsAnnouncementNeutral` — the counted mention's
delta is *identical* with the mode written and without, which is why the slot
rides the count rather than replacing the determiner;
`countedAtRandomIsPlural` — it stays `ManyOf`, which is what a mode-carrying
`Indefinite` could not be (`nounPlur (Indefinite _ _) = OneOf` is definitional).

**Tourach benches WHOLE — both of them.**
`hymnToTourach` ("Target player discards two cards at random", {B}{B} Sorcery)
and `tourachDreadCantor` ("Kicker {B}{B} / Protection from white / Whenever an
opponent discards a card, put a +1/+1 counter on Tourach. / When Tourach
enters, if it was kicked, target opponent discards two cards at random.") —
kicker, the quality keyword, the existing `tourachDiscardTrigger` and the
intervening-if all already had rows; the plural at-random determiner was the
only blocker, exactly as routed. Both carry the bench's standing discard
caveat (the owned-hand read is spelled `InZone handZ`; an owned-hand expansion
needs a subject-read noun the vocabulary does not have — pre-existing, noted at
`discardsACard`).

### E3 — the plural-GROUP library slice: pin retired against its five lines, re-pinned one step over

**Measured: exactly 5 supported carriers, 4 distinct lines** — Field of Dreams,
Lantern of Insight and **Wizened Snitches** (the fifth carrier the ticket's
four-name list omits; all three print "Players play with the top card of their
libraries revealed"), Breeches Brazen Plunderer ("the top card of each of those
opponents' libraries"), Shared Fate ("the top card of one of their opponents'
libraries"). This matches the closure table's "5 lines" exactly.

**Landed:** `slicePossessorOk` admits the DISTRIBUTIVE plurals and nothing
else —

    slicePossessorOk (Each _)        = True
    slicePossessorOk (EachOf _)      = True   -- new: "each of those opponents"
    slicePossessorOk (PlayerGroup _) = True   -- new: "their libraries"
    slicePossessorOk n               = isOne (nounPlur n)

`LibrarySlice` gets the docstring the parent asked for: **two surfaces, one
cell**, and which word pluralises says which — a plural COUNT pluralises the
card word over one library, a distributive plural POSSESSOR pluralises the ZONE
word over one card apiece, because [CR#400.1] gives each player their own
library. `outputPlur` already computed both; only the gate was refusing.

**Pin retirement and re-pin.** `Proofs.badSliceOfGroupPossessor` is **retired**
— its term (`lookAt (LibrarySlice OnTop (Lit 1) (PlayerGroup YourOpponents))`)
now spells. Its [CR#400.1] ground survives one step over and is re-pinned as
**`Proofs.badSliceOfCountedPossessor`**: "the top card of two target players'
library". The argument is `soleHolderOk`'s own recorded one, at the possessed
zone instead of the possessed object — *a counted plural does not distribute;
it asks for the one library a named two have between them, and [CR#400.1] gives
each player their own.* That is a rules-impossibility, not a measured zero, so
§1.4 is satisfied.

**Witnesses:** `playersTopCardSlice` + `playersTopCardIsPlural` (the mention
Field of Dreams's line names, and the proof that it binds `ManyOf` — the whole
of what the singular-possessor spelling could not say);
`eachOfThoseOpponentsTopCard` (Breeches' shape: damage to `Each Opponent`, then
the slice over `EachOf (Those PlayerW)`).

### RETURN fence — untouched

No `verbFacts` row was added, changed or proposed. The fence stands as written,
including its narrower open question (a stamp without a tag, decided only
against a printed witness); no witness for it appeared in this round's
measurements.

### Ledger — future work, for routing

1. **The counter partitive over a described group** — 19 distinct lines,
   Tayam's family (Tayam, Eventide's Shadow, Retribution of the Ancients,
   Hopeful Initiate, Slippery Bogbonder, Elspeth Resplendent, The Filigree
   Sylex, …). Wants a removal arm on `DividedVerb` (or a counter-side
   partitive), and a decision about `among` over a description. **Route to a
   counter-family or distribution ticket, not to this family.**
2. **The object partitive over a DESCRIBED group** — the other 23 of the 42
   non-anaphoric "from among" lines ("cards exiled with this creature", "chosen
   at random from among your opponents"). Blocked by `groupMention`, which
   admits mentions only. A `groupMention`-widening decision, deliberately not
   taken here (E1's pin rests on the current gate).
3. **The play-permission "from among" source** — 97 occurrences. `MayPlay`'s
   `from` is a `ZoneExpr`; these name a MENTION. Its own cell.
4. **"all X from among them"** — 8 distinct lines over 9 cards (Animist's
   Awakening, Beluna Grandsquall, Depala, Nissa Nature's Artisan, Tamiyo
   Collector of Tales, Tezzeret Master of the Bridge, …). "All" is not a
   `Quantity`: `Range` cannot say "every member matching the description". A
   `Quantity`/`AllOf`-partitive question.
5. **"a permanent card"** — no predicate spells the permanent-card CLASS in a
   non-battlefield zone (`Permanent` is battlefield-scoped), which is what
   blocks Wasteful Harvest, Cache Grab, Rapid Rescue and Seed of Hope from
   benching whole. 20 "a permanent card from among …" occurrences alone.
6. **Field of Dreams / Lantern of Insight / Wizened Snitches** — the slice now
   spells; the consuming static does not. "Play with the top card of their
   libraries revealed" has no row at all ([CR#401.5,401.6] is the pair to read).
7. **Shared Fate** — needs "their opponents", an opponent-of-a-named-player
   read; `Predicate.Opponent` is of-`You` only and its docstring already says
   the team form is deferred.
8. **Breeches, Brazen Plunderer whole** — needs the "one or more Pirates you
   control deal damage to your opponents" header plus item 3's play permission.
9. **The owned-hand discard read** — pre-existing and unchanged: every discard
   line on the bench writes `InZone handZ` where the printed line means the
   subject's own hand. Noted at `discardsACard` since before this round.
