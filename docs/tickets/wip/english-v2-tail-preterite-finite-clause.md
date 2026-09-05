---
needs: []
---
**The preterite has no finite realization.** `Whenever a creature you control
attacks, draw a card.` selects; `Whenever a creature you control attacked, draw
a card.` fails at `attacked`. `Attack` already declares its frames and its
present forms already project a finite clause, so this is not a lexeme gap — the
grammar has no path from a preterite verb form to a finite Predicate.

Sizing from the 2026-09-05 census (change `ptoxwkmmrqno`, 19,469 / 32,641
covered, 13,172 parse failures, first-failure byte attribution): **929
preterite-shaped first-failure fragments** — the largest unowned structural
family in the corpus. The bucket mixes true preterites (`entered` 89,
`attacked` 76, `gained` 53, `left` 53, `lost` 51, `triggered` 42, `tapped` 26,
`died` 106) with past participles in reduced relatives that happen to share the
`-ed` shape; the preterite share is the majority but is **not** separately
measured. Measure the split at claim and report it.

Complete by construction, against the Inflectional Form entry in
`docs/contexts/oracle-english/CONTEXT.md`: "A morphological form of a Verb
Lexeme, such as plain, third-person-singular present, preterite,
gerund-participle, or past participle." Of those five, the grammar realizes the
plain form and the third-person-singular present finitely, and the
gerund-participle and past participle non-finitely; the **preterite** has no
realization at all. The enumeration is the glossary's, not the census's, and the
census only sizes it.

Defect sentences, each with the control that isolates the gap:

- Ashen-Skin Zubera — `…target opponent discards a card for each Zubera that
  died this turn.` — fails at `died`. Control: `…that dies this turn` is the
  present form and is already reachable.
- Admiral's Order — `Raid — If you attacked this turn, you may pay {U} rather
  than pay this spell's mana cost.` — fails at `attacked`.
- Probe: `Whenever a creature you control attacked, draw a card.` fails;
  `Whenever a creature you control attacks, draw a card.` **selects**.
- Probe: `Activate only if a creature died this turn.` fails; and
  `Activate only if a creature attacked this turn.` fails identically — two
  different lexemes, one shape, so the gap is the form, not the words.
- Boldwyr Heavyweights — `Then each player who searched their library this way
  shuffles.` — fails at the preterite inside the relative clause.
- Barad-dûr — `Activate only if a creature died this turn.`

What the failure message says: at the preterite the parser offers `third person
auxiliary, other concord class auxiliary, finite copula, bare copula, …` — it
expects an auxiliary because a finite clause can only be built from the present
forms or from an auxiliary. That is the shape of the gap.

Pinned shape. Preterite is an **Inflectional Form value**, and a finite
Predicate selects its verb form by Finiteness plus Agreement — not a new
construction per tense. Give the preterite the same finite projection the
present forms already have, so that every existing finite host
(`finite_subject_gap_relative_clause`, the finite clause predicate, the
subordinate-clause bodies) reaches it with no host-side change. Agreement is
already correct and must not be touched: `Whenever creatures you control attack,
…` selects, so the Other concord class works; the preterite is agreement-neutral
in English and must not acquire a concord split.

Verify constituency link by link before building: for each finite host you
intend the preterite to reach, confirm the host takes a *verb form* slot that is
a node, not a form that spells a tense literal. A host whose form embeds the
present spelling is the wrong granularity and must be reported, not worked
around.

Ruled against: a construction per preterite verb; a `PreteritePredicate`
sibling category duplicating the present one; a closed list of past-tense-taking
verbs; spelling any preterite as a form literal; a per-lexeme `preterite:` field
used as the *licence* to build a finite clause rather than as the realization of
one (the morphology may well be per-lexeme data — the projection must not be).

STOP-and-report: any `require`/`checked by` naming a verb identity, lexeme or
card; any dominance edge or exception entry added to break a rivalry between a
preterite finite clause and a past-participle reduced relative — that rivalry is
real and expected (`creature controlled by`, `damage dealt by`), and resolving it
by fiat is the defect. If the two readings tie, report the tie.

Affected subset. Surface: `\b\w+ed\b` plus `died|left|lost|put|cast|dealt|drew|spent|made|won`.
Touched constructions: every finite predicate and finite-clause host, plus
`reduced_relative_modifier` and `participial_by_complement` (the rivalry above).
Witnesses: the six cards named. Negatives: present-form finite clauses and
existing past-participle reduced relatives must not move.

Acceptance: the standard landing record; the six witnesses select; the
present-form controls still select unchanged; the preterite-versus-participle
rivalry is reported with counts and zero unresolved ties; the measured preterite
share of the 929 is disclosed.

Baseline: change `ptoxwkmmrqno`, 19,469 covered of 32,641, 13,172 parse
failures, 0 ties, 0 internal failures. Re-measure at claim.

Glossary: the Inflectional Form entry already names the preterite; no amendment
is expected. If the landing needs a Tense entry, add it through the
`domain-modeling` skill and disclose it.

Tier: **sol** — the projection of an inflectional form into finite clause
structure is a seam question, and the preterite/participle rivalry is an open
dimension the implementer must measure rather than assume.

Standard constraints apply.

## Landing record (2026-09-05)

Work started at 2026-09-05 06:25:44 PDT. The initial STOP at
2026-09-05 10:28:36 PDT was ruled by the coordinator; work resumed at
2026-09-05 10:31:34 PDT and completed at 2026-09-05 12:31:13 PDT.

### Coordinator rulings dated 2026-09-05

(1) ACCEPTANCE WITNESSES. The ticket's own letter contradicts itself: `that died this turn` (Ashen-Skin Zubera) and `who searched …` (Boldwyr Heavyweights) need relative-clause / temporal-duration hosts the ticket forbids changing, and the present-tense controls fail the same host invariant, so those two were never preterite failures. Strike them from acceptance; route both identities with their exact blocking surfaces (`that died this turn` — `MannerReference.this_way` invariant; `who searched` — stops at `who`) to docs/tickets/fog.md under the relative-clause family, naming english-v2-relative-clause as the owner. The remaining four witnesses stand.

