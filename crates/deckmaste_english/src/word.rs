use std::fmt;
use std::sync::OnceLock;

use crate::syntax::ComparativeWord;

mod closed_class;
mod modifier;
mod noun;
mod reverse;
mod verb;

pub use closed_class::Auxiliary;
pub use closed_class::AuxiliaryInflection;
pub use closed_class::AuxiliaryInstance;
pub use closed_class::Gender;
pub use closed_class::Pronoun;
pub use closed_class::PronounCase;
pub use closed_class::PronounInstance;
#[cfg(test)]
use closed_class::auxiliary_instances;
pub use modifier::Adjective;
use modifier::AdjectiveComparison;
pub(crate) use modifier::AdjectiveComparisonClass;
pub use modifier::CardOrientation;
pub use modifier::ColorWord;
pub use modifier::InitialSound;
pub(crate) use modifier::comparative_word;
pub use modifier::surface_initial_sound;
pub use noun::Countability;
pub use noun::Noun;
pub use noun::NounDeclension;
pub use noun::NounDefinition;
pub use noun::NounInstance;
pub use noun::NounInstanceKind;
pub(crate) use noun::NounInstanceRepr;
#[cfg(test)]
use noun::NounSurface;
pub(crate) use noun::regular_plural;
pub use reverse::LexicalSlot;
pub use reverse::NounUsage;
pub use reverse::WordMatch;
use verb::ASK_PREDICATE_FRAMES;
use verb::ATTACK_PREDICATE_FRAMES;
pub(crate) use verb::BareNominalAdjunct;
use verb::COME_PREDICATE_FRAMES;
use verb::HAVE_PREDICATE_FRAMES;
use verb::INTRANSITIVE_PREDICATE_FRAMES;
pub use verb::IrregularVerbDef;
use verb::LOOK_PREDICATE_FRAMES;
use verb::OPEN_PREDICATE_FRAMES;
use verb::PHASE_PREDICATE_FRAMES;
pub(crate) use verb::PROFORM_PREDICATE_FRAMES;
pub(crate) use verb::PredicateComplementKind;
pub(crate) use verb::PredicateFrame;
use verb::RECIPIENT_PASSIVE_PREDICATE_FRAMES;
use verb::REQUIRED_OBJECT_PREDICATE_FRAMES;
pub(crate) use verb::VERB_SLOTS;
pub use verb::Verb;
pub use verb::VerbDefinition;
pub use verb::VerbForm;
pub use verb::VerbInstance;
pub use verb::VerbSlot;

/// Compatibility name for the inherent-realization number feature.
pub use crate::features::Number;
/// Compatibility name for the inherent-realization person feature.
pub use crate::features::Person;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub enum Tense {
    Present,
    Past,
}

#[derive(Debug, Clone, Copy, serde::Serialize)]
#[allow(
    clippy::struct_excessive_bools,
    reason = "each bool is an independent, orthogonal part-of-speech/capability flag, not encodable state"
)]
struct VocabDefinition {
    spelling: &'static str,
    noun: Option<(NounDeclension, Countability)>,
    catalog_noun: bool,
    verb: Option<VerbForm>,
    predicate_frames: &'static [PredicateFrame],
    bare_nominal_adjunct: Option<BareNominalAdjunct>,
    adjective: bool,
    adverb: bool,
    sentence_adverbial: bool,
    comparison: Option<AdjectiveComparison>,
    initial_sound: Option<InitialSound>,
}

impl VocabDefinition {
    const fn new(spelling: &'static str) -> Self {
        Self {
            spelling,
            noun: None,
            catalog_noun: false,
            verb: None,
            predicate_frames: &[],
            bare_nominal_adjunct: None,
            adjective: false,
            adverb: false,
            sentence_adverbial: false,
            comparison: None,
            initial_sound: None,
        }
    }

    const fn noun(mut self, declension: NounDeclension, countability: Countability) -> Self {
        self.noun = Some((declension, countability));
        self
    }

    const fn irregular_catalog_noun(mut self, plural: &'static str) -> Self {
        self.noun = Some((
            NounDeclension::Irregular {
                singular: self.spelling,
                plural,
            },
            Countability::Count,
        ));
        self.catalog_noun = true;
        self
    }

    const fn invariant_catalog_noun(mut self) -> Self {
        self.noun = Some((NounDeclension::Invariant, Countability::Count));
        self.catalog_noun = true;
        self
    }

    const fn verb(mut self, form: VerbForm) -> Self {
        self.verb = Some(form);
        self.predicate_frames = OPEN_PREDICATE_FRAMES;
        self
    }

    const fn predicate_frames(mut self, frames: &'static [PredicateFrame]) -> Self {
        self.predicate_frames = frames;
        self
    }

    const fn bare_nominal_adjunct(mut self, adjunct: BareNominalAdjunct) -> Self {
        self.bare_nominal_adjunct = Some(adjunct);
        self
    }

    const fn adjective(mut self) -> Self {
        self.adjective = true;
        self
    }

    const fn comparison(mut self, comparison: AdjectiveComparison) -> Self {
        self.adjective = true;
        self.comparison = Some(comparison);
        self
    }

    const fn adverb(mut self) -> Self {
        self.adverb = true;
        self
    }

    /// Closed-class discourse adverbial that may front a clause before a
    /// comma; deliberately distinct from `adverb()` so it cannot fill
    /// verb-phrase or copular adverb slots.
    const fn sentence_adverbial(mut self) -> Self {
        self.sentence_adverbial = true;
        self
    }

    const fn initial_sound(mut self, sound: InitialSound) -> Self {
        self.initial_sound = Some(sound);
        self
    }

    fn merge_regular(mut self, regular: Self) -> Self {
        if self.noun.is_none() && !self.catalog_noun {
            self.noun = regular.noun;
        }
        if self.verb.is_none() {
            self.verb = regular.verb;
            self.predicate_frames = regular.predicate_frames;
        }
        self.bare_nominal_adjunct = self.bare_nominal_adjunct.or(regular.bare_nominal_adjunct);
        self.adjective |= regular.adjective;
        self.adverb |= regular.adverb;
        self
    }

    fn add_regular_part_of_speech(&mut self, part_of_speech: &str) {
        match part_of_speech {
            "noun_count" => self.add_regular_noun(Countability::Count),
            "noun_mass" => self.add_regular_noun(Countability::Mass),
            "noun_count_or_mass" => self.add_regular_noun(Countability::CountOrMass),
            "verb" => {
                self.verb = Some(VerbForm::Regular);
                self.predicate_frames = OPEN_PREDICATE_FRAMES;
            }
            "adjective" => self.adjective = true,
            "adverb" => self.adverb = true,
            _ => panic!("unknown regular-vocabulary part of speech: {part_of_speech:?}"),
        }
    }

