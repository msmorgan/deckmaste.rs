//! Compiler-derived English quantity declarations.

use deckmaste_construction_compiler::runtime::DeclarationViolation;
use deckmaste_construction_compiler::runtime::GroupData;

use crate::features::NounCardinality;
use crate::features::Number;
use crate::numeral::Numeral;
use crate::syntax::ComparativeWord;
use crate::syntax::NumberLiteral;
use crate::syntax::Quantity;
use crate::syntax::QuantityKind;
use crate::syntax::QuantityValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum QuantityFeatureArgument {
    NumberIsOne(bool),
    Present(bool),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct QuantityFeatureProjection {
    pub(crate) cardinality: NounCardinality,
    pub(crate) standalone_number: Number,
    pub(crate) is_one: bool,
}

pub(crate) fn project_features(
    combinator: &str,
    args: &[QuantityFeatureArgument],
) -> Option<QuantityFeatureProjection> {
    use QuantityFeatureArgument::NumberIsOne;
    use QuantityFeatureArgument::Present;

    let (cardinality, standalone_number, is_one) = match (combinator, args) {
        ("quantity_exact", [NumberIsOne(one)]) => (
            if *one {
                NounCardinality::SingularOrMass
            } else {
                NounCardinality::PluralOrMass
            },
            if *one { Number::Singular } else { Number::Plural },
            *one,
        ),
        ("quantity_at_least", [NumberIsOne(one), Present(comparative)]) => (
            if !comparative && *one {
                NounCardinality::SingularOrMass
            } else {
                NounCardinality::PluralOrMass
            },
            Number::Plural,
            false,
        ),
        ("quantity_or", [NumberIsOne(first), NumberIsOne(second)]) => {
            let singular = *first && *second;
            (
                if singular {
                    NounCardinality::SingularOrMass
                } else {
                    NounCardinality::PluralOrMass
                },
                if singular { Number::Singular } else { Number::Plural },
                false,
            )
        }
        ("quantity_bound", [NumberIsOne(one)]) => (
            if *one {
                NounCardinality::SingularOrMass
            } else {
                NounCardinality::PluralOrMass
            },
            if *one { Number::Singular } else { Number::Plural },
            false,
        ),
        ("quantity_x", []) => (NounCardinality::PluralOrMass, Number::Singular, false),
        ("quantity_plural", []) => (NounCardinality::PluralOrMass, Number::Plural, false),
        ("quantity_plural_count", []) => (NounCardinality::PluralCount, Number::Plural, false),
        ("quantity_mass", []) => (NounCardinality::Mass, Number::Singular, false),
        _ => return None,
    };
    Some(QuantityFeatureProjection {
        cardinality,
        standalone_number,
        is_one,
    })
}

const fn encodes_variable(number: NumberLiteral) -> bool {
    matches!(number.numeral, Numeral::Roman) && number.value == 10
}

fn checked_number(
    construction: &'static str,
    number: NumberLiteral,
) -> Result<NumberLiteral, DeclarationViolation> {
    if encodes_variable(number) {
        Err(DeclarationViolation {
            construction,
            requirement: "literal number is not the variable X",
        })
    } else {
        Ok(number)
    }
}

fn checked_value(
    construction: &'static str,
    value: QuantityValue,
) -> Result<QuantityValue, DeclarationViolation> {
    if matches!(value, QuantityValue::Literal(number) if encodes_variable(number)) {
        Err(DeclarationViolation {
            construction,
            requirement: "literal number is not the variable X",
        })
    } else {
        Ok(value)
    }
}

fn make_exact(number: NumberLiteral) -> Result<Quantity, DeclarationViolation> {
    Ok(Quantity::unchecked_exact(checked_number(
        "quantity_exact",
        number,
    )?))
}

fn exact_parts(value: &Quantity) -> NumberLiteral {
    let QuantityKind::Exact(number) = value.kind() else {
        unreachable!("quantity_exact dispatcher admits only Exact")
    };
    number
}

const fn is_exact(value: &Quantity) -> bool {
    matches!(value.kind(), QuantityKind::Exact(_))
}

fn make_at_least(
    value: QuantityValue,
    comparative: Option<ComparativeWord>,
) -> Result<Quantity, DeclarationViolation> {
    let value = checked_value("quantity_at_least", value)?;
    Ok(
        comparative.map_or(Quantity::unchecked_at_least(value), |word| {
            Quantity::unchecked_or_comparison(value, word)
        }),
    )
}

fn at_least_parts(value: &Quantity) -> (QuantityValue, Option<ComparativeWord>) {
    match value.kind() {
        QuantityKind::AtLeast(value) => (value, None),
        QuantityKind::OrComparison(value, word) => (value, Some(word)),
        _ => unreachable!("quantity_at_least dispatcher admits only its two semantic shapes"),
    }
}

const fn is_at_least(value: &Quantity) -> bool {
    matches!(value.kind(), QuantityKind::AtLeast(_))
}

const fn is_or_comparison(value: &Quantity) -> bool {
    matches!(value.kind(), QuantityKind::OrComparison(_, _))
}

fn make_or(first: NumberLiteral, second: NumberLiteral) -> Result<Quantity, DeclarationViolation> {
    Ok(Quantity::unchecked_or(
        checked_number("quantity_or", first)?,
        checked_number("quantity_or", second)?,
    ))
}

fn or_parts(value: &Quantity) -> (NumberLiteral, NumberLiteral) {
    let QuantityKind::Or(first, second) = value.kind() else {
        unreachable!("quantity_or dispatcher admits only Or")
    };
    (first, second)
}

const fn is_or(value: &Quantity) -> bool {
    matches!(value.kind(), QuantityKind::Or(_, _))
}

macro_rules! value_adapter {
    ($make:ident, $parts:ident, $recognizer:ident, $construction:literal, $builder:ident, $variant:ident) => {
        fn $make(value: QuantityValue) -> Result<Quantity, DeclarationViolation> {
            Ok(Quantity::$builder(checked_value($construction, value)?))
        }

        fn $parts(value: &Quantity) -> QuantityValue {
            let QuantityKind::$variant(value) = value.kind() else {
                unreachable!(concat!(
                    $construction,
                    " dispatcher received the wrong variant"
                ))
            };
            value
        }

        const fn $recognizer(value: &Quantity) -> bool {
            matches!(value.kind(), QuantityKind::$variant(_))
        }
    };
}

value_adapter!(
    make_up_to,
    up_to_parts,
    is_up_to,
    "quantity_up_to",
    unchecked_up_to,
    UpTo
);
value_adapter!(
    make_more_than,
    more_than_parts,
    is_more_than,
    "quantity_more_than",
    unchecked_more_than,
    MoreThan
);
value_adapter!(
    make_fewer_than,
    fewer_than_parts,
    is_fewer_than,
    "quantity_fewer_than",
    unchecked_fewer_than,
    FewerThan
);

macro_rules! unit_adapter {
    ($make:ident, $parts:ident, $recognizer:ident, $builder:ident, $variant:ident) => {
        #[allow(
            clippy::unnecessary_wraps,
            reason = "adapted bind constructors use the compiler's checked Result interface"
        )]
        fn $make() -> Result<Quantity, DeclarationViolation> {
            Ok(Quantity::$builder())
        }

        fn $parts(value: &Quantity) {
            assert!(matches!(value.kind(), QuantityKind::$variant));
        }

        const fn $recognizer(value: &Quantity) -> bool {
            matches!(value.kind(), QuantityKind::$variant)
        }
    };
}

