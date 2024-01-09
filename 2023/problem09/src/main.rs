use std::slice::Concat;
fn diff(list: &[isize]) -> &[isize]{
	match list {
		[x, y, xs @ ..] => {
			let d = y - x;
			diff(concat!(y, xs))
		},
		[] => {
			return &[0];
		}
	};
}
fn rec_calc(list: &[isize], prev_diff: isize) -> isize {
	0
}

fn p1(s: &str) -> isize {
	let lectures_str: Vec<&str> = s.split('\n').filter(|f| f != &"").collect();
	let mut sum = 0;
	lectures_str.into_iter().for_each(|lect| {
		let lectures: Vec<isize> = lect
			.split(' ')
			.filter(|f| f != &"")
			.map(|v| v.parse().unwrap())
			.collect();
		let pos = lectures.len() - 1;
		sum += rec_calc(&lectures, 0);
	});
	
	sum
}

fn main() {
    println!("Hello, world!");
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
		assert_eq!(0,0);
	}
}
