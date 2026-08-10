---
needs: [engine-payment-obligation-window]
design: true
---
Generalize the checked payment-binder boundary beyond the runnable subset used
by the explicit payment protocol.

The initial boundary supports chooser/search binders with no whiff effect,
exact top-level `Existing(Random(...))`, deterministic references/selections,
and a bound `Produce(Move(...))`. Unsupported nested random selections and raw
producer actions now fail closed before evaluation, and search `if_none`
effects are rejected because they carry arbitrary effect grammar with no
runnable proof. A chosen cost body also rejects a nested `ChooseAndPay`: every
choice in the current protocol must be its own locked IOU.

Design a checked binder/effect representation for search whiff branches,
additional producer verbs, and deliberately nested cost binders. It must expose
every choice and random outcome to the transaction transcript, preflight the
whole selected body atomically, and deserialize invalid plugin data as an
error. Keep deterministic `TheRef(Single(SelectAll(...)))` legal; absence of a
random operation is not itself a reason to reject a referent.
