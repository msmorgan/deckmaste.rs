---
needs: []
---
**"Champion an [object]" has no keyword-line Reading, and its spelling drops
the article.** Found by `semantics-v2-subject-param-kind` (2026-10-10).
Champion now declares its description `Predicate` and forwards it as
`KeywordParam::Subject`, with its definition body written [CR#702.72a], but
the English side still lacks the article the CR template prints:

- english_v3's `SubjectKeyword` reads `head " " subject: Nominal`, which
  admits "Enchant creature you control" but not "Champion an Elemental": the
  13 supported faces whose text contains "Champion " (Boggart Mob,
  Changeling Berserker, Changeling Hero, Changeling Titan, Lightning
  Crafter, Mistbind Clique, Nova Chaser, Silvar, Devourer of the Free,
  Supreme Exemplar, Thoughtweft Trio, Unstoppable Ash, Wanderwine Prophets,
  Wren's Run Packmaster) read 0 (measured at `xwuuqwvw`,
  `cargo xtask english-v3 --text-contains "Champion "`).
- `champion.ron`'s `spelling` is `"champion <Param(0)>"`, which realizes
  `champion(hasSubtype(elemental))` as "champion Elemental"; the spelling
  template has no indefinite-article modifier ("a"/"an" by the realized
  word).

The keyword's article must come from a declared feature of the declaration
(its `grammar` parameter recipe, as affinity's `Quality(preposition: …)`
declares "for"), never a guard naming the keyword. Prior attempts and the
declaration-only versus referable distinction are in
`docs/tickets/done/english-keyword-ability-parameters.md`. Standard
constraints apply; the census delta covers the 13 champion faces.
