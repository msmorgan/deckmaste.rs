---
needs: []
---
# english-v4: the depictive Comparative excludes

`Exclude(... Grade, Comparative)` sits on DepictiveAdjunct and
DepictiveAdjective (`english.ron`), while InterleavedDepictive and
AdjectivalDepictive lack it. The oncology reviewer proposed removing both,
under Principle 1 and consistent with F1.

Its CGEL cite is wrong. p. 1154 [47iv] is a *like* PP (*It would be better
like this*), not a comparative depictive. The depictive account is Ch. 4 §5,
pp. 251 and 261–263. None of the depictive examples there is comparative. In
p. 262 [29i], *They look even more fantastic naked*, the comparative is
*look*'s predicative Complement, not the depictive.

Measured (on top of `english-v4-bare-adjective-complement-priority`):
+26,628 Readings over 923 units, with no face lost. But Mayael's Aria goes
72 -> 4,608 and Whiskervale Forerunner 2,250 -> 4,750. Comparative *greater*,
or ComparativeBoundedAlternative *10 or greater*, becomes a depictive on
*control*, and the AdjunctCoordination *[with power 10] or [greater]* forms.
Each of Mayael's three *power N or greater* sentences goes 3 -> 12.

Open question: is a comparative depictive English (*She left richer than she
arrived*)? If it is, the exclusion is wrong in principle. Removing it still
needs the coordination and bounded-amount growth handled structurally first
(see F5: ComparativeBoundedAlternative and ComparativeBoundedAmount).

Standard constraints apply. No face lost; no unit over 4,096 Readings.

Resolved: comparative depictives are English. Both Comparative excludes are
gone. The bounded-amount growth is closed structurally: an `Amount`
ComplementKind Exclude on AdjectivalDepictive, DepictiveAdjunct and
DepictiveAdjective keeps *10 or greater* from every depictive (CGEL, Ch. 15
§3.1, pp. 1323–1324, rule [1]); without it, +16,477 Readings, Mayael's Aria
4,608, Whiskervale Forerunner 4,750. The remaining +9,975 Readings on 886
units are grammatical comparative depictives, accepted as such: (D1) a
comparative depictive coordinated with a PP Adjunct (*control a creature [with
power 10] or [greater]*, the bulk), (D2) a depictive with a *than* phrase
(*with mana value [less than X]*), (D3) a bare comparative after a measure
(*costs {4} [less]*). Rejected: excluding AdjP conjuncts from
AdjunctCoordination, which loses 9 faces (Deem Inferior, Happy Hogan,
Bodyguard, Lost Days, Missy, Noble's Purse, Sphere of the Suns, Temporal
Cleansing, Trickster's Stratagem, Wan Shi Tong, All-Knowing: *enters tapped
and with three charge counters on it*) and which CGEL does not support
(Ch. 15 §3.2, p. 1328: unlike categories coordinate freely in Adjunct function).

Census delta (measured on `oymrszup` → `qkzqzuoq`): faces read 32,825 → 32,825; total Readings 316,930 → 326,905; Surplus 134,519 → 138,504; lost faces: none.
Judgment set (`qkzqzuoq`): holds; 3,382 enforced, 0 pending; `comparative_depictives.txt` added (*10 or greater* is never a depictive; fails without the guard); `postpositive_comparative_property.txt:9` loses its `readings 2`, since the accepted D1 reading is a third grammatical one.
