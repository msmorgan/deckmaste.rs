# Builtin-v2 macro spelling and grammar

> Workbench succession (2026-09-05): current workbench evidence lives in
> `lean/`; Idris paths below are historical. See
> [Lean is the workbench](lean-is-the-workbench.md).

Amended 2026-08-18: verb grammar declarations carry grammatical valence, and
the parser consumes declaration-backed vocabulary through one normalized open
inventory during both bootstrap and final plugin loading.

Packaging superseded 2026-09-02: these declarations are ordinary
`macro_ron::MacroDef<Metadata>` values read through the shared `MacroSet`;
`KeywordAction`, `Subtype`, and the other category forms are meta-macros in
that reader. The `spelling`/`grammar` payload and normalized rows are owned by
`deckmaste_construction_core`. There is no separate v2 source schema, source
kind enum, reader, or RON dialect. All field semantics below remain unchanged.

## Decision

Plugin macros keep semantic spelling and English-parser bootstrap data in two
separate fields:

- `spelling` is an uninflected, holey phrase shape used to match parsed
  English and assemble a semantic card.
- Optional `grammar` contributes the lexical category and whole realized
  surfaces the English parser needs before that match can happen.

The fields have different authorities: `spelling` selects a semantic macro;
`grammar` only helps produce an English parse. Neither field supplies the
other's missing information.

For example, a graduated Scry declaration has this shape (the semantic body is
omitted here):

~~~ron
KeywordAction(
    name: "Scry",
    params: [Amount],
    spelling: "scry <Param(0)>",
    grammar: Verb(
        bare: "scry",
        third_person: "scries",
        valence: Numerative,
    ),
)
~~~

`X` can fill `Param(0)` because it is an `Amount` surface, just as a literal
number is. Whether that `X` is bound and available is a semantic-validation
question, not an English-grammar question.

This decision supersedes the spelling-stub ticket's earlier candidate in two
specific ways: grammar is no longer an enum variant inside `spelling`, and the
restricted `<Param(n)>` notation is intentionally carried by the spelling
string. It does not reinstate v1's `template:` language, text offsets, or
`kinds:` registry.

## Positional parameters are the serialized ABI

Macro parameters are positional only. `Param(n)` is the one spelling for a
parameter reference in `spelling`, semantic bodies, guards, and any later
grammar-side reference. A macro declaration has a positional type vector, not
named formal parameters.

This makes realized RON structurally translatable to the Idris validator:

~~~text
RON:   M(a0, a1, ..., an)
Idris: M ⟦a0⟧ ⟦a1⟧ ... ⟦an⟧
~~~

There is no declaration-order lookup, name-to-index environment, or named
argument elaboration between those forms. Human-friendly role names may exist
in documentation or generated diagnostics, but they are not serialized
references and cannot affect validation.

## What `grammar` may declare

`grammar` carries the grammatical facts needed to admit the macro's English:
for example, that `scry` is a numerative verb, the realized inflections of a
compound verb, a noun's attested number forms, or a fixed clause/keyword
surface. Whole realized compound surfaces are data; there are no text offsets
or head-position markers.

The only productive morphology recipes are deliberately dumb:

- `english_verb` realizes the bare form as its lemma and third-person singular
  as the lemma plus one ASCII `s`;
- `english_noun` realizes singular as its lemma and plural as the lemma plus
  one ASCII `s`.

An omitted derived-form field selects that default. An explicit string is a
replacement, not an additional alias: `third_person: "scries"` removes the
derived `scrys`, and an invariant `plural: "Merfolk"` removes the derived
`Merfolks`. Whole compound forms are likewise explicit whole-string
replacements. A replacement equal to the default is rejected as redundant;
there are no `-es` or `-ies` heuristics and no compound-head discovery. A
record can explicitly mark a derived form unavailable when there is no
authoritative attestation. Plain omission does not mean unattested, because it
selects the default recipe. Explicit unavailability asserts the form is not
well-formed English (a defective paradigm), never merely that it is unprinted;
the grammar admits every recipe-derived regular form regardless of whether any
card realizes it. Attestation gates which facts may be DECLARED — it is
provenance for overrides, never a corpus filter on what the declared grammar
admits.

Catalog membership still does not establish that a form is attested. Authors
must check supplied replacements and explicit unavailability against the
authoritative grammar sources. Catalogs never supply morphology to the
runtime.

