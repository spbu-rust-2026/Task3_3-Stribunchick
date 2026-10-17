use std::collections::HashSet;

pub fn unique_count(names: &[String]) -> usize {
    names.iter().collect::<HashSet<_>>().len()
}