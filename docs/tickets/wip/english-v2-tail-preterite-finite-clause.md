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

## Landing record — STOP (2026-09-05)

Measured on working change `qrpltxtt`, refreshed onto claim change
`xyysysuz`. Work started at 2026-09-05 06:25:44 PDT and the STOP was
confirmed at 2026-09-05 07:33:40 PDT, after 4,076 s.

### PROVE

- The safe partial implementation adds an explicitly authored `preterite`
  surface to the declaration grammar, indexes core and plugin verb lexemes by
  that Inflectional Form, and lets every existing Concord Class-aware
  declaration-verb terminal scan it for both Concord Classes. The selected
  Inflectional Form is private compiler/runtime realization metadata; no
  public AST form tag, host-side construction, word guard, `require`,
  `checked by`, dominance edge, or exception was added. Rendering and lexical
  provenance consume that same selected form.
- The ticket subset was built with its complete surface expression
  (`\b\w+ed\b` plus `died|left|lost|put|cast|dealt|drew|spent|made|won`), the
  568 cards whose parent selection contained `reduced_relative_modifier` or
  `participial_by_complement`, and the named witnesses. It contains 14,468
  cards and 13,664 corpus units.
- Claim subset -> safe partial subset: selected and covered 6,681 -> 6,793;
  parse failures 6,983 -> 6,871; unresolved ties 0 -> 0; internal failures,
  ownership failures, round-trip mismatches, traversal failures, gap spans,
  overlap spans, synthetic claims, and provenance-plan mismatches remain 0.
  The per-unit ambiguity diff is +112 selected, -0 selected, with all 6,681
  surviving selections retaining the same selected construction path. The new
  selections are 40 unique and 72 specificity-resolved.
- Focused positive artifacts completed before the STOP:
  `declared_preterites_parse_and_render_through_both_concord_classes` passed
  (`test result: ok. 1 passed; 0 failed`),
  `declared_preterites_reach_general_finite_clause_hosts` passed
  (`test result: ok. 1 passed; 0 failed`), and the builtin-v2 declaration-row
  assertion passed (`test result: ok. 1 passed; 0 failed`). The affected-subset
  ambiguity command with `--require-resolved` exited 0. The affected-subset
  round trip reported `parse accepted 6793`, `clean 6793`, `mismatched 0`.
- Present controls and the past-participle rivalry were compared unit by unit.
  No prior selected path moves in the safe partial tree. In particular,
  Aggravate and Ballista Watcher // Ballista Wielder retain their prior
  reduced-passive `dealt damage this way` analyses; the unsafe finite
  `Deal::Preterite` declaration that moved those selections was not retained.

### DISCLOSE

The 112 provisional subset gains are named below by their selected analysis.
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

The claim-time full census was 19,469 selected and covered of 32,641, with
13,172 parse failures, 15,271 unique selections, 4,198
specificity-resolved selections, 0 unresolved ties, and 0 internal failures.
The ticket's historical 929-fragment bucket remeasured as 741 fragments at the
claim tree under the same first-failure byte test. Because this round STOPped
before an admissible full measured tree existed, no complete preterite versus
past-participle split is asserted. The safe subset establishes a lower bound of
112 true preterites that become selected; presenting that lower bound as the
requested split would be false precision.

### REPORT

- Construction declarations: 394 on the refreshed tree; this change adds or
  removes 0. Licensing checkers remain 21 permitted and 0 forbidden. The
  licensed vocabulary/lexicon homograph inventory remains 2 and the form
  literal/vocabulary overlap inventory remains 9. Selection exceptions remain
  0. No inventory was fitted to a count.
- Claim full coverage performance: 8 workers, 128 s, 151,125 ns/B, host load
  18/17/16. Final safe-subset coverage: 8 workers, 62 s, 166,228 ns/B, host
  load 25/22/19. Final safe-subset ambiguity: 8 workers, 72 s, 235,046 ns/B,
  host load 28/25/20. Final safe-subset round trip: 8 workers, 46 s,
  144,603 ns/B, host load 20/23/20. The sandbox-visible process count was 4;
  sibling contention is not visible. All ceiling exceedances occurred under
  load and are advisory.
- Assurance census: restored 0; re-spelled 0; extended 2 existing test
  functions (declaration normalization and builtin-v2 homographic form rows);
  ignored with blockers 2 named full-card witnesses; added 2 test functions
  (general finite-host realization and synthetic declaration scan/render);
  removed 0.
- Coverage lock: not blessed. No `coverage --check` or `--bless`, full
  ambiguity, full round trip, strict clippy, changed-closure gate, or cite gate
  was run after refresh because the acceptance contradiction is known. No red
  or unrun full gate is reported as done.
- Deviation: the claim-time full ambiguity report ran twice. The first
  redirected long-running process returned an obscured session and its output
  file was inspected before completion; a second foreground report was then
  run to establish the schema. This did not alter repository state but exceeded
  the requested single claim snapshot.
- Deviations and additions: production changes are limited to the declaration
  grammar, generated declaration-verb realization path, core preterite data,
  and the Search preterite row. No construction was added or changed. `Have`,
  `Cast`, `Cost`, `Put`, and `Deal` preterites are deliberately absent from the
  safe partial tree for the STOP reasons below, not silently treated as
  negative lexical evidence.
- glossary gap: none. The existing Inflectional Form, Finiteness, and Concord
  Class entries suffice.

### STOP and decision wanted

1. The ticket says Ashen-Skin Zubera's present control, `that dies this turn`,
   is already reachable and also requires all six witnesses to select, while
   forbidding host-side changes. On the refreshed claim tree both that present
   control and `that died this turn` fail the same
   `MannerReference.this_way` invariant. Boldwyr Heavyweights likewise stops at
   `who` before either `searched` or the present control `searches`. Thus the
   two required full-card selections need duration/relative-host work while the
   ticket forbids such work. This is a contradiction in the ticket's operative
   acceptance letter, so the requested STOP protocol applies.
2. An intermediate declaration of the genuinely plain/preterite-homographic
   forms `Cast`, `Put`, and `Cost` produced 3,367 unresolved subset units. The
   two analyses are the declared `InflectionalForm::Plain` finite reading and
   the declared `InflectionalForm::Preterite` finite reading through the same
   Concord Class-aware terminal. No precedence can truthfully choose between
   readings such as `you cast`; those declarations were removed from the safe
   partial tree.
3. An intermediate `Deal::Preterite` did not tie, but it changed Aggravate and
   Ballista Watcher // Ballista Wielder from their established
   past-participle reduced-passive analysis to a finite-preterite analysis.
   That is a wrong movement of the ticket's negative control, so the
   declaration was removed rather than hidden with a dominance edge.
4. Decision wanted: amend the ticket either to depend on explicit general
   duration/relative-host work (and restate the two currently false controls),
   or reduce acceptance to the finite-clause witnesses that reach the verb.
   Separately rule whether genuine plain/preterite homographs remain
   intentionally unavailable or require a new grammatical dimension capable
   of retaining both analyses. Until then this ticket cannot be landed.
