---
needs: []
---
**Close the four small residues of dedup-tables and gate-fixes.** Residue of
`workbench-dedup-tables` and `workbench-gate-fixes` (2026-09-04):

- `Effect.instrProfile (ControllerSacrifices …)` still names the lexeme
  `"Sacrifice"` in its stamp; read the deed's declared feature instead (the
  `DeedFeature` column dedup added).
- `Triggers.eventName (UnlocksDoor …)` constructs a deed label by string;
  route it through the deed table.
- `Macros.fateseal`'s library-possessor slot is unconstrained (`fateseal You
  You` is admitted); gate it so the possessor is an opponent of the agent
  [CR#701.29a], pin the self case.
- `Words.actFacts "Fateseal"` keeps `actStepwise = False` although its
  expansion is scry-shaped; set the column from the expansion and check the
  other look actions agree.
- `Words.actFacts`'s agent column is incomplete now that `Effect.Enact`
  gates on it (residue of `workbench-enact-agent-role`, 2026-09-04): eight
  rows carry an agent role; Destroy (Burning of Xinye prints "You destroy
  four lands"), Tap (Tangle Wire), Return, GainControl, Reveal and the rest
  of the printed-subject deeds do not. Author each row's `agentRole` from
  the corpus and the CR entry; pin one deed that no printed card ever gives
  a subject.

Size: S. Done when: no deed lexeme literal remains in a core gate or stamp
(`grep '"Sacrifice"\|"Search"\|"Attack"\|"Block"\|"Trigger"\|"Activate"'`
over `Effect.idr`/`Triggers.idr`/`Words.idr` finds only the deed table);
the fateseal pin refutes; build at its module count. Standard constraints
apply, including the RON-shaped constraint.

## Rulings 2026-09-04 (wayfinder session)

The four guard bullets are mechanical. The `actFacts` agent column is
authored from `research-deed-subjects` (which deeds print a subject in the
supported corpus, with the CR entry for each); pin the deed that no printed
card ever gives a subject.

## As landed

- `Effect.instrProfile (ControllerSacrifices …)`: the stamp is now
  `moveIntro (Just (deedLabel Sacrificing)) …`. New `DeedFeature` value
  `Sacrificing` on the `"Sacrifice"` row; `Words.deedLabel` is a
  proof-carrying table lookup (`{auto 0 ok : So (isJust (featureLabel f))}`)
  so a dropped column entry breaks the build instead of silently degrading
  the stamp to `Nothing`.
- `Triggers.eventName (UnlocksDoor …)`: now `VerbedAct (deedLabel Unlocking)`.
  New `DeedFeature` value `Unlocking` on the `"Unlock"` row (the half-unlock
  deed of [CR#709.5f], not `"Fully Unlock"`).
- `Macros.fateseal`: new obligation `{auto 0 op : FatesealPossessor whose}`
  over `fatesealPossessorOk` (an opponent-headed description, `PlayerGroup
  YourOpponents`, or `EachOf` of one). Pin `badFatesealYourOwnLibrary`
  (`ProofsKeyword`) with twin `okFatesealAnOpponent`.
- `Words.actFacts "Fateseal"`: `actStepwise := True`, set from the
  `lookAndSortOf` expansion. The other look actions agree — `Scry` and
  `Surveil` are the only other `lookAndSort` macros and both already carried
  it; no other row has a look-and-sort expansion.
- `Words.actFacts` agent column: authored from
  `docs/memory/scratch/2026-09-04-deed-subjects.md`. Every row the survey
  shows with a printed player subject and no agent role gained
  `agentRole := MkDeedRole [Player] [] True Nothing` (34 rows: Destroy, Tap,
  Return, GainControl, Proliferate, Transform, Convert, Unlock, Fully Unlock,
  Airbend, Amass, Attach, Behold, Blight, Clash, Cloak, Collect Evidence,
  Create, Discover, Earthbend, Exchange, Exert, Face A Villainous Choice,
  Forage, Goad, Incubate, Investigate, Manifest, Manifest Dread, Planeswalk,
  Recruit, Reveal, Suspect, Waterbend). Rows the survey shows with no printed
  subject were left as they stand. Pin: `badAgentedAgentlessAct`
  (`ProofsZone`) re-spelled onto `"Fight"` — no supported card gives Fight a
  player subject and [CR#701.14a] instructs a *creature* to fight — with the
  new positive twin `okAgentedDestroy` beside it.

Undone: nothing.

## Landing record

Measured on `@` = the working copy of this landing (parent `rpyklkry`
`workbench-deed-guard-residues`).

Numbers before → after:

- Idris modules built: 46/46 → 46/46 (clean `rm -rf build` rebuild).
- Deed-lexeme literals in a core gate or stamp
  (`grep '"Sacrifice"\|"Search"\|"Attack"\|"Block"\|"Trigger"\|"Activate"'`
  over `Effect.idr`/`Triggers.idr`/`Words.idr`, plus `"Unlock"`): 3 → 0
  outside the deed table; the grep now matches only `plainAct` rows in
  `Words.actFacts`.
- `DeedFeature` values: 5 → 7.
- `actFacts` rows carrying a player agent role: 20 → 54 (of 88).
- `actFacts` rows with `actStepwise := True`: 2 → 3.
- `Unspellable` pins across `Proofs*.idr`: 633 → 634.

Gate lines:

- `cd idris && ./scripts/build` (after `rm -rf build`): exit 0,
  `46/46: Building Cards (src/Cards.idr)`, 0 Error/Warning lines.
- `cargo xtask cite check --list-noncompliant`:
  `0 non-compliant citation-looking string(s)`.
- `cargo xtask cite check`:
  `checked 14421 citations against cr.txt (eff. 2026-08-07); 0 stale`.
- `jj --no-pager diff --git | cargo xtask cite audit --diff`:
  `audited 14 citation site(s)`; each read against its claim — in the tree,
  [CR#701.29a] (fateseal names an opponent's library) ×3, [CR#701.22a]
  (scry names your own) ×2, [CR#701.14a] (a spell or ability instructs a
  creature to fight) ×1; the rest are this record's own cites, including
  [CR#709.5f] (a player unlocks half a permanent) and [CR#508.1] (the active
  player declares attackers). No `cite bless` needed: every rule was already
  in `cr-citations.lock`.

Assurance counts: restored 0; re-spelled 1 (`badAgentedAgentlessAct`, moved
from `"Transform"` to `"Fight"` — Transform's row now carries a player agent
role because printed cards give it one, so the same asserted outcome needed a
deed that still has none); ignored 0; added 3 (`okFatesealAnOpponent`,
`badFatesealYourOwnLibrary`, `okAgentedDestroy`); removed 0.

Pin probes (each deliberately mis-stated once, message watched):

- `badFatesealYourOwnLibrary`: adding `fatesealPossessorOk You = True` →
  `badFatesealYourOwnLibrary Oh is not a valid impossible case.`
- `badAgentedAgentlessAct`: giving `"Fight"` a player agent role →
  `badAgentedAgentlessAct Oh is not a valid impossible case.`
- `deedLabel Unlocking`: dropping `actFeature := Just Unlocking` →
  `Triggers.idr:357 … Can't find an implementation for So False.`
