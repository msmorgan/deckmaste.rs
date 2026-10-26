import Semantics.Definitions
import Semantics.Check.Card
import Semantics.Cards.Static
import Semantics.Cards.Keyword

/-!
# Semantics.Proofs.UntapStep

The untap-step statics: what "doesn't untap", "untap … during each other player's untap step"
and "you may choose not to untap" expand to. In the untap step the active player determines which
permanents they control will untap, then untaps them [CR#502.3]; that untapping is the `untap`
Deed [CR#701.26b].
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.UntapStep

mutual
  /-- The named template and definition calls an authoring tree retains. -/
  private def Form.calls : Authoring.Form → List Lean.Name
    | .call name children => name :: Forms.calls children
    | .data children | .constructor _ children => Forms.calls children
    | .binder _ body => Form.calls body
    | .literal _ | .parameter _ | .raw _ => []

  private def Forms.calls : List Authoring.Form → List Lean.Name
    | [] => []
    | form :: forms => Form.calls form ++ Forms.calls forms
end

/-- "<subject> doesn't untap during [<whose>] untap step" forbids the `untap` Definition's Deed,
so it and a "can't untap" pattern constrain the same Deed. -/
theorem doesntUntapForbidsTheUntapDeed (subject : NounPhrase) :
    (untap subject).namedDeed = some (.action "untap") := rfl

/-- The stun counter's replacement effect [CR#122.1d]: "If a permanent with a stun counter on it
would become untapped, instead remove a stun counter from it." -/
private def stunReplacement : Ability :=
  static (replace (would any (untap (a (and [isPermanent, hasCounter (some stunCounter)]))))
    (instead (removeCounters (exactly 1) stunCounter it)))

/-- UD-14 (`docs/engine-adrs.md`): "doesn't untap" is a prohibition, the engine's choice where
the CR is silent, not a replacement. It forbids, with `cant`, the very `untap` Deed the stun
counter's replacement watches, so on a tapped permanent with both the untap can't happen, the
stun replacement can't apply to it [CR#614.17c], and the stun counter stays; as a replacement
it would instead be ordered against the stun counter's [CR#616.1]. -/
theorem doesntUntapIsNoStunReplacement :
    (untap thisPermanent).namedDeed =
        (untap (a (and [isPermanent, hasCounter (some stunCounter)]))).namedDeed ∧
      Ability.check (DRS.ofList []) stunReplacement = [] :=
  ⟨rfl, by decide⟩

/-- The four untap-step cards are written with `untapDuring`, so each expands through `must`. -/
theorem untapStepCardsUseUntapDuring :
    (Form.calls Cards.seedbornMuse.authoring).contains ``untapDuring ∧
      (Form.calls Cards.unwindingClock.authoring).contains ``untapDuring ∧
      (Form.calls Cards.thousandMoonsInfantry.authoring).contains ``untapDuring ∧
      (Form.calls Cards.drumbellower.authoring).contains ``untapDuring := by
  decide

/-- "You may choose not to untap <subject> during [<whose>] untap step" is no permission to untap
it: it is the "doesn't untap" prohibition, holding only if you so choose. Untapping stays the
default [CR#502.3]. -/
theorem mayDeclineUntapIsChosenProhibition (subject : NounPhrase) (whose : Option NounPhrase) :
    mayDeclineUntap subject whose ≠ duringUntapStep whose (can any (untap subject)) := by
  intro h; injection h with h; cases h

/-- The choice is a player's: "if this artifact so chooses" is refused. -/
theorem soChoosesNeedsAPlayer :
    Condition.check (DRS.ofList []) (.soChooses you) = [] ∧
      Condition.check (DRS.ofList []) (.soChooses thisArtifact)
        = [.kindMismatch .player .object] := by
  decide

/-- *Ashnod's Battle Gear* and *Helm of Possession* still check. -/
theorem mayDeclineUntapCardsCheck :
    Ability.check (DRS.ofList []) Cards.ashnodsBattleGear = [] ∧
      Cards.helmOfPossession.card.check = [] :=
  ⟨Cards.okAshnodsBattleGear, Cards.helmOfPossession.ok⟩

end Semantics.Proofs.UntapStep
