---
needs: []
---
# Mode bullets: no unlabelled costed bullet, and no noun-compound Reading of an imperative bullet

Two misanalyses remain in mode bullets (measured on `loxuknun`).

1. **`CostedModeBullet` (`• Cost — ModeEffect`) has no Oracle form.** It is
   used only by these three units, and each of them has a flavor-word label:
   Battle Menu, You're Ambushed on the Road, and School Daze. The bullet text
   before the dash (*Attack*, *Stand and Fight*, *Fight Crime*) is read as a
   verbal cost as well as a Flavor Word label.

   In Oracle, a bullet label before a spaced em dash is a flavor word: "Use
   labeled modes when the card calls for names. Each label is a flavor word"
   (style guide §10 "Modal choices"; [CR#207.2d]). A mode's cost appears only
   in two forms:
   - Spree's `+ [cost] — ` (`PlusCostedMode`) [CR#702.172a];
   - Tiered's `• [flavor word] — [cost] — ` (`LabelledCostedMode`)
     [CR#702.183a].

   None of the 235 `• X — ` lines in the Oracle snapshot puts a cost directly
   after the bullet. Each of the three units has 2 Readings, one per label
   analysis, and the outer structure differs between the two Readings.
   `english-v4-per-sentence-reading-counts` needs every unit's count to be a
   product of its segments' counts.
2. **An imperative bullet still has a noun-compound Reading.** *• Counter
   target activated or triggered ability.* (Sublime Epiphany, Defabricate,
   Brokers Confluence) reads both as an imperative `ModeBullet` and as a
   `NominalMode`: `[Counter target activated] or [triggered ability]`, with
   *target* a noun under an attributive *Counter*.

   `english-v4-stratum-1-cleanup` item 6 already ruled this pattern wrong
   ("imperative mode bullets do not parse as bare noun-compound fragments"),
   and the cleanup left this residue. Style guide §10 "Modal choices": a mode
   is an instruction ending in a period. `NominalMode` exists for the
   characteristics-only bullets of Wild Shape, Outlaws' Merriment, and Genku,
   Future Shaper (*• 1/3 Turtle with hexproof.*).

## Not defects: keep both Readings

These four units are listed by the same segment probe but are genuine
Ambiguity. Their segment boundaries are already stable. The two analyses
differ only in category over the same extent.

- *• Double the number of each kind of counter on target permanent.* and *•
  Double the number of each kind of counter you have.* (Aetheric Amplifier),
  and *• Double the number of +1/+1 counters on this creature.* (Voracious
  Hydra).
  - The `NounPhraseMode` Reading is a Noun Phrase with the multiplier
    *double* as Predeterminer: "We've had [double the number of applications
    I had expected]" (CGEL, Ch. 5 §12(c), p. 434).
  - A Corpus Unit is one line, so its header is not in view. Noun-phrase
    bullets do occur (Call Damage Control: *• Target artifact card.*).
    Syntax alone cannot tell the two Readings apart.
  - The per-line Reading counts stay 4, 4, and 4.
- *• 1/2 blue Moonfolk with flying.* (Genku, Future Shaper) has a singular
  `NominalMode` Reading and a bare-plural `NounPhraseMode` Reading. *Moonfolk*
  is the same in singular and plural, so this is a lexical number Ambiguity.
  Its count stays 3.

## Change

- Retire `CostedModeBullet`.
- Stop `NominalMode` from admitting an imperative bullet as a noun compound.
  The fix belongs in the general nominal premodification that builds
  *[Counter target]*, not in a guard on `NominalMode` (`english-v4.md` §4.2).
  It must keep the characteristics bullets listed above.
- Add Judgments, for example:
  - `+ <card=Battle Menu> • Attack — Create a 2/2 white Knight creature token. => readings 1`
  - `+ • Counter target activated or triggered ability. => !Nominal "Counter target activated"`

Read before implementing: `docs/decisions/english-v4.md` §4.2;
`docs/decisions/english-lexical-analysis.md` §2. In `docs/oracle-style-guide.md`:
the front matter through "The shortest useful checklist"; §3 "Em dashes" and
"Bullets, vertical bars, and square brackets"; §10 "Modal choices" (with
"Pawprint modes"); §14 "Ability words and flavor words" and "Anchor words".

## Done when

- No Corpus Unit's Reading count goes up and no face is lost.
- Each of the three costed-bullet units has a single outer structure, so its
  Reading count is the product of its segment counts. The scratch probe is
  described in `english-v4-keyword-line-sentence-boundaries`.
- Acceptance set: each unit below has the count shown after the arrow (before
  → after).

- Battle Menu (2 → 1): *• Attack — Create a 2/2 white Knight creature token.*
- You're Ambushed on the Road (2 → 1): *• Stand and Fight — Target creature gets +1/+3 until end of turn.*
- School Daze (2 → 1): *• Fight Crime — Counter target spell. Draw a card.*
- Sublime Epiphany, Defabricate, Brokers Confluence (2 → 1): *• Counter target activated or triggered ability.*
- The four genuine units above keep their counts: Aetheric Amplifier (4, 4), Voracious Hydra (4), and Genku, Future Shaper (3).

Standard constraints apply.

Census delta (measured on urzkpqqo): faces read 28,709 → 28,709; total Readings 327,935 → 327,929; lost faces: none.
Judgment set (urzkpqqo): 2,976 enforced at `COMPLETED_STRATUM` 6, all hold; 195 pending.
