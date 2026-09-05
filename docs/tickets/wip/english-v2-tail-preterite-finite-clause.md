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

Work started at 2026-09-05 06:25:44 PDT. Work stopped at
2026-09-05 10:28:36 PDT after a refreshed-tree negative oracle became newly
covered. Full-corpus measurement and lock blessing were not run after the
STOP.

### Coordinator rulings dated 2026-09-05

(1) ACCEPTANCE WITNESSES. The ticket's own letter contradicts itself: `that died this turn` (Ashen-Skin Zubera) and `who searched …` (Boldwyr Heavyweights) need relative-clause / temporal-duration hosts the ticket forbids changing, and the present-tense controls fail the same host invariant, so those two were never preterite failures. Strike them from acceptance; route both identities with their exact blocking surfaces (`that died this turn` — `MannerReference.this_way` invariant; `who searched` — stops at `who`) to docs/tickets/fog.md under the relative-clause family, naming english-v2-relative-clause as the owner. The remaining four witnesses stand.

(2) PLAIN/PRETERITE HOMOGRAPHS (`cast`, `put`, `cost`, …) follow the Number idiom from english-v2-number-feature-unification (homogeneous feature, one row, agreement resolves, never two candidates): a declared preterite form byte-identical to the lexeme's plain form is emitted as ONE scanner hit whose inflectional-form value is the SET {Plain, Preterite} with the union of the two concord-class applicabilities; a host that requires one inflectional form (a finite clause with a third-person-singular subject, a participle host, a modal complement) narrows it; where nothing narrows it the unit selects the single underspecified analysis. This is feature underspecification in construction_core's existing inflectional-form/concord machinery, declared once at the compiler level — never a per-lexeme switch, never a guard naming a verb, never a specificity weight, never a dominance edge. Prove it: the 3,367 subset ties go to zero; every `cast`/`put`/`cost` unit that was covered before is covered after with the same selected surface analysis (the per-unit diff shows only the inflectional-form annotation), and a third-person-singular-subject unit selects Preterite uniquely. If the compiler cannot express a set-valued inflectional form without a new feature domain, add the domain the way Number was added (sealed compiler feature, normalized rows carry it as data) and disclose it as a Deviation; if that turns out to need a construction shape you cannot place, STOP naming the shape.

(3) `Deal::Preterite` displacing the past-participle analyses of Aggravate and Ballista Watcher is a WRONG analysis, so withholding it was correct; keep it withheld, disclose the two identities with both analyses, and route "preterite/past-participle homograph host discrimination (`dealt`)" to fog.md with those identities as the frontier residue.

### PROVE

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

### DISCLOSE

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

### REPORT

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
- STOP — newly covered negative oracle and recorded-boundary contradiction:
  `general_event_relative_remains_an_exact_ordinary_parse_failure` expects
  `Destroy target creature that entered this turn.` to remain an ordinary
  parse failure, but this ticket's `Enter::Preterite` selects uniquely as
  `AbilityPlain -> SentenceImperative -> PredicateAdjunctPredicate ->
  TransitivePredicate -> SubjectRelativeQualifiedReference ->
  FiniteSubjectGapRelativeClause(Enter::Preterite) ->
  DurationPredicateAdjunct(this turn)`. The ticket simultaneously pins every
  existing finite host, explicitly including `finite_subject_gap_relative_clause`,
  to reach the preterite. Preserving the recorded negative requires authority
  to exempt or change that relative-clause/duration path; accepting the new
  analysis requires authority to retire the negative. Neither is authorized
  here. Decision wanted: rule which recorded boundary governs.
