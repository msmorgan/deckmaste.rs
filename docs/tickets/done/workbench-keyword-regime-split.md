---
needs: []
---
**Split the keyword `regime` column into the two facts it carries.** Residue
of `workbench-facts-residues` (2026-09-04, STOP 5). `keywordFacts.regime`
is read two ways: `Effect.keywordBodyFits` takes `AtCasting` as "the body
keys on a spell cast", `grantSubjectFits` takes it as "functions only while
on the stack". Prowess, extort and increment are triggered abilities of
permanents [CR#702.108a,702.101a,702.191a] that function on the battlefield
[CR#113.6], so a grant of prowess to a permanent is refused today: Bria,
Riptide Rogue ("Other creatures you control have prowess", vintage-legal,
supported) is unspellable, and it is one of two such grants in the corpus.

Fix: keep `regime` for the body's cast-keyed reading and add a
`functionsOnStack` column (hand-kept in the xtask overlay like the other
gate columns) that `grantSubjectFits` reads instead; rewrite
`Effect.abRegime`/`grantSubjectFits`; bench Bria, Riptide Rogue and the
other grant; keep the pin that refuses granting a stack-only keyword to a
permanent, probed non-vacuous.

Size: S–M. Done when: both grants are benched; the pin still refutes;
`cargo xtask facts labels` exits 0; build at its module count. Standard
constraints apply, including the RON-shaped constraint.

## As landed

- **`regime` kept as the body axis.** `Effect.keywordBodyFits` is unchanged and
  still reads `keywordStackRegime`; prowess, extort and increment keep
  `regime := Just AtCasting` so their "whenever you cast" bodies still pair.
- **`functionsOnStack` added as the grant axis.** New `KeywordFacts` column,
  hand-kept in xtask's overlay (`Row.functions_on_stack`, emitted by `render`)
  and read through the new `Words.keywordFunctionsOnStack`. 46 of the 49
  `Just AtCasting` rows carry it — the cost, alternative-cast and cast-trigger
  keywords whose ability is the spell's own [CR#113.6,113.6d,113.6e]; prowess,
  extort and increment do not, because each is a triggered ability of a
  permanent [CR#702.108a,702.101a,702.191a] functioning on the battlefield
  [CR#113.6].
- **`Effect.grantSubjectFits` reads it.** Its off-stack branch is now
  `not (abFunctionsOnStack ab)` where it was `not (castingOnly (abRegime ab))`.
  New `Effect.abFunctionsOnStack`; `Effect.castingOnly` deleted (its only
  caller was that branch). `abRegime` and `regimeMatches` are kept — see
  Deviations.
- **Both prowess grants benched, plus the extort one.**
  `Cards.Keyword.briaRiptideRogue` ("Other creatures you control have
  prowess"), `Cards.Keyword.narsetEnlightenedExile` ("Creatures you control
  have prowess"), `Cards.Keyword.pontiffOfBlight` ("Other creatures you
  control have extort") — three vintage-legal, `supported` printed cards.
- **The pin still refutes.** `ProofsKeyword.badBattlefieldConvoke` ("Creatures
  you control have convoke") is unchanged and re-elaborated in the 46/46
  build; probed non-vacuous below.
- **The printed derivation rule was rewritten, not retired.** `facts labels`'s
  `REGIME_AXIS` now states the split it used to record as a STOP, and names
  the flashback carve-out; `GATE_COLUMNS` lists eight columns.

## Landing record

Numbers before → after: `KeywordFacts` columns 9 → 10; `keywordFacts` rows 197
(unchanged); rows carrying `functionsOnStack := True` 0 → 46; rows carrying
`regime := Just AtCasting` 49 (unchanged); Idris modules 46 → 46; `xtask` unit
tests 445 → 446; `Cards.Keyword` witnesses +3. `facts labels` keyword
abilities unchanged at 195 stubs / 197 rows / 0 gaps with one stub-side and
three row-side recorded reasons; designations unchanged at 22 / 22 / 0.
`cr-citations.lock` lost 7 entries to `cite bless`'s prune — seven keyword-action
subrules under [CR#701] whose only citation sites are in `docs/tickets/done/`,
a path `cite-config.json` excludes; no rule was added.

Gates (all foreground):

- `cd idris && ./scripts/build` (clean `build/`) — `46/46: Building Cards
  (src/Cards.idr)`, exit 0, 0 `Error`/`Warning` lines, `real 1m19.996s`
- `cargo xtask facts check` — `…/idris/src/Experimental/FactsGen.idr is up to
  date`
- `cargo xtask facts labels` — exit 0; four tables, three printed derivation
  rules
- `cargo test -p xtask` — `test result: ok. 446 passed; 0 failed; 1 ignored`
- `cargo clippy -p xtask --all-targets` — 0 warnings; `cargo fmt --check` —
  exit 0
- `cargo xtask cite check --list-noncompliant` — `0 non-compliant
  citation-looking string(s)`
- `cargo xtask cite check` — `checked 14207 citations against cr.txt
  (eff. 2026-08-07); 0 stale`
- `cargo xtask cite bless` — `blessed 1422 rules`
- `jj --no-pager diff --git > /tmp/round.diff && cargo xtask cite audit --diff
  < /tmp/round.diff` — `audited 6 citation site(s)`; each rule's text read
  against the line citing it

Probes (foreground, `Experimental.ProbeTmp` typechecked and removed):

- Positive: `keywordFunctionsOnStack "Convoke" = True`,
  `keywordFunctionsOnStack "Prowess" = False`,
  `keywordFunctionsOnStack "Extort" = False`,
  `keywordFunctionsOnStack "Increment" = False` and
  `keywordStackRegime "Prowess" = Just AtCasting` all elaborate by `Refl`.
- Mis-stated: `keywordFunctionsOnStack "Prowess" = True` refuses with
  `Mismatch between: True and False.`
- `badBattlefieldConvoke` non-vacuity: the same pin with `"Prowess"` in
  Convoke's slot refuses to be a pin — `probePinMisstated Oh is not a valid
  impossible case.` — so the pin's refusal is carried by the new column, and
  the pin as written still refutes in the 46/46 build.

Assurance counts: restored 0; re-spelled 0; ignored with blockers 0; added 4
(`facts::tests::the_stack_column_is_the_grant_gate_and_the_regime_column_is_the_body_axis`
and the three bench witnesses); removed 0.

Deviations and additions:

- **`abRegime` and `regimeMatches` kept; only the off-stack branch moved.**
  The ticket says "rewrite `Effect.abRegime`". The on-stack branch of
  `grantSubjectFits` matches the granted keyword's `regime` against the
  subject noun's, and `Cards.Keyword.firesongAndSunspeaker` ("Red instant and
  sorcery spells you control have lifelink") passes only through that match:
  lifelink's regime is `Just AtResolution` and the noun's is too. Replacing
  that branch with `abFunctionsOnStack` would make the card unspellable, so
  `abRegime` stays as the regime reader for the on-stack branch and
  `abFunctionsOnStack` is a sibling rather than a rewrite of it. `castingOnly`
  had no other caller and was deleted.
- **A third witness.** The ticket names Bria plus "the other grant"; the
  corpus search (`jq 'select(.supported)'` over `data/derived/cards.jsonl` for
  "have/gain prowess|extort|increment") returns five, of which three are the
  bare "[other] creatures you control have [keyword]" shape. Narset is the
  second prowess grant the evidence names; Pontiff of Blight was added so the
  `Extort` row's flip is also carried by a printed card. The remaining two
  (Triton Wavebreaker, Wizard's Staff) grant prowess through an attachment
  host and were left out — a different shape, not this ticket's.
- **The column is authored conservatively for graveyard-cast keywords.**
  Flashback is two static abilities, "one that functions while the card is in
  a player's graveyard and another that functions while the card is on the
  stack" [CR#702.34a,113.6e], so it carries `functionsOnStack := False` and
  "Target instant or sorcery card in your graveyard gains flashback" stays
  spellable, as does `ProofsMana.okUnearthGrantInGraveyard`. The `facts
  labels` derivation rule prints this carve-out. A zone-indexed gate — asking
  whether the ability functions in the subject's own zone rather than a
  keyword-wide boolean — would subsume it and is not this ticket's.

STOP: none taken.
