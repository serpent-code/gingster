// Gingster
// Copyright Serpentcode 2019
// Released under GPL Version 3
// Please see README and LICENSE for details.

use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;
use std::fmt;


fn main() {

	println!("Gingster 0.1.3");


	let initial_hand = get_initial_hand();

	print_melds_and_deadwood(&initial_hand);

	let (eleventh_card, passed, picked) = game_start(&initial_hand);

	mainloop(&initial_hand, eleventh_card, passed, picked);


}

// -> (11th card picked up, card passed, card we know he picked up)
fn game_start(initial_hand: &HashSet<Card>) -> (Option<Card>, Option<Card>, Option<Card>) {
	println!("\nAm I first to act?");
	let mut input = String::new();
	std::io::stdin().read_line(&mut input).unwrap();
	input = input.trim().to_ascii_uppercase();

	match input.as_ref() {
		"Y" => {
			println!("What is the faceup card in the middle?");
			let ( _ , card ) = get_one_card(initial_hand);
			if eval_faceup(initial_hand , &card) {
				println!("Pickup [{}]", card);
				return(Some(card), None, None);
			} else {
				println!("Pass.");
				println!("If he passed too pick up a card from deck and enter it.");
				println!("If he picked up the faceup card => R [Card he dropped]");
				let (oppo_picked_faceup_card , card_in) = get_one_card(initial_hand);
				if oppo_picked_faceup_card {
					return(None, Some(card) , Some(card_in));
				} else {
					return(Some(card_in) , Some(card), None);
				}
			}
		},
		 _  => {
		 	println!("What did he do on his first turn?");
		 	println!("Input Example1: P KC => if he passed KC.", );
		 	println!("Input Example2: AS KC => if he picked up AS and dropped KC.", );
		 	input = String::new();
		 	std::io::stdin().read_line(&mut input).unwrap();
		 	input = input.trim().to_ascii_uppercase();
		 	let inp_vec: Vec<&str> = input.split_ascii_whitespace().collect();
		 	if inp_vec[0] == "P" {
		 		let card = parse_one_card(inp_vec[1]);
		 		if eval_faceup(initial_hand , &card) {
		 			println!("Pickup [{}]", card);
		 			return(Some(card), None, None);
		 		} else {
		 			println!("Pass.");
		 			println!("He picks up from the deck and drops:");
		 			// let (_ , card_in) = get_one_card();
		 			return(None, Some(card), None)
		 		}
		 	} else {
		 		let picked_card  = parse_one_card(inp_vec[0]);
		 		let dropped_card = parse_one_card(inp_vec[1]);
		 		// println!("He picked up: {}", picked_card);
		 		// println!("He dropped : {}", dropped_card );
		 		return(None, Some(dropped_card), Some(picked_card));
		 	}
		 },
	}
}

fn parse_one_card(inp: &str) -> Card {
	if inp.len() != 2 {
		panic!("Invalid Input.");
	}
	let num  = &inp[..1];
	let suit = &inp[1..];

	let num_int:i32;
	let suit_char:char;
	let deadwood_int:i32;

	match num.parse::<char>().unwrap() {
		'A' => {num_int = 1; deadwood_int = 1;},
		'2' ..= '9' => {num_int = num.parse::<i32>().unwrap(); deadwood_int = num_int;},
		'T' => {num_int = 10; deadwood_int = 10;},
		'J' => {num_int = 11; deadwood_int = 10;},
		'Q' => {num_int = 12; deadwood_int = 10;},
		'K' => {num_int = 13; deadwood_int = 10;},
		_ => panic!("Invalid input for card numbers."),
	}

	match suit {
		"C" => suit_char = 'C',
		"S" => suit_char = 'S',
		"D" => suit_char = 'D',
		"H" => suit_char = 'H',
		_ => panic!("Invalid input for card suits."),
	}

	Card {num: num_int, suit: suit_char, deadwood: deadwood_int}
}

fn get_initial_hand() -> HashSet<Card> {
	println!("Enter initial hand:");
	let mut input = String::new();
	std::io::stdin().read_line(&mut input).unwrap();

	input = input.trim().to_ascii_uppercase();

	if input == "TEST" {
		run_tests();
	}

	let inp_vec: Vec<&str> = input.split_ascii_whitespace().collect();
	let mut initial_hand: HashSet<Card> = HashSet::with_capacity(12);

	for x in inp_vec.iter() {
		let card = parse_one_card(x);
		initial_hand.insert(card);
	}

	initial_hand
}


fn get_one_card(hand: &HashSet<Card>) -> (bool, Card) {
	// println!("What did he drop?");
	let mut input = String::new();
	std::io::stdin().read_line(&mut input).unwrap();
	input = input.trim().to_ascii_uppercase();
	let mut inp_vec: Vec<&str> = input.split_ascii_whitespace().collect();
	let mut oppo_picked_faceup_card = false;

	match inp_vec[0] {
		"END" => {
			print_melds_and_deadwood(hand);
			std::process::exit(0);
		},
		"R" | "P"  => {oppo_picked_faceup_card = true; inp_vec.swap_remove(0);},
		_ => {},
	}

	let card = parse_one_card(inp_vec[0]);

	(oppo_picked_faceup_card , card)
}

fn melds(hand: &HashSet<Card>) -> (HashMap<i32, HashSet<Card>>, Vec<HashSet<Card>>, HashSet<Card>) {

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

	let deadwood = get_deadwood(&local_hand_hs, &mut sets, &mut runs);

	(sets, runs, deadwood)

}

