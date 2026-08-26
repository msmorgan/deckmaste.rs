use RulePosition::Lexical as L;
use RulePosition::Nonterminal as N;
use deckmaste_construction::constructions;

use crate::context::ParseContext;
use crate::parser::LexicalMatch;
use crate::parser::Rule;
use crate::parser::RulePosition;
use crate::parser::ScanInput;
use crate::parser::ownership::RawRenderedClaim;
use crate::render::Render;
use crate::render::Writer;

constructions! {
    vocab Auxiliary { May = "may", Can = "can", Cant = "can't", Must = "must", }
    vocab AtBoundary { Beginning = "the beginning of", End = "end of", }
    vocab TriggerMarker { When = "when", Whenever = "whenever", }
    vocab TurnOwnerPostmodifier {
        YourTurn = "on your turn",
        EachOpponentsTurn = "on each opponent's turn",
    }
    vocab TurnPart {
        Turn = "turn",
        BeginningPhase = "beginning phase",
        FirstMainPhase = "first main phase",
        SecondMainPhase = "second main phase",
        PrecombatMainPhase = "precombat main phase",
        PostcombatMainPhase = "postcombat main phase",
        MainPhase = "main phase",
        Combat = "combat",
        CombatPhase = "combat phase",
        EndingPhase = "ending phase",
        UntapStep = "untap step",
        Upkeep = "upkeep",
        DrawStep = "draw step",
        DeclareAttackersStep = "declare attackers step",
        DeclareBlockersStep = "declare blockers step",
        CombatDamageStep = "combat damage step",
        EndStep = "end step",
        CleanupStep = "cleanup step",
    }
    vocab TurnSpecifier {
        Your = "your",
        Each = "each",
        EachPlayer = "each player's",
        EachOpponent = "each opponent's",
        EachOfYour = "each of your",
        The = "the",
        TheNext = "the next",
    }
    vocab SubjectPronoun { He = "he", It = "it", She = "she", They = "they", You = "you", }
    vocab ObjectPronoun { Her = "her", Him = "him", It = "it", Them = "them", You = "you", }
    vocab PossessiveDeterminerPronoun {
        Her = "her",
        His = "his",
        Its = "its",
        Their = "their",
        Your = "your",
    }
    vocab PossessiveAbsolutePronoun {
        Hers = "hers",
        His = "his",
        Theirs = "theirs",
        Yours = "yours",
    }
    vocab ReflexivePronoun {
        Herself = "herself",
        Himself = "himself",
        Itself = "itself",
        Themself = "themself",
        Themselves = "themselves",
        Yourself = "yourself",
        Yourselves = "yourselves",
    }
    vocab Variable { X = "X", Y = "Y", }
    vocab Color {
        Black = "black",
        Blue = "blue",
        Green = "green",
        Monocolored = "monocolored",
        Multicolored = "multicolored",
        Red = "red",
        White = "white",
    }
    vocab Status {
        Attacking = "attacking",
        Blocked = "blocked",
        Blocking = "blocking",
        Enchanted = "enchanted",
        Tapped = "tapped",
        Untapped = "untapped",
    }
    vocab Designation { Chosen = "chosen", Exiled = "exiled", }
    vocab ChosenQuality { Color = "color", Name = "name", Type = "type", }
    vocab SingularDemonstrative { This = "this", That = "that", }
    vocab ControllerNoun { Opponent = "opponent", Player = "player", }
    vocab FixedCostSymbol {
        Variable = "X",
        White = "W",
        Blue = "U",
        Black = "B",
        Red = "R",
        Green = "G",
        Colorless = "C",
        Snow = "S",
        HybridWhiteBlue = "W/U",
        HybridWhiteBlack = "W/B",
        HybridBlueBlack = "U/B",
        HybridBlueRed = "U/R",
        HybridBlackRed = "B/R",
        HybridBlackGreen = "B/G",
        HybridRedGreen = "R/G",
        HybridRedWhite = "R/W",
        HybridGreenWhite = "G/W",
        HybridGreenBlue = "G/U",
        ColorlessHybridWhite = "C/W",
        ColorlessHybridBlue = "C/U",
        ColorlessHybridBlack = "C/B",
        ColorlessHybridRed = "C/R",
        ColorlessHybridGreen = "C/G",
        PhyrexianWhite = "W/P",
        PhyrexianBlue = "U/P",
        PhyrexianBlack = "B/P",
        PhyrexianRed = "R/P",
        PhyrexianGreen = "G/P",
        HybridPhyrexianWhiteBlue = "W/U/P",
        HybridPhyrexianWhiteBlack = "W/B/P",
        HybridPhyrexianBlueBlack = "U/B/P",
        HybridPhyrexianBlueRed = "U/R/P",
        HybridPhyrexianBlackRed = "B/R/P",
        HybridPhyrexianBlackGreen = "B/G/P",
        HybridPhyrexianRedGreen = "R/G/P",
        HybridPhyrexianRedWhite = "R/W/P",
        HybridPhyrexianGreenWhite = "G/W/P",
        HybridPhyrexianGreenBlue = "G/U/P",
        Tap = "T",
        Untap = "Q",
    }
    vocab ModalChooser {
        You = "choose",
        Opponent = "an opponent chooses",
    }
    vocab ModalChoiceBounds {
        ExactlyOne = "one",
        ExactlyTwo = "two",
        OneToTwo = "one or both",
        OneOrMore = "one or more",
        ZeroToOne = "up to one",
    }
    vocab MonocoloredHybridColor {
        White = "W",
        Blue = "U",
        Black = "B",
        Red = "R",
        Green = "G",
    }
    vocab ScalarCharacteristic { Power = "power", Toughness = "toughness", }
    vocab CounterName { Charge = "charge", Lore = "lore", Stun = "stun", Time = "time", }
    vocab DieShape { SixSided = "six-sided", }
    vocab LibraryPosition { Top = "top", Bottom = "bottom", }
    vocab Zone {
        Battlefield = "battlefield",
        Exile = "exile",
        Graveyard = "graveyard",
        Hand = "hand",
        Library = "library",
        Stack = "stack",
    }
    vocab NonCommonNoun { Token = "token", }
    vocab NonTargetCommonModifier {
        Card = "card",
        Controller = "controller",
        Opponent = "opponent",
        Owner = "owner",
        Permanent = "permanent",
        Player = "player",
        Source = "source",
        Spell = "spell",
        Token = "token",
    }
    vocab Supertype {
        Basic = "basic",
        Legendary = "legendary",
        Ongoing = "ongoing",
        Snow = "snow",
        World = "world",
    }

    morphology EnglishVerb {
        feature = Agreement;
        recipe = english_verb;
    }
    morphology EnglishNoun {
        feature = Number;
        recipe = english_noun;
    }
    lexeme CommonNoun using EnglishNoun {
        Ability = "ability" {
            Plural = "abilities",
        },
        Card = "card",
        Counter = "counter",
        Controller = "controller",
        Opponent = "opponent",
        Owner = "owner",
        Permanent = "permanent",
        Player = "player",
        Source = "source",
        Spell = "spell",
        Token = "token",
        Hand = "hand",
        Die = "die" {
            Plural = "dice",
        },
    }
    lexeme VerbLexeme using EnglishVerb {
        Add = "add",
        Deal = "deal",
        Draw = "draw",
        Enter = "enter",
        Gain = "gain",
        Lose = "lose",
        Pay = "pay",
        Put = "put",
        Remove = "remove",
        Roll = "roll",
        Have = "have" {
            ThirdPersonSingular = "has",
        },
        Look = "look",
        Leave = "leave",
        Control = "control",
        Own = "own",
        Return = "return",
        Be = "be" {
            Bare = "are",
            ThirdPersonSingular = "is",
        },
    }
    lexeme CoreIntransitiveVerb using EnglishVerb {
        Attack = "attack",
        Block = "block",
        Cycle = "cycle",
        Die = "die" {
            ThirdPersonSingular = "dies",
        },
        Enter = "enter",
        Leave = "leave",
    }
    lexeme CoreTransitiveVerb using EnglishVerb {
        Attack = "attack",
        Block = "block",
        Control = "control",
        Draw = "draw",
        Own = "own",
    }
    lexeme CoreNumerativeVerb using EnglishVerb { Draw = "draw", }

    codec IntransitiveVerb {
        generate declaration_verb {
            closed = CoreIntransitiveVerb;
            position = Verb;
            kinds = [KeywordAction];
            tail = [];
            feature = Agreement;
        }
    }
    codec TransitiveVerb {
        generate declaration_verb {
            closed = CoreTransitiveVerb;
            position = Verb;
            kinds = [KeywordAction];
            tail = [ObjectNounPhrase];
            feature = Agreement;
        }
    }
    codec NumerativeVerb {
        generate declaration_verb {
            closed = CoreNumerativeVerb;
            position = Verb;
            kinds = [KeywordAction];
            tail = [Amount];
            feature = Agreement;
        }
    }
    codec SearchForVerb {
        generate declaration_verb {
            position = Verb;
            kinds = [KeywordAction];
            tail = [
                location: ObjectNounPhrase,
                "for",
                sought: ObjectNounPhrase,
            ];
            feature = Agreement;
        }
    }

    codec TypeNoun {
        generate declaration_noun {
            position = Noun;
            kinds = [Type];
            feature = Number;
        }
    }
    codec ArtifactSubtypeNoun {
        generate declaration_noun {
            position = Noun;
            kinds = [Subtype(Artifact)];
            feature = Number;
        }
    }
    codec BattleSubtypeNoun {
        generate declaration_noun {
            position = Noun;
            kinds = [Subtype(Battle)];
            feature = Number;
        }
    }
    codec CreatureSubtypeNoun {
        generate declaration_noun {
            position = Noun;
            kinds = [Subtype(Creature)];
            feature = Number;
        }
    }
    codec EnchantmentSubtypeNoun {
        generate declaration_noun {
            position = Noun;
            kinds = [Subtype(Enchantment)];
            feature = Number;
        }
    }
    codec LandSubtypeNoun {
        generate declaration_noun {
            position = Noun;
            kinds = [Subtype(Land)];
            feature = Number;
        }
    }
    codec PlaneswalkerSubtypeNoun {
        generate declaration_noun {
            position = Noun;
            kinds = [Subtype(Planeswalker)];
            feature = Number;
        }
    }
    codec SpellSubtypeNoun {
        generate declaration_noun {
            position = Noun;
            kinds = [Subtype(Spell)];
            feature = Number;
        }
    }
    identity SelfReferenceSpelling {
        generate context {
            Full => card_name,
            Abbreviated => abbreviated_card_name,
            canonical_on_collision = Full;
        }
    }
    identity CardName {
        generate catalog_identity {
            provider = CardNames;
        }
    }
    codec CardinalNumber {
        generate english_cardinal {
            magnitude = u32;
        }
    }
    codec ScalarNumber {
        generate unsigned_decimal {
            magnitude = u32;
        }
    }
    codec LoyaltyMagnitude {
        generate unsigned_decimal {
            magnitude = NonZeroU32;
        }
    }
    abstract sum ActivationCostComponent {
        SymbolRun,
        Loyalty,
        Clause: CostClause,
    }
    abstract sum Predicate {
        Atomic: VerbPhrase,
        Coordination: PredicateCoordination,
    }
    abstract sum Clause {
        Finite: FiniteClause,
        Coordination: ClauseCoordination,
    }
    abstract sum ClauseAttachment {
        PreposedIf,
        PreposedIfPredicate,
        PostposedIf,
        PostposedIfPredicate,
        PostposedUnless,
        PostposedUnlessPredicate,
        PreposedAsLongAs,
        PreposedAsLongAsPredicate,
        PreposedWhile,
        PreposedWhilePredicate,
        PreposedDuring,
        PreposedDuringPredicate,
        PreposedUntil,
        PreposedUntilPredicate,
        ThenSequence,
        ThenPredicateSequence,
        ReflexiveSubordinate,
        ReflexivePredicateSubordinate,
    }
    abstract sum ConditionClause { FiniteCondition, ExistentialCondition, }
    construction finite_condition: FiniteCondition {
        element FiniteConditionValue { clause: FiniteClause, }
        form finite_condition = "if" clause ",";
    }
    construction existential_condition: ExistentialCondition {
        element ExistentialConditionValue { clause: ExistentialClause, }
        form existential_condition = "if" clause ",";
    }
    construction singular_existential_clause: ExistentialClause {
        element SingularExistentialClause {
            pivot: NounPhrase,
            domain: opt AmongPhrase,
        }
        require pivot.number is Singular;
        derive verb.agreement = Values::ThirdPersonSingular;
        form singular_existential_clause = "there" verb(VerbLexeme::Be) pivot domain;
    }
    construction plural_existential_clause: ExistentialClause {
        element PluralExistentialClause {
            pivot: NounPhrase,
            domain: opt AmongPhrase,
        }
        require pivot.number is Plural;
        derive verb.agreement = Values::Bare;
        form plural_existential_clause = "there" verb(VerbLexeme::Be) pivot domain;
    }
    construction among_phrase: AmongPhrase {
        element AmongPhraseValue { domain: NounPhrase, }
        form among_phrase = "among" domain;
    }
    construction plain: Ability {
        element Plain {
            body: AbilityBody,
        }
        form plain = body;
    }
    construction sentences: AbilityBody {
        element Sentences {
            sentences: seq Sentence separated by " " terminated by ".",
        }
        require len(sentences) >= 1;
        form sentences = sentences;
    }
    construction modal_mode: ModalMode {
        element ModalModeValue {
            sentences: seq Sentence separated by " " terminated by ".",
        }
        require len(sentences) >= 1;
        form modal_mode = sentence_initial("• ") sentences;
    }
    construction plain_modal: AbilityBody {
        element PlainModal {
            chooser: lex ModalChooser,
            bounds: lex ModalChoiceBounds,
            modes: seq ModalMode separated by sentence_initial("\n"),
        }
        require len(modes) >= 2;
        require any(
            chooser is You,
            all(chooser is Opponent, bounds is ExactlyOne)
        );
        form exactly_one when all(chooser is You, bounds is ExactlyOne) =
            lex(chooser) lex(bounds) sentence_initial(" —\n") modes;
        form exactly_two when all(chooser is You, bounds is ExactlyTwo) =
            lex(chooser) lex(bounds) sentence_initial(" —\n") modes;
        form one_to_two when all(chooser is You, bounds is OneToTwo) =
            lex(chooser) lex(bounds) sentence_initial(" —\n") modes;
        form one_or_more when all(chooser is You, bounds is OneOrMore) =
            lex(chooser) lex(bounds) sentence_initial(" —\n") modes;
        form zero_to_one when all(chooser is You, bounds is ZeroToOne) =
            lex(chooser) lex(bounds) sentence_initial(" —\n") modes;
        form opponent_exactly_one otherwise =
            lex(chooser) lex(bounds) sentence_initial(" —\n") modes;
    }
    construction finite: TriggerPrefix {
        element Finite {
            marker: lex TriggerMarker,
            clause: FiniteClause,
        }
        form finite = lex(marker) clause;
    }
    construction temporal: TriggerPrefix {
        element Temporal { phrase: AtPhrase, }
        form temporal = "at" phrase;
    }
    construction at_phrase: AtPhrase {
        element AtPhraseValue {
            boundary: lex AtBoundary,
            specifier: opt lex TurnSpecifier,
            part: lex TurnPart,
            postmodifier: opt lex TurnOwnerPostmodifier,
        }
        require any(
            all(
                boundary is End,
                specifier.is_none(),
                part is Combat,
                postmodifier.is_none()
            ),
            all(
                boundary is Beginning,
                specifier is EachOfYour,
                part in [
                    FirstMainPhase,
                    SecondMainPhase,
                    PrecombatMainPhase,
                    PostcombatMainPhase,
                    MainPhase
                ],
                postmodifier.is_none()
            ),
            all(
                boundary is Beginning,
                specifier.is_none(),
                postmodifier.is_none()
            ),
            all(
                boundary is Beginning,
                specifier in [Your, Each, EachPlayer, EachOpponent, The, TheNext],
                postmodifier.is_none()
            ),
            all(
                boundary is Beginning,
                specifier.is_none(),
                part is Combat,
                postmodifier.is_some()
            ),
            all(
                boundary is Beginning,
                specifier in [Your, Each, EachPlayer, EachOpponent, The, TheNext],
                part is Combat,
                postmodifier.is_some()
            )
        );
        form plural_main_phase when specifier is EachOfYour =
            lex(boundary) lex(specifier) suffix(lex(part), "s") lex(postmodifier);
        form singular otherwise =
            lex(boundary) lex(specifier) lex(part) lex(postmodifier);
    }
    construction triggered: Ability {
        element Triggered {
            trigger: TriggerPrefix,
            intervening_if: opt ConditionClause,
            body: AbilityBody,
        }
        form triggered = trigger "," intervening_if body;
    }
    construction generic_cost_symbol: CostSymbol {
        element GenericCostSymbol { magnitude: lex ScalarNumber, }
        form generic_cost_symbol = lex(magnitude);
    }
    construction fixed_cost_symbol: CostSymbol {
        element FixedSymbol { symbol: lex FixedCostSymbol, }
        form fixed_cost_symbol = lex(symbol);
    }
    construction monocolored_hybrid_symbol: CostSymbol {
        element MonocoloredHybridSymbol { color: lex MonocoloredHybridColor, }
        form monocolored_hybrid_symbol = prefix("2/", lex(color));
    }
    construction symbol_run: ActivationCostComponent {
        element SymbolRun {
            symbols: seq CostSymbol separated by "}{",
        }
        require len(symbols) >= 1;
        form symbol_run = circumfix("{", symbols, "}");
    }
    construction positive_loyalty: LoyaltyValue {
        element PositiveLoyalty { magnitude: lex LoyaltyMagnitude, }
        form positive_loyalty = prefix("+", lex(magnitude));
    }
    construction zero_loyalty: LoyaltyValue {
        element ZeroLoyalty {}
        form zero_loyalty = "0";
    }
    construction negative_loyalty: LoyaltyValue {
        element NegativeLoyalty { magnitude: lex LoyaltyMagnitude, }
        form negative_loyalty = prefix("−", lex(magnitude));
    }
    construction loyalty: ActivationCostComponent {
        element Loyalty { value: LoyaltyValue, }
        form loyalty = circumfix("[", value, "]");
    }
    construction cost_clause: ActivationCostComponent {
        element CostClause { predicate: VerbPhrase, }
        derive predicate.agreement = Values::Bare;
        form cost_clause = predicate;
    }
    construction activated: Ability {
        element Activated {
            costs: seq ActivationCostComponent separated by position {
                pair = sentence_initial(", ");
                first = sentence_initial(", ");
                middle = sentence_initial(", ");
                last = sentence_initial(", ");
            },
            body: AbilityBody,
        }
        require len(costs) >= 1;
        form activated = costs sentence_initial(": ") body;
    }
    construction imperative: Sentence {
        element Imperative { predicate: Predicate, }
        derive predicate.agreement = Values::Bare;
        form imperative = predicate;
    }
    construction declarative: Sentence {
        element Declarative { clause: Clause, }
        form declarative = clause;
    }
    construction attached: Sentence {
        element Attached { attachment: ClauseAttachment, }
        form attached = attachment;
    }
    construction preposed_if: ClauseAttachment {
        element PreposedIf { condition: FiniteClause, body: Clause, }
        form preposed_if = "if" condition "," body;
    }
    construction preposed_if_predicate: ClauseAttachment {
        element PreposedIfPredicate { condition: FiniteClause, body: Predicate, }
        derive body.agreement = Values::Bare;
        form preposed_if_predicate = "if" condition "," body;
    }
    construction postposed_if: ClauseAttachment {
        element PostposedIf { body: Clause, condition: FiniteClause, }
        form postposed_if = body "if" condition;
    }
    construction postposed_if_predicate: ClauseAttachment {
        element PostposedIfPredicate { body: Predicate, condition: FiniteClause, }
        derive body.agreement = Values::Bare;
        form postposed_if_predicate = body "if" condition;
    }
    construction postposed_unless: ClauseAttachment {
        element PostposedUnless { body: Clause, condition: FiniteClause, }
        form postposed_unless = body "unless" condition;
    }
    construction postposed_unless_predicate: ClauseAttachment {
        element PostposedUnlessPredicate { body: Predicate, condition: FiniteClause, }
        derive body.agreement = Values::Bare;
        form postposed_unless_predicate = body "unless" condition;
    }
    construction preposed_as_long_as: ClauseAttachment {
        element PreposedAsLongAs { condition: FiniteClause, body: Clause, }
        form preposed_as_long_as = "as" "long" "as" condition "," body;
    }
    construction preposed_as_long_as_predicate: ClauseAttachment {
        element PreposedAsLongAsPredicate { condition: FiniteClause, body: Predicate, }
        derive body.agreement = Values::Bare;
        form preposed_as_long_as_predicate = "as" "long" "as" condition "," body;
    }
    construction preposed_while: ClauseAttachment {
        element PreposedWhile { condition: FiniteClause, body: Clause, }
        form preposed_while = "while" condition "," body;
    }
    construction preposed_while_predicate: ClauseAttachment {
        element PreposedWhilePredicate { condition: FiniteClause, body: Predicate, }
        derive body.agreement = Values::Bare;
        form preposed_while_predicate = "while" condition "," body;
    }
    construction preposed_during: ClauseAttachment {
        element PreposedDuring { condition: FiniteClause, body: Clause, }
        form preposed_during = "during" condition "," body;
    }
    construction preposed_during_predicate: ClauseAttachment {
        element PreposedDuringPredicate { condition: FiniteClause, body: Predicate, }
        derive body.agreement = Values::Bare;
        form preposed_during_predicate = "during" condition "," body;
    }
    construction preposed_until: ClauseAttachment {
        element PreposedUntil { condition: FiniteClause, body: Clause, }
        form preposed_until = "until" condition "," body;
    }
    construction preposed_until_predicate: ClauseAttachment {
        element PreposedUntilPredicate { condition: FiniteClause, body: Predicate, }
        derive body.agreement = Values::Bare;
        form preposed_until_predicate = "until" condition "," body;
    }
    construction then_sequence: ClauseAttachment {
        element ThenSequence {
            members: seq Clause separated by position {
                pair = ", then ";
                first = ", then ";
                middle = ", then ";
                last = ", then ";
            },
        }
        require len(members) >= 2;
        form then_sequence = members;
    }
    construction then_predicate_sequence: ClauseAttachment {
        element ThenPredicateSequence {
            members: seq Predicate separated by position {
                pair = ", then ";
                first = ", then ";
                middle = ", then ";
                last = ", then ";
            },
        }
        require len(members) >= 2;
        derive members.agreement = Values::Bare;
        form then_predicate_sequence = members;
    }
    construction reflexive_subordinate: ClauseAttachment {
        element ReflexiveSubordinate { kind: lex ReflexiveSubordinateKind, body: Clause, }
        form reflexive_subordinate = lex(kind) "," body;
    }
    construction reflexive_predicate_subordinate: ClauseAttachment {
        element ReflexivePredicateSubordinate {
            kind: lex ReflexiveSubordinateKind,
            body: Predicate,
        }
        derive body.agreement = Values::Bare;
        form reflexive_predicate_subordinate = lex(kind) "," body;
    }
    construction with_where: Sentence {
        element WithWhere { body: Sentence, clause: WhereClauseCategory, }
        form with_where = body "," clause;
    }
    construction and_predicate_coordination: PredicateCoordination {
        element AndPredicateCoordination {
            members: seq VerbPhrase separated by position {
                pair = " and ";
                first = ", ";
                middle = ", ";
                last = ", and ";
            },
        }
        require len(members) >= 2;
        derive agreement = members.agreement;
        form and_predicate_coordination = members;
    }
    construction or_predicate_coordination: PredicateCoordination {
        element OrPredicateCoordination {
            members: seq VerbPhrase separated by position {
                pair = " or ";
                first = ", ";
                middle = ", ";
                last = ", or ";
            },
        }
        require len(members) >= 2;
        derive agreement = members.agreement;
        form or_predicate_coordination = members;
    }
    construction and_or_predicate_coordination: PredicateCoordination {
        element AndOrPredicateCoordination {
            members: seq VerbPhrase separated by position {
                pair = " and/or ";
                first = ", ";
                middle = ", ";
                last = ", and/or ";
            },
        }
        require len(members) >= 2;
        derive agreement = members.agreement;
        form and_or_predicate_coordination = members;
    }
    construction plain_finite_clause: FiniteClause {
        element PlainFiniteClause { subject: Subject, predicate: Predicate, }
        derive predicate.agreement = subject.agreement;
        form plain_finite_clause = subject predicate;
    }
    construction auxiliary_finite_clause: FiniteClause {
        element AuxiliaryFiniteClause {
            subject: Subject,
            auxiliary: lex Auxiliary,
            predicate: Predicate,
        }
        derive predicate.agreement = Values::Bare;
        form auxiliary_finite_clause = subject lex(auxiliary) predicate;
    }
    construction and_clause_coordination: ClauseCoordination {
        element AndClauseCoordination {
            members: seq FiniteClause separated by position {
                pair = " and ";
                first = ", ";
                middle = ", ";
                last = ", and ";
            },
        }
        require len(members) >= 2;
        form and_clause_coordination = members;
    }
    construction or_clause_coordination: ClauseCoordination {
        element OrClauseCoordination {
            members: seq FiniteClause separated by position {
                pair = " or ";
                first = ", ";
                middle = ", ";
                last = ", or ";
            },
        }
        require len(members) >= 2;
        form or_clause_coordination = members;
    }
    construction and_or_clause_coordination: ClauseCoordination {
        element AndOrClauseCoordination {
            members: seq FiniteClause separated by position {
                pair = " and/or ";
                first = ", ";
                middle = ", ";
                last = ", and/or ";
            },
        }
        require len(members) >= 2;
        form and_or_clause_coordination = members;
    }
    construction where: WhereClauseCategory {
        element WhereClause { variable: lex Variable, value: Object, }
        derive verb.agreement = Values::ThirdPersonSingular;
        form where = "where" lex(variable) verb(VerbLexeme::Be)
            "the" "number" "of" value;
    }
    construction subject_nominal: Subject {
        element NominalSubject { value: NounPhrase, }
        derive agreement = value.agreement;
        derive number = value.number;
        derive onset = value.onset;
        form subject_nominal = value;
    }
    construction subject_pronoun: Subject {
        element PersonalSubject { word: lex SubjectPronoun, }
        derive word.agreement = match word {
            He => Values::ThirdPersonSingular,
            It => Values::ThirdPersonSingular,
            She => Values::ThirdPersonSingular,
            They => Values::Bare,
            You => Values::Bare,
        };
        derive agreement = word.agreement;
        derive number = Values::Singular;
        derive onset = word.onset;
        form subject_pronoun = lex(word);
    }
    construction object_nominal: Object {
        element NominalObject { value: NounPhrase, }
        derive agreement = value.agreement;
        derive number = value.number;
        derive onset = value.onset;
        form object_nominal = value;
    }
    construction object_pronoun: Object {
        element PersonalObject { word: lex ObjectPronoun, }
        derive agreement = match word {
            Her => Values::ThirdPersonSingular,
            Him => Values::ThirdPersonSingular,
            It => Values::ThirdPersonSingular,
            Them => Values::Bare,
            You => Values::Bare,
        };
        derive number = Values::Singular;
        derive onset = word.onset;
        form object_pronoun = lex(word);
    }
    construction reflexive_object: Object {
        element ReflexiveObject { word: lex ReflexivePronoun, }
        derive agreement = match word {
            Herself => Values::ThirdPersonSingular,
            Himself => Values::ThirdPersonSingular,
            Itself => Values::ThirdPersonSingular,
            Themself => Values::Bare,
            Themselves => Values::Bare,
            Yourself => Values::Bare,
            Yourselves => Values::Bare,
        };
        derive number = match word {
            Herself => Values::Singular,
            Himself => Values::Singular,
            Itself => Values::Singular,
            Themself => Values::Singular,
            Themselves => Values::Plural,
            Yourself => Values::Singular,
            Yourselves => Values::Plural,
        };
        derive onset = word.onset;
        form reflexive_object = lex(word);
    }
    construction choice_object: Object {
        element ChoiceObject { value: NounPhrase, }
        derive agreement = value.agreement;
        derive number = value.number;
        derive onset = value.onset;
        form choice_object = value "of" "their" "choice";
    }
    construction random_object: Object {
        element RandomObject { value: NounPhrase, }
        derive agreement = value.agreement;
        derive number = value.number;
        derive onset = value.onset;
        form random_object = value "at" "random";
    }
    construction common_singular_head: SingularHead {
        element CommonSingularHead { noun: lex CommonNoun, }
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = noun.onset;
        derive possessive_ending = noun.possessive_ending;
        form common_singular_head = noun(noun);
    }
    construction type_singular_head: SingularHead {
        element TypeSingularHead { noun: lex TypeNoun, }
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = noun.onset;
        derive possessive_ending = noun.possessive_ending;
        form type_singular_head = noun(noun);
    }
    construction artifact_subtype_singular_head: SingularHead {
        element ArtifactSubtypeSingularHead { noun: lex ArtifactSubtypeNoun, }
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = noun.onset;
        derive possessive_ending = noun.possessive_ending;
        form artifact_subtype_singular_head = noun(noun);
    }
    construction battle_subtype_singular_head: SingularHead {
        element BattleSubtypeSingularHead { noun: lex BattleSubtypeNoun, }
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = noun.onset;
        derive possessive_ending = noun.possessive_ending;
        form battle_subtype_singular_head = noun(noun);
    }
    construction creature_subtype_singular_head: SingularHead {
        element CreatureSubtypeSingularHead { noun: lex CreatureSubtypeNoun, }
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = noun.onset;
        derive possessive_ending = noun.possessive_ending;
        form creature_subtype_singular_head = noun(noun);
    }
    construction enchantment_subtype_singular_head: SingularHead {
        element EnchantmentSubtypeSingularHead { noun: lex EnchantmentSubtypeNoun, }
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = noun.onset;
        derive possessive_ending = noun.possessive_ending;
        form enchantment_subtype_singular_head = noun(noun);
    }
    construction land_subtype_singular_head: SingularHead {
        element LandSubtypeSingularHead { noun: lex LandSubtypeNoun, }
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = noun.onset;
        derive possessive_ending = noun.possessive_ending;
        form land_subtype_singular_head = noun(noun);
    }
    construction planeswalker_subtype_singular_head: SingularHead {
        element PlaneswalkerSubtypeSingularHead { noun: lex PlaneswalkerSubtypeNoun, }
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = noun.onset;
        derive possessive_ending = noun.possessive_ending;
        form planeswalker_subtype_singular_head = noun(noun);
    }
    construction spell_subtype_singular_head: SingularHead {
        element SpellSubtypeSingularHead { noun: lex SpellSubtypeNoun, }
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = noun.onset;
        derive possessive_ending = noun.possessive_ending;
        form spell_subtype_singular_head = noun(noun);
    }
    construction common_plural_head: PluralHead {
        element CommonPluralHead { noun: lex CommonNoun, }
        derive noun.number = Values::Plural;
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive onset = noun.onset;
        derive possessive_ending = noun.possessive_ending;
        form common_plural_head = noun(noun);
    }
    construction type_plural_head: PluralHead {
        element TypePluralHead { noun: lex TypeNoun, }
        derive noun.number = Values::Plural;
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive onset = noun.onset;
        derive possessive_ending = noun.possessive_ending;
        form type_plural_head = noun(noun);
    }
    construction artifact_subtype_plural_head: PluralHead {
        element ArtifactSubtypePluralHead { noun: lex ArtifactSubtypeNoun, }
        derive noun.number = Values::Plural;
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive onset = noun.onset;
        derive possessive_ending = noun.possessive_ending;
        form artifact_subtype_plural_head = noun(noun);
    }
    construction battle_subtype_plural_head: PluralHead {
        element BattleSubtypePluralHead { noun: lex BattleSubtypeNoun, }
        derive noun.number = Values::Plural;
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive onset = noun.onset;
        derive possessive_ending = noun.possessive_ending;
        form battle_subtype_plural_head = noun(noun);
    }
    construction creature_subtype_plural_head: PluralHead {
        element CreatureSubtypePluralHead { noun: lex CreatureSubtypeNoun, }
        derive noun.number = Values::Plural;
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive onset = noun.onset;
        derive possessive_ending = noun.possessive_ending;
        form creature_subtype_plural_head = noun(noun);
    }
    construction enchantment_subtype_plural_head: PluralHead {
        element EnchantmentSubtypePluralHead { noun: lex EnchantmentSubtypeNoun, }
        derive noun.number = Values::Plural;
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive onset = noun.onset;
        derive possessive_ending = noun.possessive_ending;
        form enchantment_subtype_plural_head = noun(noun);
    }
    construction land_subtype_plural_head: PluralHead {
        element LandSubtypePluralHead { noun: lex LandSubtypeNoun, }
        derive noun.number = Values::Plural;
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive onset = noun.onset;
        derive possessive_ending = noun.possessive_ending;
        form land_subtype_plural_head = noun(noun);
    }
    construction planeswalker_subtype_plural_head: PluralHead {
        element PlaneswalkerSubtypePluralHead { noun: lex PlaneswalkerSubtypeNoun, }
        derive noun.number = Values::Plural;
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive onset = noun.onset;
        derive possessive_ending = noun.possessive_ending;
        form planeswalker_subtype_plural_head = noun(noun);
    }
    construction spell_subtype_plural_head: PluralHead {
        element SpellSubtypePluralHead { noun: lex SpellSubtypeNoun, }
        derive noun.number = Values::Plural;
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive onset = noun.onset;
        derive possessive_ending = noun.possessive_ending;
        form spell_subtype_plural_head = noun(noun);
    }
    construction color_modifier: NominalModifier {
        element ColorModifier { color: lex Color, }
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = color.onset;
        form color_modifier = lex(color);
    }
    construction status_modifier: NominalModifier {
        element StatusModifier { status: lex Status, }
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = status.onset;
        form status_modifier = lex(status);
    }
    construction supertype_modifier: NominalModifier {
        element SupertypeModifier { supertype: lex Supertype, }
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = supertype.onset;
        form supertype_modifier = lex(supertype);
    }
    construction common_noun_modifier: NominalModifier {
        element CommonNounModifier { noun: lex CommonNoun, }
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = noun.onset;
        form common_noun_modifier = noun(noun);
    }
    construction type_modifier: NominalModifier {
        element TypeModifier { noun: lex TypeNoun, }
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = noun.onset;
        form type_modifier = noun(noun);
    }
    construction artifact_subtype_modifier: NominalModifier {
        element ArtifactSubtypeModifier { noun: lex ArtifactSubtypeNoun, }
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = noun.onset;
        form artifact_subtype_modifier = noun(noun);
    }
    construction battle_subtype_modifier: NominalModifier {
        element BattleSubtypeModifier { noun: lex BattleSubtypeNoun, }
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = noun.onset;
        form battle_subtype_modifier = noun(noun);
    }
    construction creature_subtype_modifier: NominalModifier {
        element CreatureSubtypeModifier { noun: lex CreatureSubtypeNoun, }
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = noun.onset;
        form creature_subtype_modifier = noun(noun);
    }
    construction enchantment_subtype_modifier: NominalModifier {
        element EnchantmentSubtypeModifier { noun: lex EnchantmentSubtypeNoun, }
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = noun.onset;
        form enchantment_subtype_modifier = noun(noun);
    }
    construction land_subtype_modifier: NominalModifier {
        element LandSubtypeModifier { noun: lex LandSubtypeNoun, }
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = noun.onset;
        form land_subtype_modifier = noun(noun);
    }
    construction planeswalker_subtype_modifier: NominalModifier {
        element PlaneswalkerSubtypeModifier { noun: lex PlaneswalkerSubtypeNoun, }
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = noun.onset;
        form planeswalker_subtype_modifier = noun(noun);
    }
    construction spell_subtype_modifier: NominalModifier {
        element SpellSubtypeModifier { noun: lex SpellSubtypeNoun, }
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = noun.onset;
        form spell_subtype_modifier = noun(noun);
    }
    construction non_color_modifier: NominalModifier {
        element NonColorModifier { color: lex Color, }
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        form non_color_modifier = prefix("non", lex(color));
    }
    construction non_common_noun_modifier: NominalModifier {
        element NonCommonNounModifier { noun: lex NonCommonNoun, }
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        form non_common_noun_modifier = prefix("non", lex(noun));
    }
    construction non_status_modifier: NominalModifier {
        element NonStatusModifier { status: lex Status, }
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        form non_status_modifier = prefix("non", lex(status));
    }
    construction non_supertype_modifier: NominalModifier {
        element NonSupertypeModifier { supertype: lex Supertype, }
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        form non_supertype_modifier = prefix("non", lex(supertype));
    }
    construction non_type_modifier: NominalModifier {
        element NonTypeModifier { noun: lex TypeNoun, }
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        form non_type_modifier = prefix("non", noun(noun));
    }
    construction non_artifact_subtype_modifier: NominalModifier {
        element NonArtifactSubtypeModifier { noun: lex ArtifactSubtypeNoun, }
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        form non_artifact_subtype_modifier = prefix("non-", noun(noun));
    }
    construction non_battle_subtype_modifier: NominalModifier {
        element NonBattleSubtypeModifier { noun: lex BattleSubtypeNoun, }
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        form non_battle_subtype_modifier = prefix("non-", noun(noun));
    }
    construction non_creature_subtype_modifier: NominalModifier {
        element NonCreatureSubtypeModifier { noun: lex CreatureSubtypeNoun, }
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        form non_creature_subtype_modifier = prefix("non-", noun(noun));
    }
    construction non_enchantment_subtype_modifier: NominalModifier {
        element NonEnchantmentSubtypeModifier { noun: lex EnchantmentSubtypeNoun, }
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        form non_enchantment_subtype_modifier = prefix("non-", noun(noun));
    }
    construction non_land_subtype_modifier: NominalModifier {
        element NonLandSubtypeModifier { noun: lex LandSubtypeNoun, }
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        form non_land_subtype_modifier = prefix("non-", noun(noun));
    }
    construction non_planeswalker_subtype_modifier: NominalModifier {
        element NonPlaneswalkerSubtypeModifier { noun: lex PlaneswalkerSubtypeNoun, }
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        form non_planeswalker_subtype_modifier = prefix("non-", noun(noun));
    }
    construction non_spell_subtype_modifier: NominalModifier {
        element NonSpellSubtypeModifier { noun: lex SpellSubtypeNoun, }
        derive noun.number = Values::Singular;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        form non_spell_subtype_modifier = prefix("non-", noun(noun));
    }
    construction negative_modifier_member: NegativeNominalModifier {
        element NegativeModifierMember { value: NominalModifier, }
        require any(
            value is NonColorModifier,
            value is NonCommonNounModifier,
            value is NonStatusModifier,
            value is NonSupertypeModifier,
            value is NonTypeModifier,
            value is NonArtifactSubtypeModifier,
            value is NonBattleSubtypeModifier,
            value is NonCreatureSubtypeModifier,
            value is NonEnchantmentSubtypeModifier,
            value is NonLandSubtypeModifier,
            value is NonPlaneswalkerSubtypeModifier,
            value is NonSpellSubtypeModifier
        );
        form negative_modifier_member = value;
    }
    construction non_target_common_noun_modifier: CoordinatedNominalModifier {
        element NonTargetCommonNounModifier { noun: lex NonTargetCommonModifier, }
        derive onset = noun.onset;
        form non_target_common_noun_modifier = lex(noun);
    }
    construction coordinated_modifier_member: CoordinatedNominalModifier {
        element CoordinatedModifierMember { value: NominalModifier, }
        require any(
            value is ColorModifier,
            value is StatusModifier,
            value is SupertypeModifier,
            value is TypeModifier,
            value is ArtifactSubtypeModifier,
            value is BattleSubtypeModifier,
            value is CreatureSubtypeModifier,
            value is EnchantmentSubtypeModifier,
            value is LandSubtypeModifier,
            value is PlaneswalkerSubtypeModifier,
            value is SpellSubtypeModifier,
            value is NonColorModifier,
            value is NonCommonNounModifier,
            value is NonStatusModifier,
            value is NonSupertypeModifier,
            value is NonTypeModifier,
            value is NonArtifactSubtypeModifier,
            value is NonBattleSubtypeModifier,
            value is NonCreatureSubtypeModifier,
            value is NonEnchantmentSubtypeModifier,
            value is NonLandSubtypeModifier,
            value is NonPlaneswalkerSubtypeModifier,
            value is NonSpellSubtypeModifier
        );
        derive onset = value.onset;
        form coordinated_modifier_member = value;
    }
    construction compound_modifier_member: CompoundNominalModifier {
        element CompoundModifierMember { value: NominalModifier, }
        require any(
            value is ColorModifier,
            value is StatusModifier,
            value is SupertypeModifier,
            value is CommonNounModifier,
            value is TypeModifier,
            value is ArtifactSubtypeModifier,
            value is BattleSubtypeModifier,
            value is CreatureSubtypeModifier,
            value is EnchantmentSubtypeModifier,
            value is LandSubtypeModifier,
            value is PlaneswalkerSubtypeModifier,
            value is SpellSubtypeModifier
        );
        derive onset = value.onset;
        form compound_modifier_member = value;
    }
    construction bare_singular_nominal: SingularNominal {
        element BareSingularNominal { head: SingularHead, }
        derive agreement = head.agreement;
        derive number = head.number;
        derive onset = head.onset;
        form bare_singular_nominal = head;
    }
    construction modified_singular_nominal: SingularNominal {
        element ModifiedSingularNominal { modifier: NominalModifier, head: SingularHead, }
        derive agreement = head.agreement;
        derive number = head.number;
        derive onset = modifier.onset;
        form modified_singular_nominal = modifier head;
    }
    construction negative_modified_singular_nominal: SingularNominal {
        element NegativeModifiedSingularNominal {
            modifiers: seq NegativeNominalModifier separated by ", ",
            head: SingularHead,
        }
        require len(modifiers) >= 2;
        derive agreement = head.agreement;
        derive number = head.number;
        derive onset = Values::Consonant;
        form negative_modified_singular_nominal = modifiers head;
    }
    construction bare_plural_nominal: PluralNominal {
        element BarePluralNominal { head: PluralHead, }
        derive agreement = head.agreement;
        derive number = head.number;
        derive onset = head.onset;
        form bare_plural_nominal = head;
    }
    construction modified_plural_nominal: PluralNominal {
        element ModifiedPluralNominal { modifier: NominalModifier, head: PluralHead, }
        derive agreement = head.agreement;
        derive number = head.number;
        derive onset = modifier.onset;
        form modified_plural_nominal = modifier head;
    }
    construction compound_modified_singular_nominal: SingularNominal {
        element CompoundModifiedSingularNominal {
            first: CompoundNominalModifier,
            rest: seq CompoundNominalModifier separated by " ",
        }
        require len(rest) >= 1;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = first.onset;
        form compound_modified_singular_nominal = first rest "type";
    }
    construction compound_modified_plural_nominal: PluralNominal {
        element CompoundModifiedPluralNominal {
            first: CompoundNominalModifier,
            rest: seq CompoundNominalModifier separated by " ",
        }
        require len(rest) >= 1;
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive onset = first.onset;
        form compound_modified_plural_nominal = first rest "types";
    }
    construction negative_modified_plural_nominal: PluralNominal {
        element NegativeModifiedPluralNominal {
            modifiers: seq NegativeNominalModifier separated by ", ",
            head: PluralHead,
        }
        require len(modifiers) >= 2;
        derive agreement = head.agreement;
        derive number = head.number;
        derive onset = Values::Consonant;
        form negative_modified_plural_nominal = modifiers head;
    }
    construction bare_singular_coordination_member: SingularCoordinationMember {
        element BareSingularCoordinationMember { head: SingularHead, }
        derive agreement = head.agreement;
        derive number = head.number;
        derive onset = head.onset;
        form bare_singular_coordination_member = head;
    }
    construction modified_singular_coordination_member: SingularCoordinationMember {
        element ModifiedSingularCoordinationMember {
            modifier: CoordinatedNominalModifier,
            head: SingularHead,
        }
        derive agreement = head.agreement;
        derive number = head.number;
        derive onset = modifier.onset;
        form modified_singular_coordination_member = modifier head;
    }
    construction negative_modified_singular_coordination_member: SingularCoordinationMember {
        element NegativeModifiedSingularCoordinationMember {
            modifiers: seq NegativeNominalModifier separated by ", ",
            head: SingularHead,
        }
        require len(modifiers) >= 2;
        derive agreement = head.agreement;
        derive number = head.number;
        derive onset = Values::Consonant;
        form negative_modified_singular_coordination_member = modifiers head;
    }
    construction bare_plural_coordination_member: PluralCoordinationMember {
        element BarePluralCoordinationMember { head: PluralHead, }
        derive agreement = head.agreement;
        derive number = head.number;
        derive onset = head.onset;
        form bare_plural_coordination_member = head;
    }
    construction modified_plural_coordination_member: PluralCoordinationMember {
        element ModifiedPluralCoordinationMember {
            modifier: CoordinatedNominalModifier,
            head: PluralHead,
        }
        derive agreement = head.agreement;
        derive number = head.number;
        derive onset = modifier.onset;
        form modified_plural_coordination_member = modifier head;
    }
    construction negative_modified_plural_coordination_member: PluralCoordinationMember {
        element NegativeModifiedPluralCoordinationMember {
            modifiers: seq NegativeNominalModifier separated by ", ",
            head: PluralHead,
        }
        require len(modifiers) >= 2;
        derive agreement = head.agreement;
        derive number = head.number;
        derive onset = Values::Consonant;
        form negative_modified_plural_coordination_member = modifiers head;
    }
    construction singular_and_nominal_coordination: SingularNominalCoordination {
        element SingularAndNominalCoordination {
            members: seq SingularCoordinationMember separated by position {
                pair = " and ";
                first = ", ";
                middle = ", ";
                last = ", and ";
            },
        }
        require len(members) >= 2;
        form singular_and_nominal_coordination = members;
    }
    construction singular_or_nominal_coordination: SingularNominalCoordination {
        element SingularOrNominalCoordination {
            members: seq SingularCoordinationMember separated by position {
                pair = " or ";
                first = ", ";
                middle = ", ";
                last = ", or ";
            },
        }
        require len(members) >= 2;
        form singular_or_nominal_coordination = members;
    }
    construction singular_and_or_nominal_coordination: SingularNominalCoordination {
        element SingularAndOrNominalCoordination {
            members: seq SingularCoordinationMember separated by position {
                pair = " and/or ";
                first = ", ";
                middle = ", ";
                last = ", and/or ";
            },
        }
        require len(members) >= 2;
        form singular_and_or_nominal_coordination = members;
    }
    construction determiner_scoped_and_nominal_pair: DeterminerScopedNominalCoordination {
        element DeterminerScopedAndNominalPair {
            first: SingularCoordinationMember,
            second: SingularCoordinationMember,
        }
        derive onset = first.onset;
        form determiner_scoped_and_nominal_pair = first "and" second;
    }
    construction determiner_scoped_and_nominal_series: DeterminerScopedNominalCoordination {
        element DeterminerScopedAndNominalSeries {
            first: SingularCoordinationMember,
            middle: seq SingularCoordinationMember separated by ", ",
            last: SingularCoordinationMember,
        }
        require len(middle) >= 1;
        derive onset = first.onset;
        form determiner_scoped_and_nominal_series = first "," middle "," "and" last;
    }
    construction determiner_scoped_or_nominal_pair: DeterminerScopedNominalCoordination {
        element DeterminerScopedOrNominalPair {
            first: SingularCoordinationMember,
            second: SingularCoordinationMember,
        }
        derive onset = first.onset;
        form determiner_scoped_or_nominal_pair = first "or" second;
    }
    construction determiner_scoped_or_nominal_series: DeterminerScopedNominalCoordination {
        element DeterminerScopedOrNominalSeries {
            first: SingularCoordinationMember,
            middle: seq SingularCoordinationMember separated by ", ",
            last: SingularCoordinationMember,
        }
        require len(middle) >= 1;
        derive onset = first.onset;
        form determiner_scoped_or_nominal_series = first "," middle "," "or" last;
    }
    construction determiner_scoped_and_or_nominal_pair: DeterminerScopedNominalCoordination {
        element DeterminerScopedAndOrNominalPair {
            first: SingularCoordinationMember,
            second: SingularCoordinationMember,
        }
        derive onset = first.onset;
        form determiner_scoped_and_or_nominal_pair = first "and/or" second;
    }
    construction determiner_scoped_and_or_nominal_series: DeterminerScopedNominalCoordination {
        element DeterminerScopedAndOrNominalSeries {
            first: SingularCoordinationMember,
            middle: seq SingularCoordinationMember separated by ", ",
            last: SingularCoordinationMember,
        }
        require len(middle) >= 1;
        derive onset = first.onset;
        form determiner_scoped_and_or_nominal_series = first "," middle "," "and/or" last;
    }
    construction plural_and_nominal_coordination: PluralNominalCoordination {
        element PluralAndNominalCoordination {
            members: seq PluralCoordinationMember separated by position {
                pair = " and ";
                first = ", ";
                middle = ", ";
                last = ", and ";
            },
        }
        require len(members) >= 2;
        form plural_and_nominal_coordination = members;
    }
    construction plural_or_nominal_coordination: PluralNominalCoordination {
        element PluralOrNominalCoordination {
            members: seq PluralCoordinationMember separated by position {
                pair = " or ";
                first = ", ";
                middle = ", ";
                last = ", or ";
            },
        }
        require len(members) >= 2;
        form plural_or_nominal_coordination = members;
    }
    construction plural_and_or_nominal_coordination: PluralNominalCoordination {
        element PluralAndOrNominalCoordination {
            members: seq PluralCoordinationMember separated by position {
                pair = " and/or ";
                first = ", ";
                middle = ", ";
                last = ", and/or ";
            },
        }
        require len(members) >= 2;
        form plural_and_or_nominal_coordination = members;
    }
    construction unmarked_singular_selector: SingularSelector {
        element UnmarkedSingularSelector { nominal: SingularNominal, }
        derive agreement = nominal.agreement;
        derive number = nominal.number;
        derive onset = nominal.onset;
        form unmarked_singular_selector = nominal;
    }
    construction target_singular_selector: SingularSelector {
        element TargetSingularSelector { nominal: SingularNominal, }
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        form target_singular_selector = "target" nominal;
    }
    construction target_singular_coordination_selector: SingularSelector {
        element TargetSingularCoordinationSelector {
            coordination: SingularNominalCoordination,
        }
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        form target_singular_coordination_selector = "target" coordination;
    }
    construction other_singular_selector: SingularSelector {
        element OtherSingularSelector { nominal: SingularNominal, }
        derive agreement = nominal.agreement;
        derive number = nominal.number;
        derive onset = Values::Vowel;
        form other_singular_selector = "other" nominal;
    }
    construction other_target_singular_selector: SingularSelector {
        element OtherTargetSingularSelector { nominal: SingularNominal, }
        derive agreement = nominal.agreement;
        derive number = nominal.number;
        derive onset = Values::Vowel;
        form other_target_singular_selector = "other" "target" nominal;
    }
    construction unmarked_plural_selector: PluralSelector {
        element UnmarkedPluralSelector { nominal: PluralNominal, }
        derive agreement = nominal.agreement;
        derive number = nominal.number;
        derive onset = nominal.onset;
        form unmarked_plural_selector = nominal;
    }
    construction unmarked_plural_coordination_selector: PluralSelector {
        element UnmarkedPluralCoordinationSelector {
            coordination: PluralNominalCoordination,
        }
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive onset = Values::Consonant;
        form unmarked_plural_coordination_selector = coordination;
    }
    construction target_plural_selector: PluralSelector {
        element TargetPluralSelector { nominal: PluralNominal, }
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive onset = Values::Consonant;
        form target_plural_selector = "target" nominal;
    }
    construction target_plural_coordination_selector: PluralSelector {
        element TargetPluralCoordinationSelector {
            coordination: PluralNominalCoordination,
        }
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive onset = Values::Consonant;
        form target_plural_coordination_selector = "target" coordination;
    }
    construction other_plural_selector: PluralSelector {
        element OtherPluralSelector { nominal: PluralNominal, }
        derive agreement = nominal.agreement;
        derive number = nominal.number;
        derive onset = Values::Vowel;
        form other_plural_selector = "other" nominal;
    }
    construction other_target_plural_selector: PluralSelector {
        element OtherTargetPluralSelector { nominal: PluralNominal, }
        derive agreement = nominal.agreement;
        derive number = nominal.number;
        derive onset = Values::Vowel;
        form other_target_plural_selector = "other" "target" nominal;
    }
    construction indefinite_reference: UnqualifiedReference {
        element IndefiniteReference { nominal: SingularNominal, }
        derive agreement = nominal.agreement;
        derive number = nominal.number;
        derive onset = nominal.onset;
        form an when nominal.onset is Vowel = "an" nominal;
        form a otherwise = "a" nominal;
    }
    construction indefinite_coordination_reference: UnqualifiedReference {
        element IndefiniteCoordinationReference {
            coordination: DeterminerScopedNominalCoordination,
        }
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = coordination.onset;
        form an when coordination.onset is Vowel = "an" coordination;
        form a otherwise = "a" coordination;
    }
    construction named_card_reference: UnqualifiedReference {
        element NamedCardReference { name: identity CardName, }
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        form named_card_reference = "a" "card" "named" identity(name);
    }
    construction ordinary_singular_reference: UnqualifiedReference {
        element OrdinarySingularReference { phrase: DeterminerPhrase, }
        require any(
            phrase is TargetDeterminerPhrase,
            phrase is TargetCoordinationDeterminerPhrase
        );
        derive agreement = phrase.agreement;
        derive number = phrase.number;
        derive onset = phrase.onset;
        form ordinary_singular_reference = phrase;
    }
    construction ordinary_plural_reference: UnqualifiedReference {
        element OrdinaryPluralReference { selector: PluralSelector, }
        require any(
            selector is UnmarkedPluralSelector,
            selector is UnmarkedPluralCoordinationSelector,
            selector is OtherPluralSelector
        );
        derive agreement = selector.agreement;
        derive number = selector.number;
        derive onset = selector.onset;
        form ordinary_plural_reference = selector;
    }
    construction definite_singular_reference: UnqualifiedReference {
        element DefiniteSingularReference { selector: SingularSelector, }
        derive agreement = selector.agreement;
        derive number = selector.number;
        derive onset = Values::Consonant;
        form definite_singular_reference = "the" selector;
    }
    construction definite_plural_reference: UnqualifiedReference {
        element DefinitePluralReference { selector: PluralSelector, }
        derive agreement = selector.agreement;
        derive number = selector.number;
        derive onset = Values::Consonant;
        form definite_plural_reference = "the" selector;
    }
    construction any_target_reference: UnqualifiedReference {
        element AnyTargetReference {}
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Vowel;
        form any_target_reference = "any" "target";
    }
    construction another_reference: UnqualifiedReference {
        element AnotherReference { selector: SingularSelector, }
        require any(
            selector is UnmarkedSingularSelector,
            selector is TargetSingularSelector
        );
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Vowel;
        form another_reference = "another" selector;
    }
    construction another_coordination_reference: UnqualifiedReference {
        element AnotherCoordinationReference {
            coordination: SingularNominalCoordination,
        }
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Vowel;
        form another_coordination_reference = "another" coordination;
    }
    construction each_reference: UnqualifiedReference {
        element EachReference { selector: SingularSelector, }
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Vowel;
        form each_reference = "each" selector;
    }
    construction all_reference: UnqualifiedReference {
        element AllReference { selector: PluralSelector, }
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive onset = Values::Vowel;
        form all_reference = "all" selector;
    }
    construction fixed_reference: UnqualifiedReference {
        element FixedReference { count: CardinalQuantity, selector: PluralSelector, }
        require count.cardinality is TwoPlus;
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive onset = Values::Consonant;
        form fixed_reference = count selector;
    }
    construction variable_reference: UnqualifiedReference {
        element VariableReference { count: lex Variable, selector: PluralSelector, }
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive onset = count.onset;
        form variable_reference = lex(count) selector;
    }
    construction up_to_one_reference: UnqualifiedReference {
        element UpToOneReference { count: CardinalQuantity, selector: SingularSelector, }
        require count.cardinality is One;
        require any(
            selector is TargetSingularSelector,
            selector is TargetSingularCoordinationSelector,
            selector is OtherTargetSingularSelector
        );
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Vowel;
        form up_to_one_reference = "up" "to" count selector;
    }
    construction up_to_many_reference: UnqualifiedReference {
        element UpToManyReference { count: CardinalQuantity, selector: PluralSelector, }
        require count.cardinality is TwoPlus;
        require any(
            selector is TargetPluralSelector,
            selector is TargetPluralCoordinationSelector,
            selector is OtherTargetPluralSelector
        );
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive onset = Values::Vowel;
        form up_to_many_reference = "up" "to" count selector;
    }
    construction any_number_reference: UnqualifiedReference {
        element AnyNumberReference { selector: PluralSelector, }
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive onset = Values::Vowel;
        form any_number_reference = "any" "number" "of" selector;
    }
    construction one_or_more_reference: UnqualifiedReference {
        element OneOrMoreReference { selector: PluralSelector, }
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive onset = Values::Consonant;
        form one_or_more_reference = "one" "or" "more" selector;
    }
    construction that_many: CountReference {
        element ThatMany {}
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive onset = Values::Consonant;
        form that_many = "that" "many";
    }
    construction counted_reference: UnqualifiedReference {
        element CountedReference {
            count: CountReference,
            selector: PluralSelector,
        }
        derive agreement = count.agreement;
        derive number = count.number;
        derive onset = count.onset;
        form counted_reference = count selector;
    }
    construction this_reference: UnqualifiedReference {
        element ThisReference { nominal: SingularNominal, }
        derive agreement = nominal.agreement;
        derive number = nominal.number;
        derive onset = Values::Consonant;
        form this_reference = "this" nominal;
    }
    construction that_reference: UnqualifiedReference {
        element ThatReference { nominal: SingularNominal, }
        derive agreement = nominal.agreement;
        derive number = nominal.number;
        derive onset = Values::Consonant;
        form that_reference = "that" nominal;
    }
    construction demonstrative_possessive_reference: UnqualifiedReference {
        element DemonstrativePossessiveReference {
            demonstrative: lex SingularDemonstrative,
            possessor: SingularNominal,
            possessed: SingularNominal,
        }
        derive agreement = possessed.agreement;
        derive number = possessed.number;
        derive onset = Values::Consonant;
        form demonstrative_possessive_reference = lex(demonstrative) suffix(possessor, "'s") possessed;
    }
    construction those_reference: UnqualifiedReference {
        element ThoseReference { nominal: PluralNominal, }
        derive agreement = nominal.agreement;
        derive number = nominal.number;
        derive onset = Values::Consonant;
        form those_reference = "those" nominal;
    }
    construction designated_singular_reference: UnqualifiedReference {
        element DesignatedSingularReference { designation: lex Designation, nominal: SingularNominal, }
        derive agreement = nominal.agreement;
        derive number = nominal.number;
        derive onset = Values::Consonant;
        form designated_singular_reference = "the" lex(designation) nominal;
    }
    construction designated_plural_reference: UnqualifiedReference {
        element DesignatedPluralReference { designation: lex Designation, nominal: PluralNominal, }
        derive agreement = nominal.agreement;
        derive number = nominal.number;
        derive onset = Values::Consonant;
        form designated_plural_reference = "the" lex(designation) nominal;
    }
    construction chosen_quality_reference: UnqualifiedReference {
        element ChosenQualityReference { quality: lex ChosenQuality, }
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        form chosen_quality_reference = "the" "chosen" lex(quality);
    }
    construction possessed_singular_reference: UnqualifiedReference {
        element PossessedSingularReference {
            possessor: lex PossessiveDeterminerPronoun,
            nominal: SingularNominal,
        }
        derive agreement = nominal.agreement;
        derive number = nominal.number;
        derive onset = possessor.onset;
        form possessed_singular_reference = lex(possessor) nominal;
    }
    construction possessed_plural_reference: UnqualifiedReference {
        element PossessedPluralReference {
            possessor: lex PossessiveDeterminerPronoun,
            nominal: PluralNominal,
        }
        derive agreement = nominal.agreement;
        derive number = nominal.number;
        derive onset = possessor.onset;
        form possessed_plural_reference = lex(possessor) nominal;
    }
    construction possessive_absolute_reference: UnqualifiedReference {
        element PossessiveAbsoluteReference { word: lex PossessiveAbsolutePronoun, }
        derive agreement = match word {
            Hers => Values::ThirdPersonSingular,
            His => Values::ThirdPersonSingular,
            Theirs => Values::Bare,
            Yours => Values::Bare,
        };
        derive number = match word {
            Hers => Values::Singular,
            His => Values::Singular,
            Theirs => Values::Singular,
            Yours => Values::Singular,
        };
        derive onset = word.onset;
        form possessive_absolute_reference = lex(word);
    }
    construction target_determiner_phrase: DeterminerPhrase {
        element TargetDeterminerPhrase { nominal: SingularNominal, }
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        form target_determiner_phrase = "target" nominal;
    }
    construction target_coordination_determiner_phrase: DeterminerPhrase {
        element TargetCoordinationDeterminerPhrase {
            coordination: SingularNominalCoordination,
        }
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        form target_coordination_determiner_phrase = "target" coordination;
    }
    construction indefinite_determiner_phrase: DeterminerPhrase {
        element IndefiniteDeterminerPhrase { nominal: SingularNominal, }
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = nominal.onset;
        form an when nominal.onset is Vowel = "an" nominal;
        form a otherwise = "a" nominal;
    }
    construction this_determiner_phrase: DeterminerPhrase {
        element ThisDeterminerPhrase { nominal: SingularNominal, }
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        form this_determiner_phrase = "this" nominal;
    }
    construction that_determiner_phrase: DeterminerPhrase {
        element ThatDeterminerPhrase { nominal: SingularNominal, }
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        form that_determiner_phrase = "that" nominal;
    }
    construction another_determiner_phrase: DeterminerPhrase {
        element AnotherDeterminerPhrase { selector: SingularSelector, }
        require any(
            selector is UnmarkedSingularSelector,
            selector is TargetSingularSelector
        );
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Vowel;
        form another_determiner_phrase = "another" selector;
    }
    construction full_and_noun_phrase_coordination: FullNounPhraseCoordination {
        element FullAndNounPhraseCoordination {
            members: seq DeterminerPhrase separated by position {
                pair = " and ";
                first = ", ";
                middle = ", ";
                last = ", and ";
            },
        }
        require len(members) >= 2;
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        form full_and_noun_phrase_coordination = members;
    }
    construction full_or_noun_phrase_coordination: FullNounPhraseCoordination {
        element FullOrNounPhraseCoordination {
            members: seq DeterminerPhrase separated by position {
                pair = " or ";
                first = ", ";
                middle = ", ";
                last = ", or ";
            },
        }
        require len(members) >= 2;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        form full_or_noun_phrase_coordination = members;
    }
    construction full_and_or_noun_phrase_coordination: FullNounPhraseCoordination {
        element FullAndOrNounPhraseCoordination {
            members: seq DeterminerPhrase separated by position {
                pair = " and/or ";
                first = ", ";
                middle = ", ";
                last = ", and/or ";
            },
        }
        require len(members) >= 2;
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        form full_and_or_noun_phrase_coordination = members;
    }
    construction coordinated_noun_phrase: UnqualifiedReference {
        element CoordinatedNounPhrase { coordination: FullNounPhraseCoordination, }
        derive agreement = coordination.agreement;
        derive number = coordination.number;
        derive onset = Values::Consonant;
        form coordinated_noun_phrase = coordination;
    }
    construction self_reference: UnqualifiedReference {
        element SourceSelfReference { spelling: identity SelfReferenceSpelling, }
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = spelling.onset;
        form self_reference = identity(spelling);
    }
    construction this_way: MannerReference {
        element ThisWay {}
        form this_way = "this" "way";
    }
    construction that_much: ScalarReference {
        element ThatMuch {}
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        form that_much = "that" "much";
    }
    construction you_control: ControllerOwnerQualification {
        element YouControl { controller: lex SubjectPronoun, }
        require controller is You;
        derive verb.agreement = Values::Bare;
        form you_control = lex(controller) verb(VerbLexeme::Control);
    }
    construction opponent_controller: SingularController {
        element OpponentController { controller: lex ControllerNoun, }
        require controller is Opponent;
        derive onset = controller.onset;
        form an when controller.onset is Vowel = "an" lex(controller);
        form a otherwise = "a" lex(controller);
    }
    construction opponent_controls: ControllerOwnerQualification {
        element OpponentControls { controller: SingularController, }
        derive verb.agreement = Values::ThirdPersonSingular;
        form opponent_controls = controller verb(VerbLexeme::Control);
    }
    construction demonstrative_controls: ControllerOwnerQualification {
        element DemonstrativeControls { controller: DeterminerPhrase, }
        require controller is ThatDeterminerPhrase;
        derive verb.agreement = Values::ThirdPersonSingular;
        form demonstrative_controls = controller verb(VerbLexeme::Control);
    }
    construction you_own: ControllerOwnerQualification {
        element YouOwn { owner: lex SubjectPronoun, }
        require owner is You;
        derive verb.agreement = Values::Bare;
        form you_own = lex(owner) verb(VerbLexeme::Own);
    }
    construction possessed_zone: ZoneReference {
        element PossessedZone {
            possessor: lex PossessiveDeterminerPronoun,
            zone: lex Zone,
        }
        form possessed_zone = lex(possessor) lex(zone);
    }
    construction unpossessed_zone: ZoneReference {
        element UnpossessedZone { zone: lex Zone, }
        require zone is Exile;
        form unpossessed_zone = lex(zone);
    }
    construction singular_owner_possessor: OwnerPossessor {
        element SingularOwnerPossessor { possessor: lex PossessiveDeterminerPronoun, }
        form singular_owner_possessor = lex(possessor) "owner's";
    }
    construction plural_owner_possessor: OwnerPossessor {
        element PluralOwnerPossessor { possessor: lex PossessiveDeterminerPronoun, }
        require possessor is Their;
        form plural_owner_possessor = lex(possessor) "owners'";
    }
    construction owner_possessed_zone: ZoneReference {
        element OwnerPossessedZone {
            owner: OwnerPossessor,
            zone: lex Zone,
        }
        form owner_possessed_zone = owner lex(zone);
    }
    construction definite_zone: ZoneReference {
        element DefiniteZone { zone: lex Zone, }
        require zone is Battlefield;
        form definite_zone = "the" lex(zone);
    }
    construction possessed_library: LibraryReference {
        element PossessedLibrary {
            possessor: lex PossessiveDeterminerPronoun,
        }
        form possessed_library = lex(possessor) "library";
    }
    construction owner_possessed_library: LibraryReference {
        element OwnerPossessedLibrary { owner: OwnerPossessor, }
        form owner_possessed_library = owner "library";
    }
    construction singular_library_card_quantity: LibraryCardQuantity {
        element SingularLibraryCardQuantity {}
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        form singular_library_card_quantity = "card";
    }
    construction fixed_library_card_quantity: LibraryCardQuantity {
        element FixedLibraryCardQuantity { count: CardinalQuantity, }
        require count.cardinality is TwoPlus;
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        form fixed_library_card_quantity = count "cards";
    }
    construction from_source: FromSource {
        element FromSourceValue { zone: ZoneReference, }
        form from_source = "from" zone;
    }
    construction into_destination: IntoDestination {
        element IntoDestinationValue { zone: ZoneReference, }
        require any(
            zone is PossessedZone,
            zone is OwnerPossessedZone,
            zone is UnpossessedZone
        );
        form into_destination = "into" zone;
    }
    construction onto_battlefield_destination: OntoDestination {
        element OntoBattlefieldDestination { battlefield: ZoneReference, }
        require battlefield is DefiniteZone;
        form onto_battlefield_destination = "onto" battlefield;
    }
    construction on_library_destination: OnDestination {
        element OnLibraryDestination {
            position: lex LibraryPosition,
            library: LibraryReference,
        }
        form top when position is Top = "on" lex(position) "of" library;
        form bottom otherwise = "on" "the" lex(position) "of" library;
    }
    construction to_destination: ToDestination {
        element ToDestinationValue { zone: ZoneReference, }
        form to_destination = "to" zone;
    }
    construction tapped_post_state: PostState {
        element TappedPostState {}
        form tapped_post_state = "tapped";
    }
    construction direct_control_postmodifier: ControlPostmodifier {
        element DirectControlPostmodifier {
            controller: lex PossessiveDeterminerPronoun,
        }
        form direct_control_postmodifier = "under" lex(controller) "control";
    }
    construction owner_control_postmodifier: ControlPostmodifier {
        element OwnerControlPostmodifier { owner: OwnerPossessor, }
        form owner_control_postmodifier = "under" owner "control";
    }
    construction zone_location: ZoneLocation {
        element ZoneLocationValue { zone: ZoneReference, }
        form zone_location = zone;
    }
    construction at_location: AtLocation {
        element AtLocationValue { object: Object, }
        form at_location = "at" object;
    }
    construction in_zone: ZoneQualification {
        element InZone { zone: ZoneReference, }
        form in_zone = "in" zone;
    }
    construction from_zone: ZoneQualification {
        element FromZone { zone: ZoneReference, }
        form from_zone = "from" zone;
    }
    construction fixed_scalar_threshold: ScalarThreshold {
        element FixedScalarThreshold { value: lex ScalarNumber, }
        form fixed_scalar_threshold = lex(value);
    }
    construction variable_scalar_threshold: ScalarThreshold {
        element VariableScalarThreshold { value: lex Variable, }
        form variable_scalar_threshold = lex(value);
    }
    construction characteristic_scalar: ScalarMeasure {
        element CharacteristicScalar { characteristic: lex ScalarCharacteristic, }
        form characteristic_scalar = lex(characteristic);
    }
    construction mana_value_scalar: ScalarMeasure {
        element ManaValueScalar {}
        form mana_value_scalar = "mana" "value";
    }
    construction scalar_or_less: ScalarComparison {
        element ScalarOrLess { threshold: ScalarThreshold, }
        form scalar_or_less = threshold "or" "less";
    }
    construction scalar_or_greater: ScalarComparison {
        element ScalarOrGreater { threshold: ScalarThreshold, }
        form scalar_or_greater = threshold "or" "greater";
    }
    construction scalar_less_than: ScalarComparison {
        element ScalarLessThan { threshold: ScalarThreshold, }
        form scalar_less_than = "less" "than" threshold;
    }
    construction scalar_greater_than: ScalarComparison {
        element ScalarGreaterThan { threshold: ScalarThreshold, }
        form scalar_greater_than = "greater" "than" threshold;
    }
    construction scalar_less_than_or_equal_to: ScalarComparison {
        element ScalarLessThanOrEqualTo { threshold: ScalarThreshold, }
        form scalar_less_than_or_equal_to = "less" "than" "or" "equal" "to" threshold;
    }
    construction count_or_more: CountComparison {
        element CountOrMore {}
        form count_or_more = "or" "more";
    }
    construction count_or_fewer: CountComparison {
        element CountOrFewer {}
        form count_or_fewer = "or" "fewer";
    }
    construction scalar_qualification: ScalarQualification {
        element ScalarQualificationValue {
            measure: ScalarMeasure,
            comparison: ScalarComparison,
        }
        form scalar_qualification = "with" measure comparison;
    }
    construction possessed_scalar_value: ScalarValue {
        element PossessedScalarValue {
            possessor: lex PossessiveDeterminerPronoun,
            measure: ScalarMeasure,
        }
        form possessed_scalar_value = lex(possessor) measure;
    }
    construction scalar_equality: ScalarEquality {
        element ScalarEqualityValue { value: ScalarValue, }
        form scalar_equality = "equal" "to" value;
    }
    construction unqualified_controller_stage: ControllerStage {
        element UnqualifiedControllerStage { reference: UnqualifiedReference, }
        derive agreement = reference.agreement;
        derive number = reference.number;
        derive onset = reference.onset;
        form unqualified_controller_stage = reference;
    }
    construction controller_qualified_reference: ControllerStage {
        element ControllerQualifiedReference {
            reference: UnqualifiedReference,
            controller_owner: ControllerOwnerQualification,
        }
        derive agreement = reference.agreement;
        derive number = reference.number;
        derive onset = reference.onset;
        form controller_qualified_reference = reference controller_owner;
    }
    construction other_than_qualified_reference: ControllerStage {
        element OtherThanQualifiedReference {
            reference: UnqualifiedReference,
            excluded: UnqualifiedReference,
        }
        derive agreement = reference.agreement;
        derive number = reference.number;
        derive onset = reference.onset;
        form other_than_qualified_reference = reference "other" "than" excluded;
    }
    construction unqualified_zone_stage: ZoneStage {
        element UnqualifiedZoneStage { reference: ControllerStage, }
        derive agreement = reference.agreement;
        derive number = reference.number;
        derive onset = reference.onset;
        form unqualified_zone_stage = reference;
    }
    construction zone_qualified_reference: ZoneStage {
        element ZoneQualifiedReference {
            reference: ControllerStage,
            zone: ZoneQualification,
        }
        derive agreement = reference.agreement;
        derive number = reference.number;
        derive onset = reference.onset;
        form zone_qualified_reference = reference zone;
    }
    construction unqualified_numeric_stage: NumericStage {
        element UnqualifiedNumericStage { reference: ZoneStage, }
        derive agreement = reference.agreement;
        derive number = reference.number;
        derive onset = reference.onset;
        form unqualified_numeric_stage = reference;
    }
    construction scalar_qualified_reference: NumericStage {
        element ScalarQualifiedReference {
            reference: ZoneStage,
            scalar: ScalarQualification,
        }
        derive agreement = reference.agreement;
        derive number = reference.number;
        derive onset = reference.onset;
        form scalar_qualified_reference = reference scalar;
    }
    construction count_comparison_reference: UnqualifiedReference {
        element CountComparisonReference {
            count: CardinalQuantity,
            comparison: CountComparison,
            selector: PluralSelector,
        }
        require count.cardinality is TwoPlus;
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive onset = Values::Consonant;
        form count_comparison_reference = count comparison selector;
    }
    construction qualified_noun_phrase: NounPhrase {
        element QualifiedNounPhrase { reference: NumericStage, }
        derive agreement = reference.agreement;
        derive number = reference.number;
        derive onset = reference.onset;
        form qualified_noun_phrase = reference;
    }
    construction library_slice: NounPhrase {
        element LibrarySlice {
            position: lex LibraryPosition,
            cards: LibraryCardQuantity,
            library: LibraryReference,
        }
        derive agreement = cards.agreement;
        derive number = cards.number;
        derive onset = Values::Consonant;
        form library_slice = "the" lex(position) cards "of" library;
    }
    construction possessive_self_reference: PossessiveOwner {
        element PossessiveSelfReference { spelling: identity SelfReferenceSpelling, }
        derive number = Values::Singular;
        derive possessive_ending = spelling.possessive_ending;
        form possessive_self_reference = identity(spelling);
    }
    construction possessive_plural_noun: PossessiveOwner {
        element PossessiveNoun { head: PluralHead, }
        derive number = Values::Plural;
        derive possessive_ending = head.possessive_ending;
        form possessive_plural_noun = head;
    }
    construction possessive: Possessive {
        element PossessiveValue { owner: PossessiveOwner, }
        derive number = owner.number;
        form singular when number is Singular = suffix(owner, "'s");
        form plural_s when all(
            number is Plural,
            owner.possessive_ending is EndsInS
        ) = suffix(owner, "'");
        form plural_other otherwise = suffix(owner, "'s");
    }
    construction singular_card_quantity: CardQuantity {
        element SingularCardQuantity {}
        form singular_card_quantity = "a" "card";
    }
    construction fixed_card_quantity: CardQuantity {
        element FixedCardQuantity { count: CardinalQuantity, }
        require count.cardinality is TwoPlus;
        form fixed_card_quantity = count "cards";
    }
    construction variable_card_quantity: CardQuantity {
        element VariableCardQuantity { count: lex Variable, }
        form variable_card_quantity = lex(count) "cards";
    }
    construction anaphoric_card_quantity: CardQuantity {
        element AnaphoricCardQuantity { count: CountReference, }
        form anaphoric_card_quantity = count "cards";
    }
    construction compared_card_quantity: CardQuantity {
        element ComparedCardQuantity {
            count: CardinalQuantity,
            comparison: CountComparison,
        }
        require count.cardinality is TwoPlus;
        form compared_card_quantity = count comparison "cards";
    }
    construction positive_power_toughness_counter: CounterKind {
        element PositivePowerToughnessCounter {
            magnitudes: seq PositiveCounterMagnitude separated by "/",
        }
        require len(magnitudes) = 2;
        form positive_power_toughness_counter = magnitudes;
    }
    construction negative_power_toughness_counter: CounterKind {
        element NegativePowerToughnessCounter {
            magnitudes: seq NegativeCounterMagnitude separated by "/",
        }
        require len(magnitudes) = 2;
        form negative_power_toughness_counter = magnitudes;
    }
    construction positive_counter_magnitude: PositiveCounterMagnitude {
        element PositiveCounterMagnitudeValue { amount: Amount, }
        form positive_counter_magnitude = prefix("+", amount);
    }
    construction negative_counter_magnitude: NegativeCounterMagnitude {
        element NegativeCounterMagnitudeValue { amount: Amount, }
        form negative_counter_magnitude = prefix("-", amount);
    }
    construction named_counter: CounterKind {
        element NamedCounter { name: lex CounterName, }
        form named_counter = lex(name);
    }
    construction singular_counter_quantity: CounterQuantity {
        element SingularCounterQuantity { kind: CounterKind, }
        form singular_counter_quantity = "a" kind "counter";
    }
    construction fixed_counter_quantity: CounterQuantity {
        element FixedCounterQuantity {
            count: CardinalQuantity,
            kind: CounterKind,
        }
        require count.cardinality is TwoPlus;
        form fixed_counter_quantity = count kind "counters";
    }
    construction variable_counter_quantity: CounterQuantity {
        element VariableCounterQuantity {
            count: lex Variable,
            kind: CounterKind,
        }
        form variable_counter_quantity = lex(count) kind "counters";
    }
    construction anaphoric_counter_quantity: CounterQuantity {
        element AnaphoricCounterQuantity {
            count: CountReference,
            kind: CounterKind,
        }
        form anaphoric_counter_quantity = count kind "counters";
    }
    construction singular_die_object: DieObject {
        element SingularDieObject { shape: opt lex DieShape, }
        form singular_die_object = "a" lex(shape) "die";
    }
    construction fixed_dice_object: DieObject {
        element FixedDiceObject {
            count: CardinalQuantity,
            shape: opt lex DieShape,
        }
        require count.cardinality is TwoPlus;
        form fixed_dice_object = count lex(shape) "dice";
    }
    construction d20_object: DieObject {
        element D20Object {}
        form d20_object = "a" "d20";
    }
    construction intransitive_predicate: VerbPhrase {
        element IntransitivePredicate { head: lex IntransitiveVerb, }
        derive agreement = head.agreement;
        form intransitive_predicate = verb(head);
    }
    construction transitive_predicate: VerbPhrase {
        element TransitivePredicate { head: lex TransitiveVerb, object: Object, }
        derive agreement = head.agreement;
        form transitive_predicate = verb(head) object;
    }
    construction numerative_predicate: VerbPhrase {
        element NumerativePredicate { head: lex NumerativeVerb, amount: Amount, }
        derive agreement = head.agreement;
        form numerative_predicate = verb(head) amount;
    }
    construction damage_recipient: DamageRecipient {
        element DamageRecipientValue { object: Object, }
        form damage_recipient = "to" object;
    }
    construction counter_recipient: CounterRecipient {
        element CounterRecipientValue { object: Object, }
        form counter_recipient = "on" object;
    }
    construction counter_source: CounterSource {
        element CounterSourceValue { object: Object, }
        form counter_source = "from" object;
    }
    construction deal_damage: VerbPhrase {
        element DealDamage { amount: Amount, recipient: DamageRecipient, }
        derive agreement = verb.agreement;
        form deal_damage = verb(VerbLexeme::Deal) amount "damage" recipient;
    }
    construction gain_life: VerbPhrase {
        element GainLife { amount: Amount, }
        derive agreement = verb.agreement;
        form gain_life = verb(VerbLexeme::Gain) amount "life";
    }
    construction deal_damage_equal_to: VerbPhrase {
        element DealDamageEqualTo {
            equality: ScalarEquality,
            recipient: DamageRecipient,
        }
        derive agreement = verb.agreement;
        form deal_damage_equal_to = verb(VerbLexeme::Deal) "damage" equality recipient;
    }
    construction gain_life_equal_to: VerbPhrase {
        element GainLifeEqualTo { equality: ScalarEquality, }
        derive agreement = verb.agreement;
        form gain_life_equal_to = verb(VerbLexeme::Gain) "life" equality;
    }
    construction lose_life: VerbPhrase {
        element LoseLife { amount: Amount, }
        derive agreement = verb.agreement;
        form lose_life = verb(VerbLexeme::Lose) amount "life";
    }
    construction lose_life_equal_to: VerbPhrase {
        element LoseLifeEqualTo { equality: ScalarEquality, }
        derive agreement = verb.agreement;
        form lose_life_equal_to = verb(VerbLexeme::Lose) "life" equality;
    }
    construction pay_life: VerbPhrase {
        element PayLife { amount: Amount, }
        derive agreement = verb.agreement;
        form pay_life = verb(VerbLexeme::Pay) amount "life";
    }
    construction pay_mana: VerbPhrase {
        element PayMana { mana: ManaAmount, }
        derive agreement = verb.agreement;
        form pay_mana = verb(VerbLexeme::Pay) mana;
    }
    construction add_mana: VerbPhrase {
        element AddMana { mana: ManaAmount, }
        derive agreement = verb.agreement;
        form add_mana = verb(VerbLexeme::Add) mana;
    }
    construction draw_cards: VerbPhrase {
        element DrawCards { cards: CardQuantity, }
        derive agreement = verb.agreement;
        form draw_cards = verb(VerbLexeme::Draw) cards;
    }
    construction draw_cards_equal_to: VerbPhrase {
        element DrawCardsEqualTo { equality: ScalarEquality, }
        derive agreement = verb.agreement;
        form draw_cards_equal_to = verb(VerbLexeme::Draw) "cards" equality;
    }
    construction roll_dice: VerbPhrase {
        element RollDice { dice: DieObject, }
        derive agreement = verb.agreement;
        form roll_dice = verb(VerbLexeme::Roll) dice;
    }
    construction put_counters: VerbPhrase {
        element PutCounters {
            counters: CounterQuantity,
            recipient: CounterRecipient,
        }
        derive agreement = verb.agreement;
        form put_counters = verb(VerbLexeme::Put) counters recipient;
    }
    construction remove_counters: VerbPhrase {
        element RemoveCounters {
            counters: CounterQuantity,
            source: CounterSource,
        }
        derive agreement = verb.agreement;
        form remove_counters = verb(VerbLexeme::Remove) counters source;
    }
    construction put_into: VerbPhrase {
        element PutInto {
            object: Object,
            source: opt FromSource,
            destination: IntoDestination,
        }
        derive agreement = verb.agreement;
        form put_into = verb(VerbLexeme::Put) object source destination;
    }
    construction put_onto: VerbPhrase {
        element PutOnto {
            object: Object,
            source: opt FromSource,
            destination: OntoDestination,
            post_state: opt PostState,
            control: opt ControlPostmodifier,
        }
        derive agreement = verb.agreement;
        form put_onto = verb(VerbLexeme::Put) object source destination post_state control;
    }
    construction put_on: VerbPhrase {
        element PutOn {
            object: Object,
            source: opt FromSource,
            destination: OnDestination,
        }
        derive agreement = verb.agreement;
        form put_on = verb(VerbLexeme::Put) object source destination;
    }
    construction put_to: VerbPhrase {
        element PutTo {
            object: Object,
            destination: ToDestination,
        }
        derive agreement = verb.agreement;
        form put_to = verb(VerbLexeme::Put) object destination;
    }
    construction return_to: VerbPhrase {
        element ReturnTo {
            object: Object,
            source: opt FromSource,
            destination: ToDestination,
            post_state: opt PostState,
            control: opt ControlPostmodifier,
        }
        derive agreement = verb.agreement;
        form return_to = verb(VerbLexeme::Return) object source destination post_state control;
    }
    construction enter_post_state: VerbPhrase {
        element EnterPostState { post_state: PostState, }
        derive agreement = verb.agreement;
        form enter_post_state = verb(VerbLexeme::Enter) post_state;
    }
    construction enter_location: VerbPhrase {
        element EnterLocation {
            location: ZoneLocation,
            post_state: opt PostState,
            control: opt ControlPostmodifier,
        }
        derive agreement = verb.agreement;
        form enter_location = verb(VerbLexeme::Enter) location post_state control;
    }
    construction enter_control: VerbPhrase {
        element EnterControl { control: ControlPostmodifier, }
        derive agreement = verb.agreement;
        form enter_control = verb(VerbLexeme::Enter) control;
    }
    construction leave_location: VerbPhrase {
        element LeaveLocation { location: ZoneLocation, }
        derive agreement = verb.agreement;
        form leave_location = verb(VerbLexeme::Leave) location;
    }
    construction look_at: VerbPhrase {
        element LookAt { location: AtLocation, }
        derive agreement = verb.agreement;
        form look_at = verb(VerbLexeme::Look) location;
    }
    construction search_for: VerbPhrase {
        element SearchFor {
            head: lex SearchForVerb,
            location: LibraryReference,
            sought: Object,
        }
        derive agreement = head.agreement;
        form search_for = verb(head) location "for" sought;
    }
    construction have_cards_in_hand: VerbPhrase {
        element HaveCardsInHand { cards: CardQuantity, }
        derive agreement = verb.agreement;
        form have_cards_in_hand = verb(VerbLexeme::Have) cards "in" "hand";
    }
    construction have_life: VerbPhrase {
        element HaveLife { comparison: ScalarComparison, }
        derive agreement = verb.agreement;
        form have_life = verb(VerbLexeme::Have) comparison "life";
    }
    construction have_no_maximum_hand_size: VerbPhrase {
        element HaveNoMaximumHandSize {}
        derive agreement = verb.agreement;
        form have_no_maximum_hand_size = verb(VerbLexeme::Have)
            "no" "maximum" "hand" "size";
    }
    construction have_object_control: VerbPhrase {
        element HaveObjectControl {
            object: Object,
            predicate: VerbPhrase,
        }
        derive agreement = verb.agreement;
        derive predicate.agreement = Values::Bare;
        form have_object_control = verb(VerbLexeme::Have) object predicate;
    }
    construction mana_amount: ManaAmount {
        element ManaAmountValue { run: ActivationCostComponent, }
        require run is SymbolRun;
        form mana_amount = run;
    }
    construction number: Amount {
        element NumberAmount { number: lex ScalarNumber, }
        form number = lex(number);
    }
    construction variable: Amount {
        element VariableAmount { variable: lex Variable, }
        form variable = lex(variable);
    }
    construction scalar_reference_amount: Amount {
        element ScalarReferenceAmount { reference: ScalarReference, }
        form scalar_reference_amount = reference;
    }
    construction cardinal: CardinalQuantity {
        element CardinalQuantityValue { number: lex CardinalNumber, }
        derive cardinality = number.cardinality;
        derive number = number.number;
        form cardinal = lex(number);
    }

    abstract sum DocumentBlock { Ability, }
    vocab ReflexiveSubordinateKind { IfYouDo = "if you do", WhenYouDo = "when you do", }
    abstract product OracleText {
        blocks: seq DocumentBlock separated by "\n",
    }

    root Ability { eoi = true; standalone_render = true; }
    root Sentence { punctuation = "."; eoi = false; standalone_render = true; }
    root Possessive { eoi = true; standalone_render = true; }
    root CardinalQuantity { eoi = true; standalone_render = true; }
    root MannerReference { eoi = true; standalone_render = true; }
    root CountReference { eoi = true; standalone_render = true; }
    root ScalarReference { eoi = true; standalone_render = true; }
    root OracleText { eoi = true; standalone_render = true; }
}

