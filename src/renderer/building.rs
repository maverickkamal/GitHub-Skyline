pub fn get_max_height(contributions: &[u32]) -> u32 {
    *contributions.iter().max().unwrap_or(&1)
} 