module Experimental.Unspellable

%default total

||| `Unspellable <sort> <phrase>` — a whole phrase of that sort cannot be
||| written: the phrase, parameterised by whatever obligation it leaves open,
||| has no proof of that obligation. The sort is explicit because a refused
||| phrase may be an `Effect []`, an `Ability`, a `Card`, or smaller.
|||
||| The line is spelled out in full with the real constructors, so a pin
||| states the phrase it refuses rather than a fragment asserted to come
||| from one. `P` is never written down — the elaborator infers it from the
||| line itself, so a pin cannot refute the wrong obligation: there is
||| nowhere to name one.
public export
0 Unspellable : (0 a : Type) -> {0 P : Type} -> ((0 _ : P) -> a) -> Type
Unspellable _ {P = p} _ = Not p
