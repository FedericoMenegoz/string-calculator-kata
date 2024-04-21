fn main() {
    println!("Hello, world!");
}
const PARSING_ERROR:&str = "Parsing error should not happen, test only for correct input.";
const NEGATIVE_ERROR:&str = "negatives not allowed:";

pub fn add(numbers: String) -> Result<i32, String> {
    if numbers.len() == 0 {
        Ok(0)
    } else {
        let mut delimiter = ',';
        let mut parse_start = 0;
        let mut negatives = Vec::new();

        if numbers.starts_with("//") {
            delimiter = numbers.chars()
                .skip(2)
                .next()
                .expect(PARSING_ERROR);
            parse_start = 4;
        }

        let sum = numbers[parse_start..]
            .split(|c| c == delimiter || c == '\n')
            .map(|s| {
                let number = s.parse::<i32>().expect(PARSING_ERROR);
                if number < 0 {
                    negatives.push(number.to_string());
                }
                number
            })
            .sum();
        
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
        assert_eq!(add("".to_string()), Ok(0));
    }

    #[test]
    fn add_one_number() {
        assert_eq!(add("1".to_string()), Ok(1));
    }

    #[test]
    fn add_two_numbers() {
        assert_eq!(add("4,5".to_string()), Ok(9));
    }

    #[test]
    fn add_more_than_two_numbers() {
        assert_eq!(add("4,5,7,10,1".to_string()), Ok(27));
    }

    #[test]
    fn accept_new_lines_beetween_numbers() {
        assert_eq!(add("4\n5,7\n10,1".to_string()), Ok(27));
    }

    #[test]
    fn support_different_delimiters() {
        assert_eq!(add("//;\n1;2".to_string()), Ok(3));
    }

    #[test]
    fn negatives_not_allowed() {
        assert_eq!(add("1,4,-1".to_string()), Err("negatives not allowed: -1".to_string()));
        assert_eq!(add("1,-2,-4,-1".to_string()), Err("negatives not allowed: -2 -4 -1".to_string()));
    }
}
