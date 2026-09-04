---
needs: []
---
**Drop the copy exclusion from `wordReaches SpellW`: a copy of a spell is
itself a spell [CR#707.10,112.1a].** Fresh workbench review 2026-09-03, F4.

`Words.wordReaches SpellW` carries a `not (isCopyOrigin og)` conjunct, so
`That SpellW` skips copies. Probe P5: after `Copy FromStack …`, `That SpellW
OneOf` and `That CopyW OneOf` are both admitted, but `That SpellW ManyOf` is
**refused** — no plural spell read can ever include a copy, which contradicts
[CR#707.10] ("A copy of a spell is itself a spell") and [CR#112.1a].

The exclusion exists for resolution, not for rules content: it makes `That
SpellW OneOf` after a `Copy` resolve to the original. Three bench sites depend
on it (`Cards/Copy.idr:46,226,256`), plus whichever pins twin them.

Fix: delete the conjunct. Where a card reads the *original* after a copy,
spell it with a stamped read the grammar already has — `ItVerbed "Copy"` for
the copy, an `Other`-style exclusion or `TheVerbed` for the original. This is
the review's own advice: stop growing `Reach`, and let provenance stamps carry
disambiguation while the kind word carries rules content.

Size: S–M.

Done when: `That SpellW ManyOf` after a `Copy` reads both the original and the
copy as a typechecking bench witness; the three `Cards/Copy.idr` sites read
their original through a stamp and still typecheck; `isCopyOrigin` no longer
appears in `wordReaches`; a pin records that the copy is reachable as a spell,
probed non-vacuous; the build is 44/44 with 0 errors and 0 warnings.

The landing record names this as a **deliberate CR alignment**: the grammar
previously diverged from [CR#707.10] and no longer does, so any future
reintroduction of a copy exclusion in a kind word is a regression, not a
convenience. Standard constraints apply, plus the RON-shaped constraint: a
core constructor is admissible only if the RON re-emitter can produce it from
a RON node, and a macro only if it names a RON macro
(`docs/decisions/workbench-ron-shaped-and-label-rulings.md`).

## As landed

- `wordReaches SpellW` is now `onStackZone zn` alone; the `not (isCopyOrigin og)` conjunct is gone. Deliberate CR alignment: a copy of a spell is itself a spell [CR#707.10,112.1a], so a copy on the stack is reachable by the kind word "spell". Reintroducing a copy exclusion in `SpellW` is a regression, not a convenience.
- `Reach` did not grow, no stamp was added, and no other `wordReaches` row changed.
- Three witnesses added in `ProofsAnaphora`, beside `copyParticipleUnwritten`:
  - `okPluralSpellReadAfterCopy` — "Copy target instant or sorcery spell twice. You may choose new targets for those spells." `That SpellW ManyOf` now reads the copies (the plural binding `Copy … (Lit 2)` introduces); it was refused before, count 0. Synthetic: printed cards of this shape say "the copies" (Echo Mage's fourth level, already on the bench as `That CopyW ManyOf`).
  - `okCopyReadAfterCopy` — "Copy target instant or sorcery spell. You may choose new targets for the copy."; the positive twin, the copy read by its own kind word.
  - `badSingularSpellReadAfterCopy` — the pin. "… You may choose new targets for that spell." is now refused, because after a one-shot `Copy` the singular spell read reaches the original *and* the copy (count 2). That refusal IS the record that the copy is reachable as a spell; under the old rule the pin refused nothing.
- The three `Cards/Copy.idr` sites (46, 226, 256) were **not** re-spelled and did not need to be — see Deviations.
- `Cards/Copy.idr`, every other bench module and every pin module typecheck unchanged.

## Landing record

- Construction count unchanged (no constructor added or removed); `Reach` unchanged; `cr-citations.lock` unchanged ([CR#707.10] and [CR#112.1a] were already registered).
- Gates (foreground):
  - `cd idris && rm -rf build && ./scripts/build` → exit 0, `44/44: Building Cards (src/Cards.idr)`, 0 `Error` lines and 0 `Warning` lines.
  - `cargo xtask cite check --list-noncompliant` → `0 non-compliant citation-looking string(s)`.
  - `cargo xtask cite check` → `checked 18184 citations against cr.txt (eff. 2026-08-07); 0 stale`.
  - `jj --no-pager diff --git > /tmp/cs.diff && cargo xtask cite audit --diff < /tmp/cs.diff` → `audited 2 citation site(s)`; both rule texts read against their claims — [CR#707.10] "A copy of a spell is itself a spell, even though it has no spell card associated with it."; [CR#112.1a] "A copy of a spell is also a spell, even if it has no card associated with it."
  - `grep -n "isCopyOrigin" idris/src/Experimental/Words.idr` → no hit in the `SpellW` row. The remaining hits are the copy *kind words* themselves (`CopyW`, `AbilityCopyW`, `CopyJoinW`) and the ability-side exclusions (`AbilityW`, `AbilityJoinW`), which this ticket does not touch — see Deviations.
- Assurance: restored 0; re-spelled 0; ignored 0; added 3; removed 0.
- Pin probes: `okPluralSpellReadAfterCopy` and `badSingularSpellReadAfterCopy` were probed by restoring the conjunct — the witness then fails with `Can't find an implementation for countReach (Word SpellW) ManyOf … = 1` and the pin with `badSingularSpellReadAfterCopy Refl is not a valid impossible case`; both are non-vacuous. The four existing copy pins were re-probed under the new rule and all four still change message when mis-stated: `badCopyPermanent`, `badStackCopyAsToken`, `badTokenCopyAsCopyMention`, `badEmptyCopyTypeException`.
- Deviations and additions:
  - **The ticket's premise about the three `Cards/Copy.idr` sites is false, and the compiler says so.** In `Copy src agent what times exc`, the subject is `what : Noun (nomIntro agent) k` — elaborated strictly before `instrIntro (Copy …)` prepends the copy binding — so a copy can never be in scope for the copy's *own* subject read. All three sites (Frontline Heroism, Bonus Round, Pyromancer's Goggles) read `That SpellW OneOf` in exactly that slot, so their count stays 1 and they typecheck unchanged. Re-spelling them through a stamp would have replaced their printed wording ("copy that spell") for no semantic reason, so they were left alone.
  - **No stamped read for the original was added, because none exists and none was needed.** The ticket suggested `ItVerbed "Copy"`; `copyPayloadIn` gives the copy binding `prov = Nothing`, so `Stamped "Copy"` reaches nothing, and `copyParticipleUnwritten : actNamesParticiple "Copy" = False` is a standing pin that keeps `TheVerbed "Copy" …` unavailable. Adding a `Copy` stamp would be grammar growth with no consumer.
  - **Ledger — expressive gap, not built:** with the exclusion gone there is no spelling that reads the *original* spell after a one-shot copy (both are `SpellW`, the copy carries no provenance stamp, and counted uniqueness therefore refuses the read). No bench card needs one. A card that does will need a new spelling — a `Copy` stamp on the copy binding, or an `Other`-style exclusion. `badSingularSpellReadAfterCopy` pins the current state.
  - **Ledger — parallel divergence left in place:** `wordReaches AbilityW … = not (isCopyOrigin og)` carries the identical exclusion for abilities, against [CR#707.10] "A copy of an ability is itself an ability." Ability sorts are the `ability-kind-taxonomy` round's region and outside this ticket's letter, so it was not touched.
- STOPs: none. The evidence was decisive at each point.
