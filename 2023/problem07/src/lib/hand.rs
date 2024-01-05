/**
 * Five of a kind: where all five cards have the same label: AAAAA
 * Four of a kind: where four cards have the same label and one card has a different label: AA8AA
 * Full house: where three cards have the same label, and the remaining two cards share a different label: 23332
 * Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
 * Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
 * One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
 * High card, where all cards' labels are distinct: 23456
 */

#[derive(PartialEq, Eq, Debug, PartialOrd, Ord)]
enum HandType {
	FiveOfAKind,
	FourOfAKind,
	FullHouse,
	ThreeOfAKind,
	TwoPair,
	OnePair,
	HighCard,
	Unknown
}

fn get_card_value(c: char, jokers: bool) -> usize {
	//  A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, 2
	// 13,12,11,10, 9, 8, 7, 6, 5, 4, 3, 2, 1
	if !jokers {
		match c {
			'A' => 13, 'K' => 12, 'Q' => 11, 'J' => 10, 'T' => 9, '9' => 8, '8' => 7,
			'7' => 6, '6' => 5, '5' => 4, '4' => 3, '3' => 2, '2' => 1,
			_ => 0
		}
	} else {
		match c {
			'A' => 13, 'K' => 12, 'Q' => 11, 'J' => 0, 'T' => 9, '9' => 8, '8' => 7,
			'7' => 6, '6' => 5, '5' => 4, '4' => 3, '3' => 2, '2' => 1,
			_ => 0
		}
	}
	
}

#[derive(Eq)]
pub struct Hand {
	hand: [usize;5],
	t: HandType,
	bid: usize,
}

impl std::fmt::Debug for Hand {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		write!(f, "(\nhand: {:?}\ntype: {:?}\nbid: {}\n)",
			   self.hand,
			   self.t,
			   self.bid
		)
			.unwrap();
		Ok(())
	}
}

impl Hand {
	pub fn new(s: &str, value: usize) -> Self {
		Hand {
			hand: s.chars().map(|c| {return get_card_value(c, false)}).collect::<Vec<usize>>().try_into().unwrap(),
			t: HandType::Unknown,
			bid: value
		}
	}

	pub fn new_p2(s: &str, value: usize) -> Self {
		Hand {
			hand: s.chars().map(|c| {return get_card_value(c, true)}).collect::<Vec<usize>>().try_into().unwrap(),
			t: HandType::Unknown,
			bid: value
		}
	}

	pub fn get_bid(&self) -> usize {
		self.bid
	}

	pub fn check_hand(&mut self) {
		let mut h: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
		let cop = self.hand.clone();
		for card in cop {
			h.entry(card).and_modify(|v| *v += 1).or_insert(1);
		}
		let mut vec_of_values: Vec<usize> = h.into_values().collect();
		vec_of_values.sort_by(|a,b| b.cmp(&a));
		if vec_of_values[0] == 5 {
			self.t = HandType::FiveOfAKind;
		} else if vec_of_values[0] == 4 {
			self.t = HandType::FourOfAKind;
		} else if vec_of_values[0] == 3 {
			self.t = if vec_of_values[1] == 2 {HandType::FullHouse} else {HandType::ThreeOfAKind};
		} else if vec_of_values[0] == 2 {
			self.t = if vec_of_values[1] == 2 {HandType::TwoPair} else {HandType::OnePair};
		} else {
			self.t = HandType::HighCard;
		}
	}

	pub fn check_hand_p2(&mut self) {
		let mut h: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
		let cop = self.hand.clone();
		let mut number_of_jokers = 0;
		for card in cop {
			if card != 0 {
				h.entry(card).and_modify(|v| *v += 1).or_insert(1);
			} else {
				number_of_jokers += 1
			}
		}
		let mut vec_of_values: Vec<usize> = h.into_values().collect();
		vec_of_values.sort_by(|a,b| b.cmp(&a));
		if vec_of_values.len() == 0 {
			self.t = HandType::FiveOfAKind;
			return;
		}
		if vec_of_values[0] + number_of_jokers == 5 {
			self.t = HandType::FiveOfAKind;
		} else if vec_of_values[0] + number_of_jokers == 4 {
			self.t = HandType::FourOfAKind;
		} else if vec_of_values[0] == 3 {
			self.t = if vec_of_values[1] + number_of_jokers == 2 {HandType::FullHouse} else {HandType::ThreeOfAKind};
		} else if vec_of_values[0] + number_of_jokers == 3 {
			self.t = if vec_of_values[1] == 2 {HandType::FullHouse} else {HandType::ThreeOfAKind};
		} else if vec_of_values[0] == 2 {
			self.t = if vec_of_values[1] + number_of_jokers == 2 {HandType::TwoPair} else {HandType::OnePair};
		} else if vec_of_values[0] + number_of_jokers == 2 {
			self.t = if vec_of_values[1] == 2 {HandType::TwoPair} else {HandType::OnePair};
		} else {
			self.t = HandType::HighCard;
		}
	}
}

impl std::cmp::PartialEq for Hand {
	fn eq(&self, other: &Hand) -> bool {
		self.t == other.t
	}
}

impl std::cmp::PartialOrd for Hand {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}


/**
This is the sorting i need
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                for (a, b) in zip(self.cards, other.cards) {
                    if a != b {return a.cmp(&b)}
                }
                Ordering::Equal
            }
        }
    }
}
*/
impl std::cmp::Ord for Hand {
	/**
	 * zip(a,b):
	 * a = [1,2,3]
	 * b = [4,5,6]
	 * zip -> [(1,4), (2,5), (3,6)]
	 */
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		if self.t != other.t {
			return other.t.cmp(&self.t);
		} 
		self.hand.cmp(&other.hand)
	}
}
