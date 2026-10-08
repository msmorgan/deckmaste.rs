---
needs: []
---
# Read *permanent* as a noun premodifier: Return target permanent card from your graveyard to your hand.

## Why

*permanent card* and *permanent spell* never read. On census `xnkuznpo`
(docs-only descendant of `vvxtllnq`; 32,828 supported faces, 23,459 covered,
9,369 unread; wave-4 recon of 2026-10-08), the recon bucket "*permanent
card/spell*" touches **308** unread faces and is the sole cause on **210**.
Counts are unread faces *touched* / *sole* as defined in the recon; surface
counts, not gain forecasts.

Probes (admitted roots): "Choose target permanent card in your graveyard." 0,
while "Choose target creature card in your graveyard." 1; "Whenever you cast
a permanent spell, draw a card." 0; "Return all legendary permanent cards
from your graveyard to the battlefield." 0, while the *creature* string reads
1.

## Goal

The common noun *permanent* premodifies a Nominal (*permanent card*,
*nonland permanent card*, *Elf permanent card*, *permanent spell*), realised
byte-exactly, through the same route as *creature card*.

## Analysis

CGEL, Ch. 6 §2.4.1(a), p. 537: nouns function as attributive modifiers in the
structure of a nominal (*a government inquiry*).

Existing route: `lexeme:CommonNoun/Permanent` (`vocabulary.rs:369`) does not
declare `NounPremodifier: Yes`; examples of the property are
`core.ron:101,369`. The consumer is `NounPremodifier` /
`NounPremodifiedNominal` (`declarations.rs:2232,2275`). This ticket declares
the property for *permanent*. A Reading that premodifies with *permanent*
where Oracle has *permanent* as the head (*target permanent*) is a duplicate
to rule out by the census diff.

## Witnesses

Each witness's failing units all match this construction and no other recon
STRONG bucket (`xnkuznpo`, 0 Readings each):

- Cosmic Rebirth: "Choose target permanent card in your graveyard." (control
  with *creature* 1)
- Summon: Esper Maduin: "If it's a permanent card, put it into your hand."
  (control with *creature* 1)
- Primevals' Glorious Rebirth: "Return all legendary permanent cards from your
  graveyard to the battlefield." (control with *creature* 1)
- Divine Gambit: "That player may put a permanent card from their hand onto the
  battlefield."
- Skyshroud Poacher: "Search your library for an Elf permanent card, put it
  onto the battlefield, then shuffle." (ability body; control with *creature*
  1)

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, scope).
Deltas:

1. Verification budget (binding): One full census (`--all --workers 12
   --samples-per-face 0`) on the finished feature for the record, compared
   face by face (per-face `readings_sha256` and counts) against the claim
   parent's census; iterate on selector subsets, never on --all. Tests:
   `cargo test -p` and `cargo clippy -p … --tests -- -D warnings` for every
   crate whose code or consumed data changed (`deckmaste_english_v3` always;
   `deckmaste_lexical_source` if core.ron/verbs.ron/vocabulary.rs changed).
   No xtask tests, no Lean checks, no post-refresh re-census unless refresh
   touched grammar, lexicon or core. New tests go under tests/suite/
   registered in tests/main.rs. Evidence lives under the workspace's ignored
   `target/english-v3/`, never `/tmp`.
2. Write each witness as a test first.
3. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
4. Compare per-face Reading counts against the claim-parent census. Any
   decrease on a previously covered face is restored, or named and justified in
   the landing record; a decrease inherited from trunk is attributed by change
   id.
5. Spot-check at least 10 newly covered faces and list them by name with their
   Reading. *permanent* read as the head with *card* or *spell* as a second head, or a new Reading on an already covered *target permanent* string, is a defect. A wrong analysis that parses is a defect, not a gain.
6. No guard may name a lexeme, construction, verb, noun, preposition or card;
   read declared features only. Tests are re-spelled, never deleted. No new
   xtask subcommands, flags, fixtures or tooling.
7. May edit `grammar:` frame fields in `plugins_v2` keyword actions and lexicon
   frames in `crates/deckmaste_lexical_source/lexicon/*.ron`; plugin bodies
   untouched. Lexicon additions must be attested in the supported corpus
   (pruning rule).
