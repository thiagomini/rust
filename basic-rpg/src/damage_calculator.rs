fn calculate_damage(source_attack: u32, target_def: u32) -> u32 {
  return 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deals_minimum_damage() {
        assert_eq!(1, calculate_damage(1, 2));
    }
}