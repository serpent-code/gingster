// Gingster
// Copyright Serpentcode 2019
// Released under GPL Version 3
// Please see README and LICENSE for details.

mod structs;
mod melder;
mod evals;
mod config;
use std::collections::HashSet;
use crate::structs::card::*;
use crate::config::CONFIG;

#[macro_use]
extern crate lazy_static;

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

fn main() {

	println!("Gingster {}", VERSION.unwrap_or("unknown version"));

	if help_in_command_line_args() {
		print_help_information()
	}

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
				println!("Pickup {}", card);
				(Some(card), None, None)
			} else {
				println!("Pass.");
				println!("If he passed too pick up a card from deck and enter it.");
				println!("If he picked up the faceup card => R [Card he dropped]");
				let (oppo_picked_faceup_card , card_in) = get_one_card(initial_hand);
				if oppo_picked_faceup_card {
					(None, Some(card) , Some(card_in))
				} else {
					(Some(card_in) , Some(card), None)
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
		 			println!("Pickup {}", card);
		 			(Some(card), None, None)
		 		} else {
		 			println!("Pass.");
		 			println!("He picks up from the deck and drops:");
		 			// let (_ , card_in) = get_one_card();
		 			(None, Some(card), None)
		 		}
		 	} else {
		 		let picked_card  = parse_one_card(inp_vec[0]);
		 		let dropped_card = parse_one_card(inp_vec[1]);
		 		// println!("He picked up: {}", picked_card);
		 		// println!("He dropped : {}", dropped_card );
		 		(None, Some(dropped_card), Some(picked_card))
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

	let inp_vec: Vec<&str> = input.split_ascii_whitespace().collect();
	let mut initial_hand: HashSet<Card> = HashSet::with_capacity(12);

	for x in inp_vec.iter() {
		let card = parse_one_card(x);
		initial_hand.insert(card);
	}

	initial_hand
}


