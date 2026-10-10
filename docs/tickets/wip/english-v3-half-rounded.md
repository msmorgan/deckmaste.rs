---
needs: []
---
# Read the fraction predeterminer *half* and its rounding supplement: You lose half your life, rounded up.

## Why

*half*, *rounded* and the verb *round* are not in the vocabulary, so every
line with them reads 0. Blocker report at trunk `uoqyuxtn`
(`target/english-v3/blockers-wave7.txt`): `word:half` / `word:rounded`.
`cargo xtask english-v3 grep 'rounded (up|down)'` finds **77** faces, all
unread, **68** with that line as their only failing line;
`grep -i '\bhalf\b'` finds **84** faces, all unread, **73** sole. 73 of the 77
*rounded* faces also have *half* in the same line, so the faces need both
parts; that is why this is one ticket.

Shapes in the failing lines (approximate matches): *half their / your life* 34, *half
their library* 9, *half the creatures / permanents / cards … they control /
in their hand* 14, *half the number of …* 7, *half their / your starting life
total* 9, *half that damage* 3, *half X damage / life / cards / … counters* 8,
*half that many …* 3, *half that player's / card's / creature's …* 5,
*half of their library* 1. *rounded up* 56, *rounded down* 27; 69 of the 83
are sentence-final, the rest precede a comma, a colon or a closing quote.

## Goal

*half* reads as a predeterminer Modifier of a definite NP wherever that NP
reads today, and *, rounded up* / *, rounded down* reads as a supplement
after the clause or cost that contains the fraction, with byte-exact
realisation.

## Analysis

CGEL Ch. 5 §12(b), p. 434, [3i]: "*John had already wasted* [*half his share
of the legacy*]"; "Predeterminers expressing fractions have the form of NPs.
*Half*, *quarter*, *third*, etc., are nouns"; of *half*, "it can occur
without a determiner"; and "There is an alternative construction in which the
fractional noun functions as head of the topmost NP, with a following
partitive complement: *half of his share*". Ch. 14 §9, p. 1265, [4iib]:
"*Born in Aberdeen, Sue had never been further south than Edinburgh*"; "The
underlined non-finites are supplements with the main clause as anchor". Those
back the predeterminer function, the noun Category, bare *half* and the
existence of a past-participial supplement anchored on a clause. Everything
else here is a project ruling, including the trailing position of the
rounding supplement.

Existing routes (`crates/deckmaste_english_v3/src/declarations.rs`):
`MultiplierNounPhrase` (`[modifier: lexical(Adverb) | lexical(Noun), " ",
head: NounPhrase]`, `require modifier.PredeterminerUse = Multiplier`,
`require head.Definiteness = Definite`) with feature `PredeterminerUse { No,
Multiplier }`; `lexeme:Multiplier/Double` in `lexicon/core.ron` is the model
noun lexeme. The comma supplement is the `Supplementation` schema
(`[anchor: node, ", ", supplement: PrepositionPhrase, "."]`, instances on
`Sentence` over `Clause` and on `Ability` over `KeywordPhrase`) and
`MedialSupplementation`; `ParticipialDepictive` and
`ParticipialPostmodifiedNominal` are the existing bare-passive consumers.
The multiplier landing (`docs/tickets/done/english-v3-multiplier-predeterminer.md`)
left *half* out.

Orchestrator rulings 2026-10-10:

- HR1. *half* is a Noun lexeme in `lexicon/core.ron` declaring a fraction
  value of `PredeterminerUse`; `MultiplierNounPhrase` is refined to read the
  declared predeterminer value (a table over `PredeterminerUse`), not a second
  Construction. Rename the Construction if its name no longer fits; the
  glossary entries **Predeterminer** and **Predeterminer Use** are amended in
  the landing.
- HR2. The host is definite, as for multipliers, read from the host's
  exported `Definiteness`. *half X damage*, *half X life*, *half X cards*
  (8 lines) have no definite host: in scope only if they fall out of a host
  the grammar already calls definite or scalar (MP4 of the multiplier
  ticket); otherwise list them with counts and leave them unread.
