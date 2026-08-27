---
needs: []
---
# Extend the verb-label mechanism to `Keyword`

> **NEEDS THE USER'S CALL BEFORE CLAIMING.** Keyword residues are excluded from
> DRIFTS by [workbench-keyword-parameters-and-attachment](workbench-keyword-parameters-and-attachment.md)'s
> standing scope fence. **Do not claim this ticket while that exclusion stands**
> unless the user lifts it *for the conversion alone*. This is a conversion of the
> carrier, not a licence to work the fenced-off keyword residues.
>
> **LIFTED (user, 2026-08-26), for the conversion alone.** Direction: OPEN,
> per the subtype-labels and keyword-action rulings (meaning in data, label
> as data, vocabulary open). The round still writes the [CR#702]
> counter-argument down as this ticket demands; the fenced keyword residues
> stay fenced.

## Context

`docs/tickets/done/workbench-verb-labels-open.md` landed the label mechanism:
`VerbName`'s closed eight-arm enum became `VerbLabel = String`, gated at every
write site by `KnownVerb v = So (knownVerb v)` reading a plain `List VerbFacts`
data table, with `Composite v e` → `Enact v e`. A new keyword action is now one
data row plus one macro — proved by joining Tap with `MkVerbFacts "Tap" (Just
"tapped")` and `Macros.tap`, no constructor and no total-table clause anywhere.

`docs/tickets/done/workbench-effect-basis-realign.md` decided that `VerbName`'s
sibling `Keyword` stays a closed 33-member enum for that round, and named the
conversion question as the live one:

> `Keyword` is therefore the genuine declaration candidate of the two — the
> crate's 63 `.ron` macros are data because a keyword ability *can* be data —
> and whether to convert it is a live question for the sibling ticket rather
> than one settled by this round.

That round also recorded why the closure argument that made staying closed free
for `VerbName` does **not** transfer: no GADT is indexed on `Keyword`, its
tables (`keywordParamShape`, `keywordParamless`, `keywordCounterOk`,
`keywordStackRegime`, `keywordCardOk`) are flat data rows, and a member carries
no per-keyword typed obligation.

The ruling the conversion implements is
`docs/memory/rulings/workbench-mirrors-semantics-v2-structure.md` — the meaning
lives in the expansion, the label is data, the vocabulary is open.

## The change

Convert `Keyword` from a closed enum to an open label over its data rows, on the
`VerbLabel`/`verbFacts`/`KnownVerb` pattern:

- A `KeywordLabel = String` with a `KnownKeyword` gate reading one data table,
  so a typo fails at the gate rather than silently naming a new keyword.
- The five flat tables above collapse into fields of one per-keyword row, the
  way `verbedMarkingOk`'s per-verb rows became a `participle` field.
- The counter-argument to weigh before writing: [CR#702] is the CR's own
  enumeration of keyword abilities with no `Put`-shaped exception, so the
  closure argument is *cleaner* here than at `VerbName`. The round must say
  which wins and why — openness that buys the declaration author a data row, or
  a rule-backed closed set.

Whatever the verdict, record it; a "we looked and kept it closed" answer with
the tables collapsed into rows is a real outcome.

## Consumption boundary

`idris/src/Experimental/Words.idr` (the `Keyword` catalog and its five tables),
`idris/src/Experimental.idr` (the grant and extension constructors that name a
keyword), the pin modules `idris/src/Experimental/Proofs*.idr`, and the evidence
bench `idris/src/Experimental/Cards.idr`. If and only if the conversion lands,
`crates/deckmaste_english_v2` is where the declaration author's 63 `.ron` macros
would stop needing a core enum row — state the delta, do not build it here.

## Acceptance

- The verdict is written down with its argument, whichever way it goes.
- If converted: adding a keyword is one data row plus one macro, demonstrated by
  a keyword joined that way with a bench witness; no total-table clause per
  keyword survives; every pin that named a `Keyword` constructor is restated or
  its retirement argued.
- The DRIFTS exclusion is unchanged — no fenced keyword residue is worked here.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

## As landed

**Verdict: OPEN.** `Keyword` is gone; the vocabulary is a `String` label over
one data table.

### The [CR#702] counter-argument, and why it loses

`workbench-effect-basis-realign` put the closed case this way:

> `Keyword` stays closed too, and its closing rule is cleaner than
> `VerbName`'s: [CR#702] is the CR's own enumeration of keyword abilities,
> with no `Put`-shaped exception, so the steer's mechanism applies to
> `Keyword` *better* than to the site it was proposed for.

Read against the rule text, both halves fail.

**[CR#702] enumerates nothing.** [CR#702.1] states a NOTATION — an object
"lists only the name of the ability as a 'keyword'" — and each numbered
subsection then states one keyword's meaning. Compare the rules that really
do close a set: [CR#205.4a] names the five supertypes, [CR#122.1b] names
every keyword a counter may carry. Neither shape appears in [CR#702]. A
catalog of the keywords that exist is not a rule that only they may.

**The `Put`-shaped exception exists, three times over.** A level symbol "is
a keyword ability" [CR#711.2]; a chapter symbol "is a keyword ability"
[CR#714.2]; a class level bar "is a keyword ability" [CR#716.2] — each
stated outside [CR#702] altogether, and the card-anatomy rules say the same
([CR#107.8,107.15,107.16]). So the CR itself mints keyword abilities that
[CR#702] never lists. The workbench already writes one of them:
`ChapterMark`, which reaches the grammar as a `Triggered` trigger word and
never was a `Keyword` member. Under the closed reading the grammar was
already inconsistent with its own catalog.

**What openness buys, positively.** The standing rulings settle the
direction — meaning in data, label as data, vocabulary open
(`docs/memory/rulings/workbench-mirrors-semantics-v2-structure.md`,
`docs/memory/rulings/subtypes-are-labels.md`) — and the Rust side had
already taken it: `deckmaste_core::KeywordAbility` carries five intrinsic
variants plus `Composite { name : Ident, abilities : Vec<Ability> }`, and
`plugins/builtin/macros/keyword/` holds 68 macro rows. A keyword ability
*can* be data and on that side already is; the workbench was the last place
a printed word had to be a constructor.

**What closure would have bought: nothing typed.** The realign round's own
finding stands — no GADT is indexed on `Keyword`, its five tables were flat
data rows, and a member carried no per-keyword typed obligation. Closure
here was coverage checking and nothing else, and coverage checking is what
the gate replaces.

### Label shape: `KeywordLabel = String`

A bare alias, `VerbLabel`'s shape rather than `Subtype`'s constructor. The
label alone identifies the keyword — `Subtype` took a constructor because a
subtype needs a hosting card type ALONGSIDE its label, and a keyword needs
nothing alongside. `Eq Keyword`'s 34 hand-written arms become `Eq String`,
and Idris reduces primitive string equality during elaboration, so every
`So` gate that consumes a label discharges on a literal exactly as it did on
a constructor: `KeywordParamFits`, `KeywordCounterEligible`, `KeywordCost`,
`KeywordListOk`, and the new `KnownKeyword`.

### The five tables, collapsed

```idris
public export
record KeywordFacts where
  constructor MkKeywordFacts
  word : KeywordLabel
  paramShape : KeywordParamShape
  counterEligible : Bool
  regime : Maybe StackRegime
  onPermanentCard : Bool
  onSpellCard : Bool
```

| retired | where it went |
|---|---|
| `data Keyword` (34 arms) | `KeywordLabel` / `keywordFacts` |
| `Eq Keyword` (34 arms) | `Eq String` |
| `keywordParamShape` (34 clauses) | the `paramShape` field |
| `keywordCounterOk` (34 clauses) | the `counterEligible` field |
| `keywordStackRegime` (34 clauses) | the `regime` field |
| `keywordCardOk` (68 clauses, `Card.idr`) | the `onPermanentCard`/`onSpellCard` pair; the function is now two clauses over `CardClass`, none over a keyword |

Every reader keeps its old name and signature modulo `Keyword →
KeywordLabel`, so no call site outside the tables moved. Each is FAIL-CLOSED
on a word with no row — `keywordCounterOk` False, `keywordCardOk` False,
`keywordParamless` False, `keywordStackRegime` Nothing — which is what lets
the gates that already consumed them refuse a typo with no second check.
`keywordParamShape` alone answers `NoParam` for an unknown word, so
`keywordParamFits` folds `knownKeyword` in explicitly.

New gate, added where nothing else read a keyword: `KnownKeyword k = So
(knownKeyword k)` on `HasKeyword` and on `AbilityClass`'s `KeywordClass`
(now a GADT arm). `KeywordCounter`, `PaysCost` and `AlsoForKeywords` needed
none — `KeywordCounterEligible`, `KeywordCost` and `allParamless` were
already the fail-closed readers above.

### One data row, no macro at all

Bushido joined the vocabulary as

```idris
  , MkKeywordFacts "Bushido"          NumberParam  False Nothing             True  False
```

and nothing else. Witness: `Cards.nezumiRonin` — Nezumi Ronin, "Bushido 1"
[CR#702.45a] — written `Macros.keywordNumber "Bushido" (Lit 1)`, beside the
untouched `greaterMossdog` ("Dredge 3") which writes the same shape.

This clears the ticket's bar rather than meeting it: for keyword ABILITIES
the macro layer is already label-generic (`keyword`, `keywordCosting`,
`keywordQuality`, `keywordSubject`, `keywordNumber` take the word as an
argument), so a new keyword is one data row and no macro. The verb round
needed a macro per label because there the macro IS the expansion; here the
meaning is the keyword's own rule, which the workbench does not restate.

Each column of the row is that rule speaking: [CR#702.45a] writes the
ability as "Bushido N" (`NumberParam`); [CR#122.1b]'s counter list omits it
(`False`); it triggers on blocking, not before its object resolves
(`Nothing`); and it is an ability of a creature on the battlefield, which no
instant or sorcery card is [CR#110.4] (`True False`).

### Pins

Every pin naming a keyword passes the word as a VALUE and never matched on a
constructor, so all survived the conversion with the label re-spelled and no
discharge mechanism changed: `badKeywordContradiction`, `badKeywordOnInstant`,
`badBattlefieldConvoke`, `badBareWardLine`, `badWardQualityParam`,
`badParamOnNullaryKeyword`, `badProtectionOnInstant`,
`badProtectionFromPlayerRestriction`, `badEquipOnSorcery`,
`badKeywordEmblem`, `badBareCumulativeUpkeep`,
`badCumulativeUpkeepOnInstant`, `badCumulativeUpkeepCounter`,
`badKeywordListOnPlainLine`, `badEmptyKeywordList`,
`badKeywordListRepeatingBase`, `badBareUnearth`, `badUnearthOnInstant`,
`badFlashbackOnPermanent`, `badPayCostlessKeyword`, `badBareEcho`. None
retired, none weakened, none blocked.

Two new, both SPELLING pins in `ProofsE.idr` — [CR#702.1] leaves an unlisted
word naming no ability at all:

- `badUnknownKeywordLabel` — `KeywordAbility "Flyign" Nothing`, refused at
  `KeywordParamFits`, which now folds knownness in.
- `badUnknownKeywordPredicate` — `HasKeyword "Flyign"`, refused at
  `KnownKeyword`, the bare gate.

Pin probe: `badUnknownKeywordLabel`, `badUnknownKeywordPredicate`,
`badBareWardLine` and `badCumulativeUpkeepCounter` each perturbed to a real
word individually — all four then rejected the `impossible` clause (4/4).

### The `deckmaste_english_v2` delta (stated, not built)

None owed, and one mismatch closed. english_v2 already reads keyword names
as data (its scanner carries them as `lexeme:keyword_action/...` ids, not as
enum arms), `deckmaste_core::KeywordAbility` already carries the open
`Composite { name, abilities }` beside its five intrinsics, and the
declaration author's keyword vocabulary already lives in
`plugins/builtin/macros/keyword/` — 68 `.ron` rows today, against the
workbench's 34. The conversion means a keyword minted there no longer has a
workbench enum arm waiting for it either: the grammar's obligation for a new
keyword is now one `keywordFacts` row, the same kind of edit as the `.ron`
file. Nothing in the crate has to change for that.

### Ledger

- `Semantics.idr` (the semantics_v2 kernel mirror) keeps its own keyword
  shape and was out of this round's consumption boundary. Whether it wants
  the same treatment is not this ticket's question.
- `ConferringWord` and `AbilityWordName` are still closed enums in
  `Words.idr`. `AbilityWordName` has a rule behind it ([CR#207.2c]
  enumerates the ability words); `ConferringWord` does not, and is the next
  closed-vocabulary candidate in the same file.
- The DRIFTS keyword-residue exclusion is untouched: no fenced residue was
  worked here.

### Gates

- `idris/scripts/build` (full, from clean) — 23/23, 0 errors, 0 warnings.
- `cargo xtask cite check --list-noncompliant` — 0 non-compliant.
- `cargo xtask cite check` — 17,113 citations, 0 stale after blessing
  [CR#716.2] (the only rule new to the repo).
- `jj diff --git | cargo xtask cite audit --diff` — 47 sites read against
  their rules. [CR#702.1], [CR#711.2], [CR#714.2], [CR#716.2] and
  [CR#107.8,107.15,107.16] are the load-bearing ones and each says what the
  verdict says it says; [CR#205.4a] and [CR#122.1b] are the contrast, and
  both do enumerate their sets in the rule text.