fn print_melds_and_deadwood(hand: &HashSet<Card>) {
	let (sets, runs, deadwood) = melds(&hand);
	let mut runs_sorted = Vec::with_capacity(12);

	print!("\n");
	println!("Sets:");
	for i in sets.keys() {
		print!("[ ");
		for card in sets[i].iter() {
			print!("{} ", card );
		}
		print!("] ");
	}
	print!("\n");
	println!("Runs:");
	for run in runs.iter() {
		print!("[ ");
		for card in run.iter() {
			runs_sorted.push(card);
			runs_sorted.sort()
		}
		for card in runs_sorted.iter(){
			print!("{} ", card );
		}
		print!("] ");
		runs_sorted = Vec::with_capacity(12);
	}
	print!("\n");
	// println!("2strghs: {:?}", two_straights );

	println!("Deadwood:");
	print!("[ ");
	for card in deadwood.iter() {
		print!("{} ", card );
	}
	print!("]");
	print!("\n");

	let mut deadwood_count = 0;

	for card in deadwood.iter() {
		deadwood_count += card.deadwood;
	}
	println!("Deadwood count: {:?}", deadwood_count);

}


fn get_runs(inp: &Vec<Card>) -> Vec<HashSet<Card>> {
	// It is assumed input cards are from the same suit.
	let mut inpcl: Vec<i32> = Vec::with_capacity(12);
	let mut cards: HashSet<Card> = HashSet::with_capacity(12);
	for card in inp {
		inpcl.push(card.num);
		cards.insert(*card);
	}

	inpcl.sort();

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
		output_cards.push(_cardrun);
		_cardrun = HashSet::with_capacity(12);
	}

	output_cards
}



