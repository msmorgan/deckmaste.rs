---
needs: []
---
# Serial coordination: unlike conjuncts and mixed moods

Two general limits on serial coordination keep ordinary Oracle text unread.

1. **Unlike serial conjuncts.** An unlike serial coordination must currently
   differ between its first and last conjuncts. Nahiri, the Lithomancer's −10
   has *It has indestructible, "Equipped creature gets +5/+5 and has double
   strike," and equip {0}*: keyword, quoted ability, keyword as the Object of
   *has*. The condition should be that the conjuncts are not all of one kind,
   not that the first and last differ (CGEL, Ch. 15 §3.2, pp. 1326–1329,
   coordination of unlike categories; §3.1, p. 1323, [1]: likeness of function).
2. **Mixed moods in a serial.** Binary clause coordinations already admit an
   imperative beside a declarative; the serial shapes do not. Kaboom! (*reveal
   cards … until you reveal a nonland card, Kaboom! deals damage …, then you put
   the revealed cards …*) and Clackbridge Troll (*tap this creature, you gain 3
   life, and you draw a card*) need it, including with a *then*-linked last
   conjunct (see the Linked Coordination added by
   `english-v4-s7-then-coordination`). Mixed clause types: CGEL, Ch. 15 §3.5,
   pp. 1332–1333.

`english-v4-s7-gapping-and-ellipsis` landed part of item 2: a serial
coordination of Clauses with an imperative first and a declarative last
admits either mood between (Clackbridge Troll, Opportunistic Dragon, Gandalf,
Westward Voyager now read); the *then*-linked last conjunct (Kaboom!) remains.

Witnesses: Nahiri, the Lithomancer; Kaboom!; Clackbridge Troll. Check the
corpus for other units blocked only by these two limits.

## Constraints

Governance is `docs/decisions/english-v4.md` §4.4, Principles. Measure the
census before and after, unit by unit; one tree per analysis for each serial.
Standard constraints apply.

## Landing note

Per `docs/tickets/README.md`, end the ticket with the two landing lines.

Landed: an unlike serial or polysyndetic coordination's tail records whether
its own conjuncts already differ (`Summary::unlike_mixed`), so the whole needs
only that its conjuncts not all be alike (Nahiri now reads, one tree). The
then-linked mixed-mood schema takes the same shape as the coordinator one
(Kaboom! reads). Generalizing the mood serial alone would also admit a
declarative first with an "imperative" between, the misanalysis of Aang,
Master of Elements and Bound in Gold (*can't attack, [block, or crew
Vehicles], and ...*); a new `initial.` equation path keeps both mood serials
imperative-initial, and Aang (coordination at unlike levels, CGEL Ch. 15 §3.6)
moved to `english-v4-s7-gapping-remnants`. No other corpus unit was blocked
only by these two limits. (The ticket first cited CGEL Ch. 15 §4.4, which is
delayed right constituent coordination; corrected to §3.2.)

1. **Census delta** (measured on `vrnmrlxz`): faces read 32,734 → 32,736 (+2, Stratum 7: Nahiri, the Lithomancer; Kaboom!); total Readings (derivation product) 341,562 → 341,611 (+49), Surplus 141,589 → 141,613; lost faces: none.
2. **Judgment set**: `tests/judgments.rs` passes; the new entries (Nahiri, Kaboom!, three then-linked mixed-mood serials) hold, and the census reports 0 known-wrong-only.
