use crate::{card::Card, FightResult};

pub struct Shop {
    pub cards: Vec<Card>,
}

impl Shop {
    /// Get the price of the most expensive card in the shop
    pub fn most_expensive(&self) -> u32 {
        self.cards.iter().map(|card| card.price).max().unwrap()
    }

    /// Get the total damage of all cards in the shop
    pub fn total_damage(&self) -> u32 {
        self.cards.iter().map(|card| card.damage).sum()
    }

    /// Get the total health of all cards in the shop
    pub fn total_health(&self) -> u32 {
        self.cards.iter().map(|card| card.health).sum()
    }

    /// Simulate a fight against another store. Returns a FightResult::Win if
    /// this store wins, FightResult::Loss if this store loses, and a
    /// FightResult::Tie if both stores win the same number of battles.
    pub fn fight_store(&self, other: &Shop) -> FightResult {
        todo!()
    }
}
