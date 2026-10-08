---
needs: []
---
# Scope shortened card names to their own card, and declare each subtype once

Priority: high.

## Why

A shortened card name ("Chandra") is admitted on every face as a surface
variant of every full name it shortens. The bare word "Chandra" therefore has
**21** lexical alternatives in the english-v3 environment: **19** card-name
catalog lexemes (variant 1, the shortened name, of each of the 19 Chandra cards
the nickname catalog lists), one `catalog:planeswalker-types.txt/Chandra`
lexeme and one `lexeme:planeswalker_subtype/chandra` noun. In a subject
position the 19 name lexemes all read, so "Chandra deals 2 damage to any
target." has **19** Readings, each claiming a different card.

On the trunk census at the signed-numeral-cost landing
(`target/english-v3/english-v3-signed-numeral-cost/after.json`, trunk change
`xnkuznpo`: **23,459 covered / 1,605,535 Readings**), Chandra, Pyromaster
carries **246,240** Readings for one *Chandra* mention (19 × 12,960). By a
surface scan of that census, **625** covered faces / **614,305** Readings
contain a shortened name that several cards share. That bounds the faces whose
counts may fall. A further **636** covered faces / **71,967** Readings contain
another card's shortened name. Most are homographs such as *Dragon* and
*Beast*, which keep their other readings.

