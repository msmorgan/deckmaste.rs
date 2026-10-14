---
needs: [english-v4-compiler-and-runtime]
---
# Build `deckmaste_lexical_source_v4` with the core English lexicon and the MTG catalogs

Create `crates/deckmaste_lexical_source_v4` owning clean, human-authored RON
files for the core English lexicon (Core provenance) and the Catalogs (Catalog
provenance), using the frame schema proven in
`english-v4-compiler-and-runtime`, per [`docs/decisions/english-v4.md`](../../decisions/english-v4.md)
(§2, §5):

- `lexicon/english/`: `closed_class.ron`, `prepositions.ron` (with complement
  frames and unselected host-category distribution), `verbs.ron`, `nouns.ron`,
  `adjectives.ron`, `adverbs.ron`.
- `catalogs/`: `types.ron` (`[CR#205.2a,205.4a]`), `subtypes.ron`
  (`[CR#205.3]`), `keyword_actions.ron` (`[CR#701.1]`), `keyword_abilities.ron`
  (`[CR#702.1]`), `ability_words.ron` (`[CR#207.2c]`, including Vintage-legal
  printed ability words omitted by the CR list such as `Corrupted`),
  `counters.ron` (`[CR#122.1]`), `designations.ron`, `turn_parts.ron`
  (`[CR#500.1]`), `tokens.ron`.
- Read only `data/gen/catalogs/card-names.txt` from `data/` (plus contextual
  derived shortened self-reference nicknames `[CR#201.5,201.5c]`).

## Requirements

1. **Zero dependency on `plugins_v2/builtin`, `deckmaste_macro_def`, or
   `deckmaste_lexical_source`.**
2. **Full CR Catalog Coverage:** Every Vintage-legal CR catalog item—including every
   `[CR#701.1]` keyword action (including common verbs like `destroy`, `exile`,
   `sacrifice`, `tap`, `untap`, `counter`, `create`, `reveal`, `search`,
   `shuffle`, `fight`, `regenerate`, `attach`, `unattach`, `scry`, `mill`,
   `goad`, `proliferate`, `populate`, `investigate`, `explore`, `connive`,
   `amass`, `surveil`, etc.)—lives in its corresponding `catalogs/*.ron`
   file with its subcategorization frame.
3. **Full Morphological Paradigms:** Every verb and noun receives its full
   grammatical paradigm (`Unavailable` is permitted only for genuinely
   defective English words, never for unattested corpus forms). Pronouns are
   declared with the features Oracle gives them: `you` is second person
   singular only (Oracle never addresses more than one player as `you`).
   Each preposition's Host Categories come from corpus evidence, not open-domain
   English: unselected `on` + NP is a Nominal postmodifier only (every
   `on your turn` follows combat, a phase or a step; `put … on it` is a
   complement). The seed's `in` and `from` hosts are unverified.
4. **Registered Label Exclusivity:** An open flavor-word label never duplicates
   a registered ability word (`[CR#207.2c]`) or keyword (`[CR#702.1]`).
5. **Catalog Validator & Grammar-Leak Lint:**
   - Unit test verifying 100% coverage of Vintage-legal CR catalog inventories
     (and Scryfall's `data/catalogs/ability-words.json` for `ability_words.ron`)
     and complete morphological paradigms.
   - Structural lint verifying that no word from `catalogs/*.ron` appears as
     a closed word or hardcoded identifier in grammar declarations.

Standard constraints apply.

Measured on `uvnzzxzo`.

1. **Census delta**: does not apply yet; the census arrives with `english-v4-corpus-stratification-and-judgment-harness`.
2. **Judgment set**: does not apply yet; the judgment set arrives with the same ticket.
