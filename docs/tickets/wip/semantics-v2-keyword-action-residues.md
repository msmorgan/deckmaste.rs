---
needs: []
---
**Model gaps behind the keyword-action declarations still bodyless after
`semantics-v2-macro-bodies-keyword-actions`.** Its landing record
(`docs/tickets/done/`) holds the per-declaration STOP text. The crate gaps
are `semantics-v2-macro-param-kinds`; the RON macro-language question is
`semantics-v2-macro-capture-and-plurality`. This ticket is the Lean-first
modelling route: each shape below is added to `lean/Semantics` with its pins,
mirrored, then the declaration gets its body.

- No v2 shape: Cloak, Manifest, Manifest Dread (face-down substrate; engine
  side is `engine-face-down` and `engine-special-actions`), Collect Evidence,
  Discover, Incubate (`tokens-predefined-registry` lists Incubator), Learn,
  The Ring Tempts You (`macro-ring-emblem` holds the engine blockers; its
  "future builtin macro expands to `Action::Composite`" framing is v1 and
  moot), Venture into the Dungeon (`engine-dungeons`, same framing, same
  note; `lean-core-room-declarations` holds the designation question),
  Waterbend.
- Rules procedures, not instructions: Activate, Cast, Play.
- One phrase, several expansions: Double, Triple, Exchange
  (`lean-core-exchange-operands` is this question for Exchange alone; fold it
  in), Support.
- Assemble is Un-set only and stays bodyless permanently; delete the
  declaration if english_v3 does not need its spelling.
