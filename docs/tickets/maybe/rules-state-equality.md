---
needs: []
---
Address UD-11: No equality predicate exists for `GameState`, which prevents the engine from detecting mandatory loops as required by [CR#104.4b] (a game that consists of a "loop of mandatory actions" is a draw).

Tasks:
1. Define a `GameState` equality predicate (or a stable hash) that includes the relevant components (objects, zones, turn state, event log sequence) but excludes transients.
2. Implement a loop monitor that tracks recent states.
3. Trigger a draw outcome when a mandatory loop is detected.
