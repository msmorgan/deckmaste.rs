---
needs: []
---
**The voting procedure `[CR#701.38]` — the clause shapes and lexical
subcategorization the grammar still cannot express.** Campaign-internal
residue of `english-structural-recovery-zero`, split out so the diagnosis
survives in the tree.

Round `chance` (2026-07-27) landed the two lexical repairs the voting family
needed (`receive`, `ensue`) and one small production (`While <gerund>,
<clause>`), and deliberately **dropped the central voting-procedure stage**.
Counts are unresolved `clause` rows measured against the post-`chance` census
(clause 3744/69039, unknown 5468). **33 vote rows remain** of the 55 the round
opened with.

`[CR#701.38b]` is the governing sanction for the whole family: *"The listed
choices may be objects, words with no rules meaning that are each connected to
a different effect, or other variables relevant to the resolution of the spell
or ability."* A vote choice is therefore EITHER a real nominal OR a
rules-meaningless word. Any design that enumerates observed labels is wrong;
the label slot needs a shape.

1. **The `Starting with you, each player votes for <choices>` procedure —
   22 rows, the largest remaining piece.** A designed stage for this was
   written during round chance and
   **dropped on a falsified premise**: the plan asserted "`start` is only a
   noun in the TSV", and prescribed adding `Vocab::Start` as a regular verb.
   In fact `regular-vocabulary.tsv` already carries BOTH `start<TAB>noun_count`
   (797) AND `start<TAB>verb` (798) — **and `starting<TAB>noun_count` (799)**,
   which the plan never analyzed and which gives the fronted `Starting` token
   a competing nominal reading in exactly the slot the new production wanted.
   The plan's structural direction (a vote-scoped `VotePrelude` production
   plus a structured `VoteChoiceList` predicted only at the final slot, rather
   than a generic fronted-gerund production) remains sound; its
   lexical foundation is not. **Redesign from the real TSV state**, and settle
   the `starting` noun entry first — it is very likely a word-bank artifact,
   and whether it can be removed is the round's opening question.

   The label lists to admit, all bare and open across sets: `death or torture`,
   `wild or free`, `sprout or harvest`, `time or money`, `time or knowledge`,
   `past or present`, `innocent or guilty`, `grace or condemnation`, `feather
   or quill`, `evidence or bribery`, `dominion or guidance`, `strength or
   numbers`, `profit or security`, `carnage or homage`, `sickness or
   psychosis`, `denial or duplication`, `return or embark`, `planeswalk or
   chaos`, `Redhorn Pass or Mines of Moria`, `blue, black, red, or green`.
   Multi-token labels stay one label. The same slot also takes real nominals
   (`a nonland permanent you don't control`, `an artifact, creature, or
   enchantment card in your graveyard`, `up to one creature`), so the
   compositional branch must win for those.

2. **Preverb-adverb subcategorization — 6 rows.** Every `each player secretly
   votes for <NP>, then those votes are revealed` row fails on `secretly`
   before the finite verb. The production exists; the preverb-adverb scanner
   is pinned to `next` even though `secretly` is an ordinary TSV adverb.
   Widening it to every adverb is a corpus-wide ambiguity surface and
   hardcoding `secretly` is an identity patch, so this wants lexical metadata
   marking which adverbs are licensed preverbally. Two of the six also need
   item 1's choice-list shape in a subject-first vote predicate.

3. **Heterogeneous postnominal predicate coordination — 3 unresolved rows plus
   one WRONG TREE.** `<X> with the most votes or tied for most votes` needs a
   `NominalComplement` coordination of a PP with a participial predicate; the
   grammar's coordinated-modifier consumer is all-adjective only.
   **Council Guardian is not a positive precedent even though it parses**:
   `with the most votes` attaches to `protection` and `or tied for most votes`
   becomes a shared-predicate clause coordination subjected to `This creature`,
   when the intended host is `each color`. A byte-identical render hides it.
   Fix the three unresolved rows and Council Guardian's tree together; do not
   bless the current reading.

4. **Prepositional-object-gap relatives — 4 rows.** `a choice you voted for`
   has its gap inside a selected `for` phrase. `RelativeGap` distinguishes only
   `Subject` and `Object`, and `ObjectGapVerbPhrase` models a direct-object gap
   while requiring complete following PPs. Wants a typed prepositional-gap
   relative. Do not imitate the surface by dropping `for`, storing a raw
   suffix, or reading `choice` as opaque.

5. **Scoped arbitrary vote-label nominal modifiers — 1 row.** Travel Through
   Caradhras's `For each Mines of Moria vote, ...` uses a choice label as a
   nominal premodifier. One occurrence; deliberately not worth a global literal
   modifier scanner. Fold into item 1's label shape if that shape generalizes.

6. **`Will of the Planeswalkers` is not peeled as an ability word — 5 rows.**
   The five `Path of the *` planes keep `Will of the Planeswalkers — ` inside
   the failing span, so item 1's production cannot see through it and those
   five rows are NOT item-1 recoveries. `Will of the council` peels correctly
   on Tyrant's Choice and Council's Judgment, and **both strings are present in
   `data/catalogs/ability-words.json`** and the slot is case-insensitive, so
   the blocker is downstream of catalog membership — diagnose
   `ability_word_prefix` against a plane's ability segmentation. **This carries
   a frozen-cell decision**: ability words land in the `flavor header` opacity
   cell (622/1305), so fixing it moves a frozen cell by roughly +5. Decide and
   predict that delta explicitly before landing.

7. **Markerless reduced passive relative — 1 row.** Trial of a Time Lord's
   `each card exiled with this Saga`. `RelativeSubject` requires an explicit
   relative marker. **Held behind the campaign's blocked reduced-relative
   perturbation work** — do not attempt standalone.