unit_adapter!(make_x, x_parts, is_x, unchecked_x, X);
unit_adapter!(make_both, both_parts, is_both, unchecked_both, Both);
unit_adapter!(
    make_that_many,
    that_many_parts,
    is_that_many,
    unchecked_that_many,
    ThatMany
);
unit_adapter!(
    make_that_much,
    that_much_parts,
    is_that_much,
    unchecked_that_much,
    ThatMuch
);

deckmaste_constructions_macro::constructions! {
    group quantity;

    construction quantity_exact: Quantity {
        bind Quantity via make_exact, exact_parts {
            number: lex NumberLiteral via Numeral,
        }
        derive features = quantity_exact(number);
        witness numeral = stored number;
        form only @ 0 when check(is_exact) = lex(number);
        selection unique;
    }

    construction quantity_at_least: Quantity {
        bind Quantity via make_at_least, at_least_parts {
            bound: lex QuantityValue via Numeral,
            comparative: opt lex ComparativeWord,
        }
        derive features = quantity_at_least(bound, comparative);
        witness numeral = stored bound;
        witness comparative_word = stored comparative;
        form at_least @ 0 when check(is_at_least) = "at least" lex(bound);
        form or_comparison @ 1 when check(is_or_comparison) = lex(bound) "or" lex(comparative);
        selection unique;
    }

    construction quantity_or: Quantity {
        bind Quantity via make_or, or_parts {
            first: lex NumberLiteral via Numeral,
            second: lex NumberLiteral via Numeral,
        }
        derive features = quantity_or(first, second);
        witness first_numeral = stored first;
        witness second_numeral = stored second;
        form only @ 0 when check(is_or) = lex(first) "or" lex(second);
        selection unique;
    }

    construction quantity_x: Quantity {
        bind Quantity via make_x, x_parts {}
        derive features = quantity_x();
        form only @ 0 when check(is_x) = "X";
        selection unique;
    }

    construction quantity_both: Quantity {
        bind Quantity via make_both, both_parts {}
        derive features = quantity_plural();
        form only @ 0 when check(is_both) = "both";
        selection unique;
    }

    construction quantity_up_to: Quantity {
        bind Quantity via make_up_to, up_to_parts {
            bound: lex QuantityValue via Numeral,
        }
        derive features = quantity_bound(bound);
        witness numeral = stored bound;
        form only @ 0 when check(is_up_to) = "up to" lex(bound);
        selection unique;
    }

    construction quantity_that_many: Quantity {
        bind Quantity via make_that_many, that_many_parts {}
        derive features = quantity_plural_count();
        form only @ 0 when check(is_that_many) = "that many";
        selection unique;
    }

    construction quantity_that_much: Quantity {
        bind Quantity via make_that_much, that_much_parts {}
        derive features = quantity_mass();
        form only @ 0 when check(is_that_much) = "that much";
        selection unique;
    }

    construction quantity_more_than: Quantity {
        bind Quantity via make_more_than, more_than_parts {
            bound: lex QuantityValue via Numeral,
        }
        derive features = quantity_bound(bound);
        witness numeral = stored bound;
        form only @ 0 when check(is_more_than) = "more than" lex(bound);
        selection unique;
    }

    construction quantity_fewer_than: Quantity {
        bind Quantity via make_fewer_than, fewer_than_parts {
            bound: lex QuantityValue via Numeral,
        }
        derive features = quantity_bound(bound);
        witness numeral = stored bound;
        form only @ 0 when check(is_fewer_than) = "fewer than" lex(bound);
        selection unique;
    }
}

