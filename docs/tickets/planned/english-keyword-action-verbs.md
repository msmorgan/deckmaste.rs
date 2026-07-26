---
needs: []
---
**Keyword-action verb family — `mutate` DONE (round kwverbs, 2026-07-25);
scope corrected; remainder split out.** The original claim ("111 groups
blocked purely by missing verb lexemes") overstated the family ~2×: of E3's
111 groups only 55 were the named lexemes (mutate 31, exploit 23, champion
1). Modeling ruling: these are catalog-backed `Verb::KeywordAction`
members via the hand-curated `ABILITY_DERIVED_KEYWORD_ACTION_VERBS`
supplement in `catalog.rs` — NOT `word.rs` vocabulary — because
Champion/Exploit/Mutate are keyword ABILITIES, so Scryfall's
keyword-actions catalog will never contain their verb forms and a data
refresh must not drop the hand-added entries (durability comment at the
merge site). `mutate` landed (−32/−576). `exploit` was attempted and
REVERTED: on Henry Wu, InGen Geneticist's coordinated subject, `have
exploit`'s keyword-ability atom flipped into a wrongly-coordinated verb
reading (permanent regression-guard test in tests/public_api.rs) —
follow-up in english-ability-derived-verb-batch. The E3 tail (56 groups)
is NOT lexeme-blocked: ~8 are `you're dealt damage` contracted-subject
recipient-passive residue (english-contracted-subject-recipient-passive),
~8 are a coordevent classifier artifact (Oxford-list events split at the
first top-level comma — any E3 re-derivation should split at the LAST
top-level comma before the effect), ~5 are the deferred verbs, rest
miscellaneous (coin flips, clashes, die rolls, `complete a dungeon`,
`{X} in its mana cost`).
