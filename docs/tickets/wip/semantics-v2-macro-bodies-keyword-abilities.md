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

## Handoff

**Resume at `Madness`** — the alphabetically first untouched declaration. 66
remain untouched: Madness, MaxSpeed, Mayhem, Melee, Miracle, Mobilize,
MoreThanMeetsTheEye, Morph, Mutate, Myriad, Ninjutsu, Offering, Offspring,
Outlast, Overload, Paradigm, Partner, Phasing, Plot, PowerUp, Protection,
Prototype, Provoke, Prowl, Ravenous, ReadAhead, Rebound, Reconfigure, Recover,
Reinforce, Replicate, Retrace, Riot, Ripple, Saddle, Scavenge, Sneak, Solved,
Soulbond, SpaceSculptor, Spectacle, Splice, SplitSecond, Spree, Squad,
StartYourEngines, Station, Storied, Sunburst, Surge, Suspend, Teamwork, Tiered,
Toxic, Training, Transfigure, Transmute, Tribute, UmbraArmor, Undaunted, Unearth,
Unleash, Vanishing, Visit, Warp, WebSlinging. (`Protection` was missing from the
first round's list of 110 and is untouched too.)

**Shape decisions, added to the first round's five.**

6. **Never touch a declaration's `params` line while writing its body.** The kind
   list is load-bearing three ways at once: it types `Param(i)` for macro
   expansion, it becomes the keyword facts row's `argumentSchemas` (which
   `keywordParamFits` checks the card's `KeywordParam` against), and it drives
   `deckmaste_english_v2`'s keyword-line grammar. `builtin_v2_keyword_abilities.rs`
   holds the expected list for all 106 parameterized keywords — read it before
   changing one, and treat a mismatch as a STOP, not as a licence to edit the
   expectation.
7. **A STOP with an argument still declares it.** Write
   `params: [<kind>]` on the declaration and either forward it
   (`Cost(cost: Param(0))`) or leave the keyword's own `params: []` when the kind
   has no matching `KeywordParam` (the Infinity idiom). A STOP's `definition`
   column in `crates/xtask/src/facts.rs` is cleared.
8. **Anaphora idioms the kernel taught this round.** A definite description needs
   a `uniquifies` predicate — otherwise the referent is a pronoun. A pronoun with
   two candidate antecedents narrows with `window: Top(depth: 1)`, not with a
   different reach. "The exiled card" is `Predicate::ExiledWith(source: This)`, a
   linked reference [CR#607.2a] — a `Reach::Stamped` pronoun cannot cross an
   ability boundary. "Each opponent" is `Described(Each, Opponent)`, never
   `EachOf`. A counted group is a bare plural. An attachment's subject must name
   a type ("this Equipment"), because bare `This` has no zone. Coordinated
   disjuncts must seed the same zone.
9. **`cargo test -p xtask --test plugins_v2_declarations` does not expand
   macros.** It catches unknown field names but not unknown *variant* names
   inside a body (`IfAble`, `TheNext` both passed it). Only `lean-check` on a
   canon card that invokes the keyword finds those, which is another reason to
   add the card in the same round as the body.

**Live items for the coordinator to route.**

- The `Subject` parameter-kind mismatch that stops Champion (and would stop any
  later `Subject` keyword's body): `KeywordParam::Subject` carries a `Predicate`,
  the macro-parameter kind `Subject` is a `NounPhrase`. Wants a `crates/` ticket.
- The `Destroy` deed's `patientRole` (`fieldObject`), which still blocks
  Indestructible and any later "can't be destroyed" body — carried over from the
  first round, still unrouted.
- `deckmaste_plugin --test read_api_gate` is red on the default line, from the
  `plugins-v2-invocation-kinds` landing (details in the second-round record).
- Decayed and Exploit are the last two first-round definitions the kernel has not
  read, and neither can get a card today (reasons in the record).
