# Independent lexical analysis and retained grammatical readings

Amendment (2026-10-03): the standalone English Lean project is retired. All English-specific Lean prerequisites in this decision and existing tickets are superseded. Validate grammar, correlation, admission and roundtrip behavior through Rust tests and corpus evidence. The linguistic and retained-Reading requirements below remain in force.

Accepted 2026-09-07 after the rewrite planning dialogue and amended after the
first integration exposed the cost of fitting retained Readings into v2. Build
a fresh `english_v3` grammar and parser, salvaging useful implementation and
evidence without preserving v2 interfaces. English remains an independent NLP
project with no interaction with Semantics. The
[wayfinder](../english-grammar-wayfinder.md) names the implementation work.

## Authority and retained work

This decision supersedes conflicting production prescriptions in
[the original rewrite](english-v2-rewrite.md),
[the Lean migration](english-lean-design-workbench.md), and
[the grammar design](../english-grammar-design.md). Their source hierarchy,
Vintage support scope, linguistic requirements and authentic regression
witnesses survive. Old coverage figures describe their measured trees only.

Keep bidirectional construction declarations: one declaration drives generated
parsing rules, checked construction, rendering and traversal. Per-construction
handwritten renderers are not an alternative implementation. Build a fresh
compiler core and thin proc-macro shell for v3 rather than adding a retained-
Reading backend to the v2 compiler. The old compiler supplies candidate syntax,
diagnostics and generation techniques only; v3 owns its compiler IR, generated
types and runtime contract without compatibility adapters.

Salvage catalogs, lexical identities, declared forms and frames, immutable
indexes, source/normalization evidence, corpus subset/report/inspection tools,
and independent Lean witnesses. The generic Earley scheduler and packed forest
are reuse candidates. The archived verb-frame/breadth Rust stack is unfinished;
extract useful declarations and tests individually, rather than transplanting
it or counting its grammar families as current production.

## Construction economy

Amendment (2026-10-04): the completed grammar covering all supported cards has an
ultimate ceiling of 250 Constructions and 2,800 well-formatted declaration lines;
fewer is preferred. These are final breadth ceilings, not budgets to consume at
the current coverage level. Grammar work should normally fix or refine existing
Constructions; a new Construction requires a clear grammatical distinction.

Share bidirectional construction schemas across Categories and reuse constraint
policies where appropriate. Schema instances may specialize selected Categories
and feature equations without becoming separate named Reading variants. Count
shared schemas, ordinary Constructions, category instances and compiled
Productions separately; hiding hundreds of variants behind an authoring macro
does not satisfy the economy requirement. Keep grammatical distinctions,
category/feature correlation, authentic witnesses and both roundtrip laws.

## Lexical analysis

Analyze input independently of construction-directed scanning. Each alternative
identifies a declared Lexeme, its lexical Category, Word Form, correlated
grammatical features, and declaration/catalog provenance. Inflectional Form,
Person, Number, Tense and Finiteness remain distinct concepts; absence or
inapplicability of a feature is not permission to invent its values.

Analyses come only from declared vocabulary and its default or overridden
morphology. There is one default rule per applicable inflection; an explicit
irregular override replaces that default for the inflection, and multiple
valid alternatives are explicitly declared. Rules apply within a declared
lexical category. Unknown words and unlicensed POS/derivational readings are
reported as vocabulary gaps, never guessed. Initial overgeneration means
retaining all these licensed alternatives until grammar constrains their use.

Lexemes also declare grammatical properties, including count/mass uses and
selected Complement frames. Grammar applies general composition constraints
to these properties; morphology does not parse the surrounding phrase. Core
vocabulary, plugin declarations and catalogs feed one analysis interface even
when their authoring sources differ. Lexical inflection generation used for
analysis and realization must share the same defaults and overrides.

The lexical ticket pins the remaining representation choices against actual
inputs: atomic token boundaries, bound and overlapping multiword forms,
capitalization, feature applicability, and the exact default spelling rules.
The decision to use one default does not yet prescribe literal suffix append
versus an algorithm with ordinary orthographic cases. Record the chosen
algorithm and its overrides; do not silently introduce guessed paradigm classes.

