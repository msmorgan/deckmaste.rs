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
inside belongs to its Stratum (an auxiliary to 2, a bare relative to 3, a
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

Each unit's count is reported two ways. `derivations` is the size of its Packed
Forest, the number of trees: the census's no-Reading verdict and the judgment
evaluator's 4096-derivation limit use it, and `total_derivations` sums it. It
multiplies, so a unit of several sentences, each a little ambiguous, can count
in the thousands. The unit is therefore also split into its **Segments**
(`OUTER_CATEGORIES`): the walk descends from the root through the line, its
top-level sentence sequence, activation costs, labels, modes and modal headers,
and a Keyword Line's items and parameters, and each first Constituent of another
Category (always a `Sentence`), or leaf of an outer Production, is a Segment,
identified by its extent without the structural punctuation at its edges. A
sentence sequence nested in a Segment, as in a quoted ability, stays in it. Each
unit reports its Segments with their Derivations and its **Surplus**,
Σ(nᵢ − 1) over them, which is 0 exactly when the unit is unambiguous; the
report sums it in `total_surplus` and counts ambiguous units and Segments. A
unit with a Derivation whose Segments differ between Derivations, or do not
multiply to its count, is not segmentable (its Surplus is `derivations − 1`);
the summary lists it and `--check` fails on it.

Each unit also reports its `cheapest_tier`: how many of its Derivations share
the least Ranking Key (`docs/decisions/english-v4.md` §4.3), before the final
tie-break, counted on the Packed Forest without building trees. The summary
gives how many ambiguous units have a cheapest tier of one, the tiers' sum over
ambiguous units, and the largest tier. Ranking never changes which Readings
exist: verdicts, Readings and Surplus do not depend on it.

`COMPLETED_STRATUM` is the highest Stratum whose gate is met. A unit at or below
it that is not read is a regression, unless [`deferred.txt`](deferred.txt) names
it by card and text: a deferred unit is left unread on purpose, owned by a
ticket, and counted apart. A law failure is never deferred, and a deferred unit
that reads is stale.

```
cargo xtask english-v4 strata [--card NAME]... [--baseline target/english-v3/<census>.json] [--check]
```

prints the summary and writes the full per-unit report to
`target/english-v4/strata.json` (untracked). `--check` fails on a regression, a
stale deferral or a unit that is not segmentable.

The full lexical environment binds. Stratum 0 covers Type Lines, complete
keyword lists and non-clausal parameters, symbolic costs and structured
headers. `Document` composes lines with explicit paragraph boundaries. Outer
ability spines compose with Stratum 1 imperative and finite active clauses.
Layered nominals preserve noun, adjective and participial premodifiers, genitives,
quantities and frame-selected complements. Oracle determinatives are explicit
catalog entries; status adjectives and numerical mass distributions are declared
in their owning lexical sources. Later constructions remain pending at their
own Stratum.

The Targeting Marker `det:target` ends a Determiner Phrase: alone, singular
(*target creature*, *target Merfolk*); after an adjective (*other target
creatures*); or after an outer determiner, which gives it its Number (*two target creatures*, *each other target creature*, *[one or two] target
creatures*), including a maximum Preposition Phrase (*up to one target
creature*). Alone in the plural oblique of a Number-transparent noun it
takes that plural (*any number of target Equipment*). An ordinary Determined Noun Phrase or possessor then takes it,
so every postmodifier attaches inside the Nominal, below the determiner
(*target [creature you control]*). An
attributive adjective that needs an overt determiner (*the only creature*,
an ordinal) is an ordinary premodifier. Its homographic Target Noun is
restricted to nominal heads. Preposition
frames selecting a bare nominal take a singular count-noun head and its
selected complements, without arbitrary noun compounds or the plural/mass
alternatives already supplied by Noun Phrases. Numeric arguments use Amount
frames; numeric subjects and predicatives retain Noun Phrase analyses.
Quantity nominals use general determination. Modified quantities admit the
indefinite article, and distributive quantities preserve singular external
agreement independently of the numeral–noun agreement inside the group.
A bare slash measure is a Measure Noun Phrase only where a head's frame
selects one (*gets +2/+2*, *gets twice -X/-X*, *your choice of 5/1 or 1/5*)
and a predicative Complement after *be* or *become* (*it's 4/3*); it is a noun
only under a determiner (*an additional +2/+2*), as a bracketed loyalty symbol
is, so neither is a general Object, a Manner Adjunct or the head of an
attributive noun (*Draw a card 3/3.* has no Reading).
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
Subject and an optional locative or temporal Extension, a Complement of *be*
that is never also the Displaced Subject's postmodifier; and predicative Complements of `be`, `become`, `remain`, `get` and
complex-transitive verbs, including Preposition Phrases that could modify a
nominal (*face down*, *of the chosen type*). Locatives after `be` and `remain`
are Locative Complements, not predicatives. Passives carry their `by` phrase as
a Complement; it is *by* plus a Noun Phrase that is neither a fused-head
quantity (*two*) nor a braced symbol (*{2}*), so a selected *by* extent (*reduced by
{2}*, *increased by two*) is never also the Internalised Complement. Adjectival Passives come from frames whose passive leaves no
other Complement; they are predicative Complements only.

The profiler assigns reduced relatives and `named` postmodifiers to Stratum 3,
including adverbial relatives (*the next time a source would deal damage*),
relatives after keywords and symbols (*with flying that's attacking you*,
*mana value X whose controller*) and postmodifiers of a keyword or symbol
sequence; triggers to Stratum 4, with a keyword used as a verb (*as you
cascade*); asyndetic participle premodifiers (*nonattacking, nonblocking*) to
Stratum 6; and optional bracketed Oracle material and metalinguistic mentions
of a subtype (*with Vampire*) to Stratum 7. A stranded auxiliary whose Verb
Phrase is understood (*each opponent who can't loses 3 life*) is Ellipsis,
Stratum 7, like pro-form *do*.

Stratum 3 adds Relative Clauses as Postmodifiers of a nominal, a quantified
nominal (*the next 3 damage that would be dealt*) or, when marked, a fused head
(*one that hasn't been chosen*). A bare relative has an object, predicative or
stranded-preposition Gap, and its Clause opens with its Subject, never with a
preposed adjunct (CGEL, Ch. 12 §3.5.5, p. 1055); *that* or a relative pronoun
also introduces a subject Gap, which agrees in number with the antecedent; a
*whose* Relative Phrase is Subject or fills an object or predicative Gap (*the
player whose turn it is*); a Preposition Phrase may be fronted with its
relative pronoun (*for which*); an adverbial relative of a temporal nominal has
an Adjunct Gap (*the next time you cast a spell*). Fused relatives (*whichever comes first*) are Noun Phrases.
Gerund-participial and passive past-participial clauses postmodify the same
hosts and a gerund-participial clause is also a depictive Adjunct (*onto the
battlefield attacking*). A temporal Noun Phrase may be preposed (*This turn,
...*), interleaved before a selected Complement (*dealt to it this turn by
...*) or postmodify a temporal nominal (*the first time each turn*). *Cost*
selects a hollow *to*-infinitival (*costs {0} to cast*), and a purpose adjunct
may contain the Gap a relative clause binds (*the first spell that mana from a
Treasure was spent to cast*).

A temporal Noun Phrase Adjunct is a period noun with a deictic or distributive
determinative and no postmodifier (*this turn*, *each combat*, *this game*),
serial *time* with any determinative (*any time*, *an additional time*, *three
times*), or a temporal nominal with a relative clause (*the next time you cast a
spell*); *the turn* and *your turn* are ordinary Noun Phrases. A period noun
with a deictic or distributive determinative is otherwise a Period Noun Phrase,
only where a head's frame selects one (*skips that turn*, *the beginning of each
end step*, *after this phase*), so it is never a general Object (*can't block
this turn* has no Object). A marked relative
clause may be postposed past a predicate adjunct, its Antecedent the Object that
fills the verb's Gap (*Choose a card at random that was exiled with ...*),
ranked behind the relative in place. A single postpositive participle ranks
behind a participial clause and is never a status adjective's verb (*onto the
battlefield tapped* has only the depictive *tapped*). A copular verb takes a
central adverb before its predicative Complement (*become no longer
suspected*); a symbol sequence takes a postmodifier (*each {G}{G} spent to cast
it*); a bracketed loyalty symbol is also a counted noun (*an additional [+1]*).
Loyalty brackets retain their Stratum 0 notation role. Table rows inherit the
stratum of their payload.

Stratum 4 adds Clause Complements of prepositions: a finite Clause (*when this
creature enters*, *unless its controller pays {3}*, *as though it had flash*)
or a reducible verbless one (*if able*, *while saddled*). A triggered ability
[CR#603.1] is a Clause with its *when*, *whenever* or *at* phrase preposed as
an ordinary Adjunct; an intervening *if* phrase is a second one, and a modal
header is a Clause too (*When this creature enters, choose one —*). An
interrogative content clause, the Complement of *choose*, *guess* or *decide*,
is *whether* with a finite or *to*-infinitival Clause, a *which* phrase as
Subject or filling a Gap, or *how* with a Clause. *, where X is ...* is a
Supplement to a Clause, a keyword or an indexed instruction, never a Predicate
Adjunct. A comma-set post-adjunct (*, except it has flying*, *, unless …*) or
Supplement attaches to a finite Clause, never to a Verb Phrase, a
*to*-infinitival or a relative's Clause. Only such a comma-set Preposition Phrase takes a
comma-delimited Clause coordination as its Complement (*, except it's an
artifact, and it has flying*). A front adverbial is set off by a comma (*Once each turn, ...*). A
gerund-participial Clause may be Subject; a bare relative may relativize a
subjective predicative Complement for an ordinary noun, under a lexical verb or
a negative auxiliary (*the permanent it becomes*, *a color the other isn't*),
never affirmative *be* (*the player it's attacking* has an object Gap); a bare
relative postposes past *at random* (*choose a creature at random this Aura can
enchant*); *all* floats after a pronoun Subject. A Prepositional verb has a
stranded passive (*each card looked at*), and a passive participial Supplement
follows a Clause (*, followed by an additional main phase*). A preposed
Preposition Phrase and temporal Noun Phrase share one comma (*For one spell this
turn, ...*). A symbol sequence is a noun under a determiner (*an additional
{G}*, *six {E}*, *each {B} in a cost*) and premodifies a noun (*its {1}{G}
kicker*), as an Arabic numeral is a singular noun under one (*a natural 20*);
a noun premodifier never precedes a determined head (*amount {G}*). Keyword
rules declare the nouns they define (*if tribute wasn't paid*). *Do the same* is a pro-form, at
Stratum 7 with *do so*. *The same is true for ...* is one too, and a
text-changing effect's *instances* of words are a metalinguistic mention.

Stratum 5 adds partitives, comparisons and arithmetic. A determinative heads a
partitive Noun Phrase with its selected *of* oblique (*each of them*, *all of
that mana*), which takes the oblique's Number, a distributive head's singular,
or with free-choice *any* either (*if any of those cards shares ...*); a
quantity takes one too (*two of those cards*, *that many of those tokens*, *up
to two of them*, *a third of their life*), and a relative clause may follow
(*each of those cards that has suspend*). A number-transparent noun under *any*
takes its oblique's Number (*any number of target creatures become ...*); *the
number of* stays singular. *All* is two determinatives, as *more* is: one over
a count plural, one over a non-count singular (*all Equipment* is plural, *all
damage* singular). *Up to N* is a Preposition Phrase in Determiner
function or a fused head (*choose up to one*); *at least*, *exactly* and *no*
modify a quantity (*no more than one creature*). A comparative determinative
selects *than* (*more than one creature*, *more than twice*) and may postpose
it past its head (*more lands than you*); a comparative adjective selects it
(*greater than 2*), postposes it as an indirect Complement (*greater power than
this creature*), as *same* does *as*, or takes a measure phrase (*2 greater*,
*{1} less*, *at least 7 life more than ...*). A postpositive Adjective Phrase
with a Complement follows its noun (*damage equal to its power*, *turns other
than yours*). Superlative quantifiers determine under *the* (*the most
creatures*); a fraction or multiplier precedes a determined Noun Phrase (*half
their life*, *double that damage*), and *, rounded up* is a participial
Supplement, of a verbal cost too. *Plus* and *minus* head Preposition Phrases
on a quantity (*2 plus the number of ...*, *X plus 3 life*, *that many plus
one*), and *N times X* multiplies an amount.

Stratum 6 (complete) coordinates Clauses, Verb Phrases, Noun Phrases,
nominals, noun and participle premodifiers, Adjective, Preposition and Adverb
Phrases, keyword phrases, relative and interrogative clauses, determinatives
and symbol costs, binary and serial, after *both*, *either* or *neither*, and
in a delimited polysyndetic chain (*A, or B, or C*). An *and*-coordination of
Noun Phrases or nominals is plural, but singular when every conjunct is
determined by a distributive determinative (*each attacking creature and each
blocking creature doesn't untap*); *and/or* is plural after a plural last
conjunct and of either Number after a singular one; an *or*-coordination of
nominals shares its conjuncts' Number and of Noun Phrases takes its last
one's. Every conjunct of a Noun Phrase coordination but the last shares its
Determination; the coordination is determined only when the last is too
(*all artifacts and enchantments* has no predeterminer over coordinated bare
Noun Phrases), takes the last's Countability, and never ends in a bare mass
Noun Phrase after a determined one (*its power and toughness* is a determiner
over a coordinated nominal). A coordination of quantifiers determines as its last one does (*one or
more cards*) and, of fusable determinatives, heads a fused Noun Phrase
(*choose one or more*, *if X is 5 or more*). A postmodifier agrees with each
conjunct of a coordinated nominal. Bare imperatives and declaratives
coordinate in either order (a declarative before an imperative only after a
comma); a preposed adjunct scopes over the whole chain. A comparative-bounded
amount (*3 or less*) is an adjective phrase predicatively and the Amount value
of a noun that takes one (*power 3 or less*). Unlike categories coordinate in
one function: depictives, attributes, postmodifiers, predicate adjuncts (an
ordinal's position and a locative *on*-phrase: *second from the top or on the
bottom*) and *and*-lists of an Object with keywords (*base power and
toughness 4/4, flying, and haste*). Right Node Raising shares an NP, keyword
phrase, *to*-infinitival, predicative, bare infinitival, a noun's
single-preposition Complement (*the top or bottom of their library*), a
preposition's Amount (*less than or equal to X*) or, respectively, nouns'
Amounts under a slash Measure (*base power and toughness 4/4*) or under one
Amount (*power or toughness 2 or less*). A bare *power* or *toughness*, or a
coordination of them, is a Noun Phrase only when it ends in its value or
another dependent (*power equal to its toughness*, *base power and toughness
each equal to X*) or follows a graded adjective that implies the value
(*creatures with greater power*, *equal toughness*). It is never one by
premodification or coordination alone (*base power*, *power or toughness*).
Its counted plural is an ordinary bare plural (*different powers*). The Targeting
Marker's determiner takes a whole coordination of nominals, each with its own
postmodifiers or none (*up to one target artifact or creature*, *target
planeswalker that player controls or battle that player protects*).

The profiler recognizes bound stems and preposed qualities without treating
keyword verbs as keyword heads. A relative after a numeral (*two that share …*)
belongs to Stratum 3 even inside a keyword parameter.

A `KeywordPhrase` distinguishes supplied parameters from an omitted-parameter
mention. Keyword lists require supplied parameters; noun modifiers and selected
keyword complements can use mentions. Keyword lists separate their items with
commas: reminder-text removal turns Oracle's semicolon separators (used only
beside reminder text) into commas. Punctuation and symbol attachment are
derived from structure.

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
the Packed Forest's total derivation count, the Segments with their
Derivations, and the Surplus. Explicit `+`/`*` entries and
inputs with assertions also print the judgment Outcome. Outcomes use the full
judgment evaluator, including its round-trip checks and 4096-derivation limit;
`--limit` only limits the display, not assertion evaluation.

Up to eight cheapest admitted Readings are shown by default. `--limit 0` prints
only the summary. The summary also gives the cheapest tier: how many
Derivations share the least Ranking Key, and that key. Each Reading includes
its realized text, its Ranking Key (`class/association/cost`, in comparison
order) and
a constituent table: preorder node index (root 0), parent, Production id,
Category, byte span and surface in the realized text, grammatical function,
head lemma, lexeme id and Word Form, and Gap. The Word Form tells apart
Readings that differ only in a syncretic head (*cast* as present or preterite,
*Merfolk* as singular or plural). Judgment failures and unknown Categories are
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
