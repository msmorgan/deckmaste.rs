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
    vocab TriggerWord { Whenever = "whenever", }
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
    vocab NonCommonNoun { Token = "token", }
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
        Card = "card",
        Controller = "controller",
        Opponent = "opponent",
        Owner = "owner",
        Permanent = "permanent",
        Player = "player",
        Source = "source",
        Spell = "spell",
        Target = "target",
        Token = "token",
    }
    lexeme VerbLexeme using EnglishVerb {
        Deal = "deal",
        Gain = "gain",
        Control = "control",
        Be = "be" {
            Bare = "are",
            ThirdPersonSingular = "is",
        },
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
    construction paragraph: Ability {
        element Paragraph {
            sentences: seq Sentence separated by " " terminated by ".",
        }
        require len(sentences) >= 1;
        form paragraph = sentences;
    }
    construction triggered: Ability {
        element Triggered {
            trigger: lex TriggerWord,
            event: Clause,
            effects: seq Sentence separated by " " terminated by ".",
        }
        require event is Event;
        require len(effects) >= 1;
        form triggered = lex(trigger) event "," effects;
    }
    construction imperative: Sentence {
        element Imperative { predicate: VerbPhrase, }
        derive predicate.agreement = Values::Bare;
        form imperative = predicate;
    }
    construction declarative: Sentence {
        element Declarative { subject: Subject, predicate: VerbPhrase, }
        derive predicate.agreement = subject.agreement;
        form declarative = subject predicate;
    }
    construction with_where: Sentence {
        element WithWhere { body: Sentence, clause: Clause, }
        require clause is Where;
        form with_where = body "," clause;
    }
    construction event: Clause {
        element EventClause { subject: Subject, predicate: VerbPhrase, }
        derive predicate.agreement = subject.agreement;
        form event = subject predicate;
    }
    construction where: Clause {
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
    construction target_plural_selector: PluralSelector {
        element TargetPluralSelector { nominal: PluralNominal, }
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive onset = Values::Consonant;
        form target_plural_selector = "target" nominal;
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
    construction indefinite_reference: NounPhrase {
        element IndefiniteReference { nominal: SingularNominal, }
        derive agreement = nominal.agreement;
        derive number = nominal.number;
        derive onset = nominal.onset;
        form an when nominal.onset is Vowel = "an" nominal;
        form a otherwise = "a" nominal;
    }
    construction named_card_reference: NounPhrase {
        element NamedCardReference { name: identity CardName, }
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        form named_card_reference = "a" "card" "named" identity(name);
    }
    construction ordinary_singular_reference: NounPhrase {
        element OrdinarySingularReference { selector: SingularSelector, }
        require selector is TargetSingularSelector;
        derive agreement = selector.agreement;
        derive number = selector.number;
        derive onset = selector.onset;
        form ordinary_singular_reference = selector;
    }
    construction ordinary_plural_reference: NounPhrase {
        element OrdinaryPluralReference { selector: PluralSelector, }
        derive agreement = selector.agreement;
        derive number = selector.number;
        derive onset = selector.onset;
        form ordinary_plural_reference = selector;
    }
    construction definite_singular_reference: NounPhrase {
        element DefiniteSingularReference { selector: SingularSelector, }
        derive agreement = selector.agreement;
        derive number = selector.number;
        derive onset = Values::Consonant;
        form definite_singular_reference = "the" selector;
    }
    construction definite_plural_reference: NounPhrase {
        element DefinitePluralReference { selector: PluralSelector, }
        derive agreement = selector.agreement;
        derive number = selector.number;
        derive onset = Values::Consonant;
        form definite_plural_reference = "the" selector;
    }
    construction any_target_reference: NounPhrase {
        element AnyTargetReference {}
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Vowel;
        form any_target_reference = "any" "target";
    }
    construction another_reference: NounPhrase {
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
    construction each_reference: NounPhrase {
        element EachReference { selector: SingularSelector, }
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Vowel;
        form each_reference = "each" selector;
    }
    construction all_reference: NounPhrase {
        element AllReference { selector: PluralSelector, }
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive onset = Values::Vowel;
        form all_reference = "all" selector;
    }
    construction fixed_reference: NounPhrase {
        element FixedReference { count: CardinalQuantity, selector: PluralSelector, }
        require count.cardinality is TwoPlus;
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive onset = Values::Consonant;
        form fixed_reference = count selector;
    }
    construction variable_reference: NounPhrase {
        element VariableReference { count: lex Variable, selector: PluralSelector, }
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive onset = count.onset;
        form variable_reference = lex(count) selector;
    }
    construction up_to_one_reference: NounPhrase {
        element UpToOneReference { count: CardinalQuantity, selector: SingularSelector, }
        require count.cardinality is One;
        require any(
            selector is TargetSingularSelector,
            selector is OtherTargetSingularSelector
        );
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Vowel;
        form up_to_one_reference = "up" "to" count selector;
    }
    construction up_to_many_reference: NounPhrase {
        element UpToManyReference { count: CardinalQuantity, selector: PluralSelector, }
        require count.cardinality is TwoPlus;
        require any(
            selector is TargetPluralSelector,
            selector is OtherTargetPluralSelector
        );
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive onset = Values::Vowel;
        form up_to_many_reference = "up" "to" count selector;
    }
    construction any_number_reference: NounPhrase {
        element AnyNumberReference { selector: PluralSelector, }
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive onset = Values::Vowel;
        form any_number_reference = "any" "number" "of" selector;
    }
    construction one_or_more_reference: NounPhrase {
        element OneOrMoreReference { selector: PluralSelector, }
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive onset = Values::Consonant;
        form one_or_more_reference = "one" "or" "more" selector;
    }
    construction this_reference: NounPhrase {
        element ThisReference { nominal: SingularNominal, }
        derive agreement = nominal.agreement;
        derive number = nominal.number;
        derive onset = Values::Consonant;
        form this_reference = "this" nominal;
    }
    construction that_reference: NounPhrase {
        element ThatReference { nominal: SingularNominal, }
        derive agreement = nominal.agreement;
        derive number = nominal.number;
        derive onset = Values::Consonant;
        form that_reference = "that" nominal;
    }
    construction those_reference: NounPhrase {
        element ThoseReference { nominal: PluralNominal, }
        derive agreement = nominal.agreement;
        derive number = nominal.number;
        derive onset = Values::Consonant;
        form those_reference = "those" nominal;
    }
    construction designated_singular_reference: NounPhrase {
        element DesignatedSingularReference { designation: lex Designation, nominal: SingularNominal, }
        derive agreement = nominal.agreement;
        derive number = nominal.number;
        derive onset = Values::Consonant;
        form designated_singular_reference = "the" lex(designation) nominal;
    }
    construction designated_plural_reference: NounPhrase {
        element DesignatedPluralReference { designation: lex Designation, nominal: PluralNominal, }
        derive agreement = nominal.agreement;
        derive number = nominal.number;
        derive onset = Values::Consonant;
        form designated_plural_reference = "the" lex(designation) nominal;
    }
    construction chosen_quality_reference: NounPhrase {
        element ChosenQualityReference { quality: lex ChosenQuality, }
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = Values::Consonant;
        form chosen_quality_reference = "the" "chosen" lex(quality);
    }
    construction possessed_singular_reference: NounPhrase {
        element PossessedSingularReference {
            possessor: lex PossessiveDeterminerPronoun,
            nominal: SingularNominal,
        }
        derive agreement = nominal.agreement;
        derive number = nominal.number;
        derive onset = possessor.onset;
        form possessed_singular_reference = lex(possessor) nominal;
    }
    construction possessed_plural_reference: NounPhrase {
        element PossessedPluralReference {
            possessor: lex PossessiveDeterminerPronoun,
            nominal: PluralNominal,
        }
        derive agreement = nominal.agreement;
        derive number = nominal.number;
        derive onset = possessor.onset;
        form possessed_plural_reference = lex(possessor) nominal;
    }
    construction possessive_absolute_reference: NounPhrase {
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
    construction self_reference: NounPhrase {
        element SourceSelfReference { spelling: identity SelfReferenceSpelling, }
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        derive onset = spelling.onset;
        form self_reference = identity(spelling);
    }
    construction count: NounPhrase {
        element CountNp {
            head: PluralHead,
            controller: lex SubjectPronoun,
            threshold: lex ScalarNumber,
        }
        require controller is You;
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive onset = head.onset;
        derive controller.agreement = match controller {
            He => Values::ThirdPersonSingular,
            It => Values::ThirdPersonSingular,
            She => Values::ThirdPersonSingular,
            They => Values::Bare,
            You => Values::Bare,
        };
        derive verb.agreement = controller.agreement;
        form count = head lex(controller) verb(VerbLexeme::Control)
            "with" "power" lex(threshold) "or" "less";
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
    construction destroy: VerbPhrase {
        element Destroy { object: Object, }
        derive agreement = verb.agreement;
        form destroy = open_verb(KeywordAction, "Destroy") object;
    }
    construction connive: VerbPhrase {
        element Connive {}
        derive agreement = verb.agreement;
        form connive = open_verb(KeywordAction, "Connive");
    }
    construction deal_damage: VerbPhrase {
        element DealDamage { amount: Amount, to: Object, }
        derive agreement = verb.agreement;
        form deal_damage = verb(VerbLexeme::Deal) amount "damage" "to" to;
    }
    construction gain_life: VerbPhrase {
        element GainLife { amount: Amount, }
        derive agreement = verb.agreement;
        form gain_life = verb(VerbLexeme::Gain) amount "life";
    }
    construction number: Amount {
        element NumberAmount { number: lex ScalarNumber, }
        form number = lex(number);
    }
    construction variable: Amount {
        element VariableAmount { variable: lex Variable, }
        form variable = lex(variable);
    }
    construction cardinal: CardinalQuantity {
        element CardinalQuantityValue { number: lex CardinalNumber, }
        derive cardinality = number.cardinality;
        derive number = number.number;
        form cardinal = lex(number);
    }

    abstract sum DocumentBlock { Ability, }
    abstract product OracleText {
        blocks: seq DocumentBlock separated by "\n",
    }

    root Ability { eoi = true; standalone_render = true; }
    root Sentence { punctuation = "."; eoi = false; standalone_render = true; }
    root Possessive { eoi = true; standalone_render = true; }
    root CardinalQuantity { eoi = true; standalone_render = true; }
    root OracleText { eoi = true; standalone_render = true; }
}
