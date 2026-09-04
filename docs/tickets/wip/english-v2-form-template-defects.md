---
needs: []
---
# Four one-line narrowings in forms and roles

**R5 — Group R.** Four independent defects, one ticket because each is a single
line and none needs a design decision. Each is a value-keyed form or a role
optionality that froze one attested spelling as a rule.

1. **`edge_of_phrase` loses `the` on `Top`**
   (`crates/deckmaste_english_v2/src/constructions.rs:3615`):
   `form top when position is Top = lex(position) lex(relation) whole;` beside
   `form bottom otherwise = "the" lex(position) lex(relation) whole;`. So
   `from top of your library` parses, **`from the top of your library` does
   not**, and `on the bottom of your library` does. This is a transcription
   error frozen by a value-keyed form, not a grammar rule.
   *Correction, 2026-09-04 (review-fix):* the defect is real but is not a
   transcription error, and the article is not a fact about `Top`. Over the
   corpus the article tracks the governing preposition, not the edge: `on top
   of` 282 / `on the top of` 0; `from the top of` 365 / `from top of` 0; `on the
   bottom of` 594 / `on bottom of` 0; `from the bottom of` 5. So both arms are
   half-right and half-wrong — the `Top` arm loses 365 written faces and admits
   0 written ones. The value key is removed; the article becomes an optional
   declared role.
2. **`passive_movement_predicate` makes the source mandatory** (`:1785`):
   `source: FrameComplement` is required, so
   `Whenever a permanent an opponent controls is put into a graveyard, …` fails
   and only the attested `put into X from Y` shape is admitted. The role is
   `opt`.
3. **Struck 2026-09-04, review-fix: the premise does not hold.**
   *was:* "`bare_and_predicate_coordination` refuses a two-member `, and`
   (`:1662`): the positional separator table sets `pair = " and "`, so
   `…exile them in a face-down pile, and shuffle that pile.` is refused. Oracle
   spells both; the pair position admits both surfaces." The audit's sentence is
   a **three**-member coordination — Mangara's Tome and Parallel Thoughts read
   `search your library for five cards, exile them in a face-down pile, and
   shuffle that pile.` — which the `first`/`last` rows already spell, and the
   three-member path is live (`Draw a card, gain 1 life, and draw a card.`
   selects on trunk). No two-member `, and` predicate coordination is written:
   over 35,961 corpus faces, every sentence carrying exactly one comma plus
   `, and ` continues with a subject (`its` 27, `you` 24, `mana` 17, …), i.e.
   finite-clause coordination, not a bare predicate. `pair = " and "` is a
   correct grammatical statement, not a frozen spelling.
4. **Struck 2026-09-04, review-fix: routed to
   `english-v2-lexeme-owned-verb-frames` (R12) as an explained deferral.**
   *was:* "`codec EnterWithCountersVerb` confines a general frame to one verb
   (`:715`): the `V with ‹obj› ‹prep› ‹obj›` tail is *enter*-only, so
   `…exile that card with a dream counter on it…` fails for every counter kind.
   The frame belongs to any verb that declares it." The codec already serves any
   verb that declares the frame; nothing in it names *enter*. What confines it
   is the declaration model: a core verb spells `Role("FrameComplement")` in
   `core_verbs.ron`, but *exile* is a keyword-action declaration whose frames are
   `CustomTailAtom`, which has exactly five variants — `Literal`, `Lex`,
   `Amount`, `ObjectNounPhrase`, `PredicativeComplement`
   (`crates/deckmaste_construction_core/src/macro_def.rs:416`) — and no role
   atom. The attested tail is also a different frame from *enter*'s: it carries a
   direct object (`exile it with four time counters on it`), which
   `EnterWithCountersVerb` has no slot for. Both are frame-model work the ticket
   fences out.

Pinned shape. Fix each in place; each is independently landable and independently
measurable. Item 4 is the smallest instance of the pattern
`english-v2-lexeme-owned-verb-frames` generalizes — do it here only as far as
letting the existing codec serve more than one verb; do not start the frame
redesign in this ticket.

Fences. Adding a fifth form arm instead of removing a value key. A `checked by`
naming `top`, `bottom`, `enter`, or any card. Any census used as a gate.

Glossary: Form, Linearization, Complement, Verb Frame, Coordination. Record any
gap.