fn get_one_card(hand: &HashSet<Card>) -> (bool, Card) {
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


fn print_melds_and_deadwood(hand: &HashSet<Card>) {
	let melded_hand = melder::get_melds::get_melds(&hand);
	let mut runs_sorted = Vec::with_capacity(12);

	if !melded_hand.sets.is_empty() {
		println!();
		println!("Sets:");
		for i in melded_hand.sets.keys() {
			print!("[ ");
			for card in melded_hand.sets[i].iter() {
				print!("{}", card );
			}
			print!("] ");
		}
	}

	if !melded_hand.runs.is_empty() {
		println!();
		println!("Runs:");
		for run in melded_hand.runs.iter() {
			print!("[ ");
			for card in run.iter() {
				runs_sorted.push(card);
				runs_sorted.sort()
			}
			for card in runs_sorted.iter(){
				print!("{}", card );
			}
			print!("] ");
			runs_sorted = Vec::with_capacity(12);
		}
	}
	
	if !melded_hand.deadwood.is_empty() {
		println!();
		println!("Deadwood:");
		print!("[ ");
		for card in melded_hand.deadwood.iter() {
			print!("{}", card );
		}
		print!("]");
	}

	println!();

	let mut deadwood_count = 0;

	for card in melded_hand.deadwood.iter() {
		deadwood_count += card.deadwood;
	}
	println!("Deadwood count: {:?}", deadwood_count);

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

	if let Some(card) = picked {
		oppo_hand.insert(card);
		possible_deck.remove(&card);
	}

	if let Some(card) = passed {
		possible_deck.remove(&card);
		card_stream.push(card);
	}

	if let Some(card) = eleventh_card {
		myhand.insert(card);
		possible_deck.remove(&card);
		let dropped_card = drop(&mut myhand, 1, &possible_deck);
		card_stream.push(dropped_card);
	}

	if eleventh_card.is_none() && picked.is_some() {
		if let Some(passed) = passed {
			if eval_faceup(&myhand, &passed) {
				println!("Pickup the faceup card {}", &passed);
				myhand.insert(passed);
				possible_deck.remove(&passed);
			} else {
				println!("Pick a card from the deck and enter it");
				let (_, picked_card) = get_one_card(&myhand);
				card_stream.push(passed);
				myhand.insert(picked_card);
				possible_deck.remove(&picked_card);
			}
			let dropped_card = drop(&mut myhand, 1, &possible_deck);
			card_stream.push(dropped_card);	
		}
	}


	loop {
		println!("What is the faceup card?");
		let (oppo_picked_faceup_card, faceup_card) = get_one_card(&myhand);

		if oppo_picked_faceup_card {
			oppo_hand.insert(card_stream[card_stream.len() - 1]);
			println!("Opponent picked up {}card that I dropped.",card_stream[card_stream.len()- 1]);
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

		let dropped_card = drop(&mut myhand, round, &possible_deck);

		card_stream.push(dropped_card);

		if CONFIG.verbose {
			print_melds_and_deadwood(&myhand);			
		}

		round += 1;
	}
}


// hand is 11 cards here
fn drop(hand: &mut HashSet<Card> , round: i32, possible_deck: &HashSet<Card>) -> Card {

	if hand.len() < 11 {
		panic!("drop() called with less than 11 cards.");
	}

	let mut deadwood_count_aft_highest_drop = 0;
	let mut can_knock = false;

	// let (_sets, _runs, deadwood) = melder::get_melds::get_melds(&hand.iter().cloned().collect());
	let melded_hand = melder::get_melds::get_melds(&hand.iter().cloned().collect());

	let mut deadwood_sorted: Vec<Card> = melded_hand.deadwood.iter().cloned().collect();
	deadwood_sorted.sort();

	match deadwood_sorted.len() {
		0 => {
			println!("BIG GIN!");
			print_melds_and_deadwood(&hand);
			std::process::exit(0);
		},
		1 => {
			println!("Drop {}and declare GIN!" , deadwood_sorted[0]);
			hand.remove(&deadwood_sorted[0]);
			print_melds_and_deadwood(&hand);
			std::process::exit(0);
		},
		_ => {},
	}

	let dropped_eval_card = evals::eval_drop::eval_drop(&deadwood_sorted, possible_deck);

	let dropped_highest_card = deadwood_sorted.pop().unwrap();

	for card in deadwood_sorted {
		deadwood_count_aft_highest_drop += card.deadwood;
	}

	if deadwood_count_aft_highest_drop <= 10 {
		can_knock = true;
	}

	println!();


	match round {
		// Early game
		1 ..= 5 => {
			match can_knock {
				true  => knock(hand, dropped_highest_card),
				false => println!("Drop {}" , dropped_eval_card),
			}
		},
		// Middle game
		6 ..= 10 => {
			match can_knock {
				true => {
					match deadwood_count_aft_highest_drop {
						0 ..= 7 => knock(hand, dropped_highest_card),
						_ => println!("Drop {}" , dropped_eval_card),
					}
				},
				false => println!("Drop {}" , dropped_eval_card),
			}
		},
		// Endgame
		_ => {
			match can_knock {
				true => {
					match deadwood_count_aft_highest_drop {
						0 ..= 2 => knock(hand, dropped_highest_card),
						_ => println!("Drop {}" , dropped_eval_card),
					}
				},
				false => println!("Drop {}" , dropped_eval_card),
			}
		},
	}


	hand.remove(&dropped_eval_card);

	dropped_eval_card

}

fn eval_faceup(hand: &HashSet<Card>, candidate: &Card) -> bool {

	let melded_hand_pre = melder::get_melds::get_melds(hand);

	let mut big_hand = HashSet::with_capacity(12);

	for card in hand.iter() {
		big_hand.insert(*card);
	}

	big_hand.insert(*candidate);

	let melded_hand_aft = melder::get_melds::get_melds(&big_hand);


	if melded_hand_aft.get_all_melded_value() > melded_hand_pre.get_all_melded_value() {
		return true;
	}

	if melded_hand_aft.deadwood.len() < melded_hand_pre.deadwood.len() {
		return true;
	}

	if candidate.num <= 3 {
		return true;
	}

	false

}

fn knock(hand: &mut HashSet<Card>, knock_card: Card) {
	println!("Drop {}and Knock!" , &knock_card);
	hand.remove(&knock_card);
	print_melds_and_deadwood(&hand);
	std::process::exit(0);
}

fn help_in_command_line_args() -> bool {
	let mut result = false;
	for argument in std::env::args() {
		match argument.as_str() {
			"-h" | "--help" => result = true,
			_ => (),
		}
	}
	result
}

fn print_help_information() {
	println!("Gin Rummy playing engine");
	println!("-v  --verbose        Enable verbose mode");
	println!("-vv --very-verbose   Enable very verbose mode");
	println!("-e  --emoji          Print cards suits using emojis");
	std::process::exit(0);					
}