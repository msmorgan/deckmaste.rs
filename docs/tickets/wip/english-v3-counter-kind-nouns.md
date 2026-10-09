---
needs: []
---
# Declare the attested counter kinds: Put a petrification counter on target creature.

## Why

A counter whose kind is not declared never reads: the kind word is a
vocabulary gap. On census `xnkuznpo` (docs-only descendant of `vvxtllnq`;
32,828 supported faces, 23,459 covered, 9,369 unread; wave-4 recon of
2026-10-08), the recon bucket "counter-kind noun gap (unknown noun before
*counter*)" touches **177** unread faces and is the sole cause on **125**.
It covers 145 distinct kind words, none of them declared today. Counts are
unread faces *touched* / *sole* as defined in the recon; surface counts, not
gain forecasts.

Probes (admitted roots, `--category Sentence --readings 1`): "Put a
petrification counter on target creature." 0 (gap *petrification*), while
the *stun* version reads 1. "Put a blood counter on target creature." 1:
*blood* is a declared kind with no `grammar:` field.

## Goal

Each attested kind below reads as a counter kind wherever a declared kind
reads today. Examples are *an intel counter*, *two eon counters*, *the
number of knowledge counters on it* and *enters with a javelin counter on
it*. Realisation is byte-exact.

## Ruling and mechanism

Owner ruling, 2026-10-08: this ticket may add `CounterKind` declarations under
`plugins_v2/builtin/macros/counter_kinds/`. This is the one exception to the
standard "edit `grammar:` fields only, plugin bodies untouched" rule (Method
item 7). The exception covers new marker-counter declarations only. Existing
declarations and every other plugin body stay untouched.

How declared kinds reach the grammar today: a `CounterKind` declaration
without a `grammar:` field, such as `bloodCounter.ron` (`name`, `params: []`,
`spelling`, `holder`), is lexed as the compound noun
`lexeme:counter_kind/<name>/compound-noun` (*blood counter*) through the
declaration reader's compound-noun recipe. The construction-core test calls
these `SEMANTIC_ONLY_NAMES`
(`crates/deckmaste_construction_core/tests/builtin_v2_counter_kinds.rs`).
Kinds with a `grammar: FixedTerm(...)` field are `EXPECTED_NAMES`. Add each
new kind in the semantic-only shape and extend `SEMANTIC_ONLY_NAMES` to
match. That is a test addition, never a removal. Declare `holder: Player`
only where the corpus puts the kind on a player. Otherwise use `Object`.

Other consumers of these declarations, checked:

- `cargo xtask facts generate` writes every counter declaration into the
  tracked `lean/Semantics/Check/Facts.lean`, and `facts check` rejects drift.
  Regenerate and commit it.
- Plugin definitions change, so `cargo xtask lean check` runs once at the end
  (CLAUDE.md, Tests for a landing). This overrides "no Lean checks" in Method
  item 1 for this ticket only.
- `deckmaste_semantics_v2` reads the same declarations
  (`src/rules.rs`, `Definition::Counter`). Run its tests and clippy. Live
  `semantics-v2-*` claims may also touch `Facts.lean`; on a refresh
  conflict there, regenerate rather than hand-merge.
- `plugins/wizards` is generated from the v1 pipeline
  (`cargo xtask generate` → `deckmaste_migrations`), which does not read
  `plugins_v2`. No wizards regeneration is needed.

## Attested kinds

Supported faces containing *<kind> counter(s)* (reminder text stripped;
Vintage legal or restricted, not `funny`; rg-equivalent pass over
`data/scryfall/oracle-cards.jsonl`, 2026-10-08). Every kind has at least one
face, so the pruning rule admits all 145:

