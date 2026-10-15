---
needs: []
---
# Named tokens and anchor words as `deckmaste_lexical_source_v4` catalogs

Two kinds of name that card text introduces are declared nowhere, so every
unit using them is a vocabulary gap (`docs/english-v3-test-audit.md`, the
lexical-fact gaps; the census's `VocabularyGap` family):

- **Named tokens**: names a card's own text gives a token, in *create a ...
  token named Lotus* and in *create Angelo, a legendary ... token* (an
  appositive: the name is the Object, the description a supplement). About 80
  in the supported corpus, among them Angelo, Boo, Cadet, Stangg Twin,
  Tamiyo's Notebook, Tuktuk the Returned, Marit Lage, Vecna. A name that is
  also a card name (*Ajani's Pridemate*) is already a lexeme.
- **Anchor words** [CR#614.12c]: the two choices a replacement effect offers
  as a permanent enters, each heading a bulleted ability: Khans/Dragons,
  Abzan/Mardu/Jeskai/Sultai/Temur, Mirran/Phyrexian, Brotherhood/Enclave,
  NCR/Legion, Believe/Doubt. Its ability is linked to the ability that offers
  the choice [CR#607.2m]. Like a registered Ability Word, an anchor word is a
  label that is never also a Flavor Word.

Declare each as its own catalog with the card that introduces it as
provenance, scoped like a Shortened Name to that card's own faces when the
name is not otherwise a card name, and give anchor words the label analysis
Ability Words have.

In the choice that offers them (*choose Khans or Dragons*), anchor words are
metalinguistic NPs: what is chosen is the word itself (CGEL Ch. 5 §8, p. 401;
[CR#614.12c]). The judgments' `NounPhrase "Khans"` assertions stand. The
Object of *choose* stays an ordinary NP (*choose a creature you control*,
*choose a color*); only option words (anchor words, *odd or even*, *left or
right*, …) form a metalinguistic NP, so no arbitrary nominal gets that
reading.

Standard constraints apply.

Measured on `zumvxplz`.

1. **Census delta**: unchanged, as the grammar does not yet bind to the full lexical environment (Stratum 0's first task). Before and after: 0 of 32,828 faces read and 0 Readings; all 92,934 units unread (`Unbound`). Lost faces: none.
2. **Judgment set**: `seed.txt` holds; 0 entries enforced and 2,834 pending (no Stratum completed). Two pending entries now name their card, since their names are scoped: *named Axe* (Iron Hills Blacksmith) and *named Wasp* (The Hive).
