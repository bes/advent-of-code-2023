use aoc::Problem;

fn main() {
    println!("Hello world!");
}

fn run_trebuchet_1(file_name: &str) -> u32 {
    let problem = Problem::new(file_name);
    let split_lines = problem.data.split("\n");

    let sum = split_lines
        .filter(|line| line.len() > 0)
        .map(|line| {
            let line_sum = get_line_sum(line);
            println!("Line {line} {line_sum}");
            line_sum
        })
        .sum();

    return sum;
}

fn run_trebuchet_2(file_name: &str) -> u32 {
    let problem = Problem::new(file_name);
    let split_lines = problem.data.split("\n");
    let spelled_letters = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let sum = split_lines
        .map(|line| {
            let mut new_line = String::new();
            for (i, c) in line.chars().enumerate() {
                if c.is_ascii_digit() {
                    new_line.push(c);
                } else {
                    for (j, letter) in spelled_letters.iter().enumerate() {
                        let sub_line: String = line.chars().skip(i).collect();
                        if sub_line.starts_with(letter) {
                            new_line.push(char::from_digit(j as u32, 10).unwrap());
                            break;
                        }
                    }
                }
            }
            let line = new_line;
            if line.len() > 0 {
                let line_sum = get_line_sum(&line);
                println!("Line {line} {line_sum}");
                line_sum
            } else {
                0
            }
        })
        .sum();

    return sum;
}

fn get_line_sum(line: &str) -> u32 {
    let mut numbers_filter = line.chars().filter(|c| c.is_ascii_digit());
    let first = numbers_filter.nth(0);
    let last = numbers_filter.last();
    match (first, last) {
        (Some(first), Some(last)) => first.to_digit(10).unwrap() * 10 + last.to_digit(10).unwrap(),
        (Some(first), None) => {
            let digit = first.to_digit(10).unwrap();
            digit * 10 + digit
        }
        (None, Some(last)) => {
            let digit = last.to_digit(10).unwrap();
            digit * 10 + digit
        }
        _ => 0,
    }
}

#[cfg(test)]
mod test {
    use crate::{run_trebuchet_1, run_trebuchet_2};

    #[test]
    fn test_example_1() {
        let result = run_trebuchet_1("example_1.txt");
        assert_eq!(142, result);
    }

    #[test]
    fn problem_1() {
        let result = run_trebuchet_1("file_1.txt");
        assert_eq!(56042, result);
    }

    #[test]
    fn test_example_2() {
        let result = run_trebuchet_2("example_2.txt");
        assert_eq!(281, result);
    }

    #[test]
    fn problem_2() {
        let result = run_trebuchet_2("file_1.txt");
        assert_eq!(55358, result);
    }
}
