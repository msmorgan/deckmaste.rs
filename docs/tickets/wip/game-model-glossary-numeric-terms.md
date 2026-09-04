---
needs: []
---
**Define every rules concept the Game Model glossary uses, with its citation,
instead of deferring to the Comprehensive Rules by omission.** Ruling
2026-09-04: the glossary (`docs/contexts/game-model/CONTEXT.md`) defers to
the CR for rules concepts but is imprecise about which it defines and which
it only names; it mentions Amount once, in passing, and has no entry for
Delta, Duration or Quantity while `Delta` and `Modify` become core rows
(`workbench-delta-modify`). Duplicate each concept as a glossary entry with
its CR citation where the CR defines it, or mark it a project term where it
does not.

- Add, beside Static Spec: **Amount** (a semantic expression denoting a
  number in context — literal, count of a Selection, characteristic read;
  not a Reference); **Quantity** (the count constraint a determiner states
  over a Selection — "one", "up to two", "one or more", "any number"; not an
  Amount); **Delta** (a change stated against a numeric property: up by an
  Amount, down by an Amount, or set to an Amount; the engine derives the
  event — [CR#613.4c] for characteristic modification, [CR#119.3]
  for life); **Duration** (the span a Continuous Effect created by a
  one-shot lasts [CR#611.2a]; a Static Spec put in force for a Duration;
  avoid: span, window).
- Audit: every capitalised term the glossary uses without an entry, and
  every core sort and row name in `deckmaste_core` and the workbench that
  names a rules concept (counter, damage, life, mana, cost, phase and step,
  turn, priority, …); each gets an entry with a citation, or a project-term
  tag with its avoid-list. Oracle English's glossary gets a cross-reference
  for Quantity, since determiners are its side.
- `Window` (the anaphora read scope) is an internal term, not a glossary
  entry.

Size: S–M. Done when: no term the glossary or the core uses lacks an entry;
each rules concept carries a citation whose text the entry restates;
`cargo xtask cite check` clean. Standard constraints apply.
