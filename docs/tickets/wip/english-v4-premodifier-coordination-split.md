---
needs: []
---
# Withdrawn: *instant or sorcery spell* as [*instant*] or [*sorcery spell*]

This ticket proposed excluding a Nominal coordination whose last conjunct
begins with a noun Premodifier when the conjunct before it is a bare noun that
could itself be that Premodifier. The goal was to make *instant or sorcery
spell* read only as [*instant or sorcery*] *spell*.

## Owner's ruling: withdrawn

[*X*] or [*Y Z*] against [*X or Y*] *Z* is a genuine structural Ambiguity.
Only domain semantics separates the two, so both Readings stay. The planned
structural Ranking Key's conjunct-balance test (`english-v4-attachment-classes`)
already ranks [*instant or sorcery*] *spell* first. That is a Preference, not
an Admission change.

## Evidence

A prototype added an engine-derived rule (no new feature), and it removed the
only intended Reading of real text. In each case below, the split is what the
card means:

- **Satsuki, the Living Lore:** *Return target Saga or enchantment creature you
  control to its owner's hand.* The intended Reading is [*Saga*] or
  [*enchantment creature*]. A Saga creature is already an enchantment
  creature, so the distributive Reading would make *Saga or* redundant. No
  lexical or structural fact separates this text from *target Elf or Soldier
  creature*, where the distributive Reading is the intended one.
- **Puppet Crafting:** *Enchant artifact or non-Aura enchantment.* Two bare type
  words name permanents, and there is no carrier noun.
- ***target spell or nonland permanent*** (Commit, Press the Enemy, Sink into
  Stupor, Sudden Setback, Talion's Throneguard): the conjuncts are in
  different domains, a spell and a permanent. The rule could spare these only
  by declaring *spell* `HeadOnly`.
- **Psychic Paper:** *its name and creature type are the last chosen name and
  creature type.* The face lost every Reading.

So the ticket's premise was false: a bare type word coordinated with a
premodified nominal is sometimes exactly what Oracle means. On the census, the
prototype went from 295,697 to 281,688 Readings, Surplus from 149,101 to
140,984, and faces read from 28,709 to 28,708. It was abandoned.

## What stays

- `crates/deckmaste_english_v4/judgments/noun_premodifiers.txt:14` stays as it
  is. It endorses both Readings of *instant or sorcery card* as an Ambiguity
  witness, and that matches this ruling.
- Two defects the investigation found are ticketed separately:
  `english-v4-compound-noun-premodifier-coordination` (*artifact or non-Aura
  enchantment card*) and `english-v4-spurious-attribute-chain-reading`.

Withdrawn: no grammar change.
