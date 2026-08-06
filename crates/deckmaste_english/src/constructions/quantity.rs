//! Compiler-derived English quantity declarations.

use deckmaste_construction_compiler::runtime::DeclarationViolation;
use deckmaste_construction_compiler::runtime::GroupData;

use crate::numeral::Numeral;
use crate::syntax::ComparativeWord;
use crate::syntax::NumberLiteral;
use crate::syntax::Quantity;
use crate::syntax::QuantityValue;

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
    Ok(Quantity::Exact(checked_number("quantity_exact", number)?))
}

fn exact_parts(value: &Quantity) -> NumberLiteral {
    let Quantity::Exact(number) = value else {
        unreachable!("quantity_exact dispatcher admits only Exact")
    };
    *number
}

fn make_at_least(
    value: QuantityValue,
    comparative: Option<ComparativeWord>,
) -> Result<Quantity, DeclarationViolation> {
    let value = checked_value("quantity_at_least", value)?;
    Ok(comparative.map_or(Quantity::AtLeast(value), |word| {
        Quantity::OrComparison(value, word)
    }))
}

fn at_least_parts(value: &Quantity) -> (QuantityValue, Option<ComparativeWord>) {
    match value {
        Quantity::AtLeast(value) => (*value, None),
        Quantity::OrComparison(value, word) => (*value, Some(*word)),
        _ => unreachable!("quantity_at_least dispatcher admits only its two semantic shapes"),
    }
}

fn make_or(first: NumberLiteral, second: NumberLiteral) -> Result<Quantity, DeclarationViolation> {
    Ok(Quantity::Or(
        checked_number("quantity_or", first)?,
        checked_number("quantity_or", second)?,
    ))
}

fn or_parts(value: &Quantity) -> (NumberLiteral, NumberLiteral) {
    let Quantity::Or(first, second) = value else {
        unreachable!("quantity_or dispatcher admits only Or")
    };
    (*first, *second)
}

macro_rules! value_adapter {
    ($make:ident, $parts:ident, $construction:literal, $variant:ident) => {
        fn $make(value: QuantityValue) -> Result<Quantity, DeclarationViolation> {
            Ok(Quantity::$variant(checked_value($construction, value)?))
        }

        fn $parts(value: &Quantity) -> QuantityValue {
            let Quantity::$variant(value) = value else {
                unreachable!(concat!(
                    $construction,
                    " dispatcher received the wrong variant"
                ))
            };
            *value
        }
    };
}

value_adapter!(make_up_to, up_to_parts, "quantity_up_to", UpTo);
value_adapter!(
    make_more_than,
    more_than_parts,
    "quantity_more_than",
    MoreThan
);
value_adapter!(
    make_fewer_than,
    fewer_than_parts,
    "quantity_fewer_than",
    FewerThan
);

macro_rules! unit_adapter {
    ($make:ident, $parts:ident, $variant:ident) => {
        #[allow(
            clippy::unnecessary_wraps,
            reason = "adapted bind constructors use the compiler's checked Result interface"
        )]
        fn $make() -> Result<Quantity, DeclarationViolation> {
            Ok(Quantity::$variant)
        }

        fn $parts(value: &Quantity) {
            assert!(matches!(value, Quantity::$variant));
        }
    };
}

unit_adapter!(make_x, x_parts, X);
unit_adapter!(make_both, both_parts, Both);
unit_adapter!(make_that_many, that_many_parts, ThatMany);
unit_adapter!(make_that_much, that_much_parts, ThatMuch);

