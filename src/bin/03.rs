advent_of_code::solution!(3);

use regex::Regex;

#[derive(Debug)]
enum Instruction {
    Do,
    Dont,
    Mul(u32, u32),
}

impl Instruction {
    fn from_captures(caps: regex::Captures) -> Option<Self> {
        let instr = caps.get(0)?.as_str();
        match instr {
            "do()" => Some(Instruction::Do),
            "don't()" => Some(Instruction::Dont),
            _ if instr.starts_with("mul(") => {
                let x = caps.get(1)?.as_str().parse().ok()?;
                let y = caps.get(2)?.as_str().parse().ok()?;
                Some(Instruction::Mul(x, y))
            }
            _ => None,
        }
    }

    fn apply(&self, enabled: &mut bool) -> Option<u32> {
        match self {
            Instruction::Do => {
                *enabled = true;
                None
            }
            Instruction::Dont => {
                *enabled = false;
                None
            }
            Instruction::Mul(x, y) => {
                if *enabled {
                    Some(x * y)
                } else {
                    None
                }
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    Some(
        re.captures_iter(input)
            .map(|cap| cap[1].parse::<u32>().unwrap() * cap[2].parse::<u32>().unwrap())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap();

    Some(
        re.captures_iter(input)
            .filter_map(Instruction::from_captures)
            .scan(true, |enabled, instr| Some(instr.apply(enabled)))
            .filter_map(|x| x)
            .sum::<u32>()
            .into(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
