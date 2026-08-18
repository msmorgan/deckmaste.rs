# Builtin-v2 macro spelling and grammar

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
    grammar: Verb("scry"),
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

`grammar` carries only the lexical and morphological facts needed to admit the
macro's English: for example, that `scry` is a verb, the realized inflections
of a compound verb, a noun's attested number forms, or a fixed clause/keyword
surface. Whole realized compound surfaces are data; there are no text offsets
or head-position markers.

This decision does not declare a morphology inventory or license guessing an
unattested form. Catalog membership alone cannot establish a plural or an
inflection. Those facts must be checked against the authoritative grammar
sources when the committed records are authored.

`grammar` is not a valency or Magic-legality declaration. In particular,
declaring `scry` as a verb does not say that Scry takes an amount, and declaring
`destroy` as a verb does not say that Destroy takes an object noun phrase.
The raw grammar may therefore admit both `scry 1` and syntactically plausible
but semantically useless combinations such as `destroy 1` or `scry each
creature target opponent controls`. Typed spelling-frame matching accepts
`Scry([Amount])` and `Destroy([ObjectNoun])` and rejects the crossed readings;
the Idris layer then validates the assembled semantic card. Teaching the
English parser those semantic restrictions would violate the v2 parser's
grammar-only boundary.

The contribution vocabulary is not a plugin-extensible grammar DSL. New
lexemes and their inflections are open data; new grammatical recipes remain
reviewed compiler work.

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

Not every English vocabulary is such a declaration. Card names remain a parse
context parameter. Ability words belong to a surface-label preservation path:
they have no special rules meaning [CR#207.2c] and do not select a semantic
macro by the label alone. Supertypes remain closed structural grammar.
Non-keyword game-action verbs such as deal, gain, and draw likewise remain
closed core grammar.

The subtype rule applies to all categories the semantic model can currently
represent: artifact, battle, creature/kindred, enchantment, land,
planeswalker, and instant/sorcery spell subtypes [CR#205.3]. Planar and dungeon
subtypes are explicit model gaps: the current subtype category axis cannot
carry them. They are not silently counted as covered, and adding either
category brings its declarations under this same rule.

## Nursery records and catalog integrity

`plugins/builtin_v2/macros/stubs` is the nursery and future address of these
macros, not a parallel registry. A stub carries the literal head shape and the
grammar contribution now; graduation adds positional parameter types, extends
`spelling` with `Param(n)` holes, and adds the semantic body in place.

The canonical keyword catalogs and every currently representable subtype
catalog back category-scoped directories:

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

## Follow-up work

- [Committed keyword-action stubs](../tickets/planned/builtin-v2-keyword-action-stubs.md)
- [Committed keyword-ability stubs](../tickets/planned/builtin-v2-keyword-ability-stubs.md)
- [Committed subtype stubs](../tickets/planned/builtin-v2-subtype-stubs.md)
- [Committed card-type declarations](../tickets/planned/builtin-v2-type-declarations.md)
- [Committed counter-kind declarations](../tickets/planned/builtin-v2-counter-kind-declarations.md)
- [Committed designation declarations](../tickets/planned/builtin-v2-designation-declarations.md)
- [Read-only catalog coverage validator](../tickets/planned/builtin-v2-catalog-coverage-validator.md)

## Related decisions

- [English v2 rewrite](english-v2-rewrite.md)
- [Macros are declarative](macros-are-declarative.md)
- [Idris is a soundness gate](idris-is-a-soundness-gate.md)
- [Semantics, spelling, lowering](semantics-spelling-lowering.md)
