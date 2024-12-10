use std::collections::{HashMap, HashSet};

fn p1_p2(input: &str) -> (usize, usize) {
	let true_input = input.replace("\r", "");
	let (s1, s2) = true_input.split_once("\n\n").unwrap();
	let mut orders: HashMap<usize, HashSet<usize>> = HashMap::new();
	for l in s1.lines() {
		let (n1, n2) = l.split_once("|").unwrap(); 
		orders.entry(n2.parse().unwrap()).or_default().insert(n1.parse().unwrap());
	}
	let pages: Vec<Vec<usize>> = s2.lines()
		.map(|l| l.split(",").map(|c| c.parse::<usize>().unwrap()).collect::<Vec<usize>>()).collect::<Vec<_>>();

	let (mut p1, mut p2) = (0,0);


	for mut p in pages {
		if p.is_sorted_by(|a, b| orders[b].contains(a)) {
			p1 += p[p.len()/2];
		} else {
			p.sort_by(|a, b| orders[b].contains(a).cmp(&true));
			p2 += p[p.len()/2];
		}
	}

	(p1, p2)
}

fn main() {
	let input = std::fs::read_to_string("input/input.txt").expect("Couldn't open");
	let (p1_r, p2_r) = p1_p2(&input);
	println!("{p1_r}, {p2_r}")
}




#[cfg(test)]
mod test{
	use super::p1_p2;

	#[test]
	fn p1_test() {
		let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";
		let (result, _) = p1_p2(input);
		assert_eq!(result, 143);
	}
}
