---
needs: []
---
**Gate `Effect.Enact`'s agent slot on the facts table's agent role.**
Cleanroom review 3, 2026-09-04, finding U3.

- `Effect.Enact` takes `subj : Maybe (Noun bs Player)` with no voice check,
  while `Triggers.VerbedEvent` gates on `So (verbedVoiceOk v who what)`. So
  `Enact (Just You) "Destroy" (Move (target creature) graveyardZ [])` is
  admitted although `Words.actFacts "Destroy"` gives Destroy no agent role.
  Add `{auto 0 ag : So (enactAgentOk subj v)}`, with `enactAgentOk Nothing v =
  True` and `enactAgentOk (Just _) v = elem Player (roleKinds (agentRole
  facts))`.
- The macros then disagree with the table in the other direction:
  `Macros.exile` enacts `Just agent` over `"Exile"`, whose row is `noRole`, so
  13 bench sites spell "Exile target creature" with an agent the card never
  prints, while `Macros.destroy` is agentless. Make `exile` agentless and keep
  a separate `exiles agent n` for the "you exile"/"that player exiles"
  sentences that do print; re-spell each of the 13 by what its card prints.
- Pin an agented enact of an agentless verb, with its positive twin in the
  same module.

Size: S. Done when: `enactAgentOk` gates every `Enact`; no witness carries an
agent its card does not print; the pin probes non-vacuous; build at its module
count. Standard constraints apply, including the RON-shaped constraint.

## As landed

- `enactAgentOk` gates every `Enact`: `Effect.Enact` gained
  `{auto 0 ag : So (enactAgentOk subj v)}`, with `enactAgentOk Nothing v = True`
  and `enactAgentOk (Just _) v = deedKindOk v Agent Player` (the existing
  `Events.deedKindOk` spells the ticket's `elem Player (roleKinds (agentRole
  facts))`), written beside `EnactKeepsOuter` and mirroring
  `Triggers.verbedVoiceOk`. `Macros.scry`/`fateseal`/`surveil` pass `{ag = Oh}`
  explicitly because auto-search for `So True` is ambiguous under their local
  `So`-typed handles.
- `Macros.exile` is agentless (`Enact Nothing "Exile"`) and `Macros.exiles agent
  n` carries the printed subject. All 106 call sites re-spelled: 96 agentless,
  10 agented — the seven "You may exile … rather than pay this spell's mana
  cost" alternative costs (Shining/Disrupting/Blazing/Sickening/Nourishing
  Shoal, Sunscour, Force of Will), Thought Lash's "that player exiles all cards
  from their library", and the two `ProofsZone` "each opponent exiles" witnesses.
  Every other site is a bare-imperative "Exile …", including bodies under a
  `May`/`AltCost`/replacement that already names the actor.
- Pin `ProofsZone.badAgentedAgentlessAct` ("You transform target creature")
  with its twin `okAgentedKnownAct` ("You exile a card from your hand")
  immediately above it, both `Instruction []`.
- Eight `actFacts` rows gained `agentRole := MkDeedRole [Player] [] True
  Nothing` — the rows the workbench's own agented macros enact, each with a
  printed or CR subject: Exile (123 printed "target/each player exiles"),
  Discard (456), Mill (236, [CR#701.17a] "for a player to mill"), Put (81),
  Shuffle (108), Scry (3), Surveil ([CR#701.25c]), Fateseal ([CR#701.29a]).
- Not done: the ticket's suggested pin subject "you destroy target creature".
  See the STOP below.

## Landing record

Change id: see the commit for this ticket (single commit, `idris:` prefixed).

Numbers before → after:

- Idris modules: 46/46 → 46/46 (clean build, no `Error` and no `Warning` lines).
- `Unspellable` pins: 609 → 610.
- `actFacts` rows whose agent role admits `Player`: 12 → 20.
- `Macros.exile` call sites: 106 agented → 96 agentless + 10 `Macros.exiles`.

Gate lines:

- `cd idris && rm -rf build && ./scripts/build` → `46/46: Building Cards
  (src/Cards.idr)`, exit 0, no `Error`/`Warning` line.
- `cargo xtask cite check --list-noncompliant` → `0 non-compliant
  citation-looking string(s)`.
- `cargo xtask cite check` → `checked 14377 citations against cr.txt
  (eff. 2026-08-07); 0 stale`.
- `jj --no-pager diff --git | cargo xtask cite audit --diff` → `audited 0
  citation site(s) — nothing selected` (this round adds no CR citation; no
  `cite bless` was needed and `cr-citations.lock` is untouched).

Performance advisory: clean full Idris build 1m17.8s wall (1m13.2s user) — the
same shape as before the round; no module approaches the ~5-minute stop.

Assurance counts: restored 0; re-spelled 106 (every `Macros.exile` site, by the
voice its card prints — 96 to the agentless macro, 10 to `Macros.exiles`);
ignored 0; added 2 (`okAgentedKnownAct`, `badAgentedAgentlessAct`); removed 0.
Pin non-vacuity probe: replacing the pin's `"Transform"` with `"Exile"` (a row
that does admit a player agent) turns `badAgentedAgentlessAct Oh impossible`
into `Error: badAgentedAgentlessAct Oh is not a valid impossible case.`; the
pin was restored and the tree rebuilt green.

STOP — the ticket/brief names "you destroy target creature" as the pin, on the
premise that Destroy takes no agent. Burning of Xinye (Portal Three Kingdoms,
Vintage-legal, `supported` in `data/derived/cards.jsonl`) prints "You destroy
four lands you control, then target opponent destroys four lands they control."
Pinning that spelling as unspellable would refuse a printed card, which the
workbench forbids. Resolution: the pin keeps the ticket's shape — an agented
enact of a verb the facts table gives no player agent — over `"Transform"`,
which no printed card gives a subject (0 corpus hits) and whose row is already
`actIntransitive`. Destroy's row was left unchanged, so `Enact (Just You)
"Destroy" …` is now refused; that refusal is unpinned and unasserted.

Deviations and additions:

- Added `Macros.exiles` (the ticket's split) and the two `ProofsZone`
  definitions above. Nothing deleted.
- Added the eight `actFacts` agent roles. Only Exile is forced by the ticket's
  `exiles`; the other seven are forced by the new gate meeting the already-agented
  `discard`, `mills`, `puts`, `shuffleInto`, `scry`, `surveil`, `fateseal`
  macros, which the ticket does not ask to change. Each row is backed by printed
  card text or the keyword action's own CR definition (listed above).
- `{ag = Oh}` added at three `Macros` sites to disambiguate auto-search; no
  behaviour change.

Ledger (route at integrate): the `actFacts` agent column stays incomplete, and
the new gate now bites on it. Verbs whose rows give no player agent although the
corpus prints one: Destroy (Burning of Xinye), Tap (Tangle Wire, "that player
taps an untapped artifact, creature, or land"), Return (11 "each player
returns"), GainControl (52 "… player gains control"), Reveal, and others in the
`plainAct` tail. Filling that column verb-by-verb, with a witness for each, is
follow-up work this ticket does not cover.
