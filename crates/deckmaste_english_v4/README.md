# deckmaste_english_v4

Oracle English for English v4
([`docs/decisions/english-v4.md`](../../docs/decisions/english-v4.md)): the
grammar data file [`grammar/english.ron`](grammar/english.ron) and its binding
into `deckmaste_syntax`.

The grammar names Categories, features, Schemas and core closed-class lexemes
only. Vocabulary and Subcategorization Frames come from the lexical environment
the caller binds it with: the seed in `tests/seed/` for the seed proof, and
`deckmaste_lexical_source_v4`'s core lexicon and Catalogs for the strata.
`tests/catalog_leak.rs` checks that the grammar names only core closed-class
lexemes of that lexicon and that none of its own names spells a catalog item.

## Seed proof

`tests/seed_proof.rs` proves the frame engine end to end on a seed of about
fifteen open-class words: active complementation, same-head complement priority,
Host-Category-limited attachment with cross-head ambiguity retained (including
Seedborn Muse), passives with and without `by`, object and stranded-preposition
Gaps across finite, progressive, perfect and passive forms, and Right Node Raising.
Every Reading is admitted and realizes to its input; hand-rebuilt Readings reparse.
`chart_and_forest_sizes` prints chart items, forest nodes, families and Readings
per seed sentence.

## Corpus and units

`corpus` reads the supported corpus (Vintage legal or restricted, outside
`set_type: funny`) from `data/scryfall/oracle-cards.jsonl` through
`deckmaste_data`, one `Face` per Card Face record with every name of its card in
lexical scope. Each face yields Corpus Units: its Type Line, and each paragraph
line of its rules text after balanced reminder text is removed
(`deckmaste_data::oracle_text::strip_reminder_text`). The split needs no parse.

## Complexity Strata

`profile` assigns each unit its Complexity Stratum (`docs/decisions/english-v4.md`
§6.1). A unit is read as its longest lexical analyses; its Syntactic Skeleton
replaces each open-class catalog item with a placeholder (`<Subtype>`,
`<KeywordAction>`, ...). Its Syntactic Families are recognized from lexical
categories, word forms and declared frames: an outer spine (labels, modal
headers, bullets, `Cost : Effect`, keyword lines) is Stratum 0, and each family
inside belongs to its Stratum (an auxiliary to 2, a contact relative to 3, a
preposition with a clause complement to 4, a coordinator to 6, *instead* or a
quoted ability to 7, ...). The unit takes the highest (Max-Stratum Composition
Rule). The rules are a partition for planning and gating, not a grammar: a
Stratum ticket that finds a unit misplaced fixes the family rule, and every unit
like it moves with it.

## Census

`census::run` profiles and parses every unit and reports it **read** (a Reading
exists, round-trip law 1 holds over the whole Packed Forest and law 2 on its
cheapest Readings), **known-wrong-only** (a judgment for that exact text is
satisfied by none of its Readings) or **unread** (with the reason: the grammar
does not bind, a vocabulary gap, no Reading, or a law failure, which is a
defect). Counts are per Stratum, with faces counted at their highest unit, plus
per Syntactic Family and the most frequent skeletons. Given an earlier census's
face verdicts as a baseline, it also places every face that census did not read.
No attachment tree is enumerated except for a unit with a judgment.

`COMPLETED_STRATUM` is the highest Stratum whose gate is met. A unit at or below
it that is not read is a regression.

```
cargo xtask english-v4 strata [--card NAME]... [--baseline target/english-v3/<census>.json] [--check]
```

prints the summary and writes the full per-unit report to
`target/english-v4/strata.json` (untracked). `--check` fails on a regression.

The full lexical environment binds. Stratum 0 covers Type Lines, complete
keyword lists and non-clausal parameters, symbolic costs and structured
headers. `Document` composes lines with explicit paragraph boundaries. Outer
ability spines are also checked over seed clauses; later clausal syntax remains
pending at its own Stratum.

The profiler recognizes bound stems and preposed qualities without treating
keyword verbs as keyword heads. A relative after a numeral (*two that share …*)
belongs to Stratum 3 even inside a keyword parameter.

A `KeywordPhrase` distinguishes supplied parameters from an omitted-parameter
mention. Keyword lists require supplied parameters; noun modifiers and selected
keyword complements can use mentions. Comma and semicolon keyword separators
are distinct Productions, so their Readings retain the permitted spelling
choice. Other punctuation and symbol attachment are derived from structure.

The load report identifies the one-member `RangeMarker` lexical class: it
contains the level-range notation marker. This keeps the marker's spelling in
the lexical environment; it is not an attachment-policing feature. The
`Pawprint` singleton likewise distinguishes a mode-count symbol from a cost
symbol. Both classes are authored with the symbols in the lexical data.

## Judgment set

[`judgments/`](judgments/README.md) holds the hand-written judgments, one line
per entry. `tests/judgments.rs` always enforces `seed.txt` on the seed lexicon
and enforces every other entry whose text profiles at or below
`COMPLETED_STRATUM`; the rest are pending. Every evaluated entry also checks
both round-trip laws on each of its Readings. `stratum_zero.txt` is also
always enforced in full, including its outer spines over seed clauses.
`tests/stratum_zero.rs` independently constructs keyword-list Readings with
both separators, checks multiline document boundaries, and proves that a new
catalog keyword's parameters bind without changing the grammar. The census uses the same entries
to find known-wrong-only units.

The judgments migrated from `deckmaste_english_v3`'s test suite, and the reason
each remaining v3 test retires, are recorded in
[`docs/english-v3-test-audit.md`](../../docs/english-v3-test-audit.md).

Modal selection is a catalog-declared construction of *choose*, whose quantity
projects to a Noun Phrase Object. Ordinary verb phrases cannot introduce mode
lists. Keyword catalogs distinguish dashed verbal/composite costs from
space-separated mana costs; mandatory dash forms such as suspend retain their
catalog punctuation. The pinned Vintage corpus uses Unicode minus in all 520
negative loyalty lines (no ASCII hyphen-minus), without display brackets;
the grammar accepts that source form as well as bracketed symbol notation.

Costed modes use the corpus's Spree plus marker, pawprint quantities, or a
bulleted Flavor Word followed by its cost. A Saga chapter scopes over the modal
ability, not separately over its header. Activation effects select instructions,
so nested cost colons and bare keyword lists are rejected. Bound-keyword
premodifiers belong to the Keyword Quality before the suffix and cost.
