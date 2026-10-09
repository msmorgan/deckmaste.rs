---
needs: []
---
**The keyword-ability census.** Split from
`semantics-v2-keyword-ability-residues` (done 2026-10-08), whose routed items
landed (offspring and squad bodied; an impending canon card) and whose opening paragraph did not.

- **Census.** `semantics-v2-macro-bodies-keyword-abilities`' second landing
  record (`docs/tickets/done/`) left 87 keyword-ability declarations
  bodyless. Name each one with its STOP cause and the ticket that owns it
  (`semantics-v2-subject-param-kind`, `semantics-v2-macro-param-kinds`,
  `semantics-v2-macro-capture-and-plurality`, or this ticket), then take the
  unowned remainder Lean-first: each shape added to `lean/Semantics` with its
  pins, mirrored, then the declaration bodied. Offspring and squad are no longer
  in the list.

Standard constraints apply.

## Landing record

Scope: the census against the tree at `upmowlqn` (82 bodyless keyword-ability
declarations, not the 87 of the older record: offspring, squad, crew and the
other bodies landed since), the four bodies the existing vocabulary supports,
and every remaining residue's missing piece written into its declaration as
`STOP (waits on: …)`. No Lean node was added.

1. **Census delta** (`orylzmqs`). No English grammar, spelling, lexicon or
   corpus input changed: only keyword bodies, comments, one checker fact row
   and the facts overlay. Covered faces and Readings did not move and no
   census was run. Lost faces: none. Bodyless keyword abilities 82 → 78:
   4 bodied, 13 settled empty (the 12 earlier ones plus prototype, which has
   no registry row by design), 65 residues. Lean canon 165 → 169 proved.
2. **Witnesses** (`orylzmqs`). Alacrian Jaguar (saddle), Timeline Inquiry
   (teamwork), Coastal Discovery (awaken) and Darksteel Myr (indestructible)
   each prove `Card.check = []`. Pins in `Proofs/KeywordAbilityCensus.lean`:
   `okSaddleDefinition`, `badSaddleUnresolvedBound`, `okTeamworkDefinition`,
   `okAwakenDefinition`, `okIndestructibleDefinition`, `badDestroyedPlayer`.
   No English Readings are involved.
