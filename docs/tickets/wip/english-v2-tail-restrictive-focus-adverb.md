---
needs: []
---
**Restrictive `only` is a focus adverb, not three form literals inside a
bespoke predicate.** `Cast this spell only if you control a Goblin.` selects
while `Activate only if you control a Goblin.` fails, and `Cast this spell if
you control a Goblin.` already selects *without* the adverb — the whole
`CastingRestriction` family contributes nothing but the word `only`, and
contributes it in only three of the positions the word occupies.

Sizing from the 2026-09-05 failure census (measured on change `ykrsttluzkxm`,
18,917 / 32,641 covered, 13,724 parse failures, first-failure byte attribution;
re-measure at claim): **729 units** fail at the byte where `only` begins — the
largest family in the corpus that no ticket owns. By the constituent the adverb
focuses: 312 an `as` phrase, 166 an `if` clause, 101 a frequency adverbial, 86 a
`during` phrase, 57 an object Noun Phrase, 7 a Bare Predicate or a bare
temporal. By host, 651 follow an Object-less `Activate`.

Defect sentences (census, first-failure offset in the whole-face unit):

- Adaptive Gemguard — `Activate only as a sorcery.` — failed at `only`;
  expected a Noun Phrase (`+`, `-`, `a`, `no`, `non`, `the`, `up`, and 53 more).
- Aclazotz, Deepest Betrayal (Temple of the Dead) — `Activate only if a player
  has one or fewer cards in hand and only as a sorcery.`
- Alaborn Veteran — `Activate only during your turn, before attackers are
  declared.`
- Angus Mackenzie — `Activate only before the combat damage step.`
- Aggressive Mining — `Activate only once each turn.`
- Air Bladder — `Enchanted creature can block only creatures with flying.` —
  failed at `only`; expected a coordinator or the end of the Object.
- Agrus Kos, Eternal Soldier — `Whenever Agrus Kos becomes the target of an
  ability that targets only it, you may pay {1}{R/W}.`
- Errantry — `Enchanted creature gets +3/+0 and can only attack alone.` —
  failed at `only`; expected a copula, a declaration verb, or a Lexical Verb
  Phrase.
- Baron Strucker, HYDRA Overlord — `Do this only once each turn.`

Root cause, three symptoms of one shape.

1. `only` is a **form literal**, written three times
   (`crates/deckmaste_english_v2/src/constructions.rs`, `only_if_restriction`,
   `only_during_restriction`, `only_temporal_clause_restriction`), once per
   attested focus. It is the whole content of `abstract sum CastingRestriction`
   and of the wrapper category `RestrictionTurn`.
2. The restriction is a **required complement of a bespoke predicate**:
   `action_restriction_predicate` is `verb(_head) object restrictions` with
   `require len(restrictions) >= 1`, so the restriction cannot appear as an
   ordinary Adjunct and cannot appear at all without an overt Object. That is
   why every Object-less `Activate only …` fails: the parser has matched
   *activate* as a verb and is waiting for its Object.
3. The focus is enumerated by surface, so the same adverb before an Object
   (`block only creatures with flying`) or before a Bare Predicate (`can only
   attack alone`) has no derivation anywhere.

Pinned shape.

- **`only` becomes a declared lexeme.** Add the restrictive focus adverb to a
  `vocab FocusAdverb` beside the existing `vocab FrequencyAdverb`
  (`constructions.rs:117`) — the same closed-class-vocabulary idiom, with the
  same `lex(...)` spelling. After the landing `grep -c '"only"'
  crates/deckmaste_english_v2/src/constructions.rs` is 0.