fn get_deadwood(hand: &HashSet<Card>, sets: &mut HashMap<i32, HashSet<Card>>,
	runs: &mut Vec<HashSet<Card>>) -> HashSet<Card> {

	let mut runshs = HashSet::with_capacity(12);
	let mut setshs = HashSet::with_capacity(12);
	let mut processed = HashSet::with_capacity(3);

	for num in sets.keys() {
		for card in sets[num].iter() {
			setshs.insert(*card);
		}
	}
	for run in runs.iter() {
		for card in run.iter() {
			runshs.insert(*card);
		}
	}

	let mut _ii = 0;
	let mut bigrun: Vec<Card> = Vec::with_capacity(12);
	let mut smlrun: Vec<Card> = Vec::with_capacity(12);
	let mut runs_contain_inter = false;

	let intersections_pre: Vec<Card> = setshs.intersection(&runshs).cloned().collect();
	let mut intersections: Vec<Card> = Vec::with_capacity(12);

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
		for run in runs.iter() {
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
			intersections.push(*maxcard);
			if intersections_hm[suit].contains(&bigrun[bigrun.len() - 2]) {
				intersections.push(bigrun[bigrun.len() - 2]);
			}
		}
		if smlrun.first().unwrap() == mincard {
			intersections.push(*mincard);
			if intersections_hm[suit].contains(&smlrun[1]) {
				intersections.push(smlrun[1]);
			}
		}
		let intersections_hs: HashSet<Card> = intersections.iter().cloned().collect();
		for card in intersections_hm[suit].iter() {
			if !intersections_hs.contains(card) {
				intersections.push(*card);
			}
		}
	}

	// print!("intersections:");
	// for ccc in intersections.iter() {
	// 	print!(" {} ", ccc );
	// }
	// print!("\n");


	'main_inter: for card in intersections.iter() {
		'foo: for run in runs.iter() {
			if run.contains(card) {
				runs_contain_inter = true;
				break 'foo;
			}
		}
		if sets.get(&card.num) == None || !runs_contain_inter 
		|| processed.contains(&card.num) || runs.len() == 0 {
			continue 'main_inter;
		}
		for (i, run) in runs.iter().enumerate() {
			if run.contains(card) {
				_ii = i;
			}
		}

		let mut num_of_inters_same_num = 0;

		for _card in intersections.iter(){
			if _card.num == card.num {
				num_of_inters_same_num += 1;
			}
		}

		let mut iii = 0;

		let mut run_sorted: Vec<Card> = runs[_ii].iter().cloned().collect();
		run_sorted.sort();


		for (i, cd) in run_sorted.iter().enumerate() {
			if *cd == *card {
				iii = i;
				break;
			}
		}


		if num_of_inters_same_num == 1 {

			match runs[_ii].len() {
				3 => {
					match iii {
						0 | 1 => {
							match sets[&card.num].len() {
								3 => {sets.remove(&card.num);},
								4 => {sets.get_mut(&card.num).unwrap().remove(card);},
								_ => {},
							}
						},
						2 => {
							match sets[&card.num].len() {
								3 => {runs.swap_remove(_ii);},
								4 => {sets.get_mut(&card.num).unwrap().remove(card);},
								_ => {},
							}
						},
						_ => {},
					}
				},
				4 => {
					match iii {
						0 | 3 => {runs[_ii].remove(card);},
						1 | 2 => {
							match sets[&card.num].len() {
								3 => {sets.remove(&card.num);},
								4 => {sets.get_mut(&card.num).unwrap().remove(card);},
								_ => {},
							}
						},
						_ => {},
					}
				},
				5 => {
					match iii {
						0 | 4 => {runs[_ii].remove(card);},
						1 => {
							match sets[&card.num].len() {
								3 => {
									runs[_ii].remove(card);
									runs[_ii].remove(run_sorted.first().unwrap());
								},
								4 => {sets.get_mut(&card.num).unwrap().remove(card);},
								_ => {},
							}
							
						},
						2 => {
							match sets[&card.num].len() {
								3 => {sets.remove(&card.num);},
								4 => {sets.get_mut(&card.num).unwrap().remove(card);},
								_ => {},
							}
						},
						3 => {
							match sets[&card.num].len() {
								3 => {
									runs[_ii].remove(card);
									runs[_ii].remove(run_sorted.last().unwrap());
								},
								4 => {sets.get_mut(&card.num).unwrap().remove(card);},
								_ => {},
							}							
						},
						_ => {},
					}
				},
				6 => {
					match iii {
						0 | 5 => {runs[_ii].remove(card);},
						1 => {
							match sets[&card.num].len() {
								3 => {
									runs[_ii].remove(card);
									runs[_ii].remove(run_sorted.first().unwrap());
								},
								4 => {sets.get_mut(&card.num).unwrap().remove(card);},
								_ => {},
							}
							
						},
						2 => {
							match sets[&card.num].len() {
								3 => {
									for cardd in &run_sorted[..3] {
										runs[_ii].remove(cardd);
									}
								},
								4 => {sets.get_mut(&card.num).unwrap().remove(card);},
								_ => {},
							}
							
						},
						3 => {
							match sets[&card.num].len() {
								3 => {sets.remove(&card.num);},
								4 => {sets.get_mut(&card.num).unwrap().remove(card);},
								_ => {},
							}
						},
						4 => {
							match sets[&card.num].len() {
								3 => {
									runs[_ii].remove(card);
									runs[_ii].remove(run_sorted.last().unwrap());
								},
								4 => {sets.get_mut(&card.num).unwrap().remove(card);},
								_ => {},
							}
							
						},

						_ => {},
					}
				},
				7 => {
					match iii {
						0 | 6 => {runs[_ii].remove(card);},
						1 => {
							match sets[&card.num].len() {
								3 => {
									runs[_ii].remove(card);
									runs[_ii].remove(run_sorted.first().unwrap());
								},
								4 => {sets.get_mut(&card.num).unwrap().remove(card);},
								_ => {},
							}
						},
						2 => {
							match sets[&card.num].len() {
								3 => {
									for cardd in &run_sorted[..3] {
										runs[_ii].remove(cardd);
									}
								},
								4 => {sets.get_mut(&card.num).unwrap().remove(card);},
								_ => {},
							}
						},
						3 => {
							match sets[&card.num].len() {
								3 => {
									let mut temphs = HashSet::with_capacity(12);
									for cardd in &run_sorted[3..] {
										runs[_ii].remove(cardd);
										temphs.insert(*cardd);
									}
									temphs.remove(&run_sorted[3]);
									runs.push(temphs);
								},
								4 => {sets.get_mut(&card.num).unwrap().remove(card);},
								_ => {},
							}
							
						},
						4 => {
							match sets[&card.num].len() {
								3 => {sets.remove(&card.num);},
								4 => {sets.get_mut(&card.num).unwrap().remove(card);},
								_ => {},
							}
						},
						5 => {
							match sets[&card.num].len() {
								3 => {
									runs[_ii].remove(card);
									runs[_ii].remove(run_sorted.last().unwrap());
								},
								4 => {sets.get_mut(&card.num).unwrap().remove(card);},
								_ => {},
							}
						},
						_ => {},
					}
				},
				8 => {
					match iii {
						0 | 7 => {runs[_ii].remove(card);},
						1 => {
							match sets[&card.num].len() {
								3 => {
									runs[_ii].remove(card);
									runs[_ii].remove(run_sorted.first().unwrap());
								},
								4 => {sets.get_mut(&card.num).unwrap().remove(card);},
								_ => {},
							}
						},
						2 => {
							match sets[&card.num].len() {
								3 => {
									for cardd in &run_sorted[..3] {
										runs[_ii].remove(cardd);
									}
								},
								4 => {sets.get_mut(&card.num).unwrap().remove(card);},
								_ => {},
							}
						},
						3 | 4 => {
							match sets[&card.num].len() {
								3 => {
									let mut temphs = HashSet::with_capacity(12);
									for cardd in &run_sorted[iii..] {
										runs[_ii].remove(cardd);
										temphs.insert(*cardd);
									}
									temphs.remove(&run_sorted[iii]);
									runs.push(temphs);
								},
								4 => {sets.get_mut(&card.num).unwrap().remove(card);},
								_ => {},
							}
						},
						5 => {
							match sets[&card.num].len() {
								3 => {sets.remove(&card.num);},
								4 => {sets.get_mut(&card.num).unwrap().remove(card);},
								_ => {},
							}
						},
						6 => {
							match sets[&card.num].len() {
								3 => {
									runs[_ii].remove(card);
									runs[_ii].remove(run_sorted.last().unwrap());
								},
								4 => {sets.get_mut(&card.num).unwrap().remove(card);},
								_ => {},
							}
						},
						_ => {},

					}
				},
				9 => {
					match iii {
						0 | 8 => {runs[_ii].remove(card);},
						1 => {
							runs[_ii].remove(card);
							runs[_ii].remove(run_sorted.first().unwrap());
						},
						2 => {
							for cardd in &run_sorted[..3] {
								runs[_ii].remove(cardd);
							}
						},
						3 ..= 5 => {
							let mut temphs = HashSet::with_capacity(12);
							for cardd in &run_sorted[iii..] {
								runs[_ii].remove(cardd);
								temphs.insert(*cardd);
							}
							temphs.remove(&run_sorted[iii]);
							runs.push(temphs);
						},
						6 => {
							match sets[&card.num].len() {
								3 => {sets.remove(&card.num);},
								4 => {sets.get_mut(&card.num).unwrap().remove(card);},
								_ => {},
							}
						},
						7 => {
							runs[_ii].remove(card);
							runs[_ii].remove(run_sorted.last().unwrap());
						},

						_ => {},
					}
				},
				_ => {},
			}
		}

		else if num_of_inters_same_num == 2 {

			let mut isr = 0;
			let mut other_inter = DECK[0];

			'fooz: for _card in intersections.iter() {
				if _card.num == card.num && _card.suit != card.suit {
					other_inter = *_card;
					for (i, run) in runs.iter().enumerate() {
						if run.contains(&other_inter) {
							isr = i;
							break 'fooz;
						}
					}
				}
			}

			let mut isi = 0;

			let mut second_run_sorted: Vec<Card> = runs[isr].iter().cloned().collect();
			second_run_sorted.sort();

			for (i, cd) in second_run_sorted.iter().enumerate() {
				if cd.num == card.num {
					isi = i;
					break;
				}
			}

			let first_run_len = runs[_ii].len();
			let second_run_len = runs[isr].len();

			match sets[&card.num].len() {
				3 => {
					match first_run_len {
						3 => {
							sets.remove(&card.num);
						},
						4 => {
							match second_run_len {
								3 => {sets.remove(&card.num);},
								4 => {
									if (iii == 0 || iii == first_run_len - 1)
									&& (isi == 0 || isi == second_run_len - 1) {
										runs[_ii].remove(&card);
										runs[isr].remove(&other_inter);
									}
									else {
										sets.remove(&card.num);
									}
								},
								5 | 6 => {
									if (iii == 0 || iii == first_run_len - 1)
									&& (isi == 0 || isi == second_run_len - 1) {
										runs[_ii].remove(&card);
										runs[isr].remove(&other_inter);
									}
									else if (iii == 0 || iii == first_run_len - 1) && isi == 1 {
										runs[_ii].remove(&card);
										runs[isr].remove(&other_inter);
										runs[isr].remove(second_run_sorted.first().unwrap());
									}
									else if (isi == 0 || isi == second_run_len - 1) && iii == 1 {
										runs[_ii].remove(&card);
										runs[isr].remove(&other_inter);
										runs[_ii].remove(run_sorted.first().unwrap());
									}
									else {
										sets.remove(&card.num);
									}
								},
								_ => {},
							}
						},
						5 => {
							match second_run_len {
								3 => {sets.remove(&card.num);},
								4 | 5 => {
									if (iii == 0 || iii == first_run_len - 1)
									&& (isi == 0 || isi == second_run_len - 1) {
										runs[_ii].remove(&card);
										runs[isr].remove(&other_inter);
									}
									else if (iii == 0 || iii == first_run_len - 1) && isi == 1 {
										runs[_ii].remove(&card);
										runs[isr].remove(&other_inter);
										runs[isr].remove(second_run_sorted.first().unwrap());
									}
									else if (isi == 0 || isi == second_run_len - 1) && iii == 1 {
										runs[_ii].remove(&card);
										runs[isr].remove(&other_inter);
										runs[_ii].remove(run_sorted.first().unwrap());
									}
									else {
										sets.remove(&card.num);
									}
								},
								_ => {},
							}
						},
						6 => {
							match second_run_len {
								3 => {sets.remove(&card.num);},
								4 => {
									if (iii == 0 || iii == first_run_len - 1)
									&& (isi == 0 || isi == second_run_len - 1) {
										runs[_ii].remove(&card);
										runs[isr].remove(&other_inter);
									}
									else if (iii == 0 || iii == first_run_len - 1) && isi == 1 {
										runs[_ii].remove(&card);
										runs[isr].remove(&other_inter);
										runs[isr].remove(second_run_sorted.first().unwrap());
									}
									else if (isi == 0 || isi == second_run_len - 1) && iii == 1 {
										runs[_ii].remove(&card);
										runs[isr].remove(&other_inter);
										runs[_ii].remove(run_sorted.first().unwrap());
									}
									else {
										sets.remove(&card.num);
									}
								},
								_ => {},
							}
						},
						7 => {
							match second_run_len {
								3 => {sets.remove(&card.num);},
								_ => {},
							}
						},
						_ => {},
					}
				},
				4 => {
					match first_run_len {
						3 => {
							match second_run_len {
								3 => {
									match iii {
										0 => {
											match isi {
												0 | 1 => {sets.remove(&card.num);},
												2 => {
													sets.get_mut(&card.num).unwrap().remove(&card);
													runs.swap_remove(isr);
												},
												_ => {},
											}
										},
										1 | 2 => {
											match isi {
												0 | 1 => {
													sets.get_mut(&card.num).unwrap()
														.remove(&other_inter);
													runs.swap_remove(_ii);
												},
												2 => {
													sets.get_mut(&card.num).unwrap().remove(&card);
													runs.swap_remove(isr);
												},
												_ => {},
											}
										},
										_ => {},
									}
								},
								4 => {
									if isi == 0 {
										runs[isr].remove(second_run_sorted.first().unwrap());
										sets.get_mut(&card.num).unwrap().remove(&card);
									}
									else if isi == second_run_len - 1 {
										runs[isr].remove(second_run_sorted.last().unwrap());
										sets.get_mut(&card.num).unwrap().remove(&card);
									}
									else {
										match iii {
											0 => {sets.remove(&card.num);},
											1 | 2 => {
												sets.get_mut(&card.num).unwrap().remove(&other_inter);
												runs.swap_remove(_ii);
											},
											_ => {},
										}
									}
								},
								5 | 6 => {
									if isi == 0 {
										runs[isr].remove(second_run_sorted.first().unwrap());
										sets.get_mut(&card.num).unwrap().remove(&card);
									}
									else if isi == second_run_len - 1 {
										runs[isr].remove(second_run_sorted.last().unwrap());
										sets.get_mut(&card.num).unwrap().remove(&card);
									}
									else if isi == 1 {
										sets.get_mut(&card.num).unwrap().remove(&card);
										runs[isr].remove(&other_inter);
										runs[isr].remove(second_run_sorted.first().unwrap());
									}
									else if isi == second_run_len - 2 {
										sets.get_mut(&card.num).unwrap().remove(&card);
										runs[isr].remove(&other_inter);
										runs[isr].remove(second_run_sorted.last().unwrap());
									}
									else {
										match iii {
											0 => {sets.remove(&card.num);},
											1 | 2 => {
												sets.get_mut(&card.num).unwrap().remove(&other_inter);
												runs.swap_remove(_ii);
											},
											_ => {},
										}
									}
								},
								_ => {},
							}
						},
						4 => {
							match second_run_len {
								3 => {
									if iii == 0 {
										runs[_ii].remove(run_sorted.first().unwrap());
										sets.get_mut(&card.num).unwrap().remove(&other_inter);
									}
									else if iii == second_run_len - 1 {
										runs[_ii].remove(run_sorted.last().unwrap());
										sets.get_mut(&card.num).unwrap().remove(&other_inter);
									}
									else {
										match isi {
											0 => {sets.remove(&card.num);},
											1 | 2 => {
												sets.get_mut(&card.num).unwrap().remove(&card);
												runs.swap_remove(isr);
											},
											_ => {},
										}
									}
								},
								4 => {
									if (iii == 0 || iii == first_run_len -  1)
									&& (isi == 0 || isi == second_run_len - 1) {
										runs[_ii].remove(&card);
										runs[isr].remove(&other_inter);
									}
									else if (iii == 0 || iii == first_run_len - 1)
									&& !(isi == 0 || isi == second_run_len - 1) {
										runs[_ii].remove(&card);
										sets.get_mut(&card.num).unwrap().remove(&other_inter);
									}
									else if (isi == 0 || isi == second_run_len - 1)
									&& !(iii == 0 || iii == first_run_len -  1) {
										runs[isr].remove(&other_inter);
										sets.get_mut(&card.num).unwrap().remove(&card);
									}
									else {
										sets.remove(&card.num);
									}
								},
								5 => {
									if (iii == 0 || iii == first_run_len -  1)
									&& (isi == 0 || isi == second_run_len - 1) {
										runs[_ii].remove(&card);
										runs[isr].remove(&other_inter);
									}
									else if (iii == 0 || iii == first_run_len - 1)
									&& !(isi == 0 || isi == second_run_len - 1) {
										runs[_ii].remove(&card);
										sets.get_mut(&card.num).unwrap().remove(&other_inter);
									}
									else if (isi == 0 || isi == second_run_len - 1)
									&& !(iii == 0 || iii == first_run_len -  1) {
										runs[isr].remove(&other_inter);
										sets.get_mut(&card.num).unwrap().remove(&card);
									}
									else if isi == 1 {
										sets.get_mut(&card.num).unwrap().remove(&card);
										runs[isr].remove(&other_inter);
										runs[isr].remove(run_sorted.first().unwrap());
									}
									else if isi == 3 {
										sets.get_mut(&card.num).unwrap().remove(&card);
										runs[isr].remove(&other_inter);
										runs[isr].remove(run_sorted.last().unwrap());
									}
									else {
										sets.remove(&card.num);
									}
								},
								_ => {},
							}
						},
						5 => {
							match second_run_len {
								3 => {
									if iii == 0 {
										runs[_ii].remove(run_sorted.first().unwrap());
										sets.get_mut(&card.num).unwrap().remove(&other_inter);
									}
									else if iii == second_run_len - 1 {
										runs[_ii].remove(run_sorted.last().unwrap());
										sets.get_mut(&card.num).unwrap().remove(&other_inter);
									}
									else if iii == 1 {
										runs[_ii].remove(&card);
										runs[_ii].remove(run_sorted.first().unwrap());
										sets.get_mut(&card.num).unwrap().remove(&other_inter);
									}
									else if iii == 3 {
										runs[_ii].remove(&card);
										runs[_ii].remove(run_sorted.last().unwrap());
										sets.get_mut(&card.num).unwrap().remove(&other_inter);
									}
									else {
										match isi {
											0 => {sets.remove(&card.num);},
											1 | 2 => {
												sets.get_mut(&card.num).unwrap().remove(&card);
												runs.swap_remove(isr);
											},
											_ => {},
										}
									}
								},
								4 => {
									if (iii == 0 || iii == first_run_len -  1)
									&& (isi == 0 || isi == second_run_len - 1) {
										runs[_ii].remove(&card);
										runs[isr].remove(&other_inter);
									}
									else if (iii == 0 || iii == first_run_len - 1)
									&& !(isi == 0 || isi == second_run_len - 1) {
										runs[_ii].remove(&card);
										sets.get_mut(&card.num).unwrap().remove(&other_inter);
									}
									else if (isi == 0 || isi == second_run_len - 1)
									&& !(iii == 0 || iii == first_run_len -  1) {
										runs[isr].remove(&other_inter);
										sets.get_mut(&card.num).unwrap().remove(&card);
									}
									else if iii == 1 {
										runs[_ii].remove(&card);
										runs[_ii].remove(run_sorted.first().unwrap());
										sets.get_mut(&card.num).unwrap().remove(&other_inter);
									}
									else if iii == 3 {
										runs[_ii].remove(&card);
										runs[_ii].remove(run_sorted.last().unwrap());
										sets.get_mut(&card.num).unwrap().remove(&other_inter);
									}
									else {
										sets.remove(&card.num);
									}
								},
								_ => {},
							}
						},
						6 => {
							match second_run_len {
								3 => {
									if iii == 0 || iii == first_run_len -  1 {
										runs[_ii].remove(&card);
										sets.get_mut(&card.num).unwrap().remove(&other_inter);
									}
									else if iii == 1 {
										runs[_ii].remove(&card);
										runs[_ii].remove(run_sorted.first().unwrap());
										sets.get_mut(&card.num).unwrap().remove(&other_inter);
									}
									else if iii == 4 {
										runs[_ii].remove(&card);
										runs[_ii].remove(run_sorted.last().unwrap());
										sets.get_mut(&card.num).unwrap().remove(&other_inter);
									}
									else if iii == 2 {
										runs[_ii].remove(&card);
										runs[_ii].remove(run_sorted.first().unwrap());
										runs[_ii].remove(&run_sorted[1]);
										sets.get_mut(&card.num).unwrap().remove(&other_inter);
									}
									else if iii == 3 {
										runs[_ii].remove(&card);
										runs[_ii].remove(run_sorted.last().unwrap());
										runs[_ii].remove(&run_sorted[4]);
										sets.get_mut(&card.num).unwrap().remove(&other_inter);
									}

								},
								_ => {},
							}
						},
						_ => {},
					}
				},
				_ => {},
			}
			processed.insert(card.num);
		}

		else if num_of_inters_same_num == 3 {
			// 1 or 2 pillars: keep straights. 3 pillars:
			//keep sets. in additiong to 4th card check.

			let mut cleared_sets = false;
			let mut cleared_runs = false;
			for run in runs.iter() {
				if run.len() >= 4 {
					cleared_sets = true;
					*sets = HashMap::new();
				}
			}
			for set in sets.keys() {
				if sets[set].len() == 4 {
					cleared_runs = true;
					*runs = Vec::new();
				}
			}

			if !cleared_runs && !cleared_sets {
				match sets.len() {
					1 | 2 => {
						*sets = HashMap::new();
					},
					3 => {
						*runs = Vec::new();
					},
					_ => {},
				}
			}

			break 'main_inter;
		}
	}

	runshs = HashSet::with_capacity(12);
	setshs = HashSet::with_capacity(12);


	for num in sets.keys() {
		for card in sets[num].iter() {
			setshs.insert(*card);
		}
	}
	for run in runs.iter() {
		for card in run.iter() {
			runshs.insert(*card);
		}
	}

	let all_melds: HashSet<Card> = setshs.union(&runshs).cloned().collect();

	let mut deadwood_hs: HashSet<Card> = hand.difference(&all_melds).cloned().collect();

	let mut deadwood_hm_post_bynum: HashMap<i32, HashSet<Card>> = HashMap::with_capacity(12);
	let mut deadwood_hm_post_bysuit: HashMap<char, Vec<Card>> = HashMap::with_capacity(12);

	for c in deadwood_hs.iter() {
		deadwood_hm_post_bynum.insert(c.num, HashSet::with_capacity(4));
		deadwood_hm_post_bysuit.insert(c.suit, Vec::with_capacity(8));
	}

	for c in deadwood_hs.iter() {
		deadwood_hm_post_bynum.get_mut(&c.num).unwrap().insert(*c);
		deadwood_hm_post_bysuit.get_mut(&c.suit).unwrap().push(*c);
	}

	for rank in deadwood_hm_post_bynum.keys() {
		if deadwood_hm_post_bynum[rank].len() >= 3 {
			sets.insert(*rank, deadwood_hm_post_bynum[rank].clone());
			for cc in deadwood_hm_post_bynum[rank].iter() {
				deadwood_hs.remove(cc);
			}
		}
	}

	for suit in deadwood_hm_post_bysuit.keys() {
		let deadwood_runs_post = get_runs(&deadwood_hm_post_bysuit[suit]);
		for hs in deadwood_runs_post.iter() {
			if hs.len() >= 3 {
				runs.push(hs.clone());
				for ccc in hs.iter() {
					deadwood_hs.remove(ccc);
				}
			}
		}
	}


	deadwood_hs

}