    fn add_regular_noun(&mut self, countability: Countability) {
        let countability = match self.noun {
            None => countability,
            Some((NounDeclension::Regular, existing)) => match (existing, countability) {
                (Countability::Count, Countability::Mass)
                | (Countability::Mass, Countability::Count) => Countability::CountOrMass,
                (Countability::CountOrMass, _) | (_, Countability::CountOrMass) => {
                    Countability::CountOrMass
                }
                (Countability::Count, Countability::Count)
                | (Countability::Mass, Countability::Mass) => existing,
            },
            Some((declension, _)) => {
                panic!("regular vocabulary unexpectedly used {declension:?}")
            }
        };
        self.noun = Some((NounDeclension::Regular, countability));
    }
}

const REGULAR_VOCABULARY_TSV: &str = include_str!("regular-vocabulary.tsv");

fn regular_vocabulary() -> &'static [VocabDefinition] {
    static VOCABULARY: OnceLock<Vec<VocabDefinition>> = OnceLock::new();
    VOCABULARY.get_or_init(|| {
        let mut definitions: Vec<VocabDefinition> = Vec::new();
        let mut previous = None;

        for line in REGULAR_VOCABULARY_TSV.lines() {
            if let Some(previous) = previous {
                assert!(
                    previous < line,
                    "regular-vocabulary table must be sorted and deduplicated"
                );
            }
            previous = Some(line);

            let (lemma, part_of_speech) = line
                .split_once('\t')
                .unwrap_or_else(|| panic!("malformed regular-vocabulary row: {line:?}"));
            assert!(
                !lemma.is_empty()
                    && lemma
                        .bytes()
                        .all(|byte| { byte.is_ascii_lowercase() || matches!(byte, b'-' | b'\'') }),
                "invalid regular-vocabulary lemma: {lemma:?}"
            );

            if definitions
                .last()
                .is_none_or(|entry| entry.spelling != lemma)
            {
                definitions.push(VocabDefinition::new(lemma));
            }
            definitions
                .last_mut()
                .expect("a regular-vocabulary definition was just inserted")
                .add_regular_part_of_speech(part_of_speech);
        }

        definitions
    })
}

fn regular_definition(spelling: &str) -> Option<VocabDefinition> {
    regular_vocabulary()
        .binary_search_by_key(&spelling, |definition| definition.spelling)
        .ok()
        .map(|index| regular_vocabulary()[index])
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub struct RegularVocab(&'static str);

impl fmt::Debug for RegularVocab {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

macro_rules! vocabulary {
    ($(
        $variant:ident($spelling:literal)
        $(.$method:ident($($argument:expr),* $(,)?))*;
    )+) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
        pub enum Vocab {
            $($variant,)+
            Regular(RegularVocab),
        }

        impl Vocab {
            pub const ALL: &'static [Self] = &[$(Self::$variant,)+];

            #[must_use]
            pub const fn spelling(self) -> &'static str {
                match self {
                    $(Self::$variant => $spelling,)+
                    Self::Regular(regular) => regular.0,
                }
            }

            pub(crate) fn from_spelling(spelling: &str) -> Option<Self> {
                Self::ALL
                    .iter()
                    .copied()
                    .find(|word| word.spelling() == spelling)
                    .or_else(|| {
                        regular_definition(spelling)
                            .map(|definition| Self::Regular(RegularVocab(definition.spelling)))
                    })
            }

            pub(crate) fn predicate_frames(self) -> &'static [PredicateFrame] {
                self.definition().predicate_frames
            }

            pub(crate) fn bare_nominal_adjunct(self) -> Option<BareNominalAdjunct> {
                self.definition().bare_nominal_adjunct
            }

            pub(crate) fn comparison(self) -> Option<AdjectiveComparison> {
                self.definition().comparison
            }

            #[allow(
                dead_code,
                reason = "mirrors comparison()/adverb() accessor precedent; unused until a semantic-IR consumer (ticket english-semantic-ir) reads it"
            )]
            pub(crate) fn sentence_adverbial(self) -> bool {
                self.definition().sentence_adverbial
            }

            fn definition(self) -> VocabDefinition {
                let definition = match self {
                    $(Self::$variant => VocabDefinition::new($spelling)
                        $(.$method($($argument),*))*,)+
                    Self::Regular(regular) => return regular_definition(regular.0)
                        .expect("regular Vocab identity must come from the checked-in table"),
                };
                regular_definition(definition.spelling)
                    .map_or(definition, |regular| definition.merge_regular(regular))
            }
        }
    };
}

