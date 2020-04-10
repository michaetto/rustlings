// AsRef and AsMut allow for cheap reference-to-reference conversions.
// Read more about them at https://doc.rust-lang.org/std/convert/trait.AsRef.html
// and https://doc.rust-lang.org/std/convert/trait.AsMut.html, respectively.


// Obtain the number of bytes (not characters) in the given argument
// Add the AsRef trait appropriately as a trait bound
fn byte_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().as_bytes().len()
}


// Obtain the number of characters (not bytes) in the given argument
// Add the AsRef trait appropriately as a trait bound
fn char_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

struct MyType {
    s: &'static str
}

impl Default for MyType {
    fn default() -> Self {
        MyType{
            s: "Hello"
        }
    }
}

impl AsRef<str> for MyType {
    fn as_ref(&self) -> &str {
        self.s
    }
}

fn main() {
    let s = "Café au lait";
    println!("{}", char_counter(s));
    println!("{}", byte_counter(s));

    let s1 = MyType::default();
    println!("{}", byte_counter(&s1));
    println!("{}", byte_counter(s1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }
}
