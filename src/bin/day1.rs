fn main() {
    let input = include_str!("./input1.txt");
    let output1 = part1(input);
    dbg!(output1);
    let output2 = part2(input);
    dbg!(output2);
}

fn part1(input: &str) -> String {
    let lines = input.split("\n").collect::<Vec<&str>>();
    let mut sum = 0;
    for l in lines {
        let mut digits: Vec<char> = Vec::new();

        // find digits
        for c in l.chars() {
            if c.is_digit(10) {
                digits.push(c);
            }
        }

        // convert first and last to number
        let number = if digits.len() > 0 {
            let s = format!(
                "{}{}",
                digits.first().unwrap_or(&'0'),
                digits.last().unwrap_or(&'0')
            );
            let value: u32 = s.parse::<u32>().unwrap();
            value
        } else {
            0
        };
        sum = sum + number;
    }

    sum.to_string()
}

fn part2(input: &str) -> String {
    let mut numbers = vec![
        "one".to_string(),
        "two".to_string(),
        "three".to_string(),
        "four".to_string(),
        "five".to_string(),
        "six".to_string(),
        "seven".to_string(),
        "eight".to_string(),
        "nine".to_string(),
    ];
    for i in 0..10 {
        numbers.push(i.to_string());
    }

    let lines = input.split("\n").collect::<Vec<&str>>();
    let mut sum = 0;
    for l in lines {
        let mut first_num = "".to_string();
        let mut second_num = "".to_string();

        // find numbers (digits and text)
        for i in 1..l.len() + 1 {
            for (idx, num) in numbers.iter().enumerate() {
                if first_num == "".to_string() {
                    // dbg!(num.to_string());
                    // dbg!(l[0..i].to_string());
                    if l[0..i].contains(num) {
                        first_num = if idx < 10 {
                            (idx + 1).to_string()
                        } else {
                            num.to_string()
                        };
                        break;
                    }
                }
            }
        }

        for i in 1..l.len() + 1 {
            for (idx, num) in numbers.iter().enumerate() {
                if second_num == "".to_string() {
                    //dbg!(num.to_string());
                    //dbg!((l.len() - i)..l.len().to_string());
                    if l[(l.len() - i)..l.len()].contains(num) {
                        second_num = if idx < 10 {
                            (idx + 1).to_string()
                        } else {
                            num.to_string()
                        };
                        break;
                    }
                }
            }
        }
        let digits = format!("{}{}", first_num, second_num);
        //dbg!(digits);
        sum = sum + digits.parse::<u32>().unwrap_or(0);
    }
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let result = part1(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );
        assert_eq!(result, "142");
    }

    #[test]
    fn part2_test() {
        let result = part2(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );
        assert_eq!(result, "281");
    }
}
