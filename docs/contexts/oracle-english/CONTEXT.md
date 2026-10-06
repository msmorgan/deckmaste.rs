# Oracle English

The canonical linguistic vocabulary for English Oracle text and the
construction grammar that analyzes and realizes it. Game concepts named in
these definitions retain their [Game Model](../game-model/CONTEXT.md) meanings.

## Authority

Established linguistic terminology controls this context; explicitly
project-defined terms fill gaps needed by the Oracle-text model.

## Language

**Construction**:
A grammatical schema pairing a structural pattern with its interpretation.

**Category**:
A class of lexical or syntactic units with the same grammatical distribution.

**Constituent**:
A lexical or syntactic unit functioning as a part of a larger expression.

**Production**:
A grammar rule by which a Category is formed from an ordered pattern of other
Categories or terminals.

**Derivation** (project term):
A proof that a Constituent is formed by a sequence of Productions. Two
Derivations of the same Reading are the same Reading; they are not an
Ambiguity.
_Avoid_: Reading for the proof rather than the analysis

**Construction Cost** (project term):
A nonnegative preference weight on a Construction. A Reading's total sums
the costs of its Construction occurrences; lower totals rank earlier without
changing grammatical admission or erasing alternative Readings.

**Lexeme**:
An abstract lexical item underlying its inflected Word Forms. A lexical item
may consist of several orthographic words, as in *mana value* and *mana cost*.

**Compound Noun**:
A noun Lexeme formed from two or more bases, as in *charge counter*; its
constituents form a lexical unit rather than freely independent syntactic
Modifiers (CGEL, Ch. 5 §14.4, pp. 448–451).

**Type Word** (project term):
A Lexeme sourced from a declared card-type, subtype or supertype inventory for
ordinary noun or adjective use in Oracle text. It is distinct from an exact
Catalog atom used in a Type Line. Noun uses retain their Part of Speech when
functioning as Premodifiers.

**Lemma**:
The conventional citation form used to identify a Lexeme.

**Word Form**:
An inflected surface form of a Lexeme.

**Spelling Variant** (project term):
One of two or more declared Word Forms of a Lexeme for the same Feature Bundle,
each of which is retained as a distinct grammatical value.

