// from_into.rs
//
// The From trait is used for value-to-value conversions. If From is implemented
// correctly for a type, the Into trait should work conversely. You can read
// more about it at https://doc.rust-lang.org/std/convert/trait.From.html
//
// Execute `rustlings hint from_into` or use the `hint` watch subcommand for a
// hint.

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

// Your task is to complete this implementation in order for the line `let p =
// Person::from("Mark,20")` to compile Please note that you'll need to parse the
// age component into a `usize` with something like `"4".parse::<usize>()`. The
// outcome of this needs to be handled appropriately.
//
// Steps:
// 1. If the length of the provided string is 0, then return the default of
//    Person.
// 2. Split the given string on the commas present in it.
// 3. Extract the first element from the split operation and use it as the name.
// 4. If the name is empty, then return the default of Person.
// 5. Extract the other element from the split operation and parse it into a
//    `usize` as the age.
// If while parsing the age, something goes wrong, then return the default of
// Person Otherwise, then return an instantiated Person object with the results

impl From<&str> for Person {
    fn from(s: &str) -> Person {
        // 如果字符串为空，返回默认的 Person
        if s.is_empty() {
            return Person::default();
        }

        // 按逗号分割字符串
        let parts: Vec<&str> = s.split(',').collect();

        // 如果 name 部分为空，则返回默认的 Person
        let name = parts.get(0).unwrap_or(&"").to_string();
        if name.is_empty() {
            return Person::default();
        }

        // 解析 age，如果解析失败则返回默认的 Person
        let age = match parts.get(1).and_then(|&age_str| age_str.parse::<usize>().ok()) {
            Some(age) => age,
            None => return Person::default(),
        };

        // 返回由 name 和 age 组成的 Person
        Person { name, age }
    }
}

fn main() {
    // 使用 `from` 函数创建 Person
    let p1 = Person::from("Mark,20");
    // 由于实现了 From，应该可以使用 Into
    let p2: Person = "Gerald,70".into();
    println!("{:?}", p1);
    println!("{:?}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        // 测试默认的 Person 是否为 John，30
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }

    #[test]
    fn test_bad_convert() {
        // 测试空字符串时返回默认的 Person
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_good_convert() {
        // 测试 "Mark,20" 是否返回正确的 Person
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }

    #[test]
    fn test_bad_age() {
        // 测试无法解析年龄时返回默认的 Person
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_comma_and_age() {
        // 测试只有名字，没有逗号和年龄时返回默认的 Person
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_age() {
        // 测试只有名字和逗号，没有年龄时返回默认的 Person
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name() {
        // 测试只有逗号和年龄时返回默认的 Person
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_age() {
        // 测试只有逗号时返回默认的 Person
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_invalid_age() {
        // 测试没有名字和无法解析的年龄时返回默认的 Person
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma() {
        // 测试结尾有多余逗号时返回默认的 Person
        let p: Person = Person::from("Mike,32,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma_and_some_string() {
        // 测试结尾有多余逗号和额外字符串时返回默认的 Person
        let p: Person = Person::from("Mike,32,man");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
}