3. **Spot checks.** Four new faces, not ten: the ticket bodies four keywords.
   - Saddle 1: `activated(perform(tap(withTotal(Stat(Power), AtLeast, 1,
     allOf(and([untapped, otherCreatureYouControl(thisPermanent)]))))),
     gainDesignation(thisPermanent, saddled, untilEndOfTurn), asSorcery)`
     [CR#702.171a].
   - Teamwork 2: `addedCost(perform(tap(withTotal(… allOf(creatureYouControl)))),
     true)`; "cast using teamwork" is `keywordCostPaid(teamwork, this)`
     [CR#702.194a,702.194b].
   - Awaken 4—{5}{U}: `mayCastFor(you, cost)` plus a spell ability guarded by
     `keywordCostPaid(awaken, this)` that puts the counters on a target land
     you control and makes "that land" a 0/0 Elemental creature with haste
     [CR#702.113a].
   - Indestructible: `objectCant(destroy, thisPermanent)` [CR#702.12b].
4. **Deviations and STOPs.**
   - Checker data, not a node: the destroy deed's patient role was
     `fieldObject` (no card types), so `deedFits` refused every subject; it
     now reads a permanent on the battlefield, as sacrifice's and untap's do
     [CR#701.8a]. No pin changed outcome.
   - Facts overlay (`crates/xtask/src/facts.rs`): awaken's cast permission
     takes the keyword's casting regime (`open_regimes`), as impending's does;
     teamwork is `paid_cost` [CR#702.194b]. `Facts.lean` and `FactsGen.idr`
     regenerated; `facts check` up to date.
   - The four bodies land in one commit, not one per cluster: the pins share
     one file and the regenerated `Facts.lean` carries all four rows, so a
     per-cluster split would leave intermediate commits with stale facts.
   - Protection was attempted and stays a residue: three of its four
     consequences check, but "can't be enchanted/equipped/fortified by"
     forbids attaching an object TO this permanent, and `DeonticPatient`'s
     counterpart is the deed's other role (the attaching player), so the host
     has no slot. That is a node, so it is listed, not invented.
   - No STOP against a §7 ruling. No glossary gap.
   - **Residues, grouped by the mechanism they wait on** (each declaration's
     comment names the exact missing piece). Groups of two or more are
     candidates for one general ticket each:
     - A static ability that creates a delayed triggered ability: warp, dash,
       blitz, rebound (rebound's static is also a replacement whose
       replacement is an instruction).
     - The ability printed with the keyword (an `Ability` keyword parameter,
       or a selection of the abilities printed below it): forecast, boast,
       exhaust, solved, infinity, max speed (also a player speed amount),
       power-up (also no CR entry), backup.
     - A sharing predicate ("shares a color / creature type with X"):
       intimidate, conspire, amplify, prowl.
     - A paid-cost facet naming the object a cost acted on (the object
       analogue of `PaidFacet.chosenPlayer`): emerge, offering, harmonize.
     - An optional or action-performing entry clause: devour, riot, tribute,
       unleash (amplify needs it too).
     - A condition that a spell has any targets: casualty, replicate, epic
       (replicate's repetition and epic's delayed trigger are otherwise
       writable now).
     - A static that sets or charges for a modal spell's mode choice: entwine,
       escalate, spree, tiered.
     - A static ability that acts when a state holds, off the stack: ascend,
       storied, daybound, nightbound.
     - A text-changing effect over printed rules text: cleave, overload,
       splice.
     - A play permission that casts a card's other face: disturb, More Than
       Meets the Eye.
     - Commander and outside-the-game vocabulary: partner, freerunning,
       companion (`core-location-outside-the-game`, plus a once-per-game limit
       on a special action).
     - A named link between two objects: haunt, soulbond, cipher, mutate.
     - The damage-result rules table, not a definition: infect, lifelink,
       wither, toxic.
     - A deontic slot for an attachment's host: protection.
     - Single: suspend ("could begin to cast" and "suspended"), miracle
       (reveal permission with a timing, reveal `thisWay`), echo
       (control-change event, "your last upkeep"), frenzy ("attacks and isn't
       blocked" event), enlist (cost to attack), jump-start (own cost plus an
       additional cost on a play permission), compleated (life paid for
       Phyrexian symbols), read ahead (an ability's chapter number), craft (a
       materials parameter), paradigm (resolution-history condition, a copy
       in exile), assist (another payer; deliberate, ruling 2026-10-08).
     - Owned elsewhere: champion (`semantics-v2-subject-param-kind`), encore
       (`semantics-v2-per-member-handoff`), ripple
       (`lean-drt-anaphora-refactor`).
5. **Tests** (`orylzmqs`, after `kata refresh`, which brought in only a
   ticket move).
   - `lake build --wfail` green. `cargo xtask lean check`: canon 169/169,
     testing 8/8. `cargo xtask lean definitions`: 698/698 (218 counters, 461
     subtypes, 19 designations). `cargo xtask facts check` up to date.
   - `cargo test -p deckmaste_semantics_v2`, `-p deckmaste_construction_core`
     and `-p xtask` green (xtask 388 + 4 + 14, 1 ignored as before).
     `cargo clippy -p {deckmaste_semantics_v2,deckmaste_construction_core,xtask}
     --tests -- -D warnings` clean.
   - Cite checks: 0 non-compliant, 0 stale; `cite bless` registered 702.194b
     (read against its claim); the audit was read against each rule's text.
   - Counts: restored 0, re-spelled 0, ignored 0, added 10 (six Lean pins,
     four canon card proofs), removed 0.
6. **Timing.** No census was run. One `cargo xtask lean check` took 228
   seconds (lake default jobs).
