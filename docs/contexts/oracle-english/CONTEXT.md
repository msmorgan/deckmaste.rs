# Oracle English

The canonical linguistic vocabulary for English Oracle text, the syntax grammar
that analyzes and realizes its constituent structure, and the spelling
constructions that pair syntactic patterns with Game Model meaning. Game
concepts named in these definitions retain their
[Game Model](../game-model/CONTEXT.md) meanings.

## Authority

Oracle text is a controlled technical register (`docs/oracle-style-guide.md`),
not unrestricted literary English. The Oracle register governs which syntactic
distributions and attachment patterns are licensed; *The Cambridge Grammar of
the English Language* (Huddleston and Pullum) supplies standard grammatical
terminology and constituent-structure concepts where helpful, never a mandate to
admit open-domain literary constructions that Oracle text does not use.

## Grammar, Parsing, and Realization

**Construction**:
A Construction Grammar (CxG) pairing of a syntactic pattern with its semantic
interpretation at the spelling boundary.
_Avoid_: Construction for a purely syntactic Schema or Production

**Schema** (project term):
A declarative syntactic pattern in the grammar data file—such as a valence-frame
expansion, coordination template, or clause pattern—that compiles into one or
more Productions.
_Avoid_: Construction

**Category**:
A class of lexical or syntactic units with the same grammatical distribution.

**Constituent**:
A lexical or syntactic unit functioning as a part of a larger expression.

**Production**:
A grammar rule by which a Category is formed from an ordered pattern of other
Categories or terminals.
_Avoid_: Construction

**Derivation** (project term):
A proof that a Constituent is formed by a sequence of Productions. Two
Derivations of the same Reading are the same Reading; they are not an
Ambiguity.
_Avoid_: Reading for the proof rather than the analysis

**Reading**:
A grammatical analysis of an expression, preserving its lexical identities,
constituent structure, and grammatical distinctions.

**Reading Normal Form** (project term):
A representation of a Reading that identifies only explicitly reviewed
structural alternatives as the same analysis. Equality of sets of normal forms
provides a check that those analyses are preserved when redundant trees are
removed; it does not endorse every Reading the grammar admits.

**Ambiguity**:
The availability of more than one Reading of the same expression.

**Admission** (project term):
The relation by which the grammar accepts a Reading of a given expression at a
given Category, combining constituent derivation, feature conformance,
dependency safety, and Realization. An admitted Reading is retained regardless
of Preference.

**Packing** (project term):
Grouping Readings that every possible parent treats alike, so that the group can
be carried as one item without constructing a product of its members. Packing
preserves every distinction that can affect a parent's admissibility and keeps
correlated alternatives correlated.

**Chart** (project term):
The record of every partial and complete Derivation found for each span of an
expression during parsing.

**Packed Forest** (project term):
The shared representation of all Readings of an expression in which each
Constituent occurs once per Category, span and grammatical summary, with its
alternative Derivations grouped under it by Packing.
_Avoid_: parse tree for the whole forest

**Preference** (project term):
An optional, non-destructive ranking annotated over already-admitted Readings. A
Preference can order alternatives and name a representative; it can neither add
a Reading nor revoke one.

**Production Cost** (project term):
A nonnegative preference weight on a Production. Summed over a Reading's
Production occurrences, it is one component of the Reading's Ranking Key.
_Avoid_: Construction Cost; cost for the whole Ranking Key

**Ranking Key** (project term):
The Preference by which admitted Readings are ordered: a tuple of Attachment
Class, Right Association, Production Cost and a final tie-break, compared
lexicographically, lower first. It is purely structural.
_Avoid_: total cost, score

**Attachment Class** (project term):
A structural preference on where a kind of unselected dependent attaches,
stated by host kind and grammatical function, such as a Supplement anchoring to
its clause or a dependent after a coordination attaching to the whole
coordination.
_Avoid_: catalog preference, lexical preference

**Right Association** (project term):
The preference for attaching an unselected dependent to the nearest eligible
host, measured by the length in words of the host it attaches to.
_Avoid_: late closure, low attachment

**Realization**:
The relationship by which an abstract grammatical value is expressed in a
surface form.

**Linearization**:
The ordering of a Production's constituents into a surface sequence.

**Atom** (project term):
One annotated unit of a surface, carrying whether it is an ordinary word, an
opening or closing delimiter, a symbol, or a line boundary. The annotation is
structural metadata, never inferred from the spelling.

## Corpus, Census, and Strata

**Corpus Unit** (project term):
The smallest span of the supported corpus that the Census counts: one Type Line,
or one paragraph line of a Card Face's rules text with its reminder text
removed. A face's count is the sum over its Corpus Units.
_Avoid_: ability for the unit (one line may hold several), face for the unit

**Census** (project term):
The measurement of every Corpus Unit against the grammar. A unit is read when it
has an admitted Reading and both round-trip laws hold; known-wrong-only when it
has Readings but a Judgment shows every one of them wrong; otherwise unread.
_Avoid_: coverage for a count that includes known-wrong-only units

**Segment** (project term):
A top-level stretch of a Corpus Unit delimited by its outer structure: a
sentence of its top-level sentence sequence, an activation cost, a label before
its dash, a Keyword Line's item or parameter, a modal header, or a header. A
Segment has Derivations; a sentence sequence nested inside a Segment, as in a
quoted ability, belongs to it.
_Avoid_: sentence for a Segment that is not one; Reading for a Segment's count

**Surplus** (project term):
How far a Corpus Unit is from one Reading, summed over its Segments: Σ(nᵢ − 1),
where nᵢ is the number of Derivations of Segment i. It is 0 exactly when the
unit is unambiguous.
_Avoid_: ambiguity count for the product of the Segments' counts

**Complexity Stratum** (project term):
One of eight cumulative levels of syntactic complexity, numbered 0 to 7, that
partition the Corpus Units; each owns a set of Syntactic Families.
_Avoid_: wave, tier

**Max-Stratum Composition Rule** (project term):
A Corpus Unit belongs to the highest Complexity Stratum that owns any Syntactic
Family it contains.

**Completed Stratum** (project term):
The highest Complexity Stratum whose gate has been met: every non-deferred Corpus
Unit at or below it is read.

**Editorial Correction** (project term):
A recorded replacement for part of one face's Oracle text that breaks a
convention its printed card keeps, which the census reads instead. It cites the
printed card, or a template update the printed card predates and current Oracle
texts already carry. It never relaxes the grammar.
_Avoid_: erratum (an official Oracle change), override

**Syntactic Family** (project term):
A named syntactic pattern owned by one Complexity Stratum, such as a keyword
line, a bare relative or a trigger clause, recognized in a Corpus Unit from
its Lexical Analysis.
_Avoid_: construction family (a Construction pairs form with meaning)

**Syntactic Skeleton** (project term):
A Corpus Unit with each open-class Catalog item replaced by a placeholder naming
its Catalog, so that units of the same shape compare equal.

**Judgment** (project term):
A hand-written assertion, adjudicated against the Oracle register, that an
expression is or is not grammatical, optionally with its Constituents, heads,
grammatical functions, Gaps and endorsed Ambiguity. The judgment set is the only
tracked linguistic test evidence.
_Avoid_: golden, snapshot, tree dump

## Lexicon and Morphology

**Lexeme**:
An abstract lexical item underlying its inflected Word Forms. A lexical item
may consist of several orthographic words, as in *mana value* and *mana cost*.

**Lemma**:
The conventional citation form used to identify a Lexeme.

**Word Form**:
An inflected surface form of a Lexeme.