- **One Focus construction per category the adverb can attach in, never one per
  attested focus surface.** The focus is the general category that follows the
  adverb: a `PredicateAdjunct` (which already carries the `Prepositional`,
  `Purpose`, `Duration`, `Frequency` and `Manner` arms — `constructions.rs:1094`),
  an `Object`, and a `BarePredicate`. Each Focus construction is
  `lex(adverb) <focus>` and is a member of the same sum its focus is a member
  of, so the focused phrase attaches exactly where the unfocused phrase already
  attaches — through `predicate_adjunct_predicate` /
  `prepositional_predicate_adjunct_predicate` for the adjunct, in the Object
  slot for the object, in the Bare Predicate slot for the predicate. This is the
  `english-v2-tail-keyword-ability-grant` idiom: the general host, never a
  per-family host.
- **Delete the bespoke family.** `only_if_restriction`,
  `only_during_restriction`, `only_temporal_clause_restriction`,
  `restriction_turn` and `action_restriction_predicate` go, together with
  `abstract sum CastingRestriction`, `RestrictionTurn`,
  `ActionRestrictionPredicate` and the `ActionRestriction:` arm in each of the
  five predicate sums (`constructions.rs:1165, 1181, 1199, 1220`) and in
  `PrepositionalPredicateAdjunctHost` (`:1104`). Coordinated restrictions
  (`only during combat and only if …`) come from the general predicate/adjunct
  coordination, never from `seq CastingRestriction separated by " and "`.