- Connive 0 makes no connive event [CR#701.50e], but `conniveN(0)` still
  enacts the `connive` deed: the loader wraps the body in `Enact` whatever
  the amount, and no body can say "only if N is greater than 0" around its
  own wrapper (`plugins-v2-connive-bare-and-n` landing record). The same
  shape holds for any keyword action whose rule withholds the event at 0.
  Also from that landing: `conniveN`, like every keyword action, is
  registered at the `Deed` position as `Action("conniveN")`, a deed nothing
  enacts and the facts table does not list, so a card writing it loads and
  is refused only at check (`knownAct`).

`semantics-v1-cutover` deletes `plugins/builtin/macros/action/` wholesale;
any of the 30 still bodyless at parity is a named loss in that record.
`keyword-stub-gate-columns` defers on "once the semantics-v2 keyword macros
are written"; 35 of 65 are, so its ruling reads against this ticket.
Standard constraints apply.

## Landing record

Series on the claim `yzmwxuws`, oldest first: `wmvtyoyn` (a keyword action
enacting another's deed names none), `ymwnrtxx` (zero-amount and assemble
residues), `qntmrymk` (xtask law test re-spelled), `rsttzopl` (Lean shapes,
checker cases, Rust mirror, bench macros, pins), `osykzruy` (collect evidence,
discover, incubate, waterbend), `oxpwrqzo` (double, triple, support),
`osxtqmsn` (activate, cast, play), `pvxzpqxo` (cloak, manifest, manifest dread,
the Ring tempts you), `mrwkzkrm` (glossary, citations, routing note). Numbers
are measured on `mrwkzkrm`. `lean/` changed.

1. **Census delta**: not run. No english_v3 input changed: grammar and
   lexicon declarations are byte-identical, and a keyword spelling is not
   lexical data (`deckmaste_lexical_source/src/legacy/plugins.rs`). Lost
   faces: none.
2. **Witnesses**: `cargo xtask lean check`: canon 150/150 (12 new cards:
   Forensic Researcher, Trumpeting Carnosaur, Eyes of Gitaxias, Geyser Leaper,
   Lead by Example, Saddleback Lagac, Unleash Fury, Galvanoth, Soul Summons,
   Ransom Note, Threats Around Every Corner, Claim the Precious), testing 8/8.
   `lean definitions` 553/553. Pins: `Proofs/KeywordActionResidues.lean`, 20
   theorems. `connive` reads as a deed; `conniveN` is refused at load.
3. **Spot checks** (one per new body, the card that proves it): Forensic
   Researcher pays `collectEvidence(3)` as a cost; Trumpeting Carnosaur
   discovers 5 (run exiled, "that card" cast or put into hand, rest to the
   bottom at random); Eyes of Gitaxias incubates 3 (double-faced Incubator,
   three +1/+1 counters); Geyser Leaper's cost is `waterbend({4})`; Lead by
   Example (spell) and Saddleback Lagac (permanent) support 2 through one body;
   Unleash Fury is `doublePower(target creature, untilEndOfTurn)`; Galvanoth
   may `castWithoutPaying(it, instantOrSorcery)`; Soul Summons manifests the
   top card face down as a 2/2; Ransom Note's first mode cloaks; Threats
   Around Every Corner manifests dread; Claim the Precious has the Ring tempt
   you (named emblem, Ring-bearer chosen).
4. **Deviations and STOPs**:
   - New Lean nodes, mirrored (rename freely): `SliceCount.totalling`,
     `NounPhrase.libraryRun`, `TokenSpec.doubleFaced`, `TokenRider.faceDown`,
     `Cost.tapForGeneric`, `Instruction.procedure` with `Procedure`
     (`activate | cast | play`), and a `name` field on `CreationSpec.emblem`
     (two `ActionFamilies` pins re-spelled to `.emblem none …`). The checker
     gains only cases for them; `libraryRun` introduces the run as a group
     and the stopping card as a part of it, both moved together.
   - New RON helpers: `withTotal`, `libraryRun`, `castWithoutPaying`,
     `procedure`, `createDoubleFacedToken`, `tapForGeneric`,
     `putOntoBattlefieldFaceDown`, `getNamedEmblem`, `doublePower`,
     `doubleToughness`, `doublePowerAndToughness`, `doubleLifeTotal`,
     `triplePower`, `tripleToughness`, `triplePowerAndToughness`. Lean bench
     macros for the bodies (`Actor.collectEvidence`, `discover`, `incubate`,
     `waterbend`, `manifest`, `cloak`, `manifestDread`, `support`,
     `castWithoutPaying`, `withTotal`; `doublePower`,
     `triplePowerAndToughness`, `doubleLifeTotal`). No Lean macro for the Ring
     body; its card proves it.
   - Spellings: activate, cast, cloak, collectEvidence, discover, incubate,
     manifest, play, support and waterbend now spell `<Param(0)>`; the
     template table re-spelled to match.
   - Double, Triple, Exchange stay bodyless: one phrase, an expansion per
     thing changed, each a helper (Exchange's helpers already landed).
     Mana doubling [CR#701.10f] has no helper: no amount reads a mana pool.
   - Exchange fold (`lean-core-exchange-operands`, left in `maybe/`):
     assessed, not folded. Life totals select a group ("two target players"),
     which two `Amount` operands cannot, so the fold needs the machinery that
     ticket rules out.
   - Support has one body for both card classes, `otherThan(this)`: a spell is
     never a target creature, so on a spell it excludes nothing.
   - The Ring: "your Ring-bearer" is written `allOf`/`a` over the designation,
     not "the", since the checker's uniqueness test does not know a
     designation is one per player [CR#701.54a]; the tier counts read
     `eventTally` of the deed this game.
   - Learn routed to `core-location-outside-the-game` (owner, 2026-10-08).
     Venture into the Dungeon routed there too, my call on the same ground:
     its dungeon card comes from outside the game [CR#309.2]. Routing note
     added to that ticket.
   - Commits group by cluster, but the Lean node additions for all clusters
     share files and land in one commit (`rsttzopl`).
   - Glossary: Double-Faced Token, Face-Down Characteristics and Rules
     Procedure added to `docs/contexts/game-model/CONTEXT.md`.
   - Connive 0 [CR#701.50e] (and scry, surveil, endure 0): the term is the
     card's; withholding the event is the rules', as fight's [CR#701.14b] is.
     Owner may overrule. Assemble stays bodyless (owner, 2026-10-05).
   - xtask `lean_check::a_definition_that_breaks_a_definition_law_is_singled_out_and_the_gate_fails`
     was red on trunk (its law, `grantable`, retired); re-spelled against the
     kept `definitionScoped` law, same outcome, checker untouched.
   - STOPs: none open.
5. **Tests** (on `mrwkzkrm`): `cargo test -p deckmaste_semantics_v2` 99
   passed; `-p deckmaste_construction_core` 465 passed; `-p xtask` 392
   passed, 1 ignored (pre-existing); `-p deckmaste_lexical_source` 57 passed;
   `cargo clippy -p <each> --tests -- -D warnings` clean for all four;
   `lake build --wfail` 89 jobs; `cargo xtask lean check` 158/158;
   `lean definitions` 553/553; `facts check` up to date. Cite:
   `--list-noncompliant` 0, `cite check` 0 stale (16540), `cite audit
   --diff` read site by site, `cite bless` registered 24 rules. Restored 0,
   re-spelled 4 (xtask law test; two `ActionFamilies` emblem pins; the
   keyword spelling template table), ignored 0, added 21 (20 Lean pins, 1
   reader test), removed 0.
6. **Timing**: no census run.
