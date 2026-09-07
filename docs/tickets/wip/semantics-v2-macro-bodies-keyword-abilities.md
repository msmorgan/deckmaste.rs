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
`## Handoff` below names where a successor resumes. Numbers are measured on
change `ttpyynot` (`keyword abilities: re-spell the nursery test against bodied
declarations`), keyword facts 196 rows.

### PROVE

- **Structural laws.** `cargo xtask lean-check plugins_v2/canon` proves 37/37
  cards, trunk's 23 among them; no card was weakened and none is recorded as a
  failure. `cargo test -p xtask --test plugins_v2_declarations`: 4 passed —
  every declaration reads on both sides and the two readings name the same
  identities. `cargo xtask facts check`: both generated tables up to date.
- **No silent loss.** Nothing stopped being covered. The one test whose subject
  this ticket retires is re-spelled, not deleted (see *Assurance* below).
- **No word-naming.** No guard was added anywhere. The keyword labels written in
  the declarations and in `crates/xtask/src/facts.rs` are the declaration
  tables, which is what those files are; `lean/Semantics/Check/` is untouched.
- **Citations.** `cargo xtask cite check --list-noncompliant` is empty;
  `cargo xtask cite check` reports 15,347 citations, 0 stale. 45 rules were new
  to `cr-citations.lock` and were blessed in four rounds; each was read against
  the claim citing it (every one is the [CR#702] entry of the keyword whose file
  cites it, or a [CR#702] subrule this record's STOP text quotes).

### DISCLOSE

**The shape.** Every declaration handled gets
`body: Keyword(keyword: "<Label>", params: [...], body: [...])` and, where it
had none, `params: [...]` — `deckmaste_construction_core` refuses a body without
a signature (`ValidationError::BodyWithoutSignature`), so the two land together.
A keyword that takes an argument forwards it as the `KeywordParam` its registry
shape names (`Number(amount: Param(0))`, `Cost(cost: Param(0))`,
`Quality(predicate: Param(0))`, `Subject(predicate: Param(0))`). Bodies are
written in raw v2 constructors, because `plugins_v2/builtin/macros/` declares no
phrase macros; the one macro invocation used is `Flying`, inside afterlife's
token, which is a keyword-ability declaration invoked at its `Ability` position.

**Counts.** 46 declarations carry a written definition, 39 carry the label with
`body: []` and a STOP comment naming what is missing, 110 are untouched.

**Definitions written (46).** Absorb, Affinity, Afflict, Afterlife, Aftermath,
Annihilator, AuraSwap, Bargain, BattleCry, Bestow, Bloodthirst, Bushido,
Buyback, Champion, Changeling, CumulativeUpkeep, Cycling, Decayed, Defender,
Demonstrate, Dethrone, Devoid, Dredge, Exploit, Fear, Flanking, Flying,
ForMirrodin, Hexproof, Horsemanship, Ingest, JobSelect, Landwalk, LivingWeapon,
Menace, Mentor, Persist, Poisonous, Prowess, Rampage, Shadow, Shroud, Skulk,
Soulshift, Undying, Ward.

Flying, Bushido, Cycling, CumulativeUpkeep are the Lean `Macros.lean` worked
definitions translated constructor-for-constructor; modular, renown and storm
are among the untouched 110 and are the successor's first easy wins.

**Kernel-proved by a canon card (14 of the 46).** Flying (*Wind Drake*),
Affinity (*Frogmite*), Afterlife (*Ministrant of Obligation*), Bushido (*Kitsune
Blademaster*), Changeling (*Woodland Changeling*), Bloodthirst (*Bloodrage
Vampire*), Buyback (*Capsize*), Defender (*Wall of Wood*), Menace (*Boggart
Brute*), Hexproof (*Gladecover Scout*), Landwalk (*Bog Wraith*), Fear (*Dross
Prowler*), Prowess (*Monastery Swiftspear*), Undying (*Young Wolf*). The other
32 definitions load and expand but no card invokes them, so the kernel has not
read them; that is the gap a successor closes first, and four bodies were
corrected by exactly this route (below).

**Canon cards added (14).** Wind Drake, Frogmite, Ministrant of Obligation,
Kitsune Blademaster, Woodland Changeling, Bloodrage Vampire, Capsize, Wall of
Wood, Boggart Brute, Gladecover Scout, Bog Wraith, Dross Prowler, Monastery
Swiftspear, Young Wolf. Every one is real oracle text checked with the
`mtg-rules` `card` script. *Darksteel Myr* was written and then removed when
indestructible became a STOP (below).

**What the kernel corrected.** Four bodies were wrong as first written and the
gate said so: bushido's toughness half read its subject back as a bare pronoun
(`anaphor`) and now repeats the noun; buyback's and Capsize's zone-change
destinations were possessed zones (`destOk` — a move destination is a bare zone,
`ZoneExpr.destOk`); changeling's subject was bare `This`, and
`CharacteristicEdit.fits` requires the subject to carry the type the
`SubtypeSpace` is hosted on (`becomesOk`); landwalk's "if there is a land"
existential used `a` where `Condition.exists_` requires a bare-determiner
mention (`existentialMention`).

**A dialect finding: `r#Some`.** `PreventCut::Some` (absorb's "prevent N of that
damage") cannot be written as `Some(...)` — RON's `IMPLICIT_SOME` extension
claims the bare spelling for `Option`, and the raw-value scanner fails with
`ExpectedRawValue`. The escaped identifier `r#Some(amount: …)` reads correctly.
This is recorded in `Absorb.ron`'s comment; every other v2 variant whose name
collides with a serde builtin will need the same.

**STOPs (39), each with what is missing.** Grouped by cause; each file carries
the full reason and its citation.

- *A spell ability has no category.* `Ability.category` is `none` for
  `Ability::Spell`, so `keywordBodyFits` refuses one in a definition: Ascend's
  instant/sorcery form [CR#702.131a], Awaken [CR#702.113a], Cipher [CR#702.99a].
- *No `StaticSpec` creates a delayed triggered ability.* Blitz [CR#702.152a],
  Dash [CR#702.109a].
- *No `StaticSpec` performs an instruction when a state holds.* Ascend's
  permanent form [CR#702.131b]; `DeonticRider::StateBased`'s `StateBasedCause`
  has the single value `NonpositiveLife`.
- *No "shares a <quality> with <subject>" predicate.* Amplify [CR#702.38a],
  Conspire [CR#702.78a], Intimidate [CR#702.13b].
- *No per-mana-symbol payment alternative.* Convoke [CR#702.51a], Delve
  [CR#702.66a], Assist [CR#702.132a].
- *Damage-result substitution is the `rules/damage` table, not a `StaticSpec`.*
  Infect [CR#702.90b], Lifelink [CR#702.15b], Wither [CR#702.80a].
- *The CR entry quotes no expansion* (it states rules instead). Banding
  [CR#702.22a], Deathtouch [CR#702.2a], DoubleStrike [CR#702.4a], FirstStrike
  [CR#702.7a], Haste [CR#702.10a], Trample [CR#702.19a], Vigilance [CR#702.20b],
  Reach [CR#702.17b] (reach's content lives inside flying's restriction).
- *`KeywordParam` has no `Ability` variant.* Boast [CR#702.142a]; its definition
  also has to read the cost and effect out of that argument to add the guard and
  the limit.
- *Miscellaneous, one each.* Backup (no predicate for printed position)
  [CR#702.165a]; Cascade ("the resulting spell" has no noun phrase)
  [CR#702.85a]; Casualty (no condition for "has any targets") [CR#702.153a];
  Cleave (no text edit by printed brackets) [CR#702.148a]; Companion (outside
  the game, a special action; and its `Condition` parameter cannot fill
  `KeywordParam::DeckCondition`) [CR#702.139a]; Compleated (no amount projects
  life paid for Phyrexian symbols) [CR#702.150a]; Craft (the materials argument
  is not in the declaration's signature) [CR#702.167a]; Crew (no group selected
  by an aggregate bound) [CR#702.122a]; Daybound and Nightbound (a static acting
  on a designation change, plus an except-this-ability prohibition)
  [CR#702.145b]; Devour (no entry clause performs an instruction) [CR#702.82a];
  Disguise (no substituted cast characteristics) [CR#702.168a]; Disturb (no
  "cast transformed" play rider) [CR#702.146a]; Echo (no `Lookback` for "since
  your last upkeep") [CR#702.30a]; Flash (no `PlayTiming` for "any time you
  could cast an instant") [CR#702.8a].
- *A deed-facts row, not the grammar.* **Indestructible** [CR#702.12b]. "Can't
  be destroyed" is a deontic rule over the `Destroy` deed in the patient role,
  and that deed's `patientRole` is `fieldObject` — `bare := false` with an empty
  type list — so `deedFits` refuses *every* noun phrase in that role and no
  subject can be written. This looks like a defect in the `actFacts` role
  columns rather than a missing constructor; fixing it is outside this ticket's
  overlay authority, which covers the `definition` column alone.

**Facts overlay.** `crates/xtask/src/facts.rs`'s `definition` column now
declares the categories of each definition written: 44 rows changed or added
(Static for the statics, Activated for aura swap and cycling, and per-ability
lists for the multi-ability definitions — Buyback `[Static, Static]`, Champion
`[Triggered, Triggered]`, Decayed `[Static, Triggered]`, Shadow
`[Static, Static]`). One row was cleared again: Indestructible, when it became a
STOP. `cargo xtask facts generate` was re-run and both generated tables are
committed. Three single-line `Row { … }` literals were expanded to the
multi-line form so the column could be added uniformly; `cargo fmt` keeps them.

**Deviations and additions beyond the ticket's letter.**

- **The brief said not to edit `crates/` outside `crates/xtask/src/facts/`, and
  one test file was edited.**
  `crates/deckmaste_construction_core/tests/builtin_v2_keyword_abilities.rs`
  asserted `!declaration.is_graduated()` — "every keyword ability must remain a
  nursery declaration" — which this ticket's whole purpose falsifies, and
  asserted `params().is_none()` for paramless keywords, which the
  body-needs-a-signature rule falsifies too. Both were **re-spelled, not
  deleted**, per the assurance rule: the first now asserts that graduation and a
  written body are the same thing (a body never lands without its signature),
  the second that a paramless keyword's signature is absent *or empty*. This is
  disclosed as a deviation rather than resolved silently; a reviewer who wants
  the brief's letter should revert the two assertions and re-open the question.
- The `definition` column was set on rows whose CR entry defines several
  abilities of the same category as a repeated list (`[Triggered, Triggered]`)
  rather than the singleton the migration produced. That keeps
  `FactsGen.idr`'s `bodied` column meaning what its own comment says — "the
  entry defines ONE triggered ability with a quoted expansion" — so those rows
  do not become `bodied := True` in the Idris reference.
- Aftermath's first two clauses [CR#702.127a] are written as ONE permission with
  `DeonticRider::Play`'s `exclusive: true`, which is exactly "may cast from
  there, and only from there"; the entry counts them as two static abilities.
  Disclosed rather than split, because no second clause can say "can't be cast
  from any other zone" on its own.
- Changeling's subject is `thisCreature`, not `This`: `CharacteristicEdit.fits`
  requires the subject to carry the creature type the `SubtypeSpace` is hosted
  on. The CR says "this object" [CR#702.73a], so a changeling on a non-creature
  card reads slightly narrower than the rule. Named here as a modelling debt.
- Hexproof and shroud write the PERMANENT form of their definition
  [CR#702.11b,702.18a]; the player form is what a grant to a player reads and
  has no separate declaration.

**Assurance counts.** Restored 0; re-spelled 2 assertions in 1 test (named
above); ignored-with-blockers 0; added 0 test functions (the 14 canon cards are
the added evidence, each proved by the kernel); removed 0.

**Glossary gaps.** None. Every term this landing needed — *keyword ability*,
*ability category*, *definition* — is already in
`docs/contexts/oracle-english/CONTEXT.md` or the game-model glossary.

### REPORT

Provenance, not a gate; measured on `ttpyynot`, 37 canon cards.

- `cargo xtask gate --changed` derives
  `cargo test -p deckmaste_construction_core -p deckmaste_construction
  -p deckmaste_english_v2 -p deckmaste_semantics_v2 -p xtask`; the run is green:
  38 suites reporting ok, 0 failed, 1 ignored (pre-existing), longest 38.3s.
- `cargo xtask lean-check plugins_v2/canon`: 37/37, 0.9s warm on an unchanged
  tree; 14.2s after a canon-only change; 93.9s on the first run of the session,
  which rebuilt the Lean library from cold. Host load was 1–3 with no sibling
  workspace building. The command is not instrumented per byte.
- `cargo fmt --all` leaves no changes; `cargo xtask facts check` clean;
  `cargo xtask cite check` 15,347 citations, 0 stale.

## Handoff

**Resume at `Embalm`** — the alphabetically first untouched declaration. 110
remain untouched: Embalm, Emerge, Enchant, Encore, Enlist, Entwine, Epic, Equip,
Escalate, Escape, Eternalize, Evoke, Evolve, Exalted, Exhaust, Extort, Fabricate,
Fading, Firebending, Flashback, Forecast, Foretell, Fortify, Freerunning, Frenzy,
Fuse, Gift, Graft, Gravestorm, Harmonize, Haunt, HiddenAgenda, Hideaway,
Impending, Improvise, Increment, Infinity, JumpStart, Kicker, LevelUp,
LivingMetal, Madness, Mayhem, MaxSpeed, Melee, Miracle, Mobilize, Modular,
MoreThanMeetsTheEye, Morph, Mutate, Myriad, Ninjutsu, Offering, Offspring,
Outlast, Overload, Paradigm, Partner, Phasing, Plot, PowerUp, Prototype, Prowl,
Provoke, Ravenous, ReadAhead, Rebound, Reconfigure, Recover, Reinforce, Renown,
Replicate, Retrace, Riot, Ripple, Saddle, Scavenge, Sneak, Solved, Soulbond,
SpaceSculptor, Spectacle, Splice, SplitSecond, Spree, Squad, StartYourEngines,
Station, Storied, Storm, Sunburst, Surge, Suspend, Teamwork, Tiered, Toxic,
Training, Transfigure, Transmute, Tribute, UmbraArmor, Undaunted, Unearth,
Unleash, Vanishing, Visit, Warp, WebSlinging, plus Modular/Renown/Storm which
`lean/Semantics/Macros.lean` already holds worked.

**Shape decisions taken, to be followed rather than re-derived.**

1. Every declaration gets the `Keyword(...)` wrapper and a `params: [...]` line
   even when the definition is empty, so a card can invoke the macro at its
   `Ability` position. A STOP is an empty `body: []` plus a comment beginning
   `// STOP.` that quotes the CR and names the missing constructor.
2. The `definition` column in `crates/xtask/src/facts.rs`'s `overlay()` lists
   ONE entry per ability of the definition, in text order.
3. Bodies are raw v2 constructors. The only macro invocations allowed are other
   `plugins_v2` declarations that already carry a body.
4. A move destination is a BARE zone; a `Condition::Exists` mention is
   bare-determiner; a `CharacteristicEdit` subject must carry the type its edit
   is hosted on; a variant named like a serde builtin is written `r#Name`.
5. Prove what you write: add a real canon card for each definition where the
   corpus has a clean example. Four of the first six non-trivial bodies were
   wrong until a card made the kernel read them.

**Two things worth raising before the remaining 110.**

- The `Destroy` deed's `patientRole` (`fieldObject`) makes every deontic rule
  over destruction unspellable. Indestructible is blocked on it and so is any
  later "can't be destroyed" body. This wants its own ticket against the
  `actFacts` role columns.
- 32 of the 46 written definitions are not yet read by the kernel. Closing that
  is cheaper than it looks — most want one vanilla-plus-keyword card — and it is
  the highest-value next step, ahead of new bodies.
