use std::collections::{HashMap, HashSet};
use crate::structs::card::*;
use crate::melder::get_runs::*;
use crate::melder::get_deadwood::*;

pub fn get_melds(hand: &HashSet<Card>) -> (HashMap<i32, HashSet<Card>>, Vec<HashSet<Card>>, HashSet<Card>) {

	let mut hbynum: HashMap<i32, Vec<Card>> = HashMap::with_capacity(12);
	let mut hbysuit: HashMap<char, Vec<Card>> = HashMap::with_capacity(12);

	let local_hand_hs: HashSet<Card> =  hand.iter().cloned().collect();

	let mut sets: HashMap<i32, HashSet<Card>> = HashMap::with_capacity(12);
	let mut runs: Vec<_> = Vec::with_capacity(12);

	for card in local_hand_hs.iter() {
		hbynum.insert(card.num, Vec::with_capacity(12));
		hbysuit.insert(card.suit, Vec::with_capacity(12));
	}
	for card in local_hand_hs.iter() {
		hbynum.get_mut(&card.num).unwrap().push(*card);
		hbysuit.get_mut(&card.suit).unwrap().push(*card);
	}

	for num in hbynum.keys() {
		match hbynum[num].len() {
			1 | 2 => {},
			3 | 4 => {
				let mut _tmphs = HashSet::with_capacity(12);
				for card in &hbynum[num] {
					_tmphs.insert(*card);
				}
				sets.insert(*num, _tmphs);
			},
			_ => panic!("More than 4 kinds of same number? Impossible!"),
		}
	}

	for st in hbysuit.keys() {
		let _runs = get_runs(&hbysuit[st]);

		for run in _runs.iter().cloned() {
			match run.len() {
				0 ..= 2 => {},
				3 ..=12 => runs.push(run),
				_ => panic!("can't make bigger straights."),
			}
		}
	}

	let (out_sets, out_runs, deadwood) = get_deadwood(&local_hand_hs, &sets, &runs);

	(out_sets, out_runs, deadwood)

}
