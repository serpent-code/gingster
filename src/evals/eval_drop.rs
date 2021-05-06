use std::collections::{HashMap, HashSet};
use crate::structs::card::*;
use crate::config::CONFIG;

pub fn eval_drop(deadwood_sorted: &[Card], possible_deck: &HashSet<Card>) -> Card {

	let mut hbynum: HashMap<i32, Vec<Card>> = HashMap::with_capacity(12);
	let mut two_of_kinds: HashMap<i32, Vec<Card>> = HashMap::with_capacity(12);
	let mut two_straights: Vec<Vec<Card>> = Vec::with_capacity(12);
	let mut triangles: Vec<HashSet<Card>> = Vec::with_capacity(12);

	let mut all_candidates = HashSet::with_capacity(12);
	let mut trash = HashSet::with_capacity(12);

	let mut scores_hm_two_of_kinds: HashMap<i32, HashSet<Card>> = HashMap::with_capacity(12);
	let mut scores_hm_two_straights: HashMap<i32, HashSet<Card>> = HashMap::with_capacity(12);
	let mut scores_hm_final: HashMap<i32, HashSet<Card>> = HashMap::with_capacity(12);

	// Initialize scores_hms:

	for i in 0..=4 {
		scores_hm_two_of_kinds.insert(i, HashSet::with_capacity(12));
		scores_hm_two_straights.insert(i, HashSet::with_capacity(12));
		scores_hm_final.insert(i, HashSet::with_capacity(12));
	}

	// Get two of kinds:

	for card in deadwood_sorted {
		hbynum.insert(card.num, Vec::with_capacity(2));
	}

	for card in deadwood_sorted {
		hbynum.get_mut(&card.num).unwrap().push(*card);
	}

	for (key, value) in &hbynum {
		if value.len() == 2 {
			two_of_kinds.insert(key.to_owned(), value.to_owned());
		}
	}

	// Get two straights:

	for card in deadwood_sorted {
		for card_in in deadwood_sorted {
			if card.suit == card_in.suit && card.num == card_in.num + 1 {
				two_straights.push(vec![card_in.to_owned(), card.to_owned()]);
			}
		}
	}

	// Get triangles:

	for card_vec in &two_straights {
		for card in card_vec.iter() {
			if two_of_kinds.contains_key(&card.num) {
				let mut triangle: HashSet<Card> = HashSet::with_capacity(3);

				for _card in card_vec.iter() {
					triangle.insert(*_card);
				}

				for _card in two_of_kinds.get(&card.num).unwrap().iter() {
					triangle.insert(*_card);
				}

				triangles.push(triangle);
			}

			all_candidates.insert(card.to_owned());
		}
	}

	// Complete all_candidates and start trash:

 	for value in two_of_kinds.values() {
		for card in value {
			all_candidates.insert(card.to_owned());
		}
	}

	for card in deadwood_sorted {
		if !all_candidates.contains(card) {
			trash.insert(card.to_owned());
		}
	}


	// Assign scores and make scores_hashmaps:

	for (key, value) in &two_of_kinds {
		let mut score = 0;
		for possible_card in possible_deck {
			if key == &possible_card.num {
				score += 1;
			}
		}
		for card in value {
			scores_hm_two_of_kinds.get_mut(&score).unwrap().insert(card.to_owned());
		}
	}

	for card_vec in &two_straights {
		let mut score = 0;
		for possible_card in possible_deck {

			if (possible_card.num == card_vec[1].num + 1 || possible_card.num == card_vec[0].num - 1)
				&& possible_card.suit == card_vec[0].suit {
				score += 1;
			}
		}
		for card in card_vec.iter() {
			scores_hm_two_straights.get_mut(&score).unwrap().insert(card.to_owned());
		}
	}


	// Handle triangles:

	for ia in 1..=2 {
		for ic in 1..=2 {
			let intersection: HashSet<_> = scores_hm_two_of_kinds.get(&ia).unwrap()
				.intersection(&scores_hm_two_straights.get(&ic).unwrap()).collect();

			for card in intersection {

				for card_vec in &two_straights {
					if card_vec.contains(&card) {
						for card in card_vec {
							scores_hm_final.get_mut(&(ia + ic)).unwrap().insert(card.to_owned());
						}
					}
				}

				for value in two_of_kinds.values() {
					if value.contains(&card) {
						for card in value {
							scores_hm_final.get_mut(&(ia + ic)).unwrap().insert(card.to_owned());
						}
					}
				}
			}
		}
	}

	// Remove higher score cards from lower sets and make one scores_hm_final:

	for i in 2..=4 {
		for card in scores_hm_final.get(&i).unwrap() {
			for ii in 1..=4 {
				scores_hm_two_of_kinds.get_mut(&ii).unwrap().remove(&card);
				scores_hm_two_straights.get_mut(&ii).unwrap().remove(&card);
			}
		}
	}

	for i in 1..=2 {
		for card in scores_hm_two_of_kinds.get(&i).unwrap() {
			scores_hm_final.get_mut(&i).unwrap().insert(card.to_owned());
		}
		for card in scores_hm_two_straights.get(&i).unwrap() {
			scores_hm_final.get_mut(&i).unwrap().insert(card.to_owned());
		}
	}

	for card in scores_hm_two_of_kinds.get(&0).unwrap() {
		trash.insert(card.to_owned());
	}

	for card in scores_hm_two_straights.get(&0).unwrap() {
		trash.insert(card.to_owned());
	}

	if CONFIG.very_verbose {
		print!("Trash: [ ");
		for card in &trash {
			print!("{}", card );
		}
		println!("]");

		println!("Scores_hm_final:");
		for (key, value) in &scores_hm_final {
			print!("{} : [ ", key);
			for card in value {
				print!("{}", card );
			}
			println!("]");
		}
	}

	let mut trash_sorted: Vec<Card> = trash.iter().cloned().collect();
	trash_sorted.sort();

	if !trash_sorted.is_empty() && trash_sorted[trash_sorted.len() - 1].num >= 5 {
		return trash_sorted[trash_sorted.len() - 1];
	}


	for i in 1..=4 {
		let mut candid_trash: Vec<Card> = scores_hm_final.get(&i).unwrap().iter().cloned().collect();
		candid_trash.sort();

		if !candid_trash.is_empty() {
			let eval_drop_card = match i {
				1 | 2 => candid_trash[candid_trash.len() - 1],
				3 | 4 => {
					let mut part_of_two_straight = false;
					let mut part_of_two_of_kinds = false;
					for card_vec in &two_straights {
						if card_vec.contains(&candid_trash[candid_trash.len() - 1]) {
							part_of_two_straight = true;
						}
					}
					for card_vec in two_of_kinds.values() {
						if card_vec.contains(&candid_trash[candid_trash.len() - 1]) {
							part_of_two_of_kinds = true;
						}
					}
					match part_of_two_straight {
						true => match part_of_two_of_kinds {
								true => candid_trash[candid_trash.len() - 2],
								false => candid_trash[candid_trash.len() - 1],
						},
						false => candid_trash[candid_trash.len() - 1],
					}
				},
				_ => panic!("Inaccessible path"),
			};

			if eval_drop_card.num > 3 {
				return eval_drop_card
			}
		}	
	}

	deadwood_sorted[deadwood_sorted.len() - 1]
}
