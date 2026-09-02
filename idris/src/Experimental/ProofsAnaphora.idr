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

||| What `countOnesAt` folds: `countOnes Object`'s own test narrowed to
||| the candidates one verb slot's carrier admits. The narrowing is a
||| per-binding test like every other, so the scoped read is a `countBy`
||| fold and inherits the split and witness lemmas unchanged -- which is
||| the whole reason the sort-scoped read is a count and not a
||| preference.
public export
countOnesAtIsFold : (sl : SlotCarrier) -> (bs : Bindings) ->
                    countOnesAt sl bs = countBy (itAtReaches sl) bs
countOnesAtIsFold sl [] = Refl
countOnesAtIsFold sl (b :: bs) with (itAtReaches sl b)
  _ | True = cong S (countOnesAtIsFold sl bs)
  _ | False = countOnesAtIsFold sl bs

||| What `countVerbedIt` folds: `countOnes Object`'s own test narrowed
||| the OTHER way -- to the mentions one label stamped rather than to
||| the ones one verb slot's carrier admits. Same shape, same reason: the
||| narrowing is a per-binding test, so the verb-scoped read is a
||| `countBy` fold and inherits the split and witness lemmas unchanged.
||| Two narrowings of one gate, neither of them a preference.
public export
countVerbedItIsFold : (v : VerbLabel) -> (bs : Bindings) ->
                      countVerbedIt v bs = countBy (itVerbedReaches v) bs
countVerbedItIsFold v [] = Refl
countVerbedItIsFold v (b :: bs) with (itVerbedReaches v b)
  _ | True = cong S (countVerbedItIsFold v bs)
  _ | False = countVerbedItIsFold v bs

||| ...and the plural twin, at the plural reach test.
public export
countVerbedThemIsFold : (v : VerbLabel) -> (bs : Bindings) ->
                        countVerbedThem v bs = countBy (themVerbedReaches v) bs
countVerbedThemIsFold v [] = Refl
countVerbedThemIsFold v (b :: bs) with (themVerbedReaches v b)
  _ | True = cong S (countVerbedThemIsFold v bs)
  _ | False = countVerbedThemIsFold v bs

