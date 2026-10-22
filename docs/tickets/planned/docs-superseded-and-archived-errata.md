---
needs: []
---
# Correct rule claims in rules-taxonomy.md and add errata to two archived ADRs

`docs/rules-taxonomy.md` is superseded in vocabulary but still read, so
correct its rows in place. Archived ADRs stay as the record of their ruling.
Give each one an erratum note at the claim, and leave the ruling text as it
is. Nothing here changes code; the Lean and Rust layers are already right in
every case.

Provenance: `docs/evidence/mtg-rules-review-2026-10-21/departures-docs.md`
entries 18–21, 25 and 26 (gitignored; read it from the default checkout). All
are confirmed at head as of 2026-10-21.

## `docs/rules-taxonomy.md`

1. **Line 116, binding table.** The row claims that "the exiled card" resolves
   to at most one object. When the linked ability exiled several cards, the
   reference refers to each of them: variables sum, and actions apply to each
   card [CR#607.3]. Move "the exiled card" to a linked-group row, or annotate
   it. "Equipped creature" correctly stays at most one.
2. **Line 338, the game-rule continuous-effects row.** "double" (432) and "enters tapped" (737) are
   filed as game-rule continuous effects [CR#613.11], but neither belongs
   there. Enters-tapped is a replacement effect [CR#614.1d]. Damage doubling
   is a replacement effect [CR#701.10g]. P/T doubling and tripling are
   continuous P/T effects [CR#701.10a,701.11a]. Life, counter and mana
   doubling are one-shot [CR#701.10d,701.10e,701.10f]. Remove both entries
   from the row. Point enters-tapped to the replacement row, and split
   "double" by form. This also makes the file consistent with its own lines
   282 and 407.
3. **Lines 483 and 514, speed.** "number 1–4" gives a range the CR does not
   set. A player has no speed until it is set [CR#702.179b]. Only the inherent
   trigger stops at 4 [CR#702.179d]. "Max speed" means a speed of 4
   [CR#702.179e], which the row already states correctly. Effects that lower
   speed carry their own floors. Reword the row: a number, none until set, no
   range in the rules. Keep the engine type unbounded, as Lean's
   `PlayerStat.speed` is.
4. **Line 598, keyword counters.** "a closed CR list worth mirroring" leaves
   out the clause "as well as any variants of those keywords" [CR#122.1b].
   The fifteen keywords are a closed base, but eligibility is open over their
   variants. Reword it: a keyword counter can be any of the fifteen keywords
   or a variant of one.

## Archived ADRs (erratum notes)

5. **`docs/archive/decisions/builtin-v2-macro-spelling-and-grammar.md`
   (lines 186–189).** The ADR claims that `equipped`, `enchanted` and
   `fortified` license a bare noun, "never *the equipped creature"
   [CR#301.5f,303.4m]. Those rules define only what the phrase refers to.
   Oracle prints the article with `enchanted` (Ironclaw Curse, "the enchanted
   creature's toughness"; Merseine, "the controller of the enchanted
   creature"). Erratum: a first mention takes the bare participle, and the
   definite form is attested in anaphoric and possessive use.
6. **`docs/archive/decisions/oracle-text-is-forward-anaphoric.md` (lines
   53–56).** "[CR#109.2] gives the three carriers". The rule also has a
   `source` carrier that works in any zone [CR#109.2c], plus a scheme clause
   that is out of scope. Erratum: name [CR#109.2a,109.2b,109.2c] alongside
   [CR#109.2]. No live doc repeats "three carriers" in this sense. (The
   matches in the `workbench-anaphora-*` done tickets use the phrase for an
   unrelated idea.)
