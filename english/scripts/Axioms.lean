import Lean
import English

/-! Axiom audit for the workbench.

Enumerates every compiled theorem whose user-facing name (private names
resolved) is under `English.`, collects its transitive axiom dependencies, and
reports any use outside the permitted three. `sorryAx` is not permitted, so a
`sorry` anywhere in the closure fails this audit as well.

Run with `scripts/axioms`; it is deliberately outside the `English` library root
so `lake build` does not compile it into the model. -/

open Lean

namespace English.AxiomAudit

/-- The classical foundations the workbench is allowed to rest on. -/
def permitted : List Name := [``propext, ``Classical.choice, ``Quot.sound]

def audit : CoreM UInt32 := do
  let env ← getEnv
  let mut checked := 0
  let mut bad : Array (Name × Name) := #[]
  for (name, info) in env.constants.toList do
    unless info.isTheorem do continue
    unless (privateToUserName name).toString.startsWith "English." do continue
    checked := checked + 1
    for ax in (← collectAxioms name) do
      unless permitted.contains ax do bad := bad.push (name, ax)
  for (name, ax) in bad do
    IO.println s!"disallowed: {name} uses {ax}"
  IO.println s!"English theorems checked: {checked}"
  IO.println s!"disallowed axiom uses: {bad.size}"
  return if bad.isEmpty then 0 else 1

end English.AxiomAudit

unsafe def main : IO UInt32 := do
  initSearchPath (← findSysroot)
  let env ← importModules #[{ module := `English }] {} (trustLevel := 1024)
  let (code, _) ← English.AxiomAudit.audit.toIO
    { fileName := "axioms", fileMap := default } { env := env }
  return code
