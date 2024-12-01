pub mod template;

// Use this file to add helper functions and additional modules.

pub fn parse_line_pairs<T>(input: &str) -> (T, T)
where
    T: FromIterator<u32> + Default + Extend<u32>,
{
    input
        .lines()
        .filter_map(|line| {
            let mut nums = line
                .split_whitespace()
                .filter_map(|n| n.parse::<u32>().ok());
            match (nums.next(), nums.next()) {
                (Some(left), Some(right)) => Some((left, right)),
                _ => None,
            }
        })
        .unzip()
}
