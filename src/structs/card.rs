use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Clone, Copy, Eq, Hash)]
pub struct Card {
	pub num: i32,
	pub suit: char,
	pub deadwood: i32,
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


pub const DECK: [Card; 52] = [
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

