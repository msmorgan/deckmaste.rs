---
needs: []
---
# Decide the event algebra: composition operators, the cause channel, and the two grades of event reader

The trigger header's own holes are
[workbench-trigger-and-interception-residues](workbench-trigger-and-interception-residues.md)'s.
This ticket is the shape one level down: what an event term may be *composed
of*, and what a clause other than a trigger header may say about an event.

## Context

Source: [the v1/v2 comparison](../../memory/scratch/experimental-vs-semantics-comparison.md),
axis 13 and summary finding 2 (2026-08-24), which calls this "the largest
unrecorded shape gap on the list". `semantics-v2.md:§8`'s "reference machinery
before vocabulary breadth" covers more event *names*; nothing records a decision
about the *shape* of event composition, which is why this is a ticket rather
than an entry under a landed round.

## From the v1 comparison (2026-08-24)

> **Crate.** `EventFilter` (`event.rs:283`) is ~32 master forms *plus a
> composable algebra*: `AllOf` (616), `OneOf` (620), `Not` (625), `OneOrMore`
> (629), `Nth { n, of, within }` (633), `When(EventFilter, Condition)` (641),
> `Within(EventFilter, Lookback)` (645), `Before(Reference)` (655). Cause is its
> own narrowing channel: `Cause::Cause(CausePattern { verb, agency, agent })`
> (`event.rs:251,233`) with `Agency` enumerating `CostPayment |
> AttackDeclaration | EffectInstruction | TurnBasedAction | StateBasedAction |
> ManaAbilityResolution | SpecialAction` (`event.rs:133`). One type serves
> triggers (`TriggeredAbility.event`, `ability.rs:91`), conditions
> (`Condition::Happened { event, within }`, `condition.rs:52`), counts
> (`Count::EventCount`/`EventSum`, `count.rs:167`), durations
> (`Duration::UntilEvent`, `continuous.rs:31`), replacements
> (`Replacement::Instead { would, .. }`, `replacement.rs:20`) and
> `StaticEffect::CantHappen` (`continuous.rs:282`).
>
> **What is lost.** The algebra, entirely. There is no `Not`, no `OneOf`, no
> "the second time this turn" (`Nth`), no `When(event, condition)`, no
> `Before(Reference)`. And there is no cause/agency channel:
> `verbMoves`/`verbAgentive` (`Words.idr:~690-712`) are per-verb tables, not a
> `CausePattern` that can narrow "destroyed" from "sacrificed" inside an event
> query.

The report grants that the workbench's `GameEvent` (23 constructors,
`Experimental.idr:2178`) as an English clause is on-contract and nicer than
`ZoneChange { what, from, to, cause }`, and that `eventName`'s projection onto
the 27-member `EventName` (`Events.idr:11`) is "one vocabulary with a
classifier, not two". The composition question is orthogonal to both.

## Two corrections to the report, verified in the workbench

- **A disjunction exists at the header.** `AltEvent`
  (`Experimental.idr:4698-4701`, consumed at 4764) coordinates two events under
  one trigger word, gated by `HeaderNontarget`. So the flat claim "no `OneOf`"
  is wrong at that one site; what is missing is a disjunction that composes
  anywhere else, and the corresponding negation, ordinal and gated forms
  nowhere at all.
- **There are two grades of event reader, and that is the sharper gap.** The
  trigger header takes a full `GameEvent`; every other reader takes only the
  coarse classifier — `HappenedTo : (ev : EventName) -> (w : Lookback) -> …`
  (`Experimental.idr:172`) and `EventCount : (ev : EventName) -> (who : Noun bs
  k) -> …` (`Experimental.idr:1473`). Where the crate asks one type six
  questions, v2 asks a detailed one at the header and a name-only one
  everywhere else. Any algebra decision has to say which grade it operates on,
  or it lands at the header and leaves conditions, counts and durations behind.

## What this round decides

1. **Whether composition is a construction or a spelling.** If `Not`/`OneOf`/
   `Nth`/`When`/`Within`/`Before` become `GameEvent` constructors, each one
   needs its `bs` threading answered — what a disjunct announces when the two
   arms announce different things is the same question
   [workbench-trigger-and-interception-residues](workbench-trigger-and-interception-residues.md)
   already carries as "the coordinated header's readback — 144 of 364", and the
   answer must be one answer, not two.
