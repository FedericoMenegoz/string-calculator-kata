use regex::Regex;

const PARSING_ERROR: &str = "Parsing error should not happen, assuming only correct input.";
const NEGATIVE_ERROR: &str = "negatives not allowed:";
const CUSTOM_DELIMITER: usize = 2;
const REGEX_SPECIAL_CHAR: &[char; 14] = &[
    '.', '+', '*', '?', '^', '$', '(', ')', '[', ']', '{', '}', '|', '\\',
];
/// Adds the numbers in the given string and returns the sum.
///
/// # Arguments
///
/// * `numbers` -  A string containing numbers separated by a delimiter:
/// 1. It may begin with a custom delimiter in the format `//[delimiter]\n` followed by `n1[delimiter]n2[delimiter]…`.
/// 2. If no custom delimiter is provided, the values are separated by a comma.
/// 3. New lines are accepted between numbers.
/// 4. Negative numbers are not allowed, and numbers higher than 1000 will be ignored in the final sum.
///
/// # Returns
///
/// * `Ok(sum)` - The sum of all the numbers less than or equal to 1000 if no negatives are found.
/// * `Err(error)` - An error message if negatives are found in the numbers.
///
/// # Panics
/// Panics if the string provided does not follow the format explained above.
///
/// # Example
/// ```rust
/// use string_calculator_lib::add;
///
///
/// let sum = add("2,5,7");
/// match sum {
///     Ok(val) => println!("The sum is: {val}"),
///     Err(msg) => panic!("{msg}"),
/// }
///
/// assert_eq!(sum, Ok(14));
///
/// // Returns an error with negatives numbers.
/// let sum = add("2,-5,7");
/// assert_eq!(sum, Err("negatives not allowed: -5".to_owned()));
///
/// // Ignore numbers > 1000
/// let sum = add("1003,2,5");
/// assert_eq!(sum, Ok(7));
///
/// // Custom delimiter
/// let sum = add("//;\n32;8");
/// assert_eq!(sum, Ok(40));
/// ```

pub fn add(numbers: &str) -> Result<i32, String> {
    // No input returns 0
    if numbers.trim().is_empty() {
        Ok(0)
    } else {
        let mut parse_start = 0;
        let mut negatives = Vec::new();
        let mut delimiters = vec![",".to_owned()];
        let mut sum = 0;

        // Check for custom delimiter
        // string delimiter
        if numbers.starts_with("//[") {
            delimiters = numbers[CUSTOM_DELIMITER..]
                // first string with delimiter end with new line
                .split('\n')
                // get first line
                .nth(0)
                .expect(PARSING_ERROR)
                .split(['[', ']'])
                .filter(|s| !s.is_empty())
                .map(|s| s.to_owned())
                .collect();

            parse_start = numbers.chars().take_while(|c| *c != '\n').count() + 1;
        }
        // Single char delimiter
        else if numbers.starts_with("//") {
            delimiters[0] = numbers[2..3].to_owned();

            parse_start = 4;

            // If custom delimiter is sadly chosen as '-' then I need to check for negatives numers
            if delimiters[0] == "-" {
                let re = Regex::new(r"^-(\d+)|--(\d+)").unwrap();
                for m in re.captures_iter(&numbers[parse_start..]) {
                    // Captures first negative if any (only one dash)
                    if let Some(number) = m.get(1) {
                        negatives.push(format!("-{}", number.as_str()));
                    }
                    // Captures the other negatives (two dashes)
                    else if let Some(number) = m.get(2) {
                        negatives.push(format!("-{}", number.as_str()));
                    }
                }
                if !negatives.is_empty() {
                    return Err(format!("{} {}", NEGATIVE_ERROR, negatives.join(" ")));
                }
            }
        }
        // Using regex to separate each number,
        // need to escape special regex charachter ., +, *, ?, ^, $, (, ), [, ], {, }, |, \.
        for delimiter in delimiters.iter_mut() {
            *delimiter = delimiter
                .chars()
                .map(|c| {
                    if REGEX_SPECIAL_CHAR.contains(&c) {
                        format!("\\{c}")
                    } else {
                        format!("{c}")
                    }
                })
                .collect::<Vec<_>>()
                .join("")
        }

        // Create the regex in order to match any delimiter
        let patterns = delimiters.join("|");
        let split_regex = Regex::new(&patterns).unwrap();
        // Replacing new line with the first delimiter
        let no_space = numbers[parse_start..].replace("\n", &delimiters[0]);

        // Calculate the sum
        for str in split_regex.split(&no_space) {
            println!("{str}");
            let number = str.parse::<i32>().expect(PARSING_ERROR);
            if number < 0 {
                negatives.push(number.to_string());
            } else if number <= 1000 {
                sum += number
            }
        }

        if negatives.is_empty() {
            Ok(sum)
        } else {
            Err(format!("{} {}", NEGATIVE_ERROR, negatives.join(" ")))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::add;

    #[test]
    fn add_empty() {
        assert_eq!(add(""), Ok(0));
    }

    #[test]
    fn add_one_number() {
        assert_eq!(add("3"), Ok(3));
    }

    #[test]
    fn add_two_numbers() {
        assert_eq!(add("4,5"), Ok(9));
    }

    #[test]
    fn add_more_than_two_numbers() {
        assert_eq!(add("4,5,7,10,1"), Ok(27));
    }

    #[test]
    fn accept_new_lines_beetween_numbers() {
        assert_eq!(add("4\n5,7\n10,1"), Ok(27));
    }

    #[test]
    fn support_different_delimiters() {
        assert_eq!(add("//;\n1;2"), Ok(3));
    }

    #[test]
    fn negatives_not_allowed() {
        assert_eq!(add("1,4,-1"), Err("negatives not allowed: -1".to_owned()));
        assert_eq!(
            add("1,-2,-4,-1"),
            Err("negatives not allowed: -2 -4 -1".to_owned())
        );
    }

    #[test]
    fn custom_delimiter_negatives() {
        assert_eq!(
            add("//;\n1;-2;4;-6"),
            Err("negatives not allowed: -2 -6".to_owned())
        )
    }
    #[test]
    fn ignore_big_numbers() {
        assert_eq!(add("2,1001"), Ok(2));
    }

    #[test]
    fn dash_custom_delimiter() {
        assert_eq!(
            add("//-\n1--2-3"),
            Err("negatives not allowed: -2".to_owned())
        );
        assert_eq!(
            add("//-\n1--2--3"),
            Err("negatives not allowed: -2 -3".to_owned())
        );
        assert_eq!(add("//-\n1-2-3"), Ok(6));
    }

    #[test]
    fn custom_length_delimiter() {
        assert_eq!(add("//[***]\n3***4***3"), Ok(10));
    }

    #[test]
    fn multiple_custom_length_delimiter() {
        assert_eq!(add("//[xxx][***]\n3xxx5***8"), Ok(16));
    }

    #[test]
    fn multiple_custom_length_delimiter_special_chars() {
        assert_eq!(add("//[.][***][\\][??][---]\n1.2.6***1??10\\10"), Ok(30))
    }

    #[test]
    fn is_dashes_good_in_multiple_delimiters() {
        assert_eq!(
            add("//[--][,]\n1--2,6---7"),
            Err("negatives not allowed: -7".to_owned())
        );
    }
}
