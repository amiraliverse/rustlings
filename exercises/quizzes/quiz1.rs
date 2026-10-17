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

fn main() {
    // You can optionally experiment here.
    let number1: i128 = 30;
    let number2: i128 = 40;
    let number3: i128 = 41;
    let number4: i128 = 65;

    println!("{}", calculate_price_of_apples(number1));
    println!("{}", calculate_price_of_apples(number2));
    println!("{}", calculate_price_of_apples(number3));
    println!("{}", calculate_price_of_apples(number4));
}

fn calculate_price_of_apples(number: i128) -> i128 {
    if number <= 40 { number * 2 } else { number }
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
