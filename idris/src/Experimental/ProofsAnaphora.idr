||| Forward-anaphora closure: every anaphor gate is a fold over the
module Experimental.ProofsAnaphora

import Experimental
import Experimental.Macros

import Data.List.Elem

%default total

%unbound_implicits off



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



||| What `countOnes` folds: a singular mention whose kind the read sits
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
                          countChoice (QSort q) bs = countOnes (Quality q) bs
countQualityIsCountOnes q [] = Refl
countQualityIsCountOnes q (MkBinding d j OneOf p :: bs) with (kindLte (Quality q) j)
  _ | True = cong S (countQualityIsCountOnes q bs)
  _ | False = countQualityIsCountOnes q bs
countQualityIsCountOnes q (MkBinding d j ManyOf p :: bs) = countQualityIsCountOnes q bs

public export
countQualityIsFold : (q : QualitySort) -> (bs : Bindings) ->
                     countChoice (QSort q) bs = countBy (oneOfKind (Quality q)) bs
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

||| Every indexed pronoun read is the fold at its structural reach test.
public export
countReachIsFold : (r : Reach) -> (pl : Plurality) -> (bs : Bindings) ->
                   countReach r pl bs = countBy (reaches r pl) bs
countReachIsFold r pl [] = Refl
countReachIsFold r pl (b :: bs) with (reaches r pl b)
  _ | True = cong S (countReachIsFold r pl bs)
  _ | False = countReachIsFold r pl bs

public export
countWordIsFold : (w : NounWord) -> (bs : Bindings) ->
                  countReach (Word w) OneOf bs = countBy (reaches (Word w) OneOf) bs
countWordIsFold w bs = countReachIsFold (Word w) OneOf bs

public export
countManyWordIsFold : (w : NounWord) -> (bs : Bindings) ->
                      countReach (Word w) ManyOf bs = countBy (reaches (Word w) ManyOf) bs
countManyWordIsFold w bs = countReachIsFold (Word w) ManyOf bs

public export
countOnesAtIsFold : (sl : SlotCarrier) -> (bs : Bindings) ->
                    countReach (AtSlot sl) OneOf bs =
                      countBy (reaches (AtSlot sl) OneOf) bs
countOnesAtIsFold sl bs = countReachIsFold (AtSlot sl) OneOf bs

public export
countVerbedItIsFold : (v : VerbLabel) -> (bs : Bindings) ->
                      countReach (Stamped v) OneOf bs =
                        countBy (reaches (Stamped v) OneOf) bs
countVerbedItIsFold v bs = countReachIsFold (Stamped v) OneOf bs

public export
countVerbedThemIsFold : (v : VerbLabel) -> (bs : Bindings) ->
                        countReach (Stamped v) ManyOf bs =
                          countBy (reaches (Stamped v) ManyOf) bs
countVerbedThemIsFold v bs = countReachIsFold (Stamped v) ManyOf bs

public export
countItTokenIsFold : (bs : Bindings) ->
                     countReach TokenBorn OneOf bs = countBy (reaches TokenBorn OneOf) bs
countItTokenIsFold bs = countReachIsFold TokenBorn OneOf bs

public export
countUnionHalfIsFold : (w : NounWord) -> (bs : Bindings) ->
                       countReach (UnionHalf w) OneOf bs =
                         countBy (reaches (UnionHalf w) OneOf) bs
countUnionHalfIsFold w bs = countReachIsFold (UnionHalf w) OneOf bs

public export
countVerbedIsFold : (v : VerbLabel) -> (w : NounWord) -> (bs : Bindings) ->
                    countReach (Verbed v w Attributive) OneOf bs =
                      countBy (reaches (Verbed v w Attributive) OneOf) bs
countVerbedIsFold v w bs = countReachIsFold (Verbed v w Attributive) OneOf bs

public export
countManyVerbedIsFold : (v : VerbLabel) -> (w : NounWord) -> (bs : Bindings) ->
                        countReach (Verbed v w Attributive) ManyOf bs =
                          countBy (reaches (Verbed v w Attributive) ManyOf) bs
countManyVerbedIsFold v w bs =
  countReachIsFold (Verbed v w Attributive) ManyOf bs

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



||| Resolution for every `countOnes` gate, once.
public export
resolveOnes : (k : Kind) -> (bs : Bindings) -> countOnes k bs = 1 ->
              (b : Binding ** (Elem b bs, So (oneOfKind k b)))
resolveOnes k bs ok =
  countByWitness (oneOfKind k) bs Z (trans (sym (countOnesIsFold k bs)) ok)

public export
resolveManys : (k : Kind) -> (bs : Bindings) -> countManys k bs = 1 ->
               (b : Binding ** (Elem b bs, So (manyOfKind k b)))
resolveManys k bs ok =
  countByWitness (manyOfKind k) bs Z (trans (sym (countManysIsFold k bs)) ok)



||| The pronoun asks its context one question and no other: given a
public export
itReadsOnlyPrefix : (bs : Bindings) -> countReach Bare OneOf bs = 1 -> Noun bs Object
itReadsOnlyPrefix bs ok = Pro Bare OneOf {bs} {ok}

public export
itResolvesInPrefix : (bs : Bindings) -> countReach Bare OneOf bs = 1 ->
                     (b : Binding ** (Elem b bs, So (reaches Bare OneOf b)))
itResolvesInPrefix bs ok =
  countByWitness (reaches Bare OneOf) bs Z
    (trans (sym (countReachIsFold Bare OneOf bs)) ok)



||| The scoped pronoun asks its context ONE question too, and the same
public export
itAtReadsOnlyPrefix : (sl : SlotCarrier) -> (bs : Bindings) ->
                      countReach (AtSlot sl) OneOf bs = 1 -> Noun bs Object
