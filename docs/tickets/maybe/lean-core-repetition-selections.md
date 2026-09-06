---
needs: []
---
# Design explicit selections carried between repetitions

Parked by the user on 2026-09-06. Replacing `againExcludingChosen` with carried
selection state is a repetition redesign, not necessary to fold `repeatTimes`
into the repetition family. Leave the existing policies intact for that fold.

Explore expanded repetitions with an explicit process body and iteration
policy, with macros resolving "repeat this process". Keep fixed count,
after-iteration decision, and stopping-condition semantics distinct. Model
fresh choices separately from accumulated earlier choices; preserve outer
bindings, per-iteration scope, result publication, and cost admissibility.

Use Ad Nauseam, Another Round, and Forgotten Lore to demonstrate the different
policies and fresh versus carried selections. Show how exclusion reads reach
the intended iterations without leaking unrelated choices. Compare the
replacement's complexity with the existing policy before choosing a design.
Standard constraints apply.