acorn 1, aegis 1, aim 2, arrow 1, arrowhead 1, awakening 1, bait 1, blaze 2,
blessing 1, bloodline 1, book 1, bore 1, brain 1, bribery 1, burden 1, cage 1,
carrion 1, cell 1, chorus 1, collection 2, component 1, conqueror 1,
contested 1, corpse 4, corruption 1, credit 1, croak 1, crystal 1, cube 1,
currency 1, delay 2, descent 1, despair 1, discovery 1, dread 2, duty 1,
elixir 1, ember 1, enlightened 1, eon 2, eyeball 1, fate 2, feather 3,
feeding 1, fellowship 1, fetch 2, filibuster 1, film 1, fire 2, flame 1,
foreshadow 1, fury 1, gem 1, ghostform 1, glyph 1, gold 2, harmony 1,
hatching 2, hatchling 3, hit 3, hoofprint 1, hope 1, hourglass 1, hunger 1,
impostor 1, incarnation 1, incubation 1, infection 3, influence 1,
ingenuity 2, ingredient 1, intel 2, invasion 1, invitation 1, isolation 1,
javelin 1, judgment 2, kick 1, knowledge 1, landmark 1, loot 1, magnet 1,
manifestation 1, mannequin 1, matrix 1, memory 2, mine 1, mining 1, mire 1,
music 1, muster 1, nest 1, net 2, night 1, ore 1, pain 1, palliation 1,
paralyzation 1, petal 1, petrification 1, phylactery 1, phyresis 1, pin 1,
point 2, polyp 1, possession 1, pressure 3, prey 1, pupa 1, rally 1,
refine 1, rejection 1, reprieve 1, revival 1, ribbon 1, ritual 1, rope 1,
rust 1, saurian 1, shell 1, shred 1, silver 1, skewer 1, sleep 1, slumber 1,
soot 1, story 1, strife 1, supply 1, takeover 1, task 1, theft 1, tower 1,
trap 1, treasure 1, unity 1, valor 1, velocity 2, vitality 1, void 2,
vortex 1, vow 1, voyage 1, wage 1, winch 1.

Several kind words are also other Lexemes' spellings (*treasure*, *mine*,
*book*, *fire*, *gold*, *silver*, *hope*). Each new analysis is the
compound *<kind> counter* only, so it must not add a Reading to any string
without *counter*.

## Witnesses

Each witness's failing units all match this construction and no other recon
STRONG bucket (`xnkuznpo`, 0 Readings each, the kind reported as a gap):

- Xathrid Gorgon: "Put a petrification counter on target creature."
  (ability body; control with *stun* 1)
- Flamewar, Streetwise Operative: "Put an intel counter on each of them."
  (control with *+1/+1* 1)
- Out of the Tombs: "At the beginning of your upkeep, put two eon counters on
  this enchantment, then mill cards equal to the number of eon counters on
  it." (control with *age* 1)
- The Magic Mirror: "At the beginning of your upkeep, put a knowledge counter
  on The Magic Mirror, then draw a card for each knowledge counter on The
  Magic Mirror." (control with *lore* 1)
- Icatian Javelineers: "This creature enters with a javelin counter on it."
  (control with *charge* 1)

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, scope).
Deltas:

1. Verification budget (binding): One full census (`--all --workers 12
   --samples-per-face 0`) on the finished feature for the record, compared
   face by face (per-face `readings_sha256` and counts) against the claim
   parent's census; iterate on selector subsets, never on --all. Tests:
   `cargo test -p` and `cargo clippy -p … --tests -- -D warnings` for every
   crate whose code or consumed data changed: `deckmaste_english_v3`,
   `deckmaste_lexical_source`, `deckmaste_construction_core` and
   `deckmaste_semantics_v2`. No xtask tests. Lean: only `cargo xtask facts
   generate`/`facts check` and one `cargo xtask lean check` (see Ruling and
   mechanism). No post-refresh re-census unless refresh touched grammar,
   lexicon, core or `plugins_v2`. New tests go under tests/suite/ registered
   in tests/main.rs. Evidence lives under the workspace's ignored
   `target/english-v3/`, never `/tmp`.
2. Write each witness as a test first.
3. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
4. Compare per-face Reading counts against the claim-parent census. Any
   decrease on a previously covered face is restored, or named and justified in
   the landing record; a decrease inherited from trunk is attributed by change
   id.
5. Spot-check at least 10 newly covered faces and list them by name with their
   Reading. A kind word read as a free noun premodifying *counter*, rather than
   as the declared compound, or a new Reading on a string without
   *counter*, is a defect. A wrong analysis that parses is a defect, not a
   gain.
6. No guard may name a lexeme, construction, verb, noun, preposition or card;
   read declared features only. Tests are re-spelled, never deleted. No new
   xtask subcommands, flags, fixtures or tooling.
7. May add `CounterKind` declarations as ruled above, and may edit `grammar:`
   frame fields in `plugins_v2` keyword actions and lexicon frames in
   `crates/deckmaste_lexical_source/lexicon/*.ron`. Other plugin bodies
   stay untouched. Each added kind must be in the attested list above
   (pruning rule). A declaration comment that cites the Comprehensive Rules
   follows the CLAUDE.md citation rules.
8. A new frame or construction must not overlap an existing one on the same
   string (two labels for one constituency is a spurious duplicate, not an
   ambiguity); when it supersedes one, retire the old route and re-spell its
   tests.
