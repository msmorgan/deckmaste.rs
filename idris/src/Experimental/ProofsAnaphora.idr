||| Forward-anaphora closure: every anaphor gate is a fold over the
||| reading-order prefix and nothing else. The law and the four-clause
||| binder contract it discharges are recorded in
||| `docs/decisions/oracle-text-is-forward-anaphoric.md`.
|||
||| Section 1 is `countBy`/`anyBy` with their split and witness lemmas;
||| section 2 the identity per grammar counter onto one of those folds;
||| section 3 the per-constructor `…ReadsOnlyPrefix` witness and
||| `…ResolvesInPrefix` proof; section 4 the ungated deictics; section 5
||| the threading functions as equations.
module Experimental.ProofsAnaphora

import Experimental
import Experimental.Macros

import Data.List.Elem

%default total

%unbound_implicits off


--------------------------------------------------------------------------------
-- 1. The two fold shapes, and what a satisfied gate means
--------------------------------------------------------------------------------

||| The shape of every counted-uniqueness gate: fold the context, keep
||| the bindings a per-binding test admits.
public export
countBy : (Binding -> Bool) -> Bindings -> Nat
countBy p [] = Z
countBy p (b :: bs) = if p b then S (countBy p bs) else countBy p bs

||| The shape of every existence gate.
public export
anyBy : (Binding -> Bool) -> Bindings -> Bool
anyBy p [] = False
anyBy p (b :: bs) = p b || anyBy p bs

||| A gate reads its context left to right. Every threading function in
||| the grammar hands a clause `delta ++ bs` — what the clause itself
||| minted, then the reading-order prefix — so a gate's value there is
||| the sum of those two and nothing else. This is the sense in which a
||| gate is a function of the prefix: later material is not in the list
||| to be counted, and there is no term outside the list.
public export
countBySplit : (p : Binding -> Bool) -> (xs, ys : Bindings) ->
               countBy p (xs ++ ys) = countBy p xs + countBy p ys
countBySplit p [] ys = Refl
countBySplit p (b :: xs) ys with (p b)
  _ | True = cong S (countBySplit p xs ys)
  _ | False = countBySplit p xs ys

||| The same for an existence gate.
public export
anyBySplit : (p : Binding -> Bool) -> (xs, ys : Bindings) ->
             anyBy p (xs ++ ys) = anyBy p xs || anyBy p ys
anyBySplit p [] ys = Refl
anyBySplit p (b :: xs) ys with (p b)
  _ | True = Refl
  _ | False = anyBySplit p xs ys

||| A satisfied count exhibits its antecedent: counting is a fold, so a
||| nonzero count hands back a binding that is an element of the context
||| and passes the test. This is "resolves to a mention in `bs`" as a
||| term, not as prose.
public export
countByWitness : (p : Binding -> Bool) -> (bs : Bindings) -> (n : Nat) ->
                 countBy p bs = S n -> (b : Binding ** (Elem b bs, So (p b)))
countByWitness _ [] _ Refl impossible
countByWitness p (b :: bs) n prf with (p b) proof eq
  countByWitness p (b :: bs) n prf | True = (b ** (Here, eqToSo eq))
  countByWitness p (b :: bs) n prf | False =
    let (c ** (el, ok)) = countByWitness p bs n prf in (c ** (There el, ok))

||| The same for an existence gate.
public export
anyByWitness : (p : Binding -> Bool) -> (bs : Bindings) ->
               So (anyBy p bs) -> (b : Binding ** (Elem b bs, So (p b)))
anyByWitness _ [] Oh impossible
anyByWitness p (b :: bs) ok with (p b) proof eq
  anyByWitness p (b :: bs) ok | True = (b ** (Here, eqToSo eq))
  anyByWitness p (b :: bs) ok | False =
    let (c ** (el, ok')) = anyByWitness p bs ok in (c ** (There el, ok'))

||| An empty prefix counts nothing, so no anaphor is writable before its
||| antecedent. Every grounding claim below is this lemma at some test.
public export
countByEmpty : (p : Binding -> Bool) -> countBy p [] = Z
countByEmpty p = Refl


--------------------------------------------------------------------------------
-- 2. Each gate is one of those folds
--------------------------------------------------------------------------------