Baseline, measured on change `oulzkkoqmvuv` — re-measure at claim. Standard
constraints apply.

## Implementer STOP record (gpt-5.6-terra, superseded by the review-fix below)

**STOP — no production change retained.** Start: `2026-09-04T15:03:46-07:00`.
The refreshed-tree coverage probe disproved the proposed `Top` replacement:
adding `"the"` to the `Top` form removes 128 previously covered corpus
identities and adds 11. This is a ticket-versus-ruling contradiction: the Group
R ruling in `rulings/no-relayering-during-group-r.md` says that grammatical or
near-grammatical material stays, and the lost identities include the live
`on top of` surface (for example Academy Ruins, Run Aground, Repel, Brainstone,
and Plow Under). The requested one-line replacement removes that surface rather
than making the article optional. The coverage decrease is independently a STOP.

Item 4 is also not representable by the stated one-line scope. `Exile` is a
keyword-action declaration, not `CoreVerbIdentity`; its custom tail uses
`CustomTailAtom`, which has no `Role`/`FrameComplement` atom. An attempted
`With ObjectNounPhrase On Role("FrameComplement")` declaration was rejected
because `Role` is not a `CustomTailAtom` variant. Supporting the requested
counter tail requires a declaration-model/frame redesign, which the ticket
expressly fences out. That attempted row was removed immediately.

Coverage probe on the refreshed tree, report mode: 16,951 selected / 16,951
covered; 15,690 parse failures; 0 unresolved ties; 0 round-trip mismatches;
128 lock losses and 11 gains. No lock was blessed because this is a STOP. The
gains, with selected analyses, were:

- Gloom Surgeon — “If combat damage would be dealt to this creature, prevent that damage and exile that many cards from the top of your library.”
- Virtue of Courage // Embereth Blaze (Virtue of Courage) — “Whenever a source you control deals noncombat damage to an opponent, you may exile that many cards from the top of your library. You may play those cards this turn.”
- Doomskar Warrior — “Backup 1 … Whenever this creature deals combat damage to a player or battle, look at that many cards from the top of your library. …”
- Feldon, Ronom Excavator — “Haste … Whenever Feldon is dealt damage, exile that many cards from the top of your library. …”
- The Key to the Vault — “Whenever equipped creature deals combat damage to a player, look at that many cards from the top of your library. …”
- Expedited Inheritance — “Whenever a creature is dealt damage, its controller may exile that many cards from the top of their library. …”
- Shadow Urchin — “Whenever a creature you control with one or more counters on it dies, exile that many cards from the top of your library. …”
- Thor, Guardian of Midgard — “Flying … Whenever a source you control deals noncombat damage to an opponent, you may exile that many cards from the top of your library. …”
- Crumbling Sanctuary — “If damage would be dealt to a player, that player exiles that many cards from the top of their library instead.”
- Chandra, Legacy of Fire — “At the beginning of your end step, Chandra deals X damage … Exile that many cards from the top of your library. …”
- Dream Pillager — “Flying … Whenever this creature deals combat damage to a player, exile that many cards from the top of your library. …”

The coverage gate's complete loss delta was observed (128 names); it is not
reproduced here because no change can land after the STOP. No ambiguity
before/after comparison, roundtrip, or coverage bless was run after the STOP.
There was no genuine selected-reading tie.

Positive gates before the STOP: `cargo fmt --all` completed; strict
`cargo clippy -p deckmaste_english_v2 --all-targets -- -D warnings` completed
(`Finished dev profile … in 18.46s`). The workspace suite was started, but no
final result artifact was retained before stopping; it is not claimed as a
passing gate. Coverage performance advisory: workers 8, 162 s,
191,386 ns/B accepted CPU, host load 22.31 / 24.52 / 17.93; concurrent-process
count unavailable to the sandbox.

Assurance: restored 0; re-spelled 0; ignored 0; added 0; removed 0. Deviations
and additions: none retained. Glossary gap: none (all five ticket terms occur
in the Oracle English context). Decision wanted: split the `Top` surface into a
form that preserves both articles, and decide whether lexeme-owned verb frames
need a declaration-model extension before retrying item 4. End:
`2026-09-04T15:24:26-07:00`.

## Landing record