itAtReadsOnlyPrefix sl bs ok = Pro (AtSlot sl) OneOf {bs} {ok}

public export
itAtResolvesInPrefix : (sl : SlotCarrier) -> (bs : Bindings) ->
                       countReach (AtSlot sl) OneOf bs = 1 ->
                       (b : Binding ** (Elem b bs, So (reaches (AtSlot sl) OneOf b)))
itAtResolvesInPrefix sl bs ok =
  countByWitness (reaches (AtSlot sl) OneOf) bs Z
    (trans (sym (countReachIsFold (AtSlot sl) OneOf bs)) ok)



||| The verb-scoped pronoun asks the prefix ONE question -- how many of
public export
itVerbedReadsOnlyPrefix : (bs : Bindings) -> (v : VerbLabel) ->
                          KnownVerb v -> countReach (Stamped v) OneOf bs = 1 ->
                          Noun bs Object
itVerbedReadsOnlyPrefix bs v kn ok = Pro (Stamped v) OneOf {bs} {ok}

public export
itVerbedResolvesInPrefix : (bs : Bindings) -> (v : VerbLabel) ->
                           countReach (Stamped v) OneOf bs = 1 ->
                           (b : Binding ** (Elem b bs, So (reaches (Stamped v) OneOf b)))
itVerbedResolvesInPrefix bs v ok =
  countByWitness (reaches (Stamped v) OneOf) bs Z
    (trans (sym (countReachIsFold (Stamped v) OneOf bs)) ok)



||| The origin-scoped pronoun asks the prefix ONE question -- how many of
public export
itTokenReadsOnlyPrefix : (bs : Bindings) -> countReach TokenBorn OneOf bs = 1 ->
                         Noun bs Object
itTokenReadsOnlyPrefix bs ok = Pro TokenBorn OneOf {bs} {ok}

public export
itTokenResolvesInPrefix : (bs : Bindings) -> countReach TokenBorn OneOf bs = 1 ->
                          (b : Binding ** (Elem b bs, So (reaches TokenBorn OneOf b)))
itTokenResolvesInPrefix bs ok =
  countByWitness (reaches TokenBorn OneOf) bs Z
    (trans (sym (countReachIsFold TokenBorn OneOf bs)) ok)



||| A binding one segment holds is a binding the whole prefix holds.
public export
elemInSuffix : {0 b : Binding} -> {0 rest : Bindings} -> (co : Bindings) ->
               Elem b rest -> Elem b (co ++ rest)
elemInSuffix [] el = el
elemInSuffix (c :: cs) el = There (elemInSuffix cs el)

public export
elemInPrefix : {0 b : Binding} -> {0 made : Bindings} -> (before : Bindings) ->
               Elem b made -> Elem b (made ++ before)
elemInPrefix bef Here = Here
elemInPrefix bef (There el) = There (elemInPrefix bef el)

||| A segment read counts NO MORE than the whole prefix does, in either
public export
countBySegmentNoLarger : (p : Binding -> Bool) -> (xs, ys : Bindings) ->
                         (LTE (countBy p xs) (countBy p (xs ++ ys)),
                          LTE (countBy p ys) (countBy p (xs ++ ys)))
countBySegmentNoLarger p xs ys =
  rewrite countBySplit p xs ys in
    (lteAddRight (countBy p xs), lteRightPlus (countBy p xs) (countBy p ys))
  where
    lteRightPlus : (n, m : Nat) -> LTE m (n + m)
    lteRightPlus Z m = reflexive
    lteRightPlus (S n) m = lteSuccRight (lteRightPlus n m)

||| The co-argument-scoped pronoun asks the prefix ONE question -- how
public export
itOtherThanReadsOnlyPrefix : (co, rest : Bindings) ->
                             countOnes Object rest = 1 -> Noun (co ++ rest) Object
itOtherThanReadsOnlyPrefix co rest ok = ItOtherThan co rest {sp = Refl} {ok}

public export
itOtherThanResolvesInPrefix : (co, rest : Bindings) ->
                              countOnes Object rest = 1 ->
                              (b : Binding ** (Elem b (co ++ rest),
                                               So (oneOfKind Object b)))
itOtherThanResolvesInPrefix co rest ok =
  let (b ** (el, k)) = resolveOnes Object rest ok in
      (b ** (elemInSuffix co el, k))

||| The previous-sibling read is the same shape at the other end of the
public export
itPriorReadsOnlyPrefix : (made, before : Bindings) ->
                         countOnes Object made = 1 -> Noun (made ++ before) Object
itPriorReadsOnlyPrefix made before ok = ItPrior made before {sp = Refl} {ok}

public export
itPriorResolvesInPrefix : (made, before : Bindings) ->
                          countOnes Object made = 1 ->
                          (b : Binding ** (Elem b (made ++ before),
                                           So (oneOfKind Object b)))
itPriorResolvesInPrefix made before ok =
  let (b ** (el, k)) = resolveOnes Object made ok in
      (b ** (elemInPrefix before el, k))



public export
theyReadsOnlyPrefix : (bs : Bindings) -> countReach (Word PlayerW) OneOf bs = 1 ->
                      Noun bs Player
theyReadsOnlyPrefix bs ok = Pro (Word PlayerW) OneOf {bs} {ok}

public export
theyResolvesInPrefix : (bs : Bindings) -> countReach (Word PlayerW) OneOf bs = 1 ->
                       (b : Binding ** (Elem b bs,
                                                So (reaches (Word PlayerW) OneOf b)))
theyResolvesInPrefix bs ok =
  countByWitness (reaches (Word PlayerW) OneOf) bs Z
    (trans (sym (countReachIsFold (Word PlayerW) OneOf bs)) ok)