pub(crate) static GROUPS: &[&GroupData] = &[&QUANTITY_DECLARATION];

#[derive(Default)]
struct QuantityFeatureVisitor {
    construction: Option<&'static deckmaste_construction_compiler::runtime::ConstructionData>,
    form: Option<&'static deckmaste_construction_compiler::runtime::FormData>,
    scalar_index: usize,
    fields: Vec<Option<QuantityFeatureArgument>>,
}

impl QuantityFeatureVisitor {
    fn finish(self) -> Option<QuantityFeatureProjection> {
        let construction = self.construction?;
        let [feature] = construction.feature_combinators else {
            return None;
        };
        let args = feature
            .args
            .iter()
            .map(|name| {
                let index = construction
                    .fields
                    .iter()
                    .position(|field| field.name == *name)?;
                self.fields.get(index).copied().flatten()
            })
            .collect::<Option<Vec<_>>>()?;
        project_features(feature.combinator, &args)
    }
}

impl deckmaste_construction_compiler::runtime::LinearizationVisitor for QuantityFeatureVisitor {
    type Error = std::convert::Infallible;

    fn begin_form(
        &mut self,
        construction: &'static str,
        _form: &'static str,
        ordinal: u16,
    ) -> Result<(), Self::Error> {
        let construction = QUANTITY_DECLARATION
            .constructions
            .iter()
            .find(|candidate| candidate.id == construction)
            .expect("generated quantity form names its declaration row");
        let form = construction
            .forms
            .iter()
            .find(|candidate| candidate.ordinal == ordinal)
            .expect("generated quantity form names its declaration ordinal");
        self.construction = Some(construction);
        self.form = Some(form);
        self.scalar_index = 0;
        self.fields = construction
            .fields
            .iter()
            .map(|field| {
                matches!(
                    field.kind,
                    deckmaste_construction_compiler::runtime::FieldKindData::Optional { .. }
                )
                .then_some(QuantityFeatureArgument::Present(false))
            })
            .collect();
        Ok(())
    }

