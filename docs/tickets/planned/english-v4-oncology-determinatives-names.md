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