vocabulary! {
    Abandon("abandon").verb(VerbForm::Regular);
    Ability("ability").noun(NounDeclension::Regular, Countability::Count);
    Able("able").adjective();
    Activate("activate").verb(VerbForm::Regular);
    Adapt("adapt").verb(VerbForm::Regular);
    Add("add").verb(VerbForm::Regular);
    Additional("additional").adjective();
    Aetherborn("Aetherborn").invariant_catalog_noun();
    Affect("affect")
        .verb(VerbForm::Regular)
        .predicate_frames(REQUIRED_OBJECT_PREDICATE_FRAMES);
    Again("again").adverb();
    Airbend("airbend").verb(VerbForm::Regular);
    Alone("alone").adverb();
    Amass("amass").verb(VerbForm::Regular);
    Apply("apply").verb(VerbForm::Regular);
    Ask("ask")
        .verb(VerbForm::Regular)
        .predicate_frames(ASK_PREDICATE_FRAMES);
    Assemble("assemble").verb(VerbForm::Regular);
    Astartes("Astartes").invariant_catalog_noun();
    Attach("attach").verb(VerbForm::Regular);
    Attack("attack")
        .verb(VerbForm::Regular)
        .predicate_frames(ATTACK_PREDICATE_FRAMES);
    Aurochs("Aurochs").invariant_catalog_noun();
    Bargain("bargain").verb(VerbForm::Regular);
    Battlefield("battlefield").noun(NounDeclension::Regular, Countability::Count);
    Be("be").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_present("are", "is", "are")
            .with_past_agreement("were", "was", "were")
            .with_present_participle("being")
            .with_past_participle("been")
    )).predicate_frames(INTRANSITIVE_PREDICATE_FRAMES);
    Become("become").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("became")
            .with_past_participle("become")
    ));
    Begin("begin").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("began")
            .with_present_participle("beginning")
            .with_past_participle("begun")
    ));
    Behold("behold").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("beheld")
            .with_past_participle("beheld")
    ));
    Bid("bid")
        .verb(VerbForm::Irregular(
            IrregularVerbDef::EMPTY
                .with_past("bid")
                .with_present_participle("bidding")
                .with_past_participle("bid")
        ))
        .noun(NounDeclension::Regular, Countability::Count);
    Bison("Bison").invariant_catalog_noun();
    Blight("blight").verb(VerbForm::Regular);
    Block("block")
        .verb(VerbForm::Regular)
        .predicate_frames(ATTACK_PREDICATE_FRAMES);
    Bolster("bolster").verb(VerbForm::Regular);
    Break("break").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("broke")
            .with_past_participle("broken")
    ));
    Card("card").noun(NounDeclension::Regular, Countability::Count);
    Cast("cast").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("cast")
            .with_past_participle("cast")
    ));
    Cause("cause").verb(VerbForm::Regular);
    Change("change").verb(VerbForm::Regular);
    Chaos("chaos").noun(NounDeclension::Regular, Countability::Mass);
    Child("Child").irregular_catalog_noun("Children");
    Choose("choose").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("chose")
            .with_past_participle("chosen")
    ));
    Clash("clash").verb(VerbForm::Regular);
    Cloak("cloak").verb(VerbForm::Regular);
    Coin("coin").noun(NounDeclension::Regular, Countability::Count);
    Collect("collect").verb(VerbForm::Regular);
    Come("come").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("came")
            .with_past_participle("come")
    )).predicate_frames(COME_PREDICATE_FRAMES);
    Color("color").noun(NounDeclension::Regular, Countability::Count);
    Colorless("colorless").adjective();
    Commander("commander").noun(NounDeclension::Regular, Countability::Count);
    Combat("combat")
        .noun(NounDeclension::Regular, Countability::CountOrMass)
        .bare_nominal_adjunct(BareNominalAdjunct::Temporal);
    Conjure("conjure").verb(VerbForm::Regular);
    Connive("connive").verb(VerbForm::Regular);
    Control("control").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("controlled")
            .with_present_participle("controlling")
            .with_past_participle("controlled")
    )).noun(NounDeclension::Regular, Countability::Mass);
    Controller("controller").noun(NounDeclension::Regular, Countability::Count);
    Convert("convert").verb(VerbForm::Regular);
    Copy("copy")
        .noun(NounDeclension::Regular, Countability::Count)
        .verb(VerbForm::Regular);
    Cost("cost")
        .noun(NounDeclension::Regular, Countability::Count)
        .verb(VerbForm::Irregular(
            IrregularVerbDef::EMPTY
                .with_past("cost")
                .with_past_participle("cost")
        ));
    Count("count").verb(VerbForm::Regular);
    Counter("counter")
        .noun(NounDeclension::Regular, Countability::Count)
        .verb(VerbForm::Regular);
    Create("create").verb(VerbForm::Regular);
    Custodes("Custodes").invariant_catalog_noun();
    Cyberman("Cyberman").irregular_catalog_noun("Cybermen");
    Damage("damage").noun(NounDeclension::Regular, Countability::Mass);
    Day("day").noun(NounDeclension::Regular, Countability::Count);
    Deal("deal").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("dealt")
            .with_past_participle("dealt")
    )).predicate_frames(RECIPIENT_PASSIVE_PREDICATE_FRAMES);
    Deck("deck").noun(NounDeclension::Regular, Countability::Count);
    Defend("defend").verb(VerbForm::Regular);
    Destroy("destroy").verb(VerbForm::Regular);
    Detain("detain").verb(VerbForm::Regular);
    Die("die")
        .noun(
            NounDeclension::Irregular {
                singular: "die",
                plural: "dice",
            },
            Countability::Count,
        )
        .verb(VerbForm::Irregular(
            IrregularVerbDef::EMPTY.with_present_participle("dying")
        ));
    Discard("discard").verb(VerbForm::Regular);
    Discover("discover").verb(VerbForm::Regular);
    Double("double").verb(VerbForm::Regular);
    Do("do").verb(VerbForm::Irregular(
            IrregularVerbDef::EMPTY
                .with_present_third_singular("does")
                .with_past("did")
                .with_present_participle("doing")
                .with_past_participle("done")
        )).predicate_frames(PROFORM_PREDICATE_FRAMES);
    Draft("draft").verb(VerbForm::Regular);
    Draw("draw")
        .verb(VerbForm::Irregular(
            IrregularVerbDef::EMPTY
                .with_past("drew")
                .with_past_participle("drawn")
        ))
        .noun(NounDeclension::Regular, Countability::Count);
    Drix("Drix").invariant_catalog_noun();
    Dwarf("Dwarf").irregular_catalog_noun("Dwarves");
    Earthbend("earthbend").verb(VerbForm::Regular);
    Effect("effect").noun(NounDeclension::Regular, Countability::Count);
    Elf("Elf").irregular_catalog_noun("Elves");
    Elk("Elk").invariant_catalog_noun();
    End("end").noun(NounDeclension::Regular, Countability::Count);
    Equal("equal").verb(VerbForm::Regular).adjective();
    Enchant("enchant").verb(VerbForm::Regular);
    Endure("endure").verb(VerbForm::Regular);
    Ensue("ensue")
        .verb(VerbForm::Regular)
        .predicate_frames(INTRANSITIVE_PREDICATE_FRAMES);
    Enter("enter").verb(VerbForm::Regular);
    Equip("equip").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("equipped")
            .with_present_participle("equipping")
            .with_past_participle("equipped")
    ));
    Equipment("Equipment").invariant_catalog_noun();
    Even("even").adjective();
    Evidence("evidence").noun(NounDeclension::Regular, Countability::Mass);
    Exchange("exchange").verb(VerbForm::Regular);
    Exert("exert").verb(VerbForm::Regular);
    Exile("exile")
        .noun(NounDeclension::Regular, Countability::Mass)
        .verb(VerbForm::Regular);
    Explore("explore").verb(VerbForm::Regular);
    Fateseal("fateseal").verb(VerbForm::Regular);
    Fewer("fewer").comparison(AdjectiveComparison::OrComparative(ComparativeWord::Fewer));
    Fewest("fewest").adjective().noun(NounDeclension::Regular, Countability::CountOrMass);
    Fight("fight").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("fought")
            .with_past_participle("fought")
    ));
    Fish("Fish").invariant_catalog_noun();
    Flip("flip").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("flipped")
            .with_present_participle("flipping")
            .with_past_participle("flipped")
    ));
    Foe("foe").noun(NounDeclension::Regular, Countability::Count);
    Forage("forage").verb(VerbForm::Regular);
    Foretell("foretell").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("foretold")
            .with_past_participle("foretold")
    ));
    Friend("friend").noun(NounDeclension::Regular, Countability::Count);
    Fungus("Fungus").irregular_catalog_noun("Fungi");
    Gain("gain").verb(VerbForm::Regular);
    Game("game").noun(NounDeclension::Regular, Countability::Count);
    Get("get").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("got")
            .with_present_participle("getting")
            .with_past_participle("gotten")
    ));
    Go("go").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("went")
            .with_past_participle("gone")
    ));
    Goad("goad").verb(VerbForm::Regular);
    Graveborn("Graveborn").invariant_catalog_noun();
    Graveyard("graveyard").noun(NounDeclension::Regular, Countability::Count);
    Greater("greater").comparison(AdjectiveComparison::OrComparative(ComparativeWord::Greater));
    Greatest("greatest").adjective().noun(NounDeclension::Regular, Countability::CountOrMass);
    Hand("hand").noun(NounDeclension::Regular, Countability::Count);
    Harness("harness").verb(VerbForm::Regular);
    Have("have").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_present_third_singular("has")
            .with_past("had")
            .with_present_participle("having")
            .with_past_participle("had")
    )).predicate_frames(HAVE_PREDICATE_FRAMES);
    Heal("heal").verb(VerbForm::Regular);
    Heist("heist").verb(VerbForm::Regular);
    Hero("Hero").irregular_catalog_noun("Heroes");
    Highest("highest").adjective().noun(NounDeclension::Regular, Countability::CountOrMass);
    Hour("hour")
        .noun(NounDeclension::Regular, Countability::Count)
        .initial_sound(InitialSound::Vowel);
    Incorporate("incorporate").verb(VerbForm::Regular);
    Incubate("incubate").verb(VerbForm::Regular);
    Initiative("initiative").noun(NounDeclension::Regular, Countability::Count);
    Instead("instead").adverb();
    Investigate("investigate").verb(VerbForm::Regular);
    Jellyfish("Jellyfish").invariant_catalog_noun();
    Kick("kick").verb(VerbForm::Regular);
    Kithkin("Kithkin").invariant_catalog_noun();
    Kor("Kor").invariant_catalog_noun();
    Learn("learn").verb(VerbForm::Regular);
    Least("least").adjective().noun(NounDeclension::Regular, Countability::CountOrMass);
    Leave("leave").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("left")
            .with_past_participle("left")
    ));
    Less("less").adverb().comparison(AdjectiveComparison::OrComparative(ComparativeWord::Less));
    Library("library").noun(NounDeclension::Regular, Countability::Count);
    Life("life").noun(NounDeclension::Regular, Countability::Mass);
    Look("look")
        .verb(VerbForm::Regular)
        .predicate_frames(LOOK_PREDICATE_FRAMES);
    Lose("lose").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("lost")
            .with_past_participle("lost")
    ));
    Lowest("lowest").adjective().noun(NounDeclension::Regular, Countability::CountOrMass);
    Mana("mana").noun(NounDeclension::Regular, Countability::Mass);
    Meld("meld").verb(VerbForm::Regular);
    Merfolk("Merfolk").invariant_catalog_noun();
    Mill("mill").verb(VerbForm::Regular);
    Mode("mode").noun(NounDeclension::Regular, Countability::Count);
    Monarch("monarch").noun(NounDeclension::Regular, Countability::Count);
    Monocolored("monocolored").adjective();
    More("more").comparison(AdjectiveComparison::OrComparative(ComparativeWord::More));
    Monstrosity("monstrosity").noun(NounDeclension::Regular, Countability::Count);
    Monstrous("monstrous").adjective();
    Moonfolk("Moonfolk").invariant_catalog_noun();
    Most("most").adjective().noun(NounDeclension::Regular, Countability::CountOrMass);
    Mouse("Mouse").irregular_catalog_noun("Mice");
    Move("move").verb(VerbForm::Regular);
    Name("name").verb(VerbForm::Regular);
    Myr("Myr").invariant_catalog_noun();
    Nearest("nearest").adjective().noun(NounDeclension::Regular, Countability::CountOrMass);
    Night("night").noun(NounDeclension::Regular, Countability::Count);
    Number("number").noun(NounDeclension::Regular, Countability::Count);
    Odd("odd").adjective();
    One("one").noun(NounDeclension::Regular, Countability::Count);
    Open("open").verb(VerbForm::Regular);
    Opponent("opponent").noun(NounDeclension::Regular, Countability::Count);
    Only("only").adverb();
    Other("other")
        .comparison(AdjectiveComparison::ThanOnly)
        .noun(NounDeclension::Regular, Countability::Count);
    Otherwise("otherwise").sentence_adverbial();
    Own("own").verb(VerbForm::Regular);
    Owner("owner").noun(NounDeclension::Regular, Countability::Count);
    Ox("Ox").irregular_catalog_noun("Oxen");
    Pass("pass").verb(VerbForm::Regular);
    Pay("pay").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("paid")
            .with_past_participle("paid")
    ));
    Pegasus("Pegasus").irregular_catalog_noun("Pegasi");
    Permanent("permanent").noun(NounDeclension::Regular, Countability::Count);
    Phase("phase")
        .verb(VerbForm::Regular)
        .predicate_frames(PHASE_PREDICATE_FRAMES);
    Pile("pile").noun(NounDeclension::Regular, Countability::Count);
    Planeswalk("planeswalk").verb(VerbForm::Regular);
    Play("play").verb(VerbForm::Regular);
    Player("player").noun(NounDeclension::Regular, Countability::Count);
    Plot("plot").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("plotted")
            .with_present_participle("plotting")
            .with_past_participle("plotted")
    ));
    Poison("poison")
        .noun(NounDeclension::Regular, Countability::Count)
        .verb(VerbForm::Regular);
    Populate("populate").verb(VerbForm::Regular);
    Power("power").noun(NounDeclension::Regular, Countability::CountOrMass);
    Prevent("prevent").verb(VerbForm::Regular);
    Process("process").noun(NounDeclension::Regular, Countability::Count);
    Produce("produce").verb(VerbForm::Regular);
    Proliferate("proliferate").verb(VerbForm::Regular);
    Promise("promise").verb(VerbForm::Regular);
    Put("put").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("put")
            .with_present_participle("putting")
            .with_past_participle("put")
    ));
    Receive("receive")
        .verb(VerbForm::Regular)
        .predicate_frames(REQUIRED_OBJECT_PREDICATE_FRAMES);
    Reduce("reduce").verb(VerbForm::Regular);
    Regenerate("regenerate").verb(VerbForm::Regular);
    Remove("remove").verb(VerbForm::Regular);
    Renowned("renowned").adjective();
    Repeat("repeat").verb(VerbForm::Regular);
    Reselect("reselect").verb(VerbForm::Regular);
    Rest("rest").noun(NounDeclension::Regular, Countability::Mass);
    Result("result").noun(NounDeclension::Regular, Countability::Count);
    Return("return").verb(VerbForm::Regular);
    Reveal("reveal").verb(VerbForm::Regular);
    RingBearer("Ring-bearer").noun(NounDeclension::Regular, Countability::Count);
    Roll("roll")
        .noun(NounDeclension::Regular, Countability::Count)
        .verb(VerbForm::Regular);
    Sacrifice("sacrifice").verb(VerbForm::Regular);
    Saddle("saddle").verb(VerbForm::Regular);
    Same("same").adjective();
    Samurai("Samurai").invariant_catalog_noun();
    Scry("scry").verb(VerbForm::Regular);
    Search("search").verb(VerbForm::Regular);
    Seek("seek").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("sought")
            .with_past_participle("sought")
    ));
    Set("set").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("set")
            .with_present_participle("setting")
            .with_past_participle("set")
    ));
    Shuffle("shuffle").verb(VerbForm::Regular);
    Skip("skip").verb(VerbForm::Regular);
    So("so").adverb();
    Source("source").noun(NounDeclension::Regular, Countability::Count);
    Spacecraft("Spacecraft").invariant_catalog_noun();
    Spend("spend").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("spent")
            .with_past_participle("spent")
    ));
    Spell("spell").noun(NounDeclension::Regular, Countability::Count);
    Squid("Squid").invariant_catalog_noun();
    Starfish("Starfish").invariant_catalog_noun();
    Stand("stand").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("stood")
            .with_past_participle("stood")
    ));
    Support("support")
        .noun(NounDeclension::Regular, Countability::Mass)
        .verb(VerbForm::Regular);
    Surveil("surveil").verb(VerbForm::Regular);
    Suspend("suspend").verb(VerbForm::Regular);
    Suspect("suspect").verb(VerbForm::Regular);
    Tap("tap").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("tapped")
            .with_present_participle("tapping")
            .with_past_participle("tapped")
    ));
    Take("take").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("took")
            .with_past_participle("taken")
    ));
    Target("target")
        .noun(NounDeclension::Regular, Countability::Count)
        .verb(VerbForm::Regular)
        .adjective();
    Team("team").noun(NounDeclension::Regular, Countability::Count);
    Thalakos("Thalakos").invariant_catalog_noun();
    Then("then").adverb();
    Step("step")
        .noun(NounDeclension::Regular, Countability::Count)
        .verb(VerbForm::Regular)
        .bare_nominal_adjunct(BareNominalAdjunct::Temporal);
    Time("time")
        .noun(NounDeclension::Regular, Countability::CountOrMass)
        .verb(VerbForm::Regular)
        .bare_nominal_adjunct(BareNominalAdjunct::Temporal);
    Token("token").noun(NounDeclension::Regular, Countability::Count);
    Top("top").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("topped")
            .with_present_participle("topping")
            .with_past_participle("topped")
    ));
    Toughness("toughness").noun(NounDeclension::Regular, Countability::Mass);
    Transform("transform").verb(VerbForm::Regular);
    Treefolk("Treefolk").invariant_catalog_noun();
    Triple("triple").verb(VerbForm::Regular);
    Turn("turn")
        .noun(NounDeclension::Regular, Countability::Count)
        .verb(VerbForm::Regular)
        .bare_nominal_adjunct(BareNominalAdjunct::Temporal);
    Twice("twice").adverb();
    Type("type").noun(NounDeclension::Regular, Countability::Count);
    Value("value").noun(NounDeclension::Regular, Countability::Count);
    Untap("untap").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("untapped")
            .with_present_participle("untapping")
            .with_past_participle("untapped")
    ));
    Vedalken("Vedalken").invariant_catalog_noun();
    Venture("venture").verb(VerbForm::Regular);
    Vote("vote").verb(VerbForm::Regular);
    Waterbend("waterbend").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("waterbent")
            .with_past_participle("waterbent")
    ));
    Way("way")
        .noun(NounDeclension::Regular, Countability::Count)
        .bare_nominal_adjunct(BareNominalAdjunct::Manner);
    Werewolf("Werewolf").irregular_catalog_noun("Werewolves");
    Win("win").verb(VerbForm::Irregular(
        IrregularVerbDef::EMPTY
            .with_past("won")
            .with_present_participle("winning")
            .with_past_participle("won")
    ));
    Wolf("Wolf").irregular_catalog_noun("Wolves");
    Yell("yell").verb(VerbForm::Regular);
    Zubera("Zubera").invariant_catalog_noun();
}

