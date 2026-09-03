---
needs: [workbench-battlefield-gate-defect]
---
**Replace the sixteen anaphora constructors with one indexed `Pro` read over a
`Reach` index, and delete `ItPlayer` first.** 2026-09-02 workbench audit (F1).

`Phrase.Noun:1536-1567` has sixteen reads, each `{auto 0 ok : count<X> … bs =
1}` over a filter on `Binding`: `It` (`countOnes Object`), `ItAbility`,
`ItPlayer`, `ItAt sl` (`countOnesAt` = `itReaches` ∧ `slotZoneOk`,
`Words:1762-1769`), `ItVerbed v` (`itReaches` ∧ stamp, `Words:2006-2013`),
`ItToken` (`itReaches` ∧ origin, `Words:2062-2069`), `They`, `Them`,
`ThemVerbed`, `Those w`, `That w` (`countWord`, `Words:2181`), `ThatHalf w`
(`countUnionHalf`, `Words:2189`), `TheVerbed v w m`, `ThoseVerbed v w m`
(`verbedMatch`, `Words:2119-2160`). Each read drags four lookups (`count*`,
`provOf*`, `zoneOf*`, `tyOf*`), one of the eleven `setZone*` variants
(`Phrase:3249-3322`), and a clause in each of 13 noun-total functions.

`ItPlayer:1538` and `They:1550` carry the *same* gate `countOnes Player bs = 1`
and identical clauses everywhere (`nounEqRef:1618`, `nounZone:3471/3477`,
`nounPlur:3612/3618`); `ItPlayer` has 0 bench and 0 macro uses. `ItAt` has 0
bench uses — its only carriers `Macros.itAsPermanent:471` / `itAsCard:475` are
dead — and 1 proof.

## Fix

Delete `ItPlayer` first (S) as its own step, then:

```idris
data Reach = Bare | AtSlot SlotCarrier | Stamped VerbLabel | TokenBorn
           | Word NounWord | UnionHalf NounWord
           | Verbed VerbLabel NounWord VerbedMarking
```

one `reaches : Reach -> Plurality -> Binding -> Bool` (the union of the
existing `itReaches`/`itAtReaches`/`itVerbedReaches`/`itTokenReaches`/
`wordNow`/`halfReaches`/`verbedMatch`), one `countReach r pl bs`, and one
`Pro : (r : Reach) -> (pl : Plurality) -> {auto 0 ok : countReach r pl bs = 1}
-> Noun bs (reachKind r)`. `prov/zone/ty/setZone` become one function each
over `reaches`. `ItOtherThan co rest` / `ItPrior made before` are a different
axis (`bs = co ++ rest` splits) — keep them, or fold to one `ProIn : (seg :
Split) -> …`. Macros keep the surface names (`It`, `They`, `That w` …) so
bench text is unchanged.

This preserves the counted-uniqueness ruling exactly: the count is still `= 1`,
no recency, no ranking. The index is structural data, so call-site inference
improves against today's stuck-`Nat` equalities.

Size: L.

Done when: build is 23/23; `ItPlayer` and the other fifteen read constructors
are gone from `Phrase.Noun`; the eleven `setZone*` variants and the per-read
`count/prov/zone/ty` quintets are one function each; every bench witness that
used a deleted read still typechecks through its macro spelling; the anaphora
pins in `ProofsAnaphora` are re-spelled against `Pro` and remain non-vacuous.
Standard constraints apply.
