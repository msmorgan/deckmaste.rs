---
needs: [english-v3-per-line-readings]
---
# Trailing adjuncts attach at VP level: never over a sentence-initial trigger or intervening *if*; clause level only for clause coordination

Priority: medium. Owner decision (2026-10-09): three attachment classes are
restricted by declared feature, in this order, once the per-line census
(`english-v3-per-line-readings`) has landed:

1. this ticket;
2. `english-v3-stray-on-it-adjunct`;
3. `english-v3-temporal-pp-and-premodifier-scope`.

The retained-Reading contract in `docs/decisions/english-lexical-analysis.md`
("Chart admission and ambiguity", "Pruning rule for licensed functions and
attachments") still holds. A restriction may retire only Readings that
Oracle's conventions exclude. Every attested counter-example is a named
control and must keep reading.

## Why

A Clause-final Preposition Phrase or Nominal Adjunct (*this turn*, *until end
of turn*, an *if* PP) currently attaches at every height:

- at VP level, through the `PrepositionPredicate` and
  `NominalAdjunctPredicate` rows;
- at Clause level, through `ClausalPreposition`, including over an
  `InitialPreposition` host whose preposed dependent is the trigger
  (*Whenever …, …*);
- at VP level stacked over a predicate whose last dependent is an *if* PP.

The 2026-10-09 attachment decomposition measured this on eight high-count
faces. That evidence is the orchestrator's scratch and is not in the tree.

- **b-trig** (an Adjunct scoping over the sentence-initial trigger)
  multiplies a sentence's count by ×1.0–1.8 and occurs in 5.1% of sentences.
  The decomposition found no counter-example.
- **b-clause** (Clause-level attachment over a non-coordinated Clause) adds
  ×1.0–2.5. At either height the meaning is the same.

Oracle convention: a trailing Adjunct modifies the predicate it follows. Its
scope never includes the trigger condition or a preceding *if* condition. At
minting, 1,402 faces match
`cargo xtask english-v3 grep '\b(Whenever|When) [^.]*, [^.]* (if [^.]*|this turn|until end of turn)\.'`.
Two corpus examples:

- Belladonna Took: "Whenever a token you control enters, you gain 1 life if
  this is the first time this ability has resolved this turn." The *if* PP
  conditions *gain*; *this turn* belongs inside the *if* Clause.
- Vizier of Deferment: "When this creature enters, you may exile target
  creature if it attacked or blocked this turn." *this turn* belongs to
  *attacked or blocked*, not to the trigger or the *if*.

Same meaning at both heights: Veil of Summer, "Draw a card if an opponent has
cast a blue or black spell this turn."

CGEL (Ch. 6 §7, [5]–[7], p. 576, page-checked) lists temporal location and
duration among the VP-oriented adjuncts ([5v–vi]). It states that these are
"more closely associated with the VP constituents, and more likely to be
positioned in the VP or adjacent to the VP" ([7i]). Its clause-oriented list
([6]) has domain, modality, evaluation, speech-act-related and connective
adjuncts only. The passage is about AdvPs. Applying it to trailing PPs, to NP
adjuncts such as *this turn*, and to conditional *if* PPs is this project's
analysis and is not stated by CGEL. Cite it only for what it says.

## Goal

1. `ClausalPreposition` admits its dependent only when the host Clause is a
   Clause Coordination. The coordination may be direct, or reached through a
   stack of `ClausalPreposition`s over one. Clause-level attachment over a
   simple Clause or over `InitialPreposition` is retired. The VP-level rows
   carry the attachment instead.
2. A trailing Adjunct does not attach over a predicate whose last dependent
   is a clause-complement PP (*if*, *unless*, …). The Adjunct belongs inside
   that Clause.
3. Attachment above a modal is untouched (see Out of scope).

## Analysis

Line numbers were verified at minting against trunk `zunvpkot`, which does not
change `crates/deckmaste_english_v3/src/declarations.rs` relative to
`wnrmklzs`. Verify them against the claim tree.

- `InitialPreposition` `:2346–2355`, `ClausalPreposition` `:2357–2365`,
  `ClauseCoordination` `:2376–2382`. The Clause-series rows of
  `CoordinationSeriesEnd` start at `:2385`, with `ClauseSeries` at
  `:2412`/`:2453`/`:2490`.
- `PrepositionPredicate` schema `:1594–1601`, rows `:3827–3834`.
  `NominalAdjunctPredicate` schema `:1579–1581`, rows `:3714–3721`.
  `ClauseComplementPreposition` `:3308`.

Implement with declared features and tables only. Add no new Construction and
no guard naming a lexeme, Construction or card. If either item seems to need
one, STOP.

1. **Clause height.** Add a Clause feature recording whether the Clause is a
   coordination host, for example `{Coordinated, Simple}`:
   - `ClauseCoordination` and the Clause-series coordination rows export
     `Coordinated`;
   - `ClausalPreposition` forwards its `clause` value, so stacked Adjuncts
     over a coordination still read;
   - every other Clause Construction exports `Simple`. That includes
     `InitialPreposition`, so a trailing Adjunct can no longer scope over the
     trigger.

   `ClausalPreposition` requires `Coordinated`.
2. **Intervening *if*.** A PP-level feature marks a clause-complement PP. The
   source is `ClauseComplementPreposition`, or the head's declared
   `FiniteClauseComplement`/`VerblessClauseComplement` properties; it must
   not name *if*. The `PrepositionPredicate` rows export it as the host's
   last-dependent property. The `PrepositionPredicate` and
   `NominalAdjunctPredicate` rows then reject a modifier over a head
   carrying it, through a table.

Name both features from glossary terms. Add the entries to
`docs/contexts/oracle-english/CONTEXT.md` in the same landing; the glossary has
no term for adjunct height today.

## Witnesses

These are Sentence-category probes for single-sentence lines, so each line's
per-line count equals its sentence count. Before = probe at minting, using the
provisioned `cargo-xtask` with trunk `zunvpkot`'s lexicon. After = expected
count from a tree classifier over those Readings. The classifier drops every
Reading that has `ClausalPreposition` over a non-coordinated Clause, or a VP
Adjunct over a predicate ending in `ClauseComplementPreposition`. It is an
estimate, not a grammar run. If a sibling attachment ticket lands first,
re-probe the before counts.

Counts must FALL:

1. Goliath Daydreamer, "Whenever this creature attacks, you may cast a spell
   from among cards you own in exile with dream counters on them without
   paying its mana cost.": 1,194 → 323.
2. Goliath Daydreamer, "Whenever you cast an instant or sorcery spell from
   your hand, exile that card with a dream counter on it instead of putting it
   into your graveyard as it resolves.": 1,050 → 140.
3. Belladonna Took, the sentence above: 84 → 24. Of the 84, 48 carry a
   Clause-level attachment (24 of them over the trigger) and 12 are VP over
   *if* only.
4. Vizier of Deferment, the sentence above: 25 → 15.
5. Veil of Summer, the sentence above: 5 → 2.

Also expected: Wyluli Wolf, "Target creature gets +1/+1 until end of turn."
2 → 1; Suffocation, "Cast this spell only if you were dealt damage this turn
by a red instant or sorcery spell." 48 → 21; Scarlet Witch, Chaotic Avenger,
"Then you may cast a Hero or noncreature spell from among cards exiled with
Scarlet Witch without paying its mana cost." 410 → 164.

Controls that must NOT change:

- Suffocation, "Suffocation deals 4 damage to the controller of the last red
  instant or sorcery spell that dealt damage to you this turn.": 182.
- Teething Wurmlet, "If this is the first time this ability has resolved this
  turn, put a +1/+1 counter on this creature.": 24. The *if* PP is preposed.
- Skyrider Patrol, "When you do, put a +1/+1 counter on another target
  creature you control, and that creature gains flying until end of turn."
  goes 14 → 8 overall, but its 4 Readings that attach *until end of turn* over
  the Clause Coordination must all stay.
- Nissa of Shadowed Boughs, "You may have it become a 3/3 Elemental creature
  with haste and menace until end of turn." goes 22 → 15 overall. Of its 7
  Readings with the Adjunct above *may*, the 6 that have no Clause-level
  attachment must stay.

## Out of scope

- **b-modal.** Temporal Adjuncts above *may* are attested: Kaito, Dancing
  Shadow, "If you do, you may activate loyalty abilities of Kaito twice this
  turn rather than only once."; Urza Assembles the Titans, "You may activate
  the loyalty abilities of planeswalkers you control twice this turn rather
  than only once." `PrepositionPredicate` over `BareAuxiliaryPredicate`
  stays. Kaito's sentence reads 0 at minting, so it is not a count control.
- Preposed Adjuncts (`InitialPreposition` itself) and `Supplementation`
  (*, where X is …*).
- The other attachment classes, which have their own tickets.

## Method

Standard constraints apply. Deltas:

1. **Probe first.** Probe every witness and control before the change. Write
   them as tests. Run the tests once, at the end.
2. **Bounded census.** Selector: every face whose base per-line report lists
   `ClausalPreposition` in `constructions`, or lists
   `ClauseComplementPreposition` together with `PrepositionPredicate` or
   `NominalAdjunctPredicate`. Pass it as an `--identity-manifest`, unioned
   with 300 random covered faces chosen by `cargo xtask english-v3 diff
   --manifest … --random 300 --seed <S>`; record the seed. Run before (claim
   parent) and after on the same manifest. If the selector exceeds half of
   the supported faces, run `--all` instead and say so.
3. **Compare.** Use `cargo xtask english-v3 diff --lines` before → after.
   - Zero lost faces. A lost face is an attested use the restriction would
     wrongly exclude: STOP and name it.
   - Name every decreased face with the Reading shape it lost. Count the
     decreases by class (Clause-level over the trigger; Clause-level over a
     simple Clause; VP over *if*) in the record, and by name for the top 20.
4. **Measure.** Run each census under `/usr/bin/time -v` with `--workers 6`.
   Record wall time and peak RSS.

## Landing record

The six items in `docs/tickets/README.md` ("Landing record"), plus the per-line Readings total before and
after on the manifest.