Amendment (2026-10-04): type and subtype declaration macros supply noun
Premodifier eligibility and negative-prefix joining once per source class.
Noun premodification is a function of the existing noun, not a duplicate
Adjective Lexeme. Negative noun Lexemes retain their source noun's declared
inflectional paradigm, including overrides and unavailable plurals; their
Provenance names that source declaration. The class declares joined forms for
types (*nonartifact*) and hyphenated forms for subtypes (*non-Vampire*).
Supertypes have a declared adjective-class recipe with joined negatives
(*nonlegendary*); its members retain their existing positive lexical owners.
These are lexical formation declarations, not spelling-based admission guards.

## Source and roundtripping

Prefer a lossless token sequence with positions derived from occurrences and
surface lengths. Exact token/separator treatment is refined in the lexical
ticket. Position, lexical identity and declaration provenance are separate;
discarding redundant span storage does not discard the latter two. Reuse
existing ownership checks for their useful invariants, not to preserve their
entire representation. No source-buffer echo or opaque catch-all leaf counts
as grammatical realization.

With the same parse context and lexical environment, both laws are required:

```
v is an admitted reading of s  =>  render(v) = s
v is independently constructed and grammatically valid
                              =>  v is a reading of parse(render(v))
```

Equality in the second law preserves grammatical structure, lexical identities,
features and explicitly retained spelling variants. Computed source positions
are not grammatical identity. Every non-derivable surface distinction admitted
by the grammar must be recoverable by the declaration-driven renderer; the
lexical integration pins how it is represented. Keep raw source and any
preexisting normalization distinct. The supported corpus requires Vintage
playability (legal or restricted) and excludes Scryfall `set_type: funny`.
Remove balanced parenthesized reminder text before lexical analysis; reminder
contents are outside the current parsing scope. Retain the original Oracle
text as source metadata, record the stripped parser input separately, and apply
both roundtrip laws to that input. Preserve card-name spellings; legendary
permanent nicknames are reversible lexical variants of the full name.
Tests starting from independently constructed values remain required alongside
corpus parse/render checks.

Amendment (2026-10-04): grammatical game-semantic nonsense remains admissible,
but do not make invented nonsense instructions positive regression contracts.
Use real Oracle passages or attested constituents, recording their card and
context. A fragment occurring on cards does not make an invented standalone
effect an appropriate witness. Independently construct the corresponding values
and preserve structural assertions; this witness policy adds no semantic filter
to admission.

## Chart admission and ambiguity

Amendment (2026-10-04): Construction Costs may rank a preferred presentation
while retaining all grammatical Readings. Default cost is 1, declarations may
assign nonnegative overrides, and a Reading's total counts its Construction
occurrences. Costs do not decide admission, change Reading identity, prune the
forest or replace the complete ambiguity census. Limited enumeration can rank
only its observed Readings. This permits an expensive generic label fallback
without confusing preference with grammatical validity.

Retain Earley-family chart parsing. The lexical layer supplies alternatives;
the grammar composes Categories, lexical frame requirements and feature/
dependency constraints. Chart completion must operate on sufficient grammatical
summaries or explicit constraints, without constructing Cartesian products of
full AST child values. Generated declarations supply those checks. Full AST
construction happens when a reading is requested from the forest.

Packing preserves every distinction that can affect a parent's admissibility
and keeps alternatives correlated. It applies to incomplete derivation
histories as well as completed constituents; carrying one explicit child vector
per partial Earley item merely postpones the Cartesian product and does not
meet this requirement. A feature-sensitive key is a candidate implementation,
not a proof that an arbitrary bundle is sufficient. Summary equivalence means
that every possible parent treats the packed alternatives alike. Open
dependencies must remain represented until their context is available. Any
deferred validity check must participate in the definition of an admitted
Reading: an unchecked forest path is not a successful grammatical analysis.

