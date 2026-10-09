---
needs: [english-v3-per-line-readings]
---
# Temporal PPs never postmodify a noun; a premodifier scopes outside a postmodifier

Priority: medium. This is the third of three attachment restrictions in the
owner decision of 2026-10-09, after `english-v3-trailing-adjunct-height` and
`english-v3-stray-on-it-adjunct`. It runs only after the per-line census
(`english-v3-per-line-readings`) lands.

The retained-Reading contract in `docs/decisions/english-lexical-analysis.md`
still holds. Only Readings that Oracle's conventions exclude may be retired.
Every attested counter-example is a named control.

The ticket has two sub-items of the same shape: a NP-internal attachment that
duplicates the correct one.

## (a) Temporal PPs never postmodify a noun

### Status at minting: already landed on trunk

Trunk `lztruvkm` (integrated as `zunvpkot`, ticket
`docs/tickets/done/english-v3-as-clause-verbless-np-licensing.md`, user ruling
2026-10-09) removed `Modifier` from *until*'s `PrepositionFunctionLicence`.
`core.ron:202` now declares `"Adjunct|PreposedAdjunct"`.

That ruling reads: "*until* PPs in Oracle function as VP or Clause Adjuncts;
every nominal Modifier attachment is a wrong Reading". It is the convention
this sub-item asked for.

- No other temporal preposition declares `Modifier`: *after*, *before*,
  *during* and *at* (`core.ron:207–214`).
- The NP temporal Adjuncts *this turn* and *each turn* are
  `NominalAdjunctPhrase`. They attach only through `NominalAdjunctPredicate`
  (`declarations.rs:1579–1581`, rows `:3714–3721`) and never postmodify a
  Nominal.

So (a) needs no grammar change. What remains is verification:

- Every face matching `\buntil\b` has no `BareTemporalPreposition` as the
  modifier of `PostmodifiedNominal`.
- The (a) witnesses below already read their post-exclusion counts.

If the bounded census finds a nominal temporal attachment by another route,
retire it through that preposition's declared licence, under the ADR pruning
rule, with zero lost faces.

The decomposition measured `a_temporal` (×2.59 on Shadow Puppeteers) before
that landing.

- Convention examples: Wyluli Wolf, "Target creature gets +1/+1 until end of
  turn."; Halsin, Emerald Archdruid, "Until end of turn, target token you
  control becomes a green Bear creature …". The PP preposes, so it is a Clause
  or VP dependent.
- (a) witnesses, which must already read these counts. Before is on trunk
  `uxunykqk` with nominal *until*; after is on trunk `zunvpkot`, both probed
  at minting:
  1. Shadow Puppeteers, "Whenever a creature you control with flying attacks,
     you may have it become a red Dragon with base power and toughness 4/4 in
     addition to its other colors and types until end of turn.":
     3,028 → 1,660.
  2. Riddleform, "Whenever you cast a noncreature spell, you may have this
     enchantment become a 3/3 Sphinx creature with flying in addition to its
     other types until end of turn.": 626 → 320.
  3. Glint Hawk Idol, "This artifact becomes a 2/2 Bird artifact creature
     with flying until end of turn.": 21 → 11.
  4. Angel's Tomb, "Whenever a creature you control enters, you may have this
     artifact become a 3/3 white Angel artifact creature with flying until end
     of turn.": 55 → 40.
  5. Nissa of Shadowed Boughs, "You may have it become a 3/3 Elemental
     creature with haste and menace until end of turn.": 28 → 22.
- (a) controls: Wyluli Wolf's sentence 2, and Halsin's full sentence 260.

## (b) A premodifier scopes outside a postmodifier

### Why

Today an Adjective-phrase premodifier (`PremodifiedNominal`) can sit below a
postmodifier or relative clause as well as above it. Each NP with both
therefore reads twice:

- [first [spell you cast]] and [[first spell] you cast];
- [red [Dragon with …]] and [[red Dragon] with …].

The decomposition measured `g_premod` at ×1.0–3.0 per sentence: Saint Traft
×3.0, Shadow Puppeteers ×2.18, Hiveheart Shaman ×1.80.

Owner convention (2026-10-09): the premodifier scopes outside the
postmodifier. Keep [first [spell you cast]] and [red [Dragon with …]], and
never [[first spell] you cast] as a second Reading. Two corpus examples:

- Eluge, the Shoreless Sea, "The first instant or sorcery spell you cast each
  turn costs {U} (or {1}) less …". 19 faces match
  `\bfirst [a-z ]{0,30}spells? (you|each opponent|an opponent) casts?\b`.
- Shadow Puppeteers, "… become a red Dragon with base power and toughness 4/4
  …". 41 faces match
  `\b(a|an) (white|blue|black|red|green|colorless) [A-Z][a-z]+ [^.]{0,30}\bwith\b`.

**CGEL does not support this bracketing.** Its sample NP structure (Ch. 5 §2,
[11], p. 332, page-checked) is *all the preposterous salary from Lloyds that
Bill gets*, bracketed [[[preposterous salary] from Lloyds] that Bill gets]:
the adjective sits innermost. The restriction is an Oracle-register
representation decision of the owner. Record it as such, and do not cite CGEL
for it.

### Goal

`PostmodifiedNominal`, `ObjectRelativeNominal`, `SubjectRelativeNominal` and
`ParticipialPostmodifiedNominal` reject a head that is a
`PremodifiedNominal`. The premodifier then attaches only above the postmodified
Nominal.

