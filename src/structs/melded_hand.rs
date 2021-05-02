use std::collections::{HashMap, HashSet};
use crate::structs::card::Card;

#[derive(Clone)]
pub struct MeldedHand {
	pub sets: HashMap<i32, HashSet<Card>>,
	pub runs: Vec<HashSet<Card>>,
	pub deadwood: HashSet<Card>,
}