public export
themReadsOnlyPrefix : (bs : Bindings) -> countReach Bare ManyOf bs = 1 -> Noun bs Object
themReadsOnlyPrefix bs ok = Pro Bare ManyOf {bs} {ok}

public export
themResolvesInPrefix : (bs : Bindings) -> countReach Bare ManyOf bs = 1 ->
                       (b : Binding ** (Elem b bs, So (reaches Bare ManyOf b)))
themResolvesInPrefix bs ok =
  countByWitness (reaches Bare ManyOf) bs Z
    (trans (sym (countReachIsFold Bare ManyOf bs)) ok)



||| The verb-scoped GROUP pronoun asks the prefix the singular row's
public export
themVerbedReadsOnlyPrefix : (bs : Bindings) -> (v : VerbLabel) ->
                            KnownVerb v -> countReach (Stamped v) ManyOf bs = 1 ->
                            Noun bs Object
themVerbedReadsOnlyPrefix bs v kn ok = Pro (Stamped v) ManyOf {bs} {ok}

public export
themVerbedResolvesInPrefix : (bs : Bindings) -> (v : VerbLabel) ->
                             countReach (Stamped v) ManyOf bs = 1 ->
                             (b : Binding ** (Elem b bs,
                                                      So (reaches (Stamped v) ManyOf b)))
themVerbedResolvesInPrefix bs v ok =
  countByWitness (reaches (Stamped v) ManyOf) bs Z
    (trans (sym (countReachIsFold (Stamped v) ManyOf bs)) ok)



||| The word filter is extra content on the read, not a second context:
||| the gate is still one `countReach`, a fact about the prefix alone.
public export
thatReadsOnlyPrefix : (bs : Bindings) -> (w : NounWord) ->
                      countReach (Word w) OneOf bs = 1 -> Noun bs (kindOfW w)
thatReadsOnlyPrefix bs w ok = Pro (Word w) OneOf {bs} {ok}

public export
thatResolvesInPrefix : (bs : Bindings) -> (w : NounWord) ->
                       countReach (Word w) OneOf bs = 1 ->
                       (b : Binding ** (Elem b bs, So (reaches (Word w) OneOf b)))
thatResolvesInPrefix bs w ok =
  countByWitness (reaches (Word w) OneOf) bs Z
    (trans (sym (countReachIsFold (Word w) OneOf bs)) ok)



||| The split arm's gate counts the UNION mentions its word names a half
public export
thatHalfReadsOnlyPrefix : (bs : Bindings) -> (w : NounWord) ->
                          countReach (UnionHalf w) OneOf bs = 1 ->
                          Noun bs (kindOfW w)
thatHalfReadsOnlyPrefix bs w ok = Pro (UnionHalf w) OneOf {bs} {ok}

public export
thatHalfResolvesInPrefix : (bs : Bindings) -> (w : NounWord) ->
                           countReach (UnionHalf w) OneOf bs = 1 ->
                           (b : Binding ** (Elem b bs,
                                                    So (reaches (UnionHalf w) OneOf b)))
thatHalfResolvesInPrefix bs w ok =
  countByWitness (reaches (UnionHalf w) OneOf) bs Z
    (trans (sym (countReachIsFold (UnionHalf w) OneOf bs)) ok)



public export
thoseReadsOnlyPrefix : (bs : Bindings) -> (w : NounWord) ->
                       countReach (Word w) ManyOf bs = 1 -> Noun bs (kindOfW w)
thoseReadsOnlyPrefix bs w ok = Pro (Word w) ManyOf {bs} {ok}

public export
thoseResolvesInPrefix : (bs : Bindings) -> (w : NounWord) ->
                        countReach (Word w) ManyOf bs = 1 ->
                        (b : Binding ** (Elem b bs, So (reaches (Word w) ManyOf b)))
thoseResolvesInPrefix bs w ok =
  countByWitness (reaches (Word w) ManyOf) bs Z
    (trans (sym (countReachIsFold (Word w) ManyOf bs)) ok)



||| The participle read carries a second obligation — that the verb takes
public export
theVerbedReadsOnlyPrefix : (bs : Bindings) -> (v : VerbLabel) -> (w : NounWord) ->
                           (m : VerbedMarking) ->
                           countReach (Verbed v w m) OneOf bs = 1 ->
                           VerbedMarkingOk v m -> Noun bs (kindOfW w)
theVerbedReadsOnlyPrefix bs v w m ok mk = Pro (Verbed v w m) OneOf {bs} {ok}

public export
theVerbedResolvesInPrefix : (bs : Bindings) -> (v : VerbLabel) -> (w : NounWord) ->
                            (m : VerbedMarking) ->
                            countReach (Verbed v w m) OneOf bs = 1 ->
                            (b : Binding ** (Elem b bs,
                                                     So (reaches (Verbed v w m) OneOf b)))
theVerbedResolvesInPrefix bs v w m ok =
  countByWitness (reaches (Verbed v w m) OneOf) bs Z
    (trans (sym (countReachIsFold (Verbed v w m) OneOf bs)) ok)



public export
thoseVerbedReadsOnlyPrefix : (bs : Bindings) -> (v : VerbLabel) -> (w : NounWord) ->
                             (m : VerbedMarking) ->
                             countReach (Verbed v w m) ManyOf bs = 1 ->
                             VerbedMarkingOk v m -> Noun bs (kindOfW w)
thoseVerbedReadsOnlyPrefix bs v w m ok mk =
  Pro (Verbed v w m) ManyOf {bs} {ok}

