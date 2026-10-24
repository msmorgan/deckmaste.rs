---
needs: []
---
# Pin both numeral forms and both token-combat orders in the judgment set

Oracle prints both forms of two alternations. The corrected style guide (see
`oracle-style-guide-mtg-rules-port`, hunks 2, 12 and 16) accepts both forms of
each and keeps each card's printed form:

- **Count nouns at one hundred and above.** Oracle spells out "one hundred"
  (Baldin, Century Herdmaster: "up to one hundred target creatures") and also
  prints digits (Helix Pinnacle: "100 or more tower counters"; Vexing
  Puzzlebox: "Remove 100 charge counters"; Battle of Wits: "200 or more cards";
  The Millennium Calendar: "1,000 or more time counters").
- **Token-combat order.** Prenominal: Pugnacious Pugilist's "a tapped and
  attacking 1/1 red Devil creature token"; Furnace-Blessed Conqueror's "a
  tapped and attacking token that's a copy of it". Postposed: Launch the
  Fleet's "Soldier creature token that's tapped and attacking"; Flamerush
  Rider and Gyrus, Waker of Corpses: "a token that's a copy of … and that's
  tapped and attacking". Untapped placement: Brimaz, King of Oreskos ("token
  with vigilance that's attacking", "that's blocking that creature"); Flash
  Foliage ("token that's blocking target creature attacking you").

On 2026-10-21, `cargo xtask english-v4 parse` read every one of these forms.
The grammar already accepts both forms, and the round-trip law
(`render(v) = s`, `docs/decisions/english-lexical-analysis.md` §1) keeps each
card's printed form, because the judgment evaluator checks it. What is missing
is pins. `coordinated_participial_premodifier.txt` pins the prenominal order,
and `coordination.txt` pins only a plural postposed copy. No entry pins the
digit and word forms of a count noun at one hundred or above, the singular
postposed copy, or an untapped `that's attacking` / `that's blocking` token
outside a named token.

## Change

- Add `+ <card=…>` entries for the witnesses above to the matching topic files.
  Numerals go in `scalar_cardinals.txt` or `number_and_countability.txt`. Token
  orders go in `coordinated_participial_premodifier.txt` and
  `relative_clause_kinds.txt`. Each entry asserts the intended analysis:
  - the cardinal (digit or spelled) as the head noun's quantity, as in
    "100 … counters" and "one hundred … creatures";
  - for the prenominal order, `tapped and attacking` as the Modifier of
    `token`;
  - for the postposed order, `that's tapped and attacking` /
    `that's attacking` / `that's blocking …` as a RelativeClause modifying the
    token NounPhrase. In the copy forms, the combat clause is the second of two
    coordinated relatives.

  Assert a reading count only where exactly one analysis is endorsed.
- Vexing Puzzlebox's "Remove 100 charge counters from this artifact" had two
  derivations on 2026-10-21. Pin the endorsed attachment of `from this
  artifact`, the complement of *remove*. If the second derivation is not
  endorsed, fix the grammar rather than pinning a count that includes it.
- If any entry fails, fix the grammar or catalog. Do not weaken the entry.

Census delta: none; judgments only, no grammar or catalog change, no face lost.
Judgment set: all hold (`cargo test -p deckmaste_english_v4 --test judgments`). Added 5 numeral entries and 6 postposed token-combat entries; the prenominal order was already pinned. Vexing Puzzlebox pins `from this artifact` as Complement of remove with no count, like the sibling remove entries: the Modifier-of-counters derivation is a grammatical PP attachment ranked second, so no grammar change.
