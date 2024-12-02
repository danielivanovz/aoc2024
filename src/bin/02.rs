advent_of_code::solution!(2);

#[derive(Debug)]
enum Direction {
    Increasing,
    Decreasing,
}

impl From<bool> for Direction {
    fn from(is_positive: bool) -> Self {
        match is_positive {
            true => Direction::Increasing,
            false => Direction::Decreasing,
        }
    }
}

fn is_safe(report: &[i32], allow_one_removal: bool) -> bool {
    if check_sequence(report) {
        return true;
    }

    if allow_one_removal {
        let mut modified = Vec::with_capacity(report.len() - 1);

        for skip_idx in 0..report.len() {
            modified.clear();
            modified.extend(report.iter().take(skip_idx).copied());
            modified.extend(report.iter().skip(skip_idx + 1).copied());

            if check_sequence(&modified) {
                return true;
            }
        }
    }

    false
}

fn check_sequence(report: &[i32]) -> bool {
    let mut diffs = report.windows(2).map(|pair| pair[1] - pair[0]);

    let direction = match diffs.next() {
        Some(n) if (1..=3).contains(&n.abs()) => Direction::from(n.is_positive()),
        _ => return false,
    };

    diffs.all(|diff| {
        (1..=3).contains(&diff.abs())
            && match direction {
                Direction::Increasing => diff > 0,
                Direction::Decreasing => diff < 0,
            }
    })
}

fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|num| num.parse().ok())
                .collect()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        parse(input)
            .iter()
            .filter(|&report| report.len() < 2 || is_safe(report, false))
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        parse(input)
            .iter()
            .filter(|&report| report.len() < 3 || is_safe(report, true))
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
