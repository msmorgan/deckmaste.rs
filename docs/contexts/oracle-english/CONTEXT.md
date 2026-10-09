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
ordinary noun or adjective use in Oracle text. Subtype nouns also serve in a
Type Line; card types and supertypes retain separate exact Catalog atoms for
that use. Noun uses retain their Part of Speech when functioning as
Premodifiers.

**Shortened Name**:
A shortened spelling printed on a card in place of its full name that refers
to that same card [CR#201.5c].
_Avoid_: nickname

**Name Number** (project term):
The declared grammatical Number of a full Name for Agreement. It describes
agreement in the attested wording, independently of how many game objects the
Name refers to (owner ruling, 2026-10-08).
_Avoid_: object count

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
genitive ending attaches at the edge of a phrase. Clitic auxiliaries are
unstressed versions of tensed auxiliary forms; auxiliary stranding requires a
strong form (CGEL, Ch. 18 §§6.1–6.2, pp. 1614–1616).

**Clitic Host**:
The adjacent expression to which a Clitic attaches. Present auxiliary clitics
other than the third-person singular require a pronoun that is itself the
Subject; third-person singular and preterite clitics also permit phrase hosts
(CGEL, Ch. 18 §6.2, pp. 1615–1616).

**Subject Structure** (project term):
The distinction between a pronoun that is itself the Subject and a larger
Subject containing a pronoun or other constituents; a coordinated Subject is
of the latter kind (CGEL, Ch. 18 §6.2, p. 1615).

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

**Selected Literal** (project term):
A fixed surface expression determined by its constituents' declared features
or grammatical summaries, under the [derived-punctuation project ruling,
2026-10-08](../../tickets/done/english-v3-derived-punctuation-forms.md#why).
Its spelling contributes no independent distinction to a Reading.
_Avoid_: Spelling Variant for a structurally determined expression

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

**Supplement**:
Material set apart from a related expression, its **anchor**, without
integration into the anchor's headed syntax (CGEL, Ch. 15 §5.1, pp. 1350–1351). The variable-defining
*where X is …* Preposition Phrase is supplementary under the
[where-variable-clause project analysis](../../tickets/done/english-v3-where-variable-clause.md#project-analysis).

**Parenthetical**:
Supplementary material set off from the surrounding expression, commonly by
parentheses. It can refer to preceding linguistic material without becoming a
selected Complement of its host.

**Clause**:
A syntactic unit organized around a predication and its dependents.

**Dummy Pronoun**:
A Pronoun without independent semantic content that fills a syntactic position;
existential *there* fills Subject position and inherits Agreement features from
the displaced Subject (CGEL, Ch. 16 §6.1, p. 1391; Ch. 4 §3.2.2, p. 242).
_Avoid_: locative Adverb for dummy *there*

**Displaced Subject**:
An internal Complement of a verb corresponding semantically to the Subject of
a basic counterpart, while the actual Subject position is filled by dummy
*there* (CGEL, Ch. 16 §6.1, p. 1391).
_Avoid_: Object, a second Subject

**Existential Clause**:
A Clause defined syntactically by the combination of dummy *there* and *be*
(CGEL, Ch. 16 §6.2, p. 1392).

**Existential Use** (project term):
A Pronoun Lexeme's declared permission to fill the dummy Subject position of
an Existential Clause.

**Existential Extension**:
An element outside the displaced Subject that extends an Existential Clause;
a locative extension is a Complement of *be* (CGEL, Ch. 16 §6.2.1,
pp. 1393–1394).

**Content Clause**:
A Subordinate Clause expressing propositional content. Compound Prepositions
*as if* and *as though* take content clauses as Complements (CGEL, Ch. 13
§5.5, p. 1151).

**Irrealis**:
The non-tensed Inflectional Form of *be* used with first- or third-person
singular Subjects in modal-remoteness constructions; other Persons and
Numbers use the preterite (CGEL, Ch. 3 §1.7, pp. 86–88). An irrealis Clause
is finite (CGEL, Ch. 3 §§1.8–1.8.1, p. 88). *As if* and *as though* permit
irrealis as well as modal preterite (CGEL, Ch. 13 §5.5, p. 1152).
_Avoid_: past subjunctive; treating irrealis as a Past Tense

**Irrealis Clause**:
A finite Clause whose Predicate contains an Irrealis verb.

**Irrealis Clause Complement** (project term):
A Preposition's declared permission to take a finite Content Clause with an
Irrealis Predicate. It does not itself license a grammatical function or
attachment for the resulting Preposition Phrase.

**Subordinate Clause**:
A Clause that functions as a dependent within another Clause or phrase rather
than as an independent Sentence.

**Verbless Clause**:
A Clause whose Predicate contains no verb. A reducible Clause Complement of a
Preposition may also omit its Subject (CGEL, Ch. 14 §10(b), p. 1267).

**Verbless Predicate Use** (project term):
An Adjective's declared permission to realize a Verbless Clause's Predicate
with its selected infinitival Complement unexpressed. The licence for *able*
is the [attestation-based project decision](../../tickets/done/english-v3-verbless-if-complement.md#analysis),
not an example supplied by CGEL's account of *necessary* and *possible*.

**Verbless Clause Complement** (project term):
A Preposition's declared permission to take a Verbless Clause as its
Complement. This permission does not by itself license a grammatical function
or an attachment position for the resulting Preposition Phrase.

**Gerund-participial Clause**:
A nonfinite Clause headed by a verb in its gerund-participle Word Form. Such
Clauses occur freely as Complements of Prepositions (CGEL, Ch. 14 §8.3,
p. 1262).

**Gerund Clause Complement** (project term):
A Preposition's declared permission to take a Gerund-participial Clause as
Complement. Availability of that Complement does not itself license a
function for the resulting Preposition Phrase.

**Complement Function Restriction** (project term):
A restriction on the grammatical functions of a Preposition Phrase associated
with a particular Complement shape, in addition to its head's Preposition
Function Licence. The gerund-complemented *without* phrase has Adjunct use;
the gerund-complemented *of* phrase is internal to its selecting Compound
Preposition Phrase. These are the
[gerund-preposition-complements project requirements](../../tickets/done/english-v3-gerund-preposition-complements.md#goal),
not restrictions supplied by CGEL's general account of complementation.
Coordination retains only the functions permitted for every conjunct.

**Bare Infinitival**:
A nonfinite Clause with a plain-form verb and without the infinitival
Subordinator *to* (CGEL, Ch. 14 §1.1, pp. 1173–1174).
_Avoid_: treating the plain Word Form alone as evidence of Finiteness

**Hollow Infinitival**:
A nonfinite Clause with a non-Subject Gap linked to an Antecedent outside it;
a hollow *to*-infinitival can be an internal Complement whose Object Gap is
linked to the matrix Subject (CGEL, Ch. 14 §6.3(c), p. 1250).

**Measure Modifier Use** (project term):
A comparative head's declared permission to take a measure as a degree Modifier.
Its application to mana notation is the
[cost-scalar-complement project analysis](../../tickets/done/english-v3-cost-scalar-complement.md#project-analysis),
not a mana-symbol analysis supplied by CGEL.

**Catenative Construction**:
A Construction in which a verb selects a nonfinite internal Complement.
The complex Construction contains an intervening Noun Phrase between the
matrix and subordinate verbs (CGEL, Ch. 14 §1.2, pp. 1177–1178).

**Catenative Complement**:
A nonfinite Clause selected by a catenative verb, with a Complement function
distinct from Object or predicative Complement (CGEL, Ch. 14 §1.2,
pp. 1176–1177).

**Subordinator**:
A grammatical marker, such as content-clause or relative *that*, *whether*, or
interrogative *if*, that marks a Clause as subordinate. A Preposition taking a
Clause Complement is still a Preposition.

**Adjective**:
A lexical Category whose members characteristically modify Nominals or serve
as predicative Complements. A participial spelling can also have a distinct
Adjective analysis, as with *tapped*; sharing a spelling does not turn every
Participle into an Adjective (CGEL, Ch. 6 §2.4.3, pp. 540–542).

**Adjective Origin** (project term):
The distinction between an ordinary Adjective and one formed from a declared
participial source. An Adjective Phrase containing any participial-origin
Conjunct retains that origin. Ordinary adjectival Premodification preserves
Property Value Position; participial-origin modification does not, under the
[orchestrator ruling (2026-10-08)](../../tickets/done/english-v3-coordinated-property-values.md#post-landing-fix-2026-10-08).
_Avoid_: treating every Adjective with a participial spelling as an ordinary Adjective

**Duration Adverb**:
An Adverb expressing temporal extent, as with *long* in *It won't last long*
(CGEL, Ch. 6 §5.2, p. 569). A duration Adverb Phrase can be a selected
Complement of a Preposition, as in *for long* (CGEL, Ch. 7 §5.1(d), p. 640).
_Avoid_: classifying temporal *long* as an Adjective solely from its spelling

**Comparative Governor**:
An item that licenses a comparative Complement, such as *equal*, *less*,
*greater* or *other* (CGEL, Ch. 13 §1.3, p. 1104).

**Comparative Governor Head** (project term):
An adjectival Comparative Governor together with its selected Preposition,
preceding a bare Comparative Complement. Such left portions can share a
delayed right Complement; CGEL, Ch. 15 §4.4, p. 1343, n. 66 illustrates
comparative Coordination retaining each selected marker.

**Comparative Complement**:
The expression of the secondary term in a comparison. The bare Complement
excludes its governing Preposition; the expanded Complement includes that
Preposition (CGEL, Ch. 13 §1.3, pp. 1103–1104).

**Comparative Phrase**:
A phrase containing a Comparative Governor (CGEL, Ch. 13 §1.3, p. 1105).
An adjectival Comparative Phrase retains Adjective Phrase distribution.

**Discontinuous Comparative Nominal** (project term):
A Nominal whose Comparative Governor premodifies its head while the governor's
expanded Comparative Complement follows it, as in *same name as that land*
(CGEL, Ch. 13 §5.1, p. 1139 describes *same* with *as* + NP; the
[fixed-cost-phrases project ruling of 2026-10-06](../../tickets/done/english-v3-fixed-cost-phrases.md#stop--comparative-admission-2026-10-06-resolved-by-orchestrator)
assigns the Complement to the governor's licence).
_Avoid_: Adjunct for the governor's selected comparison

**Comparative Complement Form** (project term):
The syntactic form of a compared Constituent: nominal, bare infinitival, or
gerund-participial. Matching these forms in the attested postposed
*rather than* Construction is the
[rather-than-complements ticket's project analysis](../../tickets/done/english-v3-rather-than-complements.md#analysis).
CGEL, Ch. 14 §8.4(b), p. 1263 describes the matching infinitival form in
*would rather*; it does not establish the project's NP or gerund analysis.

**Matrix Comparison Use** (project term):
An Adverb Comparative Governor's declared permission to take an Expanded
Comparative Complement matched to a preceding matrix Constituent, under the
[rather-than-complements project analysis](../../tickets/done/english-v3-rather-than-complements.md#analysis).
_Avoid_: an unselected Adverb Adjunct for a governor with a required Complement

**Expanded Comparative Complement**:
A comparative Complement introduced by its own marker, such as the second
*as* in a scalar equality comparison (CGEL, Ch. 13 §1.3, p. 1104).
_Avoid_: treating a marker selected by a different comparative construction as interchangeable

**Scalar Equality**:
A comparison in which *as* governs a phrase and an expanded Complement
introduced by a second *as*, as in *as long as …* in the duration sense
(CGEL, Ch. 13 §1.3, p. 1104; §4.5(d), p. 1134).
_Avoid_: treating a reanalysed conditional Compound Preposition as a duration comparison

**Adjective Phrase**:
A phrase headed by an Adjective.

**Adjectival Passive**:
An Adjective Phrase related to a verb's past participle and functioning as a
predicative Complement. The containing Clause is complex-intransitive, rather
than a passive Clause. A dynamic host such as *become* can denote a change of
state while its adjectival Complement denotes the resulting state (CGEL,
Ch. 16 §10.1.3, pp. 1436, 1438–1439).
_Avoid_: Bare Passive for an adjectival Complement; adjectival passive Clause

**Preposition**:
A lexical Category whose members head phrases expressing relations and
characteristically select Complements.

**Compound Preposition**:
A Preposition whose components have coalesced into a single Lexeme, as with
*instead* and *because*. Its following Complement remains a separate phrase;
*instead of …* retains the selected marker and its Complement structure
(CGEL, Ch. 7 §3.1, pp. 622–623). Conditional *as long as* is a reanalysed
Compound Preposition meaning “provided”, distinct from the duration
comparison (CGEL, Ch. 13 §4.5(d), p. 1134).
_Avoid_: treating every adjacent sequence of Prepositions as one Lexeme

**Preposition Phrase**:
A phrase headed by a Preposition, with any selected Complements. Lexicalized
*face down* and *face up* have no overt Complement and function as adjuncts or
predicative complements (CGEL, pp. 633, 1268).

**Intransitive Preposition**:
A Preposition used without a Complement. Such a use can constitute a complete
Preposition Phrase; it does not change the head's lexical Category into Adverb
(CGEL, Ch. 7 §2.4, pp. 612–613).
_Avoid_: adverb for a complementless Preposition

**Matrix-licensed Complement**:
A Complement of a Preposition whose availability depends on features of the
containing matrix Clause, rather than solely on the Preposition's lexical
selection (CGEL, Ch. 7 §5.1(g), pp. 641–643).
_Avoid_: treating every Complement of an exception-marking Preposition as matrix-licensed

**Matrix Complement Licence** (project term):
A host's declared permission for a nominal or content-clause exception
Complement. The Oracle copy-description and selected-*for* uses are the
[project analysis requested by the except ticket](../../tickets/done/english-v3-except-matrix-complements.md#goal),
not an application established by CGEL's speech-verb example.

**Matrix Complement Requirement** (project term):
The host distribution an exception PP requires before attaching: an
internalised passive Complement, a nominal exception, or a content-clause
exception. This requirement remains separate from the PP's function licence.

**Matrix Complement Governor** (project term):
A Preposition's declared availability to head a PP whose Complement needs a
matrix licence. Availability of the governor alone does not supply that licence.

**Nominal Exception Complement Use** (project term):
A PP's declared availability as the Complement of an exception governor in
nominal exception wording, including a fused-head cardinal Complement.
The selected-*for* relation is described by CGEL, Ch. 7 §5.1(c), p. 640;
the restrictions on Oracle hosts are the exception ticket's project analysis.

**Preposition Phrase Complement Licence** (project term):
A preposition's declared restriction on the heads of its PP Complements,
independent of its permission to take an NP Complement. Prepositions can take
PP Complements (CGEL, Ch. 7 §2.4, p. 617); source-marking *from* takes locative
PPs (Ch. 8 §4.3, p. 687). The inclusion of *among* in the supported source
class is a [project decision](../../tickets/done/english-v3-from-preposition-phrase-complement.md#project-analysis),
not an example or classification supplied by those passages.

**Source Complement Use** (project term):
The declared availability of a PP as the internal Complement of a
source-marking preposition. Its head licenses this distribution independently
of the outer preposition's Complement licence. This does not by itself give
the phrase verb-selected Complement or Adjunct function.

**Source Marker** (project term):
A declared property of a preposition that marks the starting location in a
source PP. It is distinct from a PP head's eligibility as the internal
Complement of that preposition, and from the grammatical function licence
of the complete PP (CGEL, Ch. 8 §4.3, p. 687).

**Source Complement**:
A Locative Complement expressing the starting location. Its PP form retains
the governing preposition and that preposition's internal Complement
(CGEL, Ch. 8 §4.3, p. 687).

**Source Modifier Present** (project term):
Whether a nominal head has a source-marked PP Postmodifier. A source inside
a distinct embedded Noun Phrase or Clause is not a Postmodifier of this head.

**Source-free Object** (project term):
An Object Noun Phrase whose own nominal head has no source-marked PP
Postmodifier. A Verb Frame that selects its source separately can require
this distribution. Other hosts retain the source PP's licensed nominal
function; this is a [project constraint](../../tickets/done/english-v3-from-preposition-phrase-complement.md#project-analysis).

**Frame Category Projection** (project term):
The correspondence between a lexical Frame Slot's Category and the grammar
Category that fills it, including any declared feature requirements of the slot.

**Preposition Function Licence**:
Project term. The declared set of grammatical functions a Preposition's
phrase may take in Oracle English: Adjunct, Modifier or Complement of a noun,
verb-selected Complement, or predicative Complement (CGEL, Ch. 7 §2.1,
pp. 604–606). Supplementary use is additionally licensed under the
[where-variable-clause project analysis](../../tickets/done/english-v3-where-variable-clause.md#project-analysis);
this is project policy, not a claim that CGEL treats a Supplement as a syntactic
dependent. The licence is a set of functions, not a kind of Preposition
Phrase (CGEL, Ch. 7 §2.2, p. 617, n. 3), and may be narrower than in general
English. CGEL licenses predicative-*as* phrases in Adjunct function generally
(Ch. 7 §5.1, p. 637). Their licence in Oracle English admits the preposed,
comma-separated, clause-initial position under the
[orchestrator resolution of 2026-10-06](../../tickets/done/english-v3-fixed-cost-phrases.md).
The [orchestrator ruling of 2026-10-07](../../tickets/done/english-v3-focusing-only.md#goal)
defers the clause-final widening until comparative and verb-selected replacement
analyses exist; their Complements must not become Adjuncts.
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

**Bare Noun Phrase**:
A Noun Phrase formed from a Nominal without a Determiner. Plural count and
non-count Nominals can form bare indefinite Noun Phrases; bare singular count
uses require more restricted licensing (CGEL, Ch. 5 §4, p. 355).

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

**Relative Use** (project term):
A Pronoun's declared permission to function as the relative marker of a
Relative Clause. A Pronoun in Relative Use does not head an ordinary
nominative pronoun Noun Phrase; that distribution is the
[orchestrator ruling of 2026-10-08](../../tickets/done/english-v3-once-frequency.md#stop-and-resolution),
represented by the declared permission rather than by lexical identity.

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
Antecedent or context. Oracle English licenses auxiliary stranding only in
do-support and negative *can*; stranded *be*, *have* and other modals are
excluded by the user ruling of 2026-10-06, applied by the orchestrator resolution
of 2026-10-07. This is the project's attestation restriction, not a claim that
CGEL excludes those auxiliaries from English ellipsis generally.

**Auxiliary Ellipsis Licence** (project term):
The declared permission for an auxiliary to occur with an omitted Complement.
An overt Complement does not require this permission. In a coordination with a
shared omitted Complement, every auxiliary must carry the permission.
_Avoid_: inferring this permission from the auxiliary's spelling or from a
mere enumerable Gap.

**Cardinal Numeral**:
A numeral expressing a count, such as *one* or *two*.

**Measure Phrase**:
A phrase expressing an extent or scalar value, including arithmetic
combinations and comparisons with other values.

**Scalar Location**:
The expression of a position on a scale, commonly with *be* and an NP as
predicative Complement (CGEL, Ch. 8 §5.4, p. 693).

**Scalar Denotation** (project term):
A noun use or measured amount's denotation of a quantity or scalar property.
An ordinary headed phrase retains that denotation; a number-transparent
quantificational use instead denotes its Oblique. Its licensing of a value Complement after
*be* is the restriction of the
[copular-scalar-location ticket](../../tickets/done/english-v3-copular-scalar-location.md),
not a restriction asserted by CGEL's scalar-location passage.

**Scalar Value** (project term):
A value expression realized by a scalar Measure Phrase, Cardinal Numeral,
numeral Coordination, or ordering comparative Adjective Phrase (including a
coordinated equality governor), the comparative extension being orchestrator
ruling R5a (2026-10-08). All forms can supply a scalar property dependent;
numeral and measurement forms also supply scalar location predicative
Complements, while comparative forms retain Adjective Phrase predicative
distribution.

**Scalar Sign** (project term):
The presence or absence of an explicit sign in scalar Notation. A present sign
retains its positive or negative meaning and its exact spelling.

**Scalar Property Value** (project term):
A Scalar Value selected as a dependent by a scalar property Nominal; its
treatment as a Complement in
*power 2* is a [project ruling (2026-10-07)](../../tickets/done/english-v3-scalar-property-values.md#project-ruling-scalar-property-values),
not a claim made by CGEL's account of property-denoting NP Postmodifiers.

**Measure Position** (project term):
A noun's declared permission for a scalar value to precede it in measurement.
Following property Complements are licensed by Property Value Position.

**Slash Pair** (project term):
An ordered pair of scalar components realized with a slash separator, as
in *0/2* or *X/X*. Its use as a selected Property Value is the
[coordinated-property-values project ruling](../../tickets/done/english-v3-coordinated-property-values.md#project-ruling-coordinated-property-values).

**Property Value** (project term):
A selected value Complement of a property Nominal: a Scalar Property Value
for a single property, or a Slash Pair for an ordered pair of properties.
This extension is the [coordinated-property-values project ruling](../../tickets/done/english-v3-coordinated-property-values.md#project-ruling-coordinated-property-values),
not an analysis attributed to CGEL.

**Property Value Position** (project term):
A property's declared position in a value: a single scalar property, the
first or second coordinate of a Slash Pair, or their ordered additive pair.
Ordinary adjectival Premodification preserves the position; participial and
noun Premodification and all Postmodification do not. Additive Coordination
of first and second positions licenses the pair, in that order. Other
Coordinations do not license a Property Value. The modifier restriction is the
[orchestrator ruling (2026-10-08)](../../tickets/done/english-v3-coordinated-property-values.md#post-landing-fix-2026-10-08). The position and pair selection are the
[coordinated-property-values project ruling](../../tickets/done/english-v3-coordinated-property-values.md#project-ruling-coordinated-property-values).

**Scalar Comparative Use** (project term):
An Adjective's declared permission to be the comparative Conjunct in a
Scalar Property Value, as in *4 or greater*; this permission supplies no
Determiner function, and *greater* retains its Adjective Category (CGEL,
Ch. 5 §11(d), p. 432, n. 48).

**Comparative Quantity Structure** (project term):
The distinction between a numeral Coordination and a headed comparative
Determinative Phrase; only the former supplies the coordinated Scalar
Property Value of this landing.

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

**Connective Adjunct**:
An Adjunct relating its Clause to surrounding text or context; connective
elements can also link smaller units (CGEL, Ch. 8 §19, p. 775). In Oracle
English, sentence-initial *Then* uses this Adverb route (CGEL, Ch. 8 §19,
p. 778, [9ii]); sole-linking comma-*then* is a coordination marker under the
project's orchestrator resolution of 2026-10-06.

**Focus**:
The constituent whose interpretation is made prominent or restricted relative
to alternatives.

**Focus Adverb**:
An Adverb that associates with a Focus, such as restrictive *only*.

**Focusing Modifier**:
A Modifier that associates with a scopal Focus. Restrictive focusing Adverbs
include *only* and can modify a Preposition Phrase (CGEL, Ch. 6 §7.3.1,
p. 587, [41–42]); their position relative to the Focus is a separate
distinction (p. 589).

**Focusing Modifier Use** (project term):
A Lexeme's declared permission to function as a Focusing Modifier, with its
restrictive or additive interpretation kept distinct.

**Focused Use** (project term):
A phrase's having a Focusing Modifier of its own. In this project's restrictive
PP and Adverb Phrase analysis, a phrase already in Focused Use cannot itself be the head of another
restrictive Focusing Modifier (post-landing review, 2026-10-08, extended to
Adverb Phrases by the [once-frequency landing](../../tickets/done/english-v3-once-frequency.md#landing-record)).

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

**Manner Complement**:
A dependent expressing manner that is selected by its head. Manner dependents
can have NP or PP form, and their function as Complement is distinct from
their syntactic Category (CGEL, Ch. 8 §2.1, p. 671).
The project marker convention labels the NP inside selected *in* a Manner
Complement; CGEL p. 671 [6] treats the whole PP as the manner phrase.

**Lexical Verb Phrase**:
An instantiated lexical verb together with the complements it selects, before
higher auxiliary and clause structure is added.

**Verb Frame**:
An ordered lexical schema describing the complements and fixed markers a verb
licenses.
_Avoid_: Verb Phrase for the schema; Verb Frame for an instantiated phrase

**Predicate Frame Use** (project term):
The lexical licence for a Verb Frame in Predicate function, distinct from an
auxiliary's selection of a secondary Verb Phrase. Clitic perfect *have* retains
the auxiliary use; American English stative and dynamic *have* are lexical
uses without clitic forms (CGEL, Ch. 3 §§2.1.5, 2.5.6, pp. 102, 111–113).

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

**Bare Preposition Use** (project term):
The declared availability of a singular count Noun Phrase without a Determiner
as the internal Complement of a preposition, distinguishing a bare location
from a spatial edge. The noun retains its Countability; this use does not
license a bare count Subject or Object. In the right-branching
analysis of *on top of the library*, *top of the library* is the Noun Phrase
Complement of *on*. This is a [project ruling (2026-10-06)](../../tickets/done/english-v3-library-position.md#post-landing-fix-2026-10-06),
grounded in *top of X* outside the idiom; CGEL, Ch. 7 §3.1, pp. 620–623 supplies
the competing analyses and diagnostics, not a decision for *on top of*.

**Bare Nominal Complement** (project term):
A preposition's declared permission to take a singular count Noun Phrase with
Bare Preposition Use, matched to the location or spatial-edge distribution.
The noun's permission alone does not make it a Complement of every preposition ([project ruling, 2026-10-06](../../tickets/done/english-v3-library-position.md#post-landing-fix-2026-10-06)).

**Nominal Complement Marker** (project term):
A declared marker of a nominal head's selected Complement, such as *of* in
*the bottom of the library* or the fused determiner-head *each of them*
(CGEL, Ch. 5 §§9.1–9.2, pp. 411–413). A selected Complement and an optional
Postmodifier have different functions even when both are Preposition Phrases
(CGEL, Ch. 5 §14, p. 439, introduces the distinction; §14.2, p. 446 [14], illustrates PP
Postmodifiers; Ch. 7 §3.1, pp. 620–623 discusses competing internal analyses).

**Selected Preposition Use** (project term):
The declared pairing of nominal and preposition permissions that restricts the
resulting Preposition Phrase to a selected Complement function in Oracle English.
The nominal permission also identifies the NP material of the corresponding
selected manner frame. It prevents a selected manner tail from becoming an NP
Postmodifier, a Locative Complement or a free Adjunct.
Manner PPs and selection are separate grammatical distinctions (CGEL, Ch. 8
§2.1, p. 671; Ch. 7 §2.1, pp. 604–606).

**Determinative**:
A lexical Category containing words such as *the*, *this*, *each*, and *any*.
It is distinct from the Determiner function.

**Determinative Phrase**:
A phrase headed by a Determinative, including cardinal determinatives.

**Degree Modifier**:
A Modifier expressing degree, including the Determinative *that* modifying
*many* or *much* within a Determinative Phrase (CGEL, Ch. 5 §11(c), p. 431).

**Degree Use** (project term):
A Determinative's declared permission to function as a Degree Modifier or as
the head admitting one. The grammatical functions are described in CGEL,
Ch. 5 §11(c), p. 431; their declared licensing is a project representation.

**Determiner**:
The grammatical function that marks a Noun Phrase as definite, quantified, or
otherwise determined. A Determinative commonly realizes this function.

**Determiner Requirement** (project term):
A Nominal's unresolved need for an outer Determiner when a cardinal numeral
functions as its internal Modifier, as in *that one mistake* (CGEL, Ch. 5
§7.6, p. 386). Without the outer Determiner the numeral instead functions as
the Determiner.

**Fused Determiner-Head**:
An expression combining the Determiner and Head functions in a Noun Phrase;
singular demonstratives can have this use without an overt nominal
(CGEL, Ch. 5 §§9.1–9.2, pp. 410–415).
_Avoid_: Pronoun for a Determinative in fused-head use

**Fused Head Use** (project term):
A Determinative's declared permission to fuse the Determiner and Head
functions without an overt nominal or partitive Complement. Fused heads and
their distributions are described in CGEL, Ch. 5 §§9.1–9.2, pp. 410–415;
the lexical permission is a project representation.

**Comparative Postmodifier Use** (project term):
A Noun Phrase head's declared permission to take a postpositive Comparative
Adjective Phrase as its Modifier. The fused-head *anywhere* use is the
pre-integrate review ruling (2026-10-09, `english-v3-from-anywhere`),
with the surrounding nominal attachment retained as a separate Reading.

**Partitive Noun Phrase**:
A fused-head Noun Phrase containing an *of* phrase and denoting a subset of
the set denoted by that phrase's Complement, as in *one of them*.
The Complement of *of* is the partitive oblique (CGEL, Ch. 5, p. 333).
_Avoid_: an omitted noun as an invented lexical head

**Quantitative Determiner**:
A Determiner that expresses a count or amount, realized by a Cardinal Numeral or a
quantitative Preposition Phrase, a comparative Determinative Phrase, or a
numeral Coordination, or a degree-modified Determinative Phrase (CGEL, Ch. 5
§11(c), p. 431). In *up to two target creature cards*, the
Preposition Phrase *up to two* fills this function and the Nominal retains its
noun head (CGEL, Ch. 5 §4, pp. 357–358).

**Quantified Noun Phrase**:
A Noun Phrase with a Quantitative Determiner and a Nominal head. Degree
determinatives select count plural or non-count singular heads according to
their lexical distribution (CGEL, Ch. 5 §7.11, p. 393).

**Comparative Quantity** (project term):
A comparative expression of quantity, including the numeral Coordination
*three or more* (CGEL, Ch. 5 §7.6, p. 386) and the Determinative Phrase
*more than one*, whose *than* Preposition Phrase is a Complement (CGEL,
Ch. 5 §11(d), p. 432). In Determiner function, *one or more* selects a plural
head, whereas *more than one* selects a singular head (CGEL, Ch. 5 §3.4,
p. 353, n. 13).

**Comparative Quantity Use** (project term):
A comparative Determinative's declared permission to take a quantitative
comparative Complement or to occur as the comparative Conjunct of a numeral
Coordination. Comparative forms and their quantitative Complements are
described in CGEL, Ch. 5 §11(d), p. 432; their count and non-count distribution
is described in Ch. 13 §4.1.2, pp. 1126–1127.

**Quantity Conjunct** (project term):
A cardinal expression or Arabic numeral used as the first Conjunct of a
Comparative Quantity Coordination, such as *three* in *three or more* or
*0* in *0 or less*. This contextual use does not itself grant the expression
other phrase functions.

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

**Central Adjunct Licence** (project term):
An Adverb's declared permission to function as an Adjunct at each licensed
position: before a lexical verb, after a copula, after an auxiliary, or in an
imperative. The positions are licensed separately under the [central-adverb-position project analysis](../../tickets/done/english-v3-central-adverb-position.md#analysis).

**Central Adjunct Position** (project term):
The declared central Adjunct site of a lexical predicate: before a lexical
verb or after a copula. Auxiliary predicates admit the post-auxiliary site
under the [central-adverb-position project analysis](../../tickets/done/english-v3-central-adverb-position.md#analysis).

**Predicative Adjunct**:
An optional predicative dependent describing a pragmatically appropriate
predicand; an *as* phrase takes a nominal Complement in this function
(CGEL, Ch. 4 §5.3, p. 263).

**Predicative Adjunct Use** (project term):
A nominal head's declared permission to occur in an *as* Preposition Phrase
functioning as a final Adjunct. Final position follows the orchestrator ruling
of 2026-10-07 recorded in the
[clause-final predicative-as ticket](../../tickets/done/english-v3-clause-final-predicative-as.md#analysis);
the declared availability is based on attested Oracle usage, independently of
selected predicative and preposed uses.

**Quantificational Adjunct**:
A fused-head partitive functioning in Clause structure to quantify the Subject;
implicit partitives include *all*, *both* and *each* (CGEL, Ch. 5 §9.2, p. 413).

**Quantificational Adjunct Use** (project term):
A Determinative's declared permission to function as an implicit-partitive
Quantificational Adjunct (CGEL, Ch. 5 §9.2, p. 413 describes the function).

**Auxiliary Use** (project term):
A verb's declared permission for auxiliary positioning, including post-auxiliary
Quantificational Adjuncts (CGEL, Ch. 3 §2.1.5, p. 102). Copular *be* retains
auxiliary distribution (CGEL, Ch. 2 §3, p. 51).

**Purpose Infinitival**:
A *to*-infinitival Clause functioning as a purpose Adjunct; an ungoverned
Purpose Infinitival is positive, and its missing Subject need not be
syntactically controlled (CGEL, Ch. 8 §12.2, pp. 728–729).
_Avoid_: Catenative Complement for an unselected purpose Clause

**Purpose Adjunct Licence**:
Project term. A verb's declared permission for its predicate to take a Purpose
Infinitival as an Adjunct. Oracle English may restrict this permission under
its recorded construction analyses, including the
[cost-scalar-complement analysis](../../tickets/done/english-v3-cost-scalar-complement.md#project-analysis)
and the [orchestrator stative-host ruling of 2026-10-08](../../tickets/done/english-v3-purpose-infinitival.md#stop-and-resolution-2026-10-08).
Auxiliary predicates inherit the licence of their selected predicate.

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

**Attribute Noun Premodifier** (project term):
A noun Premodifier whose declared use selects a head with Attribute Premodifier
Head permission, as *card* in *card type*, *card name*, and *card pool*
([orchestrator ruling, 2026-10-08](../../tickets/done/english-v3-card-noun-premodifier.md#rulings-orchestrator-2026-10-08)).

**Attribute Premodifier Head** (project term):
A nominal head's declared permission to take an Attribute Noun Premodifier;
coordination retains this permission only when each head declares it
([orchestrator ruling, 2026-10-08](../../tickets/done/english-v3-card-noun-premodifier.md#rulings-orchestrator-2026-10-08)).
_Avoid_: Card Attribute Head

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

**Quotation**:
Cited wording set off by paired quotation marks; its internal form can be an
independent utterance or a fragment (CGEL, Ch. 20 §6, pp. 1753–1755).

**Terminal Punctuation**:
Punctuation marking the end of a sentence. When a quotation ends a matrix
sentence and both would have a full stop, the matrix full stop is suppressed
(CGEL, Ch. 20 §6, p. 1755).

**Granted Ability Complement** (project term):
A selected Complement expressing cited ability wording or a Keyword Phrase.
Its Conjuncts can have different Categories while sharing Complement function
(CGEL, Ch. 11 §9.2, pp. 1026–1028; Ch. 15 §3.2, pp. 1326–1328).

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

**Coordinator Distribution** (project term):
The licensed Conjunct roles and punctuation environments of a coordination
marker in Oracle English.

**Unmarked Conjunct Licence** (project term):
Permission for an expression to begin a Conjunct after a comma without an
overt Coordinator.

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

**Clause-mate** (project term):
Of an Anaphor, another argument of the same verb: its Subject, Object or a
Complement. A Genitive's possessor ("its power") and the noun phrase of a small
clause ("counters on it") are not clause-mates of the verb. No CR rule fits;
the boundary is the owner's ruling of 2026-10-07 (`docs/decisions/semantics-v2.md`
§7), recorded there as provisional.
_Avoid_: co-argument for possessors or small-clause objects

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

**Determiner Agreement Number** (project term):
The Number a singular-selecting Determiner fixes for its Noun Phrase even
when its Nominal head is coordinated. Other Determiners allow Nominal
Agreement Number through ([owner ruling, 2026-10-08](../../tickets/done/english-v3-coordinated-subject-agreement.md#analysis));
CGEL, Ch. 5 §18.4(a), p. 508 [28] describes singular agreement for
coordinated NPs containing distributive *each* or *every*.

**Subject Relative Number** (project term):
The Number required by a Subject-gap Relative Clause on a coordinated Nominal,
resolved against the containing Noun Phrase's Agreement after its Determiner
applies ([owner ruling, 2026-10-08](../../tickets/done/english-v3-overgeneration-prune.md#d-a-relative-clause-on-a-coordinated-nominal-agrees-plural-even-under-each)).

**Nominal Agreement Number** (project term):
The Number a Nominal contributes to its containing Noun Phrase's Agreement,
which can differ from the coordinates' Number used for shared Determiner
selection. The additive plural restriction in Oracle English is the
[coordinated-subject project analysis](../../tickets/done/english-v3-coordinated-subject-agreement.md#analysis).

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
present, preterite, gerund-participle, past participle, or irrealis. It is distinct
from Finiteness and from Agreement, though those dimensions constrain its
selection.
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

**Targeting Premodifier Use** (project term):
A Modifier's declared permission to precede a Nominal containing a Targeting
Marker. This Oracle-English licence preserves the project's existing colour
modifier restriction while admitting the attested *other target creatures* and
ordinal compositions; its evidence and limits are recorded in the
[each-of landing](../../tickets/done/english-v3-each-of-partitive.md#landing-record).

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
The expression a parameterized Keyword Ability takes as its quality argument, on a keyword
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

**Bound Keyword Stem** (project term):
The noun stem forming a Keyword Quality bound to a Bound Keyword Surface, as
*island* in *islandwalk*, derived from the keyword's declared stem class
(orchestrator ruling, 2026-10-08). Its lowercase form comes from the noun's
declared singular Word Form, with positional Capitalization of the combined
word (owner ruling, 2026-10-08,
[landwalk ticket](../../tickets/done/english-v3-landwalk-bound-suffix.md#analysis)).

## Status, Designation (Game Model terms — not grammatical categories)

`tapped`, `goaded`, `suspected`, `saddled` are the **Participles** of their
keyword-action verbs, used attributively or predicatively; `monstrous` is an
**Adjective**; `the monarch`, `the initiative` are **Noun Phrases**; `face down`
is a lexicalized **Preposition Phrase** (analysis adopted 2026-10-04,
superseding this part of the 2026-09-05 ruling). Whether such a form names a
status [CR#110.5] or a designation [CR#701.15b] is Game Model semantics resolved through the verb
declaration, never a grammatical class. _Avoid_ "Status", "Designation" as the
name of a vocabulary, codec, or construction in the grammar (ruling 2026-09-05).