Review-fix landing, 2026-09-04 (Opus reviewer/integrator). Change id
`qtyqovrt`; every number below measured on that tree, whose coverage lock reads
`covered` **17,114**.

### Numbers before/after

| | before (`@-`, trunk) | after |
|---|---|---|
| corpus units selected / covered | 17,068 | **17,114** (+46) |
| parse failures | 15,573 | 15,527 (−46) |
| coverage lock `covered` | 17,068 | 17,114 (add-only: 46 inserted, 0 removed) |
| construction declarations | 385 | 385 |
| selection census — unique | 13,452 | 13,468 |
| selection census — specificity-resolved | 3,616 | 3,646 |
| unresolved ties | 0 | 0 |
| roundtrip clean / mismatched | — | 17,114 / 0 |
| licensed vocab/lexicon homographs | 2 | 2 |
| form-literal/vocab overlaps | 5 | 9 |

Selection neutrality, proved per unit on the final tree (`ambiguity --json`
trunk vs tree, 32,641 units on both): **46 newly selected, 0 no longer
selected, 0 units whose selected construction path changed.**

### Gate artifacts

- `cargo fmt --all --check` → exit 0.
- `cargo clippy -p deckmaste_english_v2 --all-targets -- -D warnings` →
  `Finished dev profile … in 5.91s`, no warnings.
- `cargo clippy -p xtask --all-targets -- -D warnings` →
  `Finished dev profile … in 8.07s`, no warnings.
- `cargo test -p deckmaste_english_v2 -p xtask` → every suite `ok`; the largest
  is `test result: ok. 453 passed; 0 failed; 1 ignored` (the ignored test is
  pre-existing and untouched). Gate scope is the two touched crates:
  `crates/deckmaste_construction_core/src/emit/`, `core_verbs.ron` and
  `plugins/builtin_v2/` are unchanged, and `deckmaste_english_v2` has exactly
  one reverse dependency (`xtask`).
- `DECKMASTE_COVERAGE_LOCK=report cargo xtask english_v2 coverage --check
  --workers 8` → `newly covered 46 corpus identities`, **no** `no longer
  covered` line; then `--bless` (add-only, 46 insertions / 0 deletions in
  `english-v2-coverage.lock`); then `--check` again →
  `summary {"total_units":32641,"selected_units":17114,"covered_units":17114,
  "selected_uncovered_units":0,"parse_failures":15527,"unresolved_ties":0,
  "internal_failures":0,…,"roundtrip_mismatch_units":0,
  "ownership_failure_units":0,…} lock_mode=report` with a 0-new / 0-lost lock
  delta.
- `cargo xtask english_v2 ambiguity --require-resolved --workers 8` → exit 0,
  `unresolved_ties: 0`.
- `cargo xtask english_v2 roundtrip --require-clean --workers 8` →
  `clean 17114 / mismatched 0 / not parse accepted 15527`.
- `cargo xtask cite check --list-noncompliant` → `0 non-compliant
  citation-looking string(s)`. `cargo xtask cite check` → `checked 14346
  citations against cr.txt (eff. 2026-08-07); 0 stale`. No citation was added,
  changed or blessed.

### Performance advisory

`coverage --check --workers 8`: **115 s** wall against the 16.26 s
quiet-host ceiling (`criterion=capped_workers`), **137,117 ns/B** accepted
thread CPU, host load 10.75 / 11.28 / 12.03. Contention stamp: **3 executors +
2 reviews** live on this host, four of them on `constructions.rs`; the wall
figure is not comparable to a quiet-host baseline and the per-byte figure is the
one to read.

Newly covered identities (46), each with the clause its selected analysis newly reads:

*Edge-of-phrase with the definite article — `edge_of_phrase` single arm,
article present (12):*

