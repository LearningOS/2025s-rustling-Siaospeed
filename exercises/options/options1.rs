// options1.rs
//
// Execute `rustlings hint options1` or use the `hint` watch subcommand for a
// hint.
fn maybe_icecream(time_of_day: u16) -> Option<u16> {
    if time_of_day > 23 {
        None
    } else if time_of_day >= 22 {
        Some(0)
    } else {
        Some(5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(10), Some(5));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        let icecreams = maybe_icecream(12);
        assert_eq!(icecreams, Some(5));
    }
}