public export
thoseVerbedResolvesInPrefix : (bs : Bindings) -> (v : VerbLabel) -> (w : NounWord) ->
                              (m : VerbedMarking) ->
                              countReach (Verbed v w m) ManyOf bs = 1 ->
                              (b : Binding ** (Elem b bs,
                                                       So (reaches (Verbed v w m) ManyOf b)))
thoseVerbedResolvesInPrefix bs v w m ok =
  countByWitness (reaches (Verbed v w m) ManyOf) bs Z
    (trans (sym (countReachIsFold (Verbed v w m) ManyOf bs)) ok)



public export
notZeroSucc : (n : Nat) -> So (not (n == Z)) -> (k : Nat ** n = S k)
notZeroSucc Z Oh impossible
notZeroSucc (S k) ok = (k ** Refl)

||| "the rest"
public export
theRestReadsOnlyPrefix : (bs : Bindings) -> So (theRestOk bs) -> Noun bs Object
theRestReadsOnlyPrefix bs ok = TheRest {bs} {ok}

public export
theRestResolvesInPrefix : (bs : Bindings) -> So (theRestOk bs) ->
                          (p : Binding ** (Elem p bs, So (partOne p)))
theRestResolvesInPrefix bs ok =
  let (_, pOk) = soAnd {a = countGroups bs <= 1} ok
      (k ** pEq) = notZeroSucc (countParts bs) pOk
   in countByWitness partOne bs k (trans (sym (countPartsIsFold bs)) pEq)

public export
theRestGroupResolvesInPrefix : (bs : Bindings) -> countGroups bs = 1 ->
                               (g : Binding ** (Elem g bs, So (groupOne g)))
theRestGroupResolvesInPrefix bs gEq =
  countByWitness groupOne bs Z (trans (sym (countGroupsIsFold bs)) gEq)



public export
thatMuchReadsOnlyPrefix : (bs : Bindings) -> countQuantOutcomes bs = 1 -> Amount bs
thatMuchReadsOnlyPrefix bs ok = ThatMuch {bs} {ok}

public export
thatMuchResolvesInPrefix : (bs : Bindings) -> countQuantOutcomes bs = 1 ->
                           (b : Binding ** (Elem b bs, So (quantOutcome b)))
thatMuchResolvesInPrefix bs ok =
  countByWitness quantOutcome bs Z
                 (trans (sym (countQuantOutcomesIsFold bs)) ok)



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



public export
groupSizeReadsOnlyPrefix : (bs : Bindings) -> countManysAny bs = 1 -> Amount bs
groupSizeReadsOnlyPrefix bs ok = GroupSize {bs} {ok}

public export
groupSizeResolvesInPrefix : (bs : Bindings) -> countManysAny bs = 1 ->
                            (b : Binding ** (Elem b bs, So (anyMany b)))
groupSizeResolvesInPrefix bs ok =
  countByWitness anyMany bs Z (trans (sym (countManysAnyIsFold bs)) ok)



public export
theDifferenceReadsOnlyPrefix : (bs : Bindings) -> countOnes Gap bs = 1 -> Amount bs
theDifferenceReadsOnlyPrefix bs ok = TheDifference {bs} {ok}

public export
theDifferenceResolvesInPrefix : (bs : Bindings) -> countOnes Gap bs = 1 ->
                                (b : Binding ** (Elem b bs, So (oneOfKind Gap b)))
theDifferenceResolvesInPrefix bs ok = resolveOnes Gap bs ok



||| What `anyOpenLetter` folds: an introduced, still-undefined letter.
public export
openLetterIsAny : (l : Letter) -> (bs : Bindings) ->
                  anyOpenLetter l bs = anyBy (openLetter l) bs
openLetterIsAny l [] = Refl
openLetterIsAny l (b :: bs) with (openLetter l b)
  _ | True = Refl
  _ | False = openLetterIsAny l bs

||| The definition asks one thing of its context: that some open X stands
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

||| "where X is ..."
public export
badDefineWithoutUse : Not (So (anyOpenLetter X []))
badDefineWithoutUse Oh impossible