    fn literal(&mut self, _literal: &'static str) -> Result<(), Self::Error> {
        Ok(())
    }

    fn subtree<T: std::any::Any>(
        &mut self,
        category: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error> {
        unreachable!("quantity declarations contain no {category} subtree")
    }

    fn scalar<T: std::any::Any>(
        &mut self,
        codec: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        use deckmaste_construction_compiler::runtime::AtomData;

        let construction = self
            .construction
            .expect("begin_form precedes quantity scalar events");
        let path = self
            .form
            .expect("begin_form records the selected quantity form")
            .atoms
            .iter()
            .filter_map(|atom| match atom {
                AtomData::Lexeme(path) => Some(*path),
                AtomData::Literal(_) | AtomData::Hole(_) | AtomData::Identity(_) => None,
            })
            .nth(self.scalar_index)
            .expect("each quantity scalar event comes from one lexical atom");
        self.scalar_index += 1;
        let field = construction
            .fields
            .iter()
            .position(|field| field.name == path)
            .expect("quantity lexical atom names a declared field");
        let value = value as &dyn std::any::Any;
        self.fields[field] = match codec {
            "Numeral" => {
                let is_one = value
                    .downcast_ref::<NumberLiteral>()
                    .is_some_and(|number| number.value == 1)
                    || value
                        .downcast_ref::<QuantityValue>()
                        .is_some_and(|number| number.is_one());
                Some(QuantityFeatureArgument::NumberIsOne(is_one))
            }
            "ComparativeWord" => Some(QuantityFeatureArgument::Present(true)),
            _ => unreachable!("quantity declaration used unknown scalar codec {codec}"),
        };
        Ok(())
    }

    fn optional(&mut self, field: &'static str, present: bool) -> Result<(), Self::Error> {
        let construction = self
            .construction
            .expect("begin_form precedes quantity optional events");
        let field = construction
            .fields
            .iter()
            .position(|candidate| candidate.name == field)
            .expect("quantity optional event names a declared field");
        self.fields[field] = Some(QuantityFeatureArgument::Present(present));
        Ok(())
    }
}

pub(crate) fn noun_cardinality(value: Quantity) -> NounCardinality {
    feature_projection(value).cardinality
}

pub(crate) fn standalone_number(value: Quantity) -> Number {
    feature_projection(value).standalone_number
}

fn feature_projection(value: Quantity) -> QuantityFeatureProjection {
    let mut visitor = QuantityFeatureVisitor::default();
    linearize_quantity_group_with(&value, &mut visitor)
        .expect("every Quantity value has exactly one declared inverse construction");
    visitor
        .finish()
        .expect("every quantity declaration derives its feature projection")
}

#[cfg(test)]
mod tests {
    use deckmaste_construction_compiler::runtime::FieldKindData;

    use super::*;

    const fn number(value: i32, numeral: Numeral) -> NumberLiteral {
        NumberLiteral { value, numeral }
    }

