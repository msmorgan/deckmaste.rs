module Experimental.Unspellable

%default total

public export
0 Unspellable : (0 a : Type) -> {0 P : Type} -> ((0 _ : P) -> a) -> Type
Unspellable _ {P = p} _ = Not p

namespace Dependent
  public export
  0 Unspellable : (0 a : Type) -> {0 P : Type} -> {0 Q : P -> Type} ->
                  ((0 x : P) -> (0 _ : Q x) -> a) -> Type
  Unspellable _ {P = p} {Q = q} _ = Not (x : p ** q x)
