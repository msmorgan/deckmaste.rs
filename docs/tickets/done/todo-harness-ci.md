---
needs: []
---
**Run the ticket-graph harness in CI.** `tests/todo_test.fish` and its fixture
tree exercise ready/blocked selection, cycles, dangling census dependencies,
and graph traversal, but no script or workflow invokes the harness. A graph
tool whose own fixtures never run is rot-prone infrastructure.

Add Fish to the existing CI system-package setup and run
`fish tests/todo_test.fish` in a named step. Keep the harness hermetic through
its `TODO_ROOT`/`TODO_CENSUS` fixture overrides and make failures readable in
the Actions log. Add a local contributor command beside `scripts/todo`'s
documentation.

Documentation alone does not close this ticket: the harness must execute on
every ordinary CI run, and one focused negative fixture must prove the step
fails when the graph checker regresses.