deckmaste_constructions_macro::constructions! {
    group quantity;

    construction quantity_exact: Quantity {
        bind Quantity via make_exact, exact_parts {
            number: lex NumberLiteral via Numeral,
        }
        witness numeral = stored number;
        form only @ 0 = lex(number);
        selection unique;
    }

    construction quantity_at_least: Quantity {
        bind Quantity via make_at_least, at_least_parts {
            bound: lex QuantityValue via Numeral,
            comparative: opt lex ComparativeWord,
        }
        witness numeral = stored bound;
        witness comparative_word = stored comparative;
        form at_least @ 0 when comparative.is_none() = "at least" lex(bound);
        form or_comparison @ 1 when comparative.is_some() = lex(bound) "or" lex(comparative);
        selection unique;
    }

    construction quantity_or: Quantity {
        bind Quantity via make_or, or_parts {
            first: lex NumberLiteral via Numeral,
            second: lex NumberLiteral via Numeral,
        }
        witness first_numeral = stored first;
        witness second_numeral = stored second;
        form only @ 0 = lex(first) "or" lex(second);
        selection unique;
    }

    construction quantity_x: Quantity {
        bind Quantity via make_x, x_parts {}
        form only @ 0 = "X";
        selection unique;
    }

    construction quantity_both: Quantity {
        bind Quantity via make_both, both_parts {}
        form only @ 0 = "both";
        selection unique;
    }

    construction quantity_up_to: Quantity {
        bind Quantity via make_up_to, up_to_parts {
            bound: lex QuantityValue via Numeral,
        }
        witness numeral = stored bound;
        form only @ 0 = "up to" lex(bound);
        selection unique;
    }

    construction quantity_that_many: Quantity {
        bind Quantity via make_that_many, that_many_parts {}
        form only @ 0 = "that many";
        selection unique;
    }

    construction quantity_that_much: Quantity {
        bind Quantity via make_that_much, that_much_parts {}
        form only @ 0 = "that much";
        selection unique;
    }

    construction quantity_more_than: Quantity {
        bind Quantity via make_more_than, more_than_parts {
            bound: lex QuantityValue via Numeral,
        }
        witness numeral = stored bound;
        form only @ 0 = "more than" lex(bound);
        selection unique;
    }

    construction quantity_fewer_than: Quantity {
        bind Quantity via make_fewer_than, fewer_than_parts {
            bound: lex QuantityValue via Numeral,
        }
        witness numeral = stored bound;
        form only @ 0 = "fewer than" lex(bound);
        selection unique;
    }
}

pub(crate) static GROUPS: &[&GroupData] = &[&QUANTITY_DECLARATION];

pub(crate) fn linearize_with<V>(
    value: &Quantity,
    visitor: &mut V,
) -> Result<(), deckmaste_construction_compiler::runtime::LinearizationError<V::Error>>
where
    V: deckmaste_construction_compiler::runtime::LinearizationVisitor,
{
    match value {
        Quantity::Exact(_) => linearize_quantity_exact_with(value, visitor),
        Quantity::AtLeast(_) | Quantity::OrComparison(_, _) => {
            linearize_quantity_at_least_with(value, visitor)
        }
        Quantity::Or(_, _) => linearize_quantity_or_with(value, visitor),
        Quantity::X => linearize_quantity_x_with(value, visitor),
        Quantity::Both => linearize_quantity_both_with(value, visitor),
        Quantity::UpTo(_) => linearize_quantity_up_to_with(value, visitor),
        Quantity::ThatMany => linearize_quantity_that_many_with(value, visitor),
        Quantity::ThatMuch => linearize_quantity_that_much_with(value, visitor),
        Quantity::MoreThan(_) => linearize_quantity_more_than_with(value, visitor),
        Quantity::FewerThan(_) => linearize_quantity_fewer_than_with(value, visitor),
    }
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

        assert_eq!(build_quantity_exact(one).unwrap(), Quantity::Exact(one));
        assert_eq!(
            build_quantity_at_least(literal, None).unwrap(),
            Quantity::AtLeast(literal)
        );
        for word in [
            ComparativeWord::Fewer,
            ComparativeWord::Greater,
            ComparativeWord::Less,
            ComparativeWord::More,
        ] {
            assert_eq!(
                build_quantity_at_least(literal, Some(word)).unwrap(),
                Quantity::OrComparison(literal, word)
            );
        }
        assert_eq!(build_quantity_or(one, two).unwrap(), Quantity::Or(one, two));
        assert_eq!(build_quantity_x().unwrap(), Quantity::X);
        assert_eq!(build_quantity_both().unwrap(), Quantity::Both);
        assert_eq!(
            build_quantity_up_to(literal).unwrap(),
            Quantity::UpTo(literal)
        );
        assert_eq!(build_quantity_that_many().unwrap(), Quantity::ThatMany);
        assert_eq!(build_quantity_that_much().unwrap(), Quantity::ThatMuch);
        assert_eq!(
            build_quantity_more_than(QuantityValue::Variable).unwrap(),
            Quantity::MoreThan(QuantityValue::Variable)
        );
        assert_eq!(
            build_quantity_fewer_than(literal).unwrap(),
            Quantity::FewerThan(literal)
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
            Quantity::UpTo(QuantityValue::Variable)
        );
    }
}