||| What `countItToken` folds: `countOnes Object`'s own test narrowed a
||| THIRD way -- to the mentions a create clause made [CR#111.1], read
||| off the origin that clause already wrote onto its own mention. Same
||| shape as the carrier and the label narrowings: a per-binding test, so
||| the origin-scoped read is a `countBy` fold and inherits the split and
||| witness lemmas unchanged. Three narrowings of one gate, none of them
||| a preference.
public export
countItTokenIsFold : (bs : Bindings) ->
                     countItToken bs = countBy itTokenReaches bs
countItTokenIsFold [] = Refl
countItTokenIsFold (b :: bs) with (itTokenReaches b)
  _ | True = cong S (countItTokenIsFold bs)
  _ | False = countItTokenIsFold bs

||| What `countUnionHalf` folds: a singular UNION mention one half of
||| which the split arm's word names.
public export
unionHalf : NounWord -> Binding -> Bool
unionHalf w b = isOne b.plur && joinedPayload b.payload && halfReaches w b.payload

public export
countUnionHalfIsFold : (w : NounWord) -> (bs : Bindings) ->
                       countUnionHalf w bs = countBy (unionHalf w) bs
countUnionHalfIsFold w [] = Refl
countUnionHalfIsFold w (b :: bs) with (unionHalf w b)
  _ | True = cong S (countUnionHalfIsFold w bs)
  _ | False = countUnionHalfIsFold w bs

||| The definite participle read ("the exiled card") already folds a
||| per-binding test, so its identity is the fold at that very test.
public export
countVerbedIsFold : (v : VerbLabel) -> (w : NounWord) -> (bs : Bindings) ->
                    countVerbed v w bs = countBy (verbedMatch v w) bs
countVerbedIsFold v w [] = Refl
countVerbedIsFold v w (b :: bs) with (verbedMatch v w b)
  _ | True = cong S (countVerbedIsFold v w bs)
  _ | False = countVerbedIsFold v w bs

public export
countManyVerbedIsFold : (v : VerbLabel) -> (w : NounWord) -> (bs : Bindings) ->
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


-- "it" again, read at the carrier its consuming verb's rule admits.

||| The scoped pronoun asks its context ONE question too, and the same
||| kind of question: how many of the prefix's mentions the slot's
||| carrier admits. The slot is a word on the constructor, not a second
||| context, so the gate still names `bs` and nothing else.
public export
itAtReadsOnlyPrefix : (sl : SlotCarrier) -> (bs : Bindings) ->
                      countOnesAt sl bs = 1 -> Noun bs Object
itAtReadsOnlyPrefix sl bs ok = ItAt sl {bs} {ok}

||| ...and it resolves to a mention IN the prefix, by the same witness
||| lemma the unscoped read uses. Narrowing the candidates changed which
||| binding comes back, never where it comes from.
public export
itAtResolvesInPrefix : (sl : SlotCarrier) -> (bs : Bindings) ->
                       countOnesAt sl bs = 1 ->
                       (b : Binding ** (Elem b bs, So (itAtReaches sl b)))
itAtResolvesInPrefix sl bs ok =
  countByWitness (itAtReaches sl) bs Z (trans (sym (countOnesAtIsFold sl bs)) ok)


-- "it" again, read at the label that stamped its referent.

||| The verb-scoped pronoun asks the prefix ONE question -- how many of
||| its mentions that label stamped -- and carries a second obligation
||| that mentions no context at all, that the label is a real one. The
||| split is `TheVerbed`'s: only the first argument is a context read,
||| and it is a read of the prefix. What the two do not share is the
||| SPELLING obligation, which the pronoun does not incur.
public export
itVerbedReadsOnlyPrefix : (bs : Bindings) -> (v : VerbLabel) ->
                          KnownVerb v -> countVerbedIt v bs = 1 -> Noun bs Object
itVerbedReadsOnlyPrefix bs v kn ok = ItVerbed v {bs} {kn} {ok}

||| ...and it resolves to a mention IN the prefix, by the same witness
||| lemma both the unscoped and the carrier-scoped reads use. A stamp is
||| a mark on a binding the prefix already held, so narrowing on it
||| changed which binding comes back and never where it comes from.
public export
itVerbedResolvesInPrefix : (bs : Bindings) -> (v : VerbLabel) ->
                           countVerbedIt v bs = 1 ->
                           (b : Binding ** (Elem b bs, So (itVerbedReaches v b)))
itVerbedResolvesInPrefix bs v ok =
  countByWitness (itVerbedReaches v) bs Z (trans (sym (countVerbedItIsFold v bs)) ok)


-- "it" again, read at the clause that MADE its referent.

||| The origin-scoped pronoun asks the prefix ONE question -- how many of
||| its mentions a create clause made -- and carries no second obligation
||| at all: unlike the label-scoped read it names no vocabulary entry,
||| because a create clause names no keyword action. So the whole of its
||| gate is a read of `bs`.
public export
itTokenReadsOnlyPrefix : (bs : Bindings) -> countItToken bs = 1 -> Noun bs Object
itTokenReadsOnlyPrefix bs ok = ItToken {bs} {ok}

||| ...and it resolves to a mention IN the prefix, by the same witness
||| lemma the unscoped, the carrier-scoped and the label-scoped reads
||| use. An origin is a mark the create clause wrote on the binding it
||| minted, so narrowing on it changed which binding comes back and never
||| where it comes from.
public export
itTokenResolvesInPrefix : (bs : Bindings) -> countItToken bs = 1 ->
                          (b : Binding ** (Elem b bs, So (itTokenReaches b)))
itTokenResolvesInPrefix bs ok =
  countByWitness itTokenReaches bs Z (trans (sym (countItTokenIsFold bs)) ok)


-- "it" again, read over ONE NAMED SEGMENT of the prefix.
--
-- The three narrowings above ask a per-binding question of the whole
-- prefix. The two below ask `It`'s OWN question of a segment the
-- consuming construction names: the co-argument's delta is subtracted
-- (`ItOtherThan`) or the preceding clause's delta is all that is looked
-- at (`ItPrior`). The segment is written as the split `bs = xs ++ ys`,
-- so a segment read is still a `countBy` fold over material that is IN
-- `bs` -- the three lemmas here are what make that literal.

||| A binding one segment holds is a binding the whole prefix holds.
||| `Elem` is the term form of "in `bs`", so these two lemmas are what
||| carry a segment read's witness back out to the context the binder
||| contract's clause 3 names.
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
||| direction of the split. This is the sense in which every narrowing
||| in this family is a count over a provably smaller set: `countBySplit`
||| turns the whole count into the two segments' sum, and a summand never
||| exceeds its sum.
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
||| many singular objects the segment its CO-ARGUMENT did not mint holds
||| -- and carries a second obligation that names no candidate at all,
||| that the two segments are the prefix. The split is `TheVerbed`'s
||| again: only the first is a context read, and it is a read of `bs`.
public export
itOtherThanReadsOnlyPrefix : (co, rest : Bindings) ->
                             countOnes Object rest = 1 -> Noun (co ++ rest) Object
itOtherThanReadsOnlyPrefix co rest ok = ItOtherThan co rest {sp = Refl} {ok}

||| ...and it resolves to a mention IN the prefix. The witness comes back
||| out of the segment by `elemInSuffix`, so narrowing to a segment
||| changed which binding comes back and never where it comes from --
||| the same sentence the three per-binding narrowings above earn.
public export
itOtherThanResolvesInPrefix : (co, rest : Bindings) ->
                              countOnes Object rest = 1 ->
                              (b : Binding ** (Elem b (co ++ rest),
                                               So (oneOfKind Object b)))
itOtherThanResolvesInPrefix co rest ok =
  let (b ** (el, k)) = resolveOnes Object rest ok in
      (b ** (elemInSuffix co el, k))

||| The previous-sibling read is the same shape at the other end of the
||| split: the segment counted is the one the preceding clause MADE, and
||| the question asked of it is `It`'s.
public export
itPriorReadsOnlyPrefix : (made, before : Bindings) ->
                         countOnes Object made = 1 -> Noun (made ++ before) Object
itPriorReadsOnlyPrefix made before ok = ItPrior made before {sp = Refl} {ok}

||| ...and it too resolves to a mention IN the prefix, by `elemInPrefix`.
public export
itPriorResolvesInPrefix : (made, before : Bindings) ->
                          countOnes Object made = 1 ->
                          (b : Binding ** (Elem b (made ++ before),
                                           So (oneOfKind Object b)))
itPriorResolvesInPrefix made before ok =
  let (b ** (el, k)) = resolveOnes Object made ok in
      (b ** (elemInPrefix before el, k))


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


-- "them" again, read at the label that stamped its referents.

||| The verb-scoped GROUP pronoun asks the prefix the singular row's
||| question at the plural reach test -- how many of its group mentions
||| that label stamped -- and carries the same context-free second
||| obligation, that the label is a real one.
public export
themVerbedReadsOnlyPrefix : (bs : Bindings) -> (v : VerbLabel) ->
                            KnownVerb v -> countVerbedThem v bs = 1 ->
                            Noun bs Object
themVerbedReadsOnlyPrefix bs v kn ok = ThemVerbed v {bs} {kn} {ok}

||| ...and it resolves to a mention IN the prefix, by the witness lemma
||| every one of these reads uses.
public export
themVerbedResolvesInPrefix : (bs : Bindings) -> (v : VerbLabel) ->
                             countVerbedThem v bs = 1 ->
                             (b : Binding ** (Elem b bs, So (themVerbedReaches v b)))
themVerbedResolvesInPrefix bs v ok =
  countByWitness (themVerbedReaches v) bs Z
                 (trans (sym (countVerbedThemIsFold v bs)) ok)


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


-- "that <noun word>" as ONE ARM of a union's split read.

||| The split arm's gate counts the UNION mentions its word names a half
||| of. That is a smaller fact than the demonstrative's, not a different
||| kind of fact: still a fold over the prefix, still counted uniqueness.
public export
thatHalfReadsOnlyPrefix : (bs : Bindings) -> (w : NounWord) ->
                          countUnionHalf w bs = 1 -> Noun bs (kindOfW w)
thatHalfReadsOnlyPrefix bs w ok = ThatHalf w {bs} {ok}

public export
thatHalfResolvesInPrefix : (bs : Bindings) -> (w : NounWord) ->
                           countUnionHalf w bs = 1 ->
                           (b : Binding ** (Elem b bs, So (unionHalf w b)))
thatHalfResolvesInPrefix bs w ok =
  countByWitness (unionHalf w) bs Z (trans (sym (countUnionHalfIsFold w bs)) ok)


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
theVerbedReadsOnlyPrefix : (bs : Bindings) -> (v : VerbLabel) -> (w : NounWord) ->
                           (m : VerbedMarking) -> countVerbed v w bs = 1 ->
                           VerbedMarkingOk v m -> Noun bs (kindOfW w)
theVerbedReadsOnlyPrefix bs v w m ok mk = TheVerbed v w m {bs} {ok} {mk}

public export
theVerbedResolvesInPrefix : (bs : Bindings) -> (v : VerbLabel) -> (w : NounWord) ->
                            countVerbed v w bs = 1 ->
                            (b : Binding ** (Elem b bs, So (verbedMatch v w b)))
theVerbedResolvesInPrefix bs v w ok =
  countByWitness (verbedMatch v w) bs Z (trans (sym (countVerbedIsFold v w bs)) ok)


-- "the <verb>ed <noun word>s": the plural participle read.

public export
thoseVerbedReadsOnlyPrefix : (bs : Bindings) -> (v : VerbLabel) -> (w : NounWord) ->
                             (m : VerbedMarking) -> countManyVerbed v w bs = 1 ->
                             VerbedMarkingOk v m -> Noun bs (kindOfW w)
thoseVerbedReadsOnlyPrefix bs v w m ok mk =
  ThoseVerbed v w m {bs} {ok} {mk}

public export
thoseVerbedResolvesInPrefix : (bs : Bindings) -> (v : VerbLabel) -> (w : NounWord) ->
                              countManyVerbed v w bs = 1 ->
                              (b : Binding ** (Elem b bs, So (verbedMatchMany v w b)))
thoseVerbedResolvesInPrefix bs v w ok =
  countByWitness (verbedMatchMany v w) bs Z
                 (trans (sym (countManyVerbedIsFold v w bs)) ok)


-- "the rest": the complement of the parts already taken from a group.

||| ...and a count the text says is not zero is a successor.
public export
notZeroSucc : (n : Nat) -> So (not (n == Z)) -> (k : Nat ** n = S k)
notZeroSucc Z Oh impossible
notZeroSucc (S k) ok = (k ** Refl)

||| "the rest" reads two facts about the prefix — no more than one
||| assembled group stands in it, and at least one part has been taken
||| out of what the phrase is the rest OF — and neither is a fact about
||| anything later.
public export
theRestReadsOnlyPrefix : (bs : Bindings) -> So (theRestOk bs) -> Noun bs Object
theRestReadsOnlyPrefix bs ok = TheRest {bs} {ok}

||| ...and the part it presupposes names a binding IN the prefix. The
||| part alone: a choice's set is the description's extension, which no
||| binding stands for [CR#608.2d], so a text may license the phrase with
||| no group of its own.
public export
theRestResolvesInPrefix : (bs : Bindings) -> So (theRestOk bs) ->
                          (p : Binding ** (Elem p bs, So (partOne p)))
theRestResolvesInPrefix bs ok =
  let (_, pOk) = soAnd {a = countGroups bs <= 1} ok
      (k ** pEq) = notZeroSucc (countParts bs) pOk
   in countByWitness partOne bs k (trans (sym (countPartsIsFold bs)) pEq)

||| ...and where the text DID assemble the group, that group is a binding
||| in the prefix too.
public export
theRestGroupResolvesInPrefix : (bs : Bindings) -> countGroups bs = 1 ->
                               (g : Binding ** (Elem g bs, So (groupOne g)))
theRestGroupResolvesInPrefix bs gEq =
  countByWitness groupOne bs Z (trans (sym (countGroupsIsFold bs)) gEq)


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


-- "the chosen name": the same read on the name slot.

public export
chosenNameReadsOnlyPrefix : (bs : Bindings) -> countChoice (QSort CardName) bs = 1 ->
                            NameSource bs
chosenNameReadsOnlyPrefix bs ok = ChosenName {bs} {ok}

public export
chosenNameResolvesInPrefix : (bs : Bindings) -> countChoice (QSort CardName) bs = 1 ->
                             (b : Binding ** (Elem b bs,
                                              So (oneOfKind (Quality CardName) b)))
chosenNameResolvesInPrefix bs ok = ofChosenResolvesInPrefix bs CardName ok


-- "of the chosen colour", on produced mana.

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
||| patterns on two fields at once, so it is not restated as a
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

||| The gate counts a DEFINITION, so plurality is not one of the fields it
||| reads: a create clause that made one token satisfies it exactly as a
||| clause that made a batch does [CR#111.3]. Both halves are witnessed,
||| because the singular half is what this gate used to refuse.
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


-- The de-pronominalization templates, which are not reads either. Where
-- a deictic reads no context because it names the source, these read
-- none because the referent the printed pronoun spells is the
-- CONSTRUCTION'S OWN EARLIER ARGUMENT -- the forward binder contract's
-- clause 4 rather than its clause 3. Each is witnessed as writable
-- whatever else the prefix holds, which is the operational difference
-- between a template and an anaphor: an anaphor's gate can fail on a
-- crowded prefix, and none of these has a gate to fail.

||| "[n]'s controller sacrifices it" [CR#701.21a]: one noun slot, and no
||| count anywhere. Its two obligations are facts about that noun alone.
public export
controllerSacrificesReadsNoPrefix : (bs : Bindings) -> (n : Noun bs Object) ->
                                    nounPlur n = OneOf ->
                                    OnBattlefield (nounZone n) -> Effect bs
controllerSacrificesReadsNoPrefix bs n one zn = ControllerSacrifices n {one} {zn}

||| "[src] deals damage equal to its [c] to [to]" [CR#120.1]: the
||| possessor is the source slot, so the characteristic slot is a word
||| and not a phrase, and nothing is counted.
public export
dealDamageOwnReadsNoPrefix : (bs : Bindings) -> (k : Kind) ->
                             (src : Noun bs Object) -> (c : Characteristic) ->
                             (to : Noun (nomIntro src) k) ->
                             PerMember to -> DamageRecipient to -> Effect bs
dealDamageOwnReadsNoPrefix bs k src c to pm rk = DealDamageOwn src c to {pm} {rk}

||| "[n] [vp1] and [vp2]": the shared subject is written once, and the
||| parts hold no subject slot to read it back with. The obligation is a
||| fold over the parts asking about that one subject.
public export
ofSubjectReadsNoPrefix : (bs : Bindings) -> (k : Nat) -> (n : Noun bs Object) ->
                         (vps : SubjectVPs k (selfSubjIntro n)) -> IsSucc k ->
                         So (vpsOk (nounZone n) (nounRegime n) (nounHeadTys n) vps) ->
                         StaticEffect bs
ofSubjectReadsNoPrefix bs k n vps ne ok = OfSubject n vps {ne} {ok}


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

||| A type-naming test writes a card type and nothing a counted gate
||| reads: determiner, kind and plurality are the fields `oneOfKind`
||| asks about, and the re-mark touches none of them.
public export
markTyKeepsOnes : (j : Kind) -> (ty : Maybe CardType) -> (b : Binding) ->
                  oneOfKind j (markTy ty b) = oneOfKind j b
markTyKeepsOnes j ty (MkBinding det Object OneOf (ObjectP Nothing zn st og _)) = Refl
markTyKeepsOnes j ty (MkBinding det Object ManyOf (ObjectP Nothing zn st og _)) = Refl
markTyKeepsOnes j ty (MkBinding det Object plur (ObjectP (Just t) zn st og _)) = Refl
-- `markTy` writes a type onto an untyped OBJECT mention and a pile has
-- none to write [CR#700.3b], so the re-mark is the identity here.
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

||| ...and the SCOPED gate likewise: a slot's carrier is read off the
||| zone [CR#109.2], which the re-mark leaves where it was.
public export
markTyKeepsAt : (sl : SlotCarrier) -> (ty : Maybe CardType) -> (b : Binding) ->
                itAtReaches sl (markTy ty b) = itAtReaches sl b
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
||| given, one binding of it rewritten where it stood. This is the
||| licensed in-place form, stated as the property rather than assumed
||| from the shape of the definition.
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

||| ...so swapping a head binding for one the gate answers the same way
||| leaves the fold where it was.
public export
countByHeadCong : (r : Binding -> Bool) -> (x, y : Binding) -> r x = r y ->
                  (bs : Bindings) -> countBy r (x :: bs) = countBy r (y :: bs)
countByHeadCong r x y prf bs =
  trans (countByCons r x bs)
        (trans (cong (\v => keptBy v (countBy r bs)) prf)
               (sym (countByCons r y bs)))

||| ...and any gate that does not read what the re-mark writes counts the
||| re-marked prefix exactly as it counts the prefix.
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

||| ...so a condition's re-mark leaves every counted gate reading the
||| prefix it always read. This is what buys the re-mark its place in the
||| binder contract: the knowledge a test leaves behind changes what a
||| later read FINDS on a mention, never how many mentions there are.
||| One case split does both, because a condition licenses at most one
||| re-mark and `condRemarkAt` is what says which.
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
                    countOnesAt sl (condRemark c) = countOnesAt sl bs
condRemarkKeepsAt bs c sl with (condRemarkAt c)
  _ | Nothing = Refl
  _ | Just (q, ty) =
    trans (countOnesAtIsFold sl (markFirst q ty bs))
          (trans (markFirstKeeps (itAtReaches sl) q ty (markTyKeepsAt sl ty) bs)
                 (sym (countOnesAtIsFold sl bs)))

||| The same for a condition, which is what `If`'s consequent is typed
||| in -- with the prefix term now the prefix AS THE CONDITION LEFT IT
||| MARKED. The clause is still forward: `condRemark` reads the prefix
||| and rewrites one of its own bindings in place, exactly as
||| `settleTargets` and `defineLetter` do, and inserts nothing minted
||| later. The two lemmas above are what keeps the second clause of the
||| contract honest, and `markFirstKeepsLength` is the first.
public export
condIntroIsDeltaThenRemark : (bs : Bindings) -> (c : Condition bs) ->
                             condIntro c = condDelta c ++ condRemark c
condIntroIsDeltaThenRemark bs c = Refl

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
  trans (countOnesIsFold j (condDelta c ++ condRemark c))
        (trans (countBySplit (oneOfKind j) (condDelta c) (condRemark c))
               (cong2 (+) (sym (countOnesIsFold j (condDelta c)))
                          (trans (sym (countOnesIsFold j (condRemark c)))
                                 (condRemarkKeepsOnes bs c j))))

||| The SCOPED gate splits the same way. Narrowing which mentions are
||| candidates does not change that a threaded context is `delta ++ bs`
||| and that a fold over it is the sum of the two -- so the sort-scoped
||| read costs §5 nothing beyond its own statement, and the unscoped
||| lemmas above stand untouched because `It`'s gate is untouched.
public export
slotGateSplitsAtNomIntro : (bs : Bindings) -> (k : Kind) -> (n : Noun bs k) ->
                           (sl : SlotCarrier) ->
                           countOnesAt sl (nomIntro n) =
                             countOnesAt sl (nounDelta n) + countOnesAt sl bs
slotGateSplitsAtNomIntro bs k n sl =
  trans (countOnesAtIsFold sl (nounDelta n ++ bs))
        (trans (countBySplit (itAtReaches sl) (nounDelta n) bs)
               (cong2 (+) (sym (countOnesAtIsFold sl (nounDelta n)))
                          (sym (countOnesAtIsFold sl bs))))

||| ...and at a condition's context, which is where the card-carrier read
||| is written ("exile the top card of your library. If IT's a creature
||| card, …").
public export
slotGateSplitsAtCondIntro : (bs : Bindings) -> (c : Condition bs) ->
                            (sl : SlotCarrier) ->
                            countOnesAt sl (condIntro c) =
                              countOnesAt sl (condDelta c) + countOnesAt sl bs
slotGateSplitsAtCondIntro bs c sl =
  trans (countOnesAtIsFold sl (condDelta c ++ condRemark c))
        (trans (countBySplit (itAtReaches sl) (condDelta c) (condRemark c))
               (cong2 (+) (sym (countOnesAtIsFold sl (condDelta c)))
                          (trans (sym (countOnesAtIsFold sl (condRemark c)))
                                 (condRemarkKeepsAt bs c sl))))

||| The split arm's gate, likewise.
public export
unionHalfGateSplitsAtNomIntro : (bs : Bindings) -> (k : Kind) -> (n : Noun bs k) ->
                                (w : NounWord) ->
                                countUnionHalf w (nomIntro n) =
                                  countUnionHalf w (nounDelta n) + countUnionHalf w bs
unionHalfGateSplitsAtNomIntro bs k n w =
  trans (countUnionHalfIsFold w (nounDelta n ++ bs))
        (trans (countBySplit (unionHalf w) (nounDelta n) bs)
               (cong2 (+) (sym (countUnionHalfIsFold w (nounDelta n)))
                          (sym (countUnionHalfIsFold w bs))))

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

||| The SHARED-SUBJECT coordination telescope, at `vpIntro`. Same shape
||| as the statement coordination above and threaded the same way; what
||| differs is only that the subject sits outside the list, so every part
||| is typed in a context the subject's own announcement already opened.
public export
subjectVPsThreadPrefix : (bs : Bindings) -> (n : Nat) -> (vp : SubjectVP bs) ->
                         SubjectVPs n (vpIntro vp) -> SubjectVPs (S n) bs
subjectVPsThreadPrefix bs n vp rest = (::) vp rest

||| ...and each part's own output is its delta in front of the context it
||| was given: `VPGains` mints nothing, `VPGets` mints its two amounts'
||| phrases and nothing else.
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