#[cfg(test)]
mod task9_feature_tests {
    use super::*;

    fn noun_phrase(reference: UnqualifiedReference) -> NounPhrase {
        NounPhrase::QualifiedNounPhrase(QualifiedNounPhrase {
            reference: NumericStage::UnqualifiedNumericStage(UnqualifiedNumericStage {
                reference: ZoneStage::UnqualifiedZoneStage(UnqualifiedZoneStage {
                    reference: ControllerStage::UnqualifiedControllerStage(
                        UnqualifiedControllerStage { reference },
                    ),
                }),
            }),
        })
    }

    fn singular_member(noun: CommonNoun) -> SingularCoordinationMember {
        SingularCoordinationMember::BareSingularCoordinationMember(BareSingularCoordinationMember {
            head: SingularHead::CommonSingularHead(CommonSingularHead { noun }),
        })
    }

    fn plural_member(noun: CommonNoun) -> PluralCoordinationMember {
        PluralCoordinationMember::BarePluralCoordinationMember(BarePluralCoordinationMember {
            head: PluralHead::CommonPluralHead(CommonPluralHead { noun }),
        })
    }

    fn singular_nominal(noun: CommonNoun) -> SingularNominal {
        SingularNominal::BareSingularNominal(BareSingularNominal {
            head: SingularHead::CommonSingularHead(CommonSingularHead { noun }),
        })
    }

