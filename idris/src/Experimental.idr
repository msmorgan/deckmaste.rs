module Experimental
-- 
import public Core as C
-- import public Macros

import Derive.Prelude

%language ElabReflection

namespace Keyword
  mutual
    public export
    UnitKeyword : (intrinsic : Bool) -> Type
    UnitKeyword intrinsic = Keyword {intrinsic, params=False}

    public export
    data Keyword : {intrinsic : Bool} -> {params : Bool} -> Type where
      Deathtouch : Keyword {intrinsic=True, params=False}
      Trample : Keyword {intrinsic=True, params=False}
      Flying : Keyword {intrinsic=False, params=False}
      Hexproof : Keyword {intrinsic=False, params=False}
    
    %runElab derive "Keyword" [Show]
  
namespace KeywordSpec
  public export
  data KeywordSpec : {default False intrinsic : Bool} -> Endophora -> Type where
    Bare : Keyword {intrinsic} -> KeywordSpec {intrinsic} b
    WithCost : Cost b -> Keyword -> KeywordSpec b
    WithPredicate : Predicate b k -> Keyword -> KeywordSpec b
    
namespace KeywordAbility
  public export
  data KeywordAbility : Endophora -> Type where
    Intrinsic : KeywordSpec {intrinsic} b -> KeywordAbility b
    