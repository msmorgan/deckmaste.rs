---
needs: []
---
# Test the parser-engine obligations of v3's `packed.rs` and `slice.rs` in `deckmaste_syntax`

`crates/deckmaste_english_v3/tests/suite/packed.rs` and `slice.rs` test v3's
parsing engine over synthetic grammars, not English, so they have no place in
the judgment set (`docs/english-v3-test-audit.md`). The properties below belong
to the engine contract (`docs/decisions/english-lexical-analysis.md` §§1–2) and
have no `deckmaste_syntax` test yet. Assert each over a small hand grammar in
`crates/deckmaste_syntax/tests/`, or fix the engine where it fails:

1. Lexical homographs yield distinct Readings; two Productions deriving the
   same tree yield one Reading (`derivation_count` still counts both).
2. A multiword lexeme is atomic beside the separate-word analysis of the same
   text, and needs its exact separators.
3. Nullable Categories complete in any discovery order, including when
   nullability is found late; an empty literal or nullable Category at the start
   of a Production is predicted and kept.
4. Duplicate empty-span derivations collapse to one Reading; an empty literal
   is a leaf.
5. Incomplete items pack homograph choices: chart size grows with input length,
   not with the product of homograph choices.
6. Constituents with equal summaries pack and distinct summaries stay distinct;
   incomplete prefixes share histories over different children.
7. Sibling feature alternatives stay correlated under agreement: one
   alternative per child, crossed combinations rejected, both by the chart and
   by `admit` (agreement is checked on a lexeme's joint alternative, never on
   the union of its alternatives).
8. A Gap left open by a descendant survives packing and is discharged only at
   the governing parent; an unclosed Constituent is never a root.
9. A unary cycle hides no finite sibling Derivation of the same Constituent.
10. A lexical choice the parent does not inspect (count vs mass use) still
    yields distinct Readings.
11. The chart never chooses among homographs (*cast* noun or verb; *one*
    numeral, determinative or noun): each reaches every Production accepting its
    category.
12. Recognition consumes exactly the supplied lexical analysis: removing one
    alternative removes exactly the Readings that need it.
13. Both round-trip laws hold for every declared lexical value (spelling
    variants, capitalization, non-ASCII surfaces) and for every member of small
    complete phrase families (every NP of a toy lexicon, every PP over every
    preposition, imperatives with both optional attachment sites).
14. Census units: per-unit alternatives are counted without expanding the
    product of a face's units; a newline inside a quotation does not split a
    unit; two segmentations of the same text are reported, not merged.

Standard constraints apply.
