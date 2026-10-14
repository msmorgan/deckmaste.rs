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
A nonnegative preference weight on a Production. A Reading's total sums the
costs of its Production occurrences; lower totals rank earlier without changing
grammatical admission or erasing alternative Readings.
_Avoid_: Construction Cost

**Realization**:
The relationship by which an abstract grammatical value is expressed in a
surface form.

**Linearization**:
The ordering of a Production's constituents into a surface sequence.

**Atom** (project term):
One annotated unit of a surface, carrying whether it is an ordinary word, an
opening or closing delimiter, a symbol, or a line boundary. The annotation is
structural metadata, never inferred from the spelling.

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
[CR#207.2c], together with card names and their shortened names [CR#201.5c].
Core English vocabulary, closed-class or open-class, is never a Catalog.
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
heads says; a run spelled by a declared Keyword Ability is that keyword, and is
neither.

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

**Shortened Name**:
A shortened spelling printed on a card in place of its full name that refers to
that same card [CR#201.5c]. It is derived from the full name and admitted only
in the text of the card whose name it shortens.
_Avoid_: nickname

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
It is distinct from the count noun *target* and the verb *target*.

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
sentence and both would have a full stop, the matrix full stop is suppressed
(CGEL, Ch. 20 §6, p. 1755).

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

**Displaced Subject**:
An internal Complement of a verb corresponding semantically to the Subject of a
basic counterpart, while the actual Subject position is filled by dummy *there*
(CGEL, Ch. 16 §6.1, p. 1391).
_Avoid_: Object, a second Subject

**Subordinate Clause**:
A Clause that functions as a dependent within another Clause or phrase rather
than as an independent Sentence.

**Content Clause**:
A Subordinate Clause expressing propositional content (CGEL, Ch. 11 §1,
p. 950).

**Relative Clause**:
A dependent Clause that modifies a nominal expression or supplements another
expression, with a relative element connected to a position in the Clause.

**Verbless Clause**:
A Clause whose Predicate contains no verb; a reducible Clause Complement of a
Preposition may also omit its Subject (CGEL, Ch. 14 §10(b), p. 1267).

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

**Compound Noun**:
A noun Lexeme formed from two or more bases, as in *charge counter*, whose
constituents form a lexical unit (CGEL, Ch. 5 §14.4, pp. 448–451).

**Determinative**:
A lexical Category containing words such as *the*, *this*, *each*, and *any*.
It is distinct from the Determiner function.

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

**Selected Frame** (project term):
The one Subcategorization Frame of its head that a Constituent realizes.

**Host Category** (project term):
A Category that an unselected Preposition Phrase may modify in Oracle English,
declared for each pairing of a preposition with one of its Complement frames:
a Nominal (as Postmodifier), a Predicate (as Adjunct), both, or neither, in
which case the phrase occurs only as a Complement.

**Valence Alternation** (project term):
A systematic relation between the Complement realizations of one
Subcategorization Frame, such as active and passive Voice, Extraction of a
Complement to leave a Gap, or a Shared Complement under Right Node Raising.
Valence Alternations follow from a head's active frames; they are never
declared per lexeme.

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

**Amount Complement** (project term):
A Complement realized by a numeral or variable stating a number, as *2* in
*scry 2* or *X* in *scry X*. It selects no Object, so it has no passive Valence
Alternation. The Game Model's Amount is the semantic expression such a
Complement may denote; the Amount Complement is its syntactic realization.
_Avoid_: Amount for the Complement itself; Object for a numeral

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
pp. 562–563).

**Connective Adjunct**:
An Adjunct relating its Clause to surrounding text or context, such as
sentence-initial *Then* or *Otherwise* (CGEL, Ch. 8 §19, pp. 775, 778).

**Depictive Adjunct**:
An optional predicative Adjunct describing a participant during the situation,
as in *enters tapped and attacking* (CGEL, Ch. 4 §5.3, pp. 262–263; Ch. 14
§9, p. 1265).

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

**Extraction**:
A grammatical dependency between a displaced expression and its Gap, subject to
constraints on the intervening constituent boundaries.

**Gap**:
An unpronounced syntactic position related to an overt or understood element
elsewhere in the structure.

**Ellipsis**:
The omission of material whose grammatical content is recoverable from an
Antecedent or context.

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

**Number**:
The grammatical feature distinguishing singular and plural values where English
grammar requires the distinction.

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
