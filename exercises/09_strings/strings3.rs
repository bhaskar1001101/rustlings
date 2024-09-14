fn trim_me(input: &str) -> &str {
    // TODO: Remove whitespace from both ends of a string.
    &input.trim()
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There are multiple ways to do this.
    // Method 1:
    // format!("{input} world!")
    // Method 2:
    // let mut s = String::from(input);
    // s.push_str(" world!");
    // s
    // Method 3:
    input.to_string() + " world!"

}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons".
    input.replace("cars", "balloons")
}

fn main() {
    // You can optionally experiment here.
    // Use the methods above to modify the string as needed.
    let s = String::from("I think cars are cool  ");
    println!("{}", trim_me(&s));
    println!("{}", compose_me("Hello"));
    println!("{}", replace_me("I think cars are cool"));


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool",
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons",
        );
    }
}
