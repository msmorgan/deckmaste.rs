---
needs: [core-regions-substrate]
---
**Engine: make `Count::StatOf` consume a live-or-LKI reference result instead
of a bare `ObjectId`.** Today power/toughness route a stale or null id into
`LayeredView`'s live-object assertion, while mana value routes it into the live
definition lookup. The count evaluator cannot distinguish “this role has valid
last-known information” from “this announced target is now illegal and must not
be used.”

Use the unified reference channel to make that distinction explicit:

- live subjects read derived characteristics;
- source/event/cost roles that carry valid LKI read characteristics captured at
  the required time, including derived power/toughness rather than silently
  falling back to printed values; and
- unavailable or current-only operands produce an unavailable count that makes
  the consuming instruction no-op. They never become a raw null/stale lookup.

Foundations LKI witnesses: Heartfire Immolator after sacrificing itself;
Halana and Alena, Partners and Prime Speaker Zegana after the trigger source
leaves; Ovika, Enigma Goliath after the triggering spell leaves the stack.
Current-only negative witnesses shared with `engine-departed-reference-fizzle`:
Bite Down, Felling Blow, and Heroes’ Bane.

Tests must distinguish the LKI-positive and current-only cases and verify the
card result, not normalize every missing stat to zero indiscriminately.
