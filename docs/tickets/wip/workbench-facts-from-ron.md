---
needs: []
---
**Derive the facts-table rows from the RON stubs, make the parameter column a
per-row list of admitted shapes, and check the label sets match both ways.**
Fresh workbench review 2026-09-03, F3 + F7 + F16, resolved by ruling.

**Ruling (settled 2026-09-03): open string labels over facts tables stay;
enums are declined.** The rows are exactly the labels the Comprehensive Rules
define — keyword abilities [CR#702], keyword actions [CR#701] — plus the
counter kinds the CR names and the printed ones the corpus attests. A label
outside the tables is refused. Rows are **derived from the RON stubs**
(`plugins/builtin_v2/macros/stubs/{keyword_abilities,keyword_actions,counter_kinds,designations}/*.ron`;
fields `params`, `spelling`, `grammar`, `body`) wherever the stub determines
the column — `bodied`, `paidCost`, `regime`, and the admitted param shapes as
a **list** read off `params` — and hand-written only where a column exists
solely for an Idris gate. Custom (non-CR, non-corpus) label sets are a later
bridge, not this ticket.

**Per-row shape lists (F3).** `Words.KeywordFacts.paramShape` is one
`KeywordParamShape` and `Words.paramShapeFits` admits only that shape (plus
`CompoundParam h → CostParam`), so CR-defined variants are refused: P3
`keywordQuality "Hexproof" (ColorIs Black)` (Knight of Grace, "hexproof from
black" [CR#702.11d], 19 cards) → `So False`; P4 `keywordQualityCosting
"Cycling" (HasSubtype Plains) {2}` (Eternal Dragon, "plainscycling {2}"
[CR#702.29e], 88 cards) → `So False`; P4b the structurally identical Equip
compound is admitted. Fix: `paramShapes : List KeywordParamShape` (Hexproof
`[NoParam, QualityParam]`, Cycling `[CostParam, CompoundParam QualityHead]`,
landwalk-style rows unchanged) and `paramShapeFits` an `elem`.

**Column drift (F7).** `MkActFacts` takes 15 positional fields,
`MkKeywordFacts` 9 booleans, `Events.MkEventFacts` 8. Generated rows are
named-field record updates over a default row, so the F3 column addition is
local and a new column does not renumber every row by hand.

**Unknown labels (F16 and P3's message).** An unrecognised label fails today
as a bare `So False`. Give the membership gates a named witness
(`KnownKeyword`/`KnownAct`/`KnownCounter`-style) so the refusal names the
label and the table. Add the missing rows while there: `keywordFacts` for
Delve, Infect, Cascade, Prowess, Split second and Phasing, and for Decayed,
Exalted and Shadow with `counterEligible = True` [CR#122.1b]; `actFacts` for
Venture.

The check is an `xtask` subcommand run in both directions: every stub label
has a table row, and every table row has a stub. It fails on a label present
on one side only. The Idris keyword table carries ≈90 rows against 195
keyword-ability stubs and 70 keyword-action stubs, so the first run is a
worklist, not a green gate — land it with the difference recorded in the
landing record.

Size: S–M for the shape list and the rows; M for the generator and check.

Done when: Knight of Grace and Eternal Dragon are typechecking bench
witnesses; `paramShapeFits` is an `elem` over a per-row list; the facts rows
are generated from the stubs and the generated file is regenerable from a
clean tree; the label-set check passes in both directions (or names its
remaining difference and the ticket records it); an unknown keyword is refused
by a named witness and pinned, probed non-vacuous; P6's `KeywordCounter
"Ward"`/`"Shroud"` refusals still hold [CR#122.1b]; the build is 44/44 with 0
errors and 0 warnings. Standard constraints apply, plus the RON-shaped
constraint: a core constructor is admissible only if the RON re-emitter can
produce it from a RON node, and a macro only if it names a RON macro
(`docs/decisions/workbench-ron-shaped-and-label-rulings.md`).