    #[test]
    fn coordination_scopes_derive_exact_number_and_agreement() {
        let singular_shared = noun_phrase(UnqualifiedReference::OrdinarySingularReference(
            OrdinarySingularReference {
                phrase: DeterminerPhrase::TargetCoordinationDeterminerPhrase(
                    TargetCoordinationDeterminerPhrase {
                        coordination: SingularNominalCoordination::SingularOrNominalCoordination(
                            SingularOrNominalCoordination::new(vec![
                                singular_member(CommonNoun::Player),
                                singular_member(CommonNoun::Opponent),
                            ])
                            .expect("binary singular coordination satisfies minimum arity"),
                        ),
                    },
                ),
            },
        ));
        let plural_shared = noun_phrase(UnqualifiedReference::OrdinaryPluralReference(
            OrdinaryPluralReference {
                selector: PluralSelector::TargetPluralCoordinationSelector(
                    TargetPluralCoordinationSelector {
                        coordination: PluralNominalCoordination::PluralOrNominalCoordination(
                            PluralOrNominalCoordination::new(vec![
                                plural_member(CommonNoun::Player),
                                plural_member(CommonNoun::Opponent),
                            ])
                            .expect("binary plural coordination satisfies minimum arity"),
                        ),
                    },
                ),
            },
        ));
        let full_np = noun_phrase(UnqualifiedReference::CoordinatedNounPhrase(
            CoordinatedNounPhrase {
                coordination: FullNounPhraseCoordination::FullAndNounPhraseCoordination(
                    FullAndNounPhraseCoordination::new(vec![
                        DeterminerPhrase::TargetDeterminerPhrase(TargetDeterminerPhrase {
                            nominal: singular_nominal(CommonNoun::Player),
                        }),
                        DeterminerPhrase::TargetDeterminerPhrase(TargetDeterminerPhrase {
                            nominal: singular_nominal(CommonNoun::Opponent),
                        }),
                    ])
                    .expect("binary full-NP coordination satisfies minimum arity"),
                ),
            },
        ));

        assert_eq!(number_for_noun_phrase(&singular_shared), Number::Singular);
        assert_eq!(
            agreement_for_noun_phrase(&singular_shared),
            Agreement::ThirdPersonSingular
        );
        assert_eq!(number_for_noun_phrase(&plural_shared), Number::Plural);
        assert_eq!(agreement_for_noun_phrase(&plural_shared), Agreement::Bare);
        assert_eq!(number_for_noun_phrase(&full_np), Number::Plural);
        assert_eq!(agreement_for_noun_phrase(&full_np), Agreement::Bare);
    }
}