fn mainloop(hand: &HashSet<Card>, eleventh_card: Option<Card>,
	passed: Option<Card>, picked: Option<Card>){

	let mut possible_deck: HashSet<Card> = HashSet::with_capacity(52);
	let mut myhand: HashSet<Card> = HashSet::with_capacity(12);
	let mut oppo_hand: HashSet<Card> = HashSet::with_capacity(12);
	let mut card_stream: Vec<Card> = Vec::with_capacity(40);
	let mut round = 1;

	for card in DECK.iter() {
		possible_deck.insert(*card);
	}

	for card in hand.iter() {
		myhand.insert(*card);
		possible_deck.remove(card);
	}

	if picked.is_some() {
		oppo_hand.insert(picked.unwrap());
		possible_deck.remove(&picked.unwrap());
	}

	if passed.is_some() {
		possible_deck.remove(&passed.unwrap());
		card_stream.push(passed.unwrap());
	}

	if eleventh_card.is_some() {
		myhand.insert(eleventh_card.unwrap());
		possible_deck.remove(&eleventh_card.unwrap());
		let dropped_card = drop(&mut myhand, 1);
		card_stream.push(dropped_card);
	}

	if eleventh_card.is_none() && passed.is_some() && picked.is_some() {
		if eval_faceup(&myhand, &passed.unwrap()) {
			println!("Pickup the faceup card {}", &passed.unwrap());
			myhand.insert(passed.unwrap());
			possible_deck.remove(&passed.unwrap());
		} else {
			println!("Pick a card from the deck and enter it");
			let (_, picked_card) = get_one_card(&myhand);
			card_stream.push(passed.unwrap());
			myhand.insert(picked_card);
			possible_deck.remove(&picked_card);
		}
		let dropped_card = drop(&mut myhand, 1);
		card_stream.push(dropped_card);
	}






	loop {
		println!("What is the faceup card?");
		let (oppo_picked_faceup_card, faceup_card) = get_one_card(&myhand);

		if oppo_picked_faceup_card {
			oppo_hand.insert(card_stream[card_stream.len() - 1]);
			println!("Opponent picked up {} card that I dropped.",card_stream[card_stream.len()- 1]);
		}

		card_stream.push(faceup_card);

		if eval_faceup(&myhand, &faceup_card) {
			println!("Pickup the faceup card {}", faceup_card );
			myhand.insert(faceup_card);
		} else {
			println!("Pick a card from the deck and enter it");
			let (_, picked_card) = get_one_card(&myhand);
			myhand.insert(picked_card);
			possible_deck.remove(&picked_card);
		}

		possible_deck.remove(&faceup_card);

		let dropped_card = drop(&mut myhand, round);

		card_stream.push(dropped_card);

		print_melds_and_deadwood(&myhand);

		round += 1;
	}
}


