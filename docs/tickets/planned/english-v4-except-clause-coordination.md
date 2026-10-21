---
needs: [english-v4-comma-adjunct-island]
---
# A comma-delimited Clause coordination after *except*, and a Supplement-final first conjunct

## Witnesses

- **Astral Dragon:** *When this creature enters, create two tokens that are
  copies of target noncreature permanent, except they're 3/3 Dragon creatures
  in addition to their other types, and they have flying.*
- **Hulking Metamorph:** *You may have this creature enter as a copy of an
  artifact or creature you control, except it's an artifact creature in
  addition to its other types, and its power and toughness are equal to this
  creature's power and toughness.*
- **Aspect of Wolf:** *Enchanted creature gets +X/+Y, where X is half the
  number of Forests you control, rounded down, and Y is half the number of
  Forests you control, rounded up.* It is deferred here from `lxzvuwqu`
  (`english-v4-bare-relative-fronted-adjunct`). Before that change all 4 of
  its Readings were wrong: each put the comma-set Supplement inside a bare
  relative (*Forests [you control, rounded down]*). A bare relative's Clause
  now opens with its Subject and holds no comma-set dependent (CGEL, Ch. 12
  §3.5.5, p. 1055; Ch. 12 §4, p. 1058), so it has none. Its judgment
  (`slash_measure_selection.txt`, the Aspect of Wolf entry) is pending while
  it is deferred.
- **The pending `where_variable_clause.txt` entry** *Search target player's
  library for X cards, where X is the number of cards in your hand, and exile
  them. Then that player shuffles.* It has 0 Derivations.

On `pzvrlpou`, Astral Dragon has 7 Derivations and Hulking Metamorph 84, none
of them the intended analysis. Under the `english-v4-comma-adjunct-island`
candidate both have 0, so both faces go unread when that ticket lands (round
3, item 3.2). If `deferred.txt` names them, this landing removes those
entries.

## Intended analyses

- *Except* takes the coordination [A, and B] as its finite Clause Complement.
  - Copy exceptions stay in one sentence (`docs/oracle-style-guide.md` §12,
    Copies; [CR#707.9]).
  - A comma before *and* joins independent Clauses (§3, Commas and the serial
    comma).
- In the *Search* entry, [Search … for X cards, where X is …] is the first
  conjunct of a Clause coordination with *exile them*. The *where* phrase is a
  Supplement of the first conjunct (§3: "around a trailing variable
  definition").

## Causes

1. `FiniteComplementClause` (`english.ron`) has `undelimited: ["head"]`. A
   Clause coordination delimited before its coordinator therefore cannot be a
   preposition's finite Clause Complement, which blocks *except* [A, and B].
   - Find what that restriction protects by measuring the census without it.
   - Narrow it, rather than delete it, if it keeps real spurious Readings off.
2. `step.rs` `finish` sets `set_off` on any Production whose second child is a
   comma.
   - `set_off` exists for a comma-fronted dependent (*When …, …*), which
     scopes over a whole coordination and never over its first conjunct.
   - A host-first [head Clause, `,`, adjunct] Production, the comma-set
     post-adjunct or Supplement, also has a comma second. It is marked set off
     too, so it cannot be a first conjunct.
   - The island candidate adds a `fronted` summary flag: the head is last, with
     a comma before it. That is the distinction `set_off` needs.
   - This is also why Aspect of Wolf has no Reading. Its intended first
     conjunct [X is half …, rounded down] ends in `ParticipialSupplement`
     ([head Clause, `,`, Supplement]), which `set_off` marks. Measured on
     `lxzvuwqu`: *X is half the number of Forests you control, rounded down*
     as a Clause has 2 Derivations; *…, rounded down, and Y is half the number
     of Forests you control, rounded up* has 0. With *, rounded down* removed
     from the first conjunct, it has 8.

## Known trap

A naive `set_off` fix raised Aspect of Wolf from 4 Readings to 44, all the new
ones wrong:

> Enchanted creature gets +X/+Y, where X is half the number of Forests you
> control, rounded down, and Y is half the number of Forests you control,
> rounded up.

Once [Clause, where X is …] may be a first conjunct, *, and Y is …* can
coordinate with the whole sentence. It belongs inside the *where* phrase. Those
Readings are wrong. Aspect of Wolf must read through the intended analysis
alone.

## Done when

- Astral Dragon and Hulking Metamorph read. `+` Judgments assert the
  coordination as the Complement of *except*, for example
  `Clause "they're 3/3 Dragon creatures in addition to their other types, and
  they have flying" as Complement of except`.
- The *Search … where X is …, and exile them.* entry holds.
- Aspect of Wolf reads, and every Reading has the intended analysis: the
  coordination [X is …, rounded down, and Y is …, rounded up] is inside the
  *where* phrase. Its deferral is removed. Its pending judgment holds, and a `+`
  judgment asserts `Clause "X is half the number of Forests you control,
  rounded down, and Y is half the number of Forests you control, rounded up"`
  inside the *where* phrase.
- No new feature in `english.ron`. If the fix needs one, STOP and return to
  the owner.
- No face is lost, and no unit other than the witnesses gains a Reading.

Standard constraints apply.

1. **Census delta**: measured at landing.
2. **Judgment set**: measured at landing.
