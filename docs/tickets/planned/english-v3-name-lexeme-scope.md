---
needs: []
---
# Scope shortened card names to their own card, and declare each subtype once

Priority: high.

## Why

A shortened card name ("Chandra") is admitted on every face as a surface
variant of every full name it shortens. The bare word "Chandra" therefore has
**21** lexical alternatives in the english-v3 environment: **19** card-name
catalog lexemes (variant 1, the shortened name, of each of the 19 Chandra cards
the nickname catalog lists), one `catalog:planeswalker-types.txt/Chandra`
lexeme and one `lexeme:planeswalker_subtype/chandra` noun. In a subject
position the 19 name lexemes all read, so "Chandra deals 2 damage to any
target." has **19** Readings, each claiming a different card.

On the trunk census at the signed-numeral-cost landing
(`target/english-v3/english-v3-signed-numeral-cost/after.json`, trunk change
`xnkuznpo`: **23,459 covered / 1,605,535 Readings**), Chandra, Pyromaster
carries **246,240** Readings for one *Chandra* mention (19 × 12,960). By a
surface scan of that census, **625** covered faces / **614,305** Readings
contain a shortened name that several cards share. That bounds the faces whose
counts may fall. A further **636** covered faces / **71,967** Readings contain
another card's shortened name. Most are homographs such as *Dragon* and
*Beast*, which keep their other readings.