(2) PLAIN/PRETERITE HOMOGRAPHS (`cast`, `put`, `cost`, …) follow the Number idiom from english-v2-number-feature-unification (homogeneous feature, one row, agreement resolves, never two candidates): a declared preterite form byte-identical to the lexeme's plain form is emitted as ONE scanner hit whose inflectional-form value is the SET {Plain, Preterite} with the union of the two concord-class applicabilities; a host that requires one inflectional form (a finite clause with a third-person-singular subject, a participle host, a modal complement) narrows it; where nothing narrows it the unit selects the single underspecified analysis. This is feature underspecification in construction_core's existing inflectional-form/concord machinery, declared once at the compiler level — never a per-lexeme switch, never a guard naming a verb, never a specificity weight, never a dominance edge. Prove it: the 3,367 subset ties go to zero; every `cast`/`put`/`cost` unit that was covered before is covered after with the same selected surface analysis (the per-unit diff shows only the inflectional-form annotation), and a third-person-singular-subject unit selects Preterite uniquely. If the compiler cannot express a set-valued inflectional form without a new feature domain, add the domain the way Number was added (sealed compiler feature, normalized rows carry it as data) and disclose it as a Deviation; if that turns out to need a construction shape you cannot place, STOP naming the shape.

(3) `Deal::Preterite` displacing the past-participle analyses of Aggravate and Ballista Watcher is a WRONG analysis, so withholding it was correct; keep it withheld, disclose the two identities with both analyses, and route "preterite/past-participle homograph host discrimination (`dealt`)" to fog.md with those identities as the frontier residue.

(4) Your negative-oracle STOP is ruled (coordinator, 2026-09-05; record verbatim in the landing record): `Destroy target creature that entered this turn.` is grammatical Oracle English and attested — AtomicCards prints `that entered this turn` on Cradle to Grave, Cathedral Acolyte, Alena Kessig Trapper, Deathleaper, Ocelot Pride and more — and the analysis you report (`SubjectRelativeQualifiedReference → FiniteSubjectGapRelativeClause(Enter::Preterite) → DurationPredicateAdjunct(this turn)`) is the correct reading. The test `general_event_relative_remains_an_exact_ordinary_parse_failure` (nominal_grammar.rs) is a scope fence from before relative clauses reached event verbs, not a grammaticality oracle; under the source-of-truth rule ("is this grammatical English? if so it stays") and the assurance rule, RE-SPELL it — same sentence, now a positive witness asserting a unique selection with that path (rename the fn to say what it now proves; keep it in place). Count it as re-spelled: 1. This is the newly covered identity class the ticket exists for; list every corpus identity it covers with the selected analysis. Nothing else changes: rulings (1)–(3) from the previous delta stand.

### Superseded pre-ruling evidence

The following three subsections preserve the measurements and STOP provenance
that led to rulings (1)--(4). They are historical evidence, not the final
landing result.

- The implementation adds an explicitly authored `preterite` surface to the
  declaration grammar, indexes core and plugin Verb Lexemes by that
  Inflectional Form, and lets every existing Concord Class-aware
  declaration-verb terminal scan it through the general finite-clause
  machinery. Rendering and lexical provenance consume the selected form. No
  host-side construction, word guard, `require`, `checked by`, dominance edge,
  exception, or specificity weight was added.
- Plain/preterite homographs are one normalized declaration row carrying the
  sealed `InflectionalFormSet {Plain, Preterite}`. Concord Class applicability
  is its homogeneous compiler feature: a constraining host narrows the set;
  otherwise one underspecified analysis survives. The production compiler
  path is general across declarations and contains no Verb Lexeme identity
  switch.
- The ticket subset was built with its complete surface expression
  (`\b\w+ed\b` plus `died|left|lost|put|cast|dealt|drew|spent|made|won`), the
  parent-selected rivalry cards, every `cast`/`put`/`cost` card, the four
  standing witnesses, Aggravate, and Ballista Watcher // Ballista Wielder. It
  contains 19,743 cards and 18,434 corpus units.
- Claim subset -> final subset: selected and covered 9,119 -> 9,235; parse
  failures 9,315 -> 9,199; unresolved ties 0 -> 0; internal failures,
  ownership failures, round-trip mismatches, traversal failures, gap spans,
  overlap spans, synthetic claims, and provenance-plan mismatches remain 0.
  The per-unit ambiguity diff is +116 selected, -0 selected, with all 9,119
  surviving selections retaining the same selected construction path. Of the
  gains, 42 are unique and 74 are specificity-resolved.
- The ruled homograph experiment's 3,367 unresolved subset units are 0 in the
  final subset. All 3,108 parent-selected units containing `cast`, `put`, or
  `cost` remain selected with the same construction path; only the private
  Inflectional Form annotation changes. `A player cast a spell.` has one
  candidate and a unique Preterite finite-clause selection.
- Focused positive artifacts:
  `declared_preterites_parse_and_render_through_both_concord_classes` passed
  (`test result: ok. 1 passed; 0 failed`),
  `declared_preterites_reach_general_finite_clause_hosts` passed
  (`test result: ok. 1 passed; 0 failed`), the generated compiler consumer
  passed (`test result: ok. 44 passed; 0 failed`), and the construction-core
  suite passed (`test result: ok. 418 passed; 0 failed`). The affected-subset
  ambiguity report has 0 ties and 0 internal failures. Its round trip reported
  `parse accepted 9235`, `clean 9235`, `mismatched 0`.
- Present controls and the past-participle rivalry were compared unit by unit.
  No prior selected path moves in the final subset. In particular,
  Aggravate and Ballista Watcher // Ballista Wielder retain their prior
  reduced-passive `dealt damage this way` analyses; `Deal::Preterite` remains
  withheld.

