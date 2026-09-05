---
needs: []
---
**The fused keyword surface lowercases its Keyword Quality in running text.**
`english-v2-landwalk-quality-keyword` landed the Bound Keyword Surface: the
Keyword Quality is read from the type declarations and the keyword stem is
bound to it, so *Islandwalk*, *Snow landwalk* and *artifact landwalk* all
analyze as one keyword-line item. Printed Oracle text writes that same fused
word in lowercase wherever it is not line-initial — *Enchanted creature has
mountainwalk.*, *Target creature gains islandwalk until end of turn.*,
*Creatures with forestwalk can be blocked…* — and the land Subtype declaration
has no running-position lowercase reading, so those units stop at the fused
word.

Measured on the corpus at that landing: 199 faces carry a basic fused walk
surface, 123 carry a capitalized one, and **76 carry only lowercase ones**.
Sampled failures stop inside the fused word, not elsewhere in the text
(Burrowing at bytes 40..52, Crevasse 15..27, Deadfall 15..25, Coral Barrier
80..90).

What to decide and build: how a declaration-backed Keyword Quality realizes
and reads in the lowercase running-position fused form, without a per-surface
row and without a guard naming a type or a card. The candidate shape is a
case rule on the Bound Keyword Surface construction — the fused word is one
lowercase orthographic word except where sentence casing capitalizes its first
letter — not a second lowercase surface on each type declaration. Rendering
must stay byte-exact in both cases, and the capitalized readings this landing
covers must not change their selected analysis.

Vocabulary: `docs/contexts/oracle-english/CONTEXT.md` (Bound Keyword Surface,
Keyword Quality). `[CR#702.14a]` gives the surface; `[CR#702.14c]` gives the
separated and stacked readings.

Tier: **sol**. Standard constraints apply.
