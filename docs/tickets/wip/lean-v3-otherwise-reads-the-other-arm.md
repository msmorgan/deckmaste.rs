---
needs: []
---
# "Otherwise, that creature" reads the target the "if you do" arm announced

Standard constraints apply. Found landing `lean-v3-optional-arm-targets-stay-visible`, which made
an arm's targets visible after the whole `may` but kept each arm blind to the other's.

## Problem

*Insatiable Appetite*: "You may sacrifice a Food. If you do, target creature gets +5/+5 until end
of turn. Otherwise, that creature gets +3/+3 until end of turn." *Pippin's Bravery* has the same
shape. The target is written once, in the "if you do" arm, and "Otherwise, that creature" in the
other arm reads it. The target is chosen as the spell is cast, whichever arm later runs
[CR#601.2c,115.1], so the "if you don't" arm can name it.

## Change

In the `.withContinuation` arm of `stepOps` (`Check/Phrase.lean`), let the "if you don't" arm see
the targets the "if you do" arm announces (and the reverse), as ordinary referents, since both are
announced before either arm runs. Keep each arm's non-target referents its own.

## Tests

- Spell *Insatiable Appetite* and *Pippin's Bravery* in full; both check clean.
- A non-target object created in one arm is still not readable from the other.