#[cfg(test)]
mod task10_feature_tests {
    use super::*;

    fn noun_phrase(reference: UnqualifiedReference) -> NounPhrase {
        NounPhrase::QualifiedNounPhrase(QualifiedNounPhrase {
            reference: NumericStage::UnqualifiedNumericStage(UnqualifiedNumericStage {
                reference: ZoneStage::UnqualifiedZoneStage(UnqualifiedZoneStage {
                    reference: ControllerStage::UnqualifiedControllerStage(
                        UnqualifiedControllerStage { reference },
                    ),
                }),
            }),
        })
    }

    fn variable_reference(count: Variable) -> NounPhrase {
        noun_phrase(UnqualifiedReference::VariableReference(VariableReference {
            count,
            selector: PluralSelector::UnmarkedPluralSelector(UnmarkedPluralSelector {
                nominal: PluralNominal::BarePluralNominal(BarePluralNominal {
                    head: PluralHead::CommonPluralHead(CommonPluralHead {
                        noun: CommonNoun::Player,
                    }),
                }),
            }),
        }))
    }

    #[test]
    fn variable_reference_onset_is_derived_from_its_count_lexeme() {
        let context = ParseContext::new("Context Card", false, Onset::Consonant)
            .expect("test context is valid");
        let environment = crate::environment::canonical_test_environment();

        assert_eq!(
            onset_for_noun_phrase(&variable_reference(Variable::X), &context, &environment),
            Onset::Vowel,
        );
        assert_eq!(
            onset_for_noun_phrase(&variable_reference(Variable::Y), &context, &environment),
            Onset::Consonant,
        );
    }

    #[test]
    fn self_reference_onset_is_the_explicit_context_realization_fact() {
        let environment = crate::environment::canonical_test_environment();
        for (name, is_legendary, onset, spelling) in [
            (
                "+2 Mace",
                false,
                Onset::Consonant,
                SelfReferenceSpelling::Full,
            ),
            (
                "Aang, A Lot to Learn",
                true,
                Onset::Vowel,
                SelfReferenceSpelling::Abbreviated,
            ),
        ] {
            let context = ParseContext::new(name, is_legendary, onset)
                .expect("nonempty opaque card-name context is valid");
            let reference = SourceSelfReference::new(spelling, &context)
                .expect("chosen spelling is licensed by the context");
            assert_eq!(
                onset_for_noun_phrase(
                    &noun_phrase(UnqualifiedReference::SelfReference(reference)),
                    &context,
                    &environment,
                ),
                onset,
                "{name}",
            );
        }
    }
}