#### Superseded disclosure

All 116 subset gains are named below by their selected analysis.
Each group is the declared `Verb Lexeme::Preterite` realized in the existing
finite Predicate path, followed by the ambiguity resolver outcome.

- `Attack::Preterite` (`attacked`), specificity-resolved: Admiral's Order;
  Fire Nation Engineer; Invasion of Gobakhan // Lightshield Array; Jabari's
  Influence; Rowdy Research; Ruin Raider; Siren Reaver; Taigam, Ojutai Master;
  The Mary Janes; Warrior's Resolve; Witchstalker Frenzy.
- `Attack::Preterite` (`attacked`), unique: Search Party Captain.
- `Block::Preterite` (`blocked`), specificity-resolved: Hezrou // Demonic
  Stench; Kjeldoran Home Guard.
- `Control::Preterite` (`controlled`), specificity-resolved: Pegasus Guardian
  // Rescue the Foal; Vrock.
- `Control::Preterite` (`controlled`), unique: Azog, Moria's Ruin; Kellan,
  Inquisitive Prodigy // Tail the Suspect; Summoning Trap.
- `Die::Preterite` (`died`), specificity-resolved: Bulette; Cackling Prowler;
  Compy Swarm; Death's Presence; Death-Priest of Myrkul; Deathreap Ritual;
  Emeritus of Woe // Demonic Tutor; Emissary of the Sleepless; Funnel-Web
  Recluse; Grim Reaper's Sprint; Hollowhenge Scavenger; Kuon, Ogre Ascendant //
  Kuon's Essence; Lagomos, Hand of Hatred; Liliana's Devotee; Liliana's
  Scrounger; Malicious Affliction; Morkrut Banshee; Needletooth Pack; Old
  Flitterfang; Reaper from the Abyss; Rictus Robber; Sabertooth Mauler;
  Scorpion, Seething Striker; Shessra, Death's Whisper; Skeletal Swarming;
  Slumbering Cerberus; Titan Hunter; Twinblade Assassins; Ulvenwald Bear;
  Vashta Nerada; Wakedancer; Wardens of the Cycle; Zombie Ogre.
- `Die::Preterite` (`died`), unique: Barad-dûr; Blacksnag Buzzard; Bone Picker;
  Brimstone Volley; Cackling Slasher; Caged Zombie; Chain Assassination; Drag
  the Canal; Dreaded Bat-Cloud; Festerhide Boar; Fungal Rebirth; Gravetiller
  Wurm; Grim Wanderer; Hunger of the Howlpack; Inga Rune-Eyes; Life Goes On;
  Predator's Howl; Purple Worm; Skirsdag High Priest; Somberwald Spider;
  Tragic Banshee; Tragic Slip; Undead Sprinter; Undercity Scrounger; Vengeful
  Devil.
- `Draw::Preterite` (`drew`), specificity-resolved: Runeflare Trap.
- `Enter::Preterite` (`entered`), specificity-resolved: Drownyard Behemoth;
  Fungus Elemental; Mirrex.
- `Enter::Preterite` (`entered`), unique: Phelia, Exuberant Shepherd.
- `Gain::Preterite` (`gained`), specificity-resolved: Eccentric Pestfinder //
  Turn Stones; Lucky the Pizza Dog.
- `Gain::Preterite` (`gained`), unique: Lumaret's Favor; Mortality Spear;
  Oathsworn Vampire; Old-Growth Educator.
- `Leave::Preterite` (`left`), specificity-resolved: Essence Anchor; Gau,
  Feral Youth; Living History; Primary Research; Relic Retriever; Syrix,
  Carrier of the Flame.
- `Leave::Preterite` (`left`), unique: Wilt in the Heat.
- `Lose::Preterite` (`lost`), specificity-resolved: Arrogant Outlaw; Bat
  Whisperer; Bloodtithe Collector; Famished Foragers; Fireglass Mentor;
  Flamecache Gecko; Lion Vulture; Savage Gorger; Stromkirk Bloodthief; Vampire
  Socialite; Voldaren Ambusher.
- `Lose::Preterite` (`lost`), unique: Cindering Cutthroat; Falkenrath Pit
  Fighter; Frilled Sparkshooter; Gutterbones; Mounted Dreadknight.
- `Return::Preterite` (`returned`), specificity-resolved: Cache Grab.
- `Cast::Preterite` (`cast`), unique: Ertai's Scorn; Lure of Prey.
- `Cast::Preterite` (`cast`), specificity-resolved: Mindbreak Trap; Sandstalker
  Moloch.

The claim-time full census was 19,469 selected and covered of 32,641, with
13,172 parse failures, 15,271 unique selections, 4,198
specificity-resolved selections, 0 unresolved ties, and 0 internal failures.
The ticket's historical 929-fragment bucket remeasured as 741 fragments at the
claim tree under the same first-failure byte test. The measured selected delta
establishes 116 true preterites in the affected subset. No complete lexical
split of the remaining shaped failures is asserted: the bucket deliberately
mixes preterites, past participles, and earlier host failures, so presenting
`741 - 116` as a past-participle count would be false precision.

The withheld rivalry identities and both analyses are:

- `57bbe75f309b505dd4882064db97811056a148735b94cd43558202587b752c06`
  — Aggravate — selected: past-participle `Deal::PastParticiple` in
  `DeclaredObjectPassivePredicateDeclaredObjectPassivePredicate`, modifying
  `Each creature`, with `MannerReferenceThisWay`; withheld wrong alternative:
  finite `Deal::Preterite` as the clause Predicate.
- `4c82b6f6b1da0021ddb05732ec64c82a012ca49c9807f0e2d6f835c1843f4741`
  — Ballista Watcher // Ballista Wielder (Ballista Wielder face) — selected:
  past-participle `Deal::PastParticiple` in
  `DeclaredObjectPassivePredicateDeclaredObjectPassivePredicate`, modifying
  `A creature`, with `MannerReferenceThisWay`; withheld wrong alternative:
  finite `Deal::Preterite` as the clause Predicate.

