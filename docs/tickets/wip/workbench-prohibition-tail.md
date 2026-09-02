# workbench-prohibition-tail

The prohibition umbrella's surviving remainders, gathered at its close (all
three sub-rounds done 2026-08-28; details in
done/workbench-prohibition-{1,2,3}-*.md As-landed sections):

- **The `MayPlay` fold** — declined by sub-round 3 with its cost measured in
  its ticket: `MayPlay` folds into the `Deontic` carrier as the Cast/Play May
  row once `from`/`limit`/`window`/`exclusive` become label-keyed slots; a
  fourth slot was added to `MayPlay` in the meantime that the fold must carry.
- **Deed-parameter slots not yet minted**: Block's COUNT (the "can block up to
  N additional creatures" ceiling, 1 of 31 lines) and the
  `"AssignCombatDamage"` deed row (with the bare `Blocked` predicate that
  blocks the 19-cell bench — its carrier question is answered).
- **An "abilities you control" predicate** (blocks 4 of the 5 hexproof
  as-though lines).
- **The Aura coordination's anaphoric subject** — 43 activation-prohibition
  lines are conjuncts of an Aura coordination ("enchanted creature can't …")
  waiting on the coordinated subject's anaphor, not on the carrier.
- **Small recorded leftovers**: the ordinal turn-of-game window (3 lines);
  the causative triggered-ability deed (The Master, Multiplied — 1 line); the
  destruction prohibition (1); Xanathar's non-agreeing possessive; Muldrotha's
  "of each permanent type" distributive; Coram's lookback-in-partitive; the
  19 negative-zone cast headers; Necrologia's additional cost; Alhammarret's
  reveal-then-choose as-enters.
