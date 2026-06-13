---
needs: []
---
Subtype macros now register under their printed-string argument rather than
filename, so parametric subtype refs find their target and their cards graduate
instead of stalling as todos. Required the template-param refactor.
