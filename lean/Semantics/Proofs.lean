import Semantics.Proofs.CharacteristicEdits
import Semantics.Proofs.KeywordArguments
import Semantics.Proofs.EventPatterns
import Semantics.Proofs.Attachments
import Semantics.Proofs.ActionFamilies
import Semantics.Proofs.Composition
import Semantics.Proofs.Lookback
import Semantics.Proofs.ControllerSacrifice
import Semantics.Proofs.ReferenceScopes
import Semantics.Proofs.Authoring
import Semantics.Proofs.Anaphora
import Semantics.Proofs.Choice
import Semantics.Proofs.Counters
import Semantics.Proofs.Damage
import Semantics.Proofs.Deontic
import Semantics.Proofs.Description
import Semantics.Proofs.Faces
import Semantics.Proofs.InstructionForms
import Semantics.Proofs.Keyword
import Semantics.Proofs.Mana
import Semantics.Proofs.Numbers
import Semantics.Proofs.NounWords
import Semantics.Proofs.Piles
import Semantics.Proofs.Static
import Semantics.Proofs.Tables
import Semantics.Proofs.Trigger
import Semantics.Proofs.TypeEvidence
import Semantics.Proofs.Turn
import Semantics.Proofs.Zone

/-!
# Semantics.Proofs

The pin suites: the Idris `Proofs/<Family>` modules as `decide` theorems over the checker, one
module per grammar family. `lake build SemanticsProofs` runs them all.
-/