// hand is 11 cards here
fn drop(hand: &mut HashSet<Card> , round: i32) -> Card {

	if hand.len() < 11 {
		panic!("drop() called with less than 11 cards.");
	}

	let mut deadwood_count = 0;
	let mut deadwood_count_aft_drop = 0;

	let (_sets, _runs, deadwood) = melds(&hand.iter().cloned().collect());

	let mut deadwood_sorted: Vec<Card> = deadwood.iter().cloned().collect();
	deadwood_sorted.sort();

	if deadwood_sorted.len() == 0 {
		println!("BIG GIN!");
		print_melds_and_deadwood(&hand);
		std::process::exit(0);
	}

	let dropped_card = deadwood_sorted.pop().unwrap();

	for card in deadwood.iter() {
		deadwood_count += card.deadwood;
	}

	for card in deadwood_sorted.iter() {
		deadwood_count_aft_drop += card.deadwood;
	}

	match deadwood_count {
		0 => {
		println!("BIG GIN!");
		print_melds_and_deadwood(&hand);
		std::process::exit(0);
		},
		1 ..= 10 => {
			match deadwood.len() {
				1 => {
					println!("Drop {} and declare GIN!" , dropped_card);
					hand.remove(&dropped_card);
					print_melds_and_deadwood(&hand);
					std::process::exit(0);
				},
				_ => {
					println!("Drop {} and Knock!" , dropped_card);
					hand.remove(&dropped_card);
					print_melds_and_deadwood(&hand);
					std::process::exit(0);
				},
			}
		
		},

		11 ..= 20 => {
			match round {
				1 ..= 20 => {
					match deadwood_count_aft_drop {
						1 ..= 10 => {
							println!("Drop {} and Knock!" , dropped_card);
							hand.remove(&dropped_card);
							print_melds_and_deadwood(&hand);
							std::process::exit(0);
						},
						_ => {println!("Drop {}" , dropped_card);},
					}
					
				},
				_ => {println!("Drop {}" , dropped_card);},
			}
		},

		_ => {
			println!("Drop {}" , dropped_card);
		},
	}

	hand.remove(&dropped_card);

	dropped_card


}

