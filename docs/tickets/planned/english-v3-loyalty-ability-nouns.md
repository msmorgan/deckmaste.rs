---
needs: []
---
# Declare *loyalty ability* and the noun *loyalty*, and the bracketed loyalty cost in a quoted ability: Whenever you activate a loyalty ability, you gain 1 life.

## Why

*loyalty* is known only as a counter kind (*loyalty counter*, 59 faces, 47
covered). The compound *loyalty ability* and the characteristic noun *loyalty*
do not read: NP "a loyalty ability" 0 against "a mana ability" 1; "its
loyalty" 0 against "its power" 1. Blocker report at trunk `qzprwuxz`
(`target/english-v3/blockers-wave9.txt`): failing token `loyalty` 24 sole, 32
touched; `at:a loyalty` 5, 11; part of `at:" [` (11 multi-line faces).
`cargo xtask english-v3 grep -i`:

- `loyalty abilit`: **30** faces, all unread; 22 have that line as their only
  failing line.
- *loyalty* as a characteristic (*Nissa's loyalty*, *its starting loyalty*,
  *equal to its loyalty*): 8 unread faces, 4 sole.
- a quoted loyalty ability with its cost in brackets, `"\[[+−-]?\d+\]:`: 15
  faces, all unread; 4 sole (the others also need *loyalty ability* or the
  held *empower*).

Union: 48 unread faces, 29 with exactly one failing line.

## Goal

*loyalty ability* reads as a Compound Noun wherever *mana ability* does;
*loyalty* reads as a relational characteristic noun wherever *power* does; a
loyalty ability quoted inside a sentence reads with its bracketed cost
(*"[+1]: Add {R}."*). Byte-exact realisation.

## Analysis

No CGEL claim is needed for the two declarations (glossary **Compound
Noun**); the bracketed cost is Oracle Notation (glossary **Notation**, **Cost
Component Form**). Project rulings throughout.

