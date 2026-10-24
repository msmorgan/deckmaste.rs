---
needs: []
---
# Stratum 7: Cleave brackets as transparent text

Part of `english-v4-stratum-7-discourse-and-long-tail`. Implement in `crates/deckmaste_english_v4` and `deckmaste_syntax` per [`docs/decisions/english-v4.md`](../../decisions/english-v4.md) §4.4.

## Why

Cleave is a text-changing effect: if the cleave cost was paid, the spell's text changes by removing the text in square brackets ([CR#702.148a]; check the exact wording). The brackets are not a grammatical constituent. The current treatment (`english-v4-s7-cleave-brackets`, change `xplzttyr`) reads them syntactically:

- a bracketed "twin" of every Production with an `adjunct` or `modifier` child, created at bind time; it adds chart items even where no brackets occur;
- a `BracketedSentence` Production.

That cannot read Dig Up, *Search your library for a [basic land] card, [reveal it,] put it into your hand, then shuffle.*, because *[reveal it,]* swallows the serial comma.

## Change: investigate a single pass first

1. **Balanced brackets may appear anywhere.** The lexer records each balanced `[ … ]` span on a cleave card as an annotation outside the Reading, as punctuation is, and the grammar sees the text without the bracket characters. So the full, uncleaved text is parsed once by the ordinary grammar. Round trips rebuild the brackets from the annotation.
2. Remove the bracket twins from the engine and `BracketedSentence` from `english.ron`. Square brackets then mean only loyalty costs (`[+1]:`). Detect cleave brackets by the card's Cleave keyword line.
3. **Investigate the cleaved text.** Can the cleaved text be obtained, and shown to be English, without a second parse? Two options to try:
   - delete the annotated spans' tokens and check whether the remaining chart already contains a complete parse;
   - derive the cleaved tree from the uncleaved one.

   The review expects both options to fail, for the reasons in the review notes below (chart span adjacency on Dig Up; tree surgery across *[reveal it,]* and Lantern Flare's *[creature or]*), and a second parse of the cleaved stream on the 12 cleave faces is the likely outcome. Confirm this quickly rather than building either option out.

   Report what works on all 12 cleave faces, Dig Up's *[reveal it,]* in particular. A second parse of the cleaved string is the fallback, and is not part of this ticket unless the investigation shows nothing simpler works.

## Witness cards

- Dig Up, currently unread.
- The 11 cleave faces read by `xplzttyr` must keep reading: Lantern Flare, Alchemist's Gambit, Winged Portent, Inspired Idea, Fierce Retribution, Alchemist's Retrieval, Wash Away, Lunar Rejection, Path of Peril, Parasitic Grasp, Dread Fugue.

## Acceptance

- Census before and after, unit by unit: Dig Up reads, the 11 still read, and no other unit changes.
- Measure the chart or wall-time saving from dropping the twins.
- Rewrite the `cleave_bracketed_text.txt` judgments and `crates/deckmaste_syntax/tests/bracketed_text.rs` for the new treatment, and say which were rewritten or retired, with reasons.
- Update the glossary's "Bracketed Constituent" entry (or retire it) and the syntax README.
- Report the cleaved-text investigation.

## Grounding

- [CR#702.148a].
- `docs/oracle-style-guide.md` §3, "Bullets, vertical bars, and square brackets".

## Investigation: the cleaved text without a second parse

- **Deleting the bracketed tokens from the uncleaved chart** fails. A chart item spans contiguous input, so the Cleaved Text needs items the uncleaved parse never builds. On Dig Up, *a card* needs *a* and *card* adjacent, and *Search your library for a card,* then needs a conjunct predicted at *put*; in the uncleaved chart only *reveal* follows *card,*. Building those items is parsing.
- **Deriving the cleaved tree from the uncleaved one** fails as a shortcut. Dropping an unselected dependent is local (*[attacking]*, *[with flying]*), but Dig Up's *[reveal it,]* is a conjunct plus the comma that delimits it, so the four-clause coordination has to be rebuilt as a three-clause one, not pruned. The rebuilt tree would still need Admission (agreement, *a*/*an*, comma licensing), and that check is the parser's own. Lantern Flare's current Oracle text has no *[creature or]*: its brackets hold a whole sentence.
- **Outcome**: the census parses the Cleaved Text as a second text, only for the 12 bracketed cleave lines (`deckmaste_english_v4::cleave`). The engine's bracket twins and `BracketedSentence` are deleted.

## Landing note

Per `docs/tickets/README.md`, end the ticket with these two lines, stamped with the change id they were measured on:

1. **Census delta**: faces read and total Readings before and after; lost faces by name, or "none".
2. **Judgment set**: the result of running it.

## Review notes (hypotheses, verify first)

- Both no-second-parse options likely fail. Deleting tokens breaks chart span adjacency: on Dig Up, *card,* and *put* were never adjacent in the uncleaved chart. Deriving the cleaved tree needs tree surgery wherever brackets cross a constituent: Dig Up's *[reveal it,]*, and Lantern Flare's *[creature or]* (a conjunct plus its coordinator).
- Likely outcome: parse the cleaved token stream on the 12 cleave faces, at negligible cost.

Chart saving from dropping the twins (`ktrqkuqw`, every corpus unit parsed once, single-threaded debug build): 400,852,900 → 313,289,996 chart items (−21.8%), parse time 401 s → 341 s (−15%).

Census delta (measured on `ktrqkuqw`): faces read 32,769 → 32,770 (Dig Up, 0 → 1 Reading); total Readings 337,348 → 337,354 and Surplus 141,589 → 141,594 (Wash Away 2 → 6 and Lunar Rejection 1 → 2: the plain text's own Readings, as on any face; the bracketed twin had read fewer); the 11 other cleave faces still read, and no other unit changed. Lost faces: none.
Judgment set (`ktrqkuqw`): 3,548 enforced and 3 pending, all enforced holding; `cleave_bracketed_text.txt` rewritten as 24 entries, each witness's Uncleaved and Cleaved Texts.
