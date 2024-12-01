advent_of_code::solution!(1);

use std::collections::BinaryHeap;
use std::collections::HashMap;

use advent_of_code::parse_line_pairs;

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left_heap, mut right_heap): (BinaryHeap<_>, BinaryHeap<_>) = parse_line_pairs(input);

    Some(
        std::iter::from_fn(|| match (left_heap.pop(), right_heap.pop()) {
            (Some(left), Some(right)) => Some(left.abs_diff(right)),
            _ => None,
        })
        .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left_list, right_list): (Vec<_>, Vec<_>) = parse_line_pairs(input);

    let counts = right_list
        .into_iter()
        .fold(HashMap::new(), |mut counts, right| {
            *counts.entry(right).or_insert(0) += 1;
            counts
        });

    Some(
        left_list
            .into_iter()
            .map(|num| num * counts.get(&num).copied().unwrap_or_default())
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
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