The primary result retains all admitted readings. Multiple grammatical readings
are successful parsing, including unrelated homographs and different scope
classes. Preference is an optional, non-destructive view. A representative may
aid display but cannot remove alternatives. Equal text does not equate readings;
distinct derivations of the same intended structure are not automatically
distinct linguistic readings. Test that distinction explicitly.

This replaces the hard-error tie rule, global structural-specificity arbitration
as admission, and the requirement to fit all retained ambiguity into one
hoisted representative with derived site/anchor rows. Existing local scope
classes and their invariants remain useful relationships within the complete
reading set. Neither a specific alternative-subtree AST nor a general new
forest implementation is mandated; exact recoverability is the contract.

## V3 sequence and evidence

Update the independent Lean English workbench first so it accurately describes
the v3 lexical relation, connected grammar, admitted Readings, ambiguity
correlations and relational roundtrip obligations. Lean does not model Earley
scheduling or prove the Rust implementation, but Rust must not knowingly build
against a superseded grammatical judgment.

Extract a small data-only lexical model, then harden the chart so incomplete and
complete derivations are packed. Build the fresh construction compiler against
those interfaces and exercise ambiguous output and both roundtrip laws through
one generated interacting slice. Measure lexical alternatives, chart items,
intermediate and completed nodes, forest families, completion work and requested
materializations.

After that slice, translate the complete intended grammar top-down and activate
it against the supported corpus as one interconnected machine. Every planned
family must have real general productions and cross-family use, while incomplete
lexical inventories and uncommon variants may remain named residuals. Do not
resume a family-by-family or card-by-card migration before this activation.
Permissive catch-alls, unused declarations and opaque source leaves do not count
as breadth.

Run tokenization/analysis and the word inventory on the supported corpus early.
Account separately for catalog, keyword and notation material so a numeric
coverage count cannot conceal lexical gaps. Use the existing whole-grammar
family map and obligation register as the input to the complete declaration.
Group the resulting failures by shared lexical, grammatical, compiler or parser
cause. Repair those systemic causes before the long-tail handoff; mint later
residual tickets from the measured report rather than from predicted card
families. The handoff is an adaptable judgment without a fixed coverage target
or a claim that no general grammar gap can remain.

The landing contract changes only where the accepted ambiguity/position design
requires it: report no-reading failures, unique readings and multiple readings
separately; retain exact-source, lexical identity, structural traversal and
zero-internal-failure checks. The old zero-unresolved-ties requirement is
superseded. A newly admitted wrong reading is still a defect, even if a correct
reading also survives. Preserve the no-silent-loss identity accounting and
classify removed wrong analyses separately from regressions. Optional preference
has its own disclosed effects and does not determine parsing coverage.

## Lean and remaining uncertainty

Make the lexical-analysis relation constrain actual admission, preserving the
restored word-form counterexamples as evidence until their defect is repaired.
Model admitted readings across classes independently of preference; demonstrate
lost-correlation exclusions using inhabited grammatical examples. State the
relational roundtrip obligations without making them true merely by definition.
An executable Lean parser or proof of the Rust engine is not scheduled.

The v3 Lean model and proof audit handle those interfaces and retain the
substantive proof-gap obligations, including flat coordination, tense and
countability. Neither an external review nor a desire to defer work authorizes
declaring Oracle English tense-neutral. Lean models the linguistic relations
and the information the implementation must preserve; a formal Earley or SPPF
implementation is outside its scope.

The main engineering uncertainty is the size of the future-admissibility state
and the compression achieved by packing partial derivations. The chart and
compiler tickets measure those costs before whole-grammar activation. Their
results may change the representation or evaluation strategy, including making
lexical lookup lazy behind the same independent interface; they do not return
morphology to construction-directed scanning or restore eager AST admission.

## Amendment: Preposition Phrase functions are licensed per preposition (2026-10-06)

Ruling (user, 2026-10-06). In Oracle English a Preposition Phrase's grammatical
function is licensed per preposition, as a declared lexical property the
grammar reads, never as a guard naming the word. *during* functions only as a
clause or VP Adjunct, never as a Noun Phrase postmodifier. *of* functions only
NP-internally (Modifier or Complement of a noun, including discontinuous and
predicative uses), never as a clause Adjunct and never as a verb-selected
Complement.

