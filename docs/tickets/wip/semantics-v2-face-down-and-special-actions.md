---
needs: [semantics-v2-temporal-suite]
---
**One face-down spec, and special actions written as a cost plus an
instruction.** Ruling 2026-10-08 ("Face-down is one spec",
`docs/decisions/semantics-v2.md` §7). It needs the temporal suite's `Timing`
and `Duration`. Standard constraints apply.

## Shape

- `FaceDown (characteristics : Option Characteristics)`.
  - `none` is a card exiled face down: no characteristics and no status
    [CR#406.3a,110.5d].
  - `some c` is a face-down spell or permanent. `c` holds its copiable values
    (layer 1b) [CR#708.2,613.2b].
- Carriers:
  - `Instruction.turnFaceDown (subject) (spec : FaceDown)`. Manifest is
    "turn it face down; … put that card onto the battlefield face down"
    [CR#701.40a]. Later mentions of the card are anaphoric (moved-referent
    ruling, 2026-10-08).
  - The move rider `faceDown (spec : FaceDown)`, used for exiling face down
    and for entering face down. It replaces `TokenRider.faceDown`. Every
    move's `riders` list already takes a `TokenRider`; renaming that type is
    the landing's call.
  - `PlayPayment.faceDown (spec) (cost)` for morph and disguise
    [CR#702.37a,702.168a].
- Turning face up is `setStatus(faceUp, …)`. The values revert [CR#708.8].
- Special action: `StaticSpec.specialAction (when : Option Timing) (cost :
  Cost) (does : Instruction)`, meaning "any time you have priority, within
  `when`, you may pay this and do this" [CR#116.1,116.2c].
- Look rights (foretold card, hideaway [CR#702.75a], face-down permanents
  [CR#708.5]) use the existing `StaticSpec.visibility`.
- Checker: references and kinds only. Nothing face down is a token.

## Retires

`TokenRider.faceDown`; the helper
`plugins_v2/builtin/macros/instructions/putOntoBattlefieldFaceDown.ron`;
`CostNamed.ofSpecialAction` (a named cost reads `ofKeyword(label)`); the
`SpecialAction` enum (`turnFaceUp | putCompanionIntoHand | foretell |
unlockDoor`), each member becoming a `specialAction` row or a plain
instruction, its new home named in the landing record.

## Re-spells (same asserted outcome)

- Bodies: manifest, cloak, manifestDread.
- Canon: Soul Summons, Ransom Note, Threats Around Every Corner.
- Pins: `okManifest`, `okCloak` and `okManifestDread` in
  `lean/Semantics/Proofs/KeywordActionResidues.lean`, the face-down cases in
  `Faces.lean`, and every pin naming `SpecialAction` or `ofSpecialAction`.

## Bodies

- morph: `PlayPayment.faceDown` for {3}, and
  `specialAction(none, <morph cost>, setStatus(faceUp, this))`
  [CR#702.37a,116.2b].
- megamorph (undeclared today; declare it) [CR#702.37b], disguise
  [CR#702.168a] and hideaway [CR#702.75a].
- foretell: `when = inPart(turn, you, current)`, cost {2}, `does =
  sequentially([exile(this, faceDown(none)), establish(mayCastFrom(it,
  exileZone, payingInstead(Param(0)), between(endOf(turn, current),
  none)))])` [CR#702.143a,116.2h].
- plot: its cost, exile, then the free cast at sorcery speed on a later turn
  [CR#702.170a,702.170d,116.2k].
- suspend [CR#702.62a], if the temporal suite can write it. Otherwise STOP
  and name the gap.

## Witnesses

Soul Summons (manifest); a morph creature turned face up (values revert); a
foretold card cast on a later turn but not the turn it was foretold; a
plotted card. Pins: `exile(…, faceDown(none))` checks, and a `specialAction`
whose `does` reads an unbound anaphor is refused.

## Deltas

Glossary Face-Down and Special Action are already rewritten in
`docs/contexts/game-model/CONTEXT.md`. Amend them if the build shows a flaw.
Engine-side work stays in `engine-face-down` and `engine-special-actions`.

## Landing record

Series on the claim `yyrwpoyv`, oldest first: `plynpmwr` (Lean nodes, checker,
bench re-spelled, pins), `nwwlspuy` (Rust mirror, RON helpers, keyword bodies,
facts), `sntzpsxz` (canon witnesses), `kszwszzx` (glossary, §7 rider ruling,
contracts, citation lock), and this record. Numbers measured on `kszwszzx`
after `kata refresh` (onto `english-v3-counter-kind-nouns`, no conflicts).
`lean/` changed.

1. **Census delta**: bounded census, selector `--text-contains Megamorph`
   (the only english-v3 input that changed is the new `megamorph` keyword
   declaration), before = `kszwszzx` with `megamorph.ron` withheld, after =
   `kszwszzx`: 31 faces, covered 0 → 11, Readings 0 → 12. Lost faces: none.
   JSONs in `target/english-v3/semantics-v2-face-down-and-special-actions/`.
2. **Witnesses**: `cargo xtask lean check`: canon 165/165, testing 8/8. Soul
   Summons (manifest), Ransom Note and Threats Around Every Corner (manifest
   dread) still prove. New canon: Glacial Stalker (morph; turned face up by
   `setStatus(FaceUp, this)`, values reverting [CR#708.8]), Segmented Krotiq
   (megamorph), Defenestrated Phantom (disguise), Augury Raven (foretell; cast
   permission `onALaterTurn`, so not the turn it was foretold), Djinn of
   Fool's Fall (plot), Mosswort Bridge (hideaway). Pins in
   `Proofs/FaceDown.lean`: `okExileFaceDown` (`exile(…, faceDown(none))`
   checks), `badSpecialActionUnboundAnaphor` (`[anaphor bare one 0]`),
   `badEntryRiderIntoExile`, `badPlacementOntoBattlefield` (`riderKind`),
   `okMorph`, `okMegamorph`, `okDisguise`, `okForetell`,
   `foretoldCastOnALaterTurn`, `okPlot`, `okHideaway`, `okTurnFaceDown`,
   `badTurnAPlayerFaceDown`. English Readings after: Segmented Krotiq 1,
   Aerie Bowmasters 1, Gudul Lurker 2.
3. **Spot checks** (new english-v3 faces, each `Megamorph {cost}` as a
   CostKeyword line): Segmented Krotiq, Marang River Skeleton, Aerie
   Bowmasters, Aven Sunstriker, Misthoof Kirin, Marsh Hulk, Sandstorm Charger,
   Gudul Lurker, Dirgur Nemesis, Ojutai Interceptor. Bodies: morph =
   `[static(mayPlayDeed(cast, you, this, Play(None, None, None, false,
   FaceDown(faceDownCreature, mana([3]))))), static(specialAction(None,
   Param(0), setStatus(FaceUp, this)))]`; foretell's special action is
   `duringPart(Turn, you)`, {2}, `exileFaceDown(this)`, look right
   `visibility(LookAt, you, Objects(it))` while exiled, then a cast
   permission paying `Param(0)` for `onALaterTurn`; plot's is `asSorcery`,
   `Param(0)`, `exile(this)` then a cast permission without paying at
   `asSorcery` for `onALaterTurn`; manifest is `sequentially([turnFaceDown(Param(0),
   faceDownCreature), move(that(Card), wherever, battlefield,
   [FaceDown(faceDownCreature)])])`.
4. **Deviations and STOPs**:
   - Rider split (owner ruling Q28–Q29, recorded in §7): `EntryRider`
     (entersAs, entersAttacking, entersTransformed, entersMelded, withCounters,
     under, asCopyOf, faceDown), `PilePlacement` (inOrder, nthFromEnd,
     orOtherEnd, faceDown, withCounters), `MoveRider = entry | placement`
     (`entry` is `#[macro_ron(embed)]`, so an entry rider is written bare and a
     placement `Placement(…)`; the bench has a `Coe EntryRider MoveRider`).
     `PilePlacement.withCounters` is beyond the ruling's list: "exile it with
     N time counters" (`exileWithCounters`, suspend) is no battlefield arrival.
     Checker: `ZoneExpr.riderKind` (battlefield → entry, other zone or pile end
     → placement, `wherever` → either) and refusal `riderKind`.
   - `FaceDown` is a structure `(characteristics : Option Characteristics)`;
     RON helpers `exiledFaceDown`, `faceDownCreature`, `faceDownWardedCreature`
     (`face_down/`), `exileFaceDown`; aliases `turnFaceDown`, `specialAction`.
     Ward in `faceDownWardedCreature` is the bare keyword term with its cost
     (as the Lean bench writes it), so cloak's ward no longer carries its
     expansion.
   - `SpecialAction` enum homes: `turnFaceUp` → morph/megamorph/disguise
     `specialAction` rows; `foretell` → foretell's row; `unlockDoor` → the
     existing `Instruction.unlock`; `putCompanionIntoHand` → a future
     `specialAction` row in companion (STOP note kept: no outside-the-game zone,
     no once-per-game limit). `ofSpecialAction .turnFaceUp` re-spelled as
     `ofKeyword morph` and `ofKeyword disguise` (Overgrown Zealot) and dropped
     beside the existing `ofKeyword morph` (Qarsi Deceiver): residue, a
     manifested or cloaked card's face-up cost is no keyword's cost and no
     `CostNamed` reads it.
   - Faces bench: Cyber Conversion is `turnFaceDown` with the listed
     characteristics; Missy and Yedora fold "It's a …" into the entry rider's
     spec (no `entersAs faceDown`, no separate characteristic change).
   - Foretell's declaration forwards its cost (`keyword_params: []` override
     removed). Hideaway reads "the exiled card" as `theVerbed(exile, One,
     Card)`; its look right is the ability controller's ("you"). Plot does not
     write the "plotted" designation.
   - STOP, suspend: `when` is "if you could begin to cast this card by putting
     it onto the stack from your hand", which no `Timing`/`Condition` reads
     [CR#702.62a,116.2f]; its other two abilities read "suspended", unnamed.
     Bodyless, note updated.
   - Megamorph declared (facts overlay row, Lean/Idris facts regenerated).
   - Glossary: Entry Rider and Pile Placement added; no `TokenRider` mention
     existed there. Face-Down and Special Action unchanged.
5. **Tests**: `lake build --wfail` 90/90; `cargo test -p
   deckmaste_semantics_v2` (45, 7, 1, 4, 43), `-p xtask` (382 + 1 pre-existing
   ignore, 4, 14), `-p deckmaste_construction_core` (434 + 13 suites), `-p
   deckmaste_lexical_source` (26, 35): all pass; `cargo clippy -p <each>
   --tests -- -D warnings` clean; `cargo xtask facts check` up to date; `cite
   check` 0 stale, `--list-noncompliant` empty, `cite bless` registered
   702.37e, 702.168d, 708.3, 708.4. `lean definitions` 553/553. After the
   second refresh (onto `english-v3-counter-kind-nouns` complete,
   `english-v3-corpus-grep`): `lake build --wfail` 90/90, `lean check`
   165/165 + 8/8, `lean definitions` 698/698, semantics_v2,
   construction_core and lexical_source pass; `cargo test -p xtask` 387 pass,
   1 FAIL, `facts::tests::every_stub_has_a_row_or_a_recorded_reason`: 145
   counter-kind stubs without a `MkCounterFacts` row, all from
   `english-v3-counter-kind-nouns` (which ran no xtask tests). STOP: the fix
   is outside this ticket, so it is reported, not made. Counts:
   restored 0; re-spelled 13 Lean pin statements (rider spellings in
   ActionFamilies, Anaphora ×3, BindingIdentity, Counters, Piles ×2, Zone ×3,
   plus the 2 `lookAndSortSlice`/`scryBy` defs) and the bench card terms in 10
   `Cards/` files, `okManifest`/`okCloak`/`okManifestDread` unchanged over the
   new macros; Rust 1 corpus sample re-spelled (`Placement(InOrder(…))`), 3
   construction_core pins extended for megamorph (list entry, count 105 → 106,
   template, count 264 → 265); ignored 0; added 13 Lean pins, 6 canon cards,
   2 xtask renderers; removed 0.
6. **Timing**: bounded census 3 s wall, 12 workers; `cargo xtask lean check`
   132 s.