fn eval_faceup(hand: &HashSet<Card>, candidate: &Card) -> bool {

	let (_pre_sets, _pre_runs, pre_deadwood) = melds(hand);

	// let mut pre_deadwood_count = 0;

	// for card in pre_deadwood.iter() {
	// 	pre_deadwood_count += card.deadwood;
	// }

	let mut big_hand = HashSet::with_capacity(12);

	for card in hand.iter() {
		big_hand.insert(*card);
	}

	big_hand.insert(*candidate);

	let (_aft_sets, _aft_runs, aft_deadwood) = melds(&big_hand);

	// let mut aft_deadwood_count = 0;

	// for card in aft_deadwood.iter() {
	// 	aft_deadwood_count += card.deadwood;
	// }

	if pre_deadwood.len() >= aft_deadwood.len() || candidate.num <= 3 {
		return true;
	}

	false

}


#[derive(Debug, Clone, Copy, Eq, Hash)]
struct Card {
	num: i32,
	suit: char,
	deadwood: i32,
}

impl fmt::Display for Card {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let num_char = match self.num {
			1  => 'A',
			2 ..= 9 => std::char::from_digit(self.num as u32 , 10).unwrap(),
			10 => 'T',
			11 => 'J',
			12 => 'Q',
			13 => 'K',
			_ => panic!("Bad number in display."),
		};
		write!(f, "{}{}", num_char, self.suit).unwrap();
		Ok(())
	}
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.num.cmp(&other.num)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num &&
        self.suit == other.suit
    }
}