- Bösium Strip — "{3}, {T}: Until end of turn, you may cast instant and sorcery spells from the top of your graveyard."
- Chandra, Legacy of Fire — "Exile that many cards from the top of your library."
- Crumbling Sanctuary — "If damage would be dealt to a player, that player exiles that many cards from the top of their library instead."
- Doomskar Warrior — "Backup 1 Trample Whenever this creature deals combat damage to a player or battle, look at that many cards from the top of your library."
- Dream Pillager — "Flying Whenever this creature deals combat damage to a player, exile that many cards from the top of your library."
- Expedited Inheritance — "Whenever a creature is dealt damage, its controller may exile that many cards from the top of their library."
- Feldon, Ronom Excavator — "Whenever Feldon is dealt damage, exile that many cards from the top of your library."
- Gloom Surgeon — "If combat damage would be dealt to this creature, prevent that damage and exile that many cards from the top of your library."
- Shadow Urchin — "Whenever a creature you control with one or more counters on it dies, exile that many cards from the top of your library."
- The Key to the Vault — "Whenever equipped creature deals combat damage to a player, look at that many cards from the top of your library."
- Thor, Guardian of Midgard — "Flying Whenever a source you control deals noncombat damage to an opponent, you may exile that many cards from the top of your library."
- Virtue of Courage // Embereth Blaze (Virtue of Courage) — "Whenever a source you control deals noncombat damage to an opponent, you may exile that many cards from the top of your library."

*Passive movement with no source — `passive_movement_predicate`, `source`
absent (34):*

- Arcane Heist — "If that spell would be put into their graveyard, exile it instead."
- Bilbo, Thief in the Night — "If an instant or sorcery spell cast this way would be put into your graveyard, exile it instead."
- Chandra, Acolyte of Flame — "If that spell would be put into your graveyard, exile it instead."
- Chandra, Flame's Catalyst — "If that spell would be put into your graveyard, exile it instead."
- Coerced Confession — "You draw a card for each creature card put into their graveyard this way."
- Daring Waverider — "If that spell would be put into your graveyard, exile it instead."
- Deluxe Dragster — "If that spell would be put into a graveyard, exile it instead."
- Diluvian Primordial — "If a spell cast this way would be put into a graveyard, exile it instead."
- Diviner of Mist — "If that spell would be put into your graveyard, exile it instead."
- Dread Summons — "For each creature card put into a graveyard this way, you create a tapped 2/2 black Zombie creature token."
- Efreet Flamepainter — "If that spell would be put into your graveyard, exile it instead."
- Flotsam // Jetsam (Jetsam) — "If a spell cast this way would be put into a graveyard, exile it instead."
- Goblin Dark-Dwellers — "If that spell would be put into your graveyard, exile it instead."
- Gremlin Infestation — "When enchanted artifact is put into a graveyard, create a 2/2 red Gremlin creature token."
- Impulsivity — "If that spell would be put into a graveyard, exile it instead."
- Kaya's Ghostform — "Enchant creature or planeswalker you control When enchanted permanent dies or is put into exile, return that card to the battlefield under your control."
- Liliana's Indignation — "Target player loses 2 life for each creature card put into your graveyard this way."
- Mission Briefing — "If that spell would be put into your graveyard, exile it instead."
- Patient Rebuilding — "At the beginning of your upkeep, target opponent mills three cards, then you draw a card for each land card put into their graveyard this way."
- Prince of Thralls — "Whenever a permanent an opponent controls is put into a graveyard, put that card onto the battlefield under your control unless that opponent pays 3 life."
- Pulmonic Sliver — "All Slivers have \"If this permanent would be put into a graveyard, you may put it on top of its owner's library instead.\"
- Reyhan, Last of the Abzan — "Whenever a creature you control dies or is put into the command zone, if it had one or more +1/+1 counters on it, you may put that many +1/+1 counters on target"
- Samurai of the Pale Curtain — "Bushido 1 If a permanent would be put into a graveyard, exile it instead."
- Scholar of the Lost Trove — "If an instant or sorcery spell cast this way would be put into your graveyard, exile it instead."
- Sins of the Past — "If that spell would be put into your graveyard, exile it instead."
- Sorcerous Squall — "If that spell would be put into a graveyard, exile it instead."
- Storm of Memories — "If that spell would be put into a graveyard, exile it instead."
- Terastodon — "For each permanent put into a graveyard this way, its controller creates a 3/3 green Elephant creature token."
- The Dawning Archaic — "If that spell would be put into your graveyard, exile it instead."
- Torrential Gearhulk — "If that spell would be put into your graveyard, exile it instead."
- Toshiro Umezawa — "If that spell would be put into a graveyard, exile it instead."
- Victor Timely, Wily Tycoon — "If that spell would be put into your graveyard, exile it instead."
- Viridian Harvest — "Enchant artifact When enchanted artifact is put into a graveyard, you gain 6 life."
- Volcanic Eruption — "Volcanic Eruption deals damage to each creature and each player equal to the number of Mountains put into a graveyard this way."

