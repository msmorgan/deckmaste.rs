---
needs: [english-v2-stage-5-grammar-buildout-11-10]
---
Parameterize the remaining keyword stubs. Only 5 of 195 graduated stubs
carry parameter lists; "Kicker {2}", "Cycling {2}", "Ward—Pay 3 life."
fail, and two-parameter keywords ("Reinforce 3—{1}{G}") have no schema
yet. Extend the parameter schema for multi-parameter and em-dash-cost
forms (minding the pending macro-ron fold-back — payload shape, not more
standalone dialect), then author parameters across the stub inventory
from attested keyword lines. Undeclared forms keep failing silently.
Also carried: KeywordSubject enumerates two shapes instead of taking the
general reference-phrase category — "Enchant creature or Vehicle" fails
(43 of 45 attested units). Un-narrow it as part of this work. This ticket
now sits on the largest measured single-boundary bucket (~2,700 units one
byte after a complete keyword line).

Standard constraints apply.

## Realization ruling

The schema extension declares only typed parameter vectors, including
multi-parameter vectors such as `params: [Amount, Cost]`. The vector is the
payload shape intended to survive the pending macro-ron fold-back. Keyword
stubs must not declare separators, dashes, periods, or any other layout; a
stub-side layout field would relapse into the template model and is banned.

The compiler owns keyword-line realization. It derives the surface from the
declared parameter categories and the realized cost shape:

- a mana-only single cost uses a space form (`Ward {2}`);
- a sentence-shaped cost uses an em dash and terminal period
  (`Ward—Pay 3 life.`); and
- a count or quality parameter before a cost uses an em dash even when the
  cost is mana-only (`Reinforce 3—{1}{G}`).

Attested printed keyword lines remain the provenance for deciding which
parameter vector each keyword declares; punctuation is not declaration data.
