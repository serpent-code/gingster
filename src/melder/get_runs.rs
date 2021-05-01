use std::collections::HashSet;
use crate::structs::card::*;

pub fn get_runs(inp: &[Card]) -> Vec<HashSet<Card>> {
	// It is assumed input cards are from the same suit.
	let mut inpcl: Vec<i32> = Vec::with_capacity(12);
	let mut cards: HashSet<Card> = HashSet::with_capacity(12);
	for card in inp {
		inpcl.push(card.num);
		cards.insert(*card);
	}

	inpcl.sort_unstable();

	let mut output: Vec<HashSet<i32>> = Vec::with_capacity(12);
	let mut temph: HashSet<i32> = HashSet::with_capacity(12);

	for (i, num) in inpcl.iter().enumerate().skip(1) {
		if *num == inpcl[i-1] + 1 {
			temph.insert(*num);
			temph.insert(inpcl[i-1]);
			if i == inpcl.len() - 1 {
				output.push(temph);
				temph = HashSet::with_capacity(12);
			}
		}
		else {
			output.push(temph);
			temph = HashSet::with_capacity(12);
		}
	}

	let mut output_cards : Vec<HashSet<Card>> = Vec::with_capacity(12);
	let mut _cardrun = HashSet::with_capacity(12);

	for run in output.iter() {
		for num in run.iter() {
			for card in cards.iter() {
				if card.num == *num {
					_cardrun.insert(*card);
				}
			}
		}
		if _cardrun.len() >= 3 {
			output_cards.push(_cardrun);
		}
		_cardrun = HashSet::with_capacity(12);
	}

	output_cards
}

