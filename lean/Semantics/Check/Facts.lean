import Semantics.Words

/-!
# Semantics.Check.Facts

The registry table the checker reads: the keyword-action label list. Hand-maintained Lean, the
source of truth: `Proofs/Tables` pins its labels distinct and against the checker's `actFacts` in
both directions. Keyword abilities, counters, designations, relations and subtypes have no table:
each is a self-contained declaration in `Semantics.Definitions`, and the authoring boundary
refuses a raw one in a card.
-/

namespace Semantics

def keywordActionLabels : List KeywordActionLabel :=
  [ "activate",
    "adapt",
    "airbend",
    "amass",
    "assemble",
    "attach",
    "behold",
    "blight",
    "bolster",
    "cast",
    "clash",
    "cloak",
    "collectEvidence",
    "connive",
    "convert",
    "counter",
    "create",
    "destroy",
    "detain",
    "discard",
    "discover",
    "double",
    "earthbend",
    "endure",
    "exchange",
    "exert",
    "exile",
    "explore",
    "faceAVillainousChoice",
    "fateseal",
    "fight",
    "forage",
    "goad",
    "harness",
    "heal",
    "incubate",
    "investigate",
    "learn",
    "manifest",
    "manifestDread",
    "meld",
    "mill",
    "monstrosity",
    "play",
    "populate",
    "proliferate",
    "recruit",
    "regenerate",
    "reveal",
    "sacrifice",
    "scry",
    "search",
    "shuffle",
    "support",
    "surveil",
    "suspect",
    "tap",
    "theRingTemptsYou",
    "timeTravel",
    "transform",
    "triple",
    "untap",
    "ventureIntoTheDungeon",
    "vote",
    "waterbend" ]

end Semantics