Every realized form also has one effective `Onset::{Consonant,Vowel}`, a
v2-owned sealed compiler feature emitted like `Number`. The v2 normalization
path owns the bounded pronunciation recipe and accepts an optional per-form
override when the form's attested pronunciation requires one or the recipe
cannot decide. That override is reviewed, authored stub data backed by
attestation. It is never inferred from corpus acceptance, a catalog, or a
parser exception, and a first-character helper is only the recipe's
orthographic fallback. A spelling the recipe cannot classify and that lacks an
authored, attested override is a normalization error; normalization never
guesses an onset.

Every `Verb` declaration stores one closed grammatical valence:

- `Intransitive` admits no direct complement, as with `explore`;
- `Transitive` admits one ordinary object noun phrase, as with `destroy`;
- `Numerative` admits one amount expression, as with `scry`; and
- `Custom` carries one or more explicit VP-tail shapes over the exact atom
  inventory below, as with the empty or amount-bearing tails of `connive` /
  `connive N`.

The serialized `Custom` tail atom inventory is closed and finite:

- `Literal(String)` matches one nonempty exact terminal span;
- `Amount` admits the ordinary amount category;
- `ObjectNounPhrase` admits an ordinary object-position noun phrase, where
  "object" is a grammatical role rather than the Magic rules term; and
- `PredicativeComplement` admits the ordinary predicative-complement
  category.

A custom shape is an ordered list of those atoms. The shape set must be finite
and nonempty; an empty list is a valid individual shape for a no-tail
alternative, while an empty shape set is invalid. Duplicate alternatives,
empty literals, and any atom or nesting operator outside this list are load
errors. Optionality is represented by listing alternatives, not by an
optional or repetition operator. Adding another category is a reviewed
compiler/ADR change, not plugin data.

An atom may carry a compiler-only label when the same atom category occurs
more than once in one shape. Labels distinguish positions during source
validation and are erased before normalized `VerbFrameKey` matching. An
unlabelled repeated atom, a repeated label, mixed labelled and unlabelled
occurrences of the same atom, and a labelled literal remain hard errors. No
label reaches the AST, parser, renderer, claims, or normalized declaration
rows. Flat tails are limited to lexical subcategorization: a verb-selected
preposition such as *search ... for* may remain in its verb frame, but a
preposition phrase with independently compositional meaning is a shared
construction instead of a duplicated verb-local tail.

`Custom` is not an arbitrary plugin grammar production and does not accept an
untyped tail: it cannot add recursion, precedence, callbacks, or semantic
predicates. Its finite tail-shape set is declaration data compiled through the
same generic extension point as the three ordinary valences. A missing, empty,
or invalid custom shape set is a load error. Conversely, a fixed clause such
as `the Ring tempts you` uses the fixed clause recipe and is not mislabeled as
a `Custom` verb.

Valence is grammatical structure, not the semantic parameter signature or a
Magic-legality declaration. `Numerative` is enough for the English grammar to
recognize both `scry 1` and `scry X`; `Transitive` admits an object noun phrase.
The grammar therefore rejects the basic crossed shapes `destroy 1` and `scry
each creature target opponent controls`. It can still overgenerate within a
valence family. Typed spelling-frame matching must recover the declaration's
positional parameter types from the parsed subtrees, and Idris must validate
the assembled semantic card afterward. Neither valence nor the parser may
encode the macro's semantic guards or rules legality.

The contribution vocabulary is not a plugin-extensible grammar DSL. New
lexemes and their inflections are open data; new grammatical recipes remain
reviewed compiler work.

