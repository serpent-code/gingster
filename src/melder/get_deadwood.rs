use std::collections::{HashMap, HashSet};
use crate::structs::card::*;
use crate::structs::melded_hand::*;
use crate::melder::get_runs::*;

pub fn get_deadwood(hand: &HashSet<Card>, sets_in: &HashMap<i32, HashSet<Card>>,
	runs_in: &[HashSet<Card>]) -> MeldedHand {
// -> sets, runs, deadwood

	let mut all_results = Vec::with_capacity(12);
	let mut runshs = HashSet::with_capacity(12);
	let mut setshs = HashSet::with_capacity(12);

	for num in sets_in.keys() {
		for card in sets_in[num].iter() {
			setshs.insert(*card);
		}
	}
	for run in runs_in.iter() {
		for card in run.iter() {
			runshs.insert(*card);
		}
	}

	let mut bigrun: Vec<Card> = Vec::with_capacity(12);
	let mut smlrun: Vec<Card> = Vec::with_capacity(12);

	let intersections_pre: Vec<Card> = setshs.intersection(&runshs).cloned().collect();
	let mut intersections_vec: Vec<Card> = Vec::with_capacity(12);

	let mut intersections_hm: HashMap<char, HashSet<Card>> = HashMap::with_capacity(4);
	for card in intersections_pre.iter() {
		intersections_hm.insert(card.suit, HashSet::with_capacity(5));
	}
	for card in intersections_pre.iter() {
		intersections_hm.get_mut(&card.suit).unwrap().insert(*card);
	}

	for suit in intersections_hm.keys() {
		let maxcard = intersections_hm[suit].iter().max().unwrap();
		let mincard = intersections_hm[suit].iter().min().unwrap();
		for run in runs_in.iter() {
			if run.contains(maxcard) {
				for card in run.iter() {
					bigrun.push(*card);
				}
			}
			if run.contains(mincard) {
				for card in run.iter() {
					smlrun.push(*card);
				}
			}
			bigrun.sort();
			smlrun.sort();
		}
		if bigrun.last().unwrap() == maxcard {
			intersections_vec.push(*maxcard);
			if intersections_hm[suit].contains(&bigrun[bigrun.len() - 2]) {
				intersections_vec.push(bigrun[bigrun.len() - 2]);
			}
		}
		if smlrun.first().unwrap() == mincard {
			intersections_vec.push(*mincard);
			if intersections_hm[suit].contains(&smlrun[1]) {
				intersections_vec.push(smlrun[1]);
			}
		}
		let intersections_hs: HashSet<Card> = intersections_vec.iter().cloned().collect();
		for card in intersections_hm[suit].iter() {
			if !intersections_hs.contains(card) {
				intersections_vec.push(*card);
			}
		}
	}

	if intersections_vec.is_empty() {
		let (sets, runs) = copy_sets_and_runs(sets_in, runs_in);
		let deadwood = get_just_deadwood(hand, &sets, &runs);
		all_results.push(MeldedHand {sets:sets, runs:runs, deadwood:deadwood});
	}

	else if intersections_vec.len() == 1 {
		// give it to set:
		let (sets, mut runs) = copy_sets_and_runs(sets_in, runs_in);

		remove_card_from_runs(&mut runs, &intersections_vec[0]);

		let deadwood = get_just_deadwood(hand, &sets, &runs);
		all_results.push(MeldedHand {sets:sets, runs:runs, deadwood:deadwood});

		// give it to straight:
		let (mut sets, runs) = copy_sets_and_runs(sets_in, runs_in);

		remove_card_from_sets(&mut sets, &intersections_vec[0]);

		let deadwood = get_just_deadwood(hand, &sets, &runs);
		all_results.push(MeldedHand {sets:sets, runs:runs, deadwood:deadwood});
	}

	else if intersections_vec.len() == 2 {
		// give first card to set, second to set:

		let (sets, mut runs) = copy_sets_and_runs(sets_in, runs_in);

		remove_card_from_runs(&mut runs, &intersections_vec[0]);

		remove_card_from_runs(&mut runs, &intersections_vec[1]);
		
		let deadwood = get_just_deadwood(hand, &sets, &runs);
		all_results.push(MeldedHand {sets:sets, runs:runs, deadwood:deadwood});

		// give first card to straight, second to set:

		let (mut sets, mut runs) = copy_sets_and_runs(sets_in, runs_in);

		remove_card_from_sets(&mut sets, &intersections_vec[0]);

		remove_card_from_runs(&mut runs, &intersections_vec[1]);

		let deadwood = get_just_deadwood(hand, &sets, &runs);
		all_results.push(MeldedHand {sets:sets, runs:runs, deadwood:deadwood});

		// give first card to set, second to straight:

		let (mut sets, mut runs) = copy_sets_and_runs(sets_in, runs_in);

		remove_card_from_runs(&mut runs, &intersections_vec[0]);

		remove_card_from_sets(&mut sets, &intersections_vec[1]);

		let deadwood = get_just_deadwood(hand, &sets, &runs);
		all_results.push(MeldedHand {sets:sets, runs:runs, deadwood:deadwood});

		// give first card to straight, second to straight:

		let (mut sets, runs) = copy_sets_and_runs(sets_in, runs_in);

		remove_card_from_sets(&mut sets, &intersections_vec[0]);

		remove_card_from_sets(&mut sets, &intersections_vec[1]);

		let deadwood = get_just_deadwood(hand, &sets, &runs);
		all_results.push(MeldedHand {sets:sets, runs:runs, deadwood:deadwood});

	}

	else if intersections_vec.len() == 3 {
		// give first card to set, second to set, third to set:

		let (sets, mut runs) = copy_sets_and_runs(sets_in, runs_in);

		remove_card_from_runs(&mut runs, &intersections_vec[0]);

		remove_card_from_runs(&mut runs, &intersections_vec[1]);

		remove_card_from_runs(&mut runs, &intersections_vec[2]);

		let deadwood = get_just_deadwood(hand, &sets, &runs);
		all_results.push(MeldedHand {sets:sets, runs:runs, deadwood:deadwood});

		// give first card to set, second to set, third to straight:

		let (mut sets, mut runs) = copy_sets_and_runs(sets_in, runs_in);

		remove_card_from_runs(&mut runs, &intersections_vec[0]);

		remove_card_from_runs(&mut runs, &intersections_vec[1]);
		
		remove_card_from_sets(&mut sets, &intersections_vec[2]);

		let deadwood = get_just_deadwood(hand, &sets, &runs);
		all_results.push(MeldedHand {sets:sets, runs:runs, deadwood:deadwood});

		// give first card to set, second to straight, third to set:

		let (mut sets, mut runs) = copy_sets_and_runs(sets_in, runs_in);

		remove_card_from_runs(&mut runs, &intersections_vec[0]);

		remove_card_from_sets(&mut sets, &intersections_vec[1]);

		remove_card_from_runs(&mut runs, &intersections_vec[2]);

		let deadwood = get_just_deadwood(hand, &sets, &runs);
		all_results.push(MeldedHand {sets:sets, runs:runs, deadwood:deadwood});

		// give first card to set, second to straight, third to straight:

		let (mut sets, mut runs) = copy_sets_and_runs(sets_in, runs_in);

		remove_card_from_runs(&mut runs, &intersections_vec[0]);

		remove_card_from_sets(&mut sets, &intersections_vec[1]);

		remove_card_from_sets(&mut sets, &intersections_vec[2]);
		
		let deadwood = get_just_deadwood(hand, &sets, &runs);
		all_results.push(MeldedHand {sets:sets, runs:runs, deadwood:deadwood});

		// give first card to straight, second to set, third to set:

		let (mut sets, mut runs) = copy_sets_and_runs(sets_in, runs_in);

		remove_card_from_sets(&mut sets, &intersections_vec[0]);

		remove_card_from_runs(&mut runs, &intersections_vec[1]);

		remove_card_from_runs(&mut runs, &intersections_vec[2]);

		let deadwood = get_just_deadwood(hand, &sets, &runs);
		all_results.push(MeldedHand {sets:sets, runs:runs, deadwood:deadwood});

		// give first card to straight, second to set, third to straight:

		let (mut sets, mut runs) = copy_sets_and_runs(sets_in, runs_in);

		remove_card_from_sets(&mut sets, &intersections_vec[0]);

		remove_card_from_runs(&mut runs, &intersections_vec[1]);

		remove_card_from_sets(&mut sets, &intersections_vec[2]);

		let deadwood = get_just_deadwood(hand, &sets, &runs);
		all_results.push(MeldedHand {sets:sets, runs:runs, deadwood:deadwood});

		// give first card to straight, second to straight, third to set:

		let (mut sets, mut runs) = copy_sets_and_runs(sets_in, runs_in);

		remove_card_from_sets(&mut sets, &intersections_vec[0]);

		remove_card_from_sets(&mut sets, &intersections_vec[1]);
		
		remove_card_from_runs(&mut runs, &intersections_vec[2]);

		let deadwood = get_just_deadwood(hand, &sets, &runs);
		all_results.push(MeldedHand {sets:sets, runs:runs, deadwood:deadwood});

		// give first card to straight, second to straight, third to straight:

		let (mut sets, runs) = copy_sets_and_runs(sets_in, runs_in);

		remove_card_from_sets(&mut sets, &intersections_vec[0]);

		remove_card_from_sets(&mut sets, &intersections_vec[1]);

		remove_card_from_sets(&mut sets, &intersections_vec[2]);

		let deadwood = get_just_deadwood(hand, &sets, &runs);
		all_results.push(MeldedHand {sets:sets, runs:runs, deadwood:deadwood});

	}

	else if intersections_vec.len() > 3 {
		// trash all sets, only keep straights
		let (_, runs) = copy_sets_and_runs(sets_in, runs_in);
		let sets = HashMap::new();
		let deadwood = get_just_deadwood(hand, &sets, &runs);
		all_results.push(MeldedHand {sets:sets, runs:runs, deadwood:deadwood});

		// trash all straights, only keep sets
		let (sets, _) = copy_sets_and_runs(sets_in, runs_in);
		let runs = Vec::new();
		let deadwood = get_just_deadwood(hand, &sets, &runs);
		all_results.push(MeldedHand {sets:sets, runs:runs, deadwood:deadwood});
	}


	if all_results.len() == 1 {
		return all_results[0].clone();
	}

	let mut results_deadwood_vec: Vec<i32> = Vec::with_capacity(12);

	for result in &all_results {
		let mut deadwood_count = 0;

		for card in &result.deadwood {
			deadwood_count += card.deadwood;
		}
		results_deadwood_vec.push(deadwood_count);
	}

	let min_deadwood = match results_deadwood_vec.iter().min() {
		Some(v) => *v,
		None => return all_results[0].clone(),
	};

	for result in &all_results {
		let mut deadwood_count = 0;

		for card in &result.deadwood {
			deadwood_count += card.deadwood;
		}
		if min_deadwood == deadwood_count {
			return result.clone();
		}
	}

	all_results[0].clone()

}


