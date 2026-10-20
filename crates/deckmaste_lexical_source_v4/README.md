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
| `catalogs/keyword_abilities.ron` | Catalog | `keyword:` (and variants), `keyword:…:bound`, `stem:`, `keyword_verb:`, `keyword_det:`, `keyword_noun:` |
| `catalogs/ability_words.ron` | Catalog | `ability_word:` |
| `catalogs/counters.ron` | Catalog | `counter:` |
| `catalogs/designations.ron` | Catalog | `designation:` |
| `catalogs/statuses.ron` | Catalog | `status_adj:`, `prep:` |
| `catalogs/turn_parts.ron` | Catalog | `turn_part:` |
| `catalogs/tokens.ron` | Catalog | `token:` |
| `catalogs/introduced_names.ron` | Catalog | `introduced_name:` |
| `catalogs/anchor_words.ron` | Catalog | `anchor_word:`, `anchor_name:` |
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
(an interrogative content clause: *decide whether …*, *choose which creatures
attack*), `Amount` (*scry 2*), `TransitiveAmount` (*amass Zombies 2*),
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

The modals have no -s form, so their third singular is spelled like the rest
of the present. A contracted or negative auxiliary (*'re*, *aren't*, *can't*,
*'ve*) keeps its own Lexeme id and spellings but declares `form_of` the verb it
is a form of, whose lemma names it. Every auxiliary carries `VerbKind:
Auxiliary` and a `Polarity`, `Negative` for a negative form and `Affirmative`
otherwise; a lexical verb has neither. *Be* and *remain* select locative Preposition Phrases as well as
predicative Complements (CGEL, Ch. 4 §5.2, p. 257). A verb whose past participle
is spelled like a declared Adjective (*tapped*, *transformed*) carries
`AdjectivalPassive: Lexical`: that Adjective is its Adjectival Passive, as
*attached* (predicative, with its *to* Complement) is *attach*'s.

Adjective grades are adjectives of their own with a `Grade` feature
(`Plain`, `Comparative`, `Superlative`) that share their adjective's lemma
(*greater* is a grade of *great*); a comparative grade selects a *than*
Complement. A lemma that is itself comparative (*less*, *lesser*) declares its
grade, and *same* declares `Equative`, which licenses its *as* Complement. A
preposition may declare what it marks as `PrepositionMarking`: *of* an
oblique, *than* a comparative Complement. Adverbs
record their clause positions as classes (`FrontAdverb`, `CentralAdverb`,
`EndAdverb`, `PhrasalAdverb` for a focusing or multiplying one, `BoundingAdverb`
for one bounding a quantity, *at least*, *exactly*); an interrogative adverb (*how*) is also an
`InterrogativeAdverb`. Subtypes record their families as classes
(`CreatureType`, `PlaneswalkerType`, …).

## Catalog specifics

- Every catalog entry cites the rule that defines it as `rule: "[CR#701.8]"`,
  which `cargo xtask cite` checks like any other citation.
- A keyword action is realized by verbs (atomic multiword ones such as
  *manifest dread* included) and, where the CR prints it so, by nouns.
  Explicit adjective Lexemes such as *transformed* also belong to their action
  catalog entry.
- A keyword ability's variants (*megamorph*, *partner—Friends forever*) are
  Keyword Lexemes of their own, and the verbs its rule defines (*cycle*,
  *crew*, *equip*, *cascade*) are catalog verbs, never core vocabulary. So are
  the nouns its rule defines for the ability or its cost (*its {W} kicker*,
  *the gift was promised*, *if tribute wasn't paid*).
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
  with `LabelKind: AnchorWord` and class `AnchorWord`, registered labels like
  Ability Words.

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

`DashNonManaCost` distinguishes verbal/composite keyword costs,
including their terminal period, from the mandatory dash of forms such as
suspend with a mana cost.

Notation symbols declare their classes in `closed_class.ron`; the loader does
not infer their category from their spelling. Pawprint quantities are distinct
from cost-symbol sequences. Bound keyword stem patterns project a Quality with
its permitted type premodifiers inside that Quality.

The `non` prefix declares only its hyphenated and unhyphenated spellings;
sentence-initial capitalization is derived, not duplicated as spelling variants.

Oracle determinative uses are declared on their lexical owners: `target` in
core vocabulary and `enchanted`, `equipped`, and `fortified` under the corresponding
keyword abilities. Participles retain their verbal identity independently.
Permanent statuses supply adjectives and intransitive prepositions.

Nouns declare ordinary, head-only, bare-singular, or direct numerical mass distributions.
The latter licenses numerical `mana`, `life`, and `damage` without treating every
mass noun as directly countable. Temporal and manner noun kinds support NP
Adjuncts. Quantity nominals use ordinary adjectival modification and determination.

The Targeting Marker is the determinative `det:target`, its only Lexeme, with
its own `Targeting` distribution; the grammar builds it into determiners that
end in it. The count noun `noun:target`
has `HeadOnly` nominal distribution, keeping its head uses without duplicating
the marker as a noun premodifier. An attributive adjective that requires overt
determination (`adj:only`) passes that need to its nominal.
The analytic superlative *most* (*most recent*) is the determinative
`det:most`, which the grammar names directly; there are no degree adverbs. A
number-transparent quantificational noun (*number*) has `Quantity`
distribution.

Determiner distributions distinguish the indefinite article, distributive
quantifiers, and singular-or-quantified selection (CGEL, Ch. 5 §3.4,
pp. 353–354), free-choice *any*, and the fraction and multiplier
predeterminers (*half*, *double*). These feed general quantity determination without an additive
adjective list.
