---
needs: []
---
**Cards write the dialect they may write.** `plugins-v2-dialect` landed the
reader side (embeds bare, positional application, numeral leaves, case as
the macro/constructor mark) and made every card macro-only, but the cards
and family bodies were converted by identifier rewrite, so they still spell
`Simple(symbol: Specific(color: Of(color: White)))` where the dialect reads
`White`, `colorIs(color: Red)` where it reads `colorIs(Red)`, and
`Lit(value: 3)` where it reads `3`. This ticket is the load-and-reserialise
pass the second dialect rulings describe: a one-off converter keeps each
file's leading comment block verbatim and reserialises only the value
through the dialect's writer (embeds bare, positional where unambiguous,
numerals bare), over `plugins_v2/canon`, `plugins_v2/testing` and the
family bodies under `plugins_v2/builtin/macros/`; interior comments are
counted, listed, and re-placed by hand; the converter is deleted. Oracles:
`lean/Generated` byte-identical before and after, `lean-check` 118/118 and
2/2, `cargo xtask facts check` byte-identical, the corpus test green. The
writer's positional/embed elision is what `semantics-v2-embed-candidates`
reads its evidence from. Standard constraints apply.

## Landing record (2026-09-07)

**The ticket does not land. Its central instruction — load each card file and
reserialise its value through the dialect's writer — is unimplementable
against the macro-only rule the same ticket family landed four days' work
ago, and the contradiction is between two RECORDED RULINGS, so it is a STOP
rather than a choice.** What lands is the writer's proof (step 2) and the
evidence step 5 asks for; steps 3 and 4 do not run. See **STOP 1**.

Measured on `umstxnts` (`docs: what the dialect writer shows…`), `nproc` 24,
load average 1.46. No coverage lock is in play for this lane.

### PROVE

**No silent loss.** Nothing stopped being covered, and nothing in
`plugins_v2/` was edited. `cargo xtask lean-check` proves `plugins_v2/canon`
118/118 and `plugins_v2/testing` 2/2 before and after, and
`lean/Generated/{Canon,Testing}.lean` is byte-identical to the baseline taken
at step 1 — 267,615 B, `Canon.lean` `b06c72d6…`, `Testing.lean` `ac54b387…`.
`cargo xtask facts check` reports both generated files up to date; neither was
regenerated. The landing adds one test and one documentation section and
touches nothing else.

**Structural laws.** The Lean build ends in success with no warnings (80
jobs). `cargo test -p deckmaste_semantics_v2 --test lean_drift` green: no
mirror type, variant, field or binder changed. The emitter stays total and
every emitted card still proves. The new pin adds a law the corpus did not
have — the writer is LOSSLESS over all 120 cards, and it elides every
injection and both numeral leaves.

**No word-naming.** The one guard added reads declared markers, not words: the
six strings it forbids in the writer's output are the constructor-plus-binder
spellings of the four `#[macro_ron(embed)]` sites and the two
`#[macro_ron(literal)]` leaves the mirror declares, and each names a TYPE's
variant rather than a lexeme, construction, verb, noun, preposition or card
identity. No forbidden licensing checker is involved; the lane has none.

### DISCLOSE

**Step 1, the baseline.** `cargo xtask lean-check` 118/118 and 2/2 cold in
1 m 46 s; `lean/Generated` copied to the session scratchpad as the byte-identity
oracle, 267,615 B across two files.

**Step 2, the writer — it already existed, and now it is proven.** No code was
needed. `crate::ron::raw_options`' `UNWRAP_NEWTYPES` extension IS the writer
`semantics-v2.md` §11.1 describes: an `#[macro_ron(embed)]` struct variant
serializes its payload through a newtype struct named `Type.Variant`, which
the extension drops. Written today, with no change from this landing,
`Abbey Gargoyles`' cost is `cost:[2,White,White,White]` and its stats are
`power:3,toughness:4`. What was missing is the proof, which is the new pin
`every_card_writes_and_reads_back_to_the_same_value`
(`crates/deckmaste_semantics_v2/tests/corpus.rs`): all 120 cards of
`plugins_v2/canon` and `plugins_v2/testing` write, contain none of the six
elided spellings (`Simple(symbol:`, `Specific(color:`, `Of(color:`,
`Lit(color:`, `Lit(value:`, `Generic(amount:`), and read back to a value equal
to the one they were written from.

