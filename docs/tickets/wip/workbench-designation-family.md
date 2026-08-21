---
needs: []
---
# Finish the designation catalog, its readers, and the keyword expansion that confers one

The designation region as one claimable unit: the catalog's eight remaining
entries plus the two reader-side gaps (the card scope's reader, the absence
check), and the mechanism that lets a keyword's expansion body confer a
designation the bare instruction is still refused. They are one round because
both halves sit on the same declarations — `Designation`, `designationScope`,
`designationChecked`, `designationGiven`, `DesignationHolder` and the designation
predicate — and the giving cell the expansion needs is the same table the catalog
rows populate.

## The catalog's rows and its missing readers

Fourteen designation rows cover thirteen of the twenty-one catalogued entries;
eight are left, and the two reader-side gaps beside them are the same family. The
catalog is open, so most of this is rows and cells, not design.

### The catalog's eight remaining entries

- **No supported card text at all — 3**: protector, planar controller,
  archenemy.
- **Reminder-only — 1**: the three sector designations appear inside space
  sculptor's reminder text on Space Beleren and nowhere else, so like Mount they
  are neither rowed nor pinnable. Do not row them; do not pin them.
- **Written through a keyword action or an ability container — 4**, and so route
  with the keyword-action work rather than through either designation reader:
  harnessed ("Harness The Mind Stone", 2 cards, [CR#701.64a]); solved (a Case's
  "Solved —" label, 13 cards); level (a Class's level bars, 68 occurrences over
  34 cards); the unlocked pair (lock and unlock instructions, 110 occurrences
  over 79 cards, [CR#709.5c]).

None of the eight needs a design decision: each is a row and a handful of cells
on whatever round benches a carrier.

### The CARD scope's reader — two entries' worth behind one scope cell

The commander has a scope cell ([CR#903.3]: the designation "is an attribute of
the card itself", which is why it survives a zone change) and NO reader, because
neither form the corpus writes is one:

- **"your commander"** — 84 supported occurrences over 81 cards, written as a
  possessed NOUN in subject position. The biggest single designation family in
  the corpus; it wants a noun row, not a predicate.
- **"is your commander"** — 3 lines, all three inside a before-the-game static
  ability that finding 188 keeps unread.

### The designation ABSENCE check — 5 lines

"There is no monarch" asks whether ANY player holds a designation rather than
describing one that does: an existential over the holder, sibling to the other
absence checks. It is the only negative the designation family writes at all —
every "isn't the monarch"-shaped negation measures ZERO, with one "isn't
saddled" line as the exception, which rides here.

## The expansion body that confers a keyword-granted designation

Monstrosity's body cannot be written because the cell that refuses a bare
"becomes monstrous" instruction also refuses the keyword's own expansion. What is
needed is the mechanism that tells a bare sentence from an expansion body, for
all five keyword-conferred designations at once.

- The monstrosity tag was measured and **refused** (finding 526): the machinery
  keys on the DESIGNATION, which has had its row for many chapters, not on the
  action's name. "Monstrosity this way" is **0** lines and "would become
  monstrous" is **0** lines, so **no `VerbName` row is earned**.
- What is needed is the ability to WRITE [CR#701.37a]'s expansion, and the probe
  named the block precisely: `GainsDesignation This Monstrous` is refused
  because `designationGiven Monstrous` is False.
- That cell is **right about bare sentences** — no card writes "this creature
  becomes monstrous" as an instruction — and **wrong about an expansion body**.
  Telling the two apart is the mechanism question this answers.
- It is the same question the other four keyword-conferred givings ask: city's
  blessing, enduring story, renowned, saddled. All four cells stay False, and
  that was confirmed by benching three cards that print the keyword line and
  check the designation without ever instructing the conferral.
- 37 monstrosity lines, all of the form "{cost}: Monstrosity N."; **Chillerpillar**
  is the card it makes whole.

## Consumption boundary

`idris/src/Experimental/Words.idr` (`Designation`, `designationScope`,
`designationChecked`, `designationGiven`, `DesignationHolder`, the noun rows,
`VerbName`), `idris/src/Experimental.idr` (the designation predicate and the
absence checks, `GainsDesignation`, the designation rows, the keyword expansion
bodies), `idris/src/Experimental/Macros.idr` (the reader spellings and the
keyword line's spelling), the pin modules `idris/src/Experimental/Proofs*.idr`,
evidence bench `idris/src/Experimental/Cards.idr`. No Rust crate.

## Acceptance

- The four keyword/container-written designations are rowed with their cells, or
  explicitly deferred to the keyword-action round; the three unattested ones and
  the reminder-only sectors stay unrowed and unpinned, with their zero recorded.
- "Your commander" reads as a possessed noun, not as a predicate.
- The 3 "is your commander" lines stay unread while finding 188 stands — this
  round does not open the before-the-game static.
- The absence check is an existential over the holder and admits the "isn't
  saddled" line; the zero on "isn't the monarch"-shaped negations survives.
- An expansion body confers the designation while the bare-instruction refusal
  still stands for all five designations — the mechanism distinguishes them
  rather than flipping `designationGiven`.
- No `VerbName` row is minted for monstrosity; the two zeros are recorded as the
  reason.
- Chillerpillar benches whole, and the three cards benched on the keyword-line-
  plus-check shape keep passing.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.
