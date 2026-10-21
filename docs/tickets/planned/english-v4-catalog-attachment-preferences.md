---
needs: [english-v4-ranking-key-engine, english-v4-right-association-key, english-v4-attachment-classes, english-v4-comma-adjunct-island, english-v4-relational-noun-of-complement, english-v4-fronted-temporal-pp-clause, english-v4-bare-relative-fronted-adjunct, english-v4-cost-not-passivizable, english-v4-keyword-phrase-adjunct-island, english-v4-auxiliary-adjunct-canonicalisation]
---
# CatalogPreference: host preferences declared in the lexicon and catalogs

Fill the Ranking Key's **CatalogPreference** component
(`docs/decisions/english-v4.md` §4.3). It counts catalog-declared host
dispreferences hit.

Every preference is declared on a lexeme or frame in
`deckmaste_lexical_source_v4`, never by name in `english.ron` or the
engine. They are Preferences: they never block Admission.

## Declarations

1. **Zone nouns** [CR#400.1] (library, hand, battlefield, graveyard, stack,
   exile, command zone) are dispreferred hosts for unselected relative clauses
   and `with`-PPs. Declare this on them in `nouns.ron` (owner's ruling).
   - This fixes the *[card] [in <zone>] [<relative clause / with-PP>]* traps:
     - Tetsuo: *your hand with mana value …*;
     - Draugr Necromancer: *exile your opponents own*;
     - Activated Sleeper: *a graveyard that was put there*;
     - Sphere of Annihilation: *graveyards with mana value …*;
     - Mockingbird: *the battlefield with mana value …*.
   - It does not touch *play with their hand revealed*
     (`play_with_absolute.txt`). There *their hand revealed* is an absolute
     `with`-clause, not a postmodifier on *hand*.
2. **Preferred host per `(preposition, complement_frame)`** in
   `prepositions.ron`:
   - `with + NP` prefers a Nominal;
   - origin `from + NP` / `from + PP[among]` prefers a Predicate
     (`docs/oracle-style-guide.md` §9, Permissions from unusual zones: a
     permission states its origin);
   - `for + NP` attaches high (distributive *for each …*, durational *for
     the rest of the game*; `syncretic_word_forms.txt:3`,
     `predicative_gap_relatives.txt:5`).
3. **Head-declared preferences**, where a frame cannot select the phrase:
   - *in addition to* is a Predicate adjunct, declared on *addition* in
     `nouns.ron`. It is *be*'s adjunct in *it's a Bird in addition to its
     other types*.
   - A `with`-PP on *enter* (*enters with … counters*) and *exile*
     (*exiled with …*) belongs to the verb.
   - An origin phrase and *without paying its mana cost* belong to *cast* and
     *play* (`docs/oracle-style-guide.md` §9, Alternative costs: "keep the
     cost phrase adjacent to `cast`" [CR#118.9]).
   - A purpose infinitival belongs to *spend* (*spend mana … to cast those
     spells*).
   - Where a real frame is right (for example *enter with*), prefer a frame:
     same-head complement priority then applies.

## Acceptance

Measured against the judgment set (310 single-group entries), not the ten
tuning units. The cost study's simulation of the full ruled key
(AttachmentClass → CatalogPreference → RightAssociationWords →
ProductionCost), on the line at `ouyqrrlq`:

- **≥ 221 of 310** entries with an all-endorsed top tier (today 94).
- Single-cheapest units **≥ 86%** (today 33.7%). The largest cheapest tier is
  8 Readings (today 6,354).
- Every entry whose top tier moves into "no endorsed Reading" is adjudicated.
  The simulation had 27 such moves against 137 improvements.
  - Wrong judgments are rewritten (`english-v4-wrong-judgments-and-np-and-dup`
    covers `clause_final_predicative_as.txt:9` and `coordination.txt:34`).
  - Equivalent-meaning ties are resolved by the FinalTiebreak and their
    entries rewritten to assert what is wanted.
- Readings and Surplus are unchanged, and no face is lost.
- For reference, not acceptance: 8 of the 10 tuning units then rank the
  intended Reading uniquely first. Magus Lucea Kane (*this turn* over a
  predicate coordination under *next*) and Tetsuo stay open.

Standard constraints apply.

1. **Census delta**: measured at landing.
2. **Judgment set**: measured at landing.
