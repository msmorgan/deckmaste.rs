---
needs: []
---
# Read a parameter-taking keyword named without its parameter: … gains flashback until end of turn. The flashback cost is equal to its mana cost.

## Why

A keyword whose declaration takes a parameter (`params: [Cost]`, `[Amount]`,
`[Amount, Cost]`, `[Ability]`) reads only with that parameter: "Target
creature gains flashback {2} until end of turn." 1, "… gains flashback until
end of turn." 0; NP "a card with cycling {2}" 1, "a card with cycling" 0.
Oracle names such keywords bare in two positions. Blocker report at trunk
`uoqyuxtn` (`target/english-v3/blockers-wave7.txt`): part of key `at:turn .`
44, and scattered `at:<keyword> .` / `at:<keyword> cost` keys.

With K = the parameter-taking keyword surfaces that occur bare (list in
Method), `cargo xtask english-v3 grep` (per-keyword and per-head figures
count unread lines in `blockers-wave7.json`):

- (a) ability name: `\b(with|without|gains?|has|have|had|loses?) (K)` then a
  full stop, comma or function word: **89** faces, all unread, **77** with
  that line as their only failing line. *flashback* 22, *suspend* 16, *toxic*
  10, *escape* 5, *foretell* / *replicate* / *disturb* 3 each, 26 others.
- (b) premodifier: `\b(K) (costs?|abilit(y|ies))\b`: **115** faces, 108
  unread, **99** sole. Heads: *cost(s)* 57, *ability / abilities* 32.
  *flashback* 16, *equip* 8, *exhaust* 6, *escape* 5, *morph* 5.
- union: **164** faces, 157 unread, **142** sole. 33 sole lines contain both
  (*… gains flashback until end of turn. The flashback cost is equal to its
  mana cost.*), so the faces need both parts; that is why this is one ticket.

The 7 covered (b) faces are all *cycling cost* / *cycling ability*: they read
today through `core-verb:Cycle`'s gerund-participle as a `VerbalPremodifier`.

## Goal

A parameter-taking keyword's bare name reads (a) as a keyword phrase where a
nullary keyword reads today (granted by *gain* / *have* / *lose*, Complement
of *with* / *without*) and (b) as a noun Premodifier of a head that declares
it (*flashback cost*, *morph ability*, *equip abilities*), with byte-exact
realisation.

## Analysis

No CGEL claim: naming a keyword without its parameter is an Oracle
convention, and both parts are project rulings.

Existing routes (`crates/deckmaste_english_v3/src/declarations.rs`):
`BareKeyword` (`[head: lexical(Keyword)]`, `require
head.KeywordParameterClass = Nullary`) beside `AmountKeyword`, `CostKeyword`,
`AmountCostKeyword`; their consumers `KeywordGrantedAbility` and
`KeywordComplementPreposition` take any `KeywordPhrase`. The class comes from
the plugin declaration's `params:` (`keyword_parameter_class` in
`crates/deckmaste_lexical_source/src/legacy/plugins.rs`); feature
`KeywordParameterClass { Nullary, Amount, Cost, Quality, Subject, AmountCost,
QualityCost, Ability, Condition, CostPowerToughness, Instruction }`. For (b):
`NounPremodifier` (`[head: lexical(Noun)]`) and `NounPremodifiedNominal`
with `licensed_noun_premodifier(modifier.NounPremodifier,
head.AttributePremodifierHead)`, the Attribute Noun Premodifier route (*card
type*); *mana ability* is a declared compound and stays one.

Orchestrator rulings 2026-10-10:

- KM1. Part (a) refines `BareKeyword`: its class test becomes a table over
  `KeywordParameterClass` that also admits the classes attested bare. Those
  are Cost, Amount and AmountCost (*flashback*, *toxic*, *suspend*); other
  classes join only with a grep count recorded in the landing. No new
  Construction, no keyword named in the grammar.
- KM2. Part (b) refines `NounPremodifier`: a form with a `Keyword` lexeme
  head, licensed by a premodifier feature that the keyword-declaration loader
  mints for every keyword (mechanically minted, so exempt from the pruning
  rule), and selected by the head noun's declared permission (*cost*,
  *ability* declare it as data, like `AttributePremodifierHead`). No
  Adjective or Noun homograph of a keyword is minted. Nullary keywords get
  the same feature; *a landwalk ability* (0 today) may gain.
- KM3. One analysis per string. *gains flashback {2}* and the keyword lines
  "Flashback {2}{R}", "Cycling {2}", "Suspend 4—{1}{U}", "Toxic 1" keep
  exactly one Reading: no bare-name Reading with the parameter left over.
- KM4. A bare parameter-taking keyword also becomes a possible Keyword Line
  (*Flashback* alone). That is unattested and harmless; do not add a check
  whose only job is to block it. Report any covered face it changes.
