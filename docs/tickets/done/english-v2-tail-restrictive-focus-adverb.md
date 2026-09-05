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
  attested focus surface.** 2026-09-05 coordinator ruling: the following
  three-item enumeration **was** incomplete census-derived prose, not a fence:
  `PredicateAdjunct`, `Object`, and `BarePredicate`. The governing principle is
  the complete grammar-derived set: every category that restrictive *only* can
  precede in Oracle English, including `ClauseAttachment` for conditional and
  temporal tails, plus every further corpus-attested category (and every
  grammatical zero-witness category). List each category and witness in the
  landing record; attestation remains provenance, never admissibility. Each
  Focus construction is
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

### STOP: original pinned-shape contradiction (superseded)

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

### STOP: coordinator-ruling category-shape contradiction (superseded 2026-09-05)

The coordinator correctly removed the census-derived three-category fence, but
its named `ClauseAttachment` route cannot satisfy the governing shape either.
`ClauseAttachment` is not a tail constituent in the current grammar: its
postposed conditional construction has the form `body "if" condition`, and its
other postposed temporal forms likewise contain their host before their tail.
Consequently `lex(adverb) ClauseAttachment` can only spell `only <body> if
<condition>`; it cannot spell the required `<body> only if <condition>`.

Making that acceptance witness select requires a declared tail category (or a
refactor that separates a ClauseAttachment host from its tail) before a Focus
construction can be `lex(adverb) <focus>`. Adding a construction that names
the conditional surface, or giving a Focus construction a host-plus-tail
reordered form, violates the ruling's stated generic shape. The same issue
applies to `only as long as …`. This is a ticket-versus-ruling/code-shape
contradiction; work remains stopped until the coordinator names the permitted
tail seam or changes the required construction law.

Evidence: `constructions.rs` lines 1485--1492 define the only postposed `if`
routes; the Focus law above requires `lex(adverb) <focus>`; and the acceptance
witnesses require the reverse host/tail order. No production or test change was
made after the no-op refresh.

The coordinator resolved this STOP later on 2026-09-05 by authorizing the
minimal general `ClauseTail` seam: factor every existing subordinate or adjunct
tail from its attachment host, make preposed and postposed attachment operate on
that constituent, and focus the tail itself. The ruling also required this
landing to preserve every currently covered selection except the intended focus
gains and to stop on any negative oracle newly covered.

### STOP: unrestricted Predicate Adjunct focus covers a negative oracle (2026-09-05)

The authorized `ClauseTail` seam was implemented in a scratch working-copy
revision together with `FocusAdverb`, the Predicate Adjunct, Object, Bare
Predicate, and position-preserving Clause Tail focus constructions, the
Object-less *Activate* frame, and deletion of the five bespoke restriction
constructions. The first general tail draft exposed two analyses for an
ordinary postposed Predicate Adjunct: the existing
`predicate_adjunct_predicate` route and the new postposed Clause Tail route.
For example, the existing modal assurance
`Choose three. You may choose the same mode more than once.` materialized two
semantic candidates instead of one.

Representing the grammar's existing preposed/postposed tail inventories as
structural sums removed that rivalry without a dominance edge, exception, or
word-naming guard. A general mixed Clause Tail coordination also re-spelled
`Cast this spell only during your turn and only if you control a snow land.`,
and a nonrecursive Clause Tail body preserved the existing attachment
reciprocals. At that point the targeted Predicate grammar suite was 105 passed
and four failed; two failures were assertions whose former negative surfaces
are intended Object-focus gains, and one was a construction-path assertion to
re-spell. The remaining failure is a STOP:

- `Cast this spell only your turn.` is an existing negative oracle in
  `cost_frame_reciprocals_reject_crossed_boundaries`, but it newly selects.
  Its selected analysis is the ordinary Cast predicate followed by
  `FocusedPredicateAdjunct(Only, DurationPredicateAdjunct(FixedDurationPhrase(
  your turn)))` through the general predicate-adjunct host. The unfocused
  duration constituent is already admitted by the grammar; the new general
  Focus construction therefore admits it exactly as pinned.

That reading is not grammatical Oracle English: the temporal adjunct requires
the preposition in *during your turn* (or the demonstrative in *this turn*).
Preventing the new selection requires narrowing the Predicate Adjunct focus
domain, excluding the Duration member, or adding a check that recognizes this
surface. The ticket expressly rules out a closed list of adjunct classes, a
narrowed form, and any word- or construction-naming `require` / `checked by`.
Reclassifying the asserted negative as positive would contradict the ruling's
"wrong or negative newly covered analysis is a STOP" condition. There is no
authorized implementation that both keeps the category-general Focus law and
keeps this negative oracle negative.

The scratch implementation was restored in full. No production, declaration
data, test, glossary, subordinate-clause ticket, or coverage-lock change is
retained in this STOP commit.

### DISCLOSE / REPORT after the Clause Tail ruling

- Assurance: restored 0; re-spelled 0; ignored 0; added 0; removed 0.
- Deviations and additions: none retained.
- Glossary gap: `ClauseTail` remains unlanded; the authorized project term and
  Adverb / Adverb Phrase / Focus entries remain deferred with the STOP.
- Coverage, construction count, corpus selection census, ambiguity diff,
  roundtrip, and performance: not measured. The mandatory refreshed landing
  gates were not run because a targeted assurance produced a wrong newly
  covered negative before refresh.
- Targeted evidence: the position-preserving draft made all 49
  `ability_logic` tests green before the nonrecursive condition refinement; the
  refined Predicate grammar run reached 105 passed / 4 failed and identified
  the negative selection above. These scratch results are diagnostic evidence,
  not landing-gate artifacts.
- Decision wanted: authorize a grammar distinction between temporal adjuncts
  that may and may not be focused without a lexical/construction guard, or
  explicitly reclassify `Cast this spell only your turn.` and amend the
  negative-oracle STOP rule. The current letter authorizes neither choice.

The coordinator resolved that STOP on 2026-09-05 as a diagnosis: every host
classification read must see through the Focus wrapper. The implementation
made the two actual Predicate Adjunct classifiers feature-transparent:
prepositional attachment class and duration class recurse through a focused
adjunct. The asserted premise that the unfocused sentence was rejected was
then tested with the public `english_v2 probe` command on `default@`. The
premise was false: `Cast this spell your turn.` already selects uniquely on
the coordinator line as `Transitive Cast(Object(this spell))` followed by
`PredicateAdjunct(Duration(FixedDurationPhrase(your turn)))`. The existing
reciprocal test did not contain that unfocused sentence. The focused sentence
selects through the identical host and duration analysis with one transparent
Focus wrapper, so the ruling's fallback applies. The pre-existing bare-duration
defect is routed to a follow-up of `english-v2-fixed-duration-endpoint`, whose
declared temporal noun licence currently admits possessive *your turn* as a
marker-less duration.

### STOP: category recursion covers doubled focus negatives (2026-09-05)

After feature transparency, the next two existing negative oracles newly
select:

- `Cast this spell only only during your turn.` selects uniquely through
  `PrepositionalPredicateAdjunctPredicate`, with
  `FocusedPredicateAdjunct(Only, FocusedPredicateAdjunct(Only,
  PrepositionalPredicateAdjunct(during your turn)))`.
- `Cast this spell only only if you control a snow land.` selects uniquely
  through `PostposedPredicateClauseTail`, with two nested
  `FocusedPostposedClauseTail` values around the existing `IfClauseTail`.

Both reject on `default@`. They are a consequence of the pinned recursive
shape itself: the Focus construction is a member of the same sum as its focus,
so its result is immediately a focus operand again. Feature transparency does
not and should not change that category membership.

The two general repairs available under the current construction language are
both outside the authorization. A nonrecursive Focus-operand sum would exclude
the Focus member and therefore narrow the focus domain; a predicate that
rejects an already-focused operand would be construction-sensitive. The
2026-09-05 diagnosis expressly forbids both narrowing and a
construction-sensitive check, while the ticket's negative-oracle rule forbids
accepting either new analysis. No word-sensitive guard, dominance edge,
exception, or per-surface construction was added.

### DISCLOSE / REPORT after the feature-transparency diagnosis

- Assurance: restored 0; re-spelled 0; ignored 0; added 0; removed 0 in the
  safe STOP commit. The scratch Predicate grammar run reached 105 passed / 4
  failed; three failures were already-classified assertion re-spellings and
  the fourth exposed the doubled-focus negative above.
- Deviations and additions: none retained.
- Coverage, construction count, corpus selection census, ambiguity diff,
  roundtrip, and performance: not measured. The mandatory refreshed landing
  gates were not run because targeted negative assurance stopped first.
- STOP: unresolved. Decision wanted: authorize a general nonrecursive
  Focus-operand category that contains every unfocused member of each focus
  category, or authorize another general way to prevent immediately nested
  Focus without a word- or construction-sensitive guard. Reclassifying or
  deleting the two negative witnesses is not requested.

The coordinator resolved this fourth STOP on 2026-09-05 by ruling that focus
non-iteration is a declared-feature fact. The construction compiler now has
one exhaustive `Focus` feature with `Unfocused` and `Focused` values. Every
unfocused member of Predicate Adjunct, Object, Bare Predicate, and Clause Tail
derives `Unfocused`; each Focus construction requires its operand's feature to
be `Unfocused` and derives `Focused`. This is one category-general rule and
names no word, construction, verb, mechanic, or card. The two doubled-focus
sentences reject, while the third-STOP transparency fix remains in place for
all other host classifications. The pre-existing marker-less possessive
duration defect is routed to
`english-v2-bare-duration-adjunct-licence` rather than changed here.

### PROVE

Work ran from 2026-09-05 00:55:31 -07:00 through 2026-09-05 03:19:22
-07:00, including four coordinator-ruling pauses and the single post-refresh
gate pass. Corpus source fingerprint
`e85359d7b8c578df13dff2fdf7c743a520a5b367d5ed25ab0a5f03cb8b3637dd`.

**Stamp (review, after the second `kata refresh` onto B7a).** Every number
below the `### Review corrections` heading was measured on change `tlywlpov`
(`review: …`) over `mtqnuxkx`, with coverage lock `covered` **19,198** against
a refreshed parent whose lock `covered` is **19,002**. The implementer's own
figures were measured on the pre-B7a parent `ykrsttlu` (18,917 → 19,113) and
are retained above as the record of that tree; the landing delta is unchanged
at **+196 / −0** on both bases.

- **Declared restrictive focus.** `FocusAdverb::Only` is the sole declared
  vocabulary spelling; there is no `only` form literal. The five focus
  constructions are the complete grammar-derived positional set:
  `PredicateAdjunct` (`Cast this spell only during combat.`), `Object` (`This
  creature can block only creatures with flying.`), `BarePredicate`
  (`Enchanted creature gets +3/+0 and can only attack alone.`),
  `PreposedClauseTail` (`Only if you control a Goblin, activate this
  ability.`), and `PostposedClauseTail` (`Activate only if you control a
  Goblin.`). The preposed category has zero corpus gains but remains declared;
  attestation was not used as a filter.
- **General Clause Tail seam.** The extracted tail inventory preserves every
  former attachment shape: preposed `if`, `as`, `as long as`, `while`,
  `until`, and general Predicate Adjunct (including the `during` temporal
  tail); postposed `if`, `unless`, `as long as`, `for as long as`, and
  `while`; plus postposed tail coordination. Clause and Predicate hosts each
  consume the applicable positional tail with one host construction per
  attachment position, never one per tail spelling.
