use std::fs;

fn main() {
    let input_file = "data/day2.data";
    println!("Input at {}", input_file);

    let contents = fs::read_to_string(input_file).expect("Should be able to read file");

    let points = solve(&contents);
    println!("total points {}", points);
}

fn solve(contents: &str) -> i32 {
    let mut points = 0;
    for (i, line) in contents.split("\n").enumerate() {
        if line == "" {
            break;
        }
        let moves: Vec<&str> = line.split(" ").collect();
        if moves.len() < 2 {
            panic!(
                "expected exactly 2 moves per line, line {} has: '{}'",
                i, line
            );
        }
        let (oponent, you) = (moves[0], moves[1]);
        let oponent = oponent_to_play(oponent);
        let you = you_to_play(you);
        let result = you.play(oponent);
        points += you.value() + result.value();
    }
    points
}

use GameResult::*;
enum GameResult {
    Win,
    Draw,
    Lost,
}

impl GameResult {
    fn value(&self) -> i32 {
        match self {
            Win => 6,
            Draw => 3,
            Lost => 0,
        }
    }
}

fn you_to_play(you: &str) -> Play {
    match you {
        "X" => Rock,
        "Y" => Paper,
        "Z" => Scissors,
        _ => panic!("opoenent play should be A,B or C, found: '{}'", you),
    }
}
fn oponent_to_play(op: &str) -> Play {
    match op {
        "A" => Rock,
        "B" => Paper,
        "C" => Scissors,
        _ => panic!("opoenent play should be A,B or C, found: '{}'", op),
    }
}

use Play::*;
enum Play {
    Rock,
    Paper,
    Scissors,
}

impl Play {
    fn play(&self, op: Play) -> GameResult {
        match (self, op) {
            (&Rock, Rock) => Draw,
            (&Rock, Paper) => Lost,
            (&Rock, Scissors) => Win,
            (&Paper, Rock) => Win,
            (&Paper, Paper) => Draw,
            (&Paper, Scissors) => Lost,
            (&Scissors, Rock) => Lost,
            (&Scissors, Paper) => Win,
            (&Scissors, Scissors) => Draw,
        }
    }
    fn value(&self) -> i32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "A Y
B X
C Z";

        assert_eq!(15, solve(input));
    }

    #[test]
    fn actual_data() {
        let input_file = "data/day2.data";

        let contents = fs::read_to_string(input_file).expect("Should be able to read file");
        assert_eq!(13682, solve(&contents));
    }
}