8. A new frame or construction must not overlap an existing one on the same
   string (two labels for one constituency is a spurious duplicate, not an
   ambiguity); when it supersedes one, retire the old route and re-spell its
   tests.
9. Retire a superseded route on both the lexicon and the grammar side; do not
   leave unreachable declarations.
10. A CGEL citation may back only what the cited passage itself says; a project
    or orchestrator ruling is cited as a ruling, never attributed to CGEL.

## Out of scope

- *nontoken* (`english-v3-negative-common-noun-premodifier`).
- Faces whose other units fail on their own (e.g. *Descend 8* labels): note
  them.

## Landing record

1. **Census delta**: bounded claim-parent baseline `rktwynpl` (source-equivalent census `ootnvmsl`): covered faces 1,572, Readings 164,860; after `vpsommzu`: covered faces 1,769, Readings 295,018, newly covered faces 197. Lost faces: none. Every previously covered face retains its count and `readings_sha256`. At `vpsommzu`: no-reading faces 997, unique 523, multiple 1,246; realization/traversal issues, duplicate Readings and internal failures all 0.
2. **Witnesses** (`vpsommzu`, full face / ticket fragment): Cosmic Rebirth 4 / 4; Summon: Esper Maduin 24 / 1; Primevals' Glorious Rebirth 1 / 1; Divine Gambit 4 / 2; Skyshroud Poacher 7 / 7. Each fragment had 0 admitted roots and 0 Readings before implementation on `vpsommzu` with the claim-parent lexicon.
3. **Spot checks** (`vpsommzu`, each 1 Reading): Unnatural Restoration, Profound Journey, Urborg Repossession, Evolution Witness, True Ancestry, Dundoolin Weaver and Curious Forager: singular *permanent* noun Premodifier of *card*; Regenesis: singular *permanent* modifying plural *cards*; Retrieve: *noncreature* outside *permanent card*; Return to the Light Realms: *nonland* outside *permanent cards*. Sample counts and digests match the landing census; each selected *permanent* node is `NounPremodifier` over its original common-noun Lexeme, with *card/cards* as the noun head.
4. **Deviations and STOPs**: no construction/frame additions or deletions and no superseded route. Re-spelled Midnight Tilling's source-PP test against its complete sentence after the nominal gap closed; added independent spell/stacked/head-use/plural-rejection coverage. No glossary gap. At `vpsommzu`, 115 faces containing *permanent card/spell* remain unread; Descend-label cases include Wail of the Forgotten, Akawalli, the Seething Tower, and Uchbenbak, the Great Mistake, outside this ticket. **STOP resolved by owner ruling (2026-10-08)**: shared-data nickname mismatch is inherited from concurrent `english-v3-name-lexeme-scope` catalog regeneration, which that ticket owns; scope stays unchanged and integration is authorized.
5. **Tests** (`vpsommzu`): `cargo test -p deckmaste_english_v3` passed, 518 integration tests; `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` passed. `cargo test -p deckmaste_lexical_source` failed: 23 passed, 1 failed (`card_names::tests::generated_nicknames_equal_the_streamed_computation`); `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` passed. Test changes at `vpsommzu`: restored 0, re-spelled 1 (Midnight Tilling), ignored-with-blocker 0, added 7, removed 0. Both roundtrip laws have independent nominal coverage; corpus evidence validates every counted Reading's realization and traversal.
   1 lexical-source test failed on the shared nickname catalog regenerated by the concurrent english-v3-name-lexeme-scope workspace; inherited, not caused by this change; that ticket owns it.

6. **Timing** (`vpsommzu`): bounded census 42.621 seconds, 12 workers, 2,766 faces: identity manifest selecting every raw-text face containing *permanent* case-insensitively (2,466), plus 300 random previously covered faces outside that surface, seed 20261008. `--samples-per-face 0`; evidence under `target/english-v3/english-v3-permanent-noun-premodifier/`. The bounded run replaces Method's full census under the orchestrator ruling of 2026-10-08; no `--all` run.