||| a second "where X is" on one ability: the first settled every
||| instance [CR#107.3i], so none is open for the second to define.
public export
badSecondDefine : (bs : Bindings) -> (l : Letter) ->
                  Not (So (anyOpenLetter l (defineLetter l bs)))
badSecondDefine bs l ok = absurd (replace {p = So} (defineClosesLetter l bs) ok)

||| A cost's X is the same variable the text writes [CR#107.3i] and stays
public export
costXStaysOpen : So (anyOpenLetter X (costIntro (LoyaltySymbol {bs = []} LoyaltyDownX)))
costXStaysOpen = Oh



public export
ofChosenReadsOnlyPrefix : (bs : Bindings) -> (q : QualitySort) ->
                          countChoice (QSort q) bs = 1 -> ChosenQualityRead q ->
                          Predicate bs Object
ofChosenReadsOnlyPrefix bs q ok read = OfChosen q {bs} {ok} {read}

public export
ofChosenResolvesInPrefix : (bs : Bindings) -> (q : QualitySort) ->
                           countChoice (QSort q) bs = 1 ->
                           (b : Binding ** (Elem b bs,
                                            So (oneOfKind (Quality q) b)))
ofChosenResolvesInPrefix bs q ok =
  countByWitness (oneOfKind (Quality q)) bs Z
                 (trans (sym (countQualityIsFold q bs)) ok)



||| `ChoiceStands n` is `n` being a successor, so opening it is opening a
||| `Nat`.
public export
choiceStandsSucc : (n : Nat) -> ChoiceStands n -> (k : Nat ** n = S k)
choiceStandsSucc (S k) ChoiceMade = (k ** Refl)

||| The marked read asks for EXISTENCE rather than uniqueness — bindings
public export
ofLastChosenColorReadsOnlyPrefix : (bs : Bindings) ->
                                   ChoiceStands (countChoice (QSort Color) bs) ->
                                   Predicate bs Object
ofLastChosenColorReadsOnlyPrefix bs ok = OfLastChosen Color {bs} {ok}

public export
ofLastChosenColorResolvesInPrefix :
  (bs : Bindings) -> ChoiceStands (countChoice (QSort Color) bs) ->
  (b : Binding ** (Elem b bs, So (oneOfKind (Quality Color) b)))
ofLastChosenColorResolvesInPrefix bs ok =
  let (k ** eq) = choiceStandsSucc (countChoice (QSort Color) bs) ok
   in countByWitness (oneOfKind (Quality Color)) bs k
                     (trans (sym (countQualityIsFold Color bs)) eq)



public export
chosenNameReadsOnlyPrefix : (bs : Bindings) -> countChoice (QSort CardName) bs = 1 ->
                            NameSource bs
chosenNameReadsOnlyPrefix bs ok = ChosenName {bs} {ok}

public export
chosenNameResolvesInPrefix : (bs : Bindings) -> countChoice (QSort CardName) bs = 1 ->
                             (b : Binding ** (Elem b bs,
                                              So (oneOfKind (Quality CardName) b)))
chosenNameResolvesInPrefix bs ok = ofChosenResolvesInPrefix bs CardName ok



public export
ofChosenColorReadsOnlyPrefix : (bs : Bindings) -> (alt : Maybe ProducedRun) ->
                               AltRunWritten alt -> countChoice (QSort Color) bs = 1 ->
                               ChosenQualityRead Color -> ProducedMana bs
ofChosenColorReadsOnlyPrefix bs alt ar cq rd =
  OfChosenColor alt {bs} {ar} {cq} {rd}

public export
ofChosenColorResolvesInPrefix : (bs : Bindings) -> countChoice (QSort Color) bs = 1 ->
                                (b : Binding ** (Elem b bs,
                                                 So (oneOfKind (Quality Color) b)))
ofChosenColorResolvesInPrefix bs ok = ofChosenResolvesInPrefix bs Color ok



||| "Any other target"
public export
otherReadsOnlyPrefix : (bs : Bindings) -> (k : Kind) ->
                       So (anyTargeted k bs) -> Predicate bs k
otherReadsOnlyPrefix bs k ok = Other {bs} {k} {ok}

public export
otherResolvesInPrefix : (bs : Bindings) -> (k : Kind) -> So (anyTargeted k bs) ->
                        (b : Binding ** (Elem b bs, So (targetOfKind k b)))
otherResolvesInPrefix bs k ok =
  anyByWitness (targetOfKind k) bs (replace {p = So} (anyTargetedIsAny k bs) ok)



public export
turnInScopeReadsOnlyPrefix : (bs : Bindings) -> countOnes TurnRef bs = 1 ->
                             TurnDeixis (Just ThatTurns) bs
turnInScopeReadsOnlyPrefix bs ok = TurnInScope {bs} {ok}

public export
turnInScopeResolvesInPrefix : (bs : Bindings) -> countOnes TurnRef bs = 1 ->
                              (b : Binding ** (Elem b bs,
                                               So (oneOfKind TurnRef b)))
turnInScopeResolvesInPrefix bs ok = resolveOnes TurnRef bs ok



||| `countTokenSpecs` is the one gate whose definition overlaps its
public export
tokenAsThoseReadsOnlyPrefix : (bs : Bindings) -> countTokenSpecs bs = 1 ->
                              TokenSpec bs
tokenAsThoseReadsOnlyPrefix bs ok = TokenAsThose {bs} {ok}

public export
noTokenAsThoseWithoutAntecedent : Not (countTokenSpecs [] = 1)
noTokenAsThoseWithoutAntecedent Refl impossible

||| The gate counts a DEFINITION, so plurality is not one of the fields it
public export
oneTokenIsOneSpec :
  countTokenSpecs [MkBinding AD Object OneOf
                             (ObjectP (Just Creature) (Just Battlefield)
                                      Nothing (Just TokenOrigin) Nothing)] = 1
oneTokenIsOneSpec = Refl

public export
manyTokensAreOneSpec :
  countTokenSpecs [MkBinding AD Object ManyOf
                             (ObjectP (Just Creature) (Just Battlefield)
                                      Nothing (Just TokenOrigin) Nothing)] = 1
manyTokensAreOneSpec = Refl

||| A non-token object leaves no definition whatever its plurality, which
||| is the fact `badAnaphoricTokenAfterNonToken` (ProofsE) spells as a card.
public export
oneNonTokenIsNoSpec :
  countTokenSpecs [MkBinding TheD Object OneOf
                             (ObjectP (Just Creature) (Just Battlefield)
                                      Nothing Nothing Nothing)] = 0
oneNonTokenIsNoSpec = Refl



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

public export
theirChoiceResolvesInPrefix :
  (bs : Bindings) -> countChoosers bs = 1 ->
  Either (b : Binding ** (Elem b bs, So (oneOfKind Player b)))
         (b : Binding ** (Elem b bs, So (manyOfKind Player b)))
theirChoiceResolvesInPrefix bs ok =
  case sumIsOne (countOnes Player bs) (countManys Player bs) ok of
    Left one => Left (resolveOnes Player bs one)
    Right many => Right (resolveManys Player bs many)



||| The source names itself [CR#113.7]: deixis to the ability's own
public export
thisNeedsNoAntecedent : Noun [] Object
thisNeedsNoAntecedent = This

||| "You"
public export
youNeedsNoAntecedent : Noun [] Player
youNeedsNoAntecedent = You

||| The player-group words name a set the game defines, not a mention.
public export
playerGroupNeedsNoAntecedent : (w : PlayerGroupWord) -> Noun [] Player
playerGroupNeedsNoAntecedent w = PlayerGroup w

||| "Enchanted creature"
public export
attachHostNeedsNoAntecedent : (w : AttachWord) -> (h : NounWord) ->
                              AttachHeadOk w h -> Noun [] (kindOfW h)
attachHostNeedsNoAntecedent w h ok = AttachHost w h {ok}

||| The letter is writable at the empty prefix -- not as deixis but as an
||| INTRODUCTION: `LetterVal` at `[]` mints the letter it names [CR#107.3].
public export
letterValIntroducesAtEmptyPrefix : (l : Letter) -> Amount []
letterValIntroducesAtEmptyPrefix l = LetterVal l



||| "[n]'s controller sacrifices it"
public export
controllerSacrificesReadsNoPrefix : (bs : Bindings) -> (n : Noun bs Object) ->
                                    nounPlur n = OneOf ->
                                    OnBattlefield (nounZone n) -> Effect bs
controllerSacrificesReadsNoPrefix bs n one zn = ControllerSacrifices n {one} {zn}

||| "[src] deals damage equal to its [c] to [to]"
public export
dealDamageOwnReadsNoPrefix : (bs : Bindings) -> (k : Kind) ->
                             (src : Noun bs Object) -> (c : Characteristic) ->
                             (to : Noun (nomIntro src) k) ->
                             PerMember to -> DamageRecipient to -> Effect bs
dealDamageOwnReadsNoPrefix bs k src c to pm rk = DealDamageOwn src c to {pm} {rk}

||| "[n] [vp1] and [vp2]"
public export
ofSubjectReadsNoPrefix : (bs : Bindings) -> (k : Nat) -> (n : Noun bs Object) ->
                         (vps : SubjectVPs k (selfSubjIntro n)) -> IsSucc k ->
                         So (vpsOk (nounZone n) (nounRegime n) (nounHeadTys n) vps) ->
                         StaticEffect bs
ofSubjectReadsNoPrefix bs k n vps ne ok = OfSubject n vps {ne} {ok}



||| A noun hands the next clause its own mints in front of the prefix it
public export
nomIntroIsDeltaThenPrefix : (bs : Bindings) -> (k : Kind) -> (n : Noun bs k) ->
                            nomIntro n = nounDelta n ++ bs
nomIntroIsDeltaThenPrefix bs k n = Refl

||| A type-naming test writes a card type and nothing a counted gate
public export
markTyKeepsOnes : (j : Kind) -> (ty : Maybe CardType) -> (b : Binding) ->
                  oneOfKind j (markTy ty b) = oneOfKind j b
markTyKeepsOnes j ty (MkBinding det Object OneOf (ObjectP Nothing zn st og _)) = Refl
markTyKeepsOnes j ty (MkBinding det Object ManyOf (ObjectP Nothing zn st og _)) = Refl
markTyKeepsOnes j ty (MkBinding det Object plur (ObjectP (Just t) zn st og _)) = Refl
markTyKeepsOnes j ty (MkBinding det Object plur (PileP zn sz fc)) = Refl
markTyKeepsOnes j ty (MkBinding det Player plur PlayerP) = Refl
markTyKeepsOnes j ty (MkBinding det Player plur ChosenPlayerP) = Refl
markTyKeepsOnes j ty (MkBinding det (Quality q) plur QualityP) = Refl
markTyKeepsOnes j ty (MkBinding det Outcome plur (OutcomeP s)) = Refl
markTyKeepsOnes j ty (MkBinding det Gap plur GapP) = Refl
markTyKeepsOnes j ty (MkBinding det (LetterK l) plur LetterP) = Refl
markTyKeepsOnes j ty (MkBinding det TurnRef plur TurnRefP) = Refl
markTyKeepsOnes j ty (MkBinding det Ability plur (AbilityP og)) = Refl
markTyKeepsOnes j ty (MkBinding det (a \/ b) plur (JoinP l r)) = Refl

public export
markTyKeepsAt : (sl : SlotCarrier) -> (ty : Maybe CardType) -> (b : Binding) ->
                reaches (AtSlot sl) OneOf (markTy ty b) = reaches (AtSlot sl) OneOf b
markTyKeepsAt sl ty (MkBinding det Object plur (ObjectP Nothing zn st og _)) = Refl
markTyKeepsAt sl ty (MkBinding det Object plur (ObjectP (Just t) zn st og _)) = Refl
markTyKeepsAt sl ty (MkBinding det Object plur (PileP zn sz fc)) = Refl
markTyKeepsAt sl ty (MkBinding det Player plur PlayerP) = Refl
markTyKeepsAt sl ty (MkBinding det Player plur ChosenPlayerP) = Refl
markTyKeepsAt sl ty (MkBinding det (Quality q) plur QualityP) = Refl
markTyKeepsAt sl ty (MkBinding det Outcome plur (OutcomeP s)) = Refl
markTyKeepsAt sl ty (MkBinding det Gap plur GapP) = Refl
markTyKeepsAt sl ty (MkBinding det (LetterK l) plur LetterP) = Refl
markTyKeepsAt sl ty (MkBinding det TurnRef plur TurnRefP) = Refl
markTyKeepsAt sl ty (MkBinding det Ability plur (AbilityP og)) = Refl
markTyKeepsAt sl ty (MkBinding det (a \/ b) plur (JoinP l r)) = Refl

||| The re-mark drops nothing and inserts nothing: it is the list it was
public export
markFirstKeepsLength : (q : Binding -> Bool) -> (ty : Maybe CardType) ->
                       (bs : Bindings) -> length (markFirst q ty bs) = length bs
markFirstKeepsLength q ty [] = Refl
markFirstKeepsLength q ty (b :: bs) with (q b)
  _ | True = Refl
  _ | False = cong S (markFirstKeepsLength q ty bs)

||| What one binding contributes to a fold: the whole of its
||| contribution, and a function of its test's answer alone.
public export
keptBy : Bool -> Nat -> Nat
keptBy True n = S n
keptBy False n = n

public export
countByCons : (r : Binding -> Bool) -> (x : Binding) -> (bs : Bindings) ->
              countBy r (x :: bs) = keptBy (r x) (countBy r bs)
countByCons r x bs with (r x)
  _ | True = Refl
  _ | False = Refl

public export
countByHeadCong : (r : Binding -> Bool) -> (x, y : Binding) -> r x = r y ->
                  (bs : Bindings) -> countBy r (x :: bs) = countBy r (y :: bs)
countByHeadCong r x y prf bs =
  trans (countByCons r x bs)
        (trans (cong (\v => keptBy v (countBy r bs)) prf)
               (sym (countByCons r y bs)))

public export
markFirstKeeps : (r : Binding -> Bool) -> (q : Binding -> Bool) ->
                 (ty : Maybe CardType) ->
                 ((b : Binding) -> r (markTy ty b) = r b) ->
                 (bs : Bindings) -> countBy r (markFirst q ty bs) = countBy r bs
markFirstKeeps r q ty pres [] = Refl
markFirstKeeps r q ty pres (b :: bs) with (q b)
  _ | True = countByHeadCong r (markTy ty b) b (pres b) bs
  _ | False with (r b)
    _ | True = cong S (markFirstKeeps r q ty pres bs)
    _ | False = markFirstKeeps r q ty pres bs

public export
condRemarkKeepsOnes : (bs : Bindings) -> (c : Condition bs) -> (j : Kind) ->
                      countOnes j (condRemark c) = countOnes j bs
condRemarkKeepsOnes bs c j with (condRemarkAt c)
  _ | Nothing = Refl
  _ | Just (q, ty) =
    trans (countOnesIsFold j (markFirst q ty bs))
          (trans (markFirstKeeps (oneOfKind j) q ty (markTyKeepsOnes j ty) bs)
                 (sym (countOnesIsFold j bs)))

public export
condRemarkKeepsAt : (bs : Bindings) -> (c : Condition bs) -> (sl : SlotCarrier) ->
                    countReach (AtSlot sl) OneOf (condRemark c) =
                      countReach (AtSlot sl) OneOf bs
condRemarkKeepsAt bs c sl with (condRemarkAt c)
  _ | Nothing = Refl
  _ | Just (q, ty) =
    trans (countReachIsFold (AtSlot sl) OneOf (markFirst q ty bs))
          (trans (markFirstKeeps (reaches (AtSlot sl) OneOf) q ty
                                 (markTyKeepsAt sl ty) bs)
                 (sym (countReachIsFold (AtSlot sl) OneOf bs)))

||| The same for a condition, which is what `If`'s consequent is typed
public export
condIntroIsDeltaThenRemark : (bs : Bindings) -> (c : Condition bs) ->
                             condIntro c = condDelta c ++ condRemark c
condIntroIsDeltaThenRemark bs c = Refl

||| The `otherwise` arm is typed in the phrases the then-branch announced
public export
otherwiseCtxIsThenBranchOnly : (bs : Bindings) -> (e : Effect bs) ->
                               otherwiseCtx e = outcomesOnly (deedDelta e) ++ annIntro e
otherwiseCtxIsThenBranchOnly bs e = Refl

||| The consequence for gates: at a threaded context, a count is this
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

public export
gateSplitsAtCondIntro : (bs : Bindings) -> (c : Condition bs) -> (j : Kind) ->
                        countOnes j (condIntro c) =
                          countOnes j (condDelta c) + countOnes j bs
gateSplitsAtCondIntro bs c j =
  trans (countOnesIsFold j (condDelta c ++ condRemark c))
        (trans (countBySplit (oneOfKind j) (condDelta c) (condRemark c))
               (cong2 (+) (sym (countOnesIsFold j (condDelta c)))
                          (trans (sym (countOnesIsFold j (condRemark c)))
                                 (condRemarkKeepsOnes bs c j))))

||| The SCOPED gate splits the same way. Narrowing which mentions are
public export
slotGateSplitsAtNomIntro : (bs : Bindings) -> (k : Kind) -> (n : Noun bs k) ->
                           (sl : SlotCarrier) ->
                           countReach (AtSlot sl) OneOf (nomIntro n) =
                             countReach (AtSlot sl) OneOf (nounDelta n) +
                             countReach (AtSlot sl) OneOf bs
slotGateSplitsAtNomIntro bs k n sl =
  trans (countReachIsFold (AtSlot sl) OneOf (nounDelta n ++ bs))
        (trans (countBySplit (reaches (AtSlot sl) OneOf) (nounDelta n) bs)
               (cong2 (+) (sym (countReachIsFold (AtSlot sl) OneOf (nounDelta n)))
                          (sym (countReachIsFold (AtSlot sl) OneOf bs))))

public export
slotGateSplitsAtCondIntro : (bs : Bindings) -> (c : Condition bs) ->
                            (sl : SlotCarrier) ->
                            countReach (AtSlot sl) OneOf (condIntro c) =
                              countReach (AtSlot sl) OneOf (condDelta c) +
                              countReach (AtSlot sl) OneOf bs
slotGateSplitsAtCondIntro bs c sl =
  trans (countReachIsFold (AtSlot sl) OneOf (condDelta c ++ condRemark c))
        (trans (countBySplit (reaches (AtSlot sl) OneOf)
                             (condDelta c) (condRemark c))
               (cong2 (+) (sym (countReachIsFold (AtSlot sl) OneOf (condDelta c)))
                          (trans (sym (countReachIsFold (AtSlot sl) OneOf (condRemark c)))
                                 (condRemarkKeepsAt bs c sl))))

||| The split arm's gate, likewise.
public export
unionHalfGateSplitsAtNomIntro : (bs : Bindings) -> (k : Kind) -> (n : Noun bs k) ->
                                (w : NounWord) ->
                                countReach (UnionHalf w) OneOf (nomIntro n) =
                                  countReach (UnionHalf w) OneOf (nounDelta n) +
                                  countReach (UnionHalf w) OneOf bs
unionHalfGateSplitsAtNomIntro bs k n w =
  trans (countReachIsFold (UnionHalf w) OneOf (nounDelta n ++ bs))
        (trans (countBySplit (reaches (UnionHalf w) OneOf) (nounDelta n) bs)
               (cong2 (+) (sym (countReachIsFold (UnionHalf w) OneOf (nounDelta n)))
                          (sym (countReachIsFold (UnionHalf w) OneOf bs))))

||| The sequential telescope hands each member exactly its predecessors'
||| output. A member typed anywhere else would not fit here.
public export
effectsThreadPrefix : (bs : Bindings) -> (n : Nat) -> (e : Effect bs) ->
                      Effects n (effIntro e) -> Effects (S n) bs
effectsThreadPrefix bs n e es = e :: es

||| The simultaneous telescope threads the narrower `annIntro`: a
public export
simEffectsThreadPrefix : (bs : Bindings) -> (n : Nat) -> (e : Effect bs) ->
                         SimEffects n (annIntro e) -> SimEffects (S n) bs
simEffectsThreadPrefix bs n e es = e :: es

||| The cost telescope, likewise, at `costIntro`.
public export
costSeqThreadsPrefix : (bs : Bindings) -> (n : Nat) -> (c : Cost bs) ->
                       CostSeq n (costIntro c) -> CostSeq (S n) bs
costSeqThreadsPrefix bs n c cs = (::) c cs

||| The static coordination telescope, at `staticIntro`.
public export
staticPartsThreadPrefix : (bs : Bindings) -> (n : Nat) -> (se : StaticEffect bs) ->
                          NotCoord se -> StaticParts n (staticIntro se) ->
                          StaticParts (S n) bs
staticPartsThreadPrefix bs n se nc rest = (::) se {nc} rest

||| The SHARED-SUBJECT coordination telescope, at `vpIntro`. Same shape
public export
subjectVPsThreadPrefix : (bs : Bindings) -> (n : Nat) -> (vp : SubjectVP bs) ->
                         SubjectVPs n (vpIntro vp) -> SubjectVPs (S n) bs
subjectVPsThreadPrefix bs n vp rest = (::) vp rest

public export
vpIntroIsDeltaThenPrefix : (bs : Bindings) -> (pow : PtShift bs) ->
                           (tou : PtShift (shiftIntro pow)) ->
                           (sp : Maybe (Duration (shiftIntro tou))) ->
                           vpIntro (VPGets pow tou sp)
                             = shiftDelta tou ++ shiftDelta pow ++ bs
vpIntroIsDeltaThenPrefix bs pow tou sp = Refl

public export
vpGainsMintsNothing : (bs : Bindings) -> (ab : AbilityAt bs) ->
                      (sp : Maybe (Duration bs)) ->
                      vpIntro (VPGains ab sp) = bs
vpGainsMintsNothing bs ab sp = Refl

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
payThreadsPrefix bs who c pb ag = Pay who c PaidOnce {pb} {ag}

||| The postposed conditional: the condition is written after the clause
||| and reads what the clause announced. Forward.
public export
onlyIfThreadsPrefix : (bs : Bindings) -> (e : Effect bs) ->
                      Condition (preIntro e) -> Effect bs
onlyIfThreadsPrefix bs e c = OnlyIf e c Nothing

||| The leading conditional: the consequent is written after the
public export
ifThreadsPrefix : (bs : Bindings) -> (c : Condition bs) ->
                  Effect (condIntro c) -> Effect bs
ifThreadsPrefix bs c e = If c e Nothing

||| The postposed static conditional, the same shape at `staticIntro`.
public export
onlyWhileThreadsPrefix : (bs : Bindings) -> (se : StaticEffect bs) ->
                         (c : Condition (staticIntro se)) ->
                         MarkingOk AsLongAs c -> StaticEffect bs
onlyWhileThreadsPrefix bs se c mk = OnlyWhile se c AsLongAs {mk}

||| The "this way" trigger reads the enclosure's settled post-state, a
||| narrowing of what came before rather than an addition from after.
public export
thisWayThreadsPrefix : (bs : Bindings) -> (body : Effect bs) ->
                       (ev : GameEvent (effIntro body)) ->
                       Effect (thisWayCtx body ev) -> ThisWayOutcome body ->
                       Effect bs
thisWayThreadsPrefix bs body ev trig oc = ThisWay body ev trig {oc}

||| "where X is"
public export
defineIntroIsRemark : (bs : Bindings) -> (l : Letter) -> (amt : Amount bs) ->
                      (ok : So (anyOpenLetter l bs)) ->
                      effIntro (Define l amt {ok}) = defineLetter l (amtIntro amt)
defineIntroIsRemark bs l amt ok = Refl

||| One variable written twice. "This creature gets -X/-X, where X is your
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
