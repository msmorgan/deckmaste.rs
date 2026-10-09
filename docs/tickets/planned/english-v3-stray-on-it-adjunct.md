---
needs: [english-v3-per-line-readings]
---
# *on it/them* is never a stray VP or clause adjunct: *with N counters on it* is one unit

Priority: medium. This is the second of three attachment restrictions in the
owner decision of 2026-10-09, after `english-v3-trailing-adjunct-height` and
before `english-v3-temporal-pp-and-premodifier-scope`. It runs only after the
per-line census (`english-v3-per-line-readings`) lands.

The retained-Reading contract in `docs/decisions/english-lexical-analysis.md`
still holds. Only Readings that Oracle's conventions exclude may be retired.
Every attested counter-example is a named control.

## Why

*on* declares `PrepositionFunctionLicence:
"Modifier|VerbComplement|Adjunct|PredicativeComplement|PreposedAdjunct"`
(`crates/deckmaste_lexical_source/lexicon/core.ron:221`). With that licence,
the *on it/them* that closes *with N counters on it* is also free to attach
as:

- a VP Adjunct (`PrepositionPredicate`), over *cast*, *exile*, *enters*, *may
  cast* and similar;
- a Clause Adjunct (`ClausalPreposition`), over the whole sentence.

The 2026-10-09 decomposition measured a ×3–4 factor (`f_split`) on Goliath
Daydreamer, Nissa of Shadowed Boughs and Eluge, the Shoreless Sea.

Oracle convention: *on it/them* after a counter Noun Phrase locates the
counters. *with N counters on it* is one unit, and the PP belongs to the
counter NP. At minting,
`cargo xtask english-v3 grep '\bwith [^.,]{0,40}counters? on (it|them)\b'`
matches 912 faces, and `\bon (it|them)\b` matches 1,500. Two corpus examples:

- Faithful Watchdog, "This creature enters with three +1/+1 counters on it."
  (30 faces carry this exact wording).
- Grolnok, the Omnivore, "You may play lands and cast spells from among cards
  you own in exile with croak counters on them."

Attested non-adjunct uses stay. Verb-selected *on it* is one: *put … on it*,
*are put on it*, 14 faces. VP-level *on it* with *have* is another, and the
probes show it is the correct Reading:

- Felisa, Fang of Silverquill, "… if it had counters on it, … where X is the
  number of counters it had on it.": *on it* attaches to *had* inside the
  Object-gap relative.
- Crystalline Giant, "… a kind of counter … that this creature doesn't have on
  it …".
- Aven Courier, "… if it doesn't have a counter of that kind on it.".
- Eluge, "… for as long as it has a flood counter on it.".

