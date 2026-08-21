---
needs: []
---
# Audit the keyword classification, and transcribe the ability words

Two delegable tasks, neither needing a designer: both are inventory work against
a rubric that is already written down. They are grouped because they touch the
same keyword-authoring surface and can be claimed together or apart.

## 1. Keyword classification audit

`docs/keyword-policy.md` declares **9 intrinsic** (5 implemented), **19
composite-given**, **215 composite**, **1 marker**. The user estimates ~90%
confidence in that classification and suspects stragglers that should be
composite, plus one or two missed entries. The rubric is written down, so every
entry is checkable one at a time.

Do this first: the descriptive classification is pinned to **mtg-rules skill
v1.7.0** — confirm that pin is current before auditing, since a stale pin is a
plausible source of the misses.

(Check `docs-keyword-policy-refresh` before starting; if that ticket already
covers part of this surface, fold rather than duplicate.)

## 2. Ability words

Settled by user ruling: the shape is `AbilityWord Name Ability` — the word is
retained because it prints, and the shape correlation is authored as a macro. No
design dialogue is owed. What is missing is the measurement and the
transcription: measure the attested inventory of ability words, then transcribe
them against that shape.

## Consumption boundary

`docs/keyword-policy.md` for part 1; for part 2 the ability-word macro
definitions under `plugins/builtin/macros/` and whichever grammar site names the
words. No engine change in either part.

## Acceptance

- Part 1: every entry checked against the rubric, with the mtg-rules pin
  confirmed current first; reclassifications and misses listed.
- Part 2: the attested inventory is measured and transcribed; no word is added
  that the corpus does not print.

Standard constraints apply.

## As landed (2026-08-21)

### Part 1 — audit landed

Re-pinned `docs/keyword-policy.md` (and `docs/rules-taxonomy.md §10`, which
holds the pin block the policy cites) from mtg-rules **v1.10.0** — CR effective
2026-08-07, `keywords-classified.json` sha256 `6b1ab6ae89a9…`, 265 records =
195 abilities + 70 actions, matching `data/gen/catalogs/keyword-{abilities,
actions}.txt` exactly. The v1.7.0 baseline held 260.

- **Checked: 265** (the doc's "244" is the 260-row baseline minus the 16 action
  intrinsics Part II §10 governs; under 1.10.0 the same table spans 249).
- **Reclassified: 0.** No entry changed class between the pins. Intrinsic
  (9 abilities / 16 actions), composite-given (19 = 14 + 5) and marker (1) are
  identical in both; only the composite population grew, **215 → 220**
  (171 abilities + 49 actions).
- **Misses closed: 4** — §11 lacked assemble `[CR#701.45]`, exert
  `[CR#701.43]`, heal `[CR#701.69]`, recruit `[CR#701.70]`. Each is now filed
  under its decomposition target (a `RemoveDamage` row and a
  continuous-restriction row were added for heal and exert).
- **Additions since the pin: 5** — Storied `[CR#702.195]`, Power-up
  `[CR#702.193]`, Teamwork `[CR#702.194]` (ability composites, absorbed by the
  §2 count), heal and recruit (action composites, listed above). Storied and
  recruit are confirmed 1.10.0 additions; the other three are inferred from the
  1.7.0 → 1.8.0 count delta — no local copy of the v1.7.0 catalog survives
  (earliest cached is 1.8.0).
- **Misses in the other direction: 0.** Every entry the doc names exists in the
  1.10.0 catalog, with no rule-number drift on any cited entry.
- **Knowing divergences, retained: 5** — §8's stricter criterion keeps destroy /
  sacrifice / discard / exile composite where the descriptive catalog files them
  intrinsic; §10 keeps attach native where the catalog files it
  composite-given(attachment-relation). Both are argued in the doc.
- Folded from `docs-keyword-policy-refresh`: its **count-reconciliation** bullet
  (§2's rows are now labeled by catalog, so abilities and actions read
  separately). Its `ParamShape` / `KeywordDecl` decision was **not** touched.

### Part 2 — measured; transcription BLOCKED

Attested ability-word inventory (supported corpus = vintage-legal,
non-reversible; distinct oracle lines / supported cards):

All **61** words listed in `[CR#207.2c]` are attested — Landfall 129/180,
Threshold 91/99, Delirium 69/73, Domain 51/54, Channel 38/37, Raid 36/45,
Constellation 35/35, Heroic 34/44, Metalcraft 30/35, Morbid 28/29, Imprint
27/28, Magecraft 27/29, Ferocious 26/26, Enrage 23/24, Converge 22/28, Hellbent
22/22, Battalion 21/21, Coven 20/20, Alliance 19/23, Inspired 19/19, Adamant
18/18, Spell mastery 18/18, Eerie 16/16, Revolt 16/18, Formidable 14/14, Rally
14/14, Vivid 14/15, Bloodrush 13/13, Lieutenant 13/13, Strive 13/20, Survival
13/13, Undergrowth 13/13, Kinship 12/12, Renew 12/12, Void 12/14, Celebration
11/11, Flurry 11/11, Infusion 11/12, Repartee 11/12, Will of the council 11/11,
Addendum 10/10, Descend 4 10/10, Opus 10/10, Radiance 10/10, Chroma 9/9, Cohort
9/9, Council's dilemma 9/9, Disappear 9/9, Fateful hour 9/9, Paradox 9/9,
Parley 9/9, Descend 8 8/8, Pack tactics 8/8, Valiant 8/9, Grandeur 7/7,
Fathomless descent 6/6, Tempting offer 6/6, Eminence 5/5, Join forces 5/5,
Secret council 5/5, Sweep 4/4.

Two further labels are **attested but absent from `[CR#207.2c]`**: Corrupted
(24/24) and Will of the Planeswalkers (1/5) — both present in Scryfall's
`ability-words.json`. That is direct evidence for the open authority question in
`ability-word-catalog-regen`; it is left for that ticket's owner.

Eight Scryfall-listed labels print on **no** supported card (Covercast, Descend
bare, Hero's Reward, Kinfall, Landship, Legacy, Start your engines!, Underdog);
all appear only on unsupported printings or nowhere, and `Start your engines!`
is a keyword ability `[CR#702.179]` misfiled in that catalog. None were added.

**No macros were written.** The `AbilityWord Name Ability` shape has no
carrier that a plugin macro can name today:

- The user ruling (binder-unification-probe log, chapter thirty-six) gives the
  primitive to the **semantics-v2 Idris grammar** — "the grammar gets
  `AbilityWord Name Ability`". No `AbilityWord` constructor exists in
  `idris/src/**` today.
- Core has no carrier either: `Ability` (`crates/deckmaste_core/src/ability.rs`)
  has no ability-word variant, and `ron::kinds()` registers no `AbilityWord`
  kind, so a `.ron` macro whose body is `AbilityWord(…)` cannot load. Minting
  either is an engine + Idris change, excluded by this round's brief.
- `builtin-v2-macro-spelling-and-grammar` rules ability words **out** of
  declaration-backed vocabulary ("Ability words belong to a surface-label
  preservation path … do not select a semantic macro by the label alone"), so
  `plugins/builtin_v2/macros/stubs/` is not the address either.
- `aw-prefix-parsing` already records the same gap from the parser side ("no
  storage slot … requires a core-model decision") and is explicitly a
  design-dialogue ticket.

Deferred: the transcription half needs the `AbilityWord` carrier decided (core
variant vs. semantics-v2 grammar constructor) before any macro can be authored.
The measurement above is the input that decision needs.

Part 2's transcription is split out as `workbench-ability-word-primitive`:
the primitive has no home yet, and minting one is an engine/Idris decision.
