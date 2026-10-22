---
needs: []
---
# Port the corrected Oracle style guide from the mtg-rules plugin

The mtg-rules plugin carries a corrected copy of this repository's style guide:
`~/Projects/agent-plugins/mtg-rules/skills/mtg-rules/references/oracle-style.md`.
Port every difference into `docs/oracle-style-guide.md`, with one exception:
keep this guide's provenance line as it is. Do not add the copy's
"(effective 2026-09-25)".

Get the exact replacement text with
`diff -u docs/oracle-style-guide.md ~/Projects/agent-plugins/mtg-rules/skills/mtg-rules/references/oracle-style.md`.
As of 2026-10-21 that diff has 20 hunks: the provenance hunk plus the 19
listed below. The guide's front matter governs the port. Write in a copy-editing
register, with no parser, engine, Lean or corpus-count talk. Correct existing
`[CR#…]` citations, but add no new ones.

Provenance: `docs/evidence/mtg-rules-review-2026-10-21/style-guide-fixes.md`
(findings A1–A11, owner decisions B12–B13) and `departures-docs.md` entries
1–8. The evidence is gitignored, so read it from the default checkout.

## Hunks to take

Each hunk is located by the head text it replaces. The line numbers are
correct as of 2026-10-21.

| # | Section | Head text (locator) | Finding |
|---|---|---|---|
| 1 | §1 Voice and tense (~126) | "permanents *enter*, creatures *die*," becomes "permanents *enter* and *die*," | A4 (`dies` is not limited to creatures [CR#700.4]). The finding did not list this passage, but it is the same fix. |
| 2 | §4 numbers (~468) | "This convention continues beyond ten." becomes the bounded rule: spell out through ninety-nine, both forms at one hundred, digits above one hundred, no conversion when revising | A1 |
| 3 | §4 digit grouping (~515) | "the ability word `10,000 Needles`" becomes "the flavor word" | A11 [CR#207.2c,207.2d] |
| 4 | §4 comparisons (~567) | "For a variable or expression, use `less than`, …" becomes `X or less`/`X or greater` for a bare variable, a strict bound for `less than X`, and `… or equal to` only before a multiword standard | A2 |
| 5 | §5 "This way, that much, and linked events" (~677) | "Use `this way` to restrict a reference to the immediately described action …" becomes "ranges over every such event … including events from paying the spell's or ability's costs", with the two-exile and cost-exile examples | A3 |
| 6 | §6 "Determiners and modifier order" (after "separate them with commas: `target nonartifact, nonblack creature`.") | New paragraph on what `another`/`other` exclude: an earlier mention in the ability (Bond of Passion), a later mention in the same sentence (Inquisitor's Flail), otherwise the source | B12 |
| 7 | §6 Target (~758) | "Use `another target creature` to exclude the source or first referent." becomes a cross-reference to hunk 6 | B12 |
| 8 | §7 "Type changes and retention" (~1022) | "Use `in addition to its other types` when the previous types remain" and "Use each retention formula only for its prescribed function." become: both phrases have the same effect [CR#205.1b]; choose by sentence shape; `is also a [type]` is an acceptable variant for a source's own fixed types | A8 |
| 9 | §8 "Enters and dies" (~1192) | The whole subsection body becomes the owner's `enters` ruling (short form for present-tense events, including before `under [player]'s control`; long form in a look-back that names a controller and for entering from another zone or a prohibition on entering; short form in a look-back with no controller; short form in the "entering doesn't cause abilities to trigger" statics, with the long form acceptable). The `dies` paragraph becomes "any permanent put into a graveyard from the battlefield". | B13, A4 |
| 10 | §10 "Modal choices" (~1602) | "Its controller chooses the mode or modes … [CR#700.2,700.2a,700.2b]" becomes: modes are chosen at casting, activation or stack placement [CR#700.2a,700.2b]; a player the header names chooses at that point; the controller picks the chooser [CR#700.2e]; `choose one at random` is decided at the same point | A5. This also covers the choices.md audit #11 that `overview.md` merges here. |
| 11 | §12 "Moving cards and permanents" (~1836) | "`dies` only for a creature going from battlefield to graveyard" becomes "for any permanent going from the battlefield to a graveyard" | A4 |
| 12 | §12 Tokens (~1876) | "Append `that's tapped and attacking` when a newly created noncopy token enters combat. A copy uses the distinct fixed order …" becomes: entering combat doesn't tap [CR#110.5b,508.1f]; `that's attacking`/`that's blocking [creature]` for untapped tokens; `that's tapped and attacking` only when the effect taps; both orders are acceptable for either family; keep a card's existing order | A6 |
| 13 | §12 Tokens, named tokens (~1895) | "Use the appositive form required for a named token … [CR#111.9]" becomes: the appositive is house style, and the rules treat it as creating a token with that name [CR#111.9] | A9 |
| 14 | §12 Counters (~1948) | "Use `has` for testing and omit redundant `on it`: `if it has a stun counter`" becomes: keep `on it`/`on them` with an object as subject, and use no `on` with a player as subject | A7 |
| 15 | §12 Colors and types (~2018) | "`Still a land` and `in addition to its other types` carry different rules instructions …" becomes "have the same rules effect", with a cross-reference to hunk 8 | A8 |
| 16 | §12 Combat (~2043) | "For a creature placed into combat …, use `tapped and attacking`" becomes: no attack- or block-trigger wording; entering combat doesn't tap; `attacking`/`blocking [creature]` untapped; `tapped and attacking` only when the effect taps | A6 |
| 17 | §13 skip (~2277) | "For a one-shot skipped turn" becomes "For a single skipped turn" (a skip is a replacement effect [CR#614.10]) | A10 |
| 18 | §13 continuous (~2309) | "State one-shot changes with `becomes`, `gets`, `gains`, or `loses` …" becomes "changes made by a resolving spell or ability … with no duration stated, the change lasts until the end of the game"; the `also` sentence is narrowed to `becomes`/`gets` sentences and cross-references hunk 8 | A10 [CR#610.1,611.2a], A8 |
| 19 | §16 final review (~2697) | "Give each one-shot temporary change a duration …" becomes "each temporary change made by a resolving spell or ability" | A10 |

Every finding (A1–A11, B12, B13) maps to at least one hunk. No hunk lacks a
finding. Hunk 1 is the only one that extends a finding beyond the passages the
finding lists.

Citations: hunk 10 replaces `[CR#700.2]` with `[CR#700.2e]`. This corrects an
existing citation rather than adding a new one, so it needs `cargo xtask cite
bless`. The other hunks keep or remove existing citations. The bracketed rules
in the table above are this ticket's evidence and are not to be added to the
guide.

## Departures merged here

`departures-docs.md` entries 1–8 and the two style-guide items in
`overview.md` are all fixed in the copy:

- Numerals beyond ten: hunk 2. Attested at head: Baldin, Century Herdmaster
  prints "one hundred"; Helix Pinnacle and Vexing Puzzlebox print "100"; Battle
  of Wits and The Millennium Calendar print digits above one hundred.
- Comparative forms: hunk 4.
- `this way` scope: hunk 5.
- `dies` not limited to creatures [CR#700.4]: hunks 1, 9 and 11.
- Modes chosen by another player [CR#700.2e]: hunk 10.
- `tapped and attacking` for every placement into combat [CR#110.5b,508.1f]:
  hunks 12 and 16. The new examples are attested: Brimaz, King of Oreskos;
  Flash Foliage; Launch the Fleet; Pugnacious Pugilist; Combat Calligrapher;
  Mirror Match.
- `has a counter` without `on it`: hunk 14.
- `Still a land` versus `in addition to its other types` [CR#205.1b]: hunks 8,
  15 and 18.
- Named-token appositive overstating [CR#111.9]: hunk 13.
- Continuous `becomes`/`gets` changes called one-shot, which contradicts
  [CR#610.1,611.2a]: hunks 17, 18 and 19.

The port adds no fixes beyond the copy.

Finding A3's other half is outside this guide: reconcile the per-action gloss
of `thisWay` in `lean/CONTRACTS.md` with its multi-event entry, and add a
Sinister Concierge pin. That is Lean work and is not in this ticket. The
same goes for the parser and realizer notes in A1 and A6 (accept both
numeral forms and both token orders, and keep each card's order in round
trips). Those belong to the English v4 grammar.

## Open questions for the owner (undecided)

1. **"Search and shuffle" voice.** Head (~1809), unchanged in the copy: "The
   Vintage corpus uses `shuffle your deck` only on three Draft Matters cards,
   all in this pregame template:". The sentence is corpus-count talk, which
   the guide's register excludes. Should it be reworded as a template
   statement, for example "Use `shuffle your deck` only in the pregame
   template:"?
2. **Brainstealer Dragon.** Hunk 9's `enters` ruling (short form before
   `under your control` in a present-tense trigger) supports the correction
   recorded under "Missed-template-update entries" in
   `docs/tickets/planned/english-v4-oracle-editorial-corrections.md`. The
   owner may lift that erratum. If it is lifted, remove the Brainstealer Dragon
   item from that ticket. This ticket does not edit that ticket.

## Done when

- `diff` against the plugin copy shows only the provenance hunk, plus
  whatever the open questions decide.
- `cargo xtask cite check --list-noncompliant` is empty, `cargo xtask cite
  check` reports 0 stale, and `[CR#700.2e]` is blessed.