#[derive(Debug, Clone, Copy, Default, serde::Serialize)]
pub struct Vocabulary;

impl Vocabulary {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    const THIRD_PLURAL_PRESENT: VerbSlot = VerbSlot::Present {
        person: Person::Third,
        number: Number::Plural,
    };
    const THIRD_SINGULAR_PRESENT: VerbSlot = VerbSlot::Present {
        person: Person::Third,
        number: Number::Singular,
    };

    #[test]
    fn one_surface_can_fill_only_the_requested_lexical_slot() {
        let vocabulary = Vocabulary::new();

        assert_eq!(
            vocabulary.matches("cost", LexicalSlot::Noun(NounUsage::Count)),
            vec![WordMatch::Noun(NounInstance::Singular(Noun::Word(
                Vocab::Cost,
            )))]
        );
        assert_eq!(
            vocabulary.matches("cost", LexicalSlot::Verb(THIRD_PLURAL_PRESENT)),
            vec![WordMatch::Verb(VerbInstance {
                verb: Verb::Word(Vocab::Cost),
                slot: THIRD_PLURAL_PRESENT,
            })]
        );
        assert_eq!(
            vocabulary.matches("target", LexicalSlot::Adjective),
            vec![WordMatch::Adjective(Adjective::Word(Vocab::Target))]
        );
        assert_eq!(
            vocabulary.matches("target", LexicalSlot::Noun(NounUsage::Count)),
            vec![WordMatch::Noun(NounInstance::Singular(Noun::Word(
                Vocab::Target,
            )))]
        );
        assert_eq!(
            vocabulary.matches("targets", LexicalSlot::Verb(THIRD_SINGULAR_PRESENT)),
            vec![WordMatch::Verb(VerbInstance {
                verb: Verb::Word(Vocab::Target),
                slot: THIRD_SINGULAR_PRESENT,
            })]
        );
    }

