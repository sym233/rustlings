// strings3.rs
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a hint.

// I AM DONE

fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    let l = input.len();
    let mut s = 0;
    let mut e = l;
    while s < e && &input[s..s+1] == " " {
        s += 1;
    }
    while s < e && &input[e - 1..e] == " " {
        e -= 1;
    }
    input[s..e].into()
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    format!("{input} world!")
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    let mut s = String::new();
    let l = input.len();
    let mut i = 0;
    let pattern = "cars";
    let pl = pattern.len();
    let target = "balloons";
    while i < l {
        if i + pl <= l && &input[i..i + pl] == pattern {
            s += target;
            i += pl;
        } else {
            s += &input[i..i + 1];
            i += 1
        }
    }
    s
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
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