- HR3. *round* is a core verb in `lexicon/verbs.ron` with the particle frames
  the corpus attests (*up*, *down*; both lex today as
  `vocab:Adverb/Up|Down` and `vocab:Preposition/Up|Down`). Its past
  participle is an ordinary bare passive.
- HR4. The rounding supplement refines `Supplementation` /
  `MedialSupplementation`: the supplement slot admits a bare-passive
  participial phrase whose head verb declares the supplement use. The licence
  is a declared feature of the verb lexeme, never the word *rounded*. The
  anchor is the clause (project ruling; CGEL p. 1265 supports a clause
  anchor, not this position). No NP-internal postmodifier Reading of
  *life, rounded up* is added.
- HR5. One analysis per string. *half their life* reads only as
  predeterminer + NP: not as a head noun *half* with a bare NP dependent, and
  not through `PartitiveNounPhrase` (that is *half of their library*, 1 face:
  in scope only if the existing partitive route reads it once the noun
  exists). *It deals double that damage instead.* and *Copy that spell
  twice.* keep exactly their Readings.
- HR6. Newly covered faces are gains. No Reading-count increase on a
  previously covered face is expected (all three words are new); report any
  with its shape before landing.

## Witnesses

Each is the face's only failing line at `uoqyuxtn`. Probed 2026-10-10 with
the `uoqyuxtn` binary (`--readings 3`, `--as-card` where the name occurs):
witness 0, control as shown (3+ = enumeration capped). Re-probe on the claim
parent first.

- Grievous Wound: "Whenever enchanted player is dealt damage, they lose half
  their life, rounded up." (control "… they lose 2 life." 1)
- Cut Your Losses: "Target player mills half their library, rounded down."
  (control "Target player mills three cards." 1)
- Endless Ranks of the Dead: "At the beginning of your upkeep, create X 2/2
  black Zombie creature tokens, where X is half the number of Zombies you
  control, rounded down." (controls "… where X is twice the number of Zombies
  you control." 2; "… where X is the number of Zombies you control." 2)
- The Endstone: "At the beginning of your end step, your life total becomes
  half your starting life total, rounded up." (control "… becomes your
  starting life total." 1)
- Heartless Hidetsugu (document): "{T}: Heartless Hidetsugu deals damage to
  each player equal to half that player's life total, rounded down." (control
  without "half … , rounded down" 3+)
- Malignus: "Malignus's power and toughness are each equal to half the
  highest life total among your opponents, rounded up." (control without
  "half … , rounded up" 2)

Also 0 today: "That player loses half their life."; NP "half that player's
life total" (NP "twice that player's life total" 2). Controls that must not
change: "It deals double that damage instead." 1; "Copy that spell twice." 1;
"Put twice that many +1/+1 counters on it." 1; "Prevent double that damage."
1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching

    \b(half|third|tenth|twice|double|triple|rounded?)\b

(704 faces, case-insensitive) plus 300 random covered controls (`cargo xtask
english-v3 diff --manifest … --random 300 --seed 20261008`), `--workers 6
--samples-per-face 0`, compared with `cargo xtask english-v3 diff` against
the claim parent. Zero lost; HR6 for increases. Full `cargo test -p` and
`cargo clippy -p … --tests -- -D warnings` for `deckmaste_english_v3` and
`deckmaste_lexical_source`, once, at the end: a rebuild after a grammar edit
takes about 3 minutes and the test binary about 7, so batch edits; lexicon
data loads at runtime. Tests assert the role (the fraction as predeterminer
Modifier, the definite host, the participial supplement and its anchor, the
HR5 negatives), never an incidental count or a Debug-printed tree.
Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- *a third of their life*, *a tenth of the creatures …* (2 faces; no *half*):
  fraction noun as head with a partitive Complement. *a third of their life*
  reads 0 without the supplement too.
- *Round up each time.* / *Round down each time.* (4 faces, each also needs
  *half*): in scope only if the HR3 verb declaration reads the imperative
  with no further declaration; otherwise list.
- *divided evenly, rounded down, among …* (2 faces; *evenly* unknown).
- *less than or equal to half your starting life total* (4 faces): in scope
  only as a by-product of HR1.
- *half X …* beyond HR2.

## Landing record

Not landed.