Attachment participles are a `grammar` contribution class (ruling
2026-08-24): `equipped`, `enchanted`, and `fortified` occupy the determiner
slot and license a bare noun ("equipped creature", never "*the equipped
creature") [CR#301.5f,303.4m]. They form an open determiner family in the
parser environment, contributed by the attach mechanic's keyword-ability
macro (Equip, Enchant, Fortify — a custom "Enthrall" contributes
"enthralled" the same way), consumed by ONE generic reference-NP core
construction — never one construction per mechanic. The participle surfaces
are declared whole and attested (equipped doubles the p, fortified mutates
the y; only enchanted is regular append-ed), and the referent (the attached
object) is frame-layer context resolution, not parser knowledge.

## One normalized parser boundary

All declaration sources normalize into one immutable grammar inventory before
the scanner, parser, renderer, diagnostics, or construction machinery sees
them. Those consumers accept only the normalized inventory/environment. They
must not read a catalog, walk plugin files, distinguish builtin from third
party, or consult a second static vocabulary table.

Each normalized row carries:

- a category-safe open identity, so a keyword action, keyword ability,
  category-scoped subtype, type, counter kind, and designation cannot be
  confused even when their spellings or local names coincide;
- its closed recipe and complete realized surface rows, each with the frozen
  effective onset as data and the optional authored, attested per-form onset
  override that produced it;
- the declaration identity and source provenance needed for diagnostics; and
- for verbs, the required valence and any validated `Custom` tail shapes.

The registry IDs are open carriers such as checked owned or interned IDs, not
one exhaustive Rust enum variant per registered member. Closed grammatical
recipes may remain enums. Runtime indexes are compiled from the normalized
rows in both directions: surface to every compatible reading, and identity
plus features to the one render surface. A surface collision retains all
readings for ordinary ambiguity handling; registration order, catalog order,
and filesystem order never select one.

Exact-name catalogs cross this boundary only as explicit typed identity
providers. Generated grammar metadata lists every required named provider;
the environment freezes those rows and fails on a missing or duplicate
provider. xtask is the sole production adapter that reads
`deckmaste_catalogs` and supplies canonical card-name rows. Scanner, parser,
renderer, diagnostics, and construction consumers never read a catalog and do
not acquire an alternate vocabulary authority.

The grammar head and semantic spelling frame are coherent by construction and
bound to the enclosing declaration identity. For a verb, the initial literal
head in `spelling` must name that declaration's own bare grammar surface; each
other fixed recipe applies the corresponding identity-local check. A record
whose grammar declares `mill` but whose spelling begins
`scry <Param(0)>` fails to load. The compiler never repairs this by looking
up a matching global surface, and homonymous declarations remain distinct
identities.

Static semantic frames such as the current Destroy and Connive constructions
may survive during bootstrap, but they request an open declaration identity;
they do not define that identity or own its surface table. A nursery grammar
record with no typed signature or semantic body participates in parsing and
rendering but is not selectable as a semantic spelling frame.

## Bootstrap and final authority

The Stage 5 parser may be brought up corpus-first with handwritten
definitions, but this does not create a second authority or a closed official
vocabulary. Bootstrap records live at their final
`plugins/builtin_v2/macros/...` addresses and use consumer-owned
macro-definition metadata. A narrow builtin-only provider normalizes those
records into the same immutable inventory used by the parser. There is no
generated stub manifest and no later content migration. These handwritten
records, not a catalog-derived fallback, own the bootstrap grammatical facts.

The final provider resolves the complete plugin dependency closure and feeds
the same normalization boundary. It replaces the narrow bootstrap reader as a
provider, not the normalized row contract or parser ABI. At that point the
resolved `builtin_v2` plugin declarations are the builtin runtime authority,
and dependency/plugin declarations can add or shadow same-kind identities
under the ordinary loader rules. The canonical catalogs remain read-only
completeness checklists for the builtin source tree; neither bootstrap nor the
final provider turns them into runtime vocabulary, casing, valence, or
morphology authority.

Runtime efficiency comes from immutable compiled indexes over the open rows,
not exhaustive switches or a generated construction per catalog member.

## Open plugins parse their own English

The parser is built from the core grammar plus the grammar contributions from
the complete plugin dependency closure. Loading an English-authored plugin is
therefore phased:

1. Read macro headers and their `grammar` contributions from the dependency
   closure, including the plugin being loaded.
2. Build the English parser environment from core declarations plus those
   contributions.
3. Parse that plugin's English-authored cards.
4. Match the parsed English against reachable `spelling` frames and assemble
   semantic RON.
5. Run ordinary semantic and Idris validation.

Consequently, the official card corpus is a provider and conformance corpus,
not the parser's vocabulary boundary. A custom plugin can introduce a verb in
a macro and use it immediately in English-authored cards in that same plugin.
No author must first express those cards in the semantics language.

## Registry scope

The examples that motivated this decision are not its boundary. The boundary
is the plugin's declaration-backed vocabulary: if a loaded registry row can
name a semantic construction that appears in English, that declaration owns
its `spelling` and optional `grammar` contribution. In the current plugin
model this includes:

- keyword-action verb declarations;
- keyword-ability declarations;
- subtype declarations in every supported subtype category;
- card-type declarations;
- counter-kind declarations; and
- designation declarations.

This is what keeps the sets genuinely open. A same-plugin English card may use
a newly declared subtype, type reference, counter kind, designation, keyword,
or verb without that name first being admitted to an official-corpus table or
hardcoded into the parser. Catalogs seed and check the builtin inventory where
an authoritative complete catalog exists; they do not define the vocabulary
ceiling.

Not every English vocabulary is such a declaration. A card's own name used as
source self-reference denotes that particular object, and an approved
shortened printed name is equivalent [CR#201.5,201.5c]. The grammar keeps that
bare name as a per-parse context identity and stores only its full/abbreviated
choice. An independently mentioned exact card name is a typed catalog identity
admitted only inside an explicit name-bearing construction such as `a card
named Seven Dwarves`; it is never a rival bare noun phrase. Ability words
belong to a surface-label preservation path: they have no special rules
meaning [CR#207.2c] and do not select a semantic macro by the label alone.
Supertypes remain generated closed structural vocabulary, never a catalog or
open declaration domain.
Non-keyword game-action verbs such as deal, gain, and draw likewise remain
closed core grammar.

The subtype rule applies to all categories the semantic model can currently
represent: artifact, battle, creature/kindred, enchantment, land,
planeswalker, and instant/sorcery spell subtypes [CR#205.3]. Planar and dungeon
subtypes are explicit model gaps: the current subtype category axis cannot
carry them. They are not silently counted as covered, and adding either
category brings its declarations under this same rule.

## Nursery records and catalog integrity

`plugins/builtin_v2/macros` is the nursery and future address of these
macros, not a parallel registry. A stub carries the literal head shape and the
grammar contribution now; graduation adds positional parameter types, extends
`spelling` with `Param(n)` holes, and adds the semantic body in place.

The canonical keyword catalogs and every currently representable subtype
catalog back category-scoped directories:

These are the CR-derived catalogs under `data/gen/catalogs`. Legacy
Scryfall-derived vocabularies are not classification or completeness authority
for this design.

| Catalog | Current entries | Stub directory |
|---|---:|---|
| `keyword-actions.txt` | 70 | `keyword_actions/` |
| `keyword-abilities.txt` | 195 | `keyword_abilities/` |
| `artifact-types.txt` | 22 | `subtypes/artifact/` |
| `battle-types.txt` | 1 | `subtypes/battle/` |
| `creature-types.txt` | 324 | `subtypes/creature/` |
| `enchantment-types.txt` | 13 | `subtypes/enchantment/` |
| `land-types.txt` | 17 | `subtypes/land/` |
| `planeswalker-types.txt` | 80 | `subtypes/planeswalker/` |
| `spell-types.txt` | 5 | `subtypes/spell/` |

Each catalog line maps to exactly one `.ron` file. Spaces and hyphens start a
new PascalCase word; apostrophes and terminal exclamation marks are removed.
The sole counted non-ASCII exception is `∞` → `Infinity`. Thus, for example,
`Power-up`, `C'tan`, and `For Mirrodin!` map to `PowerUp`, `Ctan`, and
`ForMirrodin`. `name` equals the filename stem. Collisions, unsupported
punctuation, missing files, and extra files are hard errors. Identifiers are
category-scoped; identical names in different stub directories do not collide.

The stub and declaration files are committed, hand-maintained source code.
There is deliberately no stub generator: graduation edits these same records
in place, so regeneration must never overwrite authored grammar or semantic
bodies. A read-only validator may enforce the catalog-line ↔ filename
bijection, but it must not inspect, synthesize, or prescribe record contents.
The source catalogs remain pure word lists as required by the English-v2
decision; the committed records are the per-entry constructions that carry
grammar.

That bijection applies only to catalogs that are complete inventories for the
corresponding builtin directory. In particular,
`counter-kind-phrases.txt` contains only the CR-enumerated multi-token keyword
counter phrases [CR#122.1b]; single-token counter kinds are productive and the
file is not an inventory of `Counter` declarations. Designations have no
central CR catalog. The card-type catalog includes types the current semantic
type-line model does not support. Counter, designation, and type declaration
coverage must therefore be checked against their actual builtin declaration
manifests and loader invariants, never by pretending one of those word lists is
a closed universe.

The canonical catalogs can grow or shrink; contributors add or remove the
corresponding committed records. A new exceptional spelling, morphology,
punctuation rule, or mapping collision requires reviewed authored changes
rather than a silent guess.

## Scope boundary

Declaration-backed names and phrase shapes come from plugin macros. The
English-v2 slice's hardcoded Destroy and Connive lexemes are scaffolding only
until this macro seam is consumed.

This decision lands no declarations, validator, loader, grammar change, or
runtime consumer. Those changes are follow-up work. The stage-5 buildout must
consume open grammar contributions at the seam above; it must not mint one core
construction per official keyword.

## Stage 5 implementation relay

The Stage 5 grammar-buildout workspace has completed Tasks 1 through 3. Before
starting Task 4, refresh it onto this amendment and revise its local plan as
follows. Do not reopen the completed tasks unless one of these requirements
exposes a concrete incompatibility.

- Task 4 must give generated terminals an open, category-safe declaration
  identity carrier alongside the exhaustive enums for closed grammatical
  structure. It must not generate one member enum variant per catalog entry.
- Task 5 must receive the immutable normalized grammar environment in scan
  input. `ParserCatalogs` is not the parser vocabulary interface.
- Task 6 must seal normalized surface rows using the two dumb morphology
  recipes above. Bootstrap definitions and later plugin definitions traverse
  the same row compiler and reverse indexes.
- Task 9 must construct nouns and other registry terminals from
  declaration/registry identities in the environment, not make a catalog
  domain the final runtime type or membership authority.
- Task 10 must make provenance IDs migration-capable. Public accessors return
  `&str`, backed by owned, shared, or interned storage as appropriate; public
  `&'static str` owner IDs are forbidden.

## Follow-up work

- [Define the neutral declaration schema and bootstrap reader](../tickets/planned/builtin-v2-declaration-schema.md)
- [Build the open grammar environment](../tickets/planned/english-v2-open-grammar-environment.md)
- [Committed keyword-action stubs](../tickets/planned/builtin-v2-keyword-action-stubs.md)
- [Committed keyword-ability stubs](../tickets/planned/builtin-v2-keyword-ability-stubs.md)
- [Committed creature-type stubs](../tickets/planned/builtin-v2-creature-type-stubs.md)
- [Committed noncreature-subtype stubs](../tickets/planned/builtin-v2-noncreature-subtype-stubs.md)
- [Committed card-type declarations](../tickets/planned/builtin-v2-type-declarations.md)
- [Committed counter-kind declarations](../tickets/planned/builtin-v2-counter-kind-declarations.md)
- [Committed designation declarations](../tickets/planned/builtin-v2-designation-declarations.md)
- [Read-only catalog coverage validator](../tickets/planned/builtin-v2-catalog-coverage-validator.md)
- [Resolve full plugin grammar providers](../tickets/planned/builtin-v2-plugin-grammar-provider.md)
- [Compile and match positional spelling frames](../tickets/planned/builtin-v2-spelling-frame-consumer.md)
- [Compile English-authored card sources](../tickets/planned/builtin-v2-english-card-consumer.md)
- [Cut over the completed builtin grammar inventory](../tickets/planned/builtin-v2-grammar-consumer.md)

## Related decisions

- [English v2 rewrite](english-v2-rewrite.md)
- [Macros are declarative](macros-are-declarative.md)
- [Idris is a soundness gate](idris-is-a-soundness-gate.md)
- [Semantics, spelling, lowering](semantics-spelling-lowering.md)


## Superseded: attachment participles are adjectives (2026-08-28)

The 2026-08-24 attachment-determiner ruling above is superseded by the
12-10 landing (review-verified): `equipped`, `enchanted`, and `fortified`
are post-determiner adjective-slot participles, not an open determiner
family — the adjective analysis is what admits attested "an enchanted
creature", which the determinative model could not express. The macros'
`grammar` contribution class survives with the same attestation rules;
only the lexical category moved. The generic reference-NP construction
consuming a sole-determiner class remains for members that genuinely are
determiners.


## Amendment: Verb Frame vocabulary (2026-09-04)

This amendment supersedes the verb-valence vocabulary above without changing
the grammar. A verb declaration owns a `VerbFrameSet`; every member is one
ordered `VerbFrame`. The serialized `Verb` field is `frame_set`, the `Custom`
member field is `frames`, and the amount-selecting member is
`MeasureComplement`. `VerbFrameKey` remains an internal compiler compatibility
key that matches a realized phrase against a declared Verb Frame; it is not
itself the lexical schema.

The realized syntax tree calls an instantiated lexical verb plus its selected
Complements a `LexicalVerbPhrase`. Generic `VerbPhrase` remains the larger
category that may add auxiliary or other clause structure. Historical uses of
“valence,” “Numerative,” and “base verb frame” above are retained as the
record of the earlier decision but no longer name current declarations, roles,
or syntax-tree values.
