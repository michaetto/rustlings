// The From trait is used for value-to-value conversions.
// If From is implemented correctly for a type, the Into trait should work conversely.
// You can read more about it at https://doc.rust-lang.org/std/convert/trait.From.html
#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

// We implement the Default trait to use it as a fallback
// when the provided string is not convertible into a Person object
impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}


// Your task is to complete this implementation
// in order for the line `let p = Person::from("Mark,20")` to compile
// Please note that you'll need to parse the age component into a `usize`
// with something like `"4".parse::<usize>()`. The outcome of this needs to
// be handled appropriately.
//
// Steps:
// 1. If the length of the provided string is 0, then return the default of Person
// 2. Split the given string on the commas present in it
// 3. Extract the first element from the split operation and use it as the name
// 4. Extract the other element from the split operation and parse it into a `usize` as the age
// If while parsing the age, something goes wrong, then return the default of Person
// Otherwise, then return an instantiated Person onject with the results
fn print_type_of<T>(var_name: &str, _: &T) {
    println!("type of {}: {}", var_name, std::any::type_name::<T>())
}

impl From<&str> for Person {
    fn from(s: &str) -> Person {
        let mut iter = s.split(',').take(2);
        match (iter.next(), iter.next()) {
            (Some(name), Some(age))
                if name.len() > 0
                    && age.len() > 0
                    && name.chars().all(|c| c.is_alphabetic())
                    && age.chars().all(|c| c.is_digit(10)) =>
            {
                Person {
                    name: name.to_string(),
                    age: age.parse().unwrap(),
                }
            }
            _ => Person::default(),
        }
    }
}

fn main() {
    // Use the `from` function
    let p1 = Person::from("Mark,20");
    // Since From is implemen ted for Person, we should be able to use Into
    let p2: Person = "Gerald,70".into();
    println!("{:?}", p1);
    println!("{:?}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default() {
        // Test that the default person is 30 year old John
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }
    #[test]
    fn test_bad_convert() {
        // Test that John is returned when bad string is provided
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
    #[test]
    fn test_empty_name_with_comma_convert() {
        // Test that "Mark,20" works
        let p = Person::from(",20");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_empty_age_with_comma_convert() {
        // Test that "Mark,20" works
        let p = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_only_age_convert() {
        // Test that "Mark,20" works
        let p = Person::from("20");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_only_name_convert() {
        // Test that "Mark,20" works
        let p = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_only_wrong_value_convert() {
        // Test that "Mark,20" works
        let p = Person::from("#");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_wrong_both_values_convert_1() {
        // Test that "Mark,20" works
        let p = Person::from("#,#");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_wrong_both_values_convert_2() {
        // Test that "Mark,20" works
        let p = Person::from("12,Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_wrong_name_convert() {
        // Test that "Mark,20" works
        let p = Person::from("#,23");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_wrong_age_convert() {
        // Test that "Mark,20" works
        let p = Person::from("Mark,Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_empty_name_without_comma_convert() {
        let p = Person::from("20");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
    #[test]
    fn test_good_convert() {
        // Test that "Mark,20" works
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }
}
