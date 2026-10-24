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

## Landing note

Per `docs/tickets/README.md`, end with the two landing lines.