2. **Whether the two reader grades collapse.** Either the condition/count
   readers are widened to a full event term, or the report's finding is recorded
   as deliberate with the reason.
3. **The cause channel.** Whether narrowing an event by its cause ("destroyed"
   vs "sacrificed", a cost payment vs an effect instruction) is a slot on the
   event or stays distributed across the per-verb tables `verbMoves` and
   `verbAgentive`. Take the `Agency` enumeration as the shape to argue against,
   not as the shape to import.

Measure each operator's corpus surface before minting it. An operator with no
attested line gets a pin, not a constructor — the workbench's own standard.

## Consumption boundary

The Idris workbench only: `idris/src/Experimental.idr` (`GameEvent`,
`eventName`, `AltEvent` and the header's coordination, `HappenedTo`,
`EventCount`, the interception carriers, the duration rows),
`idris/src/Experimental/Events.idr` (`EventName`, `interceptOk`, `spanEventOk`,
`LookbackSubject`, `LookbackComplement`, `eventUse`),
`idris/src/Experimental/Words.idr` (`VerbName`, `verbMoves`, `verbAgentive` if
the cause channel lands), the pin modules `idris/src/Experimental/Proofs*.idr`,
evidence bench `idris/src/Experimental/Cards.idr`. No Rust crate.

## Acceptance

- The composition decision is recorded where `GameEvent` is defined, whichever
  way it goes; "reference machinery before vocabulary breadth" is not reused as
  cover for a shape question it does not reach.
- Each operator that lands names the corpus lines it buys; each that does not
  land is a pin with its measured zero.
- The reader-grade question is answered explicitly: either one event term serves
  the header, the condition, the count and the duration, or the split is
  recorded with its reason.
- The disjunction's announcement rule is the same rule the coordinated header's
  readback gets — one decidable comparison on `Bindings`, not two.
- The cause channel is either a slot with a stated `bs` contribution, or the
  per-verb tables are recorded as the deliberate answer with the narrowing they
  cannot express named.
- `AltEvent` is not duplicated by a general disjunction; if the general form
  lands, the header's row is retired into it.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

## As-landed

**Verdict: no composition operator became a `GameEvent` constructor; the round's
build is the decidable `Bindings` comparison the coordinated header's readback
needed.** One rule serves this ticket and
[workbench-trigger-and-interception-residues](workbench-trigger-and-interception-residues.md)'s
"coordinated header's readback", as both demanded.

- `Experimental/Words.idr` — `sameDet`/`samePlur`/`sameStamp`/`sameOrigin`/
  `sameMaybeBy`/`samePayload`/`sameBinding`/`sameBindings`: structural
  agreement on the discourse, decidable outright because a `Binding` is closed
  first-order data. `samePayload` is heterogeneously indexed on purpose; the
  `kind` field is compared separately by `sameBinding`.
