fn main() {
    println!("Hello, world!");
}
const PARSING_ERROR:&str = "Parsing error should not happen, test only for correct input.";

pub fn add(numbers: String) -> i32 {
    if numbers.len() == 0 {
        0
    } else {
        let mut delimiter = ',';
        let mut parse_start = 0;

        if numbers.starts_with("//") {
            delimiter = numbers.chars()
                .skip(2)
                .next()
                .expect(PARSING_ERROR);
            parse_start = 4;
        }

        numbers[parse_start..]
            .split(|c| c == delimiter || c == '\n')
            .map(|s| {
                s.parse::<i32>()
                    .expect(PARSING_ERROR)
            })
            .sum()
    }
}

#[cfg(test)]
mod test {
    use crate::add;

    #[test]
    fn add_empty() {
        assert_eq!(0, add("".to_string()));
    }

    #[test]
    fn add_one_number() {
        assert_eq!(1, add("1".to_string()));
    }

    #[test]
    fn add_two_numbers() {
        assert_eq!(9, add("4,5".to_string()));
    }

    #[test]
    fn add_more_than_two_numbers() {
        assert_eq!(27, add("4,5,7,10,1".to_string()));
    }

    #[test]
    fn accept_new_lines_beetween_numbers() {
        assert_eq!(27, add("4\n5,7\n10,1".to_string()));
    }

    #[test]
    fn support_different_delimiters() {
        assert_eq!(3, add("//;\n1;2".to_string()))
    }
}
