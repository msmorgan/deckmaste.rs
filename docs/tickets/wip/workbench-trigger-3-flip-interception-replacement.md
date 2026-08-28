# trigger-3: flip conditions, the interception bench, and replacement seats

Sub-round 3 of [workbench-trigger-and-interception-residues](workbench-trigger-and-interception-residues.md)
(the umbrella — authoritative). Owns its sections: "The flip verb" (condition
blockers: Kitsune Mystic's enchanted-by-two, Bushi Tenderfoot's by-source
lookback, Akki Lavarunner's source-side damage event; `badFlipEvent` and the
unflip cell stay), "Standing triggered abilities" (re-measure first), "The
ENTRY interception's owed bench" (the land cycle's "if you don't" beside
`Reflexively`; the cast-history conjunct — Hallowed Moonlight whole; the two
singletons; Sheltered Valley is NOT the cheap one — its sacrifice clause and
name-match are the real blockers), "Blood Spatter Analysis" (flat bloodstain
`CounterKind` row on finding 481's bar + the reflexive sacrifice pair), "The
granted quoted replacement" (`eventUse StatusChange` admits a replacement
antecedent WITHOUT opening deliberately trigger-only cells; the untap-step
window takes a SEAT on `Intercepts`, never a second window vocabulary;
Bewitching Leechcraft's family benches), "The by-source rider" (pay only if
the [CR#609.7] rider exists at claim; else record waiting), plus routed: the
agentless "this way" reflexive node ([CR#603.12] second form), and the
PERIPHRASTIC CAUSER COMPLEX — landed TOGETHER or not at all: the "would CAUSE
X to Y" verb (4 lines), `Intercepts`' body VOICE slot, Modular as
`keywordFacts` + triggered `AbilityClass` (composes with sub-round 1's
`AnyTriggered`), and the prospective "that would be created" predicate
(Crafty Cutpurse).

The energy replacement stays routed to the symbol round; the
non-interceptable `eventUse` partition stays untouched — the umbrella lists
it so it is not re-derived. Standard constraints apply.

Routed from trigger-1 (close, 2026-08-27): the `Causer`-as-noun widening now
carries THREE payers landing with the periphrastic complex — the umbrella's 4
"would CAUSE" lines, Karmic Justice's causer-voiced header (1 line), and
Cobra Trap's by-source lookback agent (1 line). And Fblthp, the Lost's cast
disjunct is the agentless passive "was cast from your library"
(`lookbackSubjectOk SpellCast Object` refuses) — the SAME cast-relation as
this round's cast-history conjunct (b); 4 of the 9 windowless-lookback lines
write it, so landing (b) should bench Fblthp too.

## As landed (2026-08-28)

All counts re-measured over `data/derived/cards.jsonl` through
`jq 'select(.supported)'`. Gate: `idris/scripts/build` clean, 23/23, exit 0.
`cargo xtask cite check --list-noncompliant` empty; `cite check` 0 stale over
18,615 citations; `cite bless` registered no new rule; the diff audit read
every citation site added.

### Rows built (seven)

- **`Predicate.WasCast`** — the agentless passive cast history,
  [CR#601.2]'s procedure asked as a yes/no. Neither `CastBy` (one named
  player's casting) nor `CastFrom` (one origin's) can deny that ANY casting
  happened; [CR#111.1] and [CR#707.10] are why the question is real. 4
  tables.
- **`StaticEffect.EntersUnderInstead`** — the entry with its CONTROLLER
  replaced, a fixed-body row on `Redirects`' model rather than an entry
  written into `Intercepts`' body slot [CR#614.1d,614.12]. 2 tables.
- **`GameEvent.DealsDamage`** (+ `DamagePatient`/`patientIntro`, on
  `AttackDefender`/`defenderIntro`'s model) — the damage read from the
  SOURCE's side, in combat or out of it [CR#120.1]. NOT a flag on
  `DealsCombatDamage`, whose subject carries [CR#510.1]'s battlefield gate.
  It is the prospective producer `DamageDealing` had none of, and
  `interceptOk`'s stale "stated rather than reached" note is corrected. 4
  tables. **Family re-measured: 156 supported header lines.**
- **`Predicate.AttachedBy`** — the attachment with its ATTACHERS described,
  where `IsAttached` asks only whether the permanent is attached at all
  [CR#303.4,701.3a]. The count rides the attachers' own determiner, on
  `HappenedTo`'s policy. 7 tables. Family: 4 counted lines ("two or more",
  "three or more", "exactly two", "exactly one" Auras) plus ~6 described
  ones.
- **`CounterKind.Bloodstain`** — flat, on [CR#122.1]'s ordinary marker,
  exactly the bar `Blood`, `Luck` and `Suspect` cleared. 3 tables. Finding
  481's bar is met, not lowered.
- **`Intercepts` gains a `Maybe TriggerWindow` window slot** — "during your
  untap step" takes a SEAT and reuses the header's own window vocabulary;
  `windowOk`'s refusals (bare turn included) carry over untouched. 25 call
  sites in `Cards.idr`, 2 in `Macros.idr`, 2 in the pin modules updated to
  pass `Nothing`.
- **`reflexEncloseUse (OnlyIf e _ _) = reflexEncloseUse e`** — a postposed
  condition gates whether the enclosure's ONE action happens rather than
  adding a second for the pro-verb to abbreviate [CR#603.12], exactly as
  `May`'s unbranched offer already passed through.

### Benches — thirteen whole cards, two partial witnesses

Whole: **Hallowed Moonlight**, **Containment Priest**, **Fblthp, the Lost**,
**Heart of Yavimaya**, **Sheltered Valley**, **Mox Diamond**, **Gather
Specimens**, **Akki Lavarunner // Tok-Tok, Volcano Born**, **Bushi Tenderfoot
// Kenzo the Hardhearted**, **Frostwielder**, **Blood Spatter Analysis**,
**Bewitching Leechcraft**, **Bonus Round**.

Partial, each carrying its card's remaining blocker: `dontBlinkReplacement`
(Don't Blink's replacement without its written agent) and `kitsuneMysticFlip`
(Kitsune Mystic's flip trigger).

`fblthpTargeted` is retired into the whole card.

### Corrected premises (deviations from the umbrella)

- **"If you don't" ALREADY wrote.** `May` carries BOTH arms — `ifDid` and
  `ifNot` — and `Macros.mayThenElse` spells the pair. The umbrella's "That
  pair is the whole ask" and "NOTHING writes 'if you don't'" are wrong.
  What the land cycle actually needed was nothing at all on that axis: the
  three cycle members here write with `May Nothing …` (instructed) or
  `May (Just You) …` (offered), and the second arm is reachable because
  [CR#609.3] does only as much as possible. `Reflexively` is [CR#603.12]'s
  "when you do" trigger and is not the carrier of "if you do" here.
- **`eventUse` does not exist and `StatusChange` was never trigger-only.**
  The tree's table is `interceptOk`, whose only `False` is `ChapterArrival`;
  `StatusChange` and `Entry` were both interceptable already. Bewitching
  Leechcraft's cast-as-`TriggeredOnly` blocker was stale — the WINDOW seat
  was the whole of what it lacked.
- **Bushi Tenderfoot's by-source lookback already wrote**:
  `HappenedTo DamageTaken ThisTurn (Just (Involving Macros.thisCreature))`.
  Frostwielder writes the same phrase on the interception side; both bench.
- **Fblthp needed no new cast relation.** "was cast from your library" is
  `CastFrom`, which has always been agentless ([CR#601.2a] names the zone
  the card left without naming who moved it); the other disjunct is
  trigger-1's `Lookback.Triggering`. Only "it wasn't cast" needed `WasCast`.
- **The agentless "this way" reflexive node is already landed.** `ThisWay`
  ([CR#603.12]'s second template) is in `Effect.idr` with **Inferno of the
  Star Mounts** and **Matopi Golem** (a regenerate line) both benched. The
  routed ledger item is paid; nothing was built.
- **Sheltered Valley is not blocked on a sacrifice clause.** `Does`' labelled
  act writes it [CR#701.21a] and `Named (PrintedName …)` writes the
  name match [CR#201.2a]. Its one real blocker was the bare "each OTHER",
  which needs an anchor in a PROSPECTIVE body where nothing has been
  announced; `OtherThan Macros.thisLand` is the anchor and the card benches
  whole.
- **Land cycle re-measured: 7 lands + Mox Diamond = 8 cards** writing the
  conditional pair (Balduvian Trading Post, Heart of Yavimaya, Kjeldoran
  Outpost, Lake of the Dead, Lotus Vale, Scorched Ruins, Soldevi
  Excavations), not "9 cards plus Mox Diamond".
- **"would CAUSE X to Y" is 3 supported lines, not 4** (Rain of Gore,
  Unpredictable Cyclone, Silhouette).
- **Don't Blink's two recorded blockers are both gone.** `Enters` carries a
  source zone and `Intercepts` carries the arm list, so "from exile or after
  being cast from exile" writes as two arms ([CR#601.2a] puts a spell cast
  from exile on the stack, which is where it then enters from). What is
  left is the PLURAL possessor: "their owners" distributes over a counted
  group and `OwnerOf`/`ControllerOf` are both gated to `nounPlur n = OneOf`.

### Standing triggered abilities — re-measured, no shape needed

29 supported lines (25 "Until end of turn, whenever …", 4 "this turn,
when…"). [CR#603.7b] makes a delayed trigger fire repeatedly exactly when it
has a stated duration, and `Delayed`'s span slot already carries one, so the
standing ability IS the delayed one with its duration written. **Bonus Round
benches whole** on that reading. No ability shape was designed and none is
owed.

### The by-source rider — still waiting

Grep over `idris/src` and `crates/`: zero [CR#609.7] by-source
prevention-rider sites (the only hits are [CR#609.7a] cited as the
definition of a damage SOURCE, including this round's own use on
`DealsDamage`). The item stays recorded at its L1/OPEN-2 ledger tag; no
rider was built, and Cobra Trap's by-source lookback agent stays with the
causer complex below.

### The periphrastic causer complex — NOT landed, none of it

Each member was probed; two of the five are blocked on designs this round
should not improvise, and the pin is land-together-or-not-at-all.

- **"would CAUSE X to Y" (3 lines)** — needs a GameEvent row that WRAPS
  another event. `eventName` is total and name-keyed tables read through it,
  so a nesting row raises what a caused event's NAME is — the same question
  the umbrella routes to `workbench-event-algebra` for the crate's
  `CausePattern` channel. **Blocking.**
- **`Causer`-as-noun widening** — `Causer` is the nullary `AnEffect`
  (`Words.idr`), consumed by `CreationVoice`, `CausedBy`, `TokensCreated`
  and `CounterEvent`, and pinned by `badCausedCounterWithAgent`. Karmic
  Justice needs a described causer in `VerbedEvent`'s actor seat (which is
  `Maybe (Noun bs Player)` — an ability is not a player), Zabaz needs one in
  `CounterEvent`'s, and Cobra Trap needs one on a lookback complement.
  Widening it makes `Causer` Bindings-indexed across all four rows plus the
  pin. **Blocking.**
- **`Intercepts`' body VOICE slot (Zabaz)** — reachable, but its antecedent
  is "a modular triggered ability", so it cannot be witnessed without the
  widening above.
- **Modular** — `keywordFacts` has 38 rows and no `Modular`; beside it, "a
  modular triggered ability" needs a way to say "the triggered ability of
  keyword K", which neither `AbilityHead` (takes an `AbilityClass`) nor
  `AbilityOf` (takes an object source) says. Trigger-1's `AnyTriggered` is
  the half that exists.
- **The prospective "that would be created" predicate (Crafty Cutpurse)** —
  probed to the exact refusal: `TokenPhrase` admits `CountedGroup` and
  `Indefinite` only, and the line writes `Each`. Cheap on its own, and NOT
  landed, per the pin.

### Remainders

- Don't Blink — the plural possessor (`OwnerOf`/`ControllerOf`'s
  `nounPlur n = OneOf` gate); everything else in the line writes.
- Kitsune Mystic whole — its alternative half ("Attach target Aura attached
  to a creature to another creature") describes the target by what it is
  attached TO, the reverse of every attachment phrase this vocabulary
  writes (`AttachHost`, `IsAttached`, `AttachedBy` all read from the
  attachment's own side).
- Gather Specimens' sibling in the entry-as-body family: none. That family is
  1 supported line and it now benches.
- Bewitching Leechcraft's family is 2 lines. The second, **Freyalise's
  Winds**, wants "during ITS CONTROLLER's untap step" — a window possessor
  derived from the event's subject, which `Owner` does not have. NOT minted
  here: the window vocabulary is sub-round 2's lane.
- The causer complex, whole, with the two blocking members named above.
