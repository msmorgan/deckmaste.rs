---
needs: []
---
# Correct six glossary entries that overstate or misnumber their rules

Six entries in the two glossaries cite a rule that does not say what the entry
claims. Correct them through the `domain-modeling` skill, which owns glossary
edits. Keep each entry's term. Change only the claim and the citation, and do
not reword an entry to fit an identifier. The code is already right in each
case, so this ticket edits glossary text only.

Provenance: `docs/evidence/mtg-rules-review-2026-10-21/departures-docs.md`
entries 9–11 and 22–24 (gitignored; read it from the default checkout). All
six are confirmed at head as of 2026-10-21.

## `docs/contexts/oracle-english/CONTEXT.md`

1. **Type Line** (line 291): "written with a long dash before the subtypes as
   each card type's subtype rule prescribes [CR#302.3]". [CR#302.3] covers
   creature subtypes only. The rule for every card type is [CR#205.3b]. Cite
   [CR#205.3b] for the dash, and keep [CR#205.1,205.4a] for the supertype
   order.
2. **Keyword Variant** (line 343): "A grant of a keyword that has variants
   grants each appropriate variant [CR#702.1c]." [CR#702.1c] applies only when
   an effect says "the same is true for" a list of keywords. Limit the
   sentence to that case. Otherwise a grant names the keyword or variant it
   grants, and a reference to a keyword follows that keyword's own rules, as
   [CR#702.124n] does for partner.
3. **Status, Designation** (line 1313): "`tapped`, `goaded`, `suspected`,
   `saddled` are the **Participles** of their keyword-action verbs". Saddle is
   a keyword ability, not a keyword action [CR#702.171a]. *Saddled* is a
   designation that the ability defines [CR#702.171b]. Reword so that
   `saddled` is the Participle of *saddle*, the verb defined by the saddle
   keyword ability [CR#702.171a,702.171b]. The other three remain
   keyword-action participles [CR#701.26a,701.15b,701.60b]. The lexicon
   already files saddle as a keyword ability.

## `docs/contexts/game-model/CONTEXT.md`

4. **Face-Down** (line 54): "for manifest and cloak a 2/2 creature with no
   name, subtypes or mana cost ([CR#708.2,701.40a,701.58a])". Manifest also
   gives no text [CR#701.40a]. Cloak gives ward {2} instead [CR#701.58a].
   State the two sets separately. The Lean model already separates them
   (`faceDownCreature`, `faceDownWardedCreature`).
5. **Face-Down** (lines 55–56): "A face-down Object is never a Token", with
   _Avoid_ "face-down token". No rule says this. A face-up permanent turned
   face down gets the face-down characteristics, whether or not it is a token
   [CR#708.2a]. A token is a permanent [CR#111.1], so a token can be face
   down. A token copy of a face-down permanent is a different case: it is face
   up with the face-down copiable values [CR#707.2]. Replace the sentence
   accordingly. Drop "face-down token" from _Avoid_, or make it the accepted
   term.
6. **Counter** (line 139): "Counters with the same name are interchangeable
   ([CR#122.1])". The rule reads "same name or description". Add "or
   description". The entry's modeling choice (a counter's kind is the name of
   its declaration) is unaffected.

## Follow-up check

Item 5 has a consequence outside the glossary. Check that neither the Lean nor
the Rust model treats face-down status as possible only for cards. If either
one does, ticket that separately. This ticket does not change code.
