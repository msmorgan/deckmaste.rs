---
needs: [semantics-v2-crate, lean-card-soundness-gate, lean-keyword-definition-bodies]
---
**Give every `plugins_v2/builtin/macros/stubs/keyword_abilities` declaration its semantic
`body`** (195 declarations, all bodyless today). Each body is a term-for-term
expansion over the v2 basis with no default slots, one macro per printed
phrase shape, invoking other macros where the CR definition does. The Lean
gate checks each body as it lands; Lean `Macros.lean` is the reference for
shapes already modelled there. Mechanical per declaration once the family's
shape is settled: terra tier. Standard constraints apply.

Ruling (user, 2026-09-06): each body is the keyword's definition, ported
from the v1 definitional macro under `plugins/builtin/macros/keyword/` where
one exists, carried in `Keyword(keyword: "<Label>", params: [...], body:
Some(<definition>))` under the widened law of `lean-keyword-definition-bodies`.
Keywords with no v1 body get theirs from the CR with its citation. v1 doc
comments and citations port with the body.

## Landing record

Partial. 85 of the 195 declarations are settled; 110 are untouched and the
`
## Landing record — second round (2026-09-07)

Still partial. Measured on change `knstvlyn` (`keyword abilities: champion
withdrawn to a STOP; the rules-machinery entries settle empty`), keyword facts
197 rows, `plugins_v2/canon` 83 cards.

### PROVE

- **Structural laws.** `cargo xtask lean-check plugins_v2/canon`: 83/83 cards
  prove `Card.check = []` — trunk's 23, the first round's 14, and this round's
  46. No card is recorded as a failure and none was weakened.
  `cargo test -p xtask --test plugins_v2_declarations`: 4 passed — every
  declaration reads on both sides. `cargo xtask facts check`: both generated
  tables up to date.
- **No silent loss.** One identity stopped being covered: **Champion**, whose
  definition the first round wrote, is withdrawn to a STOP (classified below as
  *wrong analysis retired* — the argument cannot reach the position the body
  needs). Nothing else lost coverage.
- **No word-naming.** No guard was added. The keyword labels in the declarations
  and in `crates/xtask/src/facts.rs` are the declaration tables;
  `lean/Semantics/Check/` is untouched.
- **Citations.** `cargo xtask cite check --list-noncompliant` empty;
  `cargo xtask cite check` 15,504 citations, 0 stale. 25 rules were new to
  `cr-citations.lock` and were blessed in one round; each was read against the
  claim citing it. One wrong-topic cite was caught and fixed before blessing:
  three canon cards cited equip's cross-reference subrule, which only points at
  the Equipment rules, instead of the subrule that quotes equip's expansion; all
  three now cite `[CR#702.6a]`.

### DISCLOSE

**Counts, over the family's 195 declarations.** 68 carry a written definition
(the first round's 46 less Champion, plus 23 written here), 10 are *settled
empty*, 51 are STOPs, 66 are untouched. The 66 untouched are named in the
handoff.

**Definitions written this round (23).** Embalm, Equip, Escape, Eternalize,
Evoke, Evolve, Exalted, Extort, Fabricate, Fading, Firebending, Flashback,
Fortify, Graft, Gravestorm, Impending, Increment, Kicker, LevelUp, LivingMetal,
Modular, Renown, Storm. Modular, Renown and Storm are
`lean/Semantics/Macros.lean`'s worked definitions translated
constructor-for-constructor; the handoff named them as the successor's first
easy wins and they are taken out of order for that reason.

**Settled empty (10), under the 2026-09-07 ruling.** A keyword whose [CR#702]
entry gives no expansion because the keyword IS rules machinery is settled, not
a STOP. The first round's eight — Banding, Deathtouch, DoubleStrike,
FirstStrike, Haste, Reach, Trample, Vigilance — are re-worded from `// STOP.` to
`// Settled empty.` with the ruling's date, and two more join them: **Enchant**
([CR#702.5a] states how the ability restricts an Aura's target and host) and
**Fuse** ([CR#702.102a,702.102b] state how the game treats casting a split card
with fuse). They are no longer counted as STOPs anywhere in this record.

**PRIORITY ONE — closing the unread-definition gap.** The first round left 32 of
its 46 definitions unread by the kernel. 30 now have a real canon card; the
kernel reads 43 of the 45 surviving first-round definitions and 17 of this
round's 23.

- *Still unread, with the reason.* **Decayed** — the keyword is printed only on
  created tokens, and tokens are not cards, so the corpus holds no card that
  invokes it. **Exploit** — every exploit card's second line is "When this
  creature exploits a creature, …", and `actFacts` names no exploiting deed, so
  the card cannot be written even though exploit's own body is fine.
- *Unread among this round's own definitions (6).* Fortify (every Fortification
  also grants indestructible to, or triggers off, the fortified land), Impending,
  Increment, Kicker (every kicker card reads its own "if this spell was kicked"
  clause), LevelUp (leveler cards need the `Leveler` card layout) and LivingMetal
  (printed only on transforming Vehicles). None of these has a corpus card whose
  remaining text is expressible today.

**Canon cards added (46).** Khenra Eternal, Accorder Paladin, Uktabi Efreet,
Barkhide Mauler, Enraged Revolutionary, Culling Drone, Greater Mossdog, Benalish
Cavalry, Barbarian General, Blade Instructor, Putrid Goblin, Frost Giant, Dauthi
Marauder, Deadly Insect, Furtive Homunculus, Kami of Empty Graves, Rimeshield
Frost Giant, Supreme Exemplar, Flayer Husk, Sylvok Battle-Chair, Lymph Sliver,
Virulent Sliver, Nyxborn Rollicker, Arcanum Wings, Warrior's Sword, Eldrazi
Conscription, Incarnation Technique, Ice Out, Farm-Market, Tah-Crop Skirmisher,
Proven Combatant, Glimpse of Freedom, Adaptive Snapjaw, Akrasan Squire,
Accomplished Automaton, Skyshroud Ridgeback, Syndic of Tithes, Think Twice, Ingot
Chewer, Loyal Fire Sage, Simic Initiate, Ominous Harvest, Trove Tracker, Arcbound
Worker, Knight of the Pilgrim's Road, Empty the Warrens. Every one is real oracle
text taken from `data/mtgjson/AtomicCards.json`; reminder text in parentheses is
dropped, as the first round's cards do. `Farm-Market` is the family's first
`Split` card — the file name cannot carry the printed `//`.

**What the kernel corrected (9 bodies).** Each was written, refused, and fixed;
none was rewritten to dodge a law.

1. **Flanking** — "the blocking creature" was `Described(The, blockerOf this)`
   and the kernel refused `uniquifying`: `Predicate.uniquifies` admits
   `inCombat .attackedBy` but not `.blockerOf`, because a creature can be blocked
   by several. The referent is the participant the trigger bound, so it now reads
   as a pronoun.
2. **LivingWeapon**, **ForMirrodin**, **JobSelect** — the attachment's subject
   was a bare `This`, whose `NounPhrase.zone` is `none`, and an attachment's
   subject must be on the battlefield (`zoneIs battlefield`). All three now write
   the printed "this Equipment", `AsType(Artifact, This, Equipment)`.
4. **Demonstrate** — the second "that copy" resolved against two copy bindings
   (`anaphor … 2`); its window is now `Top(depth: 1)`, the copy the chosen player
   just made.
5. **Bargain** — "an artifact, enchantment, or token" refused
   `parallelDisjuncts`: `IsToken` seeds the battlefield and the two `HasType`
   arms seed nothing, so the coordination was not parallel. Each type arm now
   carries the `InZone(battlefield)` the sacrifice presupposes.
6. **Exalted** — "exactly one creature is attacking" counted an `All`-determined
   group and refused `countableGroup`; the counted group is a bare plural.
7. **Extort** — "each opponent" was `EachOf(PlayerGroup(YourOpponents))` and
   refused `groupMention`: `EachOf` distributes over an already-named group, not
   over a fresh description. It is now `Described(Each, Opponent)`.
8. **Fading** — `ContinuationPolicy` has only `Optional` and `Required`; "if you
   can't" is `Required` with `if_not`.
9. **Champion** and **Encore** were corrected under kernel reading and then
   withdrawn to STOPs; see below.

**Champion: a definition withdrawn.** The first round's champion body forwards
its argument into a predicate position (`conjuncts: [Param(0), …]` inside
"another [object] you control"). The declaration's parameter kind is `Subject`,
which `crates/deckmaste_semantics_v2/src/ron.rs` maps to `NounPhrase`, so the
body could not expand at all — no card had ever invoked it. Retyping the
declaration to `Quality` made *Supreme Exemplar* prove and the kernel then
corrected two more errors in that body ("the exiled card" is a linked reference
[CR#607.2a], `Predicate::ExiledWith`, not a `Reach::Stamped` anaphor reaching
into a sibling ability; and "its owner" needed `Top(depth: 1)` to disambiguate).
But the `Subject` kind is **load-bearing**:
`deckmaste_english_v2`'s `keyword_lines` suite parses "Enchant creature you
control" through it, and that broke. So `Subject` is right and the body is
unwritable: champion is a STOP naming the mismatch, Enchant keeps `Subject` with
an unused parameter (the Infinity idiom), and the three cards that invoke either
keyword write the raw `Keyword(…, params: [Subject(predicate: …)], body: [])`
constructor, as *Kitsune Blademaster* already does for first strike. **This is
the one thing in this landing a reviewer should look at first**: the real defect
is that `KeywordParam::Subject` carries a `Predicate` while the macro-parameter
kind of the same name is a `NounPhrase`, and nothing converts one to the other.
Fixing it is a `crates/` change outside this ticket.

**New STOPs (20), each with what is missing.** Grouped by cause; each file
carries the full reason and its citation.

- *The definition qualifies the ability printed AFTER the keyword* (the cause
  Backup and Cleave share). Exhaust [CR#702.177a], Forecast [CR#702.57a].
- *`KeywordParam` has no `Ability` variant* (Boast's cause). Infinity
  [CR#702.186b], Gift [CR#702.174a,702.174b] (whose second ability is also not
  quoted).
- *No linked reference to the permanent an alternative cost consumed.* Emerge
  [CR#702.119a] and Harmonize [CR#702.180a] both reduce the cost by a property of
  the creature the sibling static's cost sacrificed or tapped;
  `Predicate::ExiledWith` is the only linked-reference predicate and a
  `Reach::Stamped` pronoun cannot read out of a sibling ability's cost.
- *No per-mana-symbol payment alternative* (convoke, delve, assist). Improvise
  [CR#702.126a].
- *A spell ability has no category.* Epic [CR#702.50a] (whose second half also
  creates a delayed triggered ability).
- *No constructor reaches a spell's own mode count.* Entwine [CR#702.42a],
  Escalate [CR#702.120a].
- *Miscellaneous, one each.* Champion (above) [CR#702.72a]; Encore
  [CR#702.141a] — written with `DoForEach` over each opponent and refused by the
  kernel: the loop binds no player the created copy's own added ability can name
  as "that opponent", and the attack requirement's patient is then unfit; Enlist
  [CR#702.154a,508.1g] — no `StaticSpec` attaches a cost to the attack
  declaration; Foretell [CR#702.143a,702.143b] — exiling face down is a special
  action, which nothing performs (Companion's cause); Freerunning [CR#702.173a] —
  no predicate for a commander; Frenzy [CR#702.68a] — no relation and no
  `GameEvent` says "attacks and isn't blocked", which fires in the declare
  blockers step; Haunt [CR#702.55a,702.55b] — no haunting link between an exiled
  card and a creature; HiddenAgenda [CR#702.106a] — conspiracy cards are outside
  this repo's Vintage-playable scope; Hideaway [CR#702.75a] — `actFacts` names no
  looking deed, so the ability the exiled card gains cannot be written;
  JumpStart [CR#702.133a] — `PlayPayment` offers the card's own cost, no cost, or
  a replacement cost, never its own cost PLUS a discard.

**Facts overlay.** `crates/xtask/src/facts.rs`: the `definition` column was set
for Embalm, Equip, Escape, Eternalize, Evoke, Fading, Flashback, Fortify, Graft,
Impending, Increment, Kicker, LevelUp and LivingMetal, and cleared for Champion,
Frenzy and Haunt (rows that became or stayed STOPs; Indestructible's cleared row
is the precedent). One non-category change: **Bargain gains `paid_cost: true`** —
"a spell is 'bargained' if its controller sacrificed a permanent as it was cast"
[CR#702.166b] is a cost a later clause reads back, and without the flag
`PaidCostName.named` refuses *Ice Out*'s "if it's bargained". Kicker's row already
carries it. `cargo xtask facts generate` was re-run; both generated tables are
committed (197 rows).

**Deviations and additions beyond the ticket's letter.**

- **A second `crates/` edit, to the same test file.** Three declarations had
  their `params` line clobbered while their STOP comments were written —
  Exhaust, Forecast (`[Ability]`) and Foretell (`[Cost]`) — and
  `builtin_v2_keyword_abilities.rs` caught all three. They are restored, not
  re-spelled. A fourth change to that file (moving Champion and Enchant from the
  `Subject` expectation list to `Quality`) was written and then **reverted** when
  english_v2's grammar showed the expectation was right; the file's only surviving
  change is still the first round's two re-spelled assertions.
- Modular, Renown and Storm are out of alphabetical order, taken from the Lean
  references as the handoff advised.
- The eight rules-machinery entries were re-worded in place under the
  2026-09-07 ruling; no body changed.

**Assurance counts.** Restored 3 declaration signatures that this round's own
edits had clobbered (named above); re-spelled 0 assertions; ignored-with-blockers
0; added 0 test functions (the 46 canon cards are the added evidence); removed 0.

**A regression this round did not cause, and did not fix.**
`cargo test -p deckmaste_plugin --test read_api_gate` fails:
`typed_card_reads_go_through_the_restricted_api` names
`crates/xtask/tests/plugins_v2_declarations.rs:175` (`let written_out: Card =
…read_str(…)`). That line is present at this feature's claim commit
(`qkvztmpvsxks`) and this feature never touched the file; it came in with the
`plugins-v2-invocation-kinds` landing. Reported, not worked around, and not
deleted. It needs its own ticket against that landing.

**Glossary gaps.** None.

### REPORT

Provenance, not a gate; measured on `knstvlyn`, 83 canon cards, keyword facts
197 rows.

- `cargo xtask gate --changed` derives
  `cargo test -p deckmaste_construction_core -p deckmaste_construction -p
  deckmaste_core -p deckmaste_card -p deckmaste_english_v2 -p deckmaste_semantics
  -p deckmaste_lowering -p deckmaste_plugin -p deckmaste_engine -p
  deckmaste_legacy_render -p deckmaste_migrations -p deckmaste_noncanon -p
  deckmaste_semantics_v2 -p deckmaste_spelling -p deckmaste_tui -p deckmaste -p
  xtask`. Run without `-p deckmaste_plugin` (the pre-existing red crate above):
  93 suites reporting ok, 0 failed, 1 ignored (pre-existing). The closure is the
  whole workspace because `plugins_v2/builtin` data feeds
  `deckmaste_construction_core`.
- `cargo xtask lean-check plugins_v2/canon`: 83/83, 0.7s warm on an unchanged
  tree, 15–41s after a canon or declaration change, on a host also running this
  session's cargo builds. The command is not instrumented per byte.
- `cargo fmt --all` leaves no changes; `cargo xtask facts check` clean;
  `cargo xtask cite check` 15,504 citations, 0 stale.

## Landing record — third round (2026-09-07)

Complete. The second round's handoff is discharged: every one of the family's
195 declarations now carries a written definition, a settled-empty body, or a
STOP with its reason and citation. Measured on change `kztlkuwy` (`keyword
abilities: bless the round's new CR citations`), keyword facts 197 rows,
`plugins_v2/canon` 118 cards.

### PROVE

- **Structural laws.** `cargo xtask lean-check plugins_v2/canon`: 118/118 cards
  prove `Card.check = []` — the 90 this round inherited plus the 28 it added. No
  card is recorded as a failure and none was weakened.
  `cargo test -p xtask --test plugins_v2_declarations`: 5 passed — every
  declaration reads on both sides. `cargo xtask facts check`: both generated
  tables up to date.
- **No silent loss.** No identity stopped being covered by a definition that had
  one at the start of this round. Three definitions this round *wrote* were then
  withdrawn to STOPs when the kernel refused them (Offspring, Protection,
  Ripple, all detailed below); each is classified *wrong analysis retired* —
  none had ever been read by a card before it was withdrawn, so nothing lost
  coverage. Nothing else changed classification.
- **No word-naming.** No guard was added. The keyword labels in the declarations
  and in `crates/xtask/src/facts.rs` are the declaration tables;
  `lean/Semantics/Check/` is untouched.
- **Citations.** `cargo xtask cite check --list-noncompliant` empty;
  `cargo xtask cite check` 15,705 citations, 0 stale. 57 rules were new to
  `cr-citations.lock` and were blessed; each was read against the claim citing
  it with `jj diff --git | cargo xtask cite audit --diff` (112 citation sites
  audited over this round's diff). One wrong-topic cite was caught and fixed
  before blessing: Visit cited `[CR#717.2a]` (an Attraction deck-construction
  minimum) for the claim that nothing reads an Attraction's lit-up numbers; it
  now cites `[CR#717.1,701.52a]`, which are the rules that speak about them.

### DISCLOSE

**Counts, over the family's 195 declarations.** 95 carry a written definition,
13 are *settled empty*, 87 are STOPs, 0 are untouched. Against the second
round's 68 / 10 / 51 / 66, this round wrote 27, settled 3 empty and stopped 36 —
66 declarations, exactly the handoff's list.

**Definitions written this round (27).** Madness, Mayhem, Melee, Mobilize,
Myriad, Ninjutsu, Outlast, Provoke, Ravenous, Reconfigure, Recover, Reinforce,
Retrace, Scavenge, Spectacle, SplitSecond, Station, Sunburst, Surge, Training,
Transfigure, Transmute, UmbraArmor, Undaunted, Unearth, Vanishing, WebSlinging.

Two of these needed constructors the family had not used before, and both are
worth a reviewer's eye. **Madness** writes "when this card is exiled this way"
as `GameEvent::Causes(cause: Event(the discard), event: the hand-to-exile
zone change)` — the first use of `Causes` anywhere in `plugins_v2` — and its
"its owner may cast it" is an `Instruction::Establish` of a `DeonticRule` with a
`Play(payment: PayingInstead(…))` rider, the idiom Lean's `flawlessForgeryLine`
and `memoryPlunder` already use for a permitted cast. **SplitSecond** is the
family's first `DeonticPatient::CounterpartsAt`: one prohibition over two deeds
(`Cast`, `Activate`) with a different patient at each. **Retrace** is the first
use of `Cost::ItsManaCost`, which is how "by discarding a land card as an
additional cost to cast it" rides on the permitted cast rather than becoming a
separate `AddedCost` that would apply to every cast.

**Settled empty (3), under the 2026-09-07 ruling.** **Phasing**
([CR#702.26a..702.26i] state when permanents phase out and in and how the game
treats a phased-out permanent), **SpaceSculptor** ([CR#702.158a,702.158c] state
the state-based sector assignment) and **StartYourEngines**
([CR#702.179a,704.5aa] state the state-based action that sets a player's speed
to 1). Each entry defines only how the game treats the keyword; none quotes an
expansion.

**PRIORITY ONE — every new definition is read by the kernel except one.** 26 of
the 27 have a real canon card that invokes them and proves. The exception is
**Station**: every printed station card uses the [CR#721] station-card layout,
whose station symbols are themselves keyword abilities, and `Card` has no
variant for that layout, so the corpus holds no card the workbench can write
today. Its body is written and reads through both readers, but no card exercises
it.

**Canon cards added (28).** Arrogant Wurm, Spider-Islanders, Menagerie
Liberator, Dalkovan Packbeasts, Wyrm's Crossing Patrol, Dokuchi Shadow-Walker,
Salt Road Patrol, Abbey Gargoyles, Brontotherium, Ravener, Leech Gauntlet, Grim
Harvest, Burrenton Bombardier, Cenn's Enlistment, Surging Sentinels, Drudge
Beetle, Dead Revels, Krosan Grip, Suntouched Myr, Jwar Isle Avenger, Gryff
Rider, Fleshwrither, Drift of Phantasms, Boar Umbra, Sublime Exhalation,
Dregscape Zombie, Waning Wurm, Spider-Man Web-Slinger. Every one is real oracle
text taken from `data/mtgjson/AtomicCards.json`; reminder text in parentheses is
dropped, as the earlier rounds' cards do. Two of them (Abbey Gargoyles for
protection, Surging Sentinels for ripple) invoke a keyword whose definition was
withdrawn to a STOP; they are kept because they are the only cards in the canon
set that exercise those two keywords' `Quality` and `Amount` arguments.

**What the kernel corrected (6 bodies).** Each was written, refused, and either
fixed or withdrawn; none was rewritten to dodge a law.

1. **Reconfigure** — "another target creature you control" used
   `Predicate::Other`, which `PhraseRules` admits only when a target of that kind
   is already bound (`[Semantics.Refusal.anyTargeted (Semantics.Kind.object),
   Semantics.Refusal.otherAnchored]`); inside the very description that
   introduces the target there is none. It now reads `OtherThan(anchor: this
   permanent)`, which is what "another" means here.
2. **Transfigure** — the "it" that goes to the battlefield after the search
   resolved against two antecedents (`anaphor (Reach.bare) (Plurality.one) 2`);
   its window is now `Top(depth: 1)`.
3. **Transmute** — the same, twice, for the "that card" it reveals and the "it"
   it puts into hand.
4. **Offspring** — withdrawn to a STOP. Both halves write, but
   `keywordBodyPartFits` requires every TRIGGERED part of a definition to key on
   the keyword's own stack regime, and offspring's registry row is
   `regime: AtCasting` because its first ability functions on the stack; the
   second is an enters trigger, whose `bodyEventRegime` is `none`. `Intrepid
   Rabbit` refused with `[Semantics.Refusal.keywordBodyFits "Offspring"]`.
   Clearing the row's regime to satisfy the second ability would misdescribe the
   first, so the definition waits on the law. Squad has the same shape (and is
   separately blocked).
5. **Protection** — withdrawn to a STOP after three of its four rows proved.
   `Abbey Gargoyles` refused with `[Semantics.Refusal.deedFits,
   Semantics.Refusal.deonticPatientOk]`; removing only the [CR#702.16c,702.16d]
   row ("can't be enchanted by Auras … equipped by Equipment … fortified by
   Fortifications that have the stated quality") made the card prove, which
   isolates the cause: the `Attach` deed's registry row gives it an agent role of
   `playerAgent` and no patient role at all, so `deedFits` refuses every noun
   phrase in the patient slot. This is the shape Indestructible's `Destroy`
   patient role already carries. A body with three of four rows would be a wrong
   analysis, so none is written.
6. **Ripple** — withdrawn to a STOP. The trigger, the reveal and the "you may
   cast any of those cards with the same name as this spell" permission all
   write; the final "then put all revealed cards not cast this way on the bottom
   of your library" has no subject. `TheRest` refused with
   `[Semantics.Refusal.theRestFits (Semantics.Kind.object)]` — the `SomeOf`
   partition the permission takes lives inside a `StaticSpec`'s patient and never
   reaches the next step's bindings, so `partsTaken` is zero — and naming the
   complement directly is blocked because "revealed this way" would be a
   `VerbedEvent` over the `Reveal` deed, whose registry row has no patient role
   (the same column gap as protection's `Attach`).

**New STOPs (36), each with what is missing.** Grouped by cause; each file
carries the full reason and its citation.

- *`KeywordParam` has no `Ability` variant, and nothing lets a definition qualify
  the ability printed after the keyword* (the causes Boast and Backup carry).
  MaxSpeed [CR#702.178a] (also needs a player's speed, which no `Amount`
  projects), PowerUp (also has no [CR#702] entry in the CR this repo ships),
  Solved [CR#702.169b..702.169d], Visit [CR#702.159a].
- *`PlayPayment` has no clause substituting the cast object's characteristics*
  (Disguise's cause). Morph [CR#702.37a], MoreThanMeetsTheEye [CR#702.162a].
- *A text-changing effect over printed words* (Cleave's cause). Overload
  [CR#702.96a,702.96c], Splice [CR#702.47a,702.47c].
- *A static ability that creates a delayed triggered ability* (Blitz and Dash's
  cause). Rebound [CR#702.88a], Warp [CR#702.185a].
- *A special action* (Companion and Foretell's cause). Plot [CR#702.170a,702.170b],
  Suspend [CR#702.62a].
- *Spell abilities have no category* (Epic's cause). Paradigm [CR#702.192a].
- *No aggregate bound over a whole group's stat* (Crew's cause). Saddle
  [CR#702.171a], Teamwork [CR#702.194a].
- *`StaticSpec::AddedCost` has no repetition count and no per-mode multiplier*
  (Escalate's cause). Replicate [CR#702.56a], Squad [CR#702.157a], Spree
  [CR#702.172a], Tiered [CR#702.183a].
- *No entry rider is optional* — `TokenRider::AsCopyOf` alone carries an
  `optional` flag. Riot [CR#702.136a], Tribute [CR#702.104a] (whose rider is also
  decided by a player other than the controller), Unleash [CR#702.98a].
- *No linked reference to the permanent an alternative cost consumed* (Emerge and
  Harmonize's cause). Offering [CR#702.48a].
- *No predicate reads "shares a creature type with <subject>"* (Amplify's cause).
  Prowl [CR#702.76a].
- *`PlayTiming` has no value for the window* (Flash's cause). Sneak [CR#702.190a].
- *A damage-RESULT substitution, which lives in `rules/damage`* (Infect's cause).
  Toxic [CR#702.164a,702.164c,120.3] — its [CR#702] entry quotes no expansion, but
  it is the shape Infect already carries rather than the rules-machinery shape the
  2026-09-07 ruling settles empty. Poisonous, which the CR does spell out as a
  triggered ability [CR#702.70a], stays written.
- *No `StaticSpec` performs an instruction when a state holds* (Ascend's cause).
  Storied [CR#702.195a].
- *Miscellaneous, one each.* Miracle [CR#702.94a] — the reveal happens AS YOU DRAW
  the card, and `DeonticRule` carries no timing at all; writing it as a
  `Replacement` of the draw would make miracle a replacement effect, which it is
  not. Mutate [CR#702.140a] — nothing gives a spell already on the stack a target,
  and no predicate names a mutating creature spell. Offspring, Protection and
  Ripple, above. Partner [CR#702.124a,702.124h] — nothing designates a commander
  or speaks about the period before the game. Prototype [CR#702.160a,718.1..718.3]
  — a layout keyword the workbench spells as the `Card::Prototype` wrapper, whose
  `[Cost, Power, Toughness]` signature `Shape::from_params` does not admit, so it
  carries no registry row and `keywordDefinition "Prototype"` is empty for every
  category. ReadAhead [CR#702.155a] — the `unless` clause reads a lore-counter
  number off the very chapter ability the prohibition ranges over, and no
  `Predicate` projects that. Soulbond [CR#702.95a,702.95b] — nothing expresses
  pairing: there is no paired designation and no instruction pairs two objects.

**Facts overlay.** `crates/xtask/src/facts.rs`: the `definition` column was set
for Madness, Mayhem, Ninjutsu, Outlast, Ravenous, Reconfigure, Reinforce,
Retrace, Scavenge, Spectacle, SplitSecond, Station, Sunburst, Surge,
Transfigure, Transmute, UmbraArmor, Undaunted, Unearth, Vanishing and
WebSlinging, and cleared for **Ripple** (which had carried `[Triggered]` from an
earlier round and is now a STOP). Offspring's and Protection's columns were set
while their bodies were written and removed again when the bodies were
withdrawn, so the net diff shows neither. Melee, Mobilize, Myriad, Provoke,
Recover and Training already carried the right column. No other column changed —
in particular no `regime`, `functions_on_stack` or `paid_cost` flag was touched.
`cargo xtask facts generate` was re-run; both generated tables are committed
(197 rows).

**Deviations and additions beyond the ticket's letter.**

- **24 declarations gained an explicit `params: []` line**: Melee, Myriad,
  Paradigm, Partner, Phasing, Provoke, Ravenous, ReadAhead, Rebound, Retrace,
  Riot, Soulbond, SpaceSculptor, SplitSecond, Spree, StartYourEngines, Station,
  Storied, Sunburst, Tiered, Training, UmbraArmor, Undaunted, Unleash. This is
  not the shape-decision-6 violation it looks like: `cargo xtask facts generate`
  refuses a declaration that carries a semantic body without an explicit
  positional signature ("use `params: []` for a nullary declaration"), and
  `Shape::from_params` maps both an absent list and `[]` to `Shape::Nothing`, so
  no row's `argumentSchemas` changed. No declaration's existing kind list was
  altered, which `deckmaste_construction_core`'s `builtin_v2_keyword_abilities`
  suite confirms.
- **Two already-landed bodies repaired.** `Instruction::RemoveCounters` takes
  `quantity: Option<Quantity>`, not `amount`; **Fading** and **Impending** both
  spelled it `amount: Lit(value: 1)`, and the reader drops unknown fields
  silently, so each read as "remove Fade/Time counters" with no count at all.
  Both now write `quantity: Range(low: 1, high: 1)`. Fading is read by a canon
  card and still proves. This is the same silent-drop hazard as the `conferral`
  argument the refresh removed from `gainDesignation`: **Renown.ron still passes
  `conferral: ByKeyword(keyword: "Renown")`**, which the reader drops and the
  emitted Lean does not carry, so it is inert rather than wrong; it is left for a
  reviewer to decide whether to prune, because pruning it changes no behaviour.
- Two canon cards carry a second ability written out beyond the keyword under
  test, because the corpus offers no simpler card: Ravener (ravenous) and Leech
  Gauntlet (reconfigure). Boar Umbra writes the raw `Keyword("Enchant", …)`
  constructor, as the second round's cards already do.
- No `lean/` file was edited by hand; `lean/Semantics/Check/Facts.lean` is
  regenerated output. No `crates/` file was edited but `crates/xtask/src/facts.rs`.

**Assurance counts.** Restored 0; re-spelled 2 already-landed bodies (Fading and
Impending, named above — same subject, same asserted outcome, corrected field
name); ignored-with-blockers 0; added 0 test functions (the 28 canon cards are
the added evidence); removed 0.

**A regression this round did not cause, and did not fix.** The second round's
report of `cargo test -p deckmaste_plugin --test read_api_gate` failing stands;
`deckmaste_plugin` is not in this round's derived closure, so it was not re-run.
It still needs its own ticket against the `plugins-v2-invocation-kinds` landing.

**Glossary gaps.** None.

### REPORT

Provenance, not a gate; measured on `kztlkuwy`, 118 canon cards, keyword facts
197 rows.

- `cargo xtask gate --changed` derives
  `cargo test -p deckmaste_construction_core -p deckmaste_construction -p
  deckmaste_english_v2 -p deckmaste_semantics_v2 -p xtask`. Run: 45 suites
  reporting ok, 1,532 tests passed, 0 failed, 1 ignored (pre-existing). The
  closure is narrower than the second round's whole-workspace line because
  `--from` defaults to `default@`, which has since advanced.
- `cargo xtask lean-check plugins_v2/canon`: 118/118, 0.7 s warm on an unchanged
  tree, 43–50 s after a canon or declaration change, on a host also running this
  session's cargo builds. The command is not instrumented per byte.
- `cargo fmt --all` leaves no changes; `cargo xtask facts check` clean;
  `cargo xtask cite check` 15,705 citations, 0 stale;
  `cargo xtask cite check --list-noncompliant` 0.

**Live items for the coordinator to route.** The second round's four stand, and
this round adds three.

- The `Subject` parameter-kind mismatch that stops Champion:
  `KeywordParam::Subject` carries a `Predicate`, the macro-parameter kind
  `Subject` is a `NounPhrase`. Wants a `crates/` ticket.
- **Deed role columns.** Three keywords now stop on the same gap, and it is one
  ticket, not three: the `Destroy` deed's `patientRole` is `fieldObject` with an
  empty type list (Indestructible), the `Attach` deed has NO patient role
  (Protection's [CR#702.16c,702.16d] row), and the `Reveal` deed has no patient
  role either (Ripple's complement, and any future "revealed this way"). All
  three live in `crates/xtask/src/facts/action_overlay.rs`, which the macro-body
  tickets' overlay authority does not cover.
- **`keywordBodyPartFits` and a keyword whose abilities function in different
  zones.** Offspring and Squad each pair a stack-functioning static with a
  battlefield enters trigger; the law ties every triggered body part to the
  keyword-level `regime`. **Impending's landed body has the same latent
  mismatch** (`regime: AtCasting` with an end-step trigger) and is one of the two
  first-round definitions the kernel still has not read, so it will refuse the
  first card that invokes it. Wants a `lean/` ticket.
- **`TheRest` after a partition taken inside a `StaticSpec`.** Ripple's cause;
  `partsTaken` does not see a `SomeOf` that lives in a `DeonticRule` patient.
- `deckmaste_plugin --test read_api_gate` is red on the default line, from the
  `plugins-v2-invocation-kinds` landing.
- Decayed and Exploit remain the two first-round definitions the kernel has not
  read, and neither can get a card today. **Station** joins them, for a different
  reason: the [CR#721] station-card layout has no `Card` variant.
- **Renown.ron's inert `conferral` argument** (above), if a reviewer wants it
  pruned.
