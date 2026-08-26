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