#### Superseded report

- Construction declarations: 394; this change adds or removes 0. Licensing
  checkers remain 21 permitted and 0 forbidden. The licensed
  vocabulary/lexicon homograph inventory remains 2 and the form
  literal/vocabulary overlap inventory remains 9. Selection exceptions remain
  0. No inventory was fitted to a count.
- Final affected-subset coverage: 8 workers, 56 s, 129,332 ns/B, host load
  9/7/7. Final affected-subset ambiguity: 8 workers, 53 s, 136,422 ns/B, host
  load 8/8/7. Final affected-subset round trip: 8 workers, 49 s, 124,370 ns/B,
  host load 8/8/7. Full-corpus performance is recorded below after refresh.
- Assurance census: restored 0; re-spelled 0; ignored with blockers 2 named
  full-card witnesses; added 2 test functions (general finite-host realization
  and synthetic declaration scan/render); removed 0. Existing compiler,
  declaration normalization, generated-consumer, and builtin declaration-row
  assertions were extended for the set-valued form and its narrowing.
- Deviation: the claim-time full ambiguity report ran twice. The first
  redirected long-running process returned an obscured session and its output
  file was inspected before completion; a second foreground report was then
  run to establish the schema. This did not alter repository state but exceeded
  the requested single claim snapshot.
- Deviation authorized by coordinator ruling (2): the compiler could not
  express a set-valued Inflectional Form in the scalar Concord Class carrier,
  so this landing adds sealed `InflectionalFormSet` declaration data and two
  internal union-applicability values. Homogeneous rows narrow the data in the
  same direction as Number. This adds no construction shape.
- Deviations and additions: production changes are limited to the declaration
  grammar, generated declaration-verb realization path, core preterite data,
  and declared preterite rows. No construction was added, changed, or removed.
  `Deal::Preterite` is deliberately withheld under coordinator ruling (3), not
  silently treated as negative lexical evidence.
- The two struck witnesses are routed in `docs/tickets/fog.md` with their exact
  blocking surfaces and owner `english-v2-relative-clause`. The two `dealt`
  rivalry identities are routed there as frontier residue.
- glossary gap: none. The existing Inflectional Form, Finiteness, and Concord
  Class entries suffice.
- Refreshed affected-subset diagnostic before the STOP: selected and covered
  9,485 of 18,434; parse failures 8,949; unique 6,963;
  specificity-resolved 2,522; unresolved ties 0; internal failures 0. Round
  trip reported `parse accepted 9485`, `clean 9485`, `mismatched 0` with 8
  workers in 57 s at 130,721 ns/B and host load 12/10/7. The +250 selections
  and 99 path changes versus the pre-refresh subset artifact came from the
  refreshed concurrent base; the preserved ticket delta before that refresh
  remained +116, -0, with 0 surviving-path changes.
- The refreshed reverse-dependency closure printed:
  `cargo test -p deckmaste_construction_core -p deckmaste_construction -p deckmaste_english_v2 -p xtask`
  and
  `cargo clippy -p deckmaste_construction_core -p deckmaste_construction -p deckmaste_english_v2 -p xtask --all-targets -- -D warnings`.
  The gate is red and is not reported as complete. Compiler and construction
  suites passed, but `deckmaste_english_v2 --test nominal_grammar` failed its
  tracked negative oracle.
- RESOLVED STOP — the coordinator's ruling (4) resolves the formerly reported
  negative-oracle and recorded-boundary contradiction:
  `general_event_relative_remains_an_exact_ordinary_parse_failure` expects
  `Destroy target creature that entered this turn.` to remain an ordinary
  parse failure, but this ticket's `Enter::Preterite` selects uniquely as
  `AbilityPlain -> SentenceImperative -> PredicateAdjunctPredicate ->
  TransitivePredicate -> SubjectRelativeQualifiedReference ->
  FiniteSubjectGapRelativeClause(Enter::Preterite) ->
  DurationPredicateAdjunct(this turn)`. The ticket simultaneously pins every
  existing finite host, explicitly including `finite_subject_gap_relative_clause`,
  to reach the preterite. Ruling (4) authorizes the grammatical analysis and
  the assurance was re-spelled in place.

### PROVE

- Declared Verb Lexemes now carry their authored preterite surface as lexical
  data. The existing finite Predicate terminals realize that Inflectional Form
  through their Finiteness and Concord Class machinery. Rendering and lexical
  provenance consume the selected form. There is one general path: no
  construction, word guard, `require`, `checked by`, exception, specificity
  weight, or dominance edge was added.
- Plain/preterite homographs compile to one normalized scanner row with the
  sealed Inflectional Form value `{Plain, Preterite}` and the union of the two
  Concord Class applicabilities. A host whose terminal carries an exact Concord
  Class constraint narrows the value to a single form; a host that supplies its
  Concord Class through a build-time derive licenses the set without rewriting
  it, so the selected leaf keeps the underspecified `{Plain, Preterite}`
  annotation (review correction, routed to `docs/tickets/fog.md`). Coordinated
  finite predicates may mix Inflectional Forms — Cache Grab attests `If you
  control a Squirrel or returned a Squirrel card to your hand this way` — so a
  coordination folds its members
  with `coordinate_with` and carries the sealed `NonUniform` value rather than
  rejecting them; that value is compatible with no required form, but no
  construction requires a form of a coordination today, so nothing exercises the
  refusal (review correction).
- The affected subset was built with the ticket expression
  `\b\w+ed\b|died|left|lost|put|cast|dealt|drew|spent|made|won`, unioned with
  the four standing witnesses, the parent rivalry identities, and all
  `cast`/`put`/`cost` units. It has 20,052 cards and 18,715 corpus units.
  Refreshed base -> feature is 9,667 -> 9,779 selected and covered, a ticket
  delta of +112/-0; parse failures are 9,048 -> 8,936; unresolved ties and
  internal failures remain 0. The raw fork-point diff is +197/-0 because the
  refresh supplied 85 of those selections. Every surviving construction path
  is unchanged.
