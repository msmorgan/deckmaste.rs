---
needs: [english-v4-compiler-and-runtime]
---
# Build `deckmaste_lexical_source_v4` with self-contained English and MTG domain catalogs

Create `crates/deckmaste_lexical_source_v4` owning clean, human-authored RON
catalog files separated by domain, using the frame schema proven in
`english-v4-compiler-and-runtime`, per [`docs/decisions/english-v4.md`](../../decisions/english-v4.md)
(§2, §5):

- `catalogs/english/`: `closed_class.ron`, `prepositions.ron` (with complement
  frames and unselected host-category distribution), `verbs.ron`, `nouns.ron`,
  `adjectives.ron`, `adverbs.ron`.
- `catalogs/mtg/`: `types.ron` (`[CR#205.2a,205.4a]`), `subtypes.ron`
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
   `amass`, `surveil`, etc.)—lives in its corresponding `catalogs/mtg/*.ron`
   file with its subcategorization frame.
3. **Full Morphological Paradigms:** Every verb and noun receives its full
   grammatical paradigm (`Unavailable` is permitted only for genuinely
   defective English words, never for unattested corpus forms).
4. **Registered Label Exclusivity:** An open flavor-word label never duplicates
   a registered ability word (`[CR#207.2c]`) or keyword (`[CR#702.1]`).
5. **Catalog Validator & Grammar-Leak Lint:**
   - Unit test verifying 100% coverage of Vintage-legal CR catalog inventories
     (and Scryfall's `data/catalogs/ability-words.json` for `ability_words.ron`)
     and complete morphological paradigms.
   - Structural lint verifying that no word from `catalogs/mtg/*.ron` appears as
     a closed word or hardcoded identifier in grammar declarations.

Standard constraints apply.