||| What `countOnes` folds: a singular mention whose kind the read sits
||| under. The order is `kindLte` rather than equality because a target
||| may be an object and/or a player [CR#115.1], so an `Object` read
||| reaches a joined `Object \/ Player` antecedent.
public export
oneOfKind : Kind -> Binding -> Bool
oneOfKind k (MkBinding _ j OneOf _) = kindLte k j
oneOfKind k (MkBinding _ _ ManyOf _) = False

public export
countOnesIsFold : (k : Kind) -> (bs : Bindings) ->
                  countOnes k bs = countBy (oneOfKind k) bs
countOnesIsFold k [] = Refl
countOnesIsFold k (MkBinding d j OneOf p :: bs) with (kindLte k j)
  _ | True = cong S (countOnesIsFold k bs)
  _ | False = countOnesIsFold k bs
countOnesIsFold k (MkBinding d j ManyOf p :: bs) = countOnesIsFold k bs

||| What `countManys` folds: the plural twin.
public export
manyOfKind : Kind -> Binding -> Bool
manyOfKind k (MkBinding _ j ManyOf _) = kindLte k j
manyOfKind k (MkBinding _ _ OneOf _) = False

public export
countManysIsFold : (k : Kind) -> (bs : Bindings) ->
                   countManys k bs = countBy (manyOfKind k) bs
countManysIsFold k [] = Refl
countManysIsFold k (MkBinding d j ManyOf p :: bs) with (kindLte k j)
  _ | True = cong S (countManysIsFold k bs)
  _ | False = countManysIsFold k bs
countManysIsFold k (MkBinding d j OneOf p :: bs) = countManysIsFold k bs

||| What `countManysAny` folds: any group mention at all, whatever it is
||| a group of — "one or more opponents" leaves a size to read back.
public export
anyMany : Binding -> Bool
anyMany (MkBinding _ _ ManyOf _) = True
anyMany (MkBinding _ _ OneOf _) = False

public export
countManysAnyIsFold : (bs : Bindings) -> countManysAny bs = countBy anyMany bs
countManysAnyIsFold [] = Refl
countManysAnyIsFold (MkBinding d j ManyOf p :: bs) = cong S (countManysAnyIsFold bs)
countManysAnyIsFold (MkBinding d j OneOf p :: bs) = countManysAnyIsFold bs

||| What `countOutcomes` folds: a singular event-outcome mention of the
||| named sort.
public export
outcomeIs : OutcomeSort -> Binding -> Bool
outcomeIs s (MkBinding _ Outcome OneOf (OutcomeP t)) = s == t
outcomeIs s (MkBinding _ _ _ _) = False

public export
countOutcomesIsFold : (s : OutcomeSort) -> (bs : Bindings) ->
                      countOutcomes s bs = countBy (outcomeIs s) bs
countOutcomesIsFold s [] = Refl
countOutcomesIsFold s (MkBinding d Outcome OneOf (OutcomeP t) :: bs) with (s == t)
  _ | True = cong S (countOutcomesIsFold s bs)
  _ | False = countOutcomesIsFold s bs
countOutcomesIsFold s (MkBinding d Outcome ManyOf (OutcomeP t) :: bs) =
  countOutcomesIsFold s bs
countOutcomesIsFold s (MkBinding d Object p pay :: bs) = countOutcomesIsFold s bs
countOutcomesIsFold s (MkBinding d Player p pay :: bs) = countOutcomesIsFold s bs
countOutcomesIsFold s (MkBinding d (Quality q) p pay :: bs) = countOutcomesIsFold s bs
countOutcomesIsFold s (MkBinding d Gap p pay :: bs) = countOutcomesIsFold s bs
countOutcomesIsFold s (MkBinding d (LetterK l) p pay :: bs) = countOutcomesIsFold s bs
countOutcomesIsFold s (MkBinding d TurnRef p pay :: bs) = countOutcomesIsFold s bs
countOutcomesIsFold s (MkBinding d Ability p pay :: bs) = countOutcomesIsFold s bs
countOutcomesIsFold s (MkBinding d (a \/ b) p pay :: bs) = countOutcomesIsFold s bs

||| What `countQuantOutcomes` folds: an outcome mention that carries a
||| number. "That much" reads one of these and no other, since a coin
||| flip's mention carries no value [CR#705.2].
public export
quantOutcome : Binding -> Bool
quantOutcome (MkBinding _ Outcome OneOf (OutcomeP t)) = outcomeIsQuantity t
quantOutcome (MkBinding _ _ _ _) = False

public export
countQuantOutcomesIsFold : (bs : Bindings) ->
                           countQuantOutcomes bs = countBy quantOutcome bs
countQuantOutcomesIsFold [] = Refl
countQuantOutcomesIsFold (MkBinding d Outcome OneOf (OutcomeP t) :: bs)
    with (outcomeIsQuantity t)
  _ | True = cong S (countQuantOutcomesIsFold bs)
  _ | False = countQuantOutcomesIsFold bs
countQuantOutcomesIsFold (MkBinding d Outcome ManyOf (OutcomeP t) :: bs) =
  countQuantOutcomesIsFold bs
countQuantOutcomesIsFold (MkBinding d Object p pay :: bs) = countQuantOutcomesIsFold bs
countQuantOutcomesIsFold (MkBinding d Player p pay :: bs) = countQuantOutcomesIsFold bs
countQuantOutcomesIsFold (MkBinding d (Quality q) p pay :: bs) = countQuantOutcomesIsFold bs
countQuantOutcomesIsFold (MkBinding d Gap p pay :: bs) = countQuantOutcomesIsFold bs
countQuantOutcomesIsFold (MkBinding d (LetterK l) p pay :: bs) = countQuantOutcomesIsFold bs
countQuantOutcomesIsFold (MkBinding d TurnRef p pay :: bs) = countQuantOutcomesIsFold bs
countQuantOutcomesIsFold (MkBinding d Ability p pay :: bs) = countQuantOutcomesIsFold bs
countQuantOutcomesIsFold (MkBinding d (a \/ b) p pay :: bs) = countQuantOutcomesIsFold bs

||| `countQuality` is `countOnes` at a quality kind: the chosen-quality
||| gates and the wildcard pronoun read the context the same way.
public export
countQualityIsCountOnes : (q : QualitySort) -> (bs : Bindings) ->
                          countQuality q bs = countOnes (Quality q) bs
countQualityIsCountOnes q [] = Refl
countQualityIsCountOnes q (MkBinding d j OneOf p :: bs) with (kindLte (Quality q) j)
  _ | True = cong S (countQualityIsCountOnes q bs)
  _ | False = countQualityIsCountOnes q bs
countQualityIsCountOnes q (MkBinding d j ManyOf p :: bs) = countQualityIsCountOnes q bs

public export
countQualityIsFold : (q : QualitySort) -> (bs : Bindings) ->
                     countQuality q bs = countBy (oneOfKind (Quality q)) bs
countQualityIsFold q bs =
  trans (countQualityIsCountOnes q bs) (countOnesIsFold (Quality q) bs)

||| `countLetter` likewise: a letter is a singular mention of a letter
||| kind [CR#107.3].
public export
countLetterIsCountOnes : (l : Letter) -> (bs : Bindings) ->
                         countLetter l bs = countOnes (LetterK l) bs
countLetterIsCountOnes l [] = Refl
countLetterIsCountOnes l (MkBinding d j OneOf p :: bs) with (kindLte (LetterK l) j)
  _ | True = cong S (countLetterIsCountOnes l bs)
  _ | False = countLetterIsCountOnes l bs
countLetterIsCountOnes l (MkBinding d j ManyOf p :: bs) = countLetterIsCountOnes l bs

public export
countLetterIsFold : (l : Letter) -> (bs : Bindings) ->
                    countLetter l bs = countBy (oneOfKind (LetterK l)) bs
countLetterIsFold l bs =
  trans (countLetterIsCountOnes l bs) (countOnesIsFold (LetterK l) bs)

||| What `countWord` folds: a singular mention the demonstrative's noun
||| word reaches AS THE REFERENT NOW STANDS — the word filter is read
||| against the binding's current fold-state, which is still a fact about
||| the prefix.
public export
wordOne : NounWord -> Binding -> Bool
wordOne w b = isOne b.plur && wordNow w b

public export
countWordIsFold : (w : NounWord) -> (bs : Bindings) ->
                  countWord w bs = countBy (wordOne w) bs
countWordIsFold w [] = Refl
countWordIsFold w (MkBinding d j OneOf p :: bs)
    with (wordNow w (MkBinding d j OneOf p))
  _ | True = cong S (countWordIsFold w bs)
  _ | False = countWordIsFold w bs
countWordIsFold w (MkBinding d j ManyOf p :: bs) = countWordIsFold w bs

||| What `countManyWord` folds: the plural demonstrative's filter.
public export
wordMany : NounWord -> Binding -> Bool
wordMany w b = not (isOne b.plur) && wordNow w b

public export
countManyWordIsFold : (w : NounWord) -> (bs : Bindings) ->
                      countManyWord w bs = countBy (wordMany w) bs
countManyWordIsFold w [] = Refl
countManyWordIsFold w (MkBinding d j ManyOf p :: bs)
    with (wordNow w (MkBinding d j ManyOf p))
  _ | True = cong S (countManyWordIsFold w bs)
  _ | False = countManyWordIsFold w bs
countManyWordIsFold w (MkBinding d j OneOf p :: bs) = countManyWordIsFold w bs

||| The definite participle read ("the exiled card") already folds a
||| per-binding test, so its identity is the fold at that very test.
public export
countVerbedIsFold : (v : VerbName) -> (w : NounWord) -> (bs : Bindings) ->
                    countVerbed v w bs = countBy (verbedMatch v w) bs
countVerbedIsFold v w [] = Refl
countVerbedIsFold v w (b :: bs) with (verbedMatch v w b)
  _ | True = cong S (countVerbedIsFold v w bs)
  _ | False = countVerbedIsFold v w bs

public export
countManyVerbedIsFold : (v : VerbName) -> (w : NounWord) -> (bs : Bindings) ->
                        countManyVerbed v w bs = countBy (verbedMatchMany v w) bs
countManyVerbedIsFold v w [] = Refl
countManyVerbedIsFold v w (b :: bs) with (verbedMatchMany v w b)
  _ | True = cong S (countManyVerbedIsFold v w bs)
  _ | False = countManyVerbedIsFold v w bs

||| What `countGroups` folds for "the rest": an assembled group of
||| objects, with the parts already taken out of it excluded.
public export
groupOne : Binding -> Bool
groupOne (MkBinding PartD _ _ _) = False
groupOne b = objGroup b

public export
countGroupsIsFold : (bs : Bindings) -> countGroups bs = countBy groupOne bs
countGroupsIsFold [] = Refl
countGroupsIsFold (MkBinding PartD j p pay :: bs) = countGroupsIsFold bs
countGroupsIsFold (MkBinding TargetD j p pay :: bs)
    with (objGroup (MkBinding TargetD j p pay))
  _ | True = cong S (countGroupsIsFold bs)
  _ | False = countGroupsIsFold bs
countGroupsIsFold (MkBinding AD j p pay :: bs) with (objGroup (MkBinding AD j p pay))
  _ | True = cong S (countGroupsIsFold bs)
  _ | False = countGroupsIsFold bs
countGroupsIsFold (MkBinding EachD j p pay :: bs)
    with (objGroup (MkBinding EachD j p pay))
  _ | True = cong S (countGroupsIsFold bs)
  _ | False = countGroupsIsFold bs
countGroupsIsFold (MkBinding AllD j p pay :: bs) with (objGroup (MkBinding AllD j p pay))
  _ | True = cong S (countGroupsIsFold bs)
  _ | False = countGroupsIsFold bs
countGroupsIsFold (MkBinding TheD j p pay :: bs) with (objGroup (MkBinding TheD j p pay))
  _ | True = cong S (countGroupsIsFold bs)
  _ | False = countGroupsIsFold bs
countGroupsIsFold (MkBinding CountD j p pay :: bs)
    with (objGroup (MkBinding CountD j p pay))
  _ | True = cong S (countGroupsIsFold bs)
  _ | False = countGroupsIsFold bs
countGroupsIsFold (MkBinding SelfD j p pay :: bs)
    with (objGroup (MkBinding SelfD j p pay))
  _ | True = cong S (countGroupsIsFold bs)
  _ | False = countGroupsIsFold bs

||| What `countParts` folds: a part already taken from a group, which is
||| what makes "the rest" have something to be the rest OF.
public export
partOne : Binding -> Bool
partOne (MkBinding PartD j _ _) = kindLte Object j
partOne b = False

public export
countPartsIsFold : (bs : Bindings) -> countParts bs = countBy partOne bs
countPartsIsFold [] = Refl
countPartsIsFold (MkBinding PartD j p pay :: bs) with (kindLte Object j)
  _ | True = cong S (countPartsIsFold bs)
  _ | False = countPartsIsFold bs
countPartsIsFold (MkBinding TargetD j p pay :: bs) = countPartsIsFold bs
countPartsIsFold (MkBinding AD j p pay :: bs) = countPartsIsFold bs
countPartsIsFold (MkBinding EachD j p pay :: bs) = countPartsIsFold bs
countPartsIsFold (MkBinding AllD j p pay :: bs) = countPartsIsFold bs
countPartsIsFold (MkBinding TheD j p pay :: bs) = countPartsIsFold bs
countPartsIsFold (MkBinding CountD j p pay :: bs) = countPartsIsFold bs
countPartsIsFold (MkBinding SelfD j p pay :: bs) = countPartsIsFold bs

||| What `anyTargeted` folds for the "other" presupposition: an announced
||| target of a kind the modifier's own kind reaches [CR#601.2c,115.4].
public export
targetOfKind : Kind -> Binding -> Bool
targetOfKind k (MkBinding TargetD j _ _) = kindLte k j
targetOfKind k b = False

public export
anyTargetedIsAny : (k : Kind) -> (bs : Bindings) ->
                   anyTargeted k bs = anyBy (targetOfKind k) bs
anyTargetedIsAny k [] = Refl
anyTargetedIsAny k (MkBinding TargetD j p pay :: bs) with (kindLte k j)
  _ | True = Refl
  _ | False = anyTargetedIsAny k bs
anyTargetedIsAny k (MkBinding AD j p pay :: bs) = anyTargetedIsAny k bs
anyTargetedIsAny k (MkBinding EachD j p pay :: bs) = anyTargetedIsAny k bs
anyTargetedIsAny k (MkBinding AllD j p pay :: bs) = anyTargetedIsAny k bs
anyTargetedIsAny k (MkBinding TheD j p pay :: bs) = anyTargetedIsAny k bs
anyTargetedIsAny k (MkBinding PartD j p pay :: bs) = anyTargetedIsAny k bs
anyTargetedIsAny k (MkBinding CountD j p pay :: bs) = anyTargetedIsAny k bs
anyTargetedIsAny k (MkBinding SelfD j p pay :: bs) = anyTargetedIsAny k bs


--------------------------------------------------------------------------------
-- 3. Per constructor: the gate is a fact about `bs`, and it names a
--    binding IN `bs`
--------------------------------------------------------------------------------

||| Resolution for every `countOnes` gate, once.
public export
resolveOnes : (k : Kind) -> (bs : Bindings) -> countOnes k bs = 1 ->
              (b : Binding ** (Elem b bs, So (oneOfKind k b)))
resolveOnes k bs ok =
  countByWitness (oneOfKind k) bs Z (trans (sym (countOnesIsFold k bs)) ok)

||| ...and for every `countManys` gate.
public export
resolveManys : (k : Kind) -> (bs : Bindings) -> countManys k bs = 1 ->
               (b : Binding ** (Elem b bs, So (manyOfKind k b)))
resolveManys k bs ok =
  countByWitness (manyOfKind k) bs Z (trans (sym (countManysIsFold k bs)) ok)


-- "it": a wildcard singular object pronoun.

||| The pronoun asks its context one question and no other: given a
||| prefix holding exactly one singular object mention, "it" is written.
||| The gate names `bs` and nothing else, so this total function exists;
||| a constructor that read a later mention could not be given this type.
public export
itReadsOnlyPrefix : (bs : Bindings) -> countOnes Object bs = 1 -> Noun bs Object
itReadsOnlyPrefix bs ok = It {bs} {ok}

||| ...and the fact it asks for is the presence of a mention IN the
||| prefix: the gate hands back the antecedent it resolved to.
public export
itResolvesInPrefix : (bs : Bindings) -> countOnes Object bs = 1 ->
                     (b : Binding ** (Elem b bs, So (oneOfKind Object b)))
itResolvesInPrefix bs ok = resolveOnes Object bs ok


-- "they": the player pronoun.

public export
theyReadsOnlyPrefix : (bs : Bindings) -> countOnes Player bs = 1 -> Noun bs Player
theyReadsOnlyPrefix bs ok = They {bs} {ok}

public export
theyResolvesInPrefix : (bs : Bindings) -> countOnes Player bs = 1 ->
                       (b : Binding ** (Elem b bs, So (oneOfKind Player b)))
theyResolvesInPrefix bs ok = resolveOnes Player bs ok


-- "them": the plural object pronoun.

public export
themReadsOnlyPrefix : (bs : Bindings) -> countManys Object bs = 1 -> Noun bs Object
themReadsOnlyPrefix bs ok = Them {bs} {ok}

public export
themResolvesInPrefix : (bs : Bindings) -> countManys Object bs = 1 ->
                       (b : Binding ** (Elem b bs, So (manyOfKind Object b)))
themResolvesInPrefix bs ok = resolveManys Object bs ok


-- "that <noun word>": the sorted singular demonstrative.

||| The word filter is extra content on the read, not a second context:
||| the gate is still `countWord w bs`, a fact about the prefix alone.
public export
thatReadsOnlyPrefix : (bs : Bindings) -> (w : NounWord) ->
                      countWord w bs = 1 -> Noun bs (kindOfW w)
thatReadsOnlyPrefix bs w ok = That w {bs} {ok}

public export
thatResolvesInPrefix : (bs : Bindings) -> (w : NounWord) -> countWord w bs = 1 ->
                       (b : Binding ** (Elem b bs, So (wordOne w b)))
thatResolvesInPrefix bs w ok =
  countByWitness (wordOne w) bs Z (trans (sym (countWordIsFold w bs)) ok)


-- "those <noun word>": the sorted plural demonstrative.

public export
thoseReadsOnlyPrefix : (bs : Bindings) -> (w : NounWord) ->
                       countManyWord w bs = 1 -> Noun bs (kindOfW w)
thoseReadsOnlyPrefix bs w ok = Those w {bs} {ok}

public export
thoseResolvesInPrefix : (bs : Bindings) -> (w : NounWord) ->
                        countManyWord w bs = 1 ->
                        (b : Binding ** (Elem b bs, So (wordMany w b)))
thoseResolvesInPrefix bs w ok =
  countByWitness (wordMany w) bs Z (trans (sym (countManyWordIsFold w bs)) ok)


-- "the <verb>ed <noun word>": the definite participle read.

||| The participle read carries a second obligation — that the verb takes
||| the marking written on it — and that obligation mentions no context
||| at all. Separating the two arguments here is the point: only the
||| first is a context read, and it is a read of the prefix.
public export
theVerbedReadsOnlyPrefix : (bs : Bindings) -> (v : VerbName) -> (w : NounWord) ->
                           (m : VerbedMarking) -> countVerbed v w bs = 1 ->
                           VerbedMarkingOk v m -> Noun bs (kindOfW w)
theVerbedReadsOnlyPrefix bs v w m ok mk = TheVerbed v w m {bs} {ok} {mk}

public export
theVerbedResolvesInPrefix : (bs : Bindings) -> (v : VerbName) -> (w : NounWord) ->
                            countVerbed v w bs = 1 ->
                            (b : Binding ** (Elem b bs, So (verbedMatch v w b)))
theVerbedResolvesInPrefix bs v w ok =
  countByWitness (verbedMatch v w) bs Z (trans (sym (countVerbedIsFold v w bs)) ok)


-- "the <verb>ed <noun word>s": the plural participle read.

public export
thoseVerbedReadsOnlyPrefix : (bs : Bindings) -> (v : VerbName) -> (w : NounWord) ->
                             (m : VerbedMarking) -> countManyVerbed v w bs = 1 ->
                             VerbedMarkingOk v m -> Noun bs (kindOfW w)
thoseVerbedReadsOnlyPrefix bs v w m ok mk =
  ThoseVerbed v w m {bs} {ok} {mk}

public export
thoseVerbedResolvesInPrefix : (bs : Bindings) -> (v : VerbName) -> (w : NounWord) ->
                              countManyVerbed v w bs = 1 ->
                              (b : Binding ** (Elem b bs, So (verbedMatchMany v w b)))
thoseVerbedResolvesInPrefix bs v w ok =
  countByWitness (verbedMatchMany v w) bs Z
                 (trans (sym (countManyVerbedIsFold v w bs)) ok)


-- "the rest": the complement of the parts already taken from a group.

||| `So (m == n)` is `m = n` for naturals; needed to open `theRestOk`.
public export
natEqSo : (m, n : Nat) -> So (m == n) -> m = n
natEqSo Z Z Oh = Refl
natEqSo Z (S _) Oh impossible
natEqSo (S _) Z Oh impossible
natEqSo (S j) (S k) ok = cong S (natEqSo j k ok)

||| ...and a count the text says is not zero is a successor.
public export
notZeroSucc : (n : Nat) -> So (not (n == Z)) -> (k : Nat ** n = S k)
notZeroSucc Z Oh impossible
notZeroSucc (S k) ok = (k ** Refl)

||| "the rest" reads two facts about the prefix — one assembled group
||| stands in it, and at least one part has been taken from that group —
||| and neither is a fact about anything later.
public export
theRestReadsOnlyPrefix : (bs : Bindings) -> So (theRestOk bs) -> Noun bs Object
theRestReadsOnlyPrefix bs ok = TheRest {bs} {ok}

||| ...and both facts name bindings IN the prefix: the group the rest is
||| the rest OF, and a part already taken from it.
public export
theRestResolvesInPrefix : (bs : Bindings) -> So (theRestOk bs) ->
                          ((g : Binding ** (Elem g bs, So (groupOne g))),
                           (p : Binding ** (Elem p bs, So (partOne p))))
theRestResolvesInPrefix bs ok =
  let (gOk, pOk) = soAnd {a = countGroups bs == 1} ok
      gEq = natEqSo (countGroups bs) 1 gOk
      (k ** pEq) = notZeroSucc (countParts bs) pOk
   in (countByWitness groupOne bs Z (trans (sym (countGroupsIsFold bs)) gEq),
       countByWitness partOne bs k (trans (sym (countPartsIsFold bs)) pEq))


-- "that much": the quantity an event-producing clause wrote.

public export
thatMuchReadsOnlyPrefix : (bs : Bindings) -> countQuantOutcomes bs = 1 -> Amount bs
thatMuchReadsOnlyPrefix bs ok = ThatMuch {bs} {ok}

public export
thatMuchResolvesInPrefix : (bs : Bindings) -> countQuantOutcomes bs = 1 ->
                           (b : Binding ** (Elem b bs, So (quantOutcome b)))
thatMuchResolvesInPrefix bs ok =
  countByWitness quantOutcome bs Z
                 (trans (sym (countQuantOutcomesIsFold bs)) ok)


-- "that much damage prevented this way": the sorted outcome read.

public export
preventedThisWayReadsOnlyPrefix : (bs : Bindings) ->
                                  countOutcomes DamagePrevented bs = 1 -> Amount bs
preventedThisWayReadsOnlyPrefix bs ok = PreventedThisWay {bs} {ok}

public export
preventedThisWayResolvesInPrefix :
  (bs : Bindings) -> countOutcomes DamagePrevented bs = 1 ->
  (b : Binding ** (Elem b bs, So (outcomeIs DamagePrevented b)))
preventedThisWayResolvesInPrefix bs ok =
  countByWitness (outcomeIs DamagePrevented) bs Z
                 (trans (sym (countOutcomesIsFold DamagePrevented bs)) ok)


-- "the result": the number the die a clause rolled came up [CR#706.2].

public export
theResultReadsOnlyPrefix : (bs : Bindings) ->
                           countOutcomes RollResult bs = 1 -> Amount bs
theResultReadsOnlyPrefix bs ok = TheResult {bs} {ok}

public export
theResultResolvesInPrefix :
  (bs : Bindings) -> countOutcomes RollResult bs = 1 ->
  (b : Binding ** (Elem b bs, So (outcomeIs RollResult b)))
theResultResolvesInPrefix bs ok =
  countByWitness (outcomeIs RollResult) bs Z
                 (trans (sym (countOutcomesIsFold RollResult bs)) ok)


-- "that many": the size of the one group the prefix assembled.

public export
groupSizeReadsOnlyPrefix : (bs : Bindings) -> countManysAny bs = 1 -> Amount bs
groupSizeReadsOnlyPrefix bs ok = GroupSize {bs} {ok}

public export
groupSizeResolvesInPrefix : (bs : Bindings) -> countManysAny bs = 1 ->
                            (b : Binding ** (Elem b bs, So (anyMany b)))
groupSizeResolvesInPrefix bs ok =
  countByWitness anyMany bs Z (trans (sym (countManysAnyIsFold bs)) ok)


-- "the difference": the margin a comparison wrote.

public export
theDifferenceReadsOnlyPrefix : (bs : Bindings) -> countOnes Gap bs = 1 -> Amount bs
theDifferenceReadsOnlyPrefix bs ok = TheDifference {bs} {ok}

public export
theDifferenceResolvesInPrefix : (bs : Bindings) -> countOnes Gap bs = 1 ->
                                (b : Binding ** (Elem b bs, So (oneOfKind Gap b)))
theDifferenceResolvesInPrefix bs ok = resolveOnes Gap bs ok


-- "X": the letter a clause brought in by use, and "where X is", which
-- defines it [CR#107.3c].

||| What `anyOpenLetter` folds: an introduced, still-undefined letter.
public export
openLetterIsAny : (l : Letter) -> (bs : Bindings) ->
                  anyOpenLetter l bs = anyBy (openLetter l) bs
openLetterIsAny l [] = Refl
openLetterIsAny l (b :: bs) with (openLetter l b)
  _ | True = Refl
  _ | False = openLetterIsAny l bs

||| The definition asks one thing of its context: that some open X stands
||| in the prefix. Existence rather than uniqueness, because "+X/+X"
||| writes the one variable twice [CR#107.3i].
public export
defineReadsOnlyPrefix : (bs : Bindings) -> (l : Letter) -> (amt : Amount bs) ->
                        So (anyOpenLetter l bs) -> Effect bs
defineReadsOnlyPrefix bs l amt ok = Define l amt {bs} {ok}

public export
defineResolvesInPrefix : (bs : Bindings) -> (l : Letter) -> So (anyOpenLetter l bs) ->
                         (b : Binding ** (Elem b bs, So (openLetter l b)))
defineResolvesInPrefix bs l ok =
  anyByWitness (openLetter l) bs (replace {p = So} (openLetterIsAny l bs) ok)

||| After a definition no instance of the letter is open.
public export
defineClosesLetter : (l : Letter) -> (bs : Bindings) ->
                     anyOpenLetter l (defineLetter l bs) = False
defineClosesLetter l [] = Refl
defineClosesLetter l (b :: bs) with (openLetter l b) proof eq
  _ | True = defineClosesLetter l bs
  _ | False = rewrite eq in defineClosesLetter l bs

||| PIN -- "where X is ..." with no X in scope. [CR#107.3c] defines the
||| value of an X the object's text uses; a definition of nothing defines
||| nothing.
public export
badDefineWithoutUse : Not (So (anyOpenLetter X []))
badDefineWithoutUse Oh impossible

||| PIN -- a second "where X is" on one ability: the first settled every
||| instance [CR#107.3i], so none is open for the second to define.
public export
badSecondDefine : (bs : Bindings) -> (l : Letter) ->
                  Not (So (anyOpenLetter l (defineLetter l bs)))
badSecondDefine bs l ok = absurd (replace {p = So} (defineClosesLetter l bs) ok)

||| A cost's X is the same variable the text writes [CR#107.3i] and stays
||| open to a definition: [CR#107.3c] gives the text's definition the value
||| of an X written "in its cost and/or its text", so no rule refuses
||| "where X is" under an X-bearing cost and nothing here pins one.
public export
costXStaysOpen : So (anyOpenLetter X (costIntro (LoyaltySymbol {bs = []} LoyaltyDownX)))
costXStaysOpen = Oh


-- "of the chosen <quality>": the read of a choice another clause made.

public export
ofChosenReadsOnlyPrefix : (bs : Bindings) -> (q : QualitySort) ->
                          countQuality q bs = 1 -> ChosenQualityRead q ->
                          Predicate bs Object
ofChosenReadsOnlyPrefix bs q ok read = OfChosen q {bs} {ok} {read}

public export
ofChosenResolvesInPrefix : (bs : Bindings) -> (q : QualitySort) ->
                           countQuality q bs = 1 ->
                           (b : Binding ** (Elem b bs,
                                            So (oneOfKind (Quality q) b)))
ofChosenResolvesInPrefix bs q ok =
  countByWitness (oneOfKind (Quality q)) bs Z
                 (trans (sym (countQualityIsFold q bs)) ok)


-- "of the last chosen colour": the marked existence read.

||| `ChoiceStands n` is `n` being a successor, so opening it is opening a
||| `Nat`.
public export
choiceStandsSucc : (n : Nat) -> ChoiceStands n -> (k : Nat ** n = S k)
choiceStandsSucc (S k) ChoiceMade = (k ** Refl)

||| The marked read asks for EXISTENCE rather than uniqueness — bindings
||| are nearest-first, so the latest choice is what it reads — and
||| existence is still a fact about the prefix alone.
public export
ofLastChosenColorReadsOnlyPrefix : (bs : Bindings) ->
                                   ChoiceStands (countQuality Color bs) ->
                                   Predicate bs Object
ofLastChosenColorReadsOnlyPrefix bs ok = OfLastChosenColor {bs} {ok}

public export
ofLastChosenColorResolvesInPrefix :
  (bs : Bindings) -> ChoiceStands (countQuality Color bs) ->
  (b : Binding ** (Elem b bs, So (oneOfKind (Quality Color) b)))
ofLastChosenColorResolvesInPrefix bs ok =
  let (k ** eq) = choiceStandsSucc (countQuality Color bs) ok
   in countByWitness (oneOfKind (Quality Color)) bs k
                     (trans (sym (countQualityIsFold Color bs)) eq)


-- "the chosen name": the same read on the name slot.

public export
chosenNameReadsOnlyPrefix : (bs : Bindings) -> countQuality CardName bs = 1 ->
                            NameSource bs
chosenNameReadsOnlyPrefix bs ok = ChosenName {bs} {ok}

public export
chosenNameResolvesInPrefix : (bs : Bindings) -> countQuality CardName bs = 1 ->
                             (b : Binding ** (Elem b bs,
                                              So (oneOfKind (Quality CardName) b)))
chosenNameResolvesInPrefix bs ok = ofChosenResolvesInPrefix bs CardName ok


-- "of the chosen colour", on produced mana.

public export
ofChosenColorReadsOnlyPrefix : (bs : Bindings) -> (alt : Maybe ProducedRun) ->
                               AltRunWritten alt -> countQuality Color bs = 1 ->
                               ChosenQualityRead Color -> ProducedMana bs
ofChosenColorReadsOnlyPrefix bs alt ar cq rd =
  OfChosenColor alt {bs} {ar} {cq} {rd}

public export
ofChosenColorResolvesInPrefix : (bs : Bindings) -> countQuality Color bs = 1 ->
                                (b : Binding ** (Elem b bs,
                                                 So (oneOfKind (Quality Color) b)))
ofChosenColorResolvesInPrefix bs ok = ofChosenResolvesInPrefix bs Color ok


-- "other": the presupposition that an earlier target exists.

||| "Any other target" is a modifier carrying a presupposition, not a
||| slot-list side condition [CR#601.2c,115.4]: the earlier target it
||| presupposes has to be in the prefix, because that is the only place
||| the gate looks.
public export
otherReadsOnlyPrefix : (bs : Bindings) -> (k : Kind) ->
                       So (anyTargeted k bs) -> Predicate bs k
otherReadsOnlyPrefix bs k ok = Other {bs} {k} {ok}

public export
otherResolvesInPrefix : (bs : Bindings) -> (k : Kind) -> So (anyTargeted k bs) ->
                        (b : Binding ** (Elem b bs, So (targetOfKind k b)))
otherResolvesInPrefix bs k ok =
  anyByWitness (targetOfKind k) bs (replace {p = So} (anyTargetedIsAny k bs) ok)


-- "that turn": the deictic on a turn a clause already named.

public export
turnInScopeReadsOnlyPrefix : (bs : Bindings) -> countOnes TurnRef bs = 1 ->
                             TurnDeixis (Just ThatTurns) bs
turnInScopeReadsOnlyPrefix bs ok = TurnInScope {bs} {ok}

public export
turnInScopeResolvesInPrefix : (bs : Bindings) -> countOnes TurnRef bs = 1 ->
                              (b : Binding ** (Elem b bs,
                                               So (oneOfKind TurnRef b)))
turnInScopeResolvesInPrefix bs ok = resolveOnes TurnRef bs ok


-- "those tokens": the read of a characteristics definition a create
-- clause wrote [CR#111.3].

||| `countTokenSpecs` is the one gate whose definition overlaps its
||| patterns on three fields at once, so it is not restated as a
||| `countBy` fold here; what it shares with every gate above is stated
||| directly instead. The structural fact — the gate takes the context
||| and nothing else — is this function's existence.
public export
tokenAsThoseReadsOnlyPrefix : (bs : Bindings) -> countTokenSpecs bs = 1 ->
                              TokenSpec bs
tokenAsThoseReadsOnlyPrefix bs ok = TokenAsThose {bs} {ok}

||| ...and the empty prefix satisfies it for no card: an anaphor before
||| its antecedent is unwritable.
public export
noTokenAsThoseWithoutAntecedent : Not (countTokenSpecs [] = 1)
noTokenAsThoseWithoutAntecedent Refl impossible


-- "of their choice": the determiner marking on a noun some player named
-- earlier chooses [CR#608.2d].

||| `countChoosers` is the two player counts summed, so one chooser means
||| exactly one of the two folds found it.
public export
sumIsOne : (m, n : Nat) -> m + n = 1 -> Either (m = 1) (n = 1)
sumIsOne Z n prf = Right prf
sumIsOne (S Z) Z prf = Left Refl
sumIsOne (S Z) (S _) Refl impossible
sumIsOne (S (S _)) _ Refl impossible

public export
theirChoiceReadsOnlyPrefix : (bs : Bindings) -> countChoosers bs = 1 ->
                             ChoiceMode bs
theirChoiceReadsOnlyPrefix bs ok = TheirChoice {bs} {ch = ok}

||| ...and the chooser it names is a player mention IN the prefix, singular
||| or plural.
public export
theirChoiceResolvesInPrefix :
  (bs : Bindings) -> countChoosers bs = 1 ->
  Either (b : Binding ** (Elem b bs, So (oneOfKind Player b)))
         (b : Binding ** (Elem b bs, So (manyOfKind Player b)))
theirChoiceResolvesInPrefix bs ok =
  case sumIsOne (countOnes Player bs) (countManys Player bs) ok of
    Left one => Left (resolveOnes Player bs one)
    Right many => Right (resolveManys Player bs many)


--------------------------------------------------------------------------------
-- 4. The deictics, which are NOT anaphora and read no context
--------------------------------------------------------------------------------

||| The source names itself [CR#113.7]: deixis to the ability's own
||| object, not a read of any mention. Writable with an empty prefix,
||| which is exactly what separates it from the pronouns above.
public export
thisNeedsNoAntecedent : Noun [] Object
thisNeedsNoAntecedent = This

||| "You" is the ability's controller [CR#109.5] — deixis again.
public export
youNeedsNoAntecedent : Noun [] Player
youNeedsNoAntecedent = You

||| The player-group words name a set the game defines, not a mention.
public export
playerGroupNeedsNoAntecedent : (w : PlayerGroupWord) -> Noun [] Player
playerGroupNeedsNoAntecedent w = PlayerGroup w

||| "Enchanted creature" / "equipped creature" is deixis to the
||| attachment [CR#303.4m,301.5f], not to a mention: its only obligation is that
||| the attachment word and the head agree, and that obligation reads no
||| context.
public export
attachHostNeedsNoAntecedent : (w : AttachWord) -> (h : NounWord) ->
                              AttachHeadOk w h -> Noun [] (kindOfW h)
attachHostNeedsNoAntecedent w h ok = AttachHost w h {ok}

||| The letter is writable at the empty prefix -- not as deixis but as an
||| INTRODUCTION: `LetterVal` at `[]` mints the letter it names [CR#107.3].
public export
letterValIntroducesAtEmptyPrefix : (l : Letter) -> Amount []
letterValIntroducesAtEmptyPrefix l = LetterVal l


--------------------------------------------------------------------------------
-- 5. Why `bs` is the reading-order prefix: the threading audit as types
--------------------------------------------------------------------------------

||| A noun hands the next clause its own mints in front of the prefix it
||| was given. Nothing is dropped and nothing is inserted from elsewhere,
||| so the context at every later position is the accumulated mints of
||| everything textually earlier.
public export
nomIntroIsDeltaThenPrefix : (bs : Bindings) -> (k : Kind) -> (n : Noun bs k) ->
                            nomIntro n = nounDelta n ++ bs
nomIntroIsDeltaThenPrefix bs k n = Refl

||| The same for a condition, which is what `If`'s consequent is typed in.
public export
condIntroIsDeltaThenPrefix : (bs : Bindings) -> (c : Condition bs) ->
                             condIntro c = condDelta c ++ bs
condIntroIsDeltaThenPrefix bs c = Refl

||| The `otherwise` arm is typed in the phrases the then-branch announced
||| plus the quantity it wrote — both facts about a clause written BEFORE
||| the arm, so the arm still reads only backwards.
public export
otherwiseCtxIsThenBranchOnly : (bs : Bindings) -> (e : Effect bs) ->
                               otherwiseCtx e = outcomesOnly (deedDelta e) ++ annIntro e
otherwiseCtxIsThenBranchOnly bs e = Refl

||| The consequence for gates: at a threaded context, a count is this
||| clause's own contribution plus the prefix's. No third term can exist,
||| because there is no third list. This is the audit's whole content,
||| stated once at `nomIntro` — the thread every noun-taking constructor
||| in `Noun`, `Amount`, `Condition`, `Effect` and `StaticEffect` is
||| built from.
public export
gateSplitsAtNomIntro : (bs : Bindings) -> (k : Kind) -> (n : Noun bs k) ->
                       (j : Kind) ->
                       countOnes j (nomIntro n) =
                         countOnes j (nounDelta n) + countOnes j bs
gateSplitsAtNomIntro bs k n j =
  trans (countOnesIsFold j (nounDelta n ++ bs))
        (trans (countBySplit (oneOfKind j) (nounDelta n) bs)
               (cong2 (+) (sym (countOnesIsFold j (nounDelta n)))
                          (sym (countOnesIsFold j bs))))

||| ...and the same at a condition's context, which is what makes leading
||| `If` forward: the consequent's gates see the condition's mints and
||| the prefix, and nothing of the `otherwise` arm that follows.
public export
gateSplitsAtCondIntro : (bs : Bindings) -> (c : Condition bs) -> (j : Kind) ->
                        countOnes j (condIntro c) =
                          countOnes j (condDelta c) + countOnes j bs
gateSplitsAtCondIntro bs c j =
  trans (countOnesIsFold j (condDelta c ++ bs))
        (trans (countBySplit (oneOfKind j) (condDelta c) bs)
               (cong2 (+) (sym (countOnesIsFold j (condDelta c)))
                          (sym (countOnesIsFold j bs))))

||| The sequential telescope hands each member exactly its predecessors'
||| output. A member typed anywhere else would not fit here.
public export
effectsThreadPrefix : (bs : Bindings) -> (n : Nat) -> (e : Effect bs) ->
                      Effects n (effIntro e) -> Effects (S n) bs
effectsThreadPrefix bs n e es = e :: es

||| The simultaneous telescope threads the narrower `annIntro`: a
||| simultaneous member may read what its predecessor NAMED but not the
||| outcome its predecessor produced, because there is no "then".
public export
simEffectsThreadPrefix : (bs : Bindings) -> (n : Nat) -> (e : Effect bs) ->
                         SimEffects n (annIntro e) -> SimEffects (S n) bs
simEffectsThreadPrefix bs n e es = e :: es

||| The cost telescope, likewise, at `costIntro`.
public export
costSeqThreadsPrefix : (bs : Bindings) -> (n : Nat) -> (c : Cost bs) ->
                       NotCompound c -> CostSeq n (costIntro c) -> CostSeq (S n) bs
costSeqThreadsPrefix bs n c nc cs = (::) c {nc} cs

||| The static coordination telescope, at `staticIntro`.
public export
staticPartsThreadPrefix : (bs : Bindings) -> (n : Nat) -> (se : StaticEffect bs) ->
                          NotCoord se -> StaticParts n (staticIntro se) ->
                          StaticParts (S n) bs
staticPartsThreadPrefix bs n se nc rest = (::) se {nc} rest

||| An arithmetic amount reads its left operand's output, not the other
||| way round: "X plus Y" types Y in X's context.
public export
amountPlusThreadsPrefix : (bs : Bindings) -> (a : Amount bs) ->
                          Amount (amtIntro a) -> Amount bs
amountPlusThreadsPrefix bs a b = Plus a b

||| A cost is paid by a player already named: `Pay` types the cost at the
||| payer's context.
public export
payThreadsPrefix : (bs : Bindings) -> (who : Noun bs Player) ->
                   (c : Cost (nomIntro who)) -> Payable c -> PayAgrees who c ->
                   Effect bs
payThreadsPrefix bs who c pb ag = Pay who c {pb} {ag}

||| The postposed conditional: the condition is written after the clause
||| and reads what the clause announced. Forward.
public export
onlyIfThreadsPrefix : (bs : Bindings) -> (e : Effect bs) ->
                      Condition (preIntro e) -> Effect bs
onlyIfThreadsPrefix bs e c = OnlyIf e c Nothing

||| The leading conditional: the consequent is written after the
||| condition and reads what the condition introduced. Also forward —
||| the two orientations differ in WHERE mentions are introduced, not in
||| which direction they are read.
public export
ifThreadsPrefix : (bs : Bindings) -> (c : Condition bs) ->
                  Effect (condIntro c) -> Effect bs
ifThreadsPrefix bs c e = If c e Nothing

||| The postposed static conditional, the same shape at `staticIntro`.
public export
onlyWhileThreadsPrefix : (bs : Bindings) -> (se : StaticEffect bs) ->
                         (c : Condition (staticIntro se)) -> NotConditional se ->
                         MarkingOk AsLongAs c -> StaticEffect bs
onlyWhileThreadsPrefix bs se c nn mk = OnlyWhile se c AsLongAs {nn} {mk}

||| The "this way" trigger reads the enclosure's settled post-state, a
||| narrowing of what came before rather than an addition from after.
public export
thisWayThreadsPrefix : (bs : Bindings) -> (body : Effect bs) ->
                       (ev : GameEvent (effIntro body)) ->
                       Effect (thisWayCtx body ev) -> ThisWayOutcome body ->
                       Effect bs
thisWayThreadsPrefix bs body ev trig oc = ThisWay body ev trig {oc}

||| "where X is" re-marks the letter it defines in place — the shape
||| `settleTargets` has — and inserts nothing minted later.
public export
defineIntroIsRemark : (bs : Bindings) -> (l : Letter) -> (amt : Amount bs) ->
                      (ok : So (anyOpenLetter l bs)) ->
                      effIntro (Define l amt {ok}) = defineLetter l (amtIntro amt)
defineIntroIsRemark bs l amt ok = Refl

||| One variable written twice. "This creature gets -X/-X, where X is your
||| life total" mints the letter ONCE: the toughness shift is typed at the
||| power shift's output, so its `LetterVal` finds the letter already in the
||| prefix and reads it [CR#107.3i]. Every twin-`Amount` slot is a telescope
||| for this reason.
public export
twinShiftMintsOneLetter :
  countLetter X (staticIntro (Gets {bs = []} Macros.thisCreature
                                   (PtDown (LetterVal X)) (PtDown (LetterVal X)))) = 1
twinShiftMintsOneLetter = Refl

||| The letter's own contribution is a fold over the prefix: the
||| introduction exactly when the prefix counts no such letter.
public export
letterValDeltaIsPrefixFold : (bs : Bindings) -> (l : Letter) ->
                             amtDelta (LetterVal l {bs}) = letterDelta l bs
letterValDeltaIsPrefixFold bs l = Refl
