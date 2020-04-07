// option1.rs
// This example panics because the second time it calls `pop`, the `vec`
// is empty, so `pop` returns `None`, and `unwrap` panics if it's called
// on `None`. Handle this in a more graceful way than calling `unwrap`!
// Execute `rustlings hint option1` for hints :)



pub fn pop_too_much() -> bool {
    let mut list = vec![3];

    let last = list.pop();
    if let Some(last_value) = last {
        println!("The last item in the list is {:?}", last_value);
    }

    let second_to_last = list.pop();
    if let Some(second_to_last_value) = second_to_last {
        println!(
            "The second-to-last item in the list is {:?}",
            second_to_last_value
        );
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_not_panic() {
        assert!(pop_too_much());
    }
}
