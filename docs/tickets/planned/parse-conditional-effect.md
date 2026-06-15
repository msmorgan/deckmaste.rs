---
needs: [engine-citys-blessing]
---
General conditional-effect grammar for the migrations effect parser:
"If [condition], [X] instead/otherwise" -> `Effect::If { condition, then,
otherwise }`, plus a condition-phrase recognizer (starting with "you have the
city's blessing" -> `Is(You, Designated("CitysBlessing"))`). Unblocks
graduation of the Ascend reader cards (Secrets of the Golden City, Golden
Demise, Kumena's Awakening, …) and the broader "if [state], [X] instead"
population. The blessing read and `Effect::If` resolution already work; only
the text parsing is missing.