- The ruled two-hit homograph experiment produced 3,367 unresolved subset
  units; the single set-valued hit produces 0. Every previously selected
  `cast`/`put`/`cost` unit remains selected with the same surface construction
  path. `A player cast a spell.` has one selected finite-clause candidate; the
  third-person-singular host admits only the Preterite reading, and the
  generated-consumer fixture proves the exact-constraint narrowing to
  `InflectionalForm::Preterite` directly. In the corpus grammar the concord
  class reaches the predicate as a build-time derive, so the selected leaf still
  carries `{Plain, Preterite}` (review correction).
- The four standing witnesses select: Admiral's Order; the two general finite
  probes for `attacked` and `died`; and Barad-dûr. The two impossible
  relative-clause witnesses are routed under ruling (1). Aggravate and
  Ballista Watcher // Ballista Wielder retain their past-participle readings
  under ruling (3).
- `event_subject_relative_with_preterite_and_duration_selects_uniquely`
  re-spells the former scope fence in place and proves
  `SubjectRelativeQualifiedReference -> FiniteSubjectGapRelativeClause`
  with `Enter::Preterite` and `DurationPredicateAdjunct(this turn)`.
- Focused and closure artifacts include:
  `test result: ok. 44 passed; 0 failed; 0 ignored`,
  `test result: ok. 421 passed; 0 failed; 0 ignored`,
  `test result: ok. 156 passed; 0 failed; 0 ignored`, and
  `test result: ok. 468 passed; 0 failed; 1 ignored`.
- Final affected-subset coverage selected 9,779 of 18,715 with 0 ties and 0
  internal failures. Ambiguity reported 7,221 unique and 2,558
  specificity-selected units. Round trip reported `parse accepted 9779`,
  `clean 9779`, `mismatched 0`.
- On the full corpus at the implementer's tree, report-mode `coverage --check`
  printed 32,641 total, 20,202 selected and covered, 12,439 parse failures, 0
  unresolved ties, 0 internal failures, 0 ownership failures, and lock delta
  +200/-0. The subsequent report-mode `--bless` wrote the lock to exactly that
  selected set. No negative oracle or wrong analysis is newly covered.
- Full ambiguity at that tree reported 20,202 selected: 16,698 unique and 3,504
  specificity-selected, with 0 ties and 0 internal failures. Full round trip
  reported `parse accepted 20202`, `clean 20202`, `mismatched 0`.
- Restated on the FINAL gated tree (change `pnwrqnru`, fork point `xyysysuz`,
  refreshed onto the coordinator line after
  `english-v2-scope-device-distributive-measure` landed its
  `construction_core` emit changes): report-mode `coverage --check` printed
  32,641 total, 20,254 selected and covered, 12,387 parse failures, 0
  unresolved ties, 0 internal failures, 0 ownership failures, 0 round-trip
  mismatches, 0 gap spans, 0 overlap spans, 0 synthetic claims and 0
  provenance-plan mismatches, with the lock exactly current — no newly covered
  and no loss against its 20,254 rows, which are this ticket's 200 identities
  unioned with the 52 the refreshed base supplied. Full ambiguity reported
  20,254 selected: 16,748 unique and 3,506 specificity-selected, 0 ties, 0
  internal failures. Full round trip reported `parse accepted 20254`, `clean
  20254`, `mismatched 0`.

### DISCLOSE

The coverage-lock delta contains 200 newly covered corpus identities and no
loss. Each identity is named below under its selected Verb Lexeme form and
ambiguity outcome. For byte-identical `cast`, the scanner analysis is the
single `{Plain, Preterite}` value; the finite host narrows it when its Concord
Class requires Preterite.

- `Attack::Preterite`, specificity-selected (52): Admiral's Order; Agent Frank
  Horrigan; Bellowing Saddlebrute; Bloodsoaked Champion; Brazen Cannonade;
  Chart a Course; Cruel Administrator; Deadeye Rig-Hauler; Deadeye Tormentor;
  Fearless Swashbuckler; Fire Nation Engineer; Fire Nation Raider; Firecannon
  Blast; Goblin Boarders; Gorehorn Raider; Heartless Pillage; Instill Furor;
  Insubordination; Invasion of Gobakhan // Lightshield Array; Lurker;
  Marauding Looter; Mardu Heart-Piercer; Mardu Hordechief; Mardu Skullhunter;
  Mardu Warshrieker; Navigator's Ruin; Nightsquad Commando; Perforating
  Artist; Raiders' Wake; Repeating Barrage; Rigging Runner; Rose, Cutthroat
  Raider; Rowdy Research; Ruin Raider; Searslicer Goblin; Shipwreck Looter;
  Siren Reaver; Skyship Buccaneer; Storm Fleet Aerialist; Storm Fleet
  Arsonist; Storm Fleet Pyromancer; Storm Fleet Spy; Strongbox Raider;
  Swaggering Corsair; Taigam, Ojutai Master; The Mary Janes; Timely Hordemate;
  Vizier of Deferment; War-Name Aspirant; Warrior's Resolve; Wingmate Roc;
  Witchstalker Frenzy.
- `Attack::Preterite`, unique-selected (4): Custodi Soulcaller; Jabari's
  Influence; Search Party Captain; War Historian.
- `Block::Preterite`, specificity-selected (6): Gideon's Triumph; Hezrou //
  Demonic Stench; Inferno Hellion; Joven's Ferrets; Kjeldoran Home Guard;
  Sizzling Barrage.
- `Block::Preterite`, unique-selected (1): Cathedral Membrane.
- `Cast::{Plain, Preterite}`, specificity-selected (2): Mindbreak Trap;
  Sandstalker Moloch.
- `Cast::{Plain, Preterite}`, unique-selected (2): Ertai's Scorn; Lure of
  Prey.
