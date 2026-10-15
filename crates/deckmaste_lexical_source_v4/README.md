# deckmaste_lexical_source_v4

The core English lexicon and the Catalogs of English v4
([`docs/decisions/english-v4.md`](../../docs/decisions/english-v4.md) §§2, 5),
loaded as one Lexical Environment: a `deckmaste_lexical::Lexicon` and the
`deckmaste_syntax::FrameCatalog` of its Subcategorization Frames.

```rust
let environment = deckmaste_lexical_source_v4::LexicalEnvironment::load()?;
let grammar = deckmaste_english_v4::bind(&environment.lexicon, &environment.frames)?;
```

## Data files

Every file is hand-authored RON, embedded at build time. Entries state only
what the defaults do not; [`src/schema.rs`](src/schema.rs) documents each
authored type.

| File | Provenance | Lexeme ids |
|---|---|---|
| `lexicon/english/closed_class.ron` | Core (symbols: Symbol) | `det:`, `pronoun:`, `coord:`, `sub:`, `verb:` (auxiliaries), `affix:`, `clitic:`, `symbol:` |
| `lexicon/english/prepositions.ron` | Core | `prep:` |
| `lexicon/english/verbs.ron` | Core | `verb:` |
| `lexicon/english/nouns.ron` | Core | `noun:` |
| `lexicon/english/adjectives.ron` | Core | `adj:` |
| `lexicon/english/adverbs.ron` | Core | `adv:` |
| `catalogs/types.ron` | Catalog | `type:`, `supertype:` |
| `catalogs/subtypes.ron` | Catalog | `subtype:` |
| `catalogs/keyword_actions.ron` | Catalog | `action:` (verbs, and nouns: *Monstrosity 3*, *the Ring*) |
| `catalogs/keyword_abilities.ron` | Catalog | `keyword:` (and variants), `keyword:…:bound`, `stem:`, `keyword_verb:` |
| `catalogs/ability_words.ron` | Catalog | `ability_word:` |
| `catalogs/counters.ron` | Catalog | `counter:` |
| `catalogs/designations.ron` | Catalog | `designation:` |
| `catalogs/turn_parts.ron` | Catalog | `turn_part:` |
| `catalogs/tokens.ron` | Catalog | `token:` |
| `catalogs/introduced_names.ron` | Catalog | `introduced_name:` |
| `catalogs/anchor_words.ron` | Catalog | `anchor_word:` |
| `data/gen/catalogs/card-names.txt` (read at load) | Catalog | `name:` |

Only Core vocabulary may be named by a grammar (`deckmaste_syntax` rejects any
other Closed Word). A word is never both Core and a catalog item of the same
Category; the loader rejects it.

## Morphology and frames

Every verb has its full paradigm and every count noun its plural; regular
spellings come from `deckmaste_lexical`'s default rules and irregular ones are
explicit overrides. Only the modals and negative auxiliaries are defective
(`finite_only`). Attestation never withholds a form.

Frames are written once, in active voice, naming prepositions by lemma:
`TransitiveWithPP(["into", "onto"])`, `ComplexTransitive` (*turn it face up*),
`ComplexCatenative(ToInf)` (*cause you to sacrifice*), `ContentClause`
(*decide whether …*), `Amount` (*scry 2*), `NominalAmount` (*amass Zombies 2*),
`Cost` (*waterbend {3}*), and for nouns, adjectives and determinatives
`Complement(PP(["of"]))`. Prepositions declare each complement
frame with its Host Category; `Intransitive` is an Intransitive Preposition
(*out* in *phase out*). The loader checks that every named preposition is
declared, that every verb has a frame, and that each frame has the shape of
its family. Only an auxiliary in `closed_class.ron` may have no frame (perfect
*have*, *hasn't*, contracted *'ve*): the grammar names those as Closed Words.
Contracted auxiliaries (*'ve*, *'re*, *'m*, *'s*) are suffix-bound verbs with
full finite bundles; the genitive *'s* is a different Lexeme with the same
spelling.

