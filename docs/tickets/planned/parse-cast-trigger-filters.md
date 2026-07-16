---
needs: []
---
Parse spell-quality filters in cast-trigger heads. `Whenever you cast a
creature spell, …` presumably parses today for simple types; the failing forms
are compound/qualified: `an instant or sorcery spell` (type disjunction),
`a Spirit or Arcane spell` (subtype disjunction), `a spell that targets ~`
(heroic's head — 29 one-away cards on its own), `a spell with mana value N or
greater`, `a noncreature spell` variants across subjects (`you` / `a player` /
`an opponent`), and `cast or copy`.

The event-filter master forms exist (`core-eventfilter-master-forms`, done)
and trigger-subject filters landed via `gen-trigger-subject-filters` (done);
this is the spell-OBJECT filter vocabulary in the triggered-ability parser
(`crates/deckmaste_migrations/src/parsers/triggered_ability.rs`), reusing the
existing quality-filter parsers rather than new per-phrase arms.

**~577 of 17,022 one-away cards** (2026-07-16 tally; 756 one-away cards fail
ONLY on their trigger head, this family is the largest slice).

Verify: `cargo xtask generate plugins/wizards` graduation delta; a heroic
card, a Spirit-or-Arcane card, and an mv-threshold card round-trip via
`cargo xtask fidelity`.
