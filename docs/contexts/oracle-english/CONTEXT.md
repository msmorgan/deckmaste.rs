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

**Lexeme**:
An abstract lexical item underlying its inflected Word Forms.

**Lemma**:
The conventional citation form used to identify a Lexeme.

**Word Form**:
An inflected surface form of a Lexeme.

**Realization**:
The relationship by which an abstract grammatical value is expressed in a
surface form.

**Linearization**:
The ordering of a Construction's constituents into a surface sequence.

**Sentence**:
The highest ordinary syntactic unit of Oracle text, terminated by sentence
punctuation and containing one or more Clauses.

**Clause**:
A syntactic unit organized around a predication and its dependents.

**Predicate**:
The grammatical function in a Clause that predicates something of the Subject.
This is distinct from a Game Model Predicate.

**Verb Phrase**:
A phrase headed by a verb.

**Bare Predicate**:
The Predicate shape available where no finite tense is expressed, such as the
complement of a modal auxiliary or of a causative verb.

**Lexical Verb Phrase**:
An instantiated lexical verb together with the complements it selects, before
higher auxiliary and clause structure is added.

**Verb Frame**:
An ordered lexical schema describing the complements and fixed markers a verb
licenses.
_Avoid_: Verb Phrase for the schema; Verb Frame for an instantiated phrase

**Nominal**:
An intermediate nominal constituent headed by a noun or noun-like element.

**Noun Phrase**:
A phrase whose head or fused head is nominal and which can fill functions such
as Subject, Object, or Complement of a preposition.

**Determinative**:
A lexical Category containing words such as *the*, *this*, *each*, and *any*.
It is distinct from the Determiner function.

**Determiner**:
The grammatical function that marks a Noun Phrase as definite, quantified, or
otherwise determined. A Determinative commonly realizes this function.

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

**Adjunct**:
An optional dependent that modifies a phrase without being selected as its
Complement.

**Duration Phrase**:
An Adjunct that bounds how long the effect of its Clause lasts, such as *until
end of turn*.

**Modifier**:
A dependent that attributes or restricts the interpretation of its head.

**Postmodifier**:
A Modifier that follows its head in the linear order.

**Coordination**:
A construction joining two or more coordinate units.

**Conjunct**:
One of the units joined in a Coordination.

**Coordinator**:
The marker that links Conjuncts, such as *and* or *or*.

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

**Inflectional Form**:
A morphological form of a Verb Lexeme, such as plain, third-person-singular
present, preterite, gerund-participle, or past participle. It is distinct from
Finiteness and from Agreement, though those dimensions constrain its selection.
_Avoid_: Verb Form when the specific morphological dimension is meant

**Onset** (project term):
The v2 compiler feature classifying a realized expression as consonantal or
vocalic for *a*/*an* selection, computed at the pronunciation boundary and
frozen as data on normalized rows; orthography is a fallback, never onset
authority.

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