    #[test]
    fn regular_count_nouns_lookup_and_render_without_source_text() {
        let vocabulary = Vocabulary::new();
        let matches = vocabulary.matches("upkeep", LexicalSlot::Noun(NounUsage::Count));
        let [WordMatch::Noun(upkeep)] = matches.as_slice() else {
            panic!("upkeep must have one singular count-noun analysis");
        };
        let NounInstanceKind::Singular(Noun::Word(upkeep)) = upkeep.kind() else {
            panic!("upkeep must have one singular count-noun analysis");
        };

        assert_eq!(upkeep.spelling(), "upkeep");
        assert_eq!(
            vocabulary.render_noun(&NounInstance::Singular(Noun::Word(*upkeep))),
            Some("upkeep".to_owned())
        );
        assert_eq!(
            vocabulary.render_noun(&NounInstance::Plural(Noun::Word(*upkeep))),
            Some("upkeeps".to_owned())
        );
        assert_eq!(
            vocabulary.matches("upkeeps", LexicalSlot::Noun(NounUsage::Count)),
            vec![WordMatch::Noun(NounInstance::Plural(Noun::Word(*upkeep)))]
        );
    }

    #[test]
    fn regular_mass_nouns_stay_out_of_count_noun_slots() {
        let vocabulary = Vocabulary::new();
        let matches = vocabulary.matches("knowledge", LexicalSlot::Noun(NounUsage::Mass));
        let [WordMatch::Noun(knowledge)] = matches.as_slice() else {
            panic!("knowledge must have one mass-noun analysis");
        };
        let NounInstanceKind::Mass(Noun::Word(knowledge)) = knowledge.kind() else {
            panic!("knowledge must have one mass-noun analysis");
        };

        assert_eq!(knowledge.spelling(), "knowledge");
        assert_eq!(
            vocabulary.render_noun(&NounInstance::Mass(Noun::Word(*knowledge))),
            Some("knowledge".to_owned())
        );
        assert!(
            vocabulary
                .matches("knowledge", LexicalSlot::Noun(NounUsage::Count))
                .is_empty()
        );
    }

