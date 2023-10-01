use std::cmp;

fn calculate_damage(source_attack: u32, target_def: u32) -> u32 {
    let unbounded_damage = source_attack.checked_sub(target_def);
    let safe_unsigned_result = unbounded_damage.unwrap_or(0);
    return cmp::max(safe_unsigned_result, 1);
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

    #[test]
    fn damage_is_attack_minus_defense() {
        assert_eq!(2, calculate_damage(3, 1));
    }
}
