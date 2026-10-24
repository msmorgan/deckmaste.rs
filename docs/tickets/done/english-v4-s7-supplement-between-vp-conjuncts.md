---
needs: []
---
# Stratum 7: a Supplement after a non-final Verb Phrase conjunct

Part of `english-v4-stratum-7-discourse-and-long-tail`; split from `english-v4-s7-long-tail-edge-cases`. Implement in `crates/deckmaste_english_v4` (and `deckmaste_syntax` if the construction needs an engine capability) per [`docs/decisions/english-v4.md`](../../decisions/english-v4.md) §4.4.

Witness: **Collective Voyage**, *Each player searches their library for up to X basic land cards, where X is the total amount of mana paid this way, puts them onto the battlefield tapped, then shuffles.* (its unit is in `deferred.txt` under this ticket).

Finding (verified on the witness): the comma-set Supplement is anchored to a finite Clause only (`english.ron`, the `host_kind: Supplement` Clause Adjunct, CGEL Ch. 15 §5.1, p. 1351), which is right at a clause's end: a Verb Phrase site there would be a second tree of one analysis. A declarative with a shared Subject coordinates Verb Phrases, so no Clause ends after the first conjunct to host it. The same sentence reads with the Supplement last, and imperative siblings (Boundless Realms, Jaheira's Respite) read because their conjuncts are Clauses.

Decided shape: the Supplement attaches to a Verb Phrase conjunct only where it is not the last conjunct, so a clause-final Supplement keeps its one Clause site. No `deckmaste_syntax` capability restricts a dependent to non-final conjuncts today: the Coordination's `initial.` paths constrain features, but every other Verb Phrase consumer would also have to reject a supplemented Verb Phrase. Design that capability (or an equivalent structural route) first, then measure that no unit gains a second Supplement site.

## Landing note

Per `docs/tickets/README.md`, end the ticket with the two landing lines.

**Census delta** (`svkxlwso`): faces read 32,813 → 32,814 of 32,828; Readings 338,233 → 338,243 (Collective Voyage 0 → 8, the internal ambiguities its clause-final variant also has; Candlekeep Inspiration 2 → 4, a grammatical second grouping in which *have …, where X is … instant cards* coordinates with *are sorcery cards, and/or have an Adventure*); lost faces: none.
**Judgment set** (`svkxlwso`): all hold (`cargo test -p deckmaste_english_v4 --test judgments`, 6 passed).