    #[test]
    fn regular_verbs_generate_and_lookup_every_inflection() {
        let vocabulary = Vocabulary::new();
        let matches = vocabulary.matches("assign", LexicalSlot::Verb(VerbSlot::Infinitive));
        let [
            WordMatch::Verb(VerbInstance {
                verb: Verb::Word(assign),
                slot: VerbSlot::Infinitive,
            }),
        ] = matches.as_slice()
        else {
            panic!("assign must have one infinitive analysis");
        };

        for (slot, surface) in [
            (THIRD_SINGULAR_PRESENT, "assigns"),
            (
                VerbSlot::Past {
                    person: Person::Third,
                    number: Number::Singular,
                },
                "assigned",
            ),
            (VerbSlot::PresentParticiple, "assigning"),
            (VerbSlot::PastParticiple, "assigned"),
        ] {
            assert_eq!(
                vocabulary.render_verb(*assign, slot).as_deref(),
                Some(surface)
            );
            assert_eq!(
                vocabulary.matches(surface, LexicalSlot::Verb(slot)),
                vec![WordMatch::Verb(VerbInstance {
                    verb: Verb::Word(*assign),
                    slot,
                })]
            );
        }
    }

    #[test]
    fn regular_adjectives_and_adverbs_fill_only_their_declared_slots() {
        let vocabulary = Vocabulary::new();
        let adjective_matches = vocabulary.matches("lethal", LexicalSlot::Adjective);
        let [WordMatch::Adjective(Adjective::Word(lethal))] = adjective_matches.as_slice() else {
            panic!("lethal must have one adjective analysis");
        };
        let adverb_matches = vocabulary.matches("already", LexicalSlot::Adverb);
        let [WordMatch::Adverb(already)] = adverb_matches.as_slice() else {
            panic!("already must have one adverb analysis");
        };

        assert_eq!(lethal.spelling(), "lethal");
        assert_eq!(already.spelling(), "already");
        assert!(vocabulary.matches("lethal", LexicalSlot::Adverb).is_empty());
        assert!(
            vocabulary
                .matches("already", LexicalSlot::Adjective)
                .is_empty()
        );
    }

    #[test]
    fn named_vocab_identity_wins_when_the_regular_table_adds_a_part_of_speech() {
        let vocabulary = Vocabulary::new();

        assert_eq!(
            vocabulary.matches("attack", LexicalSlot::Noun(NounUsage::Count)),
            vec![WordMatch::Noun(NounInstance::Singular(Noun::Word(
                Vocab::Attack,
            )))]
        );
        assert_eq!(
            vocabulary.matches("attack", LexicalSlot::Verb(VerbSlot::Infinitive)),
            vec![WordMatch::Verb(VerbInstance {
                verb: Verb::Word(Vocab::Attack),
                slot: VerbSlot::Infinitive,
            })]
        );
    }

    #[test]
    fn regular_vocabulary_table_is_sorted_deduplicated_and_well_formed() {
        const TABLE: &str = include_str!("regular-vocabulary.tsv");
        let mut previous = None;
        let mut rows = 0;

        for line in TABLE.lines() {
            let (lemma, part_of_speech) = line
                .split_once('\t')
                .unwrap_or_else(|| panic!("malformed regular-vocabulary row: {line:?}"));
            assert!(
                !lemma.is_empty()
                    && lemma
                        .bytes()
                        .all(|byte| { byte.is_ascii_lowercase() || matches!(byte, b'-' | b'\'') }),
                "invalid regular-vocabulary lemma: {lemma:?}"
            );
            assert!(
                matches!(
                    part_of_speech,
                    "adjective"
                        | "adverb"
                        | "noun_count"
                        | "noun_count_or_mass"
                        | "noun_mass"
                        | "verb"
                ),
                "unknown part of speech in regular-vocabulary row: {line:?}"
            );
            if let Some(previous) = previous {
                assert!(previous < line, "table is not sorted and deduplicated");
            }
            previous = Some(line);
            rows += 1;
        }

        assert!(rows > 900, "regular-vocabulary table is unexpectedly small");
    }

    #[test]
    fn explicit_count_or_mass_row_dominates_a_narrower_noun_row() {
        let mut definition = VocabDefinition::new("example");
        definition.add_regular_noun(Countability::Count);
        definition.add_regular_noun(Countability::CountOrMass);

        assert_eq!(
            definition.noun,
            Some((NounDeclension::Regular, Countability::CountOrMass))
        );
    }