Existing declarations: `deckmaste_lexical_source/lexicon/core.ron`,
`compound_noun_classes`, head `lexeme:CommonNoun/Ability` with the single
member `(owner: "lexeme:CommonNoun/ManaAbility", stem: "mana")`;
`vocabulary.rs`, `Power` / `Toughness` (`Relationality = Relational`);
`plugins_v2/builtin/macros/counter_kinds/loyaltyCounter.ron` (`grammar:
FixedTerm(surface: "loyalty")`). Grammar
(`crates/deckmaste_english_v3/src/declarations.rs`): `SignedScalarCost`
(`form [value: ScalarComponent]`, `require loyalty_sign(value.ScalarSign) =
Yes`) and `ZeroScalarCost` (`form ["0"]`) read the unquoted line ("+1: Add
{R}." 1, "−8: Draw a card." 1); `QuotedText` wraps a Document.

Orchestrator rulings 2026-10-11:

- LA1. *loyalty ability*: data. Add a member to the `Ability` compound-noun
  class on the *mana ability* precedent. It is the declared class that
  licenses the compound, so any further kind of ability the corpus needs later
  is one more member row; do not add a rule for "noun + *ability*".
- LA2. *loyalty* the characteristic: data, a relational noun with the
  declared features of `Power` (no plural). One lexical owner per surface
  (2026-10-08 ruling): if the loader refuses a core noun *loyalty* beside the
  counter kind's `FixedTerm(surface: "loyalty")`, export the noun from the
  plugin instead (R-plugin: a bodiless or existing definition's `grammar:`
  row, with the full plugin test set of the Method) and say so in the landing.
  Do not declare both.
- LA3. Bracketed cost: refine `SignedScalarCost` and `ZeroScalarCost` with a
  bracketed form each (`"[", value, "]"` and `"[0]"`). No new Construction
  (R-budget). The grammar does not restrict the bracketed form to quotations
  (R-prune: such a restriction would only block an unattested string); the
  unbracketed form keeps reading where it reads.
- LA4. One analysis per string (R-one). *a loyalty ability* is the compound,
  not the counter-kind term *loyalty* premodifying *ability*; if both arise,
  retire the premodifier analysis by the class's declared features, not by a
  check on the word. *loyalty counter* keeps exactly its Readings. A quoted
  *"[+1]: …"* reads exactly as many ways as the same quote with a `{T}` cost.
- LA5. Newly covered faces are gains. Covered faces containing *loyalty* (47,
  all *loyalty counter(s)*) must keep their counts; an increase there means
  the new noun is competing with the counter-kind term and is a defect unless
  the Reading is a real ambiguity, in which case report its shape.
- LA6. If LA3 needs more than the two forms, land LA1–LA2 (which carry the
  22 and 4 single-line faces of the first two shapes) and return LA3 with
  evidence.

## Witnesses

Whole faces probed 2026-10-11 on the `qzprwuxz` binary (`--category document
--as-card`): all 0. Re-probe on the claim parent first.

- Gideon the Oathless: "Whenever an opponent activates a loyalty ability,
  Gideon deals 1 damage to that player." (sole line)
- The Immortal Sun: "Players can't activate planeswalkers' loyalty
  abilities." (sole line; sentence 0)
- Eidolon of Obstruction: "Loyalty abilities of planeswalkers your opponents
  control cost {1} more to activate." (sole line)
- Nissa, Ascended Animist: "+1: Create an X/X green Phyrexian Horror creature
  token, where X is Nissa's loyalty." (sole line; "X is Nissa's loyalty." 0)
- Liliana's Talent: "Enchanted planeswalker has "[−8]: Put all creature cards
  from all graveyards onto the battlefield under your control."" (sole line;
  'Enchanted planeswalker has "[−8]: Draw a card."' 0)

Also 0: "Whenever you activate a loyalty ability, you gain 1 life."; "Its
starting loyalty is 1."; 'Planeswalkers you control have "[+1]: Add {R}."'.
Controls that must not change: "Whenever you activate a mana ability, you
gain 1 life." 1; NP "a mana ability" 1; NP "mana abilities of permanents you
control" 2; "X is its power." 1; 'Planeswalkers you control have "{T}: Add
{R}."' 1; Document "+1: Add {R}." 1; Document "−8: Draw a card." 1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `\bloyalty\b|\[` (94 and 28 faces) together with every
supported face that begins a line with a signed or zero cost,
`(?m)^[+−-]?\d+: ` (313 faces; R-reach: LA3 alters `SignedScalarCost` and
`ZeroScalarCost`, so every planeswalker face is selected), plus 300 random
covered controls (`cargo
xtask english-v3 diff --manifest … --random 300 --seed 20261008`),
`--workers 6 --samples-per-face 0`, compared with `cargo xtask english-v3
diff` against the claim parent. Zero lost; LA5 for increases; report
digest-only changes by class. Full `cargo test -p deckmaste_lexical_source`
and `-p deckmaste_english_v3` with clippy `--tests -- -D warnings`, once, at
the end; if LA2 falls back to the plugin, add the full `cargo test -p xtask`,
`-p deckmaste_semantics_v2`, `-p deckmaste_construction_core` and `cargo xtask
lean check`. Lexicon data loads at runtime; a grammar edit costs a 2–3 minute
rebuild and the six test binaries about 75 seconds. Tests assert the role
(the compound as head noun, *loyalty* as the possessed characteristic, the
bracketed value as the Cost of the quoted Ability, the LA4 negatives), never
an incidental count or a Debug-printed tree. Attestation by `cargo xtask
english-v3 grep` only.

## Out of scope

- *Empower Jace 2.* and the other held terms of
  `english-v3-plugin-term-schema-questions` (most of the eleven
  bracketed-cost faces that have a second failing line).
- *an additional [+1]* (Carth the Lion), *a Chandra planeswalker* if the
  planeswalker-type premodifier is a separate gap (report, do not chase).
- *twice X loyalty counters* (Confront the Past).
- Cleave brackets: `english-v3-cleave-bracketed-text`.

## Landing record

To be written by the claimant: the six items of `docs/tickets/README.md`.