Owner ruling (2026-10-08): "cards should only ever have their *own* nicknames
in scope. full names can be used anywhere still. And why are 'planeswalker
type' and 'subtype' different?" The rules agree. Text printed on a card that
uses its shortened name refers to that card [CR#201.5c]. Planeswalker types
are subtypes [CR#205.3j], as are the other per-type lists [CR#205.3].

Line numbers are those of the claim parent this ticket was minted on.

## Analysis

**Where the shortened names come from.** `cargo xtask catalogs generate`
(`crates/xtask/src/catalogs.rs:125–139`) writes the canonical catalogs and
`data/gen/catalogs/card-name-nicknames.json` through
`CardNameNicknames::generate` (`crates/deckmaste_catalogs/src/nicknames.rs:40–69`).
The generator keeps legendary permanents (`:133`) and records a short name
(`:219`) only when that card's own Oracle text uses it (`attested_short_name`,
`:149`). The map is keyed by the oracle unit's display name (`:47–51`) and
holds **1,806** entries over **1,393** distinct short names. **242** of those
short names are shared by more than one card (655 entries). The largest
groups are Chandra 19, Captain America 8, and Ajani, Gandalf, Gideon and Sorin
at 7 each.

**How the lexicon admits them globally.** `deckmaste_lexical_source::card_names::load`
(`crates/deckmaste_lexical_source/src/card_names.rs:8–14`) returns the map.
`load_workspace` attaches each short name as a second surface of that card's
full-name lexeme and marks the lexeme `IdentityUse = Name`
(`crates/deckmaste_lexical_source/src/lib.rs:70–78`). The census builds **one**
`Lexicon` for the whole corpus (`crates/xtask/src/english_v3/mod.rs:111–113`).
`analyze_face` calls `lexicon.analyze_source(raw, normalized)` with no face
context (`:224–236`). The analyzer is one trie walk over every declared
surface (`crates/deckmaste_lexical/src/analysis.rs:460–480`). Nothing between
the face and the lexicon is per-face, so every face sees every card's short
name. The probe does the same thing (`crates/xtask/src/english_v3/probe.rs:99–100,114`).
The face identity needed for scoping is available at `analyze_face`:
`SelectedFace` carries `oracle_id`, `name` and `face_name`
(`crates/xtask/src/raw_corpus.rs:139–148`). Grammar sees names only through
`CatalogName` (`require head.IdentityUse = Name`,
`crates/deckmaste_english_v3/src/declarations.rs:2389–2392`). No construction
needs to change for item 1.

**Why a planeswalker type is two lexemes.** There are two independent sources:

- **The CR catalog.** `deckmaste_catalogs` extracts the CR subtype lists
  (`crates/deckmaste_catalogs/src/cr.rs:37,96`) into
  `data/gen/catalogs/planeswalker-types.txt` (80 lines) and the six sibling
  `*-types.txt` files. `load_workspace` turns each line into an invariant
  `Category::Catalog` lexeme with `TypeLineRole = Subtype`
  (`crates/deckmaste_lexical_source/src/lib.rs:86–99`). Its only consumers are
  the type-line constructions `Subtypes` and `SubtypeContinuation`
  (`declarations.rs:1574–1582`, `lexical(Catalog)` +
  `require head.TypeLineRole = Subtype`).
- **The plugin declaration.** `plugins_v2/builtin/macros/subtypes/planeswalker/chandra.ron:1–8`
  is `Subtype(category: Planeswalker, …, grammar: Noun(singular: "Chandra",
  plural: Unavailable))`. It is read by `legacy::plugins::load`
  (`crates/deckmaste_lexical_source/src/legacy/plugins.rs:30`) and exported as
  a count noun `lexeme:planeswalker_subtype/chandra` (`:148–167`) with no
  `TypeLineRole`. Rules-text nominal grammar consumes it ("a Chandra
  planeswalker").

The two lexemes are not byte-identical. The catalog lexeme is invariant,
`Capitalization::Exact`, opaque and carries `TypeLineRole = Subtype`. The
plugin lexeme is a count noun with singular and plural slots and grammar
features. In rules text the catalog lexeme is a lexical alternative that no
construction selects. "Search your library for a Chandra planeswalker card."
lists both at 26–33, and all 4 of its Readings use the plugin noun. The split
is the same for every subtype kind: plugin vs CR counts are artifact 21/23,
battle 1/1, creature 324/324, enchantment 13/13, land 17/17, planeswalker
80/80 and spell 5/5. The two artifact types with no plugin declaration are
*Attraction*, which is out of scope, and *Heartwood*, which is token-only.

The CR catalogs are also the membership authority for the planned
[builtin-v2-catalog-coverage-validator](builtin-v2-catalog-coverage-validator.md).
They therefore stay generated. Only their export as lexemes is a duplicate.

## Items

### 1. A shortened name is in scope only on its own card

The full-name surface stays admitted on every face. It covers self-reference,
"a card named …", and references to other cards by full name. The
shortened-name surface (variant 1 of a card-name lexeme) is admitted only when
the text being analysed belongs to the card that the short name belongs to.

Mechanism:

- **Declared in lexical data.** When `load_workspace` attaches a short name
  (`lib.rs:70–78`), it marks that surface variant as card-scoped. This is a
  property of the form's surface variant in `deckmaste_lexical`, carried in
  the serialized `Lexeme`. It is not a grammar feature, and no construction
  reads it.
- **Applied in lexical matching.** `Lexicon` gains a scoped analysis entry
  point that takes the set of lexeme identities whose card-scoped variants are
  admitted. Trie values for a card-scoped variant of any other lexeme are not
  emitted as candidates. The existing `analyze`/`analyze_source` become the
  empty-scope case, so they admit full names only.
- **Scope = the analysed card.** `analyze_face` builds the scope from the
  face's card. That is the full-name lexemes of every oracle unit sharing the
  face's `oracle_id`, keyed by the same oracle-unit display name the generator
  used. Text on one face of a multi-face card may name another face of the same
  card, and that card is "that card" in [CR#201.5c]. Text granted to another
  object (a quoted token ability) is still printed on this card
  [CR#201.5a,201.5c], so it is covered by the same scope.
- **The probe takes a scope flag.** `cargo xtask english-v3 probe --as-card
  <card name> --text …` analyses the text as if printed on that card: the
  scope is built exactly as `analyze_face` builds it (every oracle unit
  sharing that card's `oracle_id`). Without the flag the probe is the
  empty-scope case and admits full names only. The value follows the spelling
  of the census `--card-name` selector (`SelectedFace` card names in
  `crates/xtask/src/raw_corpus.rs`).
- No guard names a card, lexeme or construction. `CatalogName` is unchanged.

A Reading that used another card's short name is a wrong analysis and may be
lost. A face that loses **all** its Readings is a defect: find which name it
needed. If the face's own name is missing from the nickname map, the fix is in
the generator (`nicknames.rs`), not in a scope exception.

### 2. One lexeme per subtype, declared as the subtype it is

Retire the catalog-derived subtype lexemes for all seven subtype kinds. The
same arm (`lib.rs:86–99`) emits them all, and all seven are subtypes
[CR#205.3]. The plugin `Subtype` declaration becomes the single lexeme:

- **Consumer side.** `legacy::plugins::load` marks every
  `DeclarationKind::Subtype(_)` lexeme `TypeLineRole = Subtype`.
  `load_workspace` stops emitting lexemes for the seven `*-types.txt`
  catalogs. `Subtypes`/`SubtypeContinuation` (`declarations.rs:1574–1582`)
  take the plugin noun's singular form instead of `lexical(Catalog)`, still
  gated only by `require head.TypeLineRole = Subtype`. Supertype and card-type
  catalog lexemes are untouched.
- **Generator side.** `catalogs generate` keeps writing the `*-types.txt`
  files as the CR membership authority for the coverage validator. Their
  lexical export is retired, and `crates/deckmaste_lexical_source/README.md`
  ("Type Line distribution", `:35–39`) is updated to say that subtype role
  comes from the plugin declaration.
- A type-line face that loses its Reading because a CR subtype has no plugin
  declaration (*Heartwood* is the known gap) gets that declaration. *Attraction*
  is out of scope and gets none.

## Witnesses

Probe counts were taken at the claim parent with
`cargo xtask english-v3 probe --category Sentence`. Face counts come from the
`after.json` census above. "Expected after" assumes the short name is the
face's only multiplier at that span. The census decides.

| Witness | Now | Lexical alternatives at the name | Expected after |
| --- | ---: | --- | ---: |
| "Chandra deals 2 damage to any target." (probe) | 19 | 19 card-name + PW-type catalog + PW subtype noun | 0 unscoped; 1 with `--as-card "Chandra, Pyromaster"` |
| "Jace draws two cards." (probe) | 4 | 4 card-name + PW-type catalog + PW subtype noun | 0 unscoped; 1 with `--as-card "Jace, Cunning Castaway"` |
| "Ajani deals 2 damage to any target." (probe) | 7 | 7 card-name + PW-type catalog + PW subtype noun | 0 unscoped; 1 with `--as-card "Ajani Vengeant"` |
| "Gandalf deals 2 damage to any target." (probe, legendary creatures) | 7 | 7 card-name | 0 unscoped; 1 with `--as-card "Gandalf the Grey"` |
| "Niv-Mizzet deals 1 damage to any target." (probe) | 5 | 5 card-name | 0 unscoped; 1 with `--as-card "Niv-Mizzet, Parun"` |
| "Search your library for a Chandra planeswalker card." (probe, control) | 4 | PW subtype noun selected; catalog lexeme admitted, unused | 4 |
| Chandra, Pyromaster (face) | 246,240 | 19 per mention | 12,960 |
| Chandra, Flame's Fury | 6,859 | 19³ | 1 |
| Chandra, Pyrogenius / Chandra, Bold Pyromancer | 13,718 each | 19³ × 2 | 2 each |
| Chandra, Flame's Catalyst | 30,324 | 19 × 1,596 (one mention; measured) | 1,596 |
| Chandra Ablaze | 418 | 19 × 22 | 22 |
| Sorin, Vengeful Bloodlord | 168 | 7 × 24 | 24 |
| Edgar Markov | 288 | 6² × 8 (two mentions; measured) | 8 |
| Niv-Mizzet, Parun / Niv-Mizzet, the Firemind | 10 / 5 | 5 per mention | 2 / 1 |

Implementation STOP (2026-10-08): Jace, the Mind Sculptor and Ajani Goldmane
have no attested shortened-name variant. The accepted independent-lexical-analysis
contract admits only declared forms, so their scoped probe counts remain zero.
The positive witnesses use Jace, Cunning Castaway and Ajani Vengeant, whose
shortened spellings are declared, preserving the existing generator's attestation
rule. No unattested variants are introduced.

For the probe rows, the scoped count of 1 (one name alternative, so one
Reading for the simple sentence) assumes no other multiplier in the sentence.

The type-line census (`--field type-line`) is the control for item 2. Every
face keeps its count. Subtype positions change to their sole plugin-noun
lexical identity, so their Reading digests change accordingly.

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, scope).
Deltas:

1. Verification budget (binding, user instruction 2026-10-08): one full text
   census (`--all --workers 12 --samples-per-face 0`) on the finished feature,
   before refresh. Compare each face's count and `readings_sha256` against the
   claim parent. Iterate on explicit selector subsets. The subsequent owner ruling also
   requests a full type-line census, as in the original ticket method. Test and clippy the changed
   crates; no xtask tests or Lean checks. After refresh, follow the user's
   bounded verification and immediate integration instructions. Evidence goes
   under the workspace's ignored `target/english-v3/`, never `/tmp`.
2. Write the witnesses as tests first. Re-spell
   `legendary_nicknames_are_reversible_variants_of_the_full_name`
   (`lib.rs:201`) against the scoped entry point. It keeps the same four
   cards and the same asserted spellings. Then add the negative: another
   card's scope admits no short name. Add a probe test or xtask unit test
   that `--as-card` scopes exactly as `analyze_face` does, i.e. the two paths
   share one scope-building function; a second copy of the scope logic is a
   review finding.
3. Zero lost faces. A Reading count may fall only on a face whose analysed
   text contains a short name with more than one card-name alternative, or a
   foreign short name. Name each face with a nonzero drop in the census delta
   by count, not by list.
4. No guard may name a lexeme, construction, card or word. Scope is lexical
   data plus face identity. No new xtask subcommands or fixtures; the one new flag is the probe scope
   flag in item 1 (owner decision 2026-10-08).
5. CR citations are already in this ticket. Re-run
   `cargo xtask cite check --list-noncompliant` on anything added.
6. Record the owner ruling as an amendment in
   `docs/decisions/english-lexical-analysis.md` ("Shortened names are
   card-scoped"). Then add the glossary term the landing needs to
   `docs/contexts/oracle-english/CONTEXT.md` through `domain-modeling`.
   Proposed term: **Shortened Name**, citing [CR#201.5c]. Avoid
   *nickname* in identifiers.

## Out of scope

- **Determiners on names (checked, no item needed).** A proper name on a card
  is a bare NP or the complement of *named*. Card-name lexemes already reject
  determiners. "Return a Gandalf to its owner's hand." reads **0**. "Exile
  that Sorin." and "Destroy each Chandra." read **1** each, but both Readings
  are `DeterminedNounPhrase` over the *subtype noun*
  (`lexeme:planeswalker_subtype/sorin`, `…/chandra`), not a name. Supported
  Oracle text attests a subtype noun heading a determined NP: Garruk, Cursed
  Huntsman ("each Garruk you control"), Countersculpt ("behold a Jace",
  "choose a Jace you control or reveal a Jace card"), and **31** cards with "a
  Jace token you control". This is the description use of a subtype
  [CR#109.2]. Under the pruning rule those Readings stay. Do not re-probe.
- Shortened names that are also common words or creature types (*Dragon*,
  *Beast*, *Sliver*, *Storm*) are a generator-quality question for
  `nicknames.rs`. Item 1 already confines them to their own card.
- The legacy `deckmaste_english` stack and `catalogs text` (legacy catalogs)
  are unchanged.
- Supertype and card-type catalog lexemes.

## Resolved implementation STOP

Owner ruling (2026-10-08): declare NameNumber on the full-name lexeme and
read it through a table, minimally changing CatalogName's number export.
Declare Wasp as a full token name, never as a scope exception. No generated
token-name catalog exists in data/gen/catalogs or deckmaste_catalogs; the
owner permits core.ron for this ticket and routes the proper generated home
to recon. Both repairs must preserve the intended analysis.

## Landing record

1. **Census delta:** pre-refresh claim parent `vnzxpvuk` → feature `zntxxpru`: covered faces
   23,658 → 23,658; total Readings 1,672,796 → 1,094,004. Lost faces: none.
   On `zntxxpru`, 578,792 wrong foreign-name Readings are retired across 335
   faces; all decreases are justified face by face in the ignored
   `target/english-v3/english-v3-name-lexeme-scope/decreased-faces.json`.
   Roundtrip/traversal issues, internal failures and unexplained decreases: 0
   (`zntxxpru`). Parent provenance is in `baseline-provenance.json` there.
   After refresh, the bounded 6,300-face check on `yywzqzyt` has 0 losses,
   decreases, issues or internal failures; inherited `lzttwzxx`/`vpsommzu`
   add 407 covered faces and 83,653 Readings. Reconciled totals are 24,065
   covered faces and 1,177,657 Readings (`yywzqzyt`, `refresh-comparison.json`).
2. **Witnesses** (`zntxxpru`): the Chandra/Jace/Ajani/Gandalf/Niv-Mizzet
   sentence probes each have 0 unscoped and 1 in their declared card scope;
   the Chandra subtype search control has 4. Jace, the Mind Sculptor and
   Ajani Goldmane admit no unattested variant (0). Face Readings: Chandra,
   Pyromaster 12,960; Flame's Fury 1; Pyrogenius 2; Bold Pyromancer 2;
   Flame's Catalyst 1,596; Chandra Ablaze 22; Sorin, Vengeful Bloodlord 24;
   Edgar Markov 8; Niv-Mizzet, Parun 2; the Firemind 1. Repair witnesses:
   Aang and Katara 10 (plural full-name Subject), The Hive 21 (Wasp as the
   Name complement of named), Shang-Chi and the Ten Rings 3 (own Shortened
   Name). Either Garruk face scope admits Garruk Relentless's declared
   Shortened Name.
3. **Spot checks:** no new faces from this ticket alone (`zntxxpru`);
   its ten scoped-Name checks remain in `spot-checks.json`. Ten newly covered
   faces inherited on refresh (`yywzqzyt`) were checked: Dazzling Sphinx
   (exiled cards, participial Premodifier); Ramosian Commander (Rebel
   permanent card, noun Premodifier); Unnatural Restoration (permanent card,
   noun Premodifier); Beluna Grandsquall (Permanent spells, noun Premodifier);
   Seek Thrills (milled cards, participial Premodifier); Cache Grab (permanent
   card, noun Premodifier); Genesis Ultimatum (permanent cards, noun
   Premodifier); Profound Journey (permanent card, noun Premodifier); Nexus
   of Becoming (exiled card, participial Premodifier); Abstruse Archaic
   (activated or triggered ability, coordinated participial Premodifiers).
   Each selected analysis is byte-exact and issue-free (`yywzqzyt`,
   `refresh-spot-checks.json`).
4. **Deviations and STOPs:** scoped realization contexts and independent
   lexical boundary validation preserve both roundtrip laws. The generator
   restores Shang-Chi's attested Shortened Name. The outside-ticket STOP was
   resolved by the owner's 2026-10-08 ruling: NameNumber is declared on the
   Aang and Katara lexeme, mapped through a table, minimally exported by
   CatalogName and propagated through ProperNameNounPhrase; Name declares
   its number feature. Wasp is a full Name in core.ron, never a scope
   exception. No token-name catalog exists; 63 distinct capitalized token
   names following named were audited in the supported corpus (`zntxxpru`,
   `named-token-inventory.json`); a generated token-name catalog is their
   proper home, routed to recon. The unattested Jace/Ajani witness STOP was
   resolved under the declared-forms contract by using Cunning Castaway and
   Vengeant. The census corrects the ticket's Edgar/Flame's Catalyst estimate
   multipliers. All seven subtype catalog lexical routes are retired and
   their singular noun routes replace them; no frame was added or deleted.
   Shortened Name and Name Number glossary gaps are filled. CGEL p. 516
   supports NP status only; declared agreement is cited as the owner ruling.
   Refresh's test-registration conflict was resolved by retaining both the
   inherited and this ticket's suites; only trailing whitespace was formatted.
5. **Tests** (`zntxxpru`): `cargo test -p deckmaste_english_v3` passed (508);
   `cargo test -p deckmaste_lexical -p deckmaste_lexical_source -p
   deckmaste_construction_v3_core -p deckmaste_catalogs` passed (59, 59, 27,
   36 respectively). `cargo clippy -p deckmaste_english_v3 -p
   deckmaste_lexical -p deckmaste_lexical_source -p
   deckmaste_construction_v3_core -p deckmaste_catalogs -p xtask --tests -- -D
   warnings` passed. Test-function counts (`zntxxpru`): restored 0,
   re-spelled 15, ignored-with-blocker 0, added 7 (the xtask unit test was
   compiled only), removed 0. The Vampire case moved out of
   `grammatical_type_nouns_remain_distinct_from_exact_catalog_atoms` was
   re-spelled into
   `subtype_nouns_are_the_single_owner_for_type_line_and_rules_text`.
   No assertion was weakened; no xtask tests or
   Lean checks ran. Citation checks: 0 noncompliant and 0 stale (`zntxxpru`).
   After refresh (`yywzqzyt`), `cargo test -p deckmaste_english_v3 --test main`
   with filters `name_lexeme_scope`, `noun_premodifier`,
   `past_participial_premodifiers`, `from_preposition_phrase_complement`
   passed (4, 20, 7, 9 respectively); `cargo test -p
   deckmaste_lexical_source` passed (59); `cargo clippy -p
   deckmaste_english_v3 -p deckmaste_lexical_source --tests -- -D warnings`
   passed. These filtered post-refresh English test runs missed the same-as
   Circu witness, now re-spelled by that ticket's follow-up. No full English
   suite or full census was repeated after refresh. Formatting and record
   follow-up (`pqqywyqz`): `cargo fmt -p xtask` completed and
   `cargo clippy -p xtask --tests -- -D warnings` passed; no census ran.
6. **Timing** (`zntxxpru`): the one full text census took 193.096144 seconds
   wall time, including setup, with 12 workers.
7. **Readings total before/after:** the full text census before refresh at `vnzxpvuk` → `zntxxpru`:
   1,672,796 → 1,094,004 Readings; 23,658 → 23,658 covered faces. Type lines:
   32,828 → 32,828 Readings and covered faces. Parent type-line totals are
   projected from one measured representative of each distinct supported
   type line (3,594, source-equivalent to `vnzxpvuk`); feature totals are from
   the full type-line census on `zntxxpru`. Item 1 removes 578,792 Readings;
   item 2 changes no type-line or rules-text count, only subtype identity in
   Type Lines. The bounded reconciliation after inherited changes gives
   24,065 covered faces and 1,177,657 Readings (`yywzqzyt`), rather than a
   second full run. Evidence is under the ignored ticket target directory.
