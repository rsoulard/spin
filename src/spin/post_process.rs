use crate::arguments::Arguments;

/// Apply post processing to a string to add things like quotes
pub fn post_process<T>(input: T, arguments: &Arguments) -> String where T: Into<String> {
    let mut output = input.into();

    if arguments.double_quotes {
        output = add_double_quotes(output);
    }

    if arguments.single_quotes {
        output = add_single_quotes(output);
    }

    output
}

fn add_double_quotes(input: String) -> String {
    format!("\"{}\"", input)
}

fn add_single_quotes(input: String) -> String {
    format!("\'{}\'", input)
}

#[cfg(test)]
mod tests {
    use regex::Regex;
    use super::*;

    #[test]
    fn adds_double_quotes() {
        let regex = Regex::new(r#"^"test"$"#).unwrap();
        assert!(regex.is_match(add_double_quotes(String::from("test")).as_str()));
    }

    #[test]
    fn adds_single_quotes() {
        let regex = Regex::new(r#"^'test'$"#).unwrap();
        assert!(regex.is_match(add_single_quotes(String::from("test")).as_str()));
    }
}