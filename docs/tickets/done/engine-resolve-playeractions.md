---
needs: []
---
Resolved the remaining `PlayerAction` verbs: GainLife, Discard, AddMana, Create,
Sacrifice, Exile, Untap, PutInLibrary (the verb landed earlier; resolution was a
todo). DiscardCards and ChooseManaColor surface as decisions. Create was deferred
to engine-tokens.
