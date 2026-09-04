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

## As landed

Game Model, Magic rules language (18 entries, each restating its rule):
Token [CR#111.1], Emblem [CR#114.1], Color [CR#105.1], Status [CR#110.5],
Counter [CR#122.1], Keyword Action [CR#701.1], Special Action [CR#116.1],
Turn-Based Action [CR#703.1], State-Based Action [CR#704.1], Priority
[CR#117.1], Turn [CR#500.1], Phase [CR#500.1], Step [CR#500.1], Damage
[CR#120.1], Life Total [CR#119.1], Mana [CR#106.1], Cost [CR#118.1], Copy
[CR#707.2].

Game Model, Engine semantic language (5 entries): Amount, Quantity, Delta and
Duration beside Static Spec as the ticket words them, plus Designation — a
project term, because the CR names each designation on its own ([CR#701.15b]
goaded, [CR#702.131c] the city's blessing) without defining the class.

Oracle English (1 entry): Quantity, cross-referencing the Game Model
constraint from the Determiner side.

Audited: the 54 pre-existing Game Model entries for capitalised terms used
without an entry, and 305 sorts — 109 in `deckmaste_core` (`map enums`), 196 in
the workbench (`map idris` over `Words.idr`, `Phrase.idr`, `Effect.idr`).

Left without an entry, deliberately:

- `Window` — the anaphora read scope; internal per the ticket.
- Power, Toughness, Mana Value, Loyalty, Defense (core `Stat`, workbench
  `Stat`/`PlayerStat`) — the Characteristic entry enumerates them from
  [CR#109.3]; a per-characteristic entry would restate that list.
- Land, Activated Mana Ability, Triggered Mana Ability — capitalised values of
  classes that already have entries (Card Type, Mana Ability).
- Search, Shuffle, Reveal, Sacrifice, Attach and the rest of core `Action`'s
  verb rows — the new Keyword Action entry defines the class [CR#701.1];
  entries per verb would duplicate [CR#701].
- Core `Deontic`, `Condition`, `Region`, `Provenance`, `Kind`, `Expr`,
  `StatValue` and the workbench's parser-shape sorts — engine and grammar
  machinery, not names for rules concepts.

## Landing record

Gates (foreground):

- `cargo xtask cite check --list-noncompliant` → `0 non-compliant
  citation-looking string(s)`
- `cargo xtask cite bless` → `blessed 1428 rules at cr_date 2026-08-07`;
  newly registered [CR#106.1a,500.2,703.1,703.1a], each read against its
  entry. No entries pruned.
- `cargo xtask cite check` → `checked 14373 citations against cr.txt
  (eff. 2026-08-07); 0 stale`
- `jj --no-pager diff --git > /tmp/round.diff && cargo xtask cite audit --diff
  < /tmp/round.diff` → `audited 59 citation site(s) — read each rule text
  against its claim`; every site read, none off-topic.

Deviations and additions: Designation added beyond the ticket's enumerated
families — it is a core sort family (`DesignationDef`, `DesignationScope`,
`DesignationUniqueness`, `DesignationPersistence`) and a workbench sort naming
a rules concept, so the audit clause covers it. Life Total is the entry name
rather than Life, matching [CR#119] and core `PlayerAttr`/`LifeOp`. No STOP
taken.
