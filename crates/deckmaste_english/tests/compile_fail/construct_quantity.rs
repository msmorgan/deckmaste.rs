use deckmaste_english::Numeral;
use deckmaste_english::syntax::NumberLiteral;
use deckmaste_english::syntax::Quantity;
use deckmaste_english::syntax::QuantityKind;

fn bypass_generated_validation() -> Quantity {
    Quantity(QuantityKind::Exact(NumberLiteral {
        value: 10,
        numeral: Numeral::Roman,
    }))
}

fn main() {}
