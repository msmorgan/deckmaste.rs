---
needs: [workbench-levelers]
---
**Give the `Card` sort a prototype wrapper: `Prototype (inner : CardFace)
(alt : (cost, box))` [CR#702.160a].** Fresh workbench review 2026-09-03, R3,
resolved by ruling.

**Ruling (settled 2026-09-03): prototype is an inner-face wrapper, the same
idiom as the leveler.** A prototype card is one face with a second, smaller
mana cost and printed box; it is not two faces, so `Transforming`/`ModalDfc`
are the wrong shape and `CardFace.cost : Maybe ManaCost` /
`CardFace.box : Maybe PrintedBox` are one each. The constructor wraps one
`CardFace` and carries the alternative pair. The lowering shape is
`Layout::Prototype { inner, alt }`.

19 supported cards print Prototype (e.g. Arcane Proxy).

Depends on `workbench-levelers`: that ticket lands the inner-face wrapper
idiom — how a wrapper interacts with `Card.FaceLaws`, what the wrapped face
owes, and how the lowering names it — and this one follows it rather than
inventing a second answer.

The alternative's type line is *not* a slot: prototype changes cost, colour,
and the printed box, and the round records which of those the pair carries
against [CR#702.160a] rather than assuming. Every slot is positional and
required, per `docs/decisions/card-authoring-binds-no-implicits.md`.

Size: S–M once the leveler wrapper exists.

Done when: Arcane Proxy is a typechecking bench witness with both costs and
both boxes; a pin refuses a prototype whose alternative cost is absent or
whose alternative box contradicts the CR's prototype rules, probed
non-vacuous; the wrapper reads through the same face laws the leveler wrapper
established; the build is 44/44 with 0 errors and 0 warnings. Standard
constraints apply, plus the RON-shaped constraint: a core constructor is
admissible only if the RON re-emitter can produce it from a RON node, and a
macro only if it names a RON macro
(`docs/decisions/workbench-ron-shaped-and-label-rulings.md`).

## As landed

**What the alternative pair carries, decided from the rule.** [CR#702.160a]
gives a prototype card "a secondary set of power, toughness, and mana cost
characteristics", and [CR#718.1] puts exactly those in the inset frame. The
pair is therefore two slots, both positional and required:

- **mana cost — carried.** [CR#702.160a,718.1]; a prototyped spell is cast
  using only its alternative mana cost [CR#718.3a].
- **printed box — carried**, as the second power and toughness
  [CR#702.160a,718.1].
- **colour — not a slot.** A prototyped spell's colour is *derived* from the
  coloured symbols of the alternative mana cost [CR#718.3b]; that is why
  [CR#718.5] mentions colour only parenthetically among the characteristics
  that change. Nothing on `CardFace` carries a colour either — every face's
  colour is read off its cost — so the round's brief's second pin (an
  alternative whose colour disagrees with its cost) has no term to refuse:
  the disagreement is unrepresentable, not refusable. Relatedly, the rule the
  brief names for that pin is not a rule: [CR#702.160a] is the only subrule of
  its rule, and there is no b. Said rather than forced, per the brief; the
  ticket's own second pin — an alternative box that contradicts the prototype
  rules — is what landed in its place.
- **The type line is not a slot**, confirmed by [CR#718.5]: a prototype card's
  characteristics other than power, toughness, mana cost and colour are the
  same either way. So the alternative box is checked against the *inner face's*
  type line, by the same `CardBox` law the face and the leveler's bands obey.

**Prototype is the constructor, not a keyword row.** Prototype is a static
keyword ability [CR#702.160a] whose parameters are precisely the pair, so the
wrapper spells it; `inner.text` does not repeat it. `Words.keywordFacts` has no
`Prototype` row and needs none — unlike the leveler's level up ability
[CR#711.4], which is printed as a separate ability on the base face, the
prototype ability *is* the inset frame [CR#718.1].

**Shapes** (`idris/src/Experimental/Card.idr`), following the leveler wrapper
idiom exactly:

- `record PrototypeAlt` — `MkPrototypeAlt (cost : ManaCost) (box : PrintedBox)`.
  Both required, neither `Maybe`: the inset frame prints both [CR#718.1]. The
  cost is a bare `ManaCost` rather than the face's `Maybe ManaCost` because an
  alternative set without a mana cost is meaningless [CR#718.3a], and the
  emptiness of the run is what the pin refuses.
- `prototypeFrameOk : TypeLine -> Maybe PrintedBox -> Bool` — a creature line
  with a printed power/toughness box, i.e. a first set for the inset frame to
  give a second [CR#718.1,718.2].
- `data PrototypeAltLaws : (l : TypeLine) -> PrototypeAlt -> Type` with
  `MkPrototypeAltLaws` carrying `ManaRun a.cost` (the written-cost witness the
  `Mana` cost constructor already uses) and `CardBox Front l [] (Just a.box)`
  — the alternative box read through the shared type line, the band's law with
  the band's text at `[]` because the inset frame carries no text of its own.
- The constructor, beside `Leveler`:

      Prototype : (inner : CardFace) -> (alt : PrototypeAlt) ->
                  {auto 0 nf : FaceLaws Front inner} ->
                  {auto 0 pf : So (prototypeFrameOk inner.line inner.box)} ->
                  {auto 0 al : PrototypeAltLaws inner.line alt} -> Card

  The inner face owes the ordinary front-face laws and nothing else; the whole
  of the difference between a prototype card and a plain creature card is the
  pair.

**Macros** (`idris/src/Experimental/Macros.idr`): `prototypeAlt (cost) (pow)
(tou)` builds the inset `PtBox`, and `prototype (name) (cost) (supers) (line)
(text) (stats) (alt)` forwards all three obligations as its own `{auto 0 …}`
parameters, so every bench site is brace-free.

**Witnesses** (`idris/src/Experimental/Cards/Faces.idr`, all through
`Macros.prototype`, all vintage-legal and `supported` in
`data/derived/cards.jsonl`): `arcaneProxy` ({7} 4/3, Prototype {1}{U}{U} — 2/1,
with the full enters-if-you-cast-it exile/copy/cast-without-paying line),
`blitzAutomaton` ({7} 6/4, Prototype {2}{R} — 3/2, haste), `goringWarplow`
({6} 5/4, Prototype {1}{B} — 1/1, deathtouch).

**Pins** (`idris/src/Experimental/ProofsFaces.idr`), with the positive twin
`okPrototypeAlt` above them:

- `badPrototypeWithoutAltCost` — an inset frame with a power/toughness but no
  mana cost: the second set includes a mana cost, and a prototyped spell is
  cast using only that one [CR#702.160a,718.3a]. Refused at `al`.
- `badPrototypeAltLoyaltyBox` — an inset frame printing loyalty instead of
  power and toughness [CR#702.160a,718.1]. Refused at `al`.
- `badPrototypeOffCreatureFrame` — prototype on a noncreature artifact with no
  printed box: no first set of power and toughness for the inset frame to give
  a second [CR#718.1,718.2]. Refused at `pf`.

## Landing record

- `cd idris && ./scripts/build` (clean `build/`): last line
  `44/44: Building Cards (src/Cards.idr)`, exit 0 — **44/44 modules, 0 Error,
  0 Warning**, bench-brace lint green. The round brief expected 46: the two
  facts modules it names are not in this workspace's base (they landed after
  this feature was claimed, and refreshing is not this round's to do), so the
  count here is the leveler round's 44 plus no new modules — this round adds
  no module.
- `cargo xtask cite check --list-noncompliant`: 3 non-compliant strings, **0 in
  this round's diff** — all three are pre-existing, untouched performance
  numbers in `docs/tickets/done/xtask-corpus-workers-env-cap.md`.
- `cargo xtask cite check`: `checked 18394 citations against cr.txt (eff.
  2026-08-07); 0 stale`.
- `cargo xtask cite bless`: registered 5 new rules —
  [CR#718.2,718.3,718.3a,718.3b,718.5] — and pruned none; the lock gained
  exactly 5 lines (`blessed 1593 rules`). [CR#702.160a] and [CR#718.1] were
  already registered by earlier rounds.
- `jj --no-pager diff --git > /tmp/pt.diff && cargo xtask cite audit --diff <
  /tmp/pt.diff`: `audited 27 citation site(s)`; every rule read against the
  claim citing it.
- Pin non-vacuity, probed: a scratch module built each pin's term with only the
  offending value corrected (no alternative cost -> `{1}{R}`; the loyalty box
  -> a 1/1 `PtBox`; the noncreature/no-box frame -> a 2/2 artifact creature)
  and typechecked all three, so each pin refuses exactly its named obligation
  and nothing else. The scratch module was removed; the standing evidence is
  `okPrototypeAlt`, kept beside the pins.
- Assurance counts: restored 0; re-spelled 0; ignored 0; **added 7** (3 bench
  witnesses, 1 positive twin, 3 pins); **removed 0**. No existing definition
  was deleted, weakened, or renamed — the leveler's shapes are untouched.
- Deviations and additions:
  - **The brief's colour pin is not spellable, and is replaced by the ticket's
    own second pin.** See the colour paragraph above: colour is derived from
    the alternative cost [CR#718.3b], so no term can state a disagreement, and
    the rule the brief cites for it does not exist. Not taken as a STOP: the brief
    asked the round to decide from the rule and say so if the rule does not
    make it meaningless, and the ticket's "Done when" already names the
    alternative-box pin that landed in its place.
  - **A third pin beyond the ticket's two**: `badPrototypeOffCreatureFrame`,
    the frame law's refusal. Justification: `pf` is an obligation of the
    constructor, and an unpinned obligation is unprobed; it is the
    `badLevelBandOffLevelerFrame` of this wrapper.
  - **Not pinned: an inner face with no normal mana cost.** Every printed
    prototype card has one, but a prototype card without a normal mana cost is
    unprinted rather than CR-meaningless — [CR#718.3] offers the choice of
    casting normally without requiring that both branches be payable — and
    pins refuse only the latter.
  - **Helpers beyond the ticket's named sorts**: `prototypeFrameOk` and
    `PrototypeAltLaws`. Justification: the ticket names the record and the
    constructor; these are the predicates its two settled obligations are
    stated over, and they sit beside `levelerFrameOk`/`LevelBandLaws` in the
    same file.
  - Nothing outside `idris/`, this ticket, and `cr-citations.lock` was touched.
    No `Words.Kind`/`Payload`/join-word, `Effect.Modal`, `Words.KeywordFacts`,
    `FactsGen.idr`/`KeywordShapes.idr`, or `crates/` edit.
- No STOP taken. No working-copy divergence.
