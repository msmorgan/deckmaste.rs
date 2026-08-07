use deckmaste_english::Numeral;
use deckmaste_english::syntax::NumberLiteral;
use deckmaste_english::syntax::Quantity;

fn bypass_generated_validation() -> Quantity {
    Quantity::Exact(NumberLiteral {
        value: 10,
        numeral: Numeral::Roman,
    })
}

fn main() {}
