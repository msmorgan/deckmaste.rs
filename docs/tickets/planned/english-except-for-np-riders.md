---
needs: []
---
**Parse `except (for) <noun-phrase list>` exception riders.** Diagnosed
2026-07-25 (round pluralposs Rev 2): `ExceptionRider` licenses only the
clause-attachment form `[l(L::Except), n(N::Clause)]`
(`crates/deckmaste_english/src/grammar/clause.rs`); there is no production for
`except for <coordinated NP>` (or bare `except <NP>`) attached to an
imperative or its object. After the pluralposs opacity-invariant repair these
faces are honest whole-sentence recoveries, e.g. Whelming Wave `Return all
creatures to their owners' hands except for Krakens, Leviathans, Octopuses,
and Serpents.`, Mageta the Lion `Destroy all creatures except for Mageta.`,
Scourglass, Slash the Ranks, Elspeth Tirel, The Argent Etchings, Blood Sun
(`except mana abilities`, no `for`), Flame Sweep, Scheming Fence — 40+ rows in
the unknown dump contain `except`/`not` residue, a large subset being this
rider shape. Design needs: a rider form taking a coordinated NP list (Oxford
comma), an attachment-site ruling (clause-level vs object-NP-level — semantics
differ), renderer inverse, and interaction with NP coordination. The `not
<participle> this way` reduced-relative family (Celestial Judgment, Thunderwave,
`not cast this way` ×5) is a sibling gap, distinct machinery. See
`recovery-harness` round records (pluralposs) for per-face attribution.

**Scope narrowed 2026-07-27 (round exrider).** The `except` population splits
into six constructions, not one. Round exrider took the largest — `except by
<PP>` predicate exception tails on passive restrictions, 54 rows — and landed
it as `PredicateAdjunct::Exception(PrepositionalPhrase)` with an append-last
`VerbPhrase -> VerbPhrase Except PrepositionalPhrase` production. **This
ticket now covers ONLY the class-B set exclusions: 19 rows**, `except for
<NP>` and bare `except <NP>` (Whelming Wave, Mageta the Lion, Blood Sun,
Scourglass, Flame Sweep, Scheming Fence, The Argent Etchings, Elspeth Tirel,
Slash the Ranks, Slinn Voda, Cyclone Summoner, Total War, Mechtitan Core,
Inspire Awe, Season of the Witch, Eye of Singularity, Sharkey, Keldon
Firebombers, Unstable Glyphbridge).

Settled design direction from the exrider plan:

- The attachment site is the **nominal**, not the clause: these restrict the
  denotation of a set (`all creatures except for Mageta`), including NPs
  nested inside other complements. Do not choose a clause attachment merely
  to avoid the nominal hazard, and do not reuse the comma-bearing clausal
  `ExceptionRider` — that would erase scope and make its comma optional for
  unrelated existing syntax.
- Provisional shape: a dedicated `NominalComplement::SetException { marker:
  Bare | For, excluded: NounPhrase }`.
- **Hazard:** a production widening `N::Nominal`/`N::NounPhrase` is the exact
  parser surface implicated by the `dealtdmg` perturbation. The round must
  settle its dot-1 gate from the live `Features::Nominal` state, requiring the
  first nominal to be determined and in a legal terminal attachment phase; if
  that state cannot categorically exclude open noun fragments at dot 1, abort
  the rule rather than relying only on reduce. Capture and compare the full
  unresolved dump, since such a production can perturb FIFO discovery even
  when its reductions never win.
- **No new coordination fold is needed.** Existing NP coordination already
  yields Oxford lists (Akroma's Vengeance) and `and/or` (Path of Mettle) as a
  single `CoordinatedNounPhrase`; verified twice, 2026-07-27.
- Out of scope for this ticket, separately diagnosed by exrider: `except
  <clause>` copy modification (20 rows, already has clausal `ExceptionRider`
  machinery — those faces fail for unrelated interior reasons), draw-event
  `except <NP>` (8 rows, Orcish Bowmasters / Notion Thief / Hullbreacher),
  `except that <Clause>` (3 rows, Fork), `except during <PP>` (Defense Grid),
  `except those that …` (Everything Comes to Dust), and the fronted `Except
  for …,` (Akron Legionnaire), which needs a separately justified fronted
  attachment.