- **The amount floor** (19 lines, "X can't be 0") is DECIDED into the amount
  vocabulary ([CR#107.3] — it forbids nothing an agent does); it waits there,
  recorded here so the decision isn't re-derived.

Re-measure everything at claim; the counts are the close-outs'.

- **Routed from workbench-description-2 (close, 2026-08-28):** the
  AS-YOU-ACTIVATE targeting restriction (Keeper of the Flame / Keeper of the
  Light — a restriction on legal targets stated at activation) — deontic
  region machinery.

- **Correction from keyword-1 (close, 2026-08-28): the Crew/Saddle deed-row
  routing premise was wrong.** The 18 as-though lines (count confirmed) need
  a COUNTERFACTUAL-VALUE premise ("as though its power were 2 greater") —
  `AsThoughOf`'s `Predicate bs Object` cannot state a value shift, so the
  as-though slot needs a second premise SORT (consistent with the ruling:
  premise sort depends on the deed). And Revoke Privileges / Bound in Gold /
  Intercessor's Arrest need a PER-DEED complement — `deonticPatientOk` checks
  one shared patient against every coordinated deed, and [CR#506.3] admits no
  artifact at attack's patient. Two named blockers; neither is a deed row.

- **Routed from workbench-keyword-2 (close, 2026-08-28):** Conqueror's Flail
  wants a TEMPORAL WINDOW on a prohibition ("during your turn, opponents
  can't cast spells" — note prohibition-2's `OnlyDuring` may already carry
  it; probe first).

- **Update from workbench-mana-family-residues (close, 2026-08-28):** the
  premise-SORT machinery now exists — `deedCounterfactual` is
  `Maybe PremiseSort` and the `"Spend"` deed carries `ManaPremise`. The
  crew/saddle COUNTERFACTUAL-VALUE premise (the 18 as-though lines) is now
  one sort in that mold, not a redesign.

- **Routed from workbench-static-frame (close, 2026-09-02):** "this token
  can't block" is unwritable — the deed table types Block's agent as Creature
  ([CR#509.1a]) and the marker word (`AsMarker`) ascribes no type; wants a
  deliberate agent-typing decision at the deed facts.

## As landed (2026-09-02)

Every count below was re-measured this round, line-level, over
`jq 'select(.supported)'`. Three of the ticket's premises were wrong and are
corrected here rather than worked around.

### The `MayPlay` fold — DONE

`MayPlay` retires into `Deontic`. A play permission is `Permit` at the
`"Cast"` / `"Play"` deed label with the played card as the statement's
`DeonticCounterpart`, and everything the row carried beyond that —
`from`, `limit`, `window`, `exclusive`, `payment` — is one new
`DeonticRider` slot on the carrier, keyed to those two deeds by a new
`deedPlays` fact. The shape follows `DeonticPatient`'s mold (an
unindexed sum checked once at the carrier against `deedFacts`), not five
universal slots: prohibition-3's measured cost was "three `Maybe` slots
plus `exclusive`", and one rider arm says the same thing without
widening every prohibition's signature by four.

**What retired with it, each because the fold made it a second spelling
of something the deed table already says:** `PlayVerb` (a two-member enum
where the deeds are labels), `castableTy` / `CastableTy` (`deedTypeOk` at
the `Cast`/`Play` patient), `PlaySource` / `MkPlaySource` (the rider
reads `playSourceOk` directly), the `PlayPermission` `StaticKind` row
(nothing read it — `SpanOk` ignores its argument), and the dead
`castComplementOk` table, retired from its seat in a prior round and
never re-consumed.

**Two seams the fold surfaced, both answered on the rules and neither
worked around:**

- **`Play`'s patient types were wrong.** `castableTy Play _ = True`
  admitted playing a conspiracy or a dungeon on no rule at all. Playing a
  CARD is defined as "play that card as a land or cast that card as a
  spell, whichever is appropriate" ([CR#305.1] and [CR#601.2] for the two
  halves), so the row now carries the union and the six types
  [CR#311.2,312.2,313.2,314.2,315.3] and [CR#309.2c] say can't be cast
  stay out. The fold TIGHTENED here.
- **The `Cast` patient's zone means two things.** [CR#601.2a] takes the
  object "from where it is" and puts it on the stack, so a licence's
  complement is a CARD in a castable zone where the same deed's
  prohibition ("spells with the chosen name can't be cast") describes the
  SPELL the deed produced. `roleZone` can name one zone and the licence's
  is not one zone, so `counterpartFits` gained a last argument saying the
  complement is named before the deed moves it; the licence's zone
  question is `playSourceOk`'s, asked at the rider with the written
  source in hand. `deedFacts`' `Just Stack` is untouched, so the
  prohibition keeps its refusal.

**Every `MayPlay` witness still elaborates**, respelled through the nine
macros (whose signatures are unchanged at the call site) plus a new
general `mayPlayDeed`; the three raw sites (Flawless Forgery, Summon:
Esper Valigarmanda, Apex of Power) go through `mayPlayDeed`. **All four
permission pins still refuse**, respelled over the carrier:
`badPlayFromBattlefield`, `badPlayFromStack` and `badPlayFromWrongZone`
at the rider's gate, `badCastALand` at `deedTypeOk`. **Pin probe run on
both re-pointed gates**: adding `Land` to `Cast`'s patient types makes
`badCastALand` fail with "not a valid impossible case", and flipping
`complementLocates (Just Battlefield)` to `True` does the same to
`badPlayFromBattlefield` — so each still refuses for its own rule.

**What the fold costs, measured:** the statement's `staticIntro` is now
the SUBJECT's (`selfSubjIntro n`) where `MayPlay`'s was the complement's.
Measured zero: no witness reads a mention the play permission's
complement announced — Rogue Class and Summon: Esper Valigarmanda both
write their second conjunct's description out, and Rogue Class already
records the anaphoric purpose as a gap on `SpendPurpose`.

### The counterfactual-VALUE premise — BUILT

**18 supported lines** (count confirmed), 4 of them the printed static
and 14 the same sentence quoted inside a created Pilot token's text; all
18 write the same characteristic and the same shift. `PremiseSort` gains
`ValuePremise` and `AsThough` gains `AsThoughGreater ch amt` —
[CR#702.122a] makes the crew cost a total POWER and [CR#702.171a] the
saddle cost, so what the permission counterfactualises is a quantity and
`AsThoughOf`'s `Predicate bs Object` could never have stated it. One
direction, minted because it is the one printed, on `LoseCause`'s law;
the amount announces nothing, on `MayPlayAdditionalLands`' law.
The `"Crew"` and `"Saddle"` DEED rows land with it — keyword-1 declined
them because no witness benched, and with the premise sort and the
per-deed complement both here, witnesses do.
**Benched: `hotshotMechanic` whole** (Hotshot Mechanic is the sentence
and nothing else), **`cloudspireCaptainCrewLine`**.

### The per-deed complement — BUILT

`DeonticPatient` gains `CounterpartsAt`, a list of `DeedComplement`
(a deed label and a noun), each checked against ITS OWN deed alone.
A second arm and not a widening of `DeonticCounterpart`: that one is the
SHARED complement, checked against every coordinated deed, which is what
a single-deed statement writes; this one is one conjunct's.
**3 supported lines over 3 cards** (Revoke Privileges, Bound in Gold,
Intercessor's Arrest — the ticket said 4) write "can't attack, block, or
crew Vehicles", where [CR#506.3] admits only a planeswalker or a battle
at an attack's patient and [CR#702.122d] states the crew conjunct's
meaning outright. The same arm carries **13 of the 18** crew/saddle lines,
which write a complement at EACH of two deeds. `distinctDeeds` is minted
beside it and is also asked of the carrier's deed list.
**Benched: `revokePrivileges` whole.**

### The self-block token cell — BUILT, and the decision recorded

The decision is at `deedFacts`, as the routing asked: **`roleBare` flips
to `True` at Attack's and Block's AGENT** and nowhere else. [CR#506.3]
says which objects can attack or block, not how a sentence may describe
the one it is said of, and the corpus settles it — "Enchanted permanent
can't attack, block, or crew Vehicles" (2 printed cards) and the **45**
supported lines quoting "This token can't block" both name an object
whose card type the noun never writes. The patient keeps its closed
types: there the rule names what may be attacked or blocked.
What still refuses is a subject that WRITES an excluded type
(`badCoordinatedLandHostBlocks`, `badPlaneswalkerAttacks`,
`badCantAttackLand`, `badCantBeAttacked` all unaffected) and a subject
off the battlefield.
**COST, recorded: `badCantDisjunctSubject` retires.** "Target creature or
land can't block this turn" was refused because a coordinated head
collapses to no type; that is the same mechanism the decision overturns,
and the sentence is not rules-impossible — the land simply also can't
block by rule. The honest refusal would ask the head's DISJUNCT types,
which `nounTy` does not expose; recorded, not built.
**Benched: `harriedSpearguard` whole.**

### Conqueror's Flail's temporal window — MEASURED ZERO

Probed before any edit, and the routing note's own hedge was right:
`OnlyDuring Turn (Just Yours)` already carried it, and Grand Abolisher
benches the identical prohibition ("your opponents can't cast spells
during your turn"). What the line still wanted was the leading attachment
condition, which `Conditionally` and `AttachedTo` already spelled.
Nothing minted. **Benched: `conquerorsFlailProhibition`** (the second
line). The card's first line waits on a count over a COLOUR axis, which
is no part of this region.

### The as-you-activate targeting restriction — MEASURED ZERO

**8 supported lines** write "as you activate this ability"; 5 of them are
the Keeper family's targeting restriction and 3 are a letter's value read
at activation. Nothing minted: the restriction is the target noun's own
predicate (a comparison of the opponent's life total against yours, which
`Compare` already spells over the player axis), and the adverbial is
spelling of the step the rules already put the choice in — [CR#602.2b]
hands activating the whole of [CR#601.2b..601.2i], whose [CR#601.2c] is
the target announcement, and [CR#608.2b] rechecks legality on resolution
whatever the line says, so no activation-only scope could be what the
phrase means. **Benched: `keeperOfTheFlame` whole.**

### The Aura coordination's anaphoric subject — BUILT, nothing minted

**53 activation-prohibition lines**, not 43 (the ticket's count was a
prior session's). The routed premise was wrong in its shape: the second
conjunct's subject is not an ELIDED one, so `SubjectVP`/`VPDeontic` was
never what it waited on — it is the host's abilities read back with a
possessive, and `AbilityOf It` over the mention the first conjunct
announced is the whole of it. **12 of the 53 write today**
("Enchanted creature can't attack or block / can't block, and its
activated abilities can't be activated"). **Benched: `arrest` whole.**

Residue, by named blocker:
- **7** write "enchanted permanent" / "enchanted artifact" as the host —
  `AttachHost` takes a type WORD and "permanent" is not one.
- **4** add "unless they're mana abilities", an exception carved out of
  the abilities description rather than a cost gate.
- **3** coordinate the untap statement instead ("doesn't untap during its
  controller's untap step and its activated abilities can't be
  activated").
- **2** write the possessive with no coordination ("Enchanted creature's
  activated abilities can't be activated"); **2** more pair it with a
  sacrifice/exile ignore-clause and **1** with a P/T shift.
- **12** are detain's parenthetical reminder text and **5** more the
  one-shot "until your next turn" statement; **5** are other shapes
  (a tapped permanent's this-turn restriction, a countered ability's).

### The amount floor

Untouched, as the ticket directs: the decision stands recorded in the
amount vocabulary ([CR#107.3]) and nothing was built here.

### Remainders this round did NOT clear

The ticket's small recorded leftovers are all still open and unmoved:
Block's COUNT slot, the `"AssignCombatDamage"` deed row, the "abilities
you control" predicate, the ordinal turn-of-game window, the causative
triggered-ability deed, the destruction prohibition, Xanathar's
non-agreeing possessive, Muldrotha's distributive, Coram's
lookback-in-partitive, the 19 negative-zone cast headers, Necrologia's
additional cost, Alhammarret's reveal-then-choose. Two more are minted
here: the disjunct-head type read above, and the attachment host word
that names no card type.

### Gates

`idris/scripts/build` green from a clean `build/`, **23/23**, 0 errors, 0
warnings. `cargo xtask cite check --list-noncompliant` empty; `cite check`
0 stale over 20,059 citations; `cite bless` newly registered
[CR#702.122b], [CR#702.122d] and [CR#702.171c], each read against its
citing claim; `jj diff --git | cargo xtask cite audit --diff` read over
**42** citation sites in this round's own diff, no wrong-topic cite found.