Every one of the 46 falls in one of the two families the two retained fixes
open; none is a negative oracle, and no analysis outside those two families
changed (per-unit proof above).

### As landed, item by item

1. **`edge_of_phrase` — value key removed, article made a declared role.** The
   two arms `form top when position is Top` / `form bottom otherwise` collapse
   to one `form edge_of_phrase = lex(definiteness) lex(position) lex(relation)
   whole;`, with `definiteness: opt lex DefiniteMarker` and a new one-member
   `vocab DefiniteMarker { The = "the", }` (the `opt lex <one-member vocab>`
   idiom already used for `PredicateNegator` and `DistributionReplacement`).
   Diagnosed on trunk before changing anything: the audit's *defect* is real
   but its *cause* was wrong. The article does not track the edge, it tracks the
   governing preposition — `on top of` 282 / `on the top of` 0, `from the top
   of` 365 / `from top of` 0, `on the bottom of` 594 / `on bottom of` 0, `from
   the bottom of` 5 across 35,961 corpus faces. So the naive "add `the` to the
   `Top` arm" the implementer tried had to lose the 282-face `on top of`
   surface (its −128 / +11 probe), and the trunk arm had to lose the 365-face
   `from the top of` surface. Neither surface is a fact about `Top`, so the key
   goes and the article becomes optional data recorded in the AST — which is
   also what keeps rendering exact (roundtrip 0 mismatches). Probes on the final
   tree: `Put that card on top of your library.`, `Put that card on the bottom
   of your library.`, `Exile that card from the top of your library.` and
   `Exile the top card of your library.` all report `ownership covered=true`.
2. **`passive_movement_predicate` — `source` made optional.**
   `source: FrameComplement` → `source: opt FrameComplement`, and the form's
   `lex(Preposition::From) source` → `marked(Preposition::From, source)` so the
   preposition is emitted only with the role (the `marked(Preposition::Under,
   control)` idiom from `put_onto`/`return_to`). Confirmed on trunk first:
   `Whenever a creature is put into a graveyard, draw a card.` failed and
   `… from the battlefield, …` selected; 27 corpus faces write `is put into
   ‹zone›` with no source against 169 with one. Both select on the final tree.
3. **Struck — the premise does not hold.** See the struck item in the letter:
   the audit's sentence is a three-member coordination that the `first`/`last`
   rows already spell (`Draw a card, gain 1 life, and draw a card.` selects on
   trunk), Mangara's Tome and Parallel Thoughts fail for their members
   (`Exile them in a face-down pile.` and `Shuffle that pile.` each fail
   standalone on trunk), and no two-member `, and` predicate coordination is
   written anywhere in the corpus — every one-comma `, and ` sentence continues
   with a subject, i.e. finite-clause coordination. `pair = " and "` is correct.
4. **Struck and routed — explained deferral.** Confirmed the implementer's
   finding and widened it: a keyword-action declaration's frames are
   `CustomTailAtom`, whose five variants (`Literal`, `Lex`, `Amount`,
   `ObjectNounPhrase`, `PredicativeComplement`,
   `crates/deckmaste_construction_core/src/macro_def.rs:416`) contain no role
   atom, so *exile* cannot name `FrameComplement` the way `core_verbs.ron` does
   for `Enter`; and the attested tail (`exile it with four time counters on
   it`) carries a direct object that `EnterWithCountersVerb` has no slot for, so
   it is a different frame besides. No data-only fix exists inside `Lex` /
   `OptionalLex` / `Marked`. Routed as a dated paragraph appended to
   `docs/tickets/planned/english-v2-lexeme-owned-verb-frames.md` (R12) naming
   the verbs, the sentences and both mechanism gaps.

### Review corrections

The implementer committed a ticket record and no production change. Findings
against that STOP, and what this landing did with each:

- **HIGH — the item 1 STOP was raised against a fix the ticket did not ask
  for.** The record reports −128 / +11 for "adding `the` to the `Top` form" and
  states it "did not diagnose why". Diagnosing it first (corpus surface counts
  plus trunk probes of both sentences) showed the general fix — remove the value
  key, make the article optional — loses nothing and gains 46. Fixed; the STOP
  is withdrawn.