    #[test]
    fn irregular_verbs_are_selected_by_fixed_slots() {
        let vocabulary = Vocabulary::new();

        assert_eq!(
            vocabulary.render_verb(
                Vocab::Draw,
                VerbSlot::Past {
                    person: Person::Third,
                    number: Number::Singular,
                },
            ),
            Some("drew".to_owned())
        );
        assert_eq!(
            vocabulary.render_verb(Vocab::Draw, VerbSlot::PastParticiple),
            Some("drawn".to_owned())
        );
        assert_eq!(
            vocabulary.matches("do", LexicalSlot::Verb(THIRD_PLURAL_PRESENT)),
            vec![WordMatch::Verb(VerbInstance {
                verb: Verb::Word(Vocab::Do),
                slot: THIRD_PLURAL_PRESENT,
            })]
        );
        assert_eq!(
            vocabulary.matches("does", LexicalSlot::Verb(THIRD_SINGULAR_PRESENT)),
            vec![WordMatch::Verb(VerbInstance {
                verb: Verb::Word(Vocab::Do),
                slot: THIRD_SINGULAR_PRESENT,
            })]
        );
    }

    #[test]
    fn derived_participles_and_gerunds_exist_only_in_the_requested_slot() {
        let vocabulary = Vocabulary::new();
        let target = Verb::Word(Vocab::Target);
        let draw = Verb::Word(Vocab::Draw);

        assert_eq!(
            vocabulary.matches("targeted", LexicalSlot::Adjective),
            vec![WordMatch::Adjective(Adjective::Participle(
                Tense::Past,
                target.clone(),
            ))]
        );
        assert_eq!(
            vocabulary.render_adjective(&Adjective::Participle(Tense::Past, target)),
            Some("targeted".to_owned())
        );
        assert_eq!(
            vocabulary.matches("drawing", LexicalSlot::Noun(NounUsage::Mass)),
            vec![WordMatch::Noun(NounInstance::Mass(Noun::Gerund(
                draw.clone(),
            )))]
        );
        assert_eq!(
            vocabulary.render_noun(&NounInstance::Mass(Noun::Gerund(draw))),
            Some("drawing".to_owned())
        );
        assert!(
            vocabulary
                .matches("targeted", LexicalSlot::Noun(NounUsage::Count))
                .is_empty()
        );
    }

    #[test]
    fn agent_nouns_are_productive_count_nouns_with_derived_spelling() {
        let vocabulary = Vocabulary::new();

        for (singular, plural, verb) in [
            ("voter", "voters", Vocab::Vote),
            ("bidder", "bidders", Vocab::Bid),
            ("attacker", "attackers", Vocab::Attack),
            ("blocker", "blockers", Vocab::Block),
            ("chooser", "choosers", Vocab::Choose),
        ] {
            let verb = Verb::Word(verb);
            let singular_noun = NounInstance::Singular(Noun::Agentive(verb.clone()));
            let plural_noun = NounInstance::Plural(Noun::Agentive(verb));
            assert_eq!(
                vocabulary.matches(singular, LexicalSlot::Noun(NounUsage::Count)),
                vec![WordMatch::Noun(singular_noun.clone())]
            );
            assert_eq!(
                vocabulary.matches(plural, LexicalSlot::Noun(NounUsage::Count)),
                vec![WordMatch::Noun(plural_noun.clone())]
            );
            assert_eq!(
                vocabulary.render_noun(&singular_noun).as_deref(),
                Some(singular)
            );
            assert_eq!(
                vocabulary.render_noun(&plural_noun).as_deref(),
                Some(plural)
            );
            assert!(
                vocabulary
                    .matches(singular, LexicalSlot::Noun(NounUsage::Mass))
                    .is_empty()
            );
        }
    }

    #[test]
    fn explicit_nouns_coexist_with_productive_agent_readings() {
        let vocabulary = Vocabulary::new();

        let matches = vocabulary.matches("player", LexicalSlot::Noun(NounUsage::Count));
        assert_eq!(matches.len(), 2);
        assert!(
            matches.contains(&WordMatch::Noun(NounInstance::Singular(Noun::Word(
                Vocab::Player,
            ))))
        );
        assert!(
            matches.contains(&WordMatch::Noun(NounInstance::Singular(Noun::Agentive(
                Verb::Word(Vocab::Play),
            ))))
        );
    }

    #[test]
    fn audited_agent_nouns_no_longer_need_lexical_rows() {
        let vocabulary = Vocabulary::new();

        for (surface, base) in [
            ("caller", "call"),
            ("hunter", "hunt"),
            ("smasher", "smash"),
            ("voyager", "voyage"),
            ("walker", "walk"),
        ] {
            let matches = vocabulary.matches(surface, LexicalSlot::Noun(NounUsage::Count));
            let [WordMatch::Noun(noun)] = matches.as_slice() else {
                panic!("{surface} must have one derived agent-noun analysis: {matches:#?}");
            };
            let NounInstanceKind::Singular(Noun::Agentive(Verb::Word(verb))) = noun.kind() else {
                panic!("{surface} must have one derived agent-noun analysis: {matches:#?}");
            };
            assert_eq!(verb.spelling(), base);
        }
        let counter = vocabulary.matches("counter", LexicalSlot::Noun(NounUsage::Count));
        assert_eq!(counter.len(), 2);
        assert!(
            counter.contains(&WordMatch::Noun(NounInstance::Singular(Noun::Word(
                Vocab::Counter,
            ))))
        );
        assert!(
            counter.contains(&WordMatch::Noun(NounInstance::Singular(Noun::Agentive(
                Verb::Word(Vocab::Count),
            ))))
        );
    }

    #[test]
    fn noun_declension_drives_both_lookup_and_rendering() {
        let vocabulary = Vocabulary::new();

        for (surface, vocab) in [("libraries", Vocab::Library), ("copies", Vocab::Copy)] {
            let noun = NounInstance::Plural(Noun::Word(vocab));
            assert_eq!(
                vocabulary.matches(surface, LexicalSlot::Noun(NounUsage::Count)),
                vec![WordMatch::Noun(noun.clone())]
            );
            assert_eq!(vocabulary.render_noun(&noun).as_deref(), Some(surface));
        }

        for noun in [
            NounInstance::Singular(Noun::Word(Vocab::Merfolk)),
            NounInstance::Plural(Noun::Word(Vocab::Merfolk)),
        ] {
            assert_eq!(vocabulary.render_noun(&noun), Some("Merfolk".to_owned()));
        }
    }

    #[test]
    fn speculative_waterbend_past_forms_are_irregular() {
        let vocabulary = Vocabulary::new();

        for slot in [
            VerbSlot::Past {
                person: Person::Third,
                number: Number::Singular,
            },
            VerbSlot::PastParticiple,
        ] {
            assert_eq!(
                vocabulary.render_verb(Vocab::Waterbend, slot),
                Some("waterbent".to_owned())
            );
            assert_eq!(
                vocabulary.matches("waterbent", LexicalSlot::Verb(slot)),
                vec![WordMatch::Verb(VerbInstance {
                    verb: Verb::Word(Vocab::Waterbend),
                    slot,
                })]
            );
        }
    }