**Spelling Variant** (project term):
One of two or more declared Word Forms of a Lexeme for the same Feature Bundle,
each of which is retained as a distinct grammatical value.

**Word Payload** (project term):
The characters a lexical Word Form owns. A Word Payload is nonempty and contains
no quotation delimiter; a multiword lexical form owns its declared internal
spaces, while external separators and delimiters belong to the surrounding
Realization.

**Closed Word** (project term):
A core closed-class Lexeme that the grammar names individually because exactly
one Lexeme can fill the relevant slot. Its Word Forms remain part of the
lexicon.

**Lexical Class** (project term):
A declared group of Lexemes that share eligibility for a grammatical slot.
Members retain their lexical Categories; membership does not identify a Word
Form.

**Part of Speech**:
A lexical Category distinguished by grammatical distribution, such as noun,
verb, or adjective.

**Lexical Analysis**:
An identification of an expression as a Word Form of a Lexeme, together with
its lexical Category and applicable grammatical features.

**Lexical Environment** (project term):
The declared vocabulary and grammatical properties available for analyzing an
expression.

**Provenance** (project term):
The declaration or catalog a Lexical Analysis came from. Provenance is part of
a Word Form's identity and is independent of its position in the source.

**Conversion**:
The relation between two Lexemes of different Parts of Speech that share a
Lemma with no affix, as the colour adjective *red* (*becomes red*) and the noun
*red* that names the colour (*protection from red*) (CGEL, Ch. 19 §3.3,
p. 1642). Distinct from an adjective used as a fused modifier-head
(*protection from multicolored*; CGEL, Ch. 5 §9.3, p. 417).

**Feature Bundle**:
A correlated collection of grammatical feature values describing one analysis.

**Morphology**:
The formation of a Lexeme's Word Forms and the grammatical distinctions they
express.

**Affix**:
A bound morpheme that combines with a word or stem, as the prefix *non-* does.

**Clitic**:
A grammatical element that attaches to an adjacent host while taking its
syntactic position independently of that host's word structure. The English
genitive ending attaches at the edge of a phrase, and clitic auxiliaries are
unstressed versions of tensed auxiliary forms (CGEL, Ch. 18 §§6.1–6.2,
pp. 1614–1616).

**Clitic Host**:
The adjacent expression to which a Clitic attaches (CGEL, Ch. 18 §6.2,
pp. 1615–1616).

**Capitalization**:
The use of uppercase and lowercase letters in a written expression.