The NP postmodifier itself stays: *a counter of that kind on it* (Quarry
Hauler, Dramatist's Puppet, Aven Courier, Blue, Loyal Raptor) and *all damage
marked on it* (Pyramids). The minting brief named "heal all damage on it" and
"another Role on it" as controls. Neither is attested: `grep` returns 0 faces
for `damage on (it|them)` and for `Role… on`. The controls below use attested
text instead.

## Goal

An *on* PP with a personal-pronoun complement is retired as a free VP Adjunct
or Clause Adjunct (`PrepositionPredicate`, `ClausalPreposition`). It keeps:

- its NP-postmodifier function (`PostmodifiedNominal`);
- verb-selected Complements;
- the *have* + Object + *on it* VP use that Felisa, Crystalline Giant, Aven
  Courier and Eluge attest.

## Analysis

Use declared features and tables only. Add no new Construction. A guard
naming *on*, *it*, *them*, *have* or a card is a STOP.

- The licence sites are the `PrepositionPredicate` schema
  (`crates/deckmaste_english_v3/src/declarations.rs:1594–1601`, which requires
  `licence_adjunct`, rows `:3827–3834`) and `ClausalPreposition`
  (`:2357–2365`). Both read the PP's exported `PrepositionFunctionLicence`
  (tables `:143–151`).
- **Withdraw the Adjunct licence for a pronominal complement.** Give the
  preposition a declared licence for pronominal complements, for example a
  second licence set on *on* that excludes `Adjunct` and `PreposedAdjunct`.
  The PP Construction resolves its exported licence through a table over (the
  head's licences, the complement's declared pronominality). The complement
  side is the existing case-pronoun NP categories (`AccusativePronoun` in the
  probe trees), exported as a declared feature, never matched by word.
- **Keep the *have* use.** Today Felisa's correct Reading attaches *on it* as
  a free Adjunct of *had* (`PrepositionPredicate<Have>`). `core-verb:Have`
  declares only its Auxiliary frame (`core.ron:1303–1305`). Choose one:
  - give *have* a declared Predicate frame with an Object and a locative
    Complement, which *on* reaches through its existing `VerbComplement`
    licence; or
  - give *have* a declared host licence that the `PrepositionPredicate` row
    reads.

  Whichever you choose, every control below must keep its *have* Reading. If
  neither route works without a word guard, STOP.
- Add any new feature name to `docs/contexts/oracle-english/CONTEXT.md`, next
  to **Preposition Function Licence** (`:523`).

## Witnesses

Before is a Sentence-category probe at minting, using the provisioned
`cargo-xtask` with trunk `zunvpkot`'s lexicon. After is the expected count
from a tree classifier that drops every Reading in which *on it/them* is the
modifier of `ClausalPreposition`, or of a `PrepositionPredicate` whose host is
not headed by *have*. It is an estimate, not a grammar run. If
`english-v3-trailing-adjunct-height` has landed, re-probe the before counts
first; Clause-level attachments will already be gone.

Counts must FALL:

1. Goliath Daydreamer, "Whenever this creature attacks, you may cast a spell
   from among cards you own in exile with dream counters on them without
   paying its mana cost.": 1,194 → 535.
2. Goliath Daydreamer, "Whenever you cast an instant or sorcery spell from
   your hand, exile that card with a dream counter on it instead of putting it
   into your graveyard as it resolves.": 1,050 → 476.
3. Eluge, the Shoreless Sea, "The first instant or sorcery spell you cast
   each turn costs {U} less to cast for each land you control with a flood
   counter on it.": 1,720 → 720. This is the Oracle sentence without the
   parenthetical "(or {1})". The full sentence reads 0.
4. Nissa of Shadowed Boughs, "You may put a creature card with mana value
   less than or equal to the number of lands you control onto the battlefield
   from your hand or graveyard with two +1/+1 counters on it.": 1,260 → 574.
5. Grolnok, the Omnivore, the sentence above: 1,533 → 643.

Also expected:

- Faithful Watchdog, the sentence above: 5 → 2. The 2 attach *on it* to
  *counters*.
- Grolnok, "Whenever a permanent card is put into your graveyard from your
  library, exile it with a croak counter on it.": 27 → 9.
- Eluge, "It's an Island in addition to its other types for as long as it has
  a flood counter on it.": 24 → 14. The 7 Readings that attach *on it* to
  *has* and the 7 that attach it to *counter* stay.
- Felisa's sentence above: 280 → 200. Every surviving Reading attaches each
  *on it* to *had* or to *counters*. The 80 retired Readings attach the final
  *on it* over *is the number …* or over the Clause.

Controls that must NOT change:

- Faithful Watchdog, "Put a +1/+1 counter on it.": 1 (verb-selected).
- Goliath Daydreamer, "Exile that card with a dream counter on it.": 9 → 4.
  The 3 Readings that attach *on it* to *dream counter* and the 1 that
  attaches it to *card with a dream counter* stay.
- Within Felisa and Eluge, the counts of the *have*-attached Readings named
  above.

## Out of scope

- The far NP attachment [[cards … with dream counters] on them]
  (`PostmodifiedNominal` over a head that already ends in a *with* PP). The
  decomposition's `f_split` counted it, so these after counts are higher than
  its ×3–4. It is NP postmodification and belongs to the far-attachment class
  (`c_far`), which has no ticket yet.
- The lexical licence of *on* with non-pronominal complements (*on target
  creature*, *combat on your turn*). The ADR leaves *combat on your turn* as
  an open question.
- Unread controls: Quarry Hauler, Aven Courier, Blue, Loyal Raptor and
  Pyramids read 0 on the probed sentences at minting. They are not count
  controls, and they must not become gains through a wrong analysis.

## Method

Standard constraints apply. Deltas:

1. **Probe first.** Probe every witness and control, write them as tests, and
   run the tests once at the end.
2. **Bounded census.** Selector: faces matching `\bon (it|them)\b` (1,500 at
   minting), taken from `cargo xtask english-v3 grep … --json` as an
   `--identity-manifest`. Union it with 300 random covered faces from `diff
   --manifest … --random 300 --seed <S>`, and record the seed. Run the claim
   parent and the feature on the same manifest.
3. **Compare.** Use `cargo xtask english-v3 diff --lines`.
   - Zero lost faces. Any loss is an attested Adjunct use: STOP and name it.
   - Name every decrease with the Reading shape it lost: VP Adjunct over
     *X*, or Clause Adjunct. Give counts by class in the record and names for
     the top 20.
4. **Measure.** Run under `/usr/bin/time -v` with `--workers 6`, and record
   wall time and peak RSS.

## Landing record

The six items in `docs/tickets/README.md` ("Landing record"), plus the per-line Readings total before and
after on the manifest.