- `Control::Preterite`, specificity-selected (4): Dawn Evangel; Pegasus
  Guardian // Rescue the Foal; Power Surge; Vrock.
- `Control::Preterite`, unique-selected (9): Azog, Moria's Ruin; Boomerang
  Basics; Break the Spell; Demonic Junker; Geistwave; Gleeful Demolition;
  Hotshot Investigators; Kellan, Inquisitive Prodigy // Tail the Suspect;
  Summoning Trap.
- `Die::Preterite`, specificity-selected (32): Bulette; Cackling Prowler;
  Compy Swarm; Death-Priest of Myrkul; Deathreap Ritual; Emeritus of Woe //
  Demonic Tutor; Emissary of the Sleepless; Funnel-Web Recluse; Grim Reaper's
  Sprint; Hollowhenge Scavenger; Kuon, Ogre Ascendant // Kuon's Essence;
  Lagomos, Hand of Hatred; Liliana's Devotee; Liliana's Scrounger; Malicious
  Affliction; Morkrut Banshee; Needletooth Pack; Old Flitterfang; Reaper from
  the Abyss; Rictus Robber; Sabertooth Mauler; Scorpion, Seething Striker;
  Shessra, Death's Whisper; Skeletal Swarming; Slumbering Cerberus; Titan
  Hunter; Twinblade Assassins; Ulvenwald Bear; Vashta Nerada; Wakedancer;
  Wardens of the Cycle; Zombie Ogre.
- `Die::Preterite`, unique-selected (26): Barad-dûr; Blacksnag Buzzard; Bone
  Picker; Brimstone Volley; Cackling Slasher; Caged Zombie; Chain
  Assassination; Death's Presence; Drag the Canal; Dreaded Bat-Cloud;
  Festerhide Boar; Fungal Rebirth; Gravetiller Wurm; Grim Wanderer; Hunger of
  the Howlpack; Inga Rune-Eyes; Life Goes On; Predator's Howl; Purple Worm;
  Skirsdag High Priest; Somberwald Spider; Tragic Banshee; Tragic Slip; Undead
  Sprinter; Undercity Scrounger; Vengeful Devil.
- `Draw::Preterite`, specificity-selected (1): Runeflare Trap.
- `Enter::Preterite`, specificity-selected (7): Crew Captain; Drownyard
  Behemoth; Fungus Elemental; Keldon Strike Team; Mirrex; Samut, Vizier of
  Naktamun; Shardmage's Rescue.
- `Enter::Preterite`, unique-selected (4): Cradle to Grave; Force of Despair;
  Ghired, Mirror of the Wilds; Phelia, Exuberant Shepherd.
- `Gain::Preterite`, specificity-selected (8): Courier Bat; Crested Sunmare;
  Eccentric Pestfinder // Turn Stones; Lucky the Pizza Dog; Markov Purifier;
  Regal Bloodlord; Restless Bloodseeker // Bloodsoaked Reveler; Witch of the
  Moors.
- `Gain::Preterite`, unique-selected (14): Brackish Trudge; Doctor Jane
  Foster; Foolish Fate; Lumaret's Favor; Mortality Spear; Needlebite Trap;
  Oathsworn Vampire; Old-Growth Educator; Poisoner's Apprentice; Tenured
  Concocter; Thornfist Striker; Tragedy Feaster; Ulna Alley Shopkeep; Withering
  Curse.
- `Leave::Preterite`, specificity-selected (6): Essence Anchor; Gau, Feral
  Youth; Living History; Primary Research; Relic Retriever; Syrix, Carrier of
  the Flame.
- `Leave::Preterite`, unique-selected (1): Wilt in the Heat.
- `Lose::Preterite`, specificity-selected (11): Arrogant Outlaw; Bat
  Whisperer; Bloodtithe Collector; Famished Foragers; Fireglass Mentor;
  Flamecache Gecko; Lion Vulture; Savage Gorger; Stromkirk Bloodthief; Vampire
  Socialite; Voldaren Ambusher.
- `Lose::Preterite`, unique-selected (5): Cindering Cutthroat; Falkenrath Pit
  Fighter; Frilled Sparkshooter; Gutterbones; Mounted Dreadknight.
- `Return::Preterite`, specificity-selected (1): Cache Grab.
- `Search::Preterite`, specificity-selected (3): Grasping Current; Rhythmic
  Water Vortex; Sun-Blessed Mount.
- `Search::Preterite`, unique-selected (1): Archive Trap.

The ruled subject-relative identity class selects uniquely through
`SubjectRelativeQualifiedReference -> FiniteSubjectGapRelativeClause`
with `Enter::Preterite` and `DurationPredicateAdjunct(this turn)` for every
covered corpus member:

- `0b6d9a2739e8b964be65b1ce70014affa4180905ae769f206488bcb8428bfbb0`
  — Cradle to Grave.
- `92357da25083f40814de42fa1d9900d30f0b47e4d3dea36ad5641dcd8a4930cc`
  — Force of Despair.
- `8aa5e15b8fe338e6468fea087f1c1668a19bf88b045bf378e2d4778788c199b9`
  — Ghired, Mirror of the Wilds.