fn get_just_deadwood(hand: &HashSet<Card>, sets_in: &HashMap<i32, HashSet<Card>>,
	runs_in: &[HashSet<Card>]) -> HashSet<Card> {
	let mut runshs = HashSet::with_capacity(12);
	let mut setshs = HashSet::with_capacity(12);


	for num in sets_in.keys() {
		for card in sets_in[num].iter() {
			setshs.insert(*card);
		}
	}
	for run in runs_in.iter() {
		for card in run.iter() {
			runshs.insert(*card);
		}
	}

	let all_melds: HashSet<Card> = setshs.union(&runshs).cloned().collect();

	let deadwood_hs: HashSet<Card> = hand.difference(&all_melds).cloned().collect();

	deadwood_hs
}

fn copy_sets_and_runs(sets_in: &HashMap<i32, HashSet<Card>>, runs_in: &[HashSet<Card>]) 
	-> (HashMap<i32, HashSet<Card>>, Vec<HashSet<Card>>) {
	let mut sets: HashMap<i32, HashSet<Card>> = HashMap::with_capacity(12);
	let mut runs: Vec<HashSet<Card>> = Vec::with_capacity(12);

	for (key, value) in sets_in {
		sets.insert(key.to_owned(), value.to_owned());
	}

	for run in runs_in {
		runs.push(run.to_owned());
	}

	(sets, runs)
}

fn remove_card_from_sets(sets: &mut HashMap<i32, HashSet<Card>>, card: &Card) {
	if sets.contains_key(&card.num) {
		sets.get_mut(&card.num).unwrap().remove(card);
		if sets.get(&card.num).unwrap().len() < 3 {
			sets.remove(&card.num);
		}
	}
}

fn remove_card_from_runs(runs: &mut Vec<HashSet<Card>>, card: &Card) {
	if !runs.is_empty() {
		let mut runs_index = 0;
		for (i, run) in runs.iter().enumerate() {
			if run.contains(card) {runs_index = i; break;}
		}
		runs[runs_index].remove(card);
		let new_runs = get_runs(&runs[runs_index].iter().cloned().collect::<Vec<Card>>());
		runs.swap_remove(runs_index);
		for run in new_runs {
			runs.push(run);
		}
	}
}