    #[test]
    fn colors_and_pronunciation_are_semantic() {
        let vocabulary = Vocabulary::new();

        assert_eq!(
            vocabulary.matches("black", LexicalSlot::Adjective),
            vec![WordMatch::Adjective(Adjective::Color(ColorWord::Black))]
        );
        assert_eq!(vocabulary.initial_sound(Vocab::Hour), InitialSound::Vowel);
    }

    #[test]
    fn supported_auxiliary_contractions_are_structured() {
        let vocabulary = Vocabulary::new();
        let contracted_can = AuxiliaryInstance {
            auxiliary: Auxiliary::Can,
            inflection: AuxiliaryInflection::Base,
            contracted_negation: crate::features::Contraction::Contracted,
        };

        assert_eq!(
            vocabulary.matches("can't", LexicalSlot::Auxiliary),
            vec![WordMatch::Auxiliary(contracted_can)]
        );
        assert_eq!(vocabulary.render_auxiliary(contracted_can), Some("can't"));
        assert!(
            vocabulary
                .matches("cannot", LexicalSlot::Auxiliary)
                .is_empty()
        );
        assert!(
            vocabulary
                .matches("mightn't've", LexicalSlot::Auxiliary)
                .is_empty()
        );
        assert_eq!(
            vocabulary.render_auxiliary(AuxiliaryInstance {
                auxiliary: Auxiliary::May,
                inflection: AuxiliaryInflection::Base,
                contracted_negation: crate::features::Contraction::Contracted,
            }),
            None
        );
    }

    #[test]
    fn unsupported_first_person_pronouns_have_no_candidate() {
        let vocabulary = Vocabulary::new();

        assert!(
            vocabulary
                .matches("I", LexicalSlot::Pronoun(PronounCase::Subject))
                .is_empty()
        );
    }

    #[test]
    fn third_person_singular_pronouns_preserve_gender() {
        let vocabulary = Vocabulary::new();

        for (gender, subject, object, possessive) in [
            (Gender::Masculine, "he", "him", "his"),
            (Gender::Feminine, "she", "her", "her"),
            (Gender::Neuter, "it", "it", "its"),
        ] {
            let pronoun = Pronoun::It(gender);
            let subject_instance = PronounInstance {
                pronoun,
                case: PronounCase::Subject,
            };
            let object_instance = PronounInstance {
                pronoun,
                case: PronounCase::Object,
            };

            assert_eq!(
                vocabulary.matches(subject, LexicalSlot::Pronoun(PronounCase::Subject)),
                vec![WordMatch::Pronoun(subject_instance)]
            );
            assert_eq!(
                vocabulary.matches(object, LexicalSlot::Pronoun(PronounCase::Object)),
                vec![WordMatch::Pronoun(object_instance)]
            );
            assert_eq!(vocabulary.render_pronoun(subject_instance), Some(subject));
            assert_eq!(vocabulary.render_pronoun(object_instance), Some(object));
            assert_eq!(
                vocabulary.render_possessive_pronoun(pronoun),
                Some(possessive)
            );
        }
    }

    #[test]
    fn every_declared_form_round_trips_through_its_slot() {
        let vocabulary = Vocabulary::new();
        let mut spellings = HashSet::new();

        for &vocab in Vocab::ALL {
            let definition = vocab.definition();
            assert!(spellings.insert(definition.spelling), "duplicate {vocab:?}");
            assert!(
                definition.noun.is_some()
                    || definition.verb.is_some()
                    || definition.adjective
                    || definition.adverb
                    || definition.sentence_adverbial,
                "{vocab:?} has no lexical role"
            );

            if let Some((_, countability)) = definition.noun
                && !definition.catalog_noun
            {
                for form in [
                    NounSurface::Singular,
                    NounSurface::Plural,
                    NounSurface::Mass,
                ] {
                    if !countability.accepts(form) {
                        continue;
                    }
                    let noun = match form {
                        NounSurface::Singular => NounInstance::Singular(Noun::Word(vocab)),
                        NounSurface::Plural => NounInstance::Plural(Noun::Word(vocab)),
                        NounSurface::Mass => NounInstance::Mass(Noun::Word(vocab)),
                    };
                    let surface = vocabulary.render_noun(&noun).unwrap();
                    let usage = match form {
                        NounSurface::Singular | NounSurface::Plural => NounUsage::Count,
                        NounSurface::Mass => NounUsage::Mass,
                    };
                    assert!(
                        vocabulary
                            .matches(&surface, LexicalSlot::Noun(usage))
                            .contains(&WordMatch::Noun(noun)),
                        "noun {vocab:?} {form:?} rendered as {surface:?} but did not parse"
                    );
                }
            }

            if definition.verb.is_some() {
                for slot in VERB_SLOTS {
                    let verb = VerbInstance {
                        verb: Verb::Word(vocab),
                        slot,
                    };
                    let surface = vocabulary.render_verb(vocab, slot).unwrap();
                    assert!(
                        vocabulary
                            .matches(&surface, LexicalSlot::Verb(slot))
                            .contains(&WordMatch::Verb(verb)),
                        "verb {vocab:?} {slot:?} rendered as {surface:?} but did not parse"
                    );
                }
            }

            if definition.adjective {
                let adjective = Adjective::Word(vocab);
                assert!(
                    vocabulary
                        .matches(definition.spelling, LexicalSlot::Adjective)
                        .contains(&WordMatch::Adjective(adjective)),
                    "adjective {vocab:?} did not parse"
                );
            }

            if definition.adverb {
                assert!(
                    vocabulary
                        .matches(definition.spelling, LexicalSlot::Adverb)
                        .contains(&WordMatch::Adverb(vocab)),
                    "adverb {vocab:?} did not parse"
                );
            }

            if definition.sentence_adverbial {
                assert!(
                    vocabulary
                        .matches(definition.spelling, LexicalSlot::SentenceAdverbial)
                        .contains(&WordMatch::SentenceAdverbial(vocab)),
                    "sentence adverbial {vocab:?} did not parse"
                );
            }
        }

        for auxiliary in auxiliary_instances() {
            if let Some(surface) = vocabulary.render_auxiliary(auxiliary) {
                assert!(
                    vocabulary
                        .matches(surface, LexicalSlot::Auxiliary)
                        .contains(&WordMatch::Auxiliary(auxiliary)),
                    "auxiliary {auxiliary:?} rendered as {surface:?} but did not parse"
                );
            }
        }
    }
}