const DECK: [Card; 52] = [
	Card {num: 1 , suit:'C' , deadwood: 1 }, Card {num: 2 , suit:'C' , deadwood: 2 },
	Card {num: 3 , suit:'C' , deadwood: 3 }, Card {num: 4 , suit:'C' , deadwood: 4 },
	Card {num: 5 , suit:'C' , deadwood: 5 }, Card {num: 6 , suit:'C' , deadwood: 6 },
	Card {num: 7 , suit:'C' , deadwood: 7 }, Card {num: 8 , suit:'C' , deadwood: 8 },
	Card {num: 9 , suit:'C' , deadwood: 9 }, Card {num: 10 , suit:'C' , deadwood: 10 },
	Card {num: 11 , suit:'C' , deadwood: 10 }, Card {num: 12 , suit:'C' , deadwood: 10 },
	Card {num: 13 , suit:'C' , deadwood: 10 },

	Card {num: 1 , suit:'S' , deadwood: 1 }, Card {num: 2 , suit:'S' , deadwood: 2 },
	Card {num: 3 , suit:'S' , deadwood: 3 }, Card {num: 4 , suit:'S' , deadwood: 4 },
	Card {num: 5 , suit:'S' , deadwood: 5 }, Card {num: 6 , suit:'S' , deadwood: 6 },
	Card {num: 7 , suit:'S' , deadwood: 7 }, Card {num: 8 , suit:'S' , deadwood: 8 },
	Card {num: 9 , suit:'S' , deadwood: 9 }, Card {num: 10 , suit:'S' , deadwood: 10 },
	Card {num: 11 , suit:'S' , deadwood: 10 }, Card {num: 12 , suit:'S' , deadwood: 10 },
	Card {num: 13 , suit:'S' , deadwood: 10 },

	Card {num: 1 , suit:'D' , deadwood: 1 }, Card {num: 2 , suit:'D' , deadwood: 2 },
	Card {num: 3 , suit:'D' , deadwood: 3 }, Card {num: 4 , suit:'D' , deadwood: 4 },
	Card {num: 5 , suit:'D' , deadwood: 5 }, Card {num: 6 , suit:'D' , deadwood: 6 },
	Card {num: 7 , suit:'D' , deadwood: 7 }, Card {num: 8 , suit:'D' , deadwood: 8 },
	Card {num: 9 , suit:'D' , deadwood: 9 }, Card {num: 10 , suit:'D' , deadwood: 10 },
	Card {num: 11 , suit:'D' , deadwood: 10 }, Card {num: 12 , suit:'D' , deadwood: 10 },
	Card {num: 13 , suit:'D' , deadwood: 10 },

	Card {num: 1 , suit:'H' , deadwood: 1 }, Card {num: 2 , suit:'H' , deadwood: 2 },
	Card {num: 3 , suit:'H' , deadwood: 3 }, Card {num: 4 , suit:'H' , deadwood: 4 },
	Card {num: 5 , suit:'H' , deadwood: 5 }, Card {num: 6 , suit:'H' , deadwood: 6 },
	Card {num: 7 , suit:'H' , deadwood: 7 }, Card {num: 8 , suit:'H' , deadwood: 8 },
	Card {num: 9 , suit:'H' , deadwood: 9 }, Card {num: 10 , suit:'H' , deadwood: 10 },
	Card {num: 11 , suit:'H' , deadwood: 10 }, Card {num: 12 , suit:'H' , deadwood: 10 },
	Card {num: 13 , suit:'H' , deadwood: 10 },

];





