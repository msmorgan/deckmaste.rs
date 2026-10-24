---
needs: []
---

# english-v4: anaphoric determinatives, ProperName number, single-Complement heavy shift

Findings F2 and F8 of the post-Stratum-7 oncology re-scan.

## F2: anaphoric determinatives with no number

The determinatives *enchanted, equipped, fortified* (keyword-ability catalog, `determiner: true`)
and *attacking, defending* (closed-class catalog) are synthesized with no Number and no
countability. So:

- plurals such as *attacking creatures* and *enchanted creatures* read both as Determined and as
  a BarePlural with a participle premodifier (duplicate tree; witness Remove Enchantments,
  claimed 4×);
- *Destroy attacking creature.* parses (ungrammatical).

Make these determinatives singular count, in catalog and lexicon data, never in productions.
Check the corpus before narrowing *attacking* or *defending* to *player* ([CR#506.2], OSG §6).

## F8.1: ProperName number

`ProperName` requires Person Third but not Number Singular, so Kaust reads *Turn* as a name with
plural *target*. Require Singular, keeping plural names (Brothers Yamazaki and any others in the
corpus) reading.

## F8.2: `shifted-object/interleaved1` with one Complement

The syntax compiler emits the `interleaved1` heavy-shift variant when the frame has one
Complement (`split == 1`), where it repeats the plain order. Show the duplicate, then stop
emitting it there.

No face may be lost; every Reading removed must be a duplicate or non-English.

## Outcome

- **F2.** Confirmed: *attacking creatures your opponents control* read as Determined (det
  *attacking*) and as BarePlural with the participle (Remove Enchantments, twice). The five
  determinatives are now `number: Singular, countability: Count` in `closed_class.ron` and
  `keyword_abilities.ron` (CGEL, Ch. 5 §4, p. 355). *Attacking* and *defending* are **not**
  narrowed to *player*: the corpus has no determinerless singular *attacking/defending X* but
  *player*, yet the owner's round-9 ruling makes them the same mechanism as *enchanted*, which
  determines any noun; *attacking creature* as a role is wrong for the rules, not ungrammatical
  (principle 1), and narrowing would need a new selection feature with one holder.
- **F8.1.** Confirmed, but `Require $.Number Singular` alone loses 21 faces: Oracle gives plural
  agreement to 25 names (*Whenever Bebop & Rocksteady attack*, *When The Notary Hobbits enter*)
  and singular to others of plural form (*Borborygmos and Fblthp enters*). A name's Number is
  lexical (CGEL, Ch. 5 §20.2.1, p. 517): every Catalog name Lexeme carries `Number` (singular
  unless `catalogs/plural_names.ron` lists it), and the grammar's `Name` declares `Number`.
- **F8.2.** Refuted. With one Complement the shifted-object draft is itself dropped (it repeats
  the plain order), and `interleaved1` is the only route for the Object postposed past an
  interleaved Adjunct alone (ADR §4.5 "Heavy Object shift"; CGEL, Ch. 16 §4, p. 1384 [7i]):
  removing it leaves Distended Mindbender and Global Ruin with no Reading. Kept.

**Census delta** (`wwnnpkls`): faces read 32,825 → 32,825 of 32,828; Readings 340,496 → 328,612 (196 units down, none up; Remove Enchantments 19,596 → 9,600); lost faces: none.
**Judgment set** (`wwnnpkls`): all hold (`cargo test -p deckmaste_english_v4 --test judgments`, 6 passed).
