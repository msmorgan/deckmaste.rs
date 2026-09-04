---
needs: []
---
**Make every read one `Pro` over a window; delete `Own` and `ItOtherThan`.**
Ruling 2026-09-04 on reads (cleanroom review 3, D-Q1; D-Q18 folds in).

- **One constructor.** `Phrase.Noun.ItOtherThan co rest {sp : bs = co ++
  rest}` and `Phrase.Noun.Own r pl own outer {sp : bs = own ++ outer}` carry
  the binding stack as term data. Both go, and the surviving read is
  `Pro : (r : Reach) -> (pl : Plurality) -> (w : Window) ->
  {auto 0 ok : countReach r pl (view w bs) = 1} -> Noun bs (reachKind r)`,
  with `Window = Whole | Top n | Below n` over a `Nat` and `view` taking or
  dropping. Counted uniqueness over the window is the only anaphora gate; no
  `bs = _ ++ _` obligation survives anywhere in the tree.
- **The number is derived, never printed.** Macros compute it:
  `ownSubject n = Pro Bare pl (Top (length (selfSubjDelta n ++ nounDelta n)))`,
  `itPrior prev = Pro Bare OneOf (Top (length (instrDelta prev)))`.
  `Macros.agentRef` re-reads the agent through the window rather than
  splitting on which reference the agent is, and `Macros.itPrior` stops
  re-quoting the whole antecedent instruction (D-Q18 — `Counters.thranduilsCompany`,
  `Counters.stunningShot`).
- **The proof machinery goes with the list arguments.** `appendAssociative` in
  `Macros.ownSubject`, the `replace {p = …} (sym nd)` in `Macros.lookedSlice`,
  and the nine obligations `Macros.scry`, `surveil` and `fateseal` each
  restate to forward them (`lookedSpilled` over `agentLookedDelta`/
  `agentLookedOuter`/`agentLookedPlur`) — one obligation on the window
  replaces the lot. The dead scry cluster listed in `workbench-dedup-tables`
  is resolved here.
- **Re-spell the sites.** The bench and pins carry 12 `Own …` spellings, 7
  `{sp = …}` handles and 5 `Refl` handles; the bench carries no implicit
  handles at all. Re-spell each as printed and re-probe every touched pin.

Size: M. Done when: `Own` and `ItOtherThan` are gone from the tree; `Pro` is
the only anaphoric noun and takes a window; no `bs = _ ++ _` obligation
remains; `grep -E '\{[a-zA-Z_]+ *= ' idris/src/Experimental/Cards/*.idr` is
empty; every re-spelled pin is probed non-vacuous; build at its module count.
Standard constraints apply, including the RON-shaped constraint.
