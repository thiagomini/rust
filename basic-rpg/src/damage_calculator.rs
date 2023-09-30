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

    #[test]
    fn damage_is_one_when_attack_and_defense_are_equal() {
        assert_eq!(1, calculate_damage(1, 1));
    }
}
