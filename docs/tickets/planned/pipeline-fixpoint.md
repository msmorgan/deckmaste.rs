---
needs: []
---
Dependency-ordered re-graduation (subtypes → keywords → cards) instead of the
current single-pass pipeline. Required once cross-file dependencies mean a card
can't graduate until a macro it depends on has graduated first.
