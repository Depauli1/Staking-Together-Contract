use chrono::{DateTime, Utc};
use std::collections::HashMap;

/**
 * @title Staking Contract
 * @author Bless Hukporti
 * @notice This contract enables users to stake tokens and earn rewards based on their stake proportion.
 * @dev The contract is designed to handle staking operations with a maximum duration of  7 days from deployment.
 *      It uses a HashMap to track stakes and calculates rewards upon distribution.
 */

fn main() {
    let mut contract = Contract::new(1_000_000);
    contract.stake(String::from("Alice"), 5_000);
    contract.stake(String::from("Bob"), 20_000);
    println!("{:?}", contract.distribute_rewards());
}

pub struct Contract {
    pub total_coins: u64,
    pub stakers: HashMap<String, u64>,
    pub start_date: DateTime<Utc>,
}

impl Contract {
    pub fn new(total_coins: u64) -> Self {
        let now = Utc::now();
        Contract {
            total_coins,
            stakers: HashMap::new(),
            start_date: now,
        }
    }

    pub fn stake(&mut self, user: String, amount: u64) {
        if Utc::now() >= self.start_date + chrono::Duration::days(7) {
            panic!("Cannot stake after 7 days");
        }
        self.stakers.insert(user, amount);
    }

    pub fn distribute_rewards(&self) -> Vec<(String, u64)> {
        let mut rewards = vec![];
        let total_staked = self.stakers.values().sum::<u64>();
        for (user, amount) in &self.stakers {
            let reward = (amount * self.total_coins) / total_staked;
            rewards.push((user.clone(), reward));
        }
        rewards
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use cool_asserts::assert_panics;

    #[test]
    fn test_contract_creation() {
        let contract = Contract::new(1_000_000);
        assert_eq!(contract.total_coins, 1_000_000);
        assert_eq!(contract.stakers.len(), 0);
    }

    #[test]
    fn test_contract_staking() {
        let mut contract = Contract::new(1_000_000);
        contract.stake(String::from("Alice"), 5_000);
        assert_eq!(contract.stakers.get("Alice").unwrap(), &5_000);
    }

    #[test]
    fn test_contract_staking_after_seven_days() {
        let mut contract = Contract::new(1_000_000);
        contract.start_date = Utc
            .with_ymd_and_hms(2024, 2, 1, 0, 0, 0)
            .single()
            .expect("Invalid date");
        let seven_days_later = Utc
            .with_ymd_and_hms(2024, 2, 8, 0, 0, 0)
            .single()
            .expect("Invalid date");
        contract.stake(String::from("Alice"), 5_000);
        assert_eq!(contract.stakers.get("Alice").unwrap(), &5_000);

        // This is to simulate the passage of seven days
        contract.start_date = seven_days_later;

        // This prepares the arguments for the stake method
        let user = String::from("Bob");
        let amount = 20_000;

        // This actually performs the assertion
        assert_panics!(|| contract.stake(user, amount));
    }

    #[test]
    fn test_contract_distribute_rewards() {
        let mut contract = Contract::new(1_000_000);
        contract.stake(String::from("Alice"), 5_000);
        contract.stake(String::from("Bob"), 20_000);
        let rewards = contract.distribute_rewards();
        assert_eq!(rewards.len(), 2);
        assert_eq!(rewards[0], ("Alice".to_string(), 250_000));
        assert_eq!(rewards[1], ("Bob".to_string(), 750_000));
    }
}