Owner ruling (2026-10-08): "cards should only ever have their *own* nicknames
in scope. full names can be used anywhere still. And why are 'planeswalker
type' and 'subtype' different?" The rules agree. Text printed on a card that
uses its shortened name refers to that card [CR#201.5c]. Planeswalker types
are subtypes [CR#205.3j], as are the other per-type lists [CR#205.3].

Line numbers are those of the claim parent this ticket was minted on.

## Analysis

**Where the shortened names come from.** `cargo xtask catalogs generate`
(`crates/xtask/src/catalogs.rs:125–139`) writes the canonical catalogs and
`data/gen/catalogs/card-name-nicknames.json` through
`CardNameNicknames::generate` (`crates/deckmaste_catalogs/src/nicknames.rs:40–69`).
The generator keeps legendary permanents (`:133`) and records a short name
(`:219`) only when that card's own Oracle text uses it (`attested_short_name`,
`:149`). The map is keyed by the oracle unit's display name (`:47–51`) and
holds **1,806** entries over **1,393** distinct short names. **242** of those
short names are shared by more than one card (655 entries). The largest
groups are Chandra 19, Captain America 8, and Ajani, Gandalf, Gideon and Sorin
at 7 each.

**How the lexicon admits them globally.** `deckmaste_lexical_source::card_names::load`
(`crates/deckmaste_lexical_source/src/card_names.rs:8–14`) returns the map.
`load_workspace` attaches each short name as a second surface of that card's
full-name lexeme and marks the lexeme `IdentityUse = Name`
(`crates/deckmaste_lexical_source/src/lib.rs:70–78`). The census builds **one**
`Lexicon` for the whole corpus (`crates/xtask/src/english_v3/mod.rs:111–113`).
`analyze_face` calls `lexicon.analyze_source(raw, normalized)` with no face
context (`:224–236`). The analyzer is one trie walk over every declared
surface (`crates/deckmaste_lexical/src/analysis.rs:460–480`). Nothing between
the face and the lexicon is per-face, so every face sees every card's short
name. The probe does the same thing (`crates/xtask/src/english_v3/probe.rs:99–100,114`).
The face identity needed for scoping is available at `analyze_face`:
`SelectedFace` carries `oracle_id`, `name` and `face_name`
(`crates/xtask/src/raw_corpus.rs:139–148`). Grammar sees names only through
`CatalogName` (`require head.IdentityUse = Name`,
`crates/deckmaste_english_v3/src/declarations.rs:2389–2392`). No construction
needs to change for item 1.

**Why a planeswalker type is two lexemes.** There are two independent sources:

- **The CR catalog.** `deckmaste_catalogs` extracts the CR subtype lists
  (`crates/deckmaste_catalogs/src/cr.rs:37,96`) into
  `data/gen/catalogs/planeswalker-types.txt` (80 lines) and the six sibling
  `*-types.txt` files. `load_workspace` turns each line into an invariant
  `Category::Catalog` lexeme with `TypeLineRole = Subtype`
  (`crates/deckmaste_lexical_source/src/lib.rs:86–99`). Its only consumers are
  the type-line constructions `Subtypes` and `SubtypeContinuation`
  (`declarations.rs:1574–1582`, `lexical(Catalog)` +
  `require head.TypeLineRole = Subtype`).
- **The plugin declaration.** `plugins_v2/builtin/macros/subtypes/planeswalker/chandra.ron:1–8`
  is `Subtype(category: Planeswalker, …, grammar: Noun(singular: "Chandra",
  plural: Unavailable))`. It is read by `legacy::plugins::load`
  (`crates/deckmaste_lexical_source/src/legacy/plugins.rs:30`) and exported as
  a count noun `lexeme:planeswalker_subtype/chandra` (`:148–167`) with no
  `TypeLineRole`. Rules-text nominal grammar consumes it ("a Chandra
  planeswalker").

The two lexemes are not byte-identical. The catalog lexeme is invariant,
`Capitalization::Exact`, opaque and carries `TypeLineRole = Subtype`. The
plugin lexeme is a count noun with singular and plural slots and grammar
features. In rules text the catalog lexeme is a lexical alternative that no
construction selects. "Search your library for a Chandra planeswalker card."
lists both at 26–33, and all 4 of its Readings use the plugin noun. The split
is the same for every subtype kind: plugin vs CR counts are artifact 21/23,
battle 1/1, creature 324/324, enchantment 13/13, land 17/17, planeswalker
80/80 and spell 5/5. The two artifact types with no plugin declaration are
*Attraction*, which is out of scope, and *Heartwood*, which is token-only.

The CR catalogs are also the membership authority for the planned
[builtin-v2-catalog-coverage-validator](builtin-v2-catalog-coverage-validator.md).
They therefore stay generated. Only their export as lexemes is a duplicate.

## Items

### 1. A shortened name is in scope only on its own card

The full-name surface stays admitted on every face. It covers self-reference,
"a card named …", and references to other cards by full name. The
shortened-name surface (variant 1 of a card-name lexeme) is admitted only when
the text being analysed belongs to the card that the short name belongs to.

Mechanism:

- **Declared in lexical data.** When `load_workspace` attaches a short name
  (`lib.rs:70–78`), it marks that surface variant as card-scoped. This is a
  property of the form's surface variant in `deckmaste_lexical`, carried in
  the serialized `Lexeme`. It is not a grammar feature, and no construction
  reads it.
- **Applied in lexical matching.** `Lexicon` gains a scoped analysis entry
  point that takes the set of lexeme identities whose card-scoped variants are
  admitted. Trie values for a card-scoped variant of any other lexeme are not
  emitted as candidates. The existing `analyze`/`analyze_source` become the
  empty-scope case, so they admit full names only.
- **Scope = the analysed card.** `analyze_face` builds the scope from the
  face's card. That is the full-name lexemes of every oracle unit sharing the
  face's `oracle_id`, keyed by the same oracle-unit display name the generator
  used. Text on one face of a multi-face card may name another face of the same
  card, and that card is "that card" in [CR#201.5c]. Text granted to another
  object (a quoted token ability) is still printed on this card
  [CR#201.5a,201.5c], so it is covered by the same scope.
- **The probe stays unscoped.** It admits full names only. Use the census
  `--card-name` selector to witness a scoped face. No new flag is needed.
- No guard names a card, lexeme or construction. `CatalogName` is unchanged.

A Reading that used another card's short name is a wrong analysis and may be
lost. A face that loses **all** its Readings is a defect: find which name it
needed. If the face's own name is missing from the nickname map, the fix is in
the generator (`nicknames.rs`), not in a scope exception.

### 2. One lexeme per subtype, declared as the subtype it is

Retire the catalog-derived subtype lexemes for all seven subtype kinds. The
same arm (`lib.rs:86–99`) emits them all, and all seven are subtypes
[CR#205.3]. The plugin `Subtype` declaration becomes the single lexeme:

- **Consumer side.** `legacy::plugins::load` marks every
  `DeclarationKind::Subtype(_)` lexeme `TypeLineRole = Subtype`.
  `load_workspace` stops emitting lexemes for the seven `*-types.txt`
  catalogs. `Subtypes`/`SubtypeContinuation` (`declarations.rs:1574–1582`)
  take the plugin noun's singular form instead of `lexical(Catalog)`, still
  gated only by `require head.TypeLineRole = Subtype`. Supertype and card-type
  catalog lexemes are untouched.
- **Generator side.** `catalogs generate` keeps writing the `*-types.txt`
  files as the CR membership authority for the coverage validator. Their
  lexical export is retired, and `crates/deckmaste_lexical_source/README.md`
  ("Type Line distribution", `:35–39`) is updated to say that subtype role
  comes from the plugin declaration.
- A type-line face that loses its Reading because a CR subtype has no plugin
  declaration (*Heartwood* is the known gap) gets that declaration. *Attraction*
  is out of scope and gets none.

## Witnesses

Probe counts were taken at the claim parent with
`cargo xtask english-v3 probe --category Sentence`. Face counts come from the
`after.json` census above. "Expected after" assumes the short name is the
face's only multiplier at that span. The census decides.

| Witness | Now | Lexical alternatives at the name | Expected after |
| --- | ---: | --- | ---: |
| "Chandra deals 2 damage to any target." (probe) | 19 | 19 card-name + PW-type catalog + PW subtype noun | 0 (probe is unscoped) |
| "Jace draws two cards." (probe) | 4 | 4 card-name + PW-type catalog + PW subtype noun | 0 |
| "Ajani deals 2 damage to any target." (probe) | 7 | 7 card-name + PW-type catalog + PW subtype noun | 0 |
| "Gandalf deals 2 damage to any target." (probe, legendary creatures) | 7 | 7 card-name | 0 |
| "Niv-Mizzet deals 1 damage to any target." (probe) | 5 | 5 card-name | 0 |
| "Search your library for a Chandra planeswalker card." (probe, control) | 4 | PW subtype noun selected; catalog lexeme admitted, unused | 4 |
| Chandra, Pyromaster (face) | 246,240 | 19 per mention | 12,960 |
| Chandra, Flame's Fury | 6,859 | 19³ | 1 |
| Chandra, Pyrogenius / Chandra, Bold Pyromancer | 13,718 each | 19³ × 2 | 2 each |
| Chandra, Flame's Catalyst | 30,324 | 19² × 84 | 84 |
| Chandra Ablaze | 418 | 19 × 22 | 22 |
| Sorin, Vengeful Bloodlord | 168 | 7 × 24 | 24 |
| Edgar Markov | 288 | 6 × 48 | 48 |
| Niv-Mizzet, Parun / Niv-Mizzet, the Firemind | 10 / 5 | 5 per mention | 2 / 1 |

The type-line census (`--field type-line`) is the control for item 2. Every
face keeps its count and digest. The only change is the selected lexeme
identity in subtype positions.

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, scope).
Deltas:

1. Verification budget (binding): one full text census (`--all --workers 12
   --samples-per-face 0`) and one full type-line census (`--field type-line
   --all`) on the finished feature. Compare each face by face (per-face
   `readings_sha256` and counts) against the claim parent. The runtime change
   to `deckmaste_lexical` makes `--incremental-from` fall back, so the proof is
   a full run. Iterate on `--card-name` / `--text-contains` subsets, never on
   `--all`. Tests: `cargo test` and `cargo clippy --tests -- -D warnings` on
   `deckmaste_lexical`, `deckmaste_lexical_source` and `deckmaste_english_v3`.
   Run `cargo test -p xtask` only for the touched `english_v3` module and
   catalog suites. No Lean checks. Evidence goes under the workspace's ignored
   `target/english-v3/`, never `/tmp`.
2. Write the witnesses as tests first. Re-spell
   `legendary_nicknames_are_reversible_variants_of_the_full_name`
   (`lib.rs:201`) against the scoped entry point. It keeps the same four
   cards and the same asserted spellings. Then add the negative: another
   card's scope admits no short name.
3. Zero lost faces. A Reading count may fall only on a face whose analysed
   text contains a short name with more than one card-name alternative, or a
   foreign short name. Name each face with a nonzero drop in the census delta
   by count, not by list.
4. No guard may name a lexeme, construction, card or word. Scope is lexical
   data plus face identity. No new xtask subcommands, flags or fixtures.
5. CR citations are already in this ticket. Re-run
   `cargo xtask cite check --list-noncompliant` on anything added.
6. Record the owner ruling as an amendment in
   `docs/decisions/english-lexical-analysis.md` ("Shortened names are
   card-scoped"). Then add the glossary term the landing needs to
   `docs/contexts/oracle-english/CONTEXT.md` through `domain-modeling`.
   Proposed term: **Shortened Name**, citing [CR#201.5c]. Avoid
   *nickname* in identifiers.

## Out of scope

- **Determiners on names (checked, no item needed).** A proper name on a card
  is a bare NP or the complement of *named*. Card-name lexemes already reject
  determiners. "Return a Gandalf to its owner's hand." reads **0**. "Exile
  that Sorin." and "Destroy each Chandra." read **1** each, but both Readings
  are `DeterminedNounPhrase` over the *subtype noun*
  (`lexeme:planeswalker_subtype/sorin`, `…/chandra`), not a name. Supported
  Oracle text attests a subtype noun heading a determined NP: Garruk, Cursed
  Huntsman ("each Garruk you control"), Countersculpt ("behold a Jace",
  "choose a Jace you control or reveal a Jace card"), and **31** cards with "a
  Jace token you control". This is the description use of a subtype
  [CR#109.2]. Under the pruning rule those Readings stay. Do not re-probe.
- Shortened names that are also common words or creature types (*Dragon*,
  *Beast*, *Sliver*, *Storm*) are a generator-quality question for
  `nicknames.rs`. Item 1 already confines them to their own card.
- The legacy `deckmaste_english` stack and `catalogs text` (legacy catalogs)
  are unchanged.
- Supertype and card-type catalog lexemes.

## Landing record

Six items per CLAUDE.md, plus:

7. **Readings total before/after.** This is the deliverable. Report the text
   census total Readings and covered faces at the claim parent and at the
   finished feature. Report the type-line census the same way. Name the drop
   attributable to item 1 and confirm item 2 changed no type-line count or
   rules-text count.
