---
needs: [workbench-turn-ref-read]
---
**Retype the turn-part possessor as `Maybe (Noun bs Player)` and delete the
fused pronoun enum `Owner`.** Cleanroom review 2026-09-03, F2. Rules-meaningful
printed headers are unspellable today because the fused word is the only
admitted spelling.

`Words.Owner` (`Words.idr:3829–3832`: `Yours | ThatPlayers | EachPlayers |
EachOpponents | EachYours | AnOpponents | ThatTurns | EachOthers`) is consumed
by `Triggers.HeaderPossessor` (`Triggers.idr:555–559`: `NoPossessor | ByWord
Owner | ByNoun n {PossessorNoun n}`), `Words.DurationEnd` (`Words.idr:3882`),
`Triggers.TriggerWindow`/`Timing` (`Triggers.idr:630–641`) and
`Effect.OnlyDuring` (`Effect.idr:439`), with `possessorB` (`Words.idr:3854`)
re-deriving the bindings a noun would have introduced. `PossessorNoun`
(`Triggers.idr:549–552`) admits exactly one noun, `AttachHost w PlayerW`.
Probes (each twin through `ByWord` typechecks):

```
BeginningOf Upkeep (ByNoun (the ChosenPlayer))          -- Black Vise, Energy Vortex
  Can't find an implementation for PossessorNoun (Described TheDet ChosenPlayer).
BeginningOf Upkeep (ByNoun (controllerOf (AttachHost Enchanted (TypeW Creature))))
  -- Apathy, Curse Artifact, Cursed Land, Erosion, Errant Minion
  Can't find an implementation for PossessorNoun (PossessorOf ControllerAx …).
BeginningOf Upkeep (ByNoun (a Opponent))                -- the compositional AnOpponents
  Can't find an implementation for PossessorNoun (Described (ADet Unmarked) Opponent).
```

## Fix

`HeaderPossessor bs = Maybe (Noun bs Player)`. `DurationEnd`,
`TriggerWindow`, `Timing.DuringPart` / `BeforePoint` and `OnlyDuring` take the
same noun. "That turn's" is a `Noun bs TurnRef` read through the `Pro` read
minted by `workbench-turn-ref-read`, replacing `TurnDeixis`
(`Triggers.idr:588`). `windowOk` / `pointWindowOk` (`Phrase.idr:3079–3095`)
and `durationPossessorOk` (`Words.idr:3866`) become `nounPlur` / `nounIsYou`
checks. Delete `Owner`, `possessorB`, `isTurnDeictic` and `PossessorNoun`.

This is ADR §4 as written: no fused pronoun forms; relational nouns compose
over any noun.

Bench migration: 82 sites spell an `Owner` word (59 through `ByWord`, the rest
in `DurationEnd` / `OnlyDuring` / `Timing`), plus 5 pins.

Size: L.

