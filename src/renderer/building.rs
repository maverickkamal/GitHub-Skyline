pub fn get_max_height(contributions: &[u32]) -> u32 {
    *contributions.iter().max().unwrap_or(&1)
} 


pub fn scale_height(contribution_count: u32, max_contributions: u32, target_height: u32, scale_name: &str) -> u32 {
    if max_contributions == 0 || contribution_count == 0 || target_height == 0 {
        return 0;
    }

    let normalized = contribution_count as f32 / max_contributions as f32;
    let scaled = match scale_name.to_lowercase().as_str() {
        "linear" => normalized,
        "sqrt" => normalized.sqrt(),
        "log" => {
            let num = (1.0 + contribution_count as f32).ln();
            let den = (1.0 + max_contributions as f32).ln();
            if den > 0.0 { num / den } else { 0.0 }
        }
        "exp" | "dramatic" => normalized.powf(1.2),
        _ => normalized.powf(1.2),
    };

    let mut result = (scaled * (target_height as f32)).round() as u32;
    if contribution_count > 0 {
        let min_nonzero = match scale_name.to_lowercase().as_str() {
            "linear" | "sqrt" | "log" => 1,
            _ => 2, 
        };
        if result < min_nonzero { result = min_nonzero; }
    }
    result
}


pub fn compute_building_heights(contributions: &[u32], max_contributions: u32, target_height: u32, scale_name: &str) -> Vec<u32> {
    contributions
        .iter()
        .map(|&count| scale_height(count, max_contributions, target_height, scale_name))
        .collect()
}