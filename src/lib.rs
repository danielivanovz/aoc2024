use itertools::Itertools;

pub mod template;

// Use this file to add helper functions and additional modules.

pub fn parse_line_pairs<T>(input: &str) -> (T, T)
where
    T: FromIterator<u32> + Default + Extend<u32>,
{
    return input
        .lines()
        .filter_map(|line| line.split_whitespace().next_tuple())
        .map(|(n, m)| (n.parse::<u32>().unwrap(), m.parse::<u32>().unwrap()))
        .unzip();
}