Done when: the build is 23/23 with 0 errors and 0 warnings; `Owner`,
`possessorB`, `isTurnDeictic` and `PossessorNoun` are gone; every possessor
slot takes a `Noun`; the three probe headers above (Black Vise, Apathy, "an
opponent's upkeep") are typechecking bench witnesses on their printed cards;
all 82 former `Owner` sites and the 5 pins are re-spelled and every one of
those pins still refutes for its named reason. Standard constraints apply.

## As landed

- `HeaderPossessor` is a three-arm sum, not the ticket's literal `Maybe (Noun
  bs Player)`: `NoPossessor | ByPlayer (Noun bs Player) | ByTurn (Noun bs
  TurnRef)`. The turn read needs a second kind, and the two alternatives were
  rejected — two `Maybe` slots admit a both-filled state that means nothing,
  and a kind-indexed slot is a kind-polymorphic constructor, which the
  workbench bans. `ByPlayer` carries no gate at all: every player noun is an
  admissible possessor, which is the whole point of F2.
- `Words.Owner` (8 members), its `Eq`, `possessorB`, `Triggers.PossessorNoun`,
  `possessorWord`, `TurnDeixis` and `isTurnDeictic` are gone from the tree;
  `BeginningOf` lost its `td` argument (the `Pro ThatTurn OneOf` read carries
  its own `= 1` gate, so the deixis wrapper had nothing left to say).
- `DurationEnd`, `DurationPossessor` and `durationPossessorOk` moved from
  `Words` to `Phrase` (they now read `nounPlur`/`nounDet`) and gained a
  `Bindings` index; `Until : DurationEnd bs -> Duration bs`. `TriggerWindow`
  and `Timing` gained the same index. `Effect.OnlyDuring`,
  `Timing.DuringPart`/`BeforePoint` and `TriggerWindow.DuringWindow` all take
  `Maybe (Noun bs Player)`.
- `possessorIntro (ByPlayer n) = selfSubjDelta n ++ Phrase.agentIntro n`
  reproduces the old `possessorB` table exactly, one determiner at a time:
  `You` and `That PlayerW` introduce nothing, `each player`/`each opponent`
  introduce one singular player through `agentIntro`'s `EachDet` clause (the
  binding's determiner is `TheD` where `possessorB` wrote `EachD`; no read
  looks at `det` except `wordNow`'s `SelfD` exclusion), `a opponent`
  introduces `AD Player OneOf`, and `AttachHost Enchanted PlayerW` /
  `controllerOf …` introduce theirs through `selfSubjDelta`/`nounDelta`.
- New gate `Phrase.partPossessorOk` ([CR#102.1]): a turn part has one active
  player, so its possessor is singular or distributes (`EachDet`). It backs
  `windowOk`, `pointWindowOk` and the new conjunct in `PartTriggerable`;
  `windowOk Turn Nothing = False` survives unchanged. `pointWindowOk` would
  otherwise have gone vacuous once the turn possessor left the slot.
- `durationPossessorOk` widened from the three-value whitelist to "singular
  and the determiner is absent or `TheD`", so "until the chosen player's next
  upkeep" is now spellable while `an opponent's`/`each player's` stay refused.
- Macros: `yours`, `eachPlayers`, `eachOpponents`, `thatTurns` are thin
  `HeaderPossessor` wrappers preserving the bench surface;
  `beginningOfPossessed` lost its `PossessorNoun` gate; `untilYourNext*` and
  `doesntUntap`/`mayDeclineUntap`/`untapsDuring` take the noun.
- All 82 `Owner` bench sites re-spelled (59 `ByWord` → the four macros, 23
  `Just <word>` → `Just You` / `Just Macros.anOpponent` /
  `Just (Macros.each Opponent)` / `Just (Macros.each Macros.otherPlayer)`).
- Witnesses: `Cards.blackVise` ("At the beginning of the chosen player's
  upkeep…") and `Cards.apathy` ("…the upkeep of enchanted creature's
  controller…") are new printed-card bench entries — the two headers probes
  B1/C1 refused. `Cards.festival` is the compositional "an opponent's upkeep"
  (probe D1's shape, `Macros.a Opponent`) on its printed card; no printed card
  spells that possessor in a `BeginningOf` header. `finalFortune`,
  `lastChance` and `chanceForGlory` keep typechecking through
  `Macros.thatTurns`, and `finalFortuneThatTurn` is unchanged.
- Pins: `badPluralPartWindow` and `badPluralAttackWindow` (were
  `badThatTurns*`), `badDurationEndAnOpponent` (was `badDurationEndThatTurns`)
  and `badDurationEndEachPlayers` re-spelled; `badDeicticTurnWithoutIntroducer`
  re-spelled onto `Macros.thatTurns`' count gate; new
  `ProofsG.badPluralPartPossessor` refuses a plural possessor on the header
  itself. `turnInScopeReadsOnlyPrefix`/`turnInScopeResolvesInPrefix` re-spelled
  from `TurnDeixis` onto `countReach ThatTurn OneOf`.
- Undone: nothing in the ticket's letter. The three `Timing`/`TriggerWindow`
  claims about "that turn's" are now structural (the slot's kind admits no
  turn noun) rather than gate refusals, so those two pins carry the surviving
  claim instead.

## Landing record

- Construction count: `Words.Owner` 8 → 0 (enum deleted);
  `Triggers.PossessorNoun` 1 → 0; `Triggers.TurnDeixis` 2 → 0;
  `HeaderPossessor` 3 → 3 (`ByWord`/`ByNoun` → `ByPlayer`/`ByTurn`);
  `DurationEnd` 2 → 2 (moved to `Phrase`, `Bindings`-indexed); `Phrase` +2
  predicates (`partPossessorOk`, and `headerPossessorOk` in `Triggers`);
  `Macros` +4; `Cards` +2 printed cards.
- Bench: 82 `Owner` sites re-spelled (59 `ByWord`, 23 `Just <word>`), 0
  witnesses removed.
- Coverage and lock state: `cr-citations.lock` unchanged — `[CR#102.1]` was
  already registered, so no `bless` was needed.
- Gates (all foreground): clean `rm -rf build && ./scripts/build`, last line
  `23/23: Building Cards (src/Cards.idr)`, 0 `Error` and 0 `Warning` lines;
  `cargo xtask cite check --list-noncompliant` → `0 non-compliant
  citation-looking string(s)`; `cargo xtask cite check` → `checked 17728
  citations against cr.txt (eff. 2026-08-07); 0 stale`; `cargo xtask cite
  audit --diff` → `audited 4 citation site(s) — read each rule text against
  its claim`, all four `[CR#102.1]`, read against their claims.
- Pin probes (scratch module, deleted afterwards): all six positive twins
  typecheck as terms — `DuringPart EndStep (Just (Macros.each AnyPlayer))`,
  `BeforePoint AttackersDeclared (Just (Macros.each AnyPlayer))`,
  `StartOf Upkeep (Just You)`, `EndOf Combat Nothing`,
  `BeginningOf Upkeep (ByPlayer (Macros.each AnyPlayer))`, and the delayed
  turn header under an `ExtraTurn`. Re-stated as pins, each is refused
  `… Oh is not a valid impossible case` (`Refl` for the turn one), so all six
  are non-vacuous.
- Assurance: restored 0; re-spelled 12 (5 pins, 2 anaphora prefix lemmas, 5
  mechanical `Owner`-word bodies in `ProofsC`/`Proofs`); ignored 0; added 3
  (`badPluralPartPossessor`, `blackVise`, `apathy`); removed 1.
- Removed, by name: `ProofsG.badNounPossessorYou` ("At the beginning of you's
  upkeep") asserted that `You` is not an admissible possessor noun — the exact
  restriction F2 exists to retire. Its slot in `ProofsG` is taken by
  `badPluralPartPossessor`, and `ByPlayer You` is now a 47-site bench witness.
- Deviations and additions:
  - `Owner.EachYours` ("each of your postcombat main phases": Sphinx of the
    Second Sun, Brazen Cannonade) collapses to `Macros.yours`. The model never
    distinguished `Yours` from `EachYours` outside `durationPossessorOk`, and
    neither card uses a duration end; the part-distributive "each of your X"
    has no possessor-slot spelling because the `each` quantifies the turn part,
    not the possessor. Residue, not covered by this ticket's letter.
  - Black Vise's second and third mentions spell "that player" as
    `Macros.the ChosenPlayer`. `choiceB PlayerC` and the header possessor's own
    binding are two bindings for one referent, so `That PlayerW` counts 2 and
    is refused as ambiguous. This is `Described TheDet` over a choice, not the
    possessor slot — any noun phrase over a choice binding does it — so it is
    recorded as residue rather than worked around here.
  - `durationPossessorOk`'s widening (above) admits "the chosen player's" where
    the old whitelist did not; no bench site depends on it.
  - `TriggerWindow`/`Timing`/`DurationEnd` gained a `Bindings` index, which the
    ticket implies but does not name.
- STOPs: none.
