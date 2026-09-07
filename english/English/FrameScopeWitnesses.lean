import English.FrameScope
import English.FrameInteractions

/-! Concrete boundary-variation witnesses for `English.FrameScope`, over the `FrameInteractions`
lexicon. Homed apart from the model module so `English.FrameScope` imports no witness module. -/
namespace English.FrameScope
open GrammaticalScope
namespace Witnesses
open FrameInteractions
open Documents (Witness)

def cluster (a b d e : Witness Lexeme lexicon nominalCategory) :
    Witness Lexeme lexicon (.verbPhrase .plain) :=
  ⟨.node (.verb .put .plain markedFrame) [pairs .and_ .on a.tree b.tree d.tree e.tree],
   (["put"] : Surface) ++ a.surface ++ (["on"] : Surface) ++ b.surface ++ (["and"] : Surface) ++
     d.surface ++ (["on"] : Surface) ++ e.surface,
   .verb ⟨rfl,rfl,rfl,Or.inr rfl⟩ (.coordinate
     (.argument (slot := ⟨.object,nominalCategory⟩) a.derives
       (.marked (slot := role) b.derives .nil))
     (.argument (slot := ⟨.object,nominalCategory⟩) d.derives
       (.marked (slot := role) e.derives .nil))),
   by
     have h := Realizes.node (.cons (Realizes.frameCoordination (coordinator := .and_)
       (.cons a.realizes (.cons (.marker (Or.inl ⟨rfl,rfl⟩)) (.cons b.realizes .nil)))
       (.cons d.realizes (.cons (.marker (Or.inl ⟨rfl,rfl⟩)) (.cons e.realizes .nil)))) .nil)
       (Linearizes.verb (lexicon := lexicon) (head := .put) (form := .plain)
         (frame := markedFrame) (voice := .active) (v := ["put"]) ⟨rfl,rfl,rfl⟩)
     simpa [pairs, Coordinator.surface, List.append_assoc] using h⟩

def left := cluster (pair creatures artifacts) (pair creatures artifacts) creatures artifacts
def right := cluster (pair creatures artifacts) creatures (pair artifacts creatures) artifacts

theorem same_surface : left.surface = right.surface := rfl

theorem both_written :
    Written lexicon left.tree (.verbPhrase .plain)
      "put creatures and artifacts on creatures and artifacts and creatures on artifacts" ∧
    Written lexicon right.tree (.verbPhrase .plain)
      "put creatures and artifacts on creatures and artifacts and creatures on artifacts" := by
  have spells : Spells left.surface
      "put creatures and artifacts on creatures and artifacts and creatures on artifacts" :=
    .cons (.cons (.cons (.cons (.cons (.cons (.cons (.cons (.cons (.cons (.cons .single))))))))))
  exact ⟨⟨_,⟨left.derives,left.realizes⟩,spells⟩,
    ⟨_,⟨right.derives,right.realizes⟩,spells⟩⟩

theorem distinct : left.tree ≠ right.tree := by intro h; cases h

def leftReading : SchemaWitness lexicon (.verbPhrase .plain) left.surface :=
  ⟨left.tree,left.derives,left.realizes⟩
def rightReading : SchemaWitness lexicon (.verbPhrase .plain) left.surface :=
  ⟨right.tree,right.derives,right.realizes⟩

theorem related : Related leftReading rightReading :=
  .step (.node (.verb Lexeme.put .plain markedFrame) [] []
    (.boundary (.exchange .and_ nominalCategory Lexeme.on creatures.tree artifacts.tree
      creatures.tree artifacts.tree creatures.tree artifacts.tree)))

theorem both_packed :
    (Preference.pack (fun a ↦ a = leftReading ∨ a = rightReading) key (key leftReading)).readings
      leftReading ∧
    (Preference.pack (fun a ↦ a = leftReading ∨ a = rightReading) key (key leftReading)).readings
      rightReading :=
  ⟨⟨Or.inl rfl,rfl⟩,⟨Or.inr rfl,(key_exact _ _).mpr (.symm related)⟩⟩

theorem same_role_count : RolePreference.roleCount left.tree =
    RolePreference.roleCount right.tree := rfl

theorem no_boundary_preference :
    ¬ RolePreference.Prefers lexicon left.tree right.tree ∧
    ¬ RolePreference.Prefers lexicon right.tree left.tree :=
  ⟨RolePreference.equal_role_counts_cannot_prefer same_role_count,
   RolePreference.equal_role_counts_cannot_prefer same_role_count.symm⟩

end Witnesses
end English.FrameScope