**Onset** (project term):
The consonantal or vocalic beginning of a realized expression, determined by
its first pronounced constituent. Lexical pronunciation, including declared
overrides, supplies this feature for *a*/*an* selection; orthography is a
fallback rather than its authority.

## Oracle Text Box, Catalogs, and Notation

**Catalog** (project term):
An extensible inventory of things the Comprehensive Rules define by
enumeration, such as card types, supertypes and subtypes
[CR#205.2a,205.3,205.4a],
keyword actions [CR#701.1], keyword abilities [CR#702.1] and ability words
[CR#207.2c], together with card names and their shortened names [CR#201.5c],
the Introduced Names card text gives objects and the Anchor Words it offers
[CR#614.12c]. Core English vocabulary, closed-class or open-class, is never a
Catalog.
_Avoid_: catalog for core English vocabulary or for any lexical data file

**Document**:
Structured rules text for one text box or independently named face portion,
including its sentences, keyword lines, and editorial boundaries.

**Type Line**:
The Document portion listing a card face's supertypes, card types and subtypes,
supertypes printed before card types [CR#205.1,205.4a], written with a long dash
before the subtypes as each card type's subtype rule prescribes [CR#302.3].
_Avoid_: Type Line as a Game Model concept — the Game Model glossary's Card Type
entry reserves it for the printed line

**Keyword Line**:
A line formed from one or more keyword-ability surfaces with their prescribed
Keyword Parameters and associated reminder text.

**Keyword Parameter** (project term):
What a Keyword Ability takes with its name on a Keyword Line: an amount (*bushido
2*), a cost after a space (*equip {2}*) or after an em dash (*ward—Pay 3 life*
[CR#702.21a]), a Keyword Quality, a card name (*partner with …*), an ability
after an em dash, or power and toughness after an em dash (*prototype {1}{B} —
1/1*); a condition or another keyword may follow an em dash as well. A keyword
may declare several alternative lists of Keyword Parameters, and a keyword with
none stands alone.

**Keyword Phrase** (project term):
A Constituent headed by a Keyword Ability, together with its Keyword Parameters
when supplied. It can express a complete keyword-ability surface or a Keyword
Mention [CR#702.1,702.1d].

**Keyword Mention** (project term):
A reference to a Keyword Ability by name, which may omit the Keyword Parameters
required on its Keyword Line, as *flashback* in *with flashback* or *flashback
cost* [CR#702.1a,702.1d]. A mention and a complete keyword-ability surface are
both Keyword Phrases; only the latter supplies an ability on a Keyword Line.

**Keyword Quality** (project term):
The Keyword Parameter a parameterized Keyword Ability takes as its quality
argument, on a keyword line or in a grant — *black* in *protection from black*
[CR#702.16a], *artifacts* in *affinity for artifacts*. It is written after the
keyword, after the preposition the keyword selects if any, or before it (*Goblin
offering*).

**Bound Keyword Surface** (project term):
The surface a Keyword Ability contributes as a suffix bound to its Keyword
Quality, so that quality and keyword are written as one word — *walk* in
*islandwalk* and in *nonbasic landwalk* [CR#702.14a].
_Avoid_: Prefix for the Keyword Quality it binds to

**Bound Keyword Stem** (project term):
The noun stem forming a Keyword Quality bound to a Bound Keyword Surface, as
*island* in *islandwalk*. It is drawn from the kind of type the keyword binds
to — a land type, a creature type, any subtype, a card type or a supertype
[CR#702.14a,702.29e].

**Keyword Variant** (project term):
A keyword ability the Comprehensive Rules define as a variant of another keyword
ability, with its own keyword name and its own Keyword Parameters, as megamorph
is a variant of morph [CR#702.37b] and typecycling of cycling [CR#702.29e]. The
*partner—[text]* abilities are variants of partner [CR#702.124i,702.124n]. A
grant of a keyword that has variants grants each appropriate variant [CR#702.1c].
_Avoid_: Variant for a casual game format such as the Commander variant

**Italic Head** (project term):
The italicized run that opens some abilities, followed by an em dash. It is
either an Ability Word or a Flavor Word, neither of which has any rules meaning,
so the Italic Head is a realization device and never changes what the ability it
heads says; a run spelled by a declared Keyword Ability or Anchor Word is that
keyword or Anchor Word, and is neither.

**Ability Word**:
One of the italicized words the Comprehensive Rules list [CR#207.2c], or a
further word printed on Vintage-legal cards that the list omits, written as an
Italic Head to tie together abilities with similar functionality. The inventory
is closed and declared, so an Ability Word is ordinary declared vocabulary, a
registered label.
_Avoid_: Keyword Ability, which has rules meaning and its own CR entry

**Flavor Word**:
An Italic Head the Comprehensive Rules do not list [CR#207.2d], tailored to the
one ability it heads. Nothing enumerates the Flavor Words, so a Flavor Word is
an open slot whose label is the italic run captured verbatim, never a declared
inventory member. A run that is a registered label, whether an Ability Word or a
Keyword Ability, is never also a Flavor Word.
_Avoid_: Flavor Text, the italicized artistic text below the rules text
[CR#207.2b]

**Anchor Word**:
A word naming one of the choices a replacement effect offers as a permanent
enters, heading the bulleted ability that the permanent has while that choice
stands; that ability is linked to the one that offers the choice
[CR#614.12c,607.2m]. It is printed in roman type, so it is no Italic Head, but
like an Ability Word it is a registered label and never a Flavor Word; it is in
scope only on the cards that print it.
_Avoid_: anchor, which is a Supplement's anchor; Flavor Word

**Shortened Name**:
A shortened spelling printed on a card in place of its full name that refers to
that same card [CR#201.5c]. A card has one only if its own text uses it, and
never one spelled like a token name or Introduced Name in scope on that card. It
is admitted only in the text of the card whose name it shortens; on a card with
two Card Faces, in the text of the face whose name it shortens
[CR#712.8d,712.8e,712.8f].
_Avoid_: nickname

**Introduced Name** (project term):
A name a card's own text gives an object that is not the name of any card: the
name of a token the card creates [CR#111.4] (*create Marit Lage, a legendary
20/20 black Avatar creature token*), or of a permanent or copy the card changes
(*She's a land named Moon*). It is in scope only in the text of the cards that
use it. A name that is also a card's name is that card's name, in scope
everywhere.
_Avoid_: token name, which also covers the predefined tokens' names [CR#111.10]

**Name Number** (project term):
The declared grammatical Number of a full Name for Agreement. It describes
agreement in the attested wording, independently of how many game objects the
Name refers to.
_Avoid_: object count

**Color Word** (project term):
One of the eight Word Forms *white*, *blue*, *black*, *red*, *green*,
*colorless*, *multicolored*, and *monocolored*. The first five name the Game
Model Colors [CR#105.1]; the last three state how many of those Colors an
object has [CR#105.2a..105.2c], and neither *multicolored* nor *colorless* is
itself a Color [CR#105.4].
_Avoid_: Color for the lexical Category

**Type Word** (project term):
A Lexeme sourced from a declared card-type, subtype, or supertype inventory for
ordinary noun or adjective use in Oracle text. Noun uses retain their Part of
Speech when functioning as Premodifiers.

**Targeting Marker** (project term):
The invariant prenominal *target* that marks an Oracle-text target description.
It is a Determinative and ends a Determiner: alone (*target creature*), after an
Adjective (*other target creatures*), or after an outer Determiner (*two target
creatures*, *each other target creature*, *up to one target creature*). Alone it
is singular; the Adjective, the outer Determiner or a Number-transparent
quantity (*any number of target creatures*) gives a plural Nominal its Number. The
Nominal it determines takes every Modifier below the Determiner (*target
creature you control*). A Noun Phrase whose Determiner ends in the Marker after
an outer Determiner, as a non-final Conjunct, coordinates only with a determined
Noun Phrase. It is distinct from the count noun *target* and the verb *target*.
_Avoid_: targeted nominal, target phrase, target description for the syntactic
unit

**Target Noun**:
The count-noun Lexeme whose Word Forms *target* and *targets* denote Game Model
Targets.

**Target Verb**:
The Verb Lexeme realized as *target*, *targets*, *targeted*, or *targeting* in
rules text stating that a Game Model Spell or Ability targets a Game Model
Object or Player.

**Notation**:
A conventional written representation using symbols, numerals, or other
specialized marks rather than ordinary word-and-phrase grammar alone.

**Symbol** (project category):
An Oracle lexical Category realized by specialized marks (such as mana, tap,
untap, and energy symbols), with a distribution distinct from ordinary words.

**Cost Component Form** (project term):
The grammatical realization of a component of a Cost [CR#118.1] as symbol
Notation or an imperative Bare Predicate.

**Mana Phrase** (project term):
A phrase realized by mana notation or coordinated alternatives of that
notation, used as a selected Complement or a governed Subject.

**Quotation**:
Cited wording set off by paired quotation marks; its internal form can be an
independent utterance or a fragment (CGEL, Ch. 20 §6, pp. 1753–1755).

**Terminal Punctuation**:
Punctuation marking the end of a sentence. When a quotation ends a matrix
sentence and both would have a full stop, only one is written (CGEL, Ch. 20
§6, p. 1755). Oracle never writes a quoted ability's own final full stop, and
writes the period or comma that follows a quotation inside its closing mark
(*has "A" and flying.*, *"A," "B," and "C."*; `docs/oracle-style-guide.md` §3,
Periods; Quotation marks and apostrophes).

**Uncleaved Text**:
A line of a cleave spell read without its square brackets, which are no
Constituent: the Bracketed text (Game Model) that a cleave spell cast for its
cleave cost removes ([CR#702.148a]; `docs/oracle-style-guide.md` §3, Bullets,
vertical bars, and square brackets) stays in place, and the brackets are
written back around it from outside any Reading.
_Avoid_: Bracketed Constituent, optional text (the brackets mark removal, not
optionality)

**Cleaved Text**:
A line of a cleave spell read with its Bracketed text removed, as the spell's
text is when its cleave cost was paid ([CR#702.148a]). It is read as English
of its own, beside the line's Uncleaved Text.

**Serial Comma**:
The comma written before the Coordinator of a flat Coordination of three or
more Conjuncts. Its presence marks the Coordination as flat rather than nested.

## Clauses, Voice, and Negation

**Sentence**:
The highest ordinary syntactic unit of Oracle text, terminated by sentence
punctuation and containing one or more Clauses.

**Clause**:
A syntactic unit organized around a predication and its dependents.

**Imperative Clause**:
A subjectless Clause whose Predicate has plain Form.

**Existential Clause**:
A Clause defined syntactically by the combination of dummy *there* and *be*
(CGEL, Ch. 16 §6.2, p. 1392).

**Existential Predicate**:
The Predicate in a Clause with a Dummy Pronoun as Subject and an internal
Complement functioning as displaced Subject (CGEL, Ch. 16 §6.1, p. 1391).

**Dummy Pronoun**:
A Pronoun without independent semantic content that fills a syntactic position;
existential *there* fills Subject position and inherits Agreement features from
the displaced Subject (CGEL, Ch. 16 §6.1, p. 1391; Ch. 4 §3.2.2, p. 242).
_Avoid_: locative Adverb for dummy *there*

**Metalinguistic Use**:
The use of an expression to cite the expression itself rather than to refer,
as a bare subtype names the word in *replacing all instances of one creature
type with Vampire* (CGEL, Ch. 5 §8.2, p. 401).

**Locative Complement**:
A Complement expressing location, source or goal, such as *on the battlefield*
after *be*; it is distinct from a predicative Complement (CGEL, Ch. 4 §5.2,
p. 257).
_Avoid_: predicative Complement for a locative

**Displaced Subject**:
An internal Complement of a verb corresponding semantically to the Subject of a
basic counterpart, while the actual Subject position is filled by dummy *there*
(CGEL, Ch. 16 §6.1, p. 1391).
_Avoid_: Object, a second Subject

**Extension**:
The element of an Existential Clause beyond *there*, *be* and the Displaced
Subject that the construction licenses; a locative or temporal extension is a
Complement of *be*, as in the non-existential counterpart (*There are seven
cards in your graveyard*: *Seven cards are in your graveyard*), not a
postmodifier inside the Displaced Subject (CGEL, Ch. 16 §6.2.1, pp. 1393–1394).
_Avoid_: Adjunct or Modifier for an existential's locative

**Subordinate Clause**:
A Clause that functions as a dependent within another Clause or phrase rather
than as an independent Sentence.

**Content Clause**:
A Subordinate Clause expressing propositional content (CGEL, Ch. 11 §1,
p. 950).

**Interrogative Content Clause**:
A Content Clause expressing a question: closed, marked by *whether* (*guesses
whether a card with that name is in your hand*), or open, with an interrogative
phrase in initial position (*choose which creatures attack*, *how each player
votes*) (CGEL, Ch. 11 §5.1, pp. 972–973).

**Relative Clause**:
A dependent Clause that modifies a nominal expression or supplements another
expression, with a relative element connected to a position in the Clause.

**Bare Relative Clause**:
A Relative Clause introduced by neither a Relative Phrase nor *that*
(*permanents [you control]*); its relativised element is never the Subject
(CGEL, Ch. 12 §2.1, p. 1034; §3.1, p. 1037).
_Avoid_: contact relative, contact clause

**Relative Phrase**:
The fronted phrase of a *wh* Relative Clause containing the relative word; it
may be larger than the relativised element (*whose controller*, *for which*)
(CGEL, Ch. 12 §3.2, p. 1039).
_Avoid_: pied-piping

**Fused Relative**:
A Noun Phrase in which a relative word combines the antecedent and the
relativised element (*whichever comes first*) (CGEL, Ch. 12 §6, p. 1068).
_Avoid_: free relative

**Postposed Relative Clause**:
A Relative Clause placed at the end of the Clause containing its antecedent,
separated from it by other material (*Choose a card at random [that was exiled
with ...]*) (CGEL, Ch. 12 §4, p. 1066).
_Avoid_: extraposed relative

**Postposed Modifier**:
A Modifier of an Object placed at the end of its Verb Phrase, separated from
the Object by other material, while still modifying the Object: a Postposed
Relative Clause, or a heavy postpositive Adjective Phrase after the verb's
selected Complements (*put a number of +1/+1 counters on it [equal to the
result]*) (CGEL, Ch. 5 §14.2, p. 445; Ch. 12 §4, p. 1066).
_Avoid_: extraposed modifier

**Verbless Clause**:
A Clause whose Predicate contains no verb; a reducible Clause Complement of a
Preposition may also omit its Subject (CGEL, Ch. 14 §10(b), p. 1267).

**Absolute Clause**:
The Complement of *with* or *without*: a non-finite or Verbless Clause that
keeps its Subject (*play with [your hand revealed]*). Its Preposition Phrase is
an Adjunct of a Clause or a Modifier in a Noun Phrase, never a selected
Complement (CGEL, Ch. 14 §8.3, p. 1262, and §10(a), p. 1267).
_Avoid_: small clause

**Gerund-participial Clause**:
A nonfinite Clause headed by a verb in its gerund-participle Word Form (CGEL,
Ch. 14 §1.1, pp. 1173–1174).

**Bare Infinitival**:
A nonfinite Clause with a plain-form verb and without the infinitival
Subordinator *to* (CGEL, Ch. 14 §1.1, pp. 1173–1174).
_Avoid_: treating the plain Word Form alone as evidence of Finiteness

**Hollow Infinitival**:
A nonfinite Clause with a non-Subject Gap linked to an Antecedent outside it
(CGEL, Ch. 14 §6.3(c), p. 1250).

**Catenative Construction**:
A Construction in which a verb selects a nonfinite internal Complement (CGEL,
Ch. 14 §1.2, pp. 1176–1178).

**Catenative Complement**:
A nonfinite Clause selected by a catenative verb, with a Complement function
distinct from Object or predicative Complement (CGEL, Ch. 14 §1.2,
pp. 1176–1177).

**Complex Catenative Construction**:
A Catenative Construction with an intervening Noun Phrase between the matrix and
subordinate verbs, functioning as Complement of one of them, usually as the
matrix verb's Object, as in *cause you to sacrifice*; a simple catenative
construction has none (CGEL, Ch. 14 §1.2, p. 1177; §3, p. 1200).

**Auxiliary Verb**:
A verb with the distinctive properties of the Negation, Inversion, Code and
Emphasis constructions: *be*, *have*, *do* and the modals (CGEL, Ch. 3 §2.1,
pp. 92–93). An auxiliary is the head of its Verb Phrase, with a nonfinite
Catenative Complement.
_Avoid_: treating the following lexical verb as the head of a modal or perfect
Clause

**Auxiliary Complement** (project term):
The Catenative Complement of an Auxiliary Verb, as *attacked* in *has attacked*
or *put* in *is put*. It bears its own function, so a Coordination of Verb
Phrases may fill it (*has attacked or blocked*).

**Purpose Adjunct**:
An infinitival Clause that functions as an Adjunct of purpose, as in *spent to
cast it* (CGEL, Ch. 8 §1, p. 669).

**Voice**:
The grammatical organization of a predication's participants, including the
active and passive patterns of Subject and Complement realization.

**Bare Passive**:
A nonfinite passive Clause without an expanding catenative verb such as *be* or
*get* (CGEL, Ch. 16 §10.1.1, p. 1430).

**Adjectival Passive**:
An Adjective Phrase related to a verb's past participle and functioning as a
predicative Complement in a complex-intransitive Clause (CGEL, Ch. 16 §10.1.3,
pp. 1436–1439).
_Avoid_: Bare Passive for an adjectival Complement; adjectival passive Clause

**Internalised Complement**:
The Complement within a passive Verb Phrase that corresponds to the Subject of
its active counterpart, normally marked by *by* (CGEL, Ch. 16 §10.1.1,
p. 1428).
_Avoid_: Agent for this syntactic function

**Irrealis**:
The non-tensed Inflectional Form of *be* (*were*) used with first- or
third-person singular Subjects in modal-remoteness constructions (CGEL, Ch. 3
§1.7, pp. 86–88).
_Avoid_: past subjunctive; treating irrealis as a Past Tense

**Irrealis Clause**:
A finite Clause whose Predicate contains an Irrealis verb.

**Polarity**:
The grammatical distinction between positive and negative expressions.

**Primary Negation**:
Verbal negation in a primary inflectional form: analytic negation adds *not*
after the auxiliary verb; synthetic negation uses its negative inflection
(CGEL, Ch. 9 §2.1, p. 799).

**Secondary Negation**:
Negation within a nonfinite Clause, with *not* premodifying its Verb Phrase
(CGEL, Ch. 9 §2.3.1, p. 803).

**Negator**:
A word whose function is to mark negation.

**Supplement**:
Material set apart from a related expression, its **anchor**, without
integration into the anchor's headed syntax (CGEL, Ch. 15 §5.1,
pp. 1350–1351).

**Parenthetical**:
Supplementary material set off from the surrounding expression, commonly by
parentheses.

## Nominals, Determiners, and Phrases

**Noun Phrase**:
A phrase whose head or fused head is nominal and which can fill functions such
as Subject, Object, or Complement of a preposition.

**Nominal**:
An intermediate nominal constituent headed by a noun or noun-like element,
below the level of Determiner selection.

**Bare Noun Phrase**:
A Noun Phrase formed from a Nominal without a Determiner (CGEL, Ch. 5 §4,
p. 355).

**Fixed Frame**:
A Preposition with which a singular count noun stands bare, though elsewhere
the noun takes a Determiner (*to hand*; *from outside the game*). The nouns a
frame admits are few and listed (CGEL, Ch. 5 §8.5(b), pp. 409–410).

**Compound Noun**:
A noun Lexeme formed from two or more bases, as in *charge counter*, whose
constituents form a lexical unit (CGEL, Ch. 5 §14.4, pp. 448–451).

**Determinative**:
A lexical Category containing words such as *the*, *this*, *each*, and *any*.
It is distinct from the Determiner function.

**Indefinite Article**:
The Determinative *a(n)*, which normally selects a singular count head;
with a modifier it also occurs before quantified plurals (*an additional
two creatures*) (CGEL, Ch. 5 §3.4, p. 353).

**Distributive Determinative**:
A Determinative selecting individual members or groups, such as *each* or
*every*. A quantified group selected by a singular distributive has singular
external Agreement (*every two turns is ...*) (CGEL, Ch. 5 §3.4, p. 354).

**Deictic Determinative**:
A Determinative locating its referent relative to the point of reference, such
as *this*, *that*, *last* or *next* (*this turn*, *next turn*) (CGEL, Ch. 8
§6.3, p. 698).

**Determinative Phrase**:
A phrase headed by a Determinative, including cardinal determinatives.

**Determiner**:
The grammatical function that marks a Noun Phrase as definite, quantified, or
otherwise determined. A Determinative commonly realizes this function.

**Predeterminer**:
An external Modifier preceding a determined Noun Phrase, such as universal
quantifiers (*all*, *both*), multipliers (*twice*, *double*), and fractions
(*half*) (CGEL, Ch. 5 §12, pp. 433–434).

**Multiplier**:
A multiplicative expression functioning as a Predeterminer of a definite Noun
Phrase, such as *twice*, *double*, or *triple* (CGEL, Ch. 5 §12(c), p. 434).

**Definiteness**:
The distinction between definite and nondefinite Noun Phrases.

**Countability**:
The distinction between count and mass uses of a noun.

**Number Transparency**:
The property of a quantificational noun use in which the Number of its Oblique
controls the whole Noun Phrase's Agreement, overriding the head noun's Number
(CGEL, Ch. 5 §3.3, pp. 349–352; §18.2, pp. 501–504).

**Oblique**:
The Noun Phrase Complement of *of* in a quantificational or partitive noun
Construction; the quantificational noun remains the syntactic head and *of*
forms a Constituent with the Oblique (CGEL, Ch. 5 §3.3, pp. 349–352).
_Avoid_: Head for the Oblique

**Fused Determiner-Head**:
An expression combining the Determiner and Head functions in a Noun Phrase
without an overt nominal head (CGEL, Ch. 5 §§9.1–9.2, pp. 410–415).
_Avoid_: Pronoun for a Determinative in fused-head use

**Fused Subject-Determiner-Head**:
A genitive that is a whole Noun Phrase, its head understood from an Antecedent
or the context, as in *power less than Mirko's* (CGEL, Ch. 5 §16.3,
pp. 467–468). A nominal immediately after the genitive is its head, not an
Antecedent (CGEL, Ch. 17 §6.1, pp. 1516–1517).
_Avoid_: Independent Genitive for a noun's genitive

**Partitive Noun Phrase**:
A fused-head Noun Phrase containing an *of* phrase and denoting a subset of the
set denoted by that phrase's Complement, as in *one of them* (CGEL, Ch. 5,
p. 333).
_Avoid_: an omitted noun as an invented lexical head

**Quantitative Determiner**:
A Determiner that expresses a count or amount, realized by a Cardinal Numeral,
a quantitative Preposition Phrase (*up to two*), a comparative Determinative
Phrase (*more than one*), a numeral Coordination (*one or more*), or a
degree-modified Determinative Phrase (*that many*) (CGEL, Ch. 5 §4,
pp. 357–358; §11, pp. 431–432).

**Quantified Noun Phrase**:
A Noun Phrase with a Quantitative Determiner and a Nominal head.

**Cardinal Numeral**:
A numeral expressing a count, such as *one* or *two*.

**Cardinal Modifier**:
A dependent of a Cardinal Numeral: an Adverb expressing approximation or
precision (*exactly two*) or a Preposition Phrase marking a bound (*at least
two*) (CGEL, Ch. 5 §11(b,e), pp. 431–432).

**Degree Modifier**:
A Modifier expressing degree, including the Determinative *that* modifying
*many* or *much* within a Determinative Phrase (CGEL, Ch. 5 §11(c), p. 431).

**Superlative Quantifier**:
A quantifier comparing members of a set by quantity rather than expressing a
proportion, as in *the most cards* (CGEL, Ch. 13 §6.3.2, p. 1166).

**Genitive**:
A grammatical form expressing possession or a related dependency, including
possessive determinatives and phrases marked with an apostrophe ending.

**Independent Genitive**:
A personal Pronoun's Genitive form used in head function, either as a fused
Determiner-Head or as a pure Head (CGEL, Ch. 5 §9.1, p. 410; §16.4,
pp. 470–471).
_Avoid_: Possessive Absolute Pronoun

**Bare Genitive**:
A genitive marked in writing by an apostrophe without an added *s*, obligatory
for plural nouns ending in *s*, as in *owners'* (CGEL, Ch. 18 §4.2, p. 1595).

**Adjective**:
A lexical Category whose members characteristically modify Nominals or serve as
predicative Complements (CGEL, Ch. 6 §2.4.3, pp. 540–542).

**Attributive Adjective**:
An Adjective functioning as a Premodifier in a Nominal. An attributive-only
adjective lacks an established predicative use; this restriction is independent
of whether the Noun Phrase has an overt Determiner (CGEL, Ch. 6 §4.1,
pp. 553–556).

**Predicative Adjective**:
An Adjective functioning as a predicative Complement or Adjunct, attributing a
property to its Predicand (CGEL, Ch. 6 §4.1, pp. 553–554).

**Adjective Phrase**:
A phrase headed by an Adjective.

**Preposition**:
A lexical Category whose members head phrases expressing relations and
characteristically select Complements (CGEL, Ch. 7, pp. 597–661).

**Compound Preposition**:
A Preposition whose components have coalesced into a single Lexeme, such as
*instead*, *because*, or conditional *as long as* (CGEL, Ch. 7 §3.1,
pp. 622–623; Ch. 13 §4.5(d), p. 1134).
_Avoid_: treating every adjacent sequence of Prepositions as one Lexeme

**Preposition Phrase**:
A phrase headed by a Preposition, with any selected Complements.

**Intransitive Preposition**:
A Preposition used without a Complement, constituting a complete Preposition
Phrase without changing its lexical Category into Adverb (CGEL, Ch. 7 §2.4,
pp. 612–613).
_Avoid_: adverb for a complementless Preposition

**Adverb**:
A lexical Category whose members characteristically modify a phrase, Clause, or
other expression without serving as a nominal argument.

**Adverb Phrase**:
A phrase headed by an Adverb.

**Subordinator**:
A grammatical marker, such as content-clause or relative *that*, *whether*, or
interrogative *if*, that marks a Clause as subordinate. A Preposition taking a
Clause Complement remains a Preposition.

**Verb Phrase**:
A phrase headed by a verb.

**Lexical Verb Phrase**:
An instantiated lexical verb together with the complements it selects, before
higher auxiliary and clause structure is added.

**Bare Predicate**:
A Predicate headed by a plain-form verb, occurring in a finite imperative or as
a nonfinite Complement.

**Secondary Verb Phrase**:
A Verb Phrase whose head has a secondary Word Form: plain, gerund-participle,
or past participle.

## Grammatical Relations, Complements, and Modifiers

**Predicate**:
The grammatical function in a Clause that predicates something of the Subject.
This is distinct from a Game Model Predicate.

**Subject**:
A grammatical relation within a Clause, not a constituent Category or a pronoun
case.

**Object**:
A grammatical relation licensed by a head, not a constituent Category. This
term may coexist with the unrelated Game Model Object.

**Postposed Object**:
An Object realized to the right of its basic position, commonly after a
Preposition Phrase, while retaining Object function (CGEL, Ch. 16 §4,
pp. 1382–1384).

**Complement**:
A dependent licensed (subcategorized) by its head (CGEL, Ch. 4 §1.2,
pp. 219–222).

**Obligatory Complement**:
A Complement its head cannot omit in a given construction. A noun's Complement
is optional in an ordinary Noun Phrase (*the excess*), but obligatory when the
noun stands bare in a fossilised Preposition Phrase, unless that idiom allows
its omission (*until end of turn*, *in excess of*, but *on top*, *in front*;
CGEL, Ch. 7 §3.1, pp. 619–620).
_Avoid_: required argument

**Subcategorization Frame**:
An ordered lexical schema describing the Complements and fixed markers a head
licenses. Verbs, prepositions, nouns, adjectives and determinatives may each
declare Subcategorization Frames.
_Avoid_: Verb Phrase for the schema; frame for an instantiated phrase

**Verb Frame**:
A Subcategorization Frame of a verb.
_Avoid_: Verb Phrase for the schema; Verb Frame for an instantiated phrase

**Frame Family** (project term):
The kind of a Subcategorization Frame, named by the shape of its Frame Slots,
such as intransitive, monotransitive, ditransitive or copular.

**Verb Idiom**:
A Verb Frame whose Complements include a fixed Preposition, as phrasal-copular
*come up* with a predicative Complement (*comes up heads*) (CGEL, Ch. 4 §6.3,
p. 288).

**Selected Frame** (project term):
The one Subcategorization Frame of its head that a Constituent realizes.

**Host Category** (project term):
A Category that an unselected Preposition Phrase may modify in Oracle English,
declared for each pairing of a preposition with one of its Complement frames:
a Nominal (as Postmodifier), a Predicate (as Adjunct), both, or neither, in
which case the phrase occurs only as a Complement. A phrase may instead be only
a Supplement to a Clause or line (*, where X is its power*), or modify a
Predicate only as a Conjunct of an Unlike Coordination of Adjuncts (*second
from the top or on the bottom*).

**Valence Alternation** (project term):
A systematic relation between the Complement realizations of one
Subcategorization Frame, such as active and passive Voice, Extraction of a
Complement to leave a Gap, or a Shared Complement under Right Node Raising.
Valence Alternations follow from a head's active frames; they are never
declared per lexeme. An Active-only Frame has no passive Valence Alternation.

**Active-only Frame** (project term):
A Verb Frame restricted to active voice: its Object cannot be externalised by
passivisation, as with *cost* and *have* (*A packet of cigarettes costs around
seven dollars*, *Jill has the best qualifications*) (CGEL, Ch. 16 §10.1.2,
p. 1432). The restriction is a lexical property of the verb in that sense, not
a sign that its post-verbal Noun Phrase is no Object.
_Avoid_: intransitive for such a verb

**Frame Slot** (project term):
One position in a lexical subcategorization frame: the grammatical relation the
position bears paired with the Category and markers that fill it.
_Avoid_: Complement for the slot descriptor

**Locative Complement**:
A Complement expressing location, source, or goal, distinct from a predicative
Complement and from an optional locative Adjunct.

**Source Complement**:
A Locative Complement expressing the starting location (CGEL, Ch. 8 §4.3,
p. 687).

**Manner Complement**:
A dependent expressing manner that is selected by its head (CGEL, Ch. 8 §2.1,
p. 671).

**Resultative Complement**:
A predicative Complement denoting the state of its predicand at the end of a
process, licensed by the verb (CGEL, Ch. 4 §§5, 5.3, pp. 251–252, 261–263).

**Predicative Complement of a Preposition**:
A Complement that ascribes a property or role to a Predicand, as the Noun
Phrase in *as a copy* does (CGEL, Ch. 7 §5.1, pp. 636–637).

**Predicand**:
The expression of which a property or role is predicated by a predicative
Complement (CGEL, Ch. 7 §5.1, p. 637).

**Complex-Transitive Clause**:
A Clause whose verb takes both an Object and an object-oriented predicative
Complement, as in *turn it face up*; with only a predicative Complement it is
complex-intransitive. The Object is the Predicand of the predicative Complement
(CGEL, Ch. 4 §1.1, p. 217).

**Measure Phrase**:
A phrase expressing an extent or scalar value, including arithmetic
combinations and comparisons with other values.

**Measure Complement**:
A Complement that supplies a quantity or extent licensed by its head.
_Avoid_: Numerative

**Measure Noun Phrase** (project term):
A bare slash measure (*+2/+2*, *5/1*) as a Noun Phrase, occurring only where
a head selects one: the Object of *get* (*gets +2/+2*) or the Complement of
*of* (*your choice of 5/1 or 1/5*). It is never a general Object, a Manner
Adjunct or a bare singular Noun Phrase elsewhere. Under a determiner a slash
measure is an ordinary noun (*an additional +2/+2*); before a noun it states
a size (*a 3/3 creature*); after *be* or *become* it is a predicative
Complement (*it's 4/3*).

**Amount Complement** (project term):
A Complement realized by a numeral or variable stating a number, as *2* in
*scry 2* or *X* in *scry X*. It selects no Object, so it has no passive Valence
Alternation. The Game Model's Amount is the semantic expression such a
Complement may denote; the Amount Complement is its syntactic realization.
_Avoid_: Amount for the Complement itself; Object for a numeral

**Comparative-bounded Amount** (project term):
A number coordinated by *or* with an intransitive comparative Adjective whose
understood standard is that number (*3 or less*, *X or greater*). It is an
Adjective Phrase as a predicative Complement (*X is 2 or less*) and the Amount
value of a noun that takes one (*power 3 or less*), never a count or measure on
its own.
_Avoid_: Scalar Amount (the Scalar terms here name scales and positions on
them); a Cardinal Modifier's bound (*at least two*), which modifies a numeral

**Cost Complement** (project term):
A Complement realized by a Cost [CR#118.1], as *{3}* in *waterbend {3}* or *{2}*
in *equip {2}*. Its realization is composed of Cost Component Forms. It selects
no Object, so it has no passive Valence Alternation.
_Avoid_: Object for a Cost

**Scalar Change Extent Complement**:
A Complement expressing how far a value changes along a scale, licensed by an
appropriate lexical head; overall extent can be marked by *by* (CGEL, Ch. 8
§§5.2–5.3, pp. 691–693).

**Scalar Location**:
The expression of a position on a scale, commonly with *be* and an NP as
predicative Complement (CGEL, Ch. 8 §5.4, p. 693).

**Comparative Governor**:
An item that licenses a comparative Complement, such as *equal*, *less*,
*greater*, or *other* (CGEL, Ch. 13 §1.3, p. 1104).

**Comparative Complement**:
The expression of the secondary term in a comparison; the bare Complement
excludes its governing Preposition while the expanded Complement includes it
(CGEL, Ch. 13 §1.3, pp. 1103–1104).

**Expanded Comparative Complement**:
A comparative Complement introduced by its own marker, such as *than …* or the
second *as* in a scalar equality comparison (CGEL, Ch. 13 §1.3, p. 1104).
_Avoid_: treating a marker selected by a different comparative construction as
interchangeable

**Comparative Phrase**:
A phrase containing a Comparative Governor (CGEL, Ch. 13 §1.3, p. 1105).

**Scalar Equality**:
A comparison in which *as* governs a phrase and an expanded Complement
introduced by a second *as* (CGEL, Ch. 13 §1.3, p. 1104).
_Avoid_: treating a reanalysed conditional Compound Preposition as a duration
comparison

**Adjunct**:
An optional dependent that modifies a Clause or Verb Phrase without being
selected as a Complement by its head.

**Adjunct Position**:
Where an Adjunct stands in its Clause: front, before the Subject; central,
between the Subject and a lexical verb, or after the first auxiliary; end, after
the verb and perhaps some or all of its dependents (CGEL, Ch. 8 §20.1,
pp. 779–780; Ch. 6 §7.1, p. 575). An Adverb that modifies an adjective, adverb,
determinative or numeral inside its phrase, rather than a Clause, has a
phrasal position instead, outside this three-way contrast (CGEL, Ch. 6 §5.1,
pp. 562–563). A connective that marks a Linked Coordination has a linking
position, initial in its Linked Conjunct (CGEL, Ch. 15 §2.10, pp. 1319–1320).
In a bare imperative, which has no Subject, front and central position
coincide.

**Connective Adjunct**:
An Adjunct relating its Clause to surrounding text or context, such as
sentence-initial *Then* or *Otherwise* (CGEL, Ch. 8 §19, pp. 775, 778).

**Depictive Adjunct**:
An optional predicative Adjunct describing a participant during the situation,
as in *enters tapped and attacking* (CGEL, Ch. 4 §5.3, pp. 262–263; Ch. 14
§9, p. 1265).

**Position Phrase** (project term):
An ordinal with a *from* Preposition Phrase naming the end of an ordered zone
it counts from (*second from the top*), functioning as a Depictive or
predicate Adjunct.

**Predicative Adjunct**:
An optional predicative dependent describing a pragmatically appropriate
predicand (CGEL, Ch. 4 §5.3, p. 263).

**Quantificational Adjunct**:
A fused-head partitive functioning in Clause structure to quantify the Subject,
such as *each*, *all*, or *both* (CGEL, Ch. 5 §9.2, p. 413).

**Purpose Infinitival**:
A *to*-infinitival Clause functioning as a purpose Adjunct (CGEL, Ch. 8 §12.2,
pp. 728–729).
_Avoid_: Catenative Complement for an unselected purpose Clause

**Means Adjunct**:
An Adjunct expressing how a situation is brought about, commonly a *by*
Preposition Phrase with a nominal or gerund-participial Complement (CGEL,
Ch. 8 §2.2, pp. 673–675).

**Frequency Adjunct**:
An Adjunct that quantifies occurrences of a situation, such as *X times* or
*twice* (CGEL, Ch. 8 §9, pp. 713–716).

**Temporal Noun Phrase Adjunct**:
A Noun Phrase functioning as an Adjunct of temporal location or frequency. A
period noun (*turn*, *game*, *combat*, *step*, *phase*) forms one only with a
Deictic or Distributive Determinative (*this turn*, *each combat*); with an
article or genitive it is an ordinary Noun Phrase (*End the turn*). Serial
*time* forms one with any Determinative (*any time*, *an additional time*,
*three times*), and either kind does with a Relative Clause (*the next time you
cast a spell*) (CGEL, Ch. 8 §6.3, p. 698; §9, pp. 713–716).

**Duration Phrase**:
An Adjunct that bounds how long the effect of its Clause lasts, such as *until
end of turn*.

**Duration Adverb**:
An Adverb expressing temporal extent, as with *long* in *for long* (CGEL, Ch. 6
§5.2, p. 569; Ch. 7 §5.1(d), p. 640).
_Avoid_: classifying temporal *long* as an Adjective solely from its spelling

**Modifier**:
A dependent that attributes or restricts the interpretation of its head without
being selected as a Complement.

**Premodifier**:
A Modifier that precedes its head in the linear order. A noun (*creature* in
*creature card*) or verbal participle (*attacking* in *attacking creature*,
*chosen* in *the chosen type*) used as a Premodifier retains its lexical
Category (CGEL, Ch. 5 §14.2, p. 444; Ch. 6 §2.4.3, pp. 541–542).

**Postmodifier**:
A Modifier that follows its head in the linear order, including Preposition
Phrases, Adjective Phrases, Relative Clauses, and subjectless nonfinite Clauses
(*creature attacking you*, *cards exiled this way*) (CGEL, Ch. 14 §9,
pp. 1264–1266).

**Focus**:
The constituent whose interpretation is made prominent or restricted relative
to alternatives.

**Focusing Modifier**:
A Modifier that associates with a scopal Focus, such as restrictive *only* or
additive *also* (CGEL, Ch. 6 §7.3.1, p. 587).

## Coordination, Extraction, and Anaphora

**Coordination**:
A construction joining two or more coordinate units (CGEL, Ch. 15,
pp. 1273–1362).

**Correlative Coordination**:
A Coordination with a marker associated with its first Conjunct and a paired
Coordinator, as in *both … and*, *either … or*, or *neither … nor*.

**Conjunct**:
One of the units joined in a Coordination.

**Coordinator**:
The marker that links Conjuncts, such as *and* or *or*.

**Right Node Raising**:
A sharing Schema in which material following a Coordination fills a
corresponding unpronounced position in each Conjunct.

**Shared Complement**:
A Complement realized once after coordinated heads that each independently
license its grammatical function.

**Unlike Coordination**:
A Coordination whose Conjuncts differ in Category but share a function, as an
Adjective Phrase and a Preposition Phrase in predicative function (CGEL,
Ch. 15 §3.2, pp. 1326–1329). Its Conjuncts need only not all be of one
Category: a serial one may begin and end alike (*indestructible, "...," and
equip {0}*). In Oracle English it joins depictives,
attributes, postmodifiers, predicate Adjuncts (*second from the top or on the
bottom*) and an Object with keyword phrases (*base power and toughness 4/4,
flying, and haste*), or keyword phrases with a slash measure naming counter
kinds (*from among flying, first strike, ..., and +1/+1*; [CR#122.1a,122.1b]).
_Avoid_: mixed coordination

**Word-Part Coordination**:
A Coordination whose non-final Conjuncts are first bases of Compound Nouns
alone, sharing the head word of the last Conjunct, a whole Compound Noun
(CGEL, Ch. 15 §3.1(g), p. 1325: *Sydney- or Melbourne-based companies*), as in
*a +1/+1, first strike, or vigilance counter*. A counter's first base is its
name: a keyword or a slash measure ([CR#122.1a,122.1b]). The last Compound
Noun stays one Lexeme.
_Avoid_: coordinated premodifiers of *counter* (that splits the Compound Noun)

**Linked Coordination** (project term):
A Coordination without a Coordinator, whose non-initial Conjunct is marked
instead by a connective adverb initial in it, its Adjunct: *Draw a card,
[then discard a card]*. The connective is the sole linking item of CGEL,
Ch. 15 §2.10, pp. 1319–1320; after a Coordinator it is an Adjunct only (*and
then discard a card*). It links main clauses and Verb Phrases, never
subordinate clauses.
_Avoid_: treating *then* as a Coordinator

**Linked Conjunct** (project term):
The Conjunct of a Linked Coordination that its connective marks.

**Delimited Polysyndetic Coordination** (project term):
A polysyndetic Coordination, with a Coordinator before every non-initial
Conjunct (CGEL, Ch. 15 §1.1, p. 1276), whose Conjuncts are each set off by a
comma (*A, or B, or C*).

**Complement Cluster** (project term):
A sequence of a verb's Complements, optionally ending in an Adjunct,
coordinated with parallel sequences after the shared verb, as in *put one of
them into your hand and the rest into your graveyard* or *has lifelink if you
control a Cleric and deathtouch if you control a Rogue*: right
nonce-constituent coordination (CGEL, Ch. 15 §4.3, pp. 1341–1343). The
sequences are parallel in function: each holds the same functions in the same
order. A single Complement is a Cluster only with its Adjunct. Each dependent
keeps its own function within its Conjunct. A Cluster after the first may
open with a remnant, a Complement whose head is elided, recovered from the
first Cluster (*gain protection from white if ..., from blue if ...*; CGEL,
Ch. 15 §4.3, p. 1343, note 65).
_Avoid_: gapping, which elides the verb from a second Clause

**Adjunct Cluster** (project term):
A sequence of two Preposition Phrase Adjuncts after a verb with no
Complement, coordinated with parallel sequences, as in *enters with a +1/+1
counter on it for each other red creature you control and a +1/+1 counter on
it for each other green creature you control*. Each Cluster has the first
Cluster's prepositions; a Cluster after the first may open with a remnant,
the Complement of a Preposition Phrase whose preposition is elided (CGEL,
Ch. 15 §4.3, p. 1343, note 65: *We'll be in Paris for a week and Bonn for
three days*).
_Avoid_: Complement Cluster, whose sequence begins with Complements

**Idiomatic Coordinate**:
A pro-form ending a Coordination after its Coordinator, standing for further
Conjuncts of the same kind: *and so on* (CGEL, Ch. 17 §7.7.3(e), p. 1540). It
is no Conjunct and takes no part in the Conjuncts' parallelism (*+1/+1 if it
has flying, +1/+1 if it has first strike, and so on for double strike, ...*).

**Medial Remnant** (project term):
A medial Conjunct with an element at its beginning elided and recovered from
the initial Conjunct, in a Coordination at the level of its final Conjunct
(CGEL, Ch. 15 §3.6, pp. 1335–1336, [31]): a Predicative Complement whose
copula is elided (*is enchanted, equipped, and has a counter on it*), or a
Verb Phrase whose Subject is elided (*you gain 4 life, draw four cards, and he
deals 4 damage to each opponent*). The initial and last Conjuncts are never
remnants.
_Avoid_: gapping, which elides the verb between a Subject and its dependents

**End-attachment Coordination**:
A Coordination whose second Conjunct is attached at the end of a Clause
rather than beside the first (CGEL, Ch. 15 §4.5, pp. 1345–1348). In its
qualifying use, *but* or *and* adds to a whole main Clause a fragment of
Adjuncts focused by *only* or *not*: *Any player may activate this ability
but only as a sorcery*.

**Extraction**:
A grammatical dependency between a displaced expression and its Gap, subject to
constraints on the intervening constituent boundaries.

**Gap**:
An unpronounced syntactic position related to an overt or understood element
elsewhere in the structure.

**Ellipsis**:
The omission of material whose grammatical content is recoverable from an
Antecedent or context.

**Stranded Auxiliary**:
An auxiliary verb before the ellipsis site of its Complement, heading its
Verb Phrase alone (*If you can't, draw a card*; CGEL, Ch. 17 §7.1, p. 1519).
A phrase right after the site would be that Complement.

**Pro-form**:
An expression whose content is supplied anaphorically; *do so* is a complex
pro-form headed by *do* (CGEL, Ch. 17 §7.5, p. 1529).

**Anaphor**:
An expression whose interpretation depends on an Antecedent in its discourse or
syntactic context.

**Antecedent**:
The expression or discourse introduction on which an Anaphor depends.

**Clause-mate** (project term):
Of an Anaphor, another argument of the same verb: its Subject, Object, or a
Complement. A Genitive's possessor ("its power") and the noun phrase of a small
clause ("counters on it") are not clause-mates of the verb (`docs/decisions/semantics-v3.md`
§1.5).
_Avoid_: co-argument for possessors or small-clause objects

**Referent**:
A Game Model Object, Player, value, or group that an expression denotes in
context.

**Quantity**:
The Game Model count constraint a Determiner states over a Selection, such as
*one*, *up to two*, *one or more*, or *any number*. Determiners are this
context's side of it; the constraint itself belongs to the
[Game Model](../game-model/CONTEXT.md).

## Agreement and Inflection

**Agreement**:
The grammatical relationship in which one expression's features constrain the
form of another. Agreement is not itself Finiteness or an Inflectional Form.

**Concord Class** (project term):
A derived morphological equivalence class used to choose a finite agreeing
form, initially third-person singular versus other.

**Case**:
A grammatical distinction between forms of a nominal expression, such as
nominative, accusative, and genitive, associated with its syntactic
distribution.

**Person**:
The grammatical feature distinguishing speaker, addressee, and other referents.
Oracle English has second and third person only: first person is unattested
in Oracle text and is excluded.

**Number**:
The grammatical feature distinguishing singular and plural values where English
grammar requires the distinction.

**Conjunct Number** (project term):
The Number of each Conjunct of a Coordination of Nominals, as distinct from
the Number of the Coordination itself. A Determiner outside the Coordination
selects it (*each creature and planeswalker*: each Conjunct singular, the
Nominal plural), and a Postmodifier agrees with it (CGEL, Ch. 15 §2.1, p. 1291;
Ch. 5 §18.4, p. 508).

**Proximity Agreement**:
Agreement of a Coordination with its nearest Conjunct instead of a value
resolved from all of them (CGEL, Ch. 5 §18.4, pp. 508–509). An *or*-Coordination
of Noun Phrases takes its last Conjunct's Number and Person, and a
quantifier Coordination determines as its last Conjunct does (*one or more
cards*). Existential *be* before an *and*-Coordination of Noun Phrases may
agree with its first Conjunct, the nearest, when that is singular (*If there
is an instant card and a sorcery card in your graveyard*; CGEL, Ch. 4 §3.2.2,
p. 242, n. 21), as well as with the plural Coordination.
_Avoid_: attraction for this resolution rule

**Feature Resolution** (project term):
The value a Coordination takes from all of its Conjuncts' values: their common
value, or a fixed one when they differ. A Coordination of Noun Phrases is
determined only when every Conjunct is, so *all* over *artifacts and
enchantments* determines a coordinated Nominal, never a Coordination of bare
Noun Phrases; every Conjunct but the last shares its Determination.

**Finiteness**:
The grammatical property distinguishing finite clauses and Verb Phrases from
nonfinite ones. It is not an Inflectional Form.

**Tense**:
The grammatical marking of temporal location relative to a reference point. It
is distinct from Finiteness and from the Word Form that realizes it.

**Grade**:
The inflectional distinction among plain, comparative and superlative forms,
which many Adjectives and a few Adverbs have and verbs and nouns lack (CGEL,
Ch. 6 §2.2, p. 533). Comparative and superlative forms are declared as
Adjectives of their own, each bearing its Grade. Analytic comparatives and
superlatives are formed with the degree adverbs *more* and *most*, and *inner*
and *outer* are derived Adjectives, not Grades of a preposition (CGEL, Ch. 6
§2.2, pp. 533–534).

**Inflectional Form**:
A morphological form of a Verb Lexeme, such as plain, third-person-singular
present, preterite, gerund-participle, past participle, or irrealis. It is
distinct from Finiteness and from Agreement, though those dimensions constrain
its selection.
_Avoid_: Verb Form when the specific morphological dimension is meant

## Status, Designation (Game Model terms — not grammatical categories)

`tapped`, `goaded`, `suspected`, `saddled` are the **Participles** of their
keyword-action verbs, used attributively or predicatively; `monstrous` is an
**Adjective**; `the monarch`, `the initiative` are **Noun Phrases**; `face down`
is a lexicalized **Preposition Phrase**. Whether such a form names a
status [CR#110.5] or a designation [CR#701.15b] is Game Model semantics resolved through the verb
declaration, never a grammatical class. _Avoid_ "Status", "Designation" as the
name of a vocabulary, codec, schema, or production in the grammar.