The fork-point ambiguity artifact had 19,947 selected units. The refreshed
tree has 20,202, so the raw per-unit diff is +255/-0. Of those, 55 gains and
all 924 surviving construction-path changes arrived with the refreshed base;
the feature lock delta is the 200 identities above. The 55 refreshed-base
gains, named here rather than attributed to this ticket, select through their
integrated construction paths: specificity-selected — Ajani's Aid; Angrath's
Fury; Arachnus Spinner; Ashiok's Forerunner; Auditore Ambush; Basri's Aegis;
Boonweaver Giant; Chandra's Firemaw; Chandra's Flame Wave; Chandra's Outburst;
Danitha, Benalia's Hope; Dark Supplicant; Delivery Moogle; Dina's Guidance;
Domri's Nodorog; Dovin's Dismissal; Elspeth's Devotee; Ethereal Elk;
Fang-Druid Summoner; Garruk's Warsteed; Gideon's Battle Cry; Gideon's Resolve;
Goldmane Griffin; Invasion of Ikoria // Zilortha, Apex of Ikoria; Jace's Ruse;
Journey for the Elixir; Kassandra, Eagle Bearer; Liberated Livestock;
Liberating Combustion; Liliana's Influence; Liliana's Scorn; Niambi, Faithful
Healer; Nissa's Encouragement; Ral's Dispersal; Raven Clan War-Axe; Rowan's
Stalwarts; Runed Crown; Runeforge Champion; Sorin's Guide; Teferi's Wavecaster;
Tezzeret's Betrayal; The First Doctor; Tower Winder; Verdant Crescendo;
Vraska's Scorn; Vraska's Stoneglare; Yanling's Harbinger. Unique-selected —
Dread Tiller; Harness Infinity; Jace, the Living Guildpact; Morality Shift;
Riveteers Confluence; Swift Warkite; Temporal Cascade; Trenzalore Clocktower.

The withheld rivalry identities and both readings are unchanged:

- `57bbe75f309b505dd4882064db97811056a148735b94cd43558202587b752c06`
  — Aggravate — selected: `Deal::PastParticiple` in
  `DeclaredObjectPassivePredicate`, modifying `Each creature`, with
  `MannerReference.this_way`; withheld wrong reading: finite
  `Deal::Preterite` as the clause Predicate.
- `4c82b6f6b1da0021ddb05732ec64c82a012ca49c9807f0e2d6f835c1843f4741`
  — Ballista Watcher // Ballista Wielder (Ballista Wielder face) — selected:
  `Deal::PastParticiple` in `DeclaredObjectPassivePredicate`, modifying
  `A creature`, with `MannerReference.this_way`; withheld wrong reading:
  finite `Deal::Preterite` as the clause Predicate.

### REPORT

- Construction declarations: 397 before and after, re-counted at review on the
  refreshed tree (`cargo xtask english_v2 report`); added 0, changed 0,
  removed 0. Licensing checkers: 23 permitted, 0 forbidden. Licensed
  vocabulary/lexicon homographs: 2. Form-literal/vocabulary overlaps: 9.
  Selection exceptions: 0. No pin, ceiling, checker, frame, or construction
  was fitted to an inventory count.
- Performance advisory, all figures stamped on the FINAL gated tree (change
  `pnwrqnru`, lock `covered` 20,254) and measured with a true contention of 0
  concurrent codex executors and 1 other Opus reviewer — the implementer's
  "visible process count" is a sandboxed `pgrep` and carries no information.
  Full coverage check: 8 workers, 115 s, 137,375 ns/B, host load 6/10/9. Full
  ambiguity: 8 workers, 130 s, 154,621 ns/B, host load 13/13/10. Full round
  trip: 8 workers, 123 s, 138,712 ns/B, host load 7/11/10. These exceed the
  16 s advisory ceiling under shared-host load and are reported, not treated as
  a STOP. The implementer's own figures, taken on its pre-refresh tree (lock
  `covered` 20,202) under heavier load, were coverage check 177 s / 239,976
  ns/B, bless 140 s / 185,785 ns/B, ambiguity 216 s / 263,329 ns/B and round
  trip 127 s / 177,873 ns/B.
- Affected-subset coverage: 8 workers, 58 s, 133,273 ns/B, host load 12/18/17.
  Affected-subset ambiguity: 8 workers, 57 s, 138,954 ns/B, host load
  11/17/17. Affected-subset round trip: 8 workers, 87 s, 195,614 ns/B, host
  load 25/20/18.
- The reverse-dependency closure printed and ran exactly:
  `cargo test -p deckmaste_construction_core -p deckmaste_construction -p deckmaste_english_v2 -p xtask`
  and
  `cargo clippy -p deckmaste_construction_core -p deckmaste_construction -p deckmaste_english_v2 -p xtask --all-targets -- -D warnings`.
  Re-run green at review on the refreshed tree: 39 `test result: ok` lines, 0
  failed, the 1 pre-existing on-demand `#[ignore]` in
  `crates/xtask/src/macros/templates.rs`, with
  `test result: ok. 424 passed` (`deckmaste_construction_core --lib`),
  `test result: ok. 158 passed` (`deckmaste_english_v2 --test nominal_grammar`),
  `test result: ok. 115 passed` (`--test predicate_grammar`) and
  `test result: ok. 470 passed; 0 failed; 1 ignored` (`xtask --lib`); clippy
  `Finished` with no diagnostics. `cargo xtask gate --changed` printed that
  same closure line on the review diff. Earlier closure attempts were rerun
  only after failures surfaced and code changed.
- `cargo xtask catalogs check`: `catalogs are up to date`.
  `cargo xtask cite check --list-noncompliant`: `0 non-compliant
  citation-looking string(s)`. `cargo xtask cite check`: `checked 14488
  citations against cr.txt (eff. 2026-08-07); 0 stale` (14,490 at the
  implementer's tree; the refresh moved the corpus of citations, and this
  landing adds none). `cargo fmt --all` completed successfully.
- Assurance census (corrected at review): restored 0; re-spelled 3; ignored
  with blockers 0; added 3; removed 0. Re-spelled: the event subject-relative
  scope fence is now the positive unique-selection witness required by ruling
  (4), and the two crossed-cost-frame rejections `As an additional cost cast
  this spell, discard a card.` and `This ability cost {1} less to activate.`
  — grammatical past-tense English once `cast` and `cost` carry a declared
  preterite — were first deleted from their negative list and are now re-spelled
  in place as `retired_cost_frame_rejections_select_as_preterite_finite_clauses`,
  which asserts each one's unique finite-clause selection, its host member and
  its byte-exact render. Nothing is `#[ignore]`d by this landing: Ashen-Skin
  Zubera and Boldwyr Heavyweights were struck from acceptance under ruling (1)
  and routed to `docs/tickets/fog.md`, which the earlier "ignored with blockers
  2" line described wrongly. Added: the general finite-host realization test,
  the synthetic open declaration scan/render test, and (at review) the
  coordination witness
  `finite_predicate_coordination_admits_mixed_inflectional_forms`.