- **The Object-less host is declared data, not a construction.** *Activate*'s
  `frame_set` in `plugins/builtin_v2/macros/stubs/keyword_actions/Activate.ron`
  becomes `Custom(frames: [[], [ObjectNounPhrase]])` — the shape two other
  keyword-action stubs already carry — so `Activate` derives through the
  existing `intransitive_predicate` (`IntransitiveLexicalVerbPhrase`) and takes
  the adjunct through the general host. No new construction, no new sum member,
  and nothing in the grammar names the verb. Because this touches declaration
  data, the gate is `cargo test --workspace` (CLAUDE.md, "Gate scope for
  compiler changes").

Attestation is provenance. The census sizes the family; it does not bound what
the construction admits. The Focus construction admits every focus its
categories support and every `FocusAdverb` member later declared (*also*,
*even*, *just*), whether or not the corpus prints them today.

Two blocks inside the 729 stay failures after this landing and are **not** in
scope; both derive with no further grammar work once their own ticket lands, and
neither may be bought with a special arm here:

- 312 `only as a sorcery` — `as` is not yet a `Preposition`. Owned by
  `english-v2-remaining-prepositions` (R8).
- 101 `only once each turn` — `once each turn` is not yet a
  `FrequencyPredicateAdjunct`; `Draw a card once each turn.` fails without the
  adverb too, so this is a distinct family and stays on the fog register.

Ruled against.

- Any construction named for a focus surface (`only_as_a_sorcery`,
  `only_once_each_turn`) or any fourth `CastingRestriction` arm — the whole sum
  is deleted, not extended.
- Keeping `action_restriction_predicate` beside the general route "for the
  cases that already parse". The adjunct-class ruling is that per-X hosts are
  *replaced*, not inherited.
- A `restriction: opt CastingRestriction` slot on any verb frame.
- Making the adverb a Determinative, a quantifier, or a member of
  `FrequencyAdverb`.
- Giving *activate* an Object-less reading with a construction, an object gap,
  or a null-complement rule instead of a declared frame.

STOP-and-report.

- Any `require` or `checked by` naming the adverb (`require adverb is Only`), a
  verb identity (`head != …Activate`), a keyword action, or a card.
- A closed list of restriction-taking verbs, or of the adjunct classes `only`
  may focus.
- Any dominance edge or exception entry added to break the rivalry between the
  object-focus reading (`targets only it`) and the adjunct-focus reading, or
  between a focused adjunct and an unfocused one.
- A narrowed existing form or a deleted covered analysis used to keep a number.

Acceptance. The standard landing record (PROVE / DISCLOSE / REPORT per CLAUDE.md
and the rewrite ADR's 2026-09-04 amendment), plus these witnesses:

- `Activate only if you control a Goblin.` and `Activate only during your turn.`
  select — they are parse failures today.
- `Activate this ability only if you control a Goblin.` and `Cast this spell
  only during combat.` still select, now through the general adjunct host.
- `Cast this spell only during combat and only if you control a Goblin.` still
  selects, now through general coordination.
- `This creature can block only creatures with flying.` and `Enchanted creature
  gets +3/+0 and can only attack alone.` select.
- `Activate this ability.` still selects (the Object-ful frame survives).
- Construction count shows the five constructions and three categories **deleted**
  rather than kept beside the Focus construction, and the `"only"` form-literal
  count is 0.
- Both byte-exact laws green with total ownership; zero unresolved ties.

Baseline: measured on change `ykrsttluzkxm`, 18,917 covered of 32,641,
13,724 parse failures, 0 unresolved ties, 0 internal failures — re-run
unchanged after the `english-v2-scope-device-mobility-declarations` integrate
rebased that change. Re-measure at claim.

Glossary. `docs/contexts/oracle-english/CONTEXT.md` defines Adjunct, Modifier,
Complement and Coordination but has **no** entry for **Adverb**, **Adverb
Phrase**, or **Focus** / **Focus Adverb**. The landing amends that glossary
through the `domain-modeling` skill as part of the work; the disclosed glossary
gap is not optional here, it is the category this ticket introduces.

Overlap to reconcile at claim: `english-v2-cost-family-lowering` lists
`CastingRestriction` and `RestrictionTurn` among the game-semantic categories it
will lower. This ticket deletes both. Strike them from that ticket's inventory
when this lands (noted there 2026-09-05).

Tier: **terra** — the shape is fully pinned (one vocabulary, one Focus
construction per attachment category, one declaration-data edit, a list of
deletions); no compiler seam and no open dimension.

Standard constraints apply.

## Landing record

Work stopped on 2026-09-05 00:55:31 -07:00 before implementation or corpus
gates. The claimed tree has no production, test, glossary, or coverage-lock
changes.

### STOP: pinned-shape contradiction

The pinned shape permits exactly three Focus constructions, each spelling
`lex(adverb) <focus>` where `<focus>` is `PredicateAdjunct`, `Object`, or
`BarePredicate`. Its acceptance witnesses nevertheless require `Activate only
if you control a Goblin.` and the coordinated `only if` tail to select.

Current `PredicateAdjunct` has exactly the Prepositional, Purpose, Duration,
Frequency, and Manner members; no member derives an `if` clause. The existing
conditional tail is instead `postposed_if_predicate: ClauseAttachment`, with
form `body "if" condition`. It is not one of the three permitted focus
categories. `Object` and `BarePredicate` likewise cannot begin with `if`.

Consequently, satisfying the witnesses requires either a fourth Focus route
over the conditional attachment or a change to the existing conditional-tail
construction. Either conflicts with the ticket's explicit three-construction
pin and its requirement that every Focus construction be `lex(adverb) <focus>`
over those attachment categories. This is a ticket-internal contradiction, so
the STOP protocol prohibits selecting either interpretation.

Evidence: `constructions.rs` lines 1094--1100 define the complete
`PredicateAdjunct` sum; lines 1489--1492 define the sole postposed conditional
predicate route; lines 1964--1988 show the two general adjunct hosts; and
lines 67--82 and 136--141 above provide the conflicting pin and witnesses.

### DISCLOSE / REPORT

- Assurance: restored 0; re-spelled 0; ignored 0; added 0; removed 0.
- Deviations and additions: none.
- Glossary gap: none landed; the required terminology amendment is deferred
  with the unresolved ticket decision.
- Coverage, construction, selection, and performance figures: not measured,
  because no implementation is valid to gate while the STOP remains open.
- Decision wanted: reconcile the permitted focus category for a conditional
  tail with the three-construction pin, then re-issue the ticket with one
  authoritative shape.
