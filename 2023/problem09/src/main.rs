fn diff(list: &mut Vec<isize>) -> Vec<isize> {
	let to_patt = list.as_slice();
	match to_patt {
		[x, y] => {
			let result: Vec<isize> = vec![y-x];
			result
		},
		[x, y, xs @ ..] => {
			let d = y-x;
			let mut n_list = xs.to_vec().clone();
			n_list.insert(0, *y);
			let mut result = diff(&mut n_list);
			result.insert(0, d);
			result
		},
		_ => unreachable!()
	}
}
fn rec_calc(list: &mut Vec<isize>) -> Vec<Vec<isize>> {
	if list.iter().all(|x| x == &0) {
		let result: Vec<Vec<isize>> = list.iter().map(|x| vec![*x]).collect();
		return result;
	} else {
		let mut result = rec_calc(&mut diff(list));
		result.insert(0, list.to_vec());
		return result
	}
}

fn sum_calc(list: Vec<Vec<isize>>) -> isize {
	let mut delta = 0;
	let m_patt = list.as_slice();
	let (prev, rest) = match m_patt {
		[prev, rest @ ..] => {
			(prev, rest)
		}
		&[] => {
			panic!("How did we get here");
		}
	};
	rest.iter().for_each(|x| delta += x.last().unwrap());
	prev.iter().last().unwrap() + delta
}

fn p1(s: &str) -> isize {
	let lectures_str: Vec<&str> = s.split('\n').filter(|f| f != &"").collect();
	let mut sum = 0;
	lectures_str.into_iter().for_each(|lect| {
		let mut lectures: Vec<isize> = lect
			.split(' ')
			.filter(|f| f != &"")
			.map(|v| v.parse().unwrap())
			.collect();
		sum += sum_calc(rec_calc(&mut lectures));
	});
	sum
}

fn p2(s: &str) -> isize {
	let lectures_str: Vec<&str> = s.split('\n').filter(|f| f != &"").collect();
	let mut sum = 0;
	lectures_str.into_iter().for_each(|lect| {
		let mut lectures: Vec<isize> = lect
			.split(' ')
			.filter(|f| f != &"")
			.rev()
			.map(|v| v.parse().unwrap())
			.collect();
		sum += sum_calc(rec_calc(&mut lectures));
	});
	sum
}

fn main() {
	let input: &str = &std::fs::read_to_string("./src/input.txt")
		.expect("IMPOSSIBLE");
	let r1 = p1(input);
	let r2 = p2(input);
	println!("P1: {}\nP2: {}", r1, r2);
}


#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn p1_working() {
		const P1_TEST: &str = "0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45";
		assert_eq!(p1(P1_TEST), 114);
	}

	#[test]
	fn p2_working() {
		const P2_TEST: &str = "10 13 16 21 30 45";
		assert_eq!(p2(P2_TEST),5);
	}
}
