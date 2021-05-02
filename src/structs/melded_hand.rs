use std::collections::{HashMap, HashSet};
use crate::structs::card::Card;

#[derive(Clone)]
pub struct MeldedHand {
	pub sets: HashMap<i32, HashSet<Card>>,
	pub runs: Vec<HashSet<Card>>,
	pub deadwood: HashSet<Card>,
}

impl MeldedHand {
	pub fn get_all_melded_value(&self) -> i32 {
		let mut result_value = 0;

		for set in self.sets.values() {
			for card in set {
				result_value += card.deadwood;
			}
		}

		for run in &self.runs {
			for card in run {
				result_value += card.deadwood;
			}
		}

		result_value
	}
}