Adjective grades are adjectives of their own with a `Grade` feature. Adverbs
record their clause positions as classes (`FrontAdverb`, `CentralAdverb`,
`EndAdverb`, `PhrasalAdverb`). Subtypes record their families as classes
(`CreatureType`, `PlaneswalkerType`, …).

## Catalog specifics

- Every catalog entry cites the rule that defines it as `rule: "[CR#701.8]"`,
  which `cargo xtask cite` checks like any other citation.
- A keyword action is realized by verbs (atomic multiword ones such as
  *manifest dread* included) and, where the CR prints it so, by nouns.
- A keyword ability's variants (*megamorph*, *partner—Friends forever*) are
  Keyword Lexemes of their own, and the verbs its rule defines (*cycle*,
  *crew*, *equip*) are catalog verbs, never core vocabulary.
- Ability words and keyword abilities are `Keyword`-category Lexemes; ability
  words carry `LabelKind: AbilityWord`. A registered label is never also
  analyzed as an open Flavor Word.
- A keyword ability's parameters on a keyword line (`Amount`, `Cost`,
  `DashCost`, `Quality(Some("from"))`, …) bind into `FrameCatalog::parameters`
  for the syntax engine's `Parameters` schema. Nouns can declare ordered
  notation parameters too, as the indexed noun in a Class header does. A Bound Keyword Surface (*walk*,
  *cycling*) is a suffix Lexeme, and its stems (*island*, *wizard*) are prefix
  `stem:` Lexemes drawn from the declared stem classes. Stems admit initial
  capitalization but are never independent nouns or collisions that suppress
  Shortened Names.
- A counter kind is the compound noun `<kind> counter`; measured kinds
  (`+1/+1 counter`) are measured compounds.
- Card names are opaque `Catalog`-category Lexemes. Each card's Shortened Names
  [CR#201.5c] are derived from its full name (before its first comma, ` the `
  or ` of `, else its first word), dropped when another declaration already
  spells them unless that is a planeswalker type, and admitted only in that
  card's own scope.
- Introduced Names (*Marit Lage*, *Moon*) and Anchor Words [CR#614.12c]
  (*Khans*, *Mirran*) list the cards whose text uses them and are admitted
  only in those cards' scope. Introduced Names are opaque `Catalog`-category
  Lexemes and never card names; Anchor Words are `Keyword`-category Lexemes
  with `LabelKind: AnchorWord`, registered labels like Ability Words.

## Tests

`cr_coverage` checks each catalog against the CR's enumeration
(`data/rules/cr.txt`; ability words also against Scryfall's
`data/catalogs/ability-words.json`), with the out-of-scope and Un-set items
named in the test. `paradigms` checks full paradigms and that every lexical
value realizes and analyzes back. `labels`, `names` and `loader` cover
registered-label exclusivity, Shortened Name, Introduced Name and Anchor Word
scope and the loader's rejections. The grammar-leak lint lives with the grammar, in
`deckmaste_english_v4/tests/catalog_leak.rs`.

## Not here

Plugin-declared vocabulary arrives when `plugins_v3/` does; at
`english-v4-final-suffix-cutover` the catalogs move into plugin declaration
metadata and this crate drops its `_v4` suffix.

Verbs can declare ordered construction parameters, including a restricted slot
projected to a phrase with its grammatical relation. Modal *choose* declares a
quantity projected to a Noun Phrase Object; its ordinary verb frames remain
unchanged. `DashNonManaCost` distinguishes verbal/composite keyword costs,
including their terminal period, from the mandatory dash of forms such as
suspend with a mana cost.

Notation symbols declare their classes in `closed_class.ron`; the loader does
not infer their category from their spelling. Pawprint quantities are distinct
from cost-symbol sequences. Bound keyword stem patterns project a Quality with
its permitted type premodifiers inside that Quality.

The `non` prefix declares only its hyphenated and unhyphenated spellings;
sentence-initial capitalization is derived, not duplicated as spelling variants.
