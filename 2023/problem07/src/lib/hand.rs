/**
 * Five of a kind: where all five cards have the same label: AAAAA
 * Four of a kind: where four cards have the same label and one card has a different label: AA8AA
 * Full house: where three cards have the same label, and the remaining two cards share a different label: 23332
 * Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
 * Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
 * One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
 * High card, where all cards' labels are distinct: 23456
 */

#[derive(PartialEq, Eq)]
enum HandType {
	FiveOfAKind,
	FourOfAKind,
	FullHouse,
	ThreeOfAKind,
	TwoPair,
	OnePair,
	HighCard,
	Unkown
}

fn get_card_value(c: char) -> usize {
	//  A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, 2
	// 13,12,11,10, 9, 8, 7, 6, 5, 4, 3, 2, 1
	match c {
		'A' => 13, 'K' => 12, 'Q' => 11, 'J' => 10, 'T' => 9, '9' => 8, '8' => 7,
		'7' => 6, '6' => 5, '5' => 4, '4' => 3, '3' => 2, '2' => 1,
		_ => 0
	}
}

#[derive(Eq)]
pub struct Hand {
	hand: Vec<(char, usize)>,
	t: HandType,
	value: usize,
}

impl Hand {
	pub fn new(s: &str, value: usize) -> Self {
		Hand {
			hand: s.chars().map(|c| {return (c, get_card_value(c))}).collect(),
			t: HandType::Unkown,
			value
		}
	}

	pub fn check_hand(&mut self) {
		let mut final_type: HandType = HandType::Unkown;
		let mut h: std::collections::HashMap<char, usize> = std::collections::HashMap::new();
		let cop = self.hand.clone();
		for card in cop {
			h.entry(card.0).and_modify(|v| *v += 1).or_insert(1);
		}

		let mut vec_of_values: Vec<usize> = h.into_values().collect();
		vec_of_values.sort();

		
		for v in vec_of_values.into_iter() {
			match v {
				5 => { final_type = HandType::FiveOfAKind; break; },
				4 => { final_type = HandType::FourOfAKind; break; },
				3 | 2 => { }
				_ => { final_type = HandType::HighCard },
			}
		}

		self.t = final_type;
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

impl std::cmp::Ord for Hand {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		let mut i = 0;
		loop {
			if self.hand[i] != other.hand[i] {
				break;
			}
			i += 1;
		}
		self.hand[i].1.cmp(&other.hand[i].1)
	}
}