This is a register restriction of Oracle English, deliberately narrower than
CGEL's general-English licensing: CGEL admits a *during* postmodifier, as in
"his behaviour during the turn" (CGEL, Ch. 5 §14.2, p. 446, [14iii]). The
functions a PP can take (adjunct, complement, modifier, predicative) are those
of CGEL, Ch. 7 §2.1, pp. 604–606; "adverbial" and "adjectival" PP are
category–function confusions (CGEL, Ch. 7 §2.2, p. 617, n. 3), so the licence
is a set of functions, not a kind of PP.

Corpus evidence, measured 2026-10-06 over 32,875 supported faces with reminder
text stripped (evidence, not a gate):

- *during*: 909 tokens, none a noun postmodifier ("each combat during",
  "turn during", "mana during": 0 each). About 150 verb-object + *during*
  tokens all attach to the VP ("cast a spell during an opponent's turn" 47,
  "untap this creature during your untap step" 26); the 3 "for the first time
  during …" tokens scope the clause.
- *of*: 17,681 tokens, none verb-selected (*consist*/*rid*/*think*/*die of*: 0)
  and none a clause Adjunct. NP-internal shapes that must keep parsing:
  predicative "creatures that aren't of the chosen type" (6); discontinuous
  modifiers "Spells you cast of the chosen type" and "one of them of their
  choice" (about 12, plus about 90 "of your/their choice" after a verb
  Object); fixed "instead of" (about 100).

Supersession. This ruling supersedes the 2026-10-05 three-attachment ruling
for Seedborn Muse ("Untap all permanents you control during each other
player's untap step."). Its attachments are now two: to *Untap*, and to
*control* inside the relative clause. The nominal attachment to *permanents
you control* is a wrong Reading to be retired.

Open question, not ruled: "At the beginning of combat on your turn" (319
cards). *on* after a noun is plausibly either a restrictor or clause scope and
needs its own decision before implementation changes *on*.

Implementation principle: each preposition declares one licence feature
listing its permitted functions (Adjunct, NP Modifier, noun Complement,
verb-selected Complement, predicative Complement). The free-adjunct and
postmodifier rules and selected frames read that feature. The 2026-09-02
"adjunct-capable / selected-only" split is a special case of it. The existing
feature `AdverbialUse` is the adjunct-capable licence under a confusing name;
implementation renames it (the identifier follows the glossary's **Preposition
Function Licence**, never the reverse). Implementation is owned by
[english-v3-selected-preposition-nominal-licensing](../tickets/planned/english-v3-selected-preposition-nominal-licensing.md).

### Pruning rule for licensed functions and attachments

A licensed preposition function, attachment site or lexeme is excluded only
when the supported corpus (the Vintage-legal faces `cargo xtask english-v3
--all` reads) shows zero attested uses and applying the exclusion loses zero
faces in the full-corpus run. Both conditions are measured, not argued.

- An attested but undecidable attachment stays licensed. Seedborn Muse's
  *during each other player's untap step* attaches at two positions; ambiguity
  the corpus does not resolve is preserved as multiple Readings, never pruned
  to one.
- A rare but attested use keeps its function licensed, however few faces use
  it. Frequency never justifies exclusion; only absence does.
- An exclusion whose replacement analysis does not exist yet is deferred: the
  function stays licensed until the prerequisite lands, and that landing
  re-applies the exclusion. The 2026-10-06 licensing landing recorded seven
  such deferrals; their owners are listed in
  [english-v3-systemic-residuals](../tickets/planned/english-v3-systemic-residuals.md).
- The same rule governs lexemes: one with zero tokens in the supported corpus
  is deleted
  ([english-v3-dead-lexeme-audit](../tickets/done/english-v3-dead-lexeme-audit.md)),
  one with any token stays.

Rationale: overgeneration is tolerable and the grammar prunes toward what
Oracle text attests, but a wrong exclusion is a silent coverage loss, which the
landing contract treats as a defect.
