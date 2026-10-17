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
ability spines compose with Stratum 1 imperative and finite active clauses.
Layered nominals preserve noun, adjective and participial premodifiers, genitives,
quantities and frame-selected complements. Oracle determinatives are explicit
catalog entries; status adjectives and numerical mass distributions are declared
in their owning lexical sources. Later constructions remain pending at their
own Stratum.

The Targeting Marker remains an attributive adjective after a determiner;
its homographic Target Noun is restricted to nominal heads. Preposition
frames selecting a bare nominal take a singular count-noun head and its
selected complements, without arbitrary noun compounds or the plural/mass
alternatives already supplied by Noun Phrases. Numeric arguments use Amount
frames; numeric subjects and predicatives retain Noun Phrase analyses.
Quantity nominals use general determination. Modified quantities admit the
indefinite article, and distributive quantities preserve singular external
agreement independently of the numeral–noun agreement inside the group.
Slash measures take nonnegative amounts with optional sign markers, so a
negative numeral does not duplicate the marker-plus-amount analysis; signed
zero and variables retain their notation. Attributive-only adjective position
is independent of determination, preserving bare plural `other types`.

Stratum 2 adds auxiliaries and modals as heads of their Verb Phrases with
bare-infinitival, perfect, progressive and passive complements; synthetic and
analytic negation (`can't`, `doesn't`; `not` after the copula or before a
*to*-infinitival); *to*-infinitival Clauses as Catenative Complements and as
Purpose Adjuncts (not on a head whose frame selects one); gerund-participial
Complements and preposed adjuncts; existential *there* with a Displaced
Subject; and predicative Complements of `be`, `become`, `remain`, `get` and
complex-transitive verbs, including Preposition Phrases that could modify a
nominal (*face down*, *of the chosen type*). Locatives after `be` and `remain`
are Locative Complements, not predicatives. Passives carry their `by` phrase as
a Complement, and Adjectival Passives come from frames whose passive leaves no
other Complement; they are predicative Complements only.

The profiler assigns reduced relatives and `named` postmodifiers to Stratum 3,
including adverbial relatives (*the next time a source would deal damage*),
relatives after keywords and symbols (*with flying that's attacking you*,
*mana value X whose controller*) and postmodifiers of a keyword or symbol
sequence; triggers to Stratum 4, with a keyword used as a verb (*as you
cascade*); asyndetic participle premodifiers (*nonattacking, nonblocking*) to
Stratum 6; and optional bracketed Oracle material and metalinguistic mentions
of a subtype (*with Vampire*) to Stratum 7.
Loyalty brackets retain their Stratum 0 notation role. Table rows inherit the
stratum of their payload.

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

## Inspecting a parse

```sh
cargo xtask english-v4 parse "+ <card=Seedborn Muse> Untap all permanents you control."
cargo xtask english-v4 parse --limit 2 'Destroy target creature.' '+NounPhrase target creature => NounPhrase "target creature" head creature' '*PrepositionPhrase target creature'
```

Quote each input separately. Inputs use the [judgment-entry syntax](judgments/README.md),
including category roots, card scopes and assertions. Bare Oracle text is parsed
as a positive entry at the grammar root. Balanced reminder text is stripped.
Each argument must contain one entry.

The command prints the profiled Stratum and Syntactic Families, vocabulary gaps,
and the Packed Forest's total derivation count. Explicit `+`/`*` entries and
inputs with assertions also print the judgment Outcome. Outcomes use the full
judgment evaluator, including its round-trip checks and 4096-derivation limit;
`--limit` only limits the display, not assertion evaluation.

Up to eight cheapest admitted Readings are shown by default. `--limit 0` prints
only the summary. Each Reading includes its realized text, Production Cost and
a constituent table: preorder node index (root 0), parent, Production id,
Category, byte span and surface in the realized text, grammatical function,
head lemma and lexeme id, and Gap. Judgment failures and unknown Categories are
reported without stopping later inputs; malformed entries and operational
errors fail the command. The dump is diagnostic output, not tracked judgment
evidence.

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

Modal headers take an imperative verb phrase followed by a dash (*Choose one —*),
with ordinary verb frames selecting their objects. Keyword catalogs distinguish
dashed verbal/composite costs from space-separated mana costs; mandatory dash
forms such as suspend retain their catalog punctuation. The pinned Vintage
corpus uses Unicode minus in all 520 negative loyalty lines (no ASCII hyphen-minus),
without display brackets; the grammar accepts that source form as well as
bracketed symbol notation.

Costed modes use bulleted costs, the corpus's Spree plus marker, pawprint
quantities, or a bulleted Flavor Word followed by its cost. A Saga chapter
scopes over the modal ability, not separately over its header. Activation
effects select sentences or modal abilities, so nested cost colons and bare
keyword lists are rejected. A symbol-cost-prefixed modal header is a Stratum 0
fragment; a verbal cost raises its profile to the cost’s Stratum. Bound-keyword
premodifiers belong to the Keyword Quality before the suffix and cost.

`ActivationCost` includes loyalty notation; keyword parameters select `Cost`,
which excludes it. Numeric zero in a neutral loyalty symbol remains a numeral
leaf. Mixed costs use one right-recursive structure, preventing duplicate trees
from overlapping symbol and verbal cost alternatives. Modal selection with
adjuncts or relatives belongs to the corresponding later strata.
