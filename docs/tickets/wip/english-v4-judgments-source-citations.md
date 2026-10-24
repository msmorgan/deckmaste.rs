---
needs: []
---
# Judgments: cite a source card for every non-verbatim entry

`crates/deckmaste_english_v4/judgments/README.md` now requires each entry to be either real Oracle text or a simplification of a named card that keeps the construction under test. A simplification cites its source with `<card=NAME>` or a `# from: NAME` comment. A sweep against the census corpus, using `Corpus::load` with corrections applied and reminder text stripped, found 969 of the 3,536 non-seed entries that are not a unit or a span of one: 797 `+` and 172 `*`. The list is in the sweep's `fails.tsv`, which can be regenerated with the same method. The biggest files are `grammar.txt` (109), `correlatives.txt` (43), `scalar_location_plain_be_and_become.txt` (42) and `verbless_state_predicate_class.txt` (35).

## Steps

1. **Match tolerantly.** Re-run the match, accepting a span whose first letter is capitalised in the entry but not on the card. About 91 entries should pass that way.
2. **Search for sources automatically.** For each remaining entry, find corpus units that share its distinctive words in order. Where one unit clearly has the entry's construction, add the citation.
3. **Classify the rest.**
   - A `*` entry pinning a wrong inflectional form (agreement, case, verb form) is kept as it is.
   - Any other entry is either replaced with real text (or a cited simplification) asserting the same structure, or retired with a reason: the construction no card prints. Keep each `!` assertion and Reading count on the replacement, since those keep spurious Readings off real text.
4. **Work file by file,** biggest first, as separate commits.

Produce a short list of entries the script could not source, for the owner to review before they are retired.

## Acceptance

- Every non-seed entry is verbatim, cites a source card containing its construction, or is an allowed inflection negative.
- All enforced judgments still hold.
- The census is unchanged.
- Each retirement is reported with its reason, per CLAUDE.md.

## Progress

Steps 1 to 3 are done mechanically; no entry was retired or replaced. A fresh sweep found 880 non-verbatim entries (91 more pass once the first letter's case is ignored). 135 now carry a `# from: CARD` comment (a source sentence checked to exist in that card's unit, or a match differing only in numbers or mana symbols). 150 `*` entries pin a wrong inflectional form and stay as they are. 595 remain for the owner to decide: 300 to retire (no card prints the construction), 251 to replace with real or cited text, 44 unsure. The list is kept outside the repo, with the proposed replacement for each entry; replacements are not yet run through the parser. Pass 2 ran the 251 replacements through the parser and judgment checker: 215 were adopted (each carries a `# from: CARD` comment above it, so 350 entries are now cited), keeping the original's marker, category, `!` assertions and Reading count. 36 stay as they were because the replacement did not hold under the same assertions: 9 would change the category (a span needing a different mark than the root), 5 would change the assertions, 5 have no Reading, 4 use undeclared vocabulary, 8 change the Reading count, 4 fail an assertion, 1 other. Of the adopted, 17 now duplicate an existing identical entry and can be dropped. Remaining for the owner: 300 to retire, 44 unsure, 36 failed replacements. Pass 3 finished the retirements.

Pass 3 applied the owner's rulings. Kept: the 21 `*` entries marked retire or unsure. Cited: 27 entries the sweep had left uncited (7 clitic pairs marked `(uncontracted pair)`), three `the player it's attacking` entries (Capricopian, the only single-head card that prints the construction) and the Junk type line (Junk Jet). Retired: 301 synthetic `+` entries, mainly paradigm grids (`verbless_state_predicate_class` 33, `scalar_location_plain_be_and_become` 31, `grammar` 25, `correlatives` 41, `partitive_determinative_heads` 15, the rest across about 50 smaller files). Of those, 42 were restored under a `# kept` comment, because each was the only judgment that exercised some Production (the correlative, shared-head and similar coordination files, plus a few single entries). Five real sentences were added to cover other Productions the retirements would have orphaned (Ichormoon Gauntlet, Riverfall Mimic, Agadeem's Awakening, Dark Matter Manipulator, Questing Druid). Replaced: the 36 failed replacements are resolved, most by a `<card=>` header, the full card sentence, or an adopted corrected span; `readings 1` was dropped on four coordinated-noun cards; `is crewed` became the cited `was crewed` and keeps `readings 2`. Deduped: 24 adopted entries that repeated an identical entry. The README now states the `*` rule and the `# kept` exception. Nothing remains open from the sweep.

## Landing note

Per `docs/tickets/README.md`, end with the two landing lines.

1. **Census delta** (pass 3): faces read 32,819 → 32,819; total Readings 338,318 → 338,318; Surplus 142,855 → 142,855; lost faces none; 0 units change (measured against the claim base's own census); `strata --check` passes.
2. **Judgment set**: all enforced judgments hold (`cargo test -p deckmaste_english_v4` and clippy pass); 259 entries retired net (301 retired, 42 restored as sole Production witnesses), 27 + 4 cited, 24 deduped, 36 failed replacements resolved, 5 real sentences added.