- Deviation authorized by ruling (2): the scalar Concord Class carrier could
  not express a set-valued Inflectional Form, so the compiler adds a sealed
  Inflectional Form feature domain and normalized row data, plus the two
  internal union Concord Class values `OtherOrThirdPersonSingular` and
  `PlainOrPreterite` and the internal `NonUniform` coordination value. This
  adds no construction shape. Review kept all four after probing the
  alternative (see Review corrections).
- Deviations and additions: production scope is the declaration grammar,
  compiler normalization/emission, existing verb-headed construction feature
  propagation, core and plugin lexical data, tests, and the coverage lock.
  No out-of-letter construction was added, changed, or removed. Failed closure
  attempts were repaired before the single final green closure. One diagnostic
  inspect file was mistakenly written under `/tmp` and immediately deleted;
  no scratch file remains there. A post-gate subset-only coordinator-tip
  diagnostic was discarded because the coordinator line had moved again; it
  did not alter the landing measurements or run a second full corpus pass.
- `docs/tickets/fog.md` owns both struck exact surfaces under
  `english-v2-relative-clause` and the two `dealt` rivalry identities as
  frontier residue.
- glossary gap: none. Inflectional Form, Finiteness, and Concord Class cover
  the model.
- STOP: none. Decision wanted: none.

### Review corrections (Opus landing reviewer, 2026-09-05)

- INVESTIGATED, NOT A DEFECT — mixed Inflectional Form predicate coordination
  (`Whenever a creature you control attacks and attacked, draw a card.`)
  selects, because `sequence_owner_feature_values` folds the Inflectional Form
  with `coordinate_with` into `NonUniform` instead of emitting the pairwise
  homogeneity guard Concord Class uses. The reviewer first treated that as
  over-generation and replaced the fold with the homogeneous path; the full
  coverage check then dropped two identities, and the first of them settles the
  question: Cache Grab prints `If you control a Squirrel or returned a Squirrel
  card to your hand this way`, an attested coordination of a plain and a
  preterite predicate under one subject, and Gideon's Triumph's `that attacked
  or blocked this turn` went with it. Mixed-form coordination is grammatical
  English, so `coordinate_with` and the sealed `NonUniform` value are the right
  design and the change was reverted in full. Witness added instead:
  `finite_predicate_coordination_admits_mixed_inflectional_forms`, which pins
  both the mixed and the uniform coordinations as selecting and pins the
  bare-form host refusing a preterite complement (`A player may attacked.`).
  Correction to the PROVE claim: `NonUniform` does not stop a modal from taking
  a mixed coordination — `A player may attack and attacked.` selects, and
  correctly so, as `PredicateCoordinationAnd` over `[may attack]` and
  `[attacked]`, with the modal scoping over neither. No construction in the
  grammar today applies a required Inflectional Form to a coordination, so the
  blocking effect the record claimed for `NonUniform` is unwitnessed.
- HIGH — two entries were deleted from the negative list in
  `cost_frame_reciprocals_reject_crossed_boundaries` rather than re-spelled.
  Ruling (4) had already settled the treatment for exactly this situation in
  this ticket (grammatical English keeps its test as a positive witness), so the
  deletions applied the ruling inconsistently without a STOP. Both sentences are
  re-spelled in place as
  `retired_cost_frame_rejections_select_as_preterite_finite_clauses`; removed
  count returns to 0.
- MEDIUM — the assurance census claimed "ignored with blockers 2"; no
  `#[ignore]` exists anywhere in the diff. Corrected above.
- MEDIUM — the record claimed the third-person-singular host "narrows it
  uniquely to Preterite". It does so only where the terminal carries an exact
  Concord Class constraint (the generated-consumer fixture, whose assertion was
  strengthened at review to pin `InflectionalForm::Preterite`); under the
  corpus grammar's build-time derive the selected leaf keeps
  `{Plain, Preterite}`. Corrected above and routed to `docs/tickets/fog.md`.
- MEDIUM — only `Cast` and `Search` of the 67 verb declarations under
  `plugins/builtin_v2/macros/` received a `preterite:` surface, and the core
  inventory leaves `Have`, `Share`, `Unlock` and `Win` without one; the record
  disclosed only the ruled `Deal` withholding. Every unauthored lexeme's
  preterite is a past-participle homograph, so the omission belongs to the
  routed `dealt` family; disclosed and routed to `docs/tickets/fog.md` with its
  attested surfaces.
- Contention stamp for the performance advisory: the figures above were taken
  with 0 concurrent codex executors and 1 other Opus reviewer running, not the
  implementer's sandboxed visible-process counts.
- Guard scan: no `require`, `checked by`, Rust predicate or comment in the diff
  names a lexeme, construction, verb, noun, preposition or card identity.
- Newly covered analyses spot-checked at review across the Attack, Block, Cast,
  Control, Die, Draw, Enter, Gain, Leave, Lose, Return and Search buckets (18
  identities, including Archive Trap, Boomerang Basics, Cathedral Membrane,
  Crested Sunmare, Custodi Soulcaller, Dawn Evangel, Death's Presence, Doctor
  Jane Foster, Force of Despair, Ghired, Gideon's Triumph, Grasping Current,
  Living History, Mirrex, Power Surge, Search Party Captain, Sun-Blessed Mount,
  War Historian): every one is a finite preterite in a finite clause or a
  subject/object-gap relative. Aggravate and Ballista Watcher // Ballista
  Wielder keep their past-participle reduced-passive readings; Ashen-Skin
  Zubera and Boldwyr Heavyweights remain parse failures as ruling (1) expects.
