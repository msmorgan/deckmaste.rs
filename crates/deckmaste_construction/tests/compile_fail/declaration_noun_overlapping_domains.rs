use deckmaste_construction::constructions;

constructions! {
    morphology EnglishNoun { feature = Number; recipe = english_noun; }
    lexeme NounLexeme using EnglishNoun { Player = "player", }
    codec SubtypeNoun {
        generate declaration_noun {
            closed = NounLexeme;
            position = Noun;
            kinds = [Subtype];
            feature = Number;
        }
    }
    codec CreatureNoun {
        generate declaration_noun {
            closed = NounLexeme;
            position = Noun;
            kinds = [Subtype(Creature)];
            feature = Number;
        }
    }
}

fn main() {}