    #[test]
    fn declaration_metadata_carries_all_ids_scalar_types_and_witnesses() {
        assert_eq!(QUANTITY_DECLARATION.constructions.len(), 10);
        assert_eq!(
            QUANTITY_DECLARATION
                .constructions
                .iter()
                .map(|construction| construction.id)
                .collect::<Vec<_>>(),
            [
                "quantity_exact",
                "quantity_at_least",
                "quantity_or",
                "quantity_x",
                "quantity_both",
                "quantity_up_to",
                "quantity_that_many",
                "quantity_that_much",
                "quantity_more_than",
                "quantity_fewer_than",
            ]
        );
        assert!(
            QUANTITY_DECLARATION
                .constructions
                .iter()
                .all(|construction| {
                    construction.feature_combinators.len() == 1
                        && construction
                            .forms
                            .iter()
                            .all(|form| form.erased_recognizer.is_some())
                })
        );
        let exact = &QUANTITY_DECLARATION.constructions[0];
        assert_eq!(
            exact.fields[0].kind,
            FieldKindData::TypedScalar {
                value_type: "NumberLiteral",
                codec: "Numeral",
            }
        );
        assert_eq!(exact.witnesses[0].name, "numeral");

        let at_least = &QUANTITY_DECLARATION.constructions[1];
        assert_eq!(
            at_least.fields[0].kind,
            FieldKindData::TypedScalar {
                value_type: "QuantityValue",
                codec: "Numeral",
            }
        );
        assert_eq!(
            at_least.fields[1].kind,
            FieldKindData::Optional {
                inner: &FieldKindData::Scalar {
                    codec: "ComparativeWord"
                }
            }
        );
        assert_eq!(
            at_least
                .witnesses
                .iter()
                .map(|witness| witness.name)
                .collect::<Vec<_>>(),
            ["numeral", "comparative_word"]
        );
    }

    #[test]
    fn generated_builders_cover_every_semantic_shape() {
        let one = number(1, Numeral::Cardinal);
        let two = number(2, Numeral::Arabic(false));
        let literal = QuantityValue::Literal(two);

        assert_eq!(
            build_quantity_exact(one).unwrap(),
            Quantity::unchecked_exact(one)
        );
        assert_eq!(
            build_quantity_at_least(literal, None).unwrap(),
            Quantity::unchecked_at_least(literal)
        );
        for word in [
            ComparativeWord::Fewer,
            ComparativeWord::Greater,
            ComparativeWord::Less,
            ComparativeWord::More,
        ] {
            assert_eq!(
                build_quantity_at_least(literal, Some(word)).unwrap(),
                Quantity::unchecked_or_comparison(literal, word)
            );
        }
        assert_eq!(
            build_quantity_or(one, two).unwrap(),
            Quantity::unchecked_or(one, two)
        );
        assert_eq!(build_quantity_x().unwrap(), Quantity::unchecked_x());
        assert_eq!(build_quantity_both().unwrap(), Quantity::unchecked_both());
        assert_eq!(
            build_quantity_up_to(literal).unwrap(),
            Quantity::unchecked_up_to(literal)
        );
        assert_eq!(
            build_quantity_that_many().unwrap(),
            Quantity::unchecked_that_many()
        );
        assert_eq!(
            build_quantity_that_much().unwrap(),
            Quantity::unchecked_that_much()
        );
        assert_eq!(
            build_quantity_more_than(QuantityValue::Variable).unwrap(),
            Quantity::unchecked_more_than(QuantityValue::Variable)
        );
        assert_eq!(
            build_quantity_fewer_than(literal).unwrap(),
            Quantity::unchecked_fewer_than(literal)
        );
    }

    #[test]
    fn generated_builders_reject_literal_x_encodings() {
        let encoded_x = number(10, Numeral::Roman);
        assert!(build_quantity_exact(encoded_x).is_err());
        assert!(build_quantity_or(encoded_x, number(2, Numeral::Cardinal)).is_err());
        assert!(build_quantity_up_to(QuantityValue::Literal(encoded_x)).is_err());
        assert_eq!(
            build_quantity_up_to(QuantityValue::Variable).unwrap(),
            Quantity::unchecked_up_to(QuantityValue::Variable)
        );
    }
}
