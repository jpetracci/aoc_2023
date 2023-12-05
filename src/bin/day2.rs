fn main() {
    let input = include_str!("./input2.txt");
    let output1 = part1(input);
    dbg!(output1);
    let output2 = part2(input);
    dbg!(output2);
}

fn part1(input: &str) -> String {
    let lines = input.split("\n").collect::<Vec<&str>>();

    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let mut valid_game_sum = 0;

    'outer: for line in lines {
        let values = line.split(":").collect::<Vec<&str>>();
        let mut game = 0;
        if values.len() == 2 {
            game = values[0].split_whitespace().collect::<Vec<&str>>()[1]
                .parse::<u32>()
                .unwrap_or(0);
            let vals = values[1].split_whitespace().collect::<Vec<&str>>();
            for (i, v) in vals.iter().enumerate() {
                if v.find("blue") != None {
                    let value = vals[i - 1].parse::<u32>().unwrap_or(0);
                    if value > max_blue {
                        continue 'outer;
                    }
                }
                if v.find("red") != None {
                    let value = vals[i - 1].parse::<u32>().unwrap_or(0);
                    if value > max_red {
                        continue 'outer;
                    }
                }
                if v.find("green") != None {
                    let value = vals[i - 1].parse::<u32>().unwrap_or(0);
                    if value > max_green {
                        continue 'outer;
                    }
                }
            }
            valid_game_sum += game;
        }
    }

    valid_game_sum.to_string()
}

fn part2(input: &str) -> String {
    let lines = input.split("\n").collect::<Vec<&str>>();

    let mut game_sum = 0;

    for line in lines {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        let values = line.split(":").collect::<Vec<&str>>();

        if values.len() == 2 {
            let vals = values[1].split_whitespace().collect::<Vec<&str>>();
            for (i, v) in vals.iter().enumerate() {
                if v.find("blue") != None {
                    let value = vals[i - 1].parse::<u32>().unwrap_or(0);
                    if value > max_blue {
                        max_blue = value;
                    }
                }
                if v.find("red") != None {
                    let value = vals[i - 1].parse::<u32>().unwrap_or(0);
                    if value > max_red {
                        max_red = value;
                    }
                }
                if v.find("green") != None {
                    let value = vals[i - 1].parse::<u32>().unwrap_or(0);
                    if value > max_green {
                        max_green = value;
                    }
                }
            }
            game_sum += (max_red * max_green * max_blue);
        }
    }
    game_sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let result = part1(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, "8");
    }

    #[test]
    fn part2_test() {
        let result = part2(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, "2286");
    }
}
