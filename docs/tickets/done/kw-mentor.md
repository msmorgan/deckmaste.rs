---
needs: []
---
# kw-mentor — Mentor + Training keyword macros

Author the **Mentor** [CR#702.134a] and **Training** [CR#702.149a] keyword
macros (Keyword-kind, `plugins/builtin/macros/keyword/`), backing the
`Keyword(Mentor)` / `Keyword(Training)` emissions so cards carrying them
graduate.

- **Mentor**: "Whenever this creature attacks, put a +1/+1 counter on target
  attacking creature with power less than this creature's power." A this-attacks
  trigger (`StateBecomes(Ref(This), Attacking)`) + a TARGET whose filter carries
  a cross-object stat comparison ("power less than this creature's power") +
  `PutCounters(Target(0), P1P1Counter, 1)`.
- **Training**: "Whenever this creature and at least one other creature with
  power greater than this creature's power attack, put a +1/+1 counter on this
  creature." Same attack trigger + an intervening-if existential ("another
  attacker has greater power than this") + `PutCounters(This, P1P1Counter, 1)`.

Engine gap closed: the **target candidate enumeration path was frameless**
(`watcher = None`), so a target filter referencing the carrier
(`Ref(This)` / `StatOf(This, …)`) tripped `todo!`. Threaded the targeting
object's `ObjectSource` as the watcher through `legal_targets` /
`surface_target_choice` / `targets_still_legal` (announce + [CR#608.2b] recheck)
and added `target::candidates_with`. The cross-object comparison itself reuses
the already-wired `Filter::Where` / `condition_holds` path (no new comparison
machinery). Training's intervening-if uses the frame-watcher-threaded
`Count::CountOf` path, which already supported carrier-relative filters.