The pin's way back is the UNRESTRICTED read, deliberately and with the reason
in its doc comment: a written value is the constructor basis, not author
vocabulary. That single line is the whole of STOP 1.

**Not delivered from step 2: positional WRITING.** The brief asks for
"one-field constructors and macro invocations written positionally". Neither
is available and neither is wanted here:

- A macro invocation cannot be written at all. Expansion is name-erasing in
  v2 by ruling (`ron::tests::no_kind_remembers_its_invocation`, and no v2 type
  carries an `Expanded` provenance variant), so by the time a card is a value
  its macro invocations do not exist. There is nothing for a value-writer to
  write positionally.
- A CONSTRUCTOR could be written positionally with a custom serializer, but it
  has no consumer without the converter, and it would cost the binder names —
  `Of(Creature, "Gargoyle")` for `Of(host: Creature, label: "Gargoyle")` —
  which §11.1 calls "the contract with the Lean constructor" and which make a
  card file readable. Speculative work behind a STOP, so it was not built.

**Step 5, the embed candidates.** Appended to
`docs/tickets/planned/semantics-v2-embed-candidates.md`, as two censuses from
the writer's own output, because the two answer different questions and the
existing table conflated them. In card SOURCE there are 1,200 one-field
constructor applications across 20 constructors, and **1,003 of them (84%) are
the five positions already marked** — the mana chain and the two numeral
leaves — so the cosmetic pass alone removes five sixths of the corpus's
written-out one-field constructors with no new ruling at all. Of the remaining
197, `SingleFaced(face:)` is 119 and cannot be an injection as the mechanism
stands, leaving 78 writes across 14 constructors as the entire realistic gain
from every further marking combined. The earlier table's rank order
(`HasType` 26, `Mana` 34, `And` 23, `Static` 14, `InZone` 13, `Sequentially`
11) does not survive into the source census: the fourth landing turned each of
those into a named one-field MACRO invocation, 176 of them across 24 macros,
and positional application of the macro — `hasType(Creature)` — is a cheaper
win there than an embed and needs no marking. The writer's-output census (1,336
applications across 89 constructors) is recorded separately and labelled as
evidence about the constructor basis, not about card files.

**Deviations and additions.** One test added, one documentation section
appended, nothing else. `cargo xtask cite bless` was NOT run: the audit selects
0 citation sites, because nothing this landing wrote cites a rule.

**STOPs.**

1. **A card cannot be reserialised through the writer, because the writer's
   output is not author vocabulary — and the two rulings that meet here
   contradict each other.** Verified, not reasoned: `Abbey Gargoyles` read,
   written through `raw_options()`, and read back at a card entry gives

   ```
   `Keyword` is not author vocabulary at `Ability`; it names a `Ability`
   variant with no macro of that name
   ```

   The mechanism is exactly as designed. `plugins-v2-dialect`'s fourth landing
   made every card macro-only at the seventeen `EXPRESSION_KINDS` (§11.1,
   Lean's `Authoring.Form.onlyMacros`), and `reader::read_values_restricted`
   is what a card entry uses. Reading a card EXPANDS its macros — v2 keeps no
   invocation — so the value holds raw constructors throughout, and writing it
   back produces a file that the card reader refuses at its first ability.
   The written `Abbey Gargoyles` is also ~15× the source, because `flying`'s
   whole `DeonticRule` body is inlined where the card wrote one word.

   This is the brief's own listed STOP — "a reader change needed to read the
   writer's output" — and turning the macro-only rule off to accept the output
   would undo the fourth landing.

   **The family bodies are blocked twice over, and the second reason is
   simpler.** `macro_ron::MacroDef` is `Deserialize`-only; it has no
   `Serialize` at all, and its `body` field is `Box<str>` holding raw RON
   source. There is no value for a writer to reserialise, and if one were
   derived it would write the body as a quoted string. So step 3 has no
   implementable target in `plugins_v2/builtin/macros/<family>/` either.

   The two recorded rulings in contradiction, both from 2026-09-07, both in
   `docs/tickets/done/plugins-v2-dialect.md`: "**Comments are preserved by
   splice.** The one-off card converter keeps each file's leading comment block
   verbatim and reserialises only the value" (rulings after the second
   landing) against "**A card may write only macros**" (§11.1, landed fourth).
   Per CLAUDE.md a ticket-vs-ruling contradiction is a STOP even when the
   implementer can resolve it, so no converter was written.

   **What resolves it, for the ruling.** The elision the ticket is after —
   1,003 of 1,200 sites, the mana chain and the numeral leaves — is entirely
   at NON-expression positions and inside macro-free sub-terms, which is not a
   coincidence: those are precisely the positions the macro-only rule carves
   out. So either
   (a) **a source-to-source converter**, the shape the third and fourth
   landings both used and the fourth explicitly preferred: walk the card TEXT
   type-directedly and reserialise only the macro-free sub-values through the
   writer, leaving macro invocations and every comment verbatim. No reader
   change, no interior comments to re-place, and the same oracles
   (`lean/Generated` byte-identity, `lean-check` 118/118, `facts check`) still
   prove it exactly; or
   (b) **retire the load-and-reserialise ruling** and say the cosmetic pass is
   an identifier rewrite, which is what (a) is, and what the fourth landing
   already did for the substitution half.
   They are the same code; (b) is (a) with the ruling amended to match. The
   converter is NOT small — the type-directed schema walk is what the previous
   two landings built and deleted twice — and rebuilding it a third time
   before deciding whether it should be kept is worth a word in the ruling
   too.