- `Experimental.idr` — `headerCtx (Just alt) ev` now answers `eventAfter ev`
  where the two arms' after-discourses agree whole, and the outer discourse
  otherwise. Whole agreement, not a meet: under partial agreement the sentence
  still cannot say which arm happened. The composition verdict (one operator
  row `NthOccurrence`; disjunction at the header alone; negation in the subject
  predicate; a conditioned event in the intervening slot [CR#603.4]; a window on
  the reader; no agency channel) is recorded on the `GameEvent` docstring.
- `Experimental/Events.idr` — `EventName`'s docstring records the reader split
  as **prospective vs retrospective** (not header-vs-rest) and the
  `eventName`-lifting pattern (`Interceptable`, `durationOk`) as the only
  sanctioned widening route.
- `Experimental/Macros.idr` — `triggeredOr`'s effect argument re-indexed from
  `Effect bs` to `Effect (headerCtx (Just alt) ev)`. Compiler-demanded and
  pre-authorised; body unchanged. It was the only such macro: no sibling failed.
- `Experimental/Cards.idr` — `chubToad` (whole quoted card, Chub Toad {2}{G}
  1/1 Creature — Frog: bare arms both announce the self, tail's "it" reads the
  common announcement) and `infernoElemental` (whole quoted card, {4}{R}{R}
  4/4 Creature — Elemental, whose printed text is this trigger and nothing
  else: both arms announce the block partner, and the tail's "that creature"
  reads that common announcement). Both halves of the family are witnessed by
  whole cards; no shape probe was needed.
- `Experimental/Proofs.idr` — `badAltHeaderMixedReadback`: mixed arms (self
  against partner) announce different things, so the demonstrative's uniqueness
  rule finds no referent. Same ground as `badIt`/`badThemAmbig`/
  `badDisjunctAntecedent`; not a corpus count. Landed on `Macros.gets` rather
  than `SetStatus` because `SetStatus`'s `OnBattlefield` gate is unsolvable
  against an empty discourse and the term could not be stated; the count gate
  (`countWord (TypeW Creature) [] = 1`) is what refuses, as designed.

`AltEvent` is not duplicated and not retired: it is confirmed as the
disjunction's one seat, and the header's `alt` slot is the one seat that never
consults `eventName`.

Regression sentries both still typecheck: `lesserGargadon` (arms agree, context
becomes the SelfD binding) and `colossalGraveReaver` (arms differ, context stays
`[]`). No other bench term or pin was disturbed by non-empty coordination
contexts, so LATITUDE 3 was never reached.

### Re-audit of the design's OPEN items

- **OPEN-1 drift confirmed.** `eventUse` does not exist anywhere in the tree.
  `Experimental/Events.idr` has no such table; its interception gate is
  `interceptOk`. The dispatch and the residues ticket's consumption boundary
  both name it; it is planned-but-unlanded prose. The residues round must not
  lean on it.
- **OPEN-2: no event-pin sibling exists.** `badFlipEvent` and
  `badHeaderMainPhaseWindow` are in no `Proofs*.idr` (the nearest names are
  `badFlipArmWithoutFlip`, ProofsG, an effect-arm pin, and
  `badHeaderBareTurnWindow`, ProofsE, a window pin — neither about
  coordination). Adjudication table below.
- **OPEN-3: landed a whole quoted card**, Chub Toad, not probe-plus-pin. Its
  arms are the bare-partner pair, which announce the self alike; the tail reads
  it back. Aisling Leprechaun (Faerie, in the vocabulary) is blocked *only* by
  its tail — see Ledger. The partner-phrase half of the family has no
  whole-card member that composes today, so it stays a shape probe.
  **Corrected on review:** it does. Inferno Elemental — whole printed text
  "Whenever this creature blocks or becomes blocked by a creature, this
  creature deals 3 damage to that creature" — composes with no new vocabulary
  and is landed in its place. Aisling Leprechaun and Flailing Drake are two of
  ~28 attested members of that half (C2 census below); their blockers are
  theirs, not the family's.
- Chapter/finding numbers: none assigned; the record is docstrings plus this
  section.

### Pin adjudication (OPEN-2)

Every `Triggered` term in every `Proofs*.idr` passes `Nothing` as `alt`. No pin
anywhere asserts anything about a coordinated header, so nothing is superseded
by the `sameBindings`/`headerCtx` rule.

| pin | file | what it refuses | rule it names | verdict |
|---|---|---|---|---|
| `badFlipEvent` | — | — | — | does not exist; residues ticket names a pin that was never written |
| `badHeaderMainPhaseWindow` | — | — | — | does not exist |
| `badFlipArmWithoutFlip` | ProofsG:414 | a win-the-flip arm with no flip written | [CR#705.2] | stays; not a coordination pin |
| `badHeaderBareTurnWindow` | ProofsE:281 | a bare `Turn` header window | its own ground: a window with no possessor restricts nothing (`windowOk`) | stays; not a coordination pin |
| `badCoordinatedLandHostBlocks` | ProofsE:389 | a land host told to block | [CR#506.3] | stays; coordination of *effects*, not events |
| `badCoordinatedHostPlural` | ProofsE:381 | plural anaphor on a single host | [CR#303.4d,301.5c] | stays; still meaningless |
| `badIt` | Proofs:133 | "it" after two singular mentions | demonstrative uniqueness | stays; still meaningless |
| `badThemAmbig` | Proofs:225 | "them" after two group mentions | demonstrative uniqueness | stays |
| `badDisjunctAntecedent` | Proofs:90 | a *noun* disjunction binding no singular player | demonstrative uniqueness | stays; the round licenses no noun-disjunction readback |

No pin retired.

### Operator measurements

All regexes run with `rg -i` over the 36,756 distinct oracle lines of the
supported corpus (`scripts/corpus`, vintage-legal non-reversible; MTGJSON
AtomicCards, CR eff. 2026-08-07).

| # | regex | design verdict | measured |
|---|---|---|---|
| M1 | `^when(ever)? [^,]+ blocks or becomes blocked` | 46 (prior session) | **44** |
| M2 | `^when(ever)? [^,]*\bor (becomes\|is \|are \|attacks\|blocks\|dies\|deals\|enters\|leaves\|gains\|loses\|taps\|untaps\|discards\|draws\|sacrifices\|casts\|activates\|puts\|is put\|was)\b[^,]*,` | 364 coordinated headers (prior) | **329** — a floor; the verb list is not exhaustive. The loose `^when(ever)? [^,]* or [^,]*,` gives 1,495, most of them noun disjunctions ("a red or green spell"). |
| M3 | M2's hits whose text after the first comma matches `\b(that (creature\|player\|permanent\|card\|spell\|token\|land\|artifact\|opponent\|source)\|those (creatures\|cards\|permanents\|tokens)\|it\|its\|them\|their)\b` | 144 readbacks (prior) | **151** |
| M4 | `the (first\|second\|third) time` | no condition/count position wants an `Nth` wrapper | 77 total; 44 header-initial; 33 non-header, of which the overwhelming majority are `if this is the Nth time this ability has resolved this turn` — an ability-**resolution** count, not a `GameEvent` ordinal — and the rest are ordinals on a replacement's `would`, which `NthOccurrence`, being a `GameEvent` row, already composes into. **0 want a new wrapper.** |
| M5 | `until [^."]+ or [^."]+[,.]` | 0 event disjunctions under a duration | 113 raw, **0 genuine** (all noun/keyword lists). Sharpened `until [^."]* or until ` = **0**. |
| M6 | `if .+ would .+ or .+, .*instead` | 0 event disjunctions under a replacement | 98 raw. Sharpened `would .{0,50} or (be \|become\|die\|dies\|leave\|enter\|deal)` = **1**: Illusionary Mask. **STOP — see below.** |
| M7 | `when .+ or .+ this turn` | 0 event disjunctions under a delayed trigger | 47 raw, **0 genuine** (all "instant or sorcery spell"). |
| M8 | `that (wasn't\|weren't\|hasn't been\|didn't)` | subject-predicate negation only | 51, **all 51** subject-predicate ("creatures that didn't attack this turn", "spell you control that wasn't cast"). |
| M9 | `^when(ever)? [^,]+ (doesn't\|don't)` | 0 event-positional | 17 total; 15 subject-side ("a creature you don't control dies") **and 2 event-positional**: Heart of Bogardan and Thought Lash, "When a player doesn't pay this enchantment's cumulative upkeep". **STOP — see below.** |
| M10 | ` before (the\|your\|each\|attackers\|blockers)` | `Timing.BeforePoint`'s seat | 38, **all** activation/casting restrictions ("Activate only before attackers are declared") or non-event ("before the game begins", "the player's life total before the damage was dealt"). **0 event-positional.** |
| M11 | `(died\|entered\|was destroyed\|was sacrificed\|dealt) .* this (turn\|game)` | 0 lookbacks inexpressible by subject predicate + complement slot | 456 raw. Sharpened: retrospective by-source `damage dealt by [^.]{0,40} this (turn\|game)` minus prevention = **2** (Backdraft, Impact Resonance) — the ledger's already-tagged by-source agent item [CR#609.7], not a reader-grade problem; from-zone `from (your\|a\|an\|the) (graveyard\|library\|hand\|exile)[^.]{0,30} this (turn\|game)` = 36, all casting permissions or `CastFrom` on the subject; manner `(died\|entered\|was destroyed\|was sacrificed) this way` = 12, the `ThisWay` carrier's business. **0 confirmed inexpressible.** |
| M12 | `to pay a cost\|as a cost\|as an additional cost.*whenever` | 0 | **0** |
| M13 | `as a result of` | 0 | **0** |
| M14 | `blocks or becomes blocked` (whole family, re-measured on review) | — | **51** distinct lines: **34** name a partner ("… by <NP>"), **17** do not. Of the 34, **28** have a tail demonstrative reading the partner back; three more (Mammoth Harness, Infinite Authority, Venom) read it with "the other creature", an anaphor the M3 regex does not catch, so the true readback population exceeds 151. Whole-card members of the partner half that compose today, verified by build: Inferno Elemental (landed) and Ornery Goblin. Talruum Champion is blocked only by the `Minotaur` subtype. |

### STOP items (measurement against design verdict)

Nothing was minted for either; both are reported rather than acted on, per the
round's instruction.

1. **`OneOfEv` — one attested non-header line (M6).** *Illusionary Mask*:
   "If the creature that spell becomes as it resolves has not been turned face
   up and **would assign or deal damage, be dealt damage, or become tapped**,
   instead it's turned face up and assigns or deals damage, is dealt damage, or
   becomes tapped." This is a genuine event disjunction under a replacement's
   `would`, which falsifies the design's ground 1 ("nothing attests an event
   disjunction under `UntilEvent`, `Intercepts`, `Delayed`, `HeldUntil` or
   `ThisWay`") for exactly one line. Grounds 2 and 3 stand untouched: every
   non-header event reader goes through `eventName`, which a disjunction term
   would leave naming one of two events; and the line is a **three**-way
   disjunction, which the header's binary `AltEvent` could not spell even at a
   header. So the disjunction's one seat is unchanged and no row was minted —
   but the "zero non-header attestation" claim is retired, and the `GameEvent`
   docstring was written structurally rather than as a corpus-zero claim.
2. **`NotEv` — two attested event-positional lines (M9).** *Heart of Bogardan*
   and *Thought Lash*: "When a player doesn't pay this enchantment's cumulative
   upkeep, …". The design expected zero of these. They do not, however, buy
   `NotEv`: the negated thing is a **cost payment**, and this vocabulary rows no
   payment event at all, so a general negation operator over existing rows would
   spell neither line. Recorded as a vocabulary gap, not an operator one.

### Ledger

- **A colour-setting effect row.** *Aisling Leprechaun* — "Whenever this
  creature blocks or becomes blocked by a creature, that creature becomes
  green." Its subject, its arms and its readback all write today; the tail does
  not. `ColorIs` exists as a *predicate* only, and the `Becomes*` effect rows
  set types (`BecomesAlso`) or copy (`BecomesCopy`). That is this CARD's exact
  and only blocker — not the family's; see M14. Worth a measured count of the
  "becomes [colour]" surface as its own family when it is scheduled.
- **The `Drake` subtype.** *Flailing Drake* — "Flying / Whenever this creature
  blocks or becomes blocked by a creature, that creature gets +1/+1 until end of
  turn" — composes in every part except its creature type, which `Subtype` does
  not carry. Not bought here: adding an enum constructor ripples through `Eq
  Subtype` and the emit tables, and this round designed no vocabulary. Again a
  card-level blocker only: Inferno Elemental (landed) needs neither.
- **A cost-payment event.** M9's two lines watch a *failure to pay* a cumulative
  upkeep. No `GameEvent` row names a payment, so neither the positive nor the
  negative is spellable.
- **The by-source agent phrase** ([CR#609.7], already tagged L1) is confirmed at
  two retrospective lines (Backdraft, Impact Resonance) and stays where it is;
  this round does not claim it.

### Gates

- `cd idris && scripts/build` — 19/19 from a clean `build/`.
- `Experimental/Cards.idr` implicit binds `{x = …}`: **0**. `{default` across
  every `Experimental*` module: **0**.
- `cargo xtask cite check --list-noncompliant` — 0 non-compliant.
- `cargo xtask cite check` — 16,571 citations, **0 stale**.
- `cargo xtask cite bless` — 1,477 rules; `cr-citations.lock` unchanged
  ([CR#603.4] was already registered).
- `jj diff --git | cargo xtask cite audit --diff` — 1 site, the new
  [CR#603.4] on the `GameEvent` docstring; the rule is the intervening-"if"
  clause rule and it argues *for* the claim (a conditioned event at a header is
  that slot, not an event operator).
- Bench: 433 → **435** cards, 178 ability terms (unchanged), 440 → **441**
  pins. Nothing lost.
