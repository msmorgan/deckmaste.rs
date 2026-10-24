---
needs: []
---
# english-v4: bare-adjective complement priority

A follow-up of the post-Stratum-7 oncology re-scan (Spiteful Repossession).

A Reading took the relative clause *who controls more lands than you equal*:
bare *equal* (`Valence:AdjectivePhrase/Bare`, its PP[to] omitted) was a
DepictiveAdjective on *controls*, which stranded *to the difference* for
*opponent*'s `Complement(PP(["to"]))` across the relative clause
(InterleavedRelative). An omitted complement whose own PP then attaches to an
outer host is not English.

Fix: same-head complement priority on `Valence:{phrase}/Bare`
(`deckmaste_syntax` compile.rs, `bare.omits_complement`) now holds for every
non-verb head, not only nouns. An adjective's PP Complements are licensed by
the head, which picks the preposition, and are mostly optional, as a noun's
are (CGEL, Ch. 6 §3.1, pp. 542–545; Ch. 5 §14.1, pp. 439–441). Every one of
the 521 Readings removed strands a bare adjective's own selected PP (*equal*
+ *to*, *tied* + *for*, *true* + *for*). The alternative, that bare *equal*
needs a plural or reciprocal predicand, is not CGEL's: a singular predicand
takes an understood secondary term (Ch. 13 §5, p. 1137).

The depictive Comparative excludes, first planned here, moved to
`english-v4-depictive-comparative-excludes`.

Standard constraints apply. No face lost.

Census delta (measured on nsszmxxl): faces read 32,825 -> 32,825 of 32,828; Readings 324,006 -> 323,485 (112 units down); lost faces: none.
Judgment set (measured on nsszmxxl): all judgments hold, including the new Spiteful Repossession entry in `difference_noun.txt`.
