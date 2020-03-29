// test1.rs
// This is a test for the following sections:
// - Variables
// - Functions

// Mary is buying apples. One apple usually costs 2 Rustbucks, but if you buy
// more than 40 at once, each apple only costs 1! Write a function that calculates
// the price of an order of apples given the order amount. No hints this time!



struct ApplePrice {
    normal: u64,
    discount: u64,
}

fn calculate_apple_price(quantity: u64, unit_price: &ApplePrice) -> u64 {
    match quantity {
        1..=40 => unit_price.normal * quantity,
        _ => unit_price.discount * quantity,
    }
}

// Don't modify this function!
#[test]
fn verify_test() {
    let applePrice = ApplePrice {
        normal: 2,
        discount: 1,
    };
    let price1 = calculate_apple_price(35, &applePrice);
    let price2 = calculate_apple_price(65, &applePrice);

    assert_eq!(70, price1);
    assert_eq!(65, price2);
}