- **MEDIUM — items 2 and 3 were never attempted.** Item 2 is fixed and
  measured. Item 3 is struck on evidence.
- **MEDIUM — item 4 was left as an undirected STOP.** Confirmed, widened and
  routed to R12, and struck in the letter with a dated `was:` line.
- **LOW — the record's per-byte perf figure was written as a decimal the cite
  checker reads as a rule number.** Rewritten as `191,386 ns/B`. The same run's
  raw wall-seconds figure, printed to nanosecond precision, tripped
  `cite check --list-noncompliant` the same way and is rewritten as `162 s`;
  both cite gates are now clean.

### Deviations and additions

- **Added** `vocab DefiniteMarker { The = "the", }` (1 line). Not in the
  ticket's letter, but item 1's "remove the value key" has no other mechanism:
  the form language has no optional-literal atom, and `opt lex <vocab>` is the
  declared way to make a closed-class word optional.
- **Attempted and reverted:** converting the four surviving `"the"` form
  literals (`definite_next_mass_quantity_reference`, `number_of_scalar_value`,
  `greatest_scalar_value`, `positional_partitive`) to
  `lex(DefiniteMarker::The)`, so that "the" would keep a single owner and the
  form-literal/vocab overlap census would stay at 5. It was reverted because the
  per-unit selection proof caught a regression: Kitsune Palliator
  (`Prevent the next 1 damage that would be dealt to each creature and each
  player this turn.`) has two candidate readings on both trees, and the
  conversion flipped its specificity tie from the correct reading (relative
  clause on "the next 1 damage", coordination inside the relative's `to`
  complement) to a reading whose object is a top-level coordination. Specificity
  scores a `lex` atom differently from a literal, so the conversion silently
  re-ranked an unrelated card. With the four reverted, the per-unit diff is
  0 flipped. The consequence is disclosed below.
- **Consequence disclosed:** `form_literal_vocab_overlaps` rises 5 → 9 — the
  four `"the"` literals now coexist with a vocabulary member that owns the same
  surface, exactly as `additional`, `to`, `next` and `other` already do. The
  xtask census assertion at
  `crates/xtask/src/english_v2/coverage.rs:2964` is updated 5 → 9 to match. This
  is a counted census moving, not a gate weakening: the assertion still pins an
  exact number, and nothing was deleted or loosened. Giving `the` a single owner
  wants the specificity model to treat `lex(V::M)` and the literal it spells
  alike; that is R12/closed-class work, not this ticket's.
- **Not done:** nothing else. No construction was added or removed (385 → 385),
  no `checked by`, `require` or comment naming a word, lexeme, construction,
  verb, noun, preposition or card was added, and no census is used as a gate.

### Assurance counts

Restored 0 · re-spelled 0 · ignored with blockers 0 · added 0 · **removed 0**.
One existing test's asserted census constant was updated in place
(`form_literal_vocab_overlaps` 5 → 9, `coverage.rs:2964`); it is neither a
removal nor a weakening — the assertion is still an exact equality on a number
this landing legitimately moved.

### STOPs

One inherited STOP (the implementer's, above), resolved by diagnosis rather than
escalation. One STOP taken and resolved by routing: item 4's frame-model gap, an
explained deferral to R12 with the letter struck and dated. No ticket-vs-ruling
contradiction was resolved silently; the Group R "no relayering" ruling the
implementer invoked is satisfied, because the landing removes no surface — the
per-unit proof shows 0 units losing their selection.

### Glossary gaps

- `glossary gap:` **definite marker**. `docs/contexts/oracle-english/CONTEXT.md`
  defines *Determiner* as the function that marks a **Noun Phrase** as definite;
  it has no term for the closed-class definite article realized inside a
  non-nominal phrase (here, an Edge Of Phrase that is a Prepositional Complement
  / Frame Complement, not a Noun Phrase). `DefiniteMarker` is named for that
  role. Worth a CONTEXT entry when the closed-class single-owner work lands.
- The ticket's five named terms — Form, Linearization, Complement, Verb Frame,
  Coordination — are all present and are used here as defined.