### Analysis

Use declared features and tables only. Add no new Construction, and no guard
naming a lexeme, Construction or card.

- `PremodifiedNominal` is at `declarations.rs:2632`. The postmodifier
  Constructions are `PostmodifiedNominal` `:2891`,
  `ParticipialPostmodifiedNominal` `:3681`, `SubjectRelativeNominal` `:3907`
  and `ObjectRelativeNominal` `:3958`.
- Add a Nominal feature recording an outermost Adjective-phrase premodifier,
  for example `{Premodified, Bare}`. `PremodifiedNominal` exports
  `Premodified`. Every other Nominal Construction exports `Bare`, and the
  four postmodifier Constructions require `Bare` of their head.
- Decide how the feature passes through Nominal coordination, for example
  [[first instant or sorcery] spell], from the probes. The coordinated
  witnesses below must keep at least one Reading per coordination analysis
  they have today.
- Name the feature from a glossary term. Add it next to **Premodifier**
  (`docs/contexts/oracle-english/CONTEXT.md:1062`).

Only Adjective-phrase premodifiers (`PremodifiedNominal`, including
`OrdinalPremodifier` AdjectivePhrases) are restricted. Noun premodifiers
(`NounPremodifiedNominal`), cardinal premodifiers
(`CardinalPremodifiedNominal`) and slash modifiers were not measured and are
out of scope.

### Witnesses (b)

Before is a Sentence-category probe at minting, using the provisioned
`cargo-xtask` with trunk `zunvpkot`'s lexicon. After is the expected count
from a tree classifier that drops every Reading in which one of the four
postmodifier Constructions has a `PremodifiedNominal` head. It is an
estimate, not a grammar run. Re-probe the before counts if either sibling
attachment ticket has landed.

Counts must FALL:

1. Hiveheart Shaman, "Whenever this creature attacks, you may search your
   library for a basic land card that doesn't share a land type with a land
   you control, put that card onto the battlefield, then shuffle.":
   2,036 → 1,193.
2. Shadow Puppeteers, the sentence above: 1,660 → 980.
3. Eluge, the Shoreless Sea, "The first instant or sorcery spell you cast each
   turn costs {U} less to cast for each land you control with a flood counter
   on it.": 1,720 → 1,376.
4. Halsin, Emerald Archdruid, the full sentence above: 260 → 158.
5. Suffocation, "Suffocation deals 4 damage to the controller of the last red
   instant or sorcery spell that dealt damage to you this turn.": 182 → 132.

Also expected:

- Felisa, Fang of Silverquill, "Whenever a nontoken creature you control
  dies, if it had counters on it, create X tapped 2/1 white and black Inkling
  creature tokens with flying, where X is the number of counters it had on
  it.": 280 → 224.
- Angel's Tomb, the sentence above: 40 → 35.

Controls that must NOT change:

- Goliath Daydreamer, "Whenever this creature attacks, you may cast a spell
  from among cards you own in exile with dream counters on them without paying
  its mana cost.": 1,194.
- Nissa of Shadowed Boughs, "You may put a creature card with mana value less
  than or equal to the number of lands you control onto the battlefield from
  your hand or graveyard with two +1/+1 counters on it.": 1,260.
- Riddleform, the sentence above: 320. *Sphinx creature* is a noun
  premodifier.
- Wyluli Wolf's sentence: 2.

## Out of scope

- **The lexical *first* split.** *first* reads both as
  `vocab:AttributiveAdjective/First` and as an Ordinal numeral
  (`OrdinalPremodifier`). That accounts for Eluge's `h_lex` factor ×4 and
  Saint Traft's. It is a lexical-ownership question, not an attachment, and
  this ticket must not retire either analysis.
- Saint Traft and Rem Karolus, Belladonna Took and Teething Wurmlet show
  `g_premod` drops (84 → 28, 84 → 28, 24 → 8). Their *first time this ability
  has resolved* is a wrong analysis that `english-v3-perfect-relative-adverbial-gap`
  replaces, so they are not witnesses here. Report their counts after both
  tickets.
- Adjunct-height and *on it* attachments (the sibling tickets).

## Method

Standard constraints apply. Deltas:

1. **Probe first.** Probe every witness and control, write them as tests, and
   run the tests once at the end.
2. **Bounded census.** The selector is the union of two sets:
   - every face whose base per-line report lists `PremodifiedNominal`
     together with one of the four postmodifier Constructions in
     `constructions`;
   - every face matching `\buntil\b`, for the (a) verification.

   Pass it as an `--identity-manifest`, union it with 300 random covered faces
   from `diff --manifest … --random 300 --seed <S>`, and record the seed.
   Run the claim parent and the feature on the same manifest. If the selector
   exceeds half of the supported faces, run `--all` instead and say so.
3. **Compare.** Use `cargo xtask english-v3 diff --lines`.
   - Zero lost faces. Any loss is an attested premodifier-inside reading:
     STOP and name the face.
   - Name every decrease with the Reading shape it lost
     ([[Adj N] Postmodifier], or a nominal temporal PP). Give counts by class
     in the record and names for the top 20.
4. **Measure.** Run under `/usr/bin/time -v` with `--workers 6`, and record
   wall time and peak RSS.

## Landing record

The six items in `docs/tickets/README.md` ("Landing record"), plus the per-line Readings total before and
after on the manifest. Item 4 states that (b) follows the owner's convention
against CGEL's sample bracketing.