- `deedLabel Sacrificing` / `deedLabel LibrarySearch`: dropping either column
  entry → `Effect.idr:1854 … Can't find an implementation for So False.`

Deviations and additions:

- `Effect.instrProfile (Search …)`'s `mkStamp (Just "Search")` was also routed
  through the table (`deedLabel LibrarySearch`). Not one of the four bullets,
  but the ticket's Done-when grep names `"Search"`, and the `LibrarySearch`
  column entry already existed.
- Added `Words.labelIn` and `Words.deedLabel` (a proof-carrying feature →
  label lookup) rather than reusing the existing `Maybe`-slot idiom
  (`stampIntro (featureLabel ControlGrant)`). Justification: a probe showed
  the `Maybe` form silently degrades to `Nothing` when a column entry is
  dropped, so the stamp would lose its verb with a green build.
- Added `Macros.opponentPred`/`anyOpponentPred`/`allOpponentPred`,
  `fatesealPossessorOk` and the `FatesealPossessor` synonym (the
  `SlicePossessor` shape at the same slot) for the fateseal gate.
- Added `okAgentedDestroy` as the twin for the re-spelled agentless-act pin,
  so the pin's non-vacuity is re-checked against a row this round opened.
- Not changed, reported instead: `"Attack"`'s agent role stays
  `MkDeedRole [Object] [Creature] …` although the survey finds 195 supported
  cards printing "Whenever you attack" and [CR#508.1] makes the active player
  the declarer. The brief assigns combat relations to a sibling round, and the
  role is load-bearing for `featureAltOk Attacking` / `attackableKind`; this
  wants its own ticket.

STOPs taken: none. `docs/memory/scratch/2026-09-04-deed-subjects.md` was
absent at the start of the round, so the four mechanical bullets ran first per
the brief; the file existed when the agent column was reached, so no STOP was
needed.
