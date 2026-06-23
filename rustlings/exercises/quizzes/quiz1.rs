// This is a quiz for the following sections:
// - Variables
// - Functions
// - If
//
// Mary is buying apples. The price of an apple is calculated as follows:
// - An apple costs 2 rustbucks.
// - However, if Mary buys more than 40 apples, the price of each apple in the
// entire order is reduced to only 1 rustbuck!

// TODO: Write a function that calculates the price of an order of apples given
// the quantity bought.
// fn calculate_price_of_apples(???) -> ??? { ??? }
fn calculate_price_of_apples(apples_quantity: i64) -> i64 {
    const DEFAULT_APPLE_PRICE: i64 = 2;

    if apples_quantity <= 40 {
        apples_quantity * DEFAULT_APPLE_PRICE
    } else {
        apples_quantity * (DEFAULT_APPLE_PRICE - 1)
    }
}

fn main() {
    fn calculate_price_of_apples(apples_quantity: i64) -> i64 {
        const DEFAULT_APPLE_PRICE: i64 = 2;

        if apples_quantity <= 40 {
            apples_quantity * DEFAULT_APPLE_PRICE
        } else {
            apples_quantity * (DEFAULT_APPLE_PRICE - 1)
        }
    }

    println!("{}", calculate_price_of_apples(30));
    println!("{}", calculate_price_of_apples(40));
    println!("{}", calculate_price_of_apples(42));
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_test() {
        assert_eq!(calculate_price_of_apples(35), 70);
        assert_eq!(calculate_price_of_apples(40), 80);
        assert_eq!(calculate_price_of_apples(41), 41);
        assert_eq!(calculate_price_of_apples(65), 65);
    }
}