**Color Word** (project term):
One of the eight Word Forms *white*, *blue*, *black*, *red*, *green*,
*colorless*, *multicolored*, and *monocolored*. The first five name the Game
Model Colors [CR#105.1]; the last three state how many of those Colors an
object has [CR#105.2a..105.2c], and neither *multicolored* nor *colorless* is
itself a Color [CR#105.4].
_Avoid_: Color for the lexical Category

**Part of Speech**:
A lexical Category distinguished by grammatical distribution, such as noun,
verb or adjective.

**Lexical Analysis**:
An identification of an expression as a Word Form of a Lexeme, together with
its lexical Category and applicable grammatical features.

**Lexical Environment** (project term):
The declared vocabulary and grammatical properties available for analyzing an
expression.

**Provenance** (project term):
The declaration or catalog a Lexical Analysis came from. Provenance is part of a
Word Form's identity and is independent of its position in the source.

**Morphology**:
The formation of a Lexeme’s Word Forms and the grammatical distinctions they
express.

**Affix**:
A bound morpheme that combines with a word or stem, as the prefix *non-* does.

**Clitic**:
A grammatical element that attaches to an adjacent host while taking its
syntactic position independently of that host's word structure. The English
genitive ending attaches at the edge of a phrase.

**Capitalization**:
The use of uppercase and lowercase letters in a written expression.

**Word Payload** (project term):
The characters a lexical Word Form owns. A Word Payload is nonempty and contains
no quotation delimiter. A multiword lexical form owns its declared internal
spaces; external separators and delimiters belong to the surrounding
Realization. Arbitrary source whitespace is not a lexical form.

**Feature Bundle**:
A correlated collection of grammatical feature values describing one analysis.

**Reading**:
A grammatical analysis of an expression, preserving its lexical identities,
constituent structure and grammatical distinctions.

**Ambiguity**:
The availability of more than one Reading of the same expression.

**Admission** (project term):
The relation by which the grammar accepts a Reading of a given expression at a
given Category, combining constituent derivation, feature conformance,
dependency safety and Realization. An admitted Reading is retained regardless of
Preference.
_Avoid_: Selection, Parsing for this relation

**Preference** (project term):
An optional, non-destructive ranking annotated over already-admitted Readings. A
Preference can order alternatives and name a representative; it can neither add
a Reading nor revoke one.
_Avoid_: Selection, Disambiguation for this annotation

**Packing** (project term):
Grouping Readings that every possible parent treats alike, so that the group can
be carried as one item without constructing a product of its members. Packing
preserves every distinction that can affect a parent's admissibility and keeps
correlated alternatives correlated.
_Avoid_: Merging, Collapsing, which suggest losing a distinction

**Scope Class** (project term):
An equivalence class of admitted Readings related by licensed scope alternations,
which retain the same lexical identities and the same host structure. Membership
in one Scope Class is not uniqueness of interpretation.

**Realization**:
The relationship by which an abstract grammatical value is expressed in a
surface form.

**Linearization**:
The ordering of a Construction's constituents into a surface sequence.

**Atom** (project term):
One annotated unit of a surface, carrying whether it is an ordinary word, an
opening or closing delimiter, a symbol, or a line boundary. The annotation is
structural metadata, never inferred from the spelling.

**Document**:
Structured rules text for one text box or independently named face portion,
including its sentences, keyword lines, and editorial boundaries.

**Keyword Line**:
A line formed from one or more keyword-ability surfaces with their prescribed
parameters and associated reminder text.

**Type Line**:
The Document portion listing a card face's supertypes, card types and subtypes,
supertypes printed before card types [CR#205.1,205.4a], written with a long dash
before the subtypes as each card type's subtype rule prescribes [CR#302.3].
_Avoid_: Type Line as a Game Model concept — the Game Model glossary's Card Type
entry reserves it for the printed line

**Notation**:
A conventional written representation using symbols, numerals, or other
specialized marks rather than ordinary word-and-phrase grammar alone.

**Mana Phrase** (project term):
A phrase realized by mana notation or coordinated alternatives of that notation,
used as a selected Complement. Its symbols preserve lexical identity and are
licensed separately from symbols that appear only in Costs.

**Amount Complement** (project term):
A numeric Complement selected by a declared Amount Frame Slot. Its notation
preserves the quantity's structure; the slot is distinct from a nominal Object
and from a frequency Adjunct.

**Sentence**:
The highest ordinary syntactic unit of Oracle text, terminated by sentence
punctuation and containing one or more Clauses.

**Parenthetical**:
Supplementary material set off from the surrounding expression, commonly by
parentheses. It can refer to preceding linguistic material without becoming a
selected Complement of its host.

**Clause**:
A syntactic unit organized around a predication and its dependents.

**Subordinate Clause**:
A Clause that functions as a dependent within another Clause or phrase rather
than as an independent Sentence.

**Subordinator**:
A grammatical marker, such as content-clause or relative *that*, *whether*, or
interrogative *if*, that marks a Clause as subordinate. A Preposition taking a
Clause Complement is still a Preposition.

**Adjective**:
A lexical Category whose members characteristically modify Nominals or serve
as predicative Complements. A participial spelling can also have a distinct
Adjective analysis, as with *tapped*; sharing a spelling does not turn every
Participle into an Adjective (CGEL, Ch. 6 §2.4.3, pp. 540–542).

**Adjective Phrase**:
A phrase headed by an Adjective.

**Preposition**:
A lexical Category whose members head phrases expressing relations and
characteristically select Complements.

**Compound Preposition**:
A Preposition whose components have coalesced into a single Lexeme, as with
*instead* and *because*. Its following Complement remains a separate phrase;
*instead of …* retains the selected marker and its Complement structure
(CGEL, Ch. 7 §3.1, pp. 622–623).
_Avoid_: treating every adjacent sequence of Prepositions as one Lexeme

**Preposition Phrase**:
A phrase headed by a Preposition, with any selected Complements. Lexicalized
*face down* and *face up* have no overt Complement and function as adjuncts or
predicative complements (CGEL, pp. 633, 1268).

**Preposition Function Licence**:
Project term. The declared set of grammatical functions a Preposition's
phrase may take in Oracle English: Adjunct, Modifier or Complement of a noun,
verb-selected Complement, or predicative Complement (CGEL, Ch. 7 §2.1,
pp. 604–606). The licence is a set of functions, not a kind of Preposition
Phrase (CGEL, Ch. 7 §2.2, p. 617, n. 3), and may be narrower than in general
English. CGEL licenses predicative-*as* phrases in Adjunct function generally
(Ch. 7 §5.1, p. 637). Their licence in Oracle English admits only the preposed,
comma-separated, clause-initial position, under the
[orchestrator resolution of 2026-10-06](../../tickets/done/english-v3-fixed-cost-phrases.md).
_Avoid_: adverbial PP, adjectival PP, AdverbialUse

**Predicative Complement of a Preposition**:
A Complement that ascribes a property or role to a Predicand, as the Noun
Phrase in *as treasurer* does; the containing Preposition Phrase may itself
function as an Adjunct (CGEL, Ch. 7 §5.1, pp. 636–637).

**Predicand**:
The expression of which a property or role is predicated by a predicative
Complement (CGEL, Ch. 7 §5.1, p. 637).

**Voice**:
The grammatical organization of a predication's participants, including the
active and passive patterns of Subject and Complement realization.

**Bare Passive**:
A nonfinite passive Clause without an expanding catenative verb such as *be* or
*get*. It may include an overt Subject or an internalised *by*-Complement
(CGEL, Ch. 16 §10.1.1, p. 1430).

**Internalised Complement**:
The Complement within a passive Verb Phrase that corresponds to the Subject
of its active counterpart, normally marked by *by*. It bears that Subject's
semantic role, which need not be agent (CGEL, Ch. 16 §10.1.1, p. 1428;
Ch. 8 §2.2, pp. 674–675). This function is distinct from a Means Adjunct and
from a Scalar Change Extent Complement selected by a lexical head.
_Avoid_: Agent for this syntactic function

**Participial Use** (project term):
The distinction, in participial Complement selection, between a Bare Passive
and an ordinary lexical or auxiliary predicate. An expanded passive has
ordinary auxiliary structure while containing a Bare Passive Complement;
Coordination preserves the uses of its members (CGEL, pp. 1430–1431).

**Polarity**:
The grammatical distinction between positive and negative expressions.

**Countability**:
The distinction between count and mass uses of a noun. A Lexeme may license
both uses; Number and choice of Determiner constrain which use is available.

**Number Transparency**:
The property of a quantificational noun use in which the Number of its Oblique
controls the whole Noun Phrase's Agreement, obligatorily overriding the head
noun's Number (CGEL, Ch. 5 §3.3, pp. 349–352; §18.2, pp. 501–504).

**Oblique**:
The Noun Phrase Complement of *of* in a quantificational noun Construction;
the quantificational noun remains the syntactic head and *of* forms a
Constituent with the Oblique (CGEL, Ch. 5 §3.3, pp. 349–352).
_Avoid_: Head for the Oblique

**Genitive**:
A grammatical form expressing possession or a related dependency, including
possessive determinatives and phrases marked with an apostrophe ending.

**Bare Genitive**:
A genitive marked in writing by an apostrophe without an added *s*, normally
on a noun ending in *s*. It is obligatory for plural nouns with that ending,
as in *owners’* (CGEL, Ch. 18 §4.2, p. 1595).

**Relative Clause**:
A dependent Clause that modifies a nominal expression or supplements another
expression, with a relative element connected to a position in the Clause.

**Extraction**:
A grammatical dependency between a displaced expression and its Gap, subject
to constraints on the intervening constituent boundaries.

**Right Node Raising**:
A sharing Construction in which material following a Coordination fills a
corresponding unpronounced position in each Conjunct.

**Identity Claim** (project term):
A lexical claim backed by a spelling supplied by the parse context or a
catalog. It records the source of the spelling without resolving a game
Referent.

**Ellipsis**:
The omission of material whose grammatical content is recoverable from an
Antecedent or context. Oracle English does not license stranded
gerund-participial auxiliaries: stranded *being* is excluded in American
English, and stranded *having* is dialect-restricted (CGEL, Ch. 17 §7.1,
pp. 1522–1523).

**Cardinal Numeral**:
A numeral expressing a count, such as *one* or *two*.

**Measure Phrase**:
A phrase expressing an extent or scalar value, including arithmetic
combinations and comparisons with other values.

**Measured Noun Phrase** (project term):
A Noun Phrase expressing a scalar amount or attribute through a quantity and
a head with a declared measure-position licence. The lexical head retains
its countability; a quantified amount can be conceptualised as one entity
(CGEL, Ch. 5 §3.4, p. 354).

**Measured Nominal** (project term):
A quantified amount in Nominal form under an outer Determiner. Its singular
quantity conceptualization permits an indefinite Determiner without giving
the lexical mass head a bare count-plural use (CGEL, Ch. 5 §3.4, p. 354).

**Cardinal Measurement Use** (project term):
A declared lexical licence for a mass Nominal to express units through
cardinal quantification, including a variable count. It is distinct from
scalar measurement and supplies no bare count-plural use.

**Adverb**:
A lexical Category whose members characteristically modify a phrase, Clause,
or other expression without serving as a nominal argument.

**Adverb Phrase**:
A phrase headed by an Adverb.

**Focus**:
The constituent whose interpretation is made prominent or restricted relative
to alternatives.

**Focus Adverb**:
An Adverb that associates with a Focus, such as restrictive *only*.

**Predicate**:
The grammatical function in a Clause that predicates something of the Subject.
This is distinct from a Game Model Predicate.

**Verb Phrase**:
A phrase headed by a verb.

**Bare Predicate**:
A Predicate headed by a plain-form verb. It can occur in a finite imperative
or as a nonfinite Complement; the Word Form alone does not determine Finiteness.

**Secondary Verb Phrase**:
A Verb Phrase whose head has a secondary Word Form: plain, gerund-participle,
or past participle. Plain form is shared by finite imperatives and subjunctives
and nonfinite infinitivals; participial forms are nonfinite.

**Locative Complement**:
A Complement expressing location, source, or goal. It is distinct from a
predicative Complement and from an optional locative Adjunct.

**Lexical Verb Phrase**:
An instantiated lexical verb together with the complements it selects, before
higher auxiliary and clause structure is added.

**Verb Frame**:
An ordered lexical schema describing the complements and fixed markers a verb
licenses.
_Avoid_: Verb Phrase for the schema; Verb Frame for an instantiated phrase

**Frame Slot** (project term):
One position in a Verb Frame: the grammatical relation the position bears —
Subject, Object or Complement — paired with the Category that fills it. A Frame
Slot whose relation is Subject is not a Complement.
_Avoid_: Complement for the slot descriptor

**Nominal**:
An intermediate nominal constituent headed by a noun or noun-like element.

**Noun Phrase**:
A phrase whose head or fused head is nominal and which can fill functions such
as Subject, Object, or Complement of a preposition.

**Determinative**:
A lexical Category containing words such as *the*, *this*, *each*, and *any*.
It is distinct from the Determiner function.

**Determinative Phrase**:
A phrase headed by a Determinative, including cardinal determinatives.

**Determiner**:
The grammatical function that marks a Noun Phrase as definite, quantified, or
otherwise determined. A Determinative commonly realizes this function.

**Determiner Requirement** (project term):
A Nominal's unresolved need for an outer Determiner when a cardinal numeral
functions as its internal Modifier, as in *that one mistake* (CGEL, Ch. 5
§7.6, p. 386). Without the outer Determiner the numeral instead functions as
the Determiner.

**Partitive Noun Phrase**:
A fused-head Noun Phrase containing an *of* phrase and denoting a subset of
the set denoted by that phrase's Complement, as in *one of them*.
The Complement of *of* is the partitive oblique (CGEL, Ch. 5, p. 333).
_Avoid_: an omitted noun as an invented lexical head

**Quantitative Determiner**:
A Determiner that expresses a count, realized by a Cardinal Numeral or a
quantitative Preposition Phrase. In *up to two target creature cards*, the
Preposition Phrase *up to two* fills this function and the Nominal retains its
noun head (CGEL, Ch. 5 §4, pp. 357–358).

**Quantitative Preposition Phrase**:
A Preposition Phrase expressing a quantity. In determiner function its
Complement is normally a Cardinal Numeral (CGEL, Ch. 5 §4, p. 357).
The sequence *up to* retains nested Preposition Phrase structure: *up* takes
the Complement *to …* (CGEL, Ch. 7 §3.2, pp. 624–625).

**Quantity**:
The Game Model count constraint a Determiner states over a Selection, such as
*one*, *up to two*, *one or more*, or *any number*. Determiners are this
context's side of it; the constraint itself belongs to the
[Game Model](../game-model/CONTEXT.md).

**Subject**:
A grammatical relation within a Clause, not a constituent Category or a
pronoun case.

**Object**:
A grammatical relation licensed by a head, not a constituent Category. This
term may coexist with the unrelated Game Model Object.

**Complement**:
A dependent selected by its head.

**Measure Complement**:
A Complement that supplies a quantity or extent licensed by its head.
_Avoid_: Numerative

**Scalar Change Extent Complement**:
A Complement expressing how far a value changes along a scale, licensed by
an appropriate lexical head; overall extent can be marked by *by* (CGEL,
Ch. 8 §§5.2–5.3, pp. 691–693).

**Adjunct**:
An optional dependent that modifies a phrase without being selected as its
Complement.

**Means Adjunct**:
An Adjunct expressing how a situation is brought about, commonly a *by*
Preposition Phrase with a nominal or gerund-participial Complement. It can
occur in active and passive Clauses independently of an Internalised
Complement (CGEL, Ch. 8 §2.2, pp. 673–675).

**Frequency Adjunct**:
An Adjunct that quantifies occurrences of a situation, such as *X times* or
*twice*. Frequency quantifies situations rather than time points or periods
(CGEL, Ch. 8, §9, pp. 713–716).

**Frequency Phrase** (project term):
A phrase licensed to function as a Frequency Adjunct; a counted Noun Phrase
can have this function without becoming an Object or changing its noun's
Part of Speech.

**Adverbial Use**:
The licensed use of a phrase as an Adjunct to a predicate or Clause, distinct
from nominal modification or a selected Complement.

**Duration Phrase**:
An Adjunct that bounds how long the effect of its Clause lasts, such as *until
end of turn*.

**Modifier**:
A dependent that attributes or restricts the interpretation of its head.

**Premodifier**:
A Modifier that precedes its head in the linear order. A noun used as a
Premodifier remains a noun: *creature* in *creature card* does not acquire an
Adjective lexical identity (CGEL, pp. 444, 537–538). Type and subtype declarations
license their singular noun forms for this function; the head determines the
Nominal's Number and Countability. A verbal gerund-participle can also be a
Premodifier, as in *attacking creature*, without becoming an Adjective
(CGEL, Ch. 6 §2.4.3, pp. 541–542). A past-participial verb phrase can likewise
premodify a noun, as in *the chosen type* (CGEL, Ch. 5 §14.2, p. 444).
The lexical declaration records this distribution; a transitive frame alone
does not supply declared past-participial Premodifier availability.

**Resultative Complement**:
A predicative Complement denoting the state of its predicand at the end of a
process, as in *painted the fence blue*. Resultatives require licensing by the
verb; optional depictives describe a participant during the situation and are
Adjuncts (CGEL, Ch. 4 §§5, 5.3, pp. 251–252, 261–263).

**Depictive Adjunct**:
An optional predicative Adjunct describing a participant during the situation,
as in *enters tapped and attacking*; it may be an Adjective Phrase or a
gerund-participial Clause (CGEL, Ch. 4 §5.3, pp. 262–263; Ch. 14 §9, p. 1265).

A past-participial verb phrase can likewise premodify a noun, as in
*the chosen type* (CGEL, Ch. 5 §14.2, p. 444).

**Postmodifier**:
A Modifier that follows its head in the linear order. Subjectless
gerund-participial Clauses (*creature attacking you*, *player being attacked*)
and Bare Passives can postmodify nouns without being Relative Clauses
(CGEL, Ch. 14 §9, pp. 1264–1266).

**Coordination**:
A construction joining two or more coordinate units. Conjuncts can have
different Categories when their grammatical function permits it, including
an Adjective Phrase and a gerund-participial Clause under shared copular/
progressive *be* (*are tapped and attacking*), without granting that selection
to other copular verbs (CGEL, Ch. 15 §3.2, pp. 1327–1328).

**Right Nonce-constituent Coordination**:
Coordination of parallel sequences at a clause's right edge, each sequence
independently filling the same functions under a shared head; coordination
gives the sequences constituent status (CGEL, Ch. 15 §4.3, pp. 1341–1343).

**Correlative Coordination**:
A Coordination with a marker associated with its first Conjunct and a paired
Coordinator, as in *both … and*, *either … or*, or *neither … nor*.

**Shared Complement**:
A Complement realized once after coordinated heads that each independently
license its grammatical function.

**Conjunct**:
One of the units joined in a Coordination.

**Coordinator**:
The marker that links Conjuncts, such as *and* or *or*.

**Anchor** (project term):
An ordered position at which a Coordination or a coordinated frame segment joins
its parts. The ordered arrangement of a Reading's Anchors, including group arity,
is part of its structural identity.

**Serial Comma**:
The comma written before the Coordinator of a flat Coordination of three or more
Conjuncts. Its presence marks the Coordination as flat rather than nested.

**Gap**:
An unpronounced syntactic position related to an overt or understood element
elsewhere in the structure.

**Anaphor**:
An expression whose interpretation depends on an Antecedent in its discourse
or syntactic context.

**Antecedent**:
The expression or discourse introduction on which an Anaphor depends.

**Referent**:
A Game Model Object, Player, value, or group that an expression denotes in
context.

**Agreement**:
The grammatical relationship in which one expression's features constrain the
form of another. Agreement is not itself Finiteness or an Inflectional Form.

**Case**:
A grammatical distinction between forms of a nominal expression, such as
nominative, accusative and genitive, associated with its syntactic distribution.

**Person**:
The grammatical feature distinguishing speaker, addressee, and other
referents.

**Number**:
The grammatical feature distinguishing singular and plural values where
English grammar requires the distinction.

**Concord Class** (project term):
A derived morphological equivalence class used to choose a finite agreeing
form, initially third-person singular versus other.

**Finiteness**:
The grammatical property distinguishing finite clauses and Verb Phrases from
nonfinite ones. It is not an Inflectional Form.

**Tense**:
The grammatical marking of temporal location relative to a reference point.
It is distinct from Finiteness and from the Word Form that realizes it.

**Inflectional Form**:
A morphological form of a Verb Lexeme, such as plain, third-person-singular
present, preterite, gerund-participle, or past participle. It is distinct from
Finiteness and from Agreement, though those dimensions constrain its selection.
_Avoid_: Verb Form when the specific morphological dimension is meant

**Onset** (project term):
The consonantal or vocalic beginning of a realized expression, determined by
its first pronounced constituent. Lexical pronunciation, including declared
overrides, supplies this feature for *a*/*an* selection; orthography is a
fallback rather than its authority.

**Targeting Marker** (project term):
The invariant prenominal *target* that marks an Oracle-text target description.
It has determinative and nominal-modifier projections but is distinct from the
count noun *target* and the verb *target*.

**Target Noun**:
The count-noun Lexeme whose Word Forms *target* and *targets* denote Game Model
Targets.

**Target Verb**:
The Verb Lexeme realized as *target*, *targets*, *targeted*, or *targeting* in
rules text stating that a Game Model Spell or Ability targets a Game Model
Object or Player.

**Italic Head** (project term):
The italicized run that opens some abilities, followed by an em dash. It is
either an Ability Word or a Flavor Word; neither has any rules meaning, so the
Italic Head is a realization device and never changes what the ability it
heads says.

**Ability Word**:
One of the italicized words the Comprehensive Rules list [CR#207.2c], written
as an Italic Head to tie together abilities with similar functionality. The
inventory is closed and declared, so an Ability Word is ordinary declared
vocabulary.
_Avoid_: Keyword Ability, which has rules meaning and its own CR entry

**Flavor Word**:
An Italic Head the Comprehensive Rules do not list [CR#207.2d], tailored to the
one ability it heads. Nothing enumerates the Flavor Words, so a Flavor Word is
an open slot whose label is the italic run captured verbatim, never a declared
inventory member.
_Avoid_: Flavor Text, the italicized artistic text below the rules text
[CR#207.2b]

**Keyword Quality** (project term):
The Nominal a parameterized Keyword Ability takes as its argument, on a keyword
line or in a grant — *black* in *protection from black*, *artifacts* in
*affinity for artifacts*. Each Keyword Ability's declaration selects whether its
Keyword Quality is bare, introduced by a Preposition, or carries the keyword's
Bound Keyword Surface.

**Bound Keyword Surface** (project term):
The surface a Keyword Ability contributes as a suffix bound to its Keyword
Quality, so that quality and keyword are written as one word — *walk* in
*islandwalk* and in *nonbasic landwalk* [CR#702.14a]. A declaration states it
separately from that Keyword Ability's own free surface. Quality and Bound
Keyword Surface are cased together as the one word they form: capitalized only
where that word begins a sentence or a keyword line — *Islandwalk*, *Snow
swampwalk*, *Enchanted creature has mountainwalk.*
_Avoid_: Prefix for the Keyword Quality it binds to

## Status, Designation (Game Model terms — not grammatical categories)

`tapped`, `goaded`, `suspected`, `saddled` are the **Participles** of their
keyword-action verbs, used attributively or predicatively; `monstrous` is an
**Adjective**; `the monarch`, `the initiative` are **Noun Phrases**; `face down`
is a lexicalized **Preposition Phrase** (analysis adopted 2026-10-04,
superseding this part of the 2026-09-05 ruling). Whether such a form names a
status [CR#110.5] or a designation [CR#701.15b] is Game Model semantics resolved through the verb
declaration, never a grammatical class. _Avoid_ "Status", "Designation" as the
name of a vocabulary, codec, or construction in the grammar (ruling 2026-09-05).
