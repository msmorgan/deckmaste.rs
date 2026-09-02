---
needs: [english-v2-stage-5-grammar-buildout-11-10]
---
Dissolve the noun-as-literal frames (gaming audits 2026-09-02:
docs/memory/scratch/plan09-postmortem/gaming-audit-static.md and
gaming-audit-dynamic.md — both converge: ~40% of selected units, 5,5-5,8k,
route through cop-out constructors; no sentence memorization exists).

G1 (5,026 units): verb-specific literal-tail frames seeded in Rust
(environment.rs:1070-1330) with MTG-named VerbPhrase constructions — the
noun is a frame literal ("damage", "life", "mana of any", "card",
"counter", "at", "with … counters", "base power and toughness"), so those
phrases are never NPs. Replace with general frames + real nominals:
damage/life/mana as mass common nouns under a quantity licence, "mana
cost" as a compound noun, counters as ordinary NPs, and core verbs moved
out of Rust into declaration data with linguistic valences only.
G2 (589): at_phrase — fixed clause with vocab-slot tokens ("each
opponent's", "on your turn") and a six-arm require that is corpus census;
replace with a general temporal PP trigger prefix (also admits the 306
excluded "next end step" units).
G3-G7: mana-cost NP as four literal constructions; "as though" as three
fixed shapes; "each combat if able"; "blocked (except) by" with the
participle literal (use the Block participle); shadow NP grammars for
card/counter/die/owner/control; `require possessor is Their`.
Shadowing (2,662 units): draw_cards, put_counters, put_into,
passive_finite_clause, copular_clause win over existing compositional
parses purely on literal specificity — delete outright.
Dead grammar: 71 constructions route zero units, including the general
Possessive root; delete or make them the live path (the possessive
literals in G2/G3/G7 carry ~750 units that belong to it).
Multi-word vocab tokens ("the beginning of", "combat damage", "end step",
"an opponent chooses") dissolve into phrases.

COMPILER TRIPWIRE (required, lands first): a Literal tail atom or form
literal whose surface equals a declared noun or verb surface is a load
error. Closed-world decidable; kills the incentive permanently. Add a
structural-depth or literal/lexicon-collision metric to the gate summary
so this class is visible in the census (the ratchet counts units only).

Acceptance: coupled dissolution — every replacement lands with its
deletion; zero net coverage loss (deleting without replacement would drop
~4,214 units); expect NET GAIN from the excluded siblings ("gains flying"
x290, "as though it were" x83, "mana of any type" x56). Naming rule:
constructions named for linguistic shape, never lexeme or mechanic. Tie
during dissolution not explained by a not-yet-deleted rival = STOP.
Standard constraints apply.

Fold-in (13-10 landing, 2026-09-02): the ruling that froze the seed table
means `gains "…"` (135 faces) cannot take `Role("QuotedAbility")` until
core verbs live in declaration data; the `have` frame already does. The
declaration-side route for a core verb's frame lands here — the quoted
grant is its first consumer.

Coordinator amendments (modal landing review, 2026-09-02):
- LEVEL ruling (F1): the `"LEVEL"` literal (constructions.rs ~:4917)
  collides case-insensitively with the declared designation surface
  `level`. Ruling: the leveler band label is the Level Up keyword
  ability's grammar contribution — move it to that stub's declared
  surfaces and delete the core literal; the compiler tripwire compares
  exact surfaces (case-sensitive), so no exemption is needed once moved.
  Land this before the tripwire so it never has to grandfather shipped
  code.
- Inventory refresh: "an opponent chooses" and "one or both" were
  dissolved by the 13-10 round — strike them; re-run the zero-routing
  construction census (the pawprint arm changed it) before deleting.
- This ticket also owns the unimplemented half of the 2026-09-02 ruling:
  the literal/lexicon-collision (or structural-depth) metric in the gate
  summary.
- The frame-key-reduction -> 14-10 needs edge is RATIFIED by the
  coordinator: dissolution precedes the long-tail loop by design.
