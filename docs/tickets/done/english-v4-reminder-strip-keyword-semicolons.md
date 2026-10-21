---
needs: []
---
# Normalize keyword-list semicolons when reminder text is stripped

Oracle text separates the keywords on a keyword line with semicolons instead of
commas when a keyword on that line carries reminder text (*Flying; banding (…)*,
*Vigilance; horsemanship (…)*, *Trample; rampage 1 (…)*). The semicolon exists
only because of the reminder text. The census strips reminder text before
parsing (`crates/deckmaste_english_v4/src/corpus.rs`, via
`deckmaste_data::oracle_text::strip_reminder_text`), and the semicolon survives
the strip. That forces the grammar to keep `SemicolonKeywords`
(`english.ron`) beside `CommaKeywords`, and the Reading records which punctuation
the line used.

Corpus check (`data/scryfall/oracle-cards.jsonl`, Vintage-legal or restricted,
all faces): 31 lines still carry a separator semicolon after reminder text is
stripped, and every one of them had reminder text. None lacks it.

## Change

- In the prenormalization step that strips reminder text, replace a keyword-list
  separator semicolon with a comma when reminder text was stripped from that line.
  Leave semicolons inside reminder text and in sentences alone. Those vanish
  with the reminder text or end in a period.
- Delete `SemicolonKeywords`. `CommaKeywords` covers every keyword line.
- Rewrite any judgment that asserts a semicolon keyword list to the normalized
  text.

## Done when

- `SemicolonKeywords` is gone, and the 31 affected lines read through
  `CommaKeywords` with no face lost and no unit gaining a Reading.
- A corpus test pins the normalization on one real card, for example Shu General.

Standard constraints apply.

1. **Census delta** (measured on `suumltxq`): faces read 28,709 → 28,709;
   total Readings 346,725 → 346,725; lost faces: none. The 31 affected units
   now read as comma lists through `CommaKeywords`, each with its baseline
   Reading count.
2. **Judgment set**: passes (2,909 enforced, 191 pending).