fn run_tests() {
	let tests = vec![
	"5h 6h 7h 6c 6d",
	"2s 3s 4s 5s 6s 7s 6c 6h",
	"2s 3s 4s 5s 6s 2h 3h 2c 3c",
	"2s 3s 4s 5s 6s 5h 6h 5c 6c",
	"4s 5s 6s 7s 8s 9s ts 7c 7d",
	"ts js qs ks kd kh",
	"9s ts js qs qd qh ks kd kh",
	"ts js qs ks td jd kd jc kc",
	"ts js qs ks td jd kd jc kc jh",
	"as 2s 3s 4s 5s 6s 7s 4h 4d",
	"3s 4s 5s 5c 5h 6h 7h",
	"3s 4s 5s 5c 5h 6h 7h 5d",
	"2s 3s 4s 2c 3c 4c 2h 3h 4h",
	"2s 3s 4s 2c 3c 4c 2h 3h 4h 2d",
	"5s 6s 7s 3h 4h 5h 4c 5c 6c",
	"3s 4s 5s 6s 4c 3d 4d 5d 6d",
	"3s 4s 5s 6s 3h 3d 4d 5d 6d",
	"2s 3s 4s 5s 3h 3d 4d 5d 6d",
	"2s 3s 4s 5s 6s 3h 3d 4d 5d 6d",
	"ah 3h 3s 4s 5s 3d 4d 7d 4c 5c 7c",
	"ah 3h 3s 4s 5s 3d 4d 7d 4c 5c 5h",
	];


	// let now = std::time::Instant::now();

	for (i, test) in tests.iter().enumerate() {
		let test2 = test.trim().to_ascii_uppercase();
		let inp_vec: Vec<&str> = test2.split_ascii_whitespace().collect();
		let mut initial_hand: HashSet<Card> = HashSet::with_capacity(12);

		for x in inp_vec.iter() {
			let card = parse_one_card(x);
			initial_hand.insert(card);
		}
		let (sets , runs , deadwood) = melds(&initial_hand);
		print!("{}) {} => [ ", i , test );
		for card in deadwood.iter() {
			print!("{} ", card  );
		}
		print!("]");

		print!(" [ ");
		for key in sets.keys() {
			for card in &sets[key] {
				print!("{} ", card  );
			}
		}
		print!("]");

		print!(" [ ");
		for run in runs.iter() {
			for card in run {
				print!("{} ", card  );
			}
		}
		println!("]");
	}

	// let new_now = std::time::Instant::now();
	// println!("Time elapsed: {:?}", new_now.duration_since(now));

	std::process::exit(0);

}
