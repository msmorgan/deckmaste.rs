---
needs: []
---
# Spurious asyndetic Attribute chains over coordinated noun Premodifiers

`AsyndeticAttributes` (`english.ron`, the `Attribute` productions) builds a
comma chain of premodifiers that share one head, as in *nonartifact, nonblack
creature* (`docs/oracle-style-guide.md` §6, Determiners and modifier order:
"If independent negative modifiers stack before one head, separate them with
commas"). The chain also matches inside a serial coordination, where its
commas are the coordination's commas. That gives a Reading nobody endorses.

## Witnesses

- *a Bat, Bird, Mouse, or Rabbit card* (Lupinflower Village: *You may reveal a
  Bat, Bird, Mouse, or Rabbit card from among them …*) has 3 Readings:
  - the serial noun Premodifier coordination [*Bat, Bird, Mouse, or Rabbit*]
    *card*, which is intended;
  - the serial Nominal coordination [*Bat*], [*Bird*], [*Mouse*], or [*Rabbit
    card*], a genuine Ambiguity (see the withdrawn
    `english-v4-premodifier-coordination-split`);
  - the spurious one: the Attribute chain *Bat*, [*Bird, Mouse, or Rabbit*],
    whose last Attribute is a noun Premodifier coordination.
- *a Swamp, Mountain, black permanent, or red permanent*
  (`asyndetic_premodifier_coordination.txt:10`) has 2 Readings:
  - the intended serial Nominal coordination [*Swamp*], [*Mountain*], [*black
    permanent*], or [*red permanent*];
  - the spurious one: [*Swamp*], [*Mountain, black permanent*], or [*red
    permanent*]. In it, the Attribute chain *Mountain, black* premodifies the
    first *permanent*.

Oracle punctuates a premodifier chain with commas only between independent
modifiers of one head, and never ends the chain with a coordinator. A chain
ending in a coordination is a serial coordination (style guide §3, "Commas
and the serial comma").

## Change

- Find the over-broad Production or Schema that lets an Attribute chain:
  - end in a coordination of Premodifiers;
  - take a Type Word noun Attribute beside an adjectival one inside a serial
    coordination.
- Tighten or delete it, using existing features or an engine mechanism.
- Keep the Readings of `asyndetic_premodifier_coordination.txt`: *nonartifact,
  nonblack creature*, *noncreature, nonland card*, *non-Vampire, non-Werewolf,
  non-Zombie creature*.

## Done when

- Both witnesses lose the spurious Reading. Each is pinned by a `+` Judgment on
  real text that asserts the intended analysis and a `!` assertion for the
  chain.
- Every Judgment in `asyndetic_premodifier_coordination.txt` still holds.
- No face is lost, and no unit gains a Reading.

## Constraint: no new features in `english.ron`

The fix adds **zero new features to `english.ron`**. It uses existing
features, lexicon or catalog frames, or an engine mechanism, and names no word
in the grammar. If it cannot be done that way, STOP and return to the owner.

Standard constraints apply.

Census delta (measured on `nqpprzws` → this landing, `mtlpovxn`): faces read 28,706 → 28,706, total Readings 218,547 → 218,063, Surplus 91,328 → 91,191; lost faces none.
Judgment set: all hold (`cargo test -p deckmaste_english_v4`), including the two new witness entries in `asyndetic_premodifier_coordination.txt`, which fail without the change (`mtlpovxn`).