9. Retire a superseded route on both the lexicon and the grammar side; do not
   leave unreachable declarations.
10. A CGEL citation may back only what the cited passage itself says; a project
    or orchestrator ruling is cited as a ruling, never attributed to CGEL.

## Out of scope

- Engine semantics of the new kinds beyond a marker counter.
- Monotransitive *get* with a player Subject (*that player gets two poison
  counters*), a separate recon bucket.
- Faces whose other units fail on their own: note them.

## Landing record

1. **Census delta** (`posytyktxlqt`): claim-parent `mqvrqlnm`-equivalent bounded runtime → finished feature: covered 4,294 → 4,404; Readings 455,168 → 475,655; no/one/multiple 1,897/1,215/3,079 → 1,787/1,221/3,183. Gained 110; lost faces: none; decreases, increases and digest-only changes: all 0. Selector: every supported face containing *counter* in any case (5,935), plus 300 covered controls from standard `diff --manifest --random 300 --seed 20261008`; 44 overlaps, 6,191 distinct faces. All enumerations complete; exact realization and traversal validate every Reading; issues, duplicate Readings and internal failures: 0. No new Reading on any covered face without *counter*. Standard `diff` and evidence: `target/english-v3/english-v3-counter-kind-nouns/`.
2. **Witnesses** (`posytyktxlqt`): all five card-scoped Sentence probes had 0 Readings before the declarations. After, complete fragment / whole-face counts: Xathrid Gorgon 1 / 8; Flamewar, Streetwise Operative 1 / 144; Out of the Tombs 37 / 259; The Magic Mirror 54 / 1,458; Icatian Javelineers 5 / 5. Every fragment Reading owns the declared compound Lexeme, realizes byte-exactly and validates traversal.
3. **Spot checks** (`posytyktxlqt`, whole-face counts): Archery Training (3), *arrow counter*; Assemble the Legion (156), *muster counter*; Aurification (12), *gold counter*; Aven Mimeomancer (36), *feather counter*; Dawn of a New Age (66), *hope counter*; Dragon's Hoard (1), *gold counter*; Cephalid Vandal (14), *shred counter*; Mine Layer (2), *mine counter*; Spell Satchel (8), *book counter*; Icatian Javelineers (5), *javelin counter*. One selected whole-Document Reading inspected per face: each kind is its declaration-owned Compound Noun, not a free noun Premodifier. All are gains in the corrected standard diff and realize exactly. Legacy's Allure remains blocked by its other construction.
4. **Deviations and STOPs** (`posytyktxlqt`): all 145 attested kinds declared as Object marker counters; none left undeclared. No new Construction, frame, guard, superseded route or glossary gap. Added an independently constructed-compound test for identity, Number and both roundtrip laws alongside the five witness tests. The orchestrator ruling of 2026-10-08 replaces the ticket's full census with the bounded selector. The inherited landwalk baseline was stale: apparent unrelated gains and Rat King, Verminister / Yoshimaru decreases disappeared against an accurate bounded parent-equivalent runtime. That baseline was captured with only the new declarations temporarily excluded, then all restored; production Rust, lexicon and existing plugins match the claim parent. Original inherited evidence is retained separately, not counted as this feature's delta. No STOPs or unresolved contradictions.
5. **Tests** (`posytyktxlqt`): `cargo test -p deckmaste_english_v3` passed 573; `cargo test -p deckmaste_lexical_source` passed 61; `cargo test -p deckmaste_construction_core` passed 465; `cargo test -p deckmaste_semantics_v2` passed 100. `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`, `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings`, `cargo clippy -p deckmaste_construction_core --tests -- -D warnings` and `cargo clippy -p deckmaste_semantics_v2 --tests -- -D warnings` all passed. `cargo xtask facts generate`, `cargo xtask facts check` and exactly one `cargo xtask lean check` passed; regenerated Facts.lean is included, Idris output unchanged. Initial lexical-source and construction-core failures were stale exact inventory totals, updated 73 → 218 without weakening any per-member assertion; only those affected crate tests were repeated. Assurance: restored 0; re-spelled 2 (`counter_class_exports_owned_complete_noun_paradigms`, `every_builtin_noun_declaration_inherits_its_class_semantics`); ignored-with-blocker 0; added 6; removed 0. The existing named-declaration inventory also gained all 145 required entries.
6. **Timing** (`posytyktxlqt`): bounded landing census 75.961 s, 12 workers.