- **Same-site attachment and non-iteration.** Focused Object and Bare Predicate
  values forward every feature their host reads. Focused Predicate Adjunct
  classification recurses through the wrapper for preposition attachment,
  complement kind, locative-temporal licence, and duration kind. Every
  focus-bearing sum carries the compiler's exhaustive `Focus::{Unfocused,
  Focused}` feature; unfocused members derive `Unfocused`, and every focus
  construction requires that value and derives `Focused`. Thus the wrapper is
  transparent to all other admissibility facts while `only only ...` cannot
  recurse.
- **Bespoke restriction family removed.** `only_if_restriction`,
  `only_during_restriction`, `only_temporal_clause_restriction`,
  `restriction_turn`, and `action_restriction_predicate` are deleted, as are
  `CastingRestriction`, `RestrictionTurn`, `ActionRestrictionPredicate`, and
  their five sum arms. Their positive witnesses remain and now select through
  the general Focus and attachment machinery.
- **Declared Object-less host.** `Activate.ron` declares frames `[]` and
  `[ObjectNounPhrase]`; no construction, guard, or exception names the verb.
  `Activate this ability.` retains the Object frame, and Object-less
  `Activate only ...` uses the existing intransitive predicate host.
- **Acceptance and negative witnesses.** The single-focus conditional,
  temporal, coordinated-tail, Object, and Bare Predicate witnesses select.
  `Cast this spell only only during your turn.` and `Cast this spell only only
  if you control a snow land.` reject. The feature-transparent pair `Cast
  this spell your turn.` / `Cast this spell only your turn.` both select
  through the same pre-existing marker-less Duration analysis; that separate
  defect is routed to `english-v2-bare-duration-adjunct-licence`.
- **No loss, wrong gain, or tie.** The report-mode check and frozen-parent
  ambiguity diff show 196 gains and zero losses. All 196 selected analyses are
  pasted below and were read. 173 traverse at least one Focus construction:
  35 Object, 51 Predicate Adjunct, and 94 Postposed Clause Tail identities,
  with seven identities containing two focus categories. The other 23 are
  correct general Clause Tail seam gains. No gain is a negative oracle or
  wrong analysis; no selection tie exists.
- **Structural laws.** Coverage reports 19,113 selected and covered, 0
  selected-uncovered, 0 unresolved ties, 0 internal failures, 0 exception
  resolutions or uses, 0 roundtrip mismatches, 0 ownership failures, 0
  construction- or leaf-traversal failures, 0 gap or overlap spans, 0
  synthetic claims, and 0 provenance-plan mismatches. Construction traversal
  is 838,897/838,897 and leaf traversal is 291,804/291,804. The independent
  roundtrip gate reports 19,113 clean of 19,113 parse-accepted.
- **No word-naming.** Coverage reports 20 permitted licensing checkers and 0
  forbidden. The new `require` reads only the declared `focus` feature. No
  `checked by`, dominance edge, exception, form narrowing, or guard added by
  this landing names a word, lexeme, construction, verb, noun, preposition,
  mechanic, or card.

Positive gates, all foreground on the refreshed tree:

- `cargo fmt --all` exited 0 (only the repository's stable-rustfmt warnings
  for nightly-only options).
- `CARGO_BUILD_JOBS=8 cargo clippy -p deckmaste_construction_core -p
  deckmaste_english_v2 -p xtask --all-targets --all-features -- -D warnings`
  exited 0: `Finished dev profile`.
- `CARGO_BUILD_JOBS=8 cargo test --workspace` exited 0. Positive artifacts
  include `test result: ok. 49 passed; 0 failed; 0 ignored`, `test result: ok.
  109 passed; 0 failed; 0 ignored`, `test result: ok. 76 passed; 0 failed; 0
  ignored`, and `test result: ok. 459 passed; 0 failed; 1 ignored`.
- `DECKMASTE_COVERAGE_LOCK=report CARGO_BUILD_JOBS=8 cargo xtask english_v2
  coverage --check --workers 8` exited 0 with the +196/-0 delta below; the
  matching report-mode `--bless` exited 0 and wrote exactly 19,113 identities.
- `CARGO_BUILD_JOBS=8 cargo xtask english_v2 ambiguity --require-resolved
  --workers 8 --json` exited 0 with 0 unresolved ties and 0 internal failures.
- `CARGO_BUILD_JOBS=8 cargo xtask english_v2 roundtrip --require-clean
  --workers 8` exited 0: `parse accepted 19113`, `clean 19113`, `mismatched 0`.
- No rules citation changed, so cite gates were not required.

### DISCLOSE

Selection census, frozen refreshed parent against the feature tree:

| census | parent `ykrsttlu` | feature `mtqnuxkx` | delta |
|---|---:|---:|---:|
| selected / covered | 18,917 | 19,113 | +196 |
| parse failures | 13,724 | 13,528 | -196 |
| unique selections | 14,870 | 14,983 | +113 |
| specificity-resolved | 4,047 | 4,130 | +83 |
| exception-resolved | 0 | 0 | 0 |
| unresolved ties | 0 | 0 | 0 |

The per-unit comparison reports 196 parse-failure-to-selected gains, 0 losses,
and 2,441 changed construction paths among the 18,917 identities already
selected by the parent. Of those existing selections, 2,422 retain the same
resolution class and are structural re-spellings from the former
tail-containing attachment products to the shared Clause Tail plus positional
host. The remaining 19 selected outcomes were read individually: Force of
Nature, Blood Clock, Fade Away, Possessed Portal, Furnace Punisher, Killing
Wave, Power Taint, Minion of Tevesh Szat, Torment of Scarabs, Unnatural
Hunger, Soul Tithe, Curse Artifact, Slow Motion, Scourge of Numai, Vampire
Lacerator, Tithe Taker, Bellowing Mauler, and Enchanter's Bane change
`unique -> specificity` because the factored tail exposes an additional
lower-ranked attachment bracketing; the selected analysis and rendered text
remain the parent's reading. Wake the Dead changes `specificity -> unique`
because deletion of the bespoke restriction route removes its losing duplicate.

Exactly 23 already-selected identities change candidate count. The 19 above
account for 19; Custody Battle and Mindwrack Demon remain specificity-resolved
while their general `unless` tail bracketings change `2 -> 4`, and Reset and
Savage Beating remain specificity-resolved while their former bespoke/general
restriction duplicates collapse `4 -> 2`. Each keeps the same selected
analysis. No existing identity changes selected semantic outcome.

The 196 gains comprise 130 unique and 66 specificity-resolved selections.
The 23 correct non-Focus seam gains are Aladdin, Arsenal Thresher, Corrupted
Shapeshifter, both Dragonlord Silumgar identities, Gift of Doom, Lim-Dûl's Hex,
Master Thief, Mind Flayer, Nivix, Possession Engine, Primal Plasma, Rescuer
Sphinx, Rootwater Matriarch, Sirocco, Stench of Evil, Thrull Champion,
Ulamog's Despoiler, War Cadence, War Tax, Willbreaker, Yidris, and Yuffie.

Excluded census blocks remain excluded:

- 312 `only as a sorcery` units contribute zero gains and remain routed to
  `english-v2-remaining-prepositions` (R8).
- 101 `only once each turn` units contribute zero gains and are routed to
  `english-v2-frequency-adverbial-family`, minted at review (2026-09-05); the
  record named that ticket before it existed.
  Battlefield Scrounger, Chronatog Totem, and Groundling Pouncer were
  temporarily exposed by an over-broad new mixed-tail coordination member;
  restricting that newly introduced structural member to the focused Adjunct
  shape removed all three wrong analyses without a word-sensitive guard.

Coverage delta pasted from report-mode `coverage --check`:

```text
newly covered 196 corpus identities
newly covered	02b00fe85c207e1de5dbc08b275a78c6cf8adb297445f1be22095ccee3367676	card "Companion of the Trials"	selected_analysis "Flying\n{1}{W}: Untap target creature. Activate only if you control a Gideon planeswalker."
newly covered	03c00ff4d9617085ecfb88e760e51b6c5d8919f5666850e7467b9d7c45e6d374	card "Cloud Spirit"	selected_analysis "Flying\nThis creature can block only creatures with flying."
newly covered	05cde50e13728032c7958e5b40723920492889975429ea8bcebcd63228754747	card "Dreamcaller Siren"	selected_analysis "Flash\nFlying\nThis creature can block only creatures with flying.\nWhen this creature enters, if you control another Pirate, tap up to two target nonland permanents."
newly covered	0611a578c2d429d83215add43a14cccab806b128db3987b7f92f32bef2581359	card "Hallowed Healer"	selected_analysis "{T}: Prevent the next 2 damage that would be dealt to any target this turn.\nThreshold — {T}: Prevent the next 4 damage that would be dealt to any target this turn. Activate only if there are seven or more cards in your graveyard."
newly covered	0676d1da6677363e08274c448fd102deabf871dcb25e546ae61fe4ed04486bb7	card "Shadows of the Past"	selected_analysis "Whenever a creature dies, scry 1.\n{4}{B}: Each opponent loses 2 life and you gain 2 life. Activate only if there are four or more creature cards in your graveyard."
newly covered	086872ac1e052f1ed1b70c5f6e4b0c58e2acb42181a1667741a54cf91a824f08	card "Sapseep Forest"	selected_analysis "This land enters tapped.\n{G}, {T}: You gain 1 life. Activate only if you control two or more green permanents."
newly covered	0962c8a858be1d888e3753d2e41055aa98adebf9f46ec5571be9c9d75eab0de5	card "Krovikan Plague"	selected_analysis "Enchant non-Wall creature you control\nWhen this Aura enters, draw a card at the beginning of the next turn's upkeep.\nTap enchanted creature: This Aura deals 1 damage to any target. Put a -0/-1 counter on enchanted creature. Activate only if enchanted creature is untapped."
newly covered	09f678e2fc6979e5f190f99de1389bf41edb30bd2beb4151d22b503af486478d	card "Necrosavant"	selected_analysis "{3}{B}{B}, Sacrifice a creature: Return this card from your graveyard to the battlefield. Activate only during your upkeep."
newly covered	0c0af4c2f6aa4339618f98df6db0df10af091f52d0fa0447f2f4b2085bb0eccc	card "Lim-Dûl's Hex"	selected_analysis "At the beginning of your upkeep, for each player, this enchantment deals 1 damage to that player unless they pay {B} or {3}."
newly covered	0ced87ba2adaf7282596ec2023c2ab3a24e23114c8bc442f8fd68ba7ad52c042	card "Soulcipher Board // Cipherbound Spirit (Cipherbound Spirit)"	selected_analysis "Flying\nThis creature can block only creatures with flying.\n{3}{U}: Draw two cards, then discard a card."
newly covered	10c9b03f93f7af5080943187d3f8f34ff0a6af14c5c85152a7c6ee73e58c4b06	card "Corrupted Shapeshifter"	selected_analysis "Devoid\nAs this creature enters, it becomes your choice of a 3/3 creature with flying, a 2/5 creature with vigilance, or a 0/12 creature with defender."
newly covered	11a4e9098e8978a76d8f9ef268ab546f886974066c92bb21fa74801fd0ba6a49	card "Scrapskin Drake"	selected_analysis "Flying\nThis creature can block only creatures with flying."
newly covered	151b3e7263444c55668da93254f45b49ddef436b7ef89304d1c35bc298de8374	card "Sirocco"	selected_analysis "Target player reveals their hand. For each blue instant card revealed this way, that player discards that card unless they pay 4 life."
newly covered	1645d639aeae85eb9d971498e845ebaf569c0b696f636799c5e20f03569abb29	card "Etherium Pteramander"	selected_analysis "Flying\nThis creature can block only creatures with flying.\n{6}{B}: Adapt 4. This ability costs {1} less to activate for each other artifact you control."
newly covered	17a20c94fc6f855dd926688f832c5d164a8486d8b0ad2248815b4c852543cc9d	card "Wanderlight Spirit"	selected_analysis "Flying\nThis creature can block only creatures with flying."
newly covered	18074883a6609ff93e1654459a93d442af7b13c584027b6f5eeb12e71480d404	card "Defenders of Humanity"	selected_analysis "When this enchantment enters, create X 2/2 white Astartes Warrior creature tokens with vigilance.\n{X}{2}{W}, Exile this enchantment: Create X 2/2 white Astartes Warrior creature tokens with vigilance. Activate only if you control no creatures and only during your turn."
newly covered	183b8a455b3a2b4048dc50f996eb10677527634f5d8ca6ddef2dffe578210a8f	card "Walker of Secret Ways"	selected_analysis "Ninjutsu {1}{U}\nWhenever this creature deals combat damage to a player, look at that player's hand.\n{1}{U}: Return target Ninja you control to its owner's hand. Activate only during your turn."
newly covered	185a064037559abdde6bc62b86380838a1d94aa32c4de9c59b7fce212a0bb1f6	card "Stratus Walk"	selected_analysis "Enchant creature\nWhen this Aura enters, draw a card.\nEnchanted creature has flying.\nEnchanted creature can block only creatures with flying."
newly covered	1b332eb95e2c9f005e9daa0d3f470baa083cdee2a88cfc1fb27a4fcd60a4817b	card "Rimewind Cryomancer"	selected_analysis "{1}, {T}: Counter target activated ability. Activate only if you control four or more snow permanents."
newly covered	1c402c202658963563c4b42639fa26f4367f468a02be52b455824299e6870487	card "Arsenal Thresher"	selected_analysis "As this creature enters, you may reveal any number of other artifact cards from your hand. This creature enters with a +1/+1 counter on it for each card revealed this way."
newly covered	1e5be7ed0a4c9d7d7f97f48392fa5a9ed45490277a63f8817b44d85b28b4cd97	card "Rootwater Matriarch"	selected_analysis "{T}: Gain control of target creature for as long as that creature is enchanted."
newly covered	1e6c3447604f409daad6548a76f38b8fbe05d5a350b26b1339f3e3c86260f879	card "Stallion of Ashmouth"	selected_analysis "Delirium — {1}{B}: This creature gets +1/+1 until end of turn. Activate only if there are four or more card types among cards in your graveyard."
newly covered	20eb2b5ebe9f2472be1c9db021aecad1ff0b137bfc00397167215e6d9a419ff4	card "Disrupting Scepter"	selected_analysis "{3}, {T}: Target player discards a card. Activate only during your turn."
newly covered	218cf38e96e7b94cf9dafe199bd75d80e16ba37e13ca7cf88130b9fec3649fcc	card "Tainted Isle"	selected_analysis "{T}: Add {C}.\n{T}: Add {U} or {B}. Activate only if you control a Swamp."
newly covered	227f7b087df85d2d34d71fe27e5fe5883d63f681b71d7b98e7a67f54ab1a4445	card "Ascending Aven"	selected_analysis "Flying\nThis creature can block only creatures with flying.\nMorph {2}{U}"
newly covered	23cb250bd6e12649038b5b206fcf8789a8c3c42ff45699e067e1df0611420daf	card "Temple of the False God"	selected_analysis "{T}: Add {C}{C}. Activate only if you control five or more lands."
newly covered	24b5ad17f44685e18708445aad0678d8a311e0eb0761851d91a3570478afcb50	card "Yuffie, Materia Hunter"	selected_analysis "Ninjutsu {1}{R}\nWhen Yuffie enters, gain control of target noncreature artifact for as long as you control Yuffie. Then you may attach an Equipment you control to Yuffie."
newly covered	27932f488ccbebc261071d049dd1403da330435c123966e670ac90f8f308eda2	card "Colossus of Sardia"	selected_analysis "Trample\nThis creature doesn't untap during your untap step.\n{9}: Untap this creature. Activate only during your upkeep."
newly covered	27ae68737c0ba7807274d9df29988d5c13218f16eef795db5bec3cd5b1f31ecd	card "Thought Shucker"	selected_analysis "Threshold — {1}{U}: Put a +1/+1 counter on this creature and draw a card. Activate only if there are seven or more cards in your graveyard and only once."
newly covered	27e03deef003eda3823f74d5f21fae69424a402d1125302ffa34ac22fbb76836	card "Kuldotha Phoenix"	selected_analysis "Flying, haste\nMetalcraft — {4}: Return this card from your graveyard to the battlefield. Activate only during your upkeep and only if you control three or more artifacts."
newly covered	28beec8892a504fe713481369fcc2e188e5cdcf5dfc8f65357ff1f46d4f9c869	card "Cloud Pirates"	selected_analysis "Flying\nThis creature can block only creatures with flying."
newly covered	28dba8fb4f815dfb4780312e5c90062f069398d5d85f6e8cde7b30be6cab2d8a	card "Llanowar Augur"	selected_analysis "Sacrifice this creature: Target creature gets +3/+3 and gains trample until end of turn. Activate only during your upkeep."
newly covered	29965a387e300449faebdd10dc9584daff9bf6aab5862148e2ac6c95697b9028	card "Willbreaker"	selected_analysis "Whenever a creature an opponent controls becomes the target of a spell or ability you control, gain control of that creature for as long as you control this creature."
newly covered	2ace77fe66ae797a44138751d6c467238b9e37f888aae2324b807ffb6ef09cf2	card "Nomad Decoy"	selected_analysis "{W}, {T}: Tap target creature.\nThreshold — {W}{W}, {T}: Tap two target creatures. Activate only if there are seven or more cards in your graveyard."
newly covered	2c30b92e1b69c26341f9c7a5d06ecc17c9c345ec1b82533a2e244c40c451e644	card "Krosan Avenger"	selected_analysis "Trample\nThreshold — {1}{G}: Regenerate this creature. Activate only if there are seven or more cards in your graveyard."
newly covered	2ca14b2d72c17a0dfe07bb6cb337d665fa711f34b0301c245602134d70ec185a	card "Inventors' Fair"	selected_analysis "At the beginning of your upkeep, if you control three or more artifacts, you gain 1 life.\n{T}: Add {C}.\n{4}, {T}, Sacrifice Inventors' Fair: Search your library for an artifact card, reveal it, put it into your hand, then shuffle. Activate only if you control three or more artifacts."
newly covered	2e9a65d1711e86bd03c0a11bda00013d5976eac82e78e86057050c3d720397e2	card "Infected Vermin"	selected_analysis "{2}{B}: This creature deals 1 damage to each creature and each player.\nThreshold — {3}{B}: This creature deals 3 damage to each creature and each player. Activate only if there are seven or more cards in your graveyard."
newly covered	2fa28cd2c05daa039b3ff24d2d8293c2fca1f97641b950fedabdf5bbff528f59	card "Cloud Sprite"	selected_analysis "Flying\nThis creature can block only creatures with flying."
newly covered	2fd0602ccbf647b3585a16e1d3419d533028ad6c69c69e7a2892b02059b3a07c	card "Earthlore"	selected_analysis "Enchant land you control\nTap enchanted land: Target blocking creature gets +1/+2 until end of turn. Activate only if enchanted land is untapped."
newly covered	30074e9a9ca6ed79bd7cb43463b2ee4654d12a957ac1e13d093f7fcd5987f0b7	card "Sceptre of Eternal Glory"	selected_analysis "{T}: Add one mana of any color.\n{T}: Add three mana of any one color. Activate only if you control three or more lands with the same name."
newly covered	30bf1ac9fa07a1a33f325295afcd2709d282ca276ce03412bc4fe259ce3a293e	card "Fleshformer"	selected_analysis "{W}{U}{B}{R}{G}: This creature gets +2/+2 and gains fear until end of turn. Target creature gets -2/-2 until end of turn. Activate only during your turn."
newly covered	30ecdbbd461253f3a79983fd24b04cbbb36588ba15b57c68b87219bff74ceda7	card "Black Carriage"	selected_analysis "Trample\nThis creature doesn't untap during your untap step.\nSacrifice a creature: Untap this creature. Activate only during your upkeep."
newly covered	313230f835e6b41a2082abe92c20c185968f09de720f8d35224185daec602282	card "Stratozeppelid"	selected_analysis "Flying\nThis creature can block only creatures with flying."
newly covered	33de7ab6d9390d1128de6a87fbf01ec7522ffb2ab5b5c76cec3ea72ce40adf86	card "Hoverguard Observer"	selected_analysis "Flying\nThis creature can block only creatures with flying."
newly covered	3466b622e4d9e12544382c26f0bc14586666cc23806c9cb7fd20e043319db993	card "Gerrard Capashen"	selected_analysis "At the beginning of your upkeep, you gain 1 life for each card in target opponent's hand.\n{3}{W}: Tap target creature. Activate only if Gerrard Capashen is attacking."
newly covered	3560dd7ad258972788b097fd2f340452e5ab8a5066aeccd323b29b82bc9d982f	card "Shi'ar Soldier"	selected_analysis "Flying\n{U}, {T}: Return another target permanent you control to its owner's hand. Activate only during your turn."
newly covered	397398457e4d71edfeb689c7299e38de46a9b0ada728f599f31ca9af8c3545d6	card "Master Thief"	selected_analysis "When this creature enters, gain control of target artifact for as long as you control this creature."
newly covered	3a36b2488a19613971bbd8a0c4f623780b900d77e79092540894ae7216e86e94	card "Bonders' Enclave"	selected_analysis "{T}: Add {C}.\n{3}, {T}: Draw a card. Activate only if you control a creature with power 4 or greater."
newly covered	3a50635fef57e29d3e17dc70ee443467e41a7e3f6c266a2dff185aba594ace42	card "Merfolk Windrobber"	selected_analysis "Flying\nWhenever this creature deals combat damage to a player, that player mills a card.\nSacrifice this creature: Draw a card. Activate only if an opponent has eight or more cards in their graveyard."
newly covered	3b8a52cfc6c3d2d64fb3e474f80d2eeb90c7f247b658c6db38313503c43701fc	card "Hushwood Verge"	selected_analysis "{T}: Add {G}.\n{T}: Add {W}. Activate only if you control a Forest or a Plains."
newly covered	3c7327b5eb8461f03a8c07bada1960ea1af22086d61ebfd1e901a5ed9ac2705d	card "Nivix, Aerie of the Firemind"	selected_analysis "{T}: Add {C}.\n{2}{U}{R}, {T}: Exile the top card of your library. Until your next turn, you may cast it if it's an instant or sorcery spell."
newly covered	3cd3964668b63620f4650f718fcf0cb1ff5b578b5d5b9e886ef00e6c75fa04f5	card "Angus Mackenzie"	selected_analysis "{G}{W}{U}, {T}: Prevent all combat damage that would be dealt this turn. Activate only before the combat damage step."
newly covered	3d28894789a3e09ee1ec88523f921057ff4a68214730dcb9332cc4e1d859bb23	card "War Tax"	selected_analysis "{X}{U}: This turn, creatures can't attack unless their controller pays {X} for each attacking creature they control."
newly covered	3e55d666747a3cd7a0bc0d3f25eb9008ab2678c35f8ff2ef3c1e26fec916efb8	card "Cinder Crawler"	selected_analysis "{R}: This creature gets +1/+0 until end of turn. Activate only if this creature is blocked."
newly covered	3fe05ab5c6198a2462893b7aa1f3df9e9b46d705351b5e6c20c6237dcc54585f	card "Nihilistic Glee"	selected_analysis "{2}{B}, Discard a card: Target opponent loses 1 life and you gain 1 life.\nHellbent — {1}, Pay 2 life: Draw a card. Activate only if you have no cards in hand."
newly covered	40168e04e299f0a8cb7c41e7ecfb21d88ecb6e98f8753c0a66863efd57d9a987	card "Raving Visionary"	selected_analysis "{U}, {T}: Draw a card, then discard a card.\nDelirium — {2}{U}, {T}: Draw a card. Activate only if there are four or more card types among cards in your graveyard."
newly covered	428a5c84d524a8e6cf51be087e52929a9438bc8bb387ed86b1c31ae28715de85	card "Vaporkin"	selected_analysis "Flying\nThis creature can block only creatures with flying."
newly covered	42b3faf60c0e756b2403df984e2a83f03da0abf753c6b50690603189a1f160ad	card "Bleachbone Verge"	selected_analysis "{T}: Add {B}.\n{T}: Add {W}. Activate only if you control a Plains or a Swamp."
newly covered	42e3bebcd58ab25d7adb06531a09f9f40fb84fbdaf1124a9d529e0e3e57e6aa3	card "Dense Canopy"	selected_analysis "Creatures with flying can block only creatures with flying."
newly covered	433a87fc8b37e24f6e25d51b24564a1e6e08d3ebb78fb30b8a2978c6ed66a9b5	card "Vona, Butcher of Magan"	selected_analysis "Vigilance, lifelink\n{T}, Pay 7 life: Destroy target nonland permanent. Activate only during your turn."
newly covered	43f14e90b11f20a925d3d3f6cf43167d284bbb0d58e9c58abebe239c6a785680	card "Kongming's Contraptions"	selected_analysis "{T}: This creature deals 2 damage to target attacking creature. Activate only during the declare attackers step and only if you've been attacked this step."
newly covered	48347a4195ce50f3ee2e61e85d5269f7700f7ec1d3494ea7cb7bb4fac651a3b1	card "Tattered Haunter"	selected_analysis "Flying\nThis creature can block only creatures with flying."
newly covered	4a8c2d76b25d1f6a0c77fbc1fc57ffe4ff0c67ff3893948ccd8bfb282736e881	card "Whisperer of the Wilds"	selected_analysis "{T}: Add {G}.\nFerocious — {T}: Add {G}{G}. Activate only if you control a creature with power 4 or greater."
newly covered	4c1b4ec40606f78226d18a1a3fecb9d75f729b6bac6fa8d9b11615039f5742eb	card "Belbe's Percher"	selected_analysis "Flying\nThis creature can block only creatures with flying."
newly covered	4c6575ef232f47590a91ce0f30f20d6beda33d0fe6d98f2a543c0b3cb3beebc5	card "Tainted Field"	selected_analysis "{T}: Add {C}.\n{T}: Add {W} or {B}. Activate only if you control a Swamp."
newly covered	4c90aabb66e727e48d53e3ae4ff8716d6e6b96d2859140dbdfaa169a7c4da478	card "Tainted Wood"	selected_analysis "{T}: Add {C}.\n{T}: Add {B} or {G}. Activate only if you control a Swamp."
newly covered	4cbe06677f6806b1837a87c50e6e16c83ad5270cc0c81a54e7f6fdef22fba178	card "Keldon Megaliths"	selected_analysis "This land enters tapped.\n{T}: Add {R}.\nHellbent — {1}{R}, {T}: This land deals 1 damage to any target. Activate only if you have no cards in hand."
newly covered	4cf2099184455c45059a6c5515e6ea3ffa820305e7305a7c772082a56015ec5a	card "Flywheel Racer"	selected_analysis "Vigilance\n{T}: Add one mana of any color. Activate only if this permanent is a creature.\nCrew 1"
newly covered	4ea70f4c38c08c3e6722e5a3f0c2ecfe8b1e6a54620aaddc39040789257ce923	card "Jade Statue"	selected_analysis "{2}: This artifact becomes a 3/6 Golem artifact creature until end of combat. Activate only during combat."
newly covered	502c660a9ec64fcfb63cd441754aef0d0587efde3acac495cb9b33ce864f0215	card "Stronghold Zeppelin"	selected_analysis "Flying\nThis creature can block only creatures with flying."
newly covered	52bfb8c04a32004117c8df671d137db13093cb2c7b17fc9e630446aceacf6f05	card "Stormcloud Djinn"	selected_analysis "Flying\nThis creature can block only creatures with flying.\n{R}{R}: This creature gets +2/+0 until end of turn and deals 1 damage to you."
newly covered	55ac0337fcf401fa450a3742f4d4baf43bfc2efd4c0624e9d6e429708310045c	card "Shacklegeist"	selected_analysis "Flying\nThis creature can block only creatures with flying.\nTap two untapped Spirits you control: Tap target creature you don't control."
newly covered	57bf5776a09571cde6504f968576e763416a44cde8ae7db1afd293af4b687b7a	card "Brazen Borrower // Petty Theft (Brazen Borrower)"	selected_analysis "Flash\nFlying\nThis creature can block only creatures with flying."
newly covered	58d86a6d57291208de752ab99957336d48121883d47033086390ca4d823500ec	card "Puca's Eye"	selected_analysis "When this artifact enters, draw a card, then choose a color. This artifact becomes the chosen color.\n{3}, {T}: Draw a card. Activate only if there are five colors among permanents you control."
newly covered	5a2850f632f184f91ae5c33181aebaef9477c6b575f7c92740bad7100e46610b	card "Welkin Tern"	selected_analysis "Flying\nThis creature can block only creatures with flying."
newly covered	5a53ee362e36ee7dc3975ae9fd89658728afad321a214702d50c7cad2703d0c6	card "Zuran Enchanter"	selected_analysis "{2}{B}, {T}: Target player discards a card. Activate only during your turn."
newly covered	5b38cc40416bc3efb77024022a7e9d6fd7b9cb86407a8d1942bc37fe4c6efddb	card "Fool's Tome"	selected_analysis "{2}, {T}: Draw a card. Activate only if you have no cards in hand."
newly covered	5b809dba06c377ac1e9083c0882c5a1b1e99d81bc7606c93eb877554ad6921a1	card "Tourach's Gate"	selected_analysis "Enchant land you control\nSacrifice a Thrull: Put three time counters on this Aura.\nAt the beginning of your upkeep, remove a time counter from this Aura. If there are no time counters on this Aura, sacrifice it.\nTap enchanted land: Attacking creatures you control get +2/-1 until end of turn. Activate only if enchanted land is untapped."
newly covered	5d2e567b9c567147dd4695d67f5456dd5618b36dc4895b04bfaa50b42b91e083	card "Thrull Champion"	selected_analysis "Thrull creatures get +1/+1.\n{T}: Gain control of target Thrull for as long as you control this creature."
newly covered	5d86b649da64a9772a54c5be4a98ff609d23838b1d5bb0e1c42896a77bf574c1	card "Kindly Stranger // Demon-Possessed Witch (Kindly Stranger)"	selected_analysis "Delirium — {2}{B}: Transform this creature. Activate only if there are four or more card types among cards in your graveyard."
newly covered	5e5c778d7c11512c2285bea43890a5482bc9b01d811249b073386b5bb43684d8	card "Spire of Industry"	selected_analysis "{T}: Add {C}.\n{T}, Pay 1 life: Add one mana of any color. Activate only if you control an artifact."
newly covered	5f1d9bf3fbbfde51d8e468ae45d2da06aa326ef1cf2a155f092a91330c3b929d	card "Possession Engine"	selected_analysis "When this Vehicle enters, gain control of target creature an opponent controls for as long as you control this Vehicle. That creature can't attack or block for as long as you control this Vehicle.\nCrew 3"
newly covered	61250d03abd9134e0584cf1b05fb58112313118ba3808d9eb1b7a44c054cf946	card "Goblin Ski Patrol"	selected_analysis "{1}{R}: This creature gets +2/+0 and gains flying. Its controller sacrifices it at the beginning of the next end step. Activate only once and only if you control a snow Mountain."
newly covered	63b8f2ee6ffe87941910c252fe350d649aae94867bf9a99f884df74b21082635	card "Cabal Torturer"	selected_analysis "{B}, {T}: Target creature gets -1/-1 until end of turn.\nThreshold — {3}{B}{B}, {T}: Target creature gets -2/-2 until end of turn. Activate only if there are seven or more cards in your graveyard."
newly covered	647f4419721c7c8ed18b5abc0a7b0918e521f69b7aa956180341ae8c4b326b53	card "Gwendlyn Di Corci"	selected_analysis "{T}: Target player discards a card at random. Activate only during your turn."
newly covered	66f35f877fda49aa03050f74a425b7a5e448019e85b2efca435bcd2824627ed3	card "Nim Devourer"	selected_analysis "This creature gets +1/+0 for each artifact you control.\n{B}{B}: Return this card from your graveyard to the battlefield, then sacrifice a creature. Activate only during your upkeep."
newly covered	676b9c81891a22d2d1c5901a19461ab4f69a5413b1848ce0d784556e5080bc31	card "Chainflinger"	selected_analysis "{1}{R}, {T}: This creature deals 1 damage to any target.\nThreshold — {2}{R}, {T}: This creature deals 2 damage to any target. Activate only if there are seven or more cards in your graveyard."
newly covered	69dd85e1b8116d8ebcc4e19c5ab8d0ff216fb8ac8f09bdc2e39782a9c8ddd736	card "Gloomwidow"	selected_analysis "Reach\nThis creature can block only creatures with flying."
newly covered	6b42d55ee7caed881eaa55a72293b92aa91f94cc20b5a03d59d457233ed34f57	card "Wastewood Verge"	selected_analysis "{T}: Add {G}.\n{T}: Add {B}. Activate only if you control a Swamp or a Forest."
newly covered	6e27128f767cbe025d89d647ee989f8a2bbdb70d17afbc01b66072eb5b103df9	card "Mox Opal"	selected_analysis "Metalcraft — {T}: Add one mana of any color. Activate only if you control three or more artifacts."
newly covered	6e5da8055cdea695921e02f17364b3b3f903f931a91ff531eca3ae1bd83265c9	card "Gloomlake Verge"	selected_analysis "{T}: Add {U}.\n{T}: Add {B}. Activate only if you control an Island or a Swamp."
newly covered	6eea97508ba80ed4ed167a8cbaae76a1d5f4bff60ef0cea5937df530c32ca494	card "Hammer of Bogardan"	selected_analysis "Hammer of Bogardan deals 3 damage to any target.\n{2}{R}{R}{R}: Return this card from your graveyard to your hand. Activate only during your upkeep."
newly covered	6f75378f5d867a2efa4d11c0f1454dbb605cd0af3f82978dc0db44d3edab2278	card "Kelpie Guide"	selected_analysis "{T}: Untap another target permanent you control.\n{T}: Tap target permanent. Activate only if you control eight or more lands."
newly covered	6f847d804ba23dd5fa9290d5007b38ab0158ba1594edd2bbcbc9b0e3a4bf4ff7	card "Argent Sphinx"	selected_analysis "Flying\nMetalcraft — {U}: Exile this creature. Return it to the battlefield under your control at the beginning of the next end step. Activate only if you control three or more artifacts."
newly covered	7205e912214ce5fd436bf8ea2863b9cbf96d728b3ffbbd9f2e3d69b5723653a2	card "Rishadan Brigand"	selected_analysis "Flying\nWhen this creature enters, each opponent sacrifices a permanent of their choice unless they pay {3}.\nThis creature can block only creatures with flying."
newly covered	73e67e86881196b0b7da0a67a8eeecdceb6a90c5c52cfe879ead1f672cd9a03c	card "Centaur Garden"	selected_analysis "{T}: Add {G}. This land deals 1 damage to you.\nThreshold — {G}, {T}, Sacrifice this land: Target creature gets +3/+3 until end of turn. Activate only if there are seven or more cards in your graveyard."
newly covered	7843d496827940baa4046a5d7106c531c0502789759013bc3c300eb727ad04d3	card "Cryptic Caves"	selected_analysis "{T}: Add {C}.\n{1}, {T}, Sacrifice this land: Draw a card. Activate only if you control five or more lands."
newly covered	7971da69411b4b6060af060e00909e4bec7aad93e4e2aa343bf8b199a319ed0f	card "Cinderhaze Wretch"	selected_analysis "{T}: Target player discards a card. Activate only during your turn.\nPut a -1/-1 counter on this creature: Untap this creature."
newly covered	7b15df99b45e3a106fe33d08084c74f4c354f08da31e014a77d8dde0f65630b2	card "Underhanded Designs"	selected_analysis "Whenever an artifact you control enters, you may pay {1}. If you do, each opponent loses 1 life and you gain 1 life.\n{1}{B}, Sacrifice this enchantment: Destroy target creature. Activate only if you control two or more artifacts."
newly covered	7b1776e7ee1af1fa3a2cce8f232cf5352553113e0b19c566f547bbc8265a5f24	card "Tablet of Compleation"	selected_analysis "{T}: Put an oil counter on this artifact.\n{T}: Add {C}. Activate only if this artifact has two or more oil counters on it.\n{1}, {T}: Draw a card. Activate only if this artifact has five or more oil counters on it."
newly covered	7bdf17eed5a2c8b6f94544c04175dcd4fb83d1c6be54c931fe8b862e894c5d5d	card "Desert"	selected_analysis "{T}: Add {C}.\n{T}: This land deals 1 damage to target attacking creature. Activate only during the end of combat step."
newly covered	7e54065c3192660712e0c8cc639cc494b0b7019d95016e8ac395afd94c1792d0	card "Augur il-Vec"	selected_analysis "Shadow\nSacrifice this creature: You gain 4 life. Activate only during your upkeep."
newly covered	7ed6864d22c87e1783b1e180bd9d41501ed71c87bd70b2c893f738915eaaf0be	card "Svyelunite Priest"	selected_analysis "{U}{U}, {T}: Target creature gains shroud until end of turn. Activate only during your upkeep."
newly covered	803ae74a6154fa596ddc284301082da3546f2a5993175099899d3e8d3d64658d	card "Tainted Peak"	selected_analysis "{T}: Add {C}.\n{T}: Add {B} or {R}. Activate only if you control a Swamp."
newly covered	814b4a0799686a8a81437c68f4d6097391eab024e9e59d4fccd291d87b489fb0	card "Mox Jasper"	selected_analysis "{T}: Add one mana of any color. Activate only if you control a Dragon."
newly covered	84f0b011f9fec75f6716b3807c874b8255fd343946c367df33d35e8152150c00	card "Ghost Town"	selected_analysis "{T}: Add {C}.\n{0}: Return this land to its owner's hand. Activate only if it's not your turn."
newly covered	862a3b6a2e5c3ff893c8ff3d81cdf501dd43479f582d2be407dda03a43412fc8	card "Steadfast Unicorn"	selected_analysis "{3}{W}: Creatures you control get +1/+1 and gain vigilance until end of turn. Activate only during your turn."
newly covered	8701b58a2e5600769129b262524be8cfa9b1ad1075ded1a3c7b0d91d633ae7cf	card "Madblind Mountain"	selected_analysis "This land enters tapped.\n{R}, {T}: Shuffle your library. Activate only if you control two or more red permanents."
newly covered	89e598684c32ecec3f88c984b94a23cc3eca23116c46a2aa224d494277038d6a	card "Urza's Workshop"	selected_analysis "{T}: Add {C}.\nMetalcraft — {T}: Add {C} for each Urza's land you control. Activate only if you control three or more artifacts."
newly covered	8cd0fdea72ef651d3dd48aa7536eb846b11e1a4167f0a06dc6bd1e1ef08e3c8f	card "Sunbillow Verge"	selected_analysis "{T}: Add {W}.\n{T}: Add {R}. Activate only if you control a Mountain or a Plains."
newly covered	8d26c214ef5a8031351a0d573da4d65e04920336e85c86f048d64fee954df242	card "Haunt of the Dead Marshes"	selected_analysis "When this creature enters, scry 1.\n{2}{B}: Return this card from your graveyard to the battlefield tapped. Activate only if you control a legendary creature."
newly covered	8d2b167a5947b457f0f47031fe0744d5e084e7e4519dbbf852e079460c477e37	card "Mtenda Griffin"	selected_analysis "Flying\n{W}, {T}: Return this creature to its owner's hand and return target Griffin card from your graveyard to your hand. Activate only during your upkeep."
newly covered	8e73aa473f0ba6f35d6b9f5b3975b220b44ecc32b37c32d4880118841fc7ac20	card "Moonring Island"	selected_analysis "This land enters tapped.\n{U}, {T}: Look at the top card of target player's library. Activate only if you control two or more blue permanents."
newly covered	8eb03e539a23d661527266e88210946a844bfb7b2cd1e6100d155442683073a2	card "Sea Gate Wreckage"	selected_analysis "{T}: Add {C}.\n{2}{C}, {T}: Draw a card. Activate only if you have no cards in hand."
newly covered	8f0bd33d0c66f34b1fd70e4c63ceab92ee285c1d600066ad29e5c58a4aa69b7d	card "Krosan Restorer"	selected_analysis "{T}: Untap target land.\nThreshold — {T}: Untap up to three target lands. Activate only if there are seven or more cards in your graveyard."
newly covered	8fc6396df1e70efdbe1c66bc8bcd0f51e16b4b3f75e0e965a9559d427a6bb07a	card "Crop Sigil"	selected_analysis "At the beginning of your upkeep, you may mill a card.\nDelirium — {2}{G}, Sacrifice this enchantment: Return up to one target creature card and up to one target land card from your graveyard to your hand. Activate only if there are four or more card types among cards in your graveyard."
newly covered	90e0c6e7e226e9a5f747f416417d4ad4752b34a3eb02de2c5b2b35d3ba44604c	card "Hakim, Loreweaver"	selected_analysis "Flying\n{U}{U}: Return target Aura card from your graveyard to the battlefield attached to Hakim. Activate only during your upkeep and only if Hakim isn't enchanted.\n{U}{U}, {T}: Destroy all Auras attached to Hakim."
newly covered	92cd7d6e34d9bdbd31d5442bcfed597b775ade6341c73a53e60d0d9bc8c40b69	card "Primal Plasma"	selected_analysis "As this creature enters, it becomes your choice of a 3/3 creature, a 2/2 creature with flying, or a 1/6 creature with defender."
newly covered	96c86dd83b0f225bfd722fff96a4dc1323968e58630802228e51941ca5638d20	card "Skywinder Drake"	selected_analysis "Flying\nThis creature can block only creatures with flying."
newly covered	977139a14d96cf24c60bf83ee3a6e8db8ca8293dcf598a135685bc6231f56ddb	card "Vivien's Jaguar"	selected_analysis "Reach\n{2}{G}: Return this card from your graveyard to your hand. Activate only if you control a Vivien planeswalker."
newly covered	9900b7999b4403397f049cac9e586d56eafb8edc0ceeb956b1b98240ec92fcc6	card "Undead Gladiator"	selected_analysis "{1}{B}, Discard a card: Return this card from your graveyard to your hand. Activate only during your upkeep.\nCycling {1}{B}"
newly covered	990970e1e1297d1d4a2d89d429f3e871cae9e16b05ca6de51e388080918bea6e	card "Floodfarm Verge"	selected_analysis "{T}: Add {W}.\n{T}: Add {U}. Activate only if you control a Plains or an Island."
newly covered	9ad236bc90576d2a6473b819631755da0eeeca3542cf5c3cbaaf200b2be66d26	card "Vedalken Certarch"	selected_analysis "Metalcraft — {T}: Tap target artifact, creature, or land. Activate only if you control three or more artifacts."
newly covered	9bc5e0a26704c0e89de88acaff10fd6e3a37bd2fcb54b8f62400d311b1bad08b	card "Aven Augur"	selected_analysis "Flying\nSacrifice this creature: Return up to two target creatures to their owners' hands. Activate only during your upkeep."
newly covered	9c499843fcefa09450c7f1014e7dd8e0f28932a95661fa291a200229021b6815	card "Bloodline Keeper // Lord of Lineage (Bloodline Keeper)"	selected_analysis "Flying\n{T}: Create a 2/2 black Vampire creature token with flying.\n{B}: Transform this creature. Activate only if you control five or more Vampires."
newly covered	9ec6fdf7509e68ab842466d8d7ea33f28aef4d5389b48099c6fb38944552e923	card "Humble Defector"	selected_analysis "{T}: Draw two cards. Target opponent gains control of this creature. Activate only during your turn."
newly covered	9fc1d84cd78516221d2661030ad0a027dd10e36e2ec85521f70456b85e024a21	card "Balduvian Hydra"	selected_analysis "This creature enters with X +1/+0 counters on it.\nRemove a +1/+0 counter from this creature: Prevent the next 1 damage that would be dealt to it this turn.\n{R}{R}{R}: Put a +1/+0 counter on this creature. Activate only during your upkeep."
newly covered	a10b76fba66b521b1e4e738b224d7bb1523591d3978ee825d0e84378d0ead31a	card "Path to Redemption"	selected_analysis "Enchant creature\nEnchanted creature can't attack or block.\n{5}, Sacrifice this Aura: Exile enchanted creature. Create a 1/1 white Ally creature token. Activate only during your turn."
newly covered	a13517851a7d0f4228f1c651539806c78c26652f3c76dcec76f791ccf9c91027	card "Hell's Caretaker"	selected_analysis "{T}, Sacrifice a creature: Return target creature card from your graveyard to the battlefield. Activate only during your upkeep."
newly covered	a56ae422da3c1e41d5b788b634a0fff2a89ae5f58177a67f6f08eb5e65212947	card "Aladdin"	selected_analysis "{1}{R}{R}, {T}: Gain control of target artifact for as long as you control this creature."
newly covered	a589dd8fa23fc417e59e1f96676a766df156eef3cbc8ca25d4c3611dd2d4be62	card "Augur of Skulls"	selected_analysis "{1}{B}: Regenerate this creature.\nSacrifice this creature: Target player discards two cards. Activate only during your upkeep."
newly covered	a6167afcf174f5124be78f432ee6b6409bbdc0d05a8dd1fe3a999a4a66df3279	card "Willowrush Verge"	selected_analysis "{T}: Add {U}.\n{T}: Add {G}. Activate only if you control a Forest or an Island."
newly covered	a9a15f67d93b687fcc4ba92bf2486ecef3eb14345cd52269c0b50854ebc1519b	card "Into the Fae Court"	selected_analysis "Draw three cards. Create a 1/1 blue Faerie creature token with flying and \"This token can block only creatures with flying.\""
newly covered	a9e9ef665fba9dba767c3b58b67995ee07f9e3c9d0b92b81802df474297c6688	card "War Cadence"	selected_analysis "{X}{R}: This turn, creatures can't block unless their controller pays {X} for each blocking creature they control."
newly covered	aabdce8d18da10817fbc9e0c31e6a4f5222ee0e940280480466e895873af23ec	card "Dwarven Armory"	selected_analysis "{2}, Sacrifice a land: Put a +2/+2 counter on target creature. Activate only during any upkeep step."
newly covered	ac9dd6ed86c1b98ed74e9dbc75210ad0a9e844d37e9819cc09f9548760ca3133	card "Stormbound Geist"	selected_analysis "Flying\nThis creature can block only creatures with flying.\nUndying"
newly covered	ad76afe077a2514e142e01217d608c6957aa6a10f8a095d8913b69266272ed57	card "Tectonic Edge"	selected_analysis "{T}: Add {C}.\n{1}, {T}, Sacrifice this land: Destroy target nonbasic land. Activate only if an opponent controls four or more lands."
newly covered	adcedee0278ee1520d0c890d00b43e25ee9c3dd262d7e6d81471bf77be28e752	card "Essence Reliquary"	selected_analysis "{T}: Return another target permanent you control and all Auras you control attached to it to their owner's hand. Activate only during your turn."
newly covered	b178359b0278b4c5f972b2d66c80bed9d01ec43d132788ac9a6e143ff2a99427	card "Sacred White Deer"	selected_analysis "{3}{G}, {T}: You gain 4 life. Activate only if you control a Yanggu planeswalker."
newly covered	b198a57734c8c8ef311d4d7a6c79e6ddfe258a28d6f4e764114ff9e30eb20d1d	card "Thornspire Verge"	selected_analysis "{T}: Add {R}.\n{T}: Add {G}. Activate only if you control a Mountain or a Forest."
newly covered	b2da08a1f7e8ee5aec9de10b14f35dfe9a8b991b155d11bb349bd0df7c5c7e6e	card "Gift of Doom"	selected_analysis "Enchant creature\nEnchanted creature has deathtouch and indestructible.\nMorph—Sacrifice another creature.\nAs this Aura is turned face up, you may attach it to a creature."
newly covered	b444f2802f9a1f6ac1c22852f4091b840cfaf735d52dbd5ab81c9ddf29fa932e	card "Blood Frenzy"	selected_analysis "Cast this spell only before the combat damage step.\nTarget attacking or blocking creature gets +4/+0 until end of turn. Destroy that creature at the beginning of the next end step."
newly covered	b57068737547286d95d828c12f6b102cd8aa954bc100c55611b338c53721c5d5	card "Cabal Pit"	selected_analysis "{T}: Add {B}. This land deals 1 damage to you.\nThreshold — {B}, {T}, Sacrifice this land: Target creature gets -2/-2 until end of turn. Activate only if there are seven or more cards in your graveyard."
newly covered	b5997896f5c75e9321468d6f571879a84289fc7df9cc41a1527de1abd9584053	card "Life Chisel"	selected_analysis "Sacrifice a creature: You gain life equal to the sacrificed creature's toughness. Activate only during your upkeep."
newly covered	b755114963c2a088a5967cb33c46a94a4aa81a851a52bb348d38378e416529c4	card "Mind Flayer"	selected_analysis "Dominate Monster — When this creature enters, gain control of target creature for as long as you control this creature."
newly covered	b81b53eaab5f14f30a186cc5bd83b7c79b8a9c43246104673698fc8f653e00c9	card "Shifting Woodland"	selected_analysis "This land enters tapped unless you control a Forest.\n{T}: Add {G}.\nDelirium — {2}{G}{G}: This land becomes a copy of target permanent card in your graveyard until end of turn. Activate only if there are four or more card types among cards in your graveyard."
newly covered	b90d29d844f552e5c44e8ff598b50d1b6a33cda55e5ca91107d2f6b7b38f677d	card "Forgestoker Dragon"	selected_analysis "Flying\n{1}{R}: This creature deals 1 damage to target creature. That creature can't block this combat. Activate only if this creature is attacking."
newly covered	b913f3bdfc4905d440b9cafc419268fcb7efddcb13fa0c405e02b64aa5937daa	card "Glint-Horn Buccaneer"	selected_analysis "Haste\nWhenever you discard a card, this creature deals 1 damage to each opponent.\n{1}{R}, Discard a card: Draw a card. Activate only if this creature is attacking."
newly covered	ba2a96ebef17cdf83dae745dd2eeb6757d235289f5db5e48c389fed083d3252c	card "Cloud Djinn"	selected_analysis "Flying\nThis creature can block only creatures with flying."
newly covered	bbeefdfe3fa0b4925c71a0ca7e4c0d05432ced23129c6eaecfedaa6effbb8fb7	card "Fanatic of Rhonas"	selected_analysis "{T}: Add {G}.\nFerocious — {T}: Add {G}{G}{G}{G}. Activate only if you control a creature with power 4 or greater.\nEternalize {2}{G}{G}"
newly covered	bc5e73ffef287cdbfcd746689cce8b97c93cbc23d9663833fc6b48bea30fb40d	card "Celestial Enforcer"	selected_analysis "{1}{W}, {T}: Tap target creature. Activate only if you control a creature with flying."
newly covered	bdf9952a840960214bddbc840c54a6fbd7e5e68e3e7cba2bea599b5777b20d74	card "Professor Zei, Anthropologist"	selected_analysis "{T}, Discard a card: Draw a card.\n{1}, {T}, Sacrifice Professor Zei: Return target instant or sorcery card from your graveyard to your hand. Activate only during your turn."
newly covered	bf0093f364a82f7237dfa7252d6ef1f89b6c4e85c263e362b3c052115fbb7350	card "Rishadan Airship"	selected_analysis "Flying\nThis creature can block only creatures with flying."
newly covered	c01efec3f5bd9e210f6a541587d275c23420e17aeaf842feebde0f33f7bc4d72	card "Chained Brute"	selected_analysis "This creature doesn't untap during your untap step.\n{1}, Sacrifice another creature: Untap this creature. Activate only during your turn."
newly covered	c026bf99d3e594c4d130007d83c4bc5eca90eb048b92f1b820c0bf2347010579	card "Barbarian Ring"	selected_analysis "{T}: Add {R}. This land deals 1 damage to you.\nThreshold — {R}, {T}, Sacrifice this land: It deals 2 damage to any target. Activate only if there are seven or more cards in your graveyard."
newly covered	c373e9807c3e04c9766591cdd8b8d46f73048be65970369482f7ab9cf258c7f1	card "Lavinia, Foil to Conspiracy"	selected_analysis "Vigilance\nWhenever you cast your second spell each turn, investigate.\n{T}: Add {C}{C}. Activate only during an opponent's turn."
newly covered	c594a444ddd6617d0cbf9e2fdcd7c683e5a018c95be35339ca63e80c118069bc	card "Dragonlord Silumgar // Dragonlord Silumgar (Dragonlord Silumgar)"	selected_analysis "Flying, deathtouch\nWhen Dragonlord Silumgar enters, gain control of target creature or planeswalker for as long as you control Dragonlord Silumgar."
newly covered	c8db9e1b3768af5636b4da0697cf84c91552bede922bcbeefd8029ad5281c8cd	card "Duggan, Private Detective"	selected_analysis "Duggan's power and toughness are each equal to the number of cards in your hand.\nWhenever Duggan enters or attacks, investigate.\nThe Most Important Punch in History — {1}{G}, {T}: Duggan deals damage equal to twice its power to another target creature. Activate only once."
newly covered	ca94f38c2ca21955a1c1e3296cab31f78758a139c81df020648d453cee132a02	card "Reaper of Flight Moonsilver"	selected_analysis "Flying\nDelirium — Sacrifice another creature: This creature gets +2/+1 until end of turn. Activate only if there are four or more card types among cards in your graveyard."
newly covered	cd158128052043cc3df27f769f7812d5699511246137974d0feb4744fe548a76	card "Portal of Sanctuary"	selected_analysis "{1}, {T}: Return target creature you control and each Aura attached to it to their owners' hands. Activate only during your turn."
newly covered	cf237961100789da568653dc348be858ccb2910034071088c44792f2c2905bd8	card "Blazemire Verge"	selected_analysis "{T}: Add {B}.\n{T}: Add {R}. Activate only if you control a Swamp or a Mountain."
newly covered	d1fdf8510f3a3e983bf71ed12b99bba8e557d8da62fbf18053875c2b2e1175f7	card "Prehistoric Pet"	selected_analysis "This creature can't be blocked by creatures with greater power.\n{1}{W}, {T}: Return another target creature you control to its owner's hand. Activate only during your turn."
newly covered	d26b55fe54fb25fa5374ebf0f2c42fd5e91eb62b78c2d4fbd4ed6716eeb5c9ad	card "Nimbus Maze"	selected_analysis "{T}: Add {C}.\n{T}: Add {W}. Activate only if you control an Island.\n{T}: Add {U}. Activate only if you control a Plains."
newly covered	d3a6d785c345066dd66445f4c83a0f9b0511632a6bf3eb3d2fbb4eb142679908	card "Dreadlight Monstrosity"	selected_analysis "Ward {2}\n{3}{U}{U}: This creature can't be blocked this turn. Activate only if you own a card in exile."
newly covered	d52b81845a1c08e027eb7275f5dbd58eabfb96dd21e6cffe7e551cc6578912a3	card "Yidris, Maelstrom Wielder"	selected_analysis "Trample\nWhenever Yidris deals combat damage to a player, as you cast spells from your hand this turn, they gain cascade."
newly covered	d603672c64c93fed670ea752a025867fdded4f284afd45aa509a9662dcf8bdce	card "Air Bladder"	selected_analysis "Enchant creature\nEnchanted creature has flying.\nEnchanted creature can block only creatures with flying."
newly covered	d74d8af3872754b884e68cc358205c034ca21c6678a4f4ad9f3045d8a1eddb78	card "Desiccated Naga"	selected_analysis "{3}{B}: Target opponent loses 2 life and you gain 2 life. Activate only if you control a Liliana planeswalker."
newly covered	d75080925cd414129929709a68f719b1aac8ecb99e86ecd8bdbf91351951daf5	card "Cloud Elemental"	selected_analysis "Flying\nThis creature can block only creatures with flying."
newly covered	db4bc99657aa416bf98c407e8e3c08cc55eb2bd000b4a793f9c209f1b5cec755	card "Dragonlord Silumgar"	selected_analysis "Flying, deathtouch\nWhen Dragonlord Silumgar enters, gain control of target creature or planeswalker for as long as you control Dragonlord Silumgar."
newly covered	dbb4d573554e1695deebd69ae5d2b37a4db962f189ce6a6dbffcd039603ba9e3	card "Leechridden Swamp"	selected_analysis "This land enters tapped.\n{B}, {T}: Each opponent loses 1 life. Activate only if you control two or more black permanents."
newly covered	debcb5fb94f9fab32ea82d98b51b5c28cdeaea1412de2e1ac8c22683fa7a287b	card "Touch of Vitae"	selected_analysis "Until end of turn, target creature gains haste and \"{0}: Untap this creature. Activate only once.\"\nDraw a card at the beginning of the next turn's upkeep."
newly covered	e23aaede9fa7bedba9d08b5bb1d3255f656e03a17ee94301286cecc731575f01	card "Matzalantli, the Great Door // The Core (Matzalantli, the Great Door)"	selected_analysis "{T}: Draw a card, then discard a card.\n{4}, {T}: Transform Matzalantli. Activate only if there are four or more permanent types among cards in your graveyard."
newly covered	e3beb7cb6a6bf6a97ad416bf7594eb09a408c600785db77dddd0e47f159b8579	card "Goblin Bird-Grabber"	selected_analysis "{R}: This creature gains flying until end of turn. Activate only if you control a creature with flying."
newly covered	e52e6d789e1c44e3de97234fc479c84b66f62991cdf4599b4689177f7d1a0470	card "Nomad Stadium"	selected_analysis "{T}: Add {W}. This land deals 1 damage to you.\nThreshold — {W}, {T}, Sacrifice this land: You gain 4 life. Activate only if there are seven or more cards in your graveyard."
newly covered	e5def5104dd26466f1641c64d0a45923206efbc96ac025308ac2c4a6b618975e	card "Weathered Wayfarer"	selected_analysis "{W}, {T}: Search your library for a land card, reveal it, put it into your hand, then shuffle. Activate only if an opponent controls more lands than you."
newly covered	e65e71b0f2c61c52636e7468af4314709131f09717dd64a0a62e57e66d1b3c50	card "Heidar, Rimewind Master"	selected_analysis "{2}, {T}: Return target permanent to its owner's hand. Activate only if you control four or more snow permanents."
newly covered	e7231f9854c247d65eb02945f71f248a0f7c30fd05bc391176c5c839223fdc43	card "Devoted Grafkeeper // Departed Soulkeeper (Departed Soulkeeper)"	selected_analysis "Flying\nThis creature can block only creatures with flying.\nIf Departed Soulkeeper would be put into a graveyard from anywhere, exile it instead."
newly covered	e7bd5491fd5333e7b5260d85cc60bf9666f0897326d7d5263a0311936b8aed1c	card "Battlefield Percher"	selected_analysis "Flying\nThis creature can block only creatures with flying.\n{1}{B}: This creature gets +1/+1 until end of turn."
newly covered	e9272b3c06d666e0703e2a70e051cc78729eec337b9420f94cd3301757309ec3	card "Riverpyre Verge"	selected_analysis "{T}: Add {R}.\n{T}: Add {U}. Activate only if you control an Island or a Mountain."
newly covered	e964934609966de6ebd595c6edb96c4637b3a78e882fc46dd746ff884199002f	card "Endless Atlas"	selected_analysis "{2}, {T}: Draw a card. Activate only if you control three or more lands with the same name."
newly covered	ec4d3a234731bc90b3b5c80d65df3a41575346eb9c28d748c5525f43d98aa274	card "Arnim Zola, Bio-Fanatic"	selected_analysis "{3}, {T}: Create a tapped 2/1 black Villain creature token with menace. Activate only if there are two or more creature cards in your graveyard."
newly covered	ecf5ef17da6fca7cd0f6c16425d6c9b72da81443bbec5b8a9e9b55c569a090eb	card "Cloud Dragon"	selected_analysis "Flying\nThis creature can block only creatures with flying."
newly covered	ee5e6799e4cd7907eca73c035fcff78a648b40227b0b7a1466b307671f865600	card "Ragamuffyn"	selected_analysis "Hellbent — {T}, Sacrifice a creature or land: Draw a card. Activate only if you have no cards in hand."
newly covered	ee5efcf84ef2408d570eaaeaa6d96b2503ca26019f104e3a2f74e64e7f245678	card "Rag Man"	selected_analysis "{B}{B}{B}, {T}: Target opponent reveals their hand and discards a creature card at random. Activate only during your turn."
newly covered	eef0f5b542ba3fc7678d562fbab0fa6872f342b40aca44b96a5881ec1684a761	card "Scepter of Fugue"	selected_analysis "{1}{B}, {T}: Target player discards a card. Activate only during your turn."
newly covered	f01b0a010312ba6a8448bfcdee3c73b9332f341b610d01a29c11a6a6dae42c82	card "Ulamog's Despoiler"	selected_analysis "As this creature enters, you may put two cards your opponents own from exile into their owners' graveyards. If you do, this creature enters with four +1/+1 counters on it."
newly covered	f01d1bc82e2bc41e95199f7acffe05e3efc466b992e647b019f29261b770872b	card "Stench of Evil"	selected_analysis "Destroy all Plains. For each land destroyed this way, Stench of Evil deals 1 damage to that land's controller unless they pay {2}."
newly covered	f32e85721bcb4e788982e399a866d4fa5fe918ba4720bce53aabe00d1630c3d7	card "Long-Finned Skywhale"	selected_analysis "Flying\nThis creature can block only creatures with flying."
newly covered	f4b3e894344f526612e1777fe3ebb32a7d6aa39e96cbcb824a7c140eb578263e	card "Rivendell"	selected_analysis "Rivendell enters tapped unless you control a legendary creature.\n{T}: Add {U}.\n{1}{U}, {T}: Scry 2. Activate only if you control a legendary creature."
newly covered	f5b32e7290939ef478465c73c23eaa4400beff204c2b0c2ddb8e89f9b04af70f	card "Emberwilde Augur"	selected_analysis "Sacrifice this creature: It deals 3 damage to target player or planeswalker. Activate only during your upkeep."
newly covered	f76476238e06d3bbb3b786d93742d2e141fa0ce9802fe9b041b7008e3bcb427e	card "Tideshaper Mystic"	selected_analysis "{T}: Target land becomes the basic land type of your choice until end of turn. Activate only during your turn."
newly covered	f8d1c7e47c79a0a3f1951b3327543600ac7e8b51de8b27620c0cd0f26acca3fc	card "Dwarven Weaponsmith"	selected_analysis "{T}, Sacrifice an artifact: Put a +1/+1 counter on target creature. Activate only during your upkeep."
newly covered	fa06cd9f34aec847c280ada7dffe8727a6ea7a78b866c44e4f93849362dc716d	card "Rescuer Sphinx"	selected_analysis "Flying\nAs this creature enters, you may return a nonland permanent you control to its owner's hand. If you do, this creature enters with a +1/+1 counter on it."
newly covered	fac284320fc7ae381e6070f5875842aa95aebd6fa29d2869b3f3203bb7fa2059	card "Storage Matrix"	selected_analysis "As long as this artifact is untapped, each player chooses artifact, creature, or land during their untap step. That player can untap only permanents of the chosen type this step."
newly covered	ff93fdac5136465c53a1440161de3497a3eb9d0889f91e8ac7aa523d33d85bd0	card "Coffin Puppets"	selected_analysis "Sacrifice two lands: Return this card from your graveyard to the battlefield. Activate only during your upkeep and only if you control a Swamp."
newly covered	ff958c763df2f779f921b8052ae6134419825531e2242d71400c5eb6c16e67cc	card "Mistveil Plains"	selected_analysis "This land enters tapped.\n{W}, {T}: Put target card from your graveyard on the bottom of your library. Activate only if you control two or more white permanents."
```

**Assurance:** restored 0; re-spelled 24 existing test functions (17 English
parser/AST/path witnesses and 7 construction-compiler inventory/feature
witnesses); ignored with blockers 0; added 0 test functions; removed 0. The
`#[test]` counts remain 49 in `ability_logic.rs`, 39 in `parser.rs`, and 109
in `predicate_grammar.rs`. The five bespoke restriction witnesses were
re-spelled in place; no asserted card/outcome pair was deleted.

