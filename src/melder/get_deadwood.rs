use std::collections::{HashMap, HashSet};
use crate::structs::card::*;
use crate::melder::get_runs::*;

pub fn get_deadwood(hand: &HashSet<Card>, sets: &mut HashMap<i32, HashSet<Card>>,
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
