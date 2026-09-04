---
needs: []
---
**Replace the catch-all `Normal | TwoFaced { layout, front, back }` model with
CR-faithful Card Form and Characteristics vocabulary.** The target-vocabulary
definitions are [`Card Face`, `Characteristic`, and `Alternative
Characteristics`](../../contexts/game-model/CONTEXT.md). `CardFace` must denote only something
the CR calls a face; `Characteristics` is the shared value used wherever a
face or rules-defined alternative supplies characteristics.

Scope (ruling 2026-09-04): the Idris workbench, core, engine, and lowering's
core-facing output only; `deckmaste_semantics` and `idris/src/Semantics.idr`
are deletion-bound and untouched; their side converges at cutover when the RON
re-emit path points at `Experimental`.

Represent these forms distinctly rather than treating all of them as
two-faced cards:

- nonmeld double-faced cards have front and back Card Faces; the meld
  oversized-face exception must not be forced into the same pair
  ([CR#712.1]);
- split cards have two card faces on one side ([CR#709.1]);
- flip cards have one card face with normal and alternative characteristics
  used according to flipped status ([CR#710.1]); and
- adventurer cards have normal characteristics plus alternative Adventure
  characteristics, not an Adventure face ([CR#715.2]).

Choose the smallest sum and product shapes that preserve those distinctions in
`deckmaste_core` and the Idris workbench (`idris/src/Experimental/`) and in the
core-facing RON. Migrate lowering's core-facing output, card lookup, generated
fixtures, and all layout consumers.

Overlaps the workbench card-shape tickets: `workbench-levelers` and
`workbench-prototype` add inner-face wrappers beside
`SingleFaced`/`Transforming`/`Adventurer`, and `workbench-supplements-out`
trims the workbench card-type set; whichever runs second reconciles the names.

Acceptance includes one double-faced, split, flip, and Adventure witness in
`deckmaste_core` and the workbench, with `cd idris && ./scripts/build` green at
its module count; each witness exposes exactly the faces and alternative
Characteristics its CR form permits. Do not preserve the old model through
compatibility aliases.

## As landed

- **Double-faced** ([CR#712.1]): core `deckmaste_card::Card::DoubleFaced {
  layout: DoubleFacedLayout, front: CardFace, back: CardFace }` —
  `DoubleFacedLayout` narrowed to `{ Transforming, ModalDfc }`, so meld's
  oversized-face exception is never forced into the pair (meld itself stays
  unmodeled, as before). Workbench: `Experimental.Card.Transforming` /
  `ModalDfc` (already present, front/back `CardFace`s with `FaceLaws`).
  Witness: Delver of Secrets // Insectile Aberration
  (`deckmaste_card::card::tests::double_faced_witness_has_front_and_back_card_faces`;
  workbench `Cards.Faces.invasionOfDominaria`/`arlinnKord`/`akkiLavarunner`
  family already carry `Transforming`).
- **Split** ([CR#709.1]): core `Card::Split { left: CardFace, right: CardFace
  }` — two Card Faces on one side, no layout tag. Workbench:
  `Experimental.Card.SplitCard` (already present). Witness: Fire // Ice
  (`deckmaste_card::card::tests::split_witness_has_two_card_faces_on_one_side`;
  workbench `Cards.Faces.profitLoss` (Profit // Loss, `SplitCard`) —
  `Cards.Faces.glassworksShatteredYard` is the separate `SharedLineSplit`
  room-door representation, unaffected by this ticket).
- **Flip** ([CR#710.1]): core `Card::Flip { normal: CardFace, alternative:
  Characteristics }` — one face plus Alternative Characteristics, never a
  second face. Workbench: `Experimental.Card.FlipCard`'s `alternative`
  param retyped from `CardFace` to `Characteristics` (a transparent alias of
  `CardFace`, so `FaceLaws`/`Macros.backFace` call sites are unchanged).
  Witness: Bushi Tenderfoot // Kenzo the Hardhearted
  (`deckmaste_card::card::tests::flip_witness_has_one_face_and_alternative_characteristics`;
  workbench `Cards.Faces.bushiTenderfoot`/`akkiLavarunner`/`kitsuneMystic`).
- **Adventurer** ([CR#715.2]): core `Card::Adventurer { normal: CardFace,
  adventure: Characteristics }` — normal characteristics plus alternative
  Adventure characteristics, not an Adventure face. Workbench:
  `Experimental.Card.Adventurer`'s second param renamed `inset` ->
  `adventure` and retyped `CardFace` -> `Characteristics` (same alias, zero
  call-site churn). Witness: Merfolk Secretkeeper // Venture Deeper
  (`deckmaste_card::card::tests::adventurer_witness_has_normal_and_alternative_adventure_characteristics`;
  workbench `Cards.Faces.merfolkSecretkeeper`).
- **Normal** (single-faced, no special form): core `Card::Normal(CardFace)`
  unchanged in shape. Workbench: `Experimental.Card.SingleFaced` unchanged.
- **Shared value**: core `deckmaste_card::Characteristics` is the renamed
  field-holding struct (was `CardFace`); `pub type CardFace =
  Characteristics` is the face-only name, so every existing `CardFace { .. }`
  construction/pattern site keeps compiling unchanged. Workbench mirrors this
  exactly: `Experimental.Card.Characteristics : Type; Characteristics =
  CardFace`. `Card::primary_face(&self) -> &CardFace` replaces the old
  `Card::Normal(f) | Card::TwoFaced { front: f, .. } => f` idiom at every
  call site (engine, plugin, noncanon) with the front/left/normal face across
  all five forms.
- Nothing undone. Meld ([CR#712.4]) stays unmodeled (as before this ticket;
  no `FaceLayout`/`DoubleFacedLayout` variant claims it), per the "not forced
  into the same pair" instruction.
- The overlapping workbench tickets (`workbench-levelers`,
  `workbench-prototype`, `workbench-supplements-out`) had already landed
  their `ModalDfc`/`SplitCard`/`SharedLineSplit`/`FlipCard`/`Adventurer`
  constructors in `Experimental.Card` before this round started; this ticket
  only fixed the `CardFace`-vs-`Characteristics` typing on `FlipCard` and
  `Adventurer`'s alternative parameter and did not need to reconcile any
  names.

## Landing record

- `cargo check --workspace --all-targets`: clean (0 errors).
- `cargo fmt --check`: clean.
- `cargo test -p deckmaste_core -p deckmaste_lowering -p deckmaste_engine`:
  **1902 passed, 0 failed, 4 ignored** (pre-existing ignores, untouched by
  this round).
- `cd idris && ./scripts/build`: **44/44** modules, no Warning lines.
- `cargo xtask cite check --list-noncompliant`: 0 non-compliant.
- `cargo xtask cite check`: 0 stale (17933 citations checked).
- `cargo xtask cite bless`: re-blessed twice (once per citation addition
  round); both times produced **no diff** to `cr-citations.lock` — every
  cited rule ([CR#109.3,709.1,710.1,712.1,712.4,712.8,712.8a,712.8d,715.2])
  was already registered.
- `jj diff --git | cargo xtask cite audit --diff`: 21 citation sites audited,
  every rule's text matches the claim citing it (one fix made mid-round: the
  `primary_face` doc originally cited only [CR#712.8d] for both the
  off-battlefield and on-battlefield claims — split into
  [CR#712.8a] (off-battlefield) + [CR#712.8d] (on-battlefield, front face
  up) after the audit read).
- `plugins/wizards`: not regenerated — no macro/RON stub vocabulary changed
  (no card RON under `plugins/builtin_v2` authors a card form; the only
  finished-card RON touched, `plugins/canon/cards/Delver of Secrets.ron`,
  changed only in a doc comment).
- Assurance counts (`deckmaste_lowering/src/card.rs`): **restored 0;
  re-spelled 6** (the 5 standalone `Lower for FaceLayout` variant tests plus
  `lowers_card_two_faced` — that impl no longer exists as a separate
  function once layout selects the Card variant, so their subject is
  retired; re-spelled as 5 new tests, one per form, asserting the same
  semantic-input fixtures now produce the correct `Card` variant);
  **ignored 0; added 4** (double-faced/split/flip/adventurer witnesses in
  `deckmaste_card::card::tests`, new coverage — no prior subject); **removed
  0** (no test deleted; every retired subject was re-spelled).
- Deviations and additions:
  - Added `Card::primary_face(&self) -> &CardFace` (not requested by name in
    the ticket) to collapse the repeated
    `Card::Normal(f) | Card::TwoFaced { front: f, .. } => f` idiom, since
    that pattern is no longer exhaustive against 5 variants; used at ~30
    call sites across `deckmaste_engine`, `deckmaste_plugin`, and
    `deckmaste_noncanon`. Justification: smallest-diff way to keep every
    existing "front/primary face" reader correct without hand-enumerating
    the OR-pattern at each site.
  - Touched `deckmaste_plugin` (`plugin.rs`, `validate.rs`,
    `tests/corpus_identity.rs`, `tests/testing.rs`), `deckmaste_noncanon`
    (`probe.rs`, `deck.rs`), and `deckmaste_tui` (compiled clean, no source
    change needed) — outside the ticket's named scope list, but required by
    the `cargo check --workspace` gate since they pattern-match
    `deckmaste_card::Card` directly. All edits are mechanical (match-arm
    extension to the new variants, or a `primary_face()` call), no behavior
    change for the `Normal`/`DoubleFaced` cases these crates actually
    exercise; their test suites (`cargo test -p deckmaste_plugin -p
    deckmaste_noncanon -p deckmaste_tui`, not part of the ticket's gate)
    were run as an extra check and are green (275 passed, 0 failed).
  - Renamed `deckmaste_card`/`deckmaste_lowering`'s `FaceLayout` to
    `DoubleFacedLayout` (2 variants: `Transforming`, `ModalDfc`) since it now
    tags only the `DoubleFaced` variant; `Split`/`Flip`/`Adventurer` carry no
    layout field.
- No STOP taken.
