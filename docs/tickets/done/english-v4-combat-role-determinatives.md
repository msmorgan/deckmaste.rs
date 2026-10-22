---
needs: []
---
# Combat roles are determinatives, as *enchanted* is

## Ruling

Owner ruling, round 9 (2026-10-22), correcting an earlier ruling: *attacking
player* and *defending player* behave exactly like *enchanted creature*. The
first word is a determinative, and the phrases use the same mechanism.

## Evidence

- **The rule.** [CR#506.2]: "During the combat phase, the active player is the
  attacking player; ... the nonactive player is the defending player". It
  defines each role as a term for one player in a game relation, as
  [CR#301.5a] defines "the equipped creature" ("The creature an Equipment is
  attached to is called the 'equipped creature'"). Nothing in the rule makes
  the phrase a compound noun.
- **The precedent.** `keyword_abilities.ron` declares Enchant with
  `determinatives: [(lemma: "enchanted")]` [CR#702.5], and Equip and Fortify
  the same way. The host "is called enchanted" [CR#303.4b].
  *Enchanted creature* parses as determinative + Nominal. After
  another determinative, the same spelling is the participle of the verb
  *enchant* (*the enchanted creature*, *target enchanted creature*).
- **Oracle uses both kinds.** In the Oracle Cards snapshot, *defending player*
  is bare 250 times (*defending player loses 1 life*, *defending player's
  hand*), and also follows a determinative (*the defending player* 3, *each
  defending player* 3; the judgment *target defending player*). *Attacking
  player* is bare (*attacking player*) and follows *the* (*Rather than the
  attacking player*, Defensive Formation). Like *enchanted*, each word needs
  both analyses.
- **What it replaced.** The phrases were two multiword nouns in `nouns.ron`
  marked `distribution: BareSingular`. That marking had its own grammar: the
  `NounDistribution` value `BareSingular`; the Productions `BareSingularTerm`
  and `BareSingularPossessor`; and `Exclude`s in `SingularNominalQuality` and
  `BarePrepositionalNominal`. A determinative makes all of it unnecessary.
  `Determined` builds the bare Noun Phrase, and `DeterminedPossessor` builds the
  genitive (*defending player's*).
- **No verb *defend* existed.** `nouns.ron` noted "no verb `defend` stands
  behind `defending`". *The defending player* and *each defending player* need
  the participle, as *the enchanted creature* needs *enchant*. Oracle uses no
  other form of *defend*.

## Change

- `catalogs/turn_parts.ron`, the turn-structure catalog, gains the entry
  `(rule: "[CR#506.2]", determinatives: [(lemma: "attacking"), (lemma:
  "defending")])`. A Turn Part's `noun` becomes optional, and it gains
  `determinatives`, as a Keyword Ability has. Its Lexemes are
  `turn_part_det:attacking` and `turn_part_det:defending`, with Catalog
  provenance owned by `506.2`.
- `verbs.ron` declares *defend* with an Intransitive frame. The frame licenses
  its attributive gerund-participle, as for *attack* (round-5 ruling, item
  7b).
- Retired: the nouns *attacking player* and *defending player*; the
  `NounDistribution` value `BareSingular` (`english.ron` and `schema.rs`);
  `BareSingularTerm`; `BareSingularPossessor`; and the two `Exclude`s.
- The `deckmaste_lexical_source_v4` README id table lists `turn_part_det:`.

## The collision, measured

*Attacking* is also the participle of *attack*. As with *enchanted*, both
analyses stay; neither suppresses the other. The determinative cannot follow
another determinative, so *the attacking player*, *each defending player* and
*target attacking creature* have only the participle reading.

Where *attacking* has no determinative before it, the determinative reading
joins the participle reading. In a bare plural (*attacking creatures you
control*), that gives two Readings where there was one, as *enchanted
creatures* and *equipped creatures* already have. The same holds where an
existing verb reading of *target* (*to target [attacking creature]*) takes a
bare Object. These readings are syntactically well formed, and telling them
apart by meaning belongs to `deckmaste_semantics_v3` ([`english-v4.md`]
(../../decisions/english-v4.md) §4.4, item 6).

- 163 Corpus Units grow and 8 shrink, and no verdict changes.
- The largest growth is Cosmic Cube (*attacking creatures you control*),
  720 → 1,440. The largest ratio is Turn the Tables (*to target attacking
  creature*), 2 → 10. No unit grows pathologically.
- The 8 units that shrink are *target X defending player controls* units
  (Orcish Squatters 18 → 12). They lost readings that the removed
  `BareSingular` Productions built. Each keeps its endorsed relative-clause
  Reading.

## Tests

- Added to `deckmaste_lexical_source_v4/tests/environment.rs`:
  `combat_role_determinatives_retain_verbal_homographs`. It checks that
  *attacking* and *defending* are catalog determinatives owned by [CR#506.2]
  and also gerund-participles of a verb, and that *attacked*, *defended* and
  *blocking* are not determinatives.
- Added to `judgments/oracle_determinatives.txt`, which
  `catalog_determinative_judgments_hold` enforces:
  - the bare determinative: *defending player*, *attacking player*;
  - the genitive: *defending player's hand*;
  - a sentence: Agate-Blade Assassin;
  - the participle after a determinative: *the attacking player*, *each
    defending player*, *target defending player*.
- Rewritten: `tests/cr_coverage.rs` `phases_and_steps` reads only the Turn
  Parts that name a noun.
- Rewritten: the header comment of `judgments/lexical_compositions.txt`. It
  named *defending player* as a noun premodifier, which it no longer is.
- Retired: none. The existing *defending player* and *attacking player*
  judgments hold unchanged (`grammar.txt`, `lexical_compositions.txt`,
  `clause_supplements_and_adjuncts.txt`, `trigger_clauses.txt` and others).

## Acceptance

- No face is lost.
- Every judgment's positive assertions hold.
- Nothing ungrammatical is admitted.
- `cargo xtask english-v4 strata --check` passes.
- No pathological forest explosion on real cards.

The cheapest-tier figures and the judgment-set S/A/P/F are reported as
statistics, not gates:

- Cheapest tier: a singleton in 15,824 of 17,634 ambiguous units (summed
  20,053, largest 8) before, and in 15,920 of 17,732 (summed 20,155, largest
  8) after.
- S/A/P/F: 518/148/9/79 before, 519/148/9/79 after. The after figure includes
  the new judgments.

Standard constraints apply.

1. **Census delta** (measured on `unwpsuqt` → `znouxquo`): faces read 28,800 →
   28,800; total Readings 237,320 → 238,489; Surplus 101,963 → 102,418; lost
   faces: none.
2. **Judgment set**: 3,049 enforced at `COMPLETED_STRATUM` 6, all hold; 205 pending.