2. **Steps 3 and 4 did not run, so their oracles are vacuous rather than
   green.** `lean/Generated` byte-identity, `facts check` byte-identity and
   `lean-check` 118/118 are all reported below and all hold, but they hold
   over an unconverted corpus; they are the baseline, not a conversion's
   proof. The three converted-card before/after examples the brief asks for
   cannot be shown. Recorded so no reader mistakes the green for evidence the
   conversion is safe.

**Glossary.** No term this landing needed is missing from
`docs/contexts/game-model/CONTEXT.md`. "Injection", "numeral leaf" and
"author vocabulary" are reader vocabulary, defined in `semantics-v2.md` §11.1
and beside the markers on the mirror.

**Assurance counts.** Restored 0, re-spelled 0, retired 0,
ignored-with-blocker 0, added 1
(`every_card_writes_and_reads_back_to_the_same_value`), removed 0.

### REPORT

- `cargo xtask lean-check`: `plugins_v2/canon` 118/118, `plugins_v2/testing`
  2/2. 1 m 46 s cold (step 1's baseline), 0.8 s warm. `lean/Generated`
  byte-identical to that baseline, 267,615 B.
- `lean/scripts/build`: success, 80 jobs, no warnings; 62.6 s cold.
- `cargo xtask facts check`: both generated files up to date; neither
  regenerated.
- `cargo test -p deckmaste_semantics_v2 --test corpus`: `plugins_v2/builtin`
  2,414 declarations across 31 kinds; 124 nullary declarations expand;
  `plugins_v2/canon` 118 cards; `plugins_v2/testing` 2 cards, 1 token,
  1 sba / 1 conferral / 1 damage row; **120 cards read, wrote, and read back
  to the same value**.
- `cargo test -p deckmaste_semantics_v2 --test lean_drift`: 4 passed.
- `cargo xtask cite check --list-noncompliant`: 0. `cargo xtask cite check`:
  15,738 citations, 0 stale. `jj diff --git --from <claim> --to @ | cargo
  xtask cite audit --diff`: 0 sites — no citation changed, so `cite bless` was
  not run.
- `cargo xtask gate --changed` closure, run green in 81 s (9
  `test result: ok` lines, 0 failures): `cargo test -p deckmaste_semantics_v2
  -p xtask`. The closure is small because the landing changes one test file
  and one planned ticket.
- `cargo fmt --all` clean; clippy clean on `deckmaste_semantics_v2`
  (`--all-targets`).
- Census: 1,200 one-field constructor applications in card source across 20
  constructors, 1,003 of them at the five marked positions; 176 named
  one-field macro invocations across 24 macros; 1,336 one-field applications
  across 89 constructors in the writer's output.

## Handoff

The workspace is parked with `@` empty and NOT integrated. Steps 1, 2 and 5
are landed and green; steps 3 and 4 are blocked on STOP 1's ruling.

When the ruling arrives the shape is (a) above, and two things are already
done for it: the writer is proven lossless over the whole corpus, so a
converter that reserialises a sub-value can trust it, and the census names
every position worth converting. The cheap oracle the fourth landing leaned on
is confirmed still cheap — `lean-check` warm is 0.8 s and `lean/Generated`
byte-identity is a `diff -r`.