- KM5. Expected increases: the 7 covered *cycling cost / ability* faces gain
  the keyword-premodifier Reading beside the gerund-participle one (both are
  grammatical; neither is removed). Report each with its counts. Any other
  Reading-count increase on a covered face is reported with its shape before
  landing and is presumed a defect under KM3.
- KM6. If (b) needs more than the form, the minted feature and head-noun
  data, land (a) and move (b) to its own ticket with the evidence; do not
  STOP the whole ticket. Newly covered faces are gains.

## Witnesses

Each is the face's only failing line at `uoqyuxtn`. Probed 2026-10-10 with
the `uoqyuxtn` binary (`--category document --readings 3`): witness 0,
control as shown (3+ = enumeration capped). Re-probe on the claim parent
first.

- Epochrasite (a): "When this creature dies, exile it with three time
  counters on it and it gains suspend." (control "… gains haste." 3+)
- Compleat Devotion (a): "Target creature you control gets +2/+2 until end of
  turn. If that creature has toxic, draw a card." (control "… has flying, …"
  1)
- Rootcoil Creeper (a): "{G}{U}, {T}, Exile this creature: Return target
  card with flashback you own from exile to your hand." (control "… with
  flash …" 2)
- Flashback (a + b): "Target instant or sorcery card in your graveyard gains
  flashback until end of turn. The flashback cost is equal to its mana cost."
  (control "… gains flash until end of turn. The mana cost is equal to its
  mana cost." 3+)
- Murasa Sproutling (b): "When this creature enters, if it was kicked, return
  target card with a kicker ability from your graveyard to your hand."
  (control "… with a mana ability …" 2)

Also 0 today: "Each creature card in your graveyard has scavenge. The
scavenge cost is equal to its mana cost." (Young Deathclaws); NP "the
flashback cost", "a morph ability". Controls that must not change: "Target
creature gains flashback {2} until end of turn." 1; "Flashback {2}{R}" 1;
"Cycling {2}" 1; "Suspend 4—{1}{U}" 1; "Toxic 1" 1; NP "a creature with ward
{2}" 1; NP "a creature with landwalk" 1; NP "a mana ability" 1. Expected to
increase (KM5): "You may pay {0} rather than pay the cycling cost of the
first card you cycle each turn." 3+ (Gavi, Nest Warden).

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching

    \b(with|without|gains?|has|have|had|loses?) (K)\b|\b(K|mana|loyalty|[a-z]+cycling|landwalk) (costs?|abilit(y|ies))\b

with K = `flashback|suspend|toxic|escape|foretell|replicate|disturb|emerge|mutate|vanishing|encore|madness|scavenge|embalm|harmonize|warp|cumulative upkeep|cycling|disguise|blitz|unearth|plot|ward|mayhem|fading|miracle|equip|eternalize|modular|kicker|morph|echo|surge|prowl|exhaust|boast|power-up|ninjutsu|backup|craft|bestow`
(819 faces: the 164 plus parameterised and compound control classes) plus
300 random covered controls (`cargo xtask english-v3 diff --manifest …
--random 300 --seed 20261008`), `--workers 6 --samples-per-face 0`, compared
with `cargo xtask english-v3 diff` against the claim parent. Zero lost; KM5
for increases. Full `cargo test -p` and `cargo clippy -p … --tests -- -D
warnings` for `deckmaste_english_v3` and `deckmaste_lexical_source`, plus
`cargo test -p xtask english_v3`, once, at the end: a rebuild after a grammar
edit takes about 3 minutes and the test binary about 7, so batch edits;
lexicon and plugin `grammar:` data load at runtime. Tests assert the role
(the bare keyword as the granted ability and as Complement of *with*; the
keyword as Premodifier of *cost* / *ability*; the KM3 negatives), never an
incidental count or a Debug-printed tree. Attestation by `cargo xtask
english-v3 grep` only. Glossary: add the premodifier permission KM2
introduces beside **Attribute Noun Premodifier**.

## Out of scope

- *if the gift was promised* (25 lines; `word:promised`), *a player who
  doesn't have max speed*, *enchant* in quoted token text.
- *using its blitz / mutate / warp ability* (8 lines): *using* is not in the
  vocabulary; (b) gets them only as far as the NP.
- *landwalk of the chosen type* (7 lines): a parameter in another form, not
  a bare name.
- *a loyalty ability* (NP 0 today): an ordinary noun Premodifier gap, not a
  keyword.
- *Spend this mana only to pay cumulative upkeep costs.*: gains only if the
  purpose clause reads once (b) lands.

## Landing record

1. **Census delta** (claim-parent code `pkomyrrn`, measured on empty
   `zvtzutzx` before edits → implementation `zvtzutzx`): bounded selection
   of 1,198 faces; covered 822 → 967; Readings 100,627 → 103,435.
   Lost none; gained 145; decreases 0; increases 8; digest-only 0;
   unchanged 1,045. After refresh, `zvtzutzx` has covered 977 and
   107,915 Readings: trunk `lvpxmkklqtms` adds 10 clause-initial *once*
   faces (4,480 Readings), with no further loss, decrease, increase or
   digest-only change. Original-parent comparison now has 155 gains,
   8 increases and 1,035 unchanged. A second refresh, measured on
   `yymxltku`, adds Dynaheir, Invoker Adept and Pit Automaton through
   trunk `zunolrnvsuqr` (52 Readings). Final original-parent comparison:
   covered 822 → 979; Readings 100,627 → 107,967; lost 0, gained 157,
   decreases 0, increases 8, digest-only 0, unchanged 1,033. Each increase is the KM5 Keyword Premodifier beside
   the retained cycling gerund-participle: Abandoned Sarcophagus 10 → 20,
   Fluctuator 4 → 8, Gavi, Nest Warden 58 → 114, Herald of the Forgotten
   37 → 73, New Perspectives 3 → 5, Rooting Moloch 8 → 15, Spellpyre
   Phoenix 17 → 32, Zenith Flare 12 → 24. No unintended gained analysis
   found in the role tests or spot checks. Evidence:
   `target/english-v3/english-v3-keyword-mention-without-parameter/`.
2. **Witnesses** (`pkomyrrn` code → `zvtzutzx`, complete fragment probes):
   Epochrasite 0 → 4; Compleat Devotion 0 → 1; Rootcoil Creeper 0 → 2;
   Flashback 0 → 5; Murasa Sproutling 0 → 3; Young Deathclaws 0 → 2;
   *the flashback cost* 0 → 1; *a morph ability* 0 → 1. All witnesses
   failed before implementation. Substituted controls remained at 4, 1,
   2, 5 and 2 respectively. Parameterized grant and Flashback, Cycling,
   Suspend and Toxic lines each retained 1; ward, landwalk and mana-ability
   NP controls each retained 1. Gavi fragment 56 → 112 (KM5).
3. **Spot checks** (`zvtzutzx`): Epochrasite (bare suspend grant), Compleat
   Devotion (bare toxic grant), Rootcoil Creeper (flashback Complement of
   *with*), Flashback (bare grant and keyword cost Premodifier), Murasa
   Sproutling (kicker ability Premodifier), Young Deathclaws (scavenge grant
   and cost Premodifier), Frenzied Raider (boast ability Premodifier),
   Memory Crystal (buyback costs Premodifier), Delay (bare suspend), Lier,
   Disciple of the Drowned (flashback grant and cost Premodifier). Full-face
   probes selected the intended roles; no probe issues.
4. **Deviations and STOPs**: no new Construction or frame. KM2 required
   mechanical propagation and coordination of the declared head permission
   through existing nominals; the existing licence table gained that
   argument. Added glossary terms **Keyword Premodifier** and **Keyword
   Premodifier Head**, citing KM2 as a project ruling. The Method selector
   was widened to all declared eligible bare keywords and all fixed keyword
   Premodifiers, yielding 901 surface faces plus 300 seeded covered controls
   (3 overlaps), rather than the ticket's narrower list. Case-insensitive
   attestation includes Fluctuator as the eighth KM5 face. A preliminary
   census exposed 138 unintended typecycling-suffix analyses on 7 faces;
   requiring declared `BoundKeyword: No` retired them before the final
   census, without removing any parent Reading. Optional *a landwalk
   ability* remains unread (bound-suffix representation); its existing
   *with landwalk* route is preserved. No STOP. Three existing tests were
   re-spelled for the authorized bare forms; an independent Cost fixture
   was corrected to its declared nominal frame without weakening equality.
   Refresh conflicted only at the test-binary registration tail; both
   module registrations were retained exactly once. Two later refresh
   attempts deferred on a concurrent named-modes workspace edit; retry
   succeeded without changing that workspace’s content.
5. **Tests** (`zvtzutzx` before refresh; final refreshed tree `yymxltku`): `cargo test -p deckmaste_english_v3` passed
   797 tests before refresh, 800 after the first and 807 after the second; `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`
   passed; `cargo test -p deckmaste_lexical_source` passed 83 tests;
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` passed.
   Assurance: restored 0 / re-spelled 3 / ignored 0 / added 8 / removed 0.
   New tests assert full independent values and construction roles. No
   xtask code, construction-core code or plugin definitions changed;
   no xtask tests or Lean checks required. After refresh, `cargo build -p
   xtask`, the same bounded census and both full suites and clippy passed.
   Citation checks: 0 stale,
   0 noncompliant, diff audit 0 new citation sites.
6. **Timing** (`yymxltku`): final bounded census 114.43 seconds wall time,
   6 workers, samples per face 0, prebuilt binary under `/usr/bin/time -v`.
   Full corpus census was not run, per the binding budget.
