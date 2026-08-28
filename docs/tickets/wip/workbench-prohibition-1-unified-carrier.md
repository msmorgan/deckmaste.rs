# prohibition-1: the unified deontic carrier

Sub-round 1 of [workbench-prohibition-and-permission-acts](workbench-prohibition-and-permission-acts.md)
(the umbrella — authoritative; its TWO RULED paragraphs of 2026-08-27 are this
round's design, verbatim). Owns: the unified carrier itself — one deontic
carrier, full modal algebra (May/Can't/Must/Gate), deed slot an open
`VerbLabel` behind `KnownVerb`, per-deed parameter shapes as label-keyed
facts/slots in the `verbFacts` mold; `Deed = Attack | Block` retires into
labels; `PlayerCant`/`ObjectCant` retire into (or respell over) the carrier;
the `asThough : Maybe premise` slot (May only, premise SORT per deed;
`PlayAsThough = HadFlash` collapses into `May Cast … asThough HasFlash`).
Plus: the umbrella's "creature permission" section (118 sentences, six cells —
the permission row FIRST, the counterfactual slot after; the
player-subject/creature-actor question of the 19-sentence cell answered
explicitly; the negated-keyword payload decision made and recorded); the
TARGETING prohibition (39 real sentences — the by-source/by-spell agent as a
label-keyed parameter, never an `ObjectAct` member); the coordination designed
ONCE across event, deed and gate (the deed's 6 "can't attack or block" lines
and the gate's 2 cards elaborate through it); and the routed
`DeonticCounterpart` self-block pin ([CR#509.1a]).

Pins: the two RULED paragraphs are STOP fences — no closed Deed enum, no
counterfactual node above a deontic, slot on May only with the other
modalities' zero measured. Measured zeros stay ("must be countered" 0,
"can't be countered unless" 0). The four permission pins still refuse after
the round. Re-measure the 118/264 surface before building. Standard
constraints apply.

## As landed (2026-08-28)

### The carrier

`StaticEffect.Deontic` is now the region's ONE modal carrier:

```
Deontic : {k : Kind} -> (n : Noun bs k) -> (c : Compulsion bs) ->
          (deeds : Deeds) -> (role : Role) ->
          (patient : DeonticPatient {bs = nomIntro n} deeds role) ->
          (asThough : Maybe (AsThough (nomIntro n))) ->
          {auto 0 ne : NonEmpty deeds} ->
          {auto 0 kd : KnownDeeds deeds} ->
          {auto 0 zn : ZoneFits (nounZone n) (deedsZone deeds role)} ->
          {auto 0 dp : DeedParticipant deeds role k (nounTy n)} ->
          {auto 0 pt : So (deonticPatientOk n deeds role patient)} ->
          {auto 0 at : So (asThoughOk c deeds asThough)} -> StaticEffect bs
```

- `Compulsion = Forbid | Require | GatedBy (Cost bs) | Permit` — the full
  May/Can't/Must/Gate algebra. `Permit` is the row that was missing.
- `Deeds = List VerbLabel`. The deed slot is the OPEN label vocabulary
  behind the fail-closed `KnownDeeds`; there is no closed `Deed` enum.
- The subject widened to any `Kind`, which is what let the player-subject
  and ability-subject carriers retire into this one.
- `asThough : Maybe (AsThough bs)` with `AsThoughOf : Predicate bs Object
  -> AsThough bs`, gated May-only by `asThoughOk` and per-deed by
  `deedCounterfactual`.

`Experimental.Events` gained the deed table in the `verbFacts` mold:
`DeedRole { roleKinds, roleTypes, roleBare, roleZone }`, `DeedFacts
{ deed, deedAgent, deedPatient, deedDefends, deedTargeted,
deedCounterfactual, deedRides }`, `deedFacts` (14 rows: Attack, Block,
Target, Cast, Play, Counter, Copy, Activate, Regenerate, GainLife,
DrawCard, SearchLibrary, LoseGame, WinGame), `knownDeed`/`KnownDeed`,
`deedKindOk`/`deedTypeOk`/`deedBareOk`/`deedZoneOf`, and `deedsZone` for
the coordination. `DeedParticipant` re-indexed on the deed LIST and
gained `BareParticipant` for the typeless participant (a player, an
ability, "spells with the chosen name").

**Deviation, flagged.** The ruling says the deed slot sits "behind the
fail-closed `KnownVerb` gate". It sits behind `KnownDeed`, a second
table in the same mold, sharing the label TYPE but not the rows. Reason:
`verbFacts` is read by `Enact`, `Does`, `VerbedEvent` and the participle
anaphors, so deed rows added there would make "you attack", "the targeted
card" and "whenever you lose the game" spellable at four seats no line
asks for — the opposite of fail-closed. The ruling's substance (no closed
enum, one open label vocabulary, membership gate, per-deed parameters as
label-keyed facts) is unchanged.

### Retirement / respell outcome

| was | now |
| --- | --- |
| `Deed = Attack \| Block` + 76-row `deedType` | two rows of `deedFacts` |
| `StaticEffect.PlayerCant` over `PlayerAct` | `Macros.playerCant` = `Deontic who Forbid [deed] Agent` |
| `StaticEffect.ObjectCant` over `ObjectAct` | `Macros.objectCant` = `Deontic what Forbid [deed] Patient` |
| `StaticEffect.OutcomeGate` over `OutcomeGateKind` | `Macros.playerCant "LoseGame"/"WinGame"` |
| `Phrase.ActSubject` (6 rows) | `roleKinds`/`roleZone` on the deed row |
| `Words.riderAct` | `deedRides` |
| `Events.PlayAsThough = HadFlash` | `AsThoughOf (HasKeyword "Flash")` |
| `Effect.CantBe`'s `ObjectAct` arg | the same deed labels + `deedRides` |

All four carriers RETIRED — none needed sub-round 2's qualified
complements, because the complement they lacked was never a slot on
them. The unification's payoff: one act at two ROLES retires the two
parallel enums, so `PlayerCant PlaysLands` and `ObjectCant Played` are
now `"Play"` at Agent and at Patient. Every existing bench witness still
elaborates.

`StaticEffect.MayPlay` did NOT retire (sub-round 3's seam, below); its
`asThough` slot took the shared premise, and `PlaySource`'s third index
became a `Bool` (premise PRESENCE), since the flash value is no longer a
distinguishable case.

### The two recorded decisions

1. **The 19-sentence cell's carrier** (recorded on `Compulsion.Permit`):
   the subject is the DEED'S ACTOR — the creature — never the player who
   chooses to apply it. [CR#510.1a] makes each attacking and blocking
   creature the thing that ASSIGNS combat damage and [CR#510.1] gives the
   player only the announcement of how; and [CR#609.4]'s "treat the game
   exactly as if the stated condition were true" has nothing to mean if
   "as though it weren't blocked" is predicated of a player. The printed
   "You may have this creature …" is `Permit` spelling its own
   optionality on the controller, not a second subject.
2. **The negated-keyword payload** (recorded on `AsThough`): YES, a
   general counterfactual vocabulary, and it is v1's arbitrary
   `Predicate` — argued FOR. The 264 sentences write six unrelated
   payload families (a keyword, a characteristic comparison, a combat
   fact, a status, counters, existence), each already a
   `Predicate bs Object` here, with `Not`/`Negatable` supplying the
   negation; [CR#609.4] states no restriction on the condition, so a
   closed negated-keyword list would refuse rules-meaningful sentences
   and pin nothing. The ONE arm is deliberate: [CR#609.4b] gives
   mana-spending its own rule and its own payload, and that arm is the
   mana family's round.

### Re-measured counts (2026-08-28, `jq 'select(.supported)'`, line-level)

| claim | umbrella | measured | note |
| --- | --- | --- | --- |
| "as though" surface | 264 / 261 cards | **264** (314 naive − 50 phasing reminder) | confirmed exactly |
| attack despite defender | 52 | **52** | confirmed |
| assign damage as though unblocked | 19 | **19** | confirmed |
| crew/saddle at greater power | 18 | **18** | confirmed |
| blocking despite evasion | 17 | **17** | confirmed |
| attack despite summoning sickness | 7 | **7** | confirmed |
| targeted despite hexproof/shroud | 5 | **5** | confirmed |
| player-may flash side | 88 | **89 lines / 88 cards** | one card writes two |
| targeting prohibition | 39 real (114 naive, 75 reminder) | **30 real (216 naive, 186 reminder)** | CORRECTED |
| "can't attack or block" | 6 lines | **109 lines** | CORRECTED |
| outcome coordination, one subject two kinds | 2 cards | **1 card** (Everybody Lives!) | CORRECTED — "Platinum Persecutor" is not a card; Abyssal Persecutor/Platinum Angel write two SUBJECTS, a plain conjunction |
| "must be countered" | 0 | **0** | zero holds |
| "can't be countered unless" | 0 | **0** | zero holds |
| "as though" under can't/must/gate | assumed | **0** | measured, not assumed — the May-only slot's warrant |

### Benched

- **Bristlepack Sentry**, whole card — the permission + premise, largest cell.
- **Pacifism**, whole card — `Forbid ["Attack","Block"]`, the coordination.
- **Gaea's Revenge**, whole card — the targeting deed with the
  by-source/by-spell agent.
- **Nowhere to Run**'s first line — the targeting PERMISSION with the premise.
- **Everybody Lives!**'s gate conjunct — `Forbid ["LoseGame","WinGame"]`,
  the SAME coordination at the gate.
- New keyword catalog row: `Defender` ([CR#702.3a,122.1b,702.3b]).

### Pins

- New: `ProofsD.badBlocksItself` — a bare `It` in `DeonticCounterpart`
  resolving to the Deontic's own subject, refused by `counterpartNotSelf`
  on [CR#509.1a]/[CR#508.1a]. **Pin probe run**: flipping the gate to
  `True` makes the pin fail with "not a valid impossible case", so it
  refuses for its stated reason and not by accident.
- Re-expressed onto the carrier, still refusing:
  `badStaticPlayerCantTargets`, `badCounteredInGraveyard`,
  `badFlashPermissionOnPermanent`, `badActivatedSpellClass`,
  `badCastAbilityClass`, `badRegeneratedInGraveyard`,
  `badTargetedOutcomeGate`, `badForbidAttackWithPatient`,
  `badCantAttackLand`, `badCantDisjunctSubject`, `badCantBeAttacked`,
  `badMustAttackLand`, `badCoordinatedLandHostBlocks`,
  `badPlaneswalkerAttacks`. The four permission pins still refuse (the
  full build is the evidence: a pin that stopped refusing fails the build).

### SEAMS for sub-rounds 2 and 3

- **`MayPlay` folds into the carrier (sub-round 3).** It is the May row
  of the "Cast"/"Play" deeds already; what keeps it separate is its
  `from`/`limit`/`window` slots. Under the ruling those become
  label-keyed slots on the Cast/Play rows of `deedFacts` (the row already
  has room). `PlaySource`'s `Bool` index is the only thing that reads the
  premise's presence; it becomes a deed fact too. `MayPlayAdditionalLands`
  and `Visibility` are untouched.
- **Qualified complements (sub-round 2)** attach as a COMPLEMENT slot on
  the carrier, not as new act rows: `Deontic who Forbid ["Cast"] Agent`
  wants the object description the 74 lines carry. Note the shape the
  targeting deed already uses — `DeonticPatient.TargetedBy` carries a
  whole `Noun` at a joined kind — which is the same answer one level up.
  `CantUntapMoreThan`'s count cap is the fifth qualifier family and is
  still its own row.
- **Ability-class residues (sub-round 2)** meet the carrier at
  `deedFacts "Activate"`, whose patient is `roleKinds = [Ability]`,
  `roleTypes = []`, `roleBare = True`.
- **Outcome-gate READERS (sub-round 3)** — "for each player who has lost
  the game", the partial-cause immunity — read a state, not this carrier;
  the gates themselves are now deed labels `"LoseGame"`/`"WinGame"`.
- **The spend deed / `[CR#609.4b]` premise (mana family)** is `AsThough`'s
  second arm: a mana-symbol matcher, not a `Predicate bs Object`.
- **Adding a deed** is one `deedFacts` row and nothing else. `Attach`,
  `Untap`, `Spend`, `AssignCombatDamage`, `Crew`, `Saddle` are rows away.

### Remainders (not this round)

- **`Blocked` is missing from the predicate vocabulary**, so the
  19-sentence assign-damage cell cannot bench yet: `Blocking`,
  `BlockedBy m` and `CouldBeBlockedBy m` all name a counterpart, and "as
  though it weren't blocked" needs the bare one. Its carrier question IS
  answered (above); only the premise word is missing. Cheap: one
  `Predicate bs Object` row.
- **"Abilities you control"** has no predicate (`ControlledBy` is
  `Object`-kinded; `ActivatedBy` is a different relation), so Glaring
  Spotlight, Detection Tower, Kaya and Autumn Willow — 4 of the 5
  hexproof lines — still do not bench. Nowhere to Run's bare "spells and
  abilities" is the one that does.
- **The unified algebra makes "must be countered" and "can't be countered
  unless" WRITABLE** where the Cant-only carriers made them unwritable by
  construction. Neither is rules-impossible, so neither gets a pin; both
  stay at their measured zero, with no bench witness. Recorded so the
  widening is not read as an accident.
- The 18-line crew/saddle cell and the 7-line summoning-sickness cell
  want `"Crew"`/`"Saddle"` deed rows and a haste premise; rows away, not
  written for want of a printed blocker to clear.
