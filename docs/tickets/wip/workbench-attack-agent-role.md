---
needs: []
---
**Give the Attack deed its player agent.** Residue of
`workbench-deed-guard-residues` (2026-09-04): `Words.actFacts "Attack"`
keeps `agentRole` at `[Object] [Creature]` although the corpus survey
(`docs/memory/scratch/2026-09-04-deed-subjects.md`) finds 195 supported
cards printing "Whenever you attack" and [CR#508.1] makes the active player
the one who declares attackers. The role is load-bearing for
`featureAltOk Attacking` and `Phrase.attackableKind`, so widening it
touches the combat-relation gates. Fix: the agent role admits a player
subject as well as a creature; the creature-attacks reading keeps its
kind gate through the `Attacking` feature; bench one "whenever you attack"
card in full; re-probe the attack pins.

Size: S. Done when: "you attack" spells; every attack pin still refutes;
build at its module count. Standard constraints apply, including the
RON-shaped constraint.

## As landed

- **The agent role admits a player subject.** `Words.actFacts "Attack"`'s
  `agentRole` is now `MkDeedRole [Object, Player] [Creature] True (Just
  Battlefield)` — the kind list gained `Player` [CR#508.1]; the type list,
  the bare flag and the zone are untouched, so nothing about the creature
  reading moved.
- **The creature-attacks reading keeps its kind gate through the `Attacking`
  feature.** `Triggers.Attacks` is now kind-polymorphic in its subject
  (`{k : Kind} -> (n : Noun bs k)`) and its bare `ZoneIs (nounZone n)
  Battlefield` obligation is replaced by one `{auto 0 ak : Attacker n}`.
  `Phrase.Attacker` is `So (attackerOk n)` with
  `attackerOk n = featureKindOk Attacking Agent k && featureNounOk Attacking
  Agent n && combatPartyKind k (nounZone n)` — the declaring-side counterpart
  of `attackableKind`/`Attackable`. New `Events.featureKindOk` is the
  kind column of `featureAltOk`/`featureNounOk`; every gate stays keyed on the
  `Attacking` feature, so no deed lexeme literal enters a core gate
  (`workbench-deed-guard-residues`). `featureAltOk Attacking Patient` and
  `Phrase.attackableKind` are unchanged: the defender side reads the patient
  role, which this round does not touch. "Target land attacks each combat if
  able" (`ProofsDeontic.okMustAttackLand`) still passes only through
  `deedAnimatedOk` [CR#205.1b]; "you attack" passes as a player deed.
- **`Effect.Enact`'s `enactAgentOk` for the new row.** No code change:
  `enactAgentOk (Just _) v = deedKindOk v Agent Player` now answers `True` for
  `"Attack"`, so `Enact (Just who) "Attack" …` is admitted where it was
  refused. Same for `deedFits ["Attack"] Agent Player` (a deontic restriction
  or requirement on a player attacking). Both are rules-meaningful under
  [CR#508.1]; neither is spelled by a macro or a witness this round.
- **One "Whenever you attack" card benched in full.**
  `Cards.Trigger.bardHeirOfGirion` — Bard, Heir of Girion ({2}{W}{U},
  Legendary Creature — Human Archer, 4/4, Vintage-legal, `supported`): reach,
  vigilance, "Other creatures you control get +1/+1." and "Whenever you
  attack, draw a card." written as `Macros.triggered Whenever (Macros.attacks
  You) (Draw You (Lit 1))`. `Macros.attacks` and `Macros.attacksPlayer` carry
  the new obligation (`ak`); `attacksPlayer` keeps its object subject.
- **Every attack pin re-probed.** Twelve pins deliberately mis-stated once
  each, message watched, restored (list in the landing record). New pin
  `ProofsTrigger.badGraveyardAttacker` ("Whenever target creature card in your
  graveyard attacks") with its twin `okPlayerAttacksHeader`
  ("Whenever you attack") beside it.

Undone: nothing.

## Landing record

Measured on `@` = the working copy of this landing (feature workspace
`workbench-attack-agent-role`).

Numbers before → after:

- Idris modules built: 46/46 → 46/46 (clean `rm -rf build` rebuild).
- `actFacts` rows whose agent role admits `Player`: 54 → 55.
- `Unspellable` pins across `Proofs*.idr`: 635 → 636.
- Cards benched under `Cards/`: 814 → 815.
- `Attacks` term-level uses: 13 → 15 (nothing re-spelled; two added).

Gate lines:

- `cd idris && rm -rf build && ./scripts/build`: exit 0,
  `46/46: Building Cards (src/Cards.idr)`, 0 Error/Warning lines.
- `cargo xtask cite check --list-noncompliant`:
  `0 non-compliant citation-looking string(s)`.
- `cargo xtask cite check`:
  `checked 14446 citations against cr.txt (eff. 2026-08-07); 0 stale`.
- `cargo xtask cite bless`: `blessed 1434 rules at cr_date 2026-08-07`, two
  newly registered rules — [CR#508.3d] ("An ability that reads 'Whenever [a
  player] attacks, . . .' triggers if one or more creatures that player
  controls are declared as attackers") and [CR#508.3c] ("… attacks with [a
  creature] … triggers if a creature that player controls is declared as an
  attacker"), the latter cited only by this record. Nothing pruned.
- `jj --no-pager diff --git | cargo xtask cite audit --diff`:
  `audited 17 citation site(s)`; each read against its claim — in the tree,
  [CR#508.1] (the active player declares attackers) ×2 and [CR#508.1a] (the
  active player chooses which creatures they control will attack) ×1 in
  `Phrase.attackerOk`'s doc, [CR#508.1] ×1 and [CR#508.3d] ×1 on the twin;
  the rest are this record's own cites, including [CR#205.1b] (an object
  keeps its prior types) and [CR#508.3c].

Performance advisory: clean full Idris build 131s wall on this host; no
module approaches the ~5-minute stop, and no module's build time moved
noticeably against the pre-round baseline.

Assurance counts: restored 0; re-spelled 0; ignored 0; added 2
(`okPlayerAttacksHeader`, `badGraveyardAttacker`); removed 0.

Pin probes (each deliberately mis-stated once, message watched, restored —
all twelve reported `… is not a valid impossible case.`):
`badGraveyardAttacker`, `badPluralAttackDefender`,
`badMixedAttackDefenderHalves`, `badThatCreatureIsSelf`,
`badWhileDoingMoment` (ProofsTrigger); `badCreatureAttackDefender`
(ProofsZone); `badStaticTargets`, `badCantBeAttacked`,
`badForbidAttackWithPatient`, `badManaPremiseAtAttack` (ProofsDeontic);
`badFightPermanent` (ProofsDamage); `badAttackedByGraveyardRelatum`
(ProofsTurn). The `Attacking`-predicate pins
(`badNoncreatureAttackingOrBlocking`, `badWrappedStatusLaunder`,
`badPartialZoneJoin`, `badAttackingNoncreature`, `badAttackingInHand`,
`badExiledWithAttacking`, `badAttackerComplementOnObject`,
`badPluralAttackWindow`) read `seedType`/`seedZone`/`predContradicts`, which
this round does not touch, and were left as they stand.

Load-bearing probe: reverting `agentRole` to `MkDeedRole [Object] [Creature]
True (Just Battlefield)` turns the twin into `Error: While processing right
hand side of okPlayerAttacksHeader. Can't find an implementation for So
False.` — the widened kind list is what admits "you attack".

Deviations and additions:

- Added `Events.featureKindOk` and `Phrase.attackerOk`/`Attacker`. The ticket
  names only the role; a kind-aware gate keyed on the feature is what makes
  the widened row bite on the event without writing an `"Attack"` lexeme
  literal into `Triggers.idr`, which `workbench-deed-guard-residues`'
  Done-when forbids.
- `Triggers.Attacks` became kind-polymorphic (one row, not two). Justification:
  "Whenever you attack" [CR#508.3d] has no other spelling — `AttacksWith` is
  [CR#508.3c]'s "attacks with [a creature]", and writing Bard as
  `AttacksWith You NoDefender (one or more creatures)` would be a reference
  not as printed. The shape follows the settled combat-relation decision that
  a combat party is kind-polymorphic (`Phrase.combatPartyKind`, reused here
  for the object attacker's battlefield requirement).
- `Macros.attacks` is kind-polymorphic and both attack macros renamed their
  zone implicit `zn` to `ak`. No call site changed.
- Nothing deleted.

STOPs taken: none.