**Deviations and additions:**

- The two 2026-09-05 coordinator rulings expand the original three-focus
  estimate to five positional Focus constructions and authorize the minimal
  Clause Tail seam. That seam accounts for the construction-count difference
  below and is recorded in `english-v2-subordinate-clause`.
- The compiler learned one exhaustive `Focus` feature and constant derived
  features on product alternatives of declared sums. The emitter also now
  handles optional lexical fields as optional patterns; the focus-bearing sum
  exposed that general code-generation defect.
- The newly introduced mixed postposed-tail coordination member accepts a
  `FocusedPredicateAdjunct`, not every Predicate Adjunct. This is the general
  structural shape required to coordinate `only during ... and only if ...`
  without licensing an unfocused marker-less Duration fragment.
- The required planned follow-up
  `english-v2-bare-duration-adjunct-licence` was added. No production fix for
  marker-less possessive durations is included.
- No construction, test function, vocabulary entry, frame, or route beyond
  the authorized Focus, Clause Tail, compiler-feature support, and declared
  Activate frame changes was added.

**glossary gap:** Subordinate Clause existed only as an unformalized concept,
and Adverb, Adverb Phrase, Focus, and Focus Adverb were absent. The Oracle
English glossary now defines those five established terms. **Clause Tail
remains a glossary gap:** the landing's project-coined entry was reverted at
review — the vocabulary of that file is the owner's, and the proposed
definition ("a subordinate or adjunct constituent that combines with a Clause
or Predicate at its boundary, whether it precedes or follows that host")
described this landing's category rather than an established notion. The
grammar identifiers `PreposedClauseTail` / `PostposedClauseTail` stand
undefined in the glossary until the owner names the concept.

**STOPs and resolutions:** The original three-category contradiction, the
whole-attachment-versus-tail contradiction, the focused bare-duration
negative diagnosis, and recursive doubled-focus negatives each stopped work
and remain recorded above. The coordinator's four 2026-09-05 rulings
respectively made the category enumeration grammar-derived, authorized the
Clause Tail seam, required full feature transparency and routed the
pre-existing bare-duration defect, and authorized the declared non-iterating
`Focus` feature. The final tree has no STOP. Decision wanted: none.

### REPORT

- Coverage lock `covered`: **18,917 -> 19,113 (+196 / -0)**. Every gained
  identity and selected analysis is in the pasted report-mode delta; no lost
  identity exists to classify or route.
- Construction inventory: **393 -> 389 (-4)**. Twenty-four declarations were
  removed: the five bespoke restriction constructions and nineteen former
  tail-containing attachment constructions. Twenty were added: fifteen
  Clause Tail member/host constructions preserving those general shapes and
  five positional Focus constructions. Thus the coordinator-authorized tail
  seam, not retained bespoke machinery, explains the difference from the
  ticket's original `-5 +3` estimate. Abstract sums are 45 -> 50; selection
  exceptions remain 0.
- `only` ownership: 0 form literals and one declared
  `FocusAdverb::Only` vocabulary spelling. Licensed vocabulary/lexicon
  homographs remain 2; form-literal/vocabulary overlaps remain 9; permitted
  licensing checkers remain 20 and forbidden checkers remain 0.
- Selection census: 32,641 total, 19,113 selected, 13,528 parse failures,
  14,983 unique, 4,130 specificity-resolved, 0 exception-resolved, 0 ties,
  and 0 internal failures.
- Performance advisory (implementer's pre-refresh pass, 8 workers): coverage
  109,959 ms, 115,492 ns/B, host load 8.02/9.01/7.94; ambiguity 109,652 ms,
  118,916 ns/B, host load 9.04/7.77/7.55; roundtrip 98,257 ms, 112,664 ns/B,
  host load 6.02/6.64/7.04. Its contention line ("0 concurrent build/test
  processes observed") is a sandbox artifact and is struck; see the review
  advisory below. The 16,260 ms advisory ceiling was exceeded under host load;
  per the ruling this is reported, not a STOP.
- Assurance: restored 0; re-spelled 24; ignored 0; added 0; removed 0.
- Coverage losses 0; wrong or negative gains 0; unresolved ties 0; roundtrip
  mismatches 0; forbidden or word-naming guards 0; deviations are enumerated
  above; decision wanted none.

### Review corrections

Opus landing review, 2026-09-05, after a second `kata refresh` onto trunk with
`english-v2-frame-complement-coordination` (B7a) integrated. Findings and the
fixes applied in change `tlywlpov`:

- **HIGH — the refreshed tree did not compile.** B7a's three new `VerbPhrase`
  constructions (`and_/or_/and_or_frame_complement_pair_coordination`) carry no
  `derive focus`, so after a textually clean merge `VerbPhrase` stopped
  providing the new `Focus` feature, `BarePredicate` stopped carrying it, and
  `focused_bare_predicate`'s `require focus.focus is Unfocused` failed with
  "role predicate `focus.focus` has no constructible feature expression". The
  merge was completed by deriving `focus = Values::Unfocused` on all three, in
  the refresh commit itself. A focus-bearing sum makes every later member of
  its constituent categories owe the feature; that obligation is now the
  standing cost of the `Focus` declaration.
- **HIGH — clippy red on a touched crate.** The new tail chain pushes
  auto-trait solving past the default evaluation depth, so `BuildValue: Send`
  became unprovable (`E0275: overflow evaluating the requirement
  Box<FocusedPredicateAdjunct>: Send`, reached through
  `PostposedClauseTailCoordinationMember`) and
  `clippy::arc_with_non_send_sync` fired twice in the untouched
  `parser/materialize.rs`. Clippy is green on trunk and was red here. Fixed
  with the compiler's own remedy, `#![recursion_limit = "256"]` on
  `deckmaste_english_v2`, not by suppressing the lint.
- **MEDIUM — focus transparency was incomplete.** The 2026-09-05 transparency
  ruling was applied to the two Predicate Adjunct classifiers but not to
  `object_is_mass_nominal` (`constructions.rs`), which matched
  `Object::ObjectNominal` and returned `false` for `Object::FocusedObject`, so
  a focused Object was classified non-mass while its operand was mass and a
  partitive whole changed admissibility under focus. Fixed by recursing
  through the wrapper, exactly as the adjunct classifiers do. These four are
  the complete set of Rust classifiers over a focus-bearing category.
- **MEDIUM — the new compiler feature had no compiled-consumer case.** The
  `Focus` feature and the new constant-feature-on-product-alternative support
  were exercised only through `deckmaste_english_v2`. Added
  `declared_two_value_feature_admits_one_wrapper_only` to
  `crates/deckmaste_construction/tests/compiled_consumer.rs`: a
  `FocusOperand` sum over a plain member and a wrapper with
  `require operand.focus is Unfocused`, asserting that one wrapper constructs
  and renders and that a second is refused both by the generated constructor
  and by the parse-time `build` rule.
- **MEDIUM — the glossary entry.** See the glossary-gap paragraph above: the
  project-coined **Clause Tail** entry was reverted; the five established
  entries stand.
- **MEDIUM — a routed block named a ticket that did not exist.** The 101-unit
  `only once each turn` block was routed to
  `english-v2-frequency-adverbial-family`; that ticket has been minted in
  `docs/tickets/planned/`.
- **LOW — a dead category.** `abstract sum ClauseTail` (twelve members) was
  declared and never used as a field type; the positional
  `PreposedClauseTail` / `SimplePostposedClauseTail` / `PostposedClauseTail`
  sums carry the seam. Deleted with its `ast.rs` and `visit.rs` re-exports and
  its two `xtask` diagnostic entries; abstract sums are 45 → 49, not 45 → 50.
  The handoff note in `english-v2-subordinate-clause` was re-worded to name
  the categories that exist.
- **LOW — wall times shaped like rule numbers.** The advisory's decimal
  seconds (three digits, a point, three more) read as bare CR citations to
  `cargo xtask cite check` and left it red with two hits; all advisory times
  are now comma-grouped milliseconds, as the E15 landing was corrected to do.
- **Not a finding.** The reject-loop oracle in `predicate_grammar.rs` changed
  from `parser.parse(text, …).is_err()` to `analysis.selected().is_none()`.
  `parse` is `analyze(…).into_parse_result()`, so the two are equivalent; the
  strict spelling was re-run against all eleven sentences and passes. The
  change only adds the analysis to the failure message.
- **Not a finding.** The acceptance line "`grep -c '\"only\"' … is 0" is
  unsatisfiable alongside the ticket's own `vocab FocusAdverb { Only = "only" }`
  requirement. The substantive criterion holds: one vocabulary spelling, zero
  form literals.

**Verification beyond the record.** The ticket's fenced rivalry — an
object-focus reading against an adjunct-focus reading of the same unit — does
not materialize anywhere: across the 3,661-card `only`/`unless`/`for as long
as` subset, **zero** selected units have two candidates that differ in which
Focus construction they traverse (measured from `ambiguity --json` candidate
paths). All 35 Object-focus units are `can block only creatures with flying`
or `can untap only permanents of the chosen type`; all 74 Predicate-Adjunct
focus units are `only during …` / `only before …` / `only once`; all 109
postposed-tail focus units are `only if …`. Both excluded blocks stay
excluded: every `only as a sorcery` and every `only once each turn` unit in
the subset is a parse failure.

**Corrected numbers on the gated tree (change `tlywlpov`).**

| census | parent trunk (B7a) | feature `tlywlpov` | delta |
|---|---:|---:|---:|
| lock `covered` / selected | 19,002 | 19,198 | +196 / −0 |
| parse failures | 13,639 | 13,443 | −196 |
| unique selections | 14,943 | 15,056 | +113 |
| specificity-resolved | 4,059 | 4,142 | +83 |
| exception-resolved | 0 | 0 | 0 |
| unresolved ties | 0 | 0 | 0 |
| constructions | 397 | 393 | −4 |
| abstract sums | 45 | 49 | +4 |

Structural laws on the gated tree: 32,641 units, 19,198 selected and covered,
0 selected-uncovered, 0 unresolved ties, 0 internal failures, 0 exception
resolutions or uses, 0 roundtrip mismatches, 0 ownership failures, construction
traversal 844,207/844,207, leaf traversal 293,649/293,649, 0 gap or overlap
spans, 0 synthetic claims, 0 provenance-plan mismatches, 21 permitted licensing
checkers and 0 forbidden, 2 licensed vocabulary/lexicon homographs, 9
form-literal/vocabulary overlaps.

Positive gate artifacts, all foreground from the workspace:

- `cargo fmt --all` clean (only the repository's stable-rustfmt warnings).
- `cargo clippy -p deckmaste_construction_core -p deckmaste_english_v2 -p
  deckmaste_construction -p xtask --all-targets --all-features -- -D warnings`
  → `Finished dev profile`, exit 0.
- `cargo test --workspace` → exit 0, 128 `test result: ok` lines, 0 failed
  (gate scope: `construction_core/src/emit/` and `plugins/builtin_v2/`).
- `DECKMASTE_COVERAGE_LOCK=report cargo xtask english_v2 coverage --check
  --workers 8` → exit 0, **0 newly covered, 0 no longer covered**, lock exactly
  current at `covered` 19,198.
- `cargo xtask english_v2 ambiguity --require-resolved --workers 8` → exit 0,
  `unresolved_ties=0`, `internal_failures=0`, `exception_uses=0`.
- `cargo xtask english_v2 roundtrip --require-clean --workers 8` → exit 0,
  `parse accepted 19198`, `clean 19198`, `mismatched 0`.
- `cargo xtask cite check --list-noncompliant` empty and `cite check` 0 stale.

**Performance advisory (review pass, 8 workers; contention: 2 concurrent
executors plus this review).** Coverage 106,001 ms, 119,966 ns/B, host load
4.37/6.99/8.70. Ambiguity 108,252 ms, 123,195 ns/B, host load 5.48/7.09/8.55.
Roundtrip 103,727 ms, 128,611 ns/B, host load 11.32/9.03/9.11. The 16,260 ms
quiet-host ceiling is exceeded under that contention; reported, not a STOP.

**Assurance after review:** restored 0; re-spelled 24; ignored with blockers 0;
**added 1** (`declared_two_value_feature_admits_one_wrapper_only`); removed 0.
Three sentences moved between the positive and negative lists in
`predicate_grammar.rs` and are named here because the implementer's counts did
not name them: `Cast only this spell if you control a snow land.` and `Draw
only a card.` moved from the reject list to the select list as intended
Object-focus gains, and `Cast this spell only your turn.` moved from the reject
list into the focused/unfocused pair assertion under the coordinator's third
ruling, with the underlying defect routed to
`english-v2-bare-duration-adjunct-licence`.

**Deviations and additions (review additions to the list above).**

- `ClauseTailBody` is `{Finite, Coordination}`, narrower than the old
  `postposed_if`'s `body: Clause`: a postposed tail can no longer take another
  postposed-tail clause as its body. The narrowing is what removes the tail /
  adjunct rivalry, costs zero coverage, and was disclosed only inside a
  superseded STOP paragraph; it belongs here.
- `PostposedClauseTailCoordinationMember` admits `SimplePostposedClauseTail`
  and `FocusedPredicateAdjunct`, not a bare `PredicateAdjunct` — a newly
  introduced member restricted at introduction, not an existing form narrowed.
- The compiler additions (the `Focus` feature, constant feature equations on
  product alternatives of a declared sum, and optional lexical fields as
  optional patterns in the emitter) are the implementer's, listed above; the
  review added only their compiled-consumer case.

**Full corpus passes used by this review: five** — `cargo test --workspace`
twice (once before and once after the `recursion_limit` fix, which is
compile-only), and `coverage --check`, `ambiguity --require-resolved` and
`roundtrip --require-clean` once each. All probing ran against a 3,661-card
`jq` subset under `~/Dump/review-tail-focus/subset.json`.
