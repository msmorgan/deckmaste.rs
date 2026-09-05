import Semantics
import Semantics.Macros
import Semantics.Check.Card
import Semantics.Cards.Anaphora
import Semantics.Cards.Copy
import Semantics.Cards.Deontic
import Semantics.Cards.Description
import Semantics.Cards.Faces
import Semantics.Cards.Piles
import Semantics.Cards.Turn

/-!
# Semantics.Cards

The printed-card bench: the Idris bench (`idris/src/Experimental/Cards/*.idr`) spelled with
the same macros, one `Semantics.Cards.<Family>` module per Idris family, imported here. A
leading dot marks a constructor; a bare name is a macro; a card is a `CardFace` around a
`Characteristics` literal naming the fields it prints.
Each card is a `Spelled`: its definition runs the checker by `decide`, so a refused spelling
does not define, as it did not elaborate in Idris.
-/
