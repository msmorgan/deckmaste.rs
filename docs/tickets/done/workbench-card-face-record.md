---
needs: []
---
**Make `Characteristics` its own record; a `CardFace` has characteristics
and whatever its layout adds.** Ruling 2026-09-04 on the alias caveat of
`card-form-characteristics-model`: `Experimental.Card.Characteristics` is
`Characteristics = CardFace`, a transparent alias, so a flip card's
alternative characteristics and an adventure's typecheck as a face and
`FaceLaws`/`Macros.backFace` accept them.

Fix: `record Characteristics` holds the characteristic set [CR#109.3];
`record CardFace` holds a `Characteristics` plus the face-only data a layout
adds (nothing for a single-faced card today; keep the slot positional and
RON-shaped). `FlipCard`'s `alternative` and `Adventurer`'s `adventure` are
typed `Characteristics`; `FaceLaws` is over `CardFace`; a pin refuses
`Macros.backFace` on a flip card, probed non-vacuous. Mirror the split in
core if `CardFace` is still an alias there (`crates/deckmaste_core`), with
the drift guard re-run.

Size: S. Done when: no alias remains; the pin refutes; every face witness
typechecks; workspace tests green; build at its module count. Standard
constraints apply.

## As landed

- **`record Characteristics`** ([CR#109.3]) is now the field-holding record in
  `Experimental.Card` (constructor `MkCharacteristics`, the seven fields the old
  `CardFace` held). **`record CardFace`** (constructor `MkFace`) holds one
  positional slot, `characteristics : Characteristics` — a single-faced card's
  layout adds nothing today, so that is the whole face. No `{…}` handle appears
  in `Cards/*.idr`; the build lint is clean.
- **The alias is gone**: `Characteristics : Type; Characteristics = CardFace`
  is deleted, and no alias replaces it in either direction.
- **`FaceLaws` is over `CardFace`**: `FaceLaws : FaceSide -> CardFace -> Type`,
  `FaceLaws side f = CharacteristicsLaws side f.characteristics`. The law bundle
  itself moved to `data CharacteristicsLaws : FaceSide -> Characteristics ->
  Type` (constructor `MkCharacteristicsLaws`, the same eight `auto 0` fields
  re-pointed at `c.line`/`c.text`/…), so an alternative characteristic set can
  carry laws without being a face.
- **`FlipCard.alternative` and `Adventurer.adventure` are `Characteristics`**
  (already their declared type; their law slots are now
  `CharacteristicsLaws Back alternative` / `CharacteristicsLaws Front adventure`
  instead of `FaceLaws`, which no longer accepts them). `FlipCard`'s
  `nh` reads `normal.characteristics.line`; `Leveler`/`Prototype` read
  `inner.characteristics.line`/`.box`.
- **New macro `Macros.alternative`** (positional, no default slots):
  `name -> cost -> supers -> line -> text -> box -> Characteristics`, cited
  [CR#710.1,715.2]. `Macros.frontFace`/`backFace` still build `CardFace`s and
  are unchanged in signature.
- **New pin `badFaceAsFlipAlternative`** ([CR#710.1] — a flip card's back is the
  normal card back, so its upside-down half is alternative characteristics, not
  a second face): a `failing "Mismatch between: CardFace and Characteristics"`
  block in `ProofsFaces.idr` spelling `FlipCard … (Macros.backFace …)`. Probed
  non-vacuous (message deliberately reversed → "Failing block failed with the
  wrong error").
- **Face witnesses re-spelled**: bench flip/adventure cards
  `merfolkSecretkeeper`, `orochiEggwatcher`, `akkiLavarunner`,
  `bushiTenderfoot`, `kitsuneMystic` now read through `Macros.alternative`;
  `planeswalkerBackWithoutLoyaltyOk`, `jointCrossAbilityChoice`,
  `badTransformingBackWithCost`, `okNamedAdventure`, `badUnnamedAdventure`,
  `okCreatureFlipHalf`, `badSpellFlipHalf` and the 28 `MkFaceLaws impossible`
  pins in `ProofsFaces.idr`/`ProofsPiles.idr` re-spelled to
  `MkCharacteristicsLaws` / `MkFace (MkCharacteristics …)`.
- **Undone: the core mirror.** `crates/deckmaste_core` defines neither
  `CardFace` nor `Characteristics` — the alias `pub type CardFace =
  Characteristics` lives in `crates/deckmaste_card` (`src/card.rs:57`), which
  this round's scope fence excludes. See the STOP below. No Rust file changed,
  so the Rust gates did not apply.

## Landing record

- Idris modules: **46/46 before, 46/46 after**, no `Warning` lines
  (`cd idris && ./scripts/build`, clean `build/` removed first; last line
  `46/46: Building Cards (src/Cards.idr)`).
- `cargo xtask cite check --list-noncompliant`: `0 non-compliant
  citation-looking string(s)`.
- `cargo xtask cite check`: `checked 14202 citations against cr.txt
  (eff. 2026-08-07); 0 stale`.
- `cargo xtask cite bless`: no diff to `cr-citations.lock` — [CR#109.3],
  [CR#710.1] and [CR#715.2] were already registered.
- `jj --no-pager diff --git | cargo xtask cite audit --diff`: every cited rule
  read against the line citing it. [CR#109.3] enumerates the characteristics a
  record now holds; [CR#710.1] states a flip card's back is the normal card
  back and its upside-down half is additional alternative characteristics
  (the pin's claim); [CR#715.2] states an Adventure's inset frame defines
  alternative characteristics.
- Rust gates (`cargo check --workspace --all-targets`, `cargo fmt --check`,
  `cargo test --workspace`): **not run — no Rust file changed** (see the STOP).
- Assurance counts: **restored 0; re-spelled 39** (28 `impossible` pins renamed
  to `MkCharacteristicsLaws`; 2 pins re-spelled through `Macros.alternative`;
  9 positive witnesses re-spelled); **ignored 0; added 1**
  (`badFaceAsFlipAlternative`); **removed 0**.
- Non-vacuity probes (each mis-stated once, message watched to change):
  `badFaceAsFlipAlternative` (reversed `failing` message → "Failing block
  failed with the wrong error"), `badSpellFlipHalf` (`Instant` → `Creature`),
  `badUnnamedAdventure` (`Instant` → `Adventure Sorcery` →
  "not a valid impossible case"), `badTransformingBackWithCost` (cost →
  `Nothing`), `badDuplicateSnow` (`[Snow, Snow]` → `[Snow]`, as a sample of the
  28 renamed pins; Idris's own coverage check rejects a vacuous `impossible`
  clause, so the rename cannot silently defuse one).
- Deviations and additions:
  - `CharacteristicsLaws` is a new name the ticket does not spell. It is
    forced: the ticket requires `FaceLaws` to be over `CardFace` while
    `FlipCard.alternative`/`Adventurer.adventure` are `Characteristics` and
    still need the same law bundle, so the bundle has to be nameable over
    `Characteristics`. `FaceLaws` stays a `CardFace`-indexed name, defined as
    that bundle at the face's characteristics.
  - `Macros.alternative` added (one macro, positional, serving both
    [CR#710.1] and [CR#715.2] alternative-characteristics slots) because the
    bench lint forbids implicit handles in `Cards/*.idr` and no existing macro
    returns `Characteristics`.
- STOP taken: **the ticket's "mirror the split in core" instruction has no
  subject in `crates/deckmaste_core`.** The Rust alias is
  `deckmaste_card::CardFace = Characteristics`, and splitting it is a
  749-reference change across 11 crates (engine, plugin, lowering, migrations,
  noncanon, tui, xtask, legacy_render, macro_ron) plus the serde shape of every
  card RON — outside this round's scope fence and outside a Size-S ticket.
  Resolution: the Idris split landed in full, Rust left untouched and reported
  for a separate ticket; the drift guard was therefore not re-